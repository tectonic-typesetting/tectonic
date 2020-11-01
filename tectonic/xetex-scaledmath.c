/* tectonic/xetex-scaledmath.c: low-level math functions
   Copyright 2017 The Tectonic Project
   Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"


int32_t
tex_round (double r)
{
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

    if (r > 2147483647.0) /* 0x7FFFFFFF */
        return 2147483647;

    if (r < -2147483648.0) /* -0x80000000 */
        return -2147483648;

    /* ANSI defines the float-to-int32_t cast to truncate towards zero, so the
     * following code is all that's necessary to get the desired behavior. The
     * truncation technically causes an uncaught "inexact" floating-point
     * exception, but exception is virtually impossible to avoid in real
     * code. */

    if (r >= 0.0)
        return (int32_t) (r + 0.5);

    return (int32_t) (r - 0.5);
}


int32_t
half(int32_t x)
{
    if (odd(x))
        return (x + 1) / 2;
    return x / 2;
}


scaled_t
mult_and_add(int32_t n, scaled_t x, scaled_t y, scaled_t max_answer)
{
    if (n < 0) {
        x = -(int32_t) x;
        n = -(int32_t) n;
    }

    if (n == 0)
        return y;
    else if (x <= (max_answer - y) / n && (-(int32_t) x <= (max_answer + y) / n))
        return n * x + y;
    else {
        arith_error = true;
        return 0;
    }
}


scaled_t
x_over_n(scaled_t x, int32_t n)
{
    if (n == 0) {
        arith_error = true;
        tex_remainder = x;
        return 0;
    } else {
        if (n < 0) {
            // negative
            x = -(int32_t) x;
            n = -(int32_t) n;
            tex_remainder = -(int32_t) tex_remainder;
        }

        if (x >= 0) {
            tex_remainder = x % n;
            return x / n;
        } else {
            tex_remainder = -(int32_t) ((-(int32_t) x) % n);
            return -(int32_t) ((-(int32_t) x) / n);
        }
    }
}


scaled_t
xn_over_d(scaled_t x, int32_t n, int32_t d)
{
    bool positive;
    int32_t t, u, v;

    if (x >= 0)
        positive = true;
    else {
        x = -(int32_t) x;
        positive = false;
    }

    t = (x % 32768L) * n;
    u = (x / 32768L) * n + (t / 32768L);
    v = (u % d) * 32768L + (t % 32768L);

    if (u / d >= 32768L)
        arith_error = true;
    else
        u = 32768L * (u / d) + (v / d);

    if (positive) {
        tex_remainder = v % d;
        return u;
    } else {
        tex_remainder = -(int32_t) (v % d);
        return -(int32_t) u;
    }
}


scaled_t
round_xn_over_d(scaled_t x, int32_t n, int32_t d)
{
    bool positive;
    int32_t t, u, v;

    if (x >= 0) {
        positive = true;
    } else {
        x = -(int32_t) x;
        positive = false;
    }
    t = (x % 32768L) * n;
    u = (x / 32768L) * n + (t / 32768L);
    v = (u % d) * 32768L + (t % 32768L);
    if (u / d >= 32768L)
        arith_error = true;
    else
        u = 32768L * (u / d) + (v / d);
    v = v % d;
    if (2 * v >= d)
        u++;
    if (positive)
        return u;
    else
        return -(int32_t) u;
}

static int32_t
make_frac(int32_t p, int32_t q)
{
    int32_t f;
    int32_t n;
    bool negative;
    int32_t be_careful;

    if (p >= 0)
        negative = false;
    else {
        p = -p;
        negative = true;
    }

    if (q <= 0) {
        q = -q;
        negative = !negative;
    }

    n = p / q;
    p = p % q;

    if (n >= 8) {
        arith_error = true;
        if (negative)
            return -0x7FFFFFFF;
        else
            return 0x7FFFFFFF;
    } else {
        n = (n - 1) * 0x10000000;
        f = 1;

        do {
            be_careful = p - q;
            p = be_careful + p;
            if (p >= 0)
                f = f + f + 1;
            else {
                f = f + f;
                p = p + q;
            }
        } while (f < 0x10000000);

        be_careful = p - q;
        if (be_careful + p >= 0)
            f += 1;

        if (negative)
            return -(f + n);
        else
            return f + n;
    }
}

