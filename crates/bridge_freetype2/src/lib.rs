// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the `FreeType2` *C* API into the Cargo framework, as well as provide
//! bindings to other tectonic crates.

#![deny(clippy::undocumented_unsafe_blocks, missing_docs)]
#![allow(clippy::unnecessary_cast)]

use std::ffi::CStr;
use std::marker::PhantomData;
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

/// Recognized tables that can be safely loaded by Rust code
pub mod tables {
    pub use crate::sys::TT_Header as Header;
    pub use crate::sys::TT_Postscript as Postscript;
    pub use crate::sys::TT_OS2 as Os2;
}

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
                panic!("FreeType initialization failed, error {error}");
            }
            SyncPtr(lib)
        })
        .0
}

/// Initialize FreeType. This function will panic if initialization fails.
pub fn init() {
    ft_lib();
}

/// A tag representing a specific table to load.
#[derive(Copy, Clone, Debug)]
pub enum TableTag {
    /// A well-known table type
    Sfnt(SfntTag),
    /// A custom table type not natively recognized by freetype
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

/// Glyph loading flags, to control whether some steps are performed, or change loading modes
/// (such as horizontal or vertical text).
#[derive(Copy, Clone, Debug)]
pub struct LoadFlags(i32);

impl LoadFlags {
    /// Default loading behavior.
    pub const DEFAULT: LoadFlags = LoadFlags(0);
    /// Don't scale the loaded glyph, keep it in font units.
    pub const NO_SCALE: LoadFlags = LoadFlags(sys::FT_LOAD_NO_SCALE);
    /// Load the glyph for a vertical text layout.
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

/// A TrueType platform ID
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlatformId(u16);

impl PlatformId {
    /// Apple unicode character map
    pub const APPLE_UNICODE: PlatformId = PlatformId(sys::TT_PLATFORM_APPLE_UNICODE as u16);
    /// Apple MacOS-specific character map
    pub const MACINTOSH: PlatformId = PlatformId(sys::TT_PLATFORM_MACINTOSH as u16);
    /// Windows-specific character map
    pub const MICROSOFT: PlatformId = PlatformId(sys::TT_PLATFORM_MICROSOFT as u16);
}

/// A TrueType encoding ID
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EncodingId(u16);

impl EncodingId {
    /// MacOS-specific roman encoding.
    pub const MAC_ROMAN: EncodingId = EncodingId(sys::TT_MAC_ID_ROMAN as u16);
    /// Windows-specific unicode encoding.
    pub const WIN_UNICODE: EncodingId = EncodingId(sys::TT_MS_ID_UNICODE_CS as u16);
}

/// A TrueType language ID
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LanguageId(u16);

impl LanguageId {
    /// MacOS-specific english language
    pub const MAC_ENGLISH: LanguageId = LanguageId(sys::TT_MAC_LANGID_ENGLISH as u16);

    /// Windows-specific US english language
    pub const WIN_ENGLISH_US: LanguageId =
        LanguageId(sys::TT_MS_LANGID_ENGLISH_UNITED_STATES as u16);
}

/// An error code returned by freetype.
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

    /// Convert this error into a descriptive message, if available.
    ///
    /// Error strings may not be available if freetype was compiled without certain options. In
    /// such a situation, this method will always return `None`.
    pub fn message(&self) -> Option<&'static str> {
        // SAFETY: This is always safe to call
        let str = unsafe { sys::FT_Error_String(self.0) };
        if str.is_null() {
            None
        } else {
            // SAFETY: If non-null, the pointer returned by FT_Error_String is a valid C-string
            let str = unsafe { CStr::from_ptr(str) };
            str::from_utf8(str.to_bytes()).ok()
        }
    }

    /// The numeric code of this error.
    ///
    /// **Warning:** These codes can, and do, change between freetype versions.
    pub fn code(&self) -> sys::FT_Error {
        self.0
    }
}

/// A font name table value
#[derive(Debug)]
#[non_exhaustive]
pub struct SfntName<'a> {
    /// The platform for this font
    pub platform_id: PlatformId,
    /// The encoding for this font
    pub encoding_id: EncodingId,
    /// The language for this font
    pub language_id: LanguageId,
    /// The type of name this is
    pub name_id: u16,
    /// The name string. May be UTF-16 BE, or any string of bytes
    pub string: &'a [u8],
}

