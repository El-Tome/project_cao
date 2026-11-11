#!/usr/bin/env sh
# The runner's apt mirrorlist and its security pocket stopped resolving
# (#884) while archive.ubuntu.com, which carries the same pockets, still
# answered: apt is pointed there first, the runner's own mirrors kept after it
# for when that one fails instead. A mirrorlist line with no priority is tried
# last, so the archive's is given the lowest.
set -eu

archive='http://archive.ubuntu.com/ubuntu/'
first=$(printf '%s\tpriority:0' "$archive")
mirrors=/etc/apt/apt-mirrors.txt
if [ -f "$mirrors" ]; then
    { echo "$first"; grep -vxF "$first" "$mirrors" || true; } |
        sudo tee "$mirrors.new" > /dev/null
    sudo mv "$mirrors.new" "$mirrors"
fi

for sources in /etc/apt/sources.list /etc/apt/sources.list.d/*.list \
    /etc/apt/sources.list.d/*.sources; do
    if [ -f "$sources" ]; then
        sudo sed -i "s|https*://security\.ubuntu\.com/ubuntu/*|$archive|g" "$sources"
    fi
done
