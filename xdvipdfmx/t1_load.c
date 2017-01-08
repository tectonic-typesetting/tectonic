/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>
#include <math.h>

#include "system.h"
#include "mfileio.h"
#include "mem.h"
#include "error.h"
#include "numbers.h"

#include "pdfobj.h"
#include "pdffont.h"

#include "pdfencoding.h"
#include "unicode.h"

#include "dpxutil.h"

#include "pst_obj.h"
#include "pst.h"

#include "cff_limits.h"
#include "cff_types.h"
#include "cff_dict.h"
#include "cff.h"

#include "t1_load.h"

/* Migrated from t1crypt */
#define T1_EEKEY   55665u
#define T1_CHARKEY 4330u

#if  0
/* We no longer need encryption. */
static unsigned short r = T1_EEKEY, c1 = 52845, c2 = 22719;

static unsigned char t1_encrypt (unsigned char plain)
{
  unsigned char cipher;
  cipher = (plain ^ (r >> 8));
  r = (cipher + r) * c1 + c2;
  return cipher;
}

static void t1_crypt_init (unsigned short key)
{
  r = key;
}
#endif /* 0 */

static void
t1_decrypt (unsigned short key,
            unsigned char *dst, const unsigned char *src,
            int skip, int len)
{
  len -= skip;
  while (skip--)
    key = (key + *src++) * 52845u + 22719u;
  while (len--) {
    unsigned char c = *src++;
    *dst++ = (c ^ (key >> 8));
    key = (key + c) * 52845u + 22719u;
  }
}
/* T1CRYPT */

#define MATCH_NAME(t,n) ((t) && PST_NAMETYPE((t))    && !strncmp(pst_data_ptr((t)),(n),strlen((n))))
#define MATCH_OP(t,n)   ((t) && PST_UNKNOWNTYPE((t)) && !strncmp(pst_data_ptr((t)),(n),strlen((n))))

#define RELEASE_TOK(t) if ((t) != NULL) {\
  pst_release_obj((t));\
  (t) = NULL;\
}

static char *
get_next_key (unsigned char **start, unsigned char *end)
{
  char *key = NULL;
  pst_obj *tok;

  while (*start < end &&
         (tok = pst_get_token(start, end)) != NULL) {
    if (PST_NAMETYPE(tok)) {
      key = (char *) pst_getSV(tok);
      RELEASE_TOK(tok);
      break;
    }
    RELEASE_TOK(tok);
  }

  return key;
}

static int
seek_operator (unsigned char **start, unsigned char *end, const char *op)
{
  pst_obj *tok = NULL;

  while (*start < end &&
         (tok = pst_get_token(start, end)) != NULL) {
    if (MATCH_OP(tok, op)) {
      break;
    }
    RELEASE_TOK(tok);
  }

  if (tok == NULL)
    return -1;
  RELEASE_TOK(tok);

  return 0;
}


static int
parse_svalue (unsigned char **start, unsigned char *end, char **value)
{
  pst_obj *tok;

  tok = pst_get_token(start, end);
  if (tok == NULL)
    return -1;
  else if (PST_NAMETYPE(tok) || PST_STRINGTYPE(tok))
    *value = (char *) pst_getSV(tok);
  else {
    RELEASE_TOK(tok);
    return -1;
  }
  RELEASE_TOK(tok);

  return 1;
}

static int
parse_bvalue (unsigned char **start, unsigned char *end, double *value)
{
  pst_obj *tok;

  tok = pst_get_token(start, end);
  if (tok == NULL)
    return -1;
  else if (PST_BOOLEANTYPE(tok))
    *value = (double) pst_getIV(tok);
  else {
    RELEASE_TOK(tok);
    return -1;
  }
  RELEASE_TOK(tok);

  return 1;
}

static int
parse_nvalue (unsigned char **start, unsigned char *end, double *value, int max)
{
  int argn = 0;
  pst_obj *tok;

  tok = pst_get_token(start, end);
  if (tok == NULL)
    return -1;
  /*
   * All array elements must be numeric token. (ATM compatible)
   */
  if (PST_NUMBERTYPE(tok) && max > 0) {
    value[0] = (double) pst_getRV(tok);
    argn = 1;
  } else if (PST_MARKTYPE(tok)) {
    /* It does not distinguish '[' and '{'... */
    RELEASE_TOK(tok);
    while (*start < end &&
           (tok = pst_get_token(start, end)) != NULL &&
           PST_NUMBERTYPE(tok) && argn < max) {
      value[argn++] = (double) pst_getRV(tok);
      RELEASE_TOK(tok);
    }
    if (tok == NULL)
      return -1;
    if (!MATCH_OP(tok, "]") && !MATCH_OP(tok, "}")) {
      argn = -1;
    }
  }
  RELEASE_TOK(tok);

  return argn;
}

