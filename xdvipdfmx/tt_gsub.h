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

#ifndef _TT_GSUB_H_
#define _TT_GSUB_H_

#include "sfnt.h"
#include "otl_opt.h"

extern void otl_gsub_set_verbose (void);

typedef struct otl_gsub otl_gsub;

/* LookupType for GSUB */
#define OTL_GSUB_TYPE_SINGLE    1
#define OTL_GSUB_TYPE_MULTIPLE  2
#define OTL_GSUB_TYPE_ALTERNATE 3
#define OTL_GSUB_TYPE_LIGATURE  4
#define OTL_GSUB_TYPE_CONTEXT   5
#define OTL_GSUB_TYPE_CCONTEXT  6
#define OTL_GSUB_TYPE_ESUBST    7

extern otl_gsub *otl_gsub_new     (void);
extern void      otl_gsub_release (otl_gsub *gsub_list);

extern int  otl_gsub_select    (otl_gsub *gsub_list,
                                const char *script,
                                const char *language,
                                const char *feature);
extern int  otl_gsub_add_feat  (otl_gsub *gsub_list,
                                const char *script,
                                const char *language,
                                const char *feature,
                                sfnt *sfont);
extern int  otl_gsub_apply     (otl_gsub *gsub_list, USHORT *gid);
extern int  otl_gsub_apply_alt (otl_gsub *gsub_list, USHORT alt_idx, USHORT *gid);
extern int  otl_gsub_apply_lig (otl_gsub *gsub_list,
                                USHORT *gid_in, USHORT num_gids,
                                USHORT *gid_out);

#if  0  
extern int  otl_gsub_dump      (otl_gsub *gsub_list,
                                const char *script,
                                const char *language,
                                const char *feature);
#endif

#endif /* _TT_GSUB_H_ */
