use crate::{
    c_api::{
        buffer::{with_buffers_mut, BufTy, GlobalBuffer},
        char_info::LexClass,
        log::{eat_bib_print, write_log_file},
        peekable::{rs_input_ln, PeekableInput},
        scan::Scan,
        xbuf::XBuf,
        BibNumber, StrNumber,
    },
    BibtexError,
};
use std::{cell::RefCell, ptr::NonNull};

const MAX_BIB_FILES: usize = 20;

pub struct BibData {
    bib_file: XBuf<Option<NonNull<PeekableInput>>>,
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

    fn cur_bib_file(&self) -> Option<NonNull<PeekableInput>> {
        self.bib_file[self.bib_ptr]
    }

    fn set_cur_bib_file(&mut self, input: Option<NonNull<PeekableInput>>) {
        self.bib_file[self.bib_ptr] = input;
    }

    pub fn line_num(&self) -> i32 {
        self.bib_line_num
    }

    pub fn set_line_num(&mut self, val: i32) {
        self.bib_line_num = val;
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

pub fn with_bibs_mut<T>(f: impl FnOnce(&mut BibData) -> T) -> T {
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
pub extern "C" fn cur_bib_file() -> Option<NonNull<PeekableInput>> {
    with_bibs(|bibs| bibs.cur_bib_file())
}

#[no_mangle]
pub extern "C" fn set_cur_bib_file(input: Option<NonNull<PeekableInput>>) {
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

pub fn rs_eat_bib_white_space(buffers: &mut GlobalBuffer) -> bool {
    let mut init = buffers.init(BufTy::Base);
    while !Scan::new()
        .not_class(LexClass::Whitespace)
        .scan_till(buffers, init)
    {
        if !rs_input_ln(
            unsafe { cur_bib_file().map(|mut ptr| ptr.as_mut()) },
            buffers,
        ) {
            return false;
        }

        with_bibs_mut(|bibs| {
            bibs.set_line_num(bibs.line_num() + 1);
        });
        buffers.set_offset(BufTy::Base, 2, 0);
        init = buffers.init(BufTy::Base);
    }
    true
}

#[no_mangle]
pub extern "C" fn eat_bib_white_space() -> bool {
    with_buffers_mut(|buffers| rs_eat_bib_white_space(buffers))
}

pub fn compress_bib_white(
    buffers: &mut GlobalBuffer,
    at_bib_command: bool,
) -> Result<bool, BibtexError> {
    if buffers.offset(BufTy::Ex, 1) == buffers.len() {
        write_log_file(&format!("Field filled up at ' ', reallocating.\n"));
        buffers.grow_all();
    }

    buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b' ');
    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
    let last = buffers.init(BufTy::Base);
    while !Scan::new()
        .not_class(LexClass::Whitespace)
        .scan_till(buffers, last)
    {
        if !rs_input_ln(
            unsafe { cur_bib_file().map(|mut ptr| ptr.as_mut()) },
            buffers,
        ) {
            if !eat_bib_print(at_bib_command) {
                return Err(BibtexError::Fatal);
            }
            return Ok(false);
        }

        with_bibs_mut(|bibs| {
            bibs.set_line_num(bibs.line_num() + 1);
        });
        with_buffers_mut(|buffers| {
            buffers.set_offset(BufTy::Base, 2, 0);
        });
    }

    return Ok(true);
}
