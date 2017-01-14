// src/io/genuine_stdout.rs -- I/O to the current processes' actual stdout
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::ffi::OsStr;
use std::io::stdout;

use super::{InputHandle, IOProvider, OpenResult, OutputHandle};


// GenuineStdoutIO provides a mechanism for the "stdout" output to actually go
// to the process's stdout.

pub struct GenuineStdoutIO {}


impl GenuineStdoutIO {
    pub fn new() -> GenuineStdoutIO {
        GenuineStdoutIO {}
    }
}


impl IOProvider for GenuineStdoutIO {
    fn output_open_name(&mut self, _: &OsStr) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::Ok(Box::new(stdout()))
    }

    fn input_open_name(&mut self, _: &OsStr) -> OpenResult<InputHandle> {
        OpenResult::NotAvailable
    }
}
