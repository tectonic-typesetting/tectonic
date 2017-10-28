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

#include "dpx-mfileio.h"

#include <fcntl.h>
#include <limits.h>
#include <stdio.h>


static void
os_error(void)
{
    _tt_abort("io:  An OS command failed that should not have.\n");
}


void
seek_relative (FILE *file, int32_t pos)
{
    if (fseek(file, (long) pos, SEEK_CUR))
        os_error();
}


static void
seek_end (FILE *file)
{
    if (fseek(file, 0L, SEEK_END))
        os_error();
}


static int32_t
tell_position (FILE *file)
{
    long size = ftell (file);
    if (size < 0)
        os_error();
#if LONG_MAX > 0x7fffffff
    if (size > 0x7fffffff)
        _tt_abort("ftell: file size %ld exceeds 0x7fffffff.\n", size);
#endif
    return size;
}


int32_t
file_size (FILE *file)
{
    int32_t size;

    seek_end (file);
    size = tell_position (file);
    rewind (file);
    return size;
}


/* Unlike fgets, mfgets works with \r, \n, or \r\n end of lines. */
char *
mfgets (char *buffer, int length, FILE *file)
{
    int ch = 0, i = 0;

    while (i < length-1 && (ch = fgetc (file)) >= 0 && ch != '\n' && ch != '\r')
        buffer[i++] = ch;

    buffer[i] = 0;

    if (ch < 0 && i == 0)
        return NULL;

    if (ch == '\r' && (ch = fgetc (file)) >= 0 && (ch != '\n'))
        ungetc (ch, file);

    return buffer;
}


/* Note: this is really just a random array used in other files. */
char work_buffer[WORK_BUFFER_SIZE];


/* Modified versions of the above functions based on the Tectonic I/O system. */

char *
tt_mfgets (char *buffer, int length, rust_input_handle_t file)
{
    int ch = 0, i = 0;

    while (i < length - 1 && (ch = ttstub_input_getc (file)) >= 0 && ch != '\n' && ch != '\r')
        buffer[i++] = ch;

    buffer[i] = '\0';

    if (ch < 0 && i == 0)
        return NULL;

    if (ch == '\r' && (ch = ttstub_input_getc (file)) >= 0 && (ch != '\n'))
        ttstub_input_ungetc (file, ch);

    return buffer;
}
