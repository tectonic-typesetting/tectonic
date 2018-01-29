#! /bin/bash
# Copyright 2018 the Tectonic Project
# Licensed under the MIT License.

set -e -x

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

# We have to build and test in --release mode because currently (1.24.0)
# PowerPC Rust has a crashing bug in debuginfo generation:
# https://github.com/rust-lang/rust/issues/41253 .

cargo build --release
cargo test --release
