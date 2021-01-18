/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _DPXUTIL_H_
#define _DPXUTIL_H_

#include "tectonic_bridge_core.h"

#include <time.h>

#undef  MIN
#define MIN(a, b) (((a) < (b)) ? (a) : (b))
#undef  MAX
#define MAX(a, b) (((a) > (b)) ? (a) : (b))
#undef  ABS
#define ABS(a)    (((a) < 0) ? -(a) : (a))

extern double min4(double v1, double v2, double v3, double v4);
extern double max4(double v1, double v2, double v3, double v4);

#define INVALID_EPOCH_VALUE ((time_t)-1)
extern time_t dpx_util_get_unique_time_if_given (void);
extern int    dpx_util_format_asn_date (char *date_string, int need_timezone);

#ifndef is_space
#define is_space(c) ((c) == ' '  || (c) == '\t' || (c) == '\f' || \
                     (c) == '\r' || (c) == '\n' || (c) == '\0')
#endif
#ifndef is_delim
#define is_delim(c) ((c) == '(' || (c) == ')' || \
                     (c) == '/' || \
                     (c) == '<' || (c) == '>' || \
                     (c) == '[' || (c) == ']' || \
                     (c) == '{' || (c) == '}' || \
                     (c) == '%')
#endif

void skip_white_spaces (unsigned char **s, unsigned char *endptr);
int  xtoi     (char c);

#define HASH_TABLE_SIZE 503

struct ht_entry {
  char  *key;
  int    keylen;

  void  *value;

  struct ht_entry *next;
};

typedef void (*hval_free_func) (void *);

struct ht_table {
  int count;
  hval_free_func hval_free_fn;
  struct ht_entry *table[HASH_TABLE_SIZE];
};

void  ht_init_table   (struct ht_table *ht,
                              hval_free_func hval_free_fn);
void  ht_clear_table  (struct ht_table *ht);
int   ht_table_size   (struct ht_table *ht);
void *ht_lookup_table (struct ht_table *ht,
                              const void *key, int keylen);
void  ht_append_table (struct ht_table *ht,
                              const void *key, int keylen, void *value) ;
int   ht_remove_table (struct ht_table *ht,
                              const void *key, int keylen);
void  ht_insert_table (struct ht_table *ht,
                              const void *key, int keylen, void *value);

struct ht_iter {
  int    index;
  void  *curr;
  struct ht_table *hash;
};

int   ht_set_iter    (struct ht_table *ht, struct ht_iter *iter);
void  ht_clear_iter  (struct ht_iter *iter);
char *ht_iter_getkey (struct ht_iter *iter, int *keylen);
void *ht_iter_getval (struct ht_iter *iter);
int   ht_iter_next   (struct ht_iter *iter);

char *parse_float_decimal (const char **pp, const char *endptr);
char *parse_c_string      (const char **pp, const char *endptr);
char *parse_c_ident       (const char **pp, const char *endptr);

#endif /* _DPXUTIL_H_ */
