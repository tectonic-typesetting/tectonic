use crate::c_api::engine::{
    rs_gettexstring, CatCode, IntPar, NativeWordNode, Selector, ACTIVE_BASE, EQTB_SIZE,
    FROZEN_NULL_FONT, NULL_CS, PRIM_EQTB_BASE, SCRIPT_SIZE, SINGLE_BASE, TEXT_SIZE,
    UNDEFINED_CONTROL_SEQUENCE,
};
use crate::c_api::globals::Globals;
use crate::c_api::hash::HASH_BASE;
use crate::ty::StrNumber;
use std::cell::RefCell;
use std::ffi::CStr;
use std::io::Write;
use std::ptr;
use std::ptr::NonNull;
use tectonic_bridge_core::{Diagnostic, OutputId};

pub const MAX_PRINT_LINE: usize = 79;
pub const BIGGEST_CHAR: i32 = 0xFFFF;
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
    digits: [u8; 23],
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
            digits: [0; 23],
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

#[no_mangle]
pub extern "C" fn dig(idx: usize) -> u8 {
    OUTPUT_CTX.with_borrow(|out| out.digits[idx])
}

#[no_mangle]
pub extern "C" fn set_dig(idx: usize, val: u8) {
    OUTPUT_CTX.with_borrow_mut(|out| out.digits[idx] = val)
}

fn rs_capture_to_diagnostic(globals: &mut Globals<'_, '_>, diagnostic: Option<Box<Diagnostic>>) {
    if let Some(diag) = globals.out.current_diagnostic.take() {
        globals.state.finish_diagnostic(*diag);
    }
    globals.out.current_diagnostic = diagnostic;
}

#[no_mangle]
pub unsafe extern "C" fn capture_to_diagnostic(diagnostic: Option<NonNull<Diagnostic>>) {
    Globals::with(|globals| {
        rs_capture_to_diagnostic(globals, diagnostic.map(|ptr| Box::from_raw(ptr.as_ptr())))
    })
}

pub fn rs_diagnostic_print_file_line(globals: &mut Globals<'_, '_>, diag: &mut Diagnostic) {
    let mut level = globals.files.in_open as usize;
    while level > 0 && globals.files.full_source_filename_stack[level] == 0 {
        level -= 1;
    }

    if level == 0 {
        diag.append("!");
    } else {
        let mut source_line = globals.files.line;
        if level != globals.files.in_open as usize {
            source_line = globals.files.line_stack[level + 1];
        }

        let filename = rs_gettexstring(globals, globals.files.full_source_filename_stack[level]);
        diag.append(format!("{}:{}", filename, source_line));
    }
}

#[no_mangle]
pub unsafe extern "C" fn diagnostic_print_file_line(diagnostic: *mut Diagnostic) {
    Globals::with(|globals| rs_diagnostic_print_file_line(globals, &mut *diagnostic))
}

#[no_mangle]
pub extern "C" fn diagnostic_begin_capture_warning_here() -> *mut Diagnostic {
    let mut warning = Diagnostic::warning();
    Globals::with(|globals| {
        rs_diagnostic_print_file_line(globals, &mut warning);
        rs_capture_to_diagnostic(globals, Some(Box::new(warning)));
        ptr::from_mut(globals.out.current_diagnostic.as_deref_mut().unwrap())
    })
}

// From C code: This replaces the "print file+line number" block at the start of errors
/// Start the error, print file line, and set the current diagnostic to a new one
pub fn rs_error_here_with_diagnostic(globals: &mut Globals<'_, '_>, message: &[u8]) {
    let mut diag = Diagnostic::error();
    rs_diagnostic_print_file_line(globals, &mut diag);
    diag.append(String::from_utf8_lossy(message));

    if globals.out.file_line_error_style_p != 0 {
        rs_print_file_line(globals)
    } else {
        rs_print_nl_bytes(globals, b"! ")
    }
    rs_print_bytes(globals, message);
    rs_capture_to_diagnostic(globals, Some(Box::new(diag)));
}

