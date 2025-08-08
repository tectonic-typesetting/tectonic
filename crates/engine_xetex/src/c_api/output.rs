use crate::c_api::engine::with_tex_string;
use crate::c_api::inputs::{FileCtx, FILE_CTX};
use std::cell::RefCell;
use tectonic_bridge_core::Diagnostic;

thread_local! {
    pub static OUTPUT_CTX: RefCell<OutputCtx> = const { RefCell::new(OutputCtx::new()) }
}

pub struct OutputCtx {
    file_line_error_style_p: i32,
}

impl OutputCtx {
    const fn new() -> OutputCtx {
        OutputCtx {
            file_line_error_style_p: 0,
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
