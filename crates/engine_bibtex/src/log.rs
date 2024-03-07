use crate::{
    auxi::AuxData,
    bibs::{BibCommand, BibData},
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    cite::CiteInfo,
    exec::{bst_ex_warn_print, bst_ln_num_print, ExecCtx},
    hash::{BstFn, HashData, HashExtra},
    other::OtherData,
    peekable::input_ln,
    pool::StringPool,
    scan::{Scan, ScanRes},
    ASCIICode, Bibtex, BibtexError, CiteNumber, FieldLoc, HashPointer, StrNumber,
};
use std::{ffi::CStr, io::Write, slice};
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

pub(crate) fn bib_close_log(ctx: &mut Bibtex<'_, '_>) {
    if let Some(log) = ctx.logs.file.take() {
        ctx.engine.output_close(log);
    }
}

pub fn print_overflow(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("Sorry---you've exceeded BibTeX's ");
    ctx.mark_fatal();
}

pub fn print_confusion(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("---this can't happen\n*Please notify the Tectonic maintainer*\n");
    ctx.mark_fatal();
}

pub(crate) fn out_token(handle: &mut OutputHandle, buffers: &GlobalBuffer) {
    let bytes = buffers.buffer(BufTy::Base);
    let start = buffers.offset(BufTy::Base, 1);
    let end = buffers.offset(BufTy::Base, 2);
    handle.write_all(&bytes[start..end]).unwrap();
}

pub(crate) fn print_a_token(ctx: &mut Bibtex<'_, '_>, buffers: &GlobalBuffer) {
    out_token(ctx.engine.get_output(ctx.logs.stdout.unwrap()), buffers);
    out_token(ctx.engine.get_output(ctx.logs.file.unwrap()), buffers);
}

pub(crate) fn print_bad_input_line(ctx: &mut Bibtex<'_, '_>, buffers: &GlobalBuffer) {
    ctx.write_logs(" : ");

    let offset2 = buffers.offset(BufTy::Base, 2);

    let slice = &buffers.buffer(BufTy::Base)[0..offset2];

    for code in slice {
        if LexClass::of(*code) == LexClass::Whitespace {
            ctx.write_logs(" ");
        } else {
            ctx.write_logs(slice::from_ref(code))
        }
    }
    ctx.write_logs("\n : ");
    let str = (0..offset2).map(|_| ' ').collect::<String>();
    ctx.write_logs(&str);

    let last = buffers.init(BufTy::Base);
    if offset2 < last {
        let slice = &buffers.buffer(BufTy::Base)[offset2..last];
        for code in slice {
            if LexClass::of(*code) == LexClass::Whitespace {
                ctx.write_logs(" ");
            } else {
                ctx.write_logs(slice::from_ref(code));
            }
        }
    }

    ctx.write_logs("\n");

    if !slice
        .iter()
        .any(|c| LexClass::of(*c) != LexClass::Whitespace)
    {
        ctx.write_logs("(Error may have been on previous line)\n");
    }

    ctx.mark_error();
}

pub(crate) fn print_skipping_whatever_remains(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("I'm skipping whatever remains of this ");
}

pub(crate) fn out_pool_str(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    s: StrNumber,
) -> Result<(), BibtexError> {
    let str = pool.try_get_str(s);
    if let Ok(str) = str {
        ctx.write_log_file(str);
        Ok(())
    } else {
        ctx.write_logs(&format!("Illegal string number: {}", s));
        print_confusion(ctx);
        Err(BibtexError::Fatal)
    }
}

pub(crate) fn print_a_pool_str(
    ctx: &mut Bibtex<'_, '_>,
    s: StrNumber,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    let str = pool.try_get_str(s);
    if let Ok(str) = str {
        ctx.write_logs(str);
        Ok(())
    } else {
        ctx.write_logs(&format!("Illegal string number: {}", s));
        print_confusion(ctx);
        Err(BibtexError::Fatal)
    }
}

pub fn sam_wrong_file_name_print(ctx: &mut Bibtex<'_, '_>, file: &CStr) {
    ctx.write_stdout(&format!(
        "I couldn't open file name `{}`",
        file.to_str().unwrap()
    ));
}

pub(crate) fn print_aux_name(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    name: StrNumber,
) -> Result<(), BibtexError> {
    print_a_pool_str(ctx, name, pool)?;
    ctx.write_logs("\n");
    Ok(())
}

pub(crate) fn log_pr_aux_name(
    ctx: &mut Bibtex<'_, '_>,
    aux: &AuxData,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    out_pool_str(ctx, pool, aux.top_file().name)?;
    ctx.write_log_file("\n");
    Ok(())
}

