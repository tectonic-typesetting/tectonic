/* tectonic/mathutil.c: low-level math functions
   Copyright 2017 The Tectonic Project
   Licensed under the MIT License.
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>


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

    if (r > 2147483647.0)
	return 2147483647;

    if (r < -2147483648.0)
	return -2147483648;

    if (r >= 0.0)
	return (integer) (r + 0.5);

    return (integer) (r - 0.5);
}
