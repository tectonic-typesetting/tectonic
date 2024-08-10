use crate::font::Font;
use crate::manager::{Engine, FontManager};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::ops::{Deref, DerefMut};
use std::ptr;
use tectonic_bridge_graphite2 as gr;
use tectonic_bridge_harfbuzz as hb;
use tectonic_bridge_icu::{UBIDI_DEFAULT_LTR, UBIDI_DEFAULT_RTL};

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

pub struct GrBreak {
    pub(crate) segment: gr::Segment,
    pub(crate) slot: gr::Slot,
    pub(crate) text_len: libc::c_uint,
}

#[repr(C)]
pub struct LayoutEngine {
    font: MaybeBorrow<'static, Font>,
    script: hb::Tag,
    pub(crate) language: hb::Language,
    pub(crate) features: Box<[hb::Feature]>,
    /// the requested shapers
    shaper_list: Vec<*const libc::c_char>,
    /// the actually used shaper
    shaper: Option<CString>,
    rgb_value: u32,
    extend: f32,
    slant: f32,
    embolden: f32,
    hb_buffer: hb::Buffer,
    pub(crate) gr_breaking: Option<GrBreak>,
}

impl LayoutEngine {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        font: MaybeBorrow<'static, Font>,
        script: hb::Tag,
        language: Option<Cow<'static, CStr>>,
        features: Box<[hb::Feature]>,
        shaper_list: Vec<*const libc::c_char>,
        rgb_value: u32,
        extend: f32,
        slant: f32,
        embolden: f32,
    ) -> LayoutEngine {
        let req_engine = FontManager::with_font_manager(|mgr| mgr.get_req_engine());
        LayoutEngine {
            font,
            script,
            // For Graphite fonts treat the language as BCP 47 tag, for OpenType we
            // treat it as a OT language tag for backward compatibility with pre-0.9999
            // XeTeX.
            language: if req_engine == Engine::Graphite {
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
            hb_buffer: hb::Buffer::new(),
            gr_breaking: None,
        }
    }

    pub fn script(&self) -> hb::Tag {
        self.script
    }

    pub fn extend(&self) -> f32 {
        self.extend
    }

    pub fn slant(&self) -> f32 {
        self.slant
    }

    pub fn embolden(&self) -> f32 {
        self.embolden
    }

    pub fn rgb(&self) -> u32 {
        self.rgb_value
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn font_mut(&mut self) -> &mut Font {
        &mut self.font
    }

    pub fn default_dir(&self) -> u8 {
        let script = self.hb_buffer.as_ref().get_script();
        if script.get_horizontal_direction() == hb::Direction::Rtl {
            UBIDI_DEFAULT_RTL
        } else {
            UBIDI_DEFAULT_LTR
        }
    }

    pub fn used_graphite(&self) -> bool {
        self.shaper
            .as_ref()
            .is_some_and(|s| s.to_bytes() == b"graphite2")
    }

    pub fn used_ot(&self) -> bool {
        self.shaper.as_ref().is_some_and(|s| s.to_bytes() == b"ot")
    }

    pub fn hb_buffer(&self) -> hb::BufferRef<'_> {
        self.hb_buffer.as_ref()
    }

    pub fn layout_chars(&mut self, chars: &[u16], offset: i32, count: i32, rtl: bool) -> usize {
        let hb_font = self.font.hb_font();
        let hb_face = hb_font.face();

        let direction = if self.font.layout_dir_vertical() {
            hb::Direction::Ttb
        } else if rtl {
            hb::Direction::Rtl
        } else {
            hb::Direction::Ltr
        };

        let script = self.script.to_script();
        self.hb_buffer.as_mut().reset();
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

        self.hb_buffer
            .as_mut()
            .add_utf16(chars, offset as usize, count as usize);
        self.hb_buffer.as_mut().set_direction(direction);
        self.hb_buffer.as_mut().set_script(script);
        self.hb_buffer.as_mut().set_language(self.language);

        self.hb_buffer.as_mut().guess_segment_properties();
        let segment_props = self.hb_buffer.as_mut().get_segment_properties();

        if self.shaper_list.is_empty() {
            // HarfBuzz gives graphite2 shaper a priority, so that for hybrid
            // Graphite/OpenType fonts, Graphite will be used. However, pre-0.9999
            // XeTeX preferred OpenType over Graphite, so we are doing the same
            // here for sake of backward compatibility. Since "ot" shaper never
            // fails, we set the shaper list to just include it.
            self.shaper_list = vec![b"ot\0".as_ptr().cast::<libc::c_char>(), ptr::null()];
        }

        let mut shape_plan = hb::ShapePlan::new_cached(
            hb_face,
            &segment_props,
            &self.features,
            Some(&self.shaper_list),
        );
        let res = shape_plan
            .as_mut()
            .execute(hb_font, self.hb_buffer.as_mut(), &self.features);

        self.shaper = None;

        if res {
            self.shaper = Some(shape_plan.as_ref().get_shaper().to_owned());
            self.hb_buffer
                .as_mut()
                .set_content_type(hb::BufferContentType::Glyphs);
        } else {
            // all selected shapers failed, retrying with default
            // we don't use _cached here as the cached plain will always fail.
            shape_plan = hb::ShapePlan::new(hb_face, &segment_props, &self.features, None);
            let res = shape_plan
                .as_mut()
                .execute(hb_font, self.hb_buffer.as_mut(), &self.features);

            if res {
                self.shaper = Some(shape_plan.as_ref().get_shaper().to_owned());
                self.hb_buffer
                    .as_mut()
                    .set_content_type(hb::BufferContentType::Glyphs);
            } else {
                panic!("all shapers failed");
            }
        }

        let glyph_count = self.hb_buffer.as_ref().len();

        // #[cfg(feature = "debug")]
        // {
        //     use std::ffi::CStr;
        //
        //     let mut buf = [0u8; 1024];
        //     let mut consumed = 0;
        //     println!("shaper: {}", CStr::from_ptr(engine.shaper));
        //
        //     let flags = HB_BUFFER_SERIALIZE_FLAGS_DEFAULT;
        //     let format = HB_BUFFER_SERIALIZE_FORMAT_JSON;
        //
        //     hb_buffer_serialize_glyphs(
        //         engine.hb_buffer,
        //         0,
        //         glyph_count,
        //         &mut buf,
        //         1024,
        //         &mut consumed,
        //         hb_font,
        //         format,
        //         flags,
        //     );
        //     if consumed != 0 {
        //         println!("buffer glyphs: {}", CStr::from_ptr(&buf));
        //     }
        // }

        glyph_count
    }
}

