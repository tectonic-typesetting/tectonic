# Copyright 2018-2019 The Tectonic Project
# Licensed under the MIT License.

FROM ubuntu:18.04

ARG uid=1000

ADD setup_priv.sh /
RUN sh /setup_priv.sh

ADD sudoers /etc/sudoers.d/nopasswd
USER rust
ENV PATH=/alpine/home/rust/.cargo/bin:/usr/sbin:/usr/bin:/sbin:/bin

ADD setup_unpriv.sh /
ADD toolwrapper.sh /alpine/home/rust/
ADD linkwrapper.sh /alpine/home/rust/
RUN sh /setup_unpriv.sh

# With the following customizations, we can just "docker run -v
# $tectonic:/alpine/home/rust/src $image cargo ..." and The Right Thing should
# happen:

ADD cargo-config.toml /alpine/home/rust/.cargo/config
ENV RUSTFLAGS="-L /alpine/usr/lib -l static=expat -l static=uuid -l static=stdc++ -C target-feature=+crt-static"
ENV TARGET_CC="x86_64-unknown-linux-musl-cc"
ENV TARGET_CXX="x86_64-unknown-linux-musl-g++"
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /alpine/home/rust/src
CMD ["cargo", "test", "--release", "--all"]
