// src/status/mod.rs -- communicating status updates to the user
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

#[macro_use] pub mod termcolor;

use std::cmp;
use std::fmt::Arguments;


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


pub trait StatusBackend {
    fn note(&mut self, args: Arguments);
    fn warning(&mut self, args: Arguments);
    fn error(&mut self, args: Arguments);
}

#[macro_export]
macro_rules! tt_note {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.note(format_args!($( $fmt_args ),*))
    };
}

#[macro_export]
macro_rules! tt_warning {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.warning(format_args!($( $fmt_args ),*))
    };
}

#[macro_export]
macro_rules! tt_error {
    ($dest:expr, $( $fmt_args:expr ),*) => {
        $dest.error(format_args!($( $fmt_args ),*))
    };
}


pub struct NoopStatusBackend { }

impl NoopStatusBackend {
    pub fn new() -> NoopStatusBackend {
        NoopStatusBackend { }
    }
}

impl StatusBackend for NoopStatusBackend {
    fn note(&mut self, _args: Arguments) {}
    fn warning(&mut self, _args: Arguments) {}
    fn error(&mut self, _args: Arguments) {}
}
