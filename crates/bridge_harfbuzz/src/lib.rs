// Copyright 2020-2023 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the Harfbuzz *C/C++* API into the Cargo framework, as well as
//! provide bindings to other tectonic crates.

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::{ptr, slice};
use tectonic_bridge_graphite2 as gr;

mod font_funcs;
pub mod ot;
#[doc(hidden)]
pub mod sys;
#[cfg(test)]
mod test_util;

pub use font_funcs::{FontFuncs, FontFuncsMut, FontFuncsRef, ImmutFontFuncs};
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
    use tectonic_bridge_freetype2 as clippyrenamehack1;
    #[allow(unused_imports)]
    #[cfg(target_os = "macos")]
    use tectonic_mac_core as clippyrenamehack2;
}

unsafe extern "C" fn dealloc<T: 'static>(user_data: *mut ()) {
    // SAFETY: Soundness precondition - this is only called on pointers created by `Box::into_raw`
    let _ = unsafe { Box::from_raw(user_data.cast::<T>()) };
}

/// Data type representing different script values
#[repr(transparent)]
pub struct Script(sys::hb_script_t);

impl Script {
    /// The invalid script
    pub const INVALID: Script = Script(sys::HB_SCRIPT_INVALID);

    /// Get the text layout direction of this script
    pub fn get_horizontal_direction(&self) -> Direction {
        // SAFETY: This is always safe to call
        unsafe { sys::hb_script_get_horizontal_direction(self.0) }
    }
}

/// Tag identifiers - used to identify tables, scripts, etc.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Tag(sys::hb_tag_t);

impl Tag {
    /// Create a new tag from a raw 4-byte integer
    pub fn new(val: u32) -> Tag {
        Tag(val)
    }

    /// Create a new tag from a string. Strings shorter than 4 characters will be padded with
    /// spaces, those longer than 4 characters will be truncated.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(val: &str) -> Tag {
        // SAFETY: The provided string is not used past this call, and not read past len
        Tag(unsafe { sys::hb_tag_from_string(val.as_ptr().cast(), val.len() as libc::c_int) })
    }

    /// Same as [`Self::from_str`], but accepts a C-string
    pub fn from_cstr(val: &CStr) -> Tag {
        // SAFETY: The provided string is not used past this call, and not read past the terminating
        //         null
        Tag(unsafe { sys::hb_tag_from_string(val.as_ptr(), -1) })
    }

    /// Convert this tag into a 4-byte integer
    pub fn to_raw(self) -> u32 {
        self.0
    }

    /// Convert this tag into a [`Script`]
    pub fn to_script(self) -> Script {
        // SAFETY: This is always safe to call
        Script(unsafe { sys::hb_ot_tag_to_script(self.0) })
    }

    /// Convert this tag into a [`Language`]
    pub fn to_language(self) -> Language {
        // SAFETY: This is always safe to call
        Language(unsafe { sys::hb_ot_tag_to_language(self.0) })
    }
}

/// Data type representing BCP 47 language tags
#[derive(Copy, Clone)]
pub struct Language(*mut sys::hb_language_impl_t);

impl Language {
    /// Convert the provided string into a language tag
    pub fn from_string(str: &str) -> Language {
        // SAFETY: The provided string is not used past this call, and not read past len
        Language(unsafe {
            sys::hb_language_from_string(str.as_ptr().cast(), str.len() as libc::c_int)
        })
    }

    /// Same as [`Self::from_string`], but accepts a C-string
    pub fn from_cstr(str: &CStr) -> Language {
        // SAFETY: The provided string is not used past this call, and not read past the terminating
        //         null
        Language(unsafe { sys::hb_language_from_string(str.as_ptr(), -1) })
    }

