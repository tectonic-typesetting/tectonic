pub type UChar = u16;
pub type UErrorCode = libc::c_int;

use crate::int32_t;

macro_rules! no_mangle_extern_fn {
    ($(
        $(#[$meta:meta])*
        pub fn $symbol:ident => $extern_symbol:ident($($argname:ident: $argtype:ty),*)
                                  -> $rettype:ty;
    )*) => {
        extern "C" {
            $(
                #[no_mangle]
                $(#[$meta])*
                fn $extern_symbol($($argname: $argtype),*) -> $rettype;
            )*
        }
    };
}

macro_rules! forward_stub_fn {
    ($(
        $(#[$meta:meta])*
        pub fn $symbol:ident => $extern_symbol:ident($($argname:ident: $argtype:ty),*)
                                  -> $rettype:ty;
    )*) => {
        $(
            #[allow(unused_variables)]
            $(#[$meta])*
            pub unsafe fn $symbol($($argname: $argtype),*) -> $rettype {
                $extern_symbol($($argname),*)
            }
        )*
    };
}
macro_rules! extern_and_forward_stub {
    ($(
        $(#[$meta:meta])*
        pub fn $symbol:ident => $extern_symbol:ident($($argname:ident: $argtype:ty),*)
                                  -> $rettype:ty;
    )*) => {
        no_mangle_extern_fn!($(
            $(#[$meta])*
            pub fn $symbol => $extern_symbol($($argname : $argtype),*)
                                    -> $rettype;
        )*);
        forward_stub_fn!($(
            $(#[$meta])*
            pub fn $symbol => $extern_symbol($($argname : $argtype),*)
                                    -> $rettype;
        )*);
    };
}
extern "C" {
    pub type UBreakIterator;
    pub type UConverter;
    pub type UBiDi;
}


pub type UConverterType = libc::c_int;
pub type UBiDiLevel = crate::uint8_t;

pub const UCNV_NUMBER_OF_SUPPORTED_CONVERTER_TYPES: UConverterType = 34;
pub const UCNV_COMPOUND_TEXT: UConverterType = 33;
pub const UCNV_IMAP_MAILBOX: UConverterType = 32;
pub const UCNV_CESU8: UConverterType = 31;
pub const UCNV_UTF32: UConverterType = 30;
pub const UCNV_UTF16: UConverterType = 29;
pub const UCNV_BOCU1: UConverterType = 28;
pub const UCNV_UTF7: UConverterType = 27;
pub const UCNV_US_ASCII: UConverterType = 26;
pub const UCNV_ISCII: UConverterType = 25;
pub const UCNV_SCSU: UConverterType = 24;
pub const UCNV_HZ: UConverterType = 23;
pub const UCNV_LMBCS_LAST: UConverterType = 22;
pub const UCNV_LMBCS_19: UConverterType = 22;
pub const UCNV_LMBCS_18: UConverterType = 21;
pub const UCNV_LMBCS_17: UConverterType = 20;
pub const UCNV_LMBCS_16: UConverterType = 19;
pub const UCNV_LMBCS_11: UConverterType = 18;
pub const UCNV_LMBCS_8: UConverterType = 17;
pub const UCNV_LMBCS_6: UConverterType = 16;
pub const UCNV_LMBCS_5: UConverterType = 15;
pub const UCNV_LMBCS_4: UConverterType = 14;
pub const UCNV_LMBCS_3: UConverterType = 13;
pub const UCNV_LMBCS_2: UConverterType = 12;
pub const UCNV_LMBCS_1: UConverterType = 11;
pub const UCNV_ISO_2022: UConverterType = 10;
pub const UCNV_EBCDIC_STATEFUL: UConverterType = 9;
pub const UCNV_UTF32_LittleEndian: UConverterType = 8;
pub const UCNV_UTF32_BigEndian: UConverterType = 7;
pub const UCNV_UTF16_LittleEndian: UConverterType = 6;
pub const UCNV_UTF16_BigEndian: UConverterType = 5;
pub const UCNV_UTF8: UConverterType = 4;
pub const UCNV_LATIN_1: UConverterType = 3;
pub const UCNV_MBCS: UConverterType = 2;
pub const UCNV_DBCS: UConverterType = 1;
pub const UCNV_SBCS: UConverterType = 0;
pub const UCNV_UNSUPPORTED_CONVERTER: UConverterType = -1;

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

pub type UBiDiDirection = libc::c_uint;
pub const UBIDI_RTL: UBiDiDirection = 1;
pub const UBIDI_NEUTRAL: UBiDiDirection = 3;
pub const UBIDI_MIXED: UBiDiDirection = 2;
pub const UBIDI_LTR: UBiDiDirection = 0;

pub type UBreakIteratorType = libc::c_uint;
pub const UBRK_COUNT: UBreakIteratorType = 5;
pub const UBRK_TITLE: UBreakIteratorType = 4;
pub const UBRK_SENTENCE: UBreakIteratorType = 3;
pub const UBRK_LINE: UBreakIteratorType = 2;
pub const UBRK_WORD: UBreakIteratorType = 1;
pub const UBRK_CHARACTER: UBreakIteratorType = 0;

extern_and_forward_stub! {
    pub fn ubidi_open => tt_ubidi_open() -> *mut UBiDi;
    pub fn ubidi_close => tt_ubidi_close(pBiDi: *mut UBiDi) -> ();
    pub fn ubidi_setPara => tt_ubidi_setPara(
        pBiDi: *mut UBiDi,
        text: *const UChar,
        length: int32_t,
        paraLevel: UBiDiLevel,
        embeddingLevels: *mut UBiDiLevel,
        pErrorCode: *mut UErrorCode
    ) -> ();
    pub fn ubidi_getDirection => tt_ubidi_getDirection(pBiDi: *const UBiDi) -> UBiDiDirection;
    pub fn ubidi_getVisualRun => tt_ubidi_getVisualRun(
        pBiDi: *mut UBiDi,
        runIndex: int32_t,
        pLogicalStart: *mut int32_t,
        pLength: *mut int32_t
    ) -> UBiDiDirection;
    pub fn ubidi_countRuns => tt_ubidi_countRuns(pBiDi: *mut UBiDi, pErrorCode: *mut UErrorCode) -> int32_t;
    pub fn ubrk_next => tt_ubrk_next(bi: *mut UBreakIterator) -> int32_t;
    pub fn ubrk_close => tt_ubrk_close(bi: *mut UBreakIterator) -> ();
    pub fn ubrk_open => tt_ubrk_open(
        type_0: UBreakIteratorType,
        locale: *const libc::c_char,
        text: *const UChar,
        textLength: int32_t,
        status: *mut UErrorCode
    ) -> *mut UBreakIterator;
    pub fn ubrk_setText => tt_ubrk_setText(
        bi: *mut UBreakIterator,
        text: *const UChar,
        textLength: int32_t,
        status: *mut UErrorCode
    ) -> ();
    pub fn ucnv_open => tt_ucnv_open(converterName: *const libc::c_char, err: *mut UErrorCode) -> *mut UConverter;
    #[no_mangle]
    pub fn ucnv_close => tt_ucnv_close(converter: *mut UConverter) -> ();
    #[no_mangle]
    pub fn ucnv_toAlgorithmic => tt_ucnv_toAlgorithmic(
        algorithmicType: UConverterType,
        cnv: *mut UConverter,
        target: *mut libc::c_char,
        targetCapacity: int32_t,
        source: *const libc::c_char,
        sourceLength: int32_t,
        pErrorCode: *mut UErrorCode
    ) -> int32_t;
}
