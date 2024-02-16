// src/lib.rs -- main module file for the Tectonic library.
// Copyright 2016-2022 the Tectonic Project
// Licensed under the MIT License.

#![recursion_limit = "1024"] // "error_chain can recurse deeply"

//! Tectonic is a complete
//! [TeX](https://www.tug.org/)/[LaTeX](https://www.latex-project.org/) engine
//! converted into a standalone library. It is derived from the
//! [XeTeX](http://xetex.sourceforge.net/) variant of TeX and uses the support
//! files packages by the [TeX Live](https://www.tug.org/texlive/) project.
//! Tectonic would not be possible without the hard work that has gone into
//! these projects.
//!
//! Because Tectonic is based on the XeTeX engine, it can take advantage of the
//! features of modern fonts (TrueType, OpenType, etc.), outputs directly to the
//! PDF file format, and supports Unicode inputs. Most importantly, the TeX
//! experience delivered by Tectonic is completely embeddable: if you link with
//! this crate you can fully process TeX documents, from source to PDF, without
//! relying on any externally installed software, configuration, or resource
//! files. This is possible because Tectonic bundles the traditional TeX tools
//! and routes their I/O through a pluggable backend system.
//!
//! This crate delivers command-line frontend, `tectonic`, that has a modernized
//! user experience that hides TeX’s copious output (by default) and never asks
//! for user input. Virtually all of the functionality of the frontend is
//! accessible programmatically through the [`driver`] module of this crate.
//!
//! This crate joins a set of sub-crates that combine to provide the Tectonic
//! user experience. Those crates generally have APIs that are more carefully
//! structured and better documented than this crate, which grew somewhat
//! organically. The foundational crates are:
//!
//! - [`tectonic_errors`](https://docs.rs/tectonic_errors) for core error
//!   handing types.
//! - [`tectonic_status_base`](https://docs.rs/tectonic_status_base) for a basic
//!   user-facing status-reporting framework.
//! - [`tectonic_io_base`](https://docs.rs/tectonic_io_base) for the I/O
//!   abstraction framework.
//! - [`tectonic_bridge_core`](https://docs.rs/tectonic_bridge_core) for a
//!   framework to launch unsafe (C/C++) "engines" through [FFI].
//!
//! [FFI]: https://doc.rust-lang.org/std/ffi/index.html
//!
//! Building on these and other support crates of less general interest are the
//! following major pieces of Tectonic’s functionality:
//!
//! - [`tectonic_bundles`](https://docs.rs/tectonic_bundles) for the"bundles" of
//!   TeX support files underlying Tectonic processing.
//! - [`tectonic_docmodel`](https://docs.rs/tectonic_docmodel) for the Tectonic
//!   "document model" expressed in `Tectonic.toml` files.
//! - [`tectonic_engine_xetex`](https://docs.rs/tectonic_engine_xetex) for the
//!   XeTeX engine.
//! - [`tectonic_engine_xdvipdfmx`](https://docs.rs/tectonic_engine_xdvipdfmx)
//!   for the `xdvipdfmx` engine.
//! - [`tectonic_engine_bibtex`](https://docs.rs/tectonic_engine_bibtex) for the
//!   BibTeX engine.
//!
//! The main module of this crate provides an all-in-wonder function for
//! compiling LaTeX code to a PDF:
//!
//! ```
//! use tectonic;
//!
//! let latex = r#"
//! \documentclass{article}
//! \begin{document}
//! Hello, world!
//! \end{document}
//! "#;
//!
//! # tectonic::test_util::activate_test_mode_augmented(env!("CARGO_MANIFEST_DIR"));
//! let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).expect("processing failed");
//! println!("Output PDF size is {} bytes", pdf_data.len());
//! ```
//!
//! The [`driver`] module provides a high-level interface for driving the
//! engines in more realistic circumstances.

pub mod config;
pub mod digest;
#[cfg(feature = "serialization")]
pub mod docmodel;
pub mod driver;
pub mod engines;
pub mod errors;
pub mod io;
pub mod status;
pub mod unstable_opts;

// Note: this module is intentionally *not* gated by #[cfg(test)] -- see its
// docstring for details.
#[doc(hidden)]
pub mod test_util;

