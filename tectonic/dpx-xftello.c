/* xftello.c: ftello with error checking.

   Copyright 1992, 1993, 1995, 2008 Karl Berry.
   Copyright 2005 Olaf Weber.

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


off_t
xftello (FILE *f, const_string filename)
{
    off_t where = ftello (f);

    if (where < 0) {
        fprintf(stderr, "ftello failed\n");
        exit(1);
    }

    return where;
}
