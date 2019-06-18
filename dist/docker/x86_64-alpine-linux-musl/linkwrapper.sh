#! /bin/bash
# Copyright 2019 The Tectonic Project
# Licensed under the MIT License.

# This script derived from GitHub user @dl00:
# <https://github.com/rust-lang/rust/issues/36710#issuecomment-364623950>

platform="x86_64-unknown-linux-musl"

gcclibdir="$(echo /alpine/usr/lib/gcc/*/*/crtbeginT.o)"
gcclibdir="${gcclibdir%/crtbeginT.o}"

args=()

for arg in "$@"; do
    if [[ $arg = *"Bdynamic"* ]]; then
	true  # we do not want this arg
    elif [[ $arg = *"crti.o"* ]]; then
	args+=("$arg" "$gcclibdir/crtbeginT.o" "-Bstatic")
    elif [[ $arg = *"crtn.o"* ]]; then
	args+=("-lgcc" "-lgcc_eh" "-lc" "$gcclibdir/crtend.o" "$arg")
    else
	args+=("$arg")
    fi
done

exec $platform-g++ "${args[@]}"
