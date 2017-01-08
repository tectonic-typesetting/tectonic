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

#include <ctype.h>
#include <string.h>
/* pow() */
#include <math.h>

#include "system.h"
#include "mem.h"
#include "error.h"

#include "numbers.h"

#include "mfileio.h"

#include "pdfobj.h"
#include "pdfdoc.h"
#include "pdfdev.h"

#include "pdfparse.h"

/* PDF */
#ifdef  is_space
#undef  is_space
#endif
#ifdef  is_delim
#undef  is_delim
#endif

#define is_space(c) ((c) == ' '  || (c) == '\t' || (c) == '\f' || \
		     (c) == '\r' || (c) == '\n' || (c) == '\0')
#define is_delim(c) ((c) == '(' || (c) == ')' || \
                     (c) == '/' || \
                     (c) == '<' || (c) == '>' || \
		     (c) == '[' || (c) == ']' || \
                     (c) == '%')
#define PDF_TOKEN_END(p,e) ((p) >= (e) || is_space(*(p)) || is_delim(*(p)))

#define istokensep(c) (is_space((c)) || is_delim((c)))

static struct {
  int tainted;
} parser_state = {
  0
};

static int xtoi (char ch);

static const char *save = NULL;

void
dump (const char *start, const char *end)
{
  const char *p = start;

#define DUMP_LIMIT 50
  MESG("\nCurrent input buffer is -->");
  while (p < end && p < start + DUMP_LIMIT)
    MESG("%c", *(p++));
  if (p == start+DUMP_LIMIT)
    MESG("...");
  MESG("<--\n");
}

#define SAVE(s,e) do {\
   save = (s);\
 } while (0)
#define DUMP_RESTORE(s,e) do {\
   dump(save, end);\
   (s) = save;\
 } while (0)

void
pdfparse_skip_line (const char **start, const char *end)
{
  while (*start < end && **start != '\n' && **start != '\r')
    (*start)++;
  /* The carriage return (CR; \r; 0x0D) and line feed (LF; \n; 0x0A)
   * characters, also called newline characters, are treated as
   * end-of-line (EOL) markers. The combination of a carriage return
   * followed immediately by a line feed is treated as one EOL marker.
   */
  if (*start < end && **start == '\r')
    (*start)++;
  if (*start < end && **start == '\n')
    (*start)++;
}

void
skip_white (const char **start, const char *end)
{
  /*
   * The null (NUL; 0x00) character is a white-space character in PDF spec
   * but isspace(0x00) returns FALSE; on the other hand, the vertical tab
   * (VT; 0x0B) character is not a white-space character in PDF spec but
   * isspace(0x0B) returns TRUE.
   */
  while (*start < end && (is_space(**start) || **start == '%')) {
    if (**start == '%')
      pdfparse_skip_line(start, end);
    else
      (*start)++;
  }
}


static char *
parsed_string (const char *start, const char *end)
{
  char *result = NULL;
  int   len;

  len = end - start;
  if (len > 0) {
    result = NEW(len + 1, char);
    memcpy(result, start, len);
    result[len] = '\0';
  }

  return result;
}

char *
parse_number (const char **start, const char *end)
{
  char *number;
  const char *p;

  skip_white(start, end);
  p = *start;
  if (p < end && (*p == '+' || *p == '-'))
    p++;
  while (p < end && isdigit((unsigned char)*p))
    p++;
  if (p < end && *p == '.') {
    p++;
    while (p < end && isdigit((unsigned char)*p))
      p++;
  }
  number = parsed_string(*start, p);

  *start = p;
  return number;
}

char *
parse_unsigned (const char **start, const char *end)
{
  char *number;
  const char *p;

  skip_white(start, end);
  for (p = *start; p < end; p++) {
    if (!isdigit((unsigned char)*p))
      break;
  }
  number = parsed_string(*start, p);

  *start = p;
  return number;
}

static char *
parse_gen_ident (const char **start, const char *end, const char *valid_chars)
{
  char *ident;
  const char *p;

  /* No skip_white(start, end)? */
  for (p = *start; p < end; p++) {
    if (!strchr(valid_chars, *p))
      break;
  }
  ident = parsed_string(*start, p);

  *start = p;
  return ident;
}

