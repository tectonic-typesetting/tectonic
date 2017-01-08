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
 * A large part of codes are brought from ttfdump-0.5.5.
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "system.h"
#include "mem.h"
#include "error.h"

#include "sfnt.h"


/* Sorry for placing this here.
 * We need to rewrite TrueType font support code...
 */
#include "cmap.h"
#include "cmap_write.h"

#include "tt_aux.h"
#include "tt_gsub.h"
#include "tt_post.h"

#include "unicode.h"
#include "agl.h"
#include "pdfparse.h"
#include "pdfresource.h"
#include "otl_conf.h"

#include "dpxfile.h"

/* Hash */
#include "dpxutil.h"

#include "tt_cmap.h"

#define VERBOSE_LEVEL_MIN 0
static int verbose = 0;
void
otf_cmap_set_verbose (void)
{
  otl_gsub_set_verbose();
  verbose++;
}

/* format 0: byte encoding table */
struct cmap0
{
  BYTE glyphIndexArray[256];
};

static struct cmap0 *
read_cmap0 (sfnt *sfont, ULONG len)
{
  struct cmap0 *map;
  int    i;

  if (len < 256)
    ERROR("invalid cmap subtable");

  map = NEW(1, struct cmap0);

  for (i = 0; i < 256; i++)
    map->glyphIndexArray[i] = sfnt_get_byte(sfont);

  return map;
}

static void
release_cmap0(struct cmap0 *map)
{
  if (map)
    RELEASE(map);
}

static USHORT
lookup_cmap0 (struct cmap0 *map, USHORT cc)
{
  return ((cc > 255) ? 0 : map->glyphIndexArray[cc]);
}

/* format 2: high-byte mapping through table */
struct SubHeader
{
  USHORT firstCode;
  USHORT entryCount;
  SHORT  idDelta;
  USHORT idRangeOffset;
};

struct cmap2
{
  USHORT  subHeaderKeys[256];
  struct SubHeader *subHeaders;
  USHORT *glyphIndexArray;
};

static struct cmap2 *
read_cmap2 (sfnt *sfont, ULONG len)
{
  struct cmap2 *map;
  USHORT i, n;

  if (len < 512)
    ERROR("invalid cmap subtable");
    
  map = NEW(1, struct cmap2);

  for (i = 0; i < 256; i++)
    map->subHeaderKeys[i] = sfnt_get_ushort(sfont);

  for (n = 0, i = 0; i < 256; i++) {
    map->subHeaderKeys[i] /= 8;
    if (n < map->subHeaderKeys[i])
      n = map->subHeaderKeys[i];
  }
  n += 1; /* the number of subHeaders is one plus the max of subHeaderKeys */

  map->subHeaders = NEW(n, struct SubHeader); 
  for (i = 0; i < n; i++) {
    map->subHeaders[i].firstCode     = sfnt_get_ushort(sfont);
    map->subHeaders[i].entryCount    = sfnt_get_ushort(sfont);
    map->subHeaders[i].idDelta       = sfnt_get_short(sfont);
    map->subHeaders[i].idRangeOffset = sfnt_get_ushort(sfont);

    /* It makes things easier to let the offset starts from
     * the beginning of glyphIndexArray.
     */
    if (map->subHeaders[i].idRangeOffset != 0)
      map->subHeaders[i].idRangeOffset -= (2 + (n - i - 1) * 8);
  }

  /* Caculate the length of glyphIndexArray, this is ugly,
   * there should be a better way to get this information.
   */
  n = (USHORT) (len - 518 - n * 8) / 2;

  map->glyphIndexArray = NEW(n, USHORT);
  for (i = 0; i < n; i++)
    map->glyphIndexArray[i] = sfnt_get_ushort(sfont);

  return map;
}

static void
release_cmap2 (struct cmap2 *map)
{
  if (map) {
    if (map->subHeaders)
      RELEASE(map->subHeaders);
    if (map->glyphIndexArray)
      RELEASE(map->glyphIndexArray);
    RELEASE(map);
  }
}

static USHORT
lookup_cmap2 (struct cmap2 *map, USHORT cc)
{
  USHORT  idx = 0;
  SHORT   idDelta;
  USHORT  firstCode, entryCount, idRangeOffset;
  int     hi, lo;
  USHORT  i;
   
  hi = (cc >> 8) & 0xff;
  lo = cc & 0xff;

  /* select which subHeader to use */
  i = map->subHeaderKeys[hi];

  firstCode     = map->subHeaders[i].firstCode;
  entryCount    = map->subHeaders[i].entryCount;
  idDelta       = map->subHeaders[i].idDelta;
  idRangeOffset = map->subHeaders[i].idRangeOffset / 2;

  if (lo >= firstCode &&
      lo < firstCode + entryCount) {
    idRangeOffset += lo - firstCode;
    idx = map->glyphIndexArray[idRangeOffset];
    if (idx != 0)
      idx = (idx + idDelta) & 0xffff;
  }

  return idx;
}

/*
 * format 4: segment mapping to delta values
 * - Microsoft standard character to glyph index mapping table
 */
struct cmap4
{
  USHORT  segCountX2;
  USHORT  searchRange;
  USHORT  entrySelector;
  USHORT  rangeShift;
  USHORT *endCount;
  USHORT  reservedPad;
  USHORT *startCount;
  USHORT *idDelta;
  USHORT *idRangeOffset;
  USHORT *glyphIndexArray;
};

static struct cmap4 *
read_cmap4(sfnt *sfont, ULONG len)
{
  struct cmap4 *map;
  USHORT i, n, segCount;

  if (len < 8)
    ERROR("invalid cmap subtable");

  map = NEW(1, struct cmap4);

  map->segCountX2    = segCount = sfnt_get_ushort(sfont);
  map->searchRange   = sfnt_get_ushort(sfont);
  map->entrySelector = sfnt_get_ushort(sfont);
  map->rangeShift    = sfnt_get_ushort(sfont);
  
  segCount /= 2;

  map->endCount = NEW(segCount, USHORT);
  for (i = 0; i < segCount; i++)
    map->endCount[i] = sfnt_get_ushort(sfont);

  map->reservedPad = sfnt_get_ushort(sfont);

  map->startCount  = NEW(segCount, USHORT);
  for (i = 0; i < segCount; i++)
    map->startCount[i] = sfnt_get_ushort(sfont);

  map->idDelta = NEW(segCount, USHORT);
  for (i = 0; i < segCount; i++)
    map->idDelta[i] = sfnt_get_ushort(sfont);

  map->idRangeOffset = NEW(segCount, USHORT);
  for (i = 0; i < segCount; i++)
    map->idRangeOffset[i] = sfnt_get_ushort(sfont);

  n = (len - 16 - 8 * segCount) / 2;
  if (n == 0)
    map->glyphIndexArray = NULL;
  else {
    map->glyphIndexArray = NEW(n, USHORT);
    for (i = 0; i < n; i++)
      map->glyphIndexArray[i] = sfnt_get_ushort(sfont);
  }

  return map;
}

static void
release_cmap4 (struct cmap4 *map)
{
  if (map) {
    if (map->endCount)   RELEASE(map->endCount);
    if (map->startCount) RELEASE(map->startCount);
    if (map->idDelta)    RELEASE(map->idDelta);
    if (map->idRangeOffset)   RELEASE(map->idRangeOffset);
    if (map->glyphIndexArray) RELEASE(map->glyphIndexArray);
    RELEASE(map);
  }
}

