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

#include "dpx-truetype.h"

#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfencoding.h"
#include "dpx-pdffont.h"
#include "dpx-pdfobj.h"
/* TrueType */
#include "dpx-sfnt.h"
#include "dpx-tfm.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_cmap.h"
#include "dpx-tt_glyf.h"
#include "dpx-tt_gsub.h"
#include "dpx-tt_post.h"
#include "dpx-tt_table.h"

/* Modifying this has no effect :P */
#ifdef ENABLE_NOEMBED
#  undef ENABLE_NOEMBED
#endif

int
pdf_font_open_truetype (pdf_font *font)
{
    char     *ident;
    int       index, encoding_id;
    pdf_obj  *fontdict, *descriptor;
    sfnt     *sfont;
    int       embedding = 1; /* Must be embedded. */
    rust_input_handle_t handle = NULL;
    int       length, error = 0;

    assert( font );

    ident = pdf_font_get_ident(font);
    index = pdf_font_get_index(font);

    assert( ident );

    handle = dpx_open_truetype_file(ident);
    if (!handle) {
        handle = dpx_open_dfont_file(ident);
        if (!handle)
            return -1;

        sfont = dfont_open(handle, index);
    } else {
        sfont = sfnt_open(handle);
    }

    if (!sfont) {
        dpx_warning("Could not open TrueType font: %s", ident);
        ttstub_input_close(handle);
        return -1;
    }

    if (sfont->type == SFNT_TYPE_TTC) {
        ULONG offset;
        offset = ttc_read_offset(sfont, index);
        if (offset == 0) _tt_abort("Invalid TTC index in %s.", ident);
        error = sfnt_read_table_directory(sfont, offset);
    } else {
        error = sfnt_read_table_directory(sfont, sfont->offset);
    }

    if (error) {
        sfnt_close(sfont);
        ttstub_input_close(handle);
        return -1; /* Silently */
    }

    /* Reading fontdict before checking fonttype conflicts with PKFONT
     * because pdf_font_get_resource() always makes a dictionary.
     */
    encoding_id = pdf_font_get_encoding(font);
    fontdict    = pdf_font_get_resource(font);
    descriptor  = pdf_font_get_descriptor(font);
#ifdef  ENABLE_NOEMBED
    embedding   = pdf_font_get_flag(font, PDF_FONT_FLAG_NOEMBED) ? 0 : 1;
#endif /* ENABLE_NOEMBED */

    assert( fontdict && descriptor );

    {
        char  fontname[256];
        int   n;
        pdf_obj  *tmp;

        memset(fontname, 0, 256);
        length = tt_get_ps_fontname(sfont, fontname, 255);
        if (length < 1) {
            length = MIN(strlen(ident), 255);
            memcpy(fontname, ident, length);
        }
        fontname[length] = '\0';
        for (n = 0; n < length; n++) {
            if (fontname[n] == 0) {
                memmove(fontname + n, fontname + n + 1, length - n - 1);
            }
        }
        if (strlen(fontname) == 0)
            _tt_abort("Can't find valid fontname for \"%s\".", ident);
        pdf_font_set_fontname(font, fontname);

        tmp  = tt_get_fontdesc(sfont, &embedding, -1, 1, fontname);
        if (!tmp) {
            sfnt_close(sfont);
            ttstub_input_close(handle);
            _tt_abort("Could not obtain necessary font info.");
        }
        assert(pdf_obj_typeof(tmp) == PDF_DICT);

        pdf_merge_dict(descriptor, tmp);
        pdf_release_obj(tmp);
    }

    if (!embedding) {
        if (encoding_id >= 0 &&
            !pdf_encoding_is_predefined(encoding_id)) {
            sfnt_close(sfont);
            _tt_abort("Custom encoding not allowed for non-embedded TrueType font.");
        } else {
            /* There are basically no guarantee for font substitution
             * can work with "symblic" fonts. At least all glyphs
             * contained in the font must be identified; glyphs covers
             * by this instance of font should contain glyphs only from
             * Adobe Standard Latin Set. We allow non-embedded font
             * only to predefined encodings for this reason. Note that
             * "builtin" encoding means "MacRoman" here.
             */
#ifndef  ENABLE_NOEMBED
            _tt_abort("Font file=\"%s\" can't be embedded due to liscence restrictions.", ident);
#else
            pdf_obj  *tmp;
            int       flags;

            pdf_font_set_flags(font, PDF_FONT_FLAG_NOEMBED);
            tmp = pdf_lookup_dict(descriptor, "Flags");
            if (tmp && pdf_obj_typeof(tmp) == PDF_NUMBER) {
                flags  = (int) pdf_number_value(tmp);
                flags &= (1 << 2); /* clear Symbolic */
                flags |= (1 << 5); /* set Nonsymbolic */
                pdf_add_dict(descriptor, pdf_new_name("Flags"), pdf_new_number(flags));
            }
#endif /* ENABLE_NOEMBED */
        }
    }

    sfnt_close(sfont);
    ttstub_input_close(handle);

    pdf_add_dict(fontdict,
                 pdf_new_name("Type"),    pdf_new_name("Font"));
    pdf_add_dict(fontdict,
                 pdf_new_name("Subtype"), pdf_new_name("TrueType"));

    return  0;
}

