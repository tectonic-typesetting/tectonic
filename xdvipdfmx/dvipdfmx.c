/* This is (x)dvipdfmx, an extended version of...

    DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
    the DVIPDFMx project team.

    Copyright (c) 2006 SIL. (xdvipdfmx extensions for XeTeX support)

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/

#include <stdio.h>
#include <string.h>
#include <limits.h>
#include <ctype.h>

#include <getopt.h>

#include <tectonic/dpx-system.h>
#include <tectonic/dpx-mem.h>

#include <tectonic/dpx-dpxconf.h>
#include <tectonic/dpx-dpxfile.h>
#include <tectonic/dpx-dpxutil.h>

#include <tectonic/dpx-dvi.h>

#include <tectonic/dpx-pdflimits.h>
#include <tectonic/dpx-pdfdoc.h>
#include <tectonic/dpx-pdfdev.h>
#include <tectonic/dpx-pdfparse.h>
#include <tectonic/dpx-pdfencrypt.h>

#include "spc_tpic.h"
#include <tectonic/dpx-specials.h>

#include <tectonic/dpx-mpost.h>

#include <tectonic/dpx-fontmap.h>
#include <tectonic/dpx-pdffont.h>
#include <tectonic/dpx-pdfximage.h>
#include <tectonic/dpx-cid.h>

#include <tectonic/dpx-dvipdfmx.h>
#include "tt_aux.h"

#include <tectonic/dpx-error.h>

int is_xdv = 0;
int translate_origin = 0;

#if defined(LIBDPX)
const char *my_name = "ApTeX";
#else
const char *my_name;
#endif /* LIBDPX */

int compat_mode = 0;     /* 0 = dvipdfmx, 1 = dvipdfm */

static int verbose = 0;

static int opt_flags = 0;

#define OPT_TPIC_TRANSPARENT_FILL (1 << 1)
#define OPT_CIDFONT_FIXEDPITCH    (1 << 2)
#define OPT_FONTMAP_FIRST_MATCH   (1 << 3)
#define OPT_PDFDOC_NO_DEST_REMOVE (1 << 4)
#define OPT_PDFOBJ_NO_PREDICTOR   (1 << 5)
#define OPT_PDFOBJ_NO_OBJSTM      (1 << 6)

static char   ignore_colors = 0;
static double annot_grow    = 0.0;
static int    bookmark_open = 0;
static double mag           = 1.0;
static int    font_dpi      = 600;
static int    really_quiet  = 0;
/*
 * Precision is essentially limited to 0.01pt.
 * See, dev_set_string() in pdfdev.c.
 */
static int pdfdecimaldigits = 3;

/* Image cache life in hours */
/*  0 means erase all old images and leave new images */
/* -1 means erase all old images and also erase new images */
/* -2 means ignore image cache (default) */
static int image_cache_life = -2;

/* Encryption */
static int     do_encryption = 0;
static int     key_bits      = 40;
static int32_t permission    = 0x003C;

/* Object stream */
static int     enable_objstm = 1;

/* Page device */
double paper_width  = 595.0;
double paper_height = 842.0;
static double x_offset = 72.0;
static double y_offset = 72.0;
int    landscape_mode  = 0;

int always_embed = 0; /* always embed fonts, regardless of licensing flags */

char *dvi_filename = NULL, *pdf_filename = NULL;


static const_string
xbasename (const_string name)
{
    const_string base = name;
    const_string p;

    for (p = base; *p; p++) {
        if (IS_DIR_SEP(*p))
            base = p + 1;
    }

    return base;
}


static void
read_config_file (const char *config);

#define FILESTRCASEEQ(a,b) (strcmp((a), (b)) == 0)

