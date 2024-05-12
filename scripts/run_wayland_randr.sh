#!/usr/bin/env bash

set -e

AppDestDir="$HOME/Documents/wayland-randr"

$AppDestDir/wayland-randr adjust --brightness 0.3 eDP-1
# ./target/release/wayland-randr adjust --brightness 0.3 eDP-1
