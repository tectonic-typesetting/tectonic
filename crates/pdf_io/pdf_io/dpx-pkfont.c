/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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

#include "dpx-pkfont.h"

#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h" /* pdf_rect */
#include "dpx-pdfencoding.h"
#include "dpx-pdffont.h"
#include "dpx-pdfobj.h"
#include "dpx-system.h"

#define ENABLE_GLYPHENC  1

#ifndef PKFONT_DPI_DEFAULT
#define PKFONT_DPI_DEFAULT 600u
#endif

static unsigned int base_dpi = PKFONT_DPI_DEFAULT;

void
PKFont_set_dpi (int dpi)
{
  if (dpi <= 0)
    _tt_abort("Invalid DPI: %d\n", dpi);
  base_dpi = dpi;
}


/* (Only) This requires TFM to get design size... */
#include "dpx-tfm.h"

static unsigned int
truedpi (const char *ident, double point_size, unsigned int bdpi)
{
  unsigned int dpi = bdpi;
  double    design_size;
  int       tfm_id;

  tfm_id = tfm_open(ident, 0);
  if (tfm_id < 0)
    return  dpi;

  design_size = tfm_get_design_size(tfm_id);
  if (design_size <= 0.0)
    dpx_warning("DESGIN_SIZE <= 0.0? (TFM=\"%s\")", ident);
  else {
    dpi  = (unsigned) ROUND(base_dpi * point_size / design_size, 1.0);
  }

  return  dpi;
}

static FILE *
dpx_open_pk_font_at (const char *ident, unsigned int dpi)
{
  /*kpse_glyph_file_type kpse_file_info;*/
  /*char * fqpn = kpse_find_glyph(ident, dpi, kpse_pk_format, &kpse_file_info);*/
  dpx_warning("Tectonic unable to generate PK font \"%s\" (dpi %u) on-the-fly", ident, dpi);
  return NULL;
}


int
pdf_font_open_pkfont (pdf_font *font)
{
  char     *ident;
  double    point_size;
  int       encoding_id;
  unsigned int dpi;
  FILE     *fp;

  ident       = pdf_font_get_ident(font);
  point_size  = pdf_font_get_param(font, PDF_FONT_PARAM_POINT_SIZE);
  encoding_id = pdf_font_get_encoding(font);

  if (!ident || point_size <= 0.0)
    return  -1;

  dpi = truedpi(ident, point_size, base_dpi);
  fp  = dpx_open_pk_font_at(ident, dpi);
  if (!fp)
    return  -1;
  fclose(fp);

  /* Type 3 fonts doesn't have FontName.
   * FontFamily is recommended for PDF 1.5.
   */
  pdf_font_set_fontname(font, ident);

  if (encoding_id >= 0) {
    pdf_encoding_used_by_type3(encoding_id);
    dpx_warning("PK font is found for font \"%s\" but non built-in encoding \"%s\" is specified.",
         ident, pdf_encoding_get_name(encoding_id));
#if  ENABLE_GLYPHENC
    dpx_warning(">> Assuming this is for glyph name assignment.");
#else
    dpx_warning(">> I can't reencode PK font. (not enough information available)");
    dpx_warning(">> Maybe you need to install pfb/opentype/truetype font.");
#endif
  }

  return  0;
}


/* We are using Mask Image. Fill black is bit clear.
 * Optimizing those codes doesn't improve things.
 */
static uint32_t
fill_black_run (unsigned char *dp, uint32_t left, uint32_t run_count)
{
  static const unsigned char mask[8] = {
    127u, 191u, 223u, 239u, 247u, 251u, 253u, 254u
  };
  uint32_t  right = left + run_count - 1;
  for ( ; left <= right; left++) {
    dp[left / 8] &= mask[left % 8];
  }
  return  run_count;
}

/* Just skip bits. See decode_packed() */
static uint32_t
fill_white_run (uint32_t run_count)
{
  return  run_count;
}

