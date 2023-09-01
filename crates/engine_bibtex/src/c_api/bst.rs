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
        scan::{rs_eat_bst_white_space, rs_scan_identifier, ScanRes},
        Bibtex, CResult, CResultBool, GlobalItems, HashPointer, StrIlk,
    },
    BibtexError,
};

macro_rules! eat_bst_white {
    ($ctx:expr, $buffers:expr, $pool:expr, $name:literal) => {
        if !rs_eat_bst_white_space($ctx, $buffers) {
            eat_bst_print();
            write_logs($name);
            rs_bst_err_print_and_look_for_blank_line($ctx, $buffers, $pool)?;
            return Ok(());
        }
    };
}

macro_rules! bst_brace {
    ('{', $ctx:expr, $buffers:expr, $pool:expr, $name:literal) => {
        if $buffers.at_offset(BufTy::Base, 2) != b'{' {
            bst_left_brace_print();
            write_logs($name);
            rs_bst_err_print_and_look_for_blank_line($ctx, $buffers, $pool)?;
            return Ok(());
        }
    };
    ('}', $ctx:expr, $buffers:expr, $pool:expr, $name:literal) => {
        if $buffers.at_offset(BufTy::Base, 2) != b'}' {
            bst_right_brace_print();
            write_logs($name);
            rs_bst_err_print_and_look_for_blank_line($ctx, $buffers, $pool)?;
            return Ok(());
        }
    };
}

fn rs_bst_entry_command(
    ctx: &mut Bibtex,
    GlobalItems {
        buffers,
        pool,
        hash,
        entries,
        other,
        ..
    }: &mut GlobalItems<'_>,
) -> Result<(), BibtexError> {
    if ctx.entry_seen {
        write_logs("Illegal, another entry command");
        rs_bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
        return Ok(());
    }
    ctx.entry_seen = true;

    eat_bst_white!(ctx, buffers, pool, "entry");
    bst_brace!('{', ctx, buffers, pool, "entry");
    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, buffers, pool, "entry");

    while buffers.at_offset(BufTy::Base, 2) != b'}' {
        let scan_res = rs_scan_identifier(buffers, b'}', b'#', b'#');
        match scan_res {
            ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
            _ => {
                rs_bst_id_print(buffers, scan_res)?;
                write_logs("entry");
                rs_bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
                return Ok(());
            }
        }

        let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
        let bst_fn = &mut buffers.buffer_mut(BufTy::Base)[range];
        bst_fn.make_ascii_lowercase();

        let res = pool.lookup_str_insert(hash, bst_fn, StrIlk::BstFn)?;
        if res.exists {
            rs_already_seen_function_print(ctx, buffers, pool, hash, res.loc)?;
            return Ok(());
        }

        hash.set_ty(res.loc, FnClass::Field);
        hash.set_ilk_info(res.loc, other.num_fields() as i32);
        other.set_num_fields(other.num_fields() + 1);

        eat_bst_white!(ctx, buffers, pool, "entry");
    }

    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, buffers, pool, "entry");

    if other.num_fields() == other.pre_defined_fields() {
        write_logs("Warning--I didn't find any fields");
        bst_warn_print(ctx, pool)?;
    }

    bst_brace!('{', ctx, buffers, pool, "entry");
    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, buffers, pool, "entry");

    while buffers.at_offset(BufTy::Base, 2) != b'}' {
        let scan_res = rs_scan_identifier(buffers, b'}', b'#', b'#');
        match scan_res {
            ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
            _ => {
                rs_bst_id_print(buffers, scan_res)?;
                write_logs("entry");
                rs_bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
                return Ok(());
            }
        }

        let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
        let bst_fn = &mut buffers.buffer_mut(BufTy::Base)[range];
        bst_fn.make_ascii_lowercase();

        let res = pool.lookup_str_insert(hash, bst_fn, StrIlk::BstFn)?;
        if res.exists {
            rs_already_seen_function_print(ctx, buffers, pool, hash, res.loc)?;
            return Ok(());
        }

        hash.set_ty(res.loc, FnClass::IntEntryVar);
        hash.set_ilk_info(res.loc, entries.num_ent_ints() as i32);
        entries.set_num_ent_ints(entries.num_ent_ints() + 1);

        eat_bst_white!(ctx, buffers, pool, "entry");
    }

    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, buffers, pool, "entry");
    bst_brace!('{', ctx, buffers, pool, "entry");
    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx, buffers, pool, "entry");

    while buffers.at_offset(BufTy::Base, 2) != b'}' {
        let scan_res = rs_scan_identifier(buffers, b'}', b'#', b'#');
        match scan_res {
            ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
            _ => {
                rs_bst_id_print(buffers, scan_res)?;
                write_logs("entry");
                rs_bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
                return Ok(());
            }
        }

        let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
        let bst_fn = &mut buffers.buffer_mut(BufTy::Base)[range];
        bst_fn.make_ascii_lowercase();

        let res = pool.lookup_str_insert(hash, bst_fn, StrIlk::BstFn)?;
        if res.exists {
            rs_already_seen_function_print(ctx, buffers, pool, hash, res.loc)?;
            return Ok(());
        }

        hash.set_ty(res.loc, FnClass::StrEntryVar);
        hash.set_ilk_info(res.loc, entries.num_ent_strs() as i32);
        entries.set_num_ent_strs(entries.num_ent_strs() + 1);

        eat_bst_white!(ctx, buffers, pool, "entry");
    }
    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

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
    eat_bst_white!(ctx.glbl_ctx_mut(), globals.buffers, globals.pool, "execute");
    bst_brace!(
        '{',
        ctx.glbl_ctx_mut(),
        globals.buffers,
        globals.pool,
        "execute"
    );
    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
    eat_bst_white!(ctx.glbl_ctx_mut(), globals.buffers, globals.pool, "execute");

    let scan_res = rs_scan_identifier(globals.buffers, b'}', b'#', b'#');
    match scan_res {
        ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent => (),
        _ => {
            rs_bst_id_print(globals.buffers, scan_res)?;
            write_logs("execute");
            rs_bst_err_print_and_look_for_blank_line(
                ctx.glbl_ctx_mut(),
                globals.buffers,
                globals.pool,
            )?;
            return Ok(());
        }
    }

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

    eat_bst_white!(ctx.glbl_ctx_mut(), globals.buffers, globals.pool, "execute");
    bst_brace!(
        '}',
        ctx.glbl_ctx_mut(),
        globals.buffers,
        globals.pool,
        "execute"
    );
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
pub unsafe extern "C" fn bst_entry_command(ctx: *mut Bibtex) -> CResult {
    GlobalItems::with_globals(|globals| rs_bst_entry_command(&mut *ctx, globals)).into()
}

#[no_mangle]
pub unsafe extern "C" fn bst_execute_command(ctx: *mut ExecCtx) -> CResult {
    GlobalItems::with_globals(|globals| rs_bst_execute_command(&mut *ctx, globals)).into()
}
