/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
 * CID-Keyed Font support:
 *
 *  Only CFF/OpenType CID-Keyed Font with Type 2 charstrings is supported.
 *
 */ 

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "system.h"
#include "numbers.h"
#include "mem.h"
#include "error.h"

#include "dpxfile.h"

#include "pdfobj.h"
/* pseudo unique tag */
#include "pdffont.h"

/* Font info. from OpenType tables */
#include "sfnt.h"
#include "tt_aux.h"
/* Metrics */
#include "tt_table.h"

#include "cff_types.h"
#include "cff_limits.h"
#include "cff.h"
#include "cff_dict.h"
#include "cs_type2.h"

/* typedef CID in cmap.h */
#include "cmap.h"
#include "type0.h"
#include "cid.h"
#include "cid_p.h"
#include "cidtype0.h"

/* Type1 --> CFF CIDFont */
#include "unicode.h"
#include "t1_load.h"
#include "t1_char.h"

#include "agl.h"

#include "cmap.h"
#include "cmap_write.h"
#include "fontmap.h"

static int  verbose   = 0;
static int  opt_flags = 0;

void
CIDFont_type0_set_verbose (void)
{
  verbose++;
}

void
CIDFont_type0_set_flags (int flags)
{
  opt_flags = flags;
}

/*
 * PDF Reference 3rd. ed., p.340, "Glyph Metrics in CID Fonts".
 */
#ifndef PDFUNIT
#define PDFUNIT(v) (ROUND((1000.0*(v))/(head->unitsPerEm),1))
#endif

static void
add_CIDHMetrics (pdf_obj *fontdict,
                 unsigned char *CIDToGIDMap, unsigned short last_cid,
                 struct tt_maxp_table *maxp,
                 struct tt_head_table *head, struct tt_longMetrics *hmtx)
{
  pdf_obj *w_array, *an_array = NULL;
  int    cid, start = 0, prev = 0;
  double defaultAdvanceWidth;
  int    empty = 1;

  defaultAdvanceWidth = PDFUNIT(hmtx[0].advance);
  /*
   * We alway use format:
   *  c [w_1 w_2 ... w_n]
   */
  w_array = pdf_new_array();
  for (cid = 0; cid <= last_cid; cid++) {
    USHORT gid;
    double advanceWidth;
    gid = CIDToGIDMap ? ((CIDToGIDMap[2*cid] << 8)|CIDToGIDMap[2*cid+1]) : cid;
    if (gid >= maxp->numGlyphs || (cid != 0 && gid == 0))
      continue;
    advanceWidth = PDFUNIT(hmtx[gid].advance);
    if (advanceWidth == defaultAdvanceWidth) {
      if (an_array) {
        pdf_add_array(w_array, pdf_new_number(start));
        pdf_add_array(w_array, an_array);
        an_array = NULL;
        empty = 0;
      }
    } else {
      if (cid != prev + 1 &&  an_array) {
          pdf_add_array(w_array, pdf_new_number(start));
          pdf_add_array(w_array, an_array);
          an_array = NULL;
          empty = 0;
      }
      if (an_array == NULL) {
        an_array = pdf_new_array();
        start = cid;
      }
      pdf_add_array(an_array, pdf_new_number(advanceWidth));
      prev = cid;
    }
  }

  if (an_array) {
    pdf_add_array(w_array, pdf_new_number(start));
    pdf_add_array(w_array, an_array);
    empty = 0;
  }

  /*
   * We always write DW for older MacOS X's preview app.
   * PDF Reference 2nd. ed, wrongly described default value of DW as 0, and
   * MacOS X's (up to 10.2.8) preview app. implements this wrong description.
   */
  pdf_add_dict(fontdict,
               pdf_new_name("DW"),
               pdf_new_number(defaultAdvanceWidth));
  if (!empty) {
    pdf_add_dict(fontdict,
                 pdf_new_name("W"),
                 pdf_ref_obj(w_array));
  }
  pdf_release_obj(w_array);

  return;
}

static void
add_CIDVMetrics (sfnt *sfont, pdf_obj *fontdict,
                 unsigned char *CIDToGIDMap, unsigned short last_cid,
                 struct tt_maxp_table *maxp,
                 struct tt_head_table *head, struct tt_longMetrics *hmtx)
{
  pdf_obj *w2_array, *an_array = NULL;
  int    cid;
#if 0
  int    prev = 0, start = 0;
#endif
  struct tt_VORG_table *vorg;
  struct tt_vhea_table *vhea  = NULL;
  struct tt_longMetrics *vmtx = NULL;
  double defaultAdvanceHeight, defaultVertOriginY;
  int    empty = 1;

  /*
   * No accurate vertical metrics can be obtained by simple way if the
   * font does not have VORG table. Only CJK fonts may have VORG.
   */
  if (sfnt_find_table_pos(sfont, "VORG") <= 0)
    return;

  vorg = tt_read_VORG_table(sfont);
  defaultVertOriginY = PDFUNIT(vorg->defaultVertOriginY);
  if (sfnt_find_table_pos(sfont, "vhea") > 0)
    vhea = tt_read_vhea_table(sfont);
  if (vhea && sfnt_find_table_pos(sfont, "vmtx") > 0) {
    sfnt_locate_table(sfont, "vmtx");
    vmtx = tt_read_longMetrics(sfont, maxp->numGlyphs, vhea->numOfLongVerMetrics, vhea->numOfExSideBearings);
  }

  if (sfnt_find_table_pos(sfont, "OS/2") <= 0) {
    struct tt_os2__table *os2;
    /* OpenType font must have OS/2 table. */
    os2 = tt_read_os2__table(sfont);
    defaultVertOriginY   = PDFUNIT(os2->sTypoAscender);
    defaultAdvanceHeight = PDFUNIT(os2->sTypoAscender - os2->sTypoDescender);
    RELEASE(os2);
  } else {
    /* Some TrueType fonts used in Macintosh does not have OS/2 table. */
    defaultAdvanceHeight = 1000;
  }

  w2_array = pdf_new_array();
  for (cid = 0; cid <= last_cid; cid++) {
    USHORT i, gid;
    double advanceHeight, vertOriginX, vertOriginY;
    gid = CIDToGIDMap ? ((CIDToGIDMap[2*cid] << 8)|CIDToGIDMap[2*cid+1]) : cid;
    if (gid >= maxp->numGlyphs || (cid != 0 && gid == 0))
      continue;
    advanceHeight = vmtx ? PDFUNIT(vmtx[gid].advance) : defaultAdvanceHeight;
    vertOriginX   = PDFUNIT(hmtx[gid].advance*0.5);
    vertOriginY   = defaultVertOriginY;
    for (i = 0;
         i < vorg->numVertOriginYMetrics && gid > vorg->vertOriginYMetrics[i].glyphIndex;
         i++) {
      if (gid == vorg->vertOriginYMetrics[i].glyphIndex)
        vertOriginY = PDFUNIT(vorg->vertOriginYMetrics[i].vertOriginY);
    }
#if 0
    /*
     * c [w1_1y v_1x v_1y w1_2y v_2x v_2y ...]
     * Not working... Why?
     * Acrobat Reader:
     *  Wrong rendering, interpretation of position vector is wrong.
     * Xpdf and gs: ignores W2?
     */
    if (vertOriginY == defaultVertOriginY &&
        advanceHeight == defaultAdvanceHeight) {
      if (an_array) {
        pdf_add_array(w2_array, pdf_new_number(start));
        pdf_add_array(w2_array, an_array);
        an_array = NULL;
        empty = 0;
      }
    } else {
      if (cid != prev + 1 && an_array) {
        pdf_add_array(w2_array, pdf_new_number(start));
        pdf_add_array(w2_array, an_array);
        an_array = NULL;
        empty = 0;
      }
      if (an_array == NULL) {
        an_array = pdf_new_array();
        start = cid;
      }
      pdf_add_array(an_array, pdf_new_number(-advanceHeight));
      pdf_add_array(an_array, pdf_new_number(vertOriginX));
      pdf_add_array(an_array, pdf_new_number(vertOriginY));
      prev = cid;
    }
#else
    /*
     * c_first c_last w1_y v_x v_y
     * This form may hit Acrobat's implementation limit of array element size, 8192.
     * AFPL GhostScript 8.11 stops with rangecheck error with this. Maybe GS's bug?
     */
    if (vertOriginY != defaultVertOriginY ||
        advanceHeight != defaultAdvanceHeight) {
      pdf_add_array(w2_array, pdf_new_number(cid));
      pdf_add_array(w2_array, pdf_new_number(cid));
      pdf_add_array(w2_array, pdf_new_number(-advanceHeight));
      pdf_add_array(w2_array, pdf_new_number(vertOriginX));
      pdf_add_array(w2_array, pdf_new_number(vertOriginY));
      empty = 0;
    }
#endif
  }

#if 0
  if (an_array) {
    pdf_add_array(w2_array, pdf_new_number(start));
    pdf_add_array(w2_array, an_array);
    empty = 0;
  }
#endif

