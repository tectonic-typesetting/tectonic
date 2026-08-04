use crate::c_api::d_to_fix;
use crate::c_api::engine::{B16x4, MemoryWord};
use crate::c_api::errors::EngineError;
use crate::c_api::globals::Globals;
use crate::ty::{Scaled, StrNumber};
use std::borrow::Cow;
use std::cell::RefCell;
#[cfg(target_os = "macos")]
use tectonic_mac_core::sys::{
    kCTFontAttributeName, kCTForegroundColorAttributeName, kCTVerticalFormsAttributeName,
    CFStringRef,
};
#[cfg(target_os = "macos")]
use tectonic_mac_core::{CFDictionary, CFNumber, CFString, CFType, CGColor, CTFont, CoreType};
use tectonic_xetex_layout::engine::LayoutEngine;
#[cfg(target_os = "macos")]
use tectonic_xetex_layout::font::get_file_name_from_ct_font;

pub const AAT_FONT_FLAG: i32 = 0xFFFF;
pub const OTGR_FONT_FLAG: i32 = 0xFFFE;

pub const FONT_FLAGS_COLORED: u8 = 0x01;
pub const FONT_FLAGS_VERTICAL: u8 = 0x02;

pub const XDV_FLAG_VERTICAL: u16 = 0x0100;
pub const XDV_FLAG_COLORED: u16 = 0x0200;
pub const XDV_FLAG_EXTEND: u16 = 0x1000;
pub const XDV_FLAG_SLANT: u16 = 0x2000;
pub const XDV_FLAG_EMBOLDEN: u16 = 0x4000;

thread_local! {
    static FONT_CTX: RefCell<FontCtx> = const { RefCell::new(FontCtx::new()) };
}

pub struct FontCtx {
    pub(crate) font_max: i32,
    pub(crate) font_ptr: i32,

    pub(crate) font_used: Vec<bool>,
    pub(crate) font_info: Vec<MemoryWord>,
    pub(crate) font_mapping: Vec<*mut libc::c_void>,
    pub(crate) font_layout_engine: Vec<*mut libc::c_void>,
    pub(crate) font_flags: Vec<u8>,
    pub(crate) font_letter_space: Vec<Scaled>,
    pub(crate) font_check: Vec<B16x4>,
    pub(crate) font_size: Vec<Scaled>,
    pub(crate) font_dsize: Vec<Scaled>,
    pub(crate) font_params: Vec<i32>, // font index
    pub(crate) font_name: Vec<StrNumber>,
    pub(crate) font_area: Vec<StrNumber>,
    pub(crate) font_bc: Vec<u16>,
    pub(crate) font_ec: Vec<u16>,
    pub(crate) font_glue: Vec<i32>,
    pub(crate) hyphen_char: Vec<i32>,
    pub(crate) skew_char: Vec<i32>,
    pub(crate) bchar_label: Vec<i32>,      // font index
    pub(crate) font_bchar: Vec<i32>,       // nine bits
    pub(crate) font_false_bchar: Vec<i32>, // nine bits
    pub(crate) char_base: Vec<i32>,
    pub(crate) width_base: Vec<i32>,
    pub(crate) height_base: Vec<i32>,
    pub(crate) depth_base: Vec<i32>,
    pub(crate) italic_base: Vec<i32>,
    pub(crate) lig_kern_base: Vec<i32>,
    pub(crate) kern_base: Vec<i32>,
    pub(crate) exten_base: Vec<i32>,
    pub(crate) param_base: Vec<i32>,

    pub(crate) xdv_buffer: Vec<u8>,
}

