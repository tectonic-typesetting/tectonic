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

/* No page independence here...
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "system.h"
#include "mem.h"
#include "error.h"

#include "dpxfile.h"

#include "pdfdoc.h"
#include "pdfdev.h"

#include "pdfcolor.h"

static int verbose = 0;
void
pdf_color_set_verbose (void)
{
  verbose++;
}

/* This function returns PDF_COLORSPACE_TYPE_GRAY,
 * PDF_COLORSPACE_TYPE_RGB, PDF_COLORSPACE_TYPE_CMYK or
 * PDF_COLORSPACE_TYPE_SPOT.
 */
int
pdf_color_type (const pdf_color *color)
{
  ASSERT(color);

  return -color->num_components;
}

int
pdf_color_rgbcolor (pdf_color *color, double r, double g, double b)
{
  ASSERT(color);

  if (r < 0.0 || r > 1.0) {
    WARN("Invalid color value specified: red=%g",   r);
    return -1;
  }
  if (g < 0.0 || g > 1.0) {
    WARN("Invalid color value specified: green=%g", g);
    return -1;
  }
  if (b < 0.0 || b > 1.0) {
    WARN("Invalid color value specified: blue=%g", b);
    return -1;
  }
  color->values[0] = r;
  color->values[1] = g;
  color->values[2] = b;

  color->num_components = 3;

  color->spot_color_name = NULL;

  return 0;
}

int
pdf_color_cmykcolor (pdf_color *color,
		     double c, double m, double y, double k)
{
  ASSERT(color);

  if (c < 0.0 || c > 1.0) {
    WARN("Invalid color value specified: cyan=%g", c);
    return -1;
  }
  if (m < 0.0 || m > 1.0) {
    WARN("Invalid color value specified: magenta=%g", m);
    return -1;
  }
  if (y < 0.0 || y > 1.0) {
    WARN("Invalid color value specified: yellow=%g", y);
    return -1;
  }
  if (k < 0.0 || k > 1.0) {
    WARN("Invalid color value specified: black=%g", k);
    return -1;
  }

  color->values[0] = c;
  color->values[1] = m;
  color->values[2] = y;
  color->values[3] = k;

  color->num_components = 4;

  color->spot_color_name = NULL;

  return 0;
}

int
pdf_color_graycolor (pdf_color *color, double g)
{
  ASSERT(color);

  if (g < 0.0 || g > 1.0) {
    WARN("Invalid color value specified: gray=%g", g);
    return -1;
  }

  color->values[0] = g;

  color->num_components = 1;

  color->spot_color_name = NULL;

  return 0;
}

int
pdf_color_spotcolor (pdf_color *color, char* name, double c)
{
  ASSERT(color);

  if (c < 0.0 || c > 1.0) {
    WARN("Invalid color value specified: grade=%g", c);
    return -1;
  }

  color->values[0] = c;
  color->values[1] = 0.0; /* Dummy */

  color->num_components = 2;

  color->spot_color_name = name;

  return 0;
}


void
pdf_color_copycolor (pdf_color *color1, const pdf_color *color2)
{
  ASSERT(color1 && color2);

  memcpy(color1, color2, sizeof(pdf_color));
}

/* Brighten up a color. f == 0 means no change, f == 1 means white. */
void
pdf_color_brighten_color (pdf_color *dst, const pdf_color *src, double f)
{
  ASSERT(dst && src);

  if (f == 1.0) {
    pdf_color_white(dst);
  } else {
    double f0, f1;
    int n;

    n = dst->num_components = src->num_components;
    f1 = n == 4 ? 0.0 : f;  /* n == 4 is CMYK, others are RGB and Gray */
    f0 = 1.0-f;

    while (n--)
      dst->values[n] = f0 * src->values[n] + f1;
  }
}

int
pdf_color_is_white (const pdf_color *color)
{
  int n;
  double f;

  ASSERT(color);

  n = color->num_components;
  switch (n) {
  case 1:  /* Gray */
  case 3:  /* RGB */
    f = 1.0;
    break;
  case 4:  /* CMYK */
    f = 0.0;
    break;
  default:
    return 0;
  }

  while (n--)
    if (color->values[n] != f)
      return 0;

  return 1;
}

