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

/* No page independence here...
 */

#include "dpx-pdfcolor.h"

#include <assert.h>
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfresource.h"

int
pdf_color_type (const pdf_color *color)
{
  assert(color);

  return color->type;
}

int
pdf_color_rgbcolor (pdf_color *color, double r, double g, double b)
{
  assert(color);

  if (r < 0.0 || r > 1.0) {
    dpx_warning("Invalid color value specified: red=%g",   r);
    return -1;
  }
  if (g < 0.0 || g > 1.0) {
    dpx_warning("Invalid color value specified: green=%g", g);
    return -1;
  }
  if (b < 0.0 || b > 1.0) {
    dpx_warning("Invalid color value specified: blue=%g", b);
    return -1;
  }
  color->values[0] = r;
  color->values[1] = g;
  color->values[2] = b;

  color->res_id = -1;
  color->type = PDF_COLORSPACE_TYPE_RGB;
  color->num_components = 3;

  color->spot_color_name = NULL;

  return 0;
}

int
pdf_color_cmykcolor (pdf_color *color,
                     double c, double m, double y, double k)
{
  assert(color);

  if (c < 0.0 || c > 1.0) {
    dpx_warning("Invalid color value specified: cyan=%g", c);
    return -1;
  }
  if (m < 0.0 || m > 1.0) {
    dpx_warning("Invalid color value specified: magenta=%g", m);
    return -1;
  }
  if (y < 0.0 || y > 1.0) {
    dpx_warning("Invalid color value specified: yellow=%g", y);
    return -1;
  }
  if (k < 0.0 || k > 1.0) {
    dpx_warning("Invalid color value specified: black=%g", k);
    return -1;
  }

  color->values[0] = c;
  color->values[1] = m;
  color->values[2] = y;
  color->values[3] = k;

  color->res_id = -1;
  color->type = PDF_COLORSPACE_TYPE_CMYK;
  color->num_components = 4;

  color->spot_color_name = NULL;

  return 0;
}

int
pdf_color_graycolor (pdf_color *color, double g)
{
  assert(color);

  if (g < 0.0 || g > 1.0) {
    dpx_warning("Invalid color value specified: gray=%g", g);
    return -1;
  }

  color->values[0] = g;

  color->res_id = -1;
  color->type = PDF_COLORSPACE_TYPE_GRAY;
  color->num_components = 1;

  color->spot_color_name = NULL;

  return 0;
}

int
pdf_color_spotcolor (pdf_color *color, char* name, double c)
{
  assert(color);

  if (c < 0.0 || c > 1.0) {
    dpx_warning("Invalid color value specified: grade=%g", c);
    return -1;
  }

  color->values[0] = c;
  color->values[1] = 0.0; /* Dummy */

  color->res_id = 1; /* ??? */
  color->type = PDF_COLORSPACE_TYPE_SPOT;
  color->num_components = 2;

  color->spot_color_name = name;

  return 0;
}


void
pdf_color_copycolor (pdf_color *color1, const pdf_color *color2)
{
  assert(color1 && color2);

  memcpy(color1, color2, sizeof(pdf_color));
}

/* Brighten up a color. f == 0 means no change, f == 1 means white. */
void
pdf_color_brighten_color (pdf_color *dst, const pdf_color *src, double f)
{
  assert(dst && src);

  if (src->type != PDF_COLORSPACE_TYPE_RGB  &&
      src->type != PDF_COLORSPACE_TYPE_CMYK &&
      src->type != PDF_COLORSPACE_TYPE_GRAY) {
    pdf_color_copycolor(dst, src);
    return;
  }

  if (f == 1.0) {
    pdf_color_white(dst);
  } else {
    double f0, f1;
    int n;

    pdf_color_copycolor(dst, src);
    n = src->num_components;
    f1 = n == 4 ? 0.0 : f;  /* n == 4 is CMYK, others are RGB and Gray */
    f0 = 1.0-f;

    while (n--)
      dst->values[n] = f0 * src->values[n] + f1;
  }
}

/* TODO: remove "is_white"...
 * pdfdoc.c only use this but not necessary if we have a flag have_bgcolor
 * to indicate if bg color was set.
 */
bool
pdf_color_is_white (const pdf_color *color)
{
  int n;
  double f;

  assert(color);

  switch (color->type) {
  case PDF_COLORSPACE_TYPE_GRAY:
  case PDF_COLORSPACE_TYPE_RGB:
    f = 1.0;
    break;
  case PDF_COLORSPACE_TYPE_CMYK:
    f = 0.0;
    break;
  default:
    return false;
  }

  n = color->num_components;
  while (n--)
    if (color->values[n] != f)
      return false;

  return true;
}

