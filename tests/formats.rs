// Copyright 2017-2018 the Tectonic Project
// Licensed under the MIT License.

/// Test that we can operate in "initex" mode to generate format files as
/// expected. Unlike TeX we set things up so that formats should be
/// byte-for-byte reproducible given the same inputs. Since formats are big,
/// we check for equality by examining their SHA256 digests.
///
/// Note that since gzip compression is done transparently in the I/O layer,
/// the desired SHA256 is that of the *uncompressed* format data, not the
/// gzipped file that ends up on disk. When implementing this test I wasted a
/// lot of time on that mistake!
///
/// Temporarily set the constant DEBUG to true to dump out the generated files
/// to disk, which may be helpful in debugging. There is probably a less gross
/// way to implement that option.
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::str::FromStr;

use tectonic::digest::DigestData;
use tectonic::engines::IoEventBackend;
use tectonic::io::{IoStack, MemoryIo};
use tectonic::TexEngine;
use tectonic_io_base::filesystem::{FilesystemIo, FilesystemPrimaryInputIo};
use tectonic_status_base::NoopStatusBackend;

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

/// Similarly, a stunted verion of CliIoEvents.
struct FormatTestEvents(HashMap<String, FileSummary>);

impl FormatTestEvents {
    fn new() -> FormatTestEvents {
        FormatTestEvents(HashMap::new())
    }
}

impl IoEventBackend for FormatTestEvents {
    fn output_opened(&mut self, name: &str) {
        self.0.insert(name.to_owned(), FileSummary::new());
    }

    fn output_closed(&mut self, name: String, digest: DigestData) {
        let summ = self
            .0
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

    // IoEvents manager that will give us output SHA256 sums.
    let mut events = FormatTestEvents::new();

    use tectonic::io::GenuineStdoutIo;
    let mut stdout = GenuineStdoutIo::new();

    // Run the engine!
    {
        let mut io = if DEBUG {
            IoStack::new(vec![&mut stdout, &mut fs_primary, &mut fs_support])
        } else {
            IoStack::new(vec![&mut mem, &mut fs_primary, &mut fs_support])
        };

        TexEngine::new()
            .initex_mode(true)
            .process(
                &mut io,
                &mut events,
                &mut NoopStatusBackend::default(),
                "unused.fmt",
                texname,
                &Default::default(),
            )
            .unwrap();
    }

    // Did we get what we expected?

    let want_digest = DigestData::from_str(sha256).unwrap();

    for (name, info) in &events.0 {
        if name == fmtname {
            let observed = info.write_digest.unwrap();

            if observed != want_digest {
                println!(
                    "expected {} to have SHA256 = {}",
                    fmtname,
                    want_digest.to_string()
                );
                println!("instead, got {}", observed.to_string());
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
        "8e33c4c9af66ddb064a36749db1e0ba681bbebd1a896d2886745a0efa9a745a1",
    )
}
