// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! An "I/O stack" is an I/O provider that delegates requests to
//! a series of sub-providers in turn.

use std::path::PathBuf;

use super::{InputHandle, IoProvider, OpenResult, OutputHandle};

/// An IoStack is an IoProvider that delegates to an ordered list of
/// subordinate IoProviders. It also checks the order in which files are read
/// and written to detect "circular" access patterns that indicate whether we
/// need to run multiple passes of the TeX engine.
// Note: needs a manual debug implementation?
pub struct IoStack<'a> {
    items: Vec<&'a mut dyn IoProvider>,
}

impl<'a> IoStack<'a> {
    /// Create a new I/O stack.
    pub fn new(items: Vec<&'a mut dyn IoProvider>) -> IoStack<'a> {
        IoStack { items }
    }
}

impl<'a> IoProvider for IoStack<'a> {
    fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
        for item in &mut self.items {
            let r = item.output_open_name(name);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r,
            };
        }

        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        for item in &mut self.items {
            let r = item.output_open_stdout();

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r,
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_name(&mut self, name: &str) -> OpenResult<InputHandle> {
        for item in &mut self.items {
            let r = item.input_open_name(name);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r,
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_name_with_abspath(
        &mut self,
        name: &str,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        for item in &mut self.items {
            let r = item.input_open_name_with_abspath(name);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r,
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_primary(&mut self) -> OpenResult<InputHandle> {
        for item in &mut self.items {
            let r = item.input_open_primary();

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r,
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_primary_with_abspath(&mut self) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        for item in &mut self.items {
            let r = item.input_open_primary_with_abspath();

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r,
            };
        }

        OpenResult::NotAvailable
    }

    fn input_open_format(&mut self, name: &str) -> OpenResult<InputHandle> {
        for item in &mut self.items {
            let r = item.input_open_format(name);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r,
            };
        }

        OpenResult::NotAvailable
    }
}
