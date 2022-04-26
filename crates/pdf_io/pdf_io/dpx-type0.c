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

/*
 * Type0 font support:
 *
 * TODO:
 *
 *  Composite font (multiple descendants) - not supported in PDF
 */

#include "dpx-type0.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-cid.h"
#include "dpx-cidtype0.h"
#include "dpx-cmap.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"

static pdf_obj *pdf_read_ToUnicode_file (const char *cmap_name);

/* PLEASE FIX THIS */
#include "dpx-tt_cmap.h"

/* Try to load ToUnicode CMap from file system first, if not found fallback to
 * font CMap reverse lookup.
 * CHANGED: CMap here is not always Unicode to CID mapping. Don't use reverse lookup.
 */
static pdf_obj *
try_load_ToUnicode_file (char *cmap_base)
{
  pdf_obj *tounicode;
  char *cmap_name;

  cmap_name = NEW(strlen(cmap_base)+strlen("-UTF16")+1, char);
  sprintf(cmap_name, "%s-UTF16", cmap_base);
  tounicode = pdf_read_ToUnicode_file(cmap_name);
  if (!tounicode) {
    sprintf(cmap_name, "%s-UCS2", cmap_base);
    tounicode = pdf_read_ToUnicode_file(cmap_name);
  }
  free(cmap_name);

  return tounicode;
}

static void
Type0Font_attach_ToUnicode_stream (pdf_font *font)
{
  pdf_obj    *tounicode;
  pdf_font   *cidfont = pdf_get_font_data(font->type0.descendant);
  CIDSysInfo *csi;
  char       *fontname;

  /*
   * ToUnicode CMap:
   *
   *  ToUnicode CMaps are usually not required for standard character
   *  collections such as Adobe-Japan1. Identity-H is used for UCS
   *  ordering CID-keyed fonts. External resource must be loaded for
   *  others.
   */

  assert(cidfont);

  if (CIDFont_is_ACCFont(cidfont)) {
    /* No need to embed ToUnicode */
    return;
  } else if (CIDFont_is_UCSFont(cidfont)) {
    /*
     * Old version of dvipdfmx mistakenly used Adobe-Identity as Unicode.
     */
    /* ref returned */
    tounicode = pdf_read_ToUnicode_file("Adobe-Identity-UCS2");
    if (!tounicode) { /* This should work */
      tounicode = pdf_new_name("Identity-H");
    }
    pdf_add_dict(font->resource, pdf_new_name("ToUnicode"), tounicode);
    return;
  }

  tounicode = NULL;
  csi       = &cidfont->cid.csi;
  if (cidfont->cid.options.embed) {
    fontname = NEW(strlen(cidfont->fontname)+8, char);
    sprintf(fontname, "%s+%s", cidfont->uniqueID, cidfont->fontname);
  } else {
    fontname = NEW(strlen(cidfont->fontname)+1, char);
    strcpy(fontname, cidfont->fontname);
  }

  switch (cidfont->subtype) {
  case PDF_FONT_FONTTYPE_CIDTYPE2:
    if (!strcmp(csi->registry, "Adobe") && !strcmp(csi->ordering, "Identity")) {
      tounicode = otf_create_ToUnicode_stream(cidfont->ident, cidfont->index,
                                              fontname, font->usedchars);
    } else {
      char *cmap_base = NEW(strlen(csi->registry) + strlen(csi->ordering) + 2, char);
      sprintf(cmap_base, "%s-%s", csi->registry, csi->ordering);
      tounicode = try_load_ToUnicode_file(cmap_base);
      free(cmap_base);
      /* In this case glyphs are re-ordered hance otf_create... won't work */
    }
    break;
  default:
    if (cidfont->flags & CIDFONT_FLAG_TYPE1C) {
      tounicode = otf_create_ToUnicode_stream(cidfont->ident, cidfont->index,
                                              fontname, font->usedchars);
    } else if (cidfont->flags & CIDFONT_FLAG_TYPE1) {
      tounicode = CIDFont_type0_t1create_ToUnicode_stream(cidfont->ident, fontname, font->usedchars);
    } else {
      tounicode = try_load_ToUnicode_file(cidfont->fontname);
      if (!tounicode) {
        tounicode = otf_create_ToUnicode_stream(cidfont->ident, cidfont->index,
                                                fontname, font->usedchars);
      }
    }
  }
  free(fontname);

  if (tounicode) {
    pdf_add_dict(font->resource,
                 pdf_new_name("ToUnicode"), tounicode);
  } else {
    dpx_warning("Failed to load ToUnicode CMap for font \"%s\"", cidfont->filename);
  }

  return;
}

