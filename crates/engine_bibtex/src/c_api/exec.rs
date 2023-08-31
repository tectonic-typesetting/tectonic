use crate::{
    c_api::{
        bibs::{with_bibs_mut, BibData},
        buffer::{with_buffers_mut, BufTy, GlobalBuffer},
        char_info::LexClass,
        cite::{with_cites, CiteInfo},
        entries::{with_entries_mut, EntryData, ENT_STR_SIZE},
        global::{with_globals_mut, GlobalData, GLOB_STR_SIZE},
        hash::{with_hash, with_hash_mut, FnClass, HashData},
        history::mark_error,
        log::{
            brace_lvl_one_letters_complaint, braces_unbalanced_complaint,
            bst_1print_string_size_exceeded, bst_2print_string_size_exceeded, print_confusion,
            rs_bst_cant_mess_with_entries_print, rs_print_a_pool_str, rs_print_fn_class,
            write_logs,
        },
        pool::{rs_add_buf_pool, with_pool, with_pool_mut, StringPool},
        scan::{
            enough_text_chars, name_scan_for_and, rs_check_brace_level, rs_decr_brace_level,
            von_name_ends_and_last_name_starts_stuff, von_token_found,
        },
        xbuf::{SafelyZero, XBuf},
        ASCIICode, Bibtex, BufPointer, CResult, HashPointer, PoolPointer, StrIlk, StrNumber,
    },
    BibtexError,
};

const LIT_STK_SIZE: usize = 100;