static uint32_t
pk_packed_num (uint32_t *np, int dyn_f, unsigned char *dp, uint32_t pl)
{
  uint32_t nmbr = 0, i = *np;
  int      nyb, j;
#define get_nyb() ((i % 2) ? dp[i/2] & 0x0f : (dp[i/2] >> 4) & 0x0f)

  if (i / 2 == pl) {
    dpx_warning("EOD reached while unpacking pk_packed_num.");
    return  0;
  }
  nyb = get_nyb(); i++;
  if (nyb == 0) {
    j = 0;
    do {
      if (i / 2 == pl) {
        dpx_warning("EOD reached while unpacking pk_packed_num.");
        break;
      }
      nyb = get_nyb(); i++;
      j++;
    } while (nyb == 0);
    nmbr = nyb;
    while (j-- > 0) {
      if (i / 2 == pl) {
        dpx_warning("EOD reached while unpacking pk_packed_num.");
        break;
      }
      nyb  = get_nyb(); i++;
      nmbr = nmbr * 16 + nyb;
    }
    nmbr += (13 - dyn_f) * 16 + dyn_f - 15;
  } else if (nyb <= dyn_f) {
    nmbr = nyb;
  } else if (nyb < 14) {
    if (i / 2 == pl) {
      dpx_warning("EOD reached while unpacking pk_packed_num.");
      return  0;
    }
    nmbr = (nyb - dyn_f - 1) * 16 + get_nyb() + dyn_f + 1;
    i++;
  }

  *np = i;
  return  nmbr;
}


static void
send_out (unsigned char *rowptr, uint32_t rowbytes, pdf_obj *stream)
{
  pdf_add_stream(stream, (void *)rowptr, rowbytes);
}

static int
pk_decode_packed (pdf_obj *stream, uint32_t wd, uint32_t ht,
                  int dyn_f, int run_color, unsigned char *dp, uint32_t pl)
{
  unsigned char  *rowptr;
  uint32_t        rowbytes;
  uint32_t        i, np = 0;
  uint32_t        run_count = 0, repeat_count = 0;

  rowbytes = (wd + 7) / 8;
  rowptr   = NEW(rowbytes, unsigned char);
  /* repeat count is applied to the *current* row.
   * "run" can span across rows.
   * If there are non-zero repeat count and if run
   * spans across row, first repeat and then continue.
   */
  for (np = 0, i = 0; i < ht; i++) {
    uint32_t rowbits_left, nbits;

    repeat_count = 0;
    memset(rowptr, 0xff, rowbytes); /* 1 is white */
    rowbits_left = wd;
    /* Fill run left over from previous row */
    if (run_count > 0) {
      nbits = MIN(rowbits_left, run_count);
      switch (run_color) {
      case  0:
        rowbits_left -= fill_black_run(rowptr, 0, nbits);
        break;
      case  1:
        rowbits_left -= fill_white_run(nbits);
        break;
      }
      run_count -= nbits;
    }

    /* Read nybbles until we have a full row */
    while (np / 2 < pl && rowbits_left > 0) {
      int  nyb;

      nyb = (np % 2) ? dp[np/2] & 0x0f : (dp[np/2] >> 4) & 0x0f;
      if (nyb == 14) { /* packed number "repeat_count" follows */
        if (repeat_count != 0)
          dpx_warning("Second repeat count for this row!");
        np++; /* Consume this nybble */
        repeat_count = pk_packed_num(&np, dyn_f, dp, pl);
      } else if (nyb == 15) {
        if (repeat_count != 0)
          dpx_warning("Second repeat count for this row!");
        np++; /* Consume this nybble */
        repeat_count = 1;
      } else { /* run_count */
        /* Interprete current nybble as packed number */
        run_count = pk_packed_num(&np, dyn_f, dp, pl);
        nbits = MIN(rowbits_left, run_count);
        run_color  = !run_color;
        run_count -= nbits;
        switch (run_color) {
        case  0:
          rowbits_left -= fill_black_run(rowptr, wd - rowbits_left, nbits);
          break;
        case  1:
          rowbits_left -= fill_white_run(nbits);
          break;
        }
      }
    }
    /* We got bitmap row data. */
    send_out(rowptr, rowbytes, stream);
    for ( ; i < ht && repeat_count > 0; repeat_count--, i++)
      send_out(rowptr, rowbytes, stream);
    }
  free(rowptr);

  return  0;
}

