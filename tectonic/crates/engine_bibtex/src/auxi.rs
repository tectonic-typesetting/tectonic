use crate::{
    bibs::BibData,
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    cite::CiteInfo,
    exec::print_bst_name,
    hash::HashData,
    log::{
        aux_end1_err_print, aux_end2_err_print, aux_err_illegal_another_print,
        aux_err_no_right_brace_print, aux_err_print, aux_err_stuff_after_right_brace_print,
        aux_err_white_space_in_argument_print, hash_cite_confusion, log_pr_aux_name,
        log_pr_bst_name, print_a_pool_str, print_a_token, print_aux_name, print_bib_name,
        print_confusion, print_overflow, write_log_file, write_logs, AuxTy,
    },
    peekable::{peekable_open, PeekableInput},
    pool::StringPool,
    scan::Scan,
    Bibtex, BibtexError, GlobalItems, StrIlk, StrNumber,
};
use std::{ffi::CString, ptr::NonNull};
use tectonic_bridge_core::FileFormat;

const AUX_STACK_SIZE: usize = 20;

pub(crate) struct AuxFile {
    pub(crate) name: StrNumber,
    pub(crate) file: Box<PeekableInput>,
    pub(crate) line: i32,
}

pub(crate) struct AuxData {
    aux: Vec<AuxFile>,
}

impl AuxData {
    pub fn new() -> AuxData {
        AuxData { aux: Vec::new() }
    }

    pub fn push_file(&mut self, file: AuxFile) {
        self.aux.push(file);
    }

    pub fn pop_file(&mut self) -> (AuxFile, bool) {
        let out = self.aux.pop().unwrap();
        (out, self.aux.is_empty())
    }

    pub fn top_file(&self) -> &AuxFile {
        self.aux.last().unwrap()
    }

    pub fn top_file_mut(&mut self) -> &mut AuxFile {
        self.aux.last_mut().unwrap()
    }

    pub fn ptr(&self) -> usize {
        self.aux.len()
    }
}

