#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

use crate::{ttstub_get_data_md5, ttstub_get_file_md5};
use libc::free;
extern "C" {
    #[no_mangle]
    fn strlen(_: *const i8) -> u64;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const i8, _: ...) -> !;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    #[no_mangle]
    static mut pool_size: i32;
    #[no_mangle]
    static mut str_pool: *mut packed_UTF16_code;
    #[no_mangle]
    static mut str_start: *mut pool_pointer;
    #[no_mangle]
    static mut pool_ptr: pool_pointer;
    #[no_mangle]
    static firstByteMark: [u8; 7];
    #[no_mangle]
    static offsetsFromUTF8: [u32; 6];
    #[no_mangle]
    static bytesFromUTF8: [u8; 256];
    #[no_mangle]
    fn make_string() -> str_number;
}
pub type __time_t = i64;
pub type size_t = u64;
pub type time_t = __time_t;
pub type str_number = i32;
pub type packed_UTF16_code = u16;
pub type UInt32 = u32;
pub type pool_pointer = i32;
pub type UInt16 = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
/* texmfmp.c: Hand-coded routines for TeX or Metafont in C.  Originally
written by Tim Morgan, drawing from other Unix ports of TeX.  This is
a collection of miscellany, everything that's easier (or only
possible) to do in C.

This file is public domain.  */
/* For `struct tm'.  Moved here for Visual Studio 2005.  */
static mut last_source_name: *mut i8 = 0 as *const i8 as *mut i8;
static mut last_lineno: i32 = 0;
pub fn get_date_and_time() -> (i32, i32, i32, i32) {
    use datetime::{DatePiece, TimePiece};
    let tm = datetime::LocalDateTime::now();
    let minutes = (tm.hour() as i32) * 60 + (tm.minute() as i32);
    let day = tm.day() as i32;
    let month = (tm.month().months_from_january() as i32) + 1;
    let year = tm.year() as i32;
    (minutes, day, month, year)
}
unsafe extern "C" fn checkpool_pointer(mut pool_ptr_0: pool_pointer, mut len: size_t) {
    if (pool_ptr_0 as u64).wrapping_add(len) >= pool_size as u64 {
        _tt_abort(
            b"string pool overflow [%i bytes]\x00" as *const u8 as *const i8,
            pool_size,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn maketexstring(mut s: *const i8) -> i32 {
    let mut len: size_t = 0;
    let mut rval: UInt32 = 0;
    let mut cp: *const u8 = s as *const u8;
    if s.is_null() || *s as i32 == 0i32 {
        return (65536 + 1i32 as i64) as i32;
    }
    len = strlen(s);
    checkpool_pointer(pool_ptr, len);
    loop {
        let fresh0 = cp;
        cp = cp.offset(1);
        rval = *fresh0 as UInt32;
        if !(rval != 0_u32) {
            break;
        }
        let mut extraBytes: UInt16 = bytesFromUTF8[rval as usize] as UInt16;
        let mut current_block_19: u64;
        match extraBytes as i32 {
            5 => {
                /* note: code falls through cases! */
                rval <<= 6i32; /* max UTF16->UTF8 expansion
                               (code units, not bytes) */
                if *cp != 0 {
                    let fresh1 = cp;
                    cp = cp.offset(1);
                    rval = (rval as u32).wrapping_add(*fresh1 as u32) as UInt32 as UInt32
                }
                current_block_19 = 15420705083065539194;
            }
            4 => {
                current_block_19 = 15420705083065539194;
            }
            3 => {
                current_block_19 = 17593909170536150684;
            }
            2 => {
                current_block_19 = 9565569445570550704;
            }
            1 => {
                current_block_19 = 4209676304665092873;
            }
            0 | _ => {
                current_block_19 = 11194104282611034094;
            }
        }
        match current_block_19 {
            15420705083065539194 => {
                rval <<= 6i32;
                if *cp != 0 {
                    let fresh2 = cp;
                    cp = cp.offset(1);
                    rval = (rval as u32).wrapping_add(*fresh2 as u32) as UInt32 as UInt32
                }
                current_block_19 = 17593909170536150684;
            }
            _ => {}
        }
        match current_block_19 {
            17593909170536150684 => {
                rval <<= 6i32;
                if *cp != 0 {
                    let fresh3 = cp;
                    cp = cp.offset(1);
                    rval = (rval as u32).wrapping_add(*fresh3 as u32) as UInt32 as UInt32
                }
                current_block_19 = 9565569445570550704;
            }
            _ => {}
        }
        match current_block_19 {
            9565569445570550704 => {
                rval <<= 6i32;
                if *cp != 0 {
                    let fresh4 = cp;
                    cp = cp.offset(1);
                    rval = (rval as u32).wrapping_add(*fresh4 as u32) as UInt32 as UInt32
                }
                current_block_19 = 4209676304665092873;
            }
            _ => {}
        }
        match current_block_19 {
            4209676304665092873 => {
                rval <<= 6i32;
                if *cp != 0 {
                    let fresh5 = cp;
                    cp = cp.offset(1);
                    rval = (rval as u32).wrapping_add(*fresh5 as u32) as UInt32 as UInt32
                }
            }
            _ => {}
        }
        rval = (rval as u32).wrapping_sub(offsetsFromUTF8[extraBytes as usize]) as UInt32 as UInt32;
        if rval > 0xffff_u32 {
            rval = (rval as u32).wrapping_sub(0x10000_u32) as UInt32 as UInt32;
            let fresh6 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            *str_pool.offset(fresh6 as isize) =
                (0xd800_u32).wrapping_add(rval.wrapping_div(0x400_u32)) as packed_UTF16_code;
            let fresh7 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            *str_pool.offset(fresh7 as isize) =
                (0xdc00_u32).wrapping_add(rval.wrapping_rem(0x400_u32)) as packed_UTF16_code
        } else {
            let fresh8 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            *str_pool.offset(fresh8 as isize) = rval as packed_UTF16_code
        }
    }
    make_string()
}
#[no_mangle]
pub unsafe extern "C" fn gettexstring(mut s: str_number) -> *mut i8 {
    let mut bytesToWrite: u32 = 0_u32;
    let mut len: pool_pointer = 0;
    let mut i: pool_pointer = 0;
    let mut j: pool_pointer = 0;
    let mut name: *mut i8 = 0 as *mut i8;
    if s as i64 >= 65536 {
        len = *str_start.offset(((s + 1i32) as i64 - 65536) as isize)
            - *str_start.offset((s as i64 - 65536) as isize)
    } else {
        len = 0i32
    }
    name = xmalloc((len * 3i32 + 1i32) as size_t) as *mut i8;
    i = 0i32;
    j = 0i32;
    while i < len {
        let mut c: u32 =
            *str_pool.offset((i + *str_start.offset((s as i64 - 65536) as isize)) as isize) as u32;
        if c >= 0xd800_u32 && c <= 0xdbff_u32 {
            i += 1;
            let mut lo: u32 = *str_pool
                .offset((i + *str_start.offset((s as i64 - 65536) as isize)) as isize)
                as u32;
            if lo >= 0xdc00_u32 && lo <= 0xdfff_u32 {
                c = c
                    .wrapping_sub(0xd800_u32)
                    .wrapping_mul(0x400_u32)
                    .wrapping_add(lo)
                    .wrapping_sub(0xdc00_u32)
                    .wrapping_add(0x10000_u32)
            } else {
                c = 0xfffd_u32
            }
        }
        if c < 0x80_u32 {
            bytesToWrite = 1_u32
        } else if c < 0x800_u32 {
            bytesToWrite = 2_u32
        } else if c < 0x10000_u32 {
            bytesToWrite = 3_u32
        } else if c < 0x110000_u32 {
            bytesToWrite = 4_u32
        } else {
            bytesToWrite = 3_u32;
            c = 0xfffd_u32
        }
        j = (j as u32).wrapping_add(bytesToWrite) as pool_pointer as pool_pointer;
        let mut current_block_28: u64;
        match bytesToWrite {
            4 => {
                /* note: everything falls through. */
                j -= 1;
                *name.offset(j as isize) = ((c | 0x80_u32) & 0xbf_u32) as i8;
                c >>= 6i32;
                current_block_28 = 9281751456159701257;
            }
            3 => {
                current_block_28 = 9281751456159701257;
            }
            2 => {
                current_block_28 = 13645261163415976511;
            }
            1 => {
                current_block_28 = 4925739576308592327;
            }
            _ => {
                current_block_28 = 2891135413264362348;
            }
        }
        match current_block_28 {
            9281751456159701257 => {
                j -= 1;
                *name.offset(j as isize) = ((c | 0x80_u32) & 0xbf_u32) as i8;
                c >>= 6i32;
                current_block_28 = 13645261163415976511;
            }
            _ => {}
        }
        match current_block_28 {
            13645261163415976511 => {
                j -= 1;
                *name.offset(j as isize) = ((c | 0x80_u32) & 0xbf_u32) as i8;
                c >>= 6i32;
                current_block_28 = 4925739576308592327;
            }
            _ => {}
        }
        match current_block_28 {
            4925739576308592327 => {
                j -= 1;
                *name.offset(j as isize) = (c | firstByteMark[bytesToWrite as usize] as u32) as i8
            }
            _ => {}
        }
        j = (j as u32).wrapping_add(bytesToWrite) as pool_pointer as pool_pointer;
        i += 1
    }
    *name.offset(j as isize) = 0_i8;
    name
}
unsafe extern "C" fn compare_paths(mut p1: *const i8, mut p2: *const i8) -> i32 {
    let mut ret: i32 = 0;
    loop {
        ret = *p1 as i32 - *p2 as i32;
        if !(ret == 0i32 && *p2 as i32 != 0i32
            || *p1 as i32 == '/' as i32 && *p2 as i32 == '/' as i32)
        {
            break;
        }
        p1 = p1.offset(1);
        p2 = p2.offset(1)
    }
    ret = if ret < 0i32 {
        -1i32
    } else if ret > 0i32 {
        1i32
    } else {
        0i32
    };
    ret
}
#[no_mangle]
pub unsafe extern "C" fn is_new_source(mut srcfilename: str_number, mut lineno: i32) -> bool {
    let mut name: *mut i8 = gettexstring(srcfilename);
    compare_paths(name, last_source_name) != 0i32 || lineno != last_lineno
}
#[no_mangle]
pub unsafe extern "C" fn remember_source_info(mut srcfilename: str_number, mut lineno: i32) {
    free(last_source_name as *mut libc::c_void);
    last_source_name = gettexstring(srcfilename);
    last_lineno = lineno;
}
#[no_mangle]
pub unsafe extern "C" fn make_src_special(
    mut srcfilename: str_number,
    mut lineno: i32,
) -> pool_pointer {
    let mut oldpool_ptr: pool_pointer = pool_ptr;
    let mut filename: *mut i8 = gettexstring(srcfilename);
    /* FIXME: Magic number. */
    let mut buf: [i8; 40] = [0; 40];
    let mut s: *mut i8 = buf.as_mut_ptr();
    /* Always put a space after the number, which makes things easier
     * to parse.
     */
    sprintf(
        buf.as_mut_ptr(),
        b"src:%d \x00" as *const u8 as *const i8,
        lineno,
    );
    if (pool_ptr as u64)
        .wrapping_add(strlen(buf.as_mut_ptr()))
        .wrapping_add(strlen(filename))
        >= pool_size as size_t
    {
        _tt_abort(b"string pool overflow\x00" as *const u8 as *const i8);
    }
    s = buf.as_mut_ptr();
    while *s != 0 {
        let fresh9 = s;
        s = s.offset(1);
        let fresh10 = pool_ptr;
        pool_ptr = pool_ptr + 1;
        *str_pool.offset(fresh10 as isize) = *fresh9 as packed_UTF16_code
    }
    s = filename;
    while *s != 0 {
        let fresh11 = s;
        s = s.offset(1);
        let fresh12 = pool_ptr;
        pool_ptr = pool_ptr + 1;
        *str_pool.offset(fresh12 as isize) = *fresh11 as packed_UTF16_code
    }
    oldpool_ptr
}
/* Converts any given string in into an allowed PDF string which is
 * hexadecimal encoded;
 * sizeof(out) should be at least lin*2+1.
 */
unsafe extern "C" fn convertStringToHexString(mut in_0: *const i8, mut out: *mut i8, mut lin: i32) {
    static mut hexchars: [i8; 17] = [
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 0,
    ];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    j = 0i32;
    i = 0i32;
    while i < lin {
        let mut c: u8 = *in_0.offset(i as isize) as u8;
        let fresh13 = j;
        j = j + 1;
        *out.offset(fresh13 as isize) = hexchars[(c as i32 >> 4i32 & 0xfi32) as usize];
        let fresh14 = j;
        j = j + 1;
        *out.offset(fresh14 as isize) = hexchars[(c as i32 & 0xfi32) as usize];
        i += 1
    }
    *out.offset(j as isize) = '\u{0}' as i32 as i8;
}
/* Functions originating in texmfmp.c */
#[no_mangle]
pub unsafe extern "C" fn getmd5sum(mut s: str_number, mut file: bool) {
    let mut digest: [i8; 16] = [0; 16];
    let mut outbuf: [i8; 33] = [0; 33];
    let mut xname: *mut i8 = 0 as *mut i8;
    let mut ret: i32 = 0;
    let mut i: i32 = 0;
    xname = gettexstring(s);
    if file {
        ret = ttstub_get_file_md5(xname, digest.as_mut_ptr())
    } else {
        ret = ttstub_get_data_md5(xname, strlen(xname), digest.as_mut_ptr())
    }
    free(xname as *mut libc::c_void);
    if ret != 0 {
        return;
    }
    if pool_ptr + 2i32 * 16i32 >= pool_size {
        /* error by str_toks that calls str_room(1) */
        return;
    }
    convertStringToHexString(digest.as_mut_ptr(), outbuf.as_mut_ptr(), 16i32);
    i = 0i32;
    while i < 2i32 * 16i32 {
        let fresh15 = pool_ptr;
        pool_ptr = pool_ptr + 1;
        *str_pool.offset(fresh15 as isize) = outbuf[i as usize] as u16;
        i += 1
    }
}