  if (defaultVertOriginY != 880 || defaultAdvanceHeight != 1000) {
    an_array = pdf_new_array();
    pdf_add_array(an_array, pdf_new_number(defaultVertOriginY));
    pdf_add_array(an_array, pdf_new_number(-defaultAdvanceHeight));
    pdf_add_dict(fontdict, pdf_new_name ("DW2"), an_array);
  }
  if (!empty) {
    pdf_add_dict(fontdict,
                 pdf_new_name("W2"), pdf_ref_obj(w2_array));
  }
  pdf_release_obj(w2_array);

  if (vorg->vertOriginYMetrics)
    RELEASE(vorg->vertOriginYMetrics);
  RELEASE(vorg);

  if (vmtx)
    RELEASE(vmtx);
  if (vhea)
    RELEASE(vhea);

  return;
}

static void
add_CIDMetrics (sfnt *sfont, pdf_obj *fontdict,
                unsigned char *CIDToGIDMap, unsigned short last_cid, int need_vmetrics)
{
  struct tt_longMetrics *hmtx;
  struct tt_head_table  *head;
  struct tt_hhea_table  *hhea;
  struct tt_maxp_table  *maxp;

  /*
   * Read head, hhea, maxp:
   *
   *   unitsPerEm       --> head
   *   numHMetrics      --> hhea
   *   numGlyphs        --> maxp
   */
  head = tt_read_head_table(sfont);
  maxp = tt_read_maxp_table(sfont);
  hhea = tt_read_hhea_table(sfont);

  sfnt_locate_table(sfont, "hmtx");
  hmtx = tt_read_longMetrics(sfont, maxp->numGlyphs, hhea->numOfLongHorMetrics, hhea->numOfExSideBearings);

  add_CIDHMetrics(fontdict, CIDToGIDMap, last_cid, maxp, head, hmtx);
  if (need_vmetrics)
    add_CIDVMetrics(sfont, fontdict, CIDToGIDMap, last_cid, maxp, head, hmtx);

  RELEASE(hmtx);
  RELEASE(hhea);
  RELEASE(maxp);
  RELEASE(head);

  return;
}

/*
 * Create an instance of embeddable font.
 */
static int
write_fontfile (CIDFont *font, cff_font *cffont)
{
  cff_index *topdict, *fdarray, *private;
  unsigned char *dest;
  int destlen = 0, i, size;
  int offset, topdict_offset, fdarray_offset;

  /*  DICT sizes (offset set to long int) */
  topdict = cff_new_index(1);
  fdarray = cff_new_index(cffont->num_fds);
  private = cff_new_index(cffont->num_fds);

  cff_dict_remove(cffont->topdict, "UniqueID");
  cff_dict_remove(cffont->topdict, "XUID");
  cff_dict_remove(cffont->topdict, "Private");  /* some bad font may have */
  cff_dict_remove(cffont->topdict, "Encoding"); /* some bad font may have */

  topdict->offset[1] = cff_dict_pack(cffont->topdict,
                                     (card8 *) work_buffer,
                                     WORK_BUFFER_SIZE) + 1;
  for (i = 0;i < cffont->num_fds; i++) {
    size = 0;
    if (cffont->private && cffont->private[i]) {
      size = cff_dict_pack(cffont->private[i],
                           (card8 *) work_buffer, WORK_BUFFER_SIZE);
      if (size < 1) { /* Private had contained only Subr */
        cff_dict_remove(cffont->fdarray[i], "Private");
      }
    }
    (private->offset)[i+1] = (private->offset)[i] + size;
    (fdarray->offset)[i+1] = (fdarray->offset)[i] +
      cff_dict_pack(cffont->fdarray[i],
                    (card8 *) work_buffer, WORK_BUFFER_SIZE);
  }

  destlen = 4; /* header size */
  destlen += cff_set_name(cffont, font->fontname);
  destlen += cff_index_size(topdict);
  destlen += cff_index_size(cffont->string);
  destlen += cff_index_size(cffont->gsubr);
  destlen += (cffont->charsets->num_entries) * 2 + 1;  /* charset format 0 */
  destlen += (cffont->fdselect->num_entries) * 3 + 5; /* fdselect format 3 */
  destlen += cff_index_size(cffont->cstrings);
  destlen += cff_index_size(fdarray);
  destlen += private->offset[private->count] - 1; /* Private is not INDEX */

  dest = NEW(destlen, card8);

  offset = 0;
  /* Header */
  offset += cff_put_header(cffont, dest + offset, destlen - offset);
  /* Name */
  offset += cff_pack_index(cffont->name, dest + offset, destlen - offset);
  /* Top DICT */
  topdict_offset = offset;
  offset += cff_index_size(topdict);
  /* Strings */
  offset += cff_pack_index(cffont->string, dest + offset, destlen - offset);
  /* Global Subrs */
  offset += cff_pack_index(cffont->gsubr, dest + offset, destlen - offset);

  /* charset */
  cff_dict_set(cffont->topdict, "charset", 0, offset);
  offset += cff_pack_charsets(cffont, dest + offset, destlen - offset);

  /* FDSelect */
  cff_dict_set(cffont->topdict, "FDSelect", 0, offset);
  offset += cff_pack_fdselect(cffont, dest + offset, destlen - offset);

  /* CharStrings */
  cff_dict_set(cffont->topdict, "CharStrings", 0, offset);
  offset += cff_pack_index(cffont->cstrings,
                           dest + offset, cff_index_size(cffont->cstrings));
  cff_release_index(cffont->cstrings);
  cffont->cstrings = NULL; /* Charstrings cosumes huge memory */

  /* FDArray and Private */
  cff_dict_set(cffont->topdict, "FDArray", 0, offset);
  fdarray_offset = offset;
  offset += cff_index_size(fdarray);

  fdarray->data = NEW(fdarray->offset[fdarray->count] - 1, card8);
  for (i = 0; i < cffont->num_fds; i++) {
    size = private->offset[i+1] - private->offset[i];
    if (cffont->private[i] && size > 0) {
      cff_dict_pack(cffont->private[i], dest + offset, size);
      cff_dict_set(cffont->fdarray[i], "Private", 0, size);
      cff_dict_set(cffont->fdarray[i], "Private", 1, offset);
    }
    cff_dict_pack(cffont->fdarray[i],
                  fdarray->data + (fdarray->offset)[i] - 1,
                  fdarray->offset[fdarray->count] - 1);
    offset += size;
  }

  cff_pack_index(fdarray, dest + fdarray_offset, cff_index_size(fdarray));
  cff_release_index(fdarray);
  cff_release_index(private);

  /* Finally Top DICT */
  topdict->data = NEW(topdict->offset[topdict->count] - 1, card8);
  cff_dict_pack(cffont->topdict,
                topdict->data, topdict->offset[topdict->count] - 1);
  cff_pack_index(topdict, dest + topdict_offset, cff_index_size(topdict));
  cff_release_index(topdict);

  /*
   * FontFile
   */
  {
    pdf_obj *fontfile, *stream_dict;

    fontfile    = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(fontfile);
    pdf_add_dict(font->descriptor,
                 pdf_new_name("FontFile3"),
                 pdf_ref_obj (fontfile));
    pdf_add_dict(stream_dict,
                 pdf_new_name("Subtype"),
                 pdf_new_name("CIDFontType0C"));
    pdf_add_stream(fontfile, (char *) dest, offset);
    pdf_release_obj(fontfile);
    RELEASE(dest);
  }

  return destlen;
}

static char *
CIDFont_type0_get_used_chars(CIDFont *font) {
  int parent_id;
  char *used_chars;

  if ((parent_id = CIDFont_get_parent_id(font, 0)) < 0 &&
      (parent_id = CIDFont_get_parent_id(font, 1)) < 0)
    ERROR("No parent Type 0 font !");

  used_chars = Type0Font_get_usedchars(Type0Font_cache_get(parent_id));
  if (!used_chars)
    ERROR("Unexpected error: Font not actually used???");

  return used_chars;
}

typedef struct
{
  FILE      *fp;
  sfnt      *sfont;
  cff_font  *cffont;
} CIDType0Info;

