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

use std::{fmt, ptr};

#[cfg(target_endian = "big")]
mod data {
    #[derive(Copy, Clone, Default)]
    #[repr(C)]
    pub struct B32x2 {
        s1: i32,
        s0: i32,
    }

    #[derive(Copy, Clone, Default)]
    #[repr(C)]
    pub struct B16x4 {
        s3: u16,
        s2: u16,
        s1: u16,
        s0: u16,
    }
}

#[cfg(not(target_endian = "big"))]
mod data {
    #[derive(Copy, Clone, Default)]
    #[repr(C)]
    pub struct B32x2 {
        pub(crate) s0: i32,
        pub(crate) s1: i32,
    }

    #[derive(Copy, Clone, Default)]
    #[repr(C)]
    pub struct B16x4 {
        pub(crate) s0: u16,
        pub(crate) s1: u16,
        pub(crate) s2: u16,
        pub(crate) s3: u16,
    }
}

pub use data::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub union MemoryWord {
    pub(crate) b32: B32x2,
    pub(crate) b16: B16x4,
    pub(crate) gr: f64,
    pub(crate) ptr: *mut (),
}

impl MemoryWord {
    pub fn i32_0(&self) -> i32 {
        unsafe { self.b32.s0 }
    }

    pub fn i32_1(&self) -> i32 {
        unsafe { self.b32.s1 }
    }

    pub fn u16_0(&self) -> u16 {
        unsafe { self.b16.s0 }
    }

    pub fn u16_1(&self) -> u16 {
        unsafe { self.b16.s1 }
    }

    pub fn u16_2(&self) -> u16 {
        unsafe { self.b16.s2 }
    }

    pub fn u16_3(&self) -> u16 {
        unsafe { self.b16.s3 }
    }

    pub fn f64(&self) -> f64 {
        unsafe { self.gr }
    }

    pub fn ptr(&self) -> *mut () {
        unsafe { self.ptr }
    }
}

impl Default for MemoryWord {
    fn default() -> Self {
        MemoryWord {
            ptr: ptr::null_mut(),
        }
    }
}

/* ## THE ORIGINAL SITUATION (archived for posterity)
 *
 * In XeTeX, a "quarterword" is 16 bits. Who knows why. A "halfword" is,
 * sensibly, 32 bits. A "memory word" is a full word: either four quarters or
 * two halves: i.e., 64 bits. The memory word union also has options for
 * doubles (called `gr`), `integer` which is an int32_t (called `cint`), and a
 * pointer (`ptr`).
 *
 * Original struct definition, LITTLE ENDIAN (condensed):
 *
 *   typedef union {
 *       struct { int32_t LH, RH; } v;
 *       struct { short B1, B0; } u;
 *   } two_halves;
 *
 *   typedef struct {
 *       struct { uint16_t B3, B2, B1, B0; } u;
 *   } four_quarters;
 *
 *   typedef union {
 *       two_halves hh;
 *
 *       struct {
 *           int32_t junk;
 *           int32_t CINT;
 *       } u;
 *
 *       struct {
 *           four_quarters QQQQ;
 *       } v;
 *   } memory_word;
 *
 *   #  define cint u.CINT
 *   #  define qqqq v.QQQQ
 *
 * Original memory layout, LITTLE ENDIAN:
 *
 *   bytes:    --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   cint:                             [lsb...............msb]
 *   hh.u:     [l..B1...m] [l..B0...m]
 *   hh.v:     [lsb......LH.......msb] [lsb......RH.......msb]
 *   quarters: [l..B3...m] [l..B2...m] [l..B1...m] [l..B0...m]
 *
 * Original struct definition, BIG ENDIAN (condensed):
 *
 *   typedef union {
 *       struct { int32_t RH, LH; } v;
 *       struct {
 *           int32_t junk;
 *           short B0, B1;
 *       } u;
 *   } two_halves;
 *
 *   typedef struct {
 *       struct { uint16_t B0, B1, B2, B3; } u;
 *   } four_quarters;
 *
 *   typedef union {
 *       two_halves hh;
 *       four_quarters qqqq;
 *   } memory_word;
 *
 * Original memory layout, BIG ENDIAN:
 *
 *   bytes:    --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   cint:     [msb...............lsb]
 *   hh.u:                             [m..B0...l] [m..B1...l]
 *   hh.v:     [msb......RH.......lsb] [msb......LH.......lsb]
 *   quarters: [m..B0...l] [m..B1...l] [m..B2...l] [m...B3..l]
 *
 * Several things to note that apply to both endiannesses:
 *
 *   1. The different B0 and B1 instances do not line up.
 *   2. `cint` is isomorphic to `hh.v.RH`
 *   3. `hh.u.B0` is isomorphic to `qqqq.u.B2`
 *   4. `hh.u.B1` is isomorphic to `qqqq.u.B3`.
 *   5. The `four_quarters` field `u` serves no discernable purpose.
 *
 * CONVERTING TO THE NEW SYSTEM
 *
 * - `w.cint` => `w.b32.s1`
 * - `w.qqqq.u.B<n>` => `w.b16.s{{3 - <n>}}` !!!!!!!!!!!
 * - similar for `<quarterword_variable>.u.B<n>` => `<quarterword_variable>.s{{3 - <n>}}` !!!
 * - `w.hh.u.B0` => `w.b16.s1`
 * - `w.hh.u.B1` => `w.b16.s0`
 * - `w.hh.v.RH` => `w.b32.s1`
 * - `w.hh.v.LH` => `w.b32.s0`
 * - `four_quarters` => `b16x4`
 * - `two_halves` => `b32x2`
 *
 */

