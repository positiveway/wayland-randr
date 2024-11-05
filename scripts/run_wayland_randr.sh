#!/usr/bin/env bash

BuiltinDisplay="eDP-1"
eGPUDisplay="eDP-1-1"

Brightness="0.25"

# ./target/release/wayland-randr adjust --brightness 0.3 eDP-1

/mnt/data/Documents/Scripts/wayland-randr adjust --brightness "$Brightness" "$BuiltinDisplay"
/mnt/data/Documents/Scripts/wayland-randr adjust --brightness "$Brightness" "$eGPUDisplay"

#confirm settings
#xrandr --current --verbose