impl From<sys::FT_SfntName> for SfntName<'_> {
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

/// A generic glyph image.
pub struct Glyph(NonNull<sys::FT_GlyphRec>);

impl Glyph {
    /// Get a glyph's 'control box'. This is faster to calculate than a proper bounding box,
    /// but may over-estimate in some cases.
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

/// A glyph loaded into a faces glyph slot.
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

    /// Create a glyph image from this slot.
    pub fn get_glyph(&self) -> Result<Glyph, Error> {
        let mut ptr = ptr::null_mut();
        // SAFETY: Internal pointer is guaranteed valid.
        let err = unsafe { sys::FT_Get_Glyph(self.0.as_ptr(), &mut ptr) };
        Error::or_else(err, || Glyph(NonNull::new(ptr).unwrap()))
    }

    /// Metrics of the currently loaded glyph.
    pub fn metrics(&self) -> &GlyphMetrics {
        &self.as_ref().metrics
    }

    /// The format of the currently loaded glyph.
    pub fn format(&self) -> GlyphFormat {
        self.as_ref().format
    }

    /// If the current format is an outline, the outline descriptor for it.
    pub fn outline(&self) -> Option<&Outline> {
        if self.format() == GlyphFormat::Outline {
            Some(&self.as_ref().outline)
        } else {
            None
        }
    }
}

/// Trait for types that represent different sfnt tables you can read from a [`Face`].
///
/// # Safety
///
/// Code may assume that the [`SfntTag`] returned by `tag` is valid to read as an instance of `Self`.
pub unsafe trait Table {
    /// The [`SfntTag`] that matches this table
    fn tag() -> SfntTag;
}

// SAFETY: SfntTag::Os2 == tables::OS2
unsafe impl Table for tables::Os2 {
    fn tag() -> SfntTag {
        SfntTag::Os2
    }
}

// SAFETY: SfntTag::Head == tables::Header
unsafe impl Table for tables::Header {
    fn tag() -> SfntTag {
        SfntTag::Head
    }
}

// SAFETY: SfntTag::Post == tables::Postscript
unsafe impl Table for tables::Postscript {
    fn tag() -> SfntTag {
        SfntTag::Post
    }
}

/// A typographic face - the combination of a general typeface and a style.
pub struct Face(NonNull<sys::FT_FaceRec>, Vec<Vec<u8>>);

impl Face {
    /// Create a new [`Face`] object from the font file at a given path, plus the index of the
    /// face within the file (as single files may contain the data for multiple styles).
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

    /// Create a new [`Face`] object from an in-memory buffer containing font file data. The index
    /// selects which face to load for data with multiple faces.
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

    /// Check whether this face is scalable
    pub fn is_scalable(&self) -> bool {
        self.inner().face_flags & sys::FT_FACE_FLAG_SCALABLE != 0
    }

    /// Check whether this face is using the SFNT storage scheme
    pub fn is_sfnt(&self) -> bool {
        self.inner().face_flags & sys::FT_FACE_FLAG_SFNT != 0
    }

    /// Check whether this face has glyph names
    pub fn has_glyph_names(&self) -> bool {
        self.inner().face_flags & sys::FT_FACE_FLAG_GLYPH_NAMES != 0
    }

    /// The the units per EM square for this face. Only relevant for scalable fonts.
    pub fn units_per_em(&self) -> u16 {
        self.inner().units_per_EM as u16
    }

    /// The typographic ascender for this face. Only relevant for scalable fonts.
    pub fn ascender(&self) -> i16 {
        self.inner().ascender as i16
    }

    /// The typographic descender for this face. Only relevant for scalable fonts.
    pub fn descender(&self) -> i16 {
        self.inner().descender as i16
    }

    /// The number of glyphs in this face
    pub fn num_glyphs(&self) -> usize {
        self.inner().num_glyphs as usize
    }

    /// The number of faces in the backing font file
    pub fn num_faces(&self) -> usize {
        self.inner().num_faces as usize
    }

