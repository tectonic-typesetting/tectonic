use crate::{
    auxi::AuxData,
    bibs::BibData,
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    cite::CiteInfo,
    exec::{bst_ex_warn_print, bst_ln_num_print, ExecCtx},
    hash::{FnClass, HashData},
    history::{mark_error, mark_fatal, mark_warning},
    other::OtherData,
    peekable::input_ln,
    pool::StringPool,
    scan::{Scan, ScanRes},
    ttbc_output_close, ttbc_output_open, ttbc_output_open_stdout, ASCIICode, Bibtex, BibtexError,
    CiteNumber, FieldLoc, HashPointer, StrNumber,
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

pub(crate) fn init_log_file(ctx: &mut Bibtex<'_, '_>, file: &CStr) -> bool {
    LOG_FILE.with(|log| {
        let ptr = log.replace(None);
        if ptr.is_none() {
            // SAFETY: Our CStr is valid for the length of the call, so this can't access bad memory
            let new = unsafe { ttbc_output_open(ctx.engine, file.as_ptr(), 0) };
            // SAFETY: Return of ttstub_output_open should be valid if non-null
            log.set(unsafe { new.as_mut() });
            !new.is_null()
        } else {
            log.set(ptr);
            true
        }
    })
}

pub(crate) fn init_standard_output(ctx: &mut Bibtex<'_, '_>) -> bool {
    STANDARD_OUTPUT.with(|out| {
        let ptr = out.replace(None);
        if ptr.is_none() {
            let stdout = ttbc_output_open_stdout(ctx.engine);
            // SAFETY: Pointer from ttstub_output_open_stdout is valid if non-null
            out.set(unsafe { stdout.as_mut() });
            !stdout.is_null()
        } else {
            out.set(ptr);
            true
        }
    })
}

pub(crate) fn bib_close_log(ctx: &mut Bibtex<'_, '_>) {
    LOG_FILE.with(|log| {
        let log = log.replace(None);
        if let Some(log) = log {
            ttbc_output_close(ctx.engine, log);
        }
    })
}

pub fn print_overflow() {
    write_logs("Sorry---you've exceeded BibTeX's ");
    mark_fatal();
}

pub fn print_confusion() {
    write_logs("---this can't happen\n*Please notify the Tectonic maintainer*\n");
    mark_fatal();
}

pub(crate) fn out_token(handle: &mut OutputHandle, buffers: &GlobalBuffer) {
    let bytes = buffers.buffer(BufTy::Base);
    let start = buffers.offset(BufTy::Base, 1);
    let end = buffers.offset(BufTy::Base, 2);
    handle.write_all(&bytes[start..end]).unwrap();
}

pub(crate) fn print_a_token(buffers: &GlobalBuffer) {
    with_stdout(|stdout| out_token(stdout, buffers));
    with_log(|log| out_token(log, buffers));
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

pub(crate) fn out_pool_str(
    pool: &StringPool,
    handle: &mut OutputHandle,
    s: StrNumber,
) -> Result<(), BibtexError> {
    let str = pool.try_get_str(s);
    if let Ok(str) = str {
        handle.write_all(str).unwrap();
        Ok(())
    } else {
        write_logs(&format!("Illegal string number: {}", s));
        print_confusion();
        Err(BibtexError::Fatal)
    }
}

pub(crate) fn print_a_pool_str(s: StrNumber, pool: &StringPool) -> Result<(), BibtexError> {
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

pub(crate) fn print_aux_name(pool: &StringPool, name: StrNumber) -> Result<(), BibtexError> {
    print_a_pool_str(name, pool)?;
    write_logs("\n");
    Ok(())
}

pub(crate) fn log_pr_aux_name(aux: &AuxData, pool: &StringPool) -> Result<(), BibtexError> {
    with_log(|log| {
        out_pool_str(pool, log, aux.top_file().name)?;
        writeln!(log).unwrap();
        Ok(())
    })
}

pub(crate) fn aux_err_print(
    buffers: &GlobalBuffer,
    aux: &AuxData,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    write_logs(&format!("---line {} of file ", aux.top_file().line));
    print_aux_name(pool, aux.top_file().name)?;
    print_bad_input_line(buffers);
    print_skipping_whatever_remains();
    write_logs("command\n");
    Ok(())
}

pub(crate) enum AuxTy {
    Data,
    Style,
}

pub(crate) fn aux_err_illegal_another_print(cmd: AuxTy) -> Result<(), BibtexError> {
    write_logs("Illegal, another \\bib");
    match cmd {
        AuxTy::Data => write_logs("data"),
        AuxTy::Style => write_logs("style"),
    }
    write_logs(" command");
    Ok(())
}

pub fn aux_err_no_right_brace_print() {
    write_logs("No \"}\"");
}

pub fn aux_err_stuff_after_right_brace_print() {
    write_logs("Stuff after \"}\"");
}

pub fn aux_err_white_space_in_argument_print() {
    write_logs("White space in argument");
}

pub fn aux_end1_err_print() {
    write_logs("I found no ");
}

pub(crate) fn aux_end2_err_print(pool: &StringPool, name: StrNumber) -> Result<(), BibtexError> {
    write_logs("---while reading file ");
    print_aux_name(pool, name)?;
    mark_error();
    Ok(())
}

pub(crate) fn print_bib_name(pool: &StringPool, bibs: &BibData) -> Result<(), BibtexError> {
    print_a_pool_str(bibs.cur_bib(), pool)?;
    let res = pool
        .try_get_str(bibs.cur_bib())
        .map_err(|_| BibtexError::Fatal)
        .map(|str| str.ends_with(b".bib"))?;
    if !res {
        write_logs(".bib");
    }
    write_logs("\n");
    Ok(())
}

pub(crate) fn log_pr_bib_name(bibs: &BibData, pool: &StringPool) -> Result<(), BibtexError> {
    with_log(|log| {
        out_pool_str(pool, log, bibs.cur_bib())?;
        let res = pool
            .try_get_str(bibs.cur_bib())
            .map(|str| str.ends_with(b".bib"))
            .map_err(|_| BibtexError::Fatal)?;
        if !res {
            write!(log, ".bib").unwrap();
        }
        writeln!(log).unwrap();
        Ok(())
    })
}

pub(crate) fn log_pr_bst_name(ctx: &Bibtex<'_, '_>, pool: &StringPool) -> Result<(), BibtexError> {
    with_log(|log| {
        // TODO: This call can panic if bst_str doesn't exist
        out_pool_str(pool, log, ctx.bst_str)?;
        writeln!(log, ".bst").unwrap();
        Ok(())
    })
}

pub(crate) fn hash_cite_confusion() {
    write_logs("Cite hash error");
    print_confusion();
}

pub(crate) fn bst_warn_print(ctx: &Bibtex<'_, '_>, pool: &StringPool) -> Result<(), BibtexError> {
    bst_ln_num_print(ctx, pool)?;
    mark_warning();
    Ok(())
}

pub fn eat_bst_print() {
    write_logs("Illegal end of style file in command: ");
}

pub fn id_scanning_confusion() {
    write_logs("Identifier scanning error");
    print_confusion();
}

pub(crate) fn bst_id_print(
    buffers: &GlobalBuffer,
    scan_result: ScanRes,
) -> Result<(), BibtexError> {
    let char = buffers.at_offset(BufTy::Base, 2) as char;
    match scan_result {
        ScanRes::IdNull => {
            write_logs(&format!("\"{}\" begins identifier, command: ", char));
            Ok(())
        }
        ScanRes::OtherCharAdjacent => {
            write_logs(&format!(
                "\"{}\" immediately follows identifier, command: ",
                char
            ));
            Ok(())
        }
        _ => {
            id_scanning_confusion();
            Err(BibtexError::Fatal)
        }
    }
}

pub fn bst_left_brace_print() {
    write_logs("\"{\" is missing in command: ");
}

pub fn bst_right_brace_print() {
    write_logs("\"}\" is missing in command: ");
}

pub(crate) fn bib_ln_num_print(pool: &StringPool, bibs: &BibData) -> Result<(), BibtexError> {
    write_logs(&format!("--line {} of file ", bibs.line_num()));
    print_bib_name(pool, bibs)
}

pub(crate) fn bib_err_print(
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    at_bib_command: bool,
) -> Result<(), BibtexError> {
    write_logs("-");
    bib_ln_num_print(pool, bibs)?;
    print_bad_input_line(buffers);
    print_skipping_whatever_remains();
    if at_bib_command {
        write_logs("command\n");
    } else {
        write_logs("entry\n");
    }
    Ok(())
}

pub(crate) fn bib_warn_print(pool: &StringPool, bibs: &BibData) -> Result<(), BibtexError> {
    bib_ln_num_print(pool, bibs)?;
    mark_warning();
    Ok(())
}

pub(crate) fn eat_bib_print(
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    at_bib_command: bool,
) -> Result<(), BibtexError> {
    write_logs("Illegal end of database file");
    bib_err_print(buffers, pool, bibs, at_bib_command)
}

pub(crate) fn bib_one_of_two_print(
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    char1: ASCIICode,
    char2: ASCIICode,
    at_bib_command: bool,
) -> Result<(), BibtexError> {
    write_logs(&format!(
        "I was expecting a `{}' or a `{}'",
        char1 as char, char2 as char
    ));
    bib_err_print(buffers, pool, bibs, at_bib_command)
}

pub(crate) fn bib_equals_sign_print(
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    at_bib_command: bool,
) -> Result<(), BibtexError> {
    write_logs("I was expecting an \"=\"");
    bib_err_print(buffers, pool, bibs, at_bib_command)
}

pub(crate) fn bib_unbalanced_braces_print(
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    at_bib_command: bool,
) -> Result<(), BibtexError> {
    write_logs("Unbalanced braces");
    bib_err_print(buffers, pool, bibs, at_bib_command)
}

pub(crate) fn macro_warn_print(buffers: &GlobalBuffer) {
    write_logs("Warning--string name \"");
    print_a_token(buffers);
    write_logs("\" is ");
}

pub(crate) fn bib_id_print(buffers: &GlobalBuffer, scan_res: ScanRes) -> Result<(), BibtexError> {
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

pub(crate) fn bib_cmd_confusion() {
    write_logs("Unknown database-file command");
    print_confusion();
}

pub fn cite_key_disappeared_confusion() {
    write_logs("A cite key disappeared");
    print_confusion();
}

pub(crate) fn rs_bad_cross_reference_print(
    pool: &StringPool,
    cites: &CiteInfo,
    cite_ptr: CiteNumber,
    s: StrNumber,
) -> Result<(), BibtexError> {
    write_logs("--entry \"");
    print_a_pool_str(cites.get_cite(cite_ptr), pool)?;
    write_logs("\"\nrefers to entry \"");
    print_a_pool_str(s, pool)?;
    write_logs("\"");
    Ok(())
}

pub(crate) fn print_missing_entry(pool: &StringPool, s: StrNumber) -> Result<(), BibtexError> {
    write_logs("Warning--I didn't find a database entry for \"");
    print_a_pool_str(s, pool)?;
    write_logs("\"\n");
    mark_warning();
    Ok(())
}

pub(crate) fn bst_mild_ex_warn_print(
    ctx: &ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
) -> Result<(), BibtexError> {
    if ctx.mess_with_entries {
        write_logs(" for entry ");
        print_a_pool_str(cites.get_cite(cites.ptr()), pool)?;
    }
    write_logs("\nwhile executing");
    bst_warn_print(ctx.glbl_ctx(), pool)
}

pub(crate) fn bst_cant_mess_with_entries_print(
    ctx: &ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
) -> Result<(), BibtexError> {
    write_logs("You can't mess with entries here");
    bst_ex_warn_print(ctx, pool, cites)
}

pub fn bst_1print_string_size_exceeded() {
    write_logs("Warning--you've exceeded ");
}

pub(crate) fn bst_2print_string_size_exceeded(
    ctx: &ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
) -> Result<(), BibtexError> {
    write_logs("-string-size,");
    bst_mild_ex_warn_print(ctx, pool, cites)?;
    write_logs("*Please notify the bibstyle designer*\n");
    Ok(())
}

pub(crate) fn braces_unbalanced_complaint(
    ctx: &ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
    pop_lit_var: StrNumber,
) -> Result<(), BibtexError> {
    write_logs("Warning--\"");
    print_a_pool_str(pop_lit_var, pool)?;
    write_logs("\" isn't a brace-balanced string");
    bst_mild_ex_warn_print(ctx, pool, cites)
}

pub(crate) fn rs_print_fn_class(hash: &HashData, fn_loc: HashPointer) {
    let ty = hash.ty(fn_loc);
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

pub(crate) fn bst_err_print_and_look_for_blank_line(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    write_logs("-");
    bst_ln_num_print(ctx, pool)?;
    print_bad_input_line(buffers);
    while buffers.init(BufTy::Base) != 0 {
        if !input_ln(ctx.bst_file.as_deref_mut(), buffers) {
            return Err(BibtexError::Recover);
        } else {
            ctx.bst_line_num += 1;
        }
    }
    buffers.set_offset(BufTy::Base, 2, buffers.init(BufTy::Base));
    Ok(())
}

pub(crate) fn already_seen_function_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
    hash: &HashData,
    seen_fn_loc: HashPointer,
) -> Result<(), BibtexError> {
    print_a_pool_str(hash.text(seen_fn_loc), pool)?;
    write_logs(" is already a type \"");
    rs_print_fn_class(hash, seen_fn_loc);
    write_logs("\" function name\n");
    bst_err_print_and_look_for_blank_line(ctx, buffers, pool)
}

pub(crate) fn rs_nonexistent_cross_reference_error(
    pool: &StringPool,
    cites: &CiteInfo,
    other: &OtherData,
    cite_ptr: CiteNumber,
    field_ptr: FieldLoc,
) -> Result<(), BibtexError> {
    write_logs("A bad cross reference-");
    rs_bad_cross_reference_print(pool, cites, cite_ptr, other.field(field_ptr))?;
    write_logs(", which doesn't exist\n");
    mark_error();
    Ok(())
}

pub(crate) fn output_bbl_line(ctx: &mut Bibtex<'_, '_>, buffers: &mut GlobalBuffer) {
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

pub(crate) fn skip_token_print(
    ctx: &Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    write_logs("-");
    bst_ln_num_print(ctx, pool)?;
    mark_error();

    Scan::new()
        .chars(b"}%")
        .class(LexClass::Whitespace)
        .scan_till(buffers, buffers.init(BufTy::Base));

    Ok(())
}

pub(crate) fn print_recursion_illegal(
    ctx: &Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    write_logs("Curse you, wizard, before you recurse me:\nfunction ");
    print_a_token(buffers);
    write_logs(" is illegal in its own definition\n");
    skip_token_print(ctx, buffers, pool)
}

pub(crate) fn skip_token_unknown_function_print(
    ctx: &Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    print_a_token(buffers);
    write_logs(" is an unknown function");
    skip_token_print(ctx, buffers, pool)
}

pub(crate) fn skip_illegal_stuff_after_token_print(
    ctx: &Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    write_logs(&format!(
        "\"{}\" can't follow a literal",
        buffers.at_offset(BufTy::Base, 2) as char
    ));
    skip_token_print(ctx, buffers, pool)
}

pub(crate) fn brace_lvl_one_letters_complaint(
    ctx: &ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
    str: StrNumber,
) -> Result<(), BibtexError> {
    write_logs("The format string \"");
    print_a_pool_str(str, pool)?;
    write_logs("\" has an illegal brace-level-1 letter");
    bst_ex_warn_print(ctx, pool, cites)?;
    Ok(())
}
