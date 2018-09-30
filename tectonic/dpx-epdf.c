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

/*
 * Concatinating content streams are only supported for streams that only uses
 * single FlateDecode filter, i.e.,
 *
 *   /Filter /FlateDecode or /Filter [/FlateDecode]
 *
 * TrimBox, BleedBox, ArtBox, Rotate ...
 */

#include "dpx-epdf.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>

#include "dpx-error.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfximage.h"

static int  rect_equal       (pdf_obj *rect1, pdf_obj *rect2);
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
 * include any extra bleed area needed to accommodate the physical
 * limitations of cutting, folding, and trimming equipment. The actual printed
 * page may include printing marks that fall outside the bleed box.
 * The default value is the page's crop box.
 *
 * TrimBox rectangle (Optional; PDF 1.3)
 *
 * The trim box (PDF 1.3) defines the intended dimensions of the finished page
 * after trimming. It may be smaller than the media box, to allow for
 * production-related content such as printing instructions, cut marks, or
 * color bars. The default value is the pageâ€™s crop box.
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

static int
rect_equal (pdf_obj *rect1, pdf_obj *rect2)
{
  int i;

  if (!rect1 || !rect2)
    return 0;
  for (i = 0; i < 4; i++) {
    if (pdf_number_value(pdf_get_array(rect1, i)) !=
        pdf_number_value(pdf_get_array(rect2, i)))
      return 0;
  }

  return 1;
}