/*
 * The 'name' table should be preserved since it contains copyright
 * information, but it might cause problem when there are invalid
 * table entries (wrongly encoded text which is often the case in
 * CJK fonts). Acrobat does not use 'name' table. Unicode TrueType
 * fonts may have 10K bytes 'name' table...
 *
 * We preserve the 'OS/2' table too, since it contains the license
 * information. PDF applications should use this table to decide
 * whether the font is embedded only for the purpose of preview &
 * printing. Otherwise, we must encrypt the document. Acrobat does
 * not use 'OS/2' table, though...
 */
static struct
{
    const char *name;
    int   must_exist;
} required_table[] = {
    {"OS/2", 0}, {"head", 1}, {"hhea", 1}, {"loca", 1}, {"maxp", 1},
    {"name", 1}, {"glyf", 1}, {"hmtx", 1}, {"fpgm", 0}, {"cvt ", 0},
    {"prep", 0}, {"cmap", 1}, {NULL, 0}
};

static void
do_widths (pdf_font *font, double *widths)
{
    pdf_obj  *fontdict;
    pdf_obj  *tmparray;
    int       code, firstchar, lastchar, tfm_id;
    char     *usedchars;

    fontdict   = pdf_font_get_resource  (font);
    usedchars  = pdf_font_get_usedchars (font);

    tmparray = pdf_new_array();
    for (firstchar = 255, lastchar = 0, code = 0; code < 256; code++) {
        if (usedchars[code]) {
            if (code < firstchar) firstchar = code;
            if (code > lastchar)  lastchar  = code;
        }
    }
    if (firstchar > lastchar) {
        dpx_warning("No glyphs actually used???");
        pdf_release_obj(tmparray);
        return;
    }
    tfm_id = tfm_open(pdf_font_get_mapname(font), 0);
    for (code = firstchar; code <= lastchar; code++) {
        if (usedchars[code]) {
            double width;
            if (tfm_id < 0) /* tfm is not found */
                width = widths[code];
            else
                width = 1000. * tfm_get_width(tfm_id, code);
            pdf_add_array(tmparray,
                          pdf_new_number(ROUND(width, 0.1)));
        } else {
            pdf_add_array(tmparray, pdf_new_number(0.0));
        }
    }

    if (pdf_array_length(tmparray) > 0) {
        pdf_add_dict(fontdict,
                     pdf_new_name("Widths"), pdf_ref_obj(tmparray));
    }
    pdf_release_obj(tmparray);

    pdf_add_dict(fontdict,
                 pdf_new_name("FirstChar"), pdf_new_number(firstchar));
    pdf_add_dict(fontdict,
                 pdf_new_name("LastChar"),  pdf_new_number(lastchar));

    return;
}

#define PDFUNIT(v) ((double) (ROUND(1000.0*(v)/(glyphs->emsize), 1)))

