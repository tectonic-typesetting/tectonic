// src/lib.rs -- main module file for the Tectonic library.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

#[macro_use]
extern crate libc;
extern crate md5;
extern crate mktemp;
extern crate zip;

mod bundle;
mod c_api;
mod file_format;

pub mod kpse_api;
pub mod io_api;
pub mod md5_api;
pub mod engine;

pub use engine::Engine;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
