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
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
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
}
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
/* ***************************************************************************
 *
 * fttypes.h
 *
 *   FreeType simple types definitions (specification only).
 *
 * Copyright (C) 1996-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
/* *************************************************************************
 *
 * @section:
 *   basic_types
 *
 * @title:
 *   Basic Data Types
 *
 * @abstract:
 *   The basic data types defined by the library.
 *
 * @description:
 *   This section contains the basic data types defined by FreeType~2,
 *   ranging from simple scalar types to bitmap descriptors.  More
 *   font-specific structures are defined in a different section.
 *
 * @order:
 *   FT_Byte
 *   FT_Bytes
 *   FT_Char
 *   FT_Int
 *   FT_UInt
 *   FT_Int16
 *   FT_UInt16
 *   FT_Int32
 *   FT_UInt32
 *   FT_Int64
 *   FT_UInt64
 *   FT_Short
 *   FT_UShort
 *   FT_Long
 *   FT_ULong
 *   FT_Bool
 *   FT_Offset
 *   FT_PtrDist
 *   FT_String
 *   FT_Tag
 *   FT_Error
 *   FT_Fixed
 *   FT_Pointer
 *   FT_Pos
 *   FT_Vector
 *   FT_BBox
 *   FT_Matrix
 *   FT_FWord
 *   FT_UFWord
 *   FT_F2Dot14
 *   FT_UnitVector
 *   FT_F26Dot6
 *   FT_Data
 *
 *   FT_MAKE_TAG
 *
 *   FT_Generic
 *   FT_Generic_Finalizer
 *
 *   FT_Bitmap
 *   FT_Pixel_Mode
 *   FT_Palette_Mode
 *   FT_Glyph_Format
 *   FT_IMAGE_TAG
 *
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Bool
 *
 * @description:
 *   A typedef of unsigned char, used for simple booleans.  As usual,
 *   values 1 and~0 represent true and false, respectively.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_FWord
 *
 * @description:
 *   A signed 16-bit integer used to store a distance in original font
 *   units.
 */
/* distance in FUnits */
/* *************************************************************************
 *
 * @type:
 *   FT_UFWord
 *
 * @description:
 *   An unsigned 16-bit integer used to store a distance in original font
 *   units.
 */
/* unsigned distance */
/* *************************************************************************
 *
 * @type:
 *   FT_Char
 *
 * @description:
 *   A simple typedef for the _signed_ char type.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Byte
 *
 * @description:
 *   A simple typedef for the _unsigned_ char type.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Bytes
 *
 * @description:
 *   A typedef for constant memory areas.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Tag
 *
 * @description:
 *   A typedef for 32-bit tags (as used in the SFNT format).
 */
/* *************************************************************************
 *
 * @type:
 *   FT_String
 *
 * @description:
 *   A simple typedef for the char type, usually used for strings.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Short
 *
 * @description:
 *   A typedef for signed short.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_UShort
 *
 * @description:
 *   A typedef for unsigned short.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Int
 *
 * @description:
 *   A typedef for the int type.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_UInt
 *
 * @description:
 *   A typedef for the unsigned int type.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Long
 *
 * @description:
 *   A typedef for signed long.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_ULong
 *
 * @description:
 *   A typedef for unsigned long.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_F2Dot14
 *
 * @description:
 *   A signed 2.14 fixed-point type used for unit vectors.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_F26Dot6
 *
 * @description:
 *   A signed 26.6 fixed-point type used for vectorial pixel coordinates.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Fixed
 *
 * @description:
 *   This type is used to store 16.16 fixed-point values, like scaling
 *   values or matrix coefficients.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Error
 *
 * @description:
 *   The FreeType error code type.  A value of~0 is always interpreted as a
 *   successful operation.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Pointer
 *
 * @description:
 *   A simple typedef for a typeless pointer.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Offset
 *
 * @description:
 *   This is equivalent to the ANSI~C `size_t` type, i.e., the largest
 *   _unsigned_ integer type used to express a file size or position, or a
 *   memory block size.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_PtrDist
 *
 * @description:
 *   This is equivalent to the ANSI~C `ptrdiff_t` type, i.e., the largest
 *   _signed_ integer type used to express the distance between two
 *   pointers.
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_UnitVector
 *
 * @description:
 *   A simple structure used to store a 2D vector unit vector.  Uses
 *   FT_F2Dot14 types.
 *
 * @fields:
 *   x ::
 *     Horizontal coordinate.
 *
 *   y ::
 *     Vertical coordinate.
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_Matrix
 *
 * @description:
 *   A simple structure used to store a 2x2 matrix.  Coefficients are in
 *   16.16 fixed-point format.  The computation performed is:
 *
 *   ```
 *     x' = x*xx + y*xy
 *     y' = x*yx + y*yy
 *   ```
 *
 * @fields:
 *   xx ::
 *     Matrix coefficient.
 *
 *   xy ::
 *     Matrix coefficient.
 *
 *   yx ::
 *     Matrix coefficient.
 *
 *   yy ::
 *     Matrix coefficient.
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_Data
 *
 * @description:
 *   Read-only binary data represented as a pointer and a length.
 *
 * @fields:
 *   pointer ::
 *     The data.
 *
 *   length ::
 *     The length of the data in bytes.
 */
/* *************************************************************************
 *
 * @functype:
 *   FT_Generic_Finalizer
 *
 * @description:
 *   Describe a function used to destroy the 'client' data of any FreeType
 *   object.  See the description of the @FT_Generic type for details of
 *   usage.
 *
 * @input:
 *   The address of the FreeType object that is under finalization.  Its
 *   client data is accessed through its `generic` field.
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_Generic
 *
 * @description:
 *   Client applications often need to associate their own data to a
 *   variety of FreeType core objects.  For example, a text layout API
 *   might want to associate a glyph cache to a given size object.
 *
 *   Some FreeType object contains a `generic` field, of type `FT_Generic`,
 *   which usage is left to client applications and font servers.
 *
 *   It can be used to store a pointer to client-specific data, as well as
 *   the address of a 'finalizer' function, which will be called by
 *   FreeType when the object is destroyed (for example, the previous
 *   client example would put the address of the glyph cache destructor in
 *   the `finalizer` field).
 *
 * @fields:
 *   data ::
 *     A typeless pointer to any client-specified data. This field is
 *     completely ignored by the FreeType library.
 *
 *   finalizer ::
 *     A pointer to a 'generic finalizer' function, which will be called
 *     when the object is destroyed.  If this field is set to `NULL`, no
 *     code will be called.
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_MAKE_TAG
 *
 * @description:
 *   This macro converts four-letter tags that are used to label TrueType
 *   tables into an unsigned long, to be used within FreeType.
 *
 * @note:
 *   The produced values **must** be 32-bit integers.  Don't redefine this
 *   macro.
 */