/*
 * There are several issues in TrueType font support in PDF.
 * How PDF viewers select TrueType cmap table is not so clear.
 * Most reliable way seem to reencode font and sort glyphs as
 * charcode == gid and to use Mac-Roman format 0 subtable.
 * It does not work with encodings that uses full 256 range since
 * GID = 0 is reserved for .notdef, so GID = 256 is not accessible.
 */
static int
do_builtin_encoding (pdf_font *font, const char *usedchars, sfnt *sfont)
{
    struct tt_glyphs *glyphs;
    char             *cmap_table;
    tt_cmap          *ttcm;
    USHORT            gid, idx;
    int               code, count;
    double            widths[256];

    ttcm = tt_cmap_read(sfont, TT_MAC, TT_MAC_ROMAN);
    if (!ttcm) {
        dpx_warning("Could not read Mac-Roman TrueType cmap table...");
        return  -1;
    }

    cmap_table = NEW(274, char);
    memset(cmap_table, 0, 274);
    sfnt_put_ushort(cmap_table,    0);            /* Version  */
    sfnt_put_ushort(cmap_table+2,  1);            /* Number of subtables */
    sfnt_put_ushort(cmap_table+4,  TT_MAC);       /* Platform ID */
    sfnt_put_ushort(cmap_table+6,  TT_MAC_ROMAN); /* Encoding ID */
    sfnt_put_ulong (cmap_table+8,  12);           /* Offset   */
    sfnt_put_ushort(cmap_table+12, 0);            /* Format   */
    sfnt_put_ushort(cmap_table+14, 262);          /* Length   */
    sfnt_put_ushort(cmap_table+16, 0);            /* Language */

    glyphs = tt_build_init();

    if (dpx_conf.verbose_level > 2)
        dpx_message("[glyphs:/.notdef");

    count = 1; /* .notdef */
    for (code = 0; code < 256; code++) {
        if (!usedchars[code])
            continue;

        if (dpx_conf.verbose_level > 2)
            dpx_message("/.c0x%02x", code);

        gid = tt_cmap_lookup(ttcm, code);
        if (gid == 0) {
            dpx_warning("Glyph for character code=0x%02x missing in font font-file=\"%s\".",
                        code, pdf_font_get_ident(font));
            idx = 0;
        } else {
            idx = tt_find_glyph(glyphs, gid);
            if (idx == 0)
                idx  = tt_add_glyph(glyphs, (USHORT)gid, (USHORT)count); /* count returned. */
        }
        cmap_table[18+code] = idx & 0xff; /* bug here */
        count++;
    }
    tt_cmap_release(ttcm);

    if (dpx_conf.verbose_level > 2)
        dpx_message("]");

    if (tt_build_tables(sfont, glyphs) < 0) {
        dpx_warning("Packing TrueType font into SFNT failed!");
        tt_build_finish(glyphs);
        free(cmap_table);
        return  -1;
    }

    for (code = 0; code < 256; code++) {
        if (usedchars[code]) {
            idx = tt_get_index(glyphs, (USHORT) cmap_table[18+code]);
            widths[code] = PDFUNIT(glyphs->gd[idx].advw);
        } else {
            widths[code] = 0.0;
        }
    }
    do_widths(font, widths);

    if (dpx_conf.verbose_level > 1)
        dpx_message("[%d glyphs]", glyphs->num_glyphs);

    tt_build_finish(glyphs);

    sfnt_set_table(sfont, "cmap", cmap_table, 274);

    return  0;
}

/* Order of lookup should be
 *  post, unicode+otl
 */
struct glyph_mapper
{
    tt_cmap  *codetogid;
    otl_gsub *gsub;
    sfnt     *sfont;
    struct tt_post_table *nametogid;
};