char *
parse_ident (const char **start, const char *end)
{
  static const char *valid_chars =
    "!\"#$&'*+,-.0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~";

  return parse_gen_ident(start, end, valid_chars);
}

char *
parse_val_ident (const char **start, const char *end)
{
  static const char *valid_chars =
    "!\"#$&'*+,-./0123456789:;?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~";

  return parse_gen_ident(start, end, valid_chars);
}

char *
parse_opt_ident (const char **start, const char *end)
{
  if (*start < end && **start == '@') {
    (*start)++;
    return parse_ident(start, end);
  }

  return NULL;
}

pdf_obj *
parse_pdf_number (const char **pp, const char *endptr)
{
  const char *p;
  double v = 0.0;
  int    nddigits = 0, sign = 1;
  int    has_dot  = 0;

  p = *pp;
  skip_white(&p, endptr);
  if (p >= endptr ||
      (!isdigit((unsigned char)p[0]) && p[0] != '.' &&
       p[0] != '+' && p[0] != '-')) {
    WARN("Could not find a numeric object.");
    return NULL;
  }

  if (p[0] == '-') {
    if (p + 1 >= endptr) {
      WARN("Could not find a numeric object.");
      return NULL;
    }
    sign = -1;
    p++;
  } else if (p[0] == '+') {
    if (p + 1 >= endptr) {
      WARN("Could not find a numeric object.");
      return NULL;
    }
    sign =  1;
    p++;
  }

  while (p < endptr && !istokensep(p[0])) {
    if (p[0] == '.') {
      if (has_dot) { /* Two dots */
        WARN("Could not find a numeric object.");
        return NULL;
      } else {
        has_dot = 1;
      }
    } else if (isdigit((unsigned char)p[0])) {
      if (has_dot) {
        v += (p[0] - '0') / pow(10, nddigits + 1);
        nddigits++;
      } else {
        v = v * 10.0 + p[0] - '0';
      }
    } else {
      WARN("Could not find a numeric object.");
      return NULL;
    }
    p++;
  }

  *pp = p;
  return pdf_new_number(sign * v);
}

/*
 * PDF Name:
 *
 *  PDF-1.2+: Two hexadecimal digits preceded by a number sign.
 */
static int
pn_getc (const char **pp, const char *endptr)
{
  int   ch = 0;
  const char *p;

  p  = *pp;
  if (p[0] == '#') {
    if (p + 2 >= endptr) {
      *pp = endptr;
      return -1;
    }
    if (!isxdigit((unsigned char)p[1]) || !isxdigit((unsigned char)p[2])) {
      *pp += 3;
      return -1;
    }
    ch   = (xtoi(p[1]) << 4);
    ch  += xtoi(p[2]);
    *pp += 3;
  } else {
    ch = p[0];
    *pp += 1;
  }

  return ch;
}

#ifndef PDF_NAME_LEN_MAX
#define PDF_NAME_LEN_MAX 128
#endif

#ifndef PDF_STRING_LEN_MAX
#define PDF_STRING_LEN_MAX 65535
#endif

#define STRING_BUFFER_SIZE PDF_STRING_LEN_MAX+1
static char sbuf[PDF_STRING_LEN_MAX+1];


pdf_obj *
parse_pdf_name (const char **pp, const char *endptr)
{
  char  name[PDF_NAME_LEN_MAX+1];
  int   ch, len = 0;

  skip_white(pp, endptr);
  if (*pp >= endptr || **pp != '/') {
    WARN("Could not find a name object.");
    return NULL;
  }

  (*pp)++;
  while (*pp < endptr && !istokensep(**pp)) {
    ch = pn_getc(pp, endptr);
    if (ch < 0 || ch > 0xff) {
      WARN("Invalid char in PDF name object. (ignored)");
    } else if (ch == 0) {
      WARN("Null char not allowed in PDF name object. (ignored)");
    } else if (len < STRING_BUFFER_SIZE) {
      if (len == PDF_NAME_LEN_MAX) {
	WARN("PDF name length too long. (>= %d bytes)", PDF_NAME_LEN_MAX);
      }
      name[len++] = ch;
    } else {
      WARN("PDF name length too long. (>= %d bytes, truncated)",
	   STRING_BUFFER_SIZE);
    }
  }
  if (len < 1) {
    WARN("No valid name object found.");
    return NULL;
  }
  name[len] = '\0';

  return pdf_new_name(name);
}

