use crate::c_api::hash::{end_of_def, with_hash, with_hash_mut, FnClass};
use crate::c_api::log::{
    eat_bst_print, print_confusion, print_recursion_illegal,
    rs_bst_err_print_and_look_for_blank_line, skip_illegal_stuff_after_token_print,
    skip_token_print, skip_token_unknown_function_print, write_logs,
};
use crate::c_api::other::with_other_mut;
use crate::c_api::peekable::rs_input_ln;
use crate::c_api::pool::{with_pool, with_pool_mut};
use crate::c_api::{
    buffer::{with_buffers, with_buffers_mut, BufTy, GlobalBuffer},
    char_info::{IdClass, LexClass},
    hash, ASCIICode, Bibtex, BufPointer, CResult, FnDefLoc, HashPointer, StrIlk,
};
use crate::BibtexError;

const QUOTE_NEXT_FN: usize = hash::HASH_BASE - 1;

/// cbindgen:rename-all=ScreamingSnakeCase
#[repr(C)]
pub enum ScanRes {
    IdNull = 0,
    SpecifiedCharAdjacent = 1,
    OtherCharAdjacent = 2,
    WhitespaceAdjacent = 3,
}

pub struct Scan<'a> {
    chars: &'a [ASCIICode],
    not_class: Option<LexClass>,
    class: Option<LexClass>,
}

impl<'a> Scan<'a> {
    pub fn new(chars: &'a [ASCIICode]) -> Scan<'a> {
        Scan {
            chars,
            not_class: None,
            class: None,
        }
    }

    fn not_class(mut self, class: LexClass) -> Self {
        self.not_class = Some(class);
        self
    }

    pub fn class(mut self, class: LexClass) -> Self {
        self.class = Some(class);
        self
    }

    fn match_char(&self, char: ASCIICode) -> bool {
        self.not_class
            .map_or(false, |class| LexClass::of(char) != class)
            || self
                .class
                .map_or(false, |class| LexClass::of(char) == class)
            || self.chars.contains(&char)
    }

    pub fn scan_till(&self, buffers: &mut GlobalBuffer, last: BufPointer) -> bool {
        buffers.set_offset(BufTy::Base, 1, buffers.offset(BufTy::Base, 2));

        let mut idx = buffers.offset(BufTy::Base, 2);
        while idx < last && !self.match_char(buffers.at(BufTy::Base, idx)) {
            idx += 1;
        }
        buffers.set_offset(BufTy::Base, 2, idx);

        idx < last
    }

    fn scan_till_nonempty(&self, last: BufPointer) -> bool {
        with_buffers_mut(|buffers| {
            let start = buffers.offset(BufTy::Base, 2);
            buffers.set_offset(BufTy::Base, 1, start);

            let mut idx = start;
            while idx < last && !self.match_char(buffers.at(BufTy::Base, idx)) {
                idx += 1;
            }
            buffers.set_offset(BufTy::Base, 2, idx);

            idx - start != 0
        })
    }
}

#[no_mangle]
pub extern "C" fn scan1(char1: ASCIICode) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new(&[char1]).scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan1_white(char1: ASCIICode) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new(&[char1])
            .class(LexClass::Whitespace)
            .scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan2(char1: ASCIICode, char2: ASCIICode) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new(&[char1, char2]).scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan2_white(char1: ASCIICode, char2: ASCIICode) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new(&[char1, char2])
            .class(LexClass::Whitespace)
            .scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan3(char1: ASCIICode, char2: ASCIICode, char3: ASCIICode) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new(&[char1, char2, char3]).scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan_alpha() -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[])
        .not_class(LexClass::Alpha)
        .scan_till_nonempty(last)
}

#[no_mangle]
pub extern "C" fn scan_white_space() -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new(&[])
            .not_class(LexClass::Whitespace)
            .scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan_identifier(char1: ASCIICode, char2: ASCIICode, char3: ASCIICode) -> ScanRes {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        let start = buffers.offset(BufTy::Base, 2);
        buffers.set_offset(BufTy::Base, 1, start);

        let mut idx = start;
        let char = buffers.at(BufTy::Base, idx);
        if LexClass::of(char) != LexClass::Numeric {
            while idx < last && IdClass::of(buffers.at(BufTy::Base, idx)) == IdClass::LegalIdChar {
                idx += 1;
            }
            buffers.set_offset(BufTy::Base, 2, idx);
        }

        let char = buffers.at(BufTy::Base, idx);
        if idx - start == 0 {
            ScanRes::IdNull
        } else if LexClass::of(char) == LexClass::Whitespace || idx == last {
            ScanRes::WhitespaceAdjacent
        } else if char == char1 || char == char2 || char == char3 {
            ScanRes::SpecifiedCharAdjacent
        } else {
            ScanRes::OtherCharAdjacent
        }
    })
}

