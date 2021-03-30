/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include "dpx-tfm.h"

#include <fcntl.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"

#define TFM_FORMAT 1
#define OFM_FORMAT 2

#define FWBASE ((double) (1<<20))

#define CHARACTER_INDEX(i)  ((i))

/*
 * TFM Record structure:
 * Multiple TFM's may be read in at once.
 */

struct tfm_font
{
    int32_t   level;
    uint32_t wlenfile;
    uint32_t wlenheader;
    uint32_t bc, ec;
    uint32_t nwidths, nheights, ndepths;
    uint32_t nitcor, nlig, nkern, nextens;
    uint32_t nfonparm;
    uint32_t fontdir;
    uint32_t nco, ncw, npc;
    fixword       *header;
    uint32_t      *char_info;
    unsigned short *width_index;
    unsigned char *height_index;
    unsigned char *depth_index;
    fixword       *width;
    fixword       *height;
    fixword       *depth;
};

static void
tfm_font_init (struct tfm_font *tfm)
{
    tfm->header = NULL;
    tfm->level   = 0;
    tfm->fontdir = 0;
    tfm->nco = tfm->ncw = tfm->npc = 0;
    tfm->char_info    = NULL;
    tfm->width_index  = NULL;
    tfm->height_index = NULL;
    tfm->depth_index  = NULL;
    tfm->width = tfm->height = tfm->depth = NULL;
}

static void
tfm_font_clear (struct tfm_font *tfm)
{
    if (tfm) {
        tfm->header = mfree(tfm->header);
        tfm->char_info = mfree(tfm->char_info);
        tfm->width = mfree(tfm->width);
        tfm->height = mfree(tfm->height);
        tfm->depth = mfree(tfm->depth);
        tfm->width_index = mfree(tfm->width_index);
        tfm->height_index = mfree(tfm->height_index);
        tfm->depth_index = mfree(tfm->depth_index);
    }
}


struct coverage
{
    int           first_char;
    int           num_chars;
};

/*
 * All characters in the same range have same metrics.
 */

struct range_map {
    unsigned short   num_coverages;
    struct coverage *coverages;
    unsigned short  *indices;
};

/* Special case of num_coverages = 1 */
struct char_map
{
    struct coverage coverage;
    unsigned short *indices;
};

static void
release_char_map (struct char_map *map)
{
    map->indices = mfree(map->indices);
    free(map);
}

static void
release_range_map (struct range_map *map)
{
    free(map->coverages);
    free(map->indices);
    map->coverages = NULL;
    map->indices   = NULL;
    free(map);
}

static int
lookup_char (const struct char_map *map, int charcode)
{
    if (charcode >= map->coverage.first_char &&
        charcode <= map->coverage.first_char + map->coverage.num_chars)
        return map->indices[CHARACTER_INDEX(charcode - map->coverage.first_char)];
    else
        return -1;
}

static int
lookup_range (const struct range_map *map, int charcode)
{
    int  idx;

    for (idx = map->num_coverages - 1; idx >= 0 &&
             charcode >= map->coverages[idx].first_char; idx--) {
        if (charcode <=
            map->coverages[idx].first_char + map->coverages[idx].num_chars)
            return map->indices[CHARACTER_INDEX(idx)];
    }

    return -1;
}

#define SOURCE_TYPE_TFM 0
#define SOURCE_TYPE_JFM 1
#define SOURCE_TYPE_OFM 2

#define MAPTYPE_NONE  0
#define MAPTYPE_CHAR  1
#define MAPTYPE_RANGE 2

#define FONT_DIR_HORIZ 0
#define FONT_DIR_VERT  1

struct font_metric
{
    char    *tex_name;
    fixword  designsize;
    char    *codingscheme;

    int  fontdir;
    int firstchar, lastchar;

    fixword *widths;
    fixword *heights;
    fixword *depths;

    struct {
        int   type;
        void *data;
    } charmap;

    int source;
};

