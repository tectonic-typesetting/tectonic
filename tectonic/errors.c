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

/*82: */
void
jump_out(void)
{
    close_files_and_terminate();
    fflush(stdout);

    switch (history) {
    case HISTORY_SPOTLESS:
    case HISTORY_WARNING_ISSUED:
	exit (0);
    default:
	exit (1);
    }
}


void error(void)
{
    error_regmem UTF16_code c;
    integer s1, s2, s3, s4;
    if (history < HISTORY_ERROR_ISSUED)
        history = HISTORY_ERROR_ISSUED;
    print_char(46 /*"." */ );
    show_context();
    if ((halt_on_error_p)) {
        history = HISTORY_FATAL_ERROR;
        jump_out();
    }
    if (interaction == 3 /*error_stop_mode */ ) /*87: */
        while (true) {

 lab22:                        /*continue */ clear_for_error_prompt();
            {
                ;
                print(65546L /*"? " */ );
                term_input();
            }
            if (last == first)
                return;
            c = buffer[first];
            if (c >= 97 /*"a" */ )
                c = c - 32;
            switch (c) {
            case 48:
            case 49:
            case 50:
            case 51:
            case 52:
            case 53:
            case 54:
            case 55:
            case 56:
            case 57:
                if (deletions_allowed) {        /*92: */
                    s1 = cur_tok;
                    s2 = cur_cmd;
                    s3 = cur_chr;
                    s4 = align_state;
                    align_state = 1000000L;
                    OK_to_interrupt = false;
                    if ((last > first + 1) && (buffer[first + 1] >= 48 /*"0" */ )
                        && (buffer[first + 1] <= 57 /*"9" */ ))
                        c = c * 10 + buffer[first + 1] - 48 * 11;
                    else
                        c = c - 48;
                    while (c > 0) {

                        get_token();
                        c--;
                    }
                    cur_tok = s1;
                    cur_cmd = s2;
                    cur_chr = s3;
                    align_state = s4;
                    OK_to_interrupt = true;
                    {
                        help_ptr = 2;
                        help_line[1] = 65559L /*"I have just deleted some text, as you asked." */ ;
                        help_line[0] = 65560L /*"You can now delete more, or insert, or whatever." */ ;
                    }
                    show_context();
                    goto lab22;
                }
                break;
                ;

            case 69:
                if (base_ptr > 0) {
                    edit_name_start = str_start[(input_stack[base_ptr].name_field) - 65536L];
                    edit_name_length =
                        str_start[(input_stack[base_ptr].name_field + 1) - 65536L] -
                        str_start[(input_stack[base_ptr].name_field) - 65536L];
                    edit_line = line;
                    jump_out();
                }
                break;
            case 72:
                {
                    if (use_err_help) {
                        give_err_help();
                        use_err_help = false;
                    } else {

                        if (help_ptr == 0) {
                            help_ptr = 2;
                            help_line[1] = 65561L /*"Sorry, I don't know how to help in this situation." */ ;
                            help_line[0] = 65562L /*"Maybe you should try asking a human?" */ ;
                        }
                        do {
                            help_ptr--;
                            print(help_line[help_ptr]);
                            print_ln();
                        } while (!(help_ptr == 0));
                    }
                    {
                        help_ptr = 4;
                        help_line[3] = 65563L /*"Sorry, I already gave what help I could..." */ ;
                        help_line[2] = 65562L /*"Maybe you should try asking a human?" */ ;
                        help_line[1] = 65564L /*"An error might have occurred before I noticed any problems." */ ;
                        help_line[0] = 65565L /*"``If all else fails, read the instructions.''" */ ;
                    }
                    goto lab22;
                }
                break;
            case 73:
                {
                    begin_file_reading();
                    if (last > first + 1) {
                        cur_input.loc_field = first + 1;
                        buffer[first] = 32 /*" " */ ;
                    } else {

                        {
                            ;
                            print(65558L /*"insert>" */ );
                            term_input();
                        }
                        cur_input.loc_field = first;
                    }
                    first = last;
                    cur_input.limit_field = last - 1;
                    return;
                }
                break;
            case 81:
            case 82:
            case 83:
                {
                    error_count = 0;
                    interaction = 0 /*batch_mode */  + c - 81;
                    print(65553L /*"OK, entering " */ );
                    switch (c) {
                    case 81:
                        {
                            print_esc(65554L /*"batchmode" */ );
                            selector--;
                        }
                        break;
                    case 82:
                        print_esc(65555L /*"nonstopmode" */ );
                        break;
                    case 83:
                        print_esc(65556L /*"scrollmode" */ );
                        break;
                    }
                    print(65557L /*"..." */ );
                    print_ln();
                    fflush(stdout);
                    return;
                }
                break;
            case 88:
                {
                    interaction = 2 /*scroll_mode */ ;
                    jump_out();
                }
                break;
            default:
                ;
                break;
            }
            {
                print(65547L /*"Type <return> to proceed, S to scroll future error messages," */ );
                print_nl(65548L /*"R to run without stopping, Q to run quietly," */ );
                print_nl(65549L /*"I to insert something, " */ );
                if (base_ptr > 0)
                    print(65550L /*"E to edit your file," */ );
                if (deletions_allowed)
                    print_nl(65551L /*"1 or ... or 9 to ignore the next 1 to 9 tokens of input," */ );
                print_nl(65552L /*"H for help, X to quit." */ );
            }
        }
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
    } else
        while (help_ptr > 0) {

            help_ptr--;
            print_nl(help_line[help_ptr]);
        }
    print_ln();
    if (interaction > 0 /*batch_mode */ )
        selector++;
    print_ln();
}

