#!/bin/bash

set -ex
cd "$TRAVIS_BUILD_DIR/dist/arch"

# Set up to run makepkg
wget https://www.archlinux.org/packages/core/x86_64/pacman/download/ -O pacman.pkg.tar.xz
tar -Jxf pacman.pkg.tar.xz
export PATH="$PATH:$(pwd)/usr/bin"
export LIBRARY="$(pwd)/usr/share/makepkg"
CONFIG="$(pwd)/etc/makepkg.conf"

# Get the repo
git config --global --add core.sshCommand "ssh -o StrictHostKeyChecking=false"
git clone ssh://aur@aur.archlinux.org/tectonic.git aur

# Update it
cp PKGBUILD aur
cd aur
makepkg --config="$CONFIG" --printsrcinfo >.SRCINFO

# Commit
git add PKGBUILD .SRCINFO
git config user.email "tectonic-deploy@example.com"
git config user.name "tectonic-deploy"
git commit -m "Release $TRAVIS_TAG"

# Deploy to AUR
git push origin master
