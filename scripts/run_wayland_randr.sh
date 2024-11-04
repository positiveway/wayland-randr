#!/usr/bin/env bash

set -e

BuiltinDisplay="eDP-1"
eGPUDisplay="eDP-1-1"

Brightness="0.25"


/mnt/data/Documents/Scripts/wayland-randr adjust --brightness "$Brightness" "$BuiltinDisplay"
/mnt/data/Documents/Scripts/wayland-randr adjust --brightness "$Brightness" "$eGPUDisplay"

# ./target/release/wayland-randr adjust --brightness 0.3 eDP-1
