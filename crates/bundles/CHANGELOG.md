# rc: minor bump

Add the `tectonic_bundles` crate! This separates out the implementation of the
various Tectonic file “bundles” into a standalone crate, so that you can use
them without having to link to harfbuzz and everything else pulled in by the
main crate.

As usual, separating out this crate led to some good API clarifications and
improvements. The API offered here includes some nontrivial breakage compared to
the old APIs in `tectonic::io::*`, but it's much more rationalized.
