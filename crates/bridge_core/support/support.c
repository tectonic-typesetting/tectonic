/* Copyright 2017-2021 the Tectonic Project
 * Licensed under the MIT License.
*/

#include "tectonic_bridge_core.h"

#include <setjmp.h>
#include <stdio.h> /*vsnprintf*/


#define BUF_SIZE 1024
static char format_buf[BUF_SIZE] = "";


/* The memory management utilities. */

void *
xcalloc(size_t nelem, size_t elsize)
{
    void *new_mem = calloc(nelem ? nelem : 1, elsize ? elsize : 1);

    if (new_mem == NULL)
        _tt_abort("xcalloc request for %lu elements of size %lu failed",
                  (unsigned long) nelem, (unsigned long) elsize);

    return new_mem;
}


void *
xmalloc(size_t size)
{
    void *new_mem = malloc(size ? size : 1);

    if (new_mem == NULL)
        _tt_abort("xmalloc request for %lu bytes failed", (unsigned long) size);

    return new_mem;
}


void *
xrealloc(void *old_ptr, size_t size)
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
xstrdup(const char *s)
{
    char *new_string = xmalloc(strlen (s) + 1);
    return strcpy(new_string, s);
}


/* C API helpers that don't rely upon the global state variables */

PRINTF_FUNC(2,0) void
ttstub_diag_vprintf(ttbc_diagnostic_t *diag, const char *format, va_list ap)
{
    vsnprintf(format_buf, BUF_SIZE, format, ap);
    ttbc_diag_append(diag, format_buf);
}


PRINTF_FUNC(2,3) void
ttstub_diag_printf(ttbc_diagnostic_t *diag, const char *format, ...)
{
    va_list ap;

    va_start(ap, format);
    ttstub_diag_vprintf(diag, format, ap);
    va_end(ap);
}


/* The global state helpers */

static ttbc_state_t *tectonic_global_bridge_core = NULL;
static jmp_buf jump_buffer;


NORETURN PRINTF_FUNC(1,2) int
_tt_abort(const char *format, ...)
{
    va_list ap;

    va_start(ap, format);
    vsnprintf(format_buf, BUF_SIZE, format, ap);
    va_end(ap);
    longjmp(jump_buffer, 1);
}


/* This function is referenced on the Rust side to get, well, the error message,
 * when there's an abort. `format_buf` is used in other places so this it's only
 * correct to use this function after a _tt_abort() is called. */
const char *
_ttbc_get_error_message(void)
{
    return format_buf;
}


jmp_buf *
ttbc_global_engine_enter(ttbc_state_t *api)
{
    tectonic_global_bridge_core = api;
    return &jump_buffer;
}


void
ttbc_global_engine_exit(void)
{
    tectonic_global_bridge_core = NULL;
}


PRINTF_FUNC(1,2) void
ttstub_issue_warning(const char *format, ...)
{
    va_list ap;

    va_start(ap, format);
    vsnprintf(format_buf, BUF_SIZE, format, ap);
    va_end(ap);
    ttbc_issue_warning(tectonic_global_bridge_core, format_buf);
}


PRINTF_FUNC(1,2) void
ttstub_issue_error(const char *format, ...)
{
    va_list ap;

    va_start(ap, format);
    vsnprintf(format_buf, BUF_SIZE, format, ap); /* Not ideal to (ab)use format_buf here */
    va_end(ap);
    ttbc_issue_error(tectonic_global_bridge_core, format_buf);
}


void
ttstub_diag_finish(ttbc_diagnostic_t *diag)
{
    ttbc_diag_finish(tectonic_global_bridge_core, diag);
}


rust_output_handle_t
ttstub_output_open(char const *path, int is_gz)
{
    return ttbc_output_open(tectonic_global_bridge_core, path, is_gz);
}


rust_output_handle_t
ttstub_output_open_stdout(void)
{
    return ttbc_output_open_stdout(tectonic_global_bridge_core);
}


