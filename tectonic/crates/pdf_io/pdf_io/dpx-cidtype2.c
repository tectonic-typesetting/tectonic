/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
 * TrueType glyf table is sorted by CID and no CIDToGIDMap is used here.
 * GhostScript can't handle CIDToGIDMap correctly.
 */

#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
/* pseudo unique tag */
#include "dpx-pdffont.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"

#ifndef PDF_NAME_LEN_MAX
#  define PDF_NAME_LEN_MAX 255
#endif

#include "dpx-cidtype2.h"

#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dpx-cid.h"
/* CID font */
#include "dpx-cmap.h"
/* TrueType */
#include "dpx-sfnt.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_cmap.h"
#include "dpx-tt_glyf.h"
#include "dpx-tt_gsub.h"
#include "dpx-tt_table.h"
#include "dpx-type0.h"
#include "dpx-unicode.h"

/*
 * PDF viewer applications use following tables (CIDFontType 2)
 *
 *  head, hhea, loca, maxp, glyf, hmtx, fpgm, cvt_, prep
 *
 *                                         - from PDF Ref. v.1.3, 2nd ed.
 *
 * The fpgm, cvt_, and prep tables appears only when TrueType instructions
 * requires them. Those tables must be preserved if they exist.
 * We use must_exist flag to indicate `preserve it if present'
 * and to make sure not to cause an error when it does not exist.
 *
 * post and name table must exist in ordinary TrueType font file,
 * but when a TrueType font is converted to CIDFontType 2 font, those tables
 * are no longer required.
 *
 * The OS/2 table (required for TrueType font for Windows and OS/2) contains
 * liscencing information, but PDF viewers seems not using them.
 *
 * The 'name' table added. See comments in ttf.c.
 */

static struct
{
    const char *name;
    int         must_exist;
} required_table[] = {
    {"OS/2", 0}, {"head", 1}, {"hhea", 1}, {"loca", 1}, {"maxp", 1},
    {"name", 1}, {"glyf", 1}, {"hmtx", 1}, {"fpgm", 0}, {"cvt ", 0},
    {"prep", 0}, {NULL, 0}
};

static int
validate_name (char *fontname, int len)
{
    int    i, count;
    char  *p;
    static const char *badstrlist[] = {
        "-WIN-RKSJ-H",
        "-WINP-RKSJ-H",
        "-WING-RKSJ-H",
        "-90pv-RKSJ-H",
        NULL
    };

    for (count = 0, i = 0; i < len; i++) {
        if (fontname[i] == 0) {
            memmove(fontname + i, fontname + i + 1, len - i);
            count++;
            len--;
        }
    }
    if (count > 0) {
        dpx_warning("Removed %d null character(s) from fontname --> %s",
                    count, fontname);
    }
    fontname[len] = '\0';

    /* For some fonts that have bad PS name. ad hoc. remove me. */
    for (i = 0; badstrlist[i] != NULL; i++) {
        p = strstr(fontname, badstrlist[i]);
        if (p && p > fontname) {
            dpx_warning("Removed string \"%s\" from fontname \"%s\".",
                        badstrlist[i], fontname);
            p[0] = '\0';
            len  = (int) (p - fontname);
            break;
        }
    }

    if (len < 1) {
        dpx_warning("No valid character found in fontname string.");
        return -1;
    }

    return 0;
}

/*
 * We will follow the convension for finding ToUnicode CMap described in PDF
 * Reference 4th ed., page 432. The name of "ToCode" (not limited to Unicode
 * here) CMap is obtained by concatenating the registry, ordering, and the
 * name of encoding.
 *
 * UCSms-UCS4, UCSms-UCS2, UCS4 added...
 */

#define WIN_UCS_INDEX_MAX   1
#define KNOWN_ENCODINGS_MAX 9
static struct
{
    unsigned short  platform;
    unsigned short  encoding;
    const char     *pdfnames[5];
} known_encodings[] = {
    {TT_WIN, TT_WIN_UCS4,     {"UCSms-UCS4", "UCSms-UCS2", "UCS4", "UCS2", NULL}},
    {TT_WIN, TT_WIN_UNICODE,  {"UCSms-UCS4", "UCSms-UCS2", "UCS4", "UCS2", NULL}},
    {TT_WIN, TT_WIN_SJIS,     {"90ms-RKSJ", NULL}},
    {TT_WIN, TT_WIN_RPC,      {"GBK-EUC",   NULL}},
    {TT_WIN, TT_WIN_BIG5,     {"ETen-B5",   NULL}},
    {TT_WIN, TT_WIN_WANSUNG,  {"KSCms-UHC", NULL}},
    {TT_MAC, TT_MAC_JAPANESE, {"90pv-RKSJ", NULL}},
    {TT_MAC, TT_MAC_TRADITIONAL_CHINESE, {"B5pc",     NULL}},
    {TT_MAC, TT_MAC_SIMPLIFIED_CHINESE,  {"GBpc-EUC", NULL}},
    {TT_MAC, TT_MAC_KOREAN,   {"KSCpc-EUC", NULL}},
    {0, 0, {NULL}}
};

