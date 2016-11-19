/* zround.c: round R to the nearest whole number.  This is supposed to
   implement the predefined Pascal round function.  Public domain. */

#include <w2c/config.h>
#include "lib.h"

integer
zround (double r)
{
  integer i;

  /* R can be outside the range of an integer if glue is stretching or
     shrinking a lot.  We can't do any better than returning the largest
     or smallest integer possible in that case.  It doesn't seem to make
     any practical difference.  Here is a sample input file which
     demonstrates the problem, from phil@cs.arizona.edu:
     	\documentstyle{article}
	\begin{document}
	\begin{flushleft}
	$\hbox{} $\hfill 
	\filbreak
	\eject
    
     djb@silverton.berkeley.edu points out we should testing against
     TeX's largest or smallest integer (32 bits), not the machine's.  So
     we might as well use a floating-point constant, and avoid potential
     compiler bugs (also noted by djb, on BSDI).  */
  if (r > 2147483647.0)
    i = 2147483647;
  /* should be ...8, but atof bugs are too common */
  else if (r < -2147483647.0)
    i = -2147483647;
  /* Admittedly some compilers don't follow the ANSI rules of casting
     meaning truncating toward zero; but it doesn't matter enough to do
     anything more complicated here.  */
  else if (r >= 0.0)
    i = (integer)(r + 0.5);
  else
    i = (integer)(r - 0.5);

  return i;
}
