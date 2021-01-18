/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include "dpx-fontmap.h"

#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-subfont.h"

/* CIDFont */
static char *strip_options (const char *map_name, fontmap_opt *opt);

void
pdf_init_fontmap_record (fontmap_rec *mrec)
{
    assert(mrec);

    mrec->map_name   = NULL;

    /* SFD char mapping */
    mrec->charmap.sfd_name   = NULL;
    mrec->charmap.subfont_id = NULL;
    /* for OFM */
    mrec->opt.mapc   = -1; /* compatibility */

    mrec->font_name  = NULL;
    mrec->enc_name   = NULL;

    mrec->opt.slant  = 0.0;
    mrec->opt.extend = 1.0;
    mrec->opt.bold   = 0.0;

    mrec->opt.flags  = 0;

    mrec->opt.design_size = -1.0;

    mrec->opt.tounicode = NULL;
    mrec->opt.otl_tags  = NULL;
    mrec->opt.index     = 0;
    mrec->opt.charcoll  = NULL;
    mrec->opt.style     = FONTMAP_STYLE_NONE;
    mrec->opt.stemv     = -1; /* not given explicitly by an option */

    mrec->opt.use_glyph_encoding = 0;
}

void
pdf_clear_fontmap_record (fontmap_rec *mrec)
{
    assert(mrec);

    free(mrec->map_name);
    free(mrec->charmap.sfd_name);
    free(mrec->charmap.subfont_id);
    free(mrec->enc_name);
    free(mrec->font_name);

    free(mrec->opt.tounicode);
    free(mrec->opt.otl_tags);
    free(mrec->opt.charcoll);
    pdf_init_fontmap_record(mrec);
}

/* strdup: just returns NULL for NULL */
static char *
mstrdup (const char *s)
{
    char  *r;
    if (!s)
        return  NULL;
    r = NEW(strlen(s) + 1, char);
    strcpy(r, s);
    return  r;
}

static void
pdf_copy_fontmap_record (fontmap_rec *dst, const fontmap_rec *src)
{
    assert( dst && src );

    dst->map_name   = mstrdup(src->map_name);

    dst->charmap.sfd_name   = mstrdup(src->charmap.sfd_name);
    dst->charmap.subfont_id = mstrdup(src->charmap.subfont_id);

    dst->font_name  = mstrdup(src->font_name);
    dst->enc_name   = mstrdup(src->enc_name);

    dst->opt.slant  = src->opt.slant;
    dst->opt.extend = src->opt.extend;
    dst->opt.bold   = src->opt.bold;

    dst->opt.flags  = src->opt.flags;
    dst->opt.mapc   = src->opt.mapc;

    dst->opt.tounicode = mstrdup(src->opt.tounicode);
    dst->opt.otl_tags  = mstrdup(src->opt.otl_tags);
    dst->opt.index     = src->opt.index;
    dst->opt.charcoll  = mstrdup(src->opt.charcoll);
    dst->opt.style     = src->opt.style;
    dst->opt.stemv     = src->opt.stemv;

    dst->opt.use_glyph_encoding = src->opt.use_glyph_encoding;
}


static void
hval_free (void *vp)
{
    fontmap_rec *mrec = (fontmap_rec *) vp;
    pdf_clear_fontmap_record(mrec);
    free(mrec);
}


static void
fill_in_defaults (fontmap_rec *mrec, const char *tex_name)
{
    if (mrec->enc_name &&
        (streq_ptr(mrec->enc_name, "default") ||
         streq_ptr(mrec->enc_name, "none"))) {
        mrec->enc_name = mfree(mrec->enc_name);
    }
    if (mrec->font_name &&
        (streq_ptr(mrec->font_name, "default") ||
         streq_ptr(mrec->font_name, "none"))) {
        mrec->font_name = mfree(mrec->font_name);
    }
    /* We *must* fill font_name either explicitly or by default */
    if (!mrec->font_name) {
        mrec->font_name = NEW(strlen(tex_name)+1, char);
        strcpy(mrec->font_name, tex_name);
    }

    mrec->map_name = NEW(strlen(tex_name)+1, char);
    strcpy(mrec->map_name, tex_name);

#ifndef WITHOUT_COMPAT
    /* Use "UCS" character collection for Unicode SFD
     * and Identity CMap combination. For backward
     * compatibility.
     */
    if (mrec->charmap.sfd_name && mrec->enc_name &&
        !mrec->opt.charcoll) {
        if ((streq_ptr(mrec->enc_name, "Identity-H") ||
             streq_ptr(mrec->enc_name, "Identity-V"))
            &&
            (strstr(mrec->charmap.sfd_name, "Uni")  ||
             strstr(mrec->charmap.sfd_name, "UBig") ||
             strstr(mrec->charmap.sfd_name, "UBg")  ||
             strstr(mrec->charmap.sfd_name, "UGB")  ||
             strstr(mrec->charmap.sfd_name, "UKS")  ||
             strstr(mrec->charmap.sfd_name, "UJIS"))) {
            mrec->opt.charcoll = NEW(strlen("UCS")+1, char);
            strcpy(mrec->opt.charcoll, "UCS");
        }
    }
#endif /* WITHOUT_COMPAT */

    return;
}

