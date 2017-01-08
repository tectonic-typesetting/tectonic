/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.
    
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

#ifndef _CFF_LIMITS_H_
#define _CFF_LIMITS_H_

#include <limits.h>

#define CFF_INT_MAX 0x7fffffff
#define CFF_INT_MIN (-0x7fffffff - 1)

#if (INT_MAX < CFF_INT_MAX || INT_MIN > CFF_INT_MIN)
#error "CFF support won't work on this system."
#endif

#define CFF_SID_MAX    64999
#define CFF_STDSTR_MAX 391

/* Type 2 Charstring */
#define CS_NUM_SUBR_MAX    65536
#define CS_STR_LEN_MAX     65536
#define CS_STEM_ZONE_MAX   96
#define CS_ARG_STACK_MAX   48
#define CS_TRANS_ARRAY_MAX 32
#define CS_SUBR_NEST_MAX   10

#endif /* _CFF_LIMITS_H_ */
