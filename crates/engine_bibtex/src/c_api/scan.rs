use crate::{
    c_api::{
        bibs::{compress_bib_white, rs_eat_bib_white_space, with_bibs_mut},
        buffer::{with_buffers_mut, BufTy, GlobalBuffer},
        char_info::{IdClass, LexClass},
        cite::{rs_add_database_cite, with_cites_mut},
        exec::ExecCtx,
        hash,
        hash::{with_hash_mut, FnClass, HashData},
        log::{
            bib_cmd_confusion, bib_unbalanced_braces_print, braces_unbalanced_complaint,
            eat_bst_print, hash_cite_confusion, macro_warn_print, print_confusion,
            print_recursion_illegal, rs_bib_err_print, rs_bib_id_print, rs_bib_warn_print,
            rs_bst_err_print_and_look_for_blank_line, rs_eat_bib_print, rs_print_a_pool_str,
            skip_illegal_stuff_after_token_print, skip_token_print,
            skip_token_unknown_function_print, write_log_file, write_logs,
        },
        other::with_other_mut,
        peekable::rs_input_ln,
        pool::{with_pool_mut, StringPool},
        ASCIICode, Bibtex, BufPointer, CResult, CResultBool, CiteNumber, FnDefLoc, HashPointer,
        StrIlk, StrNumber,
    },
    BibtexError,
};

pub const QUOTE_NEXT_FN: usize = hash::HASH_BASE - 1;

/// cbindgen:rename-all=ScreamingSnakeCase
#[repr(C)]
pub enum ScanRes {
    IdNull = 0,
    SpecifiedCharAdjacent = 1,
    OtherCharAdjacent = 2,
    WhitespaceAdjacent = 3,
}

#[derive(Default)]
pub struct Scan<'a> {
    chars: &'a [ASCIICode],
    not_class: Option<LexClass>,
    class: Option<LexClass>,
}

impl<'a> Scan<'a> {
    pub fn new() -> Scan<'a> {
        Scan {
            chars: &[],
            not_class: None,
            class: None,
        }
    }

    pub fn chars(mut self, chars: &'a [ASCIICode]) -> Scan<'a> {
        self.chars = chars;
        self
    }

    pub fn not_class(mut self, class: LexClass) -> Self {
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

    fn scan_till_nonempty(&self, buffers: &mut GlobalBuffer, last: BufPointer) -> bool {
        let start = buffers.offset(BufTy::Base, 2);
        buffers.set_offset(BufTy::Base, 1, start);

        let mut idx = start;
        while idx < last && !self.match_char(buffers.at(BufTy::Base, idx)) {
            idx += 1;
        }
        buffers.set_offset(BufTy::Base, 2, idx);

        idx - start != 0
    }
}

#[no_mangle]
pub extern "C" fn scan1(char1: ASCIICode) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new().chars(&[char1]).scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan1_white(char1: ASCIICode) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new()
            .chars(&[char1])
            .class(LexClass::Whitespace)
            .scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan2_white(char1: ASCIICode, char2: ASCIICode) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new()
            .chars(&[char1, char2])
            .class(LexClass::Whitespace)
            .scan_till(buffers, last)
    })
}

#[no_mangle]
pub extern "C" fn scan_alpha() -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        Scan::new()
            .not_class(LexClass::Alpha)
            .scan_till_nonempty(buffers, last)
    })
}

fn rs_scan_identifier(
    buffers: &mut GlobalBuffer,
    char1: ASCIICode,
    char2: ASCIICode,
    char3: ASCIICode,
) -> ScanRes {
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
}

