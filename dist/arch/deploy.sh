#! /usr/bin/env bash
# Copyright 2018-2020 the Tectonic Project
# Licensed under the MIT license

# Auto-update the Arch Linux AUR repo for Tectonic. This script is invoked in
# the CI/CD pipeline when a new release of the main Tectonic package has been
# made. It must be called *after* the Crates.io package has been updated.

set -xeuo pipefail
cd "$(dirname $0)"

if [ ! -f deploy_key ] ; then
  echo >&2 "error: the deploy_key file must be created by the CI system"
  exit 1
fi

# Get the settings that we need
version="$(cranko show version tectonic)"
url="https://crates.io/api/v1/crates/tectonic/$version/download"
wget -q --progress=dot "$url" -O tectonic.crate.gz
sha512="$(sha512sum tectonic.crate.gz |cut -d' ' -f1)"

# Set up to run makepkg
wget https://www.archlinux.org/packages/core/x86_64/pacman/download/ -O pacman.pkg.tar.zst
tar -I zstd -xf pacman.pkg.tar.zst
bindir="$(pwd)/usr/bin"
export PATH="$bindir:$PATH"
export LIBRARY="$(pwd)/usr/share/makepkg"
config="$(pwd)/etc/makepkg.conf"

# Get the repo
export GIT_SSH_COMMAND="ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -i $(pwd)/deploy_key"
git clone ssh://aur@aur.archlinux.org/tectonic.git aur

# Update it
sed -e "s|@version@|$version|g" -e "s|@sha512@|$sha512|g" PKGBUILD.in >aur/PKGBUILD
cd aur
/bin/bash "$bindir/makepkg" --config="$config" --printsrcinfo >.SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Release $version"

# Deploy to AUR
git push origin master
