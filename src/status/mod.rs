// src/status/mod.rs -- communicating status updates to the user
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

#[macro_use] pub mod termcolor;

use std::cmp;
use std::fmt::Arguments;

use errors::Error;


#[repr(usize)]
#[derive(Clone, Copy, Eq, Debug)]
pub enum ChatterLevel {
    Minimal = 0,
    Normal,
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
    fn report(&mut self, kind: MessageKind, args: Arguments, err: Option<&Error>);
}

#[macro_export]
macro_rules! tt_note {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.report($crate::status::MessageKind::Note, format_args!($( $fmt_args ),*), None)
    };
    ($dest:expr, $( $fmt_args:expr ),* ; $err:expr) => {
        $dest.report($crate::status::MessageKind::Note, format_args!($( $fmt_args ),*), Some(&$err))
    };
}

#[macro_export]
macro_rules! tt_warning {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.report($crate::status::MessageKind::Warning, format_args!($( $fmt_args ),*), None)
    };
    ($dest:expr, $( $fmt_args:expr ),* ; $err:expr) => {
        $dest.report($crate::status::MessageKind::Warning, format_args!($( $fmt_args ),*), Some(&$err))
    };
}

#[macro_export]
macro_rules! tt_error {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.report($crate::status::MessageKind::Error, format_args!($( $fmt_args ),*), None)
    };
    ($dest:expr, $( $fmt_args:expr ),* ; $err:expr) => {
        $dest.report($crate::status::MessageKind::Error, format_args!($( $fmt_args ),*), Some(&$err))
    };
}


pub struct NoopStatusBackend { }

impl NoopStatusBackend {
    pub fn new() -> NoopStatusBackend {
        NoopStatusBackend { }
    }
}

impl StatusBackend for NoopStatusBackend {
    fn report(&mut self, _kind: MessageKind, _args: Arguments, _err: Option<&Error>) {}
}
