// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic I/O implementations for `std::fs` types.

use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufReader, Seek, SeekFrom},
    path::{Path, PathBuf},
};
use tectonic_errors::Result;
use tectonic_status_base::{tt_warning, StatusBackend};

use super::{
    try_open_file, InputFeatures, InputHandle, InputOrigin, IoProvider, OpenResult, OutputHandle,
    TectonicIoError,
};

/// FilesystemPrimaryInputIo is an I/O provider that provides the TeX "primary input"
/// file off of the filesystem. This can *pretty much* be achieved with
/// Filesystem I/O, but we need the "primary input" formalism to decently support
/// processing if stdin, and by doing things this way we can handle paths on
/// Unix systems that can't be converted to UTF-8.
///
/// TODO: it might be technically superior to open the path immediately and
/// keep that handle open, rewinding as needed, but for now we're not doing
/// that.
#[derive(Debug)]
pub struct FilesystemPrimaryInputIo {
    path: PathBuf,
}

impl FilesystemPrimaryInputIo {
    /// Create a new I/O provider providing the Tectonic "primary input" from
    /// the specified path.
    pub fn new<P: AsRef<Path>>(path: P) -> FilesystemPrimaryInputIo {
        FilesystemPrimaryInputIo {
            path: path.as_ref().to_owned(),
        }
    }
}

impl IoProvider for FilesystemPrimaryInputIo {
    fn input_open_primary(&mut self, status: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        match self.input_open_primary_with_abspath(status) {
            OpenResult::Ok((ih, _path)) => OpenResult::Ok(ih),
            OpenResult::Err(e) => OpenResult::Err(e),
            OpenResult::NotAvailable => OpenResult::NotAvailable,
        }
    }

    fn input_open_primary_with_abspath(
        &mut self,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        let f = match try_open_file(&self.path) {
            OpenResult::Ok(f) => f,
            OpenResult::NotAvailable => return OpenResult::NotAvailable,
            OpenResult::Err(e) => return OpenResult::Err(e),
        };

        let handle = InputHandle::new("", BufReader::new(f), InputOrigin::Filesystem);

        let path = match make_abspath(&self.path) {
            Ok(m) => m,
            Err(e) => return OpenResult::Err(e),
        };

        OpenResult::Ok((handle, Some(path)))
    }
}

/// FilesystemIo is an I/O provider that reads, and optionally writes, files
/// from a given root directory.
///
/// NOTE: no effort is made to contain I/O within the specified root!! We have
/// an option to disallow absolute paths, but we don't do anything about
/// `../../../....` paths.
pub struct FilesystemIo {
    root: PathBuf,
    writes_allowed: bool,
    absolute_allowed: bool,
    hidden_input_paths: HashSet<PathBuf>,
    reported_paths: HashSet<PathBuf>,
}

impl FilesystemIo {
    /// Create a new filesystem I/O provider rooted at the specified path.
    pub fn new(
        root: &Path,
        writes_allowed: bool,
        absolute_allowed: bool,
        hidden_input_paths: HashSet<PathBuf>,
    ) -> FilesystemIo {
        FilesystemIo {
            root: PathBuf::from(root),
            writes_allowed,
            absolute_allowed,
            hidden_input_paths,
            reported_paths: HashSet::new(),
        }
    }

    /// Get the root filesystem path of this I/O provider.
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn construct_path(&mut self, name: &str) -> Result<PathBuf> {
        let path = Path::new(name);

        if path.is_absolute() && !self.absolute_allowed {
            return Err(TectonicIoError::PathForbidden(path.to_owned()).into());
        }

        let mut combined = PathBuf::from(&self.root);
        combined.push(path);
        Ok(combined)
    }
}

impl IoProvider for FilesystemIo {
    fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
        if !self.writes_allowed {
            return OpenResult::NotAvailable;
        }

