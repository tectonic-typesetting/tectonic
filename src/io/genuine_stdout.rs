// src/io/genuine_stdout.rs -- I/O to the current processes' actual stdout
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

use std::io::stdout;

use super::{IoProvider, OpenResult, OutputHandle};


// GenuineStdoutIo provides a mechanism for the "stdout" output to actually go
// to the process's stdout.

pub struct GenuineStdoutIo {}


impl GenuineStdoutIo {
    pub fn new() -> GenuineStdoutIo {
        GenuineStdoutIo {}
    }
}


impl IoProvider for GenuineStdoutIo {
    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::Ok(Box::new(stdout()))
    }
}
