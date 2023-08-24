use crate::{
    c_api::{
        auxi::{cur_aux, cur_aux_ln},
        bibs::{bib_line_num, cur_bib},
        buffer::{with_buffers, with_buffers_mut, BufTy, GlobalBuffer},
        char_info::LexClass,
        cite::with_cites,
        exec::{bst_ln_num_print, rs_bst_ex_warn_print, ExecCtx},
        hash::{with_hash, FnClass},
        history::{mark_error, mark_fatal, mark_warning},
        other::with_other,
        peekable::rs_input_ln,
        pool::{with_pool, StringPool},
        scan::{Scan, ScanRes},
        ttstub_output_close, ttstub_output_open, ttstub_output_open_stdout, ASCIICode, Bibtex,
        CResult, FieldLoc, HashPointer, StrNumber,
    },
    BibtexError,
};
use std::{cell::Cell, ffi::CStr, io::Write, slice};
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
    let _ = with_log(|log| log.write_all(str.as_bytes()));
    let _ = with_stdout(|out| out.write_all(str.as_bytes()));
}

pub(crate) fn write_log_file<B: ?Sized + AsBytes>(str: &B) {
    with_log(|log| log.write_all(str.as_bytes())).unwrap();
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
    let _ = with_log(|log| log.write_all(str.to_bytes()));
}

#[no_mangle]
pub extern "C" fn putc_log(c: libc::c_int) {
    let c = c as u8;
    let _ = with_log(|log| log.write_all(&[c]));
    let _ = with_stdout(|out| out.write_all(&[c]));
}

