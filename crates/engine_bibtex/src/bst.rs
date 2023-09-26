use crate::{
    bibs::get_bib_command_or_entry_and_process,
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    cite::find_cite_locs_for_this_cite_key,
    exec::{check_command_execution, execute_fn, ExecCtx},
    hash::{FnClass, HashData},
    history::mark_warning,
    log::{
        already_seen_function_print, bst_err_print_and_look_for_blank_line, bst_id_print,
        bst_left_brace_print, bst_right_brace_print, bst_warn_print,
        cite_key_disappeared_confusion, eat_bst_print, hash_cite_confusion, log_pr_bib_name,
        print_a_token, print_bib_name, print_confusion, print_missing_entry,
        rs_bad_cross_reference_print, rs_nonexistent_cross_reference_error, rs_print_fn_class,
        write_log_file, write_logs,
    },
    pool::StringPool,
    scan::{eat_bst_white_space, scan_fn_def, scan_identifier, Scan, ScanRes},
    Bibtex, BibtexError, CiteNumber, GlobalItems, HashPointer, StrIlk,
};

macro_rules! eat_bst_white {
    ($ctx:ident, $globals:ident, $name:literal) => {
        if !eat_bst_white_space($ctx.glbl_ctx_mut(), $globals.buffers) {
            eat_bst_print();
            write_logs($name);
            bst_err_print_and_look_for_blank_line(
                $ctx.glbl_ctx_mut(),
                $globals.buffers,
                $globals.pool,
            )?;
            return Ok(());
        }
    };
}

macro_rules! bst_brace {
    ('{', $ctx:expr, $globals:ident, $name:literal) => {
        if $globals.buffers.at_offset(BufTy::Base, 2) != b'{' {
            bst_left_brace_print();
            write_logs($name);
            bst_err_print_and_look_for_blank_line(
                $ctx.glbl_ctx_mut(),
                $globals.buffers,
                $globals.pool,
            )?;
            return Ok(());
        }
    };
    ('}', $ctx:expr, $globals:ident, $name:literal) => {
        if $globals.buffers.at_offset(BufTy::Base, 2) != b'}' {
            bst_right_brace_print();
            write_logs($name);
            bst_err_print_and_look_for_blank_line(
                $ctx.glbl_ctx_mut(),
                $globals.buffers,
                $globals.pool,
            )?;
            return Ok(());
        }
    };
}

macro_rules! bst_ident {
    ($ctx:ident, $globals:ident, $name:literal, $c1:literal, $c2:literal, $c3:literal) => {
        let scan_res = scan_identifier($globals.buffers, $c1, $c2, $c3);
        match scan_res {
            ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
            _ => {
                bst_id_print($globals.buffers, scan_res)?;
                write_logs($name);
                bst_err_print_and_look_for_blank_line(
                    $ctx.glbl_ctx_mut(),
                    $globals.buffers,
                    $globals.pool,
                )?;
                return Ok(());
            }
        }
    };
}

