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
#include "dpx-dvi.h"
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

/* Not working yet... */
double
pdf_dev_scale (void)
{
  return 1.0;
}

/*
 * Unit conversion, formatting and others.
 */

#define TEX_ONE_HUNDRED_BP 6578176
static struct {
  double dvi2pts;
  int    min_bp_val; /* Shortest resolvable distance in the output PDF.     */
  int    precision;  /* Number of decimal digits (in fractional part) kept. */
} dev_unit = {
  0.0,
  658,
  2
};


double
dev_unit_dviunit (void)
{
  return (1.0/dev_unit.dvi2pts);
}

#define DEV_PRECISION_MAX  8
static uint32_t ten_pow[10] = {
  1u, 10u, 100u, 1000u, 10000u, 100000u, 1000000u, 10000000u, 100000000u, 1000000000u
};

static double ten_pow_inv[10] = {
  1.0, 0.1,  0.01,  0.001,  0.0001,  0.00001,  0.000001,  0.0000001,  0.00000001,  0.000000001
};

#define bpt2spt(b) ( (spt_t) round( (b) / dev_unit.dvi2pts  ) )
#define spt2bpt(s) ( (s) * dev_unit.dvi2pts )
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
dev_sprint_bp (char *buf, spt_t value, spt_t *error)
{
  double  value_in_bp;
  double  error_in_bp;
  int     prec = dev_unit.precision;

  value_in_bp = spt2bpt(value);
  if (error) {
    error_in_bp = value_in_bp - dround_at(value_in_bp, prec);
    *error = bpt2spt(error_in_bp);
  }

  return  p_dtoa(value_in_bp, prec, buf);
}