/* the size of various node types */
pub const GLUE_SPEC_SIZE: i32 = 4;

/* Types of nodes that can occur in general lists. */
pub const HLIST_NODE: i32 = 0; /* = 0x0 */
pub const VLIST_NODE: i32 = 1; /* = 0x1 */
pub const RULE_NODE: i32 = 2; /* = 0x2 */
pub const INS_NODE: i32 = 3; /* = 0x3 */
pub const MARK_NODE: i32 = 4; /* = 0x4 */
pub const ADJUST_NODE: i32 = 5; /* = 0x5 */
pub const LIGATURE_NODE: i32 = 6; /* = 0x6 */
pub const DISC_NODE: i32 = 7; /* = 0x7 */
pub const WHATSIT_NODE: i32 = 8; /* = 0x8 */
pub const MATH_NODE: i32 = 9; /* = 0x9 */
pub const GLUE_NODE: i32 = 10; /* = 0xa */
pub const KERN_NODE: i32 = 11; /* = 0xb */
pub const PENALTY_NODE: i32 = 12; /* = 0xc */
pub const UNSET_NODE: i32 = 13; /* = 0xd */
pub const STYLE_NODE: i32 = 14; /* = 0xe */
pub const CHOICE_NODE: i32 = 15; /* = 0xf */
pub const MARGIN_KERN_NODE: i32 = 40; /* = 0x28 */

/* Additional types of "noads" that can occur in math lists. */
pub const TT_LEFT_RIGHT_MIDDLE_MODE: i32 = 1; /* = 0x1 */
pub const ORD_NOAD: i32 = 16; /* = 0x10 */
pub const OP_NOAD: i32 = 17; /* = 0x11 */
pub const BIN_NOAD: i32 = 18; /* = 0x12 */
pub const REL_NOAD: i32 = 19; /* = 0x13 */
pub const OPEN_NOAD: i32 = 20; /* = 0x14 */
pub const CLOSE_NOAD: i32 = 21; /* = 0x15 */
pub const PUNCT_NOAD: i32 = 22; /* = 0x16 */
pub const INNER_NOAD: i32 = 23; /* = 0x17 */
pub const RADICAL_NOAD: i32 = 24; /* = 0x18 */
pub const FRACTION_NOAD: i32 = 25; /* = 0x19 */
pub const UNDER_NOAD: i32 = 26; /* = 0x1a */
pub const OVER_NOAD: i32 = 27; /* = 0x1b */
pub const ACCENT_NOAD: i32 = 28; /* = 0x1c */
pub const VCENTER_NOAD: i32 = 29; /* = 0x1d */
pub const LEFT_NOAD: i32 = 30; /* = 0x1e */
pub const RIGHT_NOAD: i32 = 31; /* = 0x1f */
pub const MIDDLE_NOAD: i32 = 1;

/* Subtypes for glue nodes. */
pub const NORMAL: i32 = 0;
pub const MU_GLUE: i32 = 99;
pub const A_LEADERS: i32 = 100;
pub const C_LEADERS: i32 = 101;
pub const X_LEADERS: i32 = 102;

/* Subtypes for math style nodes. */

pub const DISPLAY_STYLE: i32 = 0;
pub const TEXT_STYLE: i32 = 2;
pub const SCRIPT_STYLE: i32 = 4;
pub const SCRIPT_SCRIPT_STYLE: i32 = 6;

/* Subtypes for math OP noads. */

pub const LIMITS: i32 = 1; /* = 0x1 */
pub const NO_LIMITS: i32 = 2; /* = 0x2 */

/* Subtypes for whatsit nodes. */
pub const OPEN_NODE: i32 = 0;
pub const WRITE_NODE: i32 = 1;
pub const CLOSE_NODE: i32 = 2;
pub const SPECIAL_NODE: i32 = 3;
pub const LANGUAGE_NODE: i32 = 4;
pub const PDF_SAVE_POS_NODE: i32 = 21;
pub const NATIVE_WORD_NODE: i32 = 40;
pub const NATIVE_WORD_NODE_AT: i32 = 41;
pub const GLYPH_NODE: i32 = 42;
pub const PIC_NODE: i32 = 43;
pub const PDF_NODE: i32 = 44;

