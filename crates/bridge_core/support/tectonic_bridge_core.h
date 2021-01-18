/* Copyright 2016-2020 the Tectonic Project
 * Licensed under the MIT License.
*/

/* This header should (eventually ...) be included first in all of Tectonic's
 * C/C++ files. It essentially concerns itself with defining basic types and
 * portability.
 */

#ifndef TECTONIC_CORE_BRIDGE_H
#define TECTONIC_CORE_BRIDGE_H

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
#include <setjmp.h> /* for global handling below */
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <time.h> /* time_t */

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

/* Get the core definitions from the Rust bridge layer. These are generated
 * during the build process by cbindgen. */
#include "tectonic_bridge_core_generated.h"

/* Now, extra definitions implemented in C */

BEGIN_EXTERN_C

/* Generic memory management wrappers used widely in the original code. */

char *xstrdup(const char *s);
void *xmalloc(size_t size);
void *xrealloc(void *old_address, size_t new_size);
void *xcalloc(size_t nelem, size_t elsize);

static inline void *mfree(void *ptr) {
    free(ptr);
    return NULL;
}

/* Generic string utilities used widely in the original code. */

#ifndef isblank
#define isblank(c) ((c) == ' ' || (c) == '\t')
#endif
#define ISBLANK(c) (isascii(c) && isblank((unsigned char)c))

/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#define IS_DIR_SEP(ch) ((ch) == '/')

static inline bool streq_ptr(const char *s1, const char *s2) {
    if (s1 && s2)
        return strcmp(s1, s2) == 0;
    return false;
}

static inline const char *strstartswith(const char *s, const char *prefix) {
    size_t length;

    length = strlen(prefix);
    if (strncmp(s, prefix, length) == 0)
        return s + length;
    return NULL;
}

/* Bridge API helpers using C library functions */

PRINTF_FUNC(2,3) void ttstub_diag_printf(ttbc_diagnostic_t *diag, const char *format, ...);
PRINTF_FUNC(2,0) void ttstub_diag_vprintf(ttbc_diagnostic_t *diag, const char *format, va_list ap);

/* Wrappers that use our global state variables: a global handle to the bridge
 * state, and a global jmp_buf longjmp() buffer for error handling.
 *
 * The naming here is a bit haphazard, for historical reasons.
 *
 * The global state APIs **must* be used in this way:
 *
 * ```
 * int myentrypoint(const ttbc_state_t *api)
 * {
 *     if (setjmp(*ttbc_global_engine_enter(api))) {
 *         ttbc_global_engine_exit();
 *         return MY_FATAL_ABORT_CODE;
 *     }
 *
 *     my_result_code = my_main_implementation();
 *     ttbc_global_engine_exit();
 *     return my_result_code;
 * }
 * ```
 *
 * They are based on setjmp/longjmp to catch fatal error conditions so you have
 * to understand how those functions work.
 */

jmp_buf *ttbc_global_engine_enter(ttbc_state_t *api);
void ttbc_global_engine_exit(void);

NORETURN PRINTF_FUNC(1,2) int _tt_abort(const char *format, ...);

PRINTF_FUNC(1,2) void ttstub_issue_warning(const char *format, ...);
PRINTF_FUNC(1,2) void ttstub_issue_error(const char *format, ...);

void ttstub_diag_finish(ttbc_diagnostic_t *diag);

rust_output_handle_t ttstub_output_open(char const *path, int is_gz);
rust_output_handle_t ttstub_output_open_stdout(void);
int ttstub_output_putc(rust_output_handle_t handle, int c);
size_t ttstub_output_write(rust_output_handle_t handle, const char *data, size_t len);
PRINTF_FUNC(2,3) int ttstub_fprintf(rust_output_handle_t handle, const char *format, ...);
int ttstub_output_flush(rust_output_handle_t handle);
int ttstub_output_close(rust_output_handle_t handle);

rust_input_handle_t ttstub_input_open(char const *path, ttbc_file_format format, int is_gz);
rust_input_handle_t ttstub_input_open_primary(void);
size_t ttstub_input_get_size(rust_input_handle_t handle);
time_t ttstub_input_get_mtime(rust_input_handle_t handle);
size_t ttstub_input_seek(rust_input_handle_t handle, ssize_t offset, int whence);
ssize_t ttstub_input_read(rust_input_handle_t handle, char *data, size_t len);
int ttstub_input_getc(rust_input_handle_t handle);
int ttstub_input_ungetc(rust_input_handle_t handle, int ch);
int ttstub_input_close(rust_input_handle_t handle);

int ttstub_get_file_md5(char const *path, char *digest);

END_EXTERN_C

#endif /* not TECTONIC_CORE_BRIDGE_H */
