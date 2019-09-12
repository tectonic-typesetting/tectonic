#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::warn;

extern crate libc;
use crate::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_link_obj, pdf_new_array, pdf_new_dict, pdf_new_name,
    pdf_new_null, pdf_new_string, pdf_obj, pdf_obj_typeof, pdf_ref_obj, pdf_release_obj,
    pdf_string_length, pdf_string_value,
};
use libc::free;
extern "C" {
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[no_mangle]
    fn ht_clear_table(ht: *mut ht_table);
    #[no_mangle]
    fn ht_lookup_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn ht_append_table(
        ht: *mut ht_table,
        key: *const libc::c_void,
        keylen: i32,
        value: *mut libc::c_void,
    );
    #[no_mangle]
    fn ht_set_iter(ht: *mut ht_table, iter: *mut ht_iter) -> i32;
    #[no_mangle]
    fn ht_clear_iter(iter: *mut ht_iter);
    #[no_mangle]
    fn ht_iter_getkey(iter: *mut ht_iter, keylen: *mut i32) -> *mut i8;
    #[no_mangle]
    fn ht_iter_getval(iter: *mut ht_iter) -> *mut libc::c_void;
    #[no_mangle]
    fn ht_iter_next(iter: *mut ht_iter) -> i32;
    #[no_mangle]
    fn pdf_transfer_label(dst: *mut pdf_obj, src: *mut pdf_obj);
    #[no_mangle]
    fn pdf_new_undefined() -> *mut pdf_obj;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
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
    fn renew(p: *mut libc::c_void, size: u32) -> *mut libc::c_void;
}
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = u64;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut i8,
    pub keylen: i32,
    pub value: *mut libc::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: i32,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_iter {
    pub index: i32,
    pub curr: *mut libc::c_void,
    pub hash: *mut ht_table,
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
/* Hash */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obj_data {
    pub object: *mut pdf_obj,
    pub closed: i32,
    /* 1 if object is closed */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct named_object {
    pub key: *mut i8,
    pub keylen: i32,
    pub value: *mut pdf_obj,
}
#[inline]
unsafe extern "C" fn mfree(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    free(ptr);
    0 as *mut libc::c_void
}
unsafe extern "C" fn printable_key(mut key: *const i8, mut keylen: i32) -> *mut i8 {
    static mut pkey: [i8; 36] = [0; 36];
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut hi: u8 = 0;
    let mut lo: u8 = 0;
    i = 0i32;
    len = 0i32;
    while i < keylen && len < 32i32 {
        if libc::isprint(*key.offset(i as isize) as _) != 0 {
            let fresh0 = len;
            len = len + 1;
            pkey[fresh0 as usize] = *key.offset(i as isize)
        } else {
            hi = (*key.offset(i as isize) as i32 >> 4i32 & 0xffi32) as u8;
            lo = (*key.offset(i as isize) as i32 & 0xffi32) as u8;
            let fresh1 = len;
            len = len + 1;
            pkey[fresh1 as usize] = '#' as i32 as i8;
            let fresh2 = len;
            len = len + 1;
            pkey[fresh2 as usize] = (if (hi as i32) < 10i32 {
                hi as i32 + '0' as i32
            } else {
                hi as i32 - 10i32 + 'A' as i32
            }) as i8;
            let fresh3 = len;
            len = len + 1;
            pkey[fresh3 as usize] = (if (lo as i32) < 10i32 {
                lo as i32 + '0' as i32
            } else {
                lo as i32 - 10i32 + 'A' as i32
            }) as i8
        }
        i += 1
    }
    pkey[len as usize] = '\u{0}' as i32 as i8;
    pkey.as_mut_ptr()
}
#[inline]
unsafe extern "C" fn hval_free(mut hval: *mut libc::c_void) {
    let mut value: *mut obj_data = 0 as *mut obj_data;
    value = hval as *mut obj_data;
    if !(*value).object.is_null() {
        pdf_release_obj((*value).object);
        (*value).object = 0 as *mut pdf_obj
    }
    free(value as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pdf_new_name_tree() -> *mut ht_table {
    let mut names: *mut ht_table = 0 as *mut ht_table;
    names =
        new((1_u64).wrapping_mul(::std::mem::size_of::<ht_table>() as u64) as u32) as *mut ht_table;
    ht_init_table(
        names,
        Some(hval_free as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
    names
}
unsafe extern "C" fn check_objects_defined(mut ht_tab: *mut ht_table) {
    let mut iter: ht_iter = ht_iter {
        index: 0,
        curr: 0 as *mut libc::c_void,
        hash: 0 as *mut ht_table,
    };
    if ht_set_iter(ht_tab, &mut iter) >= 0i32 {
        loop {
            let mut key: *mut i8 = 0 as *mut i8;
            let mut keylen: i32 = 0;
            let mut value: *mut obj_data = 0 as *mut obj_data;
            key = ht_iter_getkey(&mut iter, &mut keylen);
            value = ht_iter_getval(&mut iter) as *mut obj_data;
            assert!(!(*value).object.is_null());
            if !(*value).object.is_null() && pdf_obj_typeof((*value).object) == 10i32 {
                pdf_names_add_object(ht_tab, key as *const libc::c_void, keylen, pdf_new_null());
                dpx_warning(
                    b"Object @%s used, but not defined. Replaced by null.\x00" as *const u8
                        as *const i8,
                    printable_key(key, keylen),
                );
            }
            if !(ht_iter_next(&mut iter) >= 0i32) {
                break;
            }
        }
        ht_clear_iter(&mut iter);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pdf_delete_name_tree(mut names: *mut *mut ht_table) {
    assert!(!names.is_null() && !(*names).is_null());
    check_objects_defined(*names);
    ht_clear_table(*names);
    *names = mfree(*names as *mut libc::c_void) as *mut ht_table;
}
#[no_mangle]
pub unsafe extern "C" fn pdf_names_add_object(
    mut names: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: i32,
    mut object: *mut pdf_obj,
) -> i32 {
    let mut value: *mut obj_data = 0 as *mut obj_data;
    assert!(!names.is_null() && !object.is_null());
    if key.is_null() || keylen < 1i32 {
        warn!("Null string used for name tree key.");
        return -1i32;
    }
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if value.is_null() {
        value = new((1_u64).wrapping_mul(::std::mem::size_of::<obj_data>() as u64) as u32)
            as *mut obj_data;
        (*value).object = object;
        (*value).closed = 0i32;
        ht_append_table(names, key, keylen, value as *mut libc::c_void);
    } else {
        assert!(!(*value).object.is_null());
        if !(*value).object.is_null() && pdf_obj_typeof((*value).object) == 10i32 {
            pdf_transfer_label(object, (*value).object);
            pdf_release_obj((*value).object);
            (*value).object = object
        } else {
            dpx_warning(
                b"Object @%s already defined.\x00" as *const u8 as *const i8,
                printable_key(key as *const i8, keylen),
            );
            pdf_release_obj(object);
            return -1i32;
        }
    }
    0i32
}
/*
 * The following routine returns copies, not the original object.
 */
#[no_mangle]
pub unsafe extern "C" fn pdf_names_lookup_reference(
    mut names: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: i32,
) -> *mut pdf_obj {
    let mut value: *mut obj_data = 0 as *mut obj_data;
    let mut object: *mut pdf_obj = 0 as *mut pdf_obj;
    assert!(!names.is_null());
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if !value.is_null() {
        object = (*value).object;
        assert!(!object.is_null());
    } else {
        /* A null object as dummy would create problems because as value
         * of a dictionary entry, a null object is be equivalent to no entry
         * at all. This matters for optimization of PDF destinations.
         */
        object = pdf_new_undefined();
        pdf_names_add_object(names, key, keylen, object);
    }
    pdf_ref_obj(object)
}
#[no_mangle]
pub unsafe extern "C" fn pdf_names_lookup_object(
    mut names: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: i32,
) -> *mut pdf_obj {
    let mut value: *mut obj_data = 0 as *mut obj_data;
    assert!(!names.is_null());
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if value.is_null() || !(*value).object.is_null() && pdf_obj_typeof((*value).object) == 10i32 {
        return 0 as *mut pdf_obj;
    }
    assert!(!(*value).object.is_null());
    (*value).object
}
#[no_mangle]
pub unsafe extern "C" fn pdf_names_close_object(
    mut names: *mut ht_table,
    mut key: *const libc::c_void,
    mut keylen: i32,
) -> i32 {
    let mut value: *mut obj_data = 0 as *mut obj_data;
    assert!(!names.is_null());
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if value.is_null() || !(*value).object.is_null() && pdf_obj_typeof((*value).object) == 10i32 {
        dpx_warning(
            b"Cannot close undefined object @%s.\x00" as *const u8 as *const i8,
            printable_key(key as *const i8, keylen),
        );
        return -1i32;
    }
    assert!(!(*value).object.is_null());
    if (*value).closed != 0 {
        dpx_warning(
            b"Object @%s already closed.\x00" as *const u8 as *const i8,
            printable_key(key as *const i8, keylen),
        );
        return -1i32;
    }
    (*value).closed = 1i32;
    0i32
}
#[inline]
unsafe extern "C" fn cmp_key(mut d1: *const libc::c_void, mut d2: *const libc::c_void) -> i32 {
    let mut sd1: *const named_object = 0 as *const named_object;
    let mut sd2: *const named_object = 0 as *const named_object;
    let mut keylen: i32 = 0;
    let mut cmp: i32 = 0;
    sd1 = d1 as *const named_object;
    sd2 = d2 as *const named_object;
    if (*sd1).key.is_null() {
        cmp = -1i32
    } else if (*sd2).key.is_null() {
        cmp = 1i32
    } else {
        keylen = if (*sd1).keylen < (*sd2).keylen {
            (*sd1).keylen
        } else {
            (*sd2).keylen
        };
        cmp = memcmp(
            (*sd1).key as *const libc::c_void,
            (*sd2).key as *const libc::c_void,
            keylen as u64,
        );
        if cmp == 0 {
            cmp = (*sd1).keylen - (*sd2).keylen
        }
    }
    cmp
}
unsafe extern "C" fn build_name_tree(
    mut first: *mut named_object,
    mut num_leaves: i32,
    mut is_root: i32,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut i: i32 = 0;
    result = pdf_new_dict();
    /*
     * According to PDF Refrence, Third Edition (p.101-102), a name tree
     * always has exactly one root node, which contains a SINGLE entry:
     * either Kids or Names but not both. If the root node has a Names
     * entry, it is the only node in the tree. If it has a Kids entry,
     * then each of the remaining nodes is either an intermediate node,
     * containing a Limits entry and a Kids entry, or a leaf node,
     * containing a Limits entry and a Names entry.
     */
    if is_root == 0 {
        let mut last: *mut named_object = 0 as *mut named_object;
        let mut limits: *mut pdf_obj = 0 as *mut pdf_obj;
        limits = pdf_new_array();
        last = &mut *first.offset((num_leaves - 1i32) as isize) as *mut named_object;
        pdf_add_array(
            limits,
            pdf_new_string(
                (*first).key as *const libc::c_void,
                (*first).keylen as size_t,
            ),
        );
        pdf_add_array(
            limits,
            pdf_new_string((*last).key as *const libc::c_void, (*last).keylen as size_t),
        );
        pdf_add_dict(
            result,
            pdf_new_name(b"Limits\x00" as *const u8 as *const i8),
            limits,
        );
    }
    if num_leaves > 0i32 && num_leaves <= 2i32 * 4i32 {
        let mut names: *mut pdf_obj = 0 as *mut pdf_obj;
        /* Create leaf nodes. */
        names = pdf_new_array();
        i = 0i32;
        while i < num_leaves {
            let mut cur: *mut named_object = 0 as *mut named_object;
            cur = &mut *first.offset(i as isize) as *mut named_object;
            pdf_add_array(
                names,
                pdf_new_string((*cur).key as *const libc::c_void, (*cur).keylen as size_t),
            );
            match pdf_obj_typeof((*cur).value) {
                5 | 6 | 7 | 3 => {
                    pdf_add_array(names, pdf_ref_obj((*cur).value));
                }
                0 => {
                    _tt_abort(
                        b"Invalid object...: %s\x00" as *const u8 as *const i8,
                        printable_key((*cur).key, (*cur).keylen),
                    );
                }
                _ => {
                    pdf_add_array(names, pdf_link_obj((*cur).value));
                }
            }
            pdf_release_obj((*cur).value);
            (*cur).value = 0 as *mut pdf_obj;
            i += 1
        }
        pdf_add_dict(
            result,
            pdf_new_name(b"Names\x00" as *const u8 as *const i8),
            names,
        );
    } else if num_leaves > 0i32 {
        let mut kids: *mut pdf_obj = 0 as *mut pdf_obj;
        /* Intermediate node */
        kids = pdf_new_array();
        i = 0i32;
        while i < 4i32 {
            let mut subtree: *mut pdf_obj = 0 as *mut pdf_obj;
            let mut start: i32 = 0;
            let mut end: i32 = 0;
            start = i * num_leaves / 4i32;
            end = (i + 1i32) * num_leaves / 4i32;
            subtree = build_name_tree(&mut *first.offset(start as isize), end - start, 0i32);
            pdf_add_array(kids, pdf_ref_obj(subtree));
            pdf_release_obj(subtree);
            i += 1
        }
        pdf_add_dict(
            result,
            pdf_new_name(b"Kids\x00" as *const u8 as *const i8),
            kids,
        );
    }
    result
}
unsafe extern "C" fn flat_table(
    mut ht_tab: *mut ht_table,
    mut num_entries: *mut i32,
    mut filter: *mut ht_table,
) -> *mut named_object {
    let mut objects: *mut named_object = 0 as *mut named_object;
    let mut iter: ht_iter = ht_iter {
        index: 0,
        curr: 0 as *mut libc::c_void,
        hash: 0 as *mut ht_table,
    };
    let mut count: i32 = 0;
    assert!(!ht_tab.is_null());
    objects = new(((*ht_tab).count as u32 as u64)
        .wrapping_mul(::std::mem::size_of::<named_object>() as u64) as u32)
        as *mut named_object;
    count = 0i32;
    if ht_set_iter(ht_tab, &mut iter) >= 0i32 {
        let mut current_block_19: u64;
        loop {
            let mut key: *mut i8 = 0 as *mut i8;
            let mut keylen: i32 = 0;
            let mut value: *mut obj_data = 0 as *mut obj_data;
            key = ht_iter_getkey(&mut iter, &mut keylen);
            if !filter.is_null() {
                let mut new_obj: *mut pdf_obj =
                    ht_lookup_table(filter, key as *const libc::c_void, keylen) as *mut pdf_obj;
                if new_obj.is_null() {
                    current_block_19 = 15240798224410183470;
                } else {
                    key = pdf_string_value(new_obj) as *mut i8;
                    keylen = pdf_string_length(new_obj) as i32;
                    current_block_19 = 12800627514080957624;
                }
            } else {
                current_block_19 = 12800627514080957624;
            }
            match current_block_19 {
                12800627514080957624 => {
                    value = ht_iter_getval(&mut iter) as *mut obj_data;
                    assert!(!(*value).object.is_null());
                    if !(*value).object.is_null() && pdf_obj_typeof((*value).object) == 10i32 {
                        dpx_warning(
                            b"Object @%s\" not defined. Replaced by null.\x00" as *const u8
                                as *const i8,
                            printable_key(key, keylen),
                        );
                        let ref mut fresh4 = (*objects.offset(count as isize)).key;
                        *fresh4 = key;
                        (*objects.offset(count as isize)).keylen = keylen;
                        let ref mut fresh5 = (*objects.offset(count as isize)).value;
                        *fresh5 = pdf_new_null()
                    } else if !(*value).object.is_null() {
                        let ref mut fresh6 = (*objects.offset(count as isize)).key;
                        *fresh6 = key;
                        (*objects.offset(count as isize)).keylen = keylen;
                        let ref mut fresh7 = (*objects.offset(count as isize)).value;
                        *fresh7 = pdf_link_obj((*value).object)
                    }
                    count += 1
                }
                _ => {}
            }
            if !(ht_iter_next(&mut iter) >= 0i32) {
                break;
            }
        }
        ht_clear_iter(&mut iter);
    }
    *num_entries = count;
    objects = renew(
        objects as *mut libc::c_void,
        (count as u32 as u64).wrapping_mul(::std::mem::size_of::<named_object>() as u64) as u32,
    ) as *mut named_object;
    objects
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
/* Hash */
/* Not actually tree... */
/* Really create name tree... */
#[no_mangle]
pub unsafe extern "C" fn pdf_names_create_tree(
    mut names: *mut ht_table,
    mut count: *mut i32,
    mut filter: *mut ht_table,
) -> *mut pdf_obj {
    let mut name_tree: *mut pdf_obj = 0 as *mut pdf_obj;
    let mut flat: *mut named_object = 0 as *mut named_object;
    flat = flat_table(names, count, filter);
    if flat.is_null() {
        name_tree = 0 as *mut pdf_obj
    } else {
        qsort(
            flat as *mut libc::c_void,
            *count as size_t,
            ::std::mem::size_of::<named_object>() as u64,
            Some(
                cmp_key
                    as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
            ),
        );
        name_tree = build_name_tree(flat, *count, 1i32);
        free(flat as *mut libc::c_void);
    }
    name_tree
}
