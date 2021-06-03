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

#include "dpx-pst_obj.h"

#include <assert.h>
#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pst.h"

struct pst_obj
{
  pst_type type;
  void    *data;
};

static const char *pst_const_null  = "null";
static const char *pst_const_mark  = "mark";
/*
static const char *pst_const_true  = "true";
static const char *pst_const_false = "false";
*/

typedef char *                     pst_null;
typedef struct { char    value; }  pst_boolean;
typedef struct { int     value; }  pst_integer;
typedef struct { double  value; }  pst_real;
typedef struct { char   *value; }  pst_name;
typedef struct
{
  unsigned int   length;
  unsigned char *value;
} pst_string;


/* BOOLEAN */
static pst_boolean *pst_boolean_new     (char value)      ;
static void         pst_boolean_release (pst_boolean *obj);
static int          pst_boolean_IV      (pst_boolean *obj);
static double       pst_boolean_RV      (pst_boolean *obj);
static unsigned char *pst_boolean_SV      (pst_boolean *obj);
static unsigned int pst_boolean_length  (void);
static void        *pst_boolean_data_ptr(pst_boolean *obj);

/* NUMBERS */
static pst_integer *pst_integer_new     (int value)       ;
static void         pst_integer_release (pst_integer *obj);
static int          pst_integer_IV      (pst_integer *obj);
static double       pst_integer_RV      (pst_integer *obj);
static unsigned char      *pst_integer_SV      (pst_integer *obj);
static unsigned int        pst_integer_length  (void);
static void        *pst_integer_data_ptr(pst_integer *obj);

static pst_real *pst_real_new      (double value) ;
static void      pst_real_release  (pst_real *obj);
static int       pst_real_IV       (pst_real *obj);
static double    pst_real_RV       (pst_real *obj);
static unsigned char   *pst_real_SV       (pst_real *obj);
static void     *pst_real_data_ptr (pst_real *obj);
static unsigned int     pst_real_length   (void);

/* NAME */
static pst_name *pst_name_new      (const char *name) ;
static void      pst_name_release  (pst_name *obj);
static int       pst_name_IV       (void);
static double    pst_name_RV       (void);
static unsigned char   *pst_name_SV       (pst_name *obj);
static void     *pst_name_data_ptr (pst_name *obj);
static unsigned int     pst_name_length   (pst_name *obj);

/* STRING */
static pst_string *pst_string_parse_literal (unsigned char **inbuf, unsigned char *inbufend);
static pst_string *pst_string_parse_hex     (unsigned char **inbuf, unsigned char *inbufend);

static pst_string *pst_string_new      (unsigned char *str, unsigned int len);
static void        pst_string_release  (pst_string *obj)       ;
static int         pst_string_IV       (pst_string *obj)       ;
static double      pst_string_RV       (pst_string *obj)       ;
static unsigned char     *pst_string_SV       (pst_string *obj)       ;
static void       *pst_string_data_ptr (pst_string *obj)       ;
static unsigned int       pst_string_length   (pst_string *obj)       ;


#define TYPE_ERROR() _tt_abort("Operation not defined for this type of object.")

pst_obj *
pst_new_obj (pst_type type, void *data)
{
  pst_obj *obj;

  obj = NEW(1, struct pst_obj);
  obj->type = type;
  obj->data = data;

  return obj;
}

pst_obj *
pst_new_mark (void)
{
  char *q;

  q = NEW(strlen(pst_const_mark)+1, char);
  strcpy(q, pst_const_mark);
  return pst_new_obj(PST_TYPE_MARK, (void *)q);
}

void
pst_release_obj (pst_obj *obj)
{
  assert(obj);
  switch (obj->type) {
  case PST_TYPE_BOOLEAN: pst_boolean_release(obj->data); break;
  case PST_TYPE_INTEGER: pst_integer_release(obj->data); break;
  case PST_TYPE_REAL:    pst_real_release(obj->data);    break;
  case PST_TYPE_NAME:    pst_name_release(obj->data);    break;
  case PST_TYPE_STRING:  pst_string_release(obj->data);  break;
  case PST_TYPE_NULL:
  case PST_TYPE_MARK:
  case PST_TYPE_UNKNOWN:
    free(obj->data);
    break;
  default:
    _tt_abort("Unrecognized object type: %d", obj->type);
  }
  free(obj);
}

