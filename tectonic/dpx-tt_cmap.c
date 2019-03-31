/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * A large part of codes are brought from ttfdump-0.5.5.
 */

#include "dpx-tt_cmap.h"

#include "core-bridge.h"
#include "dpx-agl.h"
/* Sorry for placing this here.
 * We need to rewrite TrueType font support code...
 */
#include "dpx-cmap.h"
#include "dpx-cmap_write.h"
#include "dpx-dpxfile.h"
/* Hash */
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfresource.h"
#include "dpx-sfnt.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_gsub.h"
#include "dpx-tt_post.h"
#include "dpx-type0.h"
#include "dpx-unicode.h"

#define VERBOSE_LEVEL_MIN 0
static int verbose = 0;
void
otf_cmap_set_verbose (int level)
{
    otl_gsub_set_verbose(level);
    verbose = level;
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
    unsigned int i;

    if (len < 256)
        _tt_abort("invalid cmap subtable");

    map = NEW(1, struct cmap0);

    for (i = 0; i < 256; i++)
        map->glyphIndexArray[i] = sfnt_get_byte(sfont);

    return map;
}

static void
release_cmap0(struct cmap0 *map)
{
    free(map);
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
        _tt_abort("invalid cmap subtable");

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
        free(map->subHeaders);
        free(map->glyphIndexArray);
        free(map);
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
        _tt_abort("invalid cmap subtable");

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
        free(map->endCount);
        free(map->startCount);
        free(map->idDelta);
        free(map->idRangeOffset);
        free(map->glyphIndexArray);
        free(map);
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
        _tt_abort("invalid cmap subtable");

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
        free(map->glyphIndexArray);
        free(map);
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
        _tt_abort("invalid cmap subtable");

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
        free(map->groups);
        free(map);
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

    assert(sfont);

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
            dpx_warning("Unrecognized cmap subtable format.");
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
        /* dpx_warning("UCS-4 TrueType cmap table..."); */
        cmap->map = read_cmap12(sfont, length);
        break;
    default:
        dpx_warning("Unrecognized OpenType/TrueType cmap format.");
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
                _tt_abort("Unrecognized OpenType/TrueType cmap format.");
            }
        }
        free(cmap);
    }

    return;
}


USHORT
tt_cmap_lookup (tt_cmap *cmap, ULONG cc)
{
    USHORT gid = 0;

    assert(cmap);

    if (cc > 0xffffL && cmap->format < 12) {
        dpx_warning("Four bytes charcode not supported in OpenType/TrueType cmap format 0...6.");
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
        _tt_abort("Unrecognized OpenType/TrueType cmap subtable format");
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
            unsigned char *GIDToCIDMap,
            otl_gsub *gsub_vert, otl_gsub *gsub_list,
            CMap *cmap, CMap *tounicode_add)
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
                gid = (map->glyphIndexArray[j+d] + map->idDelta[i]) & 0xffff;
            }
            if (gid != 0 && gid != 0xffff) {
                if (gsub_list)
                    otl_gsub_apply_chain(gsub_list, &gid);
                if (gsub_vert)
                    otl_gsub_apply(gsub_vert, &gid);
                if (GIDToCIDMap) {
                    cid = ((GIDToCIDMap[2*gid] << 8)|GIDToCIDMap[2*gid+1]);
                    if (cid == 0)
                        dpx_warning("GID %u does not have corresponding CID %u.", gid, cid);
                } else {
                    cid = gid;
                }
                wbuf[0] = 0;
                wbuf[1] = 0;
                wbuf[2] = (ch >> 8) & 0xff;
                wbuf[3] =  ch & 0xff;
                wbuf[4] = (cid >> 8) & 0xff;
                wbuf[5] = cid & 0xff;
                CMap_add_cidchar(cmap, wbuf, 4, cid);
                if (tounicode_add) {
                    unsigned char *p = wbuf + 6;
                    size_t uc_len;
                    uc_len = UC_UTF16BE_encode_char(ch, &p, wbuf + WBUF_SIZE - 1);
                    CMap_add_bfchar(tounicode_add, wbuf + 4, 2, wbuf + 6, uc_len);
                }
            }
        }
    }

    return;
}

