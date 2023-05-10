use crate::c_api::{History, ttstub_output_open, ttstub_output_open_stdout};
use std::cell::Cell;
use std::ffi::CStr;
use std::io::Write;
use std::ptr;
use tectonic_io_base::OutputHandle;
use crate::c_api::history::{mark_fatal, set_history};

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

fn write_logs(str: &str) {
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
