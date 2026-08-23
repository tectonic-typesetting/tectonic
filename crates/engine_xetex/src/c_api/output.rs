use crate::c_api::engine::*;
use crate::c_api::font::{AAT_FONT_FLAG, OTGR_FONT_FLAG};
use crate::c_api::globals::{Globals, ALL_CTX};
use crate::c_api::hash::HASH_BASE;
use crate::c_api::pool::rs_str_length;
use crate::ty::{Scaled, StrNumber};
use std::ffi::CStr;
use std::io::Write;
use std::ptr;
use std::ptr::NonNull;
use tectonic_bridge_core::{Diagnostic, OutputId};

pub const MAX_PRINT_LINE: usize = 79;

/* characters
 *
 * TeX thinks there are only 256 character but we know better. We use UTF16
 * codepoints. Actual Unicode character codes can exceed this, up to
 * BIGGEST_USV. "USV" here means Unicode Scalar Value. */

pub const BIGGEST_CHAR: i32 = 0xFFFF;

pub const BIGGEST_USV: i32 = 0x10FFFF;

pub const NUMBER_USVS: i32 = BIGGEST_USV + 1;

pub struct OutputCtx {
    pub(crate) current_diagnostic: Option<Box<Diagnostic>>,
    file_line_error_style_p: i32,
    term_offset: i32,
    file_offset: i32,
    pub(crate) rust_stdout: Option<OutputId>,
    pub(crate) log_file: Option<OutputId>,
    pub(crate) write_open: Vec<bool>,
    pub(crate) write_file: Vec<Option<OutputId>>,
    doing_special: bool,
    /// digits
    dig: [u8; 23],
}

impl OutputCtx {
    pub(crate) const fn new() -> OutputCtx {
        OutputCtx {
            current_diagnostic: None,
            file_line_error_style_p: 0,
            term_offset: 0,
            file_offset: 0,
            rust_stdout: None,
            log_file: None,
            write_open: Vec::new(),
            write_file: Vec::new(),
            doing_special: false,
            dig: [0; 23],
        }
    }

    pub fn with<T>(f: impl FnOnce(&mut OutputCtx) -> T) -> T {
        Globals::token(|tok| ALL_CTX.with(|(_, _, _, _, out, _, _, _, _)| f(out.borrow_mut(tok))))
    }
}

c_var!(OutputCtx => file_line_error_style_p: i32);

#[no_mangle]
pub extern "C" fn current_diagnostic() -> *mut Diagnostic {
    OutputCtx::with(|out| {
        out.current_diagnostic
            .as_mut()
            .map(|b| ptr::from_mut(&mut **b))
            .unwrap_or(ptr::null_mut())
    })
}

c_var!(OutputCtx => term_offset: i32);
c_var!(OutputCtx => file_offset: i32);
c_var!(OutputCtx => rust_stdout: Option<OutputId>);
c_var!(OutputCtx => log_file: Option<OutputId>);

#[no_mangle]
pub extern "C" fn write_open(idx: usize) -> bool {
    OutputCtx::with(|out| out.write_open[idx])
}

#[no_mangle]
pub extern "C" fn set_write_open(idx: usize, val: bool) {
    OutputCtx::with(|out| {
        if out.write_open.len() < idx + 1 {
            out.write_open.resize(idx + 1, false);
        }
        out.write_open[idx] = val;
    })
}

#[no_mangle]
pub extern "C" fn write_file(idx: usize) -> Option<OutputId> {
    OutputCtx::with(|out| out.write_file[idx])
}

#[no_mangle]
pub extern "C" fn set_write_file(idx: usize, val: Option<OutputId>) {
    OutputCtx::with(|out| {
        if out.write_file.len() < idx + 1 {
            out.write_file.resize(idx + 1, None);
        }
        out.write_file[idx] = val;
    })
}

c_var!(OutputCtx => doing_special: bool);
c_arr!(OutputCtx => dig[_]: u8);

pub fn rs_capture_to_diagnostic(
    globals: &mut Globals<'_, '_>,
    diagnostic: Option<Box<Diagnostic>>,
) {
    if let Some(diag) = globals.out.current_diagnostic.take() {
        globals.state.finish_diagnostic(*diag);
    }
    globals.out.current_diagnostic = diagnostic;
}

/// A lower-level API to begin or end the capture of messages into the diagnostic
/// buffer. You can start capture by obtaining a diagnostic_t and passing it to
/// this function -- however, the other functions in this API generally do this
/// for you. Complete capture by passing NULL. Either way, if a capture is in
/// progress when this function is called, it will be completed and reported.
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

pub fn rs_diagnostic_begin_capture_warning_here(globals: &mut Globals) -> *mut Diagnostic {
    let mut warning = Diagnostic::warning();
    rs_diagnostic_print_file_line(globals, &mut warning);
    rs_capture_to_diagnostic(globals, Some(Box::new(warning)));
    ptr::from_mut(globals.out.current_diagnostic.as_deref_mut().unwrap())
}

/// Duplicate messages printed to log/terminal into a warning diagnostic buffer,
/// until a call capture_to_diagnostic(0). A standard usage of this is
/// ```c
/// ttbc_diagnostic_t *warning = diagnostic_begin_capture_warning_here();
///
/// // ... XeTeX prints some errors using print_* functions ...
///
/// capture_to_diagnostic(NULL);
/// ```
///
/// The current file and line number information are prefixed to the captured
/// output.
///
/// NOTE: the only reason there isn't also an _error_ version of this function is
/// that we haven't yet wired up anything that uses it.
#[no_mangle]
pub extern "C" fn diagnostic_begin_capture_warning_here() -> *mut Diagnostic {
    Globals::with(rs_diagnostic_begin_capture_warning_here)
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

/// A replacement for xetex print_file_line+print_nl_ctr blocks. e.g. Replace
///
/// ```c
/// if (file_line_error_style_p)
///     print_file_line();
/// else
///     print_nl_cstr("! ");
/// print_cstr("Cannot use ");
/// ```
/// with
/// ```c
/// ttbc_diagnostic_t *errmsg = error_here_with_diagnostic("Cannot use ");
/// ```
///
/// This function calls `capture_to_diagnostic(errmsg)` to begin diagnostic
/// capture. You must call `capture_to_diagnostic(NULL)` to mark the capture as
/// complete.
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
    OutputCtx::with(|out| {
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
            writeln!(globals
                .state
                .get_output(globals.out.write_file[val as usize].unwrap()))
            .unwrap();
        }
        Selector::TermOnly => {
            rs_warn_char(globals.out, '\n');
            writeln!(globals.state.get_output(globals.out.rust_stdout.unwrap())).unwrap();
            globals.out.term_offset = 0;
        }
        Selector::LogOnly => {
            rs_warn_char(globals.out, '\n');
            writeln!(globals.state.get_output(globals.out.log_file.unwrap())).unwrap();
            globals.out.file_offset = 0;
        }
        Selector::TermAndLog => {
            rs_warn_char(globals.out, '\n');
            writeln!(globals.state.get_output(globals.out.rust_stdout.unwrap())).unwrap();
            writeln!(globals.state.get_output(globals.out.log_file.unwrap())).unwrap();
            globals.out.term_offset = 0;
            globals.out.file_offset = 0;
        }
        Selector::NoPrint | Selector::Pseudo | Selector::NewString => {}
    }
}

