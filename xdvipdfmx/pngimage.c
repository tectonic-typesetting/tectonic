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


#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

/*
 * PNG SUPPORT
 *
 *  All bitdepth supported.
 *  Supported color types are: PALETTE, RGB, GRAY, RGB_ALPHA, GRAY_ALPHA.
 *  Supported ancillary chunks: tRNS, cHRM, gAMA, (sRGB), (iCCP)
 *
 *  cHRM support is not tested well. CalRGB/CalGray colorspace is used for
 *  PNG images that have cHRM chunk.
 *
 */

#include "system.h"
#include "error.h"
#include "mem.h"

#include "dvipdfmx.h"

#include "pdfcolor.h"
#include "pdfobj.h"

#define PNG_DEBUG_STR "PNG"
#define PNG_DEBUG     3

#ifdef HAVE_LIBPNG

/*
 * Write, MNG, Progressive not required.
 */
#define PNG_NO_WRITE_SUPPORTED
#define PNG_NO_MNG_FEATURES
#define PNG_NO_PROGRESSIVE_READ

#include <png.h>
#include "pngimage.h"

#include "pdfximage.h"

#define DPX_PNG_DEFAULT_GAMMA 2.2

#define PDF_TRANS_TYPE_NONE   0
#define PDF_TRANS_TYPE_BINARY 1
#define PDF_TRANS_TYPE_ALPHA  2

/* ColorSpace */
static pdf_obj *create_cspace_Indexed  (png_structp png_ptr, png_infop info_ptr);

/* CIE-Based: CalRGB/CalGray */
static pdf_obj *create_cspace_CalRGB   (png_structp png_ptr, png_infop info_ptr);
static pdf_obj *create_cspace_CalGray  (png_structp png_ptr, png_infop info_ptr);
static pdf_obj *make_param_Cal         (png_byte color_type,
                                        double G,
                                        double xw, double yw,
                                        double xr, double yr,
                                        double xg, double yg,
                                        double xb, double yb);

/* sRGB:
 *
 * We (and PDF) do not have direct sRGB support. The sRGB color space can be
 * precisely represented by ICC profile, but we use approximate CalRGB color
 * space.
 */
static pdf_obj *create_cspace_sRGB    (png_structp png_ptr, png_infop info_ptr);
static pdf_obj *get_rendering_intent  (png_structp png_ptr, png_infop info_ptr);

/* ICCBased:
 *
 * Not supported yet.
 * Must check if ICC profile is valid and can be imported to PDF.
 * There are few restrictions (should be applied to PNG too?) in ICC profile
 * support in PDF. Some information should be obtained from profile.
 */
static pdf_obj *create_cspace_ICCBased (png_structp png_ptr, png_infop info_ptr);

/* Transparency */
static int      check_transparency (png_structp png_ptr, png_infop info_ptr);
/* Color-Key Mask */
static pdf_obj *create_ckey_mask   (png_structp png_ptr, png_infop info_ptr);
/* Soft Mask:
 *
 * create_soft_mask() is for PNG_COLOR_TYPE_PALLETE.
 * Images with alpha chunnel use strip_soft_mask().
 * An object representing mask itself is returned.
 */
static pdf_obj *create_soft_mask   (png_structp png_ptr, png_infop info_ptr,
                                    png_bytep image_data_ptr,
                                    png_uint_32 width, png_uint_32 height);
static pdf_obj *strip_soft_mask    (png_structp png_ptr, png_infop info_ptr,
                                    png_bytep image_data_ptr,
                                    png_uint_32p rowbytes_ptr,
                                    png_uint_32 width, png_uint_32 height);

/* Read image body */
static void read_image_data (png_structp png_ptr,
                             png_bytep dest_ptr,
                             png_uint_32 height, png_uint_32 rowbytes);

int
check_for_png (FILE *png_file)
{
  unsigned char sigbytes[4];

  rewind (png_file);
  if (fread (sigbytes, 1, sizeof(sigbytes), png_file) !=
      sizeof(sigbytes) ||
      (png_sig_cmp (sigbytes, 0, sizeof(sigbytes))))
    return 0;
  else
    return 1;
}

static void warn(png_structp png_ptr, png_const_charp msg)
{
  (void)png_ptr; (void)msg; /* Make compiler happy */
}

int
png_include_image (pdf_ximage *ximage, FILE *png_file)
{
  pdf_obj  *stream;
  pdf_obj  *stream_dict;
  pdf_obj  *colorspace, *mask, *intent;
  png_bytep stream_data_ptr;
  int       trans_type;
  ximage_info info;
  /* Libpng stuff */
  png_structp png_ptr;
  png_infop   png_info_ptr;
  png_byte    bpc, color_type;
  png_uint_32 width, height, rowbytes;

  pdf_ximage_init_image_info(&info);

  stream      = NULL;
  stream_dict = NULL;
  colorspace  = mask = intent = NULL;

  rewind (png_file);
  png_ptr = png_create_read_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, warn);
  if (png_ptr == NULL ||
      (png_info_ptr = png_create_info_struct (png_ptr)) == NULL) {
    WARN("%s: Creating Libpng read/info struct failed.", PNG_DEBUG_STR);
    if (png_ptr)
      png_destroy_read_struct(&png_ptr, NULL, NULL);
    return -1;
  }

#if PNG_LIBPNG_VER >= 10603
  /* ignore possibly incorrect CMF bytes */
  png_set_option(png_ptr, PNG_MAXIMUM_INFLATE_WINDOW, PNG_OPTION_ON);
