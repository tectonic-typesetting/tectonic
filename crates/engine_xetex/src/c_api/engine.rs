use crate::ty::StrNumber;
use std::cell::RefCell;
use std::ffi::CStr;

thread_local! {
    static ENGINE_CTX: RefCell<EngineCtx> = const { RefCell::new(EngineCtx::new()) }
}

pub struct EngineCtx {
    selector: Selector,
}

impl EngineCtx {
    const fn new() -> EngineCtx {
        EngineCtx {
            selector: Selector::File0,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub enum Selector {
    File0 = 0,
    File1,
    File2,
    File3,
    File4,
    File5,
    File6,
    File7,
    File8,
    File9,
    File10,
    File11,
    File12,
    File13,
    File14,
    File15 = 15,
    NoPrint = 16,
    TermOnly = 17,
    LogOnly = 18,
    TermAndLog = 19,
    Pseudo = 20,
    NewString = 21,
}

impl TryFrom<u32> for Selector {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Selector::File0),
            1 => Ok(Selector::File1),
            2 => Ok(Selector::File2),
            3 => Ok(Selector::File3),
            4 => Ok(Selector::File4),
            5 => Ok(Selector::File5),
            6 => Ok(Selector::File6),
            7 => Ok(Selector::File7),
            8 => Ok(Selector::File8),
            9 => Ok(Selector::File9),
            10 => Ok(Selector::File10),
            11 => Ok(Selector::File11),
            12 => Ok(Selector::File12),
            13 => Ok(Selector::File13),
            14 => Ok(Selector::File14),
            15 => Ok(Selector::File15),
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
pub extern "C" fn selector() -> Selector {
    ENGINE_CTX.with_borrow(|engine| engine.selector)
}

#[no_mangle]
pub extern "C" fn set_selector(val: u32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.selector = Selector::try_from(val).unwrap());
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
