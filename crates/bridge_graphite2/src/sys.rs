#![allow(nonstandard_style)]

pub const gr_breakNone: libc::c_int = 0;
pub const gr_breakWord: libc::c_int = 15;
pub const gr_breakBeforeWord: libc::c_int = -15;

#[repr(C)]
pub struct gr_face(());

#[repr(C)]
pub struct gr_feature_ref(());

#[repr(C)]
pub struct gr_feature_val(());

#[repr(C)]
pub struct gr_font(());

#[repr(C)]
pub struct gr_segment(());

#[repr(C)]
pub struct gr_slot(());

#[repr(C)]
pub struct gr_char_info(());

#[repr(C)]
pub enum gr_encform {
    utf8,
    utf16,
    utf32,
}

extern "C" {
    pub fn gr_face_n_fref(pFace: *const gr_face) -> libc::c_ushort;
    pub fn gr_face_fref(pFace: *const gr_face, i: u16) -> *const gr_feature_ref;
    pub fn gr_fref_id(pfeatureref: *const gr_feature_ref) -> u32;
    pub fn gr_face_find_fref(pFace: *const gr_face, featId: u32) -> *const gr_feature_ref;
    pub fn gr_fref_n_values(pfeatureref: *const gr_feature_ref) -> u16;
    pub fn gr_fref_value(pfeatureref: *const gr_feature_ref, settingno: u16) -> i16;
    pub fn gr_face_featureval_for_lang(pFace: *const gr_face, langname: u32)
        -> *mut gr_feature_val;
    pub fn gr_fref_feature_value(
        pfeatureref: *const gr_feature_ref,
        feats: *const gr_feature_val,
    ) -> u16;
    pub fn gr_fref_label(
        pfeatureref: *const gr_feature_ref,
        langId: *mut u16,
        utf: gr_encform,
        length: *mut u32,
    ) -> *mut libc::c_void;
    pub fn gr_fref_value_label(
        pfeatureref: *const gr_feature_ref,
        setting: u16,
        lang_id: *mut u16,
        utf: gr_encform,
        length: *mut u32,
    ) -> *mut libc::c_void;
    pub fn gr_label_destroy(label: *mut libc::c_void);
    pub fn gr_make_font(ppm: f32, face: *const gr_face) -> *mut gr_font;
    pub fn gr_seg_destroy(p: *mut gr_segment);
    pub fn gr_fref_set_feature_value(
        pfeatureref: *const gr_feature_ref,
        val: u16,
        pDest: *mut gr_feature_val,
    ) -> libc::c_int;
    pub fn gr_make_seg(
        font: *const gr_font,
        face: *const gr_face,
        script: u32,
        pFeats: *const gr_feature_val,
        enc: gr_encform,
        pStart: *const libc::c_void,
        nChars: libc::size_t,
        dir: libc::c_int,
    ) -> *mut gr_segment;
    pub fn gr_seg_first_slot(pSeg: *mut gr_segment) -> *const gr_slot;
    pub fn gr_seg_last_slot(pSeg: *mut gr_segment) -> *const gr_slot;
    pub fn gr_slot_next_in_segment(p: *const gr_slot) -> *const gr_slot;
    pub fn gr_seg_cinfo(pSeg: *const gr_segment, index: libc::c_uint) -> *const gr_char_info;
    pub fn gr_slot_index(p: *const gr_slot) -> libc::c_uint;
    pub fn gr_cinfo_break_weight(p: *const gr_char_info) -> libc::c_int;
    pub fn gr_cinfo_base(p: *const gr_char_info) -> libc::size_t;
    pub fn gr_featureval_destroy(features: *mut gr_feature_val);
}
