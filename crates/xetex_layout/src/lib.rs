// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! This crate contains no Rust code. It exists to export a *C* API to C++ font
//! loading and layout code in the Cargo build framework used by [Tectonic].
//! Ideally, it will migrate to become a cbindgen C API to a Rust
//! implementation.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

macro_rules! cstr {
    ($lit:literal) => {
        // SAFETY: C string passed to from_ptr guaranteed to end with a null due to concat!
        unsafe {
            ::std::ffi::CStr::from_ptr(
                ::std::ptr::from_ref(concat!($lit, "\0")).cast::<::libc::c_char>(),
            )
        }
    };
}

mod engine;
mod font;
mod manager;
mod utils;

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

pub(crate) mod c_api {
    use crate::engine::LayoutEngine;
    use crate::font::Font;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    #[cfg(not(target_os = "macos"))]
    use tectonic_bridge_fontconfig as fc;

    mod engine;
    mod font;
    mod manager;

    #[derive(Copy, Clone, PartialEq, Debug)]
    #[repr(C)]
    pub struct FloatPoint {
        pub x: f32,
        pub y: f32,
    }

    /// cbindgen:rename-all=camelCase
    #[derive(Copy, Clone, Default, PartialEq, Debug)]
    #[repr(C)]
    pub struct GlyphBBox {
        pub x_min: f32,
        pub y_min: f32,
        pub x_max: f32,
        pub y_max: f32,
    }

    #[cfg(not(target_os = "macos"))]
    pub type Fixed = i32;
    /// cbindgen:ignore
    #[cfg(target_os = "macos")]
    pub type Fixed = u32;
    pub type OTTag = u32;
    pub type GlyphID = u16;
    /// cbindgen:ignore
    pub type XeTeXFont = *mut Font;
    /// cbindgen:ignore
    pub type XeTeXLayoutEngine = *mut LayoutEngine;
    #[cfg(not(target_os = "macos"))]
    pub(crate) type RawPlatformFontRef = *mut fc::sys::FcPattern;
    #[cfg(target_os = "macos")]
    pub(crate) type RawPlatformFontRef = tectonic_mac_core::sys::CTFontDescriptorRef;
    /// cbindgen:ignore
    #[cfg(not(target_os = "macos"))]
    pub(crate) type PlatformFontRef = fc::Pattern;
    /// cbindgen:ignore
    #[cfg(target_os = "macos")]
    pub(crate) type PlatformFontRef = tectonic_mac_core::CTFontDescriptor;

    /// key is combined value representing `(font_id << 16) + glyph`
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

    /// cbindgen:ignore
    mod ext {
        use tectonic_bridge_core::FileFormat;
        use tectonic_io_base::InputHandle;

        #[allow(improper_ctypes)]
        extern "C" {
            pub fn ttstub_input_open(
                path: *const libc::c_char,
                format: FileFormat,
                is_gz: libc::c_int,
            ) -> *mut InputHandle;
            pub fn ttstub_input_get_size(handle: *mut InputHandle) -> usize;
            pub fn ttstub_input_read(
                handle: *mut InputHandle,
                data: *mut libc::c_char,
                len: usize,
            ) -> isize;
            pub fn ttstub_input_close(handle: *mut InputHandle) -> libc::c_int;
        }
    }
    pub use ext::*;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
