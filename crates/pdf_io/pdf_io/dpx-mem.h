/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _MEM_H_
#define _MEM_H_

#include "tectonic_bridge_core.h"

#include <stdint.h>
#include <stdlib.h>

void *new (uint32_t size);
void *renew (void *p, uint32_t size);

#define NEW(n,type)     (type *) new(((uint32_t)(n))*sizeof(type))
#define RENEW(p,n,type) (type *) renew(p,((uint32_t)(n))*sizeof(type))

/* 
 * mem.h is not suitable for the following, but it is the only common
 * header file for dpxcrypt.c, pdfencrypt.c, and pdffont.c, which use
 * the function rand().
 */
void init_genrand(unsigned long long s);
long genrand_int31(void);
#define srand(x) init_genrand((x))
#define rand()   genrand_int31()
#endif /* _MEM_H_ */
