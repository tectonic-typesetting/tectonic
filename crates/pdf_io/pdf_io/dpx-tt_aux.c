/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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

#include "dpx-tt_aux.h"

#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-numbers.h"
#include "dpx-pdfobj.h"
#include "dpx-sfnt.h"
#include "dpx-tt_post.h"
#include "dpx-tt_table.h"

ULONG ttc_read_offset (sfnt *sfont, int ttc_idx)
{
  ULONG offset = 0, num_dirs = 0;

  if (sfont == NULL || sfont->handle == NULL)
    _tt_abort("file not opened");

  if (sfont->type != SFNT_TYPE_TTC)
    _tt_abort("ttc_read_offset(): invalid font type");

  sfnt_seek_set (sfont, 4); /* skip version tag */

  /* version = */ sfnt_get_ulong(sfont);
  num_dirs = sfnt_get_ulong(sfont);
  if (ttc_idx < 0 || ttc_idx > num_dirs - 1)
    _tt_abort("Invalid TTC index number");

  sfnt_seek_set (sfont, 12 + ttc_idx * 4);
  offset = sfnt_get_ulong (sfont);

  return offset;
}

/*
  Build FontDescriptor (except FontName) from TrueType tables:

   Most information found in FontDescriptor is used only when automatic
   font substitution is needed. (in the case of missing/broken font data)
   Some PDF viewers may ignore embedded TrueType glyph data. Especially,
   any embedded TrueType data for CID-keyed (CIDFontType 2) font is ignored
   by PDF viewers that only support PDF versions 1.2 or earlier.

   We use those tables to obtain various values of FontDescriptor.

   head: required

    xMin, xMax, yMin, yMax - FontBBox
    unitsPerEm - conversion to PDF unit (points). see PDFUNIT bellow.
    The head table must exist in any TrueType font.

   hhea: required

    When the OS/2 table (Windows and OS/2 only) is not available,
    Ascender and Descender values can be used to estimate Ascent
    and Descent. The hhea table is required for all TrueType fonts.
    MaxWidth can be obtained from this table.

   OS/2: required for Windows and OS/2 TrueType, and OpenType

    fsType     - liscensing information
    sCapHeight - CapHeight (version 2 only)

     The sCapHeight is only available in newer TrueType fonts which has
     version 2 OS/2 table and generally not available. Instead, we can
     use height of uppercase letter `H'. But we don't use it, we simply
     use Ascent value.

    sTypoAscender, sTypoDescender - Ascent and Descent
    usWeightClass - roughly estimate StemV.

     Estimation is based on the following expression:

      stemv = (os2->usWeightClass/65)*(os2->usWeightClass/65)+50

     . I've found this expression in some Adobe document (lost). We use
     this expression also, otherwise, we must analyze glyph data.

    xAvgCharWidth - AvgWidth (optional)
    sFamilyClass - Flags
    sFamilyClass and panose - Panose in Style dictionary (optional)

   post: required

    italicAngle - ItalicAngle

*/


#ifndef PDFUNIT
#define PDFUNIT(v) (ROUND((1000.0*(v))/(head->unitsPerEm),1))
#endif

