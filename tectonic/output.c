/* tectonic/output.c -- functions related to outputting messages
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>
#include <tectonic/synctex.h>
#include <tectonic/stubs.h>


void zprint_raw_char(UTF16_code s, boolean incr_offset)
{
    switch (selector) {
    case 19:
        {
            putc(s, stdout);
            putc(s, log_file);
            if (incr_offset) {
                term_offset++;
                file_offset++;
            }
            if (term_offset == max_print_line) {
                putc('\n', stdout);
                term_offset = 0;
            }
            if (file_offset == max_print_line) {
                putc('\n', log_file);
                file_offset = 0;
            }
        }
        break;
    case 18:
        {
            putc(s, log_file);
            if (incr_offset)
                file_offset++;
            if (file_offset == max_print_line)
                print_ln();
        }
        break;
    case 17:
        {
            putc(s, stdout);
            if (incr_offset)
                term_offset++;
            if (term_offset == max_print_line)
                print_ln();
        }
        break;
    case 16:
        ;
        break;
    case 20:
        if (tally < trick_count)
            trick_buf[tally % error_line] = s;
        break;
    case 21:
        {
            if (pool_ptr < pool_size) {
                str_pool[pool_ptr] = s;
                pool_ptr++;
            }
        }
        break;
    default:
        putc(s, write_file[selector]);
        break;
    }
    tally++;
}

void zprint_char(integer s)
{
    memory_word *eqtb = zeqtb;
    small_number l;

    if ((selector > 20 /*pseudo */ ) && (!doing_special)) {
        if (s >= 65536L) {
            print_raw_char(55296L + (s - 65536L) / 1024, true);
            print_raw_char(56320L + (s - 65536L) % 1024, true);
        } else
            print_raw_char(s, true);
        return;
    }
    if ( /*252: */ s == eqtb[8938789L /*int_base 49 */ ].cint /*:252 */ ) {

        if (selector < 20 /*pseudo */ ) {
            print_ln();
            return;
        }
    }
    if ((s < 32) && (eight_bit_p == 0) && (!doing_special)) {
        print_raw_char(94 /*"^" */ , true);
        print_raw_char(94 /*"^" */ , true);
        print_raw_char(s + 64, true);
    } else if (s < 127)
        print_raw_char(s, true);
    else if ((s == 127)) {
        if ((eight_bit_p == 0) && (!doing_special)) {
            print_raw_char(94 /*"^" */ , true);
            print_raw_char(94 /*"^" */ , true);
            print_raw_char(63 /*"?" */ , true);
        } else
            print_raw_char(s, true);
    } else if ((s < 160) && (eight_bit_p == 0) && (!doing_special)) {
        print_raw_char(94 /*"^" */ , true);
        print_raw_char(94 /*"^" */ , true);
        l = (s % 256) / 16;
        if (l < 10)
            print_raw_char(l + 48, true);
        else
            print_raw_char(l + 87, true);
        l = s % 16;
        if (l < 10)
            print_raw_char(l + 48, true);
        else
            print_raw_char(l + 87, true);
    } else {

        if (s < 2048) {
            print_raw_char(192 + s / 64, false);
            print_raw_char(128 + s % 64, true);
        } else if (s < 65536L) {
            print_raw_char(224 + (s / 4096), false);
            print_raw_char(128 + (s % 4096) / 64, false);
            print_raw_char(128 + (s % 64), true);
        } else {

            print_raw_char(240 + (s / 262144L), false);
            print_raw_char(128 + (s % 262144L) / 4096, false);
            print_raw_char(128 + (s % 4096) / 64, false);
            print_raw_char(128 + (s % 64), true);
        }
    }
}

