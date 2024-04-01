use super::font::XeTeXFontBase;
use crate::c_api::manager::getReqEngine;
use crate::c_api::{FloatPoint, GlyphBBox, XeTeXFont, XeTeXLayoutEngine};
use std::borrow::Cow;
use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::{ptr, slice};
use tectonic_bridge_graphite2 as gr;
use tectonic_bridge_graphite2::sys::{
    gr_breakBeforeWord, gr_breakNone, gr_breakWord, gr_cinfo_base, gr_cinfo_break_weight,
    gr_encform, gr_make_seg, gr_seg_cinfo, gr_seg_destroy, gr_seg_first_slot, gr_seg_last_slot,
    gr_segment, gr_slot, gr_slot_index, gr_slot_next_in_segment,
};
use tectonic_bridge_harfbuzz as hb;
use tectonic_bridge_icu::{UChar32, UBIDI_DEFAULT_LTR, UBIDI_DEFAULT_RTL};

#[repr(C)]
pub struct XeTeXLayoutEngineBase {
    font: Box<XeTeXFontBase>,
    script: hb::Tag,
    language: hb::Language,
    features: &'static [hb::Feature],
    /// the requested shapers
    shaper_list: Cow<'static, [*const libc::c_char]>,
    /// the actually used shaper
    shaper: Option<CString>,
    rgb_value: u32,
    extend: f32,
    slant: f32,
    embolden: f32,
    hb_buffer: hb::OwnBuffer,
}

