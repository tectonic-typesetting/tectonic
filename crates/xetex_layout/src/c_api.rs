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

/// key is combined value representing `(font_id, glyph)`
/// value is glyph bounding box in TeX points
///
/// `font_id` is XeTeX's internal font number, which is assigned in order of
/// first use and starts over on every engine run, while this map is
/// process-global. It must therefore not outlive the run that filled it:
/// [`clear_glyph_boxes`] empties it when the run's font manager is destroyed.
static GLYPH_BOXES: Mutex<BTreeMap<(u16, u16), GlyphBBox>> = Mutex::new(BTreeMap::new());

/// Forget every cached glyph box.
///
/// Called at the end of each engine run: when a document needs several passes
/// they run in the same process, and a font that gets a different number in a
/// later pass would otherwise be given the box of whatever font had that number
/// before.
pub(crate) fn clear_glyph_boxes() {
    GLYPH_BOXES.lock().unwrap().clear();
}

#[no_mangle]
pub unsafe extern "C" fn getCachedGlyphBBox(
    font_id: u16,
    glyph_id: u16,
    bbox: *mut GlyphBBox,
) -> i32 {
    match GLYPH_BOXES.lock().unwrap().get(&(font_id, glyph_id)) {
        Some(val) => {
            *bbox = *val;
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn cacheGlyphBBox(font_id: u16, glyph_id: u16, bbox: *const GlyphBBox) {
    GLYPH_BOXES
        .lock()
        .unwrap()
        .insert((font_id, glyph_id), *bbox);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Glyph boxes are keyed by a per-run font number, so none may survive
    /// the destruction of the font manager at the end of a run.
    #[test]
    fn glyph_boxes_do_not_outlive_the_font_manager() {
        let cached = GlyphBBox {
            x_min: 0.0,
            y_min: -1.0,
            x_max: 2.0,
            y_max: 37.0,
        };
        let mut out = GlyphBBox::default();

        unsafe {
            cacheGlyphBBox(7, 9, &cached);
            assert_eq!(getCachedGlyphBBox(7, 9, &mut out), 1);
            assert_eq!(out, cached);

            manager::destroy_font_manager();

            assert_eq!(getCachedGlyphBBox(7, 9, &mut out), 0);
        }
    }
}
