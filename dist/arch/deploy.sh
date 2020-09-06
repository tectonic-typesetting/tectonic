#!/bin/bash

set -ex
cd "$(dirname $0)"
keypath="$1"
release_desc="$2"

# Set up to run makepkg
wget https://www.archlinux.org/packages/core/x86_64/pacman/download/ -O pacman.pkg.tar.xz
tar -Jxf pacman.pkg.tar.xz
bindir="$(pwd)/usr/bin"
export PATH="$bindir:$PATH"
export LIBRARY="$(pwd)/usr/share/makepkg"
config="$(pwd)/etc/makepkg.conf"

# Get the repo
git config --global --add core.sshCommand "ssh -o StrictHostKeyChecking=false -i $keypath"
git clone ssh://aur@aur.archlinux.org/tectonic.git aur

# Update it
cp PKGBUILD aur
cd aur
/bin/bash "$bindir/makepkg" --config="$config" --printsrcinfo >.SRCINFO

# Commit
git add PKGBUILD .SRCINFO
git config user.email "tectonic-deploy@example.com"
git config user.name "tectonic-deploy"
git commit -m "Release $release_desc"

# Deploy to AUR
git push origin master
