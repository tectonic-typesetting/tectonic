use super::font::XeTeXFontBase;
use crate::c_api::manager::getReqEngine;
use crate::c_api::{FloatPoint, GlyphBBox, XeTeXFont, XeTeXLayoutEngine};
use std::borrow::Cow;
use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::ops::{Deref, DerefMut};
use std::{ptr, slice};
use tectonic_bridge_graphite2 as gr;
use tectonic_bridge_harfbuzz as hb;
use tectonic_bridge_icu::{UChar32, UBIDI_DEFAULT_LTR, UBIDI_DEFAULT_RTL};

pub enum MaybeBorrow<'a, T> {
    Owned(Box<T>),
    Borrowed(&'a mut T),
}

impl<T> Deref for MaybeBorrow<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeBorrow::Owned(val) => val,
            MaybeBorrow::Borrowed(val) => val,
        }
    }
}

impl<T> DerefMut for MaybeBorrow<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MaybeBorrow::Owned(val) => val,
            MaybeBorrow::Borrowed(val) => val,
        }
    }
}

#[repr(C)]
pub struct XeTeXLayoutEngineBase {
    font: MaybeBorrow<'static, XeTeXFontBase>,
    script: hb::Tag,
    language: hb::Language,
    features: Box<[hb::Feature]>,
    /// the requested shapers
    shaper_list: Vec<*const libc::c_char>,
    /// the actually used shaper
    shaper: Option<CString>,
    rgb_value: u32,
    extend: f32,
    slant: f32,
    embolden: f32,
    hb_buffer: hb::OwnBuffer,
}

