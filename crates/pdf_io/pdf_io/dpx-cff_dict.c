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

/*
 * CFF Font Dictionary
 *
 *  Adobe Technical Note #5176 "The Compact Font Format Specification"
 */

#include <assert.h>
#include <errno.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-error.h"
#include "dpx-mem.h"

#ifndef CFF_DEBUG_STR
#define CFF_DEBUG_STR "CFF"
#define CFF_DEBUG     5
#endif

/* #include "cff_string.h" */
#include "dpx-cff_dict.h"

/* cff_update_dict requires this. */
#include "dpx-cff.h"
#include "dpx-cff_limits.h"
#include "dpx-cff_types.h"
/* work_buffer for get_real() */
#include "dpx-mfileio.h"

#define CFF_PARSE_OK                0
#define CFF_ERROR_PARSE_ERROR      -1
#define CFF_ERROR_STACK_OVERFLOW   -2
#define CFF_ERROR_STACK_UNDERFLOW  -3
#define CFF_ERROR_STACK_RANGECHECK -4

#define DICT_ENTRY_MAX 16
cff_dict *cff_new_dict (void)
{
  cff_dict *dict;

  dict = NEW(1, cff_dict);
  dict->max     = DICT_ENTRY_MAX;
  dict->count   = 0;
  dict->entries = NEW(dict->max, cff_dict_entry);

  return dict;
}

void cff_release_dict (cff_dict *dict)
{
  if (dict) {
    if (dict->entries) {
      int i;
      for (i=0;i<dict->count;i++) {
        free((dict->entries)[i].values);
      }
      free(dict->entries);
    }
  free(dict);
  }
}

/*
 * Operand stack:
 *  only numbers are stored (as double)
 *
 * Operand types:
 *
 * number : double (integer or real)
 * bool: stored as a number
 * SID    : stored as a number
 * array  : array of numbers
 * delta  : array of numbers
 */

#define CFF_DICT_STACK_LIMIT 64
static int    stack_top = 0;
static double arg_stack[CFF_DICT_STACK_LIMIT];

/*
 * CFF DICT encoding:
 * TODO: default values
 */

#define CFF_LAST_DICT_OP1 22
#define CFF_LAST_DICT_OP2 39
#define CFF_LAST_DICT_OP (CFF_LAST_DICT_OP1 + CFF_LAST_DICT_OP2)

static struct {
  const char *opname;
  int   argtype;
} dict_operator[CFF_LAST_DICT_OP] = {
  {"version",     CFF_TYPE_SID},
  {"Notice",      CFF_TYPE_SID},
  {"FullName",    CFF_TYPE_SID},
  {"FamilyName",  CFF_TYPE_SID},
  {"Weight",      CFF_TYPE_SID},
  {"FontBBox",    CFF_TYPE_ARRAY},
  {"BlueValues",       CFF_TYPE_DELTA},
  {"OtherBlues",       CFF_TYPE_DELTA},
  {"FamilyBlues",      CFF_TYPE_DELTA},
  {"FamilyOtherBlues", CFF_TYPE_DELTA},
  {"StdHW",            CFF_TYPE_NUMBER},
  {"StdVW",            CFF_TYPE_NUMBER},
  {NULL, -1},  /* first byte of two-byte operator */
  /* Top */
  {"UniqueID",    CFF_TYPE_NUMBER},
  {"XUID",        CFF_TYPE_ARRAY},
  {"charset",     CFF_TYPE_OFFSET},
  {"Encoding",    CFF_TYPE_OFFSET},
  {"CharStrings", CFF_TYPE_OFFSET},
  {"Private",     CFF_TYPE_SZOFF}, /* two numbers (size and offset) */
  /* Private */
  {"Subrs",         CFF_TYPE_OFFSET},
  {"defaultWidthX", CFF_TYPE_NUMBER},
  {"nominalWidthX", CFF_TYPE_NUMBER},
  /* Operator 2 */
  {"Copyright",          CFF_TYPE_SID},
  {"IsFixedPitch",       CFF_TYPE_BOOLEAN},
  {"ItalicAngle",        CFF_TYPE_NUMBER},
  {"UnderlinePosition",  CFF_TYPE_NUMBER},
  {"UnderlineThickness", CFF_TYPE_NUMBER},
  {"PaintType",      CFF_TYPE_NUMBER},
  {"CharstringType", CFF_TYPE_NUMBER},
  {"FontMatrix",     CFF_TYPE_ARRAY},
  {"StrokeWidth",    CFF_TYPE_NUMBER},
  {"BlueScale", CFF_TYPE_NUMBER},
  {"BlueShift", CFF_TYPE_NUMBER},
  {"BlueFuzz",  CFF_TYPE_NUMBER},
  {"StemSnapH", CFF_TYPE_DELTA},
  {"StemSnapV", CFF_TYPE_DELTA},
  {"ForceBold", CFF_TYPE_BOOLEAN},
  {NULL, -1},
  {NULL, -1},
  {"LanguageGroup",     CFF_TYPE_NUMBER},
  {"ExpansionFactor",   CFF_TYPE_NUMBER},
  {"InitialRandomSeed", CFF_TYPE_NUMBER},
  {"SyntheticBase", CFF_TYPE_NUMBER},
  {"PostScript",    CFF_TYPE_SID},
  {"BaseFontName",  CFF_TYPE_SID},
  {"BaseFontBlend", CFF_TYPE_DELTA}, /* MMaster ? */
  {NULL, -1},
  {NULL, -1},
  {NULL, -1},
  {NULL, -1},
  {NULL, -1},
  {NULL, -1},
  /* CID-Keyed font */
  {"ROS",             CFF_TYPE_ROS}, /* SID SID number */
  {"CIDFontVersion",  CFF_TYPE_NUMBER},
  {"CIDFontRevision", CFF_TYPE_NUMBER},
  {"CIDFontType",     CFF_TYPE_NUMBER},
  {"CIDCount",        CFF_TYPE_NUMBER},
  {"UIDBase",         CFF_TYPE_NUMBER},
  {"FDArray",         CFF_TYPE_OFFSET},
  {"FDSelect",        CFF_TYPE_OFFSET},
  {"FontName",        CFF_TYPE_SID},
};