/* Flags: should not come here */
#define FIXEDWIDTH (1 << 0)  /* Fixed-width font */
#define SERIF      (1 << 1)  /* Serif font */
#define SYMBOLIC   (1 << 2)  /* Symbolic font */
#define SCRIPT     (1 << 3)  /* Script font */
#define STANDARD   (1 << 5)  /* Uses the Adobe Standard Character Set */
#define ITALIC     (1 << 6)  /* Italic */
#define ALLCAP     (1 << 16) /* All-cap font */
#define SMALLCAP   (1 << 17) /* Small-cap font */
#define FORCEBOLD  (1 << 18) /* Force bold at small text sizes */
pdf_obj *tt_get_fontdesc (sfnt *sfont, int *embed, int stemv, int type, const char* fontname)
{
  pdf_obj *descriptor = NULL;
  pdf_obj *bbox = NULL;
  int flag = SYMBOLIC;
  /* TrueType tables */
  struct tt_head_table *head;
  struct tt_os2__table *os2;
  struct tt_post_table *post;

  if (!sfont) {
    _tt_abort("font file not opened");
  }

  os2  = tt_read_os2__table(sfont);
  head = tt_read_head_table(sfont);
  post = tt_read_post_table(sfont);
  if (!post) {
    free(os2);
    free(head);
    return NULL;
  }

  descriptor = pdf_new_dict();
  pdf_add_dict (descriptor,
                pdf_new_name ("Type"),
                pdf_new_name ("FontDescriptor"));

  if (*embed && os2) {
    /*
      License:

       "Preview & Print embedding" (0x004) requires the document containing
       Preview & Print font to be opened in read-only mode. However, licensing
       information are lost when fonts are embedded in PDF document and
       the only way to make the PDF document "read-only" is to encrypt it.
       But we have no support for encryption yet. We do not embed any fonts
       with "Preview & Print embedding" setting.

       2001/11/22: Changed to allow `Preview & Print' only fonts embedding

       2006/04/19: Added support for always_embed option
    */
    if (os2->fsType == 0x0000 || (os2->fsType & 0x0008)) {
      /* the least restrictive license granted takes precedence. */
      *embed = 1;
    } else if (os2->fsType & 0x0004) {
      if (dpx_conf.verbose_level > 0)
        dpx_warning("Font \"%s\" permits \"Preview & Print\" embedding only **\n", fontname);
      *embed = 1;
    } else {
      if (dpx_conf.ignore_font_license) {
        if (dpx_conf.verbose_level > 0)
          dpx_warning("Font \"%s\" may be subject to embedding restrictions **\n", fontname);
        *embed = 1;
      }
      else {
        if (dpx_conf.verbose_level > 0)
          dpx_warning("Embedding of font \"%s\" disabled due to license restrictions", fontname);
        *embed = 0;
      }
    }
  }

  if (os2) {
    pdf_add_dict (descriptor,
                  pdf_new_name ("Ascent"),
                  pdf_new_number (PDFUNIT(os2->sTypoAscender)));
    pdf_add_dict (descriptor,
                  pdf_new_name ("Descent"),
                  pdf_new_number (PDFUNIT(os2->sTypoDescender)));
    if (stemv < 0) /* if not given by the option '-v' */
      stemv = (os2->usWeightClass/65.)*(os2->usWeightClass/65.)+50;
    pdf_add_dict (descriptor,
                  pdf_new_name ("StemV"),
                  pdf_new_number (stemv));
    if (os2->version == 0x0002) {
      pdf_add_dict (descriptor,
                    pdf_new_name("CapHeight"),
                    pdf_new_number(PDFUNIT(os2->sCapHeight)));
      /* optional */
      pdf_add_dict (descriptor,
                    pdf_new_name("XHeight"),
                    pdf_new_number(PDFUNIT(os2->sxHeight)));
    } else { /* arbitrary */
      pdf_add_dict (descriptor,
                    pdf_new_name("CapHeight"),
                    pdf_new_number(PDFUNIT(os2->sTypoAscender)));
    }
    /* optional */
    if (os2->xAvgCharWidth != 0) {
      pdf_add_dict (descriptor,
                    pdf_new_name ("AvgWidth"),
                    pdf_new_number (PDFUNIT(os2->xAvgCharWidth)));
    }
  }

  /* BoundingBox (array) */
  bbox = pdf_new_array ();
  pdf_add_array (bbox, pdf_new_number (PDFUNIT(head->xMin)));
  pdf_add_array (bbox, pdf_new_number (PDFUNIT(head->yMin)));
  pdf_add_array (bbox, pdf_new_number (PDFUNIT(head->xMax)));
  pdf_add_array (bbox, pdf_new_number (PDFUNIT(head->yMax)));
  pdf_add_dict (descriptor, pdf_new_name ("FontBBox"), bbox);

  /* post */
  pdf_add_dict (descriptor,
                pdf_new_name ("ItalicAngle"),
                pdf_new_number(fixed(post->italicAngle)));

  /* Flags */
  if (os2) {
    if (os2->fsSelection & (1 << 0))
      flag |= ITALIC;
    if (os2->fsSelection & (1 << 5))
      flag |= FORCEBOLD;
    if (((os2->sFamilyClass >> 8) & 0xff) != 8)
      flag |= SERIF;
    if (((os2->sFamilyClass >> 8) & 0xff) == 10)
      flag |= SCRIPT;
    if (post->isFixedPitch)
      flag |= FIXEDWIDTH;
  }

  pdf_add_dict (descriptor,
                pdf_new_name ("Flags"),
                pdf_new_number (flag));

  /* insert panose if you want */
  if (type == 0 && os2) { /* cid-keyed font - add panose */
    pdf_obj *styledict = NULL;
    unsigned char panose[12];

    panose[0] = os2->sFamilyClass >> 8;
    panose[1] = os2->sFamilyClass & 0xff;
    memcpy(panose+2, os2->panose, 10);

    styledict = pdf_new_dict ();
    pdf_add_dict (styledict, pdf_new_name ("Panose"),
                  pdf_new_string (panose, 12));
    pdf_add_dict (descriptor, pdf_new_name ("Style"), styledict);
  }

  free(head);
  free(os2);
  tt_release_post_table(post);

  return descriptor;
}