        let path = match self.construct_path(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e),
        };

        let f = match File::create(path) {
            Ok(f) => f,
            Err(e) => return OpenResult::Err(e.into()),
        };

        OpenResult::Ok(OutputHandle::new(name, f))
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        // TODO: option to record "stdout" into a file with a particular name?
        OpenResult::NotAvailable
    }

    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        match self.input_open_name_with_abspath(name, status) {
            OpenResult::Ok((h, _path)) => OpenResult::Ok(h),
            OpenResult::Err(e) => OpenResult::Err(e),
            OpenResult::NotAvailable => OpenResult::NotAvailable,
        }
    }

    #[allow(clippy::if_same_then_else)]
    fn input_open_name_with_abspath(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        let path = match self.construct_path(name) {
            Ok(p) => p,
            Err(e) => return OpenResult::Err(e),
        };

        // We allow users to "hide" certain paths to make it so that users can,
        // e.g., run a "first pass" on an input even if an `.aux` file is stored
        // on the filesystem for subsequent processing. Note that this test is
        // purely textual, in the "TeX space" of paths, and won't handle things
        // like directory trees. We could be cleverer here (e.g. glob support),
        // but probably don't want to try to handle stuff like symlink
        // resolution since that will force us to touch the filesystem for every
        // I/O probe.
        if self.hidden_input_paths.contains(&path) {
            return OpenResult::NotAvailable;
        }

        let f = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                return if e.kind() == io::ErrorKind::NotFound {
                    OpenResult::NotAvailable
                } else if let Some(libc::ENOTDIR) = e.raw_os_error() {
                    // xdvipdfmx has a code path that basically tries to open a
                    // font path assuming that it is a directory, which causes an
                    // ENOTDIR to happen (i.e., it tries the equivalent of
                    // open("/etc/passwd/subdir"). This circumstance is harmless.
                    OpenResult::NotAvailable
                } else {
                    OpenResult::Err(e.into())
                };
            }
        };

        // Report the absolute path only if we're able to open it, since the xetex engine tries to
        // read a lot of places.
        //
        // We check with the original requested name since construct_path might make relative paths
        // into absolute paths (e.g. when self.root is absolute).
        let name_path = Path::new(name);
        if name_path.is_absolute() && !self.reported_paths.contains(name_path) {
            tt_warning!(
                status,
                "accessing absolute path `{}`; build may not be reproducible in other environments",
                name_path.display()
            );
            self.reported_paths.insert(name_path.to_owned());
        }

        // Issue #754 - if you run Tectonic on an input that is located in a
        // directory containing a sub-directory named `latex`, you get a
        // surprising error message because the engine tries to read that
        // directory as the format file. I think the correct behavior here is to
        // treat directories as NotAvailable for the purposes of the I/O stack.
        let md = match f.metadata() {
            Ok(m) => m,
            Err(e) => return OpenResult::Err(e.into()),
        };

        if md.is_dir() {
            return OpenResult::NotAvailable;
        }

        // SyncTeX requires absolute paths.
        let path = match make_abspath(path) {
            Ok(m) => m,
            Err(e) => return OpenResult::Err(e),
        };

        // Good to go.
        let handle = InputHandle::new(name, BufReader::new(f), InputOrigin::Filesystem);
        OpenResult::Ok((handle, Some(path)))
    }
}

impl InputFeatures for File {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.metadata()?.len() as usize)
    }

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        let sys_time = self.metadata()?.modified()?;

        // No cleaner way to convert a SystemTime to time_t, as far as I can
        // tell.
        let dur = sys_time.duration_since(std::time::SystemTime::UNIX_EPOCH)?;
        Ok(Some(dur.as_secs() as i64))
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}

impl InputFeatures for BufReader<File> {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.get_mut().metadata()?.len() as usize)
    }

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        let sys_time = self.get_mut().metadata()?.modified()?;
        let dur = sys_time.duration_since(std::time::SystemTime::UNIX_EPOCH)?;
        Ok(Some(dur.as_secs() as i64))
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}

/// For SyncTeX paths we need to make sure that we return an absolute
/// path. `std::fs::canonicalize` is a bit overkill and prefixes all of
/// our paths with `\\?\` on Windows.
fn make_abspath<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let cwd = env::current_dir()?;
    Ok(cwd.join(path.as_ref()))
}
