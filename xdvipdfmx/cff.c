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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <stdio.h>
#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"
#include "mfileio.h"

#include "cff_limits.h"
#include "cff_types.h"
#include "cff_stdstr.h"
#include "cff_dict.h"

#include "cff.h"

#define CFF_DEBUG     5
#define CFF_DEBUG_STR "CFF"

static unsigned get_unsigned (FILE *stream, int n)
{
  unsigned v = 0;

  while (n-- > 0)
    v = v*0x100u + get_unsigned_byte(stream);

  return v;
}

#define get_offset(s, n) get_unsigned((s), (n))

/*
 * Read Header, Name INDEX, Top DICT INDEX, and String INDEX.
 */
cff_font *cff_open(FILE *stream, int offset, int n)
{
  cff_font  *cff;
  cff_index *idx;

  cff = NEW(1, cff_font);

  cff->fontname = NULL;
  cff->index    = n;
  cff->stream   = stream;
  cff->offset   = offset;
  cff->filter   = 0;      /* not used */
  cff->flag     = 0;

  cff->name     = NULL;
  cff->topdict  = NULL;
  cff->gsubr    = NULL;
  cff->encoding = NULL;
  cff->charsets = NULL;
  cff->fdselect = NULL;
  cff->cstrings = NULL;
  cff->fdarray  = NULL;
  cff->private  = NULL;
  cff->subrs    = NULL;

  cff->num_glyphs = 0;
  cff->num_fds    = 0;
  cff->string     = NULL;
  cff->_string    = NULL;

  cff_seek_set(cff, 0);
  cff->header.major    = get_unsigned_byte(cff->stream);
  cff->header.minor    = get_unsigned_byte(cff->stream);
  cff->header.hdr_size = get_unsigned_byte(cff->stream);
  cff->header.offsize  = get_unsigned_byte(cff->stream);
  if (cff->header.offsize < 1 ||
      cff->header.offsize > 4)
    ERROR("invalid offsize data");

  if (cff->header.major > 1 ||
      cff->header.minor > 0) {
    WARN("%s: CFF version %u.%u not supported.",
         CFF_DEBUG_STR, cff->header.major, cff->header.minor);
    cff_close(cff);
    return NULL;
  }

  cff_seek_set(cff, (cff->header).hdr_size);

  /* Name INDEX */
  idx = cff_get_index(cff);
  if (n > idx->count - 1) {
    WARN("%s: Invalid CFF fontset index number.", CFF_DEBUG_STR);
    cff_close(cff);
    return NULL;
  }

  cff->name = idx;

  cff->fontname = cff_get_name(cff);

  /* Top DICT INDEX */
  idx = cff_get_index(cff);
  if (n > idx->count - 1)
    ERROR("CFF Top DICT not exist...");
  cff->topdict = cff_dict_unpack(idx->data + idx->offset[n] - 1,
                                 idx->data + idx->offset[n + 1] - 1);
  if (!cff->topdict)
    ERROR("Parsing CFF Top DICT data failed...");
  cff_release_index(idx);

  if (cff_dict_known(cff->topdict, "CharstringType") &&
      cff_dict_get(cff->topdict, "CharstringType", 0) != 2) {
    WARN("Only Type 2 Charstrings supported...");
    cff_close(cff);
    return NULL;
  }

  if (cff_dict_known(cff->topdict, "SyntheticBase")) {
    WARN("CFF Synthetic font not supported.");
    cff_close(cff);
    return NULL;
  }

  /* String INDEX */
  cff->string = cff_get_index(cff);

  /* offset to GSubr */
  cff->gsubr_offset = tell_position(cff->stream) - offset;

  /* Number of glyphs */
  offset = cff_dict_get(cff->topdict, "CharStrings", 0);
  cff_seek_set(cff, offset);
  cff->num_glyphs = get_unsigned_pair(cff->stream);

  /* Check for font type */
  if (cff_dict_known(cff->topdict, "ROS")) {
    cff->flag |= FONTTYPE_CIDFONT;
  } else {
    cff->flag |= FONTTYPE_FONT;
  }

  /* Check for encoding */
  if (cff_dict_known(cff->topdict, "Encoding")) {
    offset = cff_dict_get(cff->topdict, "Encoding", 0);
    if (offset == 0) { /* predefined */
      cff->flag |= ENCODING_STANDARD;
    } else if (offset == 1) {
      cff->flag |= ENCODING_EXPERT;
    }
  } else {
    cff->flag |= ENCODING_STANDARD;
  }

  /* Check for charset */
  if (cff_dict_known(cff->topdict, "charset")) {
    offset = cff_dict_get(cff->topdict, "charset", 0);
    if (offset == 0) { /* predefined */
      cff->flag |= CHARSETS_ISOADOBE;
    } else if (offset == 1) {
      cff->flag |= CHARSETS_EXPERT;
    } else if (offset == 2) {
      cff->flag |= CHARSETS_EXPSUB;
    }
  } else {
    cff->flag |= CHARSETS_ISOADOBE;
  }

  cff_seek_set(cff, cff->gsubr_offset); /* seek back to GSubr */

  return cff;
}

