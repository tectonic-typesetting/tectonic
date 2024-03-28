// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the FreeType2 *C* API into the Cargo framework, as well as provide
//! bindings to other tectonic crates.

#![allow(clippy::unnecessary_cast)]

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use std::ops::BitOr;
use std::ptr::NonNull;
use std::sync::OnceLock;
use std::{ptr, slice};

mod sys;

pub use sys::FT_BBox as BBox;
pub use sys::FT_Fixed as Fixed;
pub use sys::FT_Glyph_BBox_Mode as BBoxMode;
pub use sys::FT_Glyph_Format as GlyphFormat;
pub use sys::FT_Glyph_Metrics as GlyphMetrics;
pub use sys::FT_Kerning_Mode as KerningMode;
pub use sys::FT_Outline as Outline;
pub use sys::FT_Sfnt_Tag as SfntTag;
pub use sys::FT_Vector as Vector;

pub mod tables {
    pub use crate::sys::TT_Header as Header;
    pub use crate::sys::TT_Postscript as Postscript;
    pub use crate::sys::TT_OS2 as OS2;
}

mod sealed {
    use std::mem::MaybeUninit;

    pub trait Sealed {}
    impl Sealed for u8 {}
    impl Sealed for MaybeUninit<u8> {}
}

pub trait ByteLike: sealed::Sealed {}

impl ByteLike for u8 {}
impl ByteLike for MaybeUninit<u8> {}

pub(crate) struct SyncPtr<T>(*mut T);
unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}

static FREE_TYPE_LIBRARY: OnceLock<SyncPtr<sys::FT_LibraryRec>> = OnceLock::new();

fn ft_lib() -> sys::FT_Library {
    FREE_TYPE_LIBRARY
        .get_or_init(|| {
            let mut lib = ptr::null_mut();
            let error = unsafe { sys::FT_Init_FreeType(&mut lib) };
            if error != 0 {
                panic!("FreeType initialization failed, error {}", error);
            }
            SyncPtr(lib)
        })
        .0
}

/// Initialize FreeType. This function will panic if initialization fails.
pub fn init() {
    ft_lib();
}

#[derive(Copy, Clone, Debug)]
pub enum TableTag {
    Sfnt(SfntTag),
    Other(u32),
}

