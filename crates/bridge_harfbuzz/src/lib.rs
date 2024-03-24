// Copyright 2020-2023 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the Harfbuzz *C/C++* API into the Cargo framework, as well as
//! provide bindings to other tectonic crates.

#![allow(non_camel_case_types)]

use std::ptr;

pub mod sys;

pub use sys::hb_tag_t as Tag;
pub use sys::hb_memory_mode_t as MemoryMode;

/// Import something from our bridge crates so that we ensure that we actually
/// link with them, to pull in the symbols defined in the C APIs.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_bridge_graphite2 as clippyrenamehack1;
}

pub struct Blob(*mut sys::hb_blob_t);

impl Blob {
    pub fn new(mut data: Vec<u8>) -> Blob {
        unsafe extern "C" fn data_free(ptr: *mut ()) {
            let _ = Box::from_raw(ptr.cast::<Vec<u8>>());
        }

        let raw = unsafe {
            sys::hb_blob_create(
                data.as_mut_ptr().cast(),
                data.len() as libc::c_uint,
                MemoryMode::Writable,
                Box::into_raw(Box::new(data)).cast(),
                Some(data_free),
            )
        };

        Blob(raw)
    }
}

pub struct Face(*mut sys::hb_face_t);

impl Face {
    pub fn new_tables<T: Fn(Face, Tag) -> Option<Blob> + 'static>(f: T) -> Face {
        unsafe extern "C" fn get_table<T: Fn(Face, Tag) -> Option<Blob> + 'static>(face: *mut sys::hb_face_t, tag: sys::hb_tag_t, user_data: *mut ()) -> *mut sys::hb_blob_t {
            let f = unsafe { &*user_data.cast::<T>() };
            let face = Face(face);
            match f(face, tag) {
                Some(blob) => blob.0,
                None => ptr::null_mut(),
            }
        }

        unsafe extern "C" fn dealloc<T: Fn(Face, Tag) -> Option<Blob> + 'static>(user_data: *mut ()) {
            let _ = unsafe { Box::from_raw(user_data.cast::<T>()) };
        }

        let face = unsafe { sys::hb_face_create_for_tables(
            get_table::<T>,
            Box::into_raw(Box::new(f)).cast(),
            Some(dealloc::<T>),
        ) };

        Face(face)
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        unsafe { sys::hb_face_destroy(self.0) }
    }
}

pub struct Font(*mut sys::hb_font_t);

impl Font {
    pub fn new(face: Face) -> Font {
        Font(unsafe { sys::hb_font_create(face.0) })
    }

    pub fn get_face(&self) -> Face {
        Face(unsafe { sys::hb_font_get_face(self.0) })
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { sys::hb_font_destroy(self.0) }
    }
}

#[test]
fn linkage() {}
