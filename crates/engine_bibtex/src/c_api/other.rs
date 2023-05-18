use crate::c_api::xbuf::XBuf;
use crate::c_api::{FieldLoc, FnDefLoc, HashPointer2, StrNumber, WizFnLoc};
use std::cell::RefCell;

const WIZ_FN_SPACE: usize = 3000;
const MAX_FIELDS: usize = 17250;

pub struct OtherData {
    wiz_functions: XBuf<HashPointer2>,
    wiz_def_ptr: WizFnLoc,
    field_info: XBuf<StrNumber>,
}

impl OtherData {
    fn new() -> OtherData {
        OtherData {
            wiz_functions: XBuf::new(WIZ_FN_SPACE),
            wiz_def_ptr: 0,
            field_info: XBuf::new(MAX_FIELDS),
        }
    }

    pub fn field(&self, pos: usize) -> StrNumber {
        self.field_info[pos]
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
    with_other(|other| other.wiz_functions[pos as usize])
}

#[no_mangle]
pub extern "C" fn set_wiz_functions(pos: WizFnLoc, val: HashPointer2) {
    with_other_mut(|other| other.wiz_functions[pos as usize] = val)
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
        while ptr + other.wiz_def_ptr > other.wiz_functions.len() as i32 {
            other.wiz_functions.grow(WIZ_FN_SPACE)
        }
    })
}

#[no_mangle]
pub extern "C" fn field_info(pos: FieldLoc) -> StrNumber {
    with_other(|other| other.field_info[pos as usize])
}

#[no_mangle]
pub extern "C" fn set_field_info(pos: FieldLoc, val: StrNumber) {
    with_other_mut(|other| other.field_info[pos as usize] = val)
}

#[no_mangle]
pub extern "C" fn check_field_overflow(total_fields: i32) {
    with_other_mut(|other| {
        let start_fields = other.field_info.len();
        if total_fields as usize > other.field_info.len() {
            other.field_info.grow(MAX_FIELDS);
            other.field_info[start_fields..].fill(0);
        }
    })
}