static USHORT
lookup_cmap4 (struct cmap4 *map, USHORT cc)
{
  USHORT gid = 0;
  USHORT i, j, segCount;

  /*
   * Segments are sorted in order of increasing endCode values.
   * Last segment maps 0xffff to gid 0 (?)
  */
  i = segCount = map->segCountX2 / 2;
  while (i-- > 0 &&  cc <= map->endCount[i]) {
    if (cc >= map->startCount[i]) {
      if (map->idRangeOffset[i] == 0) {
	gid = (cc + map->idDelta[i]) & 0xffff;
      } else if (cc == 0xffff && map->idRangeOffset[i] == 0xffff) {
	/* this is for protection against some old broken fonts... */
	gid = 0;
      } else {
	j  = map->idRangeOffset[i] - (segCount - i) * 2;
	j  = (cc - map->startCount[i]) + (j / 2);
	gid = map->glyphIndexArray[j];
	if (gid != 0)
	  gid = (gid + map->idDelta[i]) & 0xffff;
      }
      break;
    }
  }

  return gid;
}

/* format 6: trimmed table mapping */
struct cmap6
{
  USHORT  firstCode;
  USHORT  entryCount;
  USHORT *glyphIndexArray;
};

static struct cmap6 *
read_cmap6 (sfnt *sfont, ULONG len)
{
  struct cmap6 *map;
  USHORT i;
  
  if (len < 4)
    ERROR("invalid cmap subtable");

  map =  NEW(1, struct cmap6);
  map->firstCode       = sfnt_get_ushort(sfont);
  map->entryCount      = sfnt_get_ushort(sfont);
  map->glyphIndexArray = NEW(map->entryCount, USHORT);
  
  for (i = 0; i < map->entryCount; i++)
    map->glyphIndexArray[i] = sfnt_get_ushort(sfont);

  return map;
}

static void
release_cmap6 (struct cmap6 *map)
{
  if (map) {
    if (map->glyphIndexArray)
      RELEASE(map->glyphIndexArray);
    RELEASE(map);
  }
}

static USHORT
lookup_cmap6 (struct cmap6 *map, USHORT cc)
{
  USHORT idx;

  idx = cc - map->firstCode; 
  if (idx < map->entryCount)
    return map->glyphIndexArray[idx];
  return 0;
}

/* Format 8 and 10 not supported...
 *
 *  format  8: mixed 16-bit and 32-bit coverage
 *  format 10: trimmed array
 */

/*
 * format 12: segmented coverage
 *
 * startGlyphID is 32-bit long, however, GlyphID is still 16-bit long !
 */

struct charGroup
{
  ULONG startCharCode;
  ULONG endCharCode;
  ULONG startGlyphID;
};

struct cmap12
{
  ULONG  nGroups;
  struct charGroup *groups;
};

/* ULONG length */
static struct cmap12 *
read_cmap12 (sfnt *sfont, ULONG len)
{
  struct cmap12 *map;
  ULONG  i;
  
  if (len < 4)
    ERROR("invalid cmap subtable");

  map =  NEW(1, struct cmap12);
  map->nGroups = sfnt_get_ulong(sfont);
  map->groups  = NEW(map->nGroups, struct charGroup);

  for (i = 0; i < map->nGroups; i++) {
    map->groups[i].startCharCode = sfnt_get_ulong(sfont);
    map->groups[i].endCharCode   = sfnt_get_ulong(sfont);
    map->groups[i].startGlyphID  = sfnt_get_ulong(sfont);
  }

  return map;
}

static void
release_cmap12 (struct cmap12 *map)
{
  if (map) {
    if (map->groups)
      RELEASE(map->groups);
    RELEASE(map);
  }
}

static USHORT
lookup_cmap12 (struct cmap12 *map, ULONG cccc)
{
  USHORT gid = 0;
  int i;

  i = map->nGroups;
  while (i-- >= 0 &&
	 cccc <= map->groups[i].endCharCode) {
    if (cccc >= map->groups[i].startCharCode) {
      gid = (USHORT) ((cccc -
		       map->groups[i].startCharCode +
		       map->groups[i].startGlyphID) & 0xffff);
      break;
    }
  }

  return gid;
}

/* read cmap */
tt_cmap *
tt_cmap_read (sfnt *sfont, USHORT platform, USHORT encoding)
{
  tt_cmap *cmap = NULL;
  ULONG    offset, length = 0;
  USHORT   p_id, e_id;
  USHORT   i, n_subtabs;

  ASSERT(sfont);

  offset    = sfnt_locate_table(sfont, "cmap");
  (void)      sfnt_get_ushort(sfont);
  n_subtabs = sfnt_get_ushort(sfont);

  for (i = 0; i < n_subtabs; i++) {
    p_id = sfnt_get_ushort(sfont);
    e_id = sfnt_get_ushort(sfont);
    if (p_id != platform || e_id != encoding)
      sfnt_get_ulong(sfont);
    else {
      offset += sfnt_get_ulong(sfont);
      break;
    }
  }

  if (i == n_subtabs)
    return NULL;

  cmap = NEW(1, tt_cmap);
  cmap->map      = NULL;
  cmap->platform = platform;
  cmap->encoding = encoding;

  sfnt_seek_set(sfont, offset);
  cmap->format = sfnt_get_ushort(sfont);
  /* Length and version (language) is ULONG for
   * format 8, 10, 12 !
   */
  if (cmap->format <= 6) {
    length         = sfnt_get_ushort(sfont);
    cmap->language = sfnt_get_ushort(sfont); /* language (Mac) */
  } else {
    if (sfnt_get_ushort(sfont) != 0) { /* reverved - 0 */
      WARN("Unrecognized cmap subtable format.");
      tt_cmap_release(cmap);
      return NULL;
    } else {
      length         = sfnt_get_ulong(sfont);
      cmap->language = sfnt_get_ulong(sfont);
    }
  }
  
  switch(cmap->format) {
  case 0:
    cmap->map = read_cmap0(sfont, length);
    break;
  case 2:
    cmap->map = read_cmap2(sfont, length);
    break;
  case 4:
    cmap->map = read_cmap4(sfont, length);
    break;
  case 6:
    cmap->map = read_cmap6(sfont, length);
    break;
  case 12:
    /* WARN("UCS-4 TrueType cmap table..."); */
    cmap->map = read_cmap12(sfont, length);
    break;
  default:
    WARN("Unrecognized OpenType/TrueType cmap format.");
    tt_cmap_release(cmap);
    return NULL;
  }

  if (!cmap->map) {
    tt_cmap_release(cmap);
    cmap = NULL;
  }

  return cmap;
}

void
tt_cmap_release (tt_cmap *cmap)
{

  if (cmap) {
    if (cmap->map) {
      switch(cmap->format) {
      case 0:
	release_cmap0(cmap->map);
	break;
      case 2:
	release_cmap2(cmap->map);
	break;
      case 4:
	release_cmap4(cmap->map);
	break;
      case 6:
	release_cmap6(cmap->map);
	break;
      case 12:
	release_cmap12(cmap->map);
	break;
      default:
	ERROR("Unrecognized OpenType/TrueType cmap format.");
      }
    }
    RELEASE(cmap);
  }

  return;
}


USHORT
tt_cmap_lookup (tt_cmap *cmap, ULONG cc)
{
  USHORT gid = 0;

  ASSERT(cmap);

  if (cc > 0xffffL && cmap->format < 12) {
    WARN("Four bytes charcode not supported in OpenType/TrueType cmap format 0...6.");
    return 0;
  }

  switch (cmap->format) {
  case 0:
    gid = lookup_cmap0(cmap->map,  (USHORT) cc);
    break;
  case 2:
    gid = lookup_cmap2(cmap->map,  (USHORT) cc);
    break;
  case 4:
    gid = lookup_cmap4(cmap->map,  (USHORT) cc);
    break;
  case 6:
    gid = lookup_cmap6(cmap->map,  (USHORT) cc);
    break;
  case 12:
    gid = lookup_cmap12(cmap->map, (ULONG) cc);
    break;
  default:
    ERROR("Unrecognized OpenType/TrueType cmap subtable format");
    break;
  }

  return gid;
}

/* Sorry for placing this here.
 * We need to rewrite TrueType font support code...
 */

#define WBUF_SIZE 1024
static unsigned char wbuf[WBUF_SIZE];

