/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2008-2019 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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
 * TODO: Many things...
 *  {begin,end}_{bead,article}, box stack, name tree (not limited to dests)...
 */

#include "dpx-pdfdoc.h"

#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-jpegimage.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfencrypt.h"
#include "dpx-pdffont.h"
#include "dpx-pdfnames.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfresource.h"
#include "dpx-pdfximage.h"
#include "dpx-pngimage.h"
#include "dpx-system.h"

#define PDFDOC_PAGES_ALLOC_SIZE   128u
#define PDFDOC_ARTICLE_ALLOC_SIZE 16
#define PDFDOC_BEAD_ALLOC_SIZE    16

static char  manual_thumb_enabled  = 0;
static char *thumb_basename = NULL;

void
pdf_doc_enable_manual_thumbnails (void)
{
  manual_thumb_enabled = 1;
  // without HAVE_LIBPNG:
  // dpx_warning("Manual thumbnail is not supported without the libpng library.");
}

static pdf_obj *
read_thumbnail (const char *thumb_filename)
{
  pdf_obj *image_ref;
  int      xobj_id;
  rust_input_handle_t handle = NULL;
  load_options options = {1, 0, NULL};

  handle = ttstub_input_open(thumb_filename, TTBC_FILE_FORMAT_PICT, 0);
  if (handle == NULL) {
    dpx_warning("Could not open thumbnail file \"%s\"", thumb_filename);
    return NULL;
  }

  if (!check_for_png(handle) && !check_for_jpeg(handle)) {
    dpx_warning("Thumbnail \"%s\" not a png/jpeg file!", thumb_filename);
    ttstub_input_close(handle);
    return NULL;
  }
  ttstub_input_close(handle);

  xobj_id = pdf_ximage_findresource(thumb_filename, options);
  if (xobj_id < 0) {
    dpx_warning("Could not read thumbnail file \"%s\".", thumb_filename);
    image_ref = NULL;
  } else {
    image_ref = pdf_ximage_get_reference(xobj_id);
  }

  return image_ref;
}

typedef struct pdf_form
{
  char       *ident;

  pdf_tmatrix matrix;
  pdf_rect    cropbox;

  pdf_obj    *resources;
  pdf_obj    *contents;
} pdf_form;

struct form_list_node
{
  int      q_depth;
  pdf_form form;

  struct form_list_node *prev;
};

#define USE_MY_MEDIABOX (1 << 0)
typedef struct pdf_page
{
  pdf_obj  *page_obj;
  pdf_obj  *page_ref;

  int       flags;

  double    ref_x, ref_y;
  pdf_rect  cropbox;

  pdf_obj  *resources;

  /* Contents */
  pdf_obj  *background;
  pdf_obj  *contents;

  /* global bop, background, contents, global eop */
  pdf_obj  *content_refs[4];

  pdf_obj  *annots;
  pdf_obj  *beads;
} pdf_page;

typedef struct pdf_olitem
{
  pdf_obj *dict;

  int      is_open;

  struct pdf_olitem *first;
  struct pdf_olitem *parent;

  struct pdf_olitem *next;
} pdf_olitem;

typedef struct pdf_bead
{
  char    *id;
  int      page_no;
  pdf_rect rect;
} pdf_bead;

typedef struct pdf_article
{
  char     *id;
  pdf_obj  *info;
  unsigned int num_beads;
  unsigned int max_beads;
  pdf_bead *beads;
} pdf_article;

struct name_dict
{
  const char  *category;
  struct ht_table *data;
};


typedef struct pdf_doc
{
  struct {
    pdf_obj *dict;

    pdf_obj *viewerpref;
    pdf_obj *pagelabels;
    pdf_obj *pages;
    pdf_obj *names;
    pdf_obj *threads;
  } root;

  pdf_obj *info;

  struct {
    pdf_rect mediabox;
    pdf_obj *bop, *eop;

    unsigned int num_entries; /* This is not actually total number of pages. */
    unsigned int max_entries;
    pdf_page *entries;
  } pages;

  struct {
    pdf_olitem *first;
    pdf_olitem *current;
    int         current_depth;
  } outlines;

  struct {
    unsigned int num_entries;
    unsigned int max_entries;
    pdf_article *entries;
  } articles;

  struct name_dict *names;

  int check_gotos;
  struct ht_table gotos;

  struct {
    int    outline_open_depth;
    double annot_grow;
  } opt;

  struct form_list_node *pending_forms;

} pdf_doc;
static pdf_doc pdoc;

static void
pdf_doc_init_catalog (pdf_doc *p)
{
  p->root.viewerpref = NULL;
  p->root.pagelabels = NULL;
  p->root.pages      = NULL;
  p->root.names      = NULL;
  p->root.threads    = NULL;

  p->root.dict = pdf_new_dict();
  pdf_set_root(p->root.dict);

  return;
}

static void
pdf_doc_close_catalog (pdf_doc *p)
{
  pdf_obj *tmp;

  if (p->root.viewerpref) {
    tmp = pdf_lookup_dict(p->root.dict, "ViewerPreferences");
    if (!tmp) {
      pdf_add_dict(p->root.dict,
                   pdf_new_name("ViewerPreferences"),
                   pdf_ref_obj (p->root.viewerpref));
    } else if (PDF_OBJ_DICTTYPE(tmp)) {
      pdf_merge_dict(p->root.viewerpref, tmp);
      pdf_add_dict(p->root.dict,
                   pdf_new_name("ViewerPreferences"),
                   pdf_ref_obj (p->root.viewerpref));
    } else { /* Maybe reference */
      /* What should I do? */
      dpx_warning("Could not modify ViewerPreferences.");
    }
    pdf_release_obj(p->root.viewerpref);
    p->root.viewerpref = NULL;
  }

  if (p->root.pagelabels) {
    tmp = pdf_lookup_dict(p->root.dict, "PageLabels");
    if (!tmp) {
      tmp = pdf_new_dict();
      pdf_add_dict(tmp, pdf_new_name("Nums"),  pdf_link_obj(p->root.pagelabels));
      pdf_add_dict(p->root.dict,
                   pdf_new_name("PageLabels"), pdf_ref_obj(tmp));
      pdf_release_obj(tmp);
    } else { /* Maybe reference */
      /* What should I do? */
      dpx_warning("Could not modify PageLabels.");
    }
    pdf_release_obj(p->root.pagelabels);
    p->root.pagelabels = NULL;
  }

  pdf_add_dict(p->root.dict,
               pdf_new_name("Type"), pdf_new_name("Catalog"));
  pdf_release_obj(p->root.dict);
  p->root.dict = NULL;

  return;
}

/*
 * Pages are starting at 1.
 * The page count does not increase until the page is finished.
 */
#define LASTPAGE(p)  (&(p->pages.entries[p->pages.num_entries]))
#define FIRSTPAGE(p) (&(p->pages.entries[0]))
#define PAGECOUNT(p) (p->pages.num_entries)
#define MAXPAGES(p)  (p->pages.max_entries)

static void
doc_resize_page_entries (pdf_doc *p, unsigned int size)
{
  if (size > MAXPAGES(p)) {
    unsigned int i;

    p->pages.entries = RENEW(p->pages.entries, size, struct pdf_page);

    for (i = p->pages.max_entries; i < size; i++) {
      p->pages.entries[i].page_obj   = NULL;
      p->pages.entries[i].page_ref   = NULL;
      p->pages.entries[i].flags      = 0;
      p->pages.entries[i].resources  = NULL;
      p->pages.entries[i].background = NULL;
      p->pages.entries[i].contents   = NULL;
      p->pages.entries[i].content_refs[0] = NULL; /* global bop */
      p->pages.entries[i].content_refs[1] = NULL; /* background */
      p->pages.entries[i].content_refs[2] = NULL; /* page body  */
      p->pages.entries[i].content_refs[3] = NULL; /* global eop */
      p->pages.entries[i].annots    = NULL;
      p->pages.entries[i].beads     = NULL;
    }
    p->pages.max_entries = size;
  }

  return;
}

static pdf_page *
doc_get_page_entry (pdf_doc *p, unsigned int page_no)
{
  pdf_page *page;

  if (page_no > 65535ul) {
    _tt_abort("Page number %ul too large!", page_no);
  } else if (page_no == 0) {
    _tt_abort("Invalid Page number %ul.", page_no);
  }

  if (page_no > MAXPAGES(p)) {
    doc_resize_page_entries(p, page_no + PDFDOC_PAGES_ALLOC_SIZE);
  }

  page = &(p->pages.entries[page_no - 1]);

  return page;
}

static void pdf_doc_init_page_tree  (pdf_doc *p, double media_width, double media_height);
static void pdf_doc_close_page_tree (pdf_doc *p);

static void pdf_doc_init_names  (pdf_doc *p, int check_gotos);
static void pdf_doc_close_names (pdf_doc *p);

static void pdf_doc_add_goto (pdf_obj *annot_dict);

static void pdf_doc_init_docinfo  (pdf_doc *p);
static void pdf_doc_close_docinfo (pdf_doc *p);

static void pdf_doc_init_articles    (pdf_doc *p);
static void pdf_doc_close_articles   (pdf_doc *p);
static void pdf_doc_init_bookmarks   (pdf_doc *p, int bm_open_depth);
static void pdf_doc_close_bookmarks  (pdf_doc *p);

void
pdf_doc_set_bop_content (const char *content, unsigned int length)
{
  pdf_doc *p = &pdoc;

  assert(p);

  if (p->pages.bop) {
    pdf_release_obj(p->pages.bop);
    p->pages.bop = NULL;
  }

  if (length > 0) {
    p->pages.bop = pdf_new_stream(STREAM_COMPRESS);
    pdf_add_stream(p->pages.bop, content, length);
  } else {
    p->pages.bop = NULL;
  }

  return;
}

void
pdf_doc_set_eop_content (const char *content, unsigned int length)
{
  pdf_doc *p = &pdoc;

  if (p->pages.eop) {
    pdf_release_obj(p->pages.eop);
    p->pages.eop = NULL;
  }

  if (length > 0) {
    p->pages.eop = pdf_new_stream(STREAM_COMPRESS);
    pdf_add_stream(p->pages.eop, content, length);
  } else {
    p->pages.eop = NULL;
  }

  return;
}

static void
pdf_doc_init_docinfo (pdf_doc *p)
{
  p->info = pdf_new_dict();
  pdf_set_info(p->info);

  return;
}

