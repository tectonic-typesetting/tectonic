#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::mfree;
use crate::stub_icu as icu;
use crate::xetex_xetexd::print_c_string;
use crate::{streq_ptr, strstartswith};
use crate::{ttstub_input_close, ttstub_input_get_size, ttstub_input_open, ttstub_input_read};
use libc::free;

#[cfg(not(target_os = "macos"))]
extern "C" {
    pub type _FcPattern;
}
#[cfg(not(target_os = "macos"))]
pub type FcPattern = _FcPattern;

/// PlatformFontRef matches C++
#[cfg(not(target_os = "macos"))]
pub type PlatformFontRef = *mut FcPattern;
#[cfg(target_os = "macos")]
pub type PlatformFontRef = CTFontDescriptorRef;

#[cfg(target_os = "macos")]
use super::xetex_aatfont as aat;
use super::xetex_aatfont::cf_prelude::{
    kCFNumberFloatType, kCTFontAttributeName, kCTForegroundColorAttributeName,
    kCTVerticalFormsAttributeName, CFBooleanRef, CFDictionaryGetValue, CFDictionaryRef,
    CFNumberGetValue, CFNumberRef, CFNumberType, CFRelease, CFTypeRef, CGAffineTransform,
    CGColorGetComponents, CGColorRef, CGFloat, CTFontDescriptorRef, CTFontGetMatrix, CTFontGetSize,
    CTFontRef,
};

extern "C" {
    pub type XeTeXFont_rec;
    pub type XeTeXLayoutEngine_rec;
    pub type Opaque_TECkit_Converter;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    #[no_mangle]
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    #[no_mangle]
    fn strdup(_: *const i8) -> *mut i8;
    #[no_mangle]
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    /* tectonic/core-memory.h: basic dynamic memory helpers
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    #[no_mangle]
    fn xstrdup(s: *const i8) -> *mut i8;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xrealloc(old_address: *mut libc::c_void, new_size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    pub fn xcalloc(nelem: size_t, elsize: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn hb_tag_from_string(str: *const i8, len: i32) -> hb_tag_t;
    #[no_mangle]
    fn getCachedGlyphBBox(fontID: u16, glyphID: u16, bbox: *mut GlyphBBox) -> i32;
    #[no_mangle]
    fn cacheGlyphBBox(fontID: u16, glyphID: u16, bbox: *const GlyphBBox);
    #[no_mangle]
    fn get_cp_code(fontNum: i32, code: u32, side: i32) -> i32;
    #[no_mangle]
    fn maketexstring(s: *const i8) -> i32;
    #[no_mangle]
    fn getDefaultDirection(engine: XeTeXLayoutEngine) -> i32;
    #[no_mangle]
    fn createFont(fontRef: PlatformFontRef, pointSize: Fixed) -> XeTeXFont;
    #[no_mangle]
    fn getAscentAndDescent(engine: XeTeXLayoutEngine, ascent: *mut f32, descent: *mut f32);
    #[no_mangle]
    fn setFontLayoutDir(font: XeTeXFont, vertical: i32);
    #[no_mangle]
    fn layoutChars(
        engine: XeTeXLayoutEngine,
        chars: *mut u16,
        offset: i32,
        count: i32,
        max: i32,
        rightToLeft: bool,
    ) -> i32;
    #[no_mangle]
    fn getPointSize(engine: XeTeXLayoutEngine) -> f32;
    #[no_mangle]
    fn getGlyphPositions(engine: XeTeXLayoutEngine, positions: *mut FloatPoint);
    #[no_mangle]
    fn getGlyphAdvances(engine: XeTeXLayoutEngine, advances: *mut f32);
    #[no_mangle]
    fn getGlyphs(engine: XeTeXLayoutEngine, glyphs: *mut u32);
    #[no_mangle]
    fn findFontByName(name: *const i8, var: *mut i8, size: f64) -> PlatformFontRef;
    #[no_mangle]
    fn getReqEngine() -> i8;
    #[no_mangle]
    fn setReqEngine(reqEngine: i8);
    #[no_mangle]
    fn getFullName(fontRef: PlatformFontRef) -> *const i8;
    #[no_mangle]
    fn getFontFilename(engine: XeTeXLayoutEngine, index: *mut u32) -> *mut i8;
    #[no_mangle]
    fn getDesignSize(font: XeTeXFont) -> f64;
    #[no_mangle]
    fn deleteFont(font: XeTeXFont);
    #[no_mangle]
    fn getSlant(font: XeTeXFont) -> Fixed;
    #[no_mangle]
    fn getFontTablePtr(font: XeTeXFont, tableTag: u32) -> *mut libc::c_void;
    #[no_mangle]
    fn countScripts(font: XeTeXFont) -> u32;
    #[no_mangle]
    fn countLanguages(font: XeTeXFont, script: hb_tag_t) -> u32;
    #[no_mangle]
    fn countFeatures(font: XeTeXFont, script: hb_tag_t, language: hb_tag_t) -> u32;
    #[no_mangle]
    fn countGlyphs(font: XeTeXFont) -> u32;
    #[no_mangle]
    fn getIndScript(font: XeTeXFont, index: u32) -> hb_tag_t;
    #[no_mangle]
    fn getIndLanguage(font: XeTeXFont, script: hb_tag_t, index: u32) -> hb_tag_t;
    #[no_mangle]
    fn getIndFeature(font: XeTeXFont, script: hb_tag_t, language: hb_tag_t, index: u32)
        -> hb_tag_t;
    #[no_mangle]
    fn getGlyphWidth(font: XeTeXFont, gid: u32) -> f32;
    #[no_mangle]
    fn createFontFromFile(filename: *const i8, index: i32, pointSize: Fixed) -> XeTeXFont;
    #[no_mangle]
    fn getCapAndXHeight(engine: XeTeXLayoutEngine, capheight: *mut f32, xheight: *mut f32);
    #[no_mangle]
    fn getEmboldenFactor(engine: XeTeXLayoutEngine) -> f32;
    #[no_mangle]
    fn getSlantFactor(engine: XeTeXLayoutEngine) -> f32;
    #[no_mangle]
    fn getExtendFactor(engine: XeTeXLayoutEngine) -> f32;
    #[no_mangle]
    fn getFontRef(engine: XeTeXLayoutEngine) -> PlatformFontRef;
    #[no_mangle]
    fn getFont(engine: XeTeXLayoutEngine) -> XeTeXFont;
    #[no_mangle]
    fn deleteLayoutEngine(engine: XeTeXLayoutEngine);
    #[no_mangle]
    fn createLayoutEngine(
        fontRef: PlatformFontRef,
        font: XeTeXFont,
        script: hb_tag_t,
        language: *mut i8,
        features: *mut hb_feature_t,
        nFeatures: i32,
        shapers: *mut *mut i8,
        rgbValue: u32,
        extend: f32,
        slant: f32,
        embolden: f32,
    ) -> XeTeXLayoutEngine;
    /* graphite interface functions... */
    #[no_mangle]
    fn findGraphiteFeature(
        engine: XeTeXLayoutEngine,
        s: *const i8,
        e: *const i8,
        f: *mut hb_tag_t,
        v: *mut i32,
    ) -> bool;
    #[no_mangle]
    fn findNextGraphiteBreak() -> i32;
    #[no_mangle]
    fn initGraphiteBreaking(engine: XeTeXLayoutEngine, txtPtr: *const u16, txtLen: i32) -> bool;
    #[no_mangle]
    fn getFontCharRange(engine: XeTeXLayoutEngine, reqFirst: i32) -> i32;
    #[no_mangle]
    fn getGlyphName(font: XeTeXFont, gid: u16, len: *mut i32) -> *const i8;
    #[no_mangle]
    fn mapGlyphToIndex(engine: XeTeXLayoutEngine, glyphName: *const i8) -> i32;
    #[no_mangle]
    fn mapCharToGlyph(engine: XeTeXLayoutEngine, charCode: u32) -> u32;
    #[no_mangle]
    fn getGlyphItalCorr(engine: XeTeXLayoutEngine, glyphID: u32) -> f32;
    #[no_mangle]
    fn getGlyphSidebearings(engine: XeTeXLayoutEngine, glyphID: u32, lsb: *mut f32, rsb: *mut f32);
    #[no_mangle]
    fn getGlyphHeightDepth(
        engine: XeTeXLayoutEngine,
        glyphID: u32,
        height: *mut f32,
        depth: *mut f32,
    );
    #[no_mangle]
    fn getGlyphWidthFromEngine(engine: XeTeXLayoutEngine, glyphID: u32) -> f32;
    #[no_mangle]
    fn getGlyphBounds(engine: XeTeXLayoutEngine, glyphID: u32, bbox: *mut GlyphBBox);
    #[no_mangle]
    fn getRgbValue(engine: XeTeXLayoutEngine) -> u32;
    #[no_mangle]
    fn countGraphiteFeatures(engine: XeTeXLayoutEngine) -> u32;
    #[no_mangle]
    fn getGraphiteFeatureCode(engine: XeTeXLayoutEngine, index: u32) -> u32;
    #[no_mangle]
    fn countGraphiteFeatureSettings(engine: XeTeXLayoutEngine, feature: u32) -> u32;
    #[no_mangle]
    fn getGraphiteFeatureSettingCode(engine: XeTeXLayoutEngine, feature: u32, index: u32) -> u32;
    #[no_mangle]
    fn getGraphiteFeatureDefaultSetting(engine: XeTeXLayoutEngine, feature: u32) -> u32;
    #[no_mangle]
    fn getGraphiteFeatureLabel(engine: XeTeXLayoutEngine, feature: u32) -> *mut i8;
    #[no_mangle]
    fn getGraphiteFeatureSettingLabel(
        engine: XeTeXLayoutEngine,
        feature: u32,
        setting: u32,
    ) -> *mut i8;
    #[no_mangle]
    fn findGraphiteFeatureNamed(engine: XeTeXLayoutEngine, name: *const i8, namelength: i32)
        -> i64;
    #[no_mangle]
    fn findGraphiteFeatureSettingNamed(
        engine: XeTeXLayoutEngine,
        feature: u32,
        name: *const i8,
        namelength: i32,
    ) -> i64;
    /* not the MS compiler, so try Metrowerks' platform macros */
    /* this seems to be needed for a gcc-mingw32 build to work... */
    /*
        Create a converter object from a compiled mapping
    */
    #[no_mangle]
    fn TECkit_CreateConverter(
        mapping: *mut Byte,
        mappingSize: UInt32,
        mapForward: Byte,
        sourceForm: UInt16,
        targetForm: UInt16,
        converter: *mut TECkit_Converter,
    ) -> TECkit_Status;
    #[no_mangle]
    fn TECkit_ConvertBuffer(
        converter: TECkit_Converter,
        inBuffer: *const Byte,
        inLength: UInt32,
        inUsed: *mut UInt32,
        outBuffer: *mut Byte,
        outLength: UInt32,
        outUsed: *mut UInt32,
        inputIsComplete: Byte,
    ) -> TECkit_Status;
    #[no_mangle]
    fn gr_label_destroy(label: *mut libc::c_void);
    #[no_mangle]
    fn gettexstring(_: str_number) -> *mut i8;
    #[no_mangle]
    pub static mut name_of_file: *mut i8;
    #[no_mangle]
    pub static mut name_length: i32;
    #[no_mangle]
    static mut font_info: *mut memory_word;
    #[no_mangle]
    static mut font_area: *mut str_number;
    #[no_mangle]
    static mut font_layout_engine: *mut *mut libc::c_void;
    #[no_mangle]
    static mut font_flags: *mut i8;
    #[no_mangle]
    static mut font_letter_space: *mut scaled_t;
    #[no_mangle]
    static mut loaded_font_mapping: *mut libc::c_void;
    #[no_mangle]
    static mut loaded_font_flags: i8;
    #[no_mangle]
    static mut loaded_font_letter_space: scaled_t;
    #[no_mangle]
    static mut loaded_font_design_size: scaled_t;
    #[no_mangle]
    static mut mapped_text: *mut UTF16_code;
    #[no_mangle]
    static mut xdv_buffer: *mut i8;
    #[no_mangle]
    static mut height_base: *mut i32;
    #[no_mangle]
    static mut depth_base: *mut i32;
    #[no_mangle]
    static mut param_base: *mut i32;
    #[no_mangle]
    static mut native_font_type_flag: i32;
    #[no_mangle]
    fn begin_diagnostic();
    #[no_mangle]
    fn end_diagnostic(blank_line: bool);
    #[no_mangle]
    fn font_feature_warning(
        featureNameP: *const libc::c_void,
        featLen: i32,
        settingNameP: *const libc::c_void,
        setLen: i32,
    );
    #[no_mangle]
    fn font_mapping_warning(
        mappingNameP: *const libc::c_void,
        mappingNameLen: i32,
        warningType: i32,
    );
    #[no_mangle]
    fn get_tracing_fonts_state() -> i32;
    #[no_mangle]
    fn print_raw_char(s: UTF16_code, incr_offset: bool);
    #[no_mangle]
    fn print_char(s: i32);
    #[no_mangle]
    fn print_nl(s: str_number);
    #[no_mangle]
    fn print_int(n: i32);
    /* xetex-pagebuilder */
    /* xetex-scaledmath */
    #[no_mangle]
    fn xn_over_d(x: scaled_t, n: i32, d: i32) -> scaled_t;
}

pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;

use crate::TTInputFormat;

pub type rust_input_handle_t = *mut libc::c_void;
pub type hb_tag_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hb_feature_t {
    pub tag: hb_tag_t,
    pub value: u32,
    pub start: u32,
    pub end: u32,
}
pub type scaled_t = i32;
pub type SInt32 = i32;

#[cfg(not(target_os = "macos"))]
pub type Fixed = scaled_t;
#[cfg(target_os = "macos")]
pub type Fixed = SInt32;

#[derive(Copy, Clone)]
#[cfg_attr(not(target_os = "macos"), repr(C))]
#[cfg_attr(target_os = "macos", repr(C, packed(2)))]
pub struct FixedPoint {
    pub x: Fixed,
    pub y: Fixed,
}
pub type Boolean = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FloatPoint {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphBBox {
    pub xMin: f32,
    pub yMin: f32,
    pub xMax: f32,
    pub yMax: f32,
}

pub type XeTeXFont = *mut XeTeXFont_rec;
pub type XeTeXLayoutEngine = *mut XeTeXLayoutEngine_rec;
pub type Byte = UInt8;
pub type UInt8 = u8;
/*------------------------------------------------------------------------
Copyright (C) 2002-2014 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: TECkit_Engine.h
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:
    Public API to the TECkit conversion engine.
-------------------------------------------------------------------------*/
/*
    TECkit_Engine.h

    Public API to the TECkit encoding conversion library.

    18-Jan-2008     jk  added EXPORTED to declarations, for mingw32 cross-build
    18-Mar-2005     jk  moved version number to TECkit_Common.h as it is shared with the compiler
    19-Mar-2004     jk  updated minor version for 2.2 engine (improved matching functionality)
    23-Sep-2003     jk  updated for version 2.1 - new "...Opt" APIs
     5-Jul-2002     jk  corrected placement of WINAPI to keep MS compiler happy
    14-May-2002     jk  added WINAPI to function declarations
    22-Dec-2001     jk  initial version
*/
/* formFlags bits for normalization; if none are set, then this side of the mapping is normalization-form-agnostic on input, and may generate an unspecified mixture */
/* expects fully composed text (NC) */
/* expects fully decomposed text (NCD) */
/* generates fully composed text (NC) */
/* generates fully decomposed text (NCD) */
/* if VisualOrder is set, this side of the mapping deals with visual-order rather than logical-order text (only relevant for bidi scripts) */
/* visual rather than logical order */
/* if Unicode is set, the encoding is Unicode on this side of the mapping */
/* this is Unicode rather than a byte encoding */
/* required names */
/* "source" or LHS encoding name, e.g. "SIL-EEG_URDU-2001" */
/* "destination" or RHS encoding name, e.g. "UNICODE-3-1" */
/* source encoding description, e.g. "SIL East Eurasia Group Extended Urdu (Mac OS)" */
/* destination description, e.g. "Unicode 3.1" */
/* additional recommended names (parallel to UTR-22) */
/* "1.0b1" */
/* "mailto:nrsi@sil.org" */
/* "SIL International" */
/* "Greek (Galatia)" */
/* "(c)2002 SIL International" */
/* additional name IDs may be defined in the future */
/*
    encoding form options for TECkit_CreateConverter
*/
/*
    end of text value for TECkit_DataSource functions to return
*/
/*
    A converter object is an opaque pointer
*/
pub type TECkit_Converter = *mut Opaque_TECkit_Converter;
pub type str_number = i32;
/* tectonic/xetex-xetexd.h -- many, many XeTeX symbol definitions
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/
/* Extra stuff used in various change files for various reasons.  */
/* Array allocations. Add 1 to size to account for Pascal indexing convention. */
/*11:*/
/*18: */
pub type UTF16_code = u16;
/*
    all public functions return a status code
*/
pub type TECkit_Status = i64;
/*------------------------------------------------------------------------
Copyright (C) 2002-2016 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: TECkit_Common.h
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:
    Public definitions used by TECkit engine and compiler
-------------------------------------------------------------------------*/
/*
    Common types and defines for the engine and compiler

History:
    16-Sep-2006     jk  updated version to 2.4 (adding new compiler APIs for Bob E)
    23-May-2005     jk  patch for 64-bit architectures (thanks to Ulrik P)
    18-Mar-2005     jk  updated minor version for 2.3 (engine unchanged, XML option in compiler)
    23-Sep-2003     jk  updated for version 2.1 - extended status values
    xx-xxx-2002     jk  version 2.0 initial release
*/
/* 16.16 version number */

pub type UInt16 = u16;
pub type UInt32 = u32;

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
    pub s0: i32,
    pub s1: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: f64,
    pub ptr: *mut libc::c_void,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4_le_t {
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}
pub type UniChar = UInt16;

/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */

/* ***************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew

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

#[inline]
unsafe extern "C" fn SWAP16(p: u16) -> u16 {
    ((p as i32 >> 8i32) + ((p as i32) << 8i32)) as u16
}

#[inline]
unsafe extern "C" fn SWAP32(p: u32) -> u32 {
    (p >> 24i32)
        .wrapping_add(p >> 8i32 & 0xff00_u32)
        .wrapping_add(p << 8i32 & 0xff0000_u32)
        .wrapping_add(p << 24i32)
}

/* xetex-shipout */
/* ***************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009, 2011 by Jonathan Kew
 Copyright (c) 2012-2015 by Khaled Hosny
 Copyright (c) 2012, 2013 by Jiang Jiang

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
/* XeTeX_ext.c
 * additional plain C extensions for XeTeX - mostly platform-neutral
 */
/* for fabs() */

/* OT-related constants we need */
static mut brkIter: *mut icu::UBreakIterator =
    0 as *const icu::UBreakIterator as *mut icu::UBreakIterator;
static mut brkLocaleStrNum: i32 = 0i32;

/* info for each glyph is location (FixedPoint) + glyph ID (u16) */
/* glyph ID field in a glyph_node */
/* For Unicode encoding form interpretation... */
#[no_mangle]
pub unsafe extern "C" fn linebreak_start(
    mut f: i32,
    mut localeStrNum: i32,
    mut text: *mut u16,
    mut textLength: i32,
) {
    let mut status: icu::UErrorCode = icu::U_ZERO_ERROR;
    let mut locale: *mut i8 = gettexstring(localeStrNum);
    if *font_area.offset(f as isize) as u32 == 0xfffeu32
        && streq_ptr(locale, b"G\x00" as *const u8 as *const i8) as i32 != 0
    {
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
        if initGraphiteBreaking(engine, text, textLength) {
            /* user asked for Graphite line breaking and the font supports it */
            return;
        }
    }
    if localeStrNum != brkLocaleStrNum && !brkIter.is_null() {
        icu::ubrk_close(brkIter);
        brkIter = 0 as *mut icu::UBreakIterator
    }
    if brkIter.is_null() {
        brkIter = icu::ubrk_open(
            icu::UBRK_LINE,
            locale,
            0 as *const icu::UChar,
            0i32,
            &mut status,
        );
        if status as i32 > icu::U_ZERO_ERROR as i32 {
            begin_diagnostic();
            print_nl('E' as i32);
            print_c_string(b"rror \x00" as *const u8 as *const i8);
            print_int(status as i32);
            print_c_string(
                b" creating linebreak iterator for locale `\x00" as *const u8 as *const i8,
            );
            print_c_string(locale);
            print_c_string(b"\'; trying default locale `en_us\'.\x00" as *const u8 as *const i8);
            end_diagnostic(1i32 != 0);
            if !brkIter.is_null() {
                icu::ubrk_close(brkIter);
            }
            status = icu::U_ZERO_ERROR;
            brkIter = icu::ubrk_open(
                icu::UBRK_LINE,
                b"en_us\x00" as *const u8 as *const i8,
                0 as *const icu::UChar,
                0i32,
                &mut status,
            )
        }
        free(locale as *mut libc::c_void);
        brkLocaleStrNum = localeStrNum
    }
    if brkIter.is_null() {
        panic!(
            "failed to create linebreak iterator, status={}",
            status as i32
        );
    }
    icu::ubrk_setText(brkIter, text as *mut icu::UChar, textLength, &mut status);
}

#[no_mangle]
pub unsafe extern "C" fn linebreak_next() -> i32 {
    if !brkIter.is_null() {
        icu::ubrk_next(brkIter)
    } else {
        findNextGraphiteBreak()
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_encoding_mode_and_info(mut info: *mut i32) -> i32 {
    /* \XeTeXinputencoding "enc-name"
     *   -> name is packed in |nameoffile| as a C string, starting at [1]
     * Check if it's a built-in name; if not, try to open an ICU converter by that name
     */
    let mut err: icu::UErrorCode = icu::U_ZERO_ERROR;
    let mut cnv: *mut icu::UConverter = 0 as *mut icu::UConverter;
    *info = 0i32;
    if strcasecmp(name_of_file, b"auto\x00" as *const u8 as *const i8) == 0i32 {
        return 0i32;
    }
    if strcasecmp(name_of_file, b"utf8\x00" as *const u8 as *const i8) == 0i32 {
        return 1i32;
    }
    if strcasecmp(name_of_file, b"utf16\x00" as *const u8 as *const i8) == 0i32 {
        /* depends on host platform */
        return 3i32;
    }
    if strcasecmp(name_of_file, b"utf16be\x00" as *const u8 as *const i8) == 0i32 {
        return 2i32;
    }
    if strcasecmp(name_of_file, b"utf16le\x00" as *const u8 as *const i8) == 0i32 {
        return 3i32;
    }
    if strcasecmp(name_of_file, b"bytes\x00" as *const u8 as *const i8) == 0i32 {
        return 4i32;
    }
    /* try for an ICU converter */
    cnv = icu::ucnv_open(name_of_file, &mut err); /* ensure message starts on a new line */
    if cnv.is_null() {
        begin_diagnostic();
        print_nl('U' as i32);
        print_c_string(b"nknown encoding `\x00" as *const u8 as *const i8);
        print_c_string(name_of_file);
        print_c_string(b"\'; reading as raw bytes\x00" as *const u8 as *const i8);
        end_diagnostic(1i32 != 0);
        4i32
    } else {
        icu::ucnv_close(cnv);
        *info = maketexstring(name_of_file);
        5i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_utf8_str(mut string: *const u8, mut len: i32) {
    loop {
        let fresh1 = len;
        len = len - 1;
        if !(fresh1 > 0i32) {
            break;
        }
        let fresh2 = string;
        string = string.offset(1);
        print_raw_char(*fresh2 as UTF16_code, true);
    }
    /* bypass utf-8 encoding done in print_char() */
}

#[no_mangle]
pub unsafe extern "C" fn print_chars(mut string: *const u16, mut len: i32) {
    loop {
        let fresh3 = len;
        len = len - 1;
        if !(fresh3 > 0i32) {
            break;
        }
        let fresh4 = string;
        string = string.offset(1);
        print_char(*fresh4 as i32);
    }
}

unsafe extern "C" fn load_mapping_file(
    mut s: *const i8,
    mut e: *const i8,
    mut byteMapping: i8,
) -> *mut libc::c_void {
    let mut cnv: TECkit_Converter = 0 as TECkit_Converter;
    let mut buffer: *mut i8 =
        xmalloc((e.wrapping_offset_from(s) as i64 + 5i32 as i64) as size_t) as *mut i8;
    let mut map: rust_input_handle_t = 0 as *mut libc::c_void;
    strncpy(buffer, s, e.wrapping_offset_from(s) as i64 as u64);
    *buffer.offset(e.wrapping_offset_from(s) as i64 as isize) = 0_i8;
    strcat(buffer, b".tec\x00" as *const u8 as *const i8);
    map = ttstub_input_open(buffer, TTInputFormat::MISCFONTS, 0i32);
    if !map.is_null() {
        let mut mappingSize: size_t = ttstub_input_get_size(map);
        let mut mapping: *mut Byte = xmalloc(mappingSize) as *mut Byte;
        let mut r: ssize_t = ttstub_input_read(map, mapping as *mut i8, mappingSize);
        if r < 0i32 as i64 || r as size_t != mappingSize {
            _tt_abort(
                b"could not read mapping file \"%s\"\x00" as *const u8 as *const i8,
                buffer,
            );
        }
        ttstub_input_close(map);
        if byteMapping as i32 != 0i32 {
            TECkit_CreateConverter(
                mapping,
                mappingSize as UInt32,
                0i32 as Byte,
                4i32 as UInt16,
                1i32 as UInt16,
                &mut cnv,
            );
        } else {
            TECkit_CreateConverter(
                mapping,
                mappingSize as UInt32,
                1i32 as Byte,
                4i32 as UInt16,
                4i32 as UInt16,
                &mut cnv,
            );
        }
        if cnv.is_null() {
            /* tracing */
            font_mapping_warning(buffer as *const libc::c_void, strlen(buffer) as i32, 2i32);
        /* not loadable */
        } else if get_tracing_fonts_state() > 1i32 {
            font_mapping_warning(buffer as *const libc::c_void, strlen(buffer) as i32, 0i32);
        }
    } else {
        font_mapping_warning(buffer as *const libc::c_void, strlen(buffer) as i32, 1i32);
        /* not found */
    }
    free(buffer as *mut libc::c_void);
    cnv as *mut libc::c_void
}
static mut saved_mapping_name: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub unsafe extern "C" fn check_for_tfm_font_mapping() {
    let mut cp: *mut i8 = strstr(name_of_file, b":mapping=\x00" as *const u8 as *const i8);
    saved_mapping_name = mfree(saved_mapping_name as *mut libc::c_void) as *mut i8;
    if !cp.is_null() {
        *cp = 0_i8;
        cp = cp.offset(9);
        while *cp as i32 != 0 && *cp as i32 <= ' ' as i32 {
            cp = cp.offset(1)
        }
        if *cp != 0 {
            saved_mapping_name = xstrdup(cp)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn load_tfm_font_mapping() -> *mut libc::c_void {
    let mut rval: *mut libc::c_void = 0 as *mut libc::c_void;
    if !saved_mapping_name.is_null() {
        rval = load_mapping_file(
            saved_mapping_name,
            saved_mapping_name.offset(strlen(saved_mapping_name) as isize),
            1_i8,
        );
        saved_mapping_name = mfree(saved_mapping_name as *mut libc::c_void) as *mut i8
    }
    rval
}
#[no_mangle]
pub unsafe extern "C" fn apply_tfm_font_mapping(mut cnv: *mut libc::c_void, mut c: i32) -> i32 {
    let mut in_0: UniChar = c as UniChar;
    let mut out: [Byte; 2] = [0; 2];
    let mut inUsed: UInt32 = 0;
    let mut outUsed: UInt32 = 0;
    /* TECkit_Status status; */
    /* status = */
    TECkit_ConvertBuffer(
        cnv as TECkit_Converter,
        &mut in_0 as *mut UniChar as *const Byte,
        ::std::mem::size_of::<UniChar>() as u64 as UInt32,
        &mut inUsed,
        out.as_mut_ptr(),
        ::std::mem::size_of::<[Byte; 2]>() as u64 as UInt32,
        &mut outUsed,
        1i32 as Byte,
    );
    if outUsed < 1_u32 {
        0i32
    } else {
        out[0] as i32
    }
}
#[no_mangle]
pub unsafe extern "C" fn read_double(mut s: *mut *const i8) -> f64 {
    let mut neg: i32 = 0i32;
    let mut val: f64 = 0.0f64;
    let mut cp: *const i8 = *s;
    while *cp as i32 == ' ' as i32 || *cp as i32 == '\t' as i32 {
        cp = cp.offset(1)
    }
    if *cp as i32 == '-' as i32 {
        neg = 1i32;
        cp = cp.offset(1)
    } else if *cp as i32 == '+' as i32 {
        cp = cp.offset(1)
    }
    while *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
        val = val * 10.0f64 + *cp as i32 as f64 - '0' as i32 as f64;
        cp = cp.offset(1)
    }
    if *cp as i32 == '.' as i32 {
        let mut dec: f64 = 10.0f64;
        cp = cp.offset(1);
        while *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
            val = val + (*cp as i32 - '0' as i32) as f64 / dec;
            cp = cp.offset(1);
            dec = dec * 10.0f64
        }
    }
    *s = cp;
    if neg != 0 {
        -val
    } else {
        val
    }
}
unsafe extern "C" fn read_tag_with_param(mut cp: *const i8, mut param: *mut i32) -> hb_tag_t {
    let mut cp2: *const i8 = 0 as *const i8;
    let mut tag: hb_tag_t = 0;
    cp2 = cp;
    while *cp2 as i32 != 0
        && *cp2 as i32 != ':' as i32
        && *cp2 as i32 != ';' as i32
        && *cp2 as i32 != ',' as i32
        && *cp2 as i32 != '=' as i32
    {
        cp2 = cp2.offset(1)
    }
    tag = hb_tag_from_string(cp, cp2.wrapping_offset_from(cp) as i64 as i32);
    cp = cp2;
    if *cp as i32 == '=' as i32 {
        let mut neg: i32 = 0i32;
        cp = cp.offset(1);
        if *cp as i32 == '-' as i32 {
            neg += 1;
            cp = cp.offset(1)
        }
        while *cp as i32 >= '0' as i32 && *cp as i32 <= '9' as i32 {
            *param = *param * 10i32 + *cp as i32 - '0' as i32;
            cp = cp.offset(1)
        }
        if neg != 0 {
            *param = -*param
        }
    }
    tag
}
#[no_mangle]
pub unsafe extern "C" fn read_rgb_a(mut cp: *mut *const i8) -> u32 {
    let mut rgbValue: u32 = 0_u32;
    let mut alpha: u32 = 0_u32;
    let mut i: i32 = 0;
    i = 0i32;
    while i < 6i32 {
        if **cp as i32 >= '0' as i32 && **cp as i32 <= '9' as i32 {
            rgbValue = (rgbValue << 4i32)
                .wrapping_add(**cp as u32)
                .wrapping_sub('0' as i32 as u32)
        } else if **cp as i32 >= 'A' as i32 && **cp as i32 <= 'F' as i32 {
            rgbValue = (rgbValue << 4i32)
                .wrapping_add(**cp as u32)
                .wrapping_sub('A' as i32 as u32)
                .wrapping_add(10_u32)
        } else if **cp as i32 >= 'a' as i32 && **cp as i32 <= 'f' as i32 {
            rgbValue = (rgbValue << 4i32)
                .wrapping_add(**cp as u32)
                .wrapping_sub('a' as i32 as u32)
                .wrapping_add(10_u32)
        } else {
            return 0xff_u32;
        }
        *cp = (*cp).offset(1);
        i += 1
    }
    rgbValue <<= 8i32;
    i = 0i32;
    while i < 2i32 {
        if **cp as i32 >= '0' as i32 && **cp as i32 <= '9' as i32 {
            alpha = (alpha << 4i32)
                .wrapping_add(**cp as u32)
                .wrapping_sub('0' as i32 as u32)
        } else if **cp as i32 >= 'A' as i32 && **cp as i32 <= 'F' as i32 {
            alpha = (alpha << 4i32)
                .wrapping_add(**cp as u32)
                .wrapping_sub('A' as i32 as u32)
                .wrapping_add(10_u32)
        } else {
            if !(**cp as i32 >= 'a' as i32 && **cp as i32 <= 'f' as i32) {
                break;
            }
            alpha = (alpha << 4i32)
                .wrapping_add(**cp as u32)
                .wrapping_sub('a' as i32 as u32)
                .wrapping_add(10_u32)
        }
        *cp = (*cp).offset(1);
        i += 1
    }
    if i == 2i32 {
        rgbValue = (rgbValue as u32).wrapping_add(alpha) as u32
    } else {
        rgbValue = (rgbValue as u32).wrapping_add(0xff_u32) as u32
    }
    rgbValue
}
#[no_mangle]
pub unsafe extern "C" fn readCommonFeatures(
    mut feat: *const i8,
    mut end: *const i8,
    mut extend: *mut f32,
    mut slant: *mut f32,
    mut embolden: *mut f32,
    mut letterspace: *mut f32,
    mut rgbValue: *mut u32,
) -> i32
// returns 1 to go to next_option, -1 for bad_option, 0 to continue
{
    let mut sep: *const i8 = 0 as *const i8;
    sep = strstartswith(feat, b"mapping\x00" as *const u8 as *const i8);
    if !sep.is_null() {
        if *sep as i32 != '=' as i32 {
            return -1i32;
        }
        loaded_font_mapping = load_mapping_file(sep.offset(1), end, 0_i8);
        return 1i32;
    }
    sep = strstartswith(feat, b"extend\x00" as *const u8 as *const i8);
    if !sep.is_null() {
        if *sep as i32 != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        *extend = read_double(&mut sep) as f32;
        return 1i32;
    }
    sep = strstartswith(feat, b"slant\x00" as *const u8 as *const i8);
    if !sep.is_null() {
        if *sep as i32 != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        *slant = read_double(&mut sep) as f32;
        return 1i32;
    }
    sep = strstartswith(feat, b"embolden\x00" as *const u8 as *const i8);
    if !sep.is_null() {
        if *sep as i32 != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        *embolden = read_double(&mut sep) as f32;
        return 1i32;
    }
    sep = strstartswith(feat, b"letterspace\x00" as *const u8 as *const i8);
    if !sep.is_null() {
        if *sep as i32 != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        *letterspace = read_double(&mut sep) as f32;
        return 1i32;
    }
    sep = strstartswith(feat, b"color\x00" as *const u8 as *const i8);
    if !sep.is_null() {
        let mut s: *const i8 = 0 as *const i8;
        if *sep as i32 != '=' as i32 {
            return -1i32;
        }
        sep = sep.offset(1);
        s = sep;
        *rgbValue = read_rgb_a(&mut sep);
        if sep == s.offset(6) || sep == s.offset(8) {
            loaded_font_flags = (loaded_font_flags as i32 | 0x1i32) as i8
        } else {
            return -1i32;
        }
        return 1i32;
    }
    0i32
}
unsafe extern "C" fn readFeatureNumber(
    mut s: *const i8,
    mut e: *const i8,
    mut f: *mut hb_tag_t,
    mut v: *mut i32,
) -> bool
/* s...e is a "id=setting" string; */ {
    *f = 0i32 as hb_tag_t;
    *v = 0i32;
    if (*s as i32) < '0' as i32 || *s as i32 > '9' as i32 {
        return false;
    }
    while *s as i32 >= '0' as i32 && *s as i32 <= '9' as i32 {
        let fresh5 = s;
        s = s.offset(1);
        *f = (*f)
            .wrapping_mul(10_u32)
            .wrapping_add(*fresh5 as u32)
            .wrapping_sub('0' as i32 as u32)
    }
    while *s as i32 == ' ' as i32 || *s as i32 == '\t' as i32 {
        s = s.offset(1)
    }
    let fresh6 = s;
    s = s.offset(1);
    if *fresh6 as i32 != '=' as i32 {
        /* no setting was specified */
        return false;
    } /* NULL-terminated array */
    if (*s as i32) < '0' as i32 || *s as i32 > '9' as i32 {
        return false;
    }
    while *s as i32 >= '0' as i32 && *s as i32 <= '9' as i32 {
        let fresh7 = s;
        s = s.offset(1);
        *v = *v * 10i32 + *fresh7 as i32 - '0' as i32
    }
    while *s as i32 == ' ' as i32 || *s as i32 == '\t' as i32 {
        s = s.offset(1)
    }
    if s != e {
        return false;
    }
    true
}
unsafe extern "C" fn loadOTfont(
    mut fontRef: PlatformFontRef,
    mut font: XeTeXFont,
    mut scaled_size: Fixed,
    mut cp1: *mut i8,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut engine: XeTeXLayoutEngine = 0 as XeTeXLayoutEngine;
    let mut script: hb_tag_t = (0_u32 & 0xff_u32) << 24i32
        | (0_u32 & 0xff_u32) << 16i32
        | (0_u32 & 0xff_u32) << 8i32
        | 0_u32 & 0xff_u32;
    let mut language: *mut i8 = 0 as *mut i8;
    let mut features: *mut hb_feature_t = 0 as *mut hb_feature_t;
    let mut shapers: *mut *mut i8 = 0 as *mut *mut i8;
    let mut nFeatures: i32 = 0i32;
    let mut nShapers: i32 = 0i32;
    let mut cp2: *mut i8 = 0 as *mut i8;
    let mut cp3: *const i8 = 0 as *const i8;
    let mut tag: hb_tag_t = 0;
    let mut rgbValue: u32 = 0xff_u32;
    let mut extend: f32 = 1.0f64 as f32;
    let mut slant: f32 = 0.0f64 as f32;
    let mut embolden: f32 = 0.0f64 as f32;
    let mut letterspace: f32 = 0.0f64 as f32;
    let mut i: i32 = 0;
    let mut reqEngine: i8 = getReqEngine();
    if reqEngine as i32 == 'O' as i32 || reqEngine as i32 == 'G' as i32 {
        shapers = xrealloc(
            shapers as *mut libc::c_void,
            ((nShapers + 1i32) as u64).wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
        ) as *mut *mut i8;
        if reqEngine as i32 == 'O' as i32 {
            static mut ot_const: [i8; 3] = [111, 116, 0];
            let ref mut fresh8 = *shapers.offset(nShapers as isize);
            *fresh8 = ot_const.as_mut_ptr()
        } else if reqEngine as i32 == 'G' as i32 {
            static mut graphite2_const: [i8; 10] = [103, 114, 97, 112, 104, 105, 116, 101, 50, 0];
            let ref mut fresh9 = *shapers.offset(nShapers as isize);
            *fresh9 = graphite2_const.as_mut_ptr()
        }
        nShapers += 1
    }
    if reqEngine as i32 == 'G' as i32 {
        let mut tmpShapers: [*mut i8; 1] = [*shapers.offset(0)];
        /* create a default engine so we can query the font for Graphite features;
         * because of font caching, it's cheap to discard this and create the real one later */
        engine = createLayoutEngine(
            fontRef,
            font,
            script,
            language,
            features,
            nFeatures,
            tmpShapers.as_mut_ptr(),
            rgbValue,
            extend,
            slant,
            embolden,
        );
        if engine.is_null() {
            return 0 as *mut libc::c_void;
        }
    }
    /* scan the feature string (if any) */
    if !cp1.is_null() {
        while *cp1 != 0 {
            if *cp1 as i32 == ':' as i32 || *cp1 as i32 == ';' as i32 || *cp1 as i32 == ',' as i32 {
                cp1 = cp1.offset(1)
            }
            while *cp1 as i32 == ' ' as i32 || *cp1 as i32 == '\t' as i32 {
                /* skip leading whitespace */
                cp1 = cp1.offset(1)
            }
            if *cp1 as i32 == 0i32 {
                break;
            }
            cp2 = cp1;
            while *cp2 as i32 != 0
                && *cp2 as i32 != ':' as i32
                && *cp2 as i32 != ';' as i32
                && *cp2 as i32 != ',' as i32
            {
                cp2 = cp2.offset(1)
            }
            cp3 = strstartswith(cp1, b"script\x00" as *const u8 as *const i8);
            if !cp3.is_null() {
                if *cp3 as i32 != '=' as i32 {
                    current_block = 10622493848381539643;
                } else {
                    cp3 = cp3.offset(1);
                    script = hb_tag_from_string(cp3, cp2.wrapping_offset_from(cp3) as i64 as i32);
                    current_block = 13857423536159756434;
                }
            } else {
                cp3 = strstartswith(cp1, b"language\x00" as *const u8 as *const i8);
                if !cp3.is_null() {
                    if *cp3 as i32 != '=' as i32 {
                        current_block = 10622493848381539643;
                    } else {
                        cp3 = cp3.offset(1);
                        language =
                            xmalloc((cp2.wrapping_offset_from(cp3) as i64 + 1i32 as i64) as size_t)
                                as *mut i8;
                        *language.offset(cp2.wrapping_offset_from(cp3) as i64 as isize) =
                            '\u{0}' as i32 as i8;
                        memcpy(
                            language as *mut libc::c_void,
                            cp3 as *const libc::c_void,
                            cp2.wrapping_offset_from(cp3) as i64 as u64,
                        );
                        current_block = 13857423536159756434;
                    }
                } else {
                    cp3 = strstartswith(cp1, b"shaper\x00" as *const u8 as *const i8);
                    if !cp3.is_null() {
                        if *cp3 as i32 != '=' as i32 {
                            current_block = 10622493848381539643;
                        } else {
                            cp3 = cp3.offset(1);
                            shapers = xrealloc(
                                shapers as *mut libc::c_void,
                                ((nShapers + 1i32) as u64)
                                    .wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
                            ) as *mut *mut i8;
                            /* some dumb systems have no strndup() */
                            let ref mut fresh10 = *shapers.offset(nShapers as isize);
                            *fresh10 = strdup(cp3);
                            *(*shapers.offset(nShapers as isize))
                                .offset(cp2.wrapping_offset_from(cp3) as i64 as isize) =
                                '\u{0}' as i32 as i8;
                            nShapers += 1;
                            current_block = 13857423536159756434;
                        }
                    } else {
                        i = readCommonFeatures(
                            cp1,
                            cp2,
                            &mut extend,
                            &mut slant,
                            &mut embolden,
                            &mut letterspace,
                            &mut rgbValue,
                        );
                        if i == 1i32 {
                            current_block = 13857423536159756434;
                        } else if i == -1i32 {
                            current_block = 10622493848381539643;
                        } else {
                            if reqEngine as i32 == 'G' as i32 {
                                let mut value: i32 = 0i32;
                                if readFeatureNumber(cp1, cp2, &mut tag, &mut value) as i32 != 0
                                    || findGraphiteFeature(engine, cp1, cp2, &mut tag, &mut value)
                                        as i32
                                        != 0
                                {
                                    features = xrealloc(
                                        features as *mut libc::c_void,
                                        ((nFeatures + 1i32) as u64).wrapping_mul(
                                            ::std::mem::size_of::<hb_feature_t>() as u64,
                                        ),
                                    )
                                        as *mut hb_feature_t;
                                    (*features.offset(nFeatures as isize)).tag = tag;
                                    (*features.offset(nFeatures as isize)).value = value as u32;
                                    (*features.offset(nFeatures as isize)).start = 0_u32;
                                    (*features.offset(nFeatures as isize)).end = -1i32 as u32;
                                    nFeatures += 1;
                                    current_block = 13857423536159756434;
                                } else {
                                    current_block = 15669289850109000831;
                                }
                            } else {
                                current_block = 15669289850109000831;
                            }
                            match current_block {
                                13857423536159756434 => {}
                                _ => {
                                    if *cp1 as i32 == '+' as i32 {
                                        let mut param: i32 = 0i32;
                                        tag = read_tag_with_param(cp1.offset(1), &mut param);
                                        features = xrealloc(
                                            features as *mut libc::c_void,
                                            ((nFeatures + 1i32) as u64).wrapping_mul(
                                                ::std::mem::size_of::<hb_feature_t>() as u64,
                                            ),
                                        )
                                            as *mut hb_feature_t;
                                        (*features.offset(nFeatures as isize)).tag = tag;
                                        (*features.offset(nFeatures as isize)).start = 0_u32;
                                        (*features.offset(nFeatures as isize)).end = -1i32 as u32;
                                        // for backward compatibility with pre-0.9999 where feature
                                        // indices started from 0
                                        if param >= 0i32 {
                                            param += 1
                                        }
                                        (*features.offset(nFeatures as isize)).value = param as u32;
                                        nFeatures += 1;
                                        current_block = 13857423536159756434;
                                    } else if *cp1 as i32 == '-' as i32 {
                                        cp1 = cp1.offset(1);
                                        tag = hb_tag_from_string(
                                            cp1,
                                            cp2.wrapping_offset_from(cp1) as i64 as i32,
                                        );
                                        features = xrealloc(
                                            features as *mut libc::c_void,
                                            ((nFeatures + 1i32) as u64).wrapping_mul(
                                                ::std::mem::size_of::<hb_feature_t>() as u64,
                                            ),
                                        )
                                            as *mut hb_feature_t;
                                        (*features.offset(nFeatures as isize)).tag = tag;
                                        (*features.offset(nFeatures as isize)).start = 0_u32;
                                        (*features.offset(nFeatures as isize)).end = -1i32 as u32;
                                        (*features.offset(nFeatures as isize)).value = 0_u32;
                                        nFeatures += 1;
                                        current_block = 13857423536159756434;
                                    } else if !strstartswith(
                                        cp1,
                                        b"vertical\x00" as *const u8 as *const i8,
                                    )
                                    .is_null()
                                    {
                                        cp3 = cp2;
                                        if *cp3 as i32 == ';' as i32
                                            || *cp3 as i32 == ':' as i32
                                            || *cp3 as i32 == ',' as i32
                                        {
                                            cp3 = cp3.offset(-1)
                                        }
                                        while *cp3 as i32 == '\u{0}' as i32
                                            || *cp3 as i32 == ' ' as i32
                                            || *cp3 as i32 == '\t' as i32
                                        {
                                            cp3 = cp3.offset(-1)
                                        }
                                        if *cp3 != 0 {
                                            cp3 = cp3.offset(1)
                                        }
                                        if cp3 == cp1.offset(8) as *const i8 {
                                            loaded_font_flags =
                                                (loaded_font_flags as i32 | 0x2i32) as i8;
                                            current_block = 13857423536159756434;
                                        } else {
                                            current_block = 10622493848381539643;
                                        }
                                    } else {
                                        current_block = 10622493848381539643;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                10622493848381539643 => {
                    font_feature_warning(
                        cp1 as *mut libc::c_void,
                        cp2.wrapping_offset_from(cp1) as i64 as i32,
                        0 as *const libc::c_void,
                        0i32,
                    );
                }
                _ => {}
            }
            cp1 = cp2
        }
    }
    /* break if end of string */
    if !shapers.is_null() {
        shapers = xrealloc(
            shapers as *mut libc::c_void,
            ((nShapers + 1i32) as u64).wrapping_mul(::std::mem::size_of::<*mut i8>() as u64),
        ) as *mut *mut i8;
        let ref mut fresh11 = *shapers.offset(nShapers as isize);
        *fresh11 = 0 as *mut i8
    }
    if embolden as f64 != 0.0f64 {
        embolden = (embolden as f64 * Fix2D(scaled_size) / 100.0f64) as f32
    }
    if letterspace as f64 != 0.0f64 {
        loaded_font_letter_space = (letterspace as f64 / 100.0f64 * scaled_size as f64) as scaled_t
    }
    if loaded_font_flags as i32 & 0x1i32 == 0i32 {
        rgbValue = 0xff_u32
    }
    if loaded_font_flags as i32 & 0x2i32 != 0i32 {
        setFontLayoutDir(font, 1i32);
    }
    engine = createLayoutEngine(
        fontRef, font, script, language, features, nFeatures, shapers, rgbValue, extend, slant,
        embolden,
    );
    if engine.is_null() {
        // only free these if creation failed, otherwise the engine now owns them
        free(features as *mut libc::c_void);
        free(shapers as *mut libc::c_void);
    } else {
        native_font_type_flag = 0xfffeu32 as i32
    }
    engine as *mut libc::c_void
}

unsafe extern "C" fn splitFontName(
    mut name: *mut i8,
    mut var: *mut *mut i8,
    mut feat: *mut *mut i8,
    mut end: *mut *mut i8,
    mut index: *mut i32,
) {
    *var = 0 as *mut i8;
    *feat = 0 as *mut i8;
    *index = 0i32;
    if *name as i32 == '[' as i32 {
        let mut withinFileName: i32 = 1i32;
        name = name.offset(1);
        while *name != 0 {
            if withinFileName != 0 && *name as i32 == ']' as i32 {
                withinFileName = 0i32;
                if (*var).is_null() {
                    *var = name
                }
            } else if *name as i32 == ':' as i32 {
                if withinFileName != 0 && (*var).is_null() {
                    *var = name;
                    name = name.offset(1);
                    while *name as i32 >= '0' as i32 && *name as i32 <= '9' as i32 {
                        let fresh12 = name;
                        name = name.offset(1);
                        *index = *index * 10i32 + *fresh12 as i32 - '0' as i32
                    }
                    name = name.offset(-1)
                } else if withinFileName == 0 && (*feat).is_null() {
                    *feat = name
                }
            }
            name = name.offset(1)
        }
        *end = name
    } else {
        while *name != 0 {
            if *name as i32 == '/' as i32 && (*var).is_null() && (*feat).is_null() {
                *var = name
            } else if *name as i32 == ':' as i32 && (*feat).is_null() {
                *feat = name
            }
            name = name.offset(1)
        }
        *end = name
    }
    if (*feat).is_null() {
        *feat = name
    }
    if (*var).is_null() {
        *var = *feat
    };
}
#[no_mangle]
pub unsafe extern "C" fn find_native_font(
    mut uname: *mut i8,
    mut scaled_size: i32,
) -> *mut libc::c_void
/* scaled_size here is in TeX points, or is a negative integer for 'scaled_t' */ {
    let mut rval: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nameString: *mut i8 = 0 as *mut i8;
    let mut var: *mut i8 = 0 as *mut i8;
    let mut feat: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut name: *mut i8 = uname;
    let mut varString: *mut i8 = 0 as *mut i8;
    let mut featString: *mut i8 = 0 as *mut i8;
    let mut fontRef: PlatformFontRef = 0 as PlatformFontRef;
    let mut font: XeTeXFont = 0 as XeTeXFont;
    let mut index: i32 = 0i32;
    loaded_font_mapping = 0 as *mut libc::c_void;
    loaded_font_flags = 0_i8;
    loaded_font_letter_space = 0i32;
    splitFontName(name, &mut var, &mut feat, &mut end, &mut index);
    nameString =
        xmalloc((var.wrapping_offset_from(name) as i64 + 1i32 as i64) as size_t) as *mut i8;
    strncpy(
        nameString,
        name,
        var.wrapping_offset_from(name) as i64 as u64,
    );
    *nameString.offset(var.wrapping_offset_from(name) as i64 as isize) = 0_i8;
    if feat > var {
        varString = xmalloc(feat.wrapping_offset_from(var) as i64 as size_t) as *mut i8;
        strncpy(
            varString,
            var.offset(1),
            (feat.wrapping_offset_from(var) as i64 - 1i32 as i64) as u64,
        );
        *varString.offset((feat.wrapping_offset_from(var) as i64 - 1i32 as i64) as isize) = 0_i8
    }
    if end > feat {
        featString = xmalloc(end.wrapping_offset_from(feat) as i64 as size_t) as *mut i8;
        strncpy(
            featString,
            feat.offset(1),
            (end.wrapping_offset_from(feat) as i64 - 1i32 as i64) as u64,
        );
        *featString.offset((end.wrapping_offset_from(feat) as i64 - 1i32 as i64) as isize) = 0_i8
    }
    // check for "[filename]" form, don't search maps in this case
    if *nameString.offset(0) as i32 == '[' as i32 {
        if scaled_size < 0i32 {
            font = createFontFromFile(nameString.offset(1), index, 655360i64 as Fixed);
            if !font.is_null() {
                let mut dsize: Fixed = D2Fix(getDesignSize(font));
                if scaled_size == -1000i32 {
                    scaled_size = dsize
                } else {
                    scaled_size = xn_over_d(dsize, -scaled_size, 1000i32)
                }
                deleteFont(font);
            }
        }
        font = createFontFromFile(nameString.offset(1), index, scaled_size);
        if !font.is_null() {
            loaded_font_design_size = D2Fix(getDesignSize(font));
            /* This is duplicated in XeTeXFontMgr::findFont! */
            setReqEngine(0_i8);
            if !varString.is_null() {
                if !strstartswith(varString, b"/AAT\x00" as *const u8 as *const i8).is_null() {
                    setReqEngine('A' as i32 as i8);
                } else if !strstartswith(varString, b"/OT\x00" as *const u8 as *const i8).is_null()
                    || !strstartswith(varString, b"/ICU\x00" as *const u8 as *const i8).is_null()
                {
                    setReqEngine('O' as i32 as i8);
                } else if !strstartswith(varString, b"/GR\x00" as *const u8 as *const i8).is_null()
                {
                    setReqEngine('G' as i32 as i8);
                }
            }
            rval = loadOTfont(0 as PlatformFontRef, font, scaled_size, featString);
            if rval.is_null() {
                deleteFont(font);
            }
            if !rval.is_null() && get_tracing_fonts_state() > 0i32 {
                begin_diagnostic();
                print_nl(' ' as i32);
                print_c_string(b"-> \x00" as *const u8 as *const i8);
                print_c_string(nameString.offset(1));
                end_diagnostic(0i32 != 0);
            }
        }
    } else {
        fontRef = findFontByName(nameString, varString, Fix2D(scaled_size));
        if !fontRef.is_null() {
            /* update name_of_file to the full name of the font, for error messages during font loading */
            let mut fullName: *const i8 = getFullName(fontRef);
            name_length = strlen(fullName) as i32;
            if !featString.is_null() {
                name_length = (name_length as u64)
                    .wrapping_add(strlen(featString).wrapping_add(1i32 as u64))
                    as i32 as i32
            }
            if !varString.is_null() {
                name_length = (name_length as u64)
                    .wrapping_add(strlen(varString).wrapping_add(1i32 as u64))
                    as i32 as i32
            }
            free(name_of_file as *mut libc::c_void);
            name_of_file = xmalloc((name_length + 1i32) as size_t) as *mut i8;
            strcpy(name_of_file, fullName);
            if scaled_size < 0i32 {
                font = createFont(fontRef, scaled_size);
                if !font.is_null() {
                    let mut dsize_0: Fixed = D2Fix(getDesignSize(font));
                    if scaled_size == -1000i32 {
                        scaled_size = dsize_0
                    } else {
                        scaled_size = xn_over_d(dsize_0, -scaled_size, 1000i32)
                    }
                    deleteFont(font);
                }
            }
            font = createFont(fontRef, scaled_size);
            if !font.is_null() {
                #[cfg(not(target_os = "macos"))]
                {
                    rval = loadOTfont(fontRef, font, scaled_size, featString);
                    if rval.is_null() {
                        deleteFont(font);
                    }
                }
                #[cfg(target_os = "macos")]
                {
                    /* decide whether to use AAT or OpenType rendering with this font */
                    if getReqEngine() as libc::c_int == 'A' as i32 {
                        rval = aat::loadAATfont(fontRef, scaled_size, featString);
                        if rval.is_null() {
                            deleteFont(font);
                        }
                    } else {
                        if getReqEngine() as libc::c_int == 'O' as i32
                            || getReqEngine() as libc::c_int == 'G' as i32
                            || !getFontTablePtr(
                                font,
                                ('G' as i32 as u32 & 0xffi32 as libc::c_uint) << 24i32
                                    | ('S' as i32 as u32 & 0xffi32 as libc::c_uint) << 16i32
                                    | ('U' as i32 as u32 & 0xffi32 as libc::c_uint) << 8i32
                                    | 'B' as i32 as u32 & 0xffi32 as libc::c_uint,
                            )
                            .is_null()
                            || !getFontTablePtr(
                                font,
                                ('G' as i32 as u32 & 0xffi32 as libc::c_uint) << 24i32
                                    | ('P' as i32 as u32 & 0xffi32 as libc::c_uint) << 16i32
                                    | ('O' as i32 as u32 & 0xffi32 as libc::c_uint) << 8i32
                                    | 'S' as i32 as u32 & 0xffi32 as libc::c_uint,
                            )
                            .is_null()
                        {
                            rval = loadOTfont(fontRef, font, scaled_size, featString)
                        }
                        /* loadOTfont failed or the above check was false */
                        if rval.is_null() {
                            rval = aat::loadAATfont(fontRef, scaled_size, featString)
                        }
                        if rval.is_null() {
                            deleteFont(font);
                        }
                    }
                }
            }
            /* append the style and feature strings, so that \show\fontID will give a full result */
            if !varString.is_null() && *varString as i32 != 0i32 {
                strcat(name_of_file, b"/\x00" as *const u8 as *const i8);
                strcat(name_of_file, varString);
            }
            if !featString.is_null() && *featString as i32 != 0i32 {
                strcat(name_of_file, b":\x00" as *const u8 as *const i8);
                strcat(name_of_file, featString);
            }
            name_length = strlen(name_of_file) as i32
        }
    }
    free(varString as *mut libc::c_void);
    free(featString as *mut libc::c_void);
    free(nameString as *mut libc::c_void);
    rval
}
#[no_mangle]
pub unsafe extern "C" fn release_font_engine(mut engine: *mut libc::c_void, mut type_flag: i32) {
    match type_flag as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            CFRelease(engine as CFDictionaryRef as CFTypeRef);
        }
        0xfffeu32 => {
            deleteLayoutEngine(engine as XeTeXLayoutEngine);
        }
        _ => {}
    }
}
#[no_mangle]
pub unsafe extern "C" fn ot_get_font_metrics(
    mut pEngine: *mut libc::c_void,
    mut ascent: *mut scaled_t,
    mut descent: *mut scaled_t,
    mut xheight: *mut scaled_t,
    mut capheight: *mut scaled_t,
    mut slant: *mut scaled_t,
) {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut a: f32 = 0.;
    let mut d: f32 = 0.;
    getAscentAndDescent(engine, &mut a, &mut d);
    *ascent = D2Fix(a as f64);
    *descent = D2Fix(d as f64);
    *slant = D2Fix(
        Fix2D(getSlant(getFont(engine))) * getExtendFactor(engine) as f64
            + getSlantFactor(engine) as f64,
    );
    /* get cap and x height from OS/2 table */
    getCapAndXHeight(engine, &mut a, &mut d);
    *capheight = D2Fix(a as f64);
    *xheight = D2Fix(d as f64);
    /* fallback in case the font does not have OS/2 table */
    if *xheight == 0i32 {
        let mut glyphID: i32 = mapCharToGlyph(engine, 'x' as i32 as u32) as i32;
        if glyphID != 0i32 {
            getGlyphHeightDepth(engine, glyphID as u32, &mut a, &mut d);
            *xheight = D2Fix(a as f64)
        } else {
            *xheight = *ascent / 2i32
            /* arbitrary figure if there's no 'x' in the font */
        }
    }
    if *capheight == 0i32 {
        let mut glyphID_0: i32 = mapCharToGlyph(engine, 'X' as i32 as u32) as i32;
        if glyphID_0 != 0i32 {
            getGlyphHeightDepth(engine, glyphID_0 as u32, &mut a, &mut d);
            *capheight = D2Fix(a as f64)
        } else {
            *capheight = *ascent
            /* arbitrary figure if there's no 'X' in the font */
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ot_font_get(mut what: i32, mut pEngine: *mut libc::c_void) -> i32 {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut fontInst: XeTeXFont = getFont(engine);
    match what {
        1 => return countGlyphs(fontInst) as i32,
        8 => {
            /* ie Graphite features */
            return countGraphiteFeatures(engine) as i32;
        }
        16 => return countScripts(fontInst) as i32,
        _ => {}
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn ot_font_get_1(
    mut what: i32,
    mut pEngine: *mut libc::c_void,
    mut param: i32,
) -> i32 {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut fontInst: XeTeXFont = getFont(engine);
    match what {
        17 => return countLanguages(fontInst, param as hb_tag_t) as i32,
        19 => return getIndScript(fontInst, param as u32) as i32,
        9 => {
            /* for graphite fonts...*/
            return getGraphiteFeatureCode(engine, param as u32) as i32;
        }
        11 => return 1i32,
        12 => return countGraphiteFeatureSettings(engine, param as u32) as i32,
        _ => {}
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn ot_font_get_2(
    mut what: i32,
    mut pEngine: *mut libc::c_void,
    mut param1: i32,
    mut param2: i32,
) -> i32 {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut fontInst: XeTeXFont = getFont(engine);
    match what {
        20 => return getIndLanguage(fontInst, param1 as hb_tag_t, param2 as u32) as i32,
        18 => return countFeatures(fontInst, param1 as hb_tag_t, param2 as hb_tag_t) as i32,
        13 => {
            /* for graphite fonts */
            return getGraphiteFeatureSettingCode(engine, param1 as u32, param2 as u32) as i32;
        }
        15 => {
            return (getGraphiteFeatureDefaultSetting(engine, param1 as u32) == param2 as u32)
                as i32
        }
        _ => {}
    } /* to guarantee enough space in the buffer */
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn ot_font_get_3(
    mut what: i32,
    mut pEngine: *mut libc::c_void,
    mut param1: i32,
    mut param2: i32,
    mut param3: i32,
) -> i32 {
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    let mut fontInst: XeTeXFont = getFont(engine);
    match what {
        21 => {
            return getIndFeature(
                fontInst,
                param1 as hb_tag_t,
                param2 as hb_tag_t,
                param3 as u32,
            ) as i32
        }
        _ => {}
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn gr_print_font_name(
    mut what: i32,
    mut pEngine: *mut libc::c_void,
    mut param1: i32,
    mut param2: i32,
) {
    let mut name: *mut i8 = 0 as *mut i8;
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    match what {
        8 => name = getGraphiteFeatureLabel(engine, param1 as u32),
        9 => name = getGraphiteFeatureSettingLabel(engine, param1 as u32, param2 as u32),
        _ => {}
    }
    if !name.is_null() {
        print_c_string(name);
        gr_label_destroy(name as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gr_font_get_named(mut what: i32, mut pEngine: *mut libc::c_void) -> i32 {
    let mut rval: i64 = -1i32 as i64;
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    match what {
        10 => rval = findGraphiteFeatureNamed(engine, name_of_file, name_length),
        _ => {}
    }
    rval as i32
}
#[no_mangle]
pub unsafe extern "C" fn gr_font_get_named_1(
    mut what: i32,
    mut pEngine: *mut libc::c_void,
    mut param: i32,
) -> i32 {
    let mut rval: i64 = -1i32 as i64;
    let mut engine: XeTeXLayoutEngine = pEngine as XeTeXLayoutEngine;
    match what {
        14 => {
            rval = findGraphiteFeatureSettingNamed(engine, param as u32, name_of_file, name_length)
        }
        _ => {}
    }
    rval as i32
}
#[cfg(target_os = "macos")]
unsafe extern "C" fn cgColorToRGBA32(mut color: CGColorRef) -> UInt32 {
    let mut components: *const CGFloat = CGColorGetComponents(color);
    let mut rval: UInt32 = (*components.offset(0) * 255.0f64 + 0.5f64) as UInt8 as UInt32;
    rval <<= 8i32;
    rval = (rval as libc::c_uint)
        .wrapping_add((*components.offset(1) * 255.0f64 + 0.5f64) as UInt8 as libc::c_uint)
        as UInt32 as UInt32;
    rval <<= 8i32;
    rval = (rval as libc::c_uint)
        .wrapping_add((*components.offset(2) * 255.0f64 + 0.5f64) as UInt8 as libc::c_uint)
        as UInt32 as UInt32;
    rval <<= 8i32;
    rval = (rval as libc::c_uint)
        .wrapping_add((*components.offset(3) * 255.0f64 + 0.5f64) as UInt8 as libc::c_uint)
        as UInt32 as UInt32;
    return rval;
}
static mut xdvBufSize: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn makeXDVGlyphArrayData(mut pNode: *mut libc::c_void) -> i32 {
    let mut cp: *mut u8 = 0 as *mut u8;
    let mut glyphIDs: *mut u16 = 0 as *mut u16;
    let mut p: *mut memory_word = pNode as *mut memory_word;
    let mut glyph_info: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut locations: *mut FixedPoint = 0 as *mut FixedPoint;
    let mut width: Fixed = 0;
    let mut glyphCount: u16 = (*p.offset(4)).b16.s0;
    let mut i: i32 = glyphCount as i32 * 10i32 + 8i32;
    if i > xdvBufSize {
        free(xdv_buffer as *mut libc::c_void);
        xdvBufSize = (i / 1024i32 + 1i32) * 1024i32;
        xdv_buffer = xmalloc(xdvBufSize as size_t) as *mut i8
    }
    glyph_info = (*p.offset(5)).ptr;
    locations = glyph_info as *mut FixedPoint;
    glyphIDs = locations.offset(glyphCount as i32 as isize) as *mut u16;
    cp = xdv_buffer as *mut u8;
    width = (*p.offset(1)).b32.s1;
    let fresh13 = cp;
    cp = cp.offset(1);
    *fresh13 = (width >> 24i32 & 0xffi32) as u8;
    let fresh14 = cp;
    cp = cp.offset(1);
    *fresh14 = (width >> 16i32 & 0xffi32) as u8;
    let fresh15 = cp;
    cp = cp.offset(1);
    *fresh15 = (width >> 8i32 & 0xffi32) as u8;
    let fresh16 = cp;
    cp = cp.offset(1);
    *fresh16 = (width & 0xffi32) as u8;
    let fresh17 = cp;
    cp = cp.offset(1);
    *fresh17 = (glyphCount as i32 >> 8i32 & 0xffi32) as u8;
    let fresh18 = cp;
    cp = cp.offset(1);
    *fresh18 = (glyphCount as i32 & 0xffi32) as u8;
    i = 0i32;
    while i < glyphCount as i32 {
        let mut x: Fixed = (*locations.offset(i as isize)).x;
        let mut y: Fixed = (*locations.offset(i as isize)).y;
        let fresh19 = cp;
        cp = cp.offset(1);
        *fresh19 = (x >> 24i32 & 0xffi32) as u8;
        let fresh20 = cp;
        cp = cp.offset(1);
        *fresh20 = (x >> 16i32 & 0xffi32) as u8;
        let fresh21 = cp;
        cp = cp.offset(1);
        *fresh21 = (x >> 8i32 & 0xffi32) as u8;
        let fresh22 = cp;
        cp = cp.offset(1);
        *fresh22 = (x & 0xffi32) as u8;
        let fresh23 = cp;
        cp = cp.offset(1);
        *fresh23 = (y >> 24i32 & 0xffi32) as u8;
        let fresh24 = cp;
        cp = cp.offset(1);
        *fresh24 = (y >> 16i32 & 0xffi32) as u8;
        let fresh25 = cp;
        cp = cp.offset(1);
        *fresh25 = (y >> 8i32 & 0xffi32) as u8;
        let fresh26 = cp;
        cp = cp.offset(1);
        *fresh26 = (y & 0xffi32) as u8;
        i += 1
    }
    i = 0i32;
    while i < glyphCount as i32 {
        let mut g: u16 = *glyphIDs.offset(i as isize);
        let fresh27 = cp;
        cp = cp.offset(1);
        *fresh27 = (g as i32 >> 8i32 & 0xffi32) as u8;
        let fresh28 = cp;
        cp = cp.offset(1);
        *fresh28 = (g as i32 & 0xffi32) as u8;
        i += 1
    }
    (cp as *mut i8).wrapping_offset_from(xdv_buffer) as i64 as i32
}
#[no_mangle]
pub unsafe extern "C" fn make_font_def(mut f: i32) -> i32 {
    // XXX: seems like a good idea to make a struct FontDef
    let mut flags: u16 = 0_u16;
    let mut rgba: u32 = 0;
    let mut size: Fixed = 0;
    let mut filename: *mut i8 = 0 as *mut i8;
    let mut index: u32 = 0;
    let mut filenameLen: u8 = 0;
    let mut fontDefLength: i32 = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    /* PlatformFontRef fontRef = 0; */
    let mut extend: f32 = 1.0f64 as f32;
    let mut slant: f32 = 0.0f64 as f32;
    let mut embolden: f32 = 0.0f64 as f32;
    match *font_area.offset(f as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            let mut attributes: CFDictionaryRef = 0 as CFDictionaryRef;
            let mut font: CTFontRef = 0 as CTFontRef;
            let mut color: CGColorRef = 0 as CGColorRef;
            let mut t: CGAffineTransform = CGAffineTransform {
                a: 0.,
                b: 0.,
                c: 0.,
                d: 0.,
                tx: 0.,
                ty: 0.,
            };
            let mut emboldenNumber: CFNumberRef = 0 as CFNumberRef;
            let mut fSize: CGFloat = 0.;
            attributes = *font_layout_engine.offset(f as isize) as CFDictionaryRef;
            font = CFDictionaryGetValue(attributes, kCTFontAttributeName as *const libc::c_void)
                as CTFontRef;
            filename = aat::getFileNameFromCTFont(font, &mut index);
            assert!(!filename.is_null());
            if !CFDictionaryGetValue(
                attributes,
                kCTVerticalFormsAttributeName as *const libc::c_void,
            )
            .is_null()
            {
                flags = (flags as libc::c_int | 0x100i32) as u16
            }
            color = CFDictionaryGetValue(
                attributes,
                kCTForegroundColorAttributeName as *const libc::c_void,
            ) as CGColorRef;
            if !color.is_null() {
                rgba = cgColorToRGBA32(color)
            }
            t = CTFontGetMatrix(font);
            extend = t.a as libc::c_float;
            slant = t.c as libc::c_float;
            emboldenNumber = CFDictionaryGetValue(
                attributes,
                aat::getkXeTeXEmboldenAttributeName() as *const libc::c_void,
            ) as CFNumberRef;
            if !emboldenNumber.is_null() {
                CFNumberGetValue(
                    emboldenNumber,
                    kCFNumberFloatType as libc::c_int as CFNumberType,
                    &mut embolden as *mut libc::c_float as *mut libc::c_void,
                );
            }
            fSize = CTFontGetSize(font);
            size = D2Fix(fSize);
        }
        0xfffeu32 => {
            let mut engine: XeTeXLayoutEngine = 0 as *mut XeTeXLayoutEngine_rec;
            engine = *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
            /* fontRef = */
            getFontRef(engine);
            filename = getFontFilename(engine, &mut index);
            assert!(!filename.is_null());
            rgba = getRgbValue(engine);
            if *font_flags.offset(f as isize) as i32 & 0x2i32 != 0i32 {
                flags = (flags as i32 | 0x100i32) as u16
            }
            extend = getExtendFactor(engine);
            slant = getSlantFactor(engine);
            embolden = getEmboldenFactor(engine);
            size = D2Fix(getPointSize(engine) as f64)
        }
        _ => {
            panic!("bad native font flag in `make_font_def`");
        }
    }
    filenameLen = strlen(filename) as u8;
    /* parameters after internal font ID:
    //  size[4]
    //  flags[2]
    //  l[1] n[l]
    //  if flags & COLORED:
    //      c[4]
     */
    fontDefLength = 4i32 + 2i32 + 1i32 + filenameLen as i32 + 4i32; /* face index */
    if *font_flags.offset(f as isize) as i32 & 0x1i32 != 0i32 {
        fontDefLength += 4i32; /* 32-bit RGBA value */
        flags = (flags as i32 | 0x200i32) as u16
    }
    if extend as f64 != 1.0f64 {
        fontDefLength += 4i32;
        flags = (flags as i32 | 0x1000i32) as u16
    }
    if slant as f64 != 0.0f64 {
        fontDefLength += 4i32;
        flags = (flags as i32 | 0x2000i32) as u16
    }
    if embolden as f64 != 0.0f64 {
        fontDefLength += 4i32;
        flags = (flags as i32 | 0x4000i32) as u16
    }
    if fontDefLength > xdvBufSize {
        free(xdv_buffer as *mut libc::c_void);
        xdvBufSize = (fontDefLength / 1024i32 + 1i32) * 1024i32;
        xdv_buffer = xmalloc(xdvBufSize as size_t) as *mut i8
    }
    cp = xdv_buffer;
    *(cp as *mut Fixed) = SWAP32(size as u32) as Fixed;
    cp = cp.offset(4);
    *(cp as *mut u16) = SWAP16(flags);
    cp = cp.offset(2);
    *(cp as *mut u8) = filenameLen;
    cp = cp.offset(1);
    memcpy(
        cp as *mut libc::c_void,
        filename as *const libc::c_void,
        filenameLen as u64,
    );
    cp = cp.offset(filenameLen as i32 as isize);
    *(cp as *mut u32) = SWAP32(index);
    cp = cp.offset(4);
    if *font_flags.offset(f as isize) as i32 & 0x1i32 != 0i32 {
        *(cp as *mut u32) = SWAP32(rgba);
        cp = cp.offset(4)
    }
    if flags as i32 & 0x1000i32 != 0 {
        let mut f_0: Fixed = D2Fix(extend as f64);
        *(cp as *mut u32) = SWAP32(f_0 as u32);
        cp = cp.offset(4)
    }
    if flags as i32 & 0x2000i32 != 0 {
        let mut f_1: Fixed = D2Fix(slant as f64);
        *(cp as *mut u32) = SWAP32(f_1 as u32);
        cp = cp.offset(4)
    }
    if flags as i32 & 0x4000i32 != 0 {
        let mut f_2: Fixed = D2Fix(embolden as f64);
        *(cp as *mut u32) = SWAP32(f_2 as u32);
        cp = cp.offset(4)
    }
    free(filename as *mut libc::c_void);
    fontDefLength
}
#[no_mangle]
pub unsafe extern "C" fn apply_mapping(
    mut pCnv: *mut libc::c_void,
    mut txtPtr: *mut u16,
    mut txtLen: i32,
) -> i32 {
    let mut cnv: TECkit_Converter = pCnv as TECkit_Converter;
    let mut inUsed: UInt32 = 0;
    let mut outUsed: UInt32 = 0;
    let mut status: TECkit_Status = 0;
    static mut outLength: UInt32 = 0i32 as UInt32;
    /* allocate outBuffer if not big enough */
    if (outLength as u64)
        < (txtLen as u64)
            .wrapping_mul(::std::mem::size_of::<UniChar>() as u64)
            .wrapping_add(32i32 as u64)
    {
        free(mapped_text as *mut libc::c_void);
        outLength = (txtLen as u64)
            .wrapping_mul(::std::mem::size_of::<UniChar>() as u64)
            .wrapping_add(32i32 as u64) as UInt32;
        mapped_text = xmalloc(outLength as size_t) as *mut UTF16_code
    }
    loop
    /* try the mapping */
    {
        status = TECkit_ConvertBuffer(
            cnv,
            txtPtr as *mut Byte,
            (txtLen as u64).wrapping_mul(::std::mem::size_of::<UniChar>() as u64) as UInt32,
            &mut inUsed,
            mapped_text as *mut Byte,
            outLength,
            &mut outUsed,
            1i32 as Byte,
        );
        match status {
            0 => {
                txtPtr = mapped_text as *mut UniChar;
                return (outUsed as u64).wrapping_div(::std::mem::size_of::<UniChar>() as u64)
                    as i32;
            }
            1 => {
                outLength = (outLength as u64).wrapping_add(
                    (txtLen as u64)
                        .wrapping_mul(::std::mem::size_of::<UniChar>() as u64)
                        .wrapping_add(32i32 as u64),
                ) as UInt32 as UInt32;
                free(mapped_text as *mut libc::c_void);
                mapped_text = xmalloc(outLength as size_t) as *mut UTF16_code
            }
            _ => return 0i32,
        }
    }
}
unsafe extern "C" fn snap_zone(
    mut value: *mut scaled_t,
    mut snap_value: scaled_t,
    mut fuzz: scaled_t,
) {
    let mut difference: scaled_t = *value - snap_value;
    if difference <= fuzz && difference >= -fuzz {
        *value = snap_value
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_native_char_height_depth(
    mut font: i32,
    mut ch: i32,
    mut height: *mut scaled_t,
    mut depth: *mut scaled_t,
) {
    let mut ht: f32 = 0.0f64 as f32;
    let mut dp: f32 = 0.0f64 as f32;
    let mut fuzz: Fixed = 0;
    match *font_area.offset(font as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            let mut attributes: CFDictionaryRef =
                *font_layout_engine.offset(font as isize) as CFDictionaryRef;
            let mut gid: libc::c_int = aat::MapCharToGlyph_AAT(attributes, ch as UInt32);
            aat::GetGlyphHeightDepth_AAT(attributes, gid as u16, &mut ht, &mut dp);
        }
        0xfffeu32 => {
            let mut engine: XeTeXLayoutEngine =
                *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine;
            let mut gid: i32 = mapCharToGlyph(engine, ch as u32) as i32;
            getGlyphHeightDepth(engine, gid as u32, &mut ht, &mut dp);
        }
        _ => {
            panic!("bad native font flag in `get_native_char_height_depth`");
        }
    }
    *height = D2Fix(ht as f64);
    *depth = D2Fix(dp as f64);
    /* snap to "known" zones for baseline, x-height, cap-height if within 4% of em-size */
    fuzz = (*font_info.offset((6i32 + *param_base.offset(font as isize)) as isize))
        .b32
        .s1
        / 25i32;
    snap_zone(depth, 0i32, fuzz);
    snap_zone(height, 0i32, fuzz);
    snap_zone(
        height,
        (*font_info.offset((5i32 + *param_base.offset(font as isize)) as isize))
            .b32
            .s1,
        fuzz,
    );
    snap_zone(
        height,
        (*font_info.offset((8i32 + *param_base.offset(font as isize)) as isize))
            .b32
            .s1,
        fuzz,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getnativecharht(mut f: i32, mut c: i32) -> scaled_t {
    let mut h: scaled_t = 0;
    let mut d: scaled_t = 0;
    get_native_char_height_depth(f, c, &mut h, &mut d);
    h
}
#[no_mangle]
pub unsafe extern "C" fn getnativechardp(mut f: i32, mut c: i32) -> scaled_t {
    let mut h: scaled_t = 0;
    let mut d: scaled_t = 0;
    get_native_char_height_depth(f, c, &mut h, &mut d);
    d
}
#[no_mangle]
pub unsafe extern "C" fn get_native_char_sidebearings(
    mut font: i32,
    mut ch: i32,
    mut lsb: *mut scaled_t,
    mut rsb: *mut scaled_t,
) {
    let mut l: f32 = 0.;
    let mut r: f32 = 0.;
    match *font_area.offset(font as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            let mut attributes: CFDictionaryRef =
                *font_layout_engine.offset(font as isize) as CFDictionaryRef;
            let mut gid: libc::c_int = aat::MapCharToGlyph_AAT(attributes, ch as UInt32);
            aat::GetGlyphSidebearings_AAT(attributes, gid as u16, &mut l, &mut r);
        }
        0xfffeu32 => {
            let mut engine: XeTeXLayoutEngine =
                *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine;
            let mut gid: i32 = mapCharToGlyph(engine, ch as u32) as i32;
            getGlyphSidebearings(engine, gid as u32, &mut l, &mut r);
        }
        _ => {
            panic!("bad native font flag in `get_native_char_side_bearings`");
        }
    }
    *lsb = D2Fix(l as f64);
    *rsb = D2Fix(r as f64);
}
#[no_mangle]
pub unsafe extern "C" fn get_glyph_bounds(mut font: i32, mut edge: i32, mut gid: i32) -> scaled_t {
    /* edge codes 1,2,3,4 => L T R B */
    let mut a: f32 = 0.;
    let mut b: f32 = 0.;
    match *font_area.offset(font as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            let mut attributes: CFDictionaryRef =
                *font_layout_engine.offset(font as isize) as CFDictionaryRef;
            if edge & 1i32 != 0 {
                aat::GetGlyphSidebearings_AAT(attributes, gid as u16, &mut a, &mut b);
            } else {
                aat::GetGlyphHeightDepth_AAT(attributes, gid as u16, &mut a, &mut b);
            }
        }
        0xfffeu32 => {
            let mut engine: XeTeXLayoutEngine =
                *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine;
            if edge & 1i32 != 0 {
                getGlyphSidebearings(engine, gid as u32, &mut a, &mut b);
            } else {
                getGlyphHeightDepth(engine, gid as u32, &mut a, &mut b);
            }
        }
        _ => {
            _tt_abort(b"bad native font flag in `get_glyph_bounds`\x00" as *const u8 as *const i8);
        }
    }
    D2Fix((if edge <= 2i32 { a } else { b }) as f64)
}
#[no_mangle]
pub unsafe extern "C" fn getnativecharic(mut f: i32, mut c: i32) -> scaled_t {
    let mut lsb: scaled_t = 0;
    let mut rsb: scaled_t = 0;
    get_native_char_sidebearings(f, c, &mut lsb, &mut rsb);
    if rsb < 0i32 {
        *font_letter_space.offset(f as isize) - rsb
    } else {
        *font_letter_space.offset(f as isize)
    }
}
/* single-purpose metrics accessors */
#[no_mangle]
pub unsafe extern "C" fn getnativecharwd(mut f: i32, mut c: i32) -> scaled_t {
    let mut wd: scaled_t = 0i32;
    match *font_area.offset(f as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            let mut attributes: CFDictionaryRef =
                *font_layout_engine.offset(f as isize) as CFDictionaryRef;
            let mut gid: libc::c_int = aat::MapCharToGlyph_AAT(attributes, c as UInt32);
            wd = D2Fix(aat::GetGlyphWidth_AAT(attributes, gid as u16))
        }
        0xfffeu32 => {
            let mut engine: XeTeXLayoutEngine =
                *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
            let mut gid: i32 = mapCharToGlyph(engine, c as u32) as i32;
            wd = D2Fix(getGlyphWidthFromEngine(engine, gid as u32) as f64)
        }
        _ => {
            panic!("bad native font flag in `get_native_char_wd`");
        }
    }
    wd
}
#[no_mangle]
pub unsafe extern "C" fn real_get_native_glyph(
    mut pNode: *mut libc::c_void,
    mut index: u32,
) -> u16 {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut locations: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint;
    let mut glyphIDs: *mut u16 =
        locations.offset((*node.offset(4)).b16.s0 as i32 as isize) as *mut u16;
    if index >= (*node.offset(4)).b16.s0 as u32 {
        0_u16
    } else {
        *glyphIDs.offset(index as isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn store_justified_native_glyphs(mut pNode: *mut libc::c_void) {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut f: u32 = (*node.offset(4)).b16.s2 as u32;
    match *font_area.offset(f as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            /* separate Mac-only codepath for AAT fonts */
            aat::do_aat_layout(node as *mut libc::c_void, 1i32);
            return;
        }
        _ => {
            /* FIXME: 0xfffeu32 case, but the original code wrote it this way */
            /* save desired width */
            let mut savedWidth: i32 = (*node.offset(1)).b32.s1;
            measure_native_node(node as *mut libc::c_void, 0i32);
            if (*node.offset(1)).b32.s1 != savedWidth {
                /* see how much adjustment is needed overall */
                let mut justAmount: f64 = Fix2D(savedWidth - (*node.offset(1)).b32.s1);
                /* apply justification to spaces (or if there are none, distribute it to all glyphs as a last resort) */
                let mut locations: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint;
                let mut glyphIDs: *mut u16 =
                    locations.offset((*node.offset(4)).b16.s0 as i32 as isize) as *mut u16;
                let mut glyphCount: i32 = (*node.offset(4)).b16.s0 as i32;
                let mut spaceCount: i32 = 0i32;
                let mut i: i32 = 0;
                let mut spaceGlyph: i32 = map_char_to_glyph(f as i32, ' ' as i32);
                i = 0i32;
                while i < glyphCount {
                    if *glyphIDs.offset(i as isize) as i32 == spaceGlyph {
                        spaceCount += 1
                    }
                    i += 1
                }
                if spaceCount > 0i32 {
                    let mut adjustment: f64 = 0i32 as f64;
                    let mut spaceIndex: i32 = 0i32;
                    i = 0i32;
                    while i < glyphCount {
                        (*locations.offset(i as isize)).x =
                            D2Fix(Fix2D((*locations.offset(i as isize)).x) + adjustment);
                        if *glyphIDs.offset(i as isize) as i32 == spaceGlyph {
                            spaceIndex += 1;
                            adjustment = justAmount * spaceIndex as f64 / spaceCount as f64
                        }
                        i += 1
                    }
                } else {
                    i = 1i32;
                    while i < glyphCount {
                        (*locations.offset(i as isize)).x = D2Fix(
                            Fix2D((*locations.offset(i as isize)).x)
                                + justAmount * i as f64 / (glyphCount - 1i32) as f64,
                        );
                        i += 1
                    }
                }
                (*node.offset(1)).b32.s1 = savedWidth
            };
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn measure_native_node(
    mut pNode: *mut libc::c_void,
    mut use_glyph_metrics: i32,
) {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut txtLen: i32 = (*node.offset(4)).b16.s1 as i32;
    let mut txtPtr: *mut u16 = node.offset(6) as *mut u16;
    let mut f: u32 = (*node.offset(4)).b16.s2 as u32;
    if *font_area.offset(f as isize) as u32 == 0xfffeu32 {
        /* using this font in OT Layout mode, so font_layout_engine[f] is actually a XeTeXLayoutEngine */
        let mut engine: XeTeXLayoutEngine =
            *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
        let mut locations: *mut FixedPoint = 0 as *mut FixedPoint;
        let mut glyphIDs: *mut u16 = 0 as *mut u16;
        let mut glyphAdvances: *mut Fixed = 0 as *mut Fixed;
        let mut totalGlyphCount: i32 = 0i32;
        /* need to find direction runs within the text, and call layoutChars separately for each */
        let mut dir: icu::UBiDiDirection = icu::UBIDI_LTR;
        let mut glyph_info: *mut libc::c_void = 0 as *mut libc::c_void;
        static mut positions: *mut FloatPoint = 0 as *const FloatPoint as *mut FloatPoint;
        static mut advances: *mut f32 = 0 as *const f32 as *mut f32;
        static mut glyphs: *mut u32 = 0 as *const u32 as *mut u32;
        let mut pBiDi: *mut icu::UBiDi = icu::ubidi_open();
        let mut errorCode: icu::UErrorCode = icu::U_ZERO_ERROR;
        icu::ubidi_setPara(
            pBiDi,
            txtPtr as *const icu::UChar,
            txtLen,
            getDefaultDirection(engine) as icu::UBiDiLevel,
            0 as *mut icu::UBiDiLevel,
            &mut errorCode,
        );
        dir = icu::ubidi_getDirection(pBiDi);
        if dir as u32 == icu::UBIDI_MIXED as i32 as u32 {
            /* we actually do the layout twice here, once to count glyphs and then again to get them;
               which is inefficient, but i figure that MIXED is a relatively rare occurrence, so i can't be
               bothered to deal with the memory reallocation headache of doing it differently
            */
            let mut nRuns: i32 = icu::ubidi_countRuns(pBiDi, &mut errorCode);
            let mut width: f64 = 0i32 as f64;
            let mut i: i32 = 0;
            let mut runIndex: i32 = 0;
            let mut logicalStart: i32 = 0;
            let mut length: i32 = 0;
            runIndex = 0i32;
            while runIndex < nRuns {
                dir = icu::ubidi_getVisualRun(pBiDi, runIndex, &mut logicalStart, &mut length);
                totalGlyphCount += layoutChars(
                    engine,
                    txtPtr,
                    logicalStart,
                    length,
                    txtLen,
                    dir as u32 == icu::UBIDI_RTL as i32 as u32,
                );
                runIndex += 1
            }
            if totalGlyphCount > 0i32 {
                let mut x: f64 = 0.;
                let mut y: f64 = 0.;
                glyph_info = xcalloc(totalGlyphCount as size_t, 10i32 as size_t);
                locations = glyph_info as *mut FixedPoint;
                glyphIDs = locations.offset(totalGlyphCount as isize) as *mut u16;
                glyphAdvances = xcalloc(
                    totalGlyphCount as size_t,
                    ::std::mem::size_of::<Fixed>() as u64,
                ) as *mut Fixed;
                totalGlyphCount = 0i32;
                y = 0.0f64;
                x = y;
                runIndex = 0i32;
                while runIndex < nRuns {
                    let mut nGlyphs: i32 = 0;
                    dir = icu::ubidi_getVisualRun(pBiDi, runIndex, &mut logicalStart, &mut length);
                    nGlyphs = layoutChars(
                        engine,
                        txtPtr,
                        logicalStart,
                        length,
                        txtLen,
                        dir as u32 == icu::UBIDI_RTL as i32 as u32,
                    );
                    glyphs =
                        xcalloc(nGlyphs as size_t, ::std::mem::size_of::<u32>() as u64) as *mut u32;
                    positions = xcalloc(
                        (nGlyphs + 1i32) as size_t,
                        ::std::mem::size_of::<FloatPoint>() as u64,
                    ) as *mut FloatPoint;
                    advances =
                        xcalloc(nGlyphs as size_t, ::std::mem::size_of::<f32>() as u64) as *mut f32;
                    getGlyphs(engine, glyphs);
                    getGlyphAdvances(engine, advances);
                    getGlyphPositions(engine, positions);
                    i = 0i32;
                    while i < nGlyphs {
                        *glyphIDs.offset(totalGlyphCount as isize) =
                            *glyphs.offset(i as isize) as u16;
                        (*locations.offset(totalGlyphCount as isize)).x =
                            D2Fix((*positions.offset(i as isize)).x as f64 + x);
                        (*locations.offset(totalGlyphCount as isize)).y =
                            D2Fix((*positions.offset(i as isize)).y as f64 + y);
                        *glyphAdvances.offset(totalGlyphCount as isize) =
                            D2Fix(*advances.offset(i as isize) as f64);
                        totalGlyphCount += 1;
                        i += 1
                    }
                    x += (*positions.offset(nGlyphs as isize)).x as f64;
                    y += (*positions.offset(nGlyphs as isize)).y as f64;
                    free(glyphs as *mut libc::c_void);
                    free(positions as *mut libc::c_void);
                    free(advances as *mut libc::c_void);
                    runIndex += 1
                }
                width = x
            }
            (*node.offset(1)).b32.s1 = D2Fix(width);
            (*node.offset(4)).b16.s0 = totalGlyphCount as u16;
            let ref mut fresh29 = (*node.offset(5)).ptr;
            *fresh29 = glyph_info
        } else {
            let mut width_0: f64 = 0i32 as f64;
            totalGlyphCount = layoutChars(
                engine,
                txtPtr,
                0i32,
                txtLen,
                txtLen,
                dir as u32 == icu::UBIDI_RTL as i32 as u32,
            );
            glyphs = xcalloc(
                totalGlyphCount as size_t,
                ::std::mem::size_of::<u32>() as u64,
            ) as *mut u32;
            positions = xcalloc(
                (totalGlyphCount + 1i32) as size_t,
                ::std::mem::size_of::<FloatPoint>() as u64,
            ) as *mut FloatPoint;
            advances = xcalloc(
                totalGlyphCount as size_t,
                ::std::mem::size_of::<f32>() as u64,
            ) as *mut f32;
            getGlyphs(engine, glyphs);
            getGlyphAdvances(engine, advances);
            getGlyphPositions(engine, positions);
            if totalGlyphCount > 0i32 {
                let mut i_0: i32 = 0;
                glyph_info = xcalloc(totalGlyphCount as size_t, 10i32 as size_t);
                locations = glyph_info as *mut FixedPoint;
                glyphIDs = locations.offset(totalGlyphCount as isize) as *mut u16;
                glyphAdvances = xcalloc(
                    totalGlyphCount as size_t,
                    ::std::mem::size_of::<Fixed>() as u64,
                ) as *mut Fixed;
                i_0 = 0i32;
                while i_0 < totalGlyphCount {
                    *glyphIDs.offset(i_0 as isize) = *glyphs.offset(i_0 as isize) as u16;
                    *glyphAdvances.offset(i_0 as isize) =
                        D2Fix(*advances.offset(i_0 as isize) as f64);
                    (*locations.offset(i_0 as isize)).x =
                        D2Fix((*positions.offset(i_0 as isize)).x as f64);
                    (*locations.offset(i_0 as isize)).y =
                        D2Fix((*positions.offset(i_0 as isize)).y as f64);
                    i_0 += 1
                }
                width_0 = (*positions.offset(totalGlyphCount as isize)).x as f64
            }
            (*node.offset(1)).b32.s1 = D2Fix(width_0);
            (*node.offset(4)).b16.s0 = totalGlyphCount as u16;
            let ref mut fresh30 = (*node.offset(5)).ptr;
            *fresh30 = glyph_info;
            free(glyphs as *mut libc::c_void);
            free(positions as *mut libc::c_void);
            free(advances as *mut libc::c_void);
        }
        icu::ubidi_close(pBiDi);
        if *font_letter_space.offset(f as isize) != 0i32 {
            let mut lsDelta: Fixed = 0i32;
            let mut lsUnit: Fixed = *font_letter_space.offset(f as isize);
            let mut i_1: i32 = 0;
            i_1 = 0i32;
            while i_1 < totalGlyphCount {
                if *glyphAdvances.offset(i_1 as isize) == 0i32 && lsDelta != 0i32 {
                    lsDelta -= lsUnit
                }
                let ref mut fresh31 = (*locations.offset(i_1 as isize)).x;
                *fresh31 += lsDelta;
                lsDelta += lsUnit;
                i_1 += 1
            }
            if lsDelta != 0i32 {
                lsDelta -= lsUnit;
                let ref mut fresh32 = (*node.offset(1)).b32.s1;
                *fresh32 += lsDelta
            }
        }
        free(glyphAdvances as *mut libc::c_void);
    } else {
        panic!("bad native font flag in `measure_native_node`");
    }
    if use_glyph_metrics == 0i32 || (*node.offset(4)).b16.s0 as i32 == 0i32 {
        /* for efficiency, height and depth are the font's ascent/descent,
        not true values based on the actual content of the word,
        unless use_glyph_metrics is non-zero */
        (*node.offset(3)).b32.s1 = *height_base.offset(f as isize);
        (*node.offset(2)).b32.s1 = *depth_base.offset(f as isize)
    } else {
        /* this iterates over the glyph data whether it comes from AAT or OT layout */
        let mut locations_0: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint; /* NB negative is upwards in locations[].y! */
        let mut glyphIDs_0: *mut u16 =
            locations_0.offset((*node.offset(4)).b16.s0 as i32 as isize) as *mut u16;
        let mut yMin: f32 = 65536.0f64 as f32;
        let mut yMax: f32 = -65536.0f64 as f32;
        let mut i_2: i32 = 0;
        i_2 = 0i32;
        while i_2 < (*node.offset(4)).b16.s0 as i32 {
            let mut ht: f32 = 0.;
            let mut dp: f32 = 0.;
            let mut y_0: f32 = Fix2D(-(*locations_0.offset(i_2 as isize)).y) as f32;
            let mut bbox: GlyphBBox = GlyphBBox {
                xMin: 0.,
                yMin: 0.,
                xMax: 0.,
                yMax: 0.,
            };
            if getCachedGlyphBBox(f as u16, *glyphIDs_0.offset(i_2 as isize), &mut bbox) == 0i32 {
                match *font_area.offset(f as isize) as u32 {
                    #[cfg(target_os = "macos")]
                    0xffffu32 => {
                        aat::GetGlyphBBox_AAT(
                            *font_layout_engine.offset(f as isize) as CFDictionaryRef,
                            *glyphIDs_0.offset(i_2 as isize),
                            &mut bbox,
                        );
                    }
                    0xfffeu32 => {
                        getGlyphBounds(
                            *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine,
                            *glyphIDs_0.offset(i_2 as isize) as u32,
                            &mut bbox,
                        );
                    }
                    _ => {}
                }
                cacheGlyphBBox(f as u16, *glyphIDs_0.offset(i_2 as isize), &mut bbox);
            }
            ht = bbox.yMax;
            dp = -bbox.yMin;
            if y_0 + ht > yMax {
                yMax = y_0 + ht
            }
            if y_0 - dp < yMin {
                yMin = y_0 - dp
            }
            i_2 += 1
        }
        (*node.offset(3)).b32.s1 = D2Fix(yMax as f64);
        (*node.offset(2)).b32.s1 = -D2Fix(yMin as f64)
    };
}
#[no_mangle]
pub unsafe extern "C" fn real_get_native_italic_correction(mut pNode: *mut libc::c_void) -> Fixed {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut f: u32 = (*node.offset(4)).b16.s2 as u32;
    let mut n: u32 = (*node.offset(4)).b16.s0 as u32;
    if n > 0_u32 {
        let mut locations: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint;
        let mut glyphIDs: *mut u16 = locations.offset(n as isize) as *mut u16;
        match *font_area.offset(f as isize) as u32 {
            #[cfg(target_os = "macos")]
            0xffffu32 => {
                return D2Fix(aat::GetGlyphItalCorr_AAT(
                    *font_layout_engine.offset(f as isize) as CFDictionaryRef,
                    *glyphIDs.offset(n.wrapping_sub(1i32 as libc::c_uint) as isize),
                )) + *font_letter_space.offset(f as isize);
            }
            0xfffeu32 => {
                return D2Fix(getGlyphItalCorr(
                    *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine,
                    *glyphIDs.offset(n.wrapping_sub(1_u32) as isize) as u32,
                ) as f64)
                    + *font_letter_space.offset(f as isize);
            }
            _ => 0i32,
        }
    } else {
        0i32
    }
}
#[no_mangle]
pub unsafe extern "C" fn real_get_native_glyph_italic_correction(
    mut pNode: *mut libc::c_void,
) -> Fixed {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut gid: u16 = (*node.offset(4)).b16.s1;
    let mut f: u32 = (*node.offset(4)).b16.s2 as u32;
    match *font_area.offset(f as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            return D2Fix(aat::GetGlyphItalCorr_AAT(
                *font_layout_engine.offset(f as isize) as CFDictionaryRef,
                gid,
            ));
        }
        0xfffeu32 => {
            return D2Fix(getGlyphItalCorr(
                *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine,
                gid as u32,
            ) as f64);
        }
        _ => {
            0i32
            /* can't actually happen */
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn measure_native_glyph(
    mut pNode: *mut libc::c_void,
    mut use_glyph_metrics: i32,
) {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut gid: u16 = (*node.offset(4)).b16.s1;
    let mut f: u32 = (*node.offset(4)).b16.s2 as u32;
    let mut ht: f32 = 0.0f64 as f32;
    let mut dp: f32 = 0.0f64 as f32;
    match *font_area.offset(f as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            let mut attributes: CFDictionaryRef =
                *font_layout_engine.offset(f as isize) as CFDictionaryRef;
            (*node.offset(1)).b32.s1 = D2Fix(aat::GetGlyphWidth_AAT(attributes, gid));
            if use_glyph_metrics != 0 {
                aat::GetGlyphHeightDepth_AAT(attributes, gid, &mut ht, &mut dp);
            }
        }
        0xfffeu32 => {
            let mut engine: XeTeXLayoutEngine =
                *font_layout_engine.offset(f as isize) as XeTeXLayoutEngine;
            let mut fontInst: XeTeXFont = getFont(engine);
            (*node.offset(1)).b32.s1 = D2Fix(getGlyphWidth(fontInst, gid as u32) as f64);
            if use_glyph_metrics != 0 {
                getGlyphHeightDepth(engine, gid as u32, &mut ht, &mut dp);
            }
        }
        _ => {
            panic!("bad native font flag in `measure_native_glyph`");
        }
    }
    if use_glyph_metrics != 0 {
        (*node.offset(3)).b32.s1 = D2Fix(ht as f64);
        (*node.offset(2)).b32.s1 = D2Fix(dp as f64)
    } else {
        (*node.offset(3)).b32.s1 = *height_base.offset(f as isize);
        (*node.offset(2)).b32.s1 = *depth_base.offset(f as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn map_char_to_glyph(mut font: i32, mut ch: i32) -> i32 {
    if ch > 0x10ffffi32 || ch >= 0xd800i32 && ch <= 0xdfffi32 {
        return 0i32;
    }
    match *font_area.offset(font as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            return aat::MapCharToGlyph_AAT(
                *font_layout_engine.offset(font as isize) as CFDictionaryRef,
                ch as UInt32,
            );
        }
        0xfffeu32 => {
            return mapCharToGlyph(
                *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine,
                ch as u32,
            ) as i32;
        }
        _ => {
            panic!("bad native font flag in `map_char_to_glyph`");
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn map_glyph_to_index(mut font: i32) -> i32
/* glyph name is at name_of_file */ {
    match *font_area.offset(font as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            return aat::MapGlyphToIndex_AAT(
                *font_layout_engine.offset(font as isize) as CFDictionaryRef,
                name_of_file,
            );
        }
        0xfffeu32 => {
            return mapGlyphToIndex(
                *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine,
                name_of_file,
            );
        }
        _ => {
            panic!("bad native font flag in `map_glyph_to_index`");
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_font_char_range(mut font: i32, mut first: i32) -> i32 {
    match *font_area.offset(font as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            return aat::GetFontCharRange_AAT(
                *font_layout_engine.offset(font as isize) as CFDictionaryRef,
                first,
            );
        }
        0xfffeu32 => {
            return getFontCharRange(
                *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine,
                first,
            );
        }
        _ => {
            panic!("bad native font flag in `get_font_char_range\'`");
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn D2Fix(mut d: f64) -> Fixed {
    let rval: Fixed = (d * 65536.0f64 + 0.5f64) as i32;
    rval
}
#[no_mangle]
pub unsafe extern "C" fn Fix2D(mut f: Fixed) -> f64 {
    f as f64 / 65536.
}

#[no_mangle]
pub unsafe extern "C" fn print_glyph_name(mut font: i32, mut gid: i32) {
    let mut s: *const i8 = 0 as *const i8;
    let mut len: i32 = 0i32;
    match *font_area.offset(font as isize) as u32 {
        #[cfg(target_os = "macos")]
        0xffffu32 => {
            s = aat::GetGlyphNameFromCTFont(aat::font_from_integer(font), gid as u16, &mut len);
        }
        0xfffeu32 => {
            let mut engine: XeTeXLayoutEngine =
                *font_layout_engine.offset(font as isize) as XeTeXLayoutEngine;
            s = getGlyphName(getFont(engine), gid as u16, &mut len);
        }
        _ => {
            panic!("bad native font flag in `print_glyph_name`");
        }
    }
    loop {
        let fresh33 = len;
        len = len - 1;
        if !(fresh33 > 0i32) {
            break;
        }
        let fresh34 = s;
        s = s.offset(1);
        print_char(*fresh34 as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn real_get_native_word_cp(
    mut pNode: *mut libc::c_void,
    mut side: i32,
) -> i32 {
    let mut node: *mut memory_word = pNode as *mut memory_word;
    let mut locations: *mut FixedPoint = (*node.offset(5)).ptr as *mut FixedPoint;
    let mut glyphIDs: *mut u16 =
        locations.offset((*node.offset(4)).b16.s0 as i32 as isize) as *mut u16;
    let mut glyphCount: u16 = (*node.offset(4)).b16.s0;
    let mut f: i32 = (*node.offset(4)).b16.s2 as i32;
    let mut actual_glyph: u16 = 0;
    if glyphCount as i32 == 0i32 {
        return 0i32;
    }
    match side {
        0 => {
            actual_glyph = *glyphIDs
            // we should not reach this point
        }
        1 => actual_glyph = *glyphIDs.offset((glyphCount as i32 - 1i32) as isize),
        _ => unreachable!(),
    }
    get_cp_code(f, actual_glyph as u32, side)
}