void zprint(integer s)
{
    memory_word *eqtb = zeqtb;
    pool_pointer j;
    integer nl;

    if (s >= str_ptr)
        s = 65541L /*"???" */ ;
    else if (s < 65535L /*biggest_char */ ) {

        if (s < 0)
            s = 65541L /*"???" */ ;
        else {

            if (selector > 20 /*pseudo */ ) {
                print_char(s);
                return;
            }
            if (( /*252: */ s == eqtb[8938789L /*int_base 49 */ ].cint /*:252 */ )) {

                if (selector < 20 /*pseudo */ ) {
                    print_ln();
                    return;
                }
            }
            nl = eqtb[8938789L /*int_base 49 */ ].cint;
            eqtb[8938789L /*int_base 49 */ ].cint = -1;
            print_char(s);
            eqtb[8938789L /*int_base 49 */ ].cint = nl;
            return;
        }
    }
    j = str_start[(s) - 65536L];
    while (j < str_start[(s + 1) - 65536L]) {

        if ((str_pool[j] >= 55296L) && (str_pool[j] <= 56319L) && (j + 1 < str_start[(s + 1) - 65536L])
            && (str_pool[j + 1] >= 56320L) && (str_pool[j + 1] <= 57343L)) {
            print_char(65536L + (str_pool[j] - 55296L) * 1024 + str_pool[j + 1] - 56320L);
            j = j + 2;
        } else {

            print_char(str_pool[j]);
            j++;
        }
    }
}

void zprint_nl(str_number s)
{
    if (((term_offset > 0) && (odd(selector))) || ((file_offset > 0) && (selector >= 18 /*log_only */ )))
        print_ln();
    print(s);
}

void zprint_esc(str_number s)
{
    memory_word *eqtb = zeqtb;
    integer c;

    c = eqtb[8938785L /*int_base 45 */ ].cint /*:251 */ ;
    if (c >= 0) {

        if (c <= 1114111L /*biggest_usv */ )
            print_char(c);
    }
    print(s);
}

void zprint_the_digs(eight_bits k)
{
    while (k > 0) {
        k--;
        if (dig[k] < 10)
            print_char(48 /*"0" */  + dig[k]);
        else
            print_char(55 /*"A" -10 */  + dig[k]);
    }
}

void zprint_int(integer n)
{
    unsigned char k = 0;
    integer m;

    if (n < 0) {
        print_char(45 /*"-" */ );
        if (n > -100000000L)
            n = -(integer) n;
        else {

            m = -1 - n;
            n = m / 10;
            m = (m % 10) + 1;
            k = 1;
            if (m < 10)
                dig[0] = m;
            else {

                dig[0] = 0;
                n++;
            }
        }
    }
    do {
        dig[k] = n % 10;
        n = n / 10;
        k++;
    } while (!(n == 0));
    print_the_digs(k);
}

void zprint_cs(integer p)
{
    memory_word *eqtb = zeqtb;

    if (p < 2228226L /*hash_base */ ) {
        if (p >= 1114113L /*single_base */ ) {
            if (p == 2228225L /*null_cs */ ) {
                print_esc(65809L /*"csname" */ );
                print_esc(65810L /*"endcsname" */ );
                print_char(32 /*" " */ );
            } else {
                print_esc(p - 1114113L);
                if (eqtb[2254068L /*cat_code_base */  + p - 1114113L].hh.v.RH == 11 /*letter */ )
                    print_char(32 /*" " */ );
            }
        } else if (p < 1 /*active_base */ )
            print_esc(65811L /*"IMPOSSIBLE." */ );
        else
            print_char(p - 1);
    } else if (((p >= 2252239L /*undefined_control_sequence */ ) && (p <= 10053470L /*eqtb_size */ )) || (p > eqtb_top))
        print_esc(65811L /*"IMPOSSIBLE." */ );
    else if ((hash[p].v.RH >= str_ptr))
        print_esc(65812L /*"NONEXISTENT." */ );
    else {
        print_esc(hash[p].v.RH);
        print_char(32 /*" " */ );
    }
}

void zsprint_cs(halfword p)
{
    if (p < 2228226L /*hash_base */ ) {
        if (p < 1114113L /*single_base */ )
            print_char(p - 1);
        else if (p < 2228225L /*null_cs */ )
            print_esc(p - 1114113L);
        else {
            print_esc(65809L /*"csname" */ );
            print_esc(65810L /*"endcsname" */ );
        }
    } else
        print_esc(hash[p].v.RH);
}

