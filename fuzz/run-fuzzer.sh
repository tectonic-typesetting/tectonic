#!/usr/bin/env bash
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
