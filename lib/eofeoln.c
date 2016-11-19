/* eofeoln.c: implement Pascal's ideas for end-of-file and end-of-line
   testing.  Public domain. */

#include <w2c/config.h>
#include "lib.h"

/* Return true if we're at the end of FILE, else false.  This implements
   Pascal's `eof' builtin.  */

boolean
eof (FILE *file)
{
  register int c;

  /* If FILE doesn't exist, return true. This happens, for example,
     when a user does `mft foo.mf' -- there's no change file,
     so we never open it, so we end up calling this with a null pointer. */
  if (!file)
    return true;
    
  /* Maybe we're already at the end?  */
  if (feof (file))
    return true;

  if ((c = getc (file)) == EOF)
    return true;

  /* We weren't at the end.  Back up.  */
  (void) ungetc (c, file);

  return false;
}


/* Return true on end-of-line in FILE or at the end of FILE, else false.  */
/* Accept both CR and LF as end-of-line. */

boolean
eoln (FILE *file)
{
  register int c;

  if (feof (file))
    return true;
  
  c = getc (file);
  
  if (c != EOF)
    (void) ungetc (c, file);
    
  return c == '\n' || c == '\r' || c == EOF;
}

/* Consume input up and including the first eol encountered. */
/* Handle CRLF as a single end-of-line. */

void
readln (FILE *f)
{
    int c;
    while ((c = getc (f)) != '\n' && c != '\r' && c != EOF)
        ;
    if (c == '\r' && (c = getc (f)) != '\n' && c != EOF)
        ungetc (c, f);
}
