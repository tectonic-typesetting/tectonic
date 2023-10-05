// Copyright 2020-2023 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the Harfbuzz *C/C++* API into the Cargo framework, as well as
//! provide bindings to other tectonic crates.

#![allow(non_camel_case_types)]

/// Import something from our bridge crates so that we ensure that we actually
/// link with them, to pull in the symbols defined in the C APIs.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_bridge_graphite2 as clippyrenamehack1;
}

const fn hb_tag(text: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*text)
}

pub const HB_OT_TAG_GSUB: u32 = hb_tag(b"GSUB");
pub const HB_OT_TAG_GPOS: u32 = hb_tag(b"GPOS");

pub type hb_bool_t = libc::c_int;
pub type hb_codepoint_t = u32;
pub type hb_position_t = i32;
pub type hb_tag_t = u32;

pub type hb_destroy_func_t = unsafe extern "C" fn(user_data: *mut ());
pub type hb_font_get_nominal_glyph_func_t = unsafe extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut (),
    unicode: hb_codepoint_t,
    glyph: *mut hb_codepoint_t,
    user_data: *mut (),
) -> hb_bool_t;
pub type hb_font_get_variation_glyph_func_t = unsafe extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut (),
    unicode: hb_codepoint_t,
    variation_selector: hb_codepoint_t,
    glyph: *mut hb_codepoint_t,
    user_data: *mut (),
) -> hb_bool_t;
pub type hb_font_get_glyph_advance_func_t = unsafe extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut (),
    glyph: hb_codepoint_t,
    user_data: *mut (),
) -> hb_position_t;
pub type hb_font_get_glyph_h_advance_func_t = hb_font_get_glyph_advance_func_t;
pub type hb_font_get_glyph_v_advance_func_t = hb_font_get_glyph_advance_func_t;
pub type hb_font_get_glyph_origin_func_t = unsafe extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut (),
    glyph: hb_codepoint_t,
    x: *mut hb_position_t,
    y: *mut hb_position_t,
    user_data: *mut (),
) -> hb_bool_t;
pub type hb_font_get_glyph_h_origin_func_t = hb_font_get_glyph_origin_func_t;
pub type hb_font_get_glyph_v_origin_func_t = hb_font_get_glyph_origin_func_t;
pub type hb_font_get_glyph_kerning_func_t = unsafe extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut (),
    first_glyph: hb_codepoint_t,
    second_glyph: hb_codepoint_t,
    user_data: *mut (),
) -> hb_position_t;
pub type hb_font_get_glyph_h_kerning_func_t = hb_font_get_glyph_kerning_func_t;
pub type hb_font_get_glyph_v_kerning_func_t = hb_font_get_glyph_kerning_func_t;
pub type hb_font_get_glyph_extents_func_t = unsafe extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut (),
    glyph: hb_codepoint_t,
    extents: *mut hb_glyph_extents_t,
    user_data: *mut (),
) -> hb_bool_t;
pub type hb_font_get_glyph_contour_point_func_t = unsafe extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut (),
    glyph: hb_codepoint_t,
    point_index: libc::c_uint,
    x: *mut hb_position_t,
    y: *mut hb_position_t,
    user_data: *mut (),
) -> hb_bool_t;
pub type hb_font_get_glyph_name_func_t = unsafe extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut (),
    glyph: hb_codepoint_t,
    name: *mut libc::c_char,
    size: libc::c_uint,
    user_data: *mut (),
) -> hb_bool_t;
pub type hb_reference_table_func_t =
    unsafe extern "C" fn(face: *mut hb_face_t, tag: hb_tag_t, user_data: *mut ()) -> *mut hb_blob_t;

#[repr(C)]
pub struct hb_font_t(());

#[repr(C)]
pub struct hb_font_funcs_t(());

#[repr(C)]
pub struct hb_face_t(());

#[repr(C)]
pub struct hb_blob_t(());