/* Parse DICT data */
static double get_integer (card8 **data, card8 *endptr, int *status)
{
  int result = 0;
  card8 b0, b1, b2;

  b0 = *(*data)++;
  if (b0 == 28 && *data < endptr - 2) { /* shortint */
    b1 = *(*data)++;
    b2 = *(*data)++;
    result = b1*256+b2;
    if (result > 0x7fffL)
      result -= 0x10000L;
  } else if (b0 == 29 && *data < endptr - 4) { /* longint */
    int i;
    result = *(*data)++;
    if (result > 0x7f)
      result -= 0x100;
    for (i=0;i<3;i++) {
      result = result*256+(**data);
      *data += 1;
    }
  } else if (b0 >= 32 && b0 <= 246) { /* int (1) */
    result = b0 - 139;
  } else if (b0 >= 247 && b0 <= 250) { /* int (2) */
    b1 = *(*data)++;
    result = (b0-247)*256+b1+108;
  } else if (b0 >= 251 && b0 <= 254) {
    b1 = *(*data)++;
    result = -(b0-251)*256-b1-108;
  } else {
    *status = CFF_ERROR_PARSE_ERROR;
  }

  return (double) result;
}

/* Simply uses strtod */
static double get_real(card8 **data, card8 *endptr, int *status)
{
  double result = 0.0;
  int nibble = 0, pos = 0;
  int len = 0, fail = 0;

  if (**data != 30 || *data >= endptr -1) {
    *status = CFF_ERROR_PARSE_ERROR;
    return 0.0;
  }

  *data += 1; /* skip first byte (30) */

  pos = 0;
  while ((! fail) && len < WORK_BUFFER_SIZE - 2 && *data < endptr) {
    /* get nibble */
    if (pos % 2) {
      nibble = **data & 0x0f;
      *data += 1;
    } else {
      nibble = (**data >> 4) & 0x0f;
    }
    if (nibble >= 0x00 && nibble <= 0x09) {
      work_buffer[len++] = nibble + '0';
    } else if (nibble == 0x0a) { /* . */
      work_buffer[len++] = '.';
    } else if (nibble == 0x0b || nibble == 0x0c) { /* E, E- */
      work_buffer[len++] = 'e';
      if (nibble == 0x0c)
        work_buffer[len++] = '-';
    } else if (nibble == 0x0e) { /* `-' */
      work_buffer[len++] = '-';
    } else if (nibble == 0x0d) { /* skip */
      /* do nothing */
    } else if (nibble == 0x0f) { /* end */
      work_buffer[len++] = '\0';
      if (((pos % 2) == 0) && (**data != 0xff)) {
        fail = 1;
      }
      break;
    } else { /* invalid */
      fail = 1;
    }
    pos++;
  }

  /* returned values */
  if (fail || nibble != 0x0f) {
    *status = CFF_ERROR_PARSE_ERROR;
  } else {
    char *s;
    result = strtod(work_buffer, &s);
    if (*s != 0 || errno == ERANGE) {
      *status = CFF_ERROR_PARSE_ERROR;
    }
  }

  return result;
}

