/* input2int.c: read two or three integers from text files.  These
   routines are only used in patgen.  Public domain. */

#include <w2c/config.h>
#include "lib.h"

#ifdef INTEGER_IS_INT
#define SCAN2INT "%d %d"
#define SCAN3INT "%d %d %d"
#else
#define SCAN2INT "%ld %ld"
#define SCAN3INT "%ld %ld %ld"
#endif

/* Read two integers from stdin.  */

void
zinput2ints (integer *a,  integer *b)
{
  int ch;

  while (scanf (SCAN2INT, a, b) != 2)
    {
      while ((ch = getchar ()) != EOF && ch != '\n');
      if (ch == EOF) return;
      fprintf (stderr, "Please enter two integers.\n");
    }

  while ((ch = getchar ()) != EOF && ch != '\n');
}


/* Read three integers from stdin.  */

void
zinput3ints (integer *a,  integer *b,  integer *c)
{
  int ch;

  while (scanf (SCAN3INT, a, b, c) != 3)
    {
      while ((ch = getchar ()) != EOF && ch != '\n');
      if (ch == EOF) return;
      fprintf (stderr, "Please enter three integers.\n");
    }

  while ((ch = getchar ()) != EOF && ch != '\n');
}
