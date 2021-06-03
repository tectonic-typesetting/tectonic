/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2008-2018 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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

#include "dpx-type1.h"

#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_limits.h"
#include "dpx-cff_types.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfencoding.h"
#include "dpx-pdffont.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"
#include "dpx-t1_char.h"
#include "dpx-t1_load.h"
#include "dpx-tfm.h"

#define FONT_FLAG_FIXEDPITCH (1 << 0)  /* Fixed-width font */
#define FONT_FLAG_SERIF      (1 << 1)  /* Serif font */
#define FONT_FLAG_SYMBOLIC   (1 << 2)  /* Symbolic font */
#define FONT_FLAG_SCRIPT     (1 << 3)  /* Script font */
#define FONT_FLAG_STANDARD   (1 << 5)  /* Adobe Standard Character Set */
#define FONT_FLAG_ITALIC     (1 << 6)  /* Italic */
#define FONT_FLAG_ALLCAP     (1 << 16) /* All-cap font */
#define FONT_FLAG_SMALLCAP   (1 << 17) /* Small-cap font */
#define FONT_FLAG_FORCEBOLD  (1 << 18) /* Force bold at small text sizes */

static bool
is_basefont (const char *name)
{
    static const char *basefonts[] = {
        "Courier",                  "Courier-Bold",          "Courier-Oblique",
        "Courier-BoldOblique",      "Helvetica",             "Helvetica-Bold",
        "Helvetica-Oblique",        "Helvetica-BoldOblique", "Symbol",
        "Times-Roman",              "Times-Bold",            "Times-Italic",
        "Times-BoldItalic",         "ZapfDingbats"
    };
    int i;

    for (i = 0; i < 14; i++) {
        if (streq_ptr(name, basefonts[i]))
            return true;
    }

    return false;
}


int
pdf_font_open_type1 (pdf_font *font)
{
    char *ident;
    char fontname[PDF_NAME_LEN_MAX+1];

    assert(font);

    ident = pdf_font_get_ident(font);

    if (is_basefont(ident)) {
        pdf_font_set_fontname(font, ident);
        pdf_font_set_subtype(font, PDF_FONT_FONTTYPE_TYPE1);
        pdf_font_set_flags(font, PDF_FONT_FLAG_NOEMBED|PDF_FONT_FLAG_BASEFONT);
    } else {
        rust_input_handle_t handle;

        handle = ttstub_input_open(ident, TTBC_FILE_FORMAT_TYPE1, 0);
        /* NOTE: skipping qcheck_filetype() call in dpx_find_type1_file but we
         * call is_pfb() in just a second anyway.
         */
        if (handle == NULL)
            return -1;

        memset(fontname, 0, PDF_NAME_LEN_MAX+1);
        if (!is_pfb(handle) || t1_get_fontname(handle, fontname) < 0)
            _tt_abort("Failed to read Type 1 font \"%s\".", ident);

        ttstub_input_close(handle);
        pdf_font_set_fontname(font, fontname);
        pdf_font_set_subtype (font, PDF_FONT_FONTTYPE_TYPE1);
    }

    return 0;
}

static void
get_font_attr (pdf_font *font, cff_font *cffont)
{
    char    *fontname;
    pdf_obj *descriptor;
    double   capheight, ascent, descent;
    double   italicangle, stemv;
    double   defaultwidth, nominalwidth;
    int      flags = 0, gid, i;
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

    fontname   = pdf_font_get_fontname  (font);
    descriptor = pdf_font_get_descriptor(font);

    if (fontname && !strstr(fontname, "Sans")) {
        flags |= FONT_FLAG_SERIF;
    }
    if (fontname &&  strstr(fontname, "Caps")) {
        flags |= FONT_FLAG_SMALLCAP;
    }
    flags |= FONT_FLAG_SYMBOLIC; /* FIXME */

    pdf_add_dict(descriptor,
                 pdf_new_name("CapHeight"), pdf_new_number(capheight));
    pdf_add_dict(descriptor,
                 pdf_new_name("Ascent"), pdf_new_number(ascent));
    pdf_add_dict(descriptor,
                 pdf_new_name("Descent"), pdf_new_number(descent));
    pdf_add_dict(descriptor,
                 pdf_new_name("ItalicAngle"), pdf_new_number(italicangle));
    pdf_add_dict(descriptor,
                 pdf_new_name("StemV"), pdf_new_number(stemv));
    pdf_add_dict(descriptor,
                 pdf_new_name("Flags"), pdf_new_number(flags));
}