void
cff_close (cff_font *cff)
{
  card16 i;

  if (cff) {
    if (cff->fontname) RELEASE(cff->fontname);
    if (cff->name) cff_release_index(cff->name);
    if (cff->topdict) cff_release_dict(cff->topdict);
    if (cff->string) cff_release_index(cff->string);
    if (cff->gsubr) cff_release_index(cff->gsubr);
    if (cff->encoding) cff_release_encoding(cff->encoding);
    if (cff->charsets) cff_release_charsets(cff->charsets);
    if (cff->fdselect) cff_release_fdselect(cff->fdselect);
    if (cff->cstrings) cff_release_index(cff->cstrings);
    if (cff->fdarray) {
      for (i=0;i<cff->num_fds;i++) {
        if (cff->fdarray[i]) cff_release_dict(cff->fdarray[i]);
      }
      RELEASE(cff->fdarray);
    }
    if (cff->private) {
      for (i=0;i<cff->num_fds;i++) {
        if (cff->private[i]) cff_release_dict(cff->private[i]);
      }
      RELEASE(cff->private);
    }
    if (cff->subrs) {
      for (i=0;i<cff->num_fds;i++) {
        if (cff->subrs[i]) cff_release_index(cff->subrs[i]);
      }
      RELEASE(cff->subrs);
    }
    if (cff->_string)
      cff_release_index(cff->_string);

    RELEASE(cff);
  }

  return;
}

char *
cff_get_name (cff_font *cff)
{
  char      *fontname;
  l_offset   len;
  cff_index *idx;

  idx = cff->name;
  len = idx->offset[cff->index + 1] - idx->offset[cff->index];
  fontname = NEW(len + 1, char);
  memcpy(fontname, idx->data + idx->offset[cff->index] - 1, len);
  fontname[len] = '\0';

  return fontname;
}

int
cff_set_name (cff_font *cff, char *name)
{
  cff_index *idx;

  if (strlen(name) > 127)
    ERROR("FontName string length too large...");

  if (cff->name)
    cff_release_index(cff->name);

  cff->name = idx = NEW(1, cff_index);
  idx->count   = 1;
  idx->offsize = 1;
  idx->offset  = NEW(2, l_offset);
  (idx->offset)[0] = 1;
  (idx->offset)[1] = strlen(name) + 1;
  idx->data = NEW(strlen(name), card8);
  memmove(idx->data, name, strlen(name)); /* no trailing '\0' */

  return 5 + strlen(name);
}

int
cff_put_header (cff_font *cff, card8 *dest, int destlen)
{
  if (destlen < 4)
    ERROR("Not enough space available...");

  *(dest++) = cff->header.major;
  *(dest++) = cff->header.minor;
  *(dest++) = 4; /* Additional data in between header and
                  * Name INDEX ignored.
                  */
  /* We will set all offset (0) to four-byte integer. */
  *(dest++) = 4;
  cff->header.offsize = 4;

  return 4;
}

/* Only read header part but not body */
cff_index *
cff_get_index_header (cff_font *cff)
{
  cff_index *idx;
  card16     i, count;

  idx = NEW(1, cff_index);

  idx->count = count = get_unsigned_pair(cff->stream);
  if (count > 0) {
    idx->offsize = get_unsigned_byte(cff->stream);
    if (idx->offsize < 1 || idx->offsize > 4)
      ERROR("invalid offsize data");

    idx->offset = NEW(count+1, l_offset);
    for (i=0;i<count;i++) {
      (idx->offset)[i] = get_offset(cff->stream, idx->offsize);
    }
    if (count == 0xFFFF)
      cff_seek(cff, cff_tell(cff) + idx->offsize);
    else
      (idx->offset)[i] = get_offset(cff->stream, idx->offsize);

    if (idx->offset[0] != 1)
      ERROR("cff_get_index(): invalid index data");

    idx->data = NULL;
  } else {
    idx->offsize = 0;
    idx->offset = NULL;
    idx->data = NULL;
  }

  return idx;
}

cff_index *
cff_get_index (cff_font *cff)
{
  cff_index *idx;
  card16     i, count;
  int        length, nb_read, offset;

  idx = NEW(1, cff_index);

  idx->count = count = get_unsigned_pair(cff->stream);
  if (count > 0) {
    idx->offsize = get_unsigned_byte(cff->stream);
    if (idx->offsize < 1 || idx->offsize > 4)
      ERROR("invalid offsize data");

    idx->offset = NEW(count + 1, l_offset);
    for (i = 0 ; i < count + 1; i++) {
      idx->offset[i] = get_offset(cff->stream, idx->offsize);
    }

    if (idx->offset[0] != 1)
      ERROR("Invalid CFF Index offset data");

    length = idx->offset[count] - idx->offset[0];

    idx->data = NEW(length, card8);
    offset    = 0;
    while (length > 0) {
      nb_read = cff_read_data(idx->data + offset, length, cff);
      offset += nb_read;
      length -= nb_read;
    }
  } else {
    idx->offsize = 0;
    idx->offset  = NULL;
    idx->data    = NULL;
  }

  return idx;
}