static void
pdf_doc_close_docinfo (pdf_doc *p)
{
  pdf_obj *docinfo = p->info;

  /*
   * Excerpt from PDF Reference 4th ed., sec. 10.2.1.
   *
   * Any entry whose value is not known should be omitted from the dictionary,
   * rather than included with an empty string as its value.
   *
   * ....
   *
   * Note: Although viewer applications can store custom metadata in the document
   * information dictionary, it is inappropriate to store private content or
   * structural information there; such information should be stored in the
   * document catalog instead (see Section 3.6.1,  Document Catalog ).
   */
  const char *keys[] = {
    "Title", "Author", "Subject", "Keywords", "Creator", "Producer",
    "CreationDate", "ModDate", /* Date */
    NULL
  };
  pdf_obj *value;
  unsigned int i;

  for (i = 0; keys[i] != NULL; i++) {
    value = pdf_lookup_dict(docinfo, keys[i]);
    if (value) {
      if (!PDF_OBJ_STRINGTYPE(value)) {
        dpx_warning("\"%s\" in DocInfo dictionary not string type.", keys[i]);
        pdf_remove_dict(docinfo, keys[i]);
        dpx_warning("\"%s\" removed from DocInfo.", keys[i]);
      } else if (pdf_string_length(value) == 0) {
        /* The hyperref package often uses emtpy strings. */
        pdf_remove_dict(docinfo, keys[i]);
      }
    }
  }

  if (!pdf_lookup_dict(docinfo, "Producer")) {
    char banner[] = DVIPDFMX_PROG_NAME " (" DPX_VERSION ")";

    pdf_add_dict(docinfo,
                 pdf_new_name("Producer"),
                 pdf_new_string(banner, strlen(banner)));
  }

  if (!pdf_lookup_dict(docinfo, "CreationDate") && source_date_epoch) {
    char now[80];

    dpx_util_format_asn_date(now, 1);
    pdf_add_dict(docinfo,
                 pdf_new_name ("CreationDate"),
                 pdf_new_string(now, strlen(now)));
  }

  pdf_release_obj(docinfo);
  p->info = NULL;

  return;
}

static pdf_obj *
pdf_doc_get_page_resources (pdf_doc *p, const char *category)
{
  pdf_obj  *resources;
  pdf_page *currentpage;
  pdf_obj  *res_dict;

  if (!p || !category) {
    return NULL;
  }

  if (p->pending_forms) {
    if (p->pending_forms->form.resources) {
      res_dict = p->pending_forms->form.resources;
    } else {
      res_dict = p->pending_forms->form.resources = pdf_new_dict();
    }
  } else {
    currentpage = LASTPAGE(p);
    if (currentpage->resources) {
      res_dict = currentpage->resources;
    } else {
      res_dict = currentpage->resources = pdf_new_dict();
    }
  }
  resources = pdf_lookup_dict(res_dict, category);
  if (!resources) {
    resources = pdf_new_dict();
    pdf_add_dict(res_dict, pdf_new_name(category), resources);
  } else if (pdf_obj_typeof(resources) == PDF_INDIRECT) {
    resources = pdf_deref_obj(resources); /* FIXME: deref_obj increment link count */
    pdf_release_obj(resources); /* FIXME: just to decrement link count */
  }

  return resources;
}

void
pdf_doc_add_page_resource (const char *category,
                           const char *resource_name, pdf_obj *resource_ref)
{
  pdf_doc *p = &pdoc;
  pdf_obj *resources;
  pdf_obj *duplicate;

  resources = pdf_doc_get_page_resources(p, category);
  duplicate = pdf_lookup_dict(resources, resource_name);
  if (duplicate && pdf_compare_reference(duplicate, resource_ref)) {
    dpx_warning("Conflicting page resource found (page: %d, category: %s, name: %s).",
         pdf_doc_current_page_number(), category, resource_name);
    dpx_warning("Ignoring...");
    pdf_release_obj(resource_ref);
  } else {
    pdf_add_dict(resources, pdf_new_name(resource_name), resource_ref);
  }

  return;
}

static void
doc_flush_page (pdf_doc *p, pdf_page *page, pdf_obj *parent_ref)
{
  pdf_obj *contents_array;
  unsigned int count;

  pdf_add_dict(page->page_obj,
               pdf_new_name("Type"), pdf_new_name("Page"));
  pdf_add_dict(page->page_obj,
               pdf_new_name("Parent"), parent_ref);

  /*
   * Clipping area specified by CropBox is affected by MediaBox which
   * might be inherit from parent node. If MediaBox of the root node
   * does not have enough size to cover all page's imaging area, using
   * CropBox here gives incorrect result.
   */
  if (page->flags & USE_MY_MEDIABOX) {
    pdf_obj *mediabox;

    mediabox = pdf_new_array();
    pdf_add_array(mediabox,
                  pdf_new_number(ROUND(page->cropbox.llx, 0.01)));
    pdf_add_array(mediabox,
                  pdf_new_number(ROUND(page->cropbox.lly, 0.01)));
    pdf_add_array(mediabox,
                  pdf_new_number(ROUND(page->cropbox.urx, 0.01)));
    pdf_add_array(mediabox,
                  pdf_new_number(ROUND(page->cropbox.ury, 0.01)));
    pdf_add_dict(page->page_obj, pdf_new_name("MediaBox"),  mediabox);
  }

  count = 0;
  contents_array = pdf_new_array();
  if (page->content_refs[0]) { /* global bop */
    pdf_add_array(contents_array, page->content_refs[0]);
    count++;
  } else if (p->pages.bop &&
             pdf_stream_length(p->pages.bop) > 0) {
    pdf_add_array(contents_array, pdf_ref_obj(p->pages.bop));
    count++;
  }
  if (page->content_refs[1]) { /* background */
    pdf_add_array(contents_array, page->content_refs[1]);
    count++;
  }
  if (page->content_refs[2]) { /* page body */
    pdf_add_array(contents_array, page->content_refs[2]);
    count++;
  }
  if (page->content_refs[3]) { /* global eop */
    pdf_add_array(contents_array, page->content_refs[3]);
    count++;
  } else if (p->pages.eop &&
             pdf_stream_length(p->pages.eop) > 0) {
    pdf_add_array(contents_array, pdf_ref_obj(p->pages.eop));
    count++;
  }

  if (count == 0) {
    dpx_warning("Page with empty content found!!!");
  }
  page->content_refs[0] = NULL;
  page->content_refs[1] = NULL;
  page->content_refs[2] = NULL;
  page->content_refs[3] = NULL;

  pdf_add_dict(page->page_obj,
               pdf_new_name("Contents"), contents_array);


  if (page->annots) {
    pdf_add_dict(page->page_obj,
                 pdf_new_name("Annots"), pdf_ref_obj(page->annots));
    pdf_release_obj(page->annots);
  }
  if (page->beads) {
    pdf_add_dict(page->page_obj,
                 pdf_new_name("B"), pdf_ref_obj(page->beads));
    pdf_release_obj(page->beads);
  }
  pdf_release_obj(page->page_obj);
  pdf_release_obj(page->page_ref);

  page->page_obj = NULL;
  page->page_ref = NULL;
  page->annots   = NULL;
  page->beads    = NULL;

  return;
}

/* B-tree? */
#define PAGE_CLUSTER 4
static pdf_obj *
build_page_tree (pdf_doc  *p,
                 pdf_page *firstpage, int num_pages,
                 pdf_obj  *parent_ref)
{
  pdf_obj *self, *self_ref, *kids;
  int      i;

  self = pdf_new_dict();
  /*
   * This is a slight kludge which allow the subtree dictionary
   * generated by this routine to be merged with the real
   * page_tree dictionary, while keeping the indirect object
   * references right.
   */
  self_ref = parent_ref ? pdf_ref_obj(self) : pdf_ref_obj(p->root.pages);

  pdf_add_dict(self, pdf_new_name("Type"),  pdf_new_name("Pages"));
  pdf_add_dict(self, pdf_new_name("Count"), pdf_new_number((double) num_pages));

  if (parent_ref != NULL)
    pdf_add_dict(self, pdf_new_name("Parent"), parent_ref);

  kids = pdf_new_array();
  if (num_pages > 0 && num_pages <= PAGE_CLUSTER) {
    for (i = 0; i < num_pages; i++) {
      pdf_page *page;

      page = firstpage + i;
      if (!page->page_ref)
        page->page_ref = pdf_ref_obj(page->page_obj);
      pdf_add_array (kids, pdf_link_obj(page->page_ref));
      doc_flush_page(p, page, pdf_link_obj(self_ref));
    }
  } else if (num_pages > 0) {
    for (i = 0; i < PAGE_CLUSTER; i++) {
      int start, end;

      start = (i*num_pages)/PAGE_CLUSTER;
      end   = ((i+1)*num_pages)/PAGE_CLUSTER;
      if (end - start > 1) {
        pdf_obj *subtree;

        subtree = build_page_tree(p, firstpage + start, end - start,
                                  pdf_link_obj(self_ref));
        pdf_add_array(kids, pdf_ref_obj(subtree));
        pdf_release_obj(subtree);
      } else {
        pdf_page *page;

        page = firstpage + start;
        if (!page->page_ref)
          page->page_ref = pdf_ref_obj(page->page_obj);
        pdf_add_array (kids, pdf_link_obj(page->page_ref));
        doc_flush_page(p, page, pdf_link_obj(self_ref));
      }
    }
  }
  pdf_add_dict(self, pdf_new_name("Kids"), kids);
  pdf_release_obj(self_ref);

  return self;
}

static void
pdf_doc_init_page_tree (pdf_doc *p, double media_width, double media_height)
{
  /*
   * Create empty page tree.
   * The docroot.pages is kept open until the document is closed.
   * This allows the user to write to pages if he so choses.
   */
  p->root.pages = pdf_new_dict();

  p->pages.num_entries = 0;
  p->pages.max_entries = 0;
  p->pages.entries     = NULL;

  p->pages.bop = NULL;
  p->pages.eop = NULL;

  p->pages.mediabox.llx = 0.0;
  p->pages.mediabox.lly = 0.0;
  p->pages.mediabox.urx = media_width;
  p->pages.mediabox.ury = media_height;

  return;
}