static pdf_obj*
pdf_get_page_obj (pdf_file *pf, int page_no,
                  pdf_obj **ret_bbox, pdf_obj **ret_resources)
{
  pdf_obj *page_tree;
  pdf_obj *bbox = NULL, *resources = NULL, *rotate = NULL;
  int page_idx;

  /*
   * Get Page Tree.
   */
  page_tree = NULL;
  {
    pdf_obj *trailer, *catalog;
    pdf_obj *markinfo, *tmp;

    trailer = pdf_file_get_trailer(pf);

    if (pdf_lookup_dict(trailer, "Encrypt")) {
      dpx_warning("This PDF document is encrypted.");
      pdf_release_obj(trailer);
      return NULL;
    }

    catalog = pdf_deref_obj(pdf_lookup_dict(trailer, "Root"));
    if (!PDF_OBJ_DICTTYPE(catalog)) {
      dpx_warning("Can't read document catalog.");
      pdf_release_obj(trailer);
      pdf_release_obj(catalog);
      return NULL;
    }
    pdf_release_obj(trailer);

    markinfo = pdf_deref_obj(pdf_lookup_dict(catalog, "MarkInfo"));
    if (markinfo) {
      tmp = pdf_lookup_dict(markinfo, "Marked");
      if (PDF_OBJ_BOOLEANTYPE(tmp) && pdf_boolean_value(tmp))
        dpx_warning("PDF file is tagged... Ignoring tags.");
      pdf_release_obj(markinfo);
    }

    page_tree = pdf_deref_obj(pdf_lookup_dict(catalog, "Pages"));
    pdf_release_obj(catalog);
  }
  if (!page_tree) {
    dpx_warning("Page tree not found.");
    return NULL;
  }

  /*
   * Negative page numbers are counted from the back.
   */
  {
    int count = pdf_number_value(pdf_lookup_dict(page_tree, "Count"));
    page_idx = page_no + (page_no >= 0 ? -1 : count);
    if (page_idx < 0 || page_idx >= count) {
        dpx_warning("Page %d does not exist.", page_no);
        pdf_release_obj(page_tree);
        return NULL;
      }
    page_no = page_idx+1;
  }

  /*
   * Seek correct page. Get Media/Crop Box.
   * Media box and resources can be inherited.
   */
  {
    pdf_obj *kids_ref, *kids;
    pdf_obj *crop_box = NULL;
    pdf_obj *tmp;

    tmp = pdf_lookup_dict(page_tree, "Resources");
    resources = tmp ? pdf_deref_obj(tmp) : pdf_new_dict();

    while (1) {
      int kids_length, i;

      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "MediaBox")))) {
        pdf_release_obj(bbox);
        bbox = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "BleedBox")))) {
        if (!rect_equal(tmp, bbox)) {
          pdf_release_obj(bbox);
          bbox = tmp;
        } else {
          pdf_release_obj(tmp);
      }
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "TrimBox")))) {
        if (!rect_equal(tmp, bbox)) {
          pdf_release_obj(bbox);
          bbox = tmp;
        } else {
          pdf_release_obj(tmp);
      }
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "ArtBox")))) {
        if (!rect_equal(tmp, bbox)) {
          pdf_release_obj(bbox);
          bbox = tmp;
        } else {
          pdf_release_obj(tmp);
      }
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "CropBox")))) {
        pdf_release_obj(crop_box);
        crop_box = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "Rotate")))) {
        pdf_release_obj(rotate);
        rotate = tmp;
      }
      if ((tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "Resources")))) {
        pdf_release_obj(resources);
        resources = tmp;
      }

      kids_ref = pdf_lookup_dict(page_tree, "Kids");
      if (!kids_ref)
        break;
      kids = pdf_deref_obj(kids_ref);
      kids_length = pdf_array_length(kids);

      for (i = 0; i < kids_length; i++) {
        int count;

        pdf_release_obj(page_tree);
        page_tree = pdf_deref_obj(pdf_get_array(kids, i));

        tmp = pdf_deref_obj(pdf_lookup_dict(page_tree, "Count"));
        if (tmp) {
          /* Pages object */
          count = pdf_number_value(tmp);
          pdf_release_obj(tmp);
        } else {
          /* Page object */
          count = 1;
        }
        if (page_idx < count)
          break;

        page_idx -= count;
      }

      pdf_release_obj(kids);

      if (i == kids_length) {
        dpx_warning("Page %d not found! Broken PDF file?", page_no);
        pdf_release_obj(bbox);
        pdf_release_obj(crop_box);
        pdf_release_obj(rotate);
        pdf_release_obj(resources);
        pdf_release_obj(page_tree);
        return NULL;
      }
    }
    if (crop_box) {
      pdf_release_obj(bbox);
      bbox = crop_box;
    }
  }

  if (!bbox) {
    dpx_warning("No BoundingBox information available.");
    pdf_release_obj(page_tree);
    pdf_release_obj(resources);
    pdf_release_obj(rotate);
    return NULL;
  }

  if (rotate) {
    pdf_release_obj(rotate);
    rotate = NULL;
  }

  if (ret_bbox != NULL)
    *ret_bbox = bbox;
  if (ret_resources != NULL)
    *ret_resources = resources;

  return page_tree;
}

static pdf_obj*
pdf_get_page_content (pdf_obj* page)
{
  pdf_obj *contents, *content_new;

  contents = pdf_deref_obj(pdf_lookup_dict(page, "Contents"));
  if (!contents)
    return NULL;

  if (pdf_obj_typeof(contents) == PDF_NULL) {
    /* empty page */
    pdf_release_obj(contents);
    /* TODO: better don't include anything if the page is empty */
    contents = pdf_new_stream(0);
  } else if (PDF_OBJ_ARRAYTYPE(contents)) {
    /*
     * Concatenate all content streams.
     */
    pdf_obj *content_seg;
    int      idx = 0;
    content_new = pdf_new_stream(STREAM_COMPRESS);
    for (;;) {
      content_seg = pdf_deref_obj(pdf_get_array(contents, idx));
      if (!content_seg)
        break;
      else if (PDF_OBJ_NULLTYPE(content_seg)) {
        /* Silently ignore. */
      }  else if (!PDF_OBJ_STREAMTYPE(content_seg)) {
        dpx_warning("Page content not a stream object. Broken PDF file?");
        pdf_release_obj(content_seg);
        pdf_release_obj(content_new);
        pdf_release_obj(contents);
        return NULL;
      } else if (pdf_concat_stream(content_new, content_seg) < 0) {
        dpx_warning("Could not handle content stream with multiple segments.");
        pdf_release_obj(content_seg);
        pdf_release_obj(content_new);
        pdf_release_obj(contents);
        return NULL;
      }
      pdf_release_obj(content_seg);
      idx++;
    }
    pdf_release_obj(contents);
    contents = content_new;
  } else {
    if (!PDF_OBJ_STREAMTYPE(contents)) {
      dpx_warning("Page content not a stream object. Broken PDF file?");
      pdf_release_obj(contents);
      return NULL;
    }
    /* Flate the contents if necessary. */
    content_new = pdf_new_stream(STREAM_COMPRESS);
    if (pdf_concat_stream(content_new, contents) < 0) {
      dpx_warning("Could not handle a content stream.");
      pdf_release_obj(contents);
      pdf_release_obj(content_new);
      return NULL;
    }
    pdf_release_obj(contents);
    contents = content_new;
  }

  return contents;
}

