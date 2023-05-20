use crate::c_api::xbuf::XBuf;
use crate::c_api::{ASCIICode, StrNumber};

pub const GLOB_STR_SIZE: usize = 20000;

pub struct GlobalData {
    glb_bib_str_ptr: XBuf<StrNumber>,
    global_strs: XBuf<ASCIICode>,
    glb_str_end: XBuf<i32>,
}

pub fn reset() {
    // TODO
}
