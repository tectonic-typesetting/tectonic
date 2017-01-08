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
 * TrueType GSUB support: (incomplete)
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "system.h"

#include "error.h"
#include "mem.h"
#include "mfileio.h"

#include "sfnt.h"

#include "otl_opt.h"
#include "tt_gsub.h"

#define VERBOSE_LEVEL_MIN 0
static int verbose = 0;
void
otl_gsub_set_verbose (void)
{
  verbose++;
}

typedef USHORT Offset;
typedef USHORT GlyphID;

/* OpenType Common Layout Table */
/* Records */
struct clt_record
{
  char   tag[5]; /* 4-byte identifier */
  Offset offset;
};

/* Ranges */
/* RangeRecord */
struct clt_range
{ 
  GlyphID Start; /* First GlyphID in the range */
  GlyphID End;   /* Last GlyphID in the range */
  USHORT  StartCoverageIndex; /* Converage Index of first GID */
};

static int
clt_read_record (struct clt_record *rec, sfnt *sfont)
{
  int i;

  ASSERT(rec && sfont);

  for (i = 0; i < 4; i++) {
    rec->tag[i] = sfnt_get_char(sfont);
  }
  rec->tag[4] = '\0';
  rec->offset = sfnt_get_ushort(sfont);

  return 6;
}

static int
clt_read_range (struct clt_range *rec, sfnt *sfont)
{
  ASSERT(rec && sfont);

  rec->Start = sfnt_get_ushort(sfont);
  rec->End   = sfnt_get_ushort(sfont);
  rec->StartCoverageIndex = sfnt_get_ushort(sfont);

  return 6;
}

/*
  List structure:
   ScriptRecord (records), FeatureRecord (records), Lookup (offsets)
*/

struct clt_record_list
{
  USHORT count;
  struct clt_record *record;
};

/* offset and index list, Offset is USHORT */
struct clt_number_list
{
  USHORT  count;
  USHORT *value;
};

static int
clt_read_record_list (struct clt_record_list *list, sfnt *sfont)
{
  int len, i;

  ASSERT(list && sfont);

  list->count = sfnt_get_ushort(sfont);
  len = 2;

  if (list->count == 0)
    list->record = NULL;
  else {
    list->record = NEW(list->count, struct clt_record);
    for (i = 0; i < list->count; i++) {
      len += clt_read_record(&(list->record[i]), sfont);
    }
  }

  return len;
}

static void
clt_release_record_list (struct clt_record_list *list)
{
  if (list) {
    if (list->record)
      RELEASE(list->record);
    list->record = NULL;
    list->count  = 0;
  }
}

static int
clt_read_number_list (struct clt_number_list *list, sfnt *sfont)
{
  int i;

  ASSERT(list && sfont);

  list->count = sfnt_get_ushort(sfont);

  if (list->count == 0)
    list->value = NULL;
  else {
    list->value = NEW(list->count, USHORT);
    for (i = 0; i < list->count; i++) {
      list->value[i] = sfnt_get_ushort(sfont);
    }
  }

  return (2 + 2 * list->count);
}

static void
clt_release_number_list (struct clt_number_list *list)
{
  if (list) {
    if (list->value)
      RELEASE(list->value);
    list->value = NULL;
    list->count = 0;
  }
}

/*
 * Tables
 */


/* Coverage Table: format 1 and format 2 */
struct clt_coverage
{
  USHORT format; /* Format identifier: 1 (list), 2 (range) */
  USHORT count;  /* Glyphs/Range Count */
  GlyphID *list; /* Array of GlyphIDs - in numerical order */
  struct clt_range *range; /* Array of glyph ranges
                            *  - ordered by Start GlyphID
                            */
};

/* GSUB - The Glyph Substitution Table */
struct otl_gsub_header
{
  Fixed  version;     /* 0x00010000 */
  Offset ScriptList;  /* offset */
  Offset FeatureList; /* offset */
  Offset LookupList;  /* offset */
};

/* Single Substitution Format 1 */
struct otl_gsub_single1
{
  SHORT DeltaGlyphID;            /* Add to original GlyphID to get
                                  * substitute GlyphID */
  struct clt_coverage coverage; /* Coverage table */
};

/* Single Substitution Format 2 */
struct otl_gsub_single2
{
  USHORT   GlyphCount; /* Number of GlyphIDs in the Substitute array */
  GlyphID *Substitute; /* Array of substitute GlyphIDs
                        * - ordered by Coverage Index */
  struct clt_coverage coverage; /* Coverage table */
};

/* Alternate Subsutitution Format 1 */
struct otl_gsub_altset
{
  USHORT   GlyphCount; /* Number of GlyphIDs in the Alternate array */
  GlyphID *Alternate;  /* Array of alternate GlyphIDs
                        * - in arbitrary order
                        */
};

struct otl_gsub_alternate1
{
  USHORT   AlternateSetCount;
  struct otl_gsub_altset *AlternateSet;

  struct clt_coverage coverage;
};

/* Faithfull */
struct otl_gsub_ligtab
{
  GlyphID  LigGlyph;  /* GlyphID of ligature glyph */
  USHORT   CompCount;
  GlyphID *Component; /* CompCount - 1 elements
                       * First component excluded.
                       * Ordered in writing direction...
                       */
};

struct otl_gsub_ligset
{
  USHORT LigatureCount;
  struct otl_gsub_ligtab *Ligature;
};

struct otl_gsub_ligature1
{
  USHORT LigSetCount;
  struct otl_gsub_ligset *LigatureSet;

  struct clt_coverage     coverage;
};

/* GSUB subtable (single) */
struct otl_gsub_subtab
{
  USHORT LookupType;  /* FIXME */

  USHORT SubstFormat;
  union {
    struct otl_gsub_single1    *single1;
    struct otl_gsub_single2    *single2;
    struct otl_gsub_alternate1 *alternate1;
    struct otl_gsub_ligature1  *ligature1;
  } table;
};

