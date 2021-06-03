/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include "tectonic_bridge_core.h"
#include "dpx-mfileio.h"

typedef enum {
  DPX_RES_TYPE_T1FONT,
  DPX_RES_TYPE_TTFONT,
  DPX_RES_TYPE_OTFONT,
  DPX_RES_TYPE_PKFONT,
  DPX_RES_TYPE_DFONT,

  DPX_RES_TYPE_SFD,

  DPX_RES_TYPE_ICCPROFILE,

  DPX_RES_TYPE_BINARY,
  DPX_RES_TYPE_TEXT
} dpx_res_type;

rust_input_handle_t dpx_open_type1_file (const char *filename);
rust_input_handle_t dpx_open_truetype_file (const char *filename);
rust_input_handle_t dpx_open_opentype_file (const char *filename);
rust_input_handle_t dpx_open_dfont_file (const char *filename);

int   dpx_file_apply_filter (const char *cmdtmpl,
                                   const char *input, const char *output,
                                   int version);
char *dpx_create_temp_file  (void);
void  dpx_delete_old_cache  (int life);
void  dpx_delete_temp_file  (char *tmp, int force); /* tmp freed here */

/* Tectonic-enabled I/O alternatives */

rust_input_handle_t dpx_tt_open (const char *filename, const char *suffix,
                                 ttbc_file_format format);

#endif /* _DPXFILE_H_ */
