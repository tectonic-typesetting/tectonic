/* Collected kpathsea files in the tidied workalike version.

   Copyright 1993, 1994, 1995, 2008, 2009, 2010, 2011 Karl Berry.
   Copyright 1997, 2002, 2005 Olaf Weber.

   This library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   This library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this library; if not, see <http://www.gnu.org/licenses/>.  */

#include "w2c-config.h"
#include "trans.h"

/* c-pathch.h */

#ifndef IS_DIR_SEP_CH
#define IS_DIR_SEP_CH(ch) IS_DIR_SEP(ch)
#endif
#ifndef IS_DEVICE_SEP /* No `devices' on, e.g., Unix.  */
#define IS_DEVICE_SEP(ch) 0
#endif
#ifndef NAME_BEGINS_WITH_DEVICE
#define NAME_BEGINS_WITH_DEVICE(name) 0
#endif
#ifndef IS_UNC_NAME /* Unc names are in practice found on Win32 only. */
#define IS_UNC_NAME(name) 0
#endif

#define ISLOWER(c) (isascii (c) && islower((unsigned char)c))
#define TOUPPER(c) (ISLOWER (c) ? toupper ((unsigned char)c) : (c))

/* absolute.c  */

boolean
kpse_absolute_p (const_string filename, boolean relative_ok)
{
  boolean absolute;
  boolean explicit_relative;

  absolute = IS_DIR_SEP (*filename);
  explicit_relative
    = relative_ok
      && (*filename == '.' && (IS_DIR_SEP (filename[1])
                         || (filename[1] == '.' && IS_DIR_SEP (filename[2]))));

  /* FIXME: On UNIX an IS_DIR_SEP of any but the last character in the name
     implies relative.  */
  return absolute || explicit_relative;
}

/* concat3.c */

string
concat3 (const_string s1,  const_string s2,  const_string s3)
{
  int s2l = s2 ? strlen (s2) : 0;
  int s3l = s3 ? strlen (s3) : 0;
  string answer
      = (string) xmalloc (strlen(s1) + s2l + s3l + 1);
  strcpy (answer, s1);
  if (s2) strcat (answer, s2);
  if (s3) strcat (answer, s3);

  return answer;
}

/* concat.c */

/* Return the concatenation of S1 and S2.  See `concatn.c' for a
   `concatn', which takes a variable number of arguments.  */

string
concat (const_string s1,  const_string s2)
{
  unsigned s1len = strlen(s1);
  unsigned s2len = strlen(s2);
  string answer = (string) xmalloc (s1len + s2len + 1);
  strcpy (answer, s1);
  strcat (answer + s1len, s2);

  return answer;
}

/* concatn.c */

/* OK, it would be epsilon more efficient to compute the total length
   and then do the copying ourselves, but I doubt it matters in reality.  */

string
concatn (const_string str1, ...)
{
  string arg;
  string ret;
  va_list ap;

  if (!str1)
    return NULL;

  ret = xstrdup (str1);

  va_start (ap, str1);
  while ((arg = va_arg (ap, string)) != NULL)
    {
      string temp = concat (ret, arg);
      free (ret);
      ret = temp;
    }
  va_end (ap);

  return ret;
}

/* extend-fname.c */

const_string
extend_filename (const_string name, const_string default_suffix)
{
  const_string new_s;
  const_string suffix = find_suffix (name);

  new_s = suffix == NULL ? concat3 (name, ".", default_suffix)
                         : name;
  return new_s;
}

/* find-suffix.c */

const_string
find_suffix (const_string name)
{
  const_string dot_pos = strrchr (name, '.');
  const_string p;

  if (dot_pos == NULL)
    return NULL;

  for (p = dot_pos + 1; *p; p++) {
    if (IS_DIR_SEP (*p))
      return NULL;
  }

  return dot_pos + 1;
}

/* line.c */

/* Allocate in increments of this size.  */
#define LINE_C_BLOCK_SIZE 75

