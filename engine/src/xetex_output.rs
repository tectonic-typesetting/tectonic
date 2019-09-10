#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    #[no_mangle]
    fn ttstub_output_putc(handle: rust_output_handle_t, c: libc::c_int) -> libc::c_int;
    /* Needed here for UFILE */
    /* variables! */
    /* All the following variables are defined in xetexini.c */
    #[no_mangle]
    static mut eqtb: *mut memory_word;
    #[no_mangle]
    static mut error_line: int32_t;
    #[no_mangle]
    static mut max_print_line: int32_t;
    #[no_mangle]
    static mut pool_size: int32_t;
    #[no_mangle]
    static mut str_pool: *mut packed_UTF16_code;
    #[no_mangle]
    static mut str_start: *mut pool_pointer;
    #[no_mangle]
    static mut pool_ptr: pool_pointer;
    #[no_mangle]
    static mut str_ptr: str_number;
    #[no_mangle]
    static mut rust_stdout: rust_output_handle_t;
    #[no_mangle]
    static mut log_file: rust_output_handle_t;
    #[no_mangle]
    static mut selector: selector_t;
    #[no_mangle]
    static mut dig: [u8; 23];
    #[no_mangle]
    static mut tally: int32_t;
    #[no_mangle]
    static mut term_offset: int32_t;
    #[no_mangle]
    static mut file_offset: int32_t;
    #[no_mangle]
    static mut trick_buf: [UTF16_code; 256];
    #[no_mangle]
    static mut trick_count: int32_t;
    #[no_mangle]
    static mut doing_special: bool;
    #[no_mangle]
    static mut mem: *mut memory_word;
    #[no_mangle]
    static mut hash: *mut b32x2;
    #[no_mangle]
    static mut eqtb_top: int32_t;
    #[no_mangle]
    static mut in_open: int32_t;
    #[no_mangle]
    static mut line: int32_t;
    #[no_mangle]
    static mut line_stack: *mut int32_t;
    #[no_mangle]
    static mut full_source_filename_stack: *mut str_number;
    #[no_mangle]
    static mut write_file: [rust_output_handle_t; 16];
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type rust_output_handle_t = *mut libc::c_void;
pub type scaled_t = int32_t;
pub type selector_t = libc::c_uint;
pub const SELECTOR_NEW_STRING: selector_t = 21;
pub const SELECTOR_PSEUDO: selector_t = 20;
pub const SELECTOR_TERM_AND_LOG: selector_t = 19;
pub const SELECTOR_LOG_ONLY: selector_t = 18;
pub const SELECTOR_TERM_ONLY: selector_t = 17;
pub const SELECTOR_NO_PRINT: selector_t = 16;
pub const SELECTOR_FILE_15: selector_t = 15;
pub const SELECTOR_FILE_0: selector_t = 0;
/* tectonic/xetex-xetexd.h -- many, many XeTeX symbol definitions
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/
/* Extra stuff used in various change files for various reasons.  */
/* Array allocations. Add 1 to size to account for Pascal indexing convention. */
/*11:*/
/*18: */
pub type UTF16_code = u16;
pub type eight_bits = u8;
pub type pool_pointer = int32_t;
pub type str_number = int32_t;
pub type packed_UTF16_code = u16;
pub type small_number = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2_le_t {
    pub s0: int32_t,
    pub s1: int32_t,
}
/* The annoying `memory_word` type. We have to make sure the byte-swapping
 * that the (un)dumping routines do suffices to put things in the right place
 * in memory.
 *
 * This set of data used to be a huge mess (see comment after the
 * definitions). It is now (IMO) a lot more reasonable, but there will no
 * doubt be carryover weird terminology around the code.
 *
 * ## ENDIANNESS (cheat sheet because I'm lame)
 *
 * Intel is little-endian. Say that we have a 32-bit integer stored in memory
 * with `p` being a `uint8` pointer to its location. In little-endian land,
 * `p[0]` is least significant byte and `p[3]` is its most significant byte.
 *
 * Conversely, in big-endian land, `p[0]` is its most significant byte and
 * `p[3]` is its least significant byte.
 *
 * ## MEMORY_WORD LAYOUT
 *
 * Little endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [lsb......s0.......msb] [lsb......s1.......msb]
 *   b16:   [l..s0...m] [l..s1...m] [l..s2...m] [l..s3...m]
 *
 * Big endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [msb......s1.......lsb] [msb......s0.......lsb]
 *   b16:   [m..s3...l] [m..s2...l] [m..s1...l] [m...s0..l]
 *
 */