#[no_mangle]
pub extern "C" fn scan_identifier(char1: ASCIICode, char2: ASCIICode, char3: ASCIICode) -> ScanRes {
    with_buffers_mut(|buffers| rs_scan_identifier(buffers, char1, char2, char3))
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
        if Scan::new()
            .not_class(LexClass::Whitespace)
            .scan_till(buffers, init)
            && buffers.at_offset(BufTy::Base, 2) != b'%'
        {
            return true;
        }

        // SAFETY: bst_file guarantee valid if non-null
        let bst_file = unsafe { ctx.bst_file.map(|mut ptr| ptr.as_mut()) };
        if !rs_input_ln(bst_file, buffers) {
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
    hash: &mut HashData,
    pool: &mut StringPool,
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
                return skip_token_print(ctx, buffers, pool);
            }

            let res = {
                let str = &buffers.buffer(BufTy::Base)
                    [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
                let res = pool.lookup_str_insert(hash, str, StrIlk::Integer)?;

                if !res.exists {
                    hash.set_ty(res.loc, FnClass::IntLit);
                    hash.set_ilk_info(res.loc, token_value);
                }
                Ok(res)
            }?;

            let char = buffers.at_offset(BufTy::Base, 2);

            if buffers.offset(BufTy::Base, 2) < buffers.init(BufTy::Base)
                && LexClass::of(char) != LexClass::Whitespace
                && char != b'}'
                && char != b'%'
            {
                return skip_illegal_stuff_after_token_print(ctx, buffers, pool);
            }

            single_function.push(res.loc);
        }
        b'"' => {
            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            let init = buffers.init(BufTy::Base);
            if !Scan::new().chars(&[b'"']).scan_till(buffers, init) {
                write_logs("No `\"` to end string literal");
                return skip_token_print(ctx, buffers, pool);
            }

            let res = {
                let str = &buffers.buffer(BufTy::Base)
                    [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
                let res = pool.lookup_str_insert(hash, str, StrIlk::Text)?;
                hash.set_ty(res.loc, FnClass::StrLit);
                Ok(res)
            }?;

            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            let char = buffers.at_offset(BufTy::Base, 2);

            if buffers.offset(BufTy::Base, 2) < buffers.init(BufTy::Base)
                && LexClass::of(char) != LexClass::Whitespace
                && char != b'}'
                && char != b'%'
            {
                return skip_illegal_stuff_after_token_print(ctx, buffers, pool);
            }

            single_function.push(res.loc);
        }
        b'\'' => {
            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            let init = buffers.init(BufTy::Base);
            Scan::new()
                .chars(&[b'}', b'%'])
                .class(LexClass::Whitespace)
                .scan_till(buffers, init);

            let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
            buffers.buffer_mut(BufTy::Base)[range.clone()].make_ascii_lowercase();

            let str = &buffers.buffer(BufTy::Base)[range];
            let res = pool.lookup_str(hash, str, StrIlk::BstFn);

            if !res.exists {
                return skip_token_unknown_function_print(ctx, buffers, pool);
            } else if res.loc == wiz_loc {
                return print_recursion_illegal(ctx, buffers, pool);
            }

            single_function.push(QUOTE_NEXT_FN);
            single_function.push(res.loc);
        }
        b'{' => {
            buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

            let str = format!("'{}", ctx.impl_fn_num);

            let res = {
                let res = pool.lookup_str_insert(hash, str.as_bytes(), StrIlk::BstFn)?;

                if res.exists {
                    write_logs("Already encountered implicit function");
                    print_confusion();
                    return Err(BibtexError::Fatal);
                }
                ctx.impl_fn_num += 1;
                hash.set_ty(res.loc, FnClass::Wizard);
                Ok(res)
            }?;

            single_function.push(QUOTE_NEXT_FN);
            single_function.push(res.loc);

            inner_scan_fn_def(ctx, buffers, hash, pool, res.loc, wiz_loc)?;
        }
        _ => {
            let init = buffers.init(BufTy::Base);
            Scan::new()
                .chars(&[b'}', b'%'])
                .class(LexClass::Whitespace)
                .scan_till(buffers, init);

            let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);

            buffers.buffer_mut(BufTy::Base)[range.clone()].make_ascii_lowercase();

            let str = &buffers.buffer(BufTy::Base)[range];
            let res = pool.lookup_str(hash, str, StrIlk::BstFn);
            if !res.exists {
                return skip_token_unknown_function_print(ctx, buffers, pool);
            } else if res.loc == wiz_loc {
                return print_recursion_illegal(ctx, buffers, pool);
            }

            single_function.push(res.loc);
        }
    }
    Ok(())
}