/* ************************************************************************/
/* ************************************************************************/
/*                                                                       */
/*                    L I S T   M A N A G E M E N T                      */
/*                                                                       */
/* ************************************************************************/
/* ************************************************************************/
/* *************************************************************************
 *
 * @section:
 *   list_processing
 *
 */
/* *************************************************************************
 *
 * @type:
 *   FT_ListNode
 *
 * @description:
 *    Many elements and objects in FreeType are listed through an @FT_List
 *    record (see @FT_ListRec).  As its name suggests, an FT_ListNode is a
 *    handle to a single list element.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_List
 *
 * @description:
 *   A handle to a list record (see @FT_ListRec).
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_ListNodeRec
 *
 * @description:
 *   A structure used to hold a single list element.
 *
 * @fields:
 *   prev ::
 *     The previous element in the list.  `NULL` if first.
 *
 *   next ::
 *     The next element in the list.  `NULL` if last.
 *
 *   data ::
 *     A typeless pointer to the listed object.
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_ListRec
 *
 * @description:
 *   A structure used to hold a simple doubly-linked list.  These are used
 *   in many parts of FreeType.
 *
 * @fields:
 *   head ::
 *     The head (first element) of doubly-linked list.
 *
 *   tail ::
 *     The tail (last element) of doubly-linked list.
 */
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
/* ***************************************************************************
 *
 * freetype.h
 *
 *   FreeType high-level API and common types (specification only).
 *
 * Copyright (C) 1996-2019 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */
/* *************************************************************************
 *
 * @section:
 *   header_inclusion
 *
 * @title:
 *   FreeType's header inclusion scheme
 *
 * @abstract:
 *   How client applications should include FreeType header files.
 *
 * @description:
 *   To be as flexible as possible (and for historical reasons), FreeType
 *   uses a very special inclusion scheme to load header files, for example
 *
 *   ```
 *     #include <ft2build.h>
 *
 *     #include FT_FREETYPE_H
 *     #include FT_OUTLINE_H
 *   ```
 *
 *   A compiler and its preprocessor only needs an include path to find the
 *   file `ft2build.h`; the exact locations and names of the other FreeType
 *   header files are hidden by @header_file_macros, loaded by
 *   `ft2build.h`.  The API documentation always gives the header macro
 *   name needed for a particular function.
 *
 */
/* *************************************************************************
 *
 * @section:
 *   user_allocation
 *
 * @title:
 *   User allocation
 *
 * @abstract:
 *   How client applications should allocate FreeType data structures.
 *
 * @description:
 *   FreeType assumes that structures allocated by the user and passed as
 *   arguments are zeroed out except for the actual data.  In other words,
 *   it is recommended to use `calloc` (or variants of it) instead of
 *   `malloc` for allocation.
 *
 */
