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

use crate::streq_ptr;

pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paper {
    pub name: *const i8,
    pub pswidth: f64,
    pub psheight: f64,
}
#[no_mangle]
pub static mut paperspecs: [paper; 22] = [
    {
        let mut init = paper {
            name: b"letter\x00" as *const u8 as *const i8,
            pswidth: 612.00f64,
            psheight: 792.00f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"legal\x00" as *const u8 as *const i8,
            pswidth: 612.00f64,
            psheight: 1008.00f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"ledger\x00" as *const u8 as *const i8,
            pswidth: 1224.00f64,
            psheight: 792.00f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"tabloid\x00" as *const u8 as *const i8,
            pswidth: 792.00f64,
            psheight: 1224.00f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"a6\x00" as *const u8 as *const i8,
            pswidth: 297.638f64,
            psheight: 419.528f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"a5\x00" as *const u8 as *const i8,
            pswidth: 419.528f64,
            psheight: 595.276f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"a4\x00" as *const u8 as *const i8,
            pswidth: 595.276f64,
            psheight: 841.890f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"a3\x00" as *const u8 as *const i8,
            pswidth: 841.890f64,
            psheight: 1190.550f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"b6\x00" as *const u8 as *const i8,
            pswidth: 364.25f64,
            psheight: 515.91f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"b5\x00" as *const u8 as *const i8,
            pswidth: 515.91f64,
            psheight: 728.50f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"b4\x00" as *const u8 as *const i8,
            pswidth: 728.50f64,
            psheight: 1031.81f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"b3\x00" as *const u8 as *const i8,
            pswidth: 1031.81f64,
            psheight: 1457.00f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"b5var\x00" as *const u8 as *const i8,
            pswidth: 515.91f64,
            psheight: 651.97f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"jisb6\x00" as *const u8 as *const i8,
            pswidth: 364.25f64,
            psheight: 515.91f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"jisb5\x00" as *const u8 as *const i8,
            pswidth: 515.91f64,
            psheight: 728.50f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"jisb4\x00" as *const u8 as *const i8,
            pswidth: 728.50f64,
            psheight: 1031.81f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"jisb3\x00" as *const u8 as *const i8,
            pswidth: 1031.81f64,
            psheight: 1457.00f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"isob6\x00" as *const u8 as *const i8,
            pswidth: 354.331f64,
            psheight: 498.898f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"isob5\x00" as *const u8 as *const i8,
            pswidth: 498.898f64,
            psheight: 708.661f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"isob4\x00" as *const u8 as *const i8,
            pswidth: 708.661f64,
            psheight: 1000.630f64,
        };
        init
    },
    {
        let mut init = paper {
            name: b"isob3\x00" as *const u8 as *const i8,
            pswidth: 1000.630f64,
            psheight: 1417.320f64,
        };
        init
    },
    {
        let mut init = paper {
            name: 0 as *const i8,
            pswidth: 0i32 as f64,
            psheight: 0i32 as f64,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn paperinfo(mut ppformat: *const i8) -> *const paper {
    let mut ppinfo: *const paper = 0 as *const paper;
    if ppformat.is_null() {
        return 0 as *const paper;
    }
    ppinfo = &*paperspecs.as_ptr().offset(0) as *const paper;
    while !ppinfo.is_null()
        && !(if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).name
        } else {
            0 as *const i8
        })
        .is_null()
    {
        if streq_ptr(ppformat, (*ppinfo).name) {
            break;
        }
        ppinfo = if !ppinfo.offset(1).is_null() && !(*ppinfo.offset(1)).name.is_null() {
            ppinfo.offset(1)
        } else {
            0 as *const paper
        }
    }
    return if !ppinfo.is_null()
        && !(if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).name
        } else {
            0 as *const i8
        })
        .is_null()
    {
        ppinfo
    } else {
        0 as *const paper
    };
}
/* HAVE_LIBPAPER */
/* HAVE_LIBPAPER */
#[no_mangle]
pub unsafe extern "C" fn dumppaperinfo() {
    let mut ppinfo: *const paper = 0 as *const paper;
    ppinfo = &*paperspecs.as_ptr().offset(0) as *const paper;
    while !ppinfo.is_null()
        && !(if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).name
        } else {
            0 as *const i8
        })
        .is_null()
    {
        let mut wd: f64 = 0.;
        let mut ht: f64 = 0.;
        wd = if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).pswidth
        } else {
            0.0f64
        };
        ht = if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).psheight
        } else {
            0.0f64
        };
        println!(
            "{}: {:.2} {:.2} ({:.2}mm {:.2}mm)",
            if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
                use std::ffi::CStr;
                let name = CStr::from_ptr((*ppinfo).name);
                name.to_string_lossy()
            } else {
                use std::borrow::Cow;
                Cow::Borrowed("(null)")
            },
            wd,
            ht,
            25.4f64 * wd / 72.0f64,
            25.4f64 * ht / 72.0f64,
        );
        ppinfo = if !ppinfo.offset(1).is_null() && !(*ppinfo.offset(1)).name.is_null() {
            ppinfo.offset(1)
        } else {
            0 as *const paper
        }
    }
}
