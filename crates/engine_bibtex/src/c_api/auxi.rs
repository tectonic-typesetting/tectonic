use crate::{
    c_api::{
        bibs::{with_bibs_mut, BibData},
        buffer::{with_buffers_mut, BufTy, GlobalBuffer},
        char_info::LexClass,
        cite::{with_cites_mut, CiteInfo},
        exec::print_bst_name,
        hash::{with_hash_mut, HashData},
        log::{
            aux_err_illegal_another_print, aux_err_no_right_brace_print, aux_err_print,
            aux_err_stuff_after_right_brace_print, aux_err_white_space_in_argument_print,
            hash_cite_confusion, log_pr_bst_name, print_a_pool_str, print_confusion,
            print_overflow, rs_log_pr_aux_name, rs_print_a_token, rs_print_aux_name,
            rs_print_bib_name, write_log_file, write_logs, AuxTy,
        },
        peekable::{peekable_open, PeekableInput},
        pool::{with_pool_mut, StringPool},
        scan::Scan,
        AuxNumber, Bibtex, CResult, StrIlk, StrNumber,
    },
    BibtexError,
};
use std::{cell::RefCell, ffi::CString, ptr, ptr::NonNull};
use tectonic_bridge_core::FileFormat;

const AUX_STACK_SIZE: usize = 20;

pub struct AuxData {
    aux_list: [StrNumber; AUX_STACK_SIZE + 1],
    aux_file: [*mut PeekableInput; AUX_STACK_SIZE + 1],
    aux_ln_stack: [i32; AUX_STACK_SIZE + 1],
    aux_ptr: AuxNumber,
}

impl AuxData {
    fn new() -> AuxData {
        AuxData {
            aux_list: [0; AUX_STACK_SIZE + 1],
            aux_file: [ptr::null_mut(); AUX_STACK_SIZE + 1],
            aux_ln_stack: [0; AUX_STACK_SIZE + 1],
            aux_ptr: 0,
        }
    }

    pub fn ptr(&self) -> AuxNumber {
        self.aux_ptr
    }

    pub fn set_ptr(&mut self, ptr: AuxNumber) {
        self.aux_ptr = ptr;
    }

    pub fn at_ptr(&self) -> StrNumber {
        self.aux_list[self.aux_ptr]
    }

    pub fn set_at_ptr(&mut self, num: StrNumber) {
        self.aux_list[self.aux_ptr] = num;
    }

    fn file_at_ptr(&self) -> *mut PeekableInput {
        self.aux_file[self.aux_ptr]
    }

    pub fn set_file_at_ptr(&mut self, file: *mut PeekableInput) {
        self.aux_file[self.aux_ptr] = file;
    }

    pub fn ln_at_ptr(&self) -> i32 {
        self.aux_ln_stack[self.aux_ptr]
    }

    fn set_ln_at_ptr(&mut self, ln: i32) {
        self.aux_ln_stack[self.aux_ptr] = ln;
    }
}

thread_local! {
    pub static AUX: RefCell<AuxData> = RefCell::new(AuxData::new());
}

pub fn reset() {
    AUX.with(|aux| *aux.borrow_mut() = AuxData::new());
}

pub fn with_aux<T>(f: impl FnOnce(&AuxData) -> T) -> T {
    AUX.with(|aux| f(&aux.borrow()))
}

