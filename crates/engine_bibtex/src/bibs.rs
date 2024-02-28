use crate::{
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    cite::add_database_cite,
    hash::{BstFn, HashData, HashExtra},
    log::{
        bib_equals_sign_print, bib_err_print, bib_id_print, bib_one_of_two_print, bib_warn_print,
        cite_key_disappeared_confusion, eat_bib_print, hash_cite_confusion, print_a_token,
        print_confusion,
    },
    peekable::input_ln,
    pool::StringPool,
    scan::{scan_and_store_the_field_value_and_eat_white, scan_identifier, Scan, ScanRes},
    BibNumber, Bibtex, BibtexError, File, GlobalItems, HashPointer, StrIlk, StrNumber,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum BibCommand {
    Comment,
    Preamble,
    String,
}

pub(crate) struct BibData {
    bibs: Vec<File>,
    preamble: Vec<StrNumber>,
}

impl BibData {
    pub fn new() -> BibData {
        BibData {
            bibs: Vec::new(),
            preamble: Vec::new(),
        }
    }

    pub fn top_file(&self) -> &File {
        self.bibs.last().unwrap()
    }

    pub fn top_file_mut(&mut self) -> &mut File {
        self.bibs.last_mut().unwrap()
    }

    pub fn push_file(&mut self, file: File) {
        self.bibs.push(file);
    }

    pub fn pop_file(&mut self) -> File {
        self.bibs.pop().unwrap()
    }

    pub fn add_preamble(&mut self, s: StrNumber) {
        self.preamble.push(s);
    }

    pub fn preamble_len(&self) -> usize {
        self.preamble.len()
    }

    pub fn preamble(&self) -> &[StrNumber] {
        &self.preamble
    }

    pub fn len(&self) -> BibNumber {
        self.bibs.len()
    }
}

pub(crate) fn eat_bib_white_space(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    bibs: &mut BibData,
) -> bool {
    let mut init = buffers.init(BufTy::Base);
    while !Scan::new()
        .not_class(LexClass::Whitespace)
        .scan_till(buffers, init)
    {
        if !input_ln(ctx.engine, &mut bibs.top_file_mut().file, buffers) {
            return false;
        }

        bibs.top_file_mut().line += 1;
        buffers.set_offset(BufTy::Base, 2, 0);
        init = buffers.init(BufTy::Base);
    }
    true
}

pub(crate) fn compress_bib_white(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
    bibs: &mut BibData,
    at_bib_command: bool,
) -> Result<bool, BibtexError> {
    if buffers.offset(BufTy::Ex, 1) == buffers.len() {
        ctx.write_log_file("Field filled up at ' ', reallocating.\n");
        buffers.grow_all();
    }

    buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b' ');
    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
    let mut last = buffers.init(BufTy::Base);
    while !Scan::new()
        .not_class(LexClass::Whitespace)
        .scan_till(buffers, last)
    {
        let res = !input_ln(ctx.engine, &mut bibs.top_file_mut().file, buffers);

        if res {
            return eat_bib_print(ctx, buffers, pool, bibs, at_bib_command).map(|_| false);
        }

        bibs.top_file_mut().line += 1;
        buffers.set_offset(BufTy::Base, 2, 0);
        last = buffers.init(BufTy::Base);
    }

    Ok(true)
}