static void
fm_init (struct font_metric *fm)
{
    fm->tex_name = NULL;
    fm->firstchar = 0;
    fm->lastchar  = 0;
    fm->fontdir   = FONT_DIR_HORIZ;
    fm->codingscheme = NULL;
    fm->designsize   = 0;

    fm->widths  = NULL;
    fm->heights = NULL;
    fm->depths  = NULL;

    fm->charmap.type = MAPTYPE_NONE;
    fm->charmap.data = NULL;

    fm->source = SOURCE_TYPE_TFM;
}

static void
fm_clear (struct font_metric *fm)
{
    if (fm) {
        free(fm->tex_name);
        free(fm->widths);
        free(fm->heights);
        free(fm->depths);
        free(fm->codingscheme);

        switch (fm->charmap.type) {
        case MAPTYPE_CHAR:
            release_char_map(fm->charmap.data);
            break;
        case MAPTYPE_RANGE:
            release_range_map(fm->charmap.data);
            break;
        }
    }
}

#ifndef MAX_FONTS
#define MAX_FONTS 16
#endif

static struct font_metric *fms = NULL;
static unsigned int numfms = 0, max_fms = 0;

void
tfm_reset_global_state(void)
{
    fms = NULL;
    numfms = 0, max_fms = 0;
}

static void
fms_need (unsigned n)
{
    if (n > max_fms) {
        max_fms = MAX(max_fms + MAX_FONTS, n);
        fms = RENEW(fms, max_fms, struct font_metric);
    }
}

static int
fread_fwords (fixword *words, uint32_t nmemb, rust_input_handle_t handle)
{
    uint32_t i;

    for (i = 0; i < nmemb; i++)
        words[i] = tt_get_signed_quad(handle);

    return nmemb * 4;
}


static int
fread_uquads (uint32_t *quads, uint32_t nmemb, rust_input_handle_t handle)
{
    uint32_t i;

    for (i = 0; i < nmemb; i++)
        quads[i] = tt_get_unsigned_quad(handle);

    return nmemb * 4;
}


/*
 * TFM and JFM
 */
static void
tfm_check_size (struct tfm_font *tfm, off_t tfm_file_size)
{
    uint32_t expected_size = 6;

    /* Removed the warning message caused by EC TFM metric files.
     *
     if (tfm->wlenfile != tfm_file_size / 4) {
     dpx_warning("TFM file size is %ld bytes but it says it is %ld bytes!",
     tfm_file_size, tfm->wlenfile * 4);
     if (tfm_file_size > tfm->wlenfile * 4) {
     dpx_warning("Proceeding nervously...");
     } else {
     _tt_abort("Can't proceed...");
     }
     }
    */
    if ((int64_t)tfm_file_size < (int64_t)tfm->wlenfile * 4) {
        _tt_abort("Can't proceed...");
    }

    expected_size += (tfm->ec - tfm->bc + 1);
    expected_size += tfm->wlenheader;
    expected_size += tfm->nwidths;
    expected_size += tfm->nheights;
    expected_size += tfm->ndepths;
    expected_size += tfm->nitcor;
    expected_size += tfm->nlig;
    expected_size += tfm->nkern;
    expected_size += tfm->nextens;
    expected_size += tfm->nfonparm;

    if (expected_size != tfm->wlenfile) {
        dpx_warning("TFM file size is expected to be %" PRId64 " bytes but it says it is %" PRId64 "bytes!",
                    (int64_t)expected_size * 4, (int64_t)tfm->wlenfile * 4);
        if ((int64_t)tfm_file_size > (int64_t)expected_size *4) {
            dpx_warning("Proceeding nervously...");
        } else {
            _tt_abort("Can't proceed...");
        }
    }
}


