// src/io/stack.rs -- a stack of other IoProviders
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};

use digest::DigestData;
use status::StatusBackend;
use super::{InputHandle, IoProvider, OpenResult, OutputHandle};

/// Different patterns with which files may have been accessed by the
/// underlying engines. Once a file is marked as ReadThenWritten or
/// WrittenThenRead, its pattern does not evolve further.
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

/// A summary of the I/O that happened on a file. We record its access
/// pattern, the cryptographic digest of the file when it was last read, and
/// the cryptographic digest of the file as it was last written.
#[derive(Clone,Debug,Eq,PartialEq)]
pub struct FileSummary {
    access_pattern: AccessPattern,
    read_digest: Option<DigestData>,
    write_digest: Option<DigestData>,
}

impl FileSummary {
    pub fn new(access_pattern: AccessPattern) -> FileSummary {
        FileSummary {
            access_pattern: access_pattern,
            read_digest: None,
            write_digest: None,
        }
    }
}


/// An IoStack is an IoProvider that delegates to an ordered list of
/// subordinate IoProviders. It also checks the order in which files are read
/// and written to detect "circular" access patterns that indicate whether we
/// need to run multiple passes of the TeX engine.

pub struct IoStack<'a> {
    items: Vec<&'a mut IoProvider>,
    summaries: Option<&'a mut HashMap<OsString, FileSummary>>,
}


impl<'a> IoStack<'a> {
    pub fn new(items: Vec<&'a mut IoProvider>, summaries: Option<&'a mut HashMap<OsString, FileSummary>>) -> IoStack<'a> {
        IoStack {
            items: items,
            summaries: summaries,
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
                    if let Some(ref mut summaries) = self.summaries {
                        // Borrow-checker fight.
                        if {
                            if let Some(summ) = summaries.get_mut(name) {
                                summ.access_pattern = match summ.access_pattern {
                                    AccessPattern::Read => AccessPattern::ReadThenWritten,
                                    c => c, // identity mapping makes sense for remaining options
                                };
                                false // no, do not insert a new item
                            } else {
                                true // yes, insert a new item
                            }
                        } {
                            // The 'else' branch above returned 'true'.
                            summaries.insert(name.to_os_string(), FileSummary::new(AccessPattern::Written));
                        }
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
                    if let Some(ref mut summaries) = self.summaries {
                        // See explanation above.
                        if {
                            if let Some(summ) = summaries.get_mut(name) {
                                summ.access_pattern = match summ.access_pattern {
                                    AccessPattern::Written => AccessPattern::WrittenThenRead,
                                    c => c, // identity mapping makes sense for remaining options
                                };
                                false
                            } else {
                                true
                            }
                        } {
                            summaries.insert(name.to_os_string(), FileSummary::new(AccessPattern::Read));
                        }
                    }
                    return r;
                },
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }
}
