/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
/* Sorry for placing this here.
 * We need to rewrite TrueType font support code...
 */
#include "dpx-cmap.h"
#include "dpx-cmap_write.h"
#include "dpx-dpxconf.h"
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

    if (len < 256) {
        dpx_warning("invalid format 0 TT cmap subtable");
        return NULL;
    }

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

    if (len < 512) {
        dpx_warning("invalid fromt2 TT cmap subtable");
        return NULL;
    }

    map = NEW(1, struct cmap2);
    for (i = 0; i < 256; i++)
        map->subHeaderKeys[i] = sfnt_get_ushort(sfont);
    for (n = 0, i = 0; i < 256; i++) {
        map->subHeaderKeys[i] /= 8;
        if (n < map->subHeaderKeys[i])
            n = map->subHeaderKeys[i];
    }
    n += 1; /* the number of subHeaders is one plus the max of subHeaderKeys */
    if (len < 512 +  n * 8 ) {
        dpx_warning("invalid/truncated format2 TT cmap subtable");
        free(map);
        return NULL;
    }

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

    if (len < 8) {
        dpx_warning("invalid format 4 TT cmap subtable");
        return NULL;
    }

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
    while (i-- > 0 && cc <= map->endCount[i]) {
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

    if (len < 4) {
        dpx_warning("invalid foramt 6 TT cmap subtable");
        return NULL;
    }

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

    if (len < 4) {
        dpx_warning("invalid format 12 TT cmap subtable");
        return NULL;
    }

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
            switch (cmap->format) {
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
                dpx_warning("Unrecognized OpenType/TrueType cmap format: %d", cmap->format);
                break;
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
        dpx_warning("Unrecognized OpenType/TrueType cmap subtable format: %d", cmap->format);
        break;
    }

    return gid;
}

static unsigned char srange_min[2] = {0x00, 0x00};
static unsigned char srange_max[2] = {0xff, 0xff};
static unsigned char lrange_min[4] = {0x00, 0x00, 0x00, 0x00};
static unsigned char lrange_max[4] = {0x7f, 0xff, 0xff, 0xff};

/* OpenType CIDFont:
 *
 *  We don't use GID for them. OpenType cmap table is for
 *  charcode to GID mapping rather than to-CID mapping.
 */
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_types.h"
#include "dpx-cid.h"
#include "dpx-tt_table.h"

static void
create_GIDToCIDMap (uint16_t *GIDToCIDMap, uint16_t num_glyphs, cff_font *cffont)
{
    cff_charsets *charset;
    uint16_t      gid, i;

    assert(GIDToCIDMap);

    if (!cffont || !(cffont->flag & FONTTYPE_CIDFONT)) {
        for (gid = 0; gid < num_glyphs; gid++) {
            GIDToCIDMap[gid] = gid;
        }

        return;
    }

    memset(GIDToCIDMap, 0, num_glyphs*sizeof(uint16_t));

    charset = cffont->charsets;
    if (!charset)
        return;
    switch (charset->format) {
    case 0:
    {
        s_SID   *cids; /* CID... */

        cids = charset->data.glyphs;
        for (gid = 1, i = 0; i < charset->num_entries; i++) {
            GIDToCIDMap[gid] = cids[i];
            gid++;
        }
    }
    break;
    case 1:
    {
        cff_range1 *ranges;
        card16      cid, count;

        ranges = charset->data.range1;
        for (gid = 1, i = 0; i < charset->num_entries; i++) {
            cid   = ranges[i].first;
            count = ranges[i].n_left + 1; /* card8 */
            while (count-- > 0 && gid <= num_glyphs) {
                GIDToCIDMap[gid] = cid;
                gid++;
                cid++;
            }
        }
    }
    break;
    case 2:
    {
        cff_range2 *ranges;
        card16      cid, count;

        ranges = charset->data.range2;
        if (charset->num_entries == 1 && ranges[0].first == 1) {
            /* "Complete" CIDFont */
            for (gid = 0; gid < num_glyphs; gid++) {
                GIDToCIDMap[gid] = gid;
            }
        } else {
            /* Not trivial mapping */
            for (gid = 1, i = 0; i < charset->num_entries; i++) {
                cid   = ranges[i].first;
                count = ranges[i].n_left + 1;
                while (count-- > 0 && gid <= num_glyphs) {
                    GIDToCIDMap[gid] = cid;
                    gid++;
                    cid++;
                }
            }
        }
    }
    break;
    default:
        dpx_warning("Unknown CFF charset format...: %d", charset->format);
        break;
    }

    return;
}