fn inner_scan_fn_def(
    ctx: &mut Bibtex,
    buffers: &mut GlobalBuffer,
    hash: &mut HashData,
    pool: &mut StringPool,
    fn_hash_loc: HashPointer,
    wiz_loc: HashPointer,
) -> Result<(), BibtexError> {
    let ctx = &mut *ctx;
    let mut single_function = Vec::new();

    if !rs_eat_bst_white_space(ctx, buffers) {
        eat_bst_print();
        write_logs("function");
        rs_bst_err_print_and_look_for_blank_line(ctx, buffers, pool)?;
        return Ok(());
    }

    let mut char = buffers.at_offset(BufTy::Base, 2);
    while char != b'}' {
        handle_char(
            ctx,
            buffers,
            hash,
            pool,
            &mut single_function,
            wiz_loc,
            char,
        )?;

        if !rs_eat_bst_white_space(ctx, buffers) {
            eat_bst_print();
            write_logs("function");
            return rs_bst_err_print_and_look_for_blank_line(ctx, buffers, pool);
        }

        char = buffers.at_offset(BufTy::Base, 2);
    }

    single_function.push(HashData::end_of_def());

    with_other_mut(|other| {
        other.check_wiz_overflow(single_function.len());
        hash.set_ilk_info(fn_hash_loc, other.wiz_def_ptr() as i32);

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
    with_buffers_mut(|buffers| {
        with_hash_mut(|hash| {
            with_pool_mut(|pool| {
                inner_scan_fn_def(&mut *ctx, buffers, hash, pool, fn_hash_loc, wiz_loc)
            })
        })
    })
    .into()
}

fn scan_balanced_braces(
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
    store_field: bool,
    at_bib_command: bool,
    right_str_delim: ASCIICode,
) -> Result<bool, BibtexError> {
    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

    if (LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace
        || buffers.offset(BufTy::Base, 2) == buffers.init(BufTy::Base))
        && !compress_bib_white(buffers, pool, at_bib_command)?
    {
        return Ok(false);
    }

    if buffers.offset(BufTy::Ex, 1) > 1
        && buffers.at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) - 1) == b' '
        && buffers.at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) - 2) == b' '
    {
        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) - 1);
    }

    let mut brace_level = 0;

    if store_field {
        while buffers.at_offset(BufTy::Base, 2) != right_str_delim {
            match buffers.at_offset(BufTy::Base, 2) {
                b'{' => {
                    brace_level += 1;
                    if buffers.offset(BufTy::Ex, 1) >= buffers.len() {
                        write_log_file("Field filled up at '{', reallocating.\n");
                        buffers.grow_all();
                    }

                    buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b'{');
                    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);

                    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

                    if (LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace
                        || buffers.offset(BufTy::Base, 2) == buffers.init(BufTy::Base))
                        && !compress_bib_white(buffers, pool, at_bib_command)?
                    {
                        return Ok(false);
                    }

                    loop {
                        let c = buffers.at_offset(BufTy::Base, 2);
                        match c {
                            b'}' => brace_level -= 1,
                            b'{' => brace_level += 1,
                            _ => (),
                        }

                        if buffers.offset(BufTy::Ex, 1) >= buffers.len() {
                            match c {
                                b'}' | b'{' => {
                                    write_log_file(&format!(
                                        "Field filled up at '{}', reallocating.\n",
                                        c as char
                                    ));
                                }
                                _ => {
                                    write_log_file(&format!(
                                        "Field filled up at {}, reallocating.\n",
                                        buffers.offset(BufTy::Base, 2)
                                    ));
                                }
                            }

                            buffers.grow_all();
                        }

                        buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), c);
                        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);

                        buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

                        if (LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace
                            || buffers.offset(BufTy::Base, 2) == buffers.init(BufTy::Base))
                            && !compress_bib_white(buffers, pool, at_bib_command)?
                        {
                            return Ok(false);
                        }

                        if brace_level == 0 {
                            break;
                        }
                    }
                }
                b'}' => {
                    return bib_unbalanced_braces_print(buffers, pool, at_bib_command)
                        .map(|_| false);
                }
                c => {
                    if buffers.offset(BufTy::Ex, 1) >= buffers.len() {
                        write_log_file(&format!(
                            "Field filled up at {}, reallocating.\n",
                            buffers.offset(BufTy::Base, 2)
                        ));
                        buffers.grow_all();
                    }

                    buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), c);
                    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);

                    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

                    if (LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace
                        || buffers.offset(BufTy::Base, 2) == buffers.init(BufTy::Base))
                        && !compress_bib_white(buffers, pool, at_bib_command)?
                    {
                        return Ok(false);
                    }
                }
            }
        }
    } else {
        while buffers.at_offset(BufTy::Base, 2) != right_str_delim {
            match buffers.at_offset(BufTy::Base, 2) {
                b'{' => {
                    brace_level += 1;
                    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
                    if !rs_eat_bib_white_space(buffers) {
                        return rs_eat_bib_print(buffers, pool, at_bib_command).map(|_| false);
                    }
                    while brace_level > 0 {
                        let c = buffers.at_offset(BufTy::Base, 2);
                        match c {
                            b'{' => brace_level += 1,
                            b'}' => brace_level -= 1,
                            _ => (),
                        }

                        buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

                        let init = buffers.init(BufTy::Base);
                        if (c == b'{'
                            || c == b'}'
                            || !Scan::new().chars(&[b'{', b'}']).scan_till(buffers, init))
                            && !rs_eat_bib_white_space(buffers)
                        {
                            return rs_eat_bib_print(buffers, pool, at_bib_command).map(|_| false);
                        }
                    }
                }
                b'}' => {
                    return bib_unbalanced_braces_print(buffers, pool, at_bib_command)
                        .map(|_| false);
                }
                _ => {
                    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
                    let init = buffers.init(BufTy::Base);
                    if !Scan::new()
                        .chars(&[right_str_delim, b'{', b'}'])
                        .scan_till(buffers, init)
                        && !rs_eat_bib_white_space(buffers)
                    {
                        return rs_eat_bib_print(buffers, pool, at_bib_command).map(|_| false);
                    }
                }
            }
        }
    }

    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    Ok(true)
}

