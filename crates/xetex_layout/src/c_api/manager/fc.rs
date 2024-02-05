use super::{Font, FontManagerBackend, NameCollection};
use crate::c_api::fc::{
    FcConfigGetCurrent, FcFalse, FcFontList, FcFontSet, FcFontSetDestroy, FcInit, FcNameParse,
    FcObjectSetBuild, FcObjectSetDestroy, FcPatternDestroy, FC_FAMILY, FC_FILE, FC_FONTFORMAT,
    FC_FULLNAME, FC_INDEX, FC_SLANT, FC_STYLE, FC_WEIGHT, FC_WIDTH,
};
use crate::c_api::unicode::{ucnv_close, ucnv_open, UConverter, U_SUCCESS, U_ZERO_ERROR};
use crate::c_api::{PlatformFontRef, _tt_abort};
use std::cell::{Cell, RefCell};
use std::ffi::CStr;
use std::ptr;
use tectonic_bridge_freetype2::{FT_Init_FreeType, FT_Library};

thread_local! {
    static FREE_TYPE_LIBRARY: RefCell<FT_Library> = const { RefCell::new(ptr::null_mut()) };
    static MAC_ROMAN_CONV: Cell<*mut UConverter> = const { Cell::new(ptr::null_mut()) };
    static UTF16_BE_CONV: Cell<*mut UConverter> = const { Cell::new(ptr::null_mut()) };
    static UTF8_CONV: Cell<*mut UConverter> = const { Cell::new(ptr::null_mut()) };
}

pub struct FcBackend {
    all_fonts: *mut FcFontSet,
    cached_all: bool,
}

impl FcBackend {
    pub fn new() -> FcBackend {
        FcBackend {
            all_fonts: ptr::null_mut(),
            cached_all: false,
        }
    }
}

impl FontManagerBackend for FcBackend {
    unsafe fn initialize(&mut self) {
        if FcInit() == FcFalse {
            _tt_abort(c!("fontconfig initialization failed"));
        }

        if FREE_TYPE_LIBRARY.with_borrow(|lib| lib.is_null()) {
            let res = FREE_TYPE_LIBRARY.with_borrow_mut(|lib| FT_Init_FreeType(lib));
            if res != 0 {
                _tt_abort(c!("FreeType initialization failed"));
            }
        }

        let mut err = U_ZERO_ERROR;

        MAC_ROMAN_CONV.set(ucnv_open(c!("macintosh"), &mut err));
        if !U_SUCCESS(err) {
            err = U_ZERO_ERROR;
            MAC_ROMAN_CONV.set(ptr::null_mut());
        }

        UTF16_BE_CONV.set(ucnv_open(c!("UTF16BE"), &mut err));
        UTF8_CONV.set(ucnv_open(c!("UTF8"), &mut err));
        if !U_SUCCESS(err) {
            _tt_abort(c!("cannot read font names"));
        }

        let pat = FcNameParse(c!(":outline=true") as *const u8);
        let os = FcObjectSetBuild(
            FC_FAMILY,
            FC_STYLE,
            FC_FILE,
            FC_INDEX,
            FC_FULLNAME,
            FC_WEIGHT,
            FC_WIDTH,
            FC_SLANT,
            FC_FONTFORMAT,
            ptr::null::<()>(),
        );
        self.all_fonts = FcFontList(FcConfigGetCurrent(), pat, os);
        FcObjectSetDestroy(os);
        FcPatternDestroy(pat);
        self.cached_all = false;
    }

    unsafe fn terminate(&mut self) {
        if !self.all_fonts.is_null() {
            FcFontSetDestroy(self.all_fonts);
            self.all_fonts = ptr::null_mut();
        }

        if !MAC_ROMAN_CONV.get().is_null() {
            ucnv_close(MAC_ROMAN_CONV.get());
            MAC_ROMAN_CONV.set(ptr::null_mut());
        }
        if !UTF16_BE_CONV.get().is_null() {
            ucnv_close(UTF16_BE_CONV.get());
            UTF16_BE_CONV.set(ptr::null_mut());
        }
        if !UTF8_CONV.get().is_null() {
            ucnv_close(UTF8_CONV.get());
            UTF8_CONV.set(ptr::null_mut());
        }
    }

    fn get_platform_font_desc(&self, font: PlatformFontRef) -> &CStr {
        todo!()
    }

    fn get_op_size_rec_and_style_flags(&self, font: *mut Font) {
        todo!()
    }

    fn search_for_host_platform_fonts(&self, name: &str) {
        todo!()
    }

    fn read_names(&self, font: PlatformFontRef) -> Box<NameCollection> {
        todo!()
    }
}
