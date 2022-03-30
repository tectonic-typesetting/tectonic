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

#include "dpx-pdfdev.h"

#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_types.h"
#include "dpx-cmap.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdffont.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfximage.h"
#include "dpx-type0.h"

struct dev_param
{
  /* Text composition (direction) mode is ignored (always same
   * as font's writing mode) if autorotate is unset (value zero).
   */
  int    autorotate;

  /*
   * Ignore color migrated to here. This is device's capacity.
   * colormode 0 for ignore colors
   */
  int    colormode;

};

/*
 * Text handling routines.
 */

/* Motion state:
 *  GRAPHICS_MODE  Initial state (not within BT/ET block nor in string)
 *  TEXT_MODE      Text section is started via BT operator but not
 *                 in string.
 *  STRING_MODE    In string. A string or array of strings are currently
 *                 in process. May started '[', '<' or '('.
 */
#define GRAPHICS_MODE  1
#define TEXT_MODE      2
#define STRING_MODE    3

/*
 * In PDF, vertical text positioning is always applied when current font
 * is vertical font. While ASCII pTeX manages current writing direction
 * and font's WMode separately.
 *
 * 000/101 WMODE_HH/VV  h(v) font, h(v) direction.
 * 001    WMODE_HV    -90 deg. rotated
 * 100    WMODE_VH    +90 deg. rotated
 * 011    WMODE_HD    +90 deg. rotated
 * 111    WMODE_VD    180 deg. rotated

 * In MetaPost PostScript file processing (mp_mode = 1), only HH/VV mode
 * is applied.
 */
#define TEXT_WMODE_HH 0
#define TEXT_WMODE_HV 1
#define TEXT_WMODE_VH 4
#define TEXT_WMODE_VV 5
#define TEXT_WMODE_HD 3
#define TEXT_WMODE_VD 7

#define ANGLE_CHANGES(m1,m2) ((abs((m1)-(m2)) % 5) == 0 ? 0 : 1)
#define ROTATE_TEXT(m)       ((m) != TEXT_WMODE_HH && (m) != TEXT_WMODE_VV)

struct text_state {

  /* Current font.
   * This is index within dev_fonts.
   */
  int       font_id;

  /* Dvipdfmx does compression of text by doing text positioning
   * in relative motion and uses string array [(foo) -250 (bar)]
   * with kerning (negative kern is used for white space as suited
   * for TeX). This is offset within current string.
   */
  spt_t     offset;

  /* This is reference point of strings.
   * It may include error correction induced by rounding.
   */
  spt_t     ref_x;
  spt_t     ref_y;

  /* Using text raise and leading is highly recommended for
   * text extraction to work properly. But not implemented yet.
   * We can't do consice output for \TeX without this.
   */
  spt_t     raise;    /* unused */
  spt_t     leading;  /* unused */

  /* This is not always text matrix but rather font matrix.
   * We do not use horizontal scaling in PDF text state parameter
   * since they always apply scaling in fixed direction regardless
   * of writing mode.
   */
  struct {
    double  slant;
    double  extend;
    int     rotate; /* TEXT_WMODE_XX */
  } matrix;

  /* Fake bold parameter:
   * If bold_param is positive, use text rendering mode
   * fill-then-stroke with stroking line width specified
   * by bold_param.
   */
  double    bold_param;

  /* Text composition (direction) mode. */
  int       dir_mode;

  /* internal */

  /* Flag indicating text matrix to be forcibly reset.
   * Enabled if synthetic font features (slant, extend, etc)
   * are used for current font or when text rotation mode
   * changes.
   */
  int       force_reset;

  /* This information is duplicated from dev[font_id].format.
   * Set to 1 if font is composite (Type0) font.
   */
  int       is_mb;
};

#define PDF_FONTTYPE_SIMPLE    1
#define PDF_FONTTYPE_BITMAP    2
#define PDF_FONTTYPE_COMPOSITE 3

struct dev_font {
  /* Needs to be big enough to hold name "Fxxx"
   * where xxx is number of largest font
   */
  char     short_name[16];      /* Resource name */
  int      used_on_this_page;

  char    *tex_name;  /* String identifier of this font */
  spt_t    sptsize;   /* Point size */

  /* Returned values from font/encoding layer:
   *
   * The font_id and enc_id is font and encoding (CMap) identifier
   * used in pdf_font or encoding/cmap layer.
   * The PDF object "resource" is an indirect reference object
   * pointing font resource of this font. The used_chars is somewhat
   * misleading, this is actually used_glyphs in CIDFont for Type0
   * and is 65536/8 bytes binary data with each bits representing
   * whether the glyph is in-use or not. It is 256 char array for
   * simple font.
   */
  int      font_id;
  int      enc_id;

  pdf_obj *resource;
  char    *used_chars;

  /* Font format:
   * simple, composite or bitmap.
   */
  int      format;

  /* Writing mode:
   * Non-zero for vertical. Duplicated from CMap.
   */
  int      wmode;

  /* Syntetic Font:
   *
   * We use text matrix for creating extended or slanted font,
   * but not with font's FontMatrix since TrueType and Type0
   * font don't support them.
   */
  double   extend;
  double   slant;
  double   bold;  /* Boldness prameter */
};

/*
 * Unit conversion, formatting and others.
 */

#define TEX_ONE_HUNDRED_BP 6578176
struct dev_unit {
  double dvi2pts;
  int    min_bp_val; /* Shortest resolvable distance in the output PDF.     */
  int    precision;  /* Number of decimal digits (in fractional part) kept. */
};

#define FORMAT_BUF_SIZE 4096
struct pdf_dev {
  int               motion_state;
  struct dev_param  param;
  struct dev_unit   unit;
  struct text_state text_state;
  struct dev_font  *fonts;
  int               num_dev_fonts;
  int               max_dev_fonts;
  char              format_buffer[FORMAT_BUF_SIZE+1];
};

/*
 * For a moment declare as static variable here
 * 2020/07/23 Most of the static variables put into single struct pdev
 */
static pdf_dev pdev;

static pdf_dev *
current_device (void)
{
  return &pdev;
}

static void
dev_out (pdf_dev *p, const char *str, size_t len)
{
  pdf_doc_add_page_content(str, len);
}

static double
pdf_dev_unit_dviunit (pdf_dev *p)
{
  assert(p);

  return (1.0 / p->unit.dvi2pts);
}

#define DEV_PRECISION_MAX  8
static uint32_t ten_pow[10] = {
  1u, 10u, 100u, 1000u, 10000u, 100000u, 1000000u, 10000000u, 100000000u, 1000000000u
};

static double ten_pow_inv[10] = {
  1.0, 0.1,  0.01,  0.001,  0.0001,  0.00001,  0.000001,  0.0000001,  0.00000001,  0.000000001
};

#define bpt2spt(p, b) ( (spt_t) round( (b) / (p)->unit.dvi2pts  ) )
#define spt2bpt(p, s) ( (s) * (p)->unit.dvi2pts )
#define dround_at(v,p) (ROUND( (v), ten_pow_inv[(p)] ))