fn bst_entry_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if ctx.glbl_ctx().entry_seen {
        write_logs("Illegal, another entry command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }
    ctx.glbl_ctx_mut().entry_seen = true;

    eat_bst_white!(ctx, globals, "entry");
    bst_brace!('{', ctx, globals, "entry");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "entry");

    while globals.buffers.at_offset(BufTy::Base, 2) != b'}' {
        bst_ident!(ctx, globals, "entry", b'}', b'#', b'#');
        let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
        let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
        bst_fn.make_ascii_lowercase();

        let res = globals
            .pool
            .lookup_str_insert(globals.hash, bst_fn, StrIlk::BstFn)?;
        if res.exists {
            already_seen_function_print(
                ctx.glbl_ctx_mut(),
                globals.buffers,
                globals.pool,
                globals.hash,
                res.loc,
            )?;
            return Ok(());
        }

        globals.hash.set_ty(res.loc, FnClass::Field);
        globals
            .hash
            .set_ilk_info(res.loc, globals.other.num_fields() as i32);
        globals.other.set_num_fields(globals.other.num_fields() + 1);

        eat_bst_white!(ctx, globals, "entry");
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "entry");

    if globals.other.num_fields() == globals.other.pre_defined_fields() {
        write_logs("Warning--I didn't find any fields");
        bst_warn_print(ctx.glbl_ctx(), globals.pool)?;
    }

    bst_brace!('{', ctx, globals, "entry");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "entry");

    while globals.buffers.at_offset(BufTy::Base, 2) != b'}' {
        bst_ident!(ctx, globals, "entry", b'}', b'#', b'#');

        let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
        let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
        bst_fn.make_ascii_lowercase();

        let res = globals
            .pool
            .lookup_str_insert(globals.hash, bst_fn, StrIlk::BstFn)?;
        if res.exists {
            already_seen_function_print(
                ctx.glbl_ctx_mut(),
                globals.buffers,
                globals.pool,
                globals.hash,
                res.loc,
            )?;
            return Ok(());
        }

        globals.hash.set_ty(res.loc, FnClass::IntEntryVar);
        globals
            .hash
            .set_ilk_info(res.loc, globals.entries.num_ent_ints() as i32);
        globals
            .entries
            .set_num_ent_ints(globals.entries.num_ent_ints() + 1);

        eat_bst_white!(ctx, globals, "entry");
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "entry");
    bst_brace!('{', ctx, globals, "entry");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "entry");

    while globals.buffers.at_offset(BufTy::Base, 2) != b'}' {
        bst_ident!(ctx, globals, "entry", b'}', b'#', b'#');

        let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
        let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
        bst_fn.make_ascii_lowercase();

        let res = globals
            .pool
            .lookup_str_insert(globals.hash, bst_fn, StrIlk::BstFn)?;
        if res.exists {
            already_seen_function_print(
                ctx.glbl_ctx_mut(),
                globals.buffers,
                globals.pool,
                globals.hash,
                res.loc,
            )?;
            return Ok(());
        }

        globals.hash.set_ty(res.loc, FnClass::StrEntryVar);
        globals
            .hash
            .set_ilk_info(res.loc, globals.entries.num_ent_strs() as i32);
        globals
            .entries
            .set_num_ent_strs(globals.entries.num_ent_strs() + 1);

        eat_bst_white!(ctx, globals, "entry");
    }
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    Ok(())
}

fn bst_execute_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if !ctx.glbl_ctx().read_seen {
        write_logs("Illegal, execute command before read command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }
    eat_bst_white!(ctx, globals, "execute");
    bst_brace!('{', ctx, globals, "execute");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "execute");
    bst_ident!(ctx, globals, "execute", b'}', b'#', b'#');

    let mut fn_loc = 0;
    if bad_argument_token(
        ctx.glbl_ctx_mut(),
        Some(&mut fn_loc),
        globals.buffers,
        globals.pool,
        globals.hash,
    )? {
        return Ok(());
    }

    eat_bst_white!(ctx, globals, "execute");
    bst_brace!('}', ctx, globals, "execute");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    // TODO: Associated method on ExecCtx
    ctx.lit_stack.clear();
    ctx.bib_str_ptr = globals.pool.str_ptr();

    ctx.mess_with_entries = false;

    execute_fn(ctx, globals, fn_loc)?;
    check_command_execution(ctx, globals.pool, globals.hash, globals.cites)?;

    Ok(())
}

fn bst_function_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    eat_bst_white!(ctx, globals, "function");
    bst_brace!('{', ctx, globals, "function");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "function");
    bst_ident!(ctx, globals, "function", b'}', b'#', b'#');

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
    bst_fn.make_ascii_lowercase();

    let res = globals
        .pool
        .lookup_str_insert(globals.hash, bst_fn, StrIlk::BstFn)?;
    if res.exists {
        already_seen_function_print(
            ctx.glbl_ctx_mut(),
            globals.buffers,
            globals.pool,
            globals.hash,
            res.loc,
        )?;
        return Ok(());
    }

    globals.hash.set_ty(res.loc, FnClass::Wizard);
    if globals.hash.text(res.loc) == ctx.glbl_ctx().s_default {
        ctx._default = res.loc;
    }

    eat_bst_white!(ctx, globals, "function");
    bst_brace!('}', ctx, globals, "function");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "function");
    bst_brace!('{', ctx, globals, "function");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    scan_fn_def(
        ctx.glbl_ctx_mut(),
        globals.buffers,
        globals.hash,
        globals.pool,
        globals.other,
        res.loc,
        res.loc,
    )?;
    Ok(())
}

