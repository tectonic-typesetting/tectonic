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

#include "dpx-mpost.h"

#include <ctype.h>
#include <math.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-subfont.h"
#include "dpx-tfm.h"

/*
 * Define the origin as (llx, lly) in order to
 * match the new xetex.def and dvipdfmx.def
 */

static double Xorigin, Yorigin;
static int    translate_origin = 0;

void
mps_set_translate_origin (int v) {
  translate_origin = v;
}

/*
 * In PDF, current path is not a part of graphics state parameter.
 * Hence, current path is not saved by the "q" operator  and is not
 * recovered by the "Q" operator. This means that the following PS
 * code
 *
 *   <path construction> gsave <path painting> grestore ...
 *
 * can't be translated to PDF code
 *
 *   <path construction> q <path painting> Q ...
 *
 * . Only clipping path (which is graphics state parameter in PDF
 * too) is treated in the same way. So, we write clipping path
 * immediately and forget about it but remember current path.
 */

static int mp_parse_body (const char **start, const char *end, double x_user, double y_user);

static struct mp_font
{
  char   *font_name;
  int     font_id;
  int     tfm_id;     /* Used for text width calculation */
  int     subfont_id;
  double  pt_size;
} font_stack[PDF_GSAVE_MAX] = {
  {NULL, -1, -1, -1, 0}
};
static int currentfont = 0;

#define CURRENT_FONT() ((currentfont < 0) ? NULL : &font_stack[currentfont])
#define FONT_DEFINED(f) ((f) && (f)->font_name && ((f)->font_id >= 0))

static void
clear_mp_font_struct (struct mp_font *font)
{
  assert(font);

  if (font->font_name)
    free(font->font_name);
  font->font_name  = NULL;
  font->font_id    = -1;
  font->tfm_id     = -1;
  font->subfont_id = -1;
  font->pt_size    = 0.0;
}

/* Compatibility */
#define MP_CMODE_MPOST    0
#define MP_CMODE_DVIPSK   1
#define MP_CMODE_PTEXVERT 2
static int mp_cmode = MP_CMODE_MPOST;

static int
mp_setfont (const char *font_name, double pt_size)
{
  const char     *name = font_name;
  struct mp_font *font;
  int             subfont_id = -1;
  fontmap_rec    *mrec;

  font = CURRENT_FONT();

  mrec = pdf_lookup_fontmap_record(font_name);
  if (mrec && mrec->charmap.sfd_name && mrec->charmap.subfont_id) {
    subfont_id = sfd_load_record(mrec->charmap.sfd_name, mrec->charmap.subfont_id);
  }

  /* See comments in dvi_locate_font() in dvi.c. */
  if (mrec && mrec->map_name) {
    name = mrec->map_name;
  } else {
    name = font_name;
  }

  free(font->font_name);
  font->font_name  = NEW(strlen(font_name) + 1, char);
  strcpy(font->font_name, font_name);
  font->subfont_id = subfont_id;
  font->pt_size    = pt_size;
  font->tfm_id     = tfm_open(font_name, 0); /* Need not exist in MP mode */
  font->font_id    = pdf_dev_locate_font(name,
                                         (spt_t) (pt_size * dev_unit_dviunit()));

  if (font->font_id < 0) {
    _tt_abort("MPOST: No physical font assigned for \"%s\".", font_name);
  }

  return  0;
}

static void
save_font (void)
{
  struct mp_font *current, *next;

  current = &font_stack[currentfont++];
  next    = &font_stack[currentfont  ];
  if (FONT_DEFINED(current)) {
    next->font_name = NEW(strlen(current->font_name)+1, char);
    strcpy(next->font_name, current->font_name);
    next->font_id    = current->font_id;
    next->pt_size    = current->pt_size;
    next->subfont_id = current->subfont_id;
    next->tfm_id     = current->tfm_id;
  } else {
    next->font_name  = NULL;
    next->font_id    = -1;
    next->pt_size    = 0.0;
    next->subfont_id = -1;
    next->tfm_id     = -1;
  }
}

static void
restore_font (void)
{
  struct mp_font *current;

  current = CURRENT_FONT();
  if (current) {
    clear_mp_font_struct(current);
  }

  if (currentfont > 0) /* Tectonic safety fix */
    currentfont--;
}

static void
clear_fonts (void)
{
  while (currentfont >= 0) {
    clear_mp_font_struct(&font_stack[currentfont]);
    currentfont--;
  }

  currentfont = 0; /* Tectonic fix */
}

static bool
is_fontname (const char *token)
{
  fontmap_rec *mrec;

  mrec = pdf_lookup_fontmap_record(token);
  if (mrec)
    return true;

  return tfm_exists(token);
}