pdf_obj *
parse_pdf_boolean (const char **pp, const char *endptr)
{
  skip_white(pp, endptr);
  if (*pp + 4 <= endptr &&
      !strncmp(*pp, "true", 4)) {
    if (*pp + 4 == endptr ||
	istokensep(*(*pp + 4))) {
      *pp += 4;
      return pdf_new_boolean(1);
    }
  } else if (*pp + 5 <= endptr &&
	     !strncmp(*pp, "false", 5)) {
    if (*pp + 5 == endptr ||
	istokensep(*(*pp + 5))) {
      *pp += 5;
      return pdf_new_boolean(0);
    }
  }

  WARN("Not a boolean object.");

  return NULL;
}

pdf_obj *
parse_pdf_null (const char **pp, const char *endptr)
{
  skip_white(pp, endptr);
  if (*pp + 4 > endptr) {
    WARN("Not a null object.");
    return NULL;
  } else if (*pp + 4 < endptr &&
	     !istokensep(*(*pp+4))) {
    WARN("Not a null object.");
    return NULL;
  } else if (!strncmp(*pp, "null", 4)) {
    *pp += 4;
    return pdf_new_null();
  }

  WARN("Not a null object.");

  return NULL;
}

/*
 * PDF Literal String
 */
#ifndef isodigit
#define isodigit(c) ((c) >= '0' && (c) <= '7')
#endif
static int
ps_getescc (const char **pp, const char *endptr)
{
  int   ch, i;
  const char *p;

  p = *pp + 1; /* backslash assumed. */
  switch (p[0]) {
  case 'n': ch = '\n'; p++; break;
  case 'r': ch = '\r'; p++; break;
  case 't': ch = '\t'; p++; break;
  case 'b': ch = '\b'; p++; break;
  case 'f': ch = '\f'; p++; break;

    /*
     * An end-of-line marker preceded by a backslash must be ignored.
     */
  case '\n':
    ch = -1;
    p++;
    break;
  case '\r':
    ch = -1;
    p++;
    if (p < endptr && p[0] == '\n')
      p++;
    break;
  default:
    if (p[0] == '\\' ||
	p[0] == '('  || p[0] == ')') {
      ch = p[0];
      p++;
    } else if (isodigit(p[0])) {
      ch = 0;
      /* Don't forget isodigit() is a macro. */
      for (i = 0; i < 3 &&
	     p < endptr && isodigit(p[0]); i++) {
        ch = (ch << 3) + (p[0] - '0');
	p++;
      }
      ch = (ch & 0xff); /* Ignore overflow. */
    } else {
      ch = ((unsigned char) p[0]); /* Ignore only backslash. */
      p++;
    }
  }

  *pp = p;
  return ch;
}

static pdf_obj *
parse_pdf_literal_string (const char **pp, const char *endptr)
{
  int    ch, op_count = 0, len = 0;
  const char  *p;

  p = *pp;

  skip_white(&p, endptr);

  if (p >= endptr || p[0] != '(')
    return NULL;

  p++;

  /* The carriage return (CR, 0x0d) and line feed (LF, 0x0a) characters,
   * also called newline characters, are treated as end-of-line (EOL)
   * markers. The combination of a carriage return followed immediately
   * by a line feed is treated as one EOL marker.
   * [PDF Reference, 6th ed., version 1.7, p. 50] */

  /* If an end-of-line marker appears within a literal string
   * without a preceding backslash, the result is equivalent to
   * \n (regardless of whether the end-of-line marker was
   * a carriage return, a line feed, or both).
   * [PDF Reference, 6th ed., version 1.7, p. 55] */

  while (p < endptr) {

    ch = p[0];

    if (ch == ')' && op_count < 1)
      break;

#ifndef PDF_PARSE_STRICT
    if (parser_state.tainted) {
      if (p + 1 < endptr && (ch & 0x80)) {
	if (len + 2 >= PDF_STRING_LEN_MAX) {
	  WARN("PDF string length too long. (limit: %ld)",
	       PDF_STRING_LEN_MAX);
	  return NULL;
	}
	sbuf[len++] = p[0];
	sbuf[len++] = p[1];
	p += 2;
	continue;
      }
    }
#endif /* !PDF_PARSE_STRICT */

    if (len + 1 >= PDF_STRING_LEN_MAX) {
      WARN("PDF string length too long. (limit: %ld)",
	   PDF_STRING_LEN_MAX);
      return NULL;
    }

    switch (ch) {
    case '\\':
      ch = ps_getescc(&p, endptr);
      if (ch >= 0)
	sbuf[len++] = (ch & 0xff);
      break;
    case '\r':
      p++;
      if (p < endptr && p[0] == '\n')
	p++;
      sbuf[len++] = '\n';
      break;
    default:
      if (ch == '(')
	op_count++;
      else if (ch == ')')
	op_count--;
      sbuf[len++] = ch;
      p++;
      break;
    }
  }

  if (op_count > 0 ||
      p >= endptr  || p[0] != ')') {
    WARN("Unbalanced parens/truncated PDF literal string.");
    return NULL;
  }

  *pp = p + 1;
  return pdf_new_string(sbuf, len);
}

