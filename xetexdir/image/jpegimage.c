/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2009 by SIL International

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

/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2015 by Jin-Hwan Cho and Shunsaku Hirata,
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
 * JPEG SUPPORT
 *
 * Accroding to Libjpeg document:
 *
 *  CAUTION: it appears that Adobe Photoshop writes inverted data in CMYK
 *  JPEG files: 0 represents 100% ink coverage, rather than 0% ink as you'd
 *  expect....
 *
 * To wrok with this problem, we must detect whether CMYK JPEG file is
 * created by Photoshop. But there are no reliable way to determine this.
 *
 * According to Adobe Technical Note #5516,
 * "Supporting the DCT Filters in PostScript Level 2", Section 18, p.27.
 *
 *  DCTDecode ignores and skips any APPE marker segment does not begin with
 *  the `Adobe' 5-character string.
 *
 * PDF Reference Manual 4th ed., p.61-62.
 *
 *  The JPEG filter implementation in Adobe Acrobat products does not
 *  support features of the JPEG standard that are irrelevant to images.
 *  In addition, certain choices have been made regarding reserved marker
 *  codes and other optional features of the standard. For details, see
 *  Adobe Technical Note #5116, Supporting the DCT Filters in PostScript
 *  Level 2.
 */

#if HAVE_CONFIG_H
#include <w2c/config.h>
#endif

#include "mfileio.h"
#include "numbers.h"
#include "jpegimage.h"

#define JPEG_DEBUG_STR "JPEG"
#define JPEG_DEBUG     3

#define HAVE_APPn_JFIF  (1 << 0)
#define HAVE_APPn_ADOBE (1 << 1)
#define HAVE_APPn_ICC   (1 << 2)
#define HAVE_APPn_Exif  (1 << 3)

#define RELEASE(p)		free(p)
#define NEW(n, t)		(t*)xmalloc(n * sizeof(t))
#define RENEW(p, n, t)	((p) ? (t*)xrealloc(p, (n) * sizeof(t)) : NEW(n, t))

int
check_for_jpeg (FILE *fp)
{
  unsigned char jpeg_sig[2];

  rewind(fp);
  if (fread(jpeg_sig, sizeof(unsigned char), 2, fp) != 2)
    return 0;
  else if (jpeg_sig[0] != 0xff || jpeg_sig[1] != JM_SOI)
    return 0;

  return 1;
}

static void
JPEG_info_init (struct JPEG_info *j_info)
{
  j_info->width  = 0;
  j_info->height = 0;
  j_info->bits_per_component = 0;
  j_info->num_components = 0;

  j_info->xdpi = 0.0;
  j_info->ydpi = 0.0;

  j_info->flags    = 0;
  j_info->num_appn = 0;
  j_info->max_appn = 0;
  j_info->appn     = NULL;

  memset(j_info->skipbits, 0, MAX_COUNT / 8 + 1);
}

static JPEG_marker
JPEG_get_marker (FILE *fp)
{
  int c;

  c = fgetc(fp);
  if (c != 255)
    return -1;

  for (;;) {
    c = fgetc(fp);
    if (c < 0)
      return -1;
    else if (c > 0 && c < 255) {
      return c;
    }
  }

  return -1;
}

static int
add_APPn_marker (struct JPEG_info *j_info,
		 JPEG_marker marker, int app_sig, void *app_data)
{
  int n;

  if (j_info->num_appn >= j_info->max_appn) {
    j_info->max_appn += 16;
    j_info->appn = RENEW(j_info->appn, j_info->max_appn, struct JPEG_ext);
  }
  n = j_info->num_appn;

  j_info->appn[n].marker   = marker;
  j_info->appn[n].app_sig  = app_sig;
  j_info->appn[n].app_data = app_data;

  j_info->num_appn += 1;

  return n;
}

static unsigned short
read_APP14_Adobe (struct JPEG_info *j_info, FILE *fp)
{
  struct JPEG_APPn_Adobe *app_data;

  app_data = NEW(1, struct JPEG_APPn_Adobe);
  app_data->version   = get_unsigned_pair(fp);
  app_data->flag0     = get_unsigned_pair(fp);
  app_data->flag1     = get_unsigned_pair(fp);
  app_data->transform = get_unsigned_byte(fp);

  add_APPn_marker(j_info, JM_APP14, JS_APPn_ADOBE, app_data);

  return 7;
}

static unsigned int
read_exif_bytes(unsigned char **p, int n, int b)
{
  unsigned int rval = 0;
  unsigned char *pp = *p;
  if (b) {
    switch (n) {
      case 4:
        rval += *pp++; rval <<= 8;
        rval += *pp++; rval <<= 8;
      case 2:
        rval += *pp++; rval <<= 8;
        rval += *pp;
        break;
    }
  }
  else {
    pp += n;
    switch (n) {
      case 4:
        rval += *--pp; rval <<= 8;
        rval += *--pp; rval <<= 8;
      case 2:
        rval += *--pp; rval <<= 8;
        rval += *--pp;
        break;
    }
  }
  *p += n;
  return rval;
}

static unsigned short
read_APP1_Exif (struct JPEG_info *j_info, FILE *fp, unsigned short length)
{
  /* this doesn't save the data, just reads the tags we need */
  /* based on info from http://www.exif.org/Exif2-2.PDF */
  unsigned char *buffer = NEW(length, unsigned char);
  unsigned char *p, *rp;
  unsigned char *tiff_header;
  char bigendian;
  int i;
  int num_fields, tag, type;
  int value = 0, num = 0, den = 0;	/* silence uninitialized warnings */
  double xres = 72.0;
  double yres = 72.0;
  double res_unit = 1.0;
  fread(buffer, length, 1, fp);
  p = buffer;
  while ((p < buffer + length) && (*p == 0))
    ++p;
  tiff_header = p;
  if ((*p == 'M') && (*(p+1) == 'M'))
    bigendian = 1;
  else if ((*p == 'I') && (*(p+1) == 'I'))
    bigendian = 0;
  else
    goto err;
  p += 2;
  i = read_exif_bytes(&p, 2, bigendian);
  if (i != 42)
    goto err;
  i = read_exif_bytes(&p, 4, bigendian);
  p = tiff_header + i;
  num_fields = read_exif_bytes(&p, 2, bigendian);
  while (num_fields-- > 0) {
    tag = read_exif_bytes(&p, 2, bigendian);
    type = read_exif_bytes(&p, 2, bigendian);
    (void)read_exif_bytes(&p, 4, bigendian);
    switch (type) {
      case 1: /* byte */
        value = *p++;
        p += 3;
        break;
      case 3: /* short */
        value = read_exif_bytes(&p, 2, bigendian);
        p += 2;
        break;
      case 4: /* long */
      case 9: /* slong */
        value = read_exif_bytes(&p, 4, bigendian);
        break;
      case 5: /* rational */
      case 10: /* srational */
        value = read_exif_bytes(&p, 4, bigendian);
        rp = tiff_header + value;
        num = read_exif_bytes(&rp, 4, bigendian);
        den = read_exif_bytes(&rp, 4, bigendian);
        break;
      case 7: /* undefined */
        value = *p++;
        p += 3;
        break;
      case 2: /* ascii */
      default:
        p += 4;
        break;
    }
    switch (tag) {
      case 282: /* x res */
        if (den != 0)
          xres = num / den;
        break;
      case 283: /* y res */
        if (den != 0)
          yres = num / den;
        break;
      case 296: /* res unit */
        switch (value) {
          case 2:
            res_unit = 1.0;
            break;
          case 3:
            res_unit = 2.54;
            break;
        }
    }
  }
/*
  Do not overwrite if j_info->xdpi and j_info->ydpi are
  already determined as JFIF
*/
  if (j_info->xdpi < 0.1 && j_info->ydpi < 0.1) {
    j_info->xdpi = xres * res_unit;
    j_info->ydpi = yres * res_unit;
  }

err:
  RELEASE(buffer);
  return length;
}

static unsigned short
read_APP0_JFIF (struct JPEG_info *j_info, FILE *fp)
{
  struct JPEG_APPn_JFIF *app_data;
  unsigned short thumb_data_len;

  app_data = NEW(1, struct JPEG_APPn_JFIF);
  app_data->version  = get_unsigned_pair(fp);
  app_data->units    = get_unsigned_byte(fp);
  app_data->Xdensity = get_unsigned_pair(fp);
  app_data->Ydensity = get_unsigned_pair(fp);
  app_data->Xthumbnail = get_unsigned_byte(fp);
  app_data->Ythumbnail = get_unsigned_byte(fp);
  thumb_data_len = 3 * app_data->Xthumbnail * app_data->Ythumbnail;
  if (thumb_data_len > 0) {
    app_data->thumbnail = NEW(thumb_data_len, unsigned char);
    fread(app_data->thumbnail, 1, thumb_data_len, fp);
  } else {
    app_data->thumbnail = NULL;
  }

  add_APPn_marker(j_info, JM_APP0, JS_APPn_JFIF, app_data);

  switch (app_data->units) {
  case 1:
    j_info->xdpi = app_data->Xdensity;
    j_info->ydpi = app_data->Ydensity;
    break;
  case 2: /* density is in pixels per cm */
    j_info->xdpi = app_data->Xdensity * 2.54;
    j_info->ydpi = app_data->Ydensity * 2.54;
    break;
  default: /* FIXME: not sure what to do with this.... */
    j_info->xdpi = 72.0;
    j_info->ydpi = 72.0;
    break;
  }

  return (9 + thumb_data_len);
}

static unsigned short
read_APP0_JFXX (FILE *fp, unsigned short length)
{
  (void)get_unsigned_byte(fp);
  /* Extension Code:
   *
   * 0x10: Thumbnail coded using JPEG
   * 0x11: Thumbnail stored using 1 byte/pixel
   * 0x13: Thumbnail stored using 3 bytes/pixel
   */
  seek_relative(fp, length-1); /* Thunbnail image */

  /* Ignore */

  return length;
}

static unsigned short
read_APP2_ICC (struct JPEG_info *j_info, FILE *fp, unsigned short length)
{
  struct JPEG_APPn_ICC *app_data;

  app_data = NEW(1, struct JPEG_APPn_ICC);
  app_data->seq_id      = get_unsigned_byte(fp); /* Starting at 1 */
  app_data->num_chunks  = get_unsigned_byte(fp);
  app_data->length = length - 2;
  app_data->chunk  = NEW(app_data->length, unsigned char);
  fread(app_data->chunk, 1, app_data->length, fp);

  add_APPn_marker(j_info, JM_APP2, JS_APPn_ICC, app_data);

  return length;
}

int
JPEG_scan_file (struct JPEG_info *j_info, FILE *fp)
{
  JPEG_marker marker;
  unsigned short length;
  int  found_SOFn, count;
  char app_sig[128];

  JPEG_info_init(j_info);

  rewind(fp);
  count      = 0;
  found_SOFn = 0;
  while (!found_SOFn &&
	 (marker = JPEG_get_marker(fp)) != (JPEG_marker) -1) {
    if (marker == JM_SOI  ||
	(marker >= JM_RST0 && marker <= JM_RST7)) {
      count++;
      continue;
    }
    length = get_unsigned_pair(fp) - 2;
    switch (marker) {
    case JM_SOF0:  case JM_SOF1:  case JM_SOF2:  case JM_SOF3:
    case JM_SOF5:  case JM_SOF6:  case JM_SOF7:  case JM_SOF9:
    case JM_SOF10: case JM_SOF11: case JM_SOF13: case JM_SOF14:
    case JM_SOF15:
      j_info->bits_per_component = get_unsigned_byte(fp);
      j_info->height = get_unsigned_pair(fp);
      j_info->width  = get_unsigned_pair(fp);
      j_info->num_components = get_unsigned_byte(fp);
      found_SOFn = 1;
      break;
    case JM_APP0:
      if (length > 5) {
	if (fread(app_sig, sizeof(char), 5, fp) != 5)
	  return -1;
	length -= 5;
	if (!memcmp(app_sig, "JFIF\000", 5)) {
	  j_info->flags |= HAVE_APPn_JFIF;
	  length -= read_APP0_JFIF(j_info, fp);
	} else if (!memcmp(app_sig, "JFXX", 5)) {
	  length -= read_APP0_JFXX(fp, length);
	}
      }
      seek_relative(fp, length);
      break;
    case JM_APP1:
      if (length > 5) {
	if (fread(app_sig, sizeof(char), 5, fp) != 5)
	  return -1;
	length -= 5;
	if (!memcmp(app_sig, "Exif\000", 5)) {
	  j_info->flags |= HAVE_APPn_Exif;
	  length -= read_APP1_Exif(j_info, fp, length);
	}
      }
      seek_relative(fp, length);
      break;
    case JM_APP2:
      if (length >= 14) {
	if (fread(app_sig, sizeof(char), 12, fp) != 12)
	  return -1;
	length -= 12;
	if (!memcmp(app_sig, "ICC_PROFILE\000", 12)) {
	  j_info->flags |= HAVE_APPn_ICC;
	  length -= read_APP2_ICC(j_info, fp, length);
	  if (count < MAX_COUNT) {
	    j_info->skipbits[count / 8] |= (1 << (7 - (count % 8)));
	  }
	}
      }
      seek_relative(fp, length);
      break;
    case JM_APP14:
      if (length > 5) {
	if (fread(app_sig, sizeof(char), 5, fp) != 5)
	  return -1;
	length -= 5;
	if (!memcmp(app_sig, "Adobe", 5)) {
	  j_info->flags |= HAVE_APPn_ADOBE;
	  length -= read_APP14_Adobe(j_info, fp);
	} else {
	  if (count < MAX_COUNT) {
	    j_info->skipbits[count/8] |= (1 << (7 - (count % 8)));
	  }
	}
      }
      seek_relative(fp, length);
      break;
    default:
      seek_relative(fp, length);
      if (marker >= JM_APP0 &&
	  marker <= JM_APP15) {
	if (count < MAX_COUNT) {
	  j_info->skipbits[count / 8] |= (1 << (7 - (count % 8)));
	}
      }
      break;
    }
    count++;
  }

  return (found_SOFn ? 0 : -1);
}