static void
pdf_doc_close_page_tree (pdf_doc *p)
{
  pdf_obj *page_tree_root;
  pdf_obj *mediabox;
  unsigned int page_no;

  /*
   * Do consistency check on forward references to pages.
   */
  for (page_no = PAGECOUNT(p) + 1; page_no <= MAXPAGES(p); page_no++) {
    pdf_page  *page;

    page = doc_get_page_entry(p, page_no);
    if (page->page_obj) {
      dpx_warning("Nonexistent page #%u refered.", page_no);
      pdf_release_obj(page->page_ref);
      page->page_ref = NULL;
    }
    if (page->page_obj) {
      dpx_warning("Entry for a nonexistent page #%u created.", page_no);
      pdf_release_obj(page->page_obj);
      page->page_obj = NULL;
    }
    if (page->annots) {
      dpx_warning("Annotation attached to a nonexistent page #%u.", page_no);
      pdf_release_obj(page->annots);
      page->annots = NULL;
    }
    if (page->beads) {
      dpx_warning("Article beads attached to a nonexistent page #%u.", page_no);
      pdf_release_obj(page->beads);
      page->beads = NULL;
    }
    if (page->resources) {
      pdf_release_obj(page->resources);
      page->resources = NULL;
    }
  }

  /*
   * Connect page tree to root node.
   */
  page_tree_root = build_page_tree(p, FIRSTPAGE(p), PAGECOUNT(p), NULL);
  pdf_merge_dict (p->root.pages, page_tree_root);
  pdf_release_obj(page_tree_root);

  /* They must be after build_page_tree() */
  if (p->pages.bop) {
    pdf_add_stream (p->pages.bop, "\n", 1);
    pdf_release_obj(p->pages.bop);
    p->pages.bop = NULL;
  }
  if (p->pages.eop) {
    pdf_add_stream (p->pages.eop, "\n", 1);
    pdf_release_obj(p->pages.eop);
    p->pages.eop = NULL;
  }

  /* Create media box at root node and let the other pages inherit it. */
  mediabox = pdf_new_array();
  pdf_add_array(mediabox, pdf_new_number(ROUND(p->pages.mediabox.llx, 0.01)));
  pdf_add_array(mediabox, pdf_new_number(ROUND(p->pages.mediabox.lly, 0.01)));
  pdf_add_array(mediabox, pdf_new_number(ROUND(p->pages.mediabox.urx, 0.01)));
  pdf_add_array(mediabox, pdf_new_number(ROUND(p->pages.mediabox.ury, 0.01)));
  pdf_add_dict(p->root.pages, pdf_new_name("MediaBox"), mediabox);

  pdf_add_dict(p->root.dict,
               pdf_new_name("Pages"),
               pdf_ref_obj (p->root.pages));
  pdf_release_obj(p->root.pages);
  p->root.pages  = NULL;

  p->pages.entries = mfree(p->pages.entries);
  p->pages.num_entries = 0;
  p->pages.max_entries = 0;

  return;
}

int
pdf_doc_get_page_count (pdf_file *pf)
{
  int      count = 0;
  pdf_obj *page_tree = NULL;
  pdf_obj *catalog;

  catalog = pdf_file_get_catalog(pf);

  page_tree = pdf_deref_obj(pdf_lookup_dict(catalog, "Pages"));

  if (!PDF_OBJ_DICTTYPE(page_tree)) {
    return 0;
  }

  {
    pdf_obj *tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "Count"));
    if (!PDF_OBJ_NUMBERTYPE(tmp)) {
      pdf_release_obj(tmp);
      return 0;
    }
    count = pdf_number_value(tmp);
    pdf_release_obj(tmp);
  }

  return count;
}

static int
set_bounding_box (pdf_rect *bbox, enum pdf_page_boundary opt_bbox,
                  pdf_obj *media_box, pdf_obj *crop_box,
                  pdf_obj *art_box, pdf_obj *trim_box, pdf_obj *bleed_box)
{
  pdf_obj *box = NULL;

  if (!media_box) {
    dpx_warning("MediaBox not found in included PDF...");
    return -1;
  }
#define VALIDATE_BOX(o) if ((o)) {\
  if (!PDF_OBJ_ARRAYTYPE((o)) || pdf_array_length((o)) != 4) \
    return -1;\
}
  VALIDATE_BOX(media_box);
  VALIDATE_BOX(crop_box);
  VALIDATE_BOX(art_box);
  VALIDATE_BOX(trim_box);
  VALIDATE_BOX(bleed_box);

  if (opt_bbox == pdf_page_boundary__auto) {
    if (crop_box)
      box = pdf_link_obj(crop_box);
    else if (art_box)
      box = pdf_link_obj(art_box);
    else if (trim_box)
      box = pdf_link_obj(trim_box);
    else if (bleed_box)
      box = pdf_link_obj(bleed_box);
    else {
      box = pdf_link_obj(media_box);
    }
  } else {
    if (!crop_box) {
      crop_box = pdf_link_obj(media_box);
    }
    if (!art_box) {
      art_box = pdf_link_obj(crop_box);
    }
    if (!trim_box) {
      trim_box = pdf_link_obj(crop_box);
    }
    if (!bleed_box) {
      bleed_box = pdf_link_obj(crop_box);
    }
    /* At this point all boxes must be defined. */
    switch (opt_bbox) {
    case pdf_page_boundary_cropbox:
      box = pdf_link_obj(crop_box);
      break;
    case pdf_page_boundary_mediabox:
      box = pdf_link_obj(media_box);
      break;
    case pdf_page_boundary_artbox:
      box = pdf_link_obj(art_box);
      break;
    case pdf_page_boundary_trimbox:
      box = pdf_link_obj(trim_box);
      break;
    case pdf_page_boundary_bleedbox:
      box = pdf_link_obj(bleed_box);
      break;
    default:
      box = pdf_link_obj(crop_box);
      break;
    }
  }

  if (!box) {
    /* Impossible */
    dpx_warning("No appropriate page boudary box found???");
    return -1;
  } else {
    int i;

    for (i = 4; i--; ) {
      double x;
      pdf_obj *tmp = pdf_deref_obj(pdf_get_array(box, i));
      if (!PDF_OBJ_NUMBERTYPE(tmp)) {
        pdf_release_obj(tmp);
        pdf_release_obj(box);
        return -1;
      }
      x = pdf_number_value(tmp);
      switch (i) {
      case 0: bbox->llx = x; break;
      case 1: bbox->lly = x; break;
      case 2: bbox->urx = x; break;
      case 3: bbox->ury = x; break;
      }
      pdf_release_obj(tmp);
    }

    /* New scheme only for XDV files */
    if (dpx_conf.compat_mode == dpx_mode_xdv_mode ||
        opt_bbox != pdf_page_boundary__auto) {
      for (i = 4; i--; ) {
        double x;
        pdf_obj *tmp = pdf_deref_obj(pdf_get_array(media_box, i));
        if (!PDF_OBJ_NUMBERTYPE(tmp)) {
          pdf_release_obj(tmp);
          pdf_release_obj(box);
          return -1;
        }
        x = pdf_number_value(tmp);
        switch (i) {
        case 0: if (bbox->llx < x) bbox->llx = x; break;
        case 1: if (bbox->lly < x) bbox->lly = x; break;
        case 2: if (bbox->urx > x) bbox->urx = x; break;
        case 3: if (bbox->ury > x) bbox->ury = x; break;
        }
        pdf_release_obj(tmp);
      }
    }
  }
  pdf_release_obj(box);

  return 0;
}

static int
set_transform_matrix (pdf_tmatrix *matrix, pdf_rect *bbox, pdf_obj *rotate)
{
  double deg;
  int    rot;

  matrix->a = matrix->d = 1.0;
  matrix->b = matrix->c = 0.0;
  matrix->e = matrix->f = 0.0;
  /* Handle Rotate */
  if (rotate) {
    if (!PDF_OBJ_NUMBERTYPE(rotate)) {
      return -1;
    } else {
      deg = pdf_number_value(rotate);
      if (deg - (int)deg != 0.0) {
        dpx_warning("Invalid value specified for /Rotate: %f", deg);
        return -1;
      } else if (deg != 0.0) {
        rot = (int) deg;
        if (rot % 90 != 0.0) {
          dpx_warning("Invalid value specified for /Rotate: %f", deg);
        } else {
          rot = rot % 360;
          if (rot < 0) rot += 360;
          switch (rot) {
          case 90:
            matrix->a = matrix->d = 0;
            matrix->b = -1;
            matrix->c = 1;
            matrix->e = bbox->llx - bbox->lly;
            matrix->f = bbox->lly + bbox->urx;
            break;
          case 180:
            matrix->a = matrix->d = -1;
            matrix->b = matrix->c = 0;
            matrix->e = bbox->llx + bbox->urx;
            matrix->f = bbox->lly + bbox->ury;
            break;
          case 270:
            matrix->a = matrix->d = 0;
            matrix->b = 1;
            matrix->c = -1;
            matrix->e = bbox->llx + bbox->ury;
            matrix->f = bbox->lly - bbox->llx;
           break;
           default:
            dpx_warning("Invalid value specified for /Rotate: %f", deg);
            break;
          }
        }
      }
    }
  }
  return 0;
}

/*
 * From PDFReference15_v6.pdf (p.119 and p.834)
 *
 * MediaBox rectangle (Required; inheritable)
 *
 * The media box defines the boundaries of the physical medium on which the
 * page is to be printed. It may include any extended area surrounding the
 * finished page for bleed, printing marks, or other such purposes. It may
 * also include areas close to the edges of the medium that cannot be marked
 * because of physical limitations of the output device. Content falling
 * outside this boundary can safely be discarded without affecting the
 * meaning of the PDF file.
 *
 * CropBox rectangle (Optional; inheritable)
 *
 * The crop box defines the region to which the contents of the page are to be
 * clipped (cropped) when displayed or printed. Unlike the other boxes, the
 * crop box has no defined meaning in terms of physical page geometry or
 * intended use; it merely imposes clipping on the page contents. However,
 * in the absence of additional information (such as imposition instructions
 * specified in a JDF or PJTF job ticket), the crop box will determine how
 * the page's contents are to be positioned on the output medium. The default
 * value is the page's media box.
 *
 * BleedBox rectangle (Optional; PDF 1.3)
 *
 * The bleed box (PDF 1.3) defines the region to which the contents of the
 * page should be clipped when output in a production environment. This may
 * include any extra "bleed area" needed to accommodate the physical
 * limitations of cutting, folding, and trimming equipment. The actual printed
 * page may include printing marks that fall outside the bleed box.
 * The default value is the page's crop box.
 *
 * TrimBox rectangle (Optional; PDF 1.3)
 *
 * The trim box (PDF 1.3) defines the intended dimensions of the finished page
 * after trimming. It may be smaller than the media box, to allow for
 * production-related content such as printing instructions, cut marks, or
 * color bars. The default value is the page's crop box.
 *
 * ArtBox rectangle (Optional; PDF 1.3)
 *
 * The art box (PDF 1.3) defines the extent of the page's meaningful content
 * (including potential white space) as intended by the page's creator.
 * The default value is the page's crop box.
 *
 * Rotate integer (Optional; inheritable)
 *
 * The number of degrees by which the page should be rotated clockwise when
 * displayed or printed. The value must be a multiple of 90. Default value: 0.
 */