/* WARNING: This modifies glyphname itself */
static int
agl_decompose_glyphname (char *glyphname, char **nptrs, int size, char **suffix)
{
    char  *q, *p = glyphname;
    int    n;

    q = strchr(p, '.'); /* chop every thing after *first* dot */
    if (!q)
        *suffix = NULL;
    else {
        *q = '\0'; q++;
        *suffix = q;
    }

    nptrs[0] = p;
    for (n = 1; p && *p; n++) {
        p = strchr(p, '_');
        if (!p || p[1] == '\0')
            break;
        if (n >= size)
            _tt_abort("Uh ah..."); /* _FIXME_ */
        *p = '\0'; p++;
        nptrs[n] = p;
    }

    return  n;
}

static int
select_gsub (const char *feat, struct glyph_mapper *gm)
{
    int    idx, error = 0;

    if (!feat || *feat == 0 || !gm || !gm->gsub)
        return  -1;

    /* First treat as is */
    idx = otl_gsub_select(gm->gsub, "*", "*", feat);
    if (idx >= 0)
        return  0;

    if (dpx_conf.verbose_level > 1)
        dpx_message("\ntrutype>> Try loading OTL GSUB for \"*.*.%s\"...", feat);
    error = otl_gsub_add_feat(gm->gsub, "*", "*", feat, gm->sfont);
    if (!error) {
        idx = otl_gsub_select(gm->gsub, "*", "*", feat);
        return  (idx >= 0 ? 0 : -1);
    }

    return  -1;
}

static int findparanoiac (const char *glyph_name, USHORT *gid, struct glyph_mapper *gm);
static int resolve_glyph (const char *glyph_name, USHORT *gid, struct glyph_mapper *gm);

/* Apply GSUB. This is a bit tricky... */
static int
selectglyph (USHORT in, const char *suffix, struct glyph_mapper *gm, USHORT *out)
{
    char  *s, *q, t[5];
    const char *r;
    int    n, error = 0;

    assert(suffix && gm && out);
    assert(suffix && *suffix != 0);

    s = NEW(strlen(suffix) + 1, char);
    strcpy(s, suffix);

    /* First try converting suffix to feature tag.
     * agl.c currently only knows less ambiguos cases;
     * e.g., 'sc', 'superior', etc.
     */
    r = agl_suffix_to_otltag(s);
    if (r) { /* We found feature tag for 'suffix'. */
        error = select_gsub(r, gm); /* no fallback for this */
        if (!error)
            error = otl_gsub_apply(gm->gsub, &in);
    } else { /* 'suffix' may represent feature tag. */
        /* Try loading GSUB only when length of 'suffix' is less
         * than or equal to 4. tt_gsub give a warning otherwise.
         */
        if (strlen(s) > 4)
            error = -1; /* Uh */
        else if (strlen(s) == 4)
            error = select_gsub(s, gm);
        else { /* less than 4. pad ' '. */
            memset(t, ' ', 4); t[4] = '\0';
            memcpy(t, s, strlen(s));
            error = select_gsub(t, gm);
        }
        if (!error) /* 'suffix' represents feature tag. */
            error = otl_gsub_apply(gm->gsub, &in);
        else { /* other case: alt1, nalt10... (alternates) */
            for (q = s + strlen(s) - 1; q > s && *q >= '0' && *q <= '9'; q--);
            if (q == s)
                error = -1;
            else { /* starting at 1 */
                n = atoi(q + 1) - 1; q[1] = '\0';
                if (strlen(s) > 4)
                    error = -1;
                else { /* This may be alternate substitution. */
                    memset(t, ' ', 4); t[4] = '\0';
                    memcpy(t, s, strlen(s));
                    error = select_gsub(s, gm);
                    if (!error)
                        error = otl_gsub_apply_alt(gm->gsub, (USHORT)n, (USHORT *)&in);
                }
            }
        }
    }
    free(s);

    *out = in;
    return  error;
}


