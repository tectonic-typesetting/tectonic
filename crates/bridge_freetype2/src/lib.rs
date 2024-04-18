// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the `FreeType2` *C* API into the Cargo framework, as well as provide
//! bindings to other tectonic crates.

#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::unnecessary_cast)]

use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use std::ops::BitOr;
use std::ptr::NonNull;
use std::sync::{Mutex, OnceLock};
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

// SAFETY: It is on the user of `SyncPtr` to only use the pointer in a Send-safe way
unsafe impl<T> Send for SyncPtr<T> {}
// SAFETY: It is on the user of `SyncPtr` to only use the pointer in a Sync-safe way
unsafe impl<T> Sync for SyncPtr<T> {}

// FT_Library can be used from many threads as long as a Mutex protects FT_New_Face and FT_Done_Face
// - https://sourceforge.net/projects/freetype/files/freetype2/2.6/
static FREE_TYPE_LIBRARY: OnceLock<SyncPtr<sys::FT_LibraryRec>> = OnceLock::new();
static FACE_MUTEX: Mutex<()> = Mutex::new(());

fn ft_lib() -> sys::FT_Library {
    FREE_TYPE_LIBRARY
        .get_or_init(|| {
            let mut lib = ptr::null_mut();
            // SAFETY: FreeType initialization is always sound
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
                // SAFETY: If non-null, the value in `string` is guaranteed to be allocated and
                //         initialized to a length of `string_len`.
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
        // SAFETY: Our internal pointer is guaranteed valid
        unsafe { sys::FT_Glyph_Get_CBox(self.0.as_ptr(), mode, &mut ft_bbox) };
        ft_bbox
    }
}

impl Drop for Glyph {
    fn drop(&mut self) {
        // SAFETY: Our internal pointer is guaranteed valid, we own the pointer
        unsafe { sys::FT_Done_Glyph(self.0.as_ptr()) }
    }
}

pub struct GlyphSlot<'a>(
    NonNull<sys::FT_GlyphSlotRec>,
    PhantomData<&'a sys::FT_GlyphSlotRec>,
);

impl GlyphSlot<'_> {
    fn as_ref(&self) -> &sys::FT_GlyphSlotRec {
        // SAFETY: Internal pointer is guaranteed valid
        //         GlyphSlot is !Send+!Sync, so no C code can violate this reference.
        unsafe { self.0.as_ref() }
    }

    pub fn get_glyph(&self) -> Result<Glyph, Error> {
        let mut ptr = ptr::null_mut();
        // SAFETY: Internal pointer is guaranteed valid.
        let err = unsafe { sys::FT_Get_Glyph(self.0.as_ptr(), &mut ptr) };
        Error::or_else(err, || Glyph(NonNull::new(ptr).unwrap()))
    }

    pub fn metrics(&self) -> &GlyphMetrics {
        &self.as_ref().metrics
    }

    pub fn format(&self) -> GlyphFormat {
        self.as_ref().format
    }

    pub fn outline(&self) -> &Outline {
        &self.as_ref().outline
    }
}

/// Trait for marker types that represent different sfnt tables you can read from a [`Face`].
///
/// # Safety
///
/// Code may assume that the `SfntTag` returned by `tag` is valid to read as an instance of `table`.
pub unsafe trait Table {
    type Table;
    fn tag() -> SfntTag;
}

pub struct Os2(());

// SAFETY: SfntTag::Os2 == tables::OS2
unsafe impl Table for Os2 {
    type Table = tables::OS2;

    fn tag() -> SfntTag {
        SfntTag::Os2
    }
}

pub struct Header(());

// SAFETY: SfntTag::Head == tables::Header
unsafe impl Table for Header {
    type Table = tables::Header;

    fn tag() -> SfntTag {
        SfntTag::Head
    }
}

pub struct Postscript(());

// SAFETY: SfntTag::Post == tables::Postscript
unsafe impl Table for Postscript {
    type Table = tables::Postscript;