/* ************************************************************************/
/* ************************************************************************/
/*                                                                       */
/*                        B A S I C   T Y P E S                          */
/*                                                                       */
/* ************************************************************************/
/* ************************************************************************/
/* *************************************************************************
 *
 * @section:
 *   base_interface
 *
 * @title:
 *   Base Interface
 *
 * @abstract:
 *   The FreeType~2 base font interface.
 *
 * @description:
 *   This section describes the most important public high-level API
 *   functions of FreeType~2.
 *
 * @order:
 *   FT_Library
 *   FT_Face
 *   FT_Size
 *   FT_GlyphSlot
 *   FT_CharMap
 *   FT_Encoding
 *   FT_ENC_TAG
 *
 *   FT_FaceRec
 *
 *   FT_FACE_FLAG_SCALABLE
 *   FT_FACE_FLAG_FIXED_SIZES
 *   FT_FACE_FLAG_FIXED_WIDTH
 *   FT_FACE_FLAG_HORIZONTAL
 *   FT_FACE_FLAG_VERTICAL
 *   FT_FACE_FLAG_COLOR
 *   FT_FACE_FLAG_SFNT
 *   FT_FACE_FLAG_CID_KEYED
 *   FT_FACE_FLAG_TRICKY
 *   FT_FACE_FLAG_KERNING
 *   FT_FACE_FLAG_MULTIPLE_MASTERS
 *   FT_FACE_FLAG_VARIATION
 *   FT_FACE_FLAG_GLYPH_NAMES
 *   FT_FACE_FLAG_EXTERNAL_STREAM
 *   FT_FACE_FLAG_HINTER
 *
 *   FT_HAS_HORIZONTAL
 *   FT_HAS_VERTICAL
 *   FT_HAS_KERNING
 *   FT_HAS_FIXED_SIZES
 *   FT_HAS_GLYPH_NAMES
 *   FT_HAS_COLOR
 *   FT_HAS_MULTIPLE_MASTERS
 *
 *   FT_IS_SFNT
 *   FT_IS_SCALABLE
 *   FT_IS_FIXED_WIDTH
 *   FT_IS_CID_KEYED
 *   FT_IS_TRICKY
 *   FT_IS_NAMED_INSTANCE
 *   FT_IS_VARIATION
 *
 *   FT_STYLE_FLAG_BOLD
 *   FT_STYLE_FLAG_ITALIC
 *
 *   FT_SizeRec
 *   FT_Size_Metrics
 *
 *   FT_GlyphSlotRec
 *   FT_Glyph_Metrics
 *   FT_SubGlyph
 *
 *   FT_Bitmap_Size
 *
 *   FT_Init_FreeType
 *   FT_Done_FreeType
 *
 *   FT_New_Face
 *   FT_Done_Face
 *   FT_Reference_Face
 *   FT_New_Memory_Face
 *   FT_Face_Properties
 *   FT_Open_Face
 *   FT_Open_Args
 *   FT_Parameter
 *   FT_Attach_File
 *   FT_Attach_Stream
 *
 *   FT_Set_Char_Size
 *   FT_Set_Pixel_Sizes
 *   FT_Request_Size
 *   FT_Select_Size
 *   FT_Size_Request_Type
 *   FT_Size_RequestRec
 *   FT_Size_Request
 *   FT_Set_Transform
 *   FT_Load_Glyph
 *   FT_Get_Char_Index
 *   FT_Get_First_Char
 *   FT_Get_Next_Char
 *   FT_Get_Name_Index
 *   FT_Load_Char
 *
 *   FT_OPEN_MEMORY
 *   FT_OPEN_STREAM
 *   FT_OPEN_PATHNAME
 *   FT_OPEN_DRIVER
 *   FT_OPEN_PARAMS
 *
 *   FT_LOAD_DEFAULT
 *   FT_LOAD_RENDER
 *   FT_LOAD_MONOCHROME
 *   FT_LOAD_LINEAR_DESIGN
 *   FT_LOAD_NO_SCALE
 *   FT_LOAD_NO_HINTING
 *   FT_LOAD_NO_BITMAP
 *   FT_LOAD_NO_AUTOHINT
 *   FT_LOAD_COLOR
 *
 *   FT_LOAD_VERTICAL_LAYOUT
 *   FT_LOAD_IGNORE_TRANSFORM
 *   FT_LOAD_FORCE_AUTOHINT
 *   FT_LOAD_NO_RECURSE
 *   FT_LOAD_PEDANTIC
 *
 *   FT_LOAD_TARGET_NORMAL
 *   FT_LOAD_TARGET_LIGHT
 *   FT_LOAD_TARGET_MONO
 *   FT_LOAD_TARGET_LCD
 *   FT_LOAD_TARGET_LCD_V
 *
 *   FT_LOAD_TARGET_MODE
 *
 *   FT_Render_Glyph
 *   FT_Render_Mode
 *   FT_Get_Kerning
 *   FT_Kerning_Mode
 *   FT_Get_Track_Kerning
 *   FT_Get_Glyph_Name
 *   FT_Get_Postscript_Name
 *
 *   FT_CharMapRec
 *   FT_Select_Charmap
 *   FT_Set_Charmap
 *   FT_Get_Charmap_Index
 *
 *   FT_Get_FSType_Flags
 *   FT_Get_SubGlyph_Info
 *
 *   FT_Face_Internal
 *   FT_Size_Internal
 *   FT_Slot_Internal
 *
 *   FT_FACE_FLAG_XXX
 *   FT_STYLE_FLAG_XXX
 *   FT_OPEN_XXX
 *   FT_LOAD_XXX
 *   FT_LOAD_TARGET_XXX
 *   FT_SUBGLYPH_FLAG_XXX
 *   FT_FSTYPE_XXX
 *
 *   FT_HAS_FAST_GLYPHS
 *
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_Glyph_Metrics
 *
 * @description:
 *   A structure to model the metrics of a single glyph.  The values are
 *   expressed in 26.6 fractional pixel format; if the flag
 *   @FT_LOAD_NO_SCALE has been used while loading the glyph, values are
 *   expressed in font units instead.
 *
 * @fields:
 *   width ::
 *     The glyph's width.
 *
 *   height ::
 *     The glyph's height.
 *
 *   horiBearingX ::
 *     Left side bearing for horizontal layout.
 *
 *   horiBearingY ::
 *     Top side bearing for horizontal layout.
 *
 *   horiAdvance ::
 *     Advance width for horizontal layout.
 *
 *   vertBearingX ::
 *     Left side bearing for vertical layout.
 *
 *   vertBearingY ::
 *     Top side bearing for vertical layout.  Larger positive values mean
 *     further below the vertical glyph origin.
 *
 *   vertAdvance ::
 *     Advance height for vertical layout.  Positive values mean the glyph
 *     has a positive advance downward.
 *
 * @note:
 *   If not disabled with @FT_LOAD_NO_HINTING, the values represent
 *   dimensions of the hinted glyph (in case hinting is applicable).
 *
 *   Stroking a glyph with an outside border does not increase
 *   `horiAdvance` or `vertAdvance`; you have to manually adjust these
 *   values to account for the added width and height.
 *
 *   FreeType doesn't use the 'VORG' table data for CFF fonts because it
 *   doesn't have an interface to quickly retrieve the glyph height.  The
 *   y~coordinate of the vertical origin can be simply computed as
 *   `vertBearingY + height` after loading a glyph.
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_Bitmap_Size
 *
 * @description:
 *   This structure models the metrics of a bitmap strike (i.e., a set of
 *   glyphs for a given point size and resolution) in a bitmap font.  It is
 *   used for the `available_sizes` field of @FT_Face.
 *
 * @fields:
 *   height ::
 *     The vertical distance, in pixels, between two consecutive baselines.
 *     It is always positive.
 *
 *   width ::
 *     The average width, in pixels, of all glyphs in the strike.
 *
 *   size ::
 *     The nominal size of the strike in 26.6 fractional points.  This
 *     field is not very useful.
 *
 *   x_ppem ::
 *     The horizontal ppem (nominal width) in 26.6 fractional pixels.
 *
 *   y_ppem ::
 *     The vertical ppem (nominal height) in 26.6 fractional pixels.
 *
 * @note:
 *   Windows FNT:
 *     The nominal size given in a FNT font is not reliable.  If the driver
 *     finds it incorrect, it sets `size` to some calculated values, and
 *     `x_ppem` and `y_ppem` to the pixel width and height given in the
 *     font, respectively.
 *
 *   TrueType embedded bitmaps:
 *     `size`, `width`, and `height` values are not contained in the bitmap
 *     strike itself.  They are computed from the global font parameters.
 */
/* ************************************************************************/
/* ************************************************************************/
/*                                                                       */
/*                     O B J E C T   C L A S S E S                       */
/*                                                                       */
/* ************************************************************************/
/* ************************************************************************/
/* *************************************************************************
 *
 * @type:
 *   FT_Library
 *
 * @description:
 *   A handle to a FreeType library instance.  Each 'library' is completely
 *   independent from the others; it is the 'root' of a set of objects like
 *   fonts, faces, sizes, etc.
 *
 *   It also embeds a memory manager (see @FT_Memory), as well as a
 *   scan-line converter object (see @FT_Raster).
 *
 *   [Since 2.5.6] In multi-threaded applications it is easiest to use one
 *   `FT_Library` object per thread.  In case this is too cumbersome, a
 *   single `FT_Library` object across threads is possible also, as long as
 *   a mutex lock is used around @FT_New_Face and @FT_Done_Face.
 *
 * @note:
 *   Library objects are normally created by @FT_Init_FreeType, and
 *   destroyed with @FT_Done_FreeType.  If you need reference-counting
 *   (cf. @FT_Reference_Library), use @FT_New_Library and @FT_Done_Library.
 */
/* *************************************************************************
 *
 * @section:
 *   module_management
 *
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Module
 *
 * @description:
 *   A handle to a given FreeType module object.  A module can be a font
 *   driver, a renderer, or anything else that provides services to the
 *   former.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Driver
 *
 * @description:
 *   A handle to a given FreeType font driver object.  A font driver is a
 *   module capable of creating faces from font files.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Renderer
 *
 * @description:
 *   A handle to a given FreeType renderer.  A renderer is a module in
 *   charge of converting a glyph's outline image to a bitmap.  It supports
 *   a single glyph image format, and one or more target surface depths.
 */
