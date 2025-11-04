use crate::c_api::errors::{ffi_abort, rs_error, EngineError};
use crate::c_api::globals::Globals;
use crate::c_api::output::{
    rs_capture_to_diagnostic, rs_error_here_with_diagnostic, rs_print_bytes, rs_print_scaled,
};
use crate::ty::Scaled;
use std::cell::RefCell;
use std::mem;

thread_local! {
    static MATH_CTX: RefCell<MathCtx> = const { RefCell::new(MathCtx::new()) };
}

const POWERS_OF_TWO: [i32; 31] = {
    let mut out = [1; 31];
    let mut i = 1;
    while i < 31 {
        out[i] = out[i - 1] * 2;
        i += 1;
    }
    out
};

const SPEC_LOG: [i32; 29] = {
    let mut out = [0; 29];
    out[1] = 93032640;
    out[2] = 38612034;
    out[3] = 17922280;
    out[4] = 8662214;
    out[5] = 4261238;
    out[6] = 2113709;
    out[7] = 1052693;
    out[8] = 525315;
    out[9] = 262400;
    out[10] = 131136;
    out[11] = 65552;
    out[12] = 32772;
    out[13] = 16385;
    let mut k = 14;
    while k < 28 {
        out[k] = POWERS_OF_TWO[27 - k];
        k += 1;
    }
    out[28] = 1;
    out
};

pub struct MathCtx {
    arith_error: bool,
    tex_remainder: Scaled,
    randoms: [i32; 55],
    j_random: u8,
}

impl MathCtx {
    const fn new() -> MathCtx {
        MathCtx {
            arith_error: false,
            tex_remainder: 0,
            randoms: [0; 55],
            j_random: 0,
        }
    }

    pub fn with<T>(f: impl FnOnce(&mut MathCtx) -> T) -> T {
        MATH_CTX.with_borrow_mut(f)
    }

    fn next_rand(&mut self) -> i32 {
        if self.j_random == 0 {
            new_randoms(self);
        } else {
            self.j_random -= 1;
        }
        self.randoms[self.j_random as usize]
    }
}

c_var!(MathCtx => arith_error: bool);
c_var!(MathCtx => tex_remainder: Scaled);

#[no_mangle]
pub extern "C" fn randoms(idx: usize) -> i32 {
    MATH_CTX.with_borrow(|ctx| ctx.randoms[idx])
}

c_var!(MathCtx => j_random: u8);

#[no_mangle]
pub extern "C" fn tex_round(r: f64) -> i32 {
    /* We must reproduce very particular rounding semantics to pass the TRIP
     * test. Specifically, values within the 32-bit range of TeX int32_ts are
     * rounded to the nearest int32_t with half-integral values going away
     * from zero: 0.5 => 1, -0.5 => -1.
     *
     * `r` does not necessarily lie within the range of a 32-bit TeX int32_t;
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

    if r > 2147483647.0
    /* 0x7FFFFFFF */
    {
        return 2147483647;
    }

    if r < -2147483648.0
    /* -0x80000000 */
    {
        return -2147483648;
    }

    /* ANSI defines the float-to-int32_t cast to truncate towards zero, so the
     * following code is all that's necessary to get the desired behavior. The
     * truncation technically causes an uncaught "inexact" floating-point
     * exception, but exception is virtually impossible to avoid in real
     * code. */

    if r >= 0.0 {
        (r + 0.5) as i32
    } else {
        (r - 0.5) as i32
    }
}

#[no_mangle]
pub extern "C" fn half(x: i32) -> i32 {
    if x % 2 != 0 {
        (x + 1) / 2
    } else {
        x / 2
    }
}

#[no_mangle]
pub extern "C" fn mult_and_add(n: i32, x: Scaled, y: Scaled, max_answer: Scaled) -> Scaled {
    if n < 0 {
        mult_and_add(-n, -x, y, max_answer)
    } else if n == 0 {
        y
    } else if x <= (max_answer - y) / n && (-(x) <= (max_answer + y) / n) {
        n * x + y
    } else {
        set_arith_error(true);
        0
    }
}

#[no_mangle]
pub extern "C" fn x_over_n(x: Scaled, n: i32) -> Scaled {
    if n == 0 {
        set_arith_error(true);
        set_tex_remainder(x);
        0
    } else if n < 0 {
        set_tex_remainder(-tex_remainder());
        x_over_n(-x, -n)
    } else {
        set_tex_remainder(x % n);
        x / n
    }
}