fn bst_integers_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    eat_bst_white!(ctx, globals, "integers");
    bst_brace!('{', ctx, globals, "integers");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "integers");

    while globals.buffers.at_offset(BufTy::Base, 2) != b'}' {
        bst_ident!(ctx, globals, "integers", b'}', b'#', b'#');

        let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
        let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
        bst_fn.make_ascii_lowercase();

        let res = globals
            .pool
            .lookup_str_insert(globals.hash, bst_fn, StrIlk::BstFn)?;
        if res.exists {
            already_seen_function_print(
                ctx.glbl_ctx_mut(),
                globals.buffers,
                globals.pool,
                globals.hash,
                res.loc,
            )?;
            return Ok(());
        }

        globals.hash.set_ty(res.loc, FnClass::IntGlblVar);
        globals.hash.set_ilk_info(res.loc, 0);
        eat_bst_white!(ctx, globals, "integers");
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    Ok(())
}

fn bst_iterate_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if !ctx.glbl_ctx().read_seen {
        write_logs("Illegal, iterate command before read command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }

    eat_bst_white!(ctx, globals, "iterate");
    bst_brace!('{', ctx, globals, "iterate");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "iterate");
    bst_ident!(ctx, globals, "iterate", b'}', b'#', b'#');

    let mut fn_loc = 0;
    if bad_argument_token(
        ctx.glbl_ctx_mut(),
        Some(&mut fn_loc),
        globals.buffers,
        globals.pool,
        globals.hash,
    )? {
        return Ok(());
    }
    eat_bst_white!(ctx, globals, "iterate");
    bst_brace!('}', ctx, globals, "iterate");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    ctx.lit_stack.clear();
    ctx.bib_str_ptr = globals.pool.str_ptr();

    ctx.mess_with_entries = true;

    let mut sort_cite_ptr = 0;
    while sort_cite_ptr < globals.cites.num_cites() {
        globals.cites.set_ptr(globals.cites.info(sort_cite_ptr));
        execute_fn(ctx, globals, fn_loc)?;
        check_command_execution(ctx, globals.pool, globals.hash, globals.cites)?;
        sort_cite_ptr += 1;
    }

    Ok(())
}

fn bst_macro_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if ctx.glbl_ctx().read_seen {
        write_logs("Illegal, macro command after read command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }

    eat_bst_white!(ctx, globals, "macro");
    bst_brace!('{', ctx, globals, "macro");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "macro");
    bst_ident!(ctx, globals, "macro", b'}', b'#', b'#');

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
    bst_fn.make_ascii_lowercase();

    let res = globals
        .pool
        .lookup_str_insert(globals.hash, bst_fn, StrIlk::Macro)?;
    if res.exists {
        print_a_token(globals.buffers);
        write_logs(" is already defined as a macro");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }
    globals
        .hash
        .set_ilk_info(res.loc, globals.hash.text(res.loc) as i32);

    eat_bst_white!(ctx, globals, "macro");
    bst_brace!('}', ctx, globals, "macro");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "macro");
    bst_brace!('{', ctx, globals, "macro");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "macro");
    if globals.buffers.at_offset(BufTy::Base, 2) != b'"' {
        write_logs("A macro definition must be \"-delimited");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    let init = globals.buffers.init(BufTy::Base);
    if !Scan::new().chars(&[b'"']).scan_till(globals.buffers, init) {
        write_logs("There's no `\"' to end macro definition");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let text = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
    let res2 = globals
        .pool
        .lookup_str_insert(globals.hash, text, StrIlk::Text)?;

    globals.hash.set_ty(res2.loc, FnClass::StrLit);
    globals
        .hash
        .set_ilk_info(res.loc, globals.hash.text(res2.loc) as i32);
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "macro");
    bst_brace!('}', ctx, globals, "macro");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    Ok(())
}