    fn tag() -> SfntTag {
        SfntTag::Post
    }
}

pub struct Face(NonNull<sys::FT_FaceRec>, Vec<Vec<u8>>);

impl Face {
    pub fn new(path: &CStr, index: usize) -> Result<Face, Error> {
        let _l = FACE_MUTEX.lock();
        let mut raw_face = ptr::null_mut();
        // SAFETY: We hold the FACE_MUTEX lock, so calls to FT_New_Face are guaranteed safe
        //         with our global library instance.
        let err = unsafe {
            sys::FT_New_Face(
                ft_lib(),
                path.as_ptr(),
                index as libc::c_long,
                &mut raw_face,
            )
        };
        drop(_l);
        Error::or_else(err, || Face(NonNull::new(raw_face).unwrap(), Vec::new()))
    }

    pub fn new_memory(mut data: Vec<u8>, index: usize) -> Result<Face, Error> {
        let _l = FACE_MUTEX.lock();
        let mut raw_face = ptr::null_mut();
        // SAFETY: We hold the FACE_MUTEX lock, so calls to FT_New_Memory_Face are guaranteed safe
        //         with our global library instance.
        let err = unsafe {
            sys::FT_New_Memory_Face(
                ft_lib(),
                data.as_mut_ptr().cast(),
                data.len() as libc::c_long,
                index as libc::c_long,
                &mut raw_face,
            )
        };
        drop(_l);
        Error::or_else(err, || Face(NonNull::new(raw_face).unwrap(), vec![data]))
    }

    fn inner(&self) -> &sys::FT_FaceRec {
        // SAFETY: Internal pointer is guaranteed valid
        //         Faces are !Send+!Sync - as such, we know no C code will violate this reference
        unsafe { self.0.as_ref() }
    }

    pub fn is_scalable(&self) -> bool {
        self.inner().face_flags & sys::FT_FACE_FLAG_SCALABLE != 0
    }

    pub fn is_sfnt(&self) -> bool {
        self.inner().face_flags & sys::FT_FACE_FLAG_SFNT != 0
    }

    pub fn has_glyph_names(&self) -> bool {
        self.inner().face_flags & sys::FT_FACE_FLAG_GLYPH_NAMES != 0
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
        GlyphSlot(NonNull::new(self.inner().glyph).unwrap(), PhantomData)
    }

    pub fn get_advance(&self, index: u32, flags: LoadFlags) -> Result<i64, Error> {
        let mut advance = 0;
        // SAFETY: Out internal pointer is guaranteed valid
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
        // SAFETY: Our internal pointer is guaranteed valid
        let code = unsafe { sys::FT_Get_First_Char(self.0.as_ptr(), &mut index) };
        (code as u32, index as u32)
    }

    pub fn get_next_char(&self, char: u32) -> (u32, u32) {
        let mut index = 0;
        // SAFETY: Our internal pointer is guaranteed valid
        let code =
            unsafe { sys::FT_Get_Next_Char(self.0.as_ptr(), char as libc::c_ulong, &mut index) };
        (code as u32, index as u32)
    }

    pub fn get_char_index(&self, char: u32) -> Option<NonZeroU32> {
        // SAFETY: Our internal pointer is guaranteed valid
        NonZeroU32::new(unsafe { sys::FT_Get_Char_Index(self.0.as_ptr(), char as libc::c_ulong) })
    }

    pub fn get_char_variant_index(&self, char: u32, variant: u32) -> Option<NonZeroU32> {
        // SAFETY: Our internal pointer is guaranteed valid
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
        // SAFETY: Our internal pointer is guaranteed valid
        let err = unsafe { sys::FT_Get_Kerning(self.0.as_ptr(), left, right, mode, &mut vector) };
        Error::or_else(err, || vector)
    }

