use crate::ty::StrNumber;
use std::cell::RefCell;

pub const MAX_IN_OPEN: usize = 15;

thread_local! {
    pub static FILE_CTX: RefCell<FileCtx> = const { RefCell::new(FileCtx::new()) };
}

pub struct FileCtx {
    pub(crate) in_open: i32,
    pub(crate) full_source_filename_stack: Vec<StrNumber>,
    pub(crate) line: i32,
    pub(crate) line_stack: Vec<i32>,
}

impl FileCtx {
    const fn new() -> FileCtx {
        FileCtx {
            in_open: 0,
            full_source_filename_stack: Vec::new(),
            line: 0,
            line_stack: Vec::new(),
        }
    }
}

#[no_mangle]
pub extern "C" fn in_open() -> i32 {
    FILE_CTX.with_borrow(|files| files.in_open)
}

#[no_mangle]
pub extern "C" fn set_in_open(val: i32) {
    FILE_CTX.with_borrow_mut(|files| files.in_open = val)
}

#[no_mangle]
pub extern "C" fn full_source_filename_stack(idx: usize) -> StrNumber {
    FILE_CTX.with_borrow(|files| files.full_source_filename_stack[idx])
}

#[no_mangle]
pub extern "C" fn set_full_source_filename_stack(idx: usize, val: StrNumber) {
    FILE_CTX.with_borrow_mut(|files| {
        if files.full_source_filename_stack.len() < idx + 1 {
            files.full_source_filename_stack.resize(idx + 1, 0);
        }
        files.full_source_filename_stack[idx] = val
    })
}

#[no_mangle]
pub extern "C" fn clear_full_source_filename_stack() {
    FILE_CTX.with_borrow_mut(|files| files.full_source_filename_stack.clear())
}

#[no_mangle]
pub extern "C" fn line() -> i32 {
    FILE_CTX.with_borrow(|files| files.line)
}

#[no_mangle]
pub extern "C" fn set_line(val: i32) {
    FILE_CTX.with_borrow_mut(|files| files.line = val)
}

#[no_mangle]
pub extern "C" fn line_stack(idx: usize) -> i32 {
    FILE_CTX.with_borrow(|files| files.line_stack[idx])
}

#[no_mangle]
pub extern "C" fn set_line_stack(idx: usize, val: i32) {
    FILE_CTX.with_borrow_mut(|files| {
        if files.line_stack.len() < idx + 1 {
            files.line_stack.resize(idx + 1, 0);
        }
        files.line_stack[idx] = val;
    })
}

#[no_mangle]
pub extern "C" fn clear_line_stack() {
    FILE_CTX.with_borrow_mut(|files| files.line_stack.clear())
}