#[allow(clippy::too_many_arguments)]
fn scan_a_field_token_and_eat_white(
    buffers: &mut GlobalBuffer,
    hash: &HashData,
    pool: &StringPool,
    store_field: bool,
    at_bib_command: bool,
    command_num: i32,
    cur_macro_loc: HashPointer,
    right_outer_delim: ASCIICode,
) -> Result<bool, BibtexError> {
    match buffers.at_offset(BufTy::Base, 2) {
        b'{' => {
            if !scan_balanced_braces(buffers, pool, store_field, at_bib_command, b'}')? {
                return Ok(false);
            }
        }
        b'"' => {
            if !scan_balanced_braces(buffers, pool, store_field, at_bib_command, b'"')? {
                return Ok(false);
            }
        }
        b'0'..=b'9' => {
            let last = buffers.init(BufTy::Base);
            if !Scan::new()
                .not_class(LexClass::Numeric)
                .scan_till_nonempty(buffers, last)
            {
                write_logs("A digit disappeared");
                print_confusion();
                return Err(BibtexError::Fatal);
            }

            if store_field {
                let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
                let len = range.end - range.start;

                while len >= buffers.len() {
                    // TODO: This may change output of a field long enough to fill the buffer twice. OTOH, that is a 40KB field roughly
                    write_log_file(&format!(
                        "Field filled up at {}, reallocating.\n",
                        buffers.at_offset(BufTy::Base, 1)
                    ));
                    buffers.grow_all();
                }

                buffers.copy_within(
                    BufTy::Base,
                    BufTy::Ex,
                    range.start,
                    buffers.offset(BufTy::Ex, 1),
                    len,
                );

                buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + len);
            }
        }
        _ => {
            let res = rs_scan_identifier(buffers, b',', right_outer_delim, b'#');
            if !matches!(
                res,
                ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent
            ) {
                rs_bib_id_print(buffers, res)?;
                write_logs("a field part");
                rs_bib_err_print(buffers, pool, at_bib_command)?;
                return Ok(false);
            }

            if store_field {
                let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
                buffers.buffer_mut(BufTy::Base)[range.clone()].make_ascii_lowercase();
                let str = &buffers.buffer(BufTy::Base)[range];

                let res = pool.lookup_str(hash, str, StrIlk::Macro);
                let mut store_token = true;
                if at_bib_command && command_num == 2 /* n_bib_string */ && res.loc ==
                    cur_macro_loc
                {
                    store_token = false;
                    macro_warn_print(buffers);
                    write_logs("used in its own definition\n");
                    rs_bib_warn_print(pool)?;
                }

                if !res.exists {
                    store_token = false;
                    macro_warn_print(buffers);
                    write_logs("undefined\n");
                    rs_bib_warn_print(pool)?;
                }

                if store_token {
                    let strnum = hash.ilk_info(res.loc) as StrNumber;
                    let mut str = pool.get_str(strnum);

                    if buffers.offset(BufTy::Ex, 1) == 0
                        && LexClass::of(str[0]) == LexClass::Whitespace
                    {
                        if buffers.offset(BufTy::Ex, 1) >= buffers.len() {
                            write_log_file("Field filled up at ' ', reallocating.\n");
                            buffers.grow_all();
                        }

                        buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b' ');
                        buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);

                        while !str.is_empty() && LexClass::of(str[0]) == LexClass::Whitespace {
                            str = &str[1..];
                        }
                    }

                    for &c in str {
                        let msg = if LexClass::of(c) != LexClass::Whitespace {
                            |c| format!("Field filled up at {}, reallocating.\n", c as char)
                        } else {
                            |_| String::from("Field filled up at ' ', reallocating.\n")
                        };

                        if buffers.offset(BufTy::Ex, 1) >= buffers.len() {
                            write_log_file(&msg(c));
                            buffers.grow_all();
                        }

                        if LexClass::of(c) != LexClass::Whitespace {
                            buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), c);
                            buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                        } else if c != b' ' {
                            buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b' ');
                            buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                        }
                    }
                }
            }
        }
    }

    if !rs_eat_bib_white_space(buffers) {
        return rs_eat_bib_print(buffers, pool, at_bib_command).map(|_| false);
    }
    Ok(true)
}