static void
add_metrics (pdf_font *font, cff_font *cffont, char **enc_vec, double *widths, int num_glyphs)
{
    pdf_obj *fontdict, *descriptor;
    pdf_obj *tmp_array;
    int      code, firstchar, lastchar;
    double   val;
    int      i, tfm_id;
    char    *usedchars;
    double   scaling;

    fontdict   = pdf_font_get_resource  (font);
    descriptor = pdf_font_get_descriptor(font);
    usedchars  = pdf_font_get_usedchars (font);

    /*
     * The original FontBBox of the font is preserved, instead
     * of replacing it with tight bounding box calculated from
     * charstrings, to prevent Acrobat 4 from greeking text as
     * much as possible.
     */
    if (!cff_dict_known(cffont->topdict, "FontBBox")) {
        _tt_abort("No FontBBox?");
    }

    /* The widhts array in the font dictionary must be given relative
     * to the default scaling of 1000:1, not relative to the scaling
     * given by the font matrix.
     */
    if (cff_dict_known(cffont->topdict, "FontMatrix"))
        scaling = 1000*cff_dict_get(cffont->topdict, "FontMatrix", 0);
    else
        scaling = 1;

    tmp_array = pdf_new_array();
    for (i = 0; i < 4; i++) {
        val = cff_dict_get(cffont->topdict, "FontBBox", i);
        pdf_add_array(tmp_array, pdf_new_number(ROUND(val, 1.0)));
    }
    pdf_add_dict(descriptor, pdf_new_name("FontBBox"), tmp_array);

    tmp_array = pdf_new_array();
    if (num_glyphs <= 1) { /* This must be an error. */
        firstchar = lastchar = 0;
        pdf_add_array(tmp_array, pdf_new_number(0.0));
    } else {
        for (firstchar = 255, lastchar = 0, code = 0; code < 256; code++) {
            if (usedchars[code]) {
                if (code < firstchar) firstchar = code;
                if (code > lastchar)  lastchar  = code;
            }
        }
        if (firstchar > lastchar) {
            dpx_warning("No glyphs actually used???");
            pdf_release_obj(tmp_array);
            return;
        }

        /* PLEASE FIX THIS
         * It's wrong to use TFM width here... We should warn if TFM width
         * and actual glyph width are different.
         */

        tfm_id = tfm_open(pdf_font_get_mapname(font), 0);
        for (code = firstchar; code <= lastchar; code++) {
            if (usedchars[code]) {
                double width;
                if (tfm_id < 0) /* tfm is not found */
                    width = scaling * widths[cff_glyph_lookup(cffont, enc_vec[code])];
                else {
                    double diff;
                    width = 1000.0 * tfm_get_width(tfm_id, code);
                    diff  = width -
                        scaling * widths[cff_glyph_lookup(cffont, enc_vec[code])];
                    if (fabs(diff) > 1.0) {
                        dpx_warning("Glyph width mismatch for TFM and font (%s)",
                             pdf_font_get_mapname(font));
                        dpx_warning("TFM: %g vs. Type1 font: %g",
                             width, widths[cff_glyph_lookup(cffont, enc_vec[code])]);
                    }
                }
                pdf_add_array(tmp_array,
                              pdf_new_number(ROUND(width, 0.1)));
            } else {
                pdf_add_array(tmp_array, pdf_new_number(0.0));
            }
        }
    }

    if (pdf_array_length(tmp_array) > 0) {
        pdf_add_dict(fontdict,
                     pdf_new_name("Widths"),  pdf_ref_obj(tmp_array));
    }
    pdf_release_obj(tmp_array);

    pdf_add_dict(fontdict,
                 pdf_new_name("FirstChar"), pdf_new_number(firstchar));
    pdf_add_dict(fontdict,
                 pdf_new_name("LastChar"),  pdf_new_number(lastchar));

    return;
}


