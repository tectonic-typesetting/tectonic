/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2008-2018 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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

#ifdef _WIN32
#include <windows.h> /* GetEnvironmentVariable */
#endif

#include "dpx-pdffont.h"

#include <assert.h>
#include <errno.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
#include "dpx-cid.h"
#include "dpx-cidtype0.h"
#include "dpx-cmap.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfencoding.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"
#include "dpx-pkfont.h"
#include "dpx-truetype.h"
#include "dpx-tt_cmap.h"
#include "dpx-type0.h"
#include "dpx-type1.h"
#include "dpx-type1c.h"

#define MREC_HAS_TOUNICODE(m) ((m) && (m)->opt.tounicode)

void
pdf_font_set_dpi (int font_dpi)
{
  PKFont_set_dpi(font_dpi);
}

/* If we're using deterministic "unique" tags, `state` is a counter
 * incremented every time we generate a tag. If we're emulating stock
 * xdvipdfmx behavior, it is a boolean flag indicating whether we need to seed
 * the random number generator (`first` in the reference source).
 */
static int unique_tag_state = 1;
static int unique_tags_deterministic = 0;

void
pdf_font_reset_unique_tag_state(void)
{
    unique_tag_state = 1;
}

void
pdf_font_set_deterministic_unique_tags(int value)
{
    unique_tags_deterministic = value;
}

void
pdf_font_make_uniqueTag (char *tag)
{
  // TODO: remove randomized tag emulation
  int    i;
  char   ch;

  if (unique_tags_deterministic) {
      snprintf(tag, 7, "%06d", unique_tag_state);
      unique_tag_state++;
      return;
  }

  /* below, stock xdvipdfmx behavior: randomized tags */

  if (unique_tag_state) {
    srand(0);
    unique_tag_state = 0;
  }

  for (i = 0; i < 6; i++) {
    ch = rand() % 26;
    tag[i] = ch + 'A';
  }
  tag[6] = '\0';
}


struct pdf_font
{
  char    *ident;
  int      subtype;

  char    *map_name;

  int      encoding_id; /* encoding or CMap */

  /*
   * If subtype is Type0, it simply points font_id
   * of Type0 font. Type0 and simple font is not
   * unified yet.
   */
  int      font_id;

  /* For simple font */
  int      index;
  char    *fontname;
  char     uniqueID[7];

  /*
   * PDF font resource objects
   */
  pdf_obj *reference;
  pdf_obj *resource;
  pdf_obj *descriptor;

  /*
   * Font format specific data
   */
  char    *usedchars;
  int      flags;

  /* PK font */
  double   point_size;
  double   design_size;
};

static void
pdf_init_font_struct (pdf_font *font)
{
  assert(font);

  font->ident    = NULL;
  font->map_name = NULL;
  font->subtype  = -1;
  font->font_id  = -1; /* Type0 ID */
  font->fontname = NULL;
  memset(font->uniqueID, 0, 7);
  font->index    = 0;

  font->encoding_id = -1;

  font->reference   = NULL;
  font->resource    = NULL;
  font->descriptor  = NULL;

  font->point_size  = 0;
  font->design_size = 0;

  font->usedchars   = NULL;
  font->flags       = 0;

  return;
}