// TODO: Refactor this to bundle up arguments into structs as relevant
#[allow(clippy::too_many_arguments)]
fn rs_scan_and_store_the_field_value_and_eat_white(
    ctx: &Bibtex,
    buffers: &mut GlobalBuffer,
    hash: &mut HashData,
    pool: &mut StringPool,
    store_field: bool,
    at_bib_command: bool,
    command_num: i32,
    cite_out: Option<&mut CiteNumber>,
    cur_macro_loc: HashPointer,
    right_outer_delim: ASCIICode,
    field_name_loc: HashPointer,
) -> Result<bool, BibtexError> {
    // Consume tokens/strings separated by #
    buffers.set_offset(BufTy::Ex, 1, 0);
    if !scan_a_field_token_and_eat_white(
        buffers,
        hash,
        pool,
        store_field,
        at_bib_command,
        command_num,
        cur_macro_loc,
        right_outer_delim,
    )? {
        return Ok(false);
    }
    while buffers.at_offset(BufTy::Base, 2) == b'#' {
        buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
        if !rs_eat_bib_white_space(buffers) {
            return rs_eat_bib_print(buffers, pool, at_bib_command).map(|_| false);
        }
        if !scan_a_field_token_and_eat_white(
            buffers,
            hash,
            pool,
            store_field,
            at_bib_command,
            command_num,
            cur_macro_loc,
            right_outer_delim,
        )? {
            return Ok(false);
        }
    }

    if store_field {
        if !at_bib_command
            && buffers.offset(BufTy::Ex, 1) > 0
            && buffers.at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) - 1) == b' '
        {
            buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) - 1);
        }

        let ex_buf_xptr = if !at_bib_command
            && buffers.at(BufTy::Ex, 0) == b' '
            && buffers.offset(BufTy::Ex, 1) > 0
        {
            1
        } else {
            0
        };

        let str = &buffers.buffer(BufTy::Ex)[ex_buf_xptr..buffers.offset(BufTy::Ex, 1)];
        let res = {
            let res = pool.lookup_str_insert(hash, str, StrIlk::Text)?;

            hash.set_ty(res.loc, FnClass::StrLit);

            Ok(res)
        }?;

        if at_bib_command {
            match command_num {
                1 => with_bibs_mut(|bibs| bibs.add_preamble(hash.text(res.loc))),
                2 => hash.set_ilk_info(cur_macro_loc, hash.text(res.loc) as i32),
                _ => {
                    // TODO: Replace command_num with an enum
                    bib_cmd_confusion();
                    return Err(BibtexError::Fatal);
                }
            }
        } else {
            with_other_mut(|other| {
                with_cites_mut(|cites| {
                    let field_ptr = cites.entry_ptr() * other.num_fields()
                        + hash.ilk_info(field_name_loc) as usize;
                    if field_ptr > other.max_fields() {
                        write_logs("field_info index is out of range");
                        print_confusion();
                        return Err(BibtexError::Fatal);
                    }

                    if other.field(field_ptr) != 0
                    /* missing */
                    {
                        write_logs("Warning--I'm ignoring ");
                        rs_print_a_pool_str(cites.get_cite(cites.entry_ptr()), pool)?;
                        write_logs("'s extra \"");
                        rs_print_a_pool_str(hash.text(field_name_loc), pool)?;
                        write_logs("\" field\n");
                        rs_bib_warn_print(pool)?;
                    } else {
                        other.set_field(field_ptr, hash.text(res.loc));
                        if hash.ilk_info(field_name_loc) as usize == other.crossref_num()
                            && !ctx.all_entries
                        {
                            let end = buffers.offset(BufTy::Ex, 1);
                            // Move Ex to Out, at the same position
                            buffers.copy_within(
                                BufTy::Ex,
                                BufTy::Out,
                                ex_buf_xptr,
                                ex_buf_xptr,
                                end - ex_buf_xptr,
                            );
                            buffers.buffer_mut(BufTy::Out)[ex_buf_xptr..end].make_ascii_lowercase();
                            let str = &buffers.buffer(BufTy::Out)[ex_buf_xptr..end];
                            let lc_res = pool.lookup_str_insert(hash, str, StrIlk::LcCite)?;
                            if let Some(cite_out) = cite_out {
                                *cite_out = lc_res.loc;
                            }
                            let cite_loc = hash.ilk_info(lc_res.loc) as usize;
                            if lc_res.exists {
                                if hash.ilk_info(cite_loc) as usize >= cites.old_num_cites() {
                                    let old_info = hash.ilk_info(cite_loc) as usize;
                                    cites.set_info(old_info, old_info + 1);
                                }
                            } else {
                                let str = &buffers.buffer(BufTy::Ex)
                                    [ex_buf_xptr..buffers.offset(BufTy::Ex, 1)];
                                let c_res = pool.lookup_str_insert(hash, str, StrIlk::Cite)?;
                                if c_res.exists {
                                    hash_cite_confusion();
                                    return Err(BibtexError::Fatal);
                                }
                                let new_ptr = rs_add_database_cite(
                                    cites,
                                    other,
                                    hash,
                                    cites.ptr(),
                                    c_res.loc,
                                    lc_res.loc,
                                );
                                cites.set_ptr(new_ptr);
                                cites.set_info(hash.ilk_info(c_res.loc) as usize, 1);
                            }
                        }
                    }

                    Ok(())
                })
            })?;
        }
    }

    Ok(true)
}