typedef enum {
  CID_OPEN_ERROR_NO_ERROR             = 0,
  CID_OPEN_ERROR_CANNOT_OPEN_FILE     = -1,
  CID_OPEN_ERROR_NOT_SFNT_FONT        = -2,
  CID_OPEN_ERROR_NO_CFF_TABLE         = -3,
  CID_OPEN_ERROR_CANNOT_OPEN_CFF_FONT = -4,
  CID_OPEN_ERROR_NOT_CIDFONT          = -5,
  CID_OPEN_ERROR_IS_CIDFONT           = -6,
} CIDType0Error;

static void
CIDType0Error_Show (CIDType0Error error, const char *name)
{
  switch (error) {
    case CID_OPEN_ERROR_CANNOT_OPEN_FILE:
      ERROR("Could not open OpenType font file: %s", name);
    case CID_OPEN_ERROR_NOT_SFNT_FONT:
      ERROR("Could not open SFNT font file: %s", name);
    case CID_OPEN_ERROR_NO_CFF_TABLE:
      ERROR("Not a CFF/OpenType font: %s", name);
    case CID_OPEN_ERROR_CANNOT_OPEN_CFF_FONT:
      ERROR("Could not open CFF font: %s", name);
    case CID_OPEN_ERROR_NOT_CIDFONT:
      ERROR("Not a CIDFont: %s", name);
    case CID_OPEN_ERROR_IS_CIDFONT:
      ERROR("Should not be a CIDFont: %s", name);
    default:
      break;
  }
}

static void
CIDFontInfo_init (CIDType0Info *info)
{
  memset(info, 0, sizeof(CIDType0Info));
}

static void
CIDFontInfo_close (CIDType0Info *info)
{
  if (info->cffont)
    cff_close(info->cffont);

  if (info->sfont)
    sfnt_close(info->sfont);

  if (info->fp)
    DPXFCLOSE(info->fp);

  CIDFontInfo_init(info);
}

static CIDType0Error
CIDFont_type0_try_open (const char *name,
                        int index,
                        int required_cid,
                        CIDType0Info *info)
{
  ULONG offset = 0;
  int is_cid;

  CIDFontInfo_init(info);

  info->fp = DPXFOPEN(name, DPX_RES_TYPE_OTFONT);
  if (!info->fp) {
    info->fp = DPXFOPEN(name, DPX_RES_TYPE_TTFONT);
    if (!info->fp)
      return CID_OPEN_ERROR_CANNOT_OPEN_FILE;
  }

  info->sfont = sfnt_open(info->fp);
  if (!info->sfont)
    return CID_OPEN_ERROR_NOT_SFNT_FONT;

  if (info->sfont->type == SFNT_TYPE_TTC)
    offset = ttc_read_offset(info->sfont, index);

  if ((info->sfont->type != SFNT_TYPE_TTC &&
       info->sfont->type != SFNT_TYPE_POSTSCRIPT) ||
      sfnt_read_table_directory(info->sfont, offset) < 0 ||
      (offset = sfnt_find_table_pos(info->sfont, "CFF ")) == 0) {
    CIDFontInfo_close(info);
    return CID_OPEN_ERROR_NO_CFF_TABLE;
  }

  info->cffont = cff_open(info->sfont->stream, offset, 0);
  if (!info->cffont)
    return CID_OPEN_ERROR_CANNOT_OPEN_CFF_FONT;

  is_cid = info->cffont->flag & FONTTYPE_CIDFONT;
  if (required_cid != is_cid) {
    CIDFontInfo_close(info);
    return required_cid ? CID_OPEN_ERROR_NOT_CIDFONT
                        : CID_OPEN_ERROR_IS_CIDFONT;
  }

  return CID_OPEN_ERROR_NO_ERROR;
}

static void
CIDFont_type0_add_CIDSet(CIDFont *font, char *used_chars, card16 last_cid) {
  /*
   * CIDSet:
   * Length of CIDSet stream is not clear. Must be 8192 bytes long?
   */
  pdf_obj *cidset;

  cidset = pdf_new_stream(STREAM_COMPRESS);
  pdf_add_stream(cidset, used_chars, (last_cid / 8) + 1);
  pdf_add_dict(font->descriptor,
               pdf_new_name("CIDSet"), pdf_ref_obj(cidset));
  pdf_release_obj(cidset);
}

void
CIDFont_type0_dofont (CIDFont *font)
{
  cff_font *cffont;
  cff_index    *charstrings, *idx;
  cff_charsets *charset = NULL;
  cff_fdselect *fdselect = NULL;
  int    charstring_len, max_len;
  int    destlen = 0;
  int    size, offset = 0;
  card8 *data;
  card16 num_glyphs = 0, gid;
  int    cid;
  card16 cs_count, last_cid = 0;
  int    fd, prev_fd;
  char  *used_chars;
  unsigned char *CIDToGIDMap = NULL;
  CIDType0Error error;
  CIDType0Info info;

  ASSERT(font);

  if (!font->indirect)
    return;

  pdf_add_dict(font->fontdict, 
               pdf_new_name("FontDescriptor"),
               pdf_ref_obj (font->descriptor));

  if (CIDFont_is_BaseFont(font))
    return;
  else if (!CIDFont_get_embedding(font) &&
           (opt_flags & CIDFONT_FORCE_FIXEDPITCH)) {
    /* No metrics needed. */
    pdf_add_dict(font->fontdict,
                 pdf_new_name("DW"), pdf_new_number(1000.0));
    return;
  }

  used_chars = CIDFont_type0_get_used_chars(font);

  error = CIDFont_type0_try_open(font->ident, CIDFont_get_opt_index(font),
                                 1, &info);
  if (error != CID_OPEN_ERROR_NO_ERROR) {
    CIDType0Error_Show(error, font->ident);
    return;
  }

  cffont = info.cffont;

  cff_read_charsets(cffont);

  /*
   * DW, W, DW2 and W2:
   * Those values are obtained from OpenType table (not TFM).
   */
  if (opt_flags & CIDFONT_FORCE_FIXEDPITCH) {
    pdf_add_dict(font->fontdict,
                 pdf_new_name("DW"), pdf_new_number(1000.0));
  } else {
    int cid_count;

    if (cff_dict_known(cffont->topdict, "CIDCount")) {
      cid_count = (int) cff_dict_get(cffont->topdict, "CIDCount", 0);
    } else {
      cid_count = CID_MAX + 1;
    }

    CIDToGIDMap = NEW(2 * cid_count, unsigned char);
    memset(CIDToGIDMap, 0, 2 * cid_count);
    add_to_used_chars2(used_chars, 0); /* .notdef */
    for (cid = 0; cid <= CID_MAX; cid++) {
      if (is_used_char2(used_chars, cid)) {
        gid = cff_charsets_lookup(cffont, (card16)cid);
        if (cid != 0 && gid == 0) {
          WARN("Glyph for CID %u missing in font \"%s\".", (CID) cid, font->ident);
          used_chars[cid/8] &= ~(1 << (7 - (cid % 8)));
          continue;
        }
        CIDToGIDMap[2*cid]   = (gid >> 8) & 0xff;
        CIDToGIDMap[2*cid+1] = gid & 0xff;
        last_cid = cid;
        num_glyphs++;
      }
    }

    add_CIDMetrics(info.sfont, font->fontdict, CIDToGIDMap, last_cid,
                   ((CIDFont_get_parent_id(font, 1) < 0) ? 0 : 1));
  }

  if (!CIDFont_get_embedding(font)) {
    RELEASE(CIDToGIDMap);
    CIDFontInfo_close(&info);

    return;
  }

  /*
   * Embed font subset.
   */
  cff_read_fdselect(cffont);
  cff_read_fdarray(cffont);
  cff_read_private(cffont);

  cff_read_subrs(cffont);

  offset = (int) cff_dict_get(cffont->topdict, "CharStrings", 0);
  cff_seek_set(cffont, offset);
  idx = cff_get_index_header(cffont);
  /* offset is now absolute offset ... bad */
  offset = cff_tell(cffont);
  
  if ((cs_count = idx->count) < 2) {
    ERROR("No valid charstring data found.");
  }

  /* New Charsets data */
  charset = NEW(1, cff_charsets);
  charset->format = 0;
  charset->num_entries = 0;
  charset->data.glyphs = NEW(num_glyphs, s_SID);

  /* New FDSelect data */
  fdselect = NEW(1, cff_fdselect);
  fdselect->format = 3;
  fdselect->num_entries = 0;
  fdselect->data.ranges = NEW(num_glyphs, cff_range3);

  /* New CharStrings INDEX */
  charstrings = cff_new_index((card16)(num_glyphs+1));
  max_len = 2 * CS_STR_LEN_MAX;
  charstrings->data = NEW(max_len, card8);
  charstring_len = 0;

  /*
   * TODO: Re-assign FD number.
   */
  prev_fd = -1; gid = 0;
  data = NEW(CS_STR_LEN_MAX, card8);
  for (cid = 0; cid <= last_cid; cid++) {
    unsigned short gid_org;

    if (!is_used_char2(used_chars, cid))
      continue;

    gid_org = (CIDToGIDMap[2*cid] << 8)|(CIDToGIDMap[2*cid+1]);
    if ((size = (idx->offset)[gid_org+1] - (idx->offset)[gid_org])
        > CS_STR_LEN_MAX)
      ERROR("Charstring too long: gid=%u", gid_org);
    if (charstring_len + CS_STR_LEN_MAX >= max_len) {
      max_len = charstring_len + 2 * CS_STR_LEN_MAX;
      charstrings->data = RENEW(charstrings->data, max_len, card8);
    }
    (charstrings->offset)[gid] = charstring_len + 1;
    cff_seek(cffont, offset + (idx->offset)[gid_org] - 1);
    cff_read_data(data, size, cffont);
    fd = cff_fdselect_lookup(cffont, gid_org);
    charstring_len += cs_copy_charstring(charstrings->data + charstring_len,
                                         max_len - charstring_len,
                                         data, size,
                                         cffont->gsubr, (cffont->subrs)[fd], 0, 0, NULL);
    if (cid > 0 && gid_org > 0) {
      charset->data.glyphs[charset->num_entries] = cid;
      charset->num_entries += 1;
    }
    if (fd != prev_fd) {
      fdselect->data.ranges[fdselect->num_entries].first = gid;
      fdselect->data.ranges[fdselect->num_entries].fd    = fd;
      fdselect->num_entries += 1;
      prev_fd = fd;
    }
    gid++;
  }
  if (gid != num_glyphs)
    ERROR("Unexpeced error: ?????");
  RELEASE(data);
  cff_release_index(idx);

  RELEASE(CIDToGIDMap);
  
  (charstrings->offset)[num_glyphs] = charstring_len + 1;
  charstrings->count = num_glyphs;
  cffont->num_glyphs    = num_glyphs;
  cffont->cstrings      = charstrings;
  
  /* discard old one, set new data */
  cff_release_charsets(cffont->charsets);
  cffont->charsets = charset;
  cff_release_fdselect(cffont->fdselect);
  cffont->fdselect = fdselect;

  /* no Global subr */
  if (cffont->gsubr)
    cff_release_index(cffont->gsubr);
  cffont->gsubr = cff_new_index(0);

  for (fd = 0; fd < cffont->num_fds; fd++) {
    if (cffont->subrs && cffont->subrs[fd]) {
      cff_release_index(cffont->subrs[fd]);
      cffont->subrs[fd] = NULL;
    }
    if (cffont->private && (cffont->private)[fd]) {
      cff_dict_remove((cffont->private)[fd], "Subrs"); /* no Subrs */
    }
  }

  destlen = write_fontfile(font, cffont);

  CIDFontInfo_close(&info);

  if (verbose > 1)
    MESG("[%u/%u glyphs][%ld bytes]", num_glyphs, cs_count, destlen);

  CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}

