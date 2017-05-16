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

#ifndef _MFILEIO_H_
#define _MFILEIO_H_

#include <stdio.h>
#include <tectonic/dpx-numbers.h>

extern void seek_relative (FILE *file, int32_t pos);
extern int32_t file_size (FILE *file);

#define xseek_absolute(file, pos, name) xfseeko (file, (off_t)(pos), SEEK_SET, name)
#define xseek_relative(file, pos, name) xfseeko (file, (off_t)(pos), SEEK_CUR, name)
#define xseek_end(file, name) xfseeko (file, (off_t)0, SEEK_END, name)
#define xtell_position(file, name) xftello (file, name)

extern char *mfgets (char *buffer, int length, FILE *file);

extern char work_buffer[];

#define WORK_BUFFER_SIZE 1024

/* Tectonic-enabled versions */

extern char *tt_mfgets (char *buffer, int length, rust_input_handle_t file);

#endif /* _MFILEIO_H_ */