#[no_mangle]
pub extern "C" fn scan_nonneg_integer() -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[])
        .not_class(LexClass::Numeric)
        .scan_till_nonempty(last)
}

fn scan_integer(buffers: &mut GlobalBuffer, token_value: &mut i32) -> bool {
    let last = buffers.init(BufTy::Base);
    let start = buffers.offset(BufTy::Base, 2);
    buffers.set_offset(BufTy::Base, 1, start);

    let mut idx = start;
    let sign = if buffers.at(BufTy::Base, idx) == b'-' {
        idx += 1;
        true
    } else {
        false
    };

    *token_value = 0;
    let mut char = buffers.at(BufTy::Base, idx);
    while idx < last && LexClass::of(char) == LexClass::Numeric {
        *token_value = *token_value * 10 + (char - 48) as i32;
        idx += 1;
        char = buffers.at(BufTy::Base, idx);
    }
    buffers.set_offset(BufTy::Base, 2, idx);

    if sign {
        *token_value *= -1;
    }

    idx - start != if sign { 1 } else { 0 }
}

fn rs_eat_bst_white_space(ctx: &mut Bibtex, buffers: &mut GlobalBuffer) -> bool {
    loop {
        let init = buffers.init(BufTy::Base);
        if Scan::new(&[])
            .not_class(LexClass::Whitespace)
            .scan_till(buffers, init)
            && buffers.at_offset(BufTy::Base, 2) != b'%'
        {
            return true;
        }

        if !rs_input_ln(unsafe { ctx.bst_file.map(|mut ptr| ptr.as_mut()) }, buffers) {
            return false;
        }

        ctx.bst_line_num += 1;
        buffers.set_offset(BufTy::Base, 2, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn eat_bst_white_space(ctx: *mut Bibtex) -> bool {
    with_buffers_mut(|buffers| rs_eat_bst_white_space(&mut *ctx, buffers))
}

fn handle_char(
    ctx: &mut Bibtex,
    buffers: &mut GlobalBuffer,
    single_function: &mut Vec<FnDefLoc>,
    wiz_loc: HashPointer,
    char: ASCIICode,
) -> Result<(), BibtexError> {
    match char {
        b'#' => {
            let mut token_value = 0;
            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            if !scan_integer(buffers, &mut token_value) {
                write_logs("Illegal integer in integer literal");
                return skip_token_print(ctx, buffers);
            }

            let res = with_hash_mut(|hash| {
                let str = &buffers.buffer(BufTy::Base)
                    [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
                let res = with_pool_mut(|pool| pool.lookup_str_insert(hash, str, StrIlk::Integer))?;

                if !res.exists {
                    hash.set_ty(res.loc, FnClass::IntLit);
                    hash.set_ilk_info(res.loc, token_value);
                }
                Ok(res)
            })?;

            let char = buffers.at_offset(BufTy::Base, 2);

            if buffers.offset(BufTy::Base, 2) < buffers.init(BufTy::Base)
                && LexClass::of(char) != LexClass::Whitespace
                && char != b'}'
                && char != b'%'
            {
                return skip_illegal_stuff_after_token_print(ctx, buffers);
            }

            single_function.push(res.loc);
        }
        b'"' => {
            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            let init = buffers.init(BufTy::Base);
            if !Scan::new(&[b'"']).scan_till(buffers, init) {
                write_logs("No `\"` to end string literal");
                return skip_token_print(ctx, buffers);
            }

            let res = with_hash_mut(|hash| {
                let str = &buffers.buffer(BufTy::Base)
                    [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
                let res = with_pool_mut(|pool| pool.lookup_str_insert(hash, str, StrIlk::Text))?;
                hash.set_ty(res.loc, FnClass::StrLit);
                Ok(res)
            })?;

            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            let char = buffers.at_offset(BufTy::Base, 2);

            if buffers.offset(BufTy::Base, 2) < buffers.init(BufTy::Base)
                && LexClass::of(char) != LexClass::Whitespace
                && char != b'}'
                && char != b'%'
            {
                return skip_illegal_stuff_after_token_print(ctx, buffers);
            }

            single_function.push(res.loc);
        }
        b'\'' => {
            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            let init = buffers.init(BufTy::Base);
            Scan::new(&[b'}', b'%'])
                .class(LexClass::Whitespace)
                .scan_till(buffers, init);

            let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
            buffers.buffer_mut(BufTy::Base)[range.clone()].make_ascii_lowercase();

            let str = &buffers.buffer(BufTy::Base)[range];
            let res = with_hash(|hash| with_pool(|pool| pool.lookup_str(hash, str, StrIlk::BstFn)));

            if !res.exists {
                return skip_token_unknown_function_print(ctx, buffers);
            } else if res.loc == wiz_loc {
                return print_recursion_illegal(ctx, buffers);
            }

            single_function.push(QUOTE_NEXT_FN);
            single_function.push(res.loc);
        }
        b'{' => {
            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            let str = format!("'{}", ctx.impl_fn_num);

            let res = with_hash_mut(|hash| {
                let res = with_pool_mut(|pool| {
                    pool.lookup_str_insert(hash, str.as_bytes(), StrIlk::BstFn)
                })?;

                if res.exists {
                    write_logs("Already encountered implicit function");
                    print_confusion();
                    return Err(BibtexError::Fatal);
                }
                ctx.impl_fn_num += 1;
                hash.set_ty(res.loc, FnClass::Wizard);
                Ok(res)
            })?;

            single_function.push(QUOTE_NEXT_FN);
            single_function.push(res.loc);

            inner_scan_fn_def(ctx, buffers, res.loc, wiz_loc)?;
        }
        _ => {
            let init = buffers.init(BufTy::Base);
            Scan::new(&[b'}', b'%'])
                .class(LexClass::Whitespace)
                .scan_till(buffers, init);

            let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);

            buffers.buffer_mut(BufTy::Base)[range.clone()].make_ascii_lowercase();

            let str = &buffers.buffer(BufTy::Base)[range];
            let res = with_hash(|hash| with_pool(|pool| pool.lookup_str(hash, str, StrIlk::BstFn)));
            if !res.exists {
                return skip_token_unknown_function_print(ctx, buffers);
            } else if res.loc == wiz_loc {
                return print_recursion_illegal(ctx, buffers);
            }

            single_function.push(res.loc);
        }
    }
    Ok(())
}

fn inner_scan_fn_def(
    ctx: &mut Bibtex,
    buffers: &mut GlobalBuffer,
    fn_hash_loc: HashPointer,
    wiz_loc: HashPointer,
) -> Result<(), BibtexError> {
    let ctx = &mut *ctx;
    let mut single_function = Vec::new();

    if !rs_eat_bst_white_space(ctx, buffers) {
        eat_bst_print();
        write_logs("function");
        rs_bst_err_print_and_look_for_blank_line(ctx, buffers)?;
        return Ok(());
    }

    let mut char = buffers.at_offset(BufTy::Base, 2);
    while char != b'}' {
        handle_char(ctx, buffers, &mut single_function, wiz_loc, char)?;

        if !rs_eat_bst_white_space(ctx, buffers) {
            eat_bst_print();
            write_logs("function");
            return rs_bst_err_print_and_look_for_blank_line(ctx, buffers);
        }

        char = buffers.at_offset(BufTy::Base, 2);
    }

    single_function.push(end_of_def());

    with_other_mut(|other| {
        other.check_wiz_overflow(single_function.len());
        with_hash_mut(|hash| {
            hash.set_ilk_info(fn_hash_loc, other.wiz_def_ptr() as i32);
        });

        for ptr in single_function {
            let wiz_ptr = other.wiz_def_ptr();
            other.set_wiz_function(wiz_ptr, ptr);
            other.set_wiz_def_ptr(wiz_ptr + 1);
        }
    });

    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn scan_fn_def(
    ctx: *mut Bibtex,
    fn_hash_loc: HashPointer,
    wiz_loc: HashPointer,
) -> CResult {
    with_buffers_mut(
        |buffers| match inner_scan_fn_def(&mut *ctx, buffers, fn_hash_loc, wiz_loc) {
            Ok(()) => CResult::Ok,
            Err(BibtexError::Fatal) => CResult::Error,
            Err(BibtexError::Recover) => CResult::Recover,
        },
    )
}