int
CIDFont_type0_open (CIDFont *font, const char *name,
                    CIDSysInfo *cmap_csi, cid_opt *opt,
                    int expected_flag)
{
  CIDSysInfo *csi;
  char       *fontname;
  sfnt       *sfont = NULL;
  cff_font   *cffont;
  FILE       *fp = NULL;
  ULONG       offset = 0;
  int         is_cid_font = 0;
  int         expect_cid_font = expected_flag == 0;
  int         expect_type1_font = expected_flag & CIDFONT_FLAG_TYPE1;
  dpx_res_type expect_file_type =
      expect_type1_font ? DPX_RES_TYPE_T1FONT : DPX_RES_TYPE_OTFONT;

  ASSERT(font);

  if (expect_type1_font) {
    if (cmap_csi &&
        (strcmp(cmap_csi->registry, "Adobe")    != 0 ||
         strcmp(cmap_csi->ordering, "Identity") != 0)) {
      return -1;
    }
  }

  fp = DPXFOPEN(name, expect_file_type);
  if (!expect_type1_font) {
    if (!fp) {
      fp = DPXFOPEN(name, DPX_RES_TYPE_TTFONT);
      if (!fp) return -1;
    }

    sfont = sfnt_open(fp);
    if (!sfont) {
      ERROR("Not a CFF/OpenType font: %s", name);
    }

    if (sfont->type == SFNT_TYPE_TTC)
      offset = ttc_read_offset(sfont, opt->index);

    if ((sfont->type != SFNT_TYPE_TTC && sfont->type != SFNT_TYPE_POSTSCRIPT) ||
        sfnt_read_table_directory(sfont, offset) < 0 ||
        (offset = sfnt_find_table_pos(sfont, "CFF ")) == 0) {
      sfnt_close(sfont);
      if (fp)
        DPXFCLOSE(fp);
      return -1;
    }

    cffont = cff_open(sfont->stream, offset, 0);
    if (!cffont) {
      ERROR("Cannot read CFF font data");
    }

    is_cid_font = cffont->flag & FONTTYPE_CIDFONT;
    if (expect_cid_font != is_cid_font) {
      cff_close(cffont);
      sfnt_close(sfont);
      if (fp)
        DPXFCLOSE(fp);
      return -1;
    }

    if (is_cid_font) {
      cff_read_charsets(cffont);
      opt->cff_charsets = cffont->charsets;
      cffont->charsets = NULL;
    }
  } else {
    if (!fp)
      return -1;

    cffont = t1_load_font(NULL, 1, fp);
    if (!cffont) {
      DPXFCLOSE(fp);
      return -1;
    }
    DPXFCLOSE(fp);
  }

  csi = NEW(1, CIDSysInfo);
  if (is_cid_font) {
    csi->registry =
      cff_get_string(cffont, (s_SID)cff_dict_get(cffont->topdict, "ROS", 0));
    csi->ordering =
      cff_get_string(cffont, (s_SID)cff_dict_get(cffont->topdict, "ROS", 1));
    csi->supplement = (int)cff_dict_get(cffont->topdict, "ROS", 2);
  } else {
    csi->registry   = NEW(strlen("Adobe") + 1, char);
    strcpy(csi->registry, "Adobe");
    csi->ordering   = NEW(strlen("Identity") + 1, char);
    strcpy(csi->ordering, "Identity");
    csi->supplement = 0;
  }

  if (!expect_type1_font && cmap_csi) {
    if (strcmp(csi->registry, cmap_csi->registry) != 0 ||
        strcmp(csi->ordering, cmap_csi->ordering) != 0) {
      MESG("\nCharacter collection mismatched:\n");
      MESG("\tFont: %s-%s-%d\n", csi->registry, csi->ordering, csi->supplement);
      MESG("\tCMap: %s-%s-%d\n", cmap_csi->registry, cmap_csi->ordering, cmap_csi->supplement);
      ERROR("Inconsistent CMap specified for this font.");
    }
    if (csi->supplement < cmap_csi->supplement) {
      WARN("CMap have higher supplmement number.");
      WARN("Some characters may not be displayed or printed.");
    }
  }

  {
    char *shortname;
    int fontname_len = 8;

    shortname = cff_get_name(cffont);
    if (!shortname)
      ERROR("No valid FontName found.");
    /*
     * Mangled name requires more 7 bytes.
     * Style requires more 11 bytes.
     */
     if (is_cid_font)
      fontname_len += 11;
    fontname = NEW(strlen(shortname) + fontname_len, char);
    memset(fontname, 0, strlen(shortname) + fontname_len);
    strcpy(fontname, shortname);
    RELEASE(shortname);
  }
  cff_close(cffont);

  if (is_cid_font) {
    if (opt->embed && opt->style != FONT_STYLE_NONE) {
      WARN("Embedding disabled due to style option for %s.", name);
      opt->embed = 0;
    }
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
  } else if (expect_type1_font) {
    if (opt->style != FONT_STYLE_NONE) {
      WARN(",Bold, ,Italic, ... not supported for this type of font...");
      opt->style = FONT_STYLE_NONE;
    }
  } else {
    opt->embed = 1;
  }

  font->fontname = fontname;
  font->subtype  = CIDFONT_TYPE0;
  font->csi      = csi;
  font->flags   |= expected_flag;

  font->fontdict = pdf_new_dict();
  pdf_add_dict(font->fontdict,
               pdf_new_name("Type"),
               pdf_new_name("Font"));
  pdf_add_dict(font->fontdict,
               pdf_new_name("Subtype"),
               pdf_new_name("CIDFontType0"));

  if (expect_type1_font || opt->embed) {
    memmove(fontname + 7, fontname, strlen(fontname) + 1);
    pdf_font_make_uniqueTag(fontname); 
    fontname[6] = '+';
  }

  if (expect_type1_font) {
    font->descriptor = pdf_new_dict();
  } else {
    /* getting font info. from TrueType tables */
    if ((font->descriptor
         = tt_get_fontdesc(sfont, &(opt->embed), opt->stemv, 0, name)) == NULL)
      ERROR("Could not obtain necessary font info.");
  }

  pdf_add_dict(font->descriptor,
               pdf_new_name("FontName"),
               pdf_new_name(fontname));
  pdf_add_dict(font->fontdict, 
               pdf_new_name("BaseFont"),
               pdf_new_name(fontname));
  {
    pdf_obj *csi_dict = pdf_new_dict();
    pdf_add_dict(csi_dict,
                 pdf_new_name("Registry"),
                 pdf_new_string(csi->registry, strlen(csi->registry)));
    pdf_add_dict(csi_dict,
                 pdf_new_name("Ordering"),
                 pdf_new_string(csi->ordering, strlen(csi->ordering)));
    pdf_add_dict(csi_dict,
                 pdf_new_name("Supplement"),
                 pdf_new_number(csi->supplement));
    pdf_add_dict(font->fontdict, pdf_new_name("CIDSystemInfo"), csi_dict);
  }
  if (is_cid_font) {
    pdf_add_dict(font->fontdict,
                 pdf_new_name("DW"),
                 pdf_new_number(1000)); /* not sure */
  }

  if (!expect_type1_font) {
    sfnt_close(sfont);
    if (fp)
      DPXFCLOSE(fp);
  }

  return 0;
}