pst_type
pst_type_of (pst_obj *obj)
{
  assert(obj);
  return obj->type;
}

int
pst_length_of (pst_obj *obj)
{
  int len = 0;

  assert(obj);
  switch (obj->type) {
  case PST_TYPE_BOOLEAN: len = pst_boolean_length();          break;
  case PST_TYPE_INTEGER: len = pst_integer_length();          break;
  case PST_TYPE_REAL:    len = pst_real_length();             break;
  case PST_TYPE_NAME:    len = pst_name_length(obj->data);    break;
  case PST_TYPE_STRING:  len = pst_string_length(obj->data);  break;
  case PST_TYPE_NULL:
  case PST_TYPE_MARK:
    TYPE_ERROR();
    break;
  case PST_TYPE_UNKNOWN:
    len = strlen(obj->data);
    break;
  default:
    _tt_abort("Unrecognized object type: %d", obj->type);
  }

  return len;
}

int
pst_getIV (pst_obj *obj)
{
  int iv = 0;

  assert(obj);
  switch (obj->type) {
  case PST_TYPE_BOOLEAN: iv = pst_boolean_IV(obj->data); break;
  case PST_TYPE_INTEGER: iv = pst_integer_IV(obj->data); break;
  case PST_TYPE_REAL:    iv = pst_real_IV(obj->data);    break;
  case PST_TYPE_NAME:    iv = pst_name_IV();             break;
  case PST_TYPE_STRING:  iv = pst_string_IV(obj->data);  break;
  case PST_TYPE_NULL:
  case PST_TYPE_MARK:
    TYPE_ERROR();
    break;
  case PST_TYPE_UNKNOWN:
    _tt_abort("Cannot convert object of type UNKNOWN to integer value.");
    break;
  default:
    _tt_abort("Unrecognized object type: %d", obj->type);
  }

  return iv;
}

double
pst_getRV (pst_obj *obj)
{
  double rv = 0.0;

  assert(obj);
  switch (obj->type) {
  case PST_TYPE_BOOLEAN: rv = pst_boolean_RV(obj->data); break;
  case PST_TYPE_INTEGER: rv = pst_integer_RV(obj->data); break;
  case PST_TYPE_REAL:    rv = pst_real_RV(obj->data);    break;
  case PST_TYPE_NAME:    rv = pst_name_RV();             break;
  case PST_TYPE_STRING:  rv = pst_string_RV(obj->data);  break;
  case PST_TYPE_NULL:
  case PST_TYPE_MARK:
    TYPE_ERROR();
    break;
  case PST_TYPE_UNKNOWN:
    _tt_abort("Cannot convert object of type UNKNOWN to real value.");
    break;
  default:
    _tt_abort("Unrecognized object type: %d", obj->type);
  }

  return rv;
}

/* Length can be obtained by pst_length_of(). */
unsigned char *
pst_getSV (pst_obj *obj)
{
  unsigned char *sv = NULL;

  assert(obj);
  switch (obj->type) {
  case PST_TYPE_BOOLEAN: sv = pst_boolean_SV(obj->data); break;
  case PST_TYPE_INTEGER: sv = pst_integer_SV(obj->data); break;
  case PST_TYPE_REAL:    sv = pst_real_SV(obj->data);    break;
  case PST_TYPE_NAME:    sv = pst_name_SV(obj->data);    break;
  case PST_TYPE_STRING:  sv = pst_string_SV(obj->data);  break;
  case PST_TYPE_NULL:
  case PST_TYPE_MARK:
    TYPE_ERROR();
    break;
  case PST_TYPE_UNKNOWN:
    {
      int len;

      len = strlen((char *) obj->data);
      if (len > 0) {
        sv = NEW(len+1, unsigned char);
        memcpy(sv, obj->data, len);
        sv[len] = '\0';
      } else {
        sv = NULL;
      }
      break;
    }
  default:
    _tt_abort("Unrecognized object type: %d", obj->type);
  }

  return sv;
}

