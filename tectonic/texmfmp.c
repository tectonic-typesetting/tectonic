/* texmfmp.c: Hand-coded routines for TeX or Metafont in C.  Originally
   written by Tim Morgan, drawing from other Unix ports of TeX.  This is
   a collection of miscellany, everything that's easier (or only
   possible) to do in C.

   This file is public domain.  */

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/md5.h>
#include <kpsezip/public.h>

#include <sys/stat.h>
#include <sys/time.h>
#include <time.h> /* For `struct tm'.  Moved here for Visual Studio 2005.  */
#include <locale.h>
#include <signal.h>

/* Formerly from xetexextra.c: */

#define EXTERN
#include <xetexd.h>

/* formerly texmfmp-help.h: */

const_string XETEXHELP[] = {
    "Usage: xetex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: xetex [OPTION]... \\FIRST-LINE",
    "   or: xetex [OPTION]... &FMT ARGS",
    "  Run XeTeX on TEXNAME, usually creating TEXNAME.pdf.",
    "  Any remaining COMMANDS are processed as XeTeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `xetex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of XeTeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-etex                   enable e-TeX extensions",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-fmt=FMTNAME            use FMTNAME instead of program name or a %& line",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be xeinitex, for dumping formats; this is implicitly",
    "                          true if the program name is `xeinitex'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for XDV file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "-output-driver=CMD      use CMD as the XDV-to-PDF driver instead of xdvipdfmx",
    "-no-pdf                 generate XDV (extended DVI) output rather than PDF",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-papersize=STRING       set PDF media size to STRING",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-src-specials           insert source specials into the XDV file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the XDV file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
    "-translate-file=TCXNAME (ignored)",
    "-8bit                   make all characters printable, don't use ^^X sequences",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    NULL
};

/* end texmfmp-help.h */

/* formerly xetexextra.h: */

#define ETEX_VERSION "2.6"
#define XETEX_VERSION "0.99996"
#define BANNER "This is XeTeX, Version 3.14159265-" ETEX_VERSION "-" XETEX_VERSION
#define COPYRIGHT_HOLDER "SIL International, Jonathan Kew and Khaled Hosny"
#define AUTHOR "Jonathan Kew"
#define BUG_ADDRESS "xetex@tug.org"
#define DUMP_VAR TEX_format_default
#define DUMP_LENGTH_VAR format_default_length
#define DUMP_OPTION "fmt"
#define DUMP_EXT ".fmt"
#define INI_PROGRAM "xeinitex"
#define VIR_PROGRAM "xevirtex"

/* end xetexextra.h */

/*
   SyncTeX file name should be full path in the case where
   --output-directory option is given.
   Borrowed from LuaTeX.
*/
char *generic_synctex_get_current_name (void)
{
  char *pwdbuf, *ret;
  if (!fullnameoffile) {
    ret = xstrdup("");
    return ret;
  }
  if (kpse_absolute_p(fullnameoffile, false)) {
     return xstrdup(fullnameoffile);
  }
  pwdbuf = xgetcwd();
  ret = concat3(pwdbuf, DIR_SEP_STRING, fullnameoffile);
  free(pwdbuf) ;
  return ret;
}

/* The main program, etc.  */

#include "XeTeX_ext.h"

/* What we were invoked as and with.  */
char **argv;
int argc;

/* If the user overrides argv[0] with -progname.  */
static const_string user_progname;

/* The C version of the jobname, if given. */
static const_string c_job_name;

/* The filename for dynamic character translation, or NULL.  */
string translate_filename;
string default_translate_filename;

/* Needed for --src-specials option. */
static char *last_source_name;
static int last_lineno;
static boolean src_specials_option = false;
static void parse_src_specials_option (const_string);

/* Parsing a first %&-line in the input file. */
static void parse_first_line (const_string);

/* Parse option flags. */
static void parse_options (int, string *);

/* Try to figure out if we have been given a filename. */
static string get_input_file_name (void);

/* The entry point: set up for reading the command line, which will
   happen in `t_open_in', then call the main body.  */

