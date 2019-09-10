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
    pub type png_info_def;
    pub type png_struct_def;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> libc::c_int;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: libc::c_int,
    ) -> size_t;
    #[no_mangle]
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut i8,
        len: size_t,
    ) -> ssize_t;
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
    #[no_mangle]
    fn pdf_get_version() -> libc::c_uint;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_number(value: libc::c_double) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_string(str: *const libc::c_void, length: size_t) -> *mut pdf_obj;
    /* Name does not include the / */
    #[no_mangle]
    fn pdf_new_name(name: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    /* pdf_add_dict requires key but pdf_add_array does not.
     * pdf_add_array always append elements to array.
     * They should be pdf_put_array(array, idx, element) and
     * pdf_put_dict(dict, key, value)
     */
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
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
    fn iccp_check_colorspace(
        colortype: libc::c_int,
        profile: *const libc::c_void,
        proflen: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn iccp_load_profile(
        ident: *const i8,
        profile: *const libc::c_void,
        proflen: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn pdf_get_colorspace_reference(cspc_id: libc::c_int) -> *mut pdf_obj;
    /* Called by pngimage, jpegimage, epdf, mpost, etc. */
    #[no_mangle]
    fn pdf_ximage_init_image_info(info: *mut ximage_info);
    #[no_mangle]
    fn pdf_ximage_set_image(
        ximage: *mut pdf_ximage,
        info: *mut libc::c_void,
        resource: *mut pdf_obj,
    );
    #[no_mangle]
    fn png_sig_cmp(sig: png_const_bytep, start: size_t, num_to_check: size_t) -> libc::c_int;
    #[no_mangle]
    fn png_create_read_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    #[no_mangle]
    fn png_create_info_struct(png_ptr: png_const_structrp) -> png_infop;
    #[no_mangle]
    fn png_read_info(png_ptr: png_structrp, info_ptr: png_inforp);
    #[no_mangle]
    fn png_set_expand_gray_1_2_4_to_8(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_packing(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_background(
        png_ptr: png_structrp,
        background_color: png_const_color_16p,
        background_gamma_code: libc::c_int,
        need_expand: libc::c_int,
        background_gamma: libc::c_double,
    );
    #[no_mangle]
    fn png_set_strip_16(png_ptr: png_structrp);
    #[no_mangle]
    fn png_set_gamma(
        png_ptr: png_structrp,
        screen_gamma: libc::c_double,
        override_file_gamma: libc::c_double,
    );
    #[no_mangle]
    fn png_read_update_info(png_ptr: png_structrp, info_ptr: png_inforp);
    #[no_mangle]
    fn png_read_image(png_ptr: png_structrp, image: png_bytepp);
    #[no_mangle]
    fn png_read_end(png_ptr: png_structrp, info_ptr: png_inforp);
    #[no_mangle]
    fn png_destroy_info_struct(png_ptr: png_const_structrp, info_ptr_ptr: png_infopp);
    #[no_mangle]
    fn png_destroy_read_struct(
        png_ptr_ptr: png_structpp,
        info_ptr_ptr: png_infopp,
        end_info_ptr_ptr: png_infopp,
    );
    #[no_mangle]
    fn png_set_read_fn(png_ptr: png_structrp, io_ptr: png_voidp, read_data_fn: png_rw_ptr);
    #[no_mangle]
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    #[no_mangle]
    fn png_get_valid(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        flag: png_uint_32,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_rowbytes(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> size_t;
    #[no_mangle]
    fn png_get_image_width(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_uint_32;
    #[no_mangle]
    fn png_get_image_height(png_ptr: png_const_structrp, info_ptr: png_const_inforp)
        -> png_uint_32;
    #[no_mangle]
    fn png_get_bit_depth(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    #[no_mangle]
    fn png_get_color_type(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    #[no_mangle]
    fn png_get_x_pixels_per_meter(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_y_pixels_per_meter(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_cHRM(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        white_x: *mut libc::c_double,
        white_y: *mut libc::c_double,
        red_x: *mut libc::c_double,
        red_y: *mut libc::c_double,
        green_x: *mut libc::c_double,
        green_y: *mut libc::c_double,
        blue_x: *mut libc::c_double,
        blue_y: *mut libc::c_double,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_gAMA(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        file_gamma: *mut libc::c_double,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut libc::c_int,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_sRGB(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        file_srgb_intent: *mut libc::c_int,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_iCCP(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        name: png_charpp,
        compression_type: *mut libc::c_int,
        profile: png_bytepp,
        proflen: *mut png_uint_32,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_tRNS(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        trans_alpha: *mut png_bytep,
        num_trans: *mut libc::c_int,
        trans_color: *mut png_color_16p,
    ) -> png_uint_32;
    #[no_mangle]
    fn png_get_text(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        text_ptr: *mut png_textp,
        num_text: *mut libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn png_set_option(
        png_ptr: png_structrp,
        option: libc::c_int,
        onoff: libc::c_int,
    ) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type int32_t = __int32_t;
pub type size_t = u64;
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
pub type png_byte = u8;
pub type png_infopp = *mut *mut png_info;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_error_ptr = Option<unsafe extern "C" fn(_: png_structp, _: png_const_charp) -> ()>;
pub type png_const_charp = *const i8;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_color_16p = *mut png_color_16;
pub type png_color_16 = png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_uint_16 = u16;
pub type png_structpp = *mut *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_text = png_text_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_text_struct {
    pub compression: libc::c_int,
    pub key: png_charp,
    pub text: png_charp,
    pub text_length: size_t,
    pub itxt_length: size_t,
    pub lang: png_charp,
    pub lang_key: png_charp,
}
pub type png_charp = *mut i8;
pub type png_textp = *mut png_text;
pub type png_bytep = *mut png_byte;
pub type png_uint_32 = libc::c_uint;
pub type png_uint_32p = *mut png_uint_32;
pub type png_const_inforp = *const png_info;
pub type png_bytepp = *mut *mut png_byte;
pub type png_charpp = *mut *mut i8;
pub type png_color = png_color_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_colorp = *mut png_color;
pub type png_const_color_16p = *const png_color_16;
pub type png_size_t = size_t;
pub type png_voidp = *mut libc::c_void;
pub type png_rw_ptr = Option<unsafe extern "C" fn(_: png_structp, _: png_bytep, _: size_t) -> ()>;
pub type png_const_bytep = *const png_byte;
#[no_mangle]
pub unsafe extern "C" fn check_for_png(mut handle: rust_input_handle_t) -> libc::c_int {
    let mut sigbytes: [u8; 8] = [0; 8];
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    if ttstub_input_read(
        handle,
        sigbytes.as_mut_ptr() as *mut i8,
        ::std::mem::size_of::<[u8; 8]>() as u64,
    ) as u64
        != ::std::mem::size_of::<[u8; 8]>() as u64
        || png_sig_cmp(
            sigbytes.as_mut_ptr() as png_const_bytep,
            0i32 as size_t,
            ::std::mem::size_of::<[u8; 8]>() as u64,
        ) != 0
    {
        return 0i32;
    } else {
        return 1i32;
    };
}
unsafe extern "C" fn _png_warning_callback(mut png_ptr: png_structp, mut msg: png_const_charp) {
    /* Make compiler happy */
}
unsafe extern "C" fn _png_read(
    mut png_ptr: png_structp,
    mut outbytes: png_bytep,
    mut n: png_size_t,
) {
    let mut handle: rust_input_handle_t = png_get_io_ptr(png_ptr as *const png_struct);
    let mut r: ssize_t = 0;
    r = ttstub_input_read(handle, outbytes as *mut i8, n);
    if r < 0i32 as libc::c_long || r as size_t != n {
        _tt_abort(b"error reading PNG\x00" as *const u8 as *const i8);
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
) -> libc::c_int {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut colorspace: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut mask: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut intent: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut stream_data_ptr: png_bytep = 0 as *mut png_byte;
    let mut trans_type: libc::c_int = 0;
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
    /* Libpng stuff */
    let mut png_ptr: png_structp = 0 as *mut png_struct;
    let mut png_info_ptr: png_infop = 0 as *mut png_info;
    let mut bpc: png_byte = 0;
    let mut color_type: png_byte = 0;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut rowbytes: png_uint_32 = 0;
    pdf_ximage_init_image_info(&mut info);
    stream = 0 as *mut pdf_obj;
    stream_dict = 0 as *mut pdf_obj;
    intent = 0 as *mut pdf_obj;
    mask = intent;
    colorspace = mask;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    png_ptr = png_create_read_struct(
        b"1.6.37\x00" as *const u8 as *const i8,
        0 as *mut libc::c_void,
        None,
        Some(
            _png_warning_callback as unsafe extern "C" fn(_: png_structp, _: png_const_charp) -> (),
        ),
    );
    if png_ptr.is_null() {
        dpx_warning(
            b"%s: Creating Libpng read struct failed.\x00" as *const u8 as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        return -1i32;
    }
    png_info_ptr = png_create_info_struct(png_ptr as *const png_struct);
    if png_info_ptr.is_null() {
        dpx_warning(
            b"%s: Creating Libpng info struct failed.\x00" as *const u8 as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        png_destroy_read_struct(&mut png_ptr, 0 as png_infopp, 0 as png_infopp);
        return -1i32;
    }
    /* ignore possibly incorrect CMF bytes */
    png_set_option(png_ptr, 2i32, 3i32);
    /* Rust-backed IO */
    png_set_read_fn(
        png_ptr,
        handle,
        Some(_png_read as unsafe extern "C" fn(_: png_structp, _: png_bytep, _: png_size_t) -> ()),
    );
    /* NOTE: could use png_set_sig_bytes() to tell libpng if we started at non-zero file offset */
    /* Read PNG info-header and get some info. */
    png_read_info(png_ptr, png_info_ptr);
    color_type = png_get_color_type(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    width = png_get_image_width(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    height = png_get_image_height(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    bpc = png_get_bit_depth(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    if bpc as libc::c_int > 8i32 {
        if pdf_get_version() < 5i32 as libc::c_uint {
            /* Ask libpng to convert down to 8-bpc. */
            dpx_warning(
                b"%s: 16-bpc PNG requires PDF version 1.5.\x00" as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
            );
            png_set_strip_16(png_ptr);
            bpc = 8i32 as png_byte
        }
    } else if (bpc as libc::c_int) < 8i32 {
        /* Instruct libpng to scale each pixel color to a full byte while
        reading even though there's only 1/2/4 bits of color associated. */
        if color_type as libc::c_int == 0i32 || color_type as libc::c_int == 4i32 {
            png_set_expand_gray_1_2_4_to_8(png_ptr);
        } else {
            png_set_packing(png_ptr);
        }
        bpc = 8i32 as png_byte
    }
    /* Ask libpng to gamma-correct.
     * It is wrong to assume screen gamma value 2.2 but...
     * We do gamma correction here only when uncalibrated color space is used.
     */
    if png_get_valid(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
        0x1000u32,
    ) == 0
        && png_get_valid(
            png_ptr as *const png_struct,
            png_info_ptr as *const png_info,
            0x800u32,
        ) == 0
        && png_get_valid(
            png_ptr as *const png_struct,
            png_info_ptr as *const png_info,
            0x4u32,
        ) == 0
        && png_get_valid(
            png_ptr as *const png_struct,
            png_info_ptr as *const png_info,
            0x1u32,
        ) != 0
    {
        let mut G: libc::c_double = 1.0f64;
        png_get_gAMA(
            png_ptr as *const png_struct,
            png_info_ptr as *const png_info,
            &mut G,
        );
        png_set_gamma(png_ptr, 2.2f64, G);
    }
    trans_type = check_transparency(png_ptr, png_info_ptr);
    /* check_transparency() does not do updata_info() */
    png_read_update_info(png_ptr, png_info_ptr);
    rowbytes = png_get_rowbytes(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    ) as png_uint_32;
    /* Values listed below will not be modified in the remaining process. */
    info.width = width as libc::c_int;
    info.height = height as libc::c_int;
    info.bits_per_component = bpc as libc::c_int;
    let mut xppm: png_uint_32 = png_get_x_pixels_per_meter(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    let mut yppm: png_uint_32 = png_get_y_pixels_per_meter(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    if xppm > 0i32 as libc::c_uint {
        info.xdensity = 72.0f64 / 0.0254f64 / xppm as libc::c_double
    }
    if yppm > 0i32 as libc::c_uint {
        info.ydensity = 72.0f64 / 0.0254f64 / yppm as libc::c_double
    }
    stream = pdf_new_stream(1i32 << 0i32);
    stream_dict = pdf_stream_dict(stream);
    stream_data_ptr = new((rowbytes.wrapping_mul(height) as u64)
        .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
        as u32) as *mut png_byte;
    read_image_data(png_ptr, stream_data_ptr, height, rowbytes);
    /* Non-NULL intent means there is valid sRGB chunk. */
    intent = get_rendering_intent(png_ptr, png_info_ptr);
    if !intent.is_null() {
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"Intent\x00" as *const u8 as *const i8),
            intent,
        );
    }
    match color_type as libc::c_int {
        3 => {
            colorspace = create_cspace_Indexed(png_ptr, png_info_ptr);
            match trans_type {
                1 => {
                    /* Color-key masking */
                    mask = create_ckey_mask(png_ptr, png_info_ptr)
                }
                2 => {
                    /* Soft mask */
                    mask = create_soft_mask(png_ptr, png_info_ptr, stream_data_ptr, width, height)
                }
                _ => {}
            }
            info.num_components = 1i32
        }
        2 | 6 => {
            if png_get_valid(
                png_ptr as *const png_struct,
                png_info_ptr as *const png_info,
                0x1000u32,
            ) != 0
            {
                colorspace = create_cspace_ICCBased(png_ptr, png_info_ptr)
            } else if !intent.is_null() {
                colorspace = create_cspace_sRGB(png_ptr, png_info_ptr)
            } else {
                colorspace = create_cspace_CalRGB(png_ptr, png_info_ptr)
            }
            if colorspace.is_null() {
                colorspace = pdf_new_name(b"DeviceRGB\x00" as *const u8 as *const i8)
            }
            match trans_type {
                1 => mask = create_ckey_mask(png_ptr, png_info_ptr),
                2 => {
                    /* rowbytes changes 4 to 3 at here */
                    mask = strip_soft_mask(
                        png_ptr,
                        png_info_ptr,
                        stream_data_ptr,
                        &mut rowbytes,
                        width,
                        height,
                    )
                }
                _ => mask = 0 as *mut pdf_obj,
            }
            info.num_components = 3i32
        }
        0 | 4 => {
            if png_get_valid(
                png_ptr as *const png_struct,
                png_info_ptr as *const png_info,
                0x1000u32,
            ) != 0
            {
                colorspace = create_cspace_ICCBased(png_ptr, png_info_ptr)
            } else if !intent.is_null() {
                colorspace = create_cspace_sRGB(png_ptr, png_info_ptr)
            } else {
                colorspace = create_cspace_CalGray(png_ptr, png_info_ptr)
            }
            if colorspace.is_null() {
                colorspace = pdf_new_name(b"DeviceGray\x00" as *const u8 as *const i8)
            }
            match trans_type {
                1 => mask = create_ckey_mask(png_ptr, png_info_ptr),
                2 => {
                    mask = strip_soft_mask(
                        png_ptr,
                        png_info_ptr,
                        stream_data_ptr,
                        &mut rowbytes,
                        width,
                        height,
                    )
                }
                _ => mask = 0 as *mut pdf_obj,
            }
            info.num_components = 1i32
        }
        _ => {
            dpx_warning(
                b"%s: Unknown PNG colortype %d.\x00" as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
                color_type as libc::c_int,
            );
        }
    }
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"ColorSpace\x00" as *const u8 as *const i8),
        colorspace,
    );
    pdf_add_stream(
        stream,
        stream_data_ptr as *const libc::c_void,
        rowbytes.wrapping_mul(height) as libc::c_int,
    );
    free(stream_data_ptr as *mut libc::c_void);
    if !mask.is_null() {
        if trans_type == 1i32 {
            pdf_add_dict(
                stream_dict,
                pdf_new_name(b"Mask\x00" as *const u8 as *const i8),
                mask,
            );
        } else if trans_type == 2i32 {
            if info.bits_per_component >= 8i32 && info.width > 64i32 {
                pdf_stream_set_predictor(mask, 2i32, info.width, info.bits_per_component, 1i32);
            }
            pdf_add_dict(
                stream_dict,
                pdf_new_name(b"SMask\x00" as *const u8 as *const i8),
                pdf_ref_obj(mask),
            );
            pdf_release_obj(mask);
        } else {
            dpx_warning(
                b"%s: Unknown transparency type...???\x00" as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
            );
            pdf_release_obj(mask);
        }
    }
    /* Finally read XMP Metadata
     * See, XMP Specification Part 3, Storage in Files
     * http://www.adobe.com/jp/devnet/xmp.html
     *
     * We require libpng version >= 1.6.14 since prior versions
     * of libpng had a bug that incorrectly treat the compression
     * flag of iTxt chunks.
     */
    if pdf_get_version() >= 4i32 as libc::c_uint {
        let mut text_ptr: png_textp = 0 as *mut png_text;
        let mut XMP_stream: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut XMP_stream_dict: *mut pdf_obj = 0 as *mut pdf_obj;
        let mut i: libc::c_int = 0;
        let mut num_text: libc::c_int = 0;
        let mut have_XMP: libc::c_int = 0i32;
        num_text = png_get_text(
            png_ptr as *const png_struct,
            png_info_ptr,
            &mut text_ptr,
            0 as *mut libc::c_int,
        );
        i = 0i32;
        while i < num_text {
            if memcmp(
                (*text_ptr.offset(i as isize)).key as *const libc::c_void,
                b"XML:com.adobe.xmp\x00" as *const u8 as *const i8 as *const libc::c_void,
                17i32 as u64,
            ) == 0
            {
                /* XMP found */
                if (*text_ptr.offset(i as isize)).compression != 1i32
                    || (*text_ptr.offset(i as isize)).itxt_length == 0i32 as u64
                {
                    dpx_warning(
                        b"%s: Invalid value(s) in iTXt chunk for XMP Metadata.\x00" as *const u8
                            as *const i8,
                        b"PNG\x00" as *const u8 as *const i8,
                    );
                } else if have_XMP != 0 {
                    dpx_warning(
                        b"%s: Multiple XMP Metadata. Don\'t know how to treat it.\x00" as *const u8
                            as *const i8,
                        b"PNG\x00" as *const u8 as *const i8,
                    );
                } else {
                    /* We compress XMP metadata for included images here.
                     * It is not recommended to compress XMP metadata for PDF documents but
                     * we compress XMP metadata for included images here to avoid confusing
                     * application programs that only want PDF document global XMP metadata
                     * and scan for that.
                     */
                    XMP_stream = pdf_new_stream(1i32 << 0i32);
                    XMP_stream_dict = pdf_stream_dict(XMP_stream);
                    pdf_add_dict(
                        XMP_stream_dict,
                        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
                        pdf_new_name(b"Metadata\x00" as *const u8 as *const i8),
                    );
                    pdf_add_dict(
                        XMP_stream_dict,
                        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
                        pdf_new_name(b"XML\x00" as *const u8 as *const i8),
                    );
                    pdf_add_stream(
                        XMP_stream,
                        (*text_ptr.offset(i as isize)).text as *const libc::c_void,
                        (*text_ptr.offset(i as isize)).itxt_length as libc::c_int,
                    );
                    pdf_add_dict(
                        stream_dict,
                        pdf_new_name(b"Metadata\x00" as *const u8 as *const i8),
                        pdf_ref_obj(XMP_stream),
                    );
                    pdf_release_obj(XMP_stream);
                    have_XMP = 1i32
                }
            }
            i += 1
        }
    }
    /* PNG_LIBPNG_VER */
    png_read_end(png_ptr, 0 as *mut png_info);
    /* Cleanup */
    if !png_info_ptr.is_null() {
        png_destroy_info_struct(png_ptr as *const png_struct, &mut png_info_ptr);
    }
    if !png_ptr.is_null() {
        png_destroy_read_struct(&mut png_ptr, 0 as png_infopp, 0 as png_infopp);
    }
    if color_type as libc::c_int != 2i32 | 1i32
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
/* Transparency */
/*
 * The returned value trans_type is the type of transparency to be used for
 * this image. Possible values are:
 *
 *   PDF_TRANS_TYPE_NONE    No Masking will be used/required.
 *   PDF_TRANS_TYPE_BINARY  Pixels are either fully opaque/fully transparent.
 *   PDF_TRANS_TYPE_ALPHA   Uses alpha channel, requies SMask.(PDF-1.4)
 *
 * check_transparency() must check the current setting of output PDF version
 * and must choose appropriate trans_type value according to PDF version of
 * current output PDF document.
 *
 * If the PDF version is less than 1.3, no transparency is supported for this
 * version of PDF, hence PDF_TRANS_TYPE_NONE must be returned. And when the PDF
 * version is equal to 1.3, possible retrun values are PDF_TRANS_TYPE_BINARY or
 * PDF_TRANS_TYPE_NONE. The latter case arises when PNG file uses alpha channel
 * explicitly (color type PNG_COLOR_TYPE_XXX_ALPHA), or the tRNS chunk for the
 * PNG_COLOR_TYPE_PALETTE image contains intermediate values of opacity.
 *
 * Finally, in the case of PDF version 1.4, all kind of translucent pixels can
 * be represented with Soft-Mask.
 */
unsafe extern "C" fn check_transparency(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> libc::c_int {
    let mut trans_type: libc::c_int = 0;
    let mut pdf_version: libc::c_uint = 0;
    let mut color_type: png_byte = 0;
    let mut trans_values: png_color_16p = 0 as *mut png_color_16;
    let mut trans: png_bytep = 0 as *mut png_byte;
    let mut num_trans: libc::c_int = 0;
    pdf_version = pdf_get_version();
    color_type = png_get_color_type(png_ptr as *const png_struct, info_ptr as *const png_info);
    /*
     * First we set trans_type to appropriate value for PNG image.
     */
    if color_type as libc::c_int == 2i32 | 4i32 || color_type as libc::c_int == 4i32 {
        trans_type = 2i32
    } else if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x10u32,
    ) != 0
        && png_get_tRNS(
            png_ptr as *const png_struct,
            info_ptr,
            &mut trans,
            &mut num_trans,
            &mut trans_values,
        ) != 0
    {
        match color_type as libc::c_int {
            3 => {
                /* no transparency */
                /* Have valid tRNS chunk. */
                /* Use color-key mask if possible. */
                trans_type = 1i32;
                loop {
                    let fresh0 = num_trans;
                    num_trans = num_trans - 1;
                    if !(fresh0 > 0i32) {
                        break;
                    }
                    if !(*trans.offset(num_trans as isize) as libc::c_int != 0i32
                        && *trans.offset(num_trans as isize) as libc::c_int != 0xffi32)
                    {
                        continue;
                    }
                    /* This seems not binary transparency */
                    trans_type = 2i32;
                    break;
                }
            }
            0 | 2 => {
                /* RGB or GRAY, single color specified by trans_values is transparent. */
                trans_type = 1i32
            }
            _ => {
                /* Else tRNS silently ignored. */
                trans_type = 0i32
            }
        }
    } else {
        trans_type = 0i32
    }
    /*
     * Now we check PDF version.
     * We can convert alpha cahnnels to explicit mask via user supplied alpha-
     * threshold value. But I will not do that.
     */
    if pdf_version < 3i32 as libc::c_uint && trans_type != 0i32
        || pdf_version < 4i32 as libc::c_uint && trans_type == 2i32
    {
        /*
         *   No transparency supported but PNG uses transparency, or Soft-Mask
         * required but no support for it is available in this version of PDF.
         * We must do pre-composition of image with the background image here. But,
         * we cannot do that in general since dvipdfmx is not a rasterizer. What we
         * can do here is to composite image with a rectangle filled with the
         * background color. However, images are stored as an Image XObject which
         * can be referenced anywhere in the PDF document content. Hence, we cannot
         * know the correct background color at this time. So we will choose white
         * as background color, which is most probable color in our cases.
         * We ignore bKGD chunk.
         */
        let mut bg: png_color_16 = png_color_16 {
            index: 0,
            red: 0,
            green: 0,
            blue: 0,
            gray: 0,
        };
        bg.red = 255i32 as png_uint_16;
        bg.green = 255i32 as png_uint_16;
        bg.blue = 255i32 as png_uint_16;
        bg.gray = 255i32 as png_uint_16;
        bg.index = 0i32 as png_byte;
        png_set_background(
            png_ptr,
            &mut bg as *mut png_color_16 as png_const_color_16p,
            1i32,
            0i32,
            1.0f64,
        );
        dpx_warning(
            b"%s: Transparency will be ignored. (no support in PDF ver. < 1.3)\x00" as *const u8
                as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        if pdf_version < 3i32 as libc::c_uint {
            dpx_warning(
                b"%s: Please use -V 3 option to enable binary transparency support.\x00"
                    as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
            );
        }
        if pdf_version < 4i32 as libc::c_uint {
            dpx_warning(
                b"%s: Please use -V 4 option to enable full alpha channel support.\x00" as *const u8
                    as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
            );
        }
        trans_type = 0i32
    }
    return trans_type;
}
/*
 * sRGB:
 *
 *   If sRGB chunk is present, cHRM and gAMA chunk must be ignored.
 *
 */
unsafe extern "C" fn get_rendering_intent(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut intent: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut srgb_intent: libc::c_int = 0;
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x800u32,
    ) != 0
        && png_get_sRGB(
            png_ptr as *const png_struct,
            info_ptr as *const png_info,
            &mut srgb_intent,
        ) != 0
    {
        match srgb_intent {
            2 => intent = pdf_new_name(b"Saturation\x00" as *const u8 as *const i8),
            0 => intent = pdf_new_name(b"Perceptual\x00" as *const u8 as *const i8),
            3 => {
                intent =
                    pdf_new_name(b"AbsoluteColorimetric\x00" as *const u8 as *const i8)
            }
            1 => {
                intent =
                    pdf_new_name(b"RelativeColorimetric\x00" as *const u8 as *const i8)
            }
            _ => {
                dpx_warning(
                    b"%s: Invalid value in PNG sRGB chunk: %d\x00" as *const u8
                        as *const i8,
                    b"PNG\x00" as *const u8 as *const i8,
                    srgb_intent,
                );
                intent = 0 as *mut pdf_obj
            }
        }
    } else {
        intent = 0 as *mut pdf_obj
    }
    return intent;
}
/* sRGB:
 *
 * We (and PDF) do not have direct sRGB support. The sRGB color space can be
 * precisely represented by ICC profile, but we use approximate CalRGB color
 * space.
 */
/* Approximated sRGB */
unsafe extern "C" fn create_cspace_sRGB(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cal_param: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut color_type: png_byte = 0;
    color_type = png_get_color_type(png_ptr as *const png_struct, info_ptr as *const png_info);
    /* Parameters taken from PNG spec. section 4.2.2.3. */
    cal_param = make_param_Cal(
        color_type, 2.2f64, 0.3127f64, 0.329f64, 0.64f64, 0.33f64, 0.3f64, 0.6f64, 0.15f64, 0.06f64,
    );
    if cal_param.is_null() {
        return 0 as *mut pdf_obj;
    }
    colorspace = pdf_new_array();
    match color_type as libc::c_int {
        2 | 6 | 3 => {
            pdf_add_array(
                colorspace,
                pdf_new_name(b"CalRGB\x00" as *const u8 as *const i8),
            );
        }
        0 | 4 => {
            pdf_add_array(
                colorspace,
                pdf_new_name(b"CalGray\x00" as *const u8 as *const i8),
            );
        }
        _ => {}
    }
    pdf_add_array(colorspace, cal_param);
    return colorspace;
}
/* ICCBased:
 *
 * Not supported yet.
 * Must check if ICC profile is valid and can be imported to PDF.
 * There are few restrictions (should be applied to PNG too?) in ICC profile
 * support in PDF. Some information should be obtained from profile.
 */
unsafe extern "C" fn create_cspace_ICCBased(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = 0 as *mut pdf_obj; /* Manual page for libpng does not
                                                           * clarify whether profile data is inflated by libpng.
                                                           */
    let mut csp_id: libc::c_int = 0;
    let mut colortype: libc::c_int = 0;
    let mut color_type: png_byte = 0;
    let mut name: png_charp = 0 as *mut i8;
    let mut compression_type: libc::c_int = 0;
    let mut profile: png_bytep = 0 as *mut png_byte;
    let mut proflen: png_uint_32 = 0;
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x1000u32,
    ) == 0
        || png_get_iCCP(
            png_ptr as *const png_struct,
            info_ptr,
            &mut name,
            &mut compression_type,
            &mut profile,
            &mut proflen,
        ) == 0
    {
        return 0 as *mut pdf_obj;
    }
    color_type = png_get_color_type(png_ptr as *const png_struct, info_ptr as *const png_info);
    if color_type as libc::c_int & 2i32 != 0 {
        colortype = -3i32
    } else {
        colortype = -1i32
    }
    if iccp_check_colorspace(
        colortype,
        profile as *const libc::c_void,
        proflen as libc::c_int,
    ) < 0i32
    {
        colorspace = 0 as *mut pdf_obj
    } else {
        csp_id = iccp_load_profile(
            name as *const i8,
            profile as *const libc::c_void,
            proflen as libc::c_int,
        );
        if csp_id < 0i32 {
            colorspace = 0 as *mut pdf_obj
        } else {
            colorspace = pdf_get_colorspace_reference(csp_id)
        }
    }
    /* Rendering intent ... */
    return colorspace;
}
/* CIE-Based: CalRGB/CalGray */
/*
 * gAMA, cHRM:
 *
 *   If cHRM is present, we use CIE-Based color space. gAMA is also used here
 * if available.
 */
unsafe extern "C" fn create_cspace_CalRGB(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cal_param: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut xw: libc::c_double = 0.;
    let mut yw: libc::c_double = 0.;
    let mut xr: libc::c_double = 0.;
    let mut yr: libc::c_double = 0.;
    let mut xg: libc::c_double = 0.;
    let mut yg: libc::c_double = 0.;
    let mut xb: libc::c_double = 0.;
    let mut yb: libc::c_double = 0.;
    let mut G: libc::c_double = 0.;
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x4u32,
    ) == 0
        || png_get_cHRM(
            png_ptr as *const png_struct,
            info_ptr as *const png_info,
            &mut xw,
            &mut yw,
            &mut xr,
            &mut yr,
            &mut xg,
            &mut yg,
            &mut xb,
            &mut yb,
        ) == 0
    {
        return 0 as *mut pdf_obj;
    }
    if xw <= 0.0f64
        || yw < 1.0e-10f64
        || xr < 0.0f64
        || yr < 0.0f64
        || xg < 0.0f64
        || yg < 0.0f64
        || xb < 0.0f64
        || yb < 0.0f64
    {
        dpx_warning(
            b"%s: Invalid cHRM chunk parameters found.\x00" as *const u8 as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        return 0 as *mut pdf_obj;
    }
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x1u32,
    ) != 0
        && png_get_gAMA(
            png_ptr as *const png_struct,
            info_ptr as *const png_info,
            &mut G,
        ) != 0
    {
        if G < 1.0e-2f64 {
            dpx_warning(
                b"%s: Unusual Gamma value: 1.0 / %g\x00" as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
                G,
            );
            return 0 as *mut pdf_obj;
        }
        G = 1.0f64 / G
    /* Gamma is inverted. */
    } else {
        G = 2.2f64
    }
    cal_param = make_param_Cal(2i32 as png_byte, G, xw, yw, xr, yr, xg, yg, xb, yb);
    if cal_param.is_null() {
        return 0 as *mut pdf_obj;
    }
    colorspace = pdf_new_array();
    pdf_add_array(
        colorspace,
        pdf_new_name(b"CalRGB\x00" as *const u8 as *const i8),
    );
    pdf_add_array(colorspace, cal_param);
    return colorspace;
}
unsafe extern "C" fn create_cspace_CalGray(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cal_param: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut xw: libc::c_double = 0.;
    let mut yw: libc::c_double = 0.;
    let mut xr: libc::c_double = 0.;
    let mut yr: libc::c_double = 0.;
    let mut xg: libc::c_double = 0.;
    let mut yg: libc::c_double = 0.;
    let mut xb: libc::c_double = 0.;
    let mut yb: libc::c_double = 0.;
    let mut G: libc::c_double = 0.;
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x4u32,
    ) == 0
        || png_get_cHRM(
            png_ptr as *const png_struct,
            info_ptr as *const png_info,
            &mut xw,
            &mut yw,
            &mut xr,
            &mut yr,
            &mut xg,
            &mut yg,
            &mut xb,
            &mut yb,
        ) == 0
    {
        return 0 as *mut pdf_obj;
    }
    if xw <= 0.0f64
        || yw < 1.0e-10f64
        || xr < 0.0f64
        || yr < 0.0f64
        || xg < 0.0f64
        || yg < 0.0f64
        || xb < 0.0f64
        || yb < 0.0f64
    {
        dpx_warning(
            b"%s: Invalid cHRM chunk parameters found.\x00" as *const u8 as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        return 0 as *mut pdf_obj;
    }
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x1u32,
    ) != 0
        && png_get_gAMA(
            png_ptr as *const png_struct,
            info_ptr as *const png_info,
            &mut G,
        ) != 0
    {
        if G < 1.0e-2f64 {
            dpx_warning(
                b"%s: Unusual Gamma value: 1.0 / %g\x00" as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
                G,
            );
            return 0 as *mut pdf_obj;
        }
        G = 1.0f64 / G
    /* Gamma is inverted. */
    } else {
        G = 2.2f64
    } /* Yw = 1.0 */
    cal_param = make_param_Cal(0i32 as png_byte, G, xw, yw, xr, yr, xg, yg, xb, yb);
    if cal_param.is_null() {
        return 0 as *mut pdf_obj;
    }
    colorspace = pdf_new_array();
    pdf_add_array(
        colorspace,
        pdf_new_name(b"CalGray\x00" as *const u8 as *const i8),
    );
    pdf_add_array(colorspace, cal_param);
    return colorspace;
}
unsafe extern "C" fn make_param_Cal(
    mut color_type: png_byte,
    mut G: libc::c_double,
    mut xw: libc::c_double,
    mut yw: libc::c_double,
    mut xr: libc::c_double,
    mut yr: libc::c_double,
    mut xg: libc::c_double,
    mut yg: libc::c_double,
    mut xb: libc::c_double,
    mut yb: libc::c_double,
) -> *mut pdf_obj {
    let mut cal_param: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut white_point: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut matrix: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dev_gamma: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut Xw: libc::c_double = 0.;
    let mut Yw: libc::c_double = 0.;
    let mut Zw: libc::c_double = 0.;
    let mut Xr: libc::c_double = 0.;
    let mut Xg: libc::c_double = 0.;
    let mut Xb: libc::c_double = 0.;
    let mut Yr: libc::c_double = 0.;
    let mut Yb: libc::c_double = 0.;
    let mut Yg: libc::c_double = 0.;
    let mut Zr: libc::c_double = 0.;
    let mut Zg: libc::c_double = 0.;
    let mut Zb: libc::c_double = 0.;
    /*
     * TODO: Check validity
     *
     * Conversion found in
     *
     *  com.sixlegs.image.png - Java package to read and display PNG images
     *  Copyright (C) 1998, 1999, 2001 Chris Nokleberg
     *
     *  http://www.sixlegs.com/software/png/
     *
     */
    let mut zw: libc::c_double = 0.;
    let mut zr: libc::c_double = 0.;
    let mut zg: libc::c_double = 0.;
    let mut zb: libc::c_double = 0.;
    let mut fr: libc::c_double = 0.;
    let mut fg: libc::c_double = 0.;
    let mut fb: libc::c_double = 0.;
    let mut det: libc::c_double = 0.;
    /* WhitePoint */
    zw = 1i32 as libc::c_double - (xw + yw);
    zr = 1i32 as libc::c_double - (xr + yr);
    zg = 1i32 as libc::c_double - (xg + yg);
    zb = 1i32 as libc::c_double - (xb + yb);
    Xw = xw / yw;
    Yw = 1.0f64;
    Zw = zw / yw;
    /* Matrix */
    det = xr * (yg * zb - zg * yb) - xg * (yr * zb - zr * yb) + xb * (yr * zg - zr * yg);
    if (if det < 0i32 as libc::c_double {
        -det
    } else {
        det
    }) < 1.0e-10f64
    {
        dpx_warning(
            b"Non invertible matrix: Maybe invalid value(s) specified in cHRM chunk.\x00"
                as *const u8 as *const i8,
        );
        return 0 as *mut pdf_obj;
    }
    fr = (Xw * (yg * zb - zg * yb) - xg * (zb - Zw * yb) + xb * (zg - Zw * yg)) / det;
    fg = (xr * (zb - Zw * yb) - Xw * (yr * zb - zr * yb) + xb * (yr * Zw - zr)) / det;
    fb = (xr * (yg * Zw - zg) - xg * (yr * Zw - zr) + Xw * (yr * zg - zr * yg)) / det;
    Xr = fr * xr;
    Yr = fr * yr;
    Zr = fr * zr;
    Xg = fg * xg;
    Yg = fg * yg;
    Zg = fg * zg;
    Xb = fb * xb;
    Yb = fb * yb;
    Zb = fb * zb;
    if G < 1.0e-2f64 {
        dpx_warning(
            b"Unusual Gamma specified: 1.0 / %g\x00" as *const u8 as *const i8,
            G,
        );
        return 0 as *mut pdf_obj;
    }
    cal_param = pdf_new_dict();
    /* White point is always required. */
    white_point = pdf_new_array();
    pdf_add_array(
        white_point,
        pdf_new_number(floor(Xw / 0.00001f64 + 0.5f64) * 0.00001f64),
    );
    pdf_add_array(
        white_point,
        pdf_new_number(floor(Yw / 0.00001f64 + 0.5f64) * 0.00001f64),
    );
    pdf_add_array(
        white_point,
        pdf_new_number(floor(Zw / 0.00001f64 + 0.5f64) * 0.00001f64),
    );
    pdf_add_dict(
        cal_param,
        pdf_new_name(b"WhitePoint\x00" as *const u8 as *const i8),
        white_point,
    );
    /* Matrix - default: Identity */
    if color_type as libc::c_int & 2i32 != 0 {
        if G != 1.0f64 {
            dev_gamma = pdf_new_array(); /* Gray */
            pdf_add_array(
                dev_gamma,
                pdf_new_number(floor(G / 0.00001f64 + 0.5f64) * 0.00001f64),
            );
            pdf_add_array(
                dev_gamma,
                pdf_new_number(floor(G / 0.00001f64 + 0.5f64) * 0.00001f64),
            );
            pdf_add_array(
                dev_gamma,
                pdf_new_number(floor(G / 0.00001f64 + 0.5f64) * 0.00001f64),
            );
            pdf_add_dict(
                cal_param,
                pdf_new_name(b"Gamma\x00" as *const u8 as *const i8),
                dev_gamma,
            );
        }
        matrix = pdf_new_array();
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Xr / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Yr / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Zr / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Xg / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Yg / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Zg / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Xb / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Yb / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Zb / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_dict(
            cal_param,
            pdf_new_name(b"Matrix\x00" as *const u8 as *const i8),
            matrix,
        );
    } else if G != 1.0f64 {
        pdf_add_dict(
            cal_param,
            pdf_new_name(b"Gamma\x00" as *const u8 as *const i8),
            pdf_new_number(floor(G / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
    }
    return cal_param;
}
/* ColorSpace */
/*
 * Set up Indexed ColorSpace for color-type PALETTE:
 *
 *  PNG allows only RGB color for base color space. If gAMA and/or cHRM
 *  chunk is available, we can use CalRGB color space instead of DeviceRGB
 *  for base color space.
 *
 */
unsafe extern "C" fn create_cspace_Indexed(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut base: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut lookup: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut data_ptr: *mut png_byte = 0 as *mut png_byte;
    let mut plte: png_colorp = 0 as *mut png_color;
    let mut num_plte: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x8u32,
    ) == 0
        || png_get_PLTE(
            png_ptr as *const png_struct,
            info_ptr,
            &mut plte,
            &mut num_plte,
        ) == 0
    {
        dpx_warning(
            b"%s: PNG does not have valid PLTE chunk.\x00" as *const u8 as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        return 0 as *mut pdf_obj;
    }
    /* Order is important. */
    colorspace = pdf_new_array();
    pdf_add_array(
        colorspace,
        pdf_new_name(b"Indexed\x00" as *const u8 as *const i8),
    );
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x1000u32,
    ) != 0
    {
        base = create_cspace_ICCBased(png_ptr, info_ptr)
    } else if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x800u32,
    ) != 0
    {
        base = create_cspace_sRGB(png_ptr, info_ptr)
    } else {
        base = create_cspace_CalRGB(png_ptr, info_ptr)
    }
    if base.is_null() {
        base = pdf_new_name(b"DeviceRGB\x00" as *const u8 as *const i8)
    }
    pdf_add_array(colorspace, base);
    pdf_add_array(
        colorspace,
        pdf_new_number((num_plte - 1i32) as libc::c_double),
    );
    data_ptr = new(((num_plte * 3i32) as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
        as u32) as *mut png_byte;
    i = 0i32;
    while i < num_plte {
        *data_ptr.offset((3i32 * i) as isize) = (*plte.offset(i as isize)).red;
        *data_ptr.offset((3i32 * i + 1i32) as isize) = (*plte.offset(i as isize)).green;
        *data_ptr.offset((3i32 * i + 2i32) as isize) = (*plte.offset(i as isize)).blue;
        i += 1
    }
    lookup = pdf_new_string(data_ptr as *const libc::c_void, (num_plte * 3i32) as size_t);
    free(data_ptr as *mut libc::c_void);
    pdf_add_array(colorspace, lookup);
    return colorspace;
}
/* Color-Key Mask */
/*
 * Colorkey Mask: array
 *
 *  [component_0_min component_0_max ... component_n_min component_n_max]
 *
 */
unsafe extern "C" fn create_ckey_mask(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorkeys: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut color_type: png_byte = 0;
    let mut trans: png_bytep = 0 as *mut png_byte;
    let mut num_trans: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut colors: png_color_16p = 0 as *mut png_color_16;
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x10u32,
    ) == 0
        || png_get_tRNS(
            png_ptr as *const png_struct,
            info_ptr,
            &mut trans,
            &mut num_trans,
            &mut colors,
        ) == 0
    {
        dpx_warning(
            b"%s: PNG does not have valid tRNS chunk!\x00" as *const u8 as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        return 0 as *mut pdf_obj;
    }
    colorkeys = pdf_new_array();
    color_type = png_get_color_type(png_ptr as *const png_struct, info_ptr as *const png_info);
    match color_type as libc::c_int {
        3 => {
            i = 0i32;
            while i < num_trans {
                if *trans.offset(i as isize) as libc::c_int == 0i32 {
                    pdf_add_array(colorkeys, pdf_new_number(i as libc::c_double));
                    pdf_add_array(colorkeys, pdf_new_number(i as libc::c_double));
                } else if *trans.offset(i as isize) as libc::c_int != 0xffi32 {
                    dpx_warning(
                        b"%s: You found a bug in pngimage.c.\x00" as *const u8
                            as *const i8,
                        b"PNG\x00" as *const u8 as *const i8,
                    );
                }
                i += 1
            }
        }
        2 => {
            pdf_add_array(colorkeys, pdf_new_number((*colors).red as libc::c_double));
            pdf_add_array(colorkeys, pdf_new_number((*colors).red as libc::c_double));
            pdf_add_array(colorkeys, pdf_new_number((*colors).green as libc::c_double));
            pdf_add_array(colorkeys, pdf_new_number((*colors).green as libc::c_double));
            pdf_add_array(colorkeys, pdf_new_number((*colors).blue as libc::c_double));
            pdf_add_array(colorkeys, pdf_new_number((*colors).blue as libc::c_double));
        }
        0 => {
            pdf_add_array(colorkeys, pdf_new_number((*colors).gray as libc::c_double));
            pdf_add_array(colorkeys, pdf_new_number((*colors).gray as libc::c_double));
        }
        _ => {
            dpx_warning(
                b"%s: You found a bug in pngimage.c.\x00" as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
            );
            pdf_release_obj(colorkeys);
            colorkeys = 0 as *mut pdf_obj
        }
    }
    return colorkeys;
}
/* Soft Mask:
 *
 * create_soft_mask() is for PNG_COLOR_TYPE_PALLETE.
 * Images with alpha chunnel use strip_soft_mask().
 * An object representing mask itself is returned.
 */
/*
 * Soft-Mask: stream
 *
 *   <<
 *      /Type             /XObject
 *      /Subtype          /Image
 *      /Width            -int-
 *      /Height           -int-
 *      /BitsPerComponent bpc
 *   >>
 *   stream .... endstream
 *
 *   ColorSpace, Mask, SMask must be absent. ImageMask must be false or absent.
 */
unsafe extern "C" fn create_soft_mask(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut image_data_ptr: png_bytep,
    mut width: png_uint_32,
    mut height: png_uint_32,
) -> *mut pdf_obj {
    let mut smask: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut smask_data_ptr: png_bytep = 0 as *mut png_byte;
    let mut trans: png_bytep = 0 as *mut png_byte;
    let mut num_trans: libc::c_int = 0;
    let mut i: png_uint_32 = 0;
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x10u32,
    ) == 0
        || png_get_tRNS(
            png_ptr as *const png_struct,
            info_ptr,
            &mut trans,
            &mut num_trans,
            0 as *mut png_color_16p,
        ) == 0
    {
        dpx_warning(
            b"%s: PNG does not have valid tRNS chunk but tRNS is requested.\x00" as *const u8
                as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        return 0 as *mut pdf_obj;
    }
    smask = pdf_new_stream(1i32 << 0i32);
    dict = pdf_stream_dict(smask);
    smask_data_ptr = new((width.wrapping_mul(height) as u64)
        .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
        as u32) as *mut png_byte;
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"XObject\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"Image\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Width\x00" as *const u8 as *const i8),
        pdf_new_number(width as libc::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Height\x00" as *const u8 as *const i8),
        pdf_new_number(height as libc::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"ColorSpace\x00" as *const u8 as *const i8),
        pdf_new_name(b"DeviceGray\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"BitsPerComponent\x00" as *const u8 as *const i8),
        pdf_new_number(8i32 as libc::c_double),
    );
    i = 0i32 as png_uint_32;
    while i < width.wrapping_mul(height) {
        let mut idx: png_byte = *image_data_ptr.offset(i as isize);
        *smask_data_ptr.offset(i as isize) = (if (idx as libc::c_int) < num_trans {
            *trans.offset(idx as isize) as libc::c_int
        } else {
            0xffi32
        }) as png_byte;
        i = i.wrapping_add(1)
    }
    pdf_add_stream(
        smask,
        smask_data_ptr as *mut i8 as *const libc::c_void,
        width.wrapping_mul(height) as libc::c_int,
    );
    free(smask_data_ptr as *mut libc::c_void);
    return smask;
}
/* bitdepth is always 8 (16 is not supported) */
unsafe extern "C" fn strip_soft_mask(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut image_data_ptr: png_bytep,
    mut rowbytes_ptr: png_uint_32p,
    mut width: png_uint_32,
    mut height: png_uint_32,
) -> *mut pdf_obj {
    let mut smask: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut dict: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut color_type: png_byte = 0;
    let mut bpc: png_byte = 0;
    let mut smask_data_ptr: png_bytep = 0 as *mut png_byte;
    let mut i: png_uint_32 = 0;
    color_type = png_get_color_type(png_ptr as *const png_struct, info_ptr as *const png_info);
    bpc = png_get_bit_depth(png_ptr as *const png_struct, info_ptr as *const png_info);
    if color_type as libc::c_int & 2i32 != 0 {
        let mut bps: libc::c_int = if bpc as libc::c_int == 8i32 {
            4i32
        } else {
            8i32
        };
        if *rowbytes_ptr as u64
            != ((bps as libc::c_uint).wrapping_mul(width) as u64)
                .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
        {
            /* Something wrong */
            dpx_warning(
                b"%s: Inconsistent rowbytes value.\x00" as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
            );
            return 0 as *mut pdf_obj;
        }
    } else {
        let mut bps_0: libc::c_int = if bpc as libc::c_int == 8i32 {
            2i32
        } else {
            4i32
        };
        if *rowbytes_ptr as u64
            != ((bps_0 as libc::c_uint).wrapping_mul(width) as u64)
                .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
        {
            /* Something wrong */
            dpx_warning(
                b"%s: Inconsistent rowbytes value.\x00" as *const u8 as *const i8,
                b"PNG\x00" as *const u8 as *const i8,
            );
            return 0 as *mut pdf_obj;
        }
    }
    smask = pdf_new_stream(1i32 << 0i32);
    dict = pdf_stream_dict(smask);
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"XObject\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"Image\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Width\x00" as *const u8 as *const i8),
        pdf_new_number(width as libc::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Height\x00" as *const u8 as *const i8),
        pdf_new_number(height as libc::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"ColorSpace\x00" as *const u8 as *const i8),
        pdf_new_name(b"DeviceGray\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"BitsPerComponent\x00" as *const u8 as *const i8),
        pdf_new_number(bpc as libc::c_double),
    );
    smask_data_ptr = new((((bpc as libc::c_int / 8i32) as libc::c_uint)
        .wrapping_mul(width)
        .wrapping_mul(height) as u64)
        .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
        as u32) as *mut png_byte;
    match color_type as libc::c_int {
        6 => {
            if bpc as libc::c_int == 8i32 {
                i = 0i32 as png_uint_32;
                while i < width.wrapping_mul(height) {
                    memmove(
                        image_data_ptr.offset((3i32 as libc::c_uint).wrapping_mul(i) as isize)
                            as *mut libc::c_void,
                        image_data_ptr.offset((4i32 as libc::c_uint).wrapping_mul(i) as isize)
                            as *const libc::c_void,
                        3i32 as u64,
                    );
                    *smask_data_ptr.offset(i as isize) = *image_data_ptr.offset(
                        (4i32 as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(3i32 as libc::c_uint) as isize,
                    );
                    i = i.wrapping_add(1)
                }
                *rowbytes_ptr = ((3i32 as libc::c_uint).wrapping_mul(width) as u64)
                    .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
                    as png_uint_32
            } else {
                i = 0i32 as png_uint_32;
                while i < width.wrapping_mul(height) {
                    memmove(
                        image_data_ptr.offset((6i32 as libc::c_uint).wrapping_mul(i) as isize)
                            as *mut libc::c_void,
                        image_data_ptr.offset((8i32 as libc::c_uint).wrapping_mul(i) as isize)
                            as *const libc::c_void,
                        6i32 as u64,
                    );
                    *smask_data_ptr.offset((2i32 as libc::c_uint).wrapping_mul(i) as isize) =
                        *image_data_ptr.offset(
                            (8i32 as libc::c_uint)
                                .wrapping_mul(i)
                                .wrapping_add(6i32 as libc::c_uint)
                                as isize,
                        );
                    *smask_data_ptr.offset(
                        (2i32 as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1i32 as libc::c_uint) as isize,
                    ) = *image_data_ptr.offset(
                        (8i32 as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(7i32 as libc::c_uint) as isize,
                    );
                    i = i.wrapping_add(1)
                }
                *rowbytes_ptr = ((6i32 as libc::c_uint).wrapping_mul(width) as u64)
                    .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
                    as png_uint_32
            }
        }
        4 => {
            if bpc as libc::c_int == 8i32 {
                i = 0i32 as png_uint_32;
                while i < width.wrapping_mul(height) {
                    *image_data_ptr.offset(i as isize) =
                        *image_data_ptr.offset((2i32 as libc::c_uint).wrapping_mul(i) as isize);
                    *smask_data_ptr.offset(i as isize) = *image_data_ptr.offset(
                        (2i32 as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1i32 as libc::c_uint) as isize,
                    );
                    i = i.wrapping_add(1)
                }
                *rowbytes_ptr = (width as u64)
                    .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
                    as png_uint_32
            } else {
                i = 0i32 as png_uint_32;
                while i < width.wrapping_mul(height) {
                    *image_data_ptr.offset((2i32 as libc::c_uint).wrapping_mul(i) as isize) =
                        *image_data_ptr.offset((4i32 as libc::c_uint).wrapping_mul(i) as isize);
                    *image_data_ptr.offset(
                        (2i32 as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1i32 as libc::c_uint) as isize,
                    ) = *image_data_ptr.offset(
                        (4i32 as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1i32 as libc::c_uint) as isize,
                    );
                    *smask_data_ptr.offset((2i32 as libc::c_uint).wrapping_mul(i) as isize) =
                        *image_data_ptr.offset(
                            (4i32 as libc::c_uint)
                                .wrapping_mul(i)
                                .wrapping_add(2i32 as libc::c_uint)
                                as isize,
                        );
                    *smask_data_ptr.offset(
                        (2i32 as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(1i32 as libc::c_uint) as isize,
                    ) = *image_data_ptr.offset(
                        (4i32 as libc::c_uint)
                            .wrapping_mul(i)
                            .wrapping_add(3i32 as libc::c_uint) as isize,
                    );
                    i = i.wrapping_add(1)
                }
                *rowbytes_ptr = ((2i32 as libc::c_uint).wrapping_mul(width) as u64)
                    .wrapping_mul(::std::mem::size_of::<png_byte>() as u64)
                    as png_uint_32
            }
        }
        _ => {
            dpx_warning(b"You found a bug in pngimage.c!\x00" as *const u8 as *const i8);
            pdf_release_obj(smask);
            free(smask_data_ptr as *mut libc::c_void);
            return 0 as *mut pdf_obj;
        }
    }
    pdf_add_stream(
        smask,
        smask_data_ptr as *const libc::c_void,
        ((bpc as libc::c_int / 8i32) as libc::c_uint)
            .wrapping_mul(width)
            .wrapping_mul(height) as libc::c_int,
    );
    free(smask_data_ptr as *mut libc::c_void);
    return smask;
}
/* Read image body */
unsafe extern "C" fn read_image_data(
    mut png_ptr: png_structp,
    mut dest_ptr: png_bytep,
    mut height: png_uint_32,
    mut rowbytes: png_uint_32,
) {
    let mut rows_p: png_bytepp = 0 as *mut *mut png_byte;
    let mut i: png_uint_32 = 0;
    rows_p = new((height as u64)
        .wrapping_mul(::std::mem::size_of::<png_bytep>() as u64)
        as u32) as *mut png_bytep as png_bytepp;
    i = 0i32 as png_uint_32;
    while i < height {
        let ref mut fresh1 = *rows_p.offset(i as isize);
        *fresh1 = dest_ptr.offset(rowbytes.wrapping_mul(i) as isize);
        i = i.wrapping_add(1)
    }
    png_read_image(png_ptr, rows_p);
    free(rows_p as *mut libc::c_void);
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
pub unsafe extern "C" fn png_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut u32,
    mut height: *mut u32,
    mut xdensity: *mut libc::c_double,
    mut ydensity: *mut libc::c_double,
) -> libc::c_int {
    let mut png_ptr: png_structp = 0 as *mut png_struct;
    let mut png_info_ptr: png_infop = 0 as *mut png_info;
    ttstub_input_seek(handle, 0i32 as ssize_t, 0i32);
    png_ptr = png_create_read_struct(
        b"1.6.37\x00" as *const u8 as *const i8,
        0 as *mut libc::c_void,
        None,
        Some(
            _png_warning_callback as unsafe extern "C" fn(_: png_structp, _: png_const_charp) -> (),
        ),
    );
    if png_ptr.is_null() || {
        png_info_ptr = png_create_info_struct(png_ptr as *const png_struct);
        png_info_ptr.is_null()
    } {
        dpx_warning(
            b"%s: Creating Libpng read/info struct failed.\x00" as *const u8 as *const i8,
            b"PNG\x00" as *const u8 as *const i8,
        );
        if !png_ptr.is_null() {
            png_destroy_read_struct(&mut png_ptr, 0 as png_infopp, 0 as png_infopp);
        }
        return -1i32;
    }
    /* Rust-backed IO */
    png_set_read_fn(
        png_ptr,
        handle,
        Some(_png_read as unsafe extern "C" fn(_: png_structp, _: png_bytep, _: png_size_t) -> ()),
    );
    /* NOTE: could use png_set_sig_bytes() to tell libpng if we started at non-zero file offset */
    /* Read PNG info-header and get some info. */
    png_read_info(png_ptr, png_info_ptr);
    *width = png_get_image_width(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    *height = png_get_image_height(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    let mut xppm: png_uint_32 = png_get_x_pixels_per_meter(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    let mut yppm: png_uint_32 = png_get_y_pixels_per_meter(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
    );
    *xdensity = if xppm != 0 {
        72.0f64 / 0.0254f64 / xppm as libc::c_double
    } else {
        1.0f64
    };
    *ydensity = if yppm != 0 {
        72.0f64 / 0.0254f64 / yppm as libc::c_double
    } else {
        1.0f64
    };
    /* Cleanup */
    if !png_info_ptr.is_null() {
        png_destroy_info_struct(png_ptr as *const png_struct, &mut png_info_ptr);
    }
    if !png_ptr.is_null() {
        png_destroy_read_struct(&mut png_ptr, 0 as png_infopp, 0 as png_infopp);
    }
    return 0i32;
}
