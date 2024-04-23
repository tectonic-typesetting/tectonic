// Copyright 2020-2023 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the Harfbuzz *C/C++* API into the Cargo framework, as well as
//! provide bindings to other tectonic crates.

#![allow(clippy::unnecessary_cast)]

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::{ptr, slice};
use tectonic_bridge_graphite2 as gr;

mod font_funcs;
pub mod sys;

pub use font_funcs::{FontFuncs, FontFuncsMut, FontFuncsRef};
pub use sys::hb_buffer_content_type_t as BufferContentType;
pub use sys::hb_codepoint_t as Codepoint;
pub use sys::hb_direction_t as Direction;
pub use sys::hb_feature_t as Feature;
pub use sys::hb_glyph_extents_t as GlyphExtents;
pub use sys::hb_glyph_info_t as GlyphInfo;
pub use sys::hb_glyph_position_t as GlyphPosition;
pub use sys::hb_memory_mode_t as MemoryMode;
pub use sys::hb_ot_name_id_t as OtNameId;
pub use sys::hb_position_t as Position;
pub use sys::hb_segment_properties_t as SegmentProperties;

/// Import something from our bridge crates so that we ensure that we actually
/// link with them, to pull in the symbols defined in the C APIs.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_bridge_graphite2 as clippyrenamehack1;
}

unsafe extern "C" fn dealloc<T: 'static>(user_data: *mut ()) {
    let _ = unsafe { Box::from_raw(user_data.cast::<T>()) };
}

#[repr(transparent)]
pub struct Script(sys::hb_script_t);

impl Script {
    pub const INVALID: Script = Script(sys::HB_SCRIPT_INVALID);

    pub fn get_horizontal_direction(&self) -> Direction {
        unsafe { sys::hb_script_get_horizontal_direction(self.0) }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Tag(sys::hb_tag_t);

impl Tag {
    pub fn new(val: u32) -> Tag {
        Tag(val)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(val: &str) -> Tag {
        Tag(unsafe { sys::hb_tag_from_string(val.as_ptr().cast(), val.len() as libc::c_int) })
    }

    pub fn from_cstr(val: &CStr) -> Tag {
        Tag(unsafe { sys::hb_tag_from_string(val.as_ptr(), -1) })
    }

    pub fn to_raw(self) -> u32 {
        self.0
    }

    pub fn to_script(self) -> Script {
        Script(unsafe { sys::hb_ot_tag_to_script(self.0) })
    }

    pub fn to_language(self) -> Language {
        Language(unsafe { sys::hb_ot_tag_to_language(self.0) })
    }
}

#[derive(Copy, Clone)]
pub struct Language(*mut sys::hb_language_impl_t);

impl Language {
    pub fn from_string(str: &str) -> Language {
        Language(unsafe {
            sys::hb_language_from_string(str.as_ptr().cast(), str.len() as libc::c_int)
        })
    }

    pub fn from_cstr(str: &CStr) -> Language {
        Language(unsafe { sys::hb_language_from_string(str.as_ptr(), -1) })
    }

    pub fn to_string(&self) -> Option<&CStr> {
        let ptr = unsafe { sys::hb_language_to_string(self.0) };
        // ptr may be null if we have HB_LANGUAGE_INVALID
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        // This gets HB_LANGUAGE_INVALID
        Language(unsafe { sys::hb_language_from_string(ptr::null(), -1) })
    }
}

#[derive(Copy, Clone)]
pub struct BufferRef<'a>(NonNull<sys::hb_buffer_t>, PhantomData<&'a sys::hb_buffer_t>);

impl<'a> BufferRef<'a> {
    fn as_ptr(self) -> *mut sys::hb_buffer_t {
        self.0.as_ptr()
    }

    pub fn len(self) -> usize {
        unsafe { sys::hb_buffer_get_length(self.as_ptr()) as usize }
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    pub fn get_glyph_info(self) -> &'a [GlyphInfo] {
        let mut len = 0;
        let ptr = unsafe { sys::hb_buffer_get_glyph_infos(self.as_ptr(), &mut len) };
        unsafe { slice::from_raw_parts(ptr, len as usize) }
    }

    pub fn get_glyph_position(self) -> &'a [GlyphPosition] {
        let mut len = 0;
        let ptr = unsafe { sys::hb_buffer_get_glyph_positions(self.as_ptr(), &mut len) };
        unsafe { slice::from_raw_parts(ptr, len as usize) }
    }

    pub fn get_script(self) -> Script {
        Script(unsafe { sys::hb_buffer_get_script(self.as_ptr()) })
    }

    pub fn get_segment_properties(self) -> SegmentProperties {
        let mut props = SegmentProperties::default();
        unsafe { sys::hb_buffer_get_segment_properties(self.as_ptr(), &mut props) };
        props
    }
}

pub struct BufferMut<'a>(BufferRef<'a>, PhantomData<&'a mut sys::hb_buffer_t>);

impl BufferMut<'_> {
    fn as_ptr_mut(&mut self) -> *mut sys::hb_buffer_t {
        self.0.as_ptr()
    }