void
maininit (int ac, string *av)
{
  string main_input_file;
  /* Save to pass along to t_open_in.  */
  argc = ac;
  argv = av;

  /* Must be initialized before options are parsed.  */
  interaction_option = 4;

  /* [The "recorder" input and output functions used to be set here.] */

  /* 0 means "disable Synchronize TeXnology".
     synctexoption is a *.web variable.
     We initialize it to a weird value to catch the -synctex command line flag.
     At runtime, if synctexoption is not INT_MAX, then it contains the
     command line option provided; otherwise, no such option was given
     by the user.  */
# define SYNCTEX_NO_OPTION INT_MAX
  synctexoption = SYNCTEX_NO_OPTION;

  /* If the user says --help or --version, we need to notice early.  And
     since we want the --ini option, have to do it before getting into
     the web (which would read the base file, etc.).  */
  parse_options (ac, av);

  /* If -progname was not specified, default to the dump name.  */
  if (!user_progname)
    user_progname = dump_name;

  /* Do this early so we can inspect kpse_invocation_name and
     kpse_program_name below, and because we have to do this before
     any path searching.  */

  /* FIXME: gather engine names in a single spot. */
  xputenv ("engine", TEXMF_ENGINE_NAME);

  /* Were we given a simple filename? */
  main_input_file = get_input_file_name();

  /* Second chance to activate file:line:error style messages, this
     time from texmf.cnf. */
  if (file_line_error_style_p < 0)
    file_line_error_style_p = 0;

  /* If no dump default yet, and we're not doing anything special on
     this run, we may want to look at the first line of the main input
     file for a %&<dumpname> specifier.  */
  if (parse_first_line_p < 0)
    parse_first_line_p = 0;

  if (parse_first_line_p && (!dump_name || !translate_filename)) {
    parse_first_line (main_input_file);
  }
  /* Check whether there still is no translate_filename known.  If so,
     use the default_translate_filename. */
  /* FIXME: deprecated. */
  if (!translate_filename) {
    translate_filename = default_translate_filename;
  }
  /* If we're preloaded, I guess everything is set up.  I don't really
     know any more, it's been so long since anyone preloaded.  */
  if (ready_already != 314159) {
    if (!dump_name)
	dump_name = "xelatex";
  }

  /* Sanity check: -mltex, -enc, -etex only work in combination with -ini. */
  if (!ini_version) {
    if (mltex_p) {
      fprintf(stderr, "-mltex only works with -ini\n");
    }
  }

  /* If we've set up the fmt/base default in any of the various ways
     above, also set its length.  */
  if (dump_name) {
    const_string with_ext = NULL;
    unsigned name_len = strlen (dump_name);
    unsigned ext_len = strlen (DUMP_EXT);

    /* Provide extension if not there already.  */
    if (name_len > ext_len
        && FILESTRCASEEQ (dump_name + name_len - ext_len, DUMP_EXT)) {
      with_ext = dump_name;
    } else {
      with_ext = concat (dump_name, DUMP_EXT);
    }
    DUMP_VAR = concat (" ", with_ext); /* adjust array for Pascal */
    DUMP_LENGTH_VAR = strlen (DUMP_VAR + 1);
  } else {
    /* For dump_name to be NULL is a bug.  */
    abort();
  }

  shellenabledp = 0;
}

/* The entry point: set up for reading the command line, which will
   happen in `t_open_in', then call the main body.  */

int
main (int ac, string *av)
{
  maininit (ac, av);

  /* Call the real main program.  */
  main_body ();

  return EXIT_SUCCESS;
}

/* This is supposed to ``open the terminal for input'', but what we
   really do is copy command line arguments into TeX's or Metafont's
   buffer, so they can handle them.  If nothing is available, or we've
   been called already (and hence, argc==0), we return with
   `last=first'.  */

