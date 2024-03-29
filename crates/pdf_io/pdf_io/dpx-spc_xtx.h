/*  This is xdvipdfmx, an extended version of dvipdfmx,
    an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2013-2016 by the dvipdfmx project team.

    Copyright (c) 2006 SIL International
    Originally written by Jonathan Kew

    Copyright (C) 2002 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _SPC_XTX_H_
#define _SPC_XTX_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>

#include "dpx-specials.h"

bool spc_xtx_check_special (const char *buffer, int size);
int  spc_xtx_setup_handler (struct spc_handler *handle,
                                    struct spc_env *spe, struct spc_arg *args);
int spc_handler_xtx_do_transform (struct spc_env *spe, double x_user, double y_user, double a, double b, double c, double d, double e, double f);
int spc_handler_xtx_gsave (struct spc_env *spe, struct spc_arg *args);
int spc_handler_xtx_grestore (struct spc_env *spe, struct spc_arg *args);

#endif /* _SPC_XTX_H_ */