static void
set_default_pdf_filename(void)
{
  const char *dvi_base;

  dvi_base = xbasename(dvi_filename);
if (strlen(dvi_base) > 4 &&
    (FILESTRCASEEQ(".dvi", dvi_base+strlen(dvi_base)-4) ||
     FILESTRCASEEQ(".xdv", dvi_base+strlen(dvi_base)-4))) {
    pdf_filename = NEW(strlen(dvi_base)+1, char);
    strncpy(pdf_filename, dvi_base, strlen(dvi_base)-4);
    pdf_filename[strlen(dvi_base)-4] = '\0';
  } else {
    pdf_filename = NEW(strlen(dvi_base)+5, char);
    strcpy(pdf_filename, dvi_base);
  }

  strcat (pdf_filename, ".pdf");
}

static void
usage (void)
{
  fprintf (stderr, "\nTry \"%s --help\" for more information.\n", my_name);
  exit(1);
}


static int
read_length (double *vp, const char **pp, const char *endptr)
{
  char   *q;
  const char *p = *pp;
  double  v, u = 1.0;
  const char *_ukeys[] = {
#define K_UNIT__PT  0
#define K_UNIT__IN  1
#define K_UNIT__CM  2
#define K_UNIT__MM  3
#define K_UNIT__BP  4
    "pt", "in", "cm", "mm", "bp",
     NULL
  };
  int     k, error = 0;

  q = parse_float_decimal(&p, endptr);
  if (!q) {
    *vp = 0.0; *pp = p;
    return  -1;
  }

  v = atof(q);
  free(q);

  skip_white(&p, endptr);
  q = parse_c_ident(&p, endptr);
  if (q) {
    char *qq = q;
    if (strlen(q) >= strlen("true") &&
        !memcmp(q, "true", strlen("true"))) {
      q += strlen("true"); /* just skip "true" */
    }
    if (strlen(q) == 0) {
      free(qq);
      skip_white(&p, endptr);
      qq = q = parse_c_ident(&p, endptr);
    }
    if (q) {
      for (k = 0; _ukeys[k] && strcmp(_ukeys[k], q); k++);
      switch (k) {
      case K_UNIT__PT: u *= 72.0 / 72.27; break;
      case K_UNIT__IN: u *= 72.0; break;
      case K_UNIT__CM: u *= 72.0 / 2.54 ; break;
      case K_UNIT__MM: u *= 72.0 / 25.4 ; break;
      case K_UNIT__BP: u *= 1.0 ; break;
      default:
        WARN("Unknown unit of measure: %s", q);
        error = -1;
        break;
      }
      free(qq);
    }
    else {
      WARN("Missing unit of measure after \"true\"");
      error = -1;
    }
  }

  *vp = v * u; *pp = p;
  return  error;
}

static void
select_paper (const char *paperspec)
{
  const struct paper *pi;
  int   error = 0;

  pi = paperinfo(paperspec);
  if (pi && papername(pi)) {
    paper_width  = paperpswidth (pi);
    paper_height = paperpsheight(pi);
  } else {
    const char  *p = paperspec, *endptr, *comma;
    comma  = strchr(p, ',');
    endptr = p + strlen(p);
    if (!comma)
      ERROR("Unrecognized paper format: %s", paperspec);
    error = read_length(&paper_width,  &p, comma);
    p = comma + 1;
    error = read_length(&paper_height, &p, endptr);
  }
  if (error || paper_width <= 0.0 || paper_height <= 0.0)
    ERROR("Invalid paper size: %s (%.2fx%.2f)", paperspec, paper_width, paper_height);
}

struct page_range
{
  int first, last;
} *page_ranges = NULL;

int num_page_ranges = 0;
int max_page_ranges = 0;

