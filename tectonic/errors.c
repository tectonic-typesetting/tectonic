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

/* WEBby error-handling code: */

static void
pre_error_message (void)
{
    /* FKA normalize_selector(): */

    if (log_opened)
        selector = SELECTOR_TERM_AND_LOG;
    else
        selector = SELECTOR_TERM_ONLY;

    if (job_name == 0)
        open_log_file();

    if (interaction == 0 /*batch_mode */ )
        selector--;

    if (file_line_error_style_p)
	print_file_line();
    else
	print_nl(65544L /*"! " */ );
}


/*82: */
static NORETURN void
jump_out(void)
{
    if (interaction == 3 /*error_stop_mode */ )
	interaction = 2 /*scroll_mode */ ;

    if (log_opened)
	error();

    history = HISTORY_FATAL_ERROR;
    close_files_and_terminate();
    fflush(stdout);
    exit (1);
}


void
error(void)
{
    UTF16_code c;
    integer s1, s2, s3, s4;

    if (history < HISTORY_ERROR_ISSUED)
        history = HISTORY_ERROR_ISSUED;

    print_char(46 /*"." */ );
    show_context();
    if (halt_on_error_p) {
        history = HISTORY_FATAL_ERROR;
        jump_out();
    }

    /* This used to be where there was a bunch of code if "interaction ==
     * error_stop_mode" that would let the use interactively try to solve the
     * error. */

    error_count++;
    if (error_count == 100) {
        print_nl(65545L /*"(That makes 100 errors; please try again.)" */ );
        history = HISTORY_FATAL_ERROR;
        jump_out();
    }

    if (interaction > 0 /*batch_mode */ )
        selector--;

    if (use_err_help) {
        print_ln();
        give_err_help();
    } else {
        while (help_ptr > 0) {
            help_ptr--;
            print_nl(help_line[help_ptr]);
        }
    }

    print_ln();
    if (interaction > 0 /*batch_mode */ )
        selector++;
    print_ln();
}


void
fatal_error(str_number s)
{
    pre_error_message();
    print(65567L /*"Emergency stop" */ );
    help_ptr = 1;
    help_line[0] = s;
    jump_out();
}


void
overflow(str_number s, integer n)
{
    pre_error_message();
    print(65568L /*"TeX capacity exceeded, sorry [" */ );

    print(s);
    print_char(61 /*"=" */ );
    print_int(n);
    print_char(93 /*"]" */ );

    help_ptr = 2;
    help_line[1] = 65569L /*"If you really absolutely need more capacity," */ ;
    help_line[0] = 65570L /*"you can ask a wizard to enlarge me." */ ;
    jump_out();
}


void
confusion(str_number s)
{
    pre_error_message();

    if (history < HISTORY_ERROR_ISSUED) {
	print(65571L /*"This can't happen (" */ );
        print(s);
        print_char(41 /*")" */ );

	help_ptr = 1;
	help_line[0] = 65572L /*"I'm broken. Please show this to someone who can fix can fix" */ ;
    } else {
	print(65573L /*"I can't go on meeting you like this" */ );

	help_ptr = 2;
	help_line[1] = 65574L /*"One of your faux pas seems to have wounded me deeply..." */ ;
	help_line[0] = 65575L /*"in fact, I'm barely conscious. Please fix it and try again." */ ;
    }

    jump_out();
}


void
pdf_error(str_number t, str_number p)
{
    pre_error_message();

    print(65588L /*"Error" */ );

    if (t != 0) {
        print(65566L /*" (" */ );
        print(t);
        print(41 /*")" */ );
    }

    print(65589L /*": " */ );
    print(p);

    jump_out();
}
