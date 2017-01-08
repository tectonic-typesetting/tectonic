/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"
#include "dpxfile.h"

#include "pdfobj.h"
#include "fontmap.h"

#include "cmap.h"
#include "cid.h"

#include "type0.h"


#define TYPE0FONT_DEBUG_STR "Type0"
#define TYPE0FONT_DEBUG     3

static int __verbose = 0;

static pdf_obj *pdf_read_ToUnicode_file (const char *cmap_name);

void
Type0Font_set_verbose(void)
{
  __verbose++;
}

/*
 * used_chars:
 *
 *  Single bit is used for each CIDs since used_chars can be reused as a
 *  stream content of CIDSet by doing so. See, cid.h for add_to_used() and
 *  is_used().
 */

static char *
new_used_chars2(void)
{
  char *used_chars;

  used_chars = NEW(8192, char);
  memset(used_chars, 0, 8192);

  return used_chars;
}

#define FLAG_NONE              0
#define FLAG_USED_CHARS_SHARED (1 << 0)

struct Type0Font {
  char    *fontname;   /* BaseFont */
  char    *encoding;   /* "Identity-H" or "Identity-V" (not ID) */
  char    *used_chars; /* Used chars (CIDs) */

  /*
   * Type0 only
   */
  CIDFont *descendant; /* Only single descendant is allowed. */
  int      flags;
  int      wmode;
  int      cmap_id;

  /*
   * PDF Font Resource
   */
  pdf_obj *indirect;
  pdf_obj *fontdict;
  pdf_obj *descriptor; /* MUST BE NULL */
};

static void
Type0Font_init_font_struct (Type0Font *font)
{
  ASSERT(font);

  font->fontname   = NULL;
  font->fontdict   = NULL;
  font->indirect   = NULL;
  font->descriptor = NULL;
  font->encoding   = NULL;
  font->used_chars = NULL;
  font->descendant = NULL;
  font->wmode      = -1;
  font->cmap_id    = -1;
  font->flags      = FLAG_NONE;

  return;
}

static void
Type0Font_clean (Type0Font *font)
{
  if (font) {
    if (font->fontdict)
      ERROR("%s: Object not flushed.", TYPE0FONT_DEBUG_STR);
    if (font->indirect)
      ERROR("%s: Object not flushed.", TYPE0FONT_DEBUG_STR);
    if (font->descriptor)
      ERROR("%s: FontDescriptor unexpected for Type0 font.", TYPE0FONT_DEBUG_STR);
    if (!(font->flags & FLAG_USED_CHARS_SHARED) && font->used_chars)
      RELEASE(font->used_chars);
    if (font->encoding)
      RELEASE(font->encoding);
    if (font->fontname)
      RELEASE(font->fontname);
    font->fontdict   = NULL;
    font->indirect   = NULL;
    font->descriptor = NULL;
    font->used_chars = NULL;
    font->encoding   = NULL;
    font->fontname   = NULL;
  }
}

/* PLEASE FIX THIS */
#include "tt_cmap.h"

static pdf_obj *
Type0Font_create_ToUnicode_stream(Type0Font *font) {
  CIDFont *cidfont = font->descendant;
  return otf_create_ToUnicode_stream(CIDFont_get_ident(cidfont),
                                     CIDFont_get_opt_index(cidfont),
                                     Type0Font_get_usedchars(font),
                                     font->cmap_id);
}

/* Try to load ToUnicode CMap from file system first, if not found fallback to
 * font CMap reverse lookup. */
static pdf_obj *
Type0Font_try_load_ToUnicode_stream(Type0Font *font, char *cmap_base) {
  char *cmap_name = NEW(strlen(cmap_base) + strlen("-UTF-16"), char);
  pdf_obj *tounicode;

  sprintf(cmap_name, "%s-UTF16", cmap_base);
  tounicode = pdf_read_ToUnicode_file(cmap_name);
  if (!tounicode) {
    sprintf(cmap_name, "%s-UCS2", cmap_base);
    tounicode = pdf_read_ToUnicode_file(cmap_name);
  }

  RELEASE(cmap_name);

  if (!tounicode)
    tounicode = Type0Font_create_ToUnicode_stream(font);

  return tounicode;
}

