use crate::c_api::{xbuf::XBuf, FieldLoc, FnDefLoc, HashPointer, StrNumber, WizFnLoc};
use std::cell::RefCell;

const WIZ_FN_SPACE: usize = 3000;
const MAX_FIELDS: usize = 17250;

pub struct OtherData {
    wiz_functions: XBuf<HashPointer>,
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

    pub fn max_fields(&self) -> usize {
        self.field_info.len()
    }

    pub fn field(&self, pos: usize) -> StrNumber {
        self.field_info[pos]
    }

    pub fn set_field(&mut self, pos: usize, s: StrNumber) {
        self.field_info[pos] = s
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
        if fields > self.field_info.len() {
            self.field_info.grow(MAX_FIELDS);
        }
    }

    pub fn crossref_num(&self) -> FieldLoc {
        self.crossref_num
    }

    pub fn set_crossref_num(&mut self, val: FieldLoc) {
        self.crossref_num = val;
    }

    pub fn wiz_function(&self, pos: usize) -> HashPointer {
        self.wiz_functions[pos]
    }

    pub fn set_wiz_function(&mut self, pos: usize, val: HashPointer) {
        self.wiz_functions[pos] = val
    }

    pub fn wiz_def_ptr(&self) -> WizFnLoc {
        self.wiz_def_ptr
    }

    pub fn set_wiz_def_ptr(&mut self, ptr: WizFnLoc) {
        self.wiz_def_ptr = ptr;
    }

    pub fn check_wiz_overflow(&mut self, ptr: FnDefLoc) {
        while ptr + self.wiz_def_ptr > self.wiz_functions.len() {
            self.wiz_functions.grow(WIZ_FN_SPACE)
        }
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