pub(crate) fn get_graphite_feature_code(engine: &LayoutEngine, index: u32) -> Option<u32> {
    let id = engine
        .font()
        .hb_font()
        .face()
        .gr_face()?
        .feature_ref(index as usize)?
        .id();
    Some(id)
}

pub(crate) fn count_graphite_feature_settings(
    engine: &LayoutEngine,
    feature_id: u32,
) -> Option<u32> {
    let out = engine
        .font()
        .hb_font()
        .face()
        .gr_face()?
        .find_feature_ref(feature_id)?
        .num_values() as u32;
    Some(out)
}

pub(crate) fn get_graphite_feature_setting_code(
    engine: &LayoutEngine,
    feature_id: u32,
    index: u32,
) -> Option<u32> {
    let out = engine
        .font()
        .hb_font()
        .face()
        .gr_face()?
        .find_feature_ref(feature_id)?
        .value(index as usize) as u32;
    Some(out)
}

pub(crate) fn get_graphite_feature_default_setting(
    engine: &LayoutEngine,
    feature_id: u32,
) -> Option<u32> {
    let face = engine.font().hb_font().face().gr_face()?;
    let feat = face.find_feature_ref(feature_id)?;
    let lang = engine
        .language
        .to_string()
        .map(hb::Tag::from_cstr)
        .map(hb::Tag::to_raw)
        .unwrap_or(0);
    let feature_values = face.feature_val_for_lang(lang);
    let out = feat.feat_value(feature_values.as_ref()) as u32;
    Some(out)
}

pub(crate) fn get_graphite_feature_label(
    engine: &LayoutEngine,
    feature_id: u32,
) -> Option<gr::Label> {
    let face = engine.font().hb_font().face().gr_face()?;
    let feature = face.find_feature_ref(feature_id)?;
    let lang_id = 0x409;
    let label = feature.label(lang_id)?;
    Some(label)
}

pub(crate) fn get_graphite_feature_setting_label(
    engine: &LayoutEngine,
    feature_id: u32,
    setting_id: u32,
) -> Option<gr::Label> {
    let face = engine.font().hb_font().face().gr_face()?;

    let feature = face.find_feature_ref(feature_id)?;
    for i in 0..feature.num_values() {
        if setting_id == feature.value(i) as u32 {
            let lang_id = 0x409;
            return feature.value_label(i, lang_id);
        }
    }

    None
}

pub(crate) fn find_graphite_feature(
    engine: &LayoutEngine,
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

pub(crate) fn find_graphite_feature_named(engine: &LayoutEngine, name: &[u8]) -> Option<u32> {
    let gr_face = engine.font().hb_font().face().gr_face()?;

    let tag = hb::Tag::from_str(std::str::from_utf8(name).unwrap()).to_raw();

    for i in 0..gr_face.num_feature_refs() {
        let feature = gr_face.feature_ref(i)?;
        let lang_id = 0x409;
        let label = feature.label(lang_id)?;

        if &label.as_bytes()[..name.len()] == name || feature.id() == tag {
            return Some(feature.id());
        }
    }

    None
}

pub(crate) fn find_graphite_feature_setting_named(
    engine: &LayoutEngine,
    id: u32,
    name: &[u8],
) -> Option<i16> {
    let face = engine.font().hb_font().face().gr_face()?;

    let tag = hb::Tag::from_str(std::str::from_utf8(name).unwrap()).to_raw();

    let feature = face.find_feature_ref(id)?;
    for i in 0..feature.num_values() {
        let lang_id = 0x409;
        let label = feature.value_label(i, lang_id)?;
        if &label.as_bytes()[..name.len()] == name || feature.id() == tag {
            return Some(feature.value(i));
        }
    }
    None
}
