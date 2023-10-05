pub const FC_FILE: *const libc::c_char = b"file" as *const [u8] as *const libc::c_char;
pub const FC_INDEX: *const libc::c_char = b"index" as *const [u8] as *const libc::c_char;

#[repr(C)]
pub struct FcPattern(());

#[repr(C)]
pub enum FcResult {
    Match,
    NoMatch,
    TypeMismatch,
    ResultNoId,
    OutOfMemory,
}

extern "C" {
    pub fn FcPatternGetString(
        p: *mut FcPattern,
        object: *const libc::c_char,
        n: libc::c_int,
        s: *mut *const libc::c_char,
    ) -> FcResult;
    pub fn FcPatternGetInteger(
        p: *mut FcPattern,
        object: *const libc::c_char,
        n: libc::c_int,
        i: *mut libc::c_int,
    ) -> FcResult;
}