void
pdf_font_load_type0 (pdf_font *font)
{
  if (!font || !font->reference)
    return;

  /* FIXME: Should move to pdffont.c */
  if (!pdf_lookup_dict(font->resource, "ToUnicode")) {
    Type0Font_attach_ToUnicode_stream(font);
  }
}

int
pdf_font_open_type0 (pdf_font *font, int cid_id, int wmode)
{
  pdf_font *cidfont;
  CIDSysInfo *csi;
  char       *fontname = NULL;

  if (cid_id < 0)
    return -1;

  cidfont = pdf_get_font_data(cid_id);

  font->type0.wmode = wmode;
  font->type0.descendant = cid_id;

  /*
   * PostScript Font name:
   *
   *  Type0 font's fontname is usually descendant CID-keyed font's font name
   *  appended by -ENCODING.
   */
  if (cidfont->cid.options.embed) {
    fontname = NEW(strlen(cidfont->fontname)+8, char);
    sprintf(fontname, "%s+%s", cidfont->uniqueID, cidfont->fontname);
  } else {
    fontname = NEW(strlen(cidfont->fontname)+1, char);
    strcpy(fontname, cidfont->fontname);
  }

  if (dpx_conf.verbose_level > 0) {
    dpx_message("(CID:%s)", fontname);
  }

  switch (cidfont->subtype) {
  case PDF_FONT_FONTTYPE_CIDTYPE0:
    font->fontname  = NEW(strlen(fontname)+strlen("Identity-V")+2, char);
    sprintf(font->fontname, "%s-%s", fontname, wmode ? "Identity-V" : "Identity-H");
    font->usedchars = CIDFont_get_usedchars(cidfont);
    font->flags    |= PDF_FONT_FLAG_USEDCHAR_SHARED;
    if (wmode) {
      cidfont->cid.need_vmetrics = 1;
    }
    break;
  case PDF_FONT_FONTTYPE_CIDTYPE2:
    font->fontname = NEW(strlen(fontname)+1, char);
    strcpy(font->fontname, fontname);
    /* Adobe-Identity here means use GID as CID directly. No need to use GSUB for finding
     * vertical glyphs hence separate used_chars for H and V instances are not needed.
     */
    csi = &cidfont->cid.csi;
    if (!csi || (!strcmp(csi->registry, "Adobe") && !strcmp(csi->ordering, "Identity"))) {
      font->usedchars  = CIDFont_get_usedchars(cidfont);
      font->flags     |= PDF_FONT_FLAG_USEDCHAR_SHARED;
    } else {
      font->usedchars  = wmode ? CIDFont_get_usedchars_v(cidfont) : CIDFont_get_usedchars(cidfont);
      font->flags     |= PDF_FONT_FLAG_USEDCHAR_SHARED;
    }
    if (wmode) {
      cidfont->cid.need_vmetrics = 1;
    }
    break;
  }

  free(fontname); /* Tectonic: fix memory leak */
  font->resource = pdf_new_dict();
  pdf_add_dict(font->resource, pdf_new_name ("Type"),    pdf_new_name ("Font"));
  pdf_add_dict(font->resource, pdf_new_name ("Subtype"), pdf_new_name ("Type0"));
  pdf_add_dict(font->resource,
               pdf_new_name("BaseFont"), pdf_new_name(font->fontname));
  pdf_add_dict(font->resource,
               pdf_new_name("Encoding"), pdf_new_name(wmode ? "Identity-V" : "Identity-H"));

  return 0;
}

/******************************** COMPAT ********************************/

#ifndef WITHOUT_COMPAT

#include "dpx-pdfencoding.h"
#include "dpx-pdfresource.h"