int
cff_pack_index (cff_index *idx, card8 *dest, int destlen)
{
  int     len = 0;
  int     datalen;
  card16  i;

  if (idx->count < 1) {
    if (destlen < 2)
      ERROR("Not enough space available...");
    memset(dest, 0, 2);
    return 2;
  }

  len     = cff_index_size(idx);
  datalen = idx->offset[idx->count] - 1;

  if (destlen < len)
    ERROR("Not enough space available...");

  *(dest++) = (idx->count >> 8) & 0xff;
  *(dest++) = idx->count & 0xff;

  if (datalen < 0xffUL) {
    idx->offsize = 1;
    *(dest++)    = 1;
    for (i = 0; i <= idx->count; i++) {
      *(dest++) = (card8) (idx->offset[i] & 0xff);
    }
  } else if (datalen < 0xffffUL) {
    idx->offsize = 2;
    *(dest++)    = 2;
    for (i = 0; i <= idx->count; i++) {
      *(dest++) = (card8) ((idx->offset[i] >> 8) & 0xff);
      *(dest++) = (card8) ( idx->offset[i] & 0xff);
    }
  } else if (datalen < 0xffffffUL) {
    idx->offsize = 3;
    *(dest++)    = 3;
    for (i = 0; i <= idx->count; i++) {
      *(dest++) = (card8)((idx->offset[i] >> 16) & 0xff);
      *(dest++) = (card8)((idx->offset[i] >> 8) & 0xff);
      *(dest++) = (card8)(idx->offset[i] & 0xff);
    }
  } else {
    idx->offsize = 4;
    *(dest++)    = 4;
    for (i = 0; i <= idx->count; i++) {
      *(dest++) = (card8)((idx->offset[i] >> 24) & 0xff);
      *(dest++) = (card8)((idx->offset[i] >> 16) & 0xff);
      *(dest++) = (card8)((idx->offset[i] >> 8) & 0xff);
      *(dest++) = (card8)(idx->offset[i] & 0xff);
    }
  }

  memmove(dest, idx->data, idx->offset[idx->count] - 1);

  return len;
}

int 
cff_index_size (cff_index *idx)
{
  if (idx->count > 0) {
    l_offset datalen;

    datalen = idx->offset[idx->count] - 1;
    if (datalen < 0xffUL) {
      idx->offsize = 1;
    } else if (datalen < 0xffffUL) {
      idx->offsize = 2;
    } else if (datalen < 0xffffffUL) {
      idx->offsize = 3;
    } else {
      idx->offsize = 4;
    }
    return (3 + (idx->offsize)*(idx->count + 1) + datalen);
  } else {
    return 2;
  }
}


cff_index *cff_new_index (card16 count)
{
  cff_index *idx;

  idx = NEW(1, cff_index);
  idx->count = count;
  idx->offsize = 0;

  if (count > 0) {
    idx->offset = NEW(count + 1, l_offset);
    (idx->offset)[0] = 1;
  } else {
    idx->offset = NULL;
  }
  idx->data = NULL;

  return idx;
}

void cff_release_index (cff_index *idx)
{
  if (idx) {
    if (idx->data) {
      RELEASE(idx->data);
    }
    if (idx->offset) {
      RELEASE(idx->offset);
    }
    RELEASE(idx);
  }
}

/* Strings */
char *cff_get_string (cff_font *cff, s_SID id)
{
  char *result = NULL;
  int len;

  if (id < CFF_STDSTR_MAX) {
    len = strlen(cff_stdstr[id]);
    result = NEW(len+1, char);
    memcpy(result, cff_stdstr[id], len);
    result[len] = '\0';
  } else if (cff && cff->string) {
    cff_index *strings = cff->string;
    id -= CFF_STDSTR_MAX;
    if (id < strings->count) {
      len = (strings->offset)[id+1] - (strings->offset)[id];
      result = NEW(len + 1, char);
      memmove(result, strings->data + (strings->offset)[id] - 1, len);
      result[len] = '\0';
    }
  }

  return result;
}

int cff_get_sid (cff_font *cff, const char *str)
{
  card16 i;

  if (!cff || !str)
    return -1;

  /* I search String INDEX first. */
  if (cff && cff->string) {
    cff_index *idx = cff->string;
    for (i = 0; i < idx->count; i++) {
      if (strlen(str) == (idx->offset)[i+1] - (idx->offset)[i] &&
          !memcmp(str, (idx->data)+(idx->offset)[i]-1, strlen(str)))
        return (i + CFF_STDSTR_MAX);
    }
  }

  for (i = 0; i < CFF_STDSTR_MAX; i++) {
    if (!strcmp(str, cff_stdstr[i]))
      return i;
  }

  return -1;
}

int cff_get_seac_sid (cff_font *cff, const char *str)
{
  card16 i;

  if (!cff || !str)
    return -1;

  for (i = 0; i < CFF_STDSTR_MAX; i++) {
    if (!strcmp(str, cff_stdstr[i]))
      return i;
  }

  return -1;
}

static int cff_match_string (cff_font *cff, const char *str, s_SID sid)
{
  card16 i;

  if (sid < CFF_STDSTR_MAX) {
    return ((!strcmp(str, cff_stdstr[sid])) ? 1 : 0);
  } else {
    i = sid - CFF_STDSTR_MAX;
    if (cff == NULL || cff->string == NULL || i >= cff->string->count)
      ERROR("Invalid SID");
    if (strlen(str) == (cff->string->offset)[i+1] - (cff->string->offset)[i])
      return (!memcmp(str,
                      (cff->string->data)+(cff->string->offset)[i]-1,
                      strlen(str))) ? 1 : 0;
  }

  return 0;
}

void cff_update_string (cff_font *cff)
{
  if (cff == NULL)
    ERROR("CFF font not opened.");
  
  if (cff->string)
    cff_release_index(cff->string);
  cff->string  = cff->_string;
  cff->_string = NULL;
}

