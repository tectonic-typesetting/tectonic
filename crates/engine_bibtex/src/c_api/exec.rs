use crate::{
    c_api::{
        buffer::{with_buffers_mut, BufTy, GlobalBuffer},
        char_info::LexClass,
        cite::with_cites,
        hash::with_hash,
        history::mark_error,
        log::{
            brace_lvl_one_letters_complaint, braces_unbalanced_complaint, print_a_pool_str,
            print_confusion, write_logs,
        },
        pool::{
            bib_set_pool_ptr, bib_set_str_ptr, bib_str_ptr, bib_str_start, with_pool,
            with_pool_mut, StringPool,
        },
        scan::enough_text_chars,
        xbuf::{xrealloc_zeroed, SafelyZero},
        ASCIICode, Bibtex, BufPointer, CResult, HashPointer, PoolPointer, StrNumber,
    },
    BibtexError,
};
use std::slice;

const LIT_STK_SIZE: usize = 100;

/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum StkType {
    Integer = 0,
    String = 1,
    Function = 2,
    Missing = 3,
    // TODO: Maybe 'empty' instead?
    Illegal = 4,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct ExecVal {
    pub typ: StkType,
    pub lit: i32,
}

// SAFETY: StkType is valid as 0 because of StkType::Integer, i32 is always valid as 0
unsafe impl SafelyZero for ExecVal {}

#[repr(C)]
pub struct ExecCtx {
    pub glbl_ctx: *mut Bibtex,
    pub _default: HashPointer,
    pub pop1: ExecVal,
    pub pop2: ExecVal,
    pub pop3: ExecVal,
    // TODO: Make an XBuf after this is more encapsulated
    pub lit_stack: *mut ExecVal,
    pub lit_stk_size: usize,
    pub lit_stk_ptr: usize,

    pub mess_with_entries: bool,
    pub bib_str_ptr: StrNumber,
}

impl ExecCtx {
    fn grow_stack(&mut self) {
        let (ptr, size) = (self.lit_stack.cast(), self.lit_stk_size);
        // SAFETY: The lit_stack should be valid for lit_stk_size. We trust the C code to uphold this invariant.
        let slice = unsafe { slice::from_raw_parts_mut(ptr, size) };
        let new_stack =
            xrealloc_zeroed::<ExecVal>(slice, self.lit_stk_size + LIT_STK_SIZE).unwrap();
        self.lit_stack = (new_stack as *mut [_]).cast();
    }
}

#[no_mangle]
pub extern "C" fn print_lit(val: ExecVal) -> bool {
    match val.typ {
        StkType::Integer => {
            write_logs(&format!("{}\n", val.lit));
            true
        }
        StkType::String => {
            if !print_a_pool_str(val.lit as usize) {
                return false;
            }
            write_logs("\n");
            true
        }
        StkType::Function => {
            if !print_a_pool_str(with_hash(|hash| hash.text(val.lit as usize))) {
                return false;
            }
            write_logs("\n");
            true
        }
        StkType::Missing => {
            if !print_a_pool_str(val.lit as usize) {
                return false;
            }
            write_logs("\n");
            true
        }
        StkType::Illegal => {
            illegl_literal_confusion();
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn print_stk_lit(val: ExecVal) -> bool {
    match val.typ {
        StkType::Integer => {
            write_logs(&format!("{} is an integer literal", val.lit));
            true
        }
        StkType::String => {
            write_logs("\"");
            if !print_a_pool_str(val.lit as usize) {
                return false;
            }
            write_logs("\" is a string literal");
            true
        }
        StkType::Function => {
            write_logs("`");
            if !print_a_pool_str(with_hash(|hash| hash.text(val.lit as usize))) {
                return false;
            }
            write_logs("` is a function literal");
            true
        }
        StkType::Missing => {
            write_logs("`");
            if !print_a_pool_str(val.lit as usize) {
                return false;
            }
            write_logs("` is a missing field");
            true
        }
        StkType::Illegal => {
            illegl_literal_confusion();
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_wrong_stk_lit(
    ctx: *mut ExecCtx,
    val: ExecVal,
    typ2: StkType,
) -> bool {
    if val.typ != StkType::Illegal {
        if !print_stk_lit(val) {
            return false;
        }
        let res = match typ2 {
            StkType::Integer => {
                write_logs(", not an integer,");
                true
            }
            StkType::String => {
                write_logs(", not a string,");
                true
            }
            StkType::Function => {
                write_logs(", not a function,");
                true
            }
            StkType::Missing | StkType::Illegal => {
                illegl_literal_confusion();
                false
            }
        };
        if !res {
            return false;
        }
        bst_ex_warn_print(ctx)
    } else {
        true
    }
}

#[no_mangle]
pub unsafe extern "C" fn bst_ex_warn_print(ctx: *const ExecCtx) -> bool {
    if (*ctx).mess_with_entries {
        write_logs(" for entry ");
        let res = with_cites(|ci| print_a_pool_str(ci.get_cite(ci.ptr())));
        if !res {
            return false;
        }
    }

    write_logs("\nwhile executing-");
    bst_ln_num_print(&*(*ctx).glbl_ctx);
    mark_error();
    true
}

pub fn bst_ln_num_print(glbl_ctx: &Bibtex) -> bool {
    write_logs(&format!("--line {} of file ", glbl_ctx.bst_line_num));
    // SAFETY: Reference -> pointer makes a valid pointer
    unsafe { print_bst_name(glbl_ctx) }
}

#[no_mangle]
pub unsafe extern "C" fn print_bst_name(glbl_ctx: *const Bibtex) -> bool {
    if !print_a_pool_str((*glbl_ctx).bst_str) {
        return false;
    }
    write_logs(".bst\n");
    true
}

fn rs_push_lit_stk(ctx: &mut ExecCtx, val: ExecVal) {
    // SAFETY: Length guaranteed to be >= lit_stk_ptr
    unsafe { *ctx.lit_stack.add(ctx.lit_stk_ptr) = val };

    if ctx.lit_stk_ptr >= ctx.lit_stk_size {
        ctx.grow_stack();
    }

    ctx.lit_stk_ptr += 1;
}

#[no_mangle]
pub unsafe extern "C" fn push_lit_stk(ctx: *mut ExecCtx, val: ExecVal) {
    rs_push_lit_stk(&mut *ctx, val)
}

pub fn rs_pop_lit_stk(ctx: &mut ExecCtx) -> Result<ExecVal, BibtexError> {
    if ctx.lit_stk_ptr == 0 {
        write_logs("You can't pop an empty literal stack");
        // SAfETY: ctx guaranteed valid
        if unsafe { !bst_ex_warn_print(ctx) } {
            return Err(BibtexError::Fatal);
        }
        Ok(ExecVal {
            lit: 0,
            typ: StkType::Illegal,
        })
    } else {
        ctx.lit_stk_ptr -= 1;
        // SAFETY: lit_stack length guaranteed >= lit_stk_ptr
        let pop = unsafe { ctx.lit_stack.add(ctx.lit_stk_ptr).read() };
        if pop.typ == StkType::String && pop.lit as usize >= ctx.bib_str_ptr {
            if pop.lit as usize != bib_str_ptr() - 1 {
                write_logs("Nontop top of string stack");
                print_confusion();
                return Err(BibtexError::Fatal);
            }
            bib_set_str_ptr(bib_str_ptr() - 1);
            bib_set_pool_ptr(bib_str_start(bib_str_ptr() as StrNumber))
        }
        Ok(pop)
    }
}

#[no_mangle]
pub unsafe extern "C" fn pop_lit_stk(ctx: *mut ExecCtx, out: *mut ExecVal) -> bool {
    let ctx = &mut *ctx;
    match rs_pop_lit_stk(ctx) {
        Ok(val) => {
            *out = val;
            true
        }
        Err(_) => false,
    }
}

pub fn illegl_literal_confusion() {
    write_logs("Illegal literal type");
    print_confusion();
}

fn rs_pop_top_and_print(ctx: &mut ExecCtx) -> Result<(), BibtexError> {
    rs_pop_lit_stk(&mut *ctx).map(|val| {
        if val.typ == StkType::Illegal {
            write_logs("Empty literal\n");
        } else {
            print_lit(val);
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn pop_top_and_print(ctx: *mut ExecCtx) -> bool {
    match rs_pop_top_and_print(&mut *ctx) {
        Ok(()) => true,
        Err(_) => false,
    }
}

fn rs_pop_whole_stack(ctx: &mut ExecCtx) -> Result<(), BibtexError> {
    while ctx.lit_stk_ptr > 0 {
        rs_pop_top_and_print(ctx)?;
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn pop_whole_stack(ctx: *mut ExecCtx) -> bool {
    match rs_pop_whole_stack(&mut *ctx) {
        Ok(()) => true,
        Err(_) => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_command_execution(ctx: *mut ExecCtx) {
    let ctx = &mut *ctx;
    ctx.lit_stk_ptr = 0;
    ctx.bib_str_ptr = with_pool(|pool| pool.str_ptr());
}

pub fn skip_brace_level_greater_than_one(
    str: &[ASCIICode],
    sp_brace_level: &mut i32,
) -> PoolPointer {
    let mut pos = 0;
    while *sp_brace_level > 1 && pos < str.len() {
        if str[pos] == b'}' {
            *sp_brace_level -= 1;
        } else if str[pos] == b'{' {
            *sp_brace_level += 1;
        }
        pos += 1;
    }
    pos
}

#[allow(clippy::too_many_arguments)]
pub fn rs_figure_out_the_formatted_name(
    ctx: &mut ExecCtx,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
    first_start: BufPointer,
    first_end: BufPointer,
    last_end: BufPointer,
    von_start: BufPointer,
    von_end: BufPointer,
    name_bf_ptr: &mut BufPointer,
    name_bf_xptr: &mut BufPointer,
    jr_end: BufPointer,
    brace_level: &mut i32,
) -> Result<(), BibtexError> {
    let mut sp_xptr1;
    let mut sp_brace_level = 0;
    let sp_str = pool.get_str(ctx.pop1.lit as usize);
    let mut sp_ptr = 0;

    buffers.set_offset(BufTy::Ex, 1, 0);

    while sp_ptr < sp_str.len() {
        if sp_str[sp_ptr] == b'{' {
            sp_brace_level += 1;
            sp_ptr += 1;
            sp_xptr1 = sp_ptr;

            let mut alpha_found = false;
            let mut double_letter = false;
            let mut end_of_group = false;
            let mut to_be_written = true;
            let mut cur_token = 0;
            let mut last_token = 0;

            while !end_of_group && sp_ptr < sp_str.len() {
                if LexClass::of(sp_str[sp_ptr]) == LexClass::Alpha {
                    sp_ptr += 1;
                    if alpha_found {
                        brace_lvl_one_letters_complaint(ctx)?;
                        to_be_written = false;
                    } else {
                        match sp_str[sp_ptr - 1] {
                            b'f' | b'F' => {
                                cur_token = first_start;
                                last_token = first_end;
                                if cur_token == last_token {
                                    to_be_written = false;
                                }
                                if sp_str[sp_ptr] == b'f' || sp_str[sp_ptr] == b'F' {
                                    double_letter = true;
                                }
                            }
                            b'v' | b'V' => {
                                cur_token = von_start;
                                last_token = von_end;
                                if cur_token == last_token {
                                    to_be_written = false;
                                }
                                if sp_str[sp_ptr] == b'v' || sp_str[sp_ptr] == b'V' {
                                    double_letter = true;
                                }
                            }
                            b'l' | b'L' => {
                                cur_token = von_end;
                                last_token = last_end;
                                if cur_token == last_token {
                                    to_be_written = false;
                                }
                                if sp_str[sp_ptr] == b'l' || sp_str[sp_ptr] == b'L' {
                                    double_letter = true;
                                }
                            }
                            b'j' | b'J' => {
                                cur_token = last_end;
                                last_token = jr_end;
                                if cur_token == last_token {
                                    to_be_written = false;
                                }
                                if sp_str[sp_ptr] == b'j' || sp_str[sp_ptr] == b'J' {
                                    double_letter = true;
                                }
                            }
                            _ => {
                                brace_lvl_one_letters_complaint(ctx)?;
                                to_be_written = false;
                                break;
                            }
                        }
                        if double_letter {
                            sp_ptr += 1;
                        }
                    }
                    alpha_found = true;
                } else if sp_str[sp_ptr] == b'}' {
                    sp_brace_level -= 1;
                    sp_ptr += 1;
                    end_of_group = true;
                } else if sp_str[sp_ptr] == b'{' {
                    sp_brace_level += 1;
                    sp_ptr = skip_brace_level_greater_than_one(
                        &sp_str[sp_ptr + 1..],
                        &mut sp_brace_level,
                    ) + sp_ptr;
                    sp_ptr += 1;
                } else {
                    sp_ptr += 1;
                }
            }

            if end_of_group && to_be_written {
                let buf_ptr = buffers.offset(BufTy::Ex, 1);
                sp_ptr = sp_xptr1;
                sp_brace_level = 1;
                while sp_brace_level > 0 {
                    if LexClass::of(sp_str[sp_ptr]) == LexClass::Alpha && sp_brace_level == 1 {
                        sp_ptr += 1;
                        if double_letter {
                            sp_ptr += 1;
                        }
                        let mut use_default = true;
                        let mut sp_xptr2 = sp_ptr;
                        if sp_str[sp_ptr] == b'{' {
                            use_default = false;
                            sp_brace_level += 1;
                            sp_ptr += 1;
                            sp_xptr1 = sp_ptr;
                            sp_ptr = skip_brace_level_greater_than_one(
                                &sp_str[sp_ptr..],
                                &mut sp_brace_level,
                            ) + sp_ptr;
                            sp_xptr2 = sp_ptr - 1;
                        }
                        while cur_token < last_token {
                            *name_bf_ptr = buffers.name_tok(cur_token);
                            *name_bf_xptr = buffers.name_tok(cur_token + 1);
                            if double_letter {
                                if buffers.init(BufTy::Ex) + (*name_bf_xptr - *name_bf_ptr)
                                    > buffers.len()
                                {
                                    buffers.grow_all();
                                }
                                // TODO: Use buffers.copy_within
                                while *name_bf_ptr < *name_bf_xptr {
                                    buffers.set_at(
                                        BufTy::Ex,
                                        buffers.offset(BufTy::Ex, 1),
                                        buffers.at(BufTy::Sv, *name_bf_ptr),
                                    );
                                    buffers.set_offset(
                                        BufTy::Ex,
                                        1,
                                        buffers.offset(BufTy::Ex, 1) + 1,
                                    );
                                    *name_bf_ptr += 1;
                                }
                            } else {
                                while *name_bf_ptr < *name_bf_xptr {
                                    if LexClass::of(buffers.at(BufTy::Sv, *name_bf_ptr))
                                        == LexClass::Alpha
                                    {
                                        if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                                            buffers.grow_all();
                                        }
                                        buffers.set_at(
                                            BufTy::Ex,
                                            buffers.offset(BufTy::Ex, 1),
                                            buffers.at(BufTy::Sv, *name_bf_ptr),
                                        );
                                        buffers.set_offset(
                                            BufTy::Ex,
                                            1,
                                            buffers.offset(BufTy::Ex, 1) + 1,
                                        );
                                        break;
                                    } else if *name_bf_ptr + 1 < *name_bf_xptr
                                        && buffers.at(BufTy::Sv, *name_bf_ptr) == b'{'
                                        && buffers.at(BufTy::Sv, *name_bf_ptr + 1) == b'\\'
                                    {
                                        if buffers.offset(BufTy::Ex, 1) + 2 > buffers.len() {
                                            buffers.grow_all();
                                        }
                                        let offset = buffers.offset(BufTy::Ex, 1);
                                        buffers.set_at(BufTy::Ex, offset, b'{');
                                        buffers.set_at(BufTy::Ex, offset + 1, b'\\');
                                        buffers.set_offset(BufTy::Ex, 1, offset + 2);
                                        *name_bf_ptr += 2;
                                        let mut nm_brace_level = 1;
                                        while *name_bf_ptr < *name_bf_xptr && nm_brace_level > 0 {
                                            if buffers.at(BufTy::Sv, *name_bf_ptr) == b'}' {
                                                nm_brace_level -= 1;
                                            } else if buffers.at(BufTy::Sv, *name_bf_ptr) == b'{' {
                                                nm_brace_level += 1;
                                            }

                                            if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                                                buffers.grow_all();
                                            }

                                            buffers.set_at(
                                                BufTy::Ex,
                                                buffers.offset(BufTy::Ex, 1),
                                                buffers.at(BufTy::Sv, *name_bf_ptr),
                                            );
                                            buffers.set_offset(
                                                BufTy::Ex,
                                                1,
                                                buffers.offset(BufTy::Ex, 1) + 1,
                                            );
                                            *name_bf_ptr += 1;
                                        }
                                        break;
                                    }
                                    *name_bf_ptr += 1;
                                }
                            }

                            cur_token += 1;
                            if cur_token < last_token {
                                if use_default {
                                    if !double_letter {
                                        if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                                            buffers.grow_all();
                                        }
                                        buffers.set_at(
                                            BufTy::Ex,
                                            buffers.offset(BufTy::Ex, 1),
                                            b'.',
                                        );
                                        buffers.set_offset(
                                            BufTy::Ex,
                                            1,
                                            buffers.offset(BufTy::Ex, 1) + 1,
                                        );
                                    }

                                    if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                                        buffers.grow_all();
                                    }

                                    let c = if LexClass::of(buffers.at(BufTy::NameSep, cur_token))
                                        == LexClass::Sep
                                    {
                                        buffers.at(BufTy::NameSep, cur_token)
                                    } else if cur_token == last_token - 1
                                        || (!enough_text_chars(buffers, 3, buf_ptr, brace_level))
                                    {
                                        b'~'
                                    } else {
                                        b' '
                                    };
                                    buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), c);
                                    buffers.set_offset(
                                        BufTy::Ex,
                                        1,
                                        buffers.offset(BufTy::Ex, 1) + 1,
                                    );
                                } else {
                                    if buffers.offset(BufTy::Ex, 1) + (sp_xptr2 - sp_xptr1)
                                        > buffers.len()
                                    {
                                        buffers.grow_all();
                                    }

                                    sp_ptr = sp_xptr1;
                                    while sp_ptr < sp_xptr2 {
                                        buffers.set_at(
                                            BufTy::Ex,
                                            buffers.offset(BufTy::Ex, 1),
                                            sp_str[sp_ptr],
                                        );
                                        buffers.set_offset(
                                            BufTy::Ex,
                                            1,
                                            buffers.offset(BufTy::Ex, 1) + 1,
                                        );
                                        sp_ptr += 1;
                                    }
                                }
                            }
                        }
                        if !use_default {
                            sp_ptr = sp_xptr2 + 1;
                        }
                    } else if sp_str[sp_ptr] == b'}' {
                        sp_brace_level -= 1;
                        sp_ptr += 1;
                        if sp_brace_level > 0 {
                            if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                                buffers.grow_all();
                            }
                            buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b'}');
                            buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                        }
                    } else if sp_str[sp_ptr] == b'{' {
                        sp_brace_level += 1;
                        sp_ptr += 1;
                        if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                            buffers.grow_all();
                        }
                        buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b'{');
                        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                    } else {
                        if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                            buffers.grow_all();
                        }
                        buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), sp_str[sp_ptr]);
                        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                        sp_ptr += 1;
                    }
                }
                if buffers.offset(BufTy::Ex, 1) > 0
                    && buffers.at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) - 1) == b'~'
                {
                    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) - 1);
                    if buffers.at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) - 1) == b'~' {
                    } else if !enough_text_chars(buffers, 3, buf_ptr, brace_level) {
                        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                    } else {
                        buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b' ');
                        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                    }
                }
            }
        } else if sp_str[sp_ptr] == b'}' {
            braces_unbalanced_complaint(ctx, ctx.pop1.lit as usize)?;
            sp_ptr += 1;
        } else {
            if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                buffers.grow_all();
            }
            buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), sp_str[sp_ptr]);
            buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
            sp_ptr += 1;
        }
    }

    if sp_brace_level > 0 {
        braces_unbalanced_complaint(ctx, ctx.pop1.lit as usize)?;
    }

    buffers.set_init(BufTy::Ex, buffers.offset(BufTy::Ex, 1));

    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn figure_out_the_formatted_name(
    ctx: *mut ExecCtx,
    first_start: BufPointer,
    first_end: BufPointer,
    last_end: BufPointer,
    von_start: BufPointer,
    von_end: BufPointer,
    name_bf_ptr: *mut BufPointer,
    name_bf_xptr: *mut BufPointer,
    jr_end: BufPointer,
    brace_level: *mut i32,
) -> CResult {
    let res = with_buffers_mut(|buffers| {
        with_pool(|pool| {
            rs_figure_out_the_formatted_name(
                &mut *ctx,
                buffers,
                pool,
                first_start,
                first_end,
                last_end,
                von_start,
                von_end,
                &mut *name_bf_ptr,
                &mut *name_bf_xptr,
                jr_end,
                &mut *brace_level,
            )
        })
    });
    res.into()
}

