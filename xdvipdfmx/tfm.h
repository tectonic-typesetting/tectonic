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

#ifndef _TFM_H_
#define _TFM_H_

#include "numbers.h"

extern void tfm_set_verbose (void);

extern int  tfm_open (const char * tex_name, int must_exist);
extern void tfm_close_all (void);

extern double tfm_get_width  (int font_id, int32_t ch);
#if 0
extern double tfm_get_height (int font_id, int32_t ch);
extern double tfm_get_depth  (int font_id, int32_t ch);
#endif

extern fixword tfm_get_fw_width  (int font_id, int32_t ch);
extern fixword tfm_get_fw_height (int font_id, int32_t ch);
extern fixword tfm_get_fw_depth  (int font_id, int32_t ch);

extern fixword tfm_string_width  (int font_id, const unsigned char *s, unsigned len);
#if 0
extern fixword tfm_string_depth  (int font_id, const unsigned char *s, unsigned len);
extern fixword tfm_string_height (int font_id, const unsigned char *s, unsigned len);
#endif

/* From TFM header */
extern double tfm_get_design_size  (int font_id);
#if 0
extern char  *tfm_get_codingscheme (int font_id);
extern int tfm_is_vert (int font_id);
#endif

extern int tfm_exists  (const char *tfm_name);

#endif /* _TFM_H_ */
