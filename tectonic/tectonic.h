/* tectonic/tectonic.h: global, public header for Tectonic
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_TECTONIC_H
#define TECTONIC_TECTONIC_H

/* TODO: move these to an "internals.h" whenever possible. */

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

/* is this actually needed? */
#include <kpsezip/public.h>

/* Convenience for C++: this way Emacs doesn't try to indent the prototypes,
 * which I find annoying. */

#ifdef __cplusplus
#define BEGIN_EXTERN_C extern "C" {
#define END_EXTERN_C }
#else
#define BEGIN_EXTERN_C
#define END_EXTERN_C
#endif

/* NORETURN portability */

#if defined __GNUC__ && __GNUC__  >= 3
#define NORETURN __attribute__((__noreturn__))
#else
#define NORETURN
#endif

BEGIN_EXTERN_C

/* "schar" signed character type */

typedef signed char schar;

/* "integer" 32-bit integer type */

#if SIZEOF_LONG > 4
typedef int integer;
#define INTEGER_MAX INT_MAX
#define INTEGER_MIN INT_MIN
#else
typedef long integer;
#define INTEGER_MAX LONG_MAX
#define INTEGER_MIN LONG_MIN
#endif /* SIZEOF_LONG > 4 */

END_EXTERN_C

#endif /* not TECTONIC_TECTONIC_H */
