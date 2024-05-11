#!/usr/bin/env bash

RunAtLogin=false

AppDestDir="$HOME/Documents/wayland-randr"
LoginStartupDir="/etc/profile.d"

RunScriptDir="$HOME/Desktop"
RunAtLoginDir=$LoginStartupDir

set -e

#Linker
sudo apt install lld mold

sudo apt install pkg-config

cd ..
cargo build --release

[ -d $AppDestDir ] || mkdir $AppDestDir
cp ./target/release/wayland-randr $AppDestDir/wayland-randr

if $RunAtLogin
then
sudo cp ./scripts/run_at_login_wayland_randr.sh $RunAtLoginDir
sudo chmod +x $RunAtLoginDir/run_at_login_wayland_randr.sh
fi

cp ./scripts/run_wayland_randr.sh $RunScriptDir
sudo chmod +x $RunScriptDir/run_wayland_randr.sh

$RunScriptDir/run_wayland_randr.sh
