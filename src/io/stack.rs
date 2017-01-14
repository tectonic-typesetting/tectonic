// src/io/stack.rs -- a stack of other IOProviders
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::OsStr;

use super::{InputHandle, IOProvider, OpenResult, OutputHandle};


// An IOStack is an IOProvider that delegates to an ordered list of
// subordinate IOProviders.

pub struct IOStack<'a> {
    pub items: Vec<&'a mut IOProvider>,
}


impl<'a> IOStack<'a> {
    pub fn new(items: Vec<&mut IOProvider>) -> IOStack {
        IOStack { items: items }
    }
}


impl<'a> IOProvider for IOStack<'a> {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        for item in self.items.iter_mut() {
            let r = item.output_open_name(name);

            match r {
                OpenResult::NotAvailable => continue,
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

    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
        for item in self.items.iter_mut() {
            let r = item.input_open_name(name);

            match r {
                OpenResult::NotAvailable => continue,
                _ => return r
            };
        }

        OpenResult::NotAvailable
    }
}