static void
pdf_flush_font (pdf_font *font)
{
  char *fontname, *uniqueTag;

  if (!font) {
    return;
  }

  if (font->resource && font->reference) {
    if (font->subtype != PDF_FONT_FONTTYPE_TYPE3) {
      if (pdf_font_get_flag(font, PDF_FONT_FLAG_NOEMBED)) {
        pdf_add_dict(font->resource,
                     pdf_new_name("BaseFont"), pdf_new_name(font->fontname));
        if (font->descriptor) {
          pdf_add_dict(font->descriptor,
                       pdf_new_name("FontName"), pdf_new_name(font->fontname));
        }
      } else {
        if (!font->fontname) {
          _tt_abort("Undefined in fontname... (%s)", font->ident);
        }
        fontname  = NEW(7+strlen(font->fontname)+1, char);
        uniqueTag = pdf_font_get_uniqueTag(font);
        sprintf(fontname, "%6s+%s", uniqueTag, font->fontname);
        pdf_add_dict(font->resource,
                     pdf_new_name("BaseFont"), pdf_new_name(fontname));
        if (font->descriptor) {
          pdf_add_dict(font->descriptor,
                       pdf_new_name("FontName"), pdf_new_name(fontname));
        }
        free(fontname);
      }
      if (font->descriptor) {
        pdf_add_dict(font->resource,
                     pdf_new_name("FontDescriptor"), pdf_ref_obj(font->descriptor));
      }
    }
  }

  pdf_release_obj(font->resource);
  pdf_release_obj(font->descriptor);
  pdf_release_obj(font->reference);

  font->reference  = NULL;
  font->resource   = NULL;
  font->descriptor = NULL;

  return;
}

static void
pdf_clean_font_struct (pdf_font *font)
{
  if (font) {
    free(font->ident);
    free(font->map_name);
    free(font->fontname);
    free(font->usedchars);

    if (font->reference)
      _tt_abort("pdf_font>> Object not flushed.");
    if (font->resource)
      _tt_abort("pdf_font> Object not flushed.");
    if (font->descriptor)
      _tt_abort("pdf_font>> Object not flushed.");

    font->ident     = NULL;
    font->map_name  = NULL;
    font->fontname  = NULL;
    font->usedchars = NULL;
  }

  return;
}

#define CACHE_ALLOC_SIZE 16u

static struct {
  int       count;
  int       capacity;
  pdf_font *fonts;
} font_cache = {
  0, 0, NULL
};

void
pdf_init_fonts (void)
{
  assert(font_cache.fonts == NULL);

  agl_init_map();

  CMap_cache_init();
  pdf_init_encodings();

  Type0Font_cache_init();

  font_cache.count    = 0;
  font_cache.capacity = CACHE_ALLOC_SIZE;
  font_cache.fonts    = NEW(font_cache.capacity, pdf_font);
}

#define CHECK_ID(n) do {\
  if ((n) < 0 || (n) >= font_cache.count) {\
    _tt_abort("Invalid font ID: %d", (n));\
  }\
} while (0)
#define GET_FONT(n)  (&(font_cache.fonts[(n)]))


pdf_obj *
pdf_get_font_reference (int font_id)
{
  pdf_font  *font;

  CHECK_ID(font_id);

  font = GET_FONT(font_id);
  if (font->subtype == PDF_FONT_FONTTYPE_TYPE0) {
    Type0Font *t0font;

    t0font = Type0Font_cache_get(font->font_id);
    return Type0Font_get_resource(t0font);
  } else {
    if (!font->reference) {
      font->reference = pdf_ref_obj(pdf_font_get_resource(font));
    }
  }

  return pdf_link_obj(font->reference);
}

char *
pdf_get_font_usedchars (int font_id)
{
  pdf_font *font;

  CHECK_ID(font_id);

  font = GET_FONT(font_id);
  if (font->subtype == PDF_FONT_FONTTYPE_TYPE0) {
    Type0Font *t0font;

    t0font = Type0Font_cache_get(font->font_id);
    return Type0Font_get_usedchars(t0font);
  } else {
    if (!font->usedchars) {
      font->usedchars = NEW(256, char);
      memset(font->usedchars, 0, 256 * sizeof(char));
    }
    return font->usedchars;
  }
}

int
pdf_get_font_wmode (int font_id)
{
  pdf_font *font;

  CHECK_ID(font_id);

  font = GET_FONT(font_id);
  if (font->subtype == PDF_FONT_FONTTYPE_TYPE0) {
    Type0Font *t0font;

    t0font = Type0Font_cache_get(font->font_id);
    return Type0Font_get_wmode(t0font);
  } else {
    return 0;
  }
}

