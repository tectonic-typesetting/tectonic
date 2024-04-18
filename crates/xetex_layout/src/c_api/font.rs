#[cfg(not(target_os = "macos"))]
use crate::c_api::fc;
use crate::c_api::{
    d_to_fix, fix_to_d, raw_to_rs, ttstub_input_close, ttstub_input_get_size, ttstub_input_open,
    ttstub_input_read, Fixed, GlyphBBox, GlyphID, OTTag, PlatformFontRef, RawPlatformFontRef,
    XeTeXFont,
};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::ptr;
use std::rc::Rc;
use std::sync::OnceLock;
use tectonic_bridge_core::FileFormat;
use tectonic_bridge_freetype2 as ft;
use tectonic_bridge_harfbuzz as hb;
#[cfg(target_os = "macos")]
use tectonic_mac_core::sys::CTFontRef;
#[cfg(target_os = "macos")]
use tectonic_mac_core::{
    CFArray, CFDictionary, CFString, CFType, CFUrl, CTFont, CTFontDescriptor, CoreType,
    FontAttribute, FontNameKey,
};

fn get_glyph_advance(face: &ft::Face, gid: libc::c_uint, vertical: bool) -> ft::Fixed {
    let flags = if vertical {
        ft::LoadFlags::NO_SCALE | ft::LoadFlags::VERTICAL_LAYOUT
    } else {
        ft::LoadFlags::NO_SCALE
    };
    let out = match face.get_advance(gid, flags) {
        Ok(advance) => {
            if vertical {
                -advance
            } else {
                advance
            }
        }
        Err(_) => 0,
    };
    out as ft::Fixed
}

pub fn get_font_funcs() -> hb::FontFuncs<Rc<RefCell<ft::Face>>> {
    static FONTS: OnceLock<hb::FontFuncs<Rc<RefCell<ft::Face>>>> = OnceLock::new();

    FONTS
        .get_or_init(|| {
            let mut funcs = hb::FontFuncs::<Rc<RefCell<ft::Face>>>::new();

            funcs.nominal_glyph_func(|_, face, ch| {
                face.borrow().get_char_index(ch).map(|cc| cc.get())
            });
            funcs.variation_glyph_func(|_, face, ch, vs| {
                face.borrow()
                    .get_char_variant_index(ch, vs)
                    .map(|cc| cc.get())
            });
            funcs.glyph_h_advance(|_, face, gid| {
                get_glyph_advance(&face.borrow(), gid, false) as hb::Position
            });
            funcs.glyph_v_advance(|_, face, gid| {
                get_glyph_advance(&face.borrow(), gid, true) as hb::Position
            });
            funcs.glyph_h_origin(|_, _, _| Some((0, 0)));
            funcs.glyph_v_origin(|_, _, _| {
                Some((0, 0))

                // TODO
                // Keep the code below for reference, for now we want to keep vertical
                // origin at (0, 0) for compatibility with pre-0.9999.
                // Reconsider this (e.g. using BASE table) when we get around overhauling
                // the text directionality model and implementing real vertical typesetting.

                /*
                FT_Face face = (FT_Face) font_data;
                FT_Error error;

                error = FT_Load_Glyph (face, gid, FT_LOAD_NO_SCALE);
                if (!error) {
                    *x = face->glyph->metrics.horiBearingX -   face->glyph->metrics.vertBearingX;
                    *y = face->glyph->metrics.horiBearingY - (-face->glyph->metrics.vertBearingY);
                }

                return !error;
                 */
            });
            funcs.glyph_h_kerning(|_, face, gid1, gid2| {
                match face
                    .borrow()
                    .get_kerning(gid1, gid2, ft::KerningMode::Unscaled)
                {
                    Ok(vec) => vec.x as hb::Position,
                    Err(_) => 0,
                }
            });
            funcs.glyph_v_kerning(|_, _, _, _| 0);
            funcs.glyph_extents(|_, face, gid| {
                let mut face = face.borrow_mut();
                if face.load_glyph(gid, ft::LoadFlags::NO_SCALE).is_ok() {
                    Some(hb::GlyphExtents {
                        x_bearing: face.glyph().metrics().horiBearingX as hb::Position,
                        y_bearing: face.glyph().metrics().horiBearingY as hb::Position,
                        width: face.glyph().metrics().width as hb::Position,
                        height: -face.glyph().metrics().height as hb::Position,
                    })
                } else {
                    None
                }
            });
            funcs.glyph_contour_point(|_, face, gid, point_index| {
                let mut face = face.borrow_mut();

                let error = face.load_glyph(gid, ft::LoadFlags::NO_SCALE).is_err();
                if !error
                    && face.glyph().format() == ft::GlyphFormat::Outline
                    && point_index < (face.glyph().outline().n_points as u32)
                {
                    let x = face.glyph().outline().points()[point_index as usize].x as hb::Position;
                    let y = face.glyph().outline().points()[point_index as usize].y as hb::Position;
                    Some((x, y))
                } else {
                    None
                }
            });
            funcs.glyph_name(
                |_, face, gid, buf| match face.borrow().get_glyph_name(gid, buf) {
                    Ok(str) if !str.to_bytes().is_empty() && str.to_bytes()[0] == 0 => 0,
                    Err(_) => 0,
                    Ok(str) => str.to_bytes().len(),
                },
            );

            funcs
        })
        .clone()
}