/* operators */
static void add_dict (cff_dict *dict,
                      card8 **data, card8 *endptr, int *status)
{
  int id, argtype;

  id = **data;
  if (id == 0x0c) {
    *data += 1;
    if (*data >= endptr ||
        (id = **data + CFF_LAST_DICT_OP1) >= CFF_LAST_DICT_OP) {
      *status = CFF_ERROR_PARSE_ERROR;
      return;
    }
  } else if (id >= CFF_LAST_DICT_OP1) {
    *status = CFF_ERROR_PARSE_ERROR;
    return;
  }

  argtype = dict_operator[id].argtype;
  if (dict_operator[id].opname == NULL || argtype < 0) {
    /* YuppySC-Regular.otf from OS X for instance uses op id 37, simply ignore
       this dict instead of treat it as parsing error. */
    return;
  }

  if (dict->count >= dict->max) {
    dict->max += DICT_ENTRY_MAX;
    dict->entries = RENEW(dict->entries, dict->max, cff_dict_entry);
  }

  (dict->entries)[dict->count].id = id;
  (dict->entries)[dict->count].key = dict_operator[id].opname;
  if (argtype == CFF_TYPE_NUMBER ||
      argtype == CFF_TYPE_BOOLEAN ||
      argtype == CFF_TYPE_SID ||
      argtype == CFF_TYPE_OFFSET) {
    /* check for underflow here, as exactly one operand is expected */
    if (stack_top < 1) {
      *status = CFF_ERROR_STACK_UNDERFLOW;
      return;
    }
    stack_top--;
    (dict->entries)[dict->count].count  = 1;
    (dict->entries)[dict->count].values = NEW(1, double);
    (dict->entries)[dict->count].values[0] = arg_stack[stack_top];
    dict->count += 1;
  } else {
    /* just ignore operator if there were no operands provided;
       don't treat this as underflow (e.g. StemSnapV in TemporaLGCUni-Italic.otf) */
    if (stack_top > 0)
    {
      (dict->entries)[dict->count].count  = stack_top;
      (dict->entries)[dict->count].values = NEW(stack_top, double);
      while (stack_top > 0) {
        stack_top--;
        (dict->entries)[dict->count].values[stack_top] = arg_stack[stack_top];
      }
      dict->count += 1;
    }
  }

  *data += 1;

  return;
}

/*
 * All operands are treated as number or array of numbers.
 *  Private: two numbers, size and offset
 *  ROS    : three numbers, SID, SID, and a number
 */
cff_dict *cff_dict_unpack (card8 *data, card8 *endptr)
{
  cff_dict *dict;
  int status = CFF_PARSE_OK;

  stack_top = 0;

  dict = cff_new_dict();
  while (data < endptr && status == CFF_PARSE_OK) {
    if (*data < 22) { /* operator */
      add_dict(dict, &data, endptr, &status);
    } else if (*data == 30) { /* real - First byte of a sequence (variable) */
      if (stack_top < CFF_DICT_STACK_LIMIT) {
        arg_stack[stack_top] = get_real(&data, endptr, &status);
        stack_top++;
      } else {
        status = CFF_ERROR_STACK_OVERFLOW;
      }
    } else if (*data == 255 || (*data >= 22 && *data <= 27)) { /* reserved */
      data++;
    } else { /* everything else are integer */
      if (stack_top < CFF_DICT_STACK_LIMIT) {
        arg_stack[stack_top] = get_integer(&data, endptr, &status);
        stack_top++;
      } else {
        status = CFF_ERROR_STACK_OVERFLOW;
      }
    }
  }

  if (status != CFF_PARSE_OK) {
    _tt_abort("%s: Parsing CFF DICT failed. (error=%d)", CFF_DEBUG_STR, status);
  } else if (stack_top != 0) {
    dpx_warning("%s: Garbage in CFF DICT data.", CFF_DEBUG_STR);
    stack_top = 0;
  }

  return dict;
}