/* count_p removed: Please use different interface if you want to get total page
 * number. pdf_doc_get_page() is obviously not an interface to do such.
 */
pdf_obj *
pdf_doc_get_page (pdf_file *pf,
                  int page_no, enum pdf_page_boundary opt_bbox, /* load options */
                  pdf_rect *bbox, pdf_tmatrix *matrix,  /* returned value */
                  pdf_obj **resources_p /* returned values */
                  ) {
  pdf_obj *catalog = NULL, *page_tree = NULL;
  pdf_obj *resources = NULL, *rotate = NULL;
  pdf_obj *art_box = NULL, *trim_box = NULL, *bleed_box = NULL;
  pdf_obj *media_box = NULL, *crop_box = NULL;
  int      error = 0;

  catalog = pdf_file_get_catalog(pf);

  page_tree = pdf_deref_obj(pdf_lookup_dict(catalog, "Pages"));

  if (!PDF_OBJ_DICTTYPE(page_tree))
    goto error_exit;

  {
    int count;
    pdf_obj *tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "Count"));
    if (!PDF_OBJ_NUMBERTYPE(tmp)) {
      pdf_release_obj(tmp);
      goto error_exit;
    }
    count = pdf_number_value(tmp);
    pdf_release_obj(tmp);
    if (page_no <= 0 || page_no > count) {
      dpx_warning("Page %d does not exist.", page_no);
      goto error_silent;
    }
  }

  /*
   * Seek correct page. Get MediaBox, CropBox and Resources.
   * (Note that these entries can be inherited.)
   */
  {
    pdf_obj *kids, *tmp;
    int      depth = PDF_OBJ_MAX_DEPTH;
    int      page_idx = page_no - 1, kids_length = 1, i = 0;

    while (--depth && i != kids_length) {
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "MediaBox")))) {
        pdf_release_obj(media_box);
        media_box = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "CropBox")))) {
        pdf_release_obj(crop_box);
        crop_box = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "ArtBox")))) {
        pdf_release_obj(art_box);
        art_box = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "TrimBox")))) {
        pdf_release_obj(trim_box);
        trim_box = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "BleedBox")))) {
        pdf_release_obj(bleed_box);
        bleed_box = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "Rotate")))) {
        pdf_release_obj(rotate);
        rotate = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "Resources")))) {
        pdf_release_obj(resources);
        resources = tmp;
      }

      kids = pdf_deref_obj(pdf_lookup_dict(page_tree, "Kids"));
      if (!kids)
        break;
      else if (!PDF_OBJ_ARRAYTYPE(kids)) {
        pdf_release_obj(kids);
        goto error_exit;
      }
      kids_length = pdf_array_length(kids);

      for (i = 0; i < kids_length; i++) {
        int count;

        pdf_release_obj(page_tree);
        page_tree = pdf_deref_obj(pdf_get_array(kids, i));
        if (!PDF_OBJ_DICTTYPE(page_tree))
          goto error_exit;

        tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "Count"));
        if (PDF_OBJ_NUMBERTYPE(tmp)) {
          /* Pages object */
          count = pdf_number_value(tmp);
          pdf_release_obj(tmp);
        } else if (!tmp)
          /* Page object */
          count = 1;
        else {
          pdf_release_obj(tmp);
          goto error_exit;
        }
        if (page_idx < count)
          break;
        page_idx -= count;
      }
      pdf_release_obj(kids);
    }
    if (!depth || kids_length == i)
      goto error_exit;
  }

  if (!PDF_OBJ_DICTTYPE(resources))
    goto error_exit;
  if (resources_p)
    *resources_p = pdf_link_obj(resources);

  /* Select page boundary box */
  error = set_bounding_box(bbox, opt_bbox, media_box, crop_box, art_box, trim_box, bleed_box);
  if (error)
    goto error_exit;
  /* Set transformation matrix */
  error = set_transform_matrix(matrix, bbox, rotate);
  if (error)
    goto error_exit;

  goto clean_exit; /* Success */

 error_exit:
  dpx_warning("Error found in including PDF image.");
 error_silent:
  pdf_release_obj(page_tree);
  page_tree = NULL;

clean_exit:
  pdf_release_obj(crop_box);
  pdf_release_obj(bleed_box);
  pdf_release_obj(trim_box);
  pdf_release_obj(art_box);
  pdf_release_obj(media_box);
  pdf_release_obj(rotate);
  pdf_release_obj(resources);

  return page_tree;
}

#ifndef BOOKMARKS_OPEN_DEFAULT
#define BOOKMARKS_OPEN_DEFAULT 0
#endif

static int clean_bookmarks (pdf_olitem *item);
static int flush_bookmarks (pdf_olitem *item,
                            pdf_obj *parent_ref,
                            pdf_obj *parent_dict);

static void
pdf_doc_init_bookmarks (pdf_doc *p, int bm_open_depth)
{
  pdf_olitem *item;

#define MAX_OUTLINE_DEPTH 256u
  p->opt.outline_open_depth =
    ((bm_open_depth >= 0) ?
     bm_open_depth : MAX_OUTLINE_DEPTH - bm_open_depth);

  p->outlines.current_depth = 1;

  item = NEW(1, pdf_olitem);
  item->dict    = NULL;
  item->next    = NULL;
  item->first   = NULL;
  item->parent  = NULL;
  item->is_open = 1;

  p->outlines.current = item;
  p->outlines.first   = item;

  return;
}

static int
clean_bookmarks (pdf_olitem *item)
{
  pdf_olitem *next;

  while (item) {
    next = item->next;
    pdf_release_obj(item->dict);
    if (item->first)
      clean_bookmarks(item->first);
    free(item);

    item = next;
  }

  return 0;
}

static int
flush_bookmarks (pdf_olitem *node,
                 pdf_obj *parent_ref, pdf_obj *parent_dict)
{
  int         retval;
  int         count;
  pdf_olitem *item;
  pdf_obj    *this_ref, *prev_ref, *next_ref;

  assert(node->dict);

  this_ref = pdf_ref_obj(node->dict);
  pdf_add_dict(parent_dict,
               pdf_new_name("First"), pdf_link_obj(this_ref));

  retval = 0;
  for (item = node, prev_ref = NULL;
       item && item->dict; item = item->next) {
    if (item->first && item->first->dict) {
      count = flush_bookmarks(item->first, this_ref, item->dict);
      if (item->is_open) {
        pdf_add_dict(item->dict,
                     pdf_new_name("Count"),
                     pdf_new_number(count));
        retval += count;
      } else {
        pdf_add_dict(item->dict,
                     pdf_new_name("Count"),
                     pdf_new_number(-count));
      }
    }
    pdf_add_dict(item->dict,
                 pdf_new_name("Parent"),
                 pdf_link_obj(parent_ref));
    if (prev_ref) {
      pdf_add_dict(item->dict,
                   pdf_new_name("Prev"),
                   prev_ref);
    }
    if (item->next && item->next->dict) {
      next_ref = pdf_ref_obj(item->next->dict);
      pdf_add_dict(item->dict,
                   pdf_new_name("Next"),
                   pdf_link_obj(next_ref));
    } else {
      next_ref = NULL;
    }

    pdf_release_obj(item->dict);
    item->dict = NULL;

    prev_ref = this_ref;
    this_ref = next_ref;
    retval++;
  }

  pdf_add_dict(parent_dict,
               pdf_new_name("Last"),
               pdf_link_obj(prev_ref));

  pdf_release_obj(prev_ref);
  pdf_release_obj(node->dict);
  node->dict = NULL;

  return retval;
}

int
pdf_doc_bookmarks_up (void)
{
  pdf_doc    *p = &pdoc;
  pdf_olitem *parent, *item;

  item = p->outlines.current;
  if (!item || !item->parent) {
    dpx_warning("Can't go up above the bookmark root node!");
    return -1;
  }
  parent = item->parent;
  item   = parent->next;
  if (!parent->next) {
    parent->next  = item = NEW(1, pdf_olitem);
    item->dict    = NULL;
    item->first   = NULL;
    item->next    = NULL;
    item->is_open = 0;
    item->parent  = parent->parent;
  }
  p->outlines.current = item;
  p->outlines.current_depth--;

  return 0;
}

int
pdf_doc_bookmarks_down (void)
{
  pdf_doc    *p = &pdoc;
  pdf_olitem *item, *first;

  item = p->outlines.current;
  if (!item->dict) {
    pdf_obj *tcolor, *action;

    dpx_warning("Empty bookmark node!");
    dpx_warning("You have tried to jump more than 1 level.");

    item->dict = pdf_new_dict();

#define TITLE_STRING "<No Title>"
    pdf_add_dict(item->dict,
                 pdf_new_name("Title"),
                 pdf_new_string(TITLE_STRING, strlen(TITLE_STRING)));

    tcolor = pdf_new_array();
    pdf_add_array(tcolor, pdf_new_number(1.0));
    pdf_add_array(tcolor, pdf_new_number(0.0));
    pdf_add_array(tcolor, pdf_new_number(0.0));
    pdf_add_dict (item->dict,
                  pdf_new_name("C"), pdf_link_obj(tcolor));
    pdf_release_obj(tcolor);

    pdf_add_dict (item->dict,
                  pdf_new_name("F"), pdf_new_number(1.0));

#define JS_CODE "app.alert(\"The author of this document made this bookmark item empty!\", 3, 0)"
    action = pdf_new_dict();
    pdf_add_dict(action,
                 pdf_new_name("S"), pdf_new_name("JavaScript"));
    pdf_add_dict(action,
                 pdf_new_name("JS"), pdf_new_string(JS_CODE, strlen(JS_CODE)));
    pdf_add_dict(item->dict,
                 pdf_new_name("A"), pdf_link_obj(action));
    pdf_release_obj(action);
  }

  item->first    = first = NEW(1, pdf_olitem);
  first->dict    = NULL;
  first->is_open = 0;
  first->parent  = item;
  first->next    = NULL;
  first->first   = NULL;

  p->outlines.current = first;
  p->outlines.current_depth++;

  return 0;
}