/* Script Table */
struct clt_script_table
{
  Offset DefaultLangSys;
  struct clt_record_list LangSysRecord;
};

static int
clt_read_script_table (struct clt_script_table *tab, sfnt *sfont)
{
  int len;

  ASSERT(tab && sfont);

  tab->DefaultLangSys = sfnt_get_ushort(sfont);
  len  = 2;
  len += clt_read_record_list(&tab->LangSysRecord, sfont);

  return len;
}

static void
clt_release_script_table (struct clt_script_table *tab)
{
  if (tab)
    clt_release_record_list(&tab->LangSysRecord);
}

/* LangSys Table */
struct clt_langsys_table
{
  Offset LookupOrder;     /* reserved */
  USHORT ReqFeatureIndex;
  struct clt_number_list FeatureIndex; /* Array of indices into the
                                        * FeatureList in arbitary order.
                                        */
};

static int
clt_read_langsys_table (struct clt_langsys_table *tab, sfnt *sfont)
{
  int len;

  ASSERT(tab && sfont);

  tab->LookupOrder     = sfnt_get_ushort(sfont);
  tab->ReqFeatureIndex = sfnt_get_ushort(sfont);
  len  = 4;
  len += clt_read_number_list(&tab->FeatureIndex, sfont);

  return len;
}

static void
clt_release_langsys_table (struct clt_langsys_table *tab)
{
  if (tab)
    clt_release_number_list(&tab->FeatureIndex);
}


/* Feature Table */
struct clt_feature_table
{
  Offset FeatureParams;
  struct clt_number_list LookupListIndex; /* LookupListIndex List */
};

static int
clt_read_feature_table (struct clt_feature_table *tab, sfnt *sfont)
{
  int len;

  ASSERT(tab && sfont);

  tab->FeatureParams = sfnt_get_ushort(sfont);
  len  = 2;
  len += clt_read_number_list(&tab->LookupListIndex, sfont);

  return len;
}

static void
clt_release_feature_table (struct clt_feature_table *tab)
{
  if (tab)
    clt_release_number_list(&tab->LookupListIndex);
}

/* Lookup Table:
 * Currently, only single substitution is supported.
 * LookupFlag is ignored.
 */
struct clt_lookup_table
{
  USHORT LookupType; /* Different enumerations for GSUB and GPOS */
  USHORT LookupFlag; /* Lookup qualifiers */
  struct clt_number_list SubTableList; /* offset */
  /* offset is from beginning of Lookup table */
};

static int
clt_read_lookup_table (struct clt_lookup_table *tab, sfnt *sfont)
{
  int len;

  ASSERT(tab && sfont);

  tab->LookupType = sfnt_get_ushort(sfont);
  tab->LookupFlag = sfnt_get_ushort(sfont);
  len  = 4;
  len += clt_read_number_list(&tab->SubTableList, sfont);

  return len;
}

static void
clt_release_lookup_table (struct clt_lookup_table *tab)
{
  if (tab)
    clt_release_number_list(&tab->SubTableList);
}

static int
clt_read_coverage (struct clt_coverage *cov, sfnt *sfont)
{
  int len, i;

  ASSERT(cov && sfont);

  cov->format = sfnt_get_ushort(sfont);
  cov->count  = sfnt_get_ushort(sfont);
  len = 4;

  switch (cov->format) {
  case 1: /* list */
    if (cov->count == 0)
      cov->list = NULL;
    else {
      cov->list = NEW(cov->count, USHORT);
      for (i = 0; i < cov->count; i++) {
        cov->list[i] = sfnt_get_ushort(sfont);
      }
    }
    cov->range = NULL;
    len += 2 * cov->count;
    break;
  case 2: /* range */
    if (cov->count == 0)
      cov->range = NULL;
    else {
      cov->range = NEW(cov->count, struct clt_range);
      for (i = 0; i < cov->count; i++) {
        len += clt_read_range(&(cov->range[i]), sfont);
      }
    }
    cov->list = NULL;
    break;
  default:
    ERROR("Unknown coverage format");
  }

  return len;
}

static void
clt_release_coverage (struct clt_coverage *cov)
{
  if (cov) {
    switch (cov->format) {
    case 1: /* list */
      if (cov->list)
        RELEASE(cov->list);
      cov->list = NULL;
      break;
    case 2: /* range */
      if (cov->range)
        RELEASE(cov->range);
      cov->range = NULL;
      break;
    default:
      ERROR("Unknown coverage format");
    }
  }
  cov->count = 0;
}

/* returns -1 if not found */
static int
clt_lookup_coverage (struct clt_coverage *cov, USHORT gid)
{
  int i;

  ASSERT(cov);

  switch (cov->format) {
  case 1: /* list */
    for (i = 0; i < cov->count; i++) {
      if (cov->list[i] > gid) {
        break;
      } else if (cov->list[i] == gid) {
        return i; /* found */
      }
    }
    break;
  case 2: /* range */
    for (i = 0; i < cov->count; i++) {
      if (gid < cov->range[i].Start) {
        break;
      } else if (gid <= cov->range[i].End) { /* found */
        return (cov->range[i].StartCoverageIndex +
                gid - cov->range[i].Start);
      }
    }
    break;
  default:
    ERROR("Unknown coverage format");
  }

  return -1; /* not found */
}