pub(crate) fn aux_err_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &GlobalBuffer,
    aux: &AuxData,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    ctx.write_logs(&format!("---line {} of file ", aux.top_file().line));
    print_aux_name(ctx, pool, aux.top_file().name)?;
    print_bad_input_line(ctx, buffers);
    print_skipping_whatever_remains(ctx);
    ctx.write_logs("command\n");
    Ok(())
}

pub(crate) enum AuxTy {
    Data,
    Style,
}

pub(crate) fn aux_err_illegal_another_print(
    ctx: &mut Bibtex<'_, '_>,
    cmd: AuxTy,
) -> Result<(), BibtexError> {
    ctx.write_logs("Illegal, another \\bib");
    match cmd {
        AuxTy::Data => ctx.write_logs("data"),
        AuxTy::Style => ctx.write_logs("style"),
    }
    ctx.write_logs(" command");
    Ok(())
}

pub fn aux_err_no_right_brace_print(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("No \"}\"");
}

pub fn aux_err_stuff_after_right_brace_print(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("Stuff after \"}\"");
}

pub fn aux_err_white_space_in_argument_print(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("White space in argument");
}

pub fn aux_end1_err_print(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("I found no ");
}

pub(crate) fn aux_end2_err_print(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    name: StrNumber,
) -> Result<(), BibtexError> {
    ctx.write_logs("---while reading file ");
    print_aux_name(ctx, pool, name)?;
    ctx.mark_error();
    Ok(())
}

pub(crate) fn print_bib_name(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    name: StrNumber,
) -> Result<(), BibtexError> {
    print_a_pool_str(ctx, name, pool)?;
    let res = pool
        .try_get_str(name)
        .map_err(|_| BibtexError::Fatal)
        .map(|str| str.ends_with(b".bib"))?;
    if !res {
        ctx.write_logs(".bib");
    }
    ctx.write_logs("\n");
    Ok(())
}

pub(crate) fn log_pr_bib_name(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    name: StrNumber,
) -> Result<(), BibtexError> {
    out_pool_str(ctx, pool, name)?;
    let res = pool
        .try_get_str(name)
        .map(|str| str.ends_with(b".bib"))
        .map_err(|_| BibtexError::Fatal)?;
    if !res {
        ctx.write_log_file(".bib");
    }
    ctx.write_log_file("\n");
    Ok(())
}

pub(crate) fn log_pr_bst_name(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    // TODO: This call can panic if bst_str doesn't exist
    out_pool_str(ctx, pool, ctx.bst.as_ref().unwrap().name)?;
    ctx.write_log_file(".bst\n");
    Ok(())
}

pub(crate) fn hash_cite_confusion(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("Cite hash error");
    print_confusion(ctx);
}

pub(crate) fn bst_warn_print(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    bst_ln_num_print(ctx, pool)?;
    ctx.mark_warning();
    Ok(())
}

pub fn eat_bst_print(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("Illegal end of style file in command: ");
}

pub fn id_scanning_confusion(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("Identifier scanning error");
    print_confusion(ctx);
}

pub(crate) fn bst_id_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &GlobalBuffer,
    scan_result: ScanRes,
) -> Result<(), BibtexError> {
    let char = buffers.at_offset(BufTy::Base, 2) as char;
    match scan_result {
        ScanRes::IdNull => {
            ctx.write_logs(&format!("\"{}\" begins identifier, command: ", char));
            Ok(())
        }
        ScanRes::OtherCharAdjacent => {
            ctx.write_logs(&format!(
                "\"{}\" immediately follows identifier, command: ",
                char
            ));
            Ok(())
        }
        _ => {
            id_scanning_confusion(ctx);
            Err(BibtexError::Fatal)
        }
    }
}

pub fn bst_left_brace_print(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("\"{\" is missing in command: ");
}

pub fn bst_right_brace_print(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("\"}\" is missing in command: ");
}

pub(crate) fn bib_ln_num_print(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    bibs: &BibData,
) -> Result<(), BibtexError> {
    ctx.write_logs(&format!("--line {} of file ", bibs.top_file().line));
    print_bib_name(ctx, pool, bibs.top_file().name)
}

pub(crate) fn bib_err_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    bib_command: Option<BibCommand>,
) -> Result<(), BibtexError> {
    ctx.write_logs("-");
    bib_ln_num_print(ctx, pool, bibs)?;
    print_bad_input_line(ctx, buffers);
    print_skipping_whatever_remains(ctx);
    if bib_command.is_some() {
        ctx.write_logs("command\n");
    } else {
        ctx.write_logs("entry\n");
    }
    Ok(())
}

