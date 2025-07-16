use crate::{ASCIICode, StrNumber};

const MAX_GLOB_STRS: usize = 10;
pub(crate) const GLOB_STR_SIZE: usize = 20000;

pub(crate) struct GlobalData {
    glb_bib_str_ptr: Vec<StrNumber>,
    global_strs: Vec<ASCIICode>,
    glb_str_end: Vec<usize>,
    num_glb_strs: usize,
}

impl GlobalData {
    pub fn new() -> GlobalData {
        GlobalData {
            glb_bib_str_ptr: vec![0; MAX_GLOB_STRS + 1],
            global_strs: vec![0; (GLOB_STR_SIZE + 1) * MAX_GLOB_STRS + 1],
            glb_str_end: vec![0; MAX_GLOB_STRS + 1],
            num_glb_strs: 0,
        }
    }

    pub fn grow(&mut self) {
        self.glb_bib_str_ptr
            .resize(self.glb_bib_str_ptr.len() + MAX_GLOB_STRS, 0);
        self.global_strs.resize(
            self.global_strs.len() + (GLOB_STR_SIZE + 1) * MAX_GLOB_STRS,
            0,
        );
        self.glb_str_end
            .resize(self.glb_str_end.len() + MAX_GLOB_STRS, 0);
    }

    pub fn str(&self, pos: usize) -> &[ASCIICode] {
        let spos = pos * (GLOB_STR_SIZE + 1);
        &self.global_strs[spos..spos + self.glb_str_end[pos]]
    }

    pub fn str_ptr(&self, pos: usize) -> StrNumber {
        self.glb_bib_str_ptr[pos]
    }

    pub fn set_str_ptr(&mut self, pos: usize, val: StrNumber) {
        self.glb_bib_str_ptr[pos] = val;
    }

    pub fn set_str(&mut self, pos: usize, val: &[ASCIICode]) {
        let spos = pos * (GLOB_STR_SIZE + 1);
        self.global_strs[spos..spos + val.len()].copy_from_slice(val);
        self.glb_str_end[pos] = val.len();
    }

    pub fn num_glb_strs(&self) -> usize {
        self.num_glb_strs
    }

    pub fn set_num_glb_strs(&mut self, val: usize) {
        self.num_glb_strs = val;
    }

    pub fn len(&self) -> usize {
        self.glb_bib_str_ptr.len()
    }
}