void
CIDFont_type0_t1cdofont (CIDFont *font)
{
  cff_font  *cffont;
  cff_index *charstrings, *idx;
  int    charstring_len, max_len;
  int    destlen = 0;
  int    size, offset = 0;
  card8 *data;
  card16 num_glyphs, gid, last_cid;
  int    i, cid;
  char  *used_chars;
  double default_width, nominal_width;
  CIDType0Error error;
  CIDType0Info info;

  ASSERT(font);

  if (!font->indirect)
    return;

  pdf_add_dict(font->fontdict, 
               pdf_new_name("FontDescriptor"),
               pdf_ref_obj (font->descriptor));

  used_chars = CIDFont_type0_get_used_chars(font);

  error = CIDFont_type0_try_open(font->ident, CIDFont_get_opt_index(font),
                                 0, &info);
  if (error != CID_OPEN_ERROR_NO_ERROR) {
    CIDType0Error_Show(error, font->ident);
    return;
  }

  cffont = info.cffont;

  cff_read_private(cffont);
  cff_read_subrs  (cffont);

  if (cffont->private[0] && cff_dict_known(cffont->private[0], "StdVW")) {
    double stemv;
    stemv = cff_dict_get(cffont->private[0], "StdVW", 0);
    pdf_add_dict(font->descriptor,
                 pdf_new_name("StemV"), pdf_new_number(stemv));
  }
  if (cffont->private[0] && cff_dict_known(cffont->private[0], "defaultWidthX")) {
    default_width = (double) cff_dict_get(cffont->private[0], "defaultWidthX", 0);
  } else {
    default_width = CFF_DEFAULTWIDTHX_DEFAULT;
  }
  if (cffont->private[0] && cff_dict_known(cffont->private[0], "nominalWidthX")) {
    nominal_width = (double) cff_dict_get(cffont->private[0], "nominalWidthX", 0);
  } else {
    nominal_width = CFF_NOMINALWIDTHX_DEFAULT;
  }

  num_glyphs = 0; last_cid = 0;
  add_to_used_chars2(used_chars, 0); /* .notdef */
  for (i = 0; i < (cffont->num_glyphs + 7)/8; i++) {
    int c, j;

    c = used_chars[i];
    for (j = 7; j >= 0; j--) {
      if (c & (1 << j)) {
        num_glyphs++;
        last_cid = (i + 1) * 8 - j - 1;
      }
    }
  }

  {
    cff_fdselect *fdselect;

    fdselect = NEW(1, cff_fdselect);
    fdselect->format = 3;
    fdselect->num_entries = 1;
    fdselect->data.ranges = NEW(1, cff_range3);
    fdselect->data.ranges[0].first = 0;
    fdselect->data.ranges[0].fd    = 0;
    cffont->fdselect = fdselect;
  }

  {
    cff_charsets *charset;

    charset  = NEW(1, cff_charsets);
    charset->format = 0;
    charset->num_entries = num_glyphs-1;
    charset->data.glyphs = NEW(num_glyphs-1, s_SID);

    for (gid = 0, cid = 0; cid <= last_cid; cid++) {
      if (is_used_char2(used_chars, cid)) {
        if (gid > 0)
          charset->data.glyphs[gid-1] = cid;
        gid++;
      }
    }
    /* cff_release_charsets(cffont->charsets); */
    cffont->charsets = charset;
  }

  cff_dict_add(cffont->topdict, "CIDCount", 1);
  cff_dict_set(cffont->topdict, "CIDCount", 0, last_cid + 1);

  cffont->fdarray    = NEW(1, cff_dict *);
  cffont->fdarray[0] = cff_new_dict();
  cff_dict_add(cffont->fdarray[0], "FontName", 1);
  cff_dict_set(cffont->fdarray[0], "FontName", 0,
               (double) cff_add_string(cffont, font->fontname + 7, 1)); /* FIXME: Skip XXXXXX+ */
  cff_dict_add(cffont->fdarray[0], "Private", 2);
  cff_dict_set(cffont->fdarray[0], "Private", 0, 0.0);
  cff_dict_set(cffont->fdarray[0], "Private", 0, 0.0);
  /* FDArray  - index offset, not known yet */
  cff_dict_add(cffont->topdict, "FDArray", 1);
  cff_dict_set(cffont->topdict, "FDArray", 0, 0.0);
  /* FDSelect - offset, not known yet */
  cff_dict_add(cffont->topdict, "FDSelect", 1);
  cff_dict_set(cffont->topdict, "FDSelect", 0, 0.0);

  cff_dict_remove(cffont->topdict, "UniqueID");
  cff_dict_remove(cffont->topdict, "XUID");
  cff_dict_remove(cffont->topdict, "Private");
  cff_dict_remove(cffont->topdict, "Encoding");


  /* */
  offset = (int) cff_dict_get(cffont->topdict, "CharStrings", 0);
  cff_seek_set(cffont, offset);
  idx = cff_get_index_header(cffont);
  /* offset is now absolute offset ... bad */
  offset = cff_tell(cffont);

  if (idx->count < 2)
    ERROR("No valid charstring data found.");

  /* New CharStrings INDEX */
  charstrings = cff_new_index((card16)(num_glyphs+1));
  max_len = 2 * CS_STR_LEN_MAX;
  charstrings->data = NEW(max_len, card8);
  charstring_len = 0;

  gid  = 0;
  data = NEW(CS_STR_LEN_MAX, card8);
  for (cid = 0; cid <= last_cid; cid++) {
    if (!is_used_char2(used_chars, cid))
      continue;

    if ((size = (idx->offset)[cid+1] - (idx->offset)[cid])
        > CS_STR_LEN_MAX)
      ERROR("Charstring too long: gid=%u", cid);
    if (charstring_len + CS_STR_LEN_MAX >= max_len) {
      max_len = charstring_len + 2 * CS_STR_LEN_MAX;
      charstrings->data = RENEW(charstrings->data, max_len, card8);
    }
    (charstrings->offset)[gid] = charstring_len + 1;
    cff_seek(cffont, offset + (idx->offset)[cid] - 1);
    cff_read_data(data, size, cffont);
    charstring_len += cs_copy_charstring(charstrings->data + charstring_len,
                                         max_len - charstring_len,
                                         data, size,
                                         cffont->gsubr, (cffont->subrs)[0],
                                         default_width, nominal_width, NULL);
    gid++;
  }
  if (gid != num_glyphs)
    ERROR("Unexpeced error: ?????");
  RELEASE(data);
  cff_release_index(idx);

  (charstrings->offset)[num_glyphs] = charstring_len + 1;
  charstrings->count = num_glyphs;
  cffont->num_glyphs    = num_glyphs;
  cffont->cstrings      = charstrings;
  
  /* no Global subr */
  if (cffont->gsubr)
    cff_release_index(cffont->gsubr);
  cffont->gsubr = cff_new_index(0);

  if (cffont->subrs && cffont->subrs[0]) {
    cff_release_index(cffont->subrs[0]);
    cffont->subrs[0] = NULL;
  }
  if (cffont->private && (cffont->private)[0]) {
    cff_dict_remove((cffont->private)[0], "Subrs"); /* no Subrs */
  }

  cff_add_string(cffont, "Adobe", 1);
  cff_add_string(cffont, "Identity", 1);

  cff_dict_update(cffont->topdict, cffont);
  cff_dict_update(cffont->private[0], cffont);
  cff_update_string(cffont);

  /* CFF code need to be rewrote... */
  cff_dict_add(cffont->topdict, "ROS", 3);
  cff_dict_set(cffont->topdict, "ROS", 0,
               (double) cff_get_sid(cffont, "Adobe"));
  cff_dict_set(cffont->topdict, "ROS", 1,
               (double) cff_get_sid(cffont, "Identity"));
  cff_dict_set(cffont->topdict, "ROS", 2, 0.0);

  destlen = write_fontfile(font, cffont);

  /*
   * DW, W, DW2 and W2:
   * Those values are obtained from OpenType table (not TFM).
   */
  {
    unsigned char *CIDToGIDMap;

    CIDToGIDMap = NEW(2 * (last_cid+1), unsigned char);
    memset(CIDToGIDMap, 0, 2 * (last_cid + 1));
    for (cid = 0; cid <= last_cid; cid++) {
      if (is_used_char2(used_chars, cid)) {
        CIDToGIDMap[2*cid  ] = (cid >> 8) & 0xff;
        CIDToGIDMap[2*cid+1] = cid & 0xff;
      }
    }
    add_CIDMetrics(info.sfont, font->fontdict, CIDToGIDMap, last_cid,
                   ((CIDFont_get_parent_id(font, 1) < 0) ? 0 : 1));
    RELEASE(CIDToGIDMap);
  }

  CIDFontInfo_close(&info);

  if (verbose > 1)
    MESG("[%u glyphs][%ld bytes]", num_glyphs, destlen);

  CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}