int
pdf_doc_bookmarks_depth (void)
{
  pdf_doc *p = &pdoc;

  return p->outlines.current_depth;
}

void
pdf_doc_bookmarks_add (pdf_obj *dict, int is_open)
{
  pdf_doc    *p = &pdoc;
  pdf_olitem *item, *next;

  assert(p && dict);

  item = p->outlines.current;

  if (!item) {
    item = NEW(1, pdf_olitem);
    item->parent = NULL;
    p->outlines.first = item;
  } else if (item->dict) { /* go to next item */
    item = item->next;
  }

#define BMOPEN(b,p) (((b) < 0) ? (((p)->outlines.current_depth > (p)->opt.outline_open_depth) ? 0 : 1) : (b))

  item->dict    = dict;
  item->first   = NULL;
  item->is_open = BMOPEN(is_open, p);

  item->next    = next = NEW(1, pdf_olitem);
  next->dict    = NULL;
  next->parent  = item->parent;
  next->first   = NULL;
  next->is_open = -1;
  next->next    = NULL;

  p->outlines.current = item;

  pdf_doc_add_goto(dict);

  return;
}

static void
pdf_doc_close_bookmarks (pdf_doc *p)
{
  pdf_obj     *catalog = p->root.dict;
  pdf_olitem  *item;
  int          count;
  pdf_obj     *bm_root, *bm_root_ref;

  item = p->outlines.first;
  if (item->dict) {
    bm_root     = pdf_new_dict();
    bm_root_ref = pdf_ref_obj(bm_root);
    count       = flush_bookmarks(item, bm_root_ref, bm_root);
    pdf_add_dict(bm_root,
                 pdf_new_name("Count"),
                 pdf_new_number(count));
    pdf_add_dict(catalog,
                 pdf_new_name("Outlines"),
                 bm_root_ref);
    pdf_release_obj(bm_root);
  }
  clean_bookmarks(item);

  p->outlines.first   = NULL;
  p->outlines.current = NULL;
  p->outlines.current_depth = 0;

  return;
}


static const char *name_dict_categories[] = {
  "Dests", "AP", "JavaScript", "Pages",
  "Templates", "IDS", "URLS", "EmbeddedFiles",
  "AlternatePresentations", "Renditions"
};
#define NUM_NAME_CATEGORY (sizeof(name_dict_categories)/sizeof(name_dict_categories[0]))

static void
pdf_doc_init_names (pdf_doc *p, int check_gotos)
{
  unsigned int i;

  p->root.names   = NULL;

  p->names = NEW(NUM_NAME_CATEGORY + 1, struct name_dict);
  for (i = 0; i < NUM_NAME_CATEGORY; i++) {
    p->names[i].category = name_dict_categories[i];
    p->names[i].data     = strcmp(name_dict_categories[i], "Dests") ?
                             NULL : pdf_new_name_tree();
    /*
     * We need a non-null entry for PDF destinations in order to find
     * broken links even if no destination is defined in the DVI file.
     */
  }
  p->names[NUM_NAME_CATEGORY].category = NULL;
  p->names[NUM_NAME_CATEGORY].data     = NULL;

  p->check_gotos   = check_gotos;
  ht_init_table(&p->gotos, (void (*) (void *)) pdf_release_obj);

  return;
}

int
pdf_doc_add_names (const char *category,
                   const void *key, int keylen, pdf_obj *value)
{
  pdf_doc *p = &pdoc;
  unsigned int i;

  for (i = 0; p->names[i].category != NULL; i++) {
    if (streq_ptr(p->names[i].category, category)) {
      break;
    }
  }
  if (p->names[i].category == NULL) {
    dpx_warning("Unknown name dictionary category \"%s\".", category);
    return -1;
  }
  if (!p->names[i].data) {
    p->names[i].data = pdf_new_name_tree();
  }

  return pdf_names_add_object(p->names[i].data, key, keylen, value);
}

static void
pdf_doc_add_goto (pdf_obj *annot_dict)
{
  pdf_obj *subtype = NULL, *A = NULL, *S = NULL, *D = NULL, *D_new, *dict;
  const char *dest, *key;
  int destlen = 0;

  if (!pdoc.check_gotos)
    return;

  /*
   * An annotation dictionary coming from an annotation special
   * must have a "Subtype". An annotation dictionary coming from
   * an outline special has none.
   */
  subtype = pdf_deref_obj(pdf_lookup_dict(annot_dict, "Subtype"));
  if (subtype) {
    if (PDF_OBJ_UNDEFINED(subtype))
      goto undefined;
    else if (!PDF_OBJ_NAMETYPE(subtype))
      goto error;
    else if (strcmp(pdf_name_value(subtype), "Link"))
      goto cleanup;
  }

  dict = annot_dict;
  key = "Dest";
  D = pdf_deref_obj(pdf_lookup_dict(annot_dict, key));
  if (PDF_OBJ_UNDEFINED(D))
    goto undefined;

  A = pdf_deref_obj(pdf_lookup_dict(annot_dict, "A"));
  if (A) {
    if (PDF_OBJ_UNDEFINED(A))
      goto undefined;
    else if (D || !PDF_OBJ_DICTTYPE(A))
      goto error;
    else {
      S = pdf_deref_obj(pdf_lookup_dict(A, "S"));
      if (PDF_OBJ_UNDEFINED(S))
        goto undefined;
      else if (!PDF_OBJ_NAMETYPE(S))
        goto error;
      else if (strcmp(pdf_name_value(S), "GoTo"))
        goto cleanup;

      dict = A;
      key = "D";
      D = pdf_deref_obj(pdf_lookup_dict(A, key));
    }
  }

  if (PDF_OBJ_STRINGTYPE(D)) {
    dest = (char *) pdf_string_value(D);
    destlen = pdf_string_length(D);
  }
  else if (PDF_OBJ_ARRAYTYPE(D))
    goto cleanup;
  else if (PDF_OBJ_UNDEFINED(D))
    goto undefined;
  else
    goto error;

  D_new = ht_lookup_table(&pdoc.gotos, dest, destlen);
  if (!D_new) {
    char buf[10];

    /* We use hexadecimal notation for our numeric destinations.
     * Other bases (e.g., 10+26 or 10+2*26) would be more efficient.
     */
    sprintf(buf, "%x", ht_table_size(&pdoc.gotos));
    D_new = pdf_new_string(buf, strlen(buf));
    ht_append_table(&pdoc.gotos, dest, destlen, D_new);
  }

  {
    pdf_obj *key_obj = pdf_new_name(key);
    if (!pdf_add_dict(dict, key_obj, pdf_link_obj(D_new)))
      pdf_release_obj(key_obj);
  }

 cleanup:
  pdf_release_obj(subtype);
  pdf_release_obj(A);
  pdf_release_obj(S);
  pdf_release_obj(D);

  return;

 error:
  dpx_warning("Unknown PDF annotation format. Output file may be broken.");
  goto cleanup;

 undefined:
  dpx_warning("Cannot optimize PDF annotations. Output file may be broken."
       " Please restart with option \"-C 0x10\"\n");
  goto cleanup;
}

static void
warn_undef_dests (struct ht_table *dests, struct ht_table *gotos)
{
  struct ht_iter iter;

  if (ht_set_iter(gotos, &iter) < 0)
    return;

  do {
    int keylen;
    char *key = ht_iter_getkey(&iter, &keylen);
    if (!ht_lookup_table(dests, key, keylen)) {
      char *dest = NEW(keylen+1, char);
      memcpy(dest, key, keylen);
      dest[keylen] = 0;
      dpx_warning("PDF destination \"%s\" not defined.", dest);
      free(dest);
    }
  } while (ht_iter_next(&iter) >= 0);

  ht_clear_iter(&iter);
}

static void
pdf_doc_close_names (pdf_doc *p)
{
  pdf_obj  *tmp;
  unsigned int i;

  for (i = 0; p->names[i].category != NULL; i++) {
    if (p->names[i].data) {
      struct ht_table *data = p->names[i].data;
      pdf_obj  *name_tree;
      int count;

      if (!pdoc.check_gotos || strcmp(p->names[i].category, "Dests"))
        name_tree = pdf_names_create_tree(data, &count, NULL);
      else {
        name_tree = pdf_names_create_tree(data, &count, &pdoc.gotos);

        if (dpx_conf.verbose_level > 0 && count < data->count)
          dpx_message("\nRemoved %d unused PDF destinations\n", data->count-count);

        if (count < pdoc.gotos.count)
          warn_undef_dests(data, &pdoc.gotos);
      }

      if (name_tree) {
        if (!p->root.names)
          p->root.names = pdf_new_dict();
        pdf_add_dict(p->root.names,
                     pdf_new_name(p->names[i].category),
                     pdf_ref_obj(name_tree));
        pdf_release_obj(name_tree);
      }
      pdf_delete_name_tree(&p->names[i].data);
    }
  }

  if (p->root.names) {
    tmp = pdf_lookup_dict(p->root.dict, "Names");
    if (!tmp) {
      pdf_add_dict(p->root.dict,
                   pdf_new_name("Names"),
                   pdf_ref_obj (p->root.names));
    } else if (PDF_OBJ_DICTTYPE(tmp)) {
      pdf_merge_dict(p->root.names, tmp);
      pdf_add_dict(p->root.dict,
                   pdf_new_name("Names"),
                   pdf_ref_obj (p->root.names));
    } else { /* Maybe reference */
      /* What should I do? */
      dpx_warning("Could not modify Names dictionary.");
    }
    pdf_release_obj(p->root.names);
    p->root.names = NULL;
  }

  p->names = mfree(p->names);

  ht_clear_table(&p->gotos);

  return;
}

static void pdf_doc_get_mediabox (unsigned page_no, pdf_rect *mediabox);

