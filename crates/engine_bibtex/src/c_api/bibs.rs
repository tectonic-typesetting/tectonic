use crate::c_api::{peekable::PeekableInput, xbuf::XBuf, BibNumber, StrNumber};
use std::cell::RefCell;

const MAX_BIB_FILES: usize = 20;

pub struct BibData {
    bib_file: XBuf<*mut PeekableInput>,
    bib_list: XBuf<StrNumber>,
    bib_ptr: BibNumber,
    bib_line_num: i32,
    preamble: XBuf<StrNumber>,
    preamble_ptr: BibNumber,
}

impl BibData {
    fn new() -> BibData {
        BibData {
            bib_file: XBuf::new(MAX_BIB_FILES),
            bib_list: XBuf::new(MAX_BIB_FILES),
            bib_ptr: 0,
            bib_line_num: 0,
            preamble: XBuf::new(MAX_BIB_FILES),
            preamble_ptr: 0,
        }
    }

    fn cur_bib(&self) -> StrNumber {
        self.bib_list[self.bib_ptr]
    }

    fn set_cur_bib(&mut self, num: StrNumber) {
        self.bib_list[self.bib_ptr] = num;
    }

    fn cur_bib_file(&self) -> *mut PeekableInput {
        self.bib_file[self.bib_ptr]
    }

    fn set_cur_bib_file(&mut self, input: *mut PeekableInput) {
        self.bib_file[self.bib_ptr] = input;
    }

    fn grow(&mut self) {
        self.bib_list.grow(MAX_BIB_FILES);
        self.bib_file.grow(MAX_BIB_FILES);
        self.preamble.grow(MAX_BIB_FILES);
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
        if ptr == bibs.bib_list.len() {
            bibs.grow();
        }
    })
}

#[no_mangle]
pub extern "C" fn add_preamble(num: StrNumber) {
    with_bibs_mut(|bibs| {
        bibs.preamble[bibs.preamble_ptr] = num;
        bibs.preamble_ptr += 1;
    })
}

#[no_mangle]
pub extern "C" fn cur_preamble() -> StrNumber {
    with_bibs(|bibs| bibs.preamble[bibs.preamble_ptr])
}

#[no_mangle]
pub extern "C" fn preamble_ptr() -> BibNumber {
    with_bibs(|bibs| bibs.preamble_ptr)
}

#[no_mangle]
pub extern "C" fn set_preamble_ptr(num: BibNumber) {
    with_bibs_mut(|bibs| bibs.preamble_ptr = num)
}

#[no_mangle]
pub extern "C" fn bib_line_num() -> i32 {
    with_bibs(|bibs| bibs.bib_line_num)
}

#[no_mangle]
pub extern "C" fn set_bib_line_num(num: i32) {
    with_bibs_mut(|bibs| bibs.bib_line_num = num)
}