static void
select_pages (const char *pagespec)
{
  char  *q;
  const char *p = pagespec;

  while (*p != '\0') {
    /* Enlarge page range table if necessary */
    if (num_page_ranges >= max_page_ranges) {
      max_page_ranges += 4;
      page_ranges = RENEW(page_ranges, max_page_ranges, struct page_range);
    }

    page_ranges[num_page_ranges].first = 0;
    page_ranges[num_page_ranges].last  = 0;

    for ( ; *p && isspace((unsigned char)*p); p++);
    q = parse_unsigned(&p, p + strlen(p)); /* Can't be signed. */
    if (q) { /* '-' is allowed here */
      page_ranges[num_page_ranges].first = atoi(q) - 1;
      page_ranges[num_page_ranges].last  = page_ranges[num_page_ranges].first;
      free(q);
    }
    for ( ; *p && isspace((unsigned char)*p); p++);

    if (*p == '-') {
      for (++p; *p && isspace((unsigned char)*p); p++);
      page_ranges[num_page_ranges].last = -1;
      if (*p) {
        q = parse_unsigned(&p, p + strlen(p));
        if (q) {
          page_ranges[num_page_ranges].last = atoi(q) - 1;
          free(q);
        }
        for ( ; *p && isspace((unsigned char)*p); p++);
      }
    } else {
      page_ranges[num_page_ranges].last = page_ranges[num_page_ranges].first;
    }

    num_page_ranges++;

    if (*p == ',')
      p++;
    else  {
      for ( ; *p && isspace((unsigned char)*p); p++);
      if (*p)
        ERROR("Bad page range specification: %s", p);
    }
  }
  return;
}

static const char *optstrig = ":hD:r:m:g:x:y:o:s:t:p:clf:i:qvV:z:d:I:S:K:P:O:MC:Ee";

static struct option long_options[] = {
  {"help", 0, 0, 'h'},
  {"version", 0, 0, 130},
  {"showpaper", 0, 0, 131},
  {"dvipdfm", 0, 0, 132},
  {"mvorigin", 0, 0, 1000},
  {"kpathsea-debug", 1, 0, 133},
  {0, 0, 0, 0}
};

static void
do_early_args (int argc, char *argv[])
{
  int c;

  while ((c = getopt_long(argc, argv, optstrig, long_options, NULL)) != -1) {
    switch(c) {
    case 'h':
      exit(0);
      break;

    case 130: /* --version */
      exit(0);
      break;

    case 131: /* --showpaper */
      dumppaperinfo();
      exit(0);
      break;

    case 1000: /* --mvorigin */
      translate_origin = 1;
      break;

    case 'q':
      really_quiet = 2;
      break;

    case 'v':
      verbose++;

    default: /* ignore everything else */
      break;
    }
  }

  if (really_quiet)
    shut_up(really_quiet);
  else {
    int i;

    for (i = 0; i < verbose; i++) {
      dvi_set_verbose();
      pdf_dev_set_verbose();
      pdf_doc_set_verbose();
      pdf_enc_set_verbose();
      pdf_obj_set_verbose();
      pdf_fontmap_set_verbose();
      dpx_file_set_verbose();
      tt_aux_set_verbose();
    }
  }
}

/* Set "unsafe" to non-zero value when parsing config specials to
 * disallow overriding "D" option value.
 */