impl TableTag {
    fn as_u32(self) -> u32 {
        match self {
            TableTag::Sfnt(tag) => tag as u32,
            TableTag::Other(other) => other,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LoadFlags(i32);

impl LoadFlags {
    pub const NONE: LoadFlags = LoadFlags(0);
    pub const NO_SCALE: LoadFlags = LoadFlags(sys::FT_LOAD_NO_SCALE);
    pub const VERTICAL_LAYOUT: LoadFlags = LoadFlags(sys::FT_LOAD_VERTICAL_LAYOUT);

    fn as_i32(self) -> i32 {
        self.0
    }
}

impl BitOr for LoadFlags {
    type Output = LoadFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        LoadFlags(self.0 | rhs.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlatformId(u16);

impl PlatformId {
    pub const APPLE_UNICODE: PlatformId = PlatformId(sys::TT_PLATFORM_APPLE_UNICODE as u16);
    pub const MACINTOSH: PlatformId = PlatformId(sys::TT_PLATFORM_MACINTOSH as u16);
    pub const MICROSOFT: PlatformId = PlatformId(sys::TT_PLATFORM_MICROSOFT as u16);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EncodingId(u16);

impl EncodingId {
    pub const MAC_ROMAN: EncodingId = EncodingId(sys::TT_MAC_ID_ROMAN as u16);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LanguageId(u16);

impl LanguageId {
    pub const MAC_ENGLISH: LanguageId = LanguageId(sys::TT_MAC_LANGID_ENGLISH as u16);
}

#[derive(Debug)]
pub struct Error(sys::FT_Error);

impl Error {
    fn or_else<T>(err: sys::FT_Error, f: impl FnOnce() -> T) -> Result<T, Error> {
        if err == 0 {
            Ok(f())
        } else {
            Err(Error(err))
        }
    }

    pub fn code(&self) -> sys::FT_Error {
        self.0
    }
}

#[non_exhaustive]
pub struct SfntName<'a> {
    pub platform_id: PlatformId,
    pub encoding_id: EncodingId,
    pub language_id: LanguageId,
    pub name_id: u16,
    pub string: &'a [u8],
}

impl<'a> From<sys::FT_SfntName> for SfntName<'a> {
    fn from(value: sys::FT_SfntName) -> Self {
        SfntName {
            platform_id: PlatformId(value.platform_id as u16),
            encoding_id: EncodingId(value.encoding_id as u16),
            language_id: LanguageId(value.language_id as u16),
            name_id: value.name_id as u16,
            string: if !value.string.is_null() {
                unsafe { slice::from_raw_parts(value.string, value.string_len as usize) }
            } else {
                &[]
            },
        }
    }
}

pub struct Glyph(NonNull<sys::FT_GlyphRec>);

impl Glyph {
    pub fn get_cbox(&self, mode: BBoxMode) -> BBox {
        let mut ft_bbox = BBox::default();
        unsafe { sys::FT_Glyph_Get_CBox(self.0.as_ptr(), mode, &mut ft_bbox) };
        ft_bbox
    }
}

impl Drop for Glyph {
    fn drop(&mut self) {
        unsafe { sys::FT_Done_Glyph(self.0.as_ptr()) }
    }
}

pub struct GlyphSlot<'a>(&'a sys::FT_GlyphSlotRec);

impl GlyphSlot<'_> {
    pub fn get_glyph(&self) -> Result<Glyph, Error> {
        let mut ptr = ptr::null_mut();
        let err = unsafe { sys::FT_Get_Glyph(ptr::from_ref(self.0).cast_mut(), &mut ptr) };
        Error::or_else(err, || Glyph(NonNull::new(ptr).unwrap()))
    }

    pub fn metrics(&self) -> &GlyphMetrics {
        &self.0.metrics
    }

    pub fn format(&self) -> GlyphFormat {
        self.0.format
    }

    pub fn outline(&self) -> &Outline {
        &self.0.outline
    }
}

pub struct Face(NonNull<sys::FT_FaceRec>, Vec<Vec<u8>>);

impl Face {
    pub fn new(path: &CStr, index: usize) -> Result<Face, Error> {
        let mut raw_face = ptr::null_mut();
        let err = unsafe {
            sys::FT_New_Face(
                ft_lib(),
                path.as_ptr(),
                index as libc::c_long,
                &mut raw_face,
            )
        };
        Error::or_else(err, || Face(NonNull::new(raw_face).unwrap(), Vec::new()))
    }

    pub fn new_memory(mut data: Vec<u8>, index: usize) -> Result<Face, Error> {
        let mut raw_face = ptr::null_mut();
        let err = unsafe {
            sys::FT_New_Memory_Face(
                ft_lib(),
                data.as_mut_ptr().cast(),
                data.len() as libc::c_long,
                index as libc::c_long,
                &mut raw_face,
            )
        };

        Error::or_else(err, || Face(NonNull::new(raw_face).unwrap(), vec![data]))
    }

    fn inner(&self) -> &sys::FT_FaceRec {
        unsafe { self.0.as_ref() }
    }

    pub fn is_scalable(&self) -> bool {
        unsafe { sys::FT_IS_SCALABLE(self.0.as_ptr()) }
    }

    pub fn is_sfnt(&self) -> bool {
        unsafe { sys::FT_IS_SFNT(self.0.as_ptr()) }
    }

    pub fn has_glyph_names(&self) -> bool {
        unsafe { sys::FT_HAS_GLYPH_NAMES(self.0.as_ptr()) }
    }

    pub fn units_per_em(&self) -> u16 {
        self.inner().units_per_EM as u16
    }

    pub fn ascender(&self) -> i16 {
        self.inner().ascender as i16
    }

    pub fn descender(&self) -> i16 {
        self.inner().descender as i16
    }

    pub fn num_glyphs(&self) -> usize {
        self.inner().num_glyphs as usize
    }

    pub fn num_faces(&self) -> usize {
        self.inner().num_faces as usize
    }

    pub fn glyph(&self) -> GlyphSlot<'_> {
        GlyphSlot(unsafe { &*self.inner().glyph })
    }

    pub fn get_advance(&self, index: u32, flags: LoadFlags) -> Result<i64, Error> {
        let mut advance = 0;
        let err = unsafe {
            sys::FT_Get_Advance(
                self.0.as_ptr(),
                index as libc::c_uint,
                flags.as_i32(),
                &mut advance,
            )
        };
        Error::or_else(err, || advance as i64)
    }

    pub fn get_first_char(&self) -> (u32, u32) {
        let mut index = 0;
        let code = unsafe { sys::FT_Get_First_Char(self.0.as_ptr(), &mut index) };
        (code as u32, index as u32)
    }

    pub fn get_next_char(&self, char: u32) -> (u32, u32) {
        let mut index = 0;
        let code =
            unsafe { sys::FT_Get_Next_Char(self.0.as_ptr(), char as libc::c_ulong, &mut index) };
        (code as u32, index as u32)
    }

    pub fn get_char_index(&self, char: u32) -> Option<NonZeroU32> {
        NonZeroU32::new(unsafe { sys::FT_Get_Char_Index(self.0.as_ptr(), char as libc::c_ulong) })
    }

    pub fn get_char_variant_index(&self, char: u32, variant: u32) -> Option<NonZeroU32> {
        NonZeroU32::new(unsafe {
            sys::FT_Face_GetCharVariantIndex(
                self.0.as_ptr(),
                char as libc::c_ulong,
                variant as libc::c_ulong,
            )
        })
    }

    pub fn get_kerning(&self, left: u32, right: u32, mode: KerningMode) -> Result<Vector, Error> {
        let mut vector = Vector::default();
        let err = unsafe { sys::FT_Get_Kerning(self.0.as_ptr(), left, right, mode, &mut vector) };
        Error::or_else(err, || vector)
    }

    pub fn get_glyph_name<'a, T: ByteLike>(
        &self,
        index: u32,
        buf: &'a mut [T],
    ) -> Result<&'a CStr, Error> {
        let err = unsafe {
            sys::FT_Get_Glyph_Name(
                self.0.as_ptr(),
                index as libc::c_uint,
                buf.as_mut_ptr().cast(),
                buf.len() as libc::c_uint,
            )
        };
        Error::or_else(err, || unsafe { CStr::from_ptr(buf.as_ptr().cast()) })
    }

    pub fn get_name_index(&self, name: &CStr) -> Option<NonZeroU32> {
        NonZeroU32::new(unsafe { sys::FT_Get_Name_Index(self.0.as_ptr(), name.as_ptr()) as u32 })
    }

    pub fn get_postscript_name(&self) -> Option<&CStr> {
        unsafe {
            sys::FT_Get_Postscript_Name(self.0.as_ptr())
                .as_ref()
                .map(|ptr| CStr::from_ptr(ptr))
        }
    }

    pub fn get_sfnt_name_count(&self) -> u32 {
        unsafe { sys::FT_Get_Sfnt_Name_Count(self.0.as_ptr()) as u32 }
    }

    pub fn get_sfnt_name(&self, idx: u32) -> Result<SfntName, Error> {
        let mut name = sys::FT_SfntName::default();
        let err = unsafe { sys::FT_Get_Sfnt_Name(self.0.as_ptr(), idx as libc::c_uint, &mut name) };
        Error::or_else(err, || SfntName::from(name))
    }

    pub fn load_glyph(&mut self, index: u32, flags: LoadFlags) -> Result<(), Error> {
        let err =
            unsafe { sys::FT_Load_Glyph(self.0.as_ptr(), index as libc::c_uint, flags.as_i32()) };
        Error::or_else(err, || ())
    }

    pub fn get_sfnt_table(&self, tag: SfntTag) -> Option<NonNull<()>> {
        NonNull::new(unsafe { sys::FT_Get_Sfnt_Table(self.0.as_ptr(), tag) })
    }

    pub fn load_sfnt_table(&self, tag: TableTag) -> Result<Vec<u8>, Error> {
        let mut len = 0;
        let err = unsafe {
            sys::FT_Load_Sfnt_Table(
                self.0.as_ptr(),
                tag.as_u32() as libc::c_ulong,
                0,
                ptr::null_mut(),
                &mut len,
            )
        };
        Error::or_else(err, || ())?;

        let mut buf = vec![0u8; len as usize];

        let err = unsafe {
            sys::FT_Load_Sfnt_Table(
                self.0.as_ptr(),
                tag.as_u32() as libc::c_ulong,
                0,
                buf.as_mut_ptr().cast(),
                &mut len,
            )
        };

        Error::or_else(err, || buf)
    }

    pub fn attach_stream_mem(&mut self, mut stream: Vec<u8>) -> Result<(), Error> {
        let mut oa = sys::FT_Open_Args::default();
        oa.flags |= sys::FT_OPEN_MEMORY;
        oa.memory_base = stream.as_mut_ptr().cast();
        oa.memory_size = stream.len() as libc::c_long;
        self.1.push(stream);
        let err = unsafe { sys::FT_Attach_Stream(self.0.as_ptr(), &mut oa) };
        Error::or_else(err, || ())
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        unsafe { sys::FT_Done_Face(self.0.as_ptr()) };
    }
}
