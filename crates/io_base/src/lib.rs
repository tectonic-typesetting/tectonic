// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! Tectonic’s pluggable I/O backend.
//!
//! This crates defines the core traits and types used by Tectonic’s I/O
//! subsystem, and provides implementations for common stdlib types. It provides
//! a simplified I/O model compatible with TeX’s usage patterns, as encapsulated
//! in the [`IoProvider`] trait. Files are exposed as [`InputHandle`] or
//! [`OutputHandle`] structs, which add a layer of bookkeeping to allow the
//! higher levels of Tectonic to determine when the engine needs to be re-run.

use sha2::Digest;
use std::{
    borrow::Cow,
    fs::File,
    io::{self, Cursor, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
use tectonic_errors::{
    anyhow::{bail, ensure},
    Error, Result,
};
use tectonic_status_base::StatusBackend;
use thiserror::Error as ThisError;

use crate::digest::DigestData;

pub mod app_dirs;
pub mod digest;
pub mod filesystem;
pub mod flate2;
pub mod stack;
pub mod stdstreams;

/// Errors that are generic to Tectonic's framework, but not capturable as
/// IoErrors.
#[derive(ThisError, Debug)]
pub enum TectonicIoError {
    /// The stream is not seekable.
    #[error("cannot seek within this stream")]
    NotSeekable,

    /// The stream does not have a fixed size.
    #[error("cannot obtain the size of this stream")]
    NotSizeable,

    /// Access to this path is forbidden.
    #[error("access to the path `{}` is forbidden", .0.display())]
    PathForbidden(PathBuf),
}

/// An extension to the basic Read trait supporting additional features
/// needed for Tectonic's I/O system.
pub trait InputFeatures: Read {
    /// Get the size of the stream. Return `TectonicIoError::NotSizeable`
    /// if the operation is not well-defined for this stream.
    fn get_size(&mut self) -> Result<usize>;

    /// Try to seek within the stream. Return `TectonicIoError::NotSeekable`
    /// if the operation is not possible for this stream.
    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64>;

    /// Get the modification time of this file as a Unix time. If that quantity
    /// is not meaningfully defined for this input, return `Ok(None)`. This is
    /// what the default implementation does.
    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        Ok(None)
    }
}

/// What kind of source an input file ultimately came from. We keep track of
/// this in order to be able to emit Makefile-style dependencies for input
/// files. Right now, we only provide enough options to achieve this goal; we
/// could add more.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputOrigin {
    /// This file lives on the filesystem and might change under us. (That is,
    /// it is not a cached bundle file.)
    Filesystem,

    /// This file was never used as an input.
    NotInput,

    /// This file is none of the above.
    Other,
}

/// Input handles are basically Read objects with a few extras. We don't
/// require the standard io::Seek because we need to provide a dummy
/// implementation for GZip streams, which we wouldn't be allowed to do
/// because both the trait and the target struct are outside of our crate.
///
/// An important role for the InputHandle struct is computing a cryptographic
/// digest of the input file. The driver uses this information in order to
/// figure out if the TeX engine needs rerunning. TeX makes our life more
/// difficult, though, since it has somewhat funky file access patterns. LaTeX
/// file opens work by opening a file and immediately closing it, which tests
/// whether the file exists, and then by opening it again for real. Under the
/// hood, XeTeX reads a couple of bytes from each file upon open to sniff its
/// encoding. So we can't just stream data from `read()` calls into the SHA2
/// computer, since we end up seeking and reading redundant data.
///
/// The current system maintains some internal state that, so far, helps us Do
/// The Right Thing given all this. If there's a seek on the file, we give up
/// on our digest computation. But if there's a seek back to the file
/// beginning, we are open to the possibility of restarting the computation.
/// But if nothing is ever read from the file, we once again give up on the
/// computation. The `ExecutionState` code then has further pieces that track
/// access to nonexistent files, which we treat as being equivalent to an
/// existing empty file for these purposes.
pub struct InputHandle {
    name: String,
    inner: Box<dyn InputFeatures>,
    /// Indicates that the file cannot be written to (provided by a read-only IoProvider) and
    /// therefore it is useless to compute the digest.
    read_only: bool,
    digest: digest::DigestComputer,
    origin: InputOrigin,
    ever_read: bool,
    did_unhandled_seek: bool,
    ungetc_char: Option<u8>,
}

