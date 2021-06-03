/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 2012-2015 by Khaled Hosny <khaledhosny@eglug.org>

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

#ifndef _T1_LOAD_H_
#define _T1_LOAD_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>
#include <stdio.h>

#include "tectonic_bridge_core.h"
#include "dpx-cff.h"

cff_font *t1_load_font (char **enc_vec, int mode, rust_input_handle_t handle);
bool is_pfb (rust_input_handle_t handle);
int t1_get_fontname (rust_input_handle_t handle, char *fontname);
const char *t1_get_standard_glyph (int code);

#endif /* _T1_LOAD_H_ */