    pub fn set_content_type(&mut self, content: BufferContentType) {
        unsafe { sys::hb_buffer_set_content_type(self.as_ptr_mut(), content) }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        unsafe { sys::hb_buffer_set_direction(self.as_ptr_mut(), direction) }
    }

    pub fn set_language(&mut self, lang: Language) {
        unsafe { sys::hb_buffer_set_language(self.as_ptr_mut(), lang.0) }
    }

    pub fn set_script(&mut self, script: Script) {
        unsafe { sys::hb_buffer_set_script(self.as_ptr_mut(), script.0) }
    }

    pub fn add_utf16(&mut self, text: &[u16], offset: usize, len: usize) {
        unsafe {
            sys::hb_buffer_add_utf16(
                self.as_ptr_mut(),
                text.as_ptr(),
                text.len() as libc::c_int,
                offset as libc::c_uint,
                len as libc::c_int,
            )
        }
    }

    pub fn guess_segment_properties(&mut self) {
        unsafe { sys::hb_buffer_guess_segment_properties(self.as_ptr_mut()) }
    }

    pub fn reset(&mut self) {
        unsafe { sys::hb_buffer_reset(self.as_ptr_mut()) }
    }
}

impl<'a> Deref for BufferMut<'a> {
    type Target = BufferRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Buffer(NonNull<sys::hb_buffer_t>);

impl Buffer {
    pub fn new() -> Buffer {
        let ptr = unsafe { sys::hb_buffer_create() };
        Buffer(NonNull::new(ptr).unwrap())
    }

    pub fn as_ref(&self) -> BufferRef<'_> {
        BufferRef(self.0, PhantomData)
    }

    pub fn as_mut(&mut self) -> BufferMut<'_> {
        BufferMut(self.as_ref(), PhantomData)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { sys::hb_buffer_destroy(self.0.as_ptr()) }
    }
}

pub struct Blob(NonNull<sys::hb_blob_t>);

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

        Blob(NonNull::new(raw).unwrap())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GTag {
    GPos,
    GSub,
}

pub struct LayoutSizeParams {
    pub design_size: u32,
    pub subfamily_id: u32,
    pub subfamily_name_id: OtNameId,
    pub start: u32,
    pub end: u32,
}

#[derive(Copy, Clone)]
pub struct FaceRef<'a>(NonNull<sys::hb_face_t>, PhantomData<&'a sys::hb_face_t>);

impl<'a> FaceRef<'a> {
    fn as_ptr(self) -> *mut sys::hb_face_t {
        self.0.as_ptr()
    }

    pub fn has_ot_math_data(self) -> bool {
        unsafe { sys::hb_ot_math_has_data(self.as_ptr()) != 0 }
    }

    pub fn get_ot_layout_size_params(self) -> Option<LayoutSizeParams> {
        let mut design_size = 0;
        let mut subfamily_id = 0;
        let mut subfamily_name_id = 0;
        let mut start = 0;
        let mut end = 0;

        let res = unsafe {
            sys::hb_ot_layout_get_size_params(
                self.as_ptr(),
                &mut design_size,
                &mut subfamily_id,
                &mut subfamily_name_id,
                &mut start,
                &mut end,
            )
        };

        if res != 0 {
            Some(LayoutSizeParams {
                design_size: design_size as u32,
                subfamily_id: subfamily_id as u32,
                subfamily_name_id,
                start: start as u32,
                end: end as u32,
            })
        } else {
            None
        }
    }

