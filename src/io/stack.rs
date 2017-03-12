// src/io/stack.rs -- a stack of other IoProviders
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};

use status::StatusBackend;
use super::{InputHandle, IoProvider, OpenResult, OutputHandle};


#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum AccessPattern {
    /// This file is only ever read.
    Read,

    /// This file is only ever written. This suggests that it is
    /// a final output of the processing session.
    Written,

    /// This file is read, then written. We call this a "circular" access
    /// pattern. Multiple passes of an engine will result in outputs that
    /// change if this file's contents change, or if the file did not exist at
    /// the time of the first pass.
    ReadThenWritten,

    /// This file is written, then read. We call this a "temporary" access
    /// pattern. This file is likely a temporary buffer that is not of
    /// interest to the user.
    WrittenThenRead,
}

/// An IoStack is an IoProvider that delegates to an ordered list of
/// subordinate IoProviders. It also checks the order in which files are read
/// and written to detect "circular" access patterns that indicate whether we
/// need to run multiple passes of the TeX engine.

pub struct IoStack<'a> {
    items: Vec<&'a mut IoProvider>,
    access_patterns: Option<&'a mut HashMap<OsString, AccessPattern>>,
}


impl<'a> IoStack<'a> {
    pub fn new(items: Vec<&'a mut IoProvider>, access_patterns: Option<&'a mut HashMap<OsString, AccessPattern>>) -> IoStack<'a> {
        IoStack {
            items: items,
            access_patterns: access_patterns,
        }
    }
}


impl<'a> IoProvider for IoStack<'a> {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        for item in self.items.iter_mut() {
            let r = item.output_open_name(name);

            match r {
                OpenResult::NotAvailable => continue,
                OpenResult::Ok(_) => {
                    if let Some(ref mut access_patterns) = self.access_patterns {
                        let new_pat = if let Some(cur_pat) = access_patterns.get(name) {
                            match cur_pat {
                                &AccessPattern::Read => AccessPattern::ReadThenWritten,
                                c => *c, // identity mapping makes sense for remaining options
                            }
                        } else {
                            AccessPattern::Written
                        };
                        access_patterns.insert(name.to_os_string(), new_pat);
                    }
                    return r;
                },
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        for item in self.items.iter_mut() {
            let r = item.output_open_stdout();

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_name(&mut self, name: &OsStr, status: &mut StatusBackend) -> OpenResult<InputHandle> {
        for item in self.items.iter_mut() {
            let r = item.input_open_name(name, status);

            match r {
                OpenResult::NotAvailable => continue,
                OpenResult::Ok(_) => {
                    if let Some(ref mut access_patterns) = self.access_patterns {
                        let new_pat = if let Some(cur_pat) = access_patterns.get(name) {
                            match cur_pat {
                                &AccessPattern::Written => AccessPattern::WrittenThenRead,
                                c => *c, // identity mapping makes sense for remaining options
                            }
                        } else {
                            AccessPattern::Read
                        };
                        access_patterns.insert(name.to_os_string(), new_pat);
                    }
                    return r;
                },
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }
}