static const char *const StandardEncoding[256] = {
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", "space", "exclam", "quotedbl",
  "numbersign", "dollar", "percent", "ampersand", "quoteright",
  "parenleft", "parenright", "asterisk", "plus", "comma",
  "hyphen", "period", "slash", "zero", "one",
  "two", "three", "four", "five", "six",
  "seven", "eight", "nine", "colon", "semicolon",
  "less", "equal", "greater", "question", "at",
  "A", "B", "C", "D", "E",
  "F", "G", "H", "I", "J",
  "K", "L", "M", "N", "O",
  "P", "Q", "R", "S", "T",
  "U", "V", "W", "X", "Y",
  "Z", "bracketleft", "backslash", "bracketright", "asciicircum",
  "underscore", "quoteleft", "a", "b", "c",
  "d", "e", "f", "g", "h",
  "i", "j", "k", "l", "m",
  "n", "o", "p", "q", "r",
  "s", "t", "u", "v", "w",
  "x", "y", "z", "braceleft", "bar",
  "braceright", "asciitilde", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", "exclamdown", "cent", "sterling", "fraction",
  "yen", "florin", "section", "currency", "quotesingle",
  "quotedblleft", "guillemotleft", "guilsinglleft", "guilsinglright", "fi",
  "fl", ".notdef", "endash", "dagger", "daggerdbl",
  "periodcentered", ".notdef", "paragraph", "bullet", "quotesinglbase",
  "quotedblbase", "quotedblright", "guillemotright", "ellipsis", "perthousand",
  ".notdef", "questiondown", ".notdef", "grave", "acute",
  "circumflex", "tilde", "macron", "breve", "dotaccent",
  "dieresis", ".notdef", "ring", "cedilla", ".notdef",
  "hungarumlaut", "ogonek", "caron", "emdash", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  "AE", ".notdef", "ordfeminine", ".notdef", ".notdef",
  ".notdef", ".notdef", "Lslash", "Oslash", "OE",
  "ordmasculine", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", "ae", ".notdef", ".notdef", ".notdef",
  "dotlessi", ".notdef", ".notdef", "lslash", "oslash",
  "oe", "germandbls", ".notdef", ".notdef", ".notdef",
  ".notdef"
};

static const char *const ISOLatin1Encoding[256] = {
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", "space", "exclam", "quotedbl",
  "numbersign", "dollar", "percent", "ampersand", "quotesingle",
  "parenleft", "parenright", "asterisk", "plus", "comma",
  "hyphen", "period", "slash", "zero", "one",
  "two", "three", "four", "five", "six",
  "seven", "eight", "nine", "colon", "semicolon",
  "less", "equal", "greater", "question", "at",
  "A", "B", "C", "D", "E",
  "F", "G", "H", "I", "J",
  "K", "L", "M", "N", "O",
  "P", "Q", "R", "S", "T",
  "U", "V", "W", "X", "Y",
  "Z", "bracketleft", "backslash", "bracketright", "asciicircum",
  "underscore", "grave", "a", "b", "c",
  "d", "e", "f", "g", "h",
  "i", "j", "k", "l", "m",
  "n", "o", "p", "q", "r",
  "s", "t", "u", "v", "w",
  "x", "y", "z", "braceleft", "bar",
  "braceright", "asciitilde", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", ".notdef",
  ".notdef", ".notdef", ".notdef", ".notdef", "dotlessi",
  "quoteleft", "quoteright", "circumflex", "tilde", "macron",
  "breve", "dotaccent", "dieresis", ".notdef", "ring",
  "cedilla", ".notdef", "hungarumlaut", "ogonek", "caron",
  "space", "exclamdown", "cent", "sterling", "currency",
  "yen", "brokenbar", "section", "dieresis", "copyright",
  "ordfeminine", "guillemotleft", "logicalnot", "hyphen",
  "registered",
  "macron", "degree", "plusminus", "twosuperior", "threesuperior",
  "acute", "mu", "paragraph", "periodcentered", "cedilla",
  "onesuperior", "ordmasculine", "guillemotright", "onequarter",
  "onehalf",
  "threequarters", "questiondown", "Agrave", "Aacute", "Acircumflex",
  "Atilde", "Adieresis", "Aring", "AE", "Ccedilla",
  "Egrave", "Eacute", "Ecircumflex", "Edieresis", "Igrave",
  "Iacute", "Icircumflex", "Idieresis", "Eth", "Ntilde",
  "Ograve", "Oacute", "Ocircumflex", "Otilde", "Odieresis",
  "multiply", "Oslash", "Ugrave", "Uacute", "Ucircumflex",
  "Udieresis", "Yacute", "Thorn", "germandbls", "agrave",
  "aacute", "acircumflex", "atilde", "adieresis", "aring",
  "ae", "ccedilla", "egrave", "eacute", "ecircumflex",
  "edieresis", "igrave", "iacute", "icircumflex", "idieresis",
  "eth", "ntilde", "ograve", "oacute", "ocircumflex",
  "otilde", "odieresis", "divide", "oslash", "ugrave",
  "uacute", "ucircumflex", "udieresis", "yacute", "thorn",
  "ydieresis"
};

/* Treat cases such as "dup num num getinterval num exch putinterval"
 * or "dup num exch num get put"
 */
