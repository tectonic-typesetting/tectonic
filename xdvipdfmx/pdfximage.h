/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _PDFXIMAGE_H_
#define _PDFXIMAGE_H_

#include "pdfdev.h"

#define PDF_XOBJECT_TYPE_FORM  0
#define PDF_XOBJECT_TYPE_IMAGE 1

typedef struct {
  int  flags;

  int  width;
  int  height;

  int  bits_per_component;
  int  num_components;

  int  min_dpi; /* NOT USED YET */

  double xdensity, ydensity; /* scale factor for bp */
} ximage_info;

typedef struct {
  int         flags;
  
  pdf_rect    bbox;
  pdf_tmatrix matrix;
} xform_info;

typedef struct {
  int      page_no;
  int      bbox_type;
  pdf_obj *dict;
} load_options;

typedef struct pdf_ximage_ pdf_ximage;

extern void     pdf_ximage_set_verbose    (void);

extern void     pdf_init_images           (void);
extern void     pdf_close_images          (void);

extern char    *pdf_ximage_get_resname    (int xobj_id);
extern pdf_obj *pdf_ximage_get_reference  (int xobj_id);

/* Please use different interface than findresource...
 * This is not intended to be used for specifying page number and others.
 * Only pdf:image special in spc_pdfm.c want optinal dict!
 */
extern int      pdf_ximage_findresource   (const char  *ident,
                                           load_options options);
extern int      pdf_ximage_defineresource (const char *ident, int subtype,
                                           void *cdata, pdf_obj *resource);

/* Called by pngimage, jpegimage, epdf, mpost, etc. */
extern void pdf_ximage_init_image_info (ximage_info *info);
extern void pdf_ximage_init_form_info  (xform_info  *info);
extern void pdf_ximage_set_image (pdf_ximage *ximage,
                                  void *info, pdf_obj *resource);
extern void pdf_ximage_set_form  (pdf_ximage *ximage,
                                  void *info, pdf_obj *resource);
extern int  pdf_ximage_get_page  (pdf_ximage *I);

/* from pdfximage.c */
extern void set_distiller_template (char *s);
extern char *get_distiller_template (void);

extern int
pdf_ximage_scale_image (int            id,
                        pdf_tmatrix    *M, /* ret */
                        pdf_rect       *r, /* ret */
                        transform_info *p  /* arg */
                       );

/* from spc_pdfm.c */
extern int      pdf_ximage_get_subtype    (int xobj_id);
extern void
pdf_ximage_set_attr (int xobj_id,
                     int width, int height, double xdensity, double ydensity,
                     double llx, double lly, double urx, double ury);

/* Migrated from pdfobj.h. Those are not PDF object related... */
#define MAX_IMAGES 5000 /* This may be enough */

#endif /* _PDFXIMAGE_H_ */
