/* tectonic/errors.c -- error handling
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"

#include <stdarg.h>

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

    if (interaction == BATCH_MODE)
        selector--;

    error_here_with_diagnostic("");
}


/*82: */
static void
post_error_message(int need_to_print_it)
{
    capture_to_diagnostic(NULL);

    if (interaction == ERROR_STOP_MODE)
        interaction = SCROLL_MODE;

    if (need_to_print_it && log_opened)
        error();

    history = HISTORY_FATAL_ERROR;
    close_files_and_terminate();
    tt_cleanup();
    ttstub_output_flush(rust_stdout);
}


void
error(void)
{
    if (history < HISTORY_ERROR_ISSUED)
        history = HISTORY_ERROR_ISSUED;

    print_char('.');
    show_context();
    if (halt_on_error_p) {
        history = HISTORY_FATAL_ERROR;
        post_error_message(0);
        _tt_abort("halted on potentially-recoverable error as specified");
    }

    /* This used to be where there was a bunch of code if "interaction ==
     * error_stop_mode" that would let the use interactively try to solve the
     * error. */

    error_count++;
    if (error_count == 100) {
        print_nl_cstr("(That makes 100 errors; please try again.)");
        history = HISTORY_FATAL_ERROR;
        post_error_message(0);
        _tt_abort("halted after 100 potentially-recoverable errors");
    }

    if (interaction > BATCH_MODE)
        selector--;

    if (use_err_help) {
        print_ln();
        give_err_help();
    } else {
        while (help_ptr > 0) {
            help_ptr--;
            print_nl_cstr(help_line[help_ptr]);
        }
    }

    print_ln();
    if (interaction > BATCH_MODE)
        selector++;
    print_ln();
}


void
fatal_error(const char* s)
{
    pre_error_message();
    print_cstr("Emergency stop");
    print_nl_cstr(s);
    capture_to_diagnostic(NULL); // started in pre_error_message

    close_files_and_terminate();
    tt_cleanup();
    ttstub_output_flush(rust_stdout);
    _tt_abort("%s", s);
}


void
overflow(const char* s, int32_t n)
{
    pre_error_message();
    print_cstr("TeX capacity exceeded, sorry [");

    print_cstr(s);
    print_char('=');
    print_int(n);
    print_char(']');

    help_ptr = 2;
    help_line[1] = "If you really absolutely need more capacity,";
    help_line[0] = "you can ask a wizard to enlarge me.";
    post_error_message(1);
    _tt_abort("halted on overflow()");
}


void
confusion(const char* s)
{
    pre_error_message();

    if (history < HISTORY_ERROR_ISSUED) {
        print_cstr("This can't happen (");
        print_cstr(s);
        print_char(')');

        help_ptr = 1;
        help_line[0] = "I'm broken. Please show this to someone who can fix can fix";
    } else {
        print_cstr("I can't go on meeting you like this");

        help_ptr = 2;
        help_line[1] = "One of your faux pas seems to have wounded me deeply...";
        help_line[0] = "in fact, I'm barely conscious. Please fix it and try again.";
    }

    post_error_message(1);
    _tt_abort("halted on confusion()");
}


void
pdf_error(const char* t, const char* p)
{
    pre_error_message();

    print_cstr("Error");

    if (t != 0) {
        print_cstr(" (");
        print_cstr(t);
        print(')');
    }

    print_cstr(": ");
    print_cstr(p);

    post_error_message(1);
    _tt_abort("halted on pdf_error()");
}