    /// Convert this tag into its string representation
    pub fn to_string(&self) -> Option<&CStr> {
        // SAFETY: Internal pointer guaranteed valid
        let ptr = unsafe { sys::hb_language_to_string(self.0) };
        // ptr may be null if we have HB_LANGUAGE_INVALID
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Pointer returned by hb_language_to_string is a valid C-string if non-null
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        // This gets HB_LANGUAGE_INVALID
        // SAFETY: This function is safe to call with null and will return a marker value
        Language(unsafe { sys::hb_language_from_string(ptr::null(), -1) })
    }
}

/// A borrowed reference to a [`Buffer`]
#[derive(Copy, Clone)]
pub struct BufferRef<'a>(NonNull<sys::hb_buffer_t>, PhantomData<&'a sys::hb_buffer_t>);

impl<'a> BufferRef<'a> {
    fn as_ptr(self) -> *mut sys::hb_buffer_t {
        self.0.as_ptr()
    }

    /// Get the number of items in this buffer
    pub fn len(self) -> usize {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_buffer_get_length(self.as_ptr()) as usize }
    }

    /// Whether this buffer is empty
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// Get a slice of glyph information. Will return `None` if no information as present, such
    /// as before this buffer is shaped via a call to [`ShapePlanMut::execute`].
    pub fn glyph_info(self) -> Option<&'a [GlyphInfo]> {
        let mut len = 0;
        // SAFETY: Internal pointer guaranteed valid
        let ptr = unsafe { sys::hb_buffer_get_glyph_infos(self.as_ptr(), &mut len) };
        // FIXME(CraftSpider): This isn't fully sound unless we never allow `hb_buffer_reference` -
        //       currently it's fine, but we may need to either bite the cost of cloning, force
        //       refcounting to be on the Rust side, or do something... weird.
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Returned pointer is to an array of length `len`, and is valid as long as contents
            //         of the buffer aren't modified
            Some(unsafe { slice::from_raw_parts(ptr, len as usize) })
        }
    }

    /// Get a slice of glyph positions. Will return `None` if no information as present, such
    /// as before this buffer is shaped via a call to [`ShapePlanMut::execute`].
    pub fn glyph_positions(self) -> Option<&'a [GlyphPosition]> {
        let mut len = 0;
        // SAFETY: Internal pointer guaranteed valid
        let ptr = unsafe { sys::hb_buffer_get_glyph_positions(self.as_ptr(), &mut len) };
        // FIXME(CraftSpider): See get_glyph_info
        if ptr.is_null() {
            None
        } else {
            // SAFETY: Returned pointer is to an array of length `len`, and is valid as long as contents
            //         of the buffer aren't modified
            Some(unsafe { slice::from_raw_parts(ptr, len as usize) })
        }
    }

    /// Get the script of this buffer
    pub fn get_script(self) -> Script {
        // SAFETY: Internal pointer guaranteed valid
        Script(unsafe { sys::hb_buffer_get_script(self.as_ptr()) })
    }

    /// Get the segment properties of this buffer
    pub fn get_segment_properties(self) -> SegmentProperties {
        let mut props = SegmentProperties::default();
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_buffer_get_segment_properties(self.as_ptr(), &mut props) };
        props
    }
}

/// A borrowed mutable reference to a [`Buffer`]
pub struct BufferMut<'a>(BufferRef<'a>, PhantomData<&'a mut sys::hb_buffer_t>);

