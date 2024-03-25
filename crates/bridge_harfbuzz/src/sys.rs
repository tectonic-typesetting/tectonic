#![allow(non_camel_case_types)]

use std::ptr;
use tectonic_bridge_graphite2::gr_face;

const fn hb_tag(text: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*text)
}

pub const HB_TAG_NONE: u32 = hb_tag(b"\0\0\0\0");
pub const HB_OT_TAG_GSUB: u32 = hb_tag(b"GSUB");
pub const HB_OT_TAG_GPOS: u32 = hb_tag(b"GPOS");

pub type hb_script_t = u32;

pub const HB_SCRIPT_INVALID: u32 = HB_TAG_NONE;

pub type hb_bool_t = libc::c_int;
pub type hb_codepoint_t = u32;
pub type hb_position_t = i32;
pub type hb_tag_t = u32;
pub type hb_mask_t = u32;
pub type hb_language_t = *mut hb_language_impl_t;
pub type hb_ot_name_id_t = libc::c_uint;

#[repr(C)]
pub struct hb_language_impl_t(());

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
pub type hb_unicode_decompose_compatibility_func_t = unsafe extern "C" fn(
    ufuncs: *mut hb_unicode_funcs_t,
    u: hb_codepoint_t,
    decomposed: *mut hb_codepoint_t,
    user_data: *mut libc::c_void,
) -> libc::c_uint;

#[repr(C)]
pub struct hb_font_t(());

#[repr(C)]
pub struct hb_font_funcs_t(());

#[repr(C)]
pub struct hb_face_t(());

#[repr(C)]
pub struct hb_blob_t(());

#[repr(C)]
pub struct hb_buffer_t(());

#[repr(C)]
pub struct hb_shape_plan_t(());

#[repr(C)]
pub struct hb_unicode_funcs_t(());

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct hb_glyph_extents_t {
    pub x_bearing: hb_position_t,
    pub y_bearing: hb_position_t,
    pub width: hb_position_t,
    pub height: hb_position_t,
}

#[repr(C)]
pub struct hb_feature_t {
    pub tag: hb_tag_t,
    pub value: u32,
    start: libc::c_uint,
    end: libc::c_uint,
}

#[repr(C)]
pub enum hb_memory_mode_t {
    Duplicate,
    ReadOnly,
    Writable,
    ReadOnlyMayMakeWritable,
}

#[repr(C)]
pub enum hb_buffer_content_type_t {
    Invalid = 0,
    Unicode,
    Glyphs,
}

#[repr(C)]
pub struct hb_glyph_info_t {
    pub codepoint: hb_codepoint_t,
    mask: hb_mask_t,
    pub cluster: u32,
    var1: hb_var_int_t,
    var2: hb_var_int_t,
}

#[repr(C)]
pub struct hb_glyph_position_t {
    pub x_advance: hb_position_t,
    pub y_advance: hb_position_t,
    pub x_offset: hb_position_t,
    pub y_offset: hb_position_t,
    var: hb_var_int_t,
}

#[repr(C)]
pub struct hb_segment_properties_t {
    pub direction: hb_direction_t,
    pub script: hb_script_t,
    pub language: hb_language_t,
    reserved1: *mut libc::c_void,
    reserved2: *mut libc::c_void,
}

