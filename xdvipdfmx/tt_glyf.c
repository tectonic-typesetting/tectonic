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
 * Subsetting glyf, updating loca, hmtx, ...
 *
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "system.h"
#include "mem.h"
#include "error.h"
#include "dpxutil.h"

#include "sfnt.h"
#include "tt_table.h"
#include "tt_glyf.h"

#define NUM_GLYPH_LIMIT        65534
#define TABLE_DATA_ALLOC_SIZE  40960
#define GLYPH_ARRAY_ALLOC_SIZE 256

static USHORT
find_empty_slot (struct tt_glyphs *g)
{
  USHORT gid;

  ASSERT(g);

  for (gid = 0; gid < NUM_GLYPH_LIMIT; gid++) {
    if (!(g->used_slot[gid/8] & (1 << (7 - (gid % 8)))))
      break;
  }
  if (gid == NUM_GLYPH_LIMIT)
    ERROR("No empty glyph slot available.");

  return gid;
}

USHORT
tt_find_glyph (struct tt_glyphs *g, USHORT gid)
{
  USHORT idx, new_gid = 0;

  ASSERT(g);

  for (idx = 0; idx < g->num_glyphs; idx++) {
    if (gid == g->gd[idx].ogid) {
      new_gid = g->gd[idx].gid;
      break;
    }
  }

  return new_gid;
}

USHORT
tt_get_index (struct tt_glyphs *g, USHORT gid)
{
  USHORT idx;

  ASSERT(g);

  for (idx = 0; idx < g->num_glyphs; idx++) {
    if (gid == g->gd[idx].gid)
      break;
  }
  if (idx == g->num_glyphs)
    idx = 0;

  return idx;
}

USHORT
tt_add_glyph (struct tt_glyphs *g, USHORT gid, USHORT new_gid)
{
  ASSERT(g);

  if (g->used_slot[new_gid/8] & (1 << (7 - (new_gid % 8)))) {
    WARN("Slot %u already used.", new_gid);
  } else {
    if (g->num_glyphs+1 >= NUM_GLYPH_LIMIT)
      ERROR("Too many glyphs.");

    if (g->num_glyphs >= g->max_glyphs) {
      g->max_glyphs += GLYPH_ARRAY_ALLOC_SIZE;
      g->gd = RENEW(g->gd, g->max_glyphs, struct tt_glyph_desc);
    }
    g->gd[g->num_glyphs].gid  = new_gid;
    g->gd[g->num_glyphs].ogid = gid;
    g->gd[g->num_glyphs].length = 0;
    g->gd[g->num_glyphs].data   = NULL;
    g->used_slot[new_gid/8] |= (1 << (7 - (new_gid % 8)));
    g->num_glyphs += 1;
  }

  if (new_gid > g->last_gid) {
    g->last_gid = new_gid;
  }

  return new_gid;
}

/*
 * Initialization
 */
struct tt_glyphs *
tt_build_init (void)
{
  struct tt_glyphs *g;

  g = NEW(1, struct tt_glyphs);

  g->num_glyphs  = 0;
  g->max_glyphs  = 0;
  g->last_gid    = 0;
  g->emsize      = 1;
  g->default_advh = 0;
  g->default_tsb  = 0;
  g->gd = NULL;
  g->used_slot = NEW(8192, unsigned char);
  memset(g->used_slot, 0, 8192);
  tt_add_glyph(g, 0, 0);

  return g;
}

void
tt_build_finish (struct tt_glyphs *g)
{
  if (g) {
    if (g->gd) {
      USHORT idx;
      for (idx = 0; idx < g->num_glyphs; idx++) {
	if (g->gd[idx].data)
	  RELEASE(g->gd[idx].data);
      }
      RELEASE(g->gd);
    }
    if (g->used_slot)
      RELEASE(g->used_slot);
    RELEASE(g);
  }
}

static inline int 
glyf_cmp (const void *v1, const void *v2)
{
  int cmp = 0;
  const struct tt_glyph_desc *sv1, *sv2;

  sv1 = (const struct tt_glyph_desc *) v1;
  sv2 = (const struct tt_glyph_desc *) v2;

  if (sv1->gid == sv2->gid)
    cmp = 0;
  else if (sv1->gid < sv2->gid)
    cmp = -1;
  else
    cmp = 1;
    
  return cmp;
}

/*
 * TrueType outline data.
 */
