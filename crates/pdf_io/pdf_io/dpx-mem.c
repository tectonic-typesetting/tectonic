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

#include "dpx-mem.h"

#include <stdlib.h>

#include "tectonic_bridge_core.h"

void *new (uint32_t size)
{
  void *result = malloc ((size_t)size);
  if (!result) {
    _tt_abort("Out of memory - asked for %u bytes\n", size);
  }

  return result;
}

void *renew (void *mem, uint32_t size)
{
  if (size) {
    void *result = realloc (mem, (size_t)size);
    if (!result) {
      _tt_abort("Out of memory - asked for %u bytes\n", size);
    }
    return result;
  } else {
    /* realloc may not return NULL if size == 0 */
    free(mem);
    return NULL;
  }
}
