use crate::c_api::{Fixed, PlatformFontRef, RawPlatformFontRef, XeTeXFont};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;
use tectonic_bridge_harfbuzz::{hb_font_get_face, hb_ot_layout_get_size_params};

#[cfg(not(target_os = "macos"))]
mod fc;
#[cfg(target_os = "macos")]
mod mac;

thread_local! {
    static LOADED_FONT_DESIGN_SIZE: Cell<Fixed> = Cell::new(0);
}

#[no_mangle]
pub extern "C" fn get_loaded_font_design_size() -> Fixed {
    LOADED_FONT_DESIGN_SIZE.get()
}

#[no_mangle]
pub extern "C" fn set_loaded_font_design_size(val: Fixed) {
    LOADED_FONT_DESIGN_SIZE.set(val);
}

thread_local! {
    static FONT_MGR: RefCell<Option<FontManager>> = const { RefCell::new(None) };
    static REQ_ENGINE: Cell<libc::c_char> = const { Cell::new(0) };
}

#[derive(Default)]
pub struct OpSizeRec {
    design_size: f64,
    min_size: f64,
    max_size: f64,
    sub_family_id: libc::c_uint,
    name_code: libc::c_uint,
}

pub struct Font {
    full_name: Option<CString>,
    ps_name: CString,
    family_name: Option<CString>,
    style_name: Option<CString>,
    parent: *mut Family,
    font_ref: PlatformFontRef,
    op_size_info: OpSizeRec,
    weight: u16,
    width: u16,
    slant: i16,
    is_reg: bool,
    is_bold: bool,
    is_italic: bool,
}