char *
read_line (FILE *f)
{
  int c;
  unsigned limit = LINE_C_BLOCK_SIZE;
  unsigned loc = 0;
  char *line = xmalloc (limit);

  flockfile (f);

  while ((c = getc_unlocked (f)) != EOF && c != '\n' && c != '\r') {
    line[loc] = c;
    loc++;

    /* By testing after the assignment, we guarantee that we'll always
       have space for the null we append below.  We know we always
       have room for the first char, since we start with LINE_C_BLOCK_SIZE.  */
    if (loc == limit) {
      limit += LINE_C_BLOCK_SIZE;
      line = xrealloc (line, limit);
    }
  }

  /* If we read anything, return it, even a partial last-line-if-file
     which is not properly terminated.  */
  if (loc == 0 && c == EOF) {
    /* At end of file.  */
    free (line);
    line = NULL;
  } else {
    /* Terminate the string.  We can't represent nulls in the file,
       but this doesn't matter.  */
    line[loc] = 0;
    /* Absorb LF of a CRLF pair. */
    if (c == '\r') {
      c = getc_unlocked (f);
      if (c != '\n') {
        ungetc (c, f);
      }
    }
  }

  funlockfile (f);

  return line;
}

/* uppercasify.c */

string
uppercasify (const_string s)
{
  string target;
  string ret = xstrdup (s);

  for (target = ret; *target; target++)
    {
      *target = TOUPPER (*target);
    }

  return ret;
}

/* x*.c */

/* Return NAME with any leading path stripped off.  This returns a
   pointer into NAME.  For example, `basename ("/foo/bar.baz")'
   returns "bar.baz".  */

const_string
xbasename (const_string name)
{
    const_string base = name;
    const_string p;

    if (NAME_BEGINS_WITH_DEVICE(name))
        base += 2;

    else if (IS_UNC_NAME(name)) {
        unsigned limit;

        for (limit = 2; name[limit] && !IS_DIR_SEP (name[limit]); limit++)
            ;
        if (name[limit++] && name[limit] && !IS_DIR_SEP (name[limit])) {
            for (; name[limit] && !IS_DIR_SEP (name[limit]); limit++)
                ;
        } else
            /* malformed UNC name, backup */
            limit = 0;
        base += limit;
    }

    for (p = base; *p; p++) {
        if (IS_DIR_SEP(*p))
            base = p + 1;
    }

    return base;
}

void *
xcalloc (size_t nelem,  size_t elsize)
{
    void *new_mem = (void*)calloc(nelem ? nelem : 1, elsize ? elsize : 1);

    if (new_mem == NULL) {
        fprintf(stderr,
                "xcalloc: request for %lu elements of size %lu failed.\n",
                (unsigned long)nelem, (unsigned long)elsize);
        exit(EXIT_FAILURE);
    }

    return new_mem;
}

string
xdirname (const_string name)
{
    string ret;
    unsigned limit = 0, loc;

    /* Ignore a NULL name. */
    if (!name)
        return NULL;

    if (NAME_BEGINS_WITH_DEVICE(name)) {
        limit = 2;
    } else if (IS_UNC_NAME(name)) {
        for (limit = 2; name[limit] && !IS_DIR_SEP (name[limit]); limit++)
            ;
        if (name[limit++] && name[limit] && !IS_DIR_SEP (name[limit])) {
            for (; name[limit] && !IS_DIR_SEP (name[limit]); limit++)
                ;
            limit--;
        } else
            /* malformed UNC name, backup */
            limit = 0;
    }

    if (loc == limit) {
        if (limit == 0)
            ret = xstrdup (".");
        else if (limit == 2) {
            ret = (string)xmalloc(4);
            ret[0] = name[0];
            ret[1] = name[1];
            ret[2] = '.';
            ret[3] = '\0';
        } else {
            /* UNC name is "//server/share".  */
            ret = xstrdup (name);
        }
    } else {
        /* If have ///a, must return /, so don't strip off everything.  */
        while (loc > limit+1 && IS_DIR_SEP (name[loc-1])) {
            loc--;
        }
        ret = (string)xmalloc(loc+1);
        strncpy(ret, name, loc);
        ret[loc] = '\0';
    }

    return ret;
}

FILE *
xfopen (const_string filename,  const_string mode)
{
    FILE *f;

    assert(filename && mode);

    f = fopen(filename, mode);
    if (f == NULL)
        FATAL_PERROR(filename);

    return f;
}


void
xfclose (FILE *f,  const_string filename)
{
    assert(f);

    if (fclose(f) == EOF)
        FATAL_PERROR(filename);

}

void
xfseek (FILE *f,  long offset,  int wherefrom,  const_string filename)
{
  if (fseek (f, offset, wherefrom) < 0) {
        FATAL_PERROR(filename);
  }
}

void
xfseeko (FILE *f,  off_t offset,  int wherefrom,  const_string filename)
{
  if (fseeko (f, offset, wherefrom) < 0) {
        FATAL_PERROR(filename);
  }
}