int
mps_scan_bbox (const char **pp, const char *endptr, pdf_rect *bbox)
{
  char  *number;
  double values[4];
  int    i;

  /* skip_white() skips lines starting '%'... */
  while (*pp < endptr && isspace((unsigned char)**pp))
    (*pp)++;

  /* Scan for bounding box record */
  while (*pp < endptr && **pp == '%') {
    if (*pp + 14 < endptr &&
        strstartswith(*pp, "%%BoundingBox:")) {

      *pp += 14;

      for (i = 0; i < 4; i++) {
        skip_white(pp, endptr);
        number = parse_number(pp, endptr);
        if (!number) {
          break;
        }
        values[i] = atof(number);
        free(number);
      }
      if (i < 4) {
        return -1;
      } else {
        /* The new xetex.def and dvipdfmx.def require bbox->llx = bbox->lly = 0.  */
        if (translate_origin) {
          bbox->llx = 0;
          bbox->lly = 0;
          bbox->urx = values[2] - values[0];
          bbox->ury = values[3] - values[1];

          Xorigin = (double)values[0];
          Yorigin = (double)values[1];
        } else {
          bbox->llx = values[0];
          bbox->lly = values[1];
          bbox->urx = values[2];
          bbox->ury = values[3];

          Xorigin = 0.0;
          Yorigin = 0.0;
        }
        return 0;
      }
    }
    pdfparse_skip_line (pp, endptr);
    while (*pp < endptr && isspace((unsigned char)**pp))
      (*pp)++;
  }

  return -1;
}

static void
skip_prolog (const char **start, const char *end)
{
  int   found_prolog = 0;
  const char *save;

  save = *start;
  while (*start < end) {
    if (**start != '%')
      skip_white(start, end);
    if (*start >= end)
      break;
    if (strstartswith(*start, "%%EndProlog")) {
      found_prolog = 1;
      pdfparse_skip_line(start, end);
      break;
    } else if (strstartswith(*start, "%%Page:")) {
      pdfparse_skip_line(start, end);
      break;
    }
    pdfparse_skip_line(start, end);
  }
  if (!found_prolog) {
    *start = save;
  }

  return;
}

/* PostScript Operators */

/* Avoid conflict with SET... from <wingdi.h>.  */
#undef SETLINECAP
#undef SETLINEJOIN
#undef SETMITERLIMIT

/* These conflict with Tectonic #defines of WEB constants */
#undef POP
#undef FILL
#undef DEF
#undef PUSH

#define ADD            1
#define SUB            2
#define MUL            3
#define DIV            4
#define NEG            5
#define TRUNCATE       6

#define CLEAR          10
#define EXCH           11
#define POP            12

#define NEWPATH        31
#define CLOSEPATH      32
#define MOVETO         33
#define RMOVETO        34
#define CURVETO        35
#define RCURVETO       36
#define LINETO         37
#define RLINETO        38
#define ARC            39
#define ARCN           40

#define FILL           41
#define STROKE         42
#define SHOW           43

#define CLIP           44
#define EOCLIP         45

#define SHOWPAGE       49

#define GSAVE          50
#define GRESTORE       51

#define CONCAT         52
#define SCALE          53
#define TRANSLATE      54
#define ROTATE         55

#define SETLINEWIDTH   60
#define SETDASH        61
#define SETLINECAP     62
#define SETLINEJOIN    63
#define SETMITERLIMIT  64

#define SETGRAY        70
#define SETRGBCOLOR    71
#define SETCMYKCOLOR   72

#define CURRENTPOINT   80
#define IDTRANSFORM    81
#define DTRANSFORM     82

#define FINDFONT       201
#define SCALEFONT      202
#define SETFONT        203
#define CURRENTFONT    204

#define STRINGWIDTH    210

#define DEF            999

#define FSHOW          1001
#define STEXFIG        1002
#define ETEXFIG        1003
#define HLW            1004
#define VLW            1005
#define RD             1006
#define B              1007

static struct operators
{
  const char *token;
  int         opcode;
} ps_operators[] = {
  {"add",          ADD},
  {"mul",          MUL},
  {"div",          DIV},
  {"neg",          NEG},
  {"sub",          SUB},
  {"truncate",     TRUNCATE},

  {"clear",        CLEAR},
  {"exch",         EXCH},
  {"pop",          POP},

  {"clip",         CLIP},
  {"eoclip",       EOCLIP},
  {"closepath",    CLOSEPATH},
  {"concat",       CONCAT},

  {"newpath",      NEWPATH},
  {"moveto",       MOVETO},
  {"rmoveto",      RMOVETO},
  {"lineto",       LINETO},
  {"rlineto",      RLINETO},
  {"curveto",      CURVETO},
  {"rcurveto",     RCURVETO},
  {"arc",          ARC},
  {"arcn",         ARCN},

  {"stroke",       STROKE},
  {"fill",         FILL},
  {"show",         SHOW},
  {"showpage",     SHOWPAGE},

  {"gsave",        GSAVE},
  {"grestore",     GRESTORE},
  {"translate",    TRANSLATE},
  {"rotate",       ROTATE},
  {"scale",        SCALE},

  {"setlinecap",    SETLINECAP},
  {"setlinejoin",   SETLINEJOIN},
  {"setlinewidth",  SETLINEWIDTH},
  {"setmiterlimit", SETMITERLIMIT},
  {"setdash",       SETDASH},

  {"setgray",      SETGRAY},
  {"setrgbcolor",  SETRGBCOLOR},
  {"setcmykcolor", SETCMYKCOLOR},

  {"currentpoint", CURRENTPOINT}, /* This is here for rotate support
                                     in graphics package-not MP support */
  {"dtransform",   DTRANSFORM},
  {"idtransform",  IDTRANSFORM},

  {"findfont",     FINDFONT},
  {"scalefont",    SCALEFONT},
  {"setfont",      SETFONT},
  {"currentfont",  CURRENTFONT},

  {"stringwidth",  STRINGWIDTH},

  {"def", DEF} /* not implemented yet; just work with mptopdf */
};