static void
do_args (int argc, char *argv[], const char *source, int unsafe)
{
  int c;
  char *nextptr;
  const char *nnextptr;

  optind = 1;

  while ((c = getopt_long(argc, argv, optstrig, long_options, NULL)) != -1) {
    switch(c) {
    case 'h': case 130: case 131: case 1000: case 'q': case 'v': /* already done */
      break;

    case 132: /* --dvipdfm */
      compat_mode = 1;
      break;

    case 133: /* --kpathsea-debug */
	/*kpathsea_debug = atoi(optarg);*/
      break;

    case 'D':
      if (unsafe) {
        WARN("Ignoring \"D\" option for dvipdfmx:config special. (unsafe)");
      } else {
      set_distiller_template(optarg);
      }
      break;

    case 'r':
      if ((font_dpi = atoi(optarg)) <= 0)
        ERROR("Invalid bitmap font dpi specified: %s", optarg);
      break;

    case 'm':
      if ((mag = strtod(optarg, &nextptr)) < 0.0 || nextptr == optarg)
        ERROR("Invalid magnification specified: %s", optarg);
      break;

    case 'g':
      nnextptr = nextptr = optarg;
      read_length(&annot_grow, &nnextptr, nextptr + strlen(nextptr));
      break;

    case 'x':
      nnextptr = nextptr = optarg;
      read_length(&x_offset, &nnextptr, nextptr + strlen(nextptr));
      break;

    case 'y':
      nnextptr = nextptr = optarg;
      read_length(&y_offset, &nnextptr, nextptr + strlen(nextptr));
      break;

    case 'o':
      pdf_filename = NEW (strlen(optarg)+1, char);
      strcpy(pdf_filename, optarg);
      break;

    case 's':
      select_pages(optarg);
      break;

    case 't':
      pdf_doc_enable_manual_thumbnails();
      break;

    case 'p':
      select_paper(optarg);
      break;

    case 'c':
      ignore_colors = 1;
      break;

    case 'l':
      landscape_mode = 1;
      break;

    case 'f':
      if (opt_flags & OPT_FONTMAP_FIRST_MATCH)
        pdf_load_fontmap_file(optarg, FONTMAP_RMODE_APPEND);
      else
        pdf_load_fontmap_file(optarg, FONTMAP_RMODE_REPLACE);
      break;

    case 'i':
    {
      int optind_save= optind;
      read_config_file(optarg);
      optind = optind_save;
      break;
    }

    case 'V':
    {
      int ver_minor = atoi(optarg);
      if (ver_minor < PDF_VERSION_MIN) {
        WARN("PDF version 1.%d not supported. Using PDF 1.%d instead.",
             ver_minor, PDF_VERSION_MIN);
        ver_minor = PDF_VERSION_MIN;
      } else if (ver_minor > PDF_VERSION_MAX) {
        WARN("PDF version 1.%d not supported. Using PDF 1.%d instead.",
             ver_minor, PDF_VERSION_MAX);
        ver_minor = PDF_VERSION_MAX;
      }
      pdf_set_version((unsigned) ver_minor);
      break;
    }

    case 'z':
      pdf_set_compression(atoi(optarg));
      break;

    case 'd':
      pdfdecimaldigits = atoi(optarg);
      break;

    case 'I':
      image_cache_life = atoi(optarg);
      break;

    case 'S':
      do_encryption = 1;
      break;

    case 'K':
      key_bits = (unsigned) atoi(optarg);
      if (!(key_bits >= 40 && key_bits <= 128 && (key_bits % 8 == 0)) &&
            key_bits != 256)
        ERROR("Invalid encryption key length specified: %s", optarg);
      break;

    case 'P':
      permission = (unsigned) strtoul(optarg, &nextptr, 0);
      if (nextptr == optarg)
        ERROR("Invalid encryption permission flag: %s", optarg);
      break;

    case 'O':
      bookmark_open = atoi(optarg);
      break;

    case 'C':
    {
      int flags = (unsigned) strtol(optarg, &nextptr, 0);
      if (nextptr == optarg)
        ERROR("Invalid flag: %s", optarg);
      if (flags < 0)
        opt_flags  = -flags;
      else
        opt_flags |=  flags;
      break;
    }

    case 'E':
      always_embed = 1;
      break;

    case 'e':
      if (compat_mode) {
        WARN("dvipdfm \"-e\" option not supported.");
        break;
      } /* else fall through */

    default:
      fprintf(stderr, "%s: %s \"-%c\"\n", source ? source : my_name,
              c == ':' ? "Missing argument for" : "Unknown option",
              optopt);
      usage();
    }
  }

  if (source) {
    if (argc > optind)
      fprintf(stderr, "%s: Unexpected argument in \"%s %s\".\n", source, argv[1], argv[2]);
    return;
  }

  if (argc > optind + 1) {
    fprintf(stderr, "%s: Multiple dvi filenames?", my_name);
    usage();
  } else if (argc > optind) {
    dvi_filename = NEW(strlen(argv[optind]) + 5, char);  /* space to append ".dvi" */
    strcpy(dvi_filename, argv[optind]);
  }
}