#[repr(C)]
pub struct hb_glyph_extents_t {
    pub x_bearing: hb_position_t,
    pub y_bearing: hb_position_t,
    pub width: hb_position_t,
    pub height: hb_position_t,
}

#[repr(C)]
pub enum hb_memory_mode_t {
    Duplicate,
    ReadOnly,
    Writable,
    ReadOnlyMayMakeWritable,
}

#[link(name = "harfbuzz", kind = "static")]
extern "C" {
    pub fn hb_font_funcs_create() -> *mut hb_font_funcs_t;
    pub fn hb_font_funcs_set_nominal_glyph_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_nominal_glyph_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_variation_glyph_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_variation_glyph_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_h_advance_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_h_advance_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_v_advance_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_v_advance_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_h_origin_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_h_origin_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_v_origin_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_v_origin_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_h_kerning_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_h_kerning_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_v_kerning_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_v_kerning_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_extents_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_extents_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_contour_point_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_contour_point_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_funcs_set_glyph_name_func(
        ffuncs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_name_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_blob_create(
        data: *mut libc::c_char,
        length: libc::c_uint,
        mode: hb_memory_mode_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    ) -> *mut hb_blob_t;
    pub fn hb_face_create_for_tables(
        reference_table_func: hb_reference_table_func_t,
        user_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    ) -> *mut hb_face_t;
    pub fn hb_face_set_index(face: *mut hb_face_t, index: libc::c_uint);
    pub fn hb_face_set_upem(face: *mut hb_face_t, upem: libc::c_uint);
    pub fn hb_face_destroy(face: *mut hb_face_t);
    pub fn hb_font_create(face: *mut hb_face_t) -> *mut hb_font_t;
    pub fn hb_font_set_funcs(
        font: *mut hb_font_t,
        klass: *mut hb_font_funcs_t,
        font_data: *mut (),
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_font_set_scale(font: *mut hb_font_t, x_scale: libc::c_int, y_scale: libc::c_int);
    pub fn hb_font_set_ppem(font: *mut hb_font_t, x_ppem: libc::c_uint, y_ppem: libc::c_uint);
    pub fn hb_font_destroy(font: *mut hb_font_t);
    pub fn hb_font_get_face(font: *mut hb_font_t) -> *mut hb_face_t;
    pub fn hb_ot_layout_table_get_script_tags(
        face: *mut hb_face_t,
        table_tag: hb_tag_t,
        start_offset: libc::c_uint,
        script_count: *mut libc::c_uint,
        script_tags: *mut hb_tag_t,
    ) -> libc::c_uint;
    pub fn hb_ot_layout_script_get_language_tags(
        face: *mut hb_face_t,
        table_tag: hb_tag_t,
        script_index: libc::c_uint,
        start_offset: libc::c_uint,
        language_count: *mut libc::c_uint,
        script_tags: *mut hb_tag_t,
    ) -> libc::c_uint;
    pub fn hb_ot_layout_table_find_script(
        face: *mut hb_face_t,
        table_tag: hb_tag_t,
        script_tag: hb_tag_t,
        script_index: *mut libc::c_uint,
    ) -> hb_bool_t;
    pub fn hb_ot_layout_script_select_language(
        face: *mut hb_face_t,
        table_tag: hb_tag_t,
        script_index: libc::c_uint,
        language_count: libc::c_uint,
        language_tags: *const hb_tag_t,
        language_index: *mut libc::c_uint,
    ) -> hb_bool_t;
    pub fn hb_ot_layout_language_get_feature_tags(
        face: *mut hb_face_t,
        table_tag: hb_tag_t,
        script_index: libc::c_uint,
        language_index: libc::c_uint,
        start_offset: libc::c_uint,
        feature_count: *mut libc::c_uint,
        feature_tags: *mut hb_tag_t,
    ) -> libc::c_uint;
}

#[test]
fn linkage() {}