int
pdf_color_to_string (const pdf_color *color, char *buffer, char mask)
{
  int i, len = 0;

  if (pdf_color_type(color) == PDF_COLORSPACE_TYPE_SPOT) {
    len = sprintf(buffer, " /%s %c%c %g %c%c",
                          color->spot_color_name,
                          'C' | mask, 'S' | mask,
                          ROUND(color->values[0], 0.001),
                          'S' | mask, 'C' | mask);
  } else {
     for (i = 0; i < color->num_components; i++) {
       len += sprintf(buffer+len, " %g", ROUND(color->values[i], 0.001));
      }
  }

  return len;
}

pdf_color current_fill   = {
  1,
  NULL,
  {0.0, 0.0, 0.0, 0.0}
};

pdf_color current_stroke = {
  1,
  NULL,
  {0.0, 0.0, 0.0, 0.0}
};

/*
 * This routine is not a real color matching.
 */
int
pdf_color_compare (const pdf_color *color1, const pdf_color *color2)
{
  int n;

  n = color1->num_components;
  switch (n) {
  case 1:  /* Gray */
  case 2:  /* Spot */
  case 3:  /* RGB */
  case 4:  /* CMYK */
    break;
  default:
    return -1;
  }

  if (n != color2->num_components)
    return -1;

  while (n--)
    if (color1->values[n] != color2->values[n])
      return -1;

  if (color1->spot_color_name && color2->spot_color_name)
    return strcmp(color1->spot_color_name, color2->spot_color_name);

  return 0;
}

int
pdf_color_is_valid (const pdf_color *color)
{
  int  n;

  n = color->num_components;
  switch (n) {
  case 1:  /* Gray */
  case 2:  /* Spot */
  case 3:  /* RGB */
  case 4:  /* CMYK */
    break;
  default:
    return 0;
  }

  while (n--)
    if (color->values[n] < 0.0 || color->values[n] > 1.0) {
      WARN("Invalid color value: %g", color->values[n]);
      return 0;
    }

  if (pdf_color_type(color) == PDF_COLORSPACE_TYPE_SPOT) {
    if (!color->spot_color_name || color->spot_color_name[0] == '\0') {
      WARN("Invalid spot color: empty name");
      return 0;
    }
  }

  return 1;
}

/* Dvipdfm special */
pdf_color default_color = {
  1,
  NULL,
  {0.0, 0.0, 0.0, 0.0}
};

#define DEV_COLOR_STACK_MAX 128

static struct {
  int       current;
  pdf_color stroke[DEV_COLOR_STACK_MAX];
  pdf_color fill[DEV_COLOR_STACK_MAX];
} color_stack = {
  0,
};