static void
tfm_get_sizes (rust_input_handle_t tfm_handle, off_t tfm_file_size, struct tfm_font *tfm)
{
    tfm->wlenfile = tt_get_unsigned_pair(tfm_handle);

    tfm->wlenheader = tt_get_unsigned_pair(tfm_handle);
    tfm->bc = tt_get_unsigned_pair(tfm_handle);
    tfm->ec = tt_get_unsigned_pair(tfm_handle);
    if (tfm->ec < tfm->bc)
        _tt_abort("TFM file error: ec(%u) < bc(%u) ???", tfm->ec, tfm->bc);

    tfm->nwidths  = tt_get_unsigned_pair(tfm_handle);
    tfm->nheights = tt_get_unsigned_pair(tfm_handle);
    tfm->ndepths  = tt_get_unsigned_pair(tfm_handle);
    tfm->nitcor   = tt_get_unsigned_pair(tfm_handle);
    tfm->nlig     = tt_get_unsigned_pair(tfm_handle);
    tfm->nkern    = tt_get_unsigned_pair(tfm_handle);
    tfm->nextens  = tt_get_unsigned_pair(tfm_handle);
    tfm->nfonparm = tt_get_unsigned_pair(tfm_handle);

    tfm_check_size(tfm, tfm_file_size);

    return;
}


static void
tfm_unpack_arrays (struct font_metric *fm, struct tfm_font *tfm)
{
    uint32_t charinfo;
    unsigned short width_index;
    unsigned char  height_index, depth_index;
    uint32_t i;

    fm->widths  = NEW(256, fixword);
    fm->heights = NEW(256, fixword);
    fm->depths  = NEW(256, fixword);
    for (i = 0; i < 256; i++) {
        fm->widths [i] = 0;
        fm->heights[i] = 0;
        fm->depths [i] = 0;
    }

    for (i = tfm->bc; i <= tfm->ec; i++ ) {
        charinfo     = tfm->char_info[i - tfm->bc];
        width_index  = (charinfo >> 24);
        height_index = (charinfo >> 20) & 0xf;
        depth_index  = (charinfo >> 16) & 0xf;
        fm->widths [i] = tfm->width [width_index];
        fm->heights[i] = tfm->height[height_index];
        fm->depths [i] = tfm->depth [depth_index];
    }

    return;
}

static int
sput_bigendian (char *s, int32_t v, int n)
{
    int i;

    for (i = n-1; i >= 0; i--) {
        s[i] = (char) (v & 0xff);
        v >>= 8;
    }

    return n;
}

static void
tfm_unpack_header (struct font_metric *fm, struct tfm_font *tfm)
{
    if (tfm->wlenheader < 12) {
        fm->codingscheme = NULL;
    } else {
        int   i, len;
        char *p;

        len = (tfm->header[2] >> 24);
        if (len < 0 || len > 39)
            _tt_abort("Invalid TFM header.");
        if (len > 0) {
            fm->codingscheme = NEW(40, char);
            p = fm->codingscheme;
            p += sput_bigendian(p, tfm->header[2], 3);
            for (i = 1; i <= len / 4; i++) {
                p += sput_bigendian(p, tfm->header[2+i], 4);
            }
            fm->codingscheme[len] = '\0';
        } else {
            fm->codingscheme = NULL;
        }
    }

    fm->designsize = tfm->header[1];
}


static void
ofm_check_size_one (struct tfm_font *tfm, off_t ofm_file_size)
{
    uint32_t ofm_size = 14;

    ofm_size += 2 * (tfm->ec - tfm->bc + 1);
    ofm_size += tfm->wlenheader;
    ofm_size += tfm->nwidths;
    ofm_size += tfm->nheights;
    ofm_size += tfm->ndepths;
    ofm_size += tfm->nitcor;
    ofm_size += 2 * tfm->nlig;
    ofm_size += tfm->nkern;
    ofm_size += 2 * tfm->nextens;
    ofm_size += tfm->nfonparm;

    if (tfm->wlenfile != ofm_file_size / 4 || tfm->wlenfile != ofm_size)
        _tt_abort("OFM file problem.  Table sizes don't agree.");
}