/* ximage here is the result. DONT USE IT FOR PASSING OPTIONS! */
int
pdf_include_page (pdf_ximage        *ximage,
                  rust_input_handle_t handle,
                  const char        *ident,
                  load_options       options)
{
  pdf_file *pf;
  xform_info info;
  pdf_obj *contents = NULL, *catalog;
  pdf_obj *page = NULL, *resources = NULL, *markinfo = NULL;

  pf = pdf_open(ident, handle);
  if (!pf)
    return -1;

  if (pdf_file_get_version(pf) > pdf_get_version()) {
    dpx_warning("Trying to include PDF file which has newer version number " \
         "than output PDF: 1.%d.", pdf_get_version());
  }

  pdf_ximage_init_form_info(&info);

  if (options.page_no == 0)
    options.page_no = 1;
  page = pdf_doc_get_page(pf,
                          options.page_no, options.bbox_type,
                          &info.bbox, &info.matrix, &resources);

  if(!page)
    goto error_silent;

  catalog = pdf_file_get_catalog(pf);
  markinfo = pdf_deref_obj(pdf_lookup_dict(catalog, "MarkInfo"));
  if (markinfo) {
    pdf_obj *tmp = pdf_deref_obj(pdf_lookup_dict(markinfo, "Marked"));
    pdf_release_obj(markinfo);
    if (!PDF_OBJ_BOOLEANTYPE(tmp)) {
      pdf_release_obj(tmp);
      goto error;
    } else if (pdf_boolean_value(tmp)) {
      dpx_warning("PDF file is tagged... Ignoring tags.");
    }
    pdf_release_obj(tmp);
  }

  contents = pdf_deref_obj(pdf_lookup_dict(page, "Contents"));
  pdf_release_obj(page);
  page = NULL;

  /*
   * Handle page content stream.
   */
  {
    pdf_obj *content_new;

    if (!contents) {
      /*
       * Empty page
       */
      content_new = pdf_new_stream(0);
      /* TODO: better don't include anything if the page is empty */
    } else if (PDF_OBJ_STREAMTYPE(contents)) {
      /*
       * We must import the stream because its dictionary
       * may contain indirect references.
       */
      content_new = pdf_import_object(contents);
    } else if (PDF_OBJ_ARRAYTYPE(contents)) {
      /*
       * Concatenate all content streams.
       */
      int idx, len = pdf_array_length(contents);
      content_new = pdf_new_stream(STREAM_COMPRESS);
      for (idx = 0; idx < len; idx++) {
        pdf_obj *content_seg = pdf_deref_obj(pdf_get_array(contents, idx));
        if (!PDF_OBJ_STREAMTYPE(content_seg) ||
            pdf_concat_stream(content_new, content_seg) < 0) {
          pdf_release_obj(content_seg);
          pdf_release_obj(content_new);
          goto error;
        }
        pdf_release_obj(content_seg);
      }
    } else {
      goto error;
    }

    pdf_release_obj(contents);
    contents = content_new;
  }

  /*
   * Add entries to contents stream dictionary.
   */
  {
    pdf_obj *contents_dict, *bbox, *matrix;

    contents_dict = pdf_stream_dict(contents);
    pdf_add_dict(contents_dict,
                 pdf_new_name("Type"), pdf_new_name("XObject"));
    pdf_add_dict(contents_dict,
                 pdf_new_name("Subtype"), pdf_new_name("Form"));
    pdf_add_dict(contents_dict,
                 pdf_new_name("FormType"), pdf_new_number(1.0));

    bbox = pdf_new_array();
    pdf_add_array(bbox, pdf_new_number(info.bbox.llx));
    pdf_add_array(bbox, pdf_new_number(info.bbox.lly));
    pdf_add_array(bbox, pdf_new_number(info.bbox.urx));
    pdf_add_array(bbox, pdf_new_number(info.bbox.ury));

    pdf_add_dict(contents_dict, pdf_new_name("BBox"), bbox);

    matrix = pdf_new_array();
    pdf_add_array(matrix, pdf_new_number(info.matrix.a));
    pdf_add_array(matrix, pdf_new_number(info.matrix.b));
    pdf_add_array(matrix, pdf_new_number(info.matrix.c));
    pdf_add_array(matrix, pdf_new_number(info.matrix.d));
    pdf_add_array(matrix, pdf_new_number(info.matrix.e));
    pdf_add_array(matrix, pdf_new_number(info.matrix.f));

    pdf_add_dict(contents_dict, pdf_new_name("Matrix"), matrix);

    pdf_add_dict(contents_dict, pdf_new_name("Resources"),
                 pdf_import_object(resources));
    pdf_release_obj(resources);
  }

  pdf_close(pf);

  pdf_ximage_set_form(ximage, &info, contents);

  return 0;

 error:
  dpx_warning("Cannot parse document. Broken PDF file?");
 error_silent:
  pdf_release_obj(resources);
  pdf_release_obj(markinfo);
  pdf_release_obj(page);
  pdf_release_obj(contents);

  pdf_close(pf);

  return -1;
}

