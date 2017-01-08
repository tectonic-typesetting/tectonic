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

#include <stdio.h>
#include <stdlib.h>
#include "system.h"
#include "mfileio.h"
#include "error.h"

#ifdef IODEBUG 
static FILE *iodebug_file = NULL;
static int  event = 0;
static void io_debug_init(void)
{
  if (!iodebug_file) {
    iodebug_file = fopen ("fopen.log", "wb");
    fprintf (stderr, "\n*** File IO debugging started ***\n");
  }
  if (!iodebug_file) {
    fprintf (stderr, "\nError opening io log\n");
  }
}
#endif

#ifdef IODEBUG
FILE *mfopen(const char *name, const char *mode, const char *function, int line)
{
  FILE *tmp;
  io_debug_init();
#if defined(WIN32)
  tmp = fsyscp_fopen (name, mode);
#else
  tmp = fopen (name, mode);
#endif
  event += 1;
  fprintf(iodebug_file, "%p %07d [fopen] %s:%d\n", tmp, event,
	  function, line);
  return tmp;
}
int mfclose(FILE *file, const char *function, int line) 
{
  io_debug_init();
  event += 1;
  fprintf(iodebug_file, "%p %07d [fclose] %s:%d\n", file, event,
	  function, line);
  return fclose(file);
}
#endif

static void os_error(void)
{
  ERROR ("io:  An OS command failed that should not have.\n");
}

void seek_absolute (FILE *file, int32_t pos) 
{
  if (fseek(file, (long)pos, SEEK_SET)) {
    os_error();
  }
}

void seek_relative (FILE *file, int32_t pos)
{
  if (fseek(file, (long)pos, SEEK_CUR)) {
    os_error();
  }
}


void seek_end (FILE *file) 
{
  if (fseek(file, 0L, SEEK_END)) {
    os_error();
  }
}

int32_t tell_position (FILE *file) 
{
  long size = ftell (file);
  if (size < 0)
    os_error();
#if LONG_MAX > 0x7fffffff
  if (size > 0x7fffffff)
    ERROR ("ftell: file size %ld exceeds 0x7fffffff.\n", size);
#endif
  return size;
}

int32_t file_size (FILE *file)
{
  int32_t size;
  /* Seek to end */
  seek_end (file);
  size = tell_position (file);
  rewind (file);
  return size;
}

off_t xfile_size (FILE *file, const char *name)
{
  off_t size;
  xseek_end (file, name);
  size = xtell_position (file, name);
  rewind (file);
  return size;
}

/* Unlike fgets, mfgets works with \r, \n, or \r\n end of lines. */
char *mfgets (char *buffer, int length, FILE *file) 
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

/* As each lines may contain null-characters, so outptr here is NOT
 * null-terminated string.
 * Returns -1 for when EOF is already reached, and -2 if buffer has no
 * enough space.
 */
int
mfreadln (char *buf, int size, FILE *fp)
{
  int  c;
  int  len = 0;

  while ((c = fgetc(fp)) != EOF && c != '\n' && c != '\r') {
    if (len >= size) {
      return -2;
    }
    buf[len++] = (char) c;
  }
  if (c == EOF && len == 0) {
    return -1;
  }
  if (c == '\r' && (c = fgetc(fp)) >= 0 && (c != '\n'))
    ungetc(c, fp);

  return  len;
}

char work_buffer[WORK_BUFFER_SIZE];
