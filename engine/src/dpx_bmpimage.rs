#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::warn;

extern crate libc;

use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_add_stream, pdf_new_array, pdf_new_name, pdf_new_number,
    pdf_new_stream, pdf_new_string, pdf_obj, pdf_release_obj, pdf_stream_dict,
};
use libc::free;

extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: i32) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut i8, len: size_t) -> ssize_t;
    #[no_mangle]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8;
    /* Name does not include the / */
    /* pdf_add_dict requires key but pdf_add_array does not.
     * pdf_add_array always append elements to array.
     * They should be pdf_put_array(array, idx, element) and
     * pdf_put_dict(dict, key, value)
     */
    /* pdf_add_dict() want pdf_obj as key, however, key must always be name
     * object and pdf_lookup_dict() and pdf_remove_dict() uses const char as
     * key. This strange difference seems come from pdfdoc that first allocate
     * name objects frequently used (maybe 1000 times) such as /Type and does
     * pdf_link_obj() it rather than allocate/free-ing them each time. But I
     * already removed that.
     */
    #[no_mangle]
    fn pdf_stream_set_predictor(
        stream: *mut pdf_obj,
        predictor: i32,
        columns: i32,
        bpc: i32,
        colors: i32,
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
}
pub type __ssize_t = i64;
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ximage_info {
    pub flags: i32,
    pub width: i32,
    pub height: i32,
    pub bits_per_component: i32,
    pub num_components: i32,
    pub min_dpi: i32,
    pub xdensity: f64,
    pub ydensity: f64,
}
use crate::dpx_pdfximage::pdf_ximage;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_info {
    pub offset: u32,
    pub hsize: u32,
    pub width: u32,
    pub height: i32,
    pub compression: i32,
    pub bit_count: u16,
    pub psize: i32,
    pub x_pix_per_meter: u32,
    pub y_pix_per_meter: u32,
}
#[no_mangle]
pub unsafe extern "C" fn check_for_bmp(mut handle: rust_input_handle_t) -> i32 {
    let mut sigbytes: [u8; 2] = [0; 2];
    if handle.is_null() {
        return 0i32;
    }
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if ttstub_input_read(
        handle,
        sigbytes.as_mut_ptr() as *mut i8,
        ::std::mem::size_of::<[u8; 2]>() as u64,
    ) as u64
        != ::std::mem::size_of::<[u8; 2]>() as u64
        || sigbytes[0] as i32 != 'B' as i32
        || sigbytes[1] as i32 != 'M' as i32
    {
        return 0i32;
    }
    1i32
}
unsafe extern "C" fn get_density(
    mut xdensity: *mut f64,
    mut ydensity: *mut f64,
    mut hdr: *mut hdr_info,
) {
    if (*hdr).x_pix_per_meter > 0_u32 && (*hdr).y_pix_per_meter > 0_u32 {
        /* 0 for undefined. FIXME */
        *xdensity = 72.0f64 / ((*hdr).x_pix_per_meter as f64 * 0.0254f64);
        *ydensity = 72.0f64 / ((*hdr).y_pix_per_meter as f64 * 0.0254f64)
    } else {
        *xdensity = 1.0f64;
        *ydensity = 1.0f64
    };
}
#[no_mangle]
pub unsafe extern "C" fn bmp_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut u32,
    mut height: *mut u32,
    mut xdensity: *mut f64,
    mut ydensity: *mut f64,
) -> i32 {
    let mut r: i32 = 0;
    let mut hdr: hdr_info = {
        let mut init = hdr_info {
            offset: 0_u32,
            hsize: 0_u32,
            width: 0_u32,
            height: 0i32,
            compression: 0i32,
            bit_count: 0_u16,
            psize: 0i32,
            x_pix_per_meter: 0_u32,
            y_pix_per_meter: 0_u32,
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
    }) as u32;
    get_density(xdensity, ydensity, &mut hdr);
    r
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
) -> i32 {
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
            offset: 0_u32,
            hsize: 0_u32,
            width: 0_u32,
            height: 0i32,
            compression: 0i32,
            bit_count: 0_u16,
            psize: 0i32,
            x_pix_per_meter: 0_u32,
            y_pix_per_meter: 0_u32,
        };
        init
    };
    let mut num_palette: i32 = 0;
    let mut flip: i32 = 0;
    let mut i: i32 = 0;
    pdf_ximage_init_image_info(&mut info);
    colorspace = 0 as *mut pdf_obj;
    stream_dict = colorspace;
    stream = stream_dict;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if read_header(handle, &mut hdr) < 0i32 {
        return -1i32;
    }
    get_density(&mut info.xdensity, &mut info.ydensity, &mut hdr);
    info.width = hdr.width as i32;
    info.height = hdr.height;
    if info.height < 0i32 {
        info.height = -info.height;
        flip = 0i32
    } else {
        flip = 1i32
    }
    if (hdr.bit_count as i32) < 24i32 {
        if hdr.bit_count as i32 != 1i32
            && hdr.bit_count as i32 != 4i32
            && hdr.bit_count as i32 != 8i32
        {
            dpx_warning(
                b"Unsupported palette size: %hu\x00" as *const u8 as *const i8,
                hdr.bit_count as i32,
            );
            return -1i32;
        }
        num_palette = hdr
            .offset
            .wrapping_sub(hdr.hsize)
            .wrapping_sub(14_u32)
            .wrapping_div(hdr.psize as u32) as i32;
        info.bits_per_component = hdr.bit_count as i32;
        info.num_components = 1i32
    } else if hdr.bit_count as i32 == 24i32 {
        /* full color */
        num_palette = 1i32; /* dummy */
        info.bits_per_component = 8i32;
        info.num_components = 3i32
    } else {
        dpx_warning(
            b"Unkown/Unsupported BMP bitCount value: %hu\x00" as *const u8 as *const i8,
            hdr.bit_count as i32,
        );
        return -1i32;
    }
    if info.width == 0i32 || info.height == 0i32 || num_palette < 1i32 {
        warn!(
            "Invalid BMP file: width={}, height={}, #palette={}",
            info.width, info.height, num_palette,
        );
        return -1i32;
    }
    /* Start reading raster data */
    stream = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(stream);
    /* Color space: Indexed or DeviceRGB */
    if (hdr.bit_count as i32) < 24i32 {
        let mut lookup: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut palette: *mut u8 = 0 as *mut u8;
        let mut bgrq: [u8; 4] = [0; 4];
        palette = new(((num_palette * 3i32 + 1i32) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32) as *mut u8;
        i = 0i32;
        while i < num_palette {
            if ttstub_input_read(handle, bgrq.as_mut_ptr() as *mut i8, hdr.psize as size_t)
                != hdr.psize as i64
            {
                warn!("Reading file failed...");
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
            pdf_new_name(b"Indexed\x00" as *const u8 as *const i8),
        );
        pdf_add_array(
            colorspace,
            pdf_new_name(b"DeviceRGB\x00" as *const u8 as *const i8),
        );
        pdf_add_array(colorspace, pdf_new_number((num_palette - 1i32) as f64));
        pdf_add_array(colorspace, lookup);
    } else {
        colorspace = pdf_new_name(b"DeviceRGB\x00" as *const u8 as *const i8)
    }
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"ColorSpace\x00" as *const u8 as *const i8),
        colorspace,
    );
    /* Raster data of BMP is four-byte aligned. */
    let mut rowbytes: i32 = 0;
    let mut n: i32 = 0;
    let mut p: *mut u8 = 0 as *mut u8;
    let mut stream_data_ptr: *mut u8 = 0 as *mut u8;
    rowbytes = (info.width * hdr.bit_count as i32 + 7i32) / 8i32;
    ttstub_input_seek(handle, hdr.offset as ssize_t, 0i32);
    if hdr.compression == 0i32 {
        let mut dib_rowbytes: i32 = 0;
        let mut padding: i32 = 0;
        padding = if rowbytes % 4i32 != 0 {
            4i32 - rowbytes % 4i32
        } else {
            0i32
        };
        dib_rowbytes = rowbytes + padding;
        stream_data_ptr = new(((rowbytes * info.height + padding) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
        n = 0i32;
        while n < info.height {
            p = stream_data_ptr.offset((n * rowbytes) as isize);
            if ttstub_input_read(handle, p as *mut i8, dib_rowbytes as size_t)
                != dib_rowbytes as i64
            {
                warn!("Reading BMP raster data failed...");
                pdf_release_obj(stream);
                free(stream_data_ptr as *mut libc::c_void);
                return -1i32;
            }
            n += 1
        }
    } else if hdr.compression == 1i32 {
        stream_data_ptr = new(((rowbytes * info.height) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
        if read_raster_rle8(stream_data_ptr, info.width, info.height, handle) < 0i32 {
            warn!("Reading BMP raster data failed...");
            pdf_release_obj(stream);
            free(stream_data_ptr as *mut libc::c_void);
            return -1i32;
        }
    } else if hdr.compression == 2i32 {
        stream_data_ptr = new(((rowbytes * info.height) as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<u8>() as u64) as u32)
            as *mut u8;
        if read_raster_rle4(stream_data_ptr, info.width, info.height, handle) < 0i32 {
            warn!("Reading BMP raster data failed...");
            pdf_release_obj(stream);
            free(stream_data_ptr as *mut libc::c_void);
            return -1i32;
        }
    } else {
        warn!(
            "Unknown/Unsupported compression type for BMP image: {}",
            hdr.compression
        );
        pdf_release_obj(stream);
        return -1i32;
    }
    /* gbr --> rgb */
    if hdr.bit_count as i32 == 24i32 {
        n = 0i32;
        while n < info.width * info.height * 3i32 {
            let mut g: u8 = 0;
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
    if hdr.bit_count as i32 >= 24i32 && info.bits_per_component >= 8i32 && info.height > 64i32 {
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
    0i32
}
unsafe extern "C" fn read_header(mut handle: rust_input_handle_t, mut hdr: *mut hdr_info) -> i32 {
    let mut buf: [u8; 142] = [0; 142];
    let mut p: *mut u8 = 0 as *mut u8;
    p = buf.as_mut_ptr();
    if ttstub_input_read(
        handle,
        buf.as_mut_ptr() as *mut i8,
        (14i32 + 4i32) as size_t,
    ) != (14i32 + 4i32) as i64
    {
        warn!("Could not read BMP file header...");
        return -1i32;
    }
    if *p.offset(0) as i32 != 'B' as i32 || *p.offset(1) as i32 != 'M' as i32 {
        warn!("File not starting with \'B\' \'M\'... Not a BMP file?");
        return -1i32;
    }
    p = p.offset(2);
    /* fsize  = ULONG_LE(p); */
    p = p.offset(4);
    if *p.offset(0) as i32
        + ((*p.offset(1) as i32) << 8i32)
        + ((*p.offset(2) as i32) << 16i32)
        + ((*p.offset(3) as i32) << 24i32)
        != 0i32
    {
        warn!("Not a BMP file???");
        return -1i32;
    }
    p = p.offset(4);
    (*hdr).offset = (*p.offset(0) as i32
        + ((*p.offset(1) as i32) << 8i32)
        + ((*p.offset(2) as i32) << 16i32)
        + ((*p.offset(3) as i32) << 24i32)) as u32;
    p = p.offset(4);
    /* info header */
    (*hdr).hsize = (*p.offset(0) as i32
        + ((*p.offset(1) as i32) << 8i32)
        + ((*p.offset(2) as i32) << 16i32)
        + ((*p.offset(3) as i32) << 24i32)) as u32; /* undefined. FIXME */
    p = p.offset(4); /* undefined. FIXME */
    if ttstub_input_read(
        handle,
        p as *mut i8,
        (*hdr).hsize.wrapping_sub(4_u32) as size_t,
    ) != (*hdr).hsize.wrapping_sub(4_u32) as i64
    {
        warn!("Could not read BMP file header...");
        return -1i32;
    }
    match (*hdr).hsize {
        12 => {
            (*hdr).width = (*p.offset(0) as i32 + ((*p.offset(1) as i32) << 8i32)) as u32;
            p = p.offset(2);
            (*hdr).height = *p.offset(0) as i32 + ((*p.offset(1) as i32) << 8i32);
            p = p.offset(2);
            (*hdr).x_pix_per_meter = 0_u32;
            (*hdr).y_pix_per_meter = 0_u32;
            if *p.offset(0) as i32 + ((*p.offset(1) as i32) << 8i32) != 1i32 {
                warn!("Unknown bcPlanes value in BMP COREHEADER.");
                return -1i32;
            }
            p = p.offset(2);
            (*hdr).bit_count = (*p.offset(0) as i32 + ((*p.offset(1) as i32) << 8i32)) as u16;
            p = p.offset(2);
            (*hdr).compression = 0i32;
            (*hdr).psize = 3i32
        }
        40 | 64 | 108 | 124 => {
            (*hdr).width = (*p.offset(0) as i32
                + ((*p.offset(1) as i32) << 8i32)
                + ((*p.offset(2) as i32) << 16i32)
                + ((*p.offset(3) as i32) << 24i32)) as u32;
            p = p.offset(4);
            (*hdr).height = *p.offset(0) as i32
                + ((*p.offset(1) as i32) << 8i32)
                + ((*p.offset(2) as i32) << 16i32)
                + ((*p.offset(3) as i32) << 24i32);
            p = p.offset(4);
            if *p.offset(0) as i32 + ((*p.offset(1) as i32) << 8i32) != 1i32 {
                warn!("Unknown biPlanes value in BMP INFOHEADER.");
                return -1i32;
            }
            p = p.offset(2);
            (*hdr).bit_count = (*p.offset(0) as i32 + ((*p.offset(1) as i32) << 8i32)) as u16;
            p = p.offset(2);
            (*hdr).compression = *p.offset(0) as i32
                + ((*p.offset(1) as i32) << 8i32)
                + ((*p.offset(2) as i32) << 16i32)
                + ((*p.offset(3) as i32) << 24i32);
            p = p.offset(4);
            /* ignore biSizeImage */
            p = p.offset(4);
            (*hdr).x_pix_per_meter = (*p.offset(0) as i32
                + ((*p.offset(1) as i32) << 8i32)
                + ((*p.offset(2) as i32) << 16i32)
                + ((*p.offset(3) as i32) << 24i32)) as u32;
            p = p.offset(4);
            (*hdr).y_pix_per_meter = (*p.offset(0) as i32
                + ((*p.offset(1) as i32) << 8i32)
                + ((*p.offset(2) as i32) << 16i32)
                + ((*p.offset(3) as i32) << 24i32)) as u32;
            p = p.offset(4);
            (*hdr).psize = 4i32
        }
        _ => {
            warn!("Unknown BMP header type.");
            return -1i32;
        }
    }
    0i32
}
unsafe extern "C" fn read_raster_rle8(
    mut data_ptr: *mut u8,
    mut width: i32,
    mut height: i32,
    mut handle: rust_input_handle_t,
) -> i32 {
    let mut count: i32 = 0i32;
    let mut p: *mut u8 = 0 as *mut u8;
    let mut b0: u8 = 0;
    let mut b1: u8 = 0;
    let mut h: i32 = 0;
    let mut v: i32 = 0;
    let mut rowbytes: i32 = 0;
    let mut eol: i32 = 0;
    let mut eoi: i32 = 0;
    p = data_ptr;
    rowbytes = width;
    memset(
        data_ptr as *mut libc::c_void,
        0i32,
        (rowbytes * height) as u64,
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
            if b0 as i32 == 0i32 {
                match b1 as i32 {
                    0 => {
                        /* EOL */
                        eol = 1i32
                    }
                    1 => {
                        /* EOI */
                        eoi = 1i32
                    }
                    2 => {
                        h += tt_get_unsigned_byte(handle) as i32;
                        v += tt_get_unsigned_byte(handle) as i32;
                        count += 2i32
                    }
                    _ => {
                        h += b1 as i32;
                        if h > width {
                            warn!("RLE decode failed...");
                            return -1i32;
                        }
                        if ttstub_input_read(handle, p as *mut i8, b1 as size_t) != b1 as i64 {
                            return -1i32;
                        }
                        count += b1 as i32;
                        if b1 as i32 % 2i32 != 0 {
                            tt_get_unsigned_byte(handle);
                            count += 1
                        }
                    }
                }
            } else {
                h += b0 as i32;
                if h > width {
                    warn!("RLE decode failed...");
                    return -1i32;
                }
                memset(p as *mut libc::c_void, b1 as i32, b0 as u64);
            }
        }
        /* next row ... */
        if eol == 0 && eoi == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            if b0 as i32 != 0i32 {
                warn!("RLE decode failed...");
                return -1i32;
            } else {
                if b1 as i32 == 0x1i32 {
                    eoi = 1i32
                } else if b1 as i32 != 0i32 {
                    warn!("RLE decode failed...");
                    return -1i32;
                }
            }
        }
        v += 1
    }
    count
}
unsafe extern "C" fn read_raster_rle4(
    mut data_ptr: *mut u8,
    mut width: i32,
    mut height: i32,
    mut handle: rust_input_handle_t,
) -> i32 {
    let mut count: i32 = 0i32;
    let mut p: *mut u8 = 0 as *mut u8;
    let mut b0: u8 = 0;
    let mut b1: u8 = 0;
    let mut b: u8 = 0;
    let mut h: i32 = 0;
    let mut v: i32 = 0;
    let mut rowbytes: i32 = 0;
    let mut eol: i32 = 0;
    let mut eoi: i32 = 0;
    let mut i: i32 = 0;
    let mut nbytes: i32 = 0;
    p = data_ptr;
    rowbytes = (width + 1i32) / 2i32;
    memset(
        data_ptr as *mut libc::c_void,
        0i32,
        (rowbytes * height) as u64,
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
            if b0 as i32 == 0i32 {
                match b1 as i32 {
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
                        h += tt_get_unsigned_byte(handle) as i32;
                        v += tt_get_unsigned_byte(handle) as i32;
                        count += 2i32
                    }
                    _ => {
                        if h + b1 as i32 > width {
                            warn!("RLE decode failed...");
                            return -1i32;
                        }
                        nbytes = (b1 as i32 + 1i32) / 2i32;
                        if h % 2i32 != 0 {
                            /* starting at hi-nib */
                            i = 0i32;
                            while i < nbytes {
                                b = tt_get_unsigned_byte(handle);
                                let fresh0 = p;
                                p = p.offset(1);
                                *fresh0 = (*fresh0 as i32 | b as i32 >> 4i32 & 0xfi32) as u8;
                                *p = ((b as i32) << 4i32 & 0xf0i32) as u8;
                                i += 1
                            }
                        } else if ttstub_input_read(handle, p as *mut i8, nbytes as size_t)
                            != nbytes as i64
                        {
                            return -1i32;
                        }
                        h += b1 as i32;
                        count += nbytes;
                        if nbytes % 2i32 != 0 {
                            tt_get_unsigned_byte(handle);
                            count += 1
                        }
                    }
                }
            } else {
                if h + b0 as i32 > width {
                    warn!("RLE decode failed...");
                    return -1i32;
                }
                if h % 2i32 != 0 {
                    let fresh1 = p;
                    p = p.offset(1);
                    *fresh1 = (b1 as i32 >> 4i32 & 0xfi32) as u8;
                    b1 = ((b1 as i32) << 4i32 & 0xf0i32 | b1 as i32 >> 4i32 & 0xfi32) as u8;
                    b0 = b0.wrapping_sub(1);
                    h += 1
                }
                nbytes = (b0 as i32 + 1i32) / 2i32;
                memset(p as *mut libc::c_void, b1 as i32, nbytes as u64);
                h += b0 as i32;
                if h % 2i32 != 0 {
                    let ref mut fresh2 = *p.offset((nbytes - 1i32) as isize);
                    *fresh2 = (*fresh2 as i32 & 0xf0i32) as u8
                }
            }
        }
        /* next row ... */
        if eol == 0 && eoi == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            if b0 as i32 != 0i32 {
                warn!("No EOL/EOI marker. RLE decode failed...");
                return -1i32;
            } else {
                if b1 as i32 == 0x1i32 {
                    eoi = 1i32
                } else if b1 as i32 != 0i32 {
                    warn!("No EOL/EOI marker. RLE decode failed...");
                    return -1i32;
                }
            }
        }
        v += 1
    }
    count
}
/* Check for EOL and EOI marker */
