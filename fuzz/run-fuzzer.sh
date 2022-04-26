#!/usr/bin/env bash

# NOTE: install `llvm-dev` or whichever package provides `llvm-symbolizer` if
# you want your stack traces to have any useful information! Otherwise none of
# the binary addresses are decoded when the fuzzer finds problems. You may also
# need to add the `-D` flag to `cargo fuzz run` to get more meaningful
# backtraces, at the expense of the fuzzer running much slower.

set -e
set -o pipefail
HERE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PARENT="$HERE/../"

(test -x "${HOME}/.cargo/bin/cargo-fuzz" || cargo install cargo-fuzz)

# it is important for `cargo fuzz` to be run from the project root
cd "$PARENT"

mkdir -p "$HERE/corpus"
rustup override set nightly

# run `compile` target using `seeds` as a start point and put new corpus state into `corpus` using 4 parallel jobs
cargo fuzz run compile "$HERE/corpus" "$HERE/seeds" -j 4 --all-features