static int
try_put_or_putinterval (char **enc_vec, unsigned char **start, unsigned char *end)
{
  pst_obj *tok;
  int i, num1, num2, num3;

  tok = pst_get_token(start, end);
  if (!tok || !PST_INTEGERTYPE(tok) ||
      (num1 = pst_getIV(tok)) > 255 || num1 < 0) {
    RELEASE_TOK(tok);
    return -1;
  }
  RELEASE_TOK(tok);

  tok = pst_get_token(start, end);
  if (!tok) {
    return -1;
  } else if (MATCH_OP(tok, "exch")) {
    /* dup num exch num get put */
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!tok || !PST_INTEGERTYPE(tok) ||
        (num2 = pst_getIV(tok)) > 255 || num2 < 0) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!MATCH_OP(tok, "get")) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!MATCH_OP(tok, "put")) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);

    if (enc_vec[num1])
      RELEASE(enc_vec[num1]);
    enc_vec[num1] = xstrdup(enc_vec[num2]);
  } else if (PST_INTEGERTYPE(tok) &&
             (num2 = pst_getIV(tok)) + num1 <= 255 && num2 >= 0) {
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!MATCH_OP(tok, "getinterval")) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!tok || !PST_INTEGERTYPE(tok) ||
        (num3 = pst_getIV(tok)) + num2 > 255 || num3 < 0) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!MATCH_OP(tok, "exch")) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!MATCH_OP(tok, "putinterval")) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);

    for (i = 0; i < num2; i++) {
      if (enc_vec[num1 + i]) { /* num1 + i < 256 here */
        if (enc_vec[num3 + i]) { /* num3 + i < 256 here */
          RELEASE(enc_vec[num3 + i]);
          enc_vec[num3+i] = NULL;
        }
        enc_vec[num3 + i] = xstrdup(enc_vec[num1 + i]);
      }
    }
  } else {
    RELEASE_TOK(tok);
    return -1;
  }

  return 0;
}

static int
parse_encoding (char **enc_vec, unsigned char **start, unsigned char *end)
{
  pst_obj *tok;
  int      code;

  /*
   *  StandardEncoding def
   * or
   *  ISOLatin1Encoding def
   * or
   *  0 1 255 {1 index exch /.notdef put } for
   *  dup int name put
   *  ...
   *  [readonly] def
   */
  tok = pst_get_token(start, end);
  if (MATCH_OP(tok, "StandardEncoding")) {
    RELEASE_TOK(tok);
    if (enc_vec) {
      for (code = 0; code < 256; code++) {
        if (StandardEncoding[code] &&
            strcmp(StandardEncoding[code], ".notdef") != 0) {
          enc_vec[code] = NEW(strlen(StandardEncoding[code])+1, char);
          strcpy(enc_vec[code], StandardEncoding[code]);
        } else {
          enc_vec[code] = NULL;
        }
      }
    }
  } else if (MATCH_OP(tok, "ISOLatin1Encoding")) {
    RELEASE_TOK(tok);
    if (enc_vec) {
      for (code = 0; code < 256; code++) {
        if (ISOLatin1Encoding[code] &&
            strcmp(ISOLatin1Encoding[code], ".notdef") != 0) {
          enc_vec[code] = NEW(strlen(ISOLatin1Encoding[code])+1, char);
          strcpy(enc_vec[code], ISOLatin1Encoding[code]);
        } else {
          enc_vec[code] = NULL;
        }
      }
    }
  } else if (MATCH_OP(tok, "ExpertEncoding")) {
    RELEASE_TOK(tok);
    if (enc_vec) {
      WARN("ExpertEncoding not supported.");
      RELEASE_TOK(tok);
      return -1;
    }
    /*
     * Not supported yet.
     */
  } else {
    RELEASE_TOK(tok);
    seek_operator(start, end, "array");
    /*
     * Pick all seaquences that matches "dup n /Name put" until
     * occurrence of "def" or "readonly".
     */
    while (*start < end &&
           (tok = pst_get_token(start, end)) != NULL) {
      if (MATCH_OP(tok, "def") || MATCH_OP(tok, "readonly")) {
        RELEASE_TOK(tok);
        break;
      } else if (!MATCH_OP(tok, "dup")) {
        RELEASE_TOK(tok);
        continue;
      }
      RELEASE_TOK(tok);

      /* cmctt10.pfb for examples contains the following PS code
       *     dup num num getinterval num exch putinterval
       *     dup num exch num get put
       */
      tok = pst_get_token(start, end);
      if (MATCH_OP(tok, "dup")) { /* possibly putinterval type */
        if (enc_vec == NULL) {
          WARN ("This kind of type1 fonts are not supported as native fonts.\n"
                "                   They are supported if used with tfm fonts.\n");
        } else {
          try_put_or_putinterval(enc_vec, start, end);
        }
        RELEASE_TOK(tok)
        continue;
      } else if (!tok || !PST_INTEGERTYPE(tok) ||
          (code = pst_getIV(tok)) > 255 || code < 0) {
        RELEASE_TOK(tok);
        continue;
      }
      RELEASE_TOK(tok);

      tok = pst_get_token(start, end);
      if (!tok || !PST_NAMETYPE(tok)) {
        RELEASE_TOK(tok);
        continue;
      }
      if (enc_vec) {
        if (enc_vec[code])
          RELEASE(enc_vec[code]);
        enc_vec[code] = (char *) pst_getSV(tok);
      }
      RELEASE_TOK(tok);

      tok = pst_get_token(start, end);
      if (!MATCH_OP(tok, "put")) {
        if (enc_vec[code]) {
          RELEASE(enc_vec[code]);
          enc_vec[code] = NULL;
        }
        RELEASE_TOK(tok);
        continue;
      }
      RELEASE_TOK(tok);
    }
  }

  return 0;
}