/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum StkType {
    Integer = 0,
    String = 1,
    Function = 2,
    Missing = 3,
    Illegal = 4,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ExecVal {
    Integer(i32) = 0,
    String(StrNumber) = 1,
    Function(HashPointer) = 2,
    Missing(StrNumber) = 3,
    Illegal = 4,
}

impl ExecVal {
    fn empty() -> ExecVal {
        ExecVal::Integer(0)
    }

    #[no_mangle]
    pub extern "C" fn int_val(lit: i32) -> ExecVal {
        ExecVal::Integer(lit)
    }

    #[no_mangle]
    pub extern "C" fn str_val(str: StrNumber) -> ExecVal {
        ExecVal::String(str)
    }

    #[no_mangle]
    pub extern "C" fn func_val(f: HashPointer) -> ExecVal {
        ExecVal::Function(f)
    }

    #[no_mangle]
    pub extern "C" fn missing_val(f: StrNumber) -> ExecVal {
        ExecVal::Missing(f)
    }

    pub fn unwrap_str(&self) -> StrNumber {
        if let ExecVal::String(s) = self {
            *s
        } else {
            panic!("Expected ExecVal to be a string, got something else")
        }
    }

    pub fn ty(&self) -> StkType {
        match self {
            ExecVal::Integer(_) => StkType::Integer,
            ExecVal::String(_) => StkType::String,
            ExecVal::Function(_) => StkType::Function,
            ExecVal::Missing(_) => StkType::Missing,
            ExecVal::Illegal => StkType::Illegal,
        }
    }
}

// SAFETY: We require our zero discriminant to be an integer, which is valid for any bit pattern, including 0
unsafe impl SafelyZero for ExecVal {}

#[repr(C)]
pub struct ExecCtx {
    pub glbl_ctx: *mut Bibtex,
    pub _default: HashPointer,
    pub pop1: ExecVal,
    pub pop2: ExecVal,
    pub pop3: ExecVal,
    pub lit_stack: Box<XBuf<ExecVal>>,
    pub lit_stk_ptr: usize,
    pub mess_with_entries: bool,
    /// Pointer to the current top of the string pool, used to optimized certain string operations
    pub bib_str_ptr: StrNumber,
}

impl ExecCtx {
    pub fn push_stack(&mut self, val: ExecVal) {
        self.lit_stack[self.lit_stk_ptr] = val;

        if self.lit_stk_ptr >= self.lit_stack.len() {
            self.grow_stack();
        }

        self.lit_stk_ptr += 1;
    }

    pub fn pop_stack(&mut self, pool: &mut StringPool) -> Result<ExecVal, BibtexError> {
        if self.lit_stk_ptr == 0 {
            write_logs("You can't pop an empty literal stack");
            bst_ex_warn_print(self, pool)?;
            Ok(ExecVal::Illegal)
        } else {
            self.lit_stk_ptr -= 1;
            let pop = self.lit_stack[self.lit_stk_ptr];
            if let ExecVal::String(str) = pop {
                if str >= self.bib_str_ptr {
                    if str != pool.str_ptr() - 1 {
                        write_logs("Nontop top of string stack");
                        print_confusion();
                        return Err(BibtexError::Fatal);
                    }
                    pool.set_str_ptr(pool.str_ptr() - 1);
                    pool.set_pool_ptr(pool.str_start(pool.str_ptr()));
                }
            }
            Ok(pop)
        }
    }

    fn grow_stack(&mut self) {
        self.lit_stack.grow(LIT_STK_SIZE);
    }

    pub(crate) fn glbl_ctx(&self) -> &Bibtex {
        unsafe { &*self.glbl_ctx }
    }
}

#[no_mangle]
pub extern "C" fn init_exec_ctx(glbl_ctx: *mut Bibtex) -> ExecCtx {
    ExecCtx {
        glbl_ctx,
        _default: 0,
        pop1: ExecVal::empty(),
        pop2: ExecVal::empty(),
        pop3: ExecVal::empty(),
        lit_stack: Box::new(XBuf::new(LIT_STK_SIZE + 1)),
        lit_stk_ptr: 0,
        mess_with_entries: false,
        bib_str_ptr: 0,
    }
}

pub fn rs_print_lit(pool: &StringPool, hash: &HashData, val: ExecVal) -> Result<(), BibtexError> {
    match val {
        ExecVal::Integer(val) => {
            write_logs(&format!("{}\n", val));
        }
        ExecVal::String(str) => {
            rs_print_a_pool_str(str, pool)?;
            write_logs("\n");
        }
        ExecVal::Function(f) => {
            rs_print_a_pool_str(hash.text(f), pool)?;
            write_logs("\n");
        }
        ExecVal::Missing(s) => {
            rs_print_a_pool_str(s, pool)?;
            write_logs("\n");
        }
        ExecVal::Illegal => {
            illegl_literal_confusion();
            return Err(BibtexError::Fatal);
        }
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn print_lit(val: ExecVal) -> CResult {
    with_pool(|pool| with_hash(|hash| rs_print_lit(pool, hash, val))).into()
}

pub fn print_stk_lit(val: ExecVal, pool: &StringPool) -> Result<(), BibtexError> {
    match val {
        ExecVal::Integer(val) => write_logs(&format!("{} is an integer literal", val)),
        ExecVal::String(str) => {
            write_logs("\"");
            rs_print_a_pool_str(str as usize, pool)?;
            write_logs("\" is a string literal");
        }
        ExecVal::Function(f) => {
            write_logs("`");
            rs_print_a_pool_str(with_hash(|hash| hash.text(f as usize)), pool)?;
            write_logs("` is a function literal");
        }
        ExecVal::Missing(s) => {
            write_logs("`");
            rs_print_a_pool_str(s as usize, pool)?;
            write_logs("` is a missing field");
        }
        ExecVal::Illegal => {
            illegl_literal_confusion();
            return Err(BibtexError::Fatal);
        }
    }
    Ok(())
}

pub fn rs_print_wrong_stk_lit(
    ctx: &mut ExecCtx,
    pool: &StringPool,
    val: ExecVal,
    typ2: StkType,
) -> Result<(), BibtexError> {
    match val {
        ExecVal::Illegal => Ok(()),
        _ => {
            print_stk_lit(val, pool)?;

            match typ2 {
                StkType::Integer => write_logs(", not an integer,"),
                StkType::String => write_logs(", not a string,"),
                StkType::Function => write_logs(", not a function,"),
                StkType::Missing | StkType::Illegal => {
                    illegl_literal_confusion();
                    return Err(BibtexError::Fatal);
                }
            };

            bst_ex_warn_print(&*ctx, pool)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_wrong_stk_lit(
    ctx: *mut ExecCtx,
    val: ExecVal,
    typ2: StkType,
) -> CResult {
    with_pool(|pool| rs_print_wrong_stk_lit(&mut *ctx, pool, val, typ2)).into()
}

pub fn bst_ex_warn_print(ctx: &ExecCtx, pool: &StringPool) -> Result<(), BibtexError> {
    if ctx.mess_with_entries {
        write_logs(" for entry ");
        with_cites(|ci| rs_print_a_pool_str(ci.get_cite(ci.ptr()), pool))?;
    }

    write_logs("\nwhile executing-");
    // SAFETY: The `Bibtex` value pointed to by `ExecCtx` is guaranteed valid
    bst_ln_num_print(unsafe { &*ctx.glbl_ctx }, pool)?;
    mark_error();
    Ok(())
}

pub fn bst_ln_num_print(glbl_ctx: &Bibtex, pool: &StringPool) -> Result<(), BibtexError> {
    write_logs(&format!("--line {} of file ", glbl_ctx.bst_line_num));
    rs_print_bst_name(glbl_ctx, pool)
}

pub fn rs_print_bst_name(glbl_ctx: &Bibtex, pool: &StringPool) -> Result<(), BibtexError> {
    rs_print_a_pool_str(glbl_ctx.bst_str, pool)?;
    write_logs(".bst\n");
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn print_bst_name(glbl_ctx: *const Bibtex) -> CResult {
    with_pool(|pool| rs_print_bst_name(&*glbl_ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn push_lit_stk(ctx: *mut ExecCtx, val: ExecVal) {
    (&mut *ctx).push_stack(val)
}

#[no_mangle]
pub unsafe extern "C" fn pop_lit_stk(ctx: *mut ExecCtx, out: *mut ExecVal) -> CResult {
    let ctx = &mut *ctx;
    match with_pool_mut(|pool| ctx.pop_stack(pool)) {
        Ok(val) => {
            *out = val;
            CResult::Ok
        }
        Err(BibtexError::Fatal) => CResult::Error,
        Err(BibtexError::Recover) => CResult::Recover,
    }
}

pub fn illegl_literal_confusion() {
    write_logs("Illegal literal type");
    print_confusion();
}

fn rs_pop_top_and_print(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    hash: &HashData,
) -> Result<(), BibtexError> {
    ctx.pop_stack(pool).and_then(|val| {
        if let ExecVal::Illegal = val {
            write_logs("Empty literal\n");
            Ok(())
        } else {
            rs_print_lit(pool, hash, val)
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn pop_top_and_print(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| with_hash(|hash| rs_pop_top_and_print(&mut *ctx, pool, hash))).into()
}

fn rs_pop_whole_stack(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    hash: &HashData,
) -> Result<(), BibtexError> {
    while ctx.lit_stk_ptr > 0 {
        rs_pop_top_and_print(ctx, pool, hash)?;
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn pop_whole_stack(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| with_hash(|hash| rs_pop_whole_stack(&mut *ctx, pool, hash))).into()
}

#[no_mangle]
pub unsafe extern "C" fn init_command_execution(ctx: *mut ExecCtx) {
    let ctx = &mut *ctx;
    ctx.lit_stk_ptr = 0;
    ctx.bib_str_ptr = with_pool(|pool| pool.str_ptr());
}

pub fn skip_brace_level_greater_than_one(str: &[ASCIICode], brace_level: &mut i32) -> PoolPointer {
    let mut pos = 0;
    while *brace_level > 1 && pos < str.len() {
        if str[pos] == b'}' {
            *brace_level -= 1;
        } else if str[pos] == b'{' {
            *brace_level += 1;
        }
        pos += 1;
    }
    pos
}

#[allow(clippy::too_many_arguments)]
pub fn figure_out_the_formatted_name(
    ctx: &mut ExecCtx,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
    str: StrNumber,
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
    let mut old_idx;
    let mut inner_brace_level = 0;
    let str = pool.get_str(str);
    let mut idx = 0;

    buffers.set_offset(BufTy::Ex, 1, 0);

    while idx < str.len() {
        if str[idx] == b'{' {
            inner_brace_level += 1;
            idx += 1;
            old_idx = idx;

            let mut alpha_found = false;
            let mut double_letter = false;
            let mut end_of_group = false;
            let mut to_be_written = true;
            let mut cur_token = 0;
            let mut last_token = 0;

            while !end_of_group && idx < str.len() {
                if LexClass::of(str[idx]) == LexClass::Alpha {
                    idx += 1;
                    if alpha_found {
                        brace_lvl_one_letters_complaint(ctx, pool)?;
                        to_be_written = false;
                    } else {
                        match str[idx - 1] {
                            b'f' | b'F' => {
                                cur_token = first_start;
                                last_token = first_end;
                                if cur_token == last_token {
                                    to_be_written = false;
                                }
                                if str[idx] == b'f' || str[idx] == b'F' {
                                    double_letter = true;
                                }
                            }
                            b'v' | b'V' => {
                                cur_token = von_start;
                                last_token = von_end;
                                if cur_token == last_token {
                                    to_be_written = false;
                                }
                                if str[idx] == b'v' || str[idx] == b'V' {
                                    double_letter = true;
                                }
                            }
                            b'l' | b'L' => {
                                cur_token = von_end;
                                last_token = last_end;
                                if cur_token == last_token {
                                    to_be_written = false;
                                }
                                if str[idx] == b'l' || str[idx] == b'L' {
                                    double_letter = true;
                                }
                            }
                            b'j' | b'J' => {
                                cur_token = last_end;
                                last_token = jr_end;
                                if cur_token == last_token {
                                    to_be_written = false;
                                }
                                if str[idx] == b'j' || str[idx] == b'J' {
                                    double_letter = true;
                                }
                            }
                            _ => {
                                brace_lvl_one_letters_complaint(ctx, pool)?;
                                to_be_written = false;
                                break;
                            }
                        }
                        if double_letter {
                            idx += 1;
                        }
                    }
                    alpha_found = true;
                } else if str[idx] == b'}' {
                    inner_brace_level -= 1;
                    idx += 1;
                    end_of_group = true;
                } else if str[idx] == b'{' {
                    inner_brace_level += 1;
                    idx =
                        skip_brace_level_greater_than_one(&str[idx + 1..], &mut inner_brace_level)
                            + idx;
                    idx += 1;
                } else {
                    idx += 1;
                }
            }

            if end_of_group && to_be_written {
                let buf_ptr = buffers.offset(BufTy::Ex, 1);
                idx = old_idx;
                inner_brace_level = 1;
                while inner_brace_level > 0 {
                    if LexClass::of(str[idx]) == LexClass::Alpha && inner_brace_level == 1 {
                        idx += 1;
                        if double_letter {
                            idx += 1;
                        }
                        let mut use_default = true;
                        let mut sp_xptr2 = idx;
                        if str[idx] == b'{' {
                            use_default = false;
                            inner_brace_level += 1;
                            idx += 1;
                            old_idx = idx;
                            idx = skip_brace_level_greater_than_one(
                                &str[idx..],
                                &mut inner_brace_level,
                            ) + idx;
                            sp_xptr2 = idx - 1;
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
                                let ptr = buffers.offset(BufTy::Ex, 1);
                                let len = *name_bf_xptr - *name_bf_ptr;
                                buffers.copy_within(BufTy::Sv, BufTy::Ex, *name_bf_ptr, ptr, len);
                                buffers.set_offset(BufTy::Ex, 1, ptr + len);
                                *name_bf_ptr += len;
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
                                    if buffers.offset(BufTy::Ex, 1) + (sp_xptr2 - old_idx)
                                        > buffers.len()
                                    {
                                        buffers.grow_all();
                                    }

                                    let ptr = buffers.offset(BufTy::Ex, 1);
                                    let tmp_str = &str[old_idx..sp_xptr2];
                                    buffers.copy_from(BufTy::Ex, ptr, tmp_str);
                                    buffers.set_offset(BufTy::Ex, 1, ptr + tmp_str.len());
                                    idx = sp_xptr2;
                                }
                            }
                        }
                        if !use_default {
                            idx = sp_xptr2 + 1;
                        }
                    } else if str[idx] == b'}' {
                        inner_brace_level -= 1;
                        idx += 1;
                        if inner_brace_level > 0 {
                            if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                                buffers.grow_all();
                            }
                            buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b'}');
                            buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                        }
                    } else if str[idx] == b'{' {
                        inner_brace_level += 1;
                        idx += 1;
                        if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                            buffers.grow_all();
                        }
                        buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b'{');
                        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                    } else {
                        if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                            buffers.grow_all();
                        }
                        buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), str[idx]);
                        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                        idx += 1;
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
        } else if str[idx] == b'}' {
            braces_unbalanced_complaint(ctx, pool, ctx.pop1.unwrap_str())?;
            idx += 1;
        } else {
            if buffers.offset(BufTy::Ex, 1) == buffers.len() {
                buffers.grow_all();
            }
            buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), str[idx]);
            buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
            idx += 1;
        }
    }

    if inner_brace_level > 0 {
        braces_unbalanced_complaint(ctx, pool, ctx.pop1.unwrap_str())?;
    }

    buffers.set_init(BufTy::Ex, buffers.offset(BufTy::Ex, 1));

    Ok(())
}

fn rs_check_command_execution(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    hash: &HashData,
) -> Result<(), BibtexError> {
    if ctx.lit_stk_ptr != 0 {
        write_logs(&format!("ptr={}, stack=\n", ctx.lit_stk_ptr));
        rs_pop_whole_stack(ctx, pool, hash)?;
        write_logs("---the literal stack isn't empty");
        bst_ex_warn_print(ctx, pool)?;
    }
    if ctx.bib_str_ptr != pool.str_ptr() {
        write_logs("Nonempty empty string stack");
        print_confusion();
        return Err(BibtexError::Fatal);
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn check_command_execution(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| with_hash(|hash| rs_check_command_execution(&mut *ctx, pool, hash))).into()
}

fn rs_add_pool_buf_and_push(
    ctx: &mut ExecCtx,
    buffers: &mut GlobalBuffer,
    pool: &mut StringPool,
) -> Result<(), BibtexError> {
    buffers.set_offset(BufTy::Ex, 1, buffers.init(BufTy::Ex));
    let str = &buffers.buffer(BufTy::Ex)[0..buffers.init(BufTy::Ex)];
    ctx.push_stack(ExecVal::String(pool.add_string_raw(str)?));
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn add_pool_buf_and_push(ctx: *mut ExecCtx) -> CResult {
    with_buffers_mut(|buffers| {
        with_pool_mut(|pool| rs_add_pool_buf_and_push(&mut *ctx, buffers, pool))
    })
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn cur_lit(ctx: *mut ExecCtx) -> *mut ExecVal {
    core::ptr::addr_of_mut!((*ctx).lit_stack[(*ctx).lit_stk_ptr])
}

fn interp_eq(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;

    match (pop1, pop2) {
        (ExecVal::Integer(i1), ExecVal::Integer(i2)) => {
            ctx.push_stack(ExecVal::Integer((i1 == i2) as i32));
        }
        (ExecVal::String(s1), ExecVal::String(s2)) => {
            // TODO: Can we just compare str numbers here?
            ctx.push_stack(ExecVal::Integer(
                (pool.get_str(s1) == pool.get_str(s2)) as i32,
            ));
        }
        _ if pop1.ty() != pop2.ty() => {
            if pop1.ty() != StkType::Illegal && pop2.ty() != StkType::Illegal {
                print_stk_lit(pop1, pool)?;
                write_logs(", ");
                print_stk_lit(pop2, pool)?;
                write_logs("\n---they aren't the same literal types");
                bst_ex_warn_print(ctx, pool)?;
            }
            ctx.push_stack(ExecVal::Integer(0));
        }
        _ => {
            if pop1.ty() != StkType::Illegal {
                print_stk_lit(pop1, pool)?;
                write_logs(", not an integer or a string,");
                bst_ex_warn_print(ctx, pool)?;
            }
            ctx.push_stack(ExecVal::Integer(0))
        }
    }
    Ok(())
}

fn interp_gt(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;

    match (pop1, pop2) {
        (ExecVal::Integer(i1), ExecVal::Integer(i2)) => {
            ctx.push_stack(ExecVal::Integer((i2 > i1) as i32));
        }
        (ExecVal::Integer(_), _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
        (_, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::Integer)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
    }
    Ok(())
}

fn interp_lt(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;

    match (pop1, pop2) {
        (ExecVal::Integer(i1), ExecVal::Integer(i2)) => {
            ctx.push_stack(ExecVal::Integer((i2 < i1) as i32));
        }
        (ExecVal::Integer(_), _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
        (_, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::Integer)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
    }
    Ok(())
}

fn interp_plus(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;

    match (pop1, pop2) {
        (ExecVal::Integer(i1), ExecVal::Integer(i2)) => {
            ctx.push_stack(ExecVal::Integer(i2 + i1));
        }
        (ExecVal::Integer(_), _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
        (_, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::Integer)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
    }
    Ok(())
}

fn interp_minus(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;

    match (pop1, pop2) {
        (ExecVal::Integer(i1), ExecVal::Integer(i2)) => {
            ctx.push_stack(ExecVal::Integer(i2 - i1));
        }
        (ExecVal::Integer(_), _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
        (_, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::Integer)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
    }
    Ok(())
}

fn interp_concat(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;

    let (s1, s2) = match (pop1, pop2) {
        (ExecVal::String(s1), ExecVal::String(s2)) => (s1, s2),
        (ExecVal::String(_), _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
        (_, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
    };

    // A string pointer being >= bib_str_ptr means it's a 'scratch string' not yet saved permanently
    // TODO: Add pool API for scratch strings, instead of doing it manually through dangerous manual
    //       implementation of strings

    if s2 >= ctx.bib_str_ptr && s1 >= ctx.bib_str_ptr {
        // Both strings are 'scratch', they must be next to each-other due to external invariants,
        // se we just make one new string covering both
        pool.set_start(s1, pool.str_start(s1 + 1));
        pool.set_str_ptr(pool.str_ptr() + 1);
        pool.set_pool_ptr(pool.str_start(pool.str_ptr()));
        ctx.push_stack(pop2);
    } else if s2 >= ctx.bib_str_ptr {
        if pool.get_str(s2).is_empty() {
            ctx.push_stack(pop1);
        } else {
            // s2 is scratch, we add s1 to its end and return the new scratch string
            let s1_len = pool.get_str(s1).len();
            let ptr = pool.str_start(s2 + 1);
            pool.copy_raw(s1, ptr);
            pool.set_pool_ptr(ptr + s1_len);
            let new = pool.make_string()?;
            ctx.push_stack(ExecVal::String(new));
        }
    } else if s1 >= ctx.bib_str_ptr {
        let str1 = pool.get_str(s1);
        let str2 = pool.get_str(s2);

        if str2.is_empty() {
            // s1 is scratch and s2 is empty - just save s1 and return it
            pool.set_str_ptr(pool.str_ptr() + 1);
            pool.set_pool_ptr(pool.str_start(pool.str_ptr()));
            ctx.push_stack(pop1);
        } else if str1.is_empty() {
            // s1 is empty - just return s2
            ctx.push_stack(pop2);
        } else {
            let s1_len = str1.len();
            let s2_len = str2.len();

            // s1 is scratch and s2 is not - we want to copy s1 forward by the length of s2,
            // then write s2 in where it was, returning the new scratch string
            pool.copy_raw(s1, pool.str_start(s1 + 1) + s2_len - s1_len);
            pool.copy_raw(s2, pool.str_start(s1));
            pool.set_pool_ptr(pool.str_start(s1) + s1_len + s2_len);
            ctx.push_stack(ExecVal::String(pool.make_string()?));
        }
    } else {
        let str1 = pool.get_str(s1);
        let str2 = pool.get_str(s2);

        if str1.is_empty() {
            ctx.push_stack(pop2);
        } else if str2.is_empty() {
            ctx.push_stack(pop1);
        } else {
            // Neither is scratch or empty - make a new scratch string from the concat of both
            let s1_len = str1.len();
            let s2_len = str2.len();

            let ptr = pool.pool_ptr();
            pool.copy_raw(s2, ptr);
            pool.copy_raw(s1, ptr + s2_len);
            pool.set_pool_ptr(ptr + s1_len + s2_len);
            ctx.push_stack(ExecVal::String(pool.make_string()?));
        }
    }
    Ok(())
}

fn interp_gets(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    hash: &mut HashData,
    entries: &mut EntryData,
    globals: &mut GlobalData,
    cites: &CiteInfo,
) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;

    let f1 = match pop1 {
        ExecVal::Function(f1) => f1,
        _ => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::Function)?;
            return Ok(());
        }
    };

    let fn_ty = hash.ty(f1);
    if !ctx.mess_with_entries && fn_ty == FnClass::StrEntryVar || fn_ty == FnClass::IntEntryVar {
        rs_bst_cant_mess_with_entries_print(ctx, pool)?;
        return Ok(());
    }

    match fn_ty {
        FnClass::IntEntryVar => {
            if let ExecVal::Integer(i2) = pop2 {
                entries.set_int(
                    cites.ptr() * entries.num_ent_ints() + hash.ilk_info(f1) as usize,
                    i2,
                )
            } else {
                rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            }
        }
        FnClass::StrEntryVar => {
            if let ExecVal::String(s2) = pop2 {
                let mut s = pool.get_str(s2);
                if s.len() > ENT_STR_SIZE {
                    bst_1print_string_size_exceeded();
                    write_logs(&format!("{}, the entry", ENT_STR_SIZE));
                    bst_2print_string_size_exceeded(ctx, pool)?;
                    s = &s[..ENT_STR_SIZE];
                }
                entries.set_str(
                    cites.ptr() * entries.num_ent_strs() + hash.ilk_info(f1) as usize,
                    s,
                );
            } else {
                rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            }
        }
        FnClass::IntGlblVar => {
            if let ExecVal::Integer(i2) = pop2 {
                hash.set_ilk_info(f1, i2);
            } else {
                rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            }
        }
        FnClass::StrGlblVar => {
            if let ExecVal::String(s2) = pop2 {
                let str_glb_ptr = hash.ilk_info(f1) as usize;
                if s2 < ctx.bib_str_ptr {
                    globals.set_str_ptr(str_glb_ptr, s2);
                } else {
                    globals.set_str_ptr(str_glb_ptr, 0);
                    let mut s = pool.get_str(s2);
                    if s.len() > GLOB_STR_SIZE {
                        bst_1print_string_size_exceeded();
                        write_logs(&format!("{}, the global", GLOB_STR_SIZE));
                        bst_2print_string_size_exceeded(ctx, pool)?;
                        s = &s[..GLOB_STR_SIZE];
                    }
                    globals.set_str(str_glb_ptr, s);
                }
            } else {
                rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::String)?;
            }
        }
        _ => {
            write_logs("You can't assign to type ");
            rs_print_fn_class(hash, f1);
            write_logs(", a nonvariable function class");
            bst_ex_warn_print(ctx, pool)?;
        }
    }
    Ok(())
}

fn interp_add_period(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;

    let s1 = match pop1 {
        ExecVal::String(s1) => s1,
        _ => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
    };

    let str = pool.get_str(s1);

    if str.len() == 0 {
        ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
        return Ok(());
    }

    let pos = str.iter().copied().rposition(|c| c != b'}').unwrap_or(0);

    match str[pos] {
        b'.' | b'?' | b'!' => {
            // If scratch, save
            if s1 >= ctx.bib_str_ptr {
                pool.set_str_ptr(pool.str_ptr() + 1);
                pool.set_pool_ptr(pool.str_start(pool.str_ptr()));
            }
            ctx.push_stack(pop1);
        }
        _ => {
            if s1 < ctx.bib_str_ptr {
                let ptr = pool.pool_ptr();
                let s_len = str.len();
                pool.copy_raw(s1, ptr);
                pool.set_pool_ptr(ptr + s_len);
            } else {
                pool.set_pool_ptr(pool.str_start(s1 + 1));
                while pool.pool_ptr() + 1 > pool.len() {
                    pool.grow();
                }
            }
            pool.append(b'.');
            ctx.push_stack(ExecVal::String(pool.make_string()?));
        }
    }
    Ok(())
}

fn interp_change_case(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    hash: &HashData,
) -> Result<(), BibtexError> {
    #[derive(PartialEq)]
    enum ConvTy {
        TitleLower,
        AllLower,
        AllUpper,
        Bad,
    }

    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;

    match (pop1, pop2) {
        (ExecVal::String(s1), ExecVal::String(s2)) => {
            let mut prev_colon = false;

            let str1 = pool.get_str(s1);
            let conv_ty = if str1.len() == 1 {
                match str1[0] {
                    b't' | b'T' => ConvTy::TitleLower,
                    b'l' | b'L' => ConvTy::AllLower,
                    b'u' | b'U' => ConvTy::AllUpper,
                    _ => ConvTy::Bad,
                }
            } else {
                ConvTy::Bad
            };

            if conv_ty == ConvTy::Bad {
                rs_print_a_pool_str(s1, pool)?;
                write_logs(" is an illegal case-conversion string");
                bst_ex_warn_print(ctx, pool)?;
            }

            let mut scratch = Vec::from(pool.get_str(s2));

            let mut brace_level = 0;
            let mut idx = 0;
            while idx < scratch.len() {
                if scratch[idx] == b'{' {
                    brace_level += 1;
                    if !(brace_level != 1
                        || idx + 4 > scratch.len()
                        || scratch[idx + 1] != b'\\'
                        || (conv_ty == ConvTy::TitleLower
                            && (idx == 0
                                || (prev_colon
                                    && LexClass::of(scratch[idx - 1]) == LexClass::Whitespace))))
                    {
                        idx += 1;

                        while idx < scratch.len() && brace_level > 0 {
                            idx += 1;
                            let old_idx = idx;
                            while idx < scratch.len()
                                && LexClass::of(scratch[idx]) == LexClass::Alpha
                            {
                                idx += 1;
                            }

                            let res =
                                pool.lookup_str(hash, &scratch[old_idx..idx], StrIlk::ControlSeq);
                            if res.exists {
                                match conv_ty {
                                    ConvTy::TitleLower | ConvTy::AllLower => {
                                        match hash.ilk_info(res.loc) {
                                            3 | 5 | 7 | 9 | 11 => {
                                                scratch[old_idx..idx].make_ascii_lowercase()
                                            }
                                            _ => (),
                                        }
                                    }
                                    ConvTy::AllUpper => match hash.ilk_info(res.loc) {
                                        2 | 4 | 6 | 8 | 10 => {
                                            scratch[old_idx..idx].make_ascii_uppercase()
                                        }
                                        0 | 1 | 12 => {
                                            scratch[old_idx..idx].make_ascii_uppercase();
                                            scratch.copy_within(old_idx..idx, old_idx - 1);
                                            let old_idx = idx - 1;
                                            while idx < scratch.len()
                                                && LexClass::of(scratch[idx])
                                                    == LexClass::Whitespace
                                            {
                                                idx += 1;
                                            }
                                            scratch.copy_within(idx.., old_idx);
                                            scratch.truncate(scratch.len() - idx + old_idx);
                                            idx = old_idx;
                                        }
                                        _ => (),
                                    },
                                    ConvTy::Bad => (),
                                }
                            }

                            let old_idx = idx;
                            while idx < scratch.len() && brace_level > 0 && scratch[idx] != b'\\' {
                                match scratch[idx] {
                                    b'{' => brace_level += 1,
                                    b'}' => brace_level -= 1,
                                    _ => (),
                                }
                                idx += 1;
                            }

                            match conv_ty {
                                ConvTy::TitleLower | ConvTy::AllLower => {
                                    scratch[old_idx..idx].make_ascii_lowercase()
                                }
                                ConvTy::AllUpper => scratch[old_idx..idx].make_ascii_uppercase(),
                                ConvTy::Bad => (),
                            }
                        }
                        idx -= 1;
                    }

                    prev_colon = false;
                } else if scratch[idx] == b'}' {
                    rs_decr_brace_level(ctx, pool, s2, &mut brace_level)?;
                    prev_colon = false;
                } else if brace_level == 0 {
                    match conv_ty {
                        ConvTy::TitleLower => {
                            if idx != 0
                                && !(prev_colon
                                    && LexClass::of(scratch[idx - 1]) == LexClass::Whitespace)
                            {
                                scratch[idx].make_ascii_lowercase()
                            }

                            if scratch[idx] == b':' {
                                prev_colon = true;
                            } else if LexClass::of(scratch[idx]) != LexClass::Whitespace {
                                prev_colon = false;
                            }
                        }
                        ConvTy::AllLower => scratch[idx].make_ascii_lowercase(),
                        ConvTy::AllUpper => scratch[idx].make_ascii_uppercase(),
                        ConvTy::Bad => (),
                    }
                }
                idx += 1;
            }
            rs_check_brace_level(ctx, pool, s2, brace_level)?;
            ctx.push_stack(ExecVal::String(pool.add_string_raw(&scratch)?));
        }
        (ExecVal::String(_), _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
        }
        (_, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
        }
    }
    Ok(())
}

fn interp_chr_to_int(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    match pop1 {
        ExecVal::String(s1) => {
            let str = pool.get_str(s1);
            if str.len() != 1 {
                write_logs("\"");
                rs_print_a_pool_str(s1, pool)?;
                write_logs("\" isn't a single character");
                bst_ex_warn_print(ctx, pool)?;
                ctx.push_stack(ExecVal::Integer(0));
            } else {
                ctx.push_stack(ExecVal::Integer(str[0] as i32))
            }
        }
        _ => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::String)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
    }
    Ok(())
}

fn interp_cite(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    cites: &CiteInfo,
) -> Result<(), BibtexError> {
    if !ctx.mess_with_entries {
        rs_bst_cant_mess_with_entries_print(ctx, pool)?;
    } else {
        ctx.push_stack(ExecVal::String(cites.get_cite(cites.ptr())))
    }
    Ok(())
}

fn interp_dup(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    match pop1 {
        ExecVal::String(s1) => {
            ctx.push_stack(pop1);
            if s1 < ctx.bib_str_ptr {
                ctx.push_stack(pop1);
            } else {
                pool.set_str_ptr(pool.str_ptr() + 1);
                pool.set_pool_ptr(pool.str_start(pool.str_ptr()));

                let str_len = pool.get_str(s1).len();
                while pool.pool_ptr() + str_len > pool.len() {
                    pool.grow();
                }

                let ptr = pool.pool_ptr();
                pool.copy_raw(s1, ptr);
                pool.set_pool_ptr(ptr + str_len);
                ctx.push_stack(ExecVal::String(pool.make_string()?));
            }
        }
        _ => {
            ctx.push_stack(pop1);
            ctx.push_stack(pop1);
        }
    }
    Ok(())
}

fn interp_empty(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    match pop1 {
        ExecVal::String(s1) => {
            let str = pool.get_str(s1);
            let res = str.iter().all(|c| LexClass::of(*c) == LexClass::Whitespace);
            ctx.push_stack(ExecVal::Integer(res as i32));
        }
        ExecVal::Missing(_) => {
            ctx.push_stack(ExecVal::Integer(1));
        }
        ExecVal::Illegal => {
            ctx.push_stack(ExecVal::Integer(0));
        }
        _ => {
            print_stk_lit(pop1, pool)?;
            write_logs(", not a string or missing field,");
            bst_ex_warn_print(ctx, pool)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
    }
    Ok(())
}

fn interp_format_name(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    buffers: &mut GlobalBuffer,
    hash: &HashData,
) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;
    let pop3 = ctx.pop_stack(pool)?;

    let (s1, i2, s3) = match (pop1, pop2, pop3) {
        (ExecVal::String(s1), ExecVal::Integer(i2), ExecVal::String(s3)) => (s1, i2, s3),
        (ExecVal::String(_), ExecVal::Integer(_), _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop3, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
        (ExecVal::String(_), _, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
        (_, _, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
    };

    let mut brace_level = 0;
    let mut xptr = 0;
    buffers.set_init(BufTy::Ex, 0);
    rs_add_buf_pool(pool, buffers, s3);
    buffers.set_offset(BufTy::Ex, 1, 0);

    let mut num_names = 0;
    while num_names < i2 && buffers.offset(BufTy::Ex, 1) < buffers.init(BufTy::Ex) {
        num_names += 1;
        xptr = buffers.offset(BufTy::Ex, 1);
        name_scan_for_and(ctx, pool, buffers, s3, &mut brace_level)?;
    }

    if buffers.offset(BufTy::Ex, 1) < buffers.init(BufTy::Ex) {
        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) - 4);
    }

    if num_names < i2 {
        if i2 == 1 {
            write_logs("There is no name in \"");
        } else {
            write_logs(&format!("There aren't {} names in \"", i2));
        }
        rs_print_a_pool_str(s3, pool)?;
        write_logs("\"");
        bst_ex_warn_print(ctx, pool)?;
    }

    while buffers.offset(BufTy::Ex, 1) > xptr {
        match LexClass::of(buffers.at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) - 1)) {
            LexClass::Whitespace | LexClass::Sep => {
                buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) - 1);
            }
            _ => {
                if buffers.at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) - 1) == b',' {
                    write_logs(&format!("Name {} in \"", i2));
                    rs_print_a_pool_str(s3, pool)?;
                    write_logs("\" has a comma at the end");
                    bst_ex_warn_print(ctx, pool)?;
                    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) - 1);
                } else {
                    break;
                }
            }
        }
    }

    enum Commas {
        None,
        One(BufPointer),
        Two(BufPointer, BufPointer),
    }

    let mut num_tokens = 0;
    let mut commas = Commas::None;
    let mut name_ptr = 0;
    let mut token_starting = true;

    while xptr < buffers.offset(BufTy::Ex, 1) {
        match buffers.at(BufTy::Ex, xptr) {
            b',' => {
                match commas {
                    Commas::None => {
                        commas = Commas::One(num_tokens);
                        buffers.set_at(BufTy::NameSep, num_tokens, b',');
                    }
                    Commas::One(first) => {
                        commas = Commas::Two(first, num_tokens);
                        buffers.set_at(BufTy::NameSep, num_tokens, b',');
                    }
                    Commas::Two(_, _) => {
                        write_logs(&format!("Too many commas in name {} of \"", i2));
                        rs_print_a_pool_str(s3, pool)?;
                        write_logs("\"");
                        bst_ex_warn_print(ctx, pool)?;
                    }
                }
                xptr += 1;
                token_starting = true;
            }
            b'{' => {
                brace_level += 1;
                if token_starting {
                    buffers.set_name_tok(num_tokens, name_ptr);
                    num_tokens += 1;
                }
                buffers.set_at(BufTy::Sv, name_ptr, buffers.at(BufTy::Ex, xptr));
                name_ptr += 1;
                xptr += 1;
                while brace_level > 0 && xptr < buffers.offset(BufTy::Ex, 1) {
                    match buffers.at(BufTy::Ex, xptr) {
                        b'{' => brace_level += 1,
                        b'}' => brace_level -= 1,
                        _ => (),
                    }
                    buffers.set_at(BufTy::Sv, name_ptr, buffers.at(BufTy::Ex, xptr));
                    name_ptr += 1;
                    xptr += 1;
                }
                token_starting = false;
            }
            b'}' => {
                if token_starting {
                    buffers.set_name_tok(num_tokens, name_ptr);
                    num_tokens += 1;
                }

                write_logs(&format!("Name {} of \"", i2));
                rs_print_a_pool_str(s3, pool)?;
                write_logs("\" isn't brace balanced");
                bst_ex_warn_print(ctx, pool)?;
                xptr += 1;
                token_starting = false;
            }
            _ => match LexClass::of(buffers.at(BufTy::Ex, xptr)) {
                LexClass::Whitespace => {
                    if !token_starting {
                        buffers.set_at(BufTy::NameSep, num_tokens, b' ');
                    }
                    xptr += 1;
                    token_starting = true;
                }
                LexClass::Sep => {
                    if !token_starting {
                        buffers.set_at(BufTy::NameSep, num_tokens, buffers.at(BufTy::Ex, xptr));
                    }
                    xptr += 1;
                    token_starting = true;
                }
                _ => {
                    if token_starting {
                        buffers.set_name_tok(num_tokens, name_ptr);
                        num_tokens += 1;
                    }
                    buffers.set_at(BufTy::Sv, name_ptr, buffers.at(BufTy::Ex, xptr));
                    name_ptr += 1;
                    xptr += 1;
                    token_starting = false;
                }
            },
        }
    }

    buffers.set_name_tok(num_tokens, name_ptr);

    let mut first_start = 0;
    let first_end;
    let last_end;
    let mut von_start = 0;
    let mut von_end = 0;
    let jr_end;
    let mut name_ptr2 = 0;

    match commas {
        Commas::None => {
            last_end = num_tokens;
            jr_end = last_end;

            let mut second_loop = true;
            while von_start < last_end - 1 {
                name_ptr = buffers.name_tok(von_start);
                name_ptr2 = buffers.name_tok(von_start + 1);
                if von_token_found(buffers, hash, pool, &mut name_ptr, name_ptr2)? {
                    von_name_ends_and_last_name_starts_stuff(
                        buffers,
                        hash,
                        pool,
                        last_end,
                        von_start,
                        &mut von_end,
                        &mut name_ptr,
                        &mut name_ptr2,
                    )?;
                    second_loop = false;
                    break;
                }
                von_start += 1;
            }

            if second_loop {
                while von_start > 0 {
                    if LexClass::of(buffers.at(BufTy::NameSep, von_start)) != LexClass::Sep
                        || buffers.at(BufTy::NameSep, von_start) == b'~'
                    {
                        break;
                    }
                    von_start -= 1;
                }
                von_end = von_start;
            }
            first_end = von_start;
        }
        Commas::One(comma) => {
            last_end = comma;
            jr_end = last_end;
            first_start = jr_end;
            first_end = num_tokens;
            von_name_ends_and_last_name_starts_stuff(
                buffers,
                hash,
                pool,
                last_end,
                von_start,
                &mut von_end,
                &mut name_ptr,
                &mut name_ptr2,
            )?;
        }
        Commas::Two(comma1, comma2) => {
            last_end = comma1;
            jr_end = comma2;
            first_start = jr_end;
            first_end = num_tokens;
            von_name_ends_and_last_name_starts_stuff(
                buffers,
                hash,
                pool,
                last_end,
                von_start,
                &mut von_end,
                &mut name_ptr,
                &mut name_ptr2,
            )?;
        }
    }

    buffers.set_init(BufTy::Ex, 0);
    rs_add_buf_pool(pool, buffers, s1);
    figure_out_the_formatted_name(
        ctx,
        buffers,
        pool,
        s1,
        first_start,
        first_end,
        last_end,
        von_start,
        von_end,
        &mut name_ptr,
        &mut name_ptr2,
        jr_end,
        &mut brace_level,
    )?;
    rs_add_pool_buf_and_push(ctx, buffers, pool)?;

    Ok(())
}

fn interp_int_to_chr(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let i1 = match pop1 {
        ExecVal::Integer(i1) => i1,
        _ => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::Integer)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
    };

    if i1 < 0 || i1 > 127 {
        write_logs(&format!("{} isn't valid ASCII", i1));
        bst_ex_warn_print(ctx, pool)?;
        ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
    } else {
        if pool.pool_ptr() + 1 > pool.len() {
            pool.grow();
        }

        pool.append(i1 as u8);
        ctx.push_stack(ExecVal::String(pool.make_string()?));
    }
    Ok(())
}

fn interp_int_to_str(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let i1 = match pop1 {
        ExecVal::Integer(i1) => i1,
        _ => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::Integer)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
    };

    let scratch = i1.to_string();
    ctx.push_stack(ExecVal::String(pool.add_string_raw(&scratch.as_bytes())?));
    Ok(())
}

fn interp_missing(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    if !ctx.mess_with_entries {
        rs_bst_cant_mess_with_entries_print(ctx, pool)?;
        return Ok(());
    }
    match pop1 {
        ExecVal::String(_) => {
            ctx.push_stack(ExecVal::Integer(0));
        }
        ExecVal::Missing(_) => {
            ctx.push_stack(ExecVal::Integer(1));
        }
        ExecVal::Illegal => {
            ctx.push_stack(ExecVal::Integer(0));
        }
        _ => {
            print_stk_lit(pop1, pool)?;
            write_logs(", not a string or missing field,");
            bst_ex_warn_print(ctx, pool)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
    }
    Ok(())
}

fn interp_num_names(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    buffers: &mut GlobalBuffer,
) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    match pop1 {
        ExecVal::String(s1) => {
            buffers.set_init(BufTy::Ex, 0);
            rs_add_buf_pool(pool, buffers, s1);
            buffers.set_offset(BufTy::Ex, 1, 0);
            let mut num_names = 0;
            while buffers.offset(BufTy::Ex, 1) < buffers.init(BufTy::Ex) {
                let mut brace_level = 0;
                name_scan_for_and(ctx, pool, buffers, s1, &mut brace_level)?;
                num_names += 1;
            }
            ctx.push_stack(ExecVal::Integer(num_names))
        }
        _ => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::String)?;
            ctx.push_stack(ExecVal::Integer(0));
        }
    }
    Ok(())
}