typedef enum {
  OP_SETCOLOR          = 1,
  OP_CLOSEandCLIP      = 2,
  OP_CLIP              = 3,
  OP_CONCATMATRIX      = 4,
  OP_SETCOLORSPACE     = 5,
  OP_RECTANGLE         = 6,
  OP_CURVETO           = 7,
  OP_CLOSEPATH         = 8,
  OP_LINETO            = 9,
  OP_MOVETO            = 10,
  OP_NOOP              = 11,
  OP_GSAVE             = 12,
  OP_GRESTORE          = 13,
  OP_CURVETO1          = 14,
  OP_CURVETO2          = 15,
  OP_UNKNOWN           = 16
} pdf_opcode;

static struct operator
{
  const char *token;
  int         opcode;
} pdf_operators[] = {
  {"SCN",       OP_SETCOLOR},
  {"b*",        OP_CLOSEandCLIP},
  {"B*",        OP_CLIP},
  {"cm",        OP_CONCATMATRIX},
  {"CS",        OP_SETCOLORSPACE},
  {"f*",        0},
  {"gs",        -1},
  {"re",        OP_RECTANGLE},
  {"rg",        -3},
  {"RG",        -3},
  {"sc",        OP_SETCOLOR},
  {"SC",        OP_SETCOLOR},
  {"W*",        OP_CLIP},
  {"b",         OP_CLOSEandCLIP},
  {"B",         OP_CLIP},
  {"c",         OP_CURVETO},
  {"d",         -2},
  {"f",         0},
  {"F",         0},
  {"g",         -1},
  {"G",         -1},
  {"h",         OP_CLOSEPATH},
  {"i",         -1},
  {"j",         -1},
  {"J",         -1},
  {"k",         -4},
  {"K",         -4},
  {"l",         OP_LINETO},
  {"m",         OP_MOVETO},
  {"M",         -1},
  {"n",         OP_NOOP},
  {"q",         OP_GSAVE},
  {"Q",         OP_GRESTORE},
  {"s",         OP_CLOSEandCLIP},
  {"S",         OP_CLIP},
  {"v",         OP_CURVETO1},
  {"w",         -1},
  {"W",         OP_CLIP},
  {"y",         OP_CURVETO2}
};


