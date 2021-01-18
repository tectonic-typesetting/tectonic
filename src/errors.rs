// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic error types and support code.

// Silence warnings due to `error-chain` using `Error::cause` instead of `Error::source`. The
// former was [deprecated in Rust 1.33](https://github.com/rust-lang/rust/pull/53533). The fix for
// `error-chain` is in <https://github.com/rust-lang-nursery/error-chain/pull/255> and will
// hopefully show up in a future version.
#![allow(deprecated)]

use error_chain::error_chain;
use std::{
    convert, ffi,
    fmt::{self, Debug, Display},
    io,
    io::Write,
    num,
    result::Result as StdResult,
    str,
    sync::{Arc, Mutex, Weak},
};
use tectonic_errors::Error as NewError;
use zip::result::ZipError;

cfg_if::cfg_if! {
    if #[cfg(feature = "toml")] {
        pub type ReadError = toml::de::Error;
        pub type WriteError = toml::ser::Error;
    } else {
        use std::error;

        #[derive(Debug)]
        pub enum ReadError {}

        impl Display for ReadError {
            fn fmt(&self, _fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                Ok(())
            }
        }

        impl error::Error for ReadError { }

        #[derive(Debug)]
        pub enum WriteError {}

        impl Display for WriteError {
            fn fmt(&self, _fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        ConfigRead(ReadError);
        ConfigWrite(WriteError);
        NewStyle(NewError);
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
    }
}

/// `chain_err` compatibility between our old and new error types
pub trait ChainErrCompatExt<T> {
    fn chain_err<F, K>(self, chainer: F) -> Result<T>
    where
        F: FnOnce() -> K,
        K: Into<ErrorKind>;
}

impl<T> ChainErrCompatExt<T> for StdResult<T, NewError> {
    fn chain_err<F, K>(self, chainer: F) -> Result<T>
    where
        F: FnOnce() -> K,
        K: Into<ErrorKind>,
    {
        self.map_err(|e| {
            let e: Error = e.into();
            e.chain_err(chainer)
        })
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
        #[allow(unused_imports)]
        use $crate::errors::{ChainErrCompatExt, ResultExt};
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
        match self {
            ErrorKind::Msg(ref s) => {
                if let ErrorKind::Msg(ref o) = *other {
                    s == o
                } else {
                    false
                }
            }

            // Hacky for tex-outputs test
            ErrorKind::NewStyle(ref s) => {
                if let ErrorKind::NewStyle(ref o) = *other {
                    s.to_string() == o.to_string()
                } else {
                    false
                }
            }

            _ => false,
        }
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

// SyncError copied from:
// https://github.com/rust-lang-nursery/error-chain/issues/240
//
// This is needed to be able to turn error_chain errors into anyhow errors,
// since the latter requires that they are sync.

pub struct SyncError<T: 'static> {
    inner: Arc<Mutex<T>>,
    proxy: Option<CauseProxy<T>>,
}

impl<T: std::error::Error + 'static> SyncError<T> {
    pub fn new(err: T) -> Self {
        let arc = Arc::new(Mutex::new(err));
        let proxy = match arc.lock().unwrap().source() {
            None => None,
            Some(source) => Some(CauseProxy::new(source, Arc::downgrade(&arc), 0)),
        };

        SyncError { inner: arc, proxy }
    }

    pub fn maybe<R>(r: std::result::Result<R, T>) -> std::result::Result<R, Self> {
        match r {
            Ok(v) => Ok(v),
            Err(e) => Err(SyncError::new(e)),
        }
    }

    pub fn unwrap(self) -> T {
        Arc::try_unwrap(self.inner).unwrap().into_inner().unwrap()
    }
}

impl<T: std::error::Error + 'static> std::error::Error for SyncError<T> {
    #[cfg(backtrace)]
    fn backtrace(&self) -> Option<&Backtrace> {}

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.proxy.as_ref().map(|e| e as _)
    }
}

impl<T> Display for SyncError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.lock().unwrap().fmt(f)
    }
}

impl<T> Debug for SyncError<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.lock().unwrap().fmt(f)
    }
}

struct CauseProxy<T: 'static> {
    inner: Weak<Mutex<T>>,
    next: Option<Box<CauseProxy<T>>>,
    depth: u32,
}

impl<T: std::error::Error> CauseProxy<T> {
    fn new(err: &dyn std::error::Error, weak: Weak<Mutex<T>>, depth: u32) -> Self {
        // Can't allocate an object, or mutate the proxy safely during source(),
        // so we just take the hit at construction, recursively. We can't hold
        // references outside the mutex at all, so instead we remember how many
        // steps to get to this proxy. And if some error chain plays tricks, the
        // user gets both pieces.
        CauseProxy {
            inner: weak.clone(),
            depth,
            next: match err.source() {
                None => None,
                Some(source) => Some(Box::new(CauseProxy::new(source, weak, depth + 1))),
            },
        }
    }

    fn with_instance<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&(dyn std::error::Error + 'static)) -> R,
    {
        let arc = self.inner.upgrade().unwrap();
        {
            let e = arc.lock().unwrap();
            let mut source = e.source().unwrap();
            for _ in 0..self.depth {
                source = source.source().unwrap();
            }
            f(source)
        }
    }
}

impl<T: std::error::Error + 'static> std::error::Error for CauseProxy<T> {
    #[cfg(backtrace)]
    fn backtrace(&self) -> Option<&Backtrace> {}

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.next.as_ref().map(|e| e as _)
    }
}

impl<T> Display for CauseProxy<T>
where
    T: Display + std::error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with_instance(|i| std::fmt::Display::fmt(&i, f))
    }
}

impl<T> Debug for CauseProxy<T>
where
    T: Debug + std::error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with_instance(|i| std::fmt::Debug::fmt(&i, f))
    }
}