pub trait Node {
    /// Type of this node. `None` indicates types which are valid for all nodes.
    fn ty() -> u16;
    /// Subtype of this node. `None` indicates types which are valid for all subtypes.
    fn subty() -> Option<u16>;

    unsafe fn from_ptr(ptr: *const MemoryWord) -> *const Self;
    unsafe fn from_ptr_mut(ptr: *mut MemoryWord) -> *mut Self;
}

#[repr(C)]
pub struct NodeBase(MemoryWord);

impl NodeBase {
    pub(super) fn from_ptr(ptr: *const MemoryWord) -> *const NodeBase {
        ptr.cast()
    }

    pub fn ty(&self) -> u16 {
        unsafe { self.0.b16.s1 }
    }

    pub fn subty(&self) -> u16 {
        unsafe { self.0.b16.s0 }
    }

    pub fn next(&self) -> usize {
        unsafe { self.0.b32.s1 as usize }
    }
}

impl fmt::Debug for NodeBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeBase")
            .field("ty", &self.ty())
            .field("subty", &self.subty())
            .finish()
    }
}

#[repr(C)]
pub struct NativeWordNode {
    // p
    node: NodeBase,
    // p + 1..p + 3
    _priv: [MemoryWord; 3],
    // p + 4 - most data is here
    data: B16x4,
    // p + 5 - glyph_info pointer
    glyph_info: *mut (),
    // p+6.. - text in this node
    text: [u16],
}

impl Node for NativeWordNode {
    fn ty() -> u16 {
        WHATSIT_NODE as u16
    }

    fn subty() -> Option<u16> {
        Some(NATIVE_WORD_NODE as u16)
    }

    unsafe fn from_ptr(ptr: *const MemoryWord) -> *const Self {
        let this = ptr::slice_from_raw_parts(ptr, 0) as *const Self;
        let len = (*this).len();
        ptr::slice_from_raw_parts(ptr, len) as *const Self
    }

    unsafe fn from_ptr_mut(ptr: *mut MemoryWord) -> *mut Self {
        let this = ptr::slice_from_raw_parts_mut(ptr, 0) as *mut Self;
        let len = (*this).len();
        ptr::slice_from_raw_parts_mut(ptr, len) as *mut Self
    }
}

impl NativeWordNode {
    pub fn base(&self) -> &NodeBase {
        &self.node
    }

    pub fn len(&self) -> usize {
        self.data.s1 as usize
    }

    pub fn text(&self) -> &[u16] {
        &self.text
    }
}

pub const EQTB_SIZE: usize = 0x886f92;

pub const ACTIVE_BASE: usize = 0x1;
pub const SINGLE_BASE: usize = 0x110001;
pub const PRIM_EQTB_BASE: usize = 0x223aa6;
pub const GLUE_BASE: usize = 0x226604;
pub const SKIP_BASE: usize = 0x226617;
pub const MU_SKIP_BASE: usize = 0x226717;
pub const LOCAL_BASE: usize = 0x226817;
pub const TOKS_BASE: usize = 0x226824;
pub const ETEX_PEN_BASE: usize = 0x226924;
pub const MATH_FONT_BASE: usize = 0x226a29;
pub const CAT_CODE_BASE: usize = 0x226d29;
pub const LC_CODE_BASE: usize = 0x336d29;
pub const UC_CODE_BASE: usize = 0x446d29;
pub const SF_CODE_BASE: usize = 0x556d29;
pub const MATH_CODE_BASE: usize = 0x666d29;
pub const INT_BASE: usize = 0x776d29;
pub const COUNT_BASE: usize = 0x776d7c;
pub const DEL_CODE_BASE: usize = 0x776e7c;
pub const DIMEN_BASE: usize = 0x886e7c;
pub const SCALED_BASE: usize = 0x886e93;

/* Subcommand codes for the ABOVE command. */

pub const ABOVE_CODE: i32 = 0;
pub const OVER_CODE: i32 = 1;
pub const ATOP_CODE: i32 = 2;
pub const TT_ABOVE_WITH_DELIMS: i32 = 3;
pub const TT_OVER_WITH_DELIMS: i32 = 4;
pub const TT_ATOP_WITH_DELIMS: i32 = 5;

/* Subcommand codes for box-related commands. */

pub const BOX_CODE: i32 = 0;
pub const COPY_CODE: i32 = 1;
pub const LAST_BOX_CODE: i32 = 2;
pub const VSPLIT_CODE: i32 = 3;
pub const VTOP_CODE: i32 = 4;
pub const TT_VBOX_CODE: i32 = 5;
pub const TT_HBOX_CODE: i32 = 108;

/* Subcommand codes for the CONVERT command. */

