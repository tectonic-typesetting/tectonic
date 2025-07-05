use crate::{
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    ASCIICode, Bibtex, BibtexError, BufPointer,
};
use libc::EOF;
use std::{ffi::CStr, io};
use tectonic_bridge_core::{CoreBridgeState, FileFormat, InputId};

/* Sigh, I'm worried about ungetc() and EOF semantics in Bibtex's I/O, so
 * here's a tiny wrapper that lets us fake it. */

pub(crate) struct PeekableInput {
    id: InputId,
    peek_char: libc::c_int,
    saw_eof: bool,
}

impl PeekableInput {
    pub(crate) fn open(
        ctx: &mut Bibtex<'_, '_>,
        path: &CStr,
        format: FileFormat,
    ) -> Result<PeekableInput, BibtexError> {
        let id = ctx.engine.input_open(path.to_str().unwrap(), format, false);

        if let Some(id) = id {
            Ok(PeekableInput {
                id,
                peek_char: EOF,
                saw_eof: false,
            })
        } else {
            Err(BibtexError::Fatal)
        }
    }

    pub(crate) fn close(self, ctx: &mut Bibtex<'_, '_>) -> Result<(), BibtexError> {
        if !ctx.engine.input_close(self.id) {
            Ok(())
        } else {
            Err(BibtexError::Fatal)
        }
    }

    fn getc(&mut self, engine: &mut CoreBridgeState<'_>) -> libc::c_int {
        if self.peek_char != EOF {
            let rv = self.peek_char;
            self.peek_char = EOF;
            return rv;
        }

        // SAFETY: Internal handle guaranteed valid, unique access to this input is unique access
        //         to the handle
        let handle = engine.get_input(self.id);
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

    pub fn eof(&mut self, engine: &mut CoreBridgeState<'_>) -> bool {
        if self.saw_eof {
            return true;
        }
        let c = self.getc(engine);
        if c == EOF {
            return true;
        }
        self.ungetc(c);
        false
    }

    fn eoln(&mut self, engine: &mut CoreBridgeState<'_>) -> bool {
        if self.saw_eof {
            return true;
        }
        let c = self.getc(engine);
        if c != EOF {
            self.ungetc(c);
        }
        c == b'\n' as libc::c_int || c == '\r' as libc::c_int || c == EOF
    }
}

pub(crate) fn input_ln(
    engine: &mut CoreBridgeState<'_>,
    peekable: &mut PeekableInput,
    buffers: &mut GlobalBuffer,
) -> bool {
    buffers.set_init(BufTy::Base, 0);
    let mut last = 0;
    if peekable.eof(engine) {
        return false;
    }

    // Read up to end-of-line
    while !peekable.eoln(engine) {
        if last >= buffers.len() as BufPointer {
            buffers.grow_all();
        }

        buffers.set_at(BufTy::Base, last, peekable.getc(engine) as ASCIICode);
        last += 1;
    }

    // For side effects - consume the eoln we saw
    let eoln = peekable.getc(engine);
    if eoln == '\r' as libc::c_int {
        let next = peekable.getc(engine);
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
