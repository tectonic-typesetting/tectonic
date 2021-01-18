/* This is (x)dvipdfmx, an extended version of...

    DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2002-2020 by Jin-Hwan Cho, Matthias Franz, Shunsaku Hirata,
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

#include "tectonic_bridge_core.h"
#include "dpx-cid.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxcrypt.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-dvi.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-mpost.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfencrypt.h"
#include "dpx-pdffont.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfximage.h"
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

const XdvipdfmxConfig* dpx_config;

#define OPT_TPIC_TRANSPARENT_FILL (1 << 1)
#define OPT_CIDFONT_FIXEDPITCH    (1 << 2)
#define OPT_FONTMAP_FIRST_MATCH   (1 << 3)
#define OPT_PDFDOC_NO_DEST_REMOVE (1 << 4)
#define OPT_PDFOBJ_NO_PREDICTOR   (1 << 5)
#define OPT_PDFOBJ_NO_OBJSTM      (1 << 6)

static int    pdf_version_major = 1;
static int    pdf_version_minor = 5;
static int    compression_level = 9;

static char     ignore_colors = 0;
static double   annot_grow    = 0.0;
static int      bookmark_open = 0;
static double   mag           = 1.0;
static int      font_dpi      = 600;
static int      enable_thumbnail = 0;

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
/* Image format conversion filter template */
static char   *filter_template  = NULL;


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
static int translate_origin = 0;


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

PageRange *page_ranges = NULL;
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
}

#define SWAP(v1,v2) do {\
   double _tmp = (v1);\
   (v1) = (v2);\
   (v2) = _tmp;\
 } while (0)

