use crate::c_api::xbuf::XBuf;
use crate::c_api::{CiteNumber, HashPointer2, StrNumber};
use std::cell::RefCell;

pub const MAX_CITES: usize = 750;

pub struct CiteInfo {
    cite_list: XBuf<StrNumber>,
    cite_info: XBuf<StrNumber>,
    type_list: XBuf<HashPointer2>,
    entry_exists: XBuf<bool>,
    cite_ptr: CiteNumber,
}

impl CiteInfo {
    fn new() -> CiteInfo {
        CiteInfo {
            cite_list: XBuf::new(MAX_CITES),
            cite_info: XBuf::new(MAX_CITES),
            type_list: XBuf::new(MAX_CITES),
            entry_exists: XBuf::new(MAX_CITES),
            cite_ptr: 0,
        }
    }

    fn grow(&mut self) {
        self.cite_list.grow(MAX_CITES);
        self.cite_info.grow(MAX_CITES);
        self.type_list.grow(MAX_CITES);
        self.entry_exists.grow(MAX_CITES);
    }

    pub fn get_cite(&self, offset: usize) -> StrNumber {
        self.cite_list[offset]
    }

    pub fn set_cite(&mut self, offset: usize, num: StrNumber) {
        self.cite_list[offset] = num;
    }

    pub fn get_info(&self, offset: usize) -> StrNumber {
        self.cite_info[offset]
    }

    pub fn set_info(&mut self, offset: usize, num: StrNumber) {
        self.cite_info[offset] = num;
    }

    pub fn get_type(&self, offset: usize) -> HashPointer2 {
        self.type_list[offset]
    }

    pub fn set_type(&mut self, offset: usize, ty: HashPointer2) {
        self.type_list[offset] = ty;
    }

    pub fn get_exists(&self, offset: usize) -> bool {
        self.entry_exists[offset]
    }

    pub fn set_exists(&mut self, offset: usize, exists: bool) {
        self.entry_exists[offset] = exists;
    }

    pub fn ptr(&self) -> CiteNumber {
        self.cite_ptr
    }

    pub fn set_ptr(&mut self, ptr: CiteNumber) {
        self.cite_ptr = ptr;
    }
}

thread_local! {
    pub static CITE_INFO: RefCell<CiteInfo> = RefCell::new(CiteInfo::new());
}

pub fn reset() {
    CITE_INFO.with(|ci| *ci.borrow_mut() = CiteInfo::new());
}

pub fn with_cites<T>(f: impl FnOnce(&CiteInfo) -> T) -> T {
    CITE_INFO.with(|ci| f(&ci.borrow()))
}

pub fn with_cites_mut<T>(f: impl FnOnce(&mut CiteInfo) -> T) -> T {
    CITE_INFO.with(|ci| f(&mut ci.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn quick_sort(left_end: CiteNumber, right_end: CiteNumber) {
    with_cites_mut(|cites| cites.cite_info[left_end as usize..right_end as usize].sort())
}

#[no_mangle]
pub extern "C" fn cite_list(num: CiteNumber) -> StrNumber {
    with_cites(|cites| cites.get_cite(num as usize))
}

#[no_mangle]
pub extern "C" fn set_cite_list(num: CiteNumber, str: StrNumber) {
    with_cites_mut(|cites| cites.set_cite(num as usize, str))
}

#[no_mangle]
pub extern "C" fn cite_ptr() -> CiteNumber {
    with_cites(|cites| cites.ptr())
}

#[no_mangle]
pub extern "C" fn set_cite_ptr(num: CiteNumber) {
    with_cites_mut(|cites| cites.set_ptr(num))
}

#[no_mangle]
pub extern "C" fn check_cite_overflow(last_cite: CiteNumber) {
    with_cites_mut(|cites| {
        if last_cite as usize == cites.cite_list.len() {
            cites.grow();
        }
    })
}

#[no_mangle]
pub extern "C" fn max_cites() -> usize {
    with_cites(|cites| cites.cite_list.len())
}

#[no_mangle]
pub extern "C" fn cite_info(num: CiteNumber) -> StrNumber {
    with_cites(|cites| cites.get_info(num as usize))
}

#[no_mangle]
pub extern "C" fn set_cite_info(num: CiteNumber, info: StrNumber) {
    with_cites_mut(|cites| cites.set_info(num as usize, info))
}

#[no_mangle]
pub extern "C" fn type_list(num: CiteNumber) -> HashPointer2 {
    with_cites(|cites| cites.get_type(num as usize))
}

#[no_mangle]
pub extern "C" fn set_type_list(num: CiteNumber, ty: HashPointer2) {
    with_cites_mut(|cites| cites.set_type(num as usize, ty))
}

#[no_mangle]
pub extern "C" fn entry_exists(num: CiteNumber) -> bool {
    with_cites(|cites| cites.get_exists(num as usize))
}

#[no_mangle]
pub extern "C" fn set_entry_exists(num: CiteNumber, exists: bool) {
    with_cites_mut(|cites| cites.set_exists(num as usize, exists))
}
