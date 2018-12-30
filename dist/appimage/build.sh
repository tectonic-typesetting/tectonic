#!/bin/bash
# Copyright 2018 the Tectonic Project
# Licensed under the MIT license

# Create an AppImage of Tectonic. If the environment variable
# $TECTONIC_APPIMAGE_TAG is set, the created file will have a name like
# `tectonic-${TECTONIC_APPIMAGE_TAG}-$ARCH.AppImage`; otherwise the tag will
# be the version of the Tectonic binary. You can also set $UPDATE_INFORMATION
# to embed update info in the AppImage and cause the creation of a zsync file.
# The format of the variable should be as described here:
# https://github.com/AppImage/AppImageSpec/blob/master/draft.md#update-information
#
# This script is structured so that it can be run locally, but it's expected
# that it will primarily be used inside CI.

curl=curl

workdir="$(cd $(dirname $0) && pwd)" # these paths needs to be absolute for linuxdeploy
top="$(cd $workdir/../.. && pwd)"
assets="$workdir" # this might change ...
arch=$(uname -m)

set -ex

# Ensure we have an up-to-date release binary, and get a version tag if needed.

cd "$top"
cargo build --release

if [ -z "$TECTONIC_APPIMAGE_TAG" ] ; then
    TECTONIC_APPIMAGE_TAG="$(cargo run --quiet --release -- --version |awk '{print $2}')"
fi

# Everything else is done here:

cd "$workdir"

# Pull down linuxdeploy if needed.

linuxdeployapp=linuxdeploy-$arch.AppImage

if [ ! -x $linuxdeployapp ] ; then
    $curl -fsSL https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/$linuxdeployapp >$linuxdeployapp
    chmod +x $linuxdeployapp
fi

# Now that we have linuxdeploy, let's actually make the thing.

tectonicapp=Tectonic-$arch.AppImage
appdir="$(mktemp -d)"

OUTPUT="tectonic-$TECTONIC_APPIMAGE_TAG-$arch.AppImage" \
./$linuxdeployapp \
    --icon-file="$assets/tectonic.svg" \
    --desktop-file="$assets/tectonic.desktop" \
    --appdir="$appdir" \
    --executable="$top/target/release/tectonic" \
    --output=appimage

rm -rf "$appdir"
