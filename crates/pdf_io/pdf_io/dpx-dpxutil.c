/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include "dpx-dpxutil.h"

#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-mem.h"

int
xtoi (char c)
{
  if (c >= '0' && c <= '9')
    return c - '0';
  if (c >= 'A' && c <= 'F')
    return (c - 'A') + 10;
  if (c >= 'a' && c <= 'f')
    return (c - 'a') + 10;

  return -1;
}


double
min4 (double x1, double x2, double x3, double x4)
{
    double v = x1;
    if (x2 < v) v = x2;
    if (x3 < v) v = x3;
    if (x4 < v) v = x4;
    return v;
}


double
max4 (double x1, double x2, double x3, double x4)
{
    double v = x1;
    if (x2 > v) v = x2;
    if (x3 > v) v = x3;
    if (x4 > v) v = x4;
    return v;
}

/* Duplicate from pdfparse.c */
static void
skip_white (const char **pp, const char *endptr)
{
    while (*pp < endptr &&
          (**pp == ' '  || **pp == '\t' || **pp == '\f' ||
            **pp == '\r' || **pp == '\n' || **pp == '\0')) {
        (*pp)++;
    }
}

/* This need to allow 'true' prefix for unit and length value must be divided
 * by current magnification.
 */
int
dpx_util_read_length (double *vp, double mag, const char **pp, const char *endptr)
{
    char   *q;
    const char *p = *pp;
    double  v, u = 1.0;
    const char *_ukeys[] = {
#define K_UNIT__PT  0
#define K_UNIT__IN  1
#define K_UNIT__CM  2
#define K_UNIT__MM  3
#define K_UNIT__BP  4
#define K_UNIT__PC  5
#define K_UNIT__DD  6
#define K_UNIT__CC  7
#define K_UNIT__SP  8
      "pt", "in", "cm", "mm", "bp", "pc", "dd", "cc", "sp",
      NULL
    };
    int     k, error = 0;

    q = parse_float_decimal(&p, endptr);
    if (!q) {
        *vp = 0.0; *pp = p;
        return  -1;
    }

    v = atof(q);
    free(q);

    skip_white(&p, endptr);
    q = parse_c_ident(&p, endptr);
    if (q) {
        char *qq = q; /* remember this for RELEASE, because q may be advanced */
        if (strlen(q) >= strlen("true") &&
            !memcmp(q, "true", strlen("true"))) {
            u /= mag != 0.0 ? mag : 1.0; /* inverse magnify */
            q += strlen("true");
        }
        if (strlen(q) == 0) { /* "true" was a separate word from the units */
            free(qq);
            skip_white(&p, endptr);
            qq = q = parse_c_ident(&p, endptr);
        }
        if (q) {
            for (k = 0; _ukeys[k] && strcmp(_ukeys[k], q); k++);
            switch (k) {
            case K_UNIT__PT: u *= 72.0 / 72.27; break;
            case K_UNIT__IN: u *= 72.0; break;
            case K_UNIT__CM: u *= 72.0 / 2.54 ; break;
            case K_UNIT__MM: u *= 72.0 / 25.4 ; break;
            case K_UNIT__BP: u *= 1.0 ; break;
            case K_UNIT__PC: u *= 12.0 * 72.0 / 72.27 ; break;
            case K_UNIT__DD: u *= 1238.0 / 1157.0 * 72.0 / 72.27 ; break;
            case K_UNIT__CC: u *= 12.0 * 1238.0 / 1157.0 * 72.0 / 72.27 ; break;
            case K_UNIT__SP: u *= 72.0 / (72.27 * 65536) ; break;
            default:
                dpx_warning("Unknown unit of measure: %s", q);
                error = -1;
                break;
            }
            free(qq);
        }
        else {
            dpx_warning("Missing unit of measure after \"true\"");
            error = -1;
        }
    }

    *vp = v * u; *pp = p;
    return  error;
}


#if defined(_MSC_VER)
#define strtoll _strtoi64
#endif