impl BufferMut<'_> {
    fn as_ptr_mut(&mut self) -> *mut sys::hb_buffer_t {
        self.0.as_ptr()
    }

    /// Set the content type of this buffer. This should rarely need called, as most methods that
    /// mutate a buffer transition the content type themselves.
    pub fn set_content_type(&mut self, content: BufferContentType) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_buffer_set_content_type(self.as_ptr_mut(), content) }
    }

    /// Set the text direction for this buffer.
    pub fn set_direction(&mut self, direction: Direction) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_buffer_set_direction(self.as_ptr_mut(), direction) }
    }

    /// Set the language for this buffer.
    pub fn set_language(&mut self, lang: Language) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_buffer_set_language(self.as_ptr_mut(), lang.0) }
    }

    /// Set the script for this buffer.
    pub fn set_script(&mut self, script: Script) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_buffer_set_script(self.as_ptr_mut(), script.0) }
    }

    /// Add UTF-16 codepoints to the contents of the buffer.
    pub fn add_utf16(&mut self, text: &[u16]) {
        // SAFETY: Internal pointer guaranteed valid. Provided text isn't held past this call.
        unsafe {
            sys::hb_buffer_add_utf16(
                self.as_ptr_mut(),
                text.as_ptr(),
                text.len() as libc::c_int,
                0,
                text.len() as libc::c_int,
            )
        }
    }

    /// Set properties that haven't been set manually based on the current contents of the buffer.
    pub fn guess_segment_properties(&mut self) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_buffer_guess_segment_properties(self.as_ptr_mut()) }
    }

    /// Reset the buffer to a freshly created status. More efficient than calling [`Buffer::new`]
    /// repeatedly.
    pub fn reset(&mut self) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_buffer_reset(self.as_ptr_mut()) }
    }
}

impl<'a> Deref for BufferMut<'a> {
    type Target = BufferRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Container for text and associated properties before shaping, then glyphs and associated
/// information after shaping.
pub struct Buffer(NonNull<sys::hb_buffer_t>);

impl Buffer {
    /// Create a new buffer to use when shaping text
    pub fn new() -> Buffer {
        // SAFETY: This is always safe to call
        let ptr = unsafe { sys::hb_buffer_create() };
        Buffer(NonNull::new(ptr).unwrap())
    }

    /// Convert into a shared reference
    pub fn as_ref(&self) -> BufferRef<'_> {
        BufferRef(self.0, PhantomData)
    }

    /// Convert into a mutable reference
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
        // SAFETY: Internal pointer guaranteed valid, we own the pointer.
        unsafe { sys::hb_buffer_destroy(self.0.as_ptr()) }
    }
}

/// Blob of binary data. Facilitates interaction of program blobs (such as static memory or [`Vec`])
/// with the Harfbuzz library's lifecycle management.
pub struct Blob(NonNull<sys::hb_blob_t>);

impl Blob {
    /// Create a new blob from a [`Vec`] of bytes
    pub fn new(data: Vec<u8>) -> Blob {
        unsafe extern "C" fn blob_dealloc(ptr: *mut ()) {
            let slice = Box::from_raw(ptr.cast::<(*mut (), usize)>());
            let _ = Box::from_raw(ptr::slice_from_raw_parts_mut(slice.0.cast::<u8>(), slice.1));
        }

        let len = data.len();
        let data = Box::into_raw(data.into_boxed_slice());
        let slice_data = Box::into_raw(Box::new((data.cast::<u8>(), len)));
        // SAFETY: The provided pointer is never referenced after being creating from Box::into_raw.
        //         The pointer will live as long as the blob, and be deallocated by blob_dealloc
        //         once the blob is destroyed.
        let raw = unsafe {
            sys::hb_blob_create(
                data.cast(),
                len as libc::c_uint,
                MemoryMode::Writable,
                slice_data.cast(),
                Some(blob_dealloc),
            )
        };

        Blob(NonNull::new(raw).unwrap())
    }
}

/// Valid tags for use in OpenType layout-related methods
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GTag {
    /// GPOS table, Glyph Positioning
    GPos,
    /// GSUB table, Glyph Substitutions
    GSub,
}

impl GTag {
    fn as_tag(self) -> Tag {
        Tag::new(match self {
            GTag::GPos => sys::HB_OT_TAG_GPOS,
            GTag::GSub => sys::HB_OT_TAG_GSUB,
        })
    }
}

// TODO: OTLayout/OTTable/OTScript/OTLanguage