static int32_t
take_frac(int32_t q, int32_t f)
{
    int32_t p;
    bool negative;
    int32_t n;
    int32_t be_careful;

    if (f >= 0)
        negative = false;
    else {
        f = -f;
        negative = true;
    }

    if (q < 0) {
        q = -q;
        negative = !negative;
    }

    if (f < 0x10000000)
        n = 0;
    else {
        n = f / 0x10000000;
        f = f % 0x10000000;

        if (q <= 0x7FFFFFFF / n)
            n = n * q;
        else {
            arith_error = true;
            n = 0x7FFFFFFF;
        }
    }

    f = f + 0x10000000;
    p = 0x08000000;

    if (q < 0x40000000) {
        do {
            if (odd(f))
                p = (p + q) / 2;
            else
                p = p / 2;
            f = f / 2;
        } while (f != 1);
    } else {
        do {
            if (odd(f))
                p = p + (q - p) / 2;
            else
                p = p / 2;
            f = f / 2;
        } while (f != 1); /*:120 */
    }

    be_careful = n - 0x7FFFFFFF;
    if (be_careful + p > 0) {
        arith_error = true;
        n = 0x7FFFFFFF - p;
    }

    if (negative)
        return -(n + p);
    else
        return n + p;
}

static int32_t
m_log(int32_t x)
{
    int32_t y, z;
    int32_t k;

    if (x <= 0) { /*125: */
        error_here_with_diagnostic("Logarithm of ");
        print_scaled(x);
        print_cstr(" has been replaced by 0");
        capture_to_diagnostic(NULL);
        help_ptr = 2;
        help_line[1] = "Since I don't take logs of non-positive numbers,";
        help_line[0] = "I'm zeroing this one. Proceed, with fingers crossed.";
        error();
        return 0;
    } else {
        y = 1302456860L;
        z = 6581195L;

        while (x < 0x40000000) {
            x = x + x;
            y = y - 93032639L;
            z = z - 48782L;
        }

        y = y + (z / 65536L);
        k = 2;

        while (x > 0x40000004) { /*124: */
            z = ((x - 1) / two_to_the[k]) + 1;

            while (x < 0x40000000 + z) {
                z = (z + 1) / 2;
                k = k + 1;
            }

            y = y + spec_log[k];
            x = x - z;
        }

        return y / 8;
    }
}

static int32_t
ab_vs_cd(int32_t a, int32_t b, int32_t c, int32_t d)
{
    int32_t q, r;

    if (a < 0) {
        a = -a;
        b = -b;
    }

    if (c < 0) {
        c = -c;
        d = -d;
    }

    if (d <= 0) {
        if (b >= 0) {
            if ((a == 0 || b == 0) && (c == 0 || d == 0)) {
                return 0;
            } else {
                return 1;
            }
        }

        if (d == 0) {
            if (a == 0) {
                return 0;
            } else {
                return -1;
            }
        }

        q = a;
        a = c;
        c = q;
        q = -b;
        b = -d;
        d = q;
    } else if (b <= 0) {
        if (b < 0) {
            if (a > 0) {
                return -1;
            }
        }

        if (c == 0) {
            return 0;
        } else {
            return -1;
        }
    }

    while (true) {
        q = a / d;
        r = c / b;

        if (q != r) {
            if (q > r) {
                return 1;
            } else {
                return -1;
            }
        }

        q = a % d;
        r = c % b;

        if (r == 0) {
            if (q == 0) {
                return 0;
            } else {
                return 1;
            }
        }

        if (q == 0) {
            return -1;
        }

        a = b;
        b = q;
        c = d;
        d = r;
    }
}

static void
new_randoms(void)
{
    unsigned char k;
    int32_t x;

    for (k = 0; k < 24; k++) {
        x = randoms[k] - randoms[k + 31];
        if (x < 0)
            x = x + 0x10000000;
        randoms[k] = x;
    }

    for (k = 24; k < 55; k++) {
        x = randoms[k] - randoms[k - 24];
        if (x < 0)
            x = x + 0x10000000;
        randoms[k] = x;
    }

    j_random = 54;
}

void
init_randoms(int32_t seed)
{
    int32_t j, jj, k;
    unsigned char i;

    j = abs(seed);

    while (j >= 0x10000000)
        j = j / 2;

    k = 1;

    for (i = 0; i < 55; i++) {
        jj = k;
        k = j - k;
        j = jj;
        if (k < 0)
            k = k + 0x10000000;
        randoms[(i * 21) % 55] = j;
    }

    new_randoms();
    new_randoms();
    new_randoms();
}

int32_t
unif_rand(int32_t x)
{
    int32_t y;

    if (j_random == 0)
        new_randoms();
    else
        j_random--;

    y = take_frac(abs(x), randoms[j_random]);
    if (y == abs(x))
        return 0;
    else if (x > 0)
        return y;
    else
        return -y;
}

int32_t
norm_rand(void)
{
    int32_t x, u, l;

    do {
        do {
            if (j_random == 0)
                new_randoms();
            else
                j_random--;

            x = take_frac(112429L, randoms[j_random] - 0x08000000);

            if (j_random == 0)
                new_randoms();
            else
                j_random--;

            u = randoms[j_random];
        } while (abs(x) >= u);

        x = make_frac(x, u);
        l = 139548960L - m_log(u);
    } while (ab_vs_cd(1024, l, x, x) < 0);

    return x;
}
