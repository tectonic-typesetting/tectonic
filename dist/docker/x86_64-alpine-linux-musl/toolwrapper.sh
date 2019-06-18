#! /bin/bash
# Copyright 2019 The Tectonic Project
# Licensed under the MIT License.

# Wrap toolchain invocations to happen inside the Alpine chroot.

platform="x86_64-unknown-linux-musl"

# $0

exename="$(basename $0)"

if [[ $exename != $platform-* ]] ; then
    echo >&2 "error: executable name ($exename) must start with \"$platform-\""
    exit 1
fi

exename="${exename#$platform-}"

# working directory

curdir=$(pwd)

if [[ $curdir != /alpine/* ]] ; then
    echo >&2 "error: working directory must be inside /alpine/ prefix"
    exit 1
fi

curdir="${curdir#/alpine}"

# args. We use printf to avoid "echo" eating flag args like "-n", and a global
# sed substitution in case a single arg contains multiple paths (could happen
# with something like -Wl,foo,bar,baz).

args=()

for arg in "$@"; do
    args+=("$(printf %s "$arg" |sed -e s@/alpine@@g)")
done

# ready to go!

### set -x # <= for debugging
exec /alpine/enter-chroot -u rust sh -c "cd $curdir && \"\$@\"" -- "$exename" "${args[@]}"
