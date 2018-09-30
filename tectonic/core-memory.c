/* tectonic/core-memory.c: basic C dynamic memory helpers

   Copyright 1993, 1994, 1995, 2008, 2009, 2010, 2011 Karl Berry.
   Copyright 1997, 2002, 2005 Olaf Weber.

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

#include "core-foundation.h"
#include "core-bridge.h" /* _tt_abort */
#include "core-memory.h"


void *
xcalloc (size_t nelem,  size_t elsize)
{
    void *new_mem = calloc(nelem ? nelem : 1, elsize ? elsize : 1);

    if (new_mem == NULL)
        _tt_abort ("xcalloc request for %lu elements of size %lu failed",
                   (unsigned long) nelem, (unsigned long) elsize);

    return new_mem;
}


void *
xmalloc (size_t size)
{
    void *new_mem = malloc(size ? size : 1);

    if (new_mem == NULL)
        _tt_abort ("xmalloc request for %lu bytes failed", (unsigned long) size);

    return new_mem;
}


void *
xrealloc (void *old_ptr, size_t size)
{
    void *new_mem;

    if (old_ptr == NULL) {
        new_mem = xmalloc(size);
    } else {
        new_mem = realloc(old_ptr, size ? size : 1);
        if (new_mem == NULL)
            _tt_abort("xrealloc() to %lu bytes failed", (unsigned long) size);
    }

    return new_mem;
}


char *
xstrdup (const char *s)
{
  char *new_string = xmalloc(strlen (s) + 1);
  return strcpy(new_string, s);
}
