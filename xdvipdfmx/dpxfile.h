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

#ifndef _DPXFILE_H_
#define _DPXFILE_H_

#define DPX_CONFIG_FILE "dvipdfmx.cfg"

typedef enum {
  DPX_RES_TYPE_FONTMAP = 0,

  DPX_RES_TYPE_T1FONT,
  DPX_RES_TYPE_TTFONT,
  DPX_RES_TYPE_OTFONT,
  DPX_RES_TYPE_PKFONT,
  DPX_RES_TYPE_DFONT,

  DPX_RES_TYPE_ENC,
  DPX_RES_TYPE_CMAP,
  DPX_RES_TYPE_SFD,
  DPX_RES_TYPE_AGL,

  DPX_RES_TYPE_ICCPROFILE,

  DPX_RES_TYPE_BINARY,
  DPX_RES_TYPE_TEXT
} dpx_res_type;

#include "mfileio.h"
extern FILE *dpx_open_file (const char *filename, dpx_res_type type);

extern char * dpx_find_type1_file (const char *filename);
extern char * dpx_find_truetype_file (const char *filename);
extern char * dpx_find_opentype_file (const char *filename);
extern char * dpx_find_dfont_file (const char *filename);

#define DPXFOPEN(n,t)  dpx_open_file((const char *)(n),(t))
#define DPXFCLOSE(f)   MFCLOSE((f))

extern void  dpx_file_set_verbose  (void);

extern int   dpx_file_apply_filter (const char *cmdtmpl,
                                   const char *input, const char *output,
                                   unsigned char version);
extern char *dpx_create_temp_file  (void);
extern char *dpx_create_fix_temp_file (const char *filename);
extern void  dpx_delete_old_cache  (int life);
extern void  dpx_delete_temp_file  (char *tmp, int force); /* tmp freed here */

extern int   keep_cache;
#endif /* _DPXFILE_H_ */