static unsigned int
p_itoa (int value, char *buf)
{
  unsigned int sign, ndigits;
  char *p = buf;

  if (value < 0) {
    *p++  = '-';
    value = -value;
    sign  = 1;
  } else {
    sign  = 0;
  }

  ndigits = 0;
  /* Generate at least one digit in reverse order */
  do {
    p[ndigits++] = (value % 10) + '0';
    value /= 10;
  } while (value != 0);

  /* Reverse the digits */
  {
    unsigned int i;

    for (i = 0; i < ndigits / 2 ; i++) {
      char tmp = p[i];
      p[i] = p[ndigits-i-1];
      p[ndigits-i-1] = tmp;
    }
  }
  p[ndigits] = '\0';

  return  (sign ? ndigits + 1 : ndigits);
}

/* NOTE: Acrobat 5 and prior uses 16.16 fixed point representation for
 * real numbers.
 */
static int
p_dtoa (double value, int prec, char *buf)
{
  const int32_t p[10] = { 1, 10, 100, 1000, 10000,
                          100000, 1000000, 10000000,
                          100000000, 1000000000 };
  double i, f;
  int32_t g;
  char  *c = buf;
  int    n;

  if (value < 0) {
    value = -value;
    *c++ = '-';
    n = 1;
  } else {
    n = 0;
  }

  f = modf(value, &i);
  g = (int32_t) (f * p[prec] + 0.5);

  if (g == p[prec]) {
    g  = 0;
    i += 1;
  }

  if (i) {
    int m = sprintf(c, "%.0f", i);
    c += m;
    n += m;
  } else if (g == 0) {
    *(c = buf) = '0';
    n = 1;
  }

  if (g) {
    int j = prec;

    *c++ = '.';

    while (j--) {
      c[j] = (g % 10) + '0';
      g /= 10;
    }
    c += prec - 1;
    n += 1 + prec;

    while (*c == '0') {
      c--;
      n--;
    }
  }

  *(++c) = 0;

  return n;
}

static int
dev_sprint_bp (pdf_dev *p, char *buf, spt_t value, spt_t *error)
{
  double  value_in_bp;
  double  error_in_bp;
  int     prec;

  assert(p);

  prec = p->unit.precision;

  value_in_bp = spt2bpt(p, value);
  if (error) {
    error_in_bp = value_in_bp - dround_at(value_in_bp, prec);
    *error = bpt2spt(p, error_in_bp);
  }

  return  p_dtoa(value_in_bp, prec, buf);
}

/* They are affected by precision (set at device initialization). */
static size_t
pdf_dev_sprint_matrix (pdf_dev *p, char *buf, const pdf_tmatrix *M)
{
  size_t  len;
  int  prec0, prec2;

  assert(p);

  prec2 = MIN(p->unit.precision + 2, DEV_PRECISION_MAX);
  prec0 = MAX(p->unit.precision, 2);

  len  = p_dtoa(M->a, prec2, buf);
  buf[len++] = ' ';
  len += p_dtoa(M->b, prec2, buf+len);
  buf[len++] = ' ';
  len += p_dtoa(M->c, prec2, buf+len);
  buf[len++] = ' ';
  len += p_dtoa(M->d, prec2, buf+len);
  buf[len++] = ' ';
  len += p_dtoa(M->e, prec0, buf+len);
  buf[len++] = ' ';
  len += p_dtoa(M->f, prec0, buf+len);
  buf[len]   = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}

static size_t
pdf_dev_sprint_rect (pdf_dev *p, char *buf, const pdf_rect *rect)
{
  size_t  len;

  assert(p);

  len  = p_dtoa(rect->llx, p->unit.precision, buf);
  buf[len++] = ' ';
  len += p_dtoa(rect->lly, p->unit.precision, buf+len);
  buf[len++] = ' ';
  len += p_dtoa(rect->urx, p->unit.precision, buf+len);
  buf[len++] = ' ';
  len += p_dtoa(rect->ury, p->unit.precision, buf+len);
  buf[len]   = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}

static size_t
pdf_dev_sprint_coord (pdf_dev *p, char *buf, const pdf_coord *c)
{
  size_t  len;

  assert(p);

  len  = p_dtoa(c->x, p->unit.precision, buf);
  buf[len++] = ' ';
  len += p_dtoa(c->y, p->unit.precision, buf+len);
  buf[len]   = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}

static size_t
pdf_dev_sprint_length (pdf_dev *p, char *buf, double value)
{
  size_t  len;

  assert(p);

  len = p_dtoa(value, p->unit.precision, buf);
  buf[len] = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}

static size_t
pdf_dev_sprint_number (pdf_dev *p, char *buf, double value)
{
  size_t len;

  assert(p);

  len = p_dtoa(value, DEV_PRECISION_MAX, buf);
  buf[len] = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}

/*
 * Text handling routines.
 */

#define CURRENTFONT(p) (((p)->text_state.font_id < 0) ? \
  NULL : &((p)->fonts[(p)->text_state.font_id]))
#define GET_FONT(p, n)   (&((p)->fonts[(n)]))

static void
dev_set_text_matrix (pdf_dev *p,
    spt_t xpos, spt_t ypos,
    double slant, double extend, int rotate)
{
  pdf_tmatrix tm;
  int         len = 0;

  assert(p);

  /* slant is negated for vertical font so that right-side
   * is always lower. */
  switch (rotate) {
  case TEXT_WMODE_VH:
    /* Vertical font */
    tm.a =  slant ;   tm.b =  1.0;
    tm.c = -extend;   tm.d =  0.0   ;
    break;
  case TEXT_WMODE_HV:
    /* Horizontal font */
    tm.a =  0.0;    tm.b = -extend;
    tm.c =  1.0;    tm.d = -slant ;
    break;
  case TEXT_WMODE_HH:
    /* Horizontal font */
    tm.a =  extend; tm.b =  0.0;
    tm.c =  slant ; tm.d =  1.0;
    break;
  case TEXT_WMODE_VV:
    /* Vertical font */
    tm.a =  1.0; tm.b =  -slant;
    tm.c =  0.0; tm.d =   extend;
    break;
  case TEXT_WMODE_HD:
    /* Horizontal font */
    tm.a =  0.0;    tm.b = extend;
    tm.c = -1.0;    tm.d = slant ;
    break;
  case TEXT_WMODE_VD:
    /* Vertical font */
    tm.a = -1.0; tm.b =   slant;
    tm.c =  0.0; tm.d =  -extend;
    break;
  }
  tm.e = xpos * p->unit.dvi2pts;
  tm.f = ypos * p->unit.dvi2pts;

  p->format_buffer[len++] = ' ';
  len += pdf_dev_sprint_matrix(p, p->format_buffer+len, &tm);
  p->format_buffer[len++] = ' ';
  p->format_buffer[len++] = 'T';
  p->format_buffer[len++] = 'm';

  dev_out(p, p->format_buffer, len);  /* op: Tm */

  p->text_state.ref_x = xpos;
  p->text_state.ref_y = ypos;
  p->text_state.matrix.slant  = slant;
  p->text_state.matrix.extend = extend;
  p->text_state.matrix.rotate = rotate;
}