/* *************************************************************************
 *
 * @section:
 *   base_interface
 *
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Face
 *
 * @description:
 *   A handle to a typographic face object.  A face object models a given
 *   typeface, in a given style.
 *
 * @note:
 *   A face object also owns a single @FT_GlyphSlot object, as well as one
 *   or more @FT_Size objects.
 *
 *   Use @FT_New_Face or @FT_Open_Face to create a new face object from a
 *   given filepath or a custom input stream.
 *
 *   Use @FT_Done_Face to destroy it (along with its slot and sizes).
 *
 *   An `FT_Face` object can only be safely used from one thread at a time.
 *   Similarly, creation and destruction of `FT_Face` with the same
 *   @FT_Library object can only be done from one thread at a time.  On the
 *   other hand, functions like @FT_Load_Glyph and its siblings are
 *   thread-safe and do not need the lock to be held as long as the same
 *   `FT_Face` object is not used from multiple threads at the same time.
 *
 * @also:
 *   See @FT_FaceRec for the publicly accessible fields of a given face
 *   object.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Size
 *
 * @description:
 *   A handle to an object that models a face scaled to a given character
 *   size.
 *
 * @note:
 *   An @FT_Face has one _active_ @FT_Size object that is used by functions
 *   like @FT_Load_Glyph to determine the scaling transformation that in
 *   turn is used to load and hint glyphs and metrics.
 *
 *   You can use @FT_Set_Char_Size, @FT_Set_Pixel_Sizes, @FT_Request_Size
 *   or even @FT_Select_Size to change the content (i.e., the scaling
 *   values) of the active @FT_Size.
 *
 *   You can use @FT_New_Size to create additional size objects for a given
 *   @FT_Face, but they won't be used by other functions until you activate
 *   it through @FT_Activate_Size.  Only one size can be activated at any
 *   given time per face.
 *
 * @also:
 *   See @FT_SizeRec for the publicly accessible fields of a given size
 *   object.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_GlyphSlot
 *
 * @description:
 *   A handle to a given 'glyph slot'.  A slot is a container that can hold
 *   any of the glyphs contained in its parent face.
 *
 *   In other words, each time you call @FT_Load_Glyph or @FT_Load_Char,
 *   the slot's content is erased by the new glyph data, i.e., the glyph's
 *   metrics, its image (bitmap or outline), and other control information.
 *
 * @also:
 *   See @FT_GlyphSlotRec for the publicly accessible glyph fields.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_CharMap
 *
 * @description:
 *   A handle to a character map (usually abbreviated to 'charmap').  A
 *   charmap is used to translate character codes in a given encoding into
 *   glyph indexes for its parent's face.  Some font formats may provide
 *   several charmaps per font.
 *
 *   Each face object owns zero or more charmaps, but only one of them can
 *   be 'active', providing the data used by @FT_Get_Char_Index or
 *   @FT_Load_Char.
 *
 *   The list of available charmaps in a face is available through the
 *   `face->num_charmaps` and `face->charmaps` fields of @FT_FaceRec.
 *
 *   The currently active charmap is available as `face->charmap`.  You
 *   should call @FT_Set_Charmap to change it.
 *
 * @note:
 *   When a new face is created (either through @FT_New_Face or
 *   @FT_Open_Face), the library looks for a Unicode charmap within the
 *   list and automatically activates it.  If there is no Unicode charmap,
 *   FreeType doesn't set an 'active' charmap.
 *
 * @also:
 *   See @FT_CharMapRec for the publicly accessible fields of a given
 *   character map.
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_ENC_TAG
 *
 * @description:
 *   This macro converts four-letter tags into an unsigned long.  It is
 *   used to define 'encoding' identifiers (see @FT_Encoding).
 *
 * @note:
 *   Since many 16-bit compilers don't like 32-bit enumerations, you should
 *   redefine this macro in case of problems to something like this:
 *
 *   ```
 *     #define FT_ENC_TAG( value, a, b, c, d )  value
 *   ```
 *
 *   to get a simple enumeration without assigning special numbers.
 */
/* FT_ENC_TAG */
/* *************************************************************************
 *
 * @enum:
 *   FT_Encoding
 *
 * @description:
 *   An enumeration to specify character sets supported by charmaps.  Used
 *   in the @FT_Select_Charmap API function.
 *
 * @note:
 *   Despite the name, this enumeration lists specific character
 *   repertories (i.e., charsets), and not text encoding methods (e.g.,
 *   UTF-8, UTF-16, etc.).
 *
 *   Other encodings might be defined in the future.
 *
 * @values:
 *   FT_ENCODING_NONE ::
 *     The encoding value~0 is reserved for all formats except BDF, PCF,
 *     and Windows FNT; see below for more information.
 *
 *   FT_ENCODING_UNICODE ::
 *     The Unicode character set.  This value covers all versions of the
 *     Unicode repertoire, including ASCII and Latin-1.  Most fonts include
 *     a Unicode charmap, but not all of them.
 *
 *     For example, if you want to access Unicode value U+1F028 (and the
 *     font contains it), use value 0x1F028 as the input value for
 *     @FT_Get_Char_Index.
 *
 *   FT_ENCODING_MS_SYMBOL ::
 *     Microsoft Symbol encoding, used to encode mathematical symbols and
 *     wingdings.  For more information, see
 *     'https://www.microsoft.com/typography/otspec/recom.htm#non-standard-symbol-fonts',
 *     'http://www.kostis.net/charsets/symbol.htm', and
 *     'http://www.kostis.net/charsets/wingding.htm'.
 *
 *     This encoding uses character codes from the PUA (Private Unicode
 *     Area) in the range U+F020-U+F0FF.
 *
 *   FT_ENCODING_SJIS ::
 *     Shift JIS encoding for Japanese.  More info at
 *     'https://en.wikipedia.org/wiki/Shift_JIS'.  See note on multi-byte
 *     encodings below.
 *
 *   FT_ENCODING_PRC ::
 *     Corresponds to encoding systems mainly for Simplified Chinese as
 *     used in People's Republic of China (PRC).  The encoding layout is
 *     based on GB~2312 and its supersets GBK and GB~18030.
 *
 *   FT_ENCODING_BIG5 ::
 *     Corresponds to an encoding system for Traditional Chinese as used in
 *     Taiwan and Hong Kong.
 *
 *   FT_ENCODING_WANSUNG ::
 *     Corresponds to the Korean encoding system known as Extended Wansung
 *     (MS Windows code page 949).  For more information see
 *     'https://www.unicode.org/Public/MAPPINGS/VENDORS/MICSFT/WindowsBestFit/bestfit949.txt'.
 *
 *   FT_ENCODING_JOHAB ::
 *     The Korean standard character set (KS~C 5601-1992), which
 *     corresponds to MS Windows code page 1361.  This character set
 *     includes all possible Hangul character combinations.
 *
 *   FT_ENCODING_ADOBE_LATIN_1 ::
 *     Corresponds to a Latin-1 encoding as defined in a Type~1 PostScript
 *     font.  It is limited to 256 character codes.
 *
 *   FT_ENCODING_ADOBE_STANDARD ::
 *     Adobe Standard encoding, as found in Type~1, CFF, and OpenType/CFF
 *     fonts.  It is limited to 256 character codes.
 *
 *   FT_ENCODING_ADOBE_EXPERT ::
 *     Adobe Expert encoding, as found in Type~1, CFF, and OpenType/CFF
 *     fonts.  It is limited to 256 character codes.
 *
 *   FT_ENCODING_ADOBE_CUSTOM ::
 *     Corresponds to a custom encoding, as found in Type~1, CFF, and
 *     OpenType/CFF fonts.  It is limited to 256 character codes.
 *
 *   FT_ENCODING_APPLE_ROMAN ::
 *     Apple roman encoding.  Many TrueType and OpenType fonts contain a
 *     charmap for this 8-bit encoding, since older versions of Mac OS are
 *     able to use it.
 *
 *   FT_ENCODING_OLD_LATIN_2 ::
 *     This value is deprecated and was neither used nor reported by
 *     FreeType.  Don't use or test for it.
 *
 *   FT_ENCODING_MS_SJIS ::
 *     Same as FT_ENCODING_SJIS.  Deprecated.
 *
 *   FT_ENCODING_MS_GB2312 ::
 *     Same as FT_ENCODING_PRC.  Deprecated.
 *
 *   FT_ENCODING_MS_BIG5 ::
 *     Same as FT_ENCODING_BIG5.  Deprecated.
 *
 *   FT_ENCODING_MS_WANSUNG ::
 *     Same as FT_ENCODING_WANSUNG.  Deprecated.
 *
 *   FT_ENCODING_MS_JOHAB ::
 *     Same as FT_ENCODING_JOHAB.  Deprecated.
 *
 * @note:
 *   By default, FreeType enables a Unicode charmap and tags it with
 *   `FT_ENCODING_UNICODE` when it is either provided or can be generated
 *   from PostScript glyph name dictionaries in the font file.  All other
 *   encodings are considered legacy and tagged only if explicitly defined
 *   in the font file.  Otherwise, `FT_ENCODING_NONE` is used.
 *
 *   `FT_ENCODING_NONE` is set by the BDF and PCF drivers if the charmap is
 *   neither Unicode nor ISO-8859-1 (otherwise it is set to
 *   `FT_ENCODING_UNICODE`).  Use @FT_Get_BDF_Charset_ID to find out which
 *   encoding is really present.  If, for example, the `cs_registry` field
 *   is 'KOI8' and the `cs_encoding` field is 'R', the font is encoded in
 *   KOI8-R.
 *
 *   `FT_ENCODING_NONE` is always set (with a single exception) by the
 *   winfonts driver.  Use @FT_Get_WinFNT_Header and examine the `charset`
 *   field of the @FT_WinFNT_HeaderRec structure to find out which encoding
 *   is really present.  For example, @FT_WinFNT_ID_CP1251 (204) means
 *   Windows code page 1251 (for Russian).
 *
 *   `FT_ENCODING_NONE` is set if `platform_id` is @TT_PLATFORM_MACINTOSH
 *   and `encoding_id` is not `TT_MAC_ID_ROMAN` (otherwise it is set to
 *   `FT_ENCODING_APPLE_ROMAN`).
 *
 *   If `platform_id` is @TT_PLATFORM_MACINTOSH, use the function
 *   @FT_Get_CMap_Language_ID to query the Mac language ID that may be
 *   needed to be able to distinguish Apple encoding variants.  See
 *
 *     https://www.unicode.org/Public/MAPPINGS/VENDORS/APPLE/Readme.txt
 *
 *   to get an idea how to do that.  Basically, if the language ID is~0,
 *   don't use it, otherwise subtract 1 from the language ID.  Then examine
 *   `encoding_id`.  If, for example, `encoding_id` is `TT_MAC_ID_ROMAN`
 *   and the language ID (minus~1) is `TT_MAC_LANGID_GREEK`, it is the
 *   Greek encoding, not Roman.  `TT_MAC_ID_ARABIC` with
 *   `TT_MAC_LANGID_FARSI` means the Farsi variant the Arabic encoding.
 */
