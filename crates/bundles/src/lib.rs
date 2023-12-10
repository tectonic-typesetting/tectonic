// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! Implementations of Tectonic bundle formats.
//!
//! A Tectonic “bundle” is a collection of TeX support files. In code, bundles
//! implement the [`Bundle`] trait defined here, although most of the action in
//! a bundle will be in its implementation of [`tectonic_io_base::IoProvider`].
//!
//! This crate provides the following bundle implementations:
//!
//! - [`cache::BundleCache`] provides filesystem-backed caching for any bundle
//! - [`itar::ItarBundle`] provides filesystem-backed caching for any bundle
//! - [`dir::DirBundle`] turns a directory full of files into a bundle; it is
//!   useful for testing and lightweight usage.
//! - [`zip::ZipBundle`] for a ZIP-format bundle.

use std::{io::Read, path::PathBuf};
use tectonic_errors::Result;
use tectonic_io_base::{digest::DigestData, InputHandle, IoProvider, OpenResult};

pub mod cache;
pub mod dir;
pub mod itar;
mod ttbv1;
pub mod ttbv1_fs;
pub mod ttbv1_net;
pub mod zip;

use cache::BundleCache;
use dir::DirBundle;
use itar::ItarBundle;
use ttbv1_fs::Ttbv1FsBundle;
use ttbv1_net::Ttbv1NetBundle;
use zip::ZipBundle;

/// Uniquely identifies a file in a bundle.
pub trait FileInfo: Clone {
    /// Return a path to this file, relative to the bundle.
    fn path(&self) -> &str;

    /// Return the name of this file
    fn name(&self) -> &str;
}

/// Keeps track of
pub trait FileIndex<'this, T>
where
    Self: Sized + 'this,
    T: FileInfo + 'this,
{
    /// Iterate over all [`FileInfo`]s in this index
    fn iter(&'this self) -> Box<dyn Iterator<Item = &'this T> + 'this>;

    /// Get the number of [`FileInfo`]s in this index
    fn len(&self) -> usize;

    /// Has this index been filled with bundle data?
    /// This is always false until we call [`self.initialize()`],
    /// and is always true afterwards.
    fn is_initialized(&self) -> bool {
        return self.len() == 0;
    }

    /// Fill this index from a file
    fn initialize(&mut self, reader: &mut dyn Read) -> Result<()>;

    /// Search for a file in this index, obeying search order.
    ///
    /// Returns a `Some(FileInfo)` if a file was found, and `None` otherwise.
    fn search(&'this mut self, name: &str) -> Option<T>;
}

/// A trait for bundles of Tectonic support files.
///
/// A “bundle” is an [`IoProvider`] with a few special properties. Bundles are
/// read-only, and their contents can be enumerated In principle a bundle is
/// completely defined by its file contents, which can be summarized by a
/// cryptographic digest, obtainable using the [`Self::get_digest`] method: two
/// bundles with the same digest should contain exactly the same set of files,
/// and if any aspect of a bundle’s file contents change, so should its digest.
/// Finally, it is generally expected that a bundle will contain a large number
/// of TeX support files, and that you can generate one or more TeX format files
/// using only the files contained in a bundle.
pub trait Bundle: IoProvider {
    /// Get a cryptographic digest summarizing this bundle’s contents,
    /// which summarizes the exact contents of every file in the bundle.
    fn get_digest(&mut self) -> Result<DigestData>;

    /// Enumerate the files in this bundle.
    ///
    /// This interface is intended to be used for diagnostics, not by anything
    /// during actual execution of an engine. This should include meta-files
    /// such as the `SHA256SUM` file. The ordering of the returned filenames is
    /// unspecified.
    ///
    /// To ease implementation, the filenames are returned in one big vector of
    /// owned strings. For a large bundle, the memory consumed by this operation
    /// might be fairly substantial (although we are talking megabytes, not
    /// gigabytes).
    fn all_files(&mut self) -> Result<Vec<String>>;
}

impl<B: Bundle + ?Sized> Bundle for Box<B> {
    fn get_digest(&mut self) -> Result<DigestData> {
        (**self).get_digest()
    }

    fn all_files(&mut self) -> Result<Vec<String>> {
        (**self).all_files()
    }
}

/// A bundle that may be cached.
///
/// These methods do not implement any new features.
/// Instead, they give the [`cache::BundleCache`] wrapper
/// more granular access to existing bundle functionality.
pub trait CachableBundle<'this, F, T>
where
    Self: Bundle + 'this,
    F: FileInfo + 'this,
    T: FileIndex<'this, F>,
{
    /// Initialize this bundle's file index from an external reader
    /// This allows us to retrieve the FileIndex from the cache WITHOUT
    /// touching the network.
    fn initialize_index(&mut self, _source: &mut dyn Read) -> Result<()> {
        Ok(())
    }

    /// Get a `Read` instance to this bundle's index,
    /// reading directly from the backend.
    fn get_index_reader(&mut self) -> Result<Box<dyn Read>>;

    /// Return a reference to this bundle's FileIndex.
    fn index(&mut self) -> &mut T;

    /// Open the file that `info` points to.
    fn open_fileinfo(&mut self, info: &F) -> OpenResult<InputHandle>;

    /// Search for a file in this bundle.
    /// This should foward the call to `self.index`
    fn search(&mut self, name: &str) -> Option<F>;

    /// Return a string that corresponds to this bundle's location,
    /// probably a URL.
    ///
    /// We should NOT need to do any IO to get this value.
    fn get_location(&mut self) -> String;
}

impl<
        'this,
        F: FileInfo + 'this,
        T: FileIndex<'this, F>,
        B: CachableBundle<'this, F, T> + ?Sized,
    > CachableBundle<'this, F, T> for Box<B>
{
    fn initialize_index(&mut self, source: &mut dyn Read) -> Result<()> {
        (**self).initialize_index(source)
    }

    fn get_location(&mut self) -> String {
        (**self).get_location()
    }

    fn get_index_reader(&mut self) -> Result<Box<dyn Read>> {
        (**self).get_index_reader()
    }

    fn index(&mut self) -> &mut T {
        (**self).index()
    }

    fn open_fileinfo(&mut self, info: &F) -> OpenResult<InputHandle> {
        (**self).open_fileinfo(info)
    }

    fn search(&mut self, name: &str) -> Option<F> {
        (**self).search(name)
    }
}

/// Try to open a bundle from a string,
/// detecting its type.
///
/// Returns None if auto-detection fails.
pub fn detect_bundle(
    source: String,
    only_cached: bool,
    custom_cache_dir: Option<PathBuf>,
) -> Result<Option<Box<dyn Bundle>>> {
    use url::Url;

    // Parse URL and detect bundle type
    if let Ok(url) = Url::parse(&source) {
        if url.scheme() == "https" || url.scheme() == "http" {
            if source.ends_with("ttb") {
                let bundle = BundleCache::new(
                    Box::new(Ttbv1NetBundle::new(source)?),
                    only_cached,
                    custom_cache_dir,
                )?;
                return Ok(Some(Box::new(bundle)));
            } else {
                let bundle = BundleCache::new(
                    Box::new(ItarBundle::new(source)?),
                    only_cached,
                    custom_cache_dir,
                )?;
                return Ok(Some(Box::new(bundle)));
            }
        } else if url.scheme() == "file" {
            let file_path = url.to_file_path().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "failed to parse local path",
                )
            })?;
            return bundle_from_path(file_path);
        } else {
            return Ok(None);
        }
    } else {
        // If we couldn't parse the URL, this is probably a local path.
        return bundle_from_path(PathBuf::from(source));
    }

    fn bundle_from_path(p: PathBuf) -> Result<Option<Box<dyn Bundle>>> {
        let ext = p.extension().map_or("", |x| x.to_str().unwrap_or(""));

        if p.is_dir() {
            Ok(Some(Box::new(DirBundle::new(p))))
        } else if ext == "zip" {
            Ok(Some(Box::new(ZipBundle::open(p)?)))
        } else if ext == "ttb" {
            Ok(Some(Box::new(Ttbv1FsBundle::open(p)?)))
        } else {
            Ok(None)
        }
    }
}