long
xftell (FILE *f,  const_string filename)
{
    long where = ftell (f);

    if (where < 0)
        FATAL_PERROR(filename);

    return where;
}

off_t
xftello (FILE *f,  const_string filename)
{
    off_t where = ftello (f);

    if (where < 0)
        FATAL_PERROR(filename);

    return where;
}

static void
xchdir (string dirname)
{
    if (chdir(dirname) != 0)
        FATAL_PERROR(dirname);
}


/* Return the pathname of the current directory, or give a fatal error.  */

string
xgetcwd (void)
{
    /* If the system provides getcwd, use it.  If not, use getwd if
       available.  But provide a way not to use getcwd: on some systems
       getcwd forks, which is expensive and may in fact be impossible for
       large programs like tex.  If your system needs this define and it
       is not detected by configure, let me know.
                                       -- Olaf Weber <infovore@xs4all.nl */
    char path[PATH_MAX + 1];

    if (getcwd (path, PATH_MAX + 1) == NULL) {
        FATAL_PERROR ("getcwd");
    }

    return xstrdup (path);
}

void *
xmalloc (size_t size)
{
    void *new_mem = (void *)malloc(size ? size : 1);

    if (new_mem == NULL) {
        fprintf(stderr, "fatal: memory exhausted (xmalloc of %lu bytes).\n",
                (unsigned long)size);
        exit(EXIT_FAILURE);
    }

    return new_mem;
}

DIR *
xopendir (const_string dirname)
{
    DIR *d = opendir(dirname);

    if (d == NULL)
        FATAL_PERROR(dirname);

    return d;
}

void
xclosedir (DIR *d)
{
    int ret = closedir(d);

    if (ret != 0)
        FATAL("closedir failed");
}

/*
 * We have different arguments from the "standard" function.  A separate
 * var and value tends to be much more practical.
 *
 * The standards for putenv are clear: put the passed string into the
 * environment, and if you alter that string, the environment changes.
 * Of course various implementations are broken in a number of ways,
 * which include making copies of the passed string, and more.
 */
void
kpathsea_xputenv(/*kpathsea kpse, */const char *var, const char *value)
{
    char  *cur_item;
    char  *new_item;
    size_t var_lim;
    int    cur_loc;

    /* kpse_debug2(KPSE_DEBUG_VARS, "kpse_putenv($%s,%s)", var, value); */

    cur_item = concat3(var, "=", value);
    /* Include '=' in length. */
    var_lim = strlen(var) + 1;

    /* We set a different value. */
    if (putenv(cur_item) < 0)
	FATAL1("putenv(%s)", cur_item);
    /* Get the new string. */
    new_item = getenv(var);
    if (new_item != cur_item+var_lim) {
	/* Our new string isn't used, don't keep it around. */
	free(cur_item);
	return;
    }

    return;
}

/* A special case for setting a variable to a numeric value
   (specifically, KPATHSEA_DPI).  We don't need to dynamically allocate
   and free the string for the number, since it's saved as part of the
   environment value.  */

void
kpathsea_xputenv_int (/*kpathsea kpse, */const_string var_name,  int num)
{
  char str[MAX_INT_LENGTH];
  sprintf (str, "%d", num);

  kpathsea_xputenv (/*kpse, */var_name, str);
}

void
xputenv (const char *var, const char *value)
{
    kpathsea_xputenv (/*kpse_def, */var, value);
}

void
xputenv_int (const_string var_name,  int num)
{
    kpathsea_xputenv_int(/*kpse_def, */var_name, num);
}

void *
xrealloc (void *old_ptr, size_t size)
{
    void *new_mem;

    if (old_ptr == NULL) {
        new_mem = xmalloc(size);
    } else {
        new_mem = (void *)realloc(old_ptr, size ? size : 1);
        if (new_mem == NULL) {
            /* We used to print OLD_PTR here using %x, and casting its
               value to unsigned, but that lost on the Alpha, where
               pointers and unsigned had different sizes.  Since the info
               is of little or no value anyway, just don't print it.  */
            fprintf(stderr,
                    "fatal: memory exhausted (realloc of %lu bytes).\n",
                    (unsigned long)size);
            exit(EXIT_FAILURE);
        }
    }

    return new_mem;
}

struct stat
xstat (const_string path)
{
    struct stat s;

    if (stat(path, &s) != 0)
        FATAL_PERROR(path);

    return s;
}