void zprint_file_name(integer n, integer a, integer e)
{
    boolean must_quote = false;
    integer quote_char = 0;
    pool_pointer j;

    if (a != 0) {
        j = str_start[(a) - 65536L];
        while (((!must_quote) || (quote_char == 0)) && (j < str_start[(a + 1) - 65536L])) {

            if (str_pool[j] == 32 /*" " */ )
                must_quote = true;
            else if ((str_pool[j] == 34 /*""" */ ) || (str_pool[j] == 39 /*"'" */ )) {
                must_quote = true;
                quote_char = 73 /*""" 39 */  - str_pool[j];
            }
            j++;
        }
    }
    if (n != 0) {
        j = str_start[(n) - 65536L];
        while (((!must_quote) || (quote_char == 0)) && (j < str_start[(n + 1) - 65536L])) {

            if (str_pool[j] == 32 /*" " */ )
                must_quote = true;
            else if ((str_pool[j] == 34 /*""" */ ) || (str_pool[j] == 39 /*"'" */ )) {
                must_quote = true;
                quote_char = 73 /*""" 39 */  - str_pool[j];
            }
            j++;
        }
    }
    if (e != 0) {
        j = str_start[(e) - 65536L];
        while (((!must_quote) || (quote_char == 0)) && (j < str_start[(e + 1) - 65536L])) {

            if (str_pool[j] == 32 /*" " */ )
                must_quote = true;
            else if ((str_pool[j] == 34 /*""" */ ) || (str_pool[j] == 39 /*"'" */ )) {
                must_quote = true;
                quote_char = 73 /*""" 39 */  - str_pool[j];
            }
            j++;
        }
    }
    if (must_quote) {
        if (quote_char == 0)
            quote_char = 34 /*""" */ ;
        print_char(quote_char);
    }
    if (a != 0) {
        register integer for_end;
        j = str_start[(a) - 65536L];
        for_end = str_start[(a + 1) - 65536L] - 1;
        if (j <= for_end)
            do {
                if (str_pool[j] == quote_char) {
                    print(quote_char);
                    quote_char = 73 /*""" 39 */  - quote_char;
                    print(quote_char);
                }
                print(str_pool[j]);
            }
            while (j++ < for_end);
    }
    if (n != 0) {
        register integer for_end;
        j = str_start[(n) - 65536L];
        for_end = str_start[(n + 1) - 65536L] - 1;
        if (j <= for_end)
            do {
                if (str_pool[j] == quote_char) {
                    print(quote_char);
                    quote_char = 73 /*""" 39 */  - quote_char;
                    print(quote_char);
                }
                print(str_pool[j]);
            }
            while (j++ < for_end);
    }
    if (e != 0) {
        register integer for_end;
        j = str_start[(e) - 65536L];
        for_end = str_start[(e + 1) - 65536L] - 1;
        if (j <= for_end)
            do {
                if (str_pool[j] == quote_char) {
                    print(quote_char);
                    quote_char = 73 /*""" 39 */  - quote_char;
                    print(quote_char);
                }
                print(str_pool[j]);
            }
            while (j++ < for_end);
    }
    if (quote_char != 0)
        print_char(quote_char);
}

void zprint_size(integer s)
{
    if (s == 0 /*text_size */ )
        print_esc(65708L /*"textfont" */ );
    else if (s == 256 /*script_size */ )
        print_esc(65709L /*"scriptfont" */ );
    else
        print_esc(65710L /*"scriptscriptfont" */ );
}

void zprint_write_whatsit(str_number s, halfword p)
{
    memory_word *mem = zmem;

    print_esc(s);

    if (mem[p + 1].hh.v.LH < 16)
        print_int(mem[p + 1].hh.v.LH);
    else if (mem[p + 1].hh.v.LH == 16)
        print_char(42 /*"*" */ );
    else
        print_char(45 /*"-" */ );
}

void zprint_native_word(halfword p)
{
    memory_word *mem = zmem;
    integer i, c, cc;

    {
        register integer for_end;
        i = 0;
        for_end = mem[p + 4].qqqq.u.B2 - 1;
        if (i <= for_end)
            do {
                c = get_native_char(p, i);
                if ((c >= 55296L) && (c <= 56319L)) {
                    if (i < mem[p + 4].qqqq.u.B2 - 1) {
                        cc = get_native_char(p, i + 1);
                        if ((cc >= 56320L) && (cc <= 57343L)) {
                            c = 65536L + (c - 55296L) * 1024 + (cc - 56320L);
                            print_char(c);
                            i++;
                        } else
                            print(46 /*"." */ );
                    } else
                        print(46 /*"." */ );
                } else
                    print_char(c);
            }
            while (i++ < for_end);
    }
}

