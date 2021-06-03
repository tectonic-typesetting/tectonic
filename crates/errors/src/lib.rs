// Copyright 2020 the Tectonic Project.
// Licensed under the MIT License.

#![deny(missing_docs)]

//! Generic error handling for Tectonic.
//!
//! This crate provides a generic boxed error type, plus supporting utilities.
//! In particular:
//!
//! - The basic `Error` type is an `anyhow` 1.x boxed Error
//! - The `atry!` macro allows simple structured annotations to be added to `?`
//!   operations
//! - The `a_ok_or!` macro allows for annotations to `Option::ok_or_else` calls.

use std::{error, fmt, result::Result as StdResult};

/// The generic error type, for complex operations that can fail for a wide
/// range of reasons. This type is a reexport of the `anyhow` 1.x series Error
/// type.
///
/// There’s an appeal to not explicitly committing ourselves to using this
/// particular error implementation, but the `anyhow` error type has a
/// sufficient number of special methods and traits that it would be pretty
/// tedious to re-implement them all while pretending that we're using some
/// different type.
pub use anyhow::Error;

/// A preloaded result type where the error type is our generic error type.
pub use anyhow::Result;

/// The specific version of `anyhow` used by this crate.
pub use anyhow;

/// A simple annotated message that can be attached to errors using the
/// `anyhow::Context` methods, or be used as an error type itself. The
/// recommended way to use this for error context is using `atry!` or related
/// macros.
///
/// The `std::fmt::Display` of this type yields its primary “message”. Consumers
/// that are aware of this type can obtain additional context through its
/// `notes()` method.
#[derive(Debug, Default)]
pub struct AnnotatedMessage {
    /// The main message.
    message: String,

    /// Additional annotations that can be displayed after the primary message.
    notes: Vec<String>,
}

impl AnnotatedMessage {
    /// Set the primary message associated with this annotated report.
    pub fn set_message<T: fmt::Display>(&mut self, m: T) {
        self.message = format!("{}", m);
    }

    /// Add an additional note to be associated with this annotated report.
    pub fn add_note<T: fmt::Display>(&mut self, n: T) {
        self.notes.push(format!("{}", n));
    }

    /// Obtain the set of notes associated with this report.
    pub fn notes(&self) -> &[String] {
        &self.notes[..]
    }
}

impl fmt::Display for AnnotatedMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl error::Error for AnnotatedMessage {}

/// "Annotated try” — like `try!`, but with the ability to add extended context
/// to the error message. This tries to provide a bit more syntactic sugar than
/// anyhow's `with_context()`, and it supports our AnnotatedMessage context type.
///
/// # Example
///
/// ```
/// # use tectonic_errors::{atry, Result};
/// # fn my_fallible_operation(a: bool, b: bool) -> Result<()> { Ok(()) }
/// # fn my_caller() -> Result<()> {
/// # let arg = true;
/// # let option = true;
/// let ok_val = atry!(
///     my_fallible_operation(arg, option);
///     ["the operation on `{}` failed", arg]
///     (note "option was `{}`; maybe you should choose a better value next time", option)
/// );
/// # Ok(())
/// # }
/// ```
///
/// This is equivalent to `let ok_val = my_fallible_operation(arg)?;`, but if the
/// operation fails, the returned error will have a message formatted as per the
/// format expression in square brackets, with attached "note" text formatted as
/// per the parenthesized `note` expression. There may be zero, one, or many notes
/// attached.
#[macro_export]
macro_rules! atry {
    (@aa $ar:ident [ $($inner:tt)+ ] ) => {
        $ar.set_message(format!($($inner)+));
    };

    (@aa $ar:ident ( note $($inner:tt)+ ) ) => {
        $ar.add_note(format!($($inner)+));
    };

    ($op:expr ; $( $annotation:tt )+) => {{
        use $crate::anyhow::Context;
        $op.with_context(|| {
            let mut ar = $crate::AnnotatedMessage::default();
            $(
                atry!(@aa ar $annotation);
            )+
            ar
        })?
    }};
}

/// "annotated ok_or” — like `Option::ok_or_else()?`, but with the ability to
/// add extended context to the error. This yields an `AnnotatedMessage` as its
/// error type.
#[macro_export]
macro_rules! a_ok_or {
    ($option:expr ; $( $annotation:tt )+) => {{
        use $crate::atry;
        $option.ok_or_else(|| {
            let mut ar = $crate::AnnotatedMessage::default();
            $(
                atry!(@aa ar $annotation);
            )+
            ar
        })?
    }};
}

/// A “prelude” module providing a collection of useful names, without
/// causing compiler complaints about the ones you don’t use.
///
/// Provided names are:
///
/// - The core `anyhow::Error` type
/// - The core `anyhow::Result` type, which is `Result<T, anyhow::Error>`
/// - The `anyhow!` macro to create an Error value from a string-format expression
/// - The `bail!` macro: `bail!(...)` = `return Err(anyhow!(...))`
/// - The `ensure!` macro: `ensure!(cond, err)` = `if !cond { bail!(err); }`
/// - The `atry!` macro for annotated question-mark behavior
/// - The `a_ok_or!` macro for annotated, fallibale Option unwrapping
/// - Rust's `std::result::Result` type aliased as StdResult for convenience
pub mod prelude {
    pub use crate::{a_ok_or, atry};
    pub use anyhow::{anyhow, bail, ensure, Error, Result};
    pub use std::result::Result as StdResult;
}

#[cfg(test)]
mod test {
    use super::*;

    fn atry_eval() -> Result<()> {
        let fine: Result<usize> = Ok(0);
        let display = "a string";

        atry!(
            fine;
            ["message {}", display]
            (note "but what about {}", display)
        );
        Ok(())
    }

    #[test]
    fn atry_compile() {
        atry_eval().unwrap();
    }

    fn a_ok_or_eval() -> Result<()> {
        let fine: Option<usize> = Some(0);
        let display = "a string";

        a_ok_or!(
            fine;
            ["message {}", display]
            (note "but what about {}", display)
        );
        Ok(())
    }

    #[test]
    fn a_ok_or_compile() {
        a_ok_or_eval().unwrap();
    }
}
