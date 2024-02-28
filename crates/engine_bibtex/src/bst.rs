use crate::{
    bibs::get_bib_command_or_entry_and_process,
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    cite::find_cite_locs_for_this_cite_key,
    exec::{check_command_execution, execute_fn, ExecCtx},
    hash::{BstFn, HashData, HashExtra},
    log::{
        already_seen_function_print, bad_cross_reference_print,
        bst_err_print_and_look_for_blank_line, bst_id_print, bst_left_brace_print,
        bst_right_brace_print, bst_warn_print, cite_key_disappeared_confusion, eat_bst_print,
        hash_cite_confusion, log_pr_bib_name, nonexistent_cross_reference_error, print_a_token,
        print_bib_name, print_confusion, print_fn_class, print_missing_entry,
    },
    pool::StringPool,
    scan::{eat_bst_white_space, scan_fn_def, scan_identifier, Scan, ScanRes},
    Bibtex, BibtexError, GlobalItems, HashPointer, StrIlk,
};

macro_rules! eat_bst_white {
    ($ctx:ident, $globals:ident, $name:literal) => {
        if !eat_bst_white_space($ctx, $globals.buffers) {
            eat_bst_print($ctx);
            $ctx.write_logs($name);
            bst_err_print_and_look_for_blank_line($ctx, $globals.buffers, $globals.pool)?;
            return Ok(());
        }
    };
}

macro_rules! bst_brace {
    ('{', $ctx:expr, $globals:ident, $name:literal) => {
        if $globals.buffers.at_offset(BufTy::Base, 2) != b'{' {
            bst_left_brace_print($ctx);
            $ctx.write_logs($name);
            bst_err_print_and_look_for_blank_line($ctx, $globals.buffers, $globals.pool)?;
            return Ok(());
        }
    };
    ('}', $ctx:expr, $globals:ident, $name:literal) => {
        if $globals.buffers.at_offset(BufTy::Base, 2) != b'}' {
            bst_right_brace_print($ctx);
            $ctx.write_logs($name);
            bst_err_print_and_look_for_blank_line($ctx, $globals.buffers, $globals.pool)?;
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
                bst_id_print($ctx, $globals.buffers, scan_res)?;
                $ctx.write_logs($name);
                bst_err_print_and_look_for_blank_line($ctx, $globals.buffers, $globals.pool)?;
                return Ok(());
            }
        }
    };
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum BstCommand {
    Entry,
    Execute,
    Function,
    Integers,
    Iterate,
    Macro,
    Read,
    Reverse,
    Sort,
    Strings,
}

