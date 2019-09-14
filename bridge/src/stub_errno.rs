extern_and_forward_stub! {
    pub fn errno => tt_errno() -> libc::c_int;
    pub fn set_errno => tt_set_errno(v: libc::c_int) -> ();
}

pub const ZERO: libc::c_int = 0;

// FIXME: Are these same on all platforms?
pub const EINTR: libc::c_int = 4;
pub const ERANGE: libc::c_int = 34;