void
pdf_color_clear_stack (void)
{
  if (color_stack.current > 0) {
    WARN("You've mistakenly made a global color change within nested colors.");
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
  pdf_dev_reset_color(0);
}

void
pdf_color_push (pdf_color *sc, pdf_color *fc)
{
  if (color_stack.current >= DEV_COLOR_STACK_MAX-1) {
    WARN("Color stack overflow. Just ignore.");
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
    WARN("Color stack underflow. Just ignore.");
  } else {
    color_stack.current--;
    pdf_dev_reset_color(0);
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

#if 0
/* BUG (20060330): color change does not effect on the next page.
 *   The problem is due to the part of grestore because it restores
 *   the color values in the state of gsave which are not correct
 *   if the color values are changed inside of a page.
 */
void
pdf_dev_preserve_color (void)
{
  if (color_stack.current > 0) {
    current_stroke = color_stack.stroke[color_stack.current];
    current_fill   = color_stack.fill[color_stack.current];
  }
}
#endif

/***************************** COLOR SPACE *****************************/

static int pdf_colorspace_defineresource (const char *ident,
					  int   subtype,
					  void *cdata, pdf_obj *resource);

static int pdf_colorspace_findresource   (const char *ident,
					  int   subtype, const void *cdata);

#if 0
struct calgray_cdata
{
  double white_point[3]; /* required, second component must
			  * be equal to 1.0
			  */
  double black_point[3]; /* optional, default: [0 0 0] */
  double gamma;          /* optional, default: 1.0     */
};

struct calrgb_cdata
{
  double white_point[3]; /* required, second component must
			  * be equal to 1.0
			  */
  double black_point[3]; /* optional, default: [0 0 0] */
  double gamma[3];       /* optional, default: [1 1 1] */
  double matrix[9];      /* optional, default: identity
			  * [1 0 0 0 1 0 0 0 1]
			  */
};

static void
release_calrgb (void *cdata)
{
  struct calrgb_cdata *calrgb;

  if (cdata) {
    calrgb = (struct calrgb_cdata *) cdata;
    RELEASE(calrgb);
  }
}

static int
compare_calrgb (const char *ident1, const void *cdata1,
		const char *ident2, const void *cdata2)
{
  struct calrgb_cdata *calrgb1;
  struct calrgb_cdata *calrgb2;

  if (ident1 && ident2 &&
      !strcmp(ident1, ident2)) {
    return 0;
  }
}

static void
init_calrgb (struct calrgb_cdata *calrgb)
{
  ASSERT(calrgb);

  calrgb->white_point[0] = 1.0;
  calrgb->white_point[1] = 1.0;
  calrgb->white_point[2] = 1.0;

  calrgb->black_point[0] = 0.0;
  calrgb->black_point[1] = 0.0;
  calrgb->black_point[2] = 0.0;

  calrgb->gamma[0]  = 1.0;
  calrgb->gamma[1]  = 1.0;
  calrgb->gamma[2]  = 1.0;

  calrgb->matrix[0] = 1.0;
  calrgb->matrix[1] = 0.0;
  calrgb->matrix[2] = 0.0;

  calrgb->matrix[3] = 0.0;
  calrgb->matrix[4] = 1.0;
  calrgb->matrix[5] = 0.0;

  calrgb->matrix[6] = 0.0;
  calrgb->matrix[7] = 0.0;
  calrgb->matrix[8] = 1.0;
}

static int
valid_calrgb (struct calrgb_cdata *calrgb)
{
  if (calrgb->white_point[1] != 1.0 ||
      calrgb->white_point[0] <= 0.0 ||
      calrgb->white_point[2] <= 0.0)
    return 0;

  if (calrgb->black_point[0] < 0.0 ||
      calrgb->black_point[1] < 0.0 ||
      calrgb->black_point[2] < 0.0)
    return 0;

  if (calrgb->gamma[0] < 0.0 ||
      calrgb->gamma[1] < 0.0 ||
      calrgb->gamma[2] < 0.0)
    return 0;

  /* matrix should be invertible? */

  return 1;
}

static pdf_obj *
pdf_color_make_calrgb_resource (struct calrgb_cdata *calrgb)
{
  pdf_obj *colorspace;
  pdf_obj *calparams, *tmp_array;

  ASSERT(calrgb);

  if (!valid_calrgb(calrgb))
    return NULL;

  colorspace = pdf_new_array();
  calparams  = pdf_new_dict();

  tmp_array  = pdf_new_array();
  pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->white_point[0], 0.001)));
  pdf_add_array(tmp_array, pdf_new_number(1.0));
  pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->white_point[2], 0.001)));
  pdf_add_dict(calparams, pdf_new_name("WhitePoint"), tmp_array);

  if (calrgb->black_point[0] != 0.0 ||
      calrgb->black_point[1] != 0.0 ||
      calrgb->black_point[2] != 0.0) {
    tmp_array  = pdf_new_array();
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->black_point[0], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->black_point[1], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->black_point[2], 0.001)));
    pdf_add_dict(calparams, pdf_new_name("BlackPoint"), tmp_array);
  }

  if (calrgb->gamma[0] != 1.0 ||
      calrgb->gamma[1] != 1.0 ||
      calrgb->gamma[2] != 1.0) {
    tmp_array  = pdf_new_array();
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->gamma[0], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->gamma[1], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->gamma[2], 0.001)));
    pdf_add_dict(calparams, pdf_new_name("Gamma"), tmp_array);
  }

  if (calrgb->matrix[0] != 1.0 ||
      calrgb->matrix[1] != 0.0 ||
      calrgb->matrix[2] != 0.0 ||
      calrgb->matrix[3] != 0.0 ||
      calrgb->matrix[4] != 1.0 ||
      calrgb->matrix[5] != 0.0 ||
      calrgb->matrix[6] != 0.0 ||
      calrgb->matrix[7] != 0.0 ||
      calrgb->matrix[8] != 1.0) {
    tmp_array  = pdf_new_array();
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[0], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[1], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[2], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[3], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[4], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[5], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[6], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[7], 0.001)));
    pdf_add_array(tmp_array, pdf_new_number(ROUND(calrgb->matrix[8], 0.001)));
    pdf_add_dict(calparams,  pdf_new_name("Matrix"), tmp_array);
  }

  pdf_add_array(colorspace, pdf_new_name("CalRGB"));
  pdf_add_array(colorspace, calparams);

  return colorspace;
}
#endif

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
};

