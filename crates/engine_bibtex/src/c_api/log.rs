use crate::c_api::{buf_to_slice, BufPointer, History, ttstub_output_open, ttstub_output_open_stdout};
use std::cell::Cell;
use std::ffi::CStr;
use std::io::Write;
use std::{ptr, slice};
use tectonic_io_base::OutputHandle;
use crate::c_api::buffer::{BufTy, with_buffers};
use crate::c_api::char_info::{LEX_CLASS, LexClass};
use crate::c_api::history::{mark_error, mark_fatal, set_history};

trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for str {
    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsBytes for String {
    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsBytes for [u8] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

thread_local! {
    static STANDARD_OUTPUT: Cell<*mut OutputHandle> = Cell::new(ptr::null_mut());
    static LOG_FILE: Cell<*mut OutputHandle> = Cell::new(ptr::null_mut());
}

fn with_stdout<T>(f: impl FnOnce(&mut OutputHandle) -> T) -> T {
    STANDARD_OUTPUT.with(|out| f(unsafe { out.get().as_mut() }.unwrap()))
}

fn with_log<T>(f: impl FnOnce(&mut OutputHandle) -> T) -> T {
    LOG_FILE.with(|out| f(unsafe { out.get().as_mut() }.unwrap()))
}

fn write_logs<B: ?Sized + AsBytes>(str: &B) {
    with_log(|log| log.write_all(str.as_bytes()))
        .unwrap();
    with_stdout(|out| out.write_all(str.as_bytes()))
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn init_log_file(file: *const libc::c_char) -> *mut OutputHandle {
    LOG_FILE.with(|log| {
        let ptr = log.get();
        if ptr.is_null() {
            let new = ttstub_output_open(file, 0);
            log.set(new);
            new
        } else {
            ptr
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn standard_output() -> *mut OutputHandle {
    STANDARD_OUTPUT.with(|output| {
        let ptr = output.get();

        if ptr.is_null() {
            let stdout = ttstub_output_open_stdout();
            if stdout.is_null() {
                set_history(History::FatalError);
            }
            output.set(stdout);
            stdout
        } else {
            ptr
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn log_file() -> *mut OutputHandle {
    LOG_FILE.with(|file| file.get())
}

#[no_mangle]
pub extern "C" fn putc_log(c: libc::c_int) {
    let c = c as u8;
    with_log(|log| log.write_all(&[c]))
        .unwrap();
    with_stdout(|out| out.write_all(&[c]))
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn puts_log(str: *const libc::c_char) {
    let str = CStr::from_ptr(str);
    with_log(|log| log.write_all(str.to_bytes()))
        .unwrap();
    with_stdout(|out| out.write_all(str.to_bytes()))
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn ttstub_puts (handle: *mut OutputHandle, s: *const libc::c_char) {
    let str = CStr::from_ptr(s);
    (*handle).write(str.to_bytes())
        .unwrap();
}

#[no_mangle]
pub extern "C" fn print_overflow() {
    write_logs("Sorry---you've exceeded BibTeX's ");
    mark_fatal();
}

#[no_mangle]
pub extern "C" fn print_confusion() {
    write_logs("---this can't happen\n*Please notify the Tectonic maintainer*\n");
    mark_fatal();
}

#[no_mangle]
pub unsafe extern "C" fn out_token(handle: *mut OutputHandle) {
    with_buffers(|b| {
        let ptr = b.buffer(BufTy::Base);
        let start = b.offset(BufTy::Base, 1);
        let len = b.offset(BufTy::Base, 2) - b.offset(BufTy::Base, 1);
        let bytes = buf_to_slice(ptr, start, len);
        (*handle).write_all(bytes)
            .unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn print_a_token() {
    out_token(standard_output());
    out_token(log_file());
}

#[no_mangle]
pub unsafe extern "C" fn print_bad_input_line(last: BufPointer) {
    write_logs(" : ");

    with_buffers(|b| {
        let offset2 = b.offset(BufTy::Base, 2);

        let slice = buf_to_slice(
            b.buffer(BufTy::Base),
            0,
            offset2
        );

        for code in slice {
            if LEX_CLASS[*code as usize] == LexClass::Whitespace {
                write_logs(" ");
            } else {
                write_logs(slice::from_ref(code))
            }
        }
        write_logs("\n : ");
        let str = (0..offset2).map(|_| ' ').collect::<String>();
        write_logs(&str);

        for code in &slice[last as usize..] {
            if LEX_CLASS[*code as usize] == LexClass::Whitespace {
                write_logs(" ");
            } else {
                write_logs(slice::from_ref(code));
            }
        }

        write_logs("\n");

        if slice.iter().find(|c| LEX_CLASS[**c as usize] != LexClass::Whitespace).is_none() {
            write_logs("(Error may have been on previous line)\n");
        }
    });

    mark_error();
}

#[no_mangle]
pub unsafe extern "C" fn print_skipping_whatever_remains() {
    write_logs("I'm skipping whatever remains of this ");
}
