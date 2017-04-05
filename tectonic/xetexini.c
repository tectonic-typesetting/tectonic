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

  /* Since `size' does not change, we can write a while loop for each
     case, and avoid testing `size' for each time.  */
  switch (size)
    {
    /* 16-byte items happen on the DEC Alpha machine when we are not
       doing sharable memory dumps.  */
    case 16:
      while (nitems--)
        {
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
      while (nitems--)
        {
          SWAP (p[0], p[7]);
          SWAP (p[1], p[6]);
          SWAP (p[2], p[5]);
          SWAP (p[3], p[4]);
          p += size;
        }
      break;

    case 4:
      while (nitems--)
        {
          SWAP (p[0], p[3]);
          SWAP (p[1], p[2]);
          p += size;
        }
      break;

    case 2:
      while (nitems--)
        {
          SWAP (p[0], p[1]);
          p += size;
        }
      break;

    case 1:
      /* Nothing to do.  */
      break;

    default:
	_tt_abort("can't swap a %d-byte item for (un)dumping", size);
  }
}
#endif /* not WORDS_BIGENDIAN */


/* Here we write NITEMS items, each item being ITEM_SIZE bytes long.
   The pointer to the stuff to write is P, and we write to the file
   OUT_FILE.  */

static void
do_dump (char *p, int item_size, int nitems, rust_output_handle_t out_file)
{
#if !defined (WORDS_BIGENDIAN)
  swap_items (p, nitems, item_size);
#endif

  if (ttstub_output_write (out_file, p, item_size * nitems) != item_size * nitems)
      _tt_abort ("could not write %d %d-byte item(s) to %s",
		 nitems, item_size, name_of_file+1);

  /* Have to restore the old contents of memory, since some of it might
     get used again.  */
#if !defined (WORDS_BIGENDIAN)
  swap_items (p, nitems, item_size);
#endif
}


/* Here is the dual of the writing routine.  */

static void
do_undump (char *p, int item_size, int nitems, rust_input_handle_t in_file)
{
  if (ttstub_input_read (in_file, p, item_size * nitems) != item_size * nitems)
      _tt_abort("could not undump %d %d-byte item(s) from %s",
		nitems, item_size, name_of_file+1);

#if !defined (WORDS_BIGENDIAN)
  swap_items (p, nitems, item_size);
#endif
}


#define	dump_things(base, len) \
  do_dump ((char *) &(base), sizeof (base), (int) (len), fmt_out)
#define	undump_things(base, len) \
  do_undump ((char *) &(base), sizeof (base), (int) (len), fmt_in)

/* Like do_undump, but check each value against LOW and HIGH.  The
   slowdown isn't significant, and this improves the chances of
   detecting incompatible format files.  In fact, Knuth himself noted
   this problem with Web2c some years ago, so it seems worth fixing.  We
   can't make this a subroutine because then we lose the type of BASE.  */
#define undump_checked_things(low, high, base, len)			\
  do {                                                                  \
    unsigned i;                                                         \
    undump_things (base, len);                                           \
    for (i = 0; i < (len); i++) {                                       \
      if ((&(base))[i] < (low) || (&(base))[i] > (high)) {              \
        _tt_abort ("item %u (=%" PRIdPTR ") of .fmt array at %" PRIxPTR \
                " <%" PRIdPTR " or >%" PRIdPTR,                         \
                i, (uintptr_t) (&(base))[i], (uintptr_t) &(base),       \
                (uintptr_t) low, (uintptr_t) high);                     \
      }                                                                 \
    }									\
  } while (0)

/* Like undump_checked_things, but only check the upper value. We use
   this when the base type is unsigned, and thus all the values will be
   greater than zero by definition.  */
#define undump_upper_check_things(high, base, len)				\
  do {                                                                  \
    unsigned i;                                                         \
    undump_things (base, len);                                           \
    for (i = 0; i < (len); i++) {                                       \
      if ((&(base))[i] > (high)) {              			\
        _tt_abort ("Item %u (=%" PRIdPTR ") of .fmt array at %" PRIxPTR \
                " >%" PRIdPTR,                                          \
                i, (uintptr_t) (&(base))[i], (uintptr_t) &(base),       \
                (uintptr_t) high);                         		\
      }                                                                 \
    }									\
  } while (0)

/* Use the above for all the other dumping and undumping.  */
#define generic_dump(x) dump_things (x, 1)
#define generic_undump(x) undump_things (x, 1)

#define dump_wd   generic_dump
#define dump_hh   generic_dump
#define dump_qqqq generic_dump
#define undump_wd   generic_undump
#define undump_hh   generic_undump
#define	undump_qqqq generic_undump

/* `dump_int' is called with constant integers, so we put them into a
   variable first.  */
#define	dump_int(x)							\
  do									\
    {									\
      integer x_val = (x);						\
      generic_dump (x_val);						\
    }									\
  while (0)

#define	undump_int generic_undump


