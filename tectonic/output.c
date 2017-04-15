/* tectonic/output.c -- functions related to outputting messages
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>
#include <tectonic/synctex.h>
#include <tectonic/stubs.h>


void
print_ln(void)
{
    switch (selector) {
    case SELECTOR_TERM_AND_LOG:
	ttstub_output_putc(rust_stdout, '\n');
	ttstub_output_putc(log_file, '\n');
	term_offset = 0;
	file_offset = 0;
        break;
    case SELECTOR_LOG_ONLY:
	ttstub_output_putc(log_file, '\n');
	file_offset = 0;
        break;
    case SELECTOR_TERM_ONLY:
	ttstub_output_putc(rust_stdout, '\n');
	term_offset = 0;
        break;
    case SELECTOR_NO_PRINT:
    case SELECTOR_PSEUDO:
    case SELECTOR_NEW_STRING:
        break;
    default:
	ttstub_output_putc(write_file[selector], '\n');
        break;
    }
}


void
print_raw_char(UTF16_code s, boolean incr_offset)
{
    switch (selector) {
    case SELECTOR_TERM_AND_LOG:
	ttstub_output_putc(rust_stdout, s);
	ttstub_output_putc(log_file, s);
	if (incr_offset) {
	    term_offset++;
	    file_offset++;
	}
	if (term_offset == max_print_line) {
	    ttstub_output_putc(rust_stdout, '\n');
	    term_offset = 0;
	}
	if (file_offset == max_print_line) {
	    ttstub_output_putc(log_file, '\n');
	    file_offset = 0;
        }
        break;
    case SELECTOR_LOG_ONLY:
	ttstub_output_putc(log_file, s);
	if (incr_offset)
	    file_offset++;
	if (file_offset == max_print_line)
	    print_ln();
        break;
    case SELECTOR_TERM_ONLY:
	ttstub_output_putc(rust_stdout, s);
	if (incr_offset)
	    term_offset++;
	if (term_offset == max_print_line)
	    print_ln();
        break;
    case SELECTOR_NO_PRINT:
        break;
    case SELECTOR_PSEUDO:
        if (tally < trick_count)
            trick_buf[tally % error_line] = s;
        break;
    case SELECTOR_NEW_STRING:
	if (pool_ptr < pool_size) {
	    str_pool[pool_ptr] = s;
	    pool_ptr++;
	}
        break;
    default:
	ttstub_output_putc(write_file[selector], s);
        break;
    }
    tally++;
}


void
print_char(integer s)
{
    
    small_number l;

    if ((selector > SELECTOR_PSEUDO) && (!doing_special)) {
        if (s >= 0x10000) {
            print_raw_char(55296L + (s - 0x10000) / 1024, true);
            print_raw_char(56320L + (s - 0x10000) % 1024, true);
        } else
            print_raw_char(s, true);
        return;
    }

    if ( /*252: */ s == INTPAR(new_line_char) /*:252 */ ) {
        if (selector < SELECTOR_PSEUDO) {
            print_ln();
            return;
        }
    }

    if (s < 32 && !doing_special) {
        print_raw_char(94 /*"^" */ , true);
        print_raw_char(94 /*"^" */ , true);
        print_raw_char(s + 64, true);
    } else if (s < 127) {
        print_raw_char(s, true);
    } else if ((s == 127)) {
        if (!doing_special) {
            print_raw_char(94 /*"^" */ , true);
            print_raw_char(94 /*"^" */ , true);
            print_raw_char(63 /*"?" */ , true);
        } else {
            print_raw_char(s, true);
	}
    } else if (s < 160 && !doing_special) {
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
        } else if (s < 0x10000) {
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


void
print(integer s)
{
    
    pool_pointer j;
    integer nl;

    if (s >= str_ptr)
        s = S(___/*"???"*/);
    else if (s < BIGGEST_CHAR) {
        if (s < 0)
            s = S(___/*"???"*/);
        else {
            if (selector > SELECTOR_PSEUDO) {
                print_char(s);
                return;
            }

            if (( /*252: */ s == INTPAR(new_line_char) /*:252 */ )) {
                if (selector < SELECTOR_PSEUDO) {
                    print_ln();
                    return;
                }
            }

            nl = INTPAR(new_line_char);
            INTPAR(new_line_char) = -1;
            print_char(s);
            INTPAR(new_line_char) = nl;
            return;
        }
    }

    j = str_start[(s) - 0x10000];

    while (j < str_start[(s + 1) - 0x10000]) {
        if ((str_pool[j] >= 55296L) && (str_pool[j] <= 56319L) && (j + 1 < str_start[(s + 1) - 0x10000])
            && (str_pool[j + 1] >= 56320L) && (str_pool[j + 1] <= 57343L)) {
            print_char(0x10000 + (str_pool[j] - 55296L) * 1024 + str_pool[j + 1] - 56320L);
            j += 2;
        } else {
            print_char(str_pool[j]);
            j++;
        }
    }
}


void
print_nl(str_number s)
{
    if (((term_offset > 0) && (odd(selector))) || ((file_offset > 0) && (selector >= SELECTOR_LOG_ONLY)))
        print_ln();
    print(s);
}


void
print_esc(str_number s)
{
    
    integer c = INTPAR(escape_char) /*:251 */ ;

    if (c >= 0 && c <= BIGGEST_USV)
	print_char(c);
    print(s);
}


static void
print_the_digs(eight_bits k)
{
    while (k > 0) {
        k--;
        if (dig[k] < 10)
            print_char(48 /*"0" */  + dig[k]);
        else
            print_char(55 /*"A" -10 */  + dig[k]);
    }
}


void
print_int(integer n)
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


void
print_cs(integer p)
{
    

    if (p < HASH_BASE) {
        if (p >= SINGLE_BASE) {
            if (p == NULL_CS) {
                print_esc(S(csname));
                print_esc(S(endcsname));
                print_char(32 /*" " */ );
            } else {
                print_esc(p - 1114113L);
                if (CAT_CODE(p - 1114113L) == LETTER)
                    print_char(32 /*" " */ );
            }
        } else if (p < ACTIVE_BASE)
            print_esc(S(IMPOSSIBLE_));
        else
            print_char(p - 1);
    } else if (((p >= UNDEFINED_CONTROL_SEQUENCE) && (p <= EQTB_SIZE)) || (p > eqtb_top)) {
        print_esc(S(IMPOSSIBLE_));
    } else if ((hash[p].v.RH >= str_ptr)) {
        print_esc(S(NONEXISTENT_));
    } else {
        print_esc(hash[p].v.RH);
        print_char(32 /*" " */ );
    }
}


void
sprint_cs(int32_t p)
{
    if (p < HASH_BASE) {
        if (p < SINGLE_BASE)
            print_char(p - 1);
        else if (p < NULL_CS)
            print_esc(p - 1114113L);
        else {
            print_esc(S(csname));
            print_esc(S(endcsname));
        }
    } else
        print_esc(hash[p].v.RH);
}


void
print_file_name(integer n, integer a, integer e)
{
    boolean must_quote = false;
    integer quote_char = 0;
    pool_pointer j;

    if (a != 0) {
        j = str_start[(a) - 0x10000];
        while (((!must_quote) || (quote_char == 0)) && (j < str_start[(a + 1) - 0x10000])) {
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
        j = str_start[(n) - 0x10000];
        while (((!must_quote) || (quote_char == 0)) && (j < str_start[(n + 1) - 0x10000])) {
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
        j = str_start[(e) - 0x10000];
        while (((!must_quote) || (quote_char == 0)) && (j < str_start[(e + 1) - 0x10000])) {
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
        j = str_start[(a) - 0x10000];
        for_end = str_start[(a + 1) - 0x10000] - 1;
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
        j = str_start[(n) - 0x10000];
        for_end = str_start[(n + 1) - 0x10000] - 1;
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
        j = str_start[(e) - 0x10000];
        for_end = str_start[(e + 1) - 0x10000] - 1;
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


void
print_size(integer s)
{
    if (s == TEXT_SIZE)
        print_esc(S(textfont));
    else if (s == SCRIPT_SIZE)
        print_esc(S(scriptfont));
    else
        print_esc(S(scriptscriptfont));
}


void
print_write_whatsit(str_number s, int32_t p)
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


void
print_native_word(int32_t p)
{
    memory_word *mem = zmem;
    integer i, c, cc;
    integer for_end = mem[p + 4].qqqq.u.B2 - 1;

    for (i = 0; i <= for_end; i++) {
	c = get_native_char(p, i);
	if ((c >= 55296L) && (c <= 56319L)) {
	    if (i < mem[p + 4].qqqq.u.B2 - 1) {
		cc = get_native_char(p, i + 1);
		if ((cc >= 56320L) && (cc <= 57343L)) {
		    c = 0x10000 + (c - 55296L) * 1024 + (cc - 56320L);
		    print_char(c);
		    i++;
		} else
		    print(46 /*"." */ );
	    } else
		print(46 /*"." */ );
	} else
	    print_char(c);
    }
}


void
print_sa_num(int32_t q)
{
    memory_word *mem = zmem;
    int32_t n;

    if (mem[q].hh.u.B0 < DIMEN_VAL_LIMIT)
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


void
print_file_line(void)
{
    integer level = in_open;

    while ((level > 0) && (full_source_filename_stack[level] == 0))
        level--;

    if (level == 0)
        print_nl(S(__/*"! "*/));
    else {
        print_nl(S());
        print(full_source_filename_stack[level]);
        print(58 /*":" */ );
        if (level == in_open)
            print_int(line);
        else
            print_int(line_stack[level + 1]);
        print(S(___Z3/*": "*/));
    }
}
/*:1660*/


void
print_two(integer n)
{
    n = abs(n) % 100;
    print_char(48 /*"0" */  + (n / 10));
    print_char(48 /*"0" */  + (n % 10));
}


void
print_hex(integer n)
{
    unsigned char k = 0;

    print_char(34 /*""" */ );

    do {
        dig[k] = n % 16;
        n = n / 16;
        k++;
    } while (n != 0);

    print_the_digs(k);
}


void
print_roman_int(integer n)
{
    pool_pointer j, k;
    nonnegative_integer u, v;

    j = str_start[(S(m2d5c2l5x2v5i)) - 0x10000];
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


void
print_current_string(void)
{
    pool_pointer j = str_start[str_ptr - 0x10000];

    while (j < pool_ptr) {
        print_char(str_pool[j]);
        j++;
    }
}


void
print_scaled(scaled s)
{
    scaled delta;

    if (s < 0) {
        print_char(45 /*"-" */ );
        s = -(integer) s;
    }

    print_int(s / 0x10000);
    print_char(46 /*"." */ );
    s = 10 * (s % 0x10000) + 5;
    delta = 10;

    do {
        if (delta > 0x10000)
            s = s - 17232;
        print_char(48 /*"0" */  + (s / 0x10000));
        s = 10 * (s % 0x10000);
        delta = delta * 10;
    } while (s > delta);
}
