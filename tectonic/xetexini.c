#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>
#include <tectonic/synctex.h>
#include <tectonic/stubs.h>
#include <setjmp.h>

/* Read and write dump files.  As distributed, these files are
   architecture dependent; specifically, BigEndian and LittleEndian
   architectures produce different files.  These routines always output
   BigEndian files.  This still does not guarantee them to be
   architecture-independent, because it is possible to make a format
   that dumps a glue ratio, i.e., a floating-point number.  Fortunately,
   none of the standard formats do that.  */

#if !defined (WORDS_BIGENDIAN)

/* This macro is always invoked as a statement.  It assumes a variable
   `temp'.  */

#define SWAP(x, y) temp = (x); (x) = (y); (y) = temp


/* Make the NITEMS items pointed at by P, each of size SIZE, be the
   opposite-endianness of whatever they are now.  */

static void
swap_items (char *p, int nitems, int size)
{
    char temp;

    switch (size) {
    case 16:
        while (nitems--) {
            SWAP (p[0], p[15]);
            SWAP (p[1], p[14]);
            SWAP (p[2], p[13]);
            SWAP (p[3], p[12]);
            SWAP (p[4], p[11]);
            SWAP (p[5], p[10]);
            SWAP (p[6], p[9]);
            SWAP (p[7], p[8]);
            p += size;
        }
        break;
    case 8:
        while (nitems--) {
            SWAP (p[0], p[7]);
            SWAP (p[1], p[6]);
            SWAP (p[2], p[5]);
            SWAP (p[3], p[4]);
            p += size;
        }
        break;
    case 4:
        while (nitems--) {
            SWAP (p[0], p[3]);
            SWAP (p[1], p[2]);
            p += size;
        }
        break;
    case 2:
        while (nitems--) {
            SWAP (p[0], p[1]);
            p += size;
        }
        break;
    case 1:
        break; /* Nothing to do. */
    default:
	_tt_abort("can't swap a %d-byte item for (un)dumping", size);
    }
}
#else  /* not WORDS_BIGENDIAN */
#define swap_items(a,b,c) do {} while(0)
#endif


/* Here we write NITEMS items, each item being ITEM_SIZE bytes long.
   The pointer to the stuff to write is P, and we write to the file
   OUT_FILE.  */

static void
do_dump (char *p, int item_size, int nitems, rust_output_handle_t out_file)
{
    swap_items (p, nitems, item_size);

    if (ttstub_output_write (out_file, p, item_size * nitems) != item_size * nitems)
        _tt_abort ("could not write %d %d-byte item(s) to %s",
                   nitems, item_size, name_of_file+1);

    /* Have to restore the old contents of memory, since some of it might
       get used again.  */
    swap_items (p, nitems, item_size);
}


/* Here is the dual of the writing routine.  */

static void
do_undump (char *p, int item_size, int nitems, rust_input_handle_t in_file)
{
    if (ttstub_input_read (in_file, p, item_size * nitems) != item_size * nitems)
        _tt_abort("could not undump %d %d-byte item(s) from %s",
                  nitems, item_size, name_of_file+1);

    swap_items (p, nitems, item_size);
}


#define	dump_things(base, len)                                          \
    do_dump ((char *) &(base), sizeof (base), (int) (len), fmt_out)
#define	undump_things(base, len)                                        \
    do_undump ((char *) &(base), sizeof (base), (int) (len), fmt_in)

/* Like do_undump, but check each value against LOW and HIGH.  The
   slowdown isn't significant, and this improves the chances of
   detecting incompatible format files.  In fact, Knuth himself noted
   this problem with Web2c some years ago, so it seems worth fixing.  We
   can't make this a subroutine because then we lose the type of BASE.  */
#define undump_checked_things(low, high, base, len)			\
    do {                                                                \
        unsigned i;                                                     \
        undump_things (base, len);                                      \
        for (i = 0; i < (len); i++) {                                   \
            if ((&(base))[i] < (low) || (&(base))[i] > (high)) {        \
                _tt_abort ("item %u (=%" PRIdPTR ") of .fmt array at %" PRIxPTR \
                           " <%" PRIdPTR " or >%" PRIdPTR,              \
                           i, (uintptr_t) (&(base))[i], (uintptr_t) &(base), \
                           (uintptr_t) low, (uintptr_t) high);          \
            }                                                           \
        }                                                               \
    } while (0)

/* Like undump_checked_things, but only check the upper value. We use
   this when the base type is unsigned, and thus all the values will be
   greater than zero by definition.  */
#define undump_upper_check_things(high, base, len)                      \
    do {                                                                \
        unsigned i;                                                     \
        undump_things (base, len);                                      \
        for (i = 0; i < (len); i++) {                                   \
            if ((&(base))[i] > (high)) {              			\
                _tt_abort ("Item %u (=%" PRIdPTR ") of .fmt array at %" PRIxPTR \
                           " >%" PRIdPTR,                               \
                           i, (uintptr_t) (&(base))[i], (uintptr_t) &(base), \
                           (uintptr_t) high);                           \
            }                                                           \
        }                                                               \
    } while (0)


/* Since dump_things is a macro with a sizeof(), these all work: */
#define dump_wd(x) dump_things(x, 1)
#define dump_hh(x) dump_things(x, 1)
#define dump_qqqq(x) dump_things(x, 1)
#define undump_wd(x) undump_things(x, 1)
#define undump_hh(x) undump_things(x, 1)
#define	undump_qqqq(x) undump_things(x, 1)

/* `dump_int' is called with constant integers, so we put them into a
   variable first.  */
#define dump_int(x)                             \
    do {                                        \
        integer x_val = (x);                    \
        dump_things(x_val, 1);                  \
    } while (0)

#define	undump_int(x) undump_things(x, 1)


#define hash_offset 514
#define engine_name "xetex"
#define sup_max_strings 2097151L /* magic constant, origin unclear */
#define sup_font_mem_size 147483647L /* magic constant, origin unclear */
#define sup_pool_size 40000000L
#define sup_string_vacancies (sup_pool_size - 23000)
#define sup_hash_extra sup_max_strings
#define inf_expand_depth 10
#define sup_expand_depth 10000000L


/*:134*//*135: */

static void
sort_avail(void)
{
    memory_word *mem = zmem;
    int32_t p, q, r;
    int32_t old_rover;

    p = get_node(0x40000000);
    p = mem[rover + 1].hh.v.RH;
    mem[rover + 1].hh.v.RH = MAX_HALFWORD;
    old_rover = rover;

    /*136: */

    while (p != old_rover) {
        if (p < rover) {
            q = p;
            p = mem[q + 1].hh.v.RH;
            mem[q + 1].hh.v.RH = rover;
            rover = q;
        } else {
            q = rover;
            while (mem[q + 1].hh.v.RH < p)
                q = mem[q + 1].hh.v.RH;
            r = mem[p + 1].hh.v.RH;
            mem[p + 1].hh.v.RH = mem[q + 1].hh.v.RH;
            mem[q + 1].hh.v.RH = p;
            p = r;
        }
    }

    p = rover;

    while (mem[p + 1].hh.v.RH != MAX_HALFWORD) {
        mem[mem[p + 1].hh.v.RH + 1].hh.v.LH = p;
        p = mem[p + 1].hh.v.RH;
    }

    mem[p + 1].hh.v.RH = rover;
    mem[rover + 1].hh.v.LH = p;
}

/*:271*//*276: */

static void
primitive(str_number s, uint16_t c, int32_t o)
{
    CACHE_THE_EQTB;
    pool_pointer k;
    integer j, prim_val;
    small_number l;

    if (s < 256) {
        cur_val = s + SINGLE_BASE;
        prim_val = s;
    } else {
        k = str_start[s - 65536L];
        l = str_start[s + 1 - 65536L] - k;

        if (first + l > buf_size + 1)
            overflow(S(buffer_size), buf_size);

	for (j = 0; j <= l - 1; j++)
	    buffer[first + j] = str_pool[k + j];

        cur_val = id_lookup(first, l);
	str_ptr--;
	pool_ptr = str_start[str_ptr - 65536L];
        hash[cur_val].v.RH = s;
        prim_val = prim_lookup(s);
    }

    eqtb[cur_val].hh.u.B1 = LEVEL_ONE;
    eqtb[cur_val].hh.u.B0 = c;
    eqtb[cur_val].hh.v.RH = o;
    prim_eqtb[prim_val].hh.u.B1 = LEVEL_ONE;
    prim_eqtb[prim_val].hh.u.B0 = c;
    prim_eqtb[prim_val].hh.v.RH = o;
}

/*:925*//*977: */

trie_opcode new_trie_op(small_number d, small_number n, trie_opcode v)
{
    register trie_opcode Result;
    integer h;
    trie_opcode u;
    integer l;
    h = abs(n + 313 * d + 361 * v + 1009 * cur_lang) % (trie_op_size - neg_trie_op_size) + neg_trie_op_size;
    while (true) {

        l = trie_op_hash[h];
        if (l == 0) {
            if (trie_op_ptr == trie_op_size)
                overflow(S(pattern_memory_ops), trie_op_size);
            u = trie_used[cur_lang];
            if (u == max_trie_op)
                overflow(S(pattern_memory_ops_per_langu/*age*/), max_trie_op - min_trie_op);
            trie_op_ptr++;
            u++;
            trie_used[cur_lang] = u;
            if (u > max_op_used)
                max_op_used = u;
            hyf_distance[trie_op_ptr] = d;
            hyf_num[trie_op_ptr] = n;
            hyf_next[trie_op_ptr] = v;
            trie_op_lang[trie_op_ptr] = cur_lang;
            trie_op_hash[h] = trie_op_ptr;
            trie_op_val[trie_op_ptr] = u;
            Result = u;
            return Result;
        }
        if ((hyf_distance[l] == d) && (hyf_num[l] == n) && (hyf_next[l] == v) && (trie_op_lang[l] == cur_lang)) {
            Result = trie_op_val[l];
            return Result;
        }
        if (h > -(integer) trie_op_size)
            h--;
        else
            h = trie_op_size;
    }
    return Result;
}

trie_pointer trie_node(trie_pointer p)
{
    register trie_pointer Result;
    trie_pointer h;
    trie_pointer q;
    h = abs(trie_c[p] + 1009 * trie_o[p] + 2718 * trie_l[p] + 3142 * trie_r[p]) % trie_size;
    while (true) {

        q = trie_hash[h];
        if (q == 0) {
            trie_hash[h] = p;
            Result = p;
            return Result;
        }
        if ((trie_c[q] == trie_c[p]) && (trie_o[q] == trie_o[p]) && (trie_l[q] == trie_l[p])
            && (trie_r[q] == trie_r[p])) {
            Result = q;
            return Result;
        }
        if (h > 0)
            h--;
        else
            h = trie_size;
    }
    return Result;
}

trie_pointer compress_trie(trie_pointer p)
{
    register trie_pointer Result;
    if (p == 0)
        Result = 0;
    else {

        trie_l[p] = compress_trie(trie_l[p]);
        trie_r[p] = compress_trie(trie_r[p]);
        Result = trie_node(p);
    }
    return Result;
}

void first_fit(trie_pointer p)
{
    trie_pointer h;
    trie_pointer z;
    trie_pointer q;
    UTF16_code c;
    trie_pointer l, r;
    integer /*too_big_char */ ll;
    c = trie_c[p];
    z = trie_min[c];
    while (true) {

        h = z - c;
        if (trie_max < h + max_hyph_char) {
            if (trie_size <= h + max_hyph_char)
                overflow(S(pattern_memory), trie_size);
            do {
                trie_max++;
                trie_taken[trie_max] = false;
                trie_trl[trie_max] = trie_max + 1;
                trie_tro[trie_max] = trie_max - 1;
            } while (!(trie_max == h + max_hyph_char));
        }
        if (trie_taken[h])
            goto not_found;
        q = trie_r[p];
        while (q > 0) {

            if (trie_trl[h + trie_c[q]] == 0)
                goto not_found;
            q = trie_r[q];
        }
        goto found;
    not_found:                        /*not_found */ z = trie_trl[z];
    }
found:                        /*found *//*991: */ trie_taken[h] = true;
    trie_hash[p] = h;
    q = p;
    do {
        z = h + trie_c[q];
        l = trie_tro[z];
        r = trie_trl[z];
        trie_tro[r] = l;
        trie_trl[l] = r;
        trie_trl[z] = 0;
        if (l < max_hyph_char) {
            if (z < max_hyph_char)
                ll = z;
            else
                ll = max_hyph_char;
            do {
                trie_min[l] = r;
                l++;
            } while (!(l == ll));
        }
        q = trie_r[q];
    } while (!(q == 0 /*:991 */ ));
}

void trie_pack(trie_pointer p)
{
    trie_pointer q;
    do {
        q = trie_l[p];
        if ((q > 0) && (trie_hash[q] == 0)) {
            first_fit(q);
            trie_pack(q);
        }
        p = trie_r[p];
    } while (!(p == 0));
}

void trie_fix(trie_pointer p)
{
    trie_pointer q;
    UTF16_code c;
    trie_pointer z;
    z = trie_hash[p];
    do {
        q = trie_l[p];
        c = trie_c[p];
        trie_trl[z + c] = trie_hash[q];
        trie_trc[z + c] = c;
        trie_tro[z + c] = trie_o[p];
        if (q > 0)
            trie_fix(q);
        p = trie_r[p];
    } while (!(p == 0));
}

static void
new_patterns(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    short /*hyphenatable_length_limit 1 */ k, l;
    boolean digit_sensed;
    trie_opcode v;
    trie_pointer p, q;
    boolean first_child;
    UTF16_code c;

    if (trie_not_ready) {
        if (INTPAR(language) <= 0)
            cur_lang = 0;
        else if (INTPAR(language) > BIGGEST_LANG)
            cur_lang = 0;
        else
            cur_lang = INTPAR(language);

        scan_left_brace();
        k = 0;
        hyf[0] = 0;
        digit_sensed = false;

        while (true) {
            get_x_token();

            switch (cur_cmd) {
            case LETTER:
            case OTHER_CHAR:
                if (digit_sensed || cur_chr < 48 /*"0" */  || cur_chr > 57 /*"9" */ ) {
                    if (cur_chr == 46 /*"." */ ) {
                        cur_chr = 0;
                    } else {
                        cur_chr = LC_CODE(cur_chr);

                        if (cur_chr == 0) {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Nonletter));
                            help_ptr = 1;
                            help_line[0] = S(_See_Appendix_H__);
                            error();
                        }
                    }

                    if (cur_chr > max_hyph_char)
                        max_hyph_char = cur_chr;

                    if (k < max_hyphenatable_length()) {
                        k++;
                        hc[k] = cur_chr;
                        hyf[k] = 0;
                        digit_sensed = false;
                    }
                } else if (k < max_hyphenatable_length()) {
                    hyf[k] = cur_chr - 48;
                    digit_sensed = true;
                }
                break;

            case SPACER:
            case RIGHT_BRACE:
                if (k > 0) { /*998:*/
                    if (hc[1] == 0)
                        hyf[0] = 0;
                    if (hc[k] == 0)
                        hyf[k] = 0;

                    l = k;
                    v = min_trie_op;

                    while (true) {
                        if (hyf[l] != 0)
                            v = new_trie_op(k - l, hyf[l], v);
                        if (l > 0)
                            l--;
                        else
                            break;
                    }

                    q = 0;
                    hc[0] = cur_lang;

                    while (l <= k) {
                        c = hc[l];
                        l++;
                        p = trie_l[q];
                        first_child = true;

                        while (p > 0 && c > trie_c[p]) {
                            q = p;
                            p = trie_r[q];
                            first_child = false;
                        }

                        if (p == 0 || c < trie_c[p]) { /*999:*/
                            if (trie_ptr == trie_size)
                                overflow(S(pattern_memory), trie_size);
                            trie_ptr++;
                            trie_r[trie_ptr] = p;
                            p = trie_ptr;
                            trie_l[p] = 0;
                            if (first_child)
                                trie_l[q] = p;
                            else
                                trie_r[q] = p;
                            trie_c[p] = c;
                            trie_o[p] = min_trie_op;
                        }

                        q = p;
                    }

                    if (trie_o[q] != min_trie_op) {
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(Duplicate_pattern));
                        help_ptr = 1;
                        help_line[0] = S(_See_Appendix_H__);
                        error();
                    }

                    trie_o[q] = v;
                }

                if (cur_cmd == RIGHT_BRACE)
                    goto done;

                k = 0;
                hyf[0] = 0;
                digit_sensed = false;
                break;

            default:
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Bad_));
                print_esc(S(patterns));
                help_ptr = 1;
                help_line[0] = S(_See_Appendix_H__);
                error();
                break;
            }
        }

    done: /*:996*/
        if (INTPAR(saving_hyphs) > 0) { /*1643:*/
            c = cur_lang;
            first_child = false;
            p = 0;

            do {
                q = p;
                p = trie_r[q];
            } while (!(p == 0 || c <= trie_c[p]));

            if (p == 0 || c < trie_c[p]) { /*999:*/
                if (trie_ptr == trie_size)
                    overflow(S(pattern_memory), trie_size);
                trie_ptr++;
                trie_r[trie_ptr] = p;
                p = trie_ptr;
                trie_l[p] = 0;
                if (first_child)
                    trie_l[q] = p;
                else
                    trie_r[q] = p;
                trie_c[p] = c;
                trie_o[p] = min_trie_op;
            }

            q = p;
            p = trie_l[q];
            first_child = true;

            for (c = 0; c <= 255; c++) {
                if (LC_CODE(c) > 0 || (c == 255 && first_child)) {
                    if (p == 0) { /*999:*/
                        if (trie_ptr == trie_size)
                            overflow(S(pattern_memory), trie_size);
                        trie_ptr++;
                        trie_r[trie_ptr] = p;
                        p = trie_ptr;
                        trie_l[p] = 0;
                        if (first_child)
                            trie_l[q] = p;
                        else
                            trie_r[q] = p;
                        trie_c[p] = c;
                        trie_o[p] = min_trie_op;
                    } else {
                        trie_c[p] = c;
                    }

                    trie_o[p] = LC_CODE(c);
                    q = p;
                    p = trie_r[q];
                    first_child = false;
                }
            }

            if (first_child)
                trie_l[q] = 0;
            else
                trie_r[q] = 0; /*:1644*/
        }
    } else {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Too_late_for_));
        print_esc(S(patterns));
        help_ptr = 1;
        help_line[0] = S(All_patterns_must_be_given_b/*efore typesetting begins.*/);
        error();

        mem[mem_top - 12].hh.v.RH = scan_toks(false, false);
        flush_list(def_ref);
    }
}

