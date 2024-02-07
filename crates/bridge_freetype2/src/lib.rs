// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! This crate exists to export the FreeType2 *C* API into the Cargo framework, as well as provide
//! bindings to other tectonic crates.

#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;
use std::ptr;

pub const FT_LOAD_NO_SCALE: i32 = 1 << 0;
pub const FT_LOAD_VERTICAL_LAYOUT: i32 = 1 << 4;

pub const FT_FACE_FLAG_SCALABLE: libc::c_long = 1 << 0;
pub const FT_FACE_FLAG_SFNT: libc::c_long = 1 << 3;
pub const FT_FACE_FLAG_GLYPH_NAMES: libc::c_long = 1 << 9;

pub const FT_OPEN_MEMORY: libc::c_uint = 0x1;

pub const TT_PLATFORM_APPLE_UNICODE: libc::c_ushort = 0;
pub const TT_PLATFORM_MACINTOSH: libc::c_ushort = 1;
pub const TT_PLATFORM_MICROSOFT: libc::c_ushort = 3;

pub const TT_MAC_ID_ROMAN: libc::c_ushort = 0;

pub unsafe fn FT_IS_SCALABLE(face: FT_Face) -> bool {
    ((*face).face_flags & FT_FACE_FLAG_SCALABLE) != 0
}

pub unsafe fn FT_IS_SFNT(face: FT_Face) -> bool {
    ((*face).face_flags & FT_FACE_FLAG_SFNT) != 0
}

pub unsafe fn FT_HAS_GLYPH_NAMES(face: FT_Face) -> bool {
    (*face).face_flags & FT_FACE_FLAG_GLYPH_NAMES != 0
}

#[repr(C)]
pub enum FT_Kerning_Mode {
    Default = 0,
    Unfitted,
    Unscaled,
}

#[repr(C)]
pub enum FT_Glyph_BBox_Mode {
    Unscaled = 0,
    GridFit = 1,
    Truncate = 2,
    Pixels = 3,
}

#[repr(C)]
pub struct FT_FaceRec {
    pub num_faces: libc::c_long,
    pub face_index: libc::c_long,

    pub face_flags: libc::c_long,
    pub style_flags: libc::c_long,

    pub num_glyphs: libc::c_long,

    pub family_name: *const libc::c_char,
    pub style_name: *const libc::c_char,

    pub num_fixed_sizes: libc::c_int,
    pub available_sizes: *mut FT_Bitmap_Size,

    pub num_charmaps: libc::c_int,
    pub charmaps: *mut FT_CharMap,

    pub generic: FT_Generic,

    pub bbox: FT_BBox,

    pub units_per_EM: libc::c_ushort,
    pub ascender: libc::c_short,
    pub descender: libc::c_short,
    pub height: libc::c_short,

    pub max_advance_width: libc::c_short,
    pub max_advance_height: libc::c_short,

    pub underline_position: libc::c_short,
    pub underline_thickness: libc::c_short,

    pub glyph: FT_GlyphSlot,
    pub size: FT_Size,
    pub charmap: FT_CharMap,

    // TODO: Remaining fields
    _priv: PhantomData<*mut ()>,
}

#[repr(C)]
pub struct FT_SizeRec {
    // TODO
    _priv: [u8; 1],
}

#[repr(C)]
pub struct FT_CharMapRec {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: libc::c_ushort,
    pub encoding_id: libc::c_ushort,
}

#[repr(C)]
pub struct FT_GlyphSlotRec {
    pub library: FT_Library,
    pub face: FT_Face,
    pub next: FT_GlyphSlot,
    pub glyph_index: libc::c_uint,
    pub generic: FT_Generic,

    pub metrics: FT_Glyph_Metrics,
    pub linearHoriAdvance: FT_Fixed,
    pub linearVertAdvance: FT_Fixed,
    pub advance: FT_Vector,

    pub format: FT_Glyph_Format,

