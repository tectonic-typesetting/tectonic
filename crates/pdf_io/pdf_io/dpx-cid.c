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
 * CID-keyed font support:
 *
 *  See also, cidtype0, and cidtype2
 */

#include "dpx-cid.h"

#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_types.h"
#include "dpx-cidtype0.h"
#include "dpx-cidtype2.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"

#define PDF_CID_SUPPORT_MIN 2
#define PDF_CID_SUPPORT_MAX 6

/*
 * Unicode and PDF Standard Character Collections.
 *
 *  Adobe-Identity is only for TrueType fonts and it means font's
 *  internal glyph ordering.
 */
static struct {
  const char *registry;
  const char *ordering;
  /* Heighest Supplement values supported by PDF-1.0, 1.1, ...; see
   * also http://partners.adobe.com/public/developer/font/index.html#ckf
   */
  int   supplement[21];
} CIDFont_stdcc_def[] = {
  {"Adobe", "UCS",      {-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0}},
  {"Adobe", "GB1",      {-1, -1, 0, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4}},
  {"Adobe", "CNS1",     {-1, -1, 0, 0, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4}},
  {"Adobe", "Japan1",   {-1, -1, 2, 2, 4, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    6, 6, 6, 6, 6}},
  {"Adobe", "Korea1",   {-1, -1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2}},
  {"Adobe", "Identity", {-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0}},
  {NULL,    NULL,       { 0,  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0}}
};
#define SUP_IDX_MAX 20
#define UCS_CC    0
#define ACC_START 1
#define ACC_END   4

static char registry_Adobe[] = "Adobe";
static char ordering_Identity[] = "Identity";
static char ordering_UCS[] = "UCS";

CIDSysInfo CSI_IDENTITY = {
  registry_Adobe,
  ordering_Identity,
  0
};

CIDSysInfo CSI_UNICODE = {
  registry_Adobe,
  ordering_UCS,
  0
};

/*
 * Optional supplement after alias name.
 */
static struct {
  const char *name;
  int   index;
} CIDFont_stdcc_alias[] = {
  {"AU",     0}, {"AG1",    1}, {"AC1",    2}, {"AJ1",    3}, {"AK1",    4}, {"AI", 5},
  {"UCS",    0}, {"GB1",    1}, {"CNS1",   2}, {"Japan1", 3}, {"Korea1", 4}, {"Identity", 5},
  {"U",      0}, {"G",      1}, {"C",      2}, {"J",      3}, {"K",      4}, {"I", 5},
  {NULL,     0}
};

static void release_opt (cid_opt *opt);
static int get_cidsysinfo (CIDSysInfo *csi, const char *map_name, const fontmap_opt *fmap_opt);

int opt_flags_cidfont = 0;

int
CIDFont_is_ACCFont (pdf_font *font)
{
  int         i;
  CIDSysInfo *csi;

  assert(font);

  csi = &font->cid.csi;
  for (i = ACC_START; i <= ACC_END ; i++) {
    if (!strcmp(csi->registry, CIDFont_stdcc_def[i].registry) &&
        !strcmp(csi->ordering, CIDFont_stdcc_def[i].ordering))
      return 1;
  }

  return 0;
}

int
CIDFont_is_UCSFont (pdf_font *font)
{
  CIDSysInfo *csi;

  assert(font);

  csi = &font->cid.csi;

  if (!strcmp(csi->ordering, "UCS") ||
      !strcmp(csi->ordering, "UCS2"))
    return 1;

  return 0;
}

char *
CIDFont_get_usedchars (pdf_font *font)
{
  if (!font->usedchars) {
    font->usedchars = NEW(8192, char);
    memset(font->usedchars, 0, 8192*sizeof(char));
  }

  return font->usedchars;
}

char *
CIDFont_get_usedchars_v (pdf_font *font)
{
  if (!font->cid.usedchars_v) {
    font->cid.usedchars_v = NEW(8192, char);
    memset(font->cid.usedchars_v, 0, 8192*sizeof(char));
  }

  return font->cid.usedchars_v;
}


