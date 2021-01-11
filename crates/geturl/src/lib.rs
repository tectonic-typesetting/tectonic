// Copyright 2020 the Tectonic Project.
// Licensed under the MIT License.

#![deny(missing_docs)]

//! A simple, pluggable interface for HTTP GETs and range requests.
//!
//! The default interface is intentionally exposed as a concrete type, so that
//! crates relying on this one need not use a lot of dyns and impl Traits. It is
//! intended that the choice of HTTP backend is a build-time one, not a runtime
//! one.

use std::io::Read;
use tectonic_errors::Result;
use tectonic_status_base::StatusBackend;

/// A trait for reading byte ranges from an HTTP resource.
pub trait RangeReader {
    /// The readable type returned by the range request.
    type Response: Read;

    /// Read the specified range of bytes from this HTTP resource.
    fn read_range(&mut self, offset: u64, length: usize) -> Result<Self::Response>;
}

/// A trait for simple HTTP operations needed by the Tectonic backends.
pub trait GetUrlBackend: Default {
    /// The readable type returned by URL get requests.
    type Response: Read;

    /// The range-reader type for URLs that will undergo byte-range reads.
    type RangeReader: RangeReader;

    /// Starting with an input URL, follow redirections to get a final URL.
    ///
    /// But we attempt to detect redirects into CDNs/S3/etc and *stop* following
    /// before we get that deep.
    fn resolve_url(&mut self, url: &str, status: &mut dyn StatusBackend) -> Result<String>;

    /// Perform an HTTP GET on a URL, returning a readable result.
    fn get_url(&mut self, url: &str, status: &mut dyn StatusBackend) -> Result<Self::Response>;

    /// Open a range reader that can perform byte-range reads on the specified URL.
    fn open_range_reader(&self, url: &str) -> Self::RangeReader;
}

mod reqwest;

/// The type of the default URL-get backend.
pub use crate::reqwest::ReqwestBackend as DefaultBackend;

/// The URL type used by the default URL-get backend.
pub use ::reqwest::Url;

/// The range-reader type exposed by the URL-get backend (for convenience).
pub type DefaultRangeReader = <DefaultBackend as GetUrlBackend>::RangeReader;
