use crate::c_api::engine::{POP, TEX_INFINITY};
use crate::c_api::fatal_error;
use crate::c_api::globals::Globals;
use std::cell::RefCell;
use std::io::Write;
use std::ptr;
use tectonic_bridge_core::OutputId;

pub const DVI_BUF_SIZE: i32 = 16384;
pub const HALF_BUF: i32 = DVI_BUF_SIZE / 2;
pub const FNT_NUM_0: usize = 171; /* DVI code */

thread_local! {
    pub static DVI_CTX: RefCell<DviCtx> = const { RefCell::new(DviCtx::new()) }
}

pub struct DviCtx {
    file: Option<OutputId>,
    limit: i32,
    ptr: i32,
    offset: i32,
    gone: i32,
    cur_s: i32,
    buf: Vec<u8>,
}

impl DviCtx {
    const fn new() -> DviCtx {
        DviCtx {
            file: None,
            limit: 0,
            ptr: 0,
            offset: 0,
            gone: 0,
            cur_s: 0,
            buf: Vec::new(),
        }
    }
}

#[no_mangle]
pub extern "C" fn dvi_file() -> OutputId {
    DVI_CTX.with_borrow(|dvi| dvi.file.unwrap())
}

#[no_mangle]
pub extern "C" fn set_dvi_file(file: OutputId) {
    DVI_CTX.with_borrow_mut(|dvi| dvi.file = Some(file))
}

#[no_mangle]
pub extern "C" fn dvi_limit() -> i32 {
    DVI_CTX.with_borrow(|dvi| dvi.limit)
}

#[no_mangle]
pub extern "C" fn set_dvi_limit(val: i32) {
    DVI_CTX.with_borrow_mut(|dvi| dvi.limit = val)
}

#[no_mangle]
pub extern "C" fn dvi_ptr() -> i32 {
    DVI_CTX.with_borrow(|dvi| dvi.ptr)
}

#[no_mangle]
pub extern "C" fn set_dvi_ptr(val: i32) {
    DVI_CTX.with_borrow_mut(|dvi| dvi.ptr = val)
}

#[no_mangle]
pub extern "C" fn dvi_offset() -> i32 {
    DVI_CTX.with_borrow(|dvi| dvi.offset)
}

#[no_mangle]
pub extern "C" fn set_dvi_offset(val: i32) {
    DVI_CTX.with_borrow_mut(|dvi| dvi.offset = val)
}

#[no_mangle]
pub extern "C" fn dvi_gone() -> i32 {
    DVI_CTX.with_borrow(|dvi| dvi.gone)
}

#[no_mangle]
pub extern "C" fn set_dvi_gone(val: i32) {
    DVI_CTX.with_borrow_mut(|dvi| dvi.gone = val)
}

#[no_mangle]
pub extern "C" fn cur_s() -> i32 {
    DVI_CTX.with_borrow(|dvi| dvi.cur_s)
}

#[no_mangle]
pub extern "C" fn set_cur_s(val: i32) {
    DVI_CTX.with_borrow_mut(|dvi| dvi.cur_s = val)
}

#[no_mangle]
pub extern "C" fn dvi_buf(idx: usize) -> u8 {
    DVI_CTX.with_borrow(|engine| engine.buf[idx])
}

#[no_mangle]
pub extern "C" fn set_dvi_buf(idx: usize, val: u8) {
    DVI_CTX.with_borrow_mut(|engine| engine.buf[idx] = val)
}

#[no_mangle]
pub extern "C" fn dvi_buf_ptr(idx: usize) -> *mut u8 {
    DVI_CTX.with_borrow_mut(|engine| ptr::from_mut(&mut engine.buf[idx]))
}

#[no_mangle]
pub extern "C" fn resize_dvi_buf(len: usize) {
    DVI_CTX.with_borrow_mut(|engine| engine.buf.resize(len, 0))
}

