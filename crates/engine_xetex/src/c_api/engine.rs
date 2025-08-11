use crate::ty::StrNumber;
use std::cell::RefCell;
use std::ffi::CStr;

thread_local! {
    static ENGINE_CTX: RefCell<EngineCtx> = const { RefCell::new(EngineCtx::new()) }
}

pub struct EngineCtx {
    pub(crate) selector: Selector,
}

impl EngineCtx {
    const fn new() -> EngineCtx {
        EngineCtx {
            selector: Selector::File(0),
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