static void
add_ToUnicode (Type0Font *font)
{
  pdf_obj    *tounicode;
  CIDFont    *cidfont;
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

  cidfont = font->descendant;
  if (!cidfont) {
    ERROR("%s: No descendant CID-keyed font.", TYPE0FONT_DEBUG_STR);
    return;
  }

  if (CIDFont_is_ACCFont(cidfont)) {
    /* No need to embed ToUnicode */
    return;
  } else if (CIDFont_is_UCSFont(cidfont)) {
    /*
     * Old version of dvipdfmx mistakenly used Adobe-Identity as Unicode.
     */
    tounicode = pdf_read_ToUnicode_file("Adobe-Identity-UCS2");
    if (!tounicode) { /* This should work */
      tounicode = pdf_new_name("Identity-H");
    }
    pdf_add_dict(font->fontdict, pdf_new_name("ToUnicode"), tounicode);
    return;
  }

  tounicode = NULL;
  csi       = CIDFont_get_CIDSysInfo(cidfont);
  fontname  = CIDFont_get_fontname(cidfont);
  if (CIDFont_get_embedding(cidfont)) {
    fontname += 7; /* FIXME */
  }

  if (!strcmp(csi->registry, "Adobe")    &&
      !strcmp(csi->ordering, "Identity")) {
    switch (CIDFont_get_subtype(cidfont)) {
    case CIDFONT_TYPE2:
      /* PLEASE FIX THIS */
      tounicode = Type0Font_create_ToUnicode_stream(font);
      break;
    default:
      if (CIDFont_get_flag(cidfont, CIDFONT_FLAG_TYPE1C)) { /* FIXME */
        tounicode = Type0Font_create_ToUnicode_stream(font);
      } else if (CIDFont_get_flag(cidfont, CIDFONT_FLAG_TYPE1)) { /* FIXME */
        /* Font loader will create ToUnicode and set. */
        return;
      } else {
        tounicode = Type0Font_try_load_ToUnicode_stream(font, fontname);
      }
      break;
    }
  } else {
    char *cmap_base = NEW(strlen(csi->registry) + strlen(csi->ordering) + 2, char);
    sprintf(cmap_base, "%s-%s", csi->registry, csi->ordering);
    tounicode = Type0Font_try_load_ToUnicode_stream(font, cmap_base);
    RELEASE(cmap_base);
  }

  if (tounicode) {
    pdf_add_dict(font->fontdict,
                 pdf_new_name("ToUnicode"), tounicode);
  } else {
#if defined(LIBDPX)
    if (__verbose)
      WARN("Failed to load ToUnicode CMap for font \"%s\"", fontname);
#else
    WARN("Failed to load ToUnicode CMap for font \"%s\"", fontname);
#endif /* LIBDPX */
  }

  return;
}

void
Type0Font_set_ToUnicode (Type0Font *font, pdf_obj *cmap_ref)
{
  ASSERT(font);

  pdf_add_dict(font->fontdict,
               pdf_new_name("ToUnicode"), cmap_ref);
}

static void
Type0Font_dofont (Type0Font *font)
{
  if (!font || !font->indirect)
    return;

  if (!pdf_lookup_dict(font->fontdict, "ToUnicode")) { /* FIXME */
    add_ToUnicode(font);
  }
}

static void
Type0Font_flush (Type0Font *font)
{
  if (font) {
    if (font->fontdict)
      pdf_release_obj(font->fontdict);
    font->fontdict = NULL;
    if (font->indirect)
      pdf_release_obj(font->indirect);
    font->indirect = NULL;
    if (font->descriptor)
      ERROR("%s: FontDescriptor unexpected for Type0 font.", TYPE0FONT_DEBUG_STR);
    font->descriptor = NULL;
  }
}

int
Type0Font_get_wmode (Type0Font *font)
{
  ASSERT(font);

  return font->wmode;
}

#if 0
char *
Type0Font_get_encoding (Type0Font *font)
{
  ASSERT(font);

  return font->encoding;
}
#endif

char *
Type0Font_get_usedchars (Type0Font *font)
{
  ASSERT(font);

  return font->used_chars;
}

