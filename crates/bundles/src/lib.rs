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
//! - [`cache::CachingBundle`] for access to remote bundles with local
//!   filesystem caching.
//! - [`dir::DirBundle`] turns a directory full of files into a bundle; it is
//!   useful for testing and lightweight usage.
//! - [`zip::ZipBundle`] for a ZIP-format bundle.

use std::{io::Read, str::FromStr};
use tectonic_errors::{anyhow::bail, atry, Result};
use tectonic_io_base::{digest, digest::DigestData, IoProvider, OpenResult};
use tectonic_status_base::StatusBackend;

pub mod cache;
pub mod dir;
pub mod itar;
pub mod zip;

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
}

impl<B: Bundle + ?Sized> Bundle for Box<B> {
    fn get_digest(&mut self, status: &mut dyn StatusBackend) -> Result<DigestData> {
        (**self).get_digest(status)
    }

    fn all_files(&mut self, status: &mut dyn StatusBackend) -> Result<Vec<String>> {
        (**self).all_files(status)
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
        format!(
            "https://relay.fullyjustified.net/default_bundle_v{}.tar",
            format_version
        )
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
) -> Result<cache::CachingBundle<itar::IndexedTarBackend>> {
    let url = get_fallback_bundle_url(format_version);
    let mut cache = cache::Cache::get_user_default()?;
    cache.open(&url, only_cached, status)
}