void
t_open_in (void)
{
  int i;

  static UFILE termin_file;
  if (term_in == 0) {
    term_in = &termin_file;
    term_in->f = stdin;
    term_in->savedChar = -1;
    term_in->skipNextLF = 0;
    term_in->encodingMode = UTF8;
    term_in->conversionData = 0;
    input_file[0] = term_in;
  }

  buffer[first] = 0; /* In case there are no arguments.  */

  if (optind < argc) { /* We have command line arguments.  */
    int k = first;
    for (i = optind; i < argc; i++) {
      unsigned char *ptr = (unsigned char *)&(argv[i][0]);
      /* need to interpret UTF8 from the command line */
      UInt32 rval;
      while ((rval = *(ptr++)) != 0) {
        UInt16 extraBytes = bytesFromUTF8[rval];
        switch (extraBytes) { /* note: code falls through cases! */
          case 5: rval <<= 6; if (*ptr) rval += *(ptr++);
          case 4: rval <<= 6; if (*ptr) rval += *(ptr++);
          case 3: rval <<= 6; if (*ptr) rval += *(ptr++);
          case 2: rval <<= 6; if (*ptr) rval += *(ptr++);
          case 1: rval <<= 6; if (*ptr) rval += *(ptr++);
          case 0: ;
        };
        rval -= offsetsFromUTF8[extraBytes];
        buffer[k++] = rval;
      }
      buffer[k++] = ' ';
    }
    argc = 0;	/* Don't do this again.  */
    buffer[k] = 0;
  }

  /* Find the end of the buffer.  */
  for (last = first; buffer[last]; ++last)
    ;

  /* Make `last' be one past the last non-blank character in `buffer'.  */
  /* ??? The test for '\r' should not be necessary.  */
  for (--last; last >= first
       && ISBLANK (buffer[last]) && buffer[last] != '\r'; --last)
    ;
  last++;
}

static string
normalize_quotes (const_string name, const_string mesg)
{
    int quote_char = 0;
    boolean must_quote = false;
    int len = strlen(name);
    /* Leave room for quotes and NUL. */
    string ret;
    string p;
    const_string q;
    for (q = name; *q; q++) {
        if (*q == ' ') {
            if (!must_quote) {
                len += 2;
                must_quote = true;
            }
        }
        else if (*q == '\"' || *q == '\'') {
            must_quote = true;
            if (quote_char == 0)
                quote_char = '\"' + '\'' - *q;
            len += 2; /* this could sometimes add length we don't need */
        }
    }
    ret = xmalloc(len + 1);
    p = ret;
    if (must_quote) {
        if (quote_char == 0)
            quote_char = '\"';
        *p++ = quote_char;
    }
    for (q = name; *q; q++) {
        if (*q == quote_char) {
            *p++ = quote_char;
            quote_char = '\"' + '\'' - quote_char;
            *p++ = quote_char;
        }
        *p++ = *q;
    }
    if (quote_char != 0)
        *p++ = quote_char;
    *p = '\0';
    return ret;
}

/* Getting the input filename. */
string
get_input_file_name (void)
{
    string input_file_name = NULL;

    if (argv[optind]) {
	input_file_name = xstrdup(argv[optind]);
	argv[optind] = normalize_quotes(argv[optind], "argument");
    }

    return input_file_name;
}

/* Reading the options.  */

/* Test whether getopt found an option ``A''.
   Assumes the option index is in the variable `option_index', and the
   option table in a variable `long_options'.  */
#define ARGUMENT_IS(a) STREQ (long_options[option_index].name, a)