/* TODO: make_resource_name() in pdfresource.c with configurable prefix. */
int
pdf_color_set_color (const pdf_color *color, char *buffer, size_t buffer_len,char mask)
{
  int len = 0;
  int i;

  {
    size_t estimate = 0;
    if (color->num_components > 0) {
      estimate += 5 * (color->num_components + 1) + 4; /* Assuming color values [0, 1]... */
    }
    estimate += strlen(" /DeiceGray CS");
    if (estimate + 1 > buffer_len) {
      dpx_warning("Not enough buffer space allocated for writing set_color op...");
      return 0;
    }
  }

  switch (pdf_color_type(color)) {
  case PDF_COLORSPACE_TYPE_DEVICEGRAY:
    {
      len += sprintf(buffer+len, " /DeviceGray %c%c", 'C' | mask, 'S' | mask);
      for (i = 0; i < color->num_components; i++) {
        len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
      len += sprintf(buffer+len, " %c%c", 'S' | mask, 'C' | mask);
    }
    break;
  case PDF_COLORSPACE_TYPE_DEVICERGB:
    {
      len += sprintf(buffer+len, " /DeviceRGB %c%c", 'C' | mask, 'S' | mask);
      for (i = 0; i < color->num_components; i++) {
        len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
      len += sprintf(buffer+len, " %c%c", 'S' | mask, 'C' | mask);
    }
    break;
  case PDF_COLORSPACE_TYPE_DEVICECMYK:
    {
      len += sprintf(buffer+len, " /DeviceCMYK %c%c", 'C' | mask, 'S' | mask);
      for (i = 0; i < color->num_components; i++) {
        len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
      len += sprintf(buffer+len, " %c%c", 'S' | mask, 'C' | mask);
    }
    break;
  case PDF_COLORSPACE_TYPE_GRAY:
    {
      for (i = 0; i < color->num_components; i++) {
        len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
      len += sprintf(buffer+len, " %c", 'G' | mask);
    }
    break;
  case PDF_COLORSPACE_TYPE_RGB:
    {
      for (i = 0; i < color->num_components; i++) {
        len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
      len += sprintf(buffer+len, " %c%c", 'R' | mask, 'G' | mask);
    }
    break;
  case PDF_COLORSPACE_TYPE_CMYK:
    {
      for (i = 0; i < color->num_components; i++) {
        len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
      len += sprintf(buffer+len, " %c", 'K' | mask);
    }
    break;
  case PDF_COLORSPACE_TYPE_SPOT:
    {
      len = sprintf(buffer+len,
                    " /%s %c%c %g %c%c",
                    color->spot_color_name,
                    'C' | mask, 'S' | mask,
                    ROUND(color->values[0], 0.001),
                    'S' | mask, 'C' | mask);
    }
    break;
  case PDF_COLORSPACE_TYPE_CALGRAY:
  case PDF_COLORSPACE_TYPE_CALRGB:
  case PDF_COLORSPACE_TYPE_LAB:
  case PDF_COLORSPACE_TYPE_INDEXED:
    {
      char res_name[16];

      snprintf(res_name, 16, "XC%d", color->res_id & 0xffff); /* TODO: Upper 16bits for category ID. See, pdfresource.c */
      res_name[15] = 0;
      len += sprintf(buffer+len, " /%s %c%c", res_name, 'C' | mask, 'S' | mask);
      for (i = 0; i < color->num_components; i++) {
        len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
      len += sprintf(buffer+len, " %c%c", 'S' | mask, 'C' | mask);
      pdf_doc_add_page_resource("ColorSpace", res_name, pdf_get_resource_reference(color->res_id));
    }
    break;
  case PDF_COLORSPACE_TYPE_PATTERN:
    {
      char res_name[16];

      if (color->res_id < 0) {
        len += sprintf(buffer+len, " /Pattern %c%c", 'C' | mask, 'S' | mask);
        /* no color value but just a name */
      } else {
        snprintf(res_name, 16, "XC%d", color->res_id & 0xffff); /* TODO: Upper 16bits for category ID. See, pdfresource.c */
        res_name[15] = 0;
        len += sprintf(buffer+len, " /%s %c%c", res_name, 'C' | mask, 'S' | mask);
        for (i = 0; i < color->num_components; i++) {
          len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
        }
        pdf_doc_add_page_resource("ColorSpace", res_name, pdf_get_resource_reference(color->res_id));
      }
      snprintf(res_name, 16, "XP%d", color->pattern_id & 0xffff); /* TODO: see below */
      res_name[15] = 0;
      len += sprintf(buffer+len, " /%s %c%c%c", res_name, 'S' | mask, 'C' | mask, 'N' | mask);

      pdf_doc_add_page_resource("Pattern", res_name, pdf_get_resource_reference(color->pattern_id));
    }
    break;
  default:
    {
      char res_name[16];

      snprintf(res_name, 8, "XC%d", color->res_id & 0xffff); /* TODO: Upper 16bits for category ID. See, pdfresource.c */
      res_name[8] = 0;
      len += sprintf(buffer+len, " /%s %c%c", res_name, 'C' | mask, 'S' | mask);
      for (i = 0; i < color->num_components; i++) {
        len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
      len += sprintf(buffer+len, " %c%c%c", 'S' | mask, 'C' | mask, 'N' | mask);
      pdf_doc_add_page_resource("ColorSpace", res_name, pdf_get_resource_reference(color->res_id));
    }
  }

  return len;
}

pdf_color current_fill   = {
  -1,
  PDF_COLORSPACE_TYPE_GRAY,
  1,
  NULL,
  {0.0},
  -1
};

pdf_color current_stroke = {
  -1,
  PDF_COLORSPACE_TYPE_GRAY,
  1,
  NULL,
  {0.0},
  -1
};

/*
 * This routine is not a real color matching.
 */
int
pdf_color_compare (const pdf_color *color1, const pdf_color *color2)
{
  int n;

  /* Tectonic: the logic in upstream cannot be right; I think our
   * modified version gives what's intended? */
  if ((color1->type != PDF_COLORSPACE_TYPE_GRAY &&
      color1->type != PDF_COLORSPACE_TYPE_RGB  &&
      color1->type != PDF_COLORSPACE_TYPE_CMYK &&
      color1->type != PDF_COLORSPACE_TYPE_SPOT) ||
      (color2->type != PDF_COLORSPACE_TYPE_GRAY &&
      color2->type != PDF_COLORSPACE_TYPE_RGB  &&
      color2->type != PDF_COLORSPACE_TYPE_CMYK &&
      color2->type != PDF_COLORSPACE_TYPE_SPOT) ||
      color1->type != color2->type) {
    return -1;
  }

  n = color1->num_components;
  while (--n >= 0) {
    if (color1->values[n] != color2->values[n])
      return -1;
  }

  if (color1->spot_color_name && color2->spot_color_name)
    return strcmp(color1->spot_color_name, color2->spot_color_name);

  return 0;
}

/* Dvipdfm special */
pdf_color default_color = {
  -1,
  PDF_COLORSPACE_TYPE_GRAY,
  1,
  NULL,
  {0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0},
  -1
};

#define DEV_COLOR_STACK_MAX 128

static struct {
  int       current;
  pdf_color stroke[DEV_COLOR_STACK_MAX];
  pdf_color fill[DEV_COLOR_STACK_MAX];
} color_stack;

void
pdf_color_clear_stack (void)
{
  if (color_stack.current > 0) {
    dpx_warning("You've mistakenly made a global color change within nested colors.");
  }
  while (color_stack.current--) {
    free(color_stack.stroke[color_stack.current].spot_color_name);
    free(color_stack.fill[color_stack.current].spot_color_name);
  }
  color_stack.current = 0;
  pdf_color_black(color_stack.stroke);
  pdf_color_black(color_stack.fill);
  return;
}

void
pdf_color_set (pdf_color *sc, pdf_color *fc)
{
  pdf_color_copycolor(&color_stack.stroke[color_stack.current], sc);
  pdf_color_copycolor(&color_stack.fill[color_stack.current], fc);
  pdf_dev_reset_color(1);
}

void
pdf_color_push (pdf_color *sc, pdf_color *fc)
{
  if (color_stack.current >= DEV_COLOR_STACK_MAX-1) {
    dpx_warning("Color stack overflow. Just ignore.");
  } else {
    color_stack.current++;
    pdf_color_set(sc, fc);
  }
  return;
}

void
pdf_color_pop (void)
{
  if (color_stack.current <= 0) {
    dpx_warning("Color stack underflow. Just ignore.");
  } else {
    color_stack.current--;
    pdf_dev_reset_color(1);
  }
  return;
}

void
pdf_color_get_current (pdf_color **sc, pdf_color **fc)
{
  *sc = &color_stack.stroke[color_stack.current];
  *fc = &color_stack.fill[color_stack.current];
  return;
}

/***************************** COLOR SPACE *****************************/

/* Currently only for ICCBased color loaded while reading images */
static int pdf_colorspace_defineresource (const char *ident, int subtype, void *cdata, pdf_obj *resource);
static int pdf_colorspace_findresource   (const char *ident, int subtype, const void *cdata);

static unsigned char  nullbytes16[16] = {
  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
};

static struct
{
  int  major;
  int  minor;
} icc_versions[] = {
  {0, 0}, /* PDF-1.0, we don't support them */
  {0, 0}, /* PDF-1.1, we don't support them */
  {0, 0}, /* PDF-1.2, we don't support them */
  {0x02, 0x10}, /* PDF-1.3 */
  {0x02, 0x20}, /* PDF-1.4 */
  {0x04, 0x00}, /* PDF-1.5 */
  {0x04, 0x00}, /* PDF-1.6 */
  {0x04, 0x20}, /* PDF-1.7 */
  {0x04, 0x20}, /* Dummy(1.8)*/
  {0x04, 0x20}, /* Dummy(1.9) */
  {0x04, 0x20}  /* PDF-2.0 */
};

static int
iccp_version_supported (int major, int minor)
{
  int  idx;

  idx = pdf_get_version() - 10;
  if (idx < 11) {
    if (icc_versions[idx].major < major)
      return 0;
    else if (icc_versions[idx].major == major &&
             icc_versions[idx].minor <  minor)
      return 0;
    else {
      return 1;
    }
  }

  return 0;
}

typedef uint32_t iccSig;
static iccSig
str2iccSig (const void *s)
{
  const char  *p;

  p = (const char *) s;

  return (iccSig) ((p[0]<<24)|(p[1]<<16)|(p[2]<<8)|p[3]);
}

typedef struct
{
  int32_t X, Y, Z; /* s15Fixed16Number */
} iccXYZNumber;

typedef struct
{
  int           size;
  iccSig        CMMType;
  int32_t       version;
  iccSig        devClass;
  iccSig        colorSpace;
  iccSig        PCS;    /* Profile Connection Space */
  char          creationDate[12];
  iccSig        acsp;
  iccSig        platform;
  char          flags[4];
  iccSig        devMnfct;
  iccSig        devModel;
  char          devAttr[8];
  int32_t       intent;
  iccXYZNumber  illuminant;
  iccSig        creator;
  unsigned char ID[16]; /* MD5 checksum with Rendering intent,
                         * Header attrs, Profile ID fields are
                         * set to zeros.
                         */
  /* 28 bytes reserved - must be set to zeros */
} iccHeader;

#define iccNullSig 0
static void
iccp_init_iccHeader (iccHeader *icch)
{
  assert(icch);

  icch->size       = 0;
  icch->CMMType    = iccNullSig;
  icch->version    = 0xFFFFFF;
  icch->devClass   = iccNullSig;
  icch->colorSpace = iccNullSig;
  icch->PCS        = iccNullSig;
  memset(icch->creationDate, 0, 12);
  icch->acsp       = str2iccSig("ascp");
  icch->platform   = iccNullSig;
  memset(icch->flags, 0, 4);
  icch->devMnfct   = iccNullSig;
  icch->devModel   = iccNullSig;
  memset(icch->devAttr, 0, 8);
  icch->intent     = 0;
  icch->illuminant.X = 0;
  icch->illuminant.Y = 0;
  icch->illuminant.Z = 0;
  icch->creator      = iccNullSig;
  memset(icch->ID, 0, 16);
}

#define ICC_INTENT_TYPE(n) ((int) (((n) >> 16) & 0xff))
#define ICC_INTENT_PERCEPTUAL 0
#define ICC_INTENT_RELATIVE   1
#define ICC_INTENT_SATURATION 2
#define ICC_INTENT_ABSOLUTE   3

/*
 * In ICC profile stream dicrionary, there is /Range whose values must
 * "match the information in the profile". But where is those values in?
 *
 * How should I treat rendering intent?
 */
struct iccbased_cdata
{
  int32_t        sig; /* 'i' 'c' 'c' 'b' */

  unsigned char  checksum[16]; /* 16 bytes MD5 Checksum   */
  int            colorspace;   /* input colorspace:
                                *   RGB, Gray, CMYK, (Lab?)
                                */
  int            alternate;    /* alternate colorspace (id), unused */
};

#define check_sig(d,p,q,r,s) ((d) && (d)->sig == ((p)<<24|(q)<<16|(r)<<8|(s)))

static void
init_iccbased_cdata (struct iccbased_cdata *cdata)
{
  assert(cdata);

  cdata->sig = ('i' << 24|'c' << 16|'c' << 8|'b');
  memset(cdata->checksum, 0, 16);
  cdata->colorspace = PDF_COLORSPACE_TYPE_INVALID;
  cdata->alternate  = -1;

  return;
}

static void
release_iccbased_cdata (struct iccbased_cdata *cdata)
{
  assert(check_sig(cdata, 'i', 'c', 'c', 'b'));

  free(cdata);
}

static int
get_num_components_iccbased (const struct iccbased_cdata *cdata)
{
  int  num_components = 0;

  assert(check_sig(cdata, 'i', 'c', 'c', 'b'));

  switch (cdata->colorspace) {
  case PDF_COLORSPACE_TYPE_RGB:
    num_components = 3;
    break;
  case PDF_COLORSPACE_TYPE_CMYK:
    num_components = 4;
    break;
  case PDF_COLORSPACE_TYPE_GRAY:
    num_components = 1;
    break;
  case PDF_COLORSPACE_TYPE_LAB:
    num_components = 3;
    break;
  }

  return num_components;
}

static int
compare_iccbased (const char *ident1, const struct iccbased_cdata *cdata1,
                  const char *ident2, const struct iccbased_cdata *cdata2)
{
  if (cdata1 && cdata2) {

    assert(check_sig(cdata1, 'i', 'c', 'c', 'b'));
    assert(check_sig(cdata2, 'i', 'c', 'c', 'b'));

    if (memcmp(cdata1->checksum, nullbytes16, 16) &&
        memcmp(cdata2->checksum, nullbytes16, 16)) {
      return memcmp(cdata1->checksum, cdata2->checksum, 16);
    }
    if (cdata1->colorspace != cdata2->colorspace) {
      return (cdata1->colorspace - cdata2->colorspace);
    }

    /* Continue if checksum unknown and colorspace is same. */
  }

  if (ident1 && ident2)
    return strcmp(ident1, ident2);

  /* No way to compare */
  return -1;
}

int
iccp_check_colorspace (int colortype, const void *profile, int proflen)
{
  iccSig  colorspace;
  const unsigned char  *p;

  if (!profile || proflen < 128)
    return -1;

  p = (const unsigned char *) profile;

  colorspace = str2iccSig(p + 16);

  switch (colortype) {
  case PDF_COLORSPACE_TYPE_CALRGB:
  case PDF_COLORSPACE_TYPE_RGB:
    if (colorspace != str2iccSig("RGB ")) {
      return -1;
    }
    break;
  case PDF_COLORSPACE_TYPE_CALGRAY:
  case PDF_COLORSPACE_TYPE_GRAY:
    if (colorspace != str2iccSig("GRAY")) {
      return -1;
    }
    break;
  case PDF_COLORSPACE_TYPE_CMYK:
    if (colorspace != str2iccSig("CMYK")) {
      return -1;
    }
    break;
  default:
    return -1;
  }

  return 0;
}

pdf_obj *
iccp_get_rendering_intent (const void *profile, int proflen)
{
  pdf_obj       *ri = NULL;
  const unsigned char *p;
  int32_t        intent;

  if (!profile || proflen < 128)
    return NULL;

  p = (const unsigned char *) profile;

  intent = (p[64] << 24)|(p[65] << 16)|(p[66] << 8)|p[67];
  switch (ICC_INTENT_TYPE(intent)) {
  case ICC_INTENT_SATURATION:
    ri = pdf_new_name("Saturation");
    break;
  case ICC_INTENT_PERCEPTUAL:
    ri = pdf_new_name("Perceptual");
    break;
  case ICC_INTENT_ABSOLUTE:
    ri = pdf_new_name("AbsoluteColorimetric");
    break;
  case ICC_INTENT_RELATIVE:
    ri = pdf_new_name("RelativeColorimetric");
    break;
  default:
    dpx_warning("Invalid rendering intent type: %d", ICC_INTENT_TYPE(intent));
    ri = NULL;
  }

  return ri;
}

#define sget_signed_long(p)  ((int32_t)   ((p)[0] << 24|(p)[1] << 16|(p)[2] << 8|(p)[3]))
#define sget_signed_short(p) ((short)  ((p)[0] << 8|(p)[1]))
#define get_iccSig(p)        ((iccSig) ((p)[0] << 24|(p)[1] << 16|(p)[2] << 8|(p)[3]))

static int
iccp_unpack_header (iccHeader *icch,
                    const void *profile, int proflen, int check_size)
{
  const unsigned char *p, *endptr;

  if (check_size) {
    if (!profile || proflen < 128 ||
        proflen % 4 != 0) {
      dpx_warning("Profile size: %d", proflen);
      return -1;
    }
  }

  p      = (const unsigned char *) profile;
  endptr = p + 128;

  icch->size = sget_signed_long(p);
  if (check_size) {
    if (icch->size != proflen) {
      dpx_warning("ICC Profile size: %d(header) != %d", icch->size, proflen);
      return -1;
    }
  }
  p += 4;

  icch->CMMType    = str2iccSig(p);
  p += 4;
  icch->version    = sget_signed_long(p);
  p += 4;
  icch->devClass   = str2iccSig(p);
  p += 4;
  icch->colorSpace = str2iccSig(p);
  p += 4;
  icch->PCS        = str2iccSig(p);
  p += 4;
  memcpy(icch->creationDate, p, 12);
  p += 12;
  icch->acsp = str2iccSig(p); /* acsp */
  if (icch->acsp != str2iccSig("acsp")) {
    dpx_warning("Invalid ICC profile: not \"acsp\" - %c%c%c%c ",
         p[0], p[1], p[2], p[3]);
    return -1;
  }
  p += 4;
  icch->platform = str2iccSig(p);
  p += 4;
  memcpy(icch->flags, p, 4);
  p += 4;
  icch->devMnfct = str2iccSig(p);
  p += 4;
  icch->devModel = str2iccSig(p);
  p += 4;
  memcpy(icch->devAttr,  p, 8);
  p += 8;
  icch->intent = (p[0] << 24)|(p[1] << 16)|(p[2] << 8)|p[3];
  p += 4;
  icch->illuminant.X = sget_signed_long(p);
  p += 4;
  icch->illuminant.Y = sget_signed_long(p);
  p += 4;
  icch->illuminant.Z = sget_signed_long(p);
  p += 4;
  icch->creator = str2iccSig(p);
  p += 4;
  memcpy(icch->ID, p, 16);
  p += 16;

  /* 28 bytes reserved - must be set to zeros */
  for (; p < endptr; p++) {
    if (*p != '\0') {
      dpx_warning("Reserved pad not zero: %02x (at offset %d in ICC profile header.)",
           *p, 128 - ((int) (endptr - p)));
      return -1;
    }
  }

  return 0;
}

/* MD5 checksum with Rendering intent,
 * Header attrs, Profile ID fields are
 * set to zeros.
 */
#define ICC_HEAD_SECT1_START  0
#define ICC_HEAD_SECT1_LENGTH 56
/* 8 bytes devAttr, 4 bytes intent */
#define ICC_HEAD_SECT2_START  68
#define ICC_HEAD_SECT2_LENGTH 16
/* 16 bytes ID (checksum) */
#define ICC_HEAD_SECT3_START  100
#define ICC_HEAD_SECT3_LENGTH 28

#include "dpx-dpxcrypt.h"

static void
iccp_get_checksum (unsigned char *checksum, const void *profile, int proflen)
{
  const unsigned char *p;
  MD5_CONTEXT    md5;

  p = (const unsigned char *) profile;

  MD5_init (&md5);
  MD5_write(&md5, p + ICC_HEAD_SECT1_START, ICC_HEAD_SECT1_LENGTH);
  MD5_write(&md5, nullbytes16, 12);
  MD5_write(&md5, p + ICC_HEAD_SECT2_START, ICC_HEAD_SECT2_LENGTH);
  MD5_write(&md5, nullbytes16, 16);
  MD5_write(&md5, p + ICC_HEAD_SECT3_START, ICC_HEAD_SECT3_LENGTH);

  /* body */
  MD5_write(&md5, p + 128, proflen - 128);

  MD5_final(checksum, &md5);
}

static void
print_iccp_header (iccHeader *icch, unsigned char *checksum)
{
  int   i;

  assert(icch);

#define print_iccSig(s,t) if ((s) == 0) {\
    dpx_message("pdf_color>> %s:\t(null)\n", (t)); \
  } else if (!isprint(((s) >> 24) & 0xff) || \
             !isprint(((s) >> 16) & 0xff) || \
             !isprint(((s) >>  8) & 0xff) || \
             !isprint((s) & 0xff)) { \
    dpx_message("pdf_color>> %s:\t(invalid)\n", (t)); \
  } else { \
    dpx_message("pdf_color>> %s:\t%c%c%c%c\n",  (t), \
         ((s) >> 24) & 0xff, ((s) >> 16) & 0xff, \
         ((s) >>  8) & 0xff, (s) & 0xff); \
}

  dpx_message("\n");
  dpx_message("pdf_color>> ICC Profile Info\n");
  dpx_message("pdf_color>> Profile Size:\t%d bytes\n", icch->size);
  print_iccSig(icch->CMMType, "CMM Type");
  dpx_message("pdf_color>> Profile Version:\t%d.%01d.%01d\n",
       (icch->version >> 24) & 0xff,
       (icch->version >> 20) & 0x0f,
       (icch->version >> 16) & 0x0f);
  print_iccSig(icch->devClass,   "Device Class");
  print_iccSig(icch->colorSpace, "Color Space");
  print_iccSig(icch->PCS, "Connection Space");
  dpx_message("pdf_color>> Creation Date:\t");
  for (i = 0; i < 12; i += 2) {
    if (i == 0)
      dpx_message("%04u",
           sget_unsigned_pair((unsigned char *) icch->creationDate));
    else {
      dpx_message(":%02u",
           sget_unsigned_pair((unsigned char *) (&icch->creationDate[i])));
    }
  }
  dpx_message("\n");
  print_iccSig(icch->platform, "Primary Platform");
  dpx_message("pdf_color>> Profile Flags:\t%02x:%02x:%02x:%02x\n",
       icch->flags[0], icch->flags[1], icch->flags[2], icch->flags[3]);
  print_iccSig(icch->devMnfct, "Device Mnfct");
  print_iccSig(icch->devModel, "Device Model");
  dpx_message("pdf_color>> Device Attr:\t");
  for (i = 0; i < 8; i++) {
    if (i == 0)
      dpx_message("%02x",  icch->devAttr[i]);
    else
      dpx_message(":%02x", icch->devAttr[i]);
  }
  dpx_message("\n");
  dpx_message("pdf_color>> Rendering Intent:\t");
  switch (ICC_INTENT_TYPE(icch->intent)) {
  case ICC_INTENT_SATURATION:
    dpx_message("Saturation");
    break;
  case ICC_INTENT_PERCEPTUAL:
    dpx_message("Perceptual");
    break;
  case ICC_INTENT_ABSOLUTE:
    dpx_message("Absolute Colorimetric");
    break;
  case ICC_INTENT_RELATIVE:
    dpx_message("Relative Colorimetric");
    break;
  default:
    dpx_message("(invalid)");
    break;
  }
  dpx_message("\n");
  print_iccSig(icch->creator, "Creator");
  dpx_message("pdf_color>> Illuminant (XYZ):\t");
  dpx_message("%.3f %.3f %.3f\n",
       (double) icch->illuminant.X / 0x10000,
       (double) icch->illuminant.Y / 0x10000,
       (double) icch->illuminant.Z / 0x10000);
  dpx_message("pdf_color>> Checksum:\t");
  if (!memcmp(icch->ID, nullbytes16, 16)) {
    dpx_message("(null)");
  } else {
    for (i = 0; i < 16; i++) {
      if (i == 0)
        dpx_message("%02x",  icch->ID[i]);
      else
        dpx_message(":%02x", icch->ID[i]);
    }
  }
  dpx_message("\n");
  if (checksum) {
    dpx_message("pdf_color>> Calculated:\t");
    for (i = 0; i < 16; i++) {
      if (i == 0)
        dpx_message("%02x", checksum[i]);
      else
        dpx_message(":%02x", checksum[i]);
    }
    dpx_message("\n");
  }

  return;
}


static int
iccp_devClass_allowed (int dev_class)
{
  int    colormode;

  colormode = pdf_dev_get_param(PDF_DEV_PARAM_COLORMODE);

  switch (colormode) {
  default:
    if (dev_class != str2iccSig("scnr") &&
        dev_class != str2iccSig("mntr") &&
        dev_class != str2iccSig("prtr") &&
        dev_class != str2iccSig("spac")) {
      return 0;
    }
    break;
  }


  return 1;
}

int
iccp_load_profile (const char *ident,
                   const void *profile, int proflen)
{
  int       cspc_id;
  pdf_obj  *resource;
  pdf_obj  *stream;
  pdf_obj  *stream_dict;
  iccHeader icch;
  int       colorspace;
  unsigned char checksum[16];
  struct iccbased_cdata *cdata;

  iccp_init_iccHeader(&icch);
  if (iccp_unpack_header(&icch, profile, proflen, 1) < 0) { /* check size */
    dpx_warning("Invalid ICC profile header in \"%s\"", ident);
    print_iccp_header(&icch, NULL);
    return -1;
  }

  if (!iccp_version_supported((icch.version >> 24) & 0xff,
                              (icch.version >> 16) & 0xff)) {
    dpx_warning("ICC profile format spec. version %d.%01d.%01d"
         " not supported in current PDF version setting.",
         (icch.version >> 24) & 0xff,
         (icch.version >> 20) & 0x0f,
         (icch.version >> 16) & 0x0f);
    dpx_warning("ICC profile not embedded.");
    print_iccp_header(&icch, NULL);
    return -1;
  }

  if (!iccp_devClass_allowed(icch.devClass)) {
    dpx_warning("Unsupported ICC Profile Device Class:");
    print_iccp_header(&icch, NULL);
    return -1;
  }

  if (icch.colorSpace == str2iccSig("RGB ")) {
    colorspace = PDF_COLORSPACE_TYPE_RGB;
  } else if (icch.colorSpace == str2iccSig("GRAY")) {
    colorspace = PDF_COLORSPACE_TYPE_GRAY;
  } else if (icch.colorSpace == str2iccSig("CMYK")) {
    colorspace = PDF_COLORSPACE_TYPE_CMYK;
  } else {
    dpx_warning("Unsupported input color space.");
    print_iccp_header(&icch, NULL);
    return -1;
  }

  iccp_get_checksum(checksum, profile, proflen);
  if (memcmp(icch.ID,  nullbytes16, 16) &&
      memcmp(icch.ID,  checksum, 16)) {
    dpx_warning("Invalid ICC profile: Inconsistent checksum.");
    print_iccp_header(&icch, checksum);
    return -1;
  }

  cdata = NEW(1, struct iccbased_cdata);
  init_iccbased_cdata(cdata);
  cdata->colorspace = colorspace;
  memcpy(cdata->checksum, checksum, 16);

  cspc_id = pdf_colorspace_findresource(ident,
                                        PDF_COLORSPACE_TYPE_ICCBASED, cdata);
  if (cspc_id >= 0) {
    if (dpx_conf.verbose_level > 0)
      dpx_message("(ICCP:[id=%d])", cspc_id);
    release_iccbased_cdata(cdata);
    return cspc_id;
  }
  if (dpx_conf.verbose_level > 1) {
    print_iccp_header(&icch, checksum);
  }

  resource = pdf_new_array();

  stream = pdf_new_stream(STREAM_COMPRESS);
  pdf_add_array(resource, pdf_new_name("ICCBased"));
  pdf_add_array(resource, pdf_ref_obj (stream));

  stream_dict = pdf_stream_dict(stream);
  pdf_add_dict(stream_dict, pdf_new_name("N"),
               pdf_new_number(get_num_components_iccbased(cdata)));

  pdf_add_stream (stream, profile, proflen);
  pdf_release_obj(stream);

  cspc_id = pdf_colorspace_defineresource(ident,
                                          PDF_COLORSPACE_TYPE_ICCBASED,
                                          cdata, resource);

  return cspc_id;
}

typedef struct {
  char    *ident;
  int      subtype;

  pdf_obj *resource;
  pdf_obj *reference;

  void    *cdata;
} pdf_colorspace;

static struct {
  unsigned int count;
  unsigned int capacity;
  pdf_colorspace *colorspaces;
} cspc_cache = {
  0, 0, NULL
};

int
pdf_colorspace_findresource (const char *ident,
                             int type, const void *cdata)
{
  pdf_colorspace *colorspace;
  int  cspc_id, cmp = -1;

  for (cspc_id = 0;
       cmp && cspc_id < cspc_cache.count; cspc_id++) {
    colorspace = &cspc_cache.colorspaces[cspc_id];
    if (colorspace->subtype != type)
      continue;

    switch (colorspace->subtype) {
    case PDF_COLORSPACE_TYPE_ICCBASED:
      cmp = compare_iccbased(ident, cdata,
                             colorspace->ident, colorspace->cdata);
      break;
    }
    if (!cmp)
      return cspc_id;
  }

  return -1; /* not found */
}

static void
pdf_init_colorspace_struct (pdf_colorspace *colorspace)
{
  assert(colorspace);

  colorspace->ident     = NULL;
  colorspace->subtype   = PDF_COLORSPACE_TYPE_INVALID;

  colorspace->resource  = NULL;
  colorspace->reference = NULL;
  colorspace->cdata     = NULL;

  return;
}

static void
pdf_clean_colorspace_struct (pdf_colorspace *colorspace)
{
  assert(colorspace);

  free(colorspace->ident);
  pdf_release_obj(colorspace->resource);
  pdf_release_obj(colorspace->reference);
  colorspace->resource  = NULL;
  colorspace->reference = NULL;

  if (colorspace->cdata) {
    switch (colorspace->subtype) {
    case PDF_COLORSPACE_TYPE_ICCBASED:
      release_iccbased_cdata(colorspace->cdata);
      break;
    }
  }
  colorspace->cdata     = NULL;
  colorspace->subtype   = PDF_COLORSPACE_TYPE_INVALID;

  return;
}

static void
pdf_flush_colorspace (pdf_colorspace *colorspace)
{
  assert(colorspace);

  pdf_release_obj(colorspace->resource);
  pdf_release_obj(colorspace->reference);

  colorspace->resource  = NULL;
  colorspace->reference = NULL;
}

int
pdf_colorspace_defineresource (const char *ident,
                               int subtype, void *cdata, pdf_obj *resource)
{
  int  cspc_id;
  pdf_colorspace *colorspace;

  if (cspc_cache.count >= cspc_cache.capacity) {
    cspc_cache.capacity   += 16;
    cspc_cache.colorspaces = RENEW(cspc_cache.colorspaces,
                                   cspc_cache.capacity, pdf_colorspace);
  }
  cspc_id    = cspc_cache.count;
  colorspace = &cspc_cache.colorspaces[cspc_id];

  pdf_init_colorspace_struct(colorspace);
  if (ident) {
    colorspace->ident = NEW(strlen(ident) + 1, char);
    strcpy(colorspace->ident, ident);
  }
  colorspace->subtype  = subtype;
  colorspace->cdata    = cdata;
  colorspace->resource = resource;

  if (dpx_conf.verbose_level > 0) {
    dpx_message("(ColorSpace:%s", ident);
    if (dpx_conf.verbose_level > 1) {
      switch (subtype) {
      case PDF_COLORSPACE_TYPE_ICCBASED:
        dpx_message("[ICCBased]");
        break;
      case PDF_COLORSPACE_TYPE_CALRGB:
        dpx_message("[CalRGB]");
        break;
      case PDF_COLORSPACE_TYPE_CALGRAY:
        dpx_message("[CalGray]");
        break;
      }
    }
    dpx_message(")");
  }

  cspc_cache.count++;

  return cspc_id;
}

pdf_obj *
pdf_get_colorspace_reference (int cspc_id)
{
  pdf_colorspace *colorspace;

  colorspace = &cspc_cache.colorspaces[cspc_id];
  if (!colorspace->reference) {
    colorspace->reference = pdf_ref_obj(colorspace->resource);
    pdf_release_obj(colorspace->resource); /* .... */
    colorspace->resource = NULL;
  }

  return pdf_link_obj(colorspace->reference);
}

void
pdf_init_colors (void)
{
  cspc_cache.count    = 0;
  cspc_cache.capacity = 0;
  cspc_cache.colorspaces = NULL;
}

void
pdf_close_colors (void)
{
  unsigned int i;

  for (i = 0; i < cspc_cache.count; i++) {
    pdf_colorspace *colorspace;

    colorspace = &cspc_cache.colorspaces[i];
    pdf_flush_colorspace(colorspace);
    pdf_clean_colorspace_struct(colorspace);
  }
  cspc_cache.colorspaces = mfree(cspc_cache.colorspaces);
  cspc_cache.count = cspc_cache.capacity = 0;

}

#define PDF_COLORSPACE_FAMILY_DEVICE   0
#define PDF_COLORSPACE_FAMILY_CIEBASED 1
#define PDF_COLORSPACE_FAMILY_SPECIAL  2
