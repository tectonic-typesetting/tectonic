/* tectonic/errors.c -- error handling
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex_bindings.h"

#include <stdarg.h>

/* WEBby error-handling code: */

void
overflow(const char* s, int32_t n)
{
    pre_error_message();
    print_cstr("TeX capacity exceeded, sorry [");

    print_cstr(s);
    print_char('=');
    print_int(n);
    print_char(']');

    set_help_ptr(2);
    set_help_line(1, "If you really absolutely need more capacity,");
    set_help_line(0, "you can ask a wizard to enlarge me.");
    post_error_message(1);
    _tt_abort("halted on overflow()");
}


void
confusion(const char* s)
{
    pre_error_message();

    if (history() < HISTORY_ERROR_ISSUED) {
        print_cstr("This can't happen (");
        print_cstr(s);
        print_char(')');

        set_help_ptr(1);
        set_help_line(0, "I'm broken. Please show this to someone who can fix can fix");
    } else {
        print_cstr("I can't go on meeting you like this");

        set_help_ptr(2);
        set_help_line(1, "One of your faux pas seems to have wounded me deeply...");
        set_help_line(0, "in fact, I'm barely conscious. Please fix it and try again.");
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