void init_trie(void)
{
    trie_pointer p;
    integer j, k, t;
    trie_pointer r, s;
    max_hyph_char++;
    op_start[0] = -(integer) min_trie_op;
    {
        register integer for_end;
        j = 1;
        for_end = BIGGEST_LANG;
        if (j <= for_end)
            do
                op_start[j] = op_start[j - 1] + trie_used[j - 1];
            while (j++ < for_end);
    }
    {
        register integer for_end;
        j = 1;
        for_end = trie_op_ptr;
        if (j <= for_end)
            do
                trie_op_hash[j] = op_start[trie_op_lang[j]] + trie_op_val[j];
            while (j++ < for_end);
    }
    {
        register integer for_end;
        j = 1;
        for_end = trie_op_ptr;
        if (j <= for_end)
            do
                while (trie_op_hash[j] > j) {

                    k = trie_op_hash[j];
                    t = hyf_distance[k];
                    hyf_distance[k] = hyf_distance[j];
                    hyf_distance[j] = t;
                    t = hyf_num[k];
                    hyf_num[k] = hyf_num[j];
                    hyf_num[j] = t;
                    t = hyf_next[k];
                    hyf_next[k] = hyf_next[j];
                    hyf_next[j] = t;
                    trie_op_hash[j] = trie_op_hash[k];
                    trie_op_hash[k] = k;
                }
            while (j++ < for_end);
    }
    {
        register integer for_end;
        p = 0;
        for_end = trie_size;
        if (p <= for_end)
            do
                trie_hash[p] = 0;
            while (p++ < for_end);
    }
    trie_r[0] = compress_trie(trie_r[0]);
    trie_l[0] = compress_trie(trie_l[0]);
    {
        register integer for_end;
        p = 0;
        for_end = trie_ptr;
        if (p <= for_end)
            do
                trie_hash[p] = 0;
            while (p++ < for_end);
    }
    {
        register integer for_end;
        p = 0;
        for_end = BIGGEST_CHAR;
        if (p <= for_end)
            do
                trie_min[p] = p + 1;
            while (p++ < for_end);
    }
    trie_trl[0] = 1;
    trie_max = 0 /*:987 */ ;
    if (trie_l[0] != 0) {
        first_fit(trie_l[0]);
        trie_pack(trie_l[0]);
    }
    if (trie_r[0] != 0) {       /*1645: */
        if (trie_l[0] == 0) {
            register integer for_end;
            p = 0;
            for_end = 255;
            if (p <= for_end)
                do
                    trie_min[p] = p + 2;
                while (p++ < for_end);
        }
        first_fit(trie_r[0]);
        trie_pack(trie_r[0]);
        hyph_start = trie_hash[trie_r[0]];
    }
    if (trie_max == 0) {
        {
            register integer for_end;
            r = 0;
            for_end = max_hyph_char;
            if (r <= for_end)
                do {
                    trie_trl[r] = 0;
                    trie_tro[r] = min_trie_op;
                    trie_trc[r] = 0;
                }
                while (r++ < for_end);
        }
        trie_max = max_hyph_char;
    } else {

        if (trie_r[0] > 0)
            trie_fix(trie_r[0]);
        if (trie_l[0] > 0)
            trie_fix(trie_l[0]);
        r = 0;
        do {
            s = trie_trl[r];
            {
                trie_trl[r] = 0;
                trie_tro[r] = min_trie_op;
                trie_trc[r] = 0;
            }
            r = s;
        } while (!(r > trie_max));
    }
    trie_trc[0] = 63 /*"?" */ ;
    trie_not_ready = false;
}

/*:1001*/

static void
new_hyph_exceptions(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    short /*hyphenatable_length_limit 1 */ n;
    short /*hyphenatable_length_limit 1 */ j;
    hyph_pointer h;
    str_number k;
    int32_t p;
    int32_t q;
    str_number s;
    pool_pointer u, v;

    scan_left_brace();

    if (INTPAR(language) <= 0)
        cur_lang = 0;
    else if (INTPAR(language) > BIGGEST_LANG)
        cur_lang = 0;
    else
        cur_lang = INTPAR(language);

    if (trie_not_ready) {
        hyph_index = 0;
        goto not_found1;
    }

    if (trie_trc[hyph_start + cur_lang] != cur_lang)
        hyph_index = 0;
    else
        hyph_index = trie_trl[hyph_start + cur_lang];

not_found1: /*970:*/
    n = 0;
    p = MIN_HALFWORD;

    while (true) {
        get_x_token();

    reswitch:
        switch (cur_cmd) {
        case LETTER:
        case OTHER_CHAR:
        case CHAR_GIVEN:
            if (cur_chr == 45 /*"-" */ ) { /*973:*/
                if (n < max_hyphenatable_length()) {
                    q = get_avail();
                    mem[q].hh.v.RH = p;
                    mem[q].hh.v.LH = n;
                    p = q;
                }
            } else {
                if (hyph_index == 0 || cur_chr > 255)
                    hc[0] = LC_CODE(cur_chr);
                else if (trie_trc[hyph_index + cur_chr] != cur_chr)
                    hc[0] = 0;
                else
                    hc[0] = trie_tro[hyph_index + cur_chr];

                if (hc[0] == 0) {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Not_a_letter));
                    help_ptr = 2;
                    help_line[1] = S(Letters_in__hyphenation_word/*s must have \lccode>0.*/);
                    help_line[0] = S(Proceed__I_ll_ignore_the_cha/*racter I just read.*/);
                    error();
                } else if (n < max_hyphenatable_length()) {
                    n++;

                    if (hc[0] < 65536L) {
                        hc[n] = hc[0];
                    } else {
                        hc[n] = (hc[0] - 65536L) / 1024 + 55296L;
                        n++;
                        hc[n] = hc[0] % 1024 + 56320L;
                    }
                }
            }
            break;

        case CHAR_NUM:
            scan_char_num();
            cur_chr = cur_val;
            cur_cmd = CHAR_GIVEN;
            goto reswitch;
            break;

        case SPACER:
        case RIGHT_BRACE:
            if (n > 1) { /*974:*/
                n++;
                hc[n] = cur_lang;
                if (pool_ptr + n > pool_size)
                    overflow(S(pool_size), pool_size - init_pool_ptr);
                h = 0;

                for (j = 1; j <= n; j++) {
                    h = (h + h + hc[j]) % HYPH_PRIME;
                    str_pool[pool_ptr] = hc[j];
                    pool_ptr++;
                }

                s = make_string();

                if (hyph_next <= HYPH_PRIME) {
                    while (hyph_next > 0 && hyph_word[hyph_next - 1] > 0)
                        hyph_next--;
                }

                if (hyph_count == hyph_size || hyph_next == 0)
                    overflow(S(exception_dictionary), hyph_size);

                hyph_count++;

                while (hyph_word[h] != 0) {
                    k = hyph_word[h];
                    if (length(k) != length(s))
                        goto not_found;

                    u = str_start[(k) - 65536L];
                    v = str_start[(s) - 65536L];

                    do {
                        if (str_pool[u] != str_pool[v])
                            goto not_found;
                        u++;
                        v++;
                    } while (u != str_start[(k + 1) - 65536L]);

                    str_ptr--;
                    pool_ptr = str_start[str_ptr - 65536L];
                    s = hyph_word[h];
                    hyph_count--;
                    goto found;

                not_found: /*:976*/
                    if (hyph_link[h] == 0) {
                        hyph_link[h] = hyph_next;
                        if (hyph_next >= hyph_size)
                            hyph_next = HYPH_PRIME;
                        if (hyph_next > HYPH_PRIME)
                            hyph_next++;
                    }
                    h = hyph_link[h] - 1;
                }

            found:
                hyph_word[h] = s;
                hyph_list[h] = p; /*:975*/
            }

            if (cur_cmd == RIGHT_BRACE)
                return;

            n = 0;
            p = MIN_HALFWORD;
            break;

        default:
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Improper_));
            print_esc(S(hyphenation));
            print(S(_will_be_flushed));
            help_ptr = 2;
            help_line[1] = S(Hyphenation_exceptions_must_/*contain only letters*/);
            help_line[0] = S(and_hyphens__But_continue__I/*'ll forgive and forget.*/);
            error();
            break;
        }
    }
}


void
prefixed_command(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    small_number a;
    internal_font_number f;
    int32_t j;
    font_index k;
    int32_t p, q;
    integer n;
    boolean e;

    a = 0;

    while (cur_cmd == PREFIX) {
        if (!odd(a / cur_chr))
            a = a + cur_chr;

        do {
            get_x_token();
        } while (cur_cmd == SPACER || cur_cmd == RELAX);

        if (cur_cmd <= MAX_NON_PREFIXED_COMMAND) { /*1247:*/
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(You_can_t_use_a_prefix_with_/*`*/));
            print_cmd_chr(cur_cmd, cur_chr);
            print_char(39 /*"'" */ );
            help_ptr = 1;
            help_line[0] = S(I_ll_pretend_you_didn_t_say__Z1/*" \long or \outer or \global or \protected."*/);
            back_error();
            return;
        }

        if (INTPAR(tracing_commands) > 2) {
            show_cur_cmd_chr();
        }
    }

    if (a >= 8) {
        j = PROTECTED_TOKEN;
        a = a - 8;
    } else {
        j = 0;
    }

    if (cur_cmd != DEF && (a % 4 != 0 || j != 0)) {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(You_can_t_use__));
        print_esc(S(long));
        print(S(__or__/*"' or `"*/));
        print_esc(S(outer));
        help_ptr = 1;
        help_line[0] = S(I_ll_pretend_you_didn_t_say__Z3/*"\long or \outer or \protected here."*/);
        print(S(__or__/*"' or `"*/));
        print_esc(S(protected));
        print(S(__with__));
        print_cmd_chr(cur_cmd, cur_chr);
        print_char(39 /*"'" */ );
        error();
    }

    if (INTPAR(global_defs) != 0) {
        if (INTPAR(global_defs) < 0) {
            if (a >= 4)
                a = a - 4;
        } else {
            if (a < 4)
                a = a + 4;
        }
    }

    switch (cur_cmd) { /*1252:*/
    case SET_FONT:
        if (a >= 4)
            geq_define(CUR_FONT_LOC, DATA, cur_chr);
        else
            eq_define(CUR_FONT_LOC, DATA, cur_chr);
        break;

    case DEF:
        if (odd(cur_chr) && a < 4 && INTPAR(global_defs) >= 0)
            a = a + 4;

        e = (cur_chr >= 2);
        get_r_token();
        p = cur_cs;
        q = scan_toks(true, e);

        if (j != 0) {
            q = get_avail();
            mem[q].hh.v.LH = j;
            mem[q].hh.v.RH = mem[def_ref].hh.v.RH;
            mem[def_ref].hh.v.RH = q;
        }

        if (a >= 4)
            geq_define(p, CALL + (a % 4), def_ref);
        else
            eq_define(p, CALL + (a % 4), def_ref);
        break;

    case LET:
        n = cur_chr;
        get_r_token();
        p = cur_cs;

        if (n == NORMAL) {
            do {
                get_token();
            } while (cur_cmd == SPACER);

            if (cur_tok == (OTHER_TOKEN + 61 /*"=" */ )) {
                get_token();
                if (cur_cmd == SPACER)
                    get_token();
            }
        } else {
            get_token();
            q = cur_tok;
            get_token();
            back_input();
            cur_tok = q;
            back_input();
        }

        if (cur_cmd >= CALL) {
            mem[cur_chr].hh.v.LH++;
        } else if (cur_cmd == REGISTER || cur_cmd == TOKS_REGISTER) {
            if (cur_chr < 0 || cur_chr > 19) /* 19 = lo_mem_stat_max, I think */
                mem[cur_chr + 1].hh.v.LH++;
        }

        if (a >= 4)
            geq_define(p, cur_cmd, cur_chr);
        else
            eq_define(p, cur_cmd, cur_chr);
        break;

    case SHORTHAND_DEF:
        if (cur_chr == CHAR_SUB_DEF_CODE) {
            scan_char_num();
            p = CHAR_SUB_CODE_BASE + cur_val;
            scan_optional_equals();
            scan_char_num();
            n = cur_val;
            scan_char_num();
            if (INTPAR(tracing_char_sub_def) > 0) {
                begin_diagnostic();
                print_nl(S(New_character_substitution__/**/));
                print(p - CHAR_SUB_CODE_BASE);
                print(S(____Z6/*" = "*/));
                print(n);
                print_char(32 /*" " */ );
                print(cur_val);
                end_diagnostic(false);
            }

            n = n * 256 + cur_val;

            if (a >= 4)
                geq_define(p, DATA, n);
            else
                eq_define(p, DATA, n);

            if ((p - CHAR_SUB_CODE_BASE) < INTPAR(char_sub_def_min)) {
                if (a >= 4)
                    geq_word_define(INT_BASE + INT_PAR__char_sub_def_min, p - CHAR_SUB_CODE_BASE);
                else
                    eq_word_define(INT_BASE + INT_PAR__char_sub_def_min, p - CHAR_SUB_CODE_BASE);
            }

            if ((p - CHAR_SUB_CODE_BASE) > INTPAR(char_sub_def_max)) {
                if (a >= 4)
                    geq_word_define(INT_BASE + INT_PAR__char_sub_def_max, p - CHAR_SUB_CODE_BASE);
                else
                    eq_word_define(INT_BASE + INT_PAR__char_sub_def_max, p - CHAR_SUB_CODE_BASE);
            }
        } else {
            n = cur_chr;
            get_r_token();
            p = cur_cs;

            if (a >= 4)
                geq_define(p, RELAX, TOO_BIG_USV);
            else
                eq_define(p, RELAX, TOO_BIG_USV);

            scan_optional_equals();

            switch (n) {
            case CHAR_DEF_CODE:
                scan_usv_num();
                if (a >= 4)
                    geq_define(p, CHAR_GIVEN, cur_val);
                else
                    eq_define(p, CHAR_GIVEN, cur_val);
                break;

            case MATH_CHAR_DEF_CODE:
                scan_fifteen_bit_int();
                if (a >= 4)
                    geq_define(p, MATH_GIVEN, cur_val);
                else
                    eq_define(p, MATH_GIVEN, cur_val);
                break;

            case XETEX_MATH_CHAR_NUM_DEF_CODE:
                scan_xetex_math_char_int();
                if (a >= 4)
                    geq_define(p, XETEX_MATH_GIVEN, cur_val);
                else
                    eq_define(p, XETEX_MATH_GIVEN, cur_val);
                break;

            case XETEX_MATH_CHAR_DEF_CODE:
                scan_math_class_int();
                n = set_class(cur_val);
                scan_math_fam_int();
                n = n + set_family(cur_val);
                scan_usv_num();
                n = n + cur_val;
                if (a >= 4)
                    geq_define(p, XETEX_MATH_GIVEN, n);
                else
                    eq_define(p, XETEX_MATH_GIVEN, n);
                break;

            default:
                scan_register_num();

                if (cur_val > 255) {
                    j = n - 2;

                    if (j > MU_VAL)
                        j = TOK_VAL;

                    find_sa_element(j, cur_val, true);
                    mem[cur_ptr + 1].hh.v.LH++;

                    if (j == TOK_VAL)
                        j = TOKS_REGISTER;
                    else
                        j = REGISTER;

                    if (a >= 4)
                        geq_define(p, j, cur_ptr);
                    else
                        eq_define(p, j, cur_ptr);
                } else {
                    switch (n) {
                    case COUNT_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_INT, COUNT_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_INT, COUNT_BASE + cur_val);
                        break;
                    case DIMEN_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_DIMEN, SCALED_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_DIMEN, SCALED_BASE + cur_val);
                        break;
                    case SKIP_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_GLUE, SKIP_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_GLUE, SKIP_BASE + cur_val);
                        break;
                    case MU_SKIP_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_MU_GLUE, MU_SKIP_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_MU_GLUE, MU_SKIP_BASE + cur_val);
                        break;
                    case TOKS_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_TOKS, TOKS_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_TOKS, TOKS_BASE + cur_val);
                        break;
                    }
                }
                break;
            }
        }
        break;

    case READ_TO_CS:
        j = cur_chr;
        scan_int();
        n = cur_val;
        if (!scan_keyword(S(to))) {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Missing__to__inserted));
            help_ptr = 2;
            help_line[1] = S(You_should_have_said___read_/*number> to \cs'.*/);
            help_line[0] = S(I_m_going_to_look_for_the__c/*s now.*/);
            error();
        }

        get_r_token();
        p = cur_cs;
        read_toks(n, p, j);

        if (a >= 4)
            geq_define(p, CALL, cur_val);
        else
            eq_define(p, CALL, cur_val);
        break;

    case TOKS_REGISTER:
    case ASSIGN_TOKS:
        q = cur_cs;
        e = false;

        if (cur_cmd == TOKS_REGISTER) {
            if (cur_chr == 0) {
                scan_register_num();
                if (cur_val > 255) {
                    find_sa_element(TOK_VAL, cur_val, true);
                    cur_chr = cur_ptr;
                    e = true;
                } else {
                    cur_chr = TOKS_BASE + cur_val;
                }
            } else {
                e = true;
            }
        } else if (cur_chr == LOCAL_BASE + LOCAL__xetex_inter_char) {
            scan_char_class_not_ignored();
            cur_ptr = cur_val;
            scan_char_class_not_ignored();
            find_sa_element(INTER_CHAR_VAL, cur_ptr * CHAR_CLASS_LIMIT + cur_val, true);
            cur_chr = cur_ptr;
            e = true;
        }

        p = cur_chr;
        scan_optional_equals();

        do {
            get_x_token();
        } while (cur_cmd == SPACER || cur_cmd == RELAX);

        if (cur_cmd != LEFT_BRACE) { /*1262:*/
            if (cur_cmd == TOKS_REGISTER || cur_cmd == ASSIGN_TOKS) {
                if (cur_cmd == TOKS_REGISTER) {
                    if (cur_chr == 0) {
                        scan_register_num();
                        if (cur_val < 256) {
                            q = TOKS_REG(cur_val);
                        } else {
                            find_sa_element(TOK_VAL, cur_val, false);
                            if (cur_ptr == MIN_HALFWORD)
                                q = MIN_HALFWORD;
                            else
                                q = mem[cur_ptr + 1].hh.v.RH;
                        }
                    } else {
                        q = mem[cur_chr + 1].hh.v.RH;
                    }
                } else if (cur_chr == LOCAL_BASE + LOCAL__xetex_inter_char) {
                    scan_char_class_not_ignored();
                    cur_ptr = cur_val;
                    scan_char_class_not_ignored();
                    find_sa_element(INTER_CHAR_VAL, cur_ptr * CHAR_CLASS_LIMIT + cur_val, false);
                    if (cur_ptr == MIN_HALFWORD)
                        q = MIN_HALFWORD;
                    else
                        q = mem[cur_ptr + 1].hh.v.RH;
                } else {
                    q = eqtb[cur_chr].hh.v.RH;
                }

                if (q == MIN_HALFWORD) {
                    if (e) {
                        if (a >= 4)
                            gsa_def(p, MIN_HALFWORD);
                        else
                            sa_def(p, MIN_HALFWORD);
                    } else if (a >= 4) {
                        geq_define(p, UNDEFINED_CS, MIN_HALFWORD);
                    } else {
                        eq_define(p, UNDEFINED_CS, MIN_HALFWORD);
                    }
                } else {
                    mem[q].hh.v.LH++;
                    if (e) {
                        if (a >= 4)
                            gsa_def(p, q);
                        else
                            sa_def(p, q);
                    } else if (a >= 4) {
                        geq_define(p, CALL, q);
                    } else {
                        eq_define(p, CALL, q);
                    }
                }

                goto done;
            }
        }

        back_input();
        cur_cs = q;
        q = scan_toks(false, false);

        if (mem[def_ref].hh.v.RH == MIN_HALFWORD) {
            if (e) {
                if (a >= 4)
                    gsa_def(p, MIN_HALFWORD);
                else
                    sa_def(p, MIN_HALFWORD);
            } else if (a >= 4) {
                geq_define(p, UNDEFINED_CS, MIN_HALFWORD);
            } else {
                eq_define(p, UNDEFINED_CS, MIN_HALFWORD);
            }

            mem[def_ref].hh.v.RH = avail;
            avail = def_ref;
        } else {
            if (p == LOCAL_BASE + LOCAL__output_routine && !e) {
                mem[q].hh.v.RH = get_avail();
                q = mem[q].hh.v.RH;
                mem[q].hh.v.LH = (RIGHT_BRACE_TOKEN + 125);
                q = get_avail();
                mem[q].hh.v.LH = (LEFT_BRACE_TOKEN + 123);
                mem[q].hh.v.RH = mem[def_ref].hh.v.RH;
                mem[def_ref].hh.v.RH = q;
            }

            if (e) {
                if (a >= 4)
                    gsa_def(p, def_ref);
                else
                    sa_def(p, def_ref);
            } else if (a >= 4) {
                geq_define(p, CALL, def_ref);
            } else {
                eq_define(p, CALL, def_ref);
            }
        }

        break;

    case ASSIGN_INT:
        p = cur_chr;
        scan_optional_equals();
        scan_int();
        if (a >= 4)
            geq_word_define(p, cur_val);
        else
            eq_word_define(p, cur_val);
        break;

    case ASSIGN_DIMEN:
        p = cur_chr;
        scan_optional_equals();
        scan_dimen(false, false, false);
        if (a >= 4)
            geq_word_define(p, cur_val);
        else
            eq_word_define(p, cur_val);
        break;

    case ASSIGN_GLUE:
    case ASSIGN_MU_GLUE:
        p = cur_chr;
        n = cur_cmd;
        scan_optional_equals();
        if (n == ASSIGN_MU_GLUE)
            scan_glue(MU_VAL);
        else
            scan_glue(GLUE_VAL);
        trap_zero_glue();
        if (a >= 4)
            geq_define(p, GLUE_REF, cur_val);
        else
            eq_define(p, GLUE_REF, cur_val);
        break;

    case XETEX_DEF_CODE:
        if (cur_chr == SF_CODE_BASE) {
            p = cur_chr;
            scan_usv_num();
            p = p + cur_val;
            n = SF_CODE(cur_val) % 65536L;
            scan_optional_equals();
            scan_char_class();
            if (a >= 4)
                geq_define(p, DATA, cur_val * 65536L + n);
            else
                eq_define(p, DATA, cur_val * 65536L + n);
        } else if (cur_chr == MATH_CODE_BASE) {
            p = cur_chr;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            scan_xetex_math_char_int();
            if (a >= 4)
                geq_define(p, DATA, cur_val);
            else
                eq_define(p, DATA, cur_val);
        } else if (cur_chr == MATH_CODE_BASE + 1) {
            p = cur_chr - 1;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            scan_math_class_int();
            n = set_class(cur_val);
            scan_math_fam_int();
            n = n + set_family(cur_val);
            scan_usv_num();
            n = n + cur_val;
            if (a >= 4)
                geq_define(p, DATA, n);
            else
                eq_define(p, DATA, n);
        } else if (cur_chr == DEL_CODE_BASE) {
            p = cur_chr;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            scan_int();
            if (a >= 4)
                geq_word_define(p, cur_val);
            else
                eq_word_define(p, cur_val);
        } else {
            p = cur_chr - 1;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            n = 0x40000000; /* "extended delimiter code flag" */
            scan_math_fam_int();
            n = n + cur_val * 0x200000; /* "extended delimiter code family */
            scan_usv_num();
            n = n + cur_val;
            if (a >= 4)
                geq_word_define(p, n);
            else
                eq_word_define(p, n);
        }
        break;

    case DEF_CODE:
        if (cur_chr == CAT_CODE_BASE)
            n = MAX_CHAR_CODE;
        else if (cur_chr == MATH_CODE_BASE)
            n = 0x8000;
        else if (cur_chr == SF_CODE_BASE)
            n = 0x7FFF;
        else if (cur_chr == DEL_CODE_BASE)
            n = 0xFFFFFF;
        else
            n = BIGGEST_USV; /*:1268 */

        p = cur_chr;
        scan_usv_num();
        p = p + cur_val;
        scan_optional_equals();
        scan_int();

        if ((cur_val < 0 && p < DEL_CODE_BASE) || cur_val > n) {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Invalid_code__));
            print_int(cur_val);
            if (p < DEL_CODE_BASE)
                print(S(___should_be_in_the_range_0_/*.*/));
            else
                print(S(___should_be_at_most_));
            print_int(n);
            help_ptr = 1;
            help_line[0] = S(I_m_going_to_use_0_instead_o/*f that illegal code value.*/);
            error();
            cur_val = 0;
        }

        if (p < MATH_CODE_BASE) {
            if (p >= SF_CODE_BASE) {
                n = eqtb[p].hh.v.RH / 65536L;
                if (a >= 4)
                    geq_define(p, DATA, n * 65536L + cur_val);
                else
                    eq_define(p, DATA, n * 65536L + cur_val);
            } else if (a >= 4) {
                geq_define(p, DATA, cur_val);
            } else {
                eq_define(p, DATA, cur_val);
            }
        } else if (p < DEL_CODE_BASE) {
            if (cur_val == 32768L)
                cur_val = ACTIVE_MATH_CHAR;
            else
                cur_val = set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) + (cur_val % 256);

            if (a >= 4)
                geq_define(p, DATA, cur_val);
            else
                eq_define(p, DATA, cur_val);
        } else if (a >= 4) {
            geq_word_define(p, cur_val);
        } else {
            eq_word_define(p, cur_val);
        }
        break;

    case DEF_FAMILY:
        p = cur_chr;
        scan_math_fam_int();
        p = p + cur_val;
        scan_optional_equals();
        scan_font_ident();
        if (a >= 4)
            geq_define(p, DATA, cur_val);
        else
            eq_define(p, DATA, cur_val);
        break;

    case REGISTER:
    case ADVANCE:
    case MULTIPLY:
    case DIVIDE:
        do_register_command(a);
        break;

    case SET_BOX:
        scan_register_num();
        if (a >= 4)
            n = GLOBAL_BOX_FLAG + cur_val;
        else
            n = BOX_FLAG + cur_val;

        scan_optional_equals();

        if (set_box_allowed) {
            scan_box(n);
        } else {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Improper_));
            print_esc(S(setbox));
            help_ptr = 2;
            help_line[1] = S(Sorry___setbox_is_not_allowe/*d after \halign in a display,*/);
            help_line[0] = S(or_between__accent_and_an_ac/*cented character.*/);
            error();
        }
        break;

    case SET_AUX:
        alter_aux();
        break;

    case SET_PREV_GRAF:
        alter_prev_graf();
        break;

    case SET_PAGE_DIMEN:
        alter_page_so_far();
        break;

    case SET_PAGE_INT:
        alter_integer();
        break;

    case SET_BOX_DIMEN:
        alter_box_dimen();
        break;

    case SET_SHAPE:
        q = cur_chr;
        scan_optional_equals();
        scan_int();
        n = cur_val;

        if (n <= 0) {
            p = MIN_HALFWORD;
        } else if (q > LOCAL_BASE + LOCAL__par_shape) {
            n = (cur_val / 2) + 1;
            p = get_node(2 * n + 1);
            mem[p].hh.v.LH = n;
            n = cur_val;
            mem[p + 1].cint = n;

            for (j = p + 2; j <= p + n + 1; j++) {
                scan_int();
                mem[j].cint = cur_val;
            }

            if (!odd(n))
                mem[p + n + 2].cint = 0;
        } else {
            p = get_node(2 * n + 1);
            mem[p].hh.v.LH = n;

            for (j = 1; j <= n; j++) {
                scan_dimen(false, false, false);
                mem[p + 2 * j - 1].cint = cur_val;
                scan_dimen(false, false, false);
                mem[p + 2 * j].cint = cur_val;
            }
        }

        if (a >= 4)
            geq_define(q, SHAPE_REF, p);
        else
            eq_define(q, SHAPE_REF, p);
        break;

    case HYPH_DATA:
        if (cur_chr == 1) {
            if (in_initex_mode) {
                new_patterns();
                goto done;
            }

            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Patterns_can_be_loaded_only_/*by INITEX*/));
            help_ptr = 0;
            error();

            do {
                get_token();
            } while (cur_cmd != RIGHT_BRACE);

            return;
        } else {
            new_hyph_exceptions();
            goto done;
        }
        break;

    case ASSIGN_FONT_DIMEN:
        find_font_dimen(true);
        k = cur_val;
        scan_optional_equals();
        scan_dimen(false, false, false);
        font_info[k].cint = cur_val;
        break;

    case ASSIGN_FONT_INT:
        n = cur_chr;
        scan_font_ident();
        f = cur_val;

        if (n < 2) {
            scan_optional_equals();
            scan_int();
            if (n == 0)
                hyphen_char[f] = cur_val;
            else
                skew_char[f] = cur_val;
        } else {
            if (font_area[f] == AAT_FONT_FLAG || font_area[f] == OTGR_FONT_FLAG)
                scan_glyph_number(f);
            else
                scan_char_num();

            p = cur_val;
            scan_optional_equals();
            scan_int();
            switch (n) {
            case LP_CODE_BASE:
                set_cp_code(f, p, LEFT_SIDE, cur_val);
                break;
            case RP_CODE_BASE:
                set_cp_code(f, p, RIGHT_SIDE, cur_val);
                break;
            }
        }
        break;

    case DEF_FONT:
        new_font(a);
        break;

    case SET_INTERACTION:
        new_interaction();
        break;

    default:
        confusion(S(prefix));
        break;
    }