pdf_obj *
Type0Font_get_resource (Type0Font *font)
{
  ASSERT(font);

  /*
   * This looks somewhat strange.
   */
  if (!font->indirect) {
    pdf_obj *array;

    array = pdf_new_array();
    pdf_add_array(array, CIDFont_get_resource(font->descendant));
    pdf_add_dict(font->fontdict, pdf_new_name("DescendantFonts"), array);
    font->indirect = pdf_ref_obj(font->fontdict);
  }

  return pdf_link_obj(font->indirect);
}

/******************************** CACHE ********************************/

#define CHECK_ID(n) do {\
  if ((n) < 0 || (n) >= __cache.count)\
    ERROR("%s: Invalid ID %d", TYPE0FONT_DEBUG_STR, (n));\
} while (0)

#define CACHE_ALLOC_SIZE 16u

static struct font_cache {
  int        count;
  int        capacity;
  Type0Font *fonts;
} __cache = {
  0, 0, NULL
};

void
Type0Font_cache_init (void)
{
  if (__cache.fonts)
    ERROR("%s: Already initialized.", TYPE0FONT_DEBUG_STR);
  __cache.count    = 0;
  __cache.capacity = 0;
  __cache.fonts    = NULL;
}

Type0Font *
Type0Font_cache_get (int id)
{
  CHECK_ID(id);

  return &__cache.fonts[id];
}