/* Compose glyphs via ligature substitution. */
static int
composeglyph (USHORT *glyphs, int n_glyphs,
              const char *feat, struct glyph_mapper *gm, USHORT *gid)
{
    int   error = 0;
    char  t[5] = {' ', ' ', ' ', ' ', 0};

    assert(glyphs && n_glyphs > 0 && gm && gid);

    if (!feat || feat[0] == '\0') /* meaning "Unknown" */
        error = select_gsub("(?lig|lig?|?cmp|cmp?|frac|afrc)", gm);
    else {
        if (strlen(feat) > 4)
            error = -1;
        else {
            memcpy(t, feat, strlen(feat));
            error = select_gsub(t, gm);
        }
    }

    if (!error)
        error = otl_gsub_apply_lig(gm->gsub, (USHORT *)glyphs, (USHORT)n_glyphs,
                                   (USHORT *)gid);

    return  error;
}

/* This may be called by findparanoiac(). */
static int
composeuchar (int32_t *unicodes, int n_unicodes,
              const char *feat, struct glyph_mapper *gm, USHORT *gid)
{
    USHORT  *gids;
    int      i, error = 0;

    if (!gm->codetogid)
        return  -1;

    gids = NEW(n_unicodes, USHORT);
    for (i = 0;
         !error && i < n_unicodes; i++) {
        gids[i] = tt_cmap_lookup(gm->codetogid, unicodes[i]);
        error   = (gids[i] == 0) ? -1 : 0;
    }

    if (!error)
        error = composeglyph(gids, n_unicodes, feat, gm, gid);

    free(gids);

    return  error;
}

/* Search 'post' table. */
static int
findposttable (const char *glyph_name, USHORT *gid, struct glyph_mapper *gm)
{
    if (!gm->nametogid)
        return -1;

    *gid = tt_lookup_post_table(gm->nametogid, glyph_name);

    return (*gid == 0 ? -1 : 0);
}

/* This is wrong. We must care about '.'. */
#define is_comp(n) (strchr((n), '_') != NULL)

/* Glyph names are concatinated with '_'. */
static int
findcomposite (const char *glyphname, USHORT *gid, struct glyph_mapper *gm)
{
    char     *gname, *suffix = NULL;
    USHORT    gids[32];
    char     *nptrs[32];
    int       i, n_comp;
    int       error = 0;

    error = findposttable(glyphname, gid, gm);
    if (!error)
        return  0;

    gname = NEW(strlen(glyphname) + 1, char);
    strcpy(gname, glyphname);

    memset(gids, 0, 32 * sizeof(USHORT));
    n_comp = agl_decompose_glyphname(gname, nptrs, 32, &suffix);
    for (error = 0, i = 0; !error && i < n_comp; i++) {
        error = resolve_glyph(nptrs[i], &gids[i], gm);
        if (error)
            dpx_warning("Could not resolve glyph \"%s\" (%dth component of glyph \"%s\").",
                        nptrs[i], i, glyphname);
    }

    if (!error) {
        if (suffix &&
            (streq_ptr(suffix, "liga") || streq_ptr(suffix, "dlig") ||
             streq_ptr(suffix, "hlig") || streq_ptr(suffix, "frac") ||
             streq_ptr(suffix, "ccmp") || streq_ptr(suffix, "afrc")
                )
            ) {
            error = composeglyph(gids, n_comp, suffix, gm, gid);
        } else { /* first try composing glyph */
            error = composeglyph(gids, n_comp, NULL, gm, gid);
            if (!error && suffix) /* a_b_c.vert */
                error = selectglyph(*gid, suffix, gm, gid);
        }
    }
    free(gname);

    return  error;
}

