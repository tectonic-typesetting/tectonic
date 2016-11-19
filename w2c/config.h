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

/* The stuff from the path searching library.  */
#include <w2c/c-auto.h>
#include <stdarg.h>

/* How to open a binary file.  */
#include <tidy_kpathutil/public.h>
#include <tidy_kpathsea/public.h>

#ifdef __cplusplus
extern "C" {
#endif

/* The smallest signed type: use `signed char' if ANSI C, `short' if
   char is unsigned, otherwise `char'.  */
#ifndef SCHAR_TYPE
#if __STDC__
#define SCHAR_TYPE signed char
#else /* not __STDC */
#ifdef __CHAR_UNSIGNED__
#define SCHAR_TYPE short
#else
#define SCHAR_TYPE char
#endif
#endif /* not __STDC__ */
#endif /* not SCHAR_TYPE */
typedef SCHAR_TYPE schar;

/* The type `integer' must be a signed integer capable of holding at
   least the range of numbers (-2^31)..(2^31-1).  If your compiler goes
   to great lengths to make programs fail, you might have to change this
   definition.  If this changes, you may have to modify
   web2c/fixwrites.c, since it generates code to do integer output using
   "%ld", and casts all integral values to be printed to `long'.
   
   If you define your own INTEGER_TYPE, you have to define your own
   INTEGER_MAX and INTEGER_MIN, too. */
#ifndef INTEGER_TYPE

#if SIZEOF_LONG > 4 && !defined (NO_DUMP_SHARE)
/* If we have 64-bit longs and want to share format files (with 32-bit
   machines), use `int'.  */
#define INTEGER_IS_INT
#endif

#ifdef INTEGER_IS_INT
#define INTEGER_TYPE int
#define INTEGER_MAX INT_MAX
#define INTEGER_MIN INT_MIN
#else
#define INTEGER_TYPE long
#define INTEGER_MAX LONG_MAX
#define INTEGER_MIN LONG_MIN
#endif /* not INTEGER_IS_INT */

#endif /* not INTEGER_TYPE */

typedef INTEGER_TYPE integer;

/* We need a type that's at least off_t wide */
typedef off_t longinteger;

/* To print file offsets we cast them to `LONGINTEGER_TYPE' (or
   `unsigned LONGINTEGER_TYPE') and use the conversion specifier
   `"%" LONGINTEGER_PRI "d"' (or `"%" LONGINTEGER_PRI "u"').  */
#if defined(WIN32)
#define LONGINTEGER_TYPE __int64
#define LONGINTEGER_PRI "I64"
#elif SIZEOF_LONG < SIZEOF_OFF_T
#define LONGINTEGER_TYPE long long
#define LONGINTEGER_PRI "ll"
#else
#define LONGINTEGER_TYPE long
#define LONGINTEGER_PRI "l"
#endif

/* We also need a genuine 64-bit integer type.  */
#if defined(WIN32)
typedef __int64 integer64;
#else
typedef int64_t integer64;
#endif

/* And we need uintptr_t.  */
#ifndef HAVE_UINTPTR_T
# if SIZEOF_VOID_P == SIZEOF_INT
typedef unsigned int uintptr_t;
# elif SIZEOF_VOID_P == SIZEOF_LONG
typedef unsigned long uintptr_t;
# endif
#endif

/* I don't want to write a configure test for remove when all Unix
   machines have unlink.  But, for the sake of non-Unix machines that
   support ANSI C... */
#if !defined (unix) && !defined (__unix__) && defined (__STDC__) && !defined (unlink)
#define unlink remove
#endif

/* Window support on the Amiga is just for the Amiga.  */
#ifdef AMIGA
#define AMIGAWIN
#endif

/* Window support for WIN32 machines. */
#ifdef WIN32
#define WIN32WIN
#endif

#if defined __GNUC__ && __GNUC__ >=3
#define WEB2C_NORETURN __attribute__((__noreturn__))
#else
#define WEB2C_NORETURN
#endif

/* From uexit.c.  This is here because the lib/ and web2c/ routines
   themselves can use it, but they don't need cpascal.h.  */
WEB2C_NORETURN
extern void uexit (int status);

/* usage.c */
extern void usage (const_string progname);
extern void usagehelp (const_string *message, const_string bug_email);

#ifdef __cplusplus
}
#endif

#endif /* not WEB2C_CONFIG_H */
