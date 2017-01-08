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

#ifndef _NUMBERS_H_
#define _NUMBERS_H_

#include <stdio.h>
#include <math.h>
#ifdef HAVE_INTTYPES_H
# include <inttypes.h>
#endif
#ifdef HAVE_STDINT_H
# include <stdint.h>
#endif

#ifndef PRId64
# ifdef _WIN32
#  define PRId64 "I64d"
# elif SIZEOF_LONG == 8
#  define PRId64 "ld"
# else
#  define PRId64 "lld"
# endif
#endif

/* When reading numbers from binary files 1, 2, or 3 bytes are
   interpreted as either signed or unsigned.

   Four bytes from DVI, PK, TFM, or VF files always yield a signed
   32-bit integer (int32_t), but some of them must not be negative.

   Four byte numbers from JPEG2000, OpenType, or TrueType files are
   mostly unsigned (uint32_t) and occasionally signed (int32_t).
*/

extern unsigned char get_unsigned_byte (FILE *);
extern void skip_bytes (unsigned int, FILE *);
extern signed char get_signed_byte (FILE *);
extern unsigned short get_unsigned_pair (FILE *);
extern unsigned short sget_unsigned_pair (unsigned char *);
extern signed short get_signed_pair (FILE *);
extern unsigned int get_unsigned_triple (FILE *);
extern signed int get_signed_triple (FILE *);
extern int32_t get_signed_quad (FILE *);
extern uint32_t get_unsigned_quad (FILE *);
extern int32_t get_unsigned_num (FILE *, unsigned char);
extern uint32_t get_positive_quad (FILE *, const char *, const char *);

typedef int32_t fixword;

extern int32_t sqxfw (int32_t sq, fixword fw);

#ifndef MAX
#  define MAX(a,b) ((a)>(b)?(a):(b))
#endif
#ifndef MIN
#  define MIN(a,b) ((a)<(b)?(a):(b))
#endif
#define ISODD(n) (((n)/2)*2!=(n))
#define ISEVEN(n) (((n)/2)*2==(n))

#ifndef M_PI
#  define M_PI (4.0*atan(1.0))
#endif

#define ROUND(n,acc) (floor(((double)n)/(acc)+0.5)*(acc)) 

#if defined(__STDC_VERSION__) && __STDC_VERSION__ >= 199901L
#  define __C99__
#endif

#ifndef __C99__
#  ifndef round
#  define round(v) (floor((v) + 0.5))
#  endif
#  ifndef trunc
#  define trunc(v) ((v) > 0.0 ? floor((v)) : ceil((v)))
#  endif
#endif
#define round_at(v,acc) (round(((double)(v))/(acc))*(acc))

#endif /* _NUMBERS_H_ */
