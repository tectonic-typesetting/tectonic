use crate::c_api::font::Font;
use crate::c_api::{
    d_to_fix, fix_to_d, raw_to_rs, Fixed, PlatformFontRef, RawPlatformFontRef, XeTeXFont,
};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::{ptr, slice};
use tectonic_bridge_freetype2 as ft;

#[cfg(not(target_os = "macos"))]
mod fc;
#[cfg(target_os = "macos")]
mod mac;

#[no_mangle]
pub extern "C" fn get_loaded_font_design_size() -> Fixed {
    FontManager::with_font_manager(|mgr| mgr.loaded_font_design_size)
}

#[no_mangle]
pub extern "C" fn set_loaded_font_design_size(val: Fixed) {
    FontManager::with_font_manager(|mgr| {
        mgr.loaded_font_design_size = val;
    });
}

thread_local! {
    static FONT_MGR: RefCell<Option<FontManager>> = const { RefCell::new(None) };
}

#[derive(Default)]
pub struct OpSizeRec {
    design_size: f64,
    min_size: f64,
    max_size: f64,
    sub_family_id: libc::c_uint,
    name_code: libc::c_uint,
}

#[allow(dead_code)]
pub struct FontInfo {
    full_name: Option<CString>,
    ps_name: CString,
    family_name: CString,
    style_name: CString,
    parent: usize,
    font_ref: PlatformFontRef,
    op_size_info: OpSizeRec,
    weight: u16,
    width: u16,
    slant: i16,
    is_reg: bool,
    is_bold: bool,
    is_italic: bool,
}

impl FontInfo {
    fn new(
        font_ref: PlatformFontRef,
        ps_name: CString,
        family_name: CString,
        style_name: CString,
    ) -> FontInfo {
        let mut out = FontInfo {
            full_name: None,
            ps_name,
            family_name,
            style_name,
            parent: usize::MAX,
            font_ref,
            op_size_info: OpSizeRec::default(),
            weight: 0,
            width: 0,
            slant: 0,
            is_reg: false,
            is_bold: false,
            is_italic: false,
        };
        out.op_size_info.sub_family_id = 0;
        out.op_size_info.design_size = 10.0; /* default to 10.0pt */
        out
    }
}

#[derive(Default)]
pub struct FamilyInfo {
    styles: HashMap<CString, usize>,
    min_weight: u16,
    max_weight: u16,
    min_width: u16,
    max_width: u16,
    min_slant: i16,
    max_slant: i16,
}

impl FamilyInfo {
    fn new() -> FamilyInfo {
        FamilyInfo::default()
    }
}

#[derive(Default)]
pub struct NameCollection {
    family_names: Vec<CString>,
    style_names: Vec<CString>,
    full_names: Vec<CString>,
    ps_name: Option<CString>,
}

pub trait FontManagerBackend {
    fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> Cow<'a, CStr>;
    fn get_op_size_rec_and_style_flags(&self, font: &mut FontInfo);
    fn search_for_host_platform_fonts(&mut self, maps: &mut FontMaps, name: &CStr);
    fn read_names(&self, font: PlatformFontRef) -> NameCollection;
}

