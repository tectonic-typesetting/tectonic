// src/lib.rs -- main module file for the Tectonic library.
// Copyright 2016-2018 the Tectonic Project
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
//! Because Tectonic is based on the XeTeX engine, it can take advantage of
//! the features of modern fonts (TrueType, OpenType, etc.), outputs directly
//! to the PDF file format, and supports Unicode inputs. Tectonic differs from
//! other TeX engines in the following ways:
//!
//! - Dependencies on environment variables and configuration files have been
//!   eliminated.
//! - All I/O is routed through pluggable backends. Support data can be fetched
//!   from a single “bundle” file, and the engine’s (copious) output can be
//!   hidden or postprocessed.
//! - The command-line frontend, `tectonic`, has a modernized user interface that
//!   never asks for user input.
//! - The frontend is just a thin shim over the Tectonic Rust crate, so that
//!   the full engine can be embedded anywhere you can run Rust code.
//!
//! The main module of this crate provides an all-in-wonder function for compiling
//! LaTeX code to a PDF:
//!
//! ```
//! extern crate tectonic;
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

extern crate aho_corasick;
extern crate app_dirs;
#[macro_use] extern crate error_chain;
extern crate flate2;
extern crate fs2;
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use] extern crate lazy_static;
extern crate libc;
extern crate md5;
extern crate tempfile;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate sha2;
extern crate tectonic_xdv;
extern crate termcolor;
extern crate toml;
extern crate zip;

#[macro_use] pub mod status;
#[macro_use] pub mod errors;
pub mod config;
pub mod digest;
pub mod driver;
pub mod engines;
pub mod io;

// Note: this module is intentionally *not* gated by #[cfg(test)] -- see its
// docstring for details.
#[doc(hidden)]
pub mod test_util;

pub use engines::bibtex::BibtexEngine;
pub use engines::spx2html::Spx2HtmlEngine;
pub use engines::tex::{TexEngine, TexResult};
pub use engines::xdvipdfmx::XdvipdfmxEngine;
pub use errors::{Error, ErrorKind, Result};

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {name: "Tectonic", author: "TectonicProject"};

const FORMAT_SERIAL: u32 = 28; // keep synchronized with tectonic/xetex-constants.h!!


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
    use std::ffi::OsStr;

    let mut status = status::NoopStatusBackend::new();

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
        sb
            .bundle(bundle)
            .primary_input_buffer(latex.as_ref().as_bytes())
            .tex_input_name("texput.tex")
            .format_name("latex")
            .format_cache_path(format_cache_path)
            .keep_logs(false)
            .keep_intermediates(false)
            .print_stdout(false)
            .output_format(driver::OutputFormat::Pdf)
            .do_not_write_output_files();

        let mut sess = ctry!(sb.create(&mut status); "failed to initialize the LaTeX processing session");
        ctry!(sess.run(&mut status); "the LaTeX engine failed");
        sess.into_file_data()
    };

    match files.remove(OsStr::new("texput.pdf")) {
        Some(data) => Ok(data),
        None => Err(errmsg!("LaTeX didn't report failure, but no PDF was created (??)")),
    }
}