static char *
tt_readline (char *buf, int buf_len, rust_input_handle_t handle)
{
    char *p, *q;

    assert(buf && buf_len > 0 && handle);

    p = tt_mfgets(buf, buf_len, handle);
    if (!p)
        return  NULL;

    q = strchr(p, '%'); /* we don't have quoted string */
    if (q)
        *q = '\0';

    return p;
}

#ifndef ISBLANK
#  define ISBLANK(c) ((c) == ' ' || (c) == '\t')
#endif
static void
skip_blank (const char **pp, const char *endptr)
{
    const char  *p = *pp;
    if (!p || p >= endptr)
        return;
    for ( ; p < endptr && ISBLANK(*p); p++);
    *pp = p;
}

static char *
parse_string_value (const char **pp, const char *endptr)
{
    char  *q = NULL;
    const char *p = *pp;
    unsigned int n;

    if (!p || p >= endptr)
        return  NULL;
    if (*p == '"')
        q = parse_c_string(&p, endptr);
    else {
        for (n = 0; p < endptr && !isspace((unsigned char)*p); p++, n++);
        if (n == 0)
            return  NULL;
        q = NEW(n + 1, char);
        memcpy(q, *pp, n); q[n] = '\0';
    }

    *pp = p;
    return  q;
}

/* no preceeding spaces allowed */
static char *
parse_integer_value (const char **pp, const char *endptr, int base)
{
    char  *q;
    const char *p = *pp;
    int    has_sign = 0, has_prefix = 0, n;

    assert( base == 0 || (base >= 2 && base <= 36) );

    if (!p || p >= endptr)
        return  NULL;

    if (*p == '-' || *p == '+') {
        p++; has_sign = 1;
    }
    if ((base == 0 || base == 16) &&
        p + 2 <= endptr &&
        p[0] == '0' && p[1] == 'x') {
        p += 2; has_prefix = 1;
    }
    if (base == 0) {
        if (has_prefix)
            base = 16;
        else if (p < endptr && *p == '0')
            base = 8;
        else {
            base = 10;
        }
    }
#define ISDIGIT_WB(c,b) (                               \
        ((b) <= 10 && (c) >= '0' && (c) < '0' + (b)) || \
        ((b) >  10 && (                                 \
            ((c) >= '0' && (c) <= '9') ||               \
            ((c) >= 'a' && (c) < 'a' + ((b) - 10)) ||   \
            ((c) >= 'A' && (c) < 'A' + ((b) - 10))      \
            )                                           \
            )                                           \
        )
    for (n = 0; p < endptr && ISDIGIT_WB(*p, base); p++, n++);
    if (n == 0)
        return  NULL;
    if (has_sign)
        n += 1;
    if (has_prefix)
        n += 2;

    q = NEW(n + 1, char);
    memcpy(q, *pp, n); q[n] = '\0';

    *pp = p;
    return  q;
}