static int
iccp_version_supported (int major, int minor)
{
  int  pdf_ver;

  pdf_ver = pdf_get_version();
  if (pdf_ver < 8) {
    if (icc_versions[pdf_ver].major < major)
      return 0;
    else if (icc_versions[pdf_ver].major == major &&
             icc_versions[pdf_ver].minor <  minor)
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
  ASSERT(icch);

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
  ASSERT(cdata);

  cdata->sig = ('i' << 24|'c' << 16|'c' << 8|'b');
  memset(cdata->checksum, 0, 16);
  cdata->colorspace = PDF_COLORSPACE_TYPE_INVALID;
  cdata->alternate  = -1;

  return;
}

static void
release_iccbased_cdata (struct iccbased_cdata *cdata)
{
  ASSERT(check_sig(cdata, 'i', 'c', 'c', 'b'));

  RELEASE(cdata);
}

static int
get_num_components_iccbased (const struct iccbased_cdata *cdata)
{
  int  num_components = 0;

  ASSERT(check_sig(cdata, 'i', 'c', 'c', 'b'));

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
  case PDF_COLORSPACE_TYPE_CIELAB:
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

    ASSERT(check_sig(cdata1, 'i', 'c', 'c', 'b'));
    ASSERT(check_sig(cdata2, 'i', 'c', 'c', 'b'));

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
    WARN("Invalid rendering intent type: %d", ICC_INTENT_TYPE(intent));
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
      WARN("Profile size: %ld", proflen);
      return -1;
    }
  }

  p      = (const unsigned char *) profile;
  endptr = p + 128;

  icch->size = sget_signed_long(p);
  if (check_size) {
    if (icch->size != proflen) {
      WARN("ICC Profile size: %ld(header) != %ld", icch->size, proflen);
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
    WARN("Invalid ICC profile: not \"acsp\" - %c%c%c%c ",
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
      WARN("Reserved pad not zero: %02x (at offset %d in ICC profile header.)",
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

#include "dpxcrypt.h"
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

  ASSERT(icch);

#define print_iccSig(s,t) if ((s) == 0) {\
    MESG("pdf_color>> %s:\t(null)\n", (t)); \
  } else if (!isprint(((s) >> 24) & 0xff) || \
             !isprint(((s) >> 16) & 0xff) || \
             !isprint(((s) >>  8) & 0xff) || \
             !isprint((s) & 0xff)) { \
    MESG("pdf_color>> %s:\t(invalid)\n", (t)); \
  } else { \
    MESG("pdf_color>> %s:\t%c%c%c%c\n",  (t), \
         ((s) >> 24) & 0xff, ((s) >> 16) & 0xff, \
         ((s) >>  8) & 0xff, (s) & 0xff); \
}

  MESG("\n");
  MESG("pdf_color>> ICC Profile Info\n");
  MESG("pdf_color>> Profile Size:\t%ld bytes\n", icch->size);
  print_iccSig(icch->CMMType, "CMM Type");
  MESG("pdf_color>> Profile Version:\t%d.%01d.%01d\n",
       (icch->version >> 24) & 0xff,
       (icch->version >> 20) & 0x0f,
       (icch->version >> 16) & 0x0f);
  print_iccSig(icch->devClass,   "Device Class");
  print_iccSig(icch->colorSpace, "Color Space");
  print_iccSig(icch->PCS, "Connection Space");
  MESG("pdf_color>> Creation Date:\t");
  for (i = 0; i < 12; i += 2) {
    if (i == 0)
      MESG("%04u",
	   sget_unsigned_pair((unsigned char *) icch->creationDate));
    else {
      MESG(":%02u",
	   sget_unsigned_pair((unsigned char *) (&icch->creationDate[i])));
    }
  }
  MESG("\n");
  print_iccSig(icch->platform, "Primary Platform");
  MESG("pdf_color>> Profile Flags:\t%02x:%02x:%02x:%02x\n",
       icch->flags[0], icch->flags[1], icch->flags[2], icch->flags[3]);
  print_iccSig(icch->devMnfct, "Device Mnfct");
  print_iccSig(icch->devModel, "Device Model");
  MESG("pdf_color>> Device Attr:\t");
  for (i = 0; i < 8; i++) {
    if (i == 0)
      MESG("%02x",  icch->devAttr[i]);
    else
      MESG(":%02x", icch->devAttr[i]);
  }
  MESG("\n");
  MESG("pdf_color>> Rendering Intent:\t");
  switch (ICC_INTENT_TYPE(icch->intent)) {
  case ICC_INTENT_SATURATION:
    MESG("Saturation");
    break;
  case ICC_INTENT_PERCEPTUAL:
    MESG("Perceptual");
    break;
  case ICC_INTENT_ABSOLUTE:
    MESG("Absolute Colorimetric");
    break;
  case ICC_INTENT_RELATIVE:
    MESG("Relative Colorimetric");
    break;
  default:
    MESG("(invalid)");
    break;
  }
  MESG("\n");
  print_iccSig(icch->creator, "Creator");
  MESG("pdf_color>> Illuminant (XYZ):\t");
  MESG("%.3f %.3f %.3f\n",
       (double) icch->illuminant.X / 0x10000,
       (double) icch->illuminant.Y / 0x10000,
       (double) icch->illuminant.Z / 0x10000);
  MESG("pdf_color>> Checksum:\t");
  if (!memcmp(icch->ID, nullbytes16, 16)) {
    MESG("(null)");
  } else {
    for (i = 0; i < 16; i++) {
      if (i == 0)
	MESG("%02x",  icch->ID[i]);
      else
	MESG(":%02x", icch->ID[i]);
    }
  }
  MESG("\n");
  if (checksum) {
    MESG("pdf_color>> Calculated:\t");
    for (i = 0; i < 16; i++) {
      if (i == 0)
	MESG("%02x", checksum[i]);
      else
	MESG(":%02x", checksum[i]);
    }
    MESG("\n");
  }

  return;
}


static int
iccp_devClass_allowed (int dev_class)
{
  int    colormode;

  colormode = pdf_dev_get_param(PDF_DEV_PARAM_COLORMODE);

  switch (colormode) {
#if 0
  case PDF_DEV_COLORMODE_PDFX1:
    break;
  case PDF_DEV_COLORMODE_PDFX3:
    if (dev_class != str2iccSig("prtr")) {
      return 0;
    }
    break;
#endif
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
    WARN("Invalid ICC profile header in \"%s\"", ident);
    print_iccp_header(&icch, NULL);
    return -1;
  }

  if (!iccp_version_supported((icch.version >> 24) & 0xff,
			      (icch.version >> 16) & 0xff)) {
    WARN("ICC profile format spec. version %d.%01d.%01d"
	 " not supported in current PDF version setting.",
	 (icch.version >> 24) & 0xff,
	 (icch.version >> 20) & 0x0f,
	 (icch.version >> 16) & 0x0f);
    WARN("ICC profile not embedded.");
    print_iccp_header(&icch, NULL);
    return -1;
  }

  if (!iccp_devClass_allowed(icch.devClass)) {
    WARN("Unsupported ICC Profile Device Class:");
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
    WARN("Unsupported input color space.");
    print_iccp_header(&icch, NULL);
    return -1;
  }

  iccp_get_checksum(checksum, profile, proflen);
  if (memcmp(icch.ID,  nullbytes16, 16) &&
      memcmp(icch.ID,  checksum, 16)) {
    WARN("Invalid ICC profile: Inconsistent checksum.");
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
    if (verbose)
      MESG("(ICCP:[id=%d])", cspc_id);
    release_iccbased_cdata(cdata);
    return cspc_id;
  }
  if (verbose > 1) {
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

#if 0
#define WBUF_SIZE 4096
static unsigned char wbuf[WBUF_SIZE];

static pdf_obj *
iccp_load_file_stream (unsigned char *checksum, int length, FILE *fp)
{
  pdf_obj       *stream;
  MD5_CONTEXT    md5;
  int            nb_read;

  rewind(fp);

  if (fread(wbuf, 1, 128, fp) != 128) {
    return NULL;
  }
  length -= 128;

  stream = pdf_new_stream(STREAM_COMPRESS);

  MD5_init (&md5);
  MD5_write(&md5, wbuf + ICC_HEAD_SECT1_START, ICC_HEAD_SECT1_LENGTH);
  MD5_write(&md5, nullbytes16, 12);
  MD5_write(&md5, wbuf + ICC_HEAD_SECT2_START, ICC_HEAD_SECT2_LENGTH);
  MD5_write(&md5, nullbytes16, 16);
  MD5_write(&md5, wbuf + ICC_HEAD_SECT3_START, ICC_HEAD_SECT3_LENGTH);

  pdf_add_stream(stream, wbuf, 128);

  /* body */
  while (length > 0) {
    nb_read = fread(wbuf, 1, MIN(length, WBUF_SIZE), fp);
    MD5_write(&md5, wbuf, nb_read);
    pdf_add_stream(stream, wbuf, nb_read);
    length -= nb_read;
  }

  MD5_final(checksum, &md5);


  return stream;
}

int
pdf_colorspace_load_ICCBased (const char *ident, const char *filename)
{
  int       cspc_id;
  FILE     *fp;
  pdf_obj  *resource;
  pdf_obj  *stream;
  pdf_obj  *stream_dict;
  iccHeader icch;
  int       colorspace;
  int       size;
  unsigned char checksum[16];
  struct iccbased_cdata *cdata;


  fp = DPXFOPEN(filename, DPX_RES_TYPE_ICCPROFILE);
  if (!fp)
    return -1;

  size = file_size(fp);
  if (size < 128) {
    MFCLOSE(fp);
    return -1;
  }
  if (fread(wbuf, 1, 128, fp) != 128) {
    DPXFCLOSE(fp);
    return -1;
  }

  iccp_init_iccHeader(&icch);
  if (iccp_unpack_header(&icch, wbuf, 128, 0) < 0) {
    WARN("Invalid ICC profile header in \"%s\"", ident);
    print_iccp_header(&icch, NULL);
    DPXFCLOSE(fp);
    return -1;
  }
  if (icch.size > size) {
    WARN("File size smaller than recorded in header: %ld %ld",
	 icch.size, size);
    DPXFCLOSE(fp);
    return -1;
  }

  if (!iccp_version_supported((icch.version >> 24) & 0xff,
			      (icch.version >> 16) & 0xff)) {
    WARN("ICC profile format spec. version %d.%01d.%01d"
	 " not supported in current PDF version setting.",
	 (icch.version >> 24) & 0xff,
	 (icch.version >> 20) & 0x0f,
	 (icch.version >> 16) & 0x0f);
    WARN("ICC profile not embedded.");
    print_iccp_header(&icch, NULL);
    DPXFCLOSE(fp);
    return -1;
  }

  if (!iccp_devClass_allowed(icch.devClass)) {
    WARN("Unsupported ICC Profile Device Class:");
    print_iccp_header(&icch, NULL);
    DPXFCLOSE(fp);
    return -1;
  }

  if (icch.colorSpace == str2iccSig("RGB ")) {
    colorspace = PDF_COLORSPACE_TYPE_RGB;
  } else if (icch.colorSpace == str2iccSig("GRAY")) {
    colorspace = PDF_COLORSPACE_TYPE_GRAY;
  } else if (icch.colorSpace == str2iccSig("CMYK")) {
    colorspace = PDF_COLORSPACE_TYPE_CMYK;
  } else {
    WARN("Unsupported input color space.");
    print_iccp_header(&icch, NULL);
    DPXFCLOSE(fp);
    return -1;
  }

  stream = iccp_load_file_stream(checksum, icch.size, fp);
  DPXFCLOSE(fp);

  if (!stream) {
    WARN("Loading ICC Profile failed...: %s", filename);
    return -1;
  }

  if (memcmp(icch.ID,  nullbytes16, 16) &&
      memcmp(icch.ID,  checksum, 16)) {
    WARN("Invalid ICC profile: Inconsistent checksum.");
    print_iccp_header(&icch, NULL);
    pdf_release_obj(stream);
    return -1;
  }

  cdata = NEW(1, struct iccbased_cdata);
  init_iccbased_cdata(cdata);
  cdata->colorspace = colorspace;
  memcpy(cdata->checksum, checksum, 16);

  cspc_id = pdf_colorspace_findresource(ident,
					PDF_COLORSPACE_TYPE_ICCBASED, cdata);
  if (cspc_id >= 0) {
    if (verbose)
      MESG("(ICCP:[id=%d])", cspc_id);
    release_iccbased_cdata(cdata);
    pdf_release_obj(stream);
    return cspc_id;
  }
  if (verbose > 1) {
    print_iccp_header(&icch, checksum);
  }

  resource = pdf_new_array();

  pdf_add_array(resource, pdf_new_name("ICCBased"));
  pdf_add_array(resource, pdf_ref_obj (stream));

  stream_dict = pdf_stream_dict(stream);
  pdf_add_dict(stream_dict, pdf_new_name("N"),
	       pdf_new_number(get_num_components_iccbased(cdata)));
  pdf_release_obj(stream);

  cspc_id = pdf_colorspace_defineresource(ident,
					  PDF_COLORSPACE_TYPE_ICCBASED,
					  cdata, resource);

  return cspc_id;
}
#endif

typedef struct {
  char    *ident;
  int      subtype;

  pdf_obj *resource;
  pdf_obj *reference;

  void    *cdata;
} pdf_colorspace;

static struct {
  int  count;
  int  capacity;
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
  ASSERT(colorspace);

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
  ASSERT(colorspace);

  if (colorspace->ident)
    RELEASE(colorspace->ident);
  if (colorspace->resource)
    pdf_release_obj(colorspace->resource);
  if (colorspace->reference)
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
  ASSERT(colorspace);

  if (colorspace->resource)
    pdf_release_obj(colorspace->resource);
  if (colorspace->reference)
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

  if (verbose) {
    MESG("(ColorSpace:%s", ident);
    if (verbose > 1) {
      switch (subtype) {
      case PDF_COLORSPACE_TYPE_ICCBASED:
	MESG("[ICCBased]");
	break;
      case PDF_COLORSPACE_TYPE_CALRGB:
	MESG("[CalRGB]");
	break;
      case PDF_COLORSPACE_TYPE_CALGRAY:
	MESG("[CalGray]");
	break;
      }
    }
    MESG(")");
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

#if 0
int
pdf_get_colorspace_num_components (int cspc_id)
{
  pdf_colorspace *colorspace;
  int  num_components;

  colorspace = &cspc_cache.colorspaces[cspc_id];

  switch (colorspace->subtype) {
  case PDF_COLORSPACE_TYPE_ICCBASED:
    num_components = get_num_components_iccbased(colorspace->cdata);
    break;
  case PDF_COLORSPACE_TYPE_DEVICEGRAY:
    num_components = 1;
    break;
  case PDF_COLORSPACE_TYPE_DEVICERGB:
    num_components = 3;
    break;
  case PDF_COLORSPACE_TYPE_DEVICECMYK:
    num_components = 4;
    break;
  case PDF_COLORSPACE_TYPE_CALRGB:
    num_components = 3;
    break;
  case PDF_COLORSPACE_TYPE_CALGRAY:
    num_components = 1;
    break;
  default:
    num_components = 0;
    break;
  }

  return num_components;
}

int
pdf_get_colorspace_subtype (int cspc_id)
{
  pdf_colorspace *colorspace;

  colorspace = &cspc_cache.colorspaces[cspc_id];

  return colorspace->subtype;
}
#endif

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
  int  i;

  for (i = 0; i < cspc_cache.count; i++) {
    pdf_colorspace *colorspace;

    colorspace = &cspc_cache.colorspaces[i];
    pdf_flush_colorspace(colorspace);
    pdf_clean_colorspace_struct(colorspace);
  }
  RELEASE(cspc_cache.colorspaces);
  cspc_cache.colorspaces = NULL;
  cspc_cache.count = cspc_cache.capacity = 0;

}

#define PDF_COLORSPACE_FAMILY_DEVICE   0
#define PDF_COLORSPACE_FAMILY_CIEBASED 1
#define PDF_COLORSPACE_FAMILY_SPECIAL  2
