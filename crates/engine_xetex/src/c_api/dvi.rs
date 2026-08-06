use crate::c_api::engine::{
    rs_prepare_mag, IntPar, DEFINE_NATIVE_FONT, EOP, FNT_DEF1, FONT_BASE, POP, POST, POST_POST,
    TEX_INFINITY,
};
use crate::c_api::errors::{ffi_abort, rs_fatal_error, EngineError};
use crate::c_api::font::{make_font_def, AAT_FONT_FLAG, OTGR_FONT_FLAG};
use crate::c_api::globals::{Globals, ALL_CTX};
use crate::c_api::output::{rs_print, rs_print_bytes, rs_print_int, rs_print_nl_bytes};
use crate::c_api::pool::rs_str_length;
use crate::ty::StrNumber;
use std::io::Write;
use std::ptr;
use tectonic_bridge_core::OutputId;

pub const DVI_BUF_SIZE: i32 = 16384;
pub const HALF_BUF: i32 = DVI_BUF_SIZE / 2;
pub const FNT_NUM_0: usize = 171; /* DVI code */

pub const XDV_ID_BYTE: u8 = 7;
pub const SPX_ID_BYTE: u8 = 100;

pub struct DviCtx {
    file: Option<OutputId>,
    limit: i32,
    ptr: i32,
    offset: i32,
    gone: i32,
    cur_s: i32,
    buf: Vec<u8>,
    output_file_name: StrNumber,
}

impl DviCtx {
    pub(crate) const fn new() -> DviCtx {
        DviCtx {
            file: None,
            limit: 0,
            ptr: 0,
            offset: 0,
            gone: 0,
            cur_s: 0,
            buf: Vec::new(),
            output_file_name: 0,
        }
    }

    pub fn with<T>(f: impl FnOnce(&mut DviCtx) -> T) -> T {
        Globals::token(|tok| ALL_CTX.with(|(_, _, _, _, _, dvi, _, _, _)| f(dvi.borrow_mut(tok))))
    }
}

#[no_mangle]
pub extern "C" fn dvi_file() -> OutputId {
    DviCtx::with(|dvi| dvi.file.unwrap())
}

#[no_mangle]
pub extern "C" fn set_dvi_file(file: OutputId) {
    DviCtx::with(|dvi| dvi.file = Some(file))
}

#[no_mangle]
pub extern "C" fn dvi_limit() -> i32 {
    DviCtx::with(|dvi| dvi.limit)
}

#[no_mangle]
pub extern "C" fn set_dvi_limit(val: i32) {
    DviCtx::with(|dvi| dvi.limit = val)
}

#[no_mangle]
pub extern "C" fn dvi_ptr() -> i32 {
    DviCtx::with(|dvi| dvi.ptr)
}

#[no_mangle]
pub extern "C" fn set_dvi_ptr(val: i32) {
    DviCtx::with(|dvi| dvi.ptr = val)
}

#[no_mangle]
pub extern "C" fn dvi_offset() -> i32 {
    DviCtx::with(|dvi| dvi.offset)
}

#[no_mangle]
pub extern "C" fn set_dvi_offset(val: i32) {
    DviCtx::with(|dvi| dvi.offset = val)
}

#[no_mangle]
pub extern "C" fn dvi_gone() -> i32 {
    DviCtx::with(|dvi| dvi.gone)
}

#[no_mangle]
pub extern "C" fn set_dvi_gone(val: i32) {
    DviCtx::with(|dvi| dvi.gone = val)
}

#[no_mangle]
pub extern "C" fn cur_s() -> i32 {
    DviCtx::with(|dvi| dvi.cur_s)
}

#[no_mangle]
pub extern "C" fn set_cur_s(val: i32) {
    DviCtx::with(|dvi| dvi.cur_s = val)
}

#[no_mangle]
pub extern "C" fn output_file_name() -> i32 {
    DviCtx::with(|dvi| dvi.output_file_name)
}

#[no_mangle]
pub extern "C" fn set_output_file_name(val: i32) {
    DviCtx::with(|dvi| dvi.output_file_name = val)
}