/* SunOS cc can't initialize automatic structs, so make this static.  */
static struct option long_options[]
  = { { DUMP_OPTION,                 1, 0, 0 },
      /* FIXME: Obsolete -- for backward compatibility only. */
      { "efmt",                      1, 0, 0 },
      { "help",                      0, 0, 0 },
      { "ini",                       0, &ini_version, 1 },
      { "interaction",               1, 0, 0 },
      { "halt-on-error",             0, &halt_on_error_p, 1 },
      { "progname",                  1, 0, 0 },
      { "version",                   0, 0, 0 },
      { "mltex",                     0, &mltex_p, 1 },
      { "etex",                      0, &etex_p, 1 },
      { "output-comment",            1, 0, 0 },
      { "debug-format",              0, &debug_format_file, 1 },
      { "src-specials",              2, 0, 0 },
      /* Synchronization: just like "interaction" above */
      { "synctex",                   1, 0, 0 },
      { "file-line-error-style",     0, &file_line_error_style_p, 1 },
      { "no-file-line-error-style",  0, &file_line_error_style_p, -1 },
      /* Shorter option names for the above. */
      { "file-line-error",           0, &file_line_error_style_p, 1 },
      { "no-file-line-error",        0, &file_line_error_style_p, -1 },
      { "jobname",                   1, 0, 0 },
      { "parse-first-line",          0, &parse_first_line_p, 1 },
      { "no-parse-first-line",       0, &parse_first_line_p, -1 },
      { "translate-file",            1, 0, 0 },
      { "default-translate-file",    1, 0, 0 },
      { "8bit",                      0, &eight_bit_p, 1 },
      { "no-pdf",                    0, &no_pdf_output, 1 },
      { "output-driver",             1, 0, 0 },
      { "papersize",                 1, 0, 0 },
      { 0, 0, 0, 0 } };

static void
parse_options (int argc, string *argv)
{
  int g;   /* `getopt' return code.  */
  int option_index;

  for (;;) {
    g = getopt_long_only (argc, argv, "+", long_options, &option_index);

    if (g == -1) /* End of arguments, exit the loop.  */
      break;

    if (g == '?') { /* Unknown option.  */
      /* FIXME: usage (argv[0]); replaced by continue. */
      continue;
    }

    assert (g == 0); /* We have no short option names.  */

    if (ARGUMENT_IS ("progname")) {
      user_progname = optarg;
    } else if (ARGUMENT_IS ("papersize")) {
      papersize = optarg;
    } else if (ARGUMENT_IS ("output-driver")) {
      outputdriver = optarg;
    } else if (ARGUMENT_IS ("jobname")) {
      c_job_name = optarg;
    } else if (ARGUMENT_IS (DUMP_OPTION)) {
      dump_name = optarg;
      dump_option = true;
    } else if (ARGUMENT_IS ("efmt")) {
      dump_name = optarg;
      dump_option = true;
    } else if (ARGUMENT_IS ("output-comment")) {
      unsigned len = strlen (optarg);
      if (len < 256) {
        output_comment = optarg;
      } else {
        WARNING2 ("Comment truncated to 255 characters from %d. (%s)",
                  len, optarg);
        output_comment = xmalloc (256);
        strncpy (output_comment, optarg, 255);
        output_comment[255] = 0;
      }
    } else if (ARGUMENT_IS ("src-specials")) {
       last_source_name = xstrdup("");
       /* Option `--src" without any value means `auto' mode. */
       if (optarg == NULL) {
         insert_src_special_every_par = true;
         insert_src_special_auto = true;
         src_specials_option = true;
         src_specials_p = true;
       } else {
          parse_src_specials_option(optarg);
       }
    } else if (ARGUMENT_IS ("translate-file")) {
      translate_filename = optarg;
    } else if (ARGUMENT_IS ("default-translate-file")) {
      default_translate_filename = optarg;
    } else if (ARGUMENT_IS ("interaction")) {
        /* These numbers match @d's in *.ch */
      if (STREQ (optarg, "batchmode")) {
        interaction_option = 0;
      } else if (STREQ (optarg, "nonstopmode")) {
        interaction_option = 1;
      } else if (STREQ (optarg, "scrollmode")) {
        interaction_option = 2;
      } else if (STREQ (optarg, "errorstopmode")) {
        interaction_option = 3;
      } else {
        WARNING1 ("Ignoring unknown argument `%s' to --interaction", optarg);
      }
    } else if (ARGUMENT_IS ("help")) {
        usagehelp (XETEXHELP, BUG_ADDRESS);
    } else if (ARGUMENT_IS ("synctex")) {
		/* Synchronize TeXnology: catching the command line option as a long  */
		synctexoption = (int) strtol(optarg, NULL, 0);
    } else if (ARGUMENT_IS ("version")) {
        char *versions;
        initversionstring(&versions);
        printversionandexit (BANNER, COPYRIGHT_HOLDER, AUTHOR, versions);
    } /* Else it was a flag; getopt has already done the assignment.  */
  }
}