#[no_mangle]
pub extern "C" fn print_ln() {
    Globals::with(rs_print_ln)
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
                .write_all(raw)
                .unwrap();
            globals
                .state
                .get_output(globals.out.log_file.unwrap())
                .write_all(raw)
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
                .write_all(raw)
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
                .write_all(raw)
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
                .write_all(raw)
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
    if (0..=BIGGEST_USV).contains(&c) {
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
    if (0..=BIGGEST_USV).contains(&c) {
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
        if globals.out.dig[k] < 10 {
            rs_print_char(globals, (b'0' + globals.out.dig[k]) as i32)
        } else {
            rs_print_char(globals, (55 + globals.out.dig[k]) as i32)
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
                globals.out.dig[0] = m as u8;
            } else {
                globals.out.dig[0] = 0;
                n += 1;
            }
        }
    }

    loop {
        globals.out.dig[k] = (n % 10) as u8;
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
    Globals::with(rs_print_file_line)
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
    } else if (UNDEFINED_CONTROL_SEQUENCE..=EQTB_SIZE).contains(&p)
        || (p > globals.engine.eqtb_top as usize)
    {
        rs_print_esc_bytes(globals, b"IMPOSSIBLE.");
    } else if globals.hash.hash(p).s1 as usize >= globals.strings.str_ptr {
        rs_print_esc_bytes(globals, b"NONEXISTENT.");
    } else {
        if (PRIM_EQTB_BASE..FROZEN_NULL_FONT).contains(&p) {
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
    } else if (PRIM_EQTB_BASE..FROZEN_NULL_FONT).contains(&p) {
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
        if (0xD800..0xDC00).contains(&c) {
            if i < size - 1 {
                let cc = node.text()[i + 1];
                if (0xDC00..0xE000).contains(&cc) {
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

pub fn rs_print_sa_num(globals: &mut Globals<'_, '_>, q: i32) {
    let q = q as usize;
    // TODO: Convert to symbolic access
    let word = globals.engine.raw_mem(q);
    let n = if (word.u16_1() as usize) < DIMEN_VAL_LIMIT {
        globals.engine.raw_mem(q + 1).i32_1()
    } else {
        let next = globals.engine.base_node(q).next();
        let next2 = globals.engine.base_node(next).next();
        let next3 = globals.engine.base_node(next2).next();

        let word2 = globals.engine.raw_mem(next);
        let word3 = globals.engine.raw_mem(next2);
        let word4 = globals.engine.raw_mem(next3);

        word.u16_1() as i32 % 64
            + (64 * word2.u16_1() as i32)
            + (64 * 64 * (word3.u16_1() as i32 + 64 * word4.u16_1() as i32))
    };

    rs_print_int(globals, n);
}

#[no_mangle]
pub extern "C" fn print_sa_num(q: i32) {
    Globals::with(|globals| rs_print_sa_num(globals, q))
}

pub fn rs_print_two(globals: &mut Globals<'_, '_>, n: i32) {
    let n = (n.abs() % 100) as u8;
    rs_print_char(globals, (b'0' + n / 10) as i32);
    rs_print_char(globals, (b'0' + n % 10) as i32);
}

pub fn rs_print_hex(globals: &mut Globals<'_, '_>, mut n: i32) {
    let mut k = 0;

    rs_print_char(globals, '"' as i32);
    loop {
        globals.out.dig[k] = (n % 16) as u8;
        n /= 16;
        k += 1;
        if n == 0 {
            break;
        }
    }

    rs_print_the_digs(globals, k);
}

pub fn rs_print_scaled(globals: &mut Globals<'_, '_>, mut s: Scaled) {
    let mut delta;

    if s < 0 {
        rs_print_char(globals, '-' as i32);
        s = -s;
    }

    rs_print_int(globals, s / 0x10000);
    rs_print_char(globals, '.' as i32);
    s = 10 * (s % 0x10000) + 5;
    delta = 10;
    loop {
        if delta > 0x10000 {
            s += 0x8000 - 50000;
        }
        rs_print_char(globals, '0' as i32 + (s / 0x10000));
        s = 10 * (s % 0x10000);
        delta *= 10;

        if s <= delta {
            break;
        }
    }
}

pub fn rs_print_ucs_code(globals: &mut Globals<'_, '_>, c: char) {
    rs_print_bytes(globals, b"U+");

    let mut k = 0;
    let mut n = c as u32;
    while n > 0 {
        globals.out.dig[k] = (n % 16) as u8;
        n /= 16;
        k += 1;
    }

    while k < 4 {
        globals.out.dig[k] = 0;
        k += 1;
    }

    rs_print_the_digs(globals, k);
}

#[no_mangle]
pub extern "C" fn print_two(n: i32) {
    Globals::with(|globals| rs_print_two(globals, n))
}

#[no_mangle]
pub extern "C" fn print_hex(n: i32) {
    Globals::with(|globals| rs_print_hex(globals, n))
}

#[no_mangle]
pub extern "C" fn print_scaled(s: Scaled) {
    Globals::with(|globals| rs_print_scaled(globals, s))
}

#[no_mangle]
pub extern "C" fn print_ucs_code(n: u32) {
    Globals::with(|globals| {
        rs_print_ucs_code(
            globals,
            char::from_u32(n).unwrap_or(char::REPLACEMENT_CHARACTER),
        )
    })
}

pub fn rs_print_current_string(globals: &mut Globals<'_, '_>) {
    let start = globals.strings.str_start[globals.strings.str_ptr - 0x10000] as usize;
    let end = globals.strings.pool_ptr;
    for j in start..end {
        rs_print_char(globals, globals.strings.str_pool[j] as i32);
    }
}

#[no_mangle]
pub extern "C" fn print_current_string() {
    Globals::with(rs_print_current_string)
}

pub fn rs_print_roman_int(globals: &mut Globals<'_, '_>, mut n: i32) {
    const ROMAN_DATA: &[u8] = b"m2d5c2l5x2v5i";

    let mut j = 0;
    let mut v = 1000;

    loop {
        while n >= v {
            rs_print_char(globals, ROMAN_DATA[j] as i32);
            n -= v;
        }

        if n <= 0 {
            return;
        }

        let mut k = j + 2;
        let mut u = v / (ROMAN_DATA[k - 1] - b'0') as i32;
        if ROMAN_DATA[k - 1] == b'2' {
            k += 2;
            u /= (ROMAN_DATA[k - 1] - b'0') as i32;
        }

        if n + u >= v {
            rs_print_char(globals, ROMAN_DATA[k] as i32);
            n += u;
        } else {
            j += 2;
            v /= (ROMAN_DATA[j - 1] - b'0') as i32;
        }
    }
}

#[no_mangle]
pub extern "C" fn print_roman_int(n: i32) {
    Globals::with(|globals| rs_print_roman_int(globals, n))
}

pub fn rs_print_skip_param(globals: &mut Globals<'_, '_>, n: i32) {
    let Ok(n) = GluePar::try_from(n) else {
        rs_print_bytes(globals, b"[unknown glue parameter!]");
        return;
    };

    let s = match n {
        GluePar::LineSkip => b"lineskip" as &[_],
        GluePar::BaselineSkip => b"baselineskip",
        GluePar::ParSkip => b"parskip",
        GluePar::AboveDisplaySkip => b"abovedisplayskip",
        GluePar::BelowDisplaySkip => b"belowdisplayskip",
        GluePar::AboveDisplayShortSkip => b"abovedisplayshortskip",
        GluePar::BelowDisplayShortSkip => b"belowdisplayshortskip",
        GluePar::LeftSkip => b"leftskip",
        GluePar::RightSkip => b"rightskip",
        GluePar::TopSkip => b"topskip",
        GluePar::SplitTopSkip => b"splittopskip",
        GluePar::TabSkip => b"tabskip",
        GluePar::SpaceSkip => b"spaceskip",
        GluePar::XSpaceSkip => b"xspaceskip",
        GluePar::ParFillSkip => b"parfillskip",
        GluePar::XetexLinebreakSkip => b"XeTeXlinebreakskip",
        GluePar::ThinMuSkip => b"thinmuskip",
        GluePar::MedMuSkip => b"medmuskip",
        GluePar::ThickMuSkip => b"thickmuskip",
    };
    rs_print_esc_bytes(globals, s);
}

#[no_mangle]
pub extern "C" fn print_skip_param(n: i32) {
    Globals::with(|globals| rs_print_skip_param(globals, n))
}

pub fn print_param(globals: &mut Globals<'_, '_>, n: i32) {
    let Ok(n) = IntPar::try_from(n) else {
        rs_print_bytes(globals, b"[unknown int32_t parameter!]");
        return;
    };

    let s = match n {
        IntPar::PreTolerance => b"pretolerance" as &[_],
        IntPar::Tolerance => b"tolerance",
        IntPar::LinePenalty => b"linepenalty",
        IntPar::HyphenPenalty => b"hyphenpenalty",
        IntPar::ExHyphenPenalty => b"exhyphenpenalty",
        IntPar::ClubPenalty => b"clubpenalty",
        IntPar::WidowPenalty => b"widowpenalty",
        IntPar::DisplayWidowPenalty => b"displaywidowpenalty",
        IntPar::BrokenPenalty => b"brokenpenalty",
        IntPar::BinOpPenalty => b"binoppenalty",
        IntPar::RelPenalty => b"relpenalty",
        IntPar::PreDisplayPenalty => b"predisplaypenalty",
        IntPar::PostDisplayPenalty => b"postdisplaypenalty",
        IntPar::InterLinePenalty => b"interlinepenalty",
        IntPar::DoubleHyphenDemerits => b"doublehyphendemerits",
        IntPar::FinalHyphenDemerits => b"finalhyphendemerits",
        IntPar::AdjDemerits => b"adjdemerits",
        IntPar::Mag => b"mag",
        IntPar::DelimiterFactor => b"delimiterfactor",
        IntPar::Looseness => b"looseness",
        IntPar::Time => b"time",
        IntPar::Day => b"day",
        IntPar::Month => b"month",
        IntPar::Year => b"year",
        IntPar::ShowBoxBreadth => b"showboxbreadth",
        IntPar::ShowBoxDepth => b"showboxdepth",
        IntPar::HBadness => b"hbadness",
        IntPar::VBadness => b"vbadness",
        IntPar::Pausing => b"pausing",
        IntPar::TracingOnline => b"tracingonline",
        IntPar::TracingMacros => b"tracingmacros",
        IntPar::TracingStats => b"tracingstats",
        IntPar::TracingParagraphs => b"tracingparagraphs",
        IntPar::TracingPages => b"tracingpages",
        IntPar::TracingOutput => b"tracingoutput",
        IntPar::TracingLostChars => b"tracinglostchars",
        IntPar::TracingCommands => b"tracingcommands",
        IntPar::TracingRestores => b"tracingrestores",
        IntPar::UcHyph => b"uchyph",
        IntPar::OutputPenalty => b"outputpenalty",
        IntPar::MaxDeadCycles => b"maxdeadcycles",
        IntPar::HangAfter => b"hangafter",
        IntPar::FloatingPenalty => b"floatingpenalty",
        IntPar::GlobalDefs => b"globaldefs",
        IntPar::CurFam => b"fam",
        IntPar::EscapeChar => b"escapechar",
        IntPar::DefaultHyphenChar => b"defaulthyphenchar",
        IntPar::DefaultSkewChar => b"defaultskewchar",
        IntPar::EndLineChar => b"endlinechar",
        IntPar::NewLineChar => b"newlinechar",
        IntPar::Language => b"language",
        IntPar::LeftHyphenMin => b"lefthyphenmin",
        IntPar::RightHyphenMin => b"righthyphenmin",
        IntPar::HoldingInserts => b"holdinginserts",
        IntPar::ErrorContextLines => b"errorcontextlines",
        /* Tectonic: MLTeX char_sub* params permanently removed */
        IntPar::TracingStackLevels => b"tracingstacklevels",
        IntPar::TracingAssigns => b"tracingassigns",
        IntPar::TracingGroups => b"tracinggroups",
        IntPar::TracingIfs => b"tracingifs",
        IntPar::TracingScanTokens => b"tracingscantokens",
        IntPar::TracingNesting => b"tracingnesting",
        IntPar::PreDisplayDirection => b"predisplaydirection",
        IntPar::LastLineFit => b"lastlinefit",
        IntPar::SavingVDiscards => b"savingvdiscards",
        IntPar::SavingHyphCodes => b"savinghyphcodes",
        IntPar::SuppressFontNotFoundError => b"suppressfontnotfounderror",
        IntPar::XetexLinebreakLocale => b"XeTeXlinebreaklocale",
        IntPar::XetexLinebreakPenalty => b"XeTeXlinebreakpenalty",
        IntPar::XetexProtrudeChars => b"XeTeXprotrudechars",
        IntPar::Texxet => b"TeXXeTstate",
        IntPar::XetexDashBreak => b"XeTeXdashbreakstate",
        IntPar::XetexUpwards => b"XeTeXupwardsmode",
        IntPar::XetexUseGlyphMetrics => b"XeTeXuseglyphmetrics",
        IntPar::XetexInterCharTokens => b"XeTeXinterchartokenstate",
        IntPar::XetexInputNormalization => b"XeTeXinputnormalization",
        IntPar::XetexDefaultInputMode => b"XeTeXdefaultinputmode",
        IntPar::XetexDefaultInputEncoding => b"XeTeXdefaultinputencoding",
        IntPar::XetexTracingFonts => b"XeTeXtracingfonts",
        IntPar::XetexInterwordSpaceShaping => b"XeTeXinterwordspaceshaping",
        IntPar::XetexGenerateActualText => b"XeTeXgenerateactualtext",
        IntPar::XetexHyphenatableLength => b"XeTeXhyphenatablelength",
        IntPar::Synctex => b"synctex",
        IntPar::PdfOutput => b"pdfoutput",
    };

    rs_print_esc_bytes(globals, s);
}

pub fn print_length_param(globals: &mut Globals<'_, '_>, n: i32) {
    let Ok(d) = DimenPar::try_from(n) else {
        rs_print_bytes(globals, b"[unknown dimen parameter!]");
        return;
    };
    let s = match d {
        DimenPar::ParIndent => b"parindent" as &[_],
        DimenPar::MathSurround => b"mathsurround",
        DimenPar::LineSkipLimit => b"lineskiplimit",
        DimenPar::HSize => b"hsize",
        DimenPar::VSize => b"vsize",
        DimenPar::MaxDepth => b"maxdepth",
        DimenPar::SplitMaxDepth => b"splitmaxdepth",
        DimenPar::BoxMaxDepth => b"boxmaxdepth",
        DimenPar::HFuzz => b"hfuzz",
        DimenPar::VFuzz => b"vfuzz",
        DimenPar::DelimiterShortfall => b"delimitershortfall",
        DimenPar::NullDelimiterSpace => b"nulldelimiterspace",
        DimenPar::ScriptSpace => b"scriptspace",
        DimenPar::PreDisplaySpace => b"predisplayspace",
        DimenPar::DisplayWidth => b"displaywidth",
        DimenPar::DisplayIndent => b"displayindent",
        DimenPar::OverfullRule => b"overfullrule",
        DimenPar::HangIndent => b"hangindent",
        DimenPar::HOffset => b"hoffset",
        DimenPar::VOffset => b"voffset",
        DimenPar::EmergencyStretch => b"emergencystretch",
        DimenPar::PdfPageWidth => b"pdfpagewidth",
        DimenPar::PdfPageHeight => b"pdfpageheight",
    };
    rs_print_esc_bytes(globals, s);
}

pub fn rs_print_style(globals: &mut Globals<'_, '_>, c: i32) {
    let s: &[_] = match (c / 2) * 2 {
        DISPLAY_STYLE => b"displaystyle",
        TEXT_STYLE => b"textstyle",
        SCRIPT_STYLE => b"scriptstyle",
        SCRIPT_SCRIPT_STYLE => b"scriptscriptstyle",
        _ => {
            rs_print_bytes(globals, b"Unknown style!");
            return;
        }
    };
    rs_print_esc_bytes(globals, s);
}

#[no_mangle]
pub extern "C" fn print_style(c: i32) {
    Globals::with(|globals| rs_print_style(globals, c))
}

pub fn rs_print_cmd_chr(globals: &mut Globals<'_, '_>, cmd: u16, chr_code: i32) {
    let print = |globals, chr_code| {
        if chr_code < 65536 {
            rs_print(globals, chr_code)
        } else {
            rs_print_char(globals, chr_code)
        }
    };
    match cmd as i32 {
        LEFT_BRACE => {
            rs_print_bytes(globals, b"begin-group character ");
            print(globals, chr_code);
        }
        RIGHT_BRACE => {
            rs_print_bytes(globals, b"end-group character ");
            print(globals, chr_code);
        }
        MATH_SHIFT => {
            rs_print_bytes(globals, b"math shift character ");
            print(globals, chr_code);
        }
        MAC_PARAM => {
            rs_print_bytes(globals, b"macro parameter character ");
            print(globals, chr_code);
        }
        SUP_MARK => {
            rs_print_bytes(globals, b"superscript character ");
            print(globals, chr_code);
        }
        SUB_MARK => {
            rs_print_bytes(globals, b"subscript character ");
            print(globals, chr_code);
        }
        ENDV => rs_print_bytes(globals, b"end of alignment template"),
        SPACER => {
            rs_print_bytes(globals, b"blank space ");
            print(globals, chr_code);
        }
        LETTER => {
            rs_print_bytes(globals, b"the letter ");
            print(globals, chr_code);
        }
        OTHER_CHAR => {
            rs_print_bytes(globals, b"the character ");
            print(globals, chr_code);
        }
        ASSIGN_GLUE | ASSIGN_MU_GLUE => {
            if chr_code < SKIP_BASE as i32 {
                rs_print_skip_param(globals, chr_code - GLUE_BASE as i32);
            } else if chr_code < MU_SKIP_BASE as i32 {
                rs_print_esc_bytes(globals, b"skip");
                rs_print_int(globals, chr_code - SKIP_BASE as i32);
            } else {
                rs_print_esc_bytes(globals, b"muskip");
                rs_print_int(globals, chr_code - MU_SKIP_BASE as i32);
            }
        }
        ASSIGN_TOKS => {
            if chr_code >= TOKS_BASE as i32 {
                rs_print_esc_bytes(globals, b"toks");
                rs_print_int(globals, chr_code - TOKS_BASE as i32);
            } else {
                let Ok(l) = Local::try_from(chr_code - LOCAL_BASE as i32) else {
                    rs_print_esc_bytes(globals, b"errhelp");
                    return;
                };
                let s = match l {
                    Local::ParShape => b"parshape" as &[_],
                    Local::OutputRoutine => b"output",
                    Local::EveryPar => b"everypar",
                    Local::EveryMath => b"everymath",
                    Local::EveryDisplay => b"everydisplay",
                    Local::EveryHbox => b"everyhbox",
                    Local::EveryVbox => b"everyvbox",
                    Local::EveryJob => b"everyjob",
                    Local::EveryCr => b"everycr",
                    Local::ErrHelp => b"errhelp",
                    Local::EveryEof => b"everyeof",
                    Local::XetexInterCharToks => b"XeTeXinterchartokens",
                    Local::TectonicCodaTokens => b"TectonicCodaTokens",
                };
                rs_print_esc_bytes(globals, s);
            }
        }
        ASSIGN_INT => {
            if chr_code < COUNT_BASE as i32 {
                print_param(globals, chr_code - INT_BASE as i32);
            } else {
                rs_print_esc_bytes(globals, b"count");
                rs_print_int(globals, chr_code - COUNT_BASE as i32);
            }
        }
        ASSIGN_DIMEN => {
            if chr_code < SCALED_BASE as i32 {
                print_length_param(globals, chr_code - DIMEN_BASE as i32);
            } else {
                rs_print_esc_bytes(globals, b"dimen");
                rs_print_int(globals, chr_code - SCALED_BASE as i32);
            }
        }
        ACCENT => rs_print_esc_bytes(globals, b"accent"),
        ADVANCE => rs_print_esc_bytes(globals, b"advance"),
        AFTER_ASSIGNMENT => rs_print_esc_bytes(globals, b"afterassignment"),
        AFTER_GROUP => rs_print_esc_bytes(globals, b"aftergroup"),
        ASSIGN_FONT_DIMEN => rs_print_esc_bytes(globals, b"fontdimen"),
        BEGIN_GROUP => rs_print_esc_bytes(globals, b"begingroup"),
        BREAK_PENALTY => rs_print_esc_bytes(globals, b"penalty"),
        CHAR_NUM => rs_print_esc_bytes(globals, b"char"),
        CS_NAME => rs_print_esc_bytes(globals, b"csname"),
        DEF_FONT => rs_print_esc_bytes(globals, b"font"),
        DELIM_NUM => {
            if chr_code == 1 {
                rs_print_esc_bytes(globals, b"Udelimiter");
            } else {
                rs_print_esc_bytes(globals, b"delimiter");
            }
        }
        DIVIDE => rs_print_esc_bytes(globals, b"divide"),
        END_CS_NAME => rs_print_esc_bytes(globals, b"endcsname"),
        END_GROUP => rs_print_esc_bytes(globals, b"endgroup"),
        EX_SPACE => rs_print_esc(globals, ' ' as i32),
        EXPAND_AFTER => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"expandafter");
            } else {
                rs_print_esc_bytes(globals, b"unless");
            }
        }
        HALIGN => rs_print_esc_bytes(globals, b"halign"),
        HRULE => rs_print_esc_bytes(globals, b"hrule"),
        IGNORE_SPACES => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"ignorespaces");
            } else {
                rs_print_esc_bytes(globals, b"primitive");
            }
        }
        INSERT => rs_print_esc_bytes(globals, b"insert"),
        ITAL_CORR => rs_print_esc(globals, '/' as i32),
        MARK => {
            rs_print_esc_bytes(globals, b"mark");
            if chr_code > 0 {
                rs_print_char(globals, 's' as i32);
            }
        }
        MATH_ACCENT => {
            if chr_code == 1 {
                rs_print_esc_bytes(globals, b"Umathaccent");
            } else {
                rs_print_esc_bytes(globals, b"mathaccent");
            }
        }
        MATH_CHAR_NUM => {
            let s: &[u8] = if chr_code == 2 {
                b"Umathchar"
            } else if chr_code == 1 {
                b"Umathcharnum"
            } else {
                b"mathchar"
            };
            rs_print_esc_bytes(globals, s);
        }
        MATH_CHOICE => rs_print_esc_bytes(globals, b"mathchoice"),
        MULTIPLY => rs_print_esc_bytes(globals, b"multiply"),
        NO_ALIGN => rs_print_esc_bytes(globals, b"noalign"),
        NO_BOUNDARY => rs_print_esc_bytes(globals, b"noboundary"),
        NO_EXPAND => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"noexpand");
            } else {
                rs_print_esc_bytes(globals, b"primitive");
            }
        }
        NON_SCRIPT => rs_print_esc_bytes(globals, b"nonscript"),
        OMIT => rs_print_esc_bytes(globals, b"omit"),
        RADICAL => {
            if chr_code == 1 {
                rs_print_esc_bytes(globals, b"Uradical");
            } else {
                rs_print_esc_bytes(globals, b"radical");
            }
        }
        READ_TO_CS => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"read");
            } else {
                rs_print_esc_bytes(globals, b"readline");
            }
        }
        RELAX => rs_print_esc_bytes(globals, b"relax"),
        SET_BOX => rs_print_esc_bytes(globals, b"setbox"),
        SET_PREV_GRAF => rs_print_esc_bytes(globals, b"prevgraf"),
        SET_SHAPE => {
            let s = match Local::try_from(chr_code - LOCAL_BASE as i32) {
                Ok(Local::ParShape) => b"parshape" as &[_],
                Ok(_) | Err(_) => match EtexPenaltiesPar::try_from(chr_code - ETEX_PEN_BASE as i32)
                {
                    Ok(EtexPenaltiesPar::InterLinePenalties) => b"interlinepenalties" as &[_],
                    Ok(EtexPenaltiesPar::ClubPenalties) => b"clubpenalties",
                    Ok(EtexPenaltiesPar::WidowPenalties) => b"widowpenalties",
                    Ok(EtexPenaltiesPar::DisplayWidowPenalties) => b"displaywidowpenalties",
                    Err(_) => b"",
                },
            };
            rs_print_esc_bytes(globals, s);
        }
        THE => {
            let s = if chr_code == 0 {
                b"the" as &[_]
            } else if chr_code == 1 {
                b"unexpanded"
            } else {
                b"detokenize"
            };
            rs_print_esc_bytes(globals, s);
        }
        TOKS_REGISTER => {
            rs_print_esc_bytes(globals, b"toks");
            if chr_code != 0 {
                rs_print_sa_num(globals, chr_code);
            }
        }
        VADJUST => rs_print_esc_bytes(globals, b"vadjust"),
        VALIGN => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"valign");
            } else {
                let s = match chr_code {
                    BEGIN_L_CODE => b"beginL" as &[_],
                    END_L_CODE => b"endL",
                    BEGIN_R_CODE => b"beginR",
                    _ => b"endR",
                };
                rs_print_esc_bytes(globals, s);
            }
        }
        VCENTER => rs_print_esc_bytes(globals, b"vcenter"),
        VRULE => rs_print_esc_bytes(globals, b"vrule"),
        PAR_END => rs_print_esc_bytes(globals, b"par"),
        INPUT => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"input");
            } else if chr_code == 2 {
                rs_print_esc_bytes(globals, b"scantokens");
            } else {
                rs_print_esc_bytes(globals, b"endinput");
            }
        }
        TOP_BOT_MARK => {
            let s = match chr_code % MARKS_CODE {
                FIRST_MARK_CODE => b"firstmark" as &[_],
                BOT_MARK_CODE => b"botmark",
                SPLIT_FIRST_MARK_CODE => b"splitfirstmark",
                SPLIT_BOT_MARK_CODE => b"splitbotmark",
                _ => b"topmark",
            };
            rs_print_esc_bytes(globals, s);
            if chr_code >= MARKS_CODE {
                rs_print_char(globals, 's' as i32);
            }
        }
        REGISTER => {
            let (new_cmd, new_chr) = if chr_code < 0 || chr_code > 19 {
                (
                    unsafe { globals.engine.mem[chr_code as usize].b16.s1 / 64 } as i32,
                    chr_code,
                )
            } else {
                (chr_code, TEX_NULL)
            };

            let s: &[_] = if new_cmd == INT_VAL {
                b"count"
            } else if new_cmd == DIMEN_VAL {
                b"dimen"
            } else if new_cmd == GLUE_VAL {
                b"skip"
            } else {
                b"muskip"
            };
            rs_print_esc_bytes(globals, s);
            if new_chr != TEX_NULL {
                rs_print_sa_num(globals, new_chr);
            }
        }
        SET_AUX => {
            if chr_code == VMODE {
                rs_print_esc_bytes(globals, b"prevdepth")
            } else {
                rs_print_esc_bytes(globals, b"spacefactor")
            }
        }
        SET_PAGE_INT => {
            let s: &[_] = if chr_code == 0 {
                b"deadcycles"
            } else if chr_code == 2 {
                b"interactionmode"
            } else {
                b"insertpenalties"
            };
            rs_print_esc_bytes(globals, s);
        }
        SET_BOX_DIMEN => {
            let s = if chr_code == WIDTH_OFFSET {
                b"wd"
            } else if chr_code == HEIGHT_OFFSET {
                b"ht"
            } else {
                b"dp"
            };
            rs_print_esc_bytes(globals, s);
        }
        LAST_ITEM => {
            let s: &[_] = match chr_code {
                INT_VAL => b"lastpenalty",
                DIMEN_VAL => b"lastkern",
                GLUE_VAL => b"lastskip",
                INPUT_LINE_NO_CODE => b"inputlineno",
                LAST_NODE_TYPE_CODE => b"lastnodetype",
                ETEX_VERSION_CODE => b"eTeXversion",
                XETEX_VERSION_CODE => b"XeTeXversion",
                XETEX_COUNT_GLYPHS_CODE => b"XeTeXcountglyphs",
                XETEX_COUNT_VARIATIONS_CODE => b"XeTeXcountvariations",
                XETEX_VARIATION_CODE => b"XeTeXvariation",
                XETEX_FIND_VARIATION_BY_NAME_CODE => b"XeTeXfindvariationbyname",
                XETEX_VARIATION_MIN_CODE => b"XeTeXvariationmin",
                XETEX_VARIATION_MAX_CODE => b"XeTeXvariationmax",
                XETEX_VARIATION_DEFAULT_CODE => b"XeTeXvariationdefault",
                XETEX_COUNT_FEATURES_CODE => b"XeTeXcountfeatures",
                XETEX_FEATURE_CODE_CODE => b"XeTeXfeaturecode",
                XETEX_FIND_FEATURE_BY_NAME_CODE => b"XeTeXfindfeaturebyname",
                XETEX_IS_EXCLUSIVE_FEATURE_CODE => b"XeTeXisexclusivefeature",
                XETEX_COUNT_SELECTORS_CODE => b"XeTeXcountselectors",
                XETEX_SELECTOR_CODE_CODE => b"XeTeXselectorcode",
                XETEX_FIND_SELECTOR_BY_NAME_CODE => b"XeTeXfindselectorbyname",
                XETEX_IS_DEFAULT_SELECTOR_CODE => b"XeTeXisdefaultselector",
                XETEX_OT_COUNT_SCRIPTS_CODE => b"XeTeXOTcountscripts",
                XETEX_OT_COUNT_LANGUAGES_CODE => b"XeTeXOTcountlanguages",
                XETEX_OT_COUNT_FEATURES_CODE => b"XeTeXOTcountfeatures",
                XETEX_OT_SCRIPT_CODE => b"XeTeXOTscripttag",
                XETEX_OT_LANGUAGE_CODE => b"XeTeXOTlanguagetag",
                XETEX_OT_FEATURE_CODE => b"XeTeXOTfeaturetag",
                XETEX_MAP_CHAR_TO_GLYPH_CODE => b"XeTeXcharglyph",
                XETEX_GLYPH_INDEX_CODE => b"XeTeXglyphindex",
                XETEX_GLYPH_BOUNDS_CODE => b"XeTeXglyphbounds",
                XETEX_FONT_TYPE_CODE => b"XeTeXfonttype",
                XETEX_FIRST_CHAR_CODE => b"XeTeXfirstfontchar",
                XETEX_LAST_CHAR_CODE => b"XeTeXlastfontchar",
                XETEX_PDF_PAGE_COUNT_CODE => b"XeTeXpdfpagecount",
                CURRENT_GROUP_LEVEL_CODE => b"currentgrouplevel",
                CURRENT_GROUP_TYPE_CODE => b"currentgrouptype",
                CURRENT_IF_LEVEL_CODE => b"currentiflevel",
                CURRENT_IF_TYPE_CODE => b"currentiftype",
                CURRENT_IF_BRANCH_CODE => b"currentifbranch",
                FONT_CHAR_WD_CODE => b"fontcharwd",
                FONT_CHAR_HT_CODE => b"fontcharht",
                FONT_CHAR_DP_CODE => b"fontchardp",
                FONT_CHAR_IC_CODE => b"fontcharic",
                PAR_SHAPE_LENGTH_CODE => b"parshapelength",
                PAR_SHAPE_INDENT_CODE => b"parshapeindent",
                PAR_SHAPE_DIMEN_CODE => b"parshapedimen",
                n if n == (ETEX_EXPR - INT_VAL + INT_VAL) => b"numexpr",
                n if n == (ETEX_EXPR - INT_VAL + DIMEN_VAL) => b"dimexpr",
                n if n == (ETEX_EXPR - INT_VAL + GLUE_VAL) => b"glueexpr",
                n if n == (ETEX_EXPR - INT_VAL + MU_VAL) => b"muexpr",
                GLUE_STRETCH_ORDER_CODE => b"gluestretchorder",
                GLUE_SHRINK_ORDER_CODE => b"glueshrinkorder",
                GLUE_STRETCH_CODE => b"gluestretch",
                GLUE_SHRINK_CODE => b"glueshrink",
                MU_TO_GLUE_CODE => b"mutoglue",
                GLUE_TO_MU_CODE => b"gluetomu",
                PDF_LAST_X_POS_CODE => b"pdflastxpos",
                PDF_LAST_Y_POS_CODE => b"pdflastypos",
                ELAPSED_TIME_CODE => b"elapsedtime",
                PDF_SHELL_ESCAPE_CODE => b"shellescape",
                RANDOM_SEED_CODE => b"randomseed",
                _ => b"badness",
            };
            rs_print_esc_bytes(globals, s);
        }
        CONVERT => {
            let s: &[_] = match chr_code {
                NUMBER_CODE => b"number",
                ROMAN_NUMERAL_CODE => b"romannumeral",
                STRING_CODE => b"string",
                MEANING_CODE => b"meaning",
                FONT_NAME_CODE => b"fontname",
                ETEX_REVISION_CODE => b"eTeXrevision",
                EXPANDED_CODE => b"expanded",
                LEFT_MARGIN_KERN_CODE => b"leftmarginkern",
                RIGHT_MARGIN_KERN_CODE => b"rightmarginkern",
                PDF_CREATION_DATE_CODE => b"creationdate",
                PDF_FILE_MOD_DATE_CODE => b"filemoddate",
                PDF_FILE_SIZE_CODE => b"filesize",
                PDF_MDFIVE_SUM_CODE => b"mdfivesum",
                PDF_FILE_DUMP_CODE => b"filedump",
                PDF_STRCMP_CODE => b"strcmp",
                UNIFORM_DEVIATE_CODE => b"uniformdeviate",
                NORMAL_DEVIATE_CODE => b"normaldeviate",
                XETEX_REVISION_CODE => b"XeTeXrevision",
                XETEX_VARIATION_NAME_CODE => b"XeTeXvariationname",
                XETEX_FEATURE_NAME_CODE => b"XeTeXfeaturename",
                XETEX_SELECTOR_NAME_CODE => b"XeTeXselectorname",
                XETEX_GLYPH_NAME_CODE => b"XeTeXglyphname",
                XETEX_UCHAR_CODE => b"Uchar",
                XETEX_UCHARCAT_CODE => b"Ucharcat",
                _ => b"jobname",
            };
            rs_print_esc_bytes(globals, s);
        }
        IF_TEST => {
            if chr_code >= UNLESS_CODE {
                rs_print_esc_bytes(globals, b"unless");
            }
            let s: &[_] = match chr_code % UNLESS_CODE {
                IF_CAT_CODE => b"ifcat",
                IF_INT_CODE => b"ifnum",
                IF_DIM_CODE => b"ifdim",
                IF_ODD_CODE => b"ifodd",
                IF_VMODE_CODE => b"ifvmode",
                IF_HMODE_CODE => b"ifhmode",
                IF_MMODE_CODE => b"ifmmode",
                IF_INNER_CODE => b"ifinner",
                IF_VOID_CODE => b"ifvoid",
                IF_HBOX_CODE => b"ifhbox",
                IF_VBOX_CODE => b"ifvbox",
                IFX_CODE => b"ifx",
                IF_EOF_CODE => b"ifeof",
                IF_TRUE_CODE => b"iftrue",
                IF_FALSE_CODE => b"iffalse",
                IF_CASE_CODE => b"ifcase",
                IF_PRIMITIVE_CODE => b"ifprimitive",
                IF_DEF_CODE => b"ifdefined",
                IF_CS_CODE => b"ifcsname",
                IF_FONT_CHAR_CODE => b"iffontchar",
                IF_IN_CSNAME_CODE => b"ifincsname",
                _ => b"if",
            };
            rs_print_esc_bytes(globals, s);
        }
        FI_OR_ELSE => {
            let s: &[_] = if chr_code == FI_CODE {
                b"fi"
            } else if chr_code == OR_CODE {
                b"or"
            } else {
                b"else"
            };
            rs_print_esc_bytes(globals, s);
        }
        TAB_MARK => {
            if chr_code == SPAN_CODE {
                rs_print_esc_bytes(globals, b"span");
            } else {
                rs_print_bytes(globals, b"alignment tab character ");
                print(globals, chr_code);
            }
        }
        CAR_RET => {
            let s: &[_] = if chr_code == CR_CODE { b"cr" } else { b"crcr" };
            rs_print_esc_bytes(globals, s);
        }
        SET_PAGE_DIMEN => {
            let s: &[_] = match chr_code {
                // Genuine literals in source XeTeX and WEB
                0 => b"pagegoal",
                1 => b"pagetotal",
                2 => b"pagestretch",
                3 => b"pagefilstretch",
                4 => b"pagefillstretch",
                5 => b"pagefilllstretch",
                6 => b"pageshrink",
                _ => b"pagedepth",
            };
            rs_print_esc_bytes(globals, s);
        }
        STOP => {
            if chr_code == 1 {
                rs_print_esc_bytes(globals, b"dump");
            } else {
                rs_print_esc_bytes(globals, b"end");
            }
        }
        HSKIP => {
            let s: &[_] = match chr_code {
                SKIP_CODE => b"hskip",
                FIL_CODE => b"hfil",
                FILL_CODE => b"hfill",
                SS_CODE => b"hss",
                _ => b"hfilneg",
            };
            rs_print_esc_bytes(globals, s);
        }
        VSKIP => {
            let s: &[_] = match chr_code {
                SKIP_CODE => b"vskip",
                FIL_CODE => b"vfil",
                FILL_CODE => b"vfill",
                SS_CODE => b"vss",
                _ => b"vfilneg",
            };
            rs_print_esc_bytes(globals, s);
        }
        MSKIP => rs_print_esc_bytes(globals, b"mskip"),
        KERN => rs_print_esc_bytes(globals, b"kern"),
        MKERN => rs_print_esc_bytes(globals, b"mkern"),
        HMOVE => {
            if chr_code == 1 {
                rs_print_esc_bytes(globals, b"moveleft");
            } else {
                rs_print_esc_bytes(globals, b"moveright");
            }
        }
        VMOVE => {
            if chr_code == 1 {
                rs_print_esc_bytes(globals, b"raise");
            } else {
                rs_print_esc_bytes(globals, b"lower");
            }
        }
        MAKE_BOX => {
            let s: &[_] = match chr_code {
                BOX_CODE => b"box",
                COPY_CODE => b"copy",
                LAST_BOX_CODE => b"lastbox",
                VSPLIT_CODE => b"vsplit",
                VTOP_CODE => b"vtop",
                // Originally VTOP_CODE + VMODE
                TT_VBOX_CODE => b"vbox",
                _ => b"hbox",
            };
            rs_print_esc_bytes(globals, s);
        }
        LEADER_SHIP => {
            let s: &[_] = match chr_code {
                A_LEADERS => b"leaders",
                C_LEADERS => b"cleaders",
                X_LEADERS => b"xleaders",
                _ => b"shipout",
            };
            rs_print_esc_bytes(globals, s);
        }
        START_PAR => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"noindent");
            } else {
                rs_print_esc_bytes(globals, b"indent");
            }
        }
        REMOVE_ITEM => {
            let s: &[_] = if chr_code == GLUE_NODE {
                b"unskip"
            } else if chr_code == KERN_NODE {
                b"unkern"
            } else {
                b"unpenalty"
            };
            rs_print_esc_bytes(globals, s);
        }
        UN_HBOX => {
            if chr_code == COPY_CODE {
                rs_print_esc_bytes(globals, b"unhcopy");
            } else {
                rs_print_esc_bytes(globals, b"unhbox")
            }
        }
        UN_VBOX => {
            let s: &[_] = match chr_code {
                COPY_CODE => b"unvcopy",
                LAST_BOX_CODE => b"pagediscards",
                VSPLIT_CODE => b"splitdiscards",
                _ => b"unvbox",
            };
            rs_print_esc_bytes(globals, s);
        }
        DISCRETIONARY => {
            if chr_code == 1 {
                rs_print_esc(globals, '-' as i32);
            } else {
                rs_print_esc_bytes(globals, b"discretionary");
            }
        }
        EQ_NO => {
            if chr_code == 1 {
                rs_print_esc_bytes(globals, b"leqno")
            } else {
                rs_print_esc_bytes(globals, b"eqno")
            }
        }
        MATH_COMP => {
            let s: &[_] = match chr_code {
                ORD_NOAD => b"mathord",
                OP_NOAD => b"mathop",
                BIN_NOAD => b"mathbin",
                REL_NOAD => b"mathrel",
                OPEN_NOAD => b"mathopen",
                CLOSE_NOAD => b"mathclose",
                PUNCT_NOAD => b"mathpunct",
                INNER_NOAD => b"mathinner",
                UNDER_NOAD => b"underline",
                _ => b"overline",
            };
            rs_print_esc_bytes(globals, s);
        }
        LIMIT_SWITCH => {
            let s: &[_] = match chr_code {
                LIMITS => b"limits",
                NO_LIMITS => b"nolimits",
                _ => b"displaylimits",
            };
            rs_print_esc_bytes(globals, s);
        }
        MATH_STYLE => rs_print_style(globals, chr_code),
        ABOVE => {
            let s: &[_] = if chr_code < DELIMITED_CODE {
                match chr_code {
                    OVER_CODE => b"over",
                    ATOP_CODE => b"atop",
                    ABOVE_CODE => b"above",
                    _ => b"",
                }
            } else {
                match chr_code - DELIMITED_CODE {
                    ABOVE_CODE => b"abovewithdelims",
                    OVER_CODE => b"overwithdelims",
                    ATOP_CODE => b"atopwithdelims",
                    _ => b"",
                }
            };
            rs_print_esc_bytes(globals, s);
        }
        LEFT_RIGHT => {
            let s: &[_] = match chr_code {
                LEFT_NOAD => b"left",
                MIDDLE_NOAD => b"middle",
                _ => b"right",
            };
            rs_print_esc_bytes(globals, s);
        }
        PREFIX => {
            let s: &[_] = match chr_code {
                1 => b"long",
                2 => b"outer",
                8 => b"protected",
                _ => b"global",
            };
            rs_print_esc_bytes(globals, s);
        }
        DEF => {
            let s: &[_] = match chr_code {
                0 => b"def",
                1 => b"gdef",
                2 => b"edef",
                _ => b"xdef",
            };
            rs_print_esc_bytes(globals, s);
        }
        LET => {
            if chr_code != NORMAL {
                rs_print_esc_bytes(globals, b"futurelet");
            } else {
                rs_print_esc_bytes(globals, b"let");
            }
        }
        SHORTHAND_DEF => {
            let s: &[_] = match chr_code {
                CHAR_DEF_CODE => b"chardef",
                MATH_CHAR_DEF_CODE => b"mathchardef",
                XETEX_MATH_CHAR_DEF_CODE => b"Umathchardef",
                XETEX_MATH_CHAR_NUM_DEF_CODE => b"Umathcharnumdef",
                COUNT_DEF_CODE => b"countdef",
                DIMEN_DEF_CODE => b"dimendef",
                SKIP_DEF_CODE => b"skipdef",
                MU_SKIP_DEF_CODE => b"muskipdef",
                CHAR_SUB_DEF_CODE => b"charsubdef",
                _ => b"toksdef",
            };
            rs_print_esc_bytes(globals, s);
        }
        CHAR_GIVEN => {
            rs_print_esc_bytes(globals, b"char");
            rs_print_hex(globals, chr_code);
        }
        MATH_GIVEN => {
            rs_print_esc_bytes(globals, b"mathchar");
            rs_print_hex(globals, chr_code);
        }
        XETEX_MATH_GIVEN => {
            rs_print_esc_bytes(globals, b"Umathchar");
            rs_print_hex(globals, math_class(chr_code) as i32);
            rs_print_hex(globals, math_fam(chr_code) as i32);
            rs_print_hex(globals, math_char(chr_code) as i32);
        }
        DEF_CODE => {
            let s: &[_] = match chr_code as usize {
                CAT_CODE_BASE => b"catcode",
                MATH_CODE_BASE => b"mathcode",
                LC_CODE_BASE => b"lccode",
                UC_CODE_BASE => b"uccode",
                SF_CODE_BASE => b"sfcode",
                _ => b"delcode",
            };
            rs_print_esc_bytes(globals, s);
        }
        XETEX_DEF_CODE => {
            const MATH_STYLE_: usize = MATH_STYLE as usize;
            let s: &[_] = match chr_code as usize {
                SF_CODE_BASE => b"XeTeXcharclass",
                MATH_STYLE_ => b"Umathcodenum",
                n if n == MATH_STYLE_ + 1 => b"Umathcode",
                DEL_CODE_BASE => b"Udelcodenum",
                _ => b"Udelcode",
            };
            rs_print_esc_bytes(globals, s);
        }
        DEF_FAMILY => rs_print_size(globals, chr_code - MATH_FONT_BASE as i32),
        HYPH_DATA => {
            if chr_code == 1 {
                rs_print_esc_bytes(globals, b"patterns")
            } else {
                rs_print_esc_bytes(globals, b"hyphenation")
            }
        }
        ASSIGN_FONT_INT => {
            let s: &[_] = match chr_code {
                0 => b"hyphenchar",
                1 => b"skewchar",
                LP_CODE_BASE => b"lpcode",
                RP_CODE_BASE => b"rpcode",
                _ => b"",
            };
            rs_print_esc_bytes(globals, s);
        }
        SET_FONT => {
            rs_print_bytes(globals, b"select font ");
            let font = chr_code as usize;
            let font_name_str = globals.fonts.font_name[font];
            if globals.fonts.font_area[font] == AAT_FONT_FLAG
                || globals.fonts.font_area[font] == OTGR_FONT_FLAG
            {
                let end = rs_str_length(globals.strings, font_name_str) - 1;
                let mut quote_char = '"';

                let start = globals.strings.str_start[font_name_str as usize - 65536] as usize;
                for idx in start..=end {
                    if globals.strings.str_pool[idx] == '"' as u16 {
                        quote_char = '\'';
                    }
                }

                rs_print_char(globals, quote_char as i32);
                rs_print(globals, font_name_str);
                rs_print_char(globals, quote_char as i32);
            } else {
                rs_print(globals, font_name_str)
            };

            if globals.fonts.font_size[font] != globals.fonts.font_dsize[font] {
                rs_print_bytes(globals, b" at ");
                rs_print_scaled(globals, globals.fonts.font_size[font]);
                rs_print_bytes(globals, b"pt");
            }
        }
        SET_INTERACTION => {
            let s: &[_] = match InteractionMode::try_from(chr_code as u8) {
                Ok(InteractionMode::Batch) => b"batchmode",
                Ok(InteractionMode::Nonstop) => b"nonstopmode",
                Ok(InteractionMode::Scroll) => b"scrollmode",
                Ok(InteractionMode::ErrorStop) => b"errorstopmode",
                Err(_) => b"",
            };
            rs_print_esc_bytes(globals, s);
        }
        IN_STREAM => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"closein")
            } else {
                rs_print_esc_bytes(globals, b"openin")
            }
        }
        MESSAGE => {
            if chr_code == 0 {
                rs_print_esc_bytes(globals, b"message")
            } else {
                rs_print_esc_bytes(globals, b"errmessage")
            }
        }
        CASE_SHIFT => {
            if chr_code == LC_CODE_BASE as i32 {
                rs_print_esc_bytes(globals, b"lowercase")
            } else {
                rs_print_esc_bytes(globals, b"uppercase")
            }
        }
        XRAY => {
            let s: &[_] = match chr_code {
                SHOW_BOX_CODE => b"showbox",
                SHOW_THE_CODE => b"showthe",
                SHOW_LISTS => b"showlists",
                SHOW_GROUPS => b"showgroups",
                SHOW_TOKENS => b"showtokens",
                SHOW_IFS => b"showifs",
                _ => b"show",
            };
            rs_print_esc_bytes(globals, s);
        }
        UNDEFINED_CS => rs_print_bytes(globals, b"undefined"),
        CALL | LONG_CALL | OUTER_CALL | LONG_OUTER_CALL => {
            let mut n = cmd - CALL as u16;
            let m = unsafe { globals.engine.mem[chr_code as usize].b32.s1 };
            if unsafe { globals.engine.mem[m as usize].b32.s0 } == PROTECTED_TOKEN {
                n += 4;
            }
            if (n / 4) % 2 == 1 {
                rs_print_esc_bytes(globals, b"protected")
            }
            if n % 2 == 1 {
                rs_print_esc_bytes(globals, b"long")
            }
            if (n / 2) % 2 == 1 {
                rs_print_esc_bytes(globals, b"outer")
            }
            if n > 0 {
                rs_print_char(globals, ' ' as i32);
            }
            rs_print_bytes(globals, b"macro");
        }
        END_TEMPLATE => rs_print_esc_bytes(globals, b"outer endtemplate"),
        EXTENSION => {
            let s: &[_] = match chr_code {
                OPEN_NODE => b"openout",
                WRITE_NODE => b"write",
                CLOSE_NODE => b"closeout",
                SPECIAL_NODE => b"special",
                IMMEDIATE_CODE => b"immediate",
                SET_LANGUAGE_CODE => b"setlanguage",
                PDF_SAVE_POS_NODE => b"pdfsavepos",
                RESET_TIMER_CODE => b"resettimer",
                SET_RANDOM_SEED_CODE => b"setrandomseed",
                PIC_FILE_CODE => b"XeTeXpicfile",
                PDF_FILE_CODE => b"XeTeXpdffile",
                GLYPH_CODE => b"XeTeXglyph",
                XETEX_LINEBREAK_LOCALE_EXTENSION_CODE => b"XeTeXlinebreaklocale",
                XETEX_INPUT_ENCODING_EXTENSION_CODE => b"XeTeXinputencoding",
                XETEX_DEFAULT_ENCODING_EXTENSION_CODE => b"XeTeXdefaultencoding",
                _ => {
                    rs_print_bytes(globals, b"[unknown extension!]");
                    return;
                }
            };
            rs_print_esc_bytes(globals, s);
        }
        _ => rs_print_bytes(globals, b"[unknown command code!]"),
    }
}

#[no_mangle]
pub extern "C" fn print_cmd_chr(cmd: u16, chr_code: i32) {
    Globals::with(|globals| rs_print_cmd_chr(globals, cmd, chr_code))
}
