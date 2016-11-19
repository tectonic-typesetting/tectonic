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

/*  $Header: /home/cvsroot/dvipdfmx/src/bmpimage.c,v 1.2 2004/07/27 12:08:46 hirata Exp $

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

/*
 * BMP SUPPORT:
 */

#if HAVE_CONFIG_H
#include <w2c/config.h>
#endif

#include "bmpimage.h"

#define DIB_FILE_HEADER_SIZE 14
#define DIB_CORE_HEADER_SIZE 12
#define DIB_INFO_HEADER_SIZE 40
#define DIB_INFO_HEADER_SIZE2 64
#define DIB_INFO_HEADER_SIZE4 108
#define DIB_INFO_HEADER_SIZE5 124

#define DIB_COMPRESS_NONE 0
#define DIB_COMPRESS_RLE8 1
#define DIB_COMPRESS_RLE4 2

#define DIB_HEADER_SIZE_MAX (DIB_FILE_HEADER_SIZE+DIB_INFO_HEADER_SIZE5)

static void
WARN(const char *fmt, ...)
{
	va_list argp;
	
	fprintf(stderr, "** WARNING ** ");
	va_start(argp, fmt);
	vfprintf(stderr, fmt, argp);
	va_end(argp);
	fprintf(stderr, "\n");
}

int
check_for_bmp (FILE *fp)
{
 unsigned char sigbytes[2];

  if (!fp)
    return 0;

  rewind(fp);
  if (fread(sigbytes, 1, sizeof(sigbytes), fp) != sizeof(sigbytes) ||
      sigbytes[0] != 'B' || sigbytes[1] != 'M')
    return 0;
  else
    return 1;
  
  return 0;
}

int
bmp_scan_file(struct bmp_info *info, FILE *fp)
{
  unsigned char  buf[DIB_HEADER_SIZE_MAX+4];
  unsigned char *p;
  long offset, hsize;
  long psize; /* Bytes per palette color: 3 for OS2, 4 for Win */
  unsigned short bit_count; /* Bits per pix */
  int  num_palette;
  unsigned long biXPelsPerMeter, biYPelsPerMeter;

  p = buf;

  rewind(fp);
  if (fread(buf, 1, DIB_FILE_HEADER_SIZE + 4, fp)
      != DIB_FILE_HEADER_SIZE + 4) {
    WARN("Could not read BMP file header...");
  }

  if (p[0] != 'B' || p[1] != 'M') {
    WARN("File not starting with \'B\' \'M\'... Not a BMP file?");
    return -1;
  }
  p += 2;

#define ULONG_LE(b)  ((b)[0] + ((b)[1] << 8) +\
		      ((b)[2] << 16) + ((b)[3] << 24))
#define USHORT_LE(b) ((b)[0] + ((b)[1] << 8))

  /* ignore fsize */ p += 4;
  if (ULONG_LE(p) != 0) {
    WARN("Not a BMP file???");
    return -1;
  }
  p += 4;
  offset = ULONG_LE(p); p += 4;

  /* info header */
  hsize  = ULONG_LE(p); p += 4;
  if (fread(p, sizeof(char), hsize - 4, fp) != hsize - 4) {
    WARN("Could not read BMP file header...");
    return -1;
  }
  if (hsize == DIB_CORE_HEADER_SIZE) {
    info->width  = USHORT_LE(p); p += 2;
    info->height = USHORT_LE(p); p += 2;
    info->xdpi = 72.0; /* assume 72 DPI */
    info->ydpi = 72.0; /* assume 72 DPI */
    if (USHORT_LE(p) != 1) {
      WARN("Unknown bcPlanes value in BMP COREHEADER.");
      return -1;
    }
    p += 2;
    bit_count   = USHORT_LE(p); p += 2;
    psize = 3;
  } else if (hsize == DIB_INFO_HEADER_SIZE ||
             hsize == DIB_INFO_HEADER_SIZE2 ||
             hsize == DIB_INFO_HEADER_SIZE4 ||
             hsize == DIB_INFO_HEADER_SIZE5) {
    info->width  = ULONG_LE(p);  p += 4;
    info->height = ULONG_LE(p);  p += 4;
    if (USHORT_LE(p) != 1) {
      WARN("Unknown biPlanes value in BMP INFOHEADER.");
      return -1;
    }
    p += 2;
    bit_count   = USHORT_LE(p); p += 2;
    /* ignore compression */ p += 4;
    /* ignore biSizeImage */ p += 4;
    biXPelsPerMeter = ULONG_LE(p); p += 4;
    biYPelsPerMeter = ULONG_LE(p); p += 4;
    info->xdpi = biXPelsPerMeter * 0.0254; /* convert pixels per meter to DPI */
    info->ydpi = biYPelsPerMeter * 0.0254;
    if (info->height < 0) {
      info->height = -info->height;
    }
    psize = 4;
  } else {
    fprintf (stderr, "Unknown BMP header type.\n");
    exit (1);
    return -1; /* never reaches here */
  }

  if (bit_count < 24) {
    if (bit_count != 1 &&
	bit_count != 4 && bit_count != 8) {
      WARN("Unsupported palette size: %ld", bit_count);
      return -1;
    }
    num_palette = (offset - hsize - DIB_FILE_HEADER_SIZE) / psize;
    info->bits_per_component = bit_count;
    info->num_components = 1;
  } else if (bit_count == 24) { /* full color */
    num_palette = 1; /* dummy */
    info->bits_per_component = 8;
    info->num_components = 3;
  } else {
    WARN("Unkown BMP bitCount: %ld", bit_count);
    return -1;
  }

  if (info->width == 0 || info->height == 0 || num_palette < 1) {
    WARN("Invalid BMP file: width=%ld, height=%ld, #palette=%d",
	 info->width, info->height, num_palette);
    return -1;
  }

  return 0;
}