fn interp_preamble(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    bibs: &mut BibData,
) -> Result<(), BibtexError> {
    let mut out = Vec::with_capacity(ctx.glbl_ctx().num_preamble_strings * 32);
    bibs.set_preamble_ptr(0);
    while bibs.preamble_ptr() < ctx.glbl_ctx().num_preamble_strings {
        out.extend(pool.get_str(bibs.cur_preamble()));
        bibs.set_preamble_ptr(bibs.preamble_ptr() + 1);
    }
    let s = pool.add_string_raw(&out)?;
    ctx.push_stack(ExecVal::String(s));
    Ok(())
}

fn interp_purify(
    ctx: &mut ExecCtx,
    pool: &mut StringPool,
    hash: &HashData,
) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let s1 = match pop1 {
        ExecVal::String(s1) => s1,
        _ => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
    };

    let mut scratch = Vec::from(pool.get_str(s1));
    let mut idx = 0;
    let mut brace_level = 0;
    let mut write_idx = 0;

    while idx < scratch.len() {
        match LexClass::of(scratch[idx]) {
            LexClass::Whitespace | LexClass::Sep => {
                scratch[write_idx] = b' ';
                write_idx += 1;
            }
            LexClass::Alpha | LexClass::Numeric => {
                scratch[write_idx] = scratch[idx];
                write_idx += 1;
            }
            _ => match scratch[idx] {
                b'{' => {
                    brace_level += 1;
                    if brace_level == 1 && idx + 1 < scratch.len() && scratch[idx + 1] == b'\\' {
                        idx += 1;
                        while idx < scratch.len() && brace_level > 0 {
                            idx += 1;
                            let old_idx = idx;
                            while idx < scratch.len()
                                && LexClass::of(scratch[idx]) == LexClass::Alpha
                            {
                                idx += 1;
                            }

                            let res =
                                pool.lookup_str(hash, &scratch[old_idx..idx], StrIlk::ControlSeq);
                            if res.exists {
                                scratch[write_idx] = scratch[old_idx];
                                write_idx += 1;
                                match hash.ilk_info(res.loc) {
                                    2 | 3 | 4 | 5 | 12 => {
                                        scratch[write_idx] = scratch[old_idx + 1];
                                        write_idx += 1;
                                    }
                                    _ => (),
                                }
                            }
                            while idx < scratch.len() && brace_level > 0 && scratch[idx] != b'\\' {
                                match LexClass::of(scratch[idx]) {
                                    LexClass::Alpha | LexClass::Numeric => {
                                        scratch[write_idx] = scratch[idx];
                                        write_idx += 1;
                                    }
                                    _ => match scratch[idx] {
                                        b'{' => brace_level += 1,
                                        b'}' => brace_level -= 1,
                                        _ => (),
                                    },
                                }
                                idx += 1;
                            }
                        }
                        idx -= 1;
                    }
                }
                b'}' => {
                    if brace_level > 0 {
                        brace_level -= 1;
                    }
                }
                _ => (),
            },
        }
        idx += 1;
    }

    scratch.truncate(write_idx);
    let out = pool.add_string_raw(&scratch)?;
    ctx.push_stack(ExecVal::String(out));

    Ok(())
}

