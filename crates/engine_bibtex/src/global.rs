use crate::{xbuf::XBuf, ASCIICode, StrNumber};

const MAX_GLOB_STRS: usize = 10;
pub(crate) const GLOB_STR_SIZE: usize = 20000;

pub(crate) struct GlobalData {
    glb_bib_str_ptr: XBuf<StrNumber>,
    global_strs: XBuf<ASCIICode>,
    glb_str_end: XBuf<usize>,
    num_glb_strs: i32,
}

impl GlobalData {
    pub fn new() -> GlobalData {
        GlobalData {
            glb_bib_str_ptr: XBuf::new(MAX_GLOB_STRS),
            global_strs: XBuf::new((GLOB_STR_SIZE + 1) * MAX_GLOB_STRS),
            glb_str_end: XBuf::new(MAX_GLOB_STRS),
            num_glb_strs: 0,
        }
    }

    pub fn grow(&mut self) {
        self.glb_bib_str_ptr.grow(MAX_GLOB_STRS);
        self.global_strs.grow((GLOB_STR_SIZE + 1) * MAX_GLOB_STRS);
        self.glb_str_end.grow(MAX_GLOB_STRS);
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

    pub fn num_glb_strs(&self) -> i32 {
        self.num_glb_strs
    }

    pub fn set_num_glb_strs(&mut self, val: i32) {
        self.num_glb_strs = val;
    }

    pub fn len(&self) -> usize {
        self.glb_bib_str_ptr.len()
    }
}