static unsigned char srange_min[2] = {0x00, 0x00};
static unsigned char srange_max[2] = {0xff, 0xff};
static unsigned char lrange_min[4] = {0x00, 0x00, 0x00, 0x00};
static unsigned char lrange_max[4] = {0x7f, 0xff, 0xff, 0xff};

static void
load_cmap4 (struct cmap4 *map,
	    unsigned char *GIDToCIDMap, CMap *cmap)
{
  USHORT  c0, c1, gid, cid;
  USHORT  j, d, segCount;
  USHORT  ch;
  int     i;

  segCount = map->segCountX2 / 2;
  for (i = segCount - 1; i >= 0 ; i--) {
    c0 = map->startCount[i];
    c1 = map->endCount[i];
    d  = map->idRangeOffset[i] / 2 - (segCount - i);
    for (j = 0; j <= c1 - c0; j++) {
      ch = c0 + j;
      if (map->idRangeOffset[i] == 0) {
	gid = (ch + map->idDelta[i]) & 0xffff;
      } else if (c0 == 0xffff && c1 == 0xffff && map->idRangeOffset[i] == 0xffff) {
	/* this is for protection against some old broken fonts... */
	gid = 0;
      } else {
	gid = (map->glyphIndexArray[j+d] +
	       map->idDelta[i]) & 0xffff;
      }
      if (gid != 0 && gid != 0xffff) {
	if (GIDToCIDMap) {
	  cid = ((GIDToCIDMap[2*gid] << 8)|GIDToCIDMap[2*gid+1]);
	  if (cid == 0)
	    WARN("GID %u does not have corresponding CID %u.",
		 gid, cid);
	} else {
	  cid = gid;
	}
	wbuf[0] = 0;
	wbuf[1] = 0;
	wbuf[2] = (ch >> 8) & 0xff;
	wbuf[3] =  ch & 0xff;
	CMap_add_cidchar(cmap, wbuf, 4, cid);
      }
    }
  }

  return;
}

static void
load_cmap12 (struct cmap12 *map,
	     unsigned char *GIDToCIDMap, CMap *cmap)
{
  ULONG   i, ch;  /* LONG ? */
  USHORT  gid, cid;

  for (i = 0; i < map->nGroups; i++) {
    for (ch  = map->groups[i].startCharCode;
	 ch <= map->groups[i].endCharCode;
	 ch++) {
      int  d = ch - map->groups[i].startCharCode;
      gid = (USHORT) ((map->groups[i].startGlyphID + d) & 0xffff);
      if (GIDToCIDMap) {
	cid = ((GIDToCIDMap[2*gid] << 8)|GIDToCIDMap[2*gid+1]);
	if (cid == 0)
	  WARN("GID %u does not have corresponding CID %u.", gid, cid);
      } else {
	cid = gid;
      }
      wbuf[0] = (ch >> 24) & 0xff;
      wbuf[1] = (ch >> 16) & 0xff;
      wbuf[2] = (ch >>  8) & 0xff;
      wbuf[3] = ch & 0xff;
      CMap_add_cidchar(cmap, wbuf, 4, cid);
    }
  }

  return;
}

/* OpenType CIDFont:
 *
 *  We don't use GID for them. OpenType cmap table is for
 *  charcode to GID mapping rather than to-CID mapping.
 */
#include "cid.h"

#include "tt_table.h"
#include "cff_types.h"
#include "cff_dict.h"
#include "cff.h"

static int
handle_CIDFont (sfnt *sfont,
		unsigned char **GIDToCIDMap, CIDSysInfo *csi)
{
  cff_font *cffont;
  int       offset, i;
  card16    num_glyphs, gid;
  cff_charsets  *charset;
  unsigned char *map;
  struct tt_maxp_table *maxp;

  ASSERT(csi);

  offset = sfnt_find_table_pos(sfont, "CFF ");
  if (offset == 0) {
    csi->registry = NULL;
    csi->ordering = NULL;
    *GIDToCIDMap  = NULL;
    return 0;
  }

  maxp       = tt_read_maxp_table(sfont);
  num_glyphs = (card16) maxp->numGlyphs;
  RELEASE(maxp);
  if (num_glyphs < 1)
    ERROR("No glyph contained in this font...");

  cffont = cff_open(sfont->stream, offset, 0);
  if (!cffont)
    ERROR("Could not open CFF font...");

  
  if (!(cffont->flag & FONTTYPE_CIDFONT)) {
    cff_close(cffont);
    csi->registry = NULL;
    csi->ordering = NULL;
    *GIDToCIDMap  = NULL;
    return 0;
  }

  if (!cff_dict_known(cffont->topdict, "ROS")) {
    ERROR("No CIDSystemInfo???");
  } else {
    card16 reg, ord;

    reg = (card16) cff_dict_get(cffont->topdict, "ROS", 0);
    ord = (card16) cff_dict_get(cffont->topdict, "ROS", 1);

    csi->registry = cff_get_string(cffont, reg);
    csi->ordering = cff_get_string(cffont, ord);
    csi->supplement = (int) cff_dict_get(cffont->topdict, "ROS", 2);
  }

  cff_read_charsets(cffont);
  charset = cffont->charsets;
  if (!charset) {
    ERROR("No CFF charset data???");
  }

  map     = NEW(num_glyphs * 2, unsigned char);
  memset(map, 0, num_glyphs * 2);
  switch (charset->format) {
  case 0:
    {
      s_SID   *cids; /* CID... */

      cids = charset->data.glyphs;
      for (gid = 1, i = 0;
	   i < charset->num_entries; i++) {
	map[2*gid  ] = (cids[i] >> 8) & 0xff;
	map[2*gid+1] = cids[i] & 0xff;
	gid++;
      }
    }
    break;
  case 1:
    {
      cff_range1 *ranges;
      card16      cid, count;

      ranges = charset->data.range1;
      for (gid = 1, i = 0;
	   i < charset->num_entries; i++) {
	cid   = ranges[i].first;
	count = ranges[i].n_left + 1; /* card8 */
	while (count-- > 0 &&
	       gid <= num_glyphs) {
	  map[2*gid    ] = (cid >> 8) & 0xff;
	  map[2*gid + 1] = cid & 0xff;
	  gid++; cid++;
	}
      }
    }
    break;
  case 2:
    {
      cff_range2 *ranges;
      card16      cid, count;

      ranges = charset->data.range2;
      if (charset->num_entries == 1 &&
	  ranges[0].first == 1) {
	/* "Complete" CIDFont */
	RELEASE(map); map = NULL;
      } else {
	/* Not trivial mapping */
	for (gid = 1, i = 0;
	     i < charset->num_entries; i++) {
	  cid   = ranges[i].first;
	  count = ranges[i].n_left + 1;
	  while (count-- > 0 &&
		 gid <= num_glyphs) {
	    map[gid] = (cid >> 8) & 0xff;
	    map[gid] = cid & 0xff;
	    gid++; cid++;
	  }
	}
	
      }
    }
    break;
  default:
    RELEASE(map); map = NULL;
    ERROR("Unknown CFF charset format...: %d", charset->format);
    break;
  }
  cff_close(cffont);

  *GIDToCIDMap = map;
  return 1;
}

static int is_PUA_or_presentation (unsigned int uni)
{
  return  ((uni >= 0xE000 && uni <= 0xF8FF) || (uni >= 0xFB00 && uni <= 0xFB4F) ||
           (uni >= 0xF0000 && uni <= 0xFFFFD) || (uni >= 0x100000 && uni <= 0x10FFFD));
}

static char*
sfnt_get_glyphname(struct tt_post_table *post, cff_font *cffont, USHORT gid)
{
  char* name = NULL;

  if (post)
    name = tt_get_glyphname(post, gid);

  if (!name && cffont)
    name = cff_get_glyphname(cffont, gid);

  return name;
}

/*
 * Substituted glyphs:
 *
 *  Mapping information stored in cmap_add.
 */