/* If an environment variable SOURCE_DATE_EPOCH is correctly defined like
 * SOURCE_DATE_EPOCH=1456304492, then returns this value, to be used as the
 * 'current time', otherwise returns INVALID_EPOCH_VALUE (= (time_t)-1).
 * In the case of Microsoft Visual Studio 2010, the value should be less
 * than 32535291600.
 */

time_t
dpx_util_get_unique_time_if_given(void)
{
  const char *source_date_epoch;
  int64_t epoch;
  char *endptr;
  time_t ret = INVALID_EPOCH_VALUE;

  source_date_epoch = getenv("SOURCE_DATE_EPOCH");
  if (source_date_epoch) {
    errno = 0;
    epoch = strtoll(source_date_epoch, &endptr, 10);
    if (!(epoch < 0 || *endptr != '\0' || errno != 0)) {
      ret = (time_t) epoch;
#if defined(_MSC_VER)
      if (ret > 32535291599ULL)
        ret = 32535291599ULL;
#endif
    }
  }
  return ret;
}


/*
 * Docinfo
 */
int
dpx_util_format_asn_date (char *date_string, int need_timezone)
{
  int32_t     tz_offset = 0;
  struct tm  *bd_time = gmtime(&ttpi_source_date_epoch);

  if (need_timezone) {
    if (bd_time->tm_isdst > 0) {
      tz_offset += 3600;
    }
    sprintf(date_string, "D:%04d%02d%02d%02d%02d%02d%c%02d'%02d'",
            bd_time->tm_year + 1900, bd_time->tm_mon + 1, bd_time->tm_mday,
            bd_time->tm_hour, bd_time->tm_min, bd_time->tm_sec,
            (tz_offset > 0) ? '+' : '-', abs(tz_offset) / 3600,
                                        (abs(tz_offset) / 60) % 60);
  } else {
    sprintf(date_string, "D:%04d%02d%02d%02d%02d%02d",
            bd_time->tm_year + 1900, bd_time->tm_mon + 1, bd_time->tm_mday,
            bd_time->tm_hour, bd_time->tm_min, bd_time->tm_sec);
  }

  return strlen(date_string);
}

void
skip_white_spaces (unsigned char **s, unsigned char *endptr)
{
  while (*s < endptr)
    if (!is_space(**s))
      break;
    else
      (*s)++;
}

void
dpx_stack_init (dpx_stack *stack)
{
  stack->size   = 0;
  stack->top    = NULL;
  stack->bottom = NULL;
}

void
dpx_stack_push (dpx_stack *stack, void *data)
{
  stack_elem  *elem;

  assert(stack);

  elem = NEW(1, stack_elem);
  elem->prev = stack->top;
  elem->data = data;

  stack->top = elem;
  if (stack->size == 0)
    stack->bottom = elem;

  stack->size++;

  return;
}

void *
dpx_stack_pop (dpx_stack *stack)
{
  stack_elem *elem;
  void       *data;

  assert(stack);

  if (stack->size == 0)
    return NULL;

  data = stack->top->data;
  elem = stack->top;
  stack->top = elem->prev;
  if (stack->size == 1)
    stack->bottom = NULL;
  free(elem);

  stack->size--;

  return data;
}

void *
dpx_stack_top (dpx_stack *stack)
{
  void  *data;

  assert(stack);

  if (stack->size == 0)
    return NULL;

  data = stack->top->data;

  return data;
}

void *
dpx_stack_at (dpx_stack *stack, int pos)
{
  void       *data = NULL;
  stack_elem *elem;

  if (stack->size == 0)
    return NULL;

  elem = stack->top;
  while (pos > 0) {
    elem = elem->prev;
    pos--;
  }
  if (elem)
    data = elem->data;

  return data;
}

void
dpx_stack_roll (dpx_stack *stack, int n, int j)
{
  if (n > stack->size)
    return;
  if (n == 1)
    return;
  j = j % n;
  if (j < 0)
    j = n + j;
  while (j-- > 0) {
    int         m = n;
    stack_elem *elem, *prev, *top;

    elem = top = stack->top;
    while (--m > 0) {
      elem = elem->prev;
    }
    prev = elem->prev;
    stack->top = top->prev;
    elem->prev = top;
    top->prev  = prev;
  }
}

