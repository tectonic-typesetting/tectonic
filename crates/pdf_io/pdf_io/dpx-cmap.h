/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _CMAP_H_
#define _CMAP_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>
#include <stddef.h>
#include <sys/types.h>

#include "dpx-cid.h"
#include "dpx-cmap_p.h"

typedef unsigned short CID;
typedef unsigned short UCV16;

/* Limits */
#define CID_MAX_CID  65535
#define CID_MAX      CID_MAX_CID

#define CID_NOTDEF_CHAR   "\0\0"
#define CID_NOTDEF        0

#define UCS_NOTDEF_CHAR   "\377\375"
#define UCS_NOTDEF        0xfffd

/*
 * TYPE_IDENTITY and TYPE_CID_TO_CODE is not defined in the CMap spec.
 */
#define CMAP_TYPE_IDENTITY    0
#define CMAP_TYPE_CODE_TO_CID 1
#define CMAP_TYPE_TO_UNICODE  2
#define CMAP_TYPE_CID_TO_CODE 3

#define CMAP_PROF_TYPE_INBYTES_MIN  0
#define CMAP_PROF_TYPE_INBYTES_MAX  1
#define CMAP_PROF_TYPE_OUTBYTES_MIN 2
#define CMAP_PROF_TYPE_OUTBYTES_MAX 3

typedef struct CMap CMap;


/************************** CMAP_MAIN **************************/

void CMap_set_silent  (int value);

CMap  *CMap_new     (void);
void   CMap_release (CMap *cmap);

bool   CMap_is_valid    (CMap *cmap);
bool   CMap_is_Identity (CMap *cmap);
int    CMap_get_profile (CMap *cmap, int type);

char       *CMap_get_name (CMap *cmap);
int         CMap_get_type (CMap *cmap);
int         CMap_get_wmode(CMap *cmap);
CIDSysInfo *CMap_get_CIDSysInfo(CMap *cmap);

void   CMap_set_name   (CMap *cmap, const char *name);
void   CMap_set_type   (CMap *cmap, int type);
void   CMap_set_wmode  (CMap *cmap, int wmode);
void   CMap_set_usecmap(CMap *cmap, CMap *ucmap);
void   CMap_set_CIDSysInfo (CMap *cmap, const CIDSysInfo *csi);

/* charName not supported */
int   CMap_add_bfchar  (CMap *cmap,
                               const unsigned char *src, size_t srcdim,
                               const unsigned char *dest, size_t destdim);
int   CMap_add_cidchar (CMap *cmap,
                               const unsigned char *src, size_t srcdim, CID dest);
int   CMap_add_bfrange (CMap *cmap,
                               const unsigned char *srclo, const unsigned char *srchi, size_t srcdim,
                               const unsigned char *dest, size_t destdim);
int   CMap_add_cidrange(CMap *cmap,
                               const unsigned char *srclo, const unsigned char *hi, size_t srcdim,
                               CID base);

int CMap_add_notdefchar  (CMap *cmap, const unsigned char *src, size_t srcdim, CID dst);
int CMap_add_notdefrange (CMap *cmap,
                                 const unsigned char *srclo, const unsigned char *srchi, size_t srcdim,
                                 CID dst);

int  CMap_add_codespacerange (CMap *cmap,
                                     const unsigned char *codelo, const unsigned char *codehi, size_t dim);

void CMap_decode_char (CMap *cmap,
                              const unsigned char **inbuf, size_t *inbytesleft,
                              unsigned char **outbuf, size_t *outbytesleft);

size_t  CMap_decode (CMap *cmap,
                         const unsigned char **inbuf,  size_t *inbytesleft,
                         unsigned char **outbuf, size_t *outbytesleft);

void  CMap_cache_init  (void);
CMap *CMap_cache_get   (int id);
int   CMap_cache_find  (const char *cmap_name);
void  CMap_cache_close (void);
int   CMap_cache_add   (CMap *cmap);

#endif /* _CMAP_H_ */
