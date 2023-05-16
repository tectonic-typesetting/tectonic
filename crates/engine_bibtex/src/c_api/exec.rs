use std::{mem, slice};
use crate::c_api::cite::with_cites;
use crate::c_api::history::mark_error;
use crate::c_api::log::{print_a_pool_str, print_confusion, write_logs};
use crate::c_api::peekable::PeekableInput;
use crate::c_api::{StrNumber, xcalloc_zeroed};
use crate::c_api::pool::{bib_set_pool_ptr, bib_set_str_ptr, bib_str_ptr, bib_str_start};

const LIT_STK_SIZE: usize = 100;

#[repr(C)]
pub struct BstCtx {
    bst_file: *mut PeekableInput,
    bst_str: StrNumber,
    bst_line_num: i32,
}

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
    typ: StkType,
    lit: i32,
}

#[repr(C)]
pub struct ExecCtx {
    bst_ctx: *mut BstCtx,
    pop1: ExecVal,
    pop2: ExecVal,
    pop3: ExecVal,
    lit_stack: *mut ExecVal,
    lit_stk_size: i32,
    lit_stk_ptr: i32,

    mess_with_entries: bool,
    bib_str_ptr: StrNumber,
}

impl ExecCtx {
    fn grow_stack(&mut self) {
        let new_stack = unsafe { xcalloc_zeroed::<ExecVal>(self.lit_stk_size as usize + LIT_STK_SIZE, mem::size_of::<ExecVal>()) };
        new_stack.copy_from_slice(unsafe { slice::from_raw_parts(self.lit_stack.cast(), self.lit_stk_size as usize) });
        unsafe { libc::free((self.lit_stack as *mut ExecVal).cast()) };
        self.lit_stack = (new_stack as *mut [_]).cast();
    }
}

#[no_mangle]
pub extern "C" fn print_lit(hash_text: *const StrNumber, val: ExecVal) -> bool {
    match val.typ {
        StkType::Integer => {
            write_logs(&format!("{}\n", val.lit));
            true
        }
        StkType::String => {
            if !print_a_pool_str(val.lit) {
                return false;
            }
            write_logs("\n");
            true
        }
        StkType::Function => {
            if !print_a_pool_str(unsafe { *hash_text.add(val.lit as usize) }) {
                return false;
            }
            write_logs("\n");
            true
        }
        StkType::Missing => {
            if !print_a_pool_str(val.lit) {
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
pub extern "C" fn print_stk_lit(hash_text: *const StrNumber, val: ExecVal) -> bool {
    match val.typ {
        StkType::Integer => {
            write_logs(&format!("{} is an integer literal", val.lit));
            true
        }
        StkType::String => {
            write_logs("\"");
            if !print_a_pool_str(val.lit) {
                return false;
            }
            write_logs("\" is a string literal");
            true
        }
        StkType::Function => {
            write_logs("`");
            if !print_a_pool_str(unsafe { *hash_text.add(val.lit as usize) }) {
                return false;
            }
            write_logs("` is a function literal");
            true
        }
        StkType::Missing => {
            write_logs("`");
            if !print_a_pool_str(val.lit) {
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
pub extern "C" fn print_wrong_stk_lit(
    hash_text: *const StrNumber,
    ctx: *mut ExecCtx,
    val: ExecVal,
    typ2: StkType,
) -> bool {
    if val.typ != StkType::Illegal {
        if !print_stk_lit(hash_text, val) {
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
pub extern "C" fn bst_ex_warn_print(ctx: *const ExecCtx) -> bool {
    if unsafe { (*ctx).mess_with_entries } {
        write_logs(" for entry ");
        let res = with_cites(|ci| print_a_pool_str(ci.get_cite(ci.ptr() as usize)));
        if !res {
            return false;
        }
    }

    write_logs("\nwhile executing-");
    bst_ln_num_print(unsafe { (*ctx).bst_ctx });
    mark_error();
    true
}

#[no_mangle]
pub extern "C" fn bst_ln_num_print(bst_ctx: *const BstCtx) -> bool {
    write_logs(&format!("--line {} of file ", unsafe {
        (*bst_ctx).bst_line_num
    }));
    print_bst_name(bst_ctx)
}

#[no_mangle]
pub extern "C" fn print_bst_name(bst_ctx: *const BstCtx) -> bool {
    if !print_a_pool_str(unsafe { (*bst_ctx).bst_str }) {
        return false;
    }
    write_logs(".bst\n");
    true
}

#[no_mangle]
pub unsafe extern "C" fn push_lit_stk(ctx: *mut ExecCtx, val: ExecVal) {
    let ctx = &mut *ctx;
    *ctx.lit_stack.offset(ctx.lit_stk_ptr as isize) = val;

    if ctx.lit_stk_ptr >= ctx.lit_stk_size {
        ctx.grow_stack();
    }

    ctx.lit_stk_ptr += 1;
}

#[no_mangle]
pub unsafe extern "C" fn pop_lit_stk(ctx: *mut ExecCtx, out: *mut ExecVal) -> bool {
    let ctx = &mut *ctx;

    if ctx.lit_stk_ptr == 0 {
        write_logs("You can't pop an empty literal stack");
        if !bst_ex_warn_print(ctx) {
            return false;
        }
        *out = ExecVal { lit: 0, typ: StkType::Illegal };
    } else {
        ctx.lit_stk_ptr -= 1;
        let pop = ctx.lit_stack.offset(ctx.lit_stk_ptr as isize).read();
        if pop.typ == StkType::String {
            if pop.lit >= ctx.bib_str_ptr {
                if pop.lit != (bib_str_ptr() - 1) as i32 {
                    write_logs("Nontop top of string stack");
                    print_confusion();
                    return false;
                }
                bib_set_str_ptr(bib_str_ptr() - 1);
                bib_set_pool_ptr(bib_str_start(bib_str_ptr() as StrNumber))
            }
        }
        *out = pop;
    }

    true
}

pub fn illegl_literal_confusion() {
    write_logs("Illegal literal type");
    print_confusion();
}
