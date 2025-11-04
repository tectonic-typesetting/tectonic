use crate::c_api::engine::{
    close_files_and_terminate, rs_close_files_and_terminate, rs_open_log_file, rs_show_context,
    rs_token_show, rs_tt_cleanup, History, InteractionMode, Local, Selector,
};
use crate::c_api::globals::Globals;
use crate::c_api::output::{
    rs_capture_to_diagnostic, rs_error_here_with_diagnostic, rs_print_bytes, rs_print_char,
    rs_print_int, rs_print_ln, rs_print_nl_bytes,
};
use std::ffi::CStr;
use std::hint::unreachable_unchecked;

pub fn rs_pre_error_message(globals: &mut Globals<'_, '_>) {
    if globals.engine.log_opened {
        globals.engine.selector = Selector::TermAndLog;
    } else {
        globals.engine.selector = Selector::TermOnly;
    }

    if globals.engine.job_name == 0 {
        rs_open_log_file(globals);
    }

    if globals.engine.interaction == InteractionMode::Batch {
        globals.engine.selector = match globals.engine.selector {
            Selector::TermAndLog => Selector::LogOnly,
            Selector::TermOnly => Selector::NoPrint,
            _ => panic!(),
        }
    }

    rs_error_here_with_diagnostic(globals, b"");
}

#[no_mangle]
extern "C" fn pre_error_message() {
    Globals::with(|globals| rs_pre_error_message(globals))
}

pub fn give_err_help(globals: &mut Globals<'_, '_>) {
    rs_token_show(globals, globals.engine.local(Local::ErrHelp) as usize);
}

pub fn rs_error(globals: &mut Globals<'_, '_>) {
    if globals.engine.history < History::ErrorIssued {
        globals.engine.history = History::ErrorIssued;
    }

    rs_print_char(globals, '.' as i32);
    rs_show_context(globals);
    if globals.engine.halt_on_error_p != 0 {
        globals.engine.history = History::FatalError;
        // Execute this outside the globals lock for now
        rs_post_error_message(globals, 0);
        panic!("halted on potentially-recoverable error as specified");
    }

    /* This used to be where there was a bunch of code if "interaction ==
     * error_stop_mode" that would let the use interactively try to solve the
     * error. */

    globals.engine.error_count += 1;
    if globals.engine.error_count == 100 {
        rs_print_nl_bytes(globals, b"(That makes 100 errors; please try again.)");
        globals.engine.history = History::FatalError;
        rs_post_error_message(globals, 0);
        panic!("halted after 100 potentially-recoverable errors");
    }

    if globals.engine.interaction != InteractionMode::Batch {
        globals.engine.selector = match globals.engine.selector {
            Selector::File(i) => Selector::File(i - 1),
            Selector::NoPrint => Selector::File(15),
            Selector::TermOnly => Selector::NoPrint,
            Selector::LogOnly => Selector::TermOnly,
            Selector::TermAndLog => Selector::LogOnly,
            Selector::Pseudo => Selector::TermAndLog,
            Selector::NewString => Selector::Pseudo,
        }
    }

    if globals.engine.use_err_help {
        rs_print_ln(globals);
        give_err_help(globals);
    } else {
        while globals.engine.help_ptr > 0 {
            globals.engine.help_ptr -= 1;
            rs_print_nl_bytes(globals, unsafe {
                CStr::from_ptr(globals.engine.help_line[globals.engine.help_ptr]).to_bytes()
            });
        }
    }

    rs_print_ln(globals);
    if globals.engine.interaction != InteractionMode::Batch {
        globals.engine.selector = match globals.engine.selector {
            Selector::File(15) => Selector::NoPrint,
            Selector::File(i) => Selector::File(i + 1),
            Selector::NoPrint => Selector::TermOnly,
            Selector::TermOnly => Selector::LogOnly,
            Selector::LogOnly => Selector::TermAndLog,
            Selector::TermAndLog => Selector::Pseudo,
            Selector::Pseudo => Selector::NewString,
            Selector::NewString => unreachable!(),
        }
    }
    rs_print_ln(globals);
}

#[no_mangle]
extern "C-unwind" fn error() {
    Globals::with(|globals| rs_error(globals))
}

pub fn rs_post_error_message(globals: &mut Globals<'_, '_>, need_to_print_it: i32) {
    rs_capture_to_diagnostic(globals, None);
    if globals.engine.interaction == InteractionMode::ErrorStop {
        globals.engine.interaction = InteractionMode::Scroll;
    }

    if need_to_print_it != 0 && globals.engine.log_opened {
        rs_error(globals);
    }
    globals.engine.history = History::FatalError;
    rs_close_files_and_terminate(globals);
    rs_tt_cleanup(globals);
    globals
        .out
        .rust_stdout
        .map(|stdout| globals.state.output_flush(stdout));
}

#[no_mangle]
extern "C" fn post_error_message(need_to_print_it: i32) {
    Globals::with(|globals| rs_post_error_message(globals, need_to_print_it))
}

pub fn rs_fatal_error(globals: &mut Globals<'_, '_>, s: &[u8]) -> ! {
    rs_pre_error_message(globals);
    rs_print_bytes(globals, b"Emergency stop");
    rs_print_nl_bytes(globals, s);
    rs_capture_to_diagnostic(globals, None);
    rs_close_files_and_terminate(globals);
    rs_tt_cleanup(globals);
    globals
        .out
        .rust_stdout
        .map(|stdout| globals.state.output_flush(stdout));
    unsafe { _tt_abort(s.as_ptr().cast()) };
    // _tt_abort actually does a longjmp (I know, I know, I'll fix it later)
    unsafe { unreachable_unchecked() }
}

#[no_mangle]
pub extern "C-unwind" fn fatal_error(s: *const libc::c_char) {
    let s = unsafe { CStr::from_ptr(s) }.to_bytes();
    Globals::with(|globals| rs_fatal_error(globals, s))
}

pub fn rs_int_error(globals: &mut Globals<'_, '_>, n: i32) {
    rs_print_bytes(globals, b" (");
    rs_print_int(globals, n);
    rs_print_char(globals, ')' as i32);
    rs_error(globals);
}

#[no_mangle]
pub extern "C" fn int_error(n: i32) {
    Globals::with(|globals| rs_int_error(globals, n))
}

extern "C" {
    fn _tt_abort(s: *const libc::c_char, ...) -> libc::c_int;
}