static CMap *
find_tocode_cmap (const char *reg, const char *ord, int select)
{
    int   cmap_id = -1, i;
    char *cmap_name;
    const char *append;

    if (!reg || !ord ||
        select < 0 || select > KNOWN_ENCODINGS_MAX) {
        dpx_warning("Character set unknown.");
        return NULL;
    }

    if (streq_ptr(ord, "UCS") &&
        select <= WIN_UCS_INDEX_MAX)
        return NULL;

    for (i = 0; cmap_id < 0 && i < 5; i++) {
        append = known_encodings[select].pdfnames[i];
        if (!append)
            break;
        cmap_name = NEW(strlen(reg) + strlen(ord) + strlen(append) + 3, char);
        sprintf(cmap_name, "%s-%s-%s", reg, ord, append);
        cmap_id = CMap_cache_find(cmap_name);
        free(cmap_name);
    }
    if (cmap_id < 0) {
        dpx_warning("Could not find CID-to-Code mapping for \"%s-%s\".", reg, ord);
        dpx_warning("I tried to load (one of) the following file(s):");
        for (i = 0; i < 5; i++) {
            append = known_encodings[select].pdfnames[i];
            if (!append)
                break;
            dpx_message(" %s-%s-%s", reg, ord, append);
        }
        dpx_warning("Please check if this file exists.");
        return NULL;
    }

    return CMap_cache_get(cmap_id);
}


/*
 * CIDFont glyph metrics:
 * Mostly same as add_CID[HV]Metrics in cidtype0.c.
 */
#define PDFUNIT(v) ((double) (ROUND(1000.0*(v)/(g->emsize), 1)))

static void
add_TTCIDHMetrics (pdf_obj *fontdict, struct tt_glyphs *g,
                   char *used_chars, unsigned char *cidtogidmap, unsigned short last_cid)
{
    int cid, start = 0, prev = 0;
    pdf_obj *w_array, *an_array = NULL;
    double   dw;
    int      empty = 1;

    w_array = pdf_new_array();
    if (g->dw != 0 && g->dw <= g->emsize) {
        dw = PDFUNIT(g->dw);
    } else {
        dw = PDFUNIT(g->gd[0].advw);
    }
    for (cid = 0; cid <= last_cid; cid++) {
        USHORT idx, gid;
        double width;

        if (!is_used_char2(used_chars, cid))
            continue;
        gid = (cidtogidmap) ? ((cidtogidmap[2*cid] << 8)|cidtogidmap[2*cid+1]) : cid;
        idx = tt_get_index(g, gid);
        if (cid != 0 && idx == 0)
            continue;
        width = PDFUNIT((g->gd)[idx].advw);
        if (width == dw) {
            if (an_array) {
                pdf_add_array(w_array, pdf_new_number(start));
                pdf_add_array(w_array, an_array);
                an_array = NULL;
                empty = 0;
            }
        } else {
            if (cid != prev + 1) {
                if (an_array) {
                    pdf_add_array(w_array, pdf_new_number(start));
                    pdf_add_array(w_array, an_array);
                    an_array = NULL;
                    empty = 0;
                }
            }
            if (an_array == NULL) {
                an_array = pdf_new_array();
                start = cid;
            }
            pdf_add_array(an_array, pdf_new_number(width));
            prev = cid;
        }
    }

    if (an_array) {
        pdf_add_array(w_array, pdf_new_number(start));
        pdf_add_array(w_array, an_array);
        empty = 0;
    }

    pdf_add_dict(fontdict,
                 pdf_new_name("DW"),
                 pdf_new_number(dw));
    if (!empty) {
        pdf_add_dict(fontdict,
                     pdf_new_name("W"),
                     pdf_ref_obj(w_array));
    }
    pdf_release_obj(w_array);

    return;
}

