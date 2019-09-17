#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::ttstub_output_flush;
use crate::xetex_ini::{
    error_count, file_line_error_style_p, halt_on_error_p, help_line, help_ptr, history,
    interaction, job_name, log_opened, rust_stdout, selector, use_err_help,
};
use crate::xetex_output::{
    print, print_char, print_cstr, print_file_line, print_int, print_ln, print_nl_cstr,
};
use crate::xetex_xetex0::{close_files_and_terminate, give_err_help, open_log_file, show_context};
use bridge::_tt_abort;

use crate::TTHistory;

pub type rust_output_handle_t = *mut libc::c_void;

use super::xetex_ini::Selector;

pub type str_number = i32;
/* tectonic/errors.c -- error handling
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/
/* WEBby error-handling code: */
unsafe extern "C" fn pre_error_message() {
    /* FKA normalize_selector(): */
    if log_opened {
        selector = Selector::TERM_AND_LOG
    } else {
        selector = Selector::TERM_ONLY
    }
    if job_name == 0i32 {
        open_log_file();
    }
    if interaction as i32 == 0i32 {
        selector = (u8::from(selector) - 1).into()
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
    history = TTHistory::FATAL_ERROR;
    close_files_and_terminate();
    ttstub_output_flush(rust_stdout);
}
#[no_mangle]
pub unsafe extern "C" fn error() {
    if (history as u32) < (TTHistory::ERROR_ISSUED as u32) {
        history = TTHistory::ERROR_ISSUED
    }
    print_char('.' as i32);
    show_context();
    if halt_on_error_p != 0 {
        history = TTHistory::FATAL_ERROR;
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
        history = TTHistory::FATAL_ERROR;
        post_error_message(0i32);
        panic!("halted after 100 potentially-recoverable errors");
    }
    if interaction as i32 > 0i32 {
        selector = (u8::from(selector) - 1).into()
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
        selector = (u8::from(selector) + 1).into()
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
    panic!("halted on overflow()");
}
#[no_mangle]
pub unsafe extern "C" fn confusion(mut s: *const i8) -> ! {
    pre_error_message();
    if (history as u32) < (TTHistory::ERROR_ISSUED as u32) {
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
    panic!("halted on confusion()");
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
    panic!("halted on pdf_error()");
}