impl InputHandle {
    /// Create a new InputHandle wrapping an underlying type that implements
    /// `InputFeatures`.
    pub fn new<T: 'static + InputFeatures>(
        name: impl Into<String>,
        inner: T,
        origin: InputOrigin,
    ) -> InputHandle {
        InputHandle {
            name: name.into(),
            inner: Box::new(inner),
            read_only: false,
            digest: Default::default(),
            origin,
            ever_read: false,
            did_unhandled_seek: false,
            ungetc_char: None,
        }
    }

    /// Create a new InputHandle in read-only mode.
    pub fn new_read_only<T: 'static + InputFeatures>(
        name: impl Into<String>,
        inner: T,
        origin: InputOrigin,
    ) -> InputHandle {
        InputHandle {
            name: name.into(),
            inner: Box::new(inner),
            read_only: true,
            digest: Default::default(),
            origin,
            ever_read: false,
            did_unhandled_seek: false,
            ungetc_char: None,
        }
    }

    /// Get the name associated with this handle.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the "origin" associated with this handle.
    pub fn origin(&self) -> InputOrigin {
        self.origin
    }

    /// Consumes the object and returns the underlying readable handle that
    /// it references.
    pub fn into_inner(self) -> Box<dyn InputFeatures> {
        self.inner
    }

    /// Read any remaining data in the file and incorporate them into the
    /// digest. This helps the rerun detection logic work correctly in
    /// the somewhat-unusual case that a file is read then written, but
    /// only part of the file is read, not the entire thing. This seems
    /// to happen with biblatex XML state files.
    pub fn scan_remainder(&mut self) -> Result<()> {
        const BUFSIZE: usize = 1024;
        let mut buf: [u8; BUFSIZE] = [0; BUFSIZE];

        loop {
            let n = match self.inner.read(&mut buf[..]) {
                Ok(n) => n,

                // There are times when the engine tries to open and read
                // directories. When closing out such a handle, we'll get this
                // error, but we should ignore it.
                Err(ref ioe) if ioe.raw_os_error() == Some(libc::EISDIR) => return Ok(()),

                Err(e) => return Err(e.into()),
            };

            if n == 0 {
                break;
            }

            self.digest.update(&buf[..n]);
        }

        Ok(())
    }

    /// Consumes the object and returns the SHA256 sum of the content that was
    /// read. No digest is returned if there was ever a seek on the input
    /// stream, since in that case the results will not be reliable. We also
    /// return None if the stream was never read, which is another common
    /// TeX access pattern: files are opened, immediately closed, and then
    /// opened again. Finally, no digest is returned if the file is marked read-only.
    pub fn into_name_digest(self) -> (String, Option<DigestData>) {
        if self.did_unhandled_seek || !self.ever_read || self.read_only {
            (self.name, None)
        } else {
            (self.name, Some(DigestData::from(self.digest)))
        }
    }

    /// Various piece of TeX want to use the libc `ungetc()` function a lot.
    /// It's kind of gross, but happens often enough that we provide special
    /// support for it. Here's `getc()` emulation that can return a previously
    /// `ungetc()`-ed character.
    pub fn getc(&mut self) -> Result<u8> {
        if let Some(c) = self.ungetc_char {
            self.ungetc_char = None;
            return Ok(c);
        }

        let mut byte = [0u8; 1];

        if self.read(&mut byte[..1])? == 0 {
            // EOF
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF in getc").into());
        }

        Ok(byte[0])
    }

    /// Here's the `ungetc()` emulation.
    pub fn ungetc(&mut self, byte: u8) -> Result<()> {
        ensure!(
            self.ungetc_char.is_none(),
            "internal problem: cannot ungetc() more than once in a row"
        );
        self.ungetc_char = Some(byte);
        Ok(())
    }
}

impl Read for InputHandle {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !buf.is_empty() {
            if let Some(c) = self.ungetc_char {
                // This does sometimes happen, so we need to deal with it. It's not that bad really.
                buf[0] = c;
                self.ungetc_char = None;
                return Ok(self.read(&mut buf[1..])? + 1);
            }
        }

        self.ever_read = true;
        let n = self.inner.read(buf)?;
        if !self.read_only {
            self.digest.update(&buf[..n]);
        }
        Ok(n)
    }
}