static void
add_TTCIDVMetrics (pdf_obj *fontdict, struct tt_glyphs *g,
                   char *used_chars, unsigned short last_cid)
{
    pdf_obj *w2_array, *an_array = NULL;
    int    cid;
    double defaultVertOriginY, defaultAdvanceHeight;
    int    empty = 1;

    defaultVertOriginY   = PDFUNIT(g->default_advh - g->default_tsb);
    defaultAdvanceHeight = PDFUNIT(g->default_advh);

    w2_array = pdf_new_array();
    for (cid = 0; cid <= last_cid; cid++) {
        USHORT idx;
        double vertOriginX, vertOriginY, advanceHeight;

        if (!is_used_char2(used_chars, cid))
            continue;
        idx = tt_get_index(g, (USHORT)cid);
        if (cid != 0 && idx == 0)
            continue;
        advanceHeight = PDFUNIT(g->gd[idx].advh);
        vertOriginX   = PDFUNIT(0.5*(g->gd[idx].advw));
        vertOriginY   = PDFUNIT(g->gd[idx].tsb + g->gd[idx].ury);

        /*
         * c_first c_last w1_y v_x v_y
         * This form may hit Acrobat's implementation limit of array element size,
         * 8192. AFPL GhostScript 8.11 stops with rangecheck error with this.
         * Maybe GS's bug?
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
    }

    if (defaultVertOriginY != 880 || defaultAdvanceHeight != 1000) {
        an_array = pdf_new_array();
        pdf_add_array(an_array, pdf_new_number(defaultVertOriginY));
        pdf_add_array(an_array, pdf_new_number(-defaultAdvanceHeight));
        pdf_add_dict(fontdict, pdf_new_name ("DW2"), an_array);
    }
    if (!empty) {
        pdf_add_dict(fontdict,
                     pdf_new_name("W2"),
                     pdf_ref_obj(w2_array));
    }
    pdf_release_obj(w2_array);

    return;
}

/*
 * The following routine fixes few problems caused by vendor specific
 * Unicode mappings.
 */

#define FIX_CJK_UNIOCDE_SYMBOLS 1

static unsigned short
fix_CJK_symbols (unsigned short code)
{
    unsigned short alt_code;
    static struct
    {
        unsigned short alt1;
        unsigned short alt2;
    } CJK_Uni_symbols[] = {
        /*
         * Microsoft/Apple Unicode mapping difference:
         * They are taken from SJIS-Unicode mapping difference but nearly
         * same thing might be applied to Chinese (e.g., Big5) too.
         */
        {0x2014, 0x2015}, /* EM DASH <-> HORIZONTAL BAR */
        {0x2016, 0x2225}, /* DOUBLE VERTICAL LINE <-> PARALLEL TO */
        {0x203E, 0xFFE3}, /* OVERLINE <-> FULLWIDTH MACRON */
        {0x2026, 0x22EF}, /* HORIZONTAL ELLIPSIS <-> MIDLINE HORIZONTAL ELLIPSIS */
        {0x2212, 0xFF0D}, /* MINUS SIGN <-> FULLWIDTH HYPHEN-MINUS */
        {0x301C, 0xFF5E}, /* WAVE DASH <-> FULLWIDTH TILDE */
        {0xFFE0, 0x00A2}, /* FULLWIDTH CENT SIGN <-> CENT SIGN */
        {0xFFE1, 0x00A3}, /* FULLWIDTH POUND SIGN <-> POUND SIGN */
        {0xFFE2, 0x00AC}, /* FULLWIDTH NOT SIGN <-> NOT SIGN */
        {0xFFFF, 0xFFFF}, /* EOD */
    };
#define NUM_CJK_SYMBOLS (sizeof(CJK_Uni_symbols)/sizeof(CJK_Uni_symbols[0]))
    unsigned int i;

    alt_code = code;
    for (i = 0; i < NUM_CJK_SYMBOLS; i++) {
        if (CJK_Uni_symbols[i].alt1 == code) {
            alt_code = CJK_Uni_symbols[i].alt2;
            break;
        } else if (CJK_Uni_symbols[i].alt2 == code) {
            alt_code = CJK_Uni_symbols[i].alt1;
            break;
        }
    }

    return alt_code;
}

static int32_t
cid_to_code (CMap *cmap, CID cid, int unicode_cmap)
{
    unsigned char  inbuf[2], outbuf[32];
    size_t         inbytesleft = 2, outbytesleft = 32;
    const unsigned char *p;
    unsigned char *q;

    if (!cmap)
        return cid;

    inbuf[0] = (cid >> 8) & 0xff;
    inbuf[1] = cid & 0xff;
    p = inbuf; q = outbuf;

    CMap_decode_char(cmap, &p, &inbytesleft, &q, &outbytesleft);

    if (inbytesleft != 0)
        return -1;
    else if (outbytesleft == 31)
        return (int32_t) outbuf[0];
    else if (outbytesleft == 30)
        return (int32_t) (outbuf[0] << 8|outbuf[1]);
    else if (outbytesleft == 28) {
        if (unicode_cmap) {
            /* We assume the output encoding is UTF-16. */
            int32_t uc;
            const unsigned char *endptr;

            p = outbuf;
            endptr = p + 4;
            uc = UC_UTF16BE_decode_char(&p, endptr);
            if (p != endptr)
                dpx_warning("CID=%u mapped to non-single Unicode characters...", cid);
            return p == endptr ? uc : -1;
        } else {
            return (outbuf[0] << 24)|(outbuf[1] << 16)|(outbuf[2] << 8)|outbuf[3];
        }
    }

    return -1;
}

static uint16_t
cid_to_gid (CMap *cmap, CID cid)
{
    unsigned char        inbuf[2], outbuf[2];
    size_t               inbytesleft = 2, outbytesleft = 2;
    const unsigned char *p;
    unsigned char       *q;

    if (!cmap)
        return cid;

    inbuf[0] = (cid >> 8) & 0xff;
    inbuf[1] = cid & 0xff;
    p = inbuf; q = outbuf;

    CMap_decode_char(cmap, &p, &inbytesleft, &q, &outbytesleft);

    if (inbytesleft != 0 || outbytesleft != 0)
        return 0;

    return (uint16_t) (outbuf[0] << 8|outbuf[1]);
}

/* #define NO_GHOSTSCRIPT_BUG 1 */

int
CIDFont_type2_dofont (pdf_font *font)
{
    pdf_obj *fontfile;
    sfnt    *sfont;
    char    *h_used_chars, *v_used_chars, *used_chars;
    struct tt_glyphs *glyphs;
    CMap    *cmap = NULL;
    tt_cmap *ttcmap = NULL;
    ULONG    offset = 0;
    CID      cid, last_cid;
    unsigned char *cidtogidmap;
    USHORT   num_glyphs;
    enum {
        glyph_ordering,
        via_cid_to_code,
        via_cid_to_gid,
    } maptype = via_cid_to_code;
    int      i, unicode_cmap = 0;
    rust_input_handle_t handle = NULL;

    if (!font->reference)
        return 0;

    pdf_add_dict(font->resource,
                 pdf_new_name("FontDescriptor"), pdf_ref_obj(font->descriptor));

    if (font->flags & PDF_FONT_FLAG_BASEFONT)
        return 0;

    /*
     * CIDSystemInfo comes here since Supplement can be increased.
     */
    {
        pdf_obj *tmp;

        tmp = pdf_new_dict ();
        pdf_add_dict(tmp,
                     pdf_new_name("Registry"),
                     pdf_new_string(font->cid.csi.registry, strlen(font->cid.csi.registry)));
        pdf_add_dict(tmp,
                     pdf_new_name("Ordering"),
                     pdf_new_string(font->cid.csi.ordering, strlen(font->cid.csi.ordering)));
        pdf_add_dict(tmp,
                     pdf_new_name("Supplement"),
                     pdf_new_number(font->cid.csi.supplement));
        pdf_add_dict(font->resource, pdf_new_name("CIDSystemInfo"), tmp);
    }

    /* Quick exit for non-embedded & fixed-pitch font. */
    if (!font->cid.options.embed &&
        (opt_flags_cidfont & CIDFONT_FORCE_FIXEDPITCH)) {
        pdf_add_dict(font->resource,
                     pdf_new_name("DW"), pdf_new_number(1000.0));
        return 0;
    }

    handle = dpx_open_truetype_file(font->filename);
    if (!handle) {
        handle = dpx_open_dfont_file(font->filename);
        if (!handle) {
            dpx_warning("Could not open TTF/dfont file: %s", font->filename);
            return -1;
        }
        sfont = dfont_open(handle, font->index);
    } else {
        sfont = sfnt_open(handle);
    }

    if (!sfont) {
        dpx_warning("Could not open TTF file: %s", font->filename);
        ttstub_input_close(handle);
        return -1;
    }

    switch (sfont->type) {
    case SFNT_TYPE_TTC:
        offset = ttc_read_offset(sfont, font->index);
        if (offset == 0) {
            dpx_warning("Invalid TTC index in %s.", font->filename);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
        }
        break;
    case SFNT_TYPE_TRUETYPE:
        if (font->index > 0) {
            dpx_warning("Found TrueType font file while expecting TTC file (%s).", font->filename);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
        }
        offset = 0;
        break;
    case SFNT_TYPE_DFONT:
        offset = sfont->offset;
        break;
    default:
        dpx_warning("Not a TrueType/TTC font (%s)?", font->filename);
        sfnt_close(sfont);
        ttstub_input_close(handle);
        return -1;
        break;
    }

    if (sfnt_read_table_directory(sfont, offset) < 0) {
        dpx_warning("Could not read TrueType table directory (%s).", font->filename);
        sfnt_close(sfont);
        ttstub_input_close(handle);
        return -1;
    }

    /*
     * Adobe-Identity means font's internal glyph ordering here.
     */
    if (streq_ptr(font->cid.csi.registry, "Adobe") &&
        streq_ptr(font->cid.csi.ordering, "Identity")) {
        maptype = glyph_ordering;
        ttcmap = NULL;
        cmap = NULL;
    } else {
        if (font->cid.csi.registry && font->cid.csi.ordering) {
            char   *cmap_name;
            size_t  len;
            int     cmap_id;

            len  = strlen(font->cid.csi.registry) + strlen(font->cid.csi.ordering);
            len += strlen(font->fontname);
            len += 3;
            cmap_name = NEW(len+1, char);
            snprintf(cmap_name, len + 1, "%s-%s-%s",
                    font->cid.csi.registry, font->cid.csi.ordering, font->fontname);
            cmap_id = CMap_cache_find(cmap_name);
            free(cmap_name);
            if (cmap_id >= 0) {
                cmap    = CMap_cache_get(cmap_id);
                ttcmap  = NULL;
                maptype = via_cid_to_gid;
            }
        }
        if (maptype != via_cid_to_gid) {
            maptype = via_cid_to_code;
            /*
            * This part contains a bug. It may choose SJIS encoding TrueType cmap
            * table for Adobe-GB1.
            */
            for (i = 0; i <= KNOWN_ENCODINGS_MAX; i++) {
                ttcmap = tt_cmap_read(sfont,
                                    known_encodings[i].platform,
                                    known_encodings[i].encoding);
                if (ttcmap)
                    break;
            }
            if (!ttcmap) {
                dpx_warning("No usable TrueType cmap table found for font \"%s\".", font->filename);
                dpx_warning("CID character collection for this font is set to \"%s-%s\"",
                    font->cid.csi.registry, font->cid.csi.ordering);
                dpx_warning("Cannot continue without this...");
                sfnt_close(sfont);
                ttstub_input_close(handle);
                return -1;
            } else if (i <= WIN_UCS_INDEX_MAX) {
                unicode_cmap = 1;
            } else {
                unicode_cmap = 0;
            }

            if (streq_ptr(font->cid.csi.ordering, "UCS") && i <= WIN_UCS_INDEX_MAX) {
                cmap = NULL;
            } else {
                cmap = find_tocode_cmap(font->cid.csi.registry, font->cid.csi.ordering, i);
                if (!cmap) {
                    sfnt_close(sfont);
                    ttstub_input_close(handle);
                    return -1;
                }
            }
        }
    }

    glyphs = tt_build_init();

    last_cid   = 0;
    num_glyphs = 1; /* .notdef */
    used_chars = h_used_chars = v_used_chars = NULL;
    {
        int c;

        h_used_chars = font->usedchars;
        v_used_chars = font->cid.usedchars_v;

        assert(h_used_chars || v_used_chars);

        /*
         * Quick check of max CID.
         */
        c = 0;
        for (i = 8191; i >= 0; i--) {
            if (h_used_chars && h_used_chars[i] != 0) {
                last_cid = i * 8 + 7;
                c = h_used_chars[i];
                break;
            }
        }
        for (i = 8191; i >= 0; i--) {
            if (v_used_chars && v_used_chars[i] != 0) {
                if (i * 8 + 7 >= last_cid) {
                    c = (i * 8 + 7 > last_cid) ? (v_used_chars[i]) : (c | v_used_chars[i]);
                    last_cid = i * 8 + 7;
                    break;
                }
            }
        }
        if (last_cid > 0) {
            for (i = 0; i < 8; i++) {
                if ((c >> i) & 1)
                    break;
                last_cid--;
            }
        }

        assert(last_cid < 0xFFFFu);
    }

#ifndef NO_GHOSTSCRIPT_BUG
    cidtogidmap = NULL;
#else
    cidtogidmap = NEW((last_cid + 1) * 2, unsigned char);
    memset(cidtogidmap, 0, (last_cid + 1) * 2);
#endif /* !NO_GHOSTSCRIPT_BUG */

    /*
     * Map CIDs to GIDs.
     * Horizontal and vertical used_chars are merged.
     */

    /*
     * Horizontal
     */
    if (h_used_chars) {
        used_chars = h_used_chars;
        for (cid = 1; cid <= last_cid; cid++) {
            int32_t code;
            uint16_t gid = 0;

            if (!is_used_char2(h_used_chars, cid))
                continue;

            switch (maptype) {
            case glyph_ordering:
                gid  = cid;
                code = cid;
                break;
            case via_cid_to_gid:
                gid  = cid_to_gid(cmap, cid);
                code = cid;
                break;
            case via_cid_to_code:
                code = cid_to_code(cmap, cid, unicode_cmap);
                if (code < 0) {
                    dpx_warning("Unable to map CID to code: CID=%u", cid);
                } else {
                    gid  = tt_cmap_lookup(ttcmap, code);
#ifdef FIX_CJK_UNIOCDE_SYMBOLS
                    if (gid == 0 && unicode_cmap) {
                        int alt_code;

                        alt_code = fix_CJK_symbols((unsigned short)code);
                        if (alt_code != code) {
                            gid = tt_cmap_lookup(ttcmap, alt_code);
                            if (gid != 0) {
                                dpx_warning("Unicode char U+%04x replaced with U+%04x.",
                                            code, alt_code);
                            }
                        }
                    }
#endif /* FIX_CJK_UNIOCDE_SYMBOLS */
                }
                break;
            }

            if (gid == 0) {
                if (code >= 0)
                    dpx_warning("Glyph missing in font. (CID=%u, code=0x%04x)", cid, code);
            }

            /* TODO: duplicated glyph */
#ifndef NO_GHOSTSCRIPT_BUG
            gid = tt_add_glyph(glyphs, gid, cid);
#else
            gid = tt_add_glyph(glyphs, gid, num_glyphs);
            cidtogidmap[2*cid  ] = gid >> 8;
            cidtogidmap[2*cid+1] = gid & 0xff;
#endif /* !NO_GHOSTSCRIPT_BUG */

            num_glyphs++;
        }
    }

    /*
     * Vertical
     */
    if (v_used_chars) {
        otl_gsub *gsub_list = NULL;

        /*
         * Require `vrt2' or `vert'.
         */
        if (maptype != via_cid_to_code) {
            gsub_list = NULL;
        } else {
            gsub_list = otl_gsub_new();
            if (otl_gsub_add_feat(gsub_list,
                                  "*", "*", "vrt2", sfont) < 0) {
                if (otl_gsub_add_feat(gsub_list,
                                      "*", "*", "vert", sfont) < 0) {
                    dpx_warning("GSUB feature vrt2/vert not found.");
                    otl_gsub_release(gsub_list);
                    gsub_list = NULL;
                } else {
                    otl_gsub_select(gsub_list, "*", "*", "vert");
                }
            } else {
                otl_gsub_select(gsub_list, "*", "*", "vrt2");
            }
        }

        for (cid = 1; cid <= last_cid; cid++) {
            int32_t code;
            uint16_t gid;

            if (!is_used_char2(v_used_chars, cid))
                continue;

            /* There may be conflict of horizontal and vertical glyphs
             * when font is used with /UCS. However, we simply ignore
             * that...
             */
            if (h_used_chars && is_used_char2(h_used_chars, cid)) {
                continue;
            }

            switch (maptype) {
            case glyph_ordering:
                gid = cid;
                code = cid;
                break;
            case via_cid_to_gid:
                gid = cid_to_gid(cmap, cid);
                code = cid;
                break;
            case via_cid_to_code:
                code = cid_to_code(cmap, cid, unicode_cmap);
                if (code < 0) {
                    dpx_warning("Unable to map CID to code: CID=%u", cid);
                } else {
                    gid  = tt_cmap_lookup(ttcmap, code);
#ifdef FIX_CJK_UNIOCDE_SYMBOLS
                    if (gid == 0 && unicode_cmap) {
                        int alt_code;

                        alt_code = fix_CJK_symbols((unsigned short)code);
                        if (alt_code != code) {
                            gid = tt_cmap_lookup(ttcmap, alt_code);
                            if (gid != 0) {
                                dpx_warning("Unicode char U+%04x replaced with U+%04x.",
                                            code, alt_code);
                            }
                        }
                    }
#endif /* FIX_CJK_UNIOCDE_SYMBOLS */
                }
                break;
            }
            if (gid == 0) {
                if (code >= 0)
                    dpx_warning("Glyph missing in font. (CID=%u, code=0x%04x)", cid, code);
            } else if (gsub_list) {
                otl_gsub_apply(gsub_list, &gid);
            }

#ifndef NO_GHOSTSCRIPT_BUG
            gid = tt_add_glyph(glyphs, gid, cid);
#else
            gid = tt_add_glyph(glyphs, gid, num_glyphs);
            cidtogidmap[2*cid  ] = gid >> 8;
            cidtogidmap[2*cid+1] = gid & 0xff;
#endif /* !NO_GHOSTSCRIPT_BUG */

            if (used_chars) /* merge vertical used_chars to horizontal */
                add_to_used_chars2(used_chars, cid);

            num_glyphs++;
        }

        if (gsub_list)
            otl_gsub_release(gsub_list);

        if (!used_chars) /* We have no horizontal. */
            used_chars = v_used_chars;
    }

    assert(used_chars);

    tt_cmap_release(ttcmap);

    if (font->cid.options.embed) {
        if (tt_build_tables(sfont, glyphs) < 0) {
            dpx_warning("Could not created FontFile stream.");
            if (cidtogidmap)
                free(cidtogidmap);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
        }
        if (dpx_conf.verbose_level > 1)
            dpx_message("[%u glyphs (Max CID: %u)]", glyphs->num_glyphs, last_cid);
    } else {
        if (tt_get_metrics(sfont, glyphs) < 0) {
            dpx_warning("Reading glyph metrics failed...");
            if (cidtogidmap)
                free(cidtogidmap);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
        }
    }

    /*
     * DW, W, DW2, and W2
     */
    if (opt_flags_cidfont & CIDFONT_FORCE_FIXEDPITCH) {
        pdf_add_dict(font->resource,
                     pdf_new_name("DW"), pdf_new_number(1000.0));
    } else {
        add_TTCIDHMetrics(font->resource, glyphs, used_chars, cidtogidmap, last_cid);
        if (font->cid.need_vmetrics)
            add_TTCIDVMetrics(font->resource, glyphs, used_chars, last_cid);
    }

    /* CIDSet
     * NOTE: All glyphs including component glyph and dummy glyph must be
     * listed in CIDSet. However, .notdef glyph should be ommitted.
     */
    if (pdf_check_version(2, 0) < 0) {
      pdf_obj *cidset;
      char    *cidset_data;

      cidset_data = NEW(glyphs->last_gid/8 + 1, char);
      memset(cidset_data, 0, glyphs->last_gid/8 + 1);
      for (i = 1; i <= glyphs->last_gid; i++)
        cidset_data[i/8] |= (1 << (7 - i % 8));
      cidset = pdf_new_stream(STREAM_COMPRESS);
      pdf_add_stream(cidset, cidset_data, glyphs->last_gid/8 + 1);
      free(cidset_data);
      pdf_add_dict(font->descriptor,
        pdf_new_name("CIDSet"), pdf_ref_obj(cidset));
      pdf_release_obj(cidset);
    }

    tt_build_finish(glyphs);

    /* Finish here if not embedded. */
    if (!font->cid.options.embed) {
        free(cidtogidmap);
        sfnt_close(sfont);
        if (handle)
            ttstub_input_close(handle);

        return 0;
    }

    /* Create font file */
    for (i = 0; required_table[i].name; i++) {
        if (sfnt_require_table(sfont,
                               required_table[i].name,
                               required_table[i].must_exist) < 0) {
            dpx_warning("Some required TrueType table (%s) does not exist.", required_table[i].name);
            free(cidtogidmap);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
        }
    }

    /*
     * FontFile2
     */
    fontfile = sfnt_create_FontFile_stream(sfont);

    sfnt_close(sfont);
    if (handle)
        ttstub_input_close(handle);

    if (!fontfile) {
        dpx_warning("Could not created FontFile stream for \"%s\".", font->filename);
        free(cidtogidmap);
        return -1;
    }

    if (dpx_conf.verbose_level > 1) {
        dpx_message("[%d bytes]", pdf_stream_length(fontfile));
    }

    pdf_add_dict(font->descriptor,
                 pdf_new_name("FontFile2"),
                 pdf_ref_obj (fontfile));
    pdf_release_obj(fontfile);

    /*
     * CIDToGIDMap
     * Adobe's PDF Reference had been describing it as "optional" and
     * default value as "Identity". However, ISO 32000-1 requires it
     * for Type 2 CIDFonts with embedded font programs.
     */
    if (!cidtogidmap) {
        pdf_add_dict(font->resource,
                     pdf_new_name("CIDToGIDMap"),
                     pdf_new_name("Identity"));
    } else {
        pdf_obj *c2gmstream;

        c2gmstream = pdf_new_stream(STREAM_COMPRESS);
        pdf_add_stream(c2gmstream, cidtogidmap, (last_cid + 1) * 2);
        pdf_add_dict(font->resource,
                     pdf_new_name("CIDToGIDMap"),
                     pdf_ref_obj (c2gmstream));
        pdf_release_obj(c2gmstream);
        free(cidtogidmap);
    }

    return 0;
}

int
CIDFont_type2_open (pdf_font *font, const char *name,
                    int index, cid_opt *opt)
{
    int error;
    char    *fontname;
    sfnt    *sfont;
    ULONG    offset = 0;
    rust_input_handle_t handle = NULL;

    assert(font && opt);

    handle = dpx_open_truetype_file(name);
    if (!handle) {
        handle = dpx_open_dfont_file(name);
        if (!handle) return -1;
        sfont = dfont_open(handle, index);
    } else {
        sfont = sfnt_open(handle);
    }

    if (!sfont) {
        ttstub_input_close(handle);
        return -1;
    }

    switch (sfont->type) {
    case SFNT_TYPE_TTC:
        offset = ttc_read_offset(sfont, index);
        break;
    case SFNT_TYPE_TRUETYPE:
        if (index > 0) {
            dpx_warning("Invalid TTC index (not TTC font): %s", name);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
        }
        offset = 0;
        break;
    case SFNT_TYPE_DFONT:
        offset = sfont->offset;
        break;
    default:
        sfnt_close(sfont);
        if (handle)
            ttstub_input_close(handle);
        return -1;
    }

    if (sfnt_read_table_directory(sfont, offset) < 0) {
        dpx_warning("Reading TrueType table directory failed.");
        sfnt_close(sfont);
        ttstub_input_close(handle);
        return -1;
    }

    /* Ignore TrueType Collection with CFF table. */
    if (sfont->type == SFNT_TYPE_TTC && sfnt_find_table_pos(sfont, "CFF ")) {
        sfnt_close(sfont);
        if (handle)
            ttstub_input_close(handle);
        return -1;
    }

    {
        char *shortname;
        int   namelen;

        /* MAC-ROMAN-EN-POSTSCRIPT or WIN-UNICODE-EN(US)-POSTSCRIPT */
        shortname = NEW(PDF_NAME_LEN_MAX, char);
        namelen   = tt_get_ps_fontname(sfont, shortname, PDF_NAME_LEN_MAX);
        if (namelen == 0) {
            memset(shortname, 0, PDF_NAME_LEN_MAX);
            strncpy(shortname, name, PDF_NAME_LEN_MAX);
            namelen = strlen(shortname);
        }
        error = validate_name(shortname, namelen); /* for SJIS, UTF-16, ... string */

        if (error) {
            free(shortname);
            sfnt_close(sfont);
            ttstub_input_close(handle);
            return -1;
        }
        /*
         * Strlen works, after validate_named string.
         * Mangled name requires more 7 bytes.
         * Style requires more 11 bytes.
         */
        fontname = NEW(strlen(shortname)+19, char);
        strcpy(fontname, shortname);
        free(shortname);
    }

    if (opt->embed && opt->style != FONT_STYLE_NONE) {
        dpx_warning("Embedding disabled due to style option for %s.", name);
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
    /*
     * CIDSystemInfo is determined from CMap or from map record option.
     */
    font->fontname = fontname;
    font->subtype  = PDF_FONT_FONTTYPE_CIDTYPE2;

    if (opt->csi.registry && opt->csi.ordering) {
        font->cid.csi.registry   = NEW(strlen(opt->csi.registry)+1, char);
        strcpy(font->cid.csi.registry, opt->csi.registry);
        font->cid.csi.ordering   = NEW(strlen(opt->csi.ordering)+1, char);
        strcpy(font->cid.csi.ordering, opt->csi.ordering);
        font->cid.csi.supplement = opt->csi.supplement;
    }

    font->resource = pdf_new_dict();
    pdf_add_dict(font->resource,
                 pdf_new_name("Type"),
                 pdf_new_name("Font"));
    pdf_add_dict(font->resource,
                 pdf_new_name("Subtype"),
                 pdf_new_name("CIDFontType2"));

    font->descriptor = tt_get_fontdesc(sfont, &(opt->embed), opt->stemv, 0, name);
    if (!font->descriptor) {
        dpx_warning("Could not obtain necessary font info.");
        sfnt_close(sfont);
        ttstub_input_close(handle);
        return -1;
    }

    if (opt->embed) {
        char *tmp;

        pdf_font_make_uniqueTag(font->uniqueID);
        tmp = NEW(strlen(fontname)+8, char);
        sprintf(tmp, "%s+%s", font->uniqueID, font->fontname);
        pdf_add_dict(font->descriptor, pdf_new_name("FontName"), pdf_new_name(tmp));
        pdf_add_dict(font->resource, pdf_new_name("BaseFont"), pdf_new_name(tmp));
        free(tmp);
    } else {
        pdf_add_dict(font->descriptor, pdf_new_name("FontName"), pdf_new_name(font->fontname));
        pdf_add_dict(font->resource, pdf_new_name("BaseFont"), pdf_new_name(font->fontname));
    }

    sfnt_close(sfont);
    if (handle)
        ttstub_input_close(handle);

    /*
     * Don't write fontdict here.
     * /Supplement in /CIDSystemInfo may change.
     */

    return 0;
}
