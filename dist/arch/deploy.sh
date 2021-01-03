#! /usr/bin/env bash
# Copyright 2018-2020 the Tectonic Project
# Licensed under the MIT license

# Auto-update the Arch Linux AUR repos for Tectonic. This script is invoked in
# the CI/CD pipeline when a new release of the main Tectonic package has been
# made. It must be called *after* the Crates.io packages have been updated and
# the GitHub release artifacts have been uploaded.

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
source_sha512="$(sha512sum tectonic.crate.gz |cut -d' ' -f1)"

url="https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%40$version/tectonic-$version-x86_64-unknown-linux-gnu.tar.gz"
wget -q --progress=dot "$url" -O x86_64.tar.gz
x86_64_sha512="$(sha512sum x86_64.tar.gz |cut -d' ' -f1)"

url="https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%40$version/tectonic-$version-arm-unknown-linux-musleabihf.tar.gz"
wget -q --progress=dot "$url" -O armv7h.tar.gz
armv7h_sha512="$(sha512sum armv7h.tar.gz |cut -d' ' -f1)"

# Set up to run makepkg

wget https://www.archlinux.org/packages/core/x86_64/pacman/download/ -O pacman.pkg.tar.zst
tar -I zstd -xf pacman.pkg.tar.zst
bindir="$(pwd)/usr/bin"
export PATH="$bindir:$PATH"
export LIBRARY="$(pwd)/usr/share/makepkg"
config="$(pwd)/etc/makepkg.conf"

# Get the repos

export GIT_SSH_COMMAND="ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no -i $(pwd)/deploy_key"
git clone ssh://aur@aur.archlinux.org/tectonic.git aur-src
git clone ssh://aur@aur.archlinux.org/tectonic-bin.git aur-bin

# Update them

sed -e "s|@version@|$version|g" \
    -e "s|@source_sha512@|$source_sha512|g" \
    PKGBUILD.src.in >aur-src/PKGBUILD

(cd aur-src \
  && /bin/bash "$bindir/makepkg" --config="$config" --printsrcinfo >.SRCINFO \
  && git add PKGBUILD .SRCINFO \
  && git commit -m "Release $version")

sed -e "s|@version@|$version|g" \
    -e "s|@x86_64_sha512@|$x86_64_sha512|g" \
    -e "s|@armv7h_sha512@|$armv7h_sha512|g" \
    PKGBUILD.bin.in >aur-bin/PKGBUILD

(cd aur-bin \
  && /bin/bash "$bindir/makepkg" --config="$config" --printsrcinfo >.SRCINFO \
  && git add PKGBUILD .SRCINFO \
  && git commit -m "Release $version")

# Deploy

(cd aur-src && git push origin master)
(cd aur-bin && git push origin master)