pub type b32x2 = b32x2_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4_le_t {
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}
pub type b16x4 = b16x4_le_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: libc::c_double,
    pub ptr: *mut libc::c_void,
}
/* xetex-output */
/* tectonic/output.c -- functions related to outputting messages
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/
#[no_mangle]
pub unsafe extern "C" fn print_ln() {
    match selector as libc::c_uint {
        19 => {
            ttstub_output_putc(rust_stdout, '\n' as i32);
            ttstub_output_putc(log_file, '\n' as i32);
            term_offset = 0i32;
            file_offset = 0i32
        }
        18 => {
            ttstub_output_putc(log_file, '\n' as i32);
            file_offset = 0i32
        }
        17 => {
            ttstub_output_putc(rust_stdout, '\n' as i32);
            term_offset = 0i32
        }
        16 | 20 | 21 => {}
        _ => {
            ttstub_output_putc(write_file[selector as usize], '\n' as i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_raw_char(mut s: UTF16_code, mut incr_offset: bool) {
    match selector as libc::c_uint {
        19 => {
            ttstub_output_putc(rust_stdout, s as libc::c_int);
            ttstub_output_putc(log_file, s as libc::c_int);
            if incr_offset {
                term_offset += 1;
                file_offset += 1
            }
            if term_offset == max_print_line {
                ttstub_output_putc(rust_stdout, '\n' as i32);
                term_offset = 0i32
            }
            if file_offset == max_print_line {
                ttstub_output_putc(log_file, '\n' as i32);
                file_offset = 0i32
            }
        }
        18 => {
            ttstub_output_putc(log_file, s as libc::c_int);
            if incr_offset {
                file_offset += 1
            }
            if file_offset == max_print_line {
                print_ln();
            }
        }
        17 => {
            ttstub_output_putc(rust_stdout, s as libc::c_int);
            if incr_offset {
                term_offset += 1
            }
            if term_offset == max_print_line {
                print_ln();
            }
        }
        16 => {}
        20 => {
            if tally < trick_count {
                trick_buf[(tally % error_line) as usize] = s
            }
        }
        21 => {
            if pool_ptr < pool_size {
                *str_pool.offset(pool_ptr as isize) = s;
                pool_ptr += 1
            }
        }
        _ => {
            ttstub_output_putc(write_file[selector as usize], s as libc::c_int);
        }
    }
    tally += 1;
}
#[no_mangle]
pub unsafe extern "C" fn print_char(mut s: int32_t) {
    let mut l: small_number = 0;
    if selector as libc::c_uint > SELECTOR_PSEUDO as libc::c_int as libc::c_uint && !doing_special {
        if s >= 0x10000i32 {
            print_raw_char(
                (0xd800i32 + (s - 0x10000i32) / 1024i32) as UTF16_code,
                1i32 != 0,
            );
            print_raw_char(
                (0xdc00i32 + (s - 0x10000i32) % 1024i32) as UTF16_code,
                1i32 != 0,
            );
        } else {
            print_raw_char(s as UTF16_code, 1i32 != 0);
        }
        return;
    }
    if s == (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 49i32) as isize,
    ))
    .b32
    .s1
    {
        /*:252 */
        if (selector as libc::c_uint) < SELECTOR_PSEUDO as libc::c_int as libc::c_uint {
            print_ln();
            return;
        }
    }
    if s < 32i32 && !doing_special {
        print_raw_char('^' as i32 as UTF16_code, 1i32 != 0);
        print_raw_char('^' as i32 as UTF16_code, 1i32 != 0);
        print_raw_char((s + 64i32) as UTF16_code, 1i32 != 0);
    } else if s < 127i32 {
        print_raw_char(s as UTF16_code, 1i32 != 0);
    } else if s == 127i32 {
        if !doing_special {
            print_raw_char('^' as i32 as UTF16_code, 1i32 != 0);
            print_raw_char('^' as i32 as UTF16_code, 1i32 != 0);
            print_raw_char('?' as i32 as UTF16_code, 1i32 != 0);
        } else {
            print_raw_char(s as UTF16_code, 1i32 != 0);
        }
    } else if s < 160i32 && !doing_special {
        print_raw_char('^' as i32 as UTF16_code, 1i32 != 0);
        print_raw_char('^' as i32 as UTF16_code, 1i32 != 0);
        l = (s % 256i32 / 16i32) as small_number;
        if (l as libc::c_int) < 10i32 {
            print_raw_char(('0' as i32 + l as libc::c_int) as UTF16_code, 1i32 != 0);
        } else {
            print_raw_char(
                ('a' as i32 + l as libc::c_int - 10i32) as UTF16_code,
                1i32 != 0,
            );
        }
        l = (s % 16i32) as small_number;
        if (l as libc::c_int) < 10i32 {
            print_raw_char(('0' as i32 + l as libc::c_int) as UTF16_code, 1i32 != 0);
        } else {
            print_raw_char(
                ('a' as i32 + l as libc::c_int - 10i32) as UTF16_code,
                1i32 != 0,
            );
        }
    } else if s < 2048i32 {
        print_raw_char((192i32 + s / 64i32) as UTF16_code, 0i32 != 0);
        print_raw_char((128i32 + s % 64i32) as UTF16_code, 1i32 != 0);
    } else if s < 0x10000i32 {
        print_raw_char((224i32 + s / 4096i32) as UTF16_code, 0i32 != 0);
        print_raw_char((128i32 + s % 4096i32 / 64i32) as UTF16_code, 0i32 != 0);
        print_raw_char((128i32 + s % 64i32) as UTF16_code, 1i32 != 0);
    } else {
        print_raw_char((240i32 + s / 0x40000i32) as UTF16_code, 0i32 != 0);
        print_raw_char((128i32 + s % 0x40000i32 / 4096i32) as UTF16_code, 0i32 != 0);
        print_raw_char((128i32 + s % 4096i32 / 64i32) as UTF16_code, 0i32 != 0);
        print_raw_char((128i32 + s % 64i32) as UTF16_code, 1i32 != 0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn print(mut s: int32_t) {
    let mut nl: int32_t = 0;
    if s >= str_ptr {
        return print_cstr(b"???\x00" as *const u8 as *const libc::c_char);
    } else {
        if s < 0xffffi32 {
            if s < 0i32 {
                return print_cstr(b"???\x00" as *const u8 as *const libc::c_char);
            } else {
                if selector as libc::c_uint > SELECTOR_PSEUDO as libc::c_int as libc::c_uint {
                    print_char(s);
                    return;
                }
                if s == (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 49i32) as isize,
                ))
                .b32
                .s1
                {
                    /*:252 */
                    if (selector as libc::c_uint) < SELECTOR_PSEUDO as libc::c_int as libc::c_uint {
                        print_ln();
                        return;
                    }
                }
                nl = (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 49i32) as isize,
                ))
                .b32
                .s1;
                (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 49i32) as isize,
                ))
                .b32
                .s1 = -1i32;
                print_char(s);
                (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 49i32) as isize,
                ))
                .b32
                .s1 = nl;
                return;
            }
        }
    }
    let mut pool_idx: int32_t = s - 0x10000i32;
    let mut i: pool_pointer = *str_start.offset(pool_idx as isize);
    while i < *str_start.offset((pool_idx + 1i32) as isize) {
        if *str_pool.offset(i as isize) as libc::c_int >= 0xd800i32
            && (*str_pool.offset(i as isize) as libc::c_int) < 0xdc00i32
            && i + 1i32 < *str_start.offset((pool_idx + 1i32) as isize)
            && *str_pool.offset((i + 1i32) as isize) as libc::c_int >= 0xdc00i32
            && (*str_pool.offset((i + 1i32) as isize) as libc::c_int) < 0xe000i32
        {
            print_char(
                0x10000i32
                    + (*str_pool.offset(i as isize) as libc::c_int - 0xd800i32) * 1024i32
                    + *str_pool.offset((i + 1i32) as isize) as libc::c_int
                    - 0xdc00i32,
            );
            i += 1
        } else {
            print_char(*str_pool.offset(i as isize) as int32_t);
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_cstr(mut str: *const libc::c_char) {
    let mut i: libc::c_uint = 0i32 as libc::c_uint;
    while (i as u64) < strlen(str) {
        print_char(*str.offset(i as isize) as int32_t);
        i = i.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_nl(mut s: str_number) {
    if term_offset > 0i32 && selector as libc::c_uint & 1i32 as libc::c_uint != 0
        || file_offset > 0i32
            && selector as libc::c_uint >= SELECTOR_LOG_ONLY as libc::c_int as libc::c_uint
    {
        print_ln();
    }
    print(s);
}
#[no_mangle]
pub unsafe extern "C" fn print_nl_cstr(mut str: *const libc::c_char) {
    if term_offset > 0i32 && selector as libc::c_uint & 1i32 as libc::c_uint != 0
        || file_offset > 0i32
            && selector as libc::c_uint >= SELECTOR_LOG_ONLY as libc::c_int as libc::c_uint
    {
        print_ln();
    }
    print_cstr(str);
}
#[no_mangle]
pub unsafe extern "C" fn print_esc(mut s: str_number) {
    let mut c: int32_t = (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 45i32) as isize,
    ))
    .b32
    .s1;
    if c >= 0i32 && c <= 0x10ffffi32 {
        print_char(c);
    }
    print(s);
}
#[no_mangle]
pub unsafe extern "C" fn print_esc_cstr(mut s: *const libc::c_char) {
    let mut c: int32_t = (*eqtb.offset(
        (1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 45i32) as isize,
    ))
    .b32
    .s1;
    if c >= 0i32 && c <= 0x10ffffi32 {
        print_char(c);
    }
    print_cstr(s);
}
unsafe extern "C" fn print_the_digs(mut k: eight_bits) {
    while k as libc::c_int > 0i32 {
        k = k.wrapping_sub(1);
        if (dig[k as usize] as libc::c_int) < 10i32 {
            print_char('0' as i32 + dig[k as usize] as libc::c_int);
        } else {
            print_char(55i32 + dig[k as usize] as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_int(mut n: int32_t) {
    let mut k: u8 = 0i32 as u8;
    let mut m: int32_t = 0;
    if n < 0i32 {
        print_char('-' as i32);
        if n as libc::c_long > -100000000 {
            n = -n
        } else {
            m = -1i32 - n;
            n = m / 10i32;
            m = m % 10i32 + 1i32;
            k = 1i32 as u8;
            if m < 10i32 {
                dig[0] = m as u8
            } else {
                dig[0] = 0i32 as u8;
                n += 1
            }
        }
    }
    loop {
        dig[k as usize] = (n % 10i32) as u8;
        n = n / 10i32;
        k = k.wrapping_add(1);
        if n == 0i32 {
            break;
        }
    }
    print_the_digs(k);
}
#[no_mangle]
pub unsafe extern "C" fn print_cs(mut p: int32_t) {
    if p < 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 {
        if p >= 1i32 + (0x10ffffi32 + 1i32) {
            if p == 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) {
                print_esc_cstr(b"csname\x00" as *const u8 as *const libc::c_char);
                print_esc_cstr(b"endcsname\x00" as *const u8 as *const libc::c_char);
                print_char(' ' as i32);
            } else {
                print_esc(p - (1i32 + (0x10ffffi32 + 1i32)));
                if (*eqtb.offset(
                    (1i32
                        + (0x10ffffi32 + 1i32)
                        + (0x10ffffi32 + 1i32)
                        + 1i32
                        + 15000i32
                        + 12i32
                        + 9000i32
                        + 1i32
                        + 1i32
                        + 19i32
                        + 256i32
                        + 256i32
                        + 13i32
                        + 256i32
                        + 4i32
                        + 256i32
                        + 1i32
                        + 3i32 * 256i32
                        + (p - (1i32 + (0x10ffffi32 + 1i32)))) as isize,
                ))
                .b32
                .s1 == 11i32
                {
                    print_char(' ' as i32);
                }
            }
        } else if p < 1i32 {
            print_esc_cstr(b"IMPOSSIBLE.\x00" as *const u8 as *const libc::c_char);
        } else {
            print_char(p - 1i32);
        }
    } else if p
        >= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
        && p <= 1i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 1i32
            + 15000i32
            + 12i32
            + 9000i32
            + 1i32
            + 1i32
            + 19i32
            + 256i32
            + 256i32
            + 13i32
            + 256i32
            + 4i32
            + 256i32
            + 1i32
            + 3i32 * 256i32
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + (0x10ffffi32 + 1i32)
            + 85i32
            + 256i32
            + (0x10ffffi32 + 1i32)
            + 23i32
            + 256i32
            - 1i32
        || p > eqtb_top
    {
        print_esc_cstr(b"IMPOSSIBLE.\x00" as *const u8 as *const libc::c_char);
    } else if (*hash.offset(p as isize)).s1 >= str_ptr {
        print_esc_cstr(b"NONEXISTENT.\x00" as *const u8 as *const libc::c_char);
    } else {
        print_esc((*hash.offset(p as isize)).s1);
        print_char(' ' as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sprint_cs(mut p: int32_t) {
    if p < 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) + 1i32 {
        if p < 1i32 + (0x10ffffi32 + 1i32) {
            print_char(p - 1i32);
        } else if p < 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32) {
            print_esc(p - (1i32 + (0x10ffffi32 + 1i32)));
        } else {
            print_esc_cstr(b"csname\x00" as *const u8 as *const libc::c_char);
            print_esc_cstr(b"endcsname\x00" as *const u8 as *const libc::c_char);
        }
    } else {
        print_esc((*hash.offset(p as isize)).s1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_file_name(mut n: int32_t, mut a: int32_t, mut e: int32_t) {
    let mut must_quote: bool = 0i32 != 0;
    let mut quote_char: int32_t = 0i32;
    let mut j: pool_pointer = 0;
    if a != 0i32 {
        j = *str_start.offset((a - 0x10000i32) as isize);
        while (!must_quote || quote_char == 0i32)
            && j < *str_start.offset((a + 1i32 - 0x10000i32) as isize)
        {
            if *str_pool.offset(j as isize) as libc::c_int == ' ' as i32 {
                must_quote = 1i32 != 0
            } else if *str_pool.offset(j as isize) as libc::c_int == '\"' as i32
                || *str_pool.offset(j as isize) as libc::c_int == '\'' as i32
            {
                must_quote = 1i32 != 0;
                quote_char = 73i32 - *str_pool.offset(j as isize) as libc::c_int
            }
            j += 1
        }
    }
    if n != 0i32 {
        j = *str_start.offset((n - 0x10000i32) as isize);
        while (!must_quote || quote_char == 0i32)
            && j < *str_start.offset((n + 1i32 - 0x10000i32) as isize)
        {
            if *str_pool.offset(j as isize) as libc::c_int == ' ' as i32 {
                must_quote = 1i32 != 0
            } else if *str_pool.offset(j as isize) as libc::c_int == '\"' as i32
                || *str_pool.offset(j as isize) as libc::c_int == '\'' as i32
            {
                must_quote = 1i32 != 0;
                quote_char = 73i32 - *str_pool.offset(j as isize) as libc::c_int
            }
            j += 1
        }
    }
    if e != 0i32 {
        j = *str_start.offset((e - 0x10000i32) as isize);
        while (!must_quote || quote_char == 0i32)
            && j < *str_start.offset((e + 1i32 - 0x10000i32) as isize)
        {
            if *str_pool.offset(j as isize) as libc::c_int == ' ' as i32 {
                must_quote = 1i32 != 0
            } else if *str_pool.offset(j as isize) as libc::c_int == '\"' as i32
                || *str_pool.offset(j as isize) as libc::c_int == '\'' as i32
            {
                must_quote = 1i32 != 0;
                quote_char = 73i32 - *str_pool.offset(j as isize) as libc::c_int
            }
            j += 1
        }
    }
    if must_quote {
        if quote_char == 0i32 {
            quote_char = '\"' as i32
        }
        print_char(quote_char);
    }
    if a != 0i32 {
        let mut for_end: int32_t = 0;
        j = *str_start.offset((a - 0x10000i32) as isize);
        for_end = *str_start.offset((a + 1i32 - 0x10000i32) as isize) - 1i32;
        if j <= for_end {
            loop {
                if *str_pool.offset(j as isize) as libc::c_int == quote_char {
                    print(quote_char);
                    quote_char = 73i32 - quote_char;
                    print(quote_char);
                }
                print(*str_pool.offset(j as isize) as int32_t);
                let fresh0 = j;
                j = j + 1;
                if !(fresh0 < for_end) {
                    break;
                }
            }
        }
    }
    if n != 0i32 {
        let mut for_end_0: int32_t = 0;
        j = *str_start.offset((n - 0x10000i32) as isize);
        for_end_0 = *str_start.offset((n + 1i32 - 0x10000i32) as isize) - 1i32;
        if j <= for_end_0 {
            loop {
                if *str_pool.offset(j as isize) as libc::c_int == quote_char {
                    print(quote_char);
                    quote_char = 73i32 - quote_char;
                    print(quote_char);
                }
                print(*str_pool.offset(j as isize) as int32_t);
                let fresh1 = j;
                j = j + 1;
                if !(fresh1 < for_end_0) {
                    break;
                }
            }
        }
    }
    if e != 0i32 {
        let mut for_end_1: int32_t = 0;
        j = *str_start.offset((e - 0x10000i32) as isize);
        for_end_1 = *str_start.offset((e + 1i32 - 0x10000i32) as isize) - 1i32;
        if j <= for_end_1 {
            loop {
                if *str_pool.offset(j as isize) as libc::c_int == quote_char {
                    print(quote_char);
                    quote_char = 73i32 - quote_char;
                    print(quote_char);
                }
                print(*str_pool.offset(j as isize) as int32_t);
                let fresh2 = j;
                j = j + 1;
                if !(fresh2 < for_end_1) {
                    break;
                }
            }
        }
    }
    if quote_char != 0i32 {
        print_char(quote_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_size(mut s: int32_t) {
    if s == 0i32 {
        print_esc_cstr(b"textfont\x00" as *const u8 as *const libc::c_char);
    } else if s == 256i32 {
        print_esc_cstr(b"scriptfont\x00" as *const u8 as *const libc::c_char);
    } else {
        print_esc_cstr(b"scriptscriptfont\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_write_whatsit(mut s: *const libc::c_char, mut p: int32_t) {
    print_esc_cstr(s);
    if (*mem.offset((p + 1i32) as isize)).b32.s0 < 16i32 {
        print_int((*mem.offset((p + 1i32) as isize)).b32.s0);
    } else if (*mem.offset((p + 1i32) as isize)).b32.s0 == 16i32 {
        print_char('*' as i32);
    } else {
        print_char('-' as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn print_native_word(mut p: int32_t) {
    let mut i: int32_t = 0;
    let mut c: int32_t = 0;
    let mut cc: int32_t = 0;
    let mut for_end: int32_t = (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int - 1i32;
    i = 0i32;
    while i <= for_end {
        c = *(&mut *mem.offset((p + 6i32) as isize) as *mut memory_word as *mut u16)
            .offset(i as isize) as int32_t;
        if c >= 0xd800i32 && c < 0xdc00i32 {
            if i < (*mem.offset((p + 4i32) as isize)).b16.s1 as libc::c_int - 1i32 {
                cc = *(&mut *mem.offset((p + 6i32) as isize) as *mut memory_word
                    as *mut u16)
                    .offset((i + 1i32) as isize) as int32_t;
                if cc >= 0xdc00i32 && cc < 0xe000i32 {
                    c = 0x10000i32 + (c - 0xd800i32) * 1024i32 + (cc - 0xdc00i32);
                    print_char(c);
                    i += 1
                } else {
                    print('.' as i32);
                }
            } else {
                print('.' as i32);
            }
        } else {
            print_char(c);
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_sa_num(mut q: int32_t) {
    let mut n: int32_t = 0;
    if ((*mem.offset(q as isize)).b16.s1 as libc::c_int) < 128i32 {
        n = (*mem.offset((q + 1i32) as isize)).b32.s1
    } else {
        n = (*mem.offset(q as isize)).b16.s1 as libc::c_int % 64i32;
        q = (*mem.offset(q as isize)).b32.s1;
        n = n + 64i32 * (*mem.offset(q as isize)).b16.s1 as libc::c_int;
        q = (*mem.offset(q as isize)).b32.s1;
        n = n + 64i32
            * 64i32
            * ((*mem.offset(q as isize)).b16.s1 as libc::c_int
                + 64i32
                    * (*mem.offset((*mem.offset(q as isize)).b32.s1 as isize))
                        .b16
                        .s1 as libc::c_int)
    }
    print_int(n);
}
#[no_mangle]
pub unsafe extern "C" fn print_file_line() {
    let mut level: int32_t = in_open;
    while level > 0i32 && *full_source_filename_stack.offset(level as isize) == 0i32 {
        level -= 1
    }
    if level == 0i32 {
        print_nl_cstr(b"! \x00" as *const u8 as *const libc::c_char);
    } else {
        print_nl_cstr(b"\x00" as *const u8 as *const libc::c_char);
        print(*full_source_filename_stack.offset(level as isize));
        print(':' as i32);
        if level == in_open {
            print_int(line);
        } else {
            print_int(*line_stack.offset((level + 1i32) as isize));
        }
        print_cstr(b": \x00" as *const u8 as *const libc::c_char);
    };
}
/*:251 */
/*:251 */
/*:1660*/
#[no_mangle]
pub unsafe extern "C" fn print_two(mut n: int32_t) {
    n = abs(n) % 100i32;
    print_char('0' as i32 + n / 10i32);
    print_char('0' as i32 + n % 10i32);
}
#[no_mangle]
pub unsafe extern "C" fn print_hex(mut n: int32_t) {
    let mut k: u8 = 0i32 as u8;
    print_char('\"' as i32);
    loop {
        dig[k as usize] = (n % 16i32) as u8;
        n = n / 16i32;
        k = k.wrapping_add(1);
        if !(n != 0i32) {
            break;
        }
    }
    print_the_digs(k);
}
#[no_mangle]
pub unsafe extern "C" fn print_roman_int(mut n: int32_t) {
    let mut u: int32_t = 0;
    let mut v: int32_t = 0;
    let mut roman_data: *const libc::c_char =
        b"m2d5c2l5x2v5i\x00" as *const u8 as *const libc::c_char;
    let mut j: u8 = 0i32 as u8;
    let mut k: u8 = 0i32 as u8;
    v = 1000i32;
    loop {
        while n >= v {
            print_char(*roman_data.offset(j as isize) as int32_t);
            n = n - v
        }
        if n <= 0i32 {
            return;
        }
        k = (j as libc::c_int + 2i32) as u8;
        u = v
            / (*roman_data.offset((k as libc::c_int - 1i32) as isize) as libc::c_int - '0' as i32);
        if *roman_data.offset((k as libc::c_int - 1i32) as isize) as libc::c_int == '2' as i32 {
            k = (k as libc::c_int + 2i32) as u8;
            u = u
                / (*roman_data.offset((k as libc::c_int - 1i32) as isize) as libc::c_int
                    - '0' as i32)
        }
        if n + u >= v {
            print_char(*roman_data.offset(k as isize) as int32_t);
            n = n + u
        } else {
            j = (j as libc::c_int + 2i32) as u8;
            v = v
                / (*roman_data.offset((j as libc::c_int - 1i32) as isize) as libc::c_int
                    - '0' as i32)
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_current_string() {
    let mut j: pool_pointer = *str_start.offset((str_ptr - 0x10000i32) as isize);
    while j < pool_ptr {
        print_char(*str_pool.offset(j as isize) as int32_t);
        j += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_scaled(mut s: scaled_t) {
    let mut delta: scaled_t = 0;
    if s < 0i32 {
        print_char('-' as i32);
        s = -s
    }
    print_int(s / 0x10000i32);
    print_char('.' as i32);
    s = 10i32 * (s % 0x10000i32) + 5i32;
    delta = 10i32;
    loop {
        if delta > 0x10000i32 {
            s = s + 0x8000i32 - 50000i32
        }
        print_char('0' as i32 + s / 0x10000i32);
        s = 10i32 * (s % 0x10000i32);
        delta = delta * 10i32;
        if !(s > delta) {
            break;
        }
    }
}