static int
otl_gsub_read_single (struct otl_gsub_subtab *subtab, sfnt *sfont)
{
  int    len;
  ULONG  offset;     /* not Offset which is USHORT */
  Offset cov_offset; /* subtable offset, offset to Coverage table */

  ASSERT(subtab && sfont);

  offset = tell_position(sfont->stream);

  subtab->LookupType  = OTL_GSUB_TYPE_SINGLE;
  subtab->SubstFormat = sfnt_get_ushort(sfont);
  len = 2;

  if (subtab->SubstFormat == 1) {
    struct otl_gsub_single1 *data;

    subtab->table.single1 = data = NEW(1, struct otl_gsub_single1);
    cov_offset         = sfnt_get_ushort(sfont);
    data->DeltaGlyphID = sfnt_get_short(sfont);
    len += 4;

    sfnt_seek_set(sfont, offset + cov_offset);
    len += clt_read_coverage(&data->coverage, sfont);

  } else if (subtab->SubstFormat == 2) {
    struct otl_gsub_single2 *data;
    USHORT count;

    subtab->table.single2 = data = NEW(1, struct otl_gsub_single2);
    cov_offset       = sfnt_get_ushort(sfont);
    data->GlyphCount = sfnt_get_ushort(sfont);
    len += 4;

    if (data->GlyphCount == 0)
      data->Substitute = NULL;
    else {
      data->Substitute = NEW(data->GlyphCount, GlyphID);
      for (count = 0; count < data->GlyphCount; count++) {
        data->Substitute[count] = sfnt_get_ushort(sfont);
      }
      len += 2 * data->GlyphCount;
    }

    sfnt_seek_set(sfont, offset + cov_offset);
    len += clt_read_coverage(&data->coverage, sfont);

  } else {
    ERROR("unexpected SubstFormat");
  }
  /* not implemented yet */

  return len;
}

static int
otl_gsub_read_alternate (struct otl_gsub_subtab *subtab, sfnt *sfont)
{
  int    len;
  USHORT i, j;
  ULONG  offset;     /* not Offset which is USHORT */
  Offset cov_offset; /* subtable offset, offset to Coverage table */
  struct clt_number_list      altset_offsets;
  struct otl_gsub_alternate1 *data;

  ASSERT(subtab && sfont);

  offset = tell_position(sfont->stream);

  subtab->LookupType  = OTL_GSUB_TYPE_ALTERNATE;
  subtab->SubstFormat = sfnt_get_ushort(sfont); /* Must be 1 */
  if (subtab->SubstFormat != 1) {
    WARN("Unknown GSUB SubstFormat for Alternate: %u",
         subtab->SubstFormat);
    return -1;
  }

  len  = 2;
  subtab->table.alternate1 =
    data = NEW(1, struct otl_gsub_alternate1);

  cov_offset        = sfnt_get_ushort(sfont);
  len += 2;
  len += clt_read_number_list(&altset_offsets, sfont);
  data->AlternateSetCount = altset_offsets.count;
  if (data->AlternateSetCount == 0) {
    data->AlternateSet    = NULL;
    data->coverage.count  = 0;
    data->coverage.format = 0;
    data->coverage.list   = NULL;
    return  len;
  }
  data->AlternateSet = NEW(data->AlternateSetCount,
                           struct otl_gsub_altset);
  for (i = 0; i < data->AlternateSetCount; i++) {
    struct otl_gsub_altset *altset;
    ULONG  altset_offset;

    altset = &(data->AlternateSet[i]);

    altset_offset = offset + altset_offsets.value[i];
    sfnt_seek_set(sfont, altset_offset);
    altset->GlyphCount = sfnt_get_ushort(sfont);
    len += 2;
    if (altset->GlyphCount == 0) {
      altset->Alternate = NULL;
      break;
    }
    altset->Alternate = NEW(altset->GlyphCount, GlyphID);
    for (j = 0; j < altset->GlyphCount; j++) {
      altset->Alternate[j] = sfnt_get_ushort(sfont);
      len += 2;
    }
  }
  clt_release_number_list(&altset_offsets);

  sfnt_seek_set(sfont, offset + cov_offset);
  len += clt_read_coverage(&data->coverage, sfont);

  return  len;
}

static int
otl_gsub_read_ligature (struct otl_gsub_subtab *subtab, sfnt *sfont)
{
  int    len;
  USHORT i, j;
  ULONG  offset;     /* not Offset which is USHORT */
  Offset cov_offset; /* subtable offset, offset to Coverage table */
  struct clt_number_list     ligset_offsets;
  struct otl_gsub_ligature1 *data;

  ASSERT(subtab && sfont);

  offset = tell_position(sfont->stream);

  subtab->LookupType  = OTL_GSUB_TYPE_LIGATURE;
  subtab->SubstFormat = sfnt_get_ushort(sfont); /* Must be 1 */
  if (subtab->SubstFormat != 1) {
    WARN("Unknown GSUB SubstFormat for Ligature: %u",
         subtab->SubstFormat);
    return -1;
  }

  len  = 2;
  subtab->table.ligature1 =
    data = NEW(1, struct otl_gsub_ligature1);

  cov_offset        = sfnt_get_ushort(sfont);
  len += 2;
  len += clt_read_number_list(&ligset_offsets, sfont);
  data->LigSetCount = ligset_offsets.count;
  if (data->LigSetCount == 0) {
    data->LigatureSet    = NULL;
    data->coverage.count  = 0;
    data->coverage.format = 0;
    data->coverage.list   = NULL;
    return len;
  }
  data->LigatureSet = NEW(data->LigSetCount,
                          struct otl_gsub_ligset);
  for (i = 0; i < data->LigSetCount; i++) {
    struct clt_number_list  ligset_tab;
    struct otl_gsub_ligset *ligset;
    ULONG  ligset_offset;
    USHORT count;

    ligset = &(data->LigatureSet[i]);

    ligset_offset = offset + ligset_offsets.value[i];
    sfnt_seek_set(sfont, ligset_offset);
    len += clt_read_number_list(&ligset_tab, sfont);

    ligset->LigatureCount = ligset_tab.count;
    if (ligset_tab.count == 0) {
      ligset->Ligature = NULL;
      break;
    }
    ligset->Ligature = NEW(ligset_tab.count,
                           struct otl_gsub_ligtab);
    for (j = 0; j < ligset_tab.count; j++) {
      sfnt_seek_set(sfont, ligset_offset + ligset_tab.value[j]);
      ligset->Ligature[j].LigGlyph = sfnt_get_ushort(sfont);
      ligset->Ligature[j].CompCount = sfnt_get_ushort(sfont);
      if (ligset->Ligature[j].CompCount == 0) {
        ligset->Ligature[j].Component = NULL;
        break;
      }
      ligset->Ligature[j].Component =
        NEW(ligset->Ligature[j].CompCount - 1, GlyphID);
      for (count = 0;
           count < ligset->Ligature[j].CompCount - 1; count++) {
        ligset->Ligature[j].Component[count] = sfnt_get_ushort(sfont);
      }
      len += 4 + count * 2;
    }
    clt_release_number_list(&ligset_tab);
  }
  clt_release_number_list(&ligset_offsets);

  sfnt_seek_set(sfont, offset + cov_offset);
  len += clt_read_coverage(&data->coverage, sfont);

  return len;
}

