#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_output_flush(handle: rust_output_handle_t) -> i32;
    #[no_mangle]
    static mut file_line_error_style_p: i32;
    #[no_mangle]
    static mut halt_on_error_p: i32;
    #[no_mangle]
    static mut rust_stdout: rust_output_handle_t;
    #[no_mangle]
    static mut selector: selector_t;
    #[no_mangle]
    static mut interaction: u8;
    #[no_mangle]
    static mut history: tt_history_t;
    #[no_mangle]
    static mut error_count: i8;
    #[no_mangle]
    static mut help_line: [*const i8; 6];
    #[no_mangle]
    static mut help_ptr: u8;
    #[no_mangle]
    static mut use_err_help: bool;
    #[no_mangle]
    static mut job_name: str_number;
    #[no_mangle]
    static mut log_opened: bool;
    #[no_mangle]
    fn show_context();
    #[no_mangle]
    fn open_log_file();
    #[no_mangle]
    fn give_err_help();
    #[no_mangle]
    fn close_files_and_terminate();
    #[no_mangle]
    fn print_ln();
    #[no_mangle]
    fn print_nl_cstr(s: *const i8);
    #[no_mangle]
    fn print_char(s: i32);
    #[no_mangle]
    fn print_cstr(s: *const i8);
    #[no_mangle]
    fn print_file_line();
    #[no_mangle]
    fn print_int(n: i32);
    #[no_mangle]
    fn print(s: i32);
}
/* tectonic/core-bridge.h: declarations of C/C++ => Rust bridge API
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
/* Both XeTeX and bibtex use this enum: */
pub type tt_history_t = u32;
pub const HISTORY_FATAL_ERROR: tt_history_t = 3;
pub const HISTORY_ERROR_ISSUED: tt_history_t = 2;
pub const HISTORY_WARNING_ISSUED: tt_history_t = 1;
pub const HISTORY_SPOTLESS: tt_history_t = 0;
pub type rust_output_handle_t = *mut libc::c_void;
pub type selector_t = u32;
pub const SELECTOR_NEW_STRING: selector_t = 21;
pub const SELECTOR_PSEUDO: selector_t = 20;
pub const SELECTOR_TERM_AND_LOG: selector_t = 19;
pub const SELECTOR_LOG_ONLY: selector_t = 18;
pub const SELECTOR_TERM_ONLY: selector_t = 17;
pub const SELECTOR_NO_PRINT: selector_t = 16;
pub const SELECTOR_FILE_15: selector_t = 15;
pub const SELECTOR_FILE_0: selector_t = 0;
pub type str_number = i32;
/* tectonic/errors.c -- error handling
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/
/* WEBby error-handling code: */
unsafe extern "C" fn pre_error_message() {
    /* FKA normalize_selector(): */
    if log_opened {
        selector = SELECTOR_TERM_AND_LOG
    } else {
        selector = SELECTOR_TERM_ONLY
    }
    if job_name == 0i32 {
        open_log_file();
    }
    if interaction as i32 == 0i32 {
        selector -= 1
    }
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! \x00" as *const u8 as *const i8);
    };
}
/*82: */
unsafe extern "C" fn post_error_message(mut need_to_print_it: i32) {
    if interaction as i32 == 3i32 {
        interaction = 2_u8
    }
    if need_to_print_it != 0 && log_opened as i32 != 0 {
        error();
    }
    history = HISTORY_FATAL_ERROR;
    close_files_and_terminate();
    ttstub_output_flush(rust_stdout);
}
#[no_mangle]
pub unsafe extern "C" fn error() {
    if (history as u32) < HISTORY_ERROR_ISSUED as i32 as u32 {
        history = HISTORY_ERROR_ISSUED
    }
    print_char('.' as i32);
    show_context();
    if halt_on_error_p != 0 {
        history = HISTORY_FATAL_ERROR;
        post_error_message(0i32);
        _tt_abort(
            b"halted on potentially-recoverable error as specified\x00" as *const u8 as *const i8,
        );
    }
    /* This used to be where there was a bunch of code if "interaction ==
     * error_stop_mode" that would let the use interactively try to solve the
     * error. */
    error_count += 1;
    if error_count as i32 == 100i32 {
        print_nl_cstr(b"(That makes 100 errors; please try again.)\x00" as *const u8 as *const i8);
        history = HISTORY_FATAL_ERROR;
        post_error_message(0i32);
        _tt_abort(b"halted after 100 potentially-recoverable errors\x00" as *const u8 as *const i8);
    }
    if interaction as i32 > 0i32 {
        selector -= 1
    }
    if use_err_help {
        print_ln();
        give_err_help();
    } else {
        while help_ptr as i32 > 0i32 {
            help_ptr = help_ptr.wrapping_sub(1);
            print_nl_cstr(help_line[help_ptr as usize]);
        }
    }
    print_ln();
    if interaction as i32 > 0i32 {
        selector += 1
    }
    print_ln();
}
#[no_mangle]
pub unsafe extern "C" fn fatal_error(mut s: *const i8) -> ! {
    pre_error_message();
    print_cstr(b"Emergency stop\x00" as *const u8 as *const i8);
    print_nl_cstr(s);
    close_files_and_terminate();
    ttstub_output_flush(rust_stdout);
    _tt_abort(b"%s\x00" as *const u8 as *const i8, s);
}
#[no_mangle]
pub unsafe extern "C" fn overflow(mut s: *const i8, mut n: i32) -> ! {
    pre_error_message();
    print_cstr(b"TeX capacity exceeded, sorry [\x00" as *const u8 as *const i8);
    print_cstr(s);
    print_char('=' as i32);
    print_int(n);
    print_char(']' as i32);
    help_ptr = 2_u8;
    help_line[1] = b"If you really absolutely need more capacity,\x00" as *const u8 as *const i8;
    help_line[0] = b"you can ask a wizard to enlarge me.\x00" as *const u8 as *const i8;
    post_error_message(1i32);
    _tt_abort(b"halted on overflow()\x00" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn confusion(mut s: *const i8) -> ! {
    pre_error_message();
    if (history as u32) < HISTORY_ERROR_ISSUED as i32 as u32 {
        print_cstr(b"This can\'t happen (\x00" as *const u8 as *const i8);
        print_cstr(s);
        print_char(')' as i32);
        help_ptr = 1_u8;
        help_line[0] = b"I\'m broken. Please show this to someone who can fix can fix\x00"
            as *const u8 as *const i8
    } else {
        print_cstr(b"I can\'t go on meeting you like this\x00" as *const u8 as *const i8);
        help_ptr = 2_u8;
        help_line[1] = b"One of your faux pas seems to have wounded me deeply...\x00" as *const u8
            as *const i8;
        help_line[0] = b"in fact, I\'m barely conscious. Please fix it and try again.\x00"
            as *const u8 as *const i8
    }
    post_error_message(1i32);
    _tt_abort(b"halted on confusion()\x00" as *const u8 as *const i8);
}
/* xetex-errors */
#[no_mangle]
pub unsafe extern "C" fn pdf_error(mut t: *const i8, mut p: *const i8) -> ! {
    pre_error_message();
    print_cstr(b"Error\x00" as *const u8 as *const i8);
    if !t.is_null() {
        print_cstr(b" (\x00" as *const u8 as *const i8);
        print_cstr(t);
        print(')' as i32);
    }
    print_cstr(b": \x00" as *const u8 as *const i8);
    print_cstr(p);
    post_error_message(1i32);
    _tt_abort(b"halted on pdf_error()\x00" as *const u8 as *const i8);
}
