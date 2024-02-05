use crate::{xbuf::XBuf, FieldLoc, FnDefLoc, HashPointer, StrNumber, WizFnLoc};

const WIZ_FN_SPACE: usize = 3000;
const MAX_FIELDS: usize = 17250;

pub(crate) struct OtherData {
    wiz_functions: XBuf<HashPointer>,
    wiz_def_ptr: WizFnLoc,
    field_info: XBuf<StrNumber>,
    num_fields: FieldLoc,
    num_pre_defined_fields: FieldLoc,
    crossref_num: FieldLoc,
}

impl OtherData {
    pub fn new() -> OtherData {
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

    pub fn pre_defined_fields(&self) -> FieldLoc {
        self.num_pre_defined_fields
    }

    pub fn set_pre_defined_fields(&mut self, val: FieldLoc) {
        self.num_pre_defined_fields = val;
    }

    pub fn check_field_overflow(&mut self, fields: usize) {
        while fields > self.field_info.len() {
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