/* glyphname should not have suffix here */
static int
findparanoiac (const char *glyphname, USHORT *gid, struct glyph_mapper *gm)
{
    agl_name  *agln;
    USHORT     idx   = 0U;
    int        error = 0;

    agln = agl_lookup_list(glyphname);
    while (agln && idx == 0) {
        if (agln->suffix) {
            error = findparanoiac(agln->name, &idx, gm);
            if (error)
                return error;

            error = selectglyph(idx, agln->suffix, gm, &idx);
            if (error) {
                dpx_warning("Variant \"%s\" for glyph \"%s\" might not be found.",
                            agln->suffix, agln->name);
                dpx_warning("Using glyph name without suffix instead...");
                error = 0; /* ignore */
            }
        } else {
            if (agln->n_components == 1)
                idx = tt_cmap_lookup(gm->codetogid, agln->unicodes[0]);
            else if (agln->n_components > 1) {
                if (dpx_conf.verbose_level >= 0) /* give warning */
                    dpx_warning("Glyph \"%s\" looks like a composite glyph...",
                                agln->name);
                error = composeuchar(agln->unicodes, agln->n_components, NULL, gm, &idx);
                if (dpx_conf.verbose_level >= 0) {
                    if (error)
                        dpx_warning("Not found...");
                    else {
                        int   _i, _n = 0;
                        char *_p, _buf[256];
                        dpx_warning(">> Composite glyph glyph-name=\"%s\" found at glyph-id=\"%u\".",
                                    agln->name, idx);
                        for (_p = _buf, _i = 0; _i < agln->n_components && _n < 245; _i++) {
                            _p[_n++] = _i == 0 ? '<' : ' ';
                            if (agln->unicodes[_i] >= 0x10000)
                                _n += sprintf(_p+_n, "U+%06X", agln->unicodes[_i]);
                            else
                                _n += sprintf(_p+_n, "U+%04X", agln->unicodes[_i]);
                            _p[_n++] = _i == agln->n_components - 1 ? '>' : ',';
                        }
                        _p[_n++] = '\0';
                        dpx_warning(">> Input Unicode seq.=\"%s\" ==> glyph-id=\"%u\" in font-file=\"_please_try_-v_\".", _buf, idx);
                    }
                }
            } else assert(0); /* Boooo */
        }
        agln = agln->alternate;
    }

    *gid = idx;
    return (idx == 0 ? -1 : 0);
}

static int
resolve_glyph (const char *glyphname, USHORT *gid, struct glyph_mapper *gm)
{
    int    error = 0;
    char  *name, *suffix = NULL;
    int32_t ucv;

    assert(glyphname);

    /*
     * First we try glyph name to GID mapping using post table if post table
     * is available. If post table is not available or glyph is not listed
     * in the post table, then we try Unicode if Windows-Unicode TrueType
     * cmap is available.
     */
    error = findposttable(glyphname, gid, gm);
    if (!error)
        return  0;

    if (!gm->codetogid)
        return  -1;

    name = agl_chop_suffix(glyphname, &suffix);
    if (!name) /* .notdef, .foo */
        error = -1;
    else if (agl_name_is_unicode(name)) {
        ucv  = agl_name_convert_unicode(name);
        *gid = tt_cmap_lookup(gm->codetogid, ucv);
        error = (*gid == 0) ? -1 : 0;
    } else {
        error = findparanoiac(name, gid, gm);
    }
    if (!error && suffix) {
        error = selectglyph(*gid, suffix, gm, gid);
        if (error) {
            dpx_warning("Variant \"%s\" for glyph \"%s\" might not be found.",
                        suffix, name);
            dpx_warning("Using glyph name without suffix instead...");
            error = 0; /* ignore */
        }
    }
    free(suffix);
    free(name);

    return  error;
}

/* Things are complicated. We still need to use PostScript
 * glyph names. But OpenType fonts may not have PS name to
 * glyph mapping. We use Unicode plus OTL GSUB for finding
 * glyphs in this case.
 */
static int
setup_glyph_mapper (struct glyph_mapper *gm, sfnt *sfont)
{
    gm->sfont     = sfont;
    gm->nametogid = tt_read_post_table(sfont);
    gm->codetogid = tt_cmap_read(sfont, TT_WIN, TT_WIN_UCS4);
    if (!gm->codetogid)
        gm->codetogid = tt_cmap_read(sfont, TT_WIN, TT_WIN_UNICODE);

    if (!gm->nametogid && !gm->codetogid)
        return -1;

    gm->gsub = otl_gsub_new();

    return 0;
}

