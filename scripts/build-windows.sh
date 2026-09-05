#!/usr/bin/env bash
# Construit l'exécutable Windows 64 bits depuis macOS ou Linux.
# Prérequis :
#   rustup target add x86_64-pc-windows-gnu
#   macOS  : brew install mingw-w64
#   Debian : apt install mingw-w64
set -euo pipefail

TARGET=x86_64-pc-windows-gnu
cd "$(dirname "$0")/.."

cargo build --release -p cao_app --target "$TARGET"

OUT="target/$TARGET/release/cao.exe"
echo
echo "Exécutable : $OUT"
ls -lh "$OUT" | awk '{print "Taille     : " $5}'