int
pdf_copy_clip (FILE *image_file, int pageNo, double x_user, double y_user)
{
  pdf_obj *page_tree, *contents;
  int depth = 0, top = -1;
  const char *clip_path, *end_path;
  char *save_path, *temp;
  pdf_tmatrix M;
  double stack[6];
  pdf_file *pf;

  pf = pdf_open(NULL, image_file);
  if (!pf)
    return -1;

  pdf_dev_currentmatrix(&M);
  pdf_invertmatrix(&M);
  M.e += x_user; M.f += y_user;
  page_tree = pdf_get_page_obj (pf, pageNo, NULL, NULL);
  if (!page_tree) {
    pdf_close(pf);
    return -1;
  }

  contents = pdf_get_page_content(page_tree);
  pdf_release_obj(page_tree);
  if (!contents) {
    pdf_close(pf);
    return -1;
  }

  pdf_doc_add_page_content(" ", 1);

  save_path = xmalloc(pdf_stream_length(contents) + 1);
  strncpy(save_path, (const char *) pdf_stream_dataptr(contents),  pdf_stream_length(contents));
  clip_path = save_path;
  end_path = clip_path + pdf_stream_length(contents);
  depth = 0;

  for (; clip_path < end_path; clip_path++) {
    int color_dimen = 0; /* silence uninitialized warning */
    char *token;
    skip_white(&clip_path, end_path);
    if (clip_path == end_path)
      break;
    if (depth > 1) {
      if (*clip_path == 'q')
        depth++;
      if (*clip_path == 'Q')
        depth--;
      parse_ident(&clip_path, end_path);
      continue;
    } else if (*clip_path == '-'
            || *clip_path == '+'
            || *clip_path == '.'
            || isdigit((unsigned char)*clip_path)) {
      stack[++top] = strtod(clip_path, &temp);
      clip_path = temp;
    } else if (*clip_path == '[') {
      /* Ignore, but put a dummy value on the stack (in case of d operator) */
      parse_pdf_array(&clip_path, end_path, pf);
      stack[++top] = 0;
    } else if (*clip_path == '/') {
      if  (strncmp("/DeviceGray", clip_path, 11) == 0
        || strncmp("/Indexed",    clip_path, 8)  == 0
        || strncmp("/CalGray",    clip_path, 8)  == 0) {
        color_dimen = 1;
        continue;
      }
      else if  (strncmp("/DeviceRGB", clip_path, 10) == 0
        || strncmp("/CalRGB",         clip_path, 7)  == 0
        || strncmp("/Lab",            clip_path, 4)  == 0) {
        color_dimen = 3;
        continue;
      }
      else if  (strncmp("/DeviceCMYK", clip_path, 11) == 0) {
        color_dimen = 4;
        continue;
      }
      else {
        clip_path++;
        parse_ident(&clip_path, end_path);
        skip_white(&clip_path, end_path);
        token = parse_ident(&clip_path, end_path);
        if (streq_ptr(token, "gs")) {
          continue;
        }
        return -1;
      }
    } else {
      unsigned int j;
      pdf_tmatrix T;
      pdf_coord  p0, p1, p2, p3;

      token = parse_ident(&clip_path, end_path);
      for (j = 0; j < sizeof(pdf_operators) / sizeof(pdf_operators[0]); j++)
        if (streq_ptr(token, pdf_operators[j].token))
          break;
      if (j == sizeof(pdf_operators) / sizeof(pdf_operators[0])) {
        return -1;
      }
      switch (pdf_operators[j].opcode) {
        case  0:
        case -1:
        case -2:
        case -3:
        case -4:
          /* Just pop the stack and do nothing. */
          top += pdf_operators[j].opcode;
          if (top < -1)
            return -1;
          break;
        case OP_SETCOLOR:
          top -= color_dimen;
          if (top < -1)
            return -1;
          break;
        case OP_CLOSEandCLIP:
          pdf_dev_closepath();
        case OP_CLIP:
          pdf_dev_flushpath('W', PDF_FILL_RULE_NONZERO);
          break;
        case OP_CONCATMATRIX:
          if (top < 5)
            return -1;
          T.f = stack[top--];
          T.e = stack[top--];
          T.d = stack[top--];
          T.c = stack[top--];
          T.b = stack[top--];
          T.a = stack[top--];
          pdf_concatmatrix(&M, &T);
          break;
        case OP_SETCOLORSPACE:
          /* Do nothing. */
          break;
        case OP_RECTANGLE:
          if (top < 3)
            return -1;
          p1.y = stack[top--];
          p1.x = stack[top--];
          p0.y = stack[top--];
          p0.x = stack[top--];
          if (M.b == 0 && M.c == 0) {
            pdf_tmatrix M0;
            M0.a = M.a; M0.b = M.b; M0.c = M.c; M0.d = M.d;
            M0.e = 0; M0.f = 0;
            pdf_dev_transform(&p0, &M);
            pdf_dev_transform(&p1, &M0);
            pdf_dev_rectadd(p0.x, p0.y, p1.x, p1.y);
          } else {
            p2.x = p0.x + p1.x; p2.y = p0.y + p1.y;
            p3.x = p0.x; p3.y = p0.y + p1.y;
            p1.x += p0.x; p1.y = p0.y;
            pdf_dev_transform(&p0, &M);
            pdf_dev_transform(&p1, &M);
            pdf_dev_transform(&p2, &M);
            pdf_dev_transform(&p3, &M);
            pdf_dev_moveto(p0.x, p0.y);
            pdf_dev_lineto(p1.x, p1.y);
            pdf_dev_lineto(p2.x, p2.y);
            pdf_dev_lineto(p3.x, p3.y);
            pdf_dev_closepath();
          }
          break;
        case OP_CURVETO:
          if (top < 5)
            return -1;
          p0.y = stack[top--];
          p0.x = stack[top--];
          pdf_dev_transform(&p0, &M);
          p1.y = stack[top--];
          p1.x = stack[top--];
          pdf_dev_transform(&p1, &M);
          p2.y = stack[top--];
          p2.x = stack[top--];
          pdf_dev_transform(&p2, &M);
          pdf_dev_curveto(p2.x, p2.y, p1.x, p1.y, p0.x, p0.y);
          break;
        case OP_CLOSEPATH:
          pdf_dev_closepath();
          break;
        case OP_LINETO:
          if (top < 1)
            return -1;
          p0.y = stack[top--];
          p0.x = stack[top--];
          pdf_dev_transform(&p0, &M);
          pdf_dev_lineto(p0.x, p0.y);
          break;
        case OP_MOVETO:
          if (top < 1)
            return -1;
          p0.y = stack[top--];
          p0.x = stack[top--];
          pdf_dev_transform(&p0, &M);
          pdf_dev_moveto(p0.x, p0.y);
          break;
        case OP_NOOP:
          pdf_doc_add_page_content(" n", 2);
          break;
        case OP_GSAVE:
          depth++;
          break;
        case OP_GRESTORE:
          depth--;
          break;
        case OP_CURVETO1:
          if (top < 3)
            return -1;
          p0.y = stack[top--];
          p0.x = stack[top--];
          pdf_dev_transform(&p0, &M);
          p1.y = stack[top--];
          p1.x = stack[top--];
          pdf_dev_transform(&p1, &M);
          pdf_dev_vcurveto(p1.x, p1.y, p0.x, p0.y);
          break;
        case OP_CURVETO2:
          if (top < 3)
            return -1;
          p0.y = stack[top--];
          p0.x = stack[top--];
          pdf_dev_transform(&p0, &M);
          p1.y = stack[top--];
          p1.x = stack[top--];
          pdf_dev_transform(&p1, &M);
          pdf_dev_ycurveto(p1.x, p1.y, p0.x, p0.y);
          break;
        default:
          return -1;
      }
    }
  }
  free(save_path);

  pdf_release_obj(contents);
  pdf_close(pf);

  return 0;
}