#[no_mangle]
pub extern "C" fn dvi_buf(idx: usize) -> u8 {
    DviCtx::with(|engine| engine.buf[idx])
}

#[no_mangle]
pub extern "C" fn set_dvi_buf(idx: usize, val: u8) {
    DviCtx::with(|engine| engine.buf[idx] = val)
}

#[no_mangle]
pub extern "C" fn dvi_buf_ptr(idx: usize) -> *mut u8 {
    DviCtx::with(|engine| ptr::from_mut(&mut engine.buf[idx]))
}

#[no_mangle]
pub extern "C" fn resize_dvi_buf(len: usize) {
    DviCtx::with(|engine| engine.buf.resize(len, 0))
}

#[no_mangle]
pub extern "C" fn clear_dvi_buf() {
    DviCtx::with(|engine| engine.buf.clear())
}

pub fn rs_write_to_dvi(globals: &mut Globals<'_, '_>, a: usize, b: usize) {
    let out = globals.state.get_output(globals.dvi.file.unwrap());
    out.write_all(&globals.dvi.buf[a..=b])
        .expect("failed to write data to XDV file");
}

#[no_mangle]
pub extern "C" fn write_to_dvi(a: i32, b: i32) {
    Globals::with(|globals| rs_write_to_dvi(globals, a as usize, b as usize))
}

pub fn rs_deinitialize_shipout_variables(globals: &mut Globals<'_, '_>) {
    globals.dvi.buf.clear();
}

#[no_mangle]
pub extern "C" fn deinitialize_shipout_variables() {
    Globals::with(rs_deinitialize_shipout_variables)
}

pub fn dvi_swap(globals: &mut Globals<'_, '_>) -> Result<(), EngineError> {
    if globals.dvi.ptr > TEX_INFINITY - globals.dvi.offset {
        globals.dvi.cur_s = -2;
        rs_fatal_error(globals, c"dvi length exceeds 0x7FFFFFFF")?;
    }

    if globals.dvi.limit == DVI_BUF_SIZE {
        rs_write_to_dvi(globals, 0, (HALF_BUF - 1) as usize);
        globals.dvi.limit = HALF_BUF;
        globals.dvi.offset += DVI_BUF_SIZE;
        globals.dvi.ptr = 0;
    } else {
        rs_write_to_dvi(globals, HALF_BUF as usize, (DVI_BUF_SIZE - 1) as usize);
        globals.dvi.limit = DVI_BUF_SIZE;
    }
    globals.dvi.gone += HALF_BUF;
    Ok(())
}

pub fn rs_dvi_out(globals: &mut Globals<'_, '_>, c: u8) -> Result<(), EngineError> {
    globals.dvi.buf[globals.dvi.ptr as usize] = c;
    globals.dvi.ptr += 1;
    if globals.dvi.ptr == globals.dvi.limit {
        dvi_swap(globals)?;
    }
    Ok(())
}

#[no_mangle]
pub extern "C-unwind" fn dvi_out(c: u8) {
    let res = Globals::with(|globals| rs_dvi_out(globals, c));
    ffi_abort(res)
}

pub fn rs_dvi_four(globals: &mut Globals<'_, '_>, mut x: i32) -> Result<(), EngineError> {
    // TODO: Honestly, this could just use `x.to_*_bytes()`
    if x >= 0 {
        rs_dvi_out(globals, (x / 0x1000000) as u8)?;
    } else {
        x += 0x40000000;
        x += 0x40000000;
        rs_dvi_out(globals, ((x / 0x1000000) + 128) as u8)?;
    }

    x %= 0x1000000;
    rs_dvi_out(globals, (x / 0x10000) as u8)?;

    x %= 0x10000;
    rs_dvi_out(globals, (x / 0x100) as u8)?;
    rs_dvi_out(globals, (x % 0x100) as u8)?;
    Ok(())
}

#[no_mangle]
pub extern "C-unwind" fn dvi_four(x: i32) {
    let res = Globals::with(|globals| rs_dvi_four(globals, x));
    ffi_abort(res)
}