#endif

  /* Inititializing file IO. */
  png_init_io (png_ptr, png_file);

  /* Read PNG info-header and get some info. */
  png_read_info(png_ptr, png_info_ptr);
  color_type = png_get_color_type  (png_ptr, png_info_ptr);
  width      = png_get_image_width (png_ptr, png_info_ptr);
  height     = png_get_image_height(png_ptr, png_info_ptr);
  bpc        = png_get_bit_depth   (png_ptr, png_info_ptr);

  /* Ask libpng to convert down to 8-bpc. */
  if (bpc > 8) {
    if (pdf_get_version() < 5) {
      WARN("%s: 16-bpc PNG requires PDF version 1.5.", PNG_DEBUG_STR);
    png_set_strip_16(png_ptr);
    bpc = 8;
  }
  }
  /* Ask libpng to gamma-correct.
   * It is wrong to assume screen gamma value 2.2 but...
   * We do gamma correction here only when uncalibrated color space is used. 
   */
  if (!png_get_valid(png_ptr, png_info_ptr, PNG_INFO_iCCP) &&
      !png_get_valid(png_ptr, png_info_ptr, PNG_INFO_sRGB) &&
      !png_get_valid(png_ptr, png_info_ptr, PNG_INFO_cHRM) &&
       png_get_valid(png_ptr, png_info_ptr, PNG_INFO_gAMA)) {
    double G = 1.0;
    png_get_gAMA (png_ptr, png_info_ptr, &G);
    png_set_gamma(png_ptr, 2.2, G);
  }

  trans_type = check_transparency(png_ptr, png_info_ptr);
  /* check_transparency() does not do updata_info() */
  png_read_update_info(png_ptr, png_info_ptr);
  rowbytes = png_get_rowbytes(png_ptr, png_info_ptr);

  /* Values listed below will not be modified in the remaining process. */
  info.width  = width;
  info.height = height;
  info.bits_per_component = bpc;

  if (compat_mode)
    info.xdensity = info.ydensity = 72.0 / 100.0;
  else
  {
    png_uint_32 xppm = png_get_x_pixels_per_meter(png_ptr, png_info_ptr);
    png_uint_32 yppm = png_get_y_pixels_per_meter(png_ptr, png_info_ptr);

    if (xppm > 0)
      info.xdensity = 72.0 / 0.0254 / xppm;
    if (yppm > 0)
      info.ydensity = 72.0 / 0.0254 / yppm;
  }

  stream      = pdf_new_stream (STREAM_COMPRESS);
  stream_dict = pdf_stream_dict(stream);

  stream_data_ptr = (png_bytep) NEW(rowbytes*height, png_byte);
  read_image_data(png_ptr, stream_data_ptr, height, rowbytes);

  /* Non-NULL intent means there is valid sRGB chunk. */
  intent = get_rendering_intent(png_ptr, png_info_ptr);
  if (intent)
    pdf_add_dict(stream_dict, pdf_new_name("Intent"), intent);

  switch (color_type) {
  case PNG_COLOR_TYPE_PALETTE:

    colorspace = create_cspace_Indexed(png_ptr, png_info_ptr);

    switch (trans_type) {
    case PDF_TRANS_TYPE_BINARY:
      /* Color-key masking */
      mask = create_ckey_mask(png_ptr, png_info_ptr);
      break;
    case PDF_TRANS_TYPE_ALPHA:
      /* Soft mask */
      mask = create_soft_mask(png_ptr, png_info_ptr, stream_data_ptr, width, height);
      break;
    default:
      /* Nothing to be done here.
       * No tRNS chunk or image already composited with background color.
       */
      break;
    }
    info.num_components = 1;

    break;
  case PNG_COLOR_TYPE_RGB:
  case PNG_COLOR_TYPE_RGB_ALPHA:

    if (png_get_valid(png_ptr, png_info_ptr, PNG_INFO_iCCP))
      colorspace = create_cspace_ICCBased(png_ptr, png_info_ptr);
    else if (intent) {
      colorspace = create_cspace_sRGB(png_ptr, png_info_ptr);
    } else {
      colorspace = create_cspace_CalRGB(png_ptr, png_info_ptr);
    }
    if (!colorspace)
      colorspace = pdf_new_name("DeviceRGB");

    switch (trans_type) {
    case PDF_TRANS_TYPE_BINARY:
      mask = create_ckey_mask(png_ptr, png_info_ptr);
      break;
    /* rowbytes changes 4 to 3 at here */
    case PDF_TRANS_TYPE_ALPHA:
      mask = strip_soft_mask(png_ptr, png_info_ptr,
                             stream_data_ptr, &rowbytes, width, height);
      break;
    default:
      mask = NULL;
    }
    info.num_components = 3;
    break;

  case PNG_COLOR_TYPE_GRAY:
  case PNG_COLOR_TYPE_GRAY_ALPHA:

    if (png_get_valid(png_ptr, png_info_ptr, PNG_INFO_iCCP))
      colorspace = create_cspace_ICCBased(png_ptr, png_info_ptr);
    else if (intent) {
      colorspace = create_cspace_sRGB(png_ptr, png_info_ptr);
    } else {
      colorspace = create_cspace_CalGray(png_ptr, png_info_ptr);
    }
    if (!colorspace)
      colorspace = pdf_new_name("DeviceGray");

    switch (trans_type) {
    case PDF_TRANS_TYPE_BINARY:
      mask = create_ckey_mask(png_ptr, png_info_ptr);
      break;
    case PDF_TRANS_TYPE_ALPHA:
      mask = strip_soft_mask(png_ptr, png_info_ptr,
                             stream_data_ptr, &rowbytes, width, height);
      break;
    default:
      mask = NULL;
    }
    info.num_components = 1;
    break;

  default:
    WARN("%s: Unknown PNG colortype %d.", PNG_DEBUG_STR, color_type);
  }
  pdf_add_dict(stream_dict, pdf_new_name("ColorSpace"), colorspace);

  pdf_add_stream(stream, stream_data_ptr, rowbytes*height);
  RELEASE(stream_data_ptr);

  if (mask) {
    if (trans_type == PDF_TRANS_TYPE_BINARY)
      pdf_add_dict(stream_dict, pdf_new_name("Mask"), mask);
    else if (trans_type == PDF_TRANS_TYPE_ALPHA) {
      if (info.bits_per_component >= 8 && info.width > 64) {
        pdf_stream_set_predictor(mask, 2, info.width,
                                 info.bits_per_component, 1);
      }
      pdf_add_dict(stream_dict, pdf_new_name("SMask"), pdf_ref_obj(mask));
      pdf_release_obj(mask);
    } else {
      WARN("%s: Unknown transparency type...???", PNG_DEBUG_STR);
      pdf_release_obj(mask);
    }
  }

  /* Finally read XMP Metadata
   * See, XMP Specification Part 3, Storage in Files
   * http://www.adobe.com/jp/devnet/xmp.html
   *
   * We require libpng version >= 1.6.14 since prior versions
   * of libpng had a bug that incorrectly treat the compression
   * flag of iTxt chunks.
   */