pub const NUMBER_CODE: i32 = 0;
pub const ROMAN_NUMERAL_CODE: i32 = 1;
pub const STRING_CODE: i32 = 2;
pub const MEANING_CODE: i32 = 3;
pub const FONT_NAME_CODE: i32 = 4;
pub const ETEX_CONVERT_BASE: i32 = 5;
pub const ETEX_REVISION_CODE: i32 = 5;
pub const ETEX_CONVERT_CODES: i32 = 6;
pub const EXPANDED_CODE: i32 = 6;
pub const PDFTEX_FIRST_EXPAND_CODE: i32 = 7;
pub const LEFT_MARGIN_KERN_CODE: i32 = 16;
pub const RIGHT_MARGIN_KERN_CODE: i32 = 17;
pub const PDF_STRCMP_CODE: i32 = 18;
pub const PDF_CREATION_DATE_CODE: i32 = 22;
pub const PDF_FILE_MOD_DATE_CODE: i32 = 23;
pub const PDF_FILE_SIZE_CODE: i32 = 24;
pub const PDF_MDFIVE_SUM_CODE: i32 = 25;
pub const PDF_FILE_DUMP_CODE: i32 = 26;
pub const UNIFORM_DEVIATE_CODE: i32 = 29;
pub const NORMAL_DEVIATE_CODE: i32 = 30;
pub const PDFTEX_CONVERT_CODES: i32 = 33;
pub const XETEX_FIRST_EXPAND_CODE: i32 = 33;
pub const XETEX_REVISION_CODE: i32 = 33;
pub const XETEX_VARIATION_NAME_CODE: i32 = 34;
pub const XETEX_FEATURE_NAME_CODE: i32 = 35;
pub const XETEX_SELECTOR_NAME_CODE: i32 = 36;
pub const XETEX_GLYPH_NAME_CODE: i32 = 37;
pub const XETEX_UCHAR_CODE: i32 = 38;
pub const XETEX_UCHARCAT_CODE: i32 = 39;
pub const JOB_NAME_CODE: i32 = 40;
pub const XETEX_CONVERT_CODES: i32 = 40;

/* Subcommand codes for the EXTENSION command. */

pub const IMMEDIATE_CODE: i32 = 4;
pub const SET_LANGUAGE_CODE: i32 = 5;
pub const RESET_TIMER_CODE: i32 = 31;
pub const SET_RANDOM_SEED_CODE: i32 = 33;
pub const PIC_FILE_CODE: i32 = 41;
pub const PDF_FILE_CODE: i32 = 42;
pub const GLYPH_CODE: i32 = 43;
pub const XETEX_INPUT_ENCODING_EXTENSION_CODE: i32 = 44;
pub const XETEX_DEFAULT_ENCODING_EXTENSION_CODE: i32 = 45;
pub const XETEX_LINEBREAK_LOCALE_EXTENSION_CODE: i32 = 46;

/* Subcommand codes for the FI_OR_ELSE command. */

pub const FI_CODE: i32 = 2;
pub const ELSE_CODE: i32 = 3;
pub const OR_CODE: i32 = 4;

/* Subcommand codes for the IF_TEST command. */

pub const IF_CHAR_CODE: i32 = 0;
pub const IF_CAT_CODE: i32 = 1;
pub const IF_INT_CODE: i32 = 2;
pub const IF_DIM_CODE: i32 = 3;
pub const IF_ODD_CODE: i32 = 4;
pub const IF_VMODE_CODE: i32 = 5;
pub const IF_HMODE_CODE: i32 = 6;
pub const IF_MMODE_CODE: i32 = 7;
pub const IF_INNER_CODE: i32 = 8;
pub const IF_VOID_CODE: i32 = 9;
pub const IF_HBOX_CODE: i32 = 10;
pub const IF_VBOX_CODE: i32 = 11;
pub const IFX_CODE: i32 = 12;
pub const IF_EOF_CODE: i32 = 13;
pub const IF_TRUE_CODE: i32 = 14;
pub const IF_FALSE_CODE: i32 = 15;
pub const IF_CASE_CODE: i32 = 16;
pub const IF_DEF_CODE: i32 = 17;
pub const IF_CS_CODE: i32 = 18;
pub const IF_FONT_CHAR_CODE: i32 = 19;
pub const IF_IN_CSNAME_CODE: i32 = 20;
pub const IF_PRIMITIVE_CODE: i32 = 21;

/* Subcommand codes for the LAST_ITEM command. */

