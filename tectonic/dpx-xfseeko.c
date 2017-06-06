/* xfseeko.c: fseeko with error checking.

   Copyright 2008 Karl Berry.

   This library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   This library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this library; if not, see <http://www.gnu.org/licenses/>.  */

/*#include <kpathsea/config.h>*/

#include <stdio.h>
#include <stdlib.h>
#include <tectonic/dpx-system.h>

void
xfseeko (FILE *f,  off_t offset,  int wherefrom,  const char *filename)
{
  if (fseeko (f, offset, wherefrom) < 0) {
      /*FATAL_PERROR(filename);*/
      fprintf(stderr, "fseeko failed\n");
      exit(1);
  }
}
