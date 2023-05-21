use crate::c_api::auxi::{cur_aux, cur_aux_ln};
use crate::c_api::bibs::{bib_line_num, cur_bib};
use crate::c_api::buffer::{with_buffers, with_buffers_mut, BufTy};
use crate::c_api::char_info::LexClass;
use crate::c_api::cite::with_cites;
use crate::c_api::exec::{bst_ex_warn_print, bst_ln_num_print, ExecCtx};
use crate::c_api::hash::{with_hash, FnClass};
use crate::c_api::history::{mark_error, mark_fatal, mark_warning};
use crate::c_api::other::with_other;
use crate::c_api::peekable::input_ln;
use crate::c_api::pool::with_pool;
use crate::c_api::scan::ScanRes;
use crate::c_api::{
    ttstub_output_close, ttstub_output_open, ttstub_output_open_stdout, ASCIICode, CResult,
    FieldLoc, GlblCtx, HashPointer, StrNumber,
};
use std::cell::Cell;
use std::ffi::CStr;
use std::io::Write;
use std::slice;
use tectonic_io_base::OutputHandle;

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for str {
    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsBytes for String {
    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsBytes for [u8] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

thread_local! {
    static STANDARD_OUTPUT: Cell<Option<&'static mut OutputHandle>> = Cell::new(None);
    static LOG_FILE: Cell<Option<&'static mut OutputHandle>> = Cell::new(None);
}

pub(crate) fn reset() {
    STANDARD_OUTPUT.with(|cell| cell.set(None));
    LOG_FILE.with(|cell| cell.set(None));
}

fn with_stdout<T>(f: impl FnOnce(&mut OutputHandle) -> T) -> T {
    STANDARD_OUTPUT.with(|out| {
        let mut stdout = out.replace(None);
        let res = f(stdout.as_mut().unwrap());
        out.set(stdout);
        res
    })
}

fn with_log<T>(f: impl FnOnce(&mut OutputHandle) -> T) -> T {
    LOG_FILE.with(|out| {
        let mut log = out.replace(None);
        let res = f(log.as_mut().unwrap());
        out.set(log);
        res
    })
}

pub(crate) fn write_logs<B: ?Sized + AsBytes>(str: &B) {
    with_log(|log| log.write_all(str.as_bytes())).unwrap();
    with_stdout(|out| out.write_all(str.as_bytes())).unwrap();
}

pub fn init_log_file(file: &CStr) -> bool {
    LOG_FILE.with(|log| {
        let ptr = log.replace(None);
        if ptr.is_none() {
            // SAFETY: Our CStr is valid for the length of the call, so this can't access bad memory
            let new = unsafe { ttstub_output_open(file.as_ptr(), 0) };
            // SAFETY: Return of ttstub_output_open should be valid if non-null
            log.set(unsafe { new.as_mut() });
            !new.is_null()
        } else {
            log.set(ptr);
            true
        }
    })
}

#[no_mangle]
pub extern "C" fn init_standard_output() -> bool {
    STANDARD_OUTPUT.with(|out| {
        let ptr = out.replace(None);
        if ptr.is_none() {
            // SAFETY: This is actually fine to call, just extern
            let stdout = unsafe { ttstub_output_open_stdout() };
            // SAFETY: Pointer from ttstub_output_open_stdout is valid if non-null
            out.set(unsafe { stdout.as_mut() });
            !stdout.is_null()
        } else {
            out.set(ptr);
            true
        }
    })
}

#[no_mangle]
pub extern "C" fn bib_close_log() {
    LOG_FILE.with(|log| {
        let log = log.replace(None);
        if let Some(log) = log {
            // SAFETY: Log is valid due to being a mut ref
            unsafe { ttstub_output_close(log) };
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn bib_log_prints(str: *const libc::c_char) {
    let str = CStr::from_ptr(str);
    with_log(|log| log.write_all(str.to_bytes())).unwrap()
}

#[no_mangle]
pub extern "C" fn putc_log(c: libc::c_int) {
    let c = c as u8;
    with_log(|log| log.write_all(&[c])).unwrap();
    with_stdout(|out| out.write_all(&[c])).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn puts_log(str: *const libc::c_char) {
    let str = CStr::from_ptr(str);
    with_log(|log| log.write_all(str.to_bytes())).unwrap();
    with_stdout(|out| out.write_all(str.to_bytes())).unwrap();
}

#[no_mangle]
pub extern "C" fn print_overflow() {
    write_logs("Sorry---you've exceeded BibTeX's ");
    mark_fatal();
}

#[no_mangle]
pub extern "C" fn print_confusion() {
    write_logs("---this can't happen\n*Please notify the Tectonic maintainer*\n");
    mark_fatal();
}

pub fn out_token(handle: &mut OutputHandle) {
    with_buffers(|b| {
        let bytes = b.buffer(BufTy::Base);
        let start = b.offset(BufTy::Base, 1) as usize;
        let end = b.offset(BufTy::Base, 2) as usize;
        handle.write_all(&bytes[start..end]).unwrap();
    })
}

#[no_mangle]
pub extern "C" fn print_a_token() {
    with_stdout(out_token);
    with_log(out_token);
}

pub(crate) fn print_bad_input_line() {
    write_logs(" : ");

    with_buffers(|b| {
        let offset2 = b.offset(BufTy::Base, 2) as usize;

        let slice = &b.buffer(BufTy::Base)[0..offset2];

        for code in slice {
            if LexClass::of(*code) == LexClass::Whitespace {
                write_logs(" ");
            } else {
                write_logs(slice::from_ref(code))
            }
        }
        write_logs("\n : ");
        let str = (0..offset2).map(|_| ' ').collect::<String>();
        write_logs(&str);

        let last = b.init(BufTy::Base) as usize;
        if offset2 < last {
            let slice = &b.buffer(BufTy::Base)[offset2..last];
            for code in slice {
                if LexClass::of(*code) == LexClass::Whitespace {
                    write_logs(" ");
                } else {
                    write_logs(slice::from_ref(code));
                }
            }
        }

        write_logs("\n");

        if !slice
            .iter()
            .any(|c| LexClass::of(*c) != LexClass::Whitespace)
        {
            write_logs("(Error may have been on previous line)\n");
        }
    });

    mark_error();
}

pub(crate) fn print_skipping_whatever_remains() {
    write_logs("I'm skipping whatever remains of this ");
}

pub(crate) fn out_pool_str(handle: &mut OutputHandle, s: StrNumber) -> bool {
    with_pool(|pool| {
        let str = pool.try_get_str(s as usize);
        if let Ok(str) = str {
            handle.write_all(str).unwrap();
            true
        } else {
            write_logs(&format!("Illegal string number: {}", s));
            print_confusion();
            false
        }
    })
}

#[no_mangle]
pub extern "C" fn print_a_pool_str(s: StrNumber) -> bool {
    with_pool(|pool| {
        let str = pool.try_get_str(s as usize);
        if let Ok(str) = str {
            write_logs(str);
            true
        } else {
            write_logs(&format!("Illegal string number: {}", s));
            print_confusion();
            false
        }
    })
}

pub fn sam_wrong_file_name_print(file: &CStr) {
    with_stdout(|stdout| {
        writeln!(
            stdout,
            "I couldn't open file name `{}`",
            file.to_str().unwrap()
        )
        .unwrap();
    })
}

#[no_mangle]
pub extern "C" fn print_aux_name() -> bool {
    if !print_a_pool_str(cur_aux()) {
        return false;
    }
    write_logs("\n");
    true
}

#[no_mangle]
pub extern "C" fn log_pr_aux_name() -> bool {
    with_log(|log| {
        if !out_pool_str(log, cur_aux()) {
            return false;
        }
        write!(log, "\n").unwrap();
        true
    })
}

#[no_mangle]
pub extern "C" fn aux_err_print() -> bool {
    write_logs(&format!("---line {} of file ", cur_aux_ln()));
    if !print_aux_name() {
        return false;
    }
    print_bad_input_line();
    print_skipping_whatever_remains();
    write_logs("command\n");
    true
}

#[no_mangle]
pub extern "C" fn aux_err_illegal_another_print(cmd_num: i32) -> bool {
    write_logs("Illegal, another \\bib");
    match cmd_num {
        0 => write_logs("data"),
        1 => write_logs("style"),
        _ => {
            write_logs("Illegal auxiliary-file command");
            print_confusion();
            return false;
        }
    }
    write_logs(" command");
    true
}

#[no_mangle]
pub extern "C" fn aux_err_no_right_brace_print() {
    write_logs("No \"}\"");
}

#[no_mangle]
pub extern "C" fn aux_err_stuff_after_right_brace_print() {
    write_logs("Stuff after \"}\"");
}

#[no_mangle]
pub extern "C" fn aux_err_white_space_in_argument_print() {
    write_logs("White space in argument");
}

#[no_mangle]
pub extern "C" fn aux_end1_err_print() {
    write_logs("I found no ");
}

#[no_mangle]
pub extern "C" fn aux_end2_err_print() -> bool {
    write_logs("---while reading file ");
    if !print_aux_name() {
        return false;
    }
    mark_error();
    true
}

#[no_mangle]
pub extern "C" fn print_bib_name() -> bool {
    if !print_a_pool_str(cur_bib()) {
        return false;
    }
    let res = with_pool(|pool| {
        pool.try_get_str(cur_bib() as usize)
            .map(|str| str.ends_with(b".bib"))
    });
    match res {
        Ok(true) => (),
        Ok(false) => {
            write_logs(".bib");
        }
        Err(_) => return false,
    }
    write_logs("\n");
    true
}

#[no_mangle]
pub extern "C" fn log_pr_bib_name() -> bool {
    with_log(|log| {
        if !out_pool_str(log, cur_bib()) {
            return false;
        }
        let res = with_pool(|pool| {
            pool.try_get_str(cur_bib() as usize)
                .map(|str| str.ends_with(b".bib"))
        });
        match res {
            Ok(true) => (),
            Ok(false) => {
                write!(log, ".bib").unwrap();
            }
            Err(_) => return false,
        }
        write!(log, "\n").unwrap();
        true
    })
}

#[no_mangle]
pub unsafe extern "C" fn log_pr_bst_name(ctx: *const GlblCtx) -> bool {
    with_log(|log| {
        if !out_pool_str(log, (*ctx).bst_str) {
            return false;
        }
        write!(log, ".bst\n").unwrap();
        true
    })
}

#[no_mangle]
pub extern "C" fn hash_cite_confusion() {
    write_logs("Cite hash error");
    print_confusion();
}

#[no_mangle]
pub unsafe extern "C" fn bst_warn_print(ctx: *const GlblCtx) -> bool {
    if !bst_ln_num_print(ctx) {
        return false;
    }
    mark_warning();
    true
}

#[no_mangle]
pub extern "C" fn eat_bst_print() {
    write_logs("Illegal end of style file in command: ");
}

#[no_mangle]
pub extern "C" fn unknwn_function_class_confusion() {
    write_logs("Unknown function class");
    print_confusion();
}

pub fn id_scanning_confusion() {
    write_logs("Identifier scanning error");
    print_confusion();
}

#[no_mangle]
pub extern "C" fn bst_id_print(scan_result: ScanRes) -> bool {
    with_buffers(|buffers| {
        let char = buffers.at_offset(BufTy::Base, 2) as char;
        match scan_result {
            ScanRes::IdNull => {
                write_logs(&format!("\"{}\" begins identifier, command: ", char));
                true
            }
            ScanRes::OtherCharAdjacent => {
                write_logs(&format!(
                    "\"{}\" immediately follows identifier, command: ",
                    char
                ));
                true
            }
            _ => {
                id_scanning_confusion();
                false
            }
        }
    })
}

#[no_mangle]
pub extern "C" fn bst_left_brace_print() {
    write_logs("\"{\" is missing in command: ");
}

#[no_mangle]
pub extern "C" fn bst_right_brace_print() {
    write_logs("\"}\" is missing in command: ");
}

pub(crate) fn bib_ln_num_print() -> bool {
    write_logs(&format!("--line {} of file ", bib_line_num()));
    print_bib_name()
}

#[no_mangle]
pub extern "C" fn bib_err_print(at_bib_command: bool) -> bool {
    write_logs("-");
    if !bib_ln_num_print() {
        return false;
    }
    print_bad_input_line();
    print_skipping_whatever_remains();
    if at_bib_command {
        write_logs("command\n");
    } else {
        write_logs("entry\n");
    }
    true
}

#[no_mangle]
pub extern "C" fn bib_warn_print() -> bool {
    if !bib_ln_num_print() {
        return false;
    }
    mark_warning();
    true
}

#[no_mangle]
pub extern "C" fn eat_bib_print(at_bib_command: bool) -> bool {
    write_logs("Illegal end of database file");
    bib_err_print(at_bib_command)
}

#[no_mangle]
pub extern "C" fn bib_one_of_two_print(
    char1: ASCIICode,
    char2: ASCIICode,
    at_bib_command: bool,
) -> bool {
    write_logs(&format!(
        "I was expecting a `{}` or a `{}`",
        char1 as char, char2 as char
    ));
    bib_err_print(at_bib_command)
}

#[no_mangle]
pub extern "C" fn bib_equals_sign_print(at_bib_command: bool) -> bool {
    write_logs("I was expecting an \"=\"");
    bib_err_print(at_bib_command)
}

#[no_mangle]
pub extern "C" fn bib_unbalanced_braces_print(at_bib_command: bool) -> bool {
    write_logs("Unbalanced braces");
    bib_err_print(at_bib_command)
}

#[no_mangle]
pub extern "C" fn macro_warn_print() {
    write_logs("Warning--string name \"");
    print_a_token();
    write_logs("\" is ");
}

#[no_mangle]
pub extern "C" fn bib_id_print(scan_res: ScanRes) -> bool {
    match scan_res {
        ScanRes::IdNull => {
            write_logs("You're missing ");
            true
        }
        ScanRes::OtherCharAdjacent => {
            let char = with_buffers(|buffers| buffers.at_offset(BufTy::Base, 2));
            write_logs(&format!("\"{}\" immediately follows ", char));
            true
        }
        _ => {
            id_scanning_confusion();
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn bib_cmd_confusion() {
    write_logs("Unknown database-file command");
    print_confusion();
}

#[no_mangle]
pub extern "C" fn cite_key_disappeared_confusion() {
    write_logs("A cite key disappeared");
    print_confusion();
}

#[no_mangle]
pub extern "C" fn bad_cross_reference_print(s: StrNumber) -> bool {
    write_logs("--entry \"");
    let res = with_cites(|cites| print_a_pool_str(cites.get_cite(cites.ptr() as usize)));
    if !res {
        return false;
    }
    write_logs("\"\nrefers to entry \"");
    if !print_a_pool_str(s) {
        return false;
    }
    write_logs("\"");
    true
}

#[no_mangle]
pub extern "C" fn print_missing_entry(s: StrNumber) -> bool {
    write_logs("Warning--I didn't find a database entry for \"");
    if !print_a_pool_str(s) {
        return false;
    }
    write_logs("\"\n");
    mark_warning();
    true
}

pub(crate) unsafe fn bst_mild_ex_warn_print(ctx: &ExecCtx) -> bool {
    if ctx.mess_with_entries {
        write_logs(" for entry ");
        let res = with_cites(|cites| print_a_pool_str(cites.get_cite(cites.ptr() as usize)));
        if !res {
            return false;
        }
    }
    write_logs("\nwhile executing");
    bst_warn_print(ctx.glbl_ctx)
}

#[no_mangle]
pub unsafe extern "C" fn bst_cant_mess_with_entries_print(ctx: *const ExecCtx) -> bool {
    write_logs("You can't mess with entries here");
    bst_ex_warn_print(ctx)
}

#[no_mangle]
pub extern "C" fn bst_1print_string_size_exceeded() {
    write_logs("Warning--you've exceeded ");
}

#[no_mangle]
pub unsafe extern "C" fn bst_2print_string_size_exceeded(ctx: *const ExecCtx) -> bool {
    write_logs("-string-size,");
    if !bst_mild_ex_warn_print(&*ctx) {
        return false;
    }
    write_logs("*Please notify the bibstyle designer*\n");
    true
}

#[no_mangle]
pub unsafe extern "C" fn braces_unbalanced_complaint(
    ctx: *const ExecCtx,
    pop_lit_var: StrNumber,
) -> bool {
    write_logs("Warning--\"");
    if !print_a_pool_str(pop_lit_var) {
        return false;
    }
    write_logs("\" isn't a brace-balanced string");
    bst_mild_ex_warn_print(&*ctx)
}

#[no_mangle]
pub extern "C" fn case_conversion_confusion() {
    write_logs("Unknown type of case conversion");
    print_confusion();
}

#[no_mangle]
pub extern "C" fn print_fn_class(fn_loc: HashPointer) {
    let ty = with_hash(|hash| hash.ty(fn_loc as usize));
    match ty {
        FnClass::Builtin => write_logs("built-in"),
        FnClass::Wizard => write_logs("wizard-defined"),
        FnClass::IntLit => write_logs("integer-literal"),
        FnClass::StrLit => write_logs("string-literal"),
        FnClass::Field => write_logs("field"),
        FnClass::IntEntryVar => write_logs("integer-entry-variable"),
        FnClass::StrEntryVar => write_logs("string-entry-variable"),
        FnClass::IntGlblVar => write_logs("integer-global-variable"),
        FnClass::StrGlblVar => write_logs("string-global-variable"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn bst_err_print_and_look_for_blank_line(ctx: *mut GlblCtx) -> CResult {
    let ctx = &mut *ctx;

    write_logs("-");
    if !bst_ln_num_print(ctx) {
        return CResult::Error;
    }
    print_bad_input_line();
    while with_buffers(|buffers| buffers.init(BufTy::Base)) != 0 {
        if !input_ln(ctx.bst_file) {
            return CResult::Recover;
        } else {
            ctx.bst_line_num += 1;
        }
    }
    with_buffers_mut(|buffers| buffers.set_offset(BufTy::Base, 2, buffers.init(BufTy::Base)));

    CResult::Ok
}

#[no_mangle]
pub unsafe extern "C" fn already_seen_function_print(
    ctx: *mut GlblCtx,
    seen_fn_loc: HashPointer,
) -> CResult {
    if with_hash(|hash| !print_a_pool_str(hash.text(seen_fn_loc as usize))) {
        return CResult::Error;
    }
    write_logs(" is already a type \"");
    print_fn_class(seen_fn_loc);
    write_logs("\" function name\n");
    bst_err_print_and_look_for_blank_line(ctx)
}

#[no_mangle]
pub extern "C" fn nonexistent_cross_reference_error(field_ptr: FieldLoc) -> bool {
    write_logs("A bad cross reference-");
    if !bad_cross_reference_print(with_other(|other| other.field(field_ptr as usize))) {
        return false;
    }
    write_logs(", which doesn't exist\n");
    mark_error();
    true
}

#[no_mangle]
pub unsafe extern "C" fn output_bbl_line(ctx: *mut GlblCtx) {
    with_buffers_mut(|buffers| {
        if buffers.init(BufTy::Out) != 0 {
            let mut init = buffers.init(BufTy::Out);
            while init > 0 {
                if LexClass::of(buffers.at(BufTy::Out, (init - 1) as usize)) == LexClass::Whitespace
                {
                    init -= 1;
                } else {
                    break;
                }
            }
            buffers.set_init(BufTy::Out, init);
            if init == 0 {
                return;
            }
            let slice = &buffers.buffer(BufTy::Out)[..init as usize];
            (*(*ctx).bbl_file).write_all(slice).unwrap();
        }
        write!((*(*ctx).bbl_file), "\n").unwrap();
        (*ctx).bbl_line_num += 1;
        buffers.set_init(BufTy::Out, 0);
    })
}