s_SID cff_add_string (cff_font *cff, const char *str, int unique)
/* Setting unique == 1 eliminates redundant or predefined strings. */
{
  card16 idx;
  cff_index *strings;
  l_offset offset, size;
  int len = strlen(str);

  if (cff == NULL)
    ERROR("CFF font not opened.");

  if (cff->_string == NULL)
    cff->_string = cff_new_index(0);
  strings = cff->_string;

  if (unique) {
    /* TODO: do binary search to speed things up */
    for (idx = 0; idx < CFF_STDSTR_MAX; idx++) {
      if (cff_stdstr[idx] && !strcmp(cff_stdstr[idx], str))
        return idx;
    }
    for (idx = 0; idx < strings->count; idx++) {
      size   = strings->offset[idx+1] - strings->offset[idx];
      offset = strings->offset[idx];
      if (size == len && !memcmp(strings->data+offset-1, str, len))
        return (idx + CFF_STDSTR_MAX);
    }
  }

  offset = (strings->count > 0) ? strings->offset[strings->count] : 1;
  strings->offset = RENEW(strings->offset, strings->count+2, l_offset);
  if (strings->count == 0)
    strings->offset[0] = 1;
  idx = strings->count;
  strings->count += 1;
  strings->offset[strings->count] = offset + len;
  strings->data = RENEW(strings->data, offset+len-1, card8);
  memcpy(strings->data+offset-1, str, len);

  return (idx + CFF_STDSTR_MAX);
}

/*
 * Encoding and Charset
 *
 *  Encoding and Charset arrays always begin with GID = 1.
 */
int cff_read_encoding (cff_font *cff)
{
  cff_encoding *encoding;
  int offset, length;
  card8 i;

  if (cff->topdict == NULL) {
    ERROR("Top DICT data not found");
  }

  if (!cff_dict_known(cff->topdict, "Encoding")) {
    cff->flag |= ENCODING_STANDARD;
    cff->encoding = NULL;
    return 0;
  }

  offset = cff_dict_get(cff->topdict, "Encoding", 0);
  if (offset == 0) { /* predefined */
    cff->flag |= ENCODING_STANDARD;
    cff->encoding = NULL;
    return 0;
  } else if (offset == 1) {
    cff->flag |= ENCODING_EXPERT;
    cff->encoding = NULL;
    return 0;
  }

  cff_seek_set(cff, offset);
  cff->encoding = encoding = NEW(1, cff_encoding);
  encoding->format = get_unsigned_byte(cff->stream);
  length = 1;

  switch (encoding->format & (~0x80)) {
  case 0:
    encoding->num_entries = get_unsigned_byte(cff->stream);
    (encoding->data).codes = NEW(encoding->num_entries, card8);
    for (i=0;i<(encoding->num_entries);i++) {
      (encoding->data).codes[i] = get_unsigned_byte(cff->stream);
    }
    length += encoding->num_entries + 1;
    break;
  case 1:
    {
      cff_range1 *ranges;
      encoding->num_entries = get_unsigned_byte(cff->stream);
      encoding->data.range1 = ranges
        = NEW(encoding->num_entries, cff_range1);
      for (i=0;i<(encoding->num_entries);i++) {
        ranges[i].first = get_unsigned_byte(cff->stream);
        ranges[i].n_left = get_unsigned_byte(cff->stream);
      }
      length += (encoding->num_entries) * 2 + 1;
    }
    break;
  default:
    RELEASE(encoding);
    ERROR("Unknown Encoding format");
    break;
  }

  /* Supplementary data */
  if ((encoding->format) & 0x80) {
    cff_map *map;
    encoding->num_supps = get_unsigned_byte(cff->stream);
    encoding->supp = map = NEW(encoding->num_supps, cff_map);
    for (i=0;i<(encoding->num_supps);i++) {
      map[i].code = get_unsigned_byte(cff->stream);
      map[i].glyph = get_unsigned_pair(cff->stream); /* SID */
    }
    length += (encoding->num_supps) * 3 + 1;
  } else {
    encoding->num_supps = 0;
    encoding->supp = NULL;
  }

  return length;
}

int cff_pack_encoding (cff_font *cff, card8 *dest, int destlen)
{
  int len = 0;
  cff_encoding *encoding;
  card16 i;

  if (cff->flag & HAVE_STANDARD_ENCODING || cff->encoding == NULL)
    return 0;

  if (destlen < 2)
    ERROR("in cff_pack_encoding(): Buffer overflow");

  encoding = cff->encoding;

  dest[len++] = encoding->format;
  dest[len++] = encoding->num_entries;
  switch (encoding->format & (~0x80)) {
  case 0:
    if (destlen < len + encoding->num_entries)
      ERROR("in cff_pack_encoding(): Buffer overflow");
    for (i=0;i<(encoding->num_entries);i++) {
      dest[len++] = (encoding->data).codes[i];
    }
    break;
  case 1:
    {
      if (destlen < len + (encoding->num_entries)*2)
        ERROR("in cff_pack_encoding(): Buffer overflow");
      for (i=0;i<(encoding->num_entries);i++) {
        dest[len++] = (encoding->data).range1[i].first & 0xff;
        dest[len++] = (encoding->data).range1[i].n_left;
      }
    }
    break;
  default:
    ERROR("Unknown Encoding format");
    break;
  }

  if ((encoding->format) & 0x80) {
    if (destlen < len + (encoding->num_supps)*3 + 1)
      ERROR("in cff_pack_encoding(): Buffer overflow");
    dest[len++] = encoding->num_supps;
    for (i=0;i<(encoding->num_supps);i++) {
      dest[len++] = (encoding->supp)[i].code;
      dest[len++] = ((encoding->supp)[i].glyph >> 8) & 0xff;
      dest[len++] = (encoding->supp)[i].glyph & 0xff;
    }
  }

  return len;
}

