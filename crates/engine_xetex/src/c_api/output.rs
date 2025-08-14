use crate::c_api::engine::{with_tex_string, EngineCtx, IntPar, Selector, ENGINE_CTX};
use crate::c_api::inputs::{FileCtx, FILE_CTX};
use crate::c_api::pool::{StringPool, STRING_POOL};
use std::cell::RefCell;
use std::ffi::CStr;
use std::io::Write;
use std::ptr;
use std::ptr::NonNull;
use tectonic_bridge_core::{CoreBridgeState, Diagnostic, OutputId};

pub const MAX_PRINT_LINE: usize = 79;
pub const BIGGEST_USV: i32 = 0x10FFFF;

thread_local! {
    pub static OUTPUT_CTX: RefCell<OutputCtx> = const { RefCell::new(OutputCtx::new()) }
}

pub struct OutputCtx {
    current_diagnostic: Option<Box<Diagnostic>>,
    file_line_error_style_p: i32,
    term_offset: i32,
    file_offset: i32,
    rust_stdout: Option<OutputId>,
    log_file: Option<OutputId>,
    write_file: Vec<Option<OutputId>>,
    doing_special: bool,
}

impl OutputCtx {
    const fn new() -> OutputCtx {
        OutputCtx {
            current_diagnostic: None,
            file_line_error_style_p: 0,
            term_offset: 0,
            file_offset: 0,
            rust_stdout: None,
            log_file: None,
            write_file: Vec::new(),
            doing_special: false,
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

#[no_mangle]
pub extern "C" fn rust_stdout() -> Option<OutputId> {
    OUTPUT_CTX.with_borrow(|out| out.rust_stdout)
}

#[no_mangle]
pub extern "C" fn set_rust_stdout(val: Option<OutputId>) {
    OUTPUT_CTX.with_borrow_mut(|out| out.rust_stdout = val)
}

#[no_mangle]
pub extern "C" fn log_file() -> Option<OutputId> {
    OUTPUT_CTX.with_borrow(|out| out.log_file)
}

#[no_mangle]
pub extern "C" fn set_log_file(val: Option<OutputId>) {
    OUTPUT_CTX.with_borrow_mut(|out| out.log_file = val)
}

#[no_mangle]
pub extern "C" fn write_file(idx: usize) -> Option<OutputId> {
    OUTPUT_CTX.with_borrow(|out| out.write_file[idx])
}

#[no_mangle]
pub extern "C" fn set_write_file(idx: usize, val: Option<OutputId>) {
    OUTPUT_CTX.with_borrow_mut(|out| {
        if out.write_file.len() < idx + 1 {
            out.write_file.resize(idx + 1, None);
        }
        out.write_file[idx] = val;
    })
}

#[no_mangle]
pub extern "C" fn doing_special() -> bool {
    OUTPUT_CTX.with_borrow(|out| out.doing_special)
}

#[no_mangle]
pub extern "C" fn set_doing_special(val: bool) {
    OUTPUT_CTX.with_borrow_mut(|out| out.doing_special = val)
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

pub fn rs_warn_char(out: &mut OutputCtx, c: char) {
    if let Some(diag) = out.current_diagnostic.as_deref_mut() {
        diag.append_char(c);
    }
}

#[no_mangle]
pub extern "C" fn warn_char(c: libc::c_int) {
    OUTPUT_CTX.with_borrow_mut(|out| {
        rs_warn_char(
            out,
            char::from_u32(c as u32).unwrap_or(char::REPLACEMENT_CHARACTER),
        );
    })
}

pub fn rs_print_ln(state: &mut CoreBridgeState<'_>, engine: &mut EngineCtx, out: &mut OutputCtx) {
    match engine.selector {
        Selector::File(val) => {
            // TODO: Replace all write!(get_output) with output_write on state
            write!(
                state.get_output(out.write_file[val as usize].unwrap()),
                "\n"
            )
            .unwrap();
        }
        Selector::TermOnly => {
            rs_warn_char(out, '\n');
            write!(state.get_output(out.rust_stdout.unwrap()), "\n").unwrap();
            out.term_offset = 0;
        }
        Selector::LogOnly => {
            rs_warn_char(out, '\n');
            write!(state.get_output(out.log_file.unwrap()), "\n").unwrap();
            out.file_offset = 0;
        }
        Selector::TermAndLog => {
            rs_warn_char(out, '\n');
            write!(state.get_output(out.rust_stdout.unwrap()), "\n").unwrap();
            write!(state.get_output(out.log_file.unwrap()), "\n").unwrap();
            out.term_offset = 0;
            out.file_offset = 0;
        }
        Selector::NoPrint | Selector::Pseudo | Selector::NewString => {}
    }
}

#[no_mangle]
pub extern "C" fn print_ln() {
    CoreBridgeState::with_global_state(|state| {
        ENGINE_CTX.with_borrow_mut(|engine| {
            OUTPUT_CTX.with_borrow_mut(|out| rs_print_ln(state, engine, out))
        })
    })
}

pub fn rs_print_raw_char(
    state: &mut CoreBridgeState,
    engine: &mut EngineCtx,
    out: &mut OutputCtx,
    strings: &mut StringPool,
    s: u16,
    incr_offset: bool,
) {
    let raw = &[s as u8];
    let c = char::from_u32(s as u32).unwrap_or(char::REPLACEMENT_CHARACTER);
    match engine.selector {
        Selector::TermAndLog => {
            // TODO: This produces a malformed warning currently, since we add unicode byte-by-byte
            rs_warn_char(out, c);
            state
                .get_output(out.rust_stdout.unwrap())
                .write(raw)
                .unwrap();
            state.get_output(out.log_file.unwrap()).write(raw).unwrap();
            if incr_offset {
                out.term_offset += 1;
                out.file_offset += 1;
            }
            if out.term_offset as usize == MAX_PRINT_LINE {
                writeln!(state.get_output(out.rust_stdout.unwrap())).unwrap();
                out.term_offset = 0;
            }
            if out.file_offset as usize == MAX_PRINT_LINE {
                writeln!(state.get_output(out.log_file.unwrap())).unwrap();
                out.file_offset = 0;
            }
        }
        Selector::LogOnly => {
            rs_warn_char(out, c);
            state.get_output(out.log_file.unwrap()).write(raw).unwrap();
            if incr_offset {
                out.file_offset += 1;
            }
            if out.file_offset as usize == MAX_PRINT_LINE {
                writeln!(state.get_output(out.log_file.unwrap())).unwrap();
                out.file_offset = 0;
            }
        }
        Selector::TermOnly => {
            rs_warn_char(out, c);
            state
                .get_output(out.rust_stdout.unwrap())
                .write(raw)
                .unwrap();
            if incr_offset {
                out.term_offset += 1;
            }
            if out.term_offset as usize == MAX_PRINT_LINE {
                writeln!(state.get_output(out.rust_stdout.unwrap())).unwrap();
                out.term_offset = 0;
            }
        }
        Selector::NoPrint => (),
        Selector::Pseudo => {
            if engine.tally < engine.trick_count {
                engine.trick_buf[(engine.tally % engine.error_line) as usize] = s;
            }
        }
        Selector::NewString => {
            if strings.pool_ptr < strings.pool_size {
                strings.str_pool[strings.pool_ptr] = s;
                strings.pool_ptr += 1;
            }
        }
        Selector::File(val) => {
            state
                .get_output(out.write_file[val as usize].unwrap())
                .write(raw)
                .unwrap();
        }
    }
    engine.tally += 1;
}

#[no_mangle]
pub extern "C" fn print_raw_char(s: u16, offset: u8) {
    CoreBridgeState::with_global_state(|state| {
        ENGINE_CTX.with_borrow_mut(|engine| {
            OUTPUT_CTX.with_borrow_mut(|out| {
                STRING_POOL.with_borrow_mut(|strings| {
                    rs_print_raw_char(state, engine, out, strings, s, offset != 0)
                })
            })
        })
    })
}

pub fn rs_print_char(
    state: &mut CoreBridgeState,
    engine: &mut EngineCtx,
    out: &mut OutputCtx,
    strings: &mut StringPool,
    s: i32,
) {
    if engine.selector == Selector::NewString && !out.doing_special {
        if let Ok(s) = s.try_into() {
            rs_print_raw_char(state, engine, out, strings, s, true)
        } else {
            let s = (s - 0x10000) as u16;
            rs_print_raw_char(state, engine, out, strings, 0xD800 + s / 1024, true);
            rs_print_raw_char(state, engine, out, strings, 0xDC00 + s % 1024, true)
        }
        return;
    }

    if engine.int_par(IntPar::NewLineChar) == s
        && !matches!(engine.selector, Selector::Pseudo | Selector::NewString)
    {
        rs_print_ln(state, engine, out);
        return;
    }

    if s < 32 && !out.doing_special {
        rs_print_raw_char(state, engine, out, strings, b'^' as u16, true);
        rs_print_raw_char(state, engine, out, strings, b'^' as u16, true);
        rs_print_raw_char(state, engine, out, strings, (s + 64) as u16, true);
    } else if s < 127 {
        rs_print_raw_char(state, engine, out, strings, s as u16, true);
    } else if s == 127 {
        if !out.doing_special {
            rs_print_raw_char(state, engine, out, strings, b'^' as u16, true);
            rs_print_raw_char(state, engine, out, strings, b'^' as u16, true);
            rs_print_raw_char(state, engine, out, strings, b'?' as u16, true);
        } else {
            rs_print_raw_char(state, engine, out, strings, s as u16, true);
        }
    } else if s < 160 && !out.doing_special {
        rs_print_raw_char(state, engine, out, strings, b'^' as u16, true);
        rs_print_raw_char(state, engine, out, strings, b'^' as u16, true);

        let l = (s % 256 / 16) as u16;
        if l < 10 {
            rs_print_raw_char(state, engine, out, strings, b'0' as u16 + l, true);
        } else {
            rs_print_raw_char(state, engine, out, strings, b'a' as u16 + l - 10, true);
        }

        let l = (s % 16) as u16;
        if l < 10 {
            rs_print_raw_char(state, engine, out, strings, b'0' as u16 + l, true);
        } else {
            rs_print_raw_char(state, engine, out, strings, b'a' as u16 + l - 10, true);
        }
    } else if engine.selector == Selector::Pseudo {
        rs_print_raw_char(state, engine, out, strings, s as u16, true);
    } else {
        // Encode into UTF-8
        if s < 2048 {
            rs_print_raw_char(state, engine, out, strings, (192 + s / 64) as u16, false);
            rs_print_raw_char(state, engine, out, strings, (128 + s % 64) as u16, true);
        } else if s < 0x10000 {
            rs_print_raw_char(state, engine, out, strings, (224 + s / 4096) as u16, false);
            rs_print_raw_char(
                state,
                engine,
                out,
                strings,
                (128 + s % 4096 / 64) as u16,
                false,
            );
            rs_print_raw_char(state, engine, out, strings, (128 + s % 64) as u16, true);
        } else {
            rs_print_raw_char(
                state,
                engine,
                out,
                strings,
                (240 + s / 0x40000) as u16,
                false,
            );
            rs_print_raw_char(
                state,
                engine,
                out,
                strings,
                (128 + s % 0x40000 / 4096) as u16,
                false,
            );
            rs_print_raw_char(
                state,
                engine,
                out,
                strings,
                (128 + s % 4096 / 64) as u16,
                false,
            );
            rs_print_raw_char(state, engine, out, strings, (128 + s % 64) as u16, true);
        }
    }
}

#[no_mangle]
pub extern "C" fn print_char(s: i32) {
    CoreBridgeState::with_global_state(|state| {
        ENGINE_CTX.with_borrow_mut(|engine| {
            OUTPUT_CTX.with_borrow_mut(|out| {
                STRING_POOL.with_borrow_mut(|strings| rs_print_char(state, engine, out, strings, s))
            })
        })
    })
}

pub fn rs_print_bytes(
    state: &mut CoreBridgeState,
    engine: &mut EngineCtx,
    out: &mut OutputCtx,
    strings: &mut StringPool,
    bytes: &[u8],
) {
    for b in bytes {
        rs_print_char(state, engine, out, strings, *b as i32)
    }
}

pub fn rs_print_nl_bytes(
    state: &mut CoreBridgeState,
    engine: &mut EngineCtx,
    out: &mut OutputCtx,
    strings: &mut StringPool,
    bytes: &[u8],
) {
    if (out.term_offset > 0 && matches!(engine.selector, Selector::TermOnly | Selector::TermAndLog))
        || (out.file_offset > 0
            && matches!(engine.selector, Selector::LogOnly | Selector::TermAndLog))
    {
        rs_print_ln(state, engine, out);
    }
    rs_print_bytes(state, engine, out, strings, bytes);
}

pub fn rs_print_esc_bytes(
    state: &mut CoreBridgeState,
    engine: &mut EngineCtx,
    out: &mut OutputCtx,
    strings: &mut StringPool,
    bytes: &[u8],
) {
    let c = engine.int_par(IntPar::EscapeChar);
    if c >= 0 && c <= BIGGEST_USV {
        rs_print_char(state, engine, out, strings, c);
    }
    rs_print_bytes(state, engine, out, strings, bytes);
}

#[no_mangle]
pub extern "C" fn print_cstr(str: *const libc::c_char) {
    let bytes = unsafe { CStr::from_ptr(str) }.to_bytes();
    CoreBridgeState::with_global_state(|state| {
        ENGINE_CTX.with_borrow_mut(|engine| {
            OUTPUT_CTX.with_borrow_mut(|out| {
                STRING_POOL
                    .with_borrow_mut(|strings| rs_print_bytes(state, engine, out, strings, bytes))
            })
        })
    })
}

#[no_mangle]
pub extern "C" fn print_nl_cstr(str: *const libc::c_char) {
    let bytes = unsafe { CStr::from_ptr(str) }.to_bytes();
    CoreBridgeState::with_global_state(|state| {
        ENGINE_CTX.with_borrow_mut(|engine| {
            OUTPUT_CTX.with_borrow_mut(|out| {
                STRING_POOL.with_borrow_mut(|strings| {
                    rs_print_nl_bytes(state, engine, out, strings, bytes)
                })
            })
        })
    })
}

#[no_mangle]
pub extern "C" fn print_esc_cstr(str: *const libc::c_char) {
    let bytes = unsafe { CStr::from_ptr(str) }.to_bytes();
    CoreBridgeState::with_global_state(|state| {
        ENGINE_CTX.with_borrow_mut(|engine| {
            OUTPUT_CTX.with_borrow_mut(|out| {
                STRING_POOL.with_borrow_mut(|strings| {
                    rs_print_esc_bytes(state, engine, out, strings, bytes)
                })
            })
        })
    })
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