#define hash_offset 514
#define engine_name "xetex"
#define sup_main_memory 256000000L
#define sup_max_strings 2097151L
#define sup_font_mem_size 147483647L
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

    p = get_node(1073741824L);
    p = mem[rover + 1].hh.v.RH;
    mem[rover + 1].hh.v.RH = 1073741823L;
    old_rover = rover;
    while (p != old_rover)      /*136: */
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
    p = rover;
    while (mem[p + 1].hh.v.RH != 1073741823L) {

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
    memory_word *eqtb = zeqtb;
    pool_pointer k;
    integer j;
    small_number l;
    integer prim_val;

    if (s < 256) {
        cur_val = s + 1114113L;
        prim_val = s;
    } else {
        k = str_start[(s) - 65536L];
        l = str_start[(s + 1) - 65536L] - k;
        if (first + l > buf_size + 1)
            overflow(S(buffer_size), buf_size);

	for (j = 0; j <= l - 1; j++)
	    buffer[first + j] = str_pool[k + j];

        cur_val = id_lookup(first, l);
	str_ptr--;
	pool_ptr = str_start[(str_ptr) - 65536L];
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

trie_opcode znew_trie_op(small_number d, small_number n, trie_opcode v)
{
    register trie_opcode Result;
    new_trie_op_regmem integer h;
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

trie_pointer ztrie_node(trie_pointer p)
{
    register trie_pointer Result;
    trie_node_regmem trie_pointer h;
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

trie_pointer zcompress_trie(trie_pointer p)
{
    register trie_pointer Result;
    compress_trie_regmem if (p == 0)
        Result = 0;
    else {

        trie_l[p] = compress_trie(trie_l[p]);
        trie_r[p] = compress_trie(trie_r[p]);
        Result = trie_node(p);
    }
    return Result;
}

void zfirst_fit(trie_pointer p)
{
    first_fit_regmem trie_pointer h;
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
            goto lab45;
        q = trie_r[p];
        while (q > 0) {

            if (trie_trl[h + trie_c[q]] == 0)
                goto lab45;
            q = trie_r[q];
        }
        goto lab40;
 lab45:                        /*not_found */ z = trie_trl[z];
    }
 lab40:                        /*found *//*991: */ trie_taken[h] = true;
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

void ztrie_pack(trie_pointer p)
{
    trie_pack_regmem trie_pointer q;
    do {
        q = trie_l[p];
        if ((q > 0) && (trie_hash[q] == 0)) {
            first_fit(q);
            trie_pack(q);
        }
        p = trie_r[p];
    } while (!(p == 0));
}

void ztrie_fix(trie_pointer p)
{
    trie_fix_regmem trie_pointer q;
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

void new_patterns(void)
{
    new_patterns_regmem short /*hyphenatable_length_limit 1 */ k, l;
    boolean digit_sensed;
    trie_opcode v;
    trie_pointer p, q;
    boolean first_child;
    UTF16_code c;
    if (trie_not_ready) {
        if (eqtb[(INT_BASE + 50)].cint <= 0)
            cur_lang = 0;
        else if (eqtb[(INT_BASE + 50)].cint > BIGGEST_LANG)
            cur_lang = 0;
        else
            cur_lang = eqtb[(INT_BASE + 50)].cint;
        scan_left_brace();
        k = 0;
        hyf[0] = 0;
        digit_sensed = false;
        while (true) {

            get_x_token();
            switch (cur_cmd) {
            case 11:
            case 12:
                if (digit_sensed || (cur_chr < 48 /*"0" */ ) || (cur_chr > 57 /*"9" */ )) {
                    if (cur_chr == 46 /*"." */ )
                        cur_chr = 0;
                    else {

                        cur_chr = eqtb[LC_CODE_BASE + cur_chr].hh.v.RH;
                        if (cur_chr == 0) {
                            {
                                if (interaction == ERROR_STOP_MODE) ;
                                if (file_line_error_style_p)
                                    print_file_line();
                                else
                                    print_nl(S(__/*"! "*/));
                                print(S(Nonletter));
                            }
                            {
                                help_ptr = 1;
                                help_line[0] = S(_See_Appendix_H__);
                            }
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
            case 10:
            case 2:
                {
                    if (k > 0) {        /*998: */
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
                                goto lab31;
                        }
 lab31:                        /*done1 *//*:1000 */ ;
                        q = 0;
                        hc[0] = cur_lang;
                        while (l <= k) {

                            c = hc[l];
                            l++;
                            p = trie_l[q];
                            first_child = true;
                            while ((p > 0) && (c > trie_c[p])) {

                                q = p;
                                p = trie_r[q];
                                first_child = false;
                            }
                            if ((p == 0) || (c < trie_c[p])) {  /*999: */
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
                            {
                                if (interaction == ERROR_STOP_MODE) ;
                                if (file_line_error_style_p)
                                    print_file_line();
                                else
                                    print_nl(S(__/*"! "*/));
                                print(S(Duplicate_pattern));
                            }
                            {
                                help_ptr = 1;
                                help_line[0] = S(_See_Appendix_H__);
                            }
                            error();
                        }
                        trie_o[q] = v;
                    }
                    if (cur_cmd == RIGHT_BRACE)
                        goto lab30;
                    k = 0;
                    hyf[0] = 0;
                    digit_sensed = false;
                }
                break;
            default:
                {
                    {
                        if (interaction == ERROR_STOP_MODE) ;
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(Bad_));
                    }
                    print_esc(S(patterns));
                    {
                        help_ptr = 1;
                        help_line[0] = S(_See_Appendix_H__);
                    }
                    error();
                }
                break;
            }
        }
 lab30:                        /*done *//*:996 */ ;
        if (eqtb[(INT_BASE + 66)].cint > 0) {        /*1643: */
            c = cur_lang;
            first_child = false;
            p = 0;
            do {
                q = p;
                p = trie_r[q];
            } while (!((p == 0) || (c <= trie_c[p])));
            if ((p == 0) || (c < trie_c[p])) {  /*999: */
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
            {
                register integer for_end;
                c = 0;
                for_end = 255;
                if (c <= for_end)
                    do
                        if ((eqtb[LC_CODE_BASE + c].hh.v.RH > 0) || ((c == 255) && first_child)) {
                            if (p == 0) {       /*999: */
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
                            } else
                                trie_c[p] = c;
                            trie_o[p] = eqtb[LC_CODE_BASE + c].hh.v.RH;
                            q = p;
                            p = trie_r[q];
                            first_child = false;
                        }
                    while (c++ < for_end) ;
            }
            if (first_child)
                trie_l[q] = 0;
            else
                trie_r[q] = 0 /*:1644 */ ;
        }
    } else {

        {
            if (interaction == ERROR_STOP_MODE) ;
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Too_late_for_));
        }
        print_esc(S(patterns));
        {
            help_ptr = 1;
            help_line[0] = S(All_patterns_must_be_given_b/*efore typesetting begins.*/);
        }
        error();
        mem[mem_top - 12].hh.v.RH = scan_toks(false, false);
        flush_list(def_ref);
    }
}

void init_trie(void)
{
    init_trie_regmem trie_pointer p;
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

void new_hyph_exceptions(void)
{
    new_hyph_exceptions_regmem short /*hyphenatable_length_limit 1 */ n;
    short /*hyphenatable_length_limit 1 */ j;
    hyph_pointer h;
    str_number k;
    int32_t p;
    int32_t q;
    str_number s;
    pool_pointer u, v;
    scan_left_brace();
    if (eqtb[(INT_BASE + 50)].cint <= 0)
        cur_lang = 0;
    else if (eqtb[(INT_BASE + 50)].cint > BIGGEST_LANG)
        cur_lang = 0;
    else
        cur_lang = eqtb[(INT_BASE + 50)].cint;

    if (trie_not_ready) {
        hyph_index = 0;
        goto lab46;
    }

    if (trie_trc[hyph_start + cur_lang] != cur_lang)
        hyph_index = 0;
    else
        hyph_index = trie_trl[hyph_start + cur_lang];
 lab46:                        /*not_found1 *//*970: */ n = 0;
    p = -268435455L;
    while (true) {

        get_x_token();
 lab21:                        /*reswitch */ switch (cur_cmd) {
        case 11:
        case 12:
        case 68:
            if (cur_chr == 45 /*"-" */ ) {      /*973: */
                if (n < max_hyphenatable_length()) {
                    q = get_avail();
                    mem[q].hh.v.RH = p;
                    mem[q].hh.v.LH = n;
                    p = q;
                }
            } else {

                if ((hyph_index == 0) || ((cur_chr) > 255))
                    hc[0] = eqtb[LC_CODE_BASE + cur_chr].hh.v.RH;
                else if (trie_trc[hyph_index + cur_chr] != cur_chr)
                    hc[0] = 0;
                else
                    hc[0] = trie_tro[hyph_index + cur_chr];
                if (hc[0] == 0) {
                    {
                        if (interaction == ERROR_STOP_MODE) ;
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(Not_a_letter));
                    }
                    {
                        help_ptr = 2;
                        help_line[1] = S(Letters_in__hyphenation_word/*s must have \lccode>0.*/);
                        help_line[0] = S(Proceed__I_ll_ignore_the_cha/*racter I just read.*/);
                    }
                    error();
                } else if (n < max_hyphenatable_length()) {
                    n++;
                    if (hc[0] < 65536L)
                        hc[n] = hc[0];
                    else {

                        hc[n] = (hc[0] - 65536L) / 1024 + 55296L;
                        n++;
                        hc[n] = hc[0] % 1024 + 56320L;
                    }
                }
            }
            break;
        case 16:
            {
                scan_char_num();
                cur_chr = cur_val;
                cur_cmd = CHAR_GIVEN;
                goto lab21;
            }
            break;
        case 10:
        case 2:
            {
                if (n > 1) {    /*974: */
                    n++;
                    hc[n] = cur_lang;
                    {
                        if (pool_ptr + n > pool_size)
                            overflow(S(pool_size), pool_size - init_pool_ptr);
                    }
                    h = 0;
                    {
                        register integer for_end;
                        j = 1;
                        for_end = n;
                        if (j <= for_end)
                            do {
                                h = (h + h + hc[j]) % HYPH_PRIME;
                                {
                                    str_pool[pool_ptr] = hc[j];
                                    pool_ptr++;
                                }
                            }
                            while (j++ < for_end);
                    }
                    s = make_string();
                    if (hyph_next <= HYPH_PRIME)
                        while ((hyph_next > 0) && (hyph_word[hyph_next - 1] > 0))
                            hyph_next--;
                    if ((hyph_count == hyph_size) || (hyph_next == 0))
                        overflow(S(exception_dictionary), hyph_size);
                    hyph_count++;
                    while (hyph_word[h] != 0) {

                        k = hyph_word[h];
                        if (length(k) != length(s))
                            goto lab45;
                        u = str_start[(k) - 65536L];
                        v = str_start[(s) - 65536L];
                        do {
                            if (str_pool[u] != str_pool[v])
                                goto lab45;
                            u++;
                            v++;
                        } while (!(u == str_start[(k + 1) - 65536L]));
                        {
                            str_ptr--;
                            pool_ptr = str_start[(str_ptr) - 65536L];
                        }
                        s = hyph_word[h];
                        hyph_count--;
                        goto lab40;
 lab45:                        /*not_found *//*:976 */ ;
                        if (hyph_link[h] == 0) {
                            hyph_link[h] = hyph_next;
                            if (hyph_next >= hyph_size)
                                hyph_next = HYPH_PRIME;
                            if (hyph_next > HYPH_PRIME)
                                hyph_next++;
                        }
                        h = hyph_link[h] - 1;
                    }
 lab40:                        /*found */ hyph_word[h] = s;
                    hyph_list[h] = /*:975 */ p;
                }
                if (cur_cmd == RIGHT_BRACE)
                    return;
                n = 0;
                p = -268435455L;
            }
            break;
        default:
            {
                {
                    if (interaction == ERROR_STOP_MODE) ;
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Improper_));
                }
                print_esc(S(hyphenation));
                print(S(_will_be_flushed));
                {
                    help_ptr = 2;
                    help_line[1] = S(Hyphenation_exceptions_must_/*contain only letters*/);
                    help_line[0] = S(and_hyphens__But_continue__I/*'ll forgive and forget.*/);
                }
                error();
            }
            break;
        }
    }
}

void prefixed_command(void)
{
    prefixed_command_regmem small_number a;
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
        } while (!((cur_cmd != SPACER) && (cur_cmd != RELAX) /*:422 */ ));
        if (cur_cmd <= MAX_NON_PREFIXED_COMMAND) {     /*1247: */
            {
                if (interaction == ERROR_STOP_MODE) ;
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(You_can_t_use_a_prefix_with_/*`*/));
            }
            print_cmd_chr(cur_cmd, cur_chr);
            print_char(39 /*"'" */ );
            {
                help_ptr = 1;
                help_line[0] = S(I_ll_pretend_you_didn_t_say_/*\long or \outer or \global.*/);
            }
            if ((eTeX_mode == 1))
                help_line[0] = S(I_ll_pretend_you_didn_t_say__Z1/*"I'll pretend you didn't say \long or \outer or \global or \protected."*/);
            back_error();
            return;
        }
        if (eqtb[(INT_BASE + 36)].cint > 2) {

            if ((eTeX_mode == 1))
                show_cur_cmd_chr();
        }
    }
    if (a >= 8) {
        j = PROTECTED_TOKEN;
        a = a - 8;
    } else
        j = 0;
    if ((cur_cmd != DEF) && ((a % 4 != 0) || (j != 0))) {
        {
            if (interaction == ERROR_STOP_MODE) ;
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(You_can_t_use__));
        }
        print_esc(S(long));
        print(S(__or__/*"' or `"*/));
        print_esc(S(outer));
        {
            help_ptr = 1;
            help_line[0] = S(I_ll_pretend_you_didn_t_say__Z2/*"I'll pretend you didn't say \long or \outer here."*/);
        }
        if ((eTeX_mode == 1)) {
            help_line[0] = S(I_ll_pretend_you_didn_t_say__Z3/*"I'll pretend you didn't say \long or \outer or \protected here."*/);
            print(S(__or__/*"' or `"*/));
            print_esc(S(protected));
        }
        print(S(__with__));
        print_cmd_chr(cur_cmd, cur_chr);
        print_char(39 /*"'" */ );
        error();
    }
    if (eqtb[(INT_BASE + 43)].cint != 0) {

        if (eqtb[(INT_BASE + 43)].cint < 0) {
            if ((a >= 4))
                a = a - 4;
        } else {

            if (!(a >= 4))
                a = a + 4;
        }
    }
    switch (cur_cmd) {          /*1252: */
    case 89:
        if ((a >= 4))
            geq_define(CUR_FONT_LOC, DATA, cur_chr);
        else
            eq_define(CUR_FONT_LOC, DATA, cur_chr);
        break;
    case 99:
        {
            if (odd(cur_chr) && !(a >= 4) && (eqtb[(INT_BASE + 43)].cint >= 0))
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
            if ((a >= 4))
                geq_define(p, CALL + (a % 4), def_ref);
            else
                eq_define(p, CALL + (a % 4), def_ref);
        }
        break;
    case 96:
        {
            n = cur_chr;
            get_r_token();
            p = cur_cs;
            if (n == NORMAL) {
                do {
                    get_token();
                } while (!(cur_cmd != SPACER));
                if (cur_tok == (OTHER_TOKEN + 61)) {
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
            if (cur_cmd >= CALL)
                mem[cur_chr].hh.v.LH++;
            else if ((cur_cmd == REGISTER) || (cur_cmd == TOKS_REGISTER)) {

                if ((cur_chr < mem_bot) || (cur_chr > mem_bot + 19))
                    mem[cur_chr + 1].hh.v.LH++;
            }
            if ((a >= 4))
                geq_define(p, cur_cmd, cur_chr);
            else
                eq_define(p, cur_cmd, cur_chr);
        }
        break;
    case 97:
        if (cur_chr == CHAR_SUB_DEF_CODE) {
            scan_char_num();
            p = CHAR_SUB_CODE_BASE + cur_val;
            scan_optional_equals();
            scan_char_num();
            n = cur_val;
            scan_char_num();
            if ((eqtb[(INT_BASE + 57)].cint > 0)) {
                begin_diagnostic();
                print_nl(S(New_character_substitution__/**/));
                print(p - 7824628L);
                print(S(____Z6/*" = "*/));
                print(n);
                print_char(32 /*" " */ );
                print(cur_val);
                end_diagnostic(false);
            }
            n = n * 256 + cur_val;
            if ((a >= 4))
                geq_define(p, DATA, n);
            else
                eq_define(p, DATA, n);
            if ((p - 7824628L) < eqtb[(INT_BASE + 55)].cint) {

                if ((a >= 4))
                    geq_word_define((INT_BASE + 55), p - 7824628L);
                else
                    eq_word_define((INT_BASE + 55), p - 7824628L);
            }
            if ((p - 7824628L) > eqtb[(INT_BASE + 56)].cint) {

                if ((a >= 4))
                    geq_word_define((INT_BASE + 56), p - 7824628L);
                else
                    eq_word_define((INT_BASE + 56), p - 7824628L);
            }
        } else {

            n = cur_chr;
            get_r_token();
            p = cur_cs;
            if ((a >= 4))
                geq_define(p, RELAX, TOO_BIG_USV);
            else
                eq_define(p, RELAX, TOO_BIG_USV);
            scan_optional_equals();
            switch (n) {
            case 0:
                {
                    scan_usv_num();
                    if ((a >= 4))
                        geq_define(p, CHAR_GIVEN, cur_val);
                    else
                        eq_define(p, CHAR_GIVEN, cur_val);
                }
                break;
            case 1:
                {
                    scan_fifteen_bit_int();
                    if ((a >= 4))
                        geq_define(p, MATH_GIVEN, cur_val);
                    else
                        eq_define(p, MATH_GIVEN, cur_val);
                }
                break;
            case 8:
                {
                    scan_xetex_math_char_int();
                    if ((a >= 4))
                        geq_define(p, XETEX_MATH_GIVEN, cur_val);
                    else
                        eq_define(p, XETEX_MATH_GIVEN, cur_val);
                }
                break;
            case 9:
                {
                    scan_math_class_int();
                    n = set_class(cur_val);
                    scan_math_fam_int();
                    n = n + set_family(cur_val);
                    scan_usv_num();
                    n = n + cur_val;
                    if ((a >= 4))
                        geq_define(p, XETEX_MATH_GIVEN, n);
                    else
                        eq_define(p, XETEX_MATH_GIVEN, n);
                }
                break;
            default:
                {
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
                        if ((a >= 4))
                            geq_define(p, j, cur_ptr);
                        else
                            eq_define(p, j, cur_ptr);
                    } else
                        switch (n) {
                        case 2:
                            if ((a >= 4))
                                geq_define(p, ASSIGN_INT, COUNT_BASE + cur_val);
                            else
                                eq_define(p, ASSIGN_INT, COUNT_BASE + cur_val);
                            break;
                        case 3:
                            if ((a >= 4))
                                geq_define(p, ASSIGN_DIMEN, SCALED_BASE + cur_val);
                            else
                                eq_define(p, ASSIGN_DIMEN, SCALED_BASE + cur_val);
                            break;
                        case 4:
                            if ((a >= 4))
                                geq_define(p, ASSIGN_GLUE, SKIP_BASE + cur_val);
                            else
                                eq_define(p, ASSIGN_GLUE, SKIP_BASE + cur_val);
                            break;
                        case 5:
                            if ((a >= 4))
                                geq_define(p, ASSIGN_MU_GLUE, MU_SKIP_BASE + cur_val);
                            else
                                eq_define(p, ASSIGN_MU_GLUE, MU_SKIP_BASE + cur_val);
                            break;
                        case 6:
                            if ((a >= 4))
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
    case 98:
        {
            j = cur_chr;
            scan_int();
            n = cur_val;
            if (!scan_keyword(S(to))) {
                {
                    if (interaction == ERROR_STOP_MODE) ;
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Missing__to__inserted));
                }
                {
                    help_ptr = 2;
                    help_line[1] = S(You_should_have_said___read_/*number> to \cs'.*/);
                    help_line[0] = S(I_m_going_to_look_for_the__c/*s now.*/);
                }
                error();
            }
            get_r_token();
            p = cur_cs;
            read_toks(n, p, j);
            if ((a >= 4))
                geq_define(p, CALL, cur_val);
            else
                eq_define(p, CALL, cur_val);
        }
        break;
    case 72:
    case 73:
        {
            q = cur_cs;
            e = false;
            if (cur_cmd == TOKS_REGISTER) {

                if (cur_chr == mem_bot) {
                    scan_register_num();
                    if (cur_val > 255) {
                        find_sa_element(TOK_VAL, cur_val, true);
                        cur_chr = cur_ptr;
                        e = true;
                    } else
                        cur_chr = TOKS_BASE + cur_val;
                } else
                    e = true;
            } else if (cur_chr == XETEX_INTER_CHAR_LOC) {
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
            } while (!((cur_cmd != SPACER) && (cur_cmd != RELAX) /*:422 */ ));
            if (cur_cmd != LEFT_BRACE) {        /*1262: */

                if ((cur_cmd == TOKS_REGISTER) || (cur_cmd == ASSIGN_TOKS)) {
                    if (cur_cmd == TOKS_REGISTER) {

                        if (cur_chr == mem_bot) {
                            scan_register_num();
                            if (cur_val < 256)
                                q = eqtb[TOKS_BASE + cur_val].hh.v.RH;
                            else {

                                find_sa_element(TOK_VAL, cur_val, false);
                                if (cur_ptr == -268435455L)
                                    q = -268435455L;
                                else
                                    q = mem[cur_ptr + 1].hh.v.RH;
                            }
                        } else
                            q = mem[cur_chr + 1].hh.v.RH;
                    } else if (cur_chr == XETEX_INTER_CHAR_LOC) {
                        scan_char_class_not_ignored();
                        cur_ptr = cur_val;
                        scan_char_class_not_ignored();
                        find_sa_element(INTER_CHAR_VAL, cur_ptr * CHAR_CLASS_LIMIT + cur_val, false);
                        if (cur_ptr == -268435455L)
                            q = -268435455L;
                        else
                            q = mem[cur_ptr + 1].hh.v.RH;
                    } else
                        q = eqtb[cur_chr].hh.v.RH;
                    if (q == -268435455L) {

                        if (e) {

                            if ((a >= 4))
                                gsa_def(p, -268435455L);
                            else
                                sa_def(p, -268435455L);
                        } else if ((a >= 4))
                            geq_define(p, UNDEFINED_CS, -268435455L);
                        else
                            eq_define(p, UNDEFINED_CS, -268435455L);
                    } else {

                        mem[q].hh.v.LH++;
                        if (e) {

                            if ((a >= 4))
                                gsa_def(p, q);
                            else
                                sa_def(p, q);
                        } else if ((a >= 4))
                            geq_define(p, CALL, q);
                        else
                            eq_define(p, CALL, q);
                    }
                    goto lab30;
                }
            }
            back_input();
            cur_cs = q;
            q = scan_toks(false, false);
            if (mem[def_ref].hh.v.RH == -268435455L) {
                if (e) {

                    if ((a >= 4))
                        gsa_def(p, -268435455L);
                    else
                        sa_def(p, -268435455L);
                } else if ((a >= 4))
                    geq_define(p, UNDEFINED_CS, -268435455L);
                else
                    eq_define(p, UNDEFINED_CS, -268435455L);
                {
                    mem[def_ref].hh.v.RH = avail;
                    avail = def_ref;
                }
            } else {

                if ((p == OUTPUT_ROUTINE_LOC) && !e) {
                    mem[q].hh.v.RH = get_avail();
                    q = mem[q].hh.v.RH;
                    mem[q].hh.v.LH = (RIGHT_BRACE_TOKEN + 125);
                    q = get_avail();
                    mem[q].hh.v.LH = (LEFT_BRACE_TOKEN + 123);
                    mem[q].hh.v.RH = mem[def_ref].hh.v.RH;
                    mem[def_ref].hh.v.RH = q;
                }
                if (e) {

                    if ((a >= 4))
                        gsa_def(p, def_ref);
                    else
                        sa_def(p, def_ref);
                } else if ((a >= 4))
                    geq_define(p, CALL, def_ref);
                else
                    eq_define(p, CALL, def_ref);
            }
        }
        break;
    case 74:
        {
            p = cur_chr;
            scan_optional_equals();
            scan_int();
            if ((a >= 4))
                geq_word_define(p, cur_val);
            else
                eq_word_define(p, cur_val);
        }
        break;
    case 75:
        {
            p = cur_chr;
            scan_optional_equals();
            scan_dimen(false, false, false);
            if ((a >= 4))
                geq_word_define(p, cur_val);
            else
                eq_word_define(p, cur_val);
        }
        break;
    case 76:
    case 77:
        {
            p = cur_chr;
            n = cur_cmd;
            scan_optional_equals();
            if (n == ASSIGN_MU_GLUE)
                scan_glue(MU_VAL);
            else
                scan_glue(GLUE_VAL);
            trap_zero_glue();
            if ((a >= 4))
                geq_define(p, GLUE_REF, cur_val);
            else
                eq_define(p, GLUE_REF, cur_val);
        }
        break;
    case 87:
        {
            if (cur_chr == SF_CODE_BASE) {
                p = cur_chr;
                scan_usv_num();
                p = p + cur_val;
                n = eqtb[SF_CODE_BASE + cur_val].hh.v.RH % 65536L;
                scan_optional_equals();
                scan_char_class();
                if ((a >= 4))
                    geq_define(p, DATA, cur_val * 65536L + n);
                else
                    eq_define(p, DATA, cur_val * 65536L + n);
            } else if (cur_chr == MATH_CODE_BASE) {
                p = cur_chr;
                scan_usv_num();
                p = p + cur_val;
                scan_optional_equals();
                scan_xetex_math_char_int();
                if ((a >= 4))
                    geq_define(p, DATA, cur_val);
                else
                    eq_define(p, DATA, cur_val);
            } else if (cur_chr == (MATH_CODE_BASE + 1)) {
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
                if ((a >= 4))
                    geq_define(p, DATA, n);
                else
                    eq_define(p, DATA, n);
            } else if (cur_chr == DEL_CODE_BASE) {
                p = cur_chr;
                scan_usv_num();
                p = p + cur_val;
                scan_optional_equals();
                scan_int();
                if ((a >= 4))
                    geq_word_define(p, cur_val);
                else
                    eq_word_define(p, cur_val);
            } else {

                p = cur_chr - 1;
                scan_usv_num();
                p = p + cur_val;
                scan_optional_equals();
                n = 1073741824L;
                scan_math_fam_int();
                n = n + cur_val * 2097152L;
                scan_usv_num();
                n = n + cur_val;
                if ((a >= 4))
                    geq_word_define(p, n);
                else
                    eq_word_define(p, n);
            }
        }
        break;
    case 86:
        {
            if (cur_chr == CAT_CODE_BASE)
                n = MAX_CHAR_CODE;
            else if (cur_chr == MATH_CODE_BASE)
                n = 32768L;
            else if (cur_chr == SF_CODE_BASE)
                n = 32767;
            else if (cur_chr == DEL_CODE_BASE)
                n = 16777215L;
            else
                n = BIGGEST_USV; /*:1268 */
            p = cur_chr;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            scan_int();
            if (((cur_val < 0) && (p < DEL_CODE_BASE)) || (cur_val > n)) {
                {
                    if (interaction == ERROR_STOP_MODE) ;
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Invalid_code__));
                }
                print_int(cur_val);
                if (p < DEL_CODE_BASE)
                    print(S(___should_be_in_the_range_0_/*.*/));
                else
                    print(S(___should_be_at_most_));
                print_int(n);
                {
                    help_ptr = 1;
                    help_line[0] = S(I_m_going_to_use_0_instead_o/*f that illegal code value.*/);
                }
                error();
                cur_val = 0;
            }
            if (p < MATH_CODE_BASE) {
                if (p >= SF_CODE_BASE) {
                    n = eqtb[p].hh.v.RH / 65536L;
                    if ((a >= 4))
                        geq_define(p, DATA, n * 65536L + cur_val);
                    else
                        eq_define(p, DATA, n * 65536L + cur_val);
                } else if ((a >= 4))
                    geq_define(p, DATA, cur_val);
                else
                    eq_define(p, DATA, cur_val);
            } else if (p < DEL_CODE_BASE) {
                if (cur_val == 32768L)
                    cur_val = ACTIVE_MATH_CHAR;
                else
                    cur_val =
                        set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) + (cur_val % 256);
                if ((a >= 4))
                    geq_define(p, DATA, cur_val);
                else
                    eq_define(p, DATA, cur_val);
            } else if ((a >= 4))
                geq_word_define(p, cur_val);
            else
                eq_word_define(p, cur_val);
        }
        break;
    case 88:
        {
            p = cur_chr;
            scan_math_fam_int();
            p = p + cur_val;
            scan_optional_equals();
            scan_font_ident();
            if ((a >= 4))
                geq_define(p, DATA, cur_val);
            else
                eq_define(p, DATA, cur_val);
        }
        break;
    case 91:
    case 92:
    case 93:
    case 94:
        do_register_command(a);
        break;
    case 100:
        {
            scan_register_num();
            if ((a >= 4))
                n = 1073774592L + cur_val;
            else
                n = 1073741824L + cur_val;
            scan_optional_equals();
            if (set_box_allowed)
                scan_box(n);
            else {

                {
                    if (interaction == ERROR_STOP_MODE) ;
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Improper_));
                }
                print_esc(S(setbox));
                {
                    help_ptr = 2;
                    help_line[1] = S(Sorry___setbox_is_not_allowe/*d after \halign in a display,*/);
                    help_line[0] = S(or_between__accent_and_an_ac/*cented character.*/);
                }
                error();
            }
        }
        break;
    case 80:
        alter_aux();
        break;
    case 81:
        alter_prev_graf();
        break;
    case 82:
        alter_page_so_far();
        break;
    case 83:
        alter_integer();
        break;
    case 84:
        alter_box_dimen();
        break;
    case 85:
        {
            q = cur_chr;
            scan_optional_equals();
            scan_int();
            n = cur_val;
            if (n <= 0)
                p = -268435455L;
            else if (q > PAR_SHAPE_LOC) {
                n = (cur_val / 2) + 1;
                p = get_node(2 * n + 1);
                mem[p].hh.v.LH = n;
                n = cur_val;
                mem[p + 1].cint = n;
                {
                    register integer for_end;
                    j = p + 2;
                    for_end = p + n + 1;
                    if (j <= for_end)
                        do {
                            scan_int();
                            mem[j].cint = cur_val;
                        }
                        while (j++ < for_end);
                }
                if (!odd(n))
                    mem[p + n + 2].cint = 0;
            } else {

                p = get_node(2 * n + 1);
                mem[p].hh.v.LH = n;
                {
                    register integer for_end;
                    j = 1;
                    for_end = n;
                    if (j <= for_end)
                        do {
                            scan_dimen(false, false, false);
                            mem[p + 2 * j - 1].cint = cur_val;
                            scan_dimen(false, false, false);
                            mem[p + 2 * j].cint = cur_val;
                        }
                        while (j++ < for_end);
                }
            }
            if ((a >= 4))
                geq_define(q, SHAPE_REF, p);
            else
                eq_define(q, SHAPE_REF, p);
        }
        break;
    case 101:
        if (cur_chr == 1) {
            if (in_initex_mode) {
                new_patterns();
                goto lab30;
            }
            {
                if (interaction == ERROR_STOP_MODE) ;
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Patterns_can_be_loaded_only_/*by INITEX*/));
            }
            help_ptr = 0;
            error();
            do {
                get_token();
            } while (!(cur_cmd == RIGHT_BRACE));
            return;
        } else {

            new_hyph_exceptions();
            goto lab30;
        }
        break;
    case 78:
        {
            find_font_dimen(true);
            k = cur_val;
            scan_optional_equals();
            scan_dimen(false, false, false);
            font_info[k].cint = cur_val;
        }
        break;
    case 79:
        {
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

                if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG)))
                    scan_glyph_number(f);
                else
                    scan_char_num();
                p = cur_val;
                scan_optional_equals();
                scan_int();
                switch (n) {
                case 2:
                    set_cp_code(f, p, 0, cur_val);
                    break;
                case 3:
                    set_cp_code(f, p, 1, cur_val);
                    break;
                }
            }
        }
        break;
    case 90:
        new_font(a);
        break;
    case 102:
        new_interaction();
        break;
    default:
        confusion(S(prefix));
        break;
    }
 lab30:                        /*done *//*1304: */ if (after_token != 0) {
        cur_tok = after_token;
        back_input();
        after_token = 0;
    }
}

