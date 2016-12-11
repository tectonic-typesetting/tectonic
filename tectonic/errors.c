/* tectonic/errors.c -- error handling
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>

#include <setjmp.h>
#include <stdarg.h>

#define BUF_SIZE 1024

static jmp_buf jump_buffer;
static char error_buf[BUF_SIZE] = "";


int
_tt_setjmp ()
{
    return setjmp (jump_buffer);
}


NORETURN PRINTF_FUNC(1,2) int
_tt_abort (const_string format, ...)
{
    va_list ap;

    va_start (ap, format);
    vsnprintf (error_buf, BUF_SIZE, format, ap);
    va_end (ap);
    longjmp (jump_buffer, 1);
}

const const_string
tt_get_error_message (void)
{
    return error_buf;
}