void
parse_src_specials_option (const_string opt_list)
{
  char * toklist = xstrdup(opt_list);
  char * tok;
  insert_src_special_auto = false;
  tok = strtok (toklist, ", ");
  while (tok) {
    if (strcmp (tok, "everypar") == 0
        || strcmp (tok, "par") == 0
        || strcmp (tok, "auto") == 0) {
      insert_src_special_auto = true;
      insert_src_special_every_par = true;
    } else if (strcmp (tok, "everyparend") == 0
               || strcmp (tok, "parend") == 0)
      insert_src_special_every_parend = true;
    else if (strcmp (tok, "everycr") == 0
             || strcmp (tok, "cr") == 0)
      insert_src_special_every_cr = true;
    else if (strcmp (tok, "everymath") == 0
             || strcmp (tok, "math") == 0)
      insert_src_special_every_math = true;
    else if (strcmp (tok, "everyhbox") == 0
             || strcmp (tok, "hbox") == 0)
      insert_src_special_every_hbox = true;
    else if (strcmp (tok, "everyvbox") == 0
             || strcmp (tok, "vbox") == 0)
      insert_src_special_every_vbox = true;
    else if (strcmp (tok, "everydisplay") == 0
             || strcmp (tok, "display") == 0)
      insert_src_special_every_display = true;
    else if (strcmp (tok, "none") == 0) {
      /* This one allows to reset an option that could appear in texmf.cnf */
      insert_src_special_auto = insert_src_special_every_par =
        insert_src_special_every_parend = insert_src_special_every_cr =
        insert_src_special_every_math =  insert_src_special_every_hbox =
        insert_src_special_every_vbox = insert_src_special_every_display = false;
    } else {
      WARNING1 ("Ignoring unknown argument `%s' to --src-specials", tok);
    }
    tok = strtok(0, ", ");
  }
  free(toklist);
  src_specials_p=insert_src_special_auto | insert_src_special_every_par |
    insert_src_special_every_parend | insert_src_special_every_cr |
    insert_src_special_every_math |  insert_src_special_every_hbox |
    insert_src_special_every_vbox | insert_src_special_every_display;
  src_specials_option = true;
}

static void
parse_first_line (const_string filename)
{
}

static RETSIGTYPE
catch_interrupt (int arg)
{
  interrupt = 1;
  (void) signal (SIGINT, catch_interrupt);
}

static boolean start_time_set = false;
static time_t start_time = 0;

void init_start_time() {
    if (!start_time_set) {
        start_time_set = true;
	start_time = time((time_t *) NULL);
    }
}

/* Besides getting the date and time here, we also set up the interrupt
   handler, for no particularly good reason.  It's just that since the
   `fix_date_and_time' routine is called early on (section 1337 in TeX,
   ``Get the first line of input and prepare to start''), this is as
   good a place as any.  */

void
get_date_and_time (integer *minutes,  integer *day,
                   integer *month,  integer *year)
{
  struct tm *tmptr;

  /* whether the envvar was not set (usual case) or invalid,
     use current time.  */
  time_t myclock = time ((time_t *) 0);
  tmptr = localtime (&myclock);
  *minutes = tmptr->tm_hour * 60 + tmptr->tm_min;
  *day = tmptr->tm_mday;
  *month = tmptr->tm_mon + 1;
  *year = tmptr->tm_year + 1900;

  {
    RETSIGTYPE (*old_handler)(int);

    old_handler = signal (SIGINT, catch_interrupt);
    if (old_handler != SIG_DFL)
      signal (SIGINT, old_handler);
  }
}

/* This procedure originally due to sjc@s1-c.  TeX & Metafont call it when
   the user types `e' in response to an error, invoking a text editor on
   the erroneous source file.  FNSTART is how far into FILENAME the
   actual filename starts; FNLENGTH is how long the filename is.  */

void
call_edit (packedASCIIcode *filename,
          pool_pointer fnstart,
          integer fnlength,
          integer linenumber)
{
  /* Quit, since we found an error.  */
  exit (1);
}

