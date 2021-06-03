# tectonic_geturl 0.2.0 (2021-06-03)

- Expose a new `native-tls-vendored` Cargo feature, to allow people to control
  vendoring in the `native-tls` dependency crate.
- Work on the docs a bit.


# tectonic_geturl 0.1.0 (2021-01-16)

Initial release of "get-URL" support crate, with pluggable backends: either curl
or reqwest. Or nothing, if you know that you're not going to need the network.