#[no_mangle]
pub extern "C" fn xn_over_d(mut x: Scaled, n: i32, d: i32) -> Scaled {
    let pos = x >= 0;
    if !pos {
        x = -x;
    }
    let t = (x % 32768) * n;
    let mut u = (x / 32768) * n + (t / 32768);
    let v = (u % d) * 32768 + (t % 32768);

    if u / d >= 32768 {
        set_arith_error(true);
    } else {
        u = 32768 * (u / d) + (v / d);
    }

    if pos {
        set_tex_remainder(v % d);
        u
    } else {
        set_tex_remainder(-(v % d));
        -u
    }
}

#[no_mangle]
pub extern "C" fn round_xn_over_d(mut x: Scaled, n: i32, d: i32) -> Scaled {
    let pos = x >= 0;
    if !pos {
        x = -x;
    }
    let t = (x % 32768) * n;
    let mut u = (x / 32768) * n + (t / 32768);
    let mut v = (u % d) * 32768 + (t % 32768);

    if u / d >= 32768 {
        set_arith_error(true);
    } else {
        u = 32768 * (u / d) + (v / d);
    }
    v %= d;
    if 2 * v >= d {
        u += 1;
    }
    if pos {
        u
    } else {
        -u
    }
}

pub fn make_frac(math: &mut MathCtx, mut p: i32, mut q: i32) -> i32 {
    let mut neg = p < 0;
    if neg {
        p = -p;
    }

    if q <= 0 {
        q = -q;
        neg = !neg;
    }

    let mut n = p / q;
    p %= q;

    if n >= 8 {
        math.arith_error = true;
        if neg {
            -0x7FFFFFFF
        } else {
            0x7FFFFFFF
        }
    } else {
        n = (n - 1) * 0x10000000;
        let mut f = 1;

        let mut be_careful;
        loop {
            be_careful = p - q;
            p += be_careful;
            if p >= 0 {
                f = f + f + 1;
            } else {
                f = f + f;
                p += q;
            }

            if f >= 0x10000000 {
                break;
            }
        }

        be_careful = p - q;
        if be_careful + p >= 0 {
            f += 1;
        }

        if neg {
            -(f + n)
        } else {
            f + n
        }
    }
}

pub fn take_frac(math: &mut MathCtx, mut q: i32, mut f: i32) -> i32 {
    let mut neg = f < 0;

    if neg {
        f = -f;
    }

    if q < 0 {
        q = -q;
        neg = !neg;
    }

    let mut n;
    if f < 0x10000000 {
        n = 0;
    } else {
        n = f / 0x10000000;
        f %= 0x10000000;

        if q <= 0x7FFFFFFF / n {
            n *= q;
        } else {
            math.arith_error = true;
            n = 0x7FFFFFFF;
        }
    }

    f += 0x10000000;
    let mut p = 0x08000000;

    if q < 0x40000000 {
        loop {
            if f % 2 != 0 {
                p = (p + q) / 2;
            } else {
                p /= 2;
            }
            f /= 2;
            if f == 1 {
                break;
            }
        }
    } else {
        loop {
            if f % 2 != 0 {
                p = p + (q - p) / 2;
            } else {
                p /= 2;
            }
            f /= 2;
            if f == 1 {
                break;
            }
        }
    }

    let be_careful = n - 0x7FFFFFFF;

    if be_careful + p > 0 {
        math.arith_error = true;
        n = 0x7FFFFFFF - p;
    }

    if neg {
        -(n + p)
    } else {
        n + p
    }
}

pub fn ab_vs_cd(mut a: i32, mut b: i32, mut c: i32, mut d: i32) -> i32 {
    if a < 0 {
        return ab_vs_cd(-a, -b, c, d);
    } else if c < 0 {
        return ab_vs_cd(a, b, -c, -d);
    }

    if d <= 0 {
        if b >= 0 {
            return if a == 0 || b == 0 && c == 0 || d == 0 {
                0
            } else {
                1
            };
        } else if d == 0 {
            return if a == 0 { 0 } else { -1 };
        } else {
            mem::swap(&mut a, &mut c);
            mem::swap(&mut b, &mut d);
            b = -b;
            d = -d;
        }
    } else if b <= 0 {
        if b < 0 && a > 0 {
            return -1;
        }

        return if c == 0 { 0 } else { -1 };
    }

    loop {
        let mut q = a / d;
        let mut r = c / b;

        if q != r {
            return if q > r { 1 } else { -1 };
        }

        q = a % d;
        r = c % b;

        if r == 0 {
            return if q == 0 { 0 } else { 1 };
        }

        if q == 0 {
            return -1;
        }

        a = b;
        b = q;
        c = d;
        d = r;
    }
}