int
pdf_get_font_subtype (int font_id)
{
  pdf_font *font;

  CHECK_ID(font_id);

  font = GET_FONT(font_id);

  return font->subtype;
}

int
pdf_get_font_encoding (int font_id)
{
  pdf_font *font;

  CHECK_ID(font_id);

  font = GET_FONT(font_id);

  return font->encoding_id;
}

/* The rule for ToUnicode creation is:
 *
 *  If "tounicode" option is specified in fontmap, use that.
 *  If there is ToUnicode CMap with same name as TFM, use that.
 *  If no "tounicode" option is used and no ToUnicode CMap with
 *  same name as TFM is found, create ToUnicode CMap from glyph
 *  names and AGL file.
 */
static int
try_load_ToUnicode_CMap (pdf_font *font)
{
  pdf_obj     *fontdict;
  pdf_obj     *tounicode;
  const char  *cmap_name = NULL;
  fontmap_rec *mrec; /* Be sure fontmap is still alive here */

  assert(font);

  /* We are using different encoding for Type0 font.
   * This feature is unavailable for them.
   */
  if (font->subtype == PDF_FONT_FONTTYPE_TYPE0)
    return  0;

  assert(font->map_name);

  mrec = pdf_lookup_fontmap_record(font->map_name);
  if (MREC_HAS_TOUNICODE(mrec))
    cmap_name = mrec->opt.tounicode;
  else {
    cmap_name = font->map_name;
  }

  fontdict  = pdf_font_get_resource(font);
  tounicode = pdf_load_ToUnicode_stream(cmap_name);
  if (!tounicode && MREC_HAS_TOUNICODE(mrec))
    dpx_warning("Failed to read ToUnicode mapping \"%s\"...", mrec->opt.tounicode);
  else if (tounicode) {
    if (pdf_obj_typeof(tounicode) != PDF_STREAM)
      _tt_abort("Object returned by pdf_load_ToUnicode_stream() not stream object! (This must be bug)");
    else if (pdf_stream_length(tounicode) > 0) {
      pdf_add_dict(fontdict,
                   pdf_new_name("ToUnicode"),
                   pdf_ref_obj (tounicode)); /* _FIXME_ */
      if (dpx_conf.verbose_level > 0)
        dpx_message("pdf_font>> ToUnicode CMap \"%s\" attached to font id=\"%s\".\n",
             cmap_name, font->map_name);
    }
    pdf_release_obj(tounicode);
  }

  return  0;
}

