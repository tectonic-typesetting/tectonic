use crate::{
    bibs::{compress_bib_white, eat_bib_white_space, BibCommand, BibData},
    buffer::{BufTy, GlobalBuffer},
    char_info::{IdClass, LexClass},
    cite::{add_database_cite, CiteInfo},
    exec::{ControlSeq, ExecCtx},
    hash,
    hash::{BstFn, HashData, HashExtra},
    log::{
        bib_err_print, bib_id_print, bib_unbalanced_braces_print, bib_warn_print,
        braces_unbalanced_complaint, bst_err_print_and_look_for_blank_line, eat_bib_print,
        eat_bst_print, hash_cite_confusion, macro_warn_print, print_a_pool_str, print_confusion,
        print_recursion_illegal, skip_illegal_stuff_after_token_print, skip_token_print,
        skip_token_unknown_function_print,
    },
    peekable::input_ln,
    pool::StringPool,
    ASCIICode, Bibtex, BibtexError, BufPointer, CiteNumber, FnDefLoc, GlobalItems, HashPointer,
    StrIlk, StrNumber,
};

pub(crate) const QUOTE_NEXT_FN: usize = hash::HASH_BASE - 1;

pub(crate) enum ScanRes {
    IdNull,
    SpecifiedCharAdjacent,
    OtherCharAdjacent,
    WhitespaceAdjacent,
}

#[derive(Default)]
pub(crate) struct Scan<'a> {
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

    pub fn scan_till_nonempty(&self, buffers: &mut GlobalBuffer, last: BufPointer) -> bool {
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

pub(crate) fn scan_identifier(
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

pub(crate) fn eat_bst_white_space(ctx: &mut Bibtex<'_, '_>, buffers: &mut GlobalBuffer) -> bool {
    loop {
        let init = buffers.init(BufTy::Base);
        if Scan::new()
            .not_class(LexClass::Whitespace)
            .scan_till(buffers, init)
            && buffers.at_offset(BufTy::Base, 2) != b'%'
        {
            return true;
        }

        if !input_ln(ctx.engine, &mut ctx.bst.as_mut().unwrap().file, buffers) {
            return false;
        }

        ctx.bst.as_mut().unwrap().line += 1;
        buffers.set_offset(BufTy::Base, 2, 0);
    }
}

fn handle_char(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
    single_function: &mut Vec<FnDefLoc>,
    wiz_loc: HashPointer,
    char: ASCIICode,
) -> Result<(), BibtexError> {
    match char {
        b'#' => {
            let mut token_value = 0;
            globals
                .buffers
                .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

            if !scan_integer(globals.buffers, &mut token_value) {
                ctx.write_logs("Illegal integer in integer literal");
                return skip_token_print(ctx, globals.buffers, globals.pool);
            }

            let str = &globals.buffers.buffer(BufTy::Base)
                [globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2)];
            let res = globals.pool.lookup_str_insert(
                ctx,
                globals.hash,
                str,
                HashExtra::Integer(token_value),
            )?;

            let char = globals.buffers.at_offset(BufTy::Base, 2);

            if globals.buffers.offset(BufTy::Base, 2) < globals.buffers.init(BufTy::Base)
                && LexClass::of(char) != LexClass::Whitespace
                && char != b'}'
                && char != b'%'
            {
                return skip_illegal_stuff_after_token_print(ctx, globals.buffers, globals.pool);
            }

            single_function.push(res.loc);
        }
        b'"' => {
            globals
                .buffers
                .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

            let init = globals.buffers.init(BufTy::Base);
            if !Scan::new().chars(&[b'"']).scan_till(globals.buffers, init) {
                ctx.write_logs("No `\"` to end string literal");
                return skip_token_print(ctx, globals.buffers, globals.pool);
            }

            let str = &globals.buffers.buffer(BufTy::Base)
                [globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2)];
            let res = globals
                .pool
                .lookup_str_insert(ctx, globals.hash, str, HashExtra::Text)?;

            globals
                .buffers
                .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

            let char = globals.buffers.at_offset(BufTy::Base, 2);

            if globals.buffers.offset(BufTy::Base, 2) < globals.buffers.init(BufTy::Base)
                && LexClass::of(char) != LexClass::Whitespace
                && char != b'}'
                && char != b'%'
            {
                return skip_illegal_stuff_after_token_print(ctx, globals.buffers, globals.pool);
            }

            single_function.push(res.loc);
        }
        b'\'' => {
            globals
                .buffers
                .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

            let init = globals.buffers.init(BufTy::Base);
            Scan::new()
                .chars(b"}%")
                .class(LexClass::Whitespace)
                .scan_till(globals.buffers, init);

            let range =
                globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
            globals.buffers.buffer_mut(BufTy::Base)[range.clone()].make_ascii_lowercase();

            let str = &globals.buffers.buffer(BufTy::Base)[range];
            let res = globals.pool.lookup_str(globals.hash, str, StrIlk::BstFn);

            if !res.exists {
                return skip_token_unknown_function_print(ctx, globals.buffers, globals.pool);
            } else if res.loc == wiz_loc {
                return print_recursion_illegal(ctx, globals.buffers, globals.pool);
            }

            single_function.push(QUOTE_NEXT_FN);
            single_function.push(res.loc);
        }
        b'{' => {
            globals
                .buffers
                .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

            let str = format!("'{}", ctx.impl_fn_num);

            let res = globals.pool.lookup_str_insert(
                ctx,
                globals.hash,
                str.as_bytes(),
                HashExtra::BstFn(BstFn::Wizard(0)),
            )?;

            if res.exists {
                ctx.write_logs("Already encountered implicit function");
                print_confusion(ctx);
                return Err(BibtexError::Fatal);
            }
            ctx.impl_fn_num += 1;

            single_function.push(QUOTE_NEXT_FN);
            single_function.push(res.loc);

            scan_fn_def(ctx, globals, res.loc, wiz_loc)?;
        }
        _ => {
            let init = globals.buffers.init(BufTy::Base);
            Scan::new()
                .chars(b"}%")
                .class(LexClass::Whitespace)
                .scan_till(globals.buffers, init);

            let range =
                globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);

            globals.buffers.buffer_mut(BufTy::Base)[range.clone()].make_ascii_lowercase();

            let str = &globals.buffers.buffer(BufTy::Base)[range];
            let res = globals.pool.lookup_str(globals.hash, str, StrIlk::BstFn);
            if !res.exists {
                return skip_token_unknown_function_print(ctx, globals.buffers, globals.pool);
            } else if res.loc == wiz_loc {
                return print_recursion_illegal(ctx, globals.buffers, globals.pool);
            }

            single_function.push(res.loc);
        }
    }
    Ok(())
}

