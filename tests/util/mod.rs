// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

// An item is considered unused if at least one testing binary
// has no reference to it. This yields a lot of false-positives
// using this testing setup...
#![allow(dead_code)]

use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::env;

use tectonic::io::{FilesystemIo, GenuineStdoutIo, IoStack, MemoryIo, IoProvider, InputHandle, OutputHandle, OpenResult};
use tectonic::io::local_cache::LocalCache;
use tectonic::io::itarbundle::{HttpITarIoFactory, ITarBundle};
use tectonic::io::testing::SingleInputFileIo;
use tectonic::status::{NoopStatusBackend, StatusBackend};

const TOP: &'static str = env!("CARGO_MANIFEST_DIR");
const BUNDLE_URL: &'static str = "https://dl.bintray.com/pkgw/tectonic/tl2016extras/2016.0r4/tlextras-2016.0r4.tar";

// This allows the user to pull in new assets and modify the 'static' bundle.
// Simply run `env STATIC_BUNDLE_BACKEND="network bundle" cargo test`.
fn is_testing_for_real() -> bool {
    env::var("STATIC_BUNDLE_BACKEND") != Ok("network bundle".to_owned())
}


pub fn read_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut f = File::open(&path).unwrap();
    f.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn test_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(TOP);
    path.push("tests");
    path.push(parts.iter().collect::<PathBuf>());
    path
}


pub struct TestBundle {
    pub status_backend: NoopStatusBackend,
    pub io: Vec<Box<IoProvider>>,
    pub mem_io: MemoryIo,
}

impl TestBundle {
    pub fn new() -> Self {
        let genuine_stdout = GenuineStdoutIo::new();
        TestBundle {
            status_backend: NoopStatusBackend::new(),
            io: vec![
                Box::new(genuine_stdout),
            ],
            mem_io: MemoryIo::new(true),
        }
    }

    pub fn with_file(self, p: &Path) -> Self {
        self.with_io(SingleInputFileIo::new(p))
    }

    pub fn with_folder(self, p: &Path) -> Self {
        self.with_io(FilesystemIo::new(p, false, false, HashSet::new()))
    }

    pub fn with_static_bundle(self) -> Self {
        let mut url2digest_path = test_path(&["test_bundle", "urls"]);
        url2digest_path.push(
            BUNDLE_URL.chars().filter(|c| c.is_alphanumeric()).collect::<String>()
        );

        if is_testing_for_real() {
            // Note: This relies on the behaviour of LocalCache as the
            // local cache is only modified on successful requests to the
            // underlying backend.
            self.with_io(LocalCache::<PanickyStaticBundleIoProvider>::new(
                PanickyStaticBundleIoProvider,
                &url2digest_path,
                &test_path(&["test_bundle", "manifests"]),
                &test_path(&["test_bundle", "formats"]),
                &test_path(&["test_bundle", "files"]),
                &mut NoopStatusBackend::new()
            ).expect("Unable to initialize LocalCache (without network backend)"))
        } else {
            let itb = ITarBundle::<HttpITarIoFactory>::new(BUNDLE_URL);
            self.with_io(LocalCache::<ITarBundle<HttpITarIoFactory>>::new(
                itb,
                &url2digest_path,
                &test_path(&["test_bundle", "manifests"]),
                &test_path(&["test_bundle", "formats"]),
                &test_path(&["test_bundle", "files"]),
                &mut NoopStatusBackend::new()
            ).expect("Unable to initialize LocalCache (with network backend)"))
        }
    }

    pub fn with_io<T: IoProvider + 'static>(mut self, io: T) -> Self {
        self.io.push(Box::new(io));
        self
    }

    pub fn as_iostack(&mut self) -> IoStack {
        let mut ios = vec![&mut self.mem_io as &mut IoProvider];
        for io in &mut self.io {
            ios.push(&mut **io);
        }
        IoStack::new(ios)
    }
}

struct PanickyStaticBundleIoProvider;
impl IoProvider for PanickyStaticBundleIoProvider {
    fn output_open_name(&mut self, name: &OsStr) -> OpenResult<OutputHandle> {
        panic!("Not known to static bundle: {:?}", name)
    }

    fn input_open_name(&mut self, name: &OsStr, _: &mut StatusBackend) -> OpenResult<InputHandle> {
        panic!("Not known to static bundle {:?}", name)
    }
}


pub fn assert_file_eq(name: &OsStr, expected: &[u8], observed: &[u8]) {
    if expected == observed {
        return;
    }

    // For nontrivial tests, it's really tough to figure out what
    // changed without being able to do diffs, etc. So, write out the
    // buffers.

    {
        let mut n = name.to_owned();
        n.push(".expected");
        let mut f = File::create(&n).expect(&format!("failed to create {} for test failure diagnosis", n.to_string_lossy()));
        f.write_all(expected).expect(&format!("failed to write {} for test failure diagnosis", n.to_string_lossy()));
    }
    {
        let mut n = name.to_owned();
        n.push(".observed");
        let mut f = File::create(&n).expect(&format!("failed to create {} for test failure diagnosis", n.to_string_lossy()));
        f.write_all(observed).expect(&format!("failed to write {} for test failure diagnosis", n.to_string_lossy()));
    }

    panic!("difference in {}; contents saved to disk", name.to_string_lossy());
}
