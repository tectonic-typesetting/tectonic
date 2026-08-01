use crate::c_api::engine::{
    close_files_and_terminate, rs_close_files_and_terminate, rs_open_log_file, rs_show_context,
    rs_token_show, rs_tt_cleanup, History, InteractionMode, Local, Selector,
};
use crate::c_api::globals::Globals;
use crate::c_api::output::{
    rs_capture_to_diagnostic, rs_error_here_with_diagnostic, rs_print, rs_print_bytes,
    rs_print_char, rs_print_int, rs_print_ln, rs_print_nl_bytes,
};
use std::ffi::CStr;

struct Error;

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
    // SAFETY: YOLO
    unsafe { _tt_abort(s.as_ptr().cast()) };
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

pub fn rs_overflow(globals: &mut Globals<'_, '_>, s: &[u8], n: i32) -> ! {
    pre_error_message();
    rs_print_bytes(globals, b"TeX capacity exceeded, sorry [");
    rs_print_bytes(globals, s);
    rs_print_char(globals, '=' as i32);
    rs_print_int(globals, n);
    rs_print_char(globals, ']' as i32);

    globals.engine.help_ptr = 2;
    globals.engine.help_line[1] = c"If you really absolutely need more capacity,".as_ptr();
    globals.engine.help_line[0] = c"you can ask a wizard to enlarge me.".as_ptr();
    rs_post_error_message(globals, 1);
    panic!("halted on overflow()")
}

#[no_mangle]
pub extern "C-unwind" fn overflow(s: *const libc::c_char, n: i32) {
    let s = unsafe { CStr::from_ptr(s).to_bytes() };
    Globals::with(|globals| rs_overflow(globals, s, n))
}

pub fn rs_confusion(globals: &mut Globals<'_, '_>, s: &[u8]) -> ! {
    rs_pre_error_message(globals);

    if (globals.engine.history < History::ErrorIssued) {
        rs_print_bytes(globals, b"This can't happen (");
        rs_print_bytes(globals, s);
        rs_print_char(globals, ')' as i32);

        globals.engine.help_ptr = 1;
        globals.engine.help_line[0] =
            c"I'm broken. Please show this to someone who can fix can fix".as_ptr();
    } else {
        rs_print_bytes(globals, b"I can't go on meeting you like this");

        globals.engine.help_ptr = 2;
        globals.engine.help_line[1] =
            c"One of your faux pas seems to have wounded me deeply...".as_ptr();
        globals.engine.help_line[0] =
            c"in fact, I'm barely conscious. Please fix it and try again.".as_ptr();
    }

    post_error_message(1);
    panic!("halted on confusion()")
}

#[no_mangle]
pub extern "C-unwind" fn confusion(s: *const libc::c_char) {
    let s = unsafe { CStr::from_ptr(s).to_bytes() };
    Globals::with(|globals| rs_confusion(globals, s))
}

pub fn rs_pdf_error(globals: &mut Globals<'_, '_>, t: Option<&[u8]>, p: &[u8]) -> ! {
    pre_error_message();

    rs_print_bytes(globals, b"Error");

    if let Some(t) = t {
        rs_print_bytes(globals, b" (");
        rs_print_bytes(globals, t);
        rs_print(globals, ')' as i32);
    }

    rs_print_bytes(globals, b": ");
    rs_print_bytes(globals, p);

    post_error_message(1);
    panic!("halted on pdf_error()")
}

#[no_mangle]
pub extern "C-unwind" fn pdf_error(t: *const libc::c_char, p: *const libc::c_char) {
    let t = if t.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(t).to_bytes() })
    };
    let p = unsafe { CStr::from_ptr(p).to_bytes() };
    Globals::with(|globals| rs_pdf_error(globals, t, p))
}

extern "C" {
    pub fn _tt_abort(s: *const libc::c_char, ...) -> !;
}