/* input: code, output: glyph index */
card16 cff_encoding_lookup (cff_font *cff, card8 code)
{
  card16 gid = 0;
  cff_encoding *encoding;
  card16 i;

  if (cff->flag & (ENCODING_STANDARD|ENCODING_EXPERT)) {
    ERROR("Predefined CFF encoding not supported yet");
  } else if (cff->encoding == NULL) {
    ERROR("Encoding data not available");
  }

  encoding = cff->encoding;

  gid = 0;
  switch (encoding->format & (~0x80)) {
  case 0:
    for (i = 0; i < encoding->num_entries; i++) {
      if (code == (encoding->data).codes[i]) {
        gid = i + 1;
        break;
      }
    }
    break;
  case 1:
    for (i = 0; i < encoding->num_entries; i++) {
      if (code >= (encoding->data).range1[i].first &&
          code <= (encoding->data).range1[i].first + (encoding->data).range1[i].n_left) {
        gid += code - (encoding->data).range1[i].first + 1;
        break;
      }
      gid += (encoding->data).range1[i].n_left + 1;
    }
    if (i == encoding->num_entries)
      gid = 0;
    break;
  default:
    ERROR("Unknown Encoding format.");
  }

  /* Supplementary data */
  if (gid == 0 && ((encoding->format) & 0x80)) {
    cff_map *map;
    if (!encoding->supp)
      ERROR("No CFF supplementary encoding data read.");
    map = encoding->supp;
    for (i=0;i<(encoding->num_supps);i++) {
      if (code == map[i].code) {
        gid = cff_charsets_lookup(cff, map[i].glyph);
        break;
      }
    }
  }

  return gid;
}

void cff_release_encoding (cff_encoding *encoding)
{
  if (encoding) {
    switch (encoding->format & (~0x80)) {
    case 0:
      if (encoding->data.codes)
        RELEASE(encoding->data.codes);
      break;
    case 1:
      if (encoding->data.range1)
        RELEASE(encoding->data.range1);
      break;
    default:
      ERROR("Unknown Encoding format.");
    }
    if (encoding->format & 0x80) {
      if (encoding->supp)
        RELEASE(encoding->supp);
    }
    RELEASE(encoding);
  }
}

int cff_read_charsets (cff_font *cff)
{
  cff_charsets *charset;
  int offset, length;
  card16 count, i;

  if (cff->topdict == NULL)
    ERROR("Top DICT not available");

  if (!cff_dict_known(cff->topdict, "charset")) {
    cff->flag |= CHARSETS_ISOADOBE;
    cff->charsets = NULL;
    return 0;
  }

  offset = cff_dict_get(cff->topdict, "charset", 0);

  if (offset == 0) { /* predefined */
    cff->flag |= CHARSETS_ISOADOBE;
    cff->charsets = NULL;
    return 0;
  } else if (offset == 1) {
    cff->flag |= CHARSETS_EXPERT;
    cff->charsets = NULL;
    return 0;
  } else if (offset == 2) {
    cff->flag |= CHARSETS_EXPSUB;
    cff->charsets = NULL;
    return 0;
  }

  cff_seek_set(cff, offset);
  cff->charsets = charset = NEW(1, cff_charsets);
  charset->format = get_unsigned_byte(cff->stream);
  charset->num_entries = 0;

  count = cff->num_glyphs - 1;
  length = 1;

  /* Not sure. Not well documented. */
  switch (charset->format) {
  case 0:
    charset->num_entries = cff->num_glyphs - 1; /* no .notdef */
    charset->data.glyphs = NEW(charset->num_entries, s_SID);
    length += (charset->num_entries) * 2;
    for (i=0;i<(charset->num_entries);i++) {
      charset->data.glyphs[i] = get_unsigned_pair(cff->stream);
    }
    count = 0;
    break;
  case 1:
    {
      cff_range1 *ranges = NULL;
      while (count > 0 && charset->num_entries < cff->num_glyphs) {
        ranges = RENEW(ranges, charset->num_entries + 1, cff_range1);
        ranges[charset->num_entries].first = get_unsigned_pair(cff->stream);
        ranges[charset->num_entries].n_left = get_unsigned_byte(cff->stream);
        count -= ranges[charset->num_entries].n_left + 1; /* no-overrap */
        charset->num_entries += 1;
        charset->data.range1 = ranges;
      }
      length += (charset->num_entries) * 3;
    }
    break;
  case 2:
    {
      cff_range2 *ranges = NULL;
      while (count > 0 && charset->num_entries < cff->num_glyphs) {
        ranges = RENEW(ranges, charset->num_entries + 1, cff_range2);
        ranges[charset->num_entries].first = get_unsigned_pair(cff->stream);
        ranges[charset->num_entries].n_left = get_unsigned_pair(cff->stream);
        count -= ranges[charset->num_entries].n_left + 1; /* non-overrapping */
        charset->num_entries += 1;
      }
      charset->data.range2 = ranges;
      length += (charset->num_entries) * 4;
    }
    break;
  default:
    RELEASE(charset);
    ERROR("Unknown Charset format");
    break;
  }

  if (count > 0)
    ERROR("Charset data possibly broken");

  return length;
}

