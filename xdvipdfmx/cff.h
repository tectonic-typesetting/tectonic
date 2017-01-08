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

#ifndef _CFF_H_
#define _CFF_H_

#include "mfileio.h"
#include "cff_types.h"

/* Flag */
#define FONTTYPE_CIDFONT  (1 << 0)
#define FONTTYPE_FONT     (1 << 1)
#define FONTTYPE_MMASTER  (1 << 2)

#define ENCODING_STANDARD (1 << 3)
#define ENCODING_EXPERT   (1 << 4)

#define CHARSETS_ISOADOBE (1 << 5)
#define CHARSETS_EXPERT   (1 << 6)
#define CHARSETS_EXPSUB   (1 << 7)

#define HAVE_STANDARD_ENCODING (ENCODING_STANDARD|ENCODING_EXPERT)
#define HAVE_STANDARD_CHARSETS \
  (CHARSETS_ISOADOBE|CHARSETS_EXPERT|CHARSETS_EXPSUB)

#define CFF_STRING_NOTDEF 65535

typedef struct
{
  char         *fontname; /* FontName */

  /* - CFF structure - */
  cff_header    header;   /* CFF Header */
  cff_index    *name;     /* Name INDEX */
  cff_dict     *topdict;  /* Top DICT (single) */
  cff_index    *string;   /* String INDEX */
  cff_index    *gsubr;    /* Global Subr INDEX */
  cff_encoding *encoding; /* Encodings */
  cff_charsets *charsets; /* Charsets  */
  cff_fdselect *fdselect; /* FDSelect, CIDFont only */
  cff_index    *cstrings; /* CharStrings */
  cff_dict    **fdarray;  /* CIDFont only */
  cff_dict    **private;  /* per-Font DICT */
  cff_index   **subrs;    /* Local Subr INDEX, per-Private DICT */

  /* -- extra data -- */
  l_offset    offset;     /* non-zero for OpenType or PostScript wrapped */
  l_offset    gsubr_offset;
  card16      num_glyphs; /* number of glyphs (CharString INDEX count) */
  card8       num_fds;    /* number of Font DICT */

  /* Updated String INDEX.
   * Please fix this. We should separate input and output.
   */
  cff_index  *_string;

  FILE         *stream;

  int           filter;   /* not used, ASCII Hex filter if needed */

  int           index;    /* CFF fontset index */
  int           flag;     /* Flag: see above */
  int           is_notdef_notzero; /* 1 if .notdef is not the 1st glyph */
} cff_font;

extern cff_font *cff_open  (FILE *file, int offset, int idx);
#define cff_seek_set(c, p) seek_absolute (((c)->stream), ((c)->offset) + (p));
#define cff_read_data(d, l, c)   fread(d, 1, l, (c)->stream)
#define cff_tell(c) ftell((c)->stream)
#define cff_seek(c, p) seek_absolute((c)->stream, p)

extern void      cff_close (cff_font *cff);

/* CFF Header */
extern int cff_put_header (cff_font *cff, card8 *dest, int destlen);

/* CFF INDEX */
extern cff_index *cff_get_index        (cff_font *cff);
extern cff_index *cff_get_index_header (cff_font *cff);
extern void       cff_release_index    (cff_index *idx);
extern cff_index *cff_new_index        (card16 count);
extern int        cff_index_size       (cff_index *idx);
extern int        cff_pack_index       (cff_index *idx, card8 *dest, int destlen);

/* Name INDEX */
extern char *cff_get_name (cff_font *cff);
extern int   cff_set_name (cff_font *cff, char *name);

/* Global and Local Subrs INDEX */
extern int   cff_read_subrs (cff_font *cff);

/* Encoding */
extern int    cff_read_encoding    (cff_font *cff);
extern int    cff_pack_encoding    (cff_font *cff, card8 *dest, int destlen);
extern card16 cff_encoding_lookup  (cff_font *cff, card8 code);
extern void   cff_release_encoding (cff_encoding *encoding);

/* Charsets */
extern int    cff_read_charsets    (cff_font *cff);
extern int    cff_pack_charsets    (cff_font *cff, card8 *dest, int destlen);

/* Returns GID of PS name "glyph" */
extern card16 cff_glyph_lookup     (cff_font *cff, const char *glyph);
/* Return PS name of "gid" */
extern char*  cff_get_glyphname    (cff_font *cff, card16 gid);
/* Returns GID of glyph with SID/CID "cid" */
extern card16 cff_charsets_lookup  (cff_font *cff, card16 cid);
extern card16 cff_charsets_lookup_gid (cff_charsets *charset, card16 cid);
extern void   cff_release_charsets (cff_charsets *charset);
/* Returns SID or CID */
extern card16 cff_charsets_lookup_inverse (cff_font *cff, card16 gid);
extern card16 cff_charsets_lookup_cid(cff_charsets *charset, card16 gid);

/* FDSelect */
extern int   cff_read_fdselect    (cff_font *cff);
extern int   cff_pack_fdselect    (cff_font *cff, card8 *dest, int destlen);
extern card8 cff_fdselect_lookup  (cff_font *cff, card16 gid);
extern void  cff_release_fdselect (cff_fdselect *fdselect);

/* Font DICT(s) */
extern int   cff_read_fdarray (cff_font *cff);

/* Private DICT(s) */
extern int  cff_read_private (cff_font *cff);

/* String */
extern char *cff_get_string    (cff_font *cff, s_SID id);
extern int   cff_get_sid       (cff_font *cff, const char *str);
extern int   cff_get_seac_sid  (cff_font *cff, const char *str);
extern s_SID cff_add_string    (cff_font *cff, const char *str, int unique);
extern void  cff_update_string (cff_font *cff);

#endif /* _CFF_H_ */
