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
    static mut arith_error: bool;
    #[no_mangle]
    static mut tex_remainder: scaled_t;
}
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type scaled_t = int32_t;
/* tectonic/xetex-scaledmath.c: low-level math functions
   Copyright 2017 The Tectonic Project
   Licensed under the MIT License.
*/
#[no_mangle]
pub unsafe extern "C" fn tex_round(mut r: libc::c_double) -> int32_t {
    /* We must reproduce very particular rounding semantics to pass the TRIP
     * test. Specifically, values within the 32-bit range of TeX integers are
     * rounded to the nearest integer with half-integral values going away
     * from zero: 0.5 => 1, -0.5 => -1.
     *
     * `r` does not necessarily lie within the range of a 32-bit TeX integer;
     * if it doesn't, we clip. The following LaTeX document allegedly triggers
     * that codepath:
     *
     *   \documentstyle{article}
     *   \begin{document}
     *   \begin{flushleft}
     *   $\hbox{} $\hfill
     *   \filbreak
     *   \eject
     *
     */
    if r > 2147483647.0f64 {
        /* 0x7FFFFFFF */
        return 2147483647i32;
    }
    if r < -2147483648.0f64 {
        /* -0x80000000 */
        return -2147483648i64 as int32_t;
    }
    /* ANSI defines the float-to-integer cast to truncate towards zero, so the
     * following code is all that's necessary to get the desired behavior. The
     * truncation technically causes an uncaught "inexact" floating-point
     * exception, but exception is virtually impossible to avoid in real
     * code. */
    if r >= 0.0f64 {
        return (r + 0.5f64) as int32_t;
    }
    return (r - 0.5f64) as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn half(mut x: int32_t) -> int32_t {
    if x & 1i32 != 0 {
        return (x + 1i32) / 2i32;
    }
    return x / 2i32;
}
#[no_mangle]
pub unsafe extern "C" fn mult_and_add(
    mut n: int32_t,
    mut x: scaled_t,
    mut y: scaled_t,
    mut max_answer: scaled_t,
) -> scaled_t {
    if n < 0i32 {
        x = -x;
        n = -n
    }
    if n == 0i32 {
        return y;
    } else if x <= (max_answer - y) / n && -x <= (max_answer + y) / n {
        return n * x + y;
    } else {
        arith_error = 1i32 != 0;
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn x_over_n(mut x: scaled_t, mut n: int32_t) -> scaled_t {
    if n == 0i32 {
        arith_error = 1i32 != 0;
        tex_remainder = x;
        return 0i32;
    } else {
        if n < 0i32 {
            // negative
            x = -x;
            n = -n;
            tex_remainder = -tex_remainder
        }
        if x >= 0i32 {
            tex_remainder = x % n;
            return x / n;
        } else {
            tex_remainder = -(-x % n);
            return -(-x / n);
        }
    };
}
/* xetex-errors */
/* xetex-math */
/* xetex-output */
/* xetex-pagebuilder */
/* xetex-scaledmath */
#[no_mangle]
pub unsafe extern "C" fn xn_over_d(mut x: scaled_t, mut n: int32_t, mut d: int32_t) -> scaled_t {
    let mut positive: bool = false;
    let mut t: int32_t = 0;
    let mut u: int32_t = 0;
    let mut v: int32_t = 0;
    if x >= 0i32 {
        positive = 1i32 != 0
    } else {
        x = -x;
        positive = 0i32 != 0
    }
    t = (x as libc::c_long % 32768 * n as libc::c_long) as int32_t;
    u = (x as libc::c_long / 32768 * n as libc::c_long + t as libc::c_long / 32768)
        as int32_t;
    v = ((u % d) as libc::c_long * 32768 + t as libc::c_long % 32768) as int32_t;
    if (u / d) as libc::c_long >= 32768 {
        arith_error = 1i32 != 0
    } else {
        u = (32768 * (u / d) as libc::c_long + (v / d) as libc::c_long) as int32_t
    }
    if positive {
        tex_remainder = v % d;
        return u;
    } else {
        tex_remainder = -(v % d);
        return -u;
    };
}
#[no_mangle]
pub unsafe extern "C" fn round_xn_over_d(
    mut x: scaled_t,
    mut n: int32_t,
    mut d: int32_t,
) -> scaled_t {
    let mut positive: bool = false;
    let mut t: int32_t = 0;
    let mut u: int32_t = 0;
    let mut v: int32_t = 0;
    if x >= 0i32 {
        positive = 1i32 != 0
    } else {
        x = -x;
        positive = 0i32 != 0
    }
    t = (x as libc::c_long % 32768 * n as libc::c_long) as int32_t;
    u = (x as libc::c_long / 32768 * n as libc::c_long + t as libc::c_long / 32768)
        as int32_t;
    v = ((u % d) as libc::c_long * 32768 + t as libc::c_long % 32768) as int32_t;
    if (u / d) as libc::c_long >= 32768 {
        arith_error = 1i32 != 0
    } else {
        u = (32768 * (u / d) as libc::c_long + (v / d) as libc::c_long) as int32_t
    }
    v = v % d;
    if 2i32 * v >= d {
        u += 1
    }
    if positive {
        return u;
    } else {
        return -u;
    };
}
