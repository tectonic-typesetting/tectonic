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

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#include <tectonic/dpx-dvipdfmx.h>
#include <tectonic/dpx-error.h>
#include <tectonic/internals.h>

typedef enum _message_type {
    DPX_MESG_INFO,
    DPX_MESG_WARN,
    DPX_MESG_ERROR,
} message_type_t;

static message_type_t _last_message_type = DPX_MESG_INFO;
static int really_quiet = 0;

void
shut_up (int quietness)
{
    really_quiet = quietness;
}


static rust_output_handle_t _dpx_message_handle = NULL;
static char _dpx_message_buf[1024];

static rust_output_handle_t
_dpx_ensure_output_handle (void)
{
    _dpx_message_handle = ttstub_output_open_stdout();

    if (_dpx_message_handle == NULL)
	_tt_abort("xdvipdfmx cannot get output logging handle?!");

    return _dpx_message_handle;
}


void
MESG (const char *fmt, ...)
{
    va_list argp;
    int n;

    if (really_quiet > 0)
	return;

    va_start(argp, fmt);
    n = vsnprintf(_dpx_message_buf, sizeof(_dpx_message_buf), fmt, argp);
    va_end(argp);

    /* n is the number of bytes the vsnprintf() wanted to write -- it might be
     * bigger than sizeof(buf). */

    if (n >= sizeof(_dpx_message_buf)) {
	n = sizeof(_dpx_message_buf) - 1;
	_dpx_message_buf[n] = '\0';
    }

    ttstub_output_write(_dpx_ensure_output_handle(), _dpx_message_buf, n);
    _last_message_type = DPX_MESG_INFO;
}

void
WARN (const char *fmt, ...)
{
    va_list argp;

    if (really_quiet < 2) {
        if (_last_message_type == DPX_MESG_INFO)
            fprintf(stderr, "\n");
        fprintf(stderr, "%s:warning: ", my_name);
        va_start(argp, fmt);
        vfprintf(stderr, fmt, argp);
        va_end(argp);
        fprintf(stderr, "\n");

        _last_message_type = DPX_MESG_WARN;
    }
}

void
ERROR (const char *fmt, ...)
{
    va_list argp;

    if (really_quiet < 3) {
        if (_last_message_type == DPX_MESG_INFO)
            fprintf(stderr, "\n");
        fprintf(stderr, "%s:fatal: ", my_name);
        va_start(argp, fmt);
        vfprintf(stderr, fmt, argp);
        va_end(argp);
        fprintf(stderr, "\n");
    }

    error_cleanup();
    exit (1);
}
