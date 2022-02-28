# tectonic_geturl 0.3.1 (2022-02-28)

- No meaningful code changes; we're just fixing some new Clippy complaints.


# tectonic_geturl 0.3.0 (2021-10-11)

This release contains an essential fix for what has been the default Tectonic
configuration, which access `archive.org` to look up the default bundle.

- Update redirection logic to unbreak archive.org resolution (#832, @pkgw). The
  Internet Archive PURL service added a new layer of indirection through the URL
  `https://purl.prod.archive.org/net/pkgwpub/tectonic-default`, which had an
  unfortunate interaction with logic in Tectonic intended avoid pursuing
  redirections into S3-type hashed storage services. That logic stopped
  resolution when the final element of the URL path (i.e. the filename) did not
  contain a period character. This used to be fine when the base archive.org URL
  redirected directly to the configured destination URL, but stopped too soon
  with the new indirection layer. The logic has been updated to also continue
  pursuing the redirection if the filename of the new URL matches the filename
  of the original URL, which avoids the issue in this case and seems generally
  reasonable.
- Related to the above, the new archive.org redirection used an HTTP status code
  of 307, which is a slightly more fully-specified version of the 302 status
  code. While the redirection code accepted a final status code of 302
  (indicating that it decided to stop resolving URLs, i.e., it thinks that it
  has reached the edge of an S3-type hashed storage service), it did not accept
  a 307 result. Now it does (#832, @pkgw). Note that if this behavior had been
  in place before, Tectonic would not have broken with the new archive.org
  update, but the behavior would have been somewhat incorrect: the URL
  resolution would have stopped too soon. But given the semantic similarity of
  302 and 307, if we allow the former, we should allow the latter.

These fixes are, however, effectively superseded because the release of Tectonic
that contains them will also contain an update of the default URL to a new
dedicated service (`relay.fullyjustified.net`), since `archive.org` is sometimes
unreliable and is blocked in China.


# tectonic_geturl 0.2.1 (2021-06-15)

- Fix a deprecation warning in the latest version of `reqwest`.


# tectonic_geturl 0.2.0 (2021-06-03)

- Expose a new `native-tls-vendored` Cargo feature, to allow people to control
  vendoring in the `native-tls` dependency crate.
- Work on the docs a bit.


# tectonic_geturl 0.1.0 (2021-01-16)

Initial release of "get-URL" support crate, with pluggable backends: either curl
or reqwest. Or nothing, if you know that you're not going to need the network.
