#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type pdf_obj;
    pub type pdf_ximage_;
    #[no_mangle]
    fn pdf_ximage_set_image(
        ximage: *mut pdf_ximage,
        info: *mut libc::c_void,
        resource: *mut pdf_obj,
    );
    #[no_mangle]
    fn pdf_ximage_init_image_info(info: *mut ximage_info);
    #[no_mangle]
    fn pdf_get_colorspace_reference(cspc_id: libc::c_int) -> *mut pdf_obj;
    #[no_mangle]
    fn iccp_load_profile(
        ident: *const libc::c_char,
        profile: *const libc::c_void,
        proflen: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn iccp_check_colorspace(
        colortype: libc::c_int,
        profile: *const libc::c_void,
        proflen: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn iccp_get_rendering_intent(
        profile: *const libc::c_void,
        proflen: libc::c_int,
    ) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_stream_dataptr(stream: *mut pdf_obj) -> *const libc::c_void;
    #[no_mangle]
    fn pdf_stream_length(stream: *mut pdf_obj) -> libc::c_int;
    #[no_mangle]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
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
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_get_version() -> libc::c_uint;
    #[no_mangle]
    static mut work_buffer: [libc::c_char; 0];
    #[no_mangle]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> libc::c_ushort;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> libc::c_uchar;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn ttstub_input_getc(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn dpx_warning(fmt: *const libc::c_char, _: ...);
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
    fn new(size: uint32_t) -> *mut libc::c_void;
    #[no_mangle]
    fn renew(p: *mut libc::c_void, size: uint32_t) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
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
pub const JM_SOI: JPEG_marker = 216;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_info {
    pub height: uint16_t,
    pub width: uint16_t,
    pub bits_per_component: uint8_t,
    pub num_components: uint8_t,
    pub xdpi: libc::c_double,
    pub ydpi: libc::c_double,
    pub flags: libc::c_int,
    pub num_appn: libc::c_int,
    pub max_appn: libc::c_int,
    pub appn: *mut JPEG_ext,
    pub skipbits: [libc::c_char; 129],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_ext {
    pub marker: JPEG_marker,
    pub app_sig: JPEG_APPn_sig,
    pub app_data: *mut libc::c_void,
}
pub type JPEG_APPn_sig = libc::c_uint;
pub const JS_APPn_XMP: JPEG_APPn_sig = 3;
pub const JS_APPn_ICC: JPEG_APPn_sig = 2;
pub const JS_APPn_ADOBE: JPEG_APPn_sig = 1;
pub const JS_APPn_JFIF: JPEG_APPn_sig = 0;
pub type JPEG_marker = libc::c_uint;
pub const JM_COM: JPEG_marker = 254;
pub const JM_APP15: JPEG_marker = 239;
pub const JM_APP14: JPEG_marker = 238;
pub const JM_APP2: JPEG_marker = 226;
pub const JM_APP1: JPEG_marker = 225;
pub const JM_APP0: JPEG_marker = 224;
pub const JM_EXP: JPEG_marker = 223;
pub const JM_DHP: JPEG_marker = 222;
pub const JM_DRI: JPEG_marker = 221;
pub const JM_DNL: JPEG_marker = 220;
pub const JM_DQT: JPEG_marker = 219;
pub const JM_SOS: JPEG_marker = 218;
pub const JM_EOI: JPEG_marker = 217;
pub const JM_RST7: JPEG_marker = 215;
pub const JM_RST6: JPEG_marker = 214;
pub const JM_RST5: JPEG_marker = 213;
pub const JM_RST4: JPEG_marker = 212;
pub const JM_RST3: JPEG_marker = 211;
pub const JM_RST2: JPEG_marker = 210;
pub const JM_RST1: JPEG_marker = 209;
pub const JM_RST0: JPEG_marker = 208;
pub const JM_SOF15: JPEG_marker = 207;
pub const JM_SOF14: JPEG_marker = 206;
pub const JM_SOF13: JPEG_marker = 205;
pub const JM_DAC: JPEG_marker = 204;
pub const JM_SOF11: JPEG_marker = 203;
pub const JM_SOF10: JPEG_marker = 202;
pub const JM_SOF9: JPEG_marker = 201;
pub const JM_SOF7: JPEG_marker = 199;
pub const JM_SOF6: JPEG_marker = 198;
pub const JM_DHT: JPEG_marker = 196;
pub const JM_SOF5: JPEG_marker = 197;
pub const JM_SOF3: JPEG_marker = 195;
pub const JM_SOF2: JPEG_marker = 194;
pub const JM_SOF1: JPEG_marker = 193;
pub const JM_SOF0: JPEG_marker = 192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_APPn_XMP {
    pub packet: *mut libc::c_uchar,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_APPn_Adobe {
    pub version: uint16_t,
    pub flag0: uint16_t,
    pub flag1: uint16_t,
    pub transform: uint8_t,
    /* color transform code */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_APPn_ICC {
    pub seq_id: uint8_t,
    pub num_chunks: uint8_t,
    pub chunk: *mut libc::c_uchar,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_APPn_JFIF {
    pub version: uint16_t,
    pub units: uint8_t,
    pub Xdensity: uint16_t,
    pub Ydensity: uint16_t,
    pub Xthumbnail: uint8_t,
    pub Ythumbnail: uint8_t,
    pub thumbnail: *mut libc::c_uchar,
    /* Thumbnail data. */
}
/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn check_for_jpeg(mut handle: rust_input_handle_t) -> libc::c_int {
    let mut jpeg_sig: [libc::c_uchar; 2] = [0; 2];
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if ttstub_input_read(
        handle,
        jpeg_sig.as_mut_ptr() as *mut libc::c_char,
        2i32 as size_t,
    ) != 2i32 as libc::c_long
    {
        return 0i32;
    } else {
        if jpeg_sig[0] as libc::c_int != 0xffi32
            || jpeg_sig[1] as libc::c_int != JM_SOI as libc::c_int
        {
            return 0i32;
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut colorspace: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut colortype: libc::c_int = 0;
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
    let mut j_info: JPEG_info = JPEG_info {
        height: 0,
        width: 0,
        bits_per_component: 0,
        num_components: 0,
        xdpi: 0.,
        ydpi: 0.,
        flags: 0,
        num_appn: 0,
        max_appn: 0,
        appn: 0 as *mut JPEG_ext,
        skipbits: [0; 129],
    };
    if check_for_jpeg(handle) == 0 {
        dpx_warning(
            b"%s: Not a JPEG file?\x00" as *const u8 as *const libc::c_char,
            b"JPEG\x00" as *const u8 as *const libc::c_char,
        );
        ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
        return -1i32;
    }
    /* File position is 2 here... */
    pdf_ximage_init_image_info(&mut info);
    JPEG_info_init(&mut j_info);
    if JPEG_scan_file(&mut j_info, handle) < 0i32 {
        dpx_warning(
            b"%s: Not a JPEG file?\x00" as *const u8 as *const libc::c_char,
            b"JPEG\x00" as *const u8 as *const libc::c_char,
        );
        JPEG_info_clear(&mut j_info);
        return -1i32;
    }
    match j_info.num_components as libc::c_int {
        1 => colortype = -1i32,
        3 => colortype = -3i32,
        4 => colortype = -4i32,
        _ => {
            dpx_warning(
                b"%s: Unknown color space (num components: %d)\x00" as *const u8
                    as *const libc::c_char,
                b"JPEG\x00" as *const u8 as *const libc::c_char,
                info.num_components,
            );
            JPEG_info_clear(&mut j_info);
            return -1i32;
        }
    }
    /* JPEG image use DCTDecode. */
    stream = pdf_new_stream(0i32);
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Filter\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"DCTDecode\x00" as *const u8 as *const libc::c_char),
    );
    /* XMP Metadata */
    if pdf_get_version() >= 4i32 as libc::c_uint {
        if j_info.flags & 1i32 << 4i32 != 0 {
            let mut XMP_stream: *mut pdf_obj = 0 as *mut pdf_obj;
            XMP_stream = JPEG_get_XMP(&mut j_info);
            pdf_add_dict(
                stream_dict,
                pdf_new_name(b"Metadata\x00" as *const u8 as *const libc::c_char),
                pdf_ref_obj(XMP_stream),
            );
            pdf_release_obj(XMP_stream);
        }
    }
    /* Check embedded ICC Profile */
    colorspace = 0 as *mut pdf_obj;
    if j_info.flags & 1i32 << 2i32 != 0 {
        let mut icc_stream: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut intent: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut cspc_id: libc::c_int = 0;
        icc_stream = JPEG_get_iccp(&mut j_info);
        if icc_stream.is_null() {
            colorspace = 0 as *mut pdf_obj
        } else {
            if iccp_check_colorspace(
                colortype,
                pdf_stream_dataptr(icc_stream),
                pdf_stream_length(icc_stream),
            ) < 0i32
            {
                colorspace = 0 as *mut pdf_obj
            } else {
                cspc_id = iccp_load_profile(
                    0 as *const libc::c_char,
                    pdf_stream_dataptr(icc_stream),
                    pdf_stream_length(icc_stream),
                );
                if cspc_id < 0i32 {
                    colorspace = 0 as *mut pdf_obj
                } else {
                    colorspace = pdf_get_colorspace_reference(cspc_id);
                    intent = iccp_get_rendering_intent(
                        pdf_stream_dataptr(icc_stream),
                        pdf_stream_length(icc_stream),
                    );
                    if !intent.is_null() {
                        pdf_add_dict(
                            stream_dict,
                            pdf_new_name(b"Intent\x00" as *const u8 as *const libc::c_char),
                            intent,
                        );
                    }
                }
            }
            pdf_release_obj(icc_stream);
        }
    }
    /* No ICC or invalid ICC profile. */
    if colorspace.is_null() {
        match colortype {
            -1 => colorspace = pdf_new_name(b"DeviceGray\x00" as *const u8 as *const libc::c_char),
            -3 => colorspace = pdf_new_name(b"DeviceRGB\x00" as *const u8 as *const libc::c_char),
            -4 => colorspace = pdf_new_name(b"DeviceCMYK\x00" as *const u8 as *const libc::c_char),
            _ => {}
        }
    }
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"ColorSpace\x00" as *const u8 as *const libc::c_char),
        colorspace,
    );
    if j_info.flags & 1i32 << 1i32 != 0 && j_info.num_components as libc::c_int == 4i32 {
        let mut decode: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut i: libc::c_uint = 0;
        dpx_warning(
            b"Adobe CMYK JPEG: Inverted color assumed.\x00" as *const u8 as *const libc::c_char,
        );
        decode = pdf_new_array();
        i = 0i32 as libc::c_uint;
        while i < j_info.num_components as libc::c_uint {
            pdf_add_array(decode, pdf_new_number(1.0f64));
            pdf_add_array(decode, pdf_new_number(0.0f64));
            i = i.wrapping_add(1)
        }
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"Decode\x00" as *const u8 as *const libc::c_char),
            decode,
        );
    }
    /* Copy file */
    JPEG_copy_stream(&mut j_info, stream, handle);
    info.width = j_info.width as libc::c_int;
    info.height = j_info.height as libc::c_int;
    info.bits_per_component = j_info.bits_per_component as libc::c_int;
    info.num_components = j_info.num_components as libc::c_int;
    jpeg_get_density(&mut j_info, &mut info.xdensity, &mut info.ydensity);
    pdf_ximage_set_image(
        ximage,
        &mut info as *mut ximage_info as *mut libc::c_void,
        stream,
    );
    JPEG_info_clear(&mut j_info);
    return 0i32;
}
unsafe extern "C" fn jpeg_get_density(
    mut j_info: *mut JPEG_info,
    mut xdensity: *mut libc::c_double,
    mut ydensity: *mut libc::c_double,
) {
    /*
     * j_info->xdpi and j_info->ydpi are determined in most cases
     * in JPEG_scan_file(). FIXME: However, in some kinds of JPEG files,
     * j_info->xdpi, and j_info->ydpi are not determined in
     * JPEG_scan_file(). In this case we assume
     * that j_info->xdpi = j_info->ydpi = 72.0.
     */
    if (*j_info).xdpi < 0.1f64 && (*j_info).ydpi < 0.1f64 {
        (*j_info).ydpi = 72.0f64;
        (*j_info).xdpi = (*j_info).ydpi
    }
    *xdensity = 72.0f64 / (*j_info).xdpi;
    *ydensity = 72.0f64 / (*j_info).ydpi;
}
unsafe extern "C" fn JPEG_info_init(mut j_info: *mut JPEG_info) {
    (*j_info).width = 0i32 as uint16_t;
    (*j_info).height = 0i32 as uint16_t;
    (*j_info).bits_per_component = 0i32 as uint8_t;
    (*j_info).num_components = 0i32 as uint8_t;
    (*j_info).xdpi = 0.0f64;
    (*j_info).ydpi = 0.0f64;
    (*j_info).flags = 0i32;
    (*j_info).num_appn = 0i32;
    (*j_info).max_appn = 0i32;
    (*j_info).appn = 0 as *mut JPEG_ext;
    memset(
        (*j_info).skipbits.as_mut_ptr() as *mut libc::c_void,
        0i32,
        (1024i32 / 8i32 + 1i32) as libc::c_ulong,
    );
}
unsafe extern "C" fn JPEG_release_APPn_data(
    mut marker: JPEG_marker,
    mut app_sig: JPEG_APPn_sig,
    mut app_data: *mut libc::c_void,
) {
    if marker as libc::c_uint == JM_APP0 as libc::c_int as libc::c_uint
        && app_sig as libc::c_uint == JS_APPn_JFIF as libc::c_int as libc::c_uint
    {
        let mut data: *mut JPEG_APPn_JFIF = 0 as *mut JPEG_APPn_JFIF;
        data = app_data as *mut JPEG_APPn_JFIF;
        (*data).thumbnail = mfree((*data).thumbnail as *mut libc::c_void) as *mut libc::c_uchar;
        free(data as *mut libc::c_void);
    } else if marker as libc::c_uint == JM_APP2 as libc::c_int as libc::c_uint
        && app_sig as libc::c_uint == JS_APPn_ICC as libc::c_int as libc::c_uint
    {
        let mut data_0: *mut JPEG_APPn_ICC = 0 as *mut JPEG_APPn_ICC;
        data_0 = app_data as *mut JPEG_APPn_ICC;
        (*data_0).chunk = mfree((*data_0).chunk as *mut libc::c_void) as *mut libc::c_uchar;
        free(data_0 as *mut libc::c_void);
    } else if marker as libc::c_uint == JM_APP14 as libc::c_int as libc::c_uint
        && app_sig as libc::c_uint == JS_APPn_ADOBE as libc::c_int as libc::c_uint
    {
        let mut data_1: *mut JPEG_APPn_Adobe = 0 as *mut JPEG_APPn_Adobe;
        data_1 = app_data as *mut JPEG_APPn_Adobe;
        free(data_1 as *mut libc::c_void);
    } else if marker as libc::c_uint == JM_APP1 as libc::c_int as libc::c_uint
        && app_sig as libc::c_uint == JS_APPn_XMP as libc::c_int as libc::c_uint
    {
        let mut data_2: *mut JPEG_APPn_XMP = 0 as *mut JPEG_APPn_XMP;
        data_2 = app_data as *mut JPEG_APPn_XMP;
        free((*data_2).packet as *mut libc::c_void);
        free(data_2 as *mut libc::c_void);
    };
}
unsafe extern "C" fn JPEG_info_clear(mut j_info: *mut JPEG_info) {
    if (*j_info).num_appn > 0i32 && !(*j_info).appn.is_null() {
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < (*j_info).num_appn {
            JPEG_release_APPn_data(
                (*(*j_info).appn.offset(i as isize)).marker,
                (*(*j_info).appn.offset(i as isize)).app_sig,
                (*(*j_info).appn.offset(i as isize)).app_data,
            );
            i += 1
        }
        free((*j_info).appn as *mut libc::c_void);
    }
    (*j_info).appn = 0 as *mut JPEG_ext;
    (*j_info).num_appn = 0i32;
    (*j_info).max_appn = 0i32;
    (*j_info).flags = 0i32;
}
unsafe extern "C" fn JPEG_get_iccp(mut j_info: *mut JPEG_info) -> *mut pdf_obj {
    let mut icc_stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut icc: *mut JPEG_APPn_ICC = 0 as *mut JPEG_APPn_ICC;
    let mut i: libc::c_int = 0;
    let mut prev_id: libc::c_int = 0i32;
    let mut num_icc_seg: libc::c_int = -1i32;
    icc_stream = pdf_new_stream(1i32 << 0i32);
    i = 0i32;
    while i < (*j_info).num_appn {
        if !((*(*j_info).appn.offset(i as isize)).marker as libc::c_uint
            != JM_APP2 as libc::c_int as libc::c_uint
            || (*(*j_info).appn.offset(i as isize)).app_sig as libc::c_uint
                != JS_APPn_ICC as libc::c_int as libc::c_uint)
        {
            icc = (*(*j_info).appn.offset(i as isize)).app_data as *mut JPEG_APPn_ICC;
            if num_icc_seg < 0i32 && prev_id == 0i32 {
                num_icc_seg = (*icc).num_chunks as libc::c_int
            /* ICC chunks are sorted? */
            } else if (*icc).seq_id as libc::c_int != prev_id + 1i32
                || num_icc_seg != (*icc).num_chunks as libc::c_int
                || (*icc).seq_id as libc::c_int > (*icc).num_chunks as libc::c_int
            {
                dpx_warning(
                    b"Invalid JPEG ICC chunk: %d (p:%d, n:%d)\x00" as *const u8
                        as *const libc::c_char,
                    (*icc).seq_id as libc::c_int,
                    prev_id,
                    (*icc).num_chunks as libc::c_int,
                );
                pdf_release_obj(icc_stream);
                icc_stream = 0 as *mut pdf_obj;
                break;
            }
            pdf_add_stream(
                icc_stream,
                (*icc).chunk as *const libc::c_void,
                (*icc).length as libc::c_int,
            );
            prev_id = (*icc).seq_id as libc::c_int;
            num_icc_seg = (*icc).num_chunks as libc::c_int
        }
        i += 1
    }
    return icc_stream;
}
unsafe extern "C" fn JPEG_get_XMP(mut j_info: *mut JPEG_info) -> *mut pdf_obj {
    let mut XMP_stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut XMP: *mut JPEG_APPn_XMP = 0 as *mut JPEG_APPn_XMP;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0i32;
    /* I don't know if XMP Metadata should be compressed here.*/
    XMP_stream = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(XMP_stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"Metadata\x00" as *const u8 as *const libc::c_char),
    );
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const libc::c_char),
        pdf_new_name(b"XML\x00" as *const u8 as *const libc::c_char),
    );
    i = 0i32;
    while i < (*j_info).num_appn {
        /* Not sure for the case of multiple segments */
        if !((*(*j_info).appn.offset(i as isize)).marker as libc::c_uint
            != JM_APP1 as libc::c_int as libc::c_uint
            || (*(*j_info).appn.offset(i as isize)).app_sig as libc::c_uint
                != JS_APPn_XMP as libc::c_int as libc::c_uint)
        {
            XMP = (*(*j_info).appn.offset(i as isize)).app_data as *mut JPEG_APPn_XMP;
            pdf_add_stream(
                XMP_stream,
                (*XMP).packet as *const libc::c_void,
                (*XMP).length as libc::c_int,
            );
            count += 1
        }
        i += 1
    }
    if count > 1i32 {
        dpx_warning(
            b"%s: Multiple XMP segments found in JPEG file. (untested)\x00" as *const u8
                as *const libc::c_char,
            b"JPEG\x00" as *const u8 as *const libc::c_char,
        );
    }
    return XMP_stream;
}
unsafe extern "C" fn JPEG_get_marker(mut handle: rust_input_handle_t) -> JPEG_marker {
    let mut c: libc::c_int = 0;
    c = ttstub_input_getc(handle);
    if c != 255i32 {
        return 4294967295 as JPEG_marker;
    }
    loop {
        c = ttstub_input_getc(handle);
        if c < 0i32 {
            return 4294967295 as JPEG_marker;
        } else {
            if c > 0i32 && c < 255i32 {
                return c as JPEG_marker;
            }
        }
    }
}
unsafe extern "C" fn add_APPn_marker(
    mut j_info: *mut JPEG_info,
    mut marker: JPEG_marker,
    mut app_sig: libc::c_int,
    mut app_data: *mut libc::c_void,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if (*j_info).num_appn >= (*j_info).max_appn {
        (*j_info).max_appn += 16i32;
        (*j_info).appn = renew(
            (*j_info).appn as *mut libc::c_void,
            ((*j_info).max_appn as uint32_t as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<JPEG_ext>() as libc::c_ulong)
                as uint32_t,
        ) as *mut JPEG_ext
    }
    n = (*j_info).num_appn;
    (*(*j_info).appn.offset(n as isize)).marker = marker;
    (*(*j_info).appn.offset(n as isize)).app_sig = app_sig as JPEG_APPn_sig;
    let ref mut fresh0 = (*(*j_info).appn.offset(n as isize)).app_data;
    *fresh0 = app_data;
    (*j_info).num_appn += 1i32;
    return n;
}
unsafe extern "C" fn read_APP14_Adobe(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
) -> libc::c_ushort {
    let mut app_data: *mut JPEG_APPn_Adobe = 0 as *mut JPEG_APPn_Adobe;
    app_data = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<JPEG_APPn_Adobe>() as libc::c_ulong)
        as uint32_t) as *mut JPEG_APPn_Adobe;
    (*app_data).version = tt_get_unsigned_pair(handle);
    (*app_data).flag0 = tt_get_unsigned_pair(handle);
    (*app_data).flag1 = tt_get_unsigned_pair(handle);
    (*app_data).transform = tt_get_unsigned_byte(handle);
    add_APPn_marker(
        j_info,
        JM_APP14,
        JS_APPn_ADOBE as libc::c_int,
        app_data as *mut libc::c_void,
    );
    return 7i32 as libc::c_ushort;
}
unsafe extern "C" fn read_exif_bytes(
    mut pp: *mut *mut libc::c_uchar,
    mut n: libc::c_int,
    mut endian: libc::c_int,
) -> libc::c_int {
    let mut rval: libc::c_int = 0i32;
    let mut p: *mut libc::c_uchar = *pp;
    let mut i: libc::c_int = 0;
    match endian {
        0 => {
            i = 0i32;
            while i < n {
                rval = (rval << 8i32) + *p.offset(i as isize) as libc::c_int;
                i += 1
            }
        }
        1 => {
            i = n - 1i32;
            while i >= 0i32 {
                rval = (rval << 8i32) + *p.offset(i as isize) as libc::c_int;
                i -= 1
            }
        }
        _ => {}
    }
    *pp = (*pp).offset(n as isize);
    return rval;
}
unsafe extern "C" fn read_APP1_Exif(
    mut info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
    mut length: size_t,
) -> size_t {
    let mut current_block: u64;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut endptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tiff_header: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bigendian: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut num_fields: libc::c_int = 0;
    let mut tag: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut num: libc::c_int = 0i32;
    let mut den: libc::c_int = 0i32;
    let mut xres: libc::c_double = 0.0f64;
    let mut yres: libc::c_double = 0.0f64;
    let mut res_unit: libc::c_double = 1.0f64;
    let mut xres_ms: libc::c_uint = 0i32 as libc::c_uint;
    let mut yres_ms: libc::c_uint = 0i32 as libc::c_uint;
    let mut res_unit_ms: libc::c_double = 0.0f64;
    let mut exifxdpi: libc::c_double = 0.0f64;
    let mut exifydpi: libc::c_double = 0.0f64;
    let mut r: ssize_t = 0;
    buffer = xmalloc(length) as *mut libc::c_uchar;
    r = ttstub_input_read(handle, buffer as *mut libc::c_char, length);
    if !(r < 0i32 as libc::c_long || r as size_t != length) {
        p = buffer;
        endptr = buffer.offset(length as isize);
        while p < buffer.offset(length as isize) && *p as libc::c_int == 0i32 {
            p = p.offset(1)
        }
        if !(p.offset(8) >= endptr) {
            tiff_header = p;
            if *p as libc::c_int == 'M' as i32 && *p.offset(1) as libc::c_int == 'M' as i32 {
                bigendian = 0i32 as libc::c_char;
                current_block = 1109700713171191020;
            } else if *p as libc::c_int == 'I' as i32 && *p.offset(1) as libc::c_int == 'I' as i32 {
                bigendian = 1i32 as libc::c_char;
                current_block = 1109700713171191020;
            } else {
                dpx_warning(
                    b"JPEG: Invalid value in Exif TIFF header.\x00" as *const u8
                        as *const libc::c_char,
                );
                current_block = 10568945602212496329;
            }
            match current_block {
                10568945602212496329 => {}
                _ => {
                    p = p.offset(2);
                    i = read_exif_bytes(&mut p, 2i32, bigendian as libc::c_int);
                    if i != 42i32 {
                        dpx_warning(
                            b"JPEG: Invalid value in Exif TIFF header.\x00" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        i = read_exif_bytes(&mut p, 4i32, bigendian as libc::c_int);
                        p = tiff_header.offset(i as isize);
                        num_fields = read_exif_bytes(&mut p, 2i32, bigendian as libc::c_int);
                        loop {
                            let fresh1 = num_fields;
                            num_fields = num_fields - 1;
                            if !(fresh1 > 0i32) {
                                current_block = 576355610076403033;
                                break;
                            }
                            let mut count: libc::c_int = 0;
                            tag = read_exif_bytes(&mut p, 2i32, bigendian as libc::c_int);
                            type_0 = read_exif_bytes(&mut p, 2i32, bigendian as libc::c_int);
                            count = read_exif_bytes(&mut p, 4i32, bigendian as libc::c_int);
                            match type_0 {
                                1 => {
                                    let fresh2 = p;
                                    p = p.offset(1);
                                    value = *fresh2 as libc::c_int;
                                    p = p.offset(3)
                                }
                                3 => {
                                    value = read_exif_bytes(&mut p, 2i32, bigendian as libc::c_int);
                                    p = p.offset(2)
                                }
                                4 | 9 => {
                                    value = read_exif_bytes(&mut p, 4i32, bigendian as libc::c_int)
                                }
                                5 | 10 => {
                                    value = read_exif_bytes(&mut p, 4i32, bigendian as libc::c_int);
                                    rp = tiff_header.offset(value as isize);
                                    num = read_exif_bytes(&mut rp, 4i32, bigendian as libc::c_int);
                                    den = read_exif_bytes(&mut rp, 4i32, bigendian as libc::c_int)
                                }
                                7 => {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    value = *fresh3 as libc::c_int;
                                    p = p.offset(3)
                                }
                                2 | _ => {
                                    value = 0i32;
                                    p = p.offset(4)
                                }
                            }
                            match tag {
                                282 => {
                                    if den != 0i32 {
                                        xres = (num / den) as libc::c_double
                                    }
                                    continue;
                                }
                                283 => {
                                    if den != 0i32 {
                                        yres = (num / den) as libc::c_double
                                    }
                                    continue;
                                }
                                296 => {
                                    match value {
                                        2 => {
                                            /* inch */
                                            res_unit = 1.0f64
                                        }
                                        3 => {
                                            /* cm */
                                            res_unit = 2.54f64
                                        }
                                        _ => {}
                                    }
                                }
                                20752 => {}
                                20753 => {
                                    /* PixelPerUnitX */
                                    if type_0 != 4i32 || count != 1i32 {
                                        dpx_warning(
                                            b"%s: Invalid data for PixelPerUnitX in Exif chunk.\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                            b"JPEG\x00" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 10568945602212496329;
                                        break;
                                    } else {
                                        value =
                                            read_exif_bytes(&mut p, 4i32, bigendian as libc::c_int);
                                        xres_ms = value as libc::c_uint;
                                        continue;
                                    }
                                }
                                20754 => {
                                    /* PixelPerUnitY */
                                    if type_0 != 4i32 || count != 1i32 {
                                        dpx_warning(
                                            b"%s: Invalid data for PixelPerUnitY in Exif chunk.\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                            b"JPEG\x00" as *const u8 as *const libc::c_char,
                                        );
                                        current_block = 10568945602212496329;
                                        break;
                                    } else {
                                        value =
                                            read_exif_bytes(&mut p, 4i32, bigendian as libc::c_int);
                                        yres_ms = value as libc::c_uint;
                                        continue;
                                    }
                                }
                                _ => {
                                    continue;
                                }
                            }
                            /* PixelUnit */
                            if type_0 != 1i32 || count != 1i32 {
                                dpx_warning(
                                    b"%s: Invalid data for ResolutionUnit in Exif chunk.\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                    b"JPEG\x00" as *const u8 as *const libc::c_char,
                                ); /* Unit is meter */
                                current_block = 10568945602212496329;
                                break;
                            } else {
                                value = read_exif_bytes(&mut p, 1i32, bigendian as libc::c_int);
                                p = p.offset(3);
                                if value == 1i32 {
                                    res_unit_ms = 0.0254f64
                                } else {
                                    res_unit_ms = 0.0f64
                                }
                            }
                        }
                        match current_block {
                            10568945602212496329 => {}
                            _ => {
                                /* Calculate Exif resolution, if given. */
                                if xres > 0.0f64 && yres > 0.0f64 {
                                    exifxdpi = xres * res_unit;
                                    exifydpi = yres * res_unit
                                } else if xres_ms > 0i32 as libc::c_uint
                                    && yres_ms > 0i32 as libc::c_uint
                                    && res_unit_ms > 0.0f64
                                {
                                    exifxdpi = xres_ms as libc::c_double * res_unit_ms;
                                    exifydpi = yres_ms as libc::c_double * res_unit_ms
                                } else {
                                    exifxdpi = 72.0f64 * res_unit;
                                    exifydpi = 72.0f64 * res_unit
                                }
                                /* Do not overwrite if already specified in JFIF */
                                if (*info).xdpi < 0.1f64 && (*info).ydpi < 0.1f64 {
                                    (*info).xdpi = exifxdpi;
                                    (*info).ydpi = exifydpi
                                } else {
                                    let mut xxx1: libc::c_double = floor(exifxdpi + 0.5f64);
                                    let mut xxx2: libc::c_double = floor((*info).xdpi + 0.5f64);
                                    let mut yyy1: libc::c_double = floor(exifydpi + 0.5f64);
                                    let mut yyy2: libc::c_double = floor((*info).ydpi + 0.5f64);
                                    if xxx1 != xxx2 || yyy1 != yyy2 {
                                        dpx_warning(b"JPEG: Inconsistent resolution may have been specified in Exif and JFIF: %gx%g - %gx%g\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    xres * res_unit,
                                                    yres * res_unit,
                                                    (*info).xdpi,
                                                    (*info).ydpi);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(buffer as *mut libc::c_void);
    return length;
}
unsafe extern "C" fn read_APP0_JFIF(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
) -> size_t {
    let mut app_data: *mut JPEG_APPn_JFIF = 0 as *mut JPEG_APPn_JFIF;
    let mut thumb_data_len: size_t = 0;
    app_data = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<JPEG_APPn_JFIF>() as libc::c_ulong)
        as uint32_t) as *mut JPEG_APPn_JFIF;
    (*app_data).version = tt_get_unsigned_pair(handle);
    (*app_data).units = tt_get_unsigned_byte(handle);
    (*app_data).Xdensity = tt_get_unsigned_pair(handle);
    (*app_data).Ydensity = tt_get_unsigned_pair(handle);
    (*app_data).Xthumbnail = tt_get_unsigned_byte(handle);
    (*app_data).Ythumbnail = tt_get_unsigned_byte(handle);
    thumb_data_len = (3i32
        * (*app_data).Xthumbnail as libc::c_int
        * (*app_data).Ythumbnail as libc::c_int) as size_t;
    if thumb_data_len > 0i32 as libc::c_ulong {
        (*app_data).thumbnail = new((thumb_data_len as uint32_t as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as uint32_t) as *mut libc::c_uchar;
        ttstub_input_read(
            handle,
            (*app_data).thumbnail as *mut libc::c_char,
            thumb_data_len,
        );
    } else {
        (*app_data).thumbnail = 0 as *mut libc::c_uchar
    }
    add_APPn_marker(
        j_info,
        JM_APP0,
        JS_APPn_JFIF as libc::c_int,
        app_data as *mut libc::c_void,
    );
    match (*app_data).units as libc::c_int {
        1 => {
            (*j_info).xdpi = (*app_data).Xdensity as libc::c_double;
            (*j_info).ydpi = (*app_data).Ydensity as libc::c_double
        }
        2 => {
            /* density is in pixels per cm */
            (*j_info).xdpi = (*app_data).Xdensity as libc::c_int as libc::c_double * 2.54f64;
            (*j_info).ydpi = (*app_data).Ydensity as libc::c_int as libc::c_double * 2.54f64
        }
        _ => {
            /* FIXME: not sure what to do with this.... */
            (*j_info).xdpi = 72.0f64;
            (*j_info).ydpi = 72.0f64
        }
    }
    return (9i32 as libc::c_ulong).wrapping_add(thumb_data_len);
}
unsafe extern "C" fn read_APP0_JFXX(mut handle: rust_input_handle_t, mut length: size_t) -> size_t {
    tt_get_unsigned_byte(handle);
    /* Extension Code:
     *
     * 0x10: Thumbnail coded using JPEG
     * 0x11: Thumbnail stored using 1 byte/pixel
     * 0x13: Thumbnail stored using 3 bytes/pixel
     */
    ttstub_input_seek(
        handle,
        length.wrapping_sub(1i32 as libc::c_ulong) as ssize_t,
        1i32,
    ); /* Thunbnail image */
    /* Ignore */
    return length; /* Starting at 1 */
}
unsafe extern "C" fn read_APP1_XMP(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
    mut length: size_t,
) -> size_t {
    let mut app_data: *mut JPEG_APPn_XMP = 0 as *mut JPEG_APPn_XMP;
    app_data = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<JPEG_APPn_XMP>() as libc::c_ulong)
        as uint32_t) as *mut JPEG_APPn_XMP;
    (*app_data).length = length;
    (*app_data).packet = new(((*app_data).length as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_uchar;
    ttstub_input_read(
        handle,
        (*app_data).packet as *mut libc::c_char,
        (*app_data).length,
    );
    add_APPn_marker(
        j_info,
        JM_APP1,
        JS_APPn_XMP as libc::c_int,
        app_data as *mut libc::c_void,
    );
    return length;
}
unsafe extern "C" fn read_APP2_ICC(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
    mut length: size_t,
) -> size_t {
    let mut app_data: *mut JPEG_APPn_ICC = 0 as *mut JPEG_APPn_ICC;
    app_data = new((1i32 as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<JPEG_APPn_ICC>() as libc::c_ulong)
        as uint32_t) as *mut JPEG_APPn_ICC;
    (*app_data).seq_id = tt_get_unsigned_byte(handle);
    (*app_data).num_chunks = tt_get_unsigned_byte(handle);
    (*app_data).length = length.wrapping_sub(2i32 as libc::c_ulong);
    (*app_data).chunk = new(((*app_data).length as uint32_t as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
        as uint32_t) as *mut libc::c_uchar;
    ttstub_input_read(
        handle,
        (*app_data).chunk as *mut libc::c_char,
        (*app_data).length,
    );
    add_APPn_marker(
        j_info,
        JM_APP2,
        JS_APPn_ICC as libc::c_int,
        app_data as *mut libc::c_void,
    );
    return length;
}
unsafe extern "C" fn JPEG_copy_stream(
    mut j_info: *mut JPEG_info,
    mut stream: *mut pdf_obj,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut marker: JPEG_marker = 0 as JPEG_marker;
    let mut length: libc::c_int = 0;
    let mut found_SOFn: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    count = 0i32;
    found_SOFn = 0i32;
    while found_SOFn == 0 && count < 1024i32 && {
        marker = JPEG_get_marker(handle);
        marker as libc::c_uint != 4294967295 as JPEG_marker as libc::c_uint
    } {
        if marker as libc::c_uint == JM_SOI as libc::c_int as libc::c_uint
            || marker as libc::c_uint >= JM_RST0 as libc::c_int as libc::c_uint
                && marker as libc::c_uint <= JM_RST7 as libc::c_int as libc::c_uint
        {
            *work_buffer.as_mut_ptr().offset(0) = 0xffi32 as libc::c_char;
            *work_buffer.as_mut_ptr().offset(1) = marker as libc::c_char;
            pdf_add_stream(
                stream,
                work_buffer.as_mut_ptr() as *const libc::c_void,
                2i32,
            );
        } else {
            length = tt_get_unsigned_pair(handle) as libc::c_int - 2i32;
            match marker as libc::c_uint {
                192 | 193 | 194 | 195 | 197 | 198 | 199 | 201 | 202 | 203 | 205 | 206 | 207 => {
                    *work_buffer.as_mut_ptr().offset(0) = 0xffi32 as libc::c_char;
                    *work_buffer.as_mut_ptr().offset(1) = marker as libc::c_char;
                    *work_buffer.as_mut_ptr().offset(2) =
                        (length + 2i32 >> 8i32 & 0xffi32) as libc::c_char;
                    *work_buffer.as_mut_ptr().offset(3) = (length + 2i32 & 0xffi32) as libc::c_char;
                    pdf_add_stream(
                        stream,
                        work_buffer.as_mut_ptr() as *const libc::c_void,
                        4i32,
                    );
                    while length > 0i32 {
                        let mut nb_read: libc::c_int = ttstub_input_read(
                            handle,
                            work_buffer.as_mut_ptr(),
                            (if length < 1024i32 { length } else { 1024i32 }) as size_t,
                        ) as libc::c_int;
                        if nb_read > 0i32 {
                            pdf_add_stream(
                                stream,
                                work_buffer.as_mut_ptr() as *const libc::c_void,
                                nb_read,
                            );
                        }
                        length -= nb_read
                    }
                    found_SOFn = 1i32
                }
                _ => {
                    if (*j_info).skipbits[(count / 8i32) as usize] as libc::c_int
                        & 1i32 << 7i32 - count % 8i32
                        != 0
                    {
                        ttstub_input_seek(handle, length as ssize_t, 1i32);
                    } else {
                        *work_buffer.as_mut_ptr().offset(0) = 0xffi32 as libc::c_char;
                        *work_buffer.as_mut_ptr().offset(1) = marker as libc::c_char;
                        *work_buffer.as_mut_ptr().offset(2) =
                            (length + 2i32 >> 8i32 & 0xffi32) as libc::c_char;
                        *work_buffer.as_mut_ptr().offset(3) =
                            (length + 2i32 & 0xffi32) as libc::c_char;
                        pdf_add_stream(
                            stream,
                            work_buffer.as_mut_ptr() as *const libc::c_void,
                            4i32,
                        );
                        while length > 0i32 {
                            let mut nb_read_0: libc::c_int = ttstub_input_read(
                                handle,
                                work_buffer.as_mut_ptr(),
                                (if length < 1024i32 { length } else { 1024i32 }) as size_t,
                            )
                                as libc::c_int;
                            if nb_read_0 > 0i32 {
                                pdf_add_stream(
                                    stream,
                                    work_buffer.as_mut_ptr() as *const libc::c_void,
                                    nb_read_0,
                                );
                            }
                            length -= nb_read_0
                        }
                    }
                }
            }
        }
        count += 1
    }
    let mut total_size: size_t = ttstub_input_get_size(handle);
    let mut pos: size_t = ttstub_input_seek(handle, 0i32 as ssize_t, 1i32);
    loop {
        length = ttstub_input_read(
            handle,
            work_buffer.as_mut_ptr(),
            (if (1024i32 as libc::c_ulong) < total_size.wrapping_sub(pos) {
                1024i32 as libc::c_ulong
            } else {
                total_size.wrapping_sub(pos)
            }),
        ) as libc::c_int;
        if !(length > 0i32) {
            break;
        }
        pdf_add_stream(
            stream,
            work_buffer.as_mut_ptr() as *const libc::c_void,
            length,
        );
        pos = (pos as libc::c_ulong).wrapping_add(length as libc::c_ulong) as size_t as size_t
    }
    return if found_SOFn != 0 { 0i32 } else { -1i32 };
}
unsafe extern "C" fn JPEG_scan_file(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut marker: JPEG_marker = 0 as JPEG_marker;
    let mut found_SOFn: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut app_sig: [libc::c_char; 128] = [0; 128];
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    count = 0i32;
    found_SOFn = 0i32;
    while found_SOFn == 0 && {
        marker = JPEG_get_marker(handle);
        marker as libc::c_uint != 4294967295 as JPEG_marker as libc::c_uint
    } {
        if marker as libc::c_uint != JM_SOI as libc::c_int as libc::c_uint
            && ((marker as libc::c_uint) < JM_RST0 as libc::c_int as libc::c_uint
                || marker as libc::c_uint > JM_RST7 as libc::c_int as libc::c_uint)
        {
            let mut length: libc::c_int = tt_get_unsigned_pair(handle) as libc::c_int - 2i32;
            match marker as libc::c_uint {
                192 | 193 | 194 | 195 | 197 | 198 | 199 | 201 | 202 | 203 | 205 | 206 | 207 => {
                    (*j_info).bits_per_component = tt_get_unsigned_byte(handle);
                    (*j_info).height = tt_get_unsigned_pair(handle);
                    (*j_info).width = tt_get_unsigned_pair(handle);
                    (*j_info).num_components = tt_get_unsigned_byte(handle);
                    found_SOFn = 1i32
                }
                224 => {
                    if length > 5i32 {
                        if ttstub_input_read(handle, app_sig.as_mut_ptr(), 5i32 as size_t)
                            != 5i32 as libc::c_long
                        {
                            return -1i32;
                        }
                        length -= 5i32;
                        if memcmp(
                            app_sig.as_mut_ptr() as *const libc::c_void,
                            b"JFIF\x00\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            5i32 as libc::c_ulong,
                        ) == 0
                        {
                            (*j_info).flags |= 1i32 << 0i32;
                            length = (length as libc::c_ulong)
                                .wrapping_sub(read_APP0_JFIF(j_info, handle))
                                as libc::c_int as libc::c_int
                        } else if memcmp(
                            app_sig.as_mut_ptr() as *const libc::c_void,
                            b"JFXX\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            5i32 as libc::c_ulong,
                        ) == 0
                        {
                            length = (length as libc::c_ulong)
                                .wrapping_sub(read_APP0_JFXX(handle, length as size_t))
                                as libc::c_int as libc::c_int
                        }
                    }
                    ttstub_input_seek(handle, length as ssize_t, 1i32);
                }
                225 => {
                    if length > 5i32 {
                        if ttstub_input_read(handle, app_sig.as_mut_ptr(), 5i32 as size_t)
                            != 5i32 as libc::c_long
                        {
                            return -1i32;
                        }
                        length -= 5i32;
                        if memcmp(
                            app_sig.as_mut_ptr() as *const libc::c_void,
                            b"Exif\x00\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            5i32 as libc::c_ulong,
                        ) == 0
                        {
                            (*j_info).flags |= 1i32 << 3i32;
                            length = (length as libc::c_ulong).wrapping_sub(read_APP1_Exif(
                                j_info,
                                handle,
                                length as size_t,
                            )) as libc::c_int as libc::c_int
                        } else if memcmp(
                            app_sig.as_mut_ptr() as *const libc::c_void,
                            b"http:\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            5i32 as libc::c_ulong,
                        ) == 0
                            && length > 24i32
                        {
                            if ttstub_input_read(handle, app_sig.as_mut_ptr(), 24i32 as size_t)
                                != 24i32 as libc::c_long
                            {
                                return -1i32;
                            }
                            length -= 24i32;
                            if memcmp(
                                app_sig.as_mut_ptr() as *const libc::c_void,
                                b"//ns.adobe.com/xap/1.0/\x00\x00" as *const u8
                                    as *const libc::c_char
                                    as *const libc::c_void,
                                24i32 as libc::c_ulong,
                            ) == 0
                            {
                                (*j_info).flags |= 1i32 << 4i32;
                                length = (length as libc::c_ulong).wrapping_sub(read_APP1_XMP(
                                    j_info,
                                    handle,
                                    length as size_t,
                                )) as libc::c_int
                                    as libc::c_int;
                                if count < 1024i32 {
                                    (*j_info).skipbits[(count / 8i32) as usize] =
                                        ((*j_info).skipbits[(count / 8i32) as usize] as libc::c_int
                                            | 1i32 << 7i32 - count % 8i32)
                                            as libc::c_char
                                }
                            }
                        }
                    }
                    ttstub_input_seek(handle, length as ssize_t, 1i32);
                }
                226 => {
                    if length >= 14i32 {
                        if ttstub_input_read(handle, app_sig.as_mut_ptr(), 12i32 as size_t)
                            != 12i32 as libc::c_long
                        {
                            return -1i32;
                        }
                        length -= 12i32;
                        if memcmp(
                            app_sig.as_mut_ptr() as *const libc::c_void,
                            b"ICC_PROFILE\x00\x00" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            12i32 as libc::c_ulong,
                        ) == 0
                        {
                            (*j_info).flags |= 1i32 << 2i32;
                            length = (length as libc::c_ulong).wrapping_sub(read_APP2_ICC(
                                j_info,
                                handle,
                                length as size_t,
                            )) as libc::c_int as libc::c_int;
                            if count < 1024i32 {
                                (*j_info).skipbits[(count / 8i32) as usize] =
                                    ((*j_info).skipbits[(count / 8i32) as usize] as libc::c_int
                                        | 1i32 << 7i32 - count % 8i32)
                                        as libc::c_char
                            }
                        }
                    }
                    ttstub_input_seek(handle, length as ssize_t, 1i32);
                }
                238 => {
                    if length > 5i32 {
                        if ttstub_input_read(handle, app_sig.as_mut_ptr(), 5i32 as size_t)
                            != 5i32 as libc::c_long
                        {
                            return -1i32;
                        }
                        length -= 5i32;
                        if memcmp(
                            app_sig.as_mut_ptr() as *const libc::c_void,
                            b"Adobe\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                            5i32 as libc::c_ulong,
                        ) == 0
                        {
                            (*j_info).flags |= 1i32 << 1i32;
                            length -= read_APP14_Adobe(j_info, handle) as libc::c_int
                        } else if count < 1024i32 {
                            (*j_info).skipbits[(count / 8i32) as usize] =
                                ((*j_info).skipbits[(count / 8i32) as usize] as libc::c_int
                                    | 1i32 << 7i32 - count % 8i32)
                                    as libc::c_char
                        }
                    }
                    ttstub_input_seek(handle, length as ssize_t, 1i32);
                }
                _ => {
                    ttstub_input_seek(handle, length as ssize_t, 1i32);
                    if marker as libc::c_uint >= JM_APP0 as libc::c_int as libc::c_uint
                        && marker as libc::c_uint <= JM_APP15 as libc::c_int as libc::c_uint
                    {
                        if count < 1024i32 {
                            (*j_info).skipbits[(count / 8i32) as usize] =
                                ((*j_info).skipbits[(count / 8i32) as usize] as libc::c_int
                                    | 1i32 << 7i32 - count % 8i32)
                                    as libc::c_char
                        }
                    }
                }
            }
        }
        count += 1
    }
    /*
     * If j_info->xdpi, and j_info->ydpi are not yet determined,
     * they are assumed to be 72.0 to avoid division by zero.
     */
    if (*j_info).xdpi < 0.1f64 && (*j_info).ydpi < 0.1f64 {
        (*j_info).ydpi = 72.0f64;
        (*j_info).xdpi = (*j_info).ydpi
    }
    return if found_SOFn != 0 { 0i32 } else { -1i32 };
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
pub unsafe extern "C" fn jpeg_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut libc::c_uint,
    mut height: *mut libc::c_uint,
    mut xdensity: *mut libc::c_double,
    mut ydensity: *mut libc::c_double,
) -> libc::c_int {
    let mut j_info: JPEG_info = JPEG_info {
        height: 0,
        width: 0,
        bits_per_component: 0,
        num_components: 0,
        xdpi: 0.,
        ydpi: 0.,
        flags: 0,
        num_appn: 0,
        max_appn: 0,
        appn: 0 as *mut JPEG_ext,
        skipbits: [0; 129],
    };
    JPEG_info_init(&mut j_info);
    if JPEG_scan_file(&mut j_info, handle) < 0i32 {
        dpx_warning(
            b"%s: Not a JPEG file?\x00" as *const u8 as *const libc::c_char,
            b"JPEG\x00" as *const u8 as *const libc::c_char,
        );
        JPEG_info_clear(&mut j_info);
        return -1i32;
    }
    *width = j_info.width as libc::c_uint;
    *height = j_info.height as libc::c_uint;
    jpeg_get_density(&mut j_info, xdensity, ydensity);
    JPEG_info_clear(&mut j_info);
    return 0i32;
}