/*
// We declared lstat to prevent a warning during development.  This
// turns out to be more trouble than it is worth.
// extern int lstat ();
*/
struct stat
xlstat (const_string path)
{
    struct stat s;

    if (lstat(path, &s) != 0)
        FATAL_PERROR(path);
    return s;
}

string
xstrdup (const_string s)
{
  string new_string = (string)xmalloc(strlen (s) + 1);
  return strcpy(new_string, s);
}

/* usage.c */

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
    exit(0);
}

/* zround.c */

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

/* numbers.c */

unsigned char get_unsigned_byte (FILE *file)
{
  int ch;
  if ((ch = fgetc (file)) < 0) {
    fprintf (stderr, "File ended prematurely\n");
    exit(-1);
  }
  return (unsigned char) ch;
}

unsigned short get_unsigned_pair (FILE *file)
{
  unsigned short pair = get_unsigned_byte(file);
  pair = pair*0x100u + get_unsigned_byte(file);
  return pair;
}

/* printversion.c */

const char *version_string = " (Tectonic)";

/* We're passed in the original WEB banner string, which has the form
This is PROGRAM, Version VERSION-NUMBER
   We parse the PROGRAM and VERSION-NUMBER out of this.
   
   If COPYRIGHT_HOLDER is specified and AUTHOR isn't, then use the
   former for the latter.  If AUTHOR is specified and COPYRIGHT_HOLDER
   isn't, it means the original program is public domain.
   
   Maybe I should have just done it all inline in each individual
   program, but tangle doesn't allow multiline string constants ...  */

void
printversionandexit (const_string banner,
                     const_string copyright_holder,  
                     const_string author,
                     const_string extra_info)
{
  string prog_name;
  unsigned len;
  const_string prog_name_end = strchr (banner, ',');
  const_string prog_version = strrchr (banner, ' ');
  assert (prog_name_end && prog_version);
  prog_version++;
  
  len = prog_name_end - banner - sizeof ("This is");
  prog_name = xmalloc (len + 1);
  strncpy (prog_name, banner + sizeof ("This is"), len);
  prog_name[len] = 0;

  /* The Web2c version string starts with a space.  */
  printf ("%s %s%s\n", prog_name, prog_version, version_string);

  if (copyright_holder) {
    printf ("Copyright 2016 %s.\n", copyright_holder);
    if (!author)
      author = copyright_holder;
  }

  puts ("There is NO warranty.  Redistribution of this software is");
  fputs ("covered by the terms of ", stdout);
  printf ("both the %s copyright and\n", prog_name);
  puts ("the Lesser GNU General Public License.");
  puts ("For more information about these matters, see the file");
  printf ("named COPYING and the %s source.\n", prog_name);
  printf ("Primary author of %s: %s.\n", prog_name, author);

  if (extra_info)
    fputs (extra_info, stdout);

  exit (0);
}

/* trans.c */

void make_identity(transform* t)
{
	t->a = 1.0;
	t->b = 0.0;
	t->c = 0.0;
	t->d = 1.0;
	t->x = 0.0;
	t->y = 0.0;
}

void make_scale(transform* t, double xscale, double yscale)
{
	t->a = xscale;
	t->b = 0.0;
	t->c = 0.0;
	t->d = yscale;
	t->x = 0.0;
	t->y = 0.0;
}

void make_translation(transform* t, double dx, double dy)
{
	t->a = 1.0;
	t->b = 0.0;
	t->c = 0.0;
	t->d = 1.0;
	t->x = dx;
	t->y = dy;
}

void make_rotation(transform* t, double a)
{
	t->a = cos(a);
	t->b = sin(a);
	t->c = -sin(a);
	t->d = cos(a);
	t->x = 0.0;
	t->y = 0.0;
}

void transform_point(real_point* p, const transform* t)
{
	real_point	r;
	r.x = t->a * p->x + t->c * p->y + t->x;
	r.y = t->b * p->x + t->d * p->y + t->y;
	*p = r;
}

void transform_concat(transform* t1, const transform* t2)
{
	transform	r;
	r.a = t1->a * t2->a + t1->b * t2->c + 0.0 * t2->x;
	r.b = t1->a * t2->b + t1->b * t2->d + 0.0 * t2->y;
	r.c = t1->c * t2->a + t1->d * t2->c + 0.0 * t2->x;
	r.d = t1->c * t2->b + t1->d * t2->d + 0.0 * t2->y;
	r.x = t1->x * t2->a + t1->y * t2->c + 1.0 * t2->x;
	r.y = t1->x * t2->b + t1->y * t2->d + 1.0 * t2->y;
	*t1 = r;
}