/*
 * reset_text_state(p) outputs a BT and does any necessary coordinate
 * transformations to get ready to ship out text.
 */

static void
reset_text_state (pdf_dev *p)
{
  assert(p);

  /*
   * We need to reset the line matrix to handle slanted fonts.
   */
  dev_out(p, " BT", 3);  /* op: BT */
  /*
   * p->text_state.matrix is identity at top of page.
   * This sometimes write unnecessary "Tm"s when transition from
   * GRAPHICS_MODE to TEXT_MODE occurs.
   */
  if (p->text_state.force_reset ||
      p->text_state.matrix.slant  != 0.0 ||
      p->text_state.matrix.extend != 1.0 ||
      ROTATE_TEXT(p->text_state.matrix.rotate)) {
    dev_set_text_matrix(p, 0, 0,
                        p->text_state.matrix.slant,
                        p->text_state.matrix.extend,
                        p->text_state.matrix.rotate);
  }
  p->text_state.ref_x = 0;
  p->text_state.ref_y = 0;
  p->text_state.offset   = 0;
  p->text_state.force_reset = 0;
}

static void
pdf_dev_text_mode (pdf_dev *p)
{
  assert(p);

  switch (p->motion_state) {
  case TEXT_MODE:
    break;
  case STRING_MODE:
    dev_out(p, p->text_state.is_mb ? ">]TJ" : ")]TJ", 4);  /* op: TJ */
    break;
  case GRAPHICS_MODE:
    reset_text_state(p);
    break;
  }
  p->motion_state      = TEXT_MODE;
  p->text_state.offset = 0;
}

static void
pdf_dev_graphics_mode (pdf_dev *p)
{
  switch (p->motion_state) {
  case GRAPHICS_MODE:
    break;
  case STRING_MODE:
    dev_out(p, p->text_state.is_mb ? ">]TJ" : ")]TJ", 4);  /* op: TJ */
    /* continue */
  case TEXT_MODE:
    if (p->text_state.bold_param != 0.0) {
      /* fake-bold "2 Tr" is still active */
      dev_out(p, " 0 Tr", 5);  /* op: Tr */
      p->text_state.bold_param = 0.0;
    }
    dev_out(p, " ET", 3);  /* op: ET */
    p->text_state.force_reset =  0;
    p->text_state.font_id     = -1;
    break;
  }
  p->motion_state = GRAPHICS_MODE;
}

static void
start_string (pdf_dev *p,
  spt_t xpos, spt_t ypos,
  double slant, double extend, int rotate)
{
  spt_t delx, dely, error_delx = 0, error_dely = 0;
  spt_t desired_delx, desired_dely;
  int   len = 0;

  assert(p);

  delx = xpos - p->text_state.ref_x;
  dely = ypos - p->text_state.ref_y;
  /*
   * Precompensating for line transformation matrix.
   *
   * Line transformation matrix L for horizontal font in horizontal
   * mode and it's inverse I is
   *
   *          | e  0|          | 1/e  0|
   *   L_hh = |     | , I_hh = |       |
   *          | s  1|          |-s/e  1|
   *
   * For vertical font in vertical mode,
   *
   *          | 1  -s|          | 1  s/e|
   *   L_vv = |      | , I_vv = |       |
   *          | 0   e|          | 0  1/e|
   *
   * For vertical font in horizontal mode,
   *
   *          | s   1|          | 0  1|
   *   L_vh = |      | = L_vv x |     |
   *          |-e   0|          |-1  0|
   *
   *          | 0  -1|
   *   I_vh = |      | x I_vv
   *          | 1   0|
   *
   * For horizontal font in vertical mode,
   *
   *          | 0  -e|          | 0  -1|
   *   L_hv = |      | = L_hh x |      |
   *          | 1  -s|          | 1   0|
   *
   *          | 0   1|
   *   I_hv = |      | x I_hh
   *          |-1   0|
   *
   */
  switch (rotate) {
  case TEXT_WMODE_VH:
    /* Vertical font in horizontal mode: rot = +90
     *                           | 0  -1/e|
     * d_user =  d x I_vh = d x  |        |
     *                           | 1   s/e|
     */
    desired_delx = dely;
    desired_dely = (spt_t) (-(delx - dely*slant)/extend);

    /* error_del is in device space
     *
     *               | 0  1|
     *  e = e_user x |     | = (-e_user_y, e_user_x)
     *               |-1  0|
     *
     * We must care about rotation here but not extend/slant...
     * The extend and slant actually is font matrix.
     */
    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_delx, &error_dely);
    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_dely, &error_delx);
    error_delx = -error_delx;
    break;
  case TEXT_WMODE_HV:
    /* Horizontal font in vertical mode: rot = -90
     *
     *                         |-s/e  1|
     * d_user = d x I_hv = d x |       |
     *                         |-1/e  0|
     */
    desired_delx = (spt_t)(-(dely + delx*slant)/extend);
    desired_dely = delx;

    /*
     * e = (e_user_y, -e_user_x)
     */
    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_delx, &error_dely);
    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_dely, &error_delx);
    error_dely = -error_dely;
    break;
  case TEXT_WMODE_HH:
    /* Horizontal font in horizontal mode:
     *                         | 1/e    0|
     * d_user = d x I_hh = d x |         |
     *                         |-s/e    1|
     */
    desired_delx = (spt_t)((delx - dely*slant)/extend);
    desired_dely = dely;

    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_delx, &error_delx);
    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_dely, &error_dely);
    break;
  case TEXT_WMODE_VV:
    /* Vertical font in vertical mode:
     *                         | 1  s/e|
     * d_user = d x I_vv = d x |       |
     *                         | 0  1/e|
     */
    desired_delx = delx;
    desired_dely = (spt_t)((dely + delx*slant)/extend);

    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_delx, &error_delx);
    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_dely, &error_dely);
    break;
  case TEXT_WMODE_HD:
    /* Horizontal font in down-to-up mode: rot = +90
     *
     *                          | s/e  -1|
     * d_user = d x -I_hv = d x |        |
     *                          | 1/e   0|
     */
    desired_delx = -(spt_t)(-(dely + delx*slant)/extend);
    desired_dely = -delx;

    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_delx, &error_dely);
    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_dely, &error_delx);
    error_delx = -error_delx;
    error_dely = -error_dely;
   break;
  case TEXT_WMODE_VD:
    /* Vertical font in down-to-up mode: rot = 180
     *                          |-1 -s/e|
     * d_user = d x -I_vv = d x |       |
     *                          | 0 -1/e|
     */
    desired_delx = -delx;
    desired_dely = -(spt_t)((dely + delx*slant)/extend);

    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_delx, &error_delx);
    p->format_buffer[len++] = ' ';
    len += dev_sprint_bp(p, p->format_buffer+len, desired_dely, &error_dely);
    error_delx = -error_delx;
    error_dely = -error_dely;
    break;
  }
  dev_out(p, p->format_buffer, len);  /* op: */
  /*
   * dvipdfm wrongly using "TD" in place of "Td".
   * The TD operator set leading, but we are not using T* etc.
   */
  dev_out(p, p->text_state.is_mb ? " Td[<" : " Td[(", 5);  /* op: Td */

  /* Error correction */
  p->text_state.ref_x = xpos - error_delx;
  p->text_state.ref_y = ypos - error_dely;

  p->text_state.offset   = 0;
}