static int
pk_decode_bitmap (pdf_obj *stream, uint32_t wd, uint32_t ht,
                  int dyn_f, int run_color, unsigned char *dp, uint32_t pl)
{
  unsigned char  *rowptr, c;
  uint32_t        i, j, rowbytes;
  static const unsigned char mask[8] = {
    0x80u, 0x40u, 0x20u, 0x10u, 0x08u, 0x04u, 0x02u, 0x01u
  };

  assert( dyn_f == 14 );
  if (run_color != 0) {
    dpx_warning("run_color != 0 for bitmap pk data?");
  } else if (pl < (wd * ht + 7) / 8) {
    dpx_warning("Insufficient bitmap pk data. %dbytes expected but only %dbytes read.",
         (wd * ht + 7) / 8, pl);
    return  -1;
  }

  rowbytes = (wd + 7) / 8;
  rowptr   = NEW(rowbytes, unsigned char);
  memset(rowptr, 0, rowbytes);
  /* Flip. PK bitmap is not byte aligned for each rows. */
  for (i = 0, j = 0; i < ht * wd; i++) {
    c = dp[i / 8] & mask[i % 8];
    if (c == 0)
      rowptr[j / 8] |= mask[i % 8]; /* flip bit */
    j++;
    if (j == wd) {
      send_out(rowptr, rowbytes, stream);
      memset(rowptr, 0, rowbytes);
      j = 0;
    }
  }

  return  0;
}


static void
do_preamble (FILE *fp)
{
  /* Check for id byte */
  if (fgetc(fp) == 89) {
    /* Skip comment */
    skip_bytes(get_unsigned_byte(fp), fp);
    /* Skip other header info.  It's normally used for verifying this
       is the file wethink it is */
    skip_bytes(16, fp);
  } else {
    _tt_abort("embed_pk_font: PK ID byte is incorrect.  Are you sure this is a PK file?");
  }
  return;
}

struct pk_header_
{
  uint32_t pkt_len;
  int32_t  chrcode;
  int32_t  wd, dx, dy;
  uint32_t bm_wd, bm_ht;
  int32_t  bm_hoff, bm_voff;
  int      dyn_f, run_color;
};

static int
read_pk_char_header (struct pk_header_ *h, unsigned char opcode, FILE *fp)
{
  assert(h);

  if ((opcode & 4) == 0) { /* short */
    h->pkt_len = (opcode & 3) << 8 | get_unsigned_byte(fp);
    h->chrcode = get_unsigned_byte(fp);
    h->wd = get_unsigned_triple(fp);     /* TFM width */
    h->dx = get_unsigned_byte(fp) << 16; /* horizontal escapement */
    h->dy = 0;
    h->bm_wd    = get_unsigned_byte(fp);
    h->bm_ht    = get_unsigned_byte(fp);
    h->bm_hoff  = get_signed_byte(fp);
    h->bm_voff  = get_signed_byte(fp);
    h->pkt_len -= 8;
  } else if ((opcode & 7) == 7) { /* long */
    h->pkt_len = get_positive_quad(fp, "PK", "pkt_len");
    h->chrcode = get_signed_quad(fp);
    h->wd = get_signed_quad(fp);
    h->dx = get_signed_quad(fp); /* 16.16 fixed point number in pixels */
    h->dy = get_signed_quad(fp);
    h->bm_wd    = get_positive_quad(fp, "PK", "bm_wd");
    h->bm_ht    = get_positive_quad(fp, "PK", "bm_ht");
    h->bm_hoff  = get_signed_quad(fp);
    h->bm_voff  = get_signed_quad(fp);
    h->pkt_len -= 28;
  } else { /* extended short */
    h->pkt_len = (opcode & 3) << 16 | get_unsigned_pair(fp);
    h->chrcode = get_unsigned_byte(fp);
    h->wd = get_unsigned_triple(fp);
    h->dx = get_unsigned_pair(fp) << 16;
    h->dy = 0;
    h->bm_wd    = get_unsigned_pair(fp);
    h->bm_ht    = get_unsigned_pair(fp);
    h->bm_hoff  = get_signed_pair(fp);
    h->bm_voff  = get_signed_pair(fp);
    h->pkt_len -= 13;
  }

  h->dyn_f     = opcode / 16;
  h->run_color = (opcode & 8) ? 1 : 0;

  if ((uint32_t)h->chrcode > 0xff)
  {
    dpx_warning("Unable to handle long characters in PK files: code=0x%04x", h->chrcode);
    return  -1;
  }

  return  0;
}

