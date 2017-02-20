// src/errors.rs -- error types
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use app_dirs;
use flate2;
use hyper;
use std::{convert, ffi, io, num, str};
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
        CacheError(t: String) {
            description("an error with the local resource cache")
            display("{}", t)
        }

        ConfigError(t: String) {
            description("an error in the configuration file")
            display("{}", t)
        }

        DpxError(t: String) {
            description("an error reported by the xdvipdfmx engine")
            display("{}", t)
        }

        NotSeekable {
            description("this stream is not seekable")
            display("this stream is not seekable")
        }

        NotSizeable {
            description("the size of this stream cannot be determined")
            display("the size of this stream cannot be determined")
        }

        PathForbidden(t: String) {
            description("access to this file path is forbidden")
            display("access to the path {} is forbidden", t)
        }

        TexError(t: String) {
            description("an error reported by the TeX engine")
            display("{}", t)
        }
    }
}


impl convert::From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, format!("{}", err))
    }
}