void
pdf_close_fonts (void)
{
  int  font_id;

  for (font_id = 0;
       font_id < font_cache.count; font_id++) {
    pdf_font  *font;

    font = GET_FONT(font_id);

    if (dpx_conf.verbose_level > 0) {
      if (font->subtype != PDF_FONT_FONTTYPE_TYPE0) {
        dpx_message("(%s", pdf_font_get_ident(font));
        if (dpx_conf.verbose_level > 2 &&
            !pdf_font_get_flag(font, PDF_FONT_FLAG_NOEMBED)) {
          dpx_message("[%s+%s]",
               pdf_font_get_uniqueTag(font),
               pdf_font_get_fontname(font));
        } else if (dpx_conf.verbose_level > 1) {
          dpx_message("[%s]",
               pdf_font_get_fontname(font));
        }
        if (dpx_conf.verbose_level > 1) {
          if (pdf_font_get_encoding(font) >= 0) {
            dpx_message("[%s]",
                 pdf_encoding_get_name(pdf_font_get_encoding(font)));
          } else {
            dpx_message("[built-in]");
          }
        }

      }
    }

    /* Must come before load_xxx */
    try_load_ToUnicode_CMap(font);

    /* Type 0 is handled separately... */
    switch (font->subtype) {
    case PDF_FONT_FONTTYPE_TYPE1:
      if (dpx_conf.verbose_level > 0)
        dpx_message("[Type1]");
      if (!pdf_font_get_flag(font, PDF_FONT_FLAG_BASEFONT))
        pdf_font_load_type1(font);
      break;
    case PDF_FONT_FONTTYPE_TYPE1C:
      if (dpx_conf.verbose_level > 0)
        dpx_message("[Type1C]");
      pdf_font_load_type1c(font);
      break;
    case PDF_FONT_FONTTYPE_TRUETYPE:
      if (dpx_conf.verbose_level > 0)
        dpx_message("[TrueType]");
      pdf_font_load_truetype(font);
      break;
    case PDF_FONT_FONTTYPE_TYPE3:
      if (dpx_conf.verbose_level > 0)
        dpx_message("[Type3/PK]");
      pdf_font_load_pkfont (font);
      break;
    case PDF_FONT_FONTTYPE_TYPE0:
      break;
    default:
      _tt_abort("Unknown font type: %d", font->subtype);
      break;
    }

    if (font->encoding_id >= 0 && font->subtype != PDF_FONT_FONTTYPE_TYPE0)
      pdf_encoding_add_usedchars(font->encoding_id, font->usedchars);

    if (dpx_conf.verbose_level > 0) {
      if (font->subtype != PDF_FONT_FONTTYPE_TYPE0)
        dpx_message(")");
    }
  }

  pdf_encoding_complete();

  for (font_id = 0; font_id < font_cache.count; font_id++) {
    pdf_font *font = GET_FONT(font_id);

    if (font->encoding_id >= 0 && font->subtype != PDF_FONT_FONTTYPE_TYPE0) {
      pdf_obj *enc_obj = pdf_get_encoding_obj(font->encoding_id);
      pdf_obj *tounicode;

      /* Predefined encodings (and those simplified to them) are embedded
       * as direct objects, but this is purely a matter of taste. 
       */
      if (enc_obj)
        pdf_add_dict(font->resource,
                     pdf_new_name("Encoding"),
                     PDF_OBJ_NAMETYPE(enc_obj) ? pdf_link_obj(enc_obj) : pdf_ref_obj(enc_obj));
      /* For built-in encoding, each font loader create ToUnicode CMap. */
      if (!pdf_lookup_dict(font->resource, "ToUnicode")
          && (tounicode = pdf_encoding_get_tounicode(font->encoding_id))) {
          if (tounicode) {
            pdf_add_dict(font->resource,
                        pdf_new_name("ToUnicode"), pdf_ref_obj(tounicode));
          }
      }
    } else if (font->subtype == PDF_FONT_FONTTYPE_TRUETYPE) {
      /* encoding_id < 0 means MacRoman here (but not really)
       * We use MacRoman as "default" encoding. */
      pdf_add_dict(font->resource,
                   pdf_new_name("Encoding"),
                   pdf_new_name("MacRomanEncoding"));
    }

    pdf_flush_font(font);
    pdf_clean_font_struct(font);
  }
  font_cache.fonts = mfree(font_cache.fonts);
  font_cache.count    = 0;
  font_cache.capacity = 0;

  Type0Font_cache_close();

  CMap_cache_close();
  pdf_close_encodings();

  agl_close_map (); /* After encoding */

  return;
}

