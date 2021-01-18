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

#include "dpx-error.h"

#include <stdarg.h>
#include <stdio.h>

#include "tectonic_bridge_core.h"

typedef enum _message_type {
    DPX_MESG_INFO,
    DPX_MESG_WARN,
} message_type_t;

static message_type_t _last_message_type = DPX_MESG_INFO;
static int _dpx_quietness = 0;

void
shut_up (int quietness)
{
    _dpx_quietness = quietness;
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


PRINTF_FUNC(1, 0) static void
_dpx_print_to_stdout (const char *fmt, va_list argp, int warn)
{
    int n;

    n = vsnprintf(_dpx_message_buf, sizeof(_dpx_message_buf), fmt, argp);

    /* n is the number of bytes the vsnprintf() wanted to write -- it might be
     * bigger than sizeof(buf). */

    if (n >= sizeof(_dpx_message_buf)) {
        n = sizeof(_dpx_message_buf) - 1;
        _dpx_message_buf[n] = '\0';
    }

    if (warn)
        ttstub_issue_warning("%s", _dpx_message_buf);

    ttstub_output_write(_dpx_ensure_output_handle(), _dpx_message_buf, n);
}


void
dpx_message (const char *fmt, ...)
{
    va_list argp;

    if (_dpx_quietness > 0)
        return;

    va_start(argp, fmt);
    _dpx_print_to_stdout (fmt, argp, 0);
    va_end(argp);
    _last_message_type = DPX_MESG_INFO;
}

void
dpx_warning (const char *fmt, ...)
{
    va_list argp;

    if (_dpx_quietness > 1)
        return;

    if (_last_message_type == DPX_MESG_INFO)
        ttstub_output_write(_dpx_ensure_output_handle(), "\n", 1);

    ttstub_output_write(_dpx_ensure_output_handle(), "warning: ", 9);
    va_start(argp, fmt);
    _dpx_print_to_stdout (fmt, argp, 1);
    va_end(argp);
    ttstub_output_write(_dpx_ensure_output_handle(), "\n", 1);
    _last_message_type = DPX_MESG_WARN;
}
