// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

//! Compatibility re-exports of `tectonic_io_base::digest` types.

pub use tectonic_io_base::digest::{
    bytes_to_hex, create, hex_to_bytes, Digest, DigestComputer, DigestData, DIGEST_LEN, DIGEST_NAME,
};
