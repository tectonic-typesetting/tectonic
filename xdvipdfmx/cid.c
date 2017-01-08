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
 * CID-keyed font support:
 *
 *  See also, cidtype0, and cidtype2
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"
#include "dpxutil.h"

#include "pdfobj.h"

#include "cidtype0.h"
#include "cidtype2.h"
#include "cid_p.h"
#include "cid.h"

#include "cff.h"

#define CIDFONT_DEBUG     3
#define CIDFONT_DEBUG_STR "CIDFont"

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
  int   supplement[16];
} CIDFont_stdcc_def[] = {
  {"Adobe", "UCS",      {-1, -1, 0, 0, 0, 0, 0, 0}}, 
  {"Adobe", "GB1",      {-1, -1, 0, 2, 4, 4, 4, 4}}, 
  {"Adobe", "CNS1",     {-1, -1, 0, 0, 3, 4, 4, 4}},
  {"Adobe", "Japan1",   {-1, -1, 2, 2, 4, 5, 6, 6}},
  {"Adobe", "Korea1",   {-1, -1, 1, 1, 2, 2, 2, 2}},
  {"Adobe", "Identity", {-1, -1, 0, 0, 0, 0, 0, 0}},
  {NULL,    NULL,       { 0,  0, 0, 0, 0, 0, 0, 0}}
};
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
static CIDSysInfo *get_cidsysinfo (const char *map_name, fontmap_opt *fmap_opt);

static int   __verbose   = 0;
static int   cidoptflags = 0;

void
CIDFont_set_verbose (void)
{
  CIDFont_type0_set_verbose();
  CIDFont_type2_set_verbose();
  __verbose++;
}

#if 0
int
CIDFont_require_version (void)
{
  return PDF_CID_SUPPORT_MIN;
}
#endif

static CIDFont *
CIDFont_new (void)
{
  CIDFont *font = NULL;

  font = NEW(1, struct CIDFont);

  font->name     = NULL;
  font->fontname = NULL;
  font->ident    = NULL;

  /*
   * CIDFont
   */
  font->subtype = -1;
  font->flags   = FONT_FLAG_NONE;
  font->csi     = NULL;
  font->options = NULL;
  (font->parent)[0] = -1; /* Horizontal */
  (font->parent)[1] = -1; /* Vertical   */

  /*
   * PDF Font Resource
   */
  font->indirect = NULL;
  font->fontdict = NULL;
  font->descriptor = NULL;

  return font;
}

/* It does write PDF objects. */
static void
CIDFont_flush (CIDFont *font)
{
  if (font) {
    if (font->indirect)   pdf_release_obj(font->indirect);
    font->indirect = NULL;
    if (font->fontdict)   pdf_release_obj(font->fontdict);
    font->fontdict = NULL;
    if (font->descriptor) pdf_release_obj(font->descriptor);
    font->descriptor = NULL;
  }
}

static void
CIDFont_release (CIDFont *font)
{
  if (font) {
    if (font->indirect)
      ERROR("%s: Object not flushed.", CIDFONT_DEBUG_STR);
    if (font->fontdict)
      ERROR("%s: Object not flushed.", CIDFONT_DEBUG_STR);
    if (font->descriptor)
      ERROR("%s: Object not flushed.", CIDFONT_DEBUG_STR);

    if (font->fontname) RELEASE(font->fontname);
    if (font->name)     RELEASE(font->name);
    if (font->ident)    RELEASE(font->ident);
    if (font->csi) {
      if (font->csi->registry)
        RELEASE(font->csi->registry);
      if (font->csi->ordering)
        RELEASE(font->csi->ordering);
      RELEASE(font->csi);
    }
    if (font->options)
      release_opt(font->options);
  }
}

char *
CIDFont_get_fontname (CIDFont *font)
{
  ASSERT(font);
  return font->fontname;
}

char *
CIDFont_get_ident (CIDFont *font)
{
  ASSERT(font);
  return font->ident;
}

int
CIDFont_get_opt_index (CIDFont *font)
{
  int  opt_index;

  ASSERT(font);

  if (font->options)
    opt_index = font->options->index;
  else {
    opt_index = 0;
  }

  return opt_index;
}

int
CIDFont_get_subtype (CIDFont *font)
{
  ASSERT(font);
  return font->subtype;
}