void zfatal_error(str_number s)
{
    fatal_error_regmem normalize_selector();
    {
        if (interaction == 3 /*error_stop_mode */ ) ;
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(65544L /*"! " */ );
        print(65567L /*"Emergency stop" */ );
    }
    {
        help_ptr = 1;
        help_line[0] = s;
    }
    {
        if (interaction == 3 /*error_stop_mode */ )
            interaction = 2 /*scroll_mode */ ;
        if (log_opened)
            error();
        ;

        history = HISTORY_FATAL_ERROR;
        jump_out();
    }
}

void zoverflow(str_number s, integer n)
{
    overflow_regmem normalize_selector();
    {
        if (interaction == 3 /*error_stop_mode */ ) ;
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(65544L /*"! " */ );
        print(65568L /*"TeX capacity exceeded, sorry [" */ );
    }
    print(s);
    print_char(61 /*"=" */ );
    print_int(n);
    print_char(93 /*"]" */ );
    {
        help_ptr = 2;
        help_line[1] = 65569L /*"If you really absolutely need more capacity," */ ;
        help_line[0] = 65570L /*"you can ask a wizard to enlarge me." */ ;
    }
    {
        if (interaction == 3 /*error_stop_mode */ )
            interaction = 2 /*scroll_mode */ ;
        if (log_opened)
            error();
        ;

        history = HISTORY_FATAL_ERROR;
        jump_out();
    }
}

void zconfusion(str_number s)
{
    confusion_regmem normalize_selector();
    if (history < HISTORY_ERROR_ISSUED) {
        {
            if (interaction == 3 /*error_stop_mode */ ) ;
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(65544L /*"! " */ );
            print(65571L /*"This can't happen (" */ );
        }
        print(s);
        print_char(41 /*")" */ );
        {
            help_ptr = 1;
            help_line[0] = 65572L /*"I'm broken. Please show this to someone who can fix can fix" */ ;
        }
    } else {

        {
            if (interaction == 3 /*error_stop_mode */ ) ;
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(65544L /*"! " */ );
            print(65573L /*"I can't go on meeting you like this" */ );
        }
        {
            help_ptr = 2;
            help_line[1] = 65574L /*"One of your faux pas seems to have wounded me deeply..." */ ;
            help_line[0] = 65575L /*"in fact, I'm barely conscious. Please fix it and try again." */ ;
        }
    }
    {
        if (interaction == 3 /*error_stop_mode */ )
            interaction = 2 /*scroll_mode */ ;
        if (log_opened)
            error();
        ;

        history = HISTORY_FATAL_ERROR;
        jump_out();
    }
}