#ifndef CS_STR_LEN_MAX
#define CS_STR_LEN_MAX 65536UL
#endif
#ifndef CFF_GLYPH_MAX
#define CFF_GLYPH_MAX  CFF_SID_MAX
#endif

static int
parse_subrs (cff_font *font,
             unsigned char **start, unsigned char *end, int lenIV, int mode)
{
  cff_index *subrs;
  pst_obj   *tok;
  int        i, count, offset, max_size;
  int       *offsets, *lengths;
  card8     *data;

  tok = pst_get_token(start, end);
  if (!PST_INTEGERTYPE(tok) || pst_getIV(tok) < 0) {
    WARN("Parsing Subrs failed.");
    RELEASE_TOK(tok);
    return -1;
  }

  count = pst_getIV(tok);
  RELEASE_TOK(tok);

  if (count == 0) {
    font->subrs[0] = NULL;
    return 0;
  }

  tok = pst_get_token(start, end);
  if (!MATCH_OP(tok, "array")) {
    RELEASE_TOK(tok);
    return -1;
  }
  RELEASE_TOK(tok);

  if (mode != 1) {
    max_size = CS_STR_LEN_MAX;
    data     = NEW(max_size, card8);
    offsets  = NEW(count, int);
    lengths  = NEW(count, int);
    memset(offsets, 0, sizeof(int)*count);
    memset(lengths, 0, sizeof(int)*count);
  } else {
    max_size = 0;
    data     = NULL;
    offsets  = NULL;
    lengths  = NULL;
  }

  offset = 0;
  /* dup subr# n-bytes RD n-binary-bytes NP */
  for (i = 0; i < count;) {
    int idx, len;

    tok = pst_get_token(start, end);
    if (!tok) {
      if (data)    RELEASE(data);
      if (offsets) RELEASE(offsets);
      if (lengths) RELEASE(lengths);
      return -1;
    } else if (MATCH_OP(tok, "ND") ||
               MATCH_OP(tok, "|-") || MATCH_OP(tok, "def")) {
      RELEASE_TOK(tok);
      break;
    } else if (!MATCH_OP(tok, "dup")) {
      RELEASE_TOK(tok);
      continue;
    }
    RELEASE_TOK(tok);

    /* Found "dup" */
    tok = pst_get_token(start, end);
    if (!PST_INTEGERTYPE(tok) || pst_getIV(tok) < 0 ||
        pst_getIV(tok) >= count) {
      RELEASE_TOK(tok);
      if (data)    RELEASE(data);
      if (offsets) RELEASE(offsets);
      if (lengths) RELEASE(lengths);
      return -1;
    }
    idx = pst_getIV(tok);
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!PST_INTEGERTYPE(tok) || pst_getIV(tok) < 0 ||
        pst_getIV(tok) > CS_STR_LEN_MAX) {
      RELEASE_TOK(tok);
      return -1;
    }
    len = pst_getIV(tok);
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!MATCH_OP(tok, "RD") && !MATCH_OP(tok, "-|") &&
        seek_operator(start, end, "readstring") < 0) {
      RELEASE_TOK(tok);
      if (data)    RELEASE(data);
      if (offsets) RELEASE(offsets);
      if (lengths) RELEASE(lengths);
      return -1;
    }
    RELEASE_TOK(tok);

    *start += 1;
    if (*start + len >= end) {
      if (data)    RELEASE(data);
      if (offsets) RELEASE(offsets);
      if (lengths) RELEASE(lengths);
      return -1;
    }
    if (mode != 1) {
      if (offset + len >= max_size) {
        max_size += CS_STR_LEN_MAX;
        data = RENEW(data, max_size, card8);
      }
      if (lenIV >= 0) {
        t1_decrypt(T1_CHARKEY, data+offset, *start, lenIV, len);
        offsets[idx] = offset;
        offset += (lengths[idx] = len - lenIV);
      } else if (len > 0) {
        offsets[idx] = offset;
        lengths[idx] = len;
        memcpy(&data[offset], *start, len);
        offset += len;
      }
    }
    *start += len;
    i++;
  }

  if (mode != 1) {
    if (font->subrs[0] == NULL) {
      subrs = font->subrs[0] = cff_new_index(count);
      subrs->data = NEW(offset, card8);
      offset = 0;
      for (i = 0; i < count; i++) {
        subrs->offset[i] = offset + 1;
        if (lengths[i] > 0) {
          memcpy(subrs->data + offset, data + offsets[i], lengths[i]);
          offset += lengths[i];
        }
      }
      subrs->offset[count] = offset + 1;
    } else {
      /* Adobe's OPO_____.PFB and OPBO____.PFB have two /Subrs dicts,
       * and also have /CharStrings not followed by dicts.
       * Simply ignores those data. By ChoF on 2009/04/08. */
      WARN("Already found /Subrs; ignores the other /Subrs dicts.");
    }
    RELEASE(data);
    RELEASE(offsets);
    RELEASE(lengths);
  }

  return 0;
}