/* Read and write dump files.  As distributed, these files are
   architecture dependent; specifically, BigEndian and LittleEndian
   architectures produce different files.  These routines always output
   BigEndian files.  This still does not guarantee them to be
   architecture-independent, because it is possible to make a format
   that dumps a glue ratio, i.e., a floating-point number.  Fortunately,
   none of the standard formats do that.  */

#if !defined (WORDS_BIGENDIAN) && !defined (NO_DUMP_SHARE) /* this fn */

/* This macro is always invoked as a statement.  It assumes a variable
   `temp'.  */

#define SWAP(x, y) temp = (x); (x) = (y); (y) = temp


/* Make the NITEMS items pointed at by P, each of size SIZE, be the
   opposite-endianness of whatever they are now.  */

static void
swap_items (char *p, int nitems, int size)
{
  char temp;

  /* Since `size' does not change, we can write a while loop for each
     case, and avoid testing `size' for each time.  */
  switch (size)
    {
    /* 16-byte items happen on the DEC Alpha machine when we are not
       doing sharable memory dumps.  */
    case 16:
      while (nitems--)
        {
          SWAP (p[0], p[15]);
          SWAP (p[1], p[14]);
          SWAP (p[2], p[13]);
          SWAP (p[3], p[12]);
          SWAP (p[4], p[11]);
          SWAP (p[5], p[10]);
          SWAP (p[6], p[9]);
          SWAP (p[7], p[8]);
          p += size;
        }
      break;

    case 8:
      while (nitems--)
        {
          SWAP (p[0], p[7]);
          SWAP (p[1], p[6]);
          SWAP (p[2], p[5]);
          SWAP (p[3], p[4]);
          p += size;
        }
      break;

    case 4:
      while (nitems--)
        {
          SWAP (p[0], p[3]);
          SWAP (p[1], p[2]);
          p += size;
        }
      break;

    case 2:
      while (nitems--)
        {
          SWAP (p[0], p[1]);
          p += size;
        }
      break;

    case 1:
      /* Nothing to do.  */
      break;

    default:
      FATAL1 ("Can't swap a %d-byte item for (un)dumping", size);
  }
}
#endif /* not WORDS_BIGENDIAN and not NO_DUMP_SHARE */


/* Here we write NITEMS items, each item being ITEM_SIZE bytes long.
   The pointer to the stuff to write is P, and we write to the file
   OUT_FILE.  */

void
do_dump (char *p, int item_size, int nitems,  gzFile out_file)
{
#if !defined (WORDS_BIGENDIAN) && !defined (NO_DUMP_SHARE)
  swap_items (p, nitems, item_size);
#endif

  if (gzwrite (out_file, p, item_size * nitems) != item_size * nitems)
    {
      fprintf (stderr, "! Could not write %d %d-byte item(s) to %s.\n",
               nitems, item_size, name_of_file+1);
      exit (1);
    }

  /* Have to restore the old contents of memory, since some of it might
     get used again.  */
#if !defined (WORDS_BIGENDIAN) && !defined (NO_DUMP_SHARE)
  swap_items (p, nitems, item_size);
#endif
}


/* Here is the dual of the writing routine.  */

void
do_undump (char *p, int item_size, int nitems, gzFile in_file)
{
  if (gzread (in_file, p, item_size * nitems) != item_size * nitems)
    FATAL3 ("Could not undump %d %d-byte item(s) from %s",
            nitems, item_size, name_of_file+1);

#if !defined (WORDS_BIGENDIAN) && !defined (NO_DUMP_SHARE)
  swap_items (p, nitems, item_size);
#endif
}

/* FIXME -- some (most?) of this can/should be moved to the Pascal/WEB side. */
static void
checkpool_pointer (pool_pointer pool_ptr, size_t len)
{
  if (pool_ptr + len >= pool_size) {
    fprintf (stderr, "\nstring pool overflow [%i bytes]\n",
            (int)pool_size); /* fixme */
    exit(1);
  }
}