void
pdf_doc_add_annot (unsigned page_no, const pdf_rect *rect,
                   pdf_obj *annot_dict, int new_annot)
{
  pdf_doc  *p = &pdoc;
  pdf_page *page;
  pdf_obj  *rect_array;
  double    xpos, ypos;
  pdf_rect  annbox;

  page = doc_get_page_entry(p, page_no);
  if (!page->annots)
    page->annots = pdf_new_array();

  {
    pdf_rect  mediabox;

    pdf_doc_get_mediabox(page_no, &mediabox);
    pdf_dev_get_coord(&xpos, &ypos);
    annbox.llx = rect->llx - xpos; annbox.lly = rect->lly - ypos;
    annbox.urx = rect->urx - xpos; annbox.ury = rect->ury - ypos;

    if (annbox.llx < mediabox.llx || annbox.urx > mediabox.urx ||
        annbox.lly < mediabox.lly || annbox.ury > mediabox.ury) {
      dpx_warning("Annotation out of page boundary.");
      dpx_warning("Current page's MediaBox: [%g %g %g %g]",
           mediabox.llx, mediabox.lly, mediabox.urx, mediabox.ury);
      dpx_warning("Annotation: [%g %g %g %g]",
           annbox.llx, annbox.lly, annbox.urx, annbox.ury);
      dpx_warning("Maybe incorrect paper size specified.");
    }
    if (annbox.llx > annbox.urx || annbox.lly > annbox.ury) {
      dpx_warning("Rectangle with negative width/height: [%g %g %g %g]",
           annbox.llx, annbox.lly, annbox.urx, annbox.ury);
    }
  }

  rect_array = pdf_new_array();
  pdf_add_array(rect_array, pdf_new_number(ROUND(annbox.llx, 0.001)));
  pdf_add_array(rect_array, pdf_new_number(ROUND(annbox.lly, 0.001)));
  pdf_add_array(rect_array, pdf_new_number(ROUND(annbox.urx, 0.001)));
  pdf_add_array(rect_array, pdf_new_number(ROUND(annbox.ury, 0.001)));
  pdf_add_dict (annot_dict, pdf_new_name("Rect"), rect_array);

  pdf_add_array(page->annots, pdf_ref_obj(annot_dict));

  if (new_annot)
    pdf_doc_add_goto(annot_dict);

  return;
}


/*
 * PDF Article Thread
 */
static void
pdf_doc_init_articles (pdf_doc *p)
{
  p->root.threads = NULL;

  p->articles.num_entries = 0;
  p->articles.max_entries = 0;
  p->articles.entries     = NULL;

  return;
}

void
pdf_doc_begin_article (const char *article_id, pdf_obj *article_info)
{
  pdf_doc     *p = &pdoc;
  pdf_article *article;

  if (article_id == NULL || strlen(article_id) == 0)
    _tt_abort("Article thread without internal identifier.");

  if (p->articles.num_entries >= p->articles.max_entries) {
    p->articles.max_entries += PDFDOC_ARTICLE_ALLOC_SIZE;
    p->articles.entries = RENEW(p->articles.entries,
                                p->articles.max_entries, struct pdf_article);
  }
  article = &(p->articles.entries[p->articles.num_entries]);

  article->id = NEW(strlen(article_id)+1, char);
  strcpy(article->id, article_id);
  article->info = article_info;
  article->num_beads = 0;
  article->max_beads = 0;
  article->beads     = NULL;

  p->articles.num_entries++;

  return;
}

static pdf_bead *
find_bead (pdf_article *article, const char *bead_id)
{
  pdf_bead *bead;
  unsigned int i;

  bead = NULL;
  for (i = 0; i < article->num_beads; i++) {
    if (streq_ptr(article->beads[i].id, bead_id)) {
      bead = &(article->beads[i]);
      break;
    }
  }

  return bead;
}

void
pdf_doc_add_bead (const char *article_id,
                  const char *bead_id, int page_no, const pdf_rect *rect)
{
  pdf_doc     *p = &pdoc;
  pdf_article *article;
  pdf_bead    *bead;
  unsigned int i;

  if (!article_id) {
    _tt_abort("No article identifier specified.");
  }

  article = NULL;
  for (i = 0; i < p->articles.num_entries; i++) {
    if (streq_ptr(p->articles.entries[i].id, article_id)) {
      article = &(p->articles.entries[i]);
      break;
    }
  }
  if (!article) {
    _tt_abort("Specified article thread that doesn't exist.");
  }

  bead = bead_id ? find_bead(article, bead_id) : NULL;
  if (!bead) {
    if (article->num_beads >= article->max_beads) {
      article->max_beads += PDFDOC_BEAD_ALLOC_SIZE;
      article->beads = RENEW(article->beads,
                             article->max_beads, struct pdf_bead);
      for (i = article->num_beads; i < article->max_beads; i++) {
        article->beads[i].id = NULL;
        article->beads[i].page_no = -1;
      }
    }
    bead = &(article->beads[article->num_beads]);
    if (bead_id) {
      bead->id = NEW(strlen(bead_id)+1, char);
      strcpy(bead->id, bead_id);
    } else {
      bead->id = NULL;
    }
    article->num_beads++;
  }
  bead->rect.llx = rect->llx;
  bead->rect.lly = rect->lly;
  bead->rect.urx = rect->urx;
  bead->rect.ury = rect->ury;
  bead->page_no  = page_no;

  return;
}

static pdf_obj *
make_article (pdf_doc *p,
              pdf_article *article,
              const char **bead_ids, unsigned int num_beads,
              pdf_obj *article_info)
{
  pdf_obj *art_dict;
  pdf_obj *first, *prev, *last;
  int      i, n;

  if (!article)
    return NULL;

  art_dict = pdf_new_dict();
  first = prev = last = NULL;
  /*
   * The bead_ids represents logical order of beads in an article thread.
   * If bead_ids is not given, we create an article thread in the order of
   * beads appeared.
   */
  n = bead_ids ? num_beads : article->num_beads;
  for (i = 0; i < n; i++) {
    pdf_bead *bead;

    bead = bead_ids ? find_bead(article, bead_ids[i]) : &(article->beads[i]);
    if (!bead || bead->page_no < 0) {
      continue;
    }
    last = pdf_new_dict();
    if (prev == NULL) {
      first = last;
      pdf_add_dict(first,
                   pdf_new_name("T"), pdf_ref_obj(art_dict));
    } else {
      pdf_add_dict(prev,
                   pdf_new_name("N"), pdf_ref_obj(last));
      pdf_add_dict(last,
                   pdf_new_name("V"), pdf_ref_obj(prev));
      /* We must link first to last. */
      if (prev != first)
        pdf_release_obj(prev);
    }

    /* Realize bead now. */
    {
      pdf_page *page;
      pdf_obj  *rect;

      page = doc_get_page_entry(p, bead->page_no);
      if (!page->beads) {
        page->beads = pdf_new_array();
      }
      pdf_add_dict(last, pdf_new_name("P"), pdf_link_obj(page->page_ref));
      rect = pdf_new_array();
      pdf_add_array(rect, pdf_new_number(ROUND(bead->rect.llx, 0.01)));
      pdf_add_array(rect, pdf_new_number(ROUND(bead->rect.lly, 0.01)));
      pdf_add_array(rect, pdf_new_number(ROUND(bead->rect.urx, 0.01)));
      pdf_add_array(rect, pdf_new_number(ROUND(bead->rect.ury, 0.01)));
      pdf_add_dict (last, pdf_new_name("R"), rect);
      pdf_add_array(page->beads, pdf_ref_obj(last));
    }

    prev = last;
  }

  if (first && last) {
    pdf_add_dict(last,
                 pdf_new_name("N"), pdf_ref_obj(first));
    pdf_add_dict(first,
                 pdf_new_name("V"), pdf_ref_obj(last));
    if (first != last) {
      pdf_release_obj(last);
    }
    pdf_add_dict(art_dict,
                 pdf_new_name("F"), pdf_ref_obj(first));
    /* If article_info is supplied, we override article->info. */
    if (article_info) {
      pdf_add_dict(art_dict,
                   pdf_new_name("I"), article_info);
    } else if (article->info) {
      pdf_add_dict(art_dict,
                   pdf_new_name("I"), pdf_ref_obj(article->info));
      pdf_release_obj(article->info);
      article->info = NULL; /* We do not write as object reference. */
    }
    pdf_release_obj(first);
  } else {
    pdf_release_obj(art_dict);
    art_dict = NULL;
  }

  return art_dict;
}

static void
clean_article (pdf_article *article)
{
  if (!article)
    return;

  if (article->beads) {
    unsigned int i;

    for (i = 0; i < article->num_beads; i++) {
      free(article->beads[i].id);
    }
    article->beads = mfree(article->beads);
  }

  article->id = mfree(article->id);
  article->num_beads = 0;
  article->max_beads = 0;

  return;
}

static void
pdf_doc_close_articles (pdf_doc *p)
{
  unsigned int i;

  for (i = 0; i < p->articles.num_entries; i++) {
    pdf_article *article;

    article = &(p->articles.entries[i]);
    if (article->beads) {
      pdf_obj *art_dict;

      art_dict = make_article(p, article, NULL, 0, NULL);
      if (!p->root.threads) {
        p->root.threads = pdf_new_array();
      }
      pdf_add_array(p->root.threads, pdf_ref_obj(art_dict));
      pdf_release_obj(art_dict);
    }
    clean_article(article);
  }
  p->articles.entries = mfree(p->articles.entries);
  p->articles.num_entries = 0;
  p->articles.max_entries = 0;

  if (p->root.threads) {
    pdf_add_dict(p->root.dict,
                 pdf_new_name("Threads"),
                 pdf_ref_obj (p->root.threads));
    pdf_release_obj(p->root.threads);
    p->root.threads = NULL;
  }

  return;
}

/* page_no = 0 for root page tree node. */
void
pdf_doc_set_mediabox (unsigned page_no, const pdf_rect *mediabox)
{
  pdf_doc  *p = &pdoc;
  pdf_page *page;

  if (page_no == 0) {
    p->pages.mediabox.llx = mediabox->llx;
    p->pages.mediabox.lly = mediabox->lly;
    p->pages.mediabox.urx = mediabox->urx;
    p->pages.mediabox.ury = mediabox->ury;
  } else {
    page = doc_get_page_entry(p, page_no);
    page->cropbox.llx = mediabox->llx;
    page->cropbox.lly = mediabox->lly;
    page->cropbox.urx = mediabox->urx;
    page->cropbox.ury = mediabox->ury;
    page->flags |= USE_MY_MEDIABOX;
  }

  return;
}

static void
pdf_doc_get_mediabox (unsigned page_no, pdf_rect *mediabox)
{
  pdf_doc  *p = &pdoc;
  pdf_page *page;

  if (page_no == 0) {
    mediabox->llx = p->pages.mediabox.llx;
    mediabox->lly = p->pages.mediabox.lly;
    mediabox->urx = p->pages.mediabox.urx;
    mediabox->ury = p->pages.mediabox.ury;
  } else {
    page = doc_get_page_entry(p, page_no);
    if (page->flags & USE_MY_MEDIABOX) {
      mediabox->llx = page->cropbox.llx;
      mediabox->lly = page->cropbox.lly;
      mediabox->urx = page->cropbox.urx;
      mediabox->ury = page->cropbox.ury;
    } else {
      mediabox->llx = p->pages.mediabox.llx;
      mediabox->lly = p->pages.mediabox.lly;
      mediabox->urx = p->pages.mediabox.urx;
      mediabox->ury = p->pages.mediabox.ury;
    }
  }

  return;
}

