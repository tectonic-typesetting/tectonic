/* tectonic/mathutil.c: low-level math functions
   Copyright 2017 The Tectonic Project
   Licensed under the MIT License.
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>


integer
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
        return (integer) (r + 0.5);

    return (integer) (r - 0.5);
}


integer
half(integer x)
{
    if (odd(x))
        return (x + 1) / 2;
    return x / 2;
}


scaled
mult_and_add(integer n, scaled x, scaled y, scaled max_answer)
{
    if (n < 0) {
        x = -(integer) x;
        n = -(integer) n;
    }

    if (n == 0)
        return y;
    else if (x <= (max_answer - y) / n && (-(integer) x <= (max_answer + y) / n))
        return n * x + y;
    else {
        arith_error = true;
        return 0;
    }
}


scaled
x_over_n(scaled x, integer n)
{
    if (n == 0) {
        arith_error = true;
        tex_remainder = x;
        return 0;
    } else {
        if (n < 0) {
            // negative
            x = -(integer) x;
            n = -(integer) n;
            tex_remainder = -(integer) tex_remainder;
        }

        if (x >= 0) {
            tex_remainder = x % n;
            return x / n;
        } else {
            tex_remainder = -(integer) ((-(integer) x) % n);
            return -(integer) ((-(integer) x) / n);
        }
    }
}


scaled
xn_over_d(scaled x, integer n, integer d)
{
    bool positive;
    integer t, u, v;

    if (x >= 0)
        positive = true;
    else {
        x = -(integer) x;
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
        tex_remainder = -(integer) (v % d);
        return -(integer) u;
    }
}


scaled
round_xn_over_d(scaled x, integer n, integer d)
{
    bool positive;
    integer t, u, v;

    if (x >= 0) {
        positive = true;
    } else {
        x = -(integer) x;
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
        return -(integer) u;
}