/* Pack DICT data */
static int pack_integer (card8 *dest, int destlen, int value)
{
  int len = 0;

  if (value >= -107 && value <= 107) {
    if (destlen < 1)
      _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);
    dest[0] = (value + 139) & 0xff;
    len = 1;
  } else if (value >= 108 && value <= 1131) {
    if (destlen < 2)
      _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);
    value = 0xf700u + value - 108;
    dest[0] = (value >> 8) & 0xff;
    dest[1] = value & 0xff;
    len = 2;
  } else if (value >= -1131 && value <= -108) {
    if (destlen < 2)
      _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);
    value = 0xfb00u - value - 108;
    dest[0] = (value >> 8) & 0xff;
    dest[1] = value & 0xff;
    len = 2;
  } else if (value >= -32768 && value <= 32767) { /* shortint */
    if (destlen < 3)
      _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);
    dest[0] = 28;
    dest[1] = (value >> 8) & 0xff;
    dest[2] = value & 0xff;
    len = 3;
  } else { /* longint */
    if (destlen < 5)
      _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);
    dest[0] = 29;
    dest[1] = (value >> 24) & 0xff;
    dest[2] = (value >> 16) & 0xff;
    dest[3] = (value >> 8) & 0xff;
    dest[4] = value & 0xff;
    len = 5;
  }

  return len;
}

static int pack_real (card8 *dest, int destlen, double value)
{
  int i = 0, pos = 2;
  char buffer[32];

  if (destlen < 2)
    _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);

  dest[0] = 30;

  if (value == 0.0) {
    dest[1] = 0x0f;
    return 2;
  }

  if (value < 0.0) {
    dest[1] = 0xe0;
    value *= -1.0;
    pos++;
  }

  /* To avoid the problem with Mac OS X 10.4 Quartz,
   * change the presion of the real numbers
   * on June 27, 2007 for musix20.pfb */
  sprintf(buffer, "%.13g", value);

  for (i = 0; buffer[i] != '\0'; i++) {
    unsigned char ch = 0;
    if (buffer[i] == '.') {
      ch = 0x0a;
    } else if (buffer[i] >= '0' && buffer[i] <= '9') {
      ch = buffer[i] - '0';
    } else if (buffer[i] == 'e') {
      ch = (buffer[++i] == '-' ? 0x0c : 0x0b);
    } else {
      _tt_abort("%s: Invalid character.", CFF_DEBUG_STR);
    }

    if (destlen < pos/2 + 1)
      _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);

    if (pos % 2) {
      dest[pos/2] += ch;
    } else {
      dest[pos/2] = (ch << 4);
    }
    pos++;
  }

  if (pos % 2) {
    dest[pos/2] += 0x0f;
    pos++;
  } else {
    if (destlen < pos/2 + 1)
      _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);
    dest[pos/2] = 0xff;
    pos += 2;
  }

  return pos/2;
}

static int cff_dict_put_number (double value,
                                 card8 *dest, int destlen,
                                 int type)
{
  int    len = 0;
  double nearint;

  nearint = floor(value+0.5);
  /* set offset to longint */
  if (type == CFF_TYPE_OFFSET) {
    int lvalue;

    lvalue = (int) value;
    if (destlen < 5)
      _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);
    dest[0] = 29;
    dest[1] = (lvalue >> 24) & 0xff;
    dest[2] = (lvalue >> 16) & 0xff;
    dest[3] = (lvalue >>  8) & 0xff;
    dest[4] = lvalue         & 0xff;
    len = 5;
  } else if (value > CFF_INT_MAX || value < CFF_INT_MIN ||
             (fabs(value - nearint) > 1.0e-5)) { /* real */
    len = pack_real(dest, destlen, value);
  } else { /* integer */
    len = pack_integer(dest, destlen, (int) nearint);
  }

  return len;
}

static int
put_dict_entry (cff_dict_entry *de,
                card8 *dest, int destlen)
{
  int  len = 0;
  int  i, type, id;

  if (de->count > 0) {
    id = de->id;
    if (dict_operator[id].argtype == CFF_TYPE_OFFSET ||
        dict_operator[id].argtype == CFF_TYPE_SZOFF) {
      type = CFF_TYPE_OFFSET;
    } else {
      type = CFF_TYPE_NUMBER;
    }
    for (i = 0; i < de->count; i++) {
      len += cff_dict_put_number(de->values[i],
                                 dest+len,
                                 destlen-len, type);
    }
    if (id >= 0 && id < CFF_LAST_DICT_OP1) {
      if (len + 1 > destlen)
        _tt_abort("%s: Buffer overflow.", CFF_DEBUG_STR);
      dest[len++] = id;
    } else if (id >= 0 && id < CFF_LAST_DICT_OP) {
      if (len + 2 > destlen)
        _tt_abort("in cff_dict_pack(): Buffer overflow");
      dest[len++] = 12;
      dest[len++] = id - CFF_LAST_DICT_OP1;
    } else {
      _tt_abort("%s: Invalid CFF DICT operator ID.", CFF_DEBUG_STR);
    }
  }

  return len;
}

