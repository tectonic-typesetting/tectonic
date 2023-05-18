use crate::c_api::peekable::PeekableInput;
use crate::c_api::{AuxNumber, StrNumber};
use std::cell::RefCell;
use std::ptr;

const AUX_STACK_SIZE: usize = 20;

pub struct AuxData {
    aux_list: [StrNumber; AUX_STACK_SIZE + 1],
    aux_file: [*mut PeekableInput; AUX_STACK_SIZE + 1],
    aux_ln_stack: [i32; AUX_STACK_SIZE + 1],
    aux_ptr: AuxNumber,
}

impl AuxData {
    fn new() -> AuxData {
        AuxData {
            aux_list: [0; AUX_STACK_SIZE + 1],
            aux_file: [ptr::null_mut(); AUX_STACK_SIZE + 1],
            aux_ln_stack: [0; AUX_STACK_SIZE + 1],
            aux_ptr: 0,
        }
    }

    fn at_ptr(&self) -> StrNumber {
        self.aux_list[self.aux_ptr as usize]
    }

    fn set_at_ptr(&mut self, num: StrNumber) {
        self.aux_list[self.aux_ptr as usize] = num;
    }

    fn file_at_ptr(&self) -> *mut PeekableInput {
        self.aux_file[self.aux_ptr as usize]
    }

    fn set_file_at_ptr(&mut self, file: *mut PeekableInput) {
        self.aux_file[self.aux_ptr as usize] = file;
    }

    fn ln_at_ptr(&self) -> i32 {
        self.aux_ln_stack[self.aux_ptr as usize]
    }

    fn set_ln_at_ptr(&mut self, ln: i32) {
        self.aux_ln_stack[self.aux_ptr as usize] = ln;
    }
}

thread_local! {
    pub static AUX: RefCell<AuxData> = RefCell::new(AuxData::new());
}

pub fn reset() {
    AUX.with(|aux| *aux.borrow_mut() = AuxData::new());
}

pub fn with_aux<T>(f: impl FnOnce(&AuxData) -> T) -> T {
    AUX.with(|aux| f(&aux.borrow()))
}

pub fn with_aux_mut<T>(f: impl FnOnce(&mut AuxData) -> T) -> T {
    AUX.with(|aux| f(&mut aux.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn cur_aux() -> StrNumber {
    with_aux(|aux| aux.at_ptr())
}

#[no_mangle]
pub extern "C" fn set_cur_aux(num: StrNumber) {
    with_aux_mut(|aux| aux.set_at_ptr(num))
}

#[no_mangle]
pub extern "C" fn cur_aux_file() -> *mut PeekableInput {
    with_aux(|aux| aux.file_at_ptr())
}

#[no_mangle]
pub extern "C" fn set_cur_aux_file(file: *mut PeekableInput) {
    with_aux_mut(|aux| aux.set_file_at_ptr(file))
}

#[no_mangle]
pub extern "C" fn cur_aux_ln() -> i32 {
    with_aux(|aux| aux.ln_at_ptr())
}

#[no_mangle]
pub extern "C" fn set_cur_aux_ln(ln: i32) {
    with_aux_mut(|aux| aux.set_ln_at_ptr(ln))
}

#[no_mangle]
pub extern "C" fn aux_ptr() -> AuxNumber {
    with_aux(|aux| aux.aux_ptr)
}

#[no_mangle]
pub extern "C" fn set_aux_ptr(num: AuxNumber) {
    with_aux_mut(|aux| aux.aux_ptr = num)
}