static void
clean_glyph_mapper (struct glyph_mapper *gm)
{
    if (gm->gsub)
        otl_gsub_release(gm->gsub);
    if (gm->codetogid)
        tt_cmap_release (gm->codetogid);
    if (gm->nametogid)
        tt_release_post_table(gm->nametogid);

    gm->gsub = NULL;
    gm->codetogid = NULL;
    gm->nametogid = NULL;
    gm->sfont = NULL;

    return;
}

static int
do_custom_encoding (pdf_font *font,
                    char **encoding, const char *usedchars, sfnt *sfont)
{
    struct tt_glyphs      *glyphs;
    char                  *cmap_table;
    int                    code, count;
    double                 widths[256];
    struct glyph_mapper    gm;
    USHORT                 idx, gid;
    int                    error = 0;

    assert(font && encoding && usedchars && sfont);

    error = setup_glyph_mapper(&gm, sfont);
    if (error) {
        dpx_warning("No post table nor Unicode cmap found in font: %s",
                    pdf_font_get_ident(font));
        dpx_warning(">> I can't find glyphs without this!");
        return  -1;
    }

    cmap_table = NEW(274, char);
    memset(cmap_table, 0, 274);
    sfnt_put_ushort(cmap_table,    0);            /* Version  */
    sfnt_put_ushort(cmap_table+2,  1);            /* Number of subtables */
    sfnt_put_ushort(cmap_table+4,  TT_MAC);       /* Platform ID */
    sfnt_put_ushort(cmap_table+6,  TT_MAC_ROMAN); /* Encoding ID */
    sfnt_put_ulong (cmap_table+8,  12);           /* Offset   */
    sfnt_put_ushort(cmap_table+12, 0);            /* Format   */
    sfnt_put_ushort(cmap_table+14, 262);          /* Length   */
    sfnt_put_ushort(cmap_table+16, 0);            /* Language */

    glyphs = tt_build_init();

    count = 1; /* +1 for .notdef */
    for (code = 0; code < 256; code++) {
        if (!usedchars[code])
            continue;

        if (!encoding[code] || streq_ptr(encoding[code], ".notdef")) {
            dpx_warning("Character code=\"0x%02X\" mapped to \".notdef\" glyph used in font font-file=\"%s\"",
                        code, pdf_font_get_ident(font));
            dpx_warning(">> Maybe incorrect encoding specified?");
            idx = 0;
        } else {
            if (is_comp(encoding[code]))
                error = findcomposite(encoding[code], &gid, &gm);
            else
                error = resolve_glyph(encoding[code], &gid, &gm);

            /*
             * Older versions of gs had problem with glyphs (other than .notdef)
             * mapped to gid = 0.
             */
            if (error) {
                dpx_warning("Glyph \"%s\" not available in font \"%s\".",
                            encoding[code], pdf_font_get_ident(font));
            } else {
                if (dpx_conf.verbose_level > 1)
                    dpx_message("truetype>> Glyph glyph-name=\"%s\" found at glyph-id=\"%u\".\n", encoding[code], gid);
            }
            idx = tt_find_glyph(glyphs, gid);
            if (idx == 0) {
                idx = tt_add_glyph(glyphs, (USHORT)gid, (USHORT)count); /* count returned. */
                count++;
            }
        }
        cmap_table[18 + code] = idx & 0xff; /* bug here */
    }
    clean_glyph_mapper(&gm);

    if (tt_build_tables(sfont, glyphs) < 0) {
        dpx_warning("Packing TrueType font into SFNT file faild..."); /* _FIXME_: wrong message */
        tt_build_finish(glyphs);
        free(cmap_table);
        return  -1;
    }

    for (code = 0; code < 256; code++) {
        if (usedchars[code]) {
            idx = tt_get_index(glyphs, (USHORT) cmap_table[18+code]);
            widths[code] = PDFUNIT(glyphs->gd[idx].advw);
        } else {
            widths[code] = 0.0;
        }
    }
    do_widths(font, widths);

    if (dpx_conf.verbose_level > 1)
        dpx_message("[%d glyphs]", glyphs->num_glyphs);

    tt_build_finish(glyphs);

    sfnt_set_table(sfont, "cmap", cmap_table, 274);

    return  0;
}