static bool is_PUA_or_presentation (unsigned int uni)
{
  /* Some of CJK Radicals Supplement and Kangxi Radicals
   * are commonly double encoded, lower the priority.
   * CJK Compatibility Ideographs & Supplement added.
   */
  return  ((uni >= 0x2E80 && uni <= 0x2EF3) || (uni >= 0x2F00 && uni <= 0x2FD5) ||
           (uni >= 0xE000 && uni <= 0xF8FF) || (uni >= 0xFB00 && uni <= 0xFB4F) ||
           (uni >= 0xF900 && uni <= 0xFAFF) || (uni >= 0x2F800 && uni <= 0x2FA1F) ||
           (uni >= 0xF0000 && uni <= 0xFFFFD) || (uni >= 0x100000 && uni <= 0x10FFFD));
}

static char*
lookup_glyph_name(struct tt_post_table *post, cff_font *cffont, USHORT gid)
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

static int32_t
handle_subst_glyphs (CMap *cmap, CMap *cmap_add, char *used_chars)
{
    int32_t count = 0;
    int32_t cid;

    for (cid = 0; cid < 65536; cid++) {
        if (!is_used_char2(used_chars, cid))
            continue;
        else {
            unsigned char        buf[256];
            size_t               inbytesleft = 2, outbytesleft = 254;
            size_t               len;
            unsigned char       *outbuf = buf + 2;
            const unsigned char *inbuf  = buf;

            buf[0] = (cid >> 8) & 0xff;
            buf[1] =  cid & 0xff;
            CMap_decode(cmap_add, &inbuf, &inbytesleft, &outbuf, &outbytesleft);
            if (inbytesleft == 0) {
                len = 254 - outbytesleft;
                CMap_add_bfchar(cmap, buf, 2, buf + 2, len);
                used_chars[cid / 8] &= ~(1 << (7 - (cid % 8)));
                count++;
            }
        }
    }

    return count;
}

static int32_t
add_ToUnicode_via_glyph_name (CMap *cmap, char *used_chars, USHORT num_glyphs,
                              uint16_t *GIDToCIDMap,
                              sfnt *sfont, cff_font *cffont)
{
    int32_t               count = 0;
    USHORT                gid;
    struct tt_post_table *post = NULL;

    post = tt_read_post_table(sfont);
    if (!post && !cffont)
        return count;

    for (gid = 0; gid < num_glyphs; gid++) {
        uint16_t cid = GIDToCIDMap[gid];
        if (is_used_char2(used_chars, cid)) {
#define MAX_UNICODES 32
            char   *name;
            int32_t unicodes[MAX_UNICODES];
            int     unicode_count = -1;

            name = lookup_glyph_name(post, cffont, gid);
            if (name) {
                unicode_count = agl_get_unicodes(name, unicodes, MAX_UNICODES);
#undef MAX_UNICODES
                free(name);
                if (unicode_count > 0) {
                    unsigned char *buf;
                    unsigned char *p, *endptr;
                    int            k;
                    size_t         len = 0;

                    buf    = NEW(unicode_count*4+2, unsigned char);
                    p      = buf + 2;
                    endptr = buf + (unicode_count * 4 + 2);
                    for (k = 0; k < unicode_count; ++k) {
                        len += UC_UTF16BE_encode_char(unicodes[k], &p, endptr);
                    }
                    buf[0] = (cid >> 8) & 0xff;
                    buf[1] =  cid & 0xff;
                    CMap_add_bfchar(cmap, buf, 2, buf + 2, len);
                    used_chars[cid / 8] &= ~(1 << (7 - (cid % 8)));
                    count++;

                    free(buf);
                }
            }
        }
    }

    if (post)
        tt_release_post_table(post);

    return count;
}

static void
create_inverse_cmap4 (int32_t *map_base, int32_t *map_sub, USHORT num_glyphs,
                      struct cmap4 *map)
{
  USHORT segCount = map->segCountX2 / 2;
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
      if (is_PUA_or_presentation(ch)) {
        map_sub[gid] = ch;
      } else {
        map_base[gid] = ch;
      }
    }
  }
}

static void
create_inverse_cmap12 (int32_t *map_base, int32_t *map_sub, USHORT num_glyphs,
                       struct cmap12 *map)
{
  ULONG i, ch;

  for (i = 0; i < map->nGroups; i++) {
    for (ch  = map->groups[i].startCharCode;
         ch <= map->groups[i].endCharCode; ch++) {
      int d = ch - map->groups[i].startCharCode;
      USHORT gid = (USHORT) ((map->groups[i].startGlyphID + d) & 0xffff);
      if (is_PUA_or_presentation(ch)) {
        map_sub[gid] = ch;
      } else {
        map_base[gid] = ch;
      }
    }
  }
}

