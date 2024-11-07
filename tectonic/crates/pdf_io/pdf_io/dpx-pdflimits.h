/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _PDFLIMITS_H_
#define _PDFLIMITS_H_

#include "tectonic_bridge_core.h"

/*
 * The minimal and maximal PDF version supported by DVIPDFMx.
 * NOTE: Don't forget to update CIDFont_stdcc_def[] in cid.c
 * if you increase PDF_VERSION_MAX!
 */
#define PDF_VERSION_MIN  13
#define PDF_VERSION_MAX  20
#define PDF_VERSION_DEFAULT 15

/*
 * PDF_NAME_LEN_MAX: see, Appendix C of PDF Ref. v1.3, 2nd. ed.
 * This is Acrobat implementation limit.
 */
#define PDF_NAME_LEN_MAX 127
#define PDF_NUM_INDIRECT_MAX 8388607ul

#endif /* _PDFLIMITS_H_ */
