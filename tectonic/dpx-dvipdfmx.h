/*  DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
    the DVIPDFMx project team.

    Copyright (c) 2006 SIL. (xdvipdfmx extensions for XeTeX support)

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

#ifndef _DVIPDFMX_H_
#define _DVIPDFMX_H_

#include "tectonic_bridge_core.h"
#include "core-bindgen.h" /* XdvipdfmxConfig */

#include <stdbool.h>

#define DVIPDFMX_PROG_NAME "xdvipdfmx"

extern time_t source_date_epoch;
extern const XdvipdfmxConfig* dpx_config;

int extractbb(int argc, char *argv[]);
int dvipdfmx_main(
  const char *pdfname,
  const char *dviname,
  const char *pagespec,
  int opt_flags,
  bool translate,
  bool compress,
  bool deterministic_tags,
  bool quiet,
  unsigned int verbose,
  time_t build_date);

#endif /* _DVIPDFMX_H_ */
