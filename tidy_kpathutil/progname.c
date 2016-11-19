/* progname.c: the executable name we were invoked as; general initialization.

   Copyright 1994, 1996, 1997, 2008-2013, 2016 Karl Berry.
   Copyright 1998-2005 Olaf Weber.

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

#include <tidy_kpathutil/public.h>

string
kpse_program_basename (const_string argv0)
{
  string base = xstrdup (xbasename (argv0));
#ifdef EXEEXT
  string dot = strrchr (base, '.');
  if (dot && FILESTRCASEEQ (dot, EXEEXT))
    *dot = 0;
#endif
  return base;
}
