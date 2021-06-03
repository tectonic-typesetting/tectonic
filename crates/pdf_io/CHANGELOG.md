# tectonic_pdf_io 0.1.0 (2021-06-03)

This crate contains the vast majority of the C/C++ code from `xdvipdfmx`. It
provides I/O services to both the `tectonic_engine_xdvipdfmx` and
`tectonic_engine_xetex` crates through its C/C++ APIs. It does not provide a
Rust API.

This crate deals with general graphics I/O, not just PDF files, but the majority
of its code is PDF-centric.