pub const INT_VAL: i32 = 0;
pub const DIMEN_VAL: i32 = 1;
pub const GLUE_VAL: i32 = 2;
pub const LAST_NODE_TYPE_CODE: i32 = 3;
pub const INPUT_LINE_NO_CODE: i32 = 4;
pub const BADNESS_CODE: i32 = 5;
pub const PDFTEX_FIRST_RINT_CODE: i32 = 6;
pub const PDF_LAST_X_POS_CODE: i32 = 12;
pub const PDF_LAST_Y_POS_CODE: i32 = 13;
pub const ELAPSED_TIME_CODE: i32 = 16;
pub const PDF_SHELL_ESCAPE_CODE: i32 = 17;
pub const RANDOM_SEED_CODE: i32 = 18;
pub const ETEX_INT: i32 = 19;
pub const ETEX_VERSION_CODE: i32 = 19;
pub const CURRENT_GROUP_LEVEL_CODE: i32 = 20;
pub const CURRENT_GROUP_TYPE_CODE: i32 = 21;
pub const CURRENT_IF_LEVEL_CODE: i32 = 22;
pub const CURRENT_IF_TYPE_CODE: i32 = 23;
pub const CURRENT_IF_BRANCH_CODE: i32 = 24;
pub const GLUE_STRETCH_ORDER_CODE: i32 = 25;
pub const GLUE_SHRINK_ORDER_CODE: i32 = 26;
pub const XETEX_INT: i32 = 27;
pub const XETEX_VERSION_CODE: i32 = 27;
pub const XETEX_COUNT_GLYPHS_CODE: i32 = 28;
pub const XETEX_COUNT_VARIATIONS_CODE: i32 = 29;
pub const XETEX_VARIATION_CODE: i32 = 30;
pub const XETEX_FIND_VARIATION_BY_NAME_CODE: i32 = 31;
pub const XETEX_VARIATION_MIN_CODE: i32 = 32;
pub const XETEX_VARIATION_MAX_CODE: i32 = 33;
pub const XETEX_VARIATION_DEFAULT_CODE: i32 = 34;
pub const XETEX_COUNT_FEATURES_CODE: i32 = 35;
pub const XETEX_FEATURE_CODE_CODE: i32 = 36;
pub const XETEX_FIND_FEATURE_BY_NAME_CODE: i32 = 37;
pub const XETEX_IS_EXCLUSIVE_FEATURE_CODE: i32 = 38;
pub const XETEX_COUNT_SELECTORS_CODE: i32 = 39;
pub const XETEX_SELECTOR_CODE_CODE: i32 = 40;
pub const XETEX_FIND_SELECTOR_BY_NAME_CODE: i32 = 41;
pub const XETEX_IS_DEFAULT_SELECTOR_CODE: i32 = 42;
pub const XETEX_OT_COUNT_SCRIPTS_CODE: i32 = 43;
pub const XETEX_OT_COUNT_LANGUAGES_CODE: i32 = 44;
pub const XETEX_OT_COUNT_FEATURES_CODE: i32 = 45;
pub const XETEX_OT_SCRIPT_CODE: i32 = 46;
pub const XETEX_OT_LANGUAGE_CODE: i32 = 47;
pub const XETEX_OT_FEATURE_CODE: i32 = 48;
pub const XETEX_MAP_CHAR_TO_GLYPH_CODE: i32 = 49;
pub const XETEX_GLYPH_INDEX_CODE: i32 = 50;
pub const XETEX_FONT_TYPE_CODE: i32 = 51;
pub const XETEX_FIRST_CHAR_CODE: i32 = 52;
pub const XETEX_LAST_CHAR_CODE: i32 = 53;
pub const XETEX_PDF_PAGE_COUNT_CODE: i32 = 54;
pub const XETEX_LAST_ITEM_CODES: i32 = 54;
pub const XETEX_DIM: i32 = 55;
pub const XETEX_GLYPH_BOUNDS_CODE: i32 = 55;
pub const XETEX_LAST_DIM_CODES: i32 = 55;
pub const ETEX_DIM: i32 = 56;
pub const FONT_CHAR_WD_CODE: i32 = 56;
pub const FONT_CHAR_HT_CODE: i32 = 57;
pub const FONT_CHAR_DP_CODE: i32 = 58;
pub const FONT_CHAR_IC_CODE: i32 = 59;
pub const PAR_SHAPE_LENGTH_CODE: i32 = 60;
pub const PAR_SHAPE_INDENT_CODE: i32 = 61;
pub const PAR_SHAPE_DIMEN_CODE: i32 = 62;
pub const GLUE_STRETCH_CODE: i32 = 63;
pub const GLUE_SHRINK_CODE: i32 = 64;
pub const ETEX_GLUE: i32 = 65;
pub const MU_TO_GLUE_CODE: i32 = 65;
pub const ETEX_MU: i32 = 66;
pub const GLUE_TO_MU_CODE: i32 = 66;
pub const ETEX_EXPR: i32 = 67;
pub const TT_ETEX_NUM_EXPR_CODE: i32 = 67;
pub const TT_ETEX_DIM_EXPR_CODE: i32 = 68;
pub const TT_ETEX_GLUE_EXPR_CODE: i32 = 69;
pub const TT_ETEX_MU_EXPR_CODE: i32 = 70;

