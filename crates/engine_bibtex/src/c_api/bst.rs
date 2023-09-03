use crate::{
    c_api::{
        buffer::{with_buffers_mut, BufTy, GlobalBuffer},
        exec::{rs_check_command_execution, rs_execute_fn, ExecCtx},
        hash::{with_hash, FnClass, HashData},
        log::{
            bst_left_brace_print, bst_right_brace_print, bst_warn_print, eat_bst_print,
            rs_already_seen_function_print, rs_bst_err_print_and_look_for_blank_line,
            rs_bst_id_print, rs_print_a_token, rs_print_fn_class, write_logs,
        },
        pool::{with_pool, StringPool},
        scan::{rs_eat_bst_white_space, rs_scan_identifier, scan_fn_def, Scan, ScanRes},
        Bibtex, CResult, CResultBool, GlobalItems, HashPointer, StrIlk,
    },
    BibtexError,
};

macro_rules! eat_bst_white {
    ($ctx:ident, $globals:ident, $name:literal) => {
        if !rs_eat_bst_white_space($ctx.glbl_ctx_mut(), $globals.buffers) {
            eat_bst_print();
            write_logs($name);
            rs_bst_err_print_and_look_for_blank_line(
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
            rs_bst_err_print_and_look_for_blank_line(
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
            rs_bst_err_print_and_look_for_blank_line(
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
        let scan_res = rs_scan_identifier($globals.buffers, $c1, $c2, $c3);
        match scan_res {
            ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
            _ => {
                rs_bst_id_print($globals.buffers, scan_res)?;
                write_logs($name);
                rs_bst_err_print_and_look_for_blank_line(
                    $ctx.glbl_ctx_mut(),
                    $globals.buffers,
                    $globals.pool,
                )?;
                return Ok(());
            }
        }
    };
}

fn rs_bst_entry_command(
    ctx: &mut ExecCtx,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if ctx.glbl_ctx().entry_seen {
        write_logs("Illegal, another entry command");
        rs_bst_err_print_and_look_for_blank_line(
            ctx.glbl_ctx_mut(),
            globals.buffers,
            globals.pool,
        )?;
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
            rs_already_seen_function_print(
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
            rs_already_seen_function_print(
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
            rs_already_seen_function_print(
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

fn rs_bst_execute_command(
    ctx: &mut ExecCtx,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if !ctx.glbl_ctx().read_seen {
        write_logs("Illegal, execute command before read command");
        rs_bst_err_print_and_look_for_blank_line(
            ctx.glbl_ctx_mut(),
            globals.buffers,
            globals.pool,
        )?;
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
    if rs_bad_argument_token(
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
    ctx.lit_stk_ptr = 0;
    ctx.bib_str_ptr = globals.pool.str_ptr();

    ctx.mess_with_entries = false;

    rs_execute_fn(ctx, globals, fn_loc)?;
    rs_check_command_execution(ctx, globals.pool, globals.hash)?;

    Ok(())
}

fn rs_bst_function_command(
    ctx: &mut ExecCtx,
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
        rs_already_seen_function_print(
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

fn rs_bst_integers_command(
    ctx: &mut ExecCtx,
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
            rs_already_seen_function_print(
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

fn rs_bst_iterate_command(
    ctx: &mut ExecCtx,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if !ctx.glbl_ctx().read_seen {
        write_logs("Illegal, iterate command before read command");
        rs_bst_err_print_and_look_for_blank_line(
            ctx.glbl_ctx_mut(),
            globals.buffers,
            globals.pool,
        )?;
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
    if rs_bad_argument_token(
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

    ctx.lit_stk_ptr = 0;
    ctx.bib_str_ptr = globals.pool.str_ptr();

    ctx.mess_with_entries = true;

    let mut sort_cite_ptr = 0;
    while sort_cite_ptr < globals.cites.num_cites() {
        globals.cites.set_ptr(globals.cites.get_info(sort_cite_ptr));
        rs_execute_fn(ctx, globals, fn_loc)?;
        rs_check_command_execution(ctx, globals.pool, globals.hash)?;
        sort_cite_ptr += 1;
    }

    Ok(())
}

fn rs_bst_macro_command(
    ctx: &mut ExecCtx,
    globals: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if ctx.glbl_ctx().read_seen {
        write_logs("Illegal, macro command after read command");
        rs_bst_err_print_and_look_for_blank_line(
            ctx.glbl_ctx_mut(),
            globals.buffers,
            globals.pool,
        )?;
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
        rs_print_a_token(globals.buffers);
        write_logs(" is already defined as a macro");
        rs_bst_err_print_and_look_for_blank_line(
            ctx.glbl_ctx_mut(),
            globals.buffers,
            globals.pool,
        )?;
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
        rs_bst_err_print_and_look_for_blank_line(
            ctx.glbl_ctx_mut(),
            globals.buffers,
            globals.pool,
        )?;
        return Ok(());
    }
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    let init = globals.buffers.init(BufTy::Base);
    if !Scan::new().chars(&[b'"']).scan_till(globals.buffers, init) {
        write_logs("There's no `\"' to end macro definition");
        rs_bst_err_print_and_look_for_blank_line(
            ctx.glbl_ctx_mut(),
            globals.buffers,
            globals.pool,
        )?;
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

fn rs_bad_argument_token(
    ctx: &mut Bibtex,
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
        rs_print_a_token(buffers);
        write_logs(" is an unknown function");
        rs_bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
        Ok(true)
    } else if hash.ty(res.loc) != FnClass::Builtin && hash.ty(res.loc) != FnClass::Wizard {
        rs_print_a_token(buffers);
        write_logs(" has bad function type");
        rs_print_fn_class(hash, res.loc);
        rs_bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[no_mangle]
pub unsafe extern "C" fn bad_argument_token(
    ctx: *mut Bibtex,
    fn_out: *mut HashPointer,
) -> CResultBool {
    with_buffers_mut(|buffers| {
        with_pool(|pool| {
            with_hash(|hash| rs_bad_argument_token(&mut *ctx, fn_out.as_mut(), buffers, pool, hash))
        })
    })
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn bst_entry_command(ctx: *mut ExecCtx) -> CResult {
    GlobalItems::with(|globals| rs_bst_entry_command(&mut *ctx, globals)).into()
}

#[no_mangle]
pub unsafe extern "C" fn bst_execute_command(ctx: *mut ExecCtx) -> CResult {
    GlobalItems::with(|globals| rs_bst_execute_command(&mut *ctx, globals)).into()
}

#[no_mangle]
pub unsafe extern "C" fn bst_function_command(ctx: *mut ExecCtx) -> CResult {
    GlobalItems::with(|globals| rs_bst_function_command(&mut *ctx, globals)).into()
}

#[no_mangle]
pub unsafe extern "C" fn bst_integers_command(ctx: *mut ExecCtx) -> CResult {
    GlobalItems::with(|globals| rs_bst_integers_command(&mut *ctx, globals)).into()
}

#[no_mangle]
pub unsafe extern "C" fn bst_iterate_command(ctx: *mut ExecCtx) -> CResult {
    GlobalItems::with(|globals| rs_bst_iterate_command(&mut *ctx, globals)).into()
}

#[no_mangle]
pub unsafe extern "C" fn bst_macro_command(ctx: *mut ExecCtx) -> CResult {
    GlobalItems::with(|globals| rs_bst_macro_command(&mut *ctx, globals)).into()
}
