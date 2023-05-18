use crate::c_api::{ASCIICode, StrNumber};
use crate::c_api::xbuf::XBuf;

pub struct GlobalData {
    glb_bib_str_ptr: XBuf<StrNumber>,
    global_strs: XBuf<ASCIICode>,
    glb_str_end: XBuf<i32>,
}