impl FontCtx {
    const fn new() -> FontCtx {
        FontCtx {
            font_max: 0,
            font_ptr: 0,

            font_used: Vec::new(),
            font_info: Vec::new(),
            font_mapping: Vec::new(),
            font_layout_engine: Vec::new(),
            font_flags: Vec::new(),
            font_letter_space: Vec::new(),
            font_check: Vec::new(),
            font_size: Vec::new(),
            font_dsize: Vec::new(),
            font_params: Vec::new(),
            font_name: Vec::new(),
            font_area: Vec::new(),

            font_bc: Vec::new(),
            font_ec: Vec::new(),
            font_glue: Vec::new(),
            hyphen_char: Vec::new(),
            skew_char: Vec::new(),
            bchar_label: Vec::new(),
            font_bchar: Vec::new(),
            font_false_bchar: Vec::new(),
            char_base: Vec::new(),
            width_base: Vec::new(),
            height_base: Vec::new(),
            depth_base: Vec::new(),
            italic_base: Vec::new(),
            lig_kern_base: Vec::new(),
            kern_base: Vec::new(),
            exten_base: Vec::new(),
            param_base: Vec::new(),
            xdv_buffer: Vec::new(),
        }
    }

    pub fn with<T>(f: impl FnOnce(&mut FontCtx) -> T) -> T {
        FONT_CTX.with_borrow_mut(f)
    }
}

c_var!(FontCtx => font_max: i32);
c_var!(FontCtx => font_ptr: i32);

c_arr!(FontCtx => font_used: bool);
c_arr!(FontCtx => font_info: MemoryWord);
c_arr!(FontCtx => font_mapping: *mut libc::c_void);
c_arr!(FontCtx => font_layout_engine: *mut libc::c_void);
c_arr!(FontCtx => font_flags: u8);
c_arr!(FontCtx => font_letter_space: Scaled);
c_arr!(FontCtx => font_check: B16x4);
c_arr!(FontCtx => font_size: Scaled);
c_arr!(FontCtx => font_dsize: Scaled);
c_arr!(FontCtx => font_params: i32);
c_arr!(FontCtx => font_name: StrNumber);
c_arr!(FontCtx => font_area: StrNumber);
c_arr!(FontCtx => font_bc: u16);
c_arr!(FontCtx => font_ec: u16);
c_arr!(FontCtx => font_glue: i32);
c_arr!(FontCtx => hyphen_char: i32);
c_arr!(FontCtx => skew_char: i32);
c_arr!(FontCtx => bchar_label: i32);
c_arr!(FontCtx => font_bchar: i32);
c_arr!(FontCtx => font_false_bchar: i32);
c_arr!(FontCtx => char_base: i32);
c_arr!(FontCtx => width_base: i32);
c_arr!(FontCtx => height_base: i32);
c_arr!(FontCtx => depth_base: i32);
c_arr!(FontCtx => italic_base: i32);
c_arr!(FontCtx => lig_kern_base: i32);
c_arr!(FontCtx => kern_base: i32);
c_arr!(FontCtx => exten_base: i32);
c_arr!(FontCtx => param_base: i32);

c_arr!(FontCtx => xdv_buffer: u8);

#[no_mangle]
pub extern "C" fn xdv_buf_size() -> usize {
    FontCtx::with(|fonts| fonts.xdv_buffer.len())
}

#[cfg(target_os = "macos")]
fn cgcolor_to_rgba32(color: CGColor) -> u32 {
    let components = color.components();
    let mut out = (components[0] * 255.0 + 0.5) as u32;
    out <<= 8;
    out += (components[1] * 255.0 + 0.5) as u32;
    out <<= 8;
    out += (components[2] * 255.0 + 0.5) as u32;
    out <<= 8;
    out += (components[3] * 255.0 + 0.5) as u32;
    out
}