static struct operators mps_operators[] = {
  {"fshow",       FSHOW}, /* exch findfont exch scalefont setfont show */
  {"startTexFig", STEXFIG},
  {"endTexFig",   ETEXFIG},
  {"hlw",         HLW}, /* 0 dtransform exch truncate exch idtransform pop setlinewidth */
  {"vlw",         VLW}, /* 0 exch dtransform truncate idtransform pop setlinewidth pop */
  {"l",           LINETO},
  {"r",           RLINETO},
  {"c",           CURVETO},
  {"m",           MOVETO},
  {"p",           CLOSEPATH},
  {"n",           NEWPATH},
  {"C",           SETCMYKCOLOR},
  {"G",           SETGRAY},
  {"R",           SETRGBCOLOR},
  {"lj",          SETLINEJOIN},
  {"ml",          SETMITERLIMIT},
  {"lc",          SETLINECAP},
  {"S",           STROKE},
  {"F",           FILL},
  {"q",           GSAVE},
  {"Q",           GRESTORE},
  {"s",           SCALE},
  {"t",           CONCAT},
  {"sd",          SETDASH},
  {"rd",          RD}, /* [] 0 setdash */
  {"P",           SHOWPAGE},
  {"B",           B}, /* gsave fill grestore */
  {"W",           CLIP}
};

#define NUM_PS_OPERATORS  (sizeof(ps_operators)/sizeof(ps_operators[0]))
#define NUM_MPS_OPERATORS (sizeof(mps_operators)/sizeof(mps_operators[0]))
static int
get_opcode (const char *token)
{
  unsigned int i;

  for (i = 0; i < NUM_PS_OPERATORS; i++) {
    if (streq_ptr(token, ps_operators[i].token)) {
      return ps_operators[i].opcode;
    }
  }

  for (i = 0; i < NUM_MPS_OPERATORS; i++) {
    if (streq_ptr(token, mps_operators[i].token)) {
      return mps_operators[i].opcode;
    }
  }

  return -1;
}

#define PS_STACK_SIZE 1024

static pdf_obj *stack[PS_STACK_SIZE];
static unsigned int top_stack = 0;

#define POP_STACK()     ((top_stack > 0) ? stack[--top_stack] : NULL)
#define PUSH_STACK(o,e) { \
  if (top_stack < PS_STACK_SIZE) { \
    stack[top_stack++] = (o); \
  } else { \
    dpx_warning("PS stack overflow including MetaPost file or inline PS code"); \
    *(e) = 1; \
  } \
}

static int
do_exch (void)
{
  pdf_obj *tmp;

  if (top_stack < 2)
    return -1;

  tmp = stack[top_stack-1];
  stack[top_stack-1] = stack[top_stack-2];
  stack[top_stack-2] = tmp;

  return 0;
}

static int
do_clear (void)
{
  pdf_obj *tmp;

  while (top_stack > 0) {
    tmp = POP_STACK();
    pdf_release_obj(tmp);
  }

  return 0;
}

static int
pop_get_numbers (double *values, int count)
{
  pdf_obj *tmp;

  while (count-- > 0) {
    tmp = POP_STACK();
    if (!tmp) {
      dpx_warning("mpost: Stack underflow.");
      break;
    } else if (!PDF_OBJ_NUMBERTYPE(tmp)) {
      dpx_warning("mpost: Not a number!");
      pdf_release_obj(tmp);
      break;
    }
    values[count] = pdf_number_value(tmp);
    pdf_release_obj(tmp);
  }

  return (count + 1);
}

static int
cvr_array (pdf_obj *array, double *values, int count)
{
  if (!PDF_OBJ_ARRAYTYPE(array)) {
    dpx_warning("mpost: Not an array!");
  } else {
    pdf_obj *tmp;

    while (count-- > 0) {
      tmp = pdf_get_array(array, count);
      if (!PDF_OBJ_NUMBERTYPE(tmp)) {
        dpx_warning("mpost: Not a number!");
        break;
      }
      values[count] = pdf_number_value(tmp);
    }
  }
  pdf_release_obj(array);

  return (count + 1);
}

static bool
is_fontdict (pdf_obj *dict)
{
  pdf_obj *tmp;

  if (!PDF_OBJ_DICTTYPE(dict))
    return false;

  tmp = pdf_lookup_dict(dict, "Type");
  if (!tmp || !PDF_OBJ_NAMETYPE(tmp) ||
      strcmp(pdf_name_value(tmp), "Font")) {
    return false;
  }

  tmp = pdf_lookup_dict(dict, "FontName");
  if (!tmp || !PDF_OBJ_NAMETYPE(tmp)) {
    return false;
  }

  tmp = pdf_lookup_dict(dict, "FontScale");
  if (!tmp || !PDF_OBJ_NUMBERTYPE(tmp)) {
    return false;
  }

  return true;
}