static void
pdf_dev_string_mode (pdf_dev *p, spt_t xpos, spt_t ypos, double slant, double extend, int rotate)
{
  assert(p);

  switch (p->motion_state) {
  case STRING_MODE:
    break;
  case GRAPHICS_MODE:
    reset_text_state(p);
    /* continue */
  case TEXT_MODE:
    if (p->text_state.force_reset) {
      dev_set_text_matrix(p, xpos, ypos, slant, extend, rotate);
      dev_out(p, p->text_state.is_mb ? "[<" : "[(", 2);  /* op: */
      p->text_state.force_reset = 0;
    } else {
      start_string(p, xpos, ypos, slant, extend, rotate);
    }
    break;
  }
  p->motion_state = STRING_MODE;
}

/*
 * The purpose of the following routine is to force a Tf only
 * when it's actually necessary.  This became a problem when the
 * VF code was added.  The VF spec says to instantiate the
 * first font contained in the VF file before drawing a virtual
 * character.  However, that font may not be used for
 * many characters (e.g. small caps fonts).  For this reason,
 * dev_select_font() should not force a "physical" font selection.
 * This routine prevents a PDF Tf font selection until there's
 * really a character in that font.
 */

static int
pdf_dev_set_font (pdf_dev *p, int font_id)
{
  struct dev_font *font;
  int    text_rotate;
  double font_scale;
  int    len;
  int    vert_dir, vert_font;

  assert(p);

  /* pdf_dev_text_mode(p) must come before p->text_state.is_mb is changed. */
  pdf_dev_text_mode(p);

  font = GET_FONT(p, font_id);
  assert(font); /* Caller should check font_id. */

  p->text_state.is_mb = (font->format == PDF_FONTTYPE_COMPOSITE) ? 1 : 0;

  vert_font  = font->wmode ? 1 : 0;
  if (p->param.autorotate) {
    vert_dir = p->text_state.dir_mode;
  } else {
    vert_dir = vert_font;
  }
  text_rotate = (vert_font << 2)|vert_dir;

  if (font->slant  != p->text_state.matrix.slant  ||
      font->extend != p->text_state.matrix.extend ||
      ANGLE_CHANGES(text_rotate, p->text_state.matrix.rotate)) {
    p->text_state.force_reset = 1;
  }
  p->text_state.matrix.slant  = font->slant;
  p->text_state.matrix.extend = font->extend;
  p->text_state.matrix.rotate = text_rotate;

  if (!font->resource) {
    font->resource   = pdf_get_font_reference(font->font_id);
    font->used_chars = pdf_get_font_usedchars(font->font_id);
  }

  if (!font->used_on_this_page) {
    pdf_doc_add_page_resource("Font",
                              font->short_name,
                              pdf_link_obj(font->resource));
    font->used_on_this_page = 1;
  }

  font_scale = (double) font->sptsize * p->unit.dvi2pts;
  len  = sprintf(p->format_buffer, " /%s", font->short_name); /* space not necessary. */
  p->format_buffer[len++] = ' ';
  len += p_dtoa(font_scale, MIN(p->unit.precision+1, DEV_PRECISION_MAX), p->format_buffer+len);
  p->format_buffer[len++] = ' ';
  p->format_buffer[len++] = 'T';
  p->format_buffer[len++] = 'f';
  dev_out(p, p->format_buffer, len);  /* op: Tf */

  if (font->bold > 0.0 || font->bold != p->text_state.bold_param) {
    if (font->bold <= 0.0)
      len = sprintf(p->format_buffer, " 0 Tr");
    else
      len = sprintf(p->format_buffer, " 2 Tr %.6f w", font->bold); /* _FIXME_ */
    dev_out(p, p->format_buffer, len);  /* op: Tr w */
  }
  p->text_state.bold_param = font->bold;

  p->text_state.font_id    = font_id;

  return  0;
}

/* These tmp buffers can't be removed since the pointer to this can be
 * used as the return value of handle_multibyte_string(): str_ptr can point these.
 */
static unsigned char sbuf0[FORMAT_BUF_SIZE];

static int
handle_multibyte_string (struct dev_font *font,
                         const unsigned char **str_ptr, size_t *str_len)
{
  const unsigned char *p;
  size_t               length;

  p      = *str_ptr;
  length = *str_len;

  /*
   * Font is double-byte font. Output is assumed to be 16-bit fixed length
   * encoding.
   * TODO: A character decomposed to multiple characters.
   */
  if (font->enc_id >= 0) {
    const unsigned char *inbuf;
    unsigned char *outbuf;
    size_t         inbytesleft, outbytesleft;
    CMap          *cmap;

    cmap         = CMap_cache_get(font->enc_id);
    inbuf        = p;
    outbuf       = sbuf0;
    inbytesleft  = length;
    outbytesleft = FORMAT_BUF_SIZE;

    CMap_decode(cmap,
                &inbuf, &inbytesleft, &outbuf, &outbytesleft);
    if (inbytesleft != 0) {
      dpx_warning("CMap conversion failed. (%"PRIuZ" bytes remains)", inbytesleft);
      return -1;
    }
    length  = FORMAT_BUF_SIZE - outbytesleft;
    p       = sbuf0;
  }

  *str_ptr = p;
  *str_len = length;
  return 0;
}