static int
parse_charstrings (cff_font *font,
                   unsigned char **start, unsigned char *end, int lenIV, int mode)
{
  cff_index    *charstrings;
  cff_charsets *charset;
  pst_obj      *tok;
  int           i, count, have_notdef;
  int           max_size, offset;

  /* /CharStrings n dict dup begin
   * /GlyphName n-bytes RD -n-binary-bytes- ND
   * ...
   * end
   *  - stack - ... /CharStrings dict
   */
  tok = pst_get_token(start, end);
  if (!PST_INTEGERTYPE(tok) ||
      pst_getIV(tok) < 0 || pst_getIV(tok) > CFF_GLYPH_MAX) {
    unsigned char *s = pst_getSV(tok);
    WARN("Ignores non dict \"/CharStrings %s ...\"", s);
    RELEASE(s);
    RELEASE_TOK(tok);
    return 0;
  }
  count = pst_getIV(tok);
  RELEASE_TOK(tok);

  if (mode != 1) {
    charstrings = cff_new_index(count);
    max_size    = CS_STR_LEN_MAX;
    charstrings->data = NEW(max_size, card8);
  } else {
    charstrings = NULL;
    max_size    = 0;
  }
  font->cstrings = charstrings;

  charset = font->charsets = NEW(1, cff_charsets);
  charset->format = 0;
  charset->num_entries = count-1;
  charset->data.glyphs = NEW(count-1, s_SID);
  memset(charset->data.glyphs, 0, sizeof(s_SID)*(count-1));

  offset      = 0;
  have_notdef = 0; /* .notdef must be at gid = 0 in CFF */

  font->is_notdef_notzero = 0;
  seek_operator(start, end, "begin");
  for (i = 0; i < count; i++) {
    char *glyph_name;
    int   len, gid, j;

    /* BUG-20061126 (by ChoF):
     * Some fonts (e.g., belleek/blsy.pfb) does not have the correct number
     * of glyphs. Modify the codes even to work with these broken fonts.
     */
    tok = pst_get_token(start, end);
    glyph_name = (char *)pst_getSV(tok);

    if ((i == 0) && (glyph_name != NULL) && (strcmp (glyph_name, ".notdef") != 0))
      font->is_notdef_notzero = 1;

    if (PST_NAMETYPE(tok)) {
      RELEASE_TOK(tok);
      if (!glyph_name) {
        return -1;
      } else if (!strcmp(glyph_name, ".notdef")) {
        gid = 0;
        have_notdef = 1;
      } else if (have_notdef) {
        gid = i;
      } else if (i == count - 1) {
        WARN("No .notdef glyph???");
        return -1;
      } else {
        gid = i+1;
      }
    } else if (PST_UNKNOWNTYPE(tok) && !strcmp(glyph_name, "end")) {
      RELEASE_TOK(tok);
      break;
    } else {
      RELEASE_TOK(tok);
      return -1;
    }

    if (gid > 0)
      charset->data.glyphs[gid-1] = cff_add_string(font, glyph_name, 0);
    /*
     * We don't care about duplicate strings here since
     * later a subset font of this font will be generated.
     */

    RELEASE(glyph_name);

    tok = pst_get_token(start, end);
    if (!PST_INTEGERTYPE(tok) ||
        pst_getIV(tok) < 0 || pst_getIV(tok) > CS_STR_LEN_MAX) {
      RELEASE_TOK(tok);
      return -1;
    }
    len = pst_getIV(tok);
    RELEASE_TOK(tok);

    tok = pst_get_token(start, end);
    if (!MATCH_OP(tok, "RD") &&
        !MATCH_OP(tok, "-|") &&
        seek_operator(start, end, "readstring") < 0) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);

    if (*start + len + 1 >= end) {
      return -1;
    }
    if (mode != 1) {
      if (offset + len >= max_size) {
        max_size += MAX(len, CS_STR_LEN_MAX);
        charstrings->data = RENEW(charstrings->data, max_size, card8);
      }
      if (gid == 0) {
        if (lenIV >= 0) {
          memmove(charstrings->data + len - lenIV, charstrings->data, offset);
          for (j = 1; j <= i; j++) {
            charstrings->offset[j] += len - lenIV;
          }
        } else {
          memmove(charstrings->data + len, charstrings->data, offset);
          for (j = 1; j <= i; j++) {
            charstrings->offset[j] += len;
          }
        }
      }
    }

    *start += 1;
    if (mode != 1) {
      if (lenIV >= 0) {
        int offs = gid ? offset : 0;
        charstrings->offset[gid] = offs + 1; /* start at 1 */
        t1_decrypt(T1_CHARKEY, charstrings->data+offs, *start, lenIV, len);
        offset += len - lenIV;
      } else {
        if (gid == 0) {
          charstrings->offset[gid] = 1;
          memcpy(&charstrings->data[0], *start, len);
        } else {
          charstrings->offset[gid] = offset + 1;
          memcpy(&charstrings->data[offset], *start, len);
        }
        offset += len;
      }
    }
    *start += len;

    tok = pst_get_token(start, end);
    if (!MATCH_OP(tok, "ND") && !MATCH_OP(tok, "|-")) {
      RELEASE_TOK(tok);
      return -1;
    }
    RELEASE_TOK(tok);
  }
  if (mode != 1)
    charstrings->offset[count] = offset + 1;
  font->num_glyphs = count;

  return 0;
}

