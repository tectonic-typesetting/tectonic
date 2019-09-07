#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(extern_types)]
extern crate libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
    /* Here is the complete list of PDF object types */
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    pub type pdf_ximage_;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn pdf_ximage_set_image(
        ximage: *mut pdf_ximage,
        info: *mut libc::c_void,
        resource: *mut pdf_obj,
    );
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    #[no_mangle]
    fn rewind(__stream: *mut FILE);
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
       mostly unsigned (uint32_t) and occasionally signed (int32_t).
    */
    #[no_mangle]
    fn get_unsigned_byte(_: *mut FILE) -> libc::c_uchar;
    #[no_mangle]
    fn get_unsigned_pair(_: *mut FILE) -> libc::c_ushort;
    #[no_mangle]
    fn get_unsigned_quad(_: *mut FILE) -> uint32_t;
    #[no_mangle]
    fn seek_relative(file: *mut FILE, pos: int32_t);
    #[no_mangle]
    fn file_size(file: *mut FILE) -> int32_t;
    #[no_mangle]
    static mut work_buffer: [libc::c_char; 0];
    #[no_mangle]
    fn pdf_get_version() -> libc::c_uint;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_new_stream(flags: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: libc::c_int,
    );
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ximage_init_image_info(info: *mut ximage_info);
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ximage_info {
    pub flags: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub bits_per_component: libc::c_int,
    pub num_components: libc::c_int,
    pub min_dpi: libc::c_int,
    pub xdensity: libc::c_double,
    pub ydensity: libc::c_double,
}
pub type pdf_ximage = pdf_ximage_;
/* Label */
unsafe extern "C" fn read_box_hdr(
    mut fp: *mut FILE,
    mut lbox: *mut libc::c_uint,
    mut tbox: *mut libc::c_uint,
) -> libc::c_uint {
    let mut bytesread: libc::c_uint = 0i32 as libc::c_uint;
    *lbox = get_unsigned_quad(fp);
    *tbox = get_unsigned_quad(fp);
    bytesread = bytesread.wrapping_add(8i32 as libc::c_uint);
    if *lbox == 1i32 as libc::c_uint {
        if get_unsigned_quad(fp) != 0i32 as libc::c_uint {
            _tt_abort(
                b"JPEG2000: LBox value in JP2 file >32 bits.\nI can\'t handle this!\x00"
                    as *const u8 as *const libc::c_char,
            );
        }
        *lbox = get_unsigned_quad(fp);
        bytesread = bytesread.wrapping_add(8i32 as libc::c_uint)
    } else if *lbox > 1i32 as libc::c_uint && *lbox < 8i32 as libc::c_uint {
        dpx_warning(
            b"JPEG2000: Unknown LBox value %u in JP2 file!\x00" as *const u8 as *const libc::c_char,
            *lbox,
        );
    }
    return bytesread;
}
unsafe extern "C" fn check_jp___box(mut fp: *mut FILE) -> libc::c_int {
    if get_unsigned_quad(fp) != 0xci32 as libc::c_uint {
        return 0i32;
    }
    if get_unsigned_quad(fp) != 0x6a502020i32 as libc::c_uint {
        return 0i32;
    }
    /* Next 4 bytes shall be 0D 0A 87 0A */
    if get_unsigned_quad(fp) != 0xd0a870ai32 as libc::c_uint {
        return 0i32;
    }
    return 1i32;
}
unsafe extern "C" fn check_ftyp_data(mut fp: *mut FILE, mut size: libc::c_uint) -> libc::c_int {
    let mut supported: libc::c_int = 0i32;
    let mut BR: libc::c_uint = 0;
    let mut CLi: libc::c_uint = 0;
    BR = get_unsigned_quad(fp);
    size = size.wrapping_sub(4i32 as libc::c_uint);
    /* MinV = */
    get_unsigned_quad(fp);
    size = size.wrapping_sub(4i32 as libc::c_uint);
    match BR {
        1785737760 => {
            /* "jp2 " ... supported */
            seek_relative(fp, size as int32_t);
            size = 0i32 as libc::c_uint;
            supported = 1i32
        }
        1785755680 => {
            /* "jpx " ... baseline subset supported */
            while size > 0i32 as libc::c_uint {
                CLi = get_unsigned_quad(fp);
                if CLi == 0x6a707862i32 as libc::c_uint {
                    supported = 1i32
                }
                size = size.wrapping_sub(4i32 as libc::c_uint)
            }
        }
        _ => {
            dpx_warning(
                b"JPEG2000: Unknown JPEG 2000 File Type box Brand field value.\x00" as *const u8
                    as *const libc::c_char,
            );
            seek_relative(fp, size as int32_t);
            size = 0i32 as libc::c_uint;
            supported = 0i32
        }
    }
    return supported;
}
unsafe extern "C" fn read_res__data(
    mut info: *mut ximage_info,
    mut fp: *mut FILE,
    mut size: libc::c_uint,
) {
    let mut VR_N: libc::c_uint = 0;
    let mut VR_D: libc::c_uint = 0;
    let mut HR_N: libc::c_uint = 0;
    let mut HR_D: libc::c_uint = 0;
    let mut VR_E: libc::c_uchar = 0;
    let mut HR_E: libc::c_uchar = 0;
    VR_N = get_unsigned_pair(fp) as libc::c_uint;
    VR_D = get_unsigned_pair(fp) as libc::c_uint;
    HR_N = get_unsigned_pair(fp) as libc::c_uint;
    HR_D = get_unsigned_pair(fp) as libc::c_uint;
    VR_E = get_unsigned_byte(fp);
    HR_E = get_unsigned_byte(fp);
    (*info).xdensity = 72.0f64
        / (HR_N as libc::c_double / HR_D as libc::c_double
            * pow(10.0f64, HR_E as libc::c_double)
            * 0.0254f64);
    (*info).ydensity = 72.0f64
        / (VR_N as libc::c_double / VR_D as libc::c_double
            * pow(10.0f64, VR_E as libc::c_double)
            * 0.0254f64);
}
unsafe extern "C" fn scan_res_(
    mut info: *mut ximage_info,
    mut fp: *mut FILE,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut len: libc::c_uint = 0;
    let mut lbox: libc::c_uint = 0;
    let mut tbox: libc::c_uint = 0;
    let mut have_resd: libc::c_int = 0i32;
    while size > 0i32 as libc::c_uint {
        len = read_box_hdr(fp, &mut lbox, &mut tbox);
        if lbox == 0i32 as libc::c_uint {
            dpx_warning(
                b"JPEG2000: Unexpected lbox value 0 in JP2 Resolution box.\x00" as *const u8
                    as *const libc::c_char,
            );
            break;
        } else {
            match tbox {
                1919251299 => {
                    if have_resd == 0 {
                        read_res__data(info, fp, lbox.wrapping_sub(len));
                    } else {
                        seek_relative(fp, lbox.wrapping_sub(len) as int32_t);
                    }
                }
                1919251300 => {
                    read_res__data(info, fp, lbox.wrapping_sub(len));
                    have_resd = 1i32
                }
                _ => {
                    dpx_warning(
                        b"JPEG2000: Unknown JPEG 2000 box type in Resolution box.\x00" as *const u8
                            as *const libc::c_char,
                    );
                    seek_relative(fp, lbox.wrapping_sub(len) as int32_t);
                }
            }
            size = size.wrapping_sub(lbox)
        }
    }
    return if size == 0i32 as libc::c_uint {
        0i32
    } else {
        -1i32
    };
}
/* Acrobat seems require Channel Definition box to be defined when image data
 * contains opacity channel. However, OpenJPEG (and maybe most of JPEG 2000 coders?)
 * does not write Channel Definition box so transparency will be ignored.
 */