/*
 * PDF Hex String
 */
static int
xtoi (char ch)
{
  if (ch >= '0' && ch <= '9')
    return ch - '0';
  if (ch >= 'A' && ch <= 'F')
    return (ch - 'A') + 10;
  if (ch >= 'a' && ch <= 'f')
    return (ch - 'a') + 10;

  return -1;
}

static pdf_obj *
parse_pdf_hex_string (const char **pp, const char *endptr)
{
  const char  *p;
  int   len;

  p = *pp;

  skip_white(&p, endptr);
  if (p >= endptr || p[0] != '<')
    return NULL;

  p++;

  len = 0;
  /*
   * PDF Reference does not describe how to treat invalid char.
   * Zero is appended if final hex digit is missing.
   */
  while (p < endptr && p[0] != '>' && len < PDF_STRING_LEN_MAX) {
    int  ch;

    skip_white(&p, endptr);
    if (p >= endptr || p[0] == '>')
      break;

    ch = (xtoi(p[0]) << 4);
    p++;

    skip_white(&p, endptr);
    if (p < endptr && p[0] != '>') {
      ch += xtoi(p[0]);
      p++;
    }
    sbuf[len++] = (ch & 0xff);
  }

  if (p >= endptr) {
    WARN("Premature end of input hex string.");
    return NULL;
  } else if (p[0] != '>') {
    WARN("PDF string length too long. (limit: %ld)", PDF_STRING_LEN_MAX);
    return NULL;
  }

  *pp = p + 1;
  return pdf_new_string(sbuf, len);
}

pdf_obj *
parse_pdf_string (const char **pp, const char *endptr)
{
  skip_white(pp, endptr);
  if (*pp + 2 <= endptr) {
    if (**pp == '(')
      return parse_pdf_literal_string(pp, endptr);
    else if (**pp == '<' &&
	     (*(*pp + 1) == '>' || isxdigit((unsigned char)*(*pp + 1)))) {
      return parse_pdf_hex_string(pp, endptr);
    }
  }

  WARN("Could not find a string object.");

  return NULL;
}

#ifndef PDF_PARSE_STRICT
pdf_obj *
parse_pdf_tainted_dict (const char **pp, const char *endptr)
{
  pdf_obj *result;

  parser_state.tainted = 1;
  result  = parse_pdf_dict(pp, endptr, NULL);
  parser_state.tainted = 0;

  return result;
}
#else /* PDF_PARSE_STRICT */
pdf_obj *
parse_pdf_tainted_dict (const char **pp, const char *endptr)
{
  return parse_pdf_dict(pp, endptr, NULL);
}
#endif /* !PDF_PARSE_STRICT */

