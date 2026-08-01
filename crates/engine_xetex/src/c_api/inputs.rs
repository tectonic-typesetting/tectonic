use crate::c_api::engine::rs_gettexstring;
use crate::c_api::globals::Globals;
use crate::ty::StrNumber;
use std::cell::RefCell;
use std::io::Write;
use tectonic_bridge_core::{CoreBridgeState, FileFormat, InputId};

// pub const MAX_IN_OPEN: usize = 15;
pub const ICUMAPPING: u8 = 5;

thread_local! {
    static FILE_CTX: RefCell<FileCtx> = const { RefCell::new(FileCtx::new()) };
}

pub struct FileCtx {
    pub(crate) in_open: i32,
    pub(crate) source_filename_stack: Vec<StrNumber>,
    pub(crate) full_source_filename_stack: Vec<StrNumber>,
    pub(crate) line: i32,
    pub(crate) line_stack: Vec<i32>,
}

impl FileCtx {
    const fn new() -> FileCtx {
        FileCtx {
            in_open: 0,
            source_filename_stack: Vec::new(),
            full_source_filename_stack: Vec::new(),
            line: 0,
            line_stack: Vec::new(),
        }
    }

    pub fn with<T>(f: impl FnOnce(&mut FileCtx) -> T) -> T {
        FILE_CTX.with_borrow_mut(f)
    }
}

#[no_mangle]
pub extern "C" fn in_open() -> i32 {
    FILE_CTX.with_borrow(|files| files.in_open)
}

#[no_mangle]
pub extern "C" fn set_in_open(val: i32) {
    FILE_CTX.with_borrow_mut(|files| files.in_open = val)
}

c_arr!(FileCtx => source_filename_stack: StrNumber);

#[no_mangle]
pub extern "C" fn full_source_filename_stack(idx: usize) -> StrNumber {
    FILE_CTX.with_borrow(|files| files.full_source_filename_stack[idx])
}

#[no_mangle]
pub extern "C" fn set_full_source_filename_stack(idx: usize, val: StrNumber) {
    FILE_CTX.with_borrow_mut(|files| {
        if files.full_source_filename_stack.len() < idx + 1 {
            files.full_source_filename_stack.resize(idx + 1, 0);
        }
        files.full_source_filename_stack[idx] = val
    })
}

#[no_mangle]
pub extern "C" fn clear_full_source_filename_stack() {
    FILE_CTX.with_borrow_mut(|files| files.full_source_filename_stack.clear())
}

#[no_mangle]
pub extern "C" fn line() -> i32 {
    FILE_CTX.with_borrow(|files| files.line)
}

#[no_mangle]
pub extern "C" fn set_line(val: i32) {
    FILE_CTX.with_borrow_mut(|files| files.line = val)
}

#[no_mangle]
pub extern "C" fn line_stack(idx: usize) -> i32 {
    FILE_CTX.with_borrow(|files| files.line_stack[idx])
}

#[no_mangle]
pub extern "C" fn set_line_stack(idx: usize, val: i32) {
    FILE_CTX.with_borrow_mut(|files| {
        if files.line_stack.len() < idx + 1 {
            files.line_stack.resize(idx + 1, 0);
        }
        files.line_stack[idx] = val;
    })
}

#[no_mangle]
pub extern "C" fn clear_line_stack() {
    FILE_CTX.with_borrow_mut(|files| files.line_stack.clear())
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct UFile {
    handle: Option<InputId>,
    saved_char: i64,
    skip_next_lf: bool,
    encoding_mode: u8,
    conversion_data: *mut libc::c_void,
    conversion_drop: Option<extern "C" fn(*mut libc::c_void)>,
}

impl UFile {
    pub fn close(&mut self, state: &mut CoreBridgeState<'_>) {
        /* NULL handle is stdin/terminal file. Shouldn't happen but meh. */
        let Some(handle) = self.handle else {
            return;
        };

        state.input_close(handle);

        if self.encoding_mode == ICUMAPPING && !self.conversion_data.is_null() {
            if let Some(f) = self.conversion_drop {
                f(self.conversion_data);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn u_close(file: *mut UFile) {
    if file.is_null() {
        return;
    }
    Globals::with(|globals| (unsafe { &mut *file }).close(globals.state));
    unsafe { libc::free(file.cast()) };
}

/// Given a file name stored in the string pool, insert into the string pool text
/// giving its size in bytes.
pub fn rs_getfilesize(globals: &mut Globals<'_, '_>, s: StrNumber) {
    let name = rs_gettexstring(globals, s);
    let Some(handle) = globals.state.input_open(&name, FileFormat::Tex, false) else {
        return;
    };
    let len = globals.state.input_get_size(handle);
    globals.state.input_close(handle);

    let mut buf = [0u8; 20];
    let buf_len = buf.len();
    let mut write = &mut buf[..];

    write!(write, "{}", len).unwrap();
    let written = buf_len - write.len();
    if globals.strings.pool_ptr + written >= globals.strings.pool_size {
        globals.strings.pool_ptr = globals.strings.pool_size;
        /* error by str_toks that calls str_room(1) */
    } else {
        for b in &buf[..written] {
            globals.strings.str_pool[globals.strings.pool_ptr] = *b as u16;
            globals.strings.pool_ptr += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn getfilesize(s: StrNumber) {
    Globals::with(|globals| rs_getfilesize(globals, s))
}