/* for backward compatibility */
/* these constants are deprecated; use the corresponding `FT_Encoding` */
/* values instead                                                      */
/* *************************************************************************
 *
 * @struct:
 *   FT_CharMapRec
 *
 * @description:
 *   The base charmap structure.
 *
 * @fields:
 *   face ::
 *     A handle to the parent face object.
 *
 *   encoding ::
 *     An @FT_Encoding tag identifying the charmap.  Use this with
 *     @FT_Select_Charmap.
 *
 *   platform_id ::
 *     An ID number describing the platform for the following encoding ID.
 *     This comes directly from the TrueType specification and gets
 *     emulated for other formats.
 *
 *   encoding_id ::
 *     A platform-specific encoding number.  This also comes from the
 *     TrueType specification and gets emulated similarly.
 */
/* ************************************************************************/
/* ************************************************************************/
/*                                                                       */
/*                 B A S E   O B J E C T   C L A S S E S                 */
/*                                                                       */
/* ************************************************************************/
/* ************************************************************************/
/* *************************************************************************
 *
 * @type:
 *   FT_Face_Internal
 *
 * @description:
 *   An opaque handle to an `FT_Face_InternalRec` structure that models the
 *   private data of a given @FT_Face object.
 *
 *   This structure might change between releases of FreeType~2 and is not
 *   generally available to client applications.
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_FaceRec
 *
 * @description:
 *   FreeType root face class structure.  A face object models a typeface
 *   in a font file.
 *
 * @fields:
 *   num_faces ::
 *     The number of faces in the font file.  Some font formats can have
 *     multiple faces in a single font file.
 *
 *   face_index ::
 *     This field holds two different values.  Bits 0-15 are the index of
 *     the face in the font file (starting with value~0).  They are set
 *     to~0 if there is only one face in the font file.
 *
 *     [Since 2.6.1] Bits 16-30 are relevant to GX and OpenType variation
 *     fonts only, holding the named instance index for the current face
 *     index (starting with value~1; value~0 indicates font access without
 *     a named instance).  For non-variation fonts, bits 16-30 are ignored.
 *     If we have the third named instance of face~4, say, `face_index` is
 *     set to 0x00030004.
 *
 *     Bit 31 is always zero (this is, `face_index` is always a positive
 *     value).
 *
 *     [Since 2.9] Changing the design coordinates with
 *     @FT_Set_Var_Design_Coordinates or @FT_Set_Var_Blend_Coordinates does
 *     not influence the named instance index value (only
 *     @FT_Set_Named_Instance does that).
 *
 *   face_flags ::
 *     A set of bit flags that give important information about the face;
 *     see @FT_FACE_FLAG_XXX for the details.
 *
 *   style_flags ::
 *     The lower 16~bits contain a set of bit flags indicating the style of
 *     the face; see @FT_STYLE_FLAG_XXX for the details.
 *
 *     [Since 2.6.1] Bits 16-30 hold the number of named instances
 *     available for the current face if we have a GX or OpenType variation
 *     (sub)font.  Bit 31 is always zero (this is, `style_flags` is always
 *     a positive value).  Note that a variation font has always at least
 *     one named instance, namely the default instance.
 *
 *   num_glyphs ::
 *     The number of glyphs in the face.  If the face is scalable and has
 *     sbits (see `num_fixed_sizes`), it is set to the number of outline
 *     glyphs.
 *
 *     For CID-keyed fonts (not in an SFNT wrapper) this value gives the
 *     highest CID used in the font.
 *
 *   family_name ::
 *     The face's family name.  This is an ASCII string, usually in
 *     English, that describes the typeface's family (like 'Times New
 *     Roman', 'Bodoni', 'Garamond', etc).  This is a least common
 *     denominator used to list fonts.  Some formats (TrueType & OpenType)
 *     provide localized and Unicode versions of this string.  Applications
 *     should use the format-specific interface to access them.  Can be
 *     `NULL` (e.g., in fonts embedded in a PDF file).
 *
 *     In case the font doesn't provide a specific family name entry,
 *     FreeType tries to synthesize one, deriving it from other name
 *     entries.
 *
 *   style_name ::
 *     The face's style name.  This is an ASCII string, usually in English,
 *     that describes the typeface's style (like 'Italic', 'Bold',
 *     'Condensed', etc).  Not all font formats provide a style name, so
 *     this field is optional, and can be set to `NULL`.  As for
 *     `family_name`, some formats provide localized and Unicode versions
 *     of this string.  Applications should use the format-specific
 *     interface to access them.
 *
 *   num_fixed_sizes ::
 *     The number of bitmap strikes in the face.  Even if the face is
 *     scalable, there might still be bitmap strikes, which are called
 *     'sbits' in that case.
 *
 *   available_sizes ::
 *     An array of @FT_Bitmap_Size for all bitmap strikes in the face.  It
 *     is set to `NULL` if there is no bitmap strike.
 *
 *     Note that FreeType tries to sanitize the strike data since they are
 *     sometimes sloppy or incorrect, but this can easily fail.
 *
 *   num_charmaps ::
 *     The number of charmaps in the face.
 *
 *   charmaps ::
 *     An array of the charmaps of the face.
 *
 *   generic ::
 *     A field reserved for client uses.  See the @FT_Generic type
 *     description.
 *
 *   bbox ::
 *     The font bounding box.  Coordinates are expressed in font units (see
 *     `units_per_EM`).  The box is large enough to contain any glyph from
 *     the font.  Thus, `bbox.yMax` can be seen as the 'maximum ascender',
 *     and `bbox.yMin` as the 'minimum descender'.  Only relevant for
 *     scalable formats.
 *
 *     Note that the bounding box might be off by (at least) one pixel for
 *     hinted fonts.  See @FT_Size_Metrics for further discussion.
 *
 *   units_per_EM ::
 *     The number of font units per EM square for this face.  This is
 *     typically 2048 for TrueType fonts, and 1000 for Type~1 fonts.  Only
 *     relevant for scalable formats.
 *
 *   ascender ::
 *     The typographic ascender of the face, expressed in font units.  For
 *     font formats not having this information, it is set to `bbox.yMax`.
 *     Only relevant for scalable formats.
 *
 *   descender ::
 *     The typographic descender of the face, expressed in font units.  For
 *     font formats not having this information, it is set to `bbox.yMin`.
 *     Note that this field is negative for values below the baseline.
 *     Only relevant for scalable formats.
 *
 *   height ::
 *     This value is the vertical distance between two consecutive
 *     baselines, expressed in font units.  It is always positive.  Only
 *     relevant for scalable formats.
 *
 *     If you want the global glyph height, use `ascender - descender`.
 *
 *   max_advance_width ::
 *     The maximum advance width, in font units, for all glyphs in this
 *     face.  This can be used to make word wrapping computations faster.
 *     Only relevant for scalable formats.
 *
 *   max_advance_height ::
 *     The maximum advance height, in font units, for all glyphs in this
 *     face.  This is only relevant for vertical layouts, and is set to
 *     `height` for fonts that do not provide vertical metrics.  Only
 *     relevant for scalable formats.
 *
 *   underline_position ::
 *     The position, in font units, of the underline line for this face.
 *     It is the center of the underlining stem.  Only relevant for
 *     scalable formats.
 *
 *   underline_thickness ::
 *     The thickness, in font units, of the underline for this face.  Only
 *     relevant for scalable formats.
 *
 *   glyph ::
 *     The face's associated glyph slot(s).
 *
 *   size ::
 *     The current active size for this face.
 *
 *   charmap ::
 *     The current active charmap for this face.
 *
 * @note:
 *   Fields may be changed after a call to @FT_Attach_File or
 *   @FT_Attach_Stream.
 *
 *   For an OpenType variation font, the values of the following fields can
 *   change after a call to @FT_Set_Var_Design_Coordinates (and friends) if
 *   the font contains an 'MVAR' table: `ascender`, `descender`, `height`,
 *   `underline_position`, and `underline_thickness`.
 *
 *   Especially for TrueType fonts see also the documentation for
 *   @FT_Size_Metrics.
 */