void *
pst_data_ptr (pst_obj *obj)
{
  char *p = NULL;

  assert(obj);
  switch (obj->type) {
  case PST_TYPE_BOOLEAN: p = pst_boolean_data_ptr(obj->data); break;
  case PST_TYPE_INTEGER: p = pst_integer_data_ptr(obj->data); break;
  case PST_TYPE_REAL:    p = pst_real_data_ptr(obj->data);    break;
  case PST_TYPE_NAME:    p = pst_name_data_ptr(obj->data);    break;
  case PST_TYPE_STRING:  p = pst_string_data_ptr(obj->data);  break;
  case PST_TYPE_NULL:
  case PST_TYPE_MARK:
    TYPE_ERROR();
    break;
  case PST_TYPE_UNKNOWN:
    p = obj->data;
    break;
  default:
    _tt_abort("Unrecognized object type: %d", obj->type);
  }

  return (void *)p;
}

/* BOOLEAN */
static pst_boolean *
pst_boolean_new (char value)
{
  pst_boolean *obj;
  obj = NEW(1, pst_boolean);
  obj->value = value;
  return obj;
}

static void
pst_boolean_release (pst_boolean *obj)
{
  assert(obj);
  free(obj);
}

static int
pst_boolean_IV (pst_boolean *obj)
{
  assert(obj);
  return (int) obj->value;
}

static double
pst_boolean_RV (pst_boolean *obj)
{
  assert(obj);
  return (double) obj->value;
}

static unsigned char *
pst_boolean_SV (pst_boolean *obj)
{
  unsigned char *str;

  assert(obj);

  if (obj->value) {
    str = NEW(5, unsigned char);
    memcpy(str, "true", 4);
    str[4] = '\0';
  } else {
    str = NEW(6, unsigned char);
    memcpy(str, "false", 5);
    str[5] = '\0';
  }

  return str;
}

static unsigned int
pst_boolean_length (void)
{
  TYPE_ERROR();
  return 0;
}

static void *
pst_boolean_data_ptr (pst_boolean *obj)
{
  assert(obj);
  return (void*) &(obj->value);
}

pst_obj *
pst_parse_boolean (unsigned char **inbuf, unsigned char *inbufend)
{
  if (*inbuf + 4 <= inbufend &&
      memcmp(*inbuf, "true", 4) == 0 &&
      PST_TOKEN_END(*inbuf + 4, inbufend)) {
    *inbuf += 4;
    return pst_new_obj(PST_TYPE_BOOLEAN, pst_boolean_new(1));
  } else if (*inbuf + 5 <= inbufend &&
             memcmp(*inbuf, "false", 5) == 0 &&
             PST_TOKEN_END(*inbuf + 5, inbufend)) {
    *inbuf += 5;
    return pst_new_obj(PST_TYPE_BOOLEAN, pst_boolean_new(0));
  } else
    return NULL;
}


/* NULL */
pst_obj *
pst_parse_null (unsigned char **inbuf, unsigned char *inbufend)
{
  if (*inbuf + 4 <= inbufend &&
      memcmp(*inbuf, "null", 4) == 0 &&
      PST_TOKEN_END(*inbuf+4, inbufend)) {
    char *q;

    *inbuf += 4;
    q = NEW(strlen(pst_const_null)+1, char);
    strcpy(q, pst_const_null);
    return pst_new_obj(PST_TYPE_NULL, (void*)q);
  } else
    return NULL;
}

/* INTEGER */
static pst_integer *
pst_integer_new (int value)
{
  pst_integer *obj;
  obj = NEW(1, pst_integer);
  obj->value = value;
  return obj;
}

static void
pst_integer_release (pst_integer *obj)
{
  assert(obj);
  free(obj);
}

static int
pst_integer_IV (pst_integer *obj)
{
  assert(obj);
  return (int) obj->value;
}

static double
pst_integer_RV (pst_integer *obj)
{
  assert(obj);
  return (double) obj->value;
}

static unsigned char *
pst_integer_SV (pst_integer *obj)
{
  char *value;
  int   len;
  char  fmt_buf[PST_MAX_DIGITS+5];

  assert(obj);

  len = sprintf(fmt_buf, "%d", obj->value);

  value = NEW(len + 1, char);
  strcpy(value, fmt_buf);

  return (unsigned char *) value;
}