int
maketexstring(const_string s)
{
  size_t len;
  UInt32 rval;
  const unsigned char *cp = (const unsigned char *)s;

  if (s == NULL || *s == 0)
    return get_nullstr();

  len = strlen(s);
  checkpool_pointer (pool_ptr, len); /* in the XeTeX case, this may be more than enough */

  while ((rval = *(cp++)) != 0) {
    UInt16 extraBytes = bytesFromUTF8[rval];
    switch (extraBytes) { /* note: code falls through cases! */
      case 5: rval <<= 6; if (*cp) rval += *(cp++);
      case 4: rval <<= 6; if (*cp) rval += *(cp++);
      case 3: rval <<= 6; if (*cp) rval += *(cp++);
      case 2: rval <<= 6; if (*cp) rval += *(cp++);
      case 1: rval <<= 6; if (*cp) rval += *(cp++);
      case 0: ;
    };
    rval -= offsetsFromUTF8[extraBytes];
    if (rval > 0xffff) {
      rval -= 0x10000;
      str_pool[pool_ptr++] = 0xd800 + rval / 0x0400;
      str_pool[pool_ptr++] = 0xdc00 + rval % 0x0400;
    }
    else
      str_pool[pool_ptr++] = rval;
  }

  return make_string();
}

str_number
make_full_name_string(void)
{
  return maketexstring(fullnameoffile);
}

/* Get the job name to be used, which may have been set from the
   command line. */
str_number
get_job_name(str_number name)
{
    str_number ret = name;
    if (c_job_name != NULL)
      ret = maketexstring(c_job_name);
    return ret;
}

static int
compare_paths (const_string p1, const_string p2)
{
  int ret;
  while (
         (((ret = (*p1 - *p2)) == 0) && (*p2 != 0))
                || (IS_DIR_SEP(*p1) && IS_DIR_SEP(*p2))) {
       p1++, p2++;
  }
  ret = (ret < 0 ? -1 : (ret > 0 ? 1 : 0));
  return ret;
}

string
gettexstring (str_number s)
{
  unsigned bytesToWrite = 0;
  pool_pointer len, i, j;
  string name;
  len = str_start[s + 1 - 65536L] - str_start[s - 65536L];
  name = xmalloc(len * 3 + 1); /* max UTF16->UTF8 expansion
                                  (code units, not bytes) */
  for (i = 0, j = 0; i < len; i++) {
    unsigned c = str_pool[i + str_start[s - 65536L]];
    if (c >= 0xD800 && c <= 0xDBFF) {
      unsigned lo = str_pool[++i + str_start[s - 65536L]];
      if (lo >= 0xDC00 && lo <= 0xDFFF)
        c = (c - 0xD800) * 0x0400 + lo - 0xDC00;
      else
        c = 0xFFFD;
    }
    if (c < 0x80)
      bytesToWrite = 1;
    else if (c < 0x800)
      bytesToWrite = 2;
    else if (c < 0x10000)
      bytesToWrite = 3;
    else if (c < 0x110000)
      bytesToWrite = 4;
    else {
      bytesToWrite = 3;
      c = 0xFFFD;
    }

    j += bytesToWrite;
    switch (bytesToWrite) { /* note: everything falls through. */
      case 4: name[--j] = ((c | 0x80) & 0xBF); c >>= 6;
      case 3: name[--j] = ((c | 0x80) & 0xBF); c >>= 6;
      case 2: name[--j] = ((c | 0x80) & 0xBF); c >>= 6;
      case 1: name[--j] =  (c | firstByteMark[bytesToWrite]);
    }
    j += bytesToWrite;
  }
  name[j] = 0;
  return name;
}

boolean
is_new_source (str_number srcfilename, int lineno)
{
  char *name = gettexstring(srcfilename);
  return (compare_paths(name, last_source_name) != 0 || lineno != last_lineno);
}

void
remember_source_info (str_number srcfilename, int lineno)
{
  if (last_source_name)
       free(last_source_name);
  last_source_name = gettexstring(srcfilename);
  last_lineno = lineno;
}