int
dpx_stack_depth (dpx_stack *stack)
{
  assert(stack);

  return stack->size;
}

void
ht_init_table (struct ht_table *ht, hval_free_func hval_free_fn)
{
  int  i;

  assert(ht);

  for (i = 0; i < HASH_TABLE_SIZE; i++) {
    ht->table[i] = NULL;
  }
  ht->count = 0;
  ht->hval_free_fn = hval_free_fn;
}

void
ht_clear_table (struct ht_table *ht)
{
  int   i;

  assert(ht);

  for (i = 0; i < HASH_TABLE_SIZE; i++) {
    struct ht_entry *hent, *next;

    hent = ht->table[i];
    while (hent) {
      if (hent->value && ht->hval_free_fn) {
        ht->hval_free_fn(hent->value);
      }
      hent->value  = NULL;
      if (hent->key) {
        free(hent->key);
      }
      hent->key = NULL;
      next = hent->next;
      free(hent);
      hent = next;
    }
    ht->table[i] = NULL;
  }
  ht->count = 0;
  ht->hval_free_fn = NULL;
}

int ht_table_size (struct ht_table *ht)
{
  assert(ht);

  return ht->count;
}

static unsigned int
get_hash (const void *key, int keylen)
{
  unsigned int hkey = 0;
  int      i;

  for (i = 0; i < keylen; i++) {
    hkey = (hkey << 5) + hkey + ((const char *)key)[i];
  }

  return (hkey % HASH_TABLE_SIZE);
}

void *
ht_lookup_table (struct ht_table *ht, const void *key, int keylen)
{
  struct ht_entry *hent;
  unsigned int     hkey;

  assert(ht && key);

  hkey = get_hash(key, keylen);
  hent = ht->table[hkey];
  while (hent) {
    if (hent->keylen == keylen &&
        !memcmp(hent->key, key, keylen)) {
      return hent->value;
    }
    hent = hent->next;
  }

  return NULL;
}

int
ht_remove_table (struct ht_table *ht,
                 const void *key, int keylen)
/* returns 1 if the element was found and removed and 0 otherwise */
{
  struct ht_entry *hent, *prev;
  unsigned int     hkey;

  assert(ht && key);

  hkey = get_hash(key, keylen);
  hent = ht->table[hkey];
  prev = NULL;
  while (hent) {
    if (hent->keylen == keylen &&
        !memcmp(hent->key, key, keylen)) {
      break;
    }
    prev = hent;
    hent = hent->next;
  }
  if (hent) {
    hent->key = mfree(hent->key);
    hent->keylen = 0;
    if (hent->value && ht->hval_free_fn) {
      ht->hval_free_fn(hent->value);
    }
    hent->value  = NULL;
    if (prev) {
      prev->next = hent->next;
    } else {
      ht->table[hkey] = hent->next;
    }
    free(hent);
    ht->count--;
    return 1;
  } else
    return 0;
}

/* replace... */
void
ht_insert_table (struct ht_table *ht,
                 const void *key, int keylen, void *value)
{
  struct ht_entry *hent, *prev;
  unsigned int     hkey;

  assert(ht && key);

  hkey = get_hash(key, keylen);
  hent = ht->table[hkey];
  prev = NULL;
  while (hent) {
    if (hent->keylen == keylen &&
        !memcmp(hent->key, key, keylen)) {
      break;
    }
    prev = hent;
    hent = hent->next;
  }
  if (hent) {
      if (hent->value && ht->hval_free_fn)
        ht->hval_free_fn(hent->value);
      hent->value  = value;
  } else {
    hent = NEW(1, struct ht_entry);
    hent->key = NEW(keylen, char);
    memcpy(hent->key, key, keylen);
    hent->keylen = keylen;
    hent->value  = value;
    hent->next   = NULL;
    if (prev) {
      prev->next      = hent;
    } else {
      ht->table[hkey] = hent;
    }
    ht->count++;
  }
}

