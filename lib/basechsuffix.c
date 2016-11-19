/* basechsuffix.c: replace the last bit of a filename with something else.

   Written in 1995 by Karl Berry.  Public domain.  */

#include <w2c/config.h>
#include "lib.h"

/* Return the basename of NAME, with trailing characters OLD replaced by
   NEW.  (If last characters in NAME are not OLD, just append NEW.)
   Since this is used to turn, e.g., foo/cmr10.300pk -> cmr10.300gf,
   don't assume a `.' preceding OLD or NEW.
   
   In other words, we're implementing `basename NAME OLD`NEW.  */

string
basenamechangesuffix (const_string name,  const_string old_suffix,
                      const_string new_suffix)
{
  string answer;
  unsigned c;
  const_string base = xbasename (name);
  unsigned base_len = strlen (base);
  unsigned copy_limit = base_len;
  unsigned old_suffix_len = strlen (old_suffix);
  
  if (old_suffix_len <= base_len) {
    for (c = 0; c < old_suffix_len; c++) {
      if (!FILECHARCASEEQ (old_suffix[old_suffix_len - c - 1],
                       base[base_len - c - 1]))
        break;
    }
    if (c == old_suffix_len) {
      copy_limit -= old_suffix_len;
    }
  }
  
  answer = xmalloc (copy_limit + strlen (new_suffix) + 1);
  strncpy (answer, base, copy_limit);
  answer[copy_limit] = 0;
  strcat (answer, new_suffix);

  return answer;
}