/*# The following member variables (down to `underline_thickness`) */
/*# are only relevant to scalable outlines; cf. @FT_Bitmap_Size    */
/*# for bitmap fonts.                                              */
/*@private begin */
/* face-specific auto-hinter data */
/* unused                         */
/*@private end */
/* *************************************************************************
 *
 * @enum:
 *   FT_FACE_FLAG_XXX
 *
 * @description:
 *   A list of bit flags used in the `face_flags` field of the @FT_FaceRec
 *   structure.  They inform client applications of properties of the
 *   corresponding face.
 *
 * @values:
 *   FT_FACE_FLAG_SCALABLE ::
 *     The face contains outline glyphs.  Note that a face can contain
 *     bitmap strikes also, i.e., a face can have both this flag and
 *     @FT_FACE_FLAG_FIXED_SIZES set.
 *
 *   FT_FACE_FLAG_FIXED_SIZES ::
 *     The face contains bitmap strikes.  See also the `num_fixed_sizes`
 *     and `available_sizes` fields of @FT_FaceRec.
 *
 *   FT_FACE_FLAG_FIXED_WIDTH ::
 *     The face contains fixed-width characters (like Courier, Lucida,
 *     MonoType, etc.).
 *
 *   FT_FACE_FLAG_SFNT ::
 *     The face uses the SFNT storage scheme.  For now, this means TrueType
 *     and OpenType.
 *
 *   FT_FACE_FLAG_HORIZONTAL ::
 *     The face contains horizontal glyph metrics.  This should be set for
 *     all common formats.
 *
 *   FT_FACE_FLAG_VERTICAL ::
 *     The face contains vertical glyph metrics.  This is only available in
 *     some formats, not all of them.
 *
 *   FT_FACE_FLAG_KERNING ::
 *     The face contains kerning information.  If set, the kerning distance
 *     can be retrieved using the function @FT_Get_Kerning.  Otherwise the
 *     function always return the vector (0,0).  Note that FreeType doesn't
 *     handle kerning data from the SFNT 'GPOS' table (as present in many
 *     OpenType fonts).
 *
 *   FT_FACE_FLAG_FAST_GLYPHS ::
 *     THIS FLAG IS DEPRECATED.  DO NOT USE OR TEST IT.
 *
 *   FT_FACE_FLAG_MULTIPLE_MASTERS ::
 *     The face contains multiple masters and is capable of interpolating
 *     between them.  Supported formats are Adobe MM, TrueType GX, and
 *     OpenType variation fonts.
 *
 *     See section @multiple_masters for API details.
 *
 *   FT_FACE_FLAG_GLYPH_NAMES ::
 *     The face contains glyph names, which can be retrieved using
 *     @FT_Get_Glyph_Name.  Note that some TrueType fonts contain broken
 *     glyph name tables.  Use the function @FT_Has_PS_Glyph_Names when
 *     needed.
 *
 *   FT_FACE_FLAG_EXTERNAL_STREAM ::
 *     Used internally by FreeType to indicate that a face's stream was
 *     provided by the client application and should not be destroyed when
 *     @FT_Done_Face is called.  Don't read or test this flag.
 *
 *   FT_FACE_FLAG_HINTER ::
 *     The font driver has a hinting machine of its own.  For example, with
 *     TrueType fonts, it makes sense to use data from the SFNT 'gasp'
 *     table only if the native TrueType hinting engine (with the bytecode
 *     interpreter) is available and active.
 *
 *   FT_FACE_FLAG_CID_KEYED ::
 *     The face is CID-keyed.  In that case, the face is not accessed by
 *     glyph indices but by CID values.  For subsetted CID-keyed fonts this
 *     has the consequence that not all index values are a valid argument
 *     to @FT_Load_Glyph.  Only the CID values for which corresponding
 *     glyphs in the subsetted font exist make `FT_Load_Glyph` return
 *     successfully; in all other cases you get an
 *     `FT_Err_Invalid_Argument` error.
 *
 *     Note that CID-keyed fonts that are in an SFNT wrapper (this is, all
 *     OpenType/CFF fonts) don't have this flag set since the glyphs are
 *     accessed in the normal way (using contiguous indices); the
 *     'CID-ness' isn't visible to the application.
 *
 *   FT_FACE_FLAG_TRICKY ::
 *     The face is 'tricky', this is, it always needs the font format's
 *     native hinting engine to get a reasonable result.  A typical example
 *     is the old Chinese font `mingli.ttf` (but not `mingliu.ttc`) that
 *     uses TrueType bytecode instructions to move and scale all of its
 *     subglyphs.
 *
 *     It is not possible to auto-hint such fonts using
 *     @FT_LOAD_FORCE_AUTOHINT; it will also ignore @FT_LOAD_NO_HINTING.
 *     You have to set both @FT_LOAD_NO_HINTING and @FT_LOAD_NO_AUTOHINT to
 *     really disable hinting; however, you probably never want this except
 *     for demonstration purposes.
 *
 *     Currently, there are about a dozen TrueType fonts in the list of
 *     tricky fonts; they are hard-coded in file `ttobjs.c`.
 *
 *   FT_FACE_FLAG_COLOR ::
 *     [Since 2.5.1] The face has color glyph tables.  See @FT_LOAD_COLOR
 *     for more information.
 *
 *   FT_FACE_FLAG_VARIATION ::
 *     [Since 2.9] Set if the current face (or named instance) has been
 *     altered with @FT_Set_MM_Design_Coordinates,
 *     @FT_Set_Var_Design_Coordinates, or @FT_Set_Var_Blend_Coordinates.
 *     This flag is unset by a call to @FT_Set_Named_Instance.
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_HAS_HORIZONTAL
 *
 * @description:
 *   A macro that returns true whenever a face object contains horizontal
 *   metrics (this is true for all font formats though).
 *
 * @also:
 *   @FT_HAS_VERTICAL can be used to check for vertical metrics.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_HAS_VERTICAL
 *
 * @description:
 *   A macro that returns true whenever a face object contains real
 *   vertical metrics (and not only synthesized ones).
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_HAS_KERNING
 *
 * @description:
 *   A macro that returns true whenever a face object contains kerning data
 *   that can be accessed with @FT_Get_Kerning.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_IS_SCALABLE
 *
 * @description:
 *   A macro that returns true whenever a face object contains a scalable
 *   font face (true for TrueType, Type~1, Type~42, CID, OpenType/CFF, and
 *   PFR font formats).
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_IS_SFNT
 *
 * @description:
 *   A macro that returns true whenever a face object contains a font whose
 *   format is based on the SFNT storage scheme.  This usually means:
 *   TrueType fonts, OpenType fonts, as well as SFNT-based embedded bitmap
 *   fonts.
 *
 *   If this macro is true, all functions defined in @FT_SFNT_NAMES_H and
 *   @FT_TRUETYPE_TABLES_H are available.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_IS_FIXED_WIDTH
 *
 * @description:
 *   A macro that returns true whenever a face object contains a font face
 *   that contains fixed-width (or 'monospace', 'fixed-pitch', etc.)
 *   glyphs.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_HAS_FIXED_SIZES
 *
 * @description:
 *   A macro that returns true whenever a face object contains some
 *   embedded bitmaps.  See the `available_sizes` field of the @FT_FaceRec
 *   structure.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_HAS_FAST_GLYPHS
 *
 * @description:
 *   Deprecated.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_HAS_GLYPH_NAMES
 *
 * @description:
 *   A macro that returns true whenever a face object contains some glyph
 *   names that can be accessed through @FT_Get_Glyph_Name.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_HAS_MULTIPLE_MASTERS
 *
 * @description:
 *   A macro that returns true whenever a face object contains some
 *   multiple masters.  The functions provided by @FT_MULTIPLE_MASTERS_H
 *   are then available to choose the exact design you want.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_IS_NAMED_INSTANCE
 *
 * @description:
 *   A macro that returns true whenever a face object is a named instance
 *   of a GX or OpenType variation font.
 *
 *   [Since 2.9] Changing the design coordinates with
 *   @FT_Set_Var_Design_Coordinates or @FT_Set_Var_Blend_Coordinates does
 *   not influence the return value of this macro (only
 *   @FT_Set_Named_Instance does that).
 *
 * @since:
 *   2.7
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_IS_VARIATION
 *
 * @description:
 *   A macro that returns true whenever a face object has been altered by
 *   @FT_Set_MM_Design_Coordinates, @FT_Set_Var_Design_Coordinates, or
 *   @FT_Set_Var_Blend_Coordinates.
 *
 * @since:
 *   2.9
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_IS_CID_KEYED
 *
 * @description:
 *   A macro that returns true whenever a face object contains a CID-keyed
 *   font.  See the discussion of @FT_FACE_FLAG_CID_KEYED for more details.
 *
 *   If this macro is true, all functions defined in @FT_CID_H are
 *   available.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_IS_TRICKY
 *
 * @description:
 *   A macro that returns true whenever a face represents a 'tricky' font.
 *   See the discussion of @FT_FACE_FLAG_TRICKY for more details.
 *
 */