#[no_mangle]
pub unsafe extern "C" fn scan_and_store_the_field_value_and_eat_white(
    ctx: *mut Bibtex,
    store_field: bool,
    at_bib_command: bool,
    command_num: i32,
    cite_out: *mut CiteNumber,
    cur_macro_loc: HashPointer,
    right_outer_delim: ASCIICode,
    field_name_loc: HashPointer,
) -> CResultBool {
    let res = with_buffers_mut(|buffers| {
        with_hash_mut(|hash| {
            with_pool_mut(|pool| {
                rs_scan_and_store_the_field_value_and_eat_white(
                    &*ctx,
                    buffers,
                    hash,
                    pool,
                    store_field,
                    at_bib_command,
                    command_num,
                    cite_out.as_mut(),
                    cur_macro_loc,
                    right_outer_delim,
                    field_name_loc,
                )
            })
        })
    });

    res.into()
}

pub fn decr_brace_level(
    ctx: &ExecCtx,
    pool: &StringPool,
    pop_lit_var: StrNumber,
    brace_level: &mut i32,
) -> Result<(), BibtexError> {
    if *brace_level == 0 {
        braces_unbalanced_complaint(ctx, pool, pop_lit_var)?;
    } else {
        *brace_level -= 1;
    }

    Ok(())
}