#[no_mangle]
pub extern "C" fn clear_dvi_buf() {
    DVI_CTX.with_borrow_mut(|engine| engine.buf.clear())
}

#[no_mangle]
pub fn rs_write_to_dvi(globals: &mut Globals<'_, '_>, a: usize, b: usize) {
    let out = globals.state.get_output(globals.dvi.file.unwrap());
    out.write_all(&globals.dvi.buf[a..=b])
        .expect("failed to write data to XDV file");
}

#[no_mangle]
pub extern "C" fn write_to_dvi(a: i32, b: i32) {
    Globals::with(|globals| rs_write_to_dvi(globals, a as usize, b as usize))
}

#[no_mangle]
pub extern "C" fn deinitialize_shipout_variables() {
    clear_dvi_buf();
}

pub fn dvi_swap(globals: &mut Globals<'_, '_>) {
    if globals.dvi.ptr > TEX_INFINITY - globals.dvi.offset {
        globals.dvi.cur_s = -2;
        // TODO: This may violate Globals::with uniqueness if we're already in a fatal_error
        unsafe { fatal_error(c"dvi length exceeds 0x7FFFFFFF".as_ptr()) };
    }

    if globals.dvi.limit == DVI_BUF_SIZE {
        rs_write_to_dvi(globals, 0, (HALF_BUF - 1) as usize);
        globals.dvi.limit = HALF_BUF;
        globals.dvi.offset += DVI_BUF_SIZE;
        globals.dvi.ptr = 0;
    } else {
        rs_write_to_dvi(globals, HALF_BUF as usize, (DVI_BUF_SIZE - 1) as usize);
        globals.dvi.limit = DVI_BUF_SIZE;
    }
    globals.dvi.gone += HALF_BUF;
}

pub fn rs_dvi_out(globals: &mut Globals<'_, '_>, c: u8) {
    globals.dvi.buf[globals.dvi.ptr as usize] = c;
    globals.dvi.ptr += 1;
    if globals.dvi.ptr == globals.dvi.limit {
        dvi_swap(globals);
    }
}

#[no_mangle]
pub extern "C" fn dvi_out(c: u8) {
    Globals::with(|globals| rs_dvi_out(globals, c))
}

pub fn rs_dvi_four(globals: &mut Globals<'_, '_>, mut x: i32) {
    // TODO: Honestly, this could just use `x.to_*_bytes()`
    if x >= 0 {
        rs_dvi_out(globals, (x / 0x1000000) as u8);
    } else {
        x = x + 0x40000000;
        x = x + 0x40000000;
        rs_dvi_out(globals, ((x / 0x1000000) + 128) as u8);
    }

    x = x % 0x1000000;
    rs_dvi_out(globals, (x / 0x10000) as u8);

    x = x % 0x10000;
    rs_dvi_out(globals, (x / 0x100) as u8);
    rs_dvi_out(globals, (x % 0x100) as u8);
}

#[no_mangle]
pub extern "C" fn dvi_four(x: i32) {
    Globals::with(|globals| rs_dvi_four(globals, x))
}

pub fn rs_dvi_two(globals: &mut Globals<'_, '_>, mut s: u16) {
    rs_dvi_out(globals, (s / 0x100) as u8);
    rs_dvi_out(globals, (s % 0x100) as u8);
}

#[no_mangle]
pub extern "C" fn dvi_two(s: u16) {
    Globals::with(|globals| rs_dvi_two(globals, s))
}

pub fn rs_dvi_pop(globals: &mut Globals<'_, '_>, l: i32) {
    if l == globals.dvi.offset + globals.dvi.ptr && globals.dvi.ptr > 0 {
        globals.dvi.ptr -= 1;
    } else {
        rs_dvi_out(globals, POP);
    }
}

#[no_mangle]
pub extern "C" fn dvi_pop(l: i32) {
    Globals::with(|globals| rs_dvi_pop(globals, l))
}