fn bst_entry_command(
    ctx: &mut ExecCtx<'_, '_, '_>,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if ctx.entry_seen {
        ctx.write_logs("Illegal, another entry command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
        return Ok(());
    }
    ctx.entry_seen = true;

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

        let res = globals.pool.lookup_str_insert(
            ctx,
            globals.hash,
            bst_fn,
            HashExtra::BstFn(BstFn::Field(globals.other.num_fields())),
        )?;
        if res.exists {
            already_seen_function_print(ctx, globals.buffers, globals.pool, globals.hash, res.loc)?;
            return Ok(());
        }

        globals.other.set_num_fields(globals.other.num_fields() + 1);

        eat_bst_white!(ctx, globals, "entry");
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, globals, "entry");

    if globals.other.num_fields() == globals.other.pre_defined_fields() {
        ctx.write_logs("Warning--I didn't find any fields");
        bst_warn_print(ctx, globals.pool)?;
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

        let res = globals.pool.lookup_str_insert(
            ctx,
            globals.hash,
            bst_fn,
            HashExtra::BstFn(BstFn::IntEntry(globals.entries.num_ent_ints())),
        )?;
        if res.exists {
            already_seen_function_print(ctx, globals.buffers, globals.pool, globals.hash, res.loc)?;
            return Ok(());
        }

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

        let res = globals.pool.lookup_str_insert(
            ctx,
            globals.hash,
            bst_fn,
            HashExtra::BstFn(BstFn::StrEntry(globals.entries.num_ent_strs())),
        )?;
        if res.exists {
            already_seen_function_print(ctx, globals.buffers, globals.pool, globals.hash, res.loc)?;
            return Ok(());
        }

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
    if !ctx.read_seen {
        ctx.write_logs("Illegal, execute command before read command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
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
        ctx,
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

    let res = globals.pool.lookup_str_insert(
        ctx,
        globals.hash,
        bst_fn,
        HashExtra::BstFn(BstFn::Wizard(0)),
    )?;
    if res.exists {
        already_seen_function_print(ctx, globals.buffers, globals.pool, globals.hash, res.loc)?;
        return Ok(());
    }

    if globals.hash.text(res.loc) == ctx.s_default {
        ctx.default = res.loc;
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
        ctx,
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

        let res = globals.pool.lookup_str_insert(
            ctx,
            globals.hash,
            bst_fn,
            HashExtra::BstFn(BstFn::IntGlbl(0)),
        )?;
        if res.exists {
            already_seen_function_print(ctx, globals.buffers, globals.pool, globals.hash, res.loc)?;
            return Ok(());
        }

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
    if !ctx.read_seen {
        ctx.write_logs("Illegal, iterate command before read command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
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
        ctx,
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
    if ctx.read_seen {
        ctx.write_logs("Illegal, macro command after read command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
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
        .lookup_str_insert(ctx, globals.hash, bst_fn, HashExtra::Macro(0))?;
    if res.exists {
        print_a_token(ctx, globals.buffers);
        ctx.write_logs(" is already defined as a macro");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
        return Ok(());
    }
    // This is always unused if the macro is successfully defined, but appears to be a fallback for invalid macros.
    globals.hash.node_mut(res.loc).extra = HashExtra::Macro(globals.hash.text(res.loc));

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
        ctx.write_logs("A macro definition must be \"-delimited");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
        return Ok(());
    }
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    let init = globals.buffers.init(BufTy::Base);
    if !Scan::new().chars(&[b'"']).scan_till(globals.buffers, init) {
        ctx.write_logs("There's no `\"' to end macro definition");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
        return Ok(());
    }

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let text = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
    let res2 = globals
        .pool
        .lookup_str_insert(ctx, globals.hash, text, HashExtra::Text)?;

    globals.hash.node_mut(res.loc).extra = HashExtra::Macro(globals.hash.text(res2.loc));
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
    if ctx.read_seen {
        ctx.write_logs("Illegal, another read command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
        return Ok(());
    }
    ctx.read_seen = true;

    if !ctx.entry_seen {
        ctx.write_logs("Illegal, read command before entry command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
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

    if ctx.all_entries {
        for idx in 0..globals.cites.old_num_cites() {
            globals.cites.set_info(idx, globals.cites.get_cite(idx));
            globals.cites.set_exists(idx, false);
        }
        globals.cites.set_ptr(globals.cites.all_marker());
    } else {
        globals.cites.set_ptr(globals.cites.num_cites());
        globals.cites.set_all_marker(0);
    }

    ctx.read_performed = true;
    for idx in 0..globals.bibs.len() {
        let file = globals.bibs.top_file();
        if ctx.config.verbose {
            ctx.write_logs(&format!("Database file #{}: ", idx + 1));
            print_bib_name(ctx, globals.pool, file.name)?;
        } else {
            ctx.write_log_file(&format!("Database file #{}: ", idx + 1));
            log_pr_bib_name(ctx, globals.pool, file.name)?;
        }

        globals
            .buffers
            .set_offset(BufTy::Base, 2, globals.buffers.init(BufTy::Base));

        let mut cur_macro_loc = 0;
        let mut field_name_loc = 0;
        while !globals.bibs.top_file_mut().file.eof(ctx.engine) {
            get_bib_command_or_entry_and_process(
                ctx,
                globals,
                &mut cur_macro_loc,
                &mut field_name_loc,
            )?;
        }
        globals.bibs.pop_file().file.close(ctx)?;
    }

    ctx.reading_completed = true;
    globals.cites.set_num_cites(globals.cites.ptr());

    let cites = match globals.cites.num_cites() {
        0 => 0,
        val => val - 1,
    };
    if cites * globals.other.num_fields() + globals.other.crossref_num()
        >= globals.other.max_fields()
    {
        ctx.write_logs("field_info index is out of range");
        print_confusion(ctx);
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
                let HashExtra::LcCite(cite_loc) = globals.hash.node(find.lc_cite_loc).extra else {
                    panic!("LcCite lookup didn't have LcCite extra");
                };
                globals
                    .other
                    .set_field(field_ptr, globals.hash.text(cite_loc));

                let field_start = cite_ptr * globals.other.num_fields();
                let HashExtra::Cite(cite) = globals.hash.node(cite_loc).extra else {
                    panic!("LcCite location didn't have Cite extra");
                };
                let mut parent =
                    cite * globals.other.num_fields() + globals.other.pre_defined_fields();
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
                    hash_cite_confusion(ctx);
                    return Err(BibtexError::Fatal);
                }
                nonexistent_cross_reference_error(
                    ctx,
                    globals.pool,
                    globals.cites,
                    globals.other,
                    cite_ptr,
                    field_ptr,
                )?;
                globals.other.set_field(field_ptr, 0);
            } else {
                let HashExtra::LcCite(cite_loc) = globals.hash.node(find.lc_cite_loc).extra else {
                    panic!("LcCite lookup didn't have LcCite extra");
                };
                if find.cite_loc != cite_loc {
                    hash_cite_confusion(ctx);
                    return Err(BibtexError::Fatal);
                }

                let HashExtra::Cite(cite) = globals.hash.node(find.cite_loc).extra else {
                    panic!("Cite lookup didn't have Cite extra");
                };
                let cite_parent_ptr = cite;
                if globals.cites.get_type(cite_parent_ptr) == 0 {
                    nonexistent_cross_reference_error(
                        ctx,
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
                        ctx.write_logs("Warning--you've nested cross references");
                        bad_cross_reference_print(
                            ctx,
                            globals.pool,
                            globals.cites,
                            cite_ptr,
                            globals.cites.get_cite(cite_parent_ptr),
                        )?;
                        ctx.write_logs("\", which also refers to something\n");
                        ctx.mark_warning();
                    }
                    if !ctx.all_entries
                        && cite_parent_ptr >= globals.cites.old_num_cites()
                        && globals.cites.info(cite_parent_ptr) < ctx.config.min_crossrefs as usize
                    {
                        globals.other.set_field(field_ptr, 0);
                    }
                }
            }
        }
    }

    for cite_ptr in 0..globals.cites.num_cites() {
        if globals.cites.get_type(cite_ptr) == 0 {
            print_missing_entry(ctx, globals.pool, globals.cites.get_cite(cite_ptr))?;
        } else if ctx.all_entries
            || cite_ptr < globals.cites.old_num_cites()
            || globals.cites.info(cite_ptr) >= ctx.config.min_crossrefs as usize
        {
            if cite_ptr > ctx.cite_xptr {
                if (ctx.cite_xptr + 1) * globals.other.num_fields() > globals.other.max_fields() {
                    ctx.write_logs("field_info index is out of range");
                    print_confusion(ctx);
                    return Err(BibtexError::Fatal);
                }

                globals
                    .cites
                    .set_cite(ctx.cite_xptr, globals.cites.get_cite(cite_ptr));
                globals
                    .cites
                    .set_type(ctx.cite_xptr, globals.cites.get_type(cite_ptr));

                let find = find_cite_locs_for_this_cite_key(
                    globals.pool,
                    globals.hash,
                    globals.cites.get_cite(cite_ptr),
                );
                if !find.lc_found {
                    cite_key_disappeared_confusion(ctx);
                    return Err(BibtexError::Fatal);
                }

                let HashExtra::LcCite(cite_loc) = globals.hash.node(find.lc_cite_loc).extra else {
                    panic!("LcCite lookup didn't have LcCite extra");
                };
                if !find.cite_found || find.cite_loc != cite_loc {
                    hash_cite_confusion(ctx);
                    return Err(BibtexError::Fatal);
                }

                globals.hash.node_mut(find.cite_loc).extra = HashExtra::Cite(ctx.cite_xptr);

                let start = ctx.cite_xptr * globals.other.num_fields();
                let end = start + globals.other.num_fields();
                let tmp = cite_ptr * globals.other.num_fields();

                for idx in start..end {
                    globals
                        .other
                        .set_field(idx, globals.other.field(tmp + idx - start));
                }
            }
            ctx.cite_xptr += 1;
        }
    }

    globals.cites.set_num_cites(ctx.cite_xptr);

    if ctx.all_entries {
        for idx in globals.cites.all_marker()..globals.cites.old_num_cites() {
            if !globals.cites.exists(idx) {
                print_missing_entry(ctx, globals.pool, globals.cites.info(idx))?;
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
    if !ctx.read_seen {
        ctx.write_logs("Illegal, reverse command before read command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
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
        ctx,
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
    if !ctx.read_seen {
        ctx.write_logs("Illegal, sort command before read command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
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

        let res = globals.pool.lookup_str_insert(
            ctx,
            globals.hash,
            bst_fn,
            HashExtra::BstFn(BstFn::StrGlbl(globals.globals.num_glb_strs())),
        )?;

        if res.exists {
            already_seen_function_print(ctx, globals.buffers, globals.pool, globals.hash, res.loc)?;
            return Ok(());
        }

        if globals.globals.num_glb_strs() == globals.globals.len() {
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
        print_a_token(ctx, buffers);
        ctx.write_logs(" is an unknown function");
        bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
        Ok(true)
    } else if let HashExtra::BstFn(BstFn::Builtin(_) | BstFn::Wizard(_)) = hash.node(res.loc).extra
    {
        Ok(false)
    } else {
        print_a_token(ctx, buffers);
        ctx.write_logs(" has bad function type");
        print_fn_class(ctx, hash, res.loc);
        bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
        Ok(true)
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
        ctx.write_logs(&format!(
            "\"{}\" can't start a style-file command",
            globals.buffers.at_offset(BufTy::Base, 2) as char,
        ));
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
        return Ok(());
    }

    let range = globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
    let bst_cmd = &mut globals.buffers.buffer_mut(BufTy::Base)[range];
    bst_cmd.make_ascii_lowercase();

    let res = globals
        .pool
        .lookup_str(globals.hash, bst_cmd, StrIlk::BstCommand);
    if !res.exists {
        print_a_token(ctx, globals.buffers);
        ctx.write_logs(" is an illegal style-file command");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
        return Ok(());
    }

    let HashExtra::BstCommand(cmd) = globals.hash.node(res.loc).extra else {
        panic!("BstCommand lookup didn't have BstCommand extra");
    };

    match cmd {
        BstCommand::Entry => bst_entry_command(ctx, globals),
        BstCommand::Execute => bst_execute_command(ctx, globals),
        BstCommand::Function => bst_function_command(ctx, globals),
        BstCommand::Integers => bst_integers_command(ctx, globals),
        BstCommand::Iterate => bst_iterate_command(ctx, globals),
        BstCommand::Macro => bst_macro_command(ctx, globals),
        BstCommand::Read => bst_read_command(ctx, globals),
        BstCommand::Reverse => bst_reverse_command(ctx, globals),
        BstCommand::Sort => bst_sort_command(ctx, globals),
        BstCommand::Strings => bst_strings_command(ctx, globals),
    }
}
