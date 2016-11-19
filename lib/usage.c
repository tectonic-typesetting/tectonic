/* usage.c: Output a help message (from help.h).

   Modified in 2001 by O. Weber.
   Written in 1995 by K. Berry.  Public domain.  */

#include <w2c/config.h>

/* Call usage if the program exits with an "usage error".  STR is supposed
   to be the program name. */

void
usage (const_string str)
{
  fprintf (stderr, "Try `%s --help' for more information.\n", str);
  uexit (1);
}

/* Call usage if the program exits by printing the help message.
   MESSAGE is a NULL-terminated array of strings which make up the
   help message.  Each string is printed on a separate line.
   We use arrays instead of a single string to work around compiler
   limitations (sigh).
*/
void
usagehelp (const_string *message, const_string bug_email)
{
    if (!bug_email)
        bug_email = "tex-k@tug.org";
    while (*message) {
        printf("%s\n", *message);
        ++message;
    }
    printf("\nEmail bug reports to %s.\n", bug_email);
    uexit(0);
}
