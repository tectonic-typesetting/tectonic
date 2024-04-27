use crate::c_api::{Fixed, OTTag, RawPlatformFontRef, XeTeXFont};
use crate::font::{get_larger_script_list_table_ot, Font};
use crate::utils::{d_to_fix, fix_to_d, raw_to_rs};
use std::ffi::{CStr, CString};
use std::ptr;
use tectonic_bridge_freetype2 as ft;
use tectonic_bridge_harfbuzz as hb;

#[no_mangle]
pub unsafe extern "C" fn hasFontTable(font: XeTeXFont, table_tag: OTTag) -> bool {
    // TODO: has_font_table for efficiency
    (*font)
        .load_font_table(ft::TableTag::Other(table_tag))
        .is_some()
}

#[no_mangle]
pub unsafe extern "C" fn getSlant(font: XeTeXFont) -> Fixed {
    let angle = (*font).italic_angle() as f64;
    d_to_fix(f64::tan(-angle * std::f64::consts::PI / 180.0))
}

#[no_mangle]
pub unsafe extern "C" fn countGlyphs(font: XeTeXFont) -> libc::c_uint {
    (*font).num_glyphs() as libc::c_uint
}

#[no_mangle]
pub unsafe extern "C" fn getGlyphWidth(font: XeTeXFont, gid: u32) -> f32 {
    (*font).get_glyph_width(gid)
}

#[no_mangle]
pub unsafe extern "C" fn setFontLayoutDir(font: XeTeXFont, vertical: libc::c_int) {
    (*font).set_layout_dir_vertical(vertical != 0)
}

#[no_mangle]
pub unsafe extern "C" fn getIndScript(font: XeTeXFont, index: libc::c_uint) -> hb::Tag {
    get_larger_script_list_table_ot(&*font)
        .script(index as usize)
        .map_or(hb::Tag::new(0), |s| s.tag())
}

#[no_mangle]
pub unsafe extern "C" fn getIndLanguage(
    font: XeTeXFont,
    script: hb::Tag,
    index: libc::c_uint,
) -> hb::Tag {
    let index = index as usize;
    let script = get_larger_script_list_table_ot(&*font).find_script(script);

    if let Some(script) = script {
        if let Some(lang) = script.lang(index) {
            return lang.tag();
        }
        let lang = script.swap_table().and_then(|s| s.lang(index));
        if let Some(lang) = lang {
            return lang.tag();
        }
    }

    hb::Tag::new(0)
}

#[no_mangle]
pub unsafe extern "C" fn getIndFeature(
    font: XeTeXFont,
    script: hb::Tag,
    language: hb::Tag,
    index: libc::c_uint,
) -> hb::Tag {
    let mut index = index as usize;
    let layout = (*font).hb_font().face().ot_layout();

    for table_tag in [hb::GTag::GSub, hb::GTag::GPos] {
        let table = layout.table(table_tag);
        if let Some(script) = table.find_script(script) {
            let lang = script.select_lang(&[language]);
            if lang.is_ok() || language == hb::Tag::new(0) {
                let lang = lang.unwrap_or_else(|l| l);
                let feat = lang.feature(index);

                if let Some(feat) = feat {
                    return feat;
                }

                index -= lang.feature_tags_len();
            }
        }
    }

    hb::Tag::new(0)
}

#[no_mangle]
pub unsafe extern "C" fn getGlyphName(
    font: XeTeXFont,
    gid: u16,
    len: *mut libc::c_int,
) -> *const libc::c_char {
    match (*font).get_glyph_name(gid) {
        Some(out) => {
            *len = out.as_bytes().len() as libc::c_int;
            CString::into_raw(out)
        }
        None => {
            *len = 0;
            ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn freeGlyphName(name: *mut libc::c_char) {
    let _ = CString::from_raw(name);
}

#[no_mangle]
pub unsafe extern "C" fn ttxl_font_units_to_points(font: XeTeXFont, units: f32) -> f32 {
    (*font).units_to_points(units as f64) as f32
}

#[no_mangle]
pub unsafe extern "C" fn ttxl_font_points_to_units(font: XeTeXFont, points: f32) -> f32 {
    (*font).points_to_units(points as f64) as f32
}

#[no_mangle]
pub unsafe extern "C" fn ttxl_font_get_point_size(font: XeTeXFont) -> f32 {
    (*font).point_size()
}

#[no_mangle]
pub unsafe extern "C" fn createFont(font_ref: RawPlatformFontRef, point_size: Fixed) -> XeTeXFont {
    let font_ref = match raw_to_rs(font_ref) {
        Some(fr) => fr,
        None => return ptr::null_mut(),
    };

    match Font::new(font_ref, fix_to_d(point_size) as f32) {
        Err(_) => ptr::null_mut(),
        Ok(out) => Box::into_raw(Box::new(out)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn createFontFromFile(
    filename: *const libc::c_char,
    index: libc::c_int,
    point_size: Fixed,
) -> XeTeXFont {
    let filename = if filename.is_null() {
        None
    } else {
        Some(CStr::from_ptr(filename))
    };

    match Font::new_path_index(filename, index as usize, fix_to_d(point_size) as f32) {
        Err(_) => ptr::null_mut(),
        Ok(out) => Box::into_raw(Box::new(out)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn deleteFont(font: XeTeXFont) {
    let _ = Box::from_raw(font);
}

#[no_mangle]
pub unsafe extern "C" fn countScripts(font: XeTeXFont) -> libc::c_uint {
    get_larger_script_list_table_ot(&*font).script_tags_len() as libc::c_uint
}

#[no_mangle]
pub unsafe extern "C" fn countLanguages(font: XeTeXFont, script: hb::Tag) -> libc::c_uint {
    let table = get_larger_script_list_table_ot(&*font);
    let script = table.find_script(script);
    let out = match script {
        Some(script) => {
            script.language_tags_len()
                + script
                    .swap_table()
                    .map(|s| s.language_tags_len())
                    .unwrap_or(0)
        }
        None => 0,
    };
    out as libc::c_uint
}

#[no_mangle]
pub unsafe extern "C" fn countFeatures(
    font: XeTeXFont,
    script: hb::Tag,
    language: hb::Tag,
) -> libc::c_uint {
    let layout = (*font).hb_font().face().ot_layout();

    let mut rval = 0;
    for table_tag in [hb::GTag::GSub, hb::GTag::GPos] {
        let table = layout.table(table_tag);
        if let Some(script) = table.find_script(script) {
            let lang = script.select_lang(&[language]);
            if lang.is_ok() || language == hb::Tag::new(0) {
                rval += lang.unwrap_or_else(|l| l).feature_tags_len();
            }
        }
    }
    rval as libc::c_uint
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn getFileNameFromCTFont(
    ct_font: CTFontRef,
    index: *mut u32,
) -> *const libc::c_char {
    use std::ptr::NonNull;
    crate::c_api::font::get_file_name_from_ct_font(
        &CTFont::new_borrowed(NonNull::new(ct_font.cast_mut()).unwrap()),
        &mut *index,
    )
    .map(CString::into_raw)
    .unwrap_or(ptr::null_mut())
}