fn base_get_op_size_rec_and_style_flags(font: &mut FontInfo) {
    let xfont = match Font::new(font.font_ref.clone(), 10.0) {
        Ok(xfont) => xfont,
        Err(_) => return,
    };

    let size_rec = FontManager::get_op_size(&xfont);
    if let Some(size_rec) = size_rec {
        font.op_size_info.design_size = size_rec.design_size;
        if size_rec.sub_family_id != 0
            || size_rec.name_code != 0
            || size_rec.min_size != 0.0
            || size_rec.max_size != 0.0
        {
            font.op_size_info.sub_family_id = size_rec.sub_family_id;
            font.op_size_info.name_code = size_rec.name_code;
            font.op_size_info.min_size = size_rec.min_size;
            font.op_size_info.max_size = size_rec.max_size;
        }
    }

    let ft_face = xfont.ft_face();
    let os2_table = ft_face.get_sfnt_table::<ft::Os2>();
    if let Some(table) = os2_table {
        font.weight = table.usWeightClass;
        font.width = table.usWidthClass;
        let sel = table.fsSelection;
        font.is_reg = (sel & (1 << 6)) != 0;
        font.is_bold = (sel & (1 << 5)) != 0;
        font.is_italic = (sel & (1 << 0)) != 0;
    }

    let head_table = ft_face.get_sfnt_table::<ft::Header>();
    if let Some(table) = head_table {
        let ms = table.Mac_Style;
        if (ms & (1 << 0)) != 0 {
            font.is_bold = true;
        }
        if (ms & (1 << 1)) != 0 {
            font.is_italic = true;
        }
    }

    let post_table = ft_face.get_sfnt_table::<ft::Postscript>();
    if let Some(table) = post_table {
        font.slant = (1000.0
            * (f64::tan(fix_to_d((-table.italic_angle) as Fixed) * std::f64::consts::PI / 180.0)))
            as _;
    }
}

#[derive(Default)]
pub struct FontMaps {
    fonts: Vec<FontInfo>,
    families: Vec<FamilyInfo>,

    name_to_font: HashMap<CString, usize>,
    name_to_family: HashMap<CString, usize>,
    platform_ref_to_font: HashMap<PlatformFontRef, usize>,
    ps_name_to_font: HashMap<CString, usize>,
}

impl FontMaps {
    pub fn add_to_maps(
        &mut self,
        backend: &dyn FontManagerBackend,
        pfont: PlatformFontRef,
        names: &NameCollection,
    ) {
        if self.platform_ref_to_font.contains_key(&pfont) {
            // this font has already been cached
            return;
        }
        let ps_name = match &names.ps_name {
            Some(name) => name,
            // can't use a font that lacks a PostScript name
            None => return,
        };
        if self.ps_name_to_font.contains_key(ps_name) {
            // duplicates an earlier PS name, so skip
            return;
        }

        let family_name = if !names.family_names.is_empty() {
            names.family_names[0].clone()
        } else {
            ps_name.to_owned()
        };

        let style_name = if !names.style_names.is_empty() {
            names.style_names[0].clone()
        } else {
            CString::default()
        };

        let mut font = FontInfo::new(pfont.clone(), ps_name.to_owned(), family_name, style_name);
        backend.get_op_size_rec_and_style_flags(&mut font);
        self.fonts.push(font);
        let font_pos = self.fonts.len() - 1;
        let font = &mut self.fonts[font_pos];
        self.ps_name_to_font.insert(font.ps_name.clone(), font_pos);
        self.platform_ref_to_font.insert(pfont, font_pos);

        if !names.full_names.is_empty() {
            font.full_name = Some(names.full_names[0].clone());
        }

        for i in &names.family_names {
            let fam = self.name_to_family.get(i).copied();
            let (family, fam_pos) = match fam {
                None => {
                    let mut family = FamilyInfo::new();
                    family.min_weight = font.weight;
                    family.max_weight = font.weight;
                    family.min_width = font.width;
                    family.max_width = font.width;
                    family.min_slant = font.slant;
                    family.max_slant = font.slant;
                    self.families.push(family);
                    let fam_pos = self.families.len() - 1;
                    let family = &mut self.families[fam_pos];
                    self.name_to_family.insert(i.clone(), fam_pos);
                    (family, fam_pos)
                }
                Some(fam_pos) => {
                    let family = &mut self.families[fam_pos];
                    family.min_weight = u16::min(family.min_weight, font.weight);
                    family.max_weight = u16::max(family.max_weight, font.weight);
                    family.min_width = u16::min(family.min_width, font.width);
                    family.max_width = u16::max(family.max_width, font.width);
                    family.min_slant = i16::min(family.min_slant, font.slant);
                    family.max_slant = i16::max(family.max_slant, font.slant);
                    (family, fam_pos)
                }
            };

            if font.parent == usize::MAX {
                font.parent = fam_pos;
            }

            for style in &names.style_names {
                let f = family.styles.get(style);
                if f.is_none() {
                    family.styles.insert(style.clone(), font_pos);
                }
                /*
                else if (iFont->second != thisFont)
                    fprintf(stderr, "# Font name warning: ambiguous Style \"%s\" in Family \"%s\" (PSNames \"%s\" and \"%s\")\n",
                            j->c_str(), i->c_str(), iFont->second->m_psName->c_str(), thisFont->m_psName->c_str());
                */
            }
        }

        for i in &names.full_names {
            let f = self.name_to_font.get(i);
            if f.is_none() {
                self.name_to_font.insert(i.clone(), font_pos);
            }
            /*
            else if (iFont->second != thisFont)
                fprintf(stderr, "# Font name warning: ambiguous FullName \"%s\" (PSNames \"%s\" and \"%s\")\n",
                        i->c_str(), iFont->second->m_psName->c_str(), thisFont->m_psName->c_str());
            */
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Engine {
    Default = 0,
    Apple = b'A',
    OpenType = b'O',
    Graphite = b'G',
}

impl TryFrom<u8> for Engine {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Engine::Default,
            b'A' => Engine::Apple,
            b'O' => Engine::OpenType,
            b'G' => Engine::Graphite,
            _ => return Err(()),
        })
    }
}