int
pdf_font_findresource (const char *tex_name,
                       double font_scale, fontmap_rec *mrec)
{
  int          font_id = -1;
  pdf_font    *font;
  int          encoding_id = -1, cmap_id = -1;
  const char  *fontname;

  /*
   * Get appropriate info from map file. (PK fonts at two different
   * point sizes would be looked up twice unecessarily.)
   */
  fontname = mrec ? mrec->font_name : tex_name;
  /* XeTeX specific...
   * First try loading GID-to-CID mapping from CFF CID-keyed OpenType font.
   * There was a serious bug in xdv support... It was implemented with the wrong
   * assumption that CID always equals to GID. 
   * TODO: There is a possibility that GID-to-CID mapping is not one-to-one.
   * Use internal glyph ordering rather than map GID to CIDs.
   */
  if (mrec && mrec->opt.use_glyph_encoding) {
    int wmode = 0;
    /* Should be always Identity-H or Identity-V for XeTeX output. */
    if (mrec->enc_name) {
      if (!strcmp(mrec->enc_name, "Identity-V"))
        wmode = 1;
      else if (!strcmp(mrec->enc_name, "Identity-H"))
        wmode = 0;
      else {
        dpx_warning("Unexpected encoding specified for xdv: %s", mrec->enc_name);
      }
    /* cmap_id < 0 is returned if ...
     *  Font is not a CFF font
     *  GID to CID mapping is identity mapping
     * 
     * TODO: fontmap record still has Identity CMap assigned but actually different CMap
     * can be attached to the font here. Should we fix mrec->enc_name here?
     */
      cmap_id = otf_try_load_GID_to_CID_map(mrec->font_name, mrec->opt.index, wmode);
    }
  }
  if (cmap_id < 0 && mrec && mrec->enc_name) {
#define MAYBE_CMAP(s) (!strstr((s), ".enc") || strstr((s), ".cmap"))
    if (MAYBE_CMAP(mrec->enc_name)) {
      cmap_id = CMap_cache_find(mrec->enc_name);
      if (cmap_id >= 0) {
        CMap  *cmap;
        int    cmap_type, minbytes;

        cmap      = CMap_cache_get(cmap_id);
        cmap_type = CMap_get_type (cmap);
        minbytes  = CMap_get_profile(cmap, CMAP_PROF_TYPE_INBYTES_MIN);
        /*
         * Check for output encoding.
         */
        if (cmap_type != CMAP_TYPE_IDENTITY    &&
            cmap_type != CMAP_TYPE_CODE_TO_CID &&
            cmap_type != CMAP_TYPE_TO_UNICODE) {
          dpx_warning("Only 16-bit encoding supported for output encoding.");
        }
        /*
         * Turn on map option.
         */
        if (minbytes == 2 && mrec->opt.mapc < 0) {
          if (dpx_conf.verbose_level > 0) {
            dpx_message("\n");
            dpx_message("pdf_font>> Input encoding \"%s\" requires at least 2 bytes.\n",
                 CMap_get_name(cmap));
            dpx_message("pdf_font>> The -m <00> option will be assumed for \"%s\".\n", mrec->font_name);
          }
          /* FIXME: The following code modifies mrec. */
          mrec->opt.mapc = 0;
        }
      } else if (streq_ptr(mrec->enc_name, "unicode")) {
        cmap_id = otf_load_Unicode_CMap(mrec->font_name,
                                        mrec->opt.index, mrec->opt.otl_tags,
                                        ((mrec->opt.flags & FONTMAP_OPT_VERT) ? 1 : 0));
        if (cmap_id < 0) {
          cmap_id = t1_load_UnicodeCMap(mrec->font_name, mrec->opt.otl_tags,
                                        ((mrec->opt.flags & FONTMAP_OPT_VERT) ? 1 : 0));
        }
        if (cmap_id < 0)
          _tt_abort("Failed to read UCS2/UCS4 TrueType cmap...");
      }
    }
    if (cmap_id < 0) {
      encoding_id = pdf_encoding_findresource(mrec->enc_name);
      if (encoding_id < 0)
        _tt_abort("Could not find encoding file \"%s\".", mrec->enc_name);
    }
  }
  if (mrec && cmap_id >= 0) {
    /*
     * Composite Font
     */
    int  type0_id, found = 0;

    type0_id = Type0Font_cache_find(mrec->font_name, cmap_id, &mrec->opt);
    if (type0_id < 0) {
      return -1;
    }

    for (font_id = 0;
         font_id < font_cache.count; font_id++) {
      font = GET_FONT(font_id);
      if (font->subtype == PDF_FONT_FONTTYPE_TYPE0 &&
          font->font_id == type0_id &&
          font->encoding_id == cmap_id) {
        found = 1;
        if (dpx_conf.verbose_level > 0) {
          dpx_message("\npdf_font>> Type0 font \"%s\" (cmap_id=%d) found at font_id=%d.\n",
               mrec->font_name, cmap_id, font_id);
        }
        break;
      }
    }

    if (!found) {
      font_id = font_cache.count;
      if (font_cache.count >= font_cache.capacity) {
        font_cache.capacity += CACHE_ALLOC_SIZE;
        font_cache.fonts     = RENEW(font_cache.fonts, font_cache.capacity, pdf_font);
      }
      font    = GET_FONT(font_id);
      pdf_init_font_struct(font);

      font->font_id     = type0_id;
      font->subtype     = PDF_FONT_FONTTYPE_TYPE0;
      font->encoding_id = cmap_id;

      font_cache.count++;

      if (dpx_conf.verbose_level > 0) {
        dpx_message("\npdf_font>> Type0 font \"%s\"", fontname);
        dpx_message(" cmap_id=<%s,%d>", mrec->enc_name, font->encoding_id);
        dpx_message(" opened at font_id=<%s,%d>.\n", tex_name, font_id);
      }

    }
  } else {
    /*
     * Simple Font - always embed.
     */
    int  found = 0;

    for (font_id = 0;
         font_id < font_cache.count; font_id++) {
      font = GET_FONT(font_id);
      switch (font->subtype) {
      case PDF_FONT_FONTTYPE_TYPE1:
      case PDF_FONT_FONTTYPE_TYPE1C:
      case PDF_FONT_FONTTYPE_TRUETYPE:
        /* fontname here is font file name.
         * We must compare both font file name and encoding
         *
         * TODO: Embed a font only once if it is used
         *       with two different encodings
         */
        if (streq_ptr(fontname, font->ident)   &&
            encoding_id == font->encoding_id) {
          if (mrec && mrec->opt.index == font->index)
            found = 1;
        }
        break;
      case PDF_FONT_FONTTYPE_TYPE3:
        /* There shouldn't be any encoding specified for PK font.
         * It must be always font's build-in encoding.
         *
         * TODO: a PK font with two encodings makes no sense. Change?
         */
        if (streq_ptr(fontname, font->ident) &&
            font_scale == font->point_size) {
          found = 1;
        }
        break;
      case PDF_FONT_FONTTYPE_TYPE0:
        break;
      default:
        _tt_abort("Unknown font type: %d", font->subtype);
        break;
      }

      if (found) {
        if (dpx_conf.verbose_level > 0) {
          dpx_message("\npdf_font>> Simple font \"%s\" (enc_id=%d) found at id=%d.\n",
               fontname, encoding_id, font_id);
        }
        break;
      }
    }


    if (!found) {
      font_id = font_cache.count;
      if (font_cache.count >= font_cache.capacity) {
        font_cache.capacity += CACHE_ALLOC_SIZE;
        font_cache.fonts     = RENEW(font_cache.fonts, font_cache.capacity, pdf_font);
      }

      font = GET_FONT(font_id);

      pdf_init_font_struct(font);

      font->point_size  = font_scale;
      font->encoding_id = encoding_id;
      font->ident       = NEW(strlen(fontname) + 1, char);
      strcpy(font->ident, fontname);
      font->map_name    = NEW(strlen(tex_name) + 1, char);
      strcpy(font->map_name, tex_name);
      font->index       = (mrec && mrec->opt.index) ? mrec->opt.index : 0;

      if (pdf_font_open_type1(font) >= 0) {
        font->subtype = PDF_FONT_FONTTYPE_TYPE1;
      } else if (pdf_font_open_type1c(font) >= 0) {
        font->subtype = PDF_FONT_FONTTYPE_TYPE1C;
      } else if (pdf_font_open_truetype(font) >= 0) {
        font->subtype = PDF_FONT_FONTTYPE_TRUETYPE;
      } else if (pdf_font_open_pkfont(font) >= 0) {
        font->subtype = PDF_FONT_FONTTYPE_TYPE3;
      } else {
        pdf_clean_font_struct(font);
        return -1;
      }

      font_cache.count++;

      if (dpx_conf.verbose_level > 0) {
        dpx_message("\npdf_font>> Simple font \"%s\"", fontname);
        dpx_message(" enc_id=<%s,%d>",
             (mrec && mrec->enc_name) ? mrec->enc_name : "builtin", font->encoding_id);
        dpx_message(" opened at font_id=<%s,%d>.\n", tex_name, font_id);
      }
    }
  }

  return  font_id;
}

