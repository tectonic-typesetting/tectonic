/* This is (x)dvipdfmx, an extended version of...

    DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2002-2017 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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

#include "dpx-dvipdfmx.h"

#include <assert.h>
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core-bridge.h"
#include "dpx-cid.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-dvi.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfencrypt.h"
#include "dpx-pdffont.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-spc_tpic.h"
#include "dpx-specials.h"
#include "dpx-tfm.h"
#include "dpx-vf.h"
#include "dpx-tt_aux.h"

typedef struct page_range
{
  int first;
  int last;
} PageRange;

int is_xdv = 0;
int translate_origin = 0;

#define OPT_TPIC_TRANSPARENT_FILL (1 << 1)
#define OPT_CIDFONT_FIXEDPITCH    (1 << 2)
#define OPT_FONTMAP_FIRST_MATCH   (1 << 3)
#define OPT_PDFDOC_NO_DEST_REMOVE (1 << 4)
#define OPT_PDFOBJ_NO_PREDICTOR   (1 << 5)
#define OPT_PDFOBJ_NO_OBJSTM      (1 << 6)

static char     ignore_colors = 0;
static double   annot_grow    = 0.0;
static int      bookmark_open = 0;
static double   mag           = 1.0;
static int      font_dpi      = 600;

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

time_t source_date_epoch = (time_t) -1;

/* Page device */
double paper_width  = 595.0;
double paper_height = 842.0;
static double x_offset = 72.0;
static double y_offset = 72.0;
int    landscape_mode  = 0;

int always_embed = 0; /* always embed fonts, regardless of licensing flags */

/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
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
#define K_UNIT__PC  5
#define K_UNIT__DD  6
#define K_UNIT__CC  7
#define K_UNIT__SP  8
    "pt", "in", "cm", "mm", "bp", "pc", "dd", "cc", "sp",
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
      case K_UNIT__PC: u *= 12.0 * 72.0 / 72.27 ; break;
      case K_UNIT__DD: u *= 1238.0 / 1157.0 * 72.0 / 72.27 ; break;
      case K_UNIT__CC: u *= 12.0 * 1238.0 / 1157.0 * 72.0 / 72.27 ; break;
      case K_UNIT__SP: u *= 72.0 / (72.27 * 65536) ; break;
      default:
        dpx_warning("Unknown unit of measure: %s", q);
        error = -1;
        break;
      }
      free(qq);
    }
    else {
      dpx_warning("Missing unit of measure after \"true\"");
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
      _tt_abort("Unrecognized paper format: %s", paperspec);
    error = read_length(&paper_width,  &p, comma);
    p = comma + 1;
    error = read_length(&paper_height, &p, endptr);
  }
  if (error || paper_width <= 0.0 || paper_height <= 0.0)
    _tt_abort("Invalid paper size: %s (%.2fx%.2f)", paperspec, paper_width, paper_height);
}

static void
select_pages (
  const char *pagespec,
  PageRange **ret_page_ranges,
  unsigned int *ret_num_page_ranges)
{
  PageRange *page_ranges = NULL;
  unsigned int num_page_ranges = 0;
  unsigned int max_page_ranges = 0;
  char  *q;
  const char *p = pagespec;

  while (*p != '\0') {
    /* Enlarge page range table if necessary */
    if (num_page_ranges >= max_page_ranges) {
      max_page_ranges += 4;
      page_ranges = RENEW(page_ranges, max_page_ranges, PageRange);
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
        _tt_abort("Bad page range specification: %s", p);
    }
  }

  *ret_page_ranges = page_ranges;
  *ret_num_page_ranges = num_page_ranges;
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

#define SWAP(v1,v2) do {\
   double _tmp = (v1);\
   (v1) = (v2);\
   (v2) = _tmp;\
 } while (0)

static void
do_dvi_pages (PageRange *page_ranges, unsigned int num_page_ranges)
{
  int      page_no, step;
  unsigned int page_count, i;
  double   page_width, page_height;
  double   init_paper_width, init_paper_height;
  pdf_rect mediabox;

  spc_exec_at_begin_document();

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

        dpx_message("[%d", page_no+1);
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
        dpx_message("]");
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
    _tt_abort("No pages fall in range!");
  }

  spc_exec_at_end_document();
}


