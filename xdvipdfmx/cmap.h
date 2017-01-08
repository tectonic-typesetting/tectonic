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

#ifndef _CMAP_H_
#define _CMAP_H_

#include "cid.h"

#include <sys/types.h>

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

extern void CMap_set_verbose (void);
extern void CMap_set_silent  (int value);

extern CMap  *CMap_new     (void);
extern void   CMap_release (CMap *cmap);

extern int    CMap_is_valid    (CMap *cmap);
extern int    CMap_is_Identity (CMap *cmap);
extern int    CMap_get_profile (CMap *cmap, int type);

extern char       *CMap_get_name (CMap *cmap);
extern int         CMap_get_type (CMap *cmap);
extern int         CMap_get_wmode(CMap *cmap);
extern CIDSysInfo *CMap_get_CIDSysInfo(CMap *cmap);

extern void   CMap_set_name   (CMap *cmap, const char *name);
extern void   CMap_set_type   (CMap *cmap, int type);
extern void   CMap_set_wmode  (CMap *cmap, int wmode);
extern void   CMap_set_usecmap(CMap *cmap, CMap *ucmap);
extern void   CMap_set_CIDSysInfo (CMap *cmap, const CIDSysInfo *csi);

/* charName not supported */
extern int   CMap_add_bfchar  (CMap *cmap,
			       const unsigned char *src, int srcdim,
			       const unsigned char *dest, int destdim);
extern int   CMap_add_cidchar (CMap *cmap,
			       const unsigned char *src, int srcdim, CID dest);
extern int   CMap_add_bfrange (CMap *cmap,
			       const unsigned char *srclo, const unsigned char *srchi, int srcdim,
			       const unsigned char *dest, int destdim);
extern int   CMap_add_cidrange(CMap *cmap,
			       const unsigned char *srclo, const unsigned char *hi, int srcdim,
			       CID base);

extern int CMap_add_notdefchar  (CMap *cmap, const unsigned char *src, int srcdim, CID dst);
extern int CMap_add_notdefrange (CMap *cmap,
				 const unsigned char *srclo, const unsigned char *srchi, int srcdim,
				 CID dst);

extern int  CMap_add_codespacerange (CMap *cmap,
				     const unsigned char *codelo, const unsigned char *codehi, int dim);

extern void CMap_decode_char (CMap *cmap,
			      const unsigned char **inbuf, int *inbytesleft,
			      unsigned char **outbuf, int *outbytesleft);

extern int  CMap_decode (CMap *cmap,
			 const unsigned char **inbuf,  int *inbytesleft,
			 unsigned char **outbuf, int *outbytesleft);

extern int  CMap_reverse_decode(CMap *cmap, CID cid);

extern void  CMap_cache_init  (void);
extern CMap *CMap_cache_get   (int id);
extern int   CMap_cache_find  (const char *cmap_name);
extern void  CMap_cache_close (void);
extern int   CMap_cache_add   (CMap *cmap);

#endif /* _CMAP_H_ */