#define CHECK_ARGN_EQ(n) if (argn != (n)) {\
  WARN("%d values expected but only %d read.", (n), argn);\
  RELEASE(key);\
  return -1;\
}
#define CHECK_ARGN_GE(n) if (argn < (n)) {\
  WARN("%d values expected but only %d read.", (n), argn);\
  RELEASE(key);\
  return -1;\
}

#define MAX_ARGS 127
static int
parse_part2 (cff_font *font, unsigned char **start, unsigned char *end, int mode)
{
  char  *key;
  double argv[MAX_ARGS];
  int    argn, lenIV = 4;

  while (*start < end &&
         (key = get_next_key(start, end)) != NULL) {
    if (!strcmp(key, "Subrs")) {
      /* levIV must appear before Subrs */
      if (parse_subrs(font, start, end, lenIV, mode) < 0) {
        RELEASE(key);
        return -1;
      }
    } else if (!strcmp(key, "CharStrings")) {
      if (parse_charstrings(font, start, end, lenIV, mode) < 0) {
        RELEASE(key);
        return -1;
      }
    } else if (!strcmp(key, "lenIV")) {
      argn = parse_nvalue(start, end, argv, 1);
      CHECK_ARGN_EQ(1);
      lenIV = (int) argv[0];
    } else if (!strcmp(key, "BlueValues") ||
               !strcmp(key, "OtherBlues") ||
               !strcmp(key, "FamilyBlues") ||
               !strcmp(key, "FamilyOtherBlues") ||
               !strcmp(key, "StemSnapH") ||
               !strcmp(key, "StemSnapV")) {
      /*
       * Operand values are delta in CFF font dictionary encoding.
       */
      argn = parse_nvalue(start, end, argv, MAX_ARGS);
      CHECK_ARGN_GE(0);
      cff_dict_add(font->private[0], key, argn);
      while (argn-- > 0) {
        cff_dict_set(font->private[0], key, argn,
                     (argn == 0) ? argv[argn] : argv[argn] - argv[argn-1]);
      }
    } else if (!strcmp(key, "StdHW") ||
               !strcmp(key, "StdVW") ||
               !strcmp(key, "BlueScale") ||
               !strcmp(key, "BlueShift") ||
               !strcmp(key, "BlueFuzz")  ||
               !strcmp(key, "LanguageGroup") ||
               !strcmp(key, "ExpansionFactor")) {
      /*
       * Value of StdHW and StdVW is described as an array in the
       * Type 1 Font Specification but is a number in CFF format.
       */
      argn = parse_nvalue(start, end, argv, 1);
      CHECK_ARGN_EQ(1);
      cff_dict_add(font->private[0], key, 1);
      cff_dict_set(font->private[0], key, 0, argv[0]);
    } else if (!strcmp(key, "ForceBold")) {
      argn = parse_bvalue(start, end, &(argv[0]));
      CHECK_ARGN_EQ(1);
      if (argv[0] != 0) {
        cff_dict_add(font->private[0], key, 1);
        cff_dict_set(font->private[0], key, 0, 1);
      }
    }
    /*
     * MinFeature, RndStemUp, UniqueID, Password ignored.
     */
    RELEASE(key);
  }

  return 0;
}

#ifndef TYPE1_NAME_LEN_MAX
#define TYPE1_NAME_LEN_MAX 127
#endif