void
ht_append_table (struct ht_table *ht,
                 const void *key, int keylen, void *value)
{
  struct ht_entry *hent, *last;
  unsigned int hkey;

  hkey = get_hash(key, keylen);
  hent = ht->table[hkey];
  if (!hent) {
    hent = NEW(1, struct ht_entry);
    ht->table[hkey] = hent;
  } else {
    while (hent) {
      last = hent;
      hent = hent->next;
    }
    hent = NEW(1, struct ht_entry);
    last->next = hent;
  }
  hent->key = NEW(keylen, char);
  memcpy(hent->key, key, keylen);
  hent->keylen = keylen;
  hent->value  = value;
  hent->next   = NULL;

  ht->count++;
}

int
ht_set_iter (struct ht_table *ht, struct ht_iter *iter)
{
  int    i;

  assert(ht && iter);

  for (i = 0; i < HASH_TABLE_SIZE; i++) {
    if (ht->table[i]) {
      iter->index = i;
      iter->curr  = ht->table[i];
      iter->hash  = ht;
      return 0;
    }
  }

  return -1;
}

void
ht_clear_iter (struct ht_iter *iter)
{
  if (iter) {
    iter->index = HASH_TABLE_SIZE;
    iter->curr  = NULL;
    iter->hash  = NULL;
  }
}

char *
ht_iter_getkey (struct ht_iter *iter, int *keylen)
{
  struct ht_entry *hent;

  hent = (struct ht_entry *) iter->curr;
  if (iter && hent) {
    *keylen = hent->keylen;
    return hent->key;
  } else {
    *keylen = 0;
    return NULL;
  }
}

void *
ht_iter_getval (struct ht_iter *iter)
{
  struct ht_entry *hent;

  hent = (struct ht_entry *) iter->curr;
  if (iter && hent) {
    return hent->value;
  } else {
    return NULL;
  }
}

int
ht_iter_next (struct ht_iter *iter)
{
  struct ht_entry *hent;
  struct ht_table *ht;

  assert(iter);

  ht   = iter->hash;
  hent = (struct ht_entry *) iter->curr;
  hent = hent->next;
  while (!hent &&
         ++iter->index < HASH_TABLE_SIZE) {
    hent = ht->table[iter->index];
  }
  iter->curr = hent;

  return (hent ? 0 : -1);
}


static int
read_c_escchar (char *r, const char **pp, const char *endptr)
{
  int   c = 0, l = 1;
  const char *p = *pp;

  switch (p[0]) {
  case 'a' : c = '\a'; p++; break;
  case 'b' : c = '\b'; p++; break;
  case 'f' : c = '\f'; p++; break;
  case 'n' : c = '\n'; p++; break;
  case 'r' : c = '\r'; p++; break;
  case 't' : c = '\t'; p++; break;
  case 'v' : c = '\v'; p++; break;
  case '\\': case '?': case '\'': case '\"':
    c = p[0]; p++;
    break;
  case '\n': l = 0; p++; break;
  case '\r':
    {
      p++;
      if (p < endptr && p[0] == '\n')
        p++;
      l = 0;
    }
    break;
  case '0': case '1': case '2': case '3':
  case '4': case '5': case '6': case '7':
    {
      int  i;
      for (c = 0, i = 0;
           i < 3 && p < endptr &&
           p[0] >= '0' && p[0] <= '7'; i++, p++)
        c = (c << 3) + (p[0] - '0');
    }
    break;
  case 'x':
    {
      int  i;
      for (c = 0, i = 0, p++;
           i < 2 && p < endptr && isxdigit((unsigned char)p[0]);
           i++, p++)
        c = (c << 4) +
            (isdigit((unsigned char)p[0]) ?
             p[0] - '0' :
             (islower((unsigned char)p[0]) ? p[0] - 'a' + 10: p[0] - 'A' + 10));
    }
    break;
  default:
    dpx_warning("Unknown escape char sequence: \\%c", p[0]);
    l = 0; p++;
    break;
  }

  if (r)
    *r = (char) c;
  *pp  = p;
  return  l;
}

