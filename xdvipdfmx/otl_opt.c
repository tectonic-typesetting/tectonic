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

#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "system.h"
#include "error.h"
#include "mem.h"
#include "mfileio.h"

#include "otl_opt.h"

struct bt_node {
  int    flag;

  struct bt_node *left;
  struct bt_node *right;

  char data[4];
};

#define FLAG_NOT (1 << 0)
#define FLAG_AND (1 << 1)

static int match_expr (struct bt_node *expr, const char *key);
static int
match_expr (struct bt_node *expr, const char *key)
{
  int retval = 1;
  int i;

  if (expr) {
    if (!expr->left && !expr->right) {
      for (i = 0; i < 4; i++) {
	if (expr->data[i] != '?' &&
	    expr->data[i] != key[i]) {
	  retval = 0;
	  break;
	}
      }
    } else {
      if (expr->left) {
	retval  = match_expr(expr->left, key);
      }
      if (expr->right) {
	if (retval && (expr->flag & FLAG_AND)) /* and */
	  retval &= match_expr(expr->right, key);
	else if (!retval && !(expr->flag & FLAG_AND)) /* or */
	  retval  = match_expr(expr->right, key);
      }
    }
    if (expr->flag & FLAG_NOT) /* not */
      retval = retval ? 0 : 1;

  }

  return retval;
}

static struct bt_node *
bt_new_tree (void)
{
  struct bt_node *expr;

  expr = NEW(1, struct bt_node);
  expr->flag  = 0;
  expr->left  = NULL;
  expr->right = NULL;
  memset(expr->data, 0, 4);

  return expr;
}

static void bt_release_tree (struct bt_node *tree);

static void
bt_release_tree (struct bt_node *tree)
{
  if (tree) {
    if (tree->left)
      bt_release_tree(tree->left);
    if (tree->right)
      bt_release_tree(tree->right);
    RELEASE(tree);
  }
}

static struct bt_node *
parse_expr (const char **pp, const char *endptr)
{
  struct bt_node *root, *curr;
  
  if (*pp >= endptr)
    return NULL;

  root = curr = bt_new_tree();
  while (*pp < endptr) {
    switch (**pp) {
    case '!':
      if (curr->flag & 2)
        curr->flag &= ~FLAG_NOT;
      else
        curr->flag |=  FLAG_NOT;
      (*pp)++;
      break;
    case '(':
      (*pp)++;
      if (*pp < endptr) {
        struct bt_node *expr;

        expr = parse_expr(pp, endptr);
        if (!expr) {
            WARN("Syntax error: %s\n", *pp);
            return NULL;
        }
        if (**pp != ')') {
            WARN("Syntax error: Unbalanced ()\n");
            return NULL;
           }
        curr->left  = expr->left;
        curr->right = expr->right;
        memcpy(curr->data, expr->data, 4);

        RELEASE(expr);
      } else {
        WARN("Syntax error: Unbalanced ()\n");
        bt_release_tree(root);
        return NULL;
      }
      (*pp)++;
      break;
    case ')':
      return root;
    case '|': case '&':
      if (*pp >= endptr) {
        WARN("Syntax error: %s\n", *pp);
        bt_release_tree(root);
        return NULL;
      } else {
        struct bt_node *tmp;

        tmp        = bt_new_tree();
        tmp->left  = root;
        tmp->right = curr = bt_new_tree();
        if (**pp == '&')
          tmp->flag = 1;
        else
          tmp->flag = 0;
        root = tmp;
      }
      (*pp)++;
      break;
    case '*':
      memset(curr->data, '?', 4);
      (*pp)++;
      break;
    default:
      if (*pp + 4 <= endptr) {
        int i;

        for (i = 0; i < 4; i++) {
            if (**pp == ' '   || **pp == '?' ||
                isalpha((unsigned char)**pp) || isdigit((unsigned char)**pp))
                curr->data[i] = **pp;
            else if (**pp == '_')
                curr->data[i] = ' ';
            else {
                WARN("Invalid char in tag: %c\n", **pp);
                bt_release_tree(root);
                return NULL;
            }
            (*pp)++;
        }
      } else {
        WARN("Syntax error: %s\n", *pp);
        bt_release_tree(root);
        return NULL;
      }
      break;
    }
  }

  return root;
}


struct otl_opt
{
  struct bt_node *rule;
};

otl_opt *
otl_new_opt (void)
{
  struct otl_opt *opt;

  opt = NEW(1, struct otl_opt);
  opt->rule = NULL;

  return (otl_opt *) opt;
}


void
otl_release_opt (otl_opt *opt)
{
  if (opt->rule) {
    bt_release_tree(opt->rule);
  }
  opt->rule = NULL;
  RELEASE(opt);
}

#if 0
struct lv_range
{
  int start, end;
};

struct uc_coverage
{
  int    count;
  struct lv_range *ranges;
};

static inline int
range_cmp (const void *v1, const void *v2)
{
  struct lv_range *sv1, *sv2;

  sv1 = (struct lv_range *) v1;
  sv2 = (struct lv_range *) v2;

  if (sv1->start < sv2->start)
    return -1;
  else
    return  1;

  return 0;
}

static inline int
range_overlap (const void *v1, const void *v2)
{
  struct lv_range *sv1, *sv2;

  sv1 = (struct lv_range *) v1;
  sv2 = (struct lv_range *) v2;

  /* Must be first sort in increasing start order */
  if (sv1->end  >= sv2->start)
    return 0;
  else if (sv1->end < sv2->start)
    return -1;

  return 1;
}

static void
check_uc_coverage (struct uc_coverage *coverage)
{
  struct lv_range *r1, *r2;
  int i;

  for (i = 0; i < coverage->count; i++) {
    r1 = &coverage->ranges[i];
    r2 = bsearch(r1, coverage->ranges,
                 coverage->count, sizeof(struct lv_range),
                 range_overlap);
    if (r2 && r1 != r2) {
      WARN("Overlapping Unicode range found:");
      WARN("[%x-%x], [%x-%x] ==> [%x-%x]",
           r1->start, r1->end, r2->start, r2->end,
	   MIN(r1->start, r2->start), MAX(r1->end, r2->end));
      r2->start = MIN(r1->start, r2->start);
      r2->end   = MAX(r1->end  , r2->end  );
      if (i < coverage->count - 1) {
        memmove(&coverage->ranges[i], &coverage->ranges[i+1],
                (coverage->count - i - 1) * sizeof(struct lv_range));
        coverage->count -= 1;
      }
    }
  }
  /* ... */
  if (coverage->count == 0) {
    RELEASE(coverage->ranges);
    coverage->ranges = NULL;
  }
}
#endif

int
otl_parse_optstring (otl_opt *opt, const char *optstr)
{
  const char *p, *endptr;

  ASSERT(opt);

  if (optstr) {
    p      = optstr;
    endptr = p + strlen(optstr);
    opt->rule = parse_expr(&p, endptr);
  }

  return 0;
}

int
otl_match_optrule (otl_opt *opt, const char *tag)
{
  ASSERT(tag);

  if (!opt || !opt->rule)
    return 1;

  return match_expr(opt->rule, tag);
}
