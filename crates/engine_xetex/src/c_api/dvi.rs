use std::cell::RefCell;
use std::ptr;
use tectonic_bridge_core::OutputId;

thread_local! {
    pub static DVI_CTX: RefCell<DviCtx> = const { RefCell::new(DviCtx::new()) }
}

pub struct DviCtx {
    file: Option<OutputId>,
    limit: i32,
    ptr: i32,
    offset: i32,
    gone: i32,
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
