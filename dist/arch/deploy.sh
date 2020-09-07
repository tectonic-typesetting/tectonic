#!/bin/bash

set -ex
cd "$(dirname $0)"
release_desc="$1"

# Set up to run makepkg
wget https://www.archlinux.org/packages/core/x86_64/pacman/download/ -O pacman.pkg.tar.zst
tar -I zstd -xf pacman.pkg.tar.zst
bindir="$(pwd)/usr/bin"
export PATH="$bindir:$PATH"
export LIBRARY="$(pwd)/usr/share/makepkg"
config="$(pwd)/etc/makepkg.conf"

# Get the repo
export GIT_SSH_COMMAND="ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -i deploy_key"
git clone ssh://aur@aur.archlinux.org/tectonic.git aur

# Update it
cp PKGBUILD aur
cd aur
/bin/bash "$bindir/makepkg" --config="$config" --printsrcinfo >.SRCINFO

# Commit
git add PKGBUILD .SRCINFO
git commit -m "Release $release_desc"

# Deploy to AUR
git push origin master
