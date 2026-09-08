#!/usr/bin/env sh
# eframe is compiled with the x11 and wayland features: without these packages
# the CI fails at linking rather than at the tests, which sends one looking far
# from the cause.
set -eu

sudo apt-get update
sudo apt-get install -y --no-install-recommends \
    libxkbcommon-dev libxkbcommon-x11-dev \
    libwayland-dev wayland-protocols \
    libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
    libx11-dev libxrandr-dev libxi-dev libxcursor-dev \
    libgl1-mesa-dev libegl1-mesa-dev
