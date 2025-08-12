use crate::ty::StrNumber;
use std::cell::RefCell;
use std::ffi::CStr;

thread_local! {
    pub static ENGINE_CTX: RefCell<EngineCtx> = const { RefCell::new(EngineCtx::new()) }
}

pub struct EngineCtx {
    pub(crate) selector: Selector,
    tally: i32,
    error_line: i32,
    trick_count: i32,
    trick_buf: [u16; 256],
}

impl EngineCtx {
    const fn new() -> EngineCtx {
        EngineCtx {
            selector: Selector::File(0),
            tally: 0,
            error_line: 0,
            trick_count: 0,
            trick_buf: [0; 256],
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Selector {
    File(u8),
    NoPrint,
    TermOnly,
    LogOnly,
    TermAndLog,
    Pseudo,
    NewString,
}

impl From<Selector> for u32 {
    fn from(value: Selector) -> Self {
        match value {
            Selector::File(val) => val as u32,
            Selector::NoPrint => 16,
            Selector::TermOnly => 17,
            Selector::LogOnly => 18,
            Selector::TermAndLog => 19,
            Selector::Pseudo => 20,
            Selector::NewString => 21,
        }
    }
}

impl TryFrom<u32> for Selector {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            val @ 0..16 => Ok(Selector::File(val as u8)),
            16 => Ok(Selector::NoPrint),
            17 => Ok(Selector::TermOnly),
            18 => Ok(Selector::LogOnly),
            19 => Ok(Selector::TermAndLog),
            20 => Ok(Selector::Pseudo),
            21 => Ok(Selector::NewString),
            _ => Err(()),
        }
    }
}

#[no_mangle]
pub extern "C" fn selector() -> u32 {
    ENGINE_CTX.with_borrow(|engine| engine.selector.into())
}

#[no_mangle]
pub extern "C" fn set_selector(val: u32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.selector = Selector::try_from(val).unwrap());
}

#[no_mangle]
pub extern "C" fn tally() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.tally)
}

#[no_mangle]
pub extern "C" fn set_tally(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.tally = val)
}

#[no_mangle]
pub extern "C" fn error_line() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.error_line)
}

#[no_mangle]
pub extern "C" fn set_error_line(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.error_line = val)
}

#[no_mangle]
pub extern "C" fn trick_count() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.trick_count)
}

#[no_mangle]
pub extern "C" fn set_trick_count(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.trick_count = val)
}

#[no_mangle]
pub extern "C" fn trick_buf(idx: usize) -> u16 {
    ENGINE_CTX.with_borrow(|engine| engine.trick_buf[idx])
}

#[no_mangle]
pub extern "C" fn set_trick_buf(idx: usize, val: u16) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.trick_buf[idx] = val)
}

pub fn with_tex_string<T>(s: StrNumber, f: impl FnOnce(&CStr) -> T) -> T {
    let ptr = unsafe { gettexstring(s) };
    let str = unsafe { CStr::from_ptr(ptr) };
    let out = f(str);
    unsafe { libc::free(ptr.cast()) };
    out
}

unsafe extern "C" {
    fn gettexstring(s: StrNumber) -> *mut libc::c_char;
}