int
pdf_font_load_truetype (pdf_font *font)
{
    pdf_obj   *descriptor  = pdf_font_get_descriptor(font);
    char      *ident       = pdf_font_get_ident(font);
    int        encoding_id = pdf_font_get_encoding(font);
    char      *usedchars   = pdf_font_get_usedchars(font);
#ifdef  ENABLE_NOEMBED
    int        embedding   = pdf_font_get_flag(font, PDF_FONT_FLAG_NOEMBED) ? 0 : 1;
#endif /* ENABLE_NOEMBED */
    int        index       = pdf_font_get_index(font);
    char     **enc_vec;
    pdf_obj   *fontfile;
    rust_input_handle_t handle = NULL;
    sfnt      *sfont;
    int        i, error = 0;

    if (!pdf_font_is_in_use(font))
        return  0;

    handle = dpx_open_truetype_file(ident);
    if (handle == NULL) {
        handle = dpx_open_dfont_file(ident);
        if (handle == NULL)
            _tt_abort("Unable to open TrueType/dfont font file: %s", ident); /* Should find *truetype* here */

        sfont = dfont_open(handle, index);
    } else {
        sfont = sfnt_open(handle);
    }

    if (!sfont) {
        ttstub_input_close(handle);
        _tt_abort("Unable to open TrueType/dfont file: %s", ident);
    } else if (sfont->type != SFNT_TYPE_TRUETYPE &&
               sfont->type != SFNT_TYPE_TTC &&
               sfont->type != SFNT_TYPE_DFONT) {
        sfnt_close(sfont);
        ttstub_input_close(handle);
        _tt_abort("Font \"%s\" not a TrueType/dfont font?", ident);
    }

    if (sfont->type == SFNT_TYPE_TTC) {
        ULONG offset;
        offset = ttc_read_offset(sfont, index);
        if (offset == 0) _tt_abort("Invalid TTC index in %s.", ident);
        error = sfnt_read_table_directory(sfont, offset);
    } else {
        error = sfnt_read_table_directory(sfont, sfont->offset);
    }

    if (error) {
        sfnt_close(sfont);
        ttstub_input_close(handle);
        _tt_abort("Reading SFND table dir failed for font-file=\"%s\"... Not a TrueType font?", ident);
    }

    /*
     * Create new TrueType cmap table with MacRoman encoding.
     */
    if (encoding_id < 0)
        error = do_builtin_encoding(font, usedchars, sfont);
    else {
        enc_vec  = pdf_encoding_get_encoding(encoding_id);
        error = do_custom_encoding(font, enc_vec, usedchars, sfont);
    }
    if (error) {
        sfnt_close(sfont);
        ttstub_input_close(handle);
        _tt_abort("Error occured while creating font subfont for \"%s\"", ident);
    }

#ifdef  ENABLE_NOEMBED
    if (!embedding) {
        sfnt_close(sfont);
        ttstub_input_close(handle);
        return  0;
    }
#endif /* ENABLE_NOEMBED */

    /*
     * TODO: post table?
     */

    for (i = 0; required_table[i].name != NULL; i++) {
        if (sfnt_require_table(sfont,
                               required_table[i].name,
                               required_table[i].must_exist) < 0) {
            sfnt_close(sfont);
            ttstub_input_close(handle);
            _tt_abort("Required TrueType table \"%s\" does not exist in font: %s",
                      required_table[i].name, ident);
        }
    }

    /*
     * FontFile2
     */
    fontfile = sfnt_create_FontFile_stream(sfont);
    if (!fontfile)
        _tt_abort("Could not created FontFile stream for \"%s\".", ident);

    sfnt_close(sfont);
    ttstub_input_close(handle);

    if (dpx_conf.verbose_level > 1)
        dpx_message("[%d bytes]", pdf_stream_length(fontfile));

    pdf_add_dict(descriptor,
                 pdf_new_name("FontFile2"), pdf_ref_obj(fontfile)); /* XXX */
    pdf_release_obj(fontfile);

    return  0;
}