static void *
pst_integer_data_ptr (pst_integer *obj)
{
  assert(obj);
  return (void*) &(obj->value);
}

static unsigned int
pst_integer_length (void)
{
  TYPE_ERROR();
  return 0;
}

/* REAL */
static pst_real *
pst_real_new (double value)
{
  pst_real *obj;

  obj = NEW(1, pst_real);
  obj->value = value;

  return obj;
}

static void
pst_real_release (pst_real *obj)
{
  assert(obj);
  free(obj);
}

static int
pst_real_IV (pst_real *obj)
{
  assert(obj);
  return (int) obj->value;
}

static double
pst_real_RV (pst_real *obj)
{
  assert(obj);
  return (double) obj->value;
}

static unsigned char *
pst_real_SV (pst_real *obj)
{
  char *value;
  int   len;
  char  fmt_buf[PST_MAX_DIGITS+5];

  assert(obj);

  len = sprintf(fmt_buf, "%.5g", obj->value);

  value = NEW(len, char);
  strcpy(value, fmt_buf);

  return (unsigned char *) value;
}

static void *
pst_real_data_ptr (pst_real *obj)
{
  assert(obj);

  return (void*) &(obj->value);
}

static unsigned int
pst_real_length (void)
{
  TYPE_ERROR();
  return 0;
}

/* NOTE: the input buffer must be null-terminated, i.e., *inbufend == 0 */
/* leading white-space is ignored */
pst_obj *
pst_parse_number (unsigned char **inbuf, unsigned char *inbufend)
{
  unsigned char  *cur;
  int     lval;
  double  dval;

  errno = 0;
  lval = strtol((char *) *inbuf, (char **) (void *) &cur, 10);
  if (errno || *cur == '.' || *cur == 'e' || *cur == 'E') {
    /* real */
    errno = 0;
    dval = strtod((char *) *inbuf, (char **) (void *) &cur);
    if (!errno && PST_TOKEN_END(cur, inbufend)) {
      *inbuf = cur;
      return pst_new_obj(PST_TYPE_REAL, pst_real_new(dval));
    }
  } else if (cur != *inbuf && PST_TOKEN_END(cur, inbufend)) {
    /* integer */
    *inbuf = cur;
    return pst_new_obj(PST_TYPE_INTEGER, pst_integer_new(lval));
  } else if (lval >= 2 && lval <= 36 && *cur == '#' && isalnum(*++cur) &&
             /* strtod allows leading "0x" for hex numbers, but we don't */
             (lval != 16 || (cur[1] != 'x' && cur[1] != 'X'))) {
    /* integer with radix */
    /* Can the base have a (plus) sign? I think yes. */
    errno = 0;
    lval = strtol((char *) cur, (char **) (void *) &cur, lval);
    if (!errno && PST_TOKEN_END(cur, inbufend)) {
      *inbuf = cur;
      return pst_new_obj(PST_TYPE_INTEGER, pst_integer_new(lval));
    }
  }
  /* error */
  return NULL;
}

/* NAME */

/*
 * \0 is not allowed for name object.
 */

static pst_name *
pst_name_new (const char *name)
{
  pst_name *obj;

  obj = NEW(1, pst_name);
  obj->value = NEW(strlen(name)+1, char);
  strcpy(obj->value, name);

  return obj;
}

static void
pst_name_release (pst_name *obj)
{
  assert(obj);
  free(obj->value);
  free(obj);
}

static int
getxpair (unsigned char **s)
{
  int hi, lo;
  hi = xtoi(**s);
  if (hi < 0)
    return hi;
  (*s)++;
  lo = xtoi(**s);
  if (lo < 0)
    return lo;
  (*s)++;
  return ((hi << 4)| lo);
}