int
ttstub_output_putc(rust_output_handle_t handle, int c)
{
    return ttbc_output_putc(tectonic_global_bridge_core, handle, c);
}


size_t
ttstub_output_write(rust_output_handle_t handle, const char *data, size_t len)
{
    return ttbc_output_write(tectonic_global_bridge_core, handle, (const uint8_t*) data, len);
}


PRINTF_FUNC(2,3) int
ttstub_fprintf(rust_output_handle_t handle, const char *format, ...)
{
    static char fprintf_buf[BUF_SIZE] = "";
    va_list ap;

    va_start(ap, format);
    int len = vsnprintf(fprintf_buf, BUF_SIZE, format, ap);
    va_end(ap);

    if (len >= BUF_SIZE) {
        len = BUF_SIZE - 1;
        fprintf_buf[len] = '\0';
    }

    if (len >= 0) {
        ttstub_output_write(handle, fprintf_buf, len);
    }

    return len;
}


int
ttstub_output_flush(rust_output_handle_t handle)
{
    return ttbc_output_flush(tectonic_global_bridge_core, handle);
}


int
ttstub_output_close(rust_output_handle_t handle)
{
    return ttbc_output_close(tectonic_global_bridge_core, handle);
}


rust_input_handle_t
ttstub_input_open(char const *path, ttbc_file_format format, int is_gz)
{
    return ttbc_input_open(tectonic_global_bridge_core, path, format, is_gz);
}


rust_input_handle_t
ttstub_input_open_primary(void)
{
    return ttbc_input_open_primary(tectonic_global_bridge_core);
}


ssize_t
ttstub_get_last_input_abspath(char *buffer, size_t len)
{
    return ttbc_get_last_input_abspath(tectonic_global_bridge_core, (uint8_t *) buffer, len);
}

size_t
ttstub_input_get_size(rust_input_handle_t handle)
{
    return ttbc_input_get_size(tectonic_global_bridge_core, handle);
}


time_t
ttstub_input_get_mtime(rust_input_handle_t handle)
{
    /* Due to the Musl 1.2 "time64" transition, we can't safely bridge time_t
     * between Rust and C code. And formally, ISO C provides nearly no
     * guarantees about what the type time_t actually is. So let's just cast and
     * hope for the best. */
    int64_t ti = ttbc_input_get_mtime(tectonic_global_bridge_core, handle);
    return (time_t) ti;
}


size_t
ttstub_input_seek(rust_input_handle_t handle, ssize_t offset, int whence)
{
    int internal_error = 0;

    size_t rv = ttbc_input_seek(tectonic_global_bridge_core, handle, offset, whence, &internal_error);

    if (internal_error) {
        // Nonzero indicates a serious internal error.
        longjmp(jump_buffer, 1);
    }

    return rv;
}


ssize_t
ttstub_input_read(rust_input_handle_t handle, char *data, size_t len)
{
    return ttbc_input_read(tectonic_global_bridge_core, handle, (uint8_t *) data, len);
}


int
ttstub_input_getc(rust_input_handle_t handle)
{
    return ttbc_input_getc(tectonic_global_bridge_core, handle);
}


int
ttstub_input_ungetc(rust_input_handle_t handle, int ch)
{
    return ttbc_input_ungetc(tectonic_global_bridge_core, handle, ch);
}


int
ttstub_input_close(rust_input_handle_t handle)
{
    if (ttbc_input_close(tectonic_global_bridge_core, handle)) {
        // Nonzero return value indicates a serious internal error.
        longjmp(jump_buffer, 1);
    }

    return 0;
}


int
ttstub_get_file_md5(char const *path, char *digest)
{
    return ttbc_get_file_md5(tectonic_global_bridge_core, path, (uint8_t *) digest);
}

int
ttstub_shell_escape(const unsigned short *cmd, size_t len)
{
    return ttbc_shell_escape(tectonic_global_bridge_core, cmd, len);
}
