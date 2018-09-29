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

/* Universal headers */

#include <stdbool.h>
#include <stdint.h> /* for int32_t */

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

#endif /* not TECTONIC_CORE_FOUNDATION_H */
