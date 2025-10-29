use crate::c_api::engine::B16x4;
use crate::c_api::globals::Globals;
use crate::ty::{Scaled, StrNumber};
use std::cell::RefCell;

pub const AAT_FONT_FLAG: i32 = 0xFFFF;
pub const OTGR_FONT_FLAG: i32 = 0xFFFE;

thread_local! {
    static FONT_CTX: RefCell<FontCtx> = const { RefCell::new(FontCtx::new()) };
}

pub struct FontCtx {
    pub(crate) font_ptr: i32,
    pub(crate) font_layout_engine: Vec<*mut libc::c_void>,
    pub(crate) font_used: Vec<bool>,
    pub(crate) font_check: Vec<B16x4>,
    pub(crate) font_size: Vec<Scaled>,
    pub(crate) font_dsize: Vec<Scaled>,
    pub(crate) font_name: Vec<StrNumber>,
    pub(crate) font_area: Vec<StrNumber>,
}

impl FontCtx {
    const fn new() -> FontCtx {
        FontCtx {
            font_ptr: 0,
            font_layout_engine: Vec::new(),
            font_used: Vec::new(),
            font_check: Vec::new(),
            font_size: Vec::new(),
            font_dsize: Vec::new(),
            font_name: Vec::new(),
            font_area: Vec::new(),
        }
    }

    pub fn with<T>(f: impl FnOnce(&mut FontCtx) -> T) -> T {
        FONT_CTX.with_borrow_mut(f)
    }
}

c_var!(FontCtx => font_ptr: i32);
c_arr!(FontCtx => font_layout_engine: *mut libc::c_void);
c_arr!(FontCtx => font_used: bool);
c_arr!(FontCtx => font_check: B16x4);
c_arr!(FontCtx => font_size: Scaled);
c_arr!(FontCtx => font_dsize: Scaled);
c_arr!(FontCtx => font_name: StrNumber);
c_arr!(FontCtx => font_area: StrNumber);