pub fn rs_dvi_two(globals: &mut Globals<'_, '_>, s: u16) -> Result<(), EngineError> {
    rs_dvi_out(globals, (s / 0x100) as u8)?;
    rs_dvi_out(globals, (s % 0x100) as u8)?;
    Ok(())
}

#[no_mangle]
pub extern "C-unwind" fn dvi_two(s: u16) {
    let res = Globals::with(|globals| rs_dvi_two(globals, s));
    ffi_abort(res)
}

pub fn rs_dvi_pop(globals: &mut Globals<'_, '_>, l: i32) -> Result<(), EngineError> {
    if l == globals.dvi.offset + globals.dvi.ptr && globals.dvi.ptr > 0 {
        globals.dvi.ptr -= 1;
    } else {
        rs_dvi_out(globals, POP)?;
    }
    Ok(())
}

#[no_mangle]
pub extern "C-unwind" fn dvi_pop(l: i32) {
    let res = Globals::with(|globals| rs_dvi_pop(globals, l));
    ffi_abort(res)
}

pub fn rs_dvi_native_font_def(globals: &mut Globals<'_, '_>, f: usize) -> Result<(), EngineError> {
    rs_dvi_out(globals, DEFINE_NATIVE_FONT)?;
    rs_dvi_four(globals, (f - 1) as i32)?;
    for byte in make_font_def(globals, f)? {
        rs_dvi_out(globals, byte)?;
    }
    Ok(())
}

#[no_mangle]
pub extern "C-unwind" fn dvi_native_font_def(f: usize) {
    let res = Globals::with(|globals| rs_dvi_native_font_def(globals, f));
    ffi_abort(res)
}

pub fn rs_dvi_font_def(globals: &mut Globals<'_, '_>, f: usize) -> Result<(), EngineError> {
    if globals.fonts.font_area[f] == AAT_FONT_FLAG || globals.fonts.font_area[f] == OTGR_FONT_FLAG {
        rs_dvi_native_font_def(globals, f)?;
    } else {
        if f <= 256 {
            rs_dvi_out(globals, FNT_DEF1)?;
            rs_dvi_out(globals, (f - 1) as u8)?;
        } else {
            rs_dvi_out(globals, FNT_DEF1 + 1)?;
            rs_dvi_out(globals, ((f - 1) / 256) as u8)?;
            rs_dvi_out(globals, ((f - 1) % 256) as u8)?;
        }

        rs_dvi_out(globals, globals.fonts.font_check[f].s3 as u8)?;
        rs_dvi_out(globals, globals.fonts.font_check[f].s2 as u8)?;
        rs_dvi_out(globals, globals.fonts.font_check[f].s1 as u8)?;
        rs_dvi_out(globals, globals.fonts.font_check[f].s0 as u8)?;
        rs_dvi_four(globals, globals.fonts.font_size[f])?;
        rs_dvi_four(globals, globals.fonts.font_dsize[f])?;

        rs_dvi_out(
            globals,
            rs_str_length(globals.strings, globals.fonts.font_area[f] as StrNumber) as u8,
        )?;

        let k = globals.strings.tex_str(globals.fonts.font_name[f]);
        let l = k.iter().position(|c| *c == ':' as u16).unwrap_or(k.len());

        rs_dvi_out(globals, l as u8)?;

        let a = globals.strings.tex_str(globals.fonts.font_area[f]).to_vec();
        for b in a {
            rs_dvi_out(globals, b as u8)?;
        }
        let n = globals.strings.tex_str(globals.fonts.font_name[f]).to_vec();
        for &b in &n[..l] {
            rs_dvi_out(globals, b as u8)?;
        }
    }
    Ok(())
}

#[no_mangle]
pub extern "C-unwind" fn dvi_font_def(f: usize) {
    let res = Globals::with(|globals| rs_dvi_font_def(globals, f));
    ffi_abort(res)
}