done: /*1304:*/
    if (after_token != 0) {
        cur_tok = after_token;
        back_input();
        after_token = 0;
    }
}
/*:1328*/


/*1337:*/
static void
store_fmt_file(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer j, k, l;
    int32_t p, q;
    integer x;
    char *format_engine;
    rust_output_handle_t fmt_out;

    if (save_ptr != 0) {
	if (file_line_error_style_p)
	    print_file_line();
	else
	    print_nl(S(__/*"! "*/));

	print(S(You_can_t_dump_inside_a_grou/*p*/));
	help_ptr = 1;
	help_line[0] = S(______dump___is_a_no_no_/*`{...\\dump}' is a no-no.*/);

	if (interaction == ERROR_STOP_MODE)
	    interaction = SCROLL_MODE;
	if (log_opened)
	    error();

	history = HISTORY_FATAL_ERROR;
	close_files_and_terminate();
	ttstub_output_flush (rust_stdout);
	_tt_abort("\\dump inside a group");
    }

    selector = SELECTOR_NEW_STRING;
    print(S(__preloaded_format_));
    print(job_name);
    print_char(32 /*" " */ );
    print_int(INTPAR(year));
    print_char(46 /*"." */ );
    print_int(INTPAR(month));
    print_char(46 /*"." */ );
    print_int(INTPAR(day));
    print_char(41 /*")" */ );

    if (interaction == BATCH_MODE)
        selector = SELECTOR_LOG_ONLY;
    else
        selector = SELECTOR_TERM_AND_LOG;

    if (pool_ptr + 1 > pool_size)
        overflow(S(pool_size), pool_size - init_pool_ptr);

    format_ident = make_string();
    pack_job_name(FORMAT_EXTENSION);

    fmt_out = ttstub_output_open (name_of_file + 1, 1);
    if (fmt_out == NULL)
	_tt_abort ("cannot open format output file \"%s\"", name_of_file + 1);

    print_nl(S(Beginning_to_dump_on_file_));
    print(make_name_string());

    str_ptr--;
    pool_ptr = str_start[str_ptr - 65536L];

    print_nl(S());
    print(format_ident);

    /* Header */

    dump_int(0x57325458); /* magic constant: "W2TX" in ASCII */

    /* write the engine name, padded to align to 4 byte blocks */
    x = strlen(engine_name);
    format_engine = xmalloc_array(char, x + 4);
    strcpy(format_engine, engine_name);
    for (k = x; k < x + 4; k++)
        format_engine[k] = '\0';
    x = x + 4 - (x % 4);
    dump_int(x);
    dump_things(format_engine[0], x);
    free(format_engine);

    dump_int(STRING_POOL_CHECKSUM);
    dump_int(MAX_HALFWORD);
    dump_int(hash_high);
    dump_int(1); /* eTeX enabled? */

    while (pseudo_files != MIN_HALFWORD)
        pseudo_close();

    dump_int(0); /* mem_bot */
    dump_int(mem_top);
    dump_int(EQTB_SIZE);
    dump_int(HASH_PRIME);
    dump_int(HYPH_PRIME);

    /* fake disabled MLTex for TeXLive compatibility */

    dump_int(0x4D4C5458);
    dump_int(0);

    /* string pool */

    dump_int(pool_ptr);
    dump_int(str_ptr);
    dump_things(str_start[TOO_BIG_CHAR - 65536L], str_ptr - 65535L);
    dump_things(str_pool[0], pool_ptr);

    print_ln();
    print_int(str_ptr);
    print(S(_strings_of_total_length_));
    print_int(pool_ptr);

    /* "memory locations" */

    sort_avail();
    var_used = 0;
    dump_int(lo_mem_max);
    dump_int(rover);

    for (k = INT_VAL; k <= INTER_CHAR_VAL; k++)
        dump_int(sa_root[k]);

    p = 0;
    q = rover;
    x = 0;
    do {
        dump_things(mem[p], q + 2 - p);
        x = x + q + 2 - p;
        var_used = var_used + q - p;
        p = q + mem[q].hh.v.LH;
        q = mem[q + 1].hh.v.RH;
    } while (q != rover);

    var_used = var_used + lo_mem_max - p;
    dyn_used = mem_end + 1 - hi_mem_min;
    dump_things(mem[p], lo_mem_max + 1 - p);

    x = x + lo_mem_max + 1 - p;
    dump_int(hi_mem_min);
    dump_int(avail);
    dump_things(mem[hi_mem_min], mem_end + 1 - hi_mem_min);

    x = x + mem_end + 1 - hi_mem_min;
    p = avail;
    while (p != MIN_HALFWORD) {
        dyn_used--;
        p = mem[p].hh.v.RH;
    }

    dump_int(var_used);
    dump_int(dyn_used);

    print_ln();
    print_int(x);
    print(S(_memory_locations_dumped__cu/*rrent usage is */));
    print_int(var_used);
    print_char(38 /*"&" */ );
    print_int(dyn_used);

    /* equivalents table / primitive */

    k = ACTIVE_BASE;

    do {
        j = k;

        while (j < INT_BASE - 1) {
            if (eqtb[j].hh.v.RH == eqtb[j + 1].hh.v.RH &&
                eqtb[j].hh.u.B0 == eqtb[j + 1].hh.u.B0 &&
                eqtb[j].hh.u.B1 == eqtb[j + 1].hh.u.B1)
                goto found1;
            j++;
        }

        l = INT_BASE;
        goto done1;

    found1:
        j++;
        l = j;

        while (j < INT_BASE - 1) {
            if (eqtb[j].hh.v.RH != eqtb[j + 1].hh.v.RH ||
                eqtb[j].hh.u.B0 != eqtb[j + 1].hh.u.B0 ||
                eqtb[j].hh.u.B1 != eqtb[j + 1].hh.u.B1)
                goto done1;
            j++;
        }
    done1:

        dump_int(l - k);
        dump_things(eqtb[k], l - k);
        k = j + 1;
        dump_int(k - l);
    } while (k != INT_BASE); /*:1350*/

    do {
        j = k;

        while (j < EQTB_SIZE) {
            if (eqtb[j].cint == eqtb[j + 1].cint)
                goto found2;
            j++;
        }

        l = EQTB_SIZE + 1;
        goto done2;

    found2:
        j++;
        l = j;

        while (j < EQTB_SIZE) {
            if (eqtb[j].cint != eqtb[j + 1].cint)
                goto done2;
            j++;
        }

    done2:
        dump_int(l - k);
        dump_things(eqtb[k], l - k);
        k = j + 1;
        dump_int(k - l);
    } while (k <= EQTB_SIZE);

    if (hash_high > 0)
        dump_things(eqtb[EQTB_SIZE + 1], hash_high);

    dump_int(par_loc);
    dump_int(write_loc);

    for (p = 0; p <= PRIM_SIZE; p++)
        dump_hh(prim[p]);

    for (p = 0; p <= PRIM_SIZE; p++)
        dump_wd(prim_eqtb[p]);

    /* control sequences */

    dump_int(hash_used);
    cs_count = (FROZEN_CONTROL_SEQUENCE - 1) - hash_used + hash_high;

    for (p = HASH_BASE; p <= hash_used; p++) {
        if (hash[p].v.RH != 0) {
            dump_int(p);
            dump_hh(hash[p]);
            cs_count++;
        }
    }

    dump_things(hash[hash_used + 1], (UNDEFINED_CONTROL_SEQUENCE - 1) - hash_used);
    if (hash_high > 0)
        dump_things(hash[EQTB_SIZE + 1], hash_high);

    dump_int(cs_count);

    print_ln();
    print_int(cs_count);
    print(S(_multiletter_control_sequenc/*es*/));

    /* fonts */

    dump_int(fmem_ptr);
    dump_things(font_info[0], fmem_ptr);
    dump_int(font_ptr);
    dump_things(font_check[FONT_BASE], font_ptr + 1);
    dump_things(font_size[FONT_BASE], font_ptr + 1);
    dump_things(font_dsize[FONT_BASE], font_ptr + 1);
    dump_things(font_params[FONT_BASE], font_ptr + 1);
    dump_things(hyphen_char[FONT_BASE], font_ptr + 1);
    dump_things(skew_char[FONT_BASE], font_ptr + 1);
    dump_things(font_name[FONT_BASE], font_ptr + 1);
    dump_things(font_area[FONT_BASE], font_ptr + 1);
    dump_things(font_bc[FONT_BASE], font_ptr + 1);
    dump_things(font_ec[FONT_BASE], font_ptr + 1);
    dump_things(char_base[FONT_BASE], font_ptr + 1);
    dump_things(width_base[FONT_BASE], font_ptr + 1);
    dump_things(height_base[FONT_BASE], font_ptr + 1);
    dump_things(depth_base[FONT_BASE], font_ptr + 1);
    dump_things(italic_base[FONT_BASE], font_ptr + 1);
    dump_things(lig_kern_base[FONT_BASE], font_ptr + 1);
    dump_things(kern_base[FONT_BASE], font_ptr + 1);
    dump_things(exten_base[FONT_BASE], font_ptr + 1);
    dump_things(param_base[FONT_BASE], font_ptr + 1);
    dump_things(font_glue[FONT_BASE], font_ptr + 1);
    dump_things(bchar_label[FONT_BASE], font_ptr + 1);
    dump_things(font_bchar[FONT_BASE], font_ptr + 1);
    dump_things(font_false_bchar[FONT_BASE], font_ptr + 1);

    for (k = FONT_BASE; k <= font_ptr; k++) {
        print_nl(S(_font));
        print_esc(hash[FONT_ID_BASE + k].v.RH);
        print_char(61 /*"=" */ );

        if (font_area[k] == AAT_FONT_FLAG || font_area[k] == OTGR_FONT_FLAG || font_mapping[k] != 0) {
            print_file_name(font_name[k], S(), S());

            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Can_t__dump_a_format_with_na/*tive fonts or font-mappings*/));

            help_ptr = 3;
            help_line[2] = S(You_really__really_don_t_wan/*t to do this.*/);
            help_line[1] = S(It_won_t_work__and_only_conf/*uses me.*/);
            help_line[0] = S(_Load_them_at_runtime__not_a/*s part of the format file.)*/);
            error();
        } else {
            print_file_name(font_name[k], font_area[k], S());
        }

        if (font_size[k] != font_dsize[k]) {
            print(S(_at_));
            print_scaled(font_size[k]);
            print(S(pt));
        }
    }

    print_ln();
    print_int(fmem_ptr - 7);
    print(S(_words_of_font_info_for_));
    print_int(font_ptr - 0);
    if (font_ptr != FONT_BASE + 1)
        print(S(_preloaded_fonts));
    else
        print(S(_preloaded_font));

    /* hyphenation info */

    dump_int(hyph_count);
    if (hyph_next <= HYPH_PRIME)
        hyph_next = hyph_size;
    dump_int(hyph_next);

    for (k = 0; k <= hyph_size; k++) {
        if (hyph_word[k] != 0) {
            dump_int(k + 65536L * hyph_link[k]);
            dump_int(hyph_word[k]);
            dump_int(hyph_list[k]);
        }
    }

    print_ln();
    print_int(hyph_count);
    if (hyph_count != 1)
        print(S(_hyphenation_exceptions));
    else
        print(S(_hyphenation_exception));

    if (trie_not_ready)
        init_trie();

    dump_int(trie_max);
    dump_int(hyph_start);
    dump_things(trie_trl[0], trie_max + 1);
    dump_things(trie_tro[0], trie_max + 1);
    dump_things(trie_trc[0], trie_max + 1);
    dump_int(max_hyph_char);
    dump_int(trie_op_ptr);
    dump_things(hyf_distance[1], trie_op_ptr);
    dump_things(hyf_num[1], trie_op_ptr);
    dump_things(hyf_next[1], trie_op_ptr);

    print_nl(S(Hyphenation_trie_of_length_));
    print_int(trie_max);
    print(S(_has_));
    print_int(trie_op_ptr);
    if (trie_op_ptr != 1)
        print(S(_ops));
    else
        print(S(_op));
    print(S(_out_of_));
    print_int(trie_op_size);

    for (k = BIGGEST_LANG; k >= 0; k--) {
        if (trie_used[k] > 0) {
            print_nl(S(___Z12/*"  "*/));
            print_int(trie_used[k]);
            print(S(_for_language_));
            print_int(k);
            dump_int(k);
            dump_int(trie_used[k]);
        }
    }

    /* trailer */

    dump_int(interaction);
    dump_int(format_ident);
    dump_int(69069L);

    INTPAR(tracing_stats) = 0; /*:1361*/
    ttstub_output_close(fmt_out);
}


