/* tectonic/core-bridge.c: the C/C++ => Rust bridge
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/

#include "core-bridge.h"

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h> /*vsnprintf*/

/* TODO: these are needed for the various *_main routines which should
 * probably be moved out into other files. */
#include "bibtex.h"
#include "dpx-dvipdfmx.h"
#include "xetex-xetexd.h"


/* The global variable that represents the Rust API. Some fine day we'll get
 * rid of all of the globals ... */

static const tt_bridge_api_t *tectonic_global_bridge = NULL;


/* Highest-level abort/error handling. */

#define BUF_SIZE 1024

static jmp_buf jump_buffer;
static char error_buf[BUF_SIZE] = "";

NORETURN PRINTF_FUNC(1,2) int
_tt_abort(const char *format, ...)
{
    va_list ap;

    va_start(ap, format);
    vsnprintf(error_buf, BUF_SIZE, format, ap);
    va_end(ap);
    longjmp(jump_buffer, 1);
}

const char *
tt_get_error_message(void)
{
    return error_buf;
}


/* Running the actual engines. Those code needs to be centralized for unified
 * setjmp aborts and error message extraction. */

int
tex_simple_main(const tt_bridge_api_t *api, const char *dump_name, const char *input_file_name, time_t build_date)
{
    int rv;

    tectonic_global_bridge = api;

    if (setjmp(jump_buffer)) {
        tectonic_global_bridge = NULL;
        return HISTORY_FATAL_ERROR;
    }

    rv = tt_run_engine(dump_name, input_file_name, build_date);
    tectonic_global_bridge = NULL;
    return rv;
}


int
dvipdfmx_simple_main(const tt_bridge_api_t *api, const XdvipdfmxConfig* config, const char *dviname, const char *pdfname, bool compress, bool deterministic_tags, time_t build_date)
{
    int rv;

    tectonic_global_bridge = api;

    if (setjmp(jump_buffer)) {
        tectonic_global_bridge = NULL;
        return 99;
    }

    dpx_config = config;
    rv = dvipdfmx_main(pdfname, dviname, NULL, 0, false, compress, deterministic_tags, false, 0, build_date);
    tectonic_global_bridge = NULL;

    return rv;
}


int
bibtex_simple_main(const tt_bridge_api_t *api, const BibtexConfig *config, const char *aux_file_name)
{
    int rv;

    tectonic_global_bridge = api;

    if (setjmp(jump_buffer)) {
        tectonic_global_bridge = NULL;
        return 99;
    }

    bibtex_config = config;
    rv = bibtex_main(aux_file_name);
    tectonic_global_bridge = NULL;
    return rv;
}


/* Global symbols that route through the global API */

#define TGB tectonic_global_bridge

diagnostic_t
ttstub_diag_warn_begin(void)
{
    return diag_warn_begin();
}

diagnostic_t
ttstub_diag_error_begin(void)
{
    return diag_error_begin();
}

void
ttstub_diag_finish(diagnostic_t diag)
{
    diag_finish(TGB->context, diag);
}

void
ttstub_diag_append(diagnostic_t diag, char const *text)
{
    diag_append(diag, text);
}

PRINTF_FUNC(2,0) void
ttstub_diag_vprintf(diagnostic_t diag, const char *format, va_list ap)
{
    vsnprintf(error_buf, BUF_SIZE, format, ap); /* Not ideal to (ab)use error_buf here */
    ttstub_diag_append(diag, error_buf);
}

PRINTF_FUNC(2,3) void
ttstub_diag_printf(diagnostic_t diag, const char *format, ...)
{
    va_list ap;

    va_start(ap, format);
    ttstub_diag_vprintf(diag, format, ap);
    va_end(ap);
}

PRINTF_FUNC(1,2) void
ttstub_issue_warning(const char *format, ...)
{
    va_list ap;

    va_start(ap, format);
    vsnprintf(error_buf, BUF_SIZE, format, ap); /* Not ideal to (ab)use error_buf here */
    va_end(ap);
    issue_warning(TGB->context, error_buf);
}

PRINTF_FUNC(1,2) void
ttstub_issue_error(const char *format, ...)
{
    va_list ap;

    va_start(ap, format);
    vsnprintf(error_buf, BUF_SIZE, format, ap); /* Not ideal to (ab)use error_buf here */
    va_end(ap);
    issue_error(TGB->context, error_buf);
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
ttstub_get_file_md5(char const *path, char *digest)
{
    // TODO change uses of ttstub_get_file_md5 to pass uint8_t*
    return get_file_md5(TGB->context, path, (uint8_t*) digest);
}

int
ttstub_get_data_md5(char const *data, size_t len, char *digest)
{
    // TODO change uses of ttstub_get_data_md5 to pass uint8_t*
    return get_data_md5((uint8_t const*) data, len, (uint8_t*) digest);
}

rust_output_handle_t
ttstub_output_open(char const *path, int is_gz)
{
    return output_open(TGB->context, path, is_gz);
}

rust_output_handle_t
ttstub_output_open_stdout(void)
{
    return output_open_stdout(TGB->context);
}

int
ttstub_output_putc(rust_output_handle_t handle, int c)
{
    return output_putc(TGB->context, handle, c);
}

size_t
ttstub_output_write(rust_output_handle_t handle, const char *data, size_t len)
{
    return output_write(TGB->context, handle, (const uint8_t*) data, len);
}

int
ttstub_output_flush(rust_output_handle_t handle)
{
    return output_flush(TGB->context, handle);
}

int
ttstub_output_close(rust_output_handle_t handle)
{
    return output_close(TGB->context, handle);
}

rust_input_handle_t
ttstub_input_open(char const *path, tt_input_format_type format, int is_gz)
{
    return input_open(TGB->context, path, format, is_gz);
}

rust_input_handle_t
ttstub_input_open_primary(void)
{
    return input_open_primary(TGB->context);
}

size_t
ttstub_input_get_size(rust_input_handle_t handle)
{
    return input_get_size(TGB->context, handle);
}

time_t
ttstub_input_get_mtime (rust_input_handle_t handle)
{
    return input_get_mtime(TGB->context, handle);
}

size_t
ttstub_input_seek(rust_input_handle_t handle, ssize_t offset, int whence)
{
    int internal_error = 0;
    size_t rv = input_seek(TGB->context, handle, offset, whence, &internal_error);
    if (internal_error) {
        // Nonzero indicates a serious internal error.
        longjmp(jump_buffer, 1);
    }
    return rv;
}

ssize_t
ttstub_input_read(rust_input_handle_t handle, char *data, size_t len)
{
    return input_read(TGB->context, handle, (uint8_t*) data, len);
}

int
ttstub_input_getc(rust_input_handle_t handle)
{
    return input_getc(TGB->context, handle);
}

int
ttstub_input_ungetc(rust_input_handle_t handle, int ch)
{
    return input_ungetc(TGB->context, handle, ch);
}

int
ttstub_input_close(rust_input_handle_t handle)
{
    if (input_close(TGB->context, handle)) {
        // Nonzero return value indicates a serious internal error.
        longjmp(jump_buffer, 1);
    }
    return 0;
}