pub fn check_brace_level(
    ctx: &ExecCtx,
    pool: &StringPool,
    pop_lit_var: StrNumber,
    brace_level: i32,
) -> Result<(), BibtexError> {
    if brace_level > 0 {
        braces_unbalanced_complaint(ctx, pool, pop_lit_var)?;
    }
    Ok(())
}

pub fn name_scan_for_and(
    ctx: &ExecCtx,
    pool: &StringPool,
    buffers: &mut GlobalBuffer,
    pop_lit_var: StrNumber,
    brace_level: &mut i32,
) -> Result<(), BibtexError> {
    let mut preceding_white = false;
    let mut and_found = false;

    while !and_found && buffers.offset(BufTy::Ex, 1) < buffers.init(BufTy::Ex) {
        match buffers.at_offset(BufTy::Ex, 1) {
            b'A' | b'a' => {
                buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                if preceding_white
                    && buffers.offset(BufTy::Ex, 1) <= buffers.init(BufTy::Ex).saturating_sub(3)
                    && buffers.at_offset(BufTy::Ex, 1).to_ascii_lowercase() == b'n'
                    && buffers
                        .at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) + 1)
                        .to_ascii_lowercase()
                        == b'd'
                    && LexClass::of(buffers.at(BufTy::Ex, buffers.offset(BufTy::Ex, 1) + 2))
                        == LexClass::Whitespace
                {
                    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 2);
                    and_found = true;
                }
                preceding_white = false;
            }
            b'{' => {
                *brace_level += 1;
                buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                while *brace_level > 0 && buffers.offset(BufTy::Ex, 1) < buffers.init(BufTy::Ex) {
                    match buffers.at_offset(BufTy::Ex, 1) {
                        b'{' => *brace_level += 1,
                        b'}' => *brace_level -= 1,
                        _ => (),
                    }
                    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                }
                preceding_white = false;
            }
            b'}' => {
                decr_brace_level(ctx, pool, pop_lit_var, brace_level)?;
                buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
                preceding_white = false;
            }
            _ => {
                preceding_white =
                    LexClass::of(buffers.at_offset(BufTy::Ex, 1)) == LexClass::Whitespace;
                buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);
            }
        }
    }

    check_brace_level(ctx, pool, pop_lit_var, *brace_level)
}