/*:1328*//*1337: */

static void
store_fmt_file(void)
{
    memory_word *mem = zmem, *eqtb = zeqtb;
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
    print_int(eqtb[(INT_BASE + 23)].cint);
    print_char(46 /*"." */ );
    print_int(eqtb[(INT_BASE + 22)].cint);
    print_char(46 /*"." */ );
    print_int(eqtb[(INT_BASE + 21)].cint);
    print_char(41 /*")" */ );
    if (interaction == BATCH_MODE)
        selector = SELECTOR_LOG_ONLY;
    else
        selector = SELECTOR_TERM_AND_LOG;
    {
        if (pool_ptr + 1 > pool_size)
            overflow(S(pool_size), pool_size - init_pool_ptr);
    }
    format_ident = make_string();
    pack_job_name(FORMAT_EXTENSION);

    fmt_out = ttstub_output_open (name_of_file + 1, 1);
    if (fmt_out == NULL)
	_tt_abort ("cannot open format output file \"%s\"", name_of_file + 1);

    print_nl(S(Beginning_to_dump_on_file_));
    print(make_name_string());
    {
        str_ptr--;
        pool_ptr = str_start[(str_ptr) - 65536L];
    }
    print_nl(S());
    print(format_ident);
    dump_int(1462916184L);
    x = strlen(engine_name);
    format_engine = xmalloc_array(char, x + 4);
    strcpy((string) (format_engine), engine_name);
    {
        register integer for_end;
        k = x;
        for_end = x + 3;
        if (k <= for_end)
            do
                format_engine[k] = 0;
            while (k++ < for_end);
    }
    x = x + 4 - (x % 4);
    dump_int(x);
    dump_things(format_engine[0], x);
    free(format_engine);
    dump_int(457477274L);
    dump_int(1073741823L);
    dump_int(hash_high);
    dump_int(eTeX_mode);
    while (pseudo_files != -268435455L)
        pseudo_close();
    dump_int(mem_bot);
    dump_int(mem_top);
    dump_int(EQTB_SIZE);
    dump_int(HASH_PRIME);
    dump_int(HYPH_PRIME);
    dump_int(1296847960L);
    if (mltex_p)
        dump_int(1);
    else
        dump_int(0);
    dump_int(pool_ptr);
    dump_int(str_ptr);
    dump_things(str_start[(TOO_BIG_CHAR) - 65536L], str_ptr - 65535L);
    dump_things(str_pool[0], pool_ptr);
    print_ln();
    print_int(str_ptr);
    print(S(_strings_of_total_length_));
    print_int(pool_ptr);
    sort_avail();
    var_used = 0;
    dump_int(lo_mem_max);
    dump_int(rover);
    if ((eTeX_mode == 1)) {
        register integer for_end;
        k = INT_VAL;
        for_end = INTER_CHAR_VAL;
        if (k <= for_end)
            do
                dump_int(sa_root[k]);
            while (k++ < for_end);
    }
    p = mem_bot;
    q = rover;
    x = 0;
    do {
        dump_things(mem[p], q + 2 - p);
        x = x + q + 2 - p;
        var_used = var_used + q - p;
        p = q + mem[q].hh.v.LH;
        q = mem[q + 1].hh.v.RH;
    } while (!(q == rover));
    var_used = var_used + lo_mem_max - p;
    dyn_used = mem_end + 1 - hi_mem_min;
    dump_things(mem[p], lo_mem_max + 1 - p);
    x = x + lo_mem_max + 1 - p;
    dump_int(hi_mem_min);
    dump_int(avail);
    dump_things(mem[hi_mem_min], mem_end + 1 - hi_mem_min);
    x = x + mem_end + 1 - hi_mem_min;
    p = avail;
    while (p != -268435455L) {

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
    k = ACTIVE_BASE;
    do {
        j = k;
        while (j < (INT_BASE - 1)) {

            if ((eqtb[j].hh.v.RH == eqtb[j + 1].hh.v.RH) && (eqtb[j].hh.u.B0 == eqtb[j + 1].hh.u.B0)
                && (eqtb[j].hh.u.B1 == eqtb[j + 1].hh.u.B1))
                goto lab41;
            j++;
        }
        l = INT_BASE;
        goto lab31;
 lab41:                        /*found1 */ j++;
        l = j;
        while (j < (INT_BASE - 1)) {

            if ((eqtb[j].hh.v.RH != eqtb[j + 1].hh.v.RH) || (eqtb[j].hh.u.B0 != eqtb[j + 1].hh.u.B0)
                || (eqtb[j].hh.u.B1 != eqtb[j + 1].hh.u.B1))
                goto lab31;
            j++;
        }
 lab31:                        /*done1 */ dump_int(l - k);
        dump_things(eqtb[k], l - k);
        k = j + 1;
        dump_int(k - l);
    } while (!(k == INT_BASE )); /*:1350*/
    do {
        j = k;
        while (j < EQTB_SIZE) {

            if (eqtb[j].cint == eqtb[j + 1].cint)
                goto lab42;
            j++;
        }
        l = (EQTB_SIZE + 1);
        goto lab32;
 lab42:                        /*found2 */ j++;
        l = j;
        while (j < EQTB_SIZE) {

            if (eqtb[j].cint != eqtb[j + 1].cint)
                goto lab32;
            j++;
        }
 lab32:                        /*done2 */ dump_int(l - k);
        dump_things(eqtb[k], l - k);
        k = j + 1;
        dump_int(k - l);
    } while (!(k > EQTB_SIZE));
    if (hash_high > 0)
        dump_things(eqtb[(EQTB_SIZE + 1)], hash_high);
    dump_int(par_loc);
    dump_int(write_loc);
    {
        register integer for_end;
        p = 0;
        for_end = PRIM_SIZE;
        if (p <= for_end)
            do
                dump_hh(prim[p]);
            while (p++ < for_end);
    }
    {
        register integer for_end;
        p = 0;
        for_end = PRIM_SIZE;
        if (p <= for_end)
            do
                dump_wd(prim_eqtb[p]);
            while (p++ < for_end);
    }
    dump_int(hash_used);
    cs_count = (FROZEN_CONTROL_SEQUENCE - 1) - hash_used + hash_high;
    {
        register integer for_end;
        p = HASH_BASE;
        for_end = hash_used;
        if (p <= for_end)
            do
                if (hash[p].v.RH != 0) {
                    dump_int(p);
                    dump_hh(hash[p]);
                    cs_count++;
                }
            while (p++ < for_end) ;
    }
    dump_things(hash[hash_used + 1], (UNDEFINED_CONTROL_SEQUENCE - 1) - hash_used);
    if (hash_high > 0)
        dump_things(hash[(EQTB_SIZE + 1)], hash_high);
    dump_int(cs_count);
    print_ln();
    print_int(cs_count);
    print(S(_multiletter_control_sequenc/*es*/));
    dump_int(fmem_ptr);
    dump_things(font_info[0], fmem_ptr);
    dump_int(font_ptr);
    {
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
        {
            register integer for_end;
            k = FONT_BASE;
            for_end = font_ptr;
            if (k <= for_end)
                do {
                    print_nl(S(_font));
                    print_esc(hash[FONT_ID_BASE + k].v.RH);
                    print_char(61 /*"=" */ );
                    if (((font_area[k] == AAT_FONT_FLAG) || (font_area[k] == OTGR_FONT_FLAG))
                        || (font_mapping[k] != 0)) {
                        print_file_name(font_name[k], S(), S());
                        {
                            if (interaction == ERROR_STOP_MODE) ;
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Can_t__dump_a_format_with_na/*tive fonts or font-mappings*/));
                        }
                        {
                            help_ptr = 3;
                            help_line[2] = S(You_really__really_don_t_wan/*t to do this.*/);
                            help_line[1] = S(It_won_t_work__and_only_conf/*uses me.*/);
                            help_line[0] = S(_Load_them_at_runtime__not_a/*s part of the format file.)*/);
                        }
                        error();
                    } else
                        print_file_name(font_name[k], font_area[k], S());
                    if (font_size[k] != font_dsize[k]) {
                        print(S(_at_));
                        print_scaled(font_size[k]);
                        print(S(pt));
                    }
                }
                while (k++ < for_end);
        }
    }
    print_ln();
    print_int(fmem_ptr - 7);
    print(S(_words_of_font_info_for_));
    print_int(font_ptr - 0);
    if (font_ptr != (FONT_BASE + 1))
        print(S(_preloaded_fonts));
    else
        print(S(_preloaded_font));
    dump_int(hyph_count);
    if (hyph_next <= HYPH_PRIME)
        hyph_next = hyph_size;
    dump_int(hyph_next);
    {
        register integer for_end;
        k = 0;
        for_end = hyph_size;
        if (k <= for_end)
            do
                if (hyph_word[k] != 0) {
                    dump_int(k + 65536L * hyph_link[k]);
                    dump_int(hyph_word[k]);
                    dump_int(hyph_list[k]);
                }
            while (k++ < for_end) ;
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
    {
        register integer for_end;
        k = BIGGEST_LANG;
        for_end = 0;
        if (k >= for_end)
            do
                if (trie_used[k] > 0) {
                    print_nl(S(___Z12/*"  "*/));
                    print_int(trie_used[k]);
                    print(S(_for_language_));
                    print_int(k);
                    dump_int(k);
                    dump_int(trie_used[k]);
                }
            while (k-- > for_end) ;
    }
    dump_int(interaction);
    dump_int(format_ident);
    dump_int(69069L);
    eqtb[(INT_BASE + 31)].cint = 0 /*:1361 */ ;
    ttstub_output_close (fmt_out);
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
    memory_word *mem = zmem, *eqtb = zeqtb;
    integer j, k;
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

lab40: /* found */
    cur_input.loc = j;

    if (in_initex_mode) {
        free(font_info);
        free(str_pool);
        free(str_start);
        free(yhash);
        free(zeqtb);
        free(yzmem);
    }

    undump_int(x);
    if (x != 1462916184L)
        goto bad_fmt;
    undump_int(x);
    if ((x < 0) || (x > 256))
        goto bad_fmt;
    format_engine = xmalloc_array(char, x);
    undump_things(format_engine[0], x);
    format_engine[x - 1] = 0;
    if (strcmp(engine_name, (string) format_engine)) {
        fprintf(stdout, "---! %s was written by %s\n", (string) (name_of_file + 1), format_engine);
        free(format_engine);
        goto bad_fmt;
    }
    free(format_engine);
    undump_int(x);
    if (x != 457477274L) {
        fprintf(stdout, "---! %s doesn't match xetex.pool\n", (string) (name_of_file + 1));
        goto bad_fmt;
    }
    undump_int(x);
    if (x != 1073741823L)
        goto bad_fmt;
    undump_int(hash_high);
    if ((hash_high < 0) || (hash_high > sup_hash_extra))
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
    {
        register integer for_end;
        x = (HASH_BASE + 1);
        for_end = hash_top;
        if (x <= for_end)
            do
                hash[x] = hash[HASH_BASE];
            while (x++ < for_end);
    }
    zeqtb = xmalloc_array(memory_word, eqtb_top + 1);
    eqtb = zeqtb;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.u.B0 = UNDEFINED_CS;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.v.RH = -268435455L;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.u.B1 = LEVEL_ZERO;
    {
        register integer for_end;
        x = (EQTB_SIZE + 1);
        for_end = eqtb_top;
        if (x <= for_end)
            do
                eqtb[x] = eqtb[UNDEFINED_CONTROL_SEQUENCE];
            while (x++ < for_end);
    }
    {
        undump_int(x);
        if ((x < 0) || (x > 1))
            goto bad_fmt;
        else
            eTeX_mode = x;
    }
    if ((eTeX_mode == 1)) {
        max_reg_num = 32767;
        max_reg_help_line = S(A_register_number_must_be_be_Z1/*"A register number must be between 0 and 32767."*/);
    } else {

        max_reg_num = 255;
        max_reg_help_line = S(A_register_number_must_be_be/*tween 0 and 255.*/);
    }
    undump_int(x);
    if (x != mem_bot)
        goto bad_fmt;
    undump_int(mem_top);
    if (mem_bot + 1100 > mem_top)
        goto bad_fmt;
    cur_list.head = mem_top - 1;
    cur_list.tail = mem_top - 1;
    page_tail = mem_top - 2;
    mem_min = mem_bot - extra_mem_bot;
    mem_max = mem_top + extra_mem_top;
    yzmem = xmalloc_array(memory_word, mem_max - mem_min + 1);
    zmem = yzmem - mem_min;
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
    undump_int(x);
    if (x != 1296847960L)
        goto bad_fmt;
    undump_int(x);
    if (x == 1)
        mltex_enabled_p = true;
    else if (x != 0)
        goto bad_fmt;
    {
        undump_int(x);
        if (x < 0)
            goto bad_fmt;
        if (x > sup_pool_size - pool_free)
            _tt_abort ("must increase string_pool_size");

        pool_ptr = x;
    }
    if (pool_size < pool_ptr + pool_free)
        pool_size = pool_ptr + pool_free;
    {
        undump_int(x);
        if (x < 0)
            goto bad_fmt;
        if (x > sup_max_strings - strings_free)
            _tt_abort ("must increase sup_strings");

        str_ptr = x;
    }
    if (max_strings < str_ptr + strings_free)
        max_strings = str_ptr + strings_free;
    str_start = xmalloc_array(pool_pointer, max_strings);
    undump_checked_things(0, pool_ptr, str_start[(TOO_BIG_CHAR) - 65536L], str_ptr - 65535L);
    str_pool = xmalloc_array(packed_UTF16_code, pool_size);
    undump_things(str_pool[0], pool_ptr);
    init_str_ptr = str_ptr;
    init_pool_ptr = /*:1345 */ pool_ptr;
    {
        undump_int(x);
        if ((x < mem_bot + 1019) || (x > mem_top - 15))
            goto bad_fmt;
        else
            lo_mem_max = x;
    }
    {
        undump_int(x);
        if ((x < mem_bot + 20) || (x > lo_mem_max))
            goto bad_fmt;
        else
            rover = x;
    }
    if ((eTeX_mode == 1)) {
        register integer for_end;
        k = INT_VAL;
        for_end = INTER_CHAR_VAL;
        if (k <= for_end)
            do {
                undump_int(x);
                if ((x < -268435455L) || (x > lo_mem_max))
                    goto bad_fmt;
                else
                    sa_root[k] = x;
            }
            while (k++ < for_end);
    }
    p = mem_bot;
    q = rover;
    do {
        undump_things(mem[p], q + 2 - p);
        p = q + mem[q].hh.v.LH;
        if ((p > lo_mem_max) || ((q >= mem[q + 1].hh.v.RH) && (mem[q + 1].hh.v.RH != rover)))
            goto bad_fmt;
        q = mem[q + 1].hh.v.RH;
    } while (!(q == rover));
    undump_things(mem[p], lo_mem_max + 1 - p);
    if (mem_min < mem_bot - 2) {
        p = mem[rover + 1].hh.v.LH;
        q = mem_min + 1;
        mem[mem_min].hh.v.RH = -268435455L;
        mem[mem_min].hh.v.LH = -268435455L;
        mem[p + 1].hh.v.RH = q;
        mem[rover + 1].hh.v.LH = q;
        mem[q + 1].hh.v.RH = rover;
        mem[q + 1].hh.v.LH = p;
        mem[q].hh.v.RH = 1073741823L;
        mem[q].hh.v.LH = mem_bot - q;
    }
    {
        undump_int(x);
        if ((x < lo_mem_max + 1) || (x > mem_top - 14))
            goto bad_fmt;
        else
            hi_mem_min = x;
    }
    {
        undump_int(x);
        if ((x < -268435455L) || (x > mem_top))
            goto bad_fmt;
        else
            avail = x;
    }
    mem_end = mem_top;
    undump_things(mem[hi_mem_min], mem_end + 1 - hi_mem_min);
    undump_int(var_used);
    undump_int(dyn_used);
    k = ACTIVE_BASE;
    do {
        undump_int(x);
        if ((x < 1) || (k + x > (EQTB_SIZE + 1)))
            goto bad_fmt;
        undump_things(eqtb[k], x);
        k = k + x;
        undump_int(x);
        if ((x < 0) || (k + x > (EQTB_SIZE + 1)))
            goto bad_fmt;
        {
            register integer for_end;
            j = k;
            for_end = k + x - 1;
            if (j <= for_end)
                do
                    eqtb[j] = eqtb[k - 1];
                while (j++ < for_end);
        }
        k = k + x;
    } while (!(k > EQTB_SIZE));
    if (hash_high > 0)
        undump_things(eqtb[(EQTB_SIZE + 1)], hash_high);
    {
        undump_int(x);
        if ((x < HASH_BASE) || (x > hash_top))
            goto bad_fmt;
        else
            par_loc = x;
    }
    par_token = CS_TOKEN_FLAG + par_loc;
    {
        undump_int(x);
        if ((x < HASH_BASE) || (x > hash_top))
            goto bad_fmt;
        else
            write_loc = x;
    }
    {
        register integer for_end;
        p = 0;
        for_end = PRIM_SIZE;
        if (p <= for_end)
            do
                undump_hh(prim[p]);
            while (p++ < for_end);
    }
    {
        register integer for_end;
        p = 0;
        for_end = PRIM_SIZE;
        if (p <= for_end)
            do
                undump_wd(prim_eqtb[p]);
            while (p++ < for_end);
    }
    {
        undump_int(x);
        if ((x < HASH_BASE) || (x > FROZEN_CONTROL_SEQUENCE))
            goto bad_fmt;
        else
            hash_used = x;
    }
    p = (HASH_BASE - 1);
    do {
        {
            undump_int(x);
            if ((x < p + 1) || (x > hash_used))
                goto bad_fmt;
            else
                p = x;
        }
        undump_hh(hash[p]);
    } while (!(p == hash_used));
    undump_things(hash[hash_used + 1], (UNDEFINED_CONTROL_SEQUENCE - 1) - hash_used);
    if (hash_high > 0) {
        undump_things(hash[(EQTB_SIZE + 1)], hash_high);
    }
    undump_int(cs_count);
    {
        undump_int(x);
        if (x < 7)
            goto bad_fmt;
        if (x > sup_font_mem_size)
            _tt_abort ("must increase font_mem_size");

        fmem_ptr = x;
    }
    if (fmem_ptr > font_mem_size)
        font_mem_size = fmem_ptr;
    font_info = xmalloc_array(fmemory_word, font_mem_size);
    undump_things(font_info[0], fmem_ptr);
    {
        undump_int(x);
        if (x < FONT_BASE)
            goto bad_fmt;
        if (x > (FONT_BASE + 9000))
            _tt_abort ("must increase font_max");

        font_ptr = x;
    }
    {
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
        {
            register integer for_end;
            k = FONT_BASE;
            for_end = font_ptr;
            if (k <= for_end)
                do
                    font_mapping[k] = 0;
                while (k++ < for_end);
        }
        undump_things(font_check[FONT_BASE], font_ptr + 1);
        undump_things(font_size[FONT_BASE], font_ptr + 1);
        undump_things(font_dsize[FONT_BASE], font_ptr + 1);
        undump_checked_things(-268435455L, 1073741823L, font_params[FONT_BASE], font_ptr + 1);
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
        undump_checked_things(-268435455L, lo_mem_max, font_glue[FONT_BASE], font_ptr + 1);
        undump_checked_things(0, fmem_ptr - 1, bchar_label[FONT_BASE], font_ptr + 1);
        undump_checked_things(0, TOO_BIG_CHAR, font_bchar[FONT_BASE],
                              font_ptr + 1);
        undump_checked_things(0, TOO_BIG_CHAR, font_false_bchar[FONT_BASE],
                              font_ptr + 1);
    }
    {
        undump_int(x);
        if (x < 0)
            goto bad_fmt;
        if (x > hyph_size)
            _tt_abort ("must increase hyph_size");

        hyph_count = x;
    }
    {
        undump_int(x);
        if (x < HYPH_PRIME)
            goto bad_fmt;
        if (x > hyph_size)
            _tt_abort ("must increase hyph_size");

        hyph_next = x;
    }
    j = 0;
    {
        register integer for_end;
        k = 1;
        for_end = hyph_count;
        if (k <= for_end)
            do {
                undump_int(j);
                if (j < 0)
                    goto bad_fmt;
                if (j > 65535L) {
                    hyph_next = j / 65536L;
                    j = j - hyph_next * 65536L;
                } else
                    hyph_next = 0;
                if ((j >= hyph_size) || (hyph_next > hyph_size))
                    goto bad_fmt;
                hyph_link[j] = hyph_next;
                {
                    undump_int(x);
                    if ((x < 0) || (x > str_ptr))
                        goto bad_fmt;
                    else
                        hyph_word[j] = x;
                }
                {
                    undump_int(x);
                    if ((x < -268435455L) || (x > 1073741823L))
                        goto bad_fmt;
                    else
                        hyph_list[j] = x;
                }
            }
            while (k++ < for_end);
    }
    j++;
    if (j < HYPH_PRIME)
        j = HYPH_PRIME;
    hyph_next = j;
    if (hyph_next >= hyph_size)
        hyph_next = HYPH_PRIME;
    else if (hyph_next >= HYPH_PRIME)
        hyph_next++;
    {
        undump_int(x);
        if (x < 0)
            goto bad_fmt;
        if (x > trie_size)
	    _tt_abort ("must increase trie_size");

        j = x;
    }

    trie_max = j;
    {
        undump_int(x);
        if ((x < 0) || (x > j))
            goto bad_fmt;
        else
            hyph_start = x;
    }
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
    {
        undump_int(x);
        if (x < 0)
            goto bad_fmt;
        if (x > trie_op_size)
	    _tt_abort ("must increase trie_op_size");

        j = x;
    }

    trie_op_ptr = j;

    undump_things(hyf_distance[1], j);
    undump_things(hyf_num[1], j);
    undump_upper_check_things(max_trie_op, hyf_next[1], j);

    {
        register integer for_end;
        k = 0;
        for_end = BIGGEST_LANG;
        if (k <= for_end)
            do
                trie_used[k] = 0;
            while (k++ < for_end);
    }

    k = (BIGGEST_LANG + 1);
    while (j > 0) {

        {
            undump_int(x);
            if ((x < 0) || (x > k - 1))
                goto bad_fmt;
            else
                k = x;
        }
        {
            undump_int(x);
            if ((x < 1) || (x > j))
                goto bad_fmt;
            else
                x = x;
        }

        trie_used[k] = x;
        j = j - x;
        op_start[k] = j;
    }

    trie_not_ready = false;

    {
        undump_int(x);
        if ((x < BATCH_MODE) || (x > ERROR_STOP_MODE))
            goto bad_fmt;
        else
            interaction = x;
    }
    if (interaction_option != UNSPECIFIED_MODE)
        interaction = interaction_option;
    {
        undump_int(x);
        if ((x < 0) || (x > str_ptr))
            goto bad_fmt;
        else
            format_ident = x;
    }
    undump_int(x);
    if (x != 69069L)
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
        if ((eTeX_mode == 1))
            show_save_groups();
    }
    while (cond_ptr != -268435455L) {

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
                        if (cur_mark[c] != -268435455L)
                            delete_token_ref(cur_mark[c]);
                    while (c++ < for_end) ;
            }
            if (sa_root[MARK_VAL] != -268435455L) {

                if (do_marks(3, 0, sa_root[MARK_VAL]))
                    sa_root[MARK_VAL] = -268435455L;
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
            if (last_glue != 1073741823L)
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

static boolean
init_terminal(string input_file_name)
{
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

    buffer[k++] = ' ';

    /* Find the end of the buffer.  */
    for (last = first; buffer[last]; ++last)
	;

    /* Make `last' be one past the last non-blank character in `buffer'.  */
    /* ??? The test for '\r' should not be necessary.  */
    for (--last; last >= first
	     && ISBLANK (buffer[last]) && buffer[last] != '\r'; --last)
	;
    last++;

    /* TODO: we don't want/need special stdin handling, so the following code
     * should disappear */

    if (last > first) {
        cur_input.loc = first;
        while ((cur_input.loc < last) && (buffer[cur_input.loc] == ' '))
            cur_input.loc++;
        if (cur_input.loc < last)
            return true;
    }

    _tt_abort ("internal error TERMINPUT");
}


static void
initialize_more_variables(void)
{
    memory_word *mem = zmem, *eqtb = zeqtb;
    integer i, k;
    hyph_pointer z;

    doing_special = false;
    native_text_size = 128;
    native_text = xmalloc(native_text_size * sizeof(UTF16_code));

    if (interaction_option == UNSPECIFIED_MODE)
        interaction = ERROR_STOP_MODE;
    else
        interaction = interaction_option;

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
    cur_list.eTeX_aux = -268435455L;
    cur_list.aux.cint = -65536000L;
    cur_list.ml = 0;
    cur_list.pg = 0;
    shown_mode = 0;
    page_contents = EMPTY;
    page_tail = mem_top - 2;
    last_glue = 1073741823L;
    last_penalty = 0;
    last_kern = 0;
    page_so_far[7] = 0;
    page_max_depth = 0;

    {
        register integer for_end;
        k = INT_BASE;
        for_end = EQTB_SIZE;
        if (k <= for_end)
            do
                xeq_level[k] = LEVEL_ONE;
            while (k++ < for_end);
    }

    no_new_control_sequence = true;
    prim[0].v.LH = 0;
    prim[0].v.RH = 0;

    {
        register integer for_end;
        k = 1;
        for_end = PRIM_SIZE;
        if (k <= for_end)
            do
                prim[k] = prim[0];
            while (k++ < for_end);
    }

    prim_eqtb[0].hh.u.B1 = LEVEL_ZERO;
    prim_eqtb[0].hh.u.B0 = UNDEFINED_CS;
    prim_eqtb[0].hh.v.RH = -268435455L;

    {
        register integer for_end;
        k = 1;
        for_end = PRIM_SIZE;
        if (k <= for_end)
            do
                prim_eqtb[k] = prim_eqtb[0];
            while (k++ < for_end);
    }

    save_ptr = 0;
    cur_level = LEVEL_ONE;
    cur_group = BOTTOM_LEVEL;
    cur_boundary = 0;
    max_save_stack = 0;
    mag_set = 0;
    expand_depth_count = 0;
    is_in_csname = false;
    cur_mark[TOP_MARK_CODE] = -268435455L;
    cur_mark[FIRST_MARK_CODE] = -268435455L;
    cur_mark[BOT_MARK_CODE] = -268435455L;
    cur_mark[SPLIT_FIRST_MARK_CODE] = -268435455L;
    cur_mark[SPLIT_BOT_MARK_CODE] = -268435455L;
    cur_val = 0;
    cur_val_level = INT_VAL;
    radix = 0;
    cur_order = NORMAL;

    {
        register integer for_end;
        k = 0;
        for_end = 16;
        if (k <= for_end)
            do
                read_open[k] = CLOSED;
            while (k++ < for_end);
    }

    cond_ptr = -268435455L;
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
    down_ptr = -268435455L;
    right_ptr = -268435455L;
    adjust_tail = -268435455L;
    last_badness = 0;
    pre_adjust_tail = -268435455L;
    pack_begin_line = 0;
    empty.v.RH = EMPTY;
    empty.v.LH = -268435455L;
    null_delimiter.u.B0 = 0;
    null_delimiter.u.B1 = 0;
    null_delimiter.u.B2 = 0;
    null_delimiter.u.B3 = 0;
    align_ptr = -268435455L;
    cur_align = -268435455L;
    cur_span = -268435455L;
    cur_loop = -268435455L;
    cur_head = -268435455L;
    cur_tail = -268435455L;
    cur_pre_head = -268435455L;
    cur_pre_tail = -268435455L;
    max_hyph_char = TOO_BIG_LANG;

    {
        register integer for_end;
        z = 0;
        for_end = hyph_size;
        if (z <= for_end)
            do {
                hyph_word[z] = 0;
                hyph_list[z] = -268435455L;
                hyph_link[z] = 0;
            }
            while (z++ < for_end);
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
    {
        register integer for_end;
        k = 0;
        for_end = 17;
        if (k <= for_end)
            do
                write_open[k] = false;
            while (k++ < for_end);
    }
    LR_ptr = -268435455L;
    LR_problems = 0;
    cur_dir = LEFT_TO_RIGHT;
    pseudo_files = -268435455L;
    sa_root[MARK_VAL] = -268435455L;
    sa_null.hh.v.LH = -268435455L;
    sa_null.hh.v.RH = -268435455L;
    sa_chain = -268435455L;
    sa_level = LEVEL_ZERO;
    disc_ptr[LAST_BOX_CODE] = -268435455L;
    disc_ptr[VSPLIT_CODE] = -268435455L;
    edit_name_start = 0;
    stop_at_space = true;
    mltex_enabled_p = false;

    if (in_initex_mode) {
        {
            register integer for_end;
            k = mem_bot + 1;
            for_end = mem_bot + 19;
            if (k <= for_end)
                do
                    mem[k].cint = 0;
                while (k++ < for_end);
        }
        k = mem_bot;
        while (k <= mem_bot + 19) {

            mem[k].hh.v.RH = -268435454L;
            mem[k].hh.u.B0 = NORMAL;
            mem[k].hh.u.B1 = NORMAL;
            k = k + 4;
        }
        mem[mem_bot + 6].cint = 65536L;
        mem[mem_bot + 4].hh.u.B0 = FIL;
        mem[mem_bot + 10].cint = 65536L;
        mem[mem_bot + 8].hh.u.B0 = FILL;
        mem[mem_bot + 14].cint = 65536L;
        mem[mem_bot + 12].hh.u.B0 = FIL;
        mem[mem_bot + 15].cint = 65536L;
        mem[mem_bot + 12].hh.u.B1 = FIL;
        mem[mem_bot + 18].cint = -65536L;
        mem[mem_bot + 16].hh.u.B0 = FIL;
        rover = mem_bot + 20;
        mem[rover].hh.v.RH = 1073741823L;
        mem[rover].hh.v.LH = 1000;
        mem[rover + 1].hh.v.LH = rover;
        mem[rover + 1].hh.v.RH = rover;
        lo_mem_max = rover + 1000;
        mem[lo_mem_max].hh.v.RH = -268435455L;
        mem[lo_mem_max].hh.v.LH = -268435455L;
        {
            register integer for_end;
            k = mem_top - 14;
            for_end = mem_top;
            if (k <= for_end)
                do
                    mem[k] = mem[lo_mem_max];
                while (k++ < for_end);
        }
        mem[mem_top - 10].hh.v.LH = (CS_TOKEN_FLAG + 2243231);
        mem[mem_top - 9].hh.v.RH = UINT16_MAX + 1;
        mem[mem_top - 9].hh.v.LH = -268435455L;
        mem[mem_top - 7].hh.u.B0 = HYPHENATED;
        mem[mem_top - 6].hh.v.LH = 1073741823L;
        mem[mem_top - 7].hh.u.B1 = 0;
        mem[mem_top].hh.u.B1 = 255;
        mem[mem_top].hh.u.B0 = SPLIT_UP;
        mem[mem_top].hh.v.RH = mem_top;
        mem[mem_top - 2].hh.u.B0 = GLUE_NODE;
        mem[mem_top - 2].hh.u.B1 = NORMAL;
        avail = -268435455L;
        mem_end = mem_top;
        hi_mem_min = mem_top - 14;
        var_used = mem_bot + 20 - mem_bot;
        dyn_used = HI_MEM_STAT_USAGE;
        eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.u.B0 = UNDEFINED_CS;
        eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.v.RH = -268435455L;
        eqtb[UNDEFINED_CONTROL_SEQUENCE].hh.u.B1 = LEVEL_ZERO;
        {
            register integer for_end;
            k = ACTIVE_BASE;
            for_end = eqtb_top;
            if (k <= for_end)
                do
                    eqtb[k] = eqtb[UNDEFINED_CONTROL_SEQUENCE];
                while (k++ < for_end);
        }
        eqtb[GLUE_BASE].hh.v.RH = mem_bot;
        eqtb[GLUE_BASE].hh.u.B1 = LEVEL_ONE;
        eqtb[GLUE_BASE].hh.u.B0 = GLUE_REF;
        {
            register integer for_end;
            k = (GLUE_BASE + 1);
            for_end = (LOCAL_BASE - 1);
            if (k <= for_end)
                do
                    eqtb[k] = eqtb[GLUE_BASE];
                while (k++ < for_end);
        }
        mem[mem_bot].hh.v.RH = mem[mem_bot].hh.v.RH + 531;
        eqtb[PAR_SHAPE_LOC].hh.v.RH = -268435455L;
        eqtb[PAR_SHAPE_LOC].hh.u.B0 = SHAPE_REF;
        eqtb[PAR_SHAPE_LOC].hh.u.B1 = LEVEL_ONE;
        {
            register integer for_end;
            k = ETEX_PEN_BASE;
            for_end = (ETEX_PENS - 1);
            if (k <= for_end)
                do
                    eqtb[k] = eqtb[PAR_SHAPE_LOC];
                while (k++ < for_end);
        }
        {
            register integer for_end;
            k = OUTPUT_ROUTINE_LOC;
            for_end = TOKS_BASE + 255;
            if (k <= for_end)
                do
                    eqtb[k] = eqtb[UNDEFINED_CONTROL_SEQUENCE];
                while (k++ < for_end);
        }
        eqtb[BOX_BASE].hh.v.RH = -268435455L;
        eqtb[BOX_BASE].hh.u.B0 = BOX_REF;
        eqtb[BOX_BASE].hh.u.B1 = LEVEL_ONE;
        {
            register integer for_end;
            k = (BOX_BASE + 1);
            for_end = BOX_BASE + 255;
            if (k <= for_end)
                do
                    eqtb[k] = eqtb[BOX_BASE];
                while (k++ < for_end);
        }
        eqtb[CUR_FONT_LOC].hh.v.RH = FONT_BASE;
        eqtb[CUR_FONT_LOC].hh.u.B0 = DATA;
        eqtb[CUR_FONT_LOC].hh.u.B1 = LEVEL_ONE;
        {
            register integer for_end;
            k = MATH_FONT_BASE;
            for_end = MATH_FONT_BASE + 767;
            if (k <= for_end)
                do
                    eqtb[k] = eqtb[CUR_FONT_LOC];
                while (k++ < for_end);
        }
        eqtb[CAT_CODE_BASE].hh.v.RH = 0;
        eqtb[CAT_CODE_BASE].hh.u.B0 = DATA;
        eqtb[CAT_CODE_BASE].hh.u.B1 = LEVEL_ONE;
        {
            register integer for_end;
            k = (CAT_CODE_BASE + 1);
            for_end = (INT_BASE - 1);
            if (k <= for_end)
                do
                    eqtb[k] = eqtb[CAT_CODE_BASE];
                while (k++ < for_end);
        }
        {
            register integer for_end;
            k = 0;
            for_end = (NUMBER_USVS - 1);
            if (k <= for_end)
                do {
                    eqtb[CAT_CODE_BASE + k].hh.v.RH = OTHER_CHAR;
                    eqtb[MATH_CODE_BASE + k].hh.v.RH = k;
                    eqtb[SF_CODE_BASE + k].hh.v.RH = 1000;
                }
                while (k++ < for_end);
        }
        eqtb[(CAT_CODE_BASE + 13)].hh.v.RH = CAR_RET;
        eqtb[(CAT_CODE_BASE + 32)].hh.v.RH = SPACER;
        eqtb[(CAT_CODE_BASE + 92)].hh.v.RH = ESCAPE;
        eqtb[(CAT_CODE_BASE + 37)].hh.v.RH = COMMENT;
        eqtb[(CAT_CODE_BASE + 127)].hh.v.RH = INVALID_CHAR;
        eqtb[CAT_CODE_BASE].hh.v.RH = IGNORE;
        {
            register integer for_end;
            k = 48 /*"0" */ ;
            for_end = 57 /*"9" */ ;
            if (k <= for_end)
                do
                    eqtb[MATH_CODE_BASE + k].hh.v.RH = k + set_class(VAR_FAM_CLASS);
                while (k++ < for_end);
        }
        {
            register integer for_end;
            k = 65 /*"A" */ ;
            for_end = 90 /*"Z" */ ;
            if (k <= for_end)
                do {
                    eqtb[CAT_CODE_BASE + k].hh.v.RH = LETTER;
                    eqtb[CAT_CODE_BASE + k + 32].hh.v.RH = LETTER;
                    eqtb[MATH_CODE_BASE + k].hh.v.RH =
                        k + set_family(1) + set_class(VAR_FAM_CLASS);
                    eqtb[MATH_CODE_BASE + k + 32].hh.v.RH =
                        k + 32 + set_family(1) + set_class(VAR_FAM_CLASS);
                    eqtb[LC_CODE_BASE + k].hh.v.RH = k + 32;
                    eqtb[LC_CODE_BASE + k + 32].hh.v.RH = k + 32;
                    eqtb[UC_CODE_BASE + k].hh.v.RH = k;
                    eqtb[UC_CODE_BASE + k + 32].hh.v.RH = k;
                    eqtb[SF_CODE_BASE + k].hh.v.RH = 999;
                }
                while (k++ < for_end);
        }
        {
            register integer for_end;
            k = INT_BASE;
            for_end = (DEL_CODE_BASE - 1);
            if (k <= for_end)
                do
                    eqtb[k].cint = 0;
                while (k++ < for_end);
        }
        eqtb[(INT_BASE + 55)].cint = 256;
        eqtb[(INT_BASE + 56)].cint = -1;
        eqtb[(INT_BASE + 17)].cint = 1000;
        eqtb[(INT_BASE + 1)].cint = 10000;
        eqtb[(INT_BASE + 41)].cint = 1;
        eqtb[(INT_BASE + 40)].cint = 25;
        eqtb[(INT_BASE + 45)].cint = 92 /*"\" */ ;
        eqtb[(INT_BASE + 48)].cint = CARRIAGE_RETURN;
        {
            register integer for_end;
            k = 0;
            for_end = (NUMBER_CHARS - 1);
            if (k <= for_end)
                do
                    eqtb[DEL_CODE_BASE + k].cint = -1;
                while (k++ < for_end);
        }
        eqtb[(DEL_CODE_BASE + 46)].cint = 0;
        {
            register integer for_end;
            k = DIMEN_BASE;
            for_end = EQTB_SIZE;
            if (k <= for_end)
                do
                    eqtb[k].cint = 0;
                while (k++ < for_end);
        }
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
        {
            register integer for_end;
            k = -(integer) trie_op_size;
            for_end = trie_op_size;
            if (k <= for_end)
                do
                    trie_op_hash[k] = 0;
                while (k++ < for_end);
        }
        {
            register integer for_end;
            k = 0;
            for_end = BIGGEST_LANG;
            if (k <= for_end)
                do
                    trie_used[k] = min_trie_op;
                while (k++ < for_end);
        }
        max_op_used = min_trie_op;
        trie_op_ptr = 0;
        trie_not_ready = true;
        hash[FROZEN_PROTECTION].v.RH = S(inaccessible);
        if (in_initex_mode)
            format_ident = S(__INITEX_);
        hash[END_WRITE].v.RH = S(endwrite);
        eqtb[END_WRITE].hh.u.B1 = LEVEL_ONE;
        eqtb[END_WRITE].hh.u.B0 = OUTER_CALL;
        eqtb[END_WRITE].hh.v.RH = -268435455L;
        eTeX_mode = 0;
        max_reg_num = 255;
        max_reg_help_line = S(A_register_number_must_be_be/*tween 0 and 255.*/);
        {
            register integer for_end;
            i = INT_VAL;
            for_end = INTER_CHAR_VAL;
            if (i <= for_end)
                do
                    sa_root[i] = -268435455L;
                while (i++ < for_end);
        }
        eqtb[(ETEX_STATE_BASE + 11)].cint = 63;
    }

    synctexoffset = (INT_BASE + 83);
}


/*:1370*//*1371: */
static void
initialize_primitives(void)
{
    memory_word *eqtb = zeqtb;

    no_new_control_sequence = false;
    first = 0;

    primitive(S(lineskip), ASSIGN_GLUE, 2252240L /*glue_base 0*/);
    primitive(S(baselineskip), ASSIGN_GLUE, 2252241L /*glue_base 1*/);
    primitive(S(parskip), ASSIGN_GLUE, 2252242L /*glue_base 2*/);
    primitive(S(abovedisplayskip), ASSIGN_GLUE, 2252243L /*glue_base 3*/);
    primitive(S(belowdisplayskip), ASSIGN_GLUE, 2252244L /*glue_base 4*/);
    primitive(S(abovedisplayshortskip), ASSIGN_GLUE, 2252245L /*glue_base 5*/);
    primitive(S(belowdisplayshortskip), ASSIGN_GLUE, 2252246L /*glue_base 6*/);
    primitive(S(leftskip), ASSIGN_GLUE, 2252247L /*glue_base 7*/);
    primitive(S(rightskip), ASSIGN_GLUE, 2252248L /*glue_base 8*/);
    primitive(S(topskip), ASSIGN_GLUE, 2252249L /*glue_base 9*/);
    primitive(S(splittopskip), ASSIGN_GLUE, 2252250L /*glue_base 10*/);
    primitive(S(tabskip), ASSIGN_GLUE, 2252251L /*glue_base 11*/);
    primitive(S(spaceskip), ASSIGN_GLUE, 2252252L /*glue_base 12*/);
    primitive(S(xspaceskip), ASSIGN_GLUE, 2252253L /*glue_base 13*/);
    primitive(S(parfillskip), ASSIGN_GLUE, 2252254L /*glue_base 14*/);
    primitive(S(XeTeXlinebreakskip), ASSIGN_GLUE, 2252255L /*glue_base 15*/);
    primitive(S(thinmuskip), ASSIGN_MU_GLUE, 2252256L /*glue_base 16*/);
    primitive(S(medmuskip), ASSIGN_MU_GLUE, 2252257L /*glue_base 17*/);
    primitive(S(thickmuskip), ASSIGN_MU_GLUE, 2252258L /*glue_base 18*/);
    primitive(S(output), ASSIGN_TOKS, 2252772L /*output_routine_loc*/);
    primitive(S(everypar), ASSIGN_TOKS, 2252773L /*every_par_loc*/);
    primitive(S(everymath), ASSIGN_TOKS, 2252774L /*every_math_loc*/);
    primitive(S(everydisplay), ASSIGN_TOKS, 2252775L /*every_display_loc*/);
    primitive(S(everyhbox), ASSIGN_TOKS, 2252776L /*every_hbox_loc*/);
    primitive(S(everyvbox), ASSIGN_TOKS, 2252777L /*every_vbox_loc*/);
    primitive(S(everyjob), ASSIGN_TOKS, 2252778L /*every_job_loc*/);
    primitive(S(everycr), ASSIGN_TOKS, 2252779L /*every_cr_loc*/);
    primitive(S(errhelp), ASSIGN_TOKS, 2252780L /*err_help_loc*/);
    primitive(S(pretolerance), ASSIGN_INT, 8938740L /*int_base 0*/);
    primitive(S(tolerance), ASSIGN_INT, 8938741L /*int_base 1*/);
    primitive(S(linepenalty), ASSIGN_INT, 8938742L /*int_base 2*/);
    primitive(S(hyphenpenalty), ASSIGN_INT, 8938743L /*int_base 3*/);
    primitive(S(exhyphenpenalty), ASSIGN_INT, 8938744L /*int_base 4*/);
    primitive(S(clubpenalty), ASSIGN_INT, 8938745L /*int_base 5*/);
    primitive(S(widowpenalty), ASSIGN_INT, 8938746L /*int_base 6*/);
    primitive(S(displaywidowpenalty), ASSIGN_INT, 8938747L /*int_base 7*/);
    primitive(S(brokenpenalty), ASSIGN_INT, 8938748L /*int_base 8*/);
    primitive(S(binoppenalty), ASSIGN_INT, 8938749L /*int_base 9*/);
    primitive(S(relpenalty), ASSIGN_INT, 8938750L /*int_base 10*/);
    primitive(S(predisplaypenalty), ASSIGN_INT, 8938751L /*int_base 11*/);
    primitive(S(postdisplaypenalty), ASSIGN_INT, 8938752L /*int_base 12*/);
    primitive(S(interlinepenalty), ASSIGN_INT, 8938753L /*int_base 13*/);
    primitive(S(doublehyphendemerits), ASSIGN_INT, 8938754L /*int_base 14*/);
    primitive(S(finalhyphendemerits), ASSIGN_INT, 8938755L /*int_base 15*/);
    primitive(S(adjdemerits), ASSIGN_INT, 8938756L /*int_base 16*/);
    primitive(S(mag), ASSIGN_INT, 8938757L /*int_base 17*/);
    primitive(S(delimiterfactor), ASSIGN_INT, 8938758L /*int_base 18*/);
    primitive(S(looseness), ASSIGN_INT, 8938759L /*int_base 19*/);
    primitive(S(time), ASSIGN_INT, 8938760L /*int_base 20*/);
    primitive(S(day), ASSIGN_INT, 8938761L /*int_base 21*/);
    primitive(S(month), ASSIGN_INT, 8938762L /*int_base 22*/);
    primitive(S(year), ASSIGN_INT, 8938763L /*int_base 23*/);
    primitive(S(showboxbreadth), ASSIGN_INT, 8938764L /*int_base 24*/);
    primitive(S(showboxdepth), ASSIGN_INT, 8938765L /*int_base 25*/);
    primitive(S(hbadness), ASSIGN_INT, 8938766L /*int_base 26*/);
    primitive(S(vbadness), ASSIGN_INT, 8938767L /*int_base 27*/);
    primitive(S(pausing), ASSIGN_INT, 8938768L /*int_base 28*/);
    primitive(S(tracingonline), ASSIGN_INT, 8938769L /*int_base 29*/);
    primitive(S(tracingmacros), ASSIGN_INT, 8938770L /*int_base 30*/);
    primitive(S(tracingstats), ASSIGN_INT, 8938771L /*int_base 31*/);
    primitive(S(tracingparagraphs), ASSIGN_INT, 8938772L /*int_base 32*/);
    primitive(S(tracingpages), ASSIGN_INT, 8938773L /*int_base 33*/);
    primitive(S(tracingoutput), ASSIGN_INT, 8938774L /*int_base 34*/);
    primitive(S(tracinglostchars), ASSIGN_INT, 8938775L /*int_base 35*/);
    primitive(S(tracingcommands), ASSIGN_INT, 8938776L /*int_base 36*/);
    primitive(S(tracingrestores), ASSIGN_INT, 8938777L /*int_base 37*/);
    primitive(S(uchyph), ASSIGN_INT, 8938778L /*int_base 38*/);
    primitive(S(outputpenalty), ASSIGN_INT, 8938779L /*int_base 39*/);
    primitive(S(maxdeadcycles), ASSIGN_INT, 8938780L /*int_base 40*/);
    primitive(S(hangafter), ASSIGN_INT, 8938781L /*int_base 41*/);
    primitive(S(floatingpenalty), ASSIGN_INT, 8938782L /*int_base 42*/);
    primitive(S(globaldefs), ASSIGN_INT, 8938783L /*int_base 43*/);
    primitive(S(fam), ASSIGN_INT, 8938784L /*int_base 44*/);
    primitive(S(escapechar), ASSIGN_INT, 8938785L /*int_base 45*/);
    primitive(S(defaulthyphenchar), ASSIGN_INT, 8938786L /*int_base 46*/);
    primitive(S(defaultskewchar), ASSIGN_INT, 8938787L /*int_base 47*/);
    primitive(S(endlinechar), ASSIGN_INT, 8938788L /*int_base 48*/);
    primitive(S(newlinechar), ASSIGN_INT, 8938789L /*int_base 49*/);
    primitive(S(language), ASSIGN_INT, 8938790L /*int_base 50*/);
    primitive(S(lefthyphenmin), ASSIGN_INT, 8938791L /*int_base 51*/);
    primitive(S(righthyphenmin), ASSIGN_INT, 8938792L /*int_base 52*/);
    primitive(S(holdinginserts), ASSIGN_INT, 8938793L /*int_base 53*/);
    primitive(S(errorcontextlines), ASSIGN_INT, 8938794L /*int_base 54*/);

    if (mltex_p) {
        mltex_enabled_p = true;
        primitive(S(charsubdefmax), ASSIGN_INT, 8938796L /*int_base 56*/);
        primitive(S(tracingcharsubdef), ASSIGN_INT, 8938797L /*int_base 57*/);
    }

    primitive(S(XeTeXlinebreakpenalty), ASSIGN_INT, 8938809L /*int_base 69*/);
    primitive(S(XeTeXprotrudechars), ASSIGN_INT, 8938810L /*int_base 70*/);
    primitive(S(parindent), ASSIGN_DIMEN, 10053192L /*dimen_base 0*/);
    primitive(S(mathsurround), ASSIGN_DIMEN, 10053193L /*dimen_base 1*/);
    primitive(S(lineskiplimit), ASSIGN_DIMEN, 10053194L /*dimen_base 2*/);
    primitive(S(hsize), ASSIGN_DIMEN, 10053195L /*dimen_base 3*/);
    primitive(S(vsize), ASSIGN_DIMEN, 10053196L /*dimen_base 4*/);
    primitive(S(maxdepth), ASSIGN_DIMEN, 10053197L /*dimen_base 5*/);
    primitive(S(splitmaxdepth), ASSIGN_DIMEN, 10053198L /*dimen_base 6*/);
    primitive(S(boxmaxdepth), ASSIGN_DIMEN, 10053199L /*dimen_base 7*/);
    primitive(S(hfuzz), ASSIGN_DIMEN, 10053200L /*dimen_base 8*/);
    primitive(S(vfuzz), ASSIGN_DIMEN, 10053201L /*dimen_base 9*/);
    primitive(S(delimitershortfall), ASSIGN_DIMEN, 10053202L /*dimen_base 10*/);
    primitive(S(nulldelimiterspace), ASSIGN_DIMEN, 10053203L /*dimen_base 11*/);
    primitive(S(scriptspace), ASSIGN_DIMEN, 10053204L /*dimen_base 12*/);
    primitive(S(predisplaysize), ASSIGN_DIMEN, 10053205L /*dimen_base 13*/);
    primitive(S(displaywidth), ASSIGN_DIMEN, 10053206L /*dimen_base 14*/);
    primitive(S(displayindent), ASSIGN_DIMEN, 10053207L /*dimen_base 15*/);
    primitive(S(overfullrule), ASSIGN_DIMEN, 10053208L /*dimen_base 16*/);
    primitive(S(hangindent), ASSIGN_DIMEN, 10053209L /*dimen_base 17*/);
    primitive(S(hoffset), ASSIGN_DIMEN, 10053210L /*dimen_base 18*/);
    primitive(S(voffset), ASSIGN_DIMEN, 10053211L /*dimen_base 19*/);
    primitive(S(emergencystretch), ASSIGN_DIMEN, 10053212L /*dimen_base 20*/);
    primitive(S(pdfpagewidth), ASSIGN_DIMEN, 10053213L /*dimen_base 21*/);
    primitive(S(pdfpageheight), ASSIGN_DIMEN, 10053214L /*dimen_base 22*/);
    primitive(32 /*" " */, 64 /*ex_space*/, 0);
    primitive(47 /*"/" */, 44 /*ital_corr*/, 0);
    primitive(S(accent), 45 /*accent*/, 0);
    primitive(S(advance), 92 /*advance*/, 0);
    primitive(S(afterassignment), 40 /*after_assignment*/, 0);
    primitive(S(aftergroup), 41 /*after_group*/, 0);
    primitive(S(begingroup), 61 /*begin_group*/, 0);
    primitive(S(char), 16 /*char_num*/, 0);
    primitive(S(csname), 109 /*cs_name*/, 0);
    primitive(S(delimiter), 15 /*delim_num*/, 0);
    primitive(S(XeTeXdelimiter), 15 /*delim_num*/, 1);
    primitive(S(Udelimiter), 15 /*delim_num*/, 1);
    primitive(S(divide), 94 /*divide*/, 0);
    primitive(S(endcsname), 67 /*end_cs_name*/, 0);
    primitive(S(endgroup), 62 /*end_group*/, 0);
    hash[2243228L /*frozen_end_group*/].v.RH = S(endgroup);
    eqtb[2243228L /*frozen_end_group*/] = eqtb[cur_val];
    primitive(S(expandafter), 104 /*expand_after*/, 0);
    primitive(S(font), 90 /*def_font*/, 0);
    primitive(S(fontdimen), ASSIGN_FONT_DIMEN, 0);
    primitive(S(halign), 32 /*halign*/, 0);
    primitive(S(hrule), 36 /*hrule*/, 0);
    primitive(S(ignorespaces), 39 /*ignore_spaces*/, 0);
    primitive(S(insert), 37 /*insert*/, 0);
    primitive(S(mark), 18 /*mark*/, 0);
    primitive(S(mathaccent), 46 /*math_accent*/, 0);
    primitive(S(XeTeXmathaccent), 46 /*math_accent*/, 1);
    primitive(S(Umathaccent), 46 /*math_accent*/, 1);
    primitive(S(mathchar), 17 /*math_char_num*/, 0);
    primitive(S(XeTeXmathcharnum), 17 /*math_char_num*/, 1);
    primitive(S(Umathcharnum), 17 /*math_char_num*/, 1);
    primitive(S(XeTeXmathchar), 17 /*math_char_num*/, 2);
    primitive(S(Umathchar), 17 /*math_char_num*/, 2);
    primitive(S(mathchoice), 54 /*math_choice*/, 0);
    primitive(S(multiply), 93 /*multiply*/, 0);
    primitive(S(noalign), 34 /*no_align*/, 0);
    primitive(S(noboundary), 65 /*no_boundary*/, 0);
    primitive(S(noexpand), 105 /*no_expand*/, 0);
    primitive(S(primitive), 105 /*no_expand*/, 1);
    primitive(S(nonscript), 55 /*non_script*/, 0);
    primitive(S(omit), 63 /*omit*/, 0);
    primitive(S(parshape), 85 /*set_shape*/, 2252771L /*par_shape_loc*/);
    primitive(S(penalty), 42 /*break_penalty*/, 0);
    primitive(S(prevgraf), 81 /*set_prev_graf*/, 0);
    primitive(S(radical), 66 /*radical*/, 0);
    primitive(S(XeTeXradical), 66 /*radical*/, 1);
    primitive(S(Uradical), 66 /*radical*/, 1);
    primitive(S(read), 98 /*read_to_cs*/, 0);
    primitive(S(relax), 0 /*relax*/, 1114112L /*too_big_usv*/);
    hash[2243233L /*frozen_relax*/].v.RH = S(relax);
    eqtb[2243233L /*frozen_relax*/] = eqtb[cur_val];
    primitive(S(setbox), 100 /*set_box*/, 0);
    primitive(S(the), 111 /*the*/, 0);
    primitive(S(toks), 72 /*toks_register*/, mem_bot);
    primitive(S(vadjust), 38 /*vadjust*/, 0);
    primitive(S(valign), 33 /*valign*/, 0);
    primitive(S(vcenter), 56 /*vcenter*/, 0);
    primitive(S(vrule), 35 /*vrule*/, 0);
    primitive(S(par), 13 /*par_end*/, 1114112L /*too_big_usv*/);
    par_loc = cur_val;
    par_token = 33554431L /*cs_token_flag*/ + par_loc;
    primitive(S(input), 106 /*input*/, 0);
    primitive(S(endinput), 106 /*input*/, 1);
    primitive(S(topmark), 112 /*top_bot_mark*/, 0 /*top_mark_code*/);
    primitive(S(firstmark), 112 /*top_bot_mark*/, 1 /*first_mark_code*/);
    primitive(S(botmark), 112 /*top_bot_mark*/, 2 /*bot_mark_code*/);
    primitive(S(splitfirstmark), 112 /*top_bot_mark*/, 3 /*split_first_mark_code*/);
    primitive(S(splitbotmark), 112 /*top_bot_mark*/, 4 /*split_bot_mark_code*/);
    primitive(S(count), 91 /*register*/, mem_bot + 0);
    primitive(S(dimen), 91 /*register*/, mem_bot + 1);
    primitive(S(skip), 91 /*register*/, mem_bot + 2);
    primitive(S(muskip), 91 /*register*/, mem_bot + 3);
    primitive(S(spacefactor), 80 /*set_aux*/, 104 /*hmode*/);
    primitive(S(prevdepth), 80 /*set_aux*/, 1 /*vmode*/);
    primitive(S(deadcycles), 83 /*set_page_int*/, 0);
    primitive(S(insertpenalties), 83 /*set_page_int*/, 1);
    primitive(S(wd), 84 /*set_box_dimen*/, 1 /*width_offset*/);
    primitive(S(ht), 84 /*set_box_dimen*/, 3 /*height_offset*/);
    primitive(S(dp), 84 /*set_box_dimen*/, 2 /*depth_offset*/);
    primitive(S(lastpenalty), 71 /*last_item*/, 0 /*int_val*/);
    primitive(S(lastkern), 71 /*last_item*/, 1 /*dimen_val*/);
    primitive(S(lastskip), 71 /*last_item*/, 2 /*glue_val*/);
    primitive(S(inputlineno), 71 /*last_item*/, 4 /*input_line_no_code*/);
    primitive(S(badness), 71 /*last_item*/, 5 /*badness_code*/);
    primitive(S(number), 110 /*convert*/, 0 /*number_code*/);
    primitive(S(romannumeral), 110 /*convert*/, 1 /*roman_numeral_code*/);
    primitive(S(string), 110 /*convert*/, 2 /*string_code*/);
    primitive(S(meaning), 110 /*convert*/, 3 /*meaning_code*/);
    primitive(S(fontname), 110 /*convert*/, 4 /*font_name_code*/);
    primitive(S(jobname), 110 /*convert*/, 15 /*job_name_code*/);
    primitive(S(leftmarginkern), 110 /*convert*/, 11 /*left_margin_kern_code*/);
    primitive(S(rightmarginkern), 110 /*convert*/, 12 /*right_margin_kern_code*/);
    primitive(S(Uchar), 110 /*convert*/, 13 /*XeTeX_Uchar_code*/);
    primitive(S(Ucharcat), 110 /*convert*/, 14 /*XeTeX_Ucharcat_code*/);
    primitive(S(if), 107 /*if_test*/, 0 /*if_char_code*/);
    primitive(S(ifcat), 107 /*if_test*/, 1 /*if_cat_code*/);
    primitive(S(ifnum), 107 /*if_test*/, 2 /*if_int_code*/);
    primitive(S(ifdim), 107 /*if_test*/, 3 /*if_dim_code*/);
    primitive(S(ifodd), 107 /*if_test*/, 4 /*if_odd_code*/);
    primitive(S(ifvmode), 107 /*if_test*/, 5 /*if_vmode_code*/);
    primitive(S(ifhmode), 107 /*if_test*/, 6 /*if_hmode_code*/);
    primitive(S(ifmmode), 107 /*if_test*/, 7 /*if_mmode_code*/);
    primitive(S(ifinner), 107 /*if_test*/, 8 /*if_inner_code*/);
    primitive(S(ifvoid), 107 /*if_test*/, 9 /*if_void_code*/);
    primitive(S(ifhbox), 107 /*if_test*/, 10 /*if_hbox_code*/);
    primitive(S(ifvbox), 107 /*if_test*/, 11 /*if_vbox_code*/);
    primitive(S(ifx), 107 /*if_test*/, 12 /*ifx_code*/);
    primitive(S(ifeof), 107 /*if_test*/, 13 /*if_eof_code*/);
    primitive(S(iftrue), 107 /*if_test*/, 14 /*if_true_code*/);
    primitive(S(iffalse), 107 /*if_test*/, 15 /*if_false_code*/);
    primitive(S(ifcase), 107 /*if_test*/, 16 /*if_case_code*/);
    primitive(S(ifprimitive), 107 /*if_test*/, 21 /*if_primitive_code*/);
    primitive(S(fi), 108 /*fi_or_else*/, 2 /*fi_code*/);
    hash[2243230L /*frozen_fi*/].v.RH = S(fi);
    eqtb[2243230L /*frozen_fi*/] = eqtb[cur_val];
    primitive(S(or), 108 /*fi_or_else*/, 4 /*or_code*/);
    primitive(S(else), 108 /*fi_or_else*/, 3 /*else_code*/);
    primitive(S(nullfont), 89 /*set_font*/, FONT_BASE);
    hash[2243238L /*frozen_null_font*/].v.RH = S(nullfont);
    eqtb[2243238L /*frozen_null_font*/] = eqtb[cur_val];
    primitive(S(span), 4 /*tab_mark*/, 65537L /*span_code*/);
    primitive(S(cr), 5 /*car_ret*/, 65538L /*cr_code*/);
    hash[2243227L /*frozen_cr*/].v.RH = S(cr);
    eqtb[2243227L /*frozen_cr*/] = eqtb[cur_val];
    primitive(S(crcr), 5 /*car_ret*/, 65539L /*cr_cr_code*/);
    hash[2243231L /*frozen_end_template*/].v.RH = S(endtemplate);
    hash[2243232L /*frozen_endv*/].v.RH = S(endtemplate);
    eqtb[2243232L /*frozen_endv*/].hh.u.B0 = 9 /*endv*/;
    eqtb[2243232L /*frozen_endv*/].hh.v.RH = mem_top - 11;
    eqtb[2243232L /*frozen_endv*/].hh.u.B1 = 1 /*level_one*/;
    eqtb[2243231L /*frozen_end_template*/] = eqtb[2243232L /*frozen_endv*/];
    eqtb[2243231L /*frozen_end_template*/].hh.u.B0 = 117 /*end_template*/;
    primitive(S(pagegoal), 82 /*set_page_dimen*/, 0);
    primitive(S(pagetotal), 82 /*set_page_dimen*/, 1);
    primitive(S(pagestretch), 82 /*set_page_dimen*/, 2);
    primitive(S(pagefilstretch), 82 /*set_page_dimen*/, 3);
    primitive(S(pagefillstretch), 82 /*set_page_dimen*/, 4);
    primitive(S(pagefilllstretch), 82 /*set_page_dimen*/, 5);
    primitive(S(pageshrink), 82 /*set_page_dimen*/, 6);
    primitive(S(pagedepth), 82 /*set_page_dimen*/, 7);
    primitive(S(end), 14 /*stop*/, 0);
    primitive(S(dump), 14 /*stop*/, 1);
    primitive(S(hskip), 26 /*hskip*/, 4 /*skip_code*/);
    primitive(S(hfil), 26 /*hskip*/, 0 /*fil_code*/);
    primitive(S(hfill), 26 /*hskip*/, 1 /*fill_code*/);
    primitive(S(hss), 26 /*hskip*/, 2 /*ss_code*/);
    primitive(S(hfilneg), 26 /*hskip*/, 3 /*fil_neg_code*/);
    primitive(S(vskip), 27 /*vskip*/, 4 /*skip_code*/);
    primitive(S(vfil), 27 /*vskip*/, 0 /*fil_code*/);
    primitive(S(vfill), 27 /*vskip*/, 1 /*fill_code*/);
    primitive(S(vss), 27 /*vskip*/, 2 /*ss_code*/);
    primitive(S(vfilneg), 27 /*vskip*/, 3 /*fil_neg_code*/);
    primitive(S(mskip), 28 /*mskip*/, 5 /*mskip_code*/);
    primitive(S(kern), 29 /*kern*/, 1 /*explicit*/);
    primitive(S(mkern), 30 /*mkern*/, 99 /*mu_glue*/);
    primitive(S(moveleft), 21 /*hmove*/, 1);
    primitive(S(moveright), 21 /*hmove*/, 0);
    primitive(S(raise), 22 /*vmove*/, 1);
    primitive(S(lower), 22 /*vmove*/, 0);
    primitive(S(box), 20 /*make_box*/, 0 /*box_code*/);
    primitive(S(copy), 20 /*make_box*/, 1 /*copy_code*/);
    primitive(S(lastbox), 20 /*make_box*/, 2 /*last_box_code*/);
    primitive(S(vsplit), 20 /*make_box*/, 3 /*vsplit_code*/);
    primitive(S(vtop), 20 /*make_box*/, 4 /*vtop_code*/);
    primitive(S(vbox), 20 /*make_box*/, 5 /*vtop_code 1*/);
    primitive(S(hbox), 20 /*make_box*/, 108 /*vtop_code 104*/);
    primitive(S(shipout), 31 /*leader_ship*/, 99 /*a_leaders -1*/);
    primitive(S(leaders), 31 /*leader_ship*/, 100 /*a_leaders*/);
    primitive(S(cleaders), 31 /*leader_ship*/, 101 /*c_leaders*/);
    primitive(S(xleaders), 31 /*leader_ship*/, 102 /*x_leaders*/);
    primitive(S(indent), 43 /*start_par*/, 1);
    primitive(S(noindent), 43 /*start_par*/, 0);
    primitive(S(unpenalty), 25 /*remove_item*/, 12 /*penalty_node*/);
    primitive(S(unkern), 25 /*remove_item*/, 11 /*kern_node*/);
    primitive(S(unskip), 25 /*remove_item*/, 10 /*glue_node*/);
    primitive(S(unhbox), 23 /*un_hbox*/, 0 /*box_code*/);
    primitive(S(unhcopy), 23 /*un_hbox*/, 1 /*copy_code*/);
    primitive(S(unvbox), 24 /*un_vbox*/, 0 /*box_code*/);
    primitive(S(unvcopy), 24 /*un_vbox*/, 1 /*copy_code*/);
    primitive(45 /*"-" */, 47 /*discretionary*/, 1);
    primitive(S(discretionary), 47 /*discretionary*/, 0);
    primitive(S(eqno), 48 /*eq_no*/, 0);
    primitive(S(leqno), 48 /*eq_no*/, 1);
    primitive(S(mathord), 50 /*math_comp*/, 16 /*ord_noad*/);
    primitive(S(mathop), 50 /*math_comp*/, 17 /*op_noad*/);
    primitive(S(mathbin), 50 /*math_comp*/, 18 /*bin_noad*/);
    primitive(S(mathrel), 50 /*math_comp*/, 19 /*rel_noad*/);
    primitive(S(mathopen), 50 /*math_comp*/, 20 /*open_noad*/);
    primitive(S(mathclose), 50 /*math_comp*/, 21 /*close_noad*/);
    primitive(S(mathpunct), 50 /*math_comp*/, 22 /*punct_noad*/);
    primitive(S(mathinner), 50 /*math_comp*/, 23 /*inner_noad*/);
    primitive(S(underline), 50 /*math_comp*/, 26 /*under_noad*/);
    primitive(S(overline), 50 /*math_comp*/, 27 /*over_noad*/);
    primitive(S(displaylimits), 51 /*limit_switch*/, 0 /*normal*/);
    primitive(S(limits), 51 /*limit_switch*/, 1 /*limits*/);
    primitive(S(nolimits), 51 /*limit_switch*/, 2 /*no_limits*/);
    primitive(S(displaystyle), 53 /*math_style*/, 0 /*display_style*/);
    primitive(S(textstyle), 53 /*math_style*/, 2 /*text_style*/);
    primitive(S(scriptstyle), 53 /*math_style*/, 4 /*script_style*/);
    primitive(S(scriptscriptstyle), 53 /*math_style*/, 6 /*script_script_style*/);
    primitive(S(above), 52 /*above*/, 0 /*above_code*/);
    primitive(S(over), 52 /*above*/, 1 /*over_code*/);
    primitive(S(atop), 52 /*above*/, 2 /*atop_code*/);
    primitive(S(abovewithdelims), 52 /*above*/, 3 /*delimited_code 0*/);
    primitive(S(overwithdelims), 52 /*above*/, 4 /*delimited_code 1*/);
    primitive(S(atopwithdelims), 52 /*above*/, 5 /*delimited_code 2*/);
    primitive(S(left), 49 /*left_right*/, 30 /*left_noad*/);
    primitive(S(right), 49 /*left_right*/, 31 /*right_noad*/);
    hash[2243229L /*frozen_right*/].v.RH = S(right);
    eqtb[2243229L /*frozen_right*/] = eqtb[cur_val];
    primitive(S(long), 95 /*prefix*/, 1);
    primitive(S(outer), 95 /*prefix*/, 2);
    primitive(S(global), 95 /*prefix*/, 4);
    primitive(S(def), 99 /*def*/, 0);
    primitive(S(gdef), 99 /*def*/, 1);
    primitive(S(edef), 99 /*def*/, 2);
    primitive(S(xdef), 99 /*def*/, 3);
    primitive(S(let), 96 /*let*/, 0 /*normal*/);
    primitive(S(futurelet), 96 /*let*/, 1 /*normal 1*/);
    primitive(S(chardef), 97 /*shorthand_def*/, 0 /*char_def_code*/);
    primitive(S(mathchardef), 97 /*shorthand_def*/, 1 /*math_char_def_code*/);
    primitive(S(XeTeXmathcharnumdef), 97 /*shorthand_def*/, 8 /*XeTeX_math_char_num_def_code*/);
    primitive(S(Umathcharnumdef), 97 /*shorthand_def*/, 8 /*XeTeX_math_char_num_def_code*/);
    primitive(S(XeTeXmathchardef), 97 /*shorthand_def*/, 9 /*XeTeX_math_char_def_code*/);
    primitive(S(Umathchardef), 97 /*shorthand_def*/, 9 /*XeTeX_math_char_def_code*/);
    primitive(S(countdef), 97 /*shorthand_def*/, 2 /*count_def_code*/);
    primitive(S(dimendef), 97 /*shorthand_def*/, 3 /*dimen_def_code*/);
    primitive(S(skipdef), 97 /*shorthand_def*/, 4 /*skip_def_code*/);
    primitive(S(muskipdef), 97 /*shorthand_def*/, 5 /*mu_skip_def_code*/);
    primitive(S(toksdef), 97 /*shorthand_def*/, 6 /*toks_def_code*/);

    if (mltex_p)
        primitive(S(charsubdef), 97 /*shorthand_def*/, 7 /*char_sub_def_code*/);

    primitive(S(catcode), 86 /*def_code*/, CAT_CODE_BASE);
    primitive(S(mathcode), 86 /*def_code*/, MATH_CODE_BASE);
    primitive(S(XeTeXmathcodenum), 87 /*XeTeX_def_code*/, MATH_CODE_BASE);
    primitive(S(Umathcodenum), 87 /*XeTeX_def_code*/, MATH_CODE_BASE);
    primitive(S(XeTeXmathcode), 87 /*XeTeX_def_code*/, 6710517L /*math_code_base 1*/);
    primitive(S(Umathcode), 87 /*XeTeX_def_code*/, 6710517L /*math_code_base 1*/);
    primitive(S(lccode), 86 /*def_code*/, LC_CODE_BASE);
    primitive(S(uccode), 86 /*def_code*/, UC_CODE_BASE);
    primitive(S(sfcode), 86 /*def_code*/, SF_CODE_BASE);
    primitive(S(XeTeXcharclass), 87 /*XeTeX_def_code*/, SF_CODE_BASE);
    primitive(S(delcode), 86 /*def_code*/, DEL_CODE_BASE);
    primitive(S(XeTeXdelcodenum), 87 /*XeTeX_def_code*/, DEL_CODE_BASE);
    primitive(S(Udelcodenum), 87 /*XeTeX_def_code*/, DEL_CODE_BASE);
    primitive(S(XeTeXdelcode), 87 /*XeTeX_def_code*/, 8939081L /*del_code_base 1*/);
    primitive(S(Udelcode), 87 /*XeTeX_def_code*/, 8939081L /*del_code_base 1*/);
    primitive(S(textfont), 88 /*def_family*/, MATH_FONT_BASE);
    primitive(S(scriptfont), 88 /*def_family*/, 2253556L /*math_font_base 256*/);
    primitive(S(scriptscriptfont), 88 /*def_family*/, 2253812L /*math_font_base 512*/);
    primitive(S(hyphenation), 101 /*hyph_data*/, 0);
    primitive(S(patterns), 101 /*hyph_data*/, 1);
    primitive(S(hyphenchar), ASSIGN_FONT_INT, 0);
    primitive(S(skewchar), ASSIGN_FONT_INT, 1);
    primitive(S(lpcode), ASSIGN_FONT_INT, 2);
    primitive(S(rpcode), ASSIGN_FONT_INT, 3);
    primitive(S(batchmode), 102 /*set_interaction*/, 0 /*batch_mode*/);
    primitive(S(nonstopmode), 102 /*set_interaction*/, 1 /*nonstop_mode*/);
    primitive(S(scrollmode), 102 /*set_interaction*/, 2 /*scroll_mode*/);
    primitive(S(errorstopmode), 102 /*set_interaction*/, 3 /*error_stop_mode*/);
    primitive(S(openin), 60 /*in_stream*/, 1);
    primitive(S(closein), 60 /*in_stream*/, 0);
    primitive(S(message), 58 /*message*/, 0);
    primitive(S(errmessage), 58 /*message*/, 1);
    primitive(S(lowercase), 57 /*case_shift*/, LC_CODE_BASE);
    primitive(S(uppercase), 57 /*case_shift*/, UC_CODE_BASE);
    primitive(S(show), 19 /*xray*/, 0 /*show_code*/);
    primitive(S(showbox), 19 /*xray*/, 1 /*show_box_code*/);
    primitive(S(showthe), 19 /*xray*/, 2 /*show_the_code*/);
    primitive(S(showlists), 19 /*xray*/, 3 /*show_lists*/);
    primitive(S(openout), 59 /*extension*/, 0 /*open_node*/);
    primitive(S(write), 59 /*extension*/, 1 /*write_node*/);
    write_loc = cur_val;
    primitive(S(closeout), 59 /*extension*/, 2 /*close_node*/);
    primitive(S(special), 59 /*extension*/, 3 /*special_node*/);
    hash[2243236L /*frozen_special*/].v.RH = S(special);
    eqtb[2243236L /*frozen_special*/] = eqtb[cur_val];
    primitive(S(immediate), 59 /*extension*/, 4 /*immediate_code*/);
    primitive(S(setlanguage), 59 /*extension*/, 5 /*set_language_code*/);
    primitive(S(synctex), ASSIGN_INT, 8938823L /*int_base 83*/);

    no_new_control_sequence = true;
}


static boolean
get_strings_started(void)
{
    pool_ptr = 0;
    str_ptr = 0;
    str_start[0] = 0;
    str_ptr = 65536L /*too_big_char*/;
    str_start[(str_ptr) - 65536L] = pool_ptr;

    if (load_pool_strings(pool_size - string_vacancies) == 0)
	_tt_abort ("must increase pool_size");

    return true;
}/*:1001*/


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

    interaction_option = 4;
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
    /* FKA main_body() */
    memory_word *eqtb = zeqtb;

    /* Before anything else ... setjmp handling of super-fatal errors */

    if (setjmp (jump_buffer)) {
	history = HISTORY_FATAL_ERROR;
	return history;
    }

    /* These various parameters were configurable in web2c TeX. We don't
     * bother to allow that. */

    mem_bot = 0;
    main_memory = 5000000L;
    extra_mem_top = 0;
    extra_mem_bot = 0;
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

    if (in_initex_mode) {
        extra_mem_top = 0;
        extra_mem_bot = 0;
    }

    mem_top = mem_bot + main_memory - 1;
    mem_min = mem_bot;
    mem_max = mem_top;

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

    if (in_initex_mode) {
        yzmem = xmalloc_array(memory_word, mem_top - mem_bot + 1);
        zmem = yzmem - mem_bot;
        eqtb_top = EQTB_SIZE + hash_extra;
        if (hash_extra == 0)
            hash_top = UNDEFINED_CONTROL_SEQUENCE;
        else
            hash_top = eqtb_top;
        yhash = xmalloc_array(two_halves, 1 + hash_top - hash_offset);
        hash = yhash - hash_offset;
        hash[HASH_BASE].v.LH = 0;
        hash[HASH_BASE].v.RH = 0;
        {
            register integer for_end;
            hash_used = (HASH_BASE + 1);
            for_end = hash_top;
            if (hash_used <= for_end)
                do
                    hash[hash_used] = hash[HASH_BASE];
                while (hash_used++ < for_end);
        }
        zeqtb = xmalloc_array(memory_word, eqtb_top);
        eqtb = zeqtb;
        str_start = xmalloc_array(pool_pointer, max_strings);
        str_pool = xmalloc_array(packed_UTF16_code, pool_size);
        font_info = xmalloc_array(fmemory_word, font_mem_size);
    }

    /* Sanity-check various invariants. */

    history = HISTORY_FATAL_ERROR;
    bad = 0;

    if ((half_error_line < 30) || (half_error_line > error_line - 15))
        bad = 1;
    if (max_print_line < 60)
        bad = 2;
    if (dvi_buf_size % 8 != 0)
        bad = 3;
    if (mem_bot + 1100 > mem_top)
        bad = 4;
    if (HASH_PRIME > HASH_SIZE)
        bad = 5;
    if (max_in_open >= 128)
        bad = 6;
    if (mem_top < 267)
        bad = 7;
    if ((mem_min != mem_bot) || (mem_max != mem_top))
        bad = 10;
    if ((mem_min > mem_bot) || (mem_max < mem_top))
        bad = 10;
    if ((-268435455L > 0) || (1073741823L < 1073741823L))
        bad = 12;
    if ((mem_bot - sup_main_memory < -268435455L) || (mem_top + sup_main_memory >= 1073741823L))
        bad = 14;
    if ((MAX_FONT_MAX < -268435455L) || (MAX_FONT_MAX > 1073741823L))
        bad = 15;
    if (font_max > (FONT_BASE + 9000))
        bad = 16;
    if ((save_size > 1073741823L) || (max_strings > 1073741823L))
        bad = 17;
    if (buf_size > 1073741823L)
        bad = 18;
    if ((CS_TOKEN_FLAG + 10053470) + hash_extra > 1073741823L)
        bad = 21;
    if ((hash_offset < 0) || (hash_offset > HASH_BASE))
        bad = 42;
    if (format_default_length > INTEGER_MAX)
        bad = 31;
    if (2 * 1073741823L < mem_top - mem_min)
        bad = 41;

    if (bad > 0)
	_tt_abort ("failed internal consistency check #%d", bad);

    /* OK, ready to keep on initializing. */

    initialize_more_variables();

    if (in_initex_mode) {
        if (!get_strings_started())
            return history;
        initialize_primitives();
        init_str_ptr = str_ptr;
        init_pool_ptr = pool_ptr;
        get_date_and_time(&(eqtb[(INT_BASE + 20)].cint),
			  &(eqtb[(INT_BASE + 21)].cint),
			  &(eqtb[(INT_BASE + 22)].cint),
			  &(eqtb[(INT_BASE + 23)].cint));
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
    if_stack[0] = -268435455L;
    param_ptr = 0;
    max_param_stack = 0;

    first = buf_size;
    do {
	buffer[first] = 0;
	first--;
    } while (first != 0);

    scanner_status = 0 /*normal*/;
    warning_index = -268435455L;
    first = 1;
    cur_input.state = 33 /*new_line*/;
    cur_input.start = 1;
    cur_input.index = 0;
    line = 0;
    cur_input.name = 0;
    force_eof = false;
    align_state = 1000000L;

    if (!init_terminal(input_file_name))
	return history;

    cur_input.limit = last;
    first = last + 1;

    if ((etex_p || buffer[cur_input.loc] == 42 /*"*" */) && format_ident == S(__INITEX_)) {
	no_new_control_sequence = false;
	primitive(S(XeTeXpicfile), 59 /*extension*/, 41 /*pic_file_code*/);
	primitive(S(XeTeXpdffile), 59 /*extension*/, 42 /*pdf_file_code*/);
	primitive(S(XeTeXglyph), 59 /*extension*/, 43 /*glyph_code*/);
	primitive(S(XeTeXlinebreaklocale), 59 /*extension*/, 46 /*XeTeX_linebreak_locale_extension_code*/);
	primitive(S(XeTeXinterchartoks), ASSIGN_TOKS, 2252782L /*XeTeX_inter_char_loc*/);
	primitive(S(pdfsavepos), 59 /*extension*/, 6 /*pdftex_first_extension_code 0*/);
	primitive(S(lastnodetype), 71 /*last_item*/, 3 /*last_node_type_code*/);
	primitive(S(eTeXversion), 71 /*last_item*/, 6 /*eTeX_version_code*/);
	primitive(S(eTeXrevision), 110 /*convert*/, 5 /*eTeX_revision_code*/);
	primitive(S(XeTeXversion), 71 /*last_item*/, 14 /*XeTeX_version_code*/);
	primitive(S(XeTeXrevision), 110 /*convert*/, 6 /*XeTeX_revision_code*/);
	primitive(S(XeTeXcountglyphs), 71 /*last_item*/, 15 /*XeTeX_count_glyphs_code*/);
	primitive(S(XeTeXcountvariations), 71 /*last_item*/, 16 /*XeTeX_count_variations_code*/);
	primitive(S(XeTeXvariation), 71 /*last_item*/, 17 /*XeTeX_variation_code*/);
	primitive(S(XeTeXfindvariationbyname), 71 /*last_item*/, 18 /*XeTeX_find_variation_by_name_code*/);
	primitive(S(XeTeXvariationmin), 71 /*last_item*/, 19 /*XeTeX_variation_min_code*/);
	primitive(S(XeTeXvariationmax), 71 /*last_item*/, 20 /*XeTeX_variation_max_code*/);
	primitive(S(XeTeXvariationdefault), 71 /*last_item*/, 21 /*XeTeX_variation_default_code*/);
	primitive(S(XeTeXcountfeatures), 71 /*last_item*/, 22 /*XeTeX_count_features_code*/);
	primitive(S(XeTeXfeaturecode), 71 /*last_item*/, 23 /*XeTeX_feature_code_code*/);
	primitive(S(XeTeXfindfeaturebyname), 71 /*last_item*/, 24 /*XeTeX_find_feature_by_name_code*/);
	primitive(S(XeTeXisexclusivefeature), 71 /*last_item*/, 25 /*XeTeX_is_exclusive_feature_code*/);
	primitive(S(XeTeXcountselectors), 71 /*last_item*/, 26 /*XeTeX_count_selectors_code*/);
	primitive(S(XeTeXselectorcode), 71 /*last_item*/, 27 /*XeTeX_selector_code_code*/);
	primitive(S(XeTeXfindselectorbyname), 71 /*last_item*/, 28 /*XeTeX_find_selector_by_name_code*/);
	primitive(S(XeTeXisdefaultselector), 71 /*last_item*/, 29 /*XeTeX_is_default_selector_code*/);
	primitive(S(XeTeXvariationname), 110 /*convert*/, 7 /*XeTeX_variation_name_code*/);
	primitive(S(XeTeXfeaturename), 110 /*convert*/, XeTeX_feature_name);
	primitive(S(XeTeXselectorname), 110 /*convert*/, XeTeX_selector_name);
	primitive(S(XeTeXOTcountscripts), 71 /*last_item*/, 30 /*XeTeX_OT_count_scripts_code*/);
	primitive(S(XeTeXOTcountlanguages), 71 /*last_item*/, 31 /*XeTeX_OT_count_languages_code*/);
	primitive(S(XeTeXOTcountfeatures), 71 /*last_item*/, 32 /*XeTeX_OT_count_features_code*/);
	primitive(S(XeTeXOTscripttag), 71 /*last_item*/, 33 /*XeTeX_OT_script_code*/);
	primitive(S(XeTeXOTlanguagetag), 71 /*last_item*/, 34 /*XeTeX_OT_language_code*/);
	primitive(S(XeTeXOTfeaturetag), 71 /*last_item*/, 35 /*XeTeX_OT_feature_code*/);
	primitive(S(XeTeXcharglyph), 71 /*last_item*/, 36 /*XeTeX_map_char_to_glyph_code*/);
	primitive(S(XeTeXglyphindex), 71 /*last_item*/, 37 /*XeTeX_glyph_index_code*/);
	primitive(S(XeTeXglyphbounds), 71 /*last_item*/, 47 /*XeTeX_glyph_bounds_code*/);
	primitive(S(XeTeXglyphname), 110 /*convert*/, 10 /*XeTeX_glyph_name_code*/);
	primitive(S(XeTeXfonttype), 71 /*last_item*/, 38 /*XeTeX_font_type_code*/);
	primitive(S(XeTeXfirstfontchar), 71 /*last_item*/, 39 /*XeTeX_first_char_code*/);
	primitive(S(XeTeXlastfontchar), 71 /*last_item*/, 40 /*XeTeX_last_char_code*/);
	primitive(S(pdflastxpos), 71 /*last_item*/, 41 /*pdf_last_x_pos_code*/);
	primitive(S(pdflastypos), 71 /*last_item*/, 42 /*pdf_last_y_pos_code*/);
	primitive(S(strcmp), 110 /*convert*/, 43 /*pdf_strcmp_code*/);
	primitive(S(mdfivesum), 110 /*convert*/, 44 /*pdf_mdfive_sum_code*/);
	primitive(S(shellescape), 71 /*last_item*/, 45 /*pdf_shell_escape_code*/);
	primitive(S(XeTeXpdfpagecount), 71 /*last_item*/, 46 /*XeTeX_pdf_page_count_code*/);
	primitive(S(everyeof), ASSIGN_TOKS, 2252781L /*every_eof_loc*/);
	primitive(S(tracingassigns), ASSIGN_INT, 8938798L /*int_base 58*/);
	primitive(S(tracinggroups), ASSIGN_INT, 8938799L /*int_base 59*/);
	primitive(S(tracingifs), ASSIGN_INT, 8938800L /*int_base 60*/);
	primitive(S(tracingscantokens), ASSIGN_INT, 8938801L /*int_base 61*/);
	primitive(S(tracingnesting), ASSIGN_INT, 8938802L /*int_base 62*/);
	primitive(S(predisplaydirection), ASSIGN_INT, 8938803L /*int_base 63*/);
	primitive(S(lastlinefit), ASSIGN_INT, 8938804L /*int_base 64*/);
	primitive(S(savingvdiscards), ASSIGN_INT, 8938805L /*int_base 65*/);
	primitive(S(savinghyphcodes), ASSIGN_INT, 8938806L /*int_base 66*/);
	primitive(S(currentgrouplevel), 71 /*last_item*/, 7 /*current_group_level_code*/);
	primitive(S(currentgrouptype), 71 /*last_item*/, 8 /*current_group_type_code*/);
	primitive(S(currentiflevel), 71 /*last_item*/, 9 /*current_if_level_code*/);
	primitive(S(currentiftype), 71 /*last_item*/, 10 /*current_if_type_code*/);
	primitive(S(currentifbranch), 71 /*last_item*/, 11 /*current_if_branch_code*/);
	primitive(S(fontcharwd), 71 /*last_item*/, 48 /*font_char_wd_code*/);
	primitive(S(fontcharht), 71 /*last_item*/, 49 /*font_char_ht_code*/);
	primitive(S(fontchardp), 71 /*last_item*/, 50 /*font_char_dp_code*/);
	primitive(S(fontcharic), 71 /*last_item*/, 51 /*font_char_ic_code*/);
	primitive(S(parshapelength), 71 /*last_item*/, 52 /*par_shape_length_code*/);
	primitive(S(parshapeindent), 71 /*last_item*/, 53 /*par_shape_indent_code*/);
	primitive(S(parshapedimen), 71 /*last_item*/, 54 /*par_shape_dimen_code*/);
	primitive(S(showgroups), 19 /*xray*/, 4 /*show_groups*/);
	primitive(S(showtokens), 19 /*xray*/, 5 /*show_tokens*/);
	primitive(S(unexpanded), 111 /*the*/, 1);
	primitive(S(detokenize), 111 /*the*/, 5 /*show_tokens*/);
	primitive(S(showifs), 19 /*xray*/, 6 /*show_ifs*/);
	primitive(S(interactionmode), 83 /*set_page_int*/, 2);
	primitive(S(middle), 49 /*left_right*/, 1);
	primitive(S(suppressfontnotfounderror), ASSIGN_INT, 8938807L /*int_base 67*/);
	primitive(S(TeXXeTstate), ASSIGN_INT, 8938811L /*eTeX_state_base 0*/);
	primitive(S(XeTeXupwardsmode), ASSIGN_INT, 8938813L /*eTeX_state_base 2*/);
	primitive(S(XeTeXuseglyphmetrics), ASSIGN_INT, 8938814L /*eTeX_state_base 3*/);
	primitive(S(XeTeXinterchartokenstate), ASSIGN_INT, 8938815L /*eTeX_state_base 4*/);
	primitive(S(XeTeXdashbreakstate), ASSIGN_INT, 8938812L /*eTeX_state_base 1*/);
	primitive(S(XeTeXinputnormalization), ASSIGN_INT, 8938816L /*eTeX_state_base 5*/);
	primitive(S(XeTeXtracingfonts), ASSIGN_INT, 8938819L /*eTeX_state_base 8*/);
	primitive(S(XeTeXinterwordspaceshaping), ASSIGN_INT, 8938820L /*eTeX_state_base 9*/);
	primitive(S(XeTeXgenerateactualtext), ASSIGN_INT, 8938821L /*eTeX_state_base 10*/);
	primitive(S(XeTeXhyphenatablelength), ASSIGN_INT, 8938822L /*eTeX_state_base 11*/);
	primitive(S(XeTeXinputencoding), 59 /*extension*/, 44 /*XeTeX_input_encoding_extension_code*/);
	primitive(S(XeTeXdefaultencoding), 59 /*extension*/, 45 /*XeTeX_default_encoding_extension_code*/);
	primitive(S(beginL), 33 /*valign*/, 6 /*begin_L_code*/);
	primitive(S(endL), 33 /*valign*/, 7 /*end_L_code*/);
	primitive(S(beginR), 33 /*valign*/, 10 /*begin_R_code*/);
	primitive(S(endR), 33 /*valign*/, 11 /*end_R_code*/);
	primitive(S(scantokens), 106 /*input*/, 2);
	primitive(S(readline), 98 /*read_to_cs*/, 1);
	primitive(S(unless), 104 /*expand_after*/, 1);
	primitive(S(ifdefined), 107 /*if_test*/, 17 /*if_def_code*/);
	primitive(S(ifcsname), 107 /*if_test*/, 18 /*if_cs_code*/);
	primitive(S(iffontchar), 107 /*if_test*/, 19 /*if_font_char_code*/);
	primitive(S(ifincsname), 107 /*if_test*/, 20 /*if_in_csname_code*/);
	primitive(S(protected), 95 /*prefix*/, 8);
	primitive(S(numexpr), 71 /*last_item*/, 59 /*eTeX_expr -0 0*/);
	primitive(S(dimexpr), 71 /*last_item*/, 60 /*eTeX_expr -0 1*/);
	primitive(S(glueexpr), 71 /*last_item*/, 61 /*eTeX_expr -0 2*/);
	primitive(S(muexpr), 71 /*last_item*/, 62 /*eTeX_expr -0 3*/);
	primitive(S(gluestretchorder), 71 /*last_item*/, 12 /*glue_stretch_order_code*/);
	primitive(S(glueshrinkorder), 71 /*last_item*/, 13 /*glue_shrink_order_code*/);
	primitive(S(gluestretch), 71 /*last_item*/, 55 /*glue_stretch_code*/);
	primitive(S(glueshrink), 71 /*last_item*/, 56 /*glue_shrink_code*/);
	primitive(S(mutoglue), 71 /*last_item*/, 57 /*mu_to_glue_code*/);
	primitive(S(gluetomu), 71 /*last_item*/, 58 /*glue_to_mu_code*/);
	primitive(S(marks), 18 /*mark*/, 5);
	primitive(S(topmarks), 112 /*top_bot_mark*/, 5 /*top_mark_code 5*/);
	primitive(S(firstmarks), 112 /*top_bot_mark*/, 6 /*first_mark_code 5*/);
	primitive(S(botmarks), 112 /*top_bot_mark*/, 7 /*bot_mark_code 5*/);
	primitive(S(splitfirstmarks), 112 /*top_bot_mark*/, 8 /*split_first_mark_code 5*/);
	primitive(S(splitbotmarks), 112 /*top_bot_mark*/, 9 /*split_bot_mark_code 5*/);
	primitive(S(pagediscards), 24 /*un_vbox*/, 2 /*last_box_code*/);
	primitive(S(splitdiscards), 24 /*un_vbox*/, 3 /*vsplit_code*/);
	primitive(S(interlinepenalties), 85 /*set_shape*/, 2253039L /*inter_line_penalties_loc*/);
	primitive(S(clubpenalties), 85 /*set_shape*/, 2253040L /*club_penalties_loc*/);
	primitive(S(widowpenalties), 85 /*set_shape*/, 2253041L /*widow_penalties_loc*/);
	primitive(S(displaywidowpenalties), 85 /*set_shape*/, 2253042L /*display_widow_penalties_loc*/);

	if (buffer[cur_input.loc] == 42 /*"*" */)
	    cur_input.loc++;

	eTeX_mode = 1;
	max_reg_num = 32767;
	max_reg_help_line = S(A_register_number_must_be_be_Z1);
    }

    if (!no_new_control_sequence)
	no_new_control_sequence = true;
    else if (format_ident == 0 || buffer[cur_input.loc] == 38 /*"&" */ || dump_line) {
	if (format_ident != 0)
	    initialize_more_variables();

	if (!load_fmt_file())
	    return history;

	eqtb = zeqtb;

	while (cur_input.loc < cur_input.limit && buffer[cur_input.loc] == 32 /*" " */)
	    cur_input.loc++;
    }

    if (eTeX_mode == 1) {
	char *msg = "entering extended mode\n";
	ttstub_output_write (rust_stdout, msg, strlen (msg));
    }

    if (eqtb[8938788L /*int_base 48*/].cint < 0 || eqtb[8938788L /*int_base 48*/].cint > 255)
	cur_input.limit--;
    else
	buffer[cur_input.limit] = eqtb[8938788L /*int_base 48*/].cint;

    if (mltex_enabled_p) {
	char *msg = "MLTeX v2.2 enabled\n";
	ttstub_output_write (rust_stdout, msg, strlen (msg));
    }

    get_date_and_time(&(eqtb[8938760L /*int_base 20*/].cint),
		      &(eqtb[8938761L /*int_base 21*/].cint),
		      &(eqtb[8938762L /*int_base 22*/].cint),
		      &(eqtb[8938763L /*int_base 23*/].cint));

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
	font_area[FONT_BASE] = 65622L /*""*/;
	hyphen_char[FONT_BASE] = 45 /*"-" */;
	skew_char[FONT_BASE] = -1;
	bchar_label[FONT_BASE] = 0 /*non_address*/;
	font_bchar[FONT_BASE] = 65536L /*too_big_char*/;
	font_false_bchar[FONT_BASE] = 65536L /*too_big_char*/;
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
	font_glue[FONT_BASE] = -268435455L;
	font_params[FONT_BASE] = 7;
	font_mapping[FONT_BASE] = 0;
	param_base[FONT_BASE] = -1;

	for (font_k = 0; font_k <= 6; font_k++)
	    font_info[font_k].cint = 0;
    }

    font_used = xmalloc_array(boolean, font_max);
    for (font_k = 0; font_k <= font_max; font_k++)
	font_used[font_k] = false;

    magic_offset = str_start[(66282L /*math_spacing*/) - 65536L] - 9 * 16 /*ord_noad*//*:794*/;

    if (interaction == 0 /*batch_mode*/)
	selector = SELECTOR_NO_PRINT;
    else
	selector = SELECTOR_TERM_ONLY; /*:79*/

    /* OK, we are finally ready to go!
     *
     * Below is the key line that looks at the "first line" that we've
     * synthesized. If it doesn't begin with a control character, we pretend
     * that the user has essentially written "\input ..." */

    if (cur_input.loc < cur_input.limit
	&& eqtb[CAT_CODE_BASE + buffer[cur_input.loc]].hh.v.RH != 0 /*escape*/)
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
