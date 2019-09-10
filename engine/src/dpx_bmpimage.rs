#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    /* A deeper object hierarchy will be considered as (illegal) loop. */
    pub type pdf_obj;
    pub type pdf_ximage_;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut libc::c_char,
        len: size_t,
    ) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> libc::c_uchar;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const libc::c_char) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    /* pdf_add_dict requires key but pdf_add_array does not.
     * pdf_add_array always append elements to array.
     * They should be pdf_put_array(array, idx, element) and
     * pdf_put_dict(dict, key, value)
     */
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
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
    fn pdf_stream_set_predictor(
        stream: *mut pdf_obj,
        predictor: libc::c_int,
        columns: int32_t,
        bpc: libc::c_int,
        colors: libc::c_int,
    );
    #[no_mangle]
    fn pdf_ximage_init_image_info(info: *mut ximage_info);
    #[no_mangle]
    fn pdf_ximage_set_image(
        ximage: *mut pdf_ximage,
        info: *mut libc::c_void,
        resource: *mut pdf_obj,
    );
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
    fn new(size: u32) -> *mut libc::c_void;
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_info {
    pub offset: libc::c_uint,
    pub hsize: libc::c_uint,
    pub width: libc::c_uint,
    pub height: libc::c_int,
    pub compression: libc::c_int,
    pub bit_count: libc::c_ushort,
    pub psize: libc::c_int,
    pub x_pix_per_meter: libc::c_uint,
    pub y_pix_per_meter: libc::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn check_for_bmp(mut handle: rust_input_handle_t) -> libc::c_int {
    let mut sigbytes: [libc::c_uchar; 2] = [0; 2];
    if handle.is_null() {
        return 0i32;
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if ttstub_input_read(
        handle,
        sigbytes.as_mut_ptr() as *mut libc::c_char,
        ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong,
    ) as libc::c_ulong
        != ::std::mem::size_of::<[libc::c_uchar; 2]>() as libc::c_ulong
        || sigbytes[0] as libc::c_int != 'B' as i32
        || sigbytes[1] as libc::c_int != 'M' as i32
    {
        return 0i32;
    }
    return 1i32;
}
unsafe extern "C" fn get_density(
    mut xdensity: *mut libc::c_double,
    mut ydensity: *mut libc::c_double,
    mut hdr: *mut hdr_info,
) {
    if (*hdr).x_pix_per_meter > 0i32 as libc::c_uint
        && (*hdr).y_pix_per_meter > 0i32 as libc::c_uint
    {
        /* 0 for undefined. FIXME */
        *xdensity = 72.0f64 / ((*hdr).x_pix_per_meter as libc::c_double * 0.0254f64);
        *ydensity = 72.0f64 / ((*hdr).y_pix_per_meter as libc::c_double * 0.0254f64)
    } else {
        *xdensity = 1.0f64;
        *ydensity = 1.0f64
    };
}
#[no_mangle]
pub unsafe extern "C" fn bmp_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut libc::c_uint,
    mut height: *mut libc::c_uint,
    mut xdensity: *mut libc::c_double,
    mut ydensity: *mut libc::c_double,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut hdr: hdr_info = {
        let mut init = hdr_info {
            offset: 0i32 as libc::c_uint,
            hsize: 0i32 as libc::c_uint,
            width: 0i32 as libc::c_uint,
            height: 0i32,
            compression: 0i32,
            bit_count: 0i32 as libc::c_ushort,
            psize: 0i32,
            x_pix_per_meter: 0i32 as libc::c_uint,
            y_pix_per_meter: 0i32 as libc::c_uint,
        };
        init
    };
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    r = read_header(handle, &mut hdr);
    *width = hdr.width;
    *height = (if hdr.height < 0i32 {
        -hdr.height
    } else {
        hdr.height
    }) as libc::c_uint;
    get_density(xdensity, ydensity, &mut hdr);
    return r;
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
pub unsafe extern "C" fn bmp_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut colorspace: *mut pdf_obj = 0 as *mut pdf_obj;
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
    let mut hdr: hdr_info = {
        let mut init = hdr_info {
            offset: 0i32 as libc::c_uint,
            hsize: 0i32 as libc::c_uint,
            width: 0i32 as libc::c_uint,
            height: 0i32,
            compression: 0i32,
            bit_count: 0i32 as libc::c_ushort,
            psize: 0i32,
            x_pix_per_meter: 0i32 as libc::c_uint,
            y_pix_per_meter: 0i32 as libc::c_uint,
        };
        init
    };
    let mut num_palette: libc::c_int = 0;
    let mut flip: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    pdf_ximage_init_image_info(&mut info);
    colorspace = 0 as *mut pdf_obj;
    stream_dict = colorspace;
    stream = stream_dict;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if read_header(handle, &mut hdr) < 0i32 {
        return -1i32;
    }
    get_density(&mut info.xdensity, &mut info.ydensity, &mut hdr);
    info.width = hdr.width as libc::c_int;
    info.height = hdr.height;
    if info.height < 0i32 {
        info.height = -info.height;
        flip = 0i32
    } else {
        flip = 1i32
    }
    if (hdr.bit_count as libc::c_int) < 24i32 {
        if hdr.bit_count as libc::c_int != 1i32
            && hdr.bit_count as libc::c_int != 4i32
            && hdr.bit_count as libc::c_int != 8i32
        {
            dpx_warning(
                b"Unsupported palette size: %hu\x00" as *const u8 as *const libc::c_char,
                hdr.bit_count as libc::c_int,
            );
            return -1i32;
        }
        num_palette = hdr
            .offset
            .wrapping_sub(hdr.hsize)
            .wrapping_sub(14i32 as libc::c_uint)
            .wrapping_div(hdr.psize as libc::c_uint) as libc::c_int;
        info.bits_per_component = hdr.bit_count as libc::c_int;
        info.num_components = 1i32
    } else if hdr.bit_count as libc::c_int == 24i32 {
        /* full color */
        num_palette = 1i32; /* dummy */
        info.bits_per_component = 8i32;
        info.num_components = 3i32
    } else {
        dpx_warning(
            b"Unkown/Unsupported BMP bitCount value: %hu\x00" as *const u8 as *const libc::c_char,
            hdr.bit_count as libc::c_int,
        );
        return -1i32;
    }
    if info.width == 0i32 || info.height == 0i32 || num_palette < 1i32 {
        dpx_warning(
            b"Invalid BMP file: width=%u, height=%d, #palette=%d\x00" as *const u8
                as *const libc::c_char,
            info.width,
            info.height,
            num_palette,
        );
        return -1i32;
    }
    /* Start reading raster data */
    stream = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(stream);
    /* Color space: Indexed or DeviceRGB */
    if (hdr.bit_count as libc::c_int) < 24i32 {
        let mut lookup: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut palette: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut bgrq: [libc::c_uchar; 4] = [0; 4];
        palette = new(((num_palette * 3i32 + 1i32) as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as u32) as *mut libc::c_uchar;
        i = 0i32;
        while i < num_palette {
            if ttstub_input_read(
                handle,
                bgrq.as_mut_ptr() as *mut libc::c_char,
                hdr.psize as size_t,
            ) != hdr.psize as libc::c_long
            {
                dpx_warning(b"Reading file failed...\x00" as *const u8 as *const libc::c_char);
                free(palette as *mut libc::c_void);
                return -1i32;
            }
            /* BGR data */
            *palette.offset((3i32 * i) as isize) = bgrq[2];
            *palette.offset((3i32 * i + 1i32) as isize) = bgrq[1];
            *palette.offset((3i32 * i + 2i32) as isize) = bgrq[0];
            i += 1
        }
        lookup = pdf_new_string(
            palette as *const libc::c_void,
            (num_palette * 3i32) as size_t,
        );
        free(palette as *mut libc::c_void);
        colorspace = pdf_new_array();
        pdf_add_array(
            colorspace,
            pdf_new_name(b"Indexed\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_array(
            colorspace,
            pdf_new_name(b"DeviceRGB\x00" as *const u8 as *const libc::c_char),
        );
        pdf_add_array(
            colorspace,
            pdf_new_number((num_palette - 1i32) as libc::c_double),
        );
        pdf_add_array(colorspace, lookup);
    } else {
        colorspace = pdf_new_name(b"DeviceRGB\x00" as *const u8 as *const libc::c_char)
    }
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"ColorSpace\x00" as *const u8 as *const libc::c_char),
        colorspace,
    );
    /* Raster data of BMP is four-byte aligned. */
    let mut rowbytes: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut stream_data_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    rowbytes = (info.width * hdr.bit_count as libc::c_int + 7i32) / 8i32;
    ttstub_input_seek(handle, hdr.offset as ssize_t, 0i32);
    if hdr.compression == 0i32 {
        let mut dib_rowbytes: libc::c_int = 0;
        let mut padding: libc::c_int = 0;
        padding = if rowbytes % 4i32 != 0 {
            4i32 - rowbytes % 4i32
        } else {
            0i32
        };
        dib_rowbytes = rowbytes + padding;
        stream_data_ptr = new(
            ((rowbytes * info.height + padding) as u32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                as u32,
        ) as *mut libc::c_uchar;
        n = 0i32;
        while n < info.height {
            p = stream_data_ptr.offset((n * rowbytes) as isize);
            if ttstub_input_read(handle, p as *mut libc::c_char, dib_rowbytes as size_t)
                != dib_rowbytes as libc::c_long
            {
                dpx_warning(
                    b"Reading BMP raster data failed...\x00" as *const u8 as *const libc::c_char,
                );
                pdf_release_obj(stream);
                free(stream_data_ptr as *mut libc::c_void);
                return -1i32;
            }
            n += 1
        }
    } else if hdr.compression == 1i32 {
        stream_data_ptr = new(((rowbytes * info.height) as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as u32) as *mut libc::c_uchar;
        if read_raster_rle8(stream_data_ptr, info.width, info.height, handle) < 0i32 {
            dpx_warning(
                b"Reading BMP raster data failed...\x00" as *const u8 as *const libc::c_char,
            );
            pdf_release_obj(stream);
            free(stream_data_ptr as *mut libc::c_void);
            return -1i32;
        }
    } else if hdr.compression == 2i32 {
        stream_data_ptr = new(((rowbytes * info.height) as u32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
            as u32) as *mut libc::c_uchar;
        if read_raster_rle4(stream_data_ptr, info.width, info.height, handle) < 0i32 {
            dpx_warning(
                b"Reading BMP raster data failed...\x00" as *const u8 as *const libc::c_char,
            );
            pdf_release_obj(stream);
            free(stream_data_ptr as *mut libc::c_void);
            return -1i32;
        }
    } else {
        dpx_warning(
            b"Unknown/Unsupported compression type for BMP image: %d\x00" as *const u8
                as *const libc::c_char,
            hdr.compression,
        );
        pdf_release_obj(stream);
        return -1i32;
    }
    /* gbr --> rgb */
    if hdr.bit_count as libc::c_int == 24i32 {
        n = 0i32;
        while n < info.width * info.height * 3i32 {
            let mut g: libc::c_uchar = 0;
            g = *stream_data_ptr.offset(n as isize);
            *stream_data_ptr.offset(n as isize) = *stream_data_ptr.offset((n + 2i32) as isize);
            *stream_data_ptr.offset((n + 2i32) as isize) = g;
            n += 3i32
        }
    }
    if flip != 0 {
        n = info.height - 1i32;
        while n >= 0i32 {
            p = stream_data_ptr.offset((n * rowbytes) as isize);
            pdf_add_stream(stream, p as *const libc::c_void, rowbytes);
            n -= 1
        }
    } else {
        pdf_add_stream(
            stream,
            stream_data_ptr as *const libc::c_void,
            rowbytes * info.height,
        );
    }
    free(stream_data_ptr as *mut libc::c_void);
    /* Predictor is usually not so efficient for indexed images. */
    if hdr.bit_count as libc::c_int >= 24i32
        && info.bits_per_component >= 8i32
        && info.height > 64i32
    {
        pdf_stream_set_predictor(
            stream,
            15i32,
            info.width,
            info.bits_per_component,
            info.num_components,
        );
    }
    pdf_ximage_set_image(
        ximage,
        &mut info as *mut ximage_info as *mut libc::c_void,
        stream,
    );
    return 0i32;
}
unsafe extern "C" fn read_header(
    mut handle: rust_input_handle_t,
    mut hdr: *mut hdr_info,
) -> libc::c_int {
    let mut buf: [libc::c_uchar; 142] = [0; 142];
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = buf.as_mut_ptr();
    if ttstub_input_read(
        handle,
        buf.as_mut_ptr() as *mut libc::c_char,
        (14i32 + 4i32) as size_t,
    ) != (14i32 + 4i32) as libc::c_long
    {
        dpx_warning(b"Could not read BMP file header...\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    if *p.offset(0) as libc::c_int != 'B' as i32 || *p.offset(1) as libc::c_int != 'M' as i32 {
        dpx_warning(
            b"File not starting with \'B\' \'M\'... Not a BMP file?\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    p = p.offset(2);
    /* fsize  = ULONG_LE(p); */
    p = p.offset(4);
    if *p.offset(0) as libc::c_int
        + ((*p.offset(1) as libc::c_int) << 8i32)
        + ((*p.offset(2) as libc::c_int) << 16i32)
        + ((*p.offset(3) as libc::c_int) << 24i32)
        != 0i32
    {
        dpx_warning(b"Not a BMP file???\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    p = p.offset(4);
    (*hdr).offset = (*p.offset(0) as libc::c_int
        + ((*p.offset(1) as libc::c_int) << 8i32)
        + ((*p.offset(2) as libc::c_int) << 16i32)
        + ((*p.offset(3) as libc::c_int) << 24i32)) as libc::c_uint;
    p = p.offset(4);
    /* info header */
    (*hdr).hsize = (*p.offset(0) as libc::c_int
        + ((*p.offset(1) as libc::c_int) << 8i32)
        + ((*p.offset(2) as libc::c_int) << 16i32)
        + ((*p.offset(3) as libc::c_int) << 24i32)) as libc::c_uint; /* undefined. FIXME */
    p = p.offset(4); /* undefined. FIXME */
    if ttstub_input_read(
        handle,
        p as *mut libc::c_char,
        (*hdr).hsize.wrapping_sub(4i32 as libc::c_uint) as size_t,
    ) != (*hdr).hsize.wrapping_sub(4i32 as libc::c_uint) as libc::c_long
    {
        dpx_warning(b"Could not read BMP file header...\x00" as *const u8 as *const libc::c_char);
        return -1i32;
    }
    match (*hdr).hsize {
        12 => {
            (*hdr).width = (*p.offset(0) as libc::c_int + ((*p.offset(1) as libc::c_int) << 8i32))
                as libc::c_uint;
            p = p.offset(2);
            (*hdr).height = *p.offset(0) as libc::c_int + ((*p.offset(1) as libc::c_int) << 8i32);
            p = p.offset(2);
            (*hdr).x_pix_per_meter = 0i32 as libc::c_uint;
            (*hdr).y_pix_per_meter = 0i32 as libc::c_uint;
            if *p.offset(0) as libc::c_int + ((*p.offset(1) as libc::c_int) << 8i32) != 1i32 {
                dpx_warning(
                    b"Unknown bcPlanes value in BMP COREHEADER.\x00" as *const u8
                        as *const libc::c_char,
                );
                return -1i32;
            }
            p = p.offset(2);
            (*hdr).bit_count = (*p.offset(0) as libc::c_int
                + ((*p.offset(1) as libc::c_int) << 8i32))
                as libc::c_ushort;
            p = p.offset(2);
            (*hdr).compression = 0i32;
            (*hdr).psize = 3i32
        }
        40 | 64 | 108 | 124 => {
            (*hdr).width = (*p.offset(0) as libc::c_int
                + ((*p.offset(1) as libc::c_int) << 8i32)
                + ((*p.offset(2) as libc::c_int) << 16i32)
                + ((*p.offset(3) as libc::c_int) << 24i32))
                as libc::c_uint;
            p = p.offset(4);
            (*hdr).height = *p.offset(0) as libc::c_int
                + ((*p.offset(1) as libc::c_int) << 8i32)
                + ((*p.offset(2) as libc::c_int) << 16i32)
                + ((*p.offset(3) as libc::c_int) << 24i32);
            p = p.offset(4);
            if *p.offset(0) as libc::c_int + ((*p.offset(1) as libc::c_int) << 8i32) != 1i32 {
                dpx_warning(
                    b"Unknown biPlanes value in BMP INFOHEADER.\x00" as *const u8
                        as *const libc::c_char,
                );
                return -1i32;
            }
            p = p.offset(2);
            (*hdr).bit_count = (*p.offset(0) as libc::c_int
                + ((*p.offset(1) as libc::c_int) << 8i32))
                as libc::c_ushort;
            p = p.offset(2);
            (*hdr).compression = *p.offset(0) as libc::c_int
                + ((*p.offset(1) as libc::c_int) << 8i32)
                + ((*p.offset(2) as libc::c_int) << 16i32)
                + ((*p.offset(3) as libc::c_int) << 24i32);
            p = p.offset(4);
            /* ignore biSizeImage */
            p = p.offset(4);
            (*hdr).x_pix_per_meter = (*p.offset(0) as libc::c_int
                + ((*p.offset(1) as libc::c_int) << 8i32)
                + ((*p.offset(2) as libc::c_int) << 16i32)
                + ((*p.offset(3) as libc::c_int) << 24i32))
                as libc::c_uint;
            p = p.offset(4);
            (*hdr).y_pix_per_meter = (*p.offset(0) as libc::c_int
                + ((*p.offset(1) as libc::c_int) << 8i32)
                + ((*p.offset(2) as libc::c_int) << 16i32)
                + ((*p.offset(3) as libc::c_int) << 24i32))
                as libc::c_uint;
            p = p.offset(4);
            (*hdr).psize = 4i32
        }
        _ => {
            dpx_warning(b"Unknown BMP header type.\x00" as *const u8 as *const libc::c_char);
            return -1i32;
        }
    }
    return 0i32;
}
unsafe extern "C" fn read_raster_rle8(
    mut data_ptr: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut b0: libc::c_uchar = 0;
    let mut b1: libc::c_uchar = 0;
    let mut h: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut rowbytes: libc::c_int = 0;
    let mut eol: libc::c_int = 0;
    let mut eoi: libc::c_int = 0;
    p = data_ptr;
    rowbytes = width;
    memset(
        data_ptr as *mut libc::c_void,
        0i32,
        (rowbytes * height) as libc::c_ulong,
    );
    v = 0i32;
    eoi = 0i32;
    while v < height && eoi == 0 {
        h = 0i32;
        eol = 0i32;
        while h < width && eol == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            count += 2i32;
            p = data_ptr.offset((v * rowbytes) as isize).offset(h as isize);
            if b0 as libc::c_int == 0i32 {
                match b1 as libc::c_int {
                    0 => {
                        /* EOL */
                        eol = 1i32
                    }
                    1 => {
                        /* EOI */
                        eoi = 1i32
                    }
                    2 => {
                        h += tt_get_unsigned_byte(handle) as libc::c_int;
                        v += tt_get_unsigned_byte(handle) as libc::c_int;
                        count += 2i32
                    }
                    _ => {
                        h += b1 as libc::c_int;
                        if h > width {
                            dpx_warning(
                                b"RLE decode failed...\x00" as *const u8 as *const libc::c_char,
                            );
                            return -1i32;
                        }
                        if ttstub_input_read(handle, p as *mut libc::c_char, b1 as size_t)
                            != b1 as libc::c_long
                        {
                            return -1i32;
                        }
                        count += b1 as libc::c_int;
                        if b1 as libc::c_int % 2i32 != 0 {
                            tt_get_unsigned_byte(handle);
                            count += 1
                        }
                    }
                }
            } else {
                h += b0 as libc::c_int;
                if h > width {
                    dpx_warning(b"RLE decode failed...\x00" as *const u8 as *const libc::c_char);
                    return -1i32;
                }
                memset(
                    p as *mut libc::c_void,
                    b1 as libc::c_int,
                    b0 as libc::c_ulong,
                );
            }
        }
        /* next row ... */
        if eol == 0 && eoi == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            if b0 as libc::c_int != 0i32 {
                dpx_warning(b"RLE decode failed...\x00" as *const u8 as *const libc::c_char);
                return -1i32;
            } else {
                if b1 as libc::c_int == 0x1i32 {
                    eoi = 1i32
                } else if b1 as libc::c_int != 0i32 {
                    dpx_warning(b"RLE decode failed...\x00" as *const u8 as *const libc::c_char);
                    return -1i32;
                }
            }
        }
        v += 1
    }
    return count;
}
unsafe extern "C" fn read_raster_rle4(
    mut data_ptr: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut b0: libc::c_uchar = 0;
    let mut b1: libc::c_uchar = 0;
    let mut b: libc::c_uchar = 0;
    let mut h: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut rowbytes: libc::c_int = 0;
    let mut eol: libc::c_int = 0;
    let mut eoi: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nbytes: libc::c_int = 0;
    p = data_ptr;
    rowbytes = (width + 1i32) / 2i32;
    memset(
        data_ptr as *mut libc::c_void,
        0i32,
        (rowbytes * height) as libc::c_ulong,
    );
    v = 0i32;
    eoi = 0i32;
    while v < height && eoi == 0 {
        h = 0i32;
        eol = 0i32;
        while h < width && eol == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            count += 2i32;
            p = data_ptr
                .offset((v * rowbytes) as isize)
                .offset((h / 2i32) as isize);
            if b0 as libc::c_int == 0i32 {
                match b1 as libc::c_int {
                    0 => {
                        /* Check for EOL and EOI marker */
                        /* EOL */
                        eol = 1i32
                    }
                    1 => {
                        /* EOI */
                        eoi = 1i32
                    }
                    2 => {
                        h += tt_get_unsigned_byte(handle) as libc::c_int;
                        v += tt_get_unsigned_byte(handle) as libc::c_int;
                        count += 2i32
                    }
                    _ => {
                        if h + b1 as libc::c_int > width {
                            dpx_warning(
                                b"RLE decode failed...\x00" as *const u8 as *const libc::c_char,
                            );
                            return -1i32;
                        }
                        nbytes = (b1 as libc::c_int + 1i32) / 2i32;
                        if h % 2i32 != 0 {
                            /* starting at hi-nib */
                            i = 0i32;
                            while i < nbytes {
                                b = tt_get_unsigned_byte(handle);
                                let fresh0 = p;
                                p = p.offset(1);
                                *fresh0 = (*fresh0 as libc::c_int
                                    | b as libc::c_int >> 4i32 & 0xfi32)
                                    as libc::c_uchar;
                                *p = ((b as libc::c_int) << 4i32 & 0xf0i32) as libc::c_uchar;
                                i += 1
                            }
                        } else if ttstub_input_read(
                            handle,
                            p as *mut libc::c_char,
                            nbytes as size_t,
                        ) != nbytes as libc::c_long
                        {
                            return -1i32;
                        }
                        h += b1 as libc::c_int;
                        count += nbytes;
                        if nbytes % 2i32 != 0 {
                            tt_get_unsigned_byte(handle);
                            count += 1
                        }
                    }
                }
            } else {
                if h + b0 as libc::c_int > width {
                    dpx_warning(b"RLE decode failed...\x00" as *const u8 as *const libc::c_char);
                    return -1i32;
                }
                if h % 2i32 != 0 {
                    let fresh1 = p;
                    p = p.offset(1);
                    *fresh1 = (b1 as libc::c_int >> 4i32 & 0xfi32) as libc::c_uchar;
                    b1 = ((b1 as libc::c_int) << 4i32 & 0xf0i32
                        | b1 as libc::c_int >> 4i32 & 0xfi32)
                        as libc::c_uchar;
                    b0 = b0.wrapping_sub(1);
                    h += 1
                }
                nbytes = (b0 as libc::c_int + 1i32) / 2i32;
                memset(
                    p as *mut libc::c_void,
                    b1 as libc::c_int,
                    nbytes as libc::c_ulong,
                );
                h += b0 as libc::c_int;
                if h % 2i32 != 0 {
                    let ref mut fresh2 = *p.offset((nbytes - 1i32) as isize);
                    *fresh2 = (*fresh2 as libc::c_int & 0xf0i32) as libc::c_uchar
                }
            }
        }
        /* next row ... */
        if eol == 0 && eoi == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            if b0 as libc::c_int != 0i32 {
                dpx_warning(
                    b"No EOL/EOI marker. RLE decode failed...\x00" as *const u8
                        as *const libc::c_char,
                );
                return -1i32;
            } else {
                if b1 as libc::c_int == 0x1i32 {
                    eoi = 1i32
                } else if b1 as libc::c_int != 0i32 {
                    dpx_warning(
                        b"No EOL/EOI marker. RLE decode failed...\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -1i32;
                }
            }
        }
        v += 1
    }
    return count;
}
/* Check for EOL and EOI marker */