static void
otl_gsub_release_single (struct otl_gsub_subtab *subtab)
{
  if (subtab) {
    switch((int) subtab->SubstFormat) {
    case 1:
      {
        struct otl_gsub_single1 *data;

        data = subtab->table.single1;
        if (data) {
          clt_release_coverage(&data->coverage);
          RELEASE(data);
        }
        subtab->table.single1 = NULL;
      }
    break;
    case 2:
      {
        struct otl_gsub_single2 *data;

        data = subtab->table.single2;
        if (data) {
          if (data->Substitute)
            RELEASE(data->Substitute);
          clt_release_coverage(&data->coverage);
          RELEASE(data);
        }
        subtab->table.single2 = NULL;
      }
    break;
    default:
      ERROR("Unknown format for single substitution");
    }
  }
}

static void
otl_gsub_release_ligature (struct otl_gsub_subtab *subtab)
{
  if (subtab) {
    struct otl_gsub_ligature1 *data;
    USHORT i, j;

    data = subtab->table.ligature1;
    if (data && data->LigatureSet) {
      for (i = 0; i < data->LigSetCount; i++) {
        struct otl_gsub_ligset *ligset;

        ligset = &(data->LigatureSet[i]);
        for (j = 0;
             j < ligset->LigatureCount; j++) {
          if (ligset->Ligature[j].Component)
            RELEASE(ligset->Ligature[j].Component);
          ligset->Ligature[j].Component = NULL;
        }
        RELEASE(ligset->Ligature);
        ligset->Ligature = NULL;
      }
      RELEASE(data->LigatureSet);
    }
    clt_release_coverage(&data->coverage);
    data->LigatureSet = NULL;
    RELEASE(data);
    subtab->table.ligature1 = NULL;
  }
}

static void
otl_gsub_release_alternate (struct otl_gsub_subtab *subtab)
{
  if (subtab) {
    struct otl_gsub_alternate1 *data;
    USHORT i;

    data = subtab->table.alternate1;
    if (data && data->AlternateSet) {
      for (i = 0; i < data->AlternateSetCount; i++) {
        struct otl_gsub_altset *altset;

        altset = &(data->AlternateSet[i]);
        if (altset->Alternate)
          RELEASE(altset->Alternate);
        altset->Alternate = NULL;
      }
      RELEASE(data->AlternateSet);
    }
    clt_release_coverage(&data->coverage);
    data->AlternateSet = NULL;
    RELEASE(data);
    subtab->table.alternate1 = NULL;
  }
}

static int
otl_gsub_read_header (struct otl_gsub_header *head, sfnt *sfont)
{
  ASSERT(head && sfont);

  head->version     = sfnt_get_ulong (sfont);
  head->ScriptList  = sfnt_get_ushort(sfont);
  head->FeatureList = sfnt_get_ushort(sfont);
  head->LookupList  = sfnt_get_ushort(sfont);

  return 10;
}

/*
 * script -- langsys --> feature indices
 *        |
 *        +- langsys --> feature indices
 *
 * feature --> lookup indices
 */

struct otl_gsub_tab
{
  char *script;
  char *language;
  char *feature;

  int    num_subtables;
  struct otl_gsub_subtab *subtables;
};