static void
load_cmap12 (struct cmap12 *map,
             unsigned char *GIDToCIDMap,
             otl_gsub *gsub_vert, otl_gsub *gsub_list,
             CMap *cmap, CMap *tounicode_add)
{
    ULONG   i, ch;  /* LONG ? */
    USHORT  gid, cid;

    for (i = 0; i < map->nGroups; i++) {
        for (ch  = map->groups[i].startCharCode;
             ch <= map->groups[i].endCharCode;
             ch++) {
            int  d = ch - map->groups[i].startCharCode;
            gid = (USHORT) ((map->groups[i].startGlyphID + d) & 0xffff);
            if (gsub_list)
                otl_gsub_apply_chain(gsub_list, &gid);
            if (gsub_vert)
                otl_gsub_apply(gsub_vert, &gid);
            if (GIDToCIDMap) {
                cid = ((GIDToCIDMap[2*gid] << 8)|GIDToCIDMap[2*gid+1]);
                if (cid == 0)
                    dpx_warning("GID %u does not have corresponding CID %u.", gid, cid);
            } else {
                cid = gid;
            }
            wbuf[0] = (ch >> 24) & 0xff;
            wbuf[1] = (ch >> 16) & 0xff;
            wbuf[2] = (ch >>  8) & 0xff;
            wbuf[3] = ch & 0xff;
            wbuf[4] = (cid >> 8) & 0xff;
            wbuf[5] = cid & 0xff;
            CMap_add_cidchar(cmap, wbuf, 4, cid);
            if (tounicode_add) {
                unsigned char *p = wbuf + 6;
                size_t uc_len;
                uc_len = UC_UTF16BE_encode_char(ch, &p, wbuf + WBUF_SIZE - 1);
                CMap_add_bfchar(tounicode_add, wbuf + 4, 2, wbuf + 6, uc_len);
            }
        }
    }

    return;
}

#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_types.h"
/* OpenType CIDFont:
 *
 *  We don't use GID for them. OpenType cmap table is for
 *  charcode to GID mapping rather than to-CID mapping.
 */
#include "dpx-cid.h"
#include "dpx-tt_table.h"

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

    assert(csi);

    offset = sfnt_find_table_pos(sfont, "CFF ");
    if (offset == 0) {
        csi->registry = NULL;
        csi->ordering = NULL;
        *GIDToCIDMap  = NULL;
        return 0;
    }

    maxp       = tt_read_maxp_table(sfont);
    num_glyphs = (card16) maxp->numGlyphs;
    free(maxp);
    if (num_glyphs < 1)
        _tt_abort("No glyph contained in this font...");

    cffont = cff_open(sfont->handle, offset, 0);
    if (!cffont)
        _tt_abort("Could not open CFF font...");


    if (!(cffont->flag & FONTTYPE_CIDFONT)) {
        cff_close(cffont);
        csi->registry = NULL;
        csi->ordering = NULL;
        *GIDToCIDMap  = NULL;
        return 0;
    }

    if (!cff_dict_known(cffont->topdict, "ROS")) {
        _tt_abort("No CIDSystemInfo???");
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
        _tt_abort("No CFF charset data???");
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
            map = mfree(map);
        } else {
            /* Not trivial mapping */
            for (gid = 1, i = 0;
                 i < charset->num_entries; i++) {
                cid   = ranges[i].first;
                count = ranges[i].n_left + 1;
                while (count-- > 0 && gid <= num_glyphs) {
                    map[2*gid] = (cid >> 8) & 0xff;
                    map[2*gid+1] = cid & 0xff;
                    gid++; cid++;
                }
            }

        }
    }
    break;
    default:
        map = mfree(map);
        _tt_abort("Unknown CFF charset format...: %d", charset->format);
        break;
    }
    cff_close(cffont);

    *GIDToCIDMap = map;
    return 1;
}