void zprint_sa_num(halfword q)
{
    memory_word *mem = zmem;
    halfword n;

    if (mem[q].hh.u.B0 < 128 /*dimen_val_limit */ )
        n = mem[q + 1].hh.v.RH;
    else {

        n = mem[q].hh.u.B0 % 64;
        q = mem[q].hh.v.RH;
        n = n + 64 * mem[q].hh.u.B0;
        q = mem[q].hh.v.RH;
        n = n + 64 * 64 * (mem[q].hh.u.B0 + 64 * mem[mem[q].hh.v.RH].hh.u.B0);
    }
    print_int(n);
}

void zprint_csnames(integer hstart, integer hfinish)
{
    integer c, h;

    fprintf(stderr, "%s%ld%s%ld%c\n", "fmtdebug:csnames from ", (long)hstart, " to ", (long)hfinish, ':');
    {
        register integer for_end;
        h = hstart;
        for_end = hfinish;
        if (h <= for_end)
            do {
                if (hash[h].v.RH > 0) {
                    {
                        register integer for_end;
                        c = str_start[(hash[h].v.RH) - 65536L];
                        for_end = str_start[(hash[h].v.RH + 1) - 65536L] - 1;
                        if (c <= for_end)
                            do {
                                put_byte(str_pool[c], stderr);
                            }
                            while (c++ < for_end);
                    }
                    {
                        putc('|', stderr);
                        putc('\n', stderr);
                    }
                }
            }
            while (h++ < for_end);
    }
}

void print_file_line(void)
{
    integer level = in_open;

    while ((level > 0) && (full_source_filename_stack[level] == 0))
        level--;
    if (level == 0)
        print_nl(65544L /*"! " */ );
    else {

        print_nl(65622L /*"" */ );
        print(full_source_filename_stack[level]);
        print(58 /*":" */ );
        if (level == in_open)
            print_int(line);
        else
            print_int(line_stack[level + 1]);
        print(65589L /*": " */ );
    }
}
/*:1660*/


void zprint_two(integer n)
{
    print_two_regmem n = abs(n) % 100;
    print_char(48 /*"0" */  + (n / 10));
    print_char(48 /*"0" */  + (n % 10));
}

void zprint_hex(integer n)
{
    print_hex_regmem unsigned char k;
    k = 0;
    print_char(34 /*""" */ );
    do {
        dig[k] = n % 16;
        n = n / 16;
        k++;
    } while (!(n == 0));
    print_the_digs(k);
}

void zprint_roman_int(integer n)
{
    print_roman_int_regmem pool_pointer j, k;
    nonnegative_integer u, v;
    j = str_start[(65542L /*"m2d5c2l5x2v5i" */ ) - 65536L];
    v = 1000;
    while (true) {

        while (n >= v) {

            print_char(str_pool[j]);
            n = n - v;
        }
        if (n <= 0)
            return;
        k = j + 2;
        u = v / (str_pool[k - 1] - 48);
        if (str_pool[k - 1] == 50 /*"2" */ ) {
            k = k + 2;
            u = u / (str_pool[k - 1] - 48);
        }
        if (n + u >= v) {
            print_char(str_pool[k]);
            n = n + u;
        } else {

            j = j + 2;
            v = v / (str_pool[j - 1] - 48);
        }
    }
}

void print_current_string(void)
{
    print_current_string_regmem pool_pointer j;
    j = str_start[(str_ptr) - 65536L];
    while (j < pool_ptr) {

        print_char(str_pool[j]);
        j++;
    }
}

void zprint_scaled(scaled s)
{
    print_scaled_regmem scaled delta;
    if (s < 0) {
        print_char(45 /*"-" */ );
        s = -(integer) s;
    }
    print_int(s / 65536L);
    print_char(46 /*"." */ );
    s = 10 * (s % 65536L) + 5;
    delta = 10;
    do {
        if (delta > 65536L)
            s = s - 17232;
        print_char(48 /*"0" */  + (s / 65536L));
        s = 10 * (s % 65536L);
        delta = delta * 10;
    } while (!(s <= delta));
}
