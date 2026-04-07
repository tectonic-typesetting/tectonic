// Copyright 2017-2022 the Tectonic Project
// Licensed under the MIT License.

//! Test that we can operate in "initex" mode to generate format files as
//! expected. Unlike TeX we set things up so that formats should be
//! byte-for-byte reproducible given the same inputs. Since formats are big,
//! we check for equality by examining their SHA256 digests.
//!
//! Note that since gzip compression is done transparently in the I/O layer,
//! the desired SHA256 is that of the *uncompressed* format data, not the
//! gzipped file that ends up on disk. When implementing this test I wasted a
//! lot of time on that mistake!
//!
//! Temporarily set the constant DEBUG to true to dump out the generated files
//! to disk, which may be helpful in debugging. There is probably a less gross
//! way to implement that option.

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use tectonic::{
    digest::DigestData,
    io::{IoStack, MemoryIo},
    TexEngine,
};
use tectonic_bridge_core::{CoreBridgeLauncher, DriverHooks};
use tectonic_errors::Result;
use tectonic_io_base::{
    filesystem::{FilesystemIo, FilesystemPrimaryInputIo},
    InputHandle, IoProvider, OpenResult, OutputHandle,
};
use tectonic_status_base::{NoopStatusBackend, StatusBackend};

mod util;
use crate::util::test_path;

const DEBUG: bool = false; // TODO: this is kind of ugly

/// A stunted version of driver::FileSummary for examining the format file
/// SHA256 sum.
#[derive(Clone, Debug, Eq, PartialEq)]
struct FileSummary {
    write_digest: Option<DigestData>,
}

impl FileSummary {
    fn new() -> FileSummary {
        FileSummary { write_digest: None }
    }
}

/// Similarly, a stunted DriverHooks implementation.
struct FormatTestDriver<'a> {
    io: IoStack<'a>,
    events: HashMap<String, FileSummary>,
}

impl FormatTestDriver<'_> {
    fn new(io: IoStack) -> FormatTestDriver {
        FormatTestDriver {
            io,
            events: HashMap::new(),
        }
    }
}

// We need to provide this whole wrapper implementation just to be able to add
// file open events in `output_open_name`.
impl IoProvider for FormatTestDriver<'_> {
    fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
        let r = self.io.output_open_name(name);

        if let OpenResult::Ok(_) = r {
            self.events.insert(name.to_owned(), FileSummary::new());
        }

        r
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        self.io.output_open_stdout()
    }

    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        self.io.input_open_name(name, status)
    }

    fn input_open_primary(&mut self, status: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        self.io.input_open_primary(status)
    }

    fn input_open_format(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        self.io.input_open_format(name, status)
    }

    fn write_format(
        &mut self,
        name: &str,
        data: &[u8],
        status: &mut dyn StatusBackend,
    ) -> Result<()> {
        self.io.write_format(name, data, status)
    }
}

impl DriverHooks for FormatTestDriver<'_> {
    fn io(&mut self) -> &mut dyn IoProvider {
        self
    }

    fn event_output_closed(&mut self, name: String, digest: DigestData) {
        let summ = self
            .events
            .get_mut(&name)
            .expect("closing file that wasn't opened?");
        summ.write_digest = Some(digest);
    }
}

fn test_format_generation(texname: &str, fmtname: &str, sha256: &str) {
    util::set_test_root();

    let mut p = test_path(&["assets"]);

    // Filesystem IoProviders for input files.
    let fs_allow_writes = DEBUG;
    let mut fs_support = FilesystemIo::new(&p, fs_allow_writes, false, HashSet::new());
    p.push(texname);
    let mut fs_primary = FilesystemPrimaryInputIo::new(&p);

    // MemoryIo layer that will accept the outputs.
    let mem_stdout_allowed = !DEBUG;
    let mut mem = MemoryIo::new(mem_stdout_allowed);

    use tectonic::io::GenuineStdoutIo;
    let mut stdout = GenuineStdoutIo::new();

    // Run the engine!
    let hooks = {
        let io = if DEBUG {
            IoStack::new(vec![&mut stdout, &mut fs_primary, &mut fs_support])
        } else {
            IoStack::new(vec![&mut mem, &mut fs_primary, &mut fs_support])
        };
        let mut hooks = FormatTestDriver::new(io);
        let mut status = NoopStatusBackend::default();
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);

        TexEngine::default()
            .initex_mode(true)
            .process(&mut launcher, "unused.fmt", texname)
            .unwrap();
        hooks
    };

    // Did we get what we expected?

    let want_digest = DigestData::from_str(sha256).unwrap();

    for (name, info) in &hooks.events {
        if name == fmtname {
            let observed = info.write_digest.unwrap();

            if observed != want_digest {
                println!("expected {fmtname} to have SHA256 = {want_digest}");
                println!("instead, got {observed}");
                panic!();
            }
        }
    }
}

// Keep these alphabetized.

#[test]
fn plain_format() {
    test_format_generation(
        "plain.tex",
        "plain.fmt",
        "2b52cf6d73940ac2eadf88a437d6dc760ba7ad49fa01336d93fb2ed0b9b305db",
    )
}
