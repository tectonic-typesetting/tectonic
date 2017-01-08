/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
	
#ifndef _SPECIALS_H_
#define _SPECIALS_H_

struct spc_env {
  double x_user, y_user;
  double mag;
  int    pg;  /* current page in PDF */
};

struct spc_arg {
  const char  *curptr;
  const char  *endptr;
  const char  *base;

  const char  *command;
};

typedef int  (*spc_handler_fn_ptr) (struct spc_env *, struct spc_arg *);

struct spc_handler {
  const char         *key;
  spc_handler_fn_ptr  exec;
};

/* This should not use pdf_. */
extern void    spc_set_verbose (void);

#include <stdarg.h>
extern void    spc_warn (struct spc_env *spe, const char *fmt, ...);

#include "pdfobj.h"
/* PDF parser shouldn't depend on this...
 */
extern pdf_obj *spc_lookup_reference (const char *ident);
extern pdf_obj *spc_lookup_object    (const char *ident);

extern int      spc_begin_annot   (struct spc_env *spe, pdf_obj *annot_dict);
extern int      spc_end_annot     (struct spc_env *spe);
extern int      spc_resume_annot  (struct spc_env *spe);
extern int      spc_suspend_annot (struct spc_env *spe);

extern void     spc_push_object   (const char *key, pdf_obj *value);
extern void     spc_flush_object  (const char *key);
extern void     spc_clear_objects (void);

extern int      spc_exec_at_begin_page     (void);
extern int      spc_exec_at_end_page       (void);
extern int      spc_exec_at_begin_document (void);
extern int      spc_exec_at_end_document   (void);

extern int      spc_exec_special (const char *p, int32_t size,
				  double x_user, double y_user, double mag);

#endif /* _SPECIALS_H_ */
