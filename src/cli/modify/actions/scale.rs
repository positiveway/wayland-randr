use wayland_randr::display_config::{models::PhysicalMonitor, ApplyConfig};

use super::Action;

pub struct ScaleAction {
    pub scale: f64,
}

impl Action<'_> for ScaleAction {
    fn apply(&self, config: &mut ApplyConfig, _: &PhysicalMonitor) {
        config.scale = self.scale;
    }
}

impl std::fmt::Display for ScaleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "setting scale to {}", self.scale)
    }
}