impl XeTeXLayoutEngineBase {
    pub fn new(
        font: MaybeBorrow<'static, XeTeXFontBase>,
        script: hb::Tag,
        language: Option<Cow<'static, CStr>>,
        features: Box<[hb::Feature]>,
        shaper_list: Vec<*const libc::c_char>,
        rgb_value: u32,
        extend: f32,
        slant: f32,
        embolden: f32,
    ) -> XeTeXLayoutEngineBase {
        XeTeXLayoutEngineBase {
            font,
            script,
            // For Graphite fonts treat the language as BCP 47 tag, for OpenType we
            // treat it as a OT language tag for backward compatibility with pre-0.9999
            // XeTeX.
            language: if getReqEngine() as u8 == b'G' {
                language
                    .map(|lang| hb::Language::from_cstr(&lang))
                    .unwrap_or_default()
            } else {
                language
                    .map(|lang| hb::Tag::from_cstr(&lang).to_language())
                    .unwrap_or_default()
            },
            features,
            shaper_list,
            shaper: None,
            rgb_value,
            extend,
            slant,
            embolden,
            hb_buffer: hb::OwnBuffer::new(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn createLayoutEngine(
        font: XeTeXFont,
        script: hb::Tag,
        language: *mut libc::c_char,
        features: *mut hb::Feature,
        n_features: libc::c_int,
        shapers: *mut *const libc::c_char,
        rgb_value: u32,
        extend: f32,
        slant: f32,
        embolden: f32,
    ) -> XeTeXLayoutEngine {
        let font = Box::from_raw(font);

        let language = if !language.is_null() {
            Some(Cow::Owned(CString::from_raw(language)))
        } else {
            None
        };
        let features = if !features.is_null() {
            Box::from_raw(ptr::slice_from_raw_parts_mut(features, n_features as usize))
        } else {
            Box::new([])
        };
        let shaper_list = if !shapers.is_null() {
            let mut len = 0;
            while !(*shapers.add(len)).is_null() {
                len += 1;
            }
            len += 1;
            Vec::from_raw_parts(shapers, len, len)
        } else {
            Vec::new()
        };

        let this = Box::new(XeTeXLayoutEngineBase::new(
            MaybeBorrow::Owned(font),
            script,
            language,
            features,
            shaper_list,
            rgb_value,
            extend,
            slant,
            embolden,
        ));

        Box::into_raw(this)
    }

    #[no_mangle]
    pub unsafe extern "C" fn createLayoutEngineBorrowed(
        font: XeTeXFont,
        script: hb::Tag,
        language: *mut libc::c_char,
        features: *mut hb::Feature,
        n_features: libc::c_int,
        shapers: *mut *const libc::c_char,
        rgb_value: u32,
        extend: f32,
        slant: f32,
        embolden: f32,
    ) -> XeTeXLayoutEngine {
        let font = &mut *font;

        let language = if !language.is_null() {
            Some(Cow::Borrowed(CStr::from_ptr(language)))
        } else {
            None
        };
        let features: Box<[_]> = if !features.is_null() {
            let len = n_features as usize;
            // Clone the slice - we don't own it and it may be realloced at any time.
            Box::from(slice::from_raw_parts(features, len))
        } else {
            Box::new([])
        };
        let shaper_list = if !shapers.is_null() {
            let mut len = 0;
            while !(*shapers.add(len)).is_null() {
                len += 1;
            }
            len += 1;
            Vec::from_raw_parts(shapers, len, len)
        } else {
            Vec::new()
        };

        let this = Box::new(XeTeXLayoutEngineBase::new(
            MaybeBorrow::Borrowed(font),
            script,
            language,
            features,
            shaper_list,
            rgb_value,
            extend,
            slant,
            embolden,
        ));

        Box::into_raw(this)
    }

    #[no_mangle]
    pub unsafe extern "C" fn deleteLayoutEngine(this: XeTeXLayoutEngine) {
        let _ = Box::from_raw(this);
    }

    #[no_mangle]
    pub unsafe extern "C" fn getFont(engine: XeTeXLayoutEngine) -> XeTeXFont {
        (*engine).font_mut()
    }

    #[no_mangle]
    pub unsafe extern "C" fn getExtendFactor(engine: XeTeXLayoutEngine) -> f32 {
        (*engine).extend
    }

    #[no_mangle]
    pub unsafe extern "C" fn getSlantFactor(engine: XeTeXLayoutEngine) -> f32 {
        (*engine).slant
    }

    #[no_mangle]
    pub unsafe extern "C" fn getEmboldenFactor(engine: XeTeXLayoutEngine) -> f32 {
        (*engine).embolden
    }

    #[no_mangle]
    pub unsafe extern "C" fn getPointSize(engine: XeTeXLayoutEngine) -> f32 {
        (*engine).font().point_size()
    }

    #[no_mangle]
    pub unsafe extern "C" fn getAscentAndDescent(
        engine: XeTeXLayoutEngine,
        ascent: *mut f32,
        descent: *mut f32,
    ) {
        *ascent = (*engine).font().ascent();
        *descent = (*engine).font().descent();
    }

    #[no_mangle]
    pub unsafe extern "C" fn getCapAndXHeight(
        engine: XeTeXLayoutEngine,
        capheight: *mut f32,
        xheight: *mut f32,
    ) {
        *capheight = (*engine).font().cap_height();
        *xheight = (*engine).font().x_height();
    }

    #[no_mangle]
    pub unsafe extern "C" fn getDefaultDirection(engine: XeTeXLayoutEngine) -> libc::c_int {
        let script = (*engine).hb_buffer.get_script();
        if script.get_horizontal_direction() == hb::Direction::Rtl {
            UBIDI_DEFAULT_RTL as libc::c_int
        } else {
            UBIDI_DEFAULT_LTR as libc::c_int
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn getRgbValue(engine: XeTeXLayoutEngine) -> u32 {
        (*engine).rgb_value
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphBounds(
        engine: XeTeXLayoutEngine,
        glyph_id: u32,
        bbox: *mut GlyphBBox,
    ) {
        *bbox = (*engine).font_mut().get_glyph_bounds(glyph_id as u16);
        if (*engine).extend != 0.0 {
            (*bbox).x_min *= (*engine).extend;
            (*bbox).x_max *= (*engine).extend;
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphWidthFromEngine(
        engine: XeTeXLayoutEngine,
        glyph_id: u32,
    ) -> f32 {
        (*engine).extend * (*engine).font().get_glyph_width(glyph_id)
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphHeightDepth(
        engine: XeTeXLayoutEngine,
        glyph_id: u32,
        height: *mut f32,
        depth: *mut f32,
    ) {
        (*engine).font_mut().get_glyph_height_depth(
            glyph_id as u16,
            height.as_mut(),
            depth.as_mut(),
        );
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphSidebearings(
        engine: XeTeXLayoutEngine,
        glyph_id: u32,
        lsb: *mut f32,
        rsb: *mut f32,
    ) {
        (*engine)
            .font_mut()
            .get_glyph_sidebearings(glyph_id as u16, lsb.as_mut(), rsb.as_mut());
        if (*engine).extend != 0.0 {
            *lsb *= (*engine).extend;
            *rsb *= (*engine).extend;
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphItalCorr(engine: XeTeXLayoutEngine, glyph_id: u32) -> f32 {
        (*engine).extend * (*engine).font_mut().get_glyph_ital_corr(glyph_id as u16)
    }

    #[no_mangle]
    pub unsafe extern "C" fn mapCharToGlyph(engine: XeTeXLayoutEngine, char_code: u32) -> u32 {
        (*engine).font().map_char_to_glyph(char_code as UChar32) as u32
    }

    #[no_mangle]
    pub unsafe extern "C" fn getFontCharRange(
        engine: XeTeXLayoutEngine,
        req_first: libc::c_int,
    ) -> libc::c_int {
        if req_first != 0 {
            (*engine).font().get_first_char_code() as libc::c_int
        } else {
            (*engine).font().get_last_char_code() as libc::c_int
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn mapGlyphToIndex(
        engine: XeTeXLayoutEngine,
        glyph_name: *const libc::c_char,
    ) -> libc::c_int {
        (*engine)
            .font()
            .map_glyph_to_index(CStr::from_ptr(glyph_name)) as libc::c_int
    }

    #[no_mangle]
    pub unsafe extern "C" fn usingGraphite(engine: XeTeXLayoutEngine) -> bool {
        match &(*engine).shaper {
            Some(shaper) => shaper.to_bytes() == b"graphite2",
            None => false,
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn usingOpenType(engine: XeTeXLayoutEngine) -> bool {
        match &(*engine).shaper {
            Some(shaper) => shaper.to_bytes() == b"ot",
            None => false,
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn isOpenTypeMathFont(engine: XeTeXLayoutEngine) -> bool {
        (*engine).font().get_hb_font().get_face().has_ot_math_data()
    }

    #[no_mangle]
    pub unsafe extern "C" fn ttxl_get_hb_font(engine: XeTeXLayoutEngine) -> *mut hb::Font {
        ptr::from_ref((*engine).font().get_hb_font()).cast_mut()
    }

    #[no_mangle]
    pub unsafe extern "C" fn layoutChars(
        engine: XeTeXLayoutEngine,
        chars: *mut u16,
        offset: i32,
        count: i32,
        max: i32,
        rtl: bool,
    ) -> libc::c_int {
        let chars = slice::from_raw_parts(chars, max as usize);
        let engine = &mut *engine;

        let hb_font = engine.font.get_hb_font();
        let hb_face = hb_font.get_face();

        let direction = if engine.font.layout_dir_vertical() {
            hb::Direction::Ttb
        } else if rtl {
            hb::Direction::Rtl
        } else {
            hb::Direction::Ltr
        };

        let script = engine.script.to_script();
        engine.hb_buffer.reset();
        // TODO: figure out cfg for harfbuzz versions below 2.5
        // if hb_version_atleast(2, 5, 0) == 0 {
        //     #[derive(Copy, Clone)]
        //     struct SendSync(*mut hb_unicode_funcs_t);
        //
        //     unsafe impl Send for SendSync {}
        //     unsafe impl Sync for SendSync {}
        //
        //     unsafe extern "C" fn _decompose_compat(
        //         _: *mut hb_unicode_funcs_t,
        //         _: hb_codepoint_t,
        //         _: *mut hb_codepoint_t,
        //         _: *mut libc::c_void,
        //     ) -> libc::c_uint {
        //         0
        //     }
        //
        //     unsafe extern "C" fn _get_unicode_funcs() -> *mut hb_unicode_funcs_t {
        //         static UFUNCS: OnceLock<SendSync> = OnceLock::new();
        //         let ufuncs = *UFUNCS
        //             .get_or_init(|| SendSync(hb_unicode_funcs_create(hb_icu_get_unicode_funcs())));
        //
        //         hb_unicode_funcs_set_decompose_compatibility_func(
        //             ufuncs.0,
        //             _decompose_compat,
        //             ptr::null_mut(),
        //             None,
        //         );
        //
        //         ufuncs.0
        //     }
        //
        //     static HB_UNICODE_FUNCS: OnceLock<SendSync> = OnceLock::new();
        //     let funcs = *HB_UNICODE_FUNCS.get_or_init(|| SendSync(_get_unicode_funcs()));
        //     hb_buffer_set_unicode_funcs(engine.hb_buffer, funcs.0);
        // }

        engine
            .hb_buffer
            .add_utf16(chars, offset as usize, count as usize);
        engine.hb_buffer.set_direction(direction);
        engine.hb_buffer.set_script(script);
        engine.hb_buffer.set_language(engine.language);

        engine.hb_buffer.guess_segment_properties();
        let segment_props = engine.hb_buffer.get_segment_properties();

        if engine.shaper_list.is_empty() {
            // HarfBuzz gives graphite2 shaper a priority, so that for hybrid
            // Graphite/OpenType fonts, Graphite will be used. However, pre-0.9999
            // XeTeX preferred OpenType over Graphite, so we are doing the same
            // here for sake of backward compatibility. Since "ot" shaper never
            // fails, we set the shaper list to just include it.
            engine.shaper_list = vec![c!("ot"), ptr::null()];
        }

        let mut shape_plan = hb::OwnShapePlan::new_cached(
            hb_face,
            &segment_props,
            &engine.features,
            Some(&engine.shaper_list),
        );
        let res = shape_plan.execute(hb_font, &mut engine.hb_buffer, &engine.features);

        engine.shaper = None;

        if res {
            engine.shaper = Some(shape_plan.get_shaper().to_owned());
            engine
                .hb_buffer
                .set_content_type(hb::BufferContentType::Glyphs);
        } else {
            // all selected shapers failed, retrying with default
            // we don't use _cached here as the cached plain will always fail.
            shape_plan = hb::OwnShapePlan::new(hb_face, &segment_props, &engine.features, None);
            let res = shape_plan.execute(hb_font, &mut engine.hb_buffer, &engine.features);

            if res {
                engine.shaper = Some(shape_plan.get_shaper().to_owned());
                engine
                    .hb_buffer
                    .set_content_type(hb::BufferContentType::Glyphs);
            } else {
                panic!("all shapers failed");
            }
        }

        let glyph_count = engine.hb_buffer.len();

        #[cfg(feature = "debug")]
        {
            use std::ffi::CStr;

            let mut buf = [0u8; 1024];
            let mut consumed = 0;
            println!("shaper: {}", CStr::from_ptr(engine.shaper));

            let flags = HB_BUFFER_SERIALIZE_FLAGS_DEFAULT;
            let format = HB_BUFFER_SERIALIZE_FORMAT_JSON;

            hb_buffer_serialize_glyphs(
                engine.hb_buffer,
                0,
                glyph_count,
                &mut buf,
                1024,
                &mut consumed,
                hb_font,
                format,
                flags,
            );
            if consumed != 0 {
                println!("buffer glyphs: {}", CStr::from_ptr(&buf));
            }
        }

        glyph_count as libc::c_int
    }

    fn font(&self) -> &XeTeXFontBase {
        &self.font
    }

    fn font_mut(&mut self) -> &mut XeTeXFontBase {
        &mut self.font
    }
}

#[no_mangle]
pub unsafe extern "C" fn getFontFilename(
    engine: XeTeXLayoutEngine,
    index: *mut u32,
) -> *const libc::c_char {
    (*engine)
        .font()
        .get_filename(&mut *index)
        .to_owned()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn freeFontFilename(filename: *const libc::c_char) {
    let _ = CString::from_raw(filename.cast_mut());
}

#[no_mangle]
pub unsafe extern "C" fn getGlyphs(engine: XeTeXLayoutEngine, glyphs: *mut u32) {
    let hb_glyphs = (*engine).hb_buffer.get_glyph_info();

    for (idx, glyph) in hb_glyphs.iter().enumerate() {
        *glyphs.add(idx) = glyph.codepoint;
    }
}

#[no_mangle]
pub unsafe extern "C" fn getGlyphAdvances(engine: XeTeXLayoutEngine, advances: *mut f32) {
    let engine = &*engine;
    let hb_positions = engine.hb_buffer.get_glyph_position();

    for (i, pos) in hb_positions.iter().enumerate() {
        let advance = if engine.font().layout_dir_vertical() {
            pos.y_advance
        } else {
            pos.x_advance
        };

        *advances.add(i) = engine.font().units_to_points(advance as f64) as f32;
    }
}

#[no_mangle]
pub unsafe extern "C" fn getGlyphPositions(engine: XeTeXLayoutEngine, positions: *mut FloatPoint) {
    let engine = &mut *engine;
    let hb_positions = engine.hb_buffer.get_glyph_position();

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let font = engine.font();

    if font.layout_dir_vertical() {
        for (i, pos) in hb_positions.iter().enumerate() {
            (*positions.add(i)).x = -font.units_to_points((x + pos.y_offset as f32) as f64) as f32;
            (*positions.add(i)).y = font.units_to_points((y - pos.x_offset as f32) as f64) as f32;
            x += pos.y_advance as f32;
            y += pos.x_advance as f32;
        }

        (*positions.add(hb_positions.len())).x = -font.units_to_points(x as f64) as f32;
        (*positions.add(hb_positions.len())).y = font.units_to_points(y as f64) as f32;
    } else {
        for (i, pos) in hb_positions.iter().enumerate() {
            (*positions.add(i)).x = font.units_to_points((x + pos.x_offset as f32) as f64) as f32;
            (*positions.add(i)).y = -font.units_to_points((y + pos.y_offset as f32) as f64) as f32; /* negative is upwards */
            x += pos.x_advance as f32;
            y += pos.y_advance as f32;
        }
        (*positions.add(hb_positions.len())).x = font.units_to_points(x as f64) as f32;
        (*positions.add(hb_positions.len())).y = -font.units_to_points(y as f64) as f32;
    }

    if engine.extend != 1.0 || engine.slant != 0.0 {
        for i in 0..=hb_positions.len() {
            let pos = &mut *positions.add(i);
            pos.x = pos.x * engine.extend - pos.y * engine.slant;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn countGraphiteFeatures(engine: XeTeXLayoutEngine) -> u32 {
    let hb_face = (*engine).font().get_hb_font().get_face();
    match hb_face.gr_face() {
        Some(face) => face.num_feature_refs() as u32,
        None => 0,
    }
}

fn get_graphite_feature_code(engine: &XeTeXLayoutEngineBase, index: u32) -> Option<u32> {
    let id = engine
        .font()
        .get_hb_font()
        .get_face()
        .gr_face()?
        .feature_ref(index as usize)?
        .id();
    Some(id)
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureCode(engine: XeTeXLayoutEngine, index: u32) -> u32 {
    get_graphite_feature_code(&*engine, index).unwrap_or(0)
}

fn count_graphite_feature_settings(engine: &XeTeXLayoutEngineBase, feature_id: u32) -> Option<u32> {
    let out = engine
        .font()
        .get_hb_font()
        .get_face()
        .gr_face()?
        .find_feature_ref(feature_id)?
        .num_values() as u32;
    Some(out)
}

#[no_mangle]
pub unsafe extern "C" fn countGraphiteFeatureSettings(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
) -> u32 {
    count_graphite_feature_settings(&*engine, feature_id).unwrap_or(0)
}

fn get_graphite_feature_setting_code(
    engine: &XeTeXLayoutEngineBase,
    feature_id: u32,
    index: u32,
) -> Option<u32> {
    let out = engine
        .font()
        .get_hb_font()
        .get_face()
        .gr_face()?
        .find_feature_ref(feature_id)?
        .value(index as usize) as u32;
    Some(out)
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureSettingCode(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
    index: u32,
) -> u32 {
    get_graphite_feature_setting_code(&*engine, feature_id, index).unwrap_or(0)
}

fn get_graphite_feature_default_setting(
    engine: &XeTeXLayoutEngineBase,
    feature_id: u32,
) -> Option<u32> {
    let face = engine.font().get_hb_font().get_face().gr_face()?;
    let feat = face.find_feature_ref(feature_id)?;
    let lang = engine
        .language
        .to_string()
        .map(hb::Tag::from_cstr)
        .map(hb::Tag::to_raw)
        .unwrap_or(0);
    let feature_values = face.feature_val_for_lang(lang);
    let out = feat.feat_value(&feature_values) as u32;
    Some(out)
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureDefaultSetting(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
) -> u32 {
    get_graphite_feature_default_setting(&*engine, feature_id).unwrap_or(0)
}

fn get_graphite_feature_label(
    engine: &XeTeXLayoutEngineBase,
    feature_id: u32,
) -> Option<gr::Label> {
    let face = engine.font().get_hb_font().get_face().gr_face()?;
    let feature = face.find_feature_ref(feature_id)?;
    let lang_id = 0x409;
    let label = feature.label(lang_id)?;
    Some(label)
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureLabel(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
) -> *const libc::c_char {
    match get_graphite_feature_label(&*engine, feature_id) {
        Some(label) => label.into_raw().cast(),
        None => ptr::null_mut(),
    }
}

fn get_graphite_feature_setting_label(
    engine: &XeTeXLayoutEngineBase,
    feature_id: u32,
    setting_id: u32,
) -> Option<gr::Label> {
    let face = engine.font().get_hb_font().get_face().gr_face()?;

    let feature = face.find_feature_ref(feature_id)?;
    for i in 0..feature.num_values() {
        if setting_id == feature.value(i) as u32 {
            let lang_id = 0x409;
            return feature.value_label(i, lang_id);
        }
    }

    None
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureSettingLabel(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
    setting_id: u32,
) -> *const libc::c_char {
    match get_graphite_feature_setting_label(&*engine, feature_id, setting_id) {
        Some(label) => label.into_raw().cast(),
        None => ptr::null(),
    }
}

fn find_graphite_feature(
    engine: &XeTeXLayoutEngineBase,
    str: &[u8],
    tag: &mut hb::Tag,
    v: &mut libc::c_int,
) -> bool {
    *tag = hb::Tag::new(0);
    *v = 0;

    let mut idx = 0;
    while str[idx] == b' ' || str[idx] == b'\t' {
        idx += 1;
    }
    while str.get(idx).is_some_and(|c| *c != b'=') {
        idx += 1;
    }

    match find_graphite_feature_named(engine, &str[..idx]) {
        Some(val) => *tag = hb::Tag::new(val),
        None => return false,
    }

    idx += 1;
    while idx < str.len() && (str[idx] == b' ' || str[idx] == b'\t') {
        idx += 1;
    }

    if idx >= str.len() {
        return false;
    }

    *v = find_graphite_feature_setting_named(engine, tag.to_raw(), &str[idx..])
        .map(|i| i as libc::c_int)
        .unwrap_or(-1);

    *v != -1
}

#[no_mangle]
pub unsafe extern "C" fn findGraphiteFeature(
    engine: XeTeXLayoutEngine,
    s: *const libc::c_char,
    e: *const libc::c_char,
    f: *mut hb::Tag,
    v: *mut libc::c_int,
) -> bool {
    let len = e.byte_offset_from(s).unsigned_abs();
    let str = slice::from_raw_parts(s.cast(), len);
    find_graphite_feature(&*engine, str, &mut *f, &mut *v)
}

pub fn find_graphite_feature_named(engine: &XeTeXLayoutEngineBase, name: &[u8]) -> Option<u32> {
    let gr_face = engine.font().get_hb_font().get_face().gr_face()?;

    let tag = hb::Tag::from_str(std::str::from_utf8(name).unwrap()).to_raw();

    for i in 0..gr_face.num_feature_refs() {
        let feature = gr_face.feature_ref(i)?;
        let lang_id = 0x409;
        let label = feature.label(lang_id)?;

        if &label.as_bytes()[..name.len()] == name {
            return Some(feature.id());
        } else if feature.id() == tag {
            return Some(feature.id());
        }
    }

    None
}

#[no_mangle]
pub unsafe extern "C" fn findGraphiteFeatureNamed(
    engine: XeTeXLayoutEngine,
    name: *const libc::c_char,
    namelength: libc::c_int,
) -> libc::c_long {
    let name = if !name.is_null() && namelength > 0 {
        slice::from_raw_parts(name.cast::<u8>(), namelength as usize)
    } else {
        return -1;
    };
    find_graphite_feature_named(&*engine, name)
        .map(|i| i as libc::c_long)
        .unwrap_or(-1)
}

fn find_graphite_feature_setting_named(
    engine: &XeTeXLayoutEngineBase,
    id: u32,
    name: &[u8],
) -> Option<i16> {
    let face = engine.font().get_hb_font().get_face().gr_face()?;

    let tag = hb::Tag::from_str(std::str::from_utf8(name).unwrap()).to_raw();

    let feature = face.find_feature_ref(id)?;
    for i in 0..feature.num_values() {
        let lang_id = 0x409;
        let label = feature.value_label(i, lang_id)?;
        if &label.as_bytes()[..name.len()] == name {
            return Some(feature.value(i));
        } else if feature.id() == tag {
            return Some(feature.value(i));
        }
    }
    None
}

#[no_mangle]
pub unsafe extern "C" fn findGraphiteFeatureSettingNamed(
    engine: XeTeXLayoutEngine,
    id: u32,
    name: *const libc::c_char,
    namelength: libc::c_int,
) -> libc::c_long {
    let name = if !name.is_null() && namelength > 0 {
        slice::from_raw_parts(name.cast(), namelength as usize)
    } else {
        return -1;
    };

    find_graphite_feature_setting_named(&*engine, id, name)
        .map(|i| i as libc::c_long)
        .unwrap_or(-1)
}

// TODO: Move these into the engine or such
thread_local! {
    pub static GR_SEGMENT: Cell<Option<gr::OwnSegment>> = const { Cell::new(None) };
    pub static GR_PREV_SLOT: Cell<Option<gr::Slot>> = const { Cell::new(None) };
    pub static GR_TEXT_LEN: Cell<libc::c_uint> = const { Cell::new(0) };
}

#[no_mangle]
pub unsafe extern "C" fn initGraphiteBreaking(
    engine: XeTeXLayoutEngine,
    txt_ptr: *const u16,
    txt_len: libc::c_uint,
) -> bool {
    let engine = &*engine;
    let hb_font = engine.font().get_hb_font();
    let hb_face = hb_font.get_face();
    let Some(gr_face) = hb_face.gr_face() else {
        return false;
    };
    let Some(gr_font) = gr::OwnFont::new(hb_font.get_ptem(), gr_face) else {
        return false;
    };

    let gr_seg = GR_SEGMENT.take();
    if gr_seg.is_some() {
        drop(gr_seg);
        GR_PREV_SLOT.set(None);
    }

    let lang = engine
        .language
        .to_string()
        .map(hb::Tag::from_cstr)
        .map(hb::Tag::to_raw)
        .unwrap_or(0);
    let mut gr_feature_values = gr_face.feature_val_for_lang(lang);

    let features = &engine.features;
    for i in (0..engine.features.len()).rev() {
        let fref = gr_face.find_feature_ref(features[i].tag);
        if let Some(fref) = fref {
            let _ = fref.set_feat_value(&mut gr_feature_values, features[i].value as u16);
        }
    }

    let gr_seg = gr::OwnSegment::new(
        &gr_font,
        gr_face,
        engine.script.to_raw(),
        &gr_feature_values,
        &(txt_ptr, txt_len as usize),
    )
    .unwrap();
    GR_PREV_SLOT.set(Some(gr_seg.first_slot()));
    GR_SEGMENT.set(Some(gr_seg));
    GR_TEXT_LEN.set(txt_len);

    true
}

#[no_mangle]
pub unsafe extern "C" fn findNextGraphiteBreak() -> libc::c_int {
    let Some(gr_seg) = GR_SEGMENT.take() else {
        return -1;
    };
    let Some(gr_prev_slot) = GR_PREV_SLOT.take() else {
        return -1;
    };

    let out = if gr_prev_slot != gr_seg.last_slot() {
        let mut s = gr_seg.next(&gr_prev_slot);
        let mut ret = -1;

        while let Some(slot) = s {
            let ci = gr_seg.cinfo(gr_seg.index(&slot));
            let bw = ci.break_weight();
            if (gr::BREAK_BEFORE_WORD..gr::BREAK_NONE).contains(&bw) {
                GR_PREV_SLOT.set(Some(slot.clone()));
                ret = ci.base() as libc::c_int;
            } else if (gr::BREAK_NONE + 1..=gr::BREAK_WORD).contains(&bw) {
                GR_PREV_SLOT.set(gr_seg.next(&slot));
                ret = (ci.base() + 1) as libc::c_int;
            }

            if ret != -1 {
                break;
            }

            s = gr_seg.next(&slot);
        }

        if ret == -1 {
            GR_PREV_SLOT.set(Some(gr_seg.last_slot()));
            GR_TEXT_LEN.get() as libc::c_int
        } else {
            ret
        }
    } else {
        -1
    };
    GR_SEGMENT.set(Some(gr_seg));
    out
}