impl XeTeXLayoutEngineBase {
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
            Some(CString::from_raw(language))
        } else {
            None
        };
        let features = if !features.is_null() {
            slice::from_raw_parts(features, n_features as usize)
        } else {
            &[]
        };
        let shaper_list = Cow::Borrowed(if !shapers.is_null() {
            let mut len = 0;
            while !(*shapers.add(len)).is_null() {
                len += 1;
            }
            slice::from_raw_parts(shapers, len + 1)
        } else {
            &[]
        });

        let this = Box::new(XeTeXLayoutEngineBase {
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
        });

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

        if let Cow::Borrowed(&[]) = engine.shaper_list {
            // HarfBuzz gives graphite2 shaper a priority, so that for hybrid
            // Graphite/OpenType fonts, Graphite will be used. However, pre-0.9999
            // XeTeX preferred OpenType over Graphite, so we are doing the same
            // here for sake of backward compatibility. Since "ot" shaper never
            // fails, we set the shaper list to just include it.
            engine.shaper_list = Cow::Owned(vec![c!("ot"), ptr::null()]);
        }

        let mut shape_plan = hb::OwnShapePlan::new_cached(
            hb_face,
            &segment_props,
            engine.features,
            Some(&engine.shaper_list),
        );
        let res = shape_plan.execute(hb_font, &mut engine.hb_buffer, engine.features);

        engine.shaper = None;

        if res {
            engine.shaper = Some(shape_plan.get_shaper().to_owned());
            engine
                .hb_buffer
                .set_content_type(hb::BufferContentType::Glyphs);
        } else {
            // all selected shapers failed, retrying with default
            // we don't use _cached here as the cached plain will always fail.
            shape_plan = hb::OwnShapePlan::new(hb_face, &segment_props, engine.features, None);
            let res = shape_plan.execute(hb_font, &mut engine.hb_buffer, engine.features);

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

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureCode(engine: XeTeXLayoutEngine, index: u32) -> u32 {
    let hb_face = (*engine).font().get_hb_font().get_face();
    match hb_face.gr_face() {
        Some(face) => face.feature_ref(index as usize).unwrap().id(),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn countGraphiteFeatureSettings(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
) -> u32 {
    let hb_face = (*engine).font().get_hb_font().get_face();
    match hb_face.gr_face() {
        Some(face) => face.find_feature_ref(feature_id).unwrap().num_values() as u32,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureSettingCode(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
    index: u32,
) -> u32 {
    let hb_face = (*engine).font().get_hb_font().get_face();
    match hb_face.gr_face() {
        Some(face) => face
            .find_feature_ref(feature_id)
            .unwrap()
            .value(index as usize) as u32,
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureDefaultSetting(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
) -> u32 {
    let engine = &*engine;
    let hb_face = engine.font().get_hb_font().get_face();

    match hb_face.gr_face() {
        Some(face) => {
            let feat = face.find_feature_ref(feature_id).unwrap();
            let feature_values =
                face.feature_val_for_lang(hb::Tag::from_cstr(engine.language.to_string()).to_raw());

            feat.feat_value(&feature_values) as u32
        }
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureLabel(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
) -> *const libc::c_char {
    let hb_face = (*engine).font().get_hb_font().get_face();
    match hb_face.gr_face() {
        Some(face) => {
            let feature = face.find_feature_ref(feature_id).unwrap();
            let lang_id = 0x409;

            feature.label(lang_id).unwrap().into_raw().cast()
        }
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn getGraphiteFeatureSettingLabel(
    engine: XeTeXLayoutEngine,
    feature_id: u32,
    setting_id: u32,
) -> *const libc::c_char {
    let hb_face = (*engine).font().get_hb_font().get_face();

    let Some(gr_face) = hb_face.gr_face() else {
        return ptr::null();
    };

    let feature = gr_face.find_feature_ref(feature_id).unwrap();
    for i in 0..feature.num_values() {
        if setting_id == feature.value(i) as u32 {
            let lang_id = 0x409;
            return feature.value_label(i, lang_id).unwrap().into_raw().cast();
        }
    }

    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn findGraphiteFeature(
    engine: XeTeXLayoutEngine,
    s: *const libc::c_char,
    e: *const libc::c_char,
    f: *mut hb::Tag,
    v: *mut libc::c_int,
) -> bool {
    let mut s = s.cast::<u8>();
    let e = e.cast::<u8>();

    *f = hb::Tag::new(0);
    *v = 0;

    while *s == b' ' || *s == b'\t' {
        s = s.add(1);
    }
    let mut cp = s;
    while cp < e && *cp != b'=' {
        cp = cp.add(1);
    }

    let tmp = findGraphiteFeatureNamed(engine, s.cast(), cp.byte_offset_from(s) as libc::c_int);
    *f = hb::Tag::new(tmp as _);
    if tmp == -1 {
        return false;
    }

    cp = cp.add(1);
    while cp < e && (*cp == b' ' || *cp == b'\t') {
        cp = cp.add(1);
    }

    if cp == e {
        return false;
    }

    *v = findGraphiteFeatureSettingNamed(
        engine,
        (*f).to_raw(),
        cp.cast(),
        e.byte_offset_from(cp) as libc::c_int,
    ) as libc::c_int;

    *v != -1
}

#[no_mangle]
pub unsafe extern "C" fn findGraphiteFeatureNamed(
    engine: XeTeXLayoutEngine,
    name: *const libc::c_char,
    namelength: libc::c_int,
) -> libc::c_long {
    let name = slice::from_raw_parts(name.cast::<u8>(), namelength as usize);
    let hb_face = (*engine).font().get_hb_font().get_face();

    let Some(gr_face) = hb_face.gr_face() else {
        return -1;
    };

    for i in 0..gr_face.num_feature_refs() {
        let feature = gr_face.feature_ref(i).unwrap();
        let lang_id = 0x409;
        let label = feature.label(lang_id).unwrap();

        if label.as_bytes() == name {
            return feature.id() as libc::c_long;
        }
    }

    -1
}

#[no_mangle]
pub unsafe extern "C" fn findGraphiteFeatureSettingNamed(
    engine: XeTeXLayoutEngine,
    id: u32,
    name: *const libc::c_char,
    namelength: libc::c_int,
) -> libc::c_long {
    let name = slice::from_raw_parts(name.cast(), namelength as usize);
    let hb_face = (*engine).font().get_hb_font().get_face();
    let Some(gr_face) = hb_face.gr_face() else {
        return -1;
    };

    let feature = gr_face.find_feature_ref(id).unwrap();
    for i in 0..feature.num_values() {
        let lang_id = 0x409;
        let label = feature.value_label(i, lang_id).unwrap();

        if label.as_bytes() == name {
            return feature.value(i) as libc::c_long;
        }
    }

    -1
}

thread_local! {
    pub static GR_SEGMENT: Cell<*mut gr_segment> = const { Cell::new(ptr::null_mut()) };
    pub static GR_PREV_SLOT: Cell<*const gr_slot> = const { Cell::new(ptr::null()) };
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

    let gr_seg = GR_SEGMENT.get();

    if !gr_seg.is_null() {
        gr_seg_destroy(gr_seg);
        GR_SEGMENT.set(ptr::null_mut());
        GR_PREV_SLOT.set(ptr::null());
    }

    let mut gr_feature_values =
        gr_face.feature_val_for_lang(hb::Tag::from_cstr(engine.language.to_string()).to_raw());

    let features = engine.features;
    for i in (0..engine.features.len()).rev() {
        let fref = gr_face.find_feature_ref(features[i].tag);
        if let Some(fref) = fref {
            let _ = fref.set_feat_value(&mut gr_feature_values, features[i].value as u16);
        }
    }

    GR_SEGMENT.set(gr_make_seg(
        (*gr_font).as_ptr(),
        gr_face.as_ptr(),
        engine.script.to_raw(),
        gr_feature_values.as_ptr(),
        gr_encform::utf16,
        txt_ptr.cast(),
        txt_len as libc::size_t,
        0,
    ));
    GR_PREV_SLOT.set(gr_seg_first_slot(gr_seg));
    GR_TEXT_LEN.set(txt_len);

    true
}

#[no_mangle]
pub unsafe extern "C" fn findNextGraphiteBreak() -> libc::c_int {
    let gr_seg = GR_SEGMENT.get();
    let gr_prev_slot = GR_PREV_SLOT.get();

    if !gr_seg.is_null() && !gr_prev_slot.is_null() && gr_prev_slot != gr_seg_last_slot(gr_seg) {
        let mut s = gr_slot_next_in_segment(gr_prev_slot);
        let mut ret = -1;

        while !s.is_null() {
            let ci = gr_seg_cinfo(gr_seg, gr_slot_index(s));
            let bw = gr_cinfo_break_weight(ci);
            if (gr_breakBeforeWord..gr_breakNone).contains(&bw) {
                GR_PREV_SLOT.set(s);
                ret = gr_cinfo_base(ci) as libc::c_int;
            } else if (gr_breakNone + 1..=gr_breakWord).contains(&bw) {
                GR_PREV_SLOT.set(gr_slot_next_in_segment(s));
                ret = (gr_cinfo_base(ci) + 1) as libc::c_int;
            }

            if ret != -1 {
                break;
            }

            s = gr_slot_next_in_segment(s);
        }

        if ret == -1 {
            GR_PREV_SLOT.set(gr_seg_last_slot(gr_seg));
            GR_TEXT_LEN.get() as libc::c_int
        } else {
            ret
        }
    } else {
        -1
    }
}
