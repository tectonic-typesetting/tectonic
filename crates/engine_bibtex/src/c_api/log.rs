use crate::c_api::{History, set_history, ttstub_output_open, ttstub_output_open_stdout};
use std::cell::Cell;
use std::ptr;
use tectonic_io_base::OutputHandle;

thread_local! {
    static STANDARD_OUTPUT: Cell<*mut OutputHandle> = Cell::new(ptr::null_mut());
    static LOG_FILE: Cell<*mut OutputHandle> = Cell::new(ptr::null_mut());
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
