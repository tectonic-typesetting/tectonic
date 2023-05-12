use crate::c_api::buffer::{with_buffers, with_buffers_mut, BufTy};
use crate::c_api::char_info::{LexClass, LEX_CLASS};
use crate::c_api::{ttstub_input_close, ttstub_input_open, xcalloc, ASCIICode, BufPointer};
use libc::{free, EOF};
use std::{io, mem, ptr};
use tectonic_io_base::InputHandle;

/* Sigh, I'm worried about ungetc() and EOF semantics in Bibtex's I/O, so
 * here's a tiny wrapper that lets us fake it. */

#[repr(C)]
pub struct PeekableInput {
    handle: *mut InputHandle,
    peek_char: libc::c_int,
    saw_eof: bool,
}

impl PeekableInput {
    fn getc(&mut self) -> libc::c_int {
        if self.peek_char != EOF {
            let rv = self.peek_char;
            self.peek_char = EOF;
            return rv;
        }

        // SAFETY: Internal handle guaranteed valid, unique access to this input is unique access
        //         to the handle
        let handle = unsafe { &mut *self.handle };
        let rv = match handle.getc() {
            Ok(c) => libc::c_int::from(c),
            Err(e) => {
                if let Some(e) = e.downcast_ref::<io::Error>() {
                    if e.kind() == io::ErrorKind::UnexpectedEof {
                        return EOF;
                    }
                }
                -1
            }
        };
        if rv == EOF {
            self.saw_eof = true;
        }
        rv
    }

    fn ungetc(&mut self, c: libc::c_int) {
        assert_ne!(c, EOF);
        self.peek_char = c;
    }

    fn eof(&mut self) -> bool {
        if self.saw_eof {
            return true;
        }
        let c = self.getc();
        if c == EOF {
            return true;
        }
        self.ungetc(c);
        false
    }

    fn eoln(&mut self) -> bool {
        if self.saw_eof {
            return true;
        }
        let c = self.getc();
        if c != EOF {
            self.ungetc(c);
        }
        c == b'\n' as libc::c_int || c == '\r' as libc::c_int || c == EOF
    }
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
pub unsafe extern "C" fn peekable_close(peekable: *mut PeekableInput) -> libc::c_int {
    if peekable.is_null() {
        return 0;
    }

    let rv = ttstub_input_close((*peekable).handle);
    free(peekable.cast());
    rv
}

#[no_mangle]
pub unsafe extern "C" fn tectonic_eof(peekable: *mut PeekableInput) -> bool {
    // Check for EOF following Pascal semantics.
    let peekable = match peekable.as_mut() {
        Some(p) => p,
        None => return true,
    };
    peekable.eof()
}

#[no_mangle]
pub unsafe extern "C" fn input_ln(last: *mut BufPointer, peekable: *mut PeekableInput) -> bool {
    *last = 0;
    let peekable = &mut *peekable;
    if peekable.eof() {
        return false;
    }

    // Read up to end-of-line
    with_buffers_mut(|b| {
        while !peekable.eoln() {
            if *last >= b.len() as BufPointer {
                b.grow_all();
            }

            let ptr = &mut b.buffer_mut(BufTy::Base)[*last as usize];
            *ptr = peekable.getc() as ASCIICode;
            *last += 1;
        }
    });

    peekable.getc();

    // Trim whitespace
    with_buffers(|b| {
        while *last > 0 {
            if LEX_CLASS[b.at(BufTy::Base, (*last - 1) as usize) as usize] == LexClass::Whitespace {
                *last -= 1;
            } else {
                break;
            }
        }
    });

    true
}