fn aux_bib_data_command(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    bibs: &mut BibData,
    aux: &AuxData,
    pool: &mut StringPool,
    hash: &mut HashData,
) -> Result<(), BibtexError> {
    if ctx.bib_seen {
        aux_err_illegal_another_print(AuxTy::Data)?;
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    ctx.bib_seen = true;

    while buffers.at_offset(BufTy::Base, 2) != b'}' {
        buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
        let init = buffers.init(BufTy::Base);
        if !Scan::new()
            .chars(b"},")
            .class(LexClass::Whitespace)
            .scan_till(buffers, init)
        {
            aux_err_no_right_brace_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        if LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace {
            aux_err_white_space_in_argument_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        if buffers.init(BufTy::Base) > buffers.offset(BufTy::Base, 2) + 1
            && buffers.at_offset(BufTy::Base, 2) == b'}'
        {
            aux_err_stuff_after_right_brace_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        if bibs.ptr() == bibs.len() {
            bibs.grow();
        }

        let file = &buffers.buffer(BufTy::Base)
            [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
        let res = pool.lookup_str_insert(hash, file, StrIlk::BibFile)?;
        bibs.set_cur_bib(hash.text(res.loc));
        if res.exists {
            write_logs("This database file appears more than once: ");
            print_bib_name(pool, bibs)?;
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        let name = pool.get_str(bibs.cur_bib());
        let fname = CString::new(name).unwrap();
        let bib_in = peekable_open(ctx, &fname, FileFormat::Bib);
        if bib_in.is_null() {
            write_logs("I couldn't open database file ");
            print_bib_name(pool, bibs)?;
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }
        bibs.set_cur_bib_file(NonNull::new(bib_in));
        bibs.set_ptr(bibs.ptr() + 1);
    }

    Ok(())
}

fn aux_bib_style_command(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    aux: &AuxData,
    pool: &mut StringPool,
    hash: &mut HashData,
) -> Result<(), BibtexError> {
    if ctx.bst_seen {
        aux_err_illegal_another_print(AuxTy::Style)?;
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    ctx.bst_seen = true;

    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    let init = buffers.init(BufTy::Base);
    if !Scan::new()
        .chars(b"}")
        .class(LexClass::Whitespace)
        .scan_till(buffers, init)
    {
        aux_err_no_right_brace_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    if LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace {
        aux_err_white_space_in_argument_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    if buffers.init(BufTy::Base) > buffers.offset(BufTy::Base, 2) + 1 {
        aux_err_stuff_after_right_brace_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    let file = &buffers.buffer(BufTy::Base)
        [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
    let res = pool.lookup_str_insert(hash, file, StrIlk::BstFile)?;
    ctx.bst_str = hash.text(res.loc);
    if res.exists {
        write_logs("Already encountered style file");
        print_confusion();
        return Err(BibtexError::Fatal);
    }

    let name = pool.get_str(ctx.bst_str);
    let fname = CString::new(name).unwrap();
    let ptr = peekable_open(ctx, &fname, FileFormat::Bst);
    if ptr.is_null() {
        write_logs("I couldn't open style file ");
        print_bst_name(ctx, pool)?;
        ctx.bst_str = 0;
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    ctx.bst_file = NonNull::new(ptr);

    if ctx.config.verbose {
        write_logs("The style file: ");
        print_bst_name(ctx, pool)?;
    } else {
        write_log_file("The style file: ");
        log_pr_bst_name(ctx, pool)?;
    }

    Ok(())
}

fn aux_citation_command(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    aux: &AuxData,
    pool: &mut StringPool,
    hash: &mut HashData,
    cites: &mut CiteInfo,
) -> Result<(), BibtexError> {
    ctx.citation_seen = true;

    while buffers.at_offset(BufTy::Base, 2) != b'}' {
        buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

        let init = buffers.init(BufTy::Base);
        if !Scan::new()
            .chars(b"},")
            .class(LexClass::Whitespace)
            .scan_till(buffers, init)
        {
            aux_err_no_right_brace_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }
        if LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace {
            aux_err_white_space_in_argument_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }
        if buffers.init(BufTy::Base) > buffers.offset(BufTy::Base, 2) + 1
            && buffers.at_offset(BufTy::Base, 2) == b'}'
        {
            aux_err_stuff_after_right_brace_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        if buffers.offset(BufTy::Base, 2) - buffers.offset(BufTy::Base, 1) == 1
            && buffers.at_offset(BufTy::Base, 1) == b'*'
        {
            if ctx.all_entries {
                write_logs("Multiple inclusions of entire database\n");
                aux_err_print(buffers, aux, pool)?;
                return Ok(());
            } else {
                ctx.all_entries = true;
                cites.set_all_marker(cites.ptr());
                continue;
            }
        }

        let idx = buffers.offset(BufTy::Base, 1);
        buffers.copy_within(
            BufTy::Base,
            BufTy::Ex,
            idx,
            idx,
            buffers.offset(BufTy::Base, 2) - idx,
        );
        let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
        let lc_cite = &mut buffers.buffer_mut(BufTy::Ex)[range];
        lc_cite.make_ascii_lowercase();

        let lc_res = pool.lookup_str_insert(hash, lc_cite, StrIlk::LcCite)?;
        if lc_res.exists {
            let cite = &buffers.buffer(BufTy::Base)
                [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
            let uc_res = pool.lookup_str(hash, cite, StrIlk::Cite);
            if !uc_res.exists {
                write_logs("Case mismatch error between cite keys ");
                print_a_token(buffers);
                write_logs(" and ");
                print_a_pool_str(
                    cites.get_cite(hash.ilk_info(hash.ilk_info(lc_res.loc) as usize) as usize),
                    pool,
                )?;
                write_logs("\n");
                aux_err_print(buffers, aux, pool)?;
                return Ok(());
            }
        } else {
            let cite = &buffers.buffer(BufTy::Base)
                [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
            let uc_res = pool.lookup_str_insert(hash, cite, StrIlk::Cite)?;
            if uc_res.exists {
                hash_cite_confusion();
                return Err(BibtexError::Fatal);
            }

            if cites.ptr() == cites.len() {
                cites.grow();
            }

            cites.set_cite(cites.ptr(), hash.text(uc_res.loc));
            hash.set_ilk_info(uc_res.loc, cites.ptr() as i32);
            hash.set_ilk_info(lc_res.loc, uc_res.loc as i32);
            cites.set_ptr(cites.ptr() + 1);
        }
    }

    Ok(())
}

fn aux_input_command(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    aux: &mut AuxData,
    pool: &mut StringPool,
    hash: &mut HashData,
) -> Result<(), BibtexError> {
    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

    let init = buffers.init(BufTy::Base);
    if !Scan::new()
        .chars(b"}")
        .class(LexClass::Whitespace)
        .scan_till(buffers, init)
    {
        aux_err_no_right_brace_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    if LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace {
        aux_err_white_space_in_argument_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    if buffers.init(BufTy::Base) > buffers.offset(BufTy::Base, 2) + 1 {
        aux_err_stuff_after_right_brace_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    if aux.ptr() == AUX_STACK_SIZE {
        print_a_token(buffers);
        write_logs(": ");
        print_overflow();
        write_logs(&format!("auxiliary file depth {}\n", AUX_STACK_SIZE));
        return Err(BibtexError::Fatal);
    }

    let aux_ext = pool.get_str(ctx.s_aux_extension);
    let aux_extension_ok = !((buffers.offset(BufTy::Base, 2) - buffers.offset(BufTy::Base, 1)
        < aux_ext.len())
        || aux_ext
            != &buffers.buffer(BufTy::Base)
                [buffers.offset(BufTy::Base, 2) - aux_ext.len()..buffers.offset(BufTy::Base, 2)]);

    if !aux_extension_ok {
        print_a_token(buffers);
        write_logs(" has a wrong extension");
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    let file = &buffers.buffer(BufTy::Base)
        [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
    let res = pool.lookup_str_insert(hash, file, StrIlk::AuxFile)?;
    if res.exists {
        write_logs("Already encountered file ");
        print_aux_name(pool, hash.text(res.loc))?;
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    let name = pool.get_str(hash.text(res.loc));
    let fname = CString::new(name).unwrap();
    let file = PeekableInput::open(ctx, &fname, FileFormat::Tex);
    match file {
        Err(_) => {
            write_logs("I couldn't open auxiliary file ");
            print_aux_name(pool, hash.text(res.loc))?;
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }
        Ok(file) => {
            aux.push_file(AuxFile {
                name: hash.text(res.loc),
                file,
                line: 0,
            });
        }
    }

    write_logs(&format!("A level-{} auxiliary file: ", aux.ptr() - 1));
    log_pr_aux_name(aux, pool)?;

    Ok(())
}

pub(crate) fn get_aux_command_and_process(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    globals.buffers.set_offset(BufTy::Base, 2, 0);
    let init = globals.buffers.init(BufTy::Base);
    if !Scan::new().chars(b"{").scan_till(globals.buffers, init) {
        return Ok(());
    }

    let line = &globals.buffers.buffer(BufTy::Base)
        [globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2)];
    let res = globals
        .pool
        .lookup_str(globals.hash, line, StrIlk::AuxCommand);

    if res.exists {
        match globals.hash.ilk_info(res.loc) {
            0 => aux_bib_data_command(
                ctx,
                globals.buffers,
                globals.bibs,
                globals.aux,
                globals.pool,
                globals.hash,
            )?,
            1 => aux_bib_style_command(
                ctx,
                globals.buffers,
                globals.aux,
                globals.pool,
                globals.hash,
            )?,
            2 => aux_citation_command(
                ctx,
                globals.buffers,
                globals.aux,
                globals.pool,
                globals.hash,
                globals.cites,
            )?,
            3 => aux_input_command(
                ctx,
                globals.buffers,
                globals.aux,
                globals.pool,
                globals.hash,
            )?,
            _ => {
                write_logs("Unknown auxiliary-file command");
                print_confusion();
                return Err(BibtexError::Fatal);
            }
        }
    }
    Ok(())
}

pub(crate) fn pop_the_aux_stack(ctx: &mut Bibtex<'_, '_>, aux: &mut AuxData) -> Option<StrNumber> {
    let (file, last) = aux.pop_file();
    file.file.close(ctx).unwrap();
    if last {
        Some(file.name)
    } else {
        None
    }
}

pub(crate) fn last_check_for_aux_errors(
    ctx: &mut Bibtex<'_, '_>,
    pool: &StringPool,
    cites: &mut CiteInfo,
    bibs: &BibData,
    last_aux: StrNumber,
) -> Result<(), BibtexError> {
    cites.set_num_cites(cites.ptr());
    ctx.num_bib_files = bibs.ptr();
    if !ctx.citation_seen {
        aux_end1_err_print();
        write_logs("\\citation commands");
        aux_end2_err_print(pool, last_aux)?;
    } else if cites.num_cites() == 0 && !ctx.all_entries {
        aux_end1_err_print();
        write_logs("cite keys");
        aux_end2_err_print(pool, last_aux)?;
    }

    if !ctx.bib_seen {
        aux_end1_err_print();
        write_logs("\\bibdata command");
        aux_end2_err_print(pool, last_aux)?;
    } else if ctx.num_bib_files == 0 {
        aux_end1_err_print();
        write_logs("database files");
        aux_end2_err_print(pool, last_aux)?;
    }

    if !ctx.bst_seen {
        aux_end1_err_print();
        write_logs("\\bibstyle command");
        aux_end2_err_print(pool, last_aux)?;
    } else if ctx.bst_str == 0 {
        aux_end1_err_print();
        write_logs("style file");
        aux_end2_err_print(pool, last_aux)?;
    }

    Ok(())
}