#if PNG_LIBPNG_VER >= 10614
  if (pdf_get_version() >= 4) {
    png_textp text_ptr;
    pdf_obj  *XMP_stream, *XMP_stream_dict;
    int       i, num_text;
    int       have_XMP = 0;

    num_text = png_get_text(png_ptr, png_info_ptr, &text_ptr, NULL);
    for (i = 0; i < num_text; i++) {
      if (!memcmp(text_ptr[i].key, "XML:com.adobe.xmp", 17)) {
        /* XMP found */
        if (text_ptr[i].compression != PNG_ITXT_COMPRESSION_NONE ||
            text_ptr[i].itxt_length == 0)
          WARN("%s: Invalid value(s) in iTXt chunk for XMP Metadata.", PNG_DEBUG_STR);
        else if (have_XMP)
          WARN("%s: Multiple XMP Metadata. Don't know how to treat it.", PNG_DEBUG_STR);
        else {
          /* We compress XMP metadata for included images here.
           * It is not recommended to compress XMP metadata for PDF documents but
           * we compress XMP metadata for included images here to avoid confusing
           * application programs that only want PDF document global XMP metadata
           * and scan for that.
           */
          XMP_stream = pdf_new_stream(STREAM_COMPRESS);
          XMP_stream_dict = pdf_stream_dict(XMP_stream);
          pdf_add_dict(XMP_stream_dict,
                       pdf_new_name("Type"), pdf_new_name("Metadata"));
          pdf_add_dict(XMP_stream_dict,
                       pdf_new_name("Subtype"), pdf_new_name("XML"));
          pdf_add_stream(XMP_stream, text_ptr[i].text, text_ptr[i].itxt_length);
          pdf_add_dict(stream_dict,
                       pdf_new_name("Metadata"), pdf_ref_obj(XMP_stream));
          pdf_release_obj(XMP_stream);
          have_XMP = 1;
        }
      }
    }
  }
#endif /* PNG_LIBPNG_VER */

  png_read_end(png_ptr, NULL);

  /* Cleanup */
  if (png_info_ptr)
    png_destroy_info_struct(png_ptr, &png_info_ptr);
  if (png_ptr)
    png_destroy_read_struct(&png_ptr, NULL, NULL);
  if (color_type != PNG_COLOR_TYPE_PALETTE &&
      info.bits_per_component >= 8 &&
      info.height > 64) {
    pdf_stream_set_predictor(stream, 15, info.width,
                             info.bits_per_component, info.num_components);
  }
  pdf_ximage_set_image(ximage, &info, stream);

  return 0;
}

/*
 * The returned value trans_type is the type of transparency to be used for
 * this image. Possible values are:
 *
 *   PDF_TRANS_TYPE_NONE    No Masking will be used/required.
 *   PDF_TRANS_TYPE_BINARY  Pixels are either fully opaque/fully transparent.
 *   PDF_TRANS_TYPE_ALPHA   Uses alpha channel, requies SMask.(PDF-1.4)
 *
 * check_transparency() must check the current setting of output PDF version
 * and must choose appropriate trans_type value according to PDF version of
 * current output PDF document.
 *
 * If the PDF version is less than 1.3, no transparency is supported for this
 * version of PDF, hence PDF_TRANS_TYPE_NONE must be returned. And when the PDF
 * version is equal to 1.3, possible retrun values are PDF_TRANS_TYPE_BINARY or
 * PDF_TRANS_TYPE_NONE. The latter case arises when PNG file uses alpha channel
 * explicitly (color type PNG_COLOR_TYPE_XXX_ALPHA), or the tRNS chunk for the
 * PNG_COLOR_TYPE_PALETTE image contains intermediate values of opacity.
 *
 * Finally, in the case of PDF version 1.4, all kind of translucent pixels can
 * be represented with Soft-Mask.
 */