static pdf_obj *
create_dummy_CMap (void)
{
  pdf_obj *stream;
  char     buf[32];
  int      i, n;

#define CMAP_PART0 "\
%!PS-Adobe-3.0 Resource-CMap\n\
%%DocumentNeededResources: ProcSet (CIDInit)\n\
%%IncludeResource: ProcSet (CIDInit)\n\
%%BeginResource: CMap (Adobe-Identity-UCS2)\n\
%%Title: (Adobe-Identity-UCS2 Adobe UCS2 0)\n\
%%Version: 1.0\n\
%%Copyright:\n\
%% ---\n\
%%EndComments\n\n\
"
#define CMAP_PART1 "\
/CIDInit /ProcSet findresource begin\n\
\n\
12 dict begin\n\nbegincmap\n\n\
/CIDSystemInfo 3 dict dup begin\n\
  /Registry (Adobe) def\n\
  /Ordering (UCS2) def\n\
  /Supplement 0 def\n\
end def\n\n\
/CMapName /Adobe-Identity-UCS2 def\n\
/CMapVersion 1.0 def\n\
/CMapType 2 def\n\n\
2 begincodespacerange\n\
<0000> <FFFF>\n\
endcodespacerange\n\
"
#define CMAP_PART3 "\
endcmap\n\n\
CMapName currentdict /CMap defineresource pop\n\n\
end\nend\n\n\
%%EndResource\n\
%%EOF\n\
"

  stream = pdf_new_stream(STREAM_COMPRESS);
  pdf_add_stream(stream, CMAP_PART0, strlen(CMAP_PART0));
  pdf_add_stream(stream, CMAP_PART1, strlen(CMAP_PART1));
  pdf_add_stream(stream, "\n100 beginbfrange\n", strlen("\n100 beginbfrange\n"));
  for (i = 0; i < 0x64; i++) {
    n = sprintf(buf,
                "<%02X00> <%02XFF> <%02X00>\n", i, i, i);
    pdf_add_stream(stream, buf, n);
  }
  pdf_add_stream(stream, "endbfrange\n\n", strlen("endbfrange\n\n"));

  pdf_add_stream(stream, "\n100 beginbfrange\n", strlen("\n100 beginbfrange\n"));
  for (i = 0x64; i < 0xc8; i++) {
    n = sprintf(buf,
                "<%02X00> <%02XFF> <%02X00>\n", i, i, i);
    pdf_add_stream(stream, buf, n);
  }
  pdf_add_stream(stream, "endbfrange\n\n", strlen("endbfrange\n\n"));

  pdf_add_stream(stream, "\n48 beginbfrange\n", strlen("\n48 beginbfrange\n"));
  for (i = 0xc8; i <= 0xd7; i++) {
    n = sprintf(buf,
                "<%02X00> <%02XFF> <%02X00>\n", i, i, i);
    pdf_add_stream(stream, buf, n);
  }
  for (i = 0xe0; i <= 0xff; i++) {
    n = sprintf(buf,
                "<%02X00> <%02XFF> <%02X00>\n", i, i, i);
    pdf_add_stream(stream, buf, n);
  }
  pdf_add_stream(stream, "endbfrange\n\n", strlen("endbfrange\n\n"));

  pdf_add_stream(stream, CMAP_PART3, strlen(CMAP_PART3));

  return  stream;
}

static pdf_obj *
pdf_read_ToUnicode_file (const char *cmap_name)
{
  pdf_obj *stream;
  int      res_id = -1;

  assert(cmap_name);

  res_id = pdf_findresource("CMap", cmap_name);
  if (res_id < 0) {
    if (streq_ptr(cmap_name, "Adobe-Identity-UCS2"))
      stream = create_dummy_CMap();
    else {
      stream = pdf_load_ToUnicode_stream(cmap_name);
    }
    if (stream) {
      res_id   = pdf_defineresource("CMap",
                                    cmap_name,
                                    stream, PDF_RES_FLUSH_IMMEDIATE);
    }
  }

  return  (res_id < 0 ? NULL : pdf_get_resource_reference(res_id));
}
#endif /* !WITHOUT_COMPAT */
