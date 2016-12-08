/* tectonic/tectonic.h: global, public header for Tectonic
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_TECTONIC_H
#define TECTONIC_TECTONIC_H

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

/* NORETURN portability */

#if defined __GNUC__ && __GNUC__  >= 3
#define NORETURN __attribute__((__noreturn__))
#else
#define NORETURN
#endif

/* "integer" 32-bit integer type used frequently */

typedef int32_t integer;
#define INTEGER_MAX INT32_MAX
#define INTEGER_MIN INT32_MIN

/* TeX-specific enums */

typedef enum {
    HISTORY_SPOTLESS = 0,
    HISTORY_WARNING_ISSUED = 1,
    HISTORY_ERROR_ISSUED = 2,
    HISTORY_FATAL_ERROR = 3
} tt_history_t;

/* The actual API */

BEGIN_EXTERN_C

/* engine-interface.c */

extern int tt_set_int_variable (char *var_name, int value);
extern int tt_set_string_variable (char *var_name, char *value);

/* xetexini.c */

extern tt_history_t tt_run_engine (char *input_file_name);
extern void tt_misc_initialize (char *dump_name);

END_EXTERN_C

#endif /* not TECTONIC_TECTONIC_H */