pst_obj *
pst_parse_name (unsigned char **inbuf, unsigned char *inbufend) /* / is required */
{
  unsigned char  wbuf[PST_NAME_LEN_MAX+1];
  unsigned char  c, *p = wbuf, *cur = *inbuf;
  int     len = 0;

  if (*cur != '/')
    return NULL;
  cur++;

  while (!PST_TOKEN_END(cur, inbufend)) {
    c = *cur++;
    if (c == '#') {
      int val;
      if (cur + 2 >= inbufend) {
        dpx_warning("Premature end of input name string.");
        break;
      }
      val = getxpair(&cur);
      if (val <= 0) {
        dpx_warning("Invalid char for name object. (ignored)");
        continue;
      } else
        c = (unsigned char) val;
    }
    if (len < PST_NAME_LEN_MAX)
      *p++ = c;
    len++;
  }
  *p = '\0';

  if (len > PST_NAME_LEN_MAX)
    dpx_warning("String too long for name object. Output will be truncated.");

  *inbuf = cur;
  return pst_new_obj(PST_TYPE_NAME, pst_name_new((char *)wbuf));
}

static int
pst_name_IV (void)
{
  TYPE_ERROR();
  return 0;
}

static double
pst_name_RV (void)
{
  TYPE_ERROR();
  return 0;
}

static unsigned char *
pst_name_SV (pst_name *obj)
{
  char *value;

  value = NEW(strlen(obj->value)+1, char);
  strcpy(value, obj->value);

  return (unsigned char *) value;
}

static void *
pst_name_data_ptr (pst_name *obj)
{
  assert(obj);
  return obj->value;
}

static unsigned int
pst_name_length (pst_name *obj)
{
  assert(obj);
  return strlen(obj->value);
}


/* STRING */

/*
 * TODO: ascii85 string <~ .... ~>
 */
static pst_string *
pst_string_new (unsigned char *str, unsigned int len)
{
  pst_string *obj;
  obj = NEW(1, pst_string);
  obj->length  = len;
  obj->value = NULL;
  if (len > 0) {
    obj->value = NEW(len, unsigned char);
    if (str)
      memcpy(obj->value, str, len);
  }
  return obj;
}

static void
pst_string_release (pst_string *obj)
{
  assert(obj);
  free(obj->value);
  free(obj);
}

pst_obj *
pst_parse_string (unsigned char **inbuf, unsigned char *inbufend)
{
  if (*inbuf + 2 >= inbufend) {
    return NULL;
  } else if (**inbuf == '(')
    return pst_new_obj(PST_TYPE_STRING, pst_string_parse_literal(inbuf, inbufend));
  else if (**inbuf == '<' && *(*inbuf+1) == '~')
    _tt_abort("ASCII85 string not supported yet.");
  else if (**inbuf == '<')
    return pst_new_obj(PST_TYPE_STRING, pst_string_parse_hex(inbuf, inbufend));
  return NULL;
}

/* Overflowed value is set to invalid char.  */
static unsigned char
ostrtouc (unsigned char **inbuf, unsigned char *inbufend, unsigned char *valid)
{
  unsigned char *cur = *inbuf;
  unsigned int   val = 0;

  while (cur < inbufend && cur < *inbuf + 3 &&
         (*cur >= '0' && *cur <= '7')) {
    val = (val << 3) | (*cur - '0');
    cur++;
  }
  if (val > 255 || cur == *inbuf)
    *valid = 0;
  else
    *valid = 1;

  *inbuf = cur;
  return (unsigned char) val;
}

static unsigned char
esctouc (unsigned char **inbuf, unsigned char *inbufend, unsigned char *valid)
{
  unsigned char unescaped, escaped;

  escaped = **inbuf;
  *valid    = 1;
  switch (escaped) {
    /* Backslash, unbalanced paranthes */
  case '\\': case ')': case '(':
    unescaped = escaped;
    (*inbuf)++;
    break;
    /* Other escaped char */
  case 'n': unescaped = '\n'; (*inbuf)++; break;
  case 'r': unescaped = '\r'; (*inbuf)++; break;
  case 't': unescaped = '\t'; (*inbuf)++; break;
  case 'b': unescaped = '\b'; (*inbuf)++; break;
  case 'f': unescaped = '\f'; (*inbuf)++; break;
    /*
     * An end-of-line marker preceeded by backslash is not part of a
     * literal string
     */
  case '\r':
    unescaped = 0;
    *valid    = 0;
    *inbuf   += (*inbuf < inbufend - 1 && *(*inbuf+1) == '\n') ? 2 : 1;
    break;
  case '\n':
    unescaped = 0;
    *valid    = 0;
    (*inbuf)++;
    break;
    /* Possibly octal notion */
  default:
    unescaped = ostrtouc(inbuf, inbufend, valid);
  }

  return unescaped;
}

