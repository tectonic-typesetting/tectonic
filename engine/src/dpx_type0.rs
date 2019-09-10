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
    pub type CIDFont;
    #[no_mangle]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const libc::c_void,
        stream_data_len: i32,
    );
    #[no_mangle]
    fn pdf_new_stream(flags: i32) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> i32;
    #[no_mangle]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_array() -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_new_name(name: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[no_mangle]
    fn pdf_get_version() -> libc::c_uint;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    #[no_mangle]
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: libc::c_uint,
        __function: *const i8,
    ) -> !;
    #[no_mangle]
    fn CIDFont_get_fontname(font: *mut CIDFont) -> *mut i8;
    #[no_mangle]
    fn CIDFont_get_ident(font: *mut CIDFont) -> *mut i8;
    /* FIXME */
    #[no_mangle]
    fn CIDFont_get_opt_index(font: *mut CIDFont) -> i32;
    /* FIXME */
    #[no_mangle]
    fn CIDFont_get_flag(font: *mut CIDFont, mask: i32) -> i32;
    #[no_mangle]
    fn CIDFont_get_subtype(font: *mut CIDFont) -> i32;
    #[no_mangle]
    fn CIDFont_get_embedding(font: *mut CIDFont) -> i32;
    #[no_mangle]
    fn CIDFont_get_resource(font: *mut CIDFont) -> *mut pdf_obj;
    #[no_mangle]
    fn CIDFont_get_CIDSysInfo(font: *mut CIDFont) -> *mut CIDSysInfo;
    #[no_mangle]
    fn CIDFont_attach_parent(font: *mut CIDFont, parent_id: i32, wmode: i32);
    #[no_mangle]
    fn CIDFont_get_parent_id(font: *mut CIDFont, wmode: i32) -> i32;
    #[no_mangle]
    fn CIDFont_is_ACCFont(font: *mut CIDFont) -> bool;
    #[no_mangle]
    fn CIDFont_is_UCSFont(font: *mut CIDFont) -> bool;
    #[no_mangle]
    fn CIDFont_cache_find(
        map_name: *const i8,
        cmap_csi: *mut CIDSysInfo,
        fmap_opt: *mut fontmap_opt,
    ) -> i32;
    #[no_mangle]
    fn CIDFont_cache_get(fnt_id: i32) -> *mut CIDFont;
    #[no_mangle]
    fn CIDFont_cache_close();
    #[no_mangle]
    fn CMap_is_Identity(cmap: *mut CMap) -> bool;
    #[no_mangle]
    fn CMap_get_wmode(cmap: *mut CMap) -> i32;
    #[no_mangle]
    fn CMap_get_CIDSysInfo(cmap: *mut CMap) -> *mut CIDSysInfo;
    #[no_mangle]
    fn CMap_cache_get(id: i32) -> *mut CMap;
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
    #[no_mangle]
    fn dpx_message(fmt: *const i8, _: ...);
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
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
    /* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

        Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
        the dvipdfmx project team.

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
    /* TrueType cmap table */
    /* or version, only for Mac */
    /* Paltform ID */
    /* Platform-specific encoding ID */
    /* Windows */
    /* Mac */
    /* Indirect reference */
    #[no_mangle]
    fn otf_create_ToUnicode_stream(
        map_name: *const i8,
        ttc_index: i32,
        used_chars: *const i8,
        cmap_id: i32,
    ) -> *mut pdf_obj;
    /* Just load CMap identified with 'ident'. (parsed)
     * PDF stream object (not reference) returned.
     */
    #[no_mangle]
    fn pdf_load_ToUnicode_stream(ident: *const i8) -> *mut pdf_obj;
    #[no_mangle]
    fn pdf_defineresource(
        category: *const i8,
        resname: *const i8,
        object: *mut pdf_obj,
        flags: i32,
    ) -> i32;
    #[no_mangle]
    fn pdf_findresource(category: *const i8, resname: *const i8)
        -> i32;
    #[no_mangle]
    fn pdf_get_resource_reference(res_id: i32) -> *mut pdf_obj;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Type0Font {
    pub fontname: *mut i8,
    pub encoding: *mut i8,
    pub used_chars: *mut i8,
    pub descendant: *mut CIDFont,
    pub flags: i32,
    pub wmode: i32,
    pub cmap_id: i32,
    pub indirect: *mut pdf_obj,
    pub fontdict: *mut pdf_obj,
    pub descriptor: *mut pdf_obj,
    /* _TYPE0_H_ */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_opt {
    pub slant: f64,
    pub extend: f64,
    pub bold: f64,
    pub mapc: i32,
    pub flags: i32,
    pub otl_tags: *mut i8,
    pub tounicode: *mut i8,
    pub cff_charsets: *mut libc::c_void,
    pub design_size: f64,
    pub charcoll: *mut i8,
    pub index: i32,
    pub style: i32,
    pub stemv: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut i8,
    pub ordering: *mut i8,
    pub supplement: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_cache {
    pub count: i32,
    pub capacity: i32,
    pub fonts: *mut Type0Font,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut i8,
    pub type_0: i32,
    pub wmode: i32,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_0,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: i32,
    pub profile: C2RustUnnamed,
    pub reverseMap: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut u8,
    pub prev: *mut mapData,
    pub pos: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: i32,
    pub len: size_t,
    pub code: *mut u8,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num: libc::c_uint,
    pub max: libc::c_uint,
    pub ranges: *mut rangeDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut u8,
    pub codeHi: *mut u8,
}
#[inline]
unsafe extern "C" fn streq_ptr(mut s1: *const i8, mut s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0i32;
    }
    return 0i32 != 0;
}
static mut __verbose: i32 = 0i32;
#[no_mangle]
pub unsafe extern "C" fn Type0Font_set_verbose(mut level: i32) {
    __verbose = level;
}
unsafe extern "C" fn new_used_chars2() -> *mut i8 {
    let mut used_chars: *mut i8 = 0 as *mut i8;
    used_chars = new((8192i32 as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    memset(
        used_chars as *mut libc::c_void,
        0i32,
        8192i32 as u64,
    );
    return used_chars;
}
/* MUST BE NULL */
unsafe extern "C" fn Type0Font_init_font_struct(mut font: *mut Type0Font) {
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const i8,
            b"dpx-type0.c\x00" as *const u8 as *const i8,
            104i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[i8; 45]>(
                b"void Type0Font_init_font_struct(Type0Font *)\x00",
            ))
            .as_ptr(),
        );
    }
    (*font).fontname = 0 as *mut i8;
    (*font).fontdict = 0 as *mut pdf_obj;
    (*font).indirect = 0 as *mut pdf_obj;
    (*font).descriptor = 0 as *mut pdf_obj;
    (*font).encoding = 0 as *mut i8;
    (*font).used_chars = 0 as *mut i8;
    (*font).descendant = 0 as *mut CIDFont;
    (*font).wmode = -1i32;
    (*font).cmap_id = -1i32;
    (*font).flags = 0i32;
}
unsafe extern "C" fn Type0Font_clean(mut font: *mut Type0Font) {
    if !font.is_null() {
        if !(*font).fontdict.is_null() {
            _tt_abort(
                b"%s: Object not flushed.\x00" as *const u8 as *const i8,
                b"Type0\x00" as *const u8 as *const i8,
            );
        }
        if !(*font).indirect.is_null() {
            _tt_abort(
                b"%s: Object not flushed.\x00" as *const u8 as *const i8,
                b"Type0\x00" as *const u8 as *const i8,
            );
        }
        if !(*font).descriptor.is_null() {
            _tt_abort(
                b"%s: FontDescriptor unexpected for Type0 font.\x00" as *const u8
                    as *const i8,
                b"Type0\x00" as *const u8 as *const i8,
            );
        }
        if (*font).flags & 1i32 << 0i32 == 0 && !(*font).used_chars.is_null() {
            free((*font).used_chars as *mut libc::c_void);
        }
        free((*font).encoding as *mut libc::c_void);
        free((*font).fontname as *mut libc::c_void);
        (*font).fontdict = 0 as *mut pdf_obj;
        (*font).indirect = 0 as *mut pdf_obj;
        (*font).descriptor = 0 as *mut pdf_obj;
        (*font).used_chars = 0 as *mut i8;
        (*font).encoding = 0 as *mut i8;
        (*font).fontname = 0 as *mut i8
    };
}
/* PLEASE FIX THIS */
unsafe extern "C" fn Type0Font_create_ToUnicode_stream(mut font: *mut Type0Font) -> *mut pdf_obj {
    let mut cidfont: *mut CIDFont = (*font).descendant;
    return otf_create_ToUnicode_stream(
        CIDFont_get_ident(cidfont),
        CIDFont_get_opt_index(cidfont),
        Type0Font_get_usedchars(font),
        (*font).cmap_id,
    );
}
/* Try to load ToUnicode CMap from file system first, if not found fallback to
 * font CMap reverse lookup. */
