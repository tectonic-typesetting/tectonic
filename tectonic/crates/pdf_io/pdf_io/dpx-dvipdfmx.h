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

/* Tectonic: truncated version for the pdf_io crate */

#ifndef _DVIPDFMX_PDF_IO_INNER_H_
#define _DVIPDFMX_PDF_IO_INNER_H_

#include <time.h>

#define DVIPDFMX_PROG_NAME "xdvipdfmx"

extern time_t ttpi_source_date_epoch;
extern int landscape_mode;
extern double paper_height;
extern double paper_width;

#endif /* _DVIPDFMX_PDF_IO_INNER_H_ */