#define ARG_1_AND_2_ARE_WORDS     (1 << 0)
#define ARGS_ARE_XY_VALUES        (1 << 1)
#define ROUND_XY_TO_GRID          (1 << 2)
#define WE_HAVE_A_SCALE           (1 << 3)
#define RESERVED                  (1 << 4)
#define MORE_COMPONENT            (1 << 5)
#define WE_HAVE_AN_X_AND_Y_SCALE  (1 << 6)
#define WE_HAVE_A_TWO_BY_TWO      (1 << 7)
#define WE_HAVE_INSTRUCTIONS      (1 << 8)
#define USE_MY_METRICS            (1 << 9)

int
tt_build_tables (sfnt *sfont, struct tt_glyphs *g)
{
  char  *hmtx_table_data = NULL, *loca_table_data = NULL;
  char  *glyf_table_data = NULL;
  ULONG  hmtx_table_size, loca_table_size, glyf_table_size;
  /* some information available from other TrueType table */
  struct tt_head_table *head = NULL;
  struct tt_hhea_table *hhea = NULL;
  struct tt_maxp_table *maxp = NULL;
  struct tt_longMetrics *hmtx, *vmtx = NULL;
  struct tt_os2__table  *os2;
  /* temp */
  ULONG  *location, offset;
  int     i;
  USHORT *w_stat; /* Estimate most frequently appeared width */

  ASSERT(g);

  if (sfont == NULL || sfont->stream == NULL)
    ERROR("File not opened.");

  if (sfont->type != SFNT_TYPE_TRUETYPE &&
      sfont->type != SFNT_TYPE_TTC &&
      sfont->type != SFNT_TYPE_DFONT)
    ERROR("Invalid font type");

  if (g->num_glyphs > NUM_GLYPH_LIMIT)
    ERROR("Too many glyphs.");

  /*
   * Read head, hhea, maxp, loca:
   *
   *   unitsPerEm       --> head
   *   numHMetrics      --> hhea
   *   indexToLocFormat --> head
   *   numGlyphs        --> maxp
   */
  head = tt_read_head_table(sfont);
  hhea = tt_read_hhea_table(sfont);
  maxp = tt_read_maxp_table(sfont);

  if (hhea->metricDataFormat != 0)
    ERROR("Unknown metricDataFormat.");

  g->emsize = head->unitsPerEm;

  sfnt_locate_table(sfont, "hmtx");
  hmtx = tt_read_longMetrics(sfont, maxp->numGlyphs, hhea->numOfLongHorMetrics, hhea->numOfExSideBearings);

  os2 = tt_read_os2__table(sfont);
  if (os2) {
    g->default_advh = os2->sTypoAscender - os2->sTypoDescender;
    g->default_tsb  = g->default_advh - os2->sTypoAscender;
  }

  if (sfnt_find_table_pos(sfont, "vmtx") > 0) {
    struct tt_vhea_table *vhea;
    vhea = tt_read_vhea_table(sfont);
    sfnt_locate_table(sfont, "vmtx");
    vmtx = tt_read_longMetrics(sfont, maxp->numGlyphs, vhea->numOfLongVerMetrics, vhea->numOfExSideBearings);
    RELEASE(vhea);
  } else {
    vmtx = NULL;
  }

  sfnt_locate_table(sfont, "loca");
  location = NEW(maxp->numGlyphs + 1, ULONG);
  if (head->indexToLocFormat == 0) {
    for (i = 0; i <= maxp->numGlyphs; i++)
      location[i] = 2*((ULONG) sfnt_get_ushort(sfont));
  } else if (head->indexToLocFormat == 1) {
    for (i = 0; i <= maxp->numGlyphs; i++)
      location[i] = sfnt_get_ulong(sfont);
  } else {
    ERROR("Unknown IndexToLocFormat.");
  }

  w_stat = NEW(g->emsize+2, USHORT);
  memset(w_stat, 0, sizeof(USHORT)*(g->emsize+2));
  /*
   * Read glyf table.
   */
  offset = sfnt_locate_table(sfont, "glyf");
  /*
   * The num_glyphs may grow when composite glyph is found.
   * A component of glyph refered by a composite glyph is appended
   * to used_glyphs if it is not already registered in used_glyphs.
   * Glyph programs of composite glyphs are modified so that it
   * correctly refer to new gid of their components.
   */
  for (i = 0; i < NUM_GLYPH_LIMIT; i++) {
    USHORT gid;     /* old gid */
    ULONG  loc, len;
    BYTE  *p, *endptr;
    SHORT  number_of_contours;

    if (i >= g->num_glyphs) /* finished */
      break;

    gid = g->gd[i].ogid;
    if (gid >= maxp->numGlyphs)
      ERROR("Invalid glyph index (gid %u)", gid);

    loc = location[gid];
    len = location[gid+1] - loc;
    g->gd[i].advw = hmtx[gid].advance;
    g->gd[i].lsb  = hmtx[gid].sideBearing;
    if (vmtx) {
      g->gd[i].advh = vmtx[gid].advance;
      g->gd[i].tsb  = vmtx[gid].sideBearing;
    } else {
      g->gd[i].advh = g->default_advh;
      g->gd[i].tsb  = g->default_tsb;
    }
    g->gd[i].length = len;
    g->gd[i].data   = NULL;
    if (g->gd[i].advw <= g->emsize) {
      w_stat[g->gd[i].advw] += 1;
    } else {
      w_stat[g->emsize+1]   += 1; /* larger than em */
    }

    if (len == 0) { /* Does not contains any data. */
      continue;
    } else if (len < 10) {
      ERROR("Invalid TrueType glyph data (gid %u).", gid);
    }

    g->gd[i].data = p = NEW(len, BYTE);
    endptr = p + len;

    sfnt_seek_set(sfont, offset+loc);
    number_of_contours = sfnt_get_short(sfont);
    p += sfnt_put_short(p, number_of_contours);

    /* BoundingBox: FWord x 4 */
    g->gd[i].llx = sfnt_get_short(sfont);
    g->gd[i].lly = sfnt_get_short(sfont);
    g->gd[i].urx = sfnt_get_short(sfont);
    g->gd[i].ury = sfnt_get_short(sfont);
    /* _FIXME_ */
#if  1
    if (!vmtx) /* vertOriginY == sTypeAscender */
      g->gd[i].tsb = g->default_advh - g->default_tsb - g->gd[i].ury;
#endif
    p += sfnt_put_short(p, g->gd[i].llx);
    p += sfnt_put_short(p, g->gd[i].lly);
    p += sfnt_put_short(p, g->gd[i].urx);
    p += sfnt_put_short(p, g->gd[i].ury);

    /* Read evrything else. */
    sfnt_read(p, len - 10, sfont);
    /*
     * Fix GIDs of composite glyphs.
     */
    if (number_of_contours < 0) {
      USHORT flags, cgid, new_gid; /* flag, gid of a component */
      do {
	if (p >= endptr)
	  ERROR("Invalid TrueType glyph data (gid %u): %u bytes", gid, len);
	/*
	 * Flags and gid of component glyph are both USHORT.
	 */
	flags = ((*p) << 8)| *(p+1);
	p += 2;
	cgid  = ((*p) << 8)| *(p+1);
	if (cgid >= maxp->numGlyphs) {
	  ERROR("Invalid gid (%u > %u) in composite glyph %u.", cgid, maxp->numGlyphs, gid);
	}
	new_gid = tt_find_glyph(g, cgid);
	if (new_gid == 0) {
	  new_gid = tt_add_glyph(g, cgid, find_empty_slot(g));
	}
	p += sfnt_put_ushort(p, new_gid);
	/*
	 * Just skip remaining part.
	 */
	p += (flags & ARG_1_AND_2_ARE_WORDS) ? 4 : 2;
	if (flags & WE_HAVE_A_SCALE) /* F2Dot14 */
	  p += 2;
	else if (flags & WE_HAVE_AN_X_AND_Y_SCALE) /* F2Dot14 x 2 */
	  p += 4;
	else if (flags & WE_HAVE_A_TWO_BY_TWO) /* F2Dot14 x 4 */
	  p += 8;
      } while (flags & MORE_COMPONENT);
      /*
       * TrueType instructions comes here:
       *  length_of_instruction (ushort)
       *  instruction (byte * length_of_instruction)
       */
    }
  }
  RELEASE(location);
  RELEASE(hmtx);
  if (vmtx)
    RELEASE(vmtx);

  {
    int max_count = -1;

    g->dw = g->gd[0].advw;
    for (i = 0; i < g->emsize + 1; i++) {
      if (w_stat[i] > max_count) {
	max_count = w_stat[i];
	g->dw = i;
      }
    }
  }
  RELEASE(w_stat);

  qsort(g->gd, g->num_glyphs, sizeof(struct tt_glyph_desc), glyf_cmp);
  {
    USHORT prev, last_advw;
    char  *p, *q;
    int    padlen, num_hm_known;

    glyf_table_size = 0UL;
    num_hm_known = 0;
    last_advw = g->gd[g->num_glyphs - 1].advw;
    for (i = g->num_glyphs - 1; i >= 0; i--) {
      padlen = (g->gd[i].length % 4) ? (4 - (g->gd[i].length % 4)) : 0;
      glyf_table_size += g->gd[i].length + padlen;
      if (!num_hm_known && last_advw != g->gd[i].advw) {
	hhea->numOfLongHorMetrics = g->gd[i].gid + 2;
	num_hm_known = 1;
      }
    }
    /* All advance widths are same. */
    if (!num_hm_known) {
      hhea->numOfLongHorMetrics = 1;
    }
    hmtx_table_size = hhea->numOfLongHorMetrics * 2 + (g->last_gid + 1) * 2;

    /*
     * Choosing short format does not always give good result
     * when compressed. Sometimes increases size.
     */
    if (glyf_table_size < 0x20000UL) {
      head->indexToLocFormat = 0;
      loca_table_size = (g->last_gid + 2)*2;
    } else {
      head->indexToLocFormat = 1;
      loca_table_size = (g->last_gid + 2)*4;
    }

    hmtx_table_data = p = NEW(hmtx_table_size, char);
    loca_table_data = q = NEW(loca_table_size, char);
    glyf_table_data = NEW(glyf_table_size, char);

    offset = 0UL; prev = 0;
    for (i = 0; i < g->num_glyphs; i++) {
      int gap, j;
      gap = g->gd[i].gid - prev - 1;
      for (j = 1; j <= gap; j++) {
	if (prev + j == hhea->numOfLongHorMetrics - 1) {
	  p += sfnt_put_ushort(p, last_advw);
	} else if (prev + j < hhea->numOfLongHorMetrics) {
	  p += sfnt_put_ushort(p, 0);
	}
	p += sfnt_put_short (p, 0);
	if (head->indexToLocFormat == 0) {
	  q += sfnt_put_ushort(q, (USHORT) (offset/2));
	} else {
	  q += sfnt_put_ulong(q, offset);
	}
      }
      padlen = (g->gd[i].length % 4) ? (4 - (g->gd[i].length % 4)) : 0;
      if (g->gd[i].gid < hhea->numOfLongHorMetrics) {
	p += sfnt_put_ushort(p, g->gd[i].advw);
      }
      p += sfnt_put_short (p, g->gd[i].lsb);
      if (head->indexToLocFormat == 0) {
	q += sfnt_put_ushort(q, (USHORT) (offset/2));
      } else {
	q += sfnt_put_ulong(q, offset);
      }
      memset(glyf_table_data + offset, 0, g->gd[i].length + padlen);
      memcpy(glyf_table_data + offset, g->gd[i].data, g->gd[i].length);
      offset += g->gd[i].length + padlen;
      prev    = g->gd[i].gid;
      /* free data here since it consume much memory */
      RELEASE(g->gd[i].data);
      g->gd[i].length = 0; g->gd[i].data = NULL;
    }
    if (head->indexToLocFormat == 0) {
      q += sfnt_put_ushort(q, (USHORT) (offset/2));
    } else {
      q += sfnt_put_ulong(q, offset);
    }

    sfnt_set_table(sfont, "hmtx", (char *) hmtx_table_data, hmtx_table_size);
    sfnt_set_table(sfont, "loca", (char *) loca_table_data, loca_table_size);
    sfnt_set_table(sfont, "glyf", (char *) glyf_table_data, glyf_table_size);
  }

  head->checkSumAdjustment = 0;
  maxp->numGlyphs          = g->last_gid + 1;

  /* TODO */
  sfnt_set_table(sfont, "maxp", tt_pack_maxp_table(maxp), TT_MAXP_TABLE_SIZE);
  sfnt_set_table(sfont, "hhea", tt_pack_hhea_table(hhea), TT_HHEA_TABLE_SIZE);
  sfnt_set_table(sfont, "head", tt_pack_head_table(head), TT_HEAD_TABLE_SIZE);
  RELEASE(maxp);
  RELEASE(hhea);
  RELEASE(head);
  if (os2)
    RELEASE(os2);

  return 0;
}