    pub fn get_ot_layout_script_tags(self, tag: GTag) -> Vec<Tag> {
        let tag = Tag::new(match tag {
            GTag::GPos => sys::HB_OT_TAG_GPOS,
            GTag::GSub => sys::HB_OT_TAG_GSUB,
        });

        let mut len = unsafe {
            sys::hb_ot_layout_table_get_script_tags(
                self.as_ptr(),
                tag.0,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        let mut out = vec![Tag::new(0); len as usize];
        unsafe {
            sys::hb_ot_layout_table_get_script_tags(
                self.as_ptr(),
                tag.0,
                0,
                &mut len,
                out.as_mut_ptr().cast(),
            )
        };
        assert_eq!(len as usize, out.len());
        out
    }

    pub fn get_ot_layout_script_language_tags_len(self, tag: GTag, idx: usize) -> usize {
        let tag = Tag::new(match tag {
            GTag::GPos => sys::HB_OT_TAG_GPOS,
            GTag::GSub => sys::HB_OT_TAG_GSUB,
        });

        let len = unsafe {
            sys::hb_ot_layout_script_get_language_tags(
                self.as_ptr(),
                tag.0,
                idx as libc::c_uint,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        len as usize
    }

    pub fn get_ot_layout_script_language_tags(self, tag: GTag, idx: usize) -> Vec<Tag> {
        let tag = Tag::new(match tag {
            GTag::GPos => sys::HB_OT_TAG_GPOS,
            GTag::GSub => sys::HB_OT_TAG_GSUB,
        });

        let mut len = unsafe {
            sys::hb_ot_layout_script_get_language_tags(
                self.as_ptr(),
                tag.0,
                idx as libc::c_uint,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        let mut out = vec![Tag::new(0); len as usize];
        unsafe {
            sys::hb_ot_layout_script_get_language_tags(
                self.as_ptr(),
                tag.0,
                idx as libc::c_uint,
                0,
                &mut len,
                out.as_mut_ptr().cast(),
            )
        };
        assert_eq!(len as usize, out.len());
        out
    }

    pub fn get_ot_layout_language_feature_tags_len(
        self,
        tag: GTag,
        script_index: usize,
        lang_index: usize,
    ) -> usize {
        let tag = Tag::new(match tag {
            GTag::GPos => sys::HB_OT_TAG_GPOS,
            GTag::GSub => sys::HB_OT_TAG_GSUB,
        });

        let len = unsafe {
            sys::hb_ot_layout_language_get_feature_tags(
                self.as_ptr(),
                tag.0,
                script_index as libc::c_uint,
                lang_index as libc::c_uint,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        len as usize
    }

    pub fn get_ot_layout_language_feature_tags(
        self,
        tag: GTag,
        script_index: usize,
        lang_index: usize,
    ) -> Vec<Tag> {
        let tag = Tag::new(match tag {
            GTag::GPos => sys::HB_OT_TAG_GPOS,
            GTag::GSub => sys::HB_OT_TAG_GSUB,
        });

        let mut len = unsafe {
            sys::hb_ot_layout_language_get_feature_tags(
                self.as_ptr(),
                tag.0,
                script_index as libc::c_uint,
                lang_index as libc::c_uint,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        let mut out = vec![Tag::new(0); len as usize];
        unsafe {
            sys::hb_ot_layout_language_get_feature_tags(
                self.as_ptr(),
                tag.0,
                script_index as libc::c_uint,
                lang_index as libc::c_uint,
                0,
                &mut len,
                out.as_mut_ptr().cast(),
            )
        };
        assert_eq!(len as usize, out.len());
        out
    }

    pub fn select_ot_layout_language(
        self,
        tag: GTag,
        script_index: usize,
        langs: &[Tag],
    ) -> Result<usize, usize> {
        let tag = Tag::new(match tag {
            GTag::GPos => sys::HB_OT_TAG_GPOS,
            GTag::GSub => sys::HB_OT_TAG_GSUB,
        });

        let mut out_idx = 0;
        let found = unsafe {
            sys::hb_ot_layout_script_select_language(
                self.as_ptr(),
                tag.0,
                script_index as libc::c_uint,
                langs.len() as libc::c_uint,
                langs.as_ptr().cast(),
                &mut out_idx,
            )
        };
        if found != 0 {
            Ok(out_idx as usize)
        } else {
            Err(out_idx as usize)
        }
    }

    pub fn find_ot_layout_script(self, tag: GTag, script: Tag) -> Option<usize> {
        let tag = Tag::new(match tag {
            GTag::GPos => sys::HB_OT_TAG_GPOS,
            GTag::GSub => sys::HB_OT_TAG_GSUB,
        });

        let mut pos = 0;
        let found = unsafe {
            sys::hb_ot_layout_table_find_script(self.as_ptr(), tag.0, script.0, &mut pos)
        };
        if found != 0 {
            Some(pos as usize)
        } else {
            None
        }
    }

    pub fn gr_face(self) -> Option<gr::FaceRef<'a>> {
        let ptr = unsafe { sys::hb_graphite2_face_get_gr_face(self.as_ptr()) };
        NonNull::new(ptr).map(|ptr| unsafe { gr::FaceRef::from_raw(ptr) })
    }
}

pub struct FaceMut<'a>(FaceRef<'a>, PhantomData<&'a mut sys::hb_face_t>);

impl<'a> FaceMut<'a> {
    fn as_ptr_mut(&mut self) -> *mut sys::hb_face_t {
        self.0.as_ptr()
    }

    pub fn set_index(&mut self, index: u32) {
        unsafe { sys::hb_face_set_index(self.as_ptr_mut(), index as libc::c_uint) }
    }

    pub fn set_upem(&mut self, upem: u32) {
        unsafe { sys::hb_face_set_upem(self.as_ptr_mut(), upem as libc::c_uint) }
    }
}
pub struct Face(NonNull<sys::hb_face_t>);

impl Face {
    pub fn new_tables<T: Fn(&FaceRef, Tag) -> Option<Blob> + 'static>(f: T) -> Face {
        unsafe extern "C" fn get_table<T: Fn(&FaceRef, Tag) -> Option<Blob> + 'static>(
            face: *mut sys::hb_face_t,
            tag: sys::hb_tag_t,
            user_data: *mut (),
        ) -> *mut sys::hb_blob_t {
            let f = unsafe { &*user_data.cast::<T>() };
            let face = &*face.cast();
            match f(face, Tag(tag)) {
                Some(blob) => blob.0.as_ptr(),
                None => ptr::null_mut(),
            }
        }

        let face = unsafe {
            sys::hb_face_create_for_tables(
                get_table::<T>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<T>),
            )
        };

        Face(NonNull::new(face).unwrap())
    }

    pub fn as_ref(&self) -> FaceRef<'_> {
        FaceRef(self.0, PhantomData)
    }

    pub fn as_mut(&mut self) -> FaceMut<'_> {
        FaceMut(self.as_ref(), PhantomData)
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        unsafe { sys::hb_face_destroy(self.0.as_ptr()) }
    }
}

#[derive(Copy, Clone)]
pub struct FontRef<'a>(NonNull<sys::hb_font_t>, PhantomData<&'a sys::hb_font_t>);

impl<'a> FontRef<'a> {
    pub fn as_ptr(self) -> *mut sys::hb_font_t {
        self.0.as_ptr()
    }

    pub fn get_face(self) -> FaceRef<'a> {
        let ptr = unsafe { sys::hb_font_get_face(self.as_ptr()) };
        FaceRef(NonNull::new(ptr).unwrap(), PhantomData)
    }

    pub fn get_ptem(self) -> f32 {
        unsafe { sys::hb_font_get_ptem(self.as_ptr()) }
    }
}

pub struct FontMut<'a>(FontRef<'a>, PhantomData<&'a mut sys::hb_font_t>);

impl FontMut<'_> {
    fn as_ptr_mut(&mut self) -> *mut sys::hb_font_t {
        self.0.as_ptr()
    }

    pub fn set_scale(&mut self, x: i32, y: i32) {
        unsafe { sys::hb_font_set_scale(self.as_ptr_mut(), x as libc::c_int, y as libc::c_int) }
    }

    pub fn set_ppem(&mut self, x: u32, y: u32) {
        unsafe { sys::hb_font_set_ppem(self.as_ptr_mut(), x as libc::c_uint, y as libc::c_uint) }
    }

    pub fn set_funcs<T>(&mut self, funcs: FontFuncsRef<'static, T>, data: T)
    where
        T: 'static,
    {
        let funcs = funcs.as_ptr();
        unsafe {
            sys::hb_font_set_funcs(
                self.as_ptr_mut(),
                funcs,
                Box::into_raw(Box::new(data)).cast(),
                Some(dealloc::<T>),
            )
        }
    }
}

pub struct Font(NonNull<sys::hb_font_t>);

impl Font {
    pub fn new(face: FaceRef<'_>) -> Font {
        let ptr = unsafe { sys::hb_font_create(face.as_ptr()) };
        Font(NonNull::new(ptr).unwrap())
    }

    pub fn as_ref(&self) -> FontRef<'_> {
        FontRef(self.0, PhantomData)
    }

    pub fn as_mut(&self) -> FontMut<'_> {
        FontMut(self.as_ref(), PhantomData)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { sys::hb_font_destroy(self.0.as_ptr()) }
    }
}

