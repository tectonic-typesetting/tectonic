use crate::c_api::{Fixed, PlatformFontRef, RawPlatformFontRef};
use std::ptr::NonNull;

pub fn fix_to_d(f: Fixed) -> f64 {
    f as f64 / 65536.0
}

pub fn d_to_fix(d: f64) -> Fixed {
    (d * 65536.0 + 0.5) as Fixed
}

pub fn raw_to_rs(font: RawPlatformFontRef) -> Option<PlatformFontRef> {
    #[cfg(target_os = "macos")]
    let out = {
        use tectonic_mac_core::CoreType;
        NonNull::new(font.cast_mut()).map(PlatformFontRef::new_borrowed)
    };
    #[cfg(not(target_os = "macos"))]
    let out = { unsafe { NonNull::new(font).map(|p| PlatformFontRef::from_raw_borrowed(p)) } };
    out
}
