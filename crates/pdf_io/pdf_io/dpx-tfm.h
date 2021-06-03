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

#ifndef _TFM_H_
#define _TFM_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>
#include <stdint.h>

#include "dpx-numbers.h"

void tfm_reset_global_state(void);

int  tfm_open (const char * tex_name, int must_exist);
void tfm_close_all (void);

double tfm_get_width  (int font_id, int32_t ch);

fixword tfm_get_fw_width  (int font_id, int32_t ch);
fixword tfm_get_fw_height (int font_id, int32_t ch);
fixword tfm_get_fw_depth  (int font_id, int32_t ch);

fixword tfm_string_width  (int font_id, const unsigned char *s, unsigned len);

/* From TFM header */
double tfm_get_design_size  (int font_id);

bool tfm_exists  (const char *tfm_name);

#endif /* _TFM_H_ */