int
CIDFont_get_embedding (CIDFont *font)
{
  ASSERT(font);
  return font->options->embed;
}

CIDSysInfo *
CIDFont_get_CIDSysInfo (CIDFont *font)
{
  ASSERT(font);

  return font->csi;
}

/*
 * Returns ID of parent Type0 font
 *  wmode: 0 for horizontal, 1 for vertical
 */
int
CIDFont_get_parent_id (CIDFont *font, int wmode)
{
  ASSERT(font);

  if (wmode < 0 || wmode > 1)
    ERROR("%s: Invalid wmode value.", CIDFONT_DEBUG_STR);

  return (font->parent)[wmode];
}

pdf_obj *
CIDFont_get_resource (CIDFont *font)
{
  ASSERT(font);

  if (!font->indirect)
    font->indirect = pdf_ref_obj(font->fontdict);

  return pdf_link_obj(font->indirect);
}

/*
 * Set parent Type0 font.
 */
void
CIDFont_attach_parent (CIDFont *font, int parent_id, int wmode)
{
  ASSERT(font);

  if (wmode < 0 || wmode > 1)
    ERROR("%s: Invalid wmode value.", CIDFONT_DEBUG_STR);

  if (font->parent[wmode] >= 0)
    WARN("%s: CIDFont already have a parent Type1 font.", CIDFONT_DEBUG_STR);

  font->parent[wmode] = parent_id;
}

int
CIDFont_is_ACCFont (CIDFont *font)
{
  int   i;

  ASSERT(font);

  if (!font->csi)
    ERROR("%s: CIDSystemInfo undefined.", CIDFONT_DEBUG_STR);

  for (i = ACC_START; i <= ACC_END ; i++) {
    if (!strcmp(font->csi->registry, CIDFont_stdcc_def[i].registry) &&
        !strcmp(font->csi->ordering, CIDFont_stdcc_def[i].ordering))
      return 1;
  }

  return 0;
}

int
CIDFont_is_UCSFont (CIDFont *font)
{
  ASSERT(font);
  if (!strcmp(font->csi->ordering, "UCS") ||
      !strcmp(font->csi->ordering, "UCS2"))
    return 1;
  return 0;
}

/* FIXME */
int
CIDFont_get_flag (CIDFont *font, int mask)
{
  ASSERT(font);
  return ((font->flags & mask) ? 1 : 0);
}

static void
CIDFont_dofont (CIDFont *font)
{
  if (!font || !font->indirect)
    return;

  if (__verbose)
    MESG(":%s", font->ident);
  if (__verbose > 1) {
    if (font->fontname)
      MESG("[%s]", font->fontname);
  }

  switch (font->subtype) {
  case CIDFONT_TYPE0:
    if(__verbose)
      MESG("[CIDFontType0]");
    if (CIDFont_get_flag(font, CIDFONT_FLAG_TYPE1))
      CIDFont_type0_t1dofont(font);
    else if (CIDFont_get_flag(font, CIDFONT_FLAG_TYPE1C))
      CIDFont_type0_t1cdofont(font);
    else
      CIDFont_type0_dofont(font);
    break;
  case CIDFONT_TYPE2:
    if(__verbose)
      MESG("[CIDFontType2]");
    CIDFont_type2_dofont(font);
    break;
  default:
    ERROR("%s: Unknown CIDFontType %d.", CIDFONT_DEBUG_STR, font->subtype);
    break;
  }
}


/*
 *
 */
int
CIDFont_is_BaseFont (CIDFont *font)
{
  ASSERT(font);
  return (font->flags & FONT_FLAG_BASEFONT) ? 1 : 0;
}

#include "pdfparse.h"
#include "cid_basefont.h"

static int CIDFont_base_open (CIDFont *font,
                              const char *name, CIDSysInfo *cmap_csi, cid_opt *opt);