pdf_obj *
pdf_doc_current_page_resources (void)
{
  pdf_obj  *resources;
  pdf_doc  *p = &pdoc;
  pdf_page *currentpage;

  if (p->pending_forms) {
    if (p->pending_forms->form.resources) {
      resources = p->pending_forms->form.resources;
    } else {
      resources = p->pending_forms->form.resources = pdf_new_dict();
    }
  } else {
    currentpage = LASTPAGE(p);
    if (currentpage->resources) {
      resources = currentpage->resources;
    } else {
      resources = currentpage->resources = pdf_new_dict();
    }
  }

  return resources;
}

pdf_obj *
pdf_doc_get_dictionary (const char *category)
{
  pdf_doc *p    = &pdoc;
  pdf_obj *dict = NULL;

  assert(category);

  if (streq_ptr(category, "Names")) {
    if (!p->root.names)
      p->root.names = pdf_new_dict();
    dict = p->root.names;
  } else if (streq_ptr(category, "Pages")) {
    if (!p->root.pages)
      p->root.pages = pdf_new_dict();
    dict = p->root.pages;
  } else if (streq_ptr(category, "Catalog")) {
    if (!p->root.dict)
      p->root.dict = pdf_new_dict();
    dict = p->root.dict;
  } else if (streq_ptr(category, "Info")) {
    if (!p->info)
      p->info = pdf_new_dict();
    dict = p->info;
  } else if (streq_ptr(category, "@THISPAGE")) {
    /* Sorry for this... */
    pdf_page *currentpage;

    currentpage = LASTPAGE(p);
    dict =  currentpage->page_obj;
  }

  if (!dict) {
    _tt_abort("Document dict. \"%s\" not exist. ", category);
  }

  return dict;
}

int
pdf_doc_current_page_number (void)
{
  pdf_doc *p = &pdoc;

  return (int) (PAGECOUNT(p) + 1);
}

pdf_obj *
pdf_doc_ref_page (unsigned page_no)
{
  pdf_doc  *p = &pdoc;
  pdf_page *page;

  page = doc_get_page_entry(p, page_no);
  if (!page->page_obj) {
    page->page_obj = pdf_new_dict();
    page->page_ref = pdf_ref_obj(page->page_obj);
  }

  return pdf_link_obj(page->page_ref);
}

pdf_obj *
pdf_doc_get_reference (const char *category)
{
  pdf_obj *ref = NULL;
  int      page_no;

  assert(category);

  page_no = pdf_doc_current_page_number();
  if (streq_ptr(category, "@THISPAGE")) {
    ref = pdf_doc_ref_page(page_no);
  } else if (streq_ptr(category, "@PREVPAGE")) {
    if (page_no <= 1) {
      _tt_abort("Reference to previous page, but no pages have been completed yet.");
    }
    ref = pdf_doc_ref_page(page_no - 1);
  } else if (streq_ptr(category, "@NEXTPAGE")) {
    ref = pdf_doc_ref_page(page_no + 1);
  }

  if (!ref) {
    _tt_abort("Reference to \"%s\" not exist. ", category);
  }

  return ref;
}

static void
pdf_doc_new_page (pdf_doc *p)
{
  pdf_page *currentpage;

  if (PAGECOUNT(p) >= MAXPAGES(p)) {
    doc_resize_page_entries(p, MAXPAGES(p) + PDFDOC_PAGES_ALLOC_SIZE);
  }

  /*
   * This is confusing. pdf_doc_finish_page() have increased page count!
   */
  currentpage = LASTPAGE(p);
  /* Was this page already instantiated by a forward reference to it? */
  if (!currentpage->page_ref) {
    currentpage->page_obj = pdf_new_dict();
    currentpage->page_ref = pdf_ref_obj(currentpage->page_obj);
  }

  currentpage->background = NULL;
  currentpage->contents   = pdf_new_stream(STREAM_COMPRESS);
  currentpage->resources  = pdf_new_dict();

  currentpage->annots = NULL;
  currentpage->beads  = NULL;

  return;
}

/* This only closes contents and resources. */
static void
pdf_doc_finish_page (pdf_doc *p)
{
  pdf_page *currentpage;

  if (p->pending_forms) {
    _tt_abort("A pending form XObject at the end of page.");
  }

  currentpage = LASTPAGE(p);
  if (!currentpage->page_obj)
    currentpage->page_obj = pdf_new_dict();

  /*
   * Make Contents array.
   */

  /*
   * Global BOP content stream.
   * pdf_ref_obj() returns reference itself when the object is
   * indirect reference, not reference to the indirect reference.
   * We keep bop itself but not reference to it since it is
   * expected to be small.
   */
  if (p->pages.bop &&
      pdf_stream_length(p->pages.bop) > 0) {
    currentpage->content_refs[0] = pdf_ref_obj(p->pages.bop);
  } else {
    currentpage->content_refs[0] = NULL;
  }
  /*
   * Current page background content stream.
   */
  if (currentpage->background) {
    if (pdf_stream_length(currentpage->background) > 0) {
      currentpage->content_refs[1] = pdf_ref_obj(currentpage->background);
      pdf_add_stream (currentpage->background, "\n", 1);
    }
    pdf_release_obj(currentpage->background);
    currentpage->background = NULL;
  } else {
    currentpage->content_refs[1] = NULL;
  }

  /* Content body of current page */
  currentpage->content_refs[2] = pdf_ref_obj(currentpage->contents);
  pdf_add_stream (currentpage->contents, "\n", 1);
  pdf_release_obj(currentpage->contents);
  currentpage->contents = NULL;

  /*
   * Global EOP content stream.
   */
  if (p->pages.eop &&
      pdf_stream_length(p->pages.eop) > 0) {
    currentpage->content_refs[3] = pdf_ref_obj(p->pages.eop);
  } else {
    currentpage->content_refs[3] = NULL;
  }

  /*
   * Page resources.
   */
  if (currentpage->resources) {
    pdf_obj *procset;
    /*
     * ProcSet is obsolete in PDF-1.4 but recommended for compatibility.
     */

    procset = pdf_new_array ();
    pdf_add_array(procset, pdf_new_name("PDF"));
    pdf_add_array(procset, pdf_new_name("Text"));
    pdf_add_array(procset, pdf_new_name("ImageC"));
    pdf_add_array(procset, pdf_new_name("ImageB"));
    pdf_add_array(procset, pdf_new_name("ImageI"));
    pdf_add_dict(currentpage->resources, pdf_new_name("ProcSet"), procset);

    pdf_add_dict(currentpage->page_obj,
                 pdf_new_name("Resources"),
                 pdf_ref_obj(currentpage->resources));
    pdf_release_obj(currentpage->resources);
    currentpage->resources = NULL;
  }

  if (manual_thumb_enabled) {
    char    *thumb_filename;
    pdf_obj *thumb_ref;

    thumb_filename = NEW(strlen(thumb_basename)+7, char);
    sprintf(thumb_filename, "%s.%ld",
            thumb_basename, (p->pages.num_entries % 99999) + 1L);
    thumb_ref = read_thumbnail(thumb_filename);
    free(thumb_filename);
    if (thumb_ref)
      pdf_add_dict(currentpage->page_obj, pdf_new_name("Thumb"), thumb_ref);
  }

  p->pages.num_entries++;

  return;
}

static pdf_color bgcolor = { 1, NULL, { 1.0 } };

void
pdf_doc_set_bgcolor (const pdf_color *color)
{
  if (color)
    pdf_color_copycolor(&bgcolor, color);
  else { /* as clear... */
    pdf_color_white(&bgcolor);
  }
}

static void
doc_fill_page_background (pdf_doc *p)
{
  pdf_page  *currentpage;
  pdf_rect   r;
  int        cm;
  pdf_obj   *saved_content;

  cm = pdf_dev_get_param(PDF_DEV_PARAM_COLORMODE);
  if (!cm || pdf_color_is_white(&bgcolor)) {
    return;
  }

  pdf_doc_get_mediabox(pdf_doc_current_page_number(), &r);

  currentpage = LASTPAGE(p);
  assert(currentpage);

  if (!currentpage->background)
    currentpage->background = pdf_new_stream(STREAM_COMPRESS);

  saved_content = currentpage->contents;
  currentpage->contents = currentpage->background;

  pdf_dev_gsave();
  pdf_dev_set_nonstrokingcolor(&bgcolor);
  pdf_dev_rectfill(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);
  pdf_dev_grestore();

  currentpage->contents = saved_content;

  return;
}

void
pdf_doc_begin_page (double scale, double x_origin, double y_origin)
{
  pdf_doc     *p = &pdoc;
  pdf_tmatrix  M;

  M.a = scale; M.b = 0.0;
  M.c = 0.0  ; M.d = scale;
  M.e = x_origin;
  M.f = y_origin;

  /* pdf_doc_new_page() allocates page content stream. */
  pdf_doc_new_page(p);
  pdf_dev_bop(&M);

  return;
}

void
pdf_doc_end_page (void)
{
  pdf_doc *p = &pdoc;

  pdf_dev_eop();
  doc_fill_page_background(p);

  pdf_doc_finish_page(p);

  return;
}

void
pdf_doc_add_page_content (const char *buffer, unsigned int length)
{
  pdf_doc  *p = &pdoc;
  pdf_page *currentpage;

  if (p->pending_forms) {
    pdf_add_stream(p->pending_forms->form.contents, buffer, length);
  } else {
    currentpage = LASTPAGE(p);
    pdf_add_stream(currentpage->contents, buffer, length);
  }

  return;
}

void
pdf_open_document (const char *filename,
                   const char *creator, const unsigned char *id1, const unsigned char *id2,
                   struct pdf_setting settings)
{
  pdf_doc *p = &pdoc;

  if (settings.enable_encrypt)
    pdf_init_encryption(settings.encrypt, id1);

  pdf_out_init(filename, settings.enable_encrypt,
               settings.object.enable_objstm, settings.object.enable_predictor);
  pdf_files_init();

  pdf_doc_init_catalog(p);

  p->opt.annot_grow = settings.annot_grow_amount;
  p->opt.outline_open_depth = settings.outline_open_depth;

  pdf_init_resources();
  pdf_init_colors();
  pdf_init_fonts();
  /* Thumbnail want this to be initialized... */
  pdf_init_images();