void
pdf_dev_set_string (spt_t xpos, spt_t ypos,
                    const void *instr_ptr, size_t instr_len,
                    spt_t width,
                    int   font_id)
{
  pdf_dev *p = current_device();
  struct dev_font *font;
  const unsigned char *str_ptr; /* Pointer to the reencoded string. */
  size_t           length, i, len = 0;
  spt_t            kern, delh, delv;
  spt_t            text_xorigin;
  spt_t            text_yorigin;

  if (font_id < 0 || font_id >= p->num_dev_fonts) {
    _tt_abort("Invalid font: %d (%d)", font_id, p->num_dev_fonts);
    return;
  }
  if (font_id != p->text_state.font_id) {
    pdf_dev_set_font(p, font_id);
  }

  font = CURRENTFONT(p);
  if (!font) {
    _tt_abort("Currentfont not set.");
  }

  text_xorigin = p->text_state.ref_x;
  text_yorigin = p->text_state.ref_y;

  str_ptr = instr_ptr;
  length  = instr_len;

  if (font->format == PDF_FONTTYPE_COMPOSITE) {
    if (handle_multibyte_string(font, &str_ptr, &length) < 0) {
      _tt_abort("Error in converting input string...");
    }
    if (font->used_chars != NULL) {
      for (i = 0; i < length; i += 2) {
        unsigned short cid = (str_ptr[i] << 8) | str_ptr[i + 1];
        add_to_used_chars2(font->used_chars, cid);
      }
    }
  } else {
    if (font->used_chars != NULL) {
      for (i = 0; i < length; i++)
        font->used_chars[str_ptr[i]] = 1;
    }
  }

  /*
   * Kern is in units of character units, i.e., 1000 = 1 em.
   *
   * Positive kern means kerning (reduce excess white space).
   *
   * The following formula is of the form a*x/b where a, x, and b are signed long
   * integers.  Since in integer arithmetic (a*x) could overflow and a*(x/b) would
   * not be accurate, we use floating point arithmetic rather than trying to do
   * this all with integer arithmetic.
   *
   * 1000.0 / (font->extend * font->sptsize) is caluculated each times...
   * Is accuracy really a matter? Character widths are always rounded to integer
   * (in 1000 units per em) but dvipdfmx does not take into account of this...
   */

  if (p->text_state.dir_mode==0) {
    /* Left-to-right */
    delh = text_xorigin + p->text_state.offset - xpos;
    delv = ypos - text_yorigin;
  } else if (p->text_state.dir_mode==1) {
    /* Top-to-bottom */
    delh = ypos - text_yorigin + p->text_state.offset;
    delv = xpos - text_xorigin;
  } else {
    /* Bottom-to-top */
    delh = ypos + text_yorigin + p->text_state.offset;
    delv = xpos + text_xorigin;
  }

  /* White-space more than 3em is not considered as a part of single text.
   * So we will break string mode in that case.
   * Dvipdfmx spend most of time processing strings with kern = 0 (but far
   * more times in font handling).
   * You may want to use pre-calculated value for WORD_SPACE_MAX.
   * More text compression may be possible by replacing kern with space char
   * when -kern is equal to space char width.
   */
#define WORD_SPACE_MAX(f) (spt_t) (3.0 * (f)->extend * (f)->sptsize)

  if (p->text_state.force_reset ||
      labs(delv) > p->unit.min_bp_val ||
      labs(delh) > WORD_SPACE_MAX(font)) {
    pdf_dev_text_mode(p);
    kern = 0;
  } else {
    kern = (spt_t) (1000.0 / font->extend * delh / font->sptsize);
  }

  /* Inaccucary introduced by rounding of character width appears within
   * single text block. There are point_size/1000 rounding error per character.
   * If you really care about accuracy, you should compensate this here too.
   */
  if (p->motion_state != STRING_MODE)
    pdf_dev_string_mode(p, xpos, ypos,
                font->slant, font->extend, p->text_state.matrix.rotate);
  else if (kern != 0) {
    /*
     * Same issues as earlier. Use floating point for simplicity.
     * This routine needs to be fast, so we don't call sprintf() or strcpy().
     */
    p->text_state.offset -=
      (spt_t) (kern * font->extend * (font->sptsize / 1000.0));
    p->format_buffer[len++] = p->text_state.is_mb ? '>' : ')';
    if (font->wmode)
      len += p_itoa(-kern, p->format_buffer + len);
    else {
      len += p_itoa( kern, p->format_buffer + len);
    }
    p->format_buffer[len++] = p->text_state.is_mb ? '<' : '(';
    dev_out(p, p->format_buffer, len);  /* op: */
    len = 0;
  }

  if (p->text_state.is_mb) {
    if (FORMAT_BUF_SIZE - len < 2 * length)
      _tt_abort("Buffer overflow...");
    for (i = 0; i < length; i++) {
      int first, second;

      first  = (str_ptr[i] >> 4) & 0x0f;
      second = str_ptr[i] & 0x0f;
      p->format_buffer[len++] = ((first >= 10)  ? first  + 'W' : first  + '0');
      p->format_buffer[len++] = ((second >= 10) ? second + 'W' : second + '0');
    }
  } else {
    len += pdfobj_escape_str(p->format_buffer + len,
                             FORMAT_BUF_SIZE - len, str_ptr, length);
  }
  /* I think if you really care about speed, you should avoid memcopy here. */
  dev_out(p, p->format_buffer, len);  /* op: */

  p->text_state.offset += width;
}

void
pdf_init_device (double dvi2pts, int precision, int black_and_white)
{
  pdf_dev *p = current_device();

  p->motion_state        = GRAPHICS_MODE;
  p->unit.dvi2pts        = 0.0;
  p->unit.min_bp_val     = 658;
  p->unit.precision      = 2;
  p->param.autorotate    = 1;
  p->param.colormode     = 1;
  p->text_state.font_id  = -1;
  p->text_state.offset   = 0;
  p->text_state.matrix.slant  = 0;
  p->text_state.matrix.extend = 0;
  p->text_state.matrix.rotate = 0;
  p->text_state.bold_param  = 0;
  p->text_state.dir_mode    = 0;
  p->text_state.force_reset = 0;
  p->text_state.is_mb       = 0;

  if (precision < 0 ||
      precision > DEV_PRECISION_MAX)
    dpx_warning("Number of decimal digits out of range [0-%d].",
         DEV_PRECISION_MAX);

  if (precision < 0) {
    p->unit.precision  = 0;
  } else if (precision > DEV_PRECISION_MAX) {
    p->unit.precision  = DEV_PRECISION_MAX;
  } else {
    p->unit.precision  = precision;
  }
  p->unit.dvi2pts      = dvi2pts;
  p->unit.min_bp_val   = (int) ROUND(1.0/(ten_pow[p->unit.precision]*dvi2pts), 1);
  if (p->unit.min_bp_val < 0)
    p->unit.min_bp_val = -p->unit.min_bp_val;

  p->param.colormode = (black_and_white ? 0 : 1);

  pdf_dev_graphics_mode(p);
  pdf_color_clear_stack();
  pdf_dev_init_gstates();

  p->num_dev_fonts  = p->max_dev_fonts = 0;
  p->fonts      = NULL;
}

void
pdf_close_device (void)
{
  pdf_dev *p = current_device();

  if (p->fonts) {
    int    i;

    for (i = 0; i < p->num_dev_fonts; i++) {
      free(p->fonts[i].tex_name);
      pdf_release_obj(p->fonts[i].resource);
      p->fonts[i].tex_name = NULL;
      p->fonts[i].resource = NULL;
    }
    free(p->fonts);
    p->fonts = NULL;
  }
  pdf_dev_clear_gstates();
}

/*
 * BOP, EOP, and FONT section.
 * BOP and EOP manipulate some of the same data structures
 * as the font stuff.
 */