static int
load_base_CMap (const char *font_name, int wmode, cff_font *cffont)
{
  int       cmap_id = -1;
  CMap     *cmap;
  char     *cmap_name;
  card16    gid;
  unsigned char range_min[4] = {0x00, 0x00, 0x00, 0x00};
  unsigned char range_max[4] = {0x7f, 0xff, 0xff, 0xff};

  cmap_name = NEW(strlen(font_name)+strlen("-UCS4-H")+1, char);
  if (wmode) {
    sprintf(cmap_name, "%s-UCS4-V", font_name);
  } else {
    sprintf(cmap_name, "%s-UCS4-H", font_name);
  }

  cmap_id = CMap_cache_find(cmap_name);
  if (cmap_id >= 0) {
    RELEASE(cmap_name);
    return cmap_id;
  }

  cmap = CMap_new();
  CMap_set_name (cmap, cmap_name);
  CMap_set_type (cmap, CMAP_TYPE_CODE_TO_CID);
  CMap_set_wmode(cmap, wmode);
  CMap_add_codespacerange(cmap, range_min, range_max, 4);
  CMap_set_CIDSysInfo(cmap, &CSI_IDENTITY);
  RELEASE(cmap_name);

  for (gid = 1; gid < cffont->num_glyphs; gid++) {
    int32_t   ucv;
    s_SID     sid;
    char     *glyph, *name, *suffix;
    unsigned char  srcCode[4];

    sid   = cff_charsets_lookup_inverse(cffont, gid);
    glyph = cff_get_string (cffont, sid);

    name  = agl_chop_suffix(glyph, &suffix);
    if (!name) {
      if (suffix)
        RELEASE(suffix);
      RELEASE(glyph);
      continue;
    }

    if (suffix) {
      RELEASE(name);
      RELEASE(suffix);
      RELEASE(glyph);
      continue;
    }

    if (agl_name_is_unicode(name)) {
      ucv = agl_name_convert_unicode(name);
      srcCode[0] = (ucv >> 24) & 0xff;
      srcCode[1] = (ucv >> 16) & 0xff;
      srcCode[2] = (ucv >>  8) & 0xff;
      srcCode[3] = ucv & 0xff;
      CMap_add_cidchar(cmap, srcCode, 4, gid);
    } else {
      agl_name *agln;

      agln = agl_lookup_list(name);
      if (!agln)
        WARN("Glyph \"%s\" inaccessible (no Unicode mapping)", glyph);
      while (agln) {
        if (agln->n_components > 1) {
          WARN("Glyph \"%s\" inaccessible (composite character)", glyph);
        } else if (agln->n_components == 1) {
          ucv = agln->unicodes[0];
          srcCode[0] = (ucv >> 24) & 0xff;
          srcCode[1] = (ucv >> 16) & 0xff;
          srcCode[2] = (ucv >>  8) & 0xff;
          srcCode[3] = ucv & 0xff;
          CMap_add_cidchar(cmap, srcCode, 4, gid);
        }
        agln = agln->alternate;
      }
    }
    RELEASE(name);
    if (suffix)
      RELEASE(suffix);
    RELEASE(glyph);
  }
  cmap_id = CMap_cache_add(cmap);

  return cmap_id;
}

int
t1_load_UnicodeCMap (const char *font_name,
                     const char *otl_tags,  /* not supported yet */
                     int wmode)
{
  int       cmap_id = -1;
  cff_font *cffont;
  FILE     *fp;

  if (!font_name)
    return -1;

  fp = DPXFOPEN(font_name, DPX_RES_TYPE_T1FONT);
  if (!fp)
    return -1;

  cffont = t1_load_font(NULL, 1, fp);
  DPXFCLOSE(fp);
  if (!cffont)
    return -1;

  cmap_id = load_base_CMap(font_name, wmode, cffont);
  
  cff_close(cffont);

  if (cmap_id < 0) {
    ERROR("Failed to create Unicode charmap for font \"%s\".", font_name);
    return -1;
  }

  if (otl_tags) {
    WARN("Glyph substitution not supported for Type1 font yet...");
  }

  return cmap_id;
}


/*
 * ToUnicode CMap
 */

static pdf_obj *
create_ToUnicode_stream (cff_font *cffont,
                         const char *font_name, const char *used_glyphs)
{
  pdf_obj *stream = NULL;
  CMap    *cmap;
  CID      cid;
  card16   gid;
  int      glyph_count, total_fail_count;
  char    *cmap_name;
#define WBUF_SIZE 1024
  unsigned char  wbuf[WBUF_SIZE];
  unsigned char *p, *endptr;
  static unsigned char range_min[2] = {0x00, 0x00};
  static unsigned char range_max[2] = {0xff, 0xff};

  if (!font_name || !used_glyphs)
    return NULL;

  cmap = CMap_new();

  cmap_name = NEW(strlen(font_name)+strlen("-UTF16")+1, char);
  strcpy(cmap_name, font_name);
  strcat(cmap_name, "-UTF16");
  CMap_set_name (cmap, cmap_name);
  RELEASE(cmap_name);

  CMap_set_wmode(cmap, 0);
  CMap_set_type (cmap, CMAP_TYPE_TO_UNICODE);
  CMap_set_CIDSysInfo(cmap, &CSI_UNICODE);

  CMap_add_codespacerange(cmap, range_min, range_max, 2);

  glyph_count = total_fail_count = 0;
  p      = wbuf;
  endptr = wbuf + WBUF_SIZE;
  for (cid = 1; cid < cffont->num_glyphs; cid++) { /* Skip .notdef */
    if (is_used_char2(used_glyphs, cid)) {
      char *glyph;
      int32_t len;
      int   fail_count;

      wbuf[0] = (cid >> 8) & 0xff;
      wbuf[1] = (cid & 0xff);

      p = wbuf + 2;
      gid = cff_charsets_lookup_inverse(cffont, cid);
      if (gid == 0)
        continue;
      glyph = cff_get_string(cffont, gid);
      if (glyph) {
        len = agl_sput_UTF16BE(glyph, &p, endptr, &fail_count);
        if (len < 1 || fail_count) {
          total_fail_count += fail_count;
        } else {
          CMap_add_bfchar(cmap, wbuf, 2, wbuf+2, len);
        }
        RELEASE(glyph);
      }
      glyph_count++;
    }
  }

  if (total_fail_count != 0 &&
      total_fail_count >= glyph_count/10) {
    WARN("%d glyph names (out of %d) missing Unicode mapping.",
         total_fail_count, glyph_count);
    WARN("ToUnicode CMap \"%s-UTF16\" removed.", font_name);
  } else {
    stream = CMap_create_stream(cmap);
  }
  CMap_release(cmap);

  return stream;
}

/* Duplicate from type1.c */
#define TYPE1_NAME_LEN_MAX   127