static void
do_dvi_pages (void)
{
  int      page_no, step;
  unsigned int page_count, i;
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

        dpx_message("[%d", page_no+1);
        /* Users want to change page size even after page is started! */
        page_width = paper_width; page_height = paper_height;
        w = page_width; h = page_height; lm = landscape_mode;
        xo = x_offset; yo = y_offset;
        dvi_scan_specials(page_no,
                          &w, &h, &xo, &yo, &lm,
                          NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL);
        if (lm != landscape_mode) { /* already swapped for the first page */
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

static void
compute_id_string (unsigned char *id, const char *producer,
                   const char *dviname, const char *pdfname)
{
  char        datestr[32];
  MD5_CONTEXT md5;

  MD5_init(&md5);
  /* Don't use timezone for compatibility */
  dpx_util_format_asn_date(datestr, 0);
  MD5_write(&md5, (const unsigned char *)datestr, strlen(datestr));
  if (producer)
    MD5_write(&md5, (const unsigned char *)producer, strlen(producer));
  if (dviname)
    MD5_write(&md5, (const unsigned char *)dviname, strlen(dviname));
  if (pdfname)
    MD5_write(&md5, (const unsigned char *)pdfname, strlen(pdfname));
  MD5_final(id, &md5);
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
  double dvi2pts;
  const char *creator = NULL;
  char oplain[128] = "", uplain[128] = "";
  int has_id = 0;
  unsigned char id1[16], id2[16];
  struct pdf_setting settings;

  assert(pdf_filename);
  assert(dvi_filename);

  translate_origin = translate;

  page_ranges = NULL;
  num_page_ranges = 0;
  max_page_ranges = 0;

  dvi_reset_global_state();
  mps_reset_global_state();
  tfm_reset_global_state();
  vf_reset_global_state();
  pdf_dev_reset_global_state();
  pdf_obj_reset_global_state();
  pdf_font_reset_unique_tag_state();

  if (quiet) {
    shut_up(2);
  } else {
    dpx_conf.verbose_level = verbose;
  }

  pdf_font_set_deterministic_unique_tags(deterministic_tags ? 1 : 0);

  pdf_init_fontmaps(); /* This must come before parsing options... */

  /* We used to read the config file here. It synthesized command-line
   * arguments, so we emulate the default TeXLive config file by copying those
   * code bits. */

  pdf_set_version (5);
  select_paper(dpx_config->paperspec);
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
    select_pages(pagespec);
  }

  /*kpse_init_prog("", font_dpi, NULL, NULL);
    kpse_set_program_enabled(kpse_pk_format, true, kpse_src_texmf_cnf);*/
  pdf_font_set_dpi(font_dpi);
  dpx_delete_old_cache(image_cache_life);

  {
    /* Dependency between DVI and PDF side is rather complicated... */
    dvi2pts = dvi_init(dvi_filename, mag);
    if (dvi2pts == 0.0)
      _tt_abort("dvi_init() failed!");

    creator = dvi_comment(); /* Set PDF Creator entry */
    dvi_scan_specials(0,
                      &paper_width, &paper_height,
                      &x_offset, &y_offset, &landscape_mode,
                      &pdf_version_major, &pdf_version_minor,
                      &do_encryption, &key_bits, &permission, oplain, uplain,
                      &has_id, id1, id2);
  }

  /* Encryption and Other Settings */
  {
    memset(&settings.encrypt, 0, sizeof(struct pdf_enc_setting));
    settings.enable_encrypt = do_encryption;
    settings.encrypt.use_aes          = 1;
    settings.encrypt.encrypt_metadata = 1;
    settings.encrypt.key_size   = key_bits;
    settings.encrypt.permission = permission;
    settings.encrypt.uplain     = uplain;
    settings.encrypt.oplain     = oplain;
  }

  if (opt_flags & OPT_PDFOBJ_NO_OBJSTM) {
    settings.object.enable_objstm = 0;
  } else {
    settings.object.enable_objstm = 1;
  }
  if (opt_flags & OPT_PDFOBJ_NO_PREDICTOR) {
    settings.object.enable_predictor = 0;
  } else {
    settings.object.enable_predictor = 1;
  }

  /* Set default paper size here so that all page's can inherite it.
   * annot_grow:    Margin of annotation.
   * bookmark_open: Miximal depth of open bookmarks.
   */
  if (landscape_mode) {
    SWAP(paper_width, paper_height);
  }
  settings.media_width        = paper_width;
  settings.media_height       = paper_height;
  settings.annot_grow_amount  = annot_grow;
  settings.outline_open_depth = bookmark_open;
  settings.check_gotos        = !(opt_flags & OPT_PDFDOC_NO_DEST_REMOVE);

  settings.device.dvi2pts     = dvi2pts;
  settings.device.precision   = pdfdecimaldigits;
  settings.device.ignore_colors = ignore_colors;

  set_distiller_template(filter_template);
  /* This must come before pdf_open_document where initialization
   * of pdf_enc takes place.
   */
  pdf_init_device(dvi2pts, pdfdecimaldigits, ignore_colors);
  {
    int version = pdf_version_major * 10 + pdf_version_minor;
    pdf_set_version(version);
  }
  pdf_set_compression(compress ? compression_level : 0);
  if (enable_thumbnail)
    pdf_doc_enable_manual_thumbnails();

  if (!has_id) {
    const char *producer = "xdvipdfmx-0.1, Copyright 2002-2015 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata";
    compute_id_string(id1, producer, dvi_filename, pdf_filename);
    memcpy(id2, id1, 16);
  }

  /* Initialize PDF document creation routine. */
  pdf_open_document(pdf_filename, creator, id1, id2, settings);

  if (opt_flags & OPT_CIDFONT_FIXEDPITCH)
    CIDFont_set_flags(CIDFONT_FORCE_FIXEDPITCH);
  /* Please move this to spc_init_specials(). */
  if (opt_flags & OPT_TPIC_TRANSPARENT_FILL)
    tpic_set_fill_mode(1);
  if (translate_origin)
    mps_set_translate_origin(1);

  do_dvi_pages();

  pdf_close_document();

  pdf_close_fontmaps(); /* pdf_font may depend on fontmap. */

  dvi_close();

  dpx_message("\n");
  free(page_ranges);

  return 0;
}