void
pdf_dev_reset_fonts (int newpage)
{
  pdf_dev *p = current_device();
  int  i;

  for (i = 0; i < p->num_dev_fonts; i++) {
    p->fonts[i].used_on_this_page = 0;
  }

  p->text_state.font_id       = -1;

  p->text_state.matrix.slant  = 0.0;
  p->text_state.matrix.extend = 1.0;
  p->text_state.matrix.rotate = TEXT_WMODE_HH;

  if (newpage)
    p->text_state.bold_param  = 0.0;

  p->text_state.is_mb         = 0;
}

void
pdf_dev_reset_color (int force)
{
  pdf_color *sc, *fc;

  pdf_color_get_current(&sc, &fc);
  pdf_dev_set_color(sc,    0, force);
  pdf_dev_set_color(fc, 0x20, force);
}

void
pdf_dev_bop (const pdf_tmatrix *M)
{
  pdf_dev *p = current_device();

  pdf_dev_graphics_mode(p);

  p->text_state.force_reset  = 0;

  pdf_dev_gsave();
  pdf_dev_concat(M);

  pdf_dev_reset_fonts(1);
  pdf_dev_reset_color(0);
  pdf_dev_reset_xgstate(0);
}

void
pdf_dev_eop (void)
{
  pdf_dev *p = current_device();
  int  depth;

  pdf_dev_graphics_mode(p);

  depth = pdf_dev_current_depth();
  if (depth != 1) {
    dpx_warning("Unbalenced q/Q nesting...: %d", depth);
    pdf_dev_grestore_to(0);
  } else {
    pdf_dev_grestore();
  }
}

static void
print_fontmap (const char *font_name, fontmap_rec *mrec)
{
  if (!mrec)
    return;

  dpx_message("\n");

  dpx_message("fontmap: %s -> %s", font_name, mrec->font_name);
  if (mrec->enc_name)
    dpx_message("(%s)",  mrec->enc_name);
  if (mrec->opt.extend != 1.0)
    dpx_message("[extend:%g]", mrec->opt.extend);
  if (mrec->opt.slant  != 0.0)
    dpx_message("[slant:%g]",  mrec->opt.slant);
  if (mrec->opt.bold   != 0.0)
    dpx_message("[bold:%g]",   mrec->opt.bold);
  if (mrec->opt.flags & FONTMAP_OPT_NOEMBED)
    dpx_message("[noemb]");
  if (mrec->opt.mapc >= 0)
    dpx_message("[map:<%02x>]", mrec->opt.mapc);
  if (mrec->opt.charcoll)
    dpx_message("[csi:%s]",     mrec->opt.charcoll);
  if (mrec->opt.index)
    dpx_message("[index:%d]",   mrec->opt.index);

  switch (mrec->opt.style) {
  case FONTMAP_STYLE_BOLD:
    dpx_message("[style:bold]");
    break;
  case FONTMAP_STYLE_ITALIC:
    dpx_message("[style:italic]");
    break;
  case FONTMAP_STYLE_BOLDITALIC:
    dpx_message("[style:bolditalic]");
    break;
  }
  dpx_message("\n");

}

/* _FIXME_
 * Font is identified with font_name and point_size as in DVI here.
 * However, except for PDF_FONTTYPE_BITMAP, we can share the
 * short_name, resource and used_chars between multiple instances
 * of the same font at different sizes.
 */
int
pdf_dev_locate_font (const char *font_name, spt_t ptsize)
{
  char *pp;
  pdf_dev *p = current_device();
  int              i;
  fontmap_rec     *mrec;
  struct dev_font *font;

  if (!font_name)
    return  -1;

  if (ptsize == 0) {
    _tt_abort("pdf_dev_locate_font() called with the zero ptsize.");
  }

  for (i = 0; i < p->num_dev_fonts; i++) {
    if (streq_ptr(font_name, p->fonts[i].tex_name) && ptsize == p->fonts[i].sptsize)
      return i; /* found a dev_font that matches the request */
  }

  /*
   * Make sure we have room for a new one, even though we may not
   * actually create one.
   */
  if (p->num_dev_fonts >= p->max_dev_fonts) {
    p->max_dev_fonts += 16;
    p->fonts      = RENEW(p->fonts, p->max_dev_fonts, struct dev_font);
  }

  font = &p->fonts[p->num_dev_fonts];

  /* New font */
  mrec = pdf_lookup_fontmap_record(font_name);

/*
  The extension ".pfb" is not needed for type1 fonts.
  And the extension ".pfb" prohibits to call mktexpk with right
  arguments when pdftex.map is used and when type1 is not found.
  Thus we discard the extension ".pfb".
*/
  if (mrec && mrec->font_name) {
    pp = strrchr(mrec->font_name, '.');
    if (pp && strcasecmp(pp, ".pfb") == 0)
      *pp = '\0';
  }

  if (dpx_conf.verbose_level > 1)
    print_fontmap(font_name, mrec);

  font->font_id = pdf_font_findresource(font_name, ptsize * p->unit.dvi2pts);
  if (font->font_id < 0) {
    font->font_id = pdf_font_load_font(font_name, ptsize * p->unit.dvi2pts, mrec);
    if (font->font_id < 0)
      return  -1;
  }

  pdf_font_resource_name(font->font_id, font->short_name);

  font->used_on_this_page = 0;

  font->tex_name = NEW(strlen(font_name) + 1, char);
  strcpy(font->tex_name, font_name);
  font->sptsize  = ptsize;

  switch (pdf_get_font_subtype(font->font_id)) {
  case PDF_FONT_FONTTYPE_TYPE3:
    font->format = PDF_FONTTYPE_BITMAP;
    break;
  case PDF_FONT_FONTTYPE_TYPE0:
    font->format = PDF_FONTTYPE_COMPOSITE;
    break;
  default:
    font->format = PDF_FONTTYPE_SIMPLE;
    break;
  }

  font->wmode      = pdf_get_font_wmode   (font->font_id);
  font->enc_id     = pdf_get_font_encoding(font->font_id);

  font->resource   = NULL; /* Don't ref obj until font is actually used. */
  font->used_chars = NULL;

  font->extend     = 1.0;
  font->slant      = 0.0;
  font->bold       = 0.0;

  if (mrec) {
    font->extend = mrec->opt.extend;
    font->slant  = mrec->opt.slant;
    font->bold   = mrec->opt.bold;
  }

  return  p->num_dev_fonts++;
}


/* This does not remember current stroking width. */
static int
dev_sprint_line (pdf_dev *p, char *buf, spt_t width,
                 spt_t p0_x, spt_t p0_y, spt_t p1_x, spt_t p1_y)
{
  int    len = 0;
  double w;

  w = width * p->unit.dvi2pts;

  len += p_dtoa(w, MIN(p->unit.precision+1, DEV_PRECISION_MAX), buf+len);
  buf[len++] = ' ';
  buf[len++] = 'w';
  buf[len++] = ' ';
  len += dev_sprint_bp(p, buf+len, p0_x, NULL);
  buf[len++] = ' ';
  len += dev_sprint_bp(p, buf+len, p0_y, NULL);
  buf[len++] = ' ';
  buf[len++] = 'm';
  buf[len++] = ' ';
  len += dev_sprint_bp(p, buf+len, p1_x, NULL);
  buf[len++] = ' ';
  len += dev_sprint_bp(p, buf+len, p1_y, NULL);
  buf[len++] = ' ';
  buf[len++] = 'l';
  buf[len++] = ' ';
  buf[len++] = 'S';

  return len;
}