static void
pack_buffered_name(small_number n, integer a, integer b)
{
    integer k;
    UTF16_code c;
    integer j;

    if (n + b - a + 5 > INTEGER_MAX)
        b = a + INTEGER_MAX - n - 5;

    if (name_of_file)
        free(name_of_file);
    name_of_file = xmalloc_array(UTF8_code, n + (b - a + 1) + 5);

    k = 0;

    for (j = 1; j <= n; j++) {
	/* This junk is append_to_name(), inlined, and with UTF-8 decoding, I
	 * think. */
	c = TEX_format_default[j];
	k++;
	if (k <= INTEGER_MAX) {
	    if (c < 128) {
		name_of_file[k] = c;
	    } else if (c < 2048) {
		name_of_file[k++] = 192 + c / 64;
		name_of_file[k] = 128 + c % 64;
	    } else {
		name_of_file[k++] = 224 + c / 4096;
		name_of_file[k++] = 128 + (c % 4096) / 64;
		name_of_file[k] = 128 + (c % 4096) % 64;
	    }
	}
    }

    for (j = a; j <= b; j++) {
	c = buffer[j];
	k++;
	if (k <= INTEGER_MAX) {
	    if (c < 128) {
		name_of_file[k] = c;
	    } else if (c < 2048) {
		name_of_file[k++] = 192 + c / 64;
		name_of_file[k] = 128 + c % 64;
	    } else {
		name_of_file[k++] = 224 + c / 4096;
		name_of_file[k++] = 128 + (c % 4096) / 64;
		name_of_file[k] = 128 + (c % 4096) % 64;
	    }
	}
    }

    for (j = format_default_length - 3; j <= format_default_length; j++) {
	c = TEX_format_default[j];
	k++;
	if (k <= INTEGER_MAX) {
	    if (c < 128) {
		name_of_file[k] = c;
	    } else if (c < 2048) {
		name_of_file[k++] = 192 + c / 64;
		name_of_file[k] = 128 + c % 64;
	    } else {
		name_of_file[k++] = 224 + c / 4096;
		name_of_file[k++] = 128 + (c % 4096) / 64;
		name_of_file[k] = 128 + (c % 4096) % 64;
	    }
	}
    }

    if (k <= INTEGER_MAX)
        name_length = k;
    else
        name_length = INTEGER_MAX;

    name_of_file[name_length + 1] = 0;
}


static boolean
load_fmt_file(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer j, k, format_written_with_etex;
    int32_t p, q;
    integer x;
    char *format_engine;
    rust_input_handle_t fmt_in;

    j = cur_input.loc;

    /* This is where a first line starting with "&" used to
     * trigger code that would change the format file. */

    pack_buffered_name(format_default_length - 4, 1, 0);

    fmt_in = ttstub_input_open(name_of_file + 1, kpse_fmt_format, 1);
    if (fmt_in == NULL)
	_tt_abort ("cannot open the format file \"%s\"", TEX_format_default + 1);

    cur_input.loc = j;

    if (in_initex_mode) {
        free(font_info);
        free(str_pool);
        free(str_start);
        free(yhash);
        free(the_eqtb);
        free(yzmem);
    }

    /* start reading the header */

    undump_int(x);
    if (x != 0x57325458) /* magic constant: "W2TX" in ASCII */
        goto bad_fmt;

    undump_int(x); /* length of engine name */
    if (x < 0 || x > 256)
        goto bad_fmt;

    format_engine = xmalloc_array(char, x);
    undump_things(format_engine[0], x);
    format_engine[x - 1] = '\0';
    if (strcmp(engine_name, format_engine)) {
        free(format_engine);
        _tt_abort("format file %s from wrong engine %s", (string) name_of_file + 1, format_engine);
    }
    free(format_engine);

    undump_int(x);
    if (x != STRING_POOL_CHECKSUM)
        _tt_abort("format file %s doesn't match xetex.pool", (string) name_of_file + 1);

    undump_int(x); /* max_halfword */
    if (x != MAX_HALFWORD)
        goto bad_fmt;

    /* hash table parameters */

    undump_int(hash_high);
    if (hash_high < 0 || hash_high > sup_hash_extra)
        goto bad_fmt;
    if (hash_extra < hash_high)
        hash_extra = hash_high;

    eqtb_top = EQTB_SIZE + hash_extra;
    if (hash_extra == 0)
        hash_top = UNDEFINED_CONTROL_SEQUENCE;
    else
        hash_top = eqtb_top;

    yhash = xmalloc_array(two_halves, 1 + hash_top - hash_offset);
    hash = yhash - hash_offset;
    hash[HASH_BASE].v.LH = 0;
    hash[HASH_BASE].v.RH = 0;

    for (x = HASH_BASE + 1; x <= hash_top; x++)
        hash[x] = hash[HASH_BASE];

    eqtb = the_eqtb = xmalloc_array(memory_word, eqtb_top + 1);
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.u.B0 = UNDEFINED_CS;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.v.RH = MIN_HALFWORD;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.u.B1 = LEVEL_ZERO;

    for (x = EQTB_SIZE + 1; x <= eqtb_top; x++)
        eqtb[x] = eqtb[UNDEFINED_CONTROL_SEQUENCE];

    /* eTeX? */

    undump_int(format_written_with_etex);
    if (format_written_with_etex < 0 || format_written_with_etex > 1)
        goto bad_fmt;

    max_reg_num = 32767;
    max_reg_help_line = S(A_register_number_must_be_be_Z1/*"A register number must be between 0 and 32767."*/);

    /* "memory locations" */

    undump_int(x);
    if (x != 0) /* mem_bot */
        goto bad_fmt;

    undump_int(mem_top);
    if (mem_top < 1100)
        goto bad_fmt;

    cur_list.head = mem_top - 1;
    cur_list.tail = mem_top - 1;
    page_tail = mem_top - 2;
    yzmem = xmalloc_array(memory_word, mem_top + 1);
    zmem = yzmem;
    mem = zmem;

    undump_int(x);
    if (x != EQTB_SIZE)
        goto bad_fmt;

    undump_int(x);
    if (x != HASH_PRIME)
        goto bad_fmt;

    undump_int(x);
    if (x != HYPH_PRIME)
        goto bad_fmt;

    /* MLTeX */

    undump_int(x);
    if (x != 0x4D4C5458) /* MLTeX magic constant "MLTX" */
        goto bad_fmt;

    undump_int(x);
    if (x != 0)
        _tt_abort("this format uses MLTeX, which has been removed from Tectonic");

    /* string pool */

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > sup_pool_size - pool_free)
        _tt_abort ("must increase string_pool_size");
    pool_ptr = x;

    if (pool_size < pool_ptr + pool_free)
        pool_size = pool_ptr + pool_free;

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > sup_max_strings - strings_free)
        _tt_abort ("must increase sup_strings");
    str_ptr = x;

    if (max_strings < str_ptr + strings_free)
        max_strings = str_ptr + strings_free;

    str_start = xmalloc_array(pool_pointer, max_strings);
    undump_checked_things(0, pool_ptr, str_start[(TOO_BIG_CHAR) - 65536L], str_ptr - 65535L);
    str_pool = xmalloc_array(packed_UTF16_code, pool_size);

    undump_things(str_pool[0], pool_ptr);

    init_str_ptr = str_ptr;
    init_pool_ptr = pool_ptr; /*:1345 */

    /* "By sorting the list of available spaces in the variable-size portion
     * of |mem|, we are usually able to get by without having to dump very
     * much of the dynamic memory." */

    undump_int(x);
    if (x < 1019 || x > mem_top - 15)
        goto bad_fmt;
    else
        lo_mem_max = x;

    undump_int(x);
    if (x < 20 || x > lo_mem_max)
        goto bad_fmt;
    else
        rover = x;

    if (format_written_with_etex) {
        for (k = INT_VAL; k <= INTER_CHAR_VAL; k++) {
            undump_int(x);
            if (x < MIN_HALFWORD || x > lo_mem_max)
                goto bad_fmt;
            else
                sa_root[k] = x;
        }
    }

    p = 0;
    q = rover;

    do {
        undump_things(mem[p], q + 2 - p);
        p = q + mem[q].hh.v.LH;
        if (p > lo_mem_max || (q >= mem[q + 1].hh.v.RH && mem[q + 1].hh.v.RH != rover))
            goto bad_fmt;
        q = mem[q + 1].hh.v.RH;
    } while (q != rover);

    undump_things(mem[p], lo_mem_max + 1 - p);

    undump_int(x);
    if (x < lo_mem_max + 1 || x > mem_top - 14)
        goto bad_fmt;
    else
        hi_mem_min = x;

    undump_int(x);
    if (x < MIN_HALFWORD || x > mem_top)
        goto bad_fmt;
    else
        avail = x;

    mem_end = mem_top;

    undump_things(mem[hi_mem_min], mem_end + 1 - hi_mem_min);
    undump_int(var_used);
    undump_int(dyn_used);

    /* equivalents table / primitives
     *
     * "The table of equivalents usually contains repeated information, so we
     * dump it in compressed form: The sequence of $n + 2$ values
     * $(n, x_1, \ldots, x_n, m)$ in the format file represents $n + m$ consecutive
     * entries of |eqtb|, with |m| extra copies of $x_n$, namely
     * $(x_1, \ldots, x_n, x_n, \ldots, x_n)$"
     */

    k = ACTIVE_BASE;

    do {
        undump_int(x);
        if (x < 1 || k + x > EQTB_SIZE + 1)
            goto bad_fmt;

        undump_things(eqtb[k], x);
        k = k + x;

        undump_int(x);
        if (x < 0 || k + x > EQTB_SIZE + 1)
            goto bad_fmt;

        for (j = k; j <= k + x - 1; j++)
            eqtb[j] = eqtb[k - 1];

        k = k + x;
    } while (k <= EQTB_SIZE);

    if (hash_high > 0)
        undump_things(eqtb[EQTB_SIZE + 1], hash_high);

    undump_int(x);
    if (x < HASH_BASE || x > hash_top)
        goto bad_fmt;
    else
        par_loc = x;

    par_token = CS_TOKEN_FLAG + par_loc;

    undump_int(x);
    if (x < HASH_BASE || x > hash_top)
        goto bad_fmt;
    else
        write_loc = x;

    /* control sequence names
     *
     * "A different scheme is used to compress the hash table, since its lower
     * region is usually sparse. When |text(p) != 0| for |p <= hash_used|, we
     * output two words, |p| and |hash[p]|. The hash table is, of course,
     * densely packed for |p >= hash_used|, so the remaining entries are
     * output in a block."
     */

    for (p = 0; p <= PRIM_SIZE; p++)
        undump_hh(prim[p]);

    for (p = 0; p <= PRIM_SIZE; p++)
        undump_wd(prim_eqtb[p]);

    undump_int(x);
    if (x < HASH_BASE || x > FROZEN_CONTROL_SEQUENCE)
        goto bad_fmt;
    else
        hash_used = x;

    p = HASH_BASE - 1;

    do {
        undump_int(x);
        if (x < p + 1 || x > hash_used)
            goto bad_fmt;
        else
            p = x;
        undump_hh(hash[p]);
    } while (p != hash_used);

    undump_things(hash[hash_used + 1], (UNDEFINED_CONTROL_SEQUENCE - 1) - hash_used);

    if (hash_high > 0)
        undump_things(hash[EQTB_SIZE + 1], hash_high);

    undump_int(cs_count);

    /* font info */

    undump_int(x);
    if (x < 7)
        goto bad_fmt;
    if (x > sup_font_mem_size)
        _tt_abort ("must increase font_mem_size");

    fmem_ptr = x;
    if (fmem_ptr > font_mem_size)
        font_mem_size = fmem_ptr;

    font_info = xmalloc_array(fmemory_word, font_mem_size);
    undump_things(font_info[0], fmem_ptr);

    undump_int(x);
    if (x < FONT_BASE)
        goto bad_fmt;
    if (x > FONT_BASE + MAX_FONT_MAX)
        _tt_abort ("must increase font_max");

    font_ptr = x;

    font_mapping = xmalloc_array(void *, font_max);
    font_layout_engine = xmalloc_array(void *, font_max);
    font_flags = xmalloc_array(char, font_max);
    font_letter_space = xmalloc_array(scaled, font_max);
    font_check = xmalloc_array(four_quarters, font_max);
    font_size = xmalloc_array(scaled, font_max);
    font_dsize = xmalloc_array(scaled, font_max);
    font_params = xmalloc_array(font_index, font_max);
    font_name = xmalloc_array(str_number, font_max);
    font_area = xmalloc_array(str_number, font_max);
    font_bc = xmalloc_array(UTF16_code, font_max);
    font_ec = xmalloc_array(UTF16_code, font_max);
    font_glue = xmalloc_array(int32_t, font_max);
    hyphen_char = xmalloc_array(integer, font_max);
    skew_char = xmalloc_array(integer, font_max);
    bchar_label = xmalloc_array(font_index, font_max);
    font_bchar = xmalloc_array(nine_bits, font_max);
    font_false_bchar = xmalloc_array(nine_bits, font_max);
    char_base = xmalloc_array(integer, font_max);
    width_base = xmalloc_array(integer, font_max);
    height_base = xmalloc_array(integer, font_max);
    depth_base = xmalloc_array(integer, font_max);
    italic_base = xmalloc_array(integer, font_max);
    lig_kern_base = xmalloc_array(integer, font_max);
    kern_base = xmalloc_array(integer, font_max);
    exten_base = xmalloc_array(integer, font_max);
    param_base = xmalloc_array(integer, font_max);

    for (k = FONT_BASE; k <= font_ptr; k++)
        font_mapping[k] = 0;

    undump_things(font_check[FONT_BASE], font_ptr + 1);
    undump_things(font_size[FONT_BASE], font_ptr + 1);
    undump_things(font_dsize[FONT_BASE], font_ptr + 1);
    undump_checked_things(MIN_HALFWORD, MAX_HALFWORD, font_params[FONT_BASE], font_ptr + 1);
    undump_things(hyphen_char[FONT_BASE], font_ptr + 1);
    undump_things(skew_char[FONT_BASE], font_ptr + 1);
    undump_upper_check_things(str_ptr, font_name[FONT_BASE], font_ptr + 1);
    undump_upper_check_things(str_ptr, font_area[FONT_BASE], font_ptr + 1);
    undump_things(font_bc[FONT_BASE], font_ptr + 1);
    undump_things(font_ec[FONT_BASE], font_ptr + 1);
    undump_things(char_base[FONT_BASE], font_ptr + 1);
    undump_things(width_base[FONT_BASE], font_ptr + 1);
    undump_things(height_base[FONT_BASE], font_ptr + 1);
    undump_things(depth_base[FONT_BASE], font_ptr + 1);
    undump_things(italic_base[FONT_BASE], font_ptr + 1);
    undump_things(lig_kern_base[FONT_BASE], font_ptr + 1);
    undump_things(kern_base[FONT_BASE], font_ptr + 1);
    undump_things(exten_base[FONT_BASE], font_ptr + 1);
    undump_things(param_base[FONT_BASE], font_ptr + 1);
    undump_checked_things(MIN_HALFWORD, lo_mem_max, font_glue[FONT_BASE], font_ptr + 1);
    undump_checked_things(0, fmem_ptr - 1, bchar_label[FONT_BASE], font_ptr + 1);
    undump_checked_things(0, TOO_BIG_CHAR, font_bchar[FONT_BASE], font_ptr + 1);
    undump_checked_things(0, TOO_BIG_CHAR, font_false_bchar[FONT_BASE], font_ptr + 1);

    /* hyphenations */

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > hyph_size)
        _tt_abort ("must increase hyph_size");
    hyph_count = x;

    undump_int(x);
    if (x < HYPH_PRIME)
        goto bad_fmt;
    if (x > hyph_size)
        _tt_abort ("must increase hyph_size");
    hyph_next = x;

    j = 0;

    for (k = 1; k <= hyph_count; k++) {
        undump_int(j);
        if (j < 0)
            goto bad_fmt;
        if (j > 65535L) {
            hyph_next = j / 65536L;
            j = j - hyph_next * 65536L;
        } else {
            hyph_next = 0;
        }

        if (j >= hyph_size || hyph_next > hyph_size)
            goto bad_fmt;

        hyph_link[j] = hyph_next;

        undump_int(x);
        if (x < 0 || x > str_ptr)
            goto bad_fmt;
        else
            hyph_word[j] = x;

        undump_int(x);
        if (x < MIN_HALFWORD || x > MAX_HALFWORD)
            goto bad_fmt;
        else
            hyph_list[j] = x;
    }

    j++;
    if (j < HYPH_PRIME)
        j = HYPH_PRIME;

    hyph_next = j;
    if (hyph_next >= hyph_size)
        hyph_next = HYPH_PRIME;
    else if (hyph_next >= HYPH_PRIME)
        hyph_next++;

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > trie_size)
        _tt_abort ("must increase trie_size");

    j = x;
    trie_max = j;

    undump_int(x);
    if (x < 0 || x > j)
        goto bad_fmt;
    else
        hyph_start = x;

    if (!trie_trl)
        trie_trl = xmalloc_array(trie_pointer, j + 1);
    undump_things(trie_trl[0], j + 1);

    if (!trie_tro)
        trie_tro = xmalloc_array(trie_pointer, j + 1);
    undump_things(trie_tro[0], j + 1);

    if (!trie_trc)
        trie_trc = xmalloc_array(uint16_t, j + 1);
    undump_things(trie_trc[0], j + 1);

    undump_int(max_hyph_char);

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > trie_op_size)
        _tt_abort ("must increase trie_op_size");

    j = x;
    trie_op_ptr = j;

    undump_things(hyf_distance[1], j);
    undump_things(hyf_num[1], j);
    undump_upper_check_things(max_trie_op, hyf_next[1], j);

    for (k = 0; k <= BIGGEST_LANG; k++)
        trie_used[k] = 0;

    k = BIGGEST_LANG + 1;

    while (j > 0) {
        undump_int(x);
        if (x < 0 || x > k - 1)
            goto bad_fmt;
        else
            k = x;

        undump_int(x);
        if (x < 1 || x > j)
            goto bad_fmt;
        else
            x = x;

        trie_used[k] = x;
        j = j - x;
        op_start[k] = j;
    }

    trie_not_ready = false;

    /* trailer */

    undump_int(x);
    if (x < BATCH_MODE || x > ERROR_STOP_MODE)
        goto bad_fmt;
    else
        interaction = x;

    undump_int(x);
    if (x < 0 || x > str_ptr)
        goto bad_fmt;
    else
        format_ident = x;

    undump_int(x);
    if (x != 69069L) /* magic value */
        goto bad_fmt;

    ttstub_input_close (fmt_in);
    return true;