bool
pdf_font_is_in_use (pdf_font *font)
{
  assert(font);

  return font->reference ? true : false;
}

int
pdf_font_get_index (pdf_font *font)
{
  assert(font);

  return font->index;
}

char *
pdf_font_get_ident (pdf_font *font)
{
  assert(font);

  return font->ident;
}

char *
pdf_font_get_mapname (pdf_font *font)
{
  assert(font);

  return font->map_name;
}

char *
pdf_font_get_fontname (pdf_font *font)
{
  assert(font);

  return font->fontname;
}

pdf_obj *
pdf_font_get_resource (pdf_font *font)
{
  assert(font);

  if (!font->resource) {
    font->resource = pdf_new_dict();
    pdf_add_dict(font->resource,
                 pdf_new_name("Type"),      pdf_new_name("Font"));
    switch (font->subtype) {
    case PDF_FONT_FONTTYPE_TYPE1:
    case PDF_FONT_FONTTYPE_TYPE1C:
      pdf_add_dict(font->resource,
                   pdf_new_name("Subtype"), pdf_new_name("Type1"));
      break;
    case PDF_FONT_FONTTYPE_TYPE3:
      pdf_add_dict(font->resource,
                   pdf_new_name("Subtype"), pdf_new_name("Type3"));
      break;
    case PDF_FONT_FONTTYPE_TRUETYPE:
      pdf_add_dict(font->resource,
                   pdf_new_name("Subtype"), pdf_new_name("TrueType"));
      break;
    default:
      break;
    }
  }

  return font->resource;
}