int
Type0Font_cache_find (const char *map_name, int cmap_id, fontmap_opt *fmap_opt)
{
  int         font_id = -1;
  Type0Font  *font;
  CIDFont    *cidfont;
  CMap       *cmap;
  CIDSysInfo *csi;
  char       *fontname = NULL;
  int         cid_id = -1, parent_id = -1, wmode = 0;
  int         pdf_ver;

  pdf_ver = pdf_get_version();
  if (!map_name || cmap_id < 0 || pdf_ver < 2)
    return -1;

  /*
   * Encoding is Identity-H or Identity-V according as thier WMode value.
   * 
   * We do not use match against the map_name since fonts (TrueType) covers
   * characters across multiple character collection (eg, Adobe-Japan1 and
   * Adobe-Japan2) must be splited into multiple CID-keyed fonts.
   */

  cmap = CMap_cache_get(cmap_id);
  csi  = (CMap_is_Identity(cmap)) ? NULL : CMap_get_CIDSysInfo(cmap) ;

  cid_id = CIDFont_cache_find(map_name, csi, fmap_opt);

  if (cid_id < 0) 
    return -1;

  /*
   * The descendant CID-keyed font has already been registerd.
   * If CID-keyed font with ID = cid_id is new font, then create new parent
   * Type 0 font. Otherwise, there already exists parent Type 0 font and
   * then we find him and return his ID. We must check against their WMode.
   */

  cidfont = CIDFont_cache_get(cid_id);
  wmode   = CMap_get_wmode(cmap);

  /* Does CID-keyed font already have parent ? */
  parent_id = CIDFont_get_parent_id(cidfont, wmode);
  if (parent_id >= 0)
    return parent_id; /* If so, we don't need new one. */

  /*
   * CIDFont does not have parent or his parent's WMode does not matched with
   * wmode. Create new Type0 font.
   */

  if (__cache.count >= __cache.capacity) {
    __cache.capacity += CACHE_ALLOC_SIZE;
    __cache.fonts     = RENEW(__cache.fonts, __cache.capacity, struct Type0Font);
  }
  font_id =  __cache.count;
  font    = &__cache.fonts[font_id];

  Type0Font_init_font_struct(font);

  /*
   * All CJK double-byte characters are mapped so that resulting
   * character codes coincide with CIDs of given character collection.
   * So, the Encoding is always Identity-H for horizontal fonts or
   * Identity-V for vertical fonts.
   */
  if (wmode) {
    font->encoding = NEW(strlen("Identity-V")+1, char);
    strcpy(font->encoding, "Identity-V");
  } else {
    font->encoding = NEW(strlen("Identity-H")+1, char);
    strcpy(font->encoding, "Identity-H");
  }
  font->wmode = wmode;
  font->cmap_id = cmap_id;

  /*
   * Now we start font dictionary.
   */
  font->fontdict = pdf_new_dict();
  pdf_add_dict(font->fontdict, pdf_new_name ("Type"),    pdf_new_name ("Font"));
  pdf_add_dict(font->fontdict, pdf_new_name ("Subtype"), pdf_new_name ("Type0"));

  /*
   * Type0 font does not have FontDescriptor because it is not a simple font.
   * Instead, DescendantFonts appears here.
   *
   * Up to PDF version 1.5, Type0 font must have single descendant font which
   * is a CID-keyed font. Future PDF spec. will allow multiple desecendant
   * fonts.
   */
  font->descendant = cidfont;
  CIDFont_attach_parent(cidfont, font_id, wmode);

  /*
   * PostScript Font name:
   *
   *  Type0 font's fontname is usually descendant CID-keyed font's font name 
   *  appended by -ENCODING.
   */
  fontname = CIDFont_get_fontname(cidfont);

  if (__verbose) {
    if (CIDFont_get_embedding(cidfont) && strlen(fontname) > 7)
      MESG("(CID:%s)", fontname+7); /* skip XXXXXX+ */
    else
      MESG("(CID:%s)", fontname);
  }

  /*
   * The difference between CID-keyed font and TrueType font appears here.
   *
   * Glyph substitution for vertical writing is done in CMap mapping process
   * for CID-keyed fonts. But we must rely on OpenType layout table in the
   * case of TrueType fonts. So, we must use different used_chars for each
   * horizontal and vertical fonts in that case.
   *
   * In most PDF file, encoding name is not appended to fontname for Type0
   * fonts having CIDFontType 2 font as their descendant.
   */

  font->used_chars = NULL;
  font->flags      = FLAG_NONE;

  switch (CIDFont_get_subtype(cidfont)) {
  case CIDFONT_TYPE0:
    font->fontname = NEW(strlen(fontname)+strlen(font->encoding)+2, char);
    sprintf(font->fontname, "%s-%s", fontname, font->encoding);
    pdf_add_dict(font->fontdict,
                 pdf_new_name("BaseFont"), pdf_new_name(font->fontname));
    /*
     * Need used_chars to write W, W2.
     */
    if ((parent_id = CIDFont_get_parent_id(cidfont, wmode ? 0 : 1)) < 0) {
      font->used_chars = new_used_chars2();
    } else {
      /* Don't allocate new one. */
      font->used_chars = Type0Font_get_usedchars(Type0Font_cache_get(parent_id));
      font->flags     |= FLAG_USED_CHARS_SHARED;
    }
    break;
  case CIDFONT_TYPE2:
    /*
     * TrueType:
     *
     *  Use different used_chars for H and V.
     */
    pdf_add_dict(font->fontdict,
                 pdf_new_name("BaseFont"), pdf_new_name(fontname));
    font->used_chars = new_used_chars2();
    break;
  default:
    ERROR("Unrecognized CIDFont Type");
    break;
  }

  pdf_add_dict(font->fontdict,
               pdf_new_name("Encoding"), pdf_new_name(font->encoding));

  __cache.count++;

  return font_id;
}

void
Type0Font_cache_close (void)
{
  int   font_id;

  /*
   * This need to be fixed.
   *
   * CIDFont_cache_close() before Type0Font_release because of used_chars.
   * ToUnicode support want descendant CIDFont's CSI and fontname.
   */
  if (__cache.fonts) {
    for (font_id = 0; font_id < __cache.count; font_id++)
      Type0Font_dofont(&__cache.fonts[font_id]);
  }
  CIDFont_cache_close();
  if (__cache.fonts) {
    for (font_id = 0; font_id < __cache.count; font_id++) {
      Type0Font_flush(&__cache.fonts[font_id]);
      Type0Font_clean(&__cache.fonts[font_id]);
    }
    RELEASE(__cache.fonts);
  }
  __cache.fonts    = NULL;
  __cache.count    = 0;
  __cache.capacity = 0;
}

/******************************** COMPAT ********************************/

#ifndef WITHOUT_COMPAT

#include "cmap_read.h"
#include "cmap_write.h"
#include "pdfresource.h"
#include "pdfencoding.h"

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

  ASSERT(cmap_name);

  res_id = pdf_findresource("CMap", cmap_name);
  if (res_id < 0) {
    if (!strcmp(cmap_name, "Adobe-Identity-UCS2"))
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