fn bst_read_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if ctx.glbl_ctx().read_seen {
        write_logs("Illegal, another read command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }
    ctx.glbl_ctx_mut().read_seen = true;

    if !ctx.glbl_ctx().entry_seen {
        write_logs("Illegal, read command before entry command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }

    let start = globals.buffers.offset(BufTy::Base, 2);
    let to = globals.buffers.init(BufTy::Base);
    let sv_range = start..to;
    globals.buffers.copy_within(
        BufTy::Base,
        BufTy::Sv,
        sv_range.start,
        sv_range.start,
        sv_range.end,
    );

    globals
        .other
        .check_field_overflow(globals.other.num_fields() * globals.cites.num_cites());

    for idx in 0..globals.other.max_fields() {
        globals.other.set_field(idx, 0);
    }

    for idx in 0..globals.cites.len() {
        globals.cites.set_type(idx, 0);
        globals.cites.set_info(idx, 0);
    }
    globals.cites.set_old_num_cites(globals.cites.num_cites());

    if ctx.glbl_ctx().all_entries {
        for idx in 0..globals.cites.old_num_cites() {
            globals.cites.set_info(idx, globals.cites.get_cite(idx));
            globals.cites.set_exists(idx, false);
        }
        globals.cites.set_ptr(globals.cites.all_marker());
    } else {
        globals.cites.set_ptr(globals.cites.num_cites());
        globals.cites.set_all_marker(0);
    }

    ctx.glbl_ctx_mut().read_performed = true;
    while globals.bibs.len() != 0 {
        if ctx.glbl_ctx().config.verbose {
            write_logs(&format!("Database file #{}: ", globals.bibs.len()));
            print_bib_name(globals.pool, globals.bibs.top_file().name)?;
        } else {
            write_log_file(&format!("Database file #{}: ", globals.bibs.len()));
            log_pr_bib_name(globals.bibs, globals.pool)?;
        }

        globals
            .buffers
            .set_offset(BufTy::Base, 2, globals.buffers.init(BufTy::Base));

        let mut cur_macro_loc = 0;
        let mut field_name_loc = 0;
        while !globals.bibs.top_file_mut().file.eof() {
            get_bib_command_or_entry_and_process(
                ctx.glbl_ctx_mut(),
                globals,
                &mut cur_macro_loc,
                &mut field_name_loc,
            )?;
        }
        globals.bibs.pop_file().file.close(ctx.glbl_ctx_mut())?;
    }

    ctx.glbl_ctx_mut().reading_completed = true;
    globals.cites.set_num_cites(globals.cites.ptr());

    let cites = match globals.cites.num_cites() {
        0 => 0,
        val => val - 1,
    };
    if cites * globals.other.num_fields() + globals.other.crossref_num()
        >= globals.other.max_fields()
    {
        write_logs("field_info index is out of range");
        print_confusion();
        return Err(BibtexError::Fatal);
    }

    for cite_ptr in 0..globals.cites.num_cites() {
        let field_ptr = cite_ptr * globals.other.num_fields() + globals.other.crossref_num();
        if globals.other.field(field_ptr) != 0 {
            let find = find_cite_locs_for_this_cite_key(
                globals.pool,
                globals.hash,
                globals.other.field(field_ptr),
            );

            if find.lc_found {
                let cite_loc = globals.hash.ilk_info(find.lc_cite_loc) as CiteNumber;
                globals
                    .other
                    .set_field(field_ptr, globals.hash.text(cite_loc));

                let field_start = cite_ptr * globals.other.num_fields();
                let mut parent = globals.hash.ilk_info(cite_loc) as usize
                    * globals.other.num_fields()
                    + globals.other.pre_defined_fields();
                for idx in (field_start + globals.other.pre_defined_fields())
                    ..(field_start + globals.other.num_fields())
                {
                    if globals.other.field(idx) == 0 {
                        globals.other.set_field(idx, globals.other.field(parent));
                    }
                    parent += 1;
                }
            }
        }
    }

    for cite_ptr in 0..globals.cites.num_cites() {
        let field_ptr = cite_ptr * globals.other.num_fields() + globals.other.crossref_num();
        if globals.other.field(field_ptr) != 0 {
            let find = find_cite_locs_for_this_cite_key(
                globals.pool,
                globals.hash,
                globals.other.field(field_ptr),
            );

            if !find.lc_found {
                if find.cite_found {
                    hash_cite_confusion();
                    return Err(BibtexError::Fatal);
                }
                rs_nonexistent_cross_reference_error(
                    globals.pool,
                    globals.cites,
                    globals.other,
                    cite_ptr,
                    field_ptr,
                )?;
                globals.other.set_field(field_ptr, 0);
            } else {
                if find.cite_loc != globals.hash.ilk_info(find.lc_cite_loc) as CiteNumber {
                    hash_cite_confusion();
                    return Err(BibtexError::Fatal);
                }

                let cite_parent_ptr = globals.hash.ilk_info(find.cite_loc) as CiteNumber;
                if globals.cites.get_type(cite_parent_ptr) == 0 {
                    rs_nonexistent_cross_reference_error(
                        globals.pool,
                        globals.cites,
                        globals.other,
                        cite_ptr,
                        field_ptr,
                    )?;
                    globals.other.set_field(field_ptr, 0);
                } else {
                    let field_parent_ptr =
                        cite_parent_ptr * globals.other.num_fields() + globals.other.crossref_num();
                    if globals.other.field(field_parent_ptr) != 0 {
                        write_logs("Warning--you've nested cross references");
                        rs_bad_cross_reference_print(
                            globals.pool,
                            globals.cites,
                            cite_ptr,
                            globals.cites.get_cite(cite_parent_ptr),
                        )?;
                        write_logs("\", which also refers to something\n");
                        mark_warning();
                    }
                    if !ctx.glbl_ctx().all_entries
                        && cite_parent_ptr >= globals.cites.old_num_cites()
                        && globals.cites.info(cite_parent_ptr)
                            < ctx.glbl_ctx().config.min_crossrefs as usize
                    {
                        globals.other.set_field(field_ptr, 0);
                    }
                }
            }
        }
    }

    for cite_ptr in 0..globals.cites.num_cites() {
        if globals.cites.get_type(cite_ptr) == 0 {
            print_missing_entry(globals.pool, globals.cites.get_cite(cite_ptr))?;
        } else if ctx.glbl_ctx().all_entries
            || cite_ptr < globals.cites.old_num_cites()
            || globals.cites.info(cite_ptr) >= ctx.glbl_ctx().config.min_crossrefs as usize
        {
            if cite_ptr > ctx.glbl_ctx().cite_xptr {
                if (ctx.glbl_ctx().cite_xptr + 1) * globals.other.num_fields()
                    > globals.other.max_fields()
                {
                    write_logs("field_info index is out of range");
                    print_confusion();
                    return Err(BibtexError::Fatal);
                }

                globals
                    .cites
                    .set_cite(ctx.glbl_ctx().cite_xptr, globals.cites.get_cite(cite_ptr));
                globals
                    .cites
                    .set_type(ctx.glbl_ctx().cite_xptr, globals.cites.get_type(cite_ptr));

                let find = find_cite_locs_for_this_cite_key(
                    globals.pool,
                    globals.hash,
                    globals.cites.get_cite(cite_ptr),
                );
                if !find.lc_found {
                    cite_key_disappeared_confusion();
                    return Err(BibtexError::Fatal);
                }

                if !find.cite_found
                    || find.cite_loc != globals.hash.ilk_info(find.lc_cite_loc) as CiteNumber
                {
                    hash_cite_confusion();
                    return Err(BibtexError::Fatal);
                }

                globals
                    .hash
                    .set_ilk_info(find.cite_loc, ctx.glbl_ctx().cite_xptr as i32);

                let start = ctx.glbl_ctx().cite_xptr * globals.other.num_fields();
                let end = start + globals.other.num_fields();
                let tmp = cite_ptr * globals.other.num_fields();

                for idx in start..end {
                    globals
                        .other
                        .set_field(idx, globals.other.field(tmp + idx - start));
                }
            }
            ctx.glbl_ctx_mut().cite_xptr += 1;
        }
    }

    globals.cites.set_num_cites(ctx.glbl_ctx().cite_xptr);

    if ctx.glbl_ctx().all_entries {
        for idx in globals.cites.all_marker()..globals.cites.old_num_cites() {
            if !globals.cites.exists(idx) {
                print_missing_entry(globals.pool, globals.cites.info(idx))?;
            }
        }
    }

    globals.entries.init_entries(globals.cites);

    for idx in 0..globals.cites.num_cites() {
        globals.cites.set_info(idx, idx);
    }
    globals.cites.set_ptr(globals.cites.num_cites());

    globals.buffers.copy_within(
        BufTy::Sv,
        BufTy::Base,
        sv_range.start,
        sv_range.start,
        sv_range.end,
    );
    globals.buffers.set_offset(BufTy::Base, 2, sv_range.start);
    globals.buffers.set_init(BufTy::Base, sv_range.end);

    Ok(())
}

fn bst_reverse_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if !ctx.glbl_ctx().read_seen {
        write_logs("Illegal, reverse command before read command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }

    eat_bst_white!(ctx, globals, "reverse");
    bst_brace!('{', ctx, globals, "reverse");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "reverse");
    bst_ident!(ctx, globals, "reverse", b'}', b'#', b'#');

    let mut fn_loc = 0;
    if bad_argument_token(
        ctx.glbl_ctx_mut(),
        Some(&mut fn_loc),
        globals.buffers,
        globals.pool,
        globals.hash,
    )? {
        return Ok(());
    }

    eat_bst_white!(ctx, globals, "reverse");
    bst_brace!('}', ctx, globals, "reverse");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    ctx.lit_stack.clear();
    ctx.bib_str_ptr = globals.pool.str_ptr();

    ctx.mess_with_entries = true;

    for idx in (0..globals.cites.num_cites()).rev() {
        globals.cites.set_ptr(globals.cites.info(idx));
        execute_fn(ctx, globals, fn_loc)?;
        check_command_execution(ctx, globals.pool, globals.hash, globals.cites)?;
    }

    Ok(())
}

fn bst_sort_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if !ctx.glbl_ctx().read_seen {
        write_logs("Illegal, sort command before read command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }

    if globals.cites.num_cites() > 1 {
        globals
            .cites
            .sort_info(globals.entries, 0..=globals.cites.num_cites() - 1);
    }

    Ok(())
}

fn bst_strings_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    eat_bst_white!(ctx, globals, "strings");
    bst_brace!('{', ctx, globals, "strings");
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "strings");

    while globals.buffers.at_offset(BufTy::Base, 2) != b'}' {
        bst_ident!(ctx, globals, "strings", b'}', b'#', b'#');

        let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
        let bst_fn = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
        bst_fn.make_ascii_lowercase();

        let res = globals
            .pool
            .lookup_str_insert(globals.hash, bst_fn, StrIlk::BstFn)?;

        if res.exists {
            already_seen_function_print(
                ctx.glbl_ctx_mut(),
                globals.buffers,
                globals.pool,
                globals.hash,
                res.loc,
            )?;
            return Ok(());
        }

        globals.hash.set_ty(res.loc, FnClass::StrGlblVar);
        globals
            .hash
            .set_ilk_info(res.loc, globals.globals.num_glb_strs());

        if globals.globals.num_glb_strs() as usize == globals.globals.len() {
            globals.globals.grow();
        }

        globals
            .globals
            .set_num_glb_strs(globals.globals.num_glb_strs() + 1);

        eat_bst_white!(ctx, globals, "strings");
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    Ok(())
}

fn bad_argument_token(
    ctx: &mut Bibtex<'_, '_>,
    fn_out: Option<&mut HashPointer>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
    hash: &HashData,
) -> Result<bool, BibtexError> {
    let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
    let bst_fn = &mut buffers.buffer_mut(BufTy::Base)[range];
    bst_fn.make_ascii_lowercase();

    let res = pool.lookup_str(hash, bst_fn, StrIlk::BstFn);

    if let Some(fn_out) = fn_out {
        *fn_out = res.loc;
    }

    if !res.exists {
        print_a_token(buffers);
        write_logs(" is an unknown function");
        bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
        Ok(true)
    } else if hash.ty(res.loc) != FnClass::Builtin && hash.ty(res.loc) != FnClass::Wizard {
        print_a_token(buffers);
        write_logs(" has bad function type");
        rs_print_fn_class(hash, res.loc);
        bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub(crate) fn get_bst_command_and_process(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    let init = globals.buffers.init(BufTy::Base);
    if !Scan::new()
        .not_class(LexClass::Alpha)
        .scan_till_nonempty(globals.buffers, init)
    {
        write_logs(&format!(
            "\"{}\" can't start a style-file command",
            globals.buffers.at_offset(BufTy::Base, 2) as char,
        ));
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let bst_cmd = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
    bst_cmd.make_ascii_lowercase();

    let res = globals
        .pool
        .lookup_str(globals.hash, bst_cmd, StrIlk::BstCommand);
    if !res.exists {
        print_a_token(globals.buffers);
        write_logs(" is an illegal style-file command");
        bst_err_print_and_look_for_blank_line(ctx.glbl_ctx_mut(), globals.buffers, globals.pool)?;
        return Ok(());
    }

    match globals.hash.ilk_info(res.loc) {
        0 => bst_entry_command(ctx, globals),
        1 => bst_execute_command(ctx, globals),
        2 => bst_function_command(ctx, globals),
        3 => bst_integers_command(ctx, globals),
        4 => bst_iterate_command(ctx, globals),
        5 => bst_macro_command(ctx, globals),
        6 => bst_read_command(ctx, globals),
        7 => bst_reverse_command(ctx, globals),
        8 => bst_sort_command(ctx, globals),
        9 => bst_strings_command(ctx, globals),
        _ => {
            write_logs("Unknown style-file command");
            print_confusion();
            Err(BibtexError::Fatal)
        }
    }
}
