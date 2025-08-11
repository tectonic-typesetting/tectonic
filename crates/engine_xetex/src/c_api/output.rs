use crate::c_api::engine::with_tex_string;
use crate::c_api::inputs::{FileCtx, FILE_CTX};
use std::cell::RefCell;
use std::ptr;
use std::ptr::NonNull;
use tectonic_bridge_core::{CoreBridgeState, Diagnostic};

thread_local! {
    pub static OUTPUT_CTX: RefCell<OutputCtx> = const { RefCell::new(OutputCtx::new()) }
}

pub struct OutputCtx {
    current_diagnostic: Option<Box<Diagnostic>>,
    file_line_error_style_p: i32,
    term_offset: i32,
    file_offset: i32,
}

impl OutputCtx {
    const fn new() -> OutputCtx {
        OutputCtx {
            current_diagnostic: None,
            file_line_error_style_p: 0,
            term_offset: 0,
            file_offset: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn file_line_error_style_p() -> i32 {
    OUTPUT_CTX.with_borrow(|out| out.file_line_error_style_p)
}

#[no_mangle]
pub extern "C" fn set_file_line_error_style_p(val: i32) {
    OUTPUT_CTX.with_borrow_mut(|out| out.file_line_error_style_p = val)
}

#[no_mangle]
pub extern "C" fn current_diagnostic() -> *mut Diagnostic {
    OUTPUT_CTX.with_borrow_mut(|out| {
        out.current_diagnostic
            .as_mut()
            .map(|b| ptr::from_mut(&mut **b))
            .unwrap_or(ptr::null_mut())
    })
}

#[no_mangle]
pub extern "C" fn term_offset() -> i32 {
    OUTPUT_CTX.with_borrow(|out| out.term_offset)
}

#[no_mangle]
pub extern "C" fn set_term_offset(val: i32) {
    OUTPUT_CTX.with_borrow_mut(|out| out.term_offset = val)
}

#[no_mangle]
pub extern "C" fn file_offset() -> i32 {
    OUTPUT_CTX.with_borrow(|out| out.file_offset)
}

#[no_mangle]
pub extern "C" fn set_file_offset(val: i32) {
    OUTPUT_CTX.with_borrow_mut(|out| out.file_offset = val)
}

fn rs_capture_to_diagnostic(
    state: &mut CoreBridgeState<'_>,
    out: &mut OutputCtx,
    diagnostic: Option<Box<Diagnostic>>,
) {
    if let Some(diag) = out.current_diagnostic.take() {
        state.finish_diagnostic(*diag);
    }
    out.current_diagnostic = diagnostic;
}

#[no_mangle]
pub unsafe extern "C" fn capture_to_diagnostic(diagnostic: Option<NonNull<Diagnostic>>) {
    OUTPUT_CTX.with_borrow_mut(|out| {
        CoreBridgeState::with_global_state(|state| {
            rs_capture_to_diagnostic(
                state,
                out,
                diagnostic.map(|ptr| Box::from_raw(ptr.as_ptr())),
            )
        })
    })
}

unsafe fn rs_diagnostic_print_file_line(files: &mut FileCtx, diag: &mut Diagnostic) {
    let mut level = files.in_open as usize;
    while level > 0 && files.full_source_filename_stack[level] == 0 {
        level -= 1;
    }

    if level == 0 {
        diag.append("!");
    } else {
        let mut source_line = files.line;
        if level != files.in_open as usize {
            source_line = files.line_stack[level + 1];
        }

        with_tex_string(files.full_source_filename_stack[level], |filename| {
            diag.append(format!("{}:{}", filename.to_string_lossy(), source_line));
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn diagnostic_print_file_line(diagnostic: *mut Diagnostic) {
    FILE_CTX.with_borrow_mut(|files| rs_diagnostic_print_file_line(files, &mut *diagnostic))
}

#[no_mangle]
pub unsafe extern "C" fn diagnostic_begin_capture_warning_here() -> *mut Diagnostic {
    let mut warning = Diagnostic::warning();
    FILE_CTX.with_borrow_mut(|files| rs_diagnostic_print_file_line(files, &mut warning));
    OUTPUT_CTX.with_borrow_mut(|out| {
        CoreBridgeState::with_global_state(|state| {
            rs_capture_to_diagnostic(state, out, Some(Box::new(warning)));
            ptr::from_mut(out.current_diagnostic.as_deref_mut().unwrap())
        })
    })
}

#[no_mangle]
pub extern "C" fn warn_char(c: libc::c_int) {
    OUTPUT_CTX.with_borrow_mut(|out| {
        if let Some(diag) = out.current_diagnostic.as_deref_mut() {
            diag.append_char(char::from_u32(c as u32).unwrap_or(char::REPLACEMENT_CHARACTER));
        }
    })
}

/*
void
print_ln(void)
{
    switch (selector) {
    case SELECTOR_TERM_AND_LOG:
        warn_char('\n');
        ttstub_output_putc(rust_stdout, '\n');
        ttstub_output_putc(log_file, '\n');
        term_offset = 0;
        file_offset = 0;
        break;
    case SELECTOR_LOG_ONLY:
        warn_char('\n');
        ttstub_output_putc(log_file, '\n');
        file_offset = 0;
        break;
    case SELECTOR_TERM_ONLY:
        warn_char('\n');
        ttstub_output_putc(rust_stdout, '\n');
        term_offset = 0;
        break;
    case SELECTOR_NO_PRINT:
    case SELECTOR_PSEUDO:
    case SELECTOR_NEW_STRING:
        break;
    default:
        ttstub_output_putc(write_file[selector], '\n');
        break;
    }
}
 */

// #[no_mangle]
// pub unsafe extern "C" fn error_here_with_diagnostic(
//     message: *const libc::c_char,
// ) -> *mut Diagnostic {
//     let message = unsafe { CStr::from_ptr(message) };
//     let mut diag = Diagnostic::error();
//     FILE_CTX.with_borrow_mut(|files| {
//         rs_diagnostic_print_file_line(files, &mut diag);
//     });
//     diag.append(message.to_string_lossy());
//
//     OUTPUT_CTX.with_borrow(|out| if out.file_line_error_style_p {
//         print_file_line()
//     } else {
//         print_nl_str("! ")
//     });
//     rs_print_cstr(message);
//     capture_to_diagnostic(&mut diag);
//     Box::into_raw(Box::new(diag))
// }

/*
void
print_file_line(void)
{
    int32_t level = in_open();

    while ((level > 0) && (full_source_filename_stack(level) == 0))
        level--;

    if (level == 0)
        print_nl_cstr("! ");
    else {
        print_nl_cstr("");
        print(full_source_filename_stack(level));
        print(':');
        if (level == in_open())
            print_int(line());
        else
            print_int(line_stack(level + 1));
        print_cstr(": ");
    }
}
 */

// pub fn rs_print_file_line(files: &mut FileCtx) {
//     let level = files.in_open;
//     let mut level = files.in_open as usize;
//     while level > 0 && files.full_source_filename_stack[level] == 0 {
//         level -= 1;
//     }
//
//     if level == 0 {
//         rs_print_nl_str("! ");
//     } else {
//         rs_print_nl_str("");
//         print(files.full_source_filename_stack[level]);
//         print(':');
//
//         if level == files.in_open {
//             print_int(files.line);
//         } else {
//             print_int(files.line_stack[level + 1]);
//         }
//         print_str(": ");
//     }
// }

// #[no_mangle]
// pub extern "C" fn print_file_line() {
//     let level =
// }