static int
fontmap_parse_mapdef_dpm (fontmap_rec *mrec,
                          const char *mapdef, const char *endptr)
{
    const char  *p = mapdef;

    /*
     * Parse record line in map file.  First two fields (after TeX font
     * name) are position specific.  Arguments start at the first token
     * beginning with a  '-'.
     *
     * NOTE:
     *   Dvipdfm basically uses parse_ident() for parsing enc_name,
     *   font_name, and other string values which assumes PostScript-like
     *   syntax.
     *   skip_white() skips '\r' and '\n' but they should terminate
     *   fontmap line here.
     */

    skip_blank(&p, endptr);
    /* encoding field */
    if (p < endptr && *p != '-') { /* May be NULL */
        mrec->enc_name = parse_string_value(&p, endptr);
        skip_blank(&p, endptr);
    }

    /* fontname or font filename field */
    if (p < endptr && *p != '-') { /* May be NULL */
        mrec->font_name = parse_string_value(&p, endptr);
        skip_blank(&p, endptr);
    }
    if (mrec->font_name) {
        char  *tmp;
        /* Several options are encoded in font_name for
         * compatibility with dvipdfm.
         */
        tmp = strip_options(mrec->font_name, &mrec->opt);
        if (tmp) {
            free(mrec->font_name);
            mrec->font_name = tmp;
        }
    }

    skip_blank(&p, endptr);
    /* Parse any remaining arguments */
    while (p + 1 < endptr &&
           *p != '\r' && *p != '\n' && *p == '-') {
        char  *q, mopt = p[1];
        int    v;

        p += 2; skip_blank(&p, endptr);
        switch (mopt) {

        case  's': /* Slant option */
            q = parse_float_decimal(&p, endptr);
            if (!q) {
                dpx_warning("Missing a number value for 's' option.");
                return  -1;
            }
            mrec->opt.slant = atof(q);
            free(q);
            break;

        case  'e': /* Extend option */
            q = parse_float_decimal(&p, endptr);
            if (!q) {
                dpx_warning("Missing a number value for 'e' option.");
                return  -1;
            }
            mrec->opt.extend = atof(q);
            if (mrec->opt.extend <= 0.0) {
                dpx_warning("Invalid value for 'e' option: %s", q);
                return  -1;
            }
            free(q);
            break;

        case  'b': /* Fake-bold option */
            q = parse_float_decimal(&p, endptr);
            if (!q) {
                dpx_warning("Missing a number value for 'b' option.");
                return  -1;
            }
            mrec->opt.bold = atof(q);
            if (mrec->opt.bold <= 0.0) {
                dpx_warning("Invalid value for 'b' option: %s", q);
                return  -1;
            }
            free(q);
            break;

        case  'r': /* Remap option; obsolete; just ignore */
            break;

        case  'i':  /* TTC index */
            q = parse_integer_value(&p, endptr, 10);
            if (!q) {
                dpx_warning("Missing TTC index number...");
                return  -1;
            }
            mrec->opt.index = atoi(q);
            if (mrec->opt.index < 0) {
                dpx_warning("Invalid TTC index number: %s", q);
                return  -1;
            }
            free(q);
            break;

        case  'p': /* UCS plane: just for testing */
            q = parse_integer_value(&p, endptr, 0);
            if (!q) {
                dpx_warning("Missing a number for 'p' option.");
                return  -1;
            }
            v = strtol(q, NULL, 0);
            if (v < 0 || v > 16)
                dpx_warning("Invalid value for option 'p': %s", q);
            else {
                mrec->opt.mapc = v << 16;
            }
            free(q);
            break;

        case  'u': /* ToUnicode */
            q = parse_string_value(&p, endptr);
            if (q)
                mrec->opt.tounicode = q;
            else {
                dpx_warning("Missing string value for option 'u'.");
                return  -1;
            }
            break;

        case  'v': /* StemV */
            q = parse_integer_value(&p, endptr, 10);
            if (!q) {
                dpx_warning("Missing a number for 'v' option.");
                return  -1;
            }
            mrec->opt.stemv = strtol(q, NULL, 0);
            free(q);
            break;

            /* 2017.4.15 back again */
        case 'l':
            q = parse_string_value(&p, endptr);
            if (q)
                mrec->opt.otl_tags = q;
            else {
                dpx_warning("Missing string value for option 'l'.");
                return -1;
            }
            break;

            /* Omega uses both single-byte and double-byte set_char command
             * even for double-byte OFMs. This confuses CMap decoder.
             */
        case  'm':
            /* Map single bytes char 0xab to double byte char 0xcdab  */
            if (p + 4 <= endptr &&
                p[0] == '<' && p[3] == '>') {
                p++;
                q = parse_integer_value(&p, endptr, 16);
                if (!q) {
                    dpx_warning("Invalid value for option 'm'.");
                    return  -1;
                } else if (p < endptr && *p != '>') {
                    dpx_warning("Invalid value for option 'm': %s", q);
                    free(q);
                    return  -1;
                }
                v = strtol(q, NULL, 16);
                mrec->opt.mapc = ((v << 8) & 0x0000ff00L);
                free(q); p++;
            } else if (p + 4 <= endptr &&
                       !memcmp(p, "sfd:", strlen("sfd:"))) {
                char  *r;
                const char  *rr;
                /* SFD mapping: sfd:Big5,00 */
                p += 4; skip_blank(&p, endptr);
                q  = parse_string_value(&p, endptr);
                if (!q) {
                    dpx_warning("Missing value for option 'm'.");
                    return  -1;
                }
                r  = strchr(q, ',');
                if (!r) {
                    dpx_warning("Invalid value for option 'm': %s", q);
                    free(q);
                    return  -1;
                }
                *r = 0; rr = ++r; skip_blank(&rr, r + strlen(r));
                if (*rr == '\0') {
                    dpx_warning("Invalid value for option 'm': %s,", q);
                    free(q);
                    return  -1;
                }
                mrec->charmap.sfd_name   = mstrdup(q);
                mrec->charmap.subfont_id = mstrdup(rr);
                free(q);
            } else if (p + 4 < endptr &&
                       !memcmp(p, "pad:", strlen("pad:"))) {
                p += 4; skip_blank(&p, endptr);
                q  = parse_integer_value(&p, endptr, 16);
                if (!q) {
                    dpx_warning("Invalid value for option 'm'.");
                    return  -1;
                } else if (p < endptr && !isspace((unsigned char)*p)) {
                    dpx_warning("Invalid value for option 'm': %s", q);
                    free(q);
                    return  -1;
                }
                v = strtol(q, NULL, 16);
                mrec->opt.mapc = ((v << 8) & 0x0000ff00L);
                free(q);
            } else {
                dpx_warning("Invalid value for option 'm'.");
                return  -1;
            }
            break;

        case 'w': /* Writing mode (for unicode encoding) */
            if (!mrec->enc_name ||
                strcmp(mrec->enc_name, "unicode")) {
                dpx_warning("Fontmap option 'w' meaningless for encoding other than \"unicode\".");
                return  -1;
            }
            q  = parse_integer_value(&p, endptr, 10);
            if (!q) {
                dpx_warning("Missing wmode value...");
                return  -1;
            }
            if (atoi(q) == 1)
                mrec->opt.flags |= FONTMAP_OPT_VERT;
            else if (atoi(q) == 0)
                mrec->opt.flags &= ~FONTMAP_OPT_VERT;
            else {
                dpx_warning("Invalid value for option 'w': %s", q);
            }
            free(q);
            break;

        default:
            dpx_warning("Unrecognized font map option: '%c'", mopt);
            return  -1;
        }
        skip_blank(&p, endptr);
    }

    if (p < endptr && *p != '\r' && *p != '\n') {
        dpx_warning("Invalid char in fontmap line: %c", *p);
        return  -1;
    }

    return  0;
}