static int
do_findfont (void)
{
  int error = 0;
  pdf_obj *font_dict, *font_name;

  font_name = POP_STACK();
  if (!font_name)
    return 1;
  else if (PDF_OBJ_STRINGTYPE(font_name) ||
           PDF_OBJ_NAMETYPE(font_name)) {
    /* Do not check the existence...
     * The reason for this is that we cannot locate PK font without
     * font scale.
     */
    font_dict = pdf_new_dict();
    pdf_add_dict(font_dict,
                 pdf_new_name("Type"), pdf_new_name("Font"));
    if (PDF_OBJ_STRINGTYPE(font_name)) {
      pdf_add_dict(font_dict,
                   pdf_new_name("FontName"),
                   pdf_new_name(pdf_string_value(font_name)));
      pdf_release_obj(font_name);
    } else {
      pdf_add_dict(font_dict,
                   pdf_new_name("FontName"), font_name);
    }
    pdf_add_dict(font_dict,
                 pdf_new_name("FontScale"), pdf_new_number(1.0));

    if (top_stack < PS_STACK_SIZE) {
      stack[top_stack++] = font_dict;
    } else {
      dpx_warning("PS stack overflow including MetaPost file or inline PS code");
      pdf_release_obj(font_dict);
      error = 1;
    }
  } else {
    error = 1;
  }

  return error;
}

static int
do_scalefont (void)
{
  int error = 0;
  pdf_obj *font_dict;
  pdf_obj *font_scale;
  double   scale;

  error = pop_get_numbers(&scale, 1);
  if (error)
    return error;

  font_dict = POP_STACK();
  if (!font_dict)
    error = 1;
  else if (is_fontdict(font_dict)) {
    font_scale  = pdf_lookup_dict(font_dict, "FontScale");
    pdf_set_number(font_scale, pdf_number_value(font_scale)*scale);
    if (top_stack < PS_STACK_SIZE) {
      stack[top_stack++] = font_dict;
    } else {
      dpx_warning("PS stack overflow including MetaPost file or inline PS code");
      pdf_release_obj(font_dict);
      error = 1;
    }
  } else {
    error = 1;
  }

  return error;
}

static int
do_setfont (void)
{
  int      error = 0;
  char    *font_name;
  double   font_scale;
  pdf_obj *font_dict;

  font_dict = POP_STACK();
  if (!is_fontdict(font_dict))
    error = 1;
  else {
    /* Subfont support prevent us from managing
     * font in a single place...
     */
    font_name  = pdf_name_value  (pdf_lookup_dict(font_dict, "FontName"));
    font_scale = pdf_number_value(pdf_lookup_dict(font_dict, "FontScale"));

    error = mp_setfont(font_name, font_scale);
  }
  pdf_release_obj(font_dict);

  return error;
}

/* Push dummy font dict onto PS stack */
static int
do_currentfont (void)
{
  int             error = 0;
  struct mp_font *font;
  pdf_obj        *font_dict;

  font = CURRENT_FONT();
  if (!FONT_DEFINED(font)) {
    dpx_warning("Currentfont undefined...");
    return 1;
  } else {
    font_dict = pdf_new_dict();
    pdf_add_dict(font_dict,
                 pdf_new_name("Type"),
                 pdf_new_name("Font"));
    pdf_add_dict(font_dict,
                 pdf_new_name("FontName"),
                 pdf_new_name(font->font_name));
    pdf_add_dict(font_dict,
                 pdf_new_name("FontScale"),
                 pdf_new_number(font->pt_size));
    if (top_stack < PS_STACK_SIZE) {
      stack[top_stack++] = font_dict;
    } else {
      dpx_warning("PS stack overflow...");
      pdf_release_obj(font_dict);
      error = 1;
    }
  }

  return error;
}

static int
do_show (void)
{
  struct mp_font *font;
  pdf_coord       cp;
  pdf_obj        *text_str;
  int             length;
  unsigned char  *strptr;
  double          text_width;

  font = CURRENT_FONT();
  if (!FONT_DEFINED(font)) {
    dpx_warning("Currentfont not set."); /* Should not be error... */
    return 1;
  }

  pdf_dev_currentpoint(&cp);

  text_str = POP_STACK();
  if (!PDF_OBJ_STRINGTYPE(text_str)) {
    pdf_release_obj(text_str);
    return 1;
  }
  if (font->font_id < 0) {
    dpx_warning("mpost: not set."); /* Should not be error... */
    pdf_release_obj(text_str);
    return 1;
  }

  strptr = pdf_string_value (text_str);
  length = pdf_string_length(text_str);

  if (font->tfm_id < 0) {
    dpx_warning("mpost: TFM not found for \"%s\".", font->font_name);
    dpx_warning("mpost: Text width not calculated...");
  }

  text_width = 0.0;
  if (font->subfont_id >= 0) {
    unsigned short  uch;
    unsigned char  *ustr;
    int      i;

    ustr = NEW(length * 2, unsigned char);
    for (i = 0; i < length; i++) {
      uch = lookup_sfd_record(font->subfont_id, strptr[i]);
      ustr[2*i  ] = uch >> 8;
      ustr[2*i+1] = uch & 0xff;
      if (font->tfm_id >= 0) {
        text_width += tfm_get_width(font->tfm_id, strptr[i]);
      }
    }
    text_width *= font->pt_size;

    pdf_dev_set_string((spt_t)(cp.x * dev_unit_dviunit()),
                       (spt_t)(cp.y * dev_unit_dviunit()),
                       ustr, length * 2,
                       (spt_t)(text_width*dev_unit_dviunit()),
                       font->font_id, 0);
    free(ustr);
  } else {
#define FWBASE ((double) (1<<20))
    if (font->tfm_id >= 0) {
      text_width = (double) tfm_string_width(font->tfm_id, strptr, length)/FWBASE;
      text_width *= font->pt_size;
    }
    pdf_dev_set_string((spt_t)(cp.x * dev_unit_dviunit()),
                       (spt_t)(cp.y * dev_unit_dviunit()),
                       strptr, length,
                       (spt_t)(text_width*dev_unit_dviunit()),
                       font->font_id, 0);
  }

  if (pdf_dev_get_font_wmode(font->font_id)) {
    pdf_dev_rmoveto(0.0, -text_width);
  } else {
    pdf_dev_rmoveto(text_width, 0.0);
  }

  graphics_mode();
  pdf_release_obj(text_str);

  return 0;
}

