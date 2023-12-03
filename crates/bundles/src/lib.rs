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

use std::{io::Read, path::PathBuf, str::FromStr};
use tectonic_errors::{anyhow::bail, atry, Result};
use tectonic_io_base::{digest, digest::DigestData, IoProvider, OpenResult};
use tectonic_status_base::{NoopStatusBackend, StatusBackend};

pub mod cache;
pub mod dir;
pub mod itar;
pub mod zip;

use cache::BundleCache;
use dir::DirBundle;
use itar::ItarBundle;
use zip::ZipBundle;

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
    /// Get a cryptographic digest summarizing this bundle’s contents.
    ///
    /// The digest summarizes the exact contents of every file in the bundle. It
    /// is computed from the sorted names and SHA256 digests of the component
    /// files [as implemented in the TeXLive bundle builder][x].
    ///
    /// [x]: https://github.com/tectonic-typesetting/tectonic-texlive-bundles/blob/master/scripts/ttb_utils.py#L321
    ///
    /// The default implementation gets the digest from a file named
    /// `SHA256SUM`, which is expected to contain the digest in hex-encoded
    /// format.
    fn get_digest(&mut self, status: &mut dyn StatusBackend) -> Result<DigestData> {
        let digest_text = match self.input_open_name(digest::DIGEST_NAME, status) {
            OpenResult::Ok(h) => {
                let mut text = String::new();
                h.take(64).read_to_string(&mut text)?;
                text
            }

            OpenResult::NotAvailable => {
                // Broken or un-cacheable backend.
                bail!("bundle does not provide needed SHA256SUM file");
            }

            OpenResult::Err(e) => {
                return Err(e);
            }
        };

        Ok(atry!(DigestData::from_str(&digest_text); ["corrupted SHA256 digest data"]))
    }

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
    fn all_files(&mut self, status: &mut dyn StatusBackend) -> Result<Vec<String>>;

    /// Return a string that corresponds to this bundle's "location"
    ///
    /// The meaning of this depends on the bundle:
    /// For web bundles, location should be a link.
    /// For local files, use the bundle's digest since it's easy to get.
    ///
    /// This allows us to identify web bundles without an internet connection
    fn get_location(&mut self) -> String {
        let mut nop = NoopStatusBackend {};
        return self.get_digest(&mut nop).unwrap().to_string();
    }
}

impl<B: Bundle + ?Sized> Bundle for Box<B> {
    fn get_digest(&mut self, status: &mut dyn StatusBackend) -> Result<DigestData> {
        (**self).get_digest(status)
    }

    fn all_files(&mut self, status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        (**self).all_files(status)
    }

    fn get_location(&mut self) -> String {
        (**self).get_location()
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
    status: &mut dyn StatusBackend,
) -> Result<Option<Box<dyn Bundle>>> {
    use url::Url;

    // Parse URL and detect bundle type
    if let Ok(url) = Url::parse(&source) {
        if url.scheme() == "https" || url.scheme() == "http" {
            let bundle = BundleCache::new(
                Box::new(ItarBundle::new(source, status)?),
                only_cached,
                status,
                custom_cache_dir,
            )?;
            return Ok(Some(Box::new(bundle)));
        } else if url.scheme() == "file" {
            let file_path = url.to_file_path().map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "failed to parse local path",
                )
            })?; //TODO: handle
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
pub fn get_fallback_bundle(
    format_version: u32,
    only_cached: bool,
    status: &mut dyn StatusBackend,
) -> Result<Box<dyn Bundle>> {
    let url = get_fallback_bundle_url(format_version);

    Ok(Box::new(BundleCache::new(
        Box::new(ItarBundle::new(url, status)?),
        only_cached,
        status,
        None,
    )?))
}