static void
cleanup (void)
{
  if (dvi_filename)
    free(dvi_filename);
  if (pdf_filename)
    free(pdf_filename);
  if (page_ranges)
    free(page_ranges);
}

static void
read_config_file (const char *config)
{
  const char *start, *end;
  char *option;
  FILE *fp;
  static char argv0[] = "config_file";
  char *argv[3];

  fp = dpx_open_file(config, DPX_RES_TYPE_TEXT);
  if (!fp) {
    WARN("Could not open config file \"%s\".", config);
    return;
  }
  argv[0] = argv0;
  while ((start = mfgets (work_buffer, WORK_BUFFER_SIZE, fp)) != NULL) {
    int   argc = 1;

    end = work_buffer + strlen(work_buffer);
    skip_white (&start, end);
    if (start >= end)
      continue;
    /* Build up an argument list as if it were passed on the command
       line */
    if ((option = parse_ident (&start, end))) {
      argc = 2;
      argv[1] = NEW (strlen(option)+2, char);
      strcpy (argv[1]+1, option);
      free (option);
      *argv[1] = '-';
      skip_white (&start, end);
      if (start < end) {
        argc += 1;
        if (*start == '"') {
          argv[2] = parse_c_string (&start, end);
        }
        else
          argv[2] = parse_ident (&start, end);
      }
    }
    do_args (argc, argv, config, 0);
    while (argc > 1) {
      free (argv[--argc]);
    }
  }
  if (fp)
    fclose(fp);
}

void
read_config_special (const char **start, const char *end)
{
  char *option;
  static char argv0[] = "config_special";
  char *argv[3];
  int argc = 1;

  argv[0] = argv0;

  skip_white (start, end);
  if (*start >= end)
    return;
  /* Build up an argument list as if it were passed on the command
     line */
  if ((option = parse_ident (start, end))) {
    argc = 2;
    argv[1] = NEW (strlen(option)+2, char);
    strcpy (argv[1]+1, option);
    free (option);
    *argv[1] = '-';
    skip_white (start, end);
    if (*start < end) {
      argc += 1;
      if (**start == '"') {
	argv[2] = parse_c_string (start, end);
      }
      else
	argv[2] = parse_ident (start, end);
    }
  }
  do_args (argc, argv, argv0, 1); /* Set to unsafe */
  while (argc > 1) {
    free (argv[--argc]);
  }
}

static void
system_default (void)
{
  if (systempapername() != NULL) {
    select_paper(systempapername());
  } else if (defaultpapername() != NULL) {
    select_paper(defaultpapername());
  }
}

void
error_cleanup (void)
{
  pdf_close_images();  /* delete temporary files */
  pdf_error_cleanup();
  if (pdf_filename) {
    remove(pdf_filename);
    fprintf(stderr, "\nOutput file removed.\n");
  }
}

#define SWAP(v1,v2) do {\
   double _tmp = (v1);\
   (v1) = (v2);\
   (v2) = _tmp;\
 } while (0)