pub fn with_aux_mut<T>(f: impl FnOnce(&mut AuxData) -> T) -> T {
    AUX.with(|aux| f(&mut aux.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn cur_aux_file() -> *mut PeekableInput {
    with_aux(|aux| aux.file_at_ptr())
}

#[no_mangle]
pub extern "C" fn set_cur_aux_file(file: *mut PeekableInput) {
    with_aux_mut(|aux| aux.set_file_at_ptr(file))
}

#[no_mangle]
pub extern "C" fn cur_aux_ln() -> i32 {
    with_aux(|aux| aux.ln_at_ptr())
}

#[no_mangle]
pub extern "C" fn set_cur_aux_ln(ln: i32) {
    with_aux_mut(|aux| aux.set_ln_at_ptr(ln))
}

#[no_mangle]
pub extern "C" fn aux_ptr() -> AuxNumber {
    with_aux(|aux| aux.aux_ptr)
}

#[no_mangle]
pub extern "C" fn set_aux_ptr(num: AuxNumber) {
    with_aux_mut(|aux| aux.aux_ptr = num)
}

fn rs_aux_bib_data_command(
    ctx: &mut Bibtex,
    buffers: &mut GlobalBuffer,
    bibs: &mut BibData,
    aux: &AuxData,
    pool: &mut StringPool,
    hash: &mut HashData,
) -> Result<(), BibtexError> {
    if ctx.bib_seen {
        aux_err_illegal_another_print(AuxTy::Data)?;
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    ctx.bib_seen = true;

    while buffers.at_offset(BufTy::Base, 2) != b'}' {
        buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
        let init = buffers.init(BufTy::Base);
        if !Scan::new()
            .chars(&[b'}', b','])
            .class(LexClass::Whitespace)
            .scan_till(buffers, init)
        {
            aux_err_no_right_brace_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        if LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace {
            aux_err_white_space_in_argument_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        if buffers.init(BufTy::Base) > buffers.offset(BufTy::Base, 2) + 1
            && buffers.at_offset(BufTy::Base, 2) == b'}'
        {
            aux_err_stuff_after_right_brace_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        if bibs.ptr() == bibs.len() {
            bibs.grow();
        }

        let file = &buffers.buffer(BufTy::Base)
            [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
        let res = pool.lookup_str_insert(hash, file, StrIlk::BibFile)?;
        bibs.set_cur_bib(hash.text(res.loc));
        if res.exists {
            write_logs("This database file appears more than once: ");
            rs_print_bib_name(pool, bibs)?;
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        let name = pool.get_str(bibs.cur_bib());
        let fname = CString::new(name).unwrap();
        let bib_in = peekable_open(&fname, FileFormat::Bib);
        if bib_in.is_null() {
            write_logs("I couldn't open the database file ");
            rs_print_bib_name(pool, bibs)?;
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }
        bibs.set_cur_bib_file(NonNull::new(bib_in));
        bibs.set_ptr(bibs.ptr() + 1);
    }

    Ok(())
}

fn rs_aux_bib_style_command(
    ctx: &mut Bibtex,
    buffers: &mut GlobalBuffer,
    aux: &AuxData,
    pool: &mut StringPool,
    hash: &mut HashData,
) -> Result<(), BibtexError> {
    if ctx.bst_seen {
        aux_err_illegal_another_print(AuxTy::Style)?;
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    ctx.bst_seen = true;

    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);
    let init = buffers.init(BufTy::Base);
    if !Scan::new()
        .chars(&[b'}'])
        .class(LexClass::Whitespace)
        .scan_till(buffers, init)
    {
        aux_err_no_right_brace_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    if LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace {
        aux_err_white_space_in_argument_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    if buffers.init(BufTy::Base) > buffers.offset(BufTy::Base, 2) + 1 {
        aux_err_stuff_after_right_brace_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    let file = &buffers.buffer(BufTy::Base)
        [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
    let res = pool.lookup_str_insert(hash, file, StrIlk::BstFile)?;
    ctx.bst_str = hash.text(res.loc);
    if res.exists {
        write_logs("Already encountered style file");
        print_confusion();
        return Err(BibtexError::Fatal);
    }

    let name = pool.get_str(ctx.bst_str);
    let fname = CString::new(name).unwrap();
    let ptr = peekable_open(&fname, FileFormat::Bst);
    if ptr.is_null() {
        write_logs("I couldn't open style file ");
        print_bst_name(ctx, pool)?;
        ctx.bst_str = 0;
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    ctx.bst_file = NonNull::new(ptr);

    if ctx.config.verbose {
        write_logs("The style file: ");
        print_bst_name(ctx, pool)?;
    } else {
        write_log_file("The style file: ");
        log_pr_bst_name(ctx, pool)?;
    }

    Ok(())
}

fn rs_aux_citation_command(
    ctx: &mut Bibtex,
    buffers: &mut GlobalBuffer,
    aux: &AuxData,
    pool: &mut StringPool,
    hash: &mut HashData,
    cites: &mut CiteInfo,
) -> Result<(), BibtexError> {
    ctx.citation_seen = true;

    while buffers.at_offset(BufTy::Base, 2) != b'}' {
        buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

        let init = buffers.init(BufTy::Base);
        if !Scan::new()
            .chars(&[b'}', b','])
            .class(LexClass::Whitespace)
            .scan_till(buffers, init)
        {
            aux_err_no_right_brace_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }
        if LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace {
            aux_err_white_space_in_argument_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }
        if buffers.init(BufTy::Base) > buffers.offset(BufTy::Base, 2) + 1
            && buffers.at_offset(BufTy::Base, 2) == b'}'
        {
            aux_err_stuff_after_right_brace_print();
            aux_err_print(buffers, aux, pool)?;
            return Ok(());
        }

        if buffers.offset(BufTy::Base, 2) - buffers.offset(BufTy::Base, 1) == 1 {
            if buffers.at_offset(BufTy::Base, 1) == b'*' {
                if ctx.all_entries {
                    write_logs("Multiple inclusions of entire database\n");
                    aux_err_print(buffers, aux, pool)?;
                    return Ok(());
                } else {
                    ctx.all_entries = true;
                    cites.set_all_marker(cites.ptr());
                    continue;
                }
            }
        }

        let idx = buffers.offset(BufTy::Base, 1);
        buffers.copy_within(
            BufTy::Base,
            BufTy::Ex,
            idx,
            idx,
            buffers.offset(BufTy::Base, 2) - idx,
        );
        let range = buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2);
        let lc_cite = &mut buffers.buffer_mut(BufTy::Ex)[range];
        lc_cite.make_ascii_lowercase();

        let lc_res = pool.lookup_str_insert(hash, lc_cite, StrIlk::LcCite)?;
        if lc_res.exists {
            let cite = &buffers.buffer(BufTy::Base)
                [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
            let uc_res = pool.lookup_str(hash, cite, StrIlk::Cite);
            if !uc_res.exists {
                write_logs("Case mismatch error between cite keys ");
                rs_print_a_token(buffers);
                write_logs(" and ");
                print_a_pool_str(
                    cites.get_cite(hash.ilk_info(hash.ilk_info(lc_res.loc) as usize) as usize),
                    pool,
                )?;
                write_logs("\n");
                aux_err_print(buffers, aux, pool)?;
                return Ok(());
            }
        } else {
            let cite = &buffers.buffer(BufTy::Base)
                [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
            let uc_res = pool.lookup_str_insert(hash, cite, StrIlk::Cite)?;
            if uc_res.exists {
                hash_cite_confusion();
                return Err(BibtexError::Fatal);
            }

            if cites.ptr() == cites.len() {
                cites.grow();
            }

            cites.set_cite(cites.ptr(), hash.text(uc_res.loc));
            hash.set_ilk_info(uc_res.loc, cites.ptr() as i32);
            hash.set_ilk_info(lc_res.loc, uc_res.loc as i32);
            cites.set_ptr(cites.ptr() + 1);
        }
    }

    Ok(())
}

fn rs_aux_input_command(
    ctx: &mut Bibtex,
    buffers: &mut GlobalBuffer,
    aux: &mut AuxData,
    pool: &mut StringPool,
    hash: &mut HashData,
) -> Result<(), BibtexError> {
    buffers.set_offset(BufTy::Base, 2, buffers.offset(BufTy::Base, 2) + 1);

    let init = buffers.init(BufTy::Base);
    if !Scan::new()
        .chars(&[b'}'])
        .class(LexClass::Whitespace)
        .scan_till(buffers, init)
    {
        aux_err_no_right_brace_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    if LexClass::of(buffers.at_offset(BufTy::Base, 2)) == LexClass::Whitespace {
        aux_err_white_space_in_argument_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    if buffers.init(BufTy::Base) > buffers.offset(BufTy::Base, 2) + 1 {
        aux_err_stuff_after_right_brace_print();
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    aux.set_ptr(aux.ptr() + 1);
    if aux.ptr() == AUX_STACK_SIZE {
        rs_print_a_token(buffers);
        write_logs(": ");
        print_overflow();
        write_logs(&format!("auxiliary file depth {}\n", AUX_STACK_SIZE));
        return Err(BibtexError::Fatal);
    }

    let aux_ext = pool.get_str(ctx.s_aux_extension);
    let aux_extension_ok = if (buffers.offset(BufTy::Base, 2) - buffers.offset(BufTy::Base, 1)
        < aux_ext.len())
        || aux_ext
            != &buffers.buffer(BufTy::Base)
                [buffers.offset(BufTy::Base, 2) - aux_ext.len()..buffers.offset(BufTy::Base, 2)]
    {
        false
    } else {
        true
    };

    if !aux_extension_ok {
        rs_print_a_token(buffers);
        write_logs(" has a wrong extension");
        aux.set_ptr(aux.ptr() - 1);
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    let file = &buffers.buffer(BufTy::Base)
        [buffers.offset(BufTy::Base, 1)..buffers.offset(BufTy::Base, 2)];
    let res = pool.lookup_str_insert(hash, file, StrIlk::AuxFile)?;
    aux.set_at_ptr(hash.text(res.loc));
    if res.exists {
        write_logs("Already encountered file ");
        rs_print_aux_name(aux, pool)?;
        aux.set_ptr(aux.ptr() - 1);
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }

    let name = pool.get_str(aux.at_ptr());
    let fname = CString::new(name).unwrap();
    let ptr = peekable_open(&fname, FileFormat::Tex);
    if ptr.is_null() {
        write_logs("I couldn't open auxiliary file ");
        rs_print_aux_name(aux, pool)?;
        aux.set_ptr(aux.ptr() - 1);
        aux_err_print(buffers, aux, pool)?;
        return Ok(());
    }
    aux.set_file_at_ptr(ptr);

    write_logs(&format!("A level-{} auxiliary file: ", aux.ptr()));
    rs_log_pr_aux_name(aux, pool)?;
    aux.set_ln_at_ptr(0);

    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn aux_bib_data_command(ctx: *mut Bibtex) -> CResult {
    with_buffers_mut(|buffers| {
        with_bibs_mut(|bibs| {
            with_aux(|aux| {
                with_pool_mut(|pool| {
                    with_hash_mut(|hash| {
                        rs_aux_bib_data_command(&mut *ctx, buffers, bibs, aux, pool, hash)
                    })
                })
            })
        })
    })
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn aux_bib_style_command(ctx: *mut Bibtex) -> CResult {
    with_buffers_mut(|buffers| {
        with_aux(|aux| {
            with_pool_mut(|pool| {
                with_hash_mut(|hash| rs_aux_bib_style_command(&mut *ctx, buffers, aux, pool, hash))
            })
        })
    })
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn aux_citation_command(ctx: *mut Bibtex) -> CResult {
    with_buffers_mut(|buffers| {
        with_aux(|aux| {
            with_pool_mut(|pool| {
                with_cites_mut(|cites| {
                    with_hash_mut(|hash| {
                        rs_aux_citation_command(&mut *ctx, buffers, aux, pool, hash, cites)
                    })
                })
            })
        })
    })
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn aux_input_command(ctx: *mut Bibtex) -> CResult {
    with_buffers_mut(|buffers| {
        with_aux_mut(|aux| {
            with_pool_mut(|pool| {
                with_hash_mut(|hash| rs_aux_input_command(&mut *ctx, buffers, aux, pool, hash))
            })
        })
    })
    .into()
}