fn interp_quote(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let s = pool.add_string_raw(b"\"")?;
    ctx.push_stack(ExecVal::String(s));
    Ok(())
}

fn interp_substr(ctx: &mut ExecCtx, pool: &mut StringPool) -> Result<(), BibtexError> {
    let pop1 = ctx.pop_stack(pool)?;
    let pop2 = ctx.pop_stack(pool)?;
    let pop3 = ctx.pop_stack(pool)?;

    let (len, start, s3) = match (pop1, pop2, pop3) {
        (ExecVal::Integer(i1), ExecVal::Integer(i2), ExecVal::String(s3)) => (i1, i2, s3),
        (ExecVal::Integer(_), ExecVal::Integer(_), _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop3, StkType::String)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
        (ExecVal::Integer(_), _, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop2, StkType::Integer)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
        (_, _, _) => {
            rs_print_wrong_stk_lit(ctx, pool, pop1, StkType::Integer)?;
            ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
            return Ok(());
        }
    };

    let str = pool.get_str(s3);
    if len as usize >= str.len() && start == 1 || start == -1 {
        if s3 >= ctx.bib_str_ptr {
            pool.set_str_ptr(pool.str_ptr() + 1);
            pool.set_pool_ptr(pool.str_start(pool.str_ptr()));
        }
        ctx.push_stack(pop3);
        return Ok(());
    }

    if len <= 0 || start == 0 || start.abs() as usize > str.len() {
        ctx.push_stack(ExecVal::String(ctx.glbl_ctx().s_null));
        return Ok(());
    }

    let len = if len as usize <= str.len() + 1 - start.abs() as usize {
        len as usize
    } else {
        str.len() + 1 - start.abs() as usize
    };

    // TODO: Remove this intermediate allocation
    let new_str = if start > 0 {
        let start = start as usize;
        if start == 1 && s3 >= ctx.bib_str_ptr {
            pool.set_start(s3 + 1, pool.str_start(s3) + len);
            pool.set_str_ptr(pool.str_ptr() + 1);
            pool.set_pool_ptr(pool.str_start(pool.str_ptr()));
            ctx.push_stack(pop3);
            return Ok(());
        }
        Vec::from(&str[(start - 1)..(start - 1) + len])
    } else {
        let start = start.abs() as usize;
        Vec::from(&str[(str.len() + 1 - start) - len..str.len() + 1 - start])
    };

    let out = pool.add_string_raw(&new_str)?;
    ctx.push_stack(ExecVal::String(out));

    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn x_equals(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_eq(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_greater_than(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_gt(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_less_than(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_lt(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_plus(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_plus(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_minus(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_minus(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_concatenate(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_concat(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_gets(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| {
        with_hash_mut(|hash| {
            with_entries_mut(|entries| {
                with_globals_mut(|globals| {
                    with_cites(|cites| interp_gets(&mut *ctx, pool, hash, entries, globals, cites))
                })
            })
        })
    })
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn x_add_period(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_add_period(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_change_case(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| with_hash(|hash| interp_change_case(&mut *ctx, pool, hash))).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_chr_to_int(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_chr_to_int(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_cite(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| with_cites(|cites| interp_cite(&mut *ctx, pool, cites))).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_duplicate(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_dup(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_empty(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_empty(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_format_name(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| {
        with_buffers_mut(|buffers| {
            with_hash(|hash| interp_format_name(&mut *ctx, pool, buffers, hash))
        })
    })
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn x_int_to_chr(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_int_to_chr(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_int_to_str(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_int_to_str(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_missing(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_missing(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_num_names(ctx: *mut ExecCtx) -> CResult {
    with_buffers_mut(|buffers| with_pool_mut(|pool| interp_num_names(&mut *ctx, pool, buffers)))
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn x_preamble(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| with_bibs_mut(|bibs| interp_preamble(&mut *ctx, pool, bibs))).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_purify(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| with_hash(|hash| interp_purify(&mut *ctx, pool, hash))).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_quote(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_quote(&mut *ctx, pool)).into()
}

#[no_mangle]
pub unsafe extern "C" fn x_substring(ctx: *mut ExecCtx) -> CResult {
    with_pool_mut(|pool| interp_substr(&mut *ctx, pool)).into()
}