impl Seek for InputHandle {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.try_seek(pos)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

impl InputFeatures for InputHandle {
    fn get_size(&mut self) -> Result<usize> {
        self.inner.get_size()
    }

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        self.inner.get_unix_mtime()
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(0) => {
                // As described above, there is a common pattern in TeX file
                // accesses: read a few bytes to sniff, then go back to the
                // beginning. We should tidy up the I/O to just buffer instead
                // of seeking, but in the meantime, we can handle this.
                self.digest = Default::default();
                self.ever_read = false;
                self.ungetc_char = None;
            }
            SeekFrom::Current(0) => {
                // Noop. This must *not* clear the ungetc buffer for our
                // current PDF startxref/xref parsing code to work.
            }
            _ => {
                self.did_unhandled_seek = true;
                self.ungetc_char = None;
            }
        }

        let mut offset = self.inner.try_seek(pos)?;

        // If there was an ungetc, the effective position in the stream is one
        // byte before that of the underlying handle. Some of the code does
        // noop seeks to get the current offset for various file parsing
        // needs, so it's important that we return the right value. It should
        // never happen that the underlying stream thinks that the offset is
        // zero after we've ungetc'ed -- famous last words?

        if self.ungetc_char.is_some() {
            offset -= 1;
        }

        Ok(offset)
    }
}

/// A handle for Tectonic output streams.
pub struct OutputHandle {
    name: String,
    inner: Box<dyn Write>,
    digest: digest::DigestComputer,
}

impl OutputHandle {
    /// Create a new output handle.
    pub fn new<T: 'static + Write>(name: impl Into<String>, inner: T) -> OutputHandle {
        OutputHandle {
            name: name.into(),
            inner: Box::new(inner),
            digest: digest::create(),
        }
    }

    /// Get the name associated with this handle.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Consumes the object and returns the underlying writable handle that
    /// it references.
    pub fn into_inner(self) -> Box<dyn Write> {
        self.inner
    }

    /// Consumes the object and returns the SHA256 sum of the content that was
    /// written.
    pub fn into_name_digest(self) -> (String, DigestData) {
        (self.name, DigestData::from(self.digest))
    }
}

