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

#include "dpx-pst.h"

#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-dpxutil.h"
#include "dpx-mem.h"
#include "dpx-pst_obj.h"


#define TYPE_CHECK(o, t) do { \
                             if ((o) == NULL || pst_type_of((o)) != (t)) \
                                  _tt_abort("typecheck: object %p not of type %d.", (o), (t)); \
                             } while (0)

static pst_obj *
pst_parse_any (unsigned char **inbuf, unsigned char *inbufend)
{
  unsigned char *data;
  unsigned char *cur = *inbuf;
  unsigned int len;

  while (cur < inbufend && !PST_TOKEN_END(cur, inbufend))
    cur++;

  len = cur - (*inbuf);
  data = NEW(len+1, unsigned char);
  memcpy(data, *inbuf, len);
  data[len] = '\0';

  *inbuf = cur;
  return pst_new_obj(PST_TYPE_UNKNOWN, data);
}

static void
skip_line (unsigned char **inbuf, unsigned char *inbufend)
{
  while (*inbuf < inbufend && **inbuf != '\n' && **inbuf != '\r')
    (*inbuf)++;
  if (*inbuf < inbufend && **inbuf == '\r')
    (*inbuf)++;
  if (*inbuf < inbufend && **inbuf == '\n')
    (*inbuf)++;
}

static void
skip_comments (unsigned char **inbuf, unsigned char *inbufend)
{
  while (*inbuf < inbufend && **inbuf == '%') {
    skip_line(inbuf, inbufend);
    skip_white_spaces(inbuf, inbufend);
  }
}

/* NOTE: the input buffer must be null-terminated, i.e., *inbufend == 0 */
pst_obj *
pst_get_token (unsigned char **inbuf, unsigned char *inbufend)
{
  pst_obj *obj = NULL;
  unsigned char c;

  assert(*inbuf <= inbufend && !*inbufend);

  skip_white_spaces(inbuf, inbufend);
  skip_comments(inbuf, inbufend);
  if (*inbuf >= inbufend)
    return NULL;
  c = **inbuf;
  switch (c) {
  case '/':
    obj = pst_parse_name(inbuf, inbufend);
    break;
  case '[': case '{': /* This is wrong */
    obj = pst_new_mark();
    (*inbuf)++;
    break;
  case '<':
    if (*inbuf + 1 >= inbufend)
      return NULL;
    c = *(*inbuf+1);
    if (c == '<') {
      obj = pst_new_mark();
      *inbuf += 2;
    } else if (isxdigit(c))
      obj = pst_parse_string(inbuf, inbufend);
    else if (c == '~') /* ASCII85 */
      obj = pst_parse_string(inbuf, inbufend);
    break;
  case '(':
    obj = pst_parse_string(inbuf, inbufend);
    break;
  case '>':
    if (*inbuf + 1 >= inbufend || *(*inbuf+1) != '>') {
      _tt_abort("Unexpected end of ASCII hex string marker.");
    } else  {
      char *mark;

      mark = NEW(3, char);
      mark[0] = '>'; mark[1] = '>'; mark[2] = '\0';
      obj = pst_new_obj(PST_TYPE_UNKNOWN, mark);
      (*inbuf) += 2;
    }
    break;
  case ']': case '}':
    {
      char *mark;

      mark = NEW(2, char);
      mark[0] = c; mark[1] = '\0';
      obj = pst_new_obj(PST_TYPE_UNKNOWN, mark);
      (*inbuf)++;
    }
    break;
  default:
    if (c == 't' || c == 'f')
      obj = pst_parse_boolean(inbuf, inbufend);
    else if (c == 'n')
      obj = pst_parse_null(inbuf, inbufend);
    else if (c == '+' || c == '-' || isdigit(c) || c == '.')
      obj = pst_parse_number(inbuf, inbufend);
    break;
  }

  if (!obj) {
    obj = pst_parse_any(inbuf, inbufend);
  }

  return obj;
}
