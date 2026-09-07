#!/usr/bin/env sh
# eframe est compilé avec les features x11 et wayland : sans ces paquets, la CI
# échoue à l'édition de liens et non au test, ce qui envoie chercher loin.
set -eu

sudo apt-get update
sudo apt-get install -y --no-install-recommends \
    libxkbcommon-dev libxkbcommon-x11-dev \
    libwayland-dev wayland-protocols \
    libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libx11-dev libxrandr-dev libxi-dev libxcursor-dev \
    libgl1-mesa-dev libegl1-mesa-dev