pub(crate) fn scan_fn_def(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
    fn_hash_loc: HashPointer,
    wiz_loc: HashPointer,
) -> Result<(), BibtexError> {
    let mut single_function = Vec::new();

    if !eat_bst_white_space(ctx, globals.buffers) {
        eat_bst_print(ctx);
        ctx.write_logs("function");
        bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool)?;
        return Ok(());
    }

    let mut char = globals.buffers.at_offset(BufTy::Base, 2);
    while char != b'}' {
        handle_char(ctx, globals, &mut single_function, wiz_loc, char)?;

        if !eat_bst_white_space(ctx, globals.buffers) {
            eat_bst_print(ctx);
            ctx.write_logs("function");
            return bst_err_print_and_look_for_blank_line(ctx, globals.buffers, globals.pool);
        }

        char = globals.buffers.at_offset(BufTy::Base, 2);
    }

    single_function.push(HashData::end_of_def());

    globals.hash.node_mut(fn_hash_loc).extra =
        HashExtra::BstFn(BstFn::Wizard(globals.other.wiz_func_len()));

    for ptr in single_function {
        globals.other.push_wiz_func(ptr);
    }

    globals
        .buffers
        .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);

    Ok(())
}

fn scan_balanced_braces(
    ctx: &mut Bibtex<'_, '_>,
    buffers: &mut GlobalBuffer,
    pool: &StringPool,
    bibs: &mut BibData,
    store_field: bool,
    bib_command: Option<BibCommand>,
    right_str_delim: ASCIICode,
) -> Result<bool, BibtexError> {
    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

    if (LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace
        || buffers.offset(BufTy::Base, 2) == buffers.init(BufTy::Base))
        && !compress_bib_white(ctx, buffers, pool, bibs, bib_command)?
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
                        ctx.write_log_file("Field filled up at '{', reallocating.\n");
                        buffers.grow_all();
                    }

                    buffers.set_at(BufTy::Ex, buffers.offset(BufTy::Ex, 1), b'{');
                    buffers.set_offset(BufTy::Ex, 1, buffers.offset(BufTy::Ex, 1) + 1);

                    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

                    if (LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace
                        || buffers.offset(BufTy::Base, 2) == buffers.init(BufTy::Base))
                        && !compress_bib_white(ctx, buffers, pool, bibs, bib_command)?
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
                                    ctx.write_log_file(&format!(
                                        "Field filled up at '{}', reallocating.\n",
                                        c as char
                                    ));
                                }
                                _ => {
                                    ctx.write_log_file(&format!(
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
                            && !compress_bib_white(ctx, buffers, pool, bibs, bib_command)?
                        {
                            return Ok(false);
                        }

                        if brace_level == 0 {
                            break;
                        }
                    }
                }
                b'}' => {
                    return bib_unbalanced_braces_print(ctx, buffers, pool, bibs, bib_command)
                        .map(|_| false);
                }
                c => {
                    if buffers.offset(BufTy::Ex, 1) >= buffers.len() {
                        ctx.write_log_file(&format!(
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
                        && !compress_bib_white(ctx, buffers, pool, bibs, bib_command)?
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
                    if !eat_bib_white_space(ctx, buffers, bibs) {
                        return eat_bib_print(ctx, buffers, pool, bibs, bib_command).map(|_| false);
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
                            && !eat_bib_white_space(ctx, buffers, bibs)
                        {
                            return eat_bib_print(ctx, buffers, pool, bibs, bib_command)
                                .map(|_| false);
                        }
                    }
                }
                b'}' => {
                    return bib_unbalanced_braces_print(ctx, buffers, pool, bibs, bib_command)
                        .map(|_| false);
                }
                _ => {
                    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
                    let init = buffers.init(BufTy::Base);
                    if !Scan::new()
                        .chars(&[right_str_delim, b'{', b'}'])
                        .scan_till(buffers, init)
                        && !eat_bib_white_space(ctx, buffers, bibs)
                    {
                        return eat_bib_print(ctx, buffers, pool, bibs, bib_command).map(|_| false);
                    }
                }
            }
        }
    }

    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    Ok(true)
}