/// A borrowed reference to a [`Face`]
#[derive(Copy, Clone)]
pub struct FaceRef<'a>(NonNull<sys::hb_face_t>, PhantomData<&'a sys::hb_face_t>);

impl<'a> FaceRef<'a> {
    unsafe fn from_raw(ptr: NonNull<sys::hb_face_t>) -> FaceRef<'a> {
        FaceRef(ptr, PhantomData)
    }

    fn as_ptr(self) -> *mut sys::hb_face_t {
        self.0.as_ptr()
    }

    /// Check whether this font has an OpenType math table
    pub fn has_ot_math_data(self) -> bool {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_ot_math_has_data(self.as_ptr()) != 0 }
    }

    /// Helper for retrieving OpenType layout information for this face
    pub fn ot_layout(self) -> ot::Layout<'a> {
        ot::Layout(self)
    }

    /// Get the graphite2 face for this font
    pub fn gr_face(self) -> Option<gr::FaceRef<'a>> {
        // SAFETY: Internal pointer guaranteed valid
        let ptr = unsafe { sys::hb_graphite2_face_get_gr_face(self.as_ptr()) };
        // SAFETY: If non-null, returned pointer is a valid (non-retained) graphite face reference.
        NonNull::new(ptr).map(|ptr| unsafe { gr::FaceRef::from_raw(ptr) })
    }
}

/// A borrowed mutable reference to a [`Face`]
pub struct FaceMut<'a>(FaceRef<'a>, PhantomData<&'a mut sys::hb_face_t>);

impl FaceMut<'_> {
    fn as_ptr_mut(&mut self) -> *mut sys::hb_face_t {
        self.0.as_ptr()
    }

    /// Set the index of this face. This has no effect on the face directly.
    pub fn set_index(&mut self, index: u32) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_face_set_index(self.as_ptr_mut(), index as libc::c_uint) }
    }

    /// Set the units-per-em for this face
    pub fn set_upem(&mut self, upem: u32) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_face_set_upem(self.as_ptr_mut(), upem as libc::c_uint) }
    }
}

/// A typographic face - a typeface combined with a style, loaded from a binary blob. Not yet
/// associated with a size or variation. See [`Font`].
pub struct Face(NonNull<sys::hb_face_t>);

impl Face {
    /// Create a new [`Face`], where it is easier to provide individual tables rather than a blob
    /// of all font data. Retrieving all table won't work for faces created this way by default.
    pub fn new_tables<T: Fn(FaceRef<'_>, Tag) -> Option<Blob> + 'static>(f: T) -> Face {
        unsafe extern "C" fn get_table<T: Fn(FaceRef<'_>, Tag) -> Option<Blob> + 'static>(
            face: *mut sys::hb_face_t,
            tag: sys::hb_tag_t,
            user_data: *mut (),
        ) -> *mut sys::hb_blob_t {
            // SAFETY: Precondition of this function - it is only called with a T user-data
            let f = unsafe { &*user_data.cast::<T>() };
            let face = NonNull::new(face).unwrap();
            // SAFETY: Harfbuzz guarantees to provide a valid non-retained face reference when this
            //         is called.
            let face = unsafe { FaceRef::from_raw(face) };
            match f(face, Tag(tag)) {
                Some(blob) => blob.0.as_ptr(),
                None => ptr::null_mut(),
            }
        }

        // SAFETY: The created face will only call get_table in valid ways. The passed closure will
        //         be deallocated as its same type by `dealloc` when the face is destroyed, the
        //         static bound ensures it may live however long that is.
        let face = unsafe {
            sys::hb_face_create_for_tables(
                get_table::<T>,
                Box::into_raw(Box::new(f)).cast(),
                Some(dealloc::<T>),
            )
        };

        Face(NonNull::new(face).unwrap())
    }

    /// Convert into a shared reference
    pub fn as_ref(&self) -> FaceRef<'_> {
        FaceRef(self.0, PhantomData)
    }

    /// Convert into a mutable reference
    pub fn as_mut(&mut self) -> FaceMut<'_> {
        FaceMut(self.as_ref(), PhantomData)
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        // SAFETY: Internal pointer guaranteed valid, we own the pointer.
        unsafe { sys::hb_face_destroy(self.0.as_ptr()) }
    }
}