/* Subcommand codes for the SET_BOX_DIMEN command. */

pub const WIDTH_OFFSET: i32 = 1;
pub const DEPTH_OFFSET: i32 = 2;
pub const HEIGHT_OFFSET: i32 = 3;

/* Subcommand codes for the SHORTHAND_DEF command. */

pub const CHAR_DEF_CODE: i32 = 0;
pub const MATH_CHAR_DEF_CODE: i32 = 1;
pub const COUNT_DEF_CODE: i32 = 2;
pub const DIMEN_DEF_CODE: i32 = 3;
pub const SKIP_DEF_CODE: i32 = 4;
pub const MU_SKIP_DEF_CODE: i32 = 5;
pub const TOKS_DEF_CODE: i32 = 6;
pub const CHAR_SUB_DEF_CODE: i32 = 7;
pub const XETEX_MATH_CHAR_NUM_DEF_CODE: i32 = 8;
pub const XETEX_MATH_CHAR_DEF_CODE: i32 = 9;

/* Subcommand codes for skip-related command. */

pub const FIL_CODE: i32 = 0;
pub const FILL_CODE: i32 = 1;
pub const SS_CODE: i32 = 2;
pub const FIL_NEG_CODE: i32 = 3;
pub const SKIP_CODE: i32 = 4;
pub const MSKIP_CODE: i32 = 5;

/* Subcommand codes for the TAB_MARK and CAR_RET commands. */

pub const SPAN_CODE: i32 = 1114113;
pub const CR_CODE: i32 = 1114114;
pub const CR_CR_CODE: i32 = 1114115;

/* Subcommand codes for the TOP_BOT_MARK command. */

pub const TOP_MARK_CODE: i32 = 0;
pub const FIRST_MARK_CODE: i32 = 1;
pub const BOT_MARK_CODE: i32 = 2;
pub const SPLIT_FIRST_MARK_CODE: i32 = 3;
pub const SPLIT_BOT_MARK_CODE: i32 = 4;
pub const TT_TOP_MARKS_CODE: i32 = 5;
pub const TT_FIRST_MARKS_CODE: i32 = 6;
pub const TT_BOT_MARKS_CODE: i32 = 7;
pub const TT_SPLIT_FIRST_MARKS_CODE: i32 = 8;
pub const TT_SPLIT_BOT_MARKS_CODE: i32 = 9;

/* Subcommand codes for the XRAY command. */

pub const SHOW_CODE: i32 = 0;
pub const SHOW_BOX_CODE: i32 = 1;
pub const SHOW_THE_CODE: i32 = 2;
pub const SHOW_LISTS: i32 = 3;
pub const SHOW_GROUPS: i32 = 4;
pub const SHOW_TOKENS: i32 = 5;
pub const SHOW_IFS: i32 = 6;

pub const INT_PARS: usize = 83;

#[derive(Copy, Clone, PartialEq)]
pub enum CatCode {
    Escape = 0,
    CarRet = 5,
    Ignore = 9,
    Spacer = 10,
    Letter = 11,
    OtherChar = 12,
    Comment = 14,
    InvalidChar = 15,
    Data = 122,
}

impl TryFrom<i32> for CatCode {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => CatCode::Escape,
            5 => CatCode::CarRet,
            9 => CatCode::Ignore,
            10 => CatCode::Spacer,
            11 => CatCode::Letter,
            12 => CatCode::OtherChar,
            14 => CatCode::Comment,
            15 => CatCode::InvalidChar,
            122 => CatCode::Data,
            _ => return Err(value),
        })
    }
}

/// Integer parameters
pub enum IntPar {
    PreTolerance = 0,
    Tolerance,
    LinePenalty,
    HyphenPenalty,
    ExHyphenPenalty,
    ClubPenalty,
    WidowPenalty,
    DisplayWidowPenalty,
    BrokenPenalty,
    BinOpPenalty,
    RelPenalty,
    PreDisplayPenalty,
    PostDisplayPenalty,
    InterLinePenalty,
    DoubleHyphenDemerits,
    FinalHyphenDemerits,
    AdjDemerits,
    Mag,
    DelimiterFactor,
    Looseness,
    Time,
    Day,
    Month,
    Year,
    ShowBoxBreadth,
    ShowBoxDepth,
    HBadness,
    VBadness,
    Pausing,
    TracingOnline,
    TracingMacros,
    TracingStats,
    TracingParagraphs,
    TracingPages,
    TracingOutput,
    TracingLostChars,
    TracingCommands,
    TracingRestores,
    UcHyph,
    OutputPenalty,
    MaxDeadCycles,
    HangAfter,
    FloatingPenalty,
    GlobalDefs,
    CurFam,
    EscapeChar,
    DefaultHyphenChar,
    DefaultSkewChar,
    EndLineChar,
    NewLineChar,
    Language,
    LeftHyphenMin,
    RightHyphenMin,
    HoldingInserts,
    ErrorContextLines,
    TracingStackLevels,
    TracingAssigns,
    TracingGroups,
    TracingIfs,
    TracingScanTokens,
    TracingNesting,
    PreDisplayDirection,
    LastLineFit,
    SavingVDiscards,
    SavingHyphCodes,
    SuppressFontNotFoundError,
    XetexLinebreakLocale,
    XetexLinebreakPenalty,
    XetexProtrudeChars,
    Texxet,
    XetexDashBreak,
    XetexUpwards,
    XetexUseGlyphMetrics,
    XetexInterCharTokens,
    XetexInputNormalization,
    XetexDefaultInputMode,
    XetexDefaultInputEncoding,
    XetexTracingFonts,
    XetexInterwordSpaceShaping,
    XetexGenerateActualText,
    XetexHyphenatableLength,
    Synctex,
    PdfOutput,
}

