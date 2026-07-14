// Copyright 2016-2020 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic error types and support code.

use std::fmt::Formatter;
use std::{
    fmt::{self, Debug, Display},
    io::{self, Write},
    result::Result as StdResult,
};
pub use tectonic_errors::Error;

cfg_if::cfg_if! {
    if #[cfg(feature = "toml")] {
        /// Alias for [`toml::de::Error`]
        pub type ReadError = toml::de::Error;
        /// Alias for [`toml::ser::Error`]
        pub type WriteError = toml::ser::Error;
    } else {
        use std::error;

        /// Null error to replace lack of `toml` dependency
        #[derive(Debug)]
        pub enum ReadError {}

        impl Display for ReadError {
            fn fmt(&self, _fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                Ok(())
            }
        }

        impl error::Error for ReadError { }

        /// Null error to replace lack of `toml` dependency
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

/// "Chained try" — like `try!`, but with the ability to add context to the error message.
#[macro_export]
macro_rules! ctry {
    ($op:expr ; $( $chain_fmt_args:expr ),*) => {{
        #[allow(unused_imports)]
        use anyhow::Context;
        $op.with_context(|| format!($( $chain_fmt_args ),*))?
    }}
}

/// Error context for when an underlying engine failed catastrophically.
#[derive(Debug)]
pub struct EngineError(&'static str);

impl EngineError {
    /// Create a new [`EngineError`], given the name of an engine
    pub fn new(engine: &'static str) -> EngineError {
        EngineError(engine)
    }

    /// Get the name of the engine for this context
    pub fn engine(&self) -> &str {
        self.0
    }
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "the {} engine had an unrecoverable error", self.0)
    }
}

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
pub fn dump_uncolorized(error: &Error) {
    let mut s = io::stderr();
    writeln!(s, "{:#}", error).expect("write to stderr failed");
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
    /// Return true if two items are definitely the same, false otherwise
    fn definitely_same(&self, other: &Self) -> bool;
}

// Rust currently thinks that this impl conflicts with the one that we
// provide for Result ... I am pretty sure that's not the case since the
// Result PartialEq impl requires that T and E be PartialEq too, whereas
// our definition works for subtypes that are DefinitelySame but
// not PartialEq too.
//
// impl<T: PartialEq> DefinitelySame for T {
//     fn definitely_same(&self, other: &T) -> bool {
//         self == other
//     }
// }
//

impl DefinitelySame for Error {
    /// Hack alert! We only compare stringifications.
    fn definitely_same(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl DefinitelySame for Box<dyn std::error::Error + Send> {
    /// Hack alert! We only compare stringifications.
    fn definitely_same(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl<T: DefinitelySame> DefinitelySame for Option<T> {
    fn definitely_same(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(a), Some(b)) => a.definitely_same(b),
            _ => false,
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn test_def_same_option() {
        let a = Some(Box::from(Error::msg("A")));
        let b = Some(Box::from(Error::msg("A")));

        assert!(a.definitely_same(&b));
        assert!(b.definitely_same(&a));

        let b = Some(Box::from(Error::msg("B")));
        assert!(!a.definitely_same(&b));

        let b = None;
        let c = None;
        assert!(!a.definitely_same(&b));
        assert!(b.definitely_same(&c));
    }

    #[test]
    fn test_def_same_err() {
        let a = Error::msg("A");
        let b = Error::msg("A");

        // Different backtraces but should be same
        assert!(a.definitely_same(&b));

        let b = Error::msg("B");
        assert!(!a.definitely_same(&b));

        let a = Error::msg("A");
        assert!(!a.definitely_same(&b));

        let b = Error::msg("A");
        assert!(a.definitely_same(&b));
    }

    #[test]
    fn test_def_same_box_err() {
        let a: Box<dyn StdError + Send> = Box::from(Error::msg("A"));
        let b: Box<dyn StdError + Send> = Box::from(Error::msg("B"));

        assert!(a.definitely_same(&a));
        assert!(!a.definitely_same(&b));
    }
}
