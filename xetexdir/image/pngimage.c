/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2006 by SIL International

 SIL Author(s): Jonathan Kew

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
\****************************************************************************/

/* this file is derived from the dvipdfmx project;
   the original header follows... */

/*  $Header: /home/cvsroot/dvipdfmx/src/pngimage.c,v 1.24 2004/09/11 14:50:29 hirata Exp $

    This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team <dvipdfmx@project.ktug.or.kr>
    
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

#if HAVE_CONFIG_H
#include <w2c/config.h>
#endif

/*
 * PNG SUPPORT
 *
 *  All bitdepth less than 16 is supported.
 *  Supported color types are: PALETTE, RGB, GRAY, RGB_ALPHA, GRAY_ALPHA.
 *  Supported ancillary chunks: tRNS, cHRM + gAMA, (sRGB), (iCCP)
 * 
 *  gAMA support is available only when cHRM exists. cHRM support is not
 *  tested well. CalRGB/CalGray colorspace is used for PNG images that
 *  have cHRM chunk (but not sRGB).
 *
 * LIMITATIONS
 *
 *   Recent version of PDF (>= 1.5) support 16 bpc, but 16 bit bitdepth PNG
 *   images are automatically converted to 8 bit bitpedth image.
 *
 * TODO
 *
 *  sBIT ? iTXT, tEXT and tIME as MetaData ?, pHYS (see below)
 *  16 bpc support for PDF-1.5. JBIG compression for monochrome image.
 *  Predictor for deflate ?
 */

#define PNG_DEBUG_STR "PNG"
#define PNG_DEBUG     3

/*
 * Write, MNG, Progressive not required.
 */
#define PNG_NO_WRITE_SUPPORTED
#define PNG_NO_MNG_FEATURES
#define PNG_NO_PROGRESSIVE_READ
#if 0
/* 16_TO_8 required. */
#define PNG_NO_READ_TRANSFORMS
#endif

#include <png.h>
#include "pngimage.h"

#define PDF_TRANS_TYPE_NONE   0
#define PDF_TRANS_TYPE_BINARY 1
#define PDF_TRANS_TYPE_ALPHA  2

static void warn(png_structp png_ptr, png_const_charp msg)
{
  (void)png_ptr; (void)msg; /* Make compiler happy */
}

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

int
png_scan_file (struct png_info *info, FILE *png_file)
{
  /* Libpng stuff */
  png_structp png_ptr;
  png_infop   png_info_ptr;
  png_byte    bpc;
  png_uint_32 width, height;
  
  rewind (png_file);
  png_ptr = png_create_read_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, warn);
  if (png_ptr == NULL || 
      (png_info_ptr = png_create_info_struct (png_ptr)) == NULL) {
    fprintf(stderr, "WARNING: %s: Creating Libpng read/info struct failed.", PNG_DEBUG_STR);
    if (png_ptr)
      png_destroy_read_struct(&png_ptr, NULL, NULL);
    return -1;
  }

  /* Inititializing file IO. */
  png_init_io (png_ptr, png_file);

  /* Read PNG info-header and get some info. */
  png_read_info(png_ptr, png_info_ptr);
  width      = png_get_image_width (png_ptr, png_info_ptr);
  height     = png_get_image_height(png_ptr, png_info_ptr);
  bpc        = png_get_bit_depth   (png_ptr, png_info_ptr);

  info->xdpi  = png_get_x_pixels_per_meter(png_ptr, png_info_ptr) * 0.0254;
  info->ydpi  = png_get_y_pixels_per_meter(png_ptr, png_info_ptr) * 0.0254;

  if (info->xdpi == 0)
    info->xdpi = 72;
  if (info->ydpi == 0)
    info->ydpi = 72;

  /* Values listed below will not be modified in the remaining process. */
  info->width  = width;
  info->height = height;
  info->bits_per_component = bpc;

  /* Cleanup */
  if (png_info_ptr)
    png_destroy_info_struct(png_ptr, &png_info_ptr);
  if (png_ptr)
    png_destroy_read_struct(&png_ptr, NULL, NULL);

  return 0;
}