static int
write_fontfile (pdf_font *font, cff_font *cffont, pdf_obj *pdfcharset)
{
    pdf_obj   *descriptor;
    pdf_obj   *fontfile, *stream_dict;
    cff_index *topdict;
    int        private_size, stream_data_len, charstring_len;
    int        topdict_offset, offset;
#define  WBUF_SIZE 1024
    card8     *stream_data_ptr, wbuf[WBUF_SIZE];

    descriptor = pdf_font_get_descriptor(font);

    topdict = cff_new_index(1);
    /*
     * Force existence of Encoding.
     */
    if (!cff_dict_known(cffont->topdict, "CharStrings"))
        cff_dict_add(cffont->topdict, "CharStrings", 1);
    if (!cff_dict_known(cffont->topdict, "charset"))
        cff_dict_add(cffont->topdict, "charset", 1);
    if (!cff_dict_known(cffont->topdict, "Encoding"))
        cff_dict_add(cffont->topdict, "Encoding", 1);
    private_size = cff_dict_pack((cffont->private)[0], wbuf, WBUF_SIZE);
    /* Private dict is required (but may have size 0) */
    if (!cff_dict_known(cffont->topdict, "Private"))
        cff_dict_add(cffont->topdict, "Private", 2);
    topdict->offset[1] = cff_dict_pack(cffont->topdict, wbuf, WBUF_SIZE) + 1;

    /*
     * Estimate total size of fontfile.
     */
    charstring_len = cff_index_size(cffont->cstrings);

    stream_data_len = 4; /* header size */
    stream_data_len += cff_index_size(cffont->name);
    stream_data_len += cff_index_size(topdict);
    stream_data_len += cff_index_size(cffont->string);
    stream_data_len += cff_index_size(cffont->gsubr);
    /* We are using format 1 for Encoding and format 0 for charset.
     * TODO: Should implement cff_xxx_size().
     */
    stream_data_len += 2 + (cffont->encoding->num_entries)*2 + 1 + (cffont->encoding->num_supps)*3;
    stream_data_len += 1 + (cffont->charsets->num_entries)*2;
    stream_data_len += charstring_len;
    stream_data_len += private_size;

    /*
     * Now we create FontFile data.
     */
    stream_data_ptr = NEW(stream_data_len, card8);
    /*
     * Data Layout order as described in CFF spec., sec 2 "Data Layout".
     */
    offset = 0;
    /* Header */
    offset += cff_put_header(cffont,
                             stream_data_ptr + offset, stream_data_len - offset);
    /* Name */
    offset += cff_pack_index(cffont->name,
                             stream_data_ptr + offset, stream_data_len - offset);
    /* Top DICT */
    topdict_offset = offset;
    offset += cff_index_size(topdict);
    /* Strings */
    offset += cff_pack_index(cffont->string,
                             stream_data_ptr + offset, stream_data_len - offset);
    /* Global Subrs */
    offset += cff_pack_index(cffont->gsubr,
                             stream_data_ptr + offset, stream_data_len - offset);
    /* Encoding */
    /* TODO: don't write Encoding entry if the font is always used
     * with PDF Encoding information. Applies to type1c.c as well.
     */
    cff_dict_set(cffont->topdict, "Encoding", 0, offset);
    offset += cff_pack_encoding(cffont,
                                stream_data_ptr + offset, stream_data_len - offset);
    /* charset */
    cff_dict_set(cffont->topdict, "charset", 0, offset);
    offset += cff_pack_charsets(cffont,
                                stream_data_ptr + offset, stream_data_len - offset);
    /* CharStrings */
    cff_dict_set(cffont->topdict, "CharStrings", 0, offset);
    offset += cff_pack_index(cffont->cstrings,
                             stream_data_ptr + offset, charstring_len);
    /* Private */
    if ((cffont->private)[0] && private_size > 0) {
        private_size = cff_dict_pack(cffont->private[0],
                                     stream_data_ptr + offset, private_size);
        cff_dict_set(cffont->topdict, "Private", 1, offset);
        cff_dict_set(cffont->topdict, "Private", 0, private_size);
    }
    offset += private_size;

    /* Finally Top DICT */
    topdict->data = NEW(topdict->offset[1] - 1, card8);
    cff_dict_pack (cffont->topdict, topdict->data, topdict->offset[1] - 1);
    cff_pack_index(topdict,
                   stream_data_ptr + topdict_offset, cff_index_size(topdict));
    cff_release_index(topdict);

    /* Copyright and Trademark Notice ommited. */

    /* Flush Font File */
    fontfile    = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(fontfile);
    pdf_add_dict(descriptor,
                 pdf_new_name("FontFile3"), pdf_ref_obj (fontfile));
    pdf_add_dict(stream_dict,
                 pdf_new_name("Subtype"),   pdf_new_name("Type1C"));
    pdf_add_stream (fontfile, (void *) stream_data_ptr,  offset);
    pdf_release_obj(fontfile);
    pdf_add_dict(descriptor,
                 pdf_new_name("CharSet"),
                 pdf_new_string(pdf_stream_dataptr(pdfcharset),
                                pdf_stream_length(pdfcharset)));
    free(stream_data_ptr);

    return offset;
}


