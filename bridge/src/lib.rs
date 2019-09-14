#![feature(c_variadic)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    #[no_mangle]
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::std::ffi::VaList) -> i32;
}

pub type size_t = u64;
pub type ssize_t = i64;

pub type rust_output_handle_t = *mut libc::c_void;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_bridge_api_t {
    pub context: *mut libc::c_void,
    pub issue_warning: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8) -> ()>,
    pub issue_error: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8) -> ()>,
    pub get_file_md5:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8, _: *mut i8) -> i32>,
    pub get_data_md5: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8, _: size_t, _: *mut i8) -> i32,
    >,
    pub output_open: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: *const i8, _: i32) -> rust_output_handle_t,
    >,
    pub output_open_stdout:
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> rust_output_handle_t>,
    pub output_putc:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_output_handle_t, _: i32) -> i32>,
    pub output_write: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: rust_output_handle_t,
            _: *const i8,
            _: size_t,
        ) -> size_t,
    >,
    pub output_flush:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_output_handle_t) -> i32>,
    pub output_close:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_output_handle_t) -> i32>,
    pub input_open: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: *const i8,
            _: TTInputFormat,
            _: i32,
        ) -> rust_input_handle_t,
    >,
    pub input_open_primary:
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> rust_input_handle_t>,
    pub input_get_size:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_input_handle_t) -> size_t>,
    pub input_seek: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: rust_input_handle_t,
            _: ssize_t,
            _: i32,
            _: *mut i32,
        ) -> size_t,
    >,
    pub input_read: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: rust_input_handle_t,
            _: *mut i8,
            _: size_t,
        ) -> ssize_t,
    >,
    pub input_getc:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_input_handle_t) -> i32>,
    pub input_ungetc:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_input_handle_t, _: i32) -> i32>,
    pub input_close:
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: rust_input_handle_t) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum TTHistory {
    SPOTLESS = 0,
    WARNING_ISSUED = 1,
    ERROR_ISSUED = 2,
    FATAL_ERROR = 3,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum TTInputFormat {
    TFM = 3,
    AFM = 4,
    BIB = 6,
    BST = 7,
    CNF = 8,
    FORMAT = 10,
    FONTMAP = 11,
    OFM = 20,
    OVF = 23,
    PICT = 25,
    TEX = 26,
    TEX_PS_HEADER = 30,
    TYPE1 = 32,
    VF = 33,
    TRUETYPE = 36,
    BINARY = 40,
    MISCFONTS = 41,
    ENC = 44,
    CMAP = 45,
    SFD = 46,
    OPENTYPE = 47,
    TECTONIC_PRIMARY = 59, /* quasi-hack to get the primary input */
}

static mut tectonic_global_bridge: *const tt_bridge_api_t = std::ptr::null();

pub unsafe fn tt_with_bridge<F, T>(bridge: *const tt_bridge_api_t, f: F) -> Option<T>
where
    F: std::panic::UnwindSafe + std::ops::FnOnce() -> T,
{
    use std::panic;
    use std::ptr::null;
    tectonic_global_bridge = bridge;
    let r = panic::catch_unwind(|| (f)()).ok();
    tectonic_global_bridge = null();
    r
}

pub unsafe fn tt_get_current_bridge() -> Option<&'static tt_bridge_api_t> {
    tectonic_global_bridge.as_ref()
}

/* Global symbols that route through the global API variable. Hopefully we
 * will one day eliminate all of the global state and get rid of all of
 * these. */
