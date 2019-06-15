# Tectonic Fuzzing Support

This is *beta-level* support for [fuzzing] the Tectonic engine using
[cargo-fuzz]. At the moment, the fuzzing does not work very well because it
exposes memory leaks in the Tectonic engine; this is desirable in and of
itself, but the exposed leaks have proven to be tricky to fix. (Patches most
welcome, of course! The limiting factor here is definitely developer
bandwidth.)

See the script `./run-fuzzer.sh` for an example of how to, well, run the
fuzzer. Note that this script sets a [rustup directory override] to build
Tectonic using the nightly compilers, since nightly is currently required by
the fuzzer.

To tell the fuzzer to ignore memory leaks, run a command like the following
from the toplevel directory of the Tectonic repository:

```sh
cargo +nightly fuzz run compile fuzz/corpus fuzz/seeds -- -detect_leaks=0
```

[fuzzing]: https://en.wikipedia.org/wiki/Fuzzing
[cargo-fuzz]: https://github.com/rust-fuzz/cargo-fuzz
[rustup directory override]: https://github.com/rust-lang/rustup.rs#directory-overrides