static void
ofm_get_sizes (rust_input_handle_t ofm_handle, off_t ofm_file_size, struct tfm_font *tfm)
{
    tfm->level = tt_get_signed_quad(ofm_handle);

    tfm->wlenfile   = tt_get_positive_quad(ofm_handle, "OFM", "wlenfile");
    tfm->wlenheader = tt_get_positive_quad(ofm_handle, "OFM", "wlenheader");
    tfm->bc = tt_get_positive_quad(ofm_handle, "OFM", "bc");
    tfm->ec = tt_get_positive_quad(ofm_handle, "OFM", "ec");

    if (tfm->ec < tfm->bc)
        _tt_abort("OFM file error: ec(%u) < bc(%u) ???", tfm->ec, tfm->bc);

    tfm->nwidths  = tt_get_positive_quad(ofm_handle, "OFM", "nwidths");
    tfm->nheights = tt_get_positive_quad(ofm_handle, "OFM", "nheights");
    tfm->ndepths  = tt_get_positive_quad(ofm_handle, "OFM", "ndepths");
    tfm->nitcor   = tt_get_positive_quad(ofm_handle, "OFM", "nitcor");
    tfm->nlig     = tt_get_positive_quad(ofm_handle, "OFM", "nlig");
    tfm->nkern    = tt_get_positive_quad(ofm_handle, "OFM", "nkern");
    tfm->nextens  = tt_get_positive_quad(ofm_handle, "OFM", "nextens");
    tfm->nfonparm = tt_get_positive_quad(ofm_handle, "OFM", "nfonparm");
    tfm->fontdir  = tt_get_positive_quad(ofm_handle, "OFM", "fontdir");

    if (tfm->fontdir)
        dpx_warning("I may be interpreting a font direction incorrectly.");

    if (tfm->level == 0) {
        ofm_check_size_one(tfm, ofm_file_size);
    } else if (tfm->level == 1) {
        tfm->nco = tt_get_positive_quad(ofm_handle, "OFM", "nco");
        tfm->ncw = tt_get_positive_quad(ofm_handle, "OFM", "nco");
        tfm->npc = tt_get_positive_quad(ofm_handle, "OFM", "npc");
        ttstub_input_seek(ofm_handle, 4 * (off_t)(tfm->nco - tfm->wlenheader), SEEK_SET);
    } else {
        _tt_abort("can't handle OFM files with level > 1");
    }
}


static void
ofm_do_char_info_zero (rust_input_handle_t ofm_handle, struct tfm_font *tfm)
{
    uint32_t num_chars;

    num_chars = tfm->ec - tfm->bc + 1;

    if (num_chars != 0) {
        uint32_t i;

        tfm->width_index  = NEW(num_chars, unsigned short);
        tfm->height_index = NEW(num_chars, unsigned char);
        tfm->depth_index  = NEW(num_chars, unsigned char);

        for (i = 0; i < num_chars; i++) {
            tfm->width_index [i] = tt_get_unsigned_pair(ofm_handle);
            tfm->height_index[i] = tt_get_unsigned_byte(ofm_handle);
            tfm->depth_index [i] = tt_get_unsigned_byte(ofm_handle);
            /* Ignore remaining quad */
            tt_skip_bytes(4, ofm_handle);
        }
    }
}


static void
ofm_do_char_info_one (rust_input_handle_t ofm_handle, struct tfm_font *tfm)
{
    uint32_t num_char_infos;
    uint32_t num_chars;

    num_char_infos = tfm->ncw / (3 + (tfm->npc / 2));
    num_chars      = tfm->ec - tfm ->bc + 1;

    if (num_chars != 0) {
        uint32_t i;
        uint32_t char_infos_read;

        tfm->width_index  = NEW(num_chars, unsigned short);
        tfm->height_index = NEW(num_chars, unsigned char);
        tfm->depth_index  = NEW(num_chars, unsigned char);
        char_infos_read   = 0;

        for (i = 0; i < num_chars && char_infos_read < num_char_infos; i++) {
            uint32_t repeats, j;

            tfm->width_index [i] = tt_get_unsigned_pair(ofm_handle);
            tfm->height_index[i] = tt_get_unsigned_byte(ofm_handle);
            tfm->depth_index [i] = tt_get_unsigned_byte(ofm_handle);

            /* Ignore next quad */
            tt_skip_bytes(4, ofm_handle);
            repeats = tt_get_unsigned_pair(ofm_handle);

            /* Skip params */
            for (j = 0; j < tfm->npc; j++)
                tt_get_unsigned_pair(ofm_handle);

            /* Remove word padding if necessary */
            if (ISEVEN(tfm->npc))
                tt_get_unsigned_pair(ofm_handle);

            char_infos_read++;
            if (i + repeats > num_chars)
                _tt_abort("OFM \"repeats\" causes number of characters to be exceeded.");

            for (j = 0; j < repeats; j++) {
                tfm->width_index [i+j+1] = tfm->width_index [i];
                tfm->height_index[i+j+1] = tfm->height_index[i];
                tfm->depth_index [i+j+1] = tfm->depth_index [i];
            }

            /* Skip ahead because we have already handled repeats */
            i += repeats;
        }
    }
}


