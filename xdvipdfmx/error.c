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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <stdarg.h>
#include <stdio.h>

#include "dvipdfmx.h"
#include "error.h"

#define DPX_MESG        0
#define DPX_MESG_WARN   1
#define DPX_MESG_ERROR  2

static int _mesg_type = DPX_MESG;
#define WANT_NEWLINE() (_mesg_type != DPX_MESG_WARN && _mesg_type != DPX_MESG_ERROR)

static int  really_quiet = 0;

void
shut_up (int quietness)
{
  really_quiet = quietness;
}

void
MESG (const char *fmt, ...)
{
  va_list argp;

  if (really_quiet < 1) {
    va_start(argp, fmt);
    vfprintf(stderr, fmt, argp);
    va_end(argp);
    _mesg_type = DPX_MESG;
  }
}

void
WARN (const char *fmt, ...)
{
  va_list argp;

  if (really_quiet < 2) {
    if (WANT_NEWLINE())
      fprintf(stderr, "\n");
    fprintf(stderr, "%s:warning: ", my_name);
    va_start(argp, fmt);
    vfprintf(stderr, fmt, argp);
    va_end(argp);
    fprintf(stderr, "\n");

    _mesg_type = DPX_MESG_WARN;
  }
}

void
ERROR (const char *fmt, ...)
{
  va_list argp;

  if (really_quiet < 3) {
    if (WANT_NEWLINE())
      fprintf(stderr, "\n");
    fprintf(stderr, "%s:fatal: ", my_name);
    va_start(argp, fmt);
    vfprintf(stderr, fmt, argp);
    va_end(argp);
    fprintf(stderr, "\n");
  }
  error_cleanup();
  exit( 1 );
}