pub struct FontManager {
    backend: Box<dyn FontManagerBackend>,
    maps: FontMaps,
    req_engine: Engine,
    loaded_font_design_size: Fixed,
}

impl FontManager {
    fn init_font_manager() {
        let backend: Box<dyn FontManagerBackend>;

        #[cfg(target_os = "macos")]
        {
            backend = Box::new(mac::MacBackend::new());
        }
        #[cfg(not(target_os = "macos"))]
        {
            backend = Box::new(fc::FcBackend::new());
        }

        FONT_MGR.with_borrow_mut(|mgr| {
            *mgr = Some(FontManager {
                backend,
                maps: Default::default(),
                req_engine: Engine::Default,
                loaded_font_design_size: 0,
            })
        });
    }

    pub fn with_font_manager<T>(f: impl FnOnce(&mut FontManager) -> T) -> T {
        let init = FONT_MGR.with_borrow(|mgr| mgr.is_none());
        if init {
            Self::init_font_manager();
        }
        FONT_MGR.with_borrow_mut(|mgr| f(mgr.as_mut().unwrap()))
    }

    pub fn destroy() {
        FONT_MGR.with_borrow_mut(|mgr| {
            *mgr = None;
        })
    }

    pub fn find_font(
        &mut self,
        name: &CStr,
        variant: Option<&mut [u8]>,
        mut pt_size: f64,
    ) -> Option<PlatformFontRef> {
        let mut font = None;
        let mut dsize = 10.0;
        self.loaded_font_design_size = 655360;

        for pass in 0..2 {
            // try full name as given
            if let Some(&font_pos) = self.maps.name_to_font.get(name) {
                let temp_font = &self.maps.fonts[font_pos];
                font = Some(font_pos);
                if temp_font.op_size_info.design_size != 0.0 {
                    dsize = temp_font.op_size_info.design_size
                }
                break;
            }

            let bytes = name.to_bytes();
            let split = bytes
                .iter()
                .position(|c| *c == b'-')
                .map(|index| (&bytes[..index], &bytes[index + 1..]));

            // if there's a hyphen, split there and try Family-Style
            if let Some((family, style)) = split {
                let family = CString::new(family).unwrap();
                if let Some(&fam_pos) = self.maps.name_to_family.get(&family) {
                    let style = CString::new(style).unwrap();
                    let temp_fam = &self.maps.families[fam_pos];
                    if let Some(&font_pos) = temp_fam.styles.get(&style) {
                        let temp_font = &self.maps.fonts[font_pos];
                        font = Some(font_pos);
                        if temp_font.op_size_info.design_size != 0.0 {
                            dsize = temp_font.op_size_info.design_size;
                        }
                        break;
                    }
                }
            }

            // try as PostScript name
            if let Some(&font_pos) = self.maps.ps_name_to_font.get(name) {
                let temp_font = &self.maps.fonts[font_pos];
                font = Some(font_pos);
                if temp_font.op_size_info.design_size != 0.0 {
                    dsize = temp_font.op_size_info.design_size;
                }
                break;
            }

            // try for the name as a family name
            if let Some(&fam_pos) = self.maps.name_to_family.get(name) {
                let family = &self.maps.families[fam_pos];
                // look for a family member with the "regular" bit set in OS/2
                let mut reg_fonts = 0;
                for &font_pos in family.styles.values() {
                    let temp_font = &self.maps.fonts[font_pos];
                    if temp_font.is_reg {
                        if reg_fonts == 0 {
                            font = Some(font_pos);
                        }
                        reg_fonts += 1;
                    }
                }

                // families with Ornament or similar fonts may flag those as Regular,
                // which confuses the search above... so try some known names
                if font.is_none() || reg_fonts > 1 {
                    // try for style "Regular", "Plain", "Normal", "Roman"
                    for name in [
                        cstr!("Regular"),
                        cstr!("Plain"),
                        cstr!("Normal"),
                        cstr!("Roman"),
                    ] {
                        if let Some(&font_pos) = family.styles.get(name) {
                            font = Some(font_pos);
                            break;
                        }
                    }
                }

                if font.is_none() {
                    // look through the family for the (weight, width, slant) nearest to (80, 100, 0)
                    font = self.best_match_from_family(family, 80, 100, 0);
                }

                if font.is_some() {
                    break;
                }
            }

            if pass == 0 {
                self.search_for_host_platform_fonts(name);
            }
        }

        let mut font_pos = font?;

        let parent_pos = self.maps.fonts[font_pos].parent;

        // if there are variant requests, try to apply them
        // and delete B, I, and S=... codes from the string, just retain /engine option
        self.req_engine = Engine::Default;
        let mut req_bold = false;
        let mut req_ital = false;
        if let Some(variant) = variant {
            let mut var_str = String::new();
            let mut cp = &*variant;
            while !cp.is_empty() {
                const VARIANTS: &[(&[u8], Engine, &str)] = &[
                    (b"AAT", Engine::Apple, "AAT"),
                    (b"ICU", Engine::OpenType, "OT"),
                    (b"OT", Engine::OpenType, "OT"),
                    (b"GR", Engine::Graphite, "GR"),
                ];

                let any_var = VARIANTS.iter().any(|&(cmp, engine, var)| {
                    if cp.starts_with(cmp) {
                        self.req_engine = engine;
                        cp = &cp[cmp.len()..];
                        if var_str.chars().last().is_some_and(|c| c != '/') {
                            var_str.push_str(var);
                        }
                        true
                    } else {
                        false
                    }
                });

                if any_var {
                } else if cp.starts_with(b"S") {
                    cp = &cp[1..];
                    if cp.first() == Some(&b'=') {
                        cp = &cp[1..];
                    }
                    pt_size = 0.0;
                    while cp.first().is_some_and(|c| c.is_ascii_digit()) {
                        pt_size = pt_size * 10.0 + (cp[0] - b'0') as f64;
                        cp = &cp[1..];
                    }
                    if cp.first() == Some(&b'.') {
                        let mut dec = 1.0;
                        cp = &cp[1..];
                        while cp.first().is_some_and(|c| c.is_ascii_digit()) {
                            dec *= 10.0;
                            pt_size += (cp[0] - b'0') as f64 / dec;
                            cp = &cp[1..];
                        }
                    }
                } else {
                    // if the code is "B" or "I", we skip putting it in varString
                    loop {
                        if cp[0] == b'B' {
                            req_bold = true;
                            cp = &cp[1..];
                        } else if cp[0] == b'I' {
                            req_ital = true;
                            cp = &cp[1..];
                        } else {
                            break;
                        }
                    }
                }

                while cp.first().is_some_and(|&c| c != b'/') {
                    cp = &cp[1..];
                }
                if cp.first().is_some_and(|&c| c == b'/') {
                    cp = &cp[1..];
                }
            }

            variant[..var_str.len()].copy_from_slice(var_str.as_bytes());
            variant[var_str.len()] = 0;

            if req_ital {
                let font = &self.maps.fonts[font_pos];
                let parent = &self.maps.families[parent_pos];
                let mut best_match_pos = font_pos;
                if font.slant < parent.max_slant {
                    // try for a face with more slant
                    best_match_pos = self
                        .best_match_from_family(
                            parent,
                            font.weight as libc::c_int,
                            font.width as libc::c_int,
                            parent.max_slant as libc::c_int,
                        )
                        .unwrap_or(best_match_pos);
                }

                if best_match_pos == font_pos && font.slant > parent.min_slant {
                    // maybe the slant is negated, or maybe this was something like "Times-Italic/I"
                    best_match_pos = self
                        .best_match_from_family(
                            parent,
                            font.weight as libc::c_int,
                            font.width as libc::c_int,
                            parent.min_slant as libc::c_int,
                        )
                        .unwrap_or(best_match_pos);
                }

                let best_match = &self.maps.fonts[best_match_pos];
                if parent.min_weight == parent.max_weight && best_match.is_bold != font.is_bold {
                    // try again using the bold flag, as we can't trust weight values
                    let mut new_best = None;
                    for &style_pos in parent.styles.values() {
                        let style = &self.maps.fonts[style_pos];
                        if style.is_bold == font.is_bold
                            && new_best.is_none()
                            && style.is_italic != font.is_italic
                        {
                            new_best = Some(style_pos);
                            break;
                        }
                    }
                    if let Some(new_best) = new_best {
                        best_match_pos = new_best;
                    }
                }

                if best_match_pos == font_pos {
                    let mut new_best = None;
                    // maybe slant values weren't present; try the style bits as a fallback
                    for &style_pos in parent.styles.values() {
                        let style = &self.maps.fonts[style_pos];
                        if style.is_italic != font.is_italic {
                            if parent.min_weight != parent.max_weight {
                                // weight info was available, so try to match that
                                if new_best.is_none()
                                    || Self::weight_and_width_diff(style, font)
                                        < Self::weight_and_width_diff(
                                            &self.maps.fonts[new_best.unwrap()],
                                            font,
                                        )
                                {
                                    new_best = Some(style_pos);
                                }
                            } else {
                                // no weight info, so try matching style bits
                                if new_best.is_none() && style.is_bold == font.is_bold {
                                    new_best = Some(style_pos);
                                    break; // found a match, no need to look further as we can't distinguish!
                                }
                            }
                        }
                    }

                    if let Some(new_best) = new_best {
                        best_match_pos = new_best;
                    }
                }

                font_pos = best_match_pos;
            }

            if req_bold {
                let font = &self.maps.fonts[font_pos];
                let parent = &self.maps.families[parent_pos];
                let mut best_match = font_pos;
                if font.weight < parent.max_weight {
                    best_match = self
                        .best_match_from_family(
                            parent,
                            (font.weight + (parent.max_weight - (parent.min_weight)) / 2 + 1)
                                as libc::c_int,
                            font.width as libc::c_int,
                            font.slant as libc::c_int,
                        )
                        .unwrap_or(best_match);
                    if parent.min_slant == parent.max_slant {
                        let mut new_best = None;
                        for &style_pos in parent.styles.values() {
                            let style = &self.maps.fonts[style_pos];
                            if style.is_italic == font.is_italic
                                && (new_best.is_none()
                                    || Self::weight_and_width_diff(
                                        style,
                                        &self.maps.fonts[best_match],
                                    ) < Self::weight_and_width_diff(
                                        &self.maps.fonts[new_best.unwrap()],
                                        &self.maps.fonts[best_match],
                                    ))
                            {
                                new_best = Some(style_pos);
                            }
                        }
                        if let Some(new_best) = new_best {
                            best_match = new_best;
                        }
                    }
                }
                if best_match == font_pos && font.is_bold {
                    for &style_pos in parent.styles.values() {
                        let style = &self.maps.fonts[style_pos];
                        if style.is_italic == font.is_italic && style.is_bold {
                            best_match = style_pos;
                            break;
                        }
                    }
                }
                font_pos = best_match;
            }
        }

        if pt_size < 0.0 {
            pt_size = dsize;
        }

        let font = &self.maps.fonts[font_pos];
        let parent = &self.maps.families[parent_pos];
        if font.op_size_info.sub_family_id != 0 && pt_size > 0.0 {
            let mut best_mismatch = f64::max(
                font.op_size_info.min_size - pt_size,
                pt_size - font.op_size_info.max_size,
            );
            if best_mismatch > 0.0 {
                let mut best_match = font_pos;
                for &style_pos in parent.styles.values() {
                    let style = &self.maps.fonts[style_pos];
                    if style.op_size_info.sub_family_id != font.op_size_info.sub_family_id {
                        continue;
                    }
                    let mismatch = f64::max(
                        style.op_size_info.min_size - pt_size,
                        pt_size - style.op_size_info.max_size,
                    );
                    if mismatch < best_mismatch {
                        best_match = style_pos;
                        best_mismatch = mismatch;
                    }
                    if best_mismatch <= 0.0 {
                        break;
                    }
                }
                font_pos = best_match;
            }
        }

        let font = &self.maps.fonts[font_pos];
        if font.op_size_info.design_size != 0.0 {
            self.loaded_font_design_size = d_to_fix(font.op_size_info.design_size);
        }

        Some(font.font_ref.clone())
    }

