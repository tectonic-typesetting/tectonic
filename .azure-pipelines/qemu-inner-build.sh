#! /bin/bash
# Copyright 2018-2019 the Tectonic Project
# Licensed under the MIT License.

set -ex

work="$1"
cd "$work"

# Validate that we're actually running in big-endian land! I spent a while
# working on ARM emulation before I noticed that Ubuntu ARM builds are
# little-endian ...

cat <<'EOF' >check-bigendian.rs
#[cfg(not(target_endian = "big"))]
fn error_if_not_big() { this_platform_should_be_bigendian_but_apparently_is_not(); }
fn main() {}
EOF

rustc --crate-type=bin -o check-bigendian check-bigendian.rs

# I've read that QEMU has trouble with multithreading, so we try hard to
# serialize everything we do.

export RUSTFLAGS="-C codegen-units=1"
cargo build -j=1
RUST_TEST_THREADS=1 cargo test -j=1