fn scan_a_field_token_and_eat_white(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
    store_field: bool,
    command: Option<BibCommand>,
    cur_macro_loc: HashPointer,
    right_outer_delim: ASCIICode,
) -> Result<bool, BibtexError> {
    match globals.buffers.at_offset(BufTy::Base, 2) {
        b'{' => {
            if !scan_balanced_braces(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                store_field,
                command,
                b'}',
            )? {
                return Ok(false);
            }
        }
        b'"' => {
            if !scan_balanced_braces(
                ctx,
                globals.buffers,
                globals.pool,
                globals.bibs,
                store_field,
                command,
                b'"',
            )? {
                return Ok(false);
            }
        }
        b'0'..=b'9' => {
            let last = globals.buffers.init(BufTy::Base);
            if !Scan::new()
                .not_class(LexClass::Numeric)
                .scan_till_nonempty(globals.buffers, last)
            {
                ctx.write_logs("A digit disappeared");
                print_confusion(ctx);
                return Err(BibtexError::Fatal);
            }

            if store_field {
                let range =
                    globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
                let len = range.end - range.start;

                while len >= globals.buffers.len() {
                    // TODO: This may change output of a field long enough to fill the buffer twice. OTOH, that is a 40KB field roughly
                    ctx.write_log_file(&format!(
                        "Field filled up at {}, reallocating.\n",
                        globals.buffers.at_offset(BufTy::Base, 1)
                    ));
                    globals.buffers.grow_all();
                }

                globals.buffers.copy_within(
                    BufTy::Base,
                    BufTy::Ex,
                    range.start,
                    globals.buffers.offset(BufTy::Ex, 1),
                    len,
                );

                globals.buffers.set_offset(
                    BufTy::Ex,
                    1,
                    globals.buffers.offset(BufTy::Ex, 1) + len,
                );
            }
        }
        _ => {
            let res = scan_identifier(globals.buffers, b',', right_outer_delim, b'#');
            if !matches!(
                res,
                ScanRes::WhitespaceAdjacent | ScanRes::SpecifiedCharAdjacent
            ) {
                bib_id_print(ctx, globals.buffers, res)?;
                ctx.write_logs("a field part");
                bib_err_print(ctx, globals.buffers, globals.pool, globals.bibs, command)?;
                return Ok(false);
            }

            if store_field {
                let range =
                    globals.buffers.offset(BufTy::Base, 1)..globals.buffers.offset(BufTy::Base, 2);
                globals.buffers.buffer_mut(BufTy::Base)[range.clone()].make_ascii_lowercase();
                let str = &globals.buffers.buffer(BufTy::Base)[range];

                let res = globals.pool.lookup_str(globals.hash, str, StrIlk::Macro);
                let mut store_token = true;
                if command == Some(BibCommand::String) && res.loc == cur_macro_loc {
                    store_token = false;
                    macro_warn_print(ctx, globals.buffers);
                    ctx.write_logs("used in its own definition\n");
                    bib_warn_print(ctx, globals.pool, globals.bibs)?;
                }

                if !res.exists {
                    store_token = false;
                    macro_warn_print(ctx, globals.buffers);
                    ctx.write_logs("undefined\n");
                    bib_warn_print(ctx, globals.pool, globals.bibs)?;
                }

                if store_token {
                    let HashExtra::Macro(strnum) = globals.hash.node(res.loc).extra else {
                        panic!("Macro lookup didn't have Macro extra");
                    };
                    let mut str = globals.pool.get_str(strnum);

                    if globals.buffers.offset(BufTy::Ex, 1) == 0
                        && LexClass::of(str[0]) == LexClass::Whitespace
                    {
                        if globals.buffers.offset(BufTy::Ex, 1) >= globals.buffers.len() {
                            ctx.write_log_file("Field filled up at ' ', reallocating.\n");
                            globals.buffers.grow_all();
                        }

                        globals.buffers.set_at(
                            BufTy::Ex,
                            globals.buffers.offset(BufTy::Ex, 1),
                            b' ',
                        );
                        globals.buffers.set_offset(
                            BufTy::Ex,
                            1,
                            globals.buffers.offset(BufTy::Ex, 1) + 1,
                        );

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

                        if globals.buffers.offset(BufTy::Ex, 1) >= globals.buffers.len() {
                            ctx.write_log_file(&msg(c));
                            globals.buffers.grow_all();
                        }

                        if LexClass::of(c) != LexClass::Whitespace {
                            globals.buffers.set_at(
                                BufTy::Ex,
                                globals.buffers.offset(BufTy::Ex, 1),
                                c,
                            );
                            globals.buffers.set_offset(
                                BufTy::Ex,
                                1,
                                globals.buffers.offset(BufTy::Ex, 1) + 1,
                            );
                        } else if c != b' ' {
                            globals.buffers.set_at(
                                BufTy::Ex,
                                globals.buffers.offset(BufTy::Ex, 1),
                                b' ',
                            );
                            globals.buffers.set_offset(
                                BufTy::Ex,
                                1,
                                globals.buffers.offset(BufTy::Ex, 1) + 1,
                            );
                        }
                    }
                }
            }
        }
    }

    if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
        return eat_bib_print(ctx, globals.buffers, globals.pool, globals.bibs, command)
            .map(|_| false);
    }
    Ok(true)
}

