// src/lib.rs -- main module file for the Tectonic library.
// Copyright 2016 the Tectonic Project
// Licensed under the MIT License.

#![recursion_limit = "1024"] // "error_chain can recurse deeply"

#[macro_use]
extern crate error_chain;
extern crate flate2;
extern crate libc;
extern crate md5;
extern crate mktemp;
extern crate zip;

mod c_api;
mod file_format;

pub mod bundle;
pub mod errors;
pub mod kpse_api;
pub mod io_api;
pub mod md5_api;
pub mod io;
pub mod engine;

pub use errors::{Error, ErrorKind, Result};
pub use engine::{Engine, TeXResult};

// All sorts of sub-modules need access to the global Engine state and other
// internals, and the way Rust's visibility rules work, we have to implement
// or declare that stuff here. One of the few ways for modules to see non-pub
// stuff is if it's in their immediate parents or children, and we have a
// bunch of sibling modules, so the internals have to go in the common parent.
// Alternatively, we could make the various modules that use engine internals
// into sub-modules of the engine module. That might end up making more sense
// if we accumulate a lot of code that does *not* depend on the engine
// internals.

// The C code relies on an enormous number of global variables so, despite our
// fancy API, there can only ever actually be one Engine instance. (For now.)
// Here we set up the infrastructure to manage this. Of course, this is
// totally un-thread-safe, etc., because the underlying C code is.

// note: ptr::null_mut() gives me a compile error related to const fns right now.
static mut GLOBAL_ENGINE_PTR: *mut () = 0 as *mut _;

// This wraps a Rust function called by the C code via some ttstub_*() function.
fn with_global_engine<F, T> (f: F) -> T where F: FnOnce(&mut Engine<io::IOStack>) -> T {
    unsafe { f(&mut *(GLOBAL_ENGINE_PTR as *mut Engine<io::IOStack>)) }
}

// This wraps any activities that cause the C code to spin up.
unsafe fn assign_global_engine<F, T> (engine: &mut Engine<io::IOStack>, f: F) -> T where F: FnOnce() -> T {
    GLOBAL_ENGINE_PTR = engine as *mut Engine<io::IOStack> as *mut ();
    let rv = f();
    GLOBAL_ENGINE_PTR = 0 as *mut _;
    rv
}


use std::ffi::OsStr;
use std::io::SeekFrom;
use file_format::FileFormat;

trait EngineInternals {
    // As best I can tell, this API needs to be expressed with pointers so
    // that we can compare the handles to the Engine's internal Box<>
    // references. Almost no unsafe code since we don't dereference the
    // pointers much, though!

    fn output_open(&mut self, name: &OsStr, is_gz: bool) -> *const io::OutputHandle;
    fn output_open_stdout(&mut self) -> *const io::OutputHandle;
    fn output_write(&mut self, handle: *mut io::OutputHandle, buf: &[u8]) -> bool;
    fn output_flush(&mut self, handle: *mut io::OutputHandle) -> bool;
    fn output_close(&mut self, handle: *mut io::OutputHandle) -> bool;

    fn input_open(&mut self, name: &OsStr, format: FileFormat, is_gz: bool) -> *const io::InputHandle;
    fn input_get_size(&mut self, handle: *mut io::InputHandle) -> usize;
    fn input_seek(&mut self, handle: *mut io::InputHandle, pos: SeekFrom) -> u64;
    fn input_read(&mut self, handle: *mut io::InputHandle, buf: &mut [u8]) -> Result<()>;
    fn input_close(&mut self, handle: *mut io::InputHandle) -> bool;
}


// "Testing".

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
