#![cfg(target_os="macos")]
#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           extern_types,
           ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type __CFString;
    pub type __CFAllocator;
    pub type __CFURL;
    pub type __CFDictionary;
    pub type __CFNumber;
    pub type FT_LibraryRec_;
    pub type FT_DriverRec_;
    pub type FT_Face_InternalRec_;
    pub type FT_Size_InternalRec_;
    pub type FT_Slot_InternalRec_;
    pub type FT_SubGlyphRec_;
    pub type __CFArray;
    pub type __CFBoolean;
    pub type __CFAttributedString;
    pub type CGColor;
    pub type CGFont;
    pub type __CTFontDescriptor;
    pub type __CTFont;
    pub type __CTLine;
    pub type __CTTypesetter;
    pub type __CTRun;
    #[no_mangle]
    static kCFBooleanTrue: CFBooleanRef;
    #[no_mangle]
    static CGAffineTransformIdentity: CGAffineTransform;
    #[no_mangle]
    fn CGColorCreateGenericRGB(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> CGColorRef;
    #[no_mangle]
    static kCTFontAttributeName: CFStringRef;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    /* Macs provide Fixed and FixedPoint */
    /* Misc */
    /* gFreeTypeLibrary is defined in xetex-XeTeXFontInst_FT2.cpp,
     * also used in xetex-XeTeXFontMgr_FC.cpp and xetex-ext.c.  */
    #[no_mangle]
    static mut gFreeTypeLibrary: FT_Library;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn FT_Get_Postscript_Name(face: FT_Face) -> *const libc::c_char;
    #[no_mangle]
    fn FT_Done_Face(face: FT_Face) -> FT_Error;
    #[no_mangle]
    fn FT_New_Face(
        library: FT_Library,
        filepathname: *const libc::c_char,
        face_index: FT_Long,
        aface: *mut FT_Face,
    ) -> FT_Error;
    #[no_mangle]
    fn FT_Init_FreeType(alibrary: *mut FT_Library) -> FT_Error;
    #[no_mangle]
    static mut font_area: *mut str_number;
    #[no_mangle]
    static mut font_layout_engine: *mut *mut libc::c_void;
    #[no_mangle]
    static mut font_letter_space: *mut scaled_t;
    #[no_mangle]
    static mut loaded_font_flags: libc::c_char;
    #[no_mangle]
    static mut loaded_font_letter_space: scaled_t;
    #[no_mangle]
    static mut native_font_type_flag: int32_t;
    /* ***************************************************************************\
     Part of the XeTeX typesetting system
     Copyright (c) 1994-2008 by SIL International
     Copyright (c) 2009, 2011 by Jonathan Kew
     Copyright (c) 2012, 2013 by Jiang Jiang
     Copyright (c) 2012-2015 by Khaled Hosny

     SIL Author(s): Jonathan Kew

    Permission is hereby granted, free of charge, to any person obtaining
    a copy of this software and associated documentation files (the
    "Software"), to deal in the Software without restriction, including
    without limitation the rights to use, copy, modify, merge, publish,
    distribute, sublicense, and/or sell copies of the Software, and to
    permit persons to whom the Software is furnished to do so, subject to
    the following conditions:

    The above copyright notice and this permission notice shall be
    included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
    FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
    CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
    WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

    Except as contained in this notice, the name of the copyright holders
    shall not be used in advertising or otherwise to promote the sale,
    use or other dealings in this Software without prior written
    authorization from the copyright holders.
    \****************************************************************************/
    /* some typedefs that XeTeX uses - on Mac OS, we get these from Apple headers,
    but otherwise we'll need these substitute definitions */
    /* command codes for XeTeX extension commands */
    /* accessing info in a native_word_node */
    /* info for each glyph is location (FixedPoint) + glyph ID (uint16_t) */
    /* glyph ID field in a glyph_node */
    /* For Unicode encoding form interpretation... */
    #[no_mangle]
    fn readCommonFeatures(
        feat: *const libc::c_char,
        end: *const libc::c_char,
        extend: *mut libc::c_float,
        slant: *mut libc::c_float,
        embolden: *mut libc::c_float,
        letterspace: *mut libc::c_float,
        rgbValue: *mut uint32_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn font_feature_warning(
        featureNameP: *const libc::c_void,
        featLen: int32_t,
        settingNameP: *const libc::c_void,
        setLen: int32_t,
    );
    #[no_mangle]
    fn read_double(s: *mut *const libc::c_char) -> libc::c_double;
    #[no_mangle]
    fn Fix2D(f: Fixed) -> libc::c_double;
    #[no_mangle]
    fn D2Fix(d: libc::c_double) -> Fixed;
    #[no_mangle]
    fn CFAttributedStringCreate(
        alloc: CFAllocatorRef,
        str: CFStringRef,
        attributes: CFDictionaryRef,
    ) -> CFAttributedStringRef;
    #[no_mangle]
    static kCTFontURLAttribute: CFStringRef;
    #[no_mangle]
    static kCTKernAttributeName: CFStringRef;
    #[no_mangle]
    fn CFNumberCreate(
        allocator: CFAllocatorRef,
        theType: CFNumberType,
        valuePtr: *const libc::c_void,
    ) -> CFNumberRef;
    #[no_mangle]
    fn CGColorRelease(color: CGColorRef);
    #[no_mangle]
    fn CTRunGetGlyphCount(run: CTRunRef) -> CFIndex;
    #[no_mangle]
    static kCFTypeArrayCallBacks: CFArrayCallBacks;
    #[no_mangle]
    static kCTFontPostScriptNameKey: CFStringRef;
    #[no_mangle]
    fn CTTypesetterCreateWithAttributedString(string: CFAttributedStringRef) -> CTTypesetterRef;
    #[no_mangle]
    fn CFNumberGetValue(
        number: CFNumberRef,
        theType: CFNumberType,
        valuePtr: *mut libc::c_void,
    ) -> Boolean;
    #[no_mangle]
    fn CGFontRelease(font: CGFontRef);
    #[no_mangle]
    static kCTForegroundColorAttributeName: CFStringRef;
    #[no_mangle]
    fn CGFontGetNumberOfGlyphs(font: CGFontRef) -> size_t;
    #[no_mangle]
    fn CTRunGetAttributes(run: CTRunRef) -> CFDictionaryRef;
    #[no_mangle]
    static kCTFontCascadeListAttribute: CFStringRef;
    #[no_mangle]
    fn CFURLGetFileSystemRepresentation(
        url: CFURLRef,
        resolveAgainstBase: Boolean,
        buffer: *mut UInt8,
        maxBufLen: CFIndex,
    ) -> Boolean;
    #[no_mangle]
    static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    #[no_mangle]
    fn CFNumberCompare(
        number: CFNumberRef,
        otherNumber: CFNumberRef,
        context: *mut libc::c_void,
    ) -> CFComparisonResult;
    #[no_mangle]
    static kCTFontFeatureSettingsAttribute: CFStringRef;
    #[no_mangle]
    static kCTFontOrientationAttribute: CFStringRef;
    #[no_mangle]
    static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;
    #[no_mangle]
    fn CGRectIsNull(rect: CGRect) -> bool;
    #[no_mangle]
    fn CTTypesetterCreateLine(typesetter: CTTypesetterRef, stringRange: CFRange) -> CTLineRef;
    #[no_mangle]
    fn CTRunGetGlyphs(run: CTRunRef, range: CFRange, buffer: *mut CGGlyph);
    #[no_mangle]
    fn CTFontCreateWithFontDescriptor(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;
    #[no_mangle]
    fn CTLineCreateJustifiedLine(
        line: CTLineRef,
        justificationFactor: CGFloat,
        justificationWidth: libc::c_double,
    ) -> CTLineRef;
    #[no_mangle]
    static kCTVerticalFormsAttributeName: CFStringRef;
    #[no_mangle]
    fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        cStr: *const libc::c_char,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
    #[no_mangle]
    fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        numBytes: CFIndex,
        encoding: CFStringEncoding,
        isExternalRepresentation: Boolean,
    ) -> CFStringRef;
    #[no_mangle]
    fn CTLineGetGlyphCount(line: CTLineRef) -> CFIndex;
    #[no_mangle]
    fn CGFontCopyGlyphNameForGlyph(font: CGFontRef, glyph: CGGlyph) -> CFStringRef;
    #[no_mangle]
    fn CTRunGetPositions(run: CTRunRef, range: CFRange, buffer: *mut CGPoint);
    #[no_mangle]
    fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
    #[no_mangle]
    fn CFArrayCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        callBacks: *const CFArrayCallBacks,
    ) -> CFMutableArrayRef;
    #[no_mangle]
    fn CFStringCreateWithCStringNoCopy(
        alloc: CFAllocatorRef,
        cStr: *const libc::c_char,
        encoding: CFStringEncoding,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
    #[no_mangle]
    fn CFStringCreateWithCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        numChars: CFIndex,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
    #[no_mangle]
    fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    #[no_mangle]
    fn CTRunGetAdvances(run: CTRunRef, range: CFRange, buffer: *mut CGSize);
    #[no_mangle]
    fn CFDictionaryCreate(
        allocator: CFAllocatorRef,
        keys: *mut *const libc::c_void,
        values: *mut *const libc::c_void,
        numValues: CFIndex,
        keyCallBacks: *const CFDictionaryKeyCallBacks,
        valueCallBacks: *const CFDictionaryValueCallBacks,
    ) -> CFDictionaryRef;
    #[no_mangle]
    fn CTFontDescriptorCreateWithNameAndSize(
        name: CFStringRef,
        size: CGFloat,
    ) -> CTFontDescriptorRef;
    #[no_mangle]
    fn CFStringGetLength(theString: CFStringRef) -> CFIndex;
    #[no_mangle]
    fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    #[no_mangle]
    fn CFStringGetCString(
        theString: CFStringRef,
        buffer: *mut libc::c_char,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
    #[no_mangle]
    fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const libc::c_void;
    #[no_mangle]
    fn CFDictionaryCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        keyCallBacks: *const CFDictionaryKeyCallBacks,
        valueCallBacks: *const CFDictionaryValueCallBacks,
    ) -> CFMutableDictionaryRef;
    #[no_mangle]
    fn CTRunGetTypographicBounds(
        run: CTRunRef,
        range: CFRange,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> libc::c_double;
    #[no_mangle]
    fn CTFontCreateCopyWithAttributes(
        font: CTFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    #[no_mangle]
    fn CFStringCompare(
        theString1: CFStringRef,
        theString2: CFStringRef,
        compareOptions: CFStringCompareFlags,
    ) -> CFComparisonResult;
    #[no_mangle]
    fn CFArrayAppendValue(theArray: CFMutableArrayRef, value: *const libc::c_void);
    #[no_mangle]
    fn CTFontCopyAttribute(font: CTFontRef, attribute: CFStringRef) -> CFTypeRef;
    #[no_mangle]
    fn CFDictionaryGetValue(
        theDict: CFDictionaryRef,
        key: *const libc::c_void,
    ) -> *const libc::c_void;
    #[no_mangle]
    static kCFAllocatorDefault: CFAllocatorRef;
    #[no_mangle]
    static kCFAllocatorNull: CFAllocatorRef;
    #[no_mangle]
    fn CFDictionaryAddValue(
        theDict: CFMutableDictionaryRef,
        key: *const libc::c_void,
        value: *const libc::c_void,
    );
    #[no_mangle]
    fn CTFontCopyName(font: CTFontRef, nameKey: CFStringRef) -> CFStringRef;
    #[no_mangle]
    fn CFRelease(cf: CFTypeRef);
    #[no_mangle]
    fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean;
    #[no_mangle]
    fn CTFontGetGlyphsForCharacters(
        font: CTFontRef,
        characters: *const UniChar,
        glyphs: *mut CGGlyph,
        count: CFIndex,
    ) -> bool;
    #[no_mangle]
    fn CTFontGetGlyphWithName(font: CTFontRef, glyphName: CFStringRef) -> CGGlyph;
    #[no_mangle]
    fn CTFontGetBoundingRectsForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        boundingRects: *mut CGRect,
        count: CFIndex,
    ) -> CGRect;
    #[no_mangle]
    fn CTFontGetAdvancesForGlyphs(
        font: CTFontRef,
        orientation: CTFontOrientation,
        glyphs: *const CGGlyph,
        advances: *mut CGSize,
        count: CFIndex,
    ) -> libc::c_double;
    #[no_mangle]
    static kCTFontFeatureTypeIdentifierKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureTypeNameKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureTypeSelectorsKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureSelectorIdentifierKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureSelectorNameKey: CFStringRef;
    #[no_mangle]
    fn CTFontCopyFeatures(font: CTFontRef) -> CFArrayRef;
    #[no_mangle]
    fn CTFontCopyGraphicsFont(font: CTFontRef, attributes: *mut CTFontDescriptorRef) -> CGFontRef;
    #[no_mangle]
    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetGlyphCount(font: CTFontRef) -> CFIndex;
    #[no_mangle]
    fn CTFontGetSlantAngle(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetCapHeight(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;
    #[no_mangle]
    fn CFDictionaryGetValueIfPresent(
        theDict: CFDictionaryRef,
        key: *const libc::c_void,
        value: *mut *const libc::c_void,
    ) -> Boolean;
    #[no_mangle]
    static kCTFontFeatureTypeExclusiveKey: CFStringRef;
    #[no_mangle]
    static kCTFontFeatureSelectorDefaultKey: CFStringRef;
    #[no_mangle]
    fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
    #[no_mangle]
    fn CFStringGetCharacters(theString: CFStringRef, range: CFRange, buffer: *mut UniChar);
    
}

use super::xetex_ext::{print_chars, name_of_file, name_length, xcalloc};

pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type int32_t = libc::c_int;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type CFStringRef = *const __CFString;
pub type CFAllocatorRef = *const __CFAllocator;
pub type UniChar = UInt16;
pub type UInt16 = libc::c_ushort;
pub type CFURLRef = *const __CFURL;
pub type CFDictionaryRef = *const __CFDictionary;
pub type CGFloat = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}
pub type CFNumberRef = *const __CFNumber;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}
pub type Boolean = libc::c_uchar;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_MemoryRec_ {
    pub user: *mut libc::c_void,
    pub alloc: FT_Alloc_Func,
    pub free: FT_Free_Func,
    pub realloc: FT_Realloc_Func,
}
pub type FT_Realloc_Func = Option<
    unsafe extern "C" fn(
        _: FT_Memory,
        _: libc::c_long,
        _: libc::c_long,
        _: *mut libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type FT_Memory = *mut FT_MemoryRec_;
pub type FT_Free_Func = Option<unsafe extern "C" fn(_: FT_Memory, _: *mut libc::c_void) -> ()>;
pub type FT_Alloc_Func =
    Option<unsafe extern "C" fn(_: FT_Memory, _: libc::c_long) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_StreamRec_ {
    pub base: *mut libc::c_uchar,
    pub size: libc::c_ulong,
    pub pos: libc::c_ulong,
    pub descriptor: FT_StreamDesc,
    pub pathname: FT_StreamDesc,
    pub read: FT_Stream_IoFunc,
    pub close: FT_Stream_CloseFunc,
    pub memory: FT_Memory,
    pub cursor: *mut libc::c_uchar,
    pub limit: *mut libc::c_uchar,
}
pub type FT_Stream_CloseFunc = Option<unsafe extern "C" fn(_: FT_Stream) -> ()>;
pub type FT_Stream = *mut FT_StreamRec_;
pub type FT_Stream_IoFunc = Option<
    unsafe extern "C" fn(
        _: FT_Stream,
        _: libc::c_ulong,
        _: *mut libc::c_uchar,
        _: libc::c_ulong,
    ) -> libc::c_ulong,
>;
pub type FT_StreamDesc = FT_StreamDesc_;
#[derive(Copy, Clone)]
#[repr(C)]
pub union FT_StreamDesc_ {
    pub value: libc::c_long,
    pub pointer: *mut libc::c_void,
}
pub type FT_Pos = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Vector_ {
    pub x: FT_Pos,
    pub y: FT_Pos,
}
pub type FT_Vector = FT_Vector_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_BBox_ {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}
pub type FT_BBox = FT_BBox_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Bitmap_ {
    pub rows: libc::c_uint,
    pub width: libc::c_uint,
    pub pitch: libc::c_int,
    pub buffer: *mut libc::c_uchar,
    pub num_grays: libc::c_ushort,
    pub pixel_mode: libc::c_uchar,
    pub palette_mode: libc::c_uchar,
    pub palette: *mut libc::c_void,
}
pub type FT_Bitmap = FT_Bitmap_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Outline_ {
    pub n_contours: libc::c_short,
    pub n_points: libc::c_short,
    pub points: *mut FT_Vector,
    pub tags: *mut libc::c_char,
    pub contours: *mut libc::c_short,
    pub flags: libc::c_int,
}
pub type FT_Outline = FT_Outline_;
pub type FT_Glyph_Format_ = libc::c_uint;
pub const FT_GLYPH_FORMAT_PLOTTER: FT_Glyph_Format_ = 1886154612;
pub const FT_GLYPH_FORMAT_OUTLINE: FT_Glyph_Format_ = 1869968492;
pub const FT_GLYPH_FORMAT_BITMAP: FT_Glyph_Format_ = 1651078259;
pub const FT_GLYPH_FORMAT_COMPOSITE: FT_Glyph_Format_ = 1668246896;
pub const FT_GLYPH_FORMAT_NONE: FT_Glyph_Format_ = 0;
pub type FT_Glyph_Format = FT_Glyph_Format_;
pub type FT_String = libc::c_char;
pub type FT_Short = libc::c_short;
pub type FT_UShort = libc::c_ushort;
pub type FT_Int = libc::c_int;
pub type FT_UInt = libc::c_uint;
pub type FT_Long = libc::c_long;
pub type FT_Fixed = libc::c_long;
pub type FT_Error = libc::c_int;
pub type FT_Generic_Finalizer = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Generic_ {
    pub data: *mut libc::c_void,
    pub finalizer: FT_Generic_Finalizer,
}
pub type FT_Generic = FT_Generic_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_ListNodeRec_ {
    pub prev: FT_ListNode,
    pub next: FT_ListNode,
    pub data: *mut libc::c_void,
}
pub type FT_ListNode = *mut FT_ListNodeRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_ListRec_ {
    pub head: FT_ListNode,
    pub tail: FT_ListNode,
}
pub type FT_ListRec = FT_ListRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Glyph_Metrics_ {
    pub width: FT_Pos,
    pub height: FT_Pos,
    pub horiBearingX: FT_Pos,
    pub horiBearingY: FT_Pos,
    pub horiAdvance: FT_Pos,
    pub vertBearingX: FT_Pos,
    pub vertBearingY: FT_Pos,
    pub vertAdvance: FT_Pos,
}
pub type FT_Glyph_Metrics = FT_Glyph_Metrics_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Bitmap_Size_ {
    pub height: FT_Short,
    pub width: FT_Short,
    pub size: FT_Pos,
    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos,
}
pub type FT_Bitmap_Size = FT_Bitmap_Size_;
pub type FT_Library = *mut FT_LibraryRec_;
pub type FT_Driver = *mut FT_DriverRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_FaceRec_ {
    pub num_faces: FT_Long,
    pub face_index: FT_Long,
    pub face_flags: FT_Long,
    pub style_flags: FT_Long,
    pub num_glyphs: FT_Long,
    pub family_name: *mut FT_String,
    pub style_name: *mut FT_String,
    pub num_fixed_sizes: FT_Int,
    pub available_sizes: *mut FT_Bitmap_Size,
    pub num_charmaps: FT_Int,
    pub charmaps: *mut FT_CharMap,
    pub generic: FT_Generic,
    pub bbox: FT_BBox,
    pub units_per_EM: FT_UShort,
    pub ascender: FT_Short,
    pub descender: FT_Short,
    pub height: FT_Short,
    pub max_advance_width: FT_Short,
    pub max_advance_height: FT_Short,
    pub underline_position: FT_Short,
    pub underline_thickness: FT_Short,
    pub glyph: FT_GlyphSlot,
    pub size: FT_Size,
    pub charmap: FT_CharMap,
    pub driver: FT_Driver,
    pub memory: FT_Memory,
    pub stream: FT_Stream,
    pub sizes_list: FT_ListRec,
    pub autohint: FT_Generic,
    pub extensions: *mut libc::c_void,
    pub internal: FT_Face_Internal,
}
pub type FT_Face_Internal = *mut FT_Face_InternalRec_;
pub type FT_CharMap = *mut FT_CharMapRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_CharMapRec_ {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
}
pub type FT_Encoding = FT_Encoding_;
pub type FT_Encoding_ = libc::c_uint;
pub const FT_ENCODING_APPLE_ROMAN: FT_Encoding_ = 1634889070;
pub const FT_ENCODING_OLD_LATIN_2: FT_Encoding_ = 1818326066;
pub const FT_ENCODING_ADOBE_LATIN_1: FT_Encoding_ = 1818326065;
pub const FT_ENCODING_ADOBE_CUSTOM: FT_Encoding_ = 1094992451;
pub const FT_ENCODING_ADOBE_EXPERT: FT_Encoding_ = 1094992453;
pub const FT_ENCODING_ADOBE_STANDARD: FT_Encoding_ = 1094995778;
pub const FT_ENCODING_MS_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_MS_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_MS_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_MS_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_MS_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_PRC: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_UNICODE: FT_Encoding_ = 1970170211;
pub const FT_ENCODING_MS_SYMBOL: FT_Encoding_ = 1937337698;
pub const FT_ENCODING_NONE: FT_Encoding_ = 0;
pub type FT_Face = *mut FT_FaceRec_;
pub type FT_Size = *mut FT_SizeRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_SizeRec_ {
    pub face: FT_Face,
    pub generic: FT_Generic,
    pub metrics: FT_Size_Metrics,
    pub internal: FT_Size_Internal,
}
pub type FT_Size_Internal = *mut FT_Size_InternalRec_;
pub type FT_Size_Metrics = FT_Size_Metrics_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Size_Metrics_ {
    pub x_ppem: FT_UShort,
    pub y_ppem: FT_UShort,
    pub x_scale: FT_Fixed,
    pub y_scale: FT_Fixed,
    pub ascender: FT_Pos,
    pub descender: FT_Pos,
    pub height: FT_Pos,
    pub max_advance: FT_Pos,
}
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_GlyphSlotRec_ {
    pub library: FT_Library,
    pub face: FT_Face,
    pub next: FT_GlyphSlot,
    pub glyph_index: FT_UInt,
    pub generic: FT_Generic,
    pub metrics: FT_Glyph_Metrics,
    pub linearHoriAdvance: FT_Fixed,
    pub linearVertAdvance: FT_Fixed,
    pub advance: FT_Vector,
    pub format: FT_Glyph_Format,
    pub bitmap: FT_Bitmap,
    pub bitmap_left: FT_Int,
    pub bitmap_top: FT_Int,
    pub outline: FT_Outline,
    pub num_subglyphs: FT_UInt,
    pub subglyphs: FT_SubGlyph,
    pub control_data: *mut libc::c_void,
    pub control_len: libc::c_long,
    pub lsb_delta: FT_Pos,
    pub rsb_delta: FT_Pos,
    pub other: *mut libc::c_void,
    pub internal: FT_Slot_Internal,
}
pub type FT_Slot_Internal = *mut FT_Slot_InternalRec_;
pub type FT_SubGlyph = *mut FT_SubGlyphRec_;

pub type scaled_t = int32_t;
pub type UInt8 = libc::c_uchar;
pub type UInt32 = libc::c_uint;
pub type SInt32 = libc::c_int;
pub type Fixed = SInt32;
pub type Fract = SInt32;
#[derive(Copy, Clone)]
#[repr(C, packed(2))]
pub struct FixedPoint {
    pub x: Fixed,
    pub y: Fixed,
}
pub type CFOptionFlags = libc::c_ulong;
pub type CFHashCode = libc::c_ulong;
pub type CFIndex = libc::c_long;
pub type CFTypeRef = *const libc::c_void;
pub type CFComparisonResult = CFIndex;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}
pub type CFArrayRetainCallBack =
    Option<unsafe extern "C" fn(_: CFAllocatorRef, _: *const libc::c_void) -> *const libc::c_void>;
pub type CFArrayReleaseCallBack =
    Option<unsafe extern "C" fn(_: CFAllocatorRef, _: *const libc::c_void) -> ()>;
pub type CFArrayCopyDescriptionCallBack =
    Option<unsafe extern "C" fn(_: *const libc::c_void) -> CFStringRef>;
pub type CFArrayEqualCallBack =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> Boolean>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFArrayCallBacks {
    pub version: CFIndex,
    pub retain: CFArrayRetainCallBack,
    pub release: CFArrayReleaseCallBack,
    pub copyDescription: CFArrayCopyDescriptionCallBack,
    pub equal: CFArrayEqualCallBack,
}
pub type CFArrayRef = *const __CFArray;
pub type CFMutableArrayRef = *mut __CFArray;
pub type CFDictionaryRetainCallBack =
    Option<unsafe extern "C" fn(_: CFAllocatorRef, _: *const libc::c_void) -> *const libc::c_void>;
pub type CFDictionaryReleaseCallBack =
    Option<unsafe extern "C" fn(_: CFAllocatorRef, _: *const libc::c_void) -> ()>;
pub type CFDictionaryCopyDescriptionCallBack =
    Option<unsafe extern "C" fn(_: *const libc::c_void) -> CFStringRef>;
pub type CFDictionaryEqualCallBack =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> Boolean>;
pub type CFDictionaryHashCallBack =
    Option<unsafe extern "C" fn(_: *const libc::c_void) -> CFHashCode>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFDictionaryKeyCallBacks {
    pub version: CFIndex,
    pub retain: CFDictionaryRetainCallBack,
    pub release: CFDictionaryReleaseCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal: CFDictionaryEqualCallBack,
    pub hash: CFDictionaryHashCallBack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFDictionaryValueCallBacks {
    pub version: CFIndex,
    pub retain: CFDictionaryRetainCallBack,
    pub release: CFDictionaryReleaseCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal: CFDictionaryEqualCallBack,
}
pub type CFMutableDictionaryRef = *mut __CFDictionary;
pub type CFStringEncoding = UInt32;
pub type C2RustUnnamed = libc::c_uint;
pub const kCFStringEncodingUTF32LE: C2RustUnnamed = 469762304;
pub const kCFStringEncodingUTF32BE: C2RustUnnamed = 402653440;
pub const kCFStringEncodingUTF32: C2RustUnnamed = 201326848;
pub const kCFStringEncodingUTF16LE: C2RustUnnamed = 335544576;
pub const kCFStringEncodingUTF16BE: C2RustUnnamed = 268435712;
pub const kCFStringEncodingUTF16: C2RustUnnamed = 256;
pub const kCFStringEncodingNonLossyASCII: C2RustUnnamed = 3071;
pub const kCFStringEncodingUTF8: C2RustUnnamed = 134217984;
pub const kCFStringEncodingUnicode: C2RustUnnamed = 256;
pub const kCFStringEncodingASCII: C2RustUnnamed = 1536;
pub const kCFStringEncodingNextStepLatin: C2RustUnnamed = 2817;
pub const kCFStringEncodingISOLatin1: C2RustUnnamed = 513;
pub const kCFStringEncodingWindowsLatin1: C2RustUnnamed = 1280;
pub const kCFStringEncodingMacRoman: C2RustUnnamed = 0;
pub type CFStringCompareFlags = CFOptionFlags;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const kCFCompareForcedOrdering: C2RustUnnamed_0 = 512;
pub const kCFCompareWidthInsensitive: C2RustUnnamed_0 = 256;
pub const kCFCompareDiacriticInsensitive: C2RustUnnamed_0 = 128;
pub const kCFCompareNumerically: C2RustUnnamed_0 = 64;
pub const kCFCompareLocalized: C2RustUnnamed_0 = 32;
pub const kCFCompareNonliteral: C2RustUnnamed_0 = 16;
pub const kCFCompareAnchored: C2RustUnnamed_0 = 8;
pub const kCFCompareBackwards: C2RustUnnamed_0 = 4;
pub const kCFCompareCaseInsensitive: C2RustUnnamed_0 = 1;
pub type CFBooleanRef = *const __CFBoolean;
pub type CFNumberType = CFIndex;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const kCFNumberMaxType: C2RustUnnamed_1 = 16;
pub const kCFNumberCGFloatType: C2RustUnnamed_1 = 16;
pub const kCFNumberNSIntegerType: C2RustUnnamed_1 = 15;
pub const kCFNumberCFIndexType: C2RustUnnamed_1 = 14;
pub const kCFNumberDoubleType: C2RustUnnamed_1 = 13;
pub const kCFNumberFloatType: C2RustUnnamed_1 = 12;
pub const kCFNumberLongLongType: C2RustUnnamed_1 = 11;
pub const kCFNumberLongType: C2RustUnnamed_1 = 10;
pub const kCFNumberIntType: C2RustUnnamed_1 = 9;
pub const kCFNumberShortType: C2RustUnnamed_1 = 8;
pub const kCFNumberCharType: C2RustUnnamed_1 = 7;
pub const kCFNumberFloat64Type: C2RustUnnamed_1 = 6;
pub const kCFNumberFloat32Type: C2RustUnnamed_1 = 5;
pub const kCFNumberSInt64Type: C2RustUnnamed_1 = 4;
pub const kCFNumberSInt32Type: C2RustUnnamed_1 = 3;
pub const kCFNumberSInt16Type: C2RustUnnamed_1 = 2;
pub const kCFNumberSInt8Type: C2RustUnnamed_1 = 1;
pub type CFAttributedStringRef = *const __CFAttributedString;
pub type CGColorRef = *mut CGColor;
pub type CGFontRef = *mut CGFont;
pub type CGFontIndex = libc::c_ushort;
pub type CGGlyph = CGFontIndex;
pub type CTFontDescriptorRef = *const __CTFontDescriptor;
pub type CTFontOrientation = uint32_t;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const kCTFontVerticalOrientation: C2RustUnnamed_2 = 2;
pub const kCTFontHorizontalOrientation: C2RustUnnamed_2 = 1;
pub const kCTFontDefaultOrientation: C2RustUnnamed_2 = 0;
pub const kCTFontOrientationVertical: C2RustUnnamed_2 = 2;
pub const kCTFontOrientationHorizontal: C2RustUnnamed_2 = 1;
pub const kCTFontOrientationDefault: C2RustUnnamed_2 = 0;
pub type CTFontRef = *const __CTFont;
pub type CTLineRef = *const __CTLine;
pub type CTTypesetterRef = *const __CTTypesetter;
pub type CTRunRef = *const __CTRun;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphBBox {
    pub xMin: libc::c_float,
    pub yMin: libc::c_float,
    pub xMax: libc::c_float,
    pub yMax: libc::c_float,
}
/* The annoying `memory_word` type. We have to make sure the byte-swapping
 * that the (un)dumping routines do suffices to put things in the right place
 * in memory.
 *
 * This set of data used to be a huge mess (see comment after the
 * definitions). It is now (IMO) a lot more reasonable, but there will no
 * doubt be carryover weird terminology around the code.
 *
 * ## ENDIANNESS (cheat sheet because I'm lame)
 *
 * Intel is little-endian. Say that we have a 32-bit integer stored in memory
 * with `p` being a `uint8` pointer to its location. In little-endian land,
 * `p[0]` is least significant byte and `p[3]` is its most significant byte.
 *
 * Conversely, in big-endian land, `p[0]` is its most significant byte and
 * `p[3]` is its least significant byte.
 *
 * ## MEMORY_WORD LAYOUT
 *
 * Little endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [lsb......s0.......msb] [lsb......s1.......msb]
 *   b16:   [l..s0...m] [l..s1...m] [l..s2...m] [l..s3...m]
 *
 * Big endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [msb......s1.......lsb] [msb......s0.......lsb]
 *   b16:   [m..s3...l] [m..s2...l] [m..s1...l] [m...s0..l]
 *
 */
pub type b32x2 = b32x2_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: int32_t,
    pub s1: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: libc::c_double,
    pub ptr: *mut libc::c_void,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4_le_t {
    pub s0: uint16_t,
    pub s1: uint16_t,
    pub s2: uint16_t,
    pub s3: uint16_t,
}
pub type str_number = int32_t;
/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#[inline]
unsafe extern "C" fn strstartswith(
    mut s: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> *const libc::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    return 0 as *const libc::c_char;
}
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const libc::c_char, mut s2: *const libc::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
/* ***************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
 Copyright (c) 2012, 2013 by Jiang Jiang
 Copyright (c) 2012-2015 by Khaled Hosny

 SIL Author(s): Jonathan Kew

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the copyright holders
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in this Software without prior written
authorization from the copyright holders.
\****************************************************************************/
/* XeTeX_mac.c
 * additional plain C extensions for XeTeX - MacOS-specific routines
 */
#[inline]
unsafe extern "C" fn TeXtoPSPoints(mut pts: libc::c_double) -> libc::c_double {
    return pts * 72.0f64 / 72.27f64;
}
#[inline]
unsafe extern "C" fn PStoTeXPoints(mut pts: libc::c_double) -> libc::c_double {
    return pts * 72.27f64 / 72.0f64;
}
#[inline]
unsafe extern "C" fn FixedPStoTeXPoints(mut pts: libc::c_double) -> Fixed {
    return D2Fix(PStoTeXPoints(pts));
}
#[no_mangle]
pub unsafe extern "C" fn fontFromAttributes(mut attributes: CFDictionaryRef) -> CTFontRef {
    return CFDictionaryGetValue(attributes, kCTFontAttributeName as *const libc::c_void)
        as CTFontRef;
}
#[no_mangle]
pub unsafe extern "C" fn fontFromInteger(mut font: int32_t) -> CTFontRef {
    let mut attributes: CFDictionaryRef =
        *font_layout_engine.offset(font as isize) as CFDictionaryRef;
    return fontFromAttributes(attributes);
}

#[no_mangle]
pub unsafe extern "C" fn DoAATLayout(mut p: *mut libc::c_void, mut justify: libc::c_int) {
    let mut glyphRuns: CFArrayRef = 0 as *const __CFArray;
    let mut i: CFIndex = 0;
    let mut j: CFIndex = 0;
    let mut runCount: CFIndex = 0;
    let mut totalGlyphCount: CFIndex = 0i32 as CFIndex;
    let mut glyphIDs: *mut UInt16 = 0 as *mut UInt16;
    let mut glyphAdvances: *mut Fixed = 0 as *mut Fixed;
    let mut glyph_info: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut locations: *mut FixedPoint = 0 as *mut FixedPoint;
    let mut width: CGFloat = 0.;
    let mut txtLen: libc::c_long = 0;
    let mut txtPtr: *const UniChar = 0 as *const UniChar;
    let mut attributes: CFDictionaryRef = 0 as *const __CFDictionary;
    let mut string: CFStringRef = 0 as *const __CFString;
    let mut attrString: CFAttributedStringRef = 0 as *const __CFAttributedString;
    let mut typesetter: CTTypesetterRef = 0 as *const __CTTypesetter;
    let mut line: CTLineRef = 0 as *const __CTLine;
    let mut node: *mut memory_word = p as *mut memory_word;
    let mut f: libc::c_uint = (*node.offset(4)).b16.s2 as libc::c_uint;
    if *font_area.offset(f as isize) as libc::c_uint != 0xffffu32 {
        panic!("DoAATLayout called for non-AAT font");
    }
    txtLen = (*node.offset(4)).b16.s1 as libc::c_long;
    txtPtr = node.offset(6) as *mut UniChar;
    attributes = *font_layout_engine.offset((*node.offset(4)).b16.s2 as isize) as CFDictionaryRef;
    string =
        CFStringCreateWithCharactersNoCopy(0 as CFAllocatorRef, txtPtr, txtLen, kCFAllocatorNull);
    attrString = CFAttributedStringCreate(0 as CFAllocatorRef, string, attributes);
    CFRelease(string as CFTypeRef);
    typesetter = CTTypesetterCreateWithAttributedString(attrString);
    CFRelease(attrString as CFTypeRef);
    line = CTTypesetterCreateLine(typesetter, CFRangeMake(0i32 as CFIndex, txtLen));
    if justify != 0 {
        let mut lineWidth: CGFloat = TeXtoPSPoints(Fix2D((*node.offset(1)).b32.s1));
        let mut justifiedLine: CTLineRef = CTLineCreateJustifiedLine(
            line,
            TeXtoPSPoints(Fix2D(0x40000000i64 as Fract)),
            lineWidth,
        );
        // TODO(jjgod): how to handle the case when justification failed? for
        // now we just fallback to use the original line.
        if !justifiedLine.is_null() {
            CFRelease(line as CFTypeRef);
            line = justifiedLine
        }
    }
    glyphRuns = CTLineGetGlyphRuns(line);
    runCount = CFArrayGetCount(glyphRuns);
    totalGlyphCount = CTLineGetGlyphCount(line);
    if totalGlyphCount > 0i32 as libc::c_long {
        glyph_info = xmalloc((totalGlyphCount * 10i32 as libc::c_long) as size_t);
        locations = glyph_info as *mut FixedPoint;
        glyphIDs = locations.offset(totalGlyphCount as isize) as *mut UInt16;
        glyphAdvances = xmalloc(
            (totalGlyphCount as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<Fixed>() as libc::c_ulong),
        ) as *mut Fixed;
        totalGlyphCount = 0i32 as CFIndex;
        width = 0i32 as CGFloat;
        i = 0i32 as CFIndex;
        while i < runCount {
            let mut run: CTRunRef = CFArrayGetValueAtIndex(glyphRuns, i) as CTRunRef;
            let mut count: CFIndex = CTRunGetGlyphCount(run);
            let mut runAttributes: CFDictionaryRef = CTRunGetAttributes(run);
            let mut vertical: CFBooleanRef = CFDictionaryGetValue(
                runAttributes,
                kCTVerticalFormsAttributeName as *const libc::c_void,
            ) as CFBooleanRef;
            // TODO(jjgod): Avoid unnecessary allocation with CTRunGetFoosPtr().
            let mut glyphs: *mut CGGlyph = xmalloc(
                (count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<CGGlyph>() as libc::c_ulong),
            ) as *mut CGGlyph;
            let mut positions: *mut CGPoint = xmalloc(
                (count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<CGPoint>() as libc::c_ulong),
            ) as *mut CGPoint;
            let mut advances: *mut CGSize = xmalloc(
                (count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<CGSize>() as libc::c_ulong),
            ) as *mut CGSize;
            let mut runWidth: CGFloat = CTRunGetTypographicBounds(
                run,
                CFRangeMake(0i32 as CFIndex, 0i32 as CFIndex),
                0 as *mut CGFloat,
                0 as *mut CGFloat,
                0 as *mut CGFloat,
            );
            CTRunGetGlyphs(run, CFRangeMake(0i32 as CFIndex, 0i32 as CFIndex), glyphs);
            CTRunGetPositions(
                run,
                CFRangeMake(0i32 as CFIndex, 0i32 as CFIndex),
                positions,
            );
            CTRunGetAdvances(run, CFRangeMake(0i32 as CFIndex, 0i32 as CFIndex), advances);
            j = 0i32 as CFIndex;
            while j < count {
                // XXX Core Text has that font cascading thing that will do
                // font substitution for missing glyphs, which we do not want
                // but I can not find a way to disable it yet, so if the font
                // of the resulting run is not the same font we asked for, use
                // the glyph at index 0 (usually .notdef) instead or we will be
                // showing garbage or even invalid glyphs
                if CFEqual(
                    fontFromAttributes(attributes) as CFTypeRef,
                    fontFromAttributes(runAttributes) as CFTypeRef,
                ) == 0
                {
                    *glyphIDs.offset(totalGlyphCount as isize) = 0i32 as UInt16
                } else {
                    *glyphIDs.offset(totalGlyphCount as isize) = *glyphs.offset(j as isize)
                }
                // Swap X and Y when doing vertical layout
                if vertical == kCFBooleanTrue {
                    (*locations.offset(totalGlyphCount as isize)).x =
                        -FixedPStoTeXPoints((*positions.offset(j as isize)).y);
                    (*locations.offset(totalGlyphCount as isize)).y =
                        FixedPStoTeXPoints((*positions.offset(j as isize)).x)
                } else {
                    (*locations.offset(totalGlyphCount as isize)).x =
                        FixedPStoTeXPoints((*positions.offset(j as isize)).x);
                    (*locations.offset(totalGlyphCount as isize)).y =
                        -FixedPStoTeXPoints((*positions.offset(j as isize)).y)
                }
                *glyphAdvances.offset(totalGlyphCount as isize) =
                    (*advances.offset(j as isize)).width as Fixed;
                totalGlyphCount += 1;
                j += 1
            }
            width += FixedPStoTeXPoints(runWidth) as libc::c_double;
            free(glyphs as *mut libc::c_void);
            free(positions as *mut libc::c_void);
            free(advances as *mut libc::c_void);
            i += 1
        }
    }
    (*node.offset(4)).b16.s0 = totalGlyphCount as uint16_t;
    let ref mut fresh0 = (*node.offset(5)).ptr;
    *fresh0 = glyph_info;
    if justify == 0 {
        (*node.offset(1)).b32.s1 = width as int32_t;
        if totalGlyphCount > 0i32 as libc::c_long {
            /* this is essentially a copy from similar code in XeTeX_ext.c, easier
             * to be done here */
            if *font_letter_space.offset(f as isize) != 0i32 {
                let mut lsDelta: Fixed = 0i32;
                let mut lsUnit: Fixed = *font_letter_space.offset(f as isize);
                let mut i_0: libc::c_int = 0;
                i_0 = 0i32;
                while (i_0 as libc::c_long) < totalGlyphCount {
                    if *glyphAdvances.offset(i_0 as isize) == 0i32 && lsDelta != 0i32 {
                        lsDelta -= lsUnit
                    }
                    let ref mut fresh1 = (*locations.offset(i_0 as isize)).x;
                    *fresh1 += lsDelta;
                    lsDelta += lsUnit;
                    i_0 += 1
                }
                if lsDelta != 0i32 {
                    lsDelta -= lsUnit;
                    let ref mut fresh2 = (*node.offset(1)).b32.s1;
                    *fresh2 += lsDelta
                }
            }
        }
    }
    free(glyphAdvances as *mut libc::c_void);
    CFRelease(line as CFTypeRef);
    CFRelease(typesetter as CFTypeRef);
}
#[inline]
unsafe extern "C" fn __CGAffineTransformMake(
    mut a: CGFloat,
    mut b: CGFloat,
    mut c: CGFloat,
    mut d: CGFloat,
    mut tx: CGFloat,
    mut ty: CGFloat,
) -> CGAffineTransform {
    let mut t: CGAffineTransform = CGAffineTransform {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        tx: 0.,
        ty: 0.,
    };
    t.a = a;
    t.b = b;
    t.c = c;
    t.d = d;
    t.tx = tx;
    t.ty = ty;
    return t;
}
unsafe extern "C" fn getGlyphBBoxFromCTFont(
    mut font: CTFontRef,
    mut gid: UInt16,
    mut bbox: *mut GlyphBBox,
) {
    let mut rect: CGRect = CGRect {
        origin: CGPoint { x: 0., y: 0. },
        size: CGSize {
            width: 0.,
            height: 0.,
        },
    };
    (*bbox).xMin = 65536.0f64 as libc::c_float;
    (*bbox).yMin = 65536.0f64 as libc::c_float;
    (*bbox).xMax = -65536.0f64 as libc::c_float;
    (*bbox).yMax = -65536.0f64 as libc::c_float;
    rect = CTFontGetBoundingRectsForGlyphs(
        font,
        0i32 as CTFontOrientation,
        &mut gid as *mut UInt16 as *const CGGlyph,
        0 as *mut CGRect,
        1i32 as CFIndex,
    );
    if CGRectIsNull(rect) {
        (*bbox).yMax = 0i32 as libc::c_float;
        (*bbox).xMax = (*bbox).yMax;
        (*bbox).yMin = (*bbox).xMax;
        (*bbox).xMin = (*bbox).yMin
    } else {
        (*bbox).yMin = PStoTeXPoints(rect.origin.y) as libc::c_float;
        (*bbox).yMax = PStoTeXPoints(rect.origin.y + rect.size.height) as libc::c_float;
        (*bbox).xMin = PStoTeXPoints(rect.origin.x) as libc::c_float;
        (*bbox).xMax = PStoTeXPoints(rect.origin.x + rect.size.width) as libc::c_float
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetGlyphBBox_AAT(
    mut attributes: CFDictionaryRef,
    mut gid: UInt16,
    mut bbox: *mut GlyphBBox,
)
/* returns glyph bounding box in TeX points */
{
    let mut font: CTFontRef = fontFromAttributes(attributes);
    return getGlyphBBoxFromCTFont(font, gid, bbox);
}
unsafe extern "C" fn getGlyphWidthFromCTFont(
    mut font: CTFontRef,
    mut gid: UInt16,
) -> libc::c_double {
    return PStoTeXPoints(CTFontGetAdvancesForGlyphs(
        font,
        kCTFontOrientationHorizontal as libc::c_int as CTFontOrientation,
        &mut gid as *mut UInt16 as *const CGGlyph,
        0 as *mut CGSize,
        1i32 as CFIndex,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn GetGlyphWidth_AAT(
    mut attributes: CFDictionaryRef,
    mut gid: UInt16,
) -> libc::c_double
/* returns TeX points */ {
    let mut font: CTFontRef = fontFromAttributes(attributes);
    return getGlyphWidthFromCTFont(font, gid);
}
#[no_mangle]
pub unsafe extern "C" fn GetGlyphHeightDepth_AAT(
    mut attributes: CFDictionaryRef,
    mut gid: UInt16,
    mut ht: *mut libc::c_float,
    mut dp: *mut libc::c_float,
)
/* returns TeX points */
{
    let mut bbox: GlyphBBox = GlyphBBox {
        xMin: 0.,
        yMin: 0.,
        xMax: 0.,
        yMax: 0.,
    };
    GetGlyphBBox_AAT(attributes, gid, &mut bbox);
    *ht = bbox.yMax;
    *dp = -bbox.yMin;
}
#[no_mangle]
pub unsafe extern "C" fn GetGlyphSidebearings_AAT(
    mut attributes: CFDictionaryRef,
    mut gid: UInt16,
    mut lsb: *mut libc::c_float,
    mut rsb: *mut libc::c_float,
)
/* returns TeX points */
{
    let mut font: CTFontRef = fontFromAttributes(attributes);
    let mut advances: [CGSize; 1] = [CGSizeMake(0i32 as CGFloat, 0i32 as CGFloat)];
    let mut advance: libc::c_double = CTFontGetAdvancesForGlyphs(
        font,
        0i32 as CTFontOrientation,
        &mut gid as *mut UInt16 as *const CGGlyph,
        advances.as_mut_ptr(),
        1i32 as CFIndex,
    );
    let mut bbox: GlyphBBox = GlyphBBox {
        xMin: 0.,
        yMin: 0.,
        xMax: 0.,
        yMax: 0.,
    };
    getGlyphBBoxFromCTFont(font, gid, &mut bbox);
    *lsb = bbox.xMin;
    *rsb = (PStoTeXPoints(advance) - bbox.xMax as libc::c_double) as libc::c_float;
}
#[inline]
unsafe extern "C" fn CGSizeMake(mut width: CGFloat, mut height: CGFloat) -> CGSize {
    let mut size: CGSize = CGSize {
        width: 0.,
        height: 0.,
    };
    size.width = width;
    size.height = height;
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn GetGlyphItalCorr_AAT(
    mut attributes: CFDictionaryRef,
    mut gid: UInt16,
) -> libc::c_double {
    let mut font: CTFontRef = fontFromAttributes(attributes);
    let mut advances: [CGSize; 1] = [CGSizeMake(0i32 as CGFloat, 0i32 as CGFloat)];
    let mut advance: libc::c_double = CTFontGetAdvancesForGlyphs(
        font,
        0i32 as CTFontOrientation,
        &mut gid as *mut UInt16 as *const CGGlyph,
        advances.as_mut_ptr(),
        1i32 as CFIndex,
    );
    let mut bbox: GlyphBBox = GlyphBBox {
        xMin: 0.,
        yMin: 0.,
        xMax: 0.,
        yMax: 0.,
    };
    getGlyphBBoxFromCTFont(font, gid, &mut bbox);
    if bbox.xMax as libc::c_double > PStoTeXPoints(advance) {
        return bbox.xMax as libc::c_double - PStoTeXPoints(advance);
    }
    return 0i32 as libc::c_double;
}
unsafe extern "C" fn mapCharToGlyphFromCTFont(mut font: CTFontRef, mut ch: UInt32) -> libc::c_int {
    let mut glyphs: [CGGlyph; 2] = [0i32 as CGGlyph, 0];
    let mut txt: [UniChar; 2] = [0; 2];
    let mut len: libc::c_int = 1i32;
    if ch > 0xffffi32 as libc::c_uint {
        ch = (ch as libc::c_uint).wrapping_sub(0x10000i32 as libc::c_uint) as UInt32 as UInt32;
        txt[0] = (0xd800i32 as libc::c_uint).wrapping_add(ch.wrapping_div(1024i32 as libc::c_uint))
            as UniChar;
        txt[1] = (0xdc00i32 as libc::c_uint).wrapping_add(ch.wrapping_rem(1024i32 as libc::c_uint))
            as UniChar;
        len = 2i32
    } else {
        txt[0] = ch as UniChar
    }
    if CTFontGetGlyphsForCharacters(
        font,
        txt.as_mut_ptr() as *const UniChar,
        glyphs.as_mut_ptr(),
        len as CFIndex,
    ) {
        return glyphs[0] as libc::c_int;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn MapCharToGlyph_AAT(
    mut attributes: CFDictionaryRef,
    mut ch: UInt32,
) -> libc::c_int {
    let mut font: CTFontRef = fontFromAttributes(attributes);
    return mapCharToGlyphFromCTFont(font, ch);
}
unsafe extern "C" fn GetGlyphIDFromCTFont(
    mut ctFontRef: CTFontRef,
    mut glyphName: *const libc::c_char,
) -> libc::c_int {
    let mut glyphname: CFStringRef = CFStringCreateWithCStringNoCopy(
        kCFAllocatorDefault,
        glyphName,
        kCFStringEncodingUTF8 as libc::c_int as CFStringEncoding,
        kCFAllocatorNull,
    );
    let mut rval: libc::c_int = CTFontGetGlyphWithName(ctFontRef, glyphname) as libc::c_int;
    CFRelease(glyphname as CFTypeRef);
    return rval;
}
/* single-purpose metrics accessors */
/* the metrics params here are really TeX 'scaled' (or MacOS 'Fixed') values, but that typedef isn't available every place this is included */
/* functions in XeTeX_mac.c */
#[no_mangle]
pub unsafe extern "C" fn MapGlyphToIndex_AAT(
    mut attributes: CFDictionaryRef,
    mut glyphName: *const libc::c_char,
) -> libc::c_int {
    let mut font: CTFontRef = fontFromAttributes(attributes);
    return GetGlyphIDFromCTFont(font, glyphName);
}
#[no_mangle]
pub unsafe extern "C" fn GetGlyphNameFromCTFont(
    mut ctFontRef: CTFontRef,
    mut gid: UInt16,
    mut len: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut cgfont: CGFontRef = 0 as *mut CGFont;
    static mut buffer: [libc::c_char; 256] = [0; 256];
    buffer[0] = 0i32 as libc::c_char;
    *len = 0i32;
    cgfont = CTFontCopyGraphicsFont(ctFontRef, 0 as *mut CTFontDescriptorRef);
    if !cgfont.is_null() && (gid as libc::c_ulong) < CGFontGetNumberOfGlyphs(cgfont) {
        let mut glyphname: CFStringRef = CGFontCopyGlyphNameForGlyph(cgfont, gid);
        if !glyphname.is_null() {
            if CFStringGetCString(
                glyphname,
                buffer.as_mut_ptr(),
                256i32 as CFIndex,
                kCFStringEncodingUTF8 as libc::c_int as CFStringEncoding,
            ) != 0
            {
                *len = strlen(buffer.as_mut_ptr()) as libc::c_int
            }
            CFRelease(glyphname as CFTypeRef);
        }
        CGFontRelease(cgfont);
    }
    return &mut *buffer.as_mut_ptr().offset(0) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn GetFontCharRange_AAT(
    mut attributes: CFDictionaryRef,
    mut reqFirst: libc::c_int,
) -> libc::c_int {
    if reqFirst != 0 {
        let mut ch: libc::c_int = 0i32;
        while MapCharToGlyph_AAT(attributes, ch as UInt32) == 0i32 && ch < 0x10ffffi32 {
            ch += 1
        }
        return ch;
    } else {
        let mut ch_0: libc::c_int = 0x10ffffi32;
        while MapCharToGlyph_AAT(attributes, ch_0 as UInt32) == 0i32 && ch_0 > 0i32 {
            ch_0 -= 1
        }
        return ch_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn getNameFromCTFont(
    mut ctFontRef: CTFontRef,
    mut nameKey: CFStringRef,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: CFStringRef = CTFontCopyName(ctFontRef, nameKey);
    let mut len: CFIndex = CFStringGetLength(name);
    len = len * 6i32 as libc::c_long + 1i32 as libc::c_long;
    buf = xmalloc(len as size_t) as *mut libc::c_char;
    if CFStringGetCString(
        name,
        buf,
        len,
        kCFStringEncodingUTF8 as libc::c_int as CFStringEncoding,
    ) != 0
    {
        return buf;
    }
    free(buf as *mut libc::c_void);
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getFileNameFromCTFont(
    mut ctFontRef: CTFontRef,
    mut index: *mut uint32_t,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut url: CFURLRef = 0 as CFURLRef;
    url = CTFontCopyAttribute(ctFontRef, kCTFontURLAttribute) as CFURLRef;
    if !url.is_null() {
        let mut pathname: [UInt8; 1024] = [0; 1024];
        if CFURLGetFileSystemRepresentation(
            url,
            1i32 as Boolean,
            pathname.as_mut_ptr(),
            1024i32 as CFIndex,
        ) != 0
        {
            let mut error: FT_Error = 0;
            let mut face: FT_Face = 0 as *mut FT_FaceRec_;
            *index = 0i32 as uint32_t;
            if gFreeTypeLibrary.is_null() {
                error = FT_Init_FreeType(&mut gFreeTypeLibrary);
                if error != 0 {
                    panic!("FreeType initialization failed; error {}\x00", error);
                }
            }
            error = FT_New_Face(
                gFreeTypeLibrary,
                pathname.as_mut_ptr() as *mut libc::c_char,
                0i32 as FT_Long,
                &mut face,
            );
            if error == 0 {
                if (*face).num_faces > 1i32 as libc::c_long {
                    let mut num_faces: libc::c_int = (*face).num_faces as libc::c_int;
                    let mut ps_name1: *mut libc::c_char =
                        getNameFromCTFont(ctFontRef, kCTFontPostScriptNameKey);
                    let mut i: libc::c_int = 0;
                    *index = -1i32 as uint32_t;
                    FT_Done_Face(face);
                    i = 0i32;
                    while i < num_faces {
                        error = FT_New_Face(
                            gFreeTypeLibrary,
                            pathname.as_mut_ptr() as *mut libc::c_char,
                            i as FT_Long,
                            &mut face,
                        );
                        if error == 0 {
                            let mut ps_name2: *const libc::c_char = FT_Get_Postscript_Name(face);
                            if streq_ptr(ps_name1, ps_name2) {
                                *index = i as uint32_t;
                                break;
                            } else {
                                FT_Done_Face(face);
                            }
                        }
                        i += 1
                    }
                    free(ps_name1 as *mut libc::c_void);
                }
            }
            if *index != -1i32 as libc::c_uint {
                ret = strdup(pathname.as_mut_ptr() as *mut libc::c_char)
            }
        }
        CFRelease(url as CFTypeRef);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn findDictionaryInArrayWithIdentifier(
    mut array: CFArrayRef,
    mut identifierKey: *const libc::c_void,
    mut identifier: libc::c_int,
) -> CFDictionaryRef {
    let mut dict: CFDictionaryRef = 0 as CFDictionaryRef;
    if !array.is_null() {
        let mut value: libc::c_int = -1i32;
        let mut i: CFIndex = 0;
        i = 0i32 as CFIndex;
        while i < CFArrayGetCount(array) {
            let mut item: CFDictionaryRef = CFArrayGetValueAtIndex(array, i) as CFDictionaryRef;
            let mut itemId: CFNumberRef = CFDictionaryGetValue(item, identifierKey) as CFNumberRef;
            if !itemId.is_null() {
                CFNumberGetValue(
                    itemId,
                    kCFNumberIntType as libc::c_int as CFNumberType,
                    &mut value as *mut libc::c_int as *mut libc::c_void,
                );
                if value == identifier {
                    dict = item;
                    break;
                }
            }
            i += 1
        }
    }
    return dict;
}
#[inline(always)]
unsafe extern "C" fn CFRangeMake(mut loc: CFIndex, mut len: CFIndex) -> CFRange {
    let mut range: CFRange = CFRange {
        location: 0,
        length: 0,
    };
    range.location = loc;
    range.length = len;
    return range;
}
#[no_mangle]
pub unsafe extern "C" fn findDictionaryInArray(
    mut array: CFArrayRef,
    mut nameKey: *const libc::c_void,
    mut name: *const libc::c_char,
    mut nameLength: libc::c_int,
) -> CFDictionaryRef {
    let mut dict: CFDictionaryRef = 0 as CFDictionaryRef;
    if !array.is_null() {
        let mut itemName: CFStringRef = 0 as *const __CFString;
        let mut i: CFIndex = 0;
        itemName = CFStringCreateWithBytes(
            0 as CFAllocatorRef,
            name as *mut UInt8,
            nameLength as CFIndex,
            kCFStringEncodingUTF8 as libc::c_int as CFStringEncoding,
            0i32 as Boolean,
        );
        i = 0i32 as CFIndex;
        while i < CFArrayGetCount(array) {
            let mut item: CFDictionaryRef = CFArrayGetValueAtIndex(array, i) as CFDictionaryRef;
            let mut iName: CFStringRef = CFDictionaryGetValue(item, nameKey) as CFStringRef;
            if !iName.is_null()
                && CFStringCompare(
                    itemName,
                    iName,
                    kCFCompareCaseInsensitive as libc::c_int as CFStringCompareFlags,
                ) == 0
            {
                dict = item;
                break;
            } else {
                i += 1
            }
        }
        CFRelease(itemName as CFTypeRef);
    }
    return dict;
}
#[no_mangle]
pub unsafe extern "C" fn findSelectorByName(
    mut feature: CFDictionaryRef,
    mut name: *const libc::c_char,
    mut nameLength: libc::c_int,
) -> CFNumberRef {
    let mut selector: CFNumberRef = 0 as CFNumberRef;
    let mut selectors: CFArrayRef = CFDictionaryGetValue(
        feature,
        kCTFontFeatureTypeSelectorsKey as *const libc::c_void,
    ) as CFArrayRef;
    if !selectors.is_null() {
        let mut s: CFDictionaryRef = findDictionaryInArray(
            selectors,
            kCTFontFeatureSelectorNameKey as *const libc::c_void,
            name,
            nameLength,
        );
        if !s.is_null() {
            selector = CFDictionaryGetValue(
                s,
                kCTFontFeatureSelectorIdentifierKey as *const libc::c_void,
            ) as CFNumberRef
        }
    }
    return selector;
}
unsafe extern "C" fn createFeatureSettingDictionary(
    mut featureTypeIdentifier: CFNumberRef,
    mut featureSelectorIdentifier: CFNumberRef,
) -> CFDictionaryRef {
    let mut settingKeys: [*const libc::c_void; 2] = [
        kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
        kCTFontFeatureSelectorIdentifierKey as *const libc::c_void,
    ];
    let mut settingValues: [*const libc::c_void; 2] = [
        featureTypeIdentifier as *const libc::c_void,
        featureSelectorIdentifier as *const libc::c_void,
    ];
    return CFDictionaryCreate(
        kCFAllocatorDefault,
        settingKeys.as_mut_ptr(),
        settingValues.as_mut_ptr(),
        2i32 as CFIndex,
        &kCFTypeDictionaryKeyCallBacks,
        &kCFTypeDictionaryValueCallBacks,
    );
}
// CFSTR causes undefined builtin errors with c2rust
static mut kXeTeXEmboldenAttributeName: CFStringRef = 0 as CFStringRef;
static mut kLastResort: CFStringRef = 0 as CFStringRef;
#[no_mangle]
pub unsafe extern "C" fn getkXeTeXEmboldenAttributeName() -> CFStringRef {
    if kXeTeXEmboldenAttributeName.is_null() {
        kXeTeXEmboldenAttributeName = CFStringCreateWithCString(
            0 as CFAllocatorRef,
            b"XeTeXEmbolden\x00" as *const u8 as *const libc::c_char,
            kCFStringEncodingUTF8 as libc::c_int as CFStringEncoding,
        )
    }
    return kXeTeXEmboldenAttributeName;
}
#[no_mangle]
pub unsafe extern "C" fn getLastResort() -> CFStringRef {
    if kLastResort.is_null() {
        kLastResort = CFStringCreateWithCString(
            0 as CFAllocatorRef,
            b"LastResort\x00" as *const u8 as *const libc::c_char,
            kCFStringEncodingUTF8 as libc::c_int as CFStringEncoding,
        )
    }
    return kLastResort;
}
#[no_mangle]
pub unsafe extern "C" fn loadAATfont(
    mut descriptor: CTFontDescriptorRef,
    mut scaled_size: int32_t,
    mut cp1: *const libc::c_char,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut font: CTFontRef = 0 as *const __CTFont;
    let mut actualFont: CTFontRef = 0 as *const __CTFont;
    let mut ctSize: CGFloat = 0.;
    let mut stringAttributes: CFMutableDictionaryRef = 0 as *mut __CFDictionary;
    let mut attributes: CFMutableDictionaryRef = 0 as *mut __CFDictionary;
    let mut matrix: CGAffineTransform = CGAffineTransform {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        tx: 0.,
        ty: 0.,
    };
    let mut cascadeList: CFMutableArrayRef = 0 as *mut __CFArray;
    let mut lastResort: CTFontDescriptorRef = 0 as *const __CTFontDescriptor;
    let mut tracking: libc::c_double = 0.0f64;
    let mut extend: libc::c_float = 1.0f64 as libc::c_float;
    let mut slant: libc::c_float = 0.0f64 as libc::c_float;
    let mut embolden: libc::c_float = 0.0f64 as libc::c_float;
    let mut letterspace: libc::c_float = 0.0f64 as libc::c_float;
    let mut rgbValue: uint32_t = 0;
    // create a base font instance for applying further attributes
    ctSize = TeXtoPSPoints(Fix2D(scaled_size));
    font = CTFontCreateWithFontDescriptor(descriptor, ctSize, 0 as *const CGAffineTransform);
    if font.is_null() {
        return 0 as *mut libc::c_void;
    }
    stringAttributes = CFDictionaryCreateMutable(
        0 as CFAllocatorRef,
        0i32 as CFIndex,
        &kCFTypeDictionaryKeyCallBacks,
        &kCFTypeDictionaryValueCallBacks,
    );
    attributes = CFDictionaryCreateMutable(
        0 as CFAllocatorRef,
        0i32 as CFIndex,
        &kCFTypeDictionaryKeyCallBacks,
        &kCFTypeDictionaryValueCallBacks,
    );
    if !cp1.is_null() {
        let mut features: CFArrayRef = CTFontCopyFeatures(font);
        let mut featureSettings: CFMutableArrayRef =
            CFArrayCreateMutable(0 as CFAllocatorRef, 0i32 as CFIndex, &kCFTypeArrayCallBacks);
        // interpret features following ":"
        while *cp1 != 0 {
            let mut feature: CFDictionaryRef = 0 as *const __CFDictionary;
            let mut ret: libc::c_int = 0;
            let mut cp2: *const libc::c_char = 0 as *const libc::c_char;
            let mut cp3: *const libc::c_char = 0 as *const libc::c_char;
            // locate beginning of name=value pair
            if *cp1 as libc::c_int == ':' as i32 || *cp1 as libc::c_int == ';' as i32 {
                // skip over separator
                cp1 = cp1.offset(1)
            }
            while *cp1 as libc::c_int == ' ' as i32 || *cp1 as libc::c_int == '\t' as i32 {
                // skip leading whitespace
                cp1 = cp1.offset(1)
            }
            if *cp1 as libc::c_int == 0i32 {
                break;
            }
            // scan to end of pair
            cp2 = cp1;
            while *cp2 as libc::c_int != 0
                && *cp2 as libc::c_int != ';' as i32
                && *cp2 as libc::c_int != ':' as i32
            {
                cp2 = cp2.offset(1)
            }
            // look for the '=' separator
            cp3 = cp1;
            while cp3 < cp2 && *cp3 as libc::c_int != '=' as i32 {
                cp3 = cp3.offset(1)
            }
            if cp3 == cp2 {
                current_block = 4154772336439402900;
            } else {
                // now cp1 points to option name, cp3 to '=', cp2 to ';' or null
                // first try for a feature by this name
                feature = findDictionaryInArray(
                    features,
                    kCTFontFeatureTypeNameKey as *const libc::c_void,
                    cp1,
                    cp3.wrapping_offset_from(cp1) as libc::c_long as libc::c_int,
                );
                if !feature.is_null() {
                    // look past the '=' separator for setting names
                    let mut featLen: libc::c_int =
                        cp3.wrapping_offset_from(cp1) as libc::c_long as libc::c_int;
                    let mut zeroInteger: libc::c_int = 0i32;
                    let mut zero: CFNumberRef = CFNumberCreate(
                        0 as CFAllocatorRef,
                        kCFNumberIntType as libc::c_int as CFNumberType,
                        &mut zeroInteger as *mut libc::c_int as *const libc::c_void,
                    );
                    cp3 = cp3.offset(1);
                    while cp3 < cp2 {
                        let mut selector: CFNumberRef = 0 as *const __CFNumber;
                        let mut disable: libc::c_int = 0i32;
                        let mut cp4: *const libc::c_char = 0 as *const libc::c_char;
                        // skip leading whitespace
                        while *cp3 as libc::c_int == ' ' as i32
                            || *cp3 as libc::c_int == '\t' as i32
                        {
                            cp3 = cp3.offset(1)
                        }
                        // possibly multiple settings...
                        if *cp3 as libc::c_int == '!' as i32 {
                            // check for negation
                            disable = 1i32;
                            cp3 = cp3.offset(1)
                        }
                        // scan for end of setting name
                        cp4 = cp3;
                        while cp4 < cp2 && *cp4 as libc::c_int != ',' as i32 {
                            cp4 = cp4.offset(1)
                        }
                        // now cp3 points to name, cp4 to ',' or ';' or null
                        selector = findSelectorByName(
                            feature,
                            cp3,
                            cp4.wrapping_offset_from(cp3) as libc::c_long as libc::c_int,
                        );
                        if !selector.is_null()
                            && CFNumberCompare(selector, zero, 0 as *mut libc::c_void)
                                >= 0i32 as libc::c_long
                        {
                            let mut featureType: CFNumberRef = CFDictionaryGetValue(
                                feature,
                                kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                            )
                                as CFNumberRef;
                            let mut featureSetting: CFDictionaryRef =
                                createFeatureSettingDictionary(featureType, selector);
                            CFArrayAppendValue(
                                featureSettings,
                                featureSetting as *const libc::c_void,
                            );
                            CFRelease(featureSetting as CFTypeRef);
                        } else {
                            font_feature_warning(
                                cp1 as *const libc::c_void,
                                featLen,
                                cp3 as *const libc::c_void,
                                cp4.wrapping_offset_from(cp3) as libc::c_long as int32_t,
                            );
                        }
                        // point beyond setting name terminator
                        cp3 = cp4.offset(1)
                    }
                    CFRelease(zero as CFTypeRef);
                    current_block = 15938117740974259152;
                } else {
                    // didn't find feature, try other options...
                    ret = readCommonFeatures(
                        cp1,
                        cp2,
                        &mut extend,
                        &mut slant,
                        &mut embolden,
                        &mut letterspace,
                        &mut rgbValue,
                    );
                    if ret == 1i32 {
                        current_block = 15938117740974259152;
                    } else if ret == -1i32 {
                        current_block = 4154772336439402900;
                    } else {
                        cp3 =
                            strstartswith(cp1, b"tracking\x00" as *const u8 as *const libc::c_char);
                        if !cp3.is_null() {
                            let mut trackingNumber: CFNumberRef = 0 as *const __CFNumber;
                            if *cp3 as libc::c_int != '=' as i32 {
                                current_block = 4154772336439402900;
                            } else {
                                cp3 = cp3.offset(1);
                                tracking = read_double(&mut cp3);
                                trackingNumber = CFNumberCreate(
                                    0 as CFAllocatorRef,
                                    kCFNumberDoubleType as libc::c_int as CFNumberType,
                                    &mut tracking as *mut libc::c_double as *const libc::c_void,
                                );
                                CFDictionaryAddValue(
                                    stringAttributes,
                                    kCTKernAttributeName as *const libc::c_void,
                                    trackingNumber as *const libc::c_void,
                                );
                                CFRelease(trackingNumber as CFTypeRef);
                                current_block = 15938117740974259152;
                            }
                        } else {
                            current_block = 4154772336439402900;
                        }
                    }
                }
            }
            match current_block {
                4154772336439402900 =>
                // not a name=value pair, or not recognized....
                // check for plain "vertical" before complaining
                {
                    if !strstartswith(cp1, b"vertical\x00" as *const u8 as *const libc::c_char)
                        .is_null()
                    {
                        cp3 = cp2;
                        if *cp3 as libc::c_int == ';' as i32 || *cp3 as libc::c_int == ':' as i32 {
                            cp3 = cp3.offset(-1)
                        }
                        while *cp3 as libc::c_int == '\u{0}' as i32
                            || *cp3 as libc::c_int == ' ' as i32
                            || *cp3 as libc::c_int == '\t' as i32
                        {
                            cp3 = cp3.offset(-1)
                        }
                        if *cp3 != 0 {
                            cp3 = cp3.offset(1)
                        }
                        if cp3 == cp1.offset(8) {
                            let mut orientation: libc::c_int =
                                kCTFontOrientationVertical as libc::c_int;
                            let mut orientationNumber: CFNumberRef = CFNumberCreate(
                                0 as CFAllocatorRef,
                                kCFNumberIntType as libc::c_int as CFNumberType,
                                &mut orientation as *mut libc::c_int as *const libc::c_void,
                            );
                            CFDictionaryAddValue(
                                attributes,
                                kCTFontOrientationAttribute as *const libc::c_void,
                                orientationNumber as *const libc::c_void,
                            );
                            CFRelease(orientationNumber as CFTypeRef);
                            CFDictionaryAddValue(
                                stringAttributes,
                                kCTVerticalFormsAttributeName as *const libc::c_void,
                                kCFBooleanTrue as *const libc::c_void,
                            );
                            current_block = 15938117740974259152;
                        } else {
                            current_block = 8464383504555462953;
                        }
                    } else {
                        current_block = 8464383504555462953;
                    }
                    match current_block {
                        15938117740974259152 => {}
                        _ => {
                            font_feature_warning(
                                cp1 as *const libc::c_void,
                                cp2.wrapping_offset_from(cp1) as libc::c_long as int32_t,
                                0 as *const libc::c_void,
                                0i32,
                            );
                        }
                    }
                }
                _ => {}
            }
            // go to next name=value pair
            cp1 = cp2
        }
        // break if end of string
        if !features.is_null() {
            CFRelease(features as CFTypeRef);
        }
        if CFArrayGetCount(featureSettings as CFArrayRef) != 0 {
            CFDictionaryAddValue(
                attributes,
                kCTFontFeatureSettingsAttribute as *const libc::c_void,
                featureSettings as *const libc::c_void,
            );
        }
        CFRelease(featureSettings as CFTypeRef);
    }
    if loaded_font_flags as libc::c_int & 0x1i32 != 0i32 {
        let mut red: CGFloat = ((rgbValue & 0xff000000u32) >> 24i32) as libc::c_double / 255.0f64;
        let mut green: CGFloat =
            ((rgbValue & 0xff0000i32 as libc::c_uint) >> 16i32) as libc::c_double / 255.0f64;
        let mut blue: CGFloat =
            ((rgbValue & 0xff00i32 as libc::c_uint) >> 8i32) as libc::c_double / 255.0f64;
        let mut alpha: CGFloat = (rgbValue & 0xffi32 as libc::c_uint) as libc::c_double / 255.0f64;
        let mut color: CGColorRef = CGColorCreateGenericRGB(red, green, blue, alpha);
        CFDictionaryAddValue(
            stringAttributes,
            kCTForegroundColorAttributeName as *const libc::c_void,
            color as *const libc::c_void,
        );
        CGColorRelease(color);
    }
    matrix = CGAffineTransformIdentity;
    if extend as libc::c_double != 1.0f64 || slant as libc::c_double != 0.0f64 {
        matrix = __CGAffineTransformMake(
            extend as CGFloat,
            0i32 as CGFloat,
            slant as CGFloat,
            1.0f64,
            0i32 as CGFloat,
            0i32 as CGFloat,
        )
    }
    if embolden as libc::c_double != 0.0f64 {
        let mut emboldenNumber: CFNumberRef = 0 as *const __CFNumber;
        embolden = (embolden as libc::c_double * Fix2D(scaled_size) / 100.0f64) as libc::c_float;
        emboldenNumber = CFNumberCreate(
            0 as CFAllocatorRef,
            kCFNumberFloatType as libc::c_int as CFNumberType,
            &mut embolden as *mut libc::c_float as *const libc::c_void,
        );
        CFDictionaryAddValue(
            stringAttributes,
            getkXeTeXEmboldenAttributeName() as *const libc::c_void,
            emboldenNumber as *const libc::c_void,
        );
        CFRelease(emboldenNumber as CFTypeRef);
    }
    if letterspace as libc::c_double != 0.0f64 {
        loaded_font_letter_space =
            (letterspace as libc::c_double / 100.0f64 * scaled_size as libc::c_double) as scaled_t
    }
    // Disable Core Text font fallback (cascading) with only the last resort font
    // in the cascade list.
    cascadeList =
        CFArrayCreateMutable(0 as CFAllocatorRef, 1i32 as CFIndex, &kCFTypeArrayCallBacks);
    lastResort = CTFontDescriptorCreateWithNameAndSize(getLastResort(), 0i32 as CGFloat);
    CFArrayAppendValue(cascadeList, lastResort as *const libc::c_void);
    CFRelease(lastResort as CFTypeRef);
    CFDictionaryAddValue(
        attributes,
        kCTFontCascadeListAttribute as *const libc::c_void,
        cascadeList as *const libc::c_void,
    );
    CFRelease(cascadeList as CFTypeRef);
    descriptor = CTFontDescriptorCreateWithAttributes(attributes as CFDictionaryRef);
    CFRelease(attributes as CFTypeRef);
    actualFont = CTFontCreateCopyWithAttributes(
        font,
        ctSize,
        &mut matrix as *mut CGAffineTransform as *const CGAffineTransform,
        descriptor,
    );
    CFRelease(font as CFTypeRef);
    CFDictionaryAddValue(
        stringAttributes,
        kCTFontAttributeName as *const libc::c_void,
        actualFont as *const libc::c_void,
    );
    CFRelease(actualFont as CFTypeRef);
    native_font_type_flag = 0xffffu32 as int32_t;
    return stringAttributes as *mut libc::c_void;
}

/* the metrics params here are really TeX 'scaled' (or MacOS 'Fixed') values, but that typedef isn't available every place this is included */
/* these are here, not XeTeX_mac.c, because we need stubs on other platforms */
#[no_mangle]
pub unsafe extern "C" fn aat_get_font_metrics(
    mut attributes: CFDictionaryRef,
    mut ascent: *mut i32,
    mut descent: *mut i32,
    mut xheight: *mut i32,
    mut capheight: *mut i32,
    mut slant: *mut i32,
) {
    let mut font: CTFontRef = fontFromAttributes(attributes);
    *ascent = D2Fix(CTFontGetAscent(font));
    *descent = D2Fix(CTFontGetDescent(font));
    *xheight = D2Fix(CTFontGetXHeight(font));
    *capheight = D2Fix(CTFontGetCapHeight(font));
    *slant = D2Fix((-CTFontGetSlantAngle(font)
        * 3.14159265358979323846264338327950288f64
        / 180.0f64).tan());
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get(mut what: i32, mut attributes: CFDictionaryRef) -> i32 {
    let mut rval: libc::c_int = -1i32;
    let mut font: CTFontRef = fontFromAttributes(attributes);
    let mut list: CFArrayRef = 0 as *const __CFArray;
    match what {
        1 => rval = CTFontGetGlyphCount(font) as libc::c_int,
        8 => {
            list = CTFontCopyFeatures(font);
            if !list.is_null() {
                rval = CFArrayGetCount(list) as libc::c_int;
                CFRelease(list as CFTypeRef);
            }
        }
        _ => {}
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get_1(
    mut what: i32,
    mut attributes: CFDictionaryRef,
    mut param: i32,
) -> i32 {
    let mut rval: libc::c_int = -1i32;
    let mut font: CTFontRef = fontFromAttributes(attributes);
    match what {
        9 => {
            let mut features: CFArrayRef = CTFontCopyFeatures(font);
            if !features.is_null() {
                if CFArrayGetCount(features) > param as libc::c_long {
                    let mut feature: CFDictionaryRef =
                        CFArrayGetValueAtIndex(features, param as CFIndex) as CFDictionaryRef;
                    let mut identifier: CFNumberRef = CFDictionaryGetValue(
                        feature,
                        kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                    ) as CFNumberRef;
                    if !identifier.is_null() {
                        CFNumberGetValue(
                            identifier,
                            kCFNumberIntType as libc::c_int as CFNumberType,
                            &mut rval as *mut libc::c_int as *mut libc::c_void,
                        );
                    }
                }
                CFRelease(features as CFTypeRef);
            }
        }
        11 => {
            let mut features_0: CFArrayRef = CTFontCopyFeatures(font);
            if !features_0.is_null() {
                let mut value: CFBooleanRef = 0 as *const __CFBoolean;
                let mut feature_0: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                    features_0,
                    kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                    param,
                );
                let mut found: Boolean = CFDictionaryGetValueIfPresent(
                    feature_0,
                    kCTFontFeatureTypeExclusiveKey as *const libc::c_void,
                    &mut value as *mut CFBooleanRef as *mut *const libc::c_void,
                );
                if found != 0 {
                    rval = CFBooleanGetValue(value) as libc::c_int
                }
                CFRelease(features_0 as CFTypeRef);
            }
        }
        12 => {
            let mut features_1: CFArrayRef = CTFontCopyFeatures(font);
            if !features_1.is_null() {
                let mut feature_1: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                    features_1,
                    kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                    param,
                );
                if !feature_1.is_null() {
                    let mut selectors: CFArrayRef = CFDictionaryGetValue(
                        feature_1,
                        kCTFontFeatureTypeSelectorsKey as *const libc::c_void,
                    ) as CFArrayRef;
                    if !selectors.is_null() {
                        rval = CFArrayGetCount(selectors) as libc::c_int
                    }
                }
                CFRelease(features_1 as CFTypeRef);
            }
        }
        _ => {}
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get_2(
    mut what: i32,
    mut attributes: CFDictionaryRef,
    mut param1: i32,
    mut param2: i32,
) -> i32 {
    let mut rval: libc::c_int = -1i32;
    let mut font: CTFontRef = fontFromAttributes(attributes);
    let mut features: CFArrayRef = CTFontCopyFeatures(font);
    if !features.is_null() {
        let mut feature: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
            features,
            kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
            param1,
        );
        if !feature.is_null() {
            let mut selectors: CFArrayRef = CFDictionaryGetValue(
                feature,
                kCTFontFeatureTypeSelectorsKey as *const libc::c_void,
            ) as CFArrayRef;
            if !selectors.is_null() {
                let mut selector: CFDictionaryRef = 0 as *const __CFDictionary;
                match what {
                    13 => {
                        if CFArrayGetCount(selectors) > param2 as libc::c_long {
                            let mut identifier: CFNumberRef = 0 as *const __CFNumber;
                            selector = CFArrayGetValueAtIndex(selectors, param2 as CFIndex)
                                as CFDictionaryRef;
                            identifier = CFDictionaryGetValue(
                                selector,
                                kCTFontFeatureSelectorIdentifierKey as *const libc::c_void,
                            ) as CFNumberRef;
                            if !identifier.is_null() {
                                CFNumberGetValue(
                                    identifier,
                                    kCFNumberIntType as libc::c_int as CFNumberType,
                                    &mut rval as *mut libc::c_int as *mut libc::c_void,
                                );
                            }
                        }
                    }
                    15 => {
                        selector = findDictionaryInArrayWithIdentifier(
                            selectors,
                            kCTFontFeatureSelectorIdentifierKey as *const libc::c_void,
                            param2,
                        );
                        if !selector.is_null() {
                            let mut isDefault: CFBooleanRef = 0 as *const __CFBoolean;
                            let mut found: Boolean = CFDictionaryGetValueIfPresent(
                                selector,
                                kCTFontFeatureSelectorDefaultKey as *const libc::c_void,
                                &mut isDefault as *mut CFBooleanRef as *mut *const libc::c_void,
                            );
                            if found != 0 {
                                rval = CFBooleanGetValue(isDefault) as libc::c_int
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        CFRelease(features as CFTypeRef);
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get_named(
    mut what: libc::c_int,
    mut attributes: CFDictionaryRef,
) -> libc::c_int {
    let mut rval: libc::c_int = -1i32;
    if what == 10i32 {
        let mut font: CTFontRef = fontFromAttributes(attributes);
        let mut features: CFArrayRef = CTFontCopyFeatures(font);
        if !features.is_null() {
            let mut feature: CFDictionaryRef = findDictionaryInArray(
                features,
                kCTFontFeatureTypeNameKey as *const libc::c_void,
                name_of_file,
                name_length,
            );
            if !feature.is_null() {
                let mut identifier: CFNumberRef = CFDictionaryGetValue(
                    feature,
                    kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                ) as CFNumberRef;
                CFNumberGetValue(
                    identifier,
                    kCFNumberIntType as libc::c_int as CFNumberType,
                    &mut rval as *mut libc::c_int as *mut libc::c_void,
                );
            }
            CFRelease(features as CFTypeRef);
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_font_get_named_1(
    mut what: i32,
    mut attributes: CFDictionaryRef,
    mut param: i32,
) -> i32 {
    let mut rval: libc::c_int = -1i32;
    let mut font: CTFontRef = fontFromAttributes(attributes);
    if what == 14i32 {
        let mut features: CFArrayRef = CTFontCopyFeatures(font);
        if !features.is_null() {
            let mut feature: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                features,
                kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                param,
            );
            if !feature.is_null() {
                let mut selector: CFNumberRef =
                    findSelectorByName(feature, name_of_file, name_length);
                if !selector.is_null() {
                    CFNumberGetValue(
                        selector,
                        kCFNumberIntType as libc::c_int as CFNumberType,
                        &mut rval as *mut libc::c_int as *mut libc::c_void,
                    );
                }
            }
            CFRelease(features as CFTypeRef);
        }
    }
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn aat_print_font_name(
    mut what: i32,
    mut attributes: CFDictionaryRef,
    mut param1: i32,
    mut param2: i32,
) {
    let mut name: CFStringRef = 0 as CFStringRef;
    if what == 8i32 || what == 9i32 {
        let mut font: CTFontRef = fontFromAttributes(attributes);
        let mut features: CFArrayRef = CTFontCopyFeatures(font);
        if !features.is_null() {
            let mut feature: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                features,
                kCTFontFeatureTypeIdentifierKey as *const libc::c_void,
                param1,
            );
            if !feature.is_null() {
                if what == 8i32 {
                    name = CFDictionaryGetValue(
                        feature,
                        kCTFontFeatureTypeNameKey as *const libc::c_void,
                    ) as CFStringRef
                } else {
                    let mut selectors: CFArrayRef = CFDictionaryGetValue(
                        feature,
                        kCTFontFeatureTypeSelectorsKey as *const libc::c_void,
                    ) as CFArrayRef;
                    let mut selector: CFDictionaryRef = findDictionaryInArrayWithIdentifier(
                        selectors,
                        kCTFontFeatureSelectorIdentifierKey as *const libc::c_void,
                        param2,
                    );
                    if !selector.is_null() {
                        name = CFDictionaryGetValue(
                            selector,
                            kCTFontFeatureSelectorNameKey as *const libc::c_void,
                        ) as CFStringRef
                    }
                }
            }
            CFRelease(features as CFTypeRef);
        }
    }
    if !name.is_null() {
        let mut len: CFIndex = CFStringGetLength(name);
        let mut buf: *mut UniChar = xcalloc(
            len as size_t,
            ::std::mem::size_of::<UniChar>() as libc::c_ulong,
        ) as *mut UniChar;
        CFStringGetCharacters(name, CFRangeMake(0i32 as CFIndex, len), buf);
        print_chars(buf, len as libc::c_int);
        free(buf as *mut libc::c_void);
    };
}