unsafe extern "C" fn Type0Font_try_load_ToUnicode_stream(
    mut font: *mut Type0Font,
    mut cmap_base: *mut i8,
) -> *mut pdf_obj {
    let mut cmap_name: *mut i8 = new((strlen(cmap_base)
        .wrapping_add(strlen(b"-UTF-16\x00" as *const u8 as *const i8))
        as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<i8>() as u64)
        as u32) as *mut i8;
    let mut tounicode: *mut pdf_obj = 0 as *mut pdf_obj;
    sprintf(
        cmap_name,
        b"%s-UTF16\x00" as *const u8 as *const i8,
        cmap_base,
    );
    tounicode = pdf_read_ToUnicode_file(cmap_name);
    if tounicode.is_null() {
        sprintf(
            cmap_name,
            b"%s-UCS2\x00" as *const u8 as *const i8,
            cmap_base,
        );
        tounicode = pdf_read_ToUnicode_file(cmap_name)
    }
    free(cmap_name as *mut libc::c_void);
    if tounicode.is_null() {
        tounicode = Type0Font_create_ToUnicode_stream(font)
    }
    return tounicode;
}
unsafe extern "C" fn add_ToUnicode(mut font: *mut Type0Font) {
    let mut tounicode: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut cidfont: *mut CIDFont = 0 as *mut CIDFont;
    let mut csi: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
    let mut fontname: *mut i8 = 0 as *mut i8;
    /*
     * ToUnicode CMap:
     *
     *  ToUnicode CMaps are usually not required for standard character
     *  collections such as Adobe-Japan1. Identity-H is used for UCS
     *  ordering CID-keyed fonts. External resource must be loaded for
     *  others.
     */
    cidfont = (*font).descendant;
    if cidfont.is_null() {
        _tt_abort(
            b"%s: No descendant CID-keyed font.\x00" as *const u8 as *const i8,
            b"Type0\x00" as *const u8 as *const i8,
        );
    }
    if CIDFont_is_ACCFont(cidfont) {
        /* No need to embed ToUnicode */
        return;
    } else {
        if CIDFont_is_UCSFont(cidfont) {
            /*
             * Old version of dvipdfmx mistakenly used Adobe-Identity as Unicode.
             */
            tounicode = pdf_read_ToUnicode_file(
                b"Adobe-Identity-UCS2\x00" as *const u8 as *const i8,
            );
            if tounicode.is_null() {
                /* This should work */
                tounicode = pdf_new_name(b"Identity-H\x00" as *const u8 as *const i8)
            }
            pdf_add_dict(
                (*font).fontdict,
                pdf_new_name(b"ToUnicode\x00" as *const u8 as *const i8),
                tounicode,
            );
            return;
        }
    }
    tounicode = 0 as *mut pdf_obj;
    csi = CIDFont_get_CIDSysInfo(cidfont);
    fontname = CIDFont_get_fontname(cidfont);
    if CIDFont_get_embedding(cidfont) != 0 {
        fontname = fontname.offset(7)
        /* FIXME */
    }
    if streq_ptr(
        (*csi).registry,
        b"Adobe\x00" as *const u8 as *const i8,
    ) as i32
        != 0
        && streq_ptr(
            (*csi).ordering,
            b"Identity\x00" as *const u8 as *const i8,
        ) as i32
            != 0
    {
        match CIDFont_get_subtype(cidfont) {
            2 => {
                /* PLEASE FIX THIS */
                tounicode = Type0Font_create_ToUnicode_stream(font)
            }
            _ => {
                if CIDFont_get_flag(cidfont, 1i32 << 9i32) != 0 {
                    /* FIXME */
                    tounicode = Type0Font_create_ToUnicode_stream(font)
                } else if CIDFont_get_flag(cidfont, 1i32 << 8i32) != 0 {
                    /* FIXME */
                    /* Font loader will create ToUnicode and set. */
                    return;
                } else {
                    tounicode = Type0Font_try_load_ToUnicode_stream(font, fontname)
                }
            }
        }
    } else {
        let mut cmap_base: *mut i8 = new((strlen((*csi).registry)
            .wrapping_add(strlen((*csi).ordering))
            .wrapping_add(2i32 as u64)
            as u32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64)
            as u32) as *mut i8;
        sprintf(
            cmap_base,
            b"%s-%s\x00" as *const u8 as *const i8,
            (*csi).registry,
            (*csi).ordering,
        );
        tounicode = Type0Font_try_load_ToUnicode_stream(font, cmap_base);
        free(cmap_base as *mut libc::c_void);
    }
    if !tounicode.is_null() {
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"ToUnicode\x00" as *const u8 as *const i8),
            tounicode,
        );
    } else {
        dpx_warning(
            b"Failed to load ToUnicode CMap for font \"%s\"\x00" as *const u8
                as *const i8,
            fontname,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Type0Font_set_ToUnicode(
    mut font: *mut Type0Font,
    mut cmap_ref: *mut pdf_obj,
) {
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const i8,
            b"dpx-type0.c\x00" as *const u8 as *const i8,
            259i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[i8; 53]>(
                b"void Type0Font_set_ToUnicode(Type0Font *, pdf_obj *)\x00",
            ))
            .as_ptr(),
        );
    }
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"ToUnicode\x00" as *const u8 as *const i8),
        cmap_ref,
    );
}
unsafe extern "C" fn Type0Font_dofont(mut font: *mut Type0Font) {
    if font.is_null() || (*font).indirect.is_null() {
        return;
    }
    if pdf_lookup_dict(
        (*font).fontdict,
        b"ToUnicode\x00" as *const u8 as *const i8,
    )
    .is_null()
    {
        /* FIXME */
        add_ToUnicode(font);
    };
}
unsafe extern "C" fn Type0Font_flush(mut font: *mut Type0Font) {
    if !font.is_null() {
        pdf_release_obj((*font).fontdict);
        (*font).fontdict = 0 as *mut pdf_obj;
        pdf_release_obj((*font).indirect);
        (*font).indirect = 0 as *mut pdf_obj;
        if !(*font).descriptor.is_null() {
            _tt_abort(
                b"%s: FontDescriptor unexpected for Type0 font.\x00" as *const u8
                    as *const i8,
                b"Type0\x00" as *const u8 as *const i8,
            );
        }
        (*font).descriptor = 0 as *mut pdf_obj
    };
}
#[no_mangle]
pub unsafe extern "C" fn Type0Font_get_wmode(mut font: *mut Type0Font) -> i32 {
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const i8,
            b"dpx-type0.c\x00" as *const u8 as *const i8,
            293i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 37], &[i8; 37]>(
                b"int Type0Font_get_wmode(Type0Font *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*font).wmode;
}
#[no_mangle]
pub unsafe extern "C" fn Type0Font_get_usedchars(mut font: *mut Type0Font) -> *mut i8 {
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const i8,
            b"dpx-type0.c\x00" as *const u8 as *const i8,
            301i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 43], &[i8; 43]>(
                b"char *Type0Font_get_usedchars(Type0Font *)\x00",
            ))
            .as_ptr(),
        );
    }
    return (*font).used_chars;
}
#[no_mangle]
pub unsafe extern "C" fn Type0Font_get_resource(mut font: *mut Type0Font) -> *mut pdf_obj {
    if !font.is_null() {
    } else {
        __assert_fail(
            b"font\x00" as *const u8 as *const i8,
            b"dpx-type0.c\x00" as *const u8 as *const i8,
            309i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 45], &[i8; 45]>(
                b"pdf_obj *Type0Font_get_resource(Type0Font *)\x00",
            ))
            .as_ptr(),
        );
    }
    /*
     * This looks somewhat strange.
     */
    if (*font).indirect.is_null() {
        let mut array: *mut pdf_obj = 0 as *mut pdf_obj;
        array = pdf_new_array();
        pdf_add_array(array, CIDFont_get_resource((*font).descendant));
        pdf_add_dict(
            (*font).fontdict,
            pdf_new_name(b"DescendantFonts\x00" as *const u8 as *const i8),
            array,
        );
        (*font).indirect = pdf_ref_obj((*font).fontdict)
    }
    return pdf_link_obj((*font).indirect);
}
static mut __cache: font_cache = {
    let mut init = font_cache {
        count: 0i32,
        capacity: 0i32,
        fonts: 0 as *const Type0Font as *mut Type0Font,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn Type0Font_cache_init() {
    if !__cache.fonts.is_null() {
        _tt_abort(
            b"%s: Already initialized.\x00" as *const u8 as *const i8,
            b"Type0\x00" as *const u8 as *const i8,
        );
    }
    __cache.count = 0i32;
    __cache.capacity = 0i32;
    __cache.fonts = 0 as *mut Type0Font;
}
#[no_mangle]
pub unsafe extern "C" fn Type0Font_cache_get(mut id: i32) -> *mut Type0Font {
    if id < 0i32 || id >= __cache.count {
        _tt_abort(
            b"%s: Invalid ID %d\x00" as *const u8 as *const i8,
            b"Type0\x00" as *const u8 as *const i8,
            id,
        );
    }
    return &mut *__cache.fonts.offset(id as isize) as *mut Type0Font;
}
#[no_mangle]
pub unsafe extern "C" fn Type0Font_cache_find(
    mut map_name: *const i8,
    mut cmap_id: i32,
    mut fmap_opt: *mut fontmap_opt,
) -> i32 {
    let mut font_id: i32 = -1i32;
    let mut font: *mut Type0Font = 0 as *mut Type0Font;
    let mut cidfont: *mut CIDFont = 0 as *mut CIDFont;
    let mut cmap: *mut CMap = 0 as *mut CMap;
    let mut csi: *mut CIDSysInfo = 0 as *mut CIDSysInfo;
    let mut fontname: *mut i8 = 0 as *mut i8;
    let mut cid_id: i32 = -1i32;
    let mut parent_id: i32 = -1i32;
    let mut wmode: i32 = 0i32;
    let mut pdf_ver: i32 = 0;
    pdf_ver = pdf_get_version() as i32;
    if map_name.is_null() || cmap_id < 0i32 || pdf_ver < 2i32 {
        return -1i32;
    }
    /*
     * Encoding is Identity-H or Identity-V according as thier WMode value.
     *
     * We do not use match against the map_name since fonts (TrueType) covers
     * characters across multiple character collection (eg, Adobe-Japan1 and
     * Adobe-Japan2) must be splited into multiple CID-keyed fonts.
     */
    cmap = CMap_cache_get(cmap_id);
    csi = if CMap_is_Identity(cmap) as i32 != 0 {
        0 as *mut CIDSysInfo
    } else {
        CMap_get_CIDSysInfo(cmap)
    };
    cid_id = CIDFont_cache_find(map_name, csi, fmap_opt);
    if cid_id < 0i32 {
        return -1i32;
    }
    /*
     * The descendant CID-keyed font has already been registerd.
     * If CID-keyed font with ID = cid_id is new font, then create new parent
     * Type 0 font. Otherwise, there already exists parent Type 0 font and
     * then we find him and return his ID. We must check against their WMode.
     */
    cidfont = CIDFont_cache_get(cid_id);
    wmode = CMap_get_wmode(cmap);
    /* Does CID-keyed font already have parent ? */
    parent_id = CIDFont_get_parent_id(cidfont, wmode); /* If so, we don't need new one. */
    if parent_id >= 0i32 {
        return parent_id;
    }
    /*
     * CIDFont does not have parent or his parent's WMode does not matched with
     * wmode. Create new Type0 font.
     */
    if __cache.count >= __cache.capacity {
        __cache.capacity =
            (__cache.capacity as libc::c_uint).wrapping_add(16u32) as i32 as i32;
        __cache.fonts = renew(
            __cache.fonts as *mut libc::c_void,
            (__cache.capacity as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<Type0Font>() as u64)
                as u32,
        ) as *mut Type0Font
    }
    font_id = __cache.count;
    font = &mut *__cache.fonts.offset(font_id as isize) as *mut Type0Font;
    Type0Font_init_font_struct(font);
    /*
     * All CJK double-byte characters are mapped so that resulting
     * character codes coincide with CIDs of given character collection.
     * So, the Encoding is always Identity-H for horizontal fonts or
     * Identity-V for vertical fonts.
     */
    if wmode != 0 {
        (*font).encoding = new(
            (strlen(b"Identity-V\x00" as *const u8 as *const i8)
                .wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32,
        ) as *mut i8;
        strcpy(
            (*font).encoding,
            b"Identity-V\x00" as *const u8 as *const i8,
        );
    } else {
        (*font).encoding = new(
            (strlen(b"Identity-H\x00" as *const u8 as *const i8)
                .wrapping_add(1i32 as u64) as u32 as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32,
        ) as *mut i8;
        strcpy(
            (*font).encoding,
            b"Identity-H\x00" as *const u8 as *const i8,
        );
    }
    (*font).wmode = wmode;
    (*font).cmap_id = cmap_id;
    /*
     * Now we start font dictionary.
     */
    (*font).fontdict = pdf_new_dict();
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"Type\x00" as *const u8 as *const i8),
        pdf_new_name(b"Font\x00" as *const u8 as *const i8),
    );
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"Subtype\x00" as *const u8 as *const i8),
        pdf_new_name(b"Type0\x00" as *const u8 as *const i8),
    );
    /*
     * Type0 font does not have FontDescriptor because it is not a simple font.
     * Instead, DescendantFonts appears here.
     *
     * Up to PDF version 1.5, Type0 font must have single descendant font which
     * is a CID-keyed font. Future PDF spec. will allow multiple desecendant
     * fonts.
     */
    (*font).descendant = cidfont;
    CIDFont_attach_parent(cidfont, font_id, wmode);
    /*
     * PostScript Font name:
     *
     *  Type0 font's fontname is usually descendant CID-keyed font's font name
     *  appended by -ENCODING.
     */
    fontname = CIDFont_get_fontname(cidfont); /* skip XXXXXX+ */
    if __verbose != 0 {
        if CIDFont_get_embedding(cidfont) != 0 && strlen(fontname) > 7i32 as u64 {
            dpx_message(
                b"(CID:%s)\x00" as *const u8 as *const i8,
                fontname.offset(7),
            );
        } else {
            dpx_message(
                b"(CID:%s)\x00" as *const u8 as *const i8,
                fontname,
            );
        }
    }
    /*
     * The difference between CID-keyed font and TrueType font appears here.
     *
     * Glyph substitution for vertical writing is done in CMap mapping process
     * for CID-keyed fonts. But we must rely on OpenType layout table in the
     * case of TrueType fonts. So, we must use different used_chars for each
     * horizontal and vertical fonts in that case.
     *
     * In most PDF file, encoding name is not appended to fontname for Type0
     * fonts having CIDFontType 2 font as their descendant.
     */
    (*font).used_chars = 0 as *mut i8;
    (*font).flags = 0i32;
    match CIDFont_get_subtype(cidfont) {
        1 => {
            (*font).fontname = new((strlen(fontname)
                .wrapping_add(strlen((*font).encoding))
                .wrapping_add(2i32 as u64) as u32
                as u64)
                .wrapping_mul(::std::mem::size_of::<i8>() as u64)
                as u32) as *mut i8;
            sprintf(
                (*font).fontname,
                b"%s-%s\x00" as *const u8 as *const i8,
                fontname,
                (*font).encoding,
            );
            pdf_add_dict(
                (*font).fontdict,
                pdf_new_name(b"BaseFont\x00" as *const u8 as *const i8),
                pdf_new_name((*font).fontname),
            );
            /*
             * Need used_chars to write W, W2.
             */
            parent_id = CIDFont_get_parent_id(cidfont, if wmode != 0 { 0i32 } else { 1i32 });
            if parent_id < 0i32 {
                (*font).used_chars = new_used_chars2()
            } else {
                /* Don't allocate new one. */
                (*font).used_chars = Type0Font_get_usedchars(Type0Font_cache_get(parent_id));
                (*font).flags |= 1i32 << 0i32
            }
        }
        2 => {
            /*
             * TrueType:
             *
             *  Use different used_chars for H and V.
             */
            pdf_add_dict(
                (*font).fontdict,
                pdf_new_name(b"BaseFont\x00" as *const u8 as *const i8),
                pdf_new_name(fontname),
            );
            (*font).used_chars = new_used_chars2()
        }
        _ => {
            _tt_abort(b"Unrecognized CIDFont Type\x00" as *const u8 as *const i8);
        }
    }
    pdf_add_dict(
        (*font).fontdict,
        pdf_new_name(b"Encoding\x00" as *const u8 as *const i8),
        pdf_new_name((*font).encoding),
    );
    __cache.count += 1;
    return font_id;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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
/* ******************************* CACHE ********************************/
#[no_mangle]
pub unsafe extern "C" fn Type0Font_cache_close() {
    let mut font_id: i32 = 0;
    /*
     * This need to be fixed.
     *
     * CIDFont_cache_close() before Type0Font_release because of used_chars.
     * ToUnicode support want descendant CIDFont's CSI and fontname.
     */
    if !__cache.fonts.is_null() {
        font_id = 0i32;
        while font_id < __cache.count {
            Type0Font_dofont(&mut *__cache.fonts.offset(font_id as isize));
            font_id += 1
        }
    }
    CIDFont_cache_close();
    if !__cache.fonts.is_null() {
        font_id = 0i32;
        while font_id < __cache.count {
            Type0Font_flush(&mut *__cache.fonts.offset(font_id as isize));
            Type0Font_clean(&mut *__cache.fonts.offset(font_id as isize));
            font_id += 1
        }
        free(__cache.fonts as *mut libc::c_void);
    }
    __cache.fonts = 0 as *mut Type0Font;
    __cache.count = 0i32;
    __cache.capacity = 0i32;
}
/* ******************************* COMPAT ********************************/
unsafe extern "C" fn create_dummy_CMap() -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut buf: [i8; 32] = [0; 32];
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    stream = pdf_new_stream(1i32 << 0i32);
    pdf_add_stream(stream,
                   b"%!PS-Adobe-3.0 Resource-CMap\n%%DocumentNeededResources: ProcSet (CIDInit)\n%%IncludeResource: ProcSet (CIDInit)\n%%BeginResource: CMap (Adobe-Identity-UCS2)\n%%Title: (Adobe-Identity-UCS2 Adobe UCS2 0)\n%%Version: 1.0\n%%Copyright:\n%% ---\n%%EndComments\n\n\x00"
                       as *const u8 as *const i8 as
                       *const libc::c_void,
                   strlen(b"%!PS-Adobe-3.0 Resource-CMap\n%%DocumentNeededResources: ProcSet (CIDInit)\n%%IncludeResource: ProcSet (CIDInit)\n%%BeginResource: CMap (Adobe-Identity-UCS2)\n%%Title: (Adobe-Identity-UCS2 Adobe UCS2 0)\n%%Version: 1.0\n%%Copyright:\n%% ---\n%%EndComments\n\n\x00"
                              as *const u8 as *const i8) as
                       i32);
    pdf_add_stream(stream,
                   b"/CIDInit /ProcSet findresource begin\n\n12 dict begin\n\nbegincmap\n\n/CIDSystemInfo 3 dict dup begin\n  /Registry (Adobe) def\n  /Ordering (UCS2) def\n  /Supplement 0 def\nend def\n\n/CMapName /Adobe-Identity-UCS2 def\n/CMapVersion 1.0 def\n/CMapType 2 def\n\n2 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n\x00"
                       as *const u8 as *const i8 as
                       *const libc::c_void,
                   strlen(b"/CIDInit /ProcSet findresource begin\n\n12 dict begin\n\nbegincmap\n\n/CIDSystemInfo 3 dict dup begin\n  /Registry (Adobe) def\n  /Ordering (UCS2) def\n  /Supplement 0 def\nend def\n\n/CMapName /Adobe-Identity-UCS2 def\n/CMapVersion 1.0 def\n/CMapType 2 def\n\n2 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n\x00"
                              as *const u8 as *const i8) as
                       i32);
    pdf_add_stream(
        stream,
        b"\n100 beginbfrange\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"\n100 beginbfrange\n\x00" as *const u8 as *const i8) as i32,
    );
    i = 0i32;
    while i < 0x64i32 {
        n = sprintf(
            buf.as_mut_ptr(),
            b"<%02X00> <%02XFF> <%02X00>\n\x00" as *const u8 as *const i8,
            i,
            i,
            i,
        );
        pdf_add_stream(stream, buf.as_mut_ptr() as *const libc::c_void, n);
        i += 1
    }
    pdf_add_stream(
        stream,
        b"endbfrange\n\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"endbfrange\n\n\x00" as *const u8 as *const i8) as i32,
    );
    pdf_add_stream(
        stream,
        b"\n100 beginbfrange\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"\n100 beginbfrange\n\x00" as *const u8 as *const i8) as i32,
    );
    i = 0x64i32;
    while i < 0xc8i32 {
        n = sprintf(
            buf.as_mut_ptr(),
            b"<%02X00> <%02XFF> <%02X00>\n\x00" as *const u8 as *const i8,
            i,
            i,
            i,
        );
        pdf_add_stream(stream, buf.as_mut_ptr() as *const libc::c_void, n);
        i += 1
    }
    pdf_add_stream(
        stream,
        b"endbfrange\n\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"endbfrange\n\n\x00" as *const u8 as *const i8) as i32,
    );
    pdf_add_stream(
        stream,
        b"\n48 beginbfrange\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"\n48 beginbfrange\n\x00" as *const u8 as *const i8) as i32,
    );
    i = 0xc8i32;
    while i <= 0xd7i32 {
        n = sprintf(
            buf.as_mut_ptr(),
            b"<%02X00> <%02XFF> <%02X00>\n\x00" as *const u8 as *const i8,
            i,
            i,
            i,
        );
        pdf_add_stream(stream, buf.as_mut_ptr() as *const libc::c_void, n);
        i += 1
    }
    i = 0xe0i32;
    while i <= 0xffi32 {
        n = sprintf(
            buf.as_mut_ptr(),
            b"<%02X00> <%02XFF> <%02X00>\n\x00" as *const u8 as *const i8,
            i,
            i,
            i,
        );
        pdf_add_stream(stream, buf.as_mut_ptr() as *const libc::c_void, n);
        i += 1
    }
    pdf_add_stream(
        stream,
        b"endbfrange\n\n\x00" as *const u8 as *const i8 as *const libc::c_void,
        strlen(b"endbfrange\n\n\x00" as *const u8 as *const i8) as i32,
    );
    pdf_add_stream(stream,
                   b"endcmap\n\nCMapName currentdict /CMap defineresource pop\n\nend\nend\n\n%%EndResource\n%%EOF\n\x00"
                       as *const u8 as *const i8 as
                       *const libc::c_void,
                   strlen(b"endcmap\n\nCMapName currentdict /CMap defineresource pop\n\nend\nend\n\n%%EndResource\n%%EOF\n\x00"
                              as *const u8 as *const i8) as
                       i32);
    return stream;
}
unsafe extern "C" fn pdf_read_ToUnicode_file(mut cmap_name: *const i8) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut res_id: i32 = -1i32;
    if !cmap_name.is_null() {
    } else {
        __assert_fail(
            b"cmap_name\x00" as *const u8 as *const i8,
            b"dpx-type0.c\x00" as *const u8 as *const i8,
            646i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 47], &[i8; 47]>(
                b"pdf_obj *pdf_read_ToUnicode_file(const char *)\x00",
            ))
            .as_ptr(),
        );
    }
    res_id = pdf_findresource(b"CMap\x00" as *const u8 as *const i8, cmap_name);
    if res_id < 0i32 {
        if streq_ptr(
            cmap_name,
            b"Adobe-Identity-UCS2\x00" as *const u8 as *const i8,
        ) {
            stream = create_dummy_CMap()
        } else {
            stream = pdf_load_ToUnicode_stream(cmap_name)
        }
        if !stream.is_null() {
            res_id = pdf_defineresource(
                b"CMap\x00" as *const u8 as *const i8,
                cmap_name,
                stream,
                1i32,
            )
        }
    }
    return if res_id < 0i32 {
        0 as *mut pdf_obj
    } else {
        pdf_get_resource_reference(res_id)
    };
}
/* !WITHOUT_COMPAT */