/* Parse record line in map file of DVIPS/pdfTeX format. */
static int
fontmap_parse_mapdef_dps (fontmap_rec *mrec,
                          const char *mapdef, const char *endptr)
{
    const char *p = mapdef;
    char *q;

    skip_blank(&p, endptr);

    /* The first field (after TFM name) must be PostScript name. */
    /* However, pdftex.map allows a line without PostScript name. */

    if (*p != '"' && *p != '<') {
        if (p < endptr) {
            q = parse_string_value(&p, endptr);
            free(q);
            skip_blank(&p, endptr);
        } else {
            dpx_warning("Missing a PostScript font name.");
            return -1;
        }
    }

    if (p >= endptr) return 0;

    /* Parse any remaining arguments */
    while (p < endptr && *p != '\r' && *p != '\n' && (*p == '<' || *p == '"')) {
        switch (*p) {
        case '<': /* encoding or fontfile field */
            /* If we see <[ or <<, just ignore the second char instead
               of doing as directed (define encoding file, fully embed); sorry.  */
            if (++p < endptr && (*p == '[' || *p == '<')) p++; /*skip */
            skip_blank(&p, endptr);
            if ((q = parse_string_value(&p, endptr))) {
                int n = strlen(q);
                if (n > 4 && strstartswith(q + n - 4, ".enc"))
                    mrec->enc_name = q;
                else
                    mrec->font_name = q;
            }
            skip_blank(&p, endptr);
            break;

        case '"': /* Options */
            if ((q = parse_string_value(&p, endptr))) {
                const char *r = q, *e = q+strlen(q);
                char *s, *t;
                skip_blank(&r, e);
                while (r < e) {
                    if ((s = parse_float_decimal(&r, e))) {
                        skip_blank(&r, e);
                        if ((t = parse_string_value(&r, e))) {
                            if (streq_ptr(t, "SlantFont"))
                                mrec->opt.slant = atof(s);
                            else if (streq_ptr(t, "ExtendFont"))
                                mrec->opt.extend = atof(s);
                            free(t);
                        }
                        free(s);
                    } else if ((s = parse_string_value(&r, e))) { /* skip */
                        free(s);
                    }
                    skip_blank(&r, e);
                }
                free(q);
            }
            skip_blank(&p, endptr);
            break;

        default:
            dpx_warning("Found an invalid entry: %s", p);
            return -1;
        }
        skip_blank(&p, endptr);
    }

    if (p < endptr && *p != '\r' && *p != '\n') {
        dpx_warning("Invalid char in fontmap line: %c", *p);
        return -1;
    }

    return  0;
}