    pub fn get_full_name(&self, font: PlatformFontRef) -> *const libc::c_char {
        let font_pos = *self
            .maps
            .platform_ref_to_font
            .get(&font)
            .unwrap_or_else(|| panic!("internal error {} in FontManager", 2));
        let font = &self.maps.fonts[font_pos];
        let name = font.full_name.as_ref().unwrap_or(&font.ps_name);
        name.as_ptr()
    }

    pub fn get_design_size(&self, font: &Font) -> f64 {
        let size_rec = Self::get_op_size(font);
        match size_rec {
            None => 10.0,
            Some(size_rec) => size_rec.design_size,
        }
    }

    pub fn weight_and_width_diff(a: &FontInfo, b: &FontInfo) -> libc::c_int {
        if a.weight == 0 && a.width == 0 {
            return if a.is_bold == b.is_bold { 0 } else { 10000 };
        }

        let mut wid_diff = u16::abs_diff(a.width, b.width);
        if wid_diff < 10 {
            wid_diff *= 50;
        }

        (u16::abs_diff(a.weight, b.weight) + wid_diff) as libc::c_int
    }

    pub fn style_diff(
        a: &FontInfo,
        wt: libc::c_int,
        wd: libc::c_int,
        slant: libc::c_int,
    ) -> libc::c_int {
        let mut wid_diff = u16::abs_diff(a.width, wd as u16);
        if wid_diff < 10 {
            wid_diff *= 200;
        }

        (u32::abs_diff(a.slant.unsigned_abs() as u32, slant.unsigned_abs()) * 2
            + u32::abs_diff(a.weight as u32, wt.unsigned_abs())
            + wid_diff as u32) as libc::c_int
    }