/* Not optimized. */
#define PDF_LINE_THICKNESS_MAX 5.0
void
pdf_dev_set_rule (spt_t xpos, spt_t ypos, spt_t width, spt_t height)
{
  pdf_dev *p = current_device();
  int    len = 0;
  double width_in_bp;

  pdf_dev_graphics_mode(p);

  p->format_buffer[len++] = ' ';
  p->format_buffer[len++] = 'q';
  p->format_buffer[len++] = ' ';
  /* Don't use too thick line. */
  width_in_bp = ((width < height) ? width : height) * p->unit.dvi2pts;
  if (width_in_bp < 0.0 || /* Shouldn't happen */
      width_in_bp > PDF_LINE_THICKNESS_MAX) {
    pdf_rect rect;

    rect.llx =  p->unit.dvi2pts * xpos;
    rect.lly =  p->unit.dvi2pts * ypos;
    rect.urx =  p->unit.dvi2pts * width;
    rect.ury =  p->unit.dvi2pts * height;
    len += pdf_sprint_rect(p->format_buffer+len, &rect);
    p->format_buffer[len++] = ' ';
    p->format_buffer[len++] = 'r';
    p->format_buffer[len++] = 'e';
    p->format_buffer[len++] = ' ';
    p->format_buffer[len++] = 'f';
  } else {
    if (width > height) {
      /* NOTE:
       *  A line width of 0 denotes the thinnest line that can be rendered at
       *  device resolution. See, PDF Reference Manual 4th ed., sec. 4.3.2,
       *  "Details of Graphics State Parameters", p. 185.
       */
      if (height < p->unit.min_bp_val) {
        dpx_warning("Too thin line: height=%d (%g bp)", height, width_in_bp);
        dpx_warning("Please consider using \"-d\" option.");
      }
      len += dev_sprint_line(p, p->format_buffer+len,
                             height,
                             xpos,
                             ypos + height/2,
                             xpos + width,
                             ypos + height/2);
    } else {
      if (width < p->unit.min_bp_val) {
        dpx_warning("Too thin line: width=%d (%g bp)", width, width_in_bp);
        dpx_warning("Please consider using \"-d\" option.");
      }
      len += dev_sprint_line(p, p->format_buffer+len,
                             width,
                             xpos + width/2,
                             ypos,
                             xpos + width/2,
                             ypos + height);
    }
  }
  p->format_buffer[len++] = ' ';
  p->format_buffer[len++] = 'Q';
  dev_out(p, p->format_buffer, len);  /* op: q re f Q */
}

/* Rectangle in device space coordinate. */
void
pdf_dev_set_rect (pdf_rect *rect,
                  spt_t x_user, spt_t y_user,
                  spt_t width,  spt_t height, spt_t depth)
{
  pdf_dev *p = current_device();
  double      dev_x, dev_y;
  pdf_coord   p0, p1, p2, p3;
  double      min_x, min_y, max_x, max_y;

  dev_x = x_user * p->unit.dvi2pts;
  dev_y = y_user * p->unit.dvi2pts;
  if (p->text_state.dir_mode) {
    p0.x = dev_x - p->unit.dvi2pts * depth;
    p0.y = dev_y - p->unit.dvi2pts * width;
    p1.x = dev_x + p->unit.dvi2pts * height;
    p1.y = p0.y;
    p2.x = p1.x;
    p2.y = dev_y;
    p3.x = p0.x;
    p3.y = p2.y;
  } else {
    p0.x = dev_x;
    p0.y = dev_y - p->unit.dvi2pts * depth;
    p1.x = dev_x + p->unit.dvi2pts * width;
    p1.y = p0.y;
    p2.x = p1.x;
    p2.y = dev_y + p->unit.dvi2pts * height;
    p3.x = p0.x;
    p3.y = p2.y;
  }

  pdf_dev_transform(&p0, NULL); /* currentmatrix */
  pdf_dev_transform(&p1, NULL);
  pdf_dev_transform(&p2, NULL);
  pdf_dev_transform(&p3, NULL);

  min_x = MIN(p0.x , p1.x);
  min_x = MIN(min_x, p2.x);
  min_x = MIN(min_x, p3.x);

  max_x = MAX(p0.x , p1.x);
  max_x = MAX(max_x, p2.x);
  max_x = MAX(max_x, p3.x);

  min_y = MIN(p0.y , p1.y);
  min_y = MIN(min_y, p2.y);
  min_y = MIN(min_y, p3.y);

  max_y = MAX(p0.y , p1.y);
  max_y = MAX(max_y, p2.y);
  max_y = MAX(max_y, p3.y);

  rect->llx = min_x;
  rect->lly = min_y;
  rect->urx = max_x;
  rect->ury = max_y;

  return;
}

int
pdf_dev_get_dirmode (void)
{
  pdf_dev *p = current_device();
  return p->text_state.dir_mode;
}

void
pdf_dev_set_dirmode (int text_dir)
{
  pdf_dev *p = current_device();
  struct dev_font *font;
  int text_rotate;
  int vert_dir, vert_font;

  font = CURRENTFONT(p);

  vert_font = (font && font->wmode) ? 1 : 0;
  if (p->param.autorotate) {
    vert_dir = text_dir;
  } else {
    vert_dir = vert_font;
  }
  text_rotate = (vert_font << 2)|vert_dir;

  if (font &&
      ANGLE_CHANGES(text_rotate, p->text_state.matrix.rotate)) {
    p->text_state.force_reset = 1;
  }

  p->text_state.matrix.rotate = text_rotate;
  p->text_state.dir_mode      = text_dir;
}

static void
dev_set_param_autorotate (pdf_dev *p, int auto_rotate)
{
  struct dev_font *font;
  int    text_rotate, vert_font, vert_dir;

  assert(p);
  font = CURRENTFONT(p);

  vert_font = (font && font->wmode) ? 1 : 0;
  if (auto_rotate) {
    vert_dir = p->text_state.dir_mode;
  } else {
    vert_dir = vert_font;
  }
  text_rotate = (vert_font << 2)|vert_dir;

  if (ANGLE_CHANGES(text_rotate, p->text_state.matrix.rotate)) {
    p->text_state.force_reset = 1;
  }
  p->text_state.matrix.rotate = text_rotate;
  p->param.autorotate     = auto_rotate;
}

int
pdf_dev_get_param (int param_type)
{
  pdf_dev *p = current_device();
  int value = 0;

  switch (param_type) {
  case PDF_DEV_PARAM_AUTOROTATE:
    value = p->param.autorotate;
    break;
  case PDF_DEV_PARAM_COLORMODE:
    value = p->param.colormode;
    break;
  default:
    _tt_abort("Unknown device parameter: %d", param_type);
  }

  return value;
}

