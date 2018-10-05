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

    if (r > 2147483647.0) /* 0x7FFFFFFF */
        return 2147483647;

    if (r < -2147483648.0) /* -0x80000000 */
        return -2147483648;

    /* ANSI defines the float-to-integer cast to truncate towards zero, so the
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
