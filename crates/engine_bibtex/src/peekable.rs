use crate::{
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    ttbc_input_close, ttbc_input_open, ASCIICode, Bibtex, BibtexError, BufPointer,
};
use libc::EOF;
use std::{ffi::CStr, io, ptr::NonNull};
use tectonic_bridge_core::FileFormat;
use tectonic_io_base::InputHandle;

/* Sigh, I'm worried about ungetc() and EOF semantics in Bibtex's I/O, so
 * here's a tiny wrapper that lets us fake it. */

pub(crate) struct PeekableInput {
    handle: NonNull<InputHandle>,
    peek_char: libc::c_int,
    saw_eof: bool,
}

impl PeekableInput {
    pub(crate) fn open(
        ctx: &mut Bibtex<'_, '_>,
        path: &CStr,
        format: FileFormat,
    ) -> Result<PeekableInput, BibtexError> {
        // SAFETY: Our CStr is valid for the length of the call, so this can't access bad memory
        let handle = unsafe { ttbc_input_open(ctx.engine, path.as_ptr(), format, 0) };

        if let Some(handle) = NonNull::new(handle) {
            Ok(PeekableInput {
                handle,
                peek_char: EOF,
                saw_eof: false,
            })
        } else {
            Err(BibtexError::Fatal)
        }
    }

    pub(crate) fn close(self, ctx: &mut Bibtex<'_, '_>) -> Result<(), BibtexError> {
        let err = ttbc_input_close(ctx.engine, self.handle.as_ptr());
        if err == 0 {
            Ok(())
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

    pub fn eof(&mut self) -> bool {
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

pub(crate) fn input_ln(peekable: Option<&mut PeekableInput>, buffers: &mut GlobalBuffer) -> bool {
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
