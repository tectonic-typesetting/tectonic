// src/errors.rs -- error types
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

use flate2;
use std::io;
use zip::result::ZipError;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Flate2(flate2::DataError);
        IO(io::Error);
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
    }
}