/* NOTE: Reverse mapping code which had been placed here is removed since:
 *  - Implementation of reserve CMap mapping itself is imcomplete.
 *  - It is wrong to assume that all CMap passed here is Unicode to CID mapping.
 * Especially, the second one causes problems.
 */
static pdf_obj *
create_ToUnicode_cmap (tt_cmap *ttcmap,
                       const char *cmap_name,
                       CMap *cmap_add,
                       const char *used_chars,
                       sfnt *sfont)
{
    pdf_obj  *stream = NULL;
    int32_t *map_base = NULL, *map_sub = NULL;
    USHORT gid, num_glyphs = 0;

    assert(ttcmap);

    /* Get num_glyphs from maxp talbe */
    {
        struct tt_maxp_table *maxp;

        maxp = tt_read_maxp_table(sfont);
        if (maxp) {
            num_glyphs = maxp->numGlyphs;
            free(maxp);
        }
    }

    /* Initialize GID to Unicode mapping table */
    map_base = NEW(num_glyphs, int32_t);
    map_sub  = NEW(num_glyphs, int32_t);
    for (gid = 0; gid < num_glyphs; gid++) {
        map_base[gid] = -1;
        map_sub [gid] = -1;
    }

    /* Create "base" mapping from inverse mapping of OpenType cmap */
    switch (ttcmap->format) {
    case 4:
        create_inverse_cmap4(map_base, map_sub, num_glyphs, ttcmap->map);
        break;
    case 12:
        create_inverse_cmap12(map_base, map_sub, num_glyphs, ttcmap->map);
        break;
    }

    /* Now create ToUnicode CMap stream */
    {
        CMap     *cmap;
        int32_t   count;
        cff_font *cffont      = NULL;
        char      is_cidfont  = 0;
        uint16_t *GIDToCIDMap = NULL;
        char     *used_chars_copy = NULL;

        if (sfont->type == SFNT_TYPE_POSTSCRIPT) {
            ULONG offset;
            offset = sfnt_find_table_pos(sfont, "CFF ");
            cffont = cff_open(sfont->handle, offset, 0);
            cff_read_charsets(cffont);
        }
        is_cidfont = cffont && (cffont->flag & FONTTYPE_CIDFONT);

        /* GIT to CID mapping info. */
        GIDToCIDMap = NEW(num_glyphs, uint16_t);
        if (is_cidfont) {
            create_GIDToCIDMap(GIDToCIDMap, num_glyphs, cffont);
        } else {
            for (gid = 0; gid < num_glyphs; gid++) {
                GIDToCIDMap[gid] = gid;
            }
        }
        cmap = CMap_new();
        CMap_set_name (cmap, cmap_name);
        CMap_set_wmode(cmap, 0);
        CMap_set_type (cmap, CMAP_TYPE_TO_UNICODE);
        CMap_set_CIDSysInfo(cmap, &CSI_UNICODE);
        CMap_add_codespacerange(cmap, srange_min, srange_max, 2);

        count = 0;
        used_chars_copy = NEW(8192, char);
        memcpy(used_chars_copy, used_chars, 8192);
        for (gid = 0; gid < num_glyphs; gid++) {
            uint16_t cid = GIDToCIDMap[gid];
            if (is_used_char2(used_chars_copy, cid)) {
                int32_t        ch;
                unsigned char  src[2], dst[4];
                unsigned char *p = dst, *endptr = dst + 4;
                size_t         len;

                ch = map_base[gid];
                if (UC_is_valid(ch)) {
                    src[0] = (cid >> 8) & 0xff;
                    src[1] = cid & 0xff;
                    len = UC_UTF16BE_encode_char(ch, &p, endptr);
                    CMap_add_bfchar(cmap, src, 2, dst, len);
                    used_chars_copy[cid / 8] &= ~(1 << (7 - (cid % 8)));
                    count++;
                }
            }
        }

        /* cmap_add here stores information about all unencoded glyphs which can be
        * accessed only through OT Layout GSUB table.
        * This is only availabel when encoding is "unicode".
        */
        if (cmap_add) {
            count += handle_subst_glyphs(cmap, cmap_add, used_chars_copy);
        } else {
            /* Else, try gathering information from GSUB tables */
            count += otl_gsub_add_ToUnicode(cmap, used_chars_copy,
                                            map_base, map_sub, num_glyphs,
                                            GIDToCIDMap, sfont);
        }
        /* Find Unicode mapping via PostScript glyph names... */
        count += add_ToUnicode_via_glyph_name(cmap, used_chars_copy, num_glyphs,
                                            GIDToCIDMap, sfont, is_cidfont ? NULL : cffont);
        if (cffont)
            cff_close(cffont);

        /* Finaly, PUA and presentation forms... */
        for (gid = 0; gid < num_glyphs; gid++) {
            uint16_t cid = GIDToCIDMap[gid];
            if (is_used_char2(used_chars_copy, cid)) {
                int32_t        ch;
                unsigned char  src[2], dst[4];
                unsigned char *p = dst, *endptr = dst + 4;
                size_t         len;

                ch = map_sub[gid];
                if (UC_is_valid(ch)) {
                    src[0] = (cid >> 8) & 0xff;
                    src[1] = cid & 0xff;
                    len = UC_UTF16BE_encode_char(ch, &p, endptr);
                    CMap_add_bfchar(cmap, src, 2, dst, len);
                    used_chars_copy[cid / 8] &= ~(1 << (7 - (cid % 8)));
                    count++;
                }
            }
        }

        /* Check for missing mapping */
        if (dpx_conf.verbose_level > VERBOSE_LEVEL_MIN) {
            for (gid = 0; gid < num_glyphs; gid++) {
                uint16_t cid = GIDToCIDMap[gid];
                if (is_used_char2(used_chars_copy, cid)) {
                    dpx_warning("Unable to find ToUnicode mapping for glyph CID=%u (GID=%u)", cid, gid);
                }
            }
        }
        free(GIDToCIDMap);
        free(used_chars_copy);

        if (count < 1)
            stream = NULL;
        else {
            stream = CMap_create_stream(cmap);
        }
        CMap_release(cmap);
    }
    free(map_base);
    free(map_sub);

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
                             const char *basefont,
                             const char *used_chars)
{
    pdf_obj    *cmap_ref = NULL; /* returned value */
    CMap       *cmap_add;
    char       *cmap_name;
    rust_input_handle_t handle = NULL;
    sfnt       *sfont;
    ULONG       offset = 0;
    tt_cmap    *ttcmap;
    int         cmap_id, cmap_add_id;
    size_t      i;

    cmap_name = NEW(strlen(basefont)+strlen("-UTF16")+1, char);
    sprintf(cmap_name, "%s-UTF16", basefont);

    cmap_id = pdf_findresource("CMap", cmap_name);
    if (cmap_id >= 0) {
        free(cmap_name);
        cmap_ref = pdf_get_resource_reference(cmap_id);
        return cmap_ref;
    }

    if (dpx_conf.verbose_level > VERBOSE_LEVEL_MIN) {
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
        dpx_warning("Could not open OpenType/TrueType font file \"%s\"", font_name);
        free(cmap_name);
        ttstub_input_close(handle);
        return NULL;
    }

    switch (sfont->type) {
    case SFNT_TYPE_DFONT:
        offset = sfont->offset;
        break;
    case SFNT_TYPE_TTC:
        offset = ttc_read_offset(sfont, ttc_index);
        if (offset == 0) {
            dpx_warning("Invalid TTC index for font: %s", font_name);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            free(cmap_name);
            return NULL;
        }
        break;
    default:
        offset = 0;
        break;
    }

    if (sfnt_read_table_directory(sfont, offset) < 0) {
        dpx_warning("Could not read OpenType/TrueType table directory for font: %s", font_name);
        sfnt_close(sfont);
        ttstub_input_close(handle);
        free(cmap_name);
        return NULL;
    }

    /* cmap_add is used for storing information on ToUnicode mapping for
    * unencoded glyphs which can be reached only through GSUB substitution.
    * This is available only when "unicode" is specified in the encoding
    * field of fontmap. We remember the inverse mapping via cmap_add in this
    * case.
    */
    {
        char *cmap_add_name;
        cmap_add_name = NEW(strlen(font_name) + strlen(",000-UCS32-Add") + 1, char);
        sprintf(cmap_add_name, "%s,%03d-UCS32-Add", font_name, ttc_index);
        cmap_add_id = CMap_cache_find(cmap_add_name);
        free(cmap_add_name);
        if (cmap_add_id < 0) {
            cmap_add = NULL;
        } else {
            cmap_add = CMap_cache_get(cmap_add_id);
        }
    }

    ttcmap = NULL;
    for (i = 0; i < sizeof(cmap_plat_encs) / sizeof(cmap_plat_enc_rec); ++i) {
        ttcmap = tt_cmap_read(sfont, cmap_plat_encs[i].platform, cmap_plat_encs[i].encoding);
        if (!ttcmap)
            continue;

        if (ttcmap->format == 4 || ttcmap->format == 12) {
             break;
        } else {
            tt_cmap_release(ttcmap);
            ttcmap = NULL;
        }
    }

    if (ttcmap) {
        pdf_obj *cmap_obj;

        CMap_set_silent(1); /* many warnings without this... */
        cmap_obj = create_ToUnicode_cmap(ttcmap, cmap_name, cmap_add, used_chars, sfont);
        CMap_set_silent(0);
        if (cmap_obj) {
            cmap_id = pdf_defineresource("CMap", cmap_name,
                                      cmap_obj, PDF_RES_FLUSH_IMMEDIATE);
            cmap_ref = pdf_get_resource_reference(cmap_id);
        }
        tt_cmap_release(ttcmap);
    }

    /* Cleanup */
    free(cmap_name);
    sfnt_close(sfont);
    if (handle)
        ttstub_input_close(handle);

    if (!cmap_ref) {
        dpx_warning("Creating ToUnicode CMap failed for \"%s\"", font_name);
    }

    return cmap_ref;
}

