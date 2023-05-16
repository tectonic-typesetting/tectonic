use crate::c_api::{xcalloc_zeroed, CiteNumber, HashPointer2, StrNumber};
use std::cell::RefCell;
use std::mem;

const MAX_CITES: usize = 750;

pub struct CiteInfo {
    cite_list: &'static mut [StrNumber],
    cite_info: &'static mut [StrNumber],
    type_list: &'static mut [HashPointer2],
    entry_exists: &'static mut [bool],
    cite_ptr: CiteNumber,
}

impl CiteInfo {
    fn new() -> CiteInfo {
        CiteInfo {
            cite_list: unsafe { xcalloc_zeroed(MAX_CITES, mem::size_of::<StrNumber>()) },
            cite_info: unsafe { xcalloc_zeroed(MAX_CITES, mem::size_of::<StrNumber>()) },
            type_list: unsafe { xcalloc_zeroed(MAX_CITES, mem::size_of::<HashPointer2>()) },
            entry_exists: unsafe { xcalloc_zeroed(MAX_CITES, mem::size_of::<bool>()) },
            cite_ptr: 0,
        }
    }

    fn grow(&mut self) {
        let new_cites = unsafe {
            xcalloc_zeroed(
                self.cite_list.len() + MAX_CITES,
                mem::size_of::<StrNumber>(),
            )
        };
        new_cites.copy_from_slice(self.cite_list);
        unsafe { libc::free((self.cite_list as *mut [_]).cast()) };
        self.cite_list = new_cites;

        let new_cites = unsafe {
            xcalloc_zeroed(
                self.cite_info.len() + MAX_CITES,
                mem::size_of::<StrNumber>(),
            )
        };
        new_cites.copy_from_slice(self.cite_info);
        unsafe { libc::free((self.cite_info as *mut [_]).cast()) };
        self.cite_info = new_cites;

        let new_cites = unsafe {
            xcalloc_zeroed(
                self.type_list.len() + MAX_CITES,
                mem::size_of::<HashPointer2>(),
            )
        };
        new_cites.copy_from_slice(self.type_list);
        unsafe { libc::free((self.type_list as *mut [_]).cast()) };
        self.type_list = new_cites;

        let new_cites =
            unsafe { xcalloc_zeroed(self.entry_exists.len() + MAX_CITES, mem::size_of::<bool>()) };
        new_cites.copy_from_slice(self.entry_exists);
        unsafe { libc::free((self.entry_exists as *mut [_]).cast()) };
        self.entry_exists = new_cites;
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