/* *************************************************************************
 *
 * @macro:
 *   FT_HAS_COLOR
 *
 * @description:
 *   A macro that returns true whenever a face object contains tables for
 *   color glyphs.
 *
 * @since:
 *   2.5.1
 *
 */
/* *************************************************************************
 *
 * @enum:
 *   FT_STYLE_FLAG_XXX
 *
 * @description:
 *   A list of bit flags to indicate the style of a given face.  These are
 *   used in the `style_flags` field of @FT_FaceRec.
 *
 * @values:
 *   FT_STYLE_FLAG_ITALIC ::
 *     The face style is italic or oblique.
 *
 *   FT_STYLE_FLAG_BOLD ::
 *     The face is bold.
 *
 * @note:
 *   The style information as provided by FreeType is very basic.  More
 *   details are beyond the scope and should be done on a higher level (for
 *   example, by analyzing various fields of the 'OS/2' table in SFNT based
 *   fonts).
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Size_Internal
 *
 * @description:
 *   An opaque handle to an `FT_Size_InternalRec` structure, used to model
 *   private data of a given @FT_Size object.
 */
/* *************************************************************************
 *
 * @struct:
 *   FT_Size_Metrics
 *
 * @description:
 *   The size metrics structure gives the metrics of a size object.
 *
 * @fields:
 *   x_ppem ::
 *     The width of the scaled EM square in pixels, hence the term 'ppem'
 *     (pixels per EM).  It is also referred to as 'nominal width'.
 *
 *   y_ppem ::
 *     The height of the scaled EM square in pixels, hence the term 'ppem'
 *     (pixels per EM).  It is also referred to as 'nominal height'.
 *
 *   x_scale ::
 *     A 16.16 fractional scaling value to convert horizontal metrics from
 *     font units to 26.6 fractional pixels.  Only relevant for scalable
 *     font formats.
 *
 *   y_scale ::
 *     A 16.16 fractional scaling value to convert vertical metrics from
 *     font units to 26.6 fractional pixels.  Only relevant for scalable
 *     font formats.
 *
 *   ascender ::
 *     The ascender in 26.6 fractional pixels, rounded up to an integer
 *     value.  See @FT_FaceRec for the details.
 *
 *   descender ::
 *     The descender in 26.6 fractional pixels, rounded down to an integer
 *     value.  See @FT_FaceRec for the details.
 *
 *   height ::
 *     The height in 26.6 fractional pixels, rounded to an integer value.
 *     See @FT_FaceRec for the details.
 *
 *   max_advance ::
 *     The maximum advance width in 26.6 fractional pixels, rounded to an
 *     integer value.  See @FT_FaceRec for the details.
 *
 * @note:
 *   The scaling values, if relevant, are determined first during a size
 *   changing operation.  The remaining fields are then set by the driver.
 *   For scalable formats, they are usually set to scaled values of the
 *   corresponding fields in @FT_FaceRec.  Some values like ascender or
 *   descender are rounded for historical reasons; more precise values (for
 *   outline fonts) can be derived by scaling the corresponding @FT_FaceRec
 *   values manually, with code similar to the following.
 *
 *   ```
 *     scaled_ascender = FT_MulFix( face->ascender,
 *                                  size_metrics->y_scale );
 *   ```
 *
 *   Note that due to glyph hinting and the selected rendering mode these
 *   values are usually not exact; consequently, they must be treated as
 *   unreliable with an error margin of at least one pixel!
 *
 *   Indeed, the only way to get the exact metrics is to render _all_
 *   glyphs.  As this would be a definite performance hit, it is up to
 *   client applications to perform such computations.
 *
 *   The `FT_Size_Metrics` structure is valid for bitmap fonts also.
 *
 *
 *   **TrueType fonts with native bytecode hinting**
 *
 *   All applications that handle TrueType fonts with native hinting must
 *   be aware that TTFs expect different rounding of vertical font
 *   dimensions.  The application has to cater for this, especially if it
 *   wants to rely on a TTF's vertical data (for example, to properly align
 *   box characters vertically).
 *
 *   Only the application knows _in advance_ that it is going to use native
 *   hinting for TTFs!  FreeType, on the other hand, selects the hinting
 *   mode not at the time of creating an @FT_Size object but much later,
 *   namely while calling @FT_Load_Glyph.
 *
 *   Here is some pseudo code that illustrates a possible solution.
 *
 *   ```
 *     font_format = FT_Get_Font_Format( face );
 *
 *     if ( !strcmp( font_format, "TrueType" ) &&
 *          do_native_bytecode_hinting         )
 *     {
 *       ascender  = ROUND( FT_MulFix( face->ascender,
 *                                     size_metrics->y_scale ) );
 *       descender = ROUND( FT_MulFix( face->descender,
 *                                     size_metrics->y_scale ) );
 *     }
 *     else
 *     {
 *       ascender  = size_metrics->ascender;
 *       descender = size_metrics->descender;
 *     }
 *
 *     height      = size_metrics->height;
 *     max_advance = size_metrics->max_advance;
 *   ```
 */
