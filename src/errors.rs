// src/errors.rs -- error types
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic error types and support code.

// Silence warnings due to `error-chain` using `Error::cause` instead of `Error::source`. The
// former was [deprecated in Rust 1.33](https://github.com/rust-lang/rust/pull/53533). The fix for
// `error-chain` is in <https://github.com/rust-lang-nursery/error-chain/pull/255> and will
// hopefully show up in a future version.
#![allow(deprecated)]

use error_chain::error_chain;
use reqwest::StatusCode;
use std::io::Write;
use std::result::Result as StdResult;
use std::{convert, ffi, io, num, str};
use zip::result::ZipError;

cfg_if::cfg_if! {
    if #[cfg(feature = "toml")] {
        pub type ReadError = toml::de::Error;
        pub type WriteError = toml::ser::Error;
    } else {
        use std::fmt::{self, Formatter, Display};
        use std::error;

        #[derive(Debug)]
        pub enum ReadError {}

        impl Display for ReadError {
            fn fmt(&self, _fmt: &mut Formatter<'_>) -> fmt::Result {
                Ok(())
            }
        }

        impl error::Error for ReadError { }

        #[derive(Debug)]
        pub enum WriteError {}

        impl Display for WriteError {
            fn fmt(&self, _fmt: &mut Formatter<'_>) -> fmt::Result {
                Ok(())
            }
        }

        impl error::Error for WriteError { }
    }
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        AppDirs(app_dirs::AppDirsError);
        Io(io::Error);
        Nul(ffi::NulError);
        ParseInt(num::ParseIntError);
        Persist(tempfile::PersistError);
        Reqwest(reqwest::Error);
        ConfigRead(ReadError);
        ConfigWrite(WriteError);
        Time(std::time::SystemTimeError);
        Utf8(str::Utf8Error);
        Xdv(tectonic_xdv::XdvError);
        Zip(ZipError);
    }

    errors {
        BadLength(expected: usize, observed: usize) {
            description("the item is not the expected length")
            display("expected length {}; found {}", expected, observed)
        }

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

        EngineError(engine: &'static str) {
            description("some engine had an unrecoverable error")
            display("the {} engine had an unrecoverable error", engine)
        }

        UnexpectedHttpResponse(url: String, status: StatusCode) {
            description("unexpected HTTP response to URL")
            display("unexpected HTTP response to URL {}: {}", url, status)
        }
    }
}

/// Use string formatting to create an `Error` of kind
/// `errors::ErrorKind::Msg`.
#[macro_export]
macro_rules! errmsg {
    ($( $fmt_args:expr ),*) => {
        $crate::errors::ErrorKind::Msg(format!($( $fmt_args ),*)).into()
    };
}

/// “Chained try” — like `try!`, but with the ability to add context to the error message.
#[macro_export]
macro_rules! ctry {
    ($op:expr ; $( $chain_fmt_args:expr ),*) => {{
        use $crate::errors::ResultExt;
        $op.chain_err(|| format!($( $chain_fmt_args ),*))?
    }}
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

/// The DefinitelySame trait is a helper trait implemented because Errors do
/// not generically implement PartialEq. This is a bit of a drag for testing
/// since it's nice to be able to check if an error matches the one that's
/// expected. DefinitelySame addresses this by providing a weak equivalence
/// test: definitely_same() returns true if the two values definitely are
/// equivalent, and false otherwise. This can happen if the value are known to
/// be different, but also if we can't tell. It doesn't cover all cases, but
/// it does cover the ones that come up in our test suite.

pub trait DefinitelySame {
    fn definitely_same(&self, other: &Self) -> bool;
}

// Rust currently thinks that this impl conflicts with the one that we
// provide for Result ... I am pretty sure that's not the case since the
// Result PartialEq impl requires that T and E be PartialEq too, whereas
// our definition works for subtypes that are DefinitelySame but
// not PartialEq too.
//
//impl<T: PartialEq> DefinitelySame for T {
//    fn definitely_same(&self, other: &T) -> bool {
//        self == other
//    }
//}

impl DefinitelySame for ErrorKind {
    fn definitely_same(&self, other: &Self) -> bool {
        if let ErrorKind::Msg(ref s) = *self {
            return if let ErrorKind::Msg(ref o) = *other {
                s == o
            } else {
                false
            };
        }

        false
    }
}

impl DefinitelySame for Error {
    /// Here we abuse DefinitelySame a bit and ignore the backtrace etc.
    fn definitely_same(&self, other: &Self) -> bool {
        self.kind().definitely_same(other.kind())
    }
}

impl<T: DefinitelySame, E: DefinitelySame> DefinitelySame for StdResult<T, E> {
    fn definitely_same(&self, other: &Self) -> bool {
        match *self {
            Ok(ref st) => {
                if let Ok(ref ot) = *other {
                    st.definitely_same(ot)
                } else {
                    false
                }
            }
            Err(ref se) => {
                if let Err(ref oe) = *other {
                    se.definitely_same(oe)
                } else {
                    false
                }
            }
        }
    }
}