fn rs_check_command_execution(ctx: &mut ExecCtx) -> Result<(), BibtexError> {
    if ctx.lit_stk_ptr != 0 {
        write_logs(&format!("ptr={}, stack=\n", ctx.lit_stk_ptr));
        rs_pop_whole_stack(ctx)?;
        write_logs("---the literal stack isn't empty");
        // SAFETY: ctx guaranteed valid
        if !unsafe { bst_ex_warn_print(ctx) } {
            return Err(BibtexError::Fatal);
        }
    }
    if ctx.bib_str_ptr != with_pool(|pool| pool.str_ptr()) {
        write_logs("Nonempty empty string stack");
        print_confusion();
        return Err(BibtexError::Fatal);
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn check_command_execution(ctx: *mut ExecCtx) -> CResult {
    rs_check_command_execution(&mut *ctx).into()
}

fn rs_add_pool_buf_and_push(
    ctx: &mut ExecCtx,
    buffers: &mut GlobalBuffer,
    pool: &mut StringPool,
) -> Result<(), BibtexError> {
    buffers.set_offset(BufTy::Ex, 1, buffers.init(BufTy::Ex));
    let str = &buffers.buffer(BufTy::Ex)[0..buffers.init(BufTy::Ex)];
    rs_push_lit_stk(
        ctx,
        ExecVal {
            lit: pool.add_string_raw(str)? as i32,
            typ: StkType::String,
        },
    );
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn add_pool_buf_and_push(ctx: *mut ExecCtx) -> CResult {
    with_buffers_mut(|buffers| {
        with_pool_mut(|pool| rs_add_pool_buf_and_push(&mut *ctx, buffers, pool))
    })
    .into()
}