pub use crate::engines::bibtex::BibtexEngine;
pub use crate::engines::spx2html::Spx2HtmlEngine;
pub use crate::engines::tex::{TexEngine, TexOutcome};
pub use crate::engines::xdvipdfmx::XdvipdfmxEngine;
pub use crate::errors::{Error, ErrorKind, Result};

// Convenienece re-exports for migration into our multi-crate setup
pub use tectonic_engine_xetex::FORMAT_SERIAL;
pub use tectonic_status_base::{tt_error, tt_note, tt_warning};

/// Compile LaTeX text to a PDF.
///
/// This function is an all-in-one interface to the main Tectonic workflow. Given
/// a string representing a LaTeX input file, it will compile it to a PDF and return
/// a byte vector corresponding to the resulting file:
///
/// ```
/// let latex = r#"
/// \documentclass{article}
/// \begin{document}
/// Hello, world!
/// \end{document}
/// "#;
///
/// # tectonic::test_util::activate_test_mode_augmented(env!("CARGO_MANIFEST_DIR"));
/// let pdf_data: Vec<u8> = tectonic::latex_to_pdf(latex).expect("processing failed");
/// println!("Output PDF size is {} bytes", pdf_data.len());
/// ```
///
/// The compilation uses the default bundle, the location of which is embedded
/// in the crate or potentially specified in the user’s configuration file.
/// The current working directory will be searched for any `\\input` files.
/// Messages aimed at the user are suppressed, but (in the default
/// configuration) network I/O may occur to pull down needed resource files.
/// No outputs are written to disk; all supporting files besides the PDF
/// document are discarded. The XeTeX engine is run multiple times if needed
/// to get the output file to converge.
///
/// For more sophisticated uses, use the [`driver`] module, which provides a
/// high-level interface for driving the typesetting engines with much more
/// control over their behavior.
///
/// Note that the current engine implementations use lots of global state, so
/// they are not thread-safe. This crate uses a global mutex to serialize
/// invocations of the engines. This means that if you call this function from
/// multiple threads simultaneously, the bulk of the work will be done in
/// serial. The aim is to lift this limitation one day, but it will require
/// extensive work on the underlying C/C++ code.
pub fn latex_to_pdf<T: AsRef<str>>(latex: T) -> Result<Vec<u8>> {
    let mut status = status::NoopStatusBackend::default();

    let auto_create_config_file = false;
    let config = ctry!(config::PersistentConfig::open(auto_create_config_file);
                       "failed to open the default configuration file");

    let only_cached = false;
    let bundle = ctry!(config.default_bundle(only_cached, &mut status);
                       "failed to load the default resource bundle");

    let format_cache_path = ctry!(config.format_cache_path();
                                  "failed to set up the format cache");

    let mut files = {
        // Looking forward to non-lexical lifetimes!
        let mut sb = driver::ProcessingSessionBuilder::default();
        sb.bundle(bundle)
            .primary_input_buffer(latex.as_ref().as_bytes())
            .tex_input_name("texput.tex")
            .format_name("latex")
            .format_cache_path(format_cache_path)
            .keep_logs(false)
            .keep_intermediates(false)
            .print_stdout(false)
            .output_format(driver::OutputFormat::Pdf)
            .do_not_write_output_files();

        let mut sess =
            ctry!(sb.create(&mut status); "failed to initialize the LaTeX processing session");
        ctry!(sess.run(&mut status); "the LaTeX engine failed");
        sess.into_file_data()
    };

    match files.remove("texput.pdf") {
        Some(file) => Ok(file.data),
        None => Err(errmsg!(
            "LaTeX didn't report failure, but no PDF was created (??)"
        )),
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[allow(unused_must_use)]
    #[cfg(target_os = "linux")]
    fn no_segfault_after_failed_compilation() {
        /*
            This is mostly relevant when using tectonic as a library.
            After a compilation error xetex assumes the process will exit so
            it doesn't fully cleanup its auxiliary structures. User some
            conditions (like using fontconfig), compiling afterwards results in
            a segmentation fault.
            This test has no assertions because the simple fact that it didn't
            crash the test runner means it succeeded.
        */
        for _ in 0..2 {
            latex_to_pdf(
                r"\documentclass{article}
\usepackage{fontspec}
\setmainfont{Ubuntu Mono}
\begin{document}
\invalidcommand{}
\end{document}",
            );
        }
    }
}