    pub fn best_match_from_family(
        &self,
        family: &FamilyInfo,
        wt: libc::c_int,
        wd: libc::c_int,
        slant: libc::c_int,
    ) -> Option<usize> {
        let mut best_match = None;
        for &font_pos in family.styles.values() {
            let font = &self.maps.fonts[font_pos];
            best_match = match best_match {
                None => Some(font_pos),
                Some(best_pos) => {
                    let best = &self.maps.fonts[best_pos];
                    if Self::style_diff(font, wt, wd, slant) < Self::style_diff(best, wt, wd, slant)
                    {
                        Some(font_pos)
                    } else {
                        Some(best_pos)
                    }
                }
            };
        }
        best_match
    }

    pub fn append_to_list<T: Into<CString> + AsRef<CStr>>(list: &mut Vec<CString>, str: T) {
        if !list.iter().any(|s| **s == *str.as_ref()) {
            list.push(str.into())
        }
    }

    pub fn prepend_to_list<T: Into<CString> + AsRef<CStr>>(list: &mut Vec<CString>, str: T) {
        *list = list.drain(..).filter(|s| **s != *str.as_ref()).collect();
        list.insert(0, str.into());
    }

    pub fn get_op_size(font: &Font) -> Option<OpSizeRec> {
        let hb_font = font.try_hb_font()?;

        hb_font
            .face()
            .ot_layout()
            .size_params()
            .map(|params| OpSizeRec {
                sub_family_id: params.subfamily_id,
                name_code: params.subfamily_name_id,
                design_size: params.design_size as f64 * 72.27 / 72.0 / 10.0,
                min_size: params.start as f64 * 72.27 / 72.0 / 10.0,

                max_size: params.end as f64 * 72.27 / 72.0 / 10.0,
            })
    }