impl Default for hb_segment_properties_t {
    fn default() -> Self {
        hb_segment_properties_t {
            direction: Default::default(),
            script: 0,
            language: ptr::null_mut(),
            reserved1: ptr::null_mut(),
            reserved2: ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub union hb_var_int_t {
    u32: u32,
    i32: i32,
    u16: [u16; 2],
    i16: [i16; 2],
    u8: [u8; 4],
    i8: [i8; 4],
}

#[derive(Default, PartialEq)]
#[repr(C)]
pub enum hb_direction_t {
    #[default]
    Invalid = 0,
    Ltr = 4,
    Rtl,
    Ttb,
    Btt,
}

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
    pub fn hb_graphite2_face_get_gr_face(face: *mut hb_face_t) -> *mut gr_face;
    pub fn hb_language_from_string(str: *const libc::c_char, len: libc::c_int) -> hb_language_t;
    pub fn hb_tag_from_string(str: *const libc::c_char, len: libc::c_int) -> hb_tag_t;
    pub fn hb_ot_tag_to_language(tag: hb_tag_t) -> hb_language_t;
    pub fn hb_buffer_create() -> *mut hb_buffer_t;
    pub fn hb_buffer_destroy(buffer: *mut hb_buffer_t);
    pub fn hb_buffer_get_length(buffer: *const hb_buffer_t) -> libc::c_uint;
    pub fn hb_buffer_get_glyph_infos(
        buffer: *mut hb_buffer_t,
        length: *mut libc::c_uint,
    ) -> *mut hb_glyph_info_t;
    pub fn hb_buffer_get_glyph_positions(
        buffer: *mut hb_buffer_t,
        length: *mut libc::c_uint,
    ) -> *mut hb_glyph_position_t;
    pub fn hb_language_to_string(lang: hb_language_t) -> *const libc::c_char;
    pub fn hb_buffer_get_script(buffer: *const hb_buffer_t) -> hb_script_t;
    pub fn hb_script_get_horizontal_direction(script: hb_script_t) -> hb_direction_t;
    pub fn hb_ot_math_has_data(face: *mut hb_face_t) -> hb_bool_t;
    pub fn hb_ot_tag_to_script(tag: hb_tag_t) -> hb_script_t;
    pub fn hb_buffer_reset(buffer: *mut hb_buffer_t);
    pub fn hb_buffer_add_utf16(
        buffer: *mut hb_buffer_t,
        text: *const u16,
        text_length: libc::c_int,
        item_offset: libc::c_uint,
        item_length: libc::c_int,
    );
    pub fn hb_buffer_set_direction(buffer: *mut hb_buffer_t, direction: hb_direction_t);
    pub fn hb_buffer_set_script(buffer: *mut hb_buffer_t, script: hb_script_t);
    pub fn hb_buffer_set_language(buffer: *mut hb_buffer_t, language: hb_language_t);
    pub fn hb_buffer_guess_segment_properties(buffer: *mut hb_buffer_t);
    pub fn hb_buffer_get_segment_properties(
        buffer: *const hb_buffer_t,
        props: *mut hb_segment_properties_t,
    );
    pub fn hb_shape_plan_create(
        bufferface: *mut hb_face_t,
        props: *const hb_segment_properties_t,
        user_features: *const hb_feature_t,
        num_user_features: libc::c_uint,
        shaper_list: *const *const libc::c_char,
    ) -> *mut hb_shape_plan_t;
    pub fn hb_shape_plan_create_cached(
        face: *mut hb_face_t,
        props: *const hb_segment_properties_t,
        user_features: *const hb_feature_t,
        num_user_features: libc::c_uint,
        shaper_list: *const *const libc::c_char,
    ) -> *mut hb_shape_plan_t;
    pub fn hb_shape_plan_execute(
        shape_plan: *mut hb_shape_plan_t,
        font: *mut hb_font_t,
        buffer: *mut hb_buffer_t,
        features: *const hb_feature_t,
        num_features: libc::c_uint,
    ) -> hb_bool_t;
    pub fn hb_shape_plan_get_shaper(shape_plan: *mut hb_shape_plan_t) -> *const libc::c_char;
    pub fn hb_shape_plan_destroy(shape_plane: *mut hb_shape_plan_t);
    pub fn hb_buffer_set_content_type(
        buffer: *mut hb_buffer_t,
        content_type: hb_buffer_content_type_t,
    );
    pub fn hb_icu_get_unicode_funcs() -> *mut hb_unicode_funcs_t;
    pub fn hb_unicode_funcs_create(parent: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;
    pub fn hb_unicode_funcs_set_decompose_compatibility_func(
        ufuncs: *mut hb_unicode_funcs_t,
        func: hb_unicode_decompose_compatibility_func_t,
        user_data: *mut libc::c_void,
        destroy: Option<hb_destroy_func_t>,
    );
    pub fn hb_buffer_set_unicode_funcs(
        buffer: *mut hb_buffer_t,
        unicode_funcs: *mut hb_unicode_funcs_t,
    );
    pub fn hb_version_atleast(
        major: libc::c_uint,
        minor: libc::c_uint,
        patch: libc::c_uint,
    ) -> hb_bool_t;
    pub fn hb_font_get_ptem(font: *mut hb_font_t) -> f32;
    pub fn hb_ot_layout_get_size_params(
        face: *mut hb_face_t,
        design_size: *mut libc::c_uint,
        subfamily_id: *mut libc::c_uint,
        subfamily_name_id: *mut hb_ot_name_id_t,
        range_start: *mut libc::c_uint,
        range_end: *mut libc::c_uint,
    ) -> hb_bool_t;
}