int
dvipdfmx_main (
  const char *pdf_filename,
  const char *dvi_filename,
  const char *pagespec,
  int opt_flags,
  bool translate,
  bool compress,
  bool deterministic_tags,
  bool quiet,
  unsigned int verbose,
  time_t build_date)
{
  bool enable_object_stream = true;
  double dvi2pts;
  unsigned int num_page_ranges = 0;
  PageRange *page_ranges = NULL;

  assert(pdf_filename);
  assert(dvi_filename);

  translate_origin = translate;


  dvi_reset_global_state();
  tfm_reset_global_state();
  vf_reset_global_state();
  pdf_dev_reset_global_state();
  pdf_obj_reset_global_state();
  pdf_font_reset_unique_tag_state();

  if (quiet) {
    shut_up(2);
  } else {

    dvi_set_verbose(verbose);
    pdf_dev_set_verbose(verbose);
    pdf_doc_set_verbose(verbose);
    pdf_enc_set_verbose(verbose);
    pdf_obj_set_verbose(verbose);
    pdf_fontmap_set_verbose(verbose);
    dpx_file_set_verbose(verbose);
    tt_aux_set_verbose(verbose);
  }

  pdf_set_compression(compress ? 9 : 0);
  pdf_font_set_deterministic_unique_tags(deterministic_tags ? 1 : 0);

  system_default();

  pdf_init_fontmaps(); /* This must come before parsing options... */

  /* We used to read the config file here. It synthesized command-line
   * arguments, so we emulate the default TeXLive config file by copying those
   * code bits. */

  pdf_set_version (5);
  select_paper("letter");
  annot_grow = 0;
  bookmark_open = 0;
  key_bits = 40;
  permission = 0x003C;
  font_dpi = 600;
  pdfdecimaldigits = 5;
  image_cache_life = -2;
  source_date_epoch = build_date;
  pdf_load_fontmap_file("pdftex.map", FONTMAP_RMODE_APPEND);
  pdf_load_fontmap_file("kanjix.map", FONTMAP_RMODE_APPEND);
  pdf_load_fontmap_file("ckx.map", FONTMAP_RMODE_APPEND);

  if (pagespec) {
    select_pages(pagespec, &page_ranges, &num_page_ranges);
  }
  if (!page_ranges) {
    page_ranges = NEW(1, PageRange);
  }
  if (num_page_ranges == 0) {
    page_ranges[0].first = 0;
    page_ranges[0].last  = -1; /* last page */
    num_page_ranges = 1;
  }

  /*kpse_init_prog("", font_dpi, NULL, NULL);
    kpse_set_program_enabled(kpse_pk_format, true, kpse_src_texmf_cnf);*/
  pdf_font_set_dpi(font_dpi);
  dpx_delete_old_cache(image_cache_life);

  pdf_enc_compute_id_string(dvi_filename, pdf_filename);

  {
    int ver_major = 0,  ver_minor = 0;
    char owner_pw[MAX_PWD_LEN], user_pw[MAX_PWD_LEN];
    /* Dependency between DVI and PDF side is rather complicated... */
    dvi2pts = dvi_init(dvi_filename, mag);
    if (dvi2pts == 0.0)
      _tt_abort("dvi_init() failed!");

    pdf_doc_set_creator(dvi_comment());

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
        _tt_abort("Invalid encryption key length specified: %u", key_bits);
      else if (key_bits > 40 && pdf_get_version() < 4)
        _tt_abort("Chosen key length requires at least PDF 1.4. " \
              "Use \"-V 4\" to change.");
      do_encryption = 1;
      pdf_enc_set_passwd(key_bits, permission, owner_pw, user_pw);
    }
    if (landscape_mode) {
      SWAP(paper_width, paper_height);
    }
  }

  pdf_files_init();

  if (opt_flags & OPT_PDFOBJ_NO_OBJSTM)
    enable_object_stream = false;

  /* Set default paper size here so that all page's can inherite it.
   * annot_grow:    Margin of annotation.
   * bookmark_open: Miximal depth of open bookmarks.
   */
  pdf_open_document(pdf_filename, do_encryption, enable_object_stream,
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

  do_dvi_pages(page_ranges, num_page_ranges);

  pdf_files_close();

  /* Order of close... */
  pdf_close_device  ();
  /* pdf_close_document flushes XObject (image) and other resources. */
  pdf_close_document();

  pdf_close_fontmaps(); /* pdf_font may depend on fontmap. */

  dvi_close();

  dpx_message("\n");
  free(page_ranges);

  return 0;
}
