// src/errors.rs -- error types
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use app_dirs;
use flate2;
use hyper;
use std::{convert, ffi, io, num, str};
use std::io::Write;
use toml;
use zip::result::ZipError;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        AppDirs(app_dirs::AppDirsError);
        Flate2(flate2::DataError);
        Hyper(hyper::Error);
        Io(io::Error);
        Nul(ffi::NulError);
        ParseInt(num::ParseIntError);
        TomlDe(toml::de::Error);
        Utf8(str::Utf8Error);
        Zip(ZipError);
    }

    errors {
        NotSeekable {
            description("this stream is not seekable")
            display("this stream is not seekable")
        }

        NotSizeable {
            description("the size of this stream cannot be determined")
            display("the size of this stream cannot be determined")
        }

        PathForbidden(path: String) {
            description("access to this file path is forbidden")
            display("access to the path {} is forbidden", path)
        }
    }
}


impl convert::From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, format!("{}", err))
    }
}


impl Error {
    /// Write the information contained in this object to standard error in a
    /// somewhat user-friendly form.
    ///
    /// The `error_chain` crate provides a Display impl for its Error objects
    /// that ought to provide this functionality, but I have had enormous
    /// trouble being able to use it. So instead we emulate their code. This
    /// function is also paralleled by the implementation in
    /// `status::termcolor::TermcolorStatusBackend`, which adds the sugar of
    /// providing nice colorization if possible. This function should only be
    /// used if a `StatusBackend` is not yet available in the running program.
    pub fn dump_uncolorized(&self) {
        let mut prefix = "error:";
        let mut s = io::stderr();

        for item in self.iter() {
            writeln!(s, "{} {}", prefix, item).expect("write to stderr failed");
            prefix = "caused by:";
        }

        if let Some(backtrace) = self.backtrace() {
            writeln!(s, "debugging: backtrace follows:").expect("write to stderr failed");
            writeln!(s, "{:?}", backtrace).expect("write to stderr failed");
        }
    }
}