  pdf_doc_init_docinfo(p);
  if (creator) {
    pdf_add_dict(p->info,
                 pdf_new_name("Creator"),
                 pdf_new_string(creator, strlen(creator)));
  }

  pdf_doc_init_bookmarks(p, settings.outline_open_depth);
  pdf_doc_init_articles (p);
  pdf_doc_init_names    (p, settings.check_gotos);
  pdf_doc_init_page_tree(p, settings.media_width, settings.media_height);

  pdf_doc_set_bgcolor(NULL);

  if (settings.enable_encrypt) {
    pdf_obj *encrypt = pdf_encrypt_obj();
    pdf_set_encrypt(encrypt);
    pdf_release_obj(encrypt);
  }
  if (id1 && id2) {
    pdf_obj *id_obj = pdf_new_array();

    pdf_add_array(id_obj, pdf_new_string(id1, 16));
    pdf_add_array(id_obj, pdf_new_string(id2, 16));
    pdf_set_id(id_obj);
  }

  /* Create a default name for thumbnail image files */
  if (manual_thumb_enabled) {
      size_t fn_len = strlen(filename);

      if (fn_len > 4 && !strncmp(".pdf", filename + fn_len - 4, 4)) {
          thumb_basename = NEW(fn_len - 4 + 1, char);
          strncpy(thumb_basename, filename, fn_len - 4);
          thumb_basename[fn_len - 4] = 0;
      } else {
          thumb_basename = NEW(fn_len + 1, char);
          strcpy(thumb_basename, filename);
      }
  }

  p->pending_forms = NULL;

  pdf_init_device(settings.device.dvi2pts, settings.device.precision,
                  settings.device.ignore_colors);

  return;
}

void
pdf_close_document (void)
{
  pdf_doc *p = &pdoc;

  pdf_close_device();

  /*
   * Following things were kept around so user can add dictionary items.
   */
  pdf_doc_close_articles (p);
  pdf_doc_close_names    (p);
  pdf_doc_close_bookmarks(p);
  pdf_doc_close_page_tree(p);
  pdf_doc_close_docinfo  (p);

  pdf_doc_close_catalog  (p);

  pdf_close_images();
  pdf_close_fonts ();
  pdf_close_colors();

  pdf_close_resources(); /* Should be at last. */

  pdf_files_close();
  pdf_out_flush();

  free(thumb_basename);

  return;
}

/*
 * All this routine does is give the form a name and add a unity scaling matrix.
 * It fills in required fields.  The caller must initialize the stream.
 */
static void
pdf_doc_make_xform (pdf_obj     *xform,
                    pdf_rect    *bbox,
                    pdf_tmatrix *matrix,
                    pdf_obj     *resources,
                    pdf_obj     *attrib)
{
  pdf_obj *xform_dict;
  pdf_obj *tmp;

  xform_dict = pdf_stream_dict(xform);
  pdf_add_dict(xform_dict,
               pdf_new_name("Type"),     pdf_new_name("XObject"));
  pdf_add_dict(xform_dict,
               pdf_new_name("Subtype"),  pdf_new_name("Form"));
  pdf_add_dict(xform_dict,
               pdf_new_name("FormType"), pdf_new_number(1.0));

  if (!bbox)
    _tt_abort("No BoundingBox supplied.");

  tmp = pdf_new_array();
  pdf_add_array(tmp, pdf_new_number(ROUND(bbox->llx, .001)));
  pdf_add_array(tmp, pdf_new_number(ROUND(bbox->lly, .001)));
  pdf_add_array(tmp, pdf_new_number(ROUND(bbox->urx, .001)));
  pdf_add_array(tmp, pdf_new_number(ROUND(bbox->ury, .001)));
  pdf_add_dict(xform_dict, pdf_new_name("BBox"), tmp);

  if (matrix) {
    tmp = pdf_new_array();
    pdf_add_array(tmp, pdf_new_number(ROUND(matrix->a, .00001)));
    pdf_add_array(tmp, pdf_new_number(ROUND(matrix->b, .00001)));
    pdf_add_array(tmp, pdf_new_number(ROUND(matrix->c, .00001)));
    pdf_add_array(tmp, pdf_new_number(ROUND(matrix->d, .00001)));
    pdf_add_array(tmp, pdf_new_number(ROUND(matrix->e, .001  )));
    pdf_add_array(tmp, pdf_new_number(ROUND(matrix->f, .001  )));
    pdf_add_dict(xform_dict, pdf_new_name("Matrix"), tmp);
  }

  if (attrib) {
    pdf_merge_dict(xform_dict, attrib);
  }

  pdf_add_dict(xform_dict, pdf_new_name("Resources"), resources);

  return;
}

/*
 * begin_form_xobj creates an xobject with its "origin" at
 * xpos and ypos that is clipped to the specified bbox. Note
 * that the origin is not the lower left corner of the bbox.
 */
int
pdf_doc_begin_grabbing (const char *ident,
                        double ref_x, double ref_y, const pdf_rect *cropbox)
{
  int         xobj_id = -1;
  pdf_doc    *p = &pdoc;
  pdf_form   *form;
  struct form_list_node *fnode;
  xform_info  info;

  pdf_dev_push_gstate();

  fnode = NEW(1, struct form_list_node);

  fnode->prev    = p->pending_forms;
  fnode->q_depth = pdf_dev_current_depth();
  form           = &fnode->form;

  /*
  * The reference point of an Xobject is at the lower left corner
  * of the bounding box.  Since we would like to have an arbitrary
  * reference point, we use a transformation matrix, translating
  * the reference point to (0,0).
  */

  form->matrix.a = 1.0; form->matrix.b = 0.0;
  form->matrix.c = 0.0; form->matrix.d = 1.0;
  form->matrix.e = -ref_x;
  form->matrix.f = -ref_y;

  form->cropbox.llx = ref_x + cropbox->llx;
  form->cropbox.lly = ref_y + cropbox->lly;
  form->cropbox.urx = ref_x + cropbox->urx;
  form->cropbox.ury = ref_y + cropbox->ury;

  form->contents  = pdf_new_stream(STREAM_COMPRESS);
  form->resources = pdf_new_dict();

  pdf_ximage_init_form_info(&info);

  info.matrix.a = 1.0; info.matrix.b = 0.0;
  info.matrix.c = 0.0; info.matrix.d = 1.0;
  info.matrix.e = -ref_x;
  info.matrix.f = -ref_y;

  info.bbox.llx = cropbox->llx;
  info.bbox.lly = cropbox->lly;
  info.bbox.urx = cropbox->urx;
  info.bbox.ury = cropbox->ury;

  /* Use reference since content itself isn't available yet. */
  xobj_id = pdf_ximage_defineresource(ident,
                                      PDF_XOBJECT_TYPE_FORM,
                                      &info, pdf_ref_obj(form->contents));

  p->pending_forms = fnode;

  /*
   * Make sure the object is self-contained by adding the
   * current font and color to the object stream.
   */
  pdf_dev_reset_fonts(1);
  pdf_dev_reset_color(1);  /* force color operators to be added to stream */

  return xobj_id;
}

void
pdf_doc_end_grabbing (pdf_obj *attrib)
{
  pdf_form *form;
  pdf_obj  *procset;
  pdf_doc  *p = &pdoc;
  struct form_list_node *fnode;

  if (!p->pending_forms) {
    dpx_warning("Tried to close a nonexistent form XOject.");
    return;
  }

  fnode = p->pending_forms;
  form  = &fnode->form;

  pdf_dev_grestore_to(fnode->q_depth);

  /*
   * ProcSet is obsolete in PDF-1.4 but recommended for compatibility.
   */
  procset = pdf_new_array();
  pdf_add_array(procset, pdf_new_name("PDF"));
  pdf_add_array(procset, pdf_new_name("Text"));
  pdf_add_array(procset, pdf_new_name("ImageC"));
  pdf_add_array(procset, pdf_new_name("ImageB"));
  pdf_add_array(procset, pdf_new_name("ImageI"));
  pdf_add_dict (form->resources, pdf_new_name("ProcSet"), procset);

  pdf_doc_make_xform(form->contents,
                     &form->cropbox, &form->matrix,
                     pdf_ref_obj(form->resources), attrib);
  pdf_release_obj(form->resources);
  pdf_release_obj(form->contents);
  pdf_release_obj(attrib);

  p->pending_forms = fnode->prev;

  pdf_dev_pop_gstate();

  pdf_dev_reset_fonts(1);
  pdf_dev_reset_color(0);

  free(fnode);

  return;
}

static struct
{
  int      dirty;
  int      broken;
  pdf_obj *annot_dict;
  pdf_rect rect;
} breaking_state = {0, 0, NULL, {0.0, 0.0, 0.0, 0.0}};

static void
reset_box (void)
{
  breaking_state.rect.llx = breaking_state.rect.lly =  HUGE_VAL;
  breaking_state.rect.urx = breaking_state.rect.ury = -HUGE_VAL;
  breaking_state.dirty    = 0;
}

void
pdf_doc_begin_annot (pdf_obj *dict)
{
  breaking_state.annot_dict = dict;
  breaking_state.broken = 0;
  reset_box();
}

void
pdf_doc_end_annot (void)
{
  pdf_doc_break_annot();
  breaking_state.annot_dict = NULL;
}

void
pdf_doc_break_annot (void)
{
  pdf_doc *p = &pdoc;
  double   g = p->opt.annot_grow;

  if (breaking_state.dirty) {
    pdf_obj  *annot_dict;
    pdf_rect  rect;

    /* Copy dict */
    annot_dict = pdf_new_dict();
    pdf_merge_dict(annot_dict, breaking_state.annot_dict);
    rect = breaking_state.rect;
    rect.llx -= g;
    rect.lly -= g;
    rect.urx += g;
    rect.ury += g;
    pdf_doc_add_annot(pdf_doc_current_page_number(), &rect,
                      annot_dict, !breaking_state.broken);
    pdf_release_obj(annot_dict);

    breaking_state.broken = 1;
  }
  reset_box();
}

void
pdf_doc_expand_box (const pdf_rect *rect)
{
  breaking_state.rect.llx = MIN(breaking_state.rect.llx, rect->llx);
  breaking_state.rect.lly = MIN(breaking_state.rect.lly, rect->lly);
  breaking_state.rect.urx = MAX(breaking_state.rect.urx, rect->urx);
  breaking_state.rect.ury = MAX(breaking_state.rect.ury, rect->ury);
  breaking_state.dirty    = 1;
}