static int
check_transparency (png_structp png_ptr, png_infop info_ptr)
{
  int           trans_type;
  unsigned      pdf_version;
  png_byte      color_type;
  png_color_16p trans_values;
  png_bytep     trans;
  int           num_trans;

  pdf_version = pdf_get_version();
  color_type  = png_get_color_type(png_ptr, info_ptr);

  /*
   * First we set trans_type to appropriate value for PNG image.
   */
  if (color_type == PNG_COLOR_TYPE_RGB_ALPHA ||
      color_type == PNG_COLOR_TYPE_GRAY_ALPHA) {
    trans_type = PDF_TRANS_TYPE_ALPHA;
  } else if (png_get_valid(png_ptr, info_ptr, PNG_INFO_tRNS) &&
	     png_get_tRNS(png_ptr, info_ptr, &trans, &num_trans, &trans_values)) {
    /* Have valid tRNS chunk. */
    switch (color_type) {
    case PNG_COLOR_TYPE_PALETTE:
      /* Use color-key mask if possible. */
      trans_type = PDF_TRANS_TYPE_BINARY;
      while (num_trans-- > 0) {
	if (trans[num_trans] != 0x00 && trans[num_trans] != 0xff) {
	  /* This seems not binary transparency */
	  trans_type = PDF_TRANS_TYPE_ALPHA;
	  break;
	}
      }
      break;
    case PNG_COLOR_TYPE_GRAY:
    case PNG_COLOR_TYPE_RGB:
      /* RGB or GRAY, single color specified by trans_values is transparent. */
      trans_type = PDF_TRANS_TYPE_BINARY;
      break;
    default:
      /* Else tRNS silently ignored. */
      trans_type = PDF_TRANS_TYPE_NONE;
    }
  } else { /* no transparency */
    trans_type = PDF_TRANS_TYPE_NONE;
  }

  /*
   * Now we check PDF version.
   * We can convert alpha cahnnels to explicit mask via user supplied alpha-
   * threshold value. But I will not do that.
   */
  if (( pdf_version < 3 && trans_type != PDF_TRANS_TYPE_NONE   ) ||
      ( pdf_version < 4 && trans_type == PDF_TRANS_TYPE_ALPHA )) {
    /*
     *   No transparency supported but PNG uses transparency, or Soft-Mask
     * required but no support for it is available in this version of PDF.
     * We must do pre-composition of image with the background image here. But,
     * we cannot do that in general since dvipdfmx is not a rasterizer. What we
     * can do here is to composite image with a rectangle filled with the
     * background color. However, images are stored as an Image XObject which
     * can be referenced anywhere in the PDF document content. Hence, we cannot
     * know the correct background color at this time. So we will choose white
     * as background color, which is most probable color in our cases.
     * We ignore bKGD chunk.
     */
    png_color_16 bg;
    bg.red = 255; bg.green = 255; bg.blue  = 255; bg.gray = 255; bg.index = 0;
    png_set_background(png_ptr, &bg, PNG_BACKGROUND_GAMMA_SCREEN, 0, 1.0);
    WARN("%s: Transparency will be ignored. (no support in PDF ver. < 1.3)", PNG_DEBUG_STR);
    if (pdf_version < 3)
      WARN("%s: Please use -V 3 option to enable binary transparency support.", PNG_DEBUG_STR);
    if (pdf_version < 4)
      WARN("%s: Please use -V 4 option to enable full alpha channel support.", PNG_DEBUG_STR);
    trans_type = PDF_TRANS_TYPE_NONE;
  }

  return trans_type;
}

/*
 * sRGB:
 *
 *   If sRGB chunk is present, cHRM and gAMA chunk must be ignored.
 *
 */
static pdf_obj *
get_rendering_intent (png_structp png_ptr, png_infop info_ptr)
{
  pdf_obj *intent;
  int      srgb_intent;

  if (png_get_valid(png_ptr, info_ptr, PNG_INFO_sRGB) &&
      png_get_sRGB (png_ptr, info_ptr, &srgb_intent)) {
    switch (srgb_intent) {
    case PNG_sRGB_INTENT_SATURATION:
      intent = pdf_new_name("Saturation");
      break;
    case PNG_sRGB_INTENT_PERCEPTUAL:
      intent = pdf_new_name("Perceptual");
      break;
    case PNG_sRGB_INTENT_ABSOLUTE:
      intent = pdf_new_name("AbsoluteColorimetric");
      break;
    case PNG_sRGB_INTENT_RELATIVE:
      intent = pdf_new_name("RelativeColorimetric");
      break;
    default:
      WARN("%s: Invalid value in PNG sRGB chunk: %d", PNG_DEBUG_STR, srgb_intent);
      intent = NULL;
    }
  } else
    intent = NULL;

  return intent;
}

/* Approximated sRGB */
static pdf_obj *
create_cspace_sRGB (png_structp png_ptr, png_infop info_ptr)
{
  pdf_obj  *colorspace;
  pdf_obj  *cal_param;
  png_byte  color_type;

  color_type = png_get_color_type(png_ptr, info_ptr);

  /* Parameters taken from PNG spec. section 4.2.2.3. */
  cal_param = make_param_Cal(color_type,
			     2.2,
			     0.3127, 0.329,
			     0.64, 0.33, 0.3, 0.6, 0.15, 0.06);
  if (!cal_param)
    return NULL;

  colorspace = pdf_new_array();

  switch (color_type) {
  case PNG_COLOR_TYPE_RGB:
  case PNG_COLOR_TYPE_RGB_ALPHA:
  case PNG_COLOR_TYPE_PALETTE:
    pdf_add_array(colorspace, pdf_new_name("CalRGB"));
    break;
  case PNG_COLOR_TYPE_GRAY:
  case PNG_COLOR_TYPE_GRAY_ALPHA:
    pdf_add_array(colorspace, pdf_new_name("CalGray"));
    break;
  }
  pdf_add_array(colorspace, cal_param);

  return colorspace;
}

