use crate::c_api::{ttstub_input_close, ttstub_input_getc, ttstub_input_open, xcalloc};
use tectonic_io_base::InputHandle;
use libc::{EOF, free};
use std::{mem, ptr};

/* Sigh, I'm worried about ungetc() and EOF semantics in Bibtex's I/O, so
 * here's a tiny wrapper that lets us fake it. */

#[repr(C)]
pub struct PeekableInput {
    // rust_input_handle_t
    handle: *mut InputHandle,
    peek_char: libc::c_int,
    saw_eof: bool,
}

#[no_mangle]
pub unsafe extern "C" fn peekable_open(
    path: *const libc::c_char,
    format: tectonic_bridge_core::FileFormat,
) -> *mut PeekableInput {
    let handle = ttstub_input_open(path, format, 0);
    if handle.is_null() {
        return ptr::null_mut();
    }

    let peekable = xcalloc(1, mem::size_of::<PeekableInput>()).cast::<PeekableInput>();

    (*peekable).handle = handle;
    (*peekable).peek_char = EOF;
    (*peekable).saw_eof = false;

    peekable
}

#[no_mangle]
pub unsafe extern "C" fn peekable_close(
    peekable: *mut PeekableInput,
) -> libc::c_int {
    if peekable.is_null() {
        return 0;
    }

    let rv = ttstub_input_close((*peekable).handle);
    free(peekable.cast());
    return rv;
}

#[no_mangle]
pub unsafe extern "C" fn peekable_getc(
    peekable: *mut PeekableInput,
) -> libc::c_int {
    let peekable = &mut *peekable;
    if peekable.peek_char != EOF {
        let rv = peekable.peek_char;
        peekable.peek_char = EOF;
        return rv;
    }

    let rv = ttstub_input_getc(peekable.handle);
    if rv == EOF {
        peekable.saw_eof = true;
    }
    return rv;
}

#[no_mangle]
pub unsafe extern "C" fn peekable_ungetc(
    peekable: *mut PeekableInput,
    c: libc::c_int,
) {
    assert_ne!(c, EOF);
    (*peekable).peek_char = c;
}

#[no_mangle]
pub unsafe extern "C" fn tectonic_eof(peekable: *mut PeekableInput) -> bool {
    // Check for EOF following Pascal semantics.
    let peekable = match peekable.as_mut() {
        Some(p) => p,
        None => return true,
    };
    if peekable.saw_eof {
        return true;
    }
    let c = peekable_getc(peekable);
    if c == EOF {
        return true;
    }
    peekable_ungetc(peekable, c);

    false
}

#[no_mangle]
pub unsafe extern "C" fn eoln(peekable: *mut PeekableInput) -> bool {
    if (*peekable).saw_eof {
        return true;
    }
    let c = peekable_getc(peekable);
    if c != EOF {
        peekable_ungetc(peekable, c)
    }
    c == b'\n' as libc::c_int || c == '\r' as libc::c_int || c == EOF
}