pub struct ShapePlanRef<'a>(
    NonNull<sys::hb_shape_plan_t>,
    PhantomData<&'a sys::hb_shape_plan_t>,
);

impl ShapePlanRef<'_> {
    fn as_ptr(&self) -> *mut sys::hb_shape_plan_t {
        self.0.as_ptr()
    }

    pub fn get_shaper(&self) -> &CStr {
        let ptr = unsafe { sys::hb_shape_plan_get_shaper(self.as_ptr()) };
        unsafe { CStr::from_ptr(ptr) }
    }
}

pub struct ShapePlanMut<'a>(ShapePlanRef<'a>, PhantomData<&'a mut sys::hb_shape_plan_t>);

impl ShapePlanMut<'_> {
    fn as_ptr_mut(&mut self) -> *mut sys::hb_shape_plan_t {
        ptr::from_mut(self).cast()
    }

    pub fn execute(
        &mut self,
        font: FontRef<'_>,
        mut buffer: BufferMut<'_>,
        features: &[Feature],
    ) -> bool {
        unsafe {
            sys::hb_shape_plan_execute(
                self.as_ptr_mut(),
                font.as_ptr(),
                buffer.as_ptr_mut(),
                features.as_ptr(),
                features.len() as libc::c_uint,
            ) != 0
        }
    }
}