pub fn von_token_found(
    buffers: &GlobalBuffer,
    hash: &HashData,
    pool: &StringPool,
    name_bf_ptr: &mut BufPointer,
    name_bf_xptr: BufPointer,
) -> Result<bool, BibtexError> {
    while *name_bf_ptr < name_bf_xptr {
        let char = buffers.at(BufTy::Sv, *name_bf_ptr);
        match char {
            b'A'..=b'Z' => return Ok(false),
            b'a'..=b'z' => return Ok(true),
            b'{' => {
                let mut nm_brace_level = 1;
                *name_bf_ptr += 1;
                if *name_bf_ptr + 2 < name_bf_xptr && buffers.at(BufTy::Sv, *name_bf_ptr) == b'\\' {
                    *name_bf_ptr += 1;
                    let name_bf_yptr = *name_bf_ptr;
                    while *name_bf_ptr < name_bf_xptr
                        && LexClass::of(buffers.at(BufTy::Sv, *name_bf_ptr)) == LexClass::Alpha
                    {
                        *name_bf_ptr += 1;
                    }
                    let str = &buffers.buffer(BufTy::Sv)[name_bf_yptr..*name_bf_ptr];
                    let res = pool.lookup_str(hash, str, StrIlk::ControlSeq);
                    let ilk = res.exists.then(|| hash.ilk_info(res.loc));
                    if let Some(ilk) = ilk {
                        match ilk {
                            3 | 5 | 7 | 9 | 11 => return Ok(false),
                            0 | 1 | 2 | 4 | 6 | 8 | 10 | 12 => return Ok(true),
                            _ => {
                                write_logs("Control-sequence hash error");
                                print_confusion();
                                return Err(BibtexError::Fatal);
                            }
                        }
                    }

                    while *name_bf_ptr < name_bf_xptr && nm_brace_level > 0 {
                        let char = buffers.at(BufTy::Sv, *name_bf_ptr);
                        match char {
                            b'A'..=b'Z' => return Ok(false),
                            b'a'..=b'z' => return Ok(true),
                            b'}' => nm_brace_level -= 1,
                            b'{' => nm_brace_level += 1,
                            _ => (),
                        }
                        *name_bf_ptr += 1;
                    }
                    return Ok(false);
                } else {
                    while nm_brace_level > 0 && *name_bf_ptr < name_bf_xptr {
                        let char = buffers.at(BufTy::Sv, *name_bf_ptr);
                        match char {
                            b'{' => nm_brace_level += 1,
                            b'}' => nm_brace_level -= 1,
                            _ => (),
                        }
                        *name_bf_ptr += 1;
                    }
                }
            }
            _ => {
                *name_bf_ptr += 1;
            }
        }
    }
    Ok(false)
}

#[allow(clippy::too_many_arguments)]
pub fn von_name_ends_and_last_name_starts_stuff(
    buffers: &GlobalBuffer,
    hash: &HashData,
    pool: &StringPool,
    last_end: BufPointer,
    von_start: BufPointer,
    von_end: &mut BufPointer,
    name_bf_ptr: &mut BufPointer,
    name_bf_xptr: &mut BufPointer,
) -> Result<(), BibtexError> {
    *von_end = last_end - 1;
    while *von_end > von_start {
        *name_bf_ptr = buffers.name_tok(*von_end - 1);
        *name_bf_xptr = buffers.name_tok(*von_end);
        if von_token_found(buffers, hash, pool, name_bf_ptr, *name_bf_xptr)? {
            return Ok(());
        }
        *von_end -= 1;
    }
    Ok(())
}

pub fn enough_text_chars(
    buffers: &mut GlobalBuffer,
    enough_chars: BufPointer,
    buf_start: BufPointer,
    brace_level: &mut i32,
) -> bool {
    let mut num_text_chars = 0;
    let mut buf_cur = buf_start;

    while buf_cur < buffers.offset(BufTy::Ex, 1) && num_text_chars < enough_chars {
        buf_cur += 1;
        if buffers.at(BufTy::Ex, buf_cur - 1) == b'{' {
            *brace_level += 1;
            if *brace_level == 1
                && buf_cur < buffers.offset(BufTy::Ex, 1)
                && buffers.at(BufTy::Ex, buf_cur) == b'\\'
            {
                buf_cur += 1;
                while buf_cur < buffers.offset(BufTy::Ex, 1) && *brace_level > 0 {
                    match buffers.at(BufTy::Ex, buf_cur) {
                        b'}' => *brace_level -= 1,
                        b'{' => *brace_level += 1,
                        _ => (),
                    }
                    buf_cur += 1;
                }
            }
        } else if buffers.at(BufTy::Ex, buf_cur - 1) == b'}' {
            *brace_level -= 1;
        }
        num_text_chars += 1;
    }

    num_text_chars >= enough_chars
}
