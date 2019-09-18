// src/status/mod.rs -- communicating status updates to the user
// Copyright 2017-2018 the Tectonic Project
// Licensed under the MIT License.

//! A framework for showing status messages to the user.

pub mod termcolor;

use std::cmp;
use std::fmt::Arguments;
use std::result::Result as StdResult;
use std::str::FromStr;

use crate::errors::Error;

#[repr(usize)]
#[derive(Clone, Copy, Eq, Debug)]
pub enum ChatterLevel {
    Minimal = 0,
    Normal,
}

impl FromStr for ChatterLevel {
    type Err = &'static str;

    fn from_str(a_str: &str) -> StdResult<Self, Self::Err> {
        match a_str {
            "default" => Ok(ChatterLevel::Normal),
            "minimal" => Ok(ChatterLevel::Minimal),
            _ => Err("unsupported or unknown chatter level"),
        }
    }
}

impl PartialEq for ChatterLevel {
    #[inline]
    fn eq(&self, other: &ChatterLevel) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd for ChatterLevel {
    #[inline]
    fn partial_cmp(&self, other: &ChatterLevel) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChatterLevel {
    #[inline]
    fn cmp(&self, other: &ChatterLevel) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageKind {
    Note,
    Warning,
    Error,
}

pub trait StatusBackend {
    /// Report a message to the status backend.
    fn report(&mut self, kind: MessageKind, args: Arguments, err: Option<&Error>);

    /// Issue a note-level status, idealy highlighting a particular phrase.
    ///
    /// This is a bit of a hack. For [`driver::ProcessingSession::run`], I
    /// like the UX when we issue notes in this style. It's a bit more
    /// high-level than intended for this trait, but we can provide a nice
    /// sensible default implementation, so whatever.
    fn note_highlighted(&mut self, before: &str, highlighted: &str, after: &str) {
        self.report(
            MessageKind::Note,
            format_args!("{}{}{}", before, highlighted, after),
            None,
        )
    }
}

/// Report a formatted informational message to the user.
///
/// An `Error` object may be provided, in which case it will be shown to the
/// user as well. Generally, though, one would expect to use `tt_warning!` or
/// `tt_error!` if there’s an Error available.
#[macro_export]
macro_rules! tt_note {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.report($crate::status::MessageKind::Note, format_args!($( $fmt_args ),*), None)
    };
    ($dest:expr, $( $fmt_args:expr ),* ; $err:expr) => {
        $dest.report($crate::status::MessageKind::Note, format_args!($( $fmt_args ),*), Some(&$err))
    };
}

/// Report a formatted warning message to the user.
///
/// An `Error` object may be provided, in which case it will be shown to the
/// user as well.
#[macro_export]
macro_rules! tt_warning {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.report($crate::status::MessageKind::Warning, format_args!($( $fmt_args ),*), None)
    };
    ($dest:expr, $( $fmt_args:expr ),* ; $err:expr) => {
        $dest.report($crate::status::MessageKind::Warning, format_args!($( $fmt_args ),*), Some(&$err))
    };
}

/// Report a formatted error message to the user.
///
/// An `Error` object may be provided, in which case it will be shown to the
/// user as well.
#[macro_export]
macro_rules! tt_error {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.report($crate::status::MessageKind::Error, format_args!($( $fmt_args ),*), None)
    };
    ($dest:expr, $( $fmt_args:expr ),* ; $err:expr) => {
        $dest.report($crate::status::MessageKind::Error, format_args!($( $fmt_args ),*), Some(&$err))
    };
}

#[derive(Default)]
pub struct NoopStatusBackend {}

impl NoopStatusBackend {
    pub fn new() -> NoopStatusBackend {
        Default::default()
    }
}

impl StatusBackend for NoopStatusBackend {
    fn report(&mut self, _kind: MessageKind, _args: Arguments, _err: Option<&Error>) {}
}
