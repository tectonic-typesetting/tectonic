/* setupvar.c: Look up a value in texmf.cnf or use default.

   Adapted in 2010 by Peter Breitenlohner.  Public domain.
   Written in 1995 by Karl Berry.  Public domain.  */

#include "w2c-config.h"
#include "lib.h"

/* Look up VAR_NAME in texmf.cnf; assign either the value found there or
   DFLT to *VAR.  */

void
setup_bound_variable (integer *var, const_string var_name, integer dflt)
{
  *var = dflt;
}