void
pdf_dev_set_param (int param_type, int value)
{
  pdf_dev *p = current_device();

  switch (param_type) {
  case PDF_DEV_PARAM_AUTOROTATE:
    dev_set_param_autorotate(p, value);
    break;
  case PDF_DEV_PARAM_COLORMODE:
    p->param.colormode = value; /* 0 for B&W */
    break;
  default:
    _tt_abort("Unknown device parameter: %d", param_type);
  }

  return;
}


int
pdf_dev_put_image (int             id,
                   transform_info *ti,
                   double          ref_x,
                   double          ref_y,
                   pdf_rect *rect)
{
  pdf_dev *p = current_device();
  char        *res_name;
  pdf_tmatrix  M, M1;
  pdf_rect     r;
  int          len = 0;

  pdf_copymatrix(&M, &(ti->matrix));
  M.e += ref_x; M.f += ref_y;
  /* Just rotate by -90, but not tested yet. Any problem if M has scaling? */
  if (p->param.autorotate &&
      p->text_state.dir_mode) {
    double tmp;
    tmp = -M.a; M.a = M.b; M.b = tmp;
    tmp = -M.c; M.c = M.d; M.d = tmp;
  }

  pdf_dev_graphics_mode(p);
  pdf_dev_gsave();

  pdf_ximage_scale_image(id, &M1, &r, ti);
  pdf_concatmatrix(&M, &M1);
  pdf_dev_concat(&M);

  /* Clip */
  if (ti->flags & INFO_DO_CLIP) {
    pdf_dev_rectclip(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);
  }

  res_name = pdf_ximage_get_resname(id);
  {
    char *buf;

    buf = NEW(strlen(res_name)+6, char);
    len = sprintf(buf, " /%s Do", res_name);
    dev_out(p, buf, len);  /* op: Do */
    free(buf);
  }
  if (rect) {
    pdf_rect  r1;

    /* Sorry for ugly code. */
    pdf_dev_set_rect(&r1,
                     bpt2spt(p, r.llx), bpt2spt(p, r.lly),
                     bpt2spt(p, r.urx - r.llx), bpt2spt(p, r.ury - r.lly), 0);
    rect->llx = r1.llx; rect->lly = r1.lly;
    rect->urx = r1.urx; rect->ury = r1.ury;
  }

  pdf_dev_grestore();

  pdf_doc_add_page_resource("XObject",
                            res_name,
                            pdf_ximage_get_reference(id));

  return 0;
}

void
transform_info_clear (transform_info *info)
{
  /* Physical dimensions */
  info->width    = 0.0;
  info->height   = 0.0;
  info->depth    = 0.0;

  info->bbox.llx = 0.0;
  info->bbox.lly = 0.0;
  info->bbox.urx = 0.0;
  info->bbox.ury = 0.0;

  /* Transformation matrix */
  pdf_setmatrix(&(info->matrix), 1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

  info->flags    = 0;
}

void
pdf_dev_begin_actualtext (uint16_t *unicodes, int count)
{
  pdf_dev *p = current_device();
  int len, i, pdf_doc_enc = 1;

  /* check whether we can use PDFDocEncoding for this string
     (we punt on the 0x80..0xA0 range that does not directly correspond to unicode)  */
  for (i = 0; i < count; i++) {
    if (unicodes[i] > 0xff || (unicodes[i] > 0x7f && unicodes[i] < 0xa1)) {
      pdf_doc_enc = 0;
      break;
    }
  }

  pdf_dev_graphics_mode(p);

  dev_out(p, "\n/Span << /ActualText (", 23);
  if (!pdf_doc_enc)
    dev_out(p, "\xFE\xFF", 2);

  while (count-- > 0) {
    unsigned char s[2] = { *unicodes >> 8, *unicodes & 0xff };
    char buf[32];
    i = pdf_doc_enc; /* if using PDFDocEncoding, we only care about the low 8 bits,
                        so start with the second byte of our pair */
    len = 0;
    for (; i < 2; i++) {
      unsigned char c = s[i];
      if (c == '(' || c == ')' || c == '\\') {
        len += sprintf(buf + len, "\\%c", c);
      } else if (c < ' ') {
        len += sprintf(buf + len, "\\%03o", c);
      } else {
        len += sprintf(buf + len, "%c", c);
      }
    }
    dev_out(p, work_buffer, len);
    ++unicodes;
  }

  dev_out(p, ") >> BDC", 8);
}

void
pdf_dev_end_actualtext (void)
{
  pdf_dev *p = current_device();

  pdf_dev_graphics_mode(p);

  dev_out(p, " EMC", 4);
}

/* Compatibility functions.... For other files still using old interface */
void
graphics_mode (void)
{
  pdf_dev *p = current_device();

  pdf_dev_graphics_mode(p);
}

double
dev_unit_dviunit (void)
{
  pdf_dev *p = current_device();
  return pdf_dev_unit_dviunit(p);
}

int
pdf_sprint_matrix (char *buf, const pdf_tmatrix *M)
{
  pdf_dev *p = current_device();
  return (int) pdf_dev_sprint_matrix(p, buf, M);
}

int
pdf_sprint_rect (char *buf, const pdf_rect *rect)
{
  pdf_dev *p = current_device();
  return (int) pdf_dev_sprint_rect(p, buf, rect);
}

int
pdf_sprint_coord (char *buf, const pdf_coord *c)
{
  pdf_dev *p = current_device();
  return (int) pdf_dev_sprint_coord(p, buf, c);
}

int
pdf_sprint_length (char *buf, double value)
{
  pdf_dev *p = current_device();
  return (int) pdf_dev_sprint_length(p, buf, value);
}

int
pdf_sprint_number (char *buf, double value)
{
  pdf_dev *p = current_device();
  return (int) pdf_dev_sprint_number(p, buf, value);
}

/* Access text state parameters. */
int
pdf_dev_get_font_wmode (int font_id)
{
  pdf_dev *p = current_device();
  struct dev_font *font;

  font = GET_FONT(p, font_id);
  if (font) {
    return font->wmode;
  }

  return 0;
}

int
pdf_dev_font_minbytes (int font_id)
{
  pdf_dev         *p        = current_device();
  struct dev_font *font;
  int              minbytes = 1;

  font = GET_FONT(p, font_id);
  if (font && font->format == PDF_FONTTYPE_COMPOSITE) {
    CMap *cmap;

    cmap     = CMap_cache_get(font->enc_id);
    minbytes = CMap_get_profile(cmap, CMAP_PROF_TYPE_INBYTES_MIN);
  }

  return minbytes;
}

/* Extra Tectonic API: */
void
pdf_dev_reset_global_state(void)
{
  pdf_dev *p = current_device();

  p->fonts = NULL;
  p->num_dev_fonts   = 0;
  p->max_dev_fonts   = 0;
}