impl Write for OutputHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.digest.update(&buf[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// A convenience type for file-open operations when "not found" needs to be
/// handled specially.
#[derive(Debug)]
pub enum OpenResult<T> {
    /// The open operation succeeded.
    Ok(T),

    /// The file was not available.
    NotAvailable,

    /// Something else went wrong.
    Err(Error),
}

impl<T> OpenResult<T> {
    /// Obtain the underlying file object, or panic.
    pub fn unwrap(self) -> T {
        match self {
            OpenResult::Ok(t) => t,
            _ => panic!("expected an open file"),
        }
    }

    /// Returns true if this result is of the NotAvailable variant.
    pub fn is_not_available(&self) -> bool {
        matches!(*self, OpenResult::NotAvailable)
    }

    /// Convert this object into a plain Result, erroring if the item was not available.
    pub fn must_exist(self) -> Result<T> {
        match self {
            OpenResult::Ok(t) => Ok(t),
            OpenResult::Err(e) => Err(e),
            OpenResult::NotAvailable => {
                Err(io::Error::new(io::ErrorKind::NotFound, "not found").into())
            }
        }
    }
}

/// A hack to allow casting of Bundles to IoProviders.
///
/// The code that sets up the I/O stack is handed a reference to a Bundle
/// trait object. For the actual I/O, it needs to convert this to an
/// IoProvider trait object. [According to
/// StackExchange](https://stackoverflow.com/a/28664881/3760486), the
/// following pattern is the least-bad way to achieve the necessary upcasting.
pub trait AsIoProviderMut {
    /// Represent this value as an IoProvider trait object.
    fn as_ioprovider_mut(&mut self) -> &mut dyn IoProvider;
}

impl<T: IoProvider> AsIoProviderMut for T {
    fn as_ioprovider_mut(&mut self) -> &mut dyn IoProvider {
        self
    }
}

/// A trait for types that can read or write files needed by the TeX engine.
///
/// An IO provider is a source of handles. One wrinkle is that it's good to be
/// able to distinguish between unavailability of a given name and error
/// accessing it.
pub trait IoProvider: AsIoProviderMut {
    /// Open the named file for output.
    fn output_open_name(&mut self, _name: &str) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    /// Open the standard output stream.
    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::NotAvailable
    }

    /// Open the named file for input.
    fn input_open_name(
        &mut self,
        _name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        OpenResult::NotAvailable
    }

    /// Open the named file for input and return filesystem path information.
    ///
    /// This method extends [`Self::input_open_name`] to help support SyncTeX output.
    /// While SyncTeX output files should contain absolute source file paths,
    /// Tectonic’s pluggable I/O system makes it so that the mapping between
    /// input names and filesystem paths is not well-defined. This optional
    /// interface enables backends to provide filesystem information at the time
    /// of opening.
    ///
    /// The default implementation returns None for the path information, to
    /// preserve backwards compatibility. If you are implementing a new backend
    /// that might provide path information, or you are implementing an I/O
    /// provider that delegates to other I/O providers, you should implement
    /// this function fully, and then provide a simple implementation of
    /// [`Self::input_open_name`] that drops the pathing information.
    fn input_open_name_with_abspath(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        match self.input_open_name(name, status) {
            OpenResult::Ok(h) => OpenResult::Ok((h, None)),
            OpenResult::Err(e) => OpenResult::Err(e),
            OpenResult::NotAvailable => OpenResult::NotAvailable,
        }
    }

    /// Open the "primary" input file, which in the context of TeX is the main
    /// input that it's given. When the build is being done using the
    /// filesystem and the input is a file on the filesystem, this function
    /// isn't necesssarily that important, but those conditions don't always
    /// hold.
    fn input_open_primary(&mut self, _status: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        OpenResult::NotAvailable
    }

    /// Open the primary input and return filesystem path information.
    ///
    /// This method is as to [`Self::input_open_primary`] as
    /// [`Self::input_open_name_with_abspath`] is to [`Self::input_open_name`].
    fn input_open_primary_with_abspath(
        &mut self,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        match self.input_open_primary(status) {
            OpenResult::Ok(h) => OpenResult::Ok((h, None)),
            OpenResult::Err(e) => OpenResult::Err(e),
            OpenResult::NotAvailable => OpenResult::NotAvailable,
        }
    }

    /// Open a format file with the specified name. Format files have a
    /// specialized entry point because IOProviders may wish to handle them
    /// specially: namely, to munge the filename to one that includes the
    /// current version of the Tectonic engine, since the format contents
    /// depend sensitively on the engine internals.
    fn input_open_format(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        self.input_open_name(name, status)
    }

    /// Save an a format dump in some way that this provider may be able to
    /// recover in the future. This awkward interface is needed to write
    /// formats with their special munged file names.
    fn write_format(
        &mut self,
        _name: &str,
        _data: &[u8],
        _status: &mut dyn StatusBackend,
    ) -> Result<()> {
        bail!("this I/O layer cannot save format files");
    }
}

impl<P: IoProvider + ?Sized> IoProvider for Box<P> {
    fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
        (**self).output_open_name(name)
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        (**self).output_open_stdout()
    }

    fn input_open_name(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        (**self).input_open_name(name, status)
    }

    fn input_open_name_with_abspath(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        (**self).input_open_name_with_abspath(name, status)
    }

    fn input_open_primary(&mut self, status: &mut dyn StatusBackend) -> OpenResult<InputHandle> {
        (**self).input_open_primary(status)
    }

    fn input_open_primary_with_abspath(
        &mut self,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        (**self).input_open_primary_with_abspath(status)
    }

    fn input_open_format(
        &mut self,
        name: &str,
        status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        (**self).input_open_format(name, status)
    }

    fn write_format(
        &mut self,
        name: &str,
        data: &[u8],
        status: &mut dyn StatusBackend,
    ) -> Result<()> {
        (**self).write_format(name, data, status)
    }
}

// Some generically helpful InputFeatures impls

impl InputFeatures for Cursor<Vec<u8>> {
    fn get_size(&mut self) -> Result<usize> {
        Ok(self.get_ref().len())
    }

    fn get_unix_mtime(&mut self) -> Result<Option<i64>> {
        Ok(None)
    }

    fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
        Ok(self.seek(pos)?)
    }
}

// Helpful.

