/* tectonic/tectonic.h: global, public header for Tectonic
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_TECTONIC_H
#define TECTONIC_TECTONIC_H

/* TODO: these need to land in a configure-generated header file */

#define SIZEOF_INT 4
#define SIZEOF_LONG 8
#define SIZEOF_OFF_T 8
#define SIZEOF_VOID_P 8

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

#endif /* not TECTONIC_TECTONIC_H */