pdf_obj *
pdf_font_get_descriptor (pdf_font *font)
{
  assert(font);

  if (!font->descriptor) {
    font->descriptor = pdf_new_dict();
    pdf_add_dict(font->descriptor,
                 pdf_new_name("Type"), pdf_new_name("FontDescriptor"));
  }

  return font->descriptor;
}

char *
pdf_font_get_usedchars (pdf_font *font)
{
  assert(font);

  return font->usedchars;
}

int
pdf_font_get_encoding (pdf_font *font)
{
  assert(font);

  return font->encoding_id;
}

int
pdf_font_get_flag (pdf_font *font, int mask)
{
  assert(font);

  return ((font->flags & mask) ? 1 : 0);
}

double
pdf_font_get_param (pdf_font *font, int param_type)
{
  double param = 0.0;

  assert(font);

  switch (param_type) {
  case PDF_FONT_PARAM_DESIGN_SIZE:
    param = font->design_size;
    break;
  case PDF_FONT_PARAM_POINT_SIZE:
    param = font->point_size;
    break;
  default:
    break;
  }

  return param;
}

char *
pdf_font_get_uniqueTag (pdf_font *font)
{
  assert(font);

  if (font->uniqueID[0] == '\0') {
    pdf_font_make_uniqueTag(font->uniqueID);
  }

  return font->uniqueID;
}

int
pdf_font_set_fontname (pdf_font *font, const char *fontname)
{
  assert(font && fontname);

  if (strlen(fontname) > PDF_NAME_LEN_MAX) {
    _tt_abort("Unexpected error...");
  }
  if (font->fontname) {
    free(font->fontname);
  }
  font->fontname = NEW(strlen(fontname)+1, char);
  strcpy(font->fontname, fontname);

  return 0;
}

int
pdf_font_set_subtype (pdf_font *font, int subtype)
{
  assert(font);

  font->subtype = subtype;

  return 0;
}

int
pdf_font_set_flags (pdf_font *font, int flags)
{
  assert(font);

  font->flags |= flags;

  return 0;
}