/* Global symbols that route through the global API */
#[no_mangle]
pub unsafe extern "C" fn ttstub_issue_warning(mut format: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl; /* Not ideal to (ab)use error_buf here */
    ap = args.clone(); /* Not ideal to (ab)use error_buf here */
    vsnprintf(
        error_buf.as_mut_ptr(),
        1024i32 as u64,
        format,
        ap.as_va_list(),
    );
    (*tectonic_global_bridge)
        .issue_warning
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context,
        error_buf.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_issue_error(mut format: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(
        error_buf.as_mut_ptr(),
        1024i32 as u64,
        format,
        ap.as_va_list(),
    );
    (*tectonic_global_bridge)
        .issue_error
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context,
        error_buf.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_fprintf(
    mut handle: rust_output_handle_t,
    mut format: *const i8,
    mut args: ...
) -> i32 {
    static mut fprintf_buf: [i8; 1024] = [0; 1024];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    let mut len: i32 = vsnprintf(
        fprintf_buf.as_mut_ptr(),
        1024i32 as u64,
        format,
        ap.as_va_list(),
    );
    if len >= 1024i32 {
        len = 1024i32 - 1i32;
        fprintf_buf[len as usize] = '\u{0}' as i32 as i8
    }
    if len >= 0i32 {
        ttstub_output_write(handle, fprintf_buf.as_mut_ptr(), len as size_t);
    }
    len
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_get_file_md5(mut path: *const i8, mut digest: *mut i8) -> i32 {
    (*tectonic_global_bridge)
        .get_file_md5
        .expect("non-null function pointer")((*tectonic_global_bridge).context, path, digest)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_get_data_md5(
    mut data: *const i8,
    mut len: size_t,
    mut digest: *mut i8,
) -> i32 {
    (*tectonic_global_bridge)
        .get_data_md5
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, data, len, digest
    )
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_open(
    mut path: *const i8,
    mut is_gz: i32,
) -> rust_output_handle_t {
    (*tectonic_global_bridge)
        .output_open
        .expect("non-null function pointer")((*tectonic_global_bridge).context, path, is_gz)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_open_stdout() -> rust_output_handle_t {
    (*tectonic_global_bridge)
        .output_open_stdout
        .expect("non-null function pointer")((*tectonic_global_bridge).context)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_putc(mut handle: rust_output_handle_t, mut c: i32) -> i32 {
    (*tectonic_global_bridge)
        .output_putc
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle, c)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_write(
    mut handle: rust_output_handle_t,
    mut data: *const i8,
    mut len: size_t,
) -> size_t {
    (*tectonic_global_bridge)
        .output_write
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, handle, data, len
    )
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_flush(mut handle: rust_output_handle_t) -> i32 {
    (*tectonic_global_bridge)
        .output_flush
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_output_close(mut handle: rust_output_handle_t) -> i32 {
    (*tectonic_global_bridge)
        .output_close
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_open(
    mut path: *const i8,
    mut format: TTInputFormat,
    mut is_gz: i32,
) -> rust_input_handle_t {
    (*tectonic_global_bridge)
        .input_open
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, path, format, is_gz
    )
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_open_primary() -> rust_input_handle_t {
    (*tectonic_global_bridge)
        .input_open_primary
        .expect("non-null function pointer")((*tectonic_global_bridge).context)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_get_size(mut handle: rust_input_handle_t) -> size_t {
    (*tectonic_global_bridge)
        .input_get_size
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_seek(
    mut handle: rust_input_handle_t,
    mut offset: ssize_t,
    mut whence: i32,
) -> size_t {
    let mut internal_error: i32 = 0i32;
    let mut rv: size_t = (*tectonic_global_bridge)
        .input_seek
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context,
        handle,
        offset,
        whence,
        &mut internal_error,
    );
    if internal_error != 0 {
        // Nonzero indicates a serious internal error.
        panic!("ttstub_input_seek");
    }
    rv
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_read(
    mut handle: rust_input_handle_t,
    mut data: *mut i8,
    mut len: size_t,
) -> ssize_t {
    (*tectonic_global_bridge)
        .input_read
        .expect("non-null function pointer")(
        (*tectonic_global_bridge).context, handle, data, len
    )
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_getc(mut handle: rust_input_handle_t) -> i32 {
    (*tectonic_global_bridge)
        .input_getc
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_ungetc(mut handle: rust_input_handle_t, mut ch: i32) -> i32 {
    (*tectonic_global_bridge)
        .input_ungetc
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle, ch)
}
#[no_mangle]
pub unsafe extern "C" fn ttstub_input_close(mut handle: rust_input_handle_t) -> i32 {
    if (*tectonic_global_bridge)
        .input_close
        .expect("non-null function pointer")((*tectonic_global_bridge).context, handle)
        != 0
    {
        // Nonzero return value indicates a serious internal error.
        panic!("ttstub_input_close");
    }
    0i32
}

/* TODO: these are needed for the various *_main routines which should
 * probably be moved out into other files. */
/* The global variable that represents the Rust API. Some fine day we'll get
 * rid of all of the globals ... */
static mut error_buf: [i8; 1024] = [0; 1024];

#[no_mangle]
pub unsafe extern "C" fn _tt_abort(mut format: *const i8, mut args: ...) -> ! {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(
        error_buf.as_mut_ptr(),
        1024i32 as u64,
        format,
        ap.as_va_list(),
    );
    panic!("tt_abort")
}
#[no_mangle]
pub unsafe extern "C" fn tt_get_error_message() -> *const i8 {
    error_buf.as_mut_ptr()
}

#[macro_use]
pub mod macro_stub;
pub mod stub_errno;