pub(crate) fn bib_warn_print(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    bibs: &BibData,
) -> Result<(), BibtexError> {
    bib_ln_num_print(ctx, pool, bibs)?;
    ctx.mark_warning();
    Ok(())
}

pub(crate) fn eat_bib_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    bib_command: Option<BibCommand>,
) -> Result<(), BibtexError> {
    ctx.write_logs("Illegal end of database file");
    bib_err_print(ctx, buffers, pool, bibs, bib_command)
}

pub(crate) fn bib_one_of_two_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    char1: ASCIICode,
    char2: ASCIICode,
    bib_command: Option<BibCommand>,
) -> Result<(), BibtexError> {
    ctx.write_logs(&format!(
        "I was expecting a `{}' or a `{}'",
        char1 as char, char2 as char
    ));
    bib_err_print(ctx, buffers, pool, bibs, bib_command)
}

pub(crate) fn bib_equals_sign_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    bib_command: Option<BibCommand>,
) -> Result<(), BibtexError> {
    ctx.write_logs("I was expecting an \"=\"");
    bib_err_print(ctx, buffers, pool, bibs, bib_command)
}

pub(crate) fn bib_unbalanced_braces_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &GlobalBuffer,
    pool: &StringPool,
    bibs: &BibData,
    bib_command: Option<BibCommand>,
) -> Result<(), BibtexError> {
    ctx.write_logs("Unbalanced braces");
    bib_err_print(ctx, buffers, pool, bibs, bib_command)
}

pub(crate) fn macro_warn_print(ctx: &mut Bibtex<'_, '_>, buffers: &GlobalBuffer) {
    ctx.write_logs("Warning--string name \"");
    print_a_token(ctx, buffers);
    ctx.write_logs("\" is ");
}

pub(crate) fn bib_id_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &GlobalBuffer,
    scan_res: ScanRes,
) -> Result<(), BibtexError> {
    match scan_res {
        ScanRes::IdNull => {
            ctx.write_logs("You're missing ");
            Ok(())
        }
        ScanRes::OtherCharAdjacent => {
            let char = buffers.at_offset(BufTy::Base, 2);
            ctx.write_logs(&format!("\"{}\" immediately follows ", char));
            Ok(())
        }
        _ => {
            id_scanning_confusion(ctx);
            Err(BibtexError::Fatal)
        }
    }
}

pub fn cite_key_disappeared_confusion(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("A cite key disappeared");
    print_confusion(ctx);
}

pub(crate) fn bad_cross_reference_print(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
    cite_ptr: CiteNumber,
    s: StrNumber,
) -> Result<(), BibtexError> {
    ctx.write_logs("--entry \"");
    print_a_pool_str(ctx, cites.get_cite(cite_ptr), pool)?;
    ctx.write_logs("\"\nrefers to entry \"");
    print_a_pool_str(ctx, s, pool)?;
    ctx.write_logs("\"");
    Ok(())
}

pub(crate) fn print_missing_entry(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    s: StrNumber,
) -> Result<(), BibtexError> {
    ctx.write_logs("Warning--I didn't find a database entry for \"");
    print_a_pool_str(ctx, s, pool)?;
    ctx.write_logs("\"\n");
    ctx.mark_warning();
    Ok(())
}

pub(crate) fn bst_mild_ex_warn_print(
    ctx: &mut ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
) -> Result<(), BibtexError> {
    if ctx.mess_with_entries {
        ctx.write_logs(" for entry ");
        print_a_pool_str(ctx, cites.get_cite(cites.ptr()), pool)?;
    }
    ctx.write_logs("\nwhile executing");
    bst_warn_print(ctx, pool)
}

pub(crate) fn bst_cant_mess_with_entries_print(
    ctx: &mut ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
) -> Result<(), BibtexError> {
    ctx.write_logs("You can't mess with entries here");
    bst_ex_warn_print(ctx, pool, cites)
}

pub fn bst_1print_string_size_exceeded(ctx: &mut Bibtex<'_, '_>) {
    ctx.write_logs("Warning--you've exceeded ");
}

pub(crate) fn bst_2print_string_size_exceeded(
    ctx: &mut ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
) -> Result<(), BibtexError> {
    ctx.write_logs("-string-size,");
    bst_mild_ex_warn_print(ctx, pool, cites)?;
    ctx.write_logs("*Please notify the bibstyle designer*\n");
    Ok(())
}