#ifndef is_used_char2
#define is_used_char2(b,c) (((b)[(c)/8]) & (1 << (7-((c)%8))))
#endif

static USHORT
handle_subst_glyphs (CMap *cmap,
                     CMap *cmap_add,
                     const char *used_glyphs,
                     sfnt *sfont,
                     cff_font *cffont)
{
  USHORT count;
  USHORT i;
  struct tt_post_table *post = NULL;

  if (!cmap_add)
    post = tt_read_post_table(sfont);

  for (count = 0, i = 0; i < 8192; i++) {
    int   j;
    int32_t  len;
    int  inbytesleft, outbytesleft;
    const unsigned char *inbuf;
    unsigned char *outbuf;

    if (used_glyphs[i] == 0)
      continue;

    for (j = 0; j < 8; j++) {
      USHORT gid = 8 * i + j;

      if (!is_used_char2(used_glyphs, gid))
        continue;

      if (!cmap_add) {
#define MAX_UNICODES	16
        /* try to look up Unicode values from the glyph name... */
        char* name;
        int32_t unicodes[MAX_UNICODES];
        int  unicode_count = -1;
        name = sfnt_get_glyphname(post, cffont, gid);
        if (name) {
          unicode_count = agl_get_unicodes(name, unicodes, MAX_UNICODES);
        }
#undef MAX_UNICODES
        if (unicode_count == -1) {
#if defined(LIBDPX)
          if(verbose > VERBOSE_LEVEL_MIN) {
            if (name)
              MESG("No Unicode mapping available: GID=%u, name=%s\n", gid, name);
            else
              MESG("No Unicode mapping available: GID=%u\n", gid);
          }
#else
          if (name)
            MESG("No Unicode mapping available: GID=%u, name=%s\n", gid, name);
          else
            MESG("No Unicode mapping available: GID=%u\n", gid);
#endif /* LIBDPX */
        } else {
          /* the Unicode characters go into wbuf[2] and following, in UTF16BE */
          /* we rely on WBUF_SIZE being more than adequate for MAX_UNICODES  */
          unsigned char* p = wbuf + 2;
          int  k;
          len = 0;
          for (k = 0; k < unicode_count; ++k) {
            len += UC_UTF16BE_encode_char(unicodes[k], &p, wbuf+WBUF_SIZE);
          }
          wbuf[0] = (gid >> 8) & 0xff;
          wbuf[1] =  gid & 0xff;
          CMap_add_bfchar(cmap, wbuf, 2, wbuf + 2, len);
        }
        RELEASE(name);
      } else {
        wbuf[0] = (gid >> 8) & 0xff;
        wbuf[1] =  gid & 0xff;

        inbuf        = wbuf;
        inbytesleft  = 2;
        outbuf       = wbuf + 2;
        outbytesleft = WBUF_SIZE - 2;
        CMap_decode(cmap_add, &inbuf, &inbytesleft, &outbuf, &outbytesleft);

        if (inbytesleft != 0) {
          WARN("CMap conversion failed...");
        } else {
          len = WBUF_SIZE - 2 - outbytesleft;
          CMap_add_bfchar(cmap, wbuf, 2, wbuf + 2, len);
          count++;

          if (verbose > VERBOSE_LEVEL_MIN) {
            int _i;

            MESG("otf_cmap>> Additional ToUnicode mapping: <%04X> <", gid);
            for (_i = 0; _i < len; _i++) {
              MESG("%02X", wbuf[2 + _i]);
            }
            MESG(">\n");
          }
        }
      }
    }
  }

  if (post)
    tt_release_post_table(post);

  return count;
}

static cff_font *
prepare_CIDFont_from_sfnt(sfnt* sfont)
{
  cff_font *cffont;
  unsigned  offset = 0;

  if (sfont->type != SFNT_TYPE_POSTSCRIPT     ||
      sfnt_read_table_directory(sfont, 0) < 0 ||
      (offset = sfnt_find_table_pos(sfont, "CFF ")) == 0) {
    return NULL;
  }

  cffont = cff_open(sfont->stream, offset, 0);
  if (!cffont)
    return NULL;

  cff_read_charsets(cffont);
  return cffont;
}

static USHORT
add_to_cmap_if_used (CMap *cmap,
                     cff_font *cffont,
                     char *used_chars,
                     USHORT gid,
                     ULONG ch)
{
  USHORT count = 0;
  USHORT cid = cffont ? cff_charsets_lookup_inverse(cffont, gid) : gid;
  if (is_used_char2(used_chars, cid)) {
    int len;
    unsigned char *p = wbuf + 2;

    count++;

    wbuf[0] = (cid >> 8) & 0xff;
    wbuf[1] = (cid & 0xff);
    len = UC_UTF16BE_encode_char((int32_t) ch, &p, wbuf + WBUF_SIZE);
    CMap_add_bfchar(cmap, wbuf, 2, wbuf + 2, len);

    /* Skip PUA characters and alphabetic presentation forms, allowing
     * handle_subst_glyphs() as it might find better mapping. Fixes the
     * mapping of ligatures encoded in PUA in fonts like Linux Libertine
     * and old Adobe fonts.
     */
    if (!is_PUA_or_presentation(ch)) {
      /* Avoid duplicate entry
       * There are problem when two Unicode code is mapped to
       * single glyph...
       */
      used_chars[cid / 8] &= ~(1 << (7 - (cid % 8)));
    }
  }

  return count;
}

static USHORT
create_ToUnicode_cmap4 (CMap *cmap,
                        struct cmap4 *map,
                        char *used_chars,
                        cff_font *cffont)
{
  USHORT count = 0, segCount = map->segCountX2 / 2;
  USHORT i, j;

  for (i = 0; i < segCount; i++) {
    USHORT c0 = map->startCount[i];
    USHORT c1 = map->endCount[i];
    USHORT d  = map->idRangeOffset[i] / 2 - (segCount - i);
    for (j = 0; j <= c1 - c0; j++) {
      USHORT ch = c0 + j;
      USHORT gid;

      if (map->idRangeOffset[i] == 0) {
        gid = (ch + map->idDelta[i]) & 0xffff;
      } else if (c0 == 0xffff && c1 == 0xffff && map->idRangeOffset[i] == 0xffff) {
        /* this is for protection against some old broken fonts... */
        gid = 0;
      } else {
        gid = (map->glyphIndexArray[j + d] + map->idDelta[i]) & 0xffff;
      }

      count += add_to_cmap_if_used(cmap, cffont, used_chars, gid, ch);
    }
  }

  return count;
}

static USHORT
create_ToUnicode_cmap12 (CMap *cmap,
                         struct cmap12 *map,
                         char *used_chars,
                         cff_font *cffont)
{
  ULONG i, ch, count = 0;

  for (i = 0; i < map->nGroups; i++) {
    for (ch  = map->groups[i].startCharCode;
         ch <= map->groups[i].endCharCode; ch++) {
      int d = ch - map->groups[i].startCharCode;
      USHORT gid = (USHORT) ((map->groups[i].startGlyphID + d) & 0xffff);
      count += add_to_cmap_if_used(cmap, cffont, used_chars, gid, ch);
    }
  }

  return count;
}