static int
source_font_type (pdf_font *font)
{
  int type = PDF_FONT_FONTTYPE_CIDTYPE0;

  assert(font);

  if (font->flags & CIDFONT_FLAG_TYPE1) {
    type = PDF_FONT_FONTTYPE_TYPE1;
  } else if (font->flags & CIDFONT_FLAG_TYPE1C) {
    type = PDF_FONT_FONTTYPE_TYPE1C;
  } else if (font->flags & CIDFONT_FLAG_TRUETYPE) {
    type = PDF_FONT_FONTTYPE_TRUETYPE;
  }

  return type;
}

void
pdf_font_load_cidfont (pdf_font *font)
{
  int error = 0;

  if (!font || !font->reference)
    return;

  if (dpx_conf.verbose_level > 0)
    dpx_message(":%s", font->filename);
  if (dpx_conf.verbose_level > 1) {
    if (font->fontname)
      dpx_message("[%s]", font->fontname);
  }

  switch (font->subtype) {
  case PDF_FONT_FONTTYPE_CIDTYPE0:
    if(dpx_conf.verbose_level > 0)
      dpx_message("[CIDFontType0]");
    switch (source_font_type(font)) {
    case PDF_FONT_FONTTYPE_TYPE1:
      error = CIDFont_type0_t1dofont(font);
      break;
    case PDF_FONT_FONTTYPE_TYPE1C:
      error = CIDFont_type0_t1cdofont(font);
      break;
    default:
      error = CIDFont_type0_dofont(font);
      break;
    }
    break;
  case PDF_FONT_FONTTYPE_CIDTYPE2:
    if(dpx_conf.verbose_level > 0)
      dpx_message("[CIDFontType2]");
    error = CIDFont_type2_dofont(font);
    break;
  }

  if (error)
    _tt_abort("Error occurred while loading font: %s", font->filename);

  return;
}

#include "dpx-cid_basefont.h"
#include "dpx-pdfparse.h"

static int
CIDFont_base_open (pdf_font *font, const char *name, cid_opt *opt)
{
  pdf_obj *fontdict, *descriptor;
  char    *fontname = NULL;
  int      idx;

  assert(font);

  for (idx = 0; cid_basefont[idx].fontname != NULL; idx++) {
    if (streq_ptr(name, cid_basefont[idx].fontname) ||
        (strlen(name) == strlen(cid_basefont[idx].fontname) - strlen("-Acro") &&
         !strncmp(name, cid_basefont[idx].fontname,
                  strlen(cid_basefont[idx].fontname)-strlen("-Acro")))
        )
      break;
  }

  if (cid_basefont[idx].fontname == NULL)
    return -1;

  fontname = NEW(strlen(name)+12, char);
  memset(fontname, 0, strlen(name)+12);
  strcpy(fontname, name);

  switch (opt->style) {
  case FONT_STYLE_BOLD:
    strcat(fontname, ",Bold");
    break;
  case FONT_STYLE_ITALIC:
    strcat(fontname, ",Italic");
    break;
  case FONT_STYLE_BOLDITALIC:
    strcat(fontname, ",BoldItalic");
    break;
  }
  {
    const char *start;
    const char *end;

    start = cid_basefont[idx].fontdict;
    end   = start + strlen(start);
    fontdict   = parse_pdf_dict(&start, end, NULL);
    start = cid_basefont[idx].descriptor;
    end   = start + strlen(start);
    descriptor = parse_pdf_dict(&start, end, NULL);

    assert(fontdict && descriptor);
  }

  font->fontname = fontname;
  font->flags   |= PDF_FONT_FLAG_BASEFONT;
  {
    char    *registry, *ordering;
    int      supplement;
    pdf_obj *tmp;

    tmp = pdf_lookup_dict(fontdict, "CIDSystemInfo");

    assert( tmp && pdf_obj_typeof(tmp) == PDF_DICT );

    registry   = pdf_string_value(pdf_lookup_dict(tmp, "Registry"));
    ordering   = pdf_string_value(pdf_lookup_dict(tmp, "Ordering"));
    supplement = pdf_number_value(pdf_lookup_dict(tmp, "Supplement"));

    font->cid.csi.registry = NEW(strlen(registry)+1, char);
    font->cid.csi.ordering = NEW(strlen(ordering)+1, char);
    strcpy(font->cid.csi.registry, registry);
    strcpy(font->cid.csi.ordering, ordering);
    font->cid.csi.supplement = supplement;
  }

  {
    pdf_obj *tmp;
    char    *type;

    tmp  = pdf_lookup_dict(fontdict, "Subtype");
    assert( tmp != NULL && pdf_obj_typeof(tmp) == PDF_NAME );

    type = pdf_name_value(tmp);
    if (streq_ptr(type, "CIDFontType0"))
      font->subtype = PDF_FONT_FONTTYPE_CIDTYPE0;
    else if (streq_ptr(type, "CIDFontType2"))
      font->subtype = PDF_FONT_FONTTYPE_CIDTYPE2;
    else {
      _tt_abort("Unknown CIDFontType \"%s\"", type);
    }
  }

  if (opt_flags_cidfont & CIDFONT_FORCE_FIXEDPITCH) {
    if (pdf_lookup_dict(fontdict, "W")) {
       pdf_remove_dict(fontdict, "W");
    }
    if (pdf_lookup_dict(fontdict, "W2")) {
       pdf_remove_dict(fontdict, "W2");
    }
  }

  pdf_add_dict(fontdict,   pdf_new_name("Type"),     pdf_new_name("Font"));
  pdf_add_dict(fontdict,   pdf_new_name("BaseFont"), pdf_new_name(fontname));
  pdf_add_dict(descriptor, pdf_new_name("Type"),     pdf_new_name("FontDescriptor"));
  pdf_add_dict(descriptor, pdf_new_name("FontName"), pdf_new_name(fontname));

  font->resource   = fontdict;
  font->descriptor = descriptor;

  opt->embed = 0;

  return  0;
}



