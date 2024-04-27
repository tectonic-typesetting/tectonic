use crate::c_api::{Fixed, RawPlatformFontRef, XeTeXFont};
use crate::manager::{Engine, FontManager};
use crate::utils::raw_to_rs;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::{ptr, slice};

#[no_mangle]
pub extern "C" fn get_loaded_font_design_size() -> Fixed {
    FontManager::with_font_manager(|mgr| mgr.font_design_size())
}

#[no_mangle]
pub extern "C" fn set_loaded_font_design_size(val: Fixed) {
    FontManager::with_font_manager(|mgr| {
        mgr.set_font_design_size(val);
    });
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
