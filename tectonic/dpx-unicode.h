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

#ifndef _UNICODE_H_
#define _UNICODE_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

bool UC_is_valid      (int32_t ucv);
bool UC_UTF16BE_is_valid_string (const unsigned char *p, const unsigned char *endptr);
bool UC_UTF8_is_valid_string (const unsigned char *p, const unsigned char *endptr);

size_t  UC_UTF16BE_encode_char (int32_t ucv, unsigned char **dstpp, unsigned char *endptr);
int32_t UC_UTF16BE_decode_char (const unsigned char **pp, const unsigned char *endptr);
int32_t UC_UTF8_decode_char (const unsigned char **pp, const unsigned char *endptr);
size_t  UC_UTF8_encode_char (int32_t ucv, unsigned char **dstpp, unsigned char *endptr);

#endif /* _UNICODE_H_ */