/* Creating input CMaps from OT cmap table */

static void
load_cmap4 (struct cmap4 *map, uint16_t *GIDToCIDMap, USHORT num_glyphs,
            otl_gsub *gsub_vert, otl_gsub *gsub_list,
            CMap *cmap, int32_t *map_base, int32_t *map_sub)
{
  USHORT        c0, c1, gid, cid;
  USHORT        j, d, segCount;
  USHORT        ch;
  int           i;
  unsigned char buf[4];

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
        /* Apply GSUB features */
        if (gsub_list)
          otl_gsub_apply_chain(gsub_list, &gid);
        if (gsub_vert)
          otl_gsub_apply(gsub_vert, &gid);
        cid = (gid < num_glyphs) ? GIDToCIDMap[gid] : 0;
        buf[0] = 0;
        buf[1] = 0;
        buf[2] = (ch >> 8) & 0xff;
        buf[3] =  ch & 0xff;
        CMap_add_cidchar(cmap, buf, 4, cid);
        /* For ToUnicode creation */
        if (map_base && map_sub) {
          if (is_PUA_or_presentation(ch)) {
            map_sub[gid] = ch;
          } else {
            map_base[gid] = ch;
          }
        }
      }
    }
  }

  return;
}

static void
load_cmap12 (struct cmap12 *map, uint16_t *GIDToCIDMap, USHORT num_glyphs,
             otl_gsub *gsub_vert, otl_gsub *gsub_list,
             CMap *cmap, int32_t *map_base, int32_t *map_sub)
{
  ULONG         i, ch;
  USHORT        gid, cid;
  unsigned char buf[4];

  for (i = 0; i < map->nGroups; i++) {
    for (ch  = map->groups[i].startCharCode;
         ch <= map->groups[i].endCharCode; ch++) {
      int  d = ch - map->groups[i].startCharCode;
      gid = (USHORT) ((map->groups[i].startGlyphID + d) & 0xffff);
      if (gsub_list)
        otl_gsub_apply_chain(gsub_list, &gid);
      if (gsub_vert)
        otl_gsub_apply(gsub_vert, &gid);
      cid = (gid < num_glyphs) ? GIDToCIDMap[gid] : 0;
      buf[0] = (ch >> 24) & 0xff;
      buf[1] = (ch >> 16) & 0xff;
      buf[2] = (ch >>  8) & 0xff;
      buf[3] = ch & 0xff;
      CMap_add_cidchar(cmap, buf, 4, cid);
      if (map_base && map_sub) {
        if (is_PUA_or_presentation(ch)) {
          map_sub[gid] = ch;
        } else {
          map_base[gid] = ch;
        }
      }
    }
  }

  return;
}

