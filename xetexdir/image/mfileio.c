/*  $Header: /home/cvsroot/dvipdfmx/src/mfileio.c,v 1.3 2002/10/30 02:27:11 chofchof Exp $

    This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2015 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team <dvipdfmx@project.ktug.or.kr>
    
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

#if HAVE_CONFIG_H
#include <w2c/config.h>
#endif

#include "mfileio.h"

#ifdef IODEBUG 
static FILE *iodebug_file = NULL;
static long event = 0;
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
  tmp = fopen (name, mode);
  event += 1;
  fprintf(iodebug_file, "%p %07ld [fopen] %s:%d\n", tmp, event,
	  function, line);
  return tmp;
}
int mfclose(FILE *file, const char *function, int line) 
{
  io_debug_init();
  event += 1;
  fprintf(iodebug_file, "%p %07ld [fclose] %s:%d\n", file, event,
	  function, line);
  return fclose(file);
}
#endif

static void os_error(void)
{
  fprintf (stderr, "io:  An OS command failed that should not have.\n");
  exit(-1);
}

void seek_absolute (FILE *file, long pos) 
{
  if (fseek(file, pos, SEEK_SET)) {
    os_error();
  }
}

void seek_relative (FILE *file, long pos)
{
  if (fseek(file, pos, SEEK_CUR)) {
    os_error();
  }
}


void seek_end (FILE *file) 
{
  if (fseek(file, 0L, SEEK_END)) {
    os_error();
  }
}

long tell_position (FILE *file) 
{
  long size;
  if ((size = ftell (file)) < 0) {
    os_error();
  }
  return size;
}

long file_size (FILE *file)
{
  long size;
  /* Seek to end */
  seek_end (file);
  size = tell_position (file);
  rewind (file);
  return (size);
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

char work_buffer[WORK_BUFFER_SIZE];
