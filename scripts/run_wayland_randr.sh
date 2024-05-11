#!/usr/bin/env bash

set -e

AppDestDir="$HOME/Documents/wayland-randr"

$AppDestDir/wayland-randr adjust --brightness 0.4 eDP-1
# ./target/release/gnome-randr adjust --brightness 0.4 eDP-1