static struct ht_table *fontmap = NULL;

#define fontmap_invalid(m) (!(m) || !(m)->map_name || !(m)->font_name)
static char *
chop_sfd_name (const char *tex_name, char **sfd_name)
{
    char  *fontname;
    char  *p, *q;
    int    m, n, len;

    *sfd_name = NULL;

    p = strchr(tex_name, '@');
    if (!p ||
        p[1] == '\0' || p == tex_name) {
        return  NULL;
    }
    m = (int) (p - tex_name);
    p++;
    q = strchr(p, '@');
    if (!q || q == p) {
        return NULL;
    }
    n = (int) (q - p);
    q++;

    len = strlen(tex_name) - n;
    fontname = NEW(len+1, char);
    memcpy(fontname, tex_name, m);
    fontname[m] = '\0';
    if (*q)
        strcat(fontname, q);

    *sfd_name = NEW(n+1, char);
    memcpy(*sfd_name, p, n);
    (*sfd_name)[n] = '\0';

    return  fontname;
}

static char *
make_subfont_name (const char *map_name, const char *sfd_name, const char *sub_id)
{
    char  *tfm_name;
    int    n, m;
    char  *p, *q;

    p = strchr(map_name, '@');
    if (!p || p == map_name)
        return  NULL;
    m = (int) (p - map_name);
    q = strchr(p + 1, '@');
    if (!q || q == p + 1)
        return  NULL;
    n = (int) (q - p) + 1; /* including two '@' */
    if (strlen(sfd_name) != n - 2 ||
        memcmp(p + 1, sfd_name, n - 2))
        return  NULL;
    tfm_name = NEW(strlen(map_name) - n + strlen(sub_id) + 1, char);
    memcpy(tfm_name, map_name, m);
    tfm_name[m] = '\0';
    strcat(tfm_name, sub_id);
    if (q[1]) /* not ending with '@' */
        strcat(tfm_name, q + 1);

    return  tfm_name;
}

/* "foo@A@ ..." is expanded to
 *   fooab ... -m sfd:A,ab
 *   ...
 *   fooyz ... -m sfd:A,yz
 * where 'ab' ... 'yz' is subfont IDs in SFD 'A'.
 */
int
pdf_append_fontmap_record (const char *kp, const fontmap_rec *vp)
{
    fontmap_rec *mrec;
    char        *fnt_name, *sfd_name = NULL;

    if (!kp || fontmap_invalid(vp)) {
        dpx_warning("Invalid fontmap record...");
        return -1;
    }

    if (dpx_conf.verbose_level > 3)
        dpx_message("fontmap>> append key=\"%s\"...", kp);

    fnt_name = chop_sfd_name(kp, &sfd_name);
    if (fnt_name && sfd_name) {
        char  *tfm_name;
        char **subfont_ids;
        int    n = 0;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &n);
        if (!subfont_ids)
            return  -1;
        while (n-- > 0) {
            tfm_name = make_subfont_name(kp, sfd_name, subfont_ids[n]);
            if (!tfm_name)
                continue;
            mrec = ht_lookup_table(fontmap, tfm_name, strlen(tfm_name));
            if (!mrec) {
                mrec = NEW(1, fontmap_rec);
                pdf_init_fontmap_record(mrec);
                mrec->map_name = mstrdup(kp); /* link */
                mrec->charmap.sfd_name   = mstrdup(sfd_name);
                mrec->charmap.subfont_id = mstrdup(subfont_ids[n]);
                ht_insert_table(fontmap, tfm_name, strlen(tfm_name), mrec);
            }
            free(tfm_name);
        }
        free(fnt_name);
        free(sfd_name);
    }

    mrec = ht_lookup_table(fontmap, kp, strlen(kp));
    if (!mrec) {
        mrec = NEW(1, fontmap_rec);
        pdf_copy_fontmap_record(mrec, vp);
        if (mrec->map_name && streq_ptr(kp, mrec->map_name)) {
            mrec->map_name = mfree(mrec->map_name);
        }
        ht_insert_table(fontmap, kp, strlen(kp), mrec);
    }
    if (dpx_conf.verbose_level > 3)
        dpx_message("\n");

    return  0;
}