pub fn make_font_def(globals: &mut Globals<'_, '_>, f: usize) -> Result<Vec<u8>, EngineError> {
    let mut flags = 0;
    let rgba;
    let index;
    let size;
    let filename;
    let extend;
    let slant;
    let embolden;

    if cfg!(target_os = "macos") && globals.fonts.font_area[f] == AAT_FONT_FLAG {
        #[cfg(target_os = "macos")]
        unsafe {
            use std::ptr::NonNull;
            unsafe fn cfstr(raw: CFStringRef) -> CFString {
                CFString::new_borrowed(NonNull::new(raw.cast_mut()).unwrap())
            }

            let attributes =
                globals.fonts.font_layout_engine[f].cast::<CFDictionary<CFString, CFType>>();
            let attributes = attributes.as_ref().unwrap();

            let font = attributes
                .get(cfstr(kCTFontAttributeName))
                .unwrap()
                .downcast::<CTFont>()
                .unwrap();

            let (idx, name) = get_file_name_from_ct_font(&font).unwrap();
            index = idx;
            filename = Cow::Owned(name);

            if attributes
                .get(cfstr(kCTVerticalFormsAttributeName))
                .is_some()
            {
                flags |= XDV_FLAG_VERTICAL;
            }

            let color = attributes.get(cfstr(kCTForegroundColorAttributeName));
            if let Some(color) = color {
                rgba = cgcolor_to_rgba32(color.downcast::<CGColor>().unwrap());
            } else {
                rgba = 0;
            }

            let t = font.matrix();
            extend = t.a as f32;
            slant = t.c as f32;
            let embolden_num = attributes.get(CFString::new_static("XeTeXEmbolden"));
            if let Some(num) = embolden_num {
                embolden = num.downcast::<CFNumber>().unwrap().value::<f64>().unwrap() as f32;
            } else {
                embolden = 0.0
            }

            size = d_to_fix(font.size());
        }
        #[cfg(not(target_os = "macos"))]
        {
            unreachable!("This branch should only be reachable on MacOS")
        }
    } else if globals.fonts.font_area[f] == OTGR_FONT_FLAG {
        let engine = globals.fonts.font_layout_engine[f].cast::<LayoutEngine>();
        let engine = unsafe { engine.as_mut().unwrap() };
        let (idx, name) = engine.font().filename();
        index = idx;
        filename = Cow::Borrowed(name);
        rgba = engine.rgb();
        if globals.fonts.font_flags[f] & FONT_FLAGS_VERTICAL != 0 {
            flags |= XDV_FLAG_VERTICAL;
        }
        extend = engine.extend();
        slant = engine.slant();
        embolden = engine.embolden();
        size = d_to_fix(engine.font().point_size() as f64);
    } else {
        return Err(EngineError::new(c"bad native font flag in `make_font_def`"));
    }

    let filename_len = filename.to_bytes().len();

    // parameters after internal font ID:
    //  size[4]
    //  flags[2]
    //  l[1] n[l]
    //  if flags & COLORED:
    //      c[4]

    if globals.fonts.font_flags[f] & FONT_FLAGS_COLORED != 0 {
        flags |= XDV_FLAG_COLORED;
    }

    if extend != 1.0 {
        flags |= XDV_FLAG_EXTEND;
    }

    if slant != 0.0 {
        flags |= XDV_FLAG_SLANT;
    }

    if embolden != 0.0 {
        flags |= XDV_FLAG_EMBOLDEN;
    }

    let mut buffer = Vec::new();
    buffer.extend(size.to_be_bytes());
    buffer.extend(flags.to_be_bytes());
    buffer.extend([filename_len as u8]);
    buffer.extend(filename.to_bytes());
    buffer.extend(index.to_be_bytes());

    if flags & XDV_FLAG_COLORED != 0 {
        buffer.extend(rgba.to_be_bytes());
    }

    if flags & XDV_FLAG_EXTEND != 0 {
        buffer.extend(d_to_fix(extend as f64).to_be_bytes());
    }

    if flags & XDV_FLAG_SLANT != 0 {
        buffer.extend(d_to_fix(slant as f64).to_be_bytes());
    }

    if flags & XDV_FLAG_EMBOLDEN != 0 {
        buffer.extend(d_to_fix(embolden as f64).to_be_bytes());
    }

    Ok(buffer)
}

#[no_mangle]
pub extern "C" fn release_font_engine(engine: *mut libc::c_void, flag: i32) {
    if cfg!(target_os = "macos") && flag == AAT_FONT_FLAG {
        #[cfg(target_os = "macos")]
        unsafe {
            use std::ptr::NonNull;
            CFDictionary::<CFString, CFType>::new_owned(NonNull::new(engine).unwrap().cast())
        };
    } else if flag == OTGR_FONT_FLAG {
        let _ = unsafe { Box::from_raw(engine.cast::<LayoutEngine>()) };
    }
}
