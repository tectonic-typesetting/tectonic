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

#[macro_use] extern crate lazy_static;
extern crate tectonic;

use std::collections::{HashSet, HashMap};
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;

use tectonic::digest::DigestData;
use tectonic::engines::IoEventBackend;
use tectonic::io::{IoStack, MemoryIo};
use tectonic::io::filesystem::{FilesystemPrimaryInputIo, FilesystemIo};
use tectonic::status::NoopStatusBackend;
use tectonic::TexEngine;

const TOP: &str = env!("CARGO_MANIFEST_DIR");
const DEBUG: bool = false; // TODO: this is kind of ugly


lazy_static! {
    static ref LOCK: Mutex<u8> = Mutex::new(0u8);
}


/// A stunted version of cli_driver:FileSummary for examining the format file
/// SHA256 sum.
#[derive(Clone,Debug,Eq,PartialEq)]
struct FileSummary {
    write_digest: Option<DigestData>,
}

impl FileSummary {
    fn new() -> FileSummary {
        FileSummary {
            write_digest: None,
        }
    }
}


/// Similarly, a stunted verion of CliIoEvents.
struct FormatTestEvents(HashMap<OsString, FileSummary>);

impl FormatTestEvents {
    fn new() -> FormatTestEvents { FormatTestEvents(HashMap::new()) }
}

impl IoEventBackend for FormatTestEvents {
    fn output_opened(&mut self, name: &OsStr) {
        self.0.insert(name.to_os_string(), FileSummary::new());
    }

    fn output_closed(&mut self, name: OsString, digest: DigestData) {
        let summ = self.0.get_mut(&name).expect("closing file that wasn't opened?");
        summ.write_digest = Some(digest);
    }
}


fn test_format_generation(subdir: &str, texname: &str, fmtname: &str, sha256: &str) {
    let _guard = LOCK.lock().unwrap(); // until we're thread-safe ...

    let mut p = PathBuf::from(TOP);
    p.push("tests");
    p.push("formats");
    p.push(subdir);

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
            IoStack::new(vec![
                &mut stdout,
                &mut fs_primary,
                &mut fs_support,
            ])
        } else {
            IoStack::new(vec![
                &mut mem,
                &mut fs_primary,
                &mut fs_support,
            ])
        };

        TexEngine::new()
            .initex_mode(true)
            .process(&mut io, &mut events,
                      &mut NoopStatusBackend::new(), "unused.fmt.gz", texname).unwrap();
    }

    // Did we get what we expected?

    let want_digest = DigestData::from_str(sha256).unwrap();

    for (name, info) in &events.0 {
        if name.to_string_lossy() == fmtname {
            let observed = info.write_digest.unwrap();

            if observed != want_digest {
                println!("expected uncompressed {} to have SHA256 = {}", fmtname, want_digest.to_string());
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
        "plain",
        "plain.tex",
        "plain.fmt.gz",
        "43d229c07283184d59e590e342e1904895727ebff156a7d77b8d75a2f7580f46",
    )
}