impl TryFrom<i32> for IntPar {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => IntPar::PreTolerance,
            1 => IntPar::Tolerance,
            2 => IntPar::LinePenalty,
            3 => IntPar::HyphenPenalty,
            4 => IntPar::ExHyphenPenalty,
            5 => IntPar::ClubPenalty,
            6 => IntPar::WidowPenalty,
            7 => IntPar::DisplayWidowPenalty,
            8 => IntPar::BrokenPenalty,
            9 => IntPar::BinOpPenalty,
            10 => IntPar::RelPenalty,
            11 => IntPar::PreDisplayPenalty,
            12 => IntPar::PostDisplayPenalty,
            13 => IntPar::InterLinePenalty,
            14 => IntPar::DoubleHyphenDemerits,
            15 => IntPar::FinalHyphenDemerits,
            16 => IntPar::AdjDemerits,
            17 => IntPar::Mag,
            18 => IntPar::DelimiterFactor,
            19 => IntPar::Looseness,
            20 => IntPar::Time,
            21 => IntPar::Day,
            22 => IntPar::Month,
            23 => IntPar::Year,
            24 => IntPar::ShowBoxBreadth,
            25 => IntPar::ShowBoxDepth,
            26 => IntPar::HBadness,
            27 => IntPar::VBadness,
            28 => IntPar::Pausing,
            29 => IntPar::TracingOnline,
            30 => IntPar::TracingMacros,
            31 => IntPar::TracingStats,
            32 => IntPar::TracingParagraphs,
            33 => IntPar::TracingPages,
            34 => IntPar::TracingOutput,
            35 => IntPar::TracingLostChars,
            36 => IntPar::TracingCommands,
            37 => IntPar::TracingRestores,
            38 => IntPar::UcHyph,
            39 => IntPar::OutputPenalty,
            40 => IntPar::MaxDeadCycles,
            41 => IntPar::HangAfter,
            42 => IntPar::FloatingPenalty,
            43 => IntPar::GlobalDefs,
            44 => IntPar::CurFam,
            45 => IntPar::EscapeChar,
            46 => IntPar::DefaultHyphenChar,
            47 => IntPar::DefaultSkewChar,
            48 => IntPar::EndLineChar,
            49 => IntPar::NewLineChar,
            50 => IntPar::Language,
            51 => IntPar::LeftHyphenMin,
            52 => IntPar::RightHyphenMin,
            53 => IntPar::HoldingInserts,
            54 => IntPar::ErrorContextLines,
            55 => IntPar::TracingStackLevels,
            56 => IntPar::TracingAssigns,
            57 => IntPar::TracingGroups,
            58 => IntPar::TracingIfs,
            59 => IntPar::TracingScanTokens,
            60 => IntPar::TracingNesting,
            61 => IntPar::PreDisplayDirection,
            62 => IntPar::LastLineFit,
            63 => IntPar::SavingVDiscards,
            64 => IntPar::SavingHyphCodes,
            65 => IntPar::SuppressFontNotFoundError,
            66 => IntPar::XetexLinebreakLocale,
            67 => IntPar::XetexLinebreakPenalty,
            68 => IntPar::XetexProtrudeChars,
            69 => IntPar::Texxet,
            70 => IntPar::XetexDashBreak,
            71 => IntPar::XetexUpwards,
            72 => IntPar::XetexUseGlyphMetrics,
            73 => IntPar::XetexInterCharTokens,
            74 => IntPar::XetexInputNormalization,
            75 => IntPar::XetexDefaultInputMode,
            76 => IntPar::XetexDefaultInputEncoding,
            77 => IntPar::XetexTracingFonts,
            78 => IntPar::XetexInterwordSpaceShaping,
            79 => IntPar::XetexGenerateActualText,
            80 => IntPar::XetexHyphenatableLength,
            81 => IntPar::Synctex,
            82 => IntPar::PdfOutput,
            _ => return Err(value),
        })
    }
}