static int
CIDFont_base_open (CIDFont *font, const char *name, CIDSysInfo *cmap_csi, cid_opt *opt)
{
  pdf_obj *fontdict, *descriptor;
  char    *fontname = NULL;
  int      idx;

  ASSERT(font);

  for (idx = 0; cid_basefont[idx].fontname != NULL; idx++) {
    if (!strcmp(name, cid_basefont[idx].fontname) ||
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

    ASSERT(fontdict && descriptor);
  }

  font->fontname = fontname;
  font->flags   |= FONT_FLAG_BASEFONT;
  {
    char    *registry, *ordering;
    int      supplement;
    pdf_obj *tmp;

    tmp = pdf_lookup_dict(fontdict, "CIDSystemInfo");

    ASSERT( tmp && pdf_obj_typeof(tmp) == PDF_DICT );

    registry   = pdf_string_value(pdf_lookup_dict(tmp, "Registry"));
    ordering   = pdf_string_value(pdf_lookup_dict(tmp, "Ordering"));
    supplement = pdf_number_value(pdf_lookup_dict(tmp, "Supplement"));
    if (cmap_csi) { /* NULL for accept any */
      if (strcmp(registry, cmap_csi->registry) ||
          strcmp(ordering, cmap_csi->ordering))
        ERROR("Inconsistent CMap used for CID-keyed font %s.",
              cid_basefont[idx].fontname);
      else if (supplement < cmap_csi->supplement) {
        WARN("CMap has higher supplement number than CIDFont: %s", fontname);
        WARN("Some chracters may not be displayed or printed.");
      }
    }
    font->csi = NEW(1, CIDSysInfo);
    font->csi->registry = NEW(strlen(registry)+1, char);
    font->csi->ordering = NEW(strlen(ordering)+1, char);
    strcpy(font->csi->registry, registry);
    strcpy(font->csi->ordering, ordering);
    font->csi->supplement = supplement;
  }

  {
    pdf_obj *tmp;
    char    *type;

    tmp  = pdf_lookup_dict(fontdict, "Subtype");
    ASSERT( tmp != NULL && pdf_obj_typeof(tmp) == PDF_NAME );

    type = pdf_name_value(tmp);
    if (!strcmp(type, "CIDFontType0"))
      font->subtype = CIDFONT_TYPE0;
    else if (!strcmp(type, "CIDFontType2"))
      font->subtype = CIDFONT_TYPE2;
    else {
      ERROR("Unknown CIDFontType \"%s\"", type);
    }
  }

  if (cidoptflags & CIDFONT_FORCE_FIXEDPITCH) {
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

  font->fontdict   = fontdict;
  font->descriptor = descriptor;

  opt->embed = 0;

  return  0;
}



#define CACHE_ALLOC_SIZE  16u

struct FontCache {
  int       num;
  int       max;
  CIDFont **fonts;
};

static struct FontCache *__cache   = NULL;

#define CHECK_ID(n) do {\
                        if (! __cache)\
                           ERROR("%s: CIDFont cache not initialized.", CIDFONT_DEBUG_STR);\
                        if ((n) < 0 || (n) >= __cache->num)\
                           ERROR("%s: Invalid ID %d", CIDFONT_DEBUG_STR, (n));\
                    } while (0)

static void
CIDFont_cache_init (void)
{
  if (__cache)
    ERROR("%s: Already initialized.", CIDFONT_DEBUG_STR);

  __cache = NEW(1, struct FontCache);

  __cache->max  = CACHE_ALLOC_SIZE;
  __cache->fonts = NEW(__cache->max, struct CIDFont *);
  __cache->num  = 0;
}

CIDFont *
CIDFont_cache_get (int font_id)
{
  CHECK_ID(font_id);
  return __cache->fonts[font_id];
}

/*
 * cmap_csi is NULL if CMap is Identity.
 */
int
CIDFont_cache_find (const char *map_name,
                    CIDSysInfo *cmap_csi, fontmap_opt *fmap_opt)
{
  int      font_id = -1;
  CIDFont *font    = NULL;
  cid_opt *opt     = NULL;

  if (!__cache)
    CIDFont_cache_init();

  opt  = NEW(1, cid_opt);
  opt->style = fmap_opt->style;
  opt->index = fmap_opt->index;
  opt->embed = (fmap_opt->flags & FONTMAP_OPT_NOEMBED) ? 0 : 1;
  opt->name  = NULL;
  opt->csi   = get_cidsysinfo(map_name, fmap_opt);
  opt->stemv = fmap_opt->stemv;
  opt->cff_charsets = NULL;

  if (!opt->csi && cmap_csi) {
    /*
     * No CIDSystemInfo supplied explicitly. Copy from CMap's one if available.
     * It is not neccesary for CID-keyed fonts. But TrueType requires them.
     */
    opt->csi = NEW(1, CIDSysInfo);
    opt->csi->registry   = NEW(strlen(cmap_csi->registry)+1, char);
    strcpy(opt->csi->registry, cmap_csi->registry);
    opt->csi->ordering   = NEW(strlen(cmap_csi->ordering)+1, char);
    strcpy(opt->csi->ordering, cmap_csi->ordering);
    opt->csi->supplement = cmap_csi->supplement;
  }
  /*
   * Here, we do not compare font->ident and map_name because of
   * implicit CIDSystemInfo supplied by CMap for TrueType.
   */
  for (font_id = 0; font_id < __cache->num; font_id++) {
    font = __cache->fonts[font_id];
    if (!strcmp(font->name, map_name) &&
        font->options->style == opt->style &&
        font->options->index == opt->index) {
      if (font->options->embed == opt->embed) {
        /*
         * Case 1: CSI not available (Identity CMap)
         *         Font is TrueType --> continue
         *         Font is CIDFont  --> break
         * Case 2: CSI matched      --> break
         */
        if (!opt->csi) {
          if (font->subtype == CIDFONT_TYPE2)
            continue;
          else
            break;
        } else if (!strcmp(font->csi->registry, opt->csi->registry) &&
                   !strcmp(font->csi->ordering, opt->csi->ordering)) {
          if (font->subtype == CIDFONT_TYPE2)
            font->csi->supplement =
              MAX(opt->csi->supplement, font->csi->supplement);
          break;
        }
      } else if (CIDFont_is_BaseFont(font)) {
        opt->embed = 0;
        break;
      }
    }
  }

  if (font_id < __cache->num && cmap_csi) {
    if (strcmp(font->csi->registry, cmap_csi->registry) ||
        strcmp(font->csi->ordering, cmap_csi->ordering))
      ERROR("%s: Incompatible CMap for CIDFont \"%s\"",
            CIDFONT_DEBUG_STR, map_name);
  }

  if (font_id == __cache->num) {
    font = CIDFont_new();
    if (CIDFont_type0_open(font, map_name, cmap_csi, opt, 0) < 0 &&
        CIDFont_type2_open(font, map_name, cmap_csi, opt)    < 0 &&
        CIDFont_type0_open(font, map_name, cmap_csi, opt,
                           CIDFONT_FLAG_TYPE1)               < 0 &&
        CIDFont_type0_open(font, map_name, cmap_csi, opt,
                           CIDFONT_FLAG_TYPE1C)              < 0 &&
        CIDFont_base_open (font, map_name, cmap_csi, opt)    < 0) {
      CIDFont_release(font);
      release_opt(opt);
      return -1;
    } else {
      if (__cache->num >= __cache->max) {
        __cache->max  += CACHE_ALLOC_SIZE;
        __cache->fonts = RENEW(__cache->fonts,
                               __cache->max, struct CIDFont *);
      }
      font->name    = NEW(strlen(map_name)+1, char);
      strcpy(font->name,  map_name);
      font->ident   = NEW(strlen(map_name)+1, char);
      strcpy(font->ident, map_name);
      font->options = opt;
      __cache->fonts[font_id] = font;
      (__cache->num)++;

      fmap_opt->cff_charsets = opt->cff_charsets;
    }
  } else if (opt) {
    release_opt(opt);
  }

  return font_id;
}

void
CIDFont_cache_close (void)
{
  int  font_id;

  if (__cache) {
    for (font_id = 0;
         font_id < __cache->num; font_id++) {
      CIDFont *font;

      font = __cache->fonts[font_id];

      if (__verbose)
        MESG("(CID");

      CIDFont_dofont (font);
      CIDFont_flush  (font);
      CIDFont_release(font);

      RELEASE(font);

      if (__verbose)
        MESG(")");
    }
    RELEASE(__cache->fonts);
    RELEASE(__cache);
    __cache = NULL;
  }
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
  if (opt->csi) {
    if (opt->csi->registry)
      RELEASE(opt->csi->registry);
    if (opt->csi->ordering)
      RELEASE(opt->csi->ordering);
    RELEASE(opt->csi);
    if (opt->cff_charsets)
      cff_release_charsets((cff_charsets *) opt->cff_charsets);
  }
  RELEASE(opt);
}

static CIDSysInfo *
get_cidsysinfo (const char *map_name, fontmap_opt *fmap_opt)
{
  CIDSysInfo *csi = NULL;
  int pdf_ver;
  int i, csi_idx = -1, n, m;

  pdf_ver = pdf_get_version();

  if (!fmap_opt || !fmap_opt->charcoll)
    return NULL;

  /* First try alias for standard one. */
  for (i = 0; CIDFont_stdcc_alias[i].name != NULL; i++) {
    n = strlen(CIDFont_stdcc_alias[i].name);
    if (!strncmp(fmap_opt->charcoll,
                 CIDFont_stdcc_alias[i].name, n)) {
      csi_idx  = CIDFont_stdcc_alias[i].index;
      csi = NEW(1, CIDSysInfo);
      csi->registry = NEW(strlen(CIDFont_stdcc_def[csi_idx].registry)+1, char);
      strcpy(csi->registry, CIDFont_stdcc_def[csi_idx].registry);
      csi->ordering = NEW(strlen(CIDFont_stdcc_def[csi_idx].ordering)+1, char);
      strcpy(csi->ordering, CIDFont_stdcc_def[csi_idx].ordering);
      if (strlen(fmap_opt->charcoll) > n) {
        csi->supplement = (int) strtoul(&(fmap_opt->charcoll[n]), NULL, 10);
      } else { /* Use heighest supported value for current output PDF version. */
        csi->supplement = CIDFont_stdcc_def[csi_idx].supplement[pdf_ver];
      }
      break;
    }
  }
  if (csi == NULL) {
    char *p, *q;

    p   = (char *) fmap_opt->charcoll;
    csi = NEW(1, CIDSysInfo);

    /* Full REGISTRY-ORDERING-SUPPLEMENT */
    p = strchr(fmap_opt->charcoll, '-');
    if (!p || p[1] == '\0')
      ERROR("%s: String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s",
            CIDFONT_DEBUG_STR, fmap_opt->charcoll);
    p++;

    q = strchr(p, '-');
    if (!q || q[1] == '\0')
      ERROR("%s: String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s",
            CIDFONT_DEBUG_STR, fmap_opt->charcoll);
    q++;

    if (!isdigit((unsigned char)q[0]))
      ERROR("%s: String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s",
            CIDFONT_DEBUG_STR, fmap_opt->charcoll);

    n = strlen(fmap_opt->charcoll) - strlen(p) - 1;
    csi->registry = NEW(n+1, char);
    memcpy(csi->registry, fmap_opt->charcoll, n);
    csi->registry[n] = '\0';

    m = strlen(p) - strlen(q) - 1;
    csi->ordering = NEW(m+1, char);
    memcpy(csi->ordering, p, m);
    csi->ordering[m] = '\0';

    csi->supplement = (int) strtoul(q, NULL, 10);

    /* Check for standart character collections. */
    for (i = 0; CIDFont_stdcc_def[i].ordering != NULL; i++) {
      if ((CIDFont_stdcc_def[i].registry &&
           !strcmp(csi->registry, CIDFont_stdcc_def[i].registry)) &&
          !strcmp(csi->ordering, CIDFont_stdcc_def[i].ordering)) {
        csi_idx = i;
        break;
      }
    }
  }

  if (csi && csi_idx >= 0) {
    if (csi->supplement > CIDFont_stdcc_def[csi_idx].supplement[pdf_ver]
        && (fmap_opt->flags & FONTMAP_OPT_NOEMBED)) {
      WARN("%s: Heighest supplement number supported in PDF-1.%d for %s-%s is %d.",
           CIDFONT_DEBUG_STR, pdf_ver, csi->registry, csi->ordering,
           CIDFont_stdcc_def[csi_idx].supplement[pdf_ver]);
      WARN("%s: Some character may not shown without embedded font (--> %s).",
           CIDFONT_DEBUG_STR, map_name);
    }
  }

  return csi;
}

void
CIDFont_set_flags (int flags)
{
  CIDFont_type0_set_flags(flags);
  CIDFont_type2_set_flags(flags);
  cidoptflags |= flags;
}
