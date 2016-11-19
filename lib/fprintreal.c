/* fprintreal.c: print the real number R in the Pascal format N:M on the
   file F.  Public domain. */

#include <w2c/config.h>
#include "lib.h"

void
fprintreal (FILE *f, double r, int n, int m)
{
  char fmt[50];  /* Surely enough, since N and M won't be more than 25
                    digits each!  */

  sprintf (fmt, "%%%d.%dlf", n, m);
  fprintf (f, fmt, r);
}