static pdf_obj *
create_ToUnicode_cmap (tt_cmap *ttcmap,
                       const char *cmap_name,
                       CMap *cmap_add,
                       const char *used_chars,
                       sfnt *sfont,
                       CMap *code_to_cid_cmap)
{
  pdf_obj  *stream = NULL;
  CMap     *cmap;
  USHORT    count = 0;
  cff_font *cffont = prepare_CIDFont_from_sfnt(sfont);
  char      is_cidfont = cffont && (cffont->flag & FONTTYPE_CIDFONT);

  cmap = CMap_new();
  CMap_set_name (cmap, cmap_name);
  CMap_set_wmode(cmap, 0);
  CMap_set_type (cmap, CMAP_TYPE_TO_UNICODE);
  CMap_set_CIDSysInfo(cmap, &CSI_UNICODE);
  CMap_add_codespacerange(cmap, srange_min, srange_max, 2);

  if (code_to_cid_cmap && cffont && is_cidfont) {
    USHORT i;
    for (i = 0; i < 8192; i++) {
      int j;

      if (used_chars[i] == 0)
        continue;

      for (j = 0; j < 8; j++) {
        USHORT cid = 8 * i + j;
        int ch;

        if (!is_used_char2(used_chars, cid))
          continue;

        ch = CMap_reverse_decode(code_to_cid_cmap, cid);
        if (ch >= 0) {
          int len;
          unsigned char *p = wbuf + 2;
          wbuf[0] = (cid >> 8) & 0xff;
          wbuf[1] =  cid & 0xff;
          len = UC_UTF16BE_encode_char(ch, &p, wbuf + WBUF_SIZE);
          CMap_add_bfchar(cmap, wbuf, 2, wbuf + 2, len);
          count++;
        }
      }
    }
  } else {
    char used_chars_copy[8192];
    memcpy(used_chars_copy, used_chars, 8192);

    /* For create_ToUnicode_cmap{4,12}(), cffont is for GID -> CID lookup,
     * so it is only needed for CID fonts. */
    switch (ttcmap->format) {
      case 4:
        count = create_ToUnicode_cmap4(cmap, ttcmap->map, used_chars_copy,
                                       is_cidfont ? cffont : NULL);
        break;
      case 12:
        count = create_ToUnicode_cmap12(cmap, ttcmap->map, used_chars_copy,
                                        is_cidfont ? cffont : NULL);
        break;
    }

    /* For handle_subst_glyphs(), cffont is for GID -> glyph name lookup, so
     * it is only needed for non-CID fonts. */
    count += handle_subst_glyphs(cmap, cmap_add, used_chars_copy, sfont,
                                 is_cidfont ? NULL : cffont);
  }

  if (count < 1)
    stream = NULL;
  else {
    stream = CMap_create_stream(cmap);
  }
  CMap_release(cmap);

  if (cffont)
    cff_close(cffont);

  return stream;
}

typedef struct {
  short platform;
  short encoding;
} cmap_plat_enc_rec;

static cmap_plat_enc_rec cmap_plat_encs[] = {
    { 3, 10 },
    { 0, 3 },
    { 0, 0 },
    { 3, 1 },
    { 0, 1 }
};

pdf_obj *
otf_create_ToUnicode_stream (const char *font_name,
                             int ttc_index, /* 0 for non-TTC */
                             const char *used_chars,
                             int cmap_id)
{
  pdf_obj    *cmap_ref = NULL;
  int         res_id;
  pdf_obj    *cmap_obj = NULL;
  CMap       *cmap_add, *code_to_cid_cmap;
  int         cmap_add_id;
  tt_cmap    *ttcmap;
  char       *normalized_font_name;
  char       *cmap_name;
  FILE       *fp = NULL;
  sfnt       *sfont;
  ULONG       offset = 0;
  int         i, cmap_type;

  /* replace slash in map name with dash to make the output cmap name valid,
   * happens when XeTeX embeds full font path
   * https://sourceforge.net/p/xetex/bugs/52/
   */
  normalized_font_name = NEW(strlen(font_name)+1, char);
  strcpy(normalized_font_name, font_name);
  for (i = 0; i < strlen(font_name); ++i) {
    if (normalized_font_name[i] == '/')
		normalized_font_name[i] = '-';
  }

  cmap_name = NEW(strlen(font_name)+strlen("-UTF16")+5, char);
  sprintf(cmap_name, "%s,%03d-UTF16", normalized_font_name, ttc_index);

  res_id = pdf_findresource("CMap", cmap_name);
  if (res_id >= 0) {
    RELEASE(cmap_name);
    cmap_ref = pdf_get_resource_reference(res_id);
    return cmap_ref;
  }

  if (verbose > VERBOSE_LEVEL_MIN) {
    MESG("\n");
    MESG("otf_cmap>> Creating ToUnicode CMap for \"%s\"...\n", font_name);
  }


  if ((fp = DPXFOPEN(font_name, DPX_RES_TYPE_TTFONT)) ||
      (fp = DPXFOPEN(font_name, DPX_RES_TYPE_OTFONT))) {
    sfont = sfnt_open(fp);
  } else if ((fp = DPXFOPEN(font_name, DPX_RES_TYPE_DFONT))) {
    sfont = dfont_open(fp, ttc_index);
  } else  {
    RELEASE(cmap_name);
    return NULL;
  }

  if (!sfont) {
    ERROR("Could not open OpenType/TrueType font file \"%s\"", font_name);
  }

  switch (sfont->type) {
  case SFNT_TYPE_DFONT:
    offset = sfont->offset;
    break;
  case SFNT_TYPE_TTC:
    offset = ttc_read_offset(sfont, ttc_index);
    if (offset == 0) {
      ERROR("Invalid TTC index");
    }
    break;
  default:
    offset = 0;
    break;
  }

  if (sfnt_read_table_directory(sfont, offset) < 0) {
    ERROR("Could not read OpenType/TrueType table directory.");
  }

  code_to_cid_cmap = CMap_cache_get(cmap_id);
  cmap_type = CMap_get_type(code_to_cid_cmap);
  if (cmap_type != CMAP_TYPE_CODE_TO_CID)
    code_to_cid_cmap = NULL;

  cmap_add_id = CMap_cache_find(cmap_name);
  if (cmap_add_id < 0) {
    cmap_add = NULL;
  } else {
    cmap_add = CMap_cache_get(cmap_add_id);
  }

  CMap_set_silent(1); /* many warnings without this... */
  for (i = 0; i < sizeof(cmap_plat_encs) / sizeof(cmap_plat_enc_rec); ++i) {
    ttcmap = tt_cmap_read(sfont, cmap_plat_encs[i].platform, cmap_plat_encs[i].encoding);
    if (!ttcmap)
      continue;

    if (ttcmap->format == 4 || ttcmap->format == 12) {
      cmap_obj = create_ToUnicode_cmap(ttcmap, cmap_name, cmap_add, used_chars,
                                       sfont, code_to_cid_cmap);
      break;
    }
  }
#if defined(LIBDPX)
  if (cmap_obj == NULL && verbose > VERBOSE_LEVEL_MIN)
#else
  if (cmap_obj == NULL)
#endif /* LIBDPX */
    WARN("Unable to read OpenType/TrueType Unicode cmap table.");
  tt_cmap_release(ttcmap);
  CMap_set_silent(0);

  if (cmap_obj) {
    res_id   = pdf_defineresource("CMap", cmap_name,
				  cmap_obj, PDF_RES_FLUSH_IMMEDIATE);
    cmap_ref = pdf_get_resource_reference(res_id);
  } else {
    cmap_ref = NULL;
  }
  RELEASE(cmap_name);

  sfnt_close(sfont);
  if (fp)
    DPXFCLOSE(fp);

  return cmap_ref;
}

/* Must be smaller than (WBUF_SIZE-2)/8 */
#define MAX_UNICODES 16

struct gent
{
  USHORT gid;
  int32_t ucv; /* assigned PUA unicode */

  int     num_unicodes;
  int32_t unicodes[MAX_UNICODES];
};

static void
create_cmaps (CMap *cmap, CMap *tounicode,
	      struct ht_table *unencoded, unsigned char *GIDToCIDMap)
{
  struct ht_iter iter;

  ASSERT(cmap && unencoded);

  if (ht_set_iter(unencoded, &iter) < 0)
    return;

  CMap_set_silent(1); /* many warnings without this... */