static pst_string *
pst_string_parse_literal (unsigned char **inbuf, unsigned char *inbufend)
{
  unsigned char  wbuf[PST_STRING_LEN_MAX];
  unsigned char *cur = *inbuf, c = 0;
  int len = 0, balance = 1;

  if (cur + 2 > inbufend || *cur != '(')
    return NULL;

  cur++;
  while (cur < inbufend && len < PST_STRING_LEN_MAX && balance > 0) {
    c = *(cur++);
    switch (c) {
    case '\\':
      {
        unsigned char unescaped, valid;
        unescaped = esctouc(&cur, inbufend, &valid);
        if (valid)
          wbuf[len++] = unescaped;
      }
      break;
    case '(':
      balance++;
      wbuf[len++] = '(';
      break;
    case ')':
      balance--;
      if (balance > 0)
        wbuf[len++] = ')';
      break;
      /*
       * An end-of-line marker (\n, \r or \r\n), not preceeded by a backslash,
       * must be converted to single \n.
       */
    case '\r':
      if (cur < inbufend && *cur == '\n')
        cur++;
      wbuf[len++] = '\n';
      break;
    default:
      wbuf[len++] = c;
    }
  }
  if (c != ')')
    return NULL;

  *inbuf  = cur;
  return pst_string_new(wbuf, len);
}

static pst_string *
pst_string_parse_hex (unsigned char **inbuf, unsigned char *inbufend)
{
  unsigned char  wbuf[PST_STRING_LEN_MAX];
  unsigned char *cur = *inbuf;
  unsigned int   len = 0;

  if (cur + 2 > inbufend || *cur != '<' ||
      (*cur == '<' && *(cur+1) == '<'))
    return NULL;

  cur++;
  /* PDF Reference does not specify how to treat invalid char */
  while (cur < inbufend && len < PST_STRING_LEN_MAX) {
    int    hi, lo;
    skip_white_spaces(&cur, inbufend);
    if (*cur == '>')
      break;
    hi = xtoi(*(cur++));
    if (hi < 0) {
      dpx_warning("Invalid char for hex string <%x> treated as <0>.", *(cur-1));
      hi = 0;
    }
    skip_white_spaces(&cur, inbufend);
    if (*cur == '>')
      break;
    /* 0 is appended if final hex digit is missing */
    lo = (cur < inbufend) ? xtoi(*(cur++)) : 0;
    if (lo < 0) {
      dpx_warning("Invalid char for hex string <%x> treated as <0>.", *(cur-1));
      lo = 0;
    }
    wbuf[len++] = (hi << 4) | lo;
  }
  if (*cur++ != '>')
    return NULL;

  *inbuf = cur;
  return pst_string_new(wbuf, len);
}

static int
pst_string_IV (pst_string *obj)
{
  return (int) pst_string_RV(obj);
}

static double
pst_string_RV (pst_string *obj)
{
  pst_obj *nobj;
  unsigned char  *p, *end;
  double   rv;

  assert(obj);
  p   = obj->value;
  end = p + obj->length;
  nobj = pst_parse_number(&p, end);
  if (nobj == NULL || p != end)
    _tt_abort("Cound not convert string to real value.");
  rv = pst_getRV(nobj);
  pst_release_obj(nobj);

  return rv;
}

static unsigned char *
pst_string_SV (pst_string *obj)
{
  unsigned char *str = NULL;
  assert(obj);
  str = NEW(obj->length + 1, unsigned char);
  memcpy(str, obj->value, obj->length);
  str[obj->length] = '\0';
  return str;
}

static void *
pst_string_data_ptr (pst_string *obj)
{
  assert(obj);
  return obj->value;
}

static unsigned int
pst_string_length (pst_string *obj)
{
  assert(obj);
  return obj->length;
}