static pdf_obj *
create_cspace_ICCBased (png_structp png_ptr, png_infop info_ptr)
{
  pdf_obj   *colorspace;
  int        csp_id, colortype;
  png_byte   color_type;
  png_charp  name;
  int        compression_type;  /* Manual page for libpng does not
				 * clarify whether profile data is inflated by libpng.
				 */
#if PNG_LIBPNG_VER_MINOR < 5
  png_charp   profile;
#else
  png_bytep   profile;
#endif
  png_uint_32 proflen;

  if (!png_get_valid(png_ptr, info_ptr, PNG_INFO_iCCP) ||
      !png_get_iCCP(png_ptr, info_ptr, &name, &compression_type, &profile, &proflen))
    return NULL;

  color_type = png_get_color_type(png_ptr, info_ptr);

  if (color_type & PNG_COLOR_MASK_COLOR) {
    colortype = PDF_COLORSPACE_TYPE_RGB;
#if 0
    alternate = create_cspace_CalRGB(png_ptr, info_ptr);
#endif
  } else {
    colortype = PDF_COLORSPACE_TYPE_GRAY;
#if 0
    alternate = create_cspace_CalGray(png_ptr, info_ptr);
#endif
  }

#if 0
  if (alternate)
    pdf_add_dict(dict, pdf_new_name("Alternate"), alternate);
#endif

  if (iccp_check_colorspace(colortype, profile, proflen) < 0)
    colorspace = NULL;
  else {
    csp_id = iccp_load_profile(name, profile, proflen);
    if (csp_id < 0) {
      colorspace = NULL;
    } else {
      colorspace = pdf_get_colorspace_reference(csp_id);
    }
  }

  /* Rendering intent ... */

  return colorspace;
}

/*
 * gAMA, cHRM:
 *
 *   If cHRM is present, we use CIE-Based color space. gAMA is also used here
 * if available.
 */

#define INVALID_CHRM_VALUE(xw,yw,xr,yr,xg,yg,xb,yb) (\
  (xw) <= 0.0 || (yw) < 1.0e-10 || \
  (xr) < 0.0  || (yr) < 0.0 || (xg) < 0.0 || (yg) < 0.0 || \
  (xb) < 0.0  || (yb) < 0.0)

static pdf_obj *
create_cspace_CalRGB (png_structp png_ptr, png_infop info_ptr)
{
  pdf_obj *colorspace;
  pdf_obj *cal_param;
  double   xw, yw, xr, yr, xg, yg, xb, yb;
  double   G;

  if (!png_get_valid(png_ptr, info_ptr, PNG_INFO_cHRM) ||
      !png_get_cHRM(png_ptr, info_ptr, &xw, &yw, &xr, &yr, &xg, &yg, &xb, &yb))
    return NULL;

  if (xw <= 0.0 || yw < 1.0e-10 ||
      xr < 0.0  || yr < 0.0 || xg < 0.0 || yg < 0.0 || xb < 0.0 || yb < 0.0) {
    WARN("%s: Invalid cHRM chunk parameters found.", PNG_DEBUG_STR);
    return NULL;
  }

  if (png_get_valid(png_ptr, info_ptr, PNG_INFO_gAMA) &&
      png_get_gAMA (png_ptr, info_ptr, &G)) {
    if (G < 1.0e-2) {
      WARN("%s: Unusual Gamma value: 1.0 / %g", PNG_DEBUG_STR, G);
      return NULL;
    }
    G = 1.0 / G; /* Gamma is inverted. */
  } else {
    G = DPX_PNG_DEFAULT_GAMMA;
  }

  cal_param = make_param_Cal(PNG_COLOR_TYPE_RGB, G, xw, yw, xr, yr, xg, yg, xb, yb);

  if (!cal_param)
    return NULL;

  colorspace = pdf_new_array();
  pdf_add_array(colorspace, pdf_new_name("CalRGB"));
  pdf_add_array(colorspace, cal_param);

  return colorspace;
}

static pdf_obj *
create_cspace_CalGray (png_structp png_ptr, png_infop info_ptr)
{
  pdf_obj *colorspace;
  pdf_obj *cal_param;
  double   xw, yw, xr, yr, xg, yg, xb, yb;
  double   G;

  if (!png_get_valid(png_ptr, info_ptr, PNG_INFO_cHRM) ||
      !png_get_cHRM(png_ptr, info_ptr, &xw, &yw, &xr, &yr, &xg, &yg, &xb, &yb))
    return NULL;

  if (xw <= 0.0 || yw < 1.0e-10 ||
      xr < 0.0  || yr < 0.0 || xg < 0.0 || yg < 0.0 || xb < 0.0 || yb < 0.0) {
    WARN("%s: Invalid cHRM chunk parameters found.", PNG_DEBUG_STR);
    return NULL;
  }

  if (png_get_valid(png_ptr, info_ptr, PNG_INFO_gAMA) &&
      png_get_gAMA (png_ptr, info_ptr, &G)) {
    if (G < 1.0e-2) {
      WARN("%s: Unusual Gamma value: 1.0 / %g", PNG_DEBUG_STR, G);
      return NULL;
    }
    G = 1.0 / G; /* Gamma is inverted. */
  } else {
    G = DPX_PNG_DEFAULT_GAMMA;
  }

  cal_param = make_param_Cal(PNG_COLOR_TYPE_GRAY, G, xw, yw, xr, yr, xg, yg, xb, yb);

  if (!cal_param)
    return NULL;

  colorspace = pdf_new_array();
  pdf_add_array(colorspace, pdf_new_name("CalGray"));
  pdf_add_array(colorspace, cal_param);

  return colorspace;
}

