#![feature(extern_types)]
#![feature(ptr_wrapping_offset_from)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_unsafe
)]
#[macro_use]
extern crate tectonic_bridge as bridge;
extern crate tectonic_dvipdfmx as dpx;

pub use bridge::*;

//use log::{info, warn};

pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type size_t = u64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;

use bibtex::bibtex_main;
use dpx::dvipdfmx_main;
use xetex_ini::tt_run_engine;

#[no_mangle]
pub unsafe extern "C" fn tex_simple_main(
    mut api: *const tt_bridge_api_t,
    mut dump_name: *const i8,
    mut input_file_name: *const i8,
) -> i32 {
    bridge::tt_with_bridge(api, || tt_run_engine(dump_name, input_file_name) as i32)
        .unwrap_or(TTHistory::FATAL_ERROR as i32)
}
#[no_mangle]
pub unsafe extern "C" fn dvipdfmx_simple_main(
    mut api: *const tt_bridge_api_t,
    mut dviname: *const i8,
    mut pdfname: *const i8,
    mut compress: bool,
    mut deterministic_tags: bool,
) -> i32 {
    bridge::tt_with_bridge(api, || {
        dvipdfmx_main(
            pdfname,
            dviname,
            0 as *const i8,
            0i32,
            false,
            compress,
            deterministic_tags,
            false,
            0_u32,
        ) as i32
    })
    .unwrap_or(99)
}
#[no_mangle]
pub unsafe extern "C" fn bibtex_simple_main(
    mut api: *const tt_bridge_api_t,
    mut aux_file_name: *const i8,
) -> i32 {
    bridge::tt_with_bridge(api, || bibtex_main(aux_file_name) as i32).unwrap_or(99)
}

mod core_memory {
    use bridge::{size_t, ssize_t};
    /* tectonic/core-memory.c: basic C dynamic memory helpers

    Copyright 1993, 1994, 1995, 2008, 2009, 2010, 2011 Karl Berry.
    Copyright 1997, 2002, 2005 Olaf Weber.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Lesser General Public
    License as published by the Free Software Foundation; either
    version 2.1 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with this library; if not, see <http://www.gnu.org/licenses/>.  */
    #[no_mangle]
    pub unsafe extern "C" fn xcalloc(mut nelem: size_t, mut elsize: size_t) -> *mut libc::c_void {
        let nelem = nelem as libc::size_t; //FIXME
        let elsize = elsize as libc::size_t; //FIXME
        let mut new_mem: *mut libc::c_void = libc::calloc(
            if nelem != 0 { nelem } else { 1 },
            if elsize != 0 { elsize } else { 1 },
        );
        if new_mem.is_null() {
            panic!(
                "xcalloc request for {} elements of size {} failed",
                nelem, elsize,
            );
        }
        new_mem
    }
    #[no_mangle]
    pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut libc::c_void {
        let size = size as libc::size_t; //FIXME

        let mut new_mem: *mut libc::c_void = libc::malloc(if size != 0 { size } else { 1 });
        if new_mem.is_null() {
            panic!("xmalloc request for {} bytes failed", size,);
        }
        new_mem
    }
    #[no_mangle]
    pub unsafe extern "C" fn xrealloc(
        mut old_ptr: *mut libc::c_void,
        mut size: size_t,
    ) -> *mut libc::c_void {
        let size = size as libc::size_t; //FIXME
        let mut new_mem: *mut libc::c_void = 0 as *mut libc::c_void;
        if old_ptr.is_null() {
            new_mem = xmalloc(size as size_t)
        } else {
            new_mem = libc::realloc(old_ptr, if size != 0 { size } else { 1 });
            if new_mem.is_null() {
                panic!("xrealloc() to {} bytes failed", size,);
            }
        }
        new_mem
    }
    #[no_mangle]
    pub unsafe extern "C" fn xstrdup(mut s: *const i8) -> *mut i8 {
        let mut new_string: *mut i8 = xmalloc(libc::strlen(s).wrapping_add(1) as size_t) as *mut i8;
        libc::strcpy(new_string, s)
    }

    #[inline]
    pub(crate) unsafe extern "C" fn mfree(ptr: *mut libc::c_void) -> *mut libc::c_void {
        libc::free(ptr);
        std::ptr::null_mut()
    }
}

mod bibtex;
mod xetex_aatfont;
mod xetex_engine_interface;
mod xetex_errors;
mod xetex_ext;
mod xetex_ini;
mod xetex_io;
mod xetex_linebreak;
mod xetex_math;
mod xetex_output;
mod xetex_pagebuilder;
mod xetex_pic;
mod xetex_scaledmath;
mod xetex_shipout;
mod xetex_stringpool;
mod xetex_synctex;
mod xetex_texmfmp;
mod xetex_xetex0;
mod xetex_xetexd;

mod stub_icu;
mod stub_teckit;

pub use xetex_engine_interface::tt_xetex_set_int_variable;

#[inline]
pub(crate) unsafe extern "C" fn strstartswith(s: *const i8, prefix: *const i8) -> *const i8 {
    let length = libc::strlen(prefix);
    if libc::strncmp(s, prefix, length) == 0i32 {
        return s.offset(length as isize);
    }
    0 as *const i8
}

#[inline]
pub(crate) unsafe extern "C" fn streq_ptr(s1: *const i8, s2: *const i8) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return libc::strcmp(s1, s2) == 0i32;
    }
    false
}

mod xetex_layout_engine;
