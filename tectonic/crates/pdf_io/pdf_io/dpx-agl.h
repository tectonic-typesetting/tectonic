/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _AGL_H_
#define _AGL_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>
#include <stdint.h>

#define AGL_DEFAULT_LISTFILE "glyphlist.txt"
#define AGL_PREDEF_LISTFILE "pdfglyphlist.txt"
#define AGL_EXTRA_LISTFILE "texglyphlist.txt"

#define AGL_MAX_UNICODES 16
struct agl_name {
  char *name;
  char *suffix;
  int   n_components;
  int32_t  unicodes[AGL_MAX_UNICODES];
  struct agl_name *alternate;
  int   is_predef;
};
typedef struct agl_name agl_name;

char *agl_chop_suffix  (const char *glyphname, char **suffix);

int32_t agl_sput_UTF16BE (const char *name,
                               unsigned char **dstpp,
                               unsigned char *limptr, int *num_fails);

int   agl_get_unicodes (const char *glyphstr,
                               int32_t *unicodes, int max_uncodes);

bool     agl_name_is_unicode      (const char *glyphname);
int32_t  agl_name_convert_unicode (const char *glyphname);

const char *agl_suffix_to_otltag (const char *suffix);

agl_name   *agl_lookup_list     (const char *glyphname);

void  agl_init_map    (void);
void  agl_close_map   (void);

#endif /* _AGL_H_ */
