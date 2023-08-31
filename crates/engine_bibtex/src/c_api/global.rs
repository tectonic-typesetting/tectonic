use crate::c_api::{xbuf::XBuf, ASCIICode, StrNumber};
use std::cell::RefCell;

const MAX_GLOB_STRS: usize = 10;
pub const GLOB_STR_SIZE: usize = 20000;

pub struct GlobalData {
    glb_bib_str_ptr: XBuf<StrNumber>,
    global_strs: XBuf<ASCIICode>,
    glb_str_end: XBuf<usize>,
    num_glb_strs: i32,
}

impl GlobalData {
    fn new() -> GlobalData {
        GlobalData {
            glb_bib_str_ptr: XBuf::new(MAX_GLOB_STRS),
            global_strs: XBuf::new((GLOB_STR_SIZE + 1) * MAX_GLOB_STRS),
            glb_str_end: XBuf::new(MAX_GLOB_STRS),
            num_glb_strs: 0,
        }
    }

    fn grow(&mut self) {
        self.glb_bib_str_ptr.grow(MAX_GLOB_STRS);
        self.global_strs.grow((GLOB_STR_SIZE + 1) * MAX_GLOB_STRS);
        self.glb_str_end.grow(MAX_GLOB_STRS);
    }

    pub fn str(&self, pos: usize) -> &[ASCIICode] {
        let spos = pos * (GLOB_STR_SIZE + 1);
        &self.global_strs[spos..spos + self.glb_str_end[pos]]
    }

    pub fn str_ptr(&self, pos: usize) -> StrNumber {
        self.glb_bib_str_ptr[pos]
    }

    pub fn set_str_ptr(&mut self, pos: usize, val: StrNumber) {
        self.glb_bib_str_ptr[pos] = val;
    }

    pub fn set_str(&mut self, pos: usize, val: &[ASCIICode]) {
        let spos = pos * (GLOB_STR_SIZE + 1);
        self.global_strs[spos..spos + val.len()].copy_from_slice(val);
        self.glb_str_end[pos] = val.len();
    }

    pub fn set_str_end(&mut self, pos: usize, val: usize) {
        self.glb_str_end[pos] = val;
    }
}

thread_local! {
    pub static GLOBALS: RefCell<GlobalData> = RefCell::new(GlobalData::new());
}

pub fn reset() {
    GLOBALS.with(|globals| *globals.borrow_mut() = GlobalData::new());
}

fn with_globals<T>(f: impl FnOnce(&GlobalData) -> T) -> T {
    GLOBALS.with(|globals| f(&globals.borrow()))
}

pub fn with_globals_mut<T>(f: impl FnOnce(&mut GlobalData) -> T) -> T {
    GLOBALS.with(|globals| f(&mut globals.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn num_glb_strs() -> i32 {
    with_globals(|globals| globals.num_glb_strs)
}

#[no_mangle]
pub extern "C" fn set_num_glb_strs(val: i32) {
    with_globals_mut(|globals| globals.num_glb_strs = val)
}

#[no_mangle]
pub extern "C" fn check_grow_global_strs() {
    with_globals_mut(|globals| {
        if globals.num_glb_strs as usize == globals.glb_bib_str_ptr.len() {
            globals.grow();
        }
    })
}