static int
do_mpost_bind_def (const char *ps_code, double x_user, double y_user)
{
  int   error = 0;
  const char *start, *end;

  start = ps_code;
  end   = start + strlen(start);

  error = mp_parse_body(&start, end, x_user, y_user);

  return error;
}

static int
do_texfig_operator (int opcode, double x_user, double y_user)
{
  static transform_info fig_p;
  static int in_tfig = 0;
  static int xobj_id = -1;
  static int count   = 0;
  double values[6];
  int    error = 0;

  switch (opcode) {
  case STEXFIG:
    error = pop_get_numbers(values, 6);
    if (!error) {
      double   dvi2pts;
      char     resname[256];

      transform_info_clear(&fig_p);
      dvi2pts = 1.0/dev_unit_dviunit();

      fig_p.width    =  values[0] * dvi2pts;
      fig_p.height   =  values[1] * dvi2pts;
      fig_p.bbox.llx =  values[2] * dvi2pts;
      fig_p.bbox.lly = -values[3] * dvi2pts;
      fig_p.bbox.urx =  values[4] * dvi2pts;
      fig_p.bbox.ury = -values[5] * dvi2pts;
      fig_p.flags   |= INFO_HAS_USER_BBOX;

      sprintf(resname, "__tf%d__", count);
      xobj_id = pdf_doc_begin_grabbing(resname,
                                       fig_p.bbox.llx, fig_p.bbox.ury, &fig_p.bbox);

      in_tfig = 1;
      count++;
    }
    break;
  case ETEXFIG:
    if (!in_tfig)
      _tt_abort("endTexFig without valid startTexFig!.");

    pdf_doc_end_grabbing(NULL);
    pdf_dev_put_image(xobj_id, &fig_p, x_user, y_user);
    in_tfig = 0;
    break;
  default:
    error = 1;
  }

  return error;
}

/*
 * buggy...
 */

/*
 * CTM(Current Transformation Matrix) means the transformation of User Space
 * to Device Space coordinates. Because DVIPDFMx does not know the resolution
 * of Device Space, we assume that the resolution is 1/1000.
 */
#define DEVICE_RESOLUTION 1000
static int
ps_dev_CTM (pdf_tmatrix *M)
{
  pdf_dev_currentmatrix(M);
  M->a *= DEVICE_RESOLUTION; M->b *= DEVICE_RESOLUTION;
  M->c *= DEVICE_RESOLUTION; M->d *= DEVICE_RESOLUTION;
  M->e *= DEVICE_RESOLUTION; M->f *= DEVICE_RESOLUTION;

  return 0;
}

/*
 * Again, the only piece that needs x_user and y_user is
 * that piece dealing with texfig.
 */
