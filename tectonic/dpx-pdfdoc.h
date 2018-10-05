/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _PDFDOC_H_
#define _PDFDOC_H_

#include "dpx-core.h"

#include <stdbool.h>

#include "dpx-pdfobj.h"
#include "dpx-pdfdev.h"

#define PDF_DOC_GRABBING_NEST_MAX 4

void     pdf_doc_set_verbose (int level);

void     pdf_open_document  (const char *filename,
                                    bool enable_encrypt,
                                    bool enable_object_stream,
                                    double media_width, double media_height,
                                    double annot_grow_amount,
                                    int bookmark_open_depth,
                                    int check_gotos);
void     pdf_close_document (void);


/* PDF document metadata */
void     pdf_doc_set_creator (const char *creator);


/* They just return PDF dictionary object.
 * Callers are completely responsible for doing right thing...
 */
pdf_obj *pdf_doc_get_dictionary (const char *category);
pdf_obj *pdf_doc_get_reference  (const char *category);

#define pdf_doc_page_tree() pdf_doc_get_dictionary("Pages")
#define pdf_doc_catalog()   pdf_doc_get_dictionary("Catalog")
#define pdf_doc_docinfo()   pdf_doc_get_dictionary("Info")
#define pdf_doc_names()     pdf_doc_get_dictionary("Names")
#define pdf_doc_this_page() pdf_doc_get_dictionary("@THISPAGE")

int      pdf_doc_get_page_count (pdf_file *pf);
pdf_obj *pdf_doc_get_page (pdf_file *pf, int page_no, int options,
                           pdf_rect *bbox, pdf_tmatrix *matrix, pdf_obj **resources_p);

int      pdf_doc_current_page_number    (void);
pdf_obj *pdf_doc_current_page_resources (void);

pdf_obj *pdf_doc_ref_page (unsigned page_no);
#define pdf_doc_this_page_ref() pdf_doc_get_reference("@THISPAGE")
#define pdf_doc_next_page_ref() pdf_doc_get_reference("@NEXTPAGE")
#define pdf_doc_prev_page_ref() pdf_doc_get_reference("@PREVPAGE")

/* Not really managing tree...
 * There should be something for number tree.
 */
int      pdf_doc_add_names       (const char *category,
                                         const void *key, int keylen, pdf_obj *value);

void     pdf_doc_set_bop_content (const char *str, unsigned int length);
void     pdf_doc_set_eop_content (const char *str, unsigned int length);

/* Page */
void     pdf_doc_begin_page   (double scale, double x_origin, double y_origin);
void     pdf_doc_end_page     (void);

void     pdf_doc_set_mediabox (unsigned page_no, const pdf_rect *mediabox);

void     pdf_doc_add_page_content  (const char *buffer, unsigned int length);
void     pdf_doc_add_page_resource (const char *category,
                                           const char *resource_name, pdf_obj *resources);

/* Article thread */
void     pdf_doc_begin_article (const char *article_id, pdf_obj *info);
void     pdf_doc_add_bead      (const char *article_id,
                                       const char *bead_id,
                                       int page_no, const pdf_rect *rect);

/* Bookmarks */
int      pdf_doc_bookmarks_up    (void);
int      pdf_doc_bookmarks_down  (void);
void     pdf_doc_bookmarks_add   (pdf_obj *dict, int is_open);
int      pdf_doc_bookmarks_depth (void);


/* Returns xobj_id of started xform. */
int      pdf_doc_begin_grabbing (const char *ident,
                                        double ref_x, double ref_y,
                                        const pdf_rect *cropbox);
void     pdf_doc_end_grabbing   (pdf_obj *attrib);


/* Annotation */
void     pdf_doc_add_annot   (unsigned page_no,
                                     const pdf_rect *rect,
                                     pdf_obj *annot_dict,
                                     int new_annot);

/* Annotation with auto- clip and line (or page) break */
void     pdf_doc_begin_annot (pdf_obj *dict);
void     pdf_doc_end_annot   (void);

void     pdf_doc_break_annot (void);
void     pdf_doc_expand_box  (const pdf_rect *rect);

/* Manual thumbnail */
void     pdf_doc_enable_manual_thumbnails (void);

/* Similar to bop_content */
#include "dpx-pdfcolor.h"
void     pdf_doc_set_bgcolor   (const pdf_color *color);

#endif /* _PDFDOC_H_ */