#define C_QUOTE  '"'
#define C_ESCAPE '\\'
static int
read_c_litstrc (char *q, int len, const char **pp, const char *endptr)
{
  const char *p;
  int    l = 0;
#define Q_TERM          0
#define Q_CONT         -1
#define Q_ERROR_UNTERM -1
#define Q_ERROR_INVAL  -2
#define Q_ERROR_BUFF   -3
  int    s = Q_CONT;

  for (l = 0, p = *pp;
       s == Q_CONT && p < endptr; ) {
    switch (p[0]) {
    case C_QUOTE:
      s = Q_TERM; p++;
      break;
    case C_ESCAPE:
      if (q && l == len)
        s = Q_ERROR_BUFF;
      else {
        p++;
        l += read_c_escchar(q ? &q[l] : NULL, &p, endptr);
      }
      break;
    case '\n': case '\r':
      s = Q_ERROR_INVAL;
      break;
    default:
      if (q && l == len)
        s = Q_ERROR_BUFF;
      else {
        if (!q)
          l++;
        else
          q[l++] = p[0];
        p++;
      }
      break;
    }
  }
  if (s == Q_TERM) {
    if (q && l == len)
      s = Q_ERROR_BUFF;
    else if (q)
      q[l++] = '\0';
  }

  *pp = p;
  return ((s == Q_TERM) ? l : s);
}

char *
parse_c_string (const char **pp, const char *endptr)
{
  char  *q = NULL;
  const char *p = *pp;
  int    l = 0;

  if (p >= endptr || p[0] != C_QUOTE)
    return NULL;

  p++;
  l = read_c_litstrc(NULL, 0, &p, endptr);
  if (l >= 0) {
    q = NEW(l + 1, char);
    p = *pp + 1;
    l = read_c_litstrc(q, l + 1, &p, endptr);
  }

  *pp = p;
  return  q;
}

#define ISCNONDIGITS(c) ( \
  (c) == '_' || \
  ((c) >= 'a' && (c) <= 'z') || \
  ((c) >= 'A' && (c) <= 'Z') \
)
#define ISCIDENTCHAR(c) ( \
  ISCNONDIGITS((c)) || \
  ((c) >= '0' && (c) <= '9') \
)

char *
parse_c_ident (const char **pp, const char *endptr)
{
  char  *q = NULL;
  const char *p = *pp;
  int    n;

  if (p >= endptr || !ISCNONDIGITS(*p))
    return NULL;

  for (n = 0; p < endptr && ISCIDENTCHAR(*p); p++, n++);
  q = NEW(n + 1, char);
  memcpy(q, *pp, n); q[n] = '\0';

  *pp = p;
  return  q;
}

char *
parse_float_decimal (const char **pp, const char *endptr)
{
  char  *q = NULL;
  const char *p = *pp;
  int    s = 0, n = 0;

  if (p >= endptr)
    return NULL;

  if (p[0] == '+' || p[0] == '-')
    p++;

  /* 1. .01 001 001E-001 */
  for (s = 0, n = 0; p < endptr && s >= 0; ) {
    switch (p[0]) {
    case '+': case '-':
      if (s != 2)
        s = -1;
      else {
        s = 3; p++;
      }
      break;
    case '.':
      if (s > 0)
        s = -1;
      else {
        s =  1; p++;
      }
      break;
    case '0': case '1': case '2': case '3': case '4':
    case '5': case '6': case '7': case '8': case '9':
      n++; p++;
      break;
    case 'E': case 'e':
      if (n == 0 || s == 2)
        s = -1;
      else {
        s = 2; p++;
      }
      break;
    default:
      s = -1;
      break;
    }
  }

  if (n != 0) {
    n = (int) (p - *pp);
    q = NEW(n + 1, char);
    memcpy(q, *pp, n); q[n] = '\0';
  }

  *pp = p;
  return  q;
}