/* They are affected by precision (set at device initialization). */
int
pdf_sprint_matrix (char *buf, const pdf_tmatrix *M)
{
  int  len;
  int  prec2 = MIN(dev_unit.precision + 2, DEV_PRECISION_MAX);
  int  prec0 = MAX(dev_unit.precision, 2);

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

int
pdf_sprint_rect (char *buf, const pdf_rect *rect)
{
  int  len;

  len  = p_dtoa(rect->llx, dev_unit.precision, buf);
  buf[len++] = ' ';
  len += p_dtoa(rect->lly, dev_unit.precision, buf+len);
  buf[len++] = ' ';
  len += p_dtoa(rect->urx, dev_unit.precision, buf+len);
  buf[len++] = ' ';
  len += p_dtoa(rect->ury, dev_unit.precision, buf+len);
  buf[len]   = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}

int
pdf_sprint_coord (char *buf, const pdf_coord *p)
{
  int  len;

  len  = p_dtoa(p->x, dev_unit.precision, buf);
  buf[len++] = ' ';
  len += p_dtoa(p->y, dev_unit.precision, buf+len);
  buf[len]   = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}

int
pdf_sprint_length (char *buf, double value)
{
  int  len;

  len = p_dtoa(value, dev_unit.precision, buf);
  buf[len] = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}


int
pdf_sprint_number (char *buf, double value)
{
  int  len;

  len = p_dtoa(value, DEV_PRECISION_MAX, buf);
  buf[len] = '\0'; /* xxx_sprint_xxx NULL terminates strings. */

  return  len;
}


static struct
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

} dev_param = {
  1, /* autorotate */
  1, /* colormode  */
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

static int motion_state = GRAPHICS_MODE;

#define FORMAT_BUF_SIZE 4096
static char format_buffer[FORMAT_BUF_SIZE];

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

static struct {

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
} text_state = {
  -1,            /* font   */
  0,             /* offset */
  0, 0,          /* ref_x, ref_y   */
  0, 0,          /* raise, leading */
  {0.0, 1.0, 0},

  0.0,  /* Experimental boldness param */

  0,    /* dir_mode      */

  /* internal */
  0,    /* force_reset   */
  0     /* is_mb         */
};

#define PDF_FONTTYPE_SIMPLE    1
#define PDF_FONTTYPE_BITMAP    2
#define PDF_FONTTYPE_COMPOSITE 3

struct dev_font {
  /* Needs to be big enough to hold name "Fxxx"
   * where xxx is number of largest font
   */
  char     short_name[7];      /* Resource name */
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

  /* if >= 0, index of a dev_font that really has the resource and used_chars */
  int      real_font_index;

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

  /* Compatibility */
  int      mapc;  /* Nasty workaround for Omega */

  /* There are no font metric format supporting four-bytes
   * charcter code. So we should provide an option to specify
   * UCS group and plane.
   */
  int      ucs_group;
  int      ucs_plane;

  int      is_unicode;
};
static struct dev_font *dev_fonts = NULL;

static int num_dev_fonts   = 0;
static int max_dev_fonts   = 0;
static int num_phys_fonts  = 0;

#define CURRENTFONT() ((text_state.font_id < 0) ? NULL : &(dev_fonts[text_state.font_id]))
#define GET_FONT(n)   (&(dev_fonts[(n)]))


static void
dev_set_text_matrix (spt_t xpos, spt_t ypos, double slant, double extend, int rotate)
{
  pdf_tmatrix tm;
  int         len = 0;

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
  tm.e = xpos * dev_unit.dvi2pts;
  tm.f = ypos * dev_unit.dvi2pts;

  format_buffer[len++] = ' ';
  len += pdf_sprint_matrix(format_buffer+len, &tm);
  format_buffer[len++] = ' ';
  format_buffer[len++] = 'T';
  format_buffer[len++] = 'm';

  pdf_doc_add_page_content(format_buffer, len);  /* op: Tm */

  text_state.ref_x = xpos;
  text_state.ref_y = ypos;
  text_state.matrix.slant  = slant;
  text_state.matrix.extend = extend;
  text_state.matrix.rotate = rotate;
}

/*
 * reset_text_state() outputs a BT and does any necessary coordinate
 * transformations to get ready to ship out text.
 */

static void
reset_text_state (void)
{
  /*
   * We need to reset the line matrix to handle slanted fonts.
   */
  pdf_doc_add_page_content(" BT", 3);  /* op: BT */
  /*
   * text_state.matrix is identity at top of page.
   * This sometimes write unnecessary "Tm"s when transition from
   * GRAPHICS_MODE to TEXT_MODE occurs.
   */
  if (text_state.force_reset ||
      text_state.matrix.slant  != 0.0 ||
      text_state.matrix.extend != 1.0 ||
      ROTATE_TEXT(text_state.matrix.rotate)) {
    dev_set_text_matrix(0, 0,
                        text_state.matrix.slant,
                        text_state.matrix.extend,
                        text_state.matrix.rotate);
  }
  text_state.ref_x = 0;
  text_state.ref_y = 0;
  text_state.offset   = 0;
  text_state.force_reset = 0;
}

static void
text_mode (void)
{
  switch (motion_state) {
  case TEXT_MODE:
    break;
  case STRING_MODE:
    pdf_doc_add_page_content(text_state.is_mb ? ">]TJ" : ")]TJ", 4);  /* op: TJ */
    break;
  case GRAPHICS_MODE:
    reset_text_state();
    break;
  }
  motion_state      = TEXT_MODE;
  text_state.offset = 0;
}

void
graphics_mode (void)
{
  switch (motion_state) {
  case GRAPHICS_MODE:
    break;
  case STRING_MODE:
    pdf_doc_add_page_content(text_state.is_mb ? ">]TJ" : ")]TJ", 4);  /* op: TJ */
    /* continue */
  case TEXT_MODE:
    pdf_doc_add_page_content(" ET", 3);  /* op: ET */
    text_state.force_reset =  0;
    text_state.font_id     = -1;
    break;
  }
  motion_state = GRAPHICS_MODE;
}

static void
start_string (spt_t xpos, spt_t ypos, double slant, double extend, int rotate)
{
  spt_t delx, dely, error_delx = 0, error_dely = 0;
  spt_t desired_delx, desired_dely;
  int   len = 0;

  delx = xpos - text_state.ref_x;
  dely = ypos - text_state.ref_y;
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
    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_delx, &error_dely);
    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_dely, &error_delx);
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
    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_delx, &error_dely);
    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_dely, &error_delx);
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

    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_delx, &error_delx);
    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_dely, &error_dely);
    break;
  case TEXT_WMODE_VV:
    /* Vertical font in vertical mode:
     *                         | 1  s/e|
     * d_user = d x I_vv = d x |       |
     *                         | 0  1/e|
     */
    desired_delx = delx;
    desired_dely = (spt_t)((dely + delx*slant)/extend);

    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_delx, &error_delx);
    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_dely, &error_dely);
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

    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_delx, &error_dely);
    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_dely, &error_delx);
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

    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_delx, &error_delx);
    format_buffer[len++] = ' ';
    len += dev_sprint_bp(format_buffer+len, desired_dely, &error_dely);
    error_delx = -error_delx;
    error_dely = -error_dely;
    break;
  }
  pdf_doc_add_page_content(format_buffer, len);  /* op: */
  /*
   * dvipdfm wrongly using "TD" in place of "Td".
   * The TD operator set leading, but we are not using T* etc.
   */
  pdf_doc_add_page_content(text_state.is_mb ? " Td[<" : " Td[(", 5);  /* op: Td */

  /* Error correction */
  text_state.ref_x = xpos - error_delx;
  text_state.ref_y = ypos - error_dely;

  text_state.offset   = 0;
}