static int
otl_gsub_read_feat (struct otl_gsub_tab *gsub, sfnt *sfont)
{
  int    feat_idx, script_idx;
  ULONG  gsub_offset, offset;
  struct otl_gsub_header  head;
  struct otl_gsub_subtab *subtab = NULL;
  USHORT num_subtabs = 0;
  unsigned char feat_bits[8192];
  struct clt_record_list feature_list;
  struct clt_record_list script_list;
  struct clt_number_list lookup_list ;
  otl_opt *script, *language, *feature;

  ASSERT(gsub && sfont);

  gsub_offset = sfnt_find_table_pos(sfont, "GSUB");
  if (gsub_offset == 0)
    return -1; /* not found */

  script   = otl_new_opt();
  otl_parse_optstring(script,   gsub->script);
  language = otl_new_opt();
  otl_parse_optstring(language, gsub->language);
  feature  = otl_new_opt();
  otl_parse_optstring(feature,  gsub->feature);

  memset(feat_bits, 0, 8192);

  /* GSUB header */
  sfnt_seek_set(sfont, gsub_offset);
  otl_gsub_read_header(&head, sfont);

  /* Script */
  offset = gsub_offset + head.ScriptList;
  sfnt_seek_set(sfont, offset);
  clt_read_record_list(&script_list, sfont);

#define SET_BIT(b,p) do {\
  (b)[(p)/8] |= (1<<(7-((p) % 8)));\
} while (0)
#define BIT_SET(b,p) (((b)[(p)/8]) & (1 << (7-((p)%8))))

  for (script_idx = 0;
       script_idx < script_list.count; script_idx++) {
    if (otl_match_optrule(script,
                          script_list.record[script_idx].tag)) {
      struct clt_script_table script_tab;
      int    langsys_idx;

      offset = gsub_offset +
        head.ScriptList + script_list.record[script_idx].offset;
      sfnt_seek_set(sfont, offset);
      clt_read_script_table(&script_tab, sfont);

      if (otl_match_optrule(language, "dflt") &&
          script_tab.DefaultLangSys != 0) { 
        struct clt_langsys_table langsys_tab;

        if(verbose > VERBOSE_LEVEL_MIN) {
          MESG("otl_gsub>> OTL script-language enabled: %c%c%c%c.dflt\n",
               script_list.record[script_idx].tag[0],
               script_list.record[script_idx].tag[1],
               script_list.record[script_idx].tag[2],
               script_list.record[script_idx].tag[3]);
        }

        sfnt_seek_set(sfont, offset + script_tab.DefaultLangSys);
        clt_read_langsys_table(&langsys_tab, sfont);
        if (otl_match_optrule(feature, "____") && /* _FIXME_ */
            langsys_tab.ReqFeatureIndex != 0xFFFF)
          SET_BIT(feat_bits, langsys_tab.ReqFeatureIndex);
        for (feat_idx = 0;
             feat_idx < langsys_tab.FeatureIndex.count;
             feat_idx++) {
          SET_BIT(feat_bits,
                  langsys_tab.FeatureIndex.value[feat_idx]);
        }
        clt_release_langsys_table(&langsys_tab);
      }
      for (langsys_idx = 0;
           langsys_idx < script_tab.LangSysRecord.count;
           langsys_idx++) {
        struct clt_record  *langsys_rec;

        langsys_rec = &(script_tab.LangSysRecord.record[langsys_idx]);
        if (otl_match_optrule(language, langsys_rec->tag)) {
          struct clt_langsys_table langsys_tab;

          if(verbose > VERBOSE_LEVEL_MIN) {
            MESG("otl_gsub>> OTL script-language enabled: %c%c%c%c.%c%c%c%c\n",
                 script_list.record[script_idx].tag[0],
                 script_list.record[script_idx].tag[1],
                 script_list.record[script_idx].tag[2],
                 script_list.record[script_idx].tag[3],
                 langsys_rec->tag[0], langsys_rec->tag[1],
                 langsys_rec->tag[2], langsys_rec->tag[3]);
          }

          sfnt_seek_set(sfont, offset + langsys_rec->offset);
          clt_read_langsys_table(&langsys_tab, sfont);
          if (otl_match_optrule(feature, "____") || /* _FIXME_ */
              langsys_tab.ReqFeatureIndex != 0xFFFF)
            SET_BIT(feat_bits, langsys_tab.ReqFeatureIndex);
          for (feat_idx = 0;
               feat_idx < langsys_tab.FeatureIndex.count;
               feat_idx++) {
            SET_BIT(feat_bits,
                    langsys_tab.FeatureIndex.value[feat_idx]);
          }
          clt_release_langsys_table(&langsys_tab);
        }
      }
      clt_release_script_table(&script_tab);
    }
  }

  /* Feature List */
  offset = gsub_offset + head.FeatureList;
  sfnt_seek_set(sfont, offset);
  clt_read_record_list(&feature_list, sfont);

  /* Lookup List */
  offset = gsub_offset + head.LookupList;
  sfnt_seek_set(sfont, offset);
  clt_read_number_list(&lookup_list, sfont);

  if(verbose > VERBOSE_LEVEL_MIN) {
    MESG("otl_gsub>> Reading OTL feature(s):");
  }

  for (feat_idx = 0;
       feat_idx < feature_list.count; feat_idx++) {
    if (BIT_SET(feat_bits, feat_idx)  &&
        (otl_match_optrule(feature,
                           feature_list.record[feat_idx].tag))) {
      struct clt_feature_table feature_table;
      int    i;

      if(verbose > VERBOSE_LEVEL_MIN) {
        MESG(" %c%c%c%c",
             feature_list.record[feat_idx].tag[0],
             feature_list.record[feat_idx].tag[1],
             feature_list.record[feat_idx].tag[2],
             feature_list.record[feat_idx].tag[3]);
      }

      /* Feature Table */
      offset = gsub_offset +
        head.FeatureList + feature_list.record[feat_idx].offset;

      sfnt_seek_set(sfont, offset);
      clt_read_feature_table(&feature_table, sfont);
      if (feature_table.FeatureParams != 0) {
        ERROR("unrecognized FeatureParams");
      }

      /* Lookup table */
      for (i = 0; i < feature_table.LookupListIndex.count; i++) {
        struct clt_lookup_table lookup_table;
        int ll_idx, st_idx, r, n_st;

        ll_idx = feature_table.LookupListIndex.value[i];
        if (ll_idx >= lookup_list.count)
          ERROR("invalid Lookup index.");

        offset = gsub_offset +
          head.LookupList + (lookup_list.value)[ll_idx];
        sfnt_seek_set(sfont, offset);
        clt_read_lookup_table(&lookup_table, sfont);

        if (lookup_table.LookupType != OTL_GSUB_TYPE_SINGLE    &&
            lookup_table.LookupType != OTL_GSUB_TYPE_ALTERNATE &&
            lookup_table.LookupType != OTL_GSUB_TYPE_LIGATURE  &&
            lookup_table.LookupType != OTL_GSUB_TYPE_ESUBST) {
          if (verbose > VERBOSE_LEVEL_MIN)
            WARN("Skipping unsupported GSUB subtable: LookupType=%d", lookup_table.LookupType);
          continue;
        }

        subtab = RENEW(subtab,
                       num_subtabs + lookup_table.SubTableList.count,
                       struct otl_gsub_subtab);
        for (n_st = 0, st_idx = 0;
             st_idx < lookup_table.SubTableList.count; st_idx++) {

          offset = gsub_offset + head.LookupList +
            lookup_list.value[ll_idx] +
            (lookup_table.SubTableList.value)[st_idx];

          sfnt_seek_set(sfont, offset);

          switch ((int) lookup_table.LookupType) {
          case OTL_GSUB_TYPE_SINGLE:
            r = otl_gsub_read_single(&subtab[num_subtabs + n_st],
                                     sfont);
            if (r <= 0)
              WARN("Reading GSUB subtable (single) failed...");
            else {
              if(verbose > VERBOSE_LEVEL_MIN) {
                MESG("(single)");
              }
              n_st++;
            }
            break;

          case OTL_GSUB_TYPE_ALTERNATE:
            r = otl_gsub_read_alternate(&subtab[num_subtabs + n_st],
                                        sfont);
            if (r <= 0)
              WARN("Reading GSUB subtable (alternate) failed...");
            else {
              if(verbose > VERBOSE_LEVEL_MIN) {
                MESG("(alternate)");
              }
              n_st++;
            }
            break;

          case OTL_GSUB_TYPE_LIGATURE:
            r = otl_gsub_read_ligature(&subtab[num_subtabs + n_st],
                                       sfont);
            if (r <= 0)
              WARN("Reading GSUB subtable (ligature) failed...");
            else {
              if(verbose > VERBOSE_LEVEL_MIN) {
                MESG("(ligature)");
              }
              n_st++;
            }
            break;

          case OTL_GSUB_TYPE_ESUBST:
            {
              USHORT  SubstFormat;
              USHORT  ExtensionLookupType;
              ULONG   ExtensionOffset;

              SubstFormat = sfnt_get_ushort(sfont);
              if (SubstFormat != 1)
                break;
              ExtensionLookupType = sfnt_get_ushort(sfont);
              ExtensionOffset     = sfnt_get_ulong (sfont);

              sfnt_seek_set(sfont, offset + ExtensionOffset);
              switch (ExtensionLookupType) {
              case OTL_GSUB_TYPE_SINGLE:
                r = otl_gsub_read_single(&subtab[num_subtabs + n_st],
                                         sfont);
                if (r <= 0)
                  WARN("Reading GSUB subtable (ext:single) failed...");
                else {
                  if(verbose > VERBOSE_LEVEL_MIN) {
                    MESG("(ext:single)");
                  }
                  n_st++;
                }
                break;

              case OTL_GSUB_TYPE_ALTERNATE:
                r = otl_gsub_read_alternate(&subtab[num_subtabs + n_st],
                                            sfont);
                if (r <= 0)
                  WARN("Reading GSUB subtable (alternate) failed...");
                else {
                  if(verbose > VERBOSE_LEVEL_MIN) {
                    MESG("(alternate)");
                  }
                  n_st++;
                }
              break;

              case OTL_GSUB_TYPE_LIGATURE:
                r = otl_gsub_read_ligature(&subtab[num_subtabs + n_st],
                                           sfont);
                if (r <= 0)
                  WARN("Reading GSUB subtable (ext:ligature) failed...");
                else {
                  if(verbose > VERBOSE_LEVEL_MIN) {
                    MESG("(ext:ligature)");
                  }
                  n_st++;
                }
                break;

              }
            }
            break;

          default:
            break;
          }
        }
        num_subtabs += n_st; /* lookup_table.SubTableList.count; */
        clt_release_lookup_table(&lookup_table);
      }
      clt_release_feature_table(&feature_table);
    }
  }

  if(verbose > VERBOSE_LEVEL_MIN) {
    MESG("\n");
    MESG("otl_gsub>> %ld subtable(s) read.\n", num_subtabs);
  }

  clt_release_number_list(&lookup_list);
  clt_release_record_list(&feature_list);
  clt_release_record_list(&script_list);

  otl_release_opt(script);
  otl_release_opt(language);
  otl_release_opt(feature);

  if (subtab != NULL) {
    gsub->num_subtables = num_subtabs;
    gsub->subtables     = subtab;
  } else {
    return -1;
  }

  return 0;
}


