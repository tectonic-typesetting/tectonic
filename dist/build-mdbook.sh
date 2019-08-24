#! /bin/bash
# Copyright 2019 The Tectonic Project
# Licensed under the MIT License.

# A helper script to build a book built via the [mdbook] documentation
# program.
#
# [mdbook]: https://rust-lang-nursery.github.io/mdBook/
#
# Arguments:
#
# $1 - path to the mdbook directory in this project, relative to PWD

set -e
set -x # TEMP DEBUG

# Parameters

src_path="$1"

# Configuration that we expect to be stable.

mdbook_version=0.3.1
ci_platform=x86_64-unknown-linux-gnu
mdbook_binary_fn="mdbook-v${mdbook_version}-${ci_platform}.tar.gz"
mdbook_binary_url="https://github.com/rust-lang-nursery/mdBook/releases/download/v${mdbook_version}/${mdbook_binary_fn}"
mdbook_binary_sha256=4511fb1d4d95331099a4c1777d6af8022ac5783af70b83f018c78c896a4027ab

# Get an mdbook executable. If we end up with multiple books to build,
# this script might run multiple times, so avoid the work if possible.

mdbook="$(pwd)/mdbook"

if [ ! -x "$mdbook" ] ; then
    echo "Getting mdbook executable ..."
    wget -q --progress=dot "$mdbook_binary_url"
    echo "$mdbook_binary_sha256  $mdbook_binary_fn" |sha256sum -c
    tar xzf "$mdbook_binary_fn"
    rm -f "$mdbook_binary_fn"
fi

# Build the book.

echo "Building book ..."
pushd "$src_path"
"$mdbook" build
"$mdbook" test
popd

# And that's it.

echo "Success."
