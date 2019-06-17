#! /bin/sh
# Copyright 2019 The Tectonic Project
# Licensed under the MIT License.

set -ex

alpine_pkgs="\
g++ \
fontconfig-dev \
freetype-static \
glib-static \
graphite2-dev \
graphite2-static \
harfbuzz-dev \
harfbuzz-static \
icu-dev \
icu-static \
libpng-static \
openssl-dev \
zlib-dev \
"

cd /

export TERM=dumb
apt-get update
apt-get install -y \
        build-essential \
        sudo \
        wget
apt-get clean
rm -rf /var/lib/apt/lists/*

wget https://raw.githubusercontent.com/alpinelinux/alpine-chroot-install/v0.10.0/alpine-chroot-install
echo 'dcceb34aa63767579f533a7f2e733c4d662b0d1b  alpine-chroot-install' |sha1sum -c

# This command will error out when it attempts to bind-mount things like /proc
# because that is not allowed inside an unprivileged Docker container.
# Fortunately we can get away without making those mounts, so we can just
# ignore the error. Unfortunately this means that we'll continue on blithely
# if some other error occurs ...
sh alpine-chroot-install -b v3.9 -m "http://dl-cdn.alpinelinux.org/alpine/" || true
rm -f alpine-chroot-install

/alpine/enter-chroot sh -c "apk update && apk add $alpine_pkgs"

useradd rust --user-group --create-home --home-dir /alpine/home/rust \
        --shell /bin/bash --groups sudo --uid $uid
/alpine/enter-chroot sh -c "adduser -h /home/rust -H -D rust -u $uid"

rm -f "$0"  # self-destruct