#[no_mangle]
pub unsafe extern "C" fn createFont(font_ref: RawPlatformFontRef, point_size: Fixed) -> XeTeXFont {
    let font_ref = match raw_to_rs(font_ref) {
        Some(fr) => fr,
        None => return ptr::null_mut(),
    };

    match XeTeXFontBase::new(font_ref, fix_to_d(point_size) as f32) {
        Err(_) => ptr::null_mut(),
        Ok(out) => Box::into_raw(Box::new(out)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn createFontFromFile(
    filename: *const libc::c_char,
    index: libc::c_int,
    point_size: Fixed,
) -> XeTeXFont {
    let filename = if filename.is_null() {
        None
    } else {
        Some(CStr::from_ptr(filename))
    };

    match XeTeXFontBase::new_path_index(filename, index, fix_to_d(point_size) as f32) {
        Err(_) => ptr::null_mut(),
        Ok(out) => Box::into_raw(Box::new(out)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn deleteFont(font: XeTeXFont) {
    let _ = Box::from_raw(font);
}

pub fn get_larger_script_list_table(font: &XeTeXFontBase) -> Vec<hb::Tag> {
    let face = font.get_hb_font().get_face();

    let sl_sub = face.get_ot_layout_script_tags(hb::GTag::GSub);
    let sl_pos = face.get_ot_layout_script_tags(hb::GTag::GPos);

    if sl_sub.len() > sl_pos.len() {
        sl_sub
    } else {
        sl_pos
    }
}

#[no_mangle]
pub unsafe extern "C" fn countScripts(font: XeTeXFont) -> libc::c_uint {
    get_larger_script_list_table(&*font).len() as libc::c_uint
}

#[no_mangle]
pub unsafe extern "C" fn countLanguages(font: XeTeXFont, script: hb::Tag) -> libc::c_uint {
    let face = (*font).get_hb_font().get_face();

    let script_list = get_larger_script_list_table(&*font);
    let mut rval = 0;
    for (i, script_i) in script_list.iter().enumerate() {
        if *script_i == script {
            rval += face.get_ot_layout_script_language_tags_len(hb::GTag::GSub, i);
            rval += face.get_ot_layout_script_language_tags_len(hb::GTag::GPos, i);
            break;
        }
    }

    rval as libc::c_uint
}

#[no_mangle]
pub unsafe extern "C" fn countFeatures(
    font: XeTeXFont,
    script: hb::Tag,
    language: hb::Tag,
) -> libc::c_uint {
    let face = (*font).get_hb_font().get_face();

    let mut rval = 0;
    for i in 0..2 {
        let table_tag = if i == 0 {
            hb::GTag::GSub
        } else {
            hb::GTag::GPos
        };
        if let Some(script_idx) = face.find_ot_layout_script(table_tag, script) {
            let lang = face.select_ot_layout_language(table_tag, script_idx, &[language]);
            let lang_idx = lang.unwrap_or_else(|idx| idx);

            if lang.is_ok() || language == hb::Tag::new(0) {
                rval +=
                    face.get_ot_layout_language_feature_tags_len(table_tag, script_idx, lang_idx);
            }
        }
    }
    rval as libc::c_uint
}

enum FontKind {
    FtFont,
    #[cfg(target_os = "macos")]
    Mac(CTFontDescriptor, Option<CTFont>),
}

/// cbindgen:rename-all=camelCase
#[repr(C)]
pub struct XeTeXFontBase {
    units_per_em: libc::c_ushort,
    point_size: f32,
    ascent: f32,
    descent: f32,
    cap_height: f32,
    x_height: f32,
    italic_angle: f32,

    vertical: bool,

    filename: CString,
    index: u32,

    ft_face: Option<Rc<RefCell<ft::Face>>>,
    hb_font: Option<hb::OwnFont>,

    kind: FontKind,
}

impl XeTeXFontBase {
    #[cfg(not(target_os = "macos"))]
    pub(crate) fn new(font: PlatformFontRef, point_size: f32) -> Result<XeTeXFontBase, i32> {
        let path = font.get::<fc::pat::File>(0).ok();
        let index = font.get::<fc::pat::Index>(0).unwrap_or(0);

        XeTeXFontBase::new_path_index(path, index, point_size)
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn new(descriptor: PlatformFontRef, point_size: f32) -> Result<XeTeXFontBase, i32> {
        let mut out = XeTeXFontBase {
            units_per_em: 0,
            point_size,
            ascent: 0.0,
            descent: 0.0,
            cap_height: 0.0,
            x_height: 0.0,
            italic_angle: 0.0,
            vertical: false,
            filename: CString::new("").unwrap(),
            index: 0,
            ft_face: None,
            hb_font: None,
            kind: FontKind::Mac(descriptor, None),
        };
        let status = out.initialize_mac();
        if status != 0 {
            Err(status)
        } else {
            Ok(out)
        }
    }

    pub(crate) fn new_path_index(
        path: Option<&CStr>,
        index: libc::c_int,
        point_size: f32,
    ) -> Result<XeTeXFontBase, i32> {
        let mut out = XeTeXFontBase {
            units_per_em: 0,
            point_size,
            ascent: 0.0,
            descent: 0.0,
            cap_height: 0.0,
            x_height: 0.0,
            italic_angle: 0.0,
            vertical: false,
            filename: CString::new("").unwrap(),
            index: 0,
            ft_face: None,
            hb_font: None,
            kind: FontKind::FtFont,
        };
        let status = if let Some(path) = path {
            out.initialize_ft(path, index)
        } else {
            0
        };
        if status != 0 {
            Err(status)
        } else {
            Ok(out)
        }
    }

    fn initialize_ft(&mut self, pathname: &CStr, index: libc::c_int) -> i32 {
        let mut handle = unsafe { ttstub_input_open(pathname.as_ptr(), FileFormat::OpenType, 0) };
        if handle.is_null() {
            handle = unsafe { ttstub_input_open(pathname.as_ptr(), FileFormat::TrueType, 0) };
        }
        if handle.is_null() {
            handle = unsafe { ttstub_input_open(pathname.as_ptr(), FileFormat::Type1, 0) };
        }
        if handle.is_null() {
            return 1;
        }

        let sz = unsafe { ttstub_input_get_size(handle) };
        let mut backing_data = vec![0; sz];
        let r = unsafe { ttstub_input_read(handle, backing_data.as_mut_ptr().cast(), sz) };
        if r < 0 || (r as usize) != sz {
            panic!("failed to read font file");
        }
        unsafe { ttstub_input_close(handle) };

        self.ft_face = match ft::Face::new_memory(backing_data, index as usize) {
            Ok(face) => Some(Rc::new(RefCell::new(face))),
            Err(_) => return 1,
        };

        if !self.ft_face().is_scalable() {
            return 1;
        }

        if index == 0 && !self.ft_face().is_sfnt() {
            let pathname = pathname.to_bytes();
            let mut afm = pathname
                .rsplit(|c| *c == b'/')
                .next()
                .unwrap_or(pathname)
                .to_vec();
            let file_ty = afm.rsplit_mut(|c| *c == b'.').next();
            if let Some(file_ty) = file_ty {
                if file_ty.len() == 3
                    && file_ty[0].to_ascii_lowercase() == b'p'
                    && file_ty[1].to_ascii_lowercase() == b'f'
                {
                    file_ty.copy_from_slice(b"afm");
                }
            }
            afm.push(0);

            let afm_handle = unsafe { ttstub_input_open(afm.as_ptr().cast(), FileFormat::Afm, 0) };

            if !afm_handle.is_null() {
                let sz = unsafe { ttstub_input_get_size(afm_handle) };
                let mut backing_data2 = vec![0; sz];
                let r = unsafe { ttstub_input_read(handle, backing_data2.as_mut_ptr().cast(), sz) };

                if r < 0 || (r as usize) != sz {
                    panic!("failed to read AFM file");
                }

                self.ft_face_mut().attach_stream_mem(backing_data2).unwrap();
            }
        }

        self.filename = pathname.to_owned();
        self.index = index as u32;
        let upe = { self.ft_face().units_per_em() };
        self.units_per_em = upe;
        let a = { self.ft_face().ascender() } as f64;
        self.ascent = self.units_to_points(a) as f32;
        let d = { self.ft_face().descender() } as f64;
        self.descent = self.units_to_points(d) as f32;

        let ft_face = self.ft_face();
        let post_table = ft_face.get_sfnt_table::<ft::Postscript>();
        let ia = if let Some(table) = post_table {
            fix_to_d(table.italic_angle as Fixed) as f32
        } else {
            self.italic_angle
        };
        drop(ft_face);
        self.italic_angle = ia;

        let ft_face = self.ft_face();
        let os2_table = ft_face.get_sfnt_table::<ft::Os2>();
        let (ch, xh) = if let Some(table) = os2_table {
            let ch = self.units_to_points(table.sCapHeight as f64) as f32;
            let xh = self.units_to_points(table.sxHeight as f64) as f32;
            (ch, xh)
        } else {
            (self.cap_height, self.x_height)
        };
        drop(ft_face);
        self.cap_height = ch;
        self.x_height = xh;

        let ft_face = Rc::clone(self.ft_face.as_ref().unwrap());
        let mut hb_face = hb::OwnFace::new_tables(move |_, tag| {
            if let Ok(table) = ft_face
                .borrow()
                .load_sfnt_table(ft::TableTag::Other(tag.to_raw()))
            {
                Some(hb::Blob::new(table))
            } else {
                None
            }
        });

        hb_face.set_index(self.index);
        hb_face.set_upem(self.units_per_em as u32);

        let mut hb_font = hb::OwnFont::new(&hb_face);

        hb_font.set_funcs(get_font_funcs(), Rc::clone(self.ft_face.as_ref().unwrap()));
        hb_font.set_scale(self.units_per_em as i32, self.units_per_em as i32);
        hb_font.set_ppem(0, 0);

        self.hb_font = Some(hb_font);

        0
    }

    #[cfg(target_os = "macos")]
    pub fn initialize_mac(&mut self) -> i32 {
        let FontKind::Mac(descriptor, font_ref) = &mut self.kind else {
            return 1;
        };

        let empty_cascade_list = CFArray::<CFType>::empty();
        let attributes =
            CFDictionary::new([(FontAttribute::CascadeList.to_str(), empty_cascade_list)]);

        *descriptor = descriptor.copy_with_attrs(&attributes);
        *font_ref = Some(CTFont::new_descriptor(
            descriptor,
            self.point_size as f64 * 72.0 / 72.27,
        ));
        let mut index = 0;
        let pathname = get_file_name_from_ct_font(font_ref.as_ref().unwrap(), &mut index).unwrap();
        self.initialize_ft(&pathname, index as libc::c_int)
    }

    #[no_mangle]
    pub unsafe extern "C" fn hasFontTable(font: XeTeXFont, table_tag: OTTag) -> bool {
        // TODO: has_font_table for efficiency
        (*font)
            .load_font_table(ft::TableTag::Other(table_tag))
            .is_some()
    }

    #[no_mangle]
    pub unsafe extern "C" fn getSlant(font: XeTeXFont) -> Fixed {
        let angle = (*font).italic_angle() as f64;
        d_to_fix(f64::tan(-angle * std::f64::consts::PI / 180.0))
    }

    #[no_mangle]
    pub unsafe extern "C" fn countGlyphs(font: XeTeXFont) -> libc::c_uint {
        (*font).get_num_glyphs() as libc::c_uint
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphWidth(font: XeTeXFont, gid: u32) -> f32 {
        (*font).get_glyph_width(gid)
    }

    #[no_mangle]
    pub unsafe extern "C" fn setFontLayoutDir(font: XeTeXFont, vertical: libc::c_int) {
        (*font).set_layout_dir_vertical(vertical != 0)
    }

    #[no_mangle]
    pub unsafe extern "C" fn getIndScript(font: XeTeXFont, index: libc::c_uint) -> hb::Tag {
        let mut rval = hb::Tag::new(0);
        let script_list = get_larger_script_list_table(&*font);
        if (index as usize) < script_list.len() {
            rval = script_list[index as usize];
        }

        rval
    }

    #[no_mangle]
    pub unsafe extern "C" fn getIndLanguage(
        font: XeTeXFont,
        script: hb::Tag,
        index: libc::c_uint,
    ) -> hb::Tag {
        let index = index as usize;
        let mut rval = hb::Tag::new(0);
        let face = (*font).get_hb_font().get_face();

        let script_list = get_larger_script_list_table(&*font);
        for (i, script_i) in script_list.iter().enumerate() {
            if *script_i == script {
                let lang_list = face.get_ot_layout_script_language_tags(hb::GTag::GSub, i);
                if index < lang_list.len() {
                    rval = lang_list[index];
                    break;
                }

                let lang_list = face.get_ot_layout_script_language_tags(hb::GTag::GPos, i);
                if index < lang_list.len() {
                    rval = lang_list[index];
                    break;
                }
            }
        }

        rval
    }

    #[no_mangle]
    pub unsafe extern "C" fn getIndFeature(
        font: XeTeXFont,
        script: hb::Tag,
        language: hb::Tag,
        mut index: libc::c_uint,
    ) -> hb::Tag {
        let face = (*font).get_hb_font().get_face();

        let mut rval = hb::Tag::new(0);
        for i in 0..2 {
            let table_tag = if i == 0 {
                hb::GTag::GSub
            } else {
                hb::GTag::GPos
            };

            if let Some(script_idx) = face.find_ot_layout_script(table_tag, script) {
                let lang = face.select_ot_layout_language(table_tag, script_idx, &[language]);
                let lang_idx = lang.unwrap_or_else(|idx| idx);

                if lang.is_ok() || language == hb::Tag::new(0) {
                    let feat_list =
                        face.get_ot_layout_language_feature_tags(table_tag, script_idx, lang_idx);

                    if (index as usize) < feat_list.len() {
                        rval = feat_list[index as usize];
                        break;
                    }

                    index -= feat_list.len() as libc::c_uint;
                }
            }
        }

        rval
    }

    #[no_mangle]
    pub unsafe extern "C" fn getGlyphName(
        font: XeTeXFont,
        gid: u16,
        len: *mut libc::c_int,
    ) -> *const libc::c_char {
        match (*font).get_glyph_name(gid) {
            Some(out) => {
                *len = out.as_bytes().len() as libc::c_int;
                CString::into_raw(out)
            }
            None => {
                *len = 0;
                ptr::null()
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn freeGlyphName(name: *mut libc::c_char) {
        let _ = CString::from_raw(name);
    }

    #[no_mangle]
    pub unsafe extern "C" fn ttxl_font_units_to_points(font: XeTeXFont, units: f32) -> f32 {
        (*font).units_to_points(units as f64) as f32
    }

    #[no_mangle]
    pub unsafe extern "C" fn ttxl_font_points_to_units(font: XeTeXFont, points: f32) -> f32 {
        (*font).points_to_units(points as f64) as f32
    }

    #[no_mangle]
    pub unsafe extern "C" fn ttxl_font_get_point_size(font: XeTeXFont) -> f32 {
        (*font).point_size()
    }

    pub(crate) fn ft_face(&self) -> std::cell::Ref<'_, ft::Face> {
        self.ft_face.as_ref().unwrap().borrow()
    }

    fn ft_face_mut(&mut self) -> std::cell::RefMut<'_, ft::Face> {
        self.ft_face.as_mut().unwrap().borrow_mut()
    }

    pub(crate) fn get_glyph_name(&self, gid: u16) -> Option<CString> {
        if self.ft_face().has_glyph_names() {
            let mut buf = vec![0u8; 256];
            self.ft_face().get_glyph_name(gid as u32, &mut buf).unwrap();

            CStr::from_bytes_until_nul(&buf).map(CStr::to_owned).ok()
        } else {
            None
        }
    }

    pub(crate) fn get_glyph_sidebearings(
        &mut self,
        gid: GlyphID,
        lsb: Option<&mut f32>,
        rsb: Option<&mut f32>,
    ) {
        let width = self.get_glyph_width(gid as u32);

        let bbox = self.get_glyph_bounds(gid);

        if let Some(lsb) = lsb {
            *lsb = bbox.x_min;
        }
        if let Some(rsb) = rsb {
            *rsb = width - bbox.x_max;
        }
    }

    pub(crate) fn get_glyph_ital_corr(&mut self, gid: GlyphID) -> f32 {
        let width = self.get_glyph_width(gid as u32);
        let bbox = self.get_glyph_bounds(gid);

        if bbox.x_max > width {
            bbox.x_max - width
        } else {
            0.0
        }
    }

    pub(crate) fn map_char_to_glyph(&self, ch: u32) -> GlyphID {
        match self.ft_face().get_char_index(ch) {
            Some(val) => val.get() as GlyphID,
            None => 0,
        }
    }

    pub(crate) fn get_first_char_code(&self) -> u32 {
        self.ft_face().get_first_char().0
    }

    pub(crate) fn get_last_char_code(&self) -> u32 {
        let ft_face = self.ft_face();

        let (mut ch, mut index) = ft_face.get_first_char();
        let mut prev = ch;
        while index != 0 {
            prev = ch;
            (ch, index) = ft_face.get_next_char(ch);
        }
        prev
    }

    pub(crate) fn map_glyph_to_index(&self, glyph_name: &CStr) -> GlyphID {
        match self.ft_face().get_name_index(glyph_name) {
            Some(index) => index.get() as u16,
            None => 0,
        }
    }

    pub(crate) fn load_font_table(&self, tag: ft::TableTag) -> Option<Vec<u8>> {
        self.ft_face().load_sfnt_table(tag).ok()
    }

    pub(crate) fn get_glyph_bounds(&mut self, gid: GlyphID) -> GlyphBBox {
        let mut bbox = GlyphBBox::default();

        if self
            .ft_face_mut()
            .load_glyph(gid as u32, ft::LoadFlags::NO_SCALE)
            .is_err()
        {
            return bbox;
        }

        if let Ok(glyph) = self.ft_face().glyph().get_glyph() {
            let ft_bbox = glyph.get_cbox(ft::BBoxMode::Unscaled);
            bbox.x_min = self.units_to_points(ft_bbox.x_min as f64) as f32;
            bbox.y_min = self.units_to_points(ft_bbox.y_min as f64) as f32;
            bbox.x_max = self.units_to_points(ft_bbox.x_max as f64) as f32;
            bbox.y_max = self.units_to_points(ft_bbox.y_max as f64) as f32;
        }

        bbox
    }

    pub(crate) fn get_glyph_height_depth(
        &mut self,
        gid: GlyphID,
        height: Option<&mut f32>,
        depth: Option<&mut f32>,
    ) {
        let bbox = self.get_glyph_bounds(gid);
        if let Some(height) = height {
            *height = bbox.y_max;
        }
        if let Some(depth) = depth {
            *depth = -bbox.y_min;
        }
    }

    pub(crate) fn get_filename(&self, index: &mut u32) -> &CStr {
        *index = self.index;
        &self.filename
    }

    // pub(crate) fn get_font_table<T: ft::Table>(&self) -> Option<&T::Table> {
    //     self.ft_face().get_sfnt_table::<T>()
    // }

    pub(crate) fn italic_angle(&self) -> f32 {
        self.italic_angle
    }

    pub(crate) fn get_num_glyphs(&self) -> usize {
        self.ft_face().num_glyphs()
    }

    pub(crate) fn get_glyph_width(&self, gid: u32) -> f32 {
        self.units_to_points(get_glyph_advance(&self.ft_face(), gid, false) as f64) as f32
    }

    pub(crate) fn layout_dir_vertical(&self) -> bool {
        self.vertical
    }

    pub(crate) fn set_layout_dir_vertical(&mut self, vertical: bool) {
        self.vertical = vertical;
    }

    pub(crate) fn point_size(&self) -> f32 {
        self.point_size
    }

    pub(crate) fn ascent(&self) -> f32 {
        self.ascent
    }

    pub(crate) fn descent(&self) -> f32 {
        self.descent
    }

    pub(crate) fn cap_height(&self) -> f32 {
        self.cap_height
    }

    pub(crate) fn x_height(&self) -> f32 {
        self.x_height
    }

    pub(crate) fn get_hb_font(&self) -> &hb::Font {
        self.hb_font.as_ref().unwrap()
    }

    pub(crate) fn try_get_hb_font(&self) -> Option<&hb::Font> {
        self.hb_font.as_deref()
    }

    /* Tectonic: these are modified from the base XeTeX code to use doubles;
     * otherwise roundoff errors can accumulate leading to differences in the
     * XDV outputs. */
    pub(crate) fn units_to_points(&self, units: f64) -> f64 {
        (units * self.point_size as f64) / (self.units_per_em as f64)
    }

    fn points_to_units(&self, points: f64) -> f64 {
        (points * (self.units_per_em as f64)) / self.point_size as f64
    }
}

#[cfg(target_os = "macos")]
pub fn get_name_from_ct_font(ct_font: &CTFont, name_key: FontNameKey) -> Option<CFString> {
    ct_font.name(name_key)
}

#[cfg(target_os = "macos")]
fn get_file_name_from_ct_font(ct_font: &CTFont, index: &mut u32) -> Option<CString> {
    let url = ct_font
        .attr(FontAttribute::URL)
        .and_then(|t| t.downcast::<CFUrl>().ok())?;

    let pathname = url.fs_representation()?;
    *index = 0;

    let face = ft::Face::new(&pathname, 0);
    if let Ok(face) = face {
        if face.num_faces() > 1 {
            let num_faces = face.num_faces();
            let ps_name1 = ct_font.name(FontNameKey::PostScript);
            *index = 0xFFFFFFFF;
            for i in 0..num_faces {
                let face = ft::Face::new(&pathname, i);
                if let Ok(face) = face {
                    let ps_name2 = face.get_postscript_name();
                    match (&ps_name1, ps_name2) {
                        (None, None) => {
                            *index = i as u32;
                            break;
                        }
                        (Some(name1), Some(name2)) if &*name1.as_cstr() == name2 => {
                            *index = i as u32;
                            break;
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    if *index != 0xFFFFFFFF {
        Some(pathname)
    } else {
        None
    }
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub unsafe extern "C" fn getFileNameFromCTFont(
    ct_font: CTFontRef,
    index: *mut u32,
) -> *const libc::c_char {
    get_file_name_from_ct_font(
        &CTFont::new_borrowed(NonNull::new(ct_font.cast_mut()).unwrap()),
        &mut *index,
    )
    .map(CString::into_raw)
    .unwrap_or(ptr::null_mut())
}