  do {
    struct gent   *glyph;
    unsigned char *ucv;
    int            i, len;
    unsigned char  *p, *endptr;
    CID            cid;

    glyph = (struct gent *)   ht_iter_getval(&iter);
    ucv   = (unsigned char *) ht_iter_getkey(&iter, &len);

    if (GIDToCIDMap) {
      cid = ((GIDToCIDMap[2 * glyph->gid] << 8)|GIDToCIDMap[2 * glyph->gid + 1]);
      if (cid == 0)
	WARN("Glyph gid=%u does not have corresponding CID.", glyph->gid);
    } else {
      cid = glyph->gid;
    }

    CMap_add_cidchar(cmap, ucv, 4, cid);

    if (tounicode) {
      wbuf[0] = (cid >> 8) & 0xff;
      wbuf[1] = cid & 0xff;
      p       = wbuf + 2;
      endptr  = wbuf + WBUF_SIZE;
      len     = 0;
      for (i = 0; i < glyph->num_unicodes; i++) {
	      len += UC_UTF16BE_encode_char(glyph->unicodes[i], &p, endptr);
      }
      CMap_add_bfchar(tounicode, wbuf, 2, wbuf + 2, len);
    }
  } while (ht_iter_next(&iter) >= 0);

  CMap_set_silent(0);

  ht_clear_iter(&iter);
}

static void
add_glyph (struct ht_table *unencoded,
	   USHORT gid, int32_t ucv, int num_unicodes, int32_t *unicodes)
{
  struct gent *glyph;
  int i;

  ASSERT(unencoded);

  if (gid == 0 || num_unicodes < 1) {
    return;
  }

  wbuf[0] = (ucv >> 24) & 0xff;
  wbuf[1] = (ucv >> 16) & 0xff;
  wbuf[2] = (ucv >>  8) & 0xff;
  wbuf[3] =  ucv & 0xff;

  glyph = NEW(1, struct gent);
  glyph->gid = gid;
  glyph->num_unicodes = num_unicodes;
  for (i = 0;
       i < num_unicodes && i < MAX_UNICODES; i++) {
    glyph->unicodes[i] = unicodes[i];
  }

  ht_append_table(unencoded, wbuf, 4, glyph);
}

/* This seriously affects speed... */
static struct gent *
find_glyph (struct ht_table *unencoded, int32_t ucv)
{
  ASSERT(unencoded);

  wbuf[0] = (ucv >> 24) & 0xff;
  wbuf[1] = (ucv >> 16) & 0xff;
  wbuf[2] = (ucv >>  8) & 0xff;
  wbuf[3] =  ucv & 0xff;

  return (struct gent *) ht_lookup_table(unencoded, wbuf, 4);
}

static void
handle_subst (pdf_obj *dst_obj, pdf_obj *src_obj, int flag,
	      otl_gsub *gsub_list, tt_cmap *ttcmap,
	      struct ht_table *unencoded)
{
  pdf_obj *tmp;
  int32_t     i, j, src_size, dst_size;
  int32_t     src, dst;
  int32_t     src_start, src_end, dst_start, dst_end;

  src_size = pdf_array_length(src_obj);
  dst_size = pdf_array_length(dst_obj);

  dst_start = dst_end = -1; dst = 0;
  src_start = src_end = -1; src = 0;
  for (i = 0, j = 0;
       i < src_size && j < dst_size; i++) {
    USHORT       gid;
    int          rv;
    struct gent *glyph;

    tmp = pdf_get_array(src_obj, i);
    if (PDF_OBJ_ARRAYTYPE(tmp)) {
      src_start = (int32_t) pdf_number_value(pdf_get_array(tmp, 0));
      src_end   = (int32_t) pdf_number_value(pdf_get_array(tmp, 1));
    } else {
      src_start = src_end = (int32_t) pdf_number_value(tmp);
    }
    for (src = src_start; src <= src_end; src++) {
      glyph = find_glyph(unencoded, src);
      if (glyph)
	gid = glyph->gid;
      else {
 	gid = tt_cmap_lookup(ttcmap, src);
      }
      dst++;
      if (dst > dst_end) {
	tmp = pdf_get_array(dst_obj, j++);
	if (PDF_OBJ_ARRAYTYPE(tmp)) {
	  dst_start = (int32_t) pdf_number_value(pdf_get_array(tmp, 0));
	  dst_end   = (int32_t) pdf_number_value(pdf_get_array(tmp, 1));
	} else {
	  dst_start = dst_end = (int32_t) pdf_number_value(tmp);
	}
	dst = dst_start;
      }
      if (gid == 0) {
	if (flag == 'r' || flag == 'p') {
	  if (src < 0x10000) {
	    WARN("Font does not have glyph for U+%04X.", src);
	  } else {
	    WARN("Font does not have glyph for U+%06X.", src);
	  }
	}
	if (flag == 'r') {
	  ERROR("Missing glyph found...");
	}
	continue;
      }
      rv = otl_gsub_apply(gsub_list, &gid);
      if (rv < 0) {
	if (flag == 'p' || flag == 'r') {
	  if (src < 0x10000) {
	    WARN("No substituted glyph for U+%04X.", src);
	  } else {
	    WARN("No substituted glyph for U+%06X.", src);
	  }
	}
	if (flag == 'r') {
	  ERROR("Missing glyph found...");
	}
	continue;
      }

      if (glyph) {
	glyph->gid = gid;
      } else {
	add_glyph(unencoded, gid, dst, 1, &src);
      }

      if (verbose > VERBOSE_LEVEL_MIN) {
	if (dst < 0x10000) {
	  MESG("otf_cmap>> Substituted glyph gid=%u assigned to U+%04X\n",
	       gid, dst);
	} else {
	  MESG("otf_cmap>> Substituted glyph gid=%u assigned to U+%06X\n",
	       gid, dst);
	}
      }

    }
  }

  if (dst < dst_end || src < src_end ||
      i < src_size  || j < dst_size) {
    WARN("Number of glyphs in left-side and right-side not equal...");
    WARN("Please check .otl file...");
  }
}

static void
handle_assign (pdf_obj *dst, pdf_obj *src, int flag,
	       otl_gsub *gsub_list, tt_cmap *ttcmap,
	       struct ht_table *unencoded)
{
  int32_t  unicodes[MAX_UNICODES], ucv;
  int      i, n_unicodes, rv;
  USHORT   gid_in[MAX_UNICODES], lig;

  n_unicodes = pdf_array_length(src); /* FIXME */
  ucv = (int32_t) pdf_number_value(pdf_get_array(dst, 0)); /* FIXME */
  if (!UC_is_valid(ucv)) {
    if (flag == 'r' || flag == 'p') {
      if (ucv < 0x10000) {
	WARN("Invalid Unicode in: %04X", ucv);
      } else {
	WARN("Invalid Unicode in: %06X", ucv);
      }
    }
    if (flag == 'r') {
      ERROR("Invalid Unicode code specified.");
    }
    return;
  }

  if (verbose > VERBOSE_LEVEL_MIN) {
    MESG("otf_cmap>> Ligature component:");
  }

  for (i = 0; i < n_unicodes; i++) {
    unicodes[i] =
      (int32_t) pdf_number_value(pdf_get_array(src, i));
    gid_in[i] = tt_cmap_lookup(ttcmap, unicodes[i]);

    if (verbose > VERBOSE_LEVEL_MIN) {
      if (unicodes[i] < 0x10000) {
	MESG(" U+%04X (gid=%u)", unicodes[i], gid_in[i]);
      } else {
	MESG(" U+%06X (gid=%u)", unicodes[i], gid_in[i]);
      }
    }

    if (gid_in[i] == 0) {
      if (flag == 'r' || flag == 'p') {
	if (unicodes[i] < 0x10000) {
	  WARN("Unicode char U+%04X not exist in font...", unicodes[i]);
	} else {
	  WARN("Unicode char U+%06X not exist in font...", unicodes[i]);
	}
      }
      if (flag == 'r') {
	ERROR("Missing glyph found...");
      }
      return;
    }

  }
 
  if (verbose > VERBOSE_LEVEL_MIN) {
    MESG("\n");
  }