static void
ofm_unpack_arrays (struct font_metric *fm, struct tfm_font *tfm, uint32_t num_chars)
{
    uint32_t i;

    fm->widths  = NEW(tfm->bc + num_chars, fixword);
    fm->heights = NEW(tfm->bc + num_chars, fixword);
    fm->depths  = NEW(tfm->bc + num_chars, fixword);

    for (i = 0; i < num_chars; i++) {
        fm->widths [tfm->bc + i] = tfm->width [ tfm->width_index [i] ];
        fm->heights[tfm->bc + i] = tfm->height[ tfm->height_index[i] ];
        fm->depths [tfm->bc + i] = tfm->depth [ tfm->depth_index [i] ];
    }
}


static void
read_ofm (struct font_metric *fm, rust_input_handle_t ofm_handle, off_t ofm_file_size)
{
    struct tfm_font tfm;

    tfm_font_init(&tfm);

    ofm_get_sizes(ofm_handle, ofm_file_size, &tfm);

    if (tfm.level < 0 || tfm.level > 1)
        _tt_abort("OFM level %d not supported.", tfm.level);

    if (tfm.wlenheader > 0) {
        tfm.header = NEW(tfm.wlenheader, fixword);
        fread_fwords(tfm.header, tfm.wlenheader, ofm_handle);
    }

    if (tfm.level == 0)
        ofm_do_char_info_zero(ofm_handle, &tfm);
    else if (tfm.level == 1)
        ofm_do_char_info_one(ofm_handle, &tfm);

    if (tfm.nwidths > 0) {
        tfm.width = NEW(tfm.nwidths, fixword);
        fread_fwords(tfm.width, tfm.nwidths, ofm_handle);
    }

    if (tfm.nheights > 0) {
        tfm.height = NEW(tfm.nheights, fixword);
        fread_fwords(tfm.height, tfm.nheights, ofm_handle);
    }

    if (tfm.ndepths > 0) {
        tfm.depth = NEW(tfm.ndepths, fixword);
        fread_fwords(tfm.depth, tfm.ndepths, ofm_handle);
    }

    ofm_unpack_arrays(fm, &tfm, tfm.ec - tfm.bc + 1);
    tfm_unpack_header(fm, &tfm);
    fm->firstchar = tfm.bc;
    fm->lastchar  = tfm.ec;
    fm->source    = SOURCE_TYPE_OFM;

    tfm_font_clear(&tfm);
}


static void
read_tfm (struct font_metric *fm, rust_input_handle_t tfm_handle, off_t tfm_file_size)
{
    struct tfm_font tfm;

    tfm_font_init(&tfm);

    tfm_get_sizes(tfm_handle, tfm_file_size, &tfm);

    fm->firstchar = tfm.bc;
    fm->lastchar  = tfm.ec;

    if (tfm.wlenheader > 0) {
        tfm.header = NEW(tfm.wlenheader, fixword);
        fread_fwords(tfm.header, tfm.wlenheader, tfm_handle);
    }

    if (tfm.ec - tfm.bc + 1 > 0) {
        tfm.char_info = NEW(tfm.ec - tfm.bc + 1, uint32_t);
        fread_uquads(tfm.char_info, tfm.ec - tfm.bc + 1, tfm_handle);
    }

    if (tfm.nwidths > 0) {
        tfm.width = NEW(tfm.nwidths, fixword);
        fread_fwords(tfm.width, tfm.nwidths, tfm_handle);
    }

    if (tfm.nheights > 0) {
        tfm.height = NEW(tfm.nheights, fixword);
        fread_fwords(tfm.height, tfm.nheights, tfm_handle);
    }

    if (tfm.ndepths > 0) {
        tfm.depth = NEW(tfm.ndepths, fixword);
        fread_fwords(tfm.depth, tfm.ndepths, tfm_handle);
    }

    tfm_unpack_arrays(fm, &tfm);
    tfm_unpack_header(fm, &tfm);
    tfm_font_clear(&tfm);

    return;
}