pub fn rs_finalize_dvi_file(globals: &mut Globals<'_, '_>) -> Result<(), EngineError> {
    while globals.dvi.cur_s > -1 {
        if globals.dvi.cur_s > 0 {
            rs_dvi_out(globals, POP)?;
        } else {
            rs_dvi_out(globals, EOP)?;
            globals.engine.total_pages += 1;
        }

        globals.dvi.cur_s -= 1;
    }

    if globals.engine.total_pages == 0 {
        rs_print_nl_bytes(globals, b"No pages of output.");
        return Ok(());
    }

    if globals.dvi.cur_s == -2 {
        /* This happens when the DVI gets too big; a message has already been printed */
        return Ok(());
    }

    rs_dvi_out(globals, POST)?;
    rs_dvi_four(globals, globals.engine.last_bop)?;
    globals.engine.last_bop = globals.dvi.offset + globals.dvi.ptr - 5;
    rs_dvi_four(globals, 25400000)?; /* magic values: conversion ratio for sp */
    rs_dvi_four(globals, 473628672)?; /* magic values: conversion ratio for sp */
    rs_prepare_mag(globals)?;
    rs_dvi_four(globals, globals.engine.int_par(IntPar::Mag))?;
    rs_dvi_four(globals, globals.engine.max_v)?;
    rs_dvi_four(globals, globals.engine.max_h)?;
    rs_dvi_out(globals, (globals.engine.max_push / 256) as u8)?;
    rs_dvi_out(globals, (globals.engine.max_push % 256) as u8)?;
    rs_dvi_out(globals, (globals.engine.total_pages / 256 % 256) as u8)?;
    rs_dvi_out(globals, (globals.engine.total_pages % 256) as u8)?;

    while globals.fonts.font_ptr > FONT_BASE {
        if globals.fonts.font_used[globals.fonts.font_ptr as usize] {
            rs_dvi_font_def(globals, globals.fonts.font_ptr as usize)?;
        }
        globals.fonts.font_ptr -= 1;
    }

    rs_dvi_out(globals, POST_POST)?;
    rs_dvi_four(globals, globals.engine.last_bop)?;

    if globals.engine.semantic_pagination_enabled {
        rs_dvi_out(globals, SPX_ID_BYTE)?;
    } else {
        rs_dvi_out(globals, XDV_ID_BYTE)?;
    }

    let mut k = 4 + (DVI_BUF_SIZE - globals.dvi.ptr) % 4;
    while k > 0 {
        rs_dvi_out(globals, 223)?;
        k -= 1;
    }

    if globals.dvi.limit == HALF_BUF {
        rs_write_to_dvi(globals, HALF_BUF as usize, (DVI_BUF_SIZE - 1) as usize);
    }

    if globals.dvi.ptr > TEX_INFINITY - globals.dvi.offset {
        globals.dvi.cur_s = -2;
        rs_fatal_error(globals, c"dvi length exceeds 0x7FFFFFFF")?;
    }

    if globals.dvi.ptr > 0 {
        rs_write_to_dvi(globals, 0, (globals.dvi.ptr - 1) as usize);
    }

    let k = globals
        .dvi
        .file
        .map(|file| globals.state.output_close(file))
        .unwrap_or(false);

    if k {
        rs_print_nl_bytes(globals, b"Error ");
        rs_print_int(globals, 1);
        rs_print_bytes(globals, b" (Operation not permitted) generating output;");
        rs_print_nl_bytes(globals, b"file ");
        rs_print(globals, globals.dvi.output_file_name);
        rs_print_bytes(globals, b" may not be valid.");
        /* XeTeX adds history = OUTPUT_FAILURE = 4 here; I'm not implementing that. */
    } else {
        rs_print_nl_bytes(globals, b"Output written on ");
        rs_print(globals, globals.dvi.output_file_name);
        rs_print_bytes(globals, b" (");
        rs_print_int(globals, globals.engine.total_pages);
        let follow = if globals.engine.total_pages != 1 {
            b" pages" as &[_]
        } else {
            b" page"
        };
        rs_print_bytes(globals, follow);
        rs_print_bytes(globals, b", ");
        rs_print_int(globals, globals.dvi.offset + globals.dvi.ptr);
        rs_print_bytes(globals, b" bytes).");
    }
    Ok(())
}

#[no_mangle]
pub extern "C-unwind" fn finalize_dvi_file() {
    let res = Globals::with(rs_finalize_dvi_file);
    ffi_abort(res)
}