  rv = otl_gsub_apply_lig(gsub_list,
			  gid_in, (USHORT)n_unicodes, &lig);
  if (rv < 0) {
    if (flag == 'p')
      WARN("No ligature found...");
    else if (flag == 'r')
      ERROR("No ligature found...");
    return;
  }

  add_glyph(unencoded, lig, ucv, n_unicodes, unicodes);

  if (verbose > VERBOSE_LEVEL_MIN) {
    if (ucv < 0x10000) {
      MESG("otf_cmap>> Ligature glyph gid=%u assigned to U+%04X\n", lig, ucv);
    } else {
      MESG("otf_cmap>> Ligature glyph gid=%u assigned to U+%06X\n", lig, ucv);
    }
  }

  return;
}

static int
load_base_CMap (const char *cmap_name, int wmode,
		CIDSysInfo *csi, unsigned char *GIDToCIDMap,
		tt_cmap *ttcmap)
{
  int cmap_id;

  cmap_id = CMap_cache_find(cmap_name);
  if (cmap_id < 0) {
    CMap  *cmap;

    cmap = CMap_new();
    CMap_set_name (cmap, cmap_name);
    CMap_set_type (cmap, CMAP_TYPE_CODE_TO_CID);
    CMap_set_wmode(cmap, wmode);
    CMap_add_codespacerange(cmap, lrange_min, lrange_max, 4);

    if (csi) { /* CID */
      CMap_set_CIDSysInfo(cmap, csi);
    } else {
      CMap_set_CIDSysInfo(cmap, &CSI_IDENTITY);
    }

    if (ttcmap->format == 12) {
      load_cmap12(ttcmap->map, GIDToCIDMap, cmap);
    } else if (ttcmap->format == 4) {
      load_cmap4(ttcmap->map, GIDToCIDMap, cmap);
    }

    cmap_id = CMap_cache_add(cmap);
  }

  return cmap_id;
}

static void
load_gsub (pdf_obj *conf, otl_gsub *gsub_list, sfnt *sfont)
{
  pdf_obj  *rule;
  char     *script, *language, *feature;
  int       i, size;

  rule = otl_conf_get_rule(conf);
  if (!rule)
    return;

  script   = otl_conf_get_script  (conf);
  language = otl_conf_get_language(conf);

  size     = pdf_array_length(rule);
  for (i = 0; i < size; i += 2) {
    pdf_obj   *tmp, *commands;
    int        flag;
    int        j, num_comms;

    tmp  = pdf_get_array(rule, i);
    flag = (int) pdf_number_value(tmp);

    commands  = pdf_get_array(rule, i+1);
    num_comms = pdf_array_length(commands);

    /* (assign|substitute) tag dst src */
    for (j = 0 ; j < num_comms; j += 4) {
      tmp = pdf_get_array(commands, 1);
      if (PDF_OBJ_STRINGTYPE(tmp)) {
	feature = pdf_string_value(tmp);
	if (otl_gsub_add_feat(gsub_list,
			      script, language, feature, sfont) < 0) {
	  if (flag == 'p')
	    WARN("No OTL feature matches \"%s.%s.%s\" found.",
		 script, language, feature);
	  else if (flag == 'r')
	    ERROR("No OTL feature matches \"%s.%s.%s\" found.",
		  script, language, feature);
	}
      }

    }
  }

}

static void
handle_gsub (pdf_obj *conf,
	     tt_cmap *ttcmap, otl_gsub *gsub_list,
	     struct ht_table *unencoded)
{
  pdf_obj *rule;
  char    *script, *language, *feature;
  int      i, size;

  if (!conf)
    return;

  rule = otl_conf_get_rule(conf);
  if (!rule) {
    return;
  }

  if (!PDF_OBJ_ARRAYTYPE(rule)) {
    WARN("Not arraytype?");
    return;
  }
  script   = otl_conf_get_script  (conf);
  language = otl_conf_get_language(conf);

  size = pdf_array_length(rule);
  for (i = 0; i < size; i += 2) {
    pdf_obj  *tmp, *commands;
    int       j, num_comms;
    int       flag;

    tmp  = pdf_get_array(rule, i);
    flag = (int) pdf_number_value(tmp);

    commands  = pdf_get_array   (rule, i+1);
    num_comms = pdf_array_length(commands);

    for (j = 0; j < num_comms; j += 4) {
      pdf_obj *operator;
      pdf_obj *src, *dst, *feat;
      int      rv;

      /* (assing|substitute) tag dst src */
      operator = pdf_get_array(commands, j);

      feat     = pdf_get_array(commands, j+1);
      if (PDF_OBJ_STRINGTYPE(feat))
	feature = pdf_string_value(feat);
      else
	feature = NULL;

      dst  = pdf_get_array(commands, j+2);
      src  = pdf_get_array(commands, j+3);

      rv = otl_gsub_select(gsub_list, script, language, feature);
      if (rv < 0) {
	if (flag == 'p') {
	  WARN("No GSUB feature %s.%s.%s loaded...",
	       script, language, feature);
	} else if (flag == 'r') {
	  ERROR("No GSUB feature %s.%s.%s loaded...",
		script, language, feature);
	}
      } else {

	if (verbose > VERBOSE_LEVEL_MIN) {
	  MESG("otf_cmap>> %s:\n", pdf_name_value(operator));
	}

	if (!strcmp(pdf_name_value(operator), "assign")) {
	  handle_assign(dst, src, flag,
			gsub_list, ttcmap, unencoded);
	} else if (!strcmp(pdf_name_value(operator), "substitute")) {
	  handle_subst(dst, src, flag,
		       gsub_list, ttcmap, unencoded);
	}
      }

    }

  }

}

static inline void
hval_free (void *hval)
{
  RELEASE(hval);
}

