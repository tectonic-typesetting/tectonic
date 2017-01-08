/* This is DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

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

#ifndef _DPXCONF_H_
#define _DPXCONF_H_

#ifdef  HAVE_LIBPAPER
#include <paper.h>
#else
struct paper {
  const char* name;
  double pswidth, psheight;
};

#ifndef DEFAULT_PAPER_NAME
#define DEFAULT_PAPER_NAME "a4"
#endif

extern const struct paper  paperspecs[];
extern const struct paper *paperinfo (const char *ppformat);

#define paperinit()
#define paperdone()
#define paperpswidth(p)    (((p) && (p)->name) ? p->pswidth  : 0.0)
#define paperpsheight(p)   (((p) && (p)->name) ? p->psheight : 0.0)
#define papername(p)       (((p) && (p)->name) ? p->name : NULL)
#define paperfirst()       &(paperspecs[0])
#define papernext(p)       ((((p)+1) && ((p)+1)->name) ? (p+1) : NULL)

#define defaultpapername() DEFAULT_PAPER_NAME
#define systempapername()  DEFAULT_PAPER_NAME

#define defaultpapersizefile() NULL
#define systempapersizefile()  NULL
#endif /* HAVE_LIBPAPER */

extern void dumppaperinfo (void);

#endif /* _DPXCONF_H_ */
