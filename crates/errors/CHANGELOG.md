# rc: minor bump

The only change in this release is to add a helpful `tectonic_errors::prelude`
module, which makes it easy to get all of the names you need without getting
compiler warnings about the ones that you don't end up using.


# tectonic_errors 0.1.0 (2021-01-15)

Initial release. A new crate providing a generic boxed error type for Tectonic.

We need a boxed error type because we have a bunch of optional dependencies, and
we can't abstract around their errors without boxing them.

Strongly derived from [Cranko](https://github.com/pkgw/cranko).