int
tfm_open (const char *tfm_name, int must_exist)
{
    rust_input_handle_t tfm_handle = NULL;
    int i, format = TFM_FORMAT;
    off_t tfm_file_size;
    char *ofm_name, *suffix;

    for (i = 0; i < numfms; i++) {
        if (streq_ptr(tfm_name, fms[i].tex_name))
            return i;
    }

    /* NOTE: the following comment is no longer operative with the switch to
     * the Tectonic I/O system since we don't have `must_exist`. The logic
     * of the current implementation might not be right; to be investigated.
     * Comment preserved for posterity.
     *
     * "The procedure to search tfm or ofm files:
     * 1. Search tfm file with the given name with the must_exist flag unset.
     * 2. Search ofm file with the given name with the must_exist flag unset.
     * 3. If not found and must_exist flag is set, try again to search
     *    tfm file with the must_exist flag set.
     * 4. If not found and must_exist flag is not set, return -1.
     *
     * We first look for OFM and then TFM.
     * The reason for this change is incompatibility introduced when dvipdfmx
     * started to write correct glyph metrics to output PDF for CID fonts.
     * I'll not explain this in detail... This change is mostly specific to
     * Japanese support."
     */

    suffix = strrchr(tfm_name, '.');

    if (!suffix || (strcmp(suffix, ".tfm") != 0 && strcmp(suffix, ".ofm") != 0)) {
        ofm_name = NEW(strlen(tfm_name) + strlen(".ofm") + 1, char);
        strcpy(ofm_name, tfm_name);
        strcat(ofm_name, ".ofm");
    } else {
        ofm_name = NULL;
    }

    if (ofm_name && (tfm_handle = ttstub_input_open(ofm_name, TTBC_FILE_FORMAT_OFM, 0)) != NULL) {
        format = OFM_FORMAT;
    } else if ((tfm_handle = ttstub_input_open(tfm_name, TTBC_FILE_FORMAT_TFM, 0)) != NULL) {
        format = TFM_FORMAT;
    } else if ((tfm_handle = ttstub_input_open(tfm_name, TTBC_FILE_FORMAT_OFM, 0)) != NULL) {
        format = OFM_FORMAT;
    }

    free(ofm_name);

    if (tfm_handle == NULL) {
        if (must_exist)
            _tt_abort("Unable to find TFM file \"%s\".", tfm_name);
        return -1;
    }

    if (dpx_conf.verbose_level > 0) {
        if (format == TFM_FORMAT)
            dpx_message("(TFM:%s", tfm_name);
        else if (format == OFM_FORMAT)
            dpx_message("(OFM:%s", tfm_name);
    }

    tfm_file_size = ttstub_input_get_size (tfm_handle);
    if (tfm_file_size > 0x1FFFFFFFF)
        _tt_abort("TFM/OFM file size exceeds 33-bit");
    if (tfm_file_size < 24)
        _tt_abort("TFM/OFM file too small to be a valid file.");

    fms_need(numfms + 1);
    fm_init(fms + numfms);

    if (format == OFM_FORMAT)
        read_ofm(&fms[numfms], tfm_handle, tfm_file_size);
    else
        read_tfm(&fms[numfms], tfm_handle, tfm_file_size);

    ttstub_input_close(tfm_handle);

    fms[numfms].tex_name = NEW(strlen(tfm_name)+1, char);
    strcpy(fms[numfms].tex_name, tfm_name);

    if (dpx_conf.verbose_level > 0)
        dpx_message(")");

    return numfms++;
}


void
tfm_close_all (void)
{
    unsigned int i;

    if (fms) {
        for (i = 0; i < numfms; i++) {
            fm_clear(&(fms[i]));
        }
        free(fms);
    }
}