static int
parse_part1 (cff_font *font, char **enc_vec,
             unsigned char **start, unsigned char *end)
{
  char  *key, *strval;
  double argv[MAX_ARGS];
  int    argn; /* Macro CHECK_ARGN_XX assume 'argn' is used. */

  /*
   * We skip PostScript code inserted before the beginning of 
   * font dictionary so that parser will not be confused with
   * it. See LMRoman10-Regular (lmr10.pfb) for example.
   */
  if (seek_operator(start, end, "begin") < 0)
    return -1;

  while (*start < end &&
         (key = get_next_key(start, end)) != NULL) {
    if (!strcmp(key, "Encoding")) {
      if (parse_encoding(enc_vec, start, end) < 0) {
        RELEASE(key);
        return -1;
      }
    } else if (!strcmp(key, "FontName")) {
      argn = parse_svalue(start, end, &strval);
      CHECK_ARGN_EQ(1);
      if (strlen(strval) > TYPE1_NAME_LEN_MAX) {
        WARN("FontName too long: %s (%d bytes)", strval, strlen(strval));
        strval[TYPE1_NAME_LEN_MAX] = '\0';
      }
      cff_set_name(font, strval);
      RELEASE(strval);
    } else if (!strcmp(key, "FontType")) {
      argn = parse_nvalue(start, end, argv, 1);
      CHECK_ARGN_EQ(1);
      if (argv[0] != 1.0) {
        WARN("FontType %d not supported.", (int) argv[0]);
        RELEASE(key);
        return -1;
      }
#if 0
      /* DISABLED:
       *
       * Subsetted font shouldn't have UniqueID.
       */
    } else if (!strcmp(key, "UniqueID")) {
      argn = parse_nvalue(start, end, argv, 1);
      CHECK_ARGN_EQ(1);
      cff_dict_add(font->topdict, key, 1);
      cff_dict_set(font->topdict, key, 0, argv[0]);
#endif
    } else if (!strcmp(key, "ItalicAngle") ||
               !strcmp(key, "StrokeWidth") ||
               !strcmp(key, "PaintType")) {
      argn = parse_nvalue(start, end, argv, 1);
      CHECK_ARGN_EQ(1);
      if (argv[0] != 0.0) {
#if 0
        /*
         * Positive value in Bitstream CharterBT-Italic ???
         */
        if (!strcmp(key, "ItalicAngle") && argv[0] > 0) {
          WARN("Positive ItalicAngle value: %g", argv[0]);
          argv[0] *= -1;
        }
#endif
        cff_dict_add(font->topdict, key, 1);
        cff_dict_set(font->topdict, key, 0, argv[0]);
      }
    } else if (!strcmp(key, "UnderLinePosition") ||
               !strcmp(key, "UnderLineThickness")) {
      argn = parse_nvalue(start, end, argv, 1);
      CHECK_ARGN_EQ(1);
      cff_dict_add(font->topdict, key, 1);
      cff_dict_set(font->topdict, key, 0, argv[0]);
    } else if (!strcmp(key, "FontBBox")) {
      argn = parse_nvalue(start, end, argv, 4);
      CHECK_ARGN_EQ(4);
      cff_dict_add(font->topdict, key, 4);
      while (argn-- > 0) {
        cff_dict_set(font->topdict, key, argn, argv[argn]);
      }
    } else if (!strcmp(key, "FontMatrix")) {
      argn = parse_nvalue(start, end, argv, 6);
      CHECK_ARGN_EQ(6);
      if (argv[0] != 0.001 || argv[1] != 0.0 || argv[2] != 0.0 ||
          argv[3] != 0.001 || argv[4] != 0.0 || argv[5] != 0.0) {
        cff_dict_add(font->topdict, key, 6);
        while (argn-- > 0) {
          cff_dict_set(font->topdict, key, argn, argv[argn]);
        }
      }
    } else if (!strcmp(key, "version")  || !strcmp(key, "Notice") ||
               !strcmp(key, "FullName") || !strcmp(key, "FamilyName") ||
               !strcmp(key, "Weight")   || !strcmp(key, "Copyright")) {
      /*
       * FontInfo
       */
      argn = parse_svalue(start, end, &strval);
      CHECK_ARGN_EQ(1);
      {
        s_SID sid;

        cff_dict_add(font->topdict, key, 1);
        if ((sid = cff_get_sid(font, strval)) == CFF_STRING_NOTDEF)
          sid = cff_add_string(font, strval, 0); /* FIXME */
        /*
         * We don't care about duplicate strings here since
         * later a subset font of this font will be generated.
         */
        cff_dict_set(font->topdict, key, 0, sid);
      }
      RELEASE(strval);
    } else if (!strcmp(key, "IsFixedPitch")) {
      argn = parse_bvalue(start, end, &(argv[0]));
      CHECK_ARGN_EQ(1);
      if (argv[0] != 0.0) {
        cff_dict_add(font->private[0], key, 1);
        cff_dict_set(font->private[0], key, 0, 1);
      }
    }
    RELEASE(key);
  }

  return 0;
}

int
is_pfb (FILE *fp)
{
  char sig[15];
  int i, ch;

  rewind(fp);
  if ((ch = fgetc(fp)) != 128 ||
      (ch = fgetc (fp)) < 0 || ch > 3) {
    return 0;
  }
  for (i = 0; i < 4; i++) {
    if ((ch = fgetc(fp)) < 0) {
      return 0;
    }
  }
  for (i = 0; i < 14; i++) {
    if ((ch = fgetc(fp)) < 0) {
      return 0;
    }
    sig[i] = (char) ch;
  }
  if (!memcmp(sig, "%!PS-AdobeFont", 14) ||
      !memcmp(sig, "%!FontType1", 11)) {
    return 1;
  } else if (!memcmp(sig, "%!PS", 4)) {
    sig[14] = '\0';
    WARN("Ambiguous PostScript resource type: %s", sig);
    return 1;
  } else {
    WARN("Not a PFB font file?");
    return 0;
  }
}


#define PFB_SEG_TYPE_ASCII  1
#define PFB_SEG_TYPE_BINARY 2