/* CCITT Group 4 filter may reduce file size. */
static pdf_obj *
create_pk_CharProc_stream (struct pk_header_ *pkh,
                           double             chrwid,
                           unsigned char     *pkt_ptr, uint32_t pkt_len)
{
  pdf_obj  *stream; /* charproc */
  int32_t   llx, lly, urx, ury;
  int       len;

  llx = -pkh->bm_hoff;
  lly =  pkh->bm_voff - pkh->bm_ht;
  urx =  pkh->bm_wd - pkh->bm_hoff;
  ury =  pkh->bm_voff;

  stream = pdf_new_stream(STREAM_COMPRESS);
  /*
   * The following line is a "metric" for the PDF reader:
   *
   * PDF Reference Reference, 4th ed., p.385.
   *
   * The wx (first operand of d1) must be consistent with the corresponding
   * width in the font's Widths array. The format string of sprint() must be
   * consistent with write_number() in pdfobj.c.
   */
  len = pdf_sprint_number(work_buffer, chrwid);
  len += sprintf (work_buffer + len, " 0 %d %d %d %d d1\n", llx, lly, urx, ury);
  pdf_add_stream(stream, work_buffer, len);
  /*
   * Acrobat dislike transformation [0 0 0 0 dx dy].
   * PDF Reference, 4th ed., p.147, says,
   *
   *   Use of a noninvertible matrix when painting graphics objects can result in
   *   unpredictable behavior.
   *
   * but it does not forbid use of such transformation.
   */
  if (pkh->bm_wd != 0 && pkh->bm_ht != 0 && pkt_len > 0) {
    /* Scale and translate origin to lower left corner for raster data */
    len = sprintf (work_buffer, "q\n%u 0 0 %u %d %d cm\n", pkh->bm_wd, pkh->bm_ht, llx, lly);
    pdf_add_stream(stream, work_buffer, len);
    len = sprintf (work_buffer, "BI\n/W %u\n/H %u\n/IM true\n/BPC 1\nID ", pkh->bm_wd, pkh->bm_ht);
    pdf_add_stream(stream, work_buffer, len);
    /* Add bitmap data */
    if (pkh->dyn_f == 14) /* bitmap */
              pk_decode_bitmap(stream,
                               pkh->bm_wd, pkh->bm_ht,
                               pkh->dyn_f, pkh->run_color,
                               pkt_ptr,    pkt_len);
    else
              pk_decode_packed(stream,
                               pkh->bm_wd, pkh->bm_ht,
                               pkh->dyn_f, pkh->run_color,
                               pkt_ptr,    pkt_len);
    len = sprintf (work_buffer, "\nEI\nQ");
    pdf_add_stream(stream, work_buffer, len);
  } /* Otherwise we embed an empty stream :-( */

  return  stream;
}

#define PK_XXX1  240
#define PK_XXX2  241
#define PK_XXX3  242
#define PK_XXX4  243
#define PK_YYY   244
#define PK_POST  245
#define PK_NO_OP 246
#define PK_PRE   247