    pub fn search_for_host_platform_fonts(&mut self, name: &CStr) {
        self.backend
            .search_for_host_platform_fonts(&mut self.maps, name)
    }

    pub fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> Cow<'a, CStr> {
        self.backend.get_platform_font_desc(font)
    }

    pub fn get_req_engine(&self) -> Engine {
        self.req_engine
    }

    pub fn set_req_engine(&mut self, engine: Engine) {
        self.req_engine = engine;
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_font_manager() {
    FontManager::destroy();
}

#[no_mangle]
pub unsafe extern "C" fn findFontByName(
    name: *const libc::c_char,
    var: *mut libc::c_char,
    size: f64,
) -> RawPlatformFontRef {
    let name = CStr::from_ptr(name);
    let var = if var.is_null() {
        None
    } else {
        let len = CStr::from_ptr(var).to_bytes().len();
        Some(slice::from_raw_parts_mut(var.cast(), len))
    };

    #[cfg(target_os = "macos")]
    return FontManager::with_font_manager(|mgr| {
        use tectonic_mac_core::CoreType;
        mgr.find_font(name, var, size)
            .map(tectonic_mac_core::CTFontDescriptor::into_type_ref)
            .unwrap_or(ptr::null_mut())
    });
    #[cfg(not(target_os = "macos"))]
    FontManager::with_font_manager(|mgr| {
        mgr.find_font(name, var, size)
            .map(|pat| pat.as_ref().as_ptr())
            .unwrap_or(ptr::null_mut())
    })
}

#[no_mangle]
pub extern "C" fn getReqEngine() -> libc::c_char {
    FontManager::with_font_manager(|mgr| mgr.get_req_engine() as libc::c_char)
}

#[no_mangle]
pub extern "C" fn setReqEngine(engine: libc::c_char) {
    FontManager::with_font_manager(|mgr| {
        mgr.set_req_engine(Engine::try_from(engine as u8).unwrap())
    })
}

#[no_mangle]
pub unsafe extern "C" fn getFullName(font: RawPlatformFontRef) -> *const libc::c_char {
    match raw_to_rs(font) {
        Some(font) => FontManager::with_font_manager(|mgr| mgr.get_full_name(font)),
        None => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn getDesignSize(font: XeTeXFont) -> f64 {
    FontManager::with_font_manager(|mgr| mgr.get_design_size(&*font))
}

#[no_mangle]
pub unsafe extern "C" fn ttxl_platfont_get_desc(font: RawPlatformFontRef) -> *const libc::c_char {
    match raw_to_rs(font) {
        Some(font) => {
            FontManager::with_font_manager(|mgr| mgr.get_platform_font_desc(&font).as_ptr())
        }
        None => ptr::null_mut(),
    }
}