int
pdf_font_cidfont_lookup_cache (pdf_font *fonts, int count, const char *map_name,
                               CIDSysInfo *cmap_csi, const fontmap_opt *fmap_opt)
{
  int       font_id = -1;
  pdf_font *font    = NULL;
  cid_opt   opt;
  int       has_csi;

  assert(fonts);

  opt.style = fmap_opt->style;
  opt.embed = (fmap_opt->flags & FONTMAP_OPT_NOEMBED) ? 0 : 1;
  opt.csi.registry   = NULL;
  opt.csi.ordering   = NULL;
  opt.csi.supplement = 0;
  has_csi   = get_cidsysinfo(&opt.csi, map_name, fmap_opt);
  opt.stemv = fmap_opt->stemv;

  if (!has_csi && cmap_csi) {
    /*
     * No CIDSystemInfo supplied explicitly. Copy from CMap's one if available.
     * It is not neccesary for CID-keyed fonts. But TrueType requires them.
     */
    opt.csi.registry   = NEW(strlen(cmap_csi->registry)+1, char);
    strcpy(opt.csi.registry, cmap_csi->registry);
    opt.csi.ordering   = NEW(strlen(cmap_csi->ordering)+1, char);
    strcpy(opt.csi.ordering, cmap_csi->ordering);
    opt.csi.supplement = cmap_csi->supplement;
    has_csi = 1;
  }
  /*
   * Here, we do not compare font->ident and map_name because of
   * implicit CIDSystemInfo supplied by CMap for TrueType.
   */
  for (font_id = 0; font_id < count; font_id++) {
    font = &fonts[font_id];
    if (font->subtype != PDF_FONT_FONTTYPE_CIDTYPE0 &&
        font->subtype != PDF_FONT_FONTTYPE_CIDTYPE2)
      continue;
    if (!strcmp(font->filename, map_name) &&
        font->cid.options.style == opt.style &&
        font->index == fmap_opt->index) {
      if (font->cid.options.embed == opt.embed) {
        /*
         * Case 1: CSI not available (Identity CMap)
         *         Font is TrueType --> continue
         *         Font is CIDFont  --> break
         * Case 2: CSI matched      --> break
         */
        if (!has_csi) {
          if (font->subtype == PDF_FONT_FONTTYPE_CIDTYPE2)
            continue;
          else
            break;
        } else if (!strcmp(font->cid.csi.registry, opt.csi.registry) &&
                   !strcmp(font->cid.csi.ordering, opt.csi.ordering)) {
          if (font->subtype == PDF_FONT_FONTTYPE_CIDTYPE2)
            font->cid.csi.supplement =
              MAX(opt.csi.supplement, font->cid.csi.supplement); /* FIXME: font modified */
          break;
        }
      } else if (font->flags & PDF_FONT_FLAG_BASEFONT) {
        break;
      }
    }
  }
  release_opt(&opt);

  return (font_id < count) ? font_id : -1;
}