int cff_dict_pack (cff_dict *dict, card8 *dest, int destlen)
{
  int  len = 0;
  int  i;

  for (i = 0; i < dict->count; i++) {
    if (streq_ptr(dict->entries[i].key, "ROS")) {
      len += put_dict_entry(&dict->entries[i], dest, destlen);
      break;
    }
  }
  for (i = 0; i < dict->count; i++) {
    if (strcmp(dict->entries[i].key, "ROS")) {
      len += put_dict_entry(&dict->entries[i], dest+len, destlen-len);
    }
  }

  return len;
}

void cff_dict_add (cff_dict *dict, const char *key, int count)
{
  int id, i;

  for (id=0;id<CFF_LAST_DICT_OP;id++) {
    if (key && dict_operator[id].opname &&
        streq_ptr(dict_operator[id].opname, key))
      break;
  }

  if (id == CFF_LAST_DICT_OP)
    _tt_abort("%s: Unknown CFF DICT operator.", CFF_DEBUG_STR);

  for (i=0;i<dict->count;i++) {
    if ((dict->entries)[i].id == id) {
      if ((dict->entries)[i].count != count)
        _tt_abort("%s: Inconsistent DICT argument number.", CFF_DEBUG_STR);
      return;
    }
  }

  if (dict->count + 1 >= dict->max) {
    dict->max += 8;
    dict->entries = RENEW(dict->entries, dict->max, cff_dict_entry);
  }

  (dict->entries)[dict->count].id    = id;
  (dict->entries)[dict->count].key   = dict_operator[id].opname;
  (dict->entries)[dict->count].count = count;
  if (count > 0) {
    (dict->entries)[dict->count].values = NEW(count, double);
    memset((dict->entries)[dict->count].values,
           0, sizeof(double)*count);
  } else {
    (dict->entries)[dict->count].values = NULL;
  }
  dict->count += 1;

  return;
}

void cff_dict_remove (cff_dict *dict, const char *key)
{
  int i;
  for (i = 0; i < dict->count; i++) {
    if (streq_ptr(key, (dict->entries)[i].key)) {
      (dict->entries)[i].count = 0;
      (dict->entries)[i].values = mfree((dict->entries)[i].values);
    }
  }
}

int cff_dict_known (cff_dict *dict, const char *key)
{
  int i;

  for (i = 0; i < dict->count; i++) {
    if (streq_ptr(key, (dict->entries)[i].key)
        && (dict->entries)[i].count > 0)
      return 1;
  }

  return 0;
}

double cff_dict_get (cff_dict *dict, const char *key, int idx)
{
  double value = 0.0;
  int    i;

  assert(key && dict);

  for (i = 0; i < dict->count; i++) {
    if (streq_ptr(key, (dict->entries)[i].key)) {
      if ((dict->entries)[i].count > idx)
        value = (dict->entries)[i].values[idx];
      else
        _tt_abort("%s: Invalid index number.", CFF_DEBUG_STR);
      break;
    }
  }

  if (i == dict->count)
    _tt_abort("%s: DICT entry \"%s\" not found.", CFF_DEBUG_STR, key);

  return value;
}

void cff_dict_set (cff_dict *dict, const char *key, int idx, double value)
{
  int i;

  assert(dict && key);

  for (i = 0 ; i < dict->count; i++) {
    if (streq_ptr(key, (dict->entries)[i].key)) {
      if ((dict->entries)[i].count > idx)
        (dict->entries)[i].values[idx] = value;
      else
        _tt_abort("%s: Invalid index number.", CFF_DEBUG_STR);
      break;
    }
  }

  if (i == dict->count)
    _tt_abort("%s: DICT entry \"%s\" not found.", CFF_DEBUG_STR, key);
}

void cff_dict_update (cff_dict *dict, cff_font *cff)
{
  int i;

  for (i = 0;i < dict->count; i++) {
    if ((dict->entries)[i].count > 0) {
      char *str;
      int   id;

      id = (dict->entries)[i].id;
      if (dict_operator[id].argtype == CFF_TYPE_SID) {
        str = cff_get_string(cff, (dict->entries)[i].values[0]);
        (dict->entries)[i].values[0] = cff_add_string(cff, str, 1);
        free(str);
      } else if (dict_operator[id].argtype == CFF_TYPE_ROS) {
        str = cff_get_string(cff, (dict->entries)[i].values[0]);
        (dict->entries)[i].values[0] = cff_add_string(cff, str, 1);
        free(str);
        str = cff_get_string(cff, (dict->entries)[i].values[1]);
        (dict->entries)[i].values[1] = cff_add_string(cff, str, 1);
        free(str);
      }
    }
  }
}
