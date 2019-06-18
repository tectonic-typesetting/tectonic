#! /bin/sh
# Copyright 2019 The Tectonic Project
# Licensed under the MIT License.

set -ex

platform="x86_64-unknown-linux-musl"

cd

wget -O rustup.sh https://sh.rustup.rs
sh rustup.sh -y --default-toolchain stable
rm -f rustup.sh

rustup target add $platform

sudo chmod +x $HOME/*.sh

for tool in cc g++ ld pkg-config ; do
    ln -s ../../toolwrapper.sh $HOME/.cargo/bin/$platform-$tool
done

cat <<EOF >$HOME/.cargo/bin/pkg-config
#! /bin/bash
exec $platform-pkg-config "\$@"
EOF
chmod +x $HOME/.cargo/bin/pkg-config

sudo rm -f "$0"  # self-destruct