static int
do_operator (const char *token, double x_user, double y_user)
{
  int         error  = 0;
  int         opcode = 0;
  double      values[12];
  pdf_obj    *tmp = NULL;
  pdf_tmatrix matrix;
  pdf_coord   cp;
  pdf_color   color;

#define PUSH(o) { \
  if (top_stack < PS_STACK_SIZE) { \
    stack[top_stack++] = (o); \
  } else { \
    dpx_warning("PS stack overflow including MetaPost file or inline PS code"); \
    error=1; \
    break;\
  } \
}

  opcode = get_opcode(token);

  switch (opcode) {

    /*
     * Arithmetic operators
     */
  case ADD:
    error = pop_get_numbers(values, 2);
    if (!error)
      PUSH(pdf_new_number(values[0] + values[1]));
    break;
  case MUL:
    error = pop_get_numbers(values, 2);
    if (!error)
      PUSH(pdf_new_number(values[0]*values[1]));
    break;
  case NEG:
    error = pop_get_numbers(values, 1);
    if (!error)
      PUSH(pdf_new_number(-values[0]));
    break;
  case SUB:
    error = pop_get_numbers(values, 2);
    if (!error)
      PUSH(pdf_new_number(values[0] - values[1]));
    break;
  case DIV:
    error = pop_get_numbers(values, 2);
    if (!error)
      PUSH(pdf_new_number(values[0]/values[1]));
    break;
  case TRUNCATE: /* Round toward zero. */
    error = pop_get_numbers(values, 1);
    if (!error)
      PUSH(pdf_new_number(((values[0] > 0) ? floor(values[0]) : ceil(values[0]))));
    break;

    /* Stack operation */
  case CLEAR:
    error = do_clear();
    break;
  case POP:
    tmp = POP_STACK();
    pdf_release_obj(tmp);
    break;
  case EXCH:
    error = do_exch();
    break;

    /* Path construction */
  case MOVETO:
    error = pop_get_numbers(values, 2);
    if (!error)
      error = pdf_dev_moveto(values[0], values[1]);
    break;
  case RMOVETO:
    error = pop_get_numbers(values, 2);
    if (!error)
      error = pdf_dev_rmoveto(values[0], values[1]);
    break;
  case LINETO:
    error = pop_get_numbers(values, 2);
    if (!error)
      error = pdf_dev_lineto(values[0], values[1]);
    break;
  case RLINETO:
    error = pop_get_numbers(values, 2);
    if (!error)
      error = pdf_dev_rlineto(values[0], values[1]);
    break;
  case CURVETO:
    error = pop_get_numbers(values, 6);
    if (!error)
      error = pdf_dev_curveto(values[0], values[1],
                              values[2], values[3],
                              values[4], values[5]);
    break;
  case RCURVETO:
    error = pop_get_numbers(values, 6);
    if (!error)
      error = pdf_dev_rcurveto(values[0], values[1],
                               values[2], values[3],
                               values[4], values[5]);
    break;
  case CLOSEPATH:
    error = pdf_dev_closepath();
    break;
  case ARC:
    error = pop_get_numbers(values, 5);
    if (!error)
      error = pdf_dev_arc(values[0], values[1],
                          values[2], /* rad */
                          values[3], values[4]);
    break;
  case ARCN:
    error = pop_get_numbers(values, 5);
    if (!error)
      error = pdf_dev_arcn(values[0], values[1],
                           values[2], /* rad */
                           values[3], values[4]);
    break;

  case NEWPATH:
    pdf_dev_newpath();
    break;
  case STROKE:
    /* fill rule not supported yet */
    pdf_dev_flushpath('S', PDF_FILL_RULE_NONZERO);
    break;
  case FILL:
    pdf_dev_flushpath('f', PDF_FILL_RULE_NONZERO);
    break;

  case CLIP:
    error = pdf_dev_clip();
    break;
  case EOCLIP:
    error = pdf_dev_eoclip();
    break;

    /* Graphics state operators: */
  case GSAVE:
    error = pdf_dev_gsave();
    save_font();
    break;
  case GRESTORE:
    error = pdf_dev_grestore();
    restore_font();
    break;

  case CONCAT:
    tmp   = POP_STACK();
    error = cvr_array(tmp, values, 6); /* This does pdf_release_obj() */
    tmp   = NULL;
    if (error)
      dpx_warning("Missing array before \"concat\".");
    else {
      pdf_setmatrix(&matrix,
                    values[0], values[1],
                    values[2], values[3],
                    values[4], values[5]);
      error = pdf_dev_concat(&matrix);
    }
    break;
  case SCALE:
    error = pop_get_numbers(values, 2);
    if (!error) {
      switch (mp_cmode) {
      default:
        pdf_setmatrix(&matrix,
                      values[0], 0.0,
                      0.0      , values[1],
                      0.0      , 0.0);
        break;
      }

      error = pdf_dev_concat(&matrix);
    }
    break;
    /* Positive angle means clock-wise direction in graphicx-dvips??? */
  case ROTATE:
    error = pop_get_numbers(values, 1);
    if (!error) {
      values[0] = values[0] * M_PI / 180;

      switch (mp_cmode) {
      case MP_CMODE_DVIPSK:
      case MP_CMODE_MPOST: /* Really? */
        pdf_setmatrix(&matrix,
                      cos(values[0]), -sin(values[0]),
                      sin(values[0]),  cos(values[0]),
                      0.0,             0.0);
        break;
      default:
        pdf_setmatrix(&matrix,
                      cos(values[0]) , sin(values[0]),
                      -sin(values[0]), cos(values[0]),
                      0.0,             0.0);
        break;
      }
      error = pdf_dev_concat(&matrix);
    }
    break;
  case TRANSLATE:
    error = pop_get_numbers(values, 2);
    if (!error) {
      pdf_setmatrix(&matrix,
                    1.0,       0.0,
                    0.0,       1.0,
                    values[0], values[1]);
      error = pdf_dev_concat(&matrix);
    }
    break;

  case SETDASH:
    error = pop_get_numbers(values, 1);
    if (!error) {
      pdf_obj *pattern, *dash;
      int      i, num_dashes;
      double   dash_values[PDF_DASH_SIZE_MAX];
      double   offset;

      offset  = values[0];
      pattern = POP_STACK();
      if (!PDF_OBJ_ARRAYTYPE(pattern)) {
        pdf_release_obj(pattern);
        error = 1;
        break;
      }
      num_dashes = pdf_array_length(pattern);
      if (num_dashes > PDF_DASH_SIZE_MAX) {
        dpx_warning("Too many dashes...");
        pdf_release_obj(pattern);
        error = 1;
        break;
      }
      for (i = 0;
           i < num_dashes && !error ; i++) {
        dash = pdf_get_array(pattern, i);
        if (!PDF_OBJ_NUMBERTYPE(dash))
          error = 1;
        else {
          dash_values[i] = pdf_number_value(dash);
        }
      }
      pdf_release_obj(pattern);
      if (!error) {
        error = pdf_dev_setdash(num_dashes, dash_values, offset);
      }
    }
    break;
  case SETLINECAP:
    error = pop_get_numbers(values, 1);
    if (!error)
      error = pdf_dev_setlinecap((int)values[0]);
    break;
  case SETLINEJOIN:
    error = pop_get_numbers(values, 1);
    if (!error)
      error = pdf_dev_setlinejoin((int)values[0]);
    break;
  case SETLINEWIDTH:
    error = pop_get_numbers(values, 1);
    if (!error)
      error = pdf_dev_setlinewidth(values[0]);
    break;
  case SETMITERLIMIT:
    error = pop_get_numbers(values, 1);
    if (!error)
      error = pdf_dev_setmiterlimit(values[0]);
    break;

  case SETCMYKCOLOR:
    error = pop_get_numbers(values, 4);
    /* Not handled properly */
    if (!error) {
      pdf_color_cmykcolor(&color,
                          values[0], values[1],
                          values[2], values[3]);
      pdf_dev_set_strokingcolor(&color);
      pdf_dev_set_nonstrokingcolor(&color);
    }
    break;
  case SETGRAY:
    /* Not handled properly */
    error = pop_get_numbers(values, 1);
    if (!error) {
      pdf_color_graycolor(&color, values[0]);
      pdf_dev_set_strokingcolor(&color);
      pdf_dev_set_nonstrokingcolor(&color);
    }
    break;
  case SETRGBCOLOR:
    error = pop_get_numbers(values, 3);
    if (!error) {
      pdf_color_rgbcolor(&color,
                         values[0], values[1], values[2]);
      pdf_dev_set_strokingcolor(&color);
      pdf_dev_set_nonstrokingcolor(&color);
    }
    break;

  case SHOWPAGE: /* Let's ignore this for now */
    break;

  case CURRENTPOINT:
    error = pdf_dev_currentpoint(&cp);
    if (!error) {
      PUSH(pdf_new_number(cp.x));
      PUSH(pdf_new_number(cp.y));
    }
    break;

  case DTRANSFORM:
    {
      int  has_matrix = 0;

      tmp = POP_STACK();
      if (PDF_OBJ_ARRAYTYPE(tmp)) {
        error = cvr_array(tmp, values, 6); /* This does pdf_release_obj() */
        tmp   = NULL;
        if (error)
          break;
        pdf_setmatrix(&matrix,
                      values[0], values[1],
                      values[2], values[3],
                      values[4], values[5]);
        tmp = POP_STACK();
        has_matrix = 1;
      }

      if (!PDF_OBJ_NUMBERTYPE(tmp)) {
        error = 1;
        break;
      }
      cp.y = pdf_number_value(tmp);
      pdf_release_obj(tmp);

      tmp = POP_STACK();
      if (!PDF_OBJ_NUMBERTYPE(tmp)) {
        error = 1;
        break;
      }
      cp.x = pdf_number_value(tmp);
      pdf_release_obj(tmp);

      if (!has_matrix) {
        ps_dev_CTM(&matrix); /* Here, we need real PostScript CTM */
      }
      pdf_dev_dtransform(&cp, &matrix);
      PUSH(pdf_new_number(cp.x));
      PUSH(pdf_new_number(cp.y));
    }
    break;

  case IDTRANSFORM:
    {
      int  has_matrix = 0;

      tmp = POP_STACK();
      if (PDF_OBJ_ARRAYTYPE(tmp)) {
        error = cvr_array(tmp, values, 6); /* This does pdf_release_obj() */
        tmp   = NULL;
        if (error)
          break;
        pdf_setmatrix(&matrix,
                      values[0], values[1],
                      values[2], values[3],
                      values[4], values[5]);
        tmp = POP_STACK();
        has_matrix = 1;
      }

      if (!PDF_OBJ_NUMBERTYPE(tmp)) {
        error = 1;
        break;
      }
      cp.y = pdf_number_value(tmp);
      pdf_release_obj(tmp);

      tmp = POP_STACK();
      if (!PDF_OBJ_NUMBERTYPE(tmp)) {
        error = 1;
        break;
      }
      cp.x = pdf_number_value(tmp);
      pdf_release_obj(tmp);

      if (!has_matrix) {
        ps_dev_CTM(&matrix); /* Here, we need real PostScript CTM */
      }
      pdf_dev_idtransform(&cp, &matrix);
      PUSH(pdf_new_number(cp.x));
      PUSH(pdf_new_number(cp.y));
      break;
    }

  case FINDFONT:
    error = do_findfont();
    break;
  case SCALEFONT:
    error = do_scalefont();
    break;
  case SETFONT:
    error = do_setfont();
    break;
  case CURRENTFONT:
    error = do_currentfont();
    break;

  case SHOW:
    error = do_show();
    break;

  case STRINGWIDTH:
    error = 1;
    break;

    /* Extensions */
  case FSHOW:
    error = do_mpost_bind_def("exch findfont exch scalefont setfont show", x_user, y_user);
    break;
  case STEXFIG:
  case ETEXFIG:
    error = do_texfig_operator(opcode, x_user, y_user);
    break;
  case HLW:
    error = do_mpost_bind_def("0 dtransform exch truncate exch idtransform pop setlinewidth", x_user, y_user);
    break;
  case VLW:
    error = do_mpost_bind_def("0 exch dtransform truncate idtransform setlinewidth pop", x_user, y_user);
    break;
  case RD:
    error = do_mpost_bind_def("[] 0 setdash", x_user, y_user);
    break;
  case B:
    error = do_mpost_bind_def("gsave fill grestore", x_user, y_user);
    break;

  case DEF:
    tmp = POP_STACK();
    tmp = POP_STACK();
    /* do nothing; not implemented yet */
    break;

  default:
    if (is_fontname(token)) {
      PUSH(pdf_new_name(token));
    } else {
      dpx_warning("Unknown token \"%s\"", token);
      error = 1;
    }
    break;
  }

  return error;
}

