pub type XeTeXLayoutEngine = *mut XeTeXLayoutEngine_rec;
/// PlatformFontRef matches C++
#[cfg(not(target_os = "macos"))]
pub type PlatformFontRef = *mut FcPattern;
#[cfg(target_os = "macos")]
use crate::xetex_aatfont::cf_prelude::CTFontDescriptorRef;
#[cfg(target_os = "macos")]
pub type PlatformFontRef = CTFontDescriptorRef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphAssembly {
    pub count: u32,
    pub parts: *mut hb_ot_math_glyph_part_t,
}
pub type hb_tag_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hb_feature_t {
    pub tag: hb_tag_t,
    pub value: u32,
    pub start: u32,
    pub end: u32,
}
pub type hb_codepoint_t = u32;
pub type hb_position_t = i32;
pub type hb_ot_math_glyph_part_flags_t = u32;
pub const HB_OT_MATH_GLYPH_PART_FLAG_EXTENDER: hb_ot_math_glyph_part_flags_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hb_ot_math_glyph_part_t {
    pub glyph: hb_codepoint_t,
    pub start_connector_length: hb_position_t,
    pub end_connector_length: hb_position_t,
    pub full_advance: hb_position_t,
    pub flags: hb_ot_math_glyph_part_flags_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GlyphBBox {
    pub xMin: f32,
    pub yMin: f32,
    pub xMax: f32,
    pub yMax: f32,
}
extern "C" {
    pub type XeTeXFont_rec;
    pub type XeTeXLayoutEngine_rec;
    #[no_mangle]
    pub fn get_ot_math_constant(f: i32, n: i32) -> i32;
    #[no_mangle]
    pub fn set_cp_code(fontNum: i32, code: u32, side: i32, value: i32);
    #[no_mangle]
    pub fn get_cp_code(fontNum: i32, code: u32, side: i32) -> i32;
    #[no_mangle]
    pub fn isOpenTypeMathFont(engine: XeTeXLayoutEngine) -> bool;
    #[no_mangle]
    pub fn usingGraphite(engine: XeTeXLayoutEngine) -> bool;
    #[no_mangle]
    pub fn usingOpenType(engine: XeTeXLayoutEngine) -> bool;
    #[no_mangle]
    pub fn terminate_font_manager();
    #[no_mangle]
    pub fn destroy_font_manager();
    #[no_mangle]
    pub fn get_native_mathsy_param(f: i32, n: i32) -> i32;
    #[no_mangle]
    pub fn get_native_mathex_param(f: i32, n: i32) -> i32;
    #[no_mangle]
    pub fn get_ot_math_variant(f: i32, g: i32, v: i32, adv: *mut i32, horiz: i32) -> i32;
    #[no_mangle]
    pub fn get_ot_assembly_ptr(f: i32, g: i32, horiz: i32) -> *mut libc::c_void;
    #[no_mangle]
    pub fn free_ot_assembly(a: *mut GlyphAssembly);
    #[no_mangle]
    pub fn get_ot_math_ital_corr(f: i32, g: i32) -> i32;
    #[no_mangle]
    pub fn get_ot_math_accent_pos(f: i32, g: i32) -> i32;
    #[no_mangle]
    pub fn get_ot_math_kern(f: i32, g: i32, sf: i32, sg: i32, cmd: i32, shift: i32) -> i32;
    #[no_mangle]
    pub fn ot_part_count(a: *const GlyphAssembly) -> i32;
    #[no_mangle]
    pub fn ot_part_glyph(a: *const GlyphAssembly, i: i32) -> i32;
    #[no_mangle]
    pub fn ot_part_is_extender(a: *const GlyphAssembly, i: i32) -> bool;
    #[no_mangle]
    pub fn ot_part_start_connector(f: i32, a: *const GlyphAssembly, i: i32) -> i32;
    #[no_mangle]
    pub fn ot_part_end_connector(f: i32, a: *const GlyphAssembly, i: i32) -> i32;
    #[no_mangle]
    pub fn ot_part_full_advance(f: i32, a: *const GlyphAssembly, i: i32) -> i32;
    #[no_mangle]
    pub fn ot_min_connector_overlap(f: i32) -> i32;
    #[no_mangle]
    pub fn hb_tag_from_string(str: *const i8, len: i32) -> hb_tag_t;
    #[no_mangle]
    pub fn getCachedGlyphBBox(fontID: u16, glyphID: u16, bbox: *mut GlyphBBox) -> i32;
    #[no_mangle]
    pub fn cacheGlyphBBox(fontID: u16, glyphID: u16, bbox: *const GlyphBBox);
    #[no_mangle]
    pub fn maketexstring(s: *const i8) -> i32;
    #[no_mangle]
    pub fn getDefaultDirection(engine: XeTeXLayoutEngine) -> i32;
    #[no_mangle]
    pub fn createFont(fontRef: PlatformFontRef, pointSize: Fixed) -> XeTeXFont;
    #[no_mangle]
    pub fn getAscentAndDescent(engine: XeTeXLayoutEngine, ascent: *mut f32, descent: *mut f32);
    #[no_mangle]
    pub fn setFontLayoutDir(font: XeTeXFont, vertical: i32);
    #[no_mangle]
    pub fn layoutChars(
        engine: XeTeXLayoutEngine,
        chars: *mut u16,
        offset: i32,
        count: i32,
        max: i32,
        rightToLeft: bool,
    ) -> i32;
    #[no_mangle]
    pub fn getPointSize(engine: XeTeXLayoutEngine) -> f32;
    #[no_mangle]
    pub fn getGlyphPositions(engine: XeTeXLayoutEngine, positions: *mut FloatPoint);
    #[no_mangle]
    pub fn getGlyphAdvances(engine: XeTeXLayoutEngine, advances: *mut f32);
    #[no_mangle]
    pub fn getGlyphs(engine: XeTeXLayoutEngine, glyphs: *mut u32);
    #[no_mangle]
    pub fn findFontByName(name: *const i8, var: *mut i8, size: f64) -> PlatformFontRef;
    #[no_mangle]
    pub fn getReqEngine() -> i8;
    #[no_mangle]
    pub fn setReqEngine(reqEngine: i8);
    #[no_mangle]
    pub fn getFullName(fontRef: PlatformFontRef) -> *const i8;
    #[no_mangle]
    pub fn getFontFilename(engine: XeTeXLayoutEngine, index: *mut u32) -> *mut i8;
    #[no_mangle]
    pub fn getDesignSize(font: XeTeXFont) -> f64;
    #[no_mangle]
    pub fn deleteFont(font: XeTeXFont);
    #[no_mangle]
    pub fn getSlant(font: XeTeXFont) -> Fixed;
    #[no_mangle]
    pub fn getFontTablePtr(font: XeTeXFont, tableTag: u32) -> *mut libc::c_void;
    #[no_mangle]
    pub fn countScripts(font: XeTeXFont) -> u32;
    #[no_mangle]
    pub fn countLanguages(font: XeTeXFont, script: hb_tag_t) -> u32;
    #[no_mangle]
    pub fn countFeatures(font: XeTeXFont, script: hb_tag_t, language: hb_tag_t) -> u32;
    #[no_mangle]
    pub fn countGlyphs(font: XeTeXFont) -> u32;
    #[no_mangle]
    pub fn getIndScript(font: XeTeXFont, index: u32) -> hb_tag_t;
    #[no_mangle]
    pub fn getIndLanguage(font: XeTeXFont, script: hb_tag_t, index: u32) -> hb_tag_t;
    #[no_mangle]
    pub fn getIndFeature(
        font: XeTeXFont,
        script: hb_tag_t,
        language: hb_tag_t,
        index: u32,
    ) -> hb_tag_t;
    #[no_mangle]
    pub fn getGlyphWidth(font: XeTeXFont, gid: u32) -> f32;
    #[no_mangle]
    pub fn createFontFromFile(filename: *const i8, index: i32, pointSize: Fixed) -> XeTeXFont;
    #[no_mangle]
    pub fn getCapAndXHeight(engine: XeTeXLayoutEngine, capheight: *mut f32, xheight: *mut f32);
    #[no_mangle]
    pub fn getEmboldenFactor(engine: XeTeXLayoutEngine) -> f32;
    #[no_mangle]
    pub fn getSlantFactor(engine: XeTeXLayoutEngine) -> f32;
    #[no_mangle]
    pub fn getExtendFactor(engine: XeTeXLayoutEngine) -> f32;
    #[no_mangle]
    pub fn getFontRef(engine: XeTeXLayoutEngine) -> PlatformFontRef;
    #[no_mangle]
    pub fn getFont(engine: XeTeXLayoutEngine) -> XeTeXFont;
    #[no_mangle]
    pub fn deleteLayoutEngine(engine: XeTeXLayoutEngine);
    #[no_mangle]
    pub fn createLayoutEngine(
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
    pub fn findGraphiteFeature(
        engine: XeTeXLayoutEngine,
        s: *const i8,
        e: *const i8,
        f: *mut hb_tag_t,
        v: *mut i32,
    ) -> bool;
    #[no_mangle]
    pub fn findNextGraphiteBreak() -> i32;
    #[no_mangle]
    pub fn initGraphiteBreaking(engine: XeTeXLayoutEngine, txtPtr: *const u16, txtLen: i32)
        -> bool;
    #[no_mangle]
    pub fn getFontCharRange(engine: XeTeXLayoutEngine, reqFirst: i32) -> i32;
    #[no_mangle]
    pub fn getGlyphName(font: XeTeXFont, gid: u16, len: *mut i32) -> *const i8;
    #[no_mangle]
    pub fn mapGlyphToIndex(engine: XeTeXLayoutEngine, glyphName: *const i8) -> i32;
    #[no_mangle]
    pub fn mapCharToGlyph(engine: XeTeXLayoutEngine, charCode: u32) -> u32;
    #[no_mangle]
    pub fn getGlyphItalCorr(engine: XeTeXLayoutEngine, glyphID: u32) -> f32;
    #[no_mangle]
    pub fn getGlyphSidebearings(
        engine: XeTeXLayoutEngine,
        glyphID: u32,
        lsb: *mut f32,
        rsb: *mut f32,
    );
    #[no_mangle]
    pub fn getGlyphHeightDepth(
        engine: XeTeXLayoutEngine,
        glyphID: u32,
        height: *mut f32,
        depth: *mut f32,
    );
    #[no_mangle]
    pub fn getGlyphWidthFromEngine(engine: XeTeXLayoutEngine, glyphID: u32) -> f32;
    #[no_mangle]
    pub fn getGlyphBounds(engine: XeTeXLayoutEngine, glyphID: u32, bbox: *mut GlyphBBox);
    #[no_mangle]
    pub fn getRgbValue(engine: XeTeXLayoutEngine) -> u32;
    #[no_mangle]
    pub fn countGraphiteFeatures(engine: XeTeXLayoutEngine) -> u32;
    #[no_mangle]
    pub fn getGraphiteFeatureCode(engine: XeTeXLayoutEngine, index: u32) -> u32;
    #[no_mangle]
    pub fn countGraphiteFeatureSettings(engine: XeTeXLayoutEngine, feature: u32) -> u32;
    #[no_mangle]
    pub fn getGraphiteFeatureSettingCode(
        engine: XeTeXLayoutEngine,
        feature: u32,
        index: u32,
    ) -> u32;
    #[no_mangle]
    pub fn getGraphiteFeatureDefaultSetting(engine: XeTeXLayoutEngine, feature: u32) -> u32;
    #[no_mangle]
    pub fn getGraphiteFeatureLabel(engine: XeTeXLayoutEngine, feature: u32) -> *mut i8;
    #[no_mangle]
    pub fn getGraphiteFeatureSettingLabel(
        engine: XeTeXLayoutEngine,
        feature: u32,
        setting: u32,
    ) -> *mut i8;
    #[no_mangle]
    pub fn findGraphiteFeatureNamed(
        engine: XeTeXLayoutEngine,
        name: *const i8,
        namelength: i32,
    ) -> i64;
    #[no_mangle]
    pub fn findGraphiteFeatureSettingNamed(
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
    pub fn gr_label_destroy(label: *mut libc::c_void);
}
pub type XeTeXFont = *mut XeTeXFont_rec;
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
pub type scaled_t = i32;
pub type SInt32 = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FloatPoint {
    pub x: f32,
    pub y: f32,
}

#[cfg(not(target_os = "macos"))]
extern "C" {
    pub type _FcPattern;
}
#[cfg(not(target_os = "macos"))]
pub type FcPattern = _FcPattern;