static void
do_dvi_pages (void)
{
  int      page_no, page_count, i, step;
  double   page_width, page_height;
  double   init_paper_width, init_paper_height;
  pdf_rect mediabox;

  spc_exec_at_begin_document();

  if (num_page_ranges == 0) {
    if (!page_ranges) {
      page_ranges = NEW(1, struct page_range);
      max_page_ranges = 1;
    }
    page_ranges[0].first = 0;
    page_ranges[0].last  = -1; /* last page */
    num_page_ranges = 1;
  }

  init_paper_width  = page_width  = paper_width;
  init_paper_height = page_height = paper_height;
  page_count  = 0;

  mediabox.llx = 0.0;
  mediabox.lly = 0.0;
  mediabox.urx = paper_width;
  mediabox.ury = paper_height;

  pdf_doc_set_mediabox(0, &mediabox); /* Root node */

  for (i = 0; i < num_page_ranges && dvi_npages(); i++) {
    if (page_ranges[i].last < 0)
      page_ranges[i].last += dvi_npages();

    step    = (page_ranges[i].first <= page_ranges[i].last) ? 1 : -1;
    page_no = page_ranges[i].first;
    while (dvi_npages()) {
      if (page_no < dvi_npages()) {
        double w, h, xo, yo;
        int    lm;

        MESG("[%d", page_no+1);
        /* Users want to change page size even after page is started! */
        page_width = paper_width; page_height = paper_height;
        w = page_width; h = page_height; lm = landscape_mode;
        xo = x_offset; yo = y_offset;
        dvi_scan_specials(page_no, &w, &h, &xo, &yo, &lm, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
        if (lm != landscape_mode) {
          SWAP(w, h);
          landscape_mode = lm;
        }
        if (page_width  != w || page_height != h) {
          page_width  = w;
          page_height = h;
        }
        if (x_offset != xo || y_offset != yo) {
          x_offset = xo;
          y_offset = yo;
        }
        if (page_width  != init_paper_width ||
            page_height != init_paper_height) {
          mediabox.llx = 0.0;
          mediabox.lly = 0.0;
          mediabox.urx = page_width;
          mediabox.ury = page_height;
          pdf_doc_set_mediabox(page_count+1, &mediabox);
        }
        dvi_do_page(page_height, x_offset, y_offset);
        page_count++;
        MESG("]");
      }

      if (step > 0 &&
          page_no >= page_ranges[i].last)
        break;
      else if (step < 0 &&
               page_no <= page_ranges[i].last)
        break;
      else {
        page_no += step;
      }
    }
  }

  if (page_count < 1) {
    ERROR("No pages fall in range!");
  }

  spc_exec_at_end_document();
}


int
dvipdfmx_main (int argc, char *argv[])
{
  double dvi2pts;
  char *base;

  /*kpse_set_program_name(argv[0], "dvipdfmx");*/ /* we pretend to be dvipdfmx for kpse purposes */

  my_name = "xdvipdfmx";
  opterr = 0;

  /* Special-case single option --mvorigin, --help, --showpaper, or --version,
     to avoid possible diagnostics about config files, etc.
     Also handle -q and -v that cannot be set in config file. */
  do_early_args(argc, argv);

  system_default();

  pdf_init_fontmaps(); /* This must come before parsing options... */

  read_config_file(DPX_CONFIG_FILE);

  do_args (argc, argv, NULL, 0);

  /*kpse_init_prog("", font_dpi, NULL, NULL);
    kpse_set_program_enabled(kpse_pk_format, true, kpse_src_texmf_cnf);*/
  pdf_font_set_dpi(font_dpi);
  dpx_delete_old_cache(image_cache_life);

  if (!dvi_filename) {
    if (verbose)
      MESG("No dvi filename specified, reading standard input.\n");
    if (!pdf_filename)
      if (verbose)
        MESG("No pdf filename specified, writing to standard output.\n");
  } else if (!pdf_filename)
    set_default_pdf_filename();

  if (pdf_filename && !strcmp(pdf_filename, "-")) {
    free(pdf_filename);
    pdf_filename = NULL;
  }

  MESG("%s -> %s\n", dvi_filename ? dvi_filename : "stdin",
                     pdf_filename ? pdf_filename : "stdout");

  pdf_enc_compute_id_string(dvi_filename, pdf_filename);
  if (do_encryption) {
    if (key_bits > 40 && pdf_get_version() < 4)
      ERROR("Chosen key length requires at least PDF 1.4. "
            "Use \"-V 4\" to change.");
    pdf_enc_set_passwd(key_bits, permission, NULL, NULL);
  }

  {
    int ver_major = 0,  ver_minor = 0;
    char owner_pw[MAX_PWD_LEN], user_pw[MAX_PWD_LEN];
    /* Dependency between DVI and PDF side is rather complicated... */
    dvi2pts = dvi_init(dvi_filename, mag);
    if (dvi2pts == 0.0)
      ERROR("dvi_init() failed!");

    pdf_doc_set_creator(dvi_comment());

    if (do_encryption) {
      /* command line takes precedence */
      dvi_scan_specials(0,
                        &paper_width, &paper_height,
                        &x_offset, &y_offset, &landscape_mode,
                        &ver_major, &ver_minor,
                        NULL, NULL, NULL, NULL, NULL);
      /* FIXME: pdf_set_version() should come before ecrcyption setting.
       *        It's too late to set here...
       */
      if (ver_minor >= PDF_VERSION_MIN && ver_minor <= PDF_VERSION_MAX) {
        pdf_set_version(ver_minor);
      }
    } else {
      dvi_scan_specials(0,
                        &paper_width, &paper_height,
                        &x_offset, &y_offset, &landscape_mode,
                        &ver_major, &ver_minor,
                        &do_encryption, &key_bits, &permission, owner_pw, user_pw);
      if (ver_minor >= PDF_VERSION_MIN && ver_minor <= PDF_VERSION_MAX) {
        pdf_set_version(ver_minor);
      }
      if (do_encryption) {
        if (!(key_bits >= 40 && key_bits <= 128 && (key_bits % 8 == 0)) &&
              key_bits != 256)
          ERROR("Invalid encryption key length specified: %u", key_bits);
        else if (key_bits > 40 && pdf_get_version() < 4)
          ERROR("Chosen key length requires at least PDF 1.4. " \
                "Use \"-V 4\" to change.");
        do_encryption = 1;
        pdf_enc_set_passwd(key_bits, permission, owner_pw, user_pw);
      }
    }
    if (landscape_mode) {
      SWAP(paper_width, paper_height);
    }
  }

  pdf_files_init();

  if (opt_flags & OPT_PDFOBJ_NO_OBJSTM)
    enable_objstm = 0;

  /* Set default paper size here so that all page's can inherite it.
   * annot_grow:    Margin of annotation.
   * bookmark_open: Miximal depth of open bookmarks.
   */
  pdf_open_document(pdf_filename, do_encryption, enable_objstm,
                    paper_width, paper_height, annot_grow, bookmark_open,
                    !(opt_flags & OPT_PDFDOC_NO_DEST_REMOVE));

  /* Ignore_colors placed here since
   * they are considered as device's capacity.
   */
  pdf_init_device(dvi2pts, pdfdecimaldigits, ignore_colors);

  if (opt_flags & OPT_CIDFONT_FIXEDPITCH)
    CIDFont_set_flags(CIDFONT_FORCE_FIXEDPITCH);

  /* Please move this to spc_init_specials(). */
  if (opt_flags & OPT_TPIC_TRANSPARENT_FILL)
    tpic_set_fill_mode(1);

  if (opt_flags & OPT_PDFOBJ_NO_PREDICTOR)
    pdf_set_use_predictor(0); /* No prediction */

  do_dvi_pages();

  pdf_files_close();

  /* Order of close... */
  pdf_close_device  ();
  /* pdf_close_document flushes XObject (image) and other resources. */
  pdf_close_document();

  pdf_close_fontmaps(); /* pdf_font may depend on fontmap. */

  dvi_close();

  MESG("\n");
  cleanup();

  return 0;
}


int
dvipdfmx_simple_main(char *dviname, char *pdfname)
{
    char *argv[] = { "dvipdfmx", "-o", pdfname, dviname };
    return dvipdfmx_main(4, argv);
}