/*
 * The only sections that need to know x_user and y _user are those
 * dealing with texfig.
 */
static int
mp_parse_body (const char **start, const char *end, double x_user, double y_user)
{
  char    *token;
  pdf_obj *obj;
  int      error = 0;

  skip_white(start, end);
  while (*start < end && !error) {
    if (isdigit((unsigned char)**start) ||
        (*start < end - 1 &&
         (**start == '+' || **start == '-' || **start == '.' ))) {
      double value;
      char  *next;

      value = strtod(*start, &next);
      if (next < end && !strchr("<([{/%", *next) && !isspace((unsigned char)*next)) {
        dpx_warning("Unkown PostScript operator.");
        dump(*start, next);
        error = 1;
      } else {
        PUSH(pdf_new_number(value));
        *start = next;
      }
      /*
       * PDF parser can't handle PS operator inside arrays.
       * This shouldn't use parse_pdf_array().
       */
    } else if (**start == '[' &&
               (obj = parse_pdf_array(start, end, NULL))) {
      PUSH(obj);
      /* This cannot handle ASCII85 string. */
    } else if (*start < end - 1 &&
               (**start == '<' && *(*start+1) == '<') &&
               (obj = parse_pdf_dict(start, end, NULL))) {
      PUSH(obj);
    } else if ((**start == '(' || **start == '<') &&
               (obj = parse_pdf_string (start, end))) {
      PUSH(obj);
    } else if (**start == '/' &&
               (obj = parse_pdf_name(start, end))) {
      PUSH(obj);
    } else {
      token = parse_ident(start, end);
      if (!token)
        error = 1;
      else {
        error = do_operator(token, x_user, y_user);
        free(token);
      }
    }
    skip_white(start, end);
  }

  return error;
}