#define CHECK_ID(n) do {                                \
        if ((n) < 0 || (n) >= numfms)                   \
            _tt_abort("TFM: Invalid TFM ID: %d", (n));  \
    } while (0)

fixword
tfm_get_fw_width (int font_id, int32_t ch)
{
    struct font_metric *fm;
    int idx = 0;

    CHECK_ID(font_id);

    fm = &(fms[font_id]);
    if (ch >= fm->firstchar && ch <= fm->lastchar) {
        switch (fm->charmap.type) {
        case MAPTYPE_CHAR:
            idx = lookup_char(fm->charmap.data, ch);
            if (idx < 0)
                _tt_abort("Invalid char: %" PRId32 "\n", ch);
            break;
        case MAPTYPE_RANGE:
            idx = lookup_range(fm->charmap.data, ch);
            if (idx < 0)
                _tt_abort("Invalid char: %" PRId32 "\n", ch);
            break;
        default:
            idx = ch;
        }
    } else {
        _tt_abort("Invalid char: %" PRId32 "\n", ch);
    }

    return fm->widths[idx];
}

fixword
tfm_get_fw_height (int font_id, int32_t ch)
{
    struct font_metric *fm;
    int idx = 0;

    CHECK_ID(font_id);

    fm = &(fms[font_id]);
    if (ch >= fm->firstchar && ch <= fm->lastchar) {
        switch (fm->charmap.type) {
        case MAPTYPE_CHAR:
            idx = lookup_char(fm->charmap.data, ch);
            if (idx < 0)
                _tt_abort("Invalid char: %" PRId32 "\n", ch);
            break;
        case MAPTYPE_RANGE:
            idx = lookup_range(fm->charmap.data, ch);
            if (idx < 0)
                _tt_abort("Invalid char: %" PRId32 "\n", ch);
            break;
        default:
            idx = ch;
        }
    } else {
        _tt_abort("Invalid char: %" PRId32 "\n", ch);
    }

    return fm->heights[idx];
}

fixword
tfm_get_fw_depth (int font_id, int32_t ch)
{
    struct font_metric *fm;
    int idx = 0;

    CHECK_ID(font_id);

    fm = &(fms[font_id]);
    if (ch >= fm->firstchar && ch <= fm->lastchar) {
        switch (fm->charmap.type) {
        case MAPTYPE_CHAR:
            idx = lookup_char(fm->charmap.data, ch);
            if (idx < 0)
                _tt_abort("Invalid char: %" PRId32 "\n", ch);
            break;
        case MAPTYPE_RANGE:
            idx = lookup_range(fm->charmap.data, ch);
            if (idx < 0)
                _tt_abort("Invalid char: %" PRId32 "\n", ch);
            break;
        default:
            idx = ch;
        }
    } else {
        _tt_abort("Invalid char: %" PRId32 "\n", ch);
    }

    return fm->depths[idx];
}


/*
 * tfm_get_width returns the width of the font
 * as a (double) fraction of the design size.
 */
double
tfm_get_width (int font_id, int32_t ch)
{
    return ((double) tfm_get_fw_width(font_id, ch)/FWBASE);
}


/* tfm_string_xxx() do not work for OFM... */
fixword
tfm_string_width (int font_id, const unsigned char *s, unsigned len)
{
    fixword result = 0;
    unsigned int i;

    CHECK_ID(font_id);

    for (i = 0; i < len; i++) {
        result += tfm_get_fw_width(font_id, s[i]);
    }

    return result;
}


double
tfm_get_design_size (int font_id)
{
    CHECK_ID(font_id);

    return (double) (fms[font_id].designsize)/FWBASE*(72.0/72.27);
}


bool
tfm_exists (const char *tfm_name)
{
    rust_input_handle_t handle;

    handle = ttstub_input_open(tfm_name, TTBC_FILE_FORMAT_OFM, 0);
    if (handle) {
        ttstub_input_close(handle);
        return true;
    }

    handle = ttstub_input_open(tfm_name, TTBC_FILE_FORMAT_TFM, 0);
    if (handle) {
        ttstub_input_close(handle);
        return true;
    }

    return false;
}