int
pdf_font_open_cidfont (pdf_font *font, const char *map_name, CIDSysInfo *cmap_csi, const fontmap_opt *fmap_opt)
{
  cid_opt opt;
  int     has_csi;

  opt.style = fmap_opt->style;
  opt.embed = (fmap_opt->flags & FONTMAP_OPT_NOEMBED) ? 0 : 1;
  opt.csi.registry   = NULL;
  opt.csi.ordering   = NULL;
  opt.csi.supplement = 0;
  has_csi   = get_cidsysinfo(&opt.csi, map_name, fmap_opt);
  opt.stemv = fmap_opt->stemv;

  if (!has_csi && cmap_csi) {
    /*
     * No CIDSystemInfo supplied explicitly. Copy from CMap's one if available.
     * It is not neccesary for CID-keyed fonts. But TrueType requires them.
     */
    opt.csi.registry   = NEW(strlen(cmap_csi->registry)+1, char);
    strcpy(opt.csi.registry, cmap_csi->registry);
    opt.csi.ordering   = NEW(strlen(cmap_csi->ordering)+1, char);
    strcpy(opt.csi.ordering, cmap_csi->ordering);
    opt.csi.supplement = cmap_csi->supplement;
    has_csi = 1;
  }

  if (CIDFont_type0_open(font, map_name, fmap_opt->index, &opt) < 0 &&
      CIDFont_type2_open(font, map_name, fmap_opt->index, &opt) < 0 &&
      CIDFont_type0_open_from_t1 (font, map_name, fmap_opt->index, &opt) < 0 &&
      CIDFont_type0_open_from_t1c(font, map_name, fmap_opt->index, &opt) < 0 &&
      CIDFont_base_open (font, map_name, &opt) < 0) {
    release_opt(&opt);
    return -1;
  }

  font->filename    = NEW(strlen(map_name)+1, char);
  strcpy(font->filename,  map_name);
  font->ident       = NEW(strlen(map_name)+1, char);
  strcpy(font->ident, map_name);
  font->index       = fmap_opt->index;
  font->cid.options = opt;

  if (font->cid.csi.registry && font->cid.csi.ordering) {
    if (cmap_csi) {
      if (strcmp(font->cid.csi.registry, cmap_csi->registry) ||
          strcmp(font->cid.csi.ordering, cmap_csi->ordering)) {
        dpx_warning("Inconsistent ROS found:\n");
        dpx_message("\tFont: %s-%s-%d\n", font->cid.csi.registry, font->cid.csi.ordering, font->cid.csi.supplement);
        dpx_message("\tCMap: %s-%s-%d\n", cmap_csi->registry, cmap_csi->ordering, cmap_csi->supplement);
        _tt_abort("Incompatible CMap specified for this font.");
      }
      if (font->cid.csi.supplement < cmap_csi->supplement) {
        font->cid.csi.supplement = cmap_csi->supplement;
      }
    }
  } else {
    assert(font->subtype == PDF_FONT_FONTTYPE_CIDTYPE2);
    if (cmap_csi) {
      font->cid.csi.registry   = NEW(strlen(cmap_csi->registry)+1, char);
      strcpy(font->cid.csi.registry, cmap_csi->registry);
      font->cid.csi.ordering   = NEW(strlen(cmap_csi->ordering)+1, char);
      strcpy(font->cid.csi.ordering, cmap_csi->ordering);
      font->cid.csi.supplement = cmap_csi->supplement;
    } else { /* This means font's internal glyph ordering. */
      font->cid.csi.registry   = NEW(strlen("Adobe")+1, char);
      strcpy(font->cid.csi.registry, "Adobe");
      font->cid.csi.ordering   = NEW(strlen("Identity")+1, char);
      strcpy(font->cid.csi.ordering, "Identity");
      font->cid.csi.supplement = 0;
    }
  }

  return 0;
}

/******************************* OPTIONS *******************************/

