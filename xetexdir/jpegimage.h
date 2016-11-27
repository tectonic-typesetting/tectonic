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

/*  $Header: /home/cvsroot/dvipdfmx/src/jpegimage.h,v 1.2 2004/03/11 11:50:21 hirata Exp $

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

#ifndef _JPEGIMAGE_H_
#define _JPEGIMAGE_H_

#include "mfileio.h"

/* JPEG Markers */
typedef enum {
  JM_SOF0  = 0xc0,
  JM_SOF1  = 0xc1,
  JM_SOF2  = 0xc2,
  JM_SOF3  = 0xc3,
  JM_SOF5  = 0xc5,
  JM_DHT   = 0xc4,
  JM_SOF6  = 0xc6,
  JM_SOF7  = 0xc7,
  JM_SOF9  = 0xc9,
  JM_SOF10 = 0xca,
  JM_SOF11 = 0xcb,
  JM_DAC   = 0xcc,
  JM_SOF13 = 0xcd,
  JM_SOF14 = 0xce,
  JM_SOF15 = 0xcf,

  JM_RST0  = 0xd0,
  JM_RST1  = 0xd1,
  JM_RST2  = 0xd2,
  JM_RST3  = 0xd3,
  JM_RST4  = 0xd4,
  JM_RST5  = 0xd5,
  JM_RST6  = 0xd6,
  JM_RST7  = 0xd7,

  JM_SOI   = 0xd8,
  JM_EOI   = 0xd9,
  JM_SOS   = 0xda,
  JM_DQT   = 0xdb,
  JM_DNL   = 0xdc,
  JM_DRI   = 0xdd,
  JM_DHP   = 0xde,
  JM_EXP   = 0xdf,

  JM_APP0  = 0xe0,
  JM_APP1  = 0xe1,
  JM_APP2  = 0xe2,
  JM_APP14 = 0xee,
  JM_APP15 = 0xef,

  JM_COM   = 0xfe
} JPEG_marker;

typedef enum {
  JS_APPn_JFIF,
  JS_APPn_ADOBE,
  JS_APPn_ICC
} JPEG_APPn_sig;

struct JPEG_APPn_JFIF  /* APP0 */
{
  unsigned short version;
  unsigned char  units;      /* 0: only aspect ratio
			      * 1: dots per inch
			      * 2: dots per cm
			      */
  unsigned short Xdensity;
  unsigned short Ydensity;
  unsigned char  Xthumbnail;
  unsigned char  Ythumbnail;
  unsigned char *thumbnail;  /* Thumbnail data. */
};

struct JPEG_APPn_ICC   /* APP2 */
{
  unsigned char  seq_id;
  unsigned char  num_chunks;
  unsigned char *chunk;

  /* Length of ICC profile data in this chunk. */
  unsigned short length;
};

struct JPEG_APPn_Adobe /* APP14 */
{
  unsigned short version;
  unsigned short flag0;
  unsigned short flag1;
  unsigned char  transform; /* color transform code */
};

struct JPEG_ext
{
  JPEG_marker   marker;
  JPEG_APPn_sig app_sig;
  void         *app_data;
};

#define MAX_COUNT 1024
struct  JPEG_info
{
  unsigned short height;
  unsigned short width;

  unsigned char  bits_per_component;
  unsigned char  num_components;

  double xdpi;
  double ydpi;

  /* Application specific extensions */
  int flags;
  int num_appn, max_appn;
  struct JPEG_ext *appn;

  /* Skip chunks not necessary. */
  char skipbits[MAX_COUNT / 8 + 1];
};

extern int check_for_jpeg(FILE *fp);
extern int JPEG_scan_file(struct JPEG_info *info, FILE *fp);

#endif /* _JPEGIMAGE_H_ */