static pdf_obj *
make_param_Cal (png_byte color_type,
		double G, /* Gamma */
		double xw, double yw,
		double xr, double yr, double xg, double yg, double xb, double yb)
{
  pdf_obj *cal_param;
  pdf_obj *white_point, *matrix, *dev_gamma;
  double Xw, Yw, Zw; /* Yw = 1.0 */
  double Xr, Xg, Xb, Yr, Yb, Yg, Zr, Zg, Zb;

#ifndef ABS
#define ABS(x) ((x) < 0 ? -(x) : (x))
#endif
  /*
   * TODO: Check validity
   *
   * Conversion found in
   *
   *  com.sixlegs.image.png - Java package to read and display PNG images
   *  Copyright (C) 1998, 1999, 2001 Chris Nokleberg
   *
   *  http://www.sixlegs.com/software/png/
   *
   */
  {
    double zw, zr, zg, zb;
    double fr, fg, fb;
    double det;

    /* WhitePoint */
    zw = 1 - (xw + yw);
    zr = 1 - (xr + yr); zg = 1 - (xg + yg); zb = 1 - (xb + yb);
    Xw = xw / yw; Yw = 1.0; Zw = zw / yw;

    /* Matrix */
    det = xr * (yg * zb - zg * yb) - xg * (yr * zb - zr * yb) + xb * (yr * zg - zr * yg);
    if (ABS(det) < 1.0e-10) {
      WARN("Non invertible matrix: Maybe invalid value(s) specified in cHRM chunk.");
      return NULL;
    }
    fr  = (Xw * (yg * zb - zg * yb) - xg * (zb - Zw * yb) + xb * (zg - Zw * yg)) / det;
    fg  = (xr * (zb - Zw * yb) - Xw * (yr * zb - zr * yb) + xb * (yr * Zw - zr)) / det;
    fb  = (xr * (yg * Zw - zg) - xg * (yr * Zw - zr) + Xw * (yr * zg - zr * yg)) / det;
    Xr = fr * xr; Yr = fr * yr; Zr = fr * zr;
    Xg = fg * xg; Yg = fg * yg; Zg = fg * zg;
    Xb = fb * xb; Yb = fb * yb; Zb = fb * zb;
  }

  if (G < 1.0e-2) {
    WARN("Unusual Gamma specified: 1.0 / %g", G);
    return NULL;
  }

  cal_param = pdf_new_dict();

  /* White point is always required. */
  white_point = pdf_new_array();
  pdf_add_array(white_point, pdf_new_number(ROUND(Xw, 0.00001)));
  pdf_add_array(white_point, pdf_new_number(ROUND(Yw, 0.00001)));
  pdf_add_array(white_point, pdf_new_number(ROUND(Zw, 0.00001)));
  pdf_add_dict(cal_param, pdf_new_name("WhitePoint"), white_point);

  /* Matrix - default: Identity */
  if (color_type & PNG_COLOR_MASK_COLOR) {
    if (G != 1.0) {
      dev_gamma = pdf_new_array();
      pdf_add_array(dev_gamma, pdf_new_number(ROUND(G, 0.00001)));
      pdf_add_array(dev_gamma, pdf_new_number(ROUND(G, 0.00001)));
      pdf_add_array(dev_gamma, pdf_new_number(ROUND(G, 0.00001)));
      pdf_add_dict(cal_param, pdf_new_name("Gamma"), dev_gamma);
    }

    matrix = pdf_new_array();
    pdf_add_array(matrix, pdf_new_number(ROUND(Xr, 0.00001)));
    pdf_add_array(matrix, pdf_new_number(ROUND(Yr, 0.00001)));
    pdf_add_array(matrix, pdf_new_number(ROUND(Zr, 0.00001)));
    pdf_add_array(matrix, pdf_new_number(ROUND(Xg, 0.00001)));
    pdf_add_array(matrix, pdf_new_number(ROUND(Yg, 0.00001)));
    pdf_add_array(matrix, pdf_new_number(ROUND(Zg, 0.00001)));
    pdf_add_array(matrix, pdf_new_number(ROUND(Xb, 0.00001)));
    pdf_add_array(matrix, pdf_new_number(ROUND(Yb, 0.00001)));
    pdf_add_array(matrix, pdf_new_number(ROUND(Zb, 0.00001)));
    pdf_add_dict (cal_param, pdf_new_name("Matrix"), matrix);
  } else { /* Gray */
    if (G != 1.0)
      pdf_add_dict(cal_param,
		   pdf_new_name("Gamma"),
		   pdf_new_number(ROUND(G, 0.00001)));
  }

  return cal_param;
}

/*
 * Set up Indexed ColorSpace for color-type PALETTE:
 *
 *  PNG allows only RGB color for base color space. If gAMA and/or cHRM
 *  chunk is available, we can use CalRGB color space instead of DeviceRGB
 *  for base color space.
 *
 */
static pdf_obj *
create_cspace_Indexed (png_structp png_ptr, png_infop info_ptr)
{
  pdf_obj   *colorspace;
  pdf_obj   *base, *lookup;
  png_byte  *data_ptr;
  png_colorp plte;
  int        num_plte, i;

  if (!png_get_valid(png_ptr, info_ptr, PNG_INFO_PLTE) ||
      !png_get_PLTE(png_ptr, info_ptr, &plte, &num_plte)) {
    WARN("%s: PNG does not have valid PLTE chunk.", PNG_DEBUG_STR);
    return NULL;
  }

  /* Order is important. */
  colorspace = pdf_new_array ();
  pdf_add_array(colorspace, pdf_new_name("Indexed"));

  if (png_get_valid(png_ptr, info_ptr, PNG_INFO_iCCP))
    base = create_cspace_ICCBased(png_ptr, info_ptr);
  else {
    if (png_get_valid(png_ptr, info_ptr, PNG_INFO_sRGB))
      base = create_cspace_sRGB(png_ptr, info_ptr);
    else
      base = create_cspace_CalRGB(png_ptr, info_ptr);
  }

  if (!base)
    base = pdf_new_name("DeviceRGB");

  pdf_add_array(colorspace, base);
  pdf_add_array(colorspace, pdf_new_number(num_plte-1));
  data_ptr = NEW(num_plte*3, png_byte);
  for (i = 0; i < num_plte; i++) {
    data_ptr[3*i]   = plte[i].red;
    data_ptr[3*i+1] = plte[i].green;
    data_ptr[3*i+2] = plte[i].blue;
  }
  lookup = pdf_new_string(data_ptr, num_plte*3);
  RELEASE(data_ptr);
  pdf_add_array(colorspace, lookup);

  return colorspace;
}

