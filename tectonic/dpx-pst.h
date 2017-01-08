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

#ifndef _PST_H_
#define _PST_H_

#include <stdio.h>

#define PST_TYPE_UNKNOWN   -1
#define PST_TYPE_NULL       0
#define PST_TYPE_BOOLEAN    1
#define PST_TYPE_INTEGER    2
#define PST_TYPE_REAL       3
#define PST_TYPE_STRING     5
#define PST_TYPE_NAME       6
#define PST_TYPE_MARK       7

typedef struct pst_obj pst_obj;
typedef int            pst_type;

extern pst_obj *pst_get_token (unsigned char **inbuf, unsigned char *inbufend);

extern pst_obj *pst_new_obj    (pst_type type, void *data);
extern void     pst_release_obj(pst_obj *obj);
extern pst_obj *pst_new_mark   (void);

extern pst_type pst_type_of   (pst_obj *obj);
extern int      pst_length_of (pst_obj *obj);

extern int      pst_getIV (pst_obj *obj);
extern double   pst_getRV (pst_obj *obj);
extern unsigned char  *pst_getSV (pst_obj *obj);

extern void    *pst_data_ptr (pst_obj *obj);

#define PST_NAME_LEN_MAX   127
#define PST_STRING_LEN_MAX 4096
#define PST_MAX_DIGITS     10
#define PST_TOKEN_LEN_MAX  PST_STRING_LEN_MAX

#define PST_NULLTYPE(o)    (pst_type_of((o)) == PST_TYPE_NULL)
#define PST_BOOLEANTYPE(o) (pst_type_of((o)) == PST_TYPE_BOOLEAN)
#define PST_NAMETYPE(o)    (pst_type_of((o)) == PST_TYPE_NAME)
#define PST_STRINGTYPE(o)  (pst_type_of((o)) == PST_TYPE_STRING)
#define PST_INTEGERTYPE(o) (pst_type_of((o)) == PST_TYPE_INTEGER)
#define PST_REALTYPE(o)    (pst_type_of((o)) == PST_TYPE_REAL)
#define PST_NUMBERTYPE(o)  (PST_INTEGERTYPE((o))||PST_REALTYPE((o)))
#define PST_MARKTYPE(o)    (pst_type_of((o)) == PST_TYPE_MARK)
#define PST_UNKNOWNTYPE(o) (pst_type_of((o)) < 0)

#define PST_TOKEN_END(s,e) ((s) == (e) || is_delim(*(s)) || is_space(*(s)))

#endif /* _PST_H_ */
