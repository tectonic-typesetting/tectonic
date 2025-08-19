use crate::ty::StrNumber;
use std::cell::RefCell;
use std::ptr;

mod memory;

use crate::c_api::globals::Globals;
pub use memory::*;

pub const NULL_CS: usize = 0x220001;
pub const PRIM_SIZE: usize = 2100;
pub const UNDEFINED_CONTROL_SEQUENCE: usize = 0x226603;
pub const FROZEN_NULL_FONT: usize = 0x2242da;

pub const TEXT_SIZE: usize = 0;
pub const SCRIPT_SIZE: usize = 256;
pub const SCRIPT_SCRIPT_SIZE: usize = 512;

thread_local! {
    pub static ENGINE_CTX: RefCell<EngineCtx> = RefCell::new(EngineCtx::new())
}

pub struct EngineCtx {
    pub(crate) selector: Selector,
    pub(crate) tally: i32,
    pub(crate) error_line: i32,
    pub(crate) trick_count: i32,
    pub(crate) trick_buf: [u16; 256],
    pub(crate) eqtb_top: i32,

    pub(crate) eqtb: Vec<MemoryWord>,
    pub(crate) prim: Box<[B32x2; PRIM_SIZE + 1]>,
    pub(crate) mem: Vec<MemoryWord>,
}

struct NodeError {
    ty: u16,
    subty: u16,
}

impl EngineCtx {
    fn new() -> EngineCtx {
        EngineCtx {
            selector: Selector::File(0),
            tally: 0,
            error_line: 0,
            trick_count: 0,
            trick_buf: [0; 256],
            eqtb_top: 0,

            eqtb: Vec::new(),
            prim: Box::new([B32x2 { s0: 0, s1: 0 }; PRIM_SIZE + 1]),
            mem: Vec::new(),
        }
    }

    pub fn try_node<T: ?Sized + Node>(&self, idx: usize) -> Result<&T, NodeError> {
        let ptr = self.mem.as_ptr().wrapping_add(idx);
        let base = unsafe { &*NodeBase::from_ptr(ptr) };

        if T::ty() != base.ty() || T::subty() != base.subty() {
            return Err(NodeError {
                ty: base.ty(),
                subty: base.subty(),
            });
        }

        let ptr = unsafe { T::from_ptr(ptr) };
        Ok(unsafe { &*ptr })
    }

    pub fn node<T: ?Sized + Node>(&self, idx: usize) -> &T {
        match self.try_node::<T>(idx) {
            Ok(node) => node,
            Err(e) => {
                panic!(
                    "Invalid node type. expected {}:{}, found {}:{}",
                    e.ty,
                    e.subty,
                    T::ty(),
                    T::subty(),
                );
            }
        }
    }

    pub fn int_par(&self, par: IntPar) -> i32 {
        unsafe { self.eqtb[INT_BASE + par as usize].b32.s1 }
    }

    pub fn set_int_par(&mut self, par: IntPar, val: i32) {
        self.eqtb[INT_BASE + par as usize].b32.s1 = val
    }

    pub fn cat_code(&self, p: usize) -> Result<CatCode, i32> {
        let val = unsafe { self.eqtb[CAT_CODE_BASE + p].b32.s1 };
        CatCode::try_from(val)
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

#[no_mangle]
pub extern "C" fn eqtb_top() -> i32 {
    ENGINE_CTX.with_borrow(|engine| engine.eqtb_top)
}

#[no_mangle]
pub extern "C" fn set_eqtb_top(val: i32) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.eqtb_top = val)
}

#[no_mangle]
pub extern "C" fn eqtb(idx: usize) -> MemoryWord {
    ENGINE_CTX.with_borrow(|engine| engine.eqtb[idx])
}

#[no_mangle]
pub extern "C" fn set_eqtb(idx: usize, val: MemoryWord) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.eqtb[idx] = val)
}

#[no_mangle]
pub extern "C" fn eqtb_ptr(idx: usize) -> *mut MemoryWord {
    ENGINE_CTX.with_borrow_mut(|engine| ptr::from_mut(&mut engine.eqtb[idx]))
}

#[no_mangle]
pub extern "C" fn resize_eqtb(len: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| {
        engine.eqtb.resize(
            len,
            MemoryWord {
                ptr: ptr::null_mut(),
            },
        )
    })
}

#[no_mangle]
pub extern "C" fn clear_eqtb() {
    ENGINE_CTX.with_borrow_mut(|engine| engine.eqtb.clear())
}

#[no_mangle]
pub extern "C" fn mem(idx: usize) -> MemoryWord {
    ENGINE_CTX.with_borrow(|engine| engine.mem[idx])
}

#[no_mangle]
pub extern "C" fn set_mem(idx: usize, val: MemoryWord) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.mem[idx] = val)
}

#[no_mangle]
pub extern "C" fn mem_ptr(idx: usize) -> *mut MemoryWord {
    ENGINE_CTX.with_borrow_mut(|engine| ptr::from_mut(&mut engine.mem[idx]))
}

#[no_mangle]
pub extern "C" fn resize_mem(len: usize) {
    ENGINE_CTX.with_borrow_mut(|engine| {
        engine.mem.resize(
            len,
            MemoryWord {
                ptr: ptr::null_mut(),
            },
        )
    })
}

#[no_mangle]
pub extern "C" fn clear_mem() {
    ENGINE_CTX.with_borrow_mut(|engine| engine.mem.clear())
}

#[no_mangle]
pub extern "C" fn prim(idx: usize) -> B32x2 {
    ENGINE_CTX.with_borrow(|engine| engine.prim[idx])
}

#[no_mangle]
pub extern "C" fn set_prim(idx: usize, val: B32x2) {
    ENGINE_CTX.with_borrow_mut(|engine| engine.prim[idx] = val)
}

#[no_mangle]
pub extern "C" fn prim_ptr(idx: usize) -> *mut B32x2 {
    ENGINE_CTX.with_borrow_mut(|engine| ptr::from_mut(&mut engine.prim[idx]))
}

pub fn rs_gettexstring(globals: &mut Globals<'_, '_>, s: StrNumber) -> String {
    if s < 0x10000 {
        return String::new();
    }

    let str = globals.strings.str(s - 0x10000);

    String::from_utf16_lossy(str)
}

#[no_mangle]
pub unsafe extern "C" fn gettexstring(s: StrNumber) -> *mut libc::c_char {
    let str = Globals::with(|globals| rs_gettexstring(globals, s));
    let out = unsafe { libc::malloc(str.len() + 1) }.cast::<libc::c_char>();
    unsafe { ptr::copy_nonoverlapping(str.as_ptr().cast(), out, str.len()) };
    unsafe { out.add(str.len()).write(0) };
    out
}