// TODO: Refactor this to bundle up arguments into structs as relevant
#[allow(clippy::too_many_arguments)]
pub(crate) fn scan_and_store_the_field_value_and_eat_white(
    ctx: &mut Bibtex<'_, '_>,
    globals: &mut GlobalItems<'_>,
    store_field: bool,
    command: Option<BibCommand>,
    cite_out: Option<&mut CiteNumber>,
    cur_macro_loc: HashPointer,
    right_outer_delim: ASCIICode,
    field_name_loc: HashPointer,
) -> Result<bool, BibtexError> {
    // Consume tokens/strings separated by #
    globals.buffers.set_offset(BufTy::Ex, 1, 0);
    if !scan_a_field_token_and_eat_white(
        ctx,
        globals,
        store_field,
        command,
        cur_macro_loc,
        right_outer_delim,
    )? {
        return Ok(false);
    }
    while globals.buffers.at_offset(BufTy::Base, 2) == b'#' {
        globals
            .buffers
            .set_offset(BufTy::Base, 2, globals.buffers.offset(BufTy::Base, 2) + 1);
        if !eat_bib_white_space(ctx, globals.buffers, globals.bibs) {
            return eat_bib_print(ctx, globals.buffers, globals.pool, globals.bibs, command)
                .map(|_| false);
        }
        if !scan_a_field_token_and_eat_white(
            ctx,
            globals,
            store_field,
            command,
            cur_macro_loc,
            right_outer_delim,
        )? {
            return Ok(false);
        }
    }

    if store_field {
        if command.is_none()
            && globals.buffers.offset(BufTy::Ex, 1) > 0
            && globals
                .buffers
                .at(BufTy::Ex, globals.buffers.offset(BufTy::Ex, 1) - 1)
                == b' '
        {
            globals
                .buffers
                .set_offset(BufTy::Ex, 1, globals.buffers.offset(BufTy::Ex, 1) - 1);
        }

        let ex_buf_xptr = if command.is_none()
            && globals.buffers.at(BufTy::Ex, 0) == b' '
            && globals.buffers.offset(BufTy::Ex, 1) > 0
        {
            1
        } else {
            0
        };

        let str =
            &globals.buffers.buffer(BufTy::Ex)[ex_buf_xptr..globals.buffers.offset(BufTy::Ex, 1)];
        let res = globals
            .pool
            .lookup_str_insert(ctx, globals.hash, str, HashExtra::Text)?;

        if let Some(command) = command {
            match command {
                // TODO: Should this be `unreachable!`? This way will cover errors, but also shouldn't misbehave
                BibCommand::Comment => (),
                BibCommand::Preamble => globals.bibs.add_preamble(globals.hash.text(res.loc)),
                BibCommand::String => {
                    globals.hash.node_mut(cur_macro_loc).extra =
                        HashExtra::Macro(globals.hash.text(res.loc))
                }
            }
        } else {
            let HashExtra::BstFn(BstFn::Field(field)) = globals.hash.node(field_name_loc).extra
            else {
                panic!("field_name_loc wasn't a BstFn::Field");
            };

            let field_ptr = globals.cites.entry_ptr() * globals.other.num_fields() + field;
            if field_ptr >= globals.other.max_fields() {
                ctx.write_logs("field_info index is out of range");
                print_confusion(ctx);
                return Err(BibtexError::Fatal);
            }

            /* missing */
            if globals.other.field(field_ptr) != 0 {
                ctx.write_logs("Warning--I'm ignoring ");
                print_a_pool_str(
                    ctx,
                    globals.cites.get_cite(globals.cites.entry_ptr()),
                    globals.pool,
                )?;
                ctx.write_logs("'s extra \"");
                print_a_pool_str(ctx, globals.hash.text(field_name_loc), globals.pool)?;
                ctx.write_logs("\" field\n");
                bib_warn_print(ctx, globals.pool, globals.bibs)?;
            } else {
                globals
                    .other
                    .set_field(field_ptr, globals.hash.text(res.loc));
                if field == globals.other.crossref_num() && !ctx.all_entries {
                    let end = globals.buffers.offset(BufTy::Ex, 1);
                    // Move Ex to Out, at the same position
                    globals.buffers.copy_within(
                        BufTy::Ex,
                        BufTy::Out,
                        ex_buf_xptr,
                        ex_buf_xptr,
                        end - ex_buf_xptr,
                    );
                    globals.buffers.buffer_mut(BufTy::Out)[ex_buf_xptr..end].make_ascii_lowercase();
                    let str = &globals.buffers.buffer(BufTy::Out)[ex_buf_xptr..end];
                    let lc_res = globals.pool.lookup_str_insert(
                        ctx,
                        globals.hash,
                        str,
                        HashExtra::LcCite(0),
                    )?;
                    if let Some(cite_out) = cite_out {
                        *cite_out = lc_res.loc;
                    }
                    let HashExtra::LcCite(cite_loc) = globals.hash.node(lc_res.loc).extra else {
                        panic!("LcCite lookup didn't have LcCite extra");
                    };
                    if lc_res.exists {
                        let HashExtra::Cite(cite) = globals.hash.node(cite_loc).extra else {
                            panic!("LcCite location didn't have Cite extra");
                        };

                        if cite >= globals.cites.old_num_cites() {
                            globals.cites.set_info(cite, cite + 1);
                        }
                    } else {
                        let str = &globals.buffers.buffer(BufTy::Ex)
                            [ex_buf_xptr..globals.buffers.offset(BufTy::Ex, 1)];
                        let c_res = globals.pool.lookup_str_insert(
                            ctx,
                            globals.hash,
                            str,
                            HashExtra::Cite(0),
                        )?;
                        if c_res.exists {
                            hash_cite_confusion(ctx);
                            return Err(BibtexError::Fatal);
                        }
                        let new_ptr = add_database_cite(
                            globals.cites,
                            globals.other,
                            globals.hash,
                            globals.cites.ptr(),
                            c_res.loc,
                            lc_res.loc,
                        );

                        let HashExtra::Cite(cite) = globals.hash.node(c_res.loc).extra else {
                            panic!("Cite lookup didn't have Cite extra");
                        };
                        globals.cites.set_ptr(new_ptr);
                        globals.cites.set_info(cite, 1);
                    }
                }
            }
        }
    }

    Ok(true)
}