    fn glyph(&self) -> GlyphSlot<'_> {
        GlyphSlot(NonNull::new(self.inner().glyph).unwrap(), PhantomData)
    }

    /// Get the advance for the given glyph in this font. If [`LoadFlags::VERTICAL_LAYOUT`] is set,
    /// then the return value is the vertical advance, otherwise it is the horizontal advance.
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

    /// Returns the first character code in the charmap of this face, along with its glyph index.
    pub fn get_first_char(&self) -> (u32, u32) {
        let mut index = 0;
        // SAFETY: Our internal pointer is guaranteed valid
        let code = unsafe { sys::FT_Get_First_Char(self.0.as_ptr(), &mut index) };
        (code as u32, index as u32)
    }

    /// Given a character code, return the next character code and glyph index in this face's
    /// charmap.
    pub fn get_next_char(&self, char: u32) -> (u32, u32) {
        let mut index = 0;
        // SAFETY: Our internal pointer is guaranteed valid
        let code =
            unsafe { sys::FT_Get_Next_Char(self.0.as_ptr(), char as libc::c_ulong, &mut index) };
        (code as u32, index as u32)
    }

    /// Given a character code, get the glyph index for it, if present.
    pub fn get_char_index(&self, char: u32) -> Option<NonZeroU32> {
        // SAFETY: Our internal pointer is guaranteed valid
        NonZeroU32::new(unsafe { sys::FT_Get_Char_Index(self.0.as_ptr(), char as libc::c_ulong) })
    }

    /// Given a character code and a Unicode variant selector, get the glyph index for it, if
    /// present.
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

    /// For a given glyph index, get the kerning vector between them. This only works for 'simple'
    /// kerning, for things such as vertical layouts or complicated GPOS layouts, something
    /// such as Harfbuzz is needed.
    pub fn get_kerning(&self, left: u32, right: u32, mode: KerningMode) -> Result<Vector, Error> {
        let mut vector = Vector::default();
        // SAFETY: Our internal pointer is guaranteed valid
        let err = unsafe { sys::FT_Get_Kerning(self.0.as_ptr(), left, right, mode, &mut vector) };
        Error::or_else(err, || vector)
    }

    /// If this font supports glyph names, get the name for the given glyph index.
    ///
    /// This is the inverse of [`Self::get_name_index`].
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

    /// If this font supports glyph names, get the index of a given glyph name.
    ///
    /// This is the inverse of [`Self::get_glyph_name`].
    pub fn get_name_index(&self, name: &CStr) -> Option<NonZeroU32> {
        // SAFETY: Our internal pointer is guaranteed valid, name is valid for the length of this
        //         method call and not retained.
        NonZeroU32::new(unsafe { sys::FT_Get_Name_Index(self.0.as_ptr(), name.as_ptr()) as u32 })
    }

    /// Get the postscript name for this font, if one is available
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

    /// Get the number of items in the name table for this face
    pub fn get_sfnt_name_count(&self) -> u32 {
        // SAFETY: Our internal pointer is guaranteed valid
        unsafe { sys::FT_Get_Sfnt_Name_Count(self.0.as_ptr()) as u32 }
    }

    /// Get the item in the name table at the given index for this face
    pub fn get_sfnt_name(&self, idx: u32) -> Result<SfntName<'_>, Error> {
        let mut name = sys::FT_SfntName::default();
        // SAFETY: Our internal pointer is guaranteed valid. The returned name is valid until self
        //         is dropped.
        let err = unsafe { sys::FT_Get_Sfnt_Name(self.0.as_ptr(), idx as libc::c_uint, &mut name) };
        Error::or_else(err, || SfntName::from(name))
    }

    /// Load and return a reference to a glyph in this font. See [`LoadFlags`] for details on how
    /// different flags affect the loaded glyph.
    pub fn load_glyph(&mut self, index: u32, flags: LoadFlags) -> Result<GlyphSlot<'_>, Error> {
        // SAFETY: Our internal pointer is guaranteed valid
        let err =
            unsafe { sys::FT_Load_Glyph(self.0.as_ptr(), index as libc::c_uint, flags.as_i32()) };
        match Error::or_else(err, || ()) {
            Ok(_) => Ok(self.glyph()),
            Err(e) => Err(e),
        }
    }

    /// Get a pointer to the table represented by [`SfntTag`]. This pointer is only valid for the
    /// lifetime of this Face, and may be mutated by mutable calls to self.
    pub fn get_sfnt_table_dyn(&self, tag: SfntTag) -> Option<NonNull<()>> {
        // SAFETY: Our internal pointer is guaranteed valid
        NonNull::new(unsafe { sys::FT_Get_Sfnt_Table(self.0.as_ptr(), tag) })
    }

    /// Get a reference to a loaded TrueType table. This may return `None` if the table doesn't
    /// exist in the loaded font, or wasn't loaded.
    pub fn get_sfnt_table<T: Table>(&self) -> Option<&T> {
        // SAFETY: Our internal pointer is guaranteed valid
        let ptr = unsafe { sys::FT_Get_Sfnt_Table(self.0.as_ptr(), T::tag()) };
        // SAFETY: Per the guarantees of `Table`, the pointer returned by FT_Get_Sfnt_Table is a
        //         valid `T` if non-null.
        unsafe { ptr.cast::<T>().as_ref() }
    }

    /// Load a font table into an owned buffer.
    ///
    /// [`SfntTag::Head`] will, instead of loading the header, actually return the entire font file.
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

    /// Attach a custom byte stream to this face as extra data. This stream will be freed when this
    /// face is dropped.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Import things from our bridge crates to ensure that we actually link with
    /// them.
    mod linkage {
        #[allow(unused_imports)]
        use tectonic_bridge_flate as clippyrenamehack1;
        #[allow(unused_imports)]
        use tectonic_bridge_png as clippyrenamehack2;
    }

    fn assets_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/assets")
            .canonicalize()
            .unwrap()
    }

    fn get_face_data() -> Vec<u8> {
        let roman = assets_dir().join("lmroman12-regular.otf");
        fs::read(roman).unwrap()
    }

    fn get_face_file() -> Face {
        let roman = assets_dir().join("lmroman12-regular.otf");
        let roman_path = CString::new(roman.to_str().unwrap()).unwrap();

        Face::new(&roman_path, 0).unwrap()
    }

    fn get_face_mem() -> Face {
        let roman_data = get_face_data();

        Face::new_memory(roman_data, 0).unwrap()
    }

    fn test_faces() -> Vec<Face> {
        vec![get_face_file(), get_face_mem()]
    }

    #[test]
    fn test_invalid_face() {
        assert!(Face::new(c"/bad/path", 0).is_err());
        assert!(Face::new_memory(vec![], 0).is_err());
    }

    #[test]
    fn test_properties() {
        for face in test_faces() {
            assert!(face.is_scalable());
            assert!(face.is_sfnt());
            assert!(face.has_glyph_names());
            assert_eq!(face.units_per_em(), 1000);
            assert_eq!(face.ascender(), 1127);
            assert_eq!(face.descender(), -280);
            assert_eq!(face.num_glyphs(), 821);
            assert_eq!(face.num_faces(), 1);
        }
    }

    #[test]
    fn test_roman_name() {
        for face in test_faces() {
            assert_eq!(face.get_sfnt_name_count(), 21);

            let mac_name = face.get_sfnt_name(1).unwrap();

            let name_16 = "Latin Modern Roman"
                .encode_utf16()
                .flat_map(|val| val.to_be_bytes())
                .collect::<Vec<u8>>();

            assert_eq!(mac_name.platform_id, PlatformId::MACINTOSH);
            assert_eq!(mac_name.encoding_id, EncodingId::MAC_ROMAN);
            assert_eq!(mac_name.language_id, LanguageId::MAC_ENGLISH);
            assert_eq!(mac_name.string, b"Latin Modern Roman");

            let win_name = face.get_sfnt_name(18).unwrap();
            assert_eq!(win_name.platform_id, PlatformId::MICROSOFT);
            assert_eq!(win_name.encoding_id, EncodingId::WIN_UNICODE);
            assert_eq!(win_name.language_id, LanguageId::WIN_ENGLISH_US);
            assert_eq!(win_name.string, &name_16);
        }
    }

    #[test]
    fn test_postscript_name() {
        for face in test_faces() {
            let name = face.get_postscript_name().unwrap();
            assert_eq!(name.to_bytes(), b"LMRoman12-Regular");
        }
    }

    #[test]
    fn test_get_tables() {
        for face in test_faces() {
            let dyn_table = face.get_sfnt_table_dyn(SfntTag::Head).unwrap();
            let table = face.get_sfnt_table::<tables::Header>().unwrap();

            assert!(ptr::eq(table, dyn_table.as_ptr().cast_const().cast()));
            assert_eq!(table.Units_Per_Em, face.units_per_em());

            let dyn_table = face.get_sfnt_table_dyn(SfntTag::Os2).unwrap();
            let table = face.get_sfnt_table::<tables::Os2>().unwrap();
            assert!(ptr::eq(table, dyn_table.as_ptr().cast_const().cast()));

            let dyn_table = face.get_sfnt_table_dyn(SfntTag::Post).unwrap();
            let table = face.get_sfnt_table::<tables::Postscript>().unwrap();
            assert!(ptr::eq(table, dyn_table.as_ptr().cast_const().cast()));
        }
    }

    #[test]
    fn test_load_tables() {
        let data = get_face_data();
        for face in test_faces() {
            assert_eq!(
                data,
                face.load_sfnt_table(TableTag::Sfnt(SfntTag::Head)).unwrap()
            );
            face.load_sfnt_table(TableTag::Other(u32::MAX)).unwrap_err();
        }
    }

    #[test]
    fn test_get_char_index() {
        for face in test_faces() {
            face.get_char_index('!' as u32).unwrap();
            assert!(face.get_char_index('∀' as u32).is_none());
        }
    }

    #[test]
    fn test_attach_stream() {
        for mut face in test_faces() {
            face.attach_stream_mem(b"Hello World!".to_vec())
                .unwrap_err();
        }
    }

    #[test]
    fn test_bad_glyph() {
        const BAD_IDX: u32 = 10000;
        for mut face in test_faces() {
            assert!(face.get_name_index(c"bad_name").is_none());
            face.get_advance(BAD_IDX, LoadFlags::DEFAULT).unwrap_err();
            face.get_glyph_name(BAD_IDX, &mut []).unwrap_err();
            assert!(face.load_glyph(BAD_IDX, LoadFlags::DEFAULT).is_err());
        }
    }

    #[test]
    fn test_first_glyph() {
        let mut buf = vec![0; 32];

        for face in test_faces() {
            let (c, idx) = face.get_first_char();
            assert_eq!(c, ' ' as u32);

            let advance = face.get_advance(idx, LoadFlags::NO_SCALE).unwrap();
            assert_eq!(advance, 326);
            assert_eq!(face.get_glyph_name(idx, &mut buf).unwrap(), c"space");
            assert_eq!(face.get_char_index(' ' as u32).unwrap().get(), idx);
            assert_eq!(face.get_name_index(c"space").unwrap().get(), idx);

            let (c2, _) = face.get_next_char(c);
            assert_eq!(c2, '!' as u32);

            // No variant selectors in this font
            assert_eq!(face.get_char_variant_index(c, 0xFE00), None);
        }
    }

    #[test]
    fn test_kerning() {
        for face in test_faces() {
            let exc_idx = face.get_char_index('!' as u32).unwrap().get();
            let at_idx = face.get_char_index('@' as u32).unwrap().get();

            let kern = face
                .get_kerning(exc_idx, at_idx, KerningMode::Default)
                .unwrap();
            // TODO: Find pair of characters with non-zero kerning
            assert_eq!(kern, Vector { x: 0, y: 0 });
        }
    }

    #[test]
    fn test_glyph_load() {
        for mut face in test_faces() {
            let idx = face.get_char_index(b'@' as u32).unwrap().get();

            let slot = face.load_glyph(idx, LoadFlags::NO_SCALE).unwrap();

            assert_eq!(slot.format(), GlyphFormat::Outline);
            assert_eq!(slot.metrics().height, 714);
            assert_eq!(slot.metrics().width, 653);

            let outline = slot.outline().unwrap();
            assert_eq!(outline.points().len(), 82);
            assert_eq!(outline.contours().len(), 2);

            let glyph = slot.get_glyph().unwrap();

            assert_eq!(
                glyph.get_cbox(BBoxMode::Unscaled),
                BBox {
                    x_min: 54,
                    x_max: 707,
                    y_min: -10,
                    y_max: 704
                }
            );
        }
    }
}
