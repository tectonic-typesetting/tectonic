# tectonic_engine_xdvipdfmx 0.1.0 (2021-06-03)

This crate introduces the `xdvipdfmx` engine as a standalone crate, building on
the new "core bridge" functionality. The API is iterated somewhat from the one
that was previously provided in the main `tectonic` crate.

Note that the vast majority of the `xdvipdfmx` C/C++ code is found in the new
`tectonic_pdf_io` crate, because both this crate and the XeTeX engine need to
share library routines to do I/O on PDF files.
