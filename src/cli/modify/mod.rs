mod actions;

use wayland_randr::{display_config::ApplyConfig, DisplayConfig};
use structopt::StructOpt;

use self::actions::{Action, ModeAction, PrimaryAction, RotationAction, ScaleAction};

#[derive(Clone, Copy)]
pub enum Rotation {
    Normal,
    Left,
    Right,
    Inverted,
}

impl std::str::FromStr for Rotation {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Rotation::Normal),
            "left" => Ok(Rotation::Left),
            "right" => Ok(Rotation::Right),
            "inverted" => Ok(Rotation::Inverted),
            _ => Err(std::fmt::Error),
        }
    }
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rotation::Normal => "normal",
                Rotation::Left => "left",
                Rotation::Right => "right",
                Rotation::Inverted => "inverted",
            }
        )
    }
}

#[derive(StructOpt)]
pub struct ActionOptions {
    #[structopt(
        short,
        long = "rotate",
        help = "One of 'normal', 'left', 'right' or 'inverted'",
        long_help = "One of 'normal', 'left', 'right' or 'inverted'. This causes the output contents to be rotated in the specified direction. 'right' specifies a clockwise rotation of the picture and 'left' specifies a counter-clockwise rotation."
    )]
    pub rotation: Option<Rotation>,

    #[structopt(
        short,
        long,
        help = "A valid mode for the given display.",
        long_help = "A valid mode for the given display. To find valid modes use the \"query\" subcommand"
    )]
    pub mode: Option<String>,

    #[structopt(long, help = "Set the given monitor as the primary logical monitor")]
    pub primary: bool,

    #[structopt(long, help = "Set the scale")]
    pub scale: Option<f64>,
}

#[derive(StructOpt)]
pub struct CommandOptions {
    #[structopt(
        help = "the connector used for the physical monitor.",
        long_help = "the connector used for the physical monitor you want to modify, e.g. \"HDMI-1\". You can find these with \"query\" (no arguments) if you're unsure."
    )]
    pub connector: String,

    #[structopt(flatten)]
    pub actions: ActionOptions,

    #[structopt(
        short,
        long,
        help = "Attempt to replicate this configuration the next time this HW layout appears"
    )]
    persistent: bool,

    #[structopt(long, help = "List changes without actually applying them")]
    dry_run: bool,
}

#[derive(Debug)]
pub enum Error {
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Error::NotFound => "fatal: unable to find output.",
            }
        )
    }
}

impl std::error::Error for Error {}

pub fn handle(
    opts: &CommandOptions,
    config: &DisplayConfig,
    proxy: &dbus::blocking::Proxy<&dbus::blocking::Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (logical_monitor, physical_monitor) =
        config.search(&opts.connector).ok_or(Error::NotFound)?;

    let mut actions = Vec::<Box<dyn Action>>::new();
    let primary_is_changing = opts.actions.primary;

    if let Some(rotation) = &opts.actions.rotation {
        actions.push(Box::new(RotationAction {
            rotation: *rotation,
        }));
    }

    if let Some(mode_id) = &opts.actions.mode {
        actions.push(Box::new(ModeAction { mode: mode_id }))
    }

    if opts.actions.primary {
        actions.push(Box::new(PrimaryAction {}));
    }

    if let Some(scale) = &opts.actions.scale {
        actions.push(Box::new(ScaleAction { scale: *scale }))
    }

    if actions.is_empty() {
        println!("no changes made.");
        return Ok(());
    }

    let mut apply_config = ApplyConfig::from(logical_monitor, physical_monitor);

    if opts.persistent {
        println!("attempting to persist config to disk")
    }

    for action in actions.iter() {
        println!("{}", &action);
        action.apply(&mut apply_config, physical_monitor);
    }

    if opts.dry_run {
        println!("dry run: no changes made.");
        return Ok(());
    }

    let all_configs = config
        .monitors
        .iter()
        .filter_map(|monitor| {
            if monitor.connector == opts.connector {
                return Some(apply_config.clone());
            }

            let (logical_monitor, _) = match config.search(&monitor.connector) {
                Some(monitors) => monitors,
                None => return None,
            };

            let mut apply_config = ApplyConfig::from(logical_monitor, monitor);

            if primary_is_changing {
                apply_config.primary = false;
            }

            Some(apply_config)
        })
        .collect();

    config.apply_monitors_config(proxy, all_configs, opts.persistent)?;

    Ok(())
}
