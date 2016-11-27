/* texmfmp.h: Main include file for TeX and MF in C. This file is
   included by {tex,mf}d.h, which is the first include in the C files
   output by web2c.  */

#include "cpascal.h"
#include <tidy_kpathutil.h>
#include <kpsezip/public.h>

#ifdef XETEX_MAC
/* include this here to avoid conflict between clang's emmintrin.h and
 * texmfmem.h. Should be removed once a fixed clang is widely available
 * http://llvm.org/bugs/show_bug.cgi?id=14964 */
#include <ApplicationServices/ApplicationServices.h>
#endif

/* added typedefs for unicodefile and voidpointer */
#define XETEX_UNICODE_FILE_DEFINED	1
typedef struct {
  FILE *f;
  long  savedChar;
  short skipNextLF;
  short encodingMode;
  void *conversionData;
} UFILE;
typedef UFILE* unicodefile;
typedef unicodefile unicode_file;

typedef void* voidpointer;
typedef voidpointer void_pointer;

/* If we have these macros, use them, as they provide a better guide to
   the endianess when cross-compiling. */
#if defined (BYTE_ORDER) && defined (BIG_ENDIAN) && defined (LITTLE_ENDIAN)
#ifdef WORDS_BIGENDIAN
#undef WORDS_BIGENDIAN
#endif
#if BYTE_ORDER == BIG_ENDIAN
#define WORDS_BIGENDIAN
#endif
#endif

#define TEXMF_POOL_NAME "xetex.pool"
#define TEXMF_ENGINE_NAME "xetex"

#define DUMP_FILE fmt_file
#define write_dvi WRITE_OUT
#define flush_dvi flush_out
#define OUT_FILE dvi_file
#define OUT_BUF dvi_buf

/* Restore underscores.  */
#define dumpname dump_name

/* Hacks for TeX that are better not to #ifdef, see lib/openclose.c.  */
extern int tfm_temp, tex_input_type;

extern void getmd5sum(integer s, int file);

/* Executing shell commands.  */
extern int runsystem (const char *cmd);

/* The entry point.  */
extern void maininit (int ac, string *av);

/* All but the Omega family use this. */
extern void readtcxfile (void);
extern string translate_filename;
#define translatefilename translate_filename

/* The type `glueratio' should be a floating point type which won't
   unnecessarily increase the size of the memoryword structure.  This is
   the basic requirement.  On most machines, if you're building a
   normal-sized TeX, then glueratio must probably meet the following
   restriction: sizeof(glueratio) <= sizeof(integer).  Usually, then,
   glueratio must be `float'.  But if you build a big TeX, you can (on
   most machines) and should make it `double' to avoid loss of precision
   and conversions to and from double during calculations.  (All this
   also goes for Metafont.)  Furthermore, if you have enough memory, it
   won't hurt to have this defined to be `double' for running the
   trip/trap tests.
   
   This type is set automatically to `float' by configure if a small TeX
   is built.  */
typedef double glueratio;

/* How to flush the DVI file.  */
#define flush_out() fflush (OUT_FILE)

/* Used to write to a TFM file.  */
#define put2bytes(f, h) do { \
    integer v = (integer) (h); put_byte (v >> 8, f);  put_byte (v & 0xff, f); \
  } while (0)
#define put4bytes(f, w) do { \
    integer v = (integer) (w); \
    put_byte (v >> 24, f); put_byte (v >> 16, f); \
    put_byte (v >> 8, f);  put_byte (v & 0xff, f); \
  } while (0)

/* Read a line of input as quickly as possible.  */
#define	input_ln(stream, flag) input_line (stream)
extern boolean input_line (UFILE *);

/* This routine has to return four values.  */
#define	date_and_time(i,j,k,l) get_date_and_time (&(i), &(j), &(k), &(l))
extern void get_date_and_time (integer *, integer *, integer *, integer *);

/* Copy command-line arguments into the buffer, despite the name.  */
extern void t_open_in (void);

/* Can't prototype this since it uses poolpointer and ASCIIcode, which
   are defined later in mfd.h, and mfd.h uses stuff from here.  */
/* Therefore the department of ugly hacks decided to move this declaration
   to the *coerce.h files. */
/* extern void calledit (); */

/* `bopenin' (and out) is used only for reading (and writing) .tfm
   files; `wopenin' (and out) only for dump files.  The filenames are
   passed in as a global variable, `nameoffile'.  */