#define FONT_FLAG_FIXEDPITCH (1 << 0)  /* Fixed-width font */
#define FONT_FLAG_SERIF      (1 << 1)  /* Serif font */
#define FONT_FLAG_SYMBOLIC   (1 << 2)  /* Symbolic font */
#define FONT_FLAG_SCRIPT     (1 << 3)  /* Script font */
#define FONT_FLAG_STANDARD   (1 << 5)  /* Adobe Standard Character Set */
#define FONT_FLAG_ITALIC     (1 << 6)  /* Italic */
#define FONT_FLAG_ALLCAP     (1 << 16) /* All-cap font */
#define FONT_FLAG_SMALLCAP   (1 << 17) /* Small-cap font */
#define FONT_FLAG_FORCEBOLD  (1 << 18) /* Force bold at small text sizes */

/* pdf_font --> CIDFont */
static void
get_font_attr (CIDFont *font, cff_font *cffont)
{
  double capheight, ascent, descent;
  double italicangle, stemv;
  double defaultwidth, nominalwidth;
  int    flags = 0;
  int    gid;
  int    i;
  static const char *L_c[] = {
    "H", "P", "Pi", "Rho", NULL
  };
  static const char *L_d[] = {
    "p", "q", "mu", "eta", NULL
  };
  static const char *L_a[] = {
    "b", "h", "lambda", NULL
  };
  t1_ginfo gm;

  defaultwidth = 500.0;
  nominalwidth = 0.0;

  /*
   * CapHeight, Ascent, and Descent is meaningfull only for Latin/Greek/Cyrillic.
   * The BlueValues and OtherBlues also have those information.
   */
  if (cff_dict_known(cffont->topdict, "FontBBox")) {
    /* Default values */
    capheight = ascent = cff_dict_get(cffont->topdict, "FontBBox", 3);
    descent = cff_dict_get(cffont->topdict, "FontBBox", 1);
  } else {
    capheight =  680.0;
    ascent    =  690.0;
    descent   = -190.0;
  }
  if (cff_dict_known(cffont->private[0], "StdVW")) {
    stemv = cff_dict_get(cffont->private[0], "StdVW", 0);
  } else {
    /*
     * We may use the following values for StemV:
     *  Thin - ExtraLight: <= 50
     *  Light: 71
     *  Regular(Normal): 88
     *  Medium: 109
     *  SemiBold(DemiBold): 135
     *  Bold - Heavy: >= 166
     */
    stemv = 88.0;
  }
  if (cff_dict_known(cffont->topdict, "ItalicAngle")) {
    italicangle = cff_dict_get(cffont->topdict, "ItalicAngle", 0);
    if (italicangle != 0.0)
      flags |= FONT_FLAG_ITALIC;
  } else {
    italicangle = 0.0;
  }

  /*
   * Use "space", "H", "p", and "b" for various values.
   * Those characters should not "seac". (no accent)
   */
  gid = cff_glyph_lookup(cffont, "space");
  if (gid >= 0 && gid < cffont->cstrings->count) {
    t1char_get_metrics(cffont->cstrings->data + cffont->cstrings->offset[gid] - 1,
                       cffont->cstrings->offset[gid+1] - cffont->cstrings->offset[gid],
                       cffont->subrs[0], &gm);
    defaultwidth = gm.wx;
  }

  for (i = 0; L_c[i] != NULL; i++) {
    gid = cff_glyph_lookup(cffont, L_c[i]);
    if (gid >= 0 && gid < cffont->cstrings->count) {
      t1char_get_metrics(cffont->cstrings->data + cffont->cstrings->offset[gid] - 1,
                         cffont->cstrings->offset[gid+1] - cffont->cstrings->offset[gid],
                         cffont->subrs[0], &gm);
      capheight = gm.bbox.ury;
      break;
    }
  }

  for (i = 0; L_d[i] != NULL; i++) {
    gid = cff_glyph_lookup(cffont, L_d[i]);
    if (gid >= 0 && gid < cffont->cstrings->count) {
      t1char_get_metrics(cffont->cstrings->data + cffont->cstrings->offset[gid] - 1,
                         cffont->cstrings->offset[gid+1] - cffont->cstrings->offset[gid],
                         cffont->subrs[0], &gm);
      descent = gm.bbox.lly;
      break;
    }
  }

  for (i = 0; L_a[i] != NULL; i++) {
    gid = cff_glyph_lookup(cffont, L_a[i]);
    if (gid >= 0 && gid < cffont->cstrings->count) {
      t1char_get_metrics(cffont->cstrings->data + cffont->cstrings->offset[gid] - 1,
                         cffont->cstrings->offset[gid+1] - cffont->cstrings->offset[gid],
                         cffont->subrs[0], &gm);
      ascent = gm.bbox.ury;
      break;
    }
  }

  if (defaultwidth != 0.0) {
    cff_dict_add(cffont->private[0], "defaultWidthX", 1);
    cff_dict_set(cffont->private[0], "defaultWidthX", 0, defaultwidth);
  }
  if (nominalwidth != 0.0) {
    cff_dict_add(cffont->private[0], "nominalWidthX", 1);
    cff_dict_set(cffont->private[0], "nominalWidthX", 0, nominalwidth);
  }
  if (cff_dict_known(cffont->private[0], "ForceBold") &&
      cff_dict_get(cffont->private[0], "ForceBold", 0)) {
    flags |= FONT_FLAG_FORCEBOLD;
  }
  if (cff_dict_known(cffont->private[0], "IsFixedPitch") &&
      cff_dict_get(cffont->private[0], "IsFixedPitch", 0)) {
    flags |= FONT_FLAG_FIXEDPITCH;
  }
  if (font->fontname &&
      !strstr(font->fontname, "Sans")) {
    flags |= FONT_FLAG_SERIF;
  }
  flags |= FONT_FLAG_SYMBOLIC;

  pdf_add_dict(font->descriptor,
               pdf_new_name("CapHeight"), pdf_new_number(capheight));
  pdf_add_dict(font->descriptor,
               pdf_new_name("Ascent"), pdf_new_number(ascent));
  pdf_add_dict(font->descriptor,
               pdf_new_name("Descent"), pdf_new_number(descent));
  pdf_add_dict(font->descriptor,
               pdf_new_name("ItalicAngle"), pdf_new_number(italicangle));
  pdf_add_dict(font->descriptor,
               pdf_new_name("StemV"), pdf_new_number(stemv));
  pdf_add_dict(font->descriptor,
               pdf_new_name("Flags"), pdf_new_number(flags));
}

static void
add_metrics (CIDFont *font, cff_font *cffont,
             unsigned char *CIDToGIDMap,
             double *widths, double default_width, CID last_cid)
{
  pdf_obj *tmp;
  double   val;
  card16   cid, gid;
  char    *used_chars;
  int      i, parent_id;

  /*
   * The original FontBBox of the font is preserved, instead
   * of replacing it with tight bounding box calculated from
   * charstrings, to prevent Acrobat 4 from greeking text as
   * much as possible.
   */
  if (!cff_dict_known(cffont->topdict, "FontBBox")) {
    ERROR("No FontBBox?");
  }
  tmp = pdf_new_array();
  for (i = 0; i < 4; i++) {
    val = cff_dict_get(cffont->topdict, "FontBBox", i);
    pdf_add_array(tmp, pdf_new_number(ROUND(val, 1.0)));
  }
  pdf_add_dict(font->descriptor, pdf_new_name("FontBBox"), tmp);

  if ((parent_id = CIDFont_get_parent_id(font, 0)) < 0 &&
      (parent_id = CIDFont_get_parent_id(font, 1)) < 0)
    ERROR("No parent Type 0 font !");

  used_chars = Type0Font_get_usedchars(Type0Font_cache_get(parent_id));
  if (!used_chars) {
    ERROR("Unexpected error: Font not actually used???");
  }

  /* FIXME:
   * This writes "CID CID width".
   * I think it's better to handle each 8 char block
   * and to use "CID_start [ w0 w1 ...]".
   */
  tmp = pdf_new_array();
  for (cid = 0; cid <= last_cid; cid++) {
    if (is_used_char2(used_chars, cid)) {
      gid = (CIDToGIDMap[2*cid] << 8)|CIDToGIDMap[2*cid+1];
      if (widths[gid] != default_width) {
        pdf_add_array(tmp, pdf_new_number(cid));
        pdf_add_array(tmp, pdf_new_number(cid));
        pdf_add_array(tmp, pdf_new_number(ROUND(widths[gid], 1.0)));
      }
    }
  }
  pdf_add_dict(font->fontdict,
               pdf_new_name("DW"), pdf_new_number(default_width));
  if (pdf_array_length(tmp) > 0) {
    pdf_add_dict(font->fontdict, pdf_new_name("W"), pdf_ref_obj(tmp));
  }
  pdf_release_obj(tmp);
}