// TODO: This function is unnecessarily complicated
//       - Most at_bib_command uses are statically known
//       - tied to that, command_num is only used when at_bib_command is true
//       - There's some messed up control flow that's porting weird `goto` style, can probably be simplified
pub(crate) fn get_bib_command_or_entry_and_process(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
    cur_macro_loc: &mut HashPointer,
    field_name_loc: &mut HashPointer,
) -> Result<(), BibtexError> {
    let mut at_bib_command = false;

    let mut init = globals.buffers.init(BufTy::Base);
    while !Scan::new().chars(&[b'@']).scan_till(globals.buffers, init) {
        if !input_ln(
            ctx.engine,
            &mut globals.bibs.top_file_mut().file,
            globals.buffers,
        ) {
            return Ok(());
        }

        globals.bibs.top_file_mut().line += 1;
        globals.buffers.set_offset(BufTy::Base, 2, 0);
        init = globals.buffers.init(BufTy::Base);
    }

    if globals.buffers.at_offset(BufTy::Base, 2) != b'@' {
        ctx.write_logs("An \"@\" disappeared");
        print_confusion(ctx);
        return Err(BibtexError::Fatal);
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
        eat_bib_print(
            ctx,
            globals.buffers,
            globals.pool,
            globals.bibs,
            at_bib_command,
        )?;
        return Ok(());
    }

    let scan_res = scan_identifier(globals.buffers, b'{', b'(', b'(');
    match scan_res {
        ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
        _ => {
            bib_id_print(ctx, globals.buffers, scan_res)?;
            ctx.write_logs("an entry type");
            bib_err_print(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                at_bib_command,
            )?;
            return Ok(());
        }
    }

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let bib_cmd = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
    bib_cmd.make_ascii_lowercase();

    let res = globals
        .pool
        .lookup_str(globals.hash, bib_cmd, StrIlk::BibCommand);

    let mut lc_cite_loc = 0;

    if res.exists {
        let HashExtra::BibCommand(cmd) = globals.hash.node(res.loc).extra else {
            panic!("BibCommand lookup didn't have BibCommand extra");
        };

        at_bib_command = true;
        match cmd {
            BibCommand::Comment => (),
            BibCommand::Preamble => {
                if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
                    eat_bib_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }

                let right_outer_delim = match globals.buffers.at_offset(BufTy::Base, 2) {
                    b'{' => b'}',
                    b'(' => b')',
                    _ => {
                        bib_one_of_two_print(
                            ctx,
                            globals.buffers,
                            globals.pool,
                            globals.bibs,
                            b'{',
                            b'(',
                            at_bib_command,
                        )?;
                        return Ok(());
                    }
                };

                globals.buffers.set_offset(
                    BufTy::Base,
                    2,
                    globals.buffers.offset(BufTy::Base, 2) + 1,
                );

                if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
                    eat_bib_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }

                if !scan_and_store_the_field_value_and_eat_white(
                    ctx,
                    globals.buffers,
                    globals.hash,
                    globals.pool,
                    globals.bibs,
                    globals.other,
                    globals.cites,
                    true,
                    at_bib_command,
                    cmd,
                    Some(&mut lc_cite_loc),
                    *cur_macro_loc,
                    right_outer_delim,
                    *field_name_loc,
                )? {
                    return Ok(());
                }

                if globals.buffers.at_offset(BufTy::Base, 2) != right_outer_delim {
                    ctx.write_logs(&format!(
                        "Missing \"{}\" in preamble command",
                        right_outer_delim
                    ));
                    bib_err_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }
                globals.buffers.set_offset(
                    BufTy::Base,
                    2,
                    globals.buffers.offset(BufTy::Base, 2) + 1,
                );
            }
            BibCommand::String => {
                if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
                    eat_bib_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }

                let right_outer_delim = match globals.buffers.at_offset(BufTy::Base, 2) {
                    b'{' => b'}',
                    b'(' => b')',
                    _ => {
                        bib_one_of_two_print(
                            ctx,
                            globals.buffers,
                            globals.pool,
                            globals.bibs,
                            b'{',
                            b'(',
                            at_bib_command,
                        )?;
                        return Ok(());
                    }
                };

                globals.buffers.set_offset(
                    BufTy::Base,
                    2,
                    globals.buffers.offset(BufTy::Base, 2) + 1,
                );

                if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
                    eat_bib_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }

                let scan_res = scan_identifier(globals.buffers, b'=', b'=', b'=');
                match scan_res {
                    ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
                    _ => {
                        bib_id_print(ctx, globals.buffers, scan_res)?;
                        ctx.write_logs("a string name");
                        bib_err_print(
                            ctx,
                            globals.buffers,
                            globals.pool,
                            globals.bibs,
                            at_bib_command,
                        )?;
                        return Ok(());
                    }
                }

                let range =
                    globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
                let bib_macro = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
                bib_macro.make_ascii_lowercase();

                // let text = globals.hash.text(res.loc);
                let res = globals.pool.lookup_str_insert(
                    ctx,
                    globals.hash,
                    bib_macro,
                    HashExtra::Macro(0),
                )?;
                // TODO: Insert overwriting?
                globals.hash.node_mut(res.loc).extra = HashExtra::Macro(globals.hash.text(res.loc));
                *cur_macro_loc = res.loc;

                if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
                    eat_bib_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }

                if globals.buffers.at_offset(BufTy::Base, 2) != b'=' {
                    bib_equals_sign_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }

                globals.buffers.set_offset(
                    BufTy::Base,
                    2,
                    globals.buffers.offset(BufTy::Base, 2) + 1,
                );

                if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
                    eat_bib_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }

                if !scan_and_store_the_field_value_and_eat_white(
                    ctx,
                    globals.buffers,
                    globals.hash,
                    globals.pool,
                    globals.bibs,
                    globals.other,
                    globals.cites,
                    true,
                    at_bib_command,
                    cmd,
                    Some(&mut lc_cite_loc),
                    *cur_macro_loc,
                    right_outer_delim,
                    *field_name_loc,
                )? {
                    return Ok(());
                }

                if globals.buffers.at_offset(BufTy::Base, 2) != right_outer_delim {
                    ctx.write_logs(&format!(
                        "Missing \"{}\" in string command",
                        right_outer_delim
                    ));
                    bib_err_print(
                        ctx,
                        globals.buffers,
                        globals.pool,
                        globals.bibs,
                        at_bib_command,
                    )?;
                    return Ok(());
                }

                globals.buffers.set_offset(
                    BufTy::Base,
                    2,
                    globals.buffers.offset(BufTy::Base, 2) + 1,
                );
            }
        }
        return Ok(());
    }

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
    let bst_res = globals.pool.lookup_str(globals.hash, bst_fn, StrIlk::BstFn);

    let type_exists = if bst_res.exists {
        matches!(
            globals.hash.node(bst_res.loc).extra,
            HashExtra::BstFn(BstFn::Wizard(_))
        )
    } else {
        false
    };

    if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
        eat_bib_print(
            ctx,
            globals.buffers,
            globals.pool,
            globals.bibs,
            at_bib_command,
        )?;
        return Ok(());
    }

    let right_outer_delim = match globals.buffers.at_offset(BufTy::Base, 2) {
        b'{' => b'}',
        b'(' => b')',
        _ => {
            bib_one_of_two_print(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                b'{',
                b'(',
                at_bib_command,
            )?;
            return Ok(());
        }
    };

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
        eat_bib_print(
            ctx,
            globals.buffers,
            globals.pool,
            globals.bibs,
            at_bib_command,
        )?;
        return Ok(());
    }

    let init = globals.buffers.init(BufTy::Base);
    Scan::new()
        .chars(if right_outer_delim == b')' {
            &[b',']
        } else {
            &[b',', b'}']
        })
        .class(LexClass::Whitespace)
        .scan_till(globals.buffers, init);

    let start = globals.buffers.offset(BufTy::Base, 1);
    let end = globals.buffers.offset(BufTy::Base, 2);
    globals
        .buffers
        .copy_within(BufTy::Base, BufTy::Ex, start, start, end - start);

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let lc_cite = &mut globals.buffers.buffer_mut(BufTy::Ex)[range];
    lc_cite.make_ascii_lowercase();

    let lc_res = if ctx.all_entries {
        globals
            .pool
            .lookup_str_insert(ctx, globals.hash, lc_cite, HashExtra::LcCite(0))?
    } else {
        globals
            .pool
            .lookup_str(globals.hash, lc_cite, StrIlk::LcCite)
    };

    let mut res = lc_res;

    // TODO: Improve this tangled control flow
    let mut inner = || {
        if lc_res.exists {
            let HashExtra::LcCite(cite_loc) = globals.hash.node(lc_res.loc).extra else {
                panic!("LcCite lookup didn't have LcCite extra");
            };
            let HashExtra::Cite(cite) = globals.hash.node(cite_loc).extra else {
                panic!("LcCite location didn't have Cite extra");
            };

            globals.cites.set_entry_ptr(cite);
            let entry_ptr = globals.cites.entry_ptr();
            if !ctx.all_entries
                || entry_ptr < globals.cites.all_marker()
                || entry_ptr > globals.cites.old_num_cites()
            {
                if globals.cites.get_type(entry_ptr) == 0 {
                    if !ctx.all_entries && entry_ptr >= globals.cites.old_num_cites() {
                        let range = globals.buffers.offset(BufTy::Base, 1)
                            ..globals.buffers.offset(BufTy::Base, 2);
                        let cite = &globals.buffers.buffer(BufTy::Base)[range];
                        let uc_res = globals.pool.lookup_str_insert(
                            ctx,
                            globals.hash,
                            cite,
                            HashExtra::Cite(0),
                        );

                        let uc_res = match uc_res {
                            Ok(res) => res,
                            Err(e) => return Some(Err(e)),
                        };

                        res = uc_res;

                        if !uc_res.exists {
                            globals.hash.node_mut(lc_res.loc).extra = HashExtra::LcCite(uc_res.loc);
                            globals.hash.node_mut(uc_res.loc).extra = HashExtra::Cite(entry_ptr);
                            globals
                                .cites
                                .set_cite(entry_ptr, globals.hash.text(uc_res.loc));
                            res.exists = true;
                        }
                    }
                    return None;
                }
            } else if !globals.cites.exists(entry_ptr) {
                let s = globals.pool.get_str(globals.cites.info(entry_ptr));
                globals.buffers.copy_from(BufTy::Ex, 0, s);
                let lc_cite = &mut globals.buffers.buffer_mut(BufTy::Ex)[0..s.len()];
                lc_cite.make_ascii_lowercase();

                let lc_res2 = globals
                    .pool
                    .lookup_str(globals.hash, lc_cite, StrIlk::LcCite);

                res = lc_res2;

                if !lc_res2.exists {
                    cite_key_disappeared_confusion(ctx);
                    return Some(Err(BibtexError::Fatal));
                }
                if lc_res2.loc == lc_res.loc {
                    return None;
                }
            }

            if globals.cites.get_type(entry_ptr) == 0 {
                ctx.write_logs("The cite list is messed up");
                print_confusion(ctx);
                return Some(Err(BibtexError::Fatal));
            }

            ctx.write_logs("Repeated entry");
            return Some(bib_err_print(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                at_bib_command,
            ));
        }
        None
    };

    if let Some(ret) = inner() {
        return ret;
    }

    let store_entry = if ctx.all_entries {
        if res.exists {
            if globals.cites.entry_ptr() >= globals.cites.all_marker() {
                globals.cites.set_exists(globals.cites.entry_ptr(), true);
                let HashExtra::LcCite(cite_loc) = globals.hash.node(lc_res.loc).extra else {
                    panic!("LcCite lookup didn't have LcCite extra");
                };
                globals.cites.set_entry_ptr(globals.cites.ptr());
                let num = add_database_cite(
                    globals.cites,
                    globals.other,
                    globals.hash,
                    globals.cites.ptr(),
                    cite_loc,
                    lc_res.loc,
                );
                globals.cites.set_ptr(num);
            }
        } else {
            let cite = &globals.buffers.buffer(BufTy::Base)
                [globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2)];
            let res =
                globals
                    .pool
                    .lookup_str_insert(ctx, globals.hash, cite, HashExtra::Cite(0))?;
            if res.exists {
                hash_cite_confusion(ctx);
                return Err(BibtexError::Fatal);
            }
            globals.cites.set_entry_ptr(globals.cites.ptr());
            let num = add_database_cite(
                globals.cites,
                globals.other,
                globals.hash,
                globals.cites.ptr(),
                res.loc,
                lc_res.loc,
            );
            globals.cites.set_ptr(num);
        }
        true
    } else {
        res.exists
    };

    if store_entry {
        if type_exists {
            globals
                .cites
                .set_type(globals.cites.entry_ptr(), bst_res.loc);
        } else {
            globals
                .cites
                .set_type(globals.cites.entry_ptr(), HashData::undefined());
            ctx.write_logs("Warning--entry type for \"");
            print_a_token(ctx, globals.buffers);
            ctx.write_logs("\" isn't style-file defined\n");
            bib_warn_print(ctx, globals.pool, globals.bibs)?;
        }
    }

    if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
        eat_bib_print(
            ctx,
            globals.buffers,
            globals.pool,
            globals.bibs,
            at_bib_command,
        )?;
        return Ok(());
    }

    while globals.buffers.at_offset(BufTy::Base, 2) != right_outer_delim {
        if globals.buffers.at_offset(BufTy::Base, 2) != b',' {
            bib_one_of_two_print(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                b',',
                right_outer_delim,
                at_bib_command,
            )?;
            return Ok(());
        }

        globals
            .buffers
            .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

        if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
            eat_bib_print(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                at_bib_command,
            )?;
            return Ok(());
        }

        if globals.buffers.at_offset(BufTy::Base, 2) == right_outer_delim {
            break;
        }

        let scan_res = scan_identifier(globals.buffers, b'=', b'=', b'=');
        match scan_res {
            ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
            _ => {
                bib_id_print(ctx, globals.buffers, scan_res)?;
                ctx.write_logs("a field name");
                bib_err_print(
                    ctx,
                    globals.buffers,
                    globals.pool,
                    globals.bibs,
                    at_bib_command,
                )?;
                return Ok(());
            }
        }

        *field_name_loc = 0;
        let mut store_field = false;
        if store_entry {
            let range =
                globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
            let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
            bst_fn.make_ascii_lowercase();

            let res = globals.pool.lookup_str(globals.hash, bst_fn, StrIlk::BstFn);

            *field_name_loc = res.loc;
            if res.exists {
                if let HashExtra::BstFn(BstFn::Field(_)) = &globals.hash.node(res.loc).extra {
                    store_field = true;
                }
            }
        }

        if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
            eat_bib_print(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                at_bib_command,
            )?;
            return Ok(());
        }

        if globals.buffers.at_offset(BufTy::Base, 2) != b'=' {
            bib_equals_sign_print(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                at_bib_command,
            )?;
            return Ok(());
        }

        globals
            .buffers
            .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

        if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
            eat_bib_print(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                at_bib_command,
            )?;
            return Ok(());
        }

        if !scan_and_store_the_field_value_and_eat_white(
            ctx,
            globals.buffers,
            globals.hash,
            globals.pool,
            globals.bibs,
            globals.other,
            globals.cites,
            store_field,
            at_bib_command,
            BibCommand::Comment,
            None,
            *cur_macro_loc,
            right_outer_delim,
            *field_name_loc,
        )? {
            return Ok(());
        }
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    Ok(())
}