int cff_pack_charsets (cff_font *cff, card8 *dest, int destlen)
{
  int len = 0;
  card16 i;
  cff_charsets *charset;

  if (cff->flag & HAVE_STANDARD_CHARSETS || cff->charsets == NULL)
    return 0;

  if (destlen < 1)
    ERROR("in cff_pack_charsets(): Buffer overflow");

  charset = cff->charsets;

  dest[len++] = charset->format;
  switch (charset->format) {
  case 0:
    if (destlen < len + (charset->num_entries)*2)
      ERROR("in cff_pack_charsets(): Buffer overflow");
    for (i=0;i<(charset->num_entries);i++) {
      s_SID sid = (charset->data).glyphs[i]; /* or CID */
      dest[len++] = (sid >> 8) & 0xff;
      dest[len++] = sid & 0xff;
    }
    break;
  case 1:
    {
      if (destlen < len + (charset->num_entries)*3)
        ERROR("in cff_pack_charsets(): Buffer overflow");
      for (i=0;i<(charset->num_entries);i++) {
        dest[len++] = ((charset->data).range1[i].first >> 8) & 0xff;
        dest[len++] = (charset->data).range1[i].first & 0xff;
        dest[len++] = (charset->data).range1[i].n_left;
      }
    }
    break;
  case 2:
    {
      if (destlen < len + (charset->num_entries)*4)
        ERROR("in cff_pack_charsets(): Buffer overflow");
      for (i=0;i<(charset->num_entries);i++) {
        dest[len++] = ((charset->data).range2[i].first >> 8) & 0xff;
        dest[len++] = (charset->data).range2[i].first & 0xff;
        dest[len++] = ((charset->data).range2[i].n_left >> 8) & 0xff;
        dest[len++] = (charset->data).range2[i].n_left & 0xff;
      }
    }
    break;
  default:
    ERROR("Unknown Charset format");
    break;
  }

  return len;
}

char* cff_get_glyphname (cff_font *cff, card16 gid)
{
  s_SID sid;

  sid = cff_charsets_lookup_inverse(cff, gid);
  return cff_get_string(cff, sid);
}

card16 cff_glyph_lookup (cff_font *cff, const char *glyph)
{
  card16        gid;
  cff_charsets *charset;
  card16        i, n;

  if (cff->flag & (CHARSETS_ISOADOBE|CHARSETS_EXPERT|CHARSETS_EXPSUB)) {
    ERROR("Predefined CFF charsets not supported yet");
  } else if (cff->charsets == NULL) {
    ERROR("Charsets data not available");
  }

  /* .notdef always have glyph index 0 */
  if (!glyph || !strcmp(glyph, ".notdef")) {
    return 0;
  }

  charset = cff->charsets;

  gid = 0;
  switch (charset->format) {
  case 0:
    for (i = 0; i < charset->num_entries; i++) {
      gid++;
      if (cff_match_string(cff, glyph, charset->data.glyphs[i])) {
        return gid;
      }
    }
    break;
  case 1:
    for (i = 0; i < charset->num_entries; i++) {
      for (n = 0;
           n <= charset->data.range1[i].n_left; n++) {
        gid++;
        if (cff_match_string(cff, glyph,
                             (s_SID)(charset->data.range1[i].first + n))) {
          return gid;
        }
      }
    }
    break;
  case 2:
    for (i = 0; i <charset->num_entries; i++) {
      for (n = 0;
           n <= charset->data.range2[i].n_left; n++) {
        gid++;
        if (cff_match_string(cff, glyph,
                             (s_SID)(charset->data.range2[i].first + n))) {
          return gid;
        }
      }
    }
    break;
  default:
    ERROR("Unknown Charset format");
  }

  return 0; /* not found, returns .notdef */
}

/* Input : SID or CID (16-bit unsigned int)
 * Output: glyph index
 */
card16
cff_charsets_lookup (cff_font *cff, card16 cid)
{
  if (cff->flag & (CHARSETS_ISOADOBE|CHARSETS_EXPERT|CHARSETS_EXPSUB)) {
    ERROR("Predefined CFF charsets not supported yet");
  } else if (cff->charsets == NULL) {
    ERROR("Charsets data not available");
  }

  return cff_charsets_lookup_gid(cff->charsets, cid);
}

card16 cff_charsets_lookup_gid (cff_charsets *charset, card16 cid)
{
  card16 gid = 0;
  card16 i;

  if (cid == 0) {
    return 0; /* GID 0 (.notdef) */
  }

  switch (charset->format) {
  case 0:
    for (i = 0; i <charset->num_entries; i++) {
      if (cid == charset->data.glyphs[i]) {
        gid = i + 1;
        return gid;
      }
    }
    break;
  case 1:
    for (i = 0; i < charset->num_entries; i++) {
      if (cid >= charset->data.range1[i].first &&
          cid <= charset->data.range1[i].first + charset->data.range1[i].n_left) {
        gid += cid - charset->data.range1[i].first + 1;
        return gid;
      }
      gid += charset->data.range1[i].n_left + 1;
    }
    break;
  case 2:
    for (i = 0; i < charset->num_entries; i++) {
      if (cid >= charset->data.range2[i].first &&
          cid <= charset->data.range2[i].first + charset->data.range2[i].n_left) {
        gid += cid - charset->data.range2[i].first + 1;
        return gid;
      }
      gid += charset->data.range2[i].n_left + 1;
    }
    break;
  default:
    ERROR("Unknown Charset format");
  }

  return 0; /* not found */
}

/* Input : GID
 * Output: SID/CID (card16)
 */
card16
cff_charsets_lookup_inverse (cff_font *cff, card16 gid)
{
  if (cff->flag & (CHARSETS_ISOADOBE|CHARSETS_EXPERT|CHARSETS_EXPSUB)) {
    ERROR("Predefined CFF charsets not supported yet");
  } else if (cff->charsets == NULL) {
    ERROR("Charsets data not available");
  }

  if (gid == 0) {
    return 0;  /* .notdef */
  }

  return cff_charsets_lookup_cid(cff->charsets, gid);
}

