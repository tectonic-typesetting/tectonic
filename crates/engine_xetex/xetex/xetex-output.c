/* tectonic/output.c -- functions related to outputting messages
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-synctex.h"
#include "tectonic_bridge_core.h"
#include "xetex_bindings.h"

void
print_write_whatsit(const char* s, int32_t p)
{

    print_esc_cstr(s);

    if (mem[p + 1].b32.s0 < 16)
        print_int(mem[p + 1].b32.s0);
    else if (mem[p + 1].b32.s0 == 16)
        print_char('*');
    else
        print_char('-');
}


void
print_native_word(int32_t p)
{
    int32_t i, c, cc;
    int32_t for_end = mem[p + 4].b16.s1 - 1;

    for (i = 0; i <= for_end; i++) {
        c = NATIVE_NODE_text(p)[i];
        if ((c >= 0xD800) && (c < 0xDC00)) {
            if (i < mem[p + 4].b16.s1 - 1) {
                cc = NATIVE_NODE_text(p)[i + 1];
                if ((cc >= 0xDC00) && (cc < 0xE000)) {
                    c = 0x10000 + (c - 0xD800) * 1024 + (cc - 0xDC00);
                    print_char(c);
                    i++;
                } else
                    print('.');
            } else
                print('.');
        } else
            print_char(c);
    }
}


void
print_sa_num(int32_t q)
{
    int32_t n;

    if (mem[q].b16.s1 < DIMEN_VAL_LIMIT)
        n = mem[q + 1].b32.s1;
    else {
        n = mem[q].b16.s1 % 64;
        q = LLIST_link(q);
        n = n + 64 * mem[q].b16.s1;
        q = LLIST_link(q);
        n = n + 64 * 64 * (mem[q].b16.s1 + 64 * mem[mem[q].b32.s1].b16.s1);
    }

    print_int(n);
}

void
print_two(int32_t n)
{
    n = abs(n) % 100;
    print_char('0' + (n / 10));
    print_char('0' + (n % 10));
}


void
print_hex(int32_t n)
{
    unsigned char k = 0;

    print_char('"');

    do {
        set_dig(k, n % 16);
        n = n / 16;
        k++;
    } while (n != 0);

    print_the_digs(k);
}


void
print_roman_int(int32_t n)
{
    int32_t u, v;

    const char* roman_data = "m2d5c2l5x2v5i";
    unsigned char j = 0;
    unsigned char k = 0;
    v = 1000;

    while (true) {
        while (n >= v) {
            print_char(roman_data[j]);
            n = n - v;
        }

        if (n <= 0)
            return;

        k = j + 2;
        u = v / (roman_data[k - 1] - '0');
        if (roman_data[k - 1] == '2' ) {
            k = k + 2;
            u = u / (roman_data[k - 1] - '0');
        }

        if (n + u >= v) {
            print_char(roman_data[k]);
            n = n + u;
        } else {
            j = j + 2;
            v = v / (roman_data[j - 1] - '0');
        }
    }
}


void
print_current_string(void)
{
    pool_pointer j = str_start(str_ptr() - 0x10000);

    while (j < pool_ptr()) {
        print_char(str_pool(j));
        j++;
    }
}


void
print_scaled(scaled_t s)
{
    scaled_t delta;

    if (s < 0) {
        print_char('-');
        s = -(int32_t) s;
    }

    print_int(s / 0x10000);
    print_char('.');
    s = 10 * (s % 0x10000) + 5;
    delta = 10;

    do {
        if (delta > 0x10000)
            s = s + 0x8000 - 50000;
        print_char('0'  + (s / 0x10000));
        s = 10 * (s % 0x10000);
        delta = delta * 10;
    } while (s > delta);
}


void
print_ucs_code(UnicodeScalar n)
{
    unsigned char k = 0;

    print_cstr("U+");

    do {
        set_dig(k, n % 16);
        n = n / 16;
        k++;
    } while (n != 0);

    while (k < 4) {
        set_dig(k, 0);
        k++;
    }

    print_the_digs(k);
}
