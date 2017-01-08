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

#ifndef _CFF_TYPES_H_
#define _CFF_TYPES_H_

/* CFF Data Types */

#define CFF_TYPE_UNKNOWN 0
#define CFF_TYPE_INTEGER (1 << 0)
#define CFF_TYPE_REAL    (1 << 1)
#define CFF_TYPE_NUMBER  (CFF_TYPE_INTEGER|CFF_TYPE_REAL)
#define CFF_TYPE_BOOLEAN (1 << 2)
#define CFF_TYPE_SID     (1 << 3)
#define CFF_TYPE_ARRAY   (1 << 4)
#define CFF_TYPE_DELTA   (1 << 5)

/* SID SID number */
#define CFF_TYPE_ROS     (1 << 6)
/* offset(0) */
#define CFF_TYPE_OFFSET  (1 << 7)
/* size offset(0) */
#define CFF_TYPE_SZOFF   (1 << 8)

typedef unsigned char  card8;     /* 1-byte unsigned number */
typedef unsigned short card16;    /* 2-byte unsigned number */
typedef unsigned char  c_offsize; /* 1-byte unsigned number specifies the size
				     of an Offset field or fields, range 1-4 */
typedef uint32_t       l_offset;  /* 1, 2, 3, or 4-byte offset */
typedef unsigned short s_SID;       /* 2-byte string identifier  */

typedef struct {
  card16    count;   /* number of objects stored in INDEX */
  c_offsize offsize; /* Offset array element size, 1-4    */
  l_offset  *offset; /* Offset array, count + 1 offsets   */
  card8     *data;   /* Object data                       */
} cff_index;

typedef struct {
  card8     major;    /* format major version (starting at 1) */
  card8     minor;    /* format minor version (starting at 0) */
  card8     hdr_size; /* Header size (bytes)                  */
  c_offsize offsize;  /* Absolute offset (0) size             */
} cff_header;


/* Dictionary */
typedef struct {
  int     id;     /* encoded data value (as card8 or card16) */
  const char *key; /* opname                                 */
  int     count;  /* number of values                        */
  double *values; /* values                                  */
} cff_dict_entry;

typedef struct {
  int    max;
  int    count;
  cff_dict_entry *entries;
} cff_dict;

/* Encoding, Charset and FDSelect */
typedef struct
{
  s_SID   first;  /* SID or CID, or card8 for Encoding  */
  card8 n_left; /* no. of remaining gids/codes in this range */
} cff_range1;

typedef struct
{
  s_SID    first;  /* SID or CID (card16)      */
  card16 n_left; /* card16-version of range1 */
} cff_range2;

typedef struct
{
  card8 code;
  s_SID   glyph;
} cff_map;

typedef struct
{
  card8 format;       /* if (format & 0x80) then have supplement */
  card8 num_entries;  /* number of entries */
  union {
    card8 *codes;       /* format 0 */
    cff_range1 *range1; /* format 1 */
  } data;
  card8 num_supps; /* number of supplementary data */
  cff_map *supp;   /* supplement */
} cff_encoding;

typedef struct
{
  card8 format;
  card16 num_entries;
  union {
    s_SID *glyphs;        /* format 0 */
    cff_range1 *range1; /* format 1 */
    cff_range2 *range2; /* format 2 */
  } data;
} cff_charsets;

/* CID-Keyed font specific */
typedef struct
{
  card16 first;
  card8  fd;
} cff_range3;

typedef struct
{
  card8 format;
  card16 num_entries; /* number of glyphs/ranges */
  union {
    card8      *fds;    /* format 0 */
    cff_range3 *ranges; /* format 3 */
  } data;
  /* card16 sentinel; */ /* format 3 only, must be equals to num_glyphs */
} cff_fdselect;

#endif /* _CFF_TYPES_H_ */
