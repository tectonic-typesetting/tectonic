// src/status/mod.rs -- communicating status updates to the user
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

pub mod termcolor;

use std::cmp;


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
    fn info(&mut self, message: &str);
    fn warning(&mut self, message: &str);
    fn error(&mut self, message: &str);
}


pub struct NoopStatusBackend { }

impl NoopStatusBackend {
    pub fn new() -> NoopStatusBackend {
        NoopStatusBackend { }
    }
}

impl StatusBackend for NoopStatusBackend {
    fn info(&mut self, _message: &str) {}
    fn warning(&mut self, _message: &str) {}
    fn error(&mut self, _message: &str) {}
}