int
pdf_remove_fontmap_record (const char *kp)
{
    char  *fnt_name, *sfd_name = NULL;

    if (!kp)
        return  -1;

    if (dpx_conf.verbose_level > 3)
        dpx_message("fontmap>> remove key=\"%s\"...", kp);

    fnt_name = chop_sfd_name(kp, &sfd_name);
    if (fnt_name && sfd_name) {
        char  *tfm_name;
        char **subfont_ids;
        int    n = 0;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &n);
        if (!subfont_ids)
            return  -1;
        if (dpx_conf.verbose_level > 3)
            dpx_message("\nfontmap>> Expand @%s@:", sfd_name);
        while (n-- > 0) {
            tfm_name = make_subfont_name(kp, sfd_name, subfont_ids[n]);
            if (!tfm_name)
                continue;
            if (dpx_conf.verbose_level > 3)
                dpx_message(" %s", tfm_name);
            ht_remove_table(fontmap, tfm_name, strlen(tfm_name));
            free(tfm_name);
        }
        free(fnt_name);
        free(sfd_name);
    }

    ht_remove_table(fontmap, kp, strlen(kp));

    if (dpx_conf.verbose_level > 3)
        dpx_message("\n");

    return  0;
}

fontmap_rec *
pdf_insert_fontmap_record (const char *kp, const fontmap_rec *vp)
{
    fontmap_rec *mrec;
    char        *fnt_name, *sfd_name;

    if (!kp || fontmap_invalid(vp)) {
        dpx_warning("Invalid fontmap record...");
        return NULL;
    }

    if (dpx_conf.verbose_level > 3)
        dpx_message("fontmap>> insert key=\"%s\"...", kp);

    fnt_name = chop_sfd_name(kp, &sfd_name);
    if (fnt_name && sfd_name) {
        char  *tfm_name;
        char **subfont_ids;
        int    n = 0;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &n);
        if (!subfont_ids) {
            dpx_warning("Could not open SFD file: %s", sfd_name);
            free(fnt_name);
            free(sfd_name);
            return NULL;
        }
        if (dpx_conf.verbose_level > 3)
            dpx_message("\nfontmap>> Expand @%s@:", sfd_name);
        while (n-- > 0) {
            tfm_name = make_subfont_name(kp, sfd_name, subfont_ids[n]);
            if (!tfm_name)
                continue;
            if (dpx_conf.verbose_level > 3)
                dpx_message(" %s", tfm_name);
            mrec = NEW(1, fontmap_rec);
            pdf_init_fontmap_record(mrec);
            mrec->map_name = mstrdup(kp); /* link to this entry */
            mrec->charmap.sfd_name   = mstrdup(sfd_name);
            mrec->charmap.subfont_id = mstrdup(subfont_ids[n]);
            ht_insert_table(fontmap, tfm_name, strlen(tfm_name), mrec);
            free(tfm_name);
        }
        free(fnt_name);
        free(sfd_name);
    }

    mrec = NEW(1, fontmap_rec);
    pdf_copy_fontmap_record(mrec, vp);
    if (mrec->map_name && streq_ptr(kp, mrec->map_name)) {
        mrec->map_name = mfree(mrec->map_name);
    }
    ht_insert_table(fontmap, kp, strlen(kp), mrec);

    if (dpx_conf.verbose_level > 3)
        dpx_message("\n");

    return mrec;
}


int
pdf_read_fontmap_line (fontmap_rec *mrec, const char *mline, int mline_len, int format)
{
    int    error;
    char  *q;
    const char *p, *endptr;

    assert(mrec);

    p      = mline;
    endptr = p + mline_len;

    skip_blank(&p, endptr);
    if (p >= endptr)
        return -1;

    q = parse_string_value(&p, endptr);
    if (!q)
        return -1;

    if (format > 0) /* DVIPDFM format */
        error = fontmap_parse_mapdef_dpm(mrec, p, endptr);
    else /* DVIPS/pdfTeX format */
        error = fontmap_parse_mapdef_dps(mrec, p, endptr);
    if (!error) {
        char  *fnt_name, *sfd_name = NULL;
        fnt_name = chop_sfd_name(q, &sfd_name);
        if (fnt_name && sfd_name) {
            if (!mrec->font_name) {
                /* In the case of subfonts, the base name (before the character '@')
                 * will be used as a font_name by default.
                 * Otherwise tex_name will be used as a font_name by default.
                 */
                mrec->font_name = fnt_name;
            } else {
                free(fnt_name);
            }
            free(mrec->charmap.sfd_name);
            mrec->charmap.sfd_name = sfd_name ;
        }
        fill_in_defaults(mrec, q);
    }
    free(q);

    return  error;
}

