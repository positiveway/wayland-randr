#!/usr/bin/env bash

set -e

SimpleCopy="cp -f"

#DesktopDir="$HOME/Desktop"
DataDir="/mnt/data"
DocsDir="$DataDir/Documents"
ScriptsDir="$DocsDir/Scripts"

#Linker
sudo apt install lld mold

sudo apt install pkg-config

cd ..
cargo build --release

# Copy to Scripts
mkdir -p "$ScriptsDir"
$SimpleCopy ./target/release/wayland-randr "$ScriptsDir"

$SimpleCopy ./scripts/run_wayland_randr.sh "$ScriptsDir"
chmod +x "$ScriptsDir/run_wayland_randr.sh"

# Copy to Desktop
#$SimpleCopy ./scripts/run_wayland_randr.sh "$DesktopDir"
#chmod +x "$DesktopDir/run_wayland_randr.sh"
