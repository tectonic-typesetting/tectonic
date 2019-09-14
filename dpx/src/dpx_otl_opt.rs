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

use libc::free;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    #[no_mangle]
    fn dpx_warning(fmt: *const i8, _: ...);
    #[no_mangle]
    fn new(size: u32) -> *mut libc::c_void;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_opt {
    pub rule: *mut bt_node,
    /* _OTL_OPT_H_ */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bt_node {
    pub flag: i32,
    pub left: *mut bt_node,
    pub right: *mut bt_node,
    pub data: [i8; 4],
}
unsafe extern "C" fn match_expr(mut expr: *mut bt_node, mut key: *const i8) -> i32 {
    let mut retval: i32 = 1i32;
    let mut i: i32 = 0;
    if !expr.is_null() {
        if (*expr).left.is_null() && (*expr).right.is_null() {
            i = 0i32;
            while i < 4i32 {
                if (*expr).data[i as usize] as i32 != '?' as i32
                    && (*expr).data[i as usize] as i32 != *key.offset(i as isize) as i32
                {
                    retval = 0i32;
                    break;
                } else {
                    i += 1
                }
            }
        } else {
            if !(*expr).left.is_null() {
                retval = match_expr((*expr).left, key)
            }
            if !(*expr).right.is_null() {
                if retval != 0 && (*expr).flag & 1i32 << 1i32 != 0 {
                    retval &= match_expr((*expr).right, key)
                } else if retval == 0 && (*expr).flag & 1i32 << 1i32 == 0 {
                    retval = match_expr((*expr).right, key)
                }
            }
        }
        if (*expr).flag & 1i32 << 0i32 != 0 {
            retval = if retval != 0 { 0i32 } else { 1i32 }
        }
    }
    retval
}
unsafe extern "C" fn bt_new_tree() -> *mut bt_node {
    let mut expr: *mut bt_node = 0 as *mut bt_node;
    expr =
        new((1_u64).wrapping_mul(::std::mem::size_of::<bt_node>() as u64) as u32) as *mut bt_node;
    (*expr).flag = 0i32;
    (*expr).left = 0 as *mut bt_node;
    (*expr).right = 0 as *mut bt_node;
    memset(
        (*expr).data.as_mut_ptr() as *mut libc::c_void,
        0i32,
        4i32 as u64,
    );
    expr
}
unsafe extern "C" fn bt_release_tree(mut tree: *mut bt_node) {
    if !tree.is_null() {
        if !(*tree).left.is_null() {
            bt_release_tree((*tree).left);
        }
        if !(*tree).right.is_null() {
            bt_release_tree((*tree).right);
        }
        free(tree as *mut libc::c_void);
    };
}
unsafe extern "C" fn parse_expr(mut pp: *mut *const i8, mut endptr: *const i8) -> *mut bt_node {
    let mut root: *mut bt_node = 0 as *mut bt_node;
    let mut curr: *mut bt_node = 0 as *mut bt_node;
    if *pp >= endptr {
        return 0 as *mut bt_node;
    }
    curr = bt_new_tree();
    root = curr;
    while *pp < endptr {
        match **pp as i32 {
            33 => {
                if (*curr).flag & 2i32 != 0 {
                    (*curr).flag &= !(1i32 << 0i32)
                } else {
                    (*curr).flag |= 1i32 << 0i32
                }
                *pp = (*pp).offset(1)
            }
            40 => {
                *pp = (*pp).offset(1);
                if *pp < endptr {
                    let mut expr: *mut bt_node = 0 as *mut bt_node;
                    expr = parse_expr(pp, endptr);
                    if expr.is_null() {
                        dpx_warning(b"Syntax error: %s\n\x00" as *const u8 as *const i8, *pp);
                        return 0 as *mut bt_node;
                    }
                    if **pp as i32 != ')' as i32 {
                        warn!("Syntax error: Unbalanced ()\n");
                        return 0 as *mut bt_node;
                    }
                    (*curr).left = (*expr).left;
                    (*curr).right = (*expr).right;
                    memcpy(
                        (*curr).data.as_mut_ptr() as *mut libc::c_void,
                        (*expr).data.as_mut_ptr() as *const libc::c_void,
                        4i32 as u64,
                    );
                    free(expr as *mut libc::c_void);
                } else {
                    warn!("Syntax error: Unbalanced ()\n");
                    bt_release_tree(root);
                    return 0 as *mut bt_node;
                }
                *pp = (*pp).offset(1)
            }
            41 => return root,
            124 | 38 => {
                if *pp >= endptr {
                    dpx_warning(b"Syntax error: %s\n\x00" as *const u8 as *const i8, *pp);
                    bt_release_tree(root);
                    return 0 as *mut bt_node;
                } else {
                    let mut tmp: *mut bt_node = 0 as *mut bt_node;
                    tmp = bt_new_tree();
                    (*tmp).left = root;
                    curr = bt_new_tree();
                    (*tmp).right = curr;
                    if **pp as i32 == '&' as i32 {
                        (*tmp).flag = 1i32
                    } else {
                        (*tmp).flag = 0i32
                    }
                    root = tmp
                }
                *pp = (*pp).offset(1)
            }
            42 => {
                memset(
                    (*curr).data.as_mut_ptr() as *mut libc::c_void,
                    '?' as i32,
                    4i32 as u64,
                );
                *pp = (*pp).offset(1)
            }
            _ => {
                if (*pp).offset(4) <= endptr {
                    let mut i: i32 = 0;
                    i = 0i32;
                    while i < 4i32 {
                        if **pp as i32 == ' ' as i32
                            || **pp as i32 == '?' as i32
                            || libc::isalpha(**pp as _) != 0
                            || libc::isdigit(**pp as _) != 0
                        {
                            (*curr).data[i as usize] = **pp
                        } else if **pp as i32 == '_' as i32 {
                            (*curr).data[i as usize] = ' ' as i32 as i8
                        } else {
                            dpx_warning(
                                b"Invalid char in tag: %c\n\x00" as *const u8 as *const i8,
                                **pp as i32,
                            );
                            bt_release_tree(root);
                            return 0 as *mut bt_node;
                        }
                        *pp = (*pp).offset(1);
                        i += 1
                    }
                } else {
                    dpx_warning(b"Syntax error: %s\n\x00" as *const u8 as *const i8, *pp);
                    bt_release_tree(root);
                    return 0 as *mut bt_node;
                }
            }
        }
    }
    root
}
#[no_mangle]
pub unsafe extern "C" fn otl_new_opt() -> *mut otl_opt {
    let mut opt: *mut otl_opt = 0 as *mut otl_opt;
    opt = new((1_u64).wrapping_mul(::std::mem::size_of::<otl_opt>() as u64) as u32) as *mut otl_opt;
    (*opt).rule = 0 as *mut bt_node;
    opt as *mut otl_opt
}
#[no_mangle]
pub unsafe extern "C" fn otl_release_opt(mut opt: *mut otl_opt) {
    if !(*opt).rule.is_null() {
        bt_release_tree((*opt).rule);
    }
    (*opt).rule = 0 as *mut bt_node;
    free(opt as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn otl_parse_optstring(mut opt: *mut otl_opt, mut optstr: *const i8) -> i32 {
    let mut p: *const i8 = 0 as *const i8;
    let mut endptr: *const i8 = 0 as *const i8;
    assert!(!opt.is_null());
    if !optstr.is_null() {
        p = optstr;
        endptr = p.offset(strlen(optstr) as isize);
        (*opt).rule = parse_expr(&mut p, endptr)
    }
    0i32
}
#[no_mangle]
pub unsafe extern "C" fn otl_match_optrule(mut opt: *mut otl_opt, mut tag: *const i8) -> i32 {
    assert!(!tag.is_null());
    if opt.is_null() || (*opt).rule.is_null() {
        return 1i32;
    }
    match_expr((*opt).rule, tag)
}