/// Try to open a file on the fileystem, returning an `OpenResult` type
/// allowing easy handling if the file was not found.
pub fn try_open_file<P: AsRef<Path>>(path: P) -> OpenResult<File> {
    use std::io::ErrorKind::NotFound;

    match File::open(path) {
        Ok(f) => OpenResult::Ok(f),
        Err(e) => {
            if e.kind() == NotFound {
                OpenResult::NotAvailable
            } else {
                OpenResult::Err(e.into())
            }
        }
    }
}

// TeX path normalization:

/// Normalize a path from a TeX build.
///
/// We attempt to do this in a system independent™ way by stripping any `.`,
/// `..`, or extra separators '/' so that it is of the form.
///
/// ```text
/// path/to/my/file.txt
/// ../../path/to/parent/dir/file.txt
/// /absolute/path/to/file.txt
/// ```
///
/// Does not strip whitespace.
///
/// Returns `None` if the path refers to a parent of the root.
fn try_normalize_tex_path(path: &str) -> Option<String> {
    // TODO: we should normalize directory separators to "/".
    // And do we need to handle Windows drive prefixes, etc?
    use std::iter::repeat;

    if path.is_empty() {
        return Some("".into());
    }

    let mut r = Vec::new();
    let mut parent_level = 0;
    let mut has_root = false;

    for (i, c) in path.split('/').enumerate() {
        match c {
            "" if i == 0 => {
                has_root = true;
                r.push("");
            }
            "" | "." => {}
            ".." => {
                match r.pop() {
                    // about to pop the root
                    Some("") => return None,
                    None => parent_level += 1,
                    _ => {}
                }
            }
            _ => r.push(c),
        }
    }

    let r = repeat("..")
        .take(parent_level)
        .chain(r)
        // No `join` on `Iterator`.
        .collect::<Vec<_>>()
        .join("/");

    if r.is_empty() {
        if has_root {
            Some("/".into())
        } else {
            Some(".".into())
        }
    } else {
        Some(r)
    }
}

/// Normalize a TeX path if possible, otherwise return the original path.
///
/// A _TeX path_ is a path that obeys simplified semantics: Unix-like syntax (`/`
/// for separators, etc.), must be Unicode-able, no symlinks allowed such that
/// `..` can be stripped lexically.
pub fn normalize_tex_path(path: &str) -> Cow<str> {
    if let Some(t) = try_normalize_tex_path(path).map(String::from) {
        Cow::Owned(t)
    } else {
        Cow::Borrowed(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_normalize_tex_path() {
        // edge cases
        assert_eq!(try_normalize_tex_path(""), Some("".into()));
        assert_eq!(try_normalize_tex_path("/"), Some("/".into()));
        assert_eq!(try_normalize_tex_path("//"), Some("/".into()));
        assert_eq!(try_normalize_tex_path("."), Some(".".into()));
        assert_eq!(try_normalize_tex_path("./"), Some(".".into()));
        assert_eq!(try_normalize_tex_path(".."), Some("..".into()));
        assert_eq!(try_normalize_tex_path("././/./"), Some(".".into()));
        assert_eq!(try_normalize_tex_path("/././/."), Some("/".into()));

        assert_eq!(
            try_normalize_tex_path("my/path/file.txt"),
            Some("my/path/file.txt".into())
        );
        // preserve spaces
        assert_eq!(
            try_normalize_tex_path("  my/pa  th/file .txt "),
            Some("  my/pa  th/file .txt ".into())
        );
        assert_eq!(
            try_normalize_tex_path("/my/path/file.txt"),
            Some("/my/path/file.txt".into())
        );
        assert_eq!(
            try_normalize_tex_path("./my///path/././file.txt"),
            Some("my/path/file.txt".into())
        );
        assert_eq!(
            try_normalize_tex_path("./../my/../../../file.txt"),
            Some("../../../file.txt".into())
        );
        assert_eq!(
            try_normalize_tex_path("././my//../path/../here/file.txt"),
            Some("here/file.txt".into())
        );
        assert_eq!(
            try_normalize_tex_path("./my/.././/path/../../here//file.txt"),
            Some("../here/file.txt".into())
        );

        assert_eq!(try_normalize_tex_path("/my/../../file.txt"), None);
        assert_eq!(
            try_normalize_tex_path("/my/./.././path//../../file.txt"),
            None
        );
    }
}
