/* w2c/config.h: All .c files include this first.

Copyright 1995, 1996, 2006, 2007, 2009, 2010, 2012, 2014,
          2015 Karl Berry.

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2, or (at your option)
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, see <http://www.gnu.org/licenses/.  */

#ifndef WEB2C_CONFIG_H
#define WEB2C_CONFIG_H

/* Formerly c-auto.h */

#define _DARWIN_USE_64_BIT_INODE 1
#define HAVE_ACCESS 1
#define HAVE_ASSERT_H 1
#define HAVE_ATOI 1
#define HAVE__BOOL 1
#define HAVE_DECL_ISASCII 1
#define HAVE_DECL_STRNDUP 1
#define HAVE_DIRENT_H 1
#define HAVE_DLFCN_H 1
#define HAVE_ERRNO_H 1
#define HAVE_FLOAT_H 1
#define HAVE_FMAX 1
#define HAVE_FSEEKO 1
#define HAVE_FTIME 1
#define HAVE_GETCWD 1
#define HAVE_GETTIMEOFDAY 1
#define HAVE_GETWD 1
#define HAVE_INTTYPES_H 1
#define HAVE_IOSTREAM 1
#define HAVE_LANGINFO_H 1
#define HAVE_LIBFONTCONFIG 1
#define HAVE_LIMITS_H 1
#define HAVE_LOCALE_H 1
#define HAVE_LONG_DOUBLE 1
#define HAVE_MEMCMP 1
#define HAVE_MEMCPY 1
#define HAVE_MEMORY_H 1
#define HAVE_MKDTEMP 1
#define HAVE_MKSTEMP 1
#define HAVE_MKTEMP 1
#define HAVE_OBJECT_INITCMD_CONST_CHARP 1
#define HAVE_PUTENV 1
#define HAVE_PWD_H 1
#define HAVE_SETLOCALE 1
#define HAVE_STDBOOL_H 1
#define HAVE_STDINT_H 1
#define HAVE_STDLIB_H 1
#define HAVE_STRCHR 1
#define HAVE_STRERROR 1
#define HAVE_STRING_H 1
#define HAVE_STRINGS_H 1
#define HAVE_STRNDUP 1
#define HAVE_STRRCHR 1
#define HAVE_STRUCT_STAT_ST_MTIM 1
#define HAVE_SYS_PARAM_H 1
#define HAVE_SYS_STAT_H 1
#define HAVE_SYS_TIMEB_H 1
#define HAVE_SYS_TIME_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_SYS_WAIT_H 1
#define HAVE_TIME_H 1
#define HAVE_UINTPTR_T 1
#define HAVE_UNISTD_H 1
#define RETSIGTYPE void
#define SIZEOF_INT 4
#define SIZEOF_LONG 8
#define SIZEOF_OFF_T 8
#define SIZEOF_VOID_P 8
#define STDC_HEADERS 1
#define ZLIB_CONST 1

/* end c-auto.h */

#include <stdarg.h>

/* How to open a binary file.  */
#include <tidy_kpathutil.h>
#include <kpsezip/public.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef signed char schar;

/* The type `integer' must be a signed integer capable of holding at
   least the range of numbers (-2^31)..(2^31-1).  If your compiler goes
   to great lengths to make programs fail, you might have to change this
   definition.  If this changes, you may have to modify
   web2c/fixwrites.c, since it generates code to do integer output using
   "%ld", and casts all integral values to be printed to `long'.

   If you define your own INTEGER_TYPE, you have to define your own
   INTEGER_MAX and INTEGER_MIN, too. */
#if SIZEOF_LONG > 4
typedef int integer;
#define INTEGER_MAX INT_MAX
#define INTEGER_MIN INT_MIN
#else
typedef long integer;
#define INTEGER_MAX LONG_MAX
#define INTEGER_MIN LONG_MIN
#endif /* SIZEOF_LONG > 4 */

#if defined __GNUC__ && __GNUC__ >=3
#define WEB2C_NORETURN __attribute__((__noreturn__))
#else
#define WEB2C_NORETURN
#endif

#ifdef __cplusplus
}
#endif

#endif /* not WEB2C_CONFIG_H */