#define pk_char2name(b,c) sprintf((b), "x%02X", (unsigned char)(c))
int
pdf_font_load_pkfont (pdf_font *font)
{
  pdf_obj  *fontdict;
  char     *usedchars;
  char     *ident;
  unsigned int dpi;
  FILE     *fp;
  double    point_size, pix2charu;
  int       opcode, code, firstchar, lastchar, prev;
  pdf_obj  *charprocs, *procset, *encoding, *tmp_array;
  double    widths[256];
  pdf_rect  bbox;
  char      charavail[256];
#if  ENABLE_GLYPHENC
  int       encoding_id;
  char    **enc_vec;
#endif /* ENABLE_GLYPHENC */
  int       error = 0;

  if (!pdf_font_is_in_use(font)) {
    return 0;
  }

  ident       = pdf_font_get_ident(font);
  point_size  = pdf_font_get_param(font, PDF_FONT_PARAM_POINT_SIZE);
  usedchars   = pdf_font_get_usedchars(font);
#if  ENABLE_GLYPHENC
  encoding_id = pdf_font_get_encoding(font);
  if (encoding_id < 0)
    enc_vec = NULL;
  else {
    enc_vec = pdf_encoding_get_encoding(encoding_id);
  }
#endif /* ENABLE_GLYPHENC */

  assert(ident && usedchars && point_size > 0.0);

  dpi  = truedpi(ident, point_size, base_dpi);
  fp   = dpx_open_pk_font_at(ident, dpi);
  if (!fp) {
    _tt_abort("Could not find/open PK font file: %s (at %udpi)", ident, dpi);
  }

  memset(charavail, 0, 256);
  charprocs  = pdf_new_dict();
  /* Include bitmap as 72dpi image:
   * There seems to be problems in "scaled" bitmap glyph
   * rendering in several viewers.
   */
  pix2charu  = 72. * 1000. / ((double) base_dpi) / point_size;
  bbox.llx = bbox.lly =  HUGE_VAL;
  bbox.urx = bbox.ury = -HUGE_VAL;
  while ((opcode = fgetc(fp)) >= 0 && opcode != PK_POST) {
    if (opcode < 240) {
      struct pk_header_  pkh;

      error = read_pk_char_header(&pkh, opcode, fp);
      if (error)
        _tt_abort("Error in reading PK character header.");
      else if (charavail[pkh.chrcode & 0xff])
        dpx_warning("More than two bitmap image for single glyph?: font=\"%s\" code=0x%02x",
             ident, pkh.chrcode);

      if (!usedchars[pkh.chrcode & 0xff])
        skip_bytes(pkh.pkt_len, fp);
      else {
        char          *charname;
        pdf_obj       *charproc;
        unsigned char *pkt_ptr;
        size_t         bytesread;
        double         charwidth;

        /* Charwidth in PDF units */
        charwidth = ROUND(1000.0 * pkh.wd / (((double) (1<<20))*pix2charu), 0.1);
        widths[pkh.chrcode & 0xff] = charwidth;

        /* Update font BBox info */
        bbox.llx = MIN(bbox.llx, -pkh.bm_hoff);
        bbox.lly = MIN(bbox.lly,  (double)pkh.bm_voff - (double)pkh.bm_ht);
        bbox.urx = MAX(bbox.urx,  (double)pkh.bm_wd - (double)pkh.bm_hoff);
        bbox.ury = MAX(bbox.ury,  pkh.bm_voff);

        pkt_ptr = NEW(pkh.pkt_len, unsigned char);
        if ((bytesread = fread(pkt_ptr, 1, pkh.pkt_len, fp))!= pkh.pkt_len) {
          _tt_abort("Only %"PRIuZ" bytes PK packet read. (expected %d bytes)",
                bytesread, pkh.pkt_len);
        }
        charproc = create_pk_CharProc_stream(&pkh, charwidth, pkt_ptr, bytesread);
        free(pkt_ptr);
        if (!charproc)
          _tt_abort("Unpacking PK character data failed.");
#if  ENABLE_GLYPHENC
        if (encoding_id >= 0 && enc_vec) {
          charname = (char *) enc_vec[pkh.chrcode & 0xff];
          if (!charname) {
            dpx_warning("\".notdef\" glyph used in font (code=0x%02x): %s", pkh.chrcode, ident);
            charname = work_buffer;
            pk_char2name(charname, pkh.chrcode);
          }
        }
        else
#endif /* ENABLE_GLYPHENC */
        {
          charname = work_buffer;
          pk_char2name(charname, pkh.chrcode);
        }

        pdf_add_dict(charprocs, pdf_new_name(charname), pdf_ref_obj(charproc)); /* _FIXME_ */
        pdf_release_obj(charproc);
      }
      charavail[pkh.chrcode & 0xff] = 1;
    } else { /* A command byte */
      switch (opcode) {
      case PK_NO_OP: break;
      case PK_XXX1: case PK_XXX2: case PK_XXX3: case PK_XXX4:
      {
        int32_t len = get_unsigned_num(fp, opcode-PK_XXX1);
        if (len < 0)
          dpx_warning("PK: Special with %d bytes???", len);
        else
          skip_bytes(len, fp);
        break;
      }
      case PK_YYY:  skip_bytes(4, fp);  break;
      case PK_PRE:  do_preamble(fp); break;
      }
    }
  }
  fclose(fp);

  /* Check if we really got all glyphs needed. */
  for (code = 0; code < 256; code++) {
    if (usedchars[code] && !charavail[code])
      dpx_warning("Missing glyph code=0x%02x in PK font \"%s\".", code, ident);
  }

  /* Now actually fill fontdict. */
  fontdict = pdf_font_get_resource(font);

  pdf_add_dict(fontdict,
               pdf_new_name("CharProcs"), pdf_ref_obj(charprocs));
  pdf_release_obj(charprocs);

  /*
   * Resources:
   *
   *  PDF Reference 4th ed. describes it as "Optional but strongly recommended".
   *  There are no reason to put it in our case, but we will put this.
   *  We do not care about compatibility with Acrobat 2.x. (See implementation
   *  note 47, Appendix H of PDF Ref., 4th ed.).
   */
  procset   = pdf_new_dict();
  tmp_array = pdf_new_array();
  pdf_add_array(tmp_array, pdf_new_name("PDF"));
  pdf_add_array(tmp_array, pdf_new_name("ImageB"));
  pdf_add_dict(procset,
               pdf_new_name("ProcSet"), tmp_array);
  pdf_add_dict(fontdict,
               pdf_new_name("Resources"), procset);

  /* Encoding */
  tmp_array = pdf_new_array();
  prev = -2; firstchar = 255; lastchar = 0;
  for (code = 0; code < 256; code++) {
    char  *charname;
    if (usedchars[code]) {
      if (code < firstchar) firstchar = code;
      if (code > lastchar)  lastchar  = code;
      if (code != prev + 1)
        pdf_add_array(tmp_array, pdf_new_number(code));

#if  ENABLE_GLYPHENC
      if (encoding_id >= 0 && enc_vec) {
        charname = (char *) enc_vec[(unsigned char) code];
        if (!charname) {
          charname = work_buffer;
          pk_char2name(charname, code);
        }
      }
      else
#endif /* ENABLE_GLYPHENC */
      {
        charname = work_buffer;
        pk_char2name(charname, code);
      }
      pdf_add_array(tmp_array, pdf_new_name(charname));
      prev = code;
    }
  }
  if (firstchar > lastchar) {
    pdf_release_obj(tmp_array);
    _tt_abort("Unexpected error: firstchar > lastchar (%d %d)",
          firstchar, lastchar);
  }
#if  ENABLE_GLYPHENC
  if (encoding_id < 0 || !enc_vec) {
#else
  if (1) {
#endif /* ENABLE_GLYPHENC */
    encoding  = pdf_new_dict();
    pdf_add_dict(encoding,
                 pdf_new_name("Type"), pdf_new_name("Encoding"));
    pdf_add_dict(encoding,
                 pdf_new_name("Differences"), tmp_array);
    pdf_add_dict(fontdict,
                 pdf_new_name("Encoding"),    pdf_ref_obj(encoding));
    pdf_release_obj(encoding);
  } else
    pdf_release_obj(tmp_array);

  /* FontBBox: Accurate value is important.
   */
  tmp_array = pdf_new_array();
  pdf_add_array(tmp_array, pdf_new_number(bbox.llx));
  pdf_add_array(tmp_array, pdf_new_number(bbox.lly));
  pdf_add_array(tmp_array, pdf_new_number(bbox.urx));
  pdf_add_array(tmp_array, pdf_new_number(bbox.ury));
  pdf_add_dict (fontdict , pdf_new_name("FontBBox"), tmp_array);

  /* Widths:
   *  Indirect reference preffered. (See PDF Reference)
   */
  tmp_array = pdf_new_array();
  for (code = firstchar; code <= lastchar; code++) {
    if (usedchars[code])
      pdf_add_array(tmp_array, pdf_new_number(widths[code]));
    else {
      pdf_add_array(tmp_array, pdf_new_number(0));
    }
  }
  pdf_add_dict(fontdict,
               pdf_new_name("Widths"), pdf_ref_obj(tmp_array));
  pdf_release_obj(tmp_array);

  /* FontMatrix */
  tmp_array = pdf_new_array();
  pdf_add_array(tmp_array, pdf_new_number(0.001 * pix2charu));
  pdf_add_array(tmp_array, pdf_new_number(0.0));
  pdf_add_array(tmp_array, pdf_new_number(0.0));
  pdf_add_array(tmp_array, pdf_new_number(0.001 * pix2charu));
  pdf_add_array(tmp_array, pdf_new_number(0.0));
  pdf_add_array(tmp_array, pdf_new_number(0.0));
  pdf_add_dict (fontdict , pdf_new_name("FontMatrix"), tmp_array);


  pdf_add_dict(fontdict,
               pdf_new_name("FirstChar"), pdf_new_number(firstchar));
  pdf_add_dict(fontdict,
               pdf_new_name("LastChar"),  pdf_new_number(lastchar));

  return  0;
}