int
tt_get_metrics (sfnt *sfont, struct tt_glyphs *g)
{
  struct tt_head_table *head = NULL;
  struct tt_hhea_table *hhea = NULL;
  struct tt_maxp_table *maxp = NULL;
  struct tt_longMetrics *hmtx, *vmtx = NULL;
  struct tt_os2__table  *os2;
  /* temp */
  ULONG  *location, offset;
  int     i;
  USHORT *w_stat;

  ASSERT(g);

  if (sfont == NULL || sfont->stream == NULL)
    ERROR("File not opened.");

  if (sfont->type != SFNT_TYPE_TRUETYPE &&
      sfont->type != SFNT_TYPE_TTC &&
      sfont->type != SFNT_TYPE_DFONT)
    ERROR("Invalid font type");

  /*
   * Read head, hhea, maxp, loca:
   *
   *   unitsPerEm       --> head
   *   numHMetrics      --> hhea
   *   indexToLocFormat --> head
   *   numGlyphs        --> maxp
   */
  head = tt_read_head_table(sfont);
  hhea = tt_read_hhea_table(sfont);
  maxp = tt_read_maxp_table(sfont);

  if (hhea->metricDataFormat != 0)
    ERROR("Unknown metricDataFormat.");

  g->emsize = head->unitsPerEm;

  sfnt_locate_table(sfont, "hmtx");
  hmtx = tt_read_longMetrics(sfont, maxp->numGlyphs, hhea->numOfLongHorMetrics, hhea->numOfExSideBearings);

  os2 = tt_read_os2__table(sfont);
  g->default_advh = os2->sTypoAscender - os2->sTypoDescender;
  g->default_tsb  = g->default_advh - os2->sTypoAscender;

  if (sfnt_find_table_pos(sfont, "vmtx") > 0) {
    struct tt_vhea_table *vhea;
    vhea = tt_read_vhea_table(sfont);
    sfnt_locate_table(sfont, "vmtx");
    vmtx = tt_read_longMetrics(sfont, maxp->numGlyphs, vhea->numOfLongVerMetrics, vhea->numOfExSideBearings);
    RELEASE(vhea);
  } else {
    vmtx = NULL;
  }

  sfnt_locate_table(sfont, "loca");
  location = NEW(maxp->numGlyphs + 1, ULONG);
  if (head->indexToLocFormat == 0) {
    for (i = 0; i <= maxp->numGlyphs; i++)
      location[i] = 2*((ULONG) sfnt_get_ushort(sfont));
  } else if (head->indexToLocFormat == 1) {
    for (i = 0; i <= maxp->numGlyphs; i++)
      location[i] = sfnt_get_ulong(sfont);
  } else {
    ERROR("Unknown IndexToLocFormat.");
  }

  w_stat = NEW(g->emsize+2, USHORT);
  memset(w_stat, 0, sizeof(USHORT)*(g->emsize+2));
  /*
   * Read glyf table.
   */
  offset = sfnt_locate_table(sfont, "glyf");
  for (i = 0; i < g->num_glyphs; i++) {
    USHORT gid;     /* old gid */
    ULONG  loc, len;

    gid = g->gd[i].ogid;
    if (gid >= maxp->numGlyphs)
      ERROR("Invalid glyph index (gid %u)", gid);

    loc = location[gid];
    len = location[gid+1] - loc;
    g->gd[i].advw = hmtx[gid].advance;
    g->gd[i].lsb  = hmtx[gid].sideBearing;
    if (vmtx) {
      g->gd[i].advh = vmtx[gid].advance;
      g->gd[i].tsb  = vmtx[gid].sideBearing;
    } else {
      g->gd[i].advh = g->default_advh;
      g->gd[i].tsb  = g->default_tsb;
    }
    g->gd[i].length = len;
    g->gd[i].data   = NULL;

    if (g->gd[i].advw <= g->emsize) {
      w_stat[g->gd[i].advw] += 1;
    } else {
      w_stat[g->emsize+1]   += 1; /* larger than em */
    }

    if (len == 0) { /* Does not contains any data. */
      continue;
    } else if (len < 10) {
      ERROR("Invalid TrueType glyph data (gid %u).", gid);
    }

    sfnt_seek_set(sfont, offset+loc);
    (void)               sfnt_get_short(sfont);

    /* BoundingBox: FWord x 4 */
    g->gd[i].llx = sfnt_get_short(sfont);
    g->gd[i].lly = sfnt_get_short(sfont);
    g->gd[i].urx = sfnt_get_short(sfont);
    g->gd[i].ury = sfnt_get_short(sfont);
    /* _FIXME_ */
#if  1
    if (!vmtx) /* vertOriginY == sTypeAscender */
      g->gd[i].tsb = g->default_advh - g->default_tsb - g->gd[i].ury;
#endif
  }
  RELEASE(location);
  RELEASE(hmtx);
  RELEASE(maxp);
  RELEASE(hhea);
  RELEASE(head);
  RELEASE(os2);

  if (vmtx)
    RELEASE(vmtx);

  {
    int max_count = -1;

    g->dw = g->gd[0].advw;
    for (i = 0; i < g->emsize + 1; i++) {
      if (w_stat[i] > max_count) {
	max_count = w_stat[i];
	g->dw = i;
      }
    }
  }
  RELEASE(w_stat);


  return 0;
}