void
CIDFont_type0_t1dofont (CIDFont *font)
{
  cff_font *cffont;
  double    defaultwidth, nominalwidth;
  int       num_glyphs = 0;
  FILE     *fp;
  int       i, offset;
  char     *used_chars = NULL;
  card16    last_cid, gid, cid;
  unsigned char *CIDToGIDMap;

  ASSERT(font);

  if (!font->indirect) {
    return;
  }

  pdf_add_dict(font->fontdict, 
               pdf_new_name("FontDescriptor"),
               pdf_ref_obj (font->descriptor));

  fp = DPXFOPEN(font->ident, DPX_RES_TYPE_T1FONT);
  if (!fp) {
    ERROR("Type1: Could not open Type1 font.");
  }

  cffont = t1_load_font(NULL, 0, fp);
  if (!cffont)
    ERROR("Could not read Type 1 font...");
  DPXFCLOSE(fp);

  if (!font->fontname)
    ERROR("Fontname undefined...");

  {
    Type0Font *hparent, *vparent;
    pdf_obj   *tounicode;
    int        vparent_id, hparent_id;

    hparent_id = CIDFont_get_parent_id(font, 0);
    vparent_id = CIDFont_get_parent_id(font, 1);
    if (hparent_id < 0 && vparent_id < 0)
      ERROR("No parent Type 0 font !");

    /* usedchars is same for h and v */
    if (hparent_id < 0)
      hparent = NULL;
    else {
      hparent    = Type0Font_cache_get(hparent_id);
      used_chars = Type0Font_get_usedchars(hparent);
    }
    if (vparent_id < 0)
      vparent = NULL;
    else {
      vparent    = Type0Font_cache_get(vparent_id);
      used_chars = Type0Font_get_usedchars(vparent);
    }
    if (!used_chars)
      ERROR("Unexpected error: Font not actually used???");

    tounicode = create_ToUnicode_stream(cffont, font->fontname, used_chars);

    if (hparent)
      Type0Font_set_ToUnicode(hparent, pdf_ref_obj(tounicode));
    if (vparent)
      Type0Font_set_ToUnicode(vparent, pdf_ref_obj(tounicode));
    pdf_release_obj(tounicode);
  }

  cff_set_name(cffont, font->fontname);

  /* defaultWidthX, CapHeight, etc. */
  get_font_attr(font, cffont);
  if (cff_dict_known(cffont->private[0], "defaultWidthX")) {
    defaultwidth = cff_dict_get(cffont->private[0], "defaultWidthX", 0);
  } else {
    defaultwidth = 0.0;
  }
  if (cff_dict_known(cffont->private[0], "nominalWidthX")) {
    nominalwidth = cff_dict_get(cffont->private[0], "nominalWidthX", 0);
  } else {
    nominalwidth = 0.0;
  }

  num_glyphs = 0; last_cid = 0;
  add_to_used_chars2(used_chars, 0); /* .notdef */
  for (i = 0; i < (cffont->num_glyphs + 7)/8; i++) {
    int c, j;

    c = used_chars[i];
    for (j = 7; j >= 0; j--) {
      if (c & (1 << j)) {
        num_glyphs++;
        last_cid = (i + 1) * 8 - j - 1;
      }
    }
  }

  {
    cff_fdselect *fdselect;

    fdselect = NEW(1, cff_fdselect);
    fdselect->format = 3;
    fdselect->num_entries = 1;
    fdselect->data.ranges = NEW(1, cff_range3);
    fdselect->data.ranges[0].first = 0;
    fdselect->data.ranges[0].fd    = 0;
    cffont->fdselect = fdselect;
  }

  CIDToGIDMap = NEW(2*(last_cid+1), unsigned char);
  memset(CIDToGIDMap, 0, 2*(last_cid+1));
  {
    cff_charsets *charset;

    charset  = NEW(1, cff_charsets);
    charset->format = 0;
    charset->num_entries = num_glyphs-1;
    charset->data.glyphs = NEW(num_glyphs-1, s_SID);

    for (gid = 0, cid = 0; cid <= last_cid; cid++) {
      if (is_used_char2(used_chars, cid)) {
        if (gid > 0)
          charset->data.glyphs[gid-1] = cid;
        CIDToGIDMap[2*cid  ] = (gid >> 8) & 0xff;
        CIDToGIDMap[2*cid+1] = gid & 0xff;
        gid++;
      }
    }

    cff_release_charsets(cffont->charsets);
    cffont->charsets = charset;
  }

  cff_dict_add(cffont->topdict, "CIDCount", 1);
  cff_dict_set(cffont->topdict, "CIDCount", 0, last_cid + 1);

  cffont->fdarray    = NEW(1, cff_dict *);
  cffont->fdarray[0] = cff_new_dict();
  cff_dict_add(cffont->fdarray[0], "FontName", 1);
  cff_dict_set(cffont->fdarray[0], "FontName", 0,
               (double) cff_add_string(cffont, font->fontname + 7, 1)); /* FIXME: Skip XXXXXX+ */
  cff_dict_add(cffont->fdarray[0], "Private", 2);
  cff_dict_set(cffont->fdarray[0], "Private", 0, 0.0);
  cff_dict_set(cffont->fdarray[0], "Private", 0, 0.0);

  /* FDArray  - index offset, not known yet */
  cff_dict_add(cffont->topdict, "FDArray", 1);
  cff_dict_set(cffont->topdict, "FDArray", 0, 0.0);
  /* FDSelect - offset, not known yet */
  cff_dict_add(cffont->topdict, "FDSelect", 1);
  cff_dict_set(cffont->topdict, "FDSelect", 0, 0.0);

  cff_dict_add(cffont->topdict, "charset", 1);
  cff_dict_set(cffont->topdict, "charset", 0, 0.0);

  cff_dict_add(cffont->topdict, "CharStrings", 1);
  cff_dict_set(cffont->topdict, "CharStrings", 0, 0.0);

  {
    cff_index *cstring;
    t1_ginfo   gm;
    int        max = 0;
    double    *widths;
    int        w_stat[1001], max_count, dw;

    widths = NEW(num_glyphs, double);
    memset(w_stat, 0, sizeof(int)*1001);
    offset  = 0L;
    cstring = cff_new_index((card16)num_glyphs);
    cstring->data = NULL;
    cstring->offset[0] = 1;
    gid = 0;
    for (cid = 0; cid <= last_cid; cid++) {
      if (!is_used_char2(used_chars, cid))
          continue;

      if (offset + CS_STR_LEN_MAX >= max) {
        max += CS_STR_LEN_MAX*2;
        cstring->data = RENEW(cstring->data, max, card8);
      }
      offset += t1char_convert_charstring(cstring->data + cstring->offset[gid] - 1, CS_STR_LEN_MAX,
                                          cffont->cstrings->data + cffont->cstrings->offset[cid] - 1,
                                          cffont->cstrings->offset[cid+1] - cffont->cstrings->offset[cid],
                                          cffont->subrs[0], defaultwidth, nominalwidth, &gm);
      cstring->offset[gid+1] = offset + 1;
      if (gm.use_seac) {
        ERROR("This font using the \"seac\" command for accented characters...");
      }
      widths[gid] = gm.wx;
      if (gm.wx >= 0.0 && gm.wx <= 1000.0) {
        w_stat[((int) gm.wx)] += 1;
      }
      gid++;
    }

    cff_release_index(cffont->cstrings);
    cffont->cstrings = cstring;

    max_count = 0; dw = -1;
    for (i = 0; i <= 1000; i++) {
      if (w_stat[i] > max_count) {
        dw        = i;
        max_count = w_stat[i];
      }
    }
    if (dw >= 0) {
      add_metrics(font, cffont, CIDToGIDMap, widths, dw, last_cid);
    } else {
      add_metrics(font, cffont, CIDToGIDMap, widths, defaultwidth, last_cid);
    }
    RELEASE(widths);
  }
  cff_release_index(cffont->subrs[0]);
  cffont->subrs[0] = NULL;

  RELEASE(CIDToGIDMap);

  cff_add_string(cffont, "Adobe", 1);
  cff_add_string(cffont, "Identity", 1);

  cff_dict_update(cffont->topdict, cffont);
  cff_dict_update(cffont->private[0], cffont);

  cff_update_string(cffont);

  /* CFF code need to be rewrote... */
  cff_dict_add(cffont->topdict, "ROS", 3);
  cff_dict_set(cffont->topdict, "ROS", 0,
               (double) cff_get_sid(cffont, "Adobe"));
  cff_dict_set(cffont->topdict, "ROS", 1,
               (double) cff_get_sid(cffont, "Identity"));
  cff_dict_set(cffont->topdict, "ROS", 2, 0.0);

  cffont->num_glyphs = num_glyphs;
  offset = write_fontfile(font, cffont);

  cff_close(cffont);

  CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
}