bad_fmt:
    _tt_abort ("fatal format file error");
}


static void
final_cleanup(void)
{
    memory_word *mem = zmem;
    small_number c;

    c = cur_chr;
    if (job_name == 0)
        open_log_file();
    while (input_ptr > 0)
        if (cur_input.state == TOKEN_LIST)
            end_token_list();
        else
            end_file_reading();
    while (open_parens > 0) {

        print(S(___Z19/*" )"*/));
        open_parens--;
    }
    if (cur_level > LEVEL_ONE) {
        print_nl(40 /*"(" */ );
        print_esc(S(end_occurred_));
        print(S(inside_a_group_at_level_));
        print_int(cur_level - 1);
        print_char(41 /*")" */ );
        show_save_groups();
    }
    while (cond_ptr != MIN_HALFWORD) {

        print_nl(40 /*"(" */ );
        print_esc(S(end_occurred_));
        print(S(when_));
        print_cmd_chr(IF_TEST, cur_if);
        if (if_line != 0) {
            print(S(_on_line_));
            print_int(if_line);
        }
        print(S(_was_incomplete_));
        if_line = mem[cond_ptr + 1].cint;
        cur_if = mem[cond_ptr].hh.u.B1;
        temp_ptr = cond_ptr;
        cond_ptr = mem[cond_ptr].hh.v.RH;
        free_node(temp_ptr, IF_NODE_SIZE);
    }

    if (history != HISTORY_SPOTLESS) {
        if ((history == HISTORY_WARNING_ISSUED || (interaction < ERROR_STOP_MODE))) {

            if (selector == SELECTOR_TERM_AND_LOG) {
                selector = SELECTOR_TERM_ONLY;
                print_nl(S(_see_the_transcript_file_for/* additional information)*/));
                selector = SELECTOR_TERM_AND_LOG;
            }
        }
    }
    if (c == 1) {
        if (in_initex_mode) {
            {
                register integer for_end;
                c = TOP_MARK_CODE;
                for_end = SPLIT_BOT_MARK_CODE;
                if (c <= for_end)
                    do
                        if (cur_mark[c] != MIN_HALFWORD)
                            delete_token_ref(cur_mark[c]);
                    while (c++ < for_end) ;
            }
            if (sa_root[MARK_VAL] != MIN_HALFWORD) {

                if (do_marks(3, 0, sa_root[MARK_VAL]))
                    sa_root[MARK_VAL] = MIN_HALFWORD;
            }
            {
                register integer for_end;
                c = LAST_BOX_CODE;
                for_end = VSPLIT_CODE;
                if (c <= for_end)
                    do
                        flush_node_list(disc_ptr[c]);
                    while (c++ < for_end);
            }
            if (last_glue != MAX_HALFWORD)
                delete_glue_ref(last_glue);
            store_fmt_file();
            return;
        }
        print_nl(S(__dump_is_performed_only_by_/*INITEX)*/));
        return;
    }
}


/* Engine initialization */

static UFILE stdin_ufile;

static void
init_io(string input_file_name)
{
    /* This function used to be called init_terminal(), but since Tectonic
     * never reads from the terminal its actual role is now fairly
     * different. */

    int k;
    unsigned char *ptr = (unsigned char *) input_file_name;
    UInt32 rval;

    stdin_ufile.handle = NULL;
    stdin_ufile.savedChar = -1;
    stdin_ufile.skipNextLF = 0;
    stdin_ufile.encodingMode = UTF8;
    stdin_ufile.conversionData = 0;
    input_file[0] = &stdin_ufile;

    /* Hacky stuff that sets us up to process the input file, including UTF8
     * interpretation. */

    buffer[first] = 0;
    k = first;

    while ((rval = *(ptr++)) != 0) {
	UInt16 extraBytes = bytesFromUTF8[rval];

	switch (extraBytes) { /* note: code falls through cases! */
	case 5: rval <<= 6; if (*ptr) rval += *(ptr++);
	case 4: rval <<= 6; if (*ptr) rval += *(ptr++);
	case 3: rval <<= 6; if (*ptr) rval += *(ptr++);
	case 2: rval <<= 6; if (*ptr) rval += *(ptr++);
	case 1: rval <<= 6; if (*ptr) rval += *(ptr++);
	case 0: ;
	}

	rval -= offsetsFromUTF8[extraBytes];
	buffer[k++] = rval;
    }

    buffer[k] = ' ';
    last = k;
    cur_input.loc = first;
    cur_input.limit = last;
    first = last + 1;
}


static void
initialize_more_variables(void)
{
    memory_word *mem = zmem;
    integer i, k;
    hyph_pointer z;

    doing_special = false;
    native_text_size = 128;
    native_text = xmalloc(native_text_size * sizeof(UTF16_code));

    interaction = ERROR_STOP_MODE;

    deletions_allowed = true;
    set_box_allowed = true;
    error_count = 0;
    help_ptr = 0;
    use_err_help = false;

    nest_ptr = 0;
    max_nest_stack = 0;
    cur_list.mode = VMODE;
    cur_list.head = mem_top - 1;
    cur_list.tail = mem_top - 1;
    cur_list.eTeX_aux = MIN_HALFWORD;
    cur_list.aux.cint = IGNORE_DEPTH;
    cur_list.ml = 0;
    cur_list.pg = 0;
    shown_mode = 0;
    page_contents = EMPTY;
    page_tail = mem_top - 2;
    last_glue = MAX_HALFWORD;
    last_penalty = 0;
    last_kern = 0;
    page_so_far[7] = 0;
    page_max_depth = 0;

    for (k = INT_BASE; k <= EQTB_SIZE; k++)
        xeq_level[k] = LEVEL_ONE;

    no_new_control_sequence = true;
    prim[0].v.LH = 0;
    prim[0].v.RH = 0;

    for (k = 1; k <= PRIM_SIZE; k++)
        prim[k] = prim[0];

    prim_eqtb[0].hh.u.B1 = LEVEL_ZERO;
    prim_eqtb[0].hh.u.B0 = UNDEFINED_CS;
    prim_eqtb[0].hh.v.RH = MIN_HALFWORD;

    for (k = 1; k <= PRIM_SIZE; k++)
        prim_eqtb[k] = prim_eqtb[0];

    save_ptr = 0;
    cur_level = LEVEL_ONE;
    cur_group = BOTTOM_LEVEL;
    cur_boundary = 0;
    max_save_stack = 0;
    mag_set = 0;
    expand_depth_count = 0;
    is_in_csname = false;
    cur_mark[TOP_MARK_CODE] = MIN_HALFWORD;
    cur_mark[FIRST_MARK_CODE] = MIN_HALFWORD;
    cur_mark[BOT_MARK_CODE] = MIN_HALFWORD;
    cur_mark[SPLIT_FIRST_MARK_CODE] = MIN_HALFWORD;
    cur_mark[SPLIT_BOT_MARK_CODE] = MIN_HALFWORD;
    cur_val = 0;
    cur_val_level = INT_VAL;
    radix = 0;
    cur_order = NORMAL;

    for (k = 0; k <= 16; k++)
        read_open[k] = CLOSED;

    cond_ptr = MIN_HALFWORD;
    if_limit = NORMAL;
    cur_if = 0;
    if_line = 0;
    null_character.u.B0 = 0;
    null_character.u.B1 = 0;
    null_character.u.B2 = 0;
    null_character.u.B3 = 0;
    total_pages = 0;
    max_v = 0;
    max_h = 0;
    max_push = 0;
    last_bop = -1;
    doing_leaders = false;
    dead_cycles = 0;
    cur_s = -1;
    half_buf = dvi_buf_size / 2;
    dvi_limit = dvi_buf_size;
    dvi_ptr = 0;
    dvi_offset = 0;
    dvi_gone = 0;
    down_ptr = MIN_HALFWORD;
    right_ptr = MIN_HALFWORD;
    adjust_tail = MIN_HALFWORD;
    last_badness = 0;
    pre_adjust_tail = MIN_HALFWORD;
    pack_begin_line = 0;
    empty.v.RH = EMPTY;
    empty.v.LH = MIN_HALFWORD;
    null_delimiter.u.B0 = 0;
    null_delimiter.u.B1 = 0;
    null_delimiter.u.B2 = 0;
    null_delimiter.u.B3 = 0;
    align_ptr = MIN_HALFWORD;
    cur_align = MIN_HALFWORD;
    cur_span = MIN_HALFWORD;
    cur_loop = MIN_HALFWORD;
    cur_head = MIN_HALFWORD;
    cur_tail = MIN_HALFWORD;
    cur_pre_head = MIN_HALFWORD;
    cur_pre_tail = MIN_HALFWORD;
    max_hyph_char = TOO_BIG_LANG;

    for (z = 0; z <= hyph_size; z++) {
        hyph_word[z] = 0;
        hyph_list[z] = MIN_HALFWORD;
        hyph_link[z] = 0;
    }

    hyph_count = 0;
    hyph_next = (HYPH_PRIME + 1);
    if (hyph_next > hyph_size)
        hyph_next = HYPH_PRIME;

    output_active = false;
    insert_penalties = 0;
    ligature_present = false;
    cancel_boundary = false;
    lft_hit = false;
    rt_hit = false;
    ins_disc = false;
    after_token = 0;
    long_help_seen = false;
    format_ident = 0;

    for (k = 0; k <= 17; k++)
        write_open[k] = false;

    LR_ptr = MIN_HALFWORD;
    LR_problems = 0;
    cur_dir = LEFT_TO_RIGHT;
    pseudo_files = MIN_HALFWORD;
    sa_root[MARK_VAL] = MIN_HALFWORD;
    sa_null.hh.v.LH = MIN_HALFWORD;
    sa_null.hh.v.RH = MIN_HALFWORD;
    sa_chain = MIN_HALFWORD;
    sa_level = LEVEL_ZERO;
    disc_ptr[LAST_BOX_CODE] = MIN_HALFWORD;
    disc_ptr[VSPLIT_CODE] = MIN_HALFWORD;
    edit_name_start = 0;
    stop_at_space = true;
}

static void
initialize_more_initex_variables(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer i, k;

    for (k = 1; k <= 19; k++)
        mem[k].cint = 0;

    for (k = 0; k <= 19; k += 4) {
        mem[k].hh.v.RH = MIN_HALFWORD + 1;
        mem[k].hh.u.B0 = NORMAL;
        mem[k].hh.u.B1 = NORMAL;
    }

    mem[6].cint = 65536L;
    mem[4].hh.u.B0 = FIL;
    mem[10].cint = 65536L;
    mem[8].hh.u.B0 = FILL;
    mem[14].cint = 65536L;
    mem[12].hh.u.B0 = FIL;
    mem[15].cint = 65536L;
    mem[12].hh.u.B1 = FIL;
    mem[18].cint = -65536L;
    mem[16].hh.u.B0 = FIL;
    rover = 20;
    mem[rover].hh.v.RH = MAX_HALFWORD;
    mem[rover].hh.v.LH = 1000;
    mem[rover + 1].hh.v.LH = rover;
    mem[rover + 1].hh.v.RH = rover;
    lo_mem_max = rover + 1000;
    mem[lo_mem_max].hh.v.RH = MIN_HALFWORD;
    mem[lo_mem_max].hh.v.LH = MIN_HALFWORD;

    for (k = mem_top - 14; k <= mem_top; k++)
        mem[k] = mem[lo_mem_max];

    mem[mem_top - 10].hh.v.LH = CS_TOKEN_FLAG + FROZEN_END_TEMPLATE;
    mem[mem_top - 9].hh.v.RH = UINT16_MAX + 1;
    mem[mem_top - 9].hh.v.LH = MIN_HALFWORD;
    mem[mem_top - 7].hh.u.B0 = HYPHENATED;
    mem[mem_top - 6].hh.v.LH = MAX_HALFWORD;
    mem[mem_top - 7].hh.u.B1 = 0;
    mem[mem_top].hh.u.B1 = 255;
    mem[mem_top].hh.u.B0 = SPLIT_UP;
    mem[mem_top].hh.v.RH = mem_top;
    mem[mem_top - 2].hh.u.B0 = GLUE_NODE;
    mem[mem_top - 2].hh.u.B1 = NORMAL;
    avail = MIN_HALFWORD;
    mem_end = mem_top;
    hi_mem_min = mem_top - 14;
    var_used = 20;
    dyn_used = HI_MEM_STAT_USAGE;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.u.B0 = UNDEFINED_CS;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.v.RH = MIN_HALFWORD;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.u.B1 = LEVEL_ZERO;

    for (k = ACTIVE_BASE; k <= eqtb_top; k++)
        eqtb[k] = eqtb[UNDEFINED_CONTROL_SEQUENCE];

    eqtb[GLUE_BASE].hh.v.RH = 0;
    eqtb[GLUE_BASE].hh.u.B1 = LEVEL_ONE;
    eqtb[GLUE_BASE].hh.u.B0 = GLUE_REF;

    for (k = GLUE_BASE + 1; k <= LOCAL_BASE - 1; k++)
        eqtb[k] = eqtb[GLUE_BASE];

    mem[0].hh.v.RH += 531;
    LOCAL(par_shape) = MIN_HALFWORD;
    eqtb[LOCAL_BASE + LOCAL__par_shape].hh.u.B0 = SHAPE_REF;
    eqtb[LOCAL_BASE + LOCAL__par_shape].hh.u.B1 = LEVEL_ONE;

    for (k = ETEX_PEN_BASE; k <= ETEX_PENS - 1; k++)
        eqtb[k] = eqtb[LOCAL_BASE + LOCAL__par_shape];

    for (k = LOCAL_BASE + LOCAL__output_routine; k <= TOKS_BASE + NUMBER_REGS - 1; k++)
        eqtb[k] = eqtb[UNDEFINED_CONTROL_SEQUENCE];

    eqtb[BOX_BASE].hh.v.RH = MIN_HALFWORD;
    eqtb[BOX_BASE].hh.u.B0 = BOX_REF;
    eqtb[BOX_BASE].hh.u.B1 = LEVEL_ONE;

    for (k = BOX_BASE + 1; k <= BOX_BASE + NUMBER_REGS - 1; k++)
        eqtb[k] = eqtb[BOX_BASE];

    eqtb[CUR_FONT_LOC].hh.v.RH = FONT_BASE;
    eqtb[CUR_FONT_LOC].hh.u.B0 = DATA;
    eqtb[CUR_FONT_LOC].hh.u.B1 = LEVEL_ONE;

    for (k = MATH_FONT_BASE; k <= MATH_FONT_BASE + NUMBER_MATH_FONTS - 1; k++)
        eqtb[k] = eqtb[CUR_FONT_LOC];

    eqtb[CAT_CODE_BASE].hh.v.RH = 0;
    eqtb[CAT_CODE_BASE].hh.u.B0 = DATA;
    eqtb[CAT_CODE_BASE].hh.u.B1 = LEVEL_ONE;

    for (k = CAT_CODE_BASE + 1; k <= INT_BASE - 1; k++)
        eqtb[k] = eqtb[CAT_CODE_BASE];

    for (k = 0; k <= NUMBER_USVS - 1; k++) {
        CAT_CODE(k) = OTHER_CHAR;
        MATH_CODE(k) = k;
        SF_CODE(k) = 1000;
    }

    CAT_CODE(13) = CAR_RET;
    CAT_CODE(32) = SPACER;
    CAT_CODE(92) = ESCAPE;
    CAT_CODE(37) = COMMENT;
    CAT_CODE(127) = INVALID_CHAR;
    eqtb[CAT_CODE_BASE].hh.v.RH = IGNORE;

    for (k = 48 /*"0" */; k <= 57 /*"9" */; k++)
        MATH_CODE(k) = k + set_class(VAR_FAM_CLASS);

    for (k = 65 /*"A" */; k <= 90 /*"Z" */; k++) {
        CAT_CODE(k) = LETTER;
        CAT_CODE(k + 32) = LETTER;
        MATH_CODE(k) = k + set_family(1) + set_class(VAR_FAM_CLASS);
        MATH_CODE(k + 32) = k + 32 + set_family(1) + set_class(VAR_FAM_CLASS);
        LC_CODE(k) = k + 32;
        LC_CODE(k + 32) = k + 32;
        UC_CODE(k) = k;
        UC_CODE(k + 32) = k;
        SF_CODE(k) = 999;
    }

    for (k = INT_BASE; k <= DEL_CODE_BASE - 1; k++)
        eqtb[k].cint = 0;

    INTPAR(char_sub_def_min) = 256;
    INTPAR(char_sub_def_max) = -1;
    INTPAR(mag) = 1000;
    INTPAR(tolerance) = 10000;
    INTPAR(hang_after) = 1;
    INTPAR(max_dead_cycles) = 25;
    INTPAR(escape_char) = 92 /*"\" */ ;
    INTPAR(end_line_char) = CARRIAGE_RETURN;

    for (k = 0; k <= NUMBER_CHARS - 1; k++)
        DEL_CODE(k) = -1;

    DEL_CODE(46) = 0;

    for (k = DIMEN_BASE; k <= EQTB_SIZE; k++)
        eqtb[k].cint = 0;

    prim_used = PRIM_SIZE;
    hash_used = FROZEN_CONTROL_SEQUENCE;
    hash_high = 0;
    cs_count = 0;
    eqtb[FROZEN_DONT_EXPAND].hh.u.B0 = DONT_EXPAND;
    hash[FROZEN_DONT_EXPAND].v.RH = S(notexpanded_);
    eqtb[FROZEN_PRIMITIVE].hh.u.B0 = IGNORE_SPACES;
    eqtb[FROZEN_PRIMITIVE].hh.v.RH = 1;
    eqtb[FROZEN_PRIMITIVE].hh.u.B1 = LEVEL_ONE;
    hash[FROZEN_PRIMITIVE].v.RH = S(primitive);

    for (k = -(integer) trie_op_size; k <= trie_op_size; k++)
        trie_op_hash[k] = 0;

    for (k = 0; k <= BIGGEST_LANG; k++)
        trie_used[k] = min_trie_op;

    max_op_used = min_trie_op;
    trie_op_ptr = 0;
    trie_not_ready = true;
    hash[FROZEN_PROTECTION].v.RH = S(inaccessible);

    format_ident = S(__INITEX_);

    hash[END_WRITE].v.RH = S(endwrite);
    eqtb[END_WRITE].hh.u.B1 = LEVEL_ONE;
    eqtb[END_WRITE].hh.u.B0 = OUTER_CALL;
    eqtb[END_WRITE].hh.v.RH = MIN_HALFWORD;

    max_reg_num = 32767;
    max_reg_help_line = S(A_register_number_must_be_be_Z1);

    for (i = INT_VAL; i <= INTER_CHAR_VAL; i++)
        sa_root[i] = MIN_HALFWORD;

    STATEINT(xetex_hyphenatable_length) = 63;
}