static int
otl_gsub_apply_single (struct otl_gsub_subtab *subtab, USHORT *gid)
{
  int idx;

  ASSERT(subtab && gid);

  if (subtab->SubstFormat == 1) {
    struct otl_gsub_single1 *data;

    data = (subtab->table).single1;
    idx  = clt_lookup_coverage(&data->coverage, *gid);
    if (idx >= 0) {
      *gid += data->DeltaGlyphID;
      return 0; /* found */
    }
  } else if (subtab->SubstFormat == 2) {
    struct otl_gsub_single2 *data;

    data = (subtab->table).single2;
    idx  = clt_lookup_coverage(&data->coverage, *gid);
    if (idx >= 0 &&
        idx < data->GlyphCount) {
      *gid = (data->Substitute)[idx];
      return 0; /* found */
    }
  }

  return -1;
}

static int
otl_gsub_apply_alternate (struct otl_gsub_subtab *subtab,
                          USHORT alt_idx, USHORT *gid)
{
  int  idx;

  ASSERT(subtab && gid);

  if (subtab->SubstFormat == 1) {
    struct otl_gsub_alternate1 *data;

    data = subtab->table.alternate1;
    idx  = clt_lookup_coverage(&data->coverage, *gid);
    if (idx < 0 || idx >= data->AlternateSetCount)
      return  -1;
    else {
      struct otl_gsub_altset *altset;
      altset = &(data->AlternateSet[idx]);
      if (alt_idx >= altset->GlyphCount)
        return  -1;
      else {
        *gid = altset->Alternate[alt_idx];
        return  0;
      }
    }
  }

  return -1;
}

/* NOTE: Ligature table is in preference order */
static int
glyph_seq_cmp (GlyphID *glyph_seq0, USHORT n_glyphs0,
               GlyphID *glyph_seq1, USHORT n_glyphs1)
{
  USHORT i;

  if (n_glyphs0 != n_glyphs1)
    return n_glyphs0 - n_glyphs1;

  for (i = 0; i < n_glyphs0; i++) {
    if (glyph_seq0[i] != glyph_seq1[i])
      return glyph_seq0[i] - glyph_seq1[i];
  }

  return 0;
}