int
otf_load_Unicode_CMap (const char *map_name, int ttc_index, /* 0 for non-TTC font */
		       const char *otl_tags, int wmode)
{
  int    cmap_id = -1;
  int    tounicode_id = -1, is_cidfont = 0;
  sfnt  *sfont;
  ULONG  offset = 0;
  char  *base_name = NULL, *cmap_name = NULL;
  char  *tounicode_name = NULL;
  FILE  *fp = NULL;
  otl_gsub      *gsub_list = NULL;
  tt_cmap       *ttcmap;
  CMap          *cmap, *base, *tounicode = NULL;
  CIDSysInfo     csi = {NULL, NULL, 0};
  unsigned char *GIDToCIDMap = NULL;

  if (!map_name)
    return -1;

  if (ttc_index > 999 || ttc_index < 0) {
    return -1; /* Sorry for this... */
  }

  fp = DPXFOPEN(map_name, DPX_RES_TYPE_TTFONT);
  if (!fp) {
    fp = DPXFOPEN(map_name, DPX_RES_TYPE_OTFONT);
  }
  if (!fp) {
    fp = DPXFOPEN(map_name, DPX_RES_TYPE_DFONT);
    if (!fp) return -1;
    sfont = dfont_open(fp, ttc_index);
  } else {
    sfont = sfnt_open(fp);
  }

  if (!sfont) {
    ERROR("Could not open OpenType/TrueType/dfont font file \"%s\"", map_name);
  }
  switch (sfont->type) {
  case SFNT_TYPE_TTC:
    offset = ttc_read_offset(sfont, ttc_index);
    if (offset == 0) {
      ERROR("Invalid TTC index");
    }
    break;
  case SFNT_TYPE_TRUETYPE:
  case SFNT_TYPE_POSTSCRIPT:
    offset = 0;
    break;
  case SFNT_TYPE_DFONT:
    offset = sfont->offset;
    break;
  default:
    ERROR("Not a OpenType/TrueType/TTC font?: %s", map_name);
    break;
  }

  if (sfnt_read_table_directory(sfont, offset) < 0)
    ERROR("Could not read OpenType/TrueType table directory.");

  base_name = NEW(strlen(map_name)+strlen("-UCS4-H")+5, char);
  if (wmode)
    sprintf(base_name, "%s,%03d-UCS4-V", map_name, ttc_index);
  else {
    sprintf(base_name, "%s,%03d-UCS4-H", map_name, ttc_index);
  }

  if (otl_tags) {
    cmap_name = NEW(strlen(map_name)+strlen(otl_tags)+strlen("-UCS4-H")+6, char);
    if (wmode)
      sprintf(cmap_name, "%s,%03d,%s-UCS4-V", map_name, ttc_index, otl_tags);
    else
      sprintf(cmap_name, "%s,%03d,%s-UCS4-H", map_name, ttc_index, otl_tags);
  } else {
    cmap_name = NEW(strlen(base_name)+1, char);
    strcpy(cmap_name, base_name);
  }

  if (sfont->type == SFNT_TYPE_POSTSCRIPT) {
    is_cidfont = handle_CIDFont(sfont, &GIDToCIDMap, &csi);
  } else {
    is_cidfont = 0;
  }

  if (is_cidfont) {
    tounicode_name = NULL;
  } else {
    tounicode_name = NEW(strlen(map_name)+strlen("-UTF16")+5, char);
    sprintf(tounicode_name, "%s,%03d-UTF16", map_name, ttc_index);
  }

  if (verbose > VERBOSE_LEVEL_MIN) {
    MESG("\n");
    MESG("otf_cmap>> Unicode charmap for font=\"%s\" layout=\"%s\"\n",
	 map_name, (otl_tags ? otl_tags : "none"));
  }

  cmap_id = CMap_cache_find(cmap_name);
  if (cmap_id >= 0) {
    RELEASE(cmap_name);
    RELEASE(base_name);
    if (GIDToCIDMap)
      RELEASE(GIDToCIDMap);
    if (tounicode_name)
      RELEASE(tounicode_name);

    sfnt_close(sfont);
    DPXFCLOSE(fp);

    if (verbose > VERBOSE_LEVEL_MIN)
      MESG("otf_cmap>> Found at cmap_id=%d.\n", cmap_id);

    return cmap_id;
  }

  ttcmap = tt_cmap_read(sfont, 3, 10); /* Microsoft UCS4 */
  if (!ttcmap) {
    ttcmap = tt_cmap_read(sfont, 3, 1); /* Microsoft UCS2 */
    if (!ttcmap) {
      ttcmap = tt_cmap_read(sfont, 0, 3); /* Unicode 2.0 or later */
#if defined(LIBDPX)
      if (!ttcmap && verbose > VERBOSE_LEVEL_MIN) {
#else
      if (!ttcmap) {
#endif /* LIBDPX */
        ERROR("Unable to read OpenType/TrueType Unicode cmap table.");
      }
    }
  }
  cmap_id = load_base_CMap(base_name, wmode,
			   (is_cidfont ? &csi : NULL),
			   GIDToCIDMap, ttcmap);
  if (cmap_id < 0)
    ERROR("Failed to read OpenType/TrueType cmap table.");

  if (!otl_tags) {
    RELEASE(cmap_name);
    RELEASE(base_name);
    if (GIDToCIDMap)
      RELEASE(GIDToCIDMap);
    if (tounicode_name)
      RELEASE(tounicode_name);
    if (is_cidfont) {
      if (csi.registry)
	RELEASE(csi.registry);
      if (csi.ordering)
	RELEASE(csi.ordering);
    }
    tt_cmap_release(ttcmap);
    sfnt_close(sfont);
    DPXFCLOSE(fp);

    return cmap_id;
  }

  base = CMap_cache_get(cmap_id);

  cmap = CMap_new();
  CMap_set_name (cmap, cmap_name);
  CMap_set_type (cmap, CMAP_TYPE_CODE_TO_CID);
  CMap_set_wmode(cmap, wmode);
  /* CMap_add_codespacerange(cmap, lrange_min, lrange_max, 4); */
  CMap_set_usecmap(cmap, base);
  CMap_add_cidchar(cmap, lrange_max, 4, 0); /* FIXME */

  if (is_cidfont) {
    CMap_set_CIDSysInfo(cmap, &csi);
    if (csi.registry)
      RELEASE(csi.registry);
    if (csi.ordering)
      RELEASE(csi.ordering);
  } else {
    CMap_set_CIDSysInfo(cmap, &CSI_IDENTITY);
  }

  gsub_list = otl_gsub_new();

  {
    struct ht_table unencoded;
    char    *conf_name, *opt_tag;
    pdf_obj *conf, *opt_conf;

    conf_name = NEW(strlen(otl_tags)+1, char);
    memset (conf_name, 0, strlen(otl_tags)+1);
    opt_tag  = strchr(otl_tags, ':');
    if (opt_tag) {
      opt_tag++;
      strncpy(conf_name, otl_tags,
	      strlen(otl_tags) - strlen(opt_tag) - 1);
    } else {
      strcpy(conf_name, otl_tags);
    }

    if (verbose > VERBOSE_LEVEL_MIN) {
      MESG("otf_cmap>> Read layout config. \"%s\"\n", conf_name);
    }

    conf = otl_find_conf(conf_name);
    if (!conf)
      ERROR("Layout file \"%s\" not found...", conf_name);

    load_gsub(conf, gsub_list, sfont);
    if (opt_tag) {
      if (verbose > VERBOSE_LEVEL_MIN) {
	MESG("otf_cmap>> Layout option \"%s\" enabled\n", opt_tag);
      }
      opt_conf = otl_conf_find_opt(conf, opt_tag);
      if (!opt_conf)
	ERROR("There is no option \"%s\" in \"%s\".",
	      opt_tag, conf_name);
      load_gsub(opt_conf, gsub_list, sfont);
    }

    ht_init_table(&unencoded, hval_free);

    handle_gsub(conf, ttcmap, gsub_list, &unencoded);
    if (opt_tag) {
      opt_conf = otl_conf_find_opt(conf, opt_tag);
      if (!opt_conf)
	ERROR("There is no option \"%s\" in \"%s\".",
	      opt_tag, conf_name);
      handle_gsub(opt_conf, ttcmap, gsub_list, &unencoded);
    }
    if (is_cidfont) {
      tounicode_id = -1;
      tounicode    = NULL;
    } else {
      tounicode_id = CMap_cache_find(tounicode_name);
      if (tounicode_id >= 0)
	tounicode  = CMap_cache_get(tounicode_id);
      else {
	tounicode = CMap_new();
	CMap_set_name (tounicode, tounicode_name);
	CMap_set_type (tounicode, CMAP_TYPE_TO_UNICODE);
	CMap_set_wmode(tounicode, 0);
	CMap_add_codespacerange(tounicode, srange_min, srange_max, 2);
	CMap_set_CIDSysInfo(tounicode, &CSI_UNICODE);
	/* FIXME */
	CMap_add_bfchar(tounicode, srange_min, 2, srange_max, 2);
      }
    }
    create_cmaps(cmap, tounicode, &unencoded, GIDToCIDMap);

    ht_clear_table(&unencoded);
    RELEASE(conf_name);
  }

  cmap_id = CMap_cache_add(cmap);
  if (!is_cidfont && tounicode_id < 0) /* New */
    CMap_cache_add(tounicode);

  tt_cmap_release(ttcmap);
  if (gsub_list)
    otl_gsub_release(gsub_list);

  if (verbose > VERBOSE_LEVEL_MIN) {
    MESG("otf_cmap>> Overwrite CMap \"%s\" by \"%s\" with usecmap\n",
	 base_name, cmap_name);
  }

  if (GIDToCIDMap)
    RELEASE(GIDToCIDMap);
  if (base_name)
    RELEASE(base_name);
  if (cmap_name)
    RELEASE(cmap_name);
  if (tounicode_name)
    RELEASE(tounicode_name);

  sfnt_close(sfont);
  DPXFCLOSE(fp);

  return cmap_id;
}
