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

/*
 * Currently, this is nearly useless.
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "system.h"
#include "mem.h"
#include "error.h"

#include "dpxutil.h"

#include "pdfobj.h"

#include "pdfresource.h"

#define PDF_RESOURCE_DEBUG_STR "PDF"
#define PDF_RESOURCE_DEBUG     3

#define PDF_RESOURCE_FONT       0
#define PDF_RESOURCE_CIDFONT    1
#define PDF_RESOURCE_ENCODING   2
#define PDF_RESOURCE_CMAP       3
#define PDF_RESOURCE_XOBJECT    4
#define PDF_RESOURCE_COLORSPACE 5
#define PDF_RESOURCE_SHADING    6
#define PDF_RESOURCE_PATTERN    7
#define PDF_RESOURCE_GSTATE     8

typedef struct pdf_res
{
  char    *ident;

  int      flags;

  int      category;
  void    *cdata;

  pdf_obj *object;
  pdf_obj *reference;
} pdf_res;

static struct {
  const char *name;
  int         cat_id;
} pdf_resource_categories[] = {
  {"Font",       PDF_RESOURCE_FONT},
  {"CIDFont",    PDF_RESOURCE_CIDFONT},
  {"Encoding",   PDF_RESOURCE_ENCODING},
  {"CMap",       PDF_RESOURCE_CMAP},
  {"XObject",    PDF_RESOURCE_XOBJECT},
  {"ColorSpace", PDF_RESOURCE_COLORSPACE},
  {"Shading",    PDF_RESOURCE_SHADING},
  {"Pattern",    PDF_RESOURCE_PATTERN},
  {"ExtGState",  PDF_RESOURCE_GSTATE},
};

#define PDF_NUM_RESOURCE_CATEGORIES (sizeof(pdf_resource_categories)/sizeof(pdf_resource_categories[0]))

#define CACHE_ALLOC_SIZE 16u
struct res_cache
{
  int      count;
  int      capacity;
  pdf_res *resources;
};

static struct res_cache resources[PDF_NUM_RESOURCE_CATEGORIES];

static void
pdf_init_resource (pdf_res *res)
{
  ASSERT(res);

  res->ident     = NULL;
  res->category  = -1;
  res->flags     = 0;
  res->cdata     = NULL;
  res->object    = NULL;
  res->reference = NULL;

  return;
}

static void
pdf_flush_resource (pdf_res *res)
{
  if (res) {
    if (res->reference)
      pdf_release_obj(res->reference);
    if (res->object)
      pdf_release_obj(res->object);

    res->reference = NULL;
    res->object    = NULL;
  }
}

static void
pdf_clean_resource (pdf_res *res)
{
  if (res) {
    if (res->reference || res->object)
      WARN("Trying to release un-flushed object.");
    if (res->reference)
      pdf_release_obj(res->reference);
    if (res->object)
      pdf_release_obj(res->object);
    if (res->ident)
      RELEASE(res->ident);
    res->ident    = NULL;
    res->category = -1;
    res->flags    = 0;
  }
}

void
pdf_init_resources (void)
{
  int  i;

  for (i = 0;
       i < PDF_NUM_RESOURCE_CATEGORIES; i++) {
    resources[i].count     = 0;
    resources[i].capacity  = 0;
    resources[i].resources = NULL;
  }
}

void
pdf_close_resources (void)
{
  int  i;

  for (i = 0;
       i < PDF_NUM_RESOURCE_CATEGORIES; i++) {
    struct res_cache *rc;
    int    j;

    rc = &resources[i];
    for (j = 0; j < rc->count; j++) {
      pdf_flush_resource(&rc->resources[j]);
      pdf_clean_resource(&rc->resources[j]);
    }
    RELEASE(rc->resources);

    rc->count     = 0;
    rc->capacity  = 0;
    rc->resources = NULL;
  }
}

static int
get_category (const char *category)
{
  int  i;

  for (i = 0;
       i < PDF_NUM_RESOURCE_CATEGORIES; i++) {
    if (!strcmp(category, pdf_resource_categories[i].name)) {
      return pdf_resource_categories[i].cat_id;
    }
  }

  return -1;
}

int
pdf_defineresource (const char *category,
		    const char *resname, pdf_obj *object, int flags)
{
  int      res_id;
  struct res_cache *rc;
  int      cat_id;
  pdf_res *res = NULL;

  ASSERT(category && object);

  cat_id = get_category(category);
  if (cat_id < 0) {
    ERROR("Unknown resource category: %s", category);
    return -1;
  }

  rc = &resources[cat_id];
  if (resname) {
    for (res_id = 0; res_id < rc->count; res_id++) {
      res = &rc->resources[res_id];
      if (!strcmp(resname, res->ident)) {
	WARN("Resource %s (category: %s) already defined...",
	     resname, category);
	pdf_flush_resource(res);
	res->flags    = flags;
	if (flags & PDF_RES_FLUSH_IMMEDIATE) {
	  res->reference = pdf_ref_obj(object);
	  pdf_release_obj(object);
	} else {
	  res->object = object;
	}
	return (cat_id << 16) | res_id;
      }
    }
  } else {
    res_id = rc->count;
  }

  if (res_id == rc->count) {
    if (rc->count >= rc->capacity) {
      rc->capacity += CACHE_ALLOC_SIZE;
      rc->resources = RENEW(rc->resources, rc->capacity, pdf_res);
    }
    res = &rc->resources[res_id];

    pdf_init_resource(res);
    if (resname && resname[0] != '\0') {
      res->ident = NEW(strlen(resname) + 1, char);
      strcpy(res->ident, resname);
    }
    res->category = cat_id;
    res->flags    = flags;
    if (flags & PDF_RES_FLUSH_IMMEDIATE) {
      res->reference = pdf_ref_obj(object);
      pdf_release_obj(object);
    } else {
      res->object = object;
    }
    rc->count++;
  }

  return (cat_id << 16) | res_id;
}

#if 0
int
pdf_resource_exist (const char *category, const char *resname)
{
  int    res_id;
  struct res_cache *rc;
  int    cat_id;

  ASSERT(resname && category);

  cat_id = get_category(category);
  if (cat_id < 0)
    ERROR("Unknown resource category: %s", category);

  rc = &resources[cat_id];
  for (res_id = 0; res_id < rc->count; res_id++) {
    pdf_res *res;

    res = &rc->resources[res_id];
    if (!strcmp(resname, res->ident)) {
      return 1;
    }
  }

  return 0;
}
#endif

int
pdf_findresource (const char *category, const char *resname)
{
  pdf_res *res;
  int      res_id, cat_id;
  struct res_cache *rc;

  ASSERT(resname && category);

  cat_id = get_category(category);
  if (cat_id < 0) {
    ERROR("Unknown resource category: %s", category);
    return -1;
  }

  rc = &resources[cat_id];
  for (res_id = 0; res_id < rc->count; res_id++) {
    res = &rc->resources[res_id];
    if (!strcmp(resname, res->ident)) {
      return cat_id << 16 | res_id;
    }
  }

  return -1;
}

pdf_obj *
pdf_get_resource_reference (int rc_id)
{
  int  cat_id, res_id;
  struct res_cache *rc;
  pdf_res *res;

  cat_id = (rc_id >> 16) & 0xffff;
  res_id = rc_id & 0xffff;

  if (cat_id < 0 ||
      cat_id >= PDF_NUM_RESOURCE_CATEGORIES) {
    ERROR("Invalid category ID: %d", cat_id);
    return NULL;
  }
  rc  = &resources[cat_id];
  if (res_id < 0 || res_id >= rc->count) {
    ERROR("Invalid resource ID: %d", res_id);
    return NULL;
  }

  res = &rc->resources[res_id];
  if (!res->reference) {
    if (!res->object) {
      ERROR("Undefined object...");
      return NULL;
    } else {
      res->reference = pdf_ref_obj(res->object);
    }
  }

  return pdf_link_obj(res->reference);
}

#if 0
pdf_obj *
pdf_get_resource (int rc_id)
{
  int  cat_id, res_id;
  struct res_cache *rc;
  pdf_res *res;

  cat_id = (rc_id >> 16) & 0xffff;
  res_id = rc_id & 0xffff;

  if (cat_id < 0 ||
      cat_id >= PDF_NUM_RESOURCE_CATEGORIES) {
    ERROR("Invalid category ID: %d", cat_id);
    return NULL;
  }
  rc  = &resources[cat_id];
  if (res_id < 0 || res_id >= rc->count) {
    ERROR("Invalid resource ID: %d", res_id);
    return NULL;
  }

  res = &rc->resources[res_id];
  if (!res->object) {
    ERROR("Object already flushed???");
    return NULL;
  }

  return res->object;
}
#endif