static int
otl_gsub_apply_ligature (struct otl_gsub_subtab *subtab,
                         USHORT *gid_in,  USHORT num_gids,
                         USHORT *gid_out)
{
  int idx;

  ASSERT(subtab && gid_out);

  if (!gid_in || num_gids < 1)
    return -1;

  if (subtab->SubstFormat == 1) {
    struct otl_gsub_ligature1 *data;

    data = subtab->table.ligature1;
    idx  = clt_lookup_coverage(&data->coverage, gid_in[0]);
    if (idx >= 0 && idx < data->LigSetCount) {
      struct otl_gsub_ligset *ligset;
      USHORT j;

      ligset = &(data->LigatureSet[idx]);
      for (j = 0; j < ligset->LigatureCount; j++) {
        if (!glyph_seq_cmp(&gid_in[1], (USHORT)(num_gids - 1),
                           ligset->Ligature[j].Component,
                           (USHORT)(ligset->Ligature[j].CompCount - 1))) {
          *gid_out = ligset->Ligature[j].LigGlyph;
          return 0; /* found */
        }
      }
    }
  }

  return -1;
}

#define GSUB_LIST_MAX 32
struct otl_gsub
{
  int num_gsubs;
  int select;
  struct otl_gsub_tab gsubs[GSUB_LIST_MAX];
};

otl_gsub *
otl_gsub_new (void)
{
  struct otl_gsub *gsub_list;

  gsub_list = NEW(1, struct otl_gsub);
  gsub_list->num_gsubs = 0;
  gsub_list->select    = -1;

  return (otl_gsub *) gsub_list;
}

int
otl_gsub_add_feat (otl_gsub *gsub_list,
                   const char *script,
                   const char *language,
                   const char *feature,
                   sfnt *sfont)
{
  int    retval = -1;
  int    i;
  struct otl_gsub_tab *gsub;

  if (gsub_list->num_gsubs > GSUB_LIST_MAX) {
    ERROR("Too many GSUB features...");
    return -1;
  }
  for (i = 0; i < gsub_list->num_gsubs; i++) {
    gsub = &(gsub_list->gsubs[i]);
    if (!strcmp(script,   gsub->script)   &&
        !strcmp(language, gsub->language) &&
        !strcmp(feature,  gsub->feature)) {
      gsub_list->select = i;
      return 0;
    }
  }
  gsub = &gsub_list->gsubs[i];

  gsub->script   = NEW(strlen(script)  +1, char);
  strcpy(gsub->script,   script);
  gsub->language = NEW(strlen(language)+1, char);
  strcpy(gsub->language, language);
  gsub->feature  = NEW(strlen(feature) +1, char);
  strcpy(gsub->feature,  feature);

  if(verbose > VERBOSE_LEVEL_MIN) {
    MESG("\n");
    MESG("otl_gsub>> Reading \"%s.%s.%s\"...\n", script, language, feature);
  }

  retval = otl_gsub_read_feat(gsub, sfont);
  if (retval >= 0) {
    gsub_list->select = i;
    gsub_list->num_gsubs++;
  } else {
    if(verbose > VERBOSE_LEVEL_MIN) {
      MESG("otl_gsub>> Failed\n");
    }
    RELEASE(gsub->script);
    RELEASE(gsub->language);
    RELEASE(gsub->feature);
  }
  
  return retval;
}

void
otl_gsub_release (otl_gsub *gsub_list)
{
  struct otl_gsub_tab    *gsub;
  struct otl_gsub_subtab *subtab;
  int    i, j;

  if (!gsub_list)
    return;

  for (i = 0; i < gsub_list->num_gsubs; i++) {
    gsub = &(gsub_list->gsubs[i]);

    if (gsub->script)
      RELEASE(gsub->script);
    if (gsub->language)
      RELEASE(gsub->language);
    if (gsub->feature)
      RELEASE(gsub->feature);

    for (j = 0; j < gsub->num_subtables; j++) {
      subtab = &(gsub->subtables[j]);
      switch ((int) subtab->LookupType) {
      case OTL_GSUB_TYPE_SINGLE:
        otl_gsub_release_single(subtab);
        break;
      case OTL_GSUB_TYPE_ALTERNATE:
        otl_gsub_release_alternate(subtab);
        break;
      case OTL_GSUB_TYPE_LIGATURE:
        otl_gsub_release_ligature(subtab);
        break;
      default:
        ERROR("???");
        break;
      }
    }
    RELEASE(gsub->subtables);
  }

  RELEASE(gsub_list);
}

int
otl_gsub_apply (otl_gsub *gsub_list, USHORT *gid)
{
  int    retval = -1;
  struct otl_gsub_tab    *gsub;
  struct otl_gsub_subtab *subtab;
  int    i, j;

  if (!gsub_list || !gid)
    return retval;

  i = gsub_list->select;
  if (i < 0 || i >= gsub_list->num_gsubs) {
    ERROR("GSUB not selected...");
    return -1;
  }
  gsub = &(gsub_list->gsubs[i]);

  for (j = 0;
       retval < 0 && j < gsub->num_subtables; j++) {
    subtab = &(gsub->subtables[j]);
    switch ((int) subtab->LookupType){
    case OTL_GSUB_TYPE_SINGLE:
      retval = otl_gsub_apply_single(subtab, gid);
      break;
    default:
      break;
    }
  }

  return retval;
}

int
otl_gsub_apply_alt (otl_gsub *gsub_list, USHORT alt_idx, USHORT *gid)
{
  int    retval = -1;
  struct otl_gsub_tab    *gsub;
  struct otl_gsub_subtab *subtab;
  int    i, j;

  if (!gsub_list || !gid)
    return retval;

  i = gsub_list->select;
  if (i < 0 || i >= gsub_list->num_gsubs) {
    ERROR("GSUB not selected...");
    return -1;
  }
  gsub = &(gsub_list->gsubs[i]);

  for (j = 0;
       retval < 0 && j < gsub->num_subtables; j++) {
    subtab = &(gsub->subtables[j]);
    switch ((int) subtab->LookupType){
    case OTL_GSUB_TYPE_ALTERNATE:
      retval = otl_gsub_apply_alternate(subtab, alt_idx, gid);
      break;
    default:
      break;
    }
  }

  return retval;
}