pub(crate) fn decr_brace_level(
    ctx: &mut ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
    pop_lit_var: StrNumber,
    brace_level: &mut i32,
) -> Result<(), BibtexError> {
    if *brace_level == 0 {
        braces_unbalanced_complaint(ctx, pool, cites, pop_lit_var)?;
    } else {
        *brace_level -= 1;
    }

    Ok(())
}

pub(crate) fn check_brace_level(
    ctx: &mut ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    cites: &CiteInfo,
    pop_lit_var: StrNumber,
    brace_level: i32,
) -> Result<(), BibtexError> {
    if brace_level > 0 {
        braces_unbalanced_complaint(ctx, pool, cites, pop_lit_var)?;
    }
    Ok(())
}

pub(crate) fn name_scan_for_and(
    ctx: &mut ExecCtx<'_, '_, '_>,
    pool: &StringPool,
    buffers: &mut GlobalBuffer,
    cites: &CiteInfo,
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
                decr_brace_level(ctx, pool, cites, pop_lit_var, brace_level)?;
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

    check_brace_level(ctx, pool, cites, pop_lit_var, *brace_level)
}

pub(crate) fn von_token_found(
    buffers: &GlobalBuffer,
    hash: &HashData,
    pool: &StringPool,
    name_bf_ptr: &mut BufPointer,
    name_bf_xptr: BufPointer,
) -> bool {
    while *name_bf_ptr < name_bf_xptr {
        let char = buffers.at(BufTy::Sv, *name_bf_ptr);
        match char {
            b'A'..=b'Z' => return false,
            b'a'..=b'z' => return true,
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
                    let ilk = res
                        .exists
                        .then(|| {
                            if let HashExtra::ControlSeq(seq) = hash.node(res.loc).extra {
                                Some(seq)
                            } else {
                                None
                            }
                        })
                        .flatten();
                    if let Some(seq) = ilk {
                        return match seq {
                            ControlSeq::UpperOE
                            | ControlSeq::UpperAE
                            | ControlSeq::UpperAA
                            | ControlSeq::UpperO
                            | ControlSeq::UpperL => false,
                            ControlSeq::LowerI
                            | ControlSeq::LowerJ
                            | ControlSeq::LowerOE
                            | ControlSeq::LowerAE
                            | ControlSeq::LowerAA
                            | ControlSeq::LowerO
                            | ControlSeq::LowerL
                            | ControlSeq::LowerSS => true,
                        };
                    }

                    while *name_bf_ptr < name_bf_xptr && nm_brace_level > 0 {
                        let char = buffers.at(BufTy::Sv, *name_bf_ptr);
                        match char {
                            b'A'..=b'Z' => return false,
                            b'a'..=b'z' => return true,
                            b'}' => nm_brace_level -= 1,
                            b'{' => nm_brace_level += 1,
                            _ => (),
                        }
                        *name_bf_ptr += 1;
                    }
                    return false;
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
    false
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn von_name_ends_and_last_name_starts_stuff(
    buffers: &GlobalBuffer,
    hash: &HashData,
    pool: &StringPool,
    last_end: BufPointer,
    von_start: BufPointer,
    von_end: &mut BufPointer,
    name_bf_ptr: &mut BufPointer,
    name_bf_xptr: &mut BufPointer,
) {
    *von_end = last_end - 1;
    while *von_end > von_start {
        *name_bf_ptr = buffers.name_tok(*von_end - 1);
        *name_bf_xptr = buffers.name_tok(*von_end);
        if von_token_found(buffers, hash, pool, name_bf_ptr, *name_bf_xptr) {
            return;
        }
        *von_end -= 1;
    }
}

pub(crate) fn enough_text_chars(
    buffers: &GlobalBuffer,
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
