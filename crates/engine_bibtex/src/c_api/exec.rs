use crate::c_api::cite::with_cites;
use crate::c_api::history::mark_error;
use crate::c_api::log::{print_a_pool_str, print_confusion, write_logs};
use crate::c_api::peekable::PeekableInput;
use crate::c_api::StrNumber;

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
pub extern "C" fn print_wrong_stk_lit(hash_text: *const StrNumber, ctx: *mut ExecCtx, val: ExecVal, typ2: StkType) -> bool {
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
            return false
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
        let res = with_cites(|ci| {
            print_a_pool_str(ci.get_cite(ci.ptr() as usize))
        });
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
    write_logs(&format!("--line {} of file ", unsafe { (*bst_ctx).bst_line_num }));
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

pub fn illegl_literal_confusion() {
    write_logs("Illegal literal type");
    print_confusion();
}