card16
cff_charsets_lookup_cid(cff_charsets *charset, card16 gid)
{
  card16 sid = 0;
  card16 i;

  switch (charset->format) {
  case 0:
    if (gid - 1 >= charset->num_entries)
      ERROR("Invalid GID.");
    sid = charset->data.glyphs[gid - 1];
    break;
  case 1:
    for (i = 0; i < charset->num_entries; i++) {
      if (gid <= charset->data.range1[i].n_left + 1) {
        sid = gid + charset->data.range1[i].first - 1;
        break;
      }
      gid -= charset->data.range1[i].n_left + 1;
    }
    if (i == charset->num_entries)
      ERROR("Invalid GID");
    break;
  case 2:
    for (i = 0; i < charset->num_entries; i++) {
      if (gid <= charset->data.range2[i].n_left + 1) {
        sid = gid + charset->data.range2[i].first - 1;
        break;
      }
      gid -= charset->data.range2[i].n_left + 1;
    }
    if (i == charset->num_entries)
      ERROR("Invalid GID");
    break;
  default:
    ERROR("Unknown Charset format");
  }

  return sid;
}

void
cff_release_charsets (cff_charsets *charset)
{
  if (charset) {
    switch (charset->format) {
    case 0:
      if (charset->data.glyphs)
        RELEASE(charset->data.glyphs);
      break;
    case 1:
      if (charset->data.range1)
        RELEASE(charset->data.range1);
      break;
    case 2:
      if (charset->data.range2)
        RELEASE(charset->data.range2);
      break;
    default:
      break;
    }
    RELEASE(charset);
  }
}

/* CID-Keyed font specific */
int cff_read_fdselect (cff_font *cff)
{
  cff_fdselect *fdsel;
  int offset, length;
  card16 i;

  if (cff->topdict == NULL)
    ERROR("Top DICT not available");

  if (!(cff->flag & FONTTYPE_CIDFONT))
    return 0;

  offset = cff_dict_get(cff->topdict, "FDSelect", 0);
  cff_seek_set(cff, offset);
  cff->fdselect = fdsel = NEW(1, cff_fdselect);
  fdsel->format = get_unsigned_byte(cff->stream);

  length = 1;

  switch (fdsel->format) {
  case 0:
    fdsel->num_entries = cff->num_glyphs;
    (fdsel->data).fds = NEW(fdsel->num_entries, card8);
    for (i=0;i<(fdsel->num_entries);i++) {
      (fdsel->data).fds[i] = get_unsigned_byte(cff->stream);
    }
    length += fdsel->num_entries;
    break;
  case 3:
    {
      cff_range3 *ranges;
      fdsel->num_entries = get_unsigned_pair(cff->stream);
      fdsel->data.ranges = ranges = NEW(fdsel->num_entries, cff_range3);
      for (i=0;i<(fdsel->num_entries);i++) {
        ranges[i].first = get_unsigned_pair(cff->stream);
        ranges[i].fd = get_unsigned_byte(cff->stream);
      }
      if (ranges[0].first != 0)
        ERROR("Range not starting with 0.");
      if (cff->num_glyphs != get_unsigned_pair(cff->stream))
        ERROR("Sentinel value mismatched with number of glyphs.");
      length += (fdsel->num_entries) * 3 + 4;
    }
    break;
  default:
    RELEASE(fdsel);
    ERROR("Unknown FDSelect format.");
    break;
  }

  return length;
}

int cff_pack_fdselect (cff_font *cff, card8 *dest, int destlen)
{
  cff_fdselect *fdsel;
  int len = 0;
  card16 i;

  if (cff->fdselect == NULL)
    return 0;

  if (destlen < 1)
    ERROR("in cff_pack_fdselect(): Buffur overflow");

  fdsel = cff->fdselect;

  dest[len++] = fdsel->format;
  switch (fdsel->format) {
  case 0:
    if (fdsel->num_entries != cff->num_glyphs)
      ERROR("in cff_pack_fdselect(): Invalid data");
    if (destlen < len + fdsel->num_entries)
      ERROR("in cff_pack_fdselect(): Buffer overflow");
    for (i=0;i<fdsel->num_entries;i++) {
      dest[len++] = (fdsel->data).fds[i];
    }
    break;
  case 3:
    {
      if (destlen < len + 2)
        ERROR("in cff_pack_fdselect(): Buffer overflow");
      len += 2;
      for (i=0;i<(fdsel->num_entries);i++) {
        if (destlen < len + 3)
          ERROR("in cff_pack_fdselect(): Buffer overflow");
        dest[len++] = ((fdsel->data).ranges[i].first >> 8) & 0xff;
        dest[len++] = (fdsel->data).ranges[i].first & 0xff;
        dest[len++] = (fdsel->data).ranges[i].fd;
      }
      if (destlen < len + 2)
        ERROR("in cff_pack_fdselect(): Buffer overflow");
      dest[len++]  = (cff->num_glyphs >> 8) & 0xff;
      dest[len++]  = cff->num_glyphs & 0xff;
      dest[1] = ((len/3 - 1) >> 8) & 0xff;
      dest[2] = (len/3 - 1) & 0xff;
    }
    break;
  default:
    ERROR("Unknown FDSelect format.");
    break;
  }

  return len;
}

void cff_release_fdselect (cff_fdselect *fdselect)
{
  if (fdselect) {
    if (fdselect->format == 0) {
      if (fdselect->data.fds) RELEASE(fdselect->data.fds);
    } else if (fdselect->format == 3) {
      if (fdselect->data.ranges) RELEASE(fdselect->data.ranges);
    }
    RELEASE(fdselect);
  }
}