int
pdf_font_load_type1 (pdf_font *font)
{
    pdf_obj      *fontdict;
    pdf_obj      *pdfcharset; /* Actually string object */
    int           encoding_id;
    char         *usedchars, *ident;
    char         *fontname, *uniqueTag;
    char         *fullname; /* With pseudo unique tag */
    cff_font     *cffont;
    cff_charsets *charset;
    char        **enc_vec;
    double        defaultwidth, nominalwidth;
    double       *widths;
    card16       *GIDMap, num_glyphs = 0;
    int           offset;
    int           code;
    rust_input_handle_t handle;

    assert(font);

    if (!pdf_font_is_in_use(font))
        return 0;

    encoding_id = pdf_font_get_encoding  (font);
    fontdict    = pdf_font_get_resource  (font);

    pdf_font_get_descriptor(font);
    usedchars   = pdf_font_get_usedchars (font);
    ident       = pdf_font_get_ident     (font);
    fontname    = pdf_font_get_fontname  (font);
    uniqueTag   = pdf_font_get_uniqueTag (font);

    if (!usedchars || !ident || !fontname)
        _tt_abort("Type1: Unexpected error.");

    handle = ttstub_input_open(ident, TTBC_FILE_FORMAT_TYPE1, 0);
    if (handle == NULL)
        _tt_abort("Type1: Could not open Type1 font: %s", ident);

    GIDMap     = NULL;
    num_glyphs = 0;

    if (encoding_id >= 0) {
        enc_vec = NULL;
    } else {
        enc_vec = NEW(256, char *);
        for (code = 0; code <= 0xFF; code++)
            enc_vec[code] = NULL;
    }

    cffont = t1_load_font(enc_vec, 0, handle);
    if (!cffont)
        _tt_abort("Could not load Type 1 font: %s", ident);

    ttstub_input_close(handle);

    fullname = NEW(strlen(fontname) + 8, char);
    sprintf(fullname, "%6s+%s", uniqueTag, fontname);

    /* Encoding related things. */
    if (encoding_id >= 0) {
        enc_vec = pdf_encoding_get_encoding(encoding_id);
    } else {
        /* Create enc_vec and ToUnicode CMap for built-in encoding. */
        pdf_obj *tounicode;

        if (!pdf_lookup_dict(fontdict, "ToUnicode")) {
            tounicode = pdf_create_ToUnicode_CMap(fullname, enc_vec, usedchars);
            if (tounicode) {
                pdf_add_dict(fontdict, pdf_new_name("ToUnicode"), pdf_ref_obj (tounicode));
                pdf_release_obj(tounicode);
            }
        }
    }

    cff_set_name(cffont, fullname);
    free(fullname);

    /* defaultWidthX, CapHeight, etc. */
    get_font_attr(font, cffont);

    if (cff_dict_known(cffont->private[0], "defaultWidthX"))
        defaultwidth = cff_dict_get(cffont->private[0], "defaultWidthX", 0);
    else
        defaultwidth = 0.0;

    if (cff_dict_known(cffont->private[0], "nominalWidthX"))
        nominalwidth = cff_dict_get(cffont->private[0], "nominalWidthX", 0);
    else
        nominalwidth = 0.0;

    /* Create CFF encoding, charset, sort glyphs */
#define MAX_GLYPHS 1024
    GIDMap = NEW(MAX_GLYPHS, card16);
    pdfcharset = pdf_new_stream(0);

    {
        int     prev, duplicate;
        int     gid;
        char   *glyph;
        s_SID   sid;

        cffont->encoding = NEW(1, cff_encoding);
        cffont->encoding->format      = 1;
        cffont->encoding->num_entries = 0;
        cffont->encoding->data.range1 = NEW(256, cff_range1);
        cffont->encoding->num_supps   = 0;
        cffont->encoding->supp        = NEW(256, cff_map);

        charset = NEW(1, cff_charsets);
        charset->format      = 0;
        charset->num_entries = 0;
        charset->data.glyphs = NEW(MAX_GLYPHS, s_SID);

        gid = cff_glyph_lookup(cffont, ".notdef");
        if (gid < 0)
            _tt_abort("Type 1 font with no \".notdef\" glyph???");
        GIDMap[0] = (card16) gid;
        if (dpx_conf.verbose_level > 2)
            dpx_message("[glyphs:/.notdef");
        num_glyphs = 1;

        for (prev = -2, code = 0; code <= 0xff; code++) {
            glyph = enc_vec[code];

            if (!usedchars[code])
                continue;

            if (streq_ptr(glyph, ".notdef")) {
                dpx_warning("Character mapped to .notdef used in font: %s", fontname);
                usedchars[code] = 0;
                continue;
            }

            gid = cff_glyph_lookup(cffont, glyph);
            if (gid < 1 || gid >= cffont->cstrings->count) {
                dpx_warning("Glyph \"%s\" missing in font \"%s\".", glyph, fontname);
                usedchars[code] = 0;
                continue;
            }

            for (duplicate = 0; duplicate < code; duplicate++) {
                if (usedchars[duplicate] && enc_vec[duplicate] && streq_ptr(enc_vec[duplicate], glyph))
                    break;
            }

            sid = cff_add_string(cffont, glyph, 1); /* FIXME */

            if (duplicate < code) { /* found duplicates */
                cffont->encoding->supp[cffont->encoding->num_supps].code  = duplicate;
                cffont->encoding->supp[cffont->encoding->num_supps].glyph = sid;
                cffont->encoding->num_supps += 1;
            } else {
                GIDMap[num_glyphs] = (card16) gid;
                charset->data.glyphs[charset->num_entries] = sid;
                charset->num_entries += 1;

                if (code != prev + 1) {
                    cffont->encoding->num_entries += 1;
                    cffont->encoding->data.range1[cffont->encoding->num_entries-1].first  = code;
                    cffont->encoding->data.range1[cffont->encoding->num_entries-1].n_left = 0;
                } else {
                    cffont->encoding->data.range1[cffont->encoding->num_entries-1].n_left += 1;
                }

                prev = code;
                num_glyphs++;

                if (dpx_conf.verbose_level > 2)
                    dpx_message("/%s", glyph);

                /* CharSet is actually string object. */
                pdf_add_stream(pdfcharset, "/", 1);
                pdf_add_stream(pdfcharset, glyph, strlen(glyph));
            }
        }

        if (cffont->encoding->num_supps > 0) {
            cffont->encoding->format |= 0x80;
        } else {
            cffont->encoding->supp = mfree(cffont->encoding->supp);
        }
    }

    widths = NEW(cffont->cstrings->count, double);

    /* No more strings will be added. The Type 1 seac operator may add another
     * glyph but the glyph name of those glyphs are contained in standard
     * string. The String Index will not be modified after here. BUT: We
     * cannot update the String Index yet because then we wouldn't be able to
     * find the GIDs of the base and accent characters (unless they have been
     * used already).
     */

    {
        cff_index *cstring;
        t1_ginfo   gm;
        card16     gid, gid_orig;
        int        dstlen_max, srclen;
        card8     *srcptr, *dstptr;

        offset  = dstlen_max = 0L;
        cstring = cff_new_index(cffont->cstrings->count);
        cstring->data      = NULL;
        cstring->offset[0] = 1;

        /* The num_glyphs increases if "seac" operators are used. */
        for (gid = 0; gid < num_glyphs; gid++) {
            if (offset + CS_STR_LEN_MAX >= dstlen_max) {
                dstlen_max += CS_STR_LEN_MAX * 2;
                cstring->data = RENEW(cstring->data, dstlen_max, card8);
            }

            gid_orig = GIDMap[gid];

            dstptr   = cstring->data + cstring->offset[gid] - 1;
            srcptr   = cffont->cstrings->data + cffont->cstrings->offset[gid_orig] - 1;
            srclen   = cffont->cstrings->offset[gid_orig + 1] - cffont->cstrings->offset[gid_orig];

            offset  += t1char_convert_charstring(dstptr, CS_STR_LEN_MAX,
                                                 srcptr, srclen,
                                                 cffont->subrs[0], defaultwidth, nominalwidth, &gm);
            cstring->offset[gid + 1] = offset + 1;

            if (gm.use_seac) {
                int  bchar_gid, achar_gid, i;
                const char *bchar_name, *achar_name;

                /*
                 * NOTE:
                 *  1. seac.achar and seac.bchar must be contained in the CFF standard string.
                 *  2. Those characters need not to be encoded.
                 *  3. num_glyphs == charsets->num_entries + 1.
                 */
                achar_name = t1_get_standard_glyph(gm.seac.achar);
                achar_gid  = cff_glyph_lookup(cffont, achar_name);
                bchar_name = t1_get_standard_glyph(gm.seac.bchar);
                bchar_gid  = cff_glyph_lookup(cffont, bchar_name);

                if (achar_gid < 0) {
                    dpx_warning("Accent char \"%s\" not found. Invalid use of \"seac\" operator.",
                         achar_name);
                    continue;
                }

                if (bchar_gid < 0) {
                    dpx_warning("Base char \"%s\" not found. Invalid use of \"seac\" operator.",
                         bchar_name);
                    continue;
                }

                for (i = 0; i < num_glyphs; i++) {
                    if (GIDMap[i] == achar_gid)
                        break;
                }

                if (i == num_glyphs) {
                    if (dpx_conf.verbose_level > 2)
                        dpx_message("/%s", achar_name);
                    GIDMap[num_glyphs++] = achar_gid;
                    charset->data.glyphs[charset->num_entries] = cff_get_seac_sid(cffont, achar_name);
                    charset->num_entries += 1;
                    /* CharSet is actually string object. */
                    {
                        pdf_add_stream(pdfcharset, "/", 1);
                        pdf_add_stream(pdfcharset, achar_name, strlen(achar_name));
                    }
                }

                for (i = 0; i < num_glyphs; i++) {
                    if (GIDMap[i] == bchar_gid)
                        break;
                }
                if (i == num_glyphs) {
                    if (dpx_conf.verbose_level > 2)
                        dpx_message("/%s", bchar_name);
                    GIDMap[num_glyphs++] = bchar_gid;
                    charset->data.glyphs[charset->num_entries] = cff_get_seac_sid(cffont, bchar_name);
                    charset->num_entries += 1;
                    /* CharSet is actually string object. */
                    {
                        pdf_add_stream(pdfcharset, "/", 1);
                        pdf_add_stream(pdfcharset, achar_name, strlen(achar_name));
                    }
                }
            }
            widths[gid] = gm.wx;
        }
        cstring->count = num_glyphs;

        cff_release_index(cffont->subrs[0]);
        cffont->subrs[0] = NULL;
        cffont->subrs = mfree(cffont->subrs);

        cff_release_index(cffont->cstrings);
        cffont->cstrings = cstring;

        cff_release_charsets(cffont->charsets);
        cffont->charsets = charset;
    }

    if (dpx_conf.verbose_level > 2)
        dpx_message("]");

    /* Now we can update the String Index */
    cff_dict_update  (cffont->topdict,    cffont);
    cff_dict_update  (cffont->private[0], cffont);
    cff_update_string(cffont);

    add_metrics(font, cffont, enc_vec, widths, num_glyphs);

    offset = write_fontfile(font, cffont, pdfcharset);
    if (dpx_conf.verbose_level > 1)
        dpx_message("[%u glyphs][%d bytes]", num_glyphs, offset);

    pdf_release_obj(pdfcharset);
    cff_close(cffont);

    /* Cleanup */
    if (encoding_id < 0 && enc_vec) {
        for (code = 0; code < 256; code++) {
            enc_vec[code] = mfree(enc_vec[code]);
        }
        free(enc_vec);
    }

    free(widths);
    free(GIDMap);

    return 0;
}