impl<'a> Deref for ShapePlanMut<'a> {
    type Target = ShapePlanRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ShapePlan(NonNull<sys::hb_shape_plan_t>);

impl ShapePlan {
    pub fn new(
        face: FaceRef<'_>,
        props: &SegmentProperties,
        features: &[Feature],
        shaper_list: Option<&[*const libc::c_char]>,
    ) -> ShapePlan {
        if let Some(list) = shaper_list {
            assert!(list.last().unwrap().is_null());
        }
        let ptr = unsafe {
            sys::hb_shape_plan_create(
                face.as_ptr(),
                props,
                features.as_ptr(),
                features.len() as libc::c_uint,
                shaper_list.map(|s| s.as_ptr()).unwrap_or(ptr::null_mut()),
            )
        };
        ShapePlan(NonNull::new(ptr).unwrap())
    }

    pub fn new_cached(
        face: FaceRef<'_>,
        props: &SegmentProperties,
        features: &[Feature],
        shaper_list: Option<&[*const libc::c_char]>,
    ) -> ShapePlan {
        if let Some(list) = shaper_list {
            assert!(list.last().unwrap().is_null());
        }
        let ptr = unsafe {
            sys::hb_shape_plan_create_cached(
                face.as_ptr(),
                props,
                features.as_ptr(),
                features.len() as libc::c_uint,
                shaper_list.map(|s| s.as_ptr()).unwrap_or(ptr::null_mut()),
            )
        };
        ShapePlan(NonNull::new(ptr).unwrap())
    }

    pub fn as_ref(&self) -> ShapePlanRef<'_> {
        ShapePlanRef(self.0, PhantomData)
    }

    pub fn as_mut(&self) -> ShapePlanMut<'_> {
        ShapePlanMut(self.as_ref(), PhantomData)
    }
}

impl Drop for ShapePlan {
    fn drop(&mut self) {
        unsafe { sys::hb_shape_plan_destroy(self.0.as_ptr()) }
    }
}

#[test]
fn linkage() {}
