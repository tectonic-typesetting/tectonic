# rc: force 0.1.0

Introduce this crate, which starts implementing Tectonic's HTML output. It uses
[tera] for templating and a hacked version of [pinot] for parsing font files,
which turns out to be important for precise typography.

The implementation is still very much preliminary, and the associated HTML and
CSS templates haven't yet been published anywhere, so it's not very useful as
yet, but hopefully that will all be changing soon.

[tera]: https://crates.io/crates/tera
[pinot]: https://crates.io/crates/pinot