void
mps_eop_cleanup (void)
{
  clear_fonts();
  do_clear();
}

int
mps_stack_depth (void)
{
  return top_stack;
}

int
mps_exec_inline (const char **p, const char *endptr,
                 double x_user, double y_user)
{
  int  error;
  int  dirmode, autorotate;

  /* Compatibility for dvipsk. */
  dirmode = pdf_dev_get_dirmode();
  if (dirmode) {
    mp_cmode = MP_CMODE_PTEXVERT;
  } else {
    mp_cmode = MP_CMODE_DVIPSK;
  }

  autorotate = pdf_dev_get_param(PDF_DEV_PARAM_AUTOROTATE);
  pdf_dev_set_param(PDF_DEV_PARAM_AUTOROTATE, 0);
  //pdf_color_push(); /* ... */

  /* Comment in dvipdfm:
   * Remember that x_user and y_user are off by 0.02 %
   */
  pdf_dev_moveto(x_user, y_user);
  error = mp_parse_body(p, endptr, x_user, y_user);

  //pdf_color_pop(); /* ... */
  pdf_dev_set_param(PDF_DEV_PARAM_AUTOROTATE, autorotate);
  pdf_dev_set_dirmode(dirmode);

  return error;
}

int
mps_do_page (FILE *image_file)
{
  int       error = 0;
  pdf_rect  bbox;
  char     *buffer;
  const char *start, *end;
  int       size;
  int       dir_mode;

  rewind(image_file);
  if ((size = file_size(image_file)) == 0) {
    dpx_warning("Can't read any byte in the MPS file.");
    return -1;
  }

  buffer = NEW(size+1, char);
  fread(buffer, sizeof(char), size, image_file);
  buffer[size] = 0;
  start = buffer;
  end   = buffer + size;

  error = mps_scan_bbox(&start, end, &bbox);
  if (error) {
    dpx_warning("Error occured while scanning MetaPost file headers: Could not find BoundingBox.");
    free(buffer);
    return -1;
  }

  mp_cmode = MP_CMODE_MPOST;

  pdf_doc_begin_page  (1.0, -Xorigin, -Yorigin); /* scale, xorig, yorig */
  pdf_doc_set_mediabox(pdf_doc_current_page_number(), &bbox);

  dir_mode = pdf_dev_get_dirmode();
  pdf_dev_set_autorotate(0);

  skip_prolog(&start, end);

  error = mp_parse_body(&start, end, 0.0, 0.0);

  if (error) {
    dpx_warning("Errors occured while interpreting MetaPost file.");
  }

  pdf_dev_set_autorotate(1);
  pdf_dev_set_dirmode(dir_mode);

  pdf_doc_end_page();

  free(buffer);

  /*
   * The reason why we don't return XObject itself is
   * PDF inclusion may not be made so.
   */
  return (error ? -1 : 0);
}

void
mps_reset_global_state(void) /* Tectonic */
{
  translate_origin = 0;
  currentfont = 0;
  font_stack[0].font_name = NULL;
  font_stack[0].font_id = -1;
  font_stack[0].tfm_id = -1;
  font_stack[0].subfont_id = -1;
  font_stack[0].pt_size = 0.0;
  mp_cmode = MP_CMODE_MPOST;
  top_stack = 0;
}