impl Font {
    fn new(font_ref: PlatformFontRef, ps_name: CString) -> Font {
        let mut out = Font {
            full_name: None,
            ps_name,
            family_name: None,
            style_name: None,
            parent: ptr::null_mut(),
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
pub struct Family {
    styles: HashMap<CString, *mut Font>,
    min_weight: u16,
    max_weight: u16,
    min_width: u16,
    max_width: u16,
    min_slant: i16,
    max_slant: i16,
}

impl Family {
    fn new() -> Family {
        Family::default()
    }
}

#[derive(Default)]
pub struct NameCollection {
    family_names: Vec<CString>,
    style_names: Vec<CString>,
    full_names: Vec<CString>,
    // TODO: Lifetime is a lie, it's actually that of the source font
    ps_name: Option<&'static CStr>,
}

pub trait FontManagerBackend {
    unsafe fn initialize(&mut self);
    unsafe fn terminate(&mut self);
    unsafe fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> &'a CStr;
    unsafe fn get_op_size_rec_and_style_flags(&self, font: &mut Font);
    unsafe fn search_for_host_platform_fonts(&mut self, maps: &mut FontMaps, name: &CStr);
    unsafe fn read_names(&self, font: PlatformFontRef) -> NameCollection;
}

#[derive(Default)]
pub struct FontMaps {
    name_to_font: HashMap<CString, *mut Font>,
    name_to_family: HashMap<CString, *mut Family>,
    platform_ref_to_font: HashMap<PlatformFontRef, *mut Font>,
    ps_name_to_font: HashMap<CString, *mut Font>,
}

impl FontMaps {
    pub unsafe fn add_to_maps(
        &mut self,
        backend: &dyn FontManagerBackend,
        pfont: PlatformFontRef,
        names: &NameCollection,
    ) {
        if self.platform_ref_to_font.contains_key(&pfont) {
            // this font has already been cached
            return;
        }
        let ps_name = match names.ps_name {
            Some(name) => name,
            // can't use a font that lacks a PostScript name
            None => return,
        };
        if self.ps_name_to_font.contains_key(ps_name) {
            // duplicates an earlier PS name, so skip
            return;
        }

        let font = Box::leak(Box::new(Font::new(pfont.clone(), ps_name.to_owned())));
        backend.get_op_size_rec_and_style_flags(font);
        self.ps_name_to_font.insert(font.ps_name.clone(), font);
        self.platform_ref_to_font.insert(pfont, font);

        if !names.full_names.is_empty() {
            font.full_name = Some(names.full_names[0].clone());
        }
        // TODO: make family_name not an option
        if !names.family_names.is_empty() {
            font.family_name = Some(names.family_names[0].clone());
        } else {
            font.family_name = Some(ps_name.to_owned());
        }

        // TODO make style_name not an option
        if !names.style_names.is_empty() {
            font.style_name = Some(names.style_names[0].clone());
        } else {
            font.style_name = Some(CString::default());
        }

        for i in &names.family_names {
            let fam = self.name_to_family.get(i).copied();
            let family = match fam {
                None => {
                    let family = Box::leak(Box::new(Family::new()));
                    family.min_weight = font.weight;
                    family.max_weight = font.weight;
                    family.min_width = font.width;
                    family.max_width = font.width;
                    family.min_slant = font.slant;
                    family.max_slant = font.slant;
                    self.name_to_family.insert(i.clone(), family);
                    family
                }
                Some(fam) => {
                    let family = &mut *fam;
                    family.min_weight = u16::min(family.min_weight, font.weight);
                    family.max_weight = u16::max(family.max_weight, font.weight);
                    family.min_width = u16::min(family.min_width, font.width);
                    family.max_width = u16::max(family.max_width, font.width);
                    family.min_slant = i16::min(family.min_slant, font.slant);
                    family.max_slant = i16::max(family.max_slant, font.slant);
                    family
                }
            };

            if font.parent.is_null() {
                font.parent = family;
            }

            for style in &names.style_names {
                let f = family.styles.get(style);
                if f.is_none() {
                    family.styles.insert(style.clone(), font);
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
                self.name_to_font.insert(i.clone(), font);
            }
            /*
            else if (iFont->second != thisFont)
                fprintf(stderr, "# Font name warning: ambiguous FullName \"%s\" (PSNames \"%s\" and \"%s\")\n",
                        i->c_str(), iFont->second->m_psName->c_str(), thisFont->m_psName->c_str());
            */
        }
    }
}

pub struct FontManager {
    backend: Box<dyn FontManagerBackend>,
    maps: FontMaps,
}

impl FontManager {
    fn init_font_manager() {
        let mut backend: Box<dyn FontManagerBackend>;

        #[cfg(target_os = "macos")]
        {
            backend = Box::new(mac::MacBackend::new());
        }
        #[cfg(not(target_os = "macos"))]
        {
            backend = Box::new(fc::FcBackend::new());
        }

        unsafe { backend.initialize() };

        FONT_MGR.with_borrow_mut(|mgr| {
            *mgr = Some(FontManager {
                backend,
                maps: Default::default(),
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

    pub fn terminate() {
        FONT_MGR.with_borrow_mut(|mgr| {
            if let Some(mgr) = mgr {
                unsafe { mgr.backend.terminate() };
            }
        })
    }

    pub fn destroy() {
        FONT_MGR.with_borrow_mut(|mgr| {
            *mgr = None;
        })
    }

    pub unsafe fn find_font(
        &mut self,
        name: *const libc::c_char,
        variant: *mut libc::c_char,
        mut pt_size: f64,
    ) -> Option<PlatformFontRef> {
        let name_str = CStr::from_ptr(name);
        let mut font = ptr::null_mut();
        let mut dsize = 10.0;
        LOADED_FONT_DESIGN_SIZE.set(655360);

        for pass in 0..2 {
            // try full name as given
            if let Some(&found) = self.maps.name_to_font.get(name_str) {
                font = found;
                if (*font).op_size_info.design_size != 0.0 {
                    dsize = (*font).op_size_info.design_size
                }
                break;
            }

            let bytes = name_str.to_bytes();
            let split = bytes
                .iter()
                .position(|c| *c == b'-')
                .map(|index| (&bytes[..index], &bytes[index + 1..]));

            // if there's a hyphen, split there and try Family-Style
            if let Some((family, style)) = split {
                let family = CString::new(family).unwrap();
                if let Some(&found_fam) = self.maps.name_to_family.get(&family) {
                    let style = CString::new(style).unwrap();
                    if let Some(&found) = (*found_fam).styles.get(&style) {
                        font = found;
                        if (*font).op_size_info.design_size != 0.0 {
                            dsize = (*font).op_size_info.design_size;
                        }
                        break;
                    }
                }
            }

            // try as PostScript name
            if let Some(&found) = self.maps.ps_name_to_font.get(name_str) {
                font = found;
                if (*font).op_size_info.design_size != 0.0 {
                    dsize = (*font).op_size_info.design_size;
                }
                break;
            }

            // try for the name as a family name
            if let Some(&found_fam) = self.maps.name_to_family.get(name_str) {
                // look for a family member with the "regular" bit set in OS/2
                let mut reg_fonts = 0;
                for (_, &found) in &(*found_fam).styles {
                    if (*found).is_reg {
                        if reg_fonts == 0 {
                            font = found;
                        }
                        reg_fonts += 1;
                    }
                }

                // families with Ornament or similar fonts may flag those as Regular,
                // which confuses the search above... so try some known names
                if font.is_null() || reg_fonts > 1 {
                    // try for style "Regular", "Plain", "Normal", "Roman"
                    for name in [c!("Regular"), c!("Plain"), c!("Normal"), c!("Roman")] {
                        if let Some(&found) = (*found_fam).styles.get(CStr::from_ptr(name)) {
                            font = found;
                            break;
                        }
                    }
                }

                if font.is_null() {
                    // look through the family for the (weight, width, slant) nearest to (80, 100, 0)
                    font = Self::best_match_from_family(&*found_fam, 80, 100, 0);
                }

                if !font.is_null() {
                    break;
                }
            }

            if pass == 0 {
                self.search_for_host_platform_fonts(name_str)
            }
        }

        if font.is_null() {
            return None;
        }

        let parent = (*font).parent;

        // if there are variant requests, try to apply them
        // and delete B, I, and S=... codes from the string, just retain /engine option
        REQ_ENGINE.set(0);
        let mut req_bold = false;
        let mut req_ital = false;
        if !variant.is_null() {
            let mut var_str = String::new();
            let mut cp = CStr::from_ptr(variant).to_bytes();
            while !cp.is_empty() {
                const VARIANTS: &[(&[u8], u8, &str)] = &[
                    (b"AAT", b'A', "AAT"),
                    (b"ICU", b'O', "OT"),
                    (b"OT", b'O', "OT"),
                    (b"GR", b'G', "GR"),
                ];

                let any_var = VARIANTS.iter().any(|&(cmp, engine, var)| {
                    if cp.starts_with(b"AAT") {
                        REQ_ENGINE.set(engine as libc::c_char);
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
                    while cp.first().is_some_and(|&c| c >= b'0' && c <= b'9') {
                        pt_size = pt_size * 10.0 + (cp[0] - b'0') as f64;
                        cp = &cp[1..];
                    }
                    if cp.first() == Some(&b'.') {
                        let mut dec = 1.0;
                        cp = &cp[1..];
                        while cp.first().is_some_and(|&c| c >= b'0' && c <= b'9') {
                            dec *= 10.0;
                            pt_size = pt_size + (cp[0] - b'0') as f64 / dec;
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

            libc::strncpy(
                variant,
                var_str.as_bytes() as *const [u8] as *const libc::c_char,
                var_str.len(),
            );
            *variant.add(var_str.len()) = 0;

            if req_ital {
                let mut best_match = font;
                if (*font).slant < (*parent).max_slant {
                    // try for a face with more slant
                    best_match = Self::best_match_from_family(
                        &*parent,
                        (*font).weight as libc::c_int,
                        (*font).width as libc::c_int,
                        (*parent).max_slant as libc::c_int,
                    );
                }

                if best_match == font && (*font).slant > (*parent).min_slant {
                    // maybe the slant is negated, or maybe this was something like "Times-Italic/I"
                    best_match = Self::best_match_from_family(
                        &*parent,
                        (*font).weight as libc::c_int,
                        (*font).width as libc::c_int,
                        (*parent).min_slant as libc::c_int,
                    );
                }

                if (*parent).min_weight == (*parent).max_weight
                    && (*best_match).is_bold != (*font).is_bold
                {
                    // try again using the bold flag, as we can't trust weight values
                    let mut new_best = ptr::null_mut::<Font>();
                    for (_, &style) in &(*parent).styles {
                        if (*style).is_bold == (*font).is_bold
                            && new_best.is_null()
                            && (*style).is_italic != (*font).is_italic
                        {
                            new_best = style;
                            break;
                        }
                    }
                    if !new_best.is_null() {
                        best_match = new_best;
                    }
                }

                if best_match == font {
                    // maybe slant values weren't present; try the style bits as a fallback
                    best_match = ptr::null_mut();
                    for (_, &style) in &(*parent).styles {
                        if (*style).is_italic == !(*font).is_italic {
                            if (*parent).min_weight != (*parent).max_weight {
                                // weight info was available, so try to match that
                                if best_match.is_null()
                                    || Self::weight_and_width_diff(&*style, &*font)
                                        < Self::weight_and_width_diff(&*best_match, &*font)
                                {
                                    best_match = style;
                                }
                            } else {
                                // no weight info, so try matching style bits
                                if best_match.is_null() && (*style).is_bold == (*font).is_bold {
                                    best_match = style;
                                    break; // found a match, no need to look further as we can't distinguish!
                                }
                            }
                        }
                    }
                }

                if !best_match.is_null() {
                    font = best_match;
                }
            }

            if req_bold {
                let mut best_match = font;
                if (*font).weight < (*parent).max_weight {
                    best_match = Self::best_match_from_family(
                        &*parent,
                        ((*font).weight + ((*parent).max_weight - ((*parent).min_weight)) / 2 + 1)
                            as libc::c_int,
                        (*font).width as libc::c_int,
                        (*font).slant as libc::c_int,
                    );
                    if (*parent).min_slant == (*parent).max_slant {
                        let mut new_best = ptr::null_mut::<Font>();
                        for (_, &style) in &(*parent).styles {
                            if (*style).is_italic == (*font).is_italic
                                && new_best.is_null()
                                && Self::weight_and_width_diff(&*style, &*best_match)
                                    < Self::weight_and_width_diff(&*new_best, &*best_match)
                            {
                                new_best = style;
                            }
                        }
                        if !new_best.is_null() {
                            best_match = new_best;
                        }
                    }
                }
                if best_match == font && (*font).is_bold {
                    for (_, &style) in &(*parent).styles {
                        if (*style).is_italic == (*font).is_italic && (*style).is_bold {
                            best_match = style;
                            break;
                        }
                    }
                }
                font = best_match;
            }
        }

        if pt_size < 0.0 {
            pt_size = dsize;
        }

        if !font.is_null() && (*font).op_size_info.sub_family_id != 0 && pt_size > 0.0 {
            let mut best_mismatch = f64::max(
                (*font).op_size_info.min_size - pt_size,
                pt_size - (*font).op_size_info.max_size,
            );
            if best_mismatch > 0.0 {
                let mut best_match = font;
                for (_, &style) in &(*parent).styles {
                    if (*style).op_size_info.sub_family_id != (*font).op_size_info.sub_family_id {
                        continue;
                    }
                    let mismatch = f64::max(
                        (*style).op_size_info.min_size - pt_size,
                        pt_size - (*style).op_size_info.max_size,
                    );
                    if mismatch < best_mismatch {
                        best_match = style;
                        best_mismatch = mismatch;
                    }
                    if best_mismatch <= 0.0 {
                        break;
                    }
                }
                font = best_match;
            }
        }

        if !font.is_null() && (*font).op_size_info.design_size != 0.0 {
            LOADED_FONT_DESIGN_SIZE
                .set(((*font).op_size_info.design_size * 65536.0 + 0.5) as Fixed);
        }

        Some((*font).font_ref.clone())
    }

    pub unsafe fn get_full_name(&self, font: PlatformFontRef) -> *const libc::c_char {
        let font = *self
            .maps
            .platform_ref_to_font
            .get(&font)
            .unwrap_or_else(|| panic!("internal error {} in FontManager", 2));
        let name = (*font).full_name.as_ref().unwrap_or(&(*font).ps_name);
        name.as_ptr()
    }

    pub unsafe fn get_design_size(&self, font: XeTeXFont) -> f64 {
        let size_rec = Self::get_op_size(font);
        match size_rec {
            None => 10.0,
            Some(size_rec) => size_rec.design_size,
        }
    }

    pub fn weight_and_width_diff(a: &Font, b: &Font) -> libc::c_int {
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
        a: &Font,
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

    pub unsafe fn best_match_from_family(
        family: &Family,
        wt: libc::c_int,
        wd: libc::c_int,
        slant: libc::c_int,
    ) -> *mut Font {
        let mut best_match = None;
        for (_, &font) in &family.styles {
            best_match = match best_match {
                None => Some(font),
                Some(best) => {
                    if Self::style_diff(&*font, wt, wd, slant)
                        < Self::style_diff(&*best, wt, wd, slant)
                    {
                        Some(font)
                    } else {
                        Some(best)
                    }
                }
            };
        }
        best_match.unwrap_or(ptr::null_mut())
    }

    pub unsafe fn append_to_list<T: Into<CString> + AsRef<CStr>>(list: &mut Vec<CString>, str: T) {
        if !list.iter().any(|s| **s == *str.as_ref()) {
            list.push(str.into())
        }
    }

    pub unsafe fn prepend_to_list<T: Into<CString> + AsRef<CStr>>(list: &mut Vec<CString>, str: T) {
        *list = list.drain(..).filter(|s| **s != *str.as_ref()).collect();
        list.insert(0, str.into());
    }

    pub unsafe fn get_op_size(font: XeTeXFont) -> Option<OpSizeRec> {
        let hb_font = (*font).get_hb_font();
        if hb_font.is_null() {
            return None;
        }

        let face = hb_font_get_face(hb_font);
        let mut size_rec = OpSizeRec::default();

        let mut design_size = 0;
        let mut min_size = 0;
        let mut max_size = 0;
        let ok = hb_ot_layout_get_size_params(
            face,
            &mut design_size,
            &mut size_rec.sub_family_id,
            &mut size_rec.name_code,
            &mut min_size,
            &mut max_size,
        );
        if ok != 0 {
            size_rec.design_size = design_size as f64 * 72.27 / 72.0 / 10.0;
            size_rec.min_size = min_size as f64 * 72.27 / 72.0 / 10.0;
            size_rec.max_size = max_size as f64 * 72.27 / 72.0 / 10.0;
            Some(size_rec)
        } else {
            None
        }
    }

    pub unsafe fn get_op_size_rec_and_style_flags(&self, font: &mut Font) {
        self.backend.get_op_size_rec_and_style_flags(font)
    }

    pub unsafe fn search_for_host_platform_fonts(&mut self, name: &CStr) {
        self.backend
            .search_for_host_platform_fonts(&mut self.maps, name)
    }

    pub unsafe fn read_names(&self, font: PlatformFontRef) -> NameCollection {
        self.backend.read_names(font)
    }

    pub unsafe fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> &'a CStr {
        self.backend.get_platform_font_desc(font)
    }

    pub fn get_req_engine(&self) -> libc::c_char {
        REQ_ENGINE.get()
    }

    pub fn set_req_engine(&mut self, engine: libc::c_char) {
        REQ_ENGINE.set(engine);
    }
}

#[no_mangle]
pub unsafe extern "C" fn terminate_font_manager() {
    FontManager::terminate();
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
    #[cfg(target_os = "macos")]
    panic!();
    #[cfg(not(target_os = "macos"))]
    FontManager::with_font_manager(|mgr| {
        mgr.find_font(name, var, size)
            .map(super::fc::Pattern::into_raw)
            .unwrap_or(ptr::null_mut())
    })
}

#[no_mangle]
pub unsafe extern "C" fn getReqEngine() -> libc::c_char {
    FontManager::with_font_manager(|mgr| mgr.get_req_engine())
}

#[no_mangle]
pub unsafe extern "C" fn setReqEngine(engine: libc::c_char) {
    FontManager::with_font_manager(|mgr| mgr.set_req_engine(engine))
}

#[no_mangle]
pub unsafe extern "C" fn getFullName(font: PlatformFontRef) -> *const libc::c_char {
    FontManager::with_font_manager(|mgr| mgr.get_full_name(font))
}

#[no_mangle]
pub unsafe extern "C" fn getDesignSize(font: XeTeXFont) -> f64 {
    FontManager::with_font_manager(|mgr| mgr.get_design_size(font))
}

#[no_mangle]
pub unsafe extern "C" fn ttxl_platfont_get_desc(font: PlatformFontRef) -> *const libc::c_char {
    FontManager::with_font_manager(|mgr| mgr.get_platform_font_desc(&font).as_ptr())
}