#[no_mangle]
pub unsafe extern "C" fn puts_log(str: *const libc::c_char) {
    let str = CStr::from_ptr(str);
    let _ = with_log(|log| log.write_all(str.to_bytes()));
    let _ = with_stdout(|out| out.write_all(str.to_bytes()));
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

pub fn out_token(handle: &mut OutputHandle, buffers: &GlobalBuffer) {
    let bytes = buffers.buffer(BufTy::Base);
    let start = buffers.offset(BufTy::Base, 1);
    let end = buffers.offset(BufTy::Base, 2);
    handle.write_all(&bytes[start..end]).unwrap();
}

pub fn rs_print_a_token(buffers: &GlobalBuffer) {
    with_stdout(|stdout| out_token(stdout, buffers));
    with_log(|log| out_token(log, buffers));
}

#[no_mangle]
pub extern "C" fn print_a_token() {
    with_buffers(rs_print_a_token)
}

pub(crate) fn print_bad_input_line(buffers: &GlobalBuffer) {
    write_logs(" : ");

    let offset2 = buffers.offset(BufTy::Base, 2);

    let slice = &buffers.buffer(BufTy::Base)[0..offset2];

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

    let last = buffers.init(BufTy::Base);
    if offset2 < last {
        let slice = &buffers.buffer(BufTy::Base)[offset2..last];
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

    mark_error();
}

pub(crate) fn print_skipping_whatever_remains() {
    write_logs("I'm skipping whatever remains of this ");
}

pub(crate) fn out_pool_str(handle: &mut OutputHandle, s: StrNumber) -> bool {
    with_pool(|pool| {
        let str = pool.try_get_str(s);
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

pub fn rs_print_a_pool_str(s: StrNumber, pool: &StringPool) -> Result<(), BibtexError> {
    let str = pool.try_get_str(s);
    if let Ok(str) = str {
        write_logs(str);
        Ok(())
    } else {
        write_logs(&format!("Illegal string number: {}", s));
        print_confusion();
        Err(BibtexError::Fatal)
    }
}

#[no_mangle]
pub extern "C" fn print_a_pool_str(s: StrNumber) -> CResult {
    with_pool(|pool| rs_print_a_pool_str(s, pool)).into()
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
pub extern "C" fn print_aux_name() -> CResult {
    match print_a_pool_str(cur_aux()) {
        CResult::Ok => (),
        err => return err,
    }
    write_logs("\n");
    CResult::Ok
}

#[no_mangle]
pub extern "C" fn log_pr_aux_name() -> CResult {
    with_log(|log| {
        if !out_pool_str(log, cur_aux()) {
            return CResult::Error;
        }
        writeln!(log).unwrap();
        CResult::Ok
    })
}

#[no_mangle]
pub extern "C" fn aux_err_print() -> CResult {
    write_logs(&format!("---line {} of file ", cur_aux_ln()));
    match print_aux_name() {
        CResult::Ok => (),
        err => return err,
    }
    with_buffers(print_bad_input_line);
    print_skipping_whatever_remains();
    write_logs("command\n");
    CResult::Ok
}

#[no_mangle]
pub extern "C" fn aux_err_illegal_another_print(cmd_num: i32) -> CResult {
    write_logs("Illegal, another \\bib");
    match cmd_num {
        0 => write_logs("data"),
        1 => write_logs("style"),
        _ => {
            write_logs("Illegal auxiliary-file command");
            print_confusion();
            return CResult::Error;
        }
    }
    write_logs(" command");
    CResult::Ok
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
pub extern "C" fn aux_end2_err_print() -> CResult {
    write_logs("---while reading file ");
    match print_aux_name() {
        CResult::Ok => (),
        err => return err,
    }
    mark_error();
    CResult::Ok
}

#[no_mangle]
pub extern "C" fn print_bib_name() -> CResult {
    match print_a_pool_str(cur_bib()) {
        CResult::Ok => (),
        err => return err,
    }
    let res = with_pool(|pool| {
        pool.try_get_str(cur_bib())
            .map(|str| str.ends_with(b".bib"))
    });
    match res {
        Ok(true) => (),
        Ok(false) => {
            write_logs(".bib");
        }
        Err(_) => return CResult::Error,
    }
    write_logs("\n");
    CResult::Ok
}

#[no_mangle]
pub extern "C" fn log_pr_bib_name() -> CResult {
    with_log(|log| {
        if !out_pool_str(log, cur_bib()) {
            return CResult::Error;
        }
        let res = with_pool(|pool| {
            pool.try_get_str(cur_bib())
                .map(|str| str.ends_with(b".bib"))
        });
        match res {
            Ok(true) => (),
            Ok(false) => {
                write!(log, ".bib").unwrap();
            }
            Err(_) => return CResult::Error,
        }
        writeln!(log).unwrap();
        CResult::Ok
    })
}

#[no_mangle]
pub unsafe extern "C" fn log_pr_bst_name(ctx: *const Bibtex) -> CResult {
    with_log(|log| {
        if !out_pool_str(log, (*ctx).bst_str) {
            return CResult::Error;
        }
        writeln!(log, ".bst").unwrap();
        CResult::Ok
    })
}

#[no_mangle]
pub extern "C" fn hash_cite_confusion() {
    write_logs("Cite hash error");
    print_confusion();
}

#[no_mangle]
pub unsafe extern "C" fn bst_warn_print(ctx: *const Bibtex) -> CResult {
    match with_pool(|pool| bst_ln_num_print(&*ctx, pool)) {
        Ok(()) => (),
        err => return err.into(),
    }
    mark_warning();
    CResult::Ok
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
pub extern "C" fn bst_id_print(scan_result: ScanRes) -> CResult {
    with_buffers(|buffers| {
        let char = buffers.at_offset(BufTy::Base, 2) as char;
        match scan_result {
            ScanRes::IdNull => {
                write_logs(&format!("\"{}\" begins identifier, command: ", char));
                CResult::Ok
            }
            ScanRes::OtherCharAdjacent => {
                write_logs(&format!(
                    "\"{}\" immediately follows identifier, command: ",
                    char
                ));
                CResult::Ok
            }
            _ => {
                id_scanning_confusion();
                CResult::Error
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

pub(crate) fn bib_ln_num_print() -> Result<(), BibtexError> {
    write_logs(&format!("--line {} of file ", bib_line_num()));
    print_bib_name().into()
}

pub fn rs_bib_err_print(buffers: &GlobalBuffer, at_bib_command: bool) -> Result<(), BibtexError> {
    write_logs("-");
    bib_ln_num_print()?;
    print_bad_input_line(buffers);
    print_skipping_whatever_remains();
    if at_bib_command {
        write_logs("command\n");
    } else {
        write_logs("entry\n");
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn bib_err_print(at_bib_command: bool) -> CResult {
    with_buffers(|buffers| rs_bib_err_print(buffers, at_bib_command)).into()
}

#[no_mangle]
pub extern "C" fn bib_warn_print() -> CResult {
    match bib_ln_num_print() {
        Ok(()) => (),
        err => return err.into(),
    }
    mark_warning();
    CResult::Ok
}

pub fn rs_eat_bib_print(buffers: &GlobalBuffer, at_bib_command: bool) -> Result<(), BibtexError> {
    write_logs("Illegal end of database file");
    rs_bib_err_print(buffers, at_bib_command)
}

#[no_mangle]
pub extern "C" fn eat_bib_print(at_bib_command: bool) -> CResult {
    with_buffers(|buffers| rs_eat_bib_print(buffers, at_bib_command)).into()
}

#[no_mangle]
pub extern "C" fn bib_one_of_two_print(
    char1: ASCIICode,
    char2: ASCIICode,
    at_bib_command: bool,
) -> CResult {
    write_logs(&format!(
        "I was expecting a `{}' or a `{}'",
        char1 as char, char2 as char
    ));
    bib_err_print(at_bib_command)
}

#[no_mangle]
pub extern "C" fn bib_equals_sign_print(at_bib_command: bool) -> CResult {
    write_logs("I was expecting an \"=\"");
    bib_err_print(at_bib_command)
}

pub fn bib_unbalanced_braces_print(at_bib_command: bool) -> Result<(), BibtexError> {
    write_logs("Unbalanced braces");
    bib_err_print(at_bib_command).into()
}

pub fn macro_warn_print(buffers: &GlobalBuffer) {
    write_logs("Warning--string name \"");
    rs_print_a_token(buffers);
    write_logs("\" is ");
}

pub fn rs_bib_id_print(buffers: &GlobalBuffer, scan_res: ScanRes) -> Result<(), BibtexError> {
    match scan_res {
        ScanRes::IdNull => {
            write_logs("You're missing ");
            Ok(())
        }
        ScanRes::OtherCharAdjacent => {
            let char = buffers.at_offset(BufTy::Base, 2);
            write_logs(&format!("\"{}\" immediately follows ", char));
            Ok(())
        }
        _ => {
            id_scanning_confusion();
            Err(BibtexError::Fatal)
        }
    }
}

#[no_mangle]
pub extern "C" fn bib_id_print(scan_res: ScanRes) -> CResult {
    with_buffers(|buffers| rs_bib_id_print(buffers, scan_res)).into()
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
pub extern "C" fn bad_cross_reference_print(s: StrNumber) -> CResult {
    write_logs("--entry \"");
    match with_cites(|cites| print_a_pool_str(cites.get_cite(cites.ptr()))) {
        CResult::Ok => (),
        err => return err,
    }
    write_logs("\"\nrefers to entry \"");
    match print_a_pool_str(s) {
        CResult::Ok => (),
        err => return err,
    }
    write_logs("\"");
    CResult::Ok
}

#[no_mangle]
pub extern "C" fn print_missing_entry(s: StrNumber) -> CResult {
    write_logs("Warning--I didn't find a database entry for \"");
    match print_a_pool_str(s) {
        CResult::Ok => (),
        err => return err,
    }
    write_logs("\"\n");
    mark_warning();
    CResult::Ok
}

pub(crate) fn bst_mild_ex_warn_print(ctx: &ExecCtx) -> CResult {
    if ctx.mess_with_entries {
        write_logs(" for entry ");
        match with_cites(|cites| print_a_pool_str(cites.get_cite(cites.ptr()))) {
            CResult::Ok => (),
            err => return err,
        }
    }
    write_logs("\nwhile executing");
    // SAFETY: glbl_ctx pointer guaranteed valid
    unsafe { bst_warn_print(ctx.glbl_ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn bst_cant_mess_with_entries_print(ctx: *const ExecCtx) -> CResult {
    write_logs("You can't mess with entries here");
    with_pool(|pool| rs_bst_ex_warn_print(&*ctx, pool)).into()
}

#[no_mangle]
pub extern "C" fn bst_1print_string_size_exceeded() {
    write_logs("Warning--you've exceeded ");
}

#[no_mangle]
pub unsafe extern "C" fn bst_2print_string_size_exceeded(ctx: *const ExecCtx) -> CResult {
    write_logs("-string-size,");
    match bst_mild_ex_warn_print(&*ctx) {
        CResult::Ok => (),
        err => return err,
    }
    write_logs("*Please notify the bibstyle designer*\n");
    CResult::Ok
}

pub fn braces_unbalanced_complaint(
    ctx: &ExecCtx,
    pop_lit_var: StrNumber,
) -> Result<(), BibtexError> {
    write_logs("Warning--\"");
    match print_a_pool_str(pop_lit_var) {
        CResult::Ok => (),
        err => return err.into(),
    }
    write_logs("\" isn't a brace-balanced string");
    bst_mild_ex_warn_print(ctx).into()
}

#[no_mangle]
pub extern "C" fn case_conversion_confusion() {
    write_logs("Unknown type of case conversion");
    print_confusion();
}

#[no_mangle]
pub extern "C" fn print_fn_class(fn_loc: HashPointer) {
    let ty = with_hash(|hash| hash.ty(fn_loc));
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

pub fn rs_bst_err_print_and_look_for_blank_line(
    ctx: &mut Bibtex,
    buffers: &mut GlobalBuffer,
) -> Result<(), BibtexError> {
    write_logs("-");
    with_pool(|pool| bst_ln_num_print(ctx, pool))?;
    print_bad_input_line(buffers);
    while buffers.init(BufTy::Base) != 0 {
        // SAFETY: bst_file guaranteed valid
        let bst_file = unsafe { ctx.bst_file.map(|mut ptr| ptr.as_mut()) };
        if !rs_input_ln(bst_file, buffers) {
            return Err(BibtexError::Recover);
        } else {
            ctx.bst_line_num += 1;
        }
    }
    buffers.set_offset(BufTy::Base, 2, buffers.init(BufTy::Base));
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn bst_err_print_and_look_for_blank_line(ctx: *mut Bibtex) -> CResult {
    with_buffers_mut(|buffers| rs_bst_err_print_and_look_for_blank_line(&mut *ctx, buffers).into())
}

#[no_mangle]
pub unsafe extern "C" fn already_seen_function_print(
    ctx: *mut Bibtex,
    seen_fn_loc: HashPointer,
) -> CResult {
    match with_hash(|hash| print_a_pool_str(hash.text(seen_fn_loc))) {
        CResult::Ok => (),
        err => return err,
    }
    write_logs(" is already a type \"");
    print_fn_class(seen_fn_loc);
    write_logs("\" function name\n");
    with_buffers_mut(|buffers| rs_bst_err_print_and_look_for_blank_line(&mut *ctx, buffers)).into()
}

#[no_mangle]
pub extern "C" fn nonexistent_cross_reference_error(field_ptr: FieldLoc) -> CResult {
    write_logs("A bad cross reference-");
    match bad_cross_reference_print(with_other(|other| other.field(field_ptr))) {
        CResult::Ok => (),
        err => return err,
    }
    write_logs(", which doesn't exist\n");
    mark_error();
    CResult::Ok
}

pub fn rs_output_bbl_line(ctx: &mut Bibtex, buffers: &mut GlobalBuffer) {
    if buffers.init(BufTy::Out) != 0 {
        let mut init = buffers.init(BufTy::Out);
        while init > 0 {
            if LexClass::of(buffers.at(BufTy::Out, init - 1)) == LexClass::Whitespace {
                init -= 1;
            } else {
                break;
            }
        }
        buffers.set_init(BufTy::Out, init);
        if init == 0 {
            return;
        }
        let slice = &buffers.buffer(BufTy::Out)[..init];
        // SAFETY: The bbl_file pointer is guaranteed valid
        (unsafe { &mut *ctx.bbl_file }).write_all(slice).unwrap();
    }
    // SAFETY: The bbl_file pointer is guaranteed valid
    writeln!(unsafe { &mut *ctx.bbl_file }).unwrap();
    ctx.bbl_line_num += 1;
    buffers.set_init(BufTy::Out, 0);
}

#[no_mangle]
pub unsafe extern "C" fn output_bbl_line(ctx: *mut Bibtex) {
    with_buffers_mut(|buffers| rs_output_bbl_line(&mut *ctx, buffers))
}

pub fn skip_token_print(ctx: &Bibtex, buffers: &mut GlobalBuffer) -> Result<(), BibtexError> {
    write_logs("-");
    with_pool(|pool| bst_ln_num_print(ctx, pool))?;
    mark_error();

    Scan::new()
        .chars(&[b'}', b'%'])
        .class(LexClass::Whitespace)
        .scan_till(buffers, buffers.init(BufTy::Base));

    Ok(())
}

pub fn print_recursion_illegal(
    ctx: &Bibtex,
    buffers: &mut GlobalBuffer,
) -> Result<(), BibtexError> {
    write_logs("Curse you, wizard, before you recurse me:\nfunction ");
    rs_print_a_token(buffers);
    write_logs(" is illegal in its own definition\n");
    skip_token_print(ctx, buffers)
}

pub fn skip_token_unknown_function_print(
    ctx: &Bibtex,
    buffers: &mut GlobalBuffer,
) -> Result<(), BibtexError> {
    rs_print_a_token(buffers);
    write_logs(" is an unknown function");
    skip_token_print(ctx, buffers)
}

pub fn skip_illegal_stuff_after_token_print(
    ctx: &Bibtex,
    buffers: &mut GlobalBuffer,
) -> Result<(), BibtexError> {
    write_logs(&format!(
        "\"{}\" can't follow a literal",
        buffers.at_offset(BufTy::Base, 2) as char
    ));
    skip_token_print(ctx, buffers)
}

pub fn brace_lvl_one_letters_complaint(
    ctx: &mut ExecCtx,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    write_logs("The format string \"");
    rs_print_a_pool_str(ctx.pop1.unwrap_str(), pool)?;
    write_logs("\" has an illegal brace-level-1 letter");
    rs_bst_ex_warn_print(ctx, pool)?;
    Ok(())
}
