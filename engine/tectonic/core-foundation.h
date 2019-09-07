/* tectonic/core-foundation.h: the first header to include in Tectonic's C/C++ code
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

/* This header should (eventually ...) be included first in all of Tectonic's
 * C/C++ files. It essentially concerns itself with defining basic types and
 * portability.
 */

#ifndef TECTONIC_CORE_FOUNDATION_H
#define TECTONIC_CORE_FOUNDATION_H

/* High-level defines */

#define _DARWIN_USE_64_BIT_INODE 1

/* Some versions of g++ do not define PRId64 and friends unless we #define
 * this before including inttypes.h. This was apparently an idea that was
 * proposed for C++11 but didn't make it into the final standard. */
#define __STDC_FORMAT_MACROS

/* Universal headers */

#include <assert.h>
#include <ctype.h>
#include <errno.h>
#include <float.h>
#include <inttypes.h>
#include <limits.h>
#include <math.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

/* Convenience for C++: this way Emacs doesn't try to indent the prototypes,
 * which I find annoying. */

#ifdef __cplusplus
#define BEGIN_EXTERN_C extern "C" {
#define END_EXTERN_C }
#else
#define BEGIN_EXTERN_C
#define END_EXTERN_C
#endif

/* Portability: NORETURN annotation */

#if defined __GNUC__ && __GNUC__ >= 3
#define NORETURN __attribute__((__noreturn__))
#else
#define NORETURN
#endif

/* Portability: annotations to validate args of printf-like functions */

#if defined __GNUC__ && __GNUC__ >= 3
#define PRINTF_FUNC(ifmt,iarg) __attribute__((format(printf, ifmt, iarg)))
#else
#define PRINTF_FUNC(ifmt,iarg)
#endif

/* Portability: inline annotation */

#ifdef _MSC_VER
# ifndef __cplusplus
#  define inline __inline
# endif
#endif

/* Portability: MSVC variations on various common functions */

#ifdef _MSC_VER
# define strcasecmp _stricmp
# define strncasecmp _strnicmp
# if defined(_VC_CRT_MAJOR_VERSION) && _VC_CRT_MAJOR_VERSION < 14
#  define snprintf _snprintf
#  define strtoll _strtoi64
# endif
#endif

/* Portability: ssize_t
 *
 * On Unix, sys/types.h gives ssize_t. On MSVC we need to do the following:
 */

#if defined(_MSC_VER)
#include <BaseTsd.h>
typedef SSIZE_T ssize_t;
#endif

/* Portability: M_PI
 *
 * MSVC doesn't always define it. Based on research, sometimes #defining
 * _USE_MATH_DEFINES should fix the problem, but it's not clear if that
 * *always* works in all versions, and it's easy to cut to the heart of the
 * matter:
 */

#ifndef M_PI
# define M_PI 3.14159265358979
#endif

/* Portability: printf format arguments for various non-core types.
 *
 * <inttypes.h> ought to define these for most types. We use a custom one for
 * size_t since older MSVC doesn't provide %z.
 */

#ifndef PRId64
# if defined(SIZEOF_LONG)
#  if SIZEOF_LONG == 8
#   define PRId64 "ld"
#  else
#   define PRId64 "lld"
#  endif
# elif defined(_WIN32)
#  define PRId64 "I64d"
# else
#  error "unhandled compiler/platform for PRId64 definition"
# endif
#endif

#ifndef PRIdPTR
# define PRIdPTR "ld"
#endif
#ifndef PRIxPTR
# define PRIxPTR "lx"
#endif

#ifdef _WIN32
# define PRIuZ "Iu"
# define PRIXZ "IX"
#else
# define PRIuZ "zu"
# define PRIXZ "zX"
#endif

#endif /* not TECTONIC_CORE_FOUNDATION_H */
