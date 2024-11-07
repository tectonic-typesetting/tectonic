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

#include "dpx-tt_post.h"

#include <assert.h>
#include <stdlib.h>

#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-sfnt.h"

static const char *macglyphorder[258];

/* offset from begenning of the post table */
#define NAME_STR_OFFSET  32

static int
read_v2_post_names (struct tt_post_table *post, sfnt *sfont)
{
  USHORT i, idx, *indices, maxidx;
  int    len;

  post->numberOfGlyphs = sfnt_get_ushort(sfont);

  indices     = NEW(post->numberOfGlyphs, USHORT);
  maxidx = 257;
  for (i = 0; i < post->numberOfGlyphs; i++) {
    idx = sfnt_get_ushort(sfont);
    if (idx >= 258) {
      if (idx > maxidx)
        maxidx = idx;
      /* Tectonic: #if 0 stanza removed */
    }
    indices[i] = idx;
  }

  post->count = maxidx - 257;
  if (post->count < 1) {
    post->names = NULL;
  } else {
    post->names = NEW(post->count, char *);
    for (i = 0; i < post->count; i++) { /* read Pascal strings */
      len = sfnt_get_byte(sfont);
      if (len > 0) {
        post->names[i] = NEW(len + 1, char);
        sfnt_read(post->names[i], len, sfont);
        post->names[i][len] = 0;
      } else {
        post->names[i] = NULL;
      }
    }
  }

  post->glyphNamePtr = NEW(post->numberOfGlyphs, const char *);
  for (i = 0; i < post->numberOfGlyphs; i++) {
    idx = indices[i];
    if (idx < 258) {
      post->glyphNamePtr[i] = macglyphorder[idx];
    } else if (idx - 258 < post->count) {
      post->glyphNamePtr[i] = post->names[idx - 258];
    } else {
      dpx_warning("Invalid glyph name index number: %u (>= %u)",
           idx, post->count + 258);
      free(indices);
      return -1;
    }
  }
  free(indices);

  return 0;
}

struct tt_post_table *
tt_read_post_table (sfnt *sfont)
{
  struct tt_post_table *post;

  /* offset = */ sfnt_locate_table(sfont, "post");

  post   = NEW(1, struct tt_post_table);

  post->Version            = sfnt_get_ulong(sfont); /* Fixed */
  post->italicAngle        = sfnt_get_ulong(sfont); /* Fixed */
  post->underlinePosition  = sfnt_get_short(sfont); /* FWord */
  post->underlineThickness = sfnt_get_short(sfont); /* FWord */
  post->isFixedPitch       = sfnt_get_ulong(sfont);
  post->minMemType42       = sfnt_get_ulong(sfont);
  post->maxMemType42       = sfnt_get_ulong(sfont);
  post->minMemType1        = sfnt_get_ulong(sfont);
  post->maxMemType1        = sfnt_get_ulong(sfont);

  post->numberOfGlyphs    = 0;
  post->glyphNamePtr      = NULL;
  post->count             = 0;
  post->names             = NULL;

  if (post->Version == 0x00010000UL) {
    post->numberOfGlyphs  = 258; /* wrong */
    post->glyphNamePtr    = macglyphorder;
  } else if (post->Version == 0x00028000UL) {
    dpx_warning("TrueType 'post' version 2.5 found (deprecated)");
  } else if (post->Version == 0x00020000UL) {
    if (read_v2_post_names(post, sfont) < 0) {
      dpx_warning("Invalid version 2.0 'post' table");
      tt_release_post_table(post);
      post = NULL;
    }
  } else if (post->Version == 0x00030000UL || /* no glyph names provided */
             post->Version == 0x00040000UL) { /* Apple format for printer-based fonts */
    /* don't bother constructing char names, not sure if they'll ever be needed */
  } else { /* some broken font files have 0x00000000UL and perhaps other values */
    dpx_warning("Unknown 'post' version: %08X, assuming version 3.0", post->Version);
  }

  return post;
}

USHORT
tt_lookup_post_table (struct tt_post_table *post, const char *glyphname)
{
  USHORT  gid;

  assert(post && glyphname);

  for (gid = 0; gid < post->numberOfGlyphs; gid++) {
    if (post->glyphNamePtr[gid] && streq_ptr(glyphname, post->glyphNamePtr[gid])) {
      return  gid;
    }
  }

  return 0;
}

char*
tt_get_glyphname (struct tt_post_table *post, USHORT gid)
{
  if (gid < post->numberOfGlyphs && post->glyphNamePtr[gid])
    return xstrdup(post->glyphNamePtr[gid]);
  return NULL;
}