unsafe extern "C" fn scan_cdef(
    mut info: *mut ximage_info,
    mut smask: *mut libc::c_int,
    mut fp: *mut FILE,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut opacity_channels: libc::c_int = 0i32; /* Cn */
    let mut have_type0: libc::c_int = 0i32; /* must be 0 for SMask */
    let mut i: libc::c_uint = 0;
    let mut Cn: libc::c_uint = 0;
    let mut N: libc::c_uint = 0;
    let mut Typ: libc::c_uint = 0;
    let mut Asoc: libc::c_uint = 0;
    *smask = 0i32;
    N = get_unsigned_pair(fp) as libc::c_uint;
    if size
        < N.wrapping_mul(6i32 as libc::c_uint)
            .wrapping_add(2i32 as libc::c_uint)
    {
        dpx_warning(
            b"JPEG2000: Inconsistent N value in Channel Definition box.\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    i = 0i32 as libc::c_uint;
    while i < N {
        Cn = get_unsigned_pair(fp) as libc::c_uint;
        Typ = get_unsigned_pair(fp) as libc::c_uint;
        Asoc = get_unsigned_pair(fp) as libc::c_uint;
        if Cn > N {
            dpx_warning(
                b"JPEG2000: Invalid Cn value in Channel Definition box.\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if Typ == 1i32 as libc::c_uint {
            if Asoc == 0i32 as libc::c_uint {
                have_type0 = 1i32
            }
            opacity_channels += 1
        } else if Typ == 2i32 as libc::c_uint {
            opacity_channels += 1
        }
        i = i.wrapping_add(1)
    }
    if opacity_channels == 1i32 {
        *smask = if have_type0 != 0 { 1i32 } else { 0i32 }
    } else if opacity_channels > 1i32 {
        dpx_warning(
            b"JPEG2000: Unsupported transparency type. (ignored)\x00" as *const u8
                as *const libc::c_char,
        );
    }
    return 0i32;
}
unsafe extern "C" fn scan_jp2h(
    mut info: *mut ximage_info,
    mut smask: *mut libc::c_int,
    mut fp: *mut FILE,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut have_ihdr: libc::c_int = 0i32;
    let mut len: libc::c_uint = 0;
    let mut lbox: libc::c_uint = 0;
    let mut tbox: libc::c_uint = 0;
    while size > 0i32 as libc::c_uint && error == 0 {
        len = read_box_hdr(fp, &mut lbox, &mut tbox);
        if lbox == 0i32 as libc::c_uint {
            dpx_warning(
                b"JPEG2000: Unexpected lbox value 0 in JP2 Header box...\x00" as *const u8
                    as *const libc::c_char,
            );
            error = -1i32;
            break;
        } else {
            match tbox {
                1768449138 => {
                    (*info).height = get_unsigned_quad(fp) as libc::c_int;
                    (*info).width = get_unsigned_quad(fp) as libc::c_int;
                    (*info).num_components = get_unsigned_pair(fp) as libc::c_int;
                    /* c = */
                    get_unsigned_byte(fp); /* BPC - 1 */
                    /* c = */
                    get_unsigned_byte(fp); /* C: Compression type */
                    /* c = */
                    get_unsigned_byte(fp); /* UnkC */
                    /* c = */
                    get_unsigned_byte(fp); /* IPR */
                    have_ihdr = 1i32
                }
                1919251232 => error = scan_res_(info, fp, lbox.wrapping_sub(len)),
                1667523942 => error = scan_cdef(info, smask, fp, lbox.wrapping_sub(len)),
                1651532643 | 1668246642 | 1885564018 | 1668112752 | 1818389536 => {
                    seek_relative(fp, lbox.wrapping_sub(len) as int32_t);
                }
                _ => {
                    dpx_warning(
                        b"JPEG2000: Unknown JPEG 2000 box in JP2 Header box.\x00" as *const u8
                            as *const libc::c_char,
                    );
                    seek_relative(fp, lbox.wrapping_sub(len) as int32_t);
                    error = -1i32
                }
            }
            size = size.wrapping_sub(lbox)
        }
    }
    if have_ihdr == 0 {
        dpx_warning(
            b"JPEG2000: Expecting JPEG 2000 Image Header box but could not find.\x00" as *const u8
                as *const libc::c_char,
        );
    }
    return if error == 0 && have_ihdr != 0 && size == 0i32 as libc::c_uint {
        0i32
    } else {
        -1i32
    };
}
unsafe extern "C" fn scan_file(
    mut info: *mut ximage_info,
    mut smask: *mut libc::c_int,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut error: libc::c_int = 0i32;
    let mut have_jp2h: libc::c_int = 0i32;
    let mut size: libc::c_int = 0;
    let mut len: libc::c_uint = 0;
    let mut lbox: libc::c_uint = 0;
    let mut tbox: libc::c_uint = 0;
    size = file_size(fp);
    /* Should have already been checked before. */
    /* JPEG 2000 Singature box */
    if check_jp___box(fp) == 0 {
        return -1i32;
    }
    size -= 12i32;
    /* File Type box shall immediately follow */
    len = read_box_hdr(fp, &mut lbox, &mut tbox);
    if tbox != 0x66747970i32 as libc::c_uint {
        return -1i32;
    }
    if check_ftyp_data(fp, lbox.wrapping_sub(len)) == 0 {
        return -1i32;
    }
    size = (size as libc::c_uint).wrapping_sub(lbox) as libc::c_int as libc::c_int;
    /* Search for JP2 Header box */
    while size > 0i32 && error == 0 {
        len = read_box_hdr(fp, &mut lbox, &mut tbox);
        if lbox == 0i32 as libc::c_uint {
            lbox = size as libc::c_uint
        }
        match tbox {
            1785737832 => {
                error = scan_jp2h(info, smask, fp, lbox.wrapping_sub(len));
                have_jp2h = 1i32
            }
            1785737827 => {
                /* JP2 requires JP2H appears before JP2C. */
                if have_jp2h == 0 {
                    dpx_warning(
                        b"JPEG2000: JPEG 2000 Codestream box found before JP2 Header box.\x00"
                            as *const u8 as *const libc::c_char,
                    );
                }
                seek_relative(fp, lbox.wrapping_sub(len) as int32_t);
            }
            _ => {
                seek_relative(fp, lbox.wrapping_sub(len) as int32_t);
            }
        }
        size = (size as libc::c_uint).wrapping_sub(lbox) as libc::c_int as libc::c_int
    }
    /* From ISO/IEC 15444-2 M.9.2.7
     * The JP2 Header box shall be found in the file before the first
     * Contiguous Codestream box, Fragment Table box, Media Data box,
     * Codestream Header box, and Compositing Layer Header box. ...
     */
    if have_jp2h == 0 && error == 0 {
        dpx_warning(
            b"JPEG2000: No JP2 Header box found. Not a JP2/JPX baseline file?\x00" as *const u8
                as *const libc::c_char,
        );
        error = -1i32
    }
    return error;
}
#[no_mangle]
pub unsafe extern "C" fn check_for_jp2(mut fp: *mut FILE) -> libc::c_int {
    let mut len: libc::c_uint = 0;
    let mut lbox: libc::c_uint = 0;
    let mut tbox: libc::c_uint = 0;
    if fp.is_null() {
        return 0i32;
    }
    rewind(fp);
    /* JPEG 2000 Singature box */
    if check_jp___box(fp) == 0 {
        return 0i32;
    }
    /* File Type box shall immediately follow */
    len = read_box_hdr(fp, &mut lbox, &mut tbox);
    if tbox != 0x66747970i32 as libc::c_uint {
        return 0i32;
    }
    if check_ftyp_data(fp, lbox.wrapping_sub(len)) == 0 {
        return 0i32;
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn jp2_include_image(
    mut ximage: *mut pdf_ximage,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut pdf_version: libc::c_uint = 0;
    let mut smask: libc::c_int = 0i32;
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut info: ximage_info = ximage_info {
        flags: 0,
        width: 0,
        height: 0,
        bits_per_component: 0,
        num_components: 0,
        min_dpi: 0,
        xdensity: 0.,
        ydensity: 0.,
    };
    pdf_version = pdf_get_version();
    if pdf_version < 5i32 as libc::c_uint {
        dpx_warning(
            b"JPEG 2000 support requires PDF version >= 1.5 (Current setting 1.%d)\n\x00"
                as *const u8 as *const libc::c_char,
            pdf_version,
        );
        return -1i32;
    }
    pdf_ximage_init_image_info(&mut info);
    stream_dict = 0 as *mut pdf_obj;
    stream = stream_dict;
    rewind(fp);
    if scan_file(&mut info, &mut smask, fp) < 0i32 {
        dpx_warning(
            b"JPEG2000: Reading JPEG 2000 file failed.\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    stream = pdf_new_stream(0i32);
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Filter\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"JPXDecode\x00" as *const u8 as *const libc::c_char),
    );
    if smask != 0 {
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"SMaskInData\x00" as *const u8 as *const libc::c_char),
            pdf_new_number(1i32 as libc::c_double),
        );
    }
    /* Read whole file */
    let mut nb_read: libc::c_int = 0;
    rewind(fp);
    loop {
        nb_read = fread(
            work_buffer.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            1024i32 as libc::c_ulong,
            fp,
        ) as libc::c_int;
        if !(nb_read > 0i32) {
            break;
        }
        pdf_add_stream(
            stream,
            work_buffer.as_mut_ptr() as *const libc::c_void,
            nb_read,
        );
    }
    pdf_ximage_set_image(
        ximage,
        &mut info as *mut ximage_info as *mut libc::c_void,
        stream,
    );
    return 0i32;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Matthias Franz,
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
pub unsafe extern "C" fn jp2_get_bbox(
    mut fp: *mut FILE,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
    mut xdensity: *mut libc::c_double,
    mut ydensity: *mut libc::c_double,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut smask: libc::c_int = 0i32;
    let mut info: ximage_info = ximage_info {
        flags: 0,
        width: 0,
        height: 0,
        bits_per_component: 0,
        num_components: 0,
        min_dpi: 0,
        xdensity: 0.,
        ydensity: 0.,
    };
    pdf_ximage_init_image_info(&mut info);
    rewind(fp);
    r = scan_file(&mut info, &mut smask, fp);
    *width = info.width;
    *height = info.height;
    *xdensity = info.xdensity;
    *ydensity = info.ydensity;
    return r;
}
