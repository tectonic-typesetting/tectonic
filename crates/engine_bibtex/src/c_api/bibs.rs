use std::cell::RefCell;
use std::mem;
use crate::c_api::{BibNumber, StrNumber, xcalloc_zeroed};
use crate::c_api::peekable::PeekableInput;

const MAX_BIB_FILES: usize = 20;

pub struct BibData {
    bib_file: &'static mut [*mut PeekableInput],
    bib_list: &'static mut [StrNumber],
    bib_ptr: BibNumber,
    preamble: &'static mut [StrNumber],
    preamble_ptr: BibNumber,
}

impl BibData {
    fn new() -> BibData {
        BibData {
            bib_file: unsafe { xcalloc_zeroed(MAX_BIB_FILES + 1, mem::size_of::<*mut PeekableInput>()) },
            bib_list: unsafe { xcalloc_zeroed(MAX_BIB_FILES + 1, mem::size_of::<StrNumber>()) },
            bib_ptr: 0,
            preamble: unsafe { xcalloc_zeroed(MAX_BIB_FILES + 1, mem::size_of::<StrNumber>()) },
            preamble_ptr: 0,
        }
    }

    fn cur_bib(&self) -> StrNumber {
        self.bib_list[self.bib_ptr as usize]
    }

    fn set_cur_bib(&mut self, num: StrNumber) {
        self.bib_list[self.bib_ptr as usize] = num;
    }

    fn cur_bib_file(&self) -> *mut PeekableInput {
        self.bib_file[self.bib_ptr as usize]
    }

    fn set_cur_bib_file(&mut self, input: *mut PeekableInput) {
        self.bib_file[self.bib_ptr as usize] = input;
    }

    fn grow(&mut self) {
        let new_files = unsafe {
            xcalloc_zeroed(
                self.bib_file.len() + MAX_BIB_FILES,
                mem::size_of::<*mut PeekableInput>(),
            )
        };
        new_files.copy_from_slice(self.bib_file);
        unsafe { libc::free((self.bib_file as *mut [_]).cast()) };
        self.bib_file = new_files;

        let new_bibs = unsafe {
            xcalloc_zeroed(
                self.bib_list.len() + MAX_BIB_FILES,
                mem::size_of::<StrNumber>(),
            )
        };
        new_bibs.copy_from_slice(self.bib_list);
        unsafe { libc::free((self.bib_list as *mut [_]).cast()) };
        self.bib_list = new_bibs;

        /*
        BIB_XRETALLOC("s_preamble", s_preamble, str_number,
                      max_bib_files, max_bib_files + MAX_BIB_FILES);
         */
    }
}

thread_local! {
    pub static BIBS: RefCell<BibData> = RefCell::new(BibData::new());
}

fn with_bibs<T>(f: impl FnOnce(&BibData) -> T) -> T {
    BIBS.with(|bibs| f(&bibs.borrow()))
}

fn with_bibs_mut<T>(f: impl FnOnce(&mut BibData) -> T) -> T {
    BIBS.with(|bibs| f(&mut bibs.borrow_mut()))
}

pub fn reset() {
    BIBS.with(|bibs| *bibs.borrow_mut() = BibData::new());
}

#[no_mangle]
pub extern "C" fn cur_bib() -> StrNumber {
    with_bibs(|bibs| bibs.cur_bib())
}

#[no_mangle]
pub extern "C" fn set_cur_bib(num: StrNumber) {
    with_bibs_mut(|bibs| bibs.set_cur_bib(num))
}

#[no_mangle]
pub extern "C" fn cur_bib_file() -> *mut PeekableInput {
    with_bibs(|bibs| bibs.cur_bib_file())
}

#[no_mangle]
pub extern "C" fn set_cur_bib_file(input: *mut PeekableInput) {
    with_bibs_mut(|bibs| bibs.set_cur_bib_file(input))
}

#[no_mangle]
pub extern "C" fn bib_ptr() -> BibNumber {
    with_bibs(|bibs| bibs.bib_ptr)
}

#[no_mangle]
pub extern "C" fn set_bib_ptr(num: BibNumber) {
    with_bibs_mut(|bibs| bibs.bib_ptr = num)
}

#[no_mangle]
pub extern "C" fn check_bib_files(ptr: BibNumber) {
    with_bibs_mut(|bibs| {
        if ptr as usize == bibs.bib_list.len() {
            bibs.grow();
        }
    })
}

#[no_mangle]
pub extern "C" fn add_preamble(num: StrNumber) {
    with_bibs_mut(|bibs| {
        bibs.preamble[bibs.preamble_ptr as usize] = num;
        bibs.preamble_ptr += 1;
    })
}

#[no_mangle]
pub extern "C" fn cur_preamble() -> StrNumber {
    with_bibs(|bibs| bibs.preamble[bibs.preamble_ptr as usize])
}

#[no_mangle]
pub extern "C" fn preamble_ptr() -> BibNumber {
    with_bibs(|bibs| bibs.preamble_ptr)
}

#[no_mangle]
pub extern "C" fn set_preamble_ptr(num: BibNumber) {
    with_bibs_mut(|bibs| bibs.preamble_ptr = num)
}