int
otf_load_Unicode_CMap (const char *map_name, int ttc_index, /* 0 for non-TTC font */
                       const char *otl_tags, int wmode)
{
    int    cmap_id = -1;
    char   *cmap_name = NULL;
    sfnt  *sfont = NULL;
    ULONG  offset = 0;
    uint16_t num_glyphs = 0;
    rust_input_handle_t handle = NULL;
    tt_cmap       *ttcmap;
    CIDSysInfo     csi = {NULL, NULL, 0};
    uint16_t *GIDToCIDMap = NULL;

    if (!map_name)
        return -1;

    if (ttc_index > 999 || ttc_index < 0) {
        return -1; /* Sorry for this... */
    }

    /* First look for cache if it was already loaded */
    cmap_name = NEW(strlen(map_name)+strlen("-UCS4-H")+5, char);
    if (otl_tags) {
        cmap_name = NEW(strlen(map_name)+strlen(otl_tags)+strlen("-UCS4-H")+6, char);
        if (wmode)
            sprintf(cmap_name, "%s,%03d,%s-UCS4-V", map_name, ttc_index, otl_tags);
        else
            sprintf(cmap_name, "%s,%03d,%s-UCS4-H", map_name, ttc_index, otl_tags);
    } else {
        if (wmode)
            sprintf(cmap_name, "%s,%03d-UCS4-V", map_name, ttc_index);
        else {
            sprintf(cmap_name, "%s,%03d-UCS4-H", map_name, ttc_index);
        }
    }
    cmap_id = CMap_cache_find(cmap_name);
    if (cmap_id >= 0) {
        free(cmap_name);
        if (dpx_conf.verbose_level > VERBOSE_LEVEL_MIN)
            dpx_message("otf_cmap>> Found at cmap_id=%d.\n", cmap_id);

        return cmap_id;
    }

    /* CMap not found */
    if (dpx_conf.verbose_level > VERBOSE_LEVEL_MIN) {
        dpx_message("\n");
        dpx_message("otf_cmap>> Creating Unicode charmap for font=\"%s\" layout=\"%s\"\n",
                map_name, (otl_tags ? otl_tags : "none"));
    }

    handle = dpx_open_truetype_file(map_name);
    if (handle == NULL)
        handle = dpx_open_opentype_file(map_name);

    if (handle == NULL) {
        handle = dpx_open_dfont_file(map_name);
        if (handle == NULL) {
            free(cmap_name);
            return -1;
        }
        sfont = dfont_open(handle, ttc_index);
    } else {
        sfont = sfnt_open(handle);
    }

    if (!sfont) {
        dpx_warning("Could not open OpenType/TrueType/dfont font file \"%s\"", map_name);
        free(cmap_name);
        ttstub_input_close(handle);
        return -1;
    }
    switch (sfont->type) {
    case SFNT_TYPE_TTC:
        offset = ttc_read_offset(sfont, ttc_index);
        if (offset == 0) {
            dpx_warning("Offset=0 reurned for font=%s, TTC_index=%d", map_name, ttc_index);
            free(cmap_name);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
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
        dpx_warning("Not a OpenType/TrueType/TTC font?: %s", map_name);
        free(cmap_name);
        sfnt_close(sfont);
        ttstub_input_close(handle);
        return -1;
    }

    if (sfnt_read_table_directory(sfont, offset) < 0) {
        dpx_warning("Could not read OpenType/TrueType table directory.");
        free(cmap_name);
        sfnt_close(sfont);
        ttstub_input_close(handle);
        return -1;
    }

    {
        struct tt_maxp_table *maxp;

        maxp       = tt_read_maxp_table(sfont);
        num_glyphs = (card16) maxp->numGlyphs;
        free(maxp);
    }

    GIDToCIDMap = NEW(num_glyphs, uint16_t);
    memset(GIDToCIDMap, 0, num_glyphs*sizeof(uint16_t));
    if (sfont->type == SFNT_TYPE_POSTSCRIPT) {
        cff_font             *cffont;
        card16                gid;

        offset = sfnt_find_table_pos(sfont, "CFF ");
        cffont = cff_open(sfont->handle, offset, 0);
        if (!cffont) {
            free(cmap_name);
            free(GIDToCIDMap);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
        }
        if (!(cffont->flag & FONTTYPE_CIDFONT)) {
            csi.registry   = strdup("Adobe");
            csi.ordering   = strdup("Identity");
            csi.supplement = 0;
            for (gid = 0; gid < num_glyphs; gid++) {
                GIDToCIDMap[gid] = gid;
            }
        } else {
            if (!cff_dict_known(cffont->topdict, "ROS")) {
                csi.registry   = strdup("Adobe");
                csi.ordering   = strdup("Identity");
                csi.supplement = 0;
            } else {
                card16 reg, ord;

                reg = (card16) cff_dict_get(cffont->topdict, "ROS", 0);
                ord = (card16) cff_dict_get(cffont->topdict, "ROS", 1);
                csi.registry   = cff_get_string(cffont, reg);
                csi.ordering   = cff_get_string(cffont, ord);
                csi.supplement = (int) cff_dict_get(cffont->topdict, "ROS", 2);
            }
            cff_read_charsets(cffont);
            create_GIDToCIDMap(GIDToCIDMap, num_glyphs, cffont);
        }
        cff_close(cffont);
    } else {
        uint16_t gid;

        csi.registry   = strdup("Adobe");
        csi.ordering   = strdup("Identity");
        csi.supplement = 0;
        for (gid = 0; gid < num_glyphs; gid++) {
            GIDToCIDMap[gid] = gid;
        }
    }

    ttcmap = tt_cmap_read(sfont, 3, 10); /* Microsoft UCS4 */
    if (!ttcmap) {
        ttcmap = tt_cmap_read(sfont, 3, 1); /* Microsoft UCS2 */
        if (!ttcmap) {
            ttcmap = tt_cmap_read(sfont, 0, 3); /* Unicode 2.0 or later */
        }
    }

    if (ttcmap) {
        CMap     *cmap      = NULL;
        int32_t  *map_base, *map_sub;
        otl_gsub *gsub_vert = NULL;
        otl_gsub *gsub_list = NULL;
        uint32_t  gid;

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
        cmap = CMap_new();
        CMap_set_name(cmap, cmap_name);
        CMap_set_type(cmap, CMAP_TYPE_CODE_TO_CID);
        CMap_set_wmode(cmap, wmode);
        CMap_add_codespacerange(cmap, lrange_min, lrange_max, 4);
        CMap_set_CIDSysInfo(cmap, &csi);
        map_base = NEW(num_glyphs, int32_t);
        map_sub  = NEW(num_glyphs, int32_t);
        for (gid = 0; gid < num_glyphs; gid++) {
            map_base[gid] = -1;
            map_sub[gid]  = -1;
        }
        switch (ttcmap->format) {
        case 12:
            load_cmap12(ttcmap->map, GIDToCIDMap, num_glyphs,
                        gsub_vert, gsub_list,
                        cmap, map_base, map_sub);
            break;
        case 4:
            load_cmap4(ttcmap->map, GIDToCIDMap, num_glyphs,
                        gsub_vert, gsub_list,
                        cmap, map_base, map_sub);
            break;
        }
        if (gsub_vert)
            otl_gsub_release(gsub_vert);
        if (gsub_list)
            otl_gsub_release(gsub_list);
        tt_cmap_release(ttcmap);

        if (otl_tags) {
            CMap *tounicode = NULL;
            char *tounicode_name;
            int   tounicode_id;

            tounicode_name = NEW(strlen(map_name)+strlen(",000-UCS32-Add")+1, char);
            sprintf(tounicode_name, "%s,%03d-UCS32-Add", map_name, ttc_index);
            tounicode_id = CMap_cache_find(tounicode_name);

            if (tounicode_id >= 0) {
                tounicode = CMap_cache_get(tounicode_id);
            } else {
                tounicode = CMap_new();
                CMap_set_name (tounicode, tounicode_name);
                CMap_set_type (tounicode, CMAP_TYPE_TO_UNICODE);
                CMap_set_wmode(tounicode, 0);
                CMap_add_codespacerange(tounicode, srange_min, srange_max, 2);
                CMap_set_CIDSysInfo(tounicode, &CSI_UNICODE);
                CMap_add_bfchar(tounicode, srange_min, 2, srange_max, 2);
                tounicode_id = CMap_cache_add(tounicode);
            }

            free(tounicode_name);

            for (gid = 0; gid < num_glyphs; gid++) {
                uint16_t      cid = GIDToCIDMap[gid];
                unsigned char src[2], dst[4];
                if (cid > 0) {
                    int32_t ch = UC_is_valid(map_base[gid]) ? map_base[gid] : map_sub[gid];
                    if (UC_is_valid(ch)) {
                        unsigned char *p      = dst;
                        unsigned char *endptr = dst + 4;
                        size_t         len;
                        src[0] = (cid >> 8) & 0xff;
                        src[1] =  cid & 0xff;
                        len = UC_UTF16BE_encode_char(ch, &p, endptr);
                        if (len > 0) {
                            CMap_add_bfchar(tounicode, src, 2, dst, len);
                        }
                    }
                }
            }
        }

        cmap_id = CMap_cache_add(cmap);
    }

    free(cmap_name);
    free(GIDToCIDMap);
    free(csi.registry);
    free(csi.ordering);
    sfnt_close(sfont);
    ttstub_input_close(handle);

    return cmap_id;
}

int
otf_try_load_GID_to_CID_map (const char *map_name, int ttc_index, int wmode)
{
  int         cmap_id     = -1;
  sfnt       *sfont       = NULL;
  ULONG       offset      = 0;
  char       *cmap_name   = NULL;
  rust_input_handle_t handle = NULL;

  if (!map_name)
    return -1;

  if (ttc_index > 999 || ttc_index < 0) {
    return -1; /* Sorry for this... */
  }

  /* Check if already loaded */
  cmap_name = NEW(strlen(map_name)+strlen("-GID")+5, char);
  sprintf(cmap_name, "%s:%3d-GID", map_name, ttc_index);
  cmap_id = CMap_cache_find(cmap_name);
  if (cmap_id >= 0) {
    free(cmap_name);
    if (dpx_conf.verbose_level > VERBOSE_LEVEL_MIN)
      dpx_message("otf_cmap>> GID-to-CID mapping found at cmap_id=%d.\n", cmap_id);

    return cmap_id;
  }

  handle = dpx_open_truetype_file(map_name);
  if (handle == NULL)
      handle = dpx_open_opentype_file(map_name);
  if (handle == NULL) {
      handle = dpx_open_dfont_file(map_name);
      if (handle == NULL) {
          free(cmap_name);
          return -1;
      }
      sfont = dfont_open(handle, ttc_index);
  } else {
      sfont = sfnt_open(handle);
  }

  if (!sfont) {
    dpx_warning("Could not open OpenType/TrueType/dfont font file \"%s\"", map_name);
    free(cmap_name);
    ttstub_input_close(handle);
    return -1;
  }
  switch (sfont->type) {
  case SFNT_TYPE_TTC:
    offset = ttc_read_offset(sfont, ttc_index);
    if (offset == 0) {
      dpx_warning("Invalid TTC index for font \"%s\": %d", map_name, ttc_index);
      sfnt_close(sfont);
      ttstub_input_close(handle);
      free(cmap_name);
      return -1;
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
    dpx_warning("Not a OpenType/TrueType/TTC font?: %s", map_name);
    sfnt_close(sfont);
    ttstub_input_close(handle);
    free(cmap_name);
    return -1;
  }

  if (sfnt_read_table_directory(sfont, offset) < 0) {
    dpx_warning("Could not read OpenType/TrueType table directory: %s", map_name);
    sfnt_close(sfont);
    ttstub_input_close(handle);
    free(cmap_name);
    return -1;
  }
  if (sfont->type != SFNT_TYPE_POSTSCRIPT) {
    free(cmap_name);
    sfnt_close(sfont);
    ttstub_input_close(handle);
    return -1;
  }

  /* Read GID-to-CID mapping if CFF OpenType is found. */
  if (sfont->type == SFNT_TYPE_POSTSCRIPT) {
    cff_font             *cffont;
    struct tt_maxp_table *maxp;
    const unsigned char   csrange[4]  = {0x00, 0x00, 0xff, 0xff};
    uint16_t              num_glyphs  = 0;

    maxp       = tt_read_maxp_table(sfont);
    num_glyphs = (card16) maxp->numGlyphs;
    free(maxp);

    offset = sfnt_find_table_pos(sfont, "CFF ");
    cffont = cff_open(sfont->handle, offset, 0);
    if (cffont && cffont->flag & FONTTYPE_CIDFONT) {
      CMap       *cmap;
     uint16_t    gid;
      uint16_t   *GIDToCIDMap = NULL;
      CIDSysInfo  csi         = {NULL, NULL, 0};

      if (!cff_dict_known(cffont->topdict, "ROS")) {
        csi.registry   = strdup("Adobe");
        csi.ordering   = strdup("Identity");
        csi.supplement = 0;
      } else {
        card16 reg, ord;

        reg = (card16) cff_dict_get(cffont->topdict, "ROS", 0);
        ord = (card16) cff_dict_get(cffont->topdict, "ROS", 1);
        csi.registry   = cff_get_string(cffont, reg);
        csi.ordering   = cff_get_string(cffont, ord);
        csi.supplement = (int) cff_dict_get(cffont->topdict, "ROS", 2);
      }
      cff_read_charsets(cffont);
      GIDToCIDMap = NEW(num_glyphs, uint16_t);
      memset(GIDToCIDMap, 0, num_glyphs*sizeof(uint16_t));
      create_GIDToCIDMap(GIDToCIDMap, num_glyphs, cffont);
      cmap = CMap_new();
      CMap_set_name (cmap, cmap_name);
      CMap_set_type (cmap, CMAP_TYPE_CODE_TO_CID);
      CMap_set_wmode(cmap, wmode);
      CMap_add_codespacerange(cmap, &csrange[0], &csrange[2], 2);
      CMap_set_CIDSysInfo(cmap, &csi);
      for (gid = 0; gid < num_glyphs; gid++) {
        unsigned char src[2], dst[2];
        src[0] = (gid >> 8) & 0xff;
        src[1] = gid & 0xff;
        dst[0] = (GIDToCIDMap[gid] >> 8) & 0xff;
        dst[1] =  GIDToCIDMap[gid] & 0xff;
        CMap_add_bfchar(cmap, src, 2, dst, 2);
      }
      cmap_id = CMap_cache_add(cmap);
      if (dpx_conf.verbose_level > VERBOSE_LEVEL_MIN) {
        dpx_message("\n");
        dpx_message("otf_cmap>> Creating GID-to-CID mapping for font=\"%s\"\n", map_name);
      }
      free(GIDToCIDMap);
      if (csi.registry)
        free(csi.registry);
      if (csi.ordering)
        free(csi.ordering);
    }
    if (cffont)
      cff_close(cffont);
  }

  free(cmap_name);
  sfnt_close(sfont);
  ttstub_input_close(handle);

  return cmap_id;
}