pdf_obj *
parse_pdf_dict (const char **pp, const char *endptr, pdf_file *pf)
{
  pdf_obj *result = NULL;
  const char *p;

  p = *pp;

  skip_white(&p, endptr);

  /* At least four letter <<>>. */
  if (p + 4 > endptr ||
      p[0] != '<'    || p[1] != '<') {
    return NULL;
  }
  p += 2;

  result = pdf_new_dict();

  skip_white(&p, endptr);
  while (p < endptr && p[0] != '>') {
    pdf_obj *key, *value;

    skip_white(&p, endptr);
    key = parse_pdf_name(&p, endptr);
    if (!key) {
      WARN("Could not find a key in dictionary object.");
      pdf_release_obj(result);
      return NULL;
    }

    skip_white(&p, endptr);

    value = parse_pdf_object(&p, endptr, pf);
    if (!value) {
      pdf_release_obj(key); 
      pdf_release_obj(value);
      pdf_release_obj(result);
      WARN("Could not find a value in dictionary object.");
      return NULL;
    }
    pdf_add_dict(result, key, value);

    skip_white(&p, endptr);
  }

  if (p + 2 > endptr ||
      p[0] != '>'    || p[1] != '>') {
    WARN("Syntax error: Dictionary object ended prematurely.");
    pdf_release_obj(result);
    return NULL;
  }

  *pp = p + 2; /* skip >> */
  return result;
}

pdf_obj *
parse_pdf_array (const char **pp, const char *endptr, pdf_file *pf)
{
  pdf_obj *result;
  const char *p;

  p = *pp;

  skip_white(&p, endptr);
  if (p + 2 > endptr || p[0] != '[') {
    WARN("Could not find an array object.");
    return NULL;
  }

  result = pdf_new_array();

  p++;
  skip_white(&p, endptr);

  while (p < endptr && p[0] != ']') {
    pdf_obj *elem;

    elem = parse_pdf_object(&p, endptr, pf);
    if (!elem) {
      pdf_release_obj(result); 
      WARN("Could not find a valid object in array object.");
      return NULL;
    }
    pdf_add_array(result, elem);

    skip_white(&p, endptr);
  }

  if (p >= endptr || p[0] != ']') {
    WARN("Array object ended prematurely.");
    pdf_release_obj(result);
    return NULL;
  }

  *pp = p + 1; /* skip ] */
  return result;
}

static pdf_obj *
parse_pdf_stream (const char **pp, const char *endptr, pdf_obj *dict)
{
  pdf_obj *result = NULL;
  const char *p;
  pdf_obj *stream_dict;
  int      stream_length;

  p = *pp;
  skip_white(&p, endptr);
  if (p + 6 > endptr ||
      strncmp(p, "stream", 6)) {
    return NULL;
  }
  p += 6;

  /* The keyword stream that follows the stream dictionary
   * should be followed by an end-of-line marker consisting of
   * either a carriage return (0x0D;\r) and a line feed (0x0A;\n)
   * or just a line feed, and not by a carriage return alone.
   * [PDF Reference, 6th ed., version 1.7, pp. 60-61] */

  /* Notice that TeX translates an end-of-line marker to a single space. */
  if (p < endptr && p[0] == '\n') {
    p++;
  } else if (p + 1 < endptr &&
             (p[0] == '\r' && p[1] == '\n')) {
    p += 2;
  }

  /* Stream length */
  {
    pdf_obj *tmp, *tmp2;

    tmp = pdf_lookup_dict(dict, "Length");
 
    if (tmp != NULL) {
      tmp2 = pdf_deref_obj(tmp);
      if (pdf_obj_typeof(tmp2) != PDF_NUMBER)
        stream_length = -1;
      else {
        stream_length = (int) pdf_number_value(tmp2);
      }
      pdf_release_obj(tmp2);
    }
    else {
      return NULL;
    }
  }

  
  if (stream_length < 0 ||
      p + stream_length > endptr)
    return NULL;

  /*
   * If Filter is not applied, set STREAM_COMPRESS flag.
   * Should we use filter for ASCIIHexEncode/ASCII85Encode-ed streams?
   */
  {
    pdf_obj *filters;

    filters = pdf_lookup_dict(dict, "Filter");
    if (!filters && stream_length > 10) {
      result = pdf_new_stream(STREAM_COMPRESS);
    } else {
      result = pdf_new_stream(0);
    }
  }

  stream_dict = pdf_stream_dict(result);
  pdf_merge_dict(stream_dict, dict);

  pdf_add_stream(result, p, stream_length);
  p += stream_length;

  /* Check "endsteam" */
  {
    /* It is recommended that there be an end-of-line marker
     * after the data and before endstream; this marker is not included
     * in the stream length. 
     * [PDF Reference, 6th ed., version 1.7, pp. 61] */
    if (p < endptr && p[0] == '\r')
      p++;
    if (p < endptr && p[0] == '\n')
      p++;

    if (p + 9 > endptr ||
        memcmp(p, "endstream", 9)) {
      pdf_release_obj(result);
      return NULL;
    }
    p += 9;
  }

  *pp = p;
  return  result;
}