static bool is_PUA_or_presentation (unsigned int uni)
{
    /* KANGXI RADICALs are commonly double encoded. */
    return  ((uni >= 0x2F00 && uni <= 0x2FD5) ||
             (uni >= 0xE000 && uni <= 0xF8FF) || (uni >= 0xFB00 && uni <= 0xFB4F) ||
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
        unsigned int j;
        size_t len;
        size_t inbytesleft, outbytesleft;
        const unsigned char *inbuf;
        unsigned char *outbuf;

        if (used_glyphs[i] == 0)
            continue;

        for (j = 0; j < 8; j++) {
            USHORT gid = 8 * i + j;

            if (!is_used_char2(used_glyphs, gid))
                continue;

            if (!cmap_add) {
#define MAX_UNICODES    16
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
                    if (name)
                        dpx_message("No Unicode mapping available: GID=%u, name=%s\n", gid, name);
                    else
                        dpx_message("No Unicode mapping available: GID=%u\n", gid);
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
                free(name);
            } else {
                wbuf[0] = (gid >> 8) & 0xff;
                wbuf[1] =  gid & 0xff;

                inbuf        = wbuf;
                inbytesleft  = 2;
                outbuf       = wbuf + 2;
                outbytesleft = WBUF_SIZE - 2;
                CMap_decode(cmap_add, &inbuf, &inbytesleft, &outbuf, &outbytesleft);

                if (inbytesleft != 0) {
                    dpx_warning("CMap conversion failed...");
                } else {
                    len = WBUF_SIZE - 2 - outbytesleft;
                    CMap_add_bfchar(cmap, wbuf, 2, wbuf + 2, len);
                    count++;

                    if (verbose > VERBOSE_LEVEL_MIN) {
                        size_t _i;

                        dpx_message("otf_cmap>> Additional ToUnicode mapping: <%04X> <", gid);
                        for (_i = 0; _i < len; _i++) {
                            dpx_message("%02X", wbuf[2 + _i]);
                        }
                        dpx_message(">\n");
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
    unsigned int offset = 0;

    if (sfont->type != SFNT_TYPE_POSTSCRIPT     ||
        sfnt_read_table_directory(sfont, 0) < 0 ||
        (offset = sfnt_find_table_pos(sfont, "CFF ")) == 0) {
        return NULL;
    }

    cffont = cff_open(sfont->handle, offset, 0);
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

    /* Skip PUA characters and alphabetic presentation forms, allowing
     * handle_subst_glyphs() as it might find better mapping. Fixes the
     * mapping of ligatures encoded in PUA in fonts like Linux Libertine
     * and old Adobe fonts.
     */
    if (is_used_char2(used_chars, cid) && !is_PUA_or_presentation(ch)) {
        int len;
        unsigned char *p = wbuf + 2;

        count++;

        wbuf[0] = (cid >> 8) & 0xff;
        wbuf[1] = (cid & 0xff);
        len = UC_UTF16BE_encode_char((int32_t) ch, &p, wbuf + WBUF_SIZE);
        CMap_add_bfchar(cmap, wbuf, 2, wbuf + 2, len);

        /* Avoid duplicate entry
         * There are problem when two Unicode code is mapped to
         * single glyph...
         */
        used_chars[cid / 8] &= ~(1 << (7 - (cid % 8)));
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

    /* cmap_add here stores information about all unencoded glyphs which can be
     * accessed only through OT Layout GSUB table.
     */
    if (code_to_cid_cmap && cffont && is_cidfont && !cmap_add) {
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
    char       *cmap_name, *cmap_add_name;
    rust_input_handle_t handle = NULL;
    sfnt       *sfont;
    ULONG       offset = 0;
    int         cmap_type;
    size_t      i;

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
    free(normalized_font_name);

    res_id = pdf_findresource("CMap", cmap_name);
    if (res_id >= 0) {
        free(cmap_name);
        cmap_ref = pdf_get_resource_reference(res_id);
        return cmap_ref;
    }

    if (verbose > VERBOSE_LEVEL_MIN) {
        dpx_message("\n");
        dpx_message("otf_cmap>> Creating ToUnicode CMap for \"%s\"...\n", font_name);
    }

    if ((handle = dpx_open_truetype_file(font_name)) ||
        (handle = dpx_open_opentype_file(font_name))) {
        sfont = sfnt_open(handle);
    } else if ((handle = dpx_open_dfont_file(font_name))) {
        sfont = dfont_open(handle, ttc_index);
    } else {
        free(cmap_name);
        return NULL;
    }

    if (!sfont) {
        _tt_abort("Could not open OpenType/TrueType font file \"%s\"", font_name);
    }

    switch (sfont->type) {
    case SFNT_TYPE_DFONT:
        offset = sfont->offset;
        break;
    case SFNT_TYPE_TTC:
        offset = ttc_read_offset(sfont, ttc_index);
        if (offset == 0) {
            _tt_abort("Invalid TTC index");
        }
        break;
    default:
        offset = 0;
        break;
    }

    if (sfnt_read_table_directory(sfont, offset) < 0) {
        _tt_abort("Could not read OpenType/TrueType table directory.");
    }

    code_to_cid_cmap = CMap_cache_get(cmap_id);
    cmap_type = CMap_get_type(code_to_cid_cmap);
    if (cmap_type != CMAP_TYPE_CODE_TO_CID)
        code_to_cid_cmap = NULL;

    cmap_add_name = NEW(strlen(font_name) + strlen(",000-UCS32-Add") + 1, char);
    sprintf(cmap_add_name, "%s,%03d-UCS32-Add", font_name, ttc_index);
    cmap_add_id = CMap_cache_find(cmap_add_name);
    free(cmap_add_name);
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
    if (cmap_obj == NULL)
        dpx_warning("Unable to read OpenType/TrueType Unicode cmap table.");
    tt_cmap_release(ttcmap);
    CMap_set_silent(0);

    if (cmap_obj) {
        res_id   = pdf_defineresource("CMap", cmap_name,
                                      cmap_obj, PDF_RES_FLUSH_IMMEDIATE);
        cmap_ref = pdf_get_resource_reference(res_id);
    } else {
        cmap_ref = NULL;
    }
    free(cmap_name);

    sfnt_close(sfont);
    if (handle)
        ttstub_input_close(handle);

    return cmap_ref;
}


static int
load_base_CMap (const char *cmap_name, CMap *tounicode_add, int wmode,
                CIDSysInfo *csi, unsigned char *GIDToCIDMap,
                otl_gsub *gsub_vert, otl_gsub *gsub_list,
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
            load_cmap12(ttcmap->map, GIDToCIDMap, gsub_vert, gsub_list, cmap, tounicode_add);
        } else if (ttcmap->format == 4) {
            load_cmap4(ttcmap->map, GIDToCIDMap, gsub_vert, gsub_list, cmap, tounicode_add);
        }

        cmap_id = CMap_cache_add(cmap);
    }

    return cmap_id;
}


int
otf_load_Unicode_CMap (const char *map_name, int ttc_index, /* 0 for non-TTC font */
                       const char *otl_tags, int wmode)
{
    int    cmap_id = -1;
    /* Additional ToUnicode mappings required by OTL GSUB substitusion */
    int    tounicode_add_id = -1;
    CMap  *tounicode_add = NULL;
    char  *tounicode_add_name = NULL;
    int    is_cidfont = 0;
    sfnt  *sfont;
    ULONG  offset = 0;
    char  *base_name = NULL, *cmap_name = NULL;
    rust_input_handle_t *handle = NULL;
    otl_gsub      *gsub_vert = NULL, *gsub_list = NULL;
    tt_cmap       *ttcmap;
    CIDSysInfo     csi = {NULL, NULL, 0};
    unsigned char *GIDToCIDMap = NULL;

    if (!map_name)
        return -1;

    if (ttc_index > 999 || ttc_index < 0) {
        return -1; /* Sorry for this... */
    }

    handle = dpx_open_truetype_file(map_name);
    if (handle == NULL)
        handle = dpx_open_opentype_file(map_name);

    if (handle == NULL) {
        handle = dpx_open_dfont_file(map_name);
        if (handle == NULL)
            return -1;

        sfont = dfont_open(handle, ttc_index);
    } else {
        sfont = sfnt_open(handle);
    }

    if (!sfont) {
        _tt_abort("Could not open OpenType/TrueType/dfont font file \"%s\"", map_name);
    }
    switch (sfont->type) {
    case SFNT_TYPE_TTC:
        offset = ttc_read_offset(sfont, ttc_index);
        if (offset == 0) {
            _tt_abort("Invalid TTC index");
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
        _tt_abort("Not a OpenType/TrueType/TTC font?: %s", map_name);
        break;
    }

    if (sfnt_read_table_directory(sfont, offset) < 0)
        _tt_abort("Could not read OpenType/TrueType table directory.");

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

        /* tounicode_add here is later refered by otf_create_ToUnicode_stream()
         * for finding additional CID to Unicode mapping entries required by
         * OTL gsub substitution.
         */
        tounicode_add_name = NEW(strlen(map_name) + strlen(",000-UCS32-Add") + 1, char);
        sprintf(tounicode_add_name, "%s,%03d-UCS32-Add", map_name, ttc_index);
        tounicode_add_id = CMap_cache_find(tounicode_add_name);

        if (tounicode_add_id >= 0) {
            tounicode_add = CMap_cache_get(tounicode_add_id);
        } else {
            tounicode_add = CMap_new();
            CMap_set_name (tounicode_add, tounicode_add_name);
            CMap_set_type (tounicode_add, CMAP_TYPE_TO_UNICODE);
            CMap_set_wmode(tounicode_add, 0);
            CMap_add_codespacerange(tounicode_add, srange_min, srange_max, 2);
            CMap_set_CIDSysInfo(tounicode_add, &CSI_UNICODE);
            CMap_add_bfchar(tounicode_add, srange_min, 2, srange_max, 2);
            tounicode_add_id = CMap_cache_add(tounicode_add);
        }

        free(tounicode_add_name);
    } else {
        cmap_name = NEW(strlen(base_name)+1, char);
        strcpy(cmap_name, base_name);
    }

    if (sfont->type == SFNT_TYPE_POSTSCRIPT) {
        is_cidfont = handle_CIDFont(sfont, &GIDToCIDMap, &csi);
    } else {
        is_cidfont = 0;
    }

    if (verbose > VERBOSE_LEVEL_MIN) {
        dpx_message("\n");
        dpx_message("otf_cmap>> Unicode charmap for font=\"%s\" layout=\"%s\"\n",
                    map_name, (otl_tags ? otl_tags : "none"));
    }

    cmap_id = CMap_cache_find(cmap_name);
    if (cmap_id >= 0) {
        free(cmap_name);
        free(base_name);
        free(GIDToCIDMap);

        sfnt_close(sfont);
        ttstub_input_close(handle);

        if (verbose > VERBOSE_LEVEL_MIN)
            dpx_message("otf_cmap>> Found at cmap_id=%d.\n", cmap_id);

        return cmap_id;
    }

    ttcmap = tt_cmap_read(sfont, 3, 10); /* Microsoft UCS4 */
    if (!ttcmap) {
        ttcmap = tt_cmap_read(sfont, 3, 1); /* Microsoft UCS2 */
        if (!ttcmap) {
            ttcmap = tt_cmap_read(sfont, 0, 3); /* Unicode 2.0 or later */
            if (!ttcmap) {
                _tt_abort("Unable to read OpenType/TrueType Unicode cmap table.");
            }
        }
    }

    if (wmode == 1) {
        gsub_vert = otl_gsub_new();
        if (otl_gsub_add_feat(gsub_vert, "*", "*", "vrt2", sfont) < 0) {
            if (otl_gsub_add_feat(gsub_vert, "*", "*", "vert", sfont) < 0) {
                dpx_warning("GSUB feature vrt2/vert not found.");
                otl_gsub_release(gsub_vert);
                gsub_vert = NULL;
            } else {
                otl_gsub_select(gsub_vert, "*", "*", "vert");
            }
        } else {
            otl_gsub_select(gsub_vert, "*", "*", "vrt2");
        }
    } else {
        gsub_vert = NULL;
    }

    if (otl_tags) {
        gsub_list = otl_gsub_new();
        if (otl_gsub_add_feat_list(gsub_list, otl_tags, sfont) < 0) {
            dpx_warning("Reading GSUB feature table(s) failed for \"%s\"", otl_tags);
        } else {
            otl_gsub_set_chain(gsub_list, otl_tags);
        }
    } else {
        gsub_list = NULL;
    }

    cmap_id = load_base_CMap(cmap_name, tounicode_add, wmode,
                             (is_cidfont ? &csi : NULL), GIDToCIDMap,
                             gsub_vert, gsub_list, ttcmap);

    if (cmap_id < 0)
        _tt_abort("Failed to read OpenType/TrueType cmap table.");

    if (gsub_vert)
        otl_gsub_release(gsub_vert);
    gsub_vert = NULL;

    if (gsub_list)
        otl_gsub_release(gsub_list);
    gsub_list = NULL;

    free(cmap_name);
    free(base_name);
    free(GIDToCIDMap);

    if (is_cidfont) {
        free(csi.registry);
        free(csi.ordering);
    }

    tt_cmap_release(ttcmap);
    sfnt_close(sfont);
    ttstub_input_close(handle);
    return cmap_id;
}
