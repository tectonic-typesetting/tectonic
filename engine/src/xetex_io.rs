#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
use crate::stub_icu as icu;
extern "C" {
    pub type Opaque_TECkit_Converter;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_open_primary() -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_getc(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_ungetc(handle: rust_input_handle_t, ch: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    /* tectonic/core-memory.h: basic dynamic memory helpers
       Copyright 2016-2018 the Tectonic Project
       Licensed under the MIT License.
    */
    #[no_mangle]
    fn xstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn xcalloc(nelem: size_t, elsize: size_t) -> *mut libc::c_void;
    /*
        Convert text from a buffer in memory
    */
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
    fn TECkit_CreateConverter(
        mapping: *mut Byte,
        mappingSize: UInt32,
        mapForward: Byte,
        sourceForm: UInt16,
        targetForm: UInt16,
        converter: *mut TECkit_Converter,
    ) -> TECkit_Status;
    #[no_mangle]
    fn gettexstring(_: str_number) -> *mut libc::c_char;
    /* Needed here for UFILE */
    /* variables! */
    /* All the following variables are defined in xetexini.c */
    #[no_mangle]
    static mut eqtb: *mut memory_word;
    #[no_mangle]
    static mut name_of_file: *mut libc::c_char;
    #[no_mangle]
    static mut name_of_file16: *mut UTF16_code;
    #[no_mangle]
    static mut name_length: int32_t;
    #[no_mangle]
    static mut name_length16: int32_t;
    #[no_mangle]
    static mut buffer: *mut UnicodeScalar;
    #[no_mangle]
    static mut first: int32_t;
    #[no_mangle]
    static mut last: int32_t;
    #[no_mangle]
    static mut max_buf_stack: int32_t;
    #[no_mangle]
    static mut buf_size: int32_t;
    #[no_mangle]
    static mut cur_chr: int32_t;
    #[no_mangle]
    static mut cur_val: int32_t;
    #[no_mangle]
    static mut read_file: [*mut UFILE; 16];
    #[no_mangle]
    static mut read_open: [u8; 17];
    #[no_mangle]
    static mut cur_name: str_number;
    #[no_mangle]
    static mut cur_area: str_number;
    #[no_mangle]
    static mut cur_ext: str_number;
    #[no_mangle]
    static mut name_in_progress: bool;
    /* xetex-errors */
    /* xetex-math */
    /* xetex-output */
    #[no_mangle]
    fn print_nl(s: str_number);
    #[no_mangle]
    fn get_input_normalization_state() -> int32_t;
    #[no_mangle]
    fn bad_utf8_warning();
    #[no_mangle]
    fn begin_diagnostic();
    #[no_mangle]
    fn print_int(n: int32_t);
    #[no_mangle]
    fn print_char(s: int32_t);
    #[no_mangle]
    fn end_diagnostic(blank_line: bool);
    #[no_mangle]
    static mut stop_at_space: bool;
    #[no_mangle]
    fn scan_optional_equals();
    #[no_mangle]
    fn scan_four_bit_int();
    #[no_mangle]
    fn begin_name();
    #[no_mangle]
    fn more_name(c: UTF16_code) -> bool;
    #[no_mangle]
    fn end_name();
    #[no_mangle]
    fn pack_file_name(n: str_number, a: str_number, e: str_number);
    #[no_mangle]
    fn scan_file_name();
}

use crate::*;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
pub type UErrorCode = libc::c_int;
pub const U_ERROR_LIMIT: UErrorCode = 66818;
pub const U_PLUGIN_ERROR_LIMIT: UErrorCode = 66818;
pub const U_PLUGIN_DIDNT_SET_LEVEL: UErrorCode = 66817;
pub const U_PLUGIN_TOO_HIGH: UErrorCode = 66816;
pub const U_PLUGIN_ERROR_START: UErrorCode = 66816;
pub const U_STRINGPREP_CHECK_BIDI_ERROR: UErrorCode = 66562;
pub const U_STRINGPREP_UNASSIGNED_ERROR: UErrorCode = 66561;
pub const U_STRINGPREP_PROHIBITED_ERROR: UErrorCode = 66560;
pub const U_IDNA_ERROR_LIMIT: UErrorCode = 66569;
pub const U_IDNA_DOMAIN_NAME_TOO_LONG_ERROR: UErrorCode = 66568;
pub const U_IDNA_ZERO_LENGTH_LABEL_ERROR: UErrorCode = 66567;
pub const U_IDNA_LABEL_TOO_LONG_ERROR: UErrorCode = 66566;
pub const U_IDNA_VERIFICATION_ERROR: UErrorCode = 66565;
pub const U_IDNA_ACE_PREFIX_ERROR: UErrorCode = 66564;
pub const U_IDNA_STD3_ASCII_RULES_ERROR: UErrorCode = 66563;
pub const U_IDNA_CHECK_BIDI_ERROR: UErrorCode = 66562;
pub const U_IDNA_UNASSIGNED_ERROR: UErrorCode = 66561;
pub const U_IDNA_ERROR_START: UErrorCode = 66560;
pub const U_IDNA_PROHIBITED_ERROR: UErrorCode = 66560;
pub const U_REGEX_ERROR_LIMIT: UErrorCode = 66326;
pub const U_REGEX_INVALID_CAPTURE_GROUP_NAME: UErrorCode = 66325;
pub const U_REGEX_PATTERN_TOO_BIG: UErrorCode = 66324;
pub const U_REGEX_STOPPED_BY_CALLER: UErrorCode = 66323;
pub const U_REGEX_TIME_OUT: UErrorCode = 66322;
pub const U_REGEX_STACK_OVERFLOW: UErrorCode = 66321;
pub const U_REGEX_INVALID_RANGE: UErrorCode = 66320;
pub const U_REGEX_MISSING_CLOSE_BRACKET: UErrorCode = 66319;
pub const U_REGEX_OCTAL_TOO_BIG: UErrorCode = 66318;
pub const U_REGEX_SET_CONTAINS_STRING: UErrorCode = 66317;
pub const U_REGEX_LOOK_BEHIND_LIMIT: UErrorCode = 66316;
pub const U_REGEX_INVALID_FLAG: UErrorCode = 66315;
pub const U_REGEX_INVALID_BACK_REF: UErrorCode = 66314;
pub const U_REGEX_MAX_LT_MIN: UErrorCode = 66313;
pub const U_REGEX_BAD_INTERVAL: UErrorCode = 66312;
pub const U_REGEX_NUMBER_TOO_BIG: UErrorCode = 66311;
pub const U_REGEX_MISMATCHED_PAREN: UErrorCode = 66310;
pub const U_REGEX_UNIMPLEMENTED: UErrorCode = 66309;
pub const U_REGEX_PROPERTY_SYNTAX: UErrorCode = 66308;
pub const U_REGEX_BAD_ESCAPE_SEQUENCE: UErrorCode = 66307;
pub const U_REGEX_INVALID_STATE: UErrorCode = 66306;
pub const U_REGEX_RULE_SYNTAX: UErrorCode = 66305;
pub const U_REGEX_ERROR_START: UErrorCode = 66304;
pub const U_REGEX_INTERNAL_ERROR: UErrorCode = 66304;
pub const U_BRK_ERROR_LIMIT: UErrorCode = 66062;
pub const U_BRK_MALFORMED_RULE_TAG: UErrorCode = 66061;
pub const U_BRK_UNRECOGNIZED_OPTION: UErrorCode = 66060;
pub const U_BRK_RULE_EMPTY_SET: UErrorCode = 66059;
pub const U_BRK_INIT_ERROR: UErrorCode = 66058;
pub const U_BRK_UNDEFINED_VARIABLE: UErrorCode = 66057;
pub const U_BRK_NEW_LINE_IN_QUOTED_STRING: UErrorCode = 66056;
pub const U_BRK_MISMATCHED_PAREN: UErrorCode = 66055;
pub const U_BRK_VARIABLE_REDFINITION: UErrorCode = 66054;
pub const U_BRK_ASSIGN_ERROR: UErrorCode = 66053;
pub const U_BRK_UNCLOSED_SET: UErrorCode = 66052;
pub const U_BRK_RULE_SYNTAX: UErrorCode = 66051;
pub const U_BRK_SEMICOLON_EXPECTED: UErrorCode = 66050;
pub const U_BRK_HEX_DIGITS_EXPECTED: UErrorCode = 66049;
pub const U_BRK_ERROR_START: UErrorCode = 66048;
pub const U_BRK_INTERNAL_ERROR: UErrorCode = 66048;
pub const U_FMT_PARSE_ERROR_LIMIT: UErrorCode = 65812;
pub const U_NUMBER_SKELETON_SYNTAX_ERROR: UErrorCode = 65811;
pub const U_NUMBER_ARG_OUTOFBOUNDS_ERROR: UErrorCode = 65810;
pub const U_FORMAT_INEXACT_ERROR: UErrorCode = 65809;
pub const U_DECIMAL_NUMBER_SYNTAX_ERROR: UErrorCode = 65808;
pub const U_DEFAULT_KEYWORD_MISSING: UErrorCode = 65807;
pub const U_UNDEFINED_KEYWORD: UErrorCode = 65806;
pub const U_DUPLICATE_KEYWORD: UErrorCode = 65805;
pub const U_ARGUMENT_TYPE_MISMATCH: UErrorCode = 65804;
pub const U_UNSUPPORTED_ATTRIBUTE: UErrorCode = 65803;
pub const U_UNSUPPORTED_PROPERTY: UErrorCode = 65802;
pub const U_UNMATCHED_BRACES: UErrorCode = 65801;
pub const U_ILLEGAL_PAD_POSITION: UErrorCode = 65800;
pub const U_PATTERN_SYNTAX_ERROR: UErrorCode = 65799;
pub const U_MULTIPLE_PAD_SPECIFIERS: UErrorCode = 65798;
pub const U_MULTIPLE_PERMILL_SYMBOLS: UErrorCode = 65797;
pub const U_MULTIPLE_PERCENT_SYMBOLS: UErrorCode = 65796;
pub const U_MALFORMED_EXPONENTIAL_PATTERN: UErrorCode = 65795;
pub const U_MULTIPLE_EXPONENTIAL_SYMBOLS: UErrorCode = 65794;
pub const U_MULTIPLE_DECIMAL_SEPERATORS: UErrorCode = 65793;
pub const U_MULTIPLE_DECIMAL_SEPARATORS: UErrorCode = 65793;
pub const U_FMT_PARSE_ERROR_START: UErrorCode = 65792;
pub const U_UNEXPECTED_TOKEN: UErrorCode = 65792;
pub const U_PARSE_ERROR_LIMIT: UErrorCode = 65571;
pub const U_INVALID_FUNCTION: UErrorCode = 65570;
pub const U_INVALID_ID: UErrorCode = 65569;
pub const U_INTERNAL_TRANSLITERATOR_ERROR: UErrorCode = 65568;
pub const U_ILLEGAL_CHARACTER: UErrorCode = 65567;
pub const U_VARIABLE_RANGE_OVERLAP: UErrorCode = 65566;
pub const U_VARIABLE_RANGE_EXHAUSTED: UErrorCode = 65565;
pub const U_ILLEGAL_CHAR_IN_SEGMENT: UErrorCode = 65564;
pub const U_UNCLOSED_SEGMENT: UErrorCode = 65563;
pub const U_MALFORMED_PRAGMA: UErrorCode = 65562;
pub const U_INVALID_PROPERTY_PATTERN: UErrorCode = 65561;
pub const U_INVALID_RBT_SYNTAX: UErrorCode = 65560;
pub const U_MULTIPLE_COMPOUND_FILTERS: UErrorCode = 65559;
pub const U_MISPLACED_COMPOUND_FILTER: UErrorCode = 65558;
pub const U_RULE_MASK_ERROR: UErrorCode = 65557;
pub const U_UNTERMINATED_QUOTE: UErrorCode = 65556;
pub const U_UNQUOTED_SPECIAL: UErrorCode = 65555;
pub const U_UNDEFINED_VARIABLE: UErrorCode = 65554;
pub const U_UNDEFINED_SEGMENT_REFERENCE: UErrorCode = 65553;
pub const U_TRAILING_BACKSLASH: UErrorCode = 65552;
pub const U_MULTIPLE_POST_CONTEXTS: UErrorCode = 65551;
pub const U_MULTIPLE_CURSORS: UErrorCode = 65550;
pub const U_MULTIPLE_ANTE_CONTEXTS: UErrorCode = 65549;
pub const U_MISSING_SEGMENT_CLOSE: UErrorCode = 65548;
pub const U_MISSING_OPERATOR: UErrorCode = 65547;
pub const U_MISPLACED_QUANTIFIER: UErrorCode = 65546;
pub const U_MISPLACED_CURSOR_OFFSET: UErrorCode = 65545;
pub const U_MISPLACED_ANCHOR_START: UErrorCode = 65544;
pub const U_MISMATCHED_SEGMENT_DELIMITERS: UErrorCode = 65543;
pub const U_MALFORMED_VARIABLE_REFERENCE: UErrorCode = 65542;
pub const U_MALFORMED_VARIABLE_DEFINITION: UErrorCode = 65541;
pub const U_MALFORMED_UNICODE_ESCAPE: UErrorCode = 65540;
pub const U_MALFORMED_SYMBOL_REFERENCE: UErrorCode = 65539;
pub const U_MALFORMED_SET: UErrorCode = 65538;
pub const U_MALFORMED_RULE: UErrorCode = 65537;
pub const U_PARSE_ERROR_START: UErrorCode = 65536;
pub const U_BAD_VARIABLE_DEFINITION: UErrorCode = 65536;
pub const U_STANDARD_ERROR_LIMIT: UErrorCode = 31;
pub const U_NO_WRITE_PERMISSION: UErrorCode = 30;
pub const U_USELESS_COLLATOR_ERROR: UErrorCode = 29;
pub const U_COLLATOR_VERSION_MISMATCH: UErrorCode = 28;
pub const U_INVALID_STATE_ERROR: UErrorCode = 27;
pub const U_INVARIANT_CONVERSION_ERROR: UErrorCode = 26;
pub const U_ENUM_OUT_OF_SYNC_ERROR: UErrorCode = 25;
pub const U_TOO_MANY_ALIASES_ERROR: UErrorCode = 24;
pub const U_STATE_TOO_OLD_ERROR: UErrorCode = 23;
pub const U_PRIMARY_TOO_LONG_ERROR: UErrorCode = 22;
pub const U_CE_NOT_FOUND_ERROR: UErrorCode = 21;
pub const U_NO_SPACE_AVAILABLE: UErrorCode = 20;
pub const U_UNSUPPORTED_ESCAPE_SEQUENCE: UErrorCode = 19;
pub const U_ILLEGAL_ESCAPE_SEQUENCE: UErrorCode = 18;
pub const U_RESOURCE_TYPE_MISMATCH: UErrorCode = 17;
pub const U_UNSUPPORTED_ERROR: UErrorCode = 16;
pub const U_BUFFER_OVERFLOW_ERROR: UErrorCode = 15;
pub const U_INVALID_TABLE_FILE: UErrorCode = 14;
pub const U_INVALID_TABLE_FORMAT: UErrorCode = 13;
pub const U_ILLEGAL_CHAR_FOUND: UErrorCode = 12;
pub const U_TRUNCATED_CHAR_FOUND: UErrorCode = 11;
pub const U_INVALID_CHAR_FOUND: UErrorCode = 10;
pub const U_PARSE_ERROR: UErrorCode = 9;
pub const U_INDEX_OUTOFBOUNDS_ERROR: UErrorCode = 8;
pub const U_MEMORY_ALLOCATION_ERROR: UErrorCode = 7;
pub const U_MESSAGE_PARSE_ERROR: UErrorCode = 6;
pub const U_INTERNAL_PROGRAM_ERROR: UErrorCode = 5;
pub const U_FILE_ACCESS_ERROR: UErrorCode = 4;
pub const U_INVALID_FORMAT_ERROR: UErrorCode = 3;
pub const U_MISSING_RESOURCE_ERROR: UErrorCode = 2;
pub const U_ILLEGAL_ARGUMENT_ERROR: UErrorCode = 1;
pub const U_ZERO_ERROR: UErrorCode = 0;
pub const U_ERROR_WARNING_LIMIT: UErrorCode = -119;
pub const U_PLUGIN_CHANGED_LEVEL_WARNING: UErrorCode = -120;
pub const U_DIFFERENT_UCA_VERSION: UErrorCode = -121;
pub const U_AMBIGUOUS_ALIAS_WARNING: UErrorCode = -122;
pub const U_SORT_KEY_TOO_SHORT_WARNING: UErrorCode = -123;
pub const U_STRING_NOT_TERMINATED_WARNING: UErrorCode = -124;
pub const U_STATE_OLD_WARNING: UErrorCode = -125;
pub const U_SAFECLONE_ALLOCATED_WARNING: UErrorCode = -126;
pub const U_USING_DEFAULT_WARNING: UErrorCode = -127;
pub const U_ERROR_WARNING_START: UErrorCode = -128;
pub const U_USING_FALLBACK_WARNING: UErrorCode = -128;
/* quasi-hack to get the primary input */
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
    16-Sep-2006		jk	updated version to 2.4 (adding new compiler APIs for Bob E)
    23-May-2005		jk	patch for 64-bit architectures (thanks to Ulrik P)
    18-Mar-2005		jk	updated minor version for 2.3 (engine unchanged, XML option in compiler)
    23-Sep-2003		jk	updated for version 2.1 - extended status values
    xx-xxx-2002		jk	version 2.0 initial release
*/
/* 16.16 version number */
/* these are all predefined if using a Mac prefix */
pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = libc::c_uint;
/* NB: assumes int is 4 bytes */
/* n.b. if also using zlib.h, it must precede TECkit headers */
pub type Byte = UInt8;
/*
    all public functions return a status code
*/
pub type TECkit_Status = libc::c_long;
/*
    end of text value for TECkit_DataSource functions to return
*/
/*
    A converter object is an opaque pointer
*/
pub type TECkit_Converter = *mut Opaque_TECkit_Converter;
/* tectonic/xetex-xetexd.h -- many, many XeTeX symbol definitions
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/
/* Extra stuff used in various change files for various reasons.  */
/* Array allocations. Add 1 to size to account for Pascal indexing convention. */
/*11:*/
/*18: */
pub type UTF16_code = u16;
pub type UnicodeScalar = int32_t;
pub type str_number = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: int32_t,
    pub s1: int32_t,
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
pub struct b16x4_le_t {
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: libc::c_double,
    pub ptr: *mut libc::c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct UFILE {
    pub handle: rust_input_handle_t,
    pub savedChar: libc::c_long,
    pub skipNextLF: libc::c_short,
    pub encodingMode: libc::c_short,
    pub conversionData: *mut libc::c_void,
}

#[inline]
unsafe extern "C" fn print_c_string(mut str: *const libc::c_char) {
    while *str != 0 {
        let fresh0 = str;
        str = str.offset(1);
        print_char(*fresh0 as int32_t);
    }
}
/* tectonic/xetex-io.c: low-level input/output functions tied to the XeTeX engine
   Copyright 2016-2019 The Tectonic Project
   Licensed under the MIT License.
*/
#[no_mangle]
pub static mut name_of_input_file: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn tt_xetex_open_input(mut filefmt: libc::c_int) -> rust_input_handle_t {
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    if filefmt == TTIF_TECTONIC_PRIMARY as libc::c_int {
        handle = ttstub_input_open_primary()
    } else {
        handle = ttstub_input_open(name_of_file, filefmt as tt_input_format_type, 0i32)
    }
    if handle.is_null() {
        return 0 as *mut libc::c_void;
    }
    name_length = strlen(name_of_file) as int32_t;
    free(name_of_input_file as *mut libc::c_void);
    name_of_input_file = xstrdup(name_of_file);
    return handle;
}
/* tables/values used in UTF-8 interpretation -
code is based on ConvertUTF.[ch] sample code
published by the Unicode consortium */
#[no_mangle]
pub static mut offsetsFromUTF8: [u32; 6] = [
    0u64 as u32,
    0x3080u64 as u32,
    0xe2080u64 as u32,
    0x3c82080u64 as u32,
    0xfa082080u64 as u32,
    0x82082080u64 as u32,
];
#[no_mangle]
pub static mut bytesFromUTF8: [u8; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    4,
    4,
    4,
    4,
    5,
    5,
    5,
    5,
];
#[no_mangle]
pub static mut firstByteMark: [u8; 7] = [
    0,
    0,
    0xc0,
    0xe0,
    0xf0,
    0xf8,
    0xfc,
];
#[no_mangle]
pub unsafe extern "C" fn set_input_file_encoding(
    mut f: *mut UFILE,
    mut mode: int32_t,
    mut encodingData: int32_t,
) {
    if (*f).encodingMode as libc::c_int == 5i32 && !(*f).conversionData.is_null() {
        icu::ucnv_close((*f).conversionData as *mut icu::UConverter);
    }
    (*f).conversionData = 0 as *mut libc::c_void;
    match mode {
        1 | 2 | 3 | 4 => (*f).encodingMode = mode as libc::c_short,
        5 => {
            let mut name: *mut libc::c_char = gettexstring(encodingData);
            let mut err: UErrorCode = U_ZERO_ERROR;
            let mut cnv: *mut icu::UConverter = icu::ucnv_open(name, &mut err);
            if cnv.is_null() {
                begin_diagnostic();
                print_nl('E' as i32);
                print_c_string(b"rror \x00" as *const u8 as *const libc::c_char);
                print_int(err as int32_t);
                print_c_string(
                    b" creating Unicode converter for `\x00" as *const u8 as *const libc::c_char,
                );
                print_c_string(name);
                print_c_string(b"\'; reading as raw bytes\x00" as *const u8 as *const libc::c_char);
                end_diagnostic(1i32 != 0);
                (*f).encodingMode = 4i32 as libc::c_short
            } else {
                (*f).encodingMode = 5i32 as libc::c_short;
                (*f).conversionData = cnv as *mut libc::c_void
            }
            free(name as *mut libc::c_void);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn u_open_in(
    mut f: *mut *mut UFILE,
    mut filefmt: int32_t,
    mut fopen_mode: *const libc::c_char,
    mut mode: int32_t,
    mut encodingData: int32_t,
) -> libc::c_int {
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    let mut B1: libc::c_int = 0;
    let mut B2: libc::c_int = 0;
    handle = tt_xetex_open_input(filefmt);
    if handle.is_null() {
        return 0i32;
    }
    *f = xmalloc(::std::mem::size_of::<UFILE>() as u64) as *mut UFILE;
    (**f).encodingMode = 0i32 as libc::c_short;
    (**f).conversionData = 0 as *mut libc::c_void;
    (**f).savedChar = -1i32 as libc::c_long;
    (**f).skipNextLF = 0i32 as libc::c_short;
    (**f).handle = handle;
    if mode == 0i32 {
        /* sniff encoding form */
        B1 = ttstub_input_getc((**f).handle);
        B2 = ttstub_input_getc((**f).handle);
        if B1 == 0xfei32 && B2 == 0xffi32 {
            mode = 2i32
        } else if B2 == 0xfei32 && B1 == 0xffi32 {
            mode = 3i32
        } else if B1 == 0i32 && B2 != 0i32 {
            mode = 2i32;
            ttstub_input_seek((**f).handle, 0i32 as ssize_t, 0i32);
        } else if B2 == 0i32 && B1 != 0i32 {
            mode = 3i32;
            ttstub_input_seek((**f).handle, 0i32 as ssize_t, 0i32);
        } else if B1 == 0xefi32 && B2 == 0xbbi32 {
            let mut B3: libc::c_int = ttstub_input_getc((**f).handle);
            if B3 == 0xbfi32 {
                mode = 1i32
            }
        }
        if mode == 0i32 {
            ttstub_input_seek((**f).handle, 0i32 as ssize_t, 0i32);
            mode = 1i32
        }
    }
    set_input_file_encoding(*f, mode, encodingData);
    return 1i32;
}
unsafe extern "C" fn buffer_overflow() {
    _tt_abort(
        b"unable to read an entire line (buf_size=%u)\x00" as *const u8 as *const libc::c_char,
        buf_size as libc::c_uint,
    );
}
unsafe extern "C" fn conversion_error(mut errcode: libc::c_int) {
    begin_diagnostic();
    print_nl('U' as i32);
    print_c_string(
        b"nicode conversion failed (ICU error code = \x00" as *const u8 as *const libc::c_char,
    );
    print_int(errcode);
    print_c_string(b") discarding any remaining text\x00" as *const u8 as *const libc::c_char);
    end_diagnostic(1i32 != 0);
}
unsafe extern "C" fn apply_normalization(
    mut buf: *mut u32,
    mut len: libc::c_int,
    mut norm: libc::c_int,
) {
    static mut normalizers: [TECkit_Converter; 2] = [
        0 as *const Opaque_TECkit_Converter as TECkit_Converter,
        0 as *const Opaque_TECkit_Converter as TECkit_Converter,
    ];
    let mut status: TECkit_Status = 0;
    let mut inUsed: UInt32 = 0;
    let mut outUsed: UInt32 = 0;
    let mut normPtr: *mut TECkit_Converter =
        &mut *normalizers.as_mut_ptr().offset((norm - 1i32) as isize) as *mut TECkit_Converter;
    if (*normPtr).is_null() {
        status = TECkit_CreateConverter(
            0 as *mut Byte,
            0i32 as UInt32,
            1i32 as Byte,
            6i32 as UInt16,
            (6i32 | (if norm == 1i32 { 0x100i32 } else { 0x200i32 })) as UInt16,
            normPtr,
        );
        if status != 0i32 as libc::c_long {
            _tt_abort(
                b"failed to create normalizer: error code = %d\x00" as *const u8
                    as *const libc::c_char,
                status as libc::c_int,
            );
        }
    }
    status = TECkit_ConvertBuffer(
        *normPtr,
        buf as *mut Byte,
        (len as u64).wrapping_mul(::std::mem::size_of::<UInt32>() as u64)
            as UInt32,
        &mut inUsed,
        &mut *buffer.offset(first as isize) as *mut UnicodeScalar as *mut Byte,
        (::std::mem::size_of::<UnicodeScalar>() as u64)
            .wrapping_mul((buf_size - first) as u64) as UInt32,
        &mut outUsed,
        1i32 as Byte,
    );
    if status != 0i32 as libc::c_long {
        buffer_overflow();
    }
    last = (first as u64).wrapping_add(
        (outUsed as u64)
            .wrapping_div(::std::mem::size_of::<UnicodeScalar>() as u64),
    ) as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn input_line(mut f: *mut UFILE) -> libc::c_int {
    static mut byteBuffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut utf32Buf: *mut u32 = 0 as *const u32 as *mut u32;
    let mut i: libc::c_int = 0;
    let mut tmpLen: libc::c_int = 0;
    let mut norm: libc::c_int = get_input_normalization_state();
    if (*f).handle.is_null() {
        /* NULL 'handle' means this: */
        _tt_abort(
            b"reads from synthetic \"terminal\" file #0 should never happen\x00" as *const u8
                as *const libc::c_char,
        );
    }
    last = first;
    if (*f).encodingMode as libc::c_int == 5i32 {
        let mut bytesRead: u32 = 0i32 as u32;
        let mut cnv: *mut icu::UConverter = 0 as *mut icu::UConverter;
        let mut outLen: libc::c_int = 0;
        let mut errorCode: UErrorCode = U_ZERO_ERROR;
        if byteBuffer.is_null() {
            byteBuffer = xmalloc((buf_size + 1i32) as size_t) as *mut libc::c_char
        }
        /* Recognize either LF or CR as a line terminator; skip initial LF if prev line ended with CR.  */
        i = ttstub_input_getc((*f).handle);
        if (*f).skipNextLF != 0 {
            (*f).skipNextLF = 0i32 as libc::c_short;
            if i == '\n' as i32 {
                i = ttstub_input_getc((*f).handle)
            }
        }
        if i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
            let fresh1 = bytesRead;
            bytesRead = bytesRead.wrapping_add(1);
            *byteBuffer.offset(fresh1 as isize) = i as libc::c_char
        }
        if i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
            while bytesRead < buf_size as libc::c_uint
                && {
                    i = ttstub_input_getc((*f).handle);
                    i != -1i32
                }
                && i != '\n' as i32
                && i != '\r' as i32
            {
                let fresh2 = bytesRead;
                bytesRead = bytesRead.wrapping_add(1);
                *byteBuffer.offset(fresh2 as isize) = i as libc::c_char
            }
        }
        if i == -1i32 && *__errno_location() != 4i32 && bytesRead == 0i32 as libc::c_uint {
            return 0i32;
        }
        if i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
            buffer_overflow();
        }
        /* now apply the mapping to turn external bytes into Unicode characters in buffer */
        cnv = (*f).conversionData as *mut icu::UConverter;
        match norm {
            1 | 2 => {
                // NFC
                // NFD
                if utf32Buf.is_null() {
                    utf32Buf = xcalloc(
                        buf_size as size_t,
                        ::std::mem::size_of::<u32>() as u64,
                    ) as *mut u32
                } // sets 'last' correctly
                tmpLen = icu::ucnv_toAlgorithmic(
                    icu::UCNV_UTF32_LittleEndian,
                    cnv,
                    utf32Buf as *mut libc::c_char,
                    (buf_size as u64)
                        .wrapping_mul(::std::mem::size_of::<u32>() as u64)
                        as int32_t,
                    byteBuffer,
                    bytesRead as int32_t,
                    &mut errorCode,
                );
                if errorCode as libc::c_int != 0i32 {
                    conversion_error(errorCode as libc::c_int);
                    return 0i32;
                }
                apply_normalization(
                    utf32Buf,
                    (tmpLen as u64)
                        .wrapping_div(::std::mem::size_of::<u32>() as u64)
                        as libc::c_int,
                    norm,
                );
            }
            _ => {
                // none
                outLen = icu::ucnv_toAlgorithmic(
                    icu::UCNV_UTF32_LittleEndian,
                    cnv,
                    &mut *buffer.offset(first as isize) as *mut UnicodeScalar as *mut libc::c_char,
                    (::std::mem::size_of::<UnicodeScalar>() as u64)
                        .wrapping_mul((buf_size - first) as u64)
                        as int32_t,
                    byteBuffer,
                    bytesRead as int32_t,
                    &mut errorCode,
                );
                if errorCode as libc::c_int != 0i32 {
                    conversion_error(errorCode as libc::c_int);
                    return 0i32;
                }
                outLen = (outLen as u64)
                    .wrapping_div(::std::mem::size_of::<UnicodeScalar>() as u64)
                    as libc::c_int as libc::c_int;
                last = first + outLen
            }
        }
    } else {
        /* Recognize either LF or CR as a line terminator; skip initial LF if prev line ended with CR.  */
        i = get_uni_c(f);
        if (*f).skipNextLF != 0 {
            (*f).skipNextLF = 0i32 as libc::c_short;
            if i == '\n' as i32 {
                i = get_uni_c(f)
            }
        }
        match norm {
            1 | 2 => {
                // NFC
                // NFD
                // read Unicode chars into utf32Buf as UTF32
                if utf32Buf.is_null() {
                    utf32Buf = xcalloc(
                        buf_size as size_t,
                        ::std::mem::size_of::<u32>() as u64,
                    ) as *mut u32
                }
                tmpLen = 0i32;
                if i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
                    let fresh3 = tmpLen;
                    tmpLen = tmpLen + 1;
                    *utf32Buf.offset(fresh3 as isize) = i as u32
                }
                if i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
                    while tmpLen < buf_size
                        && {
                            i = get_uni_c(f);
                            i != -1i32
                        }
                        && i != '\n' as i32
                        && i != '\r' as i32
                    {
                        let fresh4 = tmpLen;
                        tmpLen = tmpLen + 1;
                        *utf32Buf.offset(fresh4 as isize) = i as u32
                    }
                }
                if i == -1i32 && *__errno_location() != 4i32 && tmpLen == 0i32 {
                    return 0i32;
                }
                /* We didn't get the whole line because our buffer was too small.  */
                if i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
                    buffer_overflow();
                }
                apply_normalization(utf32Buf, tmpLen, norm);
            }
            _ => {
                // none
                if last < buf_size && i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
                    let fresh5 = last;
                    last = last + 1;
                    *buffer.offset(fresh5 as isize) = i
                }
                if i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
                    while last < buf_size
                        && {
                            i = get_uni_c(f);
                            i != -1i32
                        }
                        && i != '\n' as i32
                        && i != '\r' as i32
                    {
                        let fresh6 = last;
                        last = last + 1;
                        *buffer.offset(fresh6 as isize) = i
                    }
                }
                if i == -1i32 && *__errno_location() != 4i32 && last == first {
                    return 0i32;
                }
                /* We didn't get the whole line because our buffer was too small.  */
                if i != -1i32 && i != '\n' as i32 && i != '\r' as i32 {
                    buffer_overflow();
                }
            }
        }
    }
    /* If line ended with CR, remember to skip following LF. */
    if i == '\r' as i32 {
        (*f).skipNextLF = 1i32 as libc::c_short
    }
    *buffer.offset(last as isize) = ' ' as i32;
    if last >= max_buf_stack {
        max_buf_stack = last
    }
    /* Trim trailing space or EOL characters.  */
    while last > first
        && (*buffer.offset((last - 1i32) as isize) == ' ' as i32
            || *buffer.offset((last - 1i32) as isize) == '\r' as i32
            || *buffer.offset((last - 1i32) as isize) == '\n' as i32)
    {
        last -= 1
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn u_close(mut f: *mut UFILE) {
    if f.is_null() || (*f).handle.is_null() {
        /* NULL handle is stdin/terminal file. Shouldn't happen but meh. */
        return;
    }
    ttstub_input_close((*f).handle);
    if (*f).encodingMode as libc::c_int == 5i32 && !(*f).conversionData.is_null() {
        icu::ucnv_close((*f).conversionData as *mut icu::UConverter);
    }
    free(f as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn get_uni_c(mut f: *mut UFILE) -> libc::c_int {
    let mut current_block: u64;
    let mut rval: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if (*f).savedChar != -1i32 as libc::c_long {
        rval = (*f).savedChar as libc::c_int;
        (*f).savedChar = -1i32 as libc::c_long;
        return rval;
    }
    match (*f).encodingMode as libc::c_int {
        1 => {
            rval = ttstub_input_getc((*f).handle);
            c = rval;
            if rval != -1i32 {
                let mut extraBytes: u16 = bytesFromUTF8[rval as usize] as u16;
                match extraBytes as libc::c_int {
                    3 => {
                        /* note: code falls through cases! */
                        c = ttstub_input_getc((*f).handle);
                        if c < 0x80i32 || c >= 0xc0i32 {
                            current_block = 4870039662467851697;
                        } else {
                            rval <<= 6i32;
                            rval += c;
                            current_block = 11439173586221378108;
                        }
                    }
                    2 => {
                        current_block = 11439173586221378108;
                    }
                    1 => {
                        current_block = 223857376187897572;
                    }
                    5 | 4 => {
                        current_block = 8891683451182524030;
                    }
                    0 | _ => {
                        current_block = 15925075030174552612;
                    }
                }
                match current_block {
                    11439173586221378108 => {
                        c = ttstub_input_getc((*f).handle);
                        if c < 0x80i32 || c >= 0xc0i32 {
                            current_block = 4870039662467851697;
                        } else {
                            rval <<= 6i32;
                            rval += c;
                            current_block = 223857376187897572;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    223857376187897572 => {
                        c = ttstub_input_getc((*f).handle);
                        if c < 0x80i32 || c >= 0xc0i32 {
                            current_block = 4870039662467851697;
                        } else {
                            rval <<= 6i32;
                            rval += c;
                            current_block = 15925075030174552612;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    15925075030174552612 => {
                        rval = (rval as libc::c_uint)
                            .wrapping_sub(offsetsFromUTF8[extraBytes as usize])
                            as libc::c_int as libc::c_int;
                        if rval < 0i32 || rval > 0x10ffffi32 {
                            bad_utf8_warning();
                            return 0xfffdi32;
                        }
                        current_block = 317151059986244064;
                    }
                    4870039662467851697 => {
                        if c != -1i32 {
                            ttstub_input_ungetc((*f).handle, c);
                        }
                        current_block = 8891683451182524030;
                    }
                    _ => {}
                }
                match current_block {
                    317151059986244064 => {}
                    _ => {
                        bad_utf8_warning();
                        return 0xfffdi32;
                    }
                }
            }
        }
        2 => {
            rval = ttstub_input_getc((*f).handle);
            if rval != -1i32 {
                rval <<= 8i32;
                rval += ttstub_input_getc((*f).handle);
                if rval >= 0xd800i32 && rval <= 0xdbffi32 {
                    let mut lo: libc::c_int = ttstub_input_getc((*f).handle);
                    lo <<= 8i32;
                    lo += ttstub_input_getc((*f).handle);
                    if lo >= 0xdc00i32 && lo <= 0xdfffi32 {
                        rval = 0x10000i32 + (rval - 0xd800i32) * 0x400i32 + (lo - 0xdc00i32)
                    } else {
                        rval = 0xfffdi32;
                        (*f).savedChar = lo as libc::c_long
                    }
                } else if rval >= 0xdc00i32 && rval <= 0xdfffi32 {
                    rval = 0xfffdi32
                }
            }
        }
        3 => {
            rval = ttstub_input_getc((*f).handle);
            if rval != -1i32 {
                rval += ttstub_input_getc((*f).handle) << 8i32;
                if rval >= 0xd800i32 && rval <= 0xdbffi32 {
                    let mut lo_0: libc::c_int = ttstub_input_getc((*f).handle);
                    lo_0 += ttstub_input_getc((*f).handle) << 8i32;
                    if lo_0 >= 0xdc00i32 && lo_0 <= 0xdfffi32 {
                        rval = 0x10000i32 + (rval - 0xd800i32) * 0x400i32 + (lo_0 - 0xdc00i32)
                    } else {
                        rval = 0xfffdi32;
                        (*f).savedChar = lo_0 as libc::c_long
                    }
                } else if rval >= 0xdc00i32 && rval <= 0xdfffi32 {
                    rval = 0xfffdi32
                }
            }
        }
        4 => rval = ttstub_input_getc((*f).handle),
        _ => {
            _tt_abort(
                b"internal error; file input mode=%d\x00" as *const u8 as *const libc::c_char,
                (*f).encodingMode as libc::c_int,
            );
        }
    }
    return rval;
}
/* tectonic/xetex-io.h: XeTeX-specific low-level I/O routines
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
#[no_mangle]
pub unsafe extern "C" fn make_utf16_name() {
    let mut s: *mut u8 = name_of_file as *mut u8;
    let mut rval: u32 = 0;
    let mut t: *mut u16 = 0 as *mut u16;
    static mut name16len: libc::c_int = 0i32;
    if name16len <= name_length {
        free(name_of_file16 as *mut libc::c_void);
        name16len = name_length + 10i32;
        name_of_file16 = xcalloc(
            name16len as size_t,
            ::std::mem::size_of::<u16>() as u64,
        ) as *mut UTF16_code
    }
    t = name_of_file16;
    while s < (name_of_file as *mut u8).offset(name_length as isize) {
        let mut extraBytes: u16 = 0;
        let fresh7 = s;
        s = s.offset(1);
        rval = *fresh7 as u32;
        extraBytes = bytesFromUTF8[rval as usize] as u16;
        let mut current_block_23: u64;
        match extraBytes as libc::c_int {
            5 => {
                /* note: code falls through cases! */
                rval <<= 6i32;
                if *s != 0 {
                    let fresh8 = s;
                    s = s.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh8 as libc::c_uint) as u32
                }
                current_block_23 = 1933956893526356233;
            }
            4 => {
                current_block_23 = 1933956893526356233;
            }
            3 => {
                current_block_23 = 15901505722045918842;
            }
            2 => {
                current_block_23 = 5484884370842436748;
            }
            1 => {
                current_block_23 = 1843389027537967668;
            }
            0 | _ => {
                current_block_23 = 14648156034262866959;
            }
        }
        match current_block_23 {
            1933956893526356233 => {
                rval <<= 6i32;
                if *s != 0 {
                    let fresh9 = s;
                    s = s.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh9 as libc::c_uint) as u32
                }
                current_block_23 = 15901505722045918842;
            }
            _ => {}
        }
        match current_block_23 {
            15901505722045918842 => {
                rval <<= 6i32;
                if *s != 0 {
                    let fresh10 = s;
                    s = s.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh10 as libc::c_uint) as u32
                }
                current_block_23 = 5484884370842436748;
            }
            _ => {}
        }
        match current_block_23 {
            5484884370842436748 => {
                rval <<= 6i32;
                if *s != 0 {
                    let fresh11 = s;
                    s = s.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh11 as libc::c_uint) as u32
                }
                current_block_23 = 1843389027537967668;
            }
            _ => {}
        }
        match current_block_23 {
            1843389027537967668 => {
                rval <<= 6i32;
                if *s != 0 {
                    let fresh12 = s;
                    s = s.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh12 as libc::c_uint) as u32
                }
            }
            _ => {}
        }
        rval = (rval as libc::c_uint).wrapping_sub(offsetsFromUTF8[extraBytes as usize]) as u32;
        if rval > 0xffffi32 as libc::c_uint {
            rval = (rval as libc::c_uint).wrapping_sub(0x10000i32 as libc::c_uint) as u32;
            let fresh13 = t;
            t = t.offset(1);
            *fresh13 = (0xd800i32 as libc::c_uint)
                .wrapping_add(rval.wrapping_div(0x400i32 as libc::c_uint))
                as u16;
            let fresh14 = t;
            t = t.offset(1);
            *fresh14 = (0xdc00i32 as libc::c_uint)
                .wrapping_add(rval.wrapping_rem(0x400i32 as libc::c_uint))
                as u16
        } else {
            let fresh15 = t;
            t = t.offset(1);
            *fresh15 = rval as u16
        }
    }
    name_length16 = t.wrapping_offset_from(name_of_file16) as libc::c_long as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn open_or_close_in() {
    let mut c: u8 = 0;
    let mut n: u8 = 0;
    let mut k: int32_t = 0;
    c = cur_chr as u8;
    scan_four_bit_int();
    n = cur_val as u8;
    if read_open[n as usize] as libc::c_int != 2i32 {
        u_close(read_file[n as usize]);
        read_open[n as usize] = 2i32 as u8
    }
    if c as libc::c_int != 0i32 {
        scan_optional_equals();
        scan_file_name();
        pack_file_name(cur_name, cur_area, cur_ext);
        if u_open_in(
            &mut *read_file.as_mut_ptr().offset(n as isize),
            TTIF_TEX as libc::c_int,
            b"rb\x00" as *const u8 as *const libc::c_char,
            (*eqtb.offset(
                (1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 77i32) as isize,
            ))
            .b32
            .s1,
            (*eqtb.offset(
                (1i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 1i32
                    + 15000i32
                    + 12i32
                    + 9000i32
                    + 1i32
                    + 1i32
                    + 19i32
                    + 256i32
                    + 256i32
                    + 13i32
                    + 256i32
                    + 4i32
                    + 256i32
                    + 1i32
                    + 3i32 * 256i32
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + (0x10ffffi32 + 1i32)
                    + 78i32) as isize,
            ))
            .b32
            .s1,
        ) != 0
        {
            make_utf16_name();
            name_in_progress = 1i32 != 0;
            begin_name();
            stop_at_space = 0i32 != 0;
            k = 0i32;
            while k < name_length16
                && more_name(*name_of_file16.offset(k as isize)) as libc::c_int != 0
            {
                k += 1
            }
            stop_at_space = 1i32 != 0;
            end_name();
            name_in_progress = 0i32 != 0;
            read_open[n as usize] = 1i32 as u8
        }
    };
}