static unsigned char *
get_pfb_segment (FILE *fp, int expected_type, int *length)
{
  unsigned char *buffer;
  int bytesread;

  buffer = NULL; bytesread = 0;
  for (;;) {
    int ch;

    ch = fgetc(fp);
    if (ch < 0) {
      break;
    } else if (ch != 128) {
      ERROR("Not a pfb file?");
    }
    ch = fgetc(fp);
    if (ch < 0 || ch != expected_type) {
      seek_relative(fp, -2);
      break;
    }
    {
      int  slen, rlen;
      int  i;

      slen = 0;
      for (i = 0; i < 4; i++) {
        if ((ch = fgetc(fp)) < 0) {
          if (buffer)
            RELEASE(buffer);
          return NULL;
        }
        slen = slen + (ch << (8*i));
      }
      buffer = RENEW(buffer, bytesread + slen, unsigned char);
      while (slen > 0) {
        rlen = fread(buffer + bytesread, sizeof(unsigned char), slen, fp);
        if (rlen < 0) {
          if (buffer)
            RELEASE(buffer);
          return NULL;
        }
        slen -= rlen;
        bytesread += rlen;
      }
    }
  }
  if (bytesread == 0) {
    ERROR("PFB segment length zero?");
  }

  buffer = RENEW(buffer, bytesread+1, unsigned char);
  buffer[bytesread] = 0;

  if (length)
    *length = bytesread;
  return buffer;
}

const char *
t1_get_standard_glyph (int code)
{
  if (!StandardEncoding[code])
    return NULL;

  return StandardEncoding[code];
}

int
t1_get_fontname (FILE *fp, char *fontname)
{
  unsigned char *buffer, *start, *end;
  int   length;
  char *key;
  int   fn_found = 0;

  rewind(fp);
  buffer = get_pfb_segment(fp, PFB_SEG_TYPE_ASCII, &length);
  if (buffer == NULL || length == 0)
    ERROR("Reading PFB (ASCII part) file failed.");
  start = buffer;
  end   = buffer + length;

  if (seek_operator(&start, end, "begin") < 0) {
    RELEASE(buffer);
    return -1;
  }

  while (!fn_found && start < end &&
         (key = get_next_key(&start, end)) != NULL) {
    if (!strcmp(key, "FontName")) {
      char *strval;
      if (parse_svalue(&start, end, &strval) == 1) {
        if (strlen(strval) > TYPE1_NAME_LEN_MAX) {
          WARN("FontName \"%s\" too long. (%d bytes)", strval, strlen(strval));
          strval[TYPE1_NAME_LEN_MAX] = '\0';
        }
        strcpy(fontname, strval);
        RELEASE(strval);
        fn_found = 1;
      }
    }
    RELEASE(key);
  }
  RELEASE(buffer);

  return 0;
}

static void
init_cff_font (cff_font *cff)
{
  cff->stream = NULL;
  cff->filter = 0;
  cff->fontname = NULL;
  cff->index    = 0;
  cff->flag = FONTTYPE_FONT;

  cff->header.major = 1;
  cff->header.minor = 0;
  cff->header.hdr_size = 4;
  cff->header.offsize  = 4;
  cff->name     = cff_new_index(1);
  cff->topdict  = cff_new_dict();
  cff->string   = NULL;
  cff->gsubr    = cff_new_index(0); /* No Global Subr */
  cff->encoding = NULL;
  cff->charsets = NULL;
  cff->fdselect = NULL;
  cff->cstrings = NULL;
  cff->fdarray  = NULL;
  cff->private  = NEW(1, cff_dict *);
  cff->private[0] = cff_new_dict();
  cff->subrs = NEW(1, cff_index *);
  cff->subrs[0] = NULL;

  cff->offset  = 0;
  cff->gsubr_offset = 0;
  cff->num_glyphs   = 0;
  cff->num_fds      = 1;
  cff->_string = cff_new_index(0);
}

cff_font *
t1_load_font (char **enc_vec, int mode, FILE *fp)
{
  int length;
  cff_font *cff;
  unsigned char *buffer, *start, *end;

  rewind(fp);
  /* ASCII section */
  buffer = get_pfb_segment(fp, PFB_SEG_TYPE_ASCII, &length);
  if (buffer == NULL || length == 0) {
    ERROR("Reading PFB (ASCII part) file failed.");
    return NULL;
  }

  cff = NEW(1, cff_font);
  init_cff_font(cff);

  start = buffer; end = buffer + length;
  if (parse_part1(cff, enc_vec, &start, end) < 0) {
    cff_close(cff);
    RELEASE(buffer);
    ERROR("Reading PFB (ASCII part) file failed.");
    return NULL;
  }
  RELEASE(buffer);

  /* Binary section */
  buffer = get_pfb_segment(fp, PFB_SEG_TYPE_BINARY, &length);
  if (buffer == NULL || length == 0) {
    cff_close(cff);
    RELEASE(buffer);
    ERROR("Reading PFB (BINARY part) file failed.");
    return NULL;
  } else {
    t1_decrypt(T1_EEKEY, buffer, buffer, 0, length);
  }
  start = buffer + 4; end = buffer + length;
  if (parse_part2(cff, &start, end, mode) < 0) {
    cff_close(cff);
    RELEASE(buffer);
    ERROR("Reading PFB (BINARY part) file failed.");
    return NULL;
  }
  RELEASE(buffer);

  cff_update_string(cff);

  /* Remaining section ignored. */

  return cff;
}