/// Get the URL of the default bundle.
///
/// This is a mostly-hardcoded URL of a default bundle that will provide some
/// "sensible" set of TeX support files. The higher-level `tectonic` crate
/// provides a configuration mechanism to allow the user to override this
/// setting, so you should use that if you are in a position to do so.
///
/// The URL depends on the format version supported by the engine, since that
/// roughly corresponds to a TeXLive version, and the engine and TeXLive files
/// are fairly closely coupled.
///
/// The URL template used in this function will be embedded in the binaries that
/// you create, which may be used for years into the future, so it needs to be
/// durable and reliable. We used `archive.org` for a while, but it had
/// low-level reliability problems and was blocked in China. We now use a custom
/// webservice.
pub fn get_fallback_bundle_url(format_version: u32) -> String {
    // Format version 32 (TeXLive 2021) was when we introduced versioning to the
    // URL.
    if format_version < 32 {
        "https://relay.fullyjustified.net/default_bundle.tar".to_owned()
    } else {
        format!("https://relay.fullyjustified.net/default_bundle_v{format_version}.tar")
    }
}

/// Open the fallback bundle.
///
/// This is essentially the default Tectonic bundle, but the higher-level
/// `tectonic` crate provides a configuration mechanism to allow the user to
/// override the bundle URL setting, and that should be preferred if you’re in a
/// position to use it.
pub fn get_fallback_bundle(format_version: u32, only_cached: bool) -> Result<Box<dyn Bundle>> {
    let url = get_fallback_bundle_url(format_version);

    Ok(Box::new(BundleCache::new(
        Box::new(ItarBundle::new(url)?),
        only_cached,
        None,
    )?))
}
