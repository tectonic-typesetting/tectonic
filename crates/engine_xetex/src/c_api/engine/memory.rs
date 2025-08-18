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

#[cfg(target_endian = "big")]
mod data {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct B32x2 {
        s1: i32,
        s0: i32,
    }

    #[derive(Copy, Clone)]
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct B32x2 {
        pub(crate) s0: i32,
        pub(crate) s1: i32,
    }

    #[derive(Copy, Clone)]
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

pub const EQTB_SIZE: usize = 0x886f92;

pub const ACTIVE_BASE: usize = 0x1;
pub const SINGLE_BASE: usize = 0x110001;
pub const PRIM_EQTB_BASE: usize = 0x223aa6;
pub const CAT_CODE_BASE: usize = 0x226d29;
pub const INT_BASE: usize = 0x776d29;
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