int
otl_gsub_apply_lig (otl_gsub *gsub_list,
                    USHORT *gid_in, USHORT num_gids, USHORT *gid_out)
{
  int    retval = -1;
  struct otl_gsub_tab    *gsub;
  struct otl_gsub_subtab *subtab;
  int    i, j;

  if (!gsub_list || !gid_out)
    return retval;

  i = gsub_list->select;
  if (i < 0 || i >= gsub_list->num_gsubs) {
    ERROR("GSUB not selected...");
    return -1;
  }
  gsub = &(gsub_list->gsubs[i]);

  for (j = 0;
       retval < 0 && j < gsub->num_subtables; j++) {
    subtab = &(gsub->subtables[j]);
    switch ((int) subtab->LookupType){
    case OTL_GSUB_TYPE_LIGATURE:
      retval = otl_gsub_apply_ligature(subtab,
                                       gid_in, num_gids, gid_out);
      break;
    default:
      break;
    }
  }

  return retval;
}

int
otl_gsub_select (otl_gsub *gsub_list,
                 const char *script,
                 const char *language,
                 const char *feature)
{
  int    i;
  struct otl_gsub_tab *gsub;

  for (i = 0; i < gsub_list->num_gsubs; i++) {
    gsub = &(gsub_list->gsubs[i]);
    if (!strcmp(gsub->script,   script)   &&
        !strcmp(gsub->language, language) &&
        !strcmp(gsub->feature,  feature)) {
      gsub_list->select = i;
      return i;
    }
  }

  gsub_list->select = -1;

  return -1;
}

#if  0
static int
otl_gsub_dump_single (struct otl_gsub_subtab *subtab)
{
  int  gid, idx;

  ASSERT(subtab);

  if (subtab->SubstFormat == 1) {
    struct otl_gsub_single1 *data;

    data = (subtab->table).single1;
    for (gid = 0; gid < 0x10000; gid++) {
      idx  = clt_lookup_coverage(&data->coverage, gid);
      if (idx >= 0) {
        fprintf(stdout, "substitute \\%u by \\%u;\n",
                (USHORT) gid, (USHORT) (gid + data->DeltaGlyphID));
      }
    }
  } else if (subtab->SubstFormat == 2) {
    struct otl_gsub_single2 *data;

    data = (subtab->table).single2;
    for (gid = 0; gid < 0x10000; gid++) {
      idx  = clt_lookup_coverage(&data->coverage, gid);
      if (idx >= 0 &&
          idx < data->GlyphCount) {
        fprintf(stdout, "substitute \\%u by \\%u;\n",
                (USHORT) gid, (data->Substitute)[idx]);
      }
    }
  }

  return  0;
}

static int
otl_gsub_dump_alternate (struct otl_gsub_subtab *subtab)
{
  int  gid, idx;

  ASSERT(subtab);

  if (subtab->SubstFormat == 1) {
    struct otl_gsub_alternate1 *data;

    data = subtab->table.alternate1;
    for (gid = 0; gid < 0x10000; gid++) {
      idx  = clt_lookup_coverage(&data->coverage, gid);
      if (idx >= 0 && idx < data->AlternateSetCount) {
        struct otl_gsub_altset *altset;
        USHORT i;
        altset = &(data->AlternateSet[idx]);
        if (altset->GlyphCount == 0)
          continue;
        fprintf(stdout, "substitute \\%u from [", (USHORT) gid);
        for (i = 0; i < altset->GlyphCount; i++) {
          fprintf(stdout, " \\%u", altset->Alternate[i]);
        }
        fprintf(stdout, " ];\n");
      }
    }
  }

  return  0;
}

static int
otl_gsub_dump_ligature (struct otl_gsub_subtab *subtab)
{
  int  gid, idx;

  ASSERT(subtab);

  if (subtab->SubstFormat == 1) {
    struct otl_gsub_ligature1 *data;

    data = subtab->table.ligature1;
    for (gid = 0; gid < 0x10000; gid++) {
      idx  = clt_lookup_coverage(&data->coverage, gid);
      if (idx >= 0 && idx < data->LigSetCount) {
        struct otl_gsub_ligset *ligset;
        USHORT  i, j;
        ligset = &(data->LigatureSet[idx]); 
        for (j = 0; j < ligset->LigatureCount; j++) {
          fprintf(stdout, "substitute \\%u", (USHORT) gid);
          for (i = 0; i < ligset->Ligature[j].CompCount - 1; i++) {
            fprintf(stdout, " \\%u", ligset->Ligature[j].Component[i]);
          }
          fprintf(stdout, " by \\%u;\n", ligset->Ligature[j].LigGlyph);
        }
      }
    }
  }

  return  0;
}

int
otl_gsub_dump (otl_gsub *gsub_list,
               const char *script, const char *language, const char *feature)
{
  int    error = -1;
  struct otl_gsub_tab    *gsub;
  struct otl_gsub_subtab *subtab;
  int    sel, i, j;

  if (!gsub_list)
    return  -1;

  sel   = gsub_list->select;
  error = otl_gsub_select(gsub_list, script, language, feature);
  if (error < 0) {
    ERROR("GSUB feature %s.%s.%s not found.", script, language, feature);
  }

  i = gsub_list->select;
  if (i < 0 || i >= gsub_list->num_gsubs) {
    ERROR("GSUB not selected...");
    return -1;
  }
  gsub = &(gsub_list->gsubs[i]);

  for (j = 0;
       !error &&
       j < gsub->num_subtables; j++) {
    subtab = &(gsub->subtables[j]);
    switch ((int) subtab->LookupType){
    case OTL_GSUB_TYPE_SINGLE:
      error = otl_gsub_dump_single(subtab);
      break;
    case OTL_GSUB_TYPE_ALTERNATE:
      error = otl_gsub_dump_alternate(subtab);
      break;
    case OTL_GSUB_TYPE_LIGATURE:
      error = otl_gsub_dump_ligature(subtab);
      break;
    }
  }
  gsub_list->select = sel;

  return  error;
}
#endif
