use crate::{FieldLoc, HashPointer, StrNumber};

const MAX_FIELDS: usize = 17250;

pub(crate) struct OtherData {
    wiz_functions: Vec<HashPointer>,
    field_info: Vec<StrNumber>,
    num_fields: FieldLoc,
    num_pre_defined_fields: FieldLoc,
    crossref_num: FieldLoc,
}

impl OtherData {
    pub fn new() -> OtherData {
        OtherData {
            wiz_functions: Vec::new(),
            field_info: vec![0; MAX_FIELDS + 1],
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
            self.field_info
                .resize(self.field_info.len() + MAX_FIELDS, 0);
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

    pub fn push_wiz_func(&mut self, val: HashPointer) {
        self.wiz_functions.push(val)
    }

    pub fn wiz_func_len(&self) -> usize {
        self.wiz_functions.len()
    }
}