pub(crate) fn braces_unbalanced_complaint(
    ctx: &mut ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
    pop_lit_var: StrNumber,
) -> Result<(), BibtexError> {
    ctx.write_logs("Warning--\"");
    print_a_pool_str(ctx, pop_lit_var, pool)?;
    ctx.write_logs("\" isn't a brace-balanced string");
    bst_mild_ex_warn_print(ctx, pool, cites)
}

pub(crate) fn print_fn_class(ctx: &mut Bibtex<'_, '_>, hash: &HashData, fn_loc: HashPointer) {
    match hash.node(fn_loc).extra {
        HashExtra::BstFn(BstFn::Builtin(_)) => ctx.write_logs("built-in"),
        HashExtra::BstFn(BstFn::Wizard(_)) => ctx.write_logs("wizard-defined"),
        HashExtra::Integer(_) => ctx.write_logs("integer-literal"),
        HashExtra::Text => ctx.write_logs("string-literal"),
        HashExtra::BstFn(BstFn::Field(_)) => ctx.write_logs("field"),
        HashExtra::BstFn(BstFn::IntEntry(_)) => ctx.write_logs("integer-entry-variable"),
        HashExtra::BstFn(BstFn::StrEntry(_)) => ctx.write_logs("string-entry-variable"),
        HashExtra::BstFn(BstFn::IntGlbl(_)) => ctx.write_logs("integer-global-variable"),
        HashExtra::BstFn(BstFn::StrGlbl(_)) => ctx.write_logs("string-global-variable"),
        _ => ctx.write_logs("unknown-fn"),
    }
}

pub(crate) fn bst_err_print_and_look_for_blank_line(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    ctx.write_logs("-");
    bst_ln_num_print(ctx, pool)?;
    print_bad_input_line(ctx, buffers);
    while buffers.init(BufTy::Base) != 0 {
        if !input_ln(ctx.engine, &mut ctx.bst.as_mut().unwrap().file, buffers) {
            return Err(BibtexError::Recover);
        } else {
            ctx.bst.as_mut().unwrap().line += 1;
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
    print_a_pool_str(ctx, hash.text(seen_fn_loc), pool)?;
    ctx.write_logs(" is already a type \"");
    print_fn_class(ctx, hash, seen_fn_loc);
    ctx.write_logs("\" function name\n");
    bst_err_print_and_look_for_blank_line(ctx, buffers, pool)
}

pub(crate) fn nonexistent_cross_reference_error(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
    other: &OtherData,
    cite_ptr: CiteNumber,
    field_ptr: FieldLoc,
) -> Result<(), BibtexError> {
    ctx.write_logs("A bad cross reference-");
    bad_cross_reference_print(ctx, pool, cites, cite_ptr, other.field(field_ptr))?;
    ctx.write_logs(", which doesn't exist\n");
    ctx.mark_error();
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
        ctx.engine
            .get_output(ctx.bbl_file.unwrap())
            .write_all(slice)
            .unwrap();
    }
    writeln!(ctx.engine.get_output(ctx.bbl_file.unwrap())).unwrap();
    ctx.bbl_line_num += 1;
    buffers.set_init(BufTy::Out, 0);
}

pub(crate) fn skip_token_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    ctx.write_logs("-");
    bst_ln_num_print(ctx, pool)?;
    ctx.mark_error();

    Scan::new()
        .chars(&[b'}', b'%'])
        .class(LexClass::Whitespace)
        .scan_till(buffers, buffers.init(BufTy::Base));

    Ok(())
}

pub(crate) fn print_recursion_illegal(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    ctx.write_logs("Curse you, wizard, before you recurse me:\nfunction ");
    print_a_token(ctx, buffers);
    ctx.write_logs(" is illegal in its own definition\n");
    skip_token_print(ctx, buffers, pool)
}

pub(crate) fn skip_token_unknown_function_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    print_a_token(ctx, buffers);
    ctx.write_logs(" is an unknown function");
    skip_token_print(ctx, buffers, pool)
}

pub(crate) fn skip_illegal_stuff_after_token_print(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
) -> Result<(), BibtexError> {
    ctx.write_logs(&format!(
        "\"{}\" can't follow a literal",
        buffers.at_offset(BufTy::Base, 2) as char
    ));
    skip_token_print(ctx, buffers, pool)
}

pub(crate) fn brace_lvl_one_letters_complaint(
    ctx: &mut ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
    str: StrNumber,
) -> Result<(), BibtexError> {
    ctx.write_logs("The format string \"");
    print_a_pool_str(ctx, str, pool)?;
    ctx.write_logs("\" has an illegal brace-level-1 letter");
    bst_ex_warn_print(ctx, pool, cites)?;
    Ok(())
}