/// A borrowed reference to a [`Font`]
#[derive(Copy, Clone)]
pub struct FontRef<'a>(NonNull<sys::hb_font_t>, PhantomData<&'a sys::hb_font_t>);

impl<'a> FontRef<'a> {
    unsafe fn from_raw(ptr: NonNull<sys::hb_font_t>) -> FontRef<'a> {
        FontRef(ptr, PhantomData)
    }

    #[doc(hidden)]
    pub fn as_ptr(self) -> *mut sys::hb_font_t {
        self.0.as_ptr()
    }

    /// Get the [`Face`] for this font.
    pub fn face(self) -> FaceRef<'a> {
        // SAFETY: Internal pointer guaranteed valid
        let ptr = unsafe { sys::hb_font_get_face(self.as_ptr()) };
        FaceRef(NonNull::new(ptr).unwrap(), PhantomData)
    }

    /// Get the point size of this font
    pub fn ptem(self) -> f32 {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_font_get_ptem(self.as_ptr()) }
    }
}

/// A borrowed mutable reference to a [`Font`]
pub struct FontMut<'a>(FontRef<'a>, PhantomData<&'a mut sys::hb_font_t>);

impl FontMut<'_> {
    fn as_ptr_mut(&mut self) -> *mut sys::hb_font_t {
        self.0.as_ptr()
    }

    /// Set the horizontal and vertical scale of this font
    pub fn set_scale(&mut self, x: i32, y: i32) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_font_set_scale(self.as_ptr_mut(), x as libc::c_int, y as libc::c_int) }
    }

    /// Set the pixels-per-em of this font
    pub fn set_ppem(&mut self, x: u32, y: u32) {
        // SAFETY: Internal pointer guaranteed valid
        unsafe { sys::hb_font_set_ppem(self.as_ptr_mut(), x as libc::c_uint, y as libc::c_uint) }
    }

    /// Set the font functions associated with this font
    pub fn set_funcs<T>(&mut self, funcs: FontFuncsRef<'static, T>, data: T)
    where
        T: 'static,
    {
        let funcs = funcs.as_ptr();
        // SAFETY: Internal pointer guaranteed valid. FontFuncs implementation upholds the relevant
        //         invariants for ensuring all functions is contains will be valid. Data will be
        //         deallocated as its same type by `dealloc` when the font is destroyed, or when a
        //         new set of functions is set. The static bound ensures it may live that long.
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

/// A [`Face`] that has been associated with a specific size and, possibly, variation. Ready for
/// use in shaping.
pub struct Font(NonNull<sys::hb_font_t>);

impl Font {
    /// Create a new font from the given [`Face`].
    pub fn new(face: FaceRef<'_>) -> Font {
        // SAFETY: The pointer from FaceRef is guaranteed valid
        let ptr = unsafe { sys::hb_font_create(face.as_ptr()) };
        Font(NonNull::new(ptr).unwrap())
    }

    /// Convert into a shared reference
    pub fn as_ref(&self) -> FontRef<'_> {
        FontRef(self.0, PhantomData)
    }

    /// Convert into a mutable reference
    pub fn as_mut(&mut self) -> FontMut<'_> {
        FontMut(self.as_ref(), PhantomData)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        // SAFETY: Internal pointer guaranteed valid, we own the pointer.
        unsafe { sys::hb_font_destroy(self.0.as_ptr()) }
    }
}

/// A borrowed reference to a [`ShapePlan`]
pub struct ShapePlanRef<'a>(
    NonNull<sys::hb_shape_plan_t>,
    PhantomData<&'a sys::hb_shape_plan_t>,
);