/*
 * Colorkey Mask: array
 *
 *  [component_0_min component_0_max ... component_n_min component_n_max]
 *
 */

static pdf_obj *
create_ckey_mask (png_structp png_ptr, png_infop info_ptr)
{
  pdf_obj  *colorkeys;
  png_byte  color_type;
  png_bytep trans;
  int       num_trans, i;
  png_color_16p colors;

  if (!png_get_valid(png_ptr, info_ptr, PNG_INFO_tRNS) ||
      !png_get_tRNS(png_ptr, info_ptr, &trans, &num_trans, &colors)) {
    WARN("%s: PNG does not have valid tRNS chunk!", PNG_DEBUG_STR);
    return NULL;
  }

  colorkeys  = pdf_new_array();
  color_type = png_get_color_type(png_ptr, info_ptr);

  switch (color_type) {
  case PNG_COLOR_TYPE_PALETTE:
    for (i = 0; i < num_trans; i++) {
      if (trans[i] == 0x00) {
        pdf_add_array(colorkeys, pdf_new_number(i));
        pdf_add_array(colorkeys, pdf_new_number(i));
      } else if (trans[i] != 0xff) {
        WARN("%s: You found a bug in pngimage.c.", PNG_DEBUG_STR);
      }
    }
    break;
  case PNG_COLOR_TYPE_RGB:
    pdf_add_array(colorkeys, pdf_new_number(colors->red));
    pdf_add_array(colorkeys, pdf_new_number(colors->red));
    pdf_add_array(colorkeys, pdf_new_number(colors->green));
    pdf_add_array(colorkeys, pdf_new_number(colors->green));
    pdf_add_array(colorkeys, pdf_new_number(colors->blue));
    pdf_add_array(colorkeys, pdf_new_number(colors->blue));
    break;
  case PNG_COLOR_TYPE_GRAY:
    pdf_add_array(colorkeys, pdf_new_number(colors->gray));
    pdf_add_array(colorkeys, pdf_new_number(colors->gray));
    break;
  default:
    WARN("%s: You found a bug in pngimage.c.", PNG_DEBUG_STR);
    pdf_release_obj(colorkeys);
    colorkeys = NULL;
  }

  return colorkeys;
}

/*
 * Soft-Mask: stream
 *
 *   <<
 *      /Type             /XObject
 *      /Subtype          /Image
 *      /Width            -int-
 *      /Height           -int-
 *      /BitsPerComponent bpc
 *   >>
 *   stream .... endstream
 *
 *   ColorSpace, Mask, SMask must be absent. ImageMask must be false or absent.
 */

static pdf_obj *
create_soft_mask (png_structp png_ptr, png_infop info_ptr,
		  png_bytep image_data_ptr, png_uint_32 width, png_uint_32 height)
{
  pdf_obj    *smask, *dict;
  png_bytep   smask_data_ptr;
  png_bytep   trans;
  int         num_trans;
  png_uint_32 i;

  if (!png_get_valid(png_ptr, info_ptr, PNG_INFO_tRNS) ||
      !png_get_tRNS(png_ptr, info_ptr, &trans, &num_trans, NULL)) {
    WARN("%s: PNG does not have valid tRNS chunk but tRNS is requested.", PNG_DEBUG_STR);
    return NULL;
  }

  smask = pdf_new_stream(STREAM_COMPRESS);
  dict  = pdf_stream_dict(smask);
  smask_data_ptr = (png_bytep) NEW(width*height, png_byte);
  pdf_add_dict(dict, pdf_new_name("Type"),    pdf_new_name("XObject"));
  pdf_add_dict(dict, pdf_new_name("Subtype"), pdf_new_name("Image"));
  pdf_add_dict(dict, pdf_new_name("Width"),      pdf_new_number(width));
  pdf_add_dict(dict, pdf_new_name("Height"),     pdf_new_number(height));
  pdf_add_dict(dict, pdf_new_name("ColorSpace"), pdf_new_name("DeviceGray"));
  pdf_add_dict(dict, pdf_new_name("BitsPerComponent"), pdf_new_number(8));
  for (i = 0; i < width*height; i++) {
    png_byte idx = image_data_ptr[i];
    smask_data_ptr[i] = (idx < num_trans) ? trans[idx] : 0xff;
  }
  pdf_add_stream(smask, (char *)smask_data_ptr, width*height);
  RELEASE(smask_data_ptr);

  return smask;
}

