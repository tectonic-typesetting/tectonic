// build.rs -- build helper script for Tectonic.
// Copyright 2016-2019 the Tectonic Project
// Licensed under the MIT License.

use cc;

fn main() {
    let mut stub_cfg = cc::Build::new();

    stub_cfg
        .flag("-Wall")
        .file("stub/stub_errno.c")
        .include(".");

    stub_cfg.compile("tectonic_bridge");
}