pool_pointer
make_src_special (str_number srcfilename, int lineno)
{
  pool_pointer oldpool_ptr = pool_ptr;
  char *filename = gettexstring(srcfilename);
  /* FIXME: Magic number. */
  char buf[40];
  char *s = buf;

  /* Always put a space after the number, which makes things easier
   * to parse.
   */
  sprintf (buf, "src:%d ", lineno);

  if (pool_ptr + strlen(buf) + strlen(filename) >= (size_t)pool_size) {
       fprintf (stderr, "\nstring pool overflow\n"); /* fixme */
       exit (1);
  }
  s = buf;
  while (*s)
    str_pool[pool_ptr++] = *s++;

  s = filename;
  while (*s)
    str_pool[pool_ptr++] = *s++;

  return (oldpool_ptr);
}

#define xfree(p) do { if (p != NULL) free(p); p = NULL; } while (0)
#define PRINTF_BUF_SIZE 1024

static char print_buf[PRINTF_BUF_SIZE];

/* Helper for pdftex_fail. */
static void safe_print(const char *str)
{
    const char *c;
    for (c = str; *c; ++c)
        print(*c);
}

/* pdftex_fail may be called when a buffer overflow has happened/is
   happening, therefore may not call mktexstring.  However, with the
   current implementation it appears that error messages are misleading,
   possibly because pool overflows are detected too late.

   The output format of this fuction must be the same as pdf_error in
   pdftex.web! */
__attribute__ ((noreturn, format(printf, 1, 2)))
void pdftex_fail(const char *fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    print_ln();
    safe_print("!error: ");
    vsnprintf(print_buf, PRINTF_BUF_SIZE, fmt, args);
    safe_print(print_buf);
    va_end(args);
    print_ln();
    safe_print(" ==> Fatal error occurred, output file will be damaged!");
    print_ln();
    exit(EXIT_FAILURE);
}

/* Converts any given string in into an allowed PDF string which is
 * hexadecimal encoded;
 * sizeof(out) should be at least lin*2+1.
 */
void convertStringToHexString(const char *in, char *out, int lin)
{
    static const char hexchars[] = "0123456789ABCDEF";
    int i, j;
    j = 0;

    for (i = 0; i < lin; i++) {
	unsigned char c = (unsigned char) in[i];
	out[j++] = hexchars[(c >> 4) & 0xF];
	out[j++] = hexchars[c & 0xF];
    }
    out[j] = '\0';
}

#define DIGEST_SIZE 16
#define FILE_BUF_SIZE 1024

void getmd5sum(str_number s, boolean file)
{
    md5_state_t state;
    md5_byte_t digest[DIGEST_SIZE];
    char outbuf[2 * DIGEST_SIZE + 1];
    int len = 2 * DIGEST_SIZE;
    char *xname;
    int i;

    if (file) {
        char file_buf[FILE_BUF_SIZE];
        int read = 0;
        FILE *f;
        char *file_name;

        xname = gettexstring (s);
        file_name = kpse_find_file (xname, kpse_tex_format, true);
        xfree (xname);
        if (file_name == NULL) {
            return;             /* empty string */
        }
        /* in case of error the empty string is returned,
           no need for xfopen that aborts on error.
         */
        f = fopen(file_name, FOPEN_RBIN_MODE);
        if (f == NULL) {
            xfree(file_name);
            return;
        }
        /*recorder_record_input(file_name);*/
        md5_init(&state);
        while ((read = fread(&file_buf, sizeof(char), FILE_BUF_SIZE, f)) > 0) {
            md5_append(&state, (const md5_byte_t *) file_buf, read);
        }
        md5_finish(&state, digest);
        fclose(f);

        xfree(file_name);
    } else {
        /* s contains the data */
        md5_init(&state);
        xname = gettexstring (s);
        md5_append(&state,
                   (md5_byte_t *) xname,
                   strlen (xname));
        xfree (xname);
        md5_finish(&state, digest);
    }

    if (pool_ptr + len >= pool_size) {
        /* error by str_toks that calls str_room(1) */
        return;
    }

    convertStringToHexString((char *) digest, outbuf, DIGEST_SIZE);
    for (i = 0; i < 2 * DIGEST_SIZE; i++)
        str_pool[pool_ptr++] = (uint16_t)outbuf[i];
}
