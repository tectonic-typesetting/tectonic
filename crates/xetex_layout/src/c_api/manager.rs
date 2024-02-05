use crate::c_api::{PlatformFontRef, XeTeXFont};
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
    ps_name: Option<CString>,
    family_name: Option<String>,
    style_name: Option<String>,
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
    fn new(font_ref: PlatformFontRef) -> Font {
        let mut out = Font {
            full_name: None,
            ps_name: None,
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
    styles: HashMap<String, *mut Font>,
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
    family_names: Vec<String>,
    style_names: Vec<String>,
    full_names: Vec<String>,
    ps_name: String,
    sub_family: String,
}

pub trait FontManagerBackend {
    unsafe fn initialize(&mut self);
    unsafe fn terminate(&mut self);
    fn get_platform_font_desc(&self, font: PlatformFontRef) -> &CStr;
    fn get_op_size_rec_and_style_flags(&self, font: *mut Font);
    fn search_for_host_platform_fonts(&self, name: &str);
    fn read_names(&self, font: PlatformFontRef) -> Box<NameCollection>;
}

pub struct FontManager {
    backend: Box<dyn FontManagerBackend>,

    name_to_font: HashMap<String, *mut Font>,
    name_to_family: HashMap<String, *mut Family>,
    platform_ref_to_font: HashMap<PlatformFontRef, *mut Font>,
    ps_name_to_font: HashMap<String, *mut Font>,
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
                name_to_font: Default::default(),
                name_to_family: Default::default(),
                platform_ref_to_font: Default::default(),
                ps_name_to_font: Default::default(),
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

    pub fn find_font(
        &self,
        name: *const libc::c_char,
        variant: *mut libc::c_char,
        pt_size: f64,
    ) -> PlatformFontRef {
        todo!()
    }

    pub unsafe fn get_full_name(&self, font: PlatformFontRef) -> *const libc::c_char {
        let font = *self
            .platform_ref_to_font
            .get(&font)
            .unwrap_or_else(|| panic!("internal error {} in FontManager", 2));
        let name = (*font)
            .full_name
            .as_ref()
            .or((*font).ps_name.as_ref())
            .unwrap();
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

    pub fn append_to_list(list: &mut Vec<String>, str: *const libc::c_char) {
        todo!()
    }

    pub fn prepend_to_list(list: &mut Vec<String>, str: *const libc::c_char) {
        todo!()
    }

    pub fn add_to_maps(&mut self, font: PlatformFontRef, names: *const NameCollection) {
        todo!()
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

    pub fn get_op_size_rec_and_style_flags(&self, font: *mut Font) {
        self.backend.get_op_size_rec_and_style_flags(font)
    }

    pub fn search_for_host_platform_fonts(&self, name: &str) {
        self.backend.search_for_host_platform_fonts(name)
    }

    pub fn read_names(&self, font: PlatformFontRef) -> Box<NameCollection> {
        self.backend.read_names(font)
    }

    pub fn get_platform_font_desc(&self, font: PlatformFontRef) -> &CStr {
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
) -> PlatformFontRef {
    FontManager::with_font_manager(|mgr| mgr.find_font(name, var, size))
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
    FontManager::with_font_manager(|mgr| mgr.get_platform_font_desc(font).as_ptr())
}