pub enum GluePar {
    LineSkip = 0,
    BaselineSkip,
    ParSkip,
    AboveDisplaySkip,
    BelowDisplaySkip,
    AboveDisplayShortSkip,
    BelowDisplayShortSkip,
    LeftSkip,
    RightSkip,
    TopSkip,
    SplitTopSkip,
    TabSkip,
    SpaceSkip,
    XSpaceSkip,
    ParFillSkip,
    XetexLinebreakSkip,
    ThinMuSkip,
    MedMuSkip,
    ThickMuSkip,
}

impl TryFrom<i32> for GluePar {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => GluePar::LineSkip,
            1 => GluePar::BaselineSkip,
            2 => GluePar::ParSkip,
            3 => GluePar::AboveDisplaySkip,
            4 => GluePar::BelowDisplaySkip,
            5 => GluePar::AboveDisplayShortSkip,
            6 => GluePar::BelowDisplayShortSkip,
            7 => GluePar::LeftSkip,
            8 => GluePar::RightSkip,
            9 => GluePar::TopSkip,
            10 => GluePar::SplitTopSkip,
            11 => GluePar::TabSkip,
            12 => GluePar::SpaceSkip,
            13 => GluePar::XSpaceSkip,
            14 => GluePar::ParFillSkip,
            15 => GluePar::XetexLinebreakSkip,
            16 => GluePar::ThinMuSkip,
            17 => GluePar::MedMuSkip,
            18 => GluePar::ThickMuSkip,
            _ => return Err(value),
        })
    }
}

pub enum Local {
    ParShape = 0,
    OutputRoutine,
    EveryPar,
    EveryMath,
    EveryDisplay,
    EveryHbox,
    EveryVbox,
    EveryJob,
    EveryCr,
    ErrHelp,
    EveryEof,
    XetexInterCharToks,
    TectonicCodaTokens,
}

impl TryFrom<i32> for Local {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Local::ParShape,
            1 => Local::OutputRoutine,
            2 => Local::EveryPar,
            3 => Local::EveryMath,
            4 => Local::EveryDisplay,
            5 => Local::EveryHbox,
            6 => Local::EveryVbox,
            7 => Local::EveryJob,
            8 => Local::EveryCr,
            9 => Local::ErrHelp,
            10 => Local::EveryEof,
            11 => Local::XetexInterCharToks,
            12 => Local::TectonicCodaTokens,
            _ => return Err(value),
        })
    }
}

pub enum DimenPar {
    ParIndent = 0,
    MathSurround,
    LineSkipLimit,
    HSize,
    VSize,
    MaxDepth,
    SplitMaxDepth,
    BoxMaxDepth,
    HFuzz,
    VFuzz,
    DelimiterShortfall,
    NullDelimiterSpace,
    ScriptSpace,
    PreDisplaySpace,
    DisplayWidth,
    DisplayIndent,
    OverfullRule,
    HangIndent,
    HOffset,
    VOffset,
    EmergencyStretch,
    PdfPageWidth,
    PdfPageHeight,
}

impl TryFrom<i32> for DimenPar {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => DimenPar::ParIndent,
            1 => DimenPar::MathSurround,
            2 => DimenPar::LineSkipLimit,
            3 => DimenPar::HSize,
            4 => DimenPar::VSize,
            5 => DimenPar::MaxDepth,
            6 => DimenPar::SplitMaxDepth,
            7 => DimenPar::BoxMaxDepth,
            8 => DimenPar::HFuzz,
            9 => DimenPar::VFuzz,
            10 => DimenPar::DelimiterShortfall,
            11 => DimenPar::NullDelimiterSpace,
            12 => DimenPar::ScriptSpace,
            13 => DimenPar::PreDisplaySpace,
            14 => DimenPar::DisplayWidth,
            15 => DimenPar::DisplayIndent,
            16 => DimenPar::OverfullRule,
            17 => DimenPar::HangIndent,
            18 => DimenPar::HOffset,
            19 => DimenPar::VOffset,
            20 => DimenPar::EmergencyStretch,
            21 => DimenPar::PdfPageWidth,
            22 => DimenPar::PdfPageHeight,
            _ => return Err(value),
        })
    }
}

pub enum EtexPenaltiesPar {
    InterLinePenalties = 0,
    ClubPenalties,
    WidowPenalties,
    DisplayWidowPenalties,
}

impl TryFrom<i32> for EtexPenaltiesPar {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => EtexPenaltiesPar::InterLinePenalties,
            1 => EtexPenaltiesPar::ClubPenalties,
            2 => EtexPenaltiesPar::WidowPenalties,
            3 => EtexPenaltiesPar::DisplayWidowPenalties,
            _ => return Err(value),
        })
    }
}