fn new_randoms(math: &mut MathCtx) {
    for k in 0..24 {
        let mut x = math.randoms[k] - math.randoms[k + 31];
        if x < 0 {
            x += 0x10000000;
        }
        math.randoms[k] = x;
    }

    for k in 24..55 {
        let mut x = math.randoms[k] - math.randoms[k - 24];
        if x < 0 {
            x += 0x10000000;
        }
        math.randoms[k] = x;
    }

    math.j_random = 54;
}

#[no_mangle]
pub extern "C" fn init_randoms(seed: i32) {
    MATH_CTX.with_borrow_mut(|math| {
        let mut j = seed.abs();
        while j >= 0x10000000 {
            j /= 2;
        }

        let mut k = 1;

        for i in 0..55 {
            let jj = k;
            k = j - k;
            j = jj;
            if k < 0 {
                k += 0x10000000;
            }
            math.randoms[(i * 21) % 55] = j;
        }

        new_randoms(math);
        new_randoms(math);
        new_randoms(math);
    })
}

#[no_mangle]
pub extern "C" fn unif_rand(x: i32) -> i32 {
    MATH_CTX.with_borrow_mut(|math| {
        let next_rand = math.next_rand();
        let y = take_frac(math, x.abs(), next_rand);
        if y == x.abs() {
            0
        } else if x > 0 {
            y
        } else {
            -y
        }
    })
}

pub fn rs_norm_rand(globals: &mut Globals<'_, '_>) -> Result<i32, EngineError> {
    let mut x;

    loop {
        let mut u;

        loop {
            let next_rand = globals.math.next_rand() - 0x08000000;
            x = take_frac(globals.math, 112429, next_rand);

            u = globals.math.next_rand();

            if x.abs() < u {
                break;
            }
        }

        x = make_frac(globals.math, x, u);
        let l = 139548960 - rs_m_log(globals, u)?;

        if ab_vs_cd(1024, l, x, x) >= 0 {
            break;
        }
    }

    Ok(x)
}

#[no_mangle]
pub extern "C-unwind" fn norm_rand() -> i32 {
    let res = Globals::with(|globals| rs_norm_rand(globals));
    ffi_abort(res)
}

pub fn rs_m_log(globals: &mut Globals<'_, '_>, mut x: i32) -> Result<i32, EngineError> {
    if (x <= 0) {
        /*125: */
        rs_error_here_with_diagnostic(globals, b"Logarithm of ");
        rs_print_scaled(globals, x);
        rs_print_bytes(globals, b" has been replaced by 0");
        rs_capture_to_diagnostic(globals, None);
        globals.engine.help_ptr = 2;
        globals.engine.help_line[1] = c"Since I don't take logs of non-positive numbers,".as_ptr();
        globals.engine.help_line[0] =
            c"I'm zeroing this one. Proceed, with fingers crossed.".as_ptr();
        rs_error(globals)?;
        Ok(0)
    } else {
        let mut y = 1302456860;
        let mut z = 6581195;

        while (x < 0x40000000) {
            x = x + x;
            y = y - 93032639;
            z = z - 48782;
        }

        y = y + (z / 65536);
        let mut k = 2;

        while (x > 0x40000004) {
            /*124: */
            z = ((x - 1) / POWERS_OF_TWO[k]) + 1;

            while (x < 0x40000000 + z) {
                z = (z + 1) / 2;
                k = k + 1;
            }

            y = y + SPEC_LOG[k];
            x = x - z;
        }

        Ok(y / 8)
    }
}

#[no_mangle]
pub extern "C-unwind" fn m_log(x: i32) -> i32 {
    let res = Globals::with(|globals| rs_m_log(globals, x));
    ffi_abort(res)
}