/* DVIPS/pdfTeX fontmap line if one of the following three cases found:
 *
 * (1) any line including the character '"'
 * (2) any line including the character '<'
 * (3) if the line consists of two entries (tfmname and psname)
 *
 * DVIPDFM fontmap line otherwise.
 */
int
is_pdfm_mapline (const char *mline) /* NULL terminated. */
{
    unsigned int n = 0;
    const char *p, *endptr;

    if (strchr(mline, '"') || strchr(mline, '<'))
        return -1; /* DVIPS/pdfTeX format */

    p      = mline;
    endptr = p + strlen(mline);

    skip_blank(&p, endptr);

    while (p < endptr) {
        /* Break if '-' preceeded by blanks is found. (DVIPDFM format) */
        if (*p == '-') return 1;
        for (n++; p < endptr && !ISBLANK(*p); p++);
        skip_blank(&p, endptr);
    }

    /* Two entries: TFM_NAME PS_NAME only (DVIPS format)
     * Otherwise (DVIPDFM format) */
    return (n == 2 ? 0 : 1);
}


int
pdf_load_fontmap_file (const char *filename, int mode)
{
    fontmap_rec *mrec;
    rust_input_handle_t handle;
    const char *p = NULL, *endptr;
    int llen, lpos  = 0;
    int error = 0, format = 0;

    assert(filename);
    assert(fontmap);

    if (dpx_conf.verbose_level)
        dpx_message("<FONTMAP:");

    handle = dpx_tt_open(filename, ".map", TTBC_FILE_FORMAT_FONT_MAP);
    if (handle == NULL) {
        dpx_warning("Couldn't open font map file \"%s\".", filename);
        return  -1;
    }

    while (!error && (p = tt_readline(work_buffer, WORK_BUFFER_SIZE, handle)) != NULL) {
        int m;

        lpos++;
        llen  = strlen(work_buffer);
        endptr = p + llen;

        skip_blank(&p, endptr);
        if (p == endptr)
            continue;

        m = is_pdfm_mapline(p);

        if (format * m < 0) { /* mismatch */
            dpx_warning("Found a mismatched fontmap line %d from %s.", lpos, filename);
            dpx_warning("-- Ignore the current input buffer: %s", p);
            continue;
        } else
            format += m;

        mrec  = NEW(1, fontmap_rec);
        pdf_init_fontmap_record(mrec);

        /* format > 0: DVIPDFM, format <= 0: DVIPS/pdfTeX */
        error = pdf_read_fontmap_line(mrec, p, llen, format);
        if (error) {
            dpx_warning("Invalid map record in fontmap line %d from %s.", lpos, filename);
            dpx_warning("-- Ignore the current input buffer: %s", p);
            pdf_clear_fontmap_record(mrec);
            free(mrec);
            continue;
        } else {
            switch (mode) {
            case FONTMAP_RMODE_REPLACE:
                pdf_insert_fontmap_record(mrec->map_name, mrec);
                break;
            case FONTMAP_RMODE_APPEND:
                pdf_append_fontmap_record(mrec->map_name, mrec);
                break;
            case FONTMAP_RMODE_REMOVE:
                pdf_remove_fontmap_record(mrec->map_name);
                break;
            }
        }
        pdf_clear_fontmap_record(mrec);
        free(mrec);
    }

    ttstub_input_close(handle);

    if (dpx_conf.verbose_level)
        dpx_message(">");

    return error;
}


fontmap_rec *
pdf_insert_native_fontmap_record (const char *path, uint32_t index,
                                  int layout_dir, int extend, int slant, int embolden)
{
    char        *fontmap_key;
    fontmap_rec *mrec;
    fontmap_rec *ret;

    assert(path);

    fontmap_key = xmalloc(strlen(path) + 40);      // CHECK
    sprintf(fontmap_key, "%s/%d/%c/%d/%d/%d", path, index, layout_dir == 0 ? 'H' : 'V', extend, slant, embolden);

    if (dpx_conf.verbose_level)
        dpx_message("<NATIVE-FONTMAP:%s", fontmap_key);

    mrec  = NEW(1, fontmap_rec);
    pdf_init_fontmap_record(mrec);

    mrec->map_name  = fontmap_key;
    mrec->enc_name  = mstrdup(layout_dir == 0 ? "Identity-H" : "Identity-V");
    mrec->font_name = mstrdup(path);
    mrec->opt.index = index;
    if (layout_dir != 0)
        mrec->opt.flags |= FONTMAP_OPT_VERT;

    fill_in_defaults(mrec, fontmap_key);
    free(fontmap_key);

    mrec->opt.extend = extend   / 65536.0;
    mrec->opt.slant  = slant    / 65536.0;
    mrec->opt.bold   = embolden / 65536.0;
    mrec->opt.use_glyph_encoding = 1;

    ret = pdf_insert_fontmap_record(mrec->map_name, mrec);
    pdf_clear_fontmap_record(mrec);
    free(mrec);

    if (dpx_conf.verbose_level)
        dpx_message(">");

    return ret;
}