static void
string_mode (spt_t xpos, spt_t ypos, double slant, double extend, int rotate)
{
  switch (motion_state) {
  case STRING_MODE:
    break;
  case GRAPHICS_MODE:
    reset_text_state();
    /* continue */
  case TEXT_MODE:
    if (text_state.force_reset) {
      dev_set_text_matrix(xpos, ypos, slant, extend, rotate);
      pdf_doc_add_page_content(text_state.is_mb ? "[<" : "[(", 2);  /* op: */
      text_state.force_reset = 0;
    } else {
      start_string(xpos, ypos, slant, extend, rotate);
    }
    break;
  }
  motion_state = STRING_MODE;
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
dev_set_font (int font_id)
{
  struct dev_font *font;
  struct dev_font *real_font;
  int    text_rotate;
  double font_scale;
  int    len;
  int    vert_dir, vert_font;

  /* text_mode() must come before text_state.is_mb is changed. */
  text_mode();

  font = GET_FONT(font_id);
  assert(font); /* Caller should check font_id. */

  if (font->real_font_index >= 0)
    real_font = GET_FONT(font->real_font_index);
  else
    real_font = font;

  text_state.is_mb = (font->format == PDF_FONTTYPE_COMPOSITE) ? 1 : 0;

  vert_font  = font->wmode ? 1 : 0;
  if (dev_param.autorotate) {
    vert_dir = text_state.dir_mode;
  } else {
    vert_dir = vert_font;
  }
  text_rotate = (vert_font << 2)|vert_dir;

  if (font->slant  != text_state.matrix.slant  ||
      font->extend != text_state.matrix.extend ||
      ANGLE_CHANGES(text_rotate, text_state.matrix.rotate)) {
    text_state.force_reset = 1;
  }
  text_state.matrix.slant  = font->slant;
  text_state.matrix.extend = font->extend;
  text_state.matrix.rotate = text_rotate;

  if (!real_font->resource) {
    real_font->resource   = pdf_get_font_reference(real_font->font_id);
    real_font->used_chars = pdf_get_font_usedchars(real_font->font_id);
  }

  if (!real_font->used_on_this_page) {
    pdf_doc_add_page_resource("Font",
                              real_font->short_name,
                              pdf_link_obj(real_font->resource));
    real_font->used_on_this_page = 1;
  }

  font_scale = (double) font->sptsize * dev_unit.dvi2pts;
  len  = sprintf(format_buffer, " /%s", real_font->short_name); /* space not necessary. */
  format_buffer[len++] = ' ';
  len += p_dtoa(font_scale, MIN(dev_unit.precision+1, DEV_PRECISION_MAX), format_buffer+len);
  format_buffer[len++] = ' ';
  format_buffer[len++] = 'T';
  format_buffer[len++] = 'f';
  pdf_doc_add_page_content(format_buffer, len);  /* op: Tf */

  if (font->bold > 0.0 || font->bold != text_state.bold_param) {
    if (font->bold <= 0.0)
      len = sprintf(format_buffer, " 0 Tr");
    else
      len = sprintf(format_buffer, " 2 Tr %.6f w", font->bold); /* _FIXME_ */
    pdf_doc_add_page_content(format_buffer, len);  /* op: Tr w */
  }
  text_state.bold_param = font->bold;

  text_state.font_id    = font_id;

  return  0;
}


/* Access text state parameters.
 */
int
pdf_dev_get_font_wmode (int font_id)
{
  struct dev_font *font;

  font = GET_FONT(font_id);
  if (font) {
    return font->wmode;
  }

  return 0;
}

static unsigned char sbuf0[FORMAT_BUF_SIZE];
static unsigned char sbuf1[FORMAT_BUF_SIZE];

static int
handle_multibyte_string (struct dev_font *font,
                         const unsigned char **str_ptr, size_t *str_len, int ctype)
{
  const unsigned char *p;
  size_t               i, length;

  p      = *str_ptr;
  length = *str_len;

  /* _FIXME_ */
  if (font->is_unicode) { /* UCS-4 */
    if (ctype == 1) {
      if (length * 4 >= FORMAT_BUF_SIZE) {
        dpx_warning("Too long string...");
        return -1;
      }
      for (i = 0; i < length; i++) {
        sbuf1[i*4  ] = font->ucs_group;
        sbuf1[i*4+1] = font->ucs_plane;
        sbuf1[i*4+2] = '\0';
        sbuf1[i*4+3] = p[i];
      }
      length *= 4;
    } else if (ctype == 2) {
      size_t len = 0;

      if (length * 2 >= FORMAT_BUF_SIZE) {
        dpx_warning("Too long string...");
        return -1;
      }
      for (i = 0; i < length; i += 2, len += 4) {
        sbuf1[len  ] = font->ucs_group;
        if ((p[i] & 0xf8) == 0xd8) {
          int c;
          /* Check for valid surrogate pair.  */
          if ((p[i] & 0xfc) != 0xd8 || i + 2 >= length || (p[i+2] & 0xfc) != 0xdc) {
            dpx_warning("Invalid surrogate p[%"PRIuZ"]=%02X...", i, p[i]);
            return -1;
          }
          c = (((p[i] & 0x03) << 10) | (p[i+1] << 2) | (p[i+2] & 0x03)) + 0x100;
          sbuf1[len+1] = (c >> 8) & 0xff;
          sbuf1[len+2] = c & 0xff;
          i += 2;
        } else {
          sbuf1[len+1] = font->ucs_plane;
          sbuf1[len+2] = p[i];
        }
        sbuf1[len+3] = p[i+1];
      }
      length = len;
    }
    p = sbuf1;
  } else if (ctype == 1 && font->mapc >= 0) {
    /* Omega workaround...
     * Translate single-byte chars to double byte code space.
     */
    if (length * 2 >= FORMAT_BUF_SIZE) {
      dpx_warning("Too long string...");
      return -1;
    }
    for (i = 0; i < length; i++) {
      sbuf1[i*2  ] = (font->mapc & 0xff);
      sbuf1[i*2+1] = p[i];
    }
    length *= 2;
    p       = sbuf1;
  }

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


static pdf_coord *dev_coords = NULL;
static int num_dev_coords = 0;
static int max_dev_coords = 0;

void pdf_dev_get_coord(double *xpos, double *ypos)
{
  if (num_dev_coords > 0) {
    *xpos = dev_coords[num_dev_coords-1].x;
    *ypos = dev_coords[num_dev_coords-1].y;
  } else {
    *xpos = *ypos = 0.0;
  }
}

void pdf_dev_push_coord(double xpos, double ypos)
{
  if (num_dev_coords >= max_dev_coords) {
    max_dev_coords += 4;
    dev_coords = RENEW(dev_coords, max_dev_coords, pdf_coord);
  }
  dev_coords[num_dev_coords].x = xpos;
  dev_coords[num_dev_coords].y = ypos;
  num_dev_coords++;
}

void pdf_dev_pop_coord(void)
{
  if (num_dev_coords > 0) num_dev_coords--;
}

/*
 * ctype:
 *  -1 input string contains 2-byte Freetype glyph index values
 *     (XeTeX only)
 *  0  byte-width of char can be variable and input string
 *     is properly encoded.
 *  n  Single character cosumes n bytes in input string.
 *
 * _FIXME_
 * -->
 * selectfont(font_name, point_size) and show_string(pos, string)
 */
void
pdf_dev_set_string (spt_t xpos, spt_t ypos,
                    const void *instr_ptr, size_t instr_len,
                    spt_t width,
                    int   font_id, int ctype)
{
  struct dev_font *font;
  struct dev_font *real_font;
  const unsigned char *str_ptr; /* Pointer to the reencoded string. */
  size_t           length, i, len = 0;
  spt_t            kern, delh, delv;
  spt_t            text_xorigin;
  spt_t            text_yorigin;

  if (font_id < 0 || font_id >= num_dev_fonts) {
    _tt_abort("Invalid font: %d (%d)", font_id, num_dev_fonts);
  }
  if (font_id != text_state.font_id) {
    dev_set_font(font_id);
  }

  font = CURRENTFONT();
  if (!font) {
    _tt_abort("Currentfont not set.");
  }

  if (font->real_font_index >= 0)
    real_font = GET_FONT(font->real_font_index);
  else
    real_font = font;

  text_xorigin = text_state.ref_x;
  text_yorigin = text_state.ref_y;

  str_ptr = instr_ptr;
  length  = instr_len;

  if (font->format == PDF_FONTTYPE_COMPOSITE) {
    if (handle_multibyte_string(font, &str_ptr, &length, ctype) < 0) {
      _tt_abort("Error in converting input string...");
    }
    if (real_font->used_chars != NULL) {
      for (i = 0; i < length; i += 2) {
        unsigned short cid = (str_ptr[i] << 8) | str_ptr[i + 1];
        add_to_used_chars2(real_font->used_chars, cid);
      }
    }
  } else {
    if (real_font->used_chars != NULL) {
      for (i = 0; i < length; i++)
        real_font->used_chars[str_ptr[i]] = 1;
    }
  }

  if (num_dev_coords > 0) {
    xpos -= bpt2spt(dev_coords[num_dev_coords-1].x);
    ypos -= bpt2spt(dev_coords[num_dev_coords-1].y);
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

  if (text_state.dir_mode==0) {
    /* Left-to-right */
    delh = text_xorigin + text_state.offset - xpos;
    delv = ypos - text_yorigin;
  } else if (text_state.dir_mode==1) {
    /* Top-to-bottom */
    delh = ypos - text_yorigin + text_state.offset;
    delv = xpos - text_xorigin;
  } else {
    /* Bottom-to-top */
    delh = ypos + text_yorigin + text_state.offset;
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

  if (text_state.force_reset ||
      labs(delv) > dev_unit.min_bp_val ||
      labs(delh) > WORD_SPACE_MAX(font)) {
    text_mode();
    kern = 0;
  } else {
    kern = (spt_t) (1000.0 / font->extend * delh / font->sptsize);
  }

  /* Inaccucary introduced by rounding of character width appears within
   * single text block. There are point_size/1000 rounding error per character.
   * If you really care about accuracy, you should compensate this here too.
   */
  if (motion_state != STRING_MODE)
    string_mode(xpos, ypos,
                font->slant, font->extend, text_state.matrix.rotate);
  else if (kern != 0) {
    /*
     * Same issues as earlier. Use floating point for simplicity.
     * This routine needs to be fast, so we don't call sprintf() or strcpy().
     */
    text_state.offset -=
      (spt_t) (kern * font->extend * (font->sptsize / 1000.0));
    format_buffer[len++] = text_state.is_mb ? '>' : ')';
    if (font->wmode)
      len += p_itoa(-kern, format_buffer + len);
    else {
      len += p_itoa( kern, format_buffer + len);
    }
    format_buffer[len++] = text_state.is_mb ? '<' : '(';
    pdf_doc_add_page_content(format_buffer, len);  /* op: */
    len = 0;
  }

  if (text_state.is_mb) {
    if (FORMAT_BUF_SIZE - len < 2 * length)
      _tt_abort("Buffer overflow...");
    for (i = 0; i < length; i++) {
      int first, second;

      first  = (str_ptr[i] >> 4) & 0x0f;
      second = str_ptr[i] & 0x0f;
      format_buffer[len++] = ((first >= 10)  ? first  + 'W' : first  + '0');
      format_buffer[len++] = ((second >= 10) ? second + 'W' : second + '0');
    }
  } else {
    len += pdfobj_escape_str(format_buffer + len,
                             FORMAT_BUF_SIZE - len, str_ptr, length);
  }
  /* I think if you really care about speed, you should avoid memcopy here. */
  pdf_doc_add_page_content(format_buffer, len);  /* op: */

  text_state.offset += width;
}

void
pdf_init_device (double dvi2pts, int precision, int black_and_white)
{
  if (precision < 0 ||
      precision > DEV_PRECISION_MAX)
    dpx_warning("Number of decimal digits out of range [0-%d].",
         DEV_PRECISION_MAX);

  if (precision < 0) {
    dev_unit.precision  = 0;
  } else if (precision > DEV_PRECISION_MAX) {
    dev_unit.precision  = DEV_PRECISION_MAX;
  } else {
    dev_unit.precision  = precision;
  }
  dev_unit.dvi2pts      = dvi2pts;
  dev_unit.min_bp_val   = (int) ROUND(1.0/(ten_pow[dev_unit.precision]*dvi2pts), 1);
  if (dev_unit.min_bp_val < 0)
    dev_unit.min_bp_val = -dev_unit.min_bp_val;

  dev_param.colormode = (black_and_white ? 0 : 1);

  graphics_mode();
  pdf_color_clear_stack();
  pdf_dev_init_gstates();

  num_dev_fonts  = max_dev_fonts = 0;
  dev_fonts      = NULL;
  num_dev_coords = max_dev_coords = 0;
  dev_coords     = NULL;
}

void
pdf_close_device (void)
{
  if (dev_fonts) {
    int    i;

    for (i = 0; i < num_dev_fonts; i++) {
      free(dev_fonts[i].tex_name);
      pdf_release_obj(dev_fonts[i].resource);
      dev_fonts[i].tex_name = NULL;
      dev_fonts[i].resource = NULL;
    }
    free(dev_fonts);
  }
  free(dev_coords);
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
  int  i;

  for (i = 0; i < num_dev_fonts; i++) {
    dev_fonts[i].used_on_this_page = 0;
  }

  text_state.font_id       = -1;

  text_state.matrix.slant  = 0.0;
  text_state.matrix.extend = 1.0;
  text_state.matrix.rotate = TEXT_WMODE_HH;

  if (newpage)
    text_state.bold_param  = 0.0;

  text_state.is_mb         = 0;
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
  graphics_mode();

  text_state.force_reset  = 0;

  pdf_dev_gsave();
  pdf_dev_concat(M);

  pdf_dev_reset_fonts(1);
  pdf_dev_reset_color(0);
}

void
pdf_dev_eop (void)
{
  int  depth;

  graphics_mode();

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
  int              i;
  fontmap_rec     *mrec;
  struct dev_font *font;

  if (!font_name)
    return  -1;

  if (ptsize == 0) {
    _tt_abort("pdf_dev_locate_font() called with the zero ptsize.");
  }

  for (i = 0; i < num_dev_fonts; i++) {
    if (streq_ptr(font_name, dev_fonts[i].tex_name)) {
      if (ptsize == dev_fonts[i].sptsize)
        return i; /* found a dev_font that matches the request */
      if (dev_fonts[i].format != PDF_FONTTYPE_BITMAP)
        break; /* new dev_font will share pdf resource with /i/ */
    }
  }

  /*
   * Make sure we have room for a new one, even though we may not
   * actually create one.
   */
  if (num_dev_fonts >= max_dev_fonts) {
    max_dev_fonts += 16;
    dev_fonts      = RENEW(dev_fonts, max_dev_fonts, struct dev_font);
  }

  font = &dev_fonts[num_dev_fonts];

  /* New font */
  mrec = pdf_lookup_fontmap_record(font_name);

  if (dpx_conf.verbose_level > 1)
    print_fontmap(font_name, mrec);

  font->font_id = pdf_font_findresource(font_name, ptsize * dev_unit.dvi2pts, mrec);
  if (font->font_id < 0)
    return  -1;

  /* We found device font here. */
  if (i < num_dev_fonts) {
    font->real_font_index = i;
    strcpy(font->short_name, dev_fonts[i].short_name);
  }
  else {
    font->real_font_index = -1;
    font->short_name[0] = 'F';
    p_itoa(num_phys_fonts + 1, &font->short_name[1]); /* NULL terminated here */
    num_phys_fonts++;
  }

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
  font->mapc       = -1;
  font->is_unicode = 0;
  font->ucs_group  = 0;
  font->ucs_plane  = 0;

  if (mrec) {
    font->extend = mrec->opt.extend;
    font->slant  = mrec->opt.slant;
    font->bold   = mrec->opt.bold;
    if (mrec->opt.mapc >= 0)
      font->mapc = (mrec->opt.mapc >> 8) & 0xff;
    else {
      font->mapc = -1;
    }
    if (streq_ptr(mrec->enc_name, "unicode")) {
      font->is_unicode   = 1;
      if (mrec->opt.mapc >= 0) {
        font->ucs_group  = (mrec->opt.mapc >> 24) & 0xff;
        font->ucs_plane  = (mrec->opt.mapc >> 16) & 0xff;
      } else {
        font->ucs_group  = 0;
        font->ucs_plane  = 0;
      }
    } else {
      font->is_unicode   = 0;
    }
  }

  return  num_dev_fonts++;
}


/* This does not remember current stroking width. */
static int
dev_sprint_line (char *buf, spt_t width,
                 spt_t p0_x, spt_t p0_y, spt_t p1_x, spt_t p1_y)
{
  int    len = 0;
  double w;

  w = width * dev_unit.dvi2pts;

  len += p_dtoa(w, MIN(dev_unit.precision+1, DEV_PRECISION_MAX), buf+len);
  buf[len++] = ' ';
  buf[len++] = 'w';
  buf[len++] = ' ';
  len += dev_sprint_bp(buf+len, p0_x, NULL);
  buf[len++] = ' ';
  len += dev_sprint_bp(buf+len, p0_y, NULL);
  buf[len++] = ' ';
  buf[len++] = 'm';
  buf[len++] = ' ';
  len += dev_sprint_bp(buf+len, p1_x, NULL);
  buf[len++] = ' ';
  len += dev_sprint_bp(buf+len, p1_y, NULL);
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
  int    len = 0;
  double width_in_bp;

  if (num_dev_coords > 0) {
    xpos -= bpt2spt(dev_coords[num_dev_coords-1].x);
    ypos -= bpt2spt(dev_coords[num_dev_coords-1].y);
  }

  graphics_mode();

  format_buffer[len++] = ' ';
  format_buffer[len++] = 'q';
  format_buffer[len++] = ' ';
  /* Don't use too thick line. */
  width_in_bp = ((width < height) ? width : height) * dev_unit.dvi2pts;
  if (width_in_bp < 0.0 || /* Shouldn't happen */
      width_in_bp > PDF_LINE_THICKNESS_MAX) {
    pdf_rect rect;

    rect.llx =  dev_unit.dvi2pts * xpos;
    rect.lly =  dev_unit.dvi2pts * ypos;
    rect.urx =  dev_unit.dvi2pts * width;
    rect.ury =  dev_unit.dvi2pts * height;
    len += pdf_sprint_rect(format_buffer+len, &rect);
    format_buffer[len++] = ' ';
    format_buffer[len++] = 'r';
    format_buffer[len++] = 'e';
    format_buffer[len++] = ' ';
    format_buffer[len++] = 'f';
  } else {
    if (width > height) {
      /* NOTE:
       *  A line width of 0 denotes the thinnest line that can be rendered at
       *  device resolution. See, PDF Reference Manual 4th ed., sec. 4.3.2,
       *  "Details of Graphics State Parameters", p. 185.
       */
      if (height < dev_unit.min_bp_val) {
        dpx_warning("Too thin line: height=%d (%g bp)", height, width_in_bp);
        dpx_warning("Please consider using \"-d\" option.");
      }
      len += dev_sprint_line(format_buffer+len,
                             height,
                             xpos,
                             ypos + height/2,
                             xpos + width,
                             ypos + height/2);
    } else {
      if (width < dev_unit.min_bp_val) {
        dpx_warning("Too thin line: width=%d (%g bp)", width, width_in_bp);
        dpx_warning("Please consider using \"-d\" option.");
      }
      len += dev_sprint_line(format_buffer+len,
                             width,
                             xpos + width/2,
                             ypos,
                             xpos + width/2,
                             ypos + height);
    }
  }
  format_buffer[len++] = ' ';
  format_buffer[len++] = 'Q';
  pdf_doc_add_page_content(format_buffer, len);  /* op: q re f Q */
}

/* Rectangle in device space coordinate. */
void
pdf_dev_set_rect (pdf_rect *rect,
                  spt_t x_user, spt_t y_user,
                  spt_t width,  spt_t height, spt_t depth)
{
  double      dev_x, dev_y;
  pdf_coord   p0, p1, p2, p3;
  double      min_x, min_y, max_x, max_y;

  dev_x = x_user * dev_unit.dvi2pts;
  dev_y = y_user * dev_unit.dvi2pts;
  if (text_state.dir_mode) {
    p0.x = dev_x - dev_unit.dvi2pts * depth;
    p0.y = dev_y - dev_unit.dvi2pts * width;
    p1.x = dev_x + dev_unit.dvi2pts * height;
    p1.y = p0.y;
    p2.x = p1.x;
    p2.y = dev_y;
    p3.x = p0.x;
    p3.y = p2.y;
  } else {
    p0.x = dev_x;
    p0.y = dev_y - dev_unit.dvi2pts * depth;
    p1.x = dev_x + dev_unit.dvi2pts * width;
    p1.y = p0.y;
    p2.x = p1.x;
    p2.y = dev_y + dev_unit.dvi2pts * height;
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
  return text_state.dir_mode;
}

void
pdf_dev_set_dirmode (int text_dir)
{
  struct dev_font *font;
  int text_rotate;
  int vert_dir, vert_font;

  font = CURRENTFONT();

  vert_font = (font && font->wmode) ? 1 : 0;
  if (dev_param.autorotate) {
    vert_dir = text_dir;
  } else {
    vert_dir = vert_font;
  }
  text_rotate = (vert_font << 2)|vert_dir;

  if (font &&
      ANGLE_CHANGES(text_rotate, text_state.matrix.rotate)) {
    text_state.force_reset = 1;
  }

  text_state.matrix.rotate = text_rotate;
  text_state.dir_mode      = text_dir;
}

static void
dev_set_param_autorotate (int auto_rotate)
{
  struct dev_font *font;
  int    text_rotate, vert_font, vert_dir;

  font = CURRENTFONT();

  vert_font = (font && font->wmode) ? 1 : 0;
  if (auto_rotate) {
    vert_dir = text_state.dir_mode;
  } else {
    vert_dir = vert_font;
  }
  text_rotate = (vert_font << 2)|vert_dir;

  if (ANGLE_CHANGES(text_rotate, text_state.matrix.rotate)) {
    text_state.force_reset = 1;
  }
  text_state.matrix.rotate = text_rotate;
  dev_param.autorotate     = auto_rotate;
}

int
pdf_dev_get_param (int param_type)
{
  int value = 0;

  switch (param_type) {
  case PDF_DEV_PARAM_AUTOROTATE:
    value = dev_param.autorotate;
    break;
  case PDF_DEV_PARAM_COLORMODE:
    value = dev_param.colormode;
    break;
  default:
    _tt_abort("Unknown device parameter: %d", param_type);
  }

  return value;
}

void
pdf_dev_set_param (int param_type, int value)
{
  switch (param_type) {
  case PDF_DEV_PARAM_AUTOROTATE:
    dev_set_param_autorotate(value);
    break;
  case PDF_DEV_PARAM_COLORMODE:
    dev_param.colormode = value; /* 0 for B&W */
    break;
  default:
    _tt_abort("Unknown device parameter: %d", param_type);
  }

  return;
}


int
pdf_dev_put_image (int             id,
                   transform_info *p,
                   double          ref_x,
                   double          ref_y)
{
  char        *res_name;
  pdf_tmatrix  M, M1;
  pdf_rect     r;
  int          len = 0;

  if (num_dev_coords > 0) {
    ref_x -= dev_coords[num_dev_coords-1].x;
    ref_y -= dev_coords[num_dev_coords-1].y;
  }

  pdf_copymatrix(&M, &(p->matrix));
  M.e += ref_x; M.f += ref_y;
  /* Just rotate by -90, but not tested yet. Any problem if M has scaling? */
  if (dev_param.autorotate &&
      text_state.dir_mode) {
    double tmp;
    tmp = -M.a; M.a = M.b; M.b = tmp;
    tmp = -M.c; M.c = M.d; M.d = tmp;
  }

  graphics_mode();
  pdf_dev_gsave();

  pdf_ximage_scale_image(id, &M1, &r, p);
  pdf_concatmatrix(&M, &M1);
  pdf_dev_concat(&M);

  /* Clip */
  if (p->flags & INFO_DO_CLIP) {
    pdf_dev_rectclip(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);
  }

  res_name = pdf_ximage_get_resname(id);
  len = sprintf(work_buffer, " /%s Do", res_name);
  pdf_doc_add_page_content(work_buffer, len);  /* op: Do */

  pdf_dev_grestore();

  pdf_doc_add_page_resource("XObject",
                            res_name,
                            pdf_ximage_get_reference(id));

  if (dvi_is_tracking_boxes()) {
    pdf_tmatrix P;
    unsigned int i;
    pdf_rect rect;
    pdf_coord corner[4];

    pdf_dev_set_rect(&rect, 65536 * ref_x, 65536 * ref_y,
        65536 * (r.urx - r.llx), 65536 * (r.ury - r.lly), 0);

    corner[0].x = rect.llx; corner[0].y = rect.lly;
    corner[1].x = rect.llx; corner[1].y = rect.ury;
    corner[2].x = rect.urx; corner[2].y = rect.ury;
    corner[3].x = rect.urx; corner[3].y = rect.lly;

    pdf_copymatrix(&P, &(p->matrix));
    for (i = 0; i < 4; ++i) {
      corner[i].x -= rect.llx;
      corner[i].y -= rect.lly;
      pdf_dev_transform(&(corner[i]), &P);
      corner[i].x += rect.llx;
      corner[i].y += rect.lly;
    }

    rect.llx = corner[0].x;
    rect.lly = corner[0].y;
    rect.urx = corner[0].x;
    rect.ury = corner[0].y;
    for (i = 0; i < 4; ++i) {
      if (corner[i].x < rect.llx)
        rect.llx = corner[i].x;
      if (corner[i].x > rect.urx)
        rect.urx = corner[i].x;
      if (corner[i].y < rect.lly)
        rect.lly = corner[i].y;
      if (corner[i].y > rect.ury)
        rect.ury = corner[i].y;
    }

    pdf_doc_expand_box(&rect);
  }

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
  int len, i, pdf_doc_enc = 1;

  /* check whether we can use PDFDocEncoding for this string
     (we punt on the 0x80..0xA0 range that does not directly correspond to unicode)  */
  for (i = 0; i < count; i++) {
    if (unicodes[i] > 0xff || (unicodes[i] > 0x7f && unicodes[i] < 0xa1)) {
      pdf_doc_enc = 0;
      break;
    }
  }

  graphics_mode();

  len = sprintf(work_buffer, "\n/Span<</ActualText(");
  if (!pdf_doc_enc) {
    len += sprintf(work_buffer + len, "\xFE\xFF");
  }
  pdf_doc_add_page_content(work_buffer, len);

  while (count-- > 0) {
    unsigned char s[2] = { *unicodes >> 8, *unicodes & 0xff };
    i = pdf_doc_enc; /* if using PDFDocEncoding, we only care about the low 8 bits,
                        so start with the second byte of our pair */
    len = 0;
    for (; i < 2; i++) {
      unsigned char c = s[i];
      if (c == '(' || c == ')' || c == '\\') {
        len += sprintf(work_buffer + len, "\\%c", c);
      } else if (c < ' ') {
        len += sprintf(work_buffer + len, "\\%03o", c);
      } else {
        len += sprintf(work_buffer + len, "%c", c);
      }
    }
    pdf_doc_add_page_content(work_buffer, len);
    ++unicodes;
  }

  len = sprintf(work_buffer, ")>>BDC");
  pdf_doc_add_page_content(work_buffer, len);
}

void
pdf_dev_end_actualtext (void)
{
  graphics_mode();

  pdf_doc_add_page_content(" EMC", 4);
}


void
pdf_dev_reset_global_state(void)
{
  dev_fonts = NULL;

  num_dev_fonts   = 0;
  max_dev_fonts   = 0;
  num_phys_fonts  = 0;
}