card8 cff_fdselect_lookup (cff_font *cff, card16 gid)
{
  card8 fd = 0xff;
  cff_fdselect *fdsel;

  if (cff->fdselect == NULL)
    ERROR("in cff_fdselect_lookup(): FDSelect not available");

  fdsel = cff->fdselect;

  if (gid >= cff->num_glyphs)
    ERROR("in cff_fdselect_lookup(): Invalid glyph index");

  switch (fdsel->format) {
  case 0:
    fd = fdsel->data.fds[gid];
    break;
  case 3:
    {
      if (gid == 0) {
        fd = (fdsel->data).ranges[0].fd;
      } else {
        card16 i;
        for (i=1;i<(fdsel->num_entries);i++) {
          if (gid < (fdsel->data).ranges[i].first)
            break;
        }
        fd = (fdsel->data).ranges[i-1].fd;
      }
    }
    break;
  default:
    ERROR("in cff_fdselect_lookup(): Invalid FDSelect format");
    break;
  }

  if (fd >= cff->num_fds)
    ERROR("in cff_fdselect_lookup(): Invalid Font DICT index");

  return fd;
}

int cff_read_subrs (cff_font *cff)
{
  int len = 0;
  int offset;
  int i;

  if ((cff->flag & FONTTYPE_CIDFONT) && cff->fdarray == NULL) {
    cff_read_fdarray(cff);
  }

  if (cff->private == NULL)
    cff_read_private(cff);

  if (cff->gsubr == NULL) {
    cff_seek_set(cff, cff->gsubr_offset);
    cff->gsubr = cff_get_index(cff);
  }
    
  cff->subrs = NEW(cff->num_fds, cff_index *);
  if (cff->flag & FONTTYPE_CIDFONT) {
    for (i=0;i<cff->num_fds;i++) {
      if (cff->private[i] == NULL ||
          !cff_dict_known(cff->private[i], "Subrs")) {
        (cff->subrs)[i] = NULL;
      } else {
        offset = cff_dict_get(cff->fdarray[i], "Private", 1);
        offset += cff_dict_get(cff->private[i], "Subrs", 0);
        cff_seek_set(cff, offset);
        (cff->subrs)[i] = cff_get_index(cff);
        len += cff_index_size((cff->subrs)[i]);
      }
    }
  } else {
    if (cff->private[0] == NULL ||
        !cff_dict_known(cff->private[0], "Subrs")) {
      (cff->subrs)[0] = NULL;
    } else {
      offset = cff_dict_get(cff->topdict, "Private", 1);
      offset += cff_dict_get(cff->private[0], "Subrs", 0);
      cff_seek_set(cff, offset);
      (cff->subrs)[0] = cff_get_index(cff);
      len += cff_index_size((cff->subrs)[0]);
    }
  }

  return len;
}

int cff_read_fdarray (cff_font *cff)
{
  int len = 0;
  cff_index *idx;
  int offset, size;
  card16 i;

  if (cff->topdict == NULL)
    ERROR("in cff_read_fdarray(): Top DICT not found");

  if (!(cff->flag & FONTTYPE_CIDFONT))
    return 0;

  /* must exist */
  offset = cff_dict_get(cff->topdict, "FDArray", 0);
  cff_seek_set(cff, offset);
  idx = cff_get_index(cff);
  cff->num_fds = (card8)idx->count;
  cff->fdarray = NEW(idx->count, cff_dict *);
  for (i=0;i<idx->count;i++) {
    card8 *data = idx->data + (idx->offset)[i] - 1;
    size = (idx->offset)[i+1] - (idx->offset)[i];
    if (size > 0) {
      (cff->fdarray)[i] = cff_dict_unpack(data, data+size);
    } else {
      (cff->fdarray)[i] = NULL;
    }
  }
  len = cff_index_size(idx);
  cff_release_index(idx);

  return len;
}

int cff_read_private (cff_font *cff)
{
  int len = 0;
  card8 *data;
  int offset, size;

  if (cff->flag & FONTTYPE_CIDFONT) {
    int i;

    if (cff->fdarray == NULL)
      cff_read_fdarray(cff);

    cff->private = NEW(cff->num_fds, cff_dict *);
    for (i=0;i<cff->num_fds;i++) {
      if (cff->fdarray[i] != NULL &&
          cff_dict_known(cff->fdarray[i], "Private") &&
          (size = cff_dict_get(cff->fdarray[i], "Private", 0))
          > 0) {
        offset = cff_dict_get(cff->fdarray[i], "Private", 1);
        cff_seek_set(cff, offset);
        data = NEW(size, card8);
        if (cff_read_data(data, size, cff) != size)
          ERROR("reading file failed");
        (cff->private)[i] = cff_dict_unpack(data, data+size);
        RELEASE(data);
        len += size;
      } else {
        (cff->private)[i] = NULL;
      }
    }
  } else {
    cff->num_fds = 1;
    cff->private = NEW(1, cff_dict *);
    if (cff_dict_known(cff->topdict, "Private") &&
        (size = cff_dict_get(cff->topdict, "Private", 0)) > 0) {
      offset = cff_dict_get(cff->topdict, "Private", 1);
      cff_seek_set(cff, offset);
      data = NEW(size, card8);
      if (cff_read_data(data, size, cff) != size)
        ERROR("reading file failed");
      cff->private[0] = cff_dict_unpack(data, data+size);
      RELEASE(data);
      len += size;
    } else {
      (cff->private)[0] = NULL;
      len = 0;
    }
  }

  return len;
}
