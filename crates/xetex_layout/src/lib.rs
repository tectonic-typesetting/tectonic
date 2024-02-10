// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! This crate contains no Rust code. It exists to export a *C* API to C++ font
//! loading and layout code in the Cargo build framework used by [Tectonic].
//! Ideally, it will migrate to become a cbindgen C API to a Rust
//! implementation.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_bridge_core as clipyrenamehack1;

    #[allow(unused_imports)]
    use tectonic_bridge_freetype2 as clipyrenamehack2;

    #[allow(unused_imports)]
    use tectonic_bridge_graphite2 as clipyrenamehack3;

    #[allow(unused_imports)]
    use tectonic_bridge_harfbuzz as clipyrenamehack4;

    #[allow(unused_imports)]
    use tectonic_bridge_icu as clipyrenamehack5;
}

macro_rules! cstr {
    ($lit:literal) => {
        // SAFETY: C string passed to from_ptr guaranteed to end with a null due to concat!
        unsafe {
            ::std::ffi::CStr::from_ptr(concat!($lit, "\0") as *const str as *const ::libc::c_char)
        }
    };
}

macro_rules! c {
    ($lit:literal) => {
        concat!($lit, "\0") as *const str as *const libc::c_char
    };
}

mod c_api {
    use crate::c_api::engine::XeTeXLayoutEngineBase;
    use crate::c_api::font::XeTeXFontBase;
    use std::collections::BTreeMap;
    use std::ffi::CStr;
    use std::sync::Mutex;
    use tectonic_bridge_core::FileFormat;
    use tectonic_io_base::InputHandle;

    mod engine;
    #[cfg(not(target_os = "macos"))]
    /// cbindgen:ignore
    mod fc;
    mod font;
    #[cfg(target_os = "macos")]
    mod mac_core;
    mod manager;

    pub(crate) struct SyncPtr<T>(*mut T);
    unsafe impl<T> Send for SyncPtr<T> {}
    unsafe impl<T> Sync for SyncPtr<T> {}

    #[derive(Copy, Clone, PartialEq, Debug)]
    #[repr(C)]
    pub struct FloatPoint {
        x: f32,
        y: f32,
    }

    /// cbindgen:rename-all=camelCase
    #[derive(Copy, Clone, Default, PartialEq, Debug)]
    #[repr(C)]
    pub struct GlyphBBox {
        x_min: f32,
        y_min: f32,
        x_max: f32,
        y_max: f32,
    }

    #[cfg(not(target_os = "macos"))]
    pub type Fixed = i32;
    /// cbindgen:ignore
    #[cfg(target_os = "macos")]
    pub type Fixed = u32;
    pub type OTTag = u32;
    pub type GlyphID = u16;
    /// cbindgen:ignore
    pub type XeTeXFont = *mut XeTeXFontBase;
    /// cbindgen:ignore
    pub type XeTeXLayoutEngine = *mut XeTeXLayoutEngineBase;
    #[cfg(not(target_os = "macos"))]
    type RawPlatformFontRef = *mut fc::sys::FcPattern;
    #[cfg(target_os = "macos")]
    type RawPlatformFontRef = mac_core::CTFontDescriptorRef;
    #[cfg(not(target_os = "macos"))]
    type PlatformFontRef = fc::Pattern;
    #[cfg(target_os = "macos")]
    type PlatformFontRef = mac_core::CTFontDescriptorRef;

    #[no_mangle]
    pub extern "C" fn RsFix2D(f: Fixed) -> f64 {
        f as f64 / 65536.0
    }

    #[no_mangle]
    pub extern "C" fn RsD2Fix(d: f64) -> Fixed {
        (d * 65536.0 + 0.5) as Fixed
    }

    /// key is combined value representing (font_id << 16) + glyph
    /// value is glyph bounding box in TeX points
    static GLYPH_BOXES: Mutex<BTreeMap<u32, GlyphBBox>> = Mutex::new(BTreeMap::new());

    #[no_mangle]
    pub unsafe extern "C" fn getCachedGlyphBBox(
        font_id: u16,
        glyph_id: u16,
        bbox: *mut GlyphBBox,
    ) -> i32 {
        let key = ((font_id as u32) << 16) | (glyph_id as u32);

        match GLYPH_BOXES.lock().unwrap().get(&key) {
            Some(val) => {
                *bbox = *val;
                1
            }
            None => 0,
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn cacheGlyphBBox(font_id: u16, glyph_id: u16, bbox: *const GlyphBBox) {
        let key = ((font_id as u32) << 16) | (glyph_id as u32);
        GLYPH_BOXES.lock().unwrap().insert(key, *bbox);
    }

    /* The following code used to be in a file called "hz.cpp" and there's no
     * particular reason for it to be here, but it was a tiny file with a weird
     * name so I wanted to get rid of it. The functions are invoked from the C
     * code. */

    pub const LEFT_SIDE: i32 = 0;
    pub const RIGHT_SIDE: i32 = 1;

    static LEFT_PROT: Mutex<BTreeMap<(i32, u32), i32>> = Mutex::new(BTreeMap::new());
    static RIGHT_PROT: Mutex<BTreeMap<(i32, u32), i32>> = Mutex::new(BTreeMap::new());

    #[no_mangle]
    pub extern "C" fn set_cp_code(font_num: i32, code: u32, side: i32, value: i32) {
        match side {
            LEFT_SIDE => LEFT_PROT.lock().unwrap().insert((font_num, code), value),
            RIGHT_SIDE => RIGHT_PROT.lock().unwrap().insert((font_num, code), value),
            _ => unreachable!(),
        };
    }

    #[no_mangle]
    pub extern "C" fn get_cp_code(font_num: i32, code: u32, side: i32) -> i32 {
        match side {
            LEFT_SIDE => LEFT_PROT.lock().unwrap().get(&(font_num, code)).copied(),
            RIGHT_SIDE => RIGHT_PROT.lock().unwrap().get(&(font_num, code)).copied(),
            _ => unreachable!(),
        }
        .unwrap_or(0)
    }

    /// Return NAME with any leading path stripped off. This returns a
    /// pointer into NAME.  For example, `basename("/foo/bar.baz")`
    /// returns `"bar.baz"`.
    #[no_mangle]
    pub unsafe extern "C" fn xbasename(name: *const libc::c_char) -> *const libc::c_char {
        let str = CStr::from_ptr(name);
        let pos = str.to_bytes().iter().rposition(|p| *p == b'/').unwrap_or(0);
        name.add(pos)
    }

    /// cbindgen:ignore
    extern "C" {
        fn ttstub_input_open(
            path: *const libc::c_char,
            format: FileFormat,
            is_gz: libc::c_int,
        ) -> *mut InputHandle;
        fn ttstub_input_get_size(handle: *mut InputHandle) -> usize;
        fn ttstub_input_read(
            handle: *mut InputHandle,
            data: *mut libc::c_char,
            len: usize,
        ) -> isize;
        fn ttstub_input_close(handle: *mut InputHandle) -> libc::c_int;
        fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
        fn xmalloc(s: usize) -> *mut libc::c_char;
        fn xcalloc(elems: usize, s: usize) -> *mut libc::c_char;
        fn getReqEngine() -> libc::c_char;
    }
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
