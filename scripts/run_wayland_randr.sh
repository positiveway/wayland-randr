#!/usr/bin/env bash

set -e

DataDir="/mnt/data"
DocsDir="$DataDir/Documents"
ScriptsDir="$DocsDir/Scripts"

"$ScriptsDir/wayland-randr" adjust --brightness 0.25 eDP-1
# ./target/release/wayland-randr adjust --brightness 0.3 eDP-1
