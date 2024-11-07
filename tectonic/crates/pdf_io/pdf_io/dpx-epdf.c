/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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
#include "dpx-dpxutil.h"

static pdf_obj*
get_page_content (pdf_file *pf, pdf_obj* page)
{
  pdf_obj *contents, *content_new;

  contents = pdf_deref_obj(pdf_lookup_dict(page, "Contents"));
  if (!contents)
    return NULL;

  if (PDF_OBJ_NULLTYPE(contents)) {
    /* empty page */
    pdf_release_obj(contents);
    /* TODO: better don't include anything if the page is empty */
    contents = pdf_new_stream(0);
  } else if (PDF_OBJ_ARRAYTYPE(contents)) {
    /*
     * Concatenate all content streams.
     */
    pdf_obj *content_seg;
    int      i;
    content_new = pdf_new_stream(STREAM_COMPRESS);
    for (i = 0; i < pdf_array_length(contents); i++) {
      content_seg = pdf_deref_obj(pdf_get_array(contents, i));
      if (!content_seg) {
        dpx_warning("Could not read page content stream.");
             pdf_release_obj(content_new);
        return NULL;
      }
      if (PDF_OBJ_STREAMTYPE(content_seg)) {
        pdf_concat_stream(content_new, content_seg);
      } else if (!PDF_OBJ_NULLTYPE(content_seg)) {
        dpx_warning("Page content not a stream object. Broken PDF file?");
        pdf_release_obj(content_seg);
        pdf_release_obj(content_new);
        pdf_release_obj(contents);
        return NULL;
      }
      pdf_release_obj(content_seg);
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
    pdf_concat_stream(content_new, contents);
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

  /*
   * Handle page content stream.
   */
  contents = get_page_content(pf, page);
  pdf_release_obj(page);
  page = NULL;

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

enum action {
    action_unknown,
    action_discard,
    action_path,
    action_rect,
    action_trans,
    action_clip,
    action_save,
    action_restore
};

static struct operator
{
  const char *token;
  enum action action;
  int         n_args;
} operators[] = {
  {"b*", action_clip, 0},
  {"B*", action_clip, 0},
  {"cm", action_trans, 6},
  {"f*", action_clip, 0},
  {"re", action_rect, 4},
  {"W*", action_path, 0},
  {"b",  action_clip, 0},
  {"B",  action_clip, 0},
  {"c",  action_path, 6},
  {"f",  action_clip, 0},
  {"F",  action_clip, 0},
  {"h",  action_path, 0},
  {"l",  action_path, 2},
  {"m",  action_path, 2},
  {"n",  action_path, 0},
  {"q",  action_save, 0},
  {"Q",  action_restore, 0},
  {"s",  action_clip, 0},
  {"S",  action_clip, 0},
  {"v",  action_path, 4},
  {"W",  action_path, 0},
  {"y",  action_path, 4}
};

static int
get_numbers_from_stack (dpx_stack *stack, double *v, int n)
{
  int error = 0;
  int i;

  for (i = 0; i < n; i++) {
    pdf_obj *obj;
    obj = dpx_stack_pop(stack);
    if (!obj) {
      error = -1;
      break;
    } else if (!PDF_OBJ_NUMBERTYPE(obj)) {
      pdf_release_obj(obj);
      error = -1;
      break;
    }
    v[n-i-1] = pdf_number_value(obj);
    pdf_release_obj(obj);
  }
  return error;
}

int
pdf_copy_clip (rust_input_handle_t image_file, int pageNo, double x_user, double y_user)
{
  pdf_obj *page_tree, *contents;
  int depth = 0;
  const char *p, *endptr;
  pdf_tmatrix M;
  pdf_rect bbox;
  pdf_tmatrix mtrx;
  pdf_file *pf;
  dpx_stack stack;
  int error = 0;

  pf = pdf_open(NULL, image_file);
  if (!pf)
    return -1;

  pdf_dev_currentmatrix(&M);
  pdf_invertmatrix(&M);
  M.e += x_user; M.f += y_user;

  page_tree = pdf_doc_get_page(pf, pageNo, 0, &bbox, &mtrx, NULL);
  if (!page_tree) {
    pdf_close(pf);
    return -1;
  }
  contents = get_page_content(pf, page_tree);
  pdf_release_obj(page_tree);
  if (!contents) {
    pdf_close(pf);
    return -1;
  }

  pdf_doc_add_page_content(" ", 1);

  p      = pdf_stream_dataptr(contents);
  endptr = p + pdf_stream_length(contents);
  depth  = 0;
  dpx_stack_init(&stack);

  skip_white(&p, endptr);
  while (p < endptr && !error) {
    enum action  action = action_discard;
    char        *token  = NULL;
    pdf_obj     *obj    = NULL;
    int          n_args = 0;
    char         buf[1024];
    size_t       len = 0;

    if (depth > 1) {
      if (*p == 'q')
        depth++;
      if (*p == 'Q')
        depth--;
      token = parse_ident(&p, endptr);
      skip_white(&p, endptr);
      free(token);
      continue;
    }

    switch (*p) {
    case '-': case '+': case'.':
    case '0': case '1': case '2': case '3': case '4':
    case '5': case '6': case '7': case '8': case '9':
      obj = parse_pdf_number(&p, endptr);
      break;
    case '[':
      obj = parse_pdf_array(&p, endptr, NULL); /* No indirect reference allowed here */
      break;
    case '/':
      obj = parse_pdf_name(&p, endptr);
      break;
    case '(':
      obj = parse_pdf_string(&p, endptr);
      break;
    case '<':
      if (p < endptr - 1 && p[1] == '<') {
        obj = parse_pdf_dict(&p, endptr, NULL);
      } else {
        obj = parse_pdf_string(&p, endptr);
      }
      break;
    }
    if (obj) {
      skip_white(&p, endptr);
      dpx_stack_push(&stack, obj);
      continue;
    }

    /* operator */
    token = parse_ident(&p, endptr);
    skip_white(&p, endptr);
    if (!token) {
      break;
    } else {
      int i;
      for (i = 0; i < sizeof(operators) / sizeof(operators[0]); i++) {
        if (!strcmp(token, operators[i].token)) {
          action = operators[i].action;
          n_args = operators[i].n_args;
          break;
        }
      }
    }
    switch (action) {
    case action_rect:
      {
        double v[4];

        error = get_numbers_from_stack(&stack, v, n_args); /* n_args = 4 */
        if (!error) {
          /* Not sure if this switch is required */
          if (M.b == 0.0 && M.c == 0.0) {
            /* Use "re" operator */
            pdf_coord p0;
            double    w, h;

            p0.x = v[0]; p0.y = v[1];
            w = M.a * v[2]; h = M.d * v[3];
            pdf_dev_transform(&p0, &M);
            buf[len++] = ' ';
            len += pdf_sprint_coord(buf+len, &p0);
            buf[len++] = ' ';
            len += pdf_sprint_length(buf+len, w);
            buf[len++] = ' ';
            len += pdf_sprint_length(buf+len, h);
            len += sprintf(buf+len, " re");
          } else {
            /* Converted to lineto */
            pdf_coord p0, p1, p2, p3;
            double    w, h;

            w = v[2]; h = v[3];
            p0.x = v[0]; p0.y = v[1];
            p1.x = p0.x + w; p1.y = p0.y;
            p2.x = p1.x; p2.y = p1.y + h;
            p3.x = p0.x; p3.y = p2.y;
            pdf_dev_transform(&p0, &M);
            pdf_dev_transform(&p1, &M);
            pdf_dev_transform(&p2, &M);
            pdf_dev_transform(&p3, &M);
            buf[len++] = ' ';
            len += pdf_sprint_coord(buf+len, &p0);
            len += sprintf(buf+len, " m");
            buf[len++] = ' ';
            len += pdf_sprint_coord(buf+len, &p1);
            len += sprintf(buf+len, " l");
            buf[len++] = ' ';
            len += pdf_sprint_coord(buf+len, &p2);
            len += sprintf(buf+len, " l");
            buf[len++] = ' ';
            len += pdf_sprint_coord(buf+len, &p3);
            len += sprintf(buf+len, " l h");
          }
          pdf_doc_add_page_content(buf, len);
        }
      }
      break;
    case action_path:
      {
        double    v[6];
        pdf_coord pt;
        int       i;

        error = get_numbers_from_stack(&stack, v, n_args);
        if (!error) {
          for (i = 0; i < n_args/2; i++) {
            pt.x = v[2*i];
            pt.y = v[2*i+1];
            pdf_dev_transform(&pt, &M);
            buf[len++] = ' ';
            len += pdf_sprint_coord(buf+len, &pt);
          }
          len += sprintf(buf+len, " %s", token);
          pdf_doc_add_page_content(buf, len);
        }
      }
      break;
    case action_trans:
      {
        double      v[6];
        pdf_tmatrix T;
        error = get_numbers_from_stack(&stack, v, n_args);
        if (!error) {
          T.a = v[0]; T.b = v[1]; T.c = v[2]; T.d = v[3];
          T.e = v[4]; T.f = v[5];
          pdf_concatmatrix(&M, &T);
        }
      }
      break;
    case action_clip:
      if (token[0] >= 'a' && token[0] <= 'z') {
        /* close path */
        len += sprintf(buf+len, " h");
      }
      if (strlen(token) >= 2 && token[1] == '*') {
        len += sprintf(buf+len, " W* n");
      } else {
        len += sprintf(buf+len, " W n");
      }
      pdf_doc_add_page_content(buf, len);
      break;
    case action_save:
      depth++;
      break;
    case action_restore:
      depth--;
      break;
    case action_discard:
      /* stack clearing behavior */
      while ((obj = dpx_stack_pop(&stack)) != NULL) {
        pdf_release_obj(obj);
      }
      break;
    case action_unknown:
      error = -1;
      break;
    }
    free(token);
  }
  {
    pdf_obj *obj;
  
    while ((obj = dpx_stack_pop(&stack)) != NULL) {
      pdf_release_obj(obj);
    }
  }
  pdf_release_obj(contents);
  pdf_close(pf);

  return error;
}