#define b_open_in(f)	open_input (&(f), kpse_tfm_format, FOPEN_RBIN_MODE)
#define ocp_open_in(f)	open_input (&(f), kpse_ocp_format, FOPEN_RBIN_MODE)
#define ofm_open_in(f)	open_input (&(f), kpse_ofm_format, FOPEN_RBIN_MODE)

#define b_open_out(f)	open_output (&(f), FOPEN_WBIN_MODE)
#define b_close		a_close

/* f is declared as gzFile, but we temporarily use it for a FILE *
   so that we can use the standard open calls */
#define w_open_in(f)	(open_input ((FILE**)&(f), kpse_fmt_format, FOPEN_RBIN_MODE) \
						&& (f = gzdopen(fileno((FILE*)f), FOPEN_RBIN_MODE)))
#define w_open_out(f)	(open_output ((FILE**)&(f), FOPEN_WBIN_MODE) \
						&& (f = gzdopen(fileno((FILE*)f), FOPEN_WBIN_MODE)) \
						&& (gzsetparams(f, 1, Z_DEFAULT_STRATEGY) == Z_OK))
#define w_close(f)	gzclose(f)

#define u_open_in(f,p,m,d) real_u_open_in(&(f), p, FOPEN_RBIN_MODE, m, d)

/* (Un)dumping.  These are called from the change file.  */
#define	dump_things(base, len) \
  do_dump ((char *) &(base), sizeof (base), (int) (len), DUMP_FILE)
#define	undump_things(base, len) \
  do_undump ((char *) &(base), sizeof (base), (int) (len), DUMP_FILE)

#ifndef PRIdPTR
#define PRIdPTR "ld"
#endif
#ifndef PRIxPTR
#define PRIxPTR "lx"
#endif

/* Like do_undump, but check each value against LOW and HIGH.  The
   slowdown isn't significant, and this improves the chances of
   detecting incompatible format files.  In fact, Knuth himself noted
   this problem with Web2c some years ago, so it seems worth fixing.  We
   can't make this a subroutine because then we lose the type of BASE.  */
#define undump_checked_things(low, high, base, len)			\
  do {                                                                  \
    unsigned i;                                                         \
    undump_things (base, len);                                           \
    for (i = 0; i < (len); i++) {                                       \
      if ((&(base))[i] < (low) || (&(base))[i] > (high)) {              \
        FATAL5 ("Item %u (=%" PRIdPTR ") of .fmt array at %" PRIxPTR    \
                " <%" PRIdPTR " or >%" PRIdPTR,                         \
                i, (uintptr_t) (&(base))[i], (uintptr_t) &(base),       \
                (uintptr_t) low, (uintptr_t) high);                     \
      }                                                                 \
    }									\
  } while (0)

/* Like undump_checked_things, but only check the upper value. We use
   this when the base type is unsigned, and thus all the values will be
   greater than zero by definition.  */
#define undump_upper_check_things(high, base, len)				\
  do {                                                                  \
    unsigned i;                                                         \
    undump_things (base, len);                                           \
    for (i = 0; i < (len); i++) {                                       \
      if ((&(base))[i] > (high)) {              			\
        FATAL4 ("Item %u (=%" PRIdPTR ") of .fmt array at %" PRIxPTR    \
                " >%" PRIdPTR,                                          \
                i, (uintptr_t) (&(base))[i], (uintptr_t) &(base),       \
                (uintptr_t) high);                         		\
      }                                                                 \
    }									\
  } while (0)

/* We define the routines to do the actual work in texmfmp.c.  */
#include <zlib.h>
extern void do_dump (char *, int, int, gzFile);
extern void do_undump (char *, int, int, gzFile);

/* Use the above for all the other dumping and undumping.  */
#define generic_dump(x) dump_things (x, 1)
#define generic_undump(x) undump_things (x, 1)

#define dump_wd   generic_dump
#define dump_hh   generic_dump
#define dump_qqqq generic_dump
#define undump_wd   generic_undump
#define undump_hh   generic_undump
#define	undump_qqqq generic_undump

/* `dump_int' is called with constant integers, so we put them into a
   variable first.  */
#define	dump_int(x)							\
  do									\
    {									\
      integer x_val = (x);						\
      generic_dump (x_val);						\
    }									\
  while (0)

#define	undump_int generic_undump

/* Handle SyncTeX, if requested */
#include "synctex-common.h"
extern char *generic_synctex_get_current_name(void);