#[no_mangle]
pub extern "C" fn error_here_with_diagnostic(msg: *const libc::c_char) -> *mut Diagnostic {
    let str = unsafe { CStr::from_ptr(msg) };
    Globals::with(|globals| {
        rs_error_here_with_diagnostic(globals, str.to_bytes());
        ptr::from_mut(globals.out.current_diagnostic.as_deref_mut().unwrap())
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

pub fn rs_print_ln(globals: &mut Globals<'_, '_>) {
    match globals.engine.selector {
        Selector::File(val) => {
            // TODO: Replace all write!(get_output) with output_write on state
            write!(
                globals
                    .state
                    .get_output(globals.out.write_file[val as usize].unwrap()),
                "\n"
            )
            .unwrap();
        }
        Selector::TermOnly => {
            rs_warn_char(globals.out, '\n');
            write!(
                globals.state.get_output(globals.out.rust_stdout.unwrap()),
                "\n"
            )
            .unwrap();
            globals.out.term_offset = 0;
        }
        Selector::LogOnly => {
            rs_warn_char(globals.out, '\n');
            write!(
                globals.state.get_output(globals.out.log_file.unwrap()),
                "\n"
            )
            .unwrap();
            globals.out.file_offset = 0;
        }
        Selector::TermAndLog => {
            rs_warn_char(globals.out, '\n');
            write!(
                globals.state.get_output(globals.out.rust_stdout.unwrap()),
                "\n"
            )
            .unwrap();
            write!(
                globals.state.get_output(globals.out.log_file.unwrap()),
                "\n"
            )
            .unwrap();
            globals.out.term_offset = 0;
            globals.out.file_offset = 0;
        }
        Selector::NoPrint | Selector::Pseudo | Selector::NewString => {}
    }
}

#[no_mangle]
pub extern "C" fn print_ln() {
    Globals::with(|globals| rs_print_ln(globals))
}

pub fn rs_print_raw_char(globals: &mut Globals<'_, '_>, s: u16, incr_offset: bool) {
    let raw = &[s as u8];
    let c = char::from_u32(s as u32).unwrap_or(char::REPLACEMENT_CHARACTER);
    match globals.engine.selector {
        Selector::TermAndLog => {
            // TODO: This produces a malformed warning currently, since we add unicode byte-by-byte
            rs_warn_char(globals.out, c);
            globals
                .state
                .get_output(globals.out.rust_stdout.unwrap())
                .write(raw)
                .unwrap();
            globals
                .state
                .get_output(globals.out.log_file.unwrap())
                .write(raw)
                .unwrap();
            if incr_offset {
                globals.out.term_offset += 1;
                globals.out.file_offset += 1;
            }
            if globals.out.term_offset as usize == MAX_PRINT_LINE {
                writeln!(globals.state.get_output(globals.out.rust_stdout.unwrap())).unwrap();
                globals.out.term_offset = 0;
            }
            if globals.out.file_offset as usize == MAX_PRINT_LINE {
                writeln!(globals.state.get_output(globals.out.log_file.unwrap())).unwrap();
                globals.out.file_offset = 0;
            }
        }
        Selector::LogOnly => {
            rs_warn_char(globals.out, c);
            globals
                .state
                .get_output(globals.out.log_file.unwrap())
                .write(raw)
                .unwrap();
            if incr_offset {
                globals.out.file_offset += 1;
            }
            if globals.out.file_offset as usize == MAX_PRINT_LINE {
                writeln!(globals.state.get_output(globals.out.log_file.unwrap())).unwrap();
                globals.out.file_offset = 0;
            }
        }
        Selector::TermOnly => {
            rs_warn_char(globals.out, c);
            globals
                .state
                .get_output(globals.out.rust_stdout.unwrap())
                .write(raw)
                .unwrap();
            if incr_offset {
                globals.out.term_offset += 1;
            }
            if globals.out.term_offset as usize == MAX_PRINT_LINE {
                writeln!(globals.state.get_output(globals.out.rust_stdout.unwrap())).unwrap();
                globals.out.term_offset = 0;
            }
        }
        Selector::NoPrint => (),
        Selector::Pseudo => {
            if globals.engine.tally < globals.engine.trick_count {
                globals.engine.trick_buf
                    [(globals.engine.tally % globals.engine.error_line) as usize] = s;
            }
        }
        Selector::NewString => {
            if globals.strings.pool_ptr < globals.strings.pool_size {
                globals.strings.str_pool[globals.strings.pool_ptr] = s;
                globals.strings.pool_ptr += 1;
            }
        }
        Selector::File(val) => {
            globals
                .state
                .get_output(globals.out.write_file[val as usize].unwrap())
                .write(raw)
                .unwrap();
        }
    }
    globals.engine.tally += 1;
}

#[no_mangle]
pub extern "C" fn print_raw_char(s: u16, offset: u8) {
    Globals::with(|globals| rs_print_raw_char(globals, s, offset != 0))
}

pub fn rs_print_char(globals: &mut Globals<'_, '_>, s: i32) {
    if globals.engine.selector == Selector::NewString && !globals.out.doing_special {
        if let Ok(s) = s.try_into() {
            rs_print_raw_char(globals, s, true)
        } else {
            let s = (s - 0x10000) as u16;
            rs_print_raw_char(globals, 0xD800 + s / 1024, true);
            rs_print_raw_char(globals, 0xDC00 + s % 1024, true)
        }
        return;
    }

    if globals.engine.int_par(IntPar::NewLineChar) == s
        && !matches!(
            globals.engine.selector,
            Selector::Pseudo | Selector::NewString
        )
    {
        rs_print_ln(globals);
        return;
    }

    if s < 32 && !globals.out.doing_special {
        rs_print_raw_char(globals, b'^' as u16, true);
        rs_print_raw_char(globals, b'^' as u16, true);
        rs_print_raw_char(globals, (s + 64) as u16, true);
    } else if s < 127 {
        rs_print_raw_char(globals, s as u16, true);
    } else if s == 127 {
        if !globals.out.doing_special {
            rs_print_raw_char(globals, b'^' as u16, true);
            rs_print_raw_char(globals, b'^' as u16, true);
            rs_print_raw_char(globals, b'?' as u16, true);
        } else {
            rs_print_raw_char(globals, s as u16, true);
        }
    } else if s < 160 && !globals.out.doing_special {
        rs_print_raw_char(globals, b'^' as u16, true);
        rs_print_raw_char(globals, b'^' as u16, true);

        let l = (s % 256 / 16) as u16;
        if l < 10 {
            rs_print_raw_char(globals, b'0' as u16 + l, true);
        } else {
            rs_print_raw_char(globals, b'a' as u16 + l - 10, true);
        }

        let l = (s % 16) as u16;
        if l < 10 {
            rs_print_raw_char(globals, b'0' as u16 + l, true);
        } else {
            rs_print_raw_char(globals, b'a' as u16 + l - 10, true);
        }
    } else if globals.engine.selector == Selector::Pseudo {
        rs_print_raw_char(globals, s as u16, true);
    } else {
        // Encode into UTF-8
        if s < 2048 {
            rs_print_raw_char(globals, (192 + s / 64) as u16, false);
            rs_print_raw_char(globals, (128 + s % 64) as u16, true);
        } else if s < 0x10000 {
            rs_print_raw_char(globals, (224 + s / 4096) as u16, false);
            rs_print_raw_char(globals, (128 + s % 4096 / 64) as u16, false);
            rs_print_raw_char(globals, (128 + s % 64) as u16, true);
        } else {
            rs_print_raw_char(globals, (240 + s / 0x40000) as u16, false);
            rs_print_raw_char(globals, (128 + s % 0x40000 / 4096) as u16, false);
            rs_print_raw_char(globals, (128 + s % 4096 / 64) as u16, false);
            rs_print_raw_char(globals, (128 + s % 64) as u16, true);
        }
    }
}

#[no_mangle]
pub extern "C" fn print_char(s: i32) {
    Globals::with(|globals| rs_print_char(globals, s))
}

pub fn rs_print_bytes(globals: &mut Globals<'_, '_>, bytes: &[u8]) {
    for b in bytes {
        rs_print_char(globals, *b as i32)
    }
}

pub fn rs_print_nl_bytes(globals: &mut Globals<'_, '_>, bytes: &[u8]) {
    if (globals.out.term_offset > 0
        && matches!(
            globals.engine.selector,
            Selector::TermOnly | Selector::TermAndLog
        ))
        || (globals.out.file_offset > 0
            && matches!(
                globals.engine.selector,
                Selector::LogOnly | Selector::TermAndLog
            ))
    {
        rs_print_ln(globals);
    }
    rs_print_bytes(globals, bytes);
}

pub fn rs_print_esc_bytes(globals: &mut Globals<'_, '_>, bytes: &[u8]) {
    let c = globals.engine.int_par(IntPar::EscapeChar);
    if c >= 0 && c <= BIGGEST_USV {
        rs_print_char(globals, c);
    }
    rs_print_bytes(globals, bytes);
}

#[no_mangle]
pub extern "C" fn print_cstr(str: *const libc::c_char) {
    let bytes = unsafe { CStr::from_ptr(str) }.to_bytes();
    Globals::with(|globals| rs_print_bytes(globals, bytes))
}

#[no_mangle]
pub extern "C" fn print_nl_cstr(str: *const libc::c_char) {
    let bytes = unsafe { CStr::from_ptr(str) }.to_bytes();
    Globals::with(|globals| rs_print_nl_bytes(globals, bytes))
}

#[no_mangle]
pub extern "C" fn print_esc_cstr(str: *const libc::c_char) {
    let bytes = unsafe { CStr::from_ptr(str) }.to_bytes();
    Globals::with(|globals| rs_print_esc_bytes(globals, bytes))
}

pub fn rs_print(globals: &mut Globals<'_, '_>, str: StrNumber) {
    if str as usize >= globals.strings.str_ptr {
        rs_print_bytes(globals, b"???");
        return;
    } else if str <= BIGGEST_CHAR {
        if str < 0 {
            rs_print_bytes(globals, b"???");
        } else {
            if globals.engine.selector == Selector::NewString {
                rs_print_char(globals, str);
            } else if globals.engine.int_par(IntPar::NewLineChar) == str
                && !matches!(
                    globals.engine.selector,
                    Selector::Pseudo | Selector::NewString
                )
            {
                rs_print_ln(globals);
            } else {
                let nl = globals.engine.int_par(IntPar::NewLineChar);
                globals.engine.set_int_par(IntPar::NewLineChar, -1);
                rs_print_char(globals, str);
                globals.engine.set_int_par(IntPar::NewLineChar, nl);
            }
        }
        return;
    }

    let pool_idx = str - 0x10000;

    let str_len = globals.strings.str(pool_idx).len();
    let mut idx = 0;
    while idx < str_len {
        let str = globals.strings.str(pool_idx);
        let byte = str[idx];
        if (0xD800..0xDC00).contains(&byte)
            && idx + 1 < str_len
            && (0xDC00..0xE000).contains(&str[idx + 1])
        {
            rs_print_char(
                globals,
                0x10000 + (byte as i32 - 0xD800) * 1024 + (str[idx + 1] as i32 - 0xDC00),
            );
            idx += 1;
        } else {
            rs_print_char(globals, byte as i32);
        }
        idx += 1;
    }
}

pub fn rs_print_nl(globals: &mut Globals<'_, '_>, str: StrNumber) {
    if (globals.out.term_offset > 0
        && matches!(
            globals.engine.selector,
            Selector::TermOnly | Selector::TermAndLog
        ))
        || (globals.out.file_offset > 0
            && matches!(
                globals.engine.selector,
                Selector::LogOnly | Selector::TermAndLog
            ))
    {
        rs_print_ln(globals);
    }
    rs_print(globals, str);
}

pub fn rs_print_esc(globals: &mut Globals<'_, '_>, str: StrNumber) {
    let c = globals.engine.int_par(IntPar::EscapeChar);
    if c >= 0 && c <= BIGGEST_USV {
        rs_print_char(globals, c);
    }
    rs_print(globals, str);
}

#[no_mangle]
pub extern "C" fn print(str: StrNumber) {
    Globals::with(|globals| rs_print(globals, str))
}

#[no_mangle]
pub extern "C" fn print_nl(str: StrNumber) {
    Globals::with(|globals| rs_print_nl(globals, str))
}

#[no_mangle]
pub extern "C" fn print_esc(str: StrNumber) {
    Globals::with(|globals| rs_print_esc(globals, str))
}

pub fn rs_print_the_digs(globals: &mut Globals<'_, '_>, k: usize) {
    for k in (0..k).rev() {
        if globals.out.digits[k] < 10 {
            rs_print_char(globals, (b'0' + globals.out.digits[k]) as i32)
        } else {
            rs_print_char(globals, (55 + globals.out.digits[k]) as i32)
        }
    }
}

pub fn rs_print_int(globals: &mut Globals<'_, '_>, mut n: i32) {
    let mut k = 0;

    if n < 0 {
        rs_print_char(globals, b'-' as i32);
        if n > -100000000 {
            n = -n;
        } else {
            let mut m = -1 - n;
            n = m / 10;
            m = (m % 10) + 1;
            k = 1;
            if m < 10 {
                globals.out.digits[0] = m as u8;
            } else {
                globals.out.digits[0] = 0;
                n += 1;
            }
        }
    }

    loop {
        globals.out.digits[k] = (n % 10) as u8;
        n /= 10;
        k += 1;
        if n == 0 {
            break;
        }
    }

    rs_print_the_digs(globals, k);
}

pub fn rs_print_file_line(globals: &mut Globals<'_, '_>) {
    let mut level = globals.files.in_open as usize;
    while level > 0 && globals.files.full_source_filename_stack[level] == 0 {
        level -= 1;
    }

    if level == 0 {
        rs_print_nl_bytes(globals, b"! ")
    } else {
        rs_print_nl_bytes(globals, b"");
        rs_print(globals, globals.files.full_source_filename_stack[level]);
        rs_print(globals, ':' as i32);
        if level == globals.files.in_open as usize {
            rs_print_int(globals, globals.files.line);
        } else {
            rs_print_int(globals, globals.files.line_stack[level + 1])
        }
        rs_print_bytes(globals, b": ");
    }
}

#[no_mangle]
pub extern "C" fn print_the_digs(k: u8) {
    Globals::with(|globals| rs_print_the_digs(globals, k as usize))
}

#[no_mangle]
pub extern "C" fn print_int(n: i32) {
    Globals::with(|globals| rs_print_int(globals, n))
}

#[no_mangle]
pub extern "C" fn print_file_line() {
    Globals::with(|globals| rs_print_file_line(globals))
}

pub fn rs_print_cs(globals: &mut Globals<'_, '_>, p: i32) {
    let p = p as usize;
    if p < HASH_BASE {
        if p >= SINGLE_BASE {
            if p == NULL_CS {
                rs_print_esc_bytes(globals, b"csname");
                rs_print_esc_bytes(globals, b"endcsname");
                rs_print_char(globals, b' ' as i32);
            } else {
                rs_print_esc(globals, (p - SINGLE_BASE) as i32);
                if globals.engine.cat_code(p - SINGLE_BASE) == Ok(CatCode::Letter) {
                    rs_print_char(globals, b' ' as i32);
                }
            }
        } else if p < ACTIVE_BASE {
            rs_print_esc_bytes(globals, b"IMPOSSIBLE.");
        } else {
            rs_print_char(globals, (p - 1) as i32);
        }
    } else if (p >= UNDEFINED_CONTROL_SEQUENCE && p <= EQTB_SIZE)
        || (p > globals.engine.eqtb_top as usize)
    {
        rs_print_esc_bytes(globals, b"IMPOSSIBLE.");
    } else if globals.hash.hash(p).s1 as usize >= globals.strings.str_ptr {
        rs_print_esc_bytes(globals, b"NONEXISTENT.");
    } else {
        if p >= PRIM_EQTB_BASE && p < FROZEN_NULL_FONT {
            rs_print_esc(globals, globals.engine.prim[p - PRIM_EQTB_BASE].s1 - 1);
        } else {
            rs_print_esc(globals, globals.hash.hash(p).s1);
        }
        rs_print_char(globals, b' ' as i32);
    }
}

pub fn rs_sprint_cs(globals: &mut Globals<'_, '_>, p: i32) {
    let p = p as usize;
    if p < HASH_BASE {
        if p < SINGLE_BASE {
            rs_print_char(globals, (p - 1) as i32);
        } else if p < NULL_CS {
            rs_print_esc(globals, (p - SINGLE_BASE) as i32);
        } else {
            rs_print_esc_bytes(globals, b"csname");
            rs_print_esc_bytes(globals, b"endcsname");
        }
    } else if p >= PRIM_EQTB_BASE && p < FROZEN_NULL_FONT {
        rs_print_esc(globals, globals.engine.prim[p - PRIM_EQTB_BASE].s1 - 1);
    } else {
        rs_print_esc(globals, globals.hash.hash(p).s1);
    }
}

#[no_mangle]
pub extern "C" fn print_cs(p: i32) {
    Globals::with(|globals| rs_print_cs(globals, p))
}

#[no_mangle]
pub extern "C" fn sprint_cs(p: i32) {
    Globals::with(|globals| rs_sprint_cs(globals, p))
}

pub fn rs_print_file_name(globals: &mut Globals<'_, '_>, n: i32, a: i32, e: i32) {
    let mut quote = None;

    for s in [a, n, e] {
        if s == 0 || quote.is_some() {
            continue;
        }
        let str = globals.strings.str(s - 0x10000);
        quote = str
            .iter()
            .find(|&&c| c == ' ' as u16 || c == '"' as u16 || c == '\'' as u16)
            .copied();
    }

    if quote == Some(' ' as u16) {
        quote = Some('"' as u16);
    } else if let Some(q) = quote {
        quote = Some(73 - q);
    }

    if let Some(q) = quote {
        rs_print_char(globals, q as i32);
    }

    for s in [a, n, e] {
        if s == 0 {
            continue;
        }
        // TODO: Fix up borrowing so we can use `strings.str`
        let str = globals.strings.str_range(s - 0x10000);
        for idx in str {
            let c = globals.strings.char_at(idx);
            if let Some(qc) = quote {
                if c == qc {
                    rs_print(globals, qc as i32);
                    rs_print(globals, (73 - qc) as i32);
                    quote = Some(73 - qc);
                }
            }
            rs_print(globals, c as i32);
        }
    }

    if let Some(q) = quote {
        rs_print_char(globals, q as i32);
    }
}

#[no_mangle]
pub extern "C" fn print_file_name(n: i32, a: i32, e: i32) {
    Globals::with(|globals| rs_print_file_name(globals, n, a, e))
}

pub fn rs_print_size(globals: &mut Globals<'_, '_>, s: i32) {
    let s = s as usize;
    if s == TEXT_SIZE {
        rs_print_esc_bytes(globals, b"textfont");
    } else if s == SCRIPT_SIZE {
        rs_print_esc_bytes(globals, b"scriptfont");
    } else {
        rs_print_esc_bytes(globals, b"scriptscriptfont");
    }
}

#[no_mangle]
pub extern "C" fn print_size(s: i32) {
    Globals::with(|globals| rs_print_size(globals, s))
}

pub fn rs_print_write_whatsit(globals: &mut Globals<'_, '_>, s: &[u8], p: i32) {
    rs_print_esc_bytes(globals, s);
    let p = p as usize;

    let val = globals.engine.mem[p + 1].i32_0();
    if val < 16 {
        rs_print_int(globals, val)
    } else if val == 16 {
        rs_print_char(globals, '*' as i32);
    } else {
        rs_print_char(globals, '-' as i32);
    }
}

#[no_mangle]
pub extern "C" fn print_write_whatsit(s: *const libc::c_char, p: i32) {
    let s = unsafe { CStr::from_ptr(s) }.to_bytes();
    Globals::with(|globals| rs_print_write_whatsit(globals, s, p))
}

pub fn rs_print_native_word(globals: &mut Globals<'_, '_>, p: i32) {
    let p = p as usize;
    let size = globals.engine.node::<NativeWordNode>(p).len();
    let mut skip = false;
    for i in 0..size {
        if skip {
            skip = false;
            continue;
        }

        let node = globals.engine.node::<NativeWordNode>(p);
        let c = node.text()[i];
        if c >= 0xD800 && c < 0xDC00 {
            if i < size - 1 {
                let cc = node.text()[i + 1];
                if cc >= 0xDC00 && cc < 0xE000 {
                    let c = 0x10000 + (c as i32 - 0xD800) * 1024 + (cc as i32 - 0xDC00);
                    rs_print_char(globals, c);
                    skip = true;
                } else {
                    rs_print(globals, '.' as i32);
                }
            } else {
                rs_print(globals, '.' as i32);
            }
        } else {
            rs_print_char(globals, c as i32);
        }
    }
}

#[no_mangle]
pub extern "C" fn print_native_word(p: i32) {
    Globals::with(|globals| rs_print_native_word(globals, p))
}
