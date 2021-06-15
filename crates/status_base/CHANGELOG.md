# tectonic_status_base 0.2.0 (2021-06-15)

- Add `PlainStatusBackend.always_stderr()`, allowing users to specify that
  status-reporting output in this backend should always go to standard error
  rather than standard output. This is useful in cases where a program's output
  to stdout needs to be machine-parseable, since the status-reporting could
  potentially interfere with that if not directed elsewhere (@pkgw, #768).


# tectonic_status_base 0.1.0 (2021-01-15)

Initial release: a new crate with basic Tectonic status-reporting traits.

A lot of this is admittedly close to generic logging infrastructure, but we do
have some custom methods to help support a nice polished Tectonic UX. And that
will likely continue to be the case going forward.
