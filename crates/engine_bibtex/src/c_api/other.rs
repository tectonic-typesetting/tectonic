use crate::c_api::{xbuf::XBuf, FieldLoc, FnDefLoc, HashPointer2, StrNumber, WizFnLoc};
use std::cell::RefCell;

const WIZ_FN_SPACE: usize = 3000;
const MAX_FIELDS: usize = 17250;

pub struct OtherData {
    wiz_functions: XBuf<HashPointer2>,
    wiz_def_ptr: WizFnLoc,
    field_info: XBuf<StrNumber>,
    num_fields: FieldLoc,
    num_pre_defined_fields: FieldLoc,
    crossref_num: FieldLoc,
}

impl OtherData {
    fn new() -> OtherData {
        OtherData {
            wiz_functions: XBuf::new(WIZ_FN_SPACE),
            wiz_def_ptr: 0,
            field_info: XBuf::new(MAX_FIELDS),
            num_fields: 0,
            num_pre_defined_fields: 0,
            crossref_num: 0,
        }
    }

    pub fn field(&self, pos: usize) -> StrNumber {
        self.field_info[pos]
    }

    pub fn num_fields(&self) -> FieldLoc {
        self.num_fields
    }

    pub fn set_num_fields(&mut self, val: FieldLoc) {
        self.num_fields = val;
    }

    pub fn set_pre_defined_fields(&mut self, val: FieldLoc) {
        self.num_pre_defined_fields = val;
    }

    pub fn check_field_overflow(&mut self, fields: usize) {
        let start_fields = self.field_info.len();
        if fields > self.field_info.len() {
            self.field_info.grow(MAX_FIELDS);
            self.field_info[start_fields..].fill(0);
        }
    }

    pub fn set_crossref_num(&mut self, val: FieldLoc) {
        self.crossref_num = val;
    }
}

thread_local! {
    pub static OTHER: RefCell<OtherData> = RefCell::new(OtherData::new());
}

pub fn reset() {
    OTHER.with(|other| *other.borrow_mut() = OtherData::new());
}

pub fn with_other<T>(f: impl FnOnce(&OtherData) -> T) -> T {
    OTHER.with(|other| f(&other.borrow()))
}

pub fn with_other_mut<T>(f: impl FnOnce(&mut OtherData) -> T) -> T {
    OTHER.with(|other| f(&mut other.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn wiz_functions(pos: WizFnLoc) -> HashPointer2 {
    with_other(|other| other.wiz_functions[pos])
}

#[no_mangle]
pub extern "C" fn set_wiz_functions(pos: WizFnLoc, val: HashPointer2) {
    with_other_mut(|other| other.wiz_functions[pos] = val)
}

#[no_mangle]
pub extern "C" fn wiz_def_ptr() -> WizFnLoc {
    with_other(|other| other.wiz_def_ptr)
}

#[no_mangle]
pub extern "C" fn set_wiz_def_ptr(val: WizFnLoc) {
    with_other_mut(|other| other.wiz_def_ptr = val)
}

#[no_mangle]
pub extern "C" fn check_grow_wiz(ptr: FnDefLoc) {
    with_other_mut(|other| {
        while ptr + other.wiz_def_ptr > other.wiz_functions.len() {
            other.wiz_functions.grow(WIZ_FN_SPACE)
        }
    })
}

#[no_mangle]
pub extern "C" fn field_info(pos: FieldLoc) -> StrNumber {
    with_other(|other| other.field_info[pos])
}

#[no_mangle]
pub extern "C" fn set_field_info(pos: FieldLoc, val: StrNumber) {
    with_other_mut(|other| other.field_info[pos] = val)
}

#[no_mangle]
pub extern "C" fn check_field_overflow(total_fields: usize) {
    with_other_mut(|other| other.check_field_overflow(total_fields))
}

#[no_mangle]
pub extern "C" fn max_fields() -> usize {
    with_other(|other| other.field_info.len())
}

#[no_mangle]
pub extern "C" fn num_fields() -> FieldLoc {
    with_other(|other| other.num_fields)
}

#[no_mangle]
pub extern "C" fn set_num_fields(val: FieldLoc) {
    with_other_mut(|other| other.num_fields = val)
}

#[no_mangle]
pub extern "C" fn num_pre_defined_fields() -> FieldLoc {
    with_other(|other| other.num_pre_defined_fields)
}

#[no_mangle]
pub extern "C" fn crossref_num() -> FieldLoc {
    with_other(|other| other.crossref_num)
}
