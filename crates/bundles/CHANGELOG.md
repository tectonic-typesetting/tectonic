# rc: minor bump

This release contains a major configuration change, updating the URL of the
default bundle to refer to a new, dedicated web service rather than using
`archive.org` (#833, @pkgw). The new default URL is:

https://relay.fullyjustified.net/default_bundle.tar

This switch was motivated by the recent breakage caused by a change in
archive.org's internal implementation, even though that breakage has been fixed
in the most recent release of the `tectonic_geturl` crate. The `archive.org`
redirection service has always had low-level reliability issues and, more
importantly, is blocked in China, which is a fatal issue for a potentially large
number of users.

The new webservice is a very simple nginx server set up in a Docker container
defined in the [tectonic-relay-service] repo. The associated web infrastructure
runs on Microsoft Azure and is configured using Terraform files in the
[tectonic-cloud-infra] repo.

[tectonic-relay-service]: https://github.com/tectonic-typesetting/tectonic-relay-service
[tectonic-cloud-infra]: https://github.com/tectonic-typesetting/tectonic-cloud-infra

@pkgw owns the `fullyjustified.net` domain name and the Azure subscription into
which the services are deployed.


# tectonic_bundles 0.1.0 (2021-06-15)

Add the `tectonic_bundles` crate! This separates out the implementation of the
various Tectonic file “bundles” into a standalone crate, so that you can use
them without having to link to harfbuzz and everything else pulled in by the
main crate.

As usual, separating out this crate led to some good API clarifications and
improvements. The API offered here includes some nontrivial breakage compared to
the old APIs in `tectonic::io::*`, but it's much more rationalized.