fontmap_rec *
pdf_lookup_fontmap_record (const char *tfm_name)
{
    fontmap_rec *mrec = NULL;

    if (fontmap && tfm_name)
        mrec = ht_lookup_table(fontmap, tfm_name, strlen(tfm_name));

    return  mrec;
}


void
pdf_init_fontmaps (void)
{
    fontmap = NEW(1, struct ht_table);
    ht_init_table(fontmap, hval_free);
}

void
pdf_close_fontmaps (void)
{
    if (fontmap) {
        ht_clear_table(fontmap);
        free(fontmap);
    }
    fontmap = NULL;

    release_sfd_record();
}

/* CIDFont options
 *
 * FORMAT:
 *
 *   (:int:)?!?string(/string)?(,string)?
 */

static char *
substr (const char **str, char stop)
{
    char *sstr;
    const char *endptr;

    endptr = strchr(*str, stop);
    if (!endptr || endptr == *str)
        return NULL;
    sstr = NEW(endptr-(*str)+1, char);
    memcpy(sstr, *str, endptr-(*str));
    sstr[endptr-(*str)] = '\0';

    *str = endptr+1;
    return sstr;
}

#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CID_MAPREC_CSI_DELIM '/'

static char *
strip_options (const char *map_name, fontmap_opt *opt)
{
    char *font_name;
    const char *p;
    char *next = NULL;
    int   have_csi = 0, have_style = 0;

    assert(opt);

    p = map_name;
    font_name      = NULL;
    opt->charcoll  = NULL;
    opt->index     = 0;
    opt->style     = FONTMAP_STYLE_NONE;
    opt->flags     = 0;

    if (*p == ':' && isdigit((unsigned char)*(p+1))) {
        opt->index = (int) strtoul(p+1, &next, 10);
        if (*next == ':')
            p = next + 1;
        else {
            opt->index = 0;
        }
    }
    if (*p == '!') { /* no-embedding */
        if (*(++p) == '\0')
            _tt_abort("Invalid map record: %s (--> %s)", map_name, p);
        opt->flags |= FONTMAP_OPT_NOEMBED;
    }

    if ((next = strchr(p, CID_MAPREC_CSI_DELIM)) != NULL) {
        if (next == p)
            _tt_abort("Invalid map record: %s (--> %s)", map_name, p);
        font_name = substr(&p, CID_MAPREC_CSI_DELIM);
        have_csi  = 1;
    } else if ((next = strchr(p, ',')) != NULL) {
        if (next == p)
            _tt_abort("Invalid map record: %s (--> %s)", map_name, p);
        font_name = substr(&p, ',');
        have_style = 1;
    } else {
        font_name = NEW(strlen(p)+1, char);
        strcpy(font_name, p);
    }

    if (have_csi) {
        if ((next = strchr(p, ',')) != NULL) {
            opt->charcoll = substr(&p, ',');
            have_style = 1;
        } else if (p[0] == '\0') {
            _tt_abort("Invalid map record: %s.", map_name);
        } else {
            opt->charcoll = NEW(strlen(p)+1, char);
            strcpy(opt->charcoll, p);
        }
    }

    if (have_style) {
        if (strstartswith(p, "BoldItalic")) {
            if (*(p+10))
                _tt_abort("Invalid map record: %s (--> %s)", map_name, p);
            opt->style = FONTMAP_STYLE_BOLDITALIC;
        } else if (strstartswith(p, "Bold")) {
            if (*(p+4))
                _tt_abort("Invalid map record: %s (--> %s)", map_name, p);
            opt->style = FONTMAP_STYLE_BOLD;
        } else if (strstartswith(p, "Italic")) {
            if (*(p+6))
                _tt_abort("Invalid map record: %s (--> %s)", map_name, p);
            opt->style = FONTMAP_STYLE_ITALIC;
        }
    }

    return font_name;
}
