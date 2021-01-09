// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

//! Helpers to tidy up the computation of digests in various places.

use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
    string::ToString,
};
use tectonic_errors::{Error, Result};
use thiserror::Error as ThisError;

pub use sha2::Digest;
pub use sha2::Sha256 as DigestComputer;

/// Errors that are generic to Tectonic's framework, but not capturable as
/// IoErrors.
#[derive(ThisError, Debug)]
#[error("hexadecimal text had bad length: expected {expected}, observed {observed}")]
pub struct BadLengthError {
    expected: usize,
    observed: usize,
}

// Generic helpers

/// Convert a byte slice to its hexadecimal textual representation. Letters will
/// be in lower case.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .concat()
}

/// Convert a hexadecimal string to bytes.
///
/// The expected length of the string is set by the size of the *dest* slice.
pub fn hex_to_bytes(text: &str, dest: &mut [u8]) -> Result<()> {
    let n = dest.len();
    let text_len = text.len();

    if text_len != 2 * n {
        return Err(BadLengthError {
            expected: 2 * n,
            observed: text_len,
        }
        .into());
    }

    for i in 0..n {
        dest[i] = u8::from_str_radix(&text[i * 2..(i + 1) * 2], 16)?;
    }

    Ok(())
}

// The specific implementation we're using: SHA256.

const N_BYTES: usize = 32;

/// The name of the digest algorithm used in the default implementation.
pub const DIGEST_NAME: &str = "SHA256SUM";

/// The number of bytes in the digest produced by the default implementation.
pub const DIGEST_LEN: usize = 64;

/// Create a new `DigestComputer`
pub fn create() -> DigestComputer {
    Default::default()
}

/// A wrapper for a fixed-size byte array representing a digest computed with
/// the default implementation.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DigestData([u8; N_BYTES]);

impl DigestData {
    /// Create a digest that is all zeros.
    pub fn zeros() -> DigestData {
        DigestData([0u8; N_BYTES])
    }

    /// Create a digest of a zero-size byte stream.
    pub fn of_nothing() -> DigestData {
        let dc = create();
        Self::from(dc)
    }

    /// Given a base path, create a child path from this digest's value. The
    /// child path has a subdirectory from the hex value of the first byte of
    /// the digest, then a name consisting of the rest of the hex data. **The
    /// first-byte subdirectory and all parent directories are created when
    /// you call this function!**
    pub fn create_two_part_path(&self, base: &Path) -> Result<PathBuf> {
        let mut p = base.to_path_buf();
        p.push(format!("{:02x}", self.0[0]));
        fs::create_dir_all(&p)?;
        p.push(bytes_to_hex(&self.0[1..]));
        Ok(p)
    }
}

impl ToString for DigestData {
    fn to_string(&self) -> String {
        bytes_to_hex(&self.0)
    }
}

impl FromStr for DigestData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut result = DigestData::zeros();
        hex_to_bytes(s, &mut result.0)?;
        Ok(result)
    }
}

impl From<DigestComputer> for DigestData {
    fn from(s: DigestComputer) -> DigestData {
        let mut result = DigestData::zeros();
        let res = s.finalize();
        result.0.copy_from_slice(res.as_slice());
        result
    }
}
