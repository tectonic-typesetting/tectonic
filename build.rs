// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

use std::env;

fn main() {
    // Re-export $TARGET during the build so that our executable tests know
    // what environment variable CARGO_TARGET_@TARGET@_RUNNER to check when
    // they want to spawn off executables.

    let target = env::var("TARGET").unwrap();
    println!("cargo:rustc-env=TARGET={}", target);
}