    pub bitmap: FT_Bitmap,
    pub bitmap_left: libc::c_int,
    pub bitmap_top: libc::c_int,

    pub outline: FT_Outline,
}

#[repr(C)]
pub struct FT_LibraryRec {
    // TODO
    _priv: [u8; 1],
}

#[repr(C)]
pub struct FT_StreamRec {
    // TODO
    _priv: [u8; 1],
}

#[repr(C)]
pub struct FT_ModuleRec {
    // TODO
    _priv: [u8; 1],
}

#[repr(C)]
pub struct FT_GlyphRec {
    // TODO
    _priv: [u8; 1],
}

#[repr(C)]
pub struct FT_SfntName {
    pub platform_id: libc::c_ushort,
    pub encoding_id: libc::c_ushort,
    pub language_id: libc::c_ushort,
    pub name_id: libc::c_ushort,

    /// *Not* null terminated
    pub string: *mut u8,
    pub string_len: libc::c_uint,
}

impl Default for FT_SfntName {
    fn default() -> Self {
        FT_SfntName {
            platform_id: 0,
            encoding_id: 0,
            language_id: 0,
            name_id: 0,
            string: ptr::null_mut(),
            string_len: 0,
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct FT_Vector {
    pub x: FT_Pos,
    pub y: FT_Pos,
}

#[repr(C)]
pub struct FT_Bitmap_Size {
    pub height: libc::c_short,
    pub width: libc::c_short,

    pub size: FT_Pos,

    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos,
}

#[repr(C)]
pub enum FT_Encoding {
    None = 0,
}

const fn as_variant(text: &[u8; 4]) -> isize {
    i32::from_be_bytes(*text) as isize
}

#[derive(PartialEq)]
#[repr(C)]
pub enum FT_Glyph_Format {
    None = 0,
    Composite = as_variant(b"comp"),
    Bitmap = as_variant(b"bits"),
    Outline = as_variant(b"outl"),
    Plotter = as_variant(b"plot"),
    Svg = as_variant(b"SVG "),
}

#[derive(PartialEq)]
#[repr(C)]
pub enum FT_Sfnt_Tag {
    Head,
    MaxP,
    Os2,
    HHEA,
    VHEA,
    Post,
    PCLT,
    Max,
}

#[repr(C)]
pub struct FT_Generic {
    pub data: *mut (),
    pub finalizer: FT_Generic_Finalizer,
}

#[derive(Default)]
#[repr(C)]
pub struct FT_BBox {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}

#[repr(C)]
pub struct FT_Glyph_Metrics {
    pub width: FT_Pos,
    pub height: FT_Pos,

    pub horiBearingX: FT_Pos,
    pub horiBearingY: FT_Pos,
    pub horiAdvance: FT_Pos,

    pub vertBearingX: FT_Pos,
    pub vertBearingY: FT_Pos,
    pub vertAdvance: FT_Pos,
}

#[repr(C)]
pub struct FT_Bitmap {
    pub rows: libc::c_uint,
    pub width: libc::c_uint,
    pub pitch: libc::c_int,
    pub buffer: *mut libc::c_char,
    pub num_grays: libc::c_ushort,
    pub pixel_mode: libc::c_uchar,
    pub palette_mode: libc::c_uchar,
    pub palette: *mut (),
}

#[repr(C)]
pub struct FT_Outline {
    pub n_contours: libc::c_short,
    pub n_points: libc::c_short,

    pub points: *mut FT_Vector,
    pub tags: *mut libc::c_char,
    pub contours: *mut libc::c_short,

    pub flags: libc::c_int,
}

#[repr(C)]
pub struct FT_Open_Args {
    pub flags: libc::c_uint,
    pub memory_base: *const libc::c_uchar,
    pub memory_size: libc::c_long,
    pub pathname: *mut libc::c_char,
    pub stream: FT_Stream,
    pub driver: FT_Module,
    pub num_params: libc::c_int,
    pub params: *mut FT_Parameter,
}

impl Default for FT_Open_Args {
    fn default() -> Self {
        FT_Open_Args {
            flags: 0,
            memory_base: ptr::null(),
            memory_size: 0,
            pathname: ptr::null_mut(),
            stream: ptr::null_mut(),
            driver: ptr::null_mut(),
            num_params: 0,
            params: ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub struct FT_Parameter {
    tag: libc::c_ulong,
    data: *mut (),
}

#[repr(C)]
pub struct TT_Postscript {
    pub format_type: FT_Fixed,
    pub italic_angle: FT_Fixed,
    pub underlinePosition: libc::c_short,
    pub underlineThickness: libc::c_short,
    pub isFixedPitch: libc::c_ulong,
    pub minMemType42: libc::c_ulong,
    pub maxMemType42: libc::c_ulong,
    pub minMemType1: libc::c_ulong,
    pub maxMemType1: libc::c_ulong,
}

#[repr(C)]
pub struct TT_OS2 {
    pub version: libc::c_ushort,
    pub xAvgCharWidth: libc::c_short,
    pub usWeightClass: libc::c_ushort,
    pub usWidthClass: libc::c_ushort,
    pub fsType: libc::c_ushort,
    pub ySubscriptXSize: libc::c_short,
    pub ySubscriptYSize: libc::c_short,
    pub ySubscriptXOffset: libc::c_short,
    pub ySubscriptYOffset: libc::c_short,
    pub ySuperscriptXSize: libc::c_short,
    pub ySuperscriptYSize: libc::c_short,
    pub ySuperscriptXOffset: libc::c_short,
    pub ySuperscriptYOffset: libc::c_short,
    pub yStrikeoutSize: libc::c_short,
    pub yStrikeoutPosition: libc::c_short,
    pub sFamilyClass: libc::c_short,

    pub panose: [libc::c_uchar; 10],

    pub ulUnicodeRange1: libc::c_ulong,
    pub ulUnicodeRange2: libc::c_ulong,
    pub ulUnicodeRange3: libc::c_ulong,
    pub ulUnicodeRange4: libc::c_ulong,

    pub achVendID: [libc::c_char; 4],

    pub fsSelection: libc::c_ushort,
    pub usFirstCharIndex: libc::c_ushort,
    pub usLastCharIndex: libc::c_ushort,
    pub sTypoAscender: libc::c_short,
    pub sTypoDescender: libc::c_short,
    pub sTypoLineGap: libc::c_short,
    pub usWinAscent: libc::c_ushort,
    pub usWinDescent: libc::c_ushort,

    pub ulCodePageRange1: libc::c_ulong,
    pub ulCodePageRange2: libc::c_ulong,

    pub sxHeight: libc::c_short,
    pub sCapHeight: libc::c_short,
    pub usDefaultChar: libc::c_ushort,
    pub usBreakChar: libc::c_ushort,
    pub usMaxContext: libc::c_ushort,
}

#[repr(C)]
pub struct TT_Header {
    pub Table_Version: FT_Fixed,
    pub Font_Revision: FT_Fixed,
    pub CheckSum_Adjust: libc::c_long,
    pub Magic_Number: libc::c_long,
    pub Flags: libc::c_ushort,
    pub Units_Per_Em: libc::c_ushort,
    pub Created: [libc::c_ulong; 2],
    pub Modified: [libc::c_ulong; 2],
    pub xMin: libc::c_short,
    pub yMin: libc::c_short,
    pub xMax: libc::c_short,
    pub yMax: libc::c_short,
    pub Mac_Style: libc::c_ushort,
    // TODO
    // FT_UShort  Lowest_Rec_PPEM;
    //
    // FT_Short   Font_Direction;
    // FT_Short   Index_To_Loc_Format;
    // FT_Short   Glyph_Data_Format;
}

pub type FT_Face = *mut FT_FaceRec;
pub type FT_Size = *mut FT_SizeRec;
pub type FT_CharMap = *mut FT_CharMapRec;
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec;
pub type FT_Library = *mut FT_LibraryRec;
pub type FT_Stream = *mut FT_StreamRec;
pub type FT_Module = *mut FT_ModuleRec;
pub type FT_Glyph = *mut FT_GlyphRec;
pub type FT_Fixed = libc::c_long;
pub type FT_Error = libc::c_int;
pub type FT_Pos = libc::c_long;
pub type FT_String = libc::c_char;
pub type FT_Generic_Finalizer = extern "C" fn(object: *mut ());
pub type FT_Pointer = *mut ();

extern "C" {
    pub fn FT_Get_Char_Index(face: FT_Face, charcode: libc::c_ulong) -> libc::c_uint;
    pub fn FT_Face_GetCharVariantIndex(
        face: FT_Face,
        charcode: libc::c_ulong,
        variant_selector: libc::c_ulong,
    ) -> libc::c_uint;
    pub fn FT_Get_Advance(
        face: FT_Face,
        gindex: libc::c_uint,
        load_flags: i32,
        padvance: *mut FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Get_Kerning(
        face: FT_Face,
        left_glyph: libc::c_uint,
        right_glyph: libc::c_uint,
        kern_mode: FT_Kerning_Mode,
        akerning: *mut FT_Vector,
    ) -> FT_Error;
    pub fn FT_Load_Glyph(face: FT_Face, glyph_index: libc::c_uint, load_flags: i32) -> FT_Error;
    pub fn FT_Get_Glyph_Name(
        face: FT_Face,
        glyph_index: libc::c_uint,
        buffer: FT_Pointer,
        buffer_max: libc::c_uint,
    ) -> FT_Error;
    pub fn FT_Load_Sfnt_Table(
        face: FT_Face,
        tag: libc::c_ulong,
        offset: libc::c_long,
        buffer: *mut libc::c_uchar,
        length: *mut libc::c_ulong,
    ) -> FT_Error;
    pub fn FT_Init_FreeType(alibrary: *mut FT_Library) -> FT_Error;
    pub fn FT_New_Memory_Face(
        library: FT_Library,
        file_base: *mut libc::c_uchar,
        file_size: libc::c_long,
        face_index: libc::c_long,
        aface: *mut FT_Face,
    ) -> FT_Error;
    pub fn FT_Attach_Stream(face: FT_Face, parameters: *mut FT_Open_Args) -> FT_Error;
    pub fn FT_Get_Sfnt_Table(face: FT_Face, tag: FT_Sfnt_Tag) -> *mut ();
    pub fn FT_Done_Face(face: FT_Face) -> FT_Error;
    pub fn FT_Get_Glyph(slot: FT_GlyphSlot, aglyph: *mut FT_Glyph) -> FT_Error;
    pub fn FT_Glyph_Get_CBox(glyph: FT_Glyph, bbox_mode: FT_Glyph_BBox_Mode, acbox: *mut FT_BBox);
    pub fn FT_Done_Glyph(glyph: FT_Glyph);
    pub fn FT_Get_First_Char(face: FT_Face, agindex: *mut libc::c_uint) -> libc::c_ulong;
    pub fn FT_Get_Next_Char(
        face: FT_Face,
        char_code: libc::c_ulong,
        agindex: *mut libc::c_uint,
    ) -> libc::c_ulong;
    pub fn FT_Get_Name_Index(face: FT_Face, glyph_name: *const FT_String) -> libc::c_uint;
    pub fn FT_New_Face(
        library: FT_Library,
        filepathname: *const libc::c_char,
        face_index: libc::c_long,
        aface: *mut FT_Face,
    ) -> FT_Error;
    pub fn FT_Get_Postscript_Name(face: FT_Face) -> *const libc::c_char;
    pub fn FT_Get_Sfnt_Name_Count(face: FT_Face) -> libc::c_uint;
    pub fn FT_Get_Sfnt_Name(face: FT_Face, idx: libc::c_uint, aname: *mut FT_SfntName) -> FT_Error;
}