void
tt_release_post_table (struct tt_post_table *post)
{
  USHORT i;

  assert(post);

  if (post->glyphNamePtr && post->Version != 0x00010000UL)
    free((void *)post->glyphNamePtr);
  if (post->names) {
    for (i = 0; i < post->count; i++) {
      free(post->names[i]);
    }
    free(post->names);
  }
  post->count        = 0;
  post->glyphNamePtr = NULL;
  post->names        = NULL;

  free(post);

  return;
}

/* Macintosh glyph order - from apple's TTRefMan */
static const char *
macglyphorder[258] = {
  /* 0x0000 */
  ".notdef", ".null", "nonmarkingreturn", "space", "exclam", "quotedbl",
  "numbersign", "dollar", "percent", "ampersand", "quotesingle",
  "parenleft", "parenright", "asterisk", "plus", "comma",
  /* 0x0010 */
  "hyphen", "period", "slash", "zero", "one", "two", "three", "four",
  "five", "six", "seven", "eight", "nine", "colon", "semicolon", "less",
  /* 0x0020 */
  "equal", "greater", "question", "at", "A", "B", "C", "D",
  "E", "F", "G", "H", "I", "J", "K", "L",
  /* 0x0030 */
  "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
  "Y", "Z", "bracketleft", "backslash",
  /* 0x0040 */
  "bracketright", "asciicircum", "underscore", "grave",
  "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
  /* 0x0050 */
  "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",
  "y", "z", "braceleft", "bar",
  /* 0x0060 */
  "braceright", "asciitilde", "Adieresis", "Aring", "Ccedilla",
  "Eacute", "Ntilde", "Odieresis", "Udieresis", "aacute", "agrave",
  "acircumflex", "adieresis", "atilde", "aring", "ccedilla",
  /* 0x0070 */
  "eacute", "egrave", "ecircumflex", "edieresis", "iacute", "igrave",
  "icircumflex", "idieresis", "ntilde", "oacute", "ograve", "ocircumflex",
  "odieresis", "otilde", "uacute", "ugrave",
  /* 0x0080 */
  "ucircumflex", "udieresis", "dagger", "degree", "cent", "sterling",
  "section", "bullet", "paragraph", "germandbls", "registered",
  "copyright", "trademark", "acute", "dieresis", "notequal",
  /* 0x0090 */
  "AE", "Oslash", "infinity", "plusminus", "lessequal", "greaterequal",
  "yen", "mu", "partialdiff", "summation", "product", "pi", "integral",
  "ordfeminine", "ordmasculine", "Omega",
  /* 0x00a0 */
  "ae", "oslash", "questiondown", "exclamdown", "logicalnot", "radical",
  "florin", "approxequal", "Delta", "guillemotleft", "guillemotright",
  "ellipsis", "nonbreakingspace", "Agrave", "Atilde", "Otilde",
  /* 0x00b0 */
  "OE", "oe", "endash", "emdash", "quotedblleft", "quotedblright",
  "quoteleft", "quoteright", "divide", "lozenge", "ydieresis",
  "Ydieresis", "fraction", "currency", "guilsinglleft", "guilsinglright",
  /* 0x00c0 */
  "fi", "fl", "daggerdbl", "periodcentered", "quotesinglbase",
  "quotedblbase", "perthousand", "Acircumflex", "Ecircumflex", "Aacute",
  "Edieresis", "Egrave", "Iacute", "Icircumflex", "Idieresis", "Igrave",
  /* 0x00d0 */
  "Oacute", "Ocircumflex", "apple", "Ograve", "Uacute", "Ucircumflex",
  "Ugrave", "dotlessi", "circumflex", "tilde", "macron", "breve",
  "dotaccent", "ring", "cedilla", "hungarumlaut",
  /* 0x00e0 */
  "ogonek", "caron", "Lslash", "lslash", "Scaron", "scaron", "Zcaron",
  "zcaron", "brokenbar", "Eth", "eth", "Yacute", "yacute", "Thorn",
  "thorn", "minus",
  /* 0x00f0 */
  "multiply", "onesuperior", "twosuperior", "threesuperior", "onehalf",
  "onequarter", "threequarters", "franc", "Gbreve", "gbreve", "Idotaccent",
  "Scedilla", "scedilla", "Cacute", "cacute", "Ccaron",
  /* 0x0100 */
  "ccaron", "dcroat"
};