#ifndef PDF_PARSE_STRICT

/* PLEASE REMOVE THIS */
#include "specials.h"

/* This is not PDF indirect reference. */
static pdf_obj *
parse_pdf_reference (const char **start, const char *end)
{
  pdf_obj *result = NULL;
  char    *name;

  SAVE(*start, end);

  skip_white(start, end);
  name = parse_opt_ident(start, end);
  if (name) {
    result = spc_lookup_reference(name);
    if (!result) {
      WARN("Could not find the named reference (@%s).", name);
      DUMP_RESTORE(*start, end);
    }
    RELEASE(name);
  } else {
    WARN("Could not find a reference name.");
    DUMP_RESTORE(*start, end);
    result = NULL;
  }

  return result;
}
#endif /* !PDF_PARSE_STRICT */

static pdf_obj *
try_pdf_reference (const char *start, const char *end, const char **endptr, pdf_file *pf)
{
  unsigned id = 0;
  unsigned short gen = 0;

  ASSERT(pf);

  if (endptr)
    *endptr = start;

  skip_white(&start, end);
  if (start > end - 5 || !isdigit((unsigned char)*start)) {
    return NULL;
  }
  while (!is_space(*start)) {
    if (start >= end || !isdigit((unsigned char)*start)) {
      return NULL;
    }
    id = id * 10 + (*start - '0');
    start++;
  }

  skip_white(&start, end);
  if (start >= end || !isdigit((unsigned char)*start))
    return NULL;
  while (!is_space(*start)) {
    if (start >= end || !isdigit((unsigned char)*start))
      return NULL;
    gen = gen * 10 + (*start - '0');
    start++;
  }

  skip_white(&start, end);
  if (start >= end  || *start != 'R')
    return NULL;
  start++;
  if (!PDF_TOKEN_END(start, end))
    return NULL;
    
  if (endptr)
    *endptr = start;

  return pdf_new_indirect(pf, id, gen);
}

pdf_obj *
parse_pdf_object (const char **pp, const char *endptr, pdf_file *pf)
/* If pf is NULL, then indirect references are not allowed */
{
  pdf_obj *result = NULL;
  const char *nextptr;

  skip_white(pp, endptr);
  if (*pp >= endptr) {
    WARN("Could not find any valid object.");
    return NULL;
  }

  switch (**pp) {

  case '<': 

    if (*(*pp + 1) != '<') {
      result = parse_pdf_hex_string(pp, endptr);
    } else {
      pdf_obj *dict;

      result = parse_pdf_dict(pp, endptr, pf);
      skip_white(pp, endptr);
      if ( result &&
          *pp <= endptr - 15 &&
          !memcmp(*pp, "stream", 6)) {
        dict   = result;
        result = parse_pdf_stream(pp, endptr, dict);
        pdf_release_obj(dict);
      }
    }

    break;
  case '(':
    result = parse_pdf_string(pp, endptr);
    break;
  case '[':
    result = parse_pdf_array(pp, endptr, pf);
    break;
  case '/':
    result = parse_pdf_name(pp, endptr);
    break;
  case 'n':
    result = parse_pdf_null(pp, endptr);
    break;
  case 't': case 'f':
    result = parse_pdf_boolean(pp, endptr);
    break;
  case '+': case '-': case '.':
    result = parse_pdf_number(pp, endptr);
    break;
  case '0': case '1': case '2': case '3': case '4':
  case '5': case '6': case '7': case '8': case '9':

    /*
     * If pf != NULL, then we are parsing a PDF file,
     * and indirect references are allowed.
     */
    if (pf && (result = try_pdf_reference(*pp, endptr, &nextptr, pf))) {
      *pp = nextptr;
    } else {
      result = parse_pdf_number(pp, endptr);
    }
    break;

  case '@':

#ifndef PDF_PARSE_STRICT
    result = parse_pdf_reference(pp, endptr);
#endif /* !PDF_PARSE_STRICT */
    break;

  default:
    WARN("Unknown PDF object type.");
    result = NULL;
  }

  return result;
}