/*
 * FORMAT:
 *
 *   (:int:)?!?string(/string)?(,string)?
 */

static void
release_opt (cid_opt *opt)
{
  if (opt->csi.registry)
    free(opt->csi.registry);
  if (opt->csi.ordering)
    free(opt->csi.ordering);
}

static int
get_cidsysinfo (CIDSysInfo *csi, const char *map_name, const fontmap_opt *fmap_opt)
{
  int has_csi = 0;
  int sup_idx;
  int i, csi_idx = -1, m;
  size_t n;

  sup_idx = pdf_get_version() - 10;
  sup_idx = (sup_idx > SUP_IDX_MAX) ? SUP_IDX_MAX : sup_idx;

  if (!fmap_opt || !fmap_opt->charcoll)
    return 0;

  /* First try alias for standard one. */
  for (i = 0; CIDFont_stdcc_alias[i].name != NULL; i++) {
    n = strlen(CIDFont_stdcc_alias[i].name);
    if (strstartswith(fmap_opt->charcoll, CIDFont_stdcc_alias[i].name)) {
      csi_idx  = CIDFont_stdcc_alias[i].index;
      csi->registry = NEW(strlen(CIDFont_stdcc_def[csi_idx].registry)+1, char);
      strcpy(csi->registry, CIDFont_stdcc_def[csi_idx].registry);
      csi->ordering = NEW(strlen(CIDFont_stdcc_def[csi_idx].ordering)+1, char);
      strcpy(csi->ordering, CIDFont_stdcc_def[csi_idx].ordering);
      if (strlen(fmap_opt->charcoll) > n) {
        csi->supplement = (int) strtoul(&(fmap_opt->charcoll[n]), NULL, 10);
      } else { /* Use heighest supported value for current output PDF version. */
        csi->supplement = CIDFont_stdcc_def[csi_idx].supplement[sup_idx];
      }
      has_csi = 1;
      break;
    }
  }
  if (!has_csi) {
    char *p, *q;

    p   = (char *) fmap_opt->charcoll;

    /* Full REGISTRY-ORDERING-SUPPLEMENT */
    p = strchr(fmap_opt->charcoll, '-');
    if (!p || p[1] == '\0')
      _tt_abort("String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s",
            fmap_opt->charcoll);
    p++;

    q = strchr(p, '-');
    if (!q || q[1] == '\0')
      _tt_abort("String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s",
            fmap_opt->charcoll);
    q++;

    if (!isdigit((unsigned char)q[0]))
      _tt_abort("String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s",
            fmap_opt->charcoll);

    n = strlen(fmap_opt->charcoll) - strlen(p) - 1;
    csi->registry = NEW(n+1, char);
    memcpy(csi->registry, fmap_opt->charcoll, n);
    csi->registry[n] = '\0';

    m = strlen(p) - strlen(q) - 1;
    csi->ordering = NEW(m+1, char);
    memcpy(csi->ordering, p, m);
    csi->ordering[m] = '\0';

    csi->supplement = (int) strtoul(q, NULL, 10);

    has_csi = 1;

    /* Check for standart character collections. */
    for (i = 0; CIDFont_stdcc_def[i].ordering != NULL; i++) {
      if ((CIDFont_stdcc_def[i].registry &&
           streq_ptr(csi->registry, CIDFont_stdcc_def[i].registry)) &&
          streq_ptr(csi->ordering, CIDFont_stdcc_def[i].ordering)) {
        csi_idx = i;
        break;
      }
    }
  }

  if (csi && csi_idx >= 0) {
    if (csi->supplement > CIDFont_stdcc_def[csi_idx].supplement[sup_idx]
        && (fmap_opt->flags & FONTMAP_OPT_NOEMBED)) {
      dpx_warning("Highest supplement number supported in PDF-%d.%d for %s-%s is %d.",
           pdf_get_version_major(), pdf_get_version_minor(),
           csi->registry, csi->ordering,
           CIDFont_stdcc_def[csi_idx].supplement[sup_idx]);
      dpx_warning("Some character may not shown without embedded font (--> %s).",
           map_name);
    }
  }

  return has_csi;
}

void
CIDFont_set_flags (int flags)
{
  opt_flags_cidfont |= flags;
}