/*:1370*//*1371: */
static void
initialize_primitives(void)
{
    CACHE_THE_EQTB;

    no_new_control_sequence = false;
    first = 0;

    primitive(S(lineskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__line_skip);
    primitive(S(baselineskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__baseline_skip);
    primitive(S(parskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__par_skip);
    primitive(S(abovedisplayskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__above_display_skip);
    primitive(S(belowdisplayskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__below_display_skip);
    primitive(S(abovedisplayshortskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__above_display_short_skip);
    primitive(S(belowdisplayshortskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__below_display_short_skip);
    primitive(S(leftskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__left_skip);
    primitive(S(rightskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__right_skip);
    primitive(S(topskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__top_skip);
    primitive(S(splittopskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__split_top_skip);
    primitive(S(tabskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__tab_skip);
    primitive(S(spaceskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__space_skip);
    primitive(S(xspaceskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__xspace_skip);
    primitive(S(parfillskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__par_fill_skip);
    primitive(S(XeTeXlinebreakskip), ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__xetex_linebreak_skip);

    primitive(S(thinmuskip), ASSIGN_MU_GLUE, GLUE_BASE + GLUE_PAR__thin_mu_skip);
    primitive(S(medmuskip), ASSIGN_MU_GLUE, GLUE_BASE + GLUE_PAR__med_mu_skip);
    primitive(S(thickmuskip), ASSIGN_MU_GLUE, GLUE_BASE + GLUE_PAR__thick_mu_skip);

    primitive(S(output), ASSIGN_TOKS, LOCAL_BASE + LOCAL__output_routine);
    primitive(S(everypar), ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_par);
    primitive(S(everymath), ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_math);
    primitive(S(everydisplay), ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_display);
    primitive(S(everyhbox), ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_hbox);
    primitive(S(everyvbox), ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_vbox);
    primitive(S(everyjob), ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_job);
    primitive(S(everycr), ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_cr);
    primitive(S(errhelp), ASSIGN_TOKS, LOCAL_BASE + LOCAL__err_help);

    primitive(S(pretolerance), ASSIGN_INT, INT_BASE + 0);
    primitive(S(tolerance), ASSIGN_INT, INT_BASE + 1);
    primitive(S(linepenalty), ASSIGN_INT, INT_BASE + 2);
    primitive(S(hyphenpenalty), ASSIGN_INT, INT_BASE + 3);
    primitive(S(exhyphenpenalty), ASSIGN_INT, INT_BASE + 4);
    primitive(S(clubpenalty), ASSIGN_INT, INT_BASE + 5);
    primitive(S(widowpenalty), ASSIGN_INT, INT_BASE + 6);
    primitive(S(displaywidowpenalty), ASSIGN_INT, INT_BASE + 7);
    primitive(S(brokenpenalty), ASSIGN_INT, INT_BASE + 8);
    primitive(S(binoppenalty), ASSIGN_INT, INT_BASE + 9);
    primitive(S(relpenalty), ASSIGN_INT, INT_BASE + 10);
    primitive(S(predisplaypenalty), ASSIGN_INT, INT_BASE + 11);
    primitive(S(postdisplaypenalty), ASSIGN_INT, INT_BASE + 12);
    primitive(S(interlinepenalty), ASSIGN_INT, INT_BASE + 13);
    primitive(S(doublehyphendemerits), ASSIGN_INT, INT_BASE + 14);
    primitive(S(finalhyphendemerits), ASSIGN_INT, INT_BASE + 15);
    primitive(S(adjdemerits), ASSIGN_INT, INT_BASE + 16);
    primitive(S(mag), ASSIGN_INT, INT_BASE + 17);
    primitive(S(delimiterfactor), ASSIGN_INT, INT_BASE + 18);
    primitive(S(looseness), ASSIGN_INT, INT_BASE + 19);
    primitive(S(time), ASSIGN_INT, INT_BASE + 20);
    primitive(S(day), ASSIGN_INT, INT_BASE + 21);
    primitive(S(month), ASSIGN_INT, INT_BASE + 22);
    primitive(S(year), ASSIGN_INT, INT_BASE + 23);
    primitive(S(showboxbreadth), ASSIGN_INT, INT_BASE + 24);
    primitive(S(showboxdepth), ASSIGN_INT, INT_BASE + 25);
    primitive(S(hbadness), ASSIGN_INT, INT_BASE + 26);
    primitive(S(vbadness), ASSIGN_INT, INT_BASE + 27);
    primitive(S(pausing), ASSIGN_INT, INT_BASE + 28);
    primitive(S(tracingonline), ASSIGN_INT, INT_BASE + 29);
    primitive(S(tracingmacros), ASSIGN_INT, INT_BASE + 30);
    primitive(S(tracingstats), ASSIGN_INT, INT_BASE + 31);
    primitive(S(tracingparagraphs), ASSIGN_INT, INT_BASE + 32);
    primitive(S(tracingpages), ASSIGN_INT, INT_BASE + 33);
    primitive(S(tracingoutput), ASSIGN_INT, INT_BASE + 34);
    primitive(S(tracinglostchars), ASSIGN_INT, INT_BASE + 35);
    primitive(S(tracingcommands), ASSIGN_INT, INT_BASE + 36);
    primitive(S(tracingrestores), ASSIGN_INT, INT_BASE + 37);
    primitive(S(uchyph), ASSIGN_INT, INT_BASE + 38);
    primitive(S(outputpenalty), ASSIGN_INT, INT_BASE + 39);
    primitive(S(maxdeadcycles), ASSIGN_INT, INT_BASE + 40);
    primitive(S(hangafter), ASSIGN_INT, INT_BASE + 41);
    primitive(S(floatingpenalty), ASSIGN_INT, INT_BASE + 42);
    primitive(S(globaldefs), ASSIGN_INT, INT_BASE + 43);
    primitive(S(fam), ASSIGN_INT, INT_BASE + 44);
    primitive(S(escapechar), ASSIGN_INT, INT_BASE + 45);
    primitive(S(defaulthyphenchar), ASSIGN_INT, INT_BASE + 46);
    primitive(S(defaultskewchar), ASSIGN_INT, INT_BASE + 47);
    primitive(S(endlinechar), ASSIGN_INT, INT_BASE + 48);
    primitive(S(newlinechar), ASSIGN_INT, INT_BASE + 49);
    primitive(S(language), ASSIGN_INT, INT_BASE + 50);
    primitive(S(lefthyphenmin), ASSIGN_INT, INT_BASE + 51);
    primitive(S(righthyphenmin), ASSIGN_INT, INT_BASE + 52);
    primitive(S(holdinginserts), ASSIGN_INT, INT_BASE + 53);
    primitive(S(errorcontextlines), ASSIGN_INT, INT_BASE + 54);

    primitive(S(XeTeXlinebreakpenalty), ASSIGN_INT, INT_BASE + 69);
    primitive(S(XeTeXprotrudechars), ASSIGN_INT, INT_BASE + 70);

    primitive(S(parindent), ASSIGN_DIMEN, DIMEN_BASE + 0);
    primitive(S(mathsurround), ASSIGN_DIMEN, DIMEN_BASE + 1);
    primitive(S(lineskiplimit), ASSIGN_DIMEN, DIMEN_BASE + 2);
    primitive(S(hsize), ASSIGN_DIMEN, DIMEN_BASE + 3);
    primitive(S(vsize), ASSIGN_DIMEN, DIMEN_BASE + 4);
    primitive(S(maxdepth), ASSIGN_DIMEN, DIMEN_BASE + 5);
    primitive(S(splitmaxdepth), ASSIGN_DIMEN, DIMEN_BASE + 6);
    primitive(S(boxmaxdepth), ASSIGN_DIMEN, DIMEN_BASE + 7);
    primitive(S(hfuzz), ASSIGN_DIMEN, DIMEN_BASE + 8);
    primitive(S(vfuzz), ASSIGN_DIMEN, DIMEN_BASE + 9);
    primitive(S(delimitershortfall), ASSIGN_DIMEN, DIMEN_BASE + 10);
    primitive(S(nulldelimiterspace), ASSIGN_DIMEN, DIMEN_BASE + 11);
    primitive(S(scriptspace), ASSIGN_DIMEN, DIMEN_BASE + 12);
    primitive(S(predisplaysize), ASSIGN_DIMEN, DIMEN_BASE + 13);
    primitive(S(displaywidth), ASSIGN_DIMEN, DIMEN_BASE + 14);
    primitive(S(displayindent), ASSIGN_DIMEN, DIMEN_BASE + 15);
    primitive(S(overfullrule), ASSIGN_DIMEN, DIMEN_BASE + 16);
    primitive(S(hangindent), ASSIGN_DIMEN, DIMEN_BASE + 17);
    primitive(S(hoffset), ASSIGN_DIMEN, DIMEN_BASE + 18);
    primitive(S(voffset), ASSIGN_DIMEN, DIMEN_BASE + 19);
    primitive(S(emergencystretch), ASSIGN_DIMEN, DIMEN_BASE + 20);
    primitive(S(pdfpagewidth), ASSIGN_DIMEN, DIMEN_BASE + 21);
    primitive(S(pdfpageheight), ASSIGN_DIMEN, DIMEN_BASE + 22);

    primitive(32 /*" " */, EX_SPACE, 0);
    primitive(47 /*"/" */, ITAL_CORR, 0);
    primitive(S(accent), ACCENT, 0);
    primitive(S(advance), ADVANCE, 0);
    primitive(S(afterassignment), AFTER_ASSIGNMENT, 0);
    primitive(S(aftergroup), AFTER_GROUP, 0);
    primitive(S(begingroup), BEGIN_GROUP, 0);
    primitive(S(char), CHAR_NUM, 0);
    primitive(S(csname), CS_NAME, 0);
    primitive(S(delimiter), DELIM_NUM, 0);
    primitive(S(XeTeXdelimiter), DELIM_NUM, 1);
    primitive(S(Udelimiter), DELIM_NUM, 1);
    primitive(S(divide), DIVIDE, 0);
    primitive(S(endcsname), END_CS_NAME, 0);
    primitive(S(endgroup), END_GROUP, 0);
    hash[FROZEN_END_GROUP].v.RH = S(endgroup);
    eqtb[FROZEN_END_GROUP] = eqtb[cur_val];
    primitive(S(expandafter), EXPAND_AFTER, 0);
    primitive(S(font), DEF_FONT, 0);
    primitive(S(fontdimen), ASSIGN_FONT_DIMEN, 0);
    primitive(S(halign), HALIGN, 0);
    primitive(S(hrule), HRULE, 0);
    primitive(S(ignorespaces), IGNORE_SPACES, 0);
    primitive(S(insert), INSERT, 0);
    primitive(S(mark), MARK, 0);
    primitive(S(mathaccent), MATH_ACCENT, 0);
    primitive(S(XeTeXmathaccent), MATH_ACCENT, 1);
    primitive(S(Umathaccent), MATH_ACCENT, 1);
    primitive(S(mathchar), MATH_CHAR_NUM, 0);
    primitive(S(XeTeXmathcharnum), MATH_CHAR_NUM, 1);
    primitive(S(Umathcharnum), MATH_CHAR_NUM, 1);
    primitive(S(XeTeXmathchar), MATH_CHAR_NUM, 2);
    primitive(S(Umathchar), MATH_CHAR_NUM, 2);
    primitive(S(mathchoice), MATH_CHOICE, 0);
    primitive(S(multiply), MULTIPLY, 0);
    primitive(S(noalign), NO_ALIGN, 0);
    primitive(S(noboundary), NO_BOUNDARY, 0);
    primitive(S(noexpand), NO_EXPAND, 0);
    primitive(S(primitive), NO_EXPAND, 1);
    primitive(S(nonscript), NON_SCRIPT, 0);
    primitive(S(omit), OMIT, 0);
    primitive(S(parshape), SET_SHAPE, LOCAL_BASE + LOCAL__par_shape);
    primitive(S(penalty), BREAK_PENALTY, 0);
    primitive(S(prevgraf), SET_PREV_GRAF, 0);
    primitive(S(radical), RADICAL, 0);
    primitive(S(XeTeXradical), RADICAL, 1);
    primitive(S(Uradical), RADICAL, 1);
    primitive(S(read), READ_TO_CS, 0);
    primitive(S(relax), RELAX, TOO_BIG_USV);
    hash[FROZEN_RELAX].v.RH = S(relax);
    eqtb[FROZEN_RELAX] = eqtb[cur_val];
    primitive(S(setbox), SET_BOX, 0);
    primitive(S(the), THE, 0);
    primitive(S(toks), TOKS_REGISTER, 0);
    primitive(S(vadjust), VADJUST, 0);
    primitive(S(valign), VALIGN, 0);
    primitive(S(vcenter), VCENTER, 0);
    primitive(S(vrule), VRULE, 0);
    primitive(S(par), PAR_END, TOO_BIG_USV);
    par_loc = cur_val;
    par_token = CS_TOKEN_FLAG + par_loc;

    primitive(S(input), INPUT, 0);
    primitive(S(endinput), INPUT, 1);

    primitive(S(topmark), TOP_BOT_MARK, TOP_MARK_CODE);
    primitive(S(firstmark), TOP_BOT_MARK, FIRST_MARK_CODE);
    primitive(S(botmark), TOP_BOT_MARK, BOT_MARK_CODE);
    primitive(S(splitfirstmark), TOP_BOT_MARK, SPLIT_FIRST_MARK_CODE);
    primitive(S(splitbotmark), TOP_BOT_MARK, SPLIT_BOT_MARK_CODE);

    primitive(S(count), REGISTER, 0);
    primitive(S(dimen), REGISTER, 1);
    primitive(S(skip), REGISTER, 2);
    primitive(S(muskip), REGISTER, 3);

    primitive(S(spacefactor), SET_AUX, HMODE);
    primitive(S(prevdepth), SET_AUX, VMODE);

    primitive(S(deadcycles), SET_PAGE_INT, 0);
    primitive(S(insertpenalties), SET_PAGE_INT, 1);

    primitive(S(wd), SET_BOX_DIMEN, WIDTH_OFFSET);
    primitive(S(ht), SET_BOX_DIMEN, HEIGHT_OFFSET);
    primitive(S(dp), SET_BOX_DIMEN, DEPTH_OFFSET);

    primitive(S(lastpenalty), LAST_ITEM, INT_VAL);
    primitive(S(lastkern), LAST_ITEM, DIMEN_VAL);
    primitive(S(lastskip), LAST_ITEM, GLUE_VAL);
    primitive(S(inputlineno), LAST_ITEM, INPUT_LINE_NO_CODE);
    primitive(S(badness), LAST_ITEM, BADNESS_CODE);

    primitive(S(number), CONVERT, NUMBER_CODE);
    primitive(S(romannumeral), CONVERT, ROMAN_NUMERAL_CODE);
    primitive(S(string), CONVERT, STRING_CODE);
    primitive(S(meaning), CONVERT, MEANING_CODE);
    primitive(S(fontname), CONVERT, FONT_NAME_CODE);
    primitive(S(jobname), CONVERT, JOB_NAME_CODE);
    primitive(S(leftmarginkern), CONVERT, LEFT_MARGIN_KERN_CODE);
    primitive(S(rightmarginkern), CONVERT, RIGHT_MARGIN_KERN_CODE);
    primitive(S(Uchar), CONVERT, XETEX_UCHAR_CODE);
    primitive(S(Ucharcat), CONVERT, XETEX_UCHARCAT_CODE);

    primitive(S(if), IF_TEST, IF_CHAR_CODE);
    primitive(S(ifcat), IF_TEST, IF_CAT_CODE);
    primitive(S(ifnum), IF_TEST, IF_INT_CODE);
    primitive(S(ifdim), IF_TEST, IF_DIM_CODE);
    primitive(S(ifodd), IF_TEST, IF_ODD_CODE);
    primitive(S(ifvmode), IF_TEST, IF_VMODE_CODE);
    primitive(S(ifhmode), IF_TEST, IF_HMODE_CODE);
    primitive(S(ifmmode), IF_TEST, IF_MMODE_CODE);
    primitive(S(ifinner), IF_TEST, IF_INNER_CODE);
    primitive(S(ifvoid), IF_TEST, IF_VOID_CODE);
    primitive(S(ifhbox), IF_TEST, IF_HBOX_CODE);
    primitive(S(ifvbox), IF_TEST, IF_VBOX_CODE);
    primitive(S(ifx), IF_TEST, IFX_CODE);
    primitive(S(ifeof), IF_TEST, IF_EOF_CODE);
    primitive(S(iftrue), IF_TEST, IF_TRUE_CODE);
    primitive(S(iffalse), IF_TEST, IF_FALSE_CODE);
    primitive(S(ifcase), IF_TEST, IF_CASE_CODE);
    primitive(S(ifprimitive), IF_TEST, IF_PRIMITIVE_CODE);

    primitive(S(fi), FI_OR_ELSE, FI_CODE);
    hash[FROZEN_FI].v.RH = S(fi);
    eqtb[FROZEN_FI] = eqtb[cur_val];
    primitive(S(or), FI_OR_ELSE, OR_CODE);
    primitive(S(else), FI_OR_ELSE, ELSE_CODE);

    primitive(S(nullfont), SET_FONT, FONT_BASE);
    hash[FROZEN_NULL_FONT].v.RH = S(nullfont);
    eqtb[FROZEN_NULL_FONT] = eqtb[cur_val];

    primitive(S(span), TAB_MARK, SPAN_CODE);
    primitive(S(cr), CAR_RET, CR_CODE);
    hash[FROZEN_CR].v.RH = S(cr);
    eqtb[FROZEN_CR] = eqtb[cur_val];
    primitive(S(crcr), CAR_RET, CR_CR_CODE);

    hash[FROZEN_END_TEMPLATE].v.RH = S(endtemplate);
    hash[FROZEN_ENDV].v.RH = S(endtemplate);
    eqtb[FROZEN_ENDV].hh.u.B0 = ENDV;
    eqtb[FROZEN_ENDV].hh.v.RH = mem_top - 11;
    eqtb[FROZEN_ENDV].hh.u.B1 = LEVEL_ONE;
    eqtb[FROZEN_END_TEMPLATE] = eqtb[FROZEN_ENDV];
    eqtb[FROZEN_END_TEMPLATE].hh.u.B0 = END_TEMPLATE;

    primitive(S(pagegoal), SET_PAGE_DIMEN, 0);
    primitive(S(pagetotal), SET_PAGE_DIMEN, 1);
    primitive(S(pagestretch), SET_PAGE_DIMEN, 2);
    primitive(S(pagefilstretch), SET_PAGE_DIMEN, 3);
    primitive(S(pagefillstretch), SET_PAGE_DIMEN, 4);
    primitive(S(pagefilllstretch), SET_PAGE_DIMEN, 5);
    primitive(S(pageshrink), SET_PAGE_DIMEN, 6);
    primitive(S(pagedepth), SET_PAGE_DIMEN, 7);

    primitive(S(end), STOP, 0);
    primitive(S(dump), STOP, 1);

    primitive(S(hskip), HSKIP, SKIP_CODE);
    primitive(S(hfil), HSKIP, FIL_CODE);
    primitive(S(hfill), HSKIP, FILL_CODE);
    primitive(S(hss), HSKIP, SS_CODE);
    primitive(S(hfilneg), HSKIP, FIL_NEG_CODE);
    primitive(S(vskip), VSKIP, SKIP_CODE);
    primitive(S(vfil), VSKIP, FIL_CODE);
    primitive(S(vfill), VSKIP, FILL_CODE);
    primitive(S(vss), VSKIP, SS_CODE);
    primitive(S(vfilneg), VSKIP, FIL_NEG_CODE);
    primitive(S(mskip), MSKIP, MSKIP_CODE);

    primitive(S(kern), KERN, EXPLICIT);
    primitive(S(mkern), MKERN, MU_GLUE);
    primitive(S(moveleft), HMOVE, 1);
    primitive(S(moveright), HMOVE, 0);
    primitive(S(raise), VMOVE, 1);
    primitive(S(lower), VMOVE, 0);

    primitive(S(box), MAKE_BOX, BOX_CODE);
    primitive(S(copy), MAKE_BOX, COPY_CODE);
    primitive(S(lastbox), MAKE_BOX, LAST_BOX_CODE);
    primitive(S(vsplit), MAKE_BOX, VSPLIT_CODE);
    primitive(S(vtop), MAKE_BOX, VTOP_CODE);
    primitive(S(vbox), MAKE_BOX, VTOP_CODE + 1);
    primitive(S(hbox), MAKE_BOX, VTOP_CODE + 104);

    primitive(S(shipout), LEADER_SHIP, A_LEADERS - 1);
    primitive(S(leaders), LEADER_SHIP, A_LEADERS);
    primitive(S(cleaders), LEADER_SHIP, C_LEADERS);
    primitive(S(xleaders), LEADER_SHIP, X_LEADERS);

    primitive(S(indent), START_PAR, 1);
    primitive(S(noindent), START_PAR, 0);
    primitive(S(unpenalty), REMOVE_ITEM, PENALTY_NODE);
    primitive(S(unkern), REMOVE_ITEM, KERN_NODE);
    primitive(S(unskip), REMOVE_ITEM, GLUE_NODE);
    primitive(S(unhbox), UN_HBOX, BOX_CODE);
    primitive(S(unhcopy), UN_HBOX, COPY_CODE);
    primitive(S(unvbox), UN_VBOX, BOX_CODE);
    primitive(S(unvcopy), UN_VBOX, COPY_CODE);

    primitive(45 /*"-" */, DISCRETIONARY, 1);
    primitive(S(discretionary), DISCRETIONARY, 0);

    primitive(S(eqno), EQ_NO, 0);
    primitive(S(leqno), EQ_NO, 1);

    primitive(S(mathord), MATH_COMP, ORD_NOAD);
    primitive(S(mathop), MATH_COMP, OP_NOAD);
    primitive(S(mathbin), MATH_COMP, BIN_NOAD);
    primitive(S(mathrel), MATH_COMP, REL_NOAD);
    primitive(S(mathopen), MATH_COMP, OPEN_NOAD);
    primitive(S(mathclose), MATH_COMP, CLOSE_NOAD);
    primitive(S(mathpunct), MATH_COMP, PUNCT_NOAD);
    primitive(S(mathinner), MATH_COMP, INNER_NOAD);
    primitive(S(underline), MATH_COMP, UNDER_NOAD);
    primitive(S(overline), MATH_COMP, OVER_NOAD);

    primitive(S(displaylimits), LIMIT_SWITCH, NORMAL);
    primitive(S(limits), LIMIT_SWITCH, LIMITS);
    primitive(S(nolimits), LIMIT_SWITCH, NO_LIMITS);

    primitive(S(displaystyle), MATH_STYLE, DISPLAY_STYLE);
    primitive(S(textstyle), MATH_STYLE, TEXT_STYLE);
    primitive(S(scriptstyle), MATH_STYLE, SCRIPT_STYLE);
    primitive(S(scriptscriptstyle), MATH_STYLE, SCRIPT_SCRIPT_STYLE);

    primitive(S(above), ABOVE, ABOVE_CODE);
    primitive(S(over), ABOVE, OVER_CODE);
    primitive(S(atop), ABOVE, ATOP_CODE);
    primitive(S(abovewithdelims), ABOVE, DELIMITED_CODE + 0);
    primitive(S(overwithdelims), ABOVE, DELIMITED_CODE + 1);
    primitive(S(atopwithdelims), ABOVE, DELIMITED_CODE + 2);

    primitive(S(left), LEFT_RIGHT, LEFT_NOAD);
    primitive(S(right), LEFT_RIGHT, RIGHT_NOAD);
    hash[FROZEN_RIGHT].v.RH = S(right);
    eqtb[FROZEN_RIGHT] = eqtb[cur_val];

    primitive(S(long), PREFIX, 1);
    primitive(S(outer), PREFIX, 2);
    primitive(S(global), PREFIX, 4);
    primitive(S(def), DEF, 0);
    primitive(S(gdef), DEF, 1);
    primitive(S(edef), DEF, 2);
    primitive(S(xdef), DEF, 3);
    primitive(S(let), LET, NORMAL);
    primitive(S(futurelet), LET, NORMAL + 1);

    primitive(S(chardef), SHORTHAND_DEF, CHAR_DEF_CODE);
    primitive(S(mathchardef), SHORTHAND_DEF, MATH_CHAR_DEF_CODE);
    primitive(S(XeTeXmathcharnumdef), SHORTHAND_DEF, XETEX_MATH_CHAR_NUM_DEF_CODE);
    primitive(S(Umathcharnumdef), SHORTHAND_DEF, XETEX_MATH_CHAR_NUM_DEF_CODE);
    primitive(S(XeTeXmathchardef), SHORTHAND_DEF, XETEX_MATH_CHAR_DEF_CODE);
    primitive(S(Umathchardef), SHORTHAND_DEF, XETEX_MATH_CHAR_DEF_CODE);
    primitive(S(countdef), SHORTHAND_DEF, COUNT_DEF_CODE);
    primitive(S(dimendef), SHORTHAND_DEF, DIMEN_DEF_CODE);
    primitive(S(skipdef), SHORTHAND_DEF, SKIP_DEF_CODE);
    primitive(S(muskipdef), SHORTHAND_DEF, MU_SKIP_DEF_CODE);
    primitive(S(toksdef), SHORTHAND_DEF, TOKS_DEF_CODE);

    primitive(S(catcode), DEF_CODE, CAT_CODE_BASE);
    primitive(S(mathcode), DEF_CODE, MATH_CODE_BASE);
    primitive(S(XeTeXmathcodenum), XETEX_DEF_CODE, MATH_CODE_BASE);
    primitive(S(Umathcodenum), XETEX_DEF_CODE, MATH_CODE_BASE);
    primitive(S(XeTeXmathcode), XETEX_DEF_CODE, MATH_CODE_BASE + 1);
    primitive(S(Umathcode), XETEX_DEF_CODE, MATH_CODE_BASE + 1);
    primitive(S(lccode), DEF_CODE, LC_CODE_BASE);
    primitive(S(uccode), DEF_CODE, UC_CODE_BASE);
    primitive(S(sfcode), DEF_CODE, SF_CODE_BASE);
    primitive(S(XeTeXcharclass), XETEX_DEF_CODE, SF_CODE_BASE);
    primitive(S(delcode), DEF_CODE, DEL_CODE_BASE);
    primitive(S(XeTeXdelcodenum), XETEX_DEF_CODE, DEL_CODE_BASE);
    primitive(S(Udelcodenum), XETEX_DEF_CODE, DEL_CODE_BASE);
    primitive(S(XeTeXdelcode), XETEX_DEF_CODE, DEL_CODE_BASE + 1);
    primitive(S(Udelcode), XETEX_DEF_CODE, DEL_CODE_BASE + 1);

    primitive(S(textfont), DEF_FAMILY, MATH_FONT_BASE + TEXT_SIZE);
    primitive(S(scriptfont), DEF_FAMILY, MATH_FONT_BASE + SCRIPT_SIZE);
    primitive(S(scriptscriptfont), DEF_FAMILY, MATH_FONT_BASE + SCRIPT_SCRIPT_SIZE);

    primitive(S(hyphenation), HYPH_DATA, 0);
    primitive(S(patterns), HYPH_DATA, 1);

    primitive(S(hyphenchar), ASSIGN_FONT_INT, 0);
    primitive(S(skewchar), ASSIGN_FONT_INT, 1);
    primitive(S(lpcode), ASSIGN_FONT_INT, 2);
    primitive(S(rpcode), ASSIGN_FONT_INT, 3);

    primitive(S(batchmode), SET_INTERACTION, BATCH_MODE);
    primitive(S(nonstopmode), SET_INTERACTION, NONSTOP_MODE);
    primitive(S(scrollmode), SET_INTERACTION, SCROLL_MODE);
    primitive(S(errorstopmode), SET_INTERACTION, ERROR_STOP_MODE);

    primitive(S(openin), IN_STREAM, 1);
    primitive(S(closein), IN_STREAM, 0);
    primitive(S(message), MESSAGE, 0);
    primitive(S(errmessage), MESSAGE, 1);
    primitive(S(lowercase), CASE_SHIFT, LC_CODE_BASE);
    primitive(S(uppercase), CASE_SHIFT, UC_CODE_BASE);

    primitive(S(show), XRAY, SHOW_CODE);
    primitive(S(showbox), XRAY, SHOW_BOX_CODE);
    primitive(S(showthe), XRAY, SHOW_THE_CODE);
    primitive(S(showlists), XRAY, SHOW_LISTS);

    primitive(S(openout), EXTENSION, OPEN_NODE);
    primitive(S(write), EXTENSION, WRITE_NODE);
    write_loc = cur_val;
    primitive(S(closeout), EXTENSION, CLOSE_NODE);
    primitive(S(special), EXTENSION, SPECIAL_NODE);
    hash[FROZEN_SPECIAL].v.RH = S(special);
    eqtb[FROZEN_SPECIAL] = eqtb[cur_val];
    primitive(S(immediate), EXTENSION, IMMEDIATE_CODE);
    primitive(S(setlanguage), EXTENSION, SET_LANGUAGE_CODE);

    primitive(S(synctex), ASSIGN_INT, INT_BASE + 83);

    no_new_control_sequence = true;
}


static void
get_strings_started(void)
{
    pool_ptr = 0;
    str_ptr = 0;
    str_start[0] = 0;
    str_ptr = TOO_BIG_CHAR;

    if (load_pool_strings(pool_size - string_vacancies) == 0)
	_tt_abort ("must increase pool_size");
}
/*:1001*/


/* Initialization bits that were in the C driver code */


void
tt_misc_initialize(char *dump_name)
{
    /* Miscellaneous initializations that were mostly originally done in the
     * main() driver routines. */

    /* Get our stdout handle */

    rust_stdout = ttstub_output_open_stdout ();

    /* TEX_format_default must get a leading space character for Pascal
     * style string magic. */

    size_t len = strlen (dump_name);
    TEX_format_default = xmalloc (len + 2);
    TEX_format_default[0] = ' ';
    strcpy (TEX_format_default + 1, dump_name);
    format_default_length = len + 2;

    /* Not sure why these get custom initializations. */

    synctex_options = INT_MAX;

    if (file_line_error_style_p < 0)
	file_line_error_style_p = 0;

    /* Make this something invariant so that we can use XDV files to test
     * reproducibility of the engine output. */

    output_comment = "tectonic";
}

/*:1371*//*1373: */

/* setjmp handing of fatal errors. I tried to compartmentalize this code in
 * errors.c but it seems that wrapping setjmp() in a little function does not
 * work. */

#define BUF_SIZE 1024

static jmp_buf jump_buffer;
static char error_buf[BUF_SIZE] = "";

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


tt_history_t
tt_run_engine(char *input_file_name)
{
    CACHE_THE_EQTB;

    /* Before anything else ... setjmp handling of super-fatal errors */

    if (setjmp (jump_buffer)) {
	history = HISTORY_FATAL_ERROR;
	return history;
    }

    /* These various parameters were configurable in web2c TeX. We don't
     * bother to allow that. */

    pool_size = 6250000L;
    string_vacancies = 90000L;
    pool_free = 47500L;
    max_strings = 565536L;
    strings_free = 100;
    font_mem_size = 8000000L;
    font_max = 9000;
    trie_size = 1000000L;
    hyph_size = 8191;
    buf_size = 200000L;
    nest_size = 500;
    max_in_open = 15;
    param_size = 10000;
    save_size = 80000L;
    stack_size = 5000;
    dvi_buf_size = 16384;
    error_line = 79;
    half_error_line = 50;
    max_print_line = 79;
    hash_extra = 600000L;
    expand_depth = 10000;

    mem_top = 4999999; /* the size of our main "mem" array, minus 1 */

    /* Allocate many of our big arrays. */

    buffer = xmalloc_array(UnicodeScalar, buf_size);
    nest = xmalloc_array(list_state_record, nest_size);
    save_stack = xmalloc_array(memory_word, save_size);
    input_stack = xmalloc_array(input_state_t, stack_size);
    input_file = xmalloc_array(UFILE *, max_in_open);
    line_stack = xmalloc_array(integer, max_in_open);
    eof_seen = xmalloc_array(boolean, max_in_open);
    grp_stack = xmalloc_array(save_pointer, max_in_open);
    if_stack = xmalloc_array(int32_t, max_in_open);
    source_filename_stack = xmalloc_array(str_number, max_in_open);
    full_source_filename_stack = xmalloc_array(str_number, max_in_open);
    param_stack = xmalloc_array(int32_t, param_size);
    dvi_buf = xmalloc_array(eight_bits, dvi_buf_size);
    hyph_word = xmalloc_array(str_number, hyph_size);
    hyph_list = xmalloc_array(int32_t, hyph_size);
    hyph_link = xmalloc_array(hyph_pointer, hyph_size);

    /* First bit of initex handling: more allocations. */

    if (in_initex_mode) {
        yzmem = xmalloc_array(memory_word, mem_top + 1);
        zmem = yzmem;
        eqtb_top = EQTB_SIZE + hash_extra;

        if (hash_extra == 0)
            hash_top = UNDEFINED_CONTROL_SEQUENCE;
        else
            hash_top = eqtb_top;

        yhash = xmalloc_array(two_halves, 1 + hash_top - hash_offset);
        hash = yhash - hash_offset;
        hash[HASH_BASE].v.LH = 0;
        hash[HASH_BASE].v.RH = 0;

        for (hash_used = HASH_BASE + 1; hash_used <= hash_top; hash_used++)
            hash[hash_used] = hash[HASH_BASE];

        the_eqtb = xcalloc_array(memory_word, eqtb_top);
        str_start = xmalloc_array(pool_pointer, max_strings);
        str_pool = xmalloc_array(packed_UTF16_code, pool_size);
        font_info = xmalloc_array(fmemory_word, font_mem_size);
    }

    /* Sanity-check various invariants. */

    history = HISTORY_FATAL_ERROR;
    bad = 0;

    if (half_error_line < 30 || half_error_line > error_line - 15)
        bad = 1;
    if (max_print_line < 60)
        bad = 2;
    if (dvi_buf_size % 8 != 0)
        bad = 3;
    if (1100 > mem_top)
        bad = 4;
    if (HASH_PRIME > HASH_SIZE)
        bad = 5;
    if (max_in_open >= 128)
        bad = 6;
    if (mem_top < 267)
        bad = 7;
    if (MIN_HALFWORD > 0)
        bad = 12;
    if (MAX_FONT_MAX < MIN_HALFWORD || MAX_FONT_MAX > MAX_HALFWORD)
        bad = 15;
    if (font_max > FONT_BASE + 9000)
        bad = 16;
    if (save_size > MAX_HALFWORD || max_strings > MAX_HALFWORD)
        bad = 17;
    if (buf_size > MAX_HALFWORD)
        bad = 18;
    if (CS_TOKEN_FLAG + EQTB_SIZE + hash_extra > MAX_HALFWORD)
        bad = 21;
    if (hash_offset < 0 || hash_offset > HASH_BASE)
        bad = 42;
    if (format_default_length > INTEGER_MAX)
        bad = 31;
    if (2 * MAX_HALFWORD < mem_top)
        bad = 41;

    if (bad > 0)
	_tt_abort ("failed internal consistency check #%d", bad);

    /* OK, ready to keep on initializing. */

    initialize_more_variables();

    if (in_initex_mode) {
        initialize_more_initex_variables();
        get_strings_started();
        initialize_primitives();
        init_str_ptr = str_ptr;
        init_pool_ptr = pool_ptr;
    }

    /*55:*/
    selector = SELECTOR_TERM_ONLY;
    tally = 0;
    term_offset = 0;
    file_offset = 0;
    job_name = 0;
    name_in_progress = false;
    log_opened = false;
    output_file_name = 0;
    output_file_extension = S(_xdv);
    input_ptr = 0;
    max_in_stack = 0;
    source_filename_stack[0] = 0;
    full_source_filename_stack[0] = 0;
    in_open = 0;
    open_parens = 0;
    max_buf_stack = 0;
    grp_stack[0] = 0;
    if_stack[0] = MIN_HALFWORD;
    param_ptr = 0;
    max_param_stack = 0;

    memset(buffer, 0, buf_size * sizeof(buffer[0]));
    first = 0;

    scanner_status = NORMAL;
    warning_index = MIN_HALFWORD;
    first = 1;
    cur_input.state = NEW_LINE;
    cur_input.start = 1;
    cur_input.index = 0;
    line = 0;
    cur_input.name = 0;
    force_eof = false;
    align_state = 1000000L;

    init_io(input_file_name);

    if (in_initex_mode) {
	no_new_control_sequence = false;

	primitive(S(XeTeXpicfile), EXTENSION, PIC_FILE_CODE);
	primitive(S(XeTeXpdffile), EXTENSION, PDF_FILE_CODE);
	primitive(S(XeTeXglyph), EXTENSION, GLYPH_CODE);
	primitive(S(XeTeXlinebreaklocale), EXTENSION, XETEX_LINEBREAK_LOCALE_EXTENSION_CODE);
	primitive(S(XeTeXinterchartoks), ASSIGN_TOKS, LOCAL_BASE + LOCAL__xetex_inter_char);
	primitive(S(pdfsavepos), EXTENSION, PDFTEX_FIRST_EXTENSION_CODE + 0);

	primitive(S(lastnodetype), LAST_ITEM, LAST_NODE_TYPE_CODE);
	primitive(S(eTeXversion), LAST_ITEM, ETEX_VERSION_CODE);

	primitive(S(eTeXrevision), CONVERT, ETEX_REVISION_CODE);

	primitive(S(XeTeXversion), LAST_ITEM, XETEX_VERSION_CODE);

	primitive(S(XeTeXrevision), CONVERT, XETEX_REVISION_CODE);

	primitive(S(XeTeXcountglyphs), LAST_ITEM, XETEX_COUNT_GLYPHS_CODE);
	primitive(S(XeTeXcountvariations), LAST_ITEM, XETEX_COUNT_VARIATIONS_CODE);
	primitive(S(XeTeXvariation), LAST_ITEM, XETEX_VARIATION_CODE);
	primitive(S(XeTeXfindvariationbyname), LAST_ITEM, XETEX_FIND_VARIATION_BY_NAME_CODE);
	primitive(S(XeTeXvariationmin), LAST_ITEM, XETEX_VARIATION_MIN_CODE);
	primitive(S(XeTeXvariationmax), LAST_ITEM, XETEX_VARIATION_MAX_CODE);
	primitive(S(XeTeXvariationdefault), LAST_ITEM, XETEX_VARIATION_DEFAULT_CODE);
	primitive(S(XeTeXcountfeatures), LAST_ITEM, XETEX_COUNT_FEATURES_CODE);
	primitive(S(XeTeXfeaturecode), LAST_ITEM, XETEX_FEATURE_CODE_CODE);
	primitive(S(XeTeXfindfeaturebyname), LAST_ITEM, XETEX_FIND_FEATURE_BY_NAME_CODE);
	primitive(S(XeTeXisexclusivefeature), LAST_ITEM, XETEX_IS_EXCLUSIVE_FEATURE_CODE);
	primitive(S(XeTeXcountselectors), LAST_ITEM, XETEX_COUNT_SELECTORS_CODE);
	primitive(S(XeTeXselectorcode), LAST_ITEM, XETEX_SELECTOR_CODE_CODE);
	primitive(S(XeTeXfindselectorbyname), LAST_ITEM, XETEX_FIND_SELECTOR_BY_NAME_CODE);
	primitive(S(XeTeXisdefaultselector), LAST_ITEM, XETEX_IS_DEFAULT_SELECTOR_CODE);

	primitive(S(XeTeXvariationname), CONVERT, XETEX_VARIATION_NAME_CODE);
	primitive(S(XeTeXfeaturename), CONVERT, XeTeX_feature_name);
	primitive(S(XeTeXselectorname), CONVERT, XeTeX_selector_name);

	primitive(S(XeTeXOTcountscripts), LAST_ITEM, XETEX_OT_COUNT_SCRIPTS_CODE);
	primitive(S(XeTeXOTcountlanguages), LAST_ITEM, XETEX_OT_COUNT_LANGUAGES_CODE);
	primitive(S(XeTeXOTcountfeatures), LAST_ITEM, XETEX_OT_COUNT_FEATURES_CODE);
	primitive(S(XeTeXOTscripttag), LAST_ITEM, XETEX_OT_SCRIPT_CODE);
	primitive(S(XeTeXOTlanguagetag), LAST_ITEM, XETEX_OT_LANGUAGE_CODE);
	primitive(S(XeTeXOTfeaturetag), LAST_ITEM, XETEX_OT_FEATURE_CODE);
	primitive(S(XeTeXcharglyph), LAST_ITEM, XETEX_MAP_CHAR_TO_GLYPH_CODE);
	primitive(S(XeTeXglyphindex), LAST_ITEM, XETEX_GLYPH_INDEX_CODE);
	primitive(S(XeTeXglyphbounds), LAST_ITEM, XETEX_GLYPH_BOUNDS_CODE);

	primitive(S(XeTeXglyphname), CONVERT, XETEX_GLYPH_NAME_CODE);

	primitive(S(XeTeXfonttype), LAST_ITEM, XETEX_FONT_TYPE_CODE);
	primitive(S(XeTeXfirstfontchar), LAST_ITEM, XETEX_FIRST_CHAR_CODE);
	primitive(S(XeTeXlastfontchar), LAST_ITEM, XETEX_LAST_CHAR_CODE);
	primitive(S(pdflastxpos), LAST_ITEM, PDF_LAST_X_POS_CODE);
	primitive(S(pdflastypos), LAST_ITEM, PDF_LAST_Y_POS_CODE);

	primitive(S(strcmp), CONVERT, PDF_STRCMP_CODE);
	primitive(S(mdfivesum), CONVERT, PDF_MDFIVE_SUM_CODE);

	primitive(S(shellescape), LAST_ITEM, PDF_SHELL_ESCAPE_CODE);
	primitive(S(XeTeXpdfpagecount), LAST_ITEM, XETEX_PDF_PAGE_COUNT_CODE);

	primitive(S(everyeof), ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_eof);

	primitive(S(tracingassigns), ASSIGN_INT, INT_BASE + INT_PAR__tracing_assigns);
	primitive(S(tracinggroups), ASSIGN_INT, INT_BASE + INT_PAR__tracing_groups);
	primitive(S(tracingifs), ASSIGN_INT, INT_BASE + INT_PAR__tracing_ifs);
	primitive(S(tracingscantokens), ASSIGN_INT, INT_BASE + INT_PAR__tracing_scan_tokens);
	primitive(S(tracingnesting), ASSIGN_INT, INT_BASE + INT_PAR__tracing_nesting);
	primitive(S(predisplaydirection), ASSIGN_INT, INT_BASE + INT_PAR__pre_display_correction);
	primitive(S(lastlinefit), ASSIGN_INT, INT_BASE + INT_PAR__last_line_fit);
	primitive(S(savingvdiscards), ASSIGN_INT, INT_BASE + INT_PAR__saving_vdiscards);
	primitive(S(savinghyphcodes), ASSIGN_INT, INT_BASE + INT_PAR__saving_hyphs);

	primitive(S(currentgrouplevel), LAST_ITEM, CURRENT_GROUP_LEVEL_CODE);
	primitive(S(currentgrouptype), LAST_ITEM, CURRENT_GROUP_TYPE_CODE);
	primitive(S(currentiflevel), LAST_ITEM, CURRENT_IF_LEVEL_CODE);
	primitive(S(currentiftype), LAST_ITEM, CURRENT_IF_TYPE_CODE);
	primitive(S(currentifbranch), LAST_ITEM, CURRENT_IF_BRANCH_CODE);
	primitive(S(fontcharwd), LAST_ITEM, FONT_CHAR_WD_CODE);
	primitive(S(fontcharht), LAST_ITEM, FONT_CHAR_HT_CODE);
	primitive(S(fontchardp), LAST_ITEM, FONT_CHAR_DP_CODE);
	primitive(S(fontcharic), LAST_ITEM, FONT_CHAR_IC_CODE);
	primitive(S(parshapelength), LAST_ITEM, PAR_SHAPE_LENGTH_CODE);
	primitive(S(parshapeindent), LAST_ITEM, PAR_SHAPE_INDENT_CODE);
	primitive(S(parshapedimen), LAST_ITEM, PAR_SHAPE_DIMEN_CODE);

	primitive(S(showgroups), XRAY, SHOW_GROUPS);
	primitive(S(showtokens), XRAY, SHOW_TOKENS);

	primitive(S(unexpanded), THE, 1);
	primitive(S(detokenize), THE, SHOW_TOKENS);

	primitive(S(showifs), XRAY, SHOW_IFS);

	primitive(S(interactionmode), SET_PAGE_INT, 2);

	primitive(S(middle), LEFT_RIGHT, 1);

	primitive(S(suppressfontnotfounderror), ASSIGN_INT, INT_BASE + INT_PAR__suppress_fontnotfound_error);

	primitive(S(TeXXeTstate), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__texxet);
	primitive(S(XeTeXupwardsmode), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_upwards);
	primitive(S(XeTeXuseglyphmetrics), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_use_glyph_metrics);
	primitive(S(XeTeXinterchartokenstate), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_inter_char_tokens);
	primitive(S(XeTeXdashbreakstate), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_dash_break);
	primitive(S(XeTeXinputnormalization), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_input_normalization);
	primitive(S(XeTeXtracingfonts), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_tracing_fonts);
	primitive(S(XeTeXinterwordspaceshaping), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_interword_space_shaping);
	primitive(S(XeTeXgenerateactualtext), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_generate_actual_text);
	primitive(S(XeTeXhyphenatablelength), ASSIGN_INT, ETEX_STATE_BASE + STATE_INT__xetex_hyphenatable_length);

	primitive(S(XeTeXinputencoding), EXTENSION, XETEX_INPUT_ENCODING_EXTENSION_CODE);
	primitive(S(XeTeXdefaultencoding), EXTENSION, XETEX_DEFAULT_ENCODING_EXTENSION_CODE);

	primitive(S(beginL), VALIGN, BEGIN_L_CODE);
	primitive(S(endL), VALIGN, END_L_CODE);
	primitive(S(beginR), VALIGN, BEGIN_R_CODE);
	primitive(S(endR), VALIGN, END_R_CODE);

	primitive(S(scantokens), INPUT, 2);
	primitive(S(readline), READ_TO_CS, 1);
	primitive(S(unless), EXPAND_AFTER, 1);

	primitive(S(ifdefined), IF_TEST, IF_DEF_CODE);
	primitive(S(ifcsname), IF_TEST, IF_CS_CODE);
	primitive(S(iffontchar), IF_TEST, IF_FONT_CHAR_CODE);
	primitive(S(ifincsname), IF_TEST, IF_IN_CSNAME_CODE);

	primitive(S(protected), PREFIX, 8);

	primitive(S(numexpr), LAST_ITEM, ETEX_EXPR + 0);
	primitive(S(dimexpr), LAST_ITEM, ETEX_EXPR + 1);
	primitive(S(glueexpr), LAST_ITEM, ETEX_EXPR + 2);
	primitive(S(muexpr), LAST_ITEM, ETEX_EXPR + 3);
	primitive(S(gluestretchorder), LAST_ITEM, GLUE_STRETCH_ORDER_CODE);
	primitive(S(glueshrinkorder), LAST_ITEM, GLUE_SHRINK_ORDER_CODE);
	primitive(S(gluestretch), LAST_ITEM, GLUE_STRETCH_CODE);
	primitive(S(glueshrink), LAST_ITEM, GLUE_SHRINK_CODE);
	primitive(S(mutoglue), LAST_ITEM, MU_TO_GLUE_CODE);
	primitive(S(gluetomu), LAST_ITEM, GLUE_TO_MU_CODE);

	primitive(S(marks), MARK, 5);
	primitive(S(topmarks), TOP_BOT_MARK, TOP_MARK_CODE + 5);
	primitive(S(firstmarks), TOP_BOT_MARK, FIRST_MARK_CODE + 5);
	primitive(S(botmarks), TOP_BOT_MARK, BOT_MARK_CODE + 5);
	primitive(S(splitfirstmarks), TOP_BOT_MARK, SPLIT_FIRST_MARK_CODE + 5);
	primitive(S(splitbotmarks), TOP_BOT_MARK, SPLIT_BOT_MARK_CODE + 5);

	primitive(S(pagediscards), UN_VBOX, LAST_BOX_CODE);
	primitive(S(splitdiscards), UN_VBOX, VSPLIT_CODE);

	primitive(S(interlinepenalties), SET_SHAPE, INTER_LINE_PENALTIES_LOC);
	primitive(S(clubpenalties), SET_SHAPE, CLUB_PENALTIES_LOC);
	primitive(S(widowpenalties), SET_SHAPE, WIDOW_PENALTIES_LOC);
	primitive(S(displaywidowpenalties), SET_SHAPE, DISPLAY_WIDOW_PENALTIES_LOC);

	max_reg_num = 32767;
	max_reg_help_line = S(A_register_number_must_be_be_Z1);
    }

    no_new_control_sequence = true;

    if (!in_initex_mode) {
	if (!load_fmt_file())
	    return history;
    }

    eqtb = the_eqtb;

    if (INTPAR(end_line_char) < 0 || INTPAR(end_line_char) > BIGGEST_CHAR)
	cur_input.limit--;
    else
	buffer[cur_input.limit] = INTPAR(end_line_char);

    if (in_initex_mode) {
        /* TeX initializes with the real date and time, but for format file
         * reproducibility we do this: */
        INTPAR(time) = 0;
        INTPAR(day) = 0;
        INTPAR(month) = 0;
        INTPAR(year) = 0;
    } else {
        get_date_and_time(&(INTPAR(time)),
                          &(INTPAR(day)),
                          &(INTPAR(month)),
                          &(INTPAR(year)));
    }

    if (trie_not_ready) {
	trie_trl = xmalloc_array(trie_pointer, trie_size);
	trie_tro = xmalloc_array(trie_pointer, trie_size);
	trie_trc = xmalloc_array(uint16_t, trie_size);
	trie_c = xmalloc_array(packed_UTF16_code, trie_size);
	trie_o = xmalloc_array(trie_opcode, trie_size);
	trie_l = xmalloc_array(trie_pointer, trie_size);
	trie_r = xmalloc_array(trie_pointer, trie_size);
	trie_hash = xmalloc_array(trie_pointer, trie_size);
	trie_taken = xmalloc_array(boolean, trie_size);
	trie_l[0] = 0;
	trie_c[0] = 0;
	trie_ptr = 0;
	trie_r[0] = 0;
	hyph_start = 0;
	font_mapping = xmalloc_array(void *, font_max);
	font_layout_engine = xmalloc_array(void *, font_max);
	font_flags = xmalloc_array(char, font_max);
	font_letter_space = xmalloc_array(scaled, font_max);
	font_check = xmalloc_array(four_quarters, font_max);
	font_size = xmalloc_array(scaled, font_max);
	font_dsize = xmalloc_array(scaled, font_max);
	font_params = xmalloc_array(font_index, font_max);
	font_name = xmalloc_array(str_number, font_max);
	font_area = xmalloc_array(str_number, font_max);
	font_bc = xmalloc_array(UTF16_code, font_max);
	font_ec = xmalloc_array(UTF16_code, font_max);
	font_glue = xmalloc_array(int32_t, font_max);
	hyphen_char = xmalloc_array(integer, font_max);
	skew_char = xmalloc_array(integer, font_max);
	bchar_label = xmalloc_array(font_index, font_max);
	font_bchar = xmalloc_array(nine_bits, font_max);
	font_false_bchar = xmalloc_array(nine_bits, font_max);
	char_base = xmalloc_array(integer, font_max);
	width_base = xmalloc_array(integer, font_max);
	height_base = xmalloc_array(integer, font_max);
	depth_base = xmalloc_array(integer, font_max);
	italic_base = xmalloc_array(integer, font_max);
	lig_kern_base = xmalloc_array(integer, font_max);
	kern_base = xmalloc_array(integer, font_max);
	exten_base = xmalloc_array(integer, font_max);
	param_base = xmalloc_array(integer, font_max);
	font_ptr = FONT_BASE;
	fmem_ptr = 7;
	font_name[FONT_BASE] = S(nullfont);
	font_area[FONT_BASE] = S();
	hyphen_char[FONT_BASE] = 45 /*"-" */;
	skew_char[FONT_BASE] = -1;
	bchar_label[FONT_BASE] = NON_ADDRESS;
	font_bchar[FONT_BASE] = TOO_BIG_CHAR;
	font_false_bchar[FONT_BASE] = TOO_BIG_CHAR;
	font_bc[FONT_BASE] = 1;
	font_ec[FONT_BASE] = 0;
	font_size[FONT_BASE] = 0;
	font_dsize[FONT_BASE] = 0;
	char_base[FONT_BASE] = 0;
	width_base[FONT_BASE] = 0;
	height_base[FONT_BASE] = 0;
	depth_base[FONT_BASE] = 0;
	italic_base[FONT_BASE] = 0;
	lig_kern_base[FONT_BASE] = 0;
	kern_base[FONT_BASE] = 0;
	exten_base[FONT_BASE] = 0;
	font_glue[FONT_BASE] = MIN_HALFWORD;
	font_params[FONT_BASE] = 7;
	font_mapping[FONT_BASE] = 0;
	param_base[FONT_BASE] = -1;

	for (font_k = 0; font_k <= 6; font_k++)
	    font_info[font_k].cint = 0;
    }

    font_used = xmalloc_array(boolean, font_max);
    for (font_k = 0; font_k <= font_max; font_k++)
	font_used[font_k] = false;

    /* This is only used in mlist_to_hlist() and I don't even want to know why. */
    magic_offset = str_start[MATH_SPACING - 65536L] - 9 * ORD_NOAD/*:794*/;

    if (interaction == BATCH_MODE)
	selector = SELECTOR_NO_PRINT;
    else
	selector = SELECTOR_TERM_ONLY; /*:79*/

    /* OK, we are finally ready to go! We have synthesized a "first line" in
     * cur_input that has the file name. Calling start_input() essentially
     * pretends that the user has written "\input ...". In classic TeX, this
     * codepath is only invoked if the first character is not an escape
     * character, but we don't do things that way.
     */

    start_input();
    history = HISTORY_SPOTLESS;
    synctex_init_command();
    main_control();
    final_cleanup();
    close_files_and_terminate();
    return history;
}


/* These functions don't belong here except that we want to reuse the
 * infrastructure for error handling with longjmp() and _tt_abort().
 */

int
dvipdfmx_simple_main(char *dviname, char *pdfname)
{
    extern int dvipdfmx_main(int argc, char *argv[]);

    char *argv[] = { "dvipdfmx", "-o", pdfname, dviname };

    if (setjmp (jump_buffer))
	return 99;

    return dvipdfmx_main(4, argv);
}


int
bibtex_simple_main(char *aux_file_name)
{
    extern tt_history_t bibtex_main_body(const char *aux_file_name);

    if (setjmp (jump_buffer))
        return 99;

    return bibtex_main_body(aux_file_name);
}
