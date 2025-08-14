//! Font handling - specific fonts used by an engine while shaping text.

use crate::c_api::{Fixed, GlyphBBox, GlyphID, PlatformFontRef};
use crate::utils::fix_to_d;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex, OnceLock};
use tectonic_bridge_core::{CoreBridgeState, FileFormat};
#[cfg(not(target_os = "macos"))]
use tectonic_bridge_fontconfig as fc;
use tectonic_bridge_freetype2 as ft;
use tectonic_bridge_harfbuzz as hb;
#[cfg(target_os = "macos")]
use tectonic_mac_core::{
    CFArray, CFDictionary, CFType, CFUrl, CTFont, CTFontDescriptor, FontAttribute, FontNameKey,
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

fn get_font_funcs() -> hb::FontFuncsRef<'static, Arc<Mutex<ft::Face>>> {
    static FONTS: OnceLock<hb::ImmutFontFuncs<Arc<Mutex<ft::Face>>>> = OnceLock::new();

    FONTS
        .get_or_init(|| {
            let mut funcs = hb::FontFuncs::<Arc<Mutex<ft::Face>>>::new();

            let mut f = funcs.as_mut();
            f.nominal_glyph_func(|_, face, ch| {
                face.lock().unwrap().get_char_index(ch).map(|cc| cc.get())
            });
            f.variation_glyph_func(|_, face, ch, vs| {
                face.lock()
                    .unwrap()
                    .get_char_variant_index(ch, vs)
                    .map(|cc| cc.get())
            });
            f.glyph_h_advance(|_, face, gid| {
                get_glyph_advance(&face.lock().unwrap(), gid, false) as hb::Position
            });
            f.glyph_v_advance(|_, face, gid| {
                get_glyph_advance(&face.lock().unwrap(), gid, true) as hb::Position
            });
            f.glyph_h_origin(|_, _, _| Some((0, 0)));
            f.glyph_v_origin(|_, _, _| {
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
            f.glyph_h_kerning(|_, face, gid1, gid2| {
                match face
                    .lock()
                    .unwrap()
                    .get_kerning(gid1, gid2, ft::KerningMode::Unscaled)
                {
                    Ok(vec) => vec.x as hb::Position,
                    Err(_) => 0,
                }
            });
            f.glyph_v_kerning(|_, _, _, _| 0);
            f.glyph_extents(|_, face, gid| {
                let mut face = face.lock().unwrap();
                if let Ok(glyph) = face.load_glyph(gid, ft::LoadFlags::NO_SCALE) {
                    Some(hb::GlyphExtents {
                        x_bearing: glyph.metrics().horiBearingX as hb::Position,
                        y_bearing: glyph.metrics().horiBearingY as hb::Position,
                        width: glyph.metrics().width as hb::Position,
                        height: -glyph.metrics().height as hb::Position,
                    })
                } else {
                    None
                }
            });
            f.glyph_contour_point(|_, face, gid, point_index| {
                let mut face = face.lock().unwrap();

                if let Ok(glyph) = face.load_glyph(gid, ft::LoadFlags::NO_SCALE) {
                    if let Some(outline) = glyph.outline() {
                        if point_index < (outline.n_points as u32) {
                            let x = outline.points()[point_index as usize].x as hb::Position;
                            let y = outline.points()[point_index as usize].y as hb::Position;
                            return Some((x, y));
                        }
                    }
                }
                None
            });
            f.glyph_name(
                |_, face, gid, buf| match face.lock().unwrap().get_glyph_name(gid, buf) {
                    Ok(str) if !str.to_bytes().is_empty() && str.to_bytes()[0] == 0 => 0,
                    Err(_) => 0,
                    Ok(str) => str.to_bytes().len(),
                },
            );

            funcs.make_immutable()
        })
        .as_ref()
}

pub(crate) fn get_larger_script_list_table_ot(font: &Font) -> hb::ot::Table<'_> {
    let layout = font.hb_font().face().ot_layout();

    let sl_sub = layout.table(hb::GTag::GSub);
    let sl_pos = layout.table(hb::GTag::GPos);

    if sl_sub.script_tags_len() > sl_pos.script_tags_len() {
        sl_sub
    } else {
        sl_pos
    }
}

enum FontKind {
    FtFont,
    #[cfg(target_os = "macos")]
    Mac(CTFontDescriptor, Option<CTFont>),
}

/// A font, with all the information needed by TeX to shape it for typesetting.
pub struct Font {
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

    ft_face: Option<Arc<Mutex<ft::Face>>>,
    hb_font: Option<hb::Font>,

    // Currently only used on MacOS
    #[allow(dead_code)]
    kind: FontKind,
}

impl Font {
    #[cfg(not(target_os = "macos"))]
    pub(crate) fn new(font: PlatformFontRef, point_size: f32) -> Result<Font, ()> {
        let path = font
            .as_ref()
            .get::<fc::pat::File>(0)
            .and_then(|s| s.to_str().map_err(|_| fc::FcErr::NoMatch))
            .ok();
        let index = font.as_ref().get::<fc::pat::Index>(0).unwrap_or(0);

        Font::new_path_index(path, index as usize, point_size)
    }

    #[cfg(target_os = "macos")]
    pub(crate) fn new(descriptor: PlatformFontRef, point_size: f32) -> Result<Font, ()> {
        let mut out = Font {
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
        out.initialize_mac()?;
        Ok(out)
    }

    pub(crate) fn new_path_index(
        path: Option<&str>,
        index: usize,
        point_size: f32,
    ) -> Result<Font, ()> {
        let mut out = Font {
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
        if let Some(path) = path {
            out.initialize_ft(path, index)?;
        }
        Ok(out)
    }

    fn initialize_ft(&mut self, pathname: &str, index: usize) -> Result<(), ()> {
        CoreBridgeState::with_global_state(|engine| {
            let handle = engine
                .input_open(pathname, FileFormat::OpenType, false)
                .or_else(|| engine.input_open(pathname, FileFormat::TrueType, false))
                .or_else(|| engine.input_open(pathname, FileFormat::Type1, false));
            let Some(handle) = handle else {
                return Err(());
            };

            let sz = engine.input_get_size(handle);
            let mut backing_data = vec![0; sz];
            engine
                .input_read(handle, &mut backing_data)
                .expect("failed to read font file");

            engine.input_close(handle);

            self.ft_face = match ft::Face::new_memory(backing_data, index) {
                Ok(face) => Some(Arc::new(Mutex::new(face))),
                Err(_) => return Err(()),
            };

            if !self.ft_face().is_scalable() {
                return Err(());
            }

            if index == 0 && !self.ft_face().is_sfnt() {
                let afm = Path::new(pathname)
                    .file_name()
                    .map(Path::new)
                    .unwrap_or(Path::new(pathname));
                let afm = afm.with_extension("afm");

                let afm_handle = engine.input_open(afm.to_str().unwrap(), FileFormat::Afm, false);

                if let Some(afm_handle) = afm_handle {
                    let sz = engine.input_get_size(afm_handle);
                    let mut backing_data2 = vec![0; sz];
                    engine
                        .input_read(handle, &mut backing_data2)
                        .expect("failed to read AFM file");

                    self.ft_face().attach_stream_mem(backing_data2).unwrap();
                    engine.input_close(afm_handle);
                }
            }
            Ok(())
        })?;

        self.filename = CString::from_str(pathname).unwrap();
        self.index = index as u32;
        let upe = { self.ft_face().units_per_em() };
        self.units_per_em = upe;
        let a = { self.ft_face().ascender() } as f64;
        self.ascent = self.units_to_points(a) as f32;
        let d = { self.ft_face().descender() } as f64;
        self.descent = self.units_to_points(d) as f32;

        let ft_face = self.ft_face();
        let post_table = ft_face.get_sfnt_table::<ft::tables::Postscript>();
        let ia = if let Some(table) = post_table {
            fix_to_d(table.italic_angle as Fixed) as f32
        } else {
            self.italic_angle
        };
        drop(ft_face);
        self.italic_angle = ia;

        let ft_face = self.ft_face();
        let os2_table = ft_face.get_sfnt_table::<ft::tables::Os2>();
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

        let ft_face = Arc::clone(self.ft_face.as_ref().unwrap());
        let mut hb_face = hb::Face::new_tables(move |_, tag| {
            if let Ok(table) = ft_face
                .lock()
                .unwrap()
                .load_sfnt_table(ft::TableTag::Other(tag.to_raw()))
            {
                Some(hb::Blob::new(table))
            } else {
                None
            }
        });

        hb_face.as_mut().set_index(self.index);
        hb_face.as_mut().set_upem(self.units_per_em as u32);

        let mut hb_font = hb::Font::new(hb_face.as_ref());

        hb_font
            .as_mut()
            .set_funcs(get_font_funcs(), Arc::clone(self.ft_face.as_ref().unwrap()));
        hb_font
            .as_mut()
            .set_scale(self.units_per_em as i32, self.units_per_em as i32);
        hb_font.as_mut().set_ppem(0, 0);

        self.hb_font = Some(hb_font);

        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn initialize_mac(&mut self) -> Result<(), ()> {
        let FontKind::Mac(descriptor, font_ref) = &mut self.kind else {
            return Err(());
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
        self.initialize_ft(pathname.to_str().unwrap(), index as usize)
    }

    pub(crate) fn ft_face(&self) -> std::sync::MutexGuard<'_, ft::Face> {
        self.ft_face.as_ref().unwrap().lock().unwrap()
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

    pub(crate) fn first_char_code(&self) -> u32 {
        self.ft_face().get_first_char().0
    }

    pub(crate) fn last_char_code(&self) -> u32 {
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
        let mut ft_face = self.ft_face();

        let glyph = ft_face
            .load_glyph(gid as u32, ft::LoadFlags::NO_SCALE)
            .and_then(|slot| slot.get_glyph());

        match glyph {
            Ok(glyph) => {
                let ft::BBox {
                    x_min,
                    y_min,
                    x_max,
                    y_max,
                } = glyph.get_cbox(ft::BBoxMode::Unscaled);
                drop(ft_face);
                GlyphBBox {
                    x_min: self.units_to_points(x_min as f64) as f32,
                    y_min: self.units_to_points(y_min as f64) as f32,
                    x_max: self.units_to_points(x_max as f64) as f32,
                    y_max: self.units_to_points(y_max as f64) as f32,
                }
            }
            Err(_) => GlyphBBox::default(),
        }
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

    pub(crate) fn filename(&self, index: &mut u32) -> &CStr {
        *index = self.index;
        &self.filename
    }

    // pub(crate) fn get_font_table<T: ft::Table>(&self) -> Option<&T::Table> {
    //     self.ft_face().get_sfnt_table::<T>()
    // }

    pub(crate) fn italic_angle(&self) -> f32 {
        self.italic_angle
    }

    pub(crate) fn num_glyphs(&self) -> usize {
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

    pub(crate) fn hb_font(&self) -> hb::FontRef<'_> {
        self.hb_font.as_ref().unwrap().as_ref()
    }

    pub(crate) fn try_hb_font(&self) -> Option<hb::FontRef<'_>> {
        self.hb_font.as_ref().map(|f| f.as_ref())
    }

    /* Tectonic: these are modified from the base XeTeX code to use doubles;
     * otherwise roundoff errors can accumulate leading to differences in the
     * XDV outputs. */
    pub(crate) fn units_to_points(&self, units: f64) -> f64 {
        (units * self.point_size as f64) / (self.units_per_em as f64)
    }

    pub(crate) fn points_to_units(&self, points: f64) -> f64 {
        (points * (self.units_per_em as f64)) / self.point_size as f64
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn get_file_name_from_ct_font(ct_font: &CTFont, index: &mut u32) -> Option<CString> {
    let url = ct_font
        .attr(FontAttribute::URL)
        // SAFETY: CFUrl has no generic parameters
        .and_then(|t| unsafe { t.downcast::<CFUrl>() }.ok())?;

    let pathname = url.fs_representation()?;
    *index = 0;

    let face = ft::Face::new(&pathname, 0);
    if let Ok(face) = face {
        if face.num_faces() > 1 {
            let num_faces = face.num_faces();
            let ps_name1 = ct_font.name(FontNameKey::PostScript);
            *index = u32::MAX;
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

    if *index != u32::MAX {
        Some(pathname)
    } else {
        None
    }
}