/* bitdepth is always 8 (16 is not supported) */
static pdf_obj *
strip_soft_mask (png_structp png_ptr, png_infop info_ptr,
		 /* next two values will be modified. */
		 png_bytep image_data_ptr, png_uint_32p rowbytes_ptr,
		 png_uint_32 width, png_uint_32 height)
{
  pdf_obj    *smask, *dict;
  png_byte    color_type, bpc;
  png_bytep   smask_data_ptr;
  png_uint_32 i;

  color_type = png_get_color_type(png_ptr, info_ptr);
  bpc        = png_get_bit_depth (png_ptr, info_ptr);
  if (color_type & PNG_COLOR_MASK_COLOR) {
    int bps = (bpc == 8) ? 4 : 8;
    if (*rowbytes_ptr != bps*width*sizeof(png_byte)) { /* Something wrong */
      WARN("%s: Inconsistent rowbytes value.", PNG_DEBUG_STR);
      return NULL;
    }
  } else {
    int bps = (bpc == 8) ? 2 : 4;
    if (*rowbytes_ptr != bps*width*sizeof(png_byte)) { /* Something wrong */
      WARN("%s: Inconsistent rowbytes value.", PNG_DEBUG_STR);
      return NULL;
    }
  }

  smask = pdf_new_stream(STREAM_COMPRESS);
  dict  = pdf_stream_dict(smask);
  pdf_add_dict(dict, pdf_new_name("Type"),    pdf_new_name("XObject"));
  pdf_add_dict(dict, pdf_new_name("Subtype"), pdf_new_name("Image"));
  pdf_add_dict(dict, pdf_new_name("Width"),      pdf_new_number(width));
  pdf_add_dict(dict, pdf_new_name("Height"),     pdf_new_number(height));
  pdf_add_dict(dict, pdf_new_name("ColorSpace"), pdf_new_name("DeviceGray"));
  pdf_add_dict(dict, pdf_new_name("BitsPerComponent"), pdf_new_number(bpc));

  smask_data_ptr = (png_bytep) NEW((bpc/8)*width*height, png_byte);

  switch (color_type) {
  case PNG_COLOR_TYPE_RGB_ALPHA:
    if (bpc == 8) {
    for (i = 0; i < width*height; i++) {
      memmove(image_data_ptr+(3*i), image_data_ptr+(4*i), 3);
      smask_data_ptr[i] = image_data_ptr[4*i+3];
    }
    *rowbytes_ptr = 3*width*sizeof(png_byte);
    } else {
      for (i = 0; i < width*height; i++) {
        memmove(image_data_ptr+(6*i), image_data_ptr+(8*i), 6);
        smask_data_ptr[2*i]   = image_data_ptr[8*i+6];
        smask_data_ptr[2*i+1] = image_data_ptr[8*i+7];
      }
      *rowbytes_ptr = 6*width*sizeof(png_byte);
    }
    break;
  case PNG_COLOR_TYPE_GRAY_ALPHA:
    if (bpc == 8) {
    for (i = 0; i < width*height; i++) {
      image_data_ptr[i] = image_data_ptr[2*i];
      smask_data_ptr[i] = image_data_ptr[2*i+1];
    }
    *rowbytes_ptr = width*sizeof(png_byte);
    } else {
      for (i = 0; i < width*height; i++) {
        image_data_ptr[2*i]   = image_data_ptr[4*i];
        image_data_ptr[2*i+1] = image_data_ptr[4*i+1];
        smask_data_ptr[2*i]   = image_data_ptr[4*i+2];
        smask_data_ptr[2*i+1] = image_data_ptr[4*i+3];
      }
      *rowbytes_ptr = 2*width*sizeof(png_byte);      
    }
    break;
  default:
    WARN("You found a bug in pngimage.c!");
    pdf_release_obj(smask);
    RELEASE(smask_data_ptr);
    return NULL;
  }

  pdf_add_stream(smask, smask_data_ptr, (bpc/8)*width*height);
  RELEASE(smask_data_ptr);

  return smask;
}

static void
read_image_data (png_structp png_ptr, png_bytep dest_ptr,
                 png_uint_32 height, png_uint_32 rowbytes)
{
  png_bytepp  rows_p;
  png_uint_32 i;

  rows_p = (png_bytepp) NEW (height, png_bytep);
  for (i=0; i< height; i++)
    rows_p[i] = dest_ptr + (rowbytes * i);
  png_read_image(png_ptr, rows_p);
  RELEASE(rows_p);
}

int
png_get_bbox (FILE *png_file, uint32_t *width, uint32_t *height,
	       double *xdensity, double *ydensity)
{
  png_structp png_ptr;
  png_infop   png_info_ptr;

  rewind (png_file);
  png_ptr = png_create_read_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, warn);
  if (png_ptr == NULL ||
      (png_info_ptr = png_create_info_struct (png_ptr)) == NULL) {
    WARN("%s: Creating Libpng read/info struct failed.", PNG_DEBUG_STR);
    if (png_ptr)
      png_destroy_read_struct(&png_ptr, NULL, NULL);
    return -1;
  }

  /* Inititializing file IO. */
  png_init_io (png_ptr, png_file);

  /* Read PNG info-header and get some info. */
  png_read_info(png_ptr, png_info_ptr);
  *width      = png_get_image_width (png_ptr, png_info_ptr);
  *height     = png_get_image_height(png_ptr, png_info_ptr);

  if (compat_mode)
    *xdensity = *ydensity = 72.0 / 100.0;
  else
  {
    png_uint_32 xppm = png_get_x_pixels_per_meter(png_ptr, png_info_ptr);
    png_uint_32 yppm = png_get_y_pixels_per_meter(png_ptr, png_info_ptr);

    *xdensity = xppm ? 72.0 / 0.0254 / xppm : 1.0;
    *ydensity = yppm ? 72.0 / 0.0254 / yppm : 1.0;
  }

  /* Cleanup */
  if (png_info_ptr)
    png_destroy_info_struct(png_ptr, &png_info_ptr);
  if (png_ptr)
    png_destroy_read_struct(&png_ptr, NULL, NULL);

  return 0;
}

#endif /* HAVE_LIBPNG */