    pub fn get_glyph_name<'a>(&self, index: u32, buf: &'a mut [u8]) -> Result<&'a CStr, Error> {
        // SAFETY: Our internal pointer is guaranteed valid
        let err = unsafe {
            sys::FT_Get_Glyph_Name(
                self.0.as_ptr(),
                index as libc::c_uint,
                buf.as_mut_ptr().cast(),
                buf.len() as libc::c_uint,
            )
        };
        Error::or_else(err, move || CStr::from_bytes_until_nul(buf).unwrap())
    }

    pub fn get_name_index(&self, name: &CStr) -> Option<NonZeroU32> {
        // SAFETY: Our internal pointer is guaranteed valid, name is valid for the length of this
        //         method call and not retained.
        NonZeroU32::new(unsafe { sys::FT_Get_Name_Index(self.0.as_ptr(), name.as_ptr()) as u32 })
    }

    pub fn get_postscript_name(&self) -> Option<&CStr> {
        // SAFETY: Our internal pointer is guaranteed valid
        let ptr = unsafe { sys::FT_Get_Postscript_Name(self.0.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            // SAFETY: If a non-null pointer was returned, it is guaranteed to contain a
            //         nul-terminated C-string
            Some(unsafe { CStr::from_ptr(ptr) })
        }
    }

    pub fn get_sfnt_name_count(&self) -> u32 {
        // SAFETY: Our internal pointer is guaranteed valid
        unsafe { sys::FT_Get_Sfnt_Name_Count(self.0.as_ptr()) as u32 }
    }

    pub fn get_sfnt_name(&self, idx: u32) -> Result<SfntName<'_>, Error> {
        let mut name = sys::FT_SfntName::default();
        // SAFETY: Our internal pointer is guaranteed valid
        let err = unsafe { sys::FT_Get_Sfnt_Name(self.0.as_ptr(), idx as libc::c_uint, &mut name) };
        Error::or_else(err, || SfntName::from(name))
    }

    pub fn load_glyph(&mut self, index: u32, flags: LoadFlags) -> Result<(), Error> {
        // SAFETY: Our internal pointer is guaranteed valid
        let err =
            unsafe { sys::FT_Load_Glyph(self.0.as_ptr(), index as libc::c_uint, flags.as_i32()) };
        Error::or_else(err, || ())
    }

    pub fn get_sfnt_table_dyn(&self, tag: SfntTag) -> Option<NonNull<()>> {
        // SAFETY: Our internal pointer is guaranteed valid
        NonNull::new(unsafe { sys::FT_Get_Sfnt_Table(self.0.as_ptr(), tag) })
    }

    pub fn get_sfnt_table<T: Table>(&self) -> Option<&T::Table> {
        // SAFETY: Our internal pointer is guaranteed valid
        let ptr = unsafe { sys::FT_Get_Sfnt_Table(self.0.as_ptr(), T::tag()) };
        // SAFETY: Per the guarantees of `Table`, the pointer returned by FT_Get_Sfnt_Table is a
        //         valid `T::Table` if non-null.
        unsafe { ptr.cast::<T::Table>().as_ref() }
    }

    pub fn load_sfnt_table(&self, tag: TableTag) -> Result<Vec<u8>, Error> {
        let mut len = 0;
        // SAFETY: Our internal pointer is guaranteed valid
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

        // SAFETY: Our internal pointer is guaranteed valid. We know from previously calling the
        //         method with the same tag that buffer is of sufficient length to hold the returned
        //         table.
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
        // SAFETY: Our internal pointer is guaranteed valid
        //         We ensure above that the attached stream will not be dropped until after
        //         FT_Done_Face is called.
        let err = unsafe { sys::FT_Attach_Stream(self.0.as_ptr(), &mut oa) };
        Error::or_else(err, || ())
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        let _l = FACE_MUTEX.lock();
        // SAFETY: We hold the FACE_MUTEX lock, so calls to FT_Done_Face are guaranteed safe
        //         with our global library instance.
        //         Our pointer is owned and guaranteed valid.
        unsafe { sys::FT_Done_Face(self.0.as_ptr()) };
        drop(_l);
    }
}
