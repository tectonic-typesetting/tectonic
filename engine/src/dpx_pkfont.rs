#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type pdf_obj;
    pub type pdf_font;
    #[no_mangle]
    fn pdf_font_set_fontname(font: *mut pdf_font, fontname: *const i8) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_get_param(font: *mut pdf_font, type_0: libc::c_int) -> libc::c_double;
    #[no_mangle]
    fn pdf_font_get_encoding(font: *mut pdf_font) -> libc::c_int;
    #[no_mangle]
    fn pdf_font_get_usedchars(font: *mut pdf_font) -> *mut i8;
    #[no_mangle]
    fn pdf_font_get_resource(font: *mut pdf_font) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_font_get_ident(font: *mut pdf_font) -> *mut i8;
    #[no_mangle]
    fn pdf_font_is_in_use(font: *mut pdf_font) -> bool;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: libc::c_int,
    );
    #[no_mangle]
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_name(name: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn fread(
        _: *mut libc::c_void,
        _: u64,
        _: u64,
        _: *mut FILE,
    ) -> u64;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: libc::c_uint,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

        Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

        This program is free software; you can redistribute it and/or modify
        it under the terms of the GNU General Public License as published by
        the Free Software Foundation; either version 2 of the License, or
        (at your option) any later version.

        This program is distributed in the hope that it will be useful,
        but WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
        GNU General Public License for more details.

        You should have received a copy of the GNU General Public License
        along with this program; if not, write to the Free Software
        Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
    */
    /* When reading numbers from binary files 1, 2, or 3 bytes are
       interpreted as either signed or unsigned.

       Four bytes from DVI, PK, TFM, or VF files always yield a signed
       32-bit integer (int32_t), but some of them must not be negative.

       Four byte numbers from JPEG2000, OpenType, or TrueType files are
       mostly unsigned (u32) and occasionally signed (int32_t).
    */
    #[no_mangle]
    fn get_unsigned_byte(_: *mut FILE) -> u8;
    #[no_mangle]
    fn skip_bytes(_: libc::c_uint, _: *mut FILE);
    #[no_mangle]
    fn get_signed_byte(_: *mut FILE) -> libc::c_schar;
    #[no_mangle]
    fn get_unsigned_pair(_: *mut FILE) -> u16;
    #[no_mangle]
    fn get_signed_pair(_: *mut FILE) -> libc::c_short;
    #[no_mangle]
    fn get_unsigned_triple(_: *mut FILE) -> libc::c_uint;
    #[no_mangle]
    fn get_signed_quad(_: *mut FILE) -> int32_t;
    #[no_mangle]
    fn get_unsigned_num(_: *mut FILE, _: u8) -> u32;
    #[no_mangle]
    fn get_positive_quad(_: *mut FILE, _: *const i8, _: *const i8) -> u32;
    #[no_mangle]
    static mut work_buffer: [i8; 0];
    #[no_mangle]
    fn pdf_sprint_number(buf: *mut i8, value: libc::c_double) -> libc::c_int;
    #[no_mangle]
    fn pdf_encoding_used_by_type3(enc_id: libc::c_int);
    /* WARNING:
     * Pointer(s) may change after another encoding is loaded.
     */
    #[no_mangle]
    fn pdf_encoding_get_name(enc_id: libc::c_int) -> *mut i8;
    #[no_mangle]
    fn pdf_encoding_get_encoding(enc_id: libc::c_int) -> *mut *mut i8;
    #[no_mangle]
    fn tfm_open(tex_name: *const i8, must_exist: libc::c_int) -> libc::c_int;
    /* From TFM header */
    #[no_mangle]
    fn tfm_get_design_size(font_id: libc::c_int) -> libc::c_double;
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pk_header_ {
    pub pkt_len: u32,
    pub chrcode: int32_t,
    pub wd: int32_t,
    pub dx: int32_t,
    pub dy: int32_t,
    pub bm_wd: u32,
    pub bm_ht: u32,
    pub bm_hoff: int32_t,
    pub bm_voff: int32_t,
    pub dyn_f: libc::c_int,
    pub run_color: libc::c_int,
}
static mut base_dpi: libc::c_uint = 600u32;
#[no_mangle]
pub unsafe extern "C" fn PKFont_set_dpi(mut dpi: libc::c_int) {
    if dpi <= 0i32 {
        _tt_abort(
            b"Invalid DPI: %d\n\x00" as *const u8 as *const i8,
            dpi,
        );
    }
    base_dpi = dpi as libc::c_uint;
}
/* (Only) This requires TFM to get design size... */
unsafe extern "C" fn truedpi(
    mut ident: *const i8,
    mut point_size: libc::c_double,
    mut bdpi: libc::c_uint,
) -> libc::c_uint {
    let mut dpi: libc::c_uint = bdpi;
    let mut design_size: libc::c_double = 0.;
    let mut tfm_id: libc::c_int = 0;
    tfm_id = tfm_open(ident, 0i32);
    if tfm_id < 0i32 {
        return dpi;
    }
    design_size = tfm_get_design_size(tfm_id);
    if design_size <= 0.0f64 {
        dpx_warning(
            b"DESGIN_SIZE <= 0.0? (TFM=\"%s\")\x00" as *const u8 as *const i8,
            ident,
        );
    } else {
        dpi = (floor(base_dpi as libc::c_double * point_size / design_size / 1.0f64 + 0.5f64)
            * 1.0f64) as libc::c_uint
    }
    return dpi;
}
unsafe extern "C" fn dpx_open_pk_font_at(
    mut ident: *const i8,
    mut dpi: libc::c_uint,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fqpn: *mut i8 = 0 as *mut i8;
    /*kpse_glyph_file_type kpse_file_info;*/
    fqpn = 0 as *mut i8; /*kpse_find_glyph(ident, dpi, kpse_pk_format, &kpse_file_info);*/
    if fqpn.is_null() {
        return 0 as *mut FILE;
    }
    fp = fopen(fqpn, b"rb\x00" as *const u8 as *const i8);
    free(fqpn as *mut libc::c_void);
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_font_open_pkfont(mut font: *mut pdf_font) -> libc::c_int {
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut point_size: libc::c_double = 0.;
    let mut encoding_id: libc::c_int = 0;
    let mut dpi: libc::c_uint = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    ident = pdf_font_get_ident(font);
    point_size = pdf_font_get_param(font, 2i32);
    encoding_id = pdf_font_get_encoding(font);
    if ident.is_null() || point_size <= 0.0f64 {
        return -1i32;
    }
    dpi = truedpi(ident, point_size, base_dpi);
    fp = dpx_open_pk_font_at(ident, dpi);
    if fp.is_null() {
        return -1i32;
    }
    fclose(fp);
    /* Type 3 fonts doesn't have FontName.
     * FontFamily is recommended for PDF 1.5.
     */
    pdf_font_set_fontname(font, ident);
    if encoding_id >= 0i32 {
        pdf_encoding_used_by_type3(encoding_id);
        dpx_warning(
            b"PK font is found for font \"%s\" but non built-in encoding \"%s\" is specified.\x00"
                as *const u8 as *const i8,
            ident,
            pdf_encoding_get_name(encoding_id),
        );
        dpx_warning(
            b">> Assuming this is for glyph name assignment.\x00" as *const u8
                as *const i8,
        );
    }
    return 0i32;
}
/* We are using Mask Image. Fill black is bit clear.
 * Optimizing those codes doesn't improve things.
 */
unsafe extern "C" fn fill_black_run(
    mut dp: *mut u8,
    mut left: u32,
    mut run_count: u32,
) -> u32 {
    static mut mask: [u8; 8] = [
        127u32 as u8,
        191u32 as u8,
        223u32 as u8,
        239u32 as u8,
        247u32 as u8,
        251u32 as u8,
        253u32 as u8,
        254u32 as u8,
    ];
    let mut right: u32 = left
        .wrapping_add(run_count)
        .wrapping_sub(1i32 as libc::c_uint);
    while left <= right {
        let ref mut fresh0 = *dp.offset(left.wrapping_div(8i32 as libc::c_uint) as isize);
        *fresh0 = (*fresh0 as libc::c_int
            & mask[left.wrapping_rem(8i32 as libc::c_uint) as usize] as libc::c_int)
            as u8;
        left = left.wrapping_add(1)
    }
    return run_count;
}
/* Just skip bits. See decode_packed() */
unsafe extern "C" fn fill_white_run(mut run_count: u32) -> u32 {
    return run_count;
}
unsafe extern "C" fn pk_packed_num(
    mut np: *mut u32,
    mut dyn_f: libc::c_int,
    mut dp: *mut u8,
    mut pl: u32,
) -> u32 {
    let mut nmbr: u32 = 0i32 as u32;
    let mut i: u32 = *np;
    let mut nyb: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if i.wrapping_div(2i32 as libc::c_uint) == pl {
        dpx_warning(
            b"EOD reached while unpacking pk_packed_num.\x00" as *const u8 as *const i8,
        );
        return 0i32 as u32;
    }
    nyb = if i.wrapping_rem(2i32 as libc::c_uint) != 0 {
        *dp.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int & 0xfi32
    } else {
        *dp.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int >> 4i32 & 0xfi32
    };
    i = i.wrapping_add(1);
    if nyb == 0i32 {
        j = 0i32;
        loop {
            if i.wrapping_div(2i32 as libc::c_uint) == pl {
                dpx_warning(
                    b"EOD reached while unpacking pk_packed_num.\x00" as *const u8
                        as *const i8,
                );
                break;
            } else {
                nyb = if i.wrapping_rem(2i32 as libc::c_uint) != 0 {
                    *dp.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int
                        & 0xfi32
                } else {
                    *dp.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int >> 4i32
                        & 0xfi32
                };
                i = i.wrapping_add(1);
                j += 1;
                if !(nyb == 0i32) {
                    break;
                }
            }
        }
        nmbr = nyb as u32;
        loop {
            let fresh1 = j;
            j = j - 1;
            if !(fresh1 > 0i32) {
                break;
            }
            if i.wrapping_div(2i32 as libc::c_uint) == pl {
                dpx_warning(
                    b"EOD reached while unpacking pk_packed_num.\x00" as *const u8
                        as *const i8,
                );
                break;
            } else {
                nyb = if i.wrapping_rem(2i32 as libc::c_uint) != 0 {
                    *dp.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int
                        & 0xfi32
                } else {
                    *dp.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int >> 4i32
                        & 0xfi32
                };
                i = i.wrapping_add(1);
                nmbr = nmbr
                    .wrapping_mul(16i32 as libc::c_uint)
                    .wrapping_add(nyb as libc::c_uint)
            }
        }
        nmbr = (nmbr as libc::c_uint)
            .wrapping_add(((13i32 - dyn_f) * 16i32 + dyn_f - 15i32) as libc::c_uint)
            as u32
    } else if nyb <= dyn_f {
        nmbr = nyb as u32
    } else if nyb < 14i32 {
        if i.wrapping_div(2i32 as libc::c_uint) == pl {
            dpx_warning(
                b"EOD reached while unpacking pk_packed_num.\x00" as *const u8
                    as *const i8,
            );
            return 0i32 as u32;
        }
        nmbr = ((nyb - dyn_f - 1i32) * 16i32
            + (if i.wrapping_rem(2i32 as libc::c_uint) != 0 {
                *dp.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int & 0xfi32
            } else {
                *dp.offset(i.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int >> 4i32
                    & 0xfi32
            })
            + dyn_f
            + 1i32) as u32;
        i = i.wrapping_add(1)
    }
    *np = i;
    return nmbr;
}
unsafe extern "C" fn send_out(
    mut rowptr: *mut u8,
    mut rowbytes: u32,
    mut stream: *mut pdf_obj,
) {
    pdf_add_stream(stream, rowptr as *mut libc::c_void, rowbytes as libc::c_int);
}
unsafe extern "C" fn pk_decode_packed(
    mut stream: *mut pdf_obj,
    mut wd: u32,
    mut ht: u32,
    mut dyn_f: libc::c_int,
    mut run_color: libc::c_int,
    mut dp: *mut u8,
    mut pl: u32,
) -> libc::c_int {
    let mut rowptr: *mut u8 = 0 as *mut u8;
    let mut rowbytes: u32 = 0;
    let mut i: u32 = 0;
    let mut np: u32 = 0i32 as u32;
    let mut run_count: u32 = 0i32 as u32;
    let mut repeat_count: u32 = 0i32 as u32;
    rowbytes = wd
        .wrapping_add(7i32 as libc::c_uint)
        .wrapping_div(8i32 as libc::c_uint);
    rowptr = new((rowbytes as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64)
        as u32) as *mut u8;
    /* repeat count is applied to the *current* row.
     * "run" can span across rows.
     * If there are non-zero repeat count and if run
     * spans across row, first repeat and then continue.
     */
    np = 0i32 as u32; /* 1 is white */
    i = 0i32 as u32;
    while i < ht {
        let mut rowbits_left: u32 = 0;
        let mut nbits: u32 = 0;
        repeat_count = 0i32 as u32;
        memset(
            rowptr as *mut libc::c_void,
            0xffi32,
            rowbytes as u64,
        );
        rowbits_left = wd;
        /* Fill run left over from previous row */
        if run_count > 0i32 as libc::c_uint {
            nbits = if rowbits_left < run_count {
                rowbits_left
            } else {
                run_count
            };
            match run_color {
                0 => {
                    rowbits_left = (rowbits_left as libc::c_uint).wrapping_sub(fill_black_run(
                        rowptr,
                        0i32 as u32,
                        nbits,
                    )) as u32
                }
                1 => {
                    rowbits_left = (rowbits_left as libc::c_uint)
                        .wrapping_sub(fill_white_run(nbits))
                        as u32
                }
                _ => {}
            }
            run_count = (run_count as libc::c_uint).wrapping_sub(nbits) as u32
        }
        /* Read nybbles until we have a full row */
        while np.wrapping_div(2i32 as libc::c_uint) < pl && rowbits_left > 0i32 as libc::c_uint {
            let mut nyb: libc::c_int = 0;
            nyb = if np.wrapping_rem(2i32 as libc::c_uint) != 0 {
                *dp.offset(np.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int & 0xfi32
            } else {
                *dp.offset(np.wrapping_div(2i32 as libc::c_uint) as isize) as libc::c_int >> 4i32
                    & 0xfi32
            };
            if nyb == 14i32 {
                /* packed number "repeat_count" follows */
                if repeat_count != 0i32 as libc::c_uint {
                    dpx_warning(
                        b"Second repeat count for this row!\x00" as *const u8
                            as *const i8,
                    ); /* Consume this nybble */
                } /* run_count */
                np = np.wrapping_add(1); /* Consume this nybble */
                repeat_count = pk_packed_num(&mut np, dyn_f, dp, pl)
            } else if nyb == 15i32 {
                if repeat_count != 0i32 as libc::c_uint {
                    dpx_warning(
                        b"Second repeat count for this row!\x00" as *const u8
                            as *const i8,
                    );
                }
                np = np.wrapping_add(1);
                repeat_count = 1i32 as u32
            } else {
                /* Interprete current nybble as packed number */
                run_count = pk_packed_num(&mut np, dyn_f, dp, pl);
                nbits = if rowbits_left < run_count {
                    rowbits_left
                } else {
                    run_count
                };
                run_color = (run_color == 0) as libc::c_int;
                run_count = (run_count as libc::c_uint).wrapping_sub(nbits) as u32;
                match run_color {
                    0 => {
                        rowbits_left = (rowbits_left as libc::c_uint).wrapping_sub(fill_black_run(
                            rowptr,
                            wd.wrapping_sub(rowbits_left),
                            nbits,
                        )) as u32
                    }
                    1 => {
                        rowbits_left = (rowbits_left as libc::c_uint)
                            .wrapping_sub(fill_white_run(nbits))
                            as u32
                    }
                    _ => {}
                }
            }
        }
        /* We got bitmap row data. */
        send_out(rowptr, rowbytes, stream);
        while i < ht && repeat_count > 0i32 as libc::c_uint {
            send_out(rowptr, rowbytes, stream);
            repeat_count = repeat_count.wrapping_sub(1);
            i = i.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    free(rowptr as *mut libc::c_void);
    return 0i32;
}
unsafe extern "C" fn pk_decode_bitmap(
    mut stream: *mut pdf_obj,
    mut wd: u32,
    mut ht: u32,
    mut dyn_f: libc::c_int,
    mut run_color: libc::c_int,
    mut dp: *mut u8,
    mut pl: u32,
) -> libc::c_int {
    let mut rowptr: *mut u8 = 0 as *mut u8;
    let mut c: u8 = 0;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut rowbytes: u32 = 0;
    static mut mask: [u8; 8] = [
        0x80u32 as u8,
        0x40u32 as u8,
        0x20u32 as u8,
        0x10u32 as u8,
        0x8u32 as u8,
        0x4u32 as u8,
        0x2u32 as u8,
        0x1u32 as u8,
    ];
    if dyn_f == 14i32 {
    } else {
        __assert_fail(b"dyn_f == 14\x00" as *const u8 as *const i8,
                      b"dpx-pkfont.c\x00" as *const u8 as *const i8,
                      308i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 89],
                                                &[i8; 89]>(b"int pk_decode_bitmap(pdf_obj *, uint32_t, uint32_t, int, int, unsigned char *, uint32_t)\x00")).as_ptr());
    }
    if run_color != 0i32 {
        dpx_warning(b"run_color != 0 for bitmap pk data?\x00" as *const u8 as *const i8);
    } else if pl
        < wd.wrapping_mul(ht)
            .wrapping_add(7i32 as libc::c_uint)
            .wrapping_div(8i32 as libc::c_uint)
    {
        dpx_warning(
            b"Insufficient bitmap pk data. %dbytes expected but only %dbytes read.\x00" as *const u8
                as *const i8,
            wd.wrapping_mul(ht)
                .wrapping_add(7i32 as libc::c_uint)
                .wrapping_div(8i32 as libc::c_uint),
            pl,
        );
        return -1i32;
    }
    rowbytes = wd
        .wrapping_add(7i32 as libc::c_uint)
        .wrapping_div(8i32 as libc::c_uint);
    rowptr = new((rowbytes as u64)
        .wrapping_mul(::std::mem::size_of::<u8>() as u64)
        as u32) as *mut u8;
    memset(rowptr as *mut libc::c_void, 0i32, rowbytes as u64);
    /* Flip. PK bitmap is not byte aligned for each rows. */
    i = 0i32 as u32; /* flip bit */
    j = 0i32 as u32;
    while i < ht.wrapping_mul(wd) {
        c = (*dp.offset(i.wrapping_div(8i32 as libc::c_uint) as isize) as libc::c_int
            & mask[i.wrapping_rem(8i32 as libc::c_uint) as usize] as libc::c_int)
            as u8;
        if c as libc::c_int == 0i32 {
            let ref mut fresh2 = *rowptr.offset(j.wrapping_div(8i32 as libc::c_uint) as isize);
            *fresh2 = (*fresh2 as libc::c_int
                | mask[i.wrapping_rem(8i32 as libc::c_uint) as usize] as libc::c_int)
                as u8
        }
        j = j.wrapping_add(1);
        if j == wd {
            send_out(rowptr, rowbytes, stream);
            memset(rowptr as *mut libc::c_void, 0i32, rowbytes as u64);
            j = 0i32 as u32
        }
        i = i.wrapping_add(1)
    }
    return 0i32;
}
unsafe extern "C" fn do_preamble(mut fp: *mut FILE) {
    /* Check for id byte */
    if fgetc(fp) == 89i32 {
        /* Skip comment */
        skip_bytes(get_unsigned_byte(fp) as libc::c_uint, fp);
        /* Skip other header info.  It's normally used for verifying this
        is the file wethink it is */
        skip_bytes(16i32 as libc::c_uint, fp);
    } else {
        _tt_abort(
            b"embed_pk_font: PK ID byte is incorrect.  Are you sure this is a PK file?\x00"
                as *const u8 as *const i8,
        );
    };
}
unsafe extern "C" fn read_pk_char_header(
    mut h: *mut pk_header_,
    mut opcode: u8,
    mut fp: *mut FILE,
) -> libc::c_int {
    if !h.is_null() {
    } else {
        __assert_fail(
            b"h\x00" as *const u8 as *const i8,
            b"dpx-pkfont.c\x00" as *const u8 as *const i8,
            366i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 68], &[i8; 68]>(
                b"int read_pk_char_header(struct pk_header_ *, unsigned char, FILE *)\x00",
            ))
            .as_ptr(),
        );
    }
    if opcode as libc::c_int & 4i32 == 0i32 {
        /* short */
        (*h).pkt_len = ((opcode as libc::c_int & 3i32) << 8i32
            | get_unsigned_byte(fp) as libc::c_int) as u32; /* TFM width */
        (*h).chrcode = get_unsigned_byte(fp) as int32_t; /* horizontal escapement */
        (*h).wd = get_unsigned_triple(fp) as int32_t; /* extended short */
        (*h).dx = (get_unsigned_byte(fp) as libc::c_int) << 16i32;
        (*h).dy = 0i32;
        (*h).bm_wd = get_unsigned_byte(fp) as u32;
        (*h).bm_ht = get_unsigned_byte(fp) as u32;
        (*h).bm_hoff = get_signed_byte(fp) as int32_t;
        (*h).bm_voff = get_signed_byte(fp) as int32_t;
        (*h).pkt_len = ((*h).pkt_len as libc::c_uint).wrapping_sub(8i32 as libc::c_uint) as u32
            as u32
    } else if opcode as libc::c_int & 7i32 == 7i32 {
        /* long */
        (*h).pkt_len = get_positive_quad(
            fp,
            b"PK\x00" as *const u8 as *const i8,
            b"pkt_len\x00" as *const u8 as *const i8,
        ); /* 16.16 fixed point number in pixels */
        (*h).chrcode = get_signed_quad(fp);
        (*h).wd = get_signed_quad(fp);
        (*h).dx = get_signed_quad(fp);
        (*h).dy = get_signed_quad(fp);
        (*h).bm_wd = get_positive_quad(
            fp,
            b"PK\x00" as *const u8 as *const i8,
            b"bm_wd\x00" as *const u8 as *const i8,
        );
        (*h).bm_ht = get_positive_quad(
            fp,
            b"PK\x00" as *const u8 as *const i8,
            b"bm_ht\x00" as *const u8 as *const i8,
        );
        (*h).bm_hoff = get_signed_quad(fp);
        (*h).bm_voff = get_signed_quad(fp);
        (*h).pkt_len = ((*h).pkt_len as libc::c_uint).wrapping_sub(28i32 as libc::c_uint)
            as u32
    } else {
        (*h).pkt_len = ((opcode as libc::c_int & 3i32) << 16i32
            | get_unsigned_pair(fp) as libc::c_int) as u32;
        (*h).chrcode = get_unsigned_byte(fp) as int32_t;
        (*h).wd = get_unsigned_triple(fp) as int32_t;
        (*h).dx = (get_unsigned_pair(fp) as libc::c_int) << 16i32;
        (*h).dy = 0i32;
        (*h).bm_wd = get_unsigned_pair(fp) as u32;
        (*h).bm_ht = get_unsigned_pair(fp) as u32;
        (*h).bm_hoff = get_signed_pair(fp) as int32_t;
        (*h).bm_voff = get_signed_pair(fp) as int32_t;
        (*h).pkt_len = ((*h).pkt_len as libc::c_uint).wrapping_sub(13i32 as libc::c_uint)
            as u32
    }
    (*h).dyn_f = opcode as libc::c_int / 16i32;
    (*h).run_color = if opcode as libc::c_int & 8i32 != 0 {
        1i32
    } else {
        0i32
    };
    if (*h).chrcode as u32 > 0xffi32 as libc::c_uint {
        dpx_warning(
            b"Unable to handle long characters in PK files: code=0x%04x\x00" as *const u8
                as *const i8,
            (*h).chrcode,
        );
        return -1i32;
    }
    return 0i32;
}
/* CCITT Group 4 filter may reduce file size. */
unsafe extern "C" fn create_pk_CharProc_stream(
    mut pkh: *mut pk_header_,
    mut chrwid: libc::c_double,
    mut pkt_ptr: *mut u8,
    mut pkt_len: u32,
) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj; /* charproc */
    let mut llx: int32_t = 0;
    let mut lly: int32_t = 0;
    let mut urx: int32_t = 0;
    let mut ury: int32_t = 0;
    let mut len: libc::c_int = 0;
    llx = -(*pkh).bm_hoff;
    lly = ((*pkh).bm_voff as libc::c_uint).wrapping_sub((*pkh).bm_ht) as int32_t;
    urx = (*pkh).bm_wd.wrapping_sub((*pkh).bm_hoff as libc::c_uint) as int32_t;
    ury = (*pkh).bm_voff;
    stream = pdf_new_stream(1i32 << 0i32);
    /*
     * The following line is a "metric" for the PDF reader:
     *
     * PDF Reference Reference, 4th ed., p.385.
     *
     * The wx (first operand of d1) must be consistent with the corresponding
     * width in the font's Widths array. The format string of sprint() must be
     * consistent with write_number() in pdfobj.c.
     */
    len = pdf_sprint_number(work_buffer.as_mut_ptr(), chrwid);
    len += sprintf(
        work_buffer.as_mut_ptr().offset(len as isize),
        b" 0 %d %d %d %d d1\n\x00" as *const u8 as *const i8,
        llx,
        lly,
        urx,
        ury,
    );
    pdf_add_stream(stream, work_buffer.as_mut_ptr() as *const libc::c_void, len);
    /*
     * Acrobat dislike transformation [0 0 0 0 dx dy].
     * PDF Reference, 4th ed., p.147, says,
     *
     *   Use of a noninvertible matrix when painting graphics objects can result in
     *   unpredictable behavior.
     *
     * but it does not forbid use of such transformation.
     */
    if (*pkh).bm_wd != 0i32 as libc::c_uint
        && (*pkh).bm_ht != 0i32 as libc::c_uint
        && pkt_len > 0i32 as libc::c_uint
    {
        /* Otherwise we embed an empty stream :-( */
        /* Scale and translate origin to lower left corner for raster data */
        len = sprintf(
            work_buffer.as_mut_ptr(),
            b"q\n%u 0 0 %u %d %d cm\n\x00" as *const u8 as *const i8,
            (*pkh).bm_wd,
            (*pkh).bm_ht,
            llx,
            lly,
        );
        pdf_add_stream(stream, work_buffer.as_mut_ptr() as *const libc::c_void, len);
        len = sprintf(
            work_buffer.as_mut_ptr(),
            b"BI\n/W %u\n/H %u\n/IM true\n/BPC 1\nID \x00" as *const u8 as *const i8,
            (*pkh).bm_wd,
            (*pkh).bm_ht,
        );
        pdf_add_stream(stream, work_buffer.as_mut_ptr() as *const libc::c_void, len);
        /* Add bitmap data */
        if (*pkh).dyn_f == 14i32 {
            /* bitmap */
            pk_decode_bitmap(
                stream,
                (*pkh).bm_wd,
                (*pkh).bm_ht,
                (*pkh).dyn_f,
                (*pkh).run_color,
                pkt_ptr,
                pkt_len,
            );
        } else {
            pk_decode_packed(
                stream,
                (*pkh).bm_wd,
                (*pkh).bm_ht,
                (*pkh).dyn_f,
                (*pkh).run_color,
                pkt_ptr,
                pkt_len,
            );
        }
        len = sprintf(
            work_buffer.as_mut_ptr(),
            b"\nEI\nQ\x00" as *const u8 as *const i8,
        );
        pdf_add_stream(stream, work_buffer.as_mut_ptr() as *const libc::c_void, len);
    }
    return stream;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#[no_mangle]
pub unsafe extern "C" fn pdf_font_load_pkfont(mut font: *mut pdf_font) -> libc::c_int {
    let mut fontdict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut usedchars: *mut i8 = 0 as *mut i8;
    let mut ident: *mut i8 = 0 as *mut i8;
    let mut dpi: libc::c_uint = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut point_size: libc::c_double = 0.;
    let mut pix2charu: libc::c_double = 0.;
    let mut opcode: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut firstchar: libc::c_int = 0;
    let mut lastchar: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    let mut charprocs: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut procset: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut encoding: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut tmp_array: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut widths: [libc::c_double; 256] = [0.; 256];
    let mut bbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut charavail: [i8; 256] = [0; 256];
    let mut encoding_id: libc::c_int = 0;
    let mut enc_vec: *mut *mut i8 = 0 as *mut *mut i8;
    /* ENABLE_GLYPHENC */
    let mut error: libc::c_int = 0i32;
    if !pdf_font_is_in_use(font) {
        return 0i32;
    }
    ident = pdf_font_get_ident(font);
    point_size = pdf_font_get_param(font, 2i32);
    usedchars = pdf_font_get_usedchars(font);
    encoding_id = pdf_font_get_encoding(font);
    if encoding_id < 0i32 {
        enc_vec = 0 as *mut *mut i8
    } else {
        enc_vec = pdf_encoding_get_encoding(encoding_id)
    }
    /* ENABLE_GLYPHENC */
    if !ident.is_null() && !usedchars.is_null() && point_size > 0.0f64 {
    } else {
        __assert_fail(
            b"ident && usedchars && point_size > 0.0\x00" as *const u8 as *const i8,
            b"dpx-pkfont.c\x00" as *const u8 as *const i8,
            522i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[i8; 37]>(
                b"int pdf_font_load_pkfont(pdf_font *)\x00",
            ))
            .as_ptr(),
        );
    }
    dpi = truedpi(ident, point_size, base_dpi);
    fp = dpx_open_pk_font_at(ident, dpi);
    if fp.is_null() {
        _tt_abort(
            b"Could not find/open PK font file: %s (at %udpi)\x00" as *const u8
                as *const i8,
            ident,
            dpi,
        );
    }
    memset(
        charavail.as_mut_ptr() as *mut libc::c_void,
        0i32,
        256i32 as u64,
    );
    charprocs = pdf_new_dict();
    /* Include bitmap as 72dpi image:
     * There seems to be problems in "scaled" bitmap glyph
     * rendering in several viewers.
     */
    pix2charu = 72.0f64 * 1000.0f64 / base_dpi as libc::c_double / point_size; /* A command byte */
    bbox.lly = ::std::f64::INFINITY;
    bbox.llx = bbox.lly;
    bbox.ury = -::std::f64::INFINITY;
    bbox.urx = bbox.ury;
    loop {
        opcode = fgetc(fp);
        if !(opcode >= 0i32 && opcode != 245i32) {
            break;
        }
        if opcode < 240i32 {
            let mut pkh: pk_header_ = pk_header_ {
                pkt_len: 0,
                chrcode: 0,
                wd: 0,
                dx: 0,
                dy: 0,
                bm_wd: 0,
                bm_ht: 0,
                bm_hoff: 0,
                bm_voff: 0,
                dyn_f: 0,
                run_color: 0,
            };
            error = read_pk_char_header(&mut pkh, opcode as u8, fp);
            if error != 0 {
                _tt_abort(
                    b"Error in reading PK character header.\x00" as *const u8
                        as *const i8,
                );
            } else {
                if charavail[(pkh.chrcode & 0xffi32) as usize] != 0 {
                    dpx_warning(
                        b"More than two bitmap image for single glyph?: font=\"%s\" code=0x%02x\x00"
                            as *const u8 as *const i8,
                        ident,
                        pkh.chrcode,
                    );
                }
            }
            if *usedchars.offset((pkh.chrcode & 0xffi32) as isize) == 0 {
                skip_bytes(pkh.pkt_len, fp);
            } else {
                let mut charname: *mut i8 = 0 as *mut i8;
                let mut charproc: *mut pdf_obj = 0 as *mut pdf_obj;
                let mut pkt_ptr: *mut u8 = 0 as *mut u8;
                let mut bytesread: size_t = 0;
                let mut charwidth: libc::c_double = 0.;
                /* Charwidth in PDF units */
                charwidth = floor(
                    1000.0f64 * pkh.wd as libc::c_double
                        / ((1i32 << 20i32) as libc::c_double * pix2charu)
                        / 0.1f64
                        + 0.5f64,
                ) * 0.1f64;
                widths[(pkh.chrcode & 0xffi32) as usize] = charwidth;
                /* Update font BBox info */
                bbox.llx = if bbox.llx < -pkh.bm_hoff as libc::c_double {
                    bbox.llx
                } else {
                    -pkh.bm_hoff as libc::c_double
                };
                bbox.lly = if bbox.lly < pkh.bm_voff as libc::c_double - pkh.bm_ht as libc::c_double
                {
                    bbox.lly
                } else {
                    pkh.bm_voff as libc::c_double - pkh.bm_ht as libc::c_double
                };
                bbox.urx = if bbox.urx > pkh.bm_wd as libc::c_double - pkh.bm_hoff as libc::c_double
                {
                    bbox.urx
                } else {
                    pkh.bm_wd as libc::c_double - pkh.bm_hoff as libc::c_double
                };
                bbox.ury = if bbox.ury > pkh.bm_voff as libc::c_double {
                    bbox.ury
                } else {
                    pkh.bm_voff as libc::c_double
                };
                pkt_ptr = new((pkh.pkt_len as u64)
                    .wrapping_mul(::std::mem::size_of::<u8>() as u64)
                    as u32) as *mut u8;
                bytesread = fread(
                    pkt_ptr as *mut libc::c_void,
                    1i32 as u64,
                    pkh.pkt_len as u64,
                    fp,
                );
                if bytesread != pkh.pkt_len as u64 {
                    _tt_abort(
                        b"Only %zu bytes PK packet read. (expected %d bytes)\x00" as *const u8
                            as *const i8,
                        bytesread,
                        pkh.pkt_len,
                    );
                }
                charproc =
                    create_pk_CharProc_stream(&mut pkh, charwidth, pkt_ptr, bytesread as u32);
                free(pkt_ptr as *mut libc::c_void);
                if charproc.is_null() {
                    _tt_abort(
                        b"Unpacking PK character data failed.\x00" as *const u8
                            as *const i8,
                    );
                }
                if encoding_id >= 0i32 && !enc_vec.is_null() {
                    charname = *enc_vec.offset((pkh.chrcode & 0xffi32) as isize);
                    if charname.is_null() {
                        dpx_warning(
                            b"\".notdef\" glyph used in font (code=0x%02x): %s\x00" as *const u8
                                as *const i8,
                            pkh.chrcode,
                            ident,
                        );
                        charname = work_buffer.as_mut_ptr();
                        sprintf(
                            charname,
                            b"x%02X\x00" as *const u8 as *const i8,
                            pkh.chrcode as u8 as libc::c_int,
                        );
                    }
                } else {
                    /* ENABLE_GLYPHENC */
                    charname = work_buffer.as_mut_ptr(); /* _FIXME_ */
                    sprintf(
                        charname,
                        b"x%02X\x00" as *const u8 as *const i8,
                        pkh.chrcode as u8 as libc::c_int,
                    );
                }
                pdf_add_dict(charprocs, pdf_new_name(charname), pdf_ref_obj(charproc));
                pdf_release_obj(charproc);
            }
            charavail[(pkh.chrcode & 0xffi32) as usize] = 1i32 as i8
        } else {
            match opcode {
                240 | 241 | 242 | 243 => {
                    let mut len: int32_t =
                        get_unsigned_num(fp, (opcode - 240i32) as u8) as int32_t;
                    if len < 0i32 {
                        dpx_warning(
                            b"PK: Special with %d bytes???\x00" as *const u8 as *const i8,
                            len,
                        );
                    } else {
                        skip_bytes(len as libc::c_uint, fp);
                    }
                }
                244 => {
                    skip_bytes(4i32 as libc::c_uint, fp);
                }
                247 => {
                    do_preamble(fp);
                }
                246 | _ => {}
            }
        }
    }
    fclose(fp);
    /* Check if we really got all glyphs needed. */
    code = 0i32;
    while code < 256i32 {
        if *usedchars.offset(code as isize) as libc::c_int != 0 && charavail[code as usize] == 0 {
            dpx_warning(
                b"Missing glyph code=0x%02x in PK font \"%s\".\x00" as *const u8
                    as *const i8,
                code,
                ident,
            );
        }
        code += 1
    }
    /* Now actually fill fontdict. */
    fontdict = pdf_font_get_resource(font);
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"CharProcs\x00" as *const u8 as *const i8),
        pdf_ref_obj(charprocs),
    );
    pdf_release_obj(charprocs);
    /*
     * Resources:
     *
     *  PDF Reference 4th ed. describes it as "Optional but strongly recommended".
     *  There are no reason to put it in our case, but we will put this.
     *  We do not care about compatibility with Acrobat 2.x. (See implementation
     *  note 47, Appendix H of PDF Ref., 4th ed.).
     */
    procset = pdf_new_dict();
    tmp_array = pdf_new_array();
    pdf_add_array(
        tmp_array,
        pdf_new_name(b"PDF\x00" as *const u8 as *const i8),
    );
    pdf_add_array(
        tmp_array,
        pdf_new_name(b"ImageB\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        procset,
        pdf_new_name(b"ProcSet\x00" as *const u8 as *const i8),
        tmp_array,
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"Resources\x00" as *const u8 as *const i8),
        procset,
    );
    /* Encoding */
    tmp_array = pdf_new_array();
    prev = -2i32;
    firstchar = 255i32;
    lastchar = 0i32;
    code = 0i32;
    while code < 256i32 {
        let mut charname_0: *mut i8 = 0 as *mut i8;
        if *usedchars.offset(code as isize) != 0 {
            if code < firstchar {
                firstchar = code
            }
            if code > lastchar {
                lastchar = code
            }
            if code != prev + 1i32 {
                pdf_add_array(tmp_array, pdf_new_number(code as libc::c_double));
            }
            if encoding_id >= 0i32 && !enc_vec.is_null() {
                charname_0 = *enc_vec.offset(code as u8 as isize);
                if charname_0.is_null() {
                    charname_0 = work_buffer.as_mut_ptr();
                    sprintf(
                        charname_0,
                        b"x%02X\x00" as *const u8 as *const i8,
                        code as u8 as libc::c_int,
                    );
                }
            } else {
                /* ENABLE_GLYPHENC */
                charname_0 = work_buffer.as_mut_ptr();
                sprintf(
                    charname_0,
                    b"x%02X\x00" as *const u8 as *const i8,
                    code as u8 as libc::c_int,
                );
            }
            pdf_add_array(tmp_array, pdf_new_name(charname_0));
            prev = code
        }
        code += 1
    }
    if firstchar > lastchar {
        pdf_release_obj(tmp_array);
        _tt_abort(
            b"Unexpected error: firstchar > lastchar (%d %d)\x00" as *const u8
                as *const i8,
            firstchar,
            lastchar,
        );
    }
    if encoding_id < 0i32 || enc_vec.is_null() {
        /* ENABLE_GLYPHENC */
        encoding = pdf_new_dict();
        pdf_add_dict(
            encoding,
            pdf_new_name(b"Type\x00" as *const u8 as *const i8),
            pdf_new_name(b"Encoding\x00" as *const u8 as *const i8),
        );
        pdf_add_dict(
            encoding,
            pdf_new_name(b"Differences\x00" as *const u8 as *const i8),
            tmp_array,
        );
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"Encoding\x00" as *const u8 as *const i8),
            pdf_ref_obj(encoding),
        );
        pdf_release_obj(encoding);
    } else {
        pdf_release_obj(tmp_array);
    }
    /* FontBBox: Accurate value is important.
     */
    tmp_array = pdf_new_array();
    pdf_add_array(tmp_array, pdf_new_number(bbox.llx));
    pdf_add_array(tmp_array, pdf_new_number(bbox.lly));
    pdf_add_array(tmp_array, pdf_new_number(bbox.urx));
    pdf_add_array(tmp_array, pdf_new_number(bbox.ury));
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FontBBox\x00" as *const u8 as *const i8),
        tmp_array,
    );
    /* Widths:
     *  Indirect reference preffered. (See PDF Reference)
     */
    tmp_array = pdf_new_array();
    code = firstchar;
    while code <= lastchar {
        if *usedchars.offset(code as isize) != 0 {
            pdf_add_array(tmp_array, pdf_new_number(widths[code as usize]));
        } else {
            pdf_add_array(tmp_array, pdf_new_number(0i32 as libc::c_double));
        }
        code += 1
    }
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"Widths\x00" as *const u8 as *const i8),
        pdf_ref_obj(tmp_array),
    );
    pdf_release_obj(tmp_array);
    /* FontMatrix */
    tmp_array = pdf_new_array();
    pdf_add_array(tmp_array, pdf_new_number(0.001f64 * pix2charu));
    pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    pdf_add_array(tmp_array, pdf_new_number(0.001f64 * pix2charu));
    pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FontMatrix\x00" as *const u8 as *const i8),
        tmp_array,
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FirstChar\x00" as *const u8 as *const i8),
        pdf_new_number(firstchar as libc::c_double),
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"LastChar\x00" as *const u8 as *const i8),
        pdf_new_number(lastchar as libc::c_double),
    );
    return 0i32;
}