/* horizontal pixels per EM               */
/* vertical pixels per EM                 */
/* scaling values used to convert font    */
/* units to 26.6 fractional pixels        */
/* ascender in 26.6 frac. pixels          */
/* descender in 26.6 frac. pixels         */
/* text height in 26.6 frac. pixels       */
/* max horizontal advance, in 26.6 pixels */
/* *************************************************************************
 *
 * @struct:
 *   FT_SizeRec
 *
 * @description:
 *   FreeType root size class structure.  A size object models a face
 *   object at a given size.
 *
 * @fields:
 *   face ::
 *     Handle to the parent face object.
 *
 *   generic ::
 *     A typeless pointer, unused by the FreeType library or any of its
 *     drivers.  It can be used by client applications to link their own
 *     data to each size object.
 *
 *   metrics ::
 *     Metrics for this size object.  This field is read-only.
 */
/* parent face object              */
/* generic pointer for client uses */
/* size metrics                    */
/* *************************************************************************
 *
 * @struct:
 *   FT_SubGlyph
 *
 * @description:
 *   The subglyph structure is an internal object used to describe
 *   subglyphs (for example, in the case of composites).
 *
 * @note:
 *   The subglyph implementation is not part of the high-level API, hence
 *   the forward structure declaration.
 *
 *   You can however retrieve subglyph information with
 *   @FT_Get_SubGlyph_Info.
 */
/* *************************************************************************
 *
 * @type:
 *   FT_Slot_Internal
 *
 * @description:
 *   An opaque handle to an `FT_Slot_InternalRec` structure, used to model
 *   private data of a given @FT_GlyphSlot object.
 */
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
        _tt_abort(b"DoAATLayout called for non-AAT font\x00" as *const u8 as *const libc::c_char);
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
                    _tt_abort(
                        b"FreeType initialization failed; error %d\x00" as *const u8
                            as *const libc::c_char,
                        error,
                    );
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
