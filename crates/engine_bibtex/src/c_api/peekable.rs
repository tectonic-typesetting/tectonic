use crate::{
    c_api::{
        buffer::{with_buffers_mut, BufTy, GlobalBuffer},
        char_info::LexClass,
        ttstub_input_close, ttstub_input_open, ASCIICode, BufPointer,
    },
    BibtexError,
};
use libc::EOF;
use std::{ffi::CStr, io, ptr, ptr::NonNull};
use tectonic_bridge_core::FileFormat;
use tectonic_io_base::InputHandle;

/* Sigh, I'm worried about ungetc() and EOF semantics in Bibtex's I/O, so
 * here's a tiny wrapper that lets us fake it. */

pub struct PeekableInput {
    handle: NonNull<InputHandle>,
    peek_char: libc::c_int,
    saw_eof: bool,
}

impl PeekableInput {
    pub(crate) fn open(path: &CStr, format: FileFormat) -> Result<Box<PeekableInput>, BibtexError> {
        // SAFETY: Our CStr is valid for the length of the call, so this can't access bad memory
        let handle = unsafe { ttstub_input_open(path.as_ptr(), format, 0) };

        if let Some(handle) = NonNull::new(handle) {
            Ok(Box::new(PeekableInput {
                handle,
                peek_char: EOF,
                saw_eof: false,
            }))
        } else {
            Err(BibtexError::Fatal)
        }
    }

    fn getc(&mut self) -> libc::c_int {
        if self.peek_char != EOF {
            let rv = self.peek_char;
            self.peek_char = EOF;
            return rv;
        }

        // SAFETY: Internal handle guaranteed valid, unique access to this input is unique access
        //         to the handle
        let handle = unsafe { self.handle.as_mut() };
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
    format: FileFormat,
) -> *mut PeekableInput {
    PeekableInput::open(CStr::from_ptr(path), format)
        .map(Box::into_raw)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn peekable_close(peekable: Option<NonNull<PeekableInput>>) -> libc::c_int {
    match peekable {
        Some(mut peekable) => {
            let rv = ttstub_input_close(peekable.as_mut().handle.as_ptr());
            drop(Box::<PeekableInput>::from_raw(peekable.as_ptr()));
            rv
        }
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn tectonic_eof(peekable: Option<NonNull<PeekableInput>>) -> bool {
    // Check for EOF following Pascal semantics.
    let peekable = match peekable {
        Some(mut p) => p.as_mut(),
        None => return true,
    };
    peekable.eof()
}

pub fn rs_input_ln(peekable: Option<&mut PeekableInput>, buffers: &mut GlobalBuffer) -> bool {
    let peekable = match peekable {
        Some(p) => p,
        None => return false,
    };

    buffers.set_init(BufTy::Base, 0);
    let mut last = 0;
    if peekable.eof() {
        return false;
    }

    // Read up to end-of-line
    while !peekable.eoln() {
        if last >= buffers.len() as BufPointer {
            buffers.grow_all();
        }

        buffers.set_at(BufTy::Base, last, peekable.getc() as ASCIICode);
        last += 1;
    }

    // For side effects - consume the eoln we saw
    let eoln = peekable.getc();
    if eoln == '\r' as libc::c_int {
        let next = peekable.getc();
        if next != '\n' as libc::c_int {
            peekable.ungetc(next);
        }
    }

    // Trim whitespace
    while last > 0 {
        if LexClass::of(buffers.at(BufTy::Base, last - 1)) == LexClass::Whitespace {
            last -= 1;
        } else {
            break;
        }
    }

    buffers.set_init(BufTy::Base, last);

    true
}

#[no_mangle]
pub unsafe extern "C" fn input_ln(peekable: Option<NonNull<PeekableInput>>) -> bool {
    with_buffers_mut(|buffers| rs_input_ln(peekable.map(|mut ptr| ptr.as_mut()), buffers))
}
