# tectonic_bridge_graphite2 0.2.0 (2021-06-03)

- Fix up handling of how C/C++ header file paths are exported to dependent
  crates. This is a breaking change: we've moved from a single include directory
  to a list of them.
- Some improvements to the documentation

# tectonic_bridge_graphite2 0.1.1 (2021-01-16)

- Export information about the `GRAPHITE2_STATIC` C preprocessor define that is
  sometimes needed.

# tectonic_bridge_graphite2 0.1.0 (2021-01-04)

A new crate to encapsulate the location and use of the `graphite2` library used
by Tectonic.