impl ShapePlanRef<'_> {
    fn as_ptr(&self) -> *mut sys::hb_shape_plan_t {
        self.0.as_ptr()
    }

    /// Get the shaper that will be used for this [`ShapePlan`]. May be `None` if no shaper was
    /// provided and no default could be found, in which case attempting to shape text will likely
    /// fail.
    pub fn get_shaper(&self) -> Option<&CStr> {
        // SAFETY: Internal pointer guaranteed valid
        let ptr = unsafe { sys::hb_shape_plan_get_shaper(self.as_ptr()) };
        // FIXME(CraftSpider): See BufferRef::get_glyph_info
        if ptr.is_null() {
            None
        } else {
            // SAFETY: The returned pointer is guaranteed valid for as long as ShapePlan isn't updated.
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }
}

/// A borrowed mutable reference to a [`ShapePlan`]
pub struct ShapePlanMut<'a>(ShapePlanRef<'a>, PhantomData<&'a mut sys::hb_shape_plan_t>);

impl ShapePlanMut<'_> {
    fn as_ptr_mut(&mut self) -> *mut sys::hb_shape_plan_t {
        self.0.as_ptr()
    }

    /// Shape a buffer using the provided [`Font`] and [`Feature`]s
    pub fn execute(
        &mut self,
        font: FontRef<'_>,
        mut buffer: BufferMut<'_>,
        features: &[Feature],
    ) -> bool {
        // SAFETY: Internal pointer guaranteed valid. The pointers from font and buffer are
        //         similarly guaranteed valid. Features will not be read past len and will not be
        //         used after this call.
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

/// A shaping plan, information about how Harfbuzz will shape a text segment based on the segment's
/// properties.
pub struct ShapePlan(NonNull<sys::hb_shape_plan_t>);

impl ShapePlan {
    /// Create a new shaping plan for a given [`Face`], [`Feature`]s, and [`SegmentProperties`].
    ///
    /// The shaper list will be the list of shapers to try. If `None`, then Harfbuzz will attempt
    /// to locate a default shaper to use.
    pub fn new(
        face: FaceRef<'_>,
        props: &SegmentProperties,
        features: &[Feature],
        shaper_list: Option<&[*const libc::c_char]>,
    ) -> ShapePlan {
        if let Some(list) = shaper_list {
            assert!(list.last().unwrap().is_null());
        }
        // SAFETY: Face and features pointers are guaranteed valid. The features will not be read
        //         past their provided length. shaper_list is guaranteed to end with a null if Some
        //         by above assert.
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

    /// Create a new shape plan for a given [`Face`], [`Feature`]s, and [`SegmentProperties`]. This
    /// shape plan is cached by Harfbuzz, which can make creation faster if the same arguments
    /// are expected to be used frequently.
    pub fn new_cached(
        face: FaceRef<'_>,
        props: &SegmentProperties,
        features: &[Feature],
        shaper_list: Option<&[*const libc::c_char]>,
    ) -> ShapePlan {
        if let Some(list) = shaper_list {
            assert!(list.last().unwrap().is_null());
        }
        // SAFETY: Face and features pointers are guaranteed valid. The features will not be read
        //         past their provided length. shaper_list is guaranteed to end with a null if Some
        //         by above assert.
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

    /// Convert into a shared reference
    pub fn as_ref(&self) -> ShapePlanRef<'_> {
        ShapePlanRef(self.0, PhantomData)
    }

    /// Convert into a mutable reference
    pub fn as_mut(&mut self) -> ShapePlanMut<'_> {
        ShapePlanMut(self.as_ref(), PhantomData)
    }
}

impl Drop for ShapePlan {
    fn drop(&mut self) {
        // SAFETY: Internal pointer guaranteed valid, we own the pointer.
        unsafe { sys::hb_shape_plan_destroy(self.0.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{get_font_funcs, test_faces};

    #[test]
    fn test_tag() {
        let raw = u32::from_be_bytes([b't', b'e', b's', b't']);

        assert_eq!(Tag::new(raw).to_raw(), raw);
        assert_eq!(Tag::from_str("test"), Tag::new(raw));
        assert_eq!(Tag::from_cstr(c"test"), Tag::new(raw));
    }

    #[test]
    fn test_language() {
        assert_eq!(Language::from_string("en-US").to_string(), Some(c"en-us"));
        assert_eq!(Language::from_cstr(c"en-gb").to_string(), Some(c"en-gb"));
        assert!(Language::default().to_string().is_none());
    }

    #[test]
    fn test_face_ot_math_data() {
        for (_, face) in test_faces() {
            assert!(!face.as_ref().has_ot_math_data())
        }
    }

    #[test]
    fn test_face_ot_layout() {
        for (_, face) in test_faces() {
            let layout = face.as_ref().ot_layout();

            let size = layout.size_params().unwrap();
            assert_eq!(size.design_size, 120);
            assert_eq!(size.start, 110);
            assert_eq!(size.end, 140);
            assert_eq!(size.subfamily_id, 1);
            assert_eq!(size.subfamily_name_id, 256);
        }
    }

    #[test]
    fn test_font() {
        for (_, face) in test_faces() {
            let font = Font::new(face.as_ref());

            assert_eq!(font.as_ref().ptem(), 0.0);
            assert_eq!(font.as_ref().face().0, face.0);
        }
    }

    #[test]
    fn test_shape() {
        for (ft_face, face) in test_faces() {
            let mut font = Font::new(face.as_ref());
            font.as_mut().set_funcs(get_font_funcs(), ft_face);

            let mut buffer = Buffer::new();
            buffer
                .as_mut()
                .add_utf16(&"Hello World!".encode_utf16().collect::<Vec<_>>());
            buffer.as_mut().guess_segment_properties();

            let plan1 = ShapePlan::new(
                face.as_ref(),
                &buffer.as_ref().get_segment_properties(),
                &[],
                None,
            );
            let plan2 = ShapePlan::new_cached(
                face.as_ref(),
                &buffer.as_ref().get_segment_properties(),
                &[],
                None,
            );

            for mut plan in [plan1, plan2] {
                assert_eq!(plan.as_ref().get_shaper(), Some(c"ot"));

                assert!(plan.as_mut().execute(font.as_ref(), buffer.as_mut(), &[]));

                assert_eq!(
                    buffer.as_ref().get_script().get_horizontal_direction(),
                    Direction::Ltr
                );

                let glyph_info = buffer.as_ref().glyph_info().unwrap();
                assert_eq!(glyph_info.len(), 12);
                assert_eq!(glyph_info[0].cluster, 0);
                assert_eq!(glyph_info[0].codepoint, 62);

                assert_eq!(glyph_info[1].cluster, 1);
                assert_eq!(glyph_info[1].codepoint, 50);

                let glyph_pos = buffer.as_ref().glyph_positions().unwrap();
                assert_eq!(glyph_pos.len(), 12);
                assert_eq!(glyph_pos[0].x_advance, 734);
                assert_eq!(glyph_pos[0].y_advance, 0);
                assert_eq!(glyph_pos[0].x_offset, 0);
                assert_eq!(glyph_pos[0].y_offset, 0);

                assert_eq!(glyph_pos[1].x_advance, 435);
                assert_eq!(glyph_pos[1].y_advance, 0);
                assert_eq!(glyph_pos[1].x_offset, 0);
                assert_eq!(glyph_pos[1].y_offset, 0);

                buffer.as_mut().reset();
                buffer
                    .as_mut()
                    .add_utf16(&"Hello World!".encode_utf16().collect::<Vec<_>>());
                buffer.as_mut().guess_segment_properties();
            }
        }
    }
}
