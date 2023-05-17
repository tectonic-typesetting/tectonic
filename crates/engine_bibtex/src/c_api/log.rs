use crate::c_api::buffer::{with_buffers, BufTy};
use crate::c_api::char_info::LexClass;
use crate::c_api::history::{mark_error, mark_fatal, set_history};
use crate::c_api::pool::with_pool;
use crate::c_api::{ttstub_output_open, ttstub_output_open_stdout, History, StrNumber, NameAndLen};
use std::cell::Cell;
use std::ffi::CStr;
use std::io::Write;
use std::{ptr, slice};
use tectonic_io_base::OutputHandle;
use crate::c_api::auxi::{cur_aux, cur_aux_ln};

pub trait AsBytes {
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

pub(crate) fn reset() {
    STANDARD_OUTPUT.with(|cell| cell.set(ptr::null_mut()));
    LOG_FILE.with(|cell| cell.set(ptr::null_mut()));
}

fn init_stdout(out: &Cell<*mut OutputHandle>) {
    let ptr = out.get();
    if ptr.is_null() {
        let stdout = unsafe { ttstub_output_open_stdout() };
        if stdout.is_null() {
            set_history(History::FatalError);
        }
        out.set(stdout);
    }
}

fn with_stdout<T>(f: impl FnOnce(&mut OutputHandle) -> T) -> T {
    STANDARD_OUTPUT.with(|out| {
        let ptr = out.get();
        let ptr = if ptr.is_null() {
            init_stdout(out);
            out.get()
        } else {
            ptr
        };
        f(unsafe { ptr.as_mut() }.unwrap())
    })
}

fn with_log<T>(f: impl FnOnce(&mut OutputHandle) -> T) -> T {
    LOG_FILE.with(|out| f(unsafe { out.get().as_mut() }.unwrap()))
}

pub fn write_logs<B: ?Sized + AsBytes>(str: &B) {
    with_log(|log| log.write_all(str.as_bytes())).unwrap();
    with_stdout(|out| out.write_all(str.as_bytes())).unwrap();
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
            init_stdout(output);
            output.get()
        } else {
            ptr
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn bib_log_file() -> *mut OutputHandle {
    LOG_FILE.with(|file| file.get())
}

#[no_mangle]
pub extern "C" fn putc_log(c: libc::c_int) {
    let c = c as u8;
    with_log(|log| log.write_all(&[c])).unwrap();
    with_stdout(|out| out.write_all(&[c])).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn puts_log(str: *const libc::c_char) {
    let str = CStr::from_ptr(str);
    with_log(|log| log.write_all(str.to_bytes())).unwrap();
    with_stdout(|out| out.write_all(str.to_bytes())).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn ttstub_puts(handle: *mut OutputHandle, s: *const libc::c_char) {
    let str = CStr::from_ptr(s);
    (*handle).write_all(str.to_bytes()).unwrap();
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

pub fn out_token(handle: &mut OutputHandle) {
    with_buffers(|b| {
        let bytes = b.buffer(BufTy::Base);
        let start = b.offset(BufTy::Base, 1) as usize;
        let end = b.offset(BufTy::Base, 2) as usize;
        (*handle).write_all(&bytes[start..end]).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn print_a_token() {
    with_stdout(|stdout| out_token(stdout));
    with_log(|log| out_token(log));
}

#[no_mangle]
pub unsafe extern "C" fn print_bad_input_line() {
    write_logs(" : ");

    with_buffers(|b| {
        let offset2 = b.offset(BufTy::Base, 2) as usize;

        let slice = &b.buffer(BufTy::Base)[0..offset2];

        for code in slice {
            if LexClass::of(*code) == LexClass::Whitespace {
                write_logs(" ");
            } else {
                write_logs(slice::from_ref(code))
            }
        }
        write_logs("\n : ");
        let str = (0..offset2).map(|_| ' ').collect::<String>();
        write_logs(&str);

        let last = b.init(BufTy::Base) as usize;
        if offset2 < last {
            let slice = &b.buffer(BufTy::Base)[offset2..last];
            for code in slice {
                if LexClass::of(*code) == LexClass::Whitespace {
                    write_logs(" ");
                } else {
                    write_logs(slice::from_ref(code));
                }
            }
        }

        write_logs("\n");

        if !slice
            .iter()
            .any(|c| LexClass::of(*c) != LexClass::Whitespace)
        {
            write_logs("(Error may have been on previous line)\n");
        }
    });

    mark_error();
}

#[no_mangle]
pub unsafe extern "C" fn print_skipping_whatever_remains() {
    write_logs("I'm skipping whatever remains of this ");
}

#[no_mangle]
pub unsafe extern "C" fn out_pool_str(handle: *mut OutputHandle, s: StrNumber) -> bool {
    with_pool(|pool| {
        let str = pool.try_get_str(s as usize);
        if let Some(str) = str {
            (*handle).write_all(str).unwrap();
            true
        } else {
            write_logs(&format!("Illegal string number: {}", s));
            print_confusion();
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn print_a_pool_str(s: StrNumber) -> bool {
    with_pool(|pool| {
        let str = pool.try_get_str(s as usize);
        if let Some(str) = str {
            write_logs(str);
            true
        } else {
            write_logs(&format!("Illegal string number: {}", s));
            print_confusion();
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn sam_wrong_file_name_print(file: NameAndLen) {
    with_stdout(|stdout| {
        let slice = unsafe { slice::from_raw_parts(file.name, file.len as usize) };
        write!(stdout, "I couldn't open file name `").unwrap();
        stdout.write_all(slice).unwrap();
        write!(stdout, "`\n").unwrap();
    })
}

#[no_mangle]
pub extern "C" fn print_aux_name() -> bool {
    if !print_a_pool_str(cur_aux()) {
        return false;
    }
    write_logs("\n");
    true
}

#[no_mangle]
pub unsafe extern "C" fn log_pr_aux_name() -> bool {
    if !out_pool_str(bib_log_file(), cur_aux()) {
        return false;
    }
    write_logs("\n");
    true
}

#[no_mangle]
pub unsafe extern "C" fn aux_err_print() -> bool {
    write_logs(&format!("---line {} of file ", cur_aux_ln()));
    if !print_aux_name() {
        return false;
    }
    print_bad_input_line();
    print_skipping_whatever_remains();
    write_logs("command\n");
    true
}

#[no_mangle]
pub extern "C" fn aux_err_illegal_another_print(cmd_num: i32) -> bool {
    write_logs("Illegal, another \\bib");
    match cmd_num {
        0 => write_logs("data"),
        1 => write_logs("style"),
        _ => {
            write_logs("Illegal auxiliary-file command");
            print_confusion();
            return false;
        }
    }
    write_logs(" command");
    true
}

#[no_mangle]
pub extern "C" fn aux_err_no_right_brace_print() {
    write_logs("No \"}\"");
}

#[no_mangle]
pub extern "C" fn aux_err_stuff_after_right_brace_print() {
    write_logs("Stuff after \"}\"");
}

#[no_mangle]
pub extern "C" fn aux_err_white_space_in_argument_print() {
    write_logs("White space in argument");
}

#[no_mangle]
pub extern "C" fn aux_end1_err_print() {
    write_logs("I found no ");
}

#[no_mangle]
pub extern "C" fn aux_end2_err_print() -> bool {
    write_logs("---while reading file ");
    if !print_aux_name() {
        return false;
    }
    mark_error();
    true
}
