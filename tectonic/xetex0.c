#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>
#include <tectonic/synctex.h>
#include <tectonic/stubs.h>


static void
write_to_dvi(integer a, integer b)
{
    integer n = b - a + 1;

    if (ttstub_output_write (dvi_file, (char *) &dvi_buf[a], n) != n)
	_tt_abort ("failed to write data to XDV file");
}

static void
int_error(integer n)
{
    print(S(___Z2/*" ("*/));
    print_int(n);
    print_char(41 /*")" */ );
    error();
}

int32_t badness(scaled t, scaled s)
{
    register int32_t Result;
    integer r;
    if (t == 0)
        Result = 0;
    else if (s <= 0)
        Result = INF_BAD;
    else {

        if (t <= 7230584L)
            r = (t * 297) / s;
        else if (s >= 1663497L)
            r = t / (s / 297);
        else
            r = t;
        if (r > 1290)
            Result = INF_BAD;
        else
            Result = (r * r * r + 131072L) / 262144L;
    }
    return Result;
}

/*:112*/
/*118:*/
void
show_token_list(integer p, integer q, integer l)
{
    memory_word *mem = zmem;
    integer m, c;
    integer match_chr;
    UTF16_code n;

    match_chr = 35 /*"#" */ ;
    n = 48 /*"0" */ ;
    tally = 0;

    while (p != MIN_HALFWORD && tally < l) {
        /*332:*/
        if (p == q) {
            first_count = tally;
            trick_count = tally + 1 + error_line - half_error_line;
            if (trick_count < error_line)
                trick_count = error_line;
        }

        if (p < hi_mem_min || p > mem_end) {
            print_esc(S(CLOBBERED_));
            return;
        }

        if (mem[p].hh.v.LH >= CS_TOKEN_FLAG) {
            print_cs(mem[p].hh.v.LH - CS_TOKEN_FLAG);
        } else {
            m = mem[p].hh.v.LH / MAX_CHAR_VAL;
            c = mem[p].hh.v.LH % MAX_CHAR_VAL;

            if (mem[p].hh.v.LH < 0) {
                print_esc(S(BAD_));
            } else {
                /*306:*/
                switch (m) {
                case LEFT_BRACE:
                case RIGHT_BRACE:
                case MATH_SHIFT:
                case TAB_MARK:
                case SUP_MARK:
                case SUB_MARK:
                case SPACER:
                case LETTER:
                case OTHER_CHAR:
                    print_char(c);
                    break;
                case MAC_PARAM:
                    print_char(c);
                    print_char(c);
                    break;
                case OUT_PARAM:
                    print_char(match_chr);
                    if (c <= 9) {
                        print_char(c + 48);
                    } else {
                        print_char(33 /*"!" */ );
                        return;
                    }
                    break;
                case MATCH:
                    match_chr = c;
                    print_char(c);
                    n++;
                    print_char(n);
                    if (n > 57 /*"9" */ )
                        return;
                    break;
                case END_MATCH:
                    if (c == 0)
                        print(S(___Z7/*"->"*/));
                    break;
                default:
                    print_esc(S(BAD_));
                    break;
                }
            }
        }

        p = mem[p].hh.v.RH;
    }

    if (p != MIN_HALFWORD)
        print_esc(S(ETC_));
}


void
runaway(void)
{
    memory_word *mem = zmem;
    int32_t p;

    if (scanner_status > SKIPPING) {
        switch (scanner_status) {
        case DEFINING:
            print_nl(S(Runaway_definition));
            p = def_ref;
            break;
        case MATCHING:
            print_nl(S(Runaway_argument));
            p = mem_top - 3;
            break;
        case ALIGNING:
            print_nl(S(Runaway_preamble));
            p = mem_top - 4;
            break;
        case ABSORBING:
            print_nl(S(Runaway_text));
            p = def_ref;
            break;
        }

        print_char(63 /*"?" */ );
        print_ln();
        show_token_list(mem[p].hh.v.RH, MIN_HALFWORD, error_line - 10);
    }
}


int32_t get_avail(void)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = avail;
    if (p != MIN_HALFWORD)
        avail = mem[avail].hh.v.RH;
    else if (mem_end < mem_top) {
        mem_end++;
        p = mem_end;
    } else {

        hi_mem_min--;
        p = hi_mem_min;
        if (hi_mem_min <= lo_mem_max) {
            runaway();
            overflow(S(main_memory_size), mem_top + 1);
        }
    }
    mem[p].hh.v.RH = MIN_HALFWORD;
    Result = p;
    return Result;
}

void flush_list(int32_t p)
{
    memory_word *mem = zmem; int32_t q, r;
    if (p != MIN_HALFWORD) {
        r = p;
        do {
            q = r;
            r = mem[r].hh.v.RH;
        } while (!(r == MIN_HALFWORD));
        mem[q].hh.v.RH = avail;
        avail = p;
    }
}

int32_t get_node(integer s)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    int32_t q;
    integer r;
    integer t;
 lab20:                        /*restart */ p = rover;
    do {
        /*131: */ q = p + mem[p].hh.v.LH;
        while ((mem[q].hh.v.RH == MAX_HALFWORD)) {

            t = mem[q + 1].hh.v.RH;
            if (q == rover)
                rover = t;
            mem[t + 1].hh.v.LH = mem[q + 1].hh.v.LH;
            mem[mem[q + 1].hh.v.LH + 1].hh.v.RH = t;
            q = q + mem[q].hh.v.LH;
        }
        r = q - s;
        if (r > p + 1) {        /*132: */
            mem[p].hh.v.LH = r - p;
            rover = p;
            goto lab40;
        }
        if (r == p) {

            if (mem[p + 1].hh.v.RH != p) {      /*133: */
                rover = mem[p + 1].hh.v.RH;
                t = mem[p + 1].hh.v.LH;
                mem[rover + 1].hh.v.LH = t;
                mem[t + 1].hh.v.RH = rover;
                goto lab40;
            }
        }
        mem[p].hh.v.LH = q - /*:131 */ p;
        p = mem[p + 1].hh.v.RH;
    } while (!(p == rover));
    if (s == 0x40000000) {
        Result = MAX_HALFWORD;
        return Result;
    }
    if (lo_mem_max + 2 < hi_mem_min) {

        if (lo_mem_max + 2 <= MAX_HALFWORD) {  /*130: */
            if (hi_mem_min - lo_mem_max >= 1998)
                t = lo_mem_max + 1000;
            else
                t = lo_mem_max + 1 + (hi_mem_min - lo_mem_max) / 2;
            p = mem[rover + 1].hh.v.LH;
            q = lo_mem_max;
            mem[p + 1].hh.v.RH = q;
            mem[rover + 1].hh.v.LH = q;
            if (t > MAX_HALFWORD)
                t = MAX_HALFWORD;
            mem[q + 1].hh.v.RH = rover;
            mem[q + 1].hh.v.LH = p;
            mem[q].hh.v.RH = MAX_HALFWORD;
            mem[q].hh.v.LH = t - lo_mem_max;
            lo_mem_max = t;
            mem[lo_mem_max].hh.v.RH = MIN_HALFWORD;
            mem[lo_mem_max].hh.v.LH = MIN_HALFWORD;
            rover = q;
            goto lab20;
        }
    }
    overflow(S(main_memory_size), mem_top + 1);

lab40: /*found */
    mem[r].hh.v.RH = MIN_HALFWORD;
    if (s >= MEDIUM_NODE_SIZE) {
        mem[r + s - 1].hh.v.LH = cur_input.synctex_tag;
        mem[r + s - 1].hh.v.RH = line;
    }
    Result = r;
    return Result;
}

void free_node(int32_t p, int32_t s)
{
    memory_word *mem = zmem; int32_t q;
    mem[p].hh.v.LH = s;
    mem[p].hh.v.RH = MAX_HALFWORD;
    q = mem[rover + 1].hh.v.LH;
    mem[p + 1].hh.v.LH = q;
    mem[p + 1].hh.v.RH = rover;
    mem[rover + 1].hh.v.LH = p;
    mem[q + 1].hh.v.RH = p;
}

int32_t new_null_box(void)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(BOX_NODE_SIZE);
    mem[p].hh.u.B0 = HLIST_NODE;
    mem[p].hh.u.B1 = 0;
    mem[p + 1].cint = 0;
    mem[p + 2].cint = 0;
    mem[p + 3].cint = 0;
    mem[p + 4].cint = 0;
    mem[p + 5].hh.v.RH = MIN_HALFWORD;
    mem[p + 5].hh.u.B0 = NORMAL;
    mem[p + 5].hh.u.B1 = NORMAL;
    mem[p + 6].gr = 0.0;
    Result = p;
    return Result;
}

int32_t new_rule(void)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(RULE_NODE_SIZE);
    mem[p].hh.u.B0 = RULE_NODE;
    mem[p].hh.u.B1 = 0;
    mem[p + 1].cint = NULL_FLAG;
    mem[p + 2].cint = NULL_FLAG;
    mem[p + 3].cint = NULL_FLAG;
    Result = p;
    return Result;
}

int32_t new_ligature(internal_font_number f, uint16_t c, int32_t q)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(SMALL_NODE_SIZE);
    mem[p].hh.u.B0 = LIGATURE_NODE;
    mem[p + 1].hh.u.B0 = f;
    mem[p + 1].hh.u.B1 = c;
    mem[p + 1].hh.v.RH = q;
    mem[p].hh.u.B1 = 0;
    Result = p;
    return Result;
}

int32_t new_lig_item(uint16_t c)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(SMALL_NODE_SIZE);
    mem[p].hh.u.B1 = c;
    mem[p + 1].hh.v.RH = MIN_HALFWORD;
    Result = p;
    return Result;
}

int32_t new_disc(void)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(SMALL_NODE_SIZE);
    mem[p].hh.u.B0 = DISC_NODE;
    mem[p].hh.u.B1 = 0;
    mem[p + 1].hh.v.LH = MIN_HALFWORD;
    mem[p + 1].hh.v.RH = MIN_HALFWORD;
    Result = p;
    return Result;
}

void copy_native_glyph_info(int32_t src, int32_t dest)
{
    memory_word *mem = zmem; integer glyph_count;
    if (mem[src + 5].ptr != NULL) {
        glyph_count = mem[src + 4].qqqq.u.B3;
        mem[dest + 5].ptr = xmalloc_array(char, glyph_count * NATIVE_GLYPH_INFO_SIZE);
        memcpy(mem[dest + 5].ptr, mem[src + 5].ptr, glyph_count * NATIVE_GLYPH_INFO_SIZE);
        mem[dest + 4].qqqq.u.B3 = glyph_count;
    }
}

int32_t new_math(scaled w, small_number s)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(MEDIUM_NODE_SIZE);
    mem[p].hh.u.B0 = MATH_NODE;
    mem[p].hh.u.B1 = s;
    mem[p + 1].cint = w;
    Result = p;
    return Result;
}

int32_t new_spec(int32_t p)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t q;
    q = get_node(GLUE_SPEC_SIZE);
    mem[q] = mem[p];
    mem[q].hh.v.RH = MIN_HALFWORD;
    mem[q + 1].cint = mem[p + 1].cint;
    mem[q + 2].cint = mem[p + 2].cint;
    mem[q + 3].cint = mem[p + 3].cint;
    Result = q;
    return Result;
}

int32_t new_param_glue(small_number n)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    int32_t q;

    p = get_node(MEDIUM_NODE_SIZE);
    mem[p].hh.u.B0 = GLUE_NODE;
    mem[p].hh.u.B1 = n + 1;
    mem[p + 1].hh.v.RH = MIN_HALFWORD;
    q = /*232: */ eqtb[GLUE_BASE + n].hh.v.RH /*:232 */ ;
    mem[p + 1].hh.v.LH = q;
    mem[q].hh.v.RH++;
    Result = p;
    return Result;
}

int32_t new_glue(int32_t q)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(MEDIUM_NODE_SIZE);
    mem[p].hh.u.B0 = GLUE_NODE;
    mem[p].hh.u.B1 = NORMAL;
    mem[p + 1].hh.v.RH = MIN_HALFWORD;
    mem[p + 1].hh.v.LH = q;
    mem[q].hh.v.RH++;
    Result = p;
    return Result;
}

int32_t new_skip_param(small_number n)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;

    temp_ptr = new_spec( /*232: */ eqtb[GLUE_BASE + n].hh.v.RH /*:232 */ );
    p = new_glue(temp_ptr);
    mem[temp_ptr].hh.v.RH = MIN_HALFWORD;
    mem[p].hh.u.B1 = n + 1;
    Result = p;
    return Result;
}

int32_t new_kern(scaled w)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(MEDIUM_NODE_SIZE);
    mem[p].hh.u.B0 = KERN_NODE;
    mem[p].hh.u.B1 = NORMAL;
    mem[p + 1].cint = w;
    Result = p;
    return Result;
}

int32_t new_penalty(integer m)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(MEDIUM_NODE_SIZE);
    mem[p].hh.u.B0 = PENALTY_NODE;
    mem[p].hh.u.B1 = 0;
    mem[p + 1].cint = m;
    Result = p;
    return Result;
}

/*:165*/

int32_t prev_rightmost(int32_t s, int32_t e)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    Result = MIN_HALFWORD;
    p = s;
    if (p == MIN_HALFWORD)
        return Result;
    while (mem[p].hh.v.RH != e) {

        p = mem[p].hh.v.RH;
        if (p == MIN_HALFWORD)
            return Result;
    }
    Result = p;
    return Result;
}

void
short_display(integer p)
{
    memory_word *mem = zmem;
    integer n;

    while (p > 0) {
        if (p >= hi_mem_min) {
            if (p <= mem_end) {
                if (mem[p].hh.u.B0 != font_in_short_display) {
                    if (mem[p].hh.u.B0 > font_max)
                        print_char(42 /*"*" */ );
                    else /*279:*/
                        print_esc(hash[FONT_ID_BASE + mem[p].hh.u.B0].v.RH);
                    print_char(32 /*" " */ );
                    font_in_short_display = mem[p].hh.u.B0;
                }
                print(mem[p].hh.u.B1);
            }
        } else {
            /*183:*/
            switch (mem[p].hh.u.B0) {
            case HLIST_NODE:
            case VLIST_NODE:
            case INS_NODE:
            case MARK_NODE:
            case ADJUST_NODE:
            case UNSET_NODE:
                print(S(___Z4/*"[]"*/));
                break;
            case WHATSIT_NODE:
                switch (mem[p].hh.u.B1) {
                case NATIVE_WORD_NODE:
                case NATIVE_WORD_NODE_AT:
                    if (mem[p + 4].qqqq.u.B1 != font_in_short_display) {
                        print_esc(hash[FONT_ID_BASE + mem[p + 4].qqqq.u.B1].v.RH);
                        print_char(32 /*" " */ );
                        font_in_short_display = mem[p + 4].qqqq.u.B1;
                    }
                    print_native_word(p);
                    break;
                default:
                    print(S(___Z4/*"[]"*/));
                    break;
                }
                break;
            case RULE_NODE:
                print_char(124 /*"|" */ );
                break;
            case GLUE_NODE:
                if (mem[p + 1].hh.v.LH != 0)
                    print_char(32 /*" " */ );
                break;
            case MATH_NODE:
                if (mem[p].hh.u.B1 >= L_CODE)
                    print(S(___Z4/*"[]"*/));
                else
                    print_char(36 /*"$" */ );
                break;
            case LIGATURE_NODE:
                short_display(mem[p + 1].hh.v.RH);
                break;
            case DISC_NODE:
                short_display(mem[p + 1].hh.v.LH);
                short_display(mem[p + 1].hh.v.RH);
                n = mem[p].hh.u.B1;

                while (n > 0) {
                    if (mem[p].hh.v.RH != MIN_HALFWORD)
                        p = mem[p].hh.v.RH;
                    n--;
                }
                break;
            default:
                break;
            }
        }

        p = mem[p].hh.v.RH;
    }
}


void print_font_and_char(integer p)
{
    memory_word *mem = zmem; if (p > mem_end)
        print_esc(S(CLOBBERED_));
    else {

        if ((mem[p].hh.u.B0 > font_max))
            print_char(42 /*"*" */ );
        else                    /*279: */
            print_esc(hash[FONT_ID_BASE + mem[p].hh.u.B0].v.RH);
        print_char(32 /*" " */ );
        print(mem[p].hh.u.B1);
    }
}

void print_mark(integer p)
{
    memory_word *mem = zmem; print_char(123 /*"_" */ );
    if ((p < hi_mem_min) || (p > mem_end))
        print_esc(S(CLOBBERED_));
    else
        show_token_list(mem[p].hh.v.RH, MIN_HALFWORD, max_print_line - 10);
    print_char(125 /*"_" */ );
}

void print_rule_dimen(scaled d)
{
    if ((d == NULL_FLAG))
        print_char(42 /*"*" */ );
    else
        print_scaled(d);
}

void print_glue(scaled d, integer order, str_number s)
{
    print_scaled(d);
    if ((order < NORMAL) || (order > FILLL))
        print(S(foul));
    else if (order > NORMAL) {
        print(S(fil));
        while (order > FIL) {

            print_char(108 /*"l" */ );
            order--;
        }
    } else if (s != 0)
        print(s);
}

void print_spec(integer p, str_number s)
{
    memory_word *mem = zmem;

    if (p < 0 || p >= lo_mem_max)
        print_char(42 /*"*" */ );
    else {
        print_scaled(mem[p + 1].cint);
        if (s != 0)
            print(s);
        if (mem[p + 2].cint != 0) {
            print(S(_plus_));
            print_glue(mem[p + 2].cint, mem[p].hh.u.B0, s);
        }
        if (mem[p + 3].cint != 0) {
            print(S(_minus_));
            print_glue(mem[p + 3].cint, mem[p].hh.u.B1, s);
        }
    }
}

void print_fam_and_char(int32_t p)
{
    memory_word *mem = zmem; integer c;
    print_esc(S(fam));
    print_int((mem[p].hh.u.B0 % 256) % 256);
    print_char(32 /*" " */ );
    c = ((unsigned short) mem[p].hh.u.B1 + ((mem[p].hh.u.B0 / 256) * 65536L));
    if (c < 65536L)
        print(c);
    else
        print_char(c);
}

void print_delimiter(int32_t p)
{
    memory_word *mem = zmem; integer a;
    a = (mem[p].qqqq.u.B0 % 256) * 256 + (mem[p].qqqq.u.B1 + (mem[p].qqqq.u.B0 / 256) * 65536L);
    a = a * 4096 + (mem[p].qqqq.u.B2 % 256) * 256 + (mem[p].qqqq.u.B3 + (mem[p].qqqq.u.B2 / 256) * 65536L);
    if (a < 0)
        print_int(a);
    else
        print_hex(a);
}


void
print_subsidiary_data(int32_t p, UTF16_code c)
{
    memory_word *mem = zmem;

    if (pool_ptr - str_start[(str_ptr) - 65536L] >= depth_threshold) {
        if (mem[p].hh.v.RH != EMPTY)
            print(S(____Z2/*" []"*/));
    } else {
        str_pool[pool_ptr] = c;
        pool_ptr++;
        temp_ptr = p;

        switch (mem[p].hh.v.RH) {
        case MATH_CHAR:
            print_ln();
            print_current_string();
            print_fam_and_char(p);
            break;
        case SUB_BOX:
            show_info();
            break;
        case SUB_MLIST:
            if (mem[p].hh.v.LH == MIN_HALFWORD) {
                print_ln();
                print_current_string();
                print(66232L /*"__" */ );
            } else {
                show_info();
            }
            break;
        default:
            break;
        }

        pool_ptr--;
    }
}


void print_style(integer c)
{
    switch (c / 2) {
    case 0:
        print_esc(S(displaystyle));
        break;
    case 1:
        print_esc(S(textstyle));
        break;
    case 2:
        print_esc(S(scriptstyle));
        break;
    case 3:
        print_esc(S(scriptscriptstyle));
        break;
    default:
        print(S(Unknown_style_));
        break;
    }
}

void print_skip_param(integer n)
{
    switch (n) {
    case GLUE_PAR__line_skip:
        print_esc(S(lineskip));
        break;
    case GLUE_PAR__baseline_skip:
        print_esc(S(baselineskip));
        break;
    case GLUE_PAR__par_skip:
        print_esc(S(parskip));
        break;
    case GLUE_PAR__above_display_skip:
        print_esc(S(abovedisplayskip));
        break;
    case GLUE_PAR__below_display_skip:
        print_esc(S(belowdisplayskip));
        break;
    case GLUE_PAR__above_display_short_skip:
        print_esc(S(abovedisplayshortskip));
        break;
    case GLUE_PAR__below_display_short_skip:
        print_esc(S(belowdisplayshortskip));
        break;
    case GLUE_PAR__left_skip:
        print_esc(S(leftskip));
        break;
    case GLUE_PAR__right_skip:
        print_esc(S(rightskip));
        break;
    case GLUE_PAR__top_skip:
        print_esc(S(topskip));
        break;
    case GLUE_PAR__split_top_skip:
        print_esc(S(splittopskip));
        break;
    case GLUE_PAR__tab_skip:
        print_esc(S(tabskip));
        break;
    case GLUE_PAR__space_skip:
        print_esc(S(spaceskip));
        break;
    case GLUE_PAR__xspace_skip:
        print_esc(S(xspaceskip));
        break;
    case GLUE_PAR__par_fill_skip:
        print_esc(S(parfillskip));
        break;
    case GLUE_PAR__xetex_linebreak_skip:
        print_esc(S(XeTeXlinebreakskip));
        break;
    case GLUE_PAR__thin_mu_skip:
        print_esc(S(thinmuskip));
        break;
    case GLUE_PAR__med_mu_skip:
        print_esc(S(medmuskip));
        break;
    case GLUE_PAR__thick_mu_skip:
        print_esc(S(thickmuskip));
        break;
    default:
        print(S(_unknown_glue_parameter__));
        break;
    }
}


void
show_node_list(integer p)
{
    memory_word *mem = zmem;
    integer n;
    integer i;
    double g;

    if (pool_ptr - str_start[(str_ptr) - 65536L] > depth_threshold) {
        if (p > MIN_HALFWORD)
            print(S(____Z2/*" []"*/));
        return;
    }

    n = 0;

    while (p > 0) {
        print_ln();
        print_current_string();

        if (p > mem_end) {
            print(S(Bad_link__display_aborted_));
            return;
        }

        n++;

        if (n > breadth_max) {
            print(S(etc_));
            return;
        }

        if (p >= hi_mem_min) {
            print_font_and_char(p);
        } else {
            switch (mem[p].hh.u.B0) {
            case HLIST_NODE:
            case VLIST_NODE:
            case UNSET_NODE:
                if (mem[p].hh.u.B0 == HLIST_NODE)
                    print_esc(104 /*"h" */ );
                else if (mem[p].hh.u.B0 == VLIST_NODE)
                    print_esc(118 /*"v" */ );
                else
                    print_esc(S(unset));

                print(S(box_));
                print_scaled(mem[p + 3].cint);
                print_char(43 /*"+" */ );
                print_scaled(mem[p + 2].cint);
                print(S(_x));
                print_scaled(mem[p + 1].cint);

                if (mem[p].hh.u.B0 == UNSET_NODE) { /*193:*/
                    if (mem[p].hh.u.B1 != 0) {
                        print(S(___Z2/*" ("*/));
                        print_int(mem[p].hh.u.B1 + 1);
                        print(S(_columns_));
                    }
                    if (mem[p + 6].cint != 0) {
                        print(S(__stretch_));
                        print_glue(mem[p + 6].cint, mem[p + 5].hh.u.B1, 0);
                    }
                    if (mem[p + 4].cint != 0) {
                        print(S(__shrink_));
                        print_glue(mem[p + 4].cint, mem[p + 5].hh.u.B0, 0);
                    }
                } else {
                    g = mem[p + 6].gr;

                    if (g != 0.0 && mem[p + 5].hh.u.B0 != NORMAL) {
                        print(S(__glue_set_));
                        if (mem[p + 5].hh.u.B0 == SHRINKING)
                            print(S(___Z5/*"- "*/));

                        if (fabs(g) > 20000.0) {
                            if (g > 0.0)
                                print_char(62 /*">" */ );
                            else
                                print(S(____Z3/*"< -"*/));
                            print_glue(20000 * 65536L, mem[p + 5].hh.u.B1, 0);
                        } else {
                            print_glue(tex_round(65536L * g), mem[p + 5].hh.u.B1, 0);
                        }
                    }

                    if (mem[p + 4].cint != 0) {
                        print(S(__shifted_));
                        print_scaled(mem[p + 4].cint);
                    }

                    /*1491:*/
                    if (mem[p].hh.u.B0 == HLIST_NODE && mem[p].hh.u.B1 == DLIST)
                        print(S(__display));
                }

                str_pool[pool_ptr] = 46 /*"." */ ;
                pool_ptr++;
                show_node_list(mem[p + 5].hh.v.RH);
                pool_ptr--;
                break;

            case RULE_NODE:
                print_esc(S(rule_));
                print_rule_dimen(mem[p + 3].cint);
                print_char(43 /*"+" */ );
                print_rule_dimen(mem[p + 2].cint);
                print(S(_x));
                print_rule_dimen(mem[p + 1].cint);
                break;

            case INS_NODE:
                print_esc(S(insert));
                print_int(mem[p].hh.u.B1);
                print(S(__natural_size_));
                print_scaled(mem[p + 3].cint);
                print(S(__split_));
                print_spec(mem[p + 4].hh.v.RH, 0);
                print_char(44 /*"," */ );
                print_scaled(mem[p + 2].cint);
                print(S(___float_cost_));
                print_int(mem[p + 1].cint);
                str_pool[pool_ptr] = 46 /*"." */ ;
                pool_ptr++;
                show_node_list(mem[p + 4].hh.v.LH);
                pool_ptr--;
                break;

            case WHATSIT_NODE:
                switch (mem[p].hh.u.B1) {
                case OPEN_NODE:
                    print_write_whatsit(S(openout), p);
                    print_char(61 /*"=" */ );
                    print_file_name(mem[p + 1].hh.v.RH, mem[p + 2].hh.v.LH, mem[p + 2].hh.v.RH);
                    break;
                case WRITE_NODE:
                    print_write_whatsit(S(write), p);
                    print_mark(mem[p + 1].hh.v.RH);
                    break;
                case CLOSE_NODE:
                    print_write_whatsit(S(closeout), p);
                    break;
                case SPECIAL_NODE:
                    print_esc(S(special));
                    print_mark(mem[p + 1].hh.v.RH);
                    break;
                case LANGUAGE_NODE:
                    print_esc(S(setlanguage));
                    print_int(mem[p + 1].hh.v.RH);
                    print(S(__hyphenmin_));
                    print_int(mem[p + 1].hh.u.B0);
                    print_char(44 /*"," */ );
                    print_int(mem[p + 1].hh.u.B1);
                    print_char(41 /*")" */ );
                    break;
                case NATIVE_WORD_NODE:
                case NATIVE_WORD_NODE_AT:
                    print_esc(hash[FONT_ID_BASE + mem[p + 4].qqqq.u.B1].v.RH);
                    print_char(32 /*" " */ );
                    print_native_word(p);
                    break;
                case GLYPH_NODE:
                    print_esc(hash[FONT_ID_BASE + mem[p + 4].qqqq.u.B1].v.RH);
                    print(S(_glyph_));
                    print_int(mem[p + 4].qqqq.u.B2);
                    break;
                case PIC_NODE:
                case PDF_NODE:
                    if (mem[p].hh.u.B1 == PIC_NODE)
                        print_esc(S(XeTeXpicfile));
                    else
                        print_esc(S(XeTeXpdffile));

                    print(S(___Z20/*" ""*/));
                    for (i = 0; i <= mem[p + 4].hh.u.B0 - 1; i++)
                        print_raw_char(pic_path_byte(p, i), true);
                    print(34 /*""" */ );
                    break;
                case PDF_SAVE_POS_NODE:
                    print_esc(S(pdfsavepos));
                    break;
                default:
                    print(S(whatsit_));
                    break;
                }
                break; /* WHATSIT_NODE */

            case GLUE_NODE:
                if (mem[p].hh.u.B1 >= A_LEADERS) {      /*198: */
                    print_esc(S());
                    if (mem[p].hh.u.B1 == C_LEADERS)
                        print_char(99 /*"c" */ );
                    else if (mem[p].hh.u.B1 == X_LEADERS)
                        print_char(120 /*"x" */ );
                    print(S(leaders_));
                    print_spec(mem[p + 1].hh.v.LH, 0);
                    str_pool[pool_ptr] = 46 /*"." */ ;
                    pool_ptr++;
                    show_node_list(mem[p + 1].hh.v.RH);
                    pool_ptr--;
                } else {
                    print_esc(S(glue));

                    if (mem[p].hh.u.B1 != NORMAL) {
                        print_char(40 /*"(" */ );
                        if (mem[p].hh.u.B1 < COND_MATH_GLUE)
                            print_skip_param(mem[p].hh.u.B1 - 1);
                        else if (mem[p].hh.u.B1 == COND_MATH_GLUE)
                            print_esc(S(nonscript));
                        else
                            print_esc(S(mskip));
                        print_char(41 /*")" */ );
                    }

                    if (mem[p].hh.u.B1 != COND_MATH_GLUE) {
                        print_char(32 /*" " */ );
                        if (mem[p].hh.u.B1 < COND_MATH_GLUE)
                            print_spec(mem[p + 1].hh.v.LH, 0);
                        else
                            print_spec(mem[p + 1].hh.v.LH, S(mu));
                    }
                }
                break;

            case KERN_NODE:
                if (mem[p].hh.u.B1 != MU_GLUE) {
                    print_esc(S(kern));
                    if (mem[p].hh.u.B1 != NORMAL)
                        print_char(32 /*" " */ );
                    print_scaled(mem[p + 1].cint);
                    if (mem[p].hh.u.B1 == ACC_KERN)
                        print(S(__for_accent_));
                    else if (mem[p].hh.u.B1 == SPACE_ADJUSTMENT)
                        print(S(__space_adjustment_));
                } else {
                    print_esc(S(mkern));
                    print_scaled(mem[p + 1].cint);
                    print(S(mu));
                }
                break;

            case MARGIN_KERN_NODE:
                print_esc(S(kern));
                print_scaled(mem[p + 1].cint);
                if (mem[p].hh.u.B1 == 0)
                    print(S(__left_margin_));
                else
                    print(S(__right_margin_));
                break;

            case MATH_NODE:
                if (mem[p].hh.u.B1 > AFTER) {
                    if (odd(mem[p].hh.u.B1))
                        print_esc(S(end));
                    else
                        print_esc(S(begin));
                    if (mem[p].hh.u.B1 > R_CODE)
                        print_char(82 /*"R" */ );
                    else if (mem[p].hh.u.B1 > L_CODE)
                        print_char(76 /*"L" */ );
                    else
                        print_char(77 /*"M" */ );
                } else {
                    print_esc(S(math));
                    if (mem[p].hh.u.B1 == BEFORE)
                        print(S(on));
                    else
                        print(S(off));
                    if (mem[p + 1].cint != 0) {
                        print(S(__surrounded_));
                        print_scaled(mem[p + 1].cint);
                    }
                }
                break;

            case LIGATURE_NODE:
                print_font_and_char(p + 1);
                print(S(__ligature_));
                if (mem[p].hh.u.B1 > 1)
                    print_char(124 /*"|" */ );
                font_in_short_display = mem[p + 1].hh.u.B0;
                short_display(mem[p + 1].hh.v.RH);
                if (odd(mem[p].hh.u.B1))
                    print_char(124 /*"|" */ );
                print_char(41 /*")" */ );
                break;

            case PENALTY_NODE:
                print_esc(S(penalty_));
                print_int(mem[p + 1].cint);
                break;

            case DISC_NODE:
                print_esc(S(discretionary));
                if (mem[p].hh.u.B1 > 0) {
                    print(S(_replacing_));
                    print_int(mem[p].hh.u.B1);
                }

                str_pool[pool_ptr] = 46 /*"." */ ;
                pool_ptr++;
                show_node_list(mem[p + 1].hh.v.LH);
                pool_ptr--;
                str_pool[pool_ptr] = 124 /*"|" */ ;
                pool_ptr++;
                show_node_list(mem[p + 1].hh.v.RH);
                pool_ptr--;
                break;

            case MARK_NODE:
                print_esc(S(mark));
                if (mem[p + 1].hh.v.LH != 0) {
                    print_char(115 /*"s" */ );
                    print_int(mem[p + 1].hh.v.LH);
                }
                print_mark(mem[p + 1].hh.v.RH);
                break;

            case ADJUST_NODE:
                print_esc(S(vadjust));
                if (mem[p].hh.u.B1 != 0)
                    print(S(_pre_));

                str_pool[pool_ptr] = 46 /*"." */ ;
                pool_ptr++;
                show_node_list(mem[p + 1].cint);
                pool_ptr--;
                break;

            case STYLE_NODE:
                print_style(mem[p].hh.u.B1);
                break;

            case CHOICE_NODE:
                print_esc(S(mathchoice));
                str_pool[pool_ptr] = 68 /*"D" */ ;
                pool_ptr++;
                show_node_list(mem[p + 1].hh.v.LH);
                pool_ptr--;
                str_pool[pool_ptr] = 84 /*"T" */ ;
                pool_ptr++;
                show_node_list(mem[p + 1].hh.v.RH);
                pool_ptr--;
                str_pool[pool_ptr] = 83 /*"S" */ ;
                pool_ptr++;
                show_node_list(mem[p + 2].hh.v.LH);
                pool_ptr--;
                str_pool[pool_ptr] = 115 /*"s" */ ;
                pool_ptr++;
                show_node_list(mem[p + 2].hh.v.RH);
                pool_ptr--;
                break;

            case ORD_NOAD:
            case OP_NOAD:
            case BIN_NOAD:
            case REL_NOAD:
            case OPEN_NOAD:
            case CLOSE_NOAD:
            case PUNCT_NOAD:
            case INNER_NOAD:
            case RADICAL_NOAD:
            case OVER_NOAD:
            case UNDER_NOAD:
            case VCENTER_NOAD:
            case ACCENT_NOAD:
            case LEFT_NOAD:
            case RIGHT_NOAD:
                {
                    switch (mem[p].hh.u.B0) {
                    case ORD_NOAD:
                        print_esc(S(mathord));
                        break;
                    case OP_NOAD:
                        print_esc(S(mathop));
                        break;
                    case BIN_NOAD:
                        print_esc(S(mathbin));
                        break;
                    case REL_NOAD:
                        print_esc(S(mathrel));
                        break;
                    case OPEN_NOAD:
                        print_esc(S(mathopen));
                        break;
                    case CLOSE_NOAD:
                        print_esc(S(mathclose));
                        break;
                    case PUNCT_NOAD:
                        print_esc(S(mathpunct));
                        break;
                    case INNER_NOAD:
                        print_esc(S(mathinner));
                        break;
                    case OVER_NOAD:
                        print_esc(S(overline));
                        break;
                    case UNDER_NOAD:
                        print_esc(S(underline));
                        break;
                    case VCENTER_NOAD:
                        print_esc(S(vcenter));
                        break;
                    case RADICAL_NOAD:
                        print_esc(S(radical));
                        print_delimiter(p + 4);
                        break;
                    case ACCENT_NOAD:
                        print_esc(S(accent));
                        print_fam_and_char(p + 4);
                        break;
                    case LEFT_NOAD:
                        print_esc(S(left));
                        print_delimiter(p + 1);
                        break;
                    case RIGHT_NOAD:
                        if (mem[p].hh.u.B1 == NORMAL)
                            print_esc(S(right));
                        else
                            print_esc(S(middle));
                        print_delimiter(p + 1);
                        break;
                    }

                    if (mem[p].hh.u.B0 < LEFT_NOAD) {
                        if (mem[p].hh.u.B1 != NORMAL) {
                            if (mem[p].hh.u.B1 == LIMITS)
                                print_esc(S(limits));
                            else
                                print_esc(S(nolimits));
                        }
                        print_subsidiary_data(p + 1, 46 /*"." */ );
                    }

                    print_subsidiary_data(p + 2, 94 /*"^" */ );
                    print_subsidiary_data(p + 3, 95 /*"_" */ );
                }
                break; /* many math noads */

            case FRACTION_NOAD:
                print_esc(S(fraction__thickness_));
                if (mem[p + 1].cint == DEFAULT_CODE)
                    print(S(__default));
                else
                    print_scaled(mem[p + 1].cint);

                if (mem[p + 4].qqqq.u.B0 % 256 != 0 ||
                    (mem[p + 4].qqqq.u.B1 + (mem[p + 4].qqqq.u.B0 / 256) * 65536L) != 0 ||
                    mem[p + 4].qqqq.u.B2 % 256 != 0 ||
                    (mem[p + 4].qqqq.u.B3 + (mem[p + 4].qqqq.u.B2 / 256) * 65536L) != 0) {
                    print(S(__left_delimiter_));
                    print_delimiter(p + 4);
                }

                if (mem[p + 5].qqqq.u.B0 % 256 != 0 ||
                    (mem[p + 5].qqqq.u.B1 + (mem[p + 5].qqqq.u.B0 / 256) * 65536L) != 0 ||
                    mem[p + 5].qqqq.u.B2 % 256 != 0 ||
                    (mem[p + 5].qqqq.u.B3 + (mem[p + 5].qqqq.u.B2 / 256) * 65536L) != 0) {
                    print(S(__right_delimiter_));
                    print_delimiter(p + 5);
                }

                print_subsidiary_data(p + 2, 92 /*"\" */ );
                print_subsidiary_data(p + 3, 47 /*"/" */ );
                break;

            default:
                print(S(Unknown_node_type_));
                break;
            }
        }

        p = mem[p].hh.v.RH;
    }
}


void show_box(int32_t p)
{
    CACHE_THE_EQTB;

    depth_threshold = INTPAR(show_box_depth);
    breadth_max = INTPAR(show_box_breadth) /*:244 */ ;
    if (breadth_max <= 0)
        breadth_max = 5;
    if (pool_ptr + depth_threshold >= pool_size)
        depth_threshold = pool_size - pool_ptr - 1;
    show_node_list(p);
    print_ln();
}

void short_display_n(integer p, integer m)
{
    breadth_max = m;
    depth_threshold = pool_size - pool_ptr - 1;
    show_node_list(p);
}

void delete_token_ref(int32_t p)
{
    memory_word *mem = zmem; if (mem[p].hh.v.LH == MIN_HALFWORD)
        flush_list(p);
    else
        mem[p].hh.v.LH--;
}

void delete_glue_ref(int32_t p)
{
    memory_word *mem = zmem; if (mem[p].hh.v.RH == MIN_HALFWORD)
        free_node(p, GLUE_SPEC_SIZE);
    else
        mem[p].hh.v.RH--;
}


void
flush_node_list(int32_t p)
{
    memory_word *mem = zmem;
    int32_t q;

    while (p != MIN_HALFWORD) {
        q = mem[p].hh.v.RH;

        if (p >= hi_mem_min) {
            mem[p].hh.v.RH = avail;
            avail = p;
        } else {
            switch (mem[p].hh.u.B0) {
            case HLIST_NODE:
            case VLIST_NODE:
            case UNSET_NODE:
                flush_node_list(mem[p + 5].hh.v.RH);
                free_node(p, BOX_NODE_SIZE);
                goto done;
                break;

            case RULE_NODE:
                free_node(p, RULE_NODE_SIZE);
                goto done;
                break;

            case INS_NODE:
                flush_node_list(mem[p + 4].hh.v.LH);
                delete_glue_ref(mem[p + 4].hh.v.RH);
                free_node(p, INS_NODE_SIZE);
                goto done;
                break;

            case WHATSIT_NODE:
                switch (mem[p].hh.u.B1) {
                case OPEN_NODE:
                    free_node(p, OPEN_NODE_SIZE);
                    break;
                case WRITE_NODE:
                case SPECIAL_NODE:
                    delete_token_ref(mem[p + 1].hh.v.RH);
                    free_node(p, WRITE_NODE_SIZE);
                    goto done;
                    break;
                case CLOSE_NODE:
                case LANGUAGE_NODE:
                    free_node(p, SMALL_NODE_SIZE);
                    break;
                case NATIVE_WORD_NODE:
                case NATIVE_WORD_NODE_AT:
                    if (mem[p + 5].ptr != NULL) {
                        free(mem[p + 5].ptr);
                        mem[p + 5].ptr = NULL;
                        mem[p + 4].qqqq.u.B3 = 0;
                    }
                    free_node(p, mem[p + 4].qqqq.u.B0);
                    break;
                case GLYPH_NODE:
                    free_node(p, GLYPH_NODE_SIZE);
                    break;
                case PIC_NODE:
                case PDF_NODE:
                    free_node(p,
                              (PIC_NODE_SIZE +
                               (mem[p + 4].hh.u.B0 + sizeof(memory_word) - 1) / sizeof(memory_word)));
                    break;
                case PDF_SAVE_POS_NODE:
                    free_node(p, SMALL_NODE_SIZE);
                    break;
                default:
                    confusion(S(ext3));
                    break;
                }
                goto done;
                break;

            case GLUE_NODE:
                if (mem[mem[p + 1].hh.v.LH].hh.v.RH == MIN_HALFWORD)
                    free_node(mem[p + 1].hh.v.LH, GLUE_SPEC_SIZE);
                else
                    mem[mem[p + 1].hh.v.LH].hh.v.RH--;

                if (mem[p + 1].hh.v.RH != MIN_HALFWORD)
                    flush_node_list(mem[p + 1].hh.v.RH);
                free_node(p, MEDIUM_NODE_SIZE);
                goto done;
                break;

            case KERN_NODE:
            case MATH_NODE:
            case PENALTY_NODE:
                free_node(p, MEDIUM_NODE_SIZE);
                goto done;
                break;

            case MARGIN_KERN_NODE:
                free_node(p, MARGIN_KERN_NODE_SIZE);
                goto done;
                break;

            case LIGATURE_NODE:
                flush_node_list(mem[p + 1].hh.v.RH);
                break;

            case MARK_NODE:
                delete_token_ref(mem[p + 1].hh.v.RH);
                break;

            case DISC_NODE:
                flush_node_list(mem[p + 1].hh.v.LH);
                flush_node_list(mem[p + 1].hh.v.RH);
                break;

            case ADJUST_NODE:
                flush_node_list(mem[p + 1].cint);
                break;

            case STYLE_NODE:
                free_node(p, STYLE_NODE_SIZE);
                goto done;
                break;

            case CHOICE_NODE:
                flush_node_list(mem[p + 1].hh.v.LH);
                flush_node_list(mem[p + 1].hh.v.RH);
                flush_node_list(mem[p + 2].hh.v.LH);
                flush_node_list(mem[p + 2].hh.v.RH);
                free_node(p, STYLE_NODE_SIZE);
                goto done;
                break;

            case ORD_NOAD:
            case OP_NOAD:
            case BIN_NOAD:
            case REL_NOAD:
            case OPEN_NOAD:
            case CLOSE_NOAD:
            case PUNCT_NOAD:
            case INNER_NOAD:
            case RADICAL_NOAD:
            case OVER_NOAD:
            case UNDER_NOAD:
            case VCENTER_NOAD:
            case ACCENT_NOAD:
                if (mem[p + 1].hh.v.RH >= SUB_BOX)
                    flush_node_list(mem[p + 1].hh.v.LH);
                if (mem[p + 2].hh.v.RH >= SUB_BOX)
                    flush_node_list(mem[p + 2].hh.v.LH);
                if (mem[p + 3].hh.v.RH >= SUB_BOX)
                    flush_node_list(mem[p + 3].hh.v.LH);
                if (mem[p].hh.u.B0 == RADICAL_NOAD)
                    free_node(p, RADICAL_NOAD_SIZE);
                else if (mem[p].hh.u.B0 == ACCENT_NOAD)
                    free_node(p, ACCENT_NOAD_SIZE);
                else
                    free_node(p, NOAD_SIZE);
                goto done;
                break;

            case LEFT_NOAD:
            case RIGHT_NOAD:
                free_node(p, NOAD_SIZE);
                goto done;
                break;

            case FRACTION_NOAD:
                flush_node_list(mem[p + 2].hh.v.LH);
                flush_node_list(mem[p + 3].hh.v.LH);
                free_node(p, FRACTION_NOAD_SIZE);
                goto done;
                break;

            default:
                confusion(S(flushing));
                break;
            }

            free_node(p, SMALL_NODE_SIZE);
        done:
            ;
        }

        p = q;
    }
}


int32_t
copy_node_list(int32_t p)
{
    register int32_t Result;
    memory_word *mem = zmem;
    int32_t h;
    int32_t q;
    int32_t r;
    unsigned char words;

    h = get_avail();
    q = h;

    while (p != MIN_HALFWORD) {
        words = 1;
        if (p >= hi_mem_min) {
            r = get_avail();
        } else { /*214:*/
            switch (mem[p].hh.u.B0) {
            case HLIST_NODE:
            case VLIST_NODE:
            case UNSET_NODE:
                r = get_node(BOX_NODE_SIZE);
                mem[r + 7].hh.v.LH = mem[p + 7].hh.v.LH;
                mem[r + 7].hh.v.RH = mem[p + 7].hh.v.RH;
                mem[r + 6] = mem[p + 6];
                mem[r + 5] = mem[p + 5];
                mem[r + 5].hh.v.RH = copy_node_list(mem[p + 5].hh.v.RH);
                words = 5;
                break;

            case RULE_NODE:
                r = get_node(RULE_NODE_SIZE);
                words = (RULE_NODE_SIZE - 1);
                break;

            case INS_NODE:
                r = get_node(INS_NODE_SIZE);
                mem[r + 4] = mem[p + 4];
                mem[mem[p + 4].hh.v.RH].hh.v.RH++;
                mem[r + 4].hh.v.LH = copy_node_list(mem[p + 4].hh.v.LH);
                words = (INS_NODE_SIZE - 1);
                break;

            case WHATSIT_NODE:
                switch (mem[p].hh.u.B1) {
                case OPEN_NODE:
                    r = get_node(OPEN_NODE_SIZE);
                    words = OPEN_NODE_SIZE;
                    break;
                case WRITE_NODE:
                case SPECIAL_NODE:
                    r = get_node(WRITE_NODE_SIZE);
                    mem[mem[p + 1].hh.v.RH].hh.v.LH++;
                    words = WRITE_NODE_SIZE;
                    break;
                case CLOSE_NODE:
                case LANGUAGE_NODE:
                    r = get_node(SMALL_NODE_SIZE);
                    words = SMALL_NODE_SIZE;
                    break;
                case NATIVE_WORD_NODE:
                case NATIVE_WORD_NODE_AT:
                    words = mem[p + 4].qqqq.u.B0;
                    r = get_node(words);

                    while (words > 0) {
                        words--;
                        mem[r + words] = mem[p + words];
                    }

                    mem[r + 5].ptr = NULL;
                    mem[r + 4].qqqq.u.B3 = 0;
                    copy_native_glyph_info(p, r);
                    break;
                case GLYPH_NODE:
                    r = get_node(GLYPH_NODE_SIZE);
                    words = GLYPH_NODE_SIZE;
                    break;
                case PIC_NODE:
                case PDF_NODE:
                    words =
                        (PIC_NODE_SIZE +
                         (mem[p + 4].hh.u.B0 + sizeof(memory_word) - 1) / sizeof(memory_word));
                    r = get_node(words);
                    break;
                case PDF_SAVE_POS_NODE:
                    r = get_node(SMALL_NODE_SIZE);
                    break;
                default:
                    confusion(S(ext2));
                    break;
                }
                break;

            case GLUE_NODE:
                r = get_node(MEDIUM_NODE_SIZE);
                mem[mem[p + 1].hh.v.LH].hh.v.RH++;
                mem[r + 2].hh.v.LH = mem[p + 2].hh.v.LH;
                mem[r + 2].hh.v.RH = mem[p + 2].hh.v.RH;
                mem[r + 1].hh.v.LH = mem[p + 1].hh.v.LH;
                mem[r + 1].hh.v.RH = copy_node_list(mem[p + 1].hh.v.RH);
                break;

            case KERN_NODE:
            case MATH_NODE:
            case PENALTY_NODE:
                r = get_node(MEDIUM_NODE_SIZE);
                words = MEDIUM_NODE_SIZE;
                break;

            case MARGIN_KERN_NODE:
                r = get_node(MARGIN_KERN_NODE_SIZE);
                words = MARGIN_KERN_NODE_SIZE;
                break;

            case LIGATURE_NODE:
                r = get_node(SMALL_NODE_SIZE);
                mem[r + 1] = mem[p + 1];
                mem[r + 1].hh.v.RH = copy_node_list(mem[p + 1].hh.v.RH);
                break;

            case DISC_NODE:
                r = get_node(SMALL_NODE_SIZE);
                mem[r + 1].hh.v.LH = copy_node_list(mem[p + 1].hh.v.LH);
                mem[r + 1].hh.v.RH = copy_node_list(mem[p + 1].hh.v.RH);
                break;

            case MARK_NODE:
                r = get_node(SMALL_NODE_SIZE);
                mem[mem[p + 1].hh.v.RH].hh.v.LH++;
                words = SMALL_NODE_SIZE;
                break;

            case ADJUST_NODE:
                r = get_node(SMALL_NODE_SIZE);
                mem[r + 1].cint = copy_node_list(mem[p + 1].cint);
                break;

            default:
                confusion(S(copying));
                break;
            }
        }

        while (words > 0) {
            words--;
            mem[r + words] = mem[p + words];
        }

        mem[q].hh.v.RH = r;
        q = r;
        p = mem[p].hh.v.RH;
    }

    mem[q].hh.v.RH = MIN_HALFWORD;
    q = mem[h].hh.v.RH;
    mem[h].hh.v.RH = avail;
    avail = h;
    Result = q;
    return Result;
}


void print_mode(integer m)
{
    if (m > 0)
        switch (m / ((MAX_COMMAND + 1))) {
        case 0:
            print(S(vertical_mode));
            break;
        case 1:
            print(S(horizontal_mode));
            break;
        case 2:
            print(S(display_math_mode));
            break;
    } else if (m == 0)
        print(S(no_mode));
    else
        switch ((-(integer) m) / ((MAX_COMMAND + 1))) {
        case 0:
            print(S(internal_vertical_mode));
            break;
        case 1:
            print(S(restricted_horizontal_mode));
            break;
        case 2:
            print(S(math_mode));
            break;
        }
}

void print_in_mode(integer m)
{
    if (m > 0)
        switch (m / ((MAX_COMMAND + 1))) {
        case 0:
            print(S(__in_vertical_mode));
            break;
        case 1:
            print(S(__in_horizontal_mode));
            break;
        case 2:
            print(S(__in_display_math_mode));
            break;
    } else if (m == 0)
        print(S(__in_no_mode));
    else
        switch ((-(integer) m) / ((MAX_COMMAND + 1))) {
        case 0:
            print(S(__in_internal_vertical_mode));
            break;
        case 1:
            print(S(__in_restricted_horizontal_m/*ode*/));
            break;
        case 2:
            print(S(__in_math_mode));
            break;
        }
}

void push_nest(void)
{
    if (nest_ptr > max_nest_stack) {
        max_nest_stack = nest_ptr;
        if (nest_ptr == nest_size)
            overflow(S(semantic_nest_size), nest_size);
    }
    nest[nest_ptr] = cur_list;
    nest_ptr++;
    cur_list.head = get_avail();
    cur_list.tail = cur_list.head;
    cur_list.pg = 0;
    cur_list.ml = line;
    cur_list.eTeX_aux = MIN_HALFWORD;
}

void pop_nest(void)
{
    memory_word *mem = zmem; {
        mem[cur_list.head].hh.v.RH = avail;
        avail = cur_list.head;
    }
    nest_ptr--;
    cur_list = nest[nest_ptr];
}

void show_activities(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer p;
    short /*mmode */ m;
    memory_word a;
    int32_t q, r;
    integer t;

    nest[nest_ptr] = cur_list;
    print_nl(S());
    print_ln();
    {
        register integer for_end;
        p = nest_ptr;
        for_end = 0;
        if (p >= for_end)
            do {
                m = nest[p].mode;
                a = nest[p].aux;
                print_nl(S(____/*"### "*/));
                print_mode(m);
                print(S(_entered_at_line_));
                print_int(abs(nest[p].ml));
                if (m == HMODE) {

                    if (nest[p].pg != 8585216L) {
                        print(S(__language));
                        print_int(nest[p].pg % 65536L);
                        print(S(_hyphenmin));
                        print_int(nest[p].pg / 4194304L);
                        print_char(44 /*"," */ );
                        print_int((nest[p].pg / 65536L) % 64);
                        print_char(41 /*")" */ );
                    }
                }
                if (nest[p].ml < 0)
                    print(S(___output_routine_));
                if (p == 0) {
                    if (mem_top - 2 != page_tail) {
                        print_nl(S(____current_page_));
                        if (output_active)
                            print(S(__held_over_for_next_output_/**/));
                        show_box(mem[mem_top - 2].hh.v.RH);
                        if (page_contents > EMPTY) {
                            print_nl(S(total_height_));
                            print_totals();
                            print_nl(S(_goal_height_));
                            print_scaled(page_so_far[0]);
                            r = mem[mem_top].hh.v.RH;
                            while (r != mem_top) {

                                print_ln();
                                print_esc(S(insert));
                                t = mem[r].hh.u.B1;
                                print_int(t);
                                print(S(_adds_));
                                if (COUNT_REG(t) == 1000)
                                    t = mem[r + 3].cint;
                                else
                                    t = x_over_n(mem[r + 3].cint, 1000) * COUNT_REG(t);
                                print_scaled(t);
                                if (mem[r].hh.u.B0 == SPLIT_UP) {
                                    q = mem_top - 2;
                                    t = 0;
                                    do {
                                        q = mem[q].hh.v.RH;
                                        if ((mem[q].hh.u.B0 == INS_NODE) && (mem[q].hh.u.B1 == mem[r].hh.u.B1))
                                            t++;
                                    } while (!(q == mem[r + 1].hh.v.LH));
                                    print(S(____Z5/*", #"*/));
                                    print_int(t);
                                    print(S(_might_split));
                                }
                                r = mem[r].hh.v.RH;
                            }
                        }
                    }
                    if (mem[mem_top - 1].hh.v.RH != MIN_HALFWORD)
                        print_nl(S(____recent_contributions_));
                }
                show_box(mem[nest[p].head].hh.v.RH);
                switch (abs(m) / ((MAX_COMMAND + 1))) {
                case 0:
                    {
                        print_nl(S(prevdepth_));
                        if (a.cint <= -65536000L)
                            print(S(ignored));
                        else
                            print_scaled(a.cint);
                        if (nest[p].pg != 0) {
                            print(S(__prevgraf_));
                            print_int(nest[p].pg);
                            if (nest[p].pg != 1)
                                print(S(_lines));
                            else
                                print(S(_line));
                        }
                    }
                    break;
                case 1:
                    {
                        print_nl(S(spacefactor_));
                        print_int(a.hh.v.LH);
                        if (m > 0) {

                            if (a.hh.v.RH > 0) {
                                print(S(__current_language_));
                                print_int(a.hh.v.RH);
                            }
                        }
                    }
                    break;
                case 2:
                    if (a.cint != MIN_HALFWORD) {
                        print(S(this_will_be_denominator_of_/**/));
                        show_box(a.cint);
                    }
                    break;
                }
            }
            while (p-- > for_end);
    }
}

void print_param(integer n)
{
    switch (n) {
    case 0:
        print_esc(S(pretolerance));
        break;
    case 1:
        print_esc(S(tolerance));
        break;
    case 2:
        print_esc(S(linepenalty));
        break;
    case 3:
        print_esc(S(hyphenpenalty));
        break;
    case 4:
        print_esc(S(exhyphenpenalty));
        break;
    case 5:
        print_esc(S(clubpenalty));
        break;
    case 6:
        print_esc(S(widowpenalty));
        break;
    case 7:
        print_esc(S(displaywidowpenalty));
        break;
    case 8:
        print_esc(S(brokenpenalty));
        break;
    case 9:
        print_esc(S(binoppenalty));
        break;
    case 10:
        print_esc(S(relpenalty));
        break;
    case 11:
        print_esc(S(predisplaypenalty));
        break;
    case 12:
        print_esc(S(postdisplaypenalty));
        break;
    case 13:
        print_esc(S(interlinepenalty));
        break;
    case 14:
        print_esc(S(doublehyphendemerits));
        break;
    case 15:
        print_esc(S(finalhyphendemerits));
        break;
    case 16:
        print_esc(S(adjdemerits));
        break;
    case 17:
        print_esc(S(mag));
        break;
    case 18:
        print_esc(S(delimiterfactor));
        break;
    case 19:
        print_esc(S(looseness));
        break;
    case 20:
        print_esc(S(time));
        break;
    case 21:
        print_esc(S(day));
        break;
    case 22:
        print_esc(S(month));
        break;
    case 23:
        print_esc(S(year));
        break;
    case 24:
        print_esc(S(showboxbreadth));
        break;
    case 25:
        print_esc(S(showboxdepth));
        break;
    case 26:
        print_esc(S(hbadness));
        break;
    case 27:
        print_esc(S(vbadness));
        break;
    case 28:
        print_esc(S(pausing));
        break;
    case 29:
        print_esc(S(tracingonline));
        break;
    case 30:
        print_esc(S(tracingmacros));
        break;
    case 31:
        print_esc(S(tracingstats));
        break;
    case 32:
        print_esc(S(tracingparagraphs));
        break;
    case 33:
        print_esc(S(tracingpages));
        break;
    case 34:
        print_esc(S(tracingoutput));
        break;
    case 35:
        print_esc(S(tracinglostchars));
        break;
    case 36:
        print_esc(S(tracingcommands));
        break;
    case 37:
        print_esc(S(tracingrestores));
        break;
    case 38:
        print_esc(S(uchyph));
        break;
    case 39:
        print_esc(S(outputpenalty));
        break;
    case 40:
        print_esc(S(maxdeadcycles));
        break;
    case 41:
        print_esc(S(hangafter));
        break;
    case 42:
        print_esc(S(floatingpenalty));
        break;
    case 43:
        print_esc(S(globaldefs));
        break;
    case 44:
        print_esc(S(fam));
        break;
    case 45:
        print_esc(S(escapechar));
        break;
    case 46:
        print_esc(S(defaulthyphenchar));
        break;
    case 47:
        print_esc(S(defaultskewchar));
        break;
    case 48:
        print_esc(S(endlinechar));
        break;
    case 49:
        print_esc(S(newlinechar));
        break;
    case 50:
        print_esc(S(language));
        break;
    case 51:
        print_esc(S(lefthyphenmin));
        break;
    case 52:
        print_esc(S(righthyphenmin));
        break;
    case 53:
        print_esc(S(holdinginserts));
        break;
    case 54:
        print_esc(S(errorcontextlines));
        break;
    case 55:
        print_esc(S(charsubdefmin));
        break;
    case 56:
        print_esc(S(charsubdefmax));
        break;
    case 57:
        print_esc(S(tracingcharsubdef));
        break;
    case 69:
        print_esc(S(XeTeXlinebreakpenalty));
        break;
    case 70:
        print_esc(S(XeTeXprotrudechars));
        break;
    case 83:
        print_esc(S(synctex));
        break;
    case 58:
        print_esc(S(tracingassigns));
        break;
    case 59:
        print_esc(S(tracinggroups));
        break;
    case 60:
        print_esc(S(tracingifs));
        break;
    case 61:
        print_esc(S(tracingscantokens));
        break;
    case 62:
        print_esc(S(tracingnesting));
        break;
    case 63:
        print_esc(S(predisplaydirection));
        break;
    case 64:
        print_esc(S(lastlinefit));
        break;
    case 65:
        print_esc(S(savingvdiscards));
        break;
    case 66:
        print_esc(S(savinghyphcodes));
        break;
    case 67:
        print_esc(S(suppressfontnotfounderror));
        break;
    case 71:
        print_esc(S(TeXXeTstate));
        break;
    case 73:
        print_esc(S(XeTeXupwardsmode));
        break;
    case 74:
        print_esc(S(XeTeXuseglyphmetrics));
        break;
    case 75:
        print_esc(S(XeTeXinterchartokenstate));
        break;
    case 72:
        print_esc(S(XeTeXdashbreakstate));
        break;
    case 76:
        print_esc(S(XeTeXinputnormalization));
        break;
    case 79:
        print_esc(S(XeTeXtracingfonts));
        break;
    case 80:
        print_esc(S(XeTeXinterwordspaceshaping));
        break;
    case 81:
        print_esc(S(XeTeXgenerateactualtext));
        break;
    case 82:
        print_esc(S(XeTeXhyphenatablelength));
        break;
    default:
        print(S(_unknown_integer_parameter__/**/));
        break;
    }
}

void begin_diagnostic(void)
{
    CACHE_THE_EQTB;

    old_setting = selector;

    if (INTPAR(tracing_online) <= 0 && selector == SELECTOR_TERM_AND_LOG) {
        selector--;
        if (history == HISTORY_SPOTLESS)
            history = HISTORY_WARNING_ISSUED;
    }
}

void end_diagnostic(boolean blank_line)
{
    print_nl(S());
    if (blank_line)
        print_ln();
    selector = old_setting;
}

void print_length_param(integer n)
{
    switch (n) {
    case 0:
        print_esc(S(parindent));
        break;
    case 1:
        print_esc(S(mathsurround));
        break;
    case 2:
        print_esc(S(lineskiplimit));
        break;
    case 3:
        print_esc(S(hsize));
        break;
    case 4:
        print_esc(S(vsize));
        break;
    case 5:
        print_esc(S(maxdepth));
        break;
    case 6:
        print_esc(S(splitmaxdepth));
        break;
    case 7:
        print_esc(S(boxmaxdepth));
        break;
    case 8:
        print_esc(S(hfuzz));
        break;
    case 9:
        print_esc(S(vfuzz));
        break;
    case 10:
        print_esc(S(delimitershortfall));
        break;
    case 11:
        print_esc(S(nulldelimiterspace));
        break;
    case 12:
        print_esc(S(scriptspace));
        break;
    case 13:
        print_esc(S(predisplaysize));
        break;
    case 14:
        print_esc(S(displaywidth));
        break;
    case 15:
        print_esc(S(displayindent));
        break;
    case 16:
        print_esc(S(overfullrule));
        break;
    case 17:
        print_esc(S(hangindent));
        break;
    case 18:
        print_esc(S(hoffset));
        break;
    case 19:
        print_esc(S(voffset));
        break;
    case 20:
        print_esc(S(emergencystretch));
        break;
    case 21:
        print_esc(S(pdfpagewidth));
        break;
    case 22:
        print_esc(S(pdfpageheight));
        break;
    default:
        print(S(_unknown_dimen_parameter__));
        break;
    }
}

void print_cmd_chr(uint16_t cmd, int32_t chr_code)
{
    memory_word *mem = zmem; integer n;
    str_number font_name_str;
    UTF16_code quote_char;
    switch (cmd) {
    case 1:
        {
            print(S(begin_group_character_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 2:
        {
            print(S(end_group_character_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 3:
        {
            print(S(math_shift_character_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 6:
        {
            print(S(macro_parameter_character_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 7:
        {
            print(S(superscript_character_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 8:
        {
            print(S(subscript_character_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 9:
        print(S(end_of_alignment_template));
        break;
    case 10:
        {
            print(S(blank_space_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 11:
        {
            print(S(the_letter_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 12:
        {
            print(S(the_character_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 76:
    case 77:
        if (chr_code < SKIP_BASE)
            print_skip_param(chr_code - 2252240L);
        else if (chr_code < MU_SKIP_BASE) {
            print_esc(S(skip));
            print_int(chr_code - 2252259L);
        } else {

            print_esc(S(muskip));
            print_int(chr_code - 2252515L);
        }
        break;
    case 73:
        if (chr_code >= TOKS_BASE) {
            print_esc(S(toks));
            print_int(chr_code - 2252783L);
        } else
            switch (chr_code) {
            case LOCAL_BASE + LOCAL__output_routine:
                print_esc(S(output));
                break;
            case LOCAL_BASE + LOCAL__every_par:
                print_esc(S(everypar));
                break;
            case LOCAL_BASE + LOCAL__every_math:
                print_esc(S(everymath));
                break;
            case LOCAL_BASE + LOCAL__every_display:
                print_esc(S(everydisplay));
                break;
            case LOCAL_BASE + LOCAL__every_hbox:
                print_esc(S(everyhbox));
                break;
            case LOCAL_BASE + LOCAL__every_vbox:
                print_esc(S(everyvbox));
                break;
            case LOCAL_BASE + LOCAL__every_job:
                print_esc(S(everyjob));
                break;
            case LOCAL_BASE + LOCAL__every_cr:
                print_esc(S(everycr));
                break;
            case LOCAL_BASE + LOCAL__every_eof:
                print_esc(S(everyeof));
                break;
            case LOCAL_BASE + LOCAL__xetex_inter_char:
                print_esc(S(XeTeXinterchartoks));
                break;
            default:
                print_esc(S(errhelp));
                break;
            }
        break;
    case 74:
        if (chr_code < COUNT_BASE)
            print_param(chr_code - 8938740L);
        else {

            print_esc(S(count));
            print_int(chr_code - 8938824L);
        }
        break;
    case 75:
        if (chr_code < SCALED_BASE)
            print_length_param(chr_code - 10053192L);
        else {

            print_esc(S(dimen));
            print_int(chr_code - 10053215L);
        }
        break;
    case 45:
        print_esc(S(accent));
        break;
    case 92:
        print_esc(S(advance));
        break;
    case 40:
        print_esc(S(afterassignment));
        break;
    case 41:
        print_esc(S(aftergroup));
        break;
    case 78:
        print_esc(S(fontdimen));
        break;
    case 61:
        print_esc(S(begingroup));
        break;
    case 42:
        print_esc(S(penalty));
        break;
    case 16:
        print_esc(S(char));
        break;
    case 109:
        print_esc(S(csname));
        break;
    case 90:
        print_esc(S(font));
        break;
    case 15:
        if (chr_code == 1)
            print_esc(S(Udelimiter));
        else
            print_esc(S(delimiter));
        break;
    case 94:
        print_esc(S(divide));
        break;
    case 67:
        print_esc(S(endcsname));
        break;
    case 62:
        print_esc(S(endgroup));
        break;
    case 64:
        print_esc(32 /*" " */ );
        break;
    case 104:
        if (chr_code == 0)
            print_esc(S(expandafter));
        else
            print_esc(S(unless));
        break;
    case 32:
        print_esc(S(halign));
        break;
    case 36:
        print_esc(S(hrule));
        break;
    case 39:
        if (chr_code == 0)
            print_esc(S(ignorespaces));
        else
            print_esc(S(primitive));
        break;
    case 37:
        print_esc(S(insert));
        break;
    case 44:
        print_esc(47 /*"/" */ );
        break;
    case 18:
        {
            print_esc(S(mark));
            if (chr_code > 0)
                print_char(115 /*"s" */ );
        }
        break;
    case 46:
        if (chr_code == 1)
            print_esc(S(Umathaccent));
        else
            print_esc(S(mathaccent));
        break;
    case 17:
        if (chr_code == 2)
            print_esc(S(Umathchar));
        else if (chr_code == 1)
            print_esc(S(Umathcharnum));
        else
            print_esc(S(mathchar));
        break;
    case 54:
        print_esc(S(mathchoice));
        break;
    case 93:
        print_esc(S(multiply));
        break;
    case 34:
        print_esc(S(noalign));
        break;
    case 65:
        print_esc(S(noboundary));
        break;
    case 105:
        if (chr_code == 0)
            print_esc(S(noexpand));
        else
            print_esc(S(primitive));
        break;
    case 55:
        print_esc(S(nonscript));
        break;
    case 63:
        print_esc(S(omit));
        break;
    case 66:
        if (chr_code == 1)
            print_esc(S(Uradical));
        else
            print_esc(S(radical));
        break;
    case 98:
        if (chr_code == 0)
            print_esc(S(read));
        else
            print_esc(S(readline));
        break;
    case 0:
        print_esc(S(relax));
        break;
    case 100:
        print_esc(S(setbox));
        break;
    case 81:
        print_esc(S(prevgraf));
        break;
    case 85:
        switch (chr_code) {
        case 2252771:
            print_esc(S(parshape));
            break;
        case 2253039:
            print_esc(S(interlinepenalties));
            break;
        case 2253040:
            print_esc(S(clubpenalties));
            break;
        case 2253041:
            print_esc(S(widowpenalties));
            break;
        case 2253042:
            print_esc(S(displaywidowpenalties));
            break;
        }
        break;
    case 111:
        if (chr_code == 0)
            print_esc(S(the));
        else if (chr_code == 1)
            print_esc(S(unexpanded));
        else
            print_esc(S(detokenize));
        break;
    case 72:
        {
            print_esc(S(toks));
            if (chr_code != 0)
                print_sa_num(chr_code);
        }
        break;
    case 38:
        print_esc(S(vadjust));
        break;
    case 33:
        if (chr_code == 0)
            print_esc(S(valign));
        else
            switch (chr_code) {
            case 6:
                print_esc(S(beginL));
                break;
            case 7:
                print_esc(S(endL));
                break;
            case 10:
                print_esc(S(beginR));
                break;
            default:
                print_esc(S(endR));
                break;
            }
        break;
    case 56:
        print_esc(S(vcenter));
        break;
    case 35:
        print_esc(S(vrule));
        break;
    case 13:
        print_esc(S(par));
        break;
    case INPUT:
        if (chr_code == 0)
            print_esc(S(input));
        else if (chr_code == 2)
            print_esc(S(scantokens));
        else
            print_esc(S(endinput));
        break;
    case 112:
        {
            switch ((chr_code % 5)) {
            case 1:
                print_esc(S(firstmark));
                break;
            case 2:
                print_esc(S(botmark));
                break;
            case 3:
                print_esc(S(splitfirstmark));
                break;
            case 4:
                print_esc(S(splitbotmark));
                break;
            default:
                print_esc(S(topmark));
                break;
            }
            if (chr_code >= 5)
                print_char(115 /*"s" */ );
        }
        break;
    case 91:
        {
            if (chr_code < 0 || chr_code > 19)
                cmd = (mem[chr_code].hh.u.B0 / 64);
            else {
                cmd = chr_code;
                chr_code = MIN_HALFWORD;
            }
            if (cmd == INT_VAL)
                print_esc(S(count));
            else if (cmd == DIMEN_VAL)
                print_esc(S(dimen));
            else if (cmd == GLUE_VAL)
                print_esc(S(skip));
            else
                print_esc(S(muskip));
            if (chr_code != MIN_HALFWORD)
                print_sa_num(chr_code);
        }
        break;
    case 80:
        if (chr_code == VMODE)
            print_esc(S(prevdepth));
        else
            print_esc(S(spacefactor));
        break;
    case 83:
        if (chr_code == 0)
            print_esc(S(deadcycles));
        else if (chr_code == 2)
            print_esc(S(interactionmode));
        else
            print_esc(S(insertpenalties));
        break;
    case 84:
        if (chr_code == WIDTH_OFFSET)
            print_esc(S(wd));
        else if (chr_code == HEIGHT_OFFSET)
            print_esc(S(ht));
        else
            print_esc(S(dp));
        break;
    case 71:
        switch (chr_code) {
        case 0:
            print_esc(S(lastpenalty));
            break;
        case 1:
            print_esc(S(lastkern));
            break;
        case 2:
            print_esc(S(lastskip));
            break;
        case 4:
            print_esc(S(inputlineno));
            break;
        case 45:
            print_esc(S(shellescape));
            break;
        case 3:
            print_esc(S(lastnodetype));
            break;
        case 6:
            print_esc(S(eTeXversion));
            break;
        case 14:
            print_esc(S(XeTeXversion));
            break;
        case 15:
            print_esc(S(XeTeXcountglyphs));
            break;
        case 16:
            print_esc(S(XeTeXcountvariations));
            break;
        case 17:
            print_esc(S(XeTeXvariation));
            break;
        case 18:
            print_esc(S(XeTeXfindvariationbyname));
            break;
        case 19:
            print_esc(S(XeTeXvariationmin));
            break;
        case 20:
            print_esc(S(XeTeXvariationmax));
            break;
        case 21:
            print_esc(S(XeTeXvariationdefault));
            break;
        case 22:
            print_esc(S(XeTeXcountfeatures));
            break;
        case 23:
            print_esc(S(XeTeXfeaturecode));
            break;
        case 24:
            print_esc(S(XeTeXfindfeaturebyname));
            break;
        case 25:
            print_esc(S(XeTeXisexclusivefeature));
            break;
        case 26:
            print_esc(S(XeTeXcountselectors));
            break;
        case 27:
            print_esc(S(XeTeXselectorcode));
            break;
        case 28:
            print_esc(S(XeTeXfindselectorbyname));
            break;
        case 29:
            print_esc(S(XeTeXisdefaultselector));
            break;
        case 30:
            print_esc(S(XeTeXOTcountscripts));
            break;
        case 31:
            print_esc(S(XeTeXOTcountlanguages));
            break;
        case 32:
            print_esc(S(XeTeXOTcountfeatures));
            break;
        case 33:
            print_esc(S(XeTeXOTscripttag));
            break;
        case 34:
            print_esc(S(XeTeXOTlanguagetag));
            break;
        case 35:
            print_esc(S(XeTeXOTfeaturetag));
            break;
        case 36:
            print_esc(S(XeTeXcharglyph));
            break;
        case 37:
            print_esc(S(XeTeXglyphindex));
            break;
        case 47:
            print_esc(S(XeTeXglyphbounds));
            break;
        case 38:
            print_esc(S(XeTeXfonttype));
            break;
        case 39:
            print_esc(S(XeTeXfirstfontchar));
            break;
        case 40:
            print_esc(S(XeTeXlastfontchar));
            break;
        case 41:
            print_esc(S(pdflastxpos));
            break;
        case 42:
            print_esc(S(pdflastypos));
            break;
        case 46:
            print_esc(S(XeTeXpdfpagecount));
            break;
        case 7:
            print_esc(S(currentgrouplevel));
            break;
        case 8:
            print_esc(S(currentgrouptype));
            break;
        case 9:
            print_esc(S(currentiflevel));
            break;
        case 10:
            print_esc(S(currentiftype));
            break;
        case 11:
            print_esc(S(currentifbranch));
            break;
        case 48:
            print_esc(S(fontcharwd));
            break;
        case 49:
            print_esc(S(fontcharht));
            break;
        case 50:
            print_esc(S(fontchardp));
            break;
        case 51:
            print_esc(S(fontcharic));
            break;
        case 52:
            print_esc(S(parshapelength));
            break;
        case 53:
            print_esc(S(parshapeindent));
            break;
        case 54:
            print_esc(S(parshapedimen));
            break;
        case 59:
            print_esc(S(numexpr));
            break;
        case 60:
            print_esc(S(dimexpr));
            break;
        case 61:
            print_esc(S(glueexpr));
            break;
        case 62:
            print_esc(S(muexpr));
            break;
        case 12:
            print_esc(S(gluestretchorder));
            break;
        case 13:
            print_esc(S(glueshrinkorder));
            break;
        case 55:
            print_esc(S(gluestretch));
            break;
        case 56:
            print_esc(S(glueshrink));
            break;
        case 57:
            print_esc(S(mutoglue));
            break;
        case 58:
            print_esc(S(gluetomu));
            break;
        default:
            print_esc(S(badness));
            break;
        }
        break;
    case 110:
        switch (chr_code) {
        case 0:
            print_esc(S(number));
            break;
        case 1:
            print_esc(S(romannumeral));
            break;
        case 2:
            print_esc(S(string));
            break;
        case 3:
            print_esc(S(meaning));
            break;
        case 4:
            print_esc(S(fontname));
            break;
        case 43:
            print_esc(S(strcmp));
            break;
        case 44:
            print_esc(S(mdfivesum));
            break;
        case 11:
            print_esc(S(leftmarginkern));
            break;
        case 12:
            print_esc(S(rightmarginkern));
            break;
        case 5:
            print_esc(S(eTeXrevision));
            break;
        case 6:
            print_esc(S(XeTeXrevision));
            break;
        case 7:
            print_esc(S(XeTeXvariationname));
            break;
        case 8:
            print_esc(S(XeTeXfeaturename));
            break;
        case 9:
            print_esc(S(XeTeXselectorname));
            break;
        case 10:
            print_esc(S(XeTeXglyphname));
            break;
        case 13:
            print_esc(S(Uchar));
            break;
        case 14:
            print_esc(S(Ucharcat));
            break;
        default:
            print_esc(S(jobname));
            break;
        }
        break;
    case 107:
        {
            if (chr_code >= UNLESS_CODE)
                print_esc(S(unless));
            switch (chr_code % UNLESS_CODE) {
            case 1:
                print_esc(S(ifcat));
                break;
            case 2:
                print_esc(S(ifnum));
                break;
            case 3:
                print_esc(S(ifdim));
                break;
            case 4:
                print_esc(S(ifodd));
                break;
            case 5:
                print_esc(S(ifvmode));
                break;
            case 6:
                print_esc(S(ifhmode));
                break;
            case 7:
                print_esc(S(ifmmode));
                break;
            case 8:
                print_esc(S(ifinner));
                break;
            case 9:
                print_esc(S(ifvoid));
                break;
            case 10:
                print_esc(S(ifhbox));
                break;
            case 11:
                print_esc(S(ifvbox));
                break;
            case 12:
                print_esc(S(ifx));
                break;
            case 13:
                print_esc(S(ifeof));
                break;
            case 14:
                print_esc(S(iftrue));
                break;
            case 15:
                print_esc(S(iffalse));
                break;
            case 16:
                print_esc(S(ifcase));
                break;
            case 21:
                print_esc(S(ifprimitive));
                break;
            case 17:
                print_esc(S(ifdefined));
                break;
            case 18:
                print_esc(S(ifcsname));
                break;
            case 19:
                print_esc(S(iffontchar));
                break;
            case 20:
                print_esc(S(ifincsname));
                break;
            default:
                print_esc(S(if));
                break;
            }
        }
        break;
    case 108:
        if (chr_code == FI_CODE)
            print_esc(S(fi));
        else if (chr_code == OR_CODE)
            print_esc(S(or));
        else
            print_esc(S(else));
        break;
    case 4:
        if (chr_code == SPAN_CODE)
            print_esc(S(span));
        else {

            print(S(alignment_tab_character_));
            if (chr_code < 65536L)
                print(chr_code);
            else
                print_char(chr_code);
        }
        break;
    case 5:
        if (chr_code == CR_CODE)
            print_esc(S(cr));
        else
            print_esc(S(crcr));
        break;
    case 82:
        switch (chr_code) {
        case 0:
            print_esc(S(pagegoal));
            break;
        case 1:
            print_esc(S(pagetotal));
            break;
        case 2:
            print_esc(S(pagestretch));
            break;
        case 3:
            print_esc(S(pagefilstretch));
            break;
        case 4:
            print_esc(S(pagefillstretch));
            break;
        case 5:
            print_esc(S(pagefilllstretch));
            break;
        case 6:
            print_esc(S(pageshrink));
            break;
        default:
            print_esc(S(pagedepth));
            break;
        }
        break;
    case 14:
        if (chr_code == 1)
            print_esc(S(dump));
        else
            print_esc(S(end));
        break;
    case 26:
        switch (chr_code) {
        case 4:
            print_esc(S(hskip));
            break;
        case 0:
            print_esc(S(hfil));
            break;
        case 1:
            print_esc(S(hfill));
            break;
        case 2:
            print_esc(S(hss));
            break;
        default:
            print_esc(S(hfilneg));
            break;
        }
        break;
    case 27:
        switch (chr_code) {
        case 4:
            print_esc(S(vskip));
            break;
        case 0:
            print_esc(S(vfil));
            break;
        case 1:
            print_esc(S(vfill));
            break;
        case 2:
            print_esc(S(vss));
            break;
        default:
            print_esc(S(vfilneg));
            break;
        }
        break;
    case 28:
        print_esc(S(mskip));
        break;
    case 29:
        print_esc(S(kern));
        break;
    case 30:
        print_esc(S(mkern));
        break;
    case 21:
        if (chr_code == 1)
            print_esc(S(moveleft));
        else
            print_esc(S(moveright));
        break;
    case 22:
        if (chr_code == 1)
            print_esc(S(raise));
        else
            print_esc(S(lower));
        break;
    case 20:
        switch (chr_code) {
        case 0:
            print_esc(S(box));
            break;
        case 1:
            print_esc(S(copy));
            break;
        case 2:
            print_esc(S(lastbox));
            break;
        case 3:
            print_esc(S(vsplit));
            break;
        case 4:
            print_esc(S(vtop));
            break;
        case 5:
            print_esc(S(vbox));
            break;
        default:
            print_esc(S(hbox));
            break;
        }
        break;
    case 31:
        if (chr_code == A_LEADERS)
            print_esc(S(leaders));
        else if (chr_code == C_LEADERS)
            print_esc(S(cleaders));
        else if (chr_code == X_LEADERS)
            print_esc(S(xleaders));
        else
            print_esc(S(shipout));
        break;
    case 43:
        if (chr_code == 0)
            print_esc(S(noindent));
        else
            print_esc(S(indent));
        break;
    case 25:
        if (chr_code == GLUE_NODE)
            print_esc(S(unskip));
        else if (chr_code == KERN_NODE)
            print_esc(S(unkern));
        else
            print_esc(S(unpenalty));
        break;
    case 23:
        if (chr_code == COPY_CODE)
            print_esc(S(unhcopy));
        else
            print_esc(S(unhbox));
        break;
    case 24:
        if (chr_code == COPY_CODE)
            print_esc(S(unvcopy));
        else if (chr_code == LAST_BOX_CODE)
            print_esc(S(pagediscards));
        else if (chr_code == VSPLIT_CODE)
            print_esc(S(splitdiscards));
/*:
1650*/
        else
            print_esc(S(unvbox));
        break;
    case 47:
        if (chr_code == 1)
            print_esc(45 /*"-" */ );
        else
            print_esc(S(discretionary));
        break;
    case 48:
        if (chr_code == 1)
            print_esc(S(leqno));
        else
            print_esc(S(eqno));
        break;
    case 50:
        switch (chr_code) {
        case 16:
            print_esc(S(mathord));
            break;
        case 17:
            print_esc(S(mathop));
            break;
        case 18:
            print_esc(S(mathbin));
            break;
        case 19:
            print_esc(S(mathrel));
            break;
        case 20:
            print_esc(S(mathopen));
            break;
        case 21:
            print_esc(S(mathclose));
            break;
        case 22:
            print_esc(S(mathpunct));
            break;
        case 23:
            print_esc(S(mathinner));
            break;
        case 26:
            print_esc(S(underline));
            break;
        default:
            print_esc(S(overline));
            break;
        }
        break;
    case 51:
        if (chr_code == LIMITS)
            print_esc(S(limits));
        else if (chr_code == NO_LIMITS)
            print_esc(S(nolimits));
        else
            print_esc(S(displaylimits));
        break;
    case 53:
        print_style(chr_code);
        break;
    case 52:
        switch (chr_code) {
        case 1:
            print_esc(S(over));
            break;
        case 2:
            print_esc(S(atop));
            break;
        case 3:
            print_esc(S(abovewithdelims));
            break;
        case 4:
            print_esc(S(overwithdelims));
            break;
        case 5:
            print_esc(S(atopwithdelims));
            break;
        default:
            print_esc(S(above));
            break;
        }
        break;
    case 49:
        if (chr_code == LEFT_NOAD)
            print_esc(S(left));
        else if (chr_code == 1)
            print_esc(S(middle));
        else
            print_esc(S(right));
        break;
    case 95:
        if (chr_code == 1)
            print_esc(S(long));
        else if (chr_code == 2)
            print_esc(S(outer));
        else if (chr_code == 8)
            print_esc(S(protected));
        else
            print_esc(S(global));
        break;
    case 99:
        if (chr_code == 0)
            print_esc(S(def));
        else if (chr_code == 1)
            print_esc(S(gdef));
        else if (chr_code == 2)
            print_esc(S(edef));
        else
            print_esc(S(xdef));
        break;
    case 96:
        if (chr_code != NORMAL)
            print_esc(S(futurelet));
        else
            print_esc(S(let));
        break;
    case 97:
        switch (chr_code) {
        case 0:
            print_esc(S(chardef));
            break;
        case 1:
            print_esc(S(mathchardef));
            break;
        case 9:
            print_esc(S(Umathchardef));
            break;
        case 8:
            print_esc(S(Umathcharnumdef));
            break;
        case 2:
            print_esc(S(countdef));
            break;
        case 3:
            print_esc(S(dimendef));
            break;
        case 4:
            print_esc(S(skipdef));
            break;
        case 5:
            print_esc(S(muskipdef));
            break;
        case 7:
            print_esc(S(charsubdef));
            break;
        default:
            print_esc(S(toksdef));
            break;
        }
        break;
    case 68:
        {
            print_esc(S(char));
            print_hex(chr_code);
        }
        break;
    case 69:
        {
            print_esc(S(mathchar));
            print_hex(chr_code);
        }
        break;
    case 70:
        {
            print_esc(S(Umathchar));
            print_hex(math_class(chr_code));
            print_hex(math_fam(chr_code));
            print_hex(math_char(chr_code));
        }
        break;
    case 86:
        if (chr_code == CAT_CODE_BASE)
            print_esc(S(catcode));
        else if (chr_code == MATH_CODE_BASE)
            print_esc(S(mathcode));
        else if (chr_code == LC_CODE_BASE)
            print_esc(S(lccode));
        else if (chr_code == UC_CODE_BASE)
            print_esc(S(uccode));
        else if (chr_code == SF_CODE_BASE)
            print_esc(S(sfcode));
        else
            print_esc(S(delcode));
        break;
    case 87:
        if (chr_code == SF_CODE_BASE)
            print_esc(S(XeTeXcharclass));
        else if (chr_code == MATH_CODE_BASE)
            print_esc(S(Umathcodenum));
        else if (chr_code == (MATH_CODE_BASE + 1))
            print_esc(S(Umathcode));
        else if (chr_code == DEL_CODE_BASE)
            print_esc(S(Udelcodenum));
        else
            print_esc(S(Udelcode));
        break;
    case 88:
        print_size(chr_code - 2253300L);
        break;
    case 101:
        if (chr_code == 1)
            print_esc(S(patterns));
        else
            print_esc(S(hyphenation));
        break;
    case 79:
        switch (chr_code) {
        case 0:
            print_esc(S(hyphenchar));
            break;
        case 1:
            print_esc(S(skewchar));
            break;
        case 2:
            print_esc(S(lpcode));
            break;
        case 3:
            print_esc(S(rpcode));
            break;
        }
        break;
    case 89:
        {
            print(S(select_font_));
            font_name_str = font_name[chr_code];
            if (((font_area[chr_code] == AAT_FONT_FLAG)
                 || (font_area[chr_code] == OTGR_FONT_FLAG))) {
                quote_char = 34 /*""" */ ;
                {
                    register integer for_end;
                    n = 0;
                    for_end = length(font_name_str) - 1;
                    if (n <= for_end)
                        do
                            if (str_pool[str_start[(font_name_str) - 65536L] + n] == 34 /*""" */ )
                                quote_char = 39 /*"'" */ ;
                        while (n++ < for_end) ;
                }
                print_char(quote_char);
                print(font_name_str);
                print_char(quote_char);
            } else
                print(font_name_str);
            if (font_size[chr_code] != font_dsize[chr_code]) {
                print(S(_at_));
                print_scaled(font_size[chr_code]);
                print(S(pt));
            }
        }
        break;
    case 102:
        switch (chr_code) {
        case 0:
            print_esc(S(batchmode));
            break;
        case 1:
            print_esc(S(nonstopmode));
            break;
        case 2:
            print_esc(S(scrollmode));
            break;
        default:
            print_esc(S(errorstopmode));
            break;
        }
        break;
    case 60:
        if (chr_code == 0)
            print_esc(S(closein));
        else
            print_esc(S(openin));
        break;
    case 58:
        if (chr_code == 0)
            print_esc(S(message));
        else
            print_esc(S(errmessage));
        break;
    case 57:
        if (chr_code == LC_CODE_BASE)
            print_esc(S(lowercase));
        else
            print_esc(S(uppercase));
        break;
    case 19:
        switch (chr_code) {
        case 1:
            print_esc(S(showbox));
            break;
        case 2:
            print_esc(S(showthe));
            break;
        case 3:
            print_esc(S(showlists));
            break;
        case 4:
            print_esc(S(showgroups));
            break;
        case 5:
            print_esc(S(showtokens));
            break;
        case 6:
            print_esc(S(showifs));
            break;
        default:
            print_esc(S(show));
            break;
        }
        break;
    case 103:
        print(S(undefined));
        break;
    case 113:
    case 114:
    case 115:
    case 116:
        {
            n = cmd - 113;
            if (mem[mem[chr_code].hh.v.RH].hh.v.LH == PROTECTED_TOKEN)
                n = n + 4;
            if (odd(n / 4))
                print_esc(S(protected));
            if (odd(n))
                print_esc(S(long));
            if (odd(n / 2))
                print_esc(S(outer));
            if (n > 0)
                print_char(32 /*" " */ );
            print(S(macro));
        }
        break;
    case 117:
        print_esc(S(outer_endtemplate));
        break;
    case 59:
        switch (chr_code) {
        case 0:
            print_esc(S(openout));
            break;
        case 1:
            print_esc(S(write));
            break;
        case 2:
            print_esc(S(closeout));
            break;
        case 3:
            print_esc(S(special));
            break;
        case 4:
            print_esc(S(immediate));
            break;
        case 5:
            print_esc(S(setlanguage));
            break;
        case 41:
            print_esc(S(XeTeXpicfile));
            break;
        case 42:
            print_esc(S(XeTeXpdffile));
            break;
        case 43:
            print_esc(S(XeTeXglyph));
            break;
        case 46:
            print_esc(S(XeTeXlinebreaklocale));
            break;
        case 44:
            print_esc(S(XeTeXinputencoding));
            break;
        case 45:
            print_esc(S(XeTeXdefaultencoding));
            break;
        case 6:
            print_esc(S(pdfsavepos));
            break;
        default:
            print(S(_unknown_extension__));
            break;
        }
        break;
    default:
        print(S(_unknown_command_code__));
        break;
    }
}

void not_aat_font_error(integer cmd, integer c, integer f)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Cannot_use_));
    }
    print_cmd_chr(cmd, c);
    print(S(_with_));
    print(font_name[f]);
    print(S(__not_an_AAT_font));
    error();
}

void not_aat_gr_font_error(integer cmd, integer c, integer f)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Cannot_use_));
    }
    print_cmd_chr(cmd, c);
    print(S(_with_));
    print(font_name[f]);
    print(S(__not_an_AAT_or_Graphite_fon/*t*/));
    error();
}

void not_ot_font_error(integer cmd, integer c, integer f)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Cannot_use_));
    }
    print_cmd_chr(cmd, c);
    print(S(_with_));
    print(font_name[f]);
    print(S(__not_an_OpenType_Layout_fon/*t*/));
    error();
}

void not_native_font_error(integer cmd, integer c, integer f)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Cannot_use_));
    }
    print_cmd_chr(cmd, c);
    print(S(_with_));
    print(font_name[f]);
    print(S(__not_a_native_platform_font/**/));
    error();
}

/*:1434*/

int32_t id_lookup(integer j, integer l)
{
    register int32_t Result;
    integer h;
    integer d;
    int32_t p;
    int32_t k;
    integer ll;
    h = 0;
    {
        register integer for_end;
        k = j;
        for_end = j + l - 1;
        if (k <= for_end)
            do {
                h = h + h + buffer[k];
                while (h >= HASH_PRIME)
                    h = h - 8501;
            }
            while (k++ < for_end);
    }
    p = h + 2228226L;
    ll = l;
    {
        register integer for_end;
        d = 0;
        for_end = l - 1;
        if (d <= for_end)
            do
                if (buffer[j + d] >= 65536L)
                    ll++;
            while (d++ < for_end) ;
    }
    while (true) {

        if (hash[p].v.RH > 0) {

            if (length(hash[p].v.RH) == ll) {

                if (str_eq_buf(hash[p].v.RH, j))
                    goto lab40;
            }
        }
        if (hash[p].v.LH == 0) {
            if (no_new_control_sequence)
                p = UNDEFINED_CONTROL_SEQUENCE;
            else {              /*269: */

                if (hash[p].v.RH > 0) {
                    if (hash_high < hash_extra) {
                        hash_high++;
                        hash[p].v.LH = hash_high + 10053470L;
                        p = hash_high + 10053470L;
                    } else {

                        do {
                            if ((hash_used == HASH_BASE))
                                overflow(S(hash_size), HASH_SIZE + hash_extra);
                            hash_used--;
                        } while (!(hash[hash_used].v.RH == 0));
                        hash[p].v.LH = hash_used;
                        p = hash_used;
                    }
                }
                {
                    if (pool_ptr + ll > pool_size)
                        overflow(S(pool_size), pool_size - init_pool_ptr);
                }
                d = (pool_ptr - str_start[(str_ptr) - 65536L]);
                while (pool_ptr > str_start[(str_ptr) - 65536L]) {

                    pool_ptr--;
                    str_pool[pool_ptr + l] = str_pool[pool_ptr];
                }
                {
                    register integer for_end;
                    k = j;
                    for_end = j + l - 1;
                    if (k <= for_end)
                        do {
                            if (buffer[k] < 65536L) {
                                str_pool[pool_ptr] = buffer[k];
                                pool_ptr++;
                            } else {

                                {
                                    str_pool[pool_ptr] = 55296L + (buffer[k] - 65536L) / 1024;
                                    pool_ptr++;
                                }
                                {
                                    str_pool[pool_ptr] = 56320L + (buffer[k] - 65536L) % 1024;
                                    pool_ptr++;
                                }
                            }
                        }
                        while (k++ < for_end);
                }
                hash[p].v.RH = make_string();
                pool_ptr = pool_ptr + d;
            }
            goto lab40;
        }
        p = hash[p].v.LH;
    }
 lab40:                        /*found */ Result = p;
    return Result;
}

int32_t prim_lookup(str_number s)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    integer h;
    int32_t p;
    int32_t k;
    integer j, l;

    if (s < 256) {
        p = s;
        if ((p < 0) || (prim_eqtb[p].hh.u.B1 != LEVEL_ONE))
            p = UNDEFINED_PRIMITIVE;
    } else {

        j = str_start[(s) - 65536L];
        if (s == str_ptr)
            l = (pool_ptr - str_start[(str_ptr) - 65536L]);
        else
            l = length(s);
        h = str_pool[j];
        {
            register integer for_end;
            k = j + 1;
            for_end = j + l - 1;
            if (k <= for_end)
                do {
                    h = h + h + str_pool[k];
                    while (h >= PRIM_PRIME)
                        h = h - 431;
                }
                while (k++ < for_end);
        }
        p = h + 1;
        while (true) {

            if (prim[p].v.RH > 0) {

                if (length(prim[p].v.RH) == l) {

                    if (str_eq_str(prim[p].v.RH, s))
                        goto lab40;
                }
            }
            if (prim[p].v.LH == 0) {
                if (no_new_control_sequence)
                    p = UNDEFINED_PRIMITIVE;
                else {          /*272: */

                    if (prim[p].v.RH > 0) {
                        do {
                            if ((prim_used == PRIM_BASE))
                                overflow(S(primitive_size), PRIM_SIZE);
                            prim_used--;
                        } while (!(prim[prim_used].v.RH == 0));
                        prim[p].v.LH = prim_used;
                        p = prim_used;
                    }
                    prim[p].v.RH = s;
                }
                goto lab40;
            }
            p = prim[p].v.LH;
        }
    }

lab40: /*found */
    Result = p;
    return Result;
}

/*:276*//*280: *//*296: */

void print_group(boolean e)
{
    switch (cur_group) {
    case 0:
        {
            print(S(bottom_level));
            return;
        }
        break;
    case 1:
    case 14:
        {
            if (cur_group == SEMI_SIMPLE_GROUP)
                print(S(semi_));
            print(S(simple));
        }
        break;
    case 2:
    case 3:
        {
            if (cur_group == ADJUSTED_HBOX_GROUP)
                print(S(adjusted_));
            print(S(hbox));
        }
        break;
    case 4:
        print(S(vbox));
        break;
    case 5:
        print(S(vtop));
        break;
    case 6:
    case 7:
        {
            if (cur_group == NO_ALIGN_GROUP)
                print(S(no_));
            print(S(align));
        }
        break;
    case 8:
        print(S(output));
        break;
    case 10:
        print(S(disc));
        break;
    case 11:
        print(S(insert));
        break;
    case 12:
        print(S(vcenter));
        break;
    case 9:
    case 13:
    case 15:
    case 16:
        {
            print(S(math));
            if (cur_group == MATH_CHOICE_GROUP)
                print(S(_choice));
            else if (cur_group == MATH_SHIFT_GROUP)
                print(S(_shift));
            else if (cur_group == MATH_LEFT_GROUP)
                print(S(_left));
        }
        break;
    }
    print(S(_group__level_));
    print_int(cur_level);
    print_char(41 /*")" */ );
    if (save_stack[save_ptr - 1].cint != 0) {
        if (e)
            print(S(_entered_at_line_));
        else
            print(S(_at_line_));
        print_int(save_stack[save_ptr - 1].cint);
    }
}

/*:1448*//*1449: */

boolean pseudo_input(void)
{
    register boolean Result;
    memory_word *mem = zmem; int32_t p;
    integer sz;
    four_quarters w;
    int32_t r;
    last = first;
    p = mem[pseudo_files].hh.v.LH;
    if (p == MIN_HALFWORD)
        Result = false;
    else {

        mem[pseudo_files].hh.v.LH = mem[p].hh.v.RH;
        sz = mem[p].hh.v.LH;
        if (4 * sz - 3 >= buf_size - last) {    /*35: */
            cur_input.loc = first;
            cur_input.limit = last - 1;
            overflow(S(buffer_size), buf_size);
        }
        last = first;
        {
            register integer for_end;
            r = p + 1;
            for_end = p + sz - 1;
            if (r <= for_end)
                do {
                    w = mem[r].qqqq;
                    buffer[last] = w.u.B0;
                    buffer[last + 1] = w.u.B1;
                    buffer[last + 2] = w.u.B2;
                    buffer[last + 3] = w.u.B3;
                    last = last + 4;
                }
                while (r++ < for_end);
        }
        if (last >= max_buf_stack)
            max_buf_stack = last + 1;
        while ((last > first) && (buffer[last - 1] == 32 /*" " */ ))
            last--;
        free_node(p, sz);
        Result = true;
    }
    return Result;
}

void pseudo_close(void)
{
    memory_word *mem = zmem; int32_t p, q;
    p = mem[pseudo_files].hh.v.RH;
    q = mem[pseudo_files].hh.v.LH;
    {
        mem[pseudo_files].hh.v.RH = avail;
        avail = pseudo_files;
    }
    pseudo_files = p;
    while (q != MIN_HALFWORD) {

        p = q;
        q = mem[p].hh.v.RH;
        free_node(p, mem[p].hh.v.LH);
    }
}

void group_warning(void)
{
    CACHE_THE_EQTB;
    integer i;
    boolean w;

    base_ptr = input_ptr;
    input_stack[base_ptr] = cur_input;
    i = in_open;
    w = false;
    while ((grp_stack[i] == cur_boundary) && (i > 0)) {

        if (INTPAR(tracing_nesting) > 0) {
            while ((input_stack[base_ptr].state == TOKEN_LIST) || (input_stack[base_ptr].index > i))
                base_ptr--;
            if (input_stack[base_ptr].name > 17)
                w = true;
        }
        grp_stack[i] = save_stack[save_ptr].hh.v.RH;
        i--;
    }
    if (w) {
        print_nl(S(Warning__end_of_));
        print_group(true);
        print(S(_of_a_different_file));
        print_ln();
        if (INTPAR(tracing_nesting) > 1)
            show_context();
        if (history == HISTORY_SPOTLESS)
            history = HISTORY_WARNING_ISSUED;
    }
}

void if_warning(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer i;
    boolean w;

    base_ptr = input_ptr;
    input_stack[base_ptr] = cur_input;
    i = in_open;
    w = false;
    while (if_stack[i] == cond_ptr) {

        if (INTPAR(tracing_nesting) > 0) {
            while ((input_stack[base_ptr].state == TOKEN_LIST) || (input_stack[base_ptr].index > i))
                base_ptr--;
            if (input_stack[base_ptr].name > 17)
                w = true;
        }
        if_stack[i] = mem[cond_ptr].hh.v.RH;
        i--;
    }
    if (w) {
        print_nl(S(Warning__end_of_));
        print_cmd_chr(IF_TEST, cur_if);
        if (if_line != 0) {
            print(S(_entered_on_line_));
            print_int(if_line);
        }
        print(S(_of_a_different_file));
        print_ln();
        if (INTPAR(tracing_nesting) > 1)
            show_context();
        if (history == HISTORY_SPOTLESS)
            history = HISTORY_WARNING_ISSUED;
    }
}

void file_warning(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;
    uint16_t l;
    uint16_t c;
    integer i;

    p = save_ptr;
    l = cur_level;
    c = cur_group;
    save_ptr = cur_boundary;
    while (grp_stack[in_open] != save_ptr) {

        cur_level--;
        print_nl(S(Warning__end_of_file_when_));
        print_group(true);
        print(S(_is_incomplete));
        cur_group = save_stack[save_ptr].hh.u.B1;
        save_ptr = save_stack[save_ptr].hh.v.RH;
    }
    save_ptr = p;
    cur_level = l;
    cur_group = c;
    p = cond_ptr;
    l = if_limit;
    c = cur_if;
    i = if_line;
    while (if_stack[in_open] != cond_ptr) {

        print_nl(S(Warning__end_of_file_when_));
        print_cmd_chr(IF_TEST, cur_if);
        if (if_limit == FI_CODE)
            print_esc(S(else));
        if (if_line != 0) {
            print(S(_entered_on_line_));
            print_int(if_line);
        }
        print(S(_is_incomplete));
        if_line = mem[cond_ptr + 1].cint;
        cur_if = mem[cond_ptr].hh.u.B1;
        if_limit = mem[cond_ptr].hh.u.B0;
        cond_ptr = mem[cond_ptr].hh.v.RH;
    }
    cond_ptr = p;
    if_limit = l;
    cur_if = c;
    if_line = i;
    print_ln();
    if (INTPAR(tracing_nesting) > 1)
        show_context();
    if (history == HISTORY_SPOTLESS)
        history = HISTORY_WARNING_ISSUED;
}

void delete_sa_ref(int32_t q)
{
    memory_word *mem = zmem; int32_t p;
    small_number i;
    small_number s;
    mem[q + 1].hh.v.LH--;
    if (mem[q + 1].hh.v.LH != MIN_HALFWORD)
        return;
    if (mem[q].hh.u.B0 < DIMEN_VAL_LIMIT) {

        if (mem[q + 2].cint == 0)
            s = WORD_NODE_SIZE;
        else
            return;
    } else {

        if (mem[q].hh.u.B0 < MU_VAL_LIMIT) {

            if (mem[q + 1].hh.v.RH == 0)
                delete_glue_ref(0);
            else
                return;
        } else if (mem[q + 1].hh.v.RH != MIN_HALFWORD)
            return;
        s = POINTER_NODE_SIZE;
    }
    do {
        i = mem[q].hh.u.B0 % 64;
        p = q;
        q = mem[p].hh.v.RH;
        free_node(p, s);
        if (q == MIN_HALFWORD) {
            sa_root[i] = MIN_HALFWORD;
            return;
        }
        {
            if (odd(i))
                mem[q + (i / 2) + 1].hh.v.RH = MIN_HALFWORD;
            else
                mem[q + (i / 2) + 1].hh.v.LH = MIN_HALFWORD;
            mem[q].hh.u.B1--;
        }
        s = INDEX_NODE_SIZE;
    } while (!(mem[q].hh.u.B1 > 0));
}

/*:1609*//*1611: */

void sa_save(int32_t p)
{
    memory_word *mem = zmem; int32_t q;
    uint16_t i;
    if (cur_level != sa_level) {
        if (save_ptr > max_save_stack) {
            max_save_stack = save_ptr;
            if (max_save_stack > save_size - 7)
                overflow(S(save_size), save_size);
        }
        save_stack[save_ptr].hh.u.B0 = RESTORE_SA;
        save_stack[save_ptr].hh.u.B1 = sa_level;
        save_stack[save_ptr].hh.v.RH = sa_chain;
        save_ptr++;
        sa_chain = MIN_HALFWORD;
        sa_level = cur_level;
    }
    i = mem[p].hh.u.B0;
    if (i < DIMEN_VAL_LIMIT) {
        if (mem[p + 2].cint == 0) {
            q = get_node(POINTER_NODE_SIZE);
            i = TOK_VAL_LIMIT;
        } else {

            q = get_node(WORD_NODE_SIZE);
            mem[q + 2].cint = mem[p + 2].cint;
        }
        mem[q + 1].hh.v.RH = MIN_HALFWORD;
    } else {

        q = get_node(POINTER_NODE_SIZE);
        mem[q + 1].hh.v.RH = mem[p + 1].hh.v.RH;
    }
    mem[q + 1].hh.v.LH = p;
    mem[q].hh.u.B0 = i;
    mem[q].hh.u.B1 = mem[p].hh.u.B1;
    mem[q].hh.v.RH = sa_chain;
    sa_chain = q;
    mem[p + 1].hh.v.LH++;
}

void sa_destroy(int32_t p)
{
    memory_word *mem = zmem; if (mem[p].hh.u.B0 < MU_VAL_LIMIT)
        delete_glue_ref(mem[p + 1].hh.v.RH);
    else if (mem[p + 1].hh.v.RH != MIN_HALFWORD) {

        if (mem[p].hh.u.B0 < BOX_VAL_LIMIT)
            flush_node_list(mem[p + 1].hh.v.RH);
        else
            delete_token_ref(mem[p + 1].hh.v.RH);
    }
}

void sa_def(int32_t p, int32_t e)
{
    memory_word *mem = zmem;

    mem[p + 1].hh.v.LH++;
    if (mem[p + 1].hh.v.RH == e) {
        sa_destroy(p);
    } else {
        if (mem[p].hh.u.B1 == cur_level)
            sa_destroy(p);
        else
            sa_save(p);
        mem[p].hh.u.B1 = cur_level;
        mem[p + 1].hh.v.RH = e;
    }
    delete_sa_ref(p);
}

void sa_w_def(int32_t p, integer w)
{
    memory_word *mem = zmem;

    mem[p + 1].hh.v.LH++;

    if (mem[p + 2].cint == w) {
    } else {
        if (mem[p].hh.u.B1 != cur_level)
            sa_save(p);
        mem[p].hh.u.B1 = cur_level;
        mem[p + 2].cint = w;
    }
    delete_sa_ref(p);
}

void gsa_def(int32_t p, int32_t e)
{
    memory_word *mem = zmem;

    mem[p + 1].hh.v.LH++;
    sa_destroy(p);
    mem[p].hh.u.B1 = LEVEL_ONE;
    mem[p + 1].hh.v.RH = e;
    delete_sa_ref(p);
}

void gsa_w_def(int32_t p, integer w)
{
    memory_word *mem = zmem;

    mem[p + 1].hh.v.LH++;
    mem[p].hh.u.B1 = LEVEL_ONE;
    mem[p + 2].cint = w;
    delete_sa_ref(p);
}

void sa_restore(void)
{
    memory_word *mem = zmem;
    int32_t p;

    do {
        p = mem[sa_chain + 1].hh.v.LH;
        if (mem[p].hh.u.B1 == LEVEL_ONE) {
            if (mem[p].hh.u.B0 >= DIMEN_VAL_LIMIT)
                sa_destroy(sa_chain);
        } else {

            if (mem[p].hh.u.B0 < DIMEN_VAL_LIMIT) {

                if (mem[sa_chain].hh.u.B0 < DIMEN_VAL_LIMIT)
                    mem[p + 2].cint = mem[sa_chain + 2].cint;
                else
                    mem[p + 2].cint = 0;
            } else {

                sa_destroy(p);
                mem[p + 1].hh.v.RH = mem[sa_chain + 1].hh.v.RH;
            }
            mem[p].hh.u.B1 = mem[sa_chain].hh.u.B1;
        }
        delete_sa_ref(p);
        p = sa_chain;
        sa_chain = mem[p].hh.v.RH;
        if (mem[p].hh.u.B0 < DIMEN_VAL_LIMIT)
            free_node(p, WORD_NODE_SIZE);
        else
            free_node(p, POINTER_NODE_SIZE);
    } while (!(sa_chain == MIN_HALFWORD));
}

void new_save_level(group_code c)
{
    if (save_ptr > max_save_stack) {
        max_save_stack = save_ptr;
        if (max_save_stack > save_size - 7)
            overflow(S(save_size), save_size);
    }

    save_stack[save_ptr + 0].cint = line;
    save_ptr++;
    save_stack[save_ptr].hh.u.B0 = LEVEL_BOUNDARY;
    save_stack[save_ptr].hh.u.B1 = cur_group;
    save_stack[save_ptr].hh.v.RH = cur_boundary;
    if (cur_level == UINT16_MAX)
        overflow(S(grouping_levels), UINT16_MAX);
    cur_boundary = save_ptr;
    cur_group = c;
    cur_level++;
    save_ptr++;
}

void eq_destroy(memory_word w)
{
    memory_word *mem = zmem; int32_t q;
    switch (w.hh.u.B0) {
    case 113:
    case 114:
    case 115:
    case 116:
        delete_token_ref(w.hh.v.RH);
        break;
    case 119:
        delete_glue_ref(w.hh.v.RH);
        break;
    case 120:
        {
            q = w.hh.v.RH;
            if (q != MIN_HALFWORD)
                free_node(q, mem[q].hh.v.LH + mem[q].hh.v.LH + 1);
        }
        break;
    case 121:
        flush_node_list(w.hh.v.RH);
        break;
    case 72:
    case 91:
        if (w.hh.v.RH < 0 || w.hh.v.RH > 19)
            delete_sa_ref(w.hh.v.RH);
        break;
    default:
        ;
        break;
    }
}

void eq_save(int32_t p, uint16_t l)
{
    CACHE_THE_EQTB;

    if (save_ptr > max_save_stack) {
        max_save_stack = save_ptr;
        if (max_save_stack > save_size - 7)
            overflow(S(save_size), save_size);
    }
    if (l == LEVEL_ZERO)
        save_stack[save_ptr].hh.u.B0 = RESTORE_ZERO;
    else {

        save_stack[save_ptr] = eqtb[p];
        save_ptr++;
        save_stack[save_ptr].hh.u.B0 = RESTORE_OLD_VALUE;
    }
    save_stack[save_ptr].hh.u.B1 = l;
    save_stack[save_ptr].hh.v.RH = p;
    save_ptr++;
}

void
eq_define(int32_t p, uint16_t t, int32_t e)
{
    CACHE_THE_EQTB;

    if (eqtb[p].hh.u.B0 == t && eqtb[p].hh.v.RH == e) {
        eq_destroy(eqtb[p]);
        return;
    }

    if (eqtb[p].hh.u.B1 == cur_level)
        eq_destroy(eqtb[p]);
    else if (cur_level > LEVEL_ONE)
        eq_save(p, eqtb[p].hh.u.B1);

    eqtb[p].hh.u.B1 = cur_level;
    eqtb[p].hh.u.B0 = t;
    eqtb[p].hh.v.RH = e;
}

void
eq_word_define(int32_t p, integer w)
{
    CACHE_THE_EQTB;

    if (eqtb[p].cint == w)
        return;

    if (xeq_level[p] != cur_level) {
        eq_save(p, xeq_level[p]);
        xeq_level[p] = cur_level;
    }
    eqtb[p].cint = w;
}

void geq_define(int32_t p, uint16_t t, int32_t e)
{
    CACHE_THE_EQTB;

    eq_destroy(eqtb[p]);
    eqtb[p].hh.u.B1 = LEVEL_ONE;
    eqtb[p].hh.u.B0 = t;
    eqtb[p].hh.v.RH = e;
}

void geq_word_define(int32_t p, integer w)
{
    CACHE_THE_EQTB;

    eqtb[p].cint = w;
    xeq_level[p] = LEVEL_ONE;
}

void save_for_after(int32_t t)
{
    if (cur_level > LEVEL_ONE) {
        if (save_ptr > max_save_stack) {
            max_save_stack = save_ptr;
            if (max_save_stack > save_size - 7)
                overflow(S(save_size), save_size);
        }
        save_stack[save_ptr].hh.u.B0 = INSERT_TOKEN;
        save_stack[save_ptr].hh.u.B1 = LEVEL_ZERO;
        save_stack[save_ptr].hh.v.RH = t;
        save_ptr++;
    }
}

void unsave(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem; int32_t p;
    uint16_t l;
    int32_t t;
    boolean a;

    a = false;
    if (cur_level > LEVEL_ONE) {
        cur_level--;
        while (true) {

            save_ptr--;
            if (save_stack[save_ptr].hh.u.B0 == LEVEL_BOUNDARY)
                goto done;
            p = save_stack[save_ptr].hh.v.RH;
            if (save_stack[save_ptr].hh.u.B0 == INSERT_TOKEN) {   /*338: */
                t = cur_tok;
                cur_tok = p;
                if (a) {
                    p = get_avail();
                    mem[p].hh.v.LH = cur_tok;
                    mem[p].hh.v.RH = cur_input.loc;
                    cur_input.loc = p;
                    cur_input.start = p;
                    if (cur_tok < RIGHT_BRACE_LIMIT) {

                        if (cur_tok < LEFT_BRACE_LIMIT)
                            align_state--;
                        else
                            align_state++;
                    }
                } else {
                    back_input();
                    a = true;
                }
                cur_tok = t;
            } else if (save_stack[save_ptr].hh.u.B0 == RESTORE_SA) {
                sa_restore();
                sa_chain = p;
                sa_level = save_stack[save_ptr].hh.u.B1;
            } else {

                if (save_stack[save_ptr].hh.u.B0 == RESTORE_OLD_VALUE) {
                    l = save_stack[save_ptr].hh.u.B1;
                    save_ptr--;
                } else
                    save_stack[save_ptr] = eqtb[UNDEFINED_CONTROL_SEQUENCE];
                if ((p < INT_BASE) || (p > EQTB_SIZE)) {

                    if (eqtb[p].hh.u.B1 == LEVEL_ONE) {
                        eq_destroy(save_stack[save_ptr]);
                    } else {
                        eq_destroy(eqtb[p]);
                        eqtb[p] = save_stack[save_ptr];
                    }
                } else if (xeq_level[p] != LEVEL_ONE) {
                    eqtb[p] = save_stack[save_ptr];
                    xeq_level[p] = l;
                }
            }
        }

    done:
        if (grp_stack[in_open] == cur_boundary)
            group_warning();
        cur_group = save_stack[save_ptr].hh.u.B1;
        cur_boundary = save_stack[save_ptr].hh.v.RH;
        save_ptr--;
    } else
        confusion(S(curlevel));
}

void prepare_mag(void)
{
    CACHE_THE_EQTB;

    if (mag_set > 0 && INTPAR(mag) != mag_set) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Incompatible_magnification__/**/));
        }
        print_int(INTPAR(mag));
        print(S(___Z6/*");"*/));
        print_nl(S(_the_previous_value_will_be_/*retained*/));
        {
            help_ptr = 2;
            help_line[1] = S(I_can_handle_only_one_magnif/*ication ratio per job. So I've*/);
            help_line[0] = S(reverted_to_the_magnificatio/*n you used earlier on this run.*/);
        }
        int_error(mag_set);
        geq_word_define(INT_BASE + INT_PAR__mag, mag_set);
    }
    if ((INTPAR(mag) <= 0) || (INTPAR(mag) > 32768L)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Illegal_magnification_has_be/*en changed to 1000*/));
        }
        {
            help_ptr = 1;
            help_line[0] = S(The_magnification_ratio_must/* be between 1 and 32768.*/);
        }
        int_error(INTPAR(mag));
        geq_word_define(INT_BASE + INT_PAR__mag, 1000);
    }
    mag_set = INTPAR(mag);
}

void token_show(int32_t p)
{
    memory_word *mem = zmem;

    if (p != MIN_HALFWORD)
        show_token_list(mem[p].hh.v.RH, MIN_HALFWORD, 10000000L);
}

void print_meaning(void)
{
    print_cmd_chr(cur_cmd, cur_chr);
    if (cur_cmd >= CALL) {
        print_char(58 /*":" */ );
        print_ln();
        token_show(cur_chr);
    } else if ((cur_cmd == TOP_BOT_MARK) && (cur_chr < 5)) {
        print_char(58 /*":" */ );
        print_ln();
        token_show(cur_mark[cur_chr]);
    }
}

void show_cur_cmd_chr(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer n;
    integer l;
    int32_t p;

    begin_diagnostic();
    print_nl(123 /*"_" */ );
    if (cur_list.mode != shown_mode) {
        print_mode(cur_list.mode);
        print(S(___Z3/*": "*/));
        shown_mode = cur_list.mode;
    }
    print_cmd_chr(cur_cmd, cur_chr);
    if (INTPAR(tracing_ifs) > 0) {

        if (cur_cmd >= IF_TEST) {

            if (cur_cmd <= FI_OR_ELSE) {
                print(S(___Z3/*": "*/));
                if (cur_cmd == FI_OR_ELSE) {
                    print_cmd_chr(IF_TEST, cur_if);
                    print_char(32 /*" " */ );
                    n = 0;
                    l = if_line;
                } else {

                    n = 1;
                    l = line;
                }
                p = cond_ptr;
                while (p != MIN_HALFWORD) {

                    n++;
                    p = mem[p].hh.v.RH;
                }
                print(S(_level_));
                print_int(n);
                print_char(41 /*")" */ );
                if (l != 0) {
                    print(S(_entered_on_line_));
                    print_int(l);
                }
            }
        }
    }
    print_char(125 /*"_" */ );
    end_diagnostic(false);
}

void show_context(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    unsigned char /*max_selector */ old_setting;
    integer nn;
    boolean bottom_line;
    integer i;
    integer j;
    integer l;
    integer m;
    integer n;
    integer p;
    integer q;

    base_ptr = input_ptr;
    input_stack[base_ptr] = cur_input;
    nn = -1;
    bottom_line = false;

    while (true) {

        cur_input = input_stack[base_ptr];
        if ((cur_input.state != TOKEN_LIST)) {

            if ((cur_input.name > 19) || (base_ptr == 0))
                bottom_line = true;
        }
        if ((base_ptr == input_ptr) || bottom_line || (nn < INTPAR(error_context_lines))) {   /*324: */
            if ((base_ptr == input_ptr) || (cur_input.state != TOKEN_LIST)
                || (cur_input.index != BACKED_UP) || (cur_input.loc != MIN_HALFWORD)) {
                tally = 0;
                old_setting = selector;
                if (cur_input.state != TOKEN_LIST) {
                    if (cur_input.name <= 17) {

                        if ((cur_input.name == 0)) {

                            if (base_ptr == 0)
                                print_nl(S(____Z4/*"<*>"*/));
                            else
                                print_nl(S(_insert__));
                        } else {

                            print_nl(S(_read_));
                            if (cur_input.name == 17)
                                print_char(42 /*"*" */ );
                            else
                                print_int(cur_input.name - 1);
                            print_char(62 /*">" */ );
                        }
                    } else {

                        print_nl(S(l_));
                        if (cur_input.index == in_open)
                            print_int(line);
                        else
                            print_int(line_stack[cur_input.index + 1]);
                    }
                    print_char(32 /*" " */ );
                    {
                        l = tally;
                        tally = 0;
                        selector = SELECTOR_PSEUDO;
                        trick_count = 1000000L;
                    }
                    if (buffer[cur_input.limit] == INTPAR(end_line_char))
                        j = cur_input.limit;
                    else
                        j = cur_input.limit + 1;
                    if (j > 0) {
                        register integer for_end;
                        i = cur_input.start;
                        for_end = j - 1;
                        if (i <= for_end)
                            do {
                                if (i == cur_input.loc) {
                                    first_count = tally;
                                    trick_count = tally + 1 + error_line - half_error_line;
                                    if (trick_count < error_line)
                                        trick_count = error_line;
                                }
                                print_char(buffer[i]);
                            }
                            while (i++ < for_end);
                    }
                } else {

                    switch (cur_input.index) {
                    case 0:
                        print_nl(S(_argument__));
                        break;
                    case 1:
                    case 2:
                        print_nl(S(_template__));
                        break;
                    case 3:
                    case 4:
                        if (cur_input.loc == MIN_HALFWORD)
                            print_nl(S(_recently_read__));
                        else
                            print_nl(S(_to_be_read_again__));
                        break;
                    case 5:
                        print_nl(S(_inserted_text__));
                        break;
                    case 6:
                        {
                            print_ln();
                            print_cs(cur_input.name);
                        }
                        break;
                    case 7:
                        print_nl(S(_output__));
                        break;
                    case 8:
                        print_nl(S(_everypar__));
                        break;
                    case 9:
                        print_nl(S(_everymath__));
                        break;
                    case 10:
                        print_nl(S(_everydisplay__));
                        break;
                    case 11:
                        print_nl(S(_everyhbox__));
                        break;
                    case 12:
                        print_nl(S(_everyvbox__));
                        break;
                    case 13:
                        print_nl(S(_everyjob__));
                        break;
                    case 14:
                        print_nl(S(_everycr__));
                        break;
                    case 15:
                        print_nl(S(_mark__));
                        break;
                    case 16:
                        print_nl(S(_everyeof__));
                        break;
                    case 17:
                        print_nl(S(_XeTeXinterchartoks__));
                        break;
                    case 18:
                        print_nl(S(_write__));
                        break;
                    default:
                        print_nl(63 /*"?" */ );
                        break;
                    }
                    {
                        l = tally;
                        tally = 0;
                        selector = SELECTOR_PSEUDO;
                        trick_count = 1000000L;
                    }
                    if (cur_input.index < MACRO)
                        show_token_list(cur_input.start, cur_input.loc, 100000L);
                    else
                        show_token_list(mem[cur_input.start].hh.v.RH, cur_input.loc, 100000L);
                }
                selector = old_setting;
                if (trick_count == 1000000L) {
                    first_count = tally;
                    trick_count = tally + 1 + error_line - half_error_line;
                    if (trick_count < error_line)
                        trick_count = error_line;
                }
                if (tally < trick_count)
                    m = tally - first_count;
                else
                    m = trick_count - first_count;
                if (l + first_count <= half_error_line) {
                    p = 0;
                    n = l + first_count;
                } else {

                    print(S(____Z1/*"..."*/));
                    p = l + first_count - half_error_line + 3;
                    n = half_error_line;
                }
                {
                    register integer for_end;
                    q = p;
                    for_end = first_count - 1;
                    if (q <= for_end)
                        do
                            print_raw_char(trick_buf[q % error_line], true);
                        while (q++ < for_end);
                }
                print_ln();
                {
                    register integer for_end;
                    q = 1;
                    for_end = n;
                    if (q <= for_end)
                        do
                            print_raw_char(32 /*" " */ , true);
                        while (q++ < for_end);
                }
                if (m + n <= error_line)
                    p = first_count + m;
                else
                    p = first_count + (error_line - n - 3);
                {
                    register integer for_end;
                    q = first_count;
                    for_end = p - 1;
                    if (q <= for_end)
                        do
                            print_raw_char(trick_buf[q % error_line], true);
                        while (q++ < for_end);
                }
                if (m + n > error_line)
                    print(S(____Z1/*"..."*/));
                nn++;
            }
        } else if (nn == INTPAR(error_context_lines)) {
            print_nl(S(____Z1/*"..."*/));
            nn++;
        }
        if (bottom_line)
            goto done;
        base_ptr--;
    }
done:
    cur_input = input_stack[input_ptr];
}

void begin_token_list(int32_t p, uint16_t t)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;

    {
        if (input_ptr > max_in_stack) {
            max_in_stack = input_ptr;
            if (input_ptr == stack_size)
                overflow(S(input_stack_size), stack_size);
        }
        input_stack[input_ptr] = cur_input;
        input_ptr++;
    }

    cur_input.state = TOKEN_LIST;
    cur_input.start = p;
    cur_input.index = t;
    if (t >= MACRO) {
        mem[p].hh.v.LH++;
        if (t == MACRO)
            cur_input.limit = param_ptr;
        else {

            cur_input.loc = mem[p].hh.v.RH;
            if (INTPAR(tracing_macros) > 1) {
                begin_diagnostic();
                print_nl(S());
                switch (t) {
                case 15:
                    print_esc(S(mark));
                    break;
                case 18:
                    print_esc(S(write));
                    break;
                default:
                    print_cmd_chr(ASSIGN_TOKS, t + 2252765L);
                    break;
                }
                print(S(___Z7/*"->"*/));
                token_show(p);
                end_diagnostic(false);
            }
        }
    } else
        cur_input.loc = p;
}

void end_token_list(void)
{
    if (cur_input.index >= BACKED_UP) {
        if (cur_input.index <= INSERTED)
            flush_list(cur_input.start);
        else {

            delete_token_ref(cur_input.start);
            if (cur_input.index == MACRO)
                while (param_ptr > cur_input.limit) {

                    param_ptr--;
                    flush_list(param_stack[param_ptr]);
                }
        }
    } else if (cur_input.index == U_TEMPLATE) {

        if (align_state > 500000L)
            align_state = 0;
        else
            fatal_error(S(_interwoven_alignment_preamb/*les are not allowed)*/));
    }
    {
        input_ptr--;
        cur_input = input_stack[input_ptr];
    }
}

void back_input(void)
{
    memory_word *mem = zmem; int32_t p;
    while ((cur_input.state == TOKEN_LIST) && (cur_input.loc == MIN_HALFWORD)
           && (cur_input.index != V_TEMPLATE))
        end_token_list();
    p = get_avail();
    mem[p].hh.v.LH = cur_tok;
    if (cur_tok < RIGHT_BRACE_LIMIT) {

        if (cur_tok < LEFT_BRACE_LIMIT)
            align_state--;
        else
            align_state++;
    }
    {
        if (input_ptr > max_in_stack) {
            max_in_stack = input_ptr;
            if (input_ptr == stack_size)
                overflow(S(input_stack_size), stack_size);
        }
        input_stack[input_ptr] = cur_input;
        input_ptr++;
    }
    cur_input.state = TOKEN_LIST;
    cur_input.start = p;
    cur_input.index = BACKED_UP;
    cur_input.loc = p;
}

void
back_error(void)
{
    back_input();
    error();
}

void
ins_error(void)
{
    back_input();
    cur_input.index = INSERTED;
    error();
}

void begin_file_reading(void)
{
    if (in_open == max_in_open)
        overflow(S(text_input_levels), max_in_open);
    if (first == buf_size)
        overflow(S(buffer_size), buf_size);
    in_open++;
    {
        if (input_ptr > max_in_stack) {
            max_in_stack = input_ptr;
            if (input_ptr == stack_size)
                overflow(S(input_stack_size), stack_size);
        }
        input_stack[input_ptr] = cur_input;
        input_ptr++;
    }
    cur_input.index = in_open;
    source_filename_stack[cur_input.index] = 0;
    full_source_filename_stack[cur_input.index] = 0;
    eof_seen[cur_input.index] = false;
    grp_stack[cur_input.index] = cur_boundary;
    if_stack[cur_input.index] = cond_ptr;
    line_stack[cur_input.index] = line;
    cur_input.start = first;
    cur_input.state = MID_LINE;
    cur_input.name = 0;
    cur_input.synctex_tag = 0;
}

void end_file_reading(void)
{
    first = cur_input.start;
    line = line_stack[cur_input.index];
    if ((cur_input.name == 18) || (cur_input.name == 19))
        pseudo_close();
    else if (cur_input.name > 17)
        u_close(input_file[cur_input.index]);
    {
        input_ptr--;
        cur_input = input_stack[input_ptr];
    }
    in_open--;
}

void check_outer_validity(void)
{
    memory_word *mem = zmem; int32_t p;
    int32_t q;
    if (scanner_status != NORMAL) {
        deletions_allowed = false;
        if (cur_cs != 0) {
            if ((cur_input.state == TOKEN_LIST) || (cur_input.name < 1)
                || (cur_input.name > 17)) {
                p = get_avail();
                mem[p].hh.v.LH = CS_TOKEN_FLAG + cur_cs;
                begin_token_list(p, BACKED_UP);
            }
            cur_cmd = SPACER;
            cur_chr = 32 /*" " */ ;
        }
        if (scanner_status > SKIPPING) {        /*350: */
            runaway();
            if (cur_cs == 0) {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(File_ended));
            } else {

                cur_cs = 0;
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Forbidden_control_sequence_f/*ound*/));
                }
            }
            p = get_avail();
            switch (scanner_status) {
            case 2:
                {
                    print(S(_while_scanning_definition));
                    mem[p].hh.v.LH = (RIGHT_BRACE_TOKEN + 125);
                }
                break;
            case 3:
                {
                    print(S(_while_scanning_use));
                    mem[p].hh.v.LH = par_token;
                    long_state = OUTER_CALL;
                }
                break;
            case 4:
                {
                    print(S(_while_scanning_preamble));
                    mem[p].hh.v.LH = (RIGHT_BRACE_TOKEN + 125);
                    q = p;
                    p = get_avail();
                    mem[p].hh.v.RH = q;
                    mem[p].hh.v.LH = (CS_TOKEN_FLAG + 2243227);
                    align_state = -1000000L;
                }
                break;
            case 5:
                {
                    print(S(_while_scanning_text));
                    mem[p].hh.v.LH = (RIGHT_BRACE_TOKEN + 125);
                }
                break;
            }
            begin_token_list(p, INSERTED);
            print(S(_of_));
            sprint_cs(warning_index);
            {
                help_ptr = 4;
                help_line[3] = 65927L /*"I suspect you have forgotten a `_', causing me" */ ;
                help_line[2] = S(to_read_past_where_you_wante/*d me to stop.*/);
                help_line[1] = S(I_ll_try_to_recover__but_if_/*the error is serious,*/);
                help_line[0] = S(you_d_better_type__E__or__X_/* now and fix your file.*/);
            }
            error();
        } else {

            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Incomplete_));
            }
            print_cmd_chr(IF_TEST, cur_if);
            print(S(__all_text_was_ignored_after/* line */));
            print_int(skip_line);
            {
                help_ptr = 3;
                help_line[2] = S(A_forbidden_control_sequence/* occurred in skipped text.*/);
                help_line[1] = S(This_kind_of_error_happens_w/*hen you say `\if...' and forget*/);
                help_line[0] = S(the_matching___fi___I_ve_ins/*erted a `\fi'; this might work.*/);
            }
            if (cur_cs != 0)
                cur_cs = 0;
            else
                help_line[2] = S(The_file_ended_while_I_was_s/*kipping conditional text.*/);
            cur_tok = (CS_TOKEN_FLAG + 2243230);
            ins_error();
        }
        deletions_allowed = true;
    }
}

void get_next(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer k;
    int32_t t;
    unsigned char /*max_char_code */ cat;
    UnicodeScalar c;
    UTF16_code lower;
    small_number d;
    small_number sup_count;

lab20: /*restart */
    cur_cs = 0;

    if (cur_input.state != TOKEN_LIST) {  /*355: */
 lab25:                        /*switch */ if (cur_input.loc <= cur_input.limit) {
            cur_chr = buffer[cur_input.loc];
            cur_input.loc++;
            if ((cur_chr >= 55296L) && (cur_chr < 56320L) && (cur_input.loc <= cur_input.limit)
                && (buffer[cur_input.loc] >= 56320L) && (buffer[cur_input.loc] < 57344L)) {
                lower = buffer[cur_input.loc] - 56320L;
                cur_input.loc++;
                cur_chr = 65536L + (cur_chr - 55296L) * 1024 + lower;
            }
 lab21:    /*reswitch */ cur_cmd = CAT_CODE(cur_chr);
            switch (cur_input.state + cur_cmd) {  /*357: */
            case 10:
            case 26:
            case 42:
            case 27:
            case 43:
                goto lab25;
                break;
            case 1:
            case 17:
            case 33:
                {
                    if (cur_input.loc > cur_input.limit)
                        cur_cs = NULL_CS;
                    else {

 lab26:                        /*start_cs */ k = cur_input.loc;
                        cur_chr = buffer[k];
                        cat = CAT_CODE(cur_chr);
                        k++;
                        if (cat == LETTER)
                            cur_input.state = SKIP_BLANKS;
                        else if (cat == SPACER)
                            cur_input.state = SKIP_BLANKS;
                        else
                            cur_input.state = MID_LINE;
                        if ((cat == LETTER) && (k <= cur_input.limit)) { /*368: */
                            do {
                                cur_chr = buffer[k];
                                cat = CAT_CODE(cur_chr);
                                k++;
                            } while (!((cat != LETTER) || (k > cur_input.limit)));
                            {
                                if ((cat == SUP_MARK) && (buffer[k] == cur_chr) && (k < cur_input.limit)) {
                                    sup_count = 2;
                                    while ((sup_count < 6) && (k + 2 * sup_count - 2 <= cur_input.limit)
                                           && (buffer[k + sup_count - 1] == cur_chr))
                                        sup_count++;
                                    {
                                        register integer for_end;
                                        d = 1;
                                        for_end = sup_count;
                                        if (d <= for_end)
                                            do
                                                if (!
                                                    (((buffer[k + sup_count - 2 + d] >= 48 /*"0" */ )
                                                      && (buffer[k + sup_count - 2 + d] <= 57 /*"9" */ ))
                                                     || ((buffer[k + sup_count - 2 + d] >= 97 /*"a" */ )
                                                         && (buffer[k + sup_count - 2 + d] <= 102 /*"f" */ )))) {
                                                    c = buffer[k + 1];
                                                    if (c < 128) {
                                                        if (c < 64)
                                                            buffer[k - 1] = c + 64;
                                                        else
                                                            buffer[k - 1] = c - 64;
                                                        d = 2;
                                                        cur_input.limit = cur_input.limit - d;
                                                        while (k <= cur_input.limit) {

                                                            buffer[k] = buffer[k + d];
                                                            k++;
                                                        }
                                                        goto lab26;
                                                    } else
                                                        sup_count = 0;
                                                }
                                            while (d++ < for_end) ;
                                    }
                                    if (sup_count > 0) {
                                        cur_chr = 0;
                                        {
                                            register integer for_end;
                                            d = 1;
                                            for_end = sup_count;
                                            if (d <= for_end)
                                                do {
                                                    c = buffer[k + sup_count - 2 + d];
                                                    if (c <= 57 /*"9" */ )
                                                        cur_chr = 16 * cur_chr + c - 48;
                                                    else
                                                        cur_chr = 16 * cur_chr + c - 87;
                                                }
                                                while (d++ < for_end);
                                        }
                                        if (cur_chr > BIGGEST_USV)
                                            cur_chr = buffer[k];
                                        else {

                                            buffer[k - 1] = cur_chr;
                                            d = 2 * sup_count - 1;
                                            cur_input.limit = cur_input.limit - d;
                                            while (k <= cur_input.limit) {

                                                buffer[k] = buffer[k + d];
                                                k++;
                                            }
                                            goto lab26;
                                        }
                                    }
                                }
                            }
                            if (cat != LETTER)
                                k--;
                            if (k > cur_input.loc + 1) {
                                cur_cs = id_lookup(cur_input.loc, k - cur_input.loc);
                                cur_input.loc = k;
                                goto lab40;
                            }
                        } else {        /*367: */

                            if ((cat == SUP_MARK) && (buffer[k] == cur_chr) && (k < cur_input.limit)) {
                                sup_count = 2;
                                while ((sup_count < 6) && (k + 2 * sup_count - 2 <= cur_input.limit)
                                       && (buffer[k + sup_count - 1] == cur_chr))
                                    sup_count++;
                                {
                                    register integer for_end;
                                    d = 1;
                                    for_end = sup_count;
                                    if (d <= for_end)
                                        do
                                            if (!
                                                (((buffer[k + sup_count - 2 + d] >= 48 /*"0" */ )
                                                  && (buffer[k + sup_count - 2 + d] <= 57 /*"9" */ ))
                                                 || ((buffer[k + sup_count - 2 + d] >= 97 /*"a" */ )
                                                     && (buffer[k + sup_count - 2 + d] <= 102 /*"f" */ )))) {
                                                c = buffer[k + 1];
                                                if (c < 128) {
                                                    if (c < 64)
                                                        buffer[k - 1] = c + 64;
                                                    else
                                                        buffer[k - 1] = c - 64;
                                                    d = 2;
                                                    cur_input.limit = cur_input.limit - d;
                                                    while (k <= cur_input.limit) {

                                                        buffer[k] = buffer[k + d];
                                                        k++;
                                                    }
                                                    goto lab26;
                                                } else
                                                    sup_count = 0;
                                            }
                                        while (d++ < for_end) ;
                                }
                                if (sup_count > 0) {
                                    cur_chr = 0;
                                    {
                                        register integer for_end;
                                        d = 1;
                                        for_end = sup_count;
                                        if (d <= for_end)
                                            do {
                                                c = buffer[k + sup_count - 2 + d];
                                                if (c <= 57 /*"9" */ )
                                                    cur_chr = 16 * cur_chr + c - 48;
                                                else
                                                    cur_chr = 16 * cur_chr + c - 87;
                                            }
                                            while (d++ < for_end);
                                    }
                                    if (cur_chr > BIGGEST_USV)
                                        cur_chr = buffer[k];
                                    else {

                                        buffer[k - 1] = cur_chr;
                                        d = 2 * sup_count - 1;
                                        cur_input.limit = cur_input.limit - d;
                                        while (k <= cur_input.limit) {

                                            buffer[k] = buffer[k + d];
                                            k++;
                                        }
                                        goto lab26;
                                    }
                                }
                            }
                        }
                        if (buffer[cur_input.loc] > 65535L) {
                            cur_cs = id_lookup(cur_input.loc, 1);
                            cur_input.loc++;
                            goto lab40;
                        }
                        cur_cs = SINGLE_BASE + buffer[cur_input.loc];
                        cur_input.loc++;
                    }
 lab40:                        /*found */ cur_cmd = eqtb[cur_cs].hh.u.B0;
                    cur_chr = eqtb[cur_cs].hh.v.RH;
                    if (cur_cmd >= OUTER_CALL)
                        check_outer_validity();
                }
                break;
            case 14:
            case 30:
            case 46:
                {
                    cur_cs = cur_chr + 1;
                    cur_cmd = eqtb[cur_cs].hh.u.B0;
                    cur_chr = eqtb[cur_cs].hh.v.RH;
                    cur_input.state = MID_LINE;
                    if (cur_cmd >= OUTER_CALL)
                        check_outer_validity();
                }
                break;
            case 8:
            case 24:
            case 40:
                {
                    if (cur_chr == buffer[cur_input.loc]) {

                        if (cur_input.loc < cur_input.limit) {
                            sup_count = 2;
                            while ((sup_count < 6) && (cur_input.loc + 2 * sup_count - 2 <= cur_input.limit)
                                   && (cur_chr == buffer[cur_input.loc + sup_count - 1]))
                                sup_count++;
                            {
                                register integer for_end;
                                d = 1;
                                for_end = sup_count;
                                if (d <= for_end)
                                    do
                                        if (!
                                            (((buffer[cur_input.loc + sup_count - 2 + d] >= 48 /*"0" */ )
                                              && (buffer[cur_input.loc + sup_count - 2 + d] <= 57 /*"9" */ ))
                                             || ((buffer[cur_input.loc + sup_count - 2 + d] >= 97 /*"a" */ )
                                                 && (buffer[cur_input.loc + sup_count - 2 + d] <=
                                                     102 /*"f" */ )))) {
                                            c = buffer[cur_input.loc + 1];
                                            if (c < 128) {
                                                cur_input.loc = cur_input.loc + 2;
                                                if (c < 64)
                                                    cur_chr = c + 64;
                                                else
                                                    cur_chr = c - 64;
                                                goto lab21;
                                            }
                                            goto lab27;
                                        }
                                    while (d++ < for_end) ;
                            }
                            cur_chr = 0;
                            {
                                register integer for_end;
                                d = 1;
                                for_end = sup_count;
                                if (d <= for_end)
                                    do {
                                        c = buffer[cur_input.loc + sup_count - 2 + d];
                                        if (c <= 57 /*"9" */ )
                                            cur_chr = 16 * cur_chr + c - 48;
                                        else
                                            cur_chr = 16 * cur_chr + c - 87;
                                    }
                                    while (d++ < for_end);
                            }
                            if (cur_chr > BIGGEST_USV) {
                                cur_chr = buffer[cur_input.loc];
                                goto lab27;
                            }
                            cur_input.loc = cur_input.loc + 2 * sup_count - 1;
                            goto lab21;
                        }
                    }
 lab27:            /*not_exp */ cur_input.state = MID_LINE;
                }
                break;
            case 16:
            case 32:
            case 48:
                {
                    {
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(Text_line_contains_an_invali/*d character*/));
                    }
                    {
                        help_ptr = 2;
                        help_line[1] = S(A_funny_symbol_that_I_can_t_/*read has just been input.*/);
                        help_line[0] = S(Continue__and_I_ll_forget_th/*at it ever happened.*/);
                    }
                    deletions_allowed = false;
                    error();
                    deletions_allowed = true;
                    goto lab20;
                }
                break;
            case 11:
                {
                    cur_input.state = SKIP_BLANKS;
                    cur_chr = 32 /*" " */ ;
                }
                break;
            case 6:
                {
                    cur_input.loc = cur_input.limit + 1;
                    cur_cmd = SPACER;
                    cur_chr = 32 /*" " */ ;
                }
                break;
            case 22:
            case 15:
            case 31:
            case 47:
                {
                    cur_input.loc = cur_input.limit + 1;
                    goto lab25;
                }
                break;
            case 38:
                {
                    cur_input.loc = cur_input.limit + 1;
                    cur_cs = par_loc;
                    cur_cmd = eqtb[cur_cs].hh.u.B0;
                    cur_chr = eqtb[cur_cs].hh.v.RH;
                    if (cur_cmd >= OUTER_CALL)
                        check_outer_validity();
                }
                break;
            case 2:
                align_state++;
                break;
            case 18:
            case 34:
                {
                    cur_input.state = MID_LINE;
                    align_state++;
                }
                break;
            case 3:
                align_state--;
                break;
            case 19:
            case 35:
                {
                    cur_input.state = MID_LINE;
                    align_state--;
                }
                break;
            case 20:
            case 21:
            case 23:
            case 25:
            case 28:
            case 29:
            case 36:
            case 37:
            case 39:
            case 41:
            case 44:
            case 45:
                cur_input.state = MID_LINE;
                break;
            default:
                ;
                break;
            }
        } else {

            cur_input.state = NEW_LINE;
            if (cur_input.name > 17) {    /*374: */
                line++;
                first = cur_input.start;
                if (!force_eof) {

                    if (cur_input.name <= 19) {
                        if (pseudo_input())
                            cur_input.limit = last;
                        else if ((LOCAL(every_eof) != MIN_HALFWORD)
                                 && !eof_seen[cur_input.index]) {
                            cur_input.limit = first - 1;
                            eof_seen[cur_input.index] = true;
                            begin_token_list(LOCAL(every_eof), EVERY_EOF_TEXT);
                            goto lab20;
                        } else
                            force_eof = true;
                    } else {

                        if (input_line(input_file[cur_input.index]))
                            cur_input.limit = last;
                        else if ((LOCAL(every_eof) != MIN_HALFWORD)
                                 && !eof_seen[cur_input.index]) {
                            cur_input.limit = first - 1;
                            eof_seen[cur_input.index] = true;
                            begin_token_list(LOCAL(every_eof), EVERY_EOF_TEXT);
                            goto lab20;
                        } else
                            force_eof = true;
                    }
                }
                if (force_eof) {
                    if (INTPAR(tracing_nesting) > 0) {

                        if ((grp_stack[in_open] != cur_boundary) || (if_stack[in_open] != cond_ptr))
                            file_warning();
                    }
                    if (cur_input.name >= 19) {
                        print_char(41 /*")" */ );
                        open_parens--;
                        ttstub_output_flush (rust_stdout);
                    }
                    force_eof = false;
                    end_file_reading();
                    check_outer_validity();
                    goto lab20;
                }
                if ((INTPAR(end_line_char) < 0) || (INTPAR(end_line_char) > 255))
                    cur_input.limit--;
                else
                    buffer[cur_input.limit] = INTPAR(end_line_char);
                first = cur_input.limit + 1;
                cur_input.loc = cur_input.start;
            } else {

                if (!(cur_input.name == 0)) {
                    cur_cmd = 0;
                    cur_chr = 0;
                    return;
                }
                if (input_ptr > 0) {
                    end_file_reading();
                    goto lab20;
                }
                if (selector < SELECTOR_LOG_ONLY)
                    open_log_file();

                fatal_error(S(_____job_aborted__no_legal__/*end found)*/));
            }
            goto lab25;
        }
    } else /*369: */ if (cur_input.loc != MIN_HALFWORD) {
        t = mem[cur_input.loc].hh.v.LH;
        cur_input.loc = mem[cur_input.loc].hh.v.RH;
        if (t >= CS_TOKEN_FLAG) {
            cur_cs = t - CS_TOKEN_FLAG;
            cur_cmd = eqtb[cur_cs].hh.u.B0;
            cur_chr = eqtb[cur_cs].hh.v.RH;
            if (cur_cmd >= OUTER_CALL) {

                if (cur_cmd == DONT_EXPAND) { /*370: */
                    cur_cs = mem[cur_input.loc].hh.v.LH - CS_TOKEN_FLAG;
                    cur_input.loc = MIN_HALFWORD;
                    cur_cmd = eqtb[cur_cs].hh.u.B0;
                    cur_chr = eqtb[cur_cs].hh.v.RH;
                    if (cur_cmd > MAX_COMMAND) {
                        cur_cmd = RELAX;
                        cur_chr = NO_EXPAND_FLAG;
                    }
                } else
                    check_outer_validity();
            }
        } else {

            cur_cmd = t / MAX_CHAR_VAL;
            cur_chr = t % MAX_CHAR_VAL;
            switch (cur_cmd) {
            case 1:
                align_state++;
                break;
            case 2:
                align_state--;
                break;
            case 5:
                {
                    begin_token_list(param_stack[cur_input.limit + cur_chr - 1], PARAMETER);
                    goto lab20;
                }
                break;
            default:
                ;
                break;
            }
        }
    } else {

        end_token_list();
        goto lab20;
    }
    if (cur_cmd <= CAR_RET) {

        if (cur_cmd >= TAB_MARK) {

            if (align_state == 0) {     /*818: */
                if ((scanner_status == ALIGNING) || (cur_align == MIN_HALFWORD))
                    fatal_error(S(_interwoven_alignment_preamb/*les are not allowed)*/));
                cur_cmd = mem[cur_align + 5].hh.v.LH;
                mem[cur_align + 5].hh.v.LH = cur_chr;
                if (cur_cmd == OMIT)
                    begin_token_list(mem_top - 10, V_TEMPLATE);
                else
                    begin_token_list(mem[cur_align + 2].cint, V_TEMPLATE);
                align_state = 1000000L;
                goto lab20;
            }
        }
    }
}

void get_token(void)
{
    no_new_control_sequence = false;
    get_next();
    no_new_control_sequence = true;
    if (cur_cs == 0)
        cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
    else
        cur_tok = CS_TOKEN_FLAG + cur_cs;
}

void macro_call(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t r;
    int32_t p;
    int32_t q;
    int32_t s;
    int32_t t;
    int32_t u, v;
    int32_t rbrace_ptr;
    small_number n;
    int32_t unbalance;
    int32_t m;
    int32_t ref_count;
    small_number save_scanner_status;
    int32_t save_warning_index;
    UTF16_code match_chr;

    save_scanner_status = scanner_status;
    save_warning_index = warning_index;
    warning_index = cur_cs;
    ref_count = cur_chr;
    r = mem[ref_count].hh.v.RH;
    n = 0;
    if (INTPAR(tracing_macros) > 0) {    /*419: */
        begin_diagnostic();
        print_ln();
        print_cs(warning_index);
        token_show(ref_count);
        end_diagnostic(false);
    }
    if (mem[r].hh.v.LH == PROTECTED_TOKEN)
        r = mem[r].hh.v.RH;
    if (mem[r].hh.v.LH != END_MATCH_TOKEN) {    /*409: */
        scanner_status = MATCHING;
        unbalance = 0;
        long_state = eqtb[cur_cs].hh.u.B0;
        if (long_state >= OUTER_CALL)
            long_state = long_state - 2;
        do {
            mem[mem_top - 3].hh.v.RH = MIN_HALFWORD;
            if ((mem[r].hh.v.LH >= END_MATCH_TOKEN) || (mem[r].hh.v.LH < MATCH_TOKEN))
                s = MIN_HALFWORD;
            else {

                match_chr = mem[r].hh.v.LH - 27262976L;
                s = mem[r].hh.v.RH;
                r = s;
                p = mem_top - 3;
                m = 0;
            }
 lab22:                        /*continue */ get_token();
            if (cur_tok == mem[r].hh.v.LH) {    /*412: */
                r = mem[r].hh.v.RH;
                if ((mem[r].hh.v.LH >= MATCH_TOKEN)
                    && (mem[r].hh.v.LH <= END_MATCH_TOKEN)) {
                    if (cur_tok < LEFT_BRACE_LIMIT)
                        align_state--;
                    goto lab40;
                } else
                    goto lab22;
            }
            if (s != r) {

                if (s == MIN_HALFWORD) { /*416: */
                    {
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(Use_of_));
                    }
                    sprint_cs(warning_index);
                    print(S(_doesn_t_match_its_definitio/*n*/));
                    {
                        help_ptr = 4;
                        help_line[3] = 65975L /*"If you say, e.g., `\def\a1_..._', then you must always" */ ;
                        help_line[2] = S(put__1__after___a___since_co/*ntrol sequence names are*/);
                        help_line[1] = S(made_up_of_letters_only__The/* macro here has not been*/);
                        help_line[0] = S(followed_by_the_required_stu/*ff, so I'm ignoring it.*/);
                    }
                    error();
                    goto exit;
                } else {

                    t = s;
                    do {
                        {
                            q = get_avail();
                            mem[p].hh.v.RH = q;
                            mem[q].hh.v.LH = mem[t].hh.v.LH;
                            p = q;
                        }
                        m++;
                        u = mem[t].hh.v.RH;
                        v = s;
                        while (true) {

                            if (u == r) {

                                if (cur_tok != mem[v].hh.v.LH)
                                    goto done;
                                else {

                                    r = mem[v].hh.v.RH;
                                    goto lab22;
                                }
                            }
                            if (mem[u].hh.v.LH != mem[v].hh.v.LH)
                                goto done;
                            u = mem[u].hh.v.RH;
                            v = mem[v].hh.v.RH;
                        }
                    done:
                        t = mem[t].hh.v.RH;
                    } while (!(t == r));
                    r = s;
                }
            }
            if (cur_tok == par_token) {

                if (long_state != LONG_CALL) {        /*414: */
                    if (long_state == CALL) {
                        runaway();
                        {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Paragraph_ended_before_));
                        }
                        sprint_cs(warning_index);
                        print(S(_was_complete));
                        {
                            help_ptr = 3;
                            help_line[2] = 65970L /*"I suspect you've forgotten a `_', causing me to apply this" */ ;
                            help_line[1] = S(control_sequence_to_too_much/* text. How can we recover?*/);
                            help_line[0] = S(My_plan_is_to_forget_the_who/*le thing and hope for the best.*/);
                        }
                        back_error();
                    }
                    pstack[n] = mem[mem_top - 3].hh.v.RH;
                    align_state = align_state - unbalance;
                    {
                        register integer for_end;
                        m = 0;
                        for_end = n;
                        if (m <= for_end)
                            do
                                flush_list(pstack[m]);
                            while (m++ < for_end);
                    }
                    goto exit;
                }
            }
            if (cur_tok < RIGHT_BRACE_LIMIT) {

                if (cur_tok < LEFT_BRACE_LIMIT) {        /*417: */
                    unbalance = 1;
                    while (true) {

                        {
                            {
                                q = avail;
                                if (q == MIN_HALFWORD)
                                    q = get_avail();
                                else {

                                    avail = mem[q].hh.v.RH;
                                    mem[q].hh.v.RH = MIN_HALFWORD;
                                }
                            }
                            mem[p].hh.v.RH = q;
                            mem[q].hh.v.LH = cur_tok;
                            p = q;
                        }
                        get_token();
                        if (cur_tok == par_token) {

                            if (long_state != LONG_CALL) {    /*414: */
                                if (long_state == CALL) {
                                    runaway();
                                    {
                                        if (file_line_error_style_p)
                                            print_file_line();
                                        else
                                            print_nl(S(__/*"! "*/));
                                        print(S(Paragraph_ended_before_));
                                    }
                                    sprint_cs(warning_index);
                                    print(S(_was_complete));
                                    {
                                        help_ptr = 3;
                                        help_line[2] =
                                            65970L /*"I suspect you've forgotten a `_', causing me to apply this" */ ;
                                        help_line[1] =
                                            S(control_sequence_to_too_much/* text. How can we recover?*/);
                                        help_line[0] =
                                            S(My_plan_is_to_forget_the_who/*le thing and hope for the best.*/);
                                    }
                                    back_error();
                                }
                                pstack[n] = mem[mem_top - 3].hh.v.RH;
                                align_state = align_state - unbalance;
                                {
                                    register integer for_end;
                                    m = 0;
                                    for_end = n;
                                    if (m <= for_end)
                                        do
                                            flush_list(pstack[m]);
                                        while (m++ < for_end);
                                }
                                goto exit;
                            }
                        }
                        if (cur_tok < RIGHT_BRACE_LIMIT) {

                            if (cur_tok < LEFT_BRACE_LIMIT)
                                unbalance++;
                            else {

                                unbalance--;
                                if (unbalance == 0)
                                    goto lab31;
                            }
                        }
                    }
 lab31:                        /*done1 */ rbrace_ptr = p;
                    {
                        q = get_avail();
                        mem[p].hh.v.RH = q;
                        mem[q].hh.v.LH = cur_tok;
                        p = q;
                    }
                } else {        /*413: */

                    back_input();
                    {
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(Argument_of_));
                    }
                    sprint_cs(warning_index);
                    print(65961L /*" has an extra _" */ );
                    {
                        help_ptr = 6;
                        help_line[5] = 65962L /*"I've run across a `_' that doesn't seem to match anything." */ ;
                        help_line[4] = 65963L /*"For example, `\def\a#1_..._' and `\a_' would produce" */ ;
                        help_line[3] = S(this_error__If_you_simply_pr/*oceed now, the `\par' that*/);
                        help_line[2] = S(I_ve_just_inserted_will_caus/*e me to report a runaway*/);
                        help_line[1] = S(argument_that_might_be_the_r/*oot of the problem. But if*/);
                        help_line[0] = 65967L /*"your `_' was spurious, just type `2' and it will go away." */ ;
                    }
                    align_state++;
                    long_state = CALL;
                    cur_tok = par_token;
                    ins_error();
                    goto lab22;
                }
            } else {            /*411: */

                if (cur_tok == SPACE_TOKEN) {

                    if (mem[r].hh.v.LH <= END_MATCH_TOKEN) {

                        if (mem[r].hh.v.LH >= MATCH_TOKEN)
                            goto lab22;
                    }
                }
                {
                    q = get_avail();
                    mem[p].hh.v.RH = q;
                    mem[q].hh.v.LH = cur_tok;
                    p = q;
                }
            }
            m++;
            if (mem[r].hh.v.LH > END_MATCH_TOKEN)
                goto lab22;
            if (mem[r].hh.v.LH < MATCH_TOKEN)
                goto lab22;
 lab40:                                        /*found */ if (s != MIN_HALFWORD) {
                                                /*418: */
                if ((m == 1) && (mem[p].hh.v.LH < RIGHT_BRACE_LIMIT) && (p != mem_top - 3)) {
                    mem[rbrace_ptr].hh.v.RH = MIN_HALFWORD;
                    {
                        mem[p].hh.v.RH = avail;
                        avail = p;
                    }
                    p = mem[mem_top - 3].hh.v.RH;
                    pstack[n] = mem[p].hh.v.RH;
                    {
                        mem[p].hh.v.RH = avail;
                        avail = p;
                    }
                } else
                    pstack[n] = mem[mem_top - 3].hh.v.RH;
                n++;
                if (INTPAR(tracing_macros) > 0) {
                    begin_diagnostic();
                    print_nl(match_chr);
                    print_int(n);
                    print(S(___Z9/*"<-"*/));
                    show_token_list(pstack[n - 1], MIN_HALFWORD, 1000);
                    end_diagnostic(false);
                }
            }
        } while (!(mem[r].hh.v.LH == END_MATCH_TOKEN));
    }
    while ((cur_input.state == TOKEN_LIST) && (cur_input.loc == MIN_HALFWORD)
           && (cur_input.index != V_TEMPLATE))
        end_token_list();
    begin_token_list(ref_count, MACRO);
    cur_input.name = warning_index;
    cur_input.loc = mem[r].hh.v.RH;
    if (n > 0) {
        if (param_ptr + n > max_param_stack) {
            max_param_stack = param_ptr + n;
            if (max_param_stack > param_size)
                overflow(S(parameter_stack_size), param_size);
        }
        {
            register integer for_end;
            m = 0;
            for_end = n - 1;
            if (m <= for_end)
                do
                    param_stack[param_ptr + m] = pstack[m];
                while (m++ < for_end);
        }
        param_ptr = param_ptr + n;
    }

exit:
    scanner_status = save_scanner_status;
    warning_index = save_warning_index;
}

void insert_relax(void)
{
    cur_tok = CS_TOKEN_FLAG + cur_cs;
    back_input();
    cur_tok = (CS_TOKEN_FLAG + 2243233);
    back_input();
    cur_input.index = INSERTED;
}

void new_index(uint16_t i, int32_t q)
{
    memory_word *mem = zmem; small_number k;
    cur_ptr = get_node(INDEX_NODE_SIZE);
    mem[cur_ptr].hh.u.B0 = i;
    mem[cur_ptr].hh.u.B1 = 0;
    mem[cur_ptr].hh.v.RH = q;
    {
        register integer for_end;
        k = 1;
        for_end = (INDEX_NODE_SIZE - 1);
        if (k <= for_end)
            do
                mem[cur_ptr + k] = sa_null;
            while (k++ < for_end);
    }
}

void find_sa_element(small_number t, int32_t n, boolean w)
{
    memory_word *mem = zmem; int32_t q;
    small_number i;
    cur_ptr = sa_root[t];
    {
        if (cur_ptr == MIN_HALFWORD) {

            if (w)
                goto lab45;
            else
                return;
        }
    }
    q = cur_ptr;
    i = n / 262144L;
    if (odd(i))
        cur_ptr = mem[q + (i / 2) + 1].hh.v.RH;
    else
        cur_ptr = mem[q + (i / 2) + 1].hh.v.LH;
    {
        if (cur_ptr == MIN_HALFWORD) {

            if (w)
                goto lab46;
            else
                return;
        }
    }
    q = cur_ptr;
    i = (n / 4096) % 64;
    if (odd(i))
        cur_ptr = mem[q + (i / 2) + 1].hh.v.RH;
    else
        cur_ptr = mem[q + (i / 2) + 1].hh.v.LH;
    {
        if (cur_ptr == MIN_HALFWORD) {

            if (w)
                goto lab47;
            else
                return;
        }
    }
    q = cur_ptr;
    i = (n / 64) % 64;
    if (odd(i))
        cur_ptr = mem[q + (i / 2) + 1].hh.v.RH;
    else
        cur_ptr = mem[q + (i / 2) + 1].hh.v.LH;
    {
        if (cur_ptr == MIN_HALFWORD) {

            if (w)
                goto lab48;
            else
                return;
        }
    }
    q = cur_ptr;
    i = n % 64;
    if (odd(i))
        cur_ptr = mem[q + (i / 2) + 1].hh.v.RH;
    else
        cur_ptr = mem[q + (i / 2) + 1].hh.v.LH;
    if ((cur_ptr == MIN_HALFWORD) && w)
        goto lab49;
    return;
 lab45:                        /*not_found */ new_index(t, MIN_HALFWORD);
    sa_root[t] = cur_ptr;
    q = cur_ptr;
    i = n / 262144L;
 lab46:                        /*not_found1 */ new_index(i, q);
    {
        if (odd(i))
            mem[q + (i / 2) + 1].hh.v.RH = cur_ptr;
        else
            mem[q + (i / 2) + 1].hh.v.LH = cur_ptr;
        mem[q].hh.u.B1++;
    }
    q = cur_ptr;
    i = (n / 4096) % 64;
 lab47:                        /*not_found2 */ new_index(i, q);
    {
        if (odd(i))
            mem[q + (i / 2) + 1].hh.v.RH = cur_ptr;
        else
            mem[q + (i / 2) + 1].hh.v.LH = cur_ptr;
        mem[q].hh.u.B1++;
    }
    q = cur_ptr;
    i = (n / 64) % 64;
 lab48:                        /*not_found3 */ new_index(i, q);
    {
        if (odd(i))
            mem[q + (i / 2) + 1].hh.v.RH = cur_ptr;
        else
            mem[q + (i / 2) + 1].hh.v.LH = cur_ptr;
        mem[q].hh.u.B1++;
    }
    q = cur_ptr;
    i = n % 64;
 lab49:/*not_found4 *//*1608: */ if (t == MARK_VAL) {
        cur_ptr = get_node(MARK_CLASS_NODE_SIZE);
        mem[cur_ptr + 1] = sa_null;
        mem[cur_ptr + 2] = sa_null;
        mem[cur_ptr + 3] = sa_null;
    } else {

        if (t <= DIMEN_VAL) {
            cur_ptr = get_node(WORD_NODE_SIZE);
            mem[cur_ptr + 2].cint = 0;
            mem[cur_ptr + 1].hh.v.RH = n;
        } else {

            cur_ptr = get_node(POINTER_NODE_SIZE);
            if (t <= MU_VAL) {
                mem[cur_ptr + 1].hh.v.RH = 0;
                mem[0].hh.v.RH++;
            } else
                mem[cur_ptr + 1].hh.v.RH = MIN_HALFWORD;
        }
        mem[cur_ptr + 1].hh.v.LH = MIN_HALFWORD;
    }
    mem[cur_ptr].hh.u.B0 = 64 * t + i;
    mem[cur_ptr].hh.u.B1 = 1 /*level_one *//*:1608 */ ;
    mem[cur_ptr].hh.v.RH = q;
    {
        if (odd(i))
            mem[q + (i / 2) + 1].hh.v.RH = cur_ptr;
        else
            mem[q + (i / 2) + 1].hh.v.LH = cur_ptr;
        mem[q].hh.u.B1++;
    }
}

void expand(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t t;
    boolean b;
    int32_t p, q, r;
    integer j;
    integer cv_backup;
    small_number cvl_backup, radix_backup, co_backup;
    int32_t backup_backup;
    small_number save_scanner_status;

    expand_depth_count++;
    if (expand_depth_count >= expand_depth)
        overflow(S(expansion_depth), expand_depth);
    cv_backup = cur_val;
    cvl_backup = cur_val_level;
    radix_backup = radix;
    co_backup = cur_order;
    backup_backup = mem[mem_top - 13].hh.v.RH;
 lab21:/*reswitch */ if (cur_cmd < CALL) {   /*384: */
        if (INTPAR(tracing_commands) > 1)
            show_cur_cmd_chr();
        switch (cur_cmd) {
        case 112:
            {
                t = cur_chr % 5;
                if (cur_chr >= 5)
                    scan_register_num();
                else
                    cur_val = 0;
                if (cur_val == 0)
                    cur_ptr = cur_mark[t];
                else {          /*1612: */

                    find_sa_element(MARK_VAL, cur_val, false);
                    if (cur_ptr != MIN_HALFWORD) {

                        if (odd(t))
                            cur_ptr = mem[cur_ptr + (t / 2) + 1].hh.v.RH;
                        else
                            cur_ptr = mem[cur_ptr + (t / 2) + 1].hh.v.LH;
                    }
                }
                if (cur_ptr != MIN_HALFWORD)
                    begin_token_list(cur_ptr, MARK_TEXT);
            }
            break;
        case 104:
            if (cur_chr == 0) { /*385: */
                get_token();
                t = cur_tok;
                get_token();
                if (cur_cmd > MAX_COMMAND)
                    expand();
                else
                    back_input();
                cur_tok = t;
                back_input();
            } else {            /*1553: */

                get_token();
                if ((cur_cmd == IF_TEST) && (cur_chr != IF_CASE_CODE)) {
                    cur_chr = cur_chr + 32;
                    goto lab21;
                }
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(You_can_t_use__));
                }
                print_esc(S(unless));
                print(S(__before__));
                print_cmd_chr(cur_cmd, cur_chr);
                print_char(39 /*"'" */ );
                {
                    help_ptr = 1;
                    help_line[0] = S(Continue__and_I_ll_forget_th/*at it ever happened.*/);
                }
                back_error();
            }
            break;
        case 105:
            if (cur_chr == 0) { /*386: */
                save_scanner_status = scanner_status;
                scanner_status = NORMAL;
                get_token();
                scanner_status = save_scanner_status;
                t = cur_tok;
                back_input();
                if (t >= CS_TOKEN_FLAG) {
                    p = get_avail();
                    mem[p].hh.v.LH = (CS_TOKEN_FLAG + 2243235);
                    mem[p].hh.v.RH = cur_input.loc;
                    cur_input.start = p;
                    cur_input.loc = p;
                }
            } else {            /*387: */

                save_scanner_status = scanner_status;
                scanner_status = NORMAL;
                get_token();
                scanner_status = save_scanner_status;
                if (cur_cs < HASH_BASE)
                    cur_cs = prim_lookup(cur_cs - 257);
                else
                    cur_cs = prim_lookup(hash[cur_cs].v.RH);
                if (cur_cs != UNDEFINED_PRIMITIVE) {
                    t = prim_eqtb[cur_cs].hh.u.B0;
                    if (t > MAX_COMMAND) {
                        cur_cmd = t;
                        cur_chr = prim_eqtb[cur_cs].hh.v.RH;
                        cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                        cur_cs = 0;
                        goto lab21;
                    } else {

                        back_input();
                        p = get_avail();
                        mem[p].hh.v.LH = (CS_TOKEN_FLAG + 2243237);
                        mem[p].hh.v.RH = cur_input.loc;
                        cur_input.loc = p;
                        cur_input.start = p;
                    }
                }
            }
            break;
        case 109:
            {
                r = get_avail();
                p = r;
                b = is_in_csname;
                is_in_csname = true;
                do {
                    get_x_token();
                    if (cur_cs == 0) {
                        q = get_avail();
                        mem[p].hh.v.RH = q;
                        mem[q].hh.v.LH = cur_tok;
                        p = q;
                    }
                } while (!(cur_cs != 0));
                if (cur_cmd != END_CS_NAME) {  /*391: */
                    {
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(Missing_));
                    }
                    print_esc(S(endcsname));
                    print(S(_inserted));
                    {
                        help_ptr = 2;
                        help_line[1] = S(The_control_sequence_marked_/*<to be read again> should*/);
                        help_line[0] = S(not_appear_between__csname_a/*nd \endcsname.*/);
                    }
                    back_error();
                }
                is_in_csname = b;
                j = first;
                p = mem[r].hh.v.RH;
                while (p != MIN_HALFWORD) {

                    if (j >= max_buf_stack) {
                        max_buf_stack = j + 1;
                        if (max_buf_stack == buf_size)
                            overflow(S(buffer_size), buf_size);
                    }
                    buffer[j] = mem[p].hh.v.LH % MAX_CHAR_VAL;
                    j++;
                    p = mem[p].hh.v.RH;
                }
                if ((j > first + 1) || (buffer[first] > 65535L)) {
                    no_new_control_sequence = false;
                    cur_cs = id_lookup(first, j - first);
                    no_new_control_sequence = true;
                } else if (j == first)
                    cur_cs = NULL_CS;
                else
                    cur_cs = SINGLE_BASE + buffer[first] /*:392 */ ;
                flush_list(r);
                if (eqtb[cur_cs].hh.u.B0 == UNDEFINED_CS) {
                    eq_define(cur_cs, RELAX, TOO_BIG_USV);
                }
                cur_tok = cur_cs + CS_TOKEN_FLAG;
                back_input();
            }
            break;
        case 110:
            conv_toks();
            break;
        case 111:
            ins_the_toks();
            break;
        case 107:
            conditional();
            break;
        case 108:
            {
                if (INTPAR(tracing_ifs) > 0) {

                    if (INTPAR(tracing_commands) <= 1)
                        show_cur_cmd_chr();
                }
                if (cur_chr > if_limit) {

                    if (if_limit == IF_CODE)
                        insert_relax();
                    else {

                        {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Extra_));
                        }
                        print_cmd_chr(FI_OR_ELSE, cur_chr);
                        {
                            help_ptr = 1;
                            help_line[0] = S(I_m_ignoring_this__it_doesn_/*t match any \if.*/);
                        }
                        error();
                    }
                } else {

                    while (cur_chr != FI_CODE)
                        pass_text();
                    {
                        if (if_stack[in_open] == cond_ptr)
                            if_warning();
                        p = cond_ptr;
                        if_line = mem[p + 1].cint;
                        cur_if = mem[p].hh.u.B1;
                        if_limit = mem[p].hh.u.B0;
                        cond_ptr = mem[p].hh.v.RH;
                        free_node(p, IF_NODE_SIZE);
                    }
                }
            }
            break;
        case INPUT:
            if (cur_chr == 1) /* \endinput */
                force_eof = true /*1537: */ ;
            else if (cur_chr == 2) /* \scantokens */
                pseudo_start();
            else if (name_in_progress)
                insert_relax();
            else /* \input */
                start_input();
            break;
        default:
            {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Undefined_control_sequence));
                }
                {
                    help_ptr = 5;
                    help_line[4] = S(The_control_sequence_at_the_/*end of the top line*/);
                    help_line[3] = S(of_your_error_message_was_ne/*ver \def'ed. If you have*/);
                    help_line[2] = S(misspelled_it__e_g_____hobx_/*), type `I' and the correct*/);
                    help_line[1] = S(spelling__e_g____I_hbox____O/*therwise just continue,*/);
                    help_line[0] = S(and_I_ll_forget_about_whatev/*er was undefined.*/);
                }
                error();
            }
            break;
        }
    } else if (cur_cmd < END_TEMPLATE)
        macro_call();
    else {                      /*393: */

        cur_tok = (CS_TOKEN_FLAG + 2243232);
        back_input();
    }
    cur_val = cv_backup;
    cur_val_level = cvl_backup;
    radix = radix_backup;
    cur_order = co_backup;
    mem[mem_top - 13].hh.v.RH = backup_backup;
    expand_depth_count--;
}

void get_x_token(void)
{
 lab20:     /*restart */ get_next();
    if (cur_cmd <= MAX_COMMAND)
        goto done;
    if (cur_cmd >= CALL) {

        if (cur_cmd < END_TEMPLATE)
            macro_call();
        else {

            cur_cs = FROZEN_ENDV;
            cur_cmd = ENDV;
            goto done;
        }
    } else
        expand();
    goto lab20;
done:
    if (cur_cs == 0)
        cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
    else
        cur_tok = CS_TOKEN_FLAG + cur_cs;
}

void x_token(void)
{
    while (cur_cmd > MAX_COMMAND) {

        expand();
        get_next();
    }
    if (cur_cs == 0)
        cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
    else
        cur_tok = CS_TOKEN_FLAG + cur_cs;
}

void scan_left_brace(void)
{
    do {
        get_x_token();
    } while (!((cur_cmd != SPACER) && (cur_cmd != RELAX) /*:422 */ ));

    if (cur_cmd != LEFT_BRACE) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(65980L /*"Missing _ inserted" */ );
        }
        {
            help_ptr = 4;
            help_line[3] = S(A_left_brace_was_mandatory_h/*ere, so I've put one in.*/);
            help_line[2] = S(You_might_want_to_delete_and/*/or insert some corrections*/);
            help_line[1] = S(so_that_I_will_find_a_matchi/*ng right brace soon.*/);
            help_line[0] = 65984L /*"(If you're confused by all this, try typing `I_' now.)" */ ;
        }
        back_error();
        cur_tok = (LEFT_BRACE_TOKEN + 123);
        cur_cmd = LEFT_BRACE;
        cur_chr = 123 /*"_" */ ;
        align_state++;
    }
}

void scan_optional_equals(void)
{
    do {
        get_x_token();
    } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
    if (cur_tok != (OTHER_TOKEN + 61))
        back_input();
}

boolean scan_keyword(str_number s)
{
    register boolean Result;
    memory_word *mem = zmem; int32_t p;
    int32_t q;
    pool_pointer k;
    p = mem_top - 13;
    mem[p].hh.v.RH = MIN_HALFWORD;
    if (s < TOO_BIG_CHAR) {
        while (true) {

            get_x_token();
            if ((cur_cs == 0) && ((cur_chr == s) || (cur_chr == s - 32))) {
                {
                    q = get_avail();
                    mem[p].hh.v.RH = q;
                    mem[q].hh.v.LH = cur_tok;
                    p = q;
                }
                flush_list(mem[mem_top - 13].hh.v.RH);
                Result = true;
                return Result;
            } else if ((cur_cmd != SPACER) || (p != mem_top - 13)) {
                back_input();
                if (p != mem_top - 13)
                    begin_token_list(mem[mem_top - 13].hh.v.RH, BACKED_UP);
                Result = false;
                return Result;
            }
        }
    }
    k = str_start[(s) - 65536L];
    while (k < str_start[(s + 1) - 65536L]) {

        get_x_token();
        if ((cur_cs == 0) && ((cur_chr == str_pool[k]) || (cur_chr == str_pool[k] - 32))) {
            {
                q = get_avail();
                mem[p].hh.v.RH = q;
                mem[q].hh.v.LH = cur_tok;
                p = q;
            }
            k++;
        } else if ((cur_cmd != SPACER) || (p != mem_top - 13)) {
            back_input();
            if (p != mem_top - 13)
                begin_token_list(mem[mem_top - 13].hh.v.RH, BACKED_UP);
            Result = false;
            return Result;
        }
    }
    flush_list(mem[mem_top - 13].hh.v.RH);
    Result = true;
    return Result;
}

void mu_error(void)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Incompatible_glue_units));
    }
    {
        help_ptr = 1;
        help_line[0] = S(I_m_going_to_assume_that_1mu/*=1pt when they're mixed.*/);
    }
    error();
}

void scan_glyph_number(internal_font_number f)
{
    if (scan_keyword(47 /*"/" */ )) {
        scan_and_pack_name();
        {
            cur_val = map_glyph_to_index(f);
            cur_val_level = INT_VAL;
        }
    } else if (scan_keyword(117 /*"u" */ )) {
        scan_char_num();
        {
            cur_val = map_char_to_glyph(f, cur_val);
            cur_val_level = INT_VAL;
        }
    } else
        scan_int();
}

void scan_char_class(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > CHAR_CLASS_LIMIT)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_character_class));
        }
        {
            help_ptr = 2;
            help_line[1] = S(A_character_class_must_be_be/*tween 0 and 4096.*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_char_class_not_ignored(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > CHAR_CLASS_LIMIT)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_character_class));
        }
        {
            help_ptr = 2;
            help_line[1] = S(A_class_for_inter_character_/*transitions must be between 0 and 4095.*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_eight_bit_int(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > 255)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_register_code));
        }
        {
            help_ptr = 2;
            help_line[1] = S(A_register_code_or_char_clas/*s must be between 0 and 255.*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_usv_num(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > BIGGEST_USV)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_character_code));
        }
        {
            help_ptr = 2;
            help_line[1] = S(A_Unicode_scalar_value_must_/*be between 0 and "10FFFF.*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_char_num(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > BIGGEST_CHAR)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_character_code));
        }
        {
            help_ptr = 2;
            help_line[1] = S(A_character_number_must_be_b/*etween 0 and 65535.*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_xetex_math_char_int(void)
{
    scan_int();
    if (math_char(cur_val) == ACTIVE_MATH_CHAR) {
        if (cur_val != ACTIVE_MATH_CHAR) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Bad_active_XeTeX_math_code));
            }
            {
                help_ptr = 2;
                help_line[1] = S(Since_I_ignore_class_and_fam/*ily for active math chars,*/);
                help_line[0] = S(I_changed_this_one_to__1FFFF/*F.*/);
            }
            int_error(cur_val);
            cur_val = ACTIVE_MATH_CHAR;
        }
    } else if (math_char(cur_val) > BIGGEST_USV) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_XeTeX_math_character_cod/*e*/));
        }
        {
            help_ptr = 2;
            help_line[1] = S(Since_I_expected_a_character/* number between 0 and "10FFFF,*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_math_class_int(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > 7)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_math_class));
        }
        {
            help_ptr = 2;
            help_line[1] = S(Since_I_expected_to_read_a_n/*umber between 0 and 7,*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_math_fam_int(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > (NUMBER_MATH_FAMILIES - 1))) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_math_family));
        }
        {
            help_ptr = 2;
            help_line[1] = S(Since_I_expected_to_read_a_n_Z1/*"Since I expected to read a number between 0 and 255,"*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_four_bit_int(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > 15)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_number));
        }
        {
            help_ptr = 2;
            help_line[1] = S(Since_I_expected_to_read_a_n_Z2); /* ... "between 0 and 15" */
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_fifteen_bit_int(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > 32767)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_mathchar));
        }
        {
            help_ptr = 2;
            help_line[1] = S(A_mathchar_number_must_be_be_Z1/*"A mathchar number must be between 0 and 32767."*/);
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_delimiter_int(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > 134217727L)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_delimiter_code));
        }
        {
            help_ptr = 2;
            help_line[1] = 66044L /*"A numeric delimiter code must be between 0 and 2^_27_-1." */ ;
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_register_num(void)
{
    scan_int();
    if ((cur_val < 0) || (cur_val > max_reg_num)) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_register_code));
        }
        {
            help_ptr = 2;
            help_line[1] = max_reg_help_line;
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void scan_four_bit_int_or_18(void)
{
    scan_int();
    if ((cur_val < 0) || ((cur_val > 15) && (cur_val != 18))) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_number));
        }
        {
            help_ptr = 2;
            help_line[1] = S(Since_I_expected_to_read_a_n_Z2); /* ... "between 0 and 15" */
            help_line[0] = S(I_changed_this_one_to_zero_);
        }
        int_error(cur_val);
        cur_val = 0;
    }
}

void get_x_or_protected(void)
{
    memory_word *mem = zmem; while (true) {

        get_token();
        if (cur_cmd <= MAX_COMMAND)
            return;
        if ((cur_cmd >= CALL) && (cur_cmd < END_TEMPLATE)) {

            if (mem[mem[cur_chr].hh.v.RH].hh.v.LH == PROTECTED_TOKEN)
                return;
        }
        expand();
    }
}


integer
effective_char(boolean err_p, internal_font_number f, uint16_t c)
{
    if (!xtx_ligature_present && font_mapping[f] != NULL)
        c = apply_tfm_font_mapping(font_mapping[f], c);

    xtx_ligature_present = false;
    return c;
}


void scan_font_ident(void)
{
    CACHE_THE_EQTB;
    internal_font_number f;
    int32_t m;

    do {
        get_x_token();
    } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));

    if (cur_cmd == DEF_FONT)
        f = eqtb[CUR_FONT_LOC].hh.v.RH;
    else if (cur_cmd == SET_FONT)
        f = cur_chr;
    else if (cur_cmd == DEF_FAMILY) {
        m = cur_chr;
        scan_math_fam_int();
        f = eqtb[m + cur_val].hh.v.RH;
    } else {

        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Missing_font_identifier));
        }
        {
            help_ptr = 2;
            help_line[1] = S(I_was_looking_for_a_control_/*sequence whose*/);
            help_line[0] = S(current_meaning_has_been_def/*ined by \font.*/);
        }
        back_error();
        f = FONT_BASE;
    }
    cur_val = f;
}

void find_font_dimen(boolean writing)
{
    internal_font_number f;
    integer n;
    scan_int();
    n = cur_val;
    scan_font_ident();
    f = cur_val;
    if (n <= 0)
        cur_val = fmem_ptr;
    else {

        if (writing && (n <= SPACE_SHRINK_CODE) && (n >= SPACE_CODE) && (font_glue[f] != MIN_HALFWORD)) {
            delete_glue_ref(font_glue[f]);
            font_glue[f] = MIN_HALFWORD;
        }
        if (n > font_params[f]) {

            if (f < font_ptr)
                cur_val = fmem_ptr;
            else {              /*599: */

                do {
                    if (fmem_ptr == font_mem_size)
                        overflow(S(font_memory), font_mem_size);
                    font_info[fmem_ptr].cint = 0;
                    fmem_ptr++;
                    font_params[f]++;
                } while (!(n == font_params[f]));
                cur_val = fmem_ptr - 1;
            }
        } else
            cur_val = n + param_base[f];
    }
    if (cur_val == fmem_ptr) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Font_));
        }
        print_esc(hash[FONT_ID_BASE + f].v.RH);
        print(S(_has_only_));
        print_int(font_params[f]);
        print(S(_fontdimen_parameters));
        {
            help_ptr = 2;
            help_line[1] = S(To_increase_the_number_of_fo/*nt parameters, you must*/);
            help_line[0] = S(use__fontdimen_immediately_a/*fter the \font is loaded.*/);
        }
        error();
    }
}


void
scan_something_internal(small_number level, boolean negative)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t m;
    integer n, k, kk;
    int32_t q, r;
    int32_t tx;
    four_quarters i;
    integer p;

    m = cur_chr;

    switch (cur_cmd) {
    case DEF_CODE:
        scan_usv_num();
        if (m == MATH_CODE_BASE) {
            cur_val1 = MATH_CODE(cur_val);
            if (math_char(cur_val1) == ACTIVE_MATH_CHAR) {
                cur_val1 = 0x8000;
            } else if (math_class(cur_val1) > 7 || math_fam(cur_val1) > 15 || math_char(cur_val1) > 255) {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Extended_mathchar_used_as_ma/*thchar*/));
                help_ptr = 2;
                help_line[1] = S(A_mathchar_number_must_be_be/*tween 0 and "7FFF.*/);
                help_line[0] = S(I_changed_this_one_to_zero_);
                int_error(cur_val1);
                cur_val1 = 0;
            }

            cur_val1 = math_class(cur_val1) * 0x1000 + math_fam(cur_val1) * 0x100 + math_char(cur_val1);
            cur_val = cur_val1;
            cur_val_level = INT_VAL;
        } else if (m == DEL_CODE_BASE) {
            cur_val1 = DEL_CODE(cur_val);
            if (cur_val1 >= 0x40000000) {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Extended_delcode_used_as_del/*code*/));
                help_ptr = 2;
                help_line[1] = S(A_delimiter_code_must_be_bet/*ween 0 and "7FFFFFF.*/);
                help_line[0] = S(I_changed_this_one_to_zero_);
                error();
                cur_val = 0;
                cur_val_level = INT_VAL;
            } else {
                cur_val = cur_val1;
                cur_val_level = INT_VAL;
            }
        } else if (m < SF_CODE_BASE) {
            cur_val = eqtb[m + cur_val].hh.v.RH;
            cur_val_level = INT_VAL;
        } else if (m < MATH_CODE_BASE) {
            cur_val = eqtb[m + cur_val].hh.v.RH % 65536L;
            cur_val_level = INT_VAL;
        } else {
            cur_val = eqtb[m + cur_val].cint;
            cur_val_level = INT_VAL;
        }
        break;

    case XETEX_DEF_CODE:
        scan_usv_num();

        if (m == SF_CODE_BASE) {
            cur_val = SF_CODE(cur_val) / 65536L;
            cur_val_level = INT_VAL;
        } else if (m == MATH_CODE_BASE) {
            cur_val = MATH_CODE(cur_val);
            cur_val_level = INT_VAL;
        } else if (m == MATH_CODE_BASE + 1) {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Can_t_use__Umathcode_as_a_nu/*mber (try \Umathcodenum)*/));
            help_ptr = 2;
            help_line[1] = S(_Umathcode_is_for_setting_a_/*mathcode from separate values;*/);
            help_line[0] = S(use__Umathcodenum_to_access_/*them as single values.*/);
            error();
            cur_val = 0;
            cur_val_level = INT_VAL;
        } else if (m == DEL_CODE_BASE) {
            cur_val = DEL_CODE(cur_val);
            cur_val_level = INT_VAL;
        } else {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Can_t_use__Udelcode_as_a_num/*ber (try \Udelcodenum)*/));
            help_ptr = 2;
            help_line[1] = S(_Udelcode_is_for_setting_a_d/*elcode from separate values;*/);
            help_line[0] = S(use__Udelcodenum_to_access_t/*hem as single values.*/);
            error();
            cur_val = 0;
            cur_val_level = INT_VAL;
        }
        break;

    case TOKS_REGISTER:
    case ASSIGN_TOKS:
    case DEF_FAMILY:
    case SET_FONT:
    case DEF_FONT:
        if (level != TOK_VAL) {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Missing_number__treated_as_z/*ero*/));
            help_ptr = 3;
            help_line[2] = S(A_number_should_have_been_he/*re; I inserted `0'.*/);
            help_line[1] = S(_If_you_can_t_figure_out_why/* I needed to see a number,*/);
            help_line[0] = S(look_up__weird_error__in_the/* index to The TeXbook.)*/);
            back_error();
            cur_val = 0;
            cur_val_level = DIMEN_VAL;
        } else if (cur_cmd <= ASSIGN_TOKS) {
            if (cur_cmd < ASSIGN_TOKS) {
                if (m == 0) {
                    scan_register_num();
                    if (cur_val < 256) {
                        cur_val = TOKS_REG(cur_val);
                    } else {
                        find_sa_element(TOK_VAL, cur_val, false);
                        if (cur_ptr == MIN_HALFWORD)
                            cur_val = MIN_HALFWORD;
                        else
                            cur_val = mem[cur_ptr + 1].hh.v.RH;
                    }
                } else {
                    cur_val = mem[m + 1].hh.v.RH;
                }
            } else if (cur_chr == LOCAL_BASE + LOCAL__xetex_inter_char) {
                scan_char_class_not_ignored();
                cur_ptr = cur_val;
                scan_char_class_not_ignored();
                find_sa_element(INTER_CHAR_VAL, cur_ptr * CHAR_CLASS_LIMIT + cur_val, false);
                if (cur_ptr == MIN_HALFWORD)
                    cur_val = MIN_HALFWORD;
                else
                    cur_val = mem[cur_ptr + 1].hh.v.RH;
            } else {
                cur_val = eqtb[m].hh.v.RH;
            }
            cur_val_level = TOK_VAL;
        } else {
            back_input();
            scan_font_ident();
            cur_val = FONT_ID_BASE + cur_val;
            cur_val_level = IDENT_VAL;
        }
        break;

    case ASSIGN_INT:
        cur_val = eqtb[m].cint;
        cur_val_level = INT_VAL;
        break;

    case ASSIGN_DIMEN:
        cur_val = eqtb[m].cint;
        cur_val_level = DIMEN_VAL;
        break;

    case ASSIGN_GLUE:
        cur_val = eqtb[m].hh.v.RH;
        cur_val_level = GLUE_VAL;
        break;

    case ASSIGN_MU_GLUE:
        cur_val = eqtb[m].hh.v.RH;
        cur_val_level = MU_VAL;
        break;

    case SET_AUX:
        if (abs(cur_list.mode) != m) {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Improper_));
            print_cmd_chr(SET_AUX, m);
            help_ptr = 4;
            help_line[3] = S(You_can_refer_to__spacefacto/*r only in horizontal mode;*/);
            help_line[2] = S(you_can_refer_to__prevdepth_/*only in vertical mode; and*/);
            help_line[1] = S(neither_of_these_is_meaningf/*ul inside \write. So*/);
            help_line[0] = S(I_m_forgetting_what_you_said/* and using zero instead.*/);
            error();

            if (level != TOK_VAL) {
                cur_val = 0;
                cur_val_level = DIMEN_VAL;
            } else {
                cur_val = 0;
                cur_val_level = INT_VAL;
            }
        } else if (m == VMODE) {
            cur_val = cur_list.aux.cint;
            cur_val_level = DIMEN_VAL;
        } else {
            cur_val = cur_list.aux.hh.v.LH;
            cur_val_level = INT_VAL;
        }
        break;

    case SET_PREV_GRAF:
        if (cur_list.mode == 0) {
            cur_val = 0;
            cur_val_level = INT_VAL;
        } else {
            nest[nest_ptr] = cur_list;
            p = nest_ptr;
            while (abs(nest[p].mode) != VMODE)
                p--;

            cur_val = nest[p].pg;
            cur_val_level = INT_VAL;
        }
        break;

    case SET_PAGE_INT:
        if (m == 0)
            cur_val = dead_cycles;
        else if (m == 2)
            cur_val = interaction;
        else
            cur_val = insert_penalties;
        cur_val_level = INT_VAL;
        break;

    case SET_PAGE_DIMEN:
        if (page_contents == EMPTY && !output_active) {
            if (m == 0)
                cur_val = MAX_HALFWORD;
            else
                cur_val = 0;
        } else {
            cur_val = page_so_far[m];
        }

        cur_val_level = DIMEN_VAL;
        break;

    case SET_SHAPE:
        if (m > LOCAL_BASE + LOCAL__par_shape) { /*1654:*/
            scan_int();
            if (eqtb[m].hh.v.RH == MIN_HALFWORD || cur_val < 0) {
                cur_val = 0;
            } else {
                if (cur_val > mem[eqtb[m].hh.v.RH + 1].cint)
                    cur_val = mem[eqtb[m].hh.v.RH + 1].cint;
                cur_val = mem[eqtb[m].hh.v.RH + cur_val + 1].cint;
            }
        } else if (LOCAL(par_shape) == MIN_HALFWORD) {
            cur_val = 0;
        } else {
            cur_val = mem[LOCAL(par_shape)].hh.v.LH;
        }

        cur_val_level = INT_VAL;
        break;

    case SET_BOX_DIMEN:
        scan_register_num();

        if (cur_val < 256) {
            q = BOX_REG(cur_val);
        } else {
            find_sa_element(4, cur_val, false);
            if (cur_ptr == MIN_HALFWORD)
                q = MIN_HALFWORD;
            else
                q = mem[cur_ptr + 1].hh.v.RH;
        }

        if (q == MIN_HALFWORD)
            cur_val = 0;
        else
            cur_val = mem[q + m].cint;
        cur_val_level = DIMEN_VAL;
        break;

    case CHAR_GIVEN:
    case MATH_GIVEN:
        cur_val = cur_chr;
        cur_val_level = INT_VAL;
        break;

    case ASSIGN_FONT_DIMEN:
        find_font_dimen(false);
        font_info[fmem_ptr].cint = 0;
        cur_val = font_info[cur_val].cint;
        cur_val_level = DIMEN_VAL;
        break;

    case ASSIGN_FONT_INT:
        scan_font_ident();
        if (m == 0) {
            cur_val = hyphen_char[cur_val];
            cur_val_level = INT_VAL;
        } else if (m == 1) {
            cur_val = skew_char[cur_val];
            cur_val_level = INT_VAL;
        } else {
            n = cur_val;

            if (font_area[n] == AAT_FONT_FLAG || font_area[n] == OTGR_FONT_FLAG)
                scan_glyph_number(n);
            else
                scan_char_num();

            k = cur_val;
            switch (m) {
            case LP_CODE_BASE:
                cur_val = get_cp_code(n, k, LEFT_SIDE);
                cur_val_level = INT_VAL;
                break;
            case RP_CODE_BASE:
                cur_val = get_cp_code(n, k, RIGHT_SIDE);
                cur_val_level = INT_VAL;
                break;
            }
        }
        break;

    case REGISTER:
        if (m < 0 || m > 19) { /* 19 = "lo_mem_stat_max" */
            cur_val_level = (mem[m].hh.u.B0 / 64);
            if (cur_val_level < GLUE_VAL)
                cur_val = mem[m + 2].cint;
            else
                cur_val = mem[m + 1].hh.v.RH;
        } else {
            scan_register_num();
            cur_val_level = m;
            if (cur_val > 255) {
                find_sa_element(cur_val_level, cur_val, false);
                if (cur_ptr == MIN_HALFWORD)
                    cur_val = 0;
                else if (cur_val_level < GLUE_VAL)
                    cur_val = mem[cur_ptr + 2].cint;
                else
                    cur_val = mem[cur_ptr + 1].hh.v.RH;
            } else {
                switch (cur_val_level) {
                case INT_VAL:
                    cur_val = COUNT_REG(cur_val);
                    break;
                case DIMEN_VAL:
                    cur_val = SCALED_REG(cur_val);
                    break;
                case GLUE_VAL:
                    cur_val = SKIP_REG(cur_val);
                    break;
                case MU_VAL:
                    cur_val = MU_SKIP_REG(cur_val);
                    break;
                }
            }
        }
        break;

    case LAST_ITEM:
        if (m >= INPUT_LINE_NO_CODE) {
            if (m >= ETEX_GLUE) { /*1568:*/
                if (m < ETEX_MU) {
                    switch (m) { /*1595:*/
                    case MU_TO_GLUE_CODE:
                        scan_mu_glue();
                        break;
                    }
                    cur_val_level = GLUE_VAL;
                } else if (m < ETEX_EXPR) {
                    switch (m) { /*1596:*/
                    case GLUE_TO_MU_CODE:
                        scan_normal_glue();
                        break;
                    }
                    cur_val_level = MU_VAL;
                } else {
                    cur_val_level = m - ETEX_EXPR;
                    scan_expr();
                }

                while (cur_val_level > level) {
                    if (cur_val_level == GLUE_VAL) {
                        m = cur_val;
                        cur_val = mem[m + 1].cint;
                        delete_glue_ref(m);
                    } else if (cur_val_level == MU_VAL) {
                        mu_error();
                    }
                    cur_val_level--;
                }

                if (negative) {
                    if (cur_val_level >= GLUE_VAL) {
                        m = cur_val;
                        cur_val = new_spec(m);
                        delete_glue_ref(m);
                        mem[cur_val + 1].cint = -(integer) mem[cur_val + 1].cint;
                        mem[cur_val + 2].cint = -(integer) mem[cur_val + 2].cint;
                        mem[cur_val + 3].cint = -(integer) mem[cur_val + 3].cint;
                    } else {
                        cur_val = -(integer) cur_val;
                    }
                }
                return;
            }

            if (m >= XETEX_DIM) {
                switch (m) { /*1435:*/
                case XETEX_GLYPH_BOUNDS_CODE:
                    if (font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == AAT_FONT_FLAG ||
                        font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == OTGR_FONT_FLAG) {
                        scan_int();
                        n = cur_val;
                        if (n < 1 || n > 4) {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(__XeTeXglyphbounds_requires_/*an edge index from 1 to 4;*/));
                            print_nl(S(I_don_t_know_anything_about_/*edge */));
                            print_int(n);
                            error();
                            cur_val = 0;
                        } else {
                            scan_int();
                            cur_val = get_glyph_bounds(eqtb[CUR_FONT_LOC].hh.v.RH, n, cur_val);
                        }
                    } else {
                        not_native_font_error(LAST_ITEM, m, eqtb[CUR_FONT_LOC].hh.v.RH);
                        cur_val = 0;
                    }
                    break;

                case FONT_CHAR_WD_CODE:
                case FONT_CHAR_HT_CODE:
                case FONT_CHAR_DP_CODE:
                case FONT_CHAR_IC_CODE:
                    scan_font_ident();
                    q = cur_val;
                    scan_usv_num();
                    if (font_area[q] == AAT_FONT_FLAG || font_area[q] == OTGR_FONT_FLAG) {
                        switch (m) {
                        case FONT_CHAR_WD_CODE:
                            cur_val = getnativecharwd(q, cur_val);
                            break;
                        case FONT_CHAR_HT_CODE:
                            cur_val = getnativecharht(q, cur_val);
                            break;
                        case FONT_CHAR_DP_CODE:
                            cur_val = getnativechardp(q, cur_val);
                            break;
                        case FONT_CHAR_IC_CODE:
                            cur_val = getnativecharic(q, cur_val);
                            break;
                        }
                    } else {
                        if (font_bc[q] <= cur_val && font_ec[q] >= cur_val) {
                            i = font_info[char_base[q] + effective_char(true, q, cur_val)].qqqq;
                            switch (m) {
                            case FONT_CHAR_WD_CODE:
                                cur_val = font_info[width_base[q] + i.u.B0].cint;
                                break;
                            case FONT_CHAR_HT_CODE:
                                cur_val = font_info[height_base[q] + (i.u.B1) / 16].cint;
                                break;
                            case FONT_CHAR_DP_CODE:
                                cur_val = font_info[depth_base[q] + (i.u.B1) % 16].cint;
                                break;
                            case FONT_CHAR_IC_CODE:
                                cur_val = font_info[italic_base[q] + (i.u.B2) / 4].cint;
                                break;
                            }
                        } else {
                            cur_val = 0;
                        }
                    }
                    break;

                case PAR_SHAPE_LENGTH_CODE:
                case PAR_SHAPE_INDENT_CODE:
                case PAR_SHAPE_DIMEN_CODE:
                    q = cur_chr - PAR_SHAPE_LENGTH_CODE;
                    scan_int();
                    if (LOCAL(par_shape) == MIN_HALFWORD || cur_val <= 0) {
                        cur_val = 0;
                    } else {
                        if (q == 2) {
                            q = cur_val % 2;
                            cur_val = (cur_val + q) / 2;
                        }
                        if (cur_val > mem[LOCAL(par_shape)].hh.v.LH)
                            cur_val = mem[LOCAL(par_shape)].hh.v.LH;
                        cur_val = mem[LOCAL(par_shape) + 2 * cur_val - q].cint;
                    }
                    cur_val_level = DIMEN_VAL;
                    break;

                case GLUE_STRETCH_CODE:
                case GLUE_SHRINK_CODE:
                    scan_normal_glue();
                    q = cur_val;
                    if (m == GLUE_STRETCH_CODE)
                        cur_val = mem[q + 2].cint;
                    else
                        cur_val = mem[q + 3].cint;
                    delete_glue_ref(q);
                    break;
                }
                cur_val_level = DIMEN_VAL;
            } else { /* if(m >= XETEX_DIM) */
                switch (m) {
                case INPUT_LINE_NO_CODE:
                    cur_val = line;
                    break;

                case BADNESS_CODE:
                    cur_val = last_badness;
                    break;

                case PDF_SHELL_ESCAPE_CODE:
		    cur_val = 0; /* shellenabledp */
                    break;

                case ETEX_VERSION_CODE:
                    cur_val = ETEX_VERSION;
                    break;

                case XETEX_VERSION_CODE:
                    cur_val = XETEX_VERSION;
                    break;

                case XETEX_COUNT_GLYPHS_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG)
                        cur_val = aat_font_get(m - 14, font_layout_engine[n]);
                    else if (font_area[n] == OTGR_FONT_FLAG)
                        cur_val = ot_font_get(m - 14, font_layout_engine[n]);
                    else
                        cur_val = 0;
                    break;

                case XETEX_COUNT_FEATURES_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG)
                        cur_val = aat_font_get(m - 14, font_layout_engine[n]);
                    else if (font_area[n] == OTGR_FONT_FLAG && usingGraphite(font_layout_engine[n]))
                        cur_val = ot_font_get(m - 14, font_layout_engine[n]);
                    else
                        cur_val = 0;
                    break;

                case XETEX_VARIATION_CODE:
                case XETEX_VARIATION_MIN_CODE:
                case XETEX_VARIATION_MAX_CODE:
                case XETEX_VARIATION_DEFAULT_CODE:
                case XETEX_COUNT_VARIATIONS_CODE:
                    scan_font_ident();
                    n = cur_val;
                    cur_val = 0;
                    break;

                case XETEX_FEATURE_CODE_CODE:
                case XETEX_IS_EXCLUSIVE_FEATURE_CODE:
                case XETEX_COUNT_SELECTORS_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG) {
                        scan_int();
                        k = cur_val;
                        cur_val = aat_font_get_1(m - 14, font_layout_engine[n], k);
                    } else if (font_area[n] == OTGR_FONT_FLAG && usingGraphite(font_layout_engine[n])) {
                        scan_int();
                        k = cur_val;
                        cur_val = ot_font_get_1(m - 14, font_layout_engine[n], k);
                    } else {
                        not_aat_gr_font_error(LAST_ITEM, m, n);
                        cur_val = -1;
                    }
                    break;

                case XETEX_SELECTOR_CODE_CODE:
                case XETEX_IS_DEFAULT_SELECTOR_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG) {
                        scan_int();
                        k = cur_val;
                        scan_int();
                        cur_val = aat_font_get_2(m - 14, font_layout_engine[n], k, cur_val);
                    } else if (font_area[n] == OTGR_FONT_FLAG && usingGraphite(font_layout_engine[n])) {
                        scan_int();
                        k = cur_val;
                        scan_int();
                        cur_val = ot_font_get_2(m - 14, font_layout_engine[n], k, cur_val);
                    } else {
                        not_aat_gr_font_error(LAST_ITEM, m, n);
                        cur_val = -1;
                    }
                    break;

                case XETEX_FIND_VARIATION_BY_NAME_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG) {
                        scan_and_pack_name();
                        cur_val = aat_font_get_named(m - 14, font_layout_engine[n]);
                    } else {
                        not_aat_font_error(LAST_ITEM, m, n);
                        cur_val = -1;
                    }
                    break;

                case XETEX_FIND_FEATURE_BY_NAME_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG) {
                        scan_and_pack_name();
                        cur_val = aat_font_get_named(m - 14, font_layout_engine[n]);
                    } else if (font_area[n] == OTGR_FONT_FLAG && usingGraphite(font_layout_engine[n])) {
                        scan_and_pack_name();
                        cur_val = gr_font_get_named(m - 14, font_layout_engine[n]);
                    } else {
                        not_aat_gr_font_error(LAST_ITEM, m, n);
                        cur_val = -1;
                    }
                    break;

                case XETEX_FIND_SELECTOR_BY_NAME_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG) {
                        scan_int();
                        k = cur_val;
                        scan_and_pack_name();
                        cur_val = aat_font_get_named_1(m - 14, font_layout_engine[n], k);
                    } else if (font_area[n] == OTGR_FONT_FLAG && usingGraphite(font_layout_engine[n])) {
                        scan_int();
                        k = cur_val;
                        scan_and_pack_name();
                        cur_val = gr_font_get_named_1(m - 14, font_layout_engine[n], k);
                    } else {
                        not_aat_gr_font_error(LAST_ITEM, m, n);
                        cur_val = -1;
                    }
                    break;

                case XETEX_OT_COUNT_SCRIPTS_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == OTGR_FONT_FLAG && usingOpenType(font_layout_engine[n])) {
                        cur_val = ot_font_get(m - 14, font_layout_engine[n]);
                    } else {
                        cur_val = 0;
                    }
                    break;

                case XETEX_OT_COUNT_LANGUAGES_CODE:
                case XETEX_OT_SCRIPT_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == OTGR_FONT_FLAG && usingOpenType(font_layout_engine[n])) {
                        scan_int();
                        cur_val = ot_font_get_1(m - 14, font_layout_engine[n], cur_val);
                    } else {
                        not_ot_font_error(LAST_ITEM, m, n);
                        cur_val = -1;
                    }
                    break;

                case XETEX_OT_COUNT_FEATURES_CODE:
                case XETEX_OT_LANGUAGE_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == OTGR_FONT_FLAG && usingOpenType(font_layout_engine[n])) {
                        scan_int();
                        k = cur_val;
                        scan_int();
                        cur_val = ot_font_get_2(m - 14, font_layout_engine[n], k, cur_val);
                    } else {
                        not_ot_font_error(LAST_ITEM, m, n);
                        cur_val = -1;
                    }
                    break;

                case XETEX_OT_FEATURE_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == OTGR_FONT_FLAG && usingOpenType(font_layout_engine[n])) {
                        scan_int();
                        k = cur_val;
                        scan_int();
                        kk = cur_val;
                        scan_int();
                        cur_val = ot_font_get_3(m - 14, font_layout_engine[n], k, kk, cur_val);
                    } else {
                        not_ot_font_error(LAST_ITEM, m, n);
                        cur_val = -1;
                    }
                    break;

                case XETEX_MAP_CHAR_TO_GLYPH_CODE:
                    if (font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == AAT_FONT_FLAG ||
                        font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == OTGR_FONT_FLAG) {
                        scan_int();
                        n = cur_val;
                        cur_val = map_char_to_glyph(eqtb[CUR_FONT_LOC].hh.v.RH, n);
                    } else {
                        not_native_font_error(LAST_ITEM, m, eqtb[CUR_FONT_LOC].hh.v.RH);
                        cur_val = 0;
                    }
                    break;

                case XETEX_GLYPH_INDEX_CODE:
                    if (font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == AAT_FONT_FLAG ||
                        font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == OTGR_FONT_FLAG) {
                        scan_and_pack_name();
                        cur_val = map_glyph_to_index(eqtb[CUR_FONT_LOC].hh.v.RH);
                    } else {
                        not_native_font_error(LAST_ITEM, m, eqtb[CUR_FONT_LOC].hh.v.RH);
                        cur_val = 0;
                    }
                    break;

                case XETEX_FONT_TYPE_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG)
                        cur_val = 1;
                    else if (font_area[n] == OTGR_FONT_FLAG && usingOpenType(font_layout_engine[n]))
                        cur_val = 2;
                    else if (font_area[n] == OTGR_FONT_FLAG && usingGraphite(font_layout_engine[n]))
                        cur_val = 3;
                    else
                        cur_val = 0;
                    break;

                case XETEX_FIRST_CHAR_CODE:
                case XETEX_LAST_CHAR_CODE:
                    scan_font_ident();
                    n = cur_val;
                    if (font_area[n] == AAT_FONT_FLAG || font_area[n] == OTGR_FONT_FLAG) {
                        cur_val = get_font_char_range(n, m == XETEX_FIRST_CHAR_CODE);
                    } else {
                        if (m == XETEX_FIRST_CHAR_CODE)
                            cur_val = font_bc[n];
                        else
                            cur_val = font_ec[n];
                    }
                    break;

                case PDF_LAST_X_POS_CODE:
                    cur_val = pdf_last_x_pos;
                    break;

                case PDF_LAST_Y_POS_CODE:
                    cur_val = pdf_last_y_pos;
                    break;

                case XETEX_PDF_PAGE_COUNT_CODE:
                    scan_and_pack_name();
                    cur_val = count_pdf_file_pages();
                    break;

                case CURRENT_GROUP_LEVEL_CODE:
                    cur_val = cur_level - 1;
                    break;

                case CURRENT_GROUP_TYPE_CODE:
                    cur_val = cur_group;
                    break;

                case CURRENT_IF_LEVEL_CODE:
                    q = cond_ptr;
                    cur_val = 0;
                    while (q != MIN_HALFWORD) {
                        cur_val++;
                        q = mem[q].hh.v.RH;
                    }
                    break;

                case CURRENT_IF_TYPE_CODE:
                    if (cond_ptr == MIN_HALFWORD)
                        cur_val = 0;
                    else if (cur_if < UNLESS_CODE)
                        cur_val = cur_if + 1;
                    else
                        cur_val = -(integer) (cur_if - 31);
                    break;

                case CURRENT_IF_BRANCH_CODE:
                    if (if_limit == OR_CODE || if_limit == ELSE_CODE)
                        cur_val = 1;
                    else if (if_limit == FI_CODE)
                        cur_val = -1;
                    else
                        cur_val = 0;
                    break;

                case GLUE_STRETCH_ORDER_CODE:
                case GLUE_SHRINK_ORDER_CODE:
                    scan_normal_glue();
                    q = cur_val;
                    if (m == GLUE_STRETCH_ORDER_CODE)
                        cur_val = mem[q].hh.u.B0;
                    else
                        cur_val = mem[q].hh.u.B1;
                    delete_glue_ref(q);
                    break;
                }

                cur_val_level = INT_VAL;
            }
        } else {
            cur_val = 0;
            tx = cur_list.tail;

            if (tx < hi_mem_min) {
                if (mem[tx].hh.u.B0 == MATH_NODE && mem[tx].hh.u.B1 == END_M_CODE) {
                    r = cur_list.head;
                    do {
                        q = r;
                        r = mem[q].hh.v.RH;
                    } while (r != tx);
                    tx = q;
                }
            }

            if (cur_chr == LAST_NODE_TYPE_CODE) {
                cur_val_level = INT_VAL;
                if (tx == cur_list.head || cur_list.mode == 0)
                    cur_val = -1;
            } else {
                cur_val_level = cur_chr;
            }

            if (tx < hi_mem_min && cur_list.mode != 0)
                switch (cur_chr) {
                case INT_VAL:
                    if (mem[tx].hh.u.B0 == PENALTY_NODE)
                        cur_val = mem[tx + 1].cint;
                    break;
                case DIMEN_VAL:
                    if (mem[tx].hh.u.B0 == KERN_NODE)
                        cur_val = mem[tx + 1].cint;
                    break;
                case GLUE_VAL:
                    if (mem[tx].hh.u.B0 == GLUE_NODE) {
                        cur_val = mem[tx + 1].hh.v.LH;
                        if (mem[tx].hh.u.B1 == MU_GLUE)
                            cur_val_level = MU_VAL;
                    }
                    break;
                case LAST_NODE_TYPE_CODE:
                    if (mem[tx].hh.u.B0 <= UNSET_NODE)
                        cur_val = mem[tx].hh.u.B0 + 1;
                    else
                        cur_val = (UNSET_NODE + 2);
                    break;
            } else if (cur_list.mode == VMODE && tx == cur_list.head)
                switch (cur_chr) {
                case INT_VAL:
                    cur_val = last_penalty;
                    break;
                case DIMEN_VAL:
                    cur_val = last_kern;
                    break;
                case GLUE_VAL:
                    if (last_glue != MAX_HALFWORD)
                        cur_val = last_glue;
                    break;
                case LAST_NODE_TYPE_CODE:
                    cur_val = last_node_type;
                    break;
                }
        }
        break;

    default:
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(You_can_t_use__));
        print_cmd_chr(cur_cmd, cur_chr);
        print(S(__after_));
        print_esc(S(the));
        help_ptr = 1;
        help_line[0] = S(I_m_forgetting_what_you_said/* and using zero instead.*/);
        error();
        cur_val = 0;
        if (level != TOK_VAL) {
            cur_val_level = DIMEN_VAL;
        } else {
            cur_val_level = INT_VAL;
        }
        break;
    }

    while (cur_val_level > level) { /*447:*/
        if (cur_val_level == GLUE_VAL)
            cur_val = mem[cur_val + 1].cint;
        else if (cur_val_level == MU_VAL)
            mu_error();
        cur_val_level--;
    }

    if (negative) {
        if (cur_val_level >= GLUE_VAL) {
            cur_val = new_spec(cur_val);
            mem[cur_val + 1].cint = -(integer) mem[cur_val + 1].cint;
            mem[cur_val + 2].cint = -(integer) mem[cur_val + 2].cint;
            mem[cur_val + 3].cint = -(integer) mem[cur_val + 3].cint;
        } else {
            cur_val = -(integer) cur_val;
        }
    } else if (cur_val_level >= GLUE_VAL && cur_val_level <= MU_VAL) {
        mem[cur_val].hh.v.RH++;
    }
}


void
scan_int(void)
{
    boolean negative;
    integer m;
    small_number d;
    boolean vacuous;
    boolean OK_so_far;

    radix = 0;
    OK_so_far = true;
    negative = false;

    do { /*424:*/
        do {
            get_x_token();
        } while (cur_cmd == SPACER);

        if (cur_tok == OTHER_TOKEN + 45 /*"-" */ ) {
            negative = !negative;
            cur_tok = OTHER_TOKEN + 43 /*"+" */;
        }
    } while (cur_tok == OTHER_TOKEN + 43 /*"+" */);

    if (cur_tok == ALPHA_TOKEN) { /*460:*/
        get_token();

        if (cur_tok < CS_TOKEN_FLAG) {
            cur_val = cur_chr;
            if (cur_cmd <= RIGHT_BRACE) {
                if (cur_cmd == RIGHT_BRACE)
                    align_state++;
                else
                    align_state--;
            }
        } else if (cur_tok < CS_TOKEN_FLAG + SINGLE_BASE) {
            cur_val = cur_tok - (CS_TOKEN_FLAG + ACTIVE_BASE);
        } else {
            cur_val = cur_tok - (CS_TOKEN_FLAG + SINGLE_BASE);
        }

        if (cur_val > BIGGEST_USV) {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Improper_alphabetic_constant/**/));
            help_ptr = 2;
            help_line[1] = S(A_one_character_control_sequ/*ence belongs after a ` mark.*/);
            help_line[0] = S(So_I_m_essentially_inserting/* \0 here.*/);
            cur_val = 48 /*"0" */ ;
            back_error();
        } else { /*461:*/
            get_x_token();
            if (cur_cmd != SPACER)
                back_input();
        }
    } else if (cur_cmd >= MIN_INTERNAL && cur_cmd <= MAX_INTERNAL) {
        scan_something_internal(INT_VAL, false);
    } else { /*462:*/
        radix = 10;
        m = 0xCCCCCCC;

        if (cur_tok == OCTAL_TOKEN) {
            radix = 8;
            m = 0x10000000;
            get_x_token();
        } else if (cur_tok == HEX_TOKEN) {
            radix = 16;
            m = 0x8000000;
            get_x_token();
        }

        vacuous = true;
        cur_val = 0;

        while (true) {
            if (cur_tok < ZERO_TOKEN + radix && cur_tok >= ZERO_TOKEN && cur_tok <= ZERO_TOKEN + 9) {
                d = cur_tok - ZERO_TOKEN;
            } else if (radix == 16) {
                if (cur_tok <= A_TOKEN + 5 && cur_tok >= A_TOKEN)
                    d = cur_tok - A_TOKEN + 10;
                else if (cur_tok <= OTHER_A_TOKEN + 5 && cur_tok >= OTHER_A_TOKEN)
                    d = cur_tok - OTHER_A_TOKEN + 10;
                else
                    break;
            } else {
                break;
            }

            vacuous = false;

            if (cur_val >= m && (cur_val > m || d > 7 || radix != 10)) {
                if (OK_so_far) {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Number_too_big));
                    help_ptr = 2;
                    help_line[1] = S(I_can_only_go_up_to_21474836/*47='17777777777="7FFFFFFF,*/);
                    help_line[0] = S(so_I_m_using_that_number_ins/*tead of yours.*/);
                    error();
                    cur_val = TEX_INFINITY;
                    OK_so_far = false;
                }
            } else {
                cur_val = cur_val * radix + d;
            }

            get_x_token();
        } /*:463*/

        if (vacuous) { /*464:*/
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Missing_number__treated_as_z/*ero*/));
            help_ptr = 3;
            help_line[2] = S(A_number_should_have_been_he/*re; I inserted `0'.*/);
            help_line[1] = S(_If_you_can_t_figure_out_why/* I needed to see a number,*/);
            help_line[0] = S(look_up__weird_error__in_the/* index to The TeXbook.)*/);
            back_error();
        } else if (cur_cmd != SPACER) {
            back_input();
        }
    }

    if (negative)
        cur_val = -(integer) cur_val;
}


static scaled
round_decimals(small_number k)
{
    integer a = 0;

    while (k > 0) {
        k--;
        a = (a + dig[k] * 131072L) / 10;
    }

    return (a + 1) / 2;
}


void xetex_scan_dimen(boolean mu, boolean inf, boolean shortcut, boolean requires_units)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;;
    boolean negative;
    integer f;
    integer num, denom;
    small_number k, kk;
    int32_t p, q;
    scaled v;
    integer save_cur_val;

    f = 0;
    arith_error = false;
    cur_order = NORMAL;
    negative = false;
    if (!shortcut) {
        negative = false;
        do {
            /*424: */
            do {
                get_x_token();
            } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
            if (cur_tok == (OTHER_TOKEN + 45)) {
                negative = !negative;
                cur_tok = (OTHER_TOKEN + 43);
            }
        } while (!(cur_tok != 25165867L /*other_token 43 *//*:459 */ ));
        if ((cur_cmd >= MIN_INTERNAL) && (cur_cmd <= MAX_INTERNAL)) { /*468: */

            if (mu) {
                scan_something_internal(MU_VAL, false);
                if (cur_val_level >= GLUE_VAL) {
                    v = mem[cur_val + 1].cint;
                    delete_glue_ref(cur_val);
                    cur_val = v;
                }
                if (cur_val_level == MU_VAL)
                    goto lab89;
                if (cur_val_level != INT_VAL)
                    mu_error();
            } else {

                scan_something_internal(DIMEN_VAL, false);
                if (cur_val_level == DIMEN_VAL)
                    goto lab89;
            }
        } else {

            back_input();
            if (cur_tok == CONTINENTAL_POINT_TOKEN)
                cur_tok = POINT_TOKEN;
            if (cur_tok != POINT_TOKEN)
                scan_int();
            else {

                radix = 10;
                cur_val = 0;
            }
            if (cur_tok == CONTINENTAL_POINT_TOKEN)
                cur_tok = POINT_TOKEN;
            if ((radix == 10) && (cur_tok == POINT_TOKEN)) {    /*471: */
                k = 0;
                p = MIN_HALFWORD;
                get_token();
                while (true) {

                    get_x_token();
                    if ((cur_tok > (ZERO_TOKEN + 9)) || (cur_tok < ZERO_TOKEN))
                        goto lab31;
                    if (k < 17) {
                        q = get_avail();
                        mem[q].hh.v.RH = p;
                        mem[q].hh.v.LH = cur_tok - 25165872L;
                        p = q;
                        k++;
                    }
                }
 lab31:                        /*done1 */  {
                    register integer for_end;
                    kk = k;
                    for_end = 1;
                    if (kk >= for_end)
                        do {
                            dig[kk - 1] = mem[p].hh.v.LH;
                            q = p;
                            p = mem[p].hh.v.RH;
                            {
                                mem[q].hh.v.RH = avail;
                                avail = q;
                            }
                        }
                        while (kk-- > for_end);
                }
                f = round_decimals(k);
                if (cur_cmd != SPACER)
                    back_input();
            }
        }
    }
    if (cur_val < 0) {
        negative = !negative;
        cur_val = -(integer) cur_val;
    }
    if (requires_units) {
        if (inf) {              /*473: */

            if (scan_keyword(S(fil))) {
                cur_order = FIL;
                while (scan_keyword(108 /*"l" */ )) {

                    if (cur_order == FILLL) {
                        {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Illegal_unit_of_measure__));
                        }
                        print(S(replaced_by_filll_));
                        {
                            help_ptr = 1;
                            help_line[0] = S(I_dddon_t_go_any_higher_than/* filll.*/);
                        }
                        error();
                    } else
                        cur_order++;
                }
                goto lab88;
            }
        }
        save_cur_val = cur_val;
        do {
            get_x_token();
        } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
        if ((cur_cmd < MIN_INTERNAL) || (cur_cmd > MAX_INTERNAL))
            back_input();
        else {

            if (mu) {
                scan_something_internal(MU_VAL, false);
                if (cur_val_level >= GLUE_VAL) {
                    v = mem[cur_val + 1].cint;
                    delete_glue_ref(cur_val);
                    cur_val = v;
                }
                if (cur_val_level != MU_VAL)
                    mu_error();
            } else
                scan_something_internal(DIMEN_VAL, false);
            v = cur_val;
            goto lab40;
        }
        if (mu)
            goto lab45;
        if (scan_keyword(S(em)))
            v = ( /*577: */ font_info[QUAD_CODE + param_base[eqtb[CUR_FONT_LOC].hh.v.RH]].
                 cint /*:577 */ );
        else if (scan_keyword(S(ex)))
            v = ( /*578: */ font_info[X_HEIGHT_CODE + param_base[eqtb[CUR_FONT_LOC].hh.v.RH]].
                 cint /*:578 */ );
        else
            goto lab45;
        {
            get_x_token();
            if (cur_cmd != SPACER)
                back_input();
        }
 lab40:                        /*found */ cur_val = mult_and_add(save_cur_val, v, xn_over_d(v, f, 65536L), MAX_HALFWORD);
        goto lab89;
 lab45:                        /*not_found *//*:474 */ ;
        if (mu) {               /*475: */

            if (scan_keyword(S(mu)))
                goto lab88;
            else {

                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Illegal_unit_of_measure__));
                }
                print(S(mu_inserted_));
                {
                    help_ptr = 4;
                    help_line[3] = S(The_unit_of_measurement_in_m/*ath glue must be mu.*/);
                    help_line[2] = S(To_recover_gracefully_from_t/*his error, it's best to*/);
                    help_line[1] = S(delete_the_erroneous_units__/*e.g., type `2' to delete*/);
                    help_line[0] = S(two_letters___See_Chapter_27/* of The TeXbook.)*/);
                }
                error();
                goto lab88;
            }
        }
        if (scan_keyword(S(true))) {        /*476: */
            prepare_mag();
            if (INTPAR(mag) != 1000) {
                cur_val = xn_over_d(cur_val, 1000, INTPAR(mag));
                f = (1000 * f + 65536L * tex_remainder) / INTPAR(mag);
                cur_val = cur_val + (f / 65536L);
                f = f % 65536L;
            }
        }
        if (scan_keyword(S(pt)))
            goto lab88;
        if (scan_keyword(S(in))) {
            num = 7227;
            denom = 100;
        } else if (scan_keyword(S(pc))) {
            num = 12;
            denom = 1;
        } else if (scan_keyword(S(cm))) {
            num = 7227;
            denom = 254;
        } else if (scan_keyword(S(mm))) {
            num = 7227;
            denom = 2540;
        } else if (scan_keyword(S(bp))) {
            num = 7227;
            denom = 7200;
        } else if (scan_keyword(S(dd))) {
            num = 1238;
            denom = 1157;
        } else if (scan_keyword(S(cc))) {
            num = 14856;
            denom = 1157;
        } else if (scan_keyword(S(sp)))
            goto done;
        else {                  /*478: */

            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Illegal_unit_of_measure__));
            }
            print(S(pt_inserted_));
            {
                help_ptr = 6;
                help_line[5] = S(Dimensions_can_be_in_units_o/*f em, ex, in, pt, pc,*/);
                help_line[4] = S(cm__mm__dd__cc__bp__or_sp__b/*ut yours is a new one!*/);
                help_line[3] = S(I_ll_assume_that_you_meant_t/*o say pt, for printer's points.*/);
                help_line[2] = S(To_recover_gracefully_from_t/*his error, it's best to*/);
                help_line[1] = S(delete_the_erroneous_units__/*e.g., type `2' to delete*/);
                help_line[0] = S(two_letters___See_Chapter_27/* of The TeXbook.)*/);
            }
            error();
            goto lab32;
        }
        cur_val = xn_over_d(cur_val, num, denom);
        f = (num * f + 65536L * tex_remainder) / denom;
        cur_val = cur_val + (f / 65536L);
        f = f % 65536L;
 lab32:                        /*done2 *//*:477 */ ;
 lab88:                        /*attach_fraction */ if (cur_val >= 16384)
            arith_error = true;
        else
            cur_val = cur_val * 65536L + f;
 /*:472 */
    done:
        {
            get_x_token();
            if (cur_cmd != SPACER)
                back_input();
        }
    } else {

        if (cur_val >= 16384)
            arith_error = true;
        else
            cur_val = cur_val * 65536L + f;
    }
 lab89:                                                                        /*attach_sign */ if (arith_error || (abs(cur_val) >= 1073741824L)) {
                                                                                /*479: */
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Dimension_too_large));
        }
        {
            help_ptr = 2;
            help_line[1] = S(I_can_t_work_with_sizes_bigg/*er than about 19 feet.*/);
            help_line[0] = S(Continue_and_I_ll_use_the_la/*rgest value I can.*/);
        }
        error();
        cur_val = MAX_HALFWORD;
        arith_error = false;
    }
    if (negative)
        cur_val = -(integer) cur_val;
}

void scan_dimen(boolean mu, boolean inf, boolean shortcut)
{
    xetex_scan_dimen(mu, inf, shortcut, true);
}

void scan_decimal(void)
{
    xetex_scan_dimen(false, false, false, false);
}

void scan_glue(small_number level)
{
    memory_word *mem = zmem; boolean negative;
    int32_t q;
    boolean mu;
    mu = (level == MU_VAL);
    negative = false;
    do {
        /*424: */
        do {
            get_x_token();
        } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
        if (cur_tok == (OTHER_TOKEN + 45)) {
            negative = !negative;
            cur_tok = (OTHER_TOKEN + 43);
        }
    } while (!(cur_tok != 25165867L /*other_token 43 *//*:459 */ ));
    if ((cur_cmd >= MIN_INTERNAL) && (cur_cmd <= MAX_INTERNAL)) {
        scan_something_internal(level, negative);
        if (cur_val_level >= GLUE_VAL) {
            if (cur_val_level != level)
                mu_error();
            return;
        }
        if (cur_val_level == INT_VAL)
            scan_dimen(mu, false, true);
        else if (level == MU_VAL)
            mu_error();
    } else {

        back_input();
        scan_dimen(mu, false, false);
        if (negative)
            cur_val = -(integer) cur_val;
    }
    q = new_spec(0);
    mem[q + 1].cint = cur_val;
    if (scan_keyword(S(plus))) {
        scan_dimen(mu, true, false);
        mem[q + 2].cint = cur_val;
        mem[q].hh.u.B0 = cur_order;
    }
    if (scan_keyword(S(minus))) {
        scan_dimen(mu, true, false);
        mem[q + 3].cint = cur_val;
        mem[q].hh.u.B1 = cur_order;
    }
    cur_val = /*:481 */ q;
}

integer add_or_sub(integer x, integer y, integer max_answer, boolean negative)
{
    register integer Result;
    integer a;
    if (negative)
        y = -(integer) y;
    if (x >= 0) {

        if (y <= max_answer - x)
            a = x + y;
        else {

            arith_error = true;
            a = 0;
        }
    } else if (y >= -(integer) max_answer - x)
        a = x + y;
    else {

        arith_error = true;
        a = 0;
    }
    Result = a;
    return Result;
}

integer quotient(integer n, integer d)
{
    register integer Result;
    boolean negative;
    integer a;
    if (d == 0) {
        arith_error = true;
        a = 0;
    } else {

        if (d > 0)
            negative = false;
        else {

            d = -(integer) d;
            negative = true;
        }
        if (n < 0) {
            n = -(integer) n;
            negative = !negative;
        }
        a = n / d;
        n = n - a * d;
        d = n - d;
        if (d + n >= 0)
            a++;
        if (negative)
            a = -(integer) a;
    }
    Result = a;
    return Result;
}

integer fract(integer x, integer n, integer d, integer max_answer)
{
    register integer Result;
    boolean negative;
    integer a;
    integer f;
    integer h;
    integer r;
    integer t;
    if (d == 0)
        goto lab88;
    a = 0;
    if (d > 0)
        negative = false;
    else {

        d = -(integer) d;
        negative = true;
    }
    if (x < 0) {
        x = -(integer) x;
        negative = !negative;
    } else if (x == 0)
        goto done;
    if (n < 0) {
        n = -(integer) n;
        negative = !negative;
    }
    t = n / d;
    if (t > max_answer / x)
        goto lab88;
    a = t * x;
    n = n - t * d;
    if (n == 0)
        goto lab40;
    t = x / d;
    if (t > (max_answer - a) / n)
        goto lab88;
    a = a + t * n;
    x = x - t * d;
    if (x == 0)
        goto lab40;
    if (x < n) {
        t = x;
        x = n;
        n = t;
    }
    f = 0;
    r = (d / 2) - d;
    h = -(integer) r;
    while (true) {

        if (odd(n)) {
            r = r + x;
            if (r >= 0) {
                r = r - d;
                f++;
            }
        }
        n = n / 2;
        if (n == 0)
            goto lab41;
        if (x < h)
            x = x + x;
        else {

            t = x - d;
            x = t + x;
            f = f + n;
            if (x < n) {
                if (x == 0)
                    goto lab41;
                t = x;
                x = n;
                n = t;
            }
        }
    }
 lab41:                        /*found1 *//*:1588 */ if (f > (max_answer - a))
        goto lab88;
    a = a + f;
 lab40:                        /*found */ if (negative)
        a = -(integer) a;
    goto done;
 lab88:                        /*too_big */  {

        arith_error = true;
        a = 0;
    }
 done:
    Result = a;
    return Result;
}

void scan_expr(void)
{
    memory_word *mem = zmem; boolean a, b;
    small_number l;
    small_number r;
    small_number s;
    small_number o;
    integer e;
    integer t;
    integer f;
    integer n;
    int32_t p;
    int32_t q;
    l = cur_val_level;
    a = arith_error;
    b = false;
    p = MIN_HALFWORD;
 lab20:/*restart */ r = EXPR_NONE;
    e = 0;
    s = EXPR_NONE;
    t = 0;
    n = 0;
 lab22:/*continue */ if (s == EXPR_NONE)
        o = l;
    else
        o = INT_VAL;
    do {
        get_x_token();
    } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
    if (cur_tok == (OTHER_TOKEN + 40)) {    /*1576: */
        q = get_node(EXPR_NODE_SIZE);
        mem[q].hh.v.RH = p;
        mem[q].hh.u.B0 = l;
        mem[q].hh.u.B1 = 4 * s + r;
        mem[q + 1].cint = e;
        mem[q + 2].cint = t;
        mem[q + 3].cint = n;
        p = q;
        l = o;
        goto lab20;
    }
    back_input();
    if (o == INT_VAL)
        scan_int();
    else if (o == DIMEN_VAL)
        scan_dimen(false, false, false);
    else if (o == GLUE_VAL)
        scan_normal_glue();
    else
        scan_mu_glue();
    f = /*:1573 */ cur_val;
 lab40:                        /*found *//*1572: *//*424: */
    do {
        get_x_token();
    } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
    if (cur_tok == (OTHER_TOKEN + 43))
        o = EXPR_ADD;
    else if (cur_tok == (OTHER_TOKEN + 45))
        o = EXPR_SUB;
    else if (cur_tok == (OTHER_TOKEN + 42))
        o = EXPR_MULT;
    else if (cur_tok == (OTHER_TOKEN + 47))
        o = EXPR_DIV;
    else {

        o = EXPR_NONE;
        if (p == MIN_HALFWORD) {
            if (cur_cmd != RELAX)
                back_input();
        } else if (cur_tok != (OTHER_TOKEN + 41)) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Missing___inserted_for_expre/*ssion*/));
            }
            {
                help_ptr = 1;
                help_line[0] = S(I_was_expecting_to_see_______Z1/*"I was expecting to see `+', `-', `*', `/', or `)'. Didn't."*/);
            }
            back_error();
        }
    }
    arith_error = b;
    if ((l == INT_VAL) || (s > EXPR_SUB)) {
        if ((f > TEX_INFINITY) || (f < -TEX_INFINITY)) {
            arith_error = true;
            f = 0;
        }
    } else if (l == DIMEN_VAL) {
        if (abs(f) > MAX_HALFWORD) {
            arith_error = true;
            f = 0;
        }
    } else {

        if ((abs(mem[f + 1].cint) > MAX_HALFWORD) || (abs(mem[f + 2].cint) > MAX_HALFWORD)
            || (abs(mem[f + 3].cint) > MAX_HALFWORD)) {
            arith_error = true;
            delete_glue_ref(f);
            f = new_spec(0);
        }
    }
    switch (s) {                /*1579: */
    case 0:
        if ((l >= GLUE_VAL) && (o != EXPR_NONE)) {
            t = new_spec(f);
            delete_glue_ref(f);
            if (mem[t + 2].cint == 0)
                mem[t].hh.u.B0 = NORMAL;
            if (mem[t + 3].cint == 0)
                mem[t].hh.u.B1 = NORMAL;
        } else
            t = f;
        break;
    case 3:
        if (o == EXPR_DIV) {
            n = f;
            o = EXPR_SCALE;
        } else if (l == INT_VAL)
            t = mult_and_add(t, f, 0, TEX_INFINITY);
        else if (l == DIMEN_VAL)
            t = mult_and_add(t, f, 0, MAX_HALFWORD);
        else {

            mem[t + 1].cint = mult_and_add(mem[t + 1].cint, f, 0, MAX_HALFWORD);
            mem[t + 2].cint = mult_and_add(mem[t + 2].cint, f, 0, MAX_HALFWORD);
            mem[t + 3].cint = mult_and_add(mem[t + 3].cint, f, 0, MAX_HALFWORD);
        }
        break;
    case 4:
        if (l < GLUE_VAL)
            t = quotient(t, f);
        else {

            mem[t + 1].cint = quotient(mem[t + 1].cint, f);
            mem[t + 2].cint = quotient(mem[t + 2].cint, f);
            mem[t + 3].cint = quotient(mem[t + 3].cint, f);
        }
        break;
    case 5:
        if (l == INT_VAL)
            t = fract(t, n, f, TEX_INFINITY);
        else if (l == DIMEN_VAL)
            t = fract(t, n, f, MAX_HALFWORD);
        else {

            mem[t + 1].cint = fract(mem[t + 1].cint, n, f, MAX_HALFWORD);
            mem[t + 2].cint = fract(mem[t + 2].cint, n, f, MAX_HALFWORD);
            mem[t + 3].cint = fract(mem[t + 3].cint, n, f, MAX_HALFWORD);
        }
        break;
    }
    if (o > EXPR_SUB)
        s = o;
    else {                      /*1580: */

        s = EXPR_NONE;
        if (r == EXPR_NONE)
            e = t;
        else if (l == INT_VAL)
            e = add_or_sub(e, t, TEX_INFINITY, r == EXPR_SUB);
        else if (l == DIMEN_VAL)
            e = add_or_sub(e, t, MAX_HALFWORD, r == EXPR_SUB);
        else {                  /*1582: */

            mem[e + 1].cint = add_or_sub(mem[e + 1].cint, mem[t + 1].cint, MAX_HALFWORD, r == EXPR_SUB);
            if (mem[e].hh.u.B0 == mem[t].hh.u.B0)
                mem[e + 2].cint = add_or_sub(mem[e + 2].cint, mem[t + 2].cint, MAX_HALFWORD, r == EXPR_SUB);
            else if ((mem[e].hh.u.B0 < mem[t].hh.u.B0) && (mem[t + 2].cint != 0)) {
                mem[e + 2].cint = mem[t + 2].cint;
                mem[e].hh.u.B0 = mem[t].hh.u.B0;
            }
            if (mem[e].hh.u.B1 == mem[t].hh.u.B1)
                mem[e + 3].cint = add_or_sub(mem[e + 3].cint, mem[t + 3].cint, MAX_HALFWORD, r == EXPR_SUB);
            else if ((mem[e].hh.u.B1 < mem[t].hh.u.B1) && (mem[t + 3].cint != 0)) {
                mem[e + 3].cint = mem[t + 3].cint;
                mem[e].hh.u.B1 = mem[t].hh.u.B1;
            }
            delete_glue_ref(t);
            if (mem[e + 2].cint == 0)
                mem[e].hh.u.B0 = NORMAL;
            if (mem[e + 3].cint == 0)
                mem[e].hh.u.B1 = NORMAL;
        }
        r = o;
    }
    b = arith_error;
    if (o != EXPR_NONE)
        goto lab22;
    if (p != MIN_HALFWORD) {     /*1577: */
        f = e;
        q = p;
        e = mem[q + 1].cint;
        t = mem[q + 2].cint;
        n = mem[q + 3].cint;
        s = mem[q].hh.u.B1 / 4;
        r = mem[q].hh.u.B1 % 4;
        l = mem[q].hh.u.B0;
        p = mem[q].hh.v.RH;
        free_node(q, EXPR_NODE_SIZE);
        goto lab40;
    }
    if (b) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Arithmetic_overflow));
        }
        {
            help_ptr = 2;
            help_line[1] = S(I_can_t_evaluate_this_expres/*sion,*/);
            help_line[0] = S(since_the_result_is_out_of_r/*ange.*/);
        }
        error();
        if (l >= GLUE_VAL) {
            delete_glue_ref(e);
            e = 0;
            mem[e].hh.v.RH++;
        } else
            e = 0;
    }
    arith_error = a;
    cur_val = e;
    cur_val_level = l;
}

void scan_normal_glue(void)
{
    scan_glue(GLUE_VAL);
}

void scan_mu_glue(void)
{
    scan_glue(MU_VAL);
}

int32_t scan_rule_spec(void)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t q;
    q = new_rule();
    if (cur_cmd == VRULE)
        mem[q + 1].cint = DEFAULT_RULE;
    else {

        mem[q + 3].cint = DEFAULT_RULE;
        mem[q + 2].cint = 0;
    }
 lab21:/*reswitch */ if (scan_keyword(S(width))) {
        scan_dimen(false, false, false);
        mem[q + 1].cint = cur_val;
        goto lab21;
    }
    if (scan_keyword(S(height))) {
        scan_dimen(false, false, false);
        mem[q + 3].cint = cur_val;
        goto lab21;
    }
    if (scan_keyword(S(depth))) {
        scan_dimen(false, false, false);
        mem[q + 2].cint = cur_val;
        goto lab21;
    }
    Result = q;
    return Result;
}

void scan_general_text(void)
{
    memory_word *mem = zmem; unsigned char /*absorbing */ s;
    int32_t w;
    int32_t d;
    int32_t p;
    int32_t q;
    int32_t unbalance;
    s = scanner_status;
    w = warning_index;
    d = def_ref;
    scanner_status = ABSORBING;
    warning_index = cur_cs;
    def_ref = get_avail();
    mem[def_ref].hh.v.LH = MIN_HALFWORD;
    p = def_ref;
    scan_left_brace();
    unbalance = 1;
    while (true) {

        get_token();
        if (cur_tok < RIGHT_BRACE_LIMIT) {

            if (cur_cmd < RIGHT_BRACE)
                unbalance++;
            else {

                unbalance--;
                if (unbalance == 0)
                    goto lab40;
            }
        }
        {
            q = get_avail();
            mem[p].hh.v.RH = q;
            mem[q].hh.v.LH = cur_tok;
            p = q;
        }
    }
 lab40:                        /*found */ q = mem[def_ref].hh.v.RH;
    {
        mem[def_ref].hh.v.RH = avail;
        avail = def_ref;
    }
    if (q == MIN_HALFWORD)
        cur_val = mem_top - 3;
    else
        cur_val = p;
    mem[mem_top - 3].hh.v.RH = q;
    scanner_status = s;
    warning_index = w;
    def_ref = d;
}

void pseudo_start(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    unsigned char /*max_selector */ old_setting;
    str_number s;
    pool_pointer l, m;
    int32_t p, q, r;
    four_quarters w;
    integer nl, sz;

    scan_general_text();
    old_setting = selector;
    selector = SELECTOR_NEW_STRING ;
    token_show(mem_top - 3);
    selector = old_setting;
    flush_list(mem[mem_top - 3].hh.v.RH);
    {
        if (pool_ptr + 1 > pool_size)
            overflow(S(pool_size), pool_size - init_pool_ptr);
    }
    s = make_string();
    str_pool[pool_ptr] = 32 /*" " */ ;
    l = str_start[(s) - 65536L];
    nl = INTPAR(new_line_char);
    p = get_avail();
    q = p;
    while (l < pool_ptr) {

        m = l;
        while ((l < pool_ptr) && (str_pool[l] != nl))
            l++;
        sz = (l - m + 7) / 4;
        if (sz == 1)
            sz = 2;
        r = get_node(sz);
        mem[q].hh.v.RH = r;
        q = r;
        mem[q].hh.v.LH = sz;
        while (sz > 2) {

            sz--;
            r++;
            w.u.B0 = str_pool[m];
            w.u.B1 = str_pool[m + 1];
            w.u.B2 = str_pool[m + 2];
            w.u.B3 = str_pool[m + 3];
            mem[r].qqqq = w;
            m = m + 4;
        }
        w.u.B0 = 32 /*" " */ ;
        w.u.B1 = 32 /*" " */ ;
        w.u.B2 = 32 /*" " */ ;
        w.u.B3 = 32 /*" " */ ;
        if (l > m) {
            w.u.B0 = str_pool[m];
            if (l > m + 1) {
                w.u.B1 = str_pool[m + 1];
                if (l > m + 2) {
                    w.u.B2 = str_pool[m + 2];
                    if (l > m + 3)
                        w.u.B3 = str_pool[m + 3];
                }
            }
        }
        mem[r + 1].qqqq = w;
        if (str_pool[l] == nl)
            l++;
    }
    mem[p].hh.v.LH = mem[p].hh.v.RH;
    mem[p].hh.v.RH = pseudo_files;
    pseudo_files = /*:1542 */ p;
    {
        str_ptr--;
        pool_ptr = str_start[(str_ptr) - 65536L];
    }
    begin_file_reading();
    line = 0;
    cur_input.limit = cur_input.start;
    cur_input.loc = cur_input.limit + 1;
    if (INTPAR(tracing_scan_tokens) > 0) {
        if (term_offset > max_print_line - 3)
            print_ln();
        else if ((term_offset > 0) || (file_offset > 0))
            print_char(32 /*" " */ );
        cur_input.name = 19;
        print(S(___Z21/*"( "*/));
        open_parens++;
        ttstub_output_flush (rust_stdout);
    } else {

        cur_input.name = 18;
        cur_input.synctex_tag = 0;
    }
}

int32_t str_toks_cat(pool_pointer b, small_number cat)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    int32_t q;
    int32_t t;
    pool_pointer k;
    {
        if (pool_ptr + 1 > pool_size)
            overflow(S(pool_size), pool_size - init_pool_ptr);
    }
    p = mem_top - 3;
    mem[p].hh.v.RH = MIN_HALFWORD;
    k = b;
    while (k < pool_ptr) {

        t = str_pool[k];
        if ((t == 32 /*" " */ ) && (cat == 0))
            t = SPACE_TOKEN;
        else {

            if ((t >= 55296L) && (t <= 56319L) && (k + 1 < pool_ptr) && (str_pool[k + 1] >= 56320L)
                && (str_pool[k + 1] <= 57343L)) {
                k++;
                t = 65536L + (t - 55296L) * 1024 + (str_pool[k] - 56320L);
            }
            if (cat == 0)
                t = OTHER_TOKEN + t;
            else
                t = MAX_CHAR_VAL * cat + t;
        }
        {
            {
                q = avail;
                if (q == MIN_HALFWORD)
                    q = get_avail();
                else {

                    avail = mem[q].hh.v.RH;
                    mem[q].hh.v.RH = MIN_HALFWORD;
                }
            }
            mem[p].hh.v.RH = q;
            mem[q].hh.v.LH = t;
            p = q;
        }
        k++;
    }
    pool_ptr = b;
    Result = p;
    return Result;
}

int32_t str_toks(pool_pointer b)
{
    register int32_t Result;
    Result = str_toks_cat(b, 0);
    return Result;
}

int32_t the_toks(void)
{
    register int32_t Result;
    memory_word *mem = zmem; unsigned char /*max_selector */ old_setting;
    int32_t p, q, r;
    pool_pointer b;
    small_number c;
    if (odd(cur_chr)) {
        c = cur_chr;
        scan_general_text();
        if (c == 1)
            Result = cur_val;
        else {

            old_setting = selector;
            selector = SELECTOR_NEW_STRING ;
            b = pool_ptr;
            p = get_avail();
            mem[p].hh.v.RH = mem[mem_top - 3].hh.v.RH;
            token_show(p);
            flush_list(p);
            selector = old_setting;
            Result = str_toks(b);
        }
        return Result;
    }
    get_x_token();
    scan_something_internal(TOK_VAL, false);
    if (cur_val_level >= IDENT_VAL) {   /*485: */
        p = mem_top - 3;
        mem[p].hh.v.RH = MIN_HALFWORD;
        if (cur_val_level == IDENT_VAL) {
            q = get_avail();
            mem[p].hh.v.RH = q;
            mem[q].hh.v.LH = CS_TOKEN_FLAG + cur_val;
            p = q;
        } else if (cur_val != MIN_HALFWORD) {
            r = mem[cur_val].hh.v.RH;
            while (r != MIN_HALFWORD) {

                {
                    {
                        q = avail;
                        if (q == MIN_HALFWORD)
                            q = get_avail();
                        else {

                            avail = mem[q].hh.v.RH;
                            mem[q].hh.v.RH = MIN_HALFWORD;
                        }
                    }
                    mem[p].hh.v.RH = q;
                    mem[q].hh.v.LH = mem[r].hh.v.LH;
                    p = q;
                }
                r = mem[r].hh.v.RH;
            }
        }
        Result = p;
    } else {

        old_setting = selector;
        selector = SELECTOR_NEW_STRING ;
        b = pool_ptr;
        switch (cur_val_level) {
        case 0:
            print_int(cur_val);
            break;
        case 1:
            {
                print_scaled(cur_val);
                print(S(pt));
            }
            break;
        case 2:
            {
                print_spec(cur_val, S(pt));
                delete_glue_ref(cur_val);
            }
            break;
        case 3:
            {
                print_spec(cur_val, S(mu));
                delete_glue_ref(cur_val);
            }
            break;
        }
        selector = old_setting;
        Result = str_toks(b);
    }
    return Result;
}

void ins_the_toks(void)
{
    memory_word *mem = zmem;
    mem[mem_top - 12].hh.v.RH = the_toks();
    begin_token_list(mem[mem_top - 3].hh.v.RH, INSERTED);
}

void conv_toks(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    unsigned char /*max_selector */ old_setting;
    int32_t save_warning_index, save_def_ref;
    boolean boolvar;
    str_number s;
    str_number u;
    small_number c;
    small_number save_scanner_status;
    pool_pointer b;
    integer fnt, arg1, arg2;
    str_number font_name_str;
    small_number i;
    UTF16_code quote_char;
    small_number cat;
    UnicodeScalar saved_chr;
    int32_t p, q;

    cat = 0;
    c = cur_chr;
    switch (c) {
    case 0:
    case 1:
        scan_int();
        break;
    case 2:
    case 3:
        {
            save_scanner_status = scanner_status;
            scanner_status = NORMAL;
            get_token();
            scanner_status = save_scanner_status;
        }
        break;
    case 4:
        scan_font_ident();
        break;
    case 13:
        scan_usv_num();
        break;
    case 14:
        {
            scan_usv_num();
            saved_chr = cur_val;
            scan_int();
            if ((cur_val < LEFT_BRACE) || (cur_val > OTHER_CHAR) || (cur_val == OUT_PARAM)
                || (cur_val == IGNORE)) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Invalid_code__));
                }
                print_int(cur_val);
                print(S(___should_be_in_the_ranges_1/*..4, 6..8, 10..12*/));
                {
                    help_ptr = 1;
                    help_line[0] = S(I_m_going_to_use_12_instead_/*of that illegal code value.*/);
                }
                error();
                cat = 12;
            } else
                cat = cur_val;
            cur_val = saved_chr;
        }
        break;
    case 5:
        ;
        break;
    case 43:
        {
            save_scanner_status = scanner_status;
            save_warning_index = warning_index;
            save_def_ref = def_ref;
            if (str_start[(str_ptr) - 65536L] < pool_ptr)
                u = make_string();
            else
                u = 0;
            compare_strings();
            def_ref = save_def_ref;
            warning_index = save_warning_index;
            scanner_status = save_scanner_status;
            if (u != 0)
                str_ptr--;
        }
        break;
    case 44:
        {
            save_scanner_status = scanner_status;
            save_warning_index = warning_index;
            save_def_ref = def_ref;
            if (str_start[(str_ptr) - 65536L] < pool_ptr)
                u = make_string();
            else
                u = 0;
            boolvar = scan_keyword(S(file));
            scan_pdf_ext_toks();
            if (selector == SELECTOR_NEW_STRING )
                pdf_error(S(tokens), S(tokens_to_string___called_wh/*ile selector = new_string*/));
            old_setting = selector;
            selector = SELECTOR_NEW_STRING ;
            show_token_list(mem[def_ref].hh.v.RH, MIN_HALFWORD, pool_size - pool_ptr);
            selector = old_setting;
            s = make_string();
            delete_token_ref(def_ref);
            def_ref = save_def_ref;
            warning_index = save_warning_index;
            scanner_status = save_scanner_status;
            b = pool_ptr;
            getmd5sum(s, boolvar);
            mem[mem_top - 12].hh.v.RH = str_toks(b);
            if ((s == str_ptr - 1)) {
                str_ptr--;
                pool_ptr = str_start[(str_ptr) - 65536L];
            }
            begin_token_list(mem[mem_top - 3].hh.v.RH, INSERTED);
            if (u != 0)
                str_ptr--;
            return;
        }
        break;
    case 6:
        ;
        break;
    case 7:
        {
            scan_font_ident();
            fnt = cur_val;
            if ((font_area[fnt] == AAT_FONT_FLAG)) {
                scan_int();
                arg1 = cur_val;
                arg2 = 0;
            } else
                not_aat_font_error(CONVERT, c, fnt);
        }
        break;
    case 8:
        {
            scan_font_ident();
            fnt = cur_val;
            if ((font_area[fnt] == AAT_FONT_FLAG)
                || ((font_area[fnt] == OTGR_FONT_FLAG) && (usingGraphite(font_layout_engine[fnt])))) {
                scan_int();
                arg1 = cur_val;
                arg2 = 0;
            } else
                not_aat_gr_font_error(CONVERT, c, fnt);
        }
        break;
    case 9:
        {
            scan_font_ident();
            fnt = cur_val;
            if ((font_area[fnt] == AAT_FONT_FLAG)
                || ((font_area[fnt] == OTGR_FONT_FLAG) && (usingGraphite(font_layout_engine[fnt])))) {
                scan_int();
                arg1 = cur_val;
                scan_int();
                arg2 = cur_val;
            } else
                not_aat_gr_font_error(CONVERT, c, fnt);
        }
        break;
    case 10:
        {
            scan_font_ident();
            fnt = cur_val;
            if (((font_area[fnt] == AAT_FONT_FLAG) || (font_area[fnt] == OTGR_FONT_FLAG))) {
                scan_int();
                arg1 = cur_val;
            } else
                not_native_font_error(CONVERT, c, fnt);
        }
        break;
    case 11:
    case 12:
        {
            scan_register_num();
            if (cur_val < 256)
                p = BOX_REG(cur_val);
            else {

                find_sa_element(4, cur_val, false);
                if (cur_ptr == MIN_HALFWORD)
                    p = MIN_HALFWORD;
                else
                    p = mem[cur_ptr + 1].hh.v.RH;
            }
            if ((p == MIN_HALFWORD) || (mem[p].hh.u.B0 != HLIST_NODE))
                pdf_error(S(marginkern), S(a_non_empty_hbox_expected));
        }
        break;
    case 15:
        if (job_name == 0)
            open_log_file();
        break;
    }
    old_setting = selector;
    selector = SELECTOR_NEW_STRING ;
    b = pool_ptr;
    switch (c) {
    case 0:
        print_int(cur_val);
        break;
    case 1:
        print_roman_int(cur_val);
        break;
    case 2:
        if (cur_cs != 0)
            sprint_cs(cur_cs);
        else
            print_char(cur_chr);
        break;
    case 3:
        print_meaning();
        break;
    case 4:
        {
            font_name_str = font_name[cur_val];
            if (((font_area[cur_val] == AAT_FONT_FLAG)
                 || (font_area[cur_val] == OTGR_FONT_FLAG))) {
                quote_char = 34 /*""" */ ;
                {
                    register integer for_end;
                    i = 0;
                    for_end = length(font_name_str) - 1;
                    if (i <= for_end)
                        do
                            if (str_pool[str_start[(font_name_str) - 65536L] + i] == 34 /*""" */ )
                                quote_char = 39 /*"'" */ ;
                        while (i++ < for_end) ;
                }
                print_char(quote_char);
                print(font_name_str);
                print_char(quote_char);
            } else
                print(font_name_str);
            if (font_size[cur_val] != font_dsize[cur_val]) {
                print(S(_at_));
                print_scaled(font_size[cur_val]);
                print(S(pt));
            }
        }
        break;
    case 13:
    case 14:
        print_char(cur_val);
        break;
    case 5:
        print(S(_6));
        break;
    case 43:
        print_int(cur_val);
        break;
    case 6:
        print(S(_99996));
        break;
    case 7:
        if ((font_area[fnt] == AAT_FONT_FLAG))
            aat_print_font_name(c, font_layout_engine[fnt], arg1, arg2);
        break;
    case 8:
    case 9:
        if ((font_area[fnt] == AAT_FONT_FLAG))
            aat_print_font_name(c, font_layout_engine[fnt], arg1, arg2);
        else if (((font_area[fnt] == OTGR_FONT_FLAG) && (usingGraphite(font_layout_engine[fnt]))))
            gr_print_font_name(c, font_layout_engine[fnt], arg1, arg2);
        break;
    case 10:
        if (((font_area[fnt] == AAT_FONT_FLAG) || (font_area[fnt] == OTGR_FONT_FLAG)))
            print_glyph_name(fnt, arg1);
        break;
    case 11:
        {
            p = mem[p + 5].hh.v.RH;
            while ((p != MIN_HALFWORD)
                   &&
                   ((!(p >= hi_mem_min)
                     && ((mem[p].hh.u.B0 == INS_NODE) || (mem[p].hh.u.B0 == MARK_NODE)
                         || (mem[p].hh.u.B0 == ADJUST_NODE) || (mem[p].hh.u.B0 == PENALTY_NODE)
                         || ((mem[p].hh.u.B0 == DISC_NODE) && (mem[p + 1].hh.v.LH == MIN_HALFWORD)
                             && (mem[p + 1].hh.v.RH == MIN_HALFWORD) && (mem[p].hh.u.B1 == 0))
                         || ((mem[p].hh.u.B0 == MATH_NODE) && (mem[p + 1].cint == 0))
                         || ((mem[p].hh.u.B0 == KERN_NODE)
                             && ((mem[p + 1].cint == 0) || (mem[p].hh.u.B1 == NORMAL)))
                         || ((mem[p].hh.u.B0 == GLUE_NODE) && (mem[p + 1].hh.v.LH == 0))
                         || ((mem[p].hh.u.B0 == HLIST_NODE) && (mem[p + 1].cint == 0) && (mem[p + 3].cint == 0)
                             && (mem[p + 2].cint == 0) && (mem[p + 5].hh.v.RH == MIN_HALFWORD))))
                    || ((!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == GLUE_NODE)
                        && (mem[p].hh.u.B1 == (GLUE_PAR__left_skip + 1)))))
                p = mem[p].hh.v.RH;
            if ((p != MIN_HALFWORD) && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == MARGIN_KERN_NODE)
                && (mem[p].hh.u.B1 == 0))
                print_scaled(mem[p + 1].cint);
            else
                print(48 /*"0" */ );
            print(S(pt));
        }
        break;
    case 12:
        {
            q = mem[p + 5].hh.v.RH;
            p = prev_rightmost(q, MIN_HALFWORD);
            while ((p != MIN_HALFWORD)
                   &&
                   ((!(p >= hi_mem_min)
                     && ((mem[p].hh.u.B0 == INS_NODE) || (mem[p].hh.u.B0 == MARK_NODE)
                         || (mem[p].hh.u.B0 == ADJUST_NODE) || (mem[p].hh.u.B0 == PENALTY_NODE)
                         || ((mem[p].hh.u.B0 == DISC_NODE) && (mem[p + 1].hh.v.LH == MIN_HALFWORD)
                             && (mem[p + 1].hh.v.RH == MIN_HALFWORD) && (mem[p].hh.u.B1 == 0))
                         || ((mem[p].hh.u.B0 == MATH_NODE) && (mem[p + 1].cint == 0))
                         || ((mem[p].hh.u.B0 == KERN_NODE)
                             && ((mem[p + 1].cint == 0) || (mem[p].hh.u.B1 == NORMAL)))
                         || ((mem[p].hh.u.B0 == GLUE_NODE) && (mem[p + 1].hh.v.LH == 0))
                         || ((mem[p].hh.u.B0 == HLIST_NODE) && (mem[p + 1].cint == 0) && (mem[p + 3].cint == 0)
                             && (mem[p + 2].cint == 0) && (mem[p + 5].hh.v.RH == MIN_HALFWORD))))
                    || ((!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == GLUE_NODE)
                        && (mem[p].hh.u.B1 == (GLUE_PAR__right_skip + 1)))))
                p = prev_rightmost(q, p);
            if ((p != MIN_HALFWORD) && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == MARGIN_KERN_NODE)
                && (mem[p].hh.u.B1 == 1))
                print_scaled(mem[p + 1].cint);
            else
                print(48 /*"0" */ );
            print(S(pt));
        }
        break;
    case 15:
        print_file_name(job_name, 0, 0);
        break;
    }
    selector = old_setting;
    mem[mem_top - 12].hh.v.RH = str_toks_cat(b, cat);
    begin_token_list(mem[mem_top - 3].hh.v.RH, INSERTED);
}

int32_t scan_toks(boolean macro_def, boolean xpand)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t t;
    int32_t s;
    int32_t p;
    int32_t q;
    int32_t unbalance;
    int32_t hash_brace;
    if (macro_def)
        scanner_status = DEFINING;
    else
        scanner_status = ABSORBING;
    warning_index = cur_cs;
    def_ref = get_avail();
    mem[def_ref].hh.v.LH = MIN_HALFWORD;
    p = def_ref;
    hash_brace = 0;
    t = ZERO_TOKEN;
    if (macro_def) {            /*493: */
        while (true) {

            get_token();
            if (cur_tok < RIGHT_BRACE_LIMIT)
                goto lab31;
            if (cur_cmd == MAC_PARAM) { /*495: */
                s = MATCH_TOKEN + cur_chr;
                get_token();
                if (cur_cmd == LEFT_BRACE) {
                    hash_brace = cur_tok;
                    {
                        q = get_avail();
                        mem[p].hh.v.RH = q;
                        mem[q].hh.v.LH = cur_tok;
                        p = q;
                    }
                    {
                        q = get_avail();
                        mem[p].hh.v.RH = q;
                        mem[q].hh.v.LH = END_MATCH_TOKEN;
                        p = q;
                    }
                    goto done;
                }
                if (t == (ZERO_TOKEN + 9)) {
                    {
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(You_already_have_nine_parame/*ters*/));
                    }
                    {
                        help_ptr = 1;
                        help_line[0] = S(I_m_going_to_ignore_the___si/*gn you just used.*/);
                    }
                    error();
                } else {

                    t++;
                    if (cur_tok != t) {
                        {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Parameters_must_be_numbered_/*consecutively*/));
                        }
                        {
                            help_ptr = 2;
                            help_line[1] = S(I_ve_inserted_the_digit_you_/*should have used after the #.*/);
                            help_line[0] = S(Type__1__to_delete_what_you_/*did use.*/);
                        }
                        back_error();
                    }
                    cur_tok = s;
                }
            }
            {
                q = get_avail();
                mem[p].hh.v.RH = q;
                mem[q].hh.v.LH = cur_tok;
                p = q;
            }
        }
 lab31:                        /*done1 */  {

            q = get_avail();
            mem[p].hh.v.RH = q;
            mem[q].hh.v.LH = END_MATCH_TOKEN;
            p = q;
        }
        if (cur_cmd == RIGHT_BRACE) {   /*494: */
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(65980L /*"Missing _ inserted" */ );
            }
            align_state++;
            {
                help_ptr = 2;
                help_line[1] = 66098L /*"Where was the left brace? You said something like `\def\a_'," */ ;
                help_line[0] = 66099L /*"which I'm going to interpret as `\def\a__'." */ ;
            }
            error();
            goto lab40;
        }
    done:
        ;
    } else
        scan_left_brace();
    unbalance = 1;
    while (true) {

        if (xpand) {            /*497: */
            while (true) {

                get_next();
                if (cur_cmd >= CALL) {

                    if (mem[mem[cur_chr].hh.v.RH].hh.v.LH == PROTECTED_TOKEN) {
                        cur_cmd = RELAX;
                        cur_chr = NO_EXPAND_FLAG;
                    }
                }
                if (cur_cmd <= MAX_COMMAND)
                    goto lab32;
                if (cur_cmd != THE)
                    expand();
                else {

                    q = the_toks();
                    if (mem[mem_top - 3].hh.v.RH != MIN_HALFWORD) {
                        mem[p].hh.v.RH = mem[mem_top - 3].hh.v.RH;
                        p = q;
                    }
                }
            }
 lab32:                        /*done2 */ x_token();
        } else
            get_token();
        if (cur_tok < RIGHT_BRACE_LIMIT) {

            if (cur_cmd < RIGHT_BRACE)
                unbalance++;
            else {

                unbalance--;
                if (unbalance == 0)
                    goto lab40;
            }
        } else if (cur_cmd == MAC_PARAM) {

            if (macro_def) {    /*498: */
                s = cur_tok;
                if (xpand)
                    get_x_token();
                else
                    get_token();
                if (cur_cmd != MAC_PARAM) {

                    if ((cur_tok <= ZERO_TOKEN) || (cur_tok > t)) {
                        {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Illegal_parameter_number_in_/*definition of */));
                        }
                        sprint_cs(warning_index);
                        {
                            help_ptr = 3;
                            help_line[2] = S(You_meant_to_type____instead/* of #, right?*/);
                            help_line[1] = 66107L /*"Or maybe a _ was forgotten somewhere earlier, and things" */ ;
                            help_line[0] = S(are_all_screwed_up__I_m_goin/*g to assume that you meant ##.*/);
                        }
                        back_error();
                        cur_tok = s;
                    } else
                        cur_tok = (OUT_PARAM_TOKEN - 48) + cur_chr;
                }
            }
        }
        {
            q = get_avail();
            mem[p].hh.v.RH = q;
            mem[q].hh.v.LH = cur_tok;
            p = q;
        }
    }
 lab40:/*found */ scanner_status = NORMAL;
    if (hash_brace != 0) {
        q = get_avail();
        mem[p].hh.v.RH = q;
        mem[q].hh.v.LH = hash_brace;
        p = q;
    }
    Result = p;
    return Result;
}

void read_toks(integer n, int32_t r, int32_t j)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;
    int32_t q;
    integer s;
    small_number m;

    scanner_status = DEFINING;
    warning_index = r;
    def_ref = get_avail();
    mem[def_ref].hh.v.LH = MIN_HALFWORD;
    p = def_ref;
    {
        q = get_avail();
        mem[p].hh.v.RH = q;
        mem[q].hh.v.LH = END_MATCH_TOKEN;
        p = q;
    }
    if ((n < 0) || (n > 15))
        m = 16;
    else
        m = n;
    s = align_state;
    align_state = 1000000L;
    do {
        /*502: */ begin_file_reading();
        cur_input.name = m + 1;
        if (read_open[m] == CLOSED) {   /*503: */
	    _tt_abort ("terminal input forbidden");
        } else if (read_open[m] == JUST_OPEN) { /*504: */

            if (input_line(read_file[m]))
                read_open[m] = NORMAL;
            else {

                u_close(read_file[m]);
                read_open[m] = CLOSED;
            }
        } else {                /*505: */

            if (!input_line(read_file[m])) {
                u_close(read_file[m]);
                read_open[m] = CLOSED;
                if (align_state != 1000000L) {
                    runaway();
                    {
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(File_ended_within_));
                    }
                    print_esc(S(read));
                    {
                        help_ptr = 1;
                        help_line[0] = S(This__read_has_unbalanced_br/*aces.*/);
                    }
                    align_state = 1000000L;
                    error();
                }
            }
        }
        cur_input.limit = last;
        if ((INTPAR(end_line_char) < 0) || (INTPAR(end_line_char) > 255))
            cur_input.limit--;
        else
            buffer[cur_input.limit] = INTPAR(end_line_char);
        first = cur_input.limit + 1;
        cur_input.loc = cur_input.start;
        cur_input.state = NEW_LINE;
        if (j == 1) {
            while (cur_input.loc <= cur_input.limit) {

                cur_chr = buffer[cur_input.loc];
                cur_input.loc++;
                if (cur_chr == 32 /*" " */ )
                    cur_tok = SPACE_TOKEN;
                else
                    cur_tok = cur_chr + 25165824L;
                {
                    q = get_avail();
                    mem[p].hh.v.RH = q;
                    mem[q].hh.v.LH = cur_tok;
                    p = q;
                }
            }
            goto done;
        }
        while (true) {

            get_token();
            if (cur_tok == 0)
                goto done;
            if (align_state < 1000000L) {
                do {
                    get_token();
                } while (!(cur_tok == 0));
                align_state = 1000000L;
                goto done;
            }
            {
                q = get_avail();
                mem[p].hh.v.RH = q;
                mem[q].hh.v.LH = cur_tok;
                p = q;
            }
        }
    done:
        end_file_reading();
    } while (!(align_state == 1000000L));
    cur_val = def_ref;
    scanner_status = NORMAL;
    align_state = s;
}

void pass_text(void)
{
    CACHE_THE_EQTB;
    integer l;
    small_number save_scanner_status;

    save_scanner_status = scanner_status;
    scanner_status = SKIPPING;
    l = 0;
    skip_line = line;

    while (true) {

        get_next();
        if (cur_cmd == FI_OR_ELSE) {
            if (l == 0)
                goto done;
            if (cur_chr == FI_CODE)
                l--;
        } else if (cur_cmd == IF_TEST)
            l++;
    }
done:
    scanner_status = save_scanner_status;
    if (INTPAR(tracing_ifs) > 0)
        show_cur_cmd_chr();
}

void change_if_limit(small_number l, int32_t p)
{
    memory_word *mem = zmem; int32_t q;
    if (p == cond_ptr)
        if_limit = l;
    else {

        q = cond_ptr;
        while (true) {

            if (q == MIN_HALFWORD)
                confusion(S(if));
            if (mem[q].hh.v.RH == p) {
                mem[q].hh.u.B0 = l;
                return;
            }
            q = mem[q].hh.v.RH;
        }
    }
}

void conditional(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    boolean b;
    boolean e;
    unsigned char /*">" */ r;
    integer m, n;
    int32_t p, q;
    small_number save_scanner_status;
    int32_t save_cond_ptr;
    small_number this_if;
    boolean is_unless;

    if (INTPAR(tracing_ifs) > 0) {

        if (INTPAR(tracing_commands) <= 1)
            show_cur_cmd_chr();
    }
    {
        p = get_node(IF_NODE_SIZE);
        mem[p].hh.v.RH = cond_ptr;
        mem[p].hh.u.B0 = if_limit;
        mem[p].hh.u.B1 = cur_if;
        mem[p + 1].cint = if_line;
        cond_ptr = p;
        cur_if = cur_chr;
        if_limit = IF_CODE;
        if_line = line;
    }
    save_cond_ptr = cond_ptr;
    is_unless = (cur_chr >= UNLESS_CODE);
    this_if = cur_chr % UNLESS_CODE;
    switch (this_if) {
    case 0:
    case 1:
        {
            {
                get_x_token();
                if (cur_cmd == RELAX) {

                    if (cur_chr == NO_EXPAND_FLAG) {
                        cur_cmd = ACTIVE_CHAR;
                        cur_chr = cur_tok - 33554432L;
                    }
                }
            }
            if ((cur_cmd > ACTIVE_CHAR) || (cur_chr > BIGGEST_USV)) {
                m = RELAX;
                n = TOO_BIG_USV;
            } else {

                m = cur_cmd;
                n = cur_chr;
            }
            {
                get_x_token();
                if (cur_cmd == RELAX) {

                    if (cur_chr == NO_EXPAND_FLAG) {
                        cur_cmd = ACTIVE_CHAR;
                        cur_chr = cur_tok - 33554432L;
                    }
                }
            }
            if ((cur_cmd > ACTIVE_CHAR) || (cur_chr > BIGGEST_USV)) {
                cur_cmd = RELAX;
                cur_chr = TOO_BIG_USV;
            }
            if (this_if == IF_CHAR_CODE)
                b = (n == cur_chr);
            else
                b = (m == cur_cmd);
        }
        break;
    case 2:
    case 3:
        {
            if (this_if == IF_INT_CODE)
                scan_int();
            else
                scan_dimen(false, false, false);
            n = cur_val;
            do {
                get_x_token();
            } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
            if ((cur_tok >= (OTHER_TOKEN + 60)) && (cur_tok <= (OTHER_TOKEN + 62)))
                r = cur_tok - 25165824L;
            else {

                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Missing___inserted_for_));
                }
                print_cmd_chr(IF_TEST, this_if);
                {
                    help_ptr = 1;
                    help_line[0] = S(I_was_expecting_to_see______/*`=', or `>'. Didn't.*/);
                }
                back_error();
                r = 61 /*"=" */ ;
            }
            if (this_if == IF_INT_CODE)
                scan_int();
            else
                scan_dimen(false, false, false);
            switch (r) {
            case 60:
                b = (n < cur_val);
                break;
            case 61:
                b = (n == cur_val);
                break;
            case 62:
                b = (n > cur_val);
                break;
            }
        }
        break;
    case 4:
        {
            scan_int();
            b = odd(cur_val);
        }
        break;
    case 5:
        b = (abs(cur_list.mode) == VMODE);
        break;
    case 6:
        b = (abs(cur_list.mode) == HMODE);
        break;
    case 7:
        b = (abs(cur_list.mode) == MMODE);
        break;
    case 8:
        b = (cur_list.mode < 0);
        break;
    case 9:
    case 10:
    case 11:
        {
            scan_register_num();
            if (cur_val < 256)
                p = BOX_REG(cur_val);
            else {

                find_sa_element(4, cur_val, false);
                if (cur_ptr == MIN_HALFWORD)
                    p = MIN_HALFWORD;
                else
                    p = mem[cur_ptr + 1].hh.v.RH;
            }
            if (this_if == IF_VOID_CODE)
                b = (p == MIN_HALFWORD);
            else if (p == MIN_HALFWORD)
                b = false;
            else if (this_if == IF_HBOX_CODE)
                b = (mem[p].hh.u.B0 == HLIST_NODE);
            else
                b = (mem[p].hh.u.B0 == VLIST_NODE);
        }
        break;
    case 12:
        {
            save_scanner_status = scanner_status;
            scanner_status = NORMAL;
            get_next();
            n = cur_cs;
            p = cur_cmd;
            q = cur_chr;
            get_next();
            if (cur_cmd != p)
                b = false;
            else if (cur_cmd < CALL)
                b = (cur_chr == q);
            else {              /*527: */

                p = mem[cur_chr].hh.v.RH;
                q = mem[eqtb[n].hh.v.RH].hh.v.RH;
                if (p == q)
                    b = true;
                else {

                    while ((p != MIN_HALFWORD) && (q != MIN_HALFWORD))
                        if (mem[p].hh.v.LH != mem[q].hh.v.LH)
                            p = MIN_HALFWORD;
                        else {

                            p = mem[p].hh.v.RH;
                            q = mem[q].hh.v.RH;
                        }
                    b = ((p == MIN_HALFWORD) && (q == MIN_HALFWORD));
                }
            }
            scanner_status = save_scanner_status;
        }
        break;
    case 13:
        {
            scan_four_bit_int_or_18();
            if (cur_val == 18)
                b = 1; /* !shellenabledp */
            else
                b = (read_open[cur_val] == CLOSED);
        }
        break;
    case 14:
        b = true;
        break;
    case 15:
        b = false;
        break;
    case 17:
        {
            save_scanner_status = scanner_status;
            scanner_status = NORMAL;
            get_next();
            b = (cur_cmd != UNDEFINED_CS);
            scanner_status = save_scanner_status;
        }
        break;
    case 18:
        {
            n = get_avail();
            p = n;
            e = is_in_csname;
            is_in_csname = true;
            do {
                get_x_token();
                if (cur_cs == 0) {
                    q = get_avail();
                    mem[p].hh.v.RH = q;
                    mem[q].hh.v.LH = cur_tok;
                    p = q;
                }
            } while (!(cur_cs != 0));
            if (cur_cmd != END_CS_NAME) {      /*391: */
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Missing_));
                }
                print_esc(S(endcsname));
                print(S(_inserted));
                {
                    help_ptr = 2;
                    help_line[1] = S(The_control_sequence_marked_/*<to be read again> should*/);
                    help_line[0] = S(not_appear_between__csname_a/*nd \endcsname.*/);
                }
                back_error();
            }
            m = first;
            p = mem[n].hh.v.RH;
            while (p != MIN_HALFWORD) {

                if (m >= max_buf_stack) {
                    max_buf_stack = m + 1;
                    if (max_buf_stack == buf_size)
                        overflow(S(buffer_size), buf_size);
                }
                buffer[m] = mem[p].hh.v.LH % MAX_CHAR_VAL;
                m++;
                p = mem[p].hh.v.RH;
            }
            if (m > first + 1)
                cur_cs = id_lookup(first, m - first);
            else if (m == first)
                cur_cs = NULL_CS;
            else
                cur_cs = SINGLE_BASE + buffer[first]     /*
                                                                           :1556 */ ;
            flush_list(n);
            b = (eqtb[cur_cs].hh.u.B0 != UNDEFINED_CS);
            is_in_csname = e;
        }
        break;
    case 20:
        b = is_in_csname;
        break;
    case 19:
        {
            scan_font_ident();
            n = cur_val;
            scan_usv_num();
            if (((font_area[n] == AAT_FONT_FLAG) || (font_area[n] == OTGR_FONT_FLAG)))
                b = (map_char_to_glyph(n, cur_val) > 0);
            else {

                if ((font_bc[n] <= cur_val) && (font_ec[n] >= cur_val))
                    b = (font_info[char_base[n] + effective_char(true, n, cur_val)].qqqq.u.B0 > 0);
                else
                    b = false;
            }
        }
        break;
    case 16:
        {
            scan_int();
            n = cur_val;
            if (INTPAR(tracing_commands) > 1) {
                begin_diagnostic();
                print(66140L /*"_case " */ );
                print_int(n);
                print_char(125 /*"_" */ );
                end_diagnostic(false);
            }
            while (n != 0) {

                pass_text();
                if (cond_ptr == save_cond_ptr) {

                    if (cur_chr == OR_CODE)
                        n--;
                    else
                        goto lab50;
                } else if (cur_chr == FI_CODE) {        /*515: */
                    if (if_stack[in_open] == cond_ptr)
                        if_warning();
                    p = cond_ptr;
                    if_line = mem[p + 1].cint;
                    cur_if = mem[p].hh.u.B1;
                    if_limit = mem[p].hh.u.B0;
                    cond_ptr = mem[p].hh.v.RH;
                    free_node(p, IF_NODE_SIZE);
                }
            }
            change_if_limit(OR_CODE, save_cond_ptr);
            return;
        }
        break;
    case 21:
        {
            save_scanner_status = scanner_status;
            scanner_status = NORMAL;
            get_next();
            scanner_status = save_scanner_status;
            if (cur_cs < HASH_BASE)
                m = prim_lookup(cur_cs - 257);
            else
                m = prim_lookup(hash[cur_cs].v.RH);
            b = ((cur_cmd != UNDEFINED_CS) && (m != UNDEFINED_PRIMITIVE)
                 && (cur_cmd == prim_eqtb[m].hh.u.B0) && (cur_chr == prim_eqtb[m].hh.v.RH));
        }
        break;
    }
    if (is_unless)
        b = !b;
    if (INTPAR(tracing_commands) > 1) {    /*521: */
        begin_diagnostic();
        if (b)
            print(66136L /*"_true_" */ );
        else
            print(66137L /*"_false_" */ );
        end_diagnostic(false);
    }
    if (b) {
        change_if_limit(ELSE_CODE, save_cond_ptr);
        return;
    }
    while (true) {

        pass_text();
        if (cond_ptr == save_cond_ptr) {
            if (cur_chr != OR_CODE)
                goto lab50;
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Extra_));
            }
            print_esc(S(or));
            {
                help_ptr = 1;
                help_line[0] = S(I_m_ignoring_this__it_doesn_/*t match any \if.*/);
            }
            error();
        } else if (cur_chr == FI_CODE) {        /*515: */
            if (if_stack[in_open] == cond_ptr)
                if_warning();
            p = cond_ptr;
            if_line = mem[p + 1].cint;
            cur_if = mem[p].hh.u.B1;
            if_limit = mem[p].hh.u.B0;
            cond_ptr = mem[p].hh.v.RH;
            free_node(p, IF_NODE_SIZE);
        }
    }
 lab50:/*common_ending */ if (cur_chr == FI_CODE) {    /*515: */
        if (if_stack[in_open] == cond_ptr)
            if_warning();
        p = cond_ptr;
        if_line = mem[p + 1].cint;
        cur_if = mem[p].hh.u.B1;
        if_limit = mem[p].hh.u.B0;
        cond_ptr = mem[p].hh.v.RH;
        free_node(p, IF_NODE_SIZE);
    } else
        if_limit = FI_CODE;
}

void begin_name(void)
{
    area_delimiter = 0;
    ext_delimiter = 0;
    quoted_filename = false;
    file_name_quote_char = 0;
}

boolean more_name(UTF16_code c)
{
    register boolean Result;
    if (stop_at_space && (c == 32 /*" " */ ) && (file_name_quote_char == 0))
        Result = false;
    else if (stop_at_space && (file_name_quote_char != 0) && (c == file_name_quote_char)) {
        file_name_quote_char = 0;
        Result = true;
    } else if (stop_at_space && (file_name_quote_char == 0) && ((c == 34 /*""" */ ) || (c == 39 /*"'" */ ))) {
        file_name_quote_char = c;
        quoted_filename = true;
        Result = true;
    } else {

        {
            if (pool_ptr + 1 > pool_size)
                overflow(S(pool_size), pool_size - init_pool_ptr);
        }
        {
            str_pool[pool_ptr] = c;
            pool_ptr++;
        }
        if (IS_DIR_SEP(c)) {
            area_delimiter = (pool_ptr - str_start[(str_ptr) - 65536L]);
            ext_delimiter = 0;
        } else if (c == 46 /*"." */ )
            ext_delimiter = (pool_ptr - str_start[(str_ptr) - 65536L]);
        Result = true;
    }
    return Result;
}

void end_name(void)
{
    str_number temp_str;
    pool_pointer j;
    if (str_ptr + 3 > max_strings)
        overflow(S(number_of_strings), max_strings - init_str_ptr);
    if (area_delimiter == 0)
        cur_area = S();
    else {

        cur_area = str_ptr;
        str_start[(str_ptr + 1) - 65536L] = str_start[(str_ptr) - 65536L] + area_delimiter;
        str_ptr++;
        temp_str = search_string(cur_area);
        if (temp_str > 0) {
            cur_area = temp_str;
            str_ptr--;
            {
                register integer for_end;
                j = str_start[(str_ptr + 1) - 65536L];
                for_end = pool_ptr - 1;
                if (j <= for_end)
                    do {
                        str_pool[j - area_delimiter] = str_pool[j];
                    }
                    while (j++ < for_end);
            }
            pool_ptr = pool_ptr - area_delimiter;
        }
    }
    if (ext_delimiter == 0) {
        cur_ext = S();
        cur_name = slow_make_string();
    } else {

        cur_name = str_ptr;
        str_start[(str_ptr + 1) - 65536L] = str_start[(str_ptr) - 65536L] + ext_delimiter - area_delimiter - 1;
        str_ptr++;
        cur_ext = make_string();
        str_ptr--;
        temp_str = search_string(cur_name);
        if (temp_str > 0) {
            cur_name = temp_str;
            str_ptr--;
            {
                register integer for_end;
                j = str_start[(str_ptr + 1) - 65536L];
                for_end = pool_ptr - 1;
                if (j <= for_end)
                    do {
                        str_pool[j - ext_delimiter + area_delimiter + 1] = str_pool[j];
                    }
                    while (j++ < for_end);
            }
            pool_ptr = pool_ptr - ext_delimiter + area_delimiter + 1;
        }
        cur_ext = slow_make_string();
    }
}

void pack_file_name(str_number n, str_number a, str_number e)
{
    integer k;
    UTF16_code c;
    pool_pointer j;
    k = 0;
    if (name_of_file)
        free(name_of_file);
    name_of_file = xmalloc_array(UTF8_code, (length(a) + length(n) + length(e)) * 3 + 1);
    {
        register integer for_end;
        j = str_start[(a) - 65536L];
        for_end = str_start[(a + 1) - 65536L] - 1;
        if (j <= for_end)
            do {
                c = str_pool[j];
                k++;
                if (k <= INTEGER_MAX) {
                    if ((c < 128))
                        name_of_file[k] = c;
                    else if ((c < 2048)) {
                        name_of_file[k] = 192 + c / 64;
                        k++;
                        name_of_file[k] = 128 + c % 64;
                    } else {

                        name_of_file[k] = 224 + c / 4096;
                        k++;
                        name_of_file[k] = 128 + (c % 4096) / 64;
                        k++;
                        name_of_file[k] = 128 + (c % 4096) % 64;
                    }
                }
            }
            while (j++ < for_end);
    }
    {
        register integer for_end;
        j = str_start[(n) - 65536L];
        for_end = str_start[(n + 1) - 65536L] - 1;
        if (j <= for_end)
            do {
                c = str_pool[j];
                k++;
                if (k <= INTEGER_MAX) {
                    if ((c < 128))
                        name_of_file[k] = c;
                    else if ((c < 2048)) {
                        name_of_file[k] = 192 + c / 64;
                        k++;
                        name_of_file[k] = 128 + c % 64;
                    } else {

                        name_of_file[k] = 224 + c / 4096;
                        k++;
                        name_of_file[k] = 128 + (c % 4096) / 64;
                        k++;
                        name_of_file[k] = 128 + (c % 4096) % 64;
                    }
                }
            }
            while (j++ < for_end);
    }
    {
        register integer for_end;
        j = str_start[(e) - 65536L];
        for_end = str_start[(e + 1) - 65536L] - 1;
        if (j <= for_end)
            do {
                c = str_pool[j];
                k++;
                if (k <= INTEGER_MAX) {
                    if ((c < 128))
                        name_of_file[k] = c;
                    else if ((c < 2048)) {
                        name_of_file[k] = 192 + c / 64;
                        k++;
                        name_of_file[k] = 128 + c % 64;
                    } else {

                        name_of_file[k] = 224 + c / 4096;
                        k++;
                        name_of_file[k] = 128 + (c % 4096) / 64;
                        k++;
                        name_of_file[k] = 128 + (c % 4096) % 64;
                    }
                }
            }
            while (j++ < for_end);
    }
    if (k <= INTEGER_MAX)
        name_length = k;
    else
        name_length = INTEGER_MAX;
    name_of_file[name_length + 1] = 0;
}

str_number
make_name_string(void)
{
    str_number Result;
    integer k;
    pool_pointer save_area_delimiter, save_ext_delimiter;
    boolean save_name_in_progress, save_stop_at_space;

    if ((pool_ptr + name_length > pool_size) || (str_ptr == max_strings)
        || ((pool_ptr - str_start[(str_ptr) - 65536L]) > 0))
        Result = 63 /*"?" */ ;
    else {

        make_utf16_name();
        {
            register integer for_end;
            k = 0;
            for_end = name_length16 - 1;
            if (k <= for_end)
                do {
                    str_pool[pool_ptr] = name_of_file16[k];
                    pool_ptr++;
                }
                while (k++ < for_end);
        }
        Result = make_string();
        save_area_delimiter = area_delimiter;
        save_ext_delimiter = ext_delimiter;
        save_name_in_progress = name_in_progress;
        save_stop_at_space = stop_at_space;
        name_in_progress = true;
        begin_name();
        stop_at_space = false;
        k = 0;
        while ((k < name_length16) && (more_name(name_of_file16[k])))
            k++;
        stop_at_space = save_stop_at_space;
        end_name();
        name_in_progress = save_name_in_progress;
        area_delimiter = save_area_delimiter;
        ext_delimiter = save_ext_delimiter;
    }
    return Result;
}


void scan_file_name(void)
{
    name_in_progress = true;
    begin_name();
    do {
        get_x_token();
    } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
    while (true) {

        if ((cur_cmd > OTHER_CHAR) || (cur_chr > BIGGEST_CHAR)) {
            back_input();
            goto done;
        }
        if (!more_name(cur_chr))
            goto done;
        get_x_token();
    }
done:
    end_name();
    name_in_progress = false;
}

void pack_job_name(str_number s)
{
    cur_area = S();
    cur_ext = s;
    cur_name = job_name;
    pack_file_name(cur_name, cur_area, cur_ext);
}


void
open_log_file(void)
{
    CACHE_THE_EQTB;
    unsigned char old_setting;
    integer k;
    integer l;

    old_setting = selector;
    if (job_name == 0)
        job_name = get_job_name(S(texput));

    pack_job_name(S(_log));

    log_file = ttstub_output_open (name_of_file + 1, 0);
    if (log_file == NULL)
	_tt_abort ("cannot open log file output \"%s\"", name_of_file + 1);

    texmf_log_name = make_name_string();
    selector = SELECTOR_LOG_ONLY;
    log_opened = true;

    input_stack[input_ptr] = cur_input;

    /* Here we catch the log file up with anything that has already been
     * printed. The eqtb reference is end_line_char. */

    print_nl(S(___Z11/*"**"*/));
    l = input_stack[0].limit;
    if (buffer[l] == INTPAR(end_line_char))
        l--;

    for (k = 1; k <= l; k++)
	print(buffer[k]);

    print_ln();
    selector = old_setting + 2;
}


void
start_input(void)
{
    CACHE_THE_EQTB;
    str_number temp_str;
    integer k;

    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
    begin_file_reading();

    if (!u_open_in(&input_file[cur_input.index], kpse_tex_format, "rb",
		  STATEINT(xetex_default_input_mode), STATEINT(xetex_default_input_encoding)))
	_tt_abort ("failed to open input file \"%s\"", name_of_file + 1);

    make_utf16_name();
    name_in_progress = true;
    begin_name();
    stop_at_space = false;
    k = 0;
    while (k < name_length16 && more_name(name_of_file16[k]))
	k++;
    stop_at_space = true;
    end_name();
    name_in_progress = false;

    cur_input.name = make_name_string();
    source_filename_stack[in_open] = cur_input.name;
    full_source_filename_stack[in_open] = make_full_name_string();
    if (cur_input.name == str_ptr - 1) {
        temp_str = search_string(cur_input.name);
        if (temp_str > 0) {
            cur_input.name = temp_str;
	    str_ptr--;
	    pool_ptr = str_start[(str_ptr) - 65536L];
        }
    }

    if (job_name == 0) {
        job_name = get_job_name(cur_name);
        open_log_file();
    }

    if (term_offset + length(full_source_filename_stack[in_open]) > max_print_line - 2)
        print_ln();
    else if ((term_offset > 0) || (file_offset > 0))
        print_char(32 /*" " */ );
    print_char(40 /*"(" */ );
    open_parens++;
    print(full_source_filename_stack[in_open]);
    ttstub_output_flush (rust_stdout);

    cur_input.state = NEW_LINE;

    synctex_start_input();

    line = 1;
    input_line(input_file[cur_input.index]);
    cur_input.limit = last;
    if ((INTPAR(end_line_char) < 0) || (INTPAR(end_line_char) > 255))
	cur_input.limit--;
    else
	buffer[cur_input.limit] = INTPAR(end_line_char);
    first = cur_input.limit + 1;
    cur_input.loc = cur_input.start;
}


four_quarters
effective_char_info(internal_font_number f, uint16_t c)
{
    if (!xtx_ligature_present && font_mapping[f] != NULL)
        c = apply_tfm_font_mapping(font_mapping[f], c);

    xtx_ligature_present = false;
    return font_info[char_base[f] + c].qqqq;
}


void char_warning(internal_font_number f, integer c)
{
    CACHE_THE_EQTB;
    integer old_setting;

    if (INTPAR(tracing_lost_chars) > 0) {
        old_setting = INTPAR(tracing_online);
        if (INTPAR(tracing_lost_chars) > 1)
            INTPAR(tracing_online) = 1;

        begin_diagnostic();
        print_nl(S(Missing_character__There_is_/*no */));
        if (c < 65536L)
            print(c);
        else
            print_char(c);
        print(S(_in_font_));
        print(font_name[f]);
        print_char(33 /*"!" */ );
        end_diagnostic(false);

        INTPAR(tracing_online) = old_setting;
    }
}

int32_t new_native_word_node(internal_font_number f, integer n)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    memory_word *mem = zmem;
    integer l;
    int32_t q;

    l = NATIVE_NODE_SIZE + (n * sizeof(UTF16_code) + sizeof(memory_word) - 1) / sizeof(memory_word);
    q = get_node(l);
    mem[q].hh.u.B0 = WHATSIT_NODE;
    if ((STATEINT(xetex_generate_actual_text) > 0))
        mem[q].hh.u.B1 = NATIVE_WORD_NODE_AT;
    else
        mem[q].hh.u.B1 = NATIVE_WORD_NODE;
    mem[q + 4].qqqq.u.B0 = l;
    mem[q + 4].qqqq.u.B1 = f;
    mem[q + 4].qqqq.u.B2 = n;
    mem[q + 4].qqqq.u.B3 = 0;
    mem[q + 5].ptr = NULL;
    Result = q;
    return Result;
}

int32_t new_native_character(internal_font_number f, UnicodeScalar c)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    memory_word *mem = zmem;
    int32_t p;
    integer i, len;

    if (font_mapping[f] != 0) {
        if (c > 65535L) {
            {
                if (pool_ptr + 2 > pool_size)
                    overflow(S(pool_size), pool_size - init_pool_ptr);
            }
            {
                str_pool[pool_ptr] = (c - 65536L) / 1024 + 55296L;
                pool_ptr++;
            }
            {
                str_pool[pool_ptr] = (c - 65536L) % 1024 + 56320L;
                pool_ptr++;
            }
        } else {

            {
                if (pool_ptr + 1 > pool_size)
                    overflow(S(pool_size), pool_size - init_pool_ptr);
            }
            {
                str_pool[pool_ptr] = c;
                pool_ptr++;
            }
        }
        len =
            apply_mapping(font_mapping[f], &(str_pool[str_start[(str_ptr) - 65536L]]),
                          (pool_ptr - str_start[(str_ptr) - 65536L]));
        pool_ptr = str_start[(str_ptr) - 65536L];
        i = 0;
        while (i < len) {

            if ((mapped_text[i] >= 55296L) && (mapped_text[i] < 56320L)) {
                c = (mapped_text[i] - 55296L) * 1024 + mapped_text[i + 1] + 9216;
                if (map_char_to_glyph(f, c) == 0) {
                    char_warning(f, c);
                }
                i = i + 2;
            } else {

                if (map_char_to_glyph(f, mapped_text[i]) == 0) {
                    char_warning(f, mapped_text[i]);
                }
                i = i + 1;
            }
        }
        p = new_native_word_node(f, len);
        {
            register integer for_end;
            i = 0;
            for_end = len - 1;
            if (i <= for_end)
                do {
                    set_native_char(p, i, mapped_text[i]);
                }
                while (i++ < for_end);
        }
    } else {

        if (INTPAR(tracing_lost_chars) > 0) {

            if (map_char_to_glyph(f, c) == 0) {
                char_warning(f, c);
            }
        }
        p = get_node((NATIVE_NODE_SIZE + 1));
        mem[p].hh.u.B0 = WHATSIT_NODE;
        mem[p].hh.u.B1 = NATIVE_WORD_NODE;
        mem[p + 4].qqqq.u.B0 = (NATIVE_NODE_SIZE + 1);
        mem[p + 4].qqqq.u.B3 = 0;
        mem[p + 5].ptr = NULL;
        mem[p + 4].qqqq.u.B1 = f;
        if (c > 65535L) {
            mem[p + 4].qqqq.u.B2 = 2;
            set_native_char(p, 0, (c - 65536L) / 1024 + 55296L);
            set_native_char(p, 1, (c - 65536L) % 1024 + 56320L);
        } else {

            mem[p + 4].qqqq.u.B2 = 1;
            set_native_char(p, 0, c);
        }
    }
    set_native_metrics(p, (STATEINT(xetex_use_glyph_metrics) > 0));
    Result = p;
    return Result;
}

void font_feature_warning(void *featureNameP, integer featLen, void *settingNameP, integer setLen)
{

    integer i;

    begin_diagnostic();
    print_nl(S(Unknown_));
    if (setLen > 0) {
        print(S(selector__));
        print_utf8_str(settingNameP, setLen);
        print(S(__for_));
    }
    print(S(feature__));
    print_utf8_str(featureNameP, featLen);
    print(S(__in_font__));
    i = 1;
    while (name_of_file[i] != 0) {
        print_raw_char(name_of_file[i], true);
        i++;
    }
    print(S(___Z10/*"'."*/));
    end_diagnostic(false);
}

void font_mapping_warning(void *mappingNameP, integer mappingNameLen, integer warningType)
{

    integer i;

    begin_diagnostic();
    if (warningType == 0)
        print_nl(S(Loaded_mapping__));
    else
        print_nl(S(Font_mapping__));
    print_utf8_str(mappingNameP, mappingNameLen);
    print(S(__for_font__));
    i = 1;
    while (name_of_file[i] != 0) {
        print_raw_char(name_of_file[i], true);
        i++;
    }
    switch (warningType) {
    case 1:
        print(S(__not_found_));
        break;
    case 2:
        {
            print(S(__not_usable_));
            print_nl(S(bad_mapping_file_or_incorrec/*t mapping type.*/));
        }
        break;
    default:
        print(S(___Z10/*"'."*/));
        break;
    }
    end_diagnostic(false);
}

void graphite_warning(void)
{

    integer i;

    begin_diagnostic();
    print_nl(S(Font__));
    i = 1;
    while (name_of_file[i] != 0) {
        print_raw_char(name_of_file[i], true);
        i++;
    }
    print(S(__does_not_support_Graphite_/* Trying OpenType layout instead.*/));
    end_diagnostic(false);
}

internal_font_number load_native_font(int32_t u, str_number nom, str_number aire, scaled s)
{
    CACHE_THE_EQTB;

    /*done */
#define first_math_fontdimen ( 10 )
    register internal_font_number Result;
    memory_word *mem = zmem; integer k, num_font_dimens;
    void *font_engine;
    scaled actual_size;
    int32_t p;
    scaled ascent, descent, font_slant, x_ht, cap_ht;
    internal_font_number f;
    str_number full_name;
    Result = FONT_BASE;
    font_engine = find_native_font(name_of_file + 1, s);
    if (font_engine == 0)
        goto done;
    if (s >= 0)
        actual_size = s;
    else {

        if ((s != -1000))
            actual_size = xn_over_d(loaded_font_design_size, -(integer) s, 1000);
        else
            actual_size = loaded_font_design_size;
    }
    {
        if (pool_ptr + name_length > pool_size)
            overflow(S(pool_size), pool_size - init_pool_ptr);
    }
    {
        register integer for_end;
        k = 1;
        for_end = name_length;
        if (k <= for_end)
            do {
                str_pool[pool_ptr] = name_of_file[k];
                pool_ptr++;
            }
            while (k++ < for_end);
    }
    full_name = make_string();
    {
        register integer for_end;
        f = (FONT_BASE + 1);
        for_end = font_ptr;
        if (f <= for_end)
            do
                if ((font_area[f] == native_font_type_flag) && str_eq_str(font_name[f], full_name)
                    && (font_size[f] == actual_size)) {
                    release_font_engine(font_engine, native_font_type_flag);
                    {
                        str_ptr--;
                        pool_ptr = str_start[(str_ptr) - 65536L];
                    }
                    Result = f;
                    goto done;
                }
            while (f++ < for_end) ;
    }
    if ((native_font_type_flag == OTGR_FONT_FLAG) && isOpenTypeMathFont(font_engine))
        num_font_dimens = first_math_fontdimen + 55;
    else
        num_font_dimens = 8;
    if ((font_ptr == font_max) || (fmem_ptr + num_font_dimens > font_mem_size)) {
        {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Font_));
            }
            sprint_cs(u);
            print_char(61 /*"=" */ );
            if (file_name_quote_char != 0)
                print_char(file_name_quote_char);
            print_file_name(nom, aire, cur_ext);
            if (file_name_quote_char != 0)
                print_char(file_name_quote_char);
            if (s >= 0) {
                print(S(_at_));
                print_scaled(s);
                print(S(pt));
            } else if (s != -1000) {
                print(S(_scaled_));
                print_int(-(integer) s);
            }
            print(S(_not_loaded__Not_enough_room/* left*/));
            {
                help_ptr = 4;
                help_line[3] = S(I_m_afraid_I_won_t_be_able_t/*o make use of this font,*/);
                help_line[2] = S(because_my_memory_for_charac/*ter-size data is too small.*/);
                help_line[1] = S(If_you_re_really_stuck__ask_/*a wizard to enlarge me.*/);
                help_line[0] = S(Or_maybe_try__I_font_same_fo/*nt id>=<name of loaded font>'.*/);
            }
            error();
            goto done;
        }
    }
    font_ptr++;
    font_area[font_ptr] = native_font_type_flag;
    font_name[font_ptr] = full_name;
    font_check[font_ptr].u.B0 = 0;
    font_check[font_ptr].u.B1 = 0;
    font_check[font_ptr].u.B2 = 0;
    font_check[font_ptr].u.B3 = 0;
    font_glue[font_ptr] = MIN_HALFWORD;
    font_dsize[font_ptr] = loaded_font_design_size;
    font_size[font_ptr] = actual_size;
    if ((native_font_type_flag == AAT_FONT_FLAG)) {
        aat_get_font_metrics(font_engine, &ascent, &descent, &x_ht, &cap_ht,
                             &font_slant);
    } else {

        ot_get_font_metrics(font_engine, &ascent, &descent, &x_ht, &cap_ht,
                            &font_slant);
    }
    height_base[font_ptr] = ascent;
    depth_base[font_ptr] = -(integer) descent;
    font_params[font_ptr] = num_font_dimens;
    font_bc[font_ptr] = 0;
    font_ec[font_ptr] = 65535L;
    font_used[font_ptr] = false;
    hyphen_char[font_ptr] = INTPAR(default_hyphen_char);
    skew_char[font_ptr] = INTPAR(default_skew_char);
    param_base[font_ptr] = fmem_ptr - 1;
    font_layout_engine[font_ptr] = font_engine;
    font_mapping[font_ptr] = 0;
    font_letter_space[font_ptr] = loaded_font_letter_space;
    p = new_native_character(font_ptr, 32 /*" " */ );
    s = mem[p + 1].cint + loaded_font_letter_space;
    free_node(p, mem[p + 4].qqqq.u.B0);
    font_info[fmem_ptr].cint = font_slant;
    fmem_ptr++;
    font_info[fmem_ptr].cint = s;
    fmem_ptr++;
    font_info[fmem_ptr].cint = s / 2;
    fmem_ptr++;
    font_info[fmem_ptr].cint = s / 3;
    fmem_ptr++;
    font_info[fmem_ptr].cint = x_ht;
    fmem_ptr++;
    font_info[fmem_ptr].cint = font_size[font_ptr];
    fmem_ptr++;
    font_info[fmem_ptr].cint = s / 3;
    fmem_ptr++;
    font_info[fmem_ptr].cint = cap_ht;
    fmem_ptr++;
    if (num_font_dimens == first_math_fontdimen + 55) {
        font_info[fmem_ptr].cint = num_font_dimens;
        fmem_ptr++;
        {
            register integer for_end;
            k = 0;
            for_end = LASTMATHCONSTANT;
            if (k <= for_end)
                do {
                    font_info[fmem_ptr].cint = get_ot_math_constant(font_ptr, k);
                    fmem_ptr++;
                }
                while (k++ < for_end);
        }
    }
    font_mapping[font_ptr] = loaded_font_mapping;
    font_flags[font_ptr] = loaded_font_flags;
    Result = font_ptr;
done:
    return Result;
}

void do_locale_linebreaks(integer s, integer len)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer offs, prevOffs, i;
    boolean use_penalty, use_skip;

    if ((INTPAR(xetex_linebreak_locale) == 0) || (len == 1)) {
        mem[cur_list.tail].hh.v.RH = new_native_word_node(main_f, len);
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
        {
            register integer for_end;
            i = 0;
            for_end = len - 1;
            if (i <= for_end)
                do
                    set_native_char(cur_list.tail, i, native_text[s + i]);
                while (i++ < for_end);
        }
        set_native_metrics(cur_list.tail, (STATEINT(xetex_use_glyph_metrics) > 0));
    } else {

        use_skip = GLUEPAR(xetex_linebreak_skip) != 0;
        use_penalty = INTPAR(xetex_linebreak_penalty) != 0 || !use_skip;
        linebreak_start(main_f, INTPAR(xetex_linebreak_locale), native_text + s, len);
        offs = 0;
        do {
            prevOffs = offs;
            offs = linebreak_next();
            if (offs > 0) {
                if (prevOffs != 0) {
                    if (use_penalty) {
                        mem[cur_list.tail].hh.v.RH = new_penalty(INTPAR(xetex_linebreak_penalty));
                        cur_list.tail = mem[cur_list.tail].hh.v.RH;
                    }
                    if (use_skip) {
                        mem[cur_list.tail].hh.v.RH = new_param_glue(GLUE_PAR__xetex_linebreak_skip);
                        cur_list.tail = mem[cur_list.tail].hh.v.RH;
                    }
                }
                mem[cur_list.tail].hh.v.RH = new_native_word_node(main_f, offs - prevOffs);
                cur_list.tail = mem[cur_list.tail].hh.v.RH;
                {
                    register integer for_end;
                    i = prevOffs;
                    for_end = offs - 1;
                    if (i <= for_end)
                        do
                            set_native_char(cur_list.tail, i - prevOffs, native_text[s + i]);
                        while (i++ < for_end);
                }
                set_native_metrics(cur_list.tail, (STATEINT(xetex_use_glyph_metrics) > 0));
            }
        } while (!(offs < 0));
    }
}

void bad_utf8_warning(void)
{
    begin_diagnostic();
    print_nl(S(Invalid_UTF_8_byte_or_sequen/*ce*/));
    if ((cur_input.name == 0))
        print(S(_in_terminal_input));
    else {

        print(S(_at_line_));
        print_int(line);
    }
    print(S(_replaced_by_U_FFFD_));
    end_diagnostic(false);
}

integer get_input_normalization_state(void)
{
    CACHE_THE_EQTB;
    register integer Result;

    if (eqtb == NULL)
        Result = 0;
    else
        Result = STATEINT(xetex_input_normalization);
    return Result;
}

integer get_tracing_fonts_state(void)
{
    CACHE_THE_EQTB;

    return STATEINT(xetex_tracing_fonts);
}

internal_font_number
read_font_info(int32_t u, str_number nom, str_number aire, scaled s)
{
    CACHE_THE_EQTB;
    font_index k;
    boolean name_too_long;
    boolean file_opened;
    int32_t lf, lh, bc, ec, nw, nh, nd, ni, nl, nk, ne, np;
    internal_font_number f;
    internal_font_number g;
    eight_bits a, b, c, d;
    four_quarters qw;
    scaled sw;
    integer bch_label;
    short bchar;
    scaled z;
    integer alpha;
    unsigned char beta;
    rust_input_handle_t tfm_file;

    g = FONT_BASE;

    file_opened = false;
    pack_file_name(nom, aire, cur_ext);

    if (STATEINT(xetex_tracing_fonts) > 0) {
        begin_diagnostic();
        print_nl(S(Requested_font__));
        print_c_string((string) (name_of_file + 1));
        print('"');
        if (s < 0) {
            print(S(_scaled_));
            print_int(-(integer) s);
        } else {
            print(S(_at_));
            print_scaled(s);
            print(S(pt));
        }
        end_diagnostic(false);
    }

    if (quoted_filename) {
        g = load_native_font(u, nom, aire, s);
        if (g != FONT_BASE)
            goto done;
    }

    name_too_long = (length(nom) > 255 || length(aire) > 255);
    if (name_too_long)
        goto bad_tfm;

    pack_file_name(nom, aire, S());
    check_for_tfm_font_mapping();

    tfm_file = tt_open_input (kpse_tfm_format);
    if (tfm_file == NULL) {
	if (!quoted_filename) {
	    g = load_native_font(u, nom, aire, s);
	    if (g != FONT_BASE)
		goto done;
	}
	goto bad_tfm;
    }

    file_opened = true; /*:582*/

    /* We are a bit cavalier about EOF-checking since we can't very
     * conveniently implement feof() in the Rust layer, and it only ever is
     * used in this one place. */

#define READFIFTEEN(x) do { \
	x = ttstub_input_getc (tfm_file); \
	if (x > 127 || x == EOF) \
	    goto bad_tfm; \
	x *= 256; \
	x += ttstub_input_getc (tfm_file);\
    } while (0)

    READFIFTEEN(lf);
    READFIFTEEN(lh);
    READFIFTEEN(bc);
    READFIFTEEN(ec);

    if (bc > ec + 1 || ec > 255)
	goto bad_tfm;

    if (bc > 255) {
	bc = 1;
	ec = 0;
    }

    READFIFTEEN(nw);
    READFIFTEEN(nh);
    READFIFTEEN(nd);
    READFIFTEEN(ni);
    READFIFTEEN(nl);
    READFIFTEEN(nk);
    READFIFTEEN(ne);
    READFIFTEEN(np);

    if (lf != 6 + lh + (ec - bc + 1) + nw + nh + nd + ni + nl + nk + ne + np)
	goto bad_tfm;
    if (nw == 0 || nh == 0 || nd == 0 || ni == 0)
	goto bad_tfm;

    lf = lf - 6 - lh;
    if (np < 7)
	lf = lf + 7 - np;

    if (font_ptr == font_max || fmem_ptr + lf > font_mem_size)
	_tt_abort("not enough memory to load another font");

    f = font_ptr + 1;
    char_base[f] = fmem_ptr - bc;
    width_base[f] = char_base[f] + ec + 1;
    height_base[f] = width_base[f] + nw;
    depth_base[f] = height_base[f] + nh;
    italic_base[f] = depth_base[f] + nd;
    lig_kern_base[f] = italic_base[f] + ni;
    kern_base[f] = lig_kern_base[f] + nl - 256 * (128);
    exten_base[f] = kern_base[f] + 256 * (128) + nk;
    param_base[f] = exten_base[f] + /*:585 */ ne;

    if (lh < 2)
	goto bad_tfm;

    qw.u.B0 = a = ttstub_input_getc (tfm_file);
    qw.u.B1 = b = ttstub_input_getc (tfm_file);
    qw.u.B2 = c = ttstub_input_getc (tfm_file);
    qw.u.B3 = d = ttstub_input_getc (tfm_file);
    if (a == EOF || b == EOF || c == EOF || d == EOF)
	goto bad_tfm;
    font_check[f] = qw;

    READFIFTEEN(z);
    z = z * 256 + ttstub_input_getc (tfm_file);
    z = (z * 16) + (ttstub_input_getc (tfm_file) / 16);
    if (z < 65536L)
	goto bad_tfm;

    while (lh > 2) {
	ttstub_input_getc (tfm_file);
	ttstub_input_getc (tfm_file);
	ttstub_input_getc (tfm_file);
	ttstub_input_getc (tfm_file);
	lh--;
    }

    font_dsize[f] = z;
    if (s != -1000) {
	if (s >= 0)
	    z = s;
	else
	    z = xn_over_d(z, -(integer) s, 1000);
    }

    font_size[f] = z;

    for (k = fmem_ptr; k <= width_base[f] - 1; k++) {
	qw.u.B0 = a = ttstub_input_getc (tfm_file);
	qw.u.B1 = b = ttstub_input_getc (tfm_file);
	qw.u.B2 = c = ttstub_input_getc (tfm_file);
	qw.u.B3 = d = ttstub_input_getc (tfm_file);
	if (a == EOF || b == EOF || c == EOF || d == EOF)
	    goto bad_tfm;
	font_info[k].qqqq = qw;

	if (a >= nw || b / 16 >= nh || b % 16 >= nd || c / 4 >= ni)
	    goto bad_tfm;

	switch (c % 4) {
	case 1:
	    if (d >= nl)
		goto bad_tfm;
	    break;
	case 3:
	    if (d >= ne)
		goto bad_tfm;
	    break;
	case 2:
	    if (d < bc || d > ec)
		goto bad_tfm;

	    while (d < k + bc - fmem_ptr) {
		qw = font_info[char_base[f] + d].qqqq;
		if ((qw.u.B2 % 4) != LIST_TAG)
		    goto not_found;
		d = qw.u.B3;
	    }

	    if (d == k + bc - fmem_ptr)
		goto bad_tfm;

	not_found:
	    break;
	}
    }

    alpha = 16;
    while (z >= 8388608L) {
	z = z / 2;
	alpha = alpha + alpha;
    }
    beta = 256 / alpha;
    alpha = alpha * z;

    for (k = width_base[f]; k <= lig_kern_base[f] - 1; k++) {
	a = ttstub_input_getc (tfm_file);
	b = ttstub_input_getc (tfm_file);
	c = ttstub_input_getc (tfm_file);
	d = ttstub_input_getc (tfm_file);
	if (a == EOF || b == EOF || c == EOF || d == EOF)
	    goto bad_tfm;
	sw = (((((d * z) / 256) + c * z) / 256) + b * z) / beta;

	if (a == 0)
	    font_info[k].cint = sw;
	else if (a == 255)
	    font_info[k].cint = sw - alpha;
	else
	    goto bad_tfm;
    }

    if (font_info[width_base[f]].cint != 0)
	goto bad_tfm;
    if (font_info[height_base[f]].cint != 0)
	goto bad_tfm;
    if (font_info[depth_base[f]].cint != 0)
	goto bad_tfm;
    if (font_info[italic_base[f]].cint != 0)
	goto bad_tfm;

    bch_label = 32767;
    bchar = 256;

    if (nl > 0) {
	for (k = lig_kern_base[f]; k <= kern_base[f] + 256 * 128 - 1; k++) {
	    qw.u.B0 = a = ttstub_input_getc (tfm_file);
	    qw.u.B1 = b = ttstub_input_getc (tfm_file);
	    qw.u.B2 = c = ttstub_input_getc (tfm_file);
	    qw.u.B3 = d = ttstub_input_getc (tfm_file);
	    if (a == EOF || b == EOF || c == EOF || d == EOF)
		goto bad_tfm;
	    font_info[k].qqqq = qw;

	    if (a > 128) {
		if (256 * c + d >= nl)
		    goto bad_tfm;

		if (a == 255 && k == lig_kern_base[f])
		    bchar = b;
	    } else {
		if (b != bchar) {
		    if ((b < bc) || (b > ec))
			goto bad_tfm;

		    qw = font_info[char_base[f] + b].qqqq;
		    if (!(qw.u.B0 > 0))
			goto bad_tfm;
		}

		if (c < 128) {
		    if ((d < bc) || (d > ec))
			goto bad_tfm;
		    qw = font_info[char_base[f] + d].qqqq;
		    if (!(qw.u.B0 > 0))
			goto bad_tfm;
		} else if (256 * (c - 128) + d >= nk)
		    goto bad_tfm;

		if (a < 128 && k - lig_kern_base[f] + a + 1 >= nl)
		    goto bad_tfm;
	    }
	}

	if (a == 255)
	    bch_label = 256 * c + d;
    }

    for (k = kern_base[f] + 256 * 128; k <= exten_base[f] - 1; k++) {
	a = ttstub_input_getc (tfm_file);
	b = ttstub_input_getc (tfm_file);
	c = ttstub_input_getc (tfm_file);
	d = ttstub_input_getc (tfm_file);
	if (a == EOF || b == EOF || c == EOF || d == EOF)
	    goto bad_tfm;
	sw = (((((d * z) / 256) + c * z) / 256) + b * z) / beta;

	if (a == 0)
	    font_info[k].cint = sw;
	else if (a == 255)
	    font_info[k].cint = sw - alpha;
	else
	    goto bad_tfm;
    }

    for (k = exten_base[f]; k <= param_base[f] - 1; k++) {
	qw.u.B0 = a = ttstub_input_getc (tfm_file);
	qw.u.B1 = b = ttstub_input_getc (tfm_file);
	qw.u.B2 = c = ttstub_input_getc (tfm_file);
	qw.u.B3 = d = ttstub_input_getc (tfm_file);
	if (a == EOF || b == EOF || c == EOF || d == EOF)
	    goto bad_tfm;
	font_info[k].qqqq = qw;

	if (a != 0) {
	    if ((a < bc) || (a > ec))
		goto bad_tfm;
	    qw = font_info[char_base[f] + a].qqqq;
	    if (!(qw.u.B0 > 0))
		goto bad_tfm;
	}

	if (b != 0) {
	    if ((b < bc) || (b > ec))
		goto bad_tfm;
	    qw = font_info[char_base[f] + b].qqqq;
	    if (!(qw.u.B0 > 0))
		goto bad_tfm;
	}

	if (c != 0) {
	    if ((c < bc) || (c > ec))
		goto bad_tfm;
	    qw = font_info[char_base[f] + c].qqqq;
	    if (!(qw.u.B0 > 0))
		goto bad_tfm;
	}

	if ((d < bc) || (d > ec))
	    goto bad_tfm;
	qw = font_info[char_base[f] + d].qqqq;
	if (!(qw.u.B0 > 0))
	    goto bad_tfm;
    }

    for (k = 1; k <= np; k++) {
	if (k == 1) {
	    sw = ttstub_input_getc (tfm_file);
	    if (sw == EOF)
		goto bad_tfm;
	    if (sw > 127)
		sw = sw - 256;

	    sw = sw * 256 + ttstub_input_getc (tfm_file);
	    sw = sw * 256 + ttstub_input_getc (tfm_file);
	    font_info[param_base[f]].cint = (sw * 16) + (ttstub_input_getc (tfm_file) / 16);
	} else {
	    a = ttstub_input_getc (tfm_file);
	    b = ttstub_input_getc (tfm_file);
	    c = ttstub_input_getc (tfm_file);
	    d = ttstub_input_getc (tfm_file);
	    if (a == EOF || b == EOF || c == EOF || d == EOF)
		goto bad_tfm;
	    sw = (((((d * z) / 256) + c * z) / 256) + b * z) / beta;

	    if (a == 0)
		font_info[param_base[f] + k - 1].cint = sw;
	    else if (a == 255)
		font_info[param_base[f] + k - 1].cint = sw - alpha;
	    else
		goto bad_tfm;
	}
    }

    for (k = np + 1; k <= 7; k++)
	font_info[param_base[f] + k - 1].cint = 0;

    if (np >= 7)
	font_params[f] = np;
    else
	font_params[f] = 7;

    hyphen_char[f] = INTPAR(default_hyphen_char);
    skew_char[f] = INTPAR(default_skew_char);
    if (bch_label < nl)
	bchar_label[f] = bch_label + lig_kern_base[f];
    else
	bchar_label[f] = NON_ADDRESS;
    font_bchar[f] = bchar;
    font_false_bchar[f] = bchar;

    if (bchar <= ec) {
	if (bchar >= bc) {
	    qw = font_info[char_base[f] + bchar].qqqq;
	    if ((qw.u.B0 > 0))
		font_false_bchar[f] = TOO_BIG_CHAR;
	}
    }

    font_name[f] = nom;
    font_area[f] = aire;
    font_bc[f] = bc;
    font_ec[f] = ec;
    font_glue[f] = MIN_HALFWORD;
    char_base[f] = char_base[f];
    width_base[f] = width_base[f];
    lig_kern_base[f] = lig_kern_base[f];
    kern_base[f] = kern_base[f];
    exten_base[f] = exten_base[f];
    param_base[f]--;
    fmem_ptr = fmem_ptr + lf;
    font_ptr = f;
    g = f;
    font_mapping[f] = load_tfm_font_mapping();
    goto done;

bad_tfm:
    if (INTPAR(suppress_fontnotfound_error) == 0) {
	/* NOTE: must preserve this path to keep passing the TRIP tests */

	if (file_line_error_style_p)
	    print_file_line();
	else
	    print_nl(S(__/*"! "*/));
	print(S(Font_));
        sprint_cs(u);
        print_char(61 /*"=" */ );
        if (file_name_quote_char != 0)
            print_char(file_name_quote_char);
        print_file_name(nom, aire, cur_ext);
        if (file_name_quote_char != 0)
            print_char(file_name_quote_char);
        if (s >= 0) {
            print(S(_at_));
            print_scaled(s);
            print(S(pt));
        } else if (s != -1000) {
            print(S(_scaled_));
            print_int(-(integer) s);
        }

        if (file_opened)
            print(S(_not_loadable__Bad_metric__T/*FM) file*/));
        else if (name_too_long)
            print(S(_not_loadable__Metric__TFM__/*file name too long*/));
        else
            print(S(_not_loadable__Metric__TFM___Z1/*" not loadable: Metric (TFM) file or installed font not found"*/));

	help_ptr = 5;
	help_line[4] = S(I_wasn_t_able_to_read_the_si/*ze data for this font,*/);
	help_line[3] = S(so_I_will_ignore_the_font_sp/*ecification.*/);
	help_line[2] = S(_Wizards_can_fix_TFM_files_u/*sing TFtoPL/PLtoTF.]*/);
	help_line[1] = S(You_might_try_inserting_a_di/*fferent font spec;*/);
	help_line[0] = S(e_g___type__I_font_same_font/* id>=<substitute font name>'.*/);

        error();
    }

done:
    if (file_opened)
        ttstub_input_close (tfm_file);

    if (STATEINT(xetex_tracing_fonts) > 0) {
        if (g == FONT_BASE) {
            begin_diagnostic();
            print_nl(S(____font_not_found__using__n/*ullfont"*/));
            end_diagnostic(false);
        } else if (file_opened) {
            begin_diagnostic();
            print_nl(S(_____Z1/*" -> "*/));
            print_c_string((string) (name_of_file + 1));
            end_diagnostic(false);
        }
    }

    return g;
}

int32_t new_character(internal_font_number f, UTF16_code c)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    uint16_t ec;
    if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))) {
        Result = new_native_character(f, c);
        return Result;
    }
    ec = effective_char(false, f, c);
    if (font_bc[f] <= ec) {

        if (font_ec[f] >= ec) {

            if ((font_info[char_base[f] + ec].qqqq.u.B0 > 0)) {
                p = get_avail();
                mem[p].hh.u.B0 = f;
                mem[p].hh.u.B1 = c;
                Result = p;
                return Result;
            }
        }
    }
    char_warning(f, c);
    Result = MIN_HALFWORD;
    return Result;
}

void dvi_swap(void)
{
    if (dvi_ptr > (TEX_INFINITY - dvi_offset)) {
        cur_s = -2;
        fatal_error(S(dvi_length_exceeds__7FFFFFFF/**/));
    }
    if (dvi_limit == dvi_buf_size) {
        write_to_dvi(0, half_buf - 1);
        dvi_limit = half_buf;
        dvi_offset = dvi_offset + dvi_buf_size;
        dvi_ptr = 0;
    } else {

        write_to_dvi(half_buf, dvi_buf_size - 1);
        dvi_limit = dvi_buf_size;
    }
    dvi_gone = dvi_gone + half_buf;
}


void
dvi_four(integer x)
{
    if (x >= 0) {
        dvi_buf[dvi_ptr] = x / 0x1000000;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    } else {
        x = x + 0x40000000;
        x = x + 0x40000000;

        dvi_buf[dvi_ptr] = (x / 0x1000000) + 128;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    }

    x = x % 0x1000000;
    dvi_buf[dvi_ptr] = x / 0x10000;
    dvi_ptr++;
    if (dvi_ptr == dvi_limit)
        dvi_swap();

    x = x % 0x10000;
    dvi_buf[dvi_ptr] = x / 0x100;
    dvi_ptr++;
    if (dvi_ptr == dvi_limit)
        dvi_swap();

    dvi_buf[dvi_ptr] = x % 0x100;
    dvi_ptr++;
    if (dvi_ptr == dvi_limit)
        dvi_swap();
}


void
dvi_two(UTF16_code s)
{
    dvi_buf[dvi_ptr] = s / 0x100;
    dvi_ptr++;
    if (dvi_ptr == dvi_limit)
        dvi_swap();

    dvi_buf[dvi_ptr] = s % 0x100;
    dvi_ptr++;
    if (dvi_ptr == dvi_limit)
        dvi_swap();
}


void dvi_pop(integer l)
{
    if ((l == dvi_offset + dvi_ptr) && (dvi_ptr > 0))
        dvi_ptr--;
    else {

        dvi_buf[dvi_ptr] = POP;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    }
}

void dvi_native_font_def(internal_font_number f)
{
    integer font_def_length, i;
    {
        dvi_buf[dvi_ptr] = DEFINE_NATIVE_FONT;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    }
    dvi_four(f - 1);
    font_def_length = make_font_def(f);
    {
        register integer for_end;
        i = 0;
        for_end = font_def_length - 1;
        if (i <= for_end)
            do {
                dvi_buf[dvi_ptr] = xdv_buffer[i];
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
            while (i++ < for_end);
    }
}

void dvi_font_def(internal_font_number f)
{
    pool_pointer k;
    integer l;
    if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG)))
        dvi_native_font_def(f);
    else {

        if (f <= 256) {
            {
                dvi_buf[dvi_ptr] = FNT_DEF1;
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
            {
                dvi_buf[dvi_ptr] = f - 1;
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
        } else {

            {
                dvi_buf[dvi_ptr] = (FNT_DEF1 + 1);
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
            {
                dvi_buf[dvi_ptr] = (f - 1) / 256;
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
            {
                dvi_buf[dvi_ptr] = (f - 1) % 256;
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
        }
        {
            dvi_buf[dvi_ptr] = font_check[f].u.B0;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = font_check[f].u.B1;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = font_check[f].u.B2;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = font_check[f].u.B3;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        dvi_four(font_size[f]);
        dvi_four(font_dsize[f]);
        {
            dvi_buf[dvi_ptr] = length(font_area[f]);
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        l = 0;
        k = str_start[(font_name[f]) - 65536L];
        while ((l == 0) && (k < str_start[(font_name[f] + 1) - 65536L])) {

            if (str_pool[k] == 58 /*":" */ )
                l = k - str_start[(font_name[f]) - 65536L];
            k++;
        }
        if (l == 0)
            l = length(font_name[f]);
        {
            dvi_buf[dvi_ptr] = l;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            register integer for_end;
            k = str_start[(font_area[f]) - 65536L];
            for_end = str_start[(font_area[f] + 1) - 65536L] - 1;
            if (k <= for_end)
                do {
                    dvi_buf[dvi_ptr] = str_pool[k];
                    dvi_ptr++;
                    if (dvi_ptr == dvi_limit)
                        dvi_swap();
                }
                while (k++ < for_end);
        }
        {
            register integer for_end;
            k = str_start[(font_name[f]) - 65536L];
            for_end = str_start[(font_name[f]) - 65536L] + l - 1;
            if (k <= for_end)
                do {
                    dvi_buf[dvi_ptr] = str_pool[k];
                    dvi_ptr++;
                    if (dvi_ptr == dvi_limit)
                        dvi_swap();
                }
                while (k++ < for_end);
        }
    }
}

void movement(scaled w, eight_bits o)
{
    memory_word *mem = zmem; small_number mstate;
    int32_t p, q;
    integer k;
    q = get_node(MOVEMENT_NODE_SIZE);
    mem[q + 1].cint = w;
    mem[q + 2].cint = dvi_offset + dvi_ptr;
    if (o == DOWN1) {
        mem[q].hh.v.RH = down_ptr;
        down_ptr = q;
    } else {

        mem[q].hh.v.RH = right_ptr;
        right_ptr = q;
    }
    p = mem[q].hh.v.RH;
    mstate = NONE_SEEN;
    while (p != MIN_HALFWORD) {

        if (mem[p + 1].cint == w)       /*632: */
            switch (mstate + mem[p].hh.v.LH) {
            case 3:
            case 4:
            case 15:
            case 16:
                if (mem[p + 2].cint < dvi_gone)
                    goto lab45;
                else {          /*633: */

                    k = mem[p + 2].cint - dvi_offset;
                    if (k < 0)
                        k = k + dvi_buf_size;
                    dvi_buf[k] = dvi_buf[k] + 5;
                    mem[p].hh.v.LH = Y_HERE;
                    goto lab40;
                }
                break;
            case 5:
            case 9:
            case 11:
                if (mem[p + 2].cint < dvi_gone)
                    goto lab45;
                else {          /*634: */

                    k = mem[p + 2].cint - dvi_offset;
                    if (k < 0)
                        k = k + dvi_buf_size;
                    dvi_buf[k] = dvi_buf[k] + 10;
                    mem[p].hh.v.LH = Z_HERE;
                    goto lab40;
                }
                break;
            case 1:
            case 2:
            case 8:
            case 13:
                goto lab40;
                break;
            default:
                ;
                break;
        } else
            switch (mstate + mem[p].hh.v.LH) {
            case 1:
                mstate = Y_SEEN;
                break;
            case 2:
                mstate = Z_SEEN;
                break;
            case 8:
            case 13:
                goto lab45;
                break;
            default:
                ;
                break;
            }
        p = mem[p].hh.v.RH;
    }
 lab45:                        /*not_found *//*:631 */ ;
    mem[q].hh.v.LH = YZ_OK;
    if (abs(w) >= 8388608L) {
        {
            dvi_buf[dvi_ptr] = o + 3;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        dvi_four(w);
        return;
    }
    if (abs(w) >= 32768L) {
        {
            dvi_buf[dvi_ptr] = o + 2;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        if (w < 0)
            w = w + 16777216L;
        {
            dvi_buf[dvi_ptr] = w / 65536L;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        w = w % 65536L;
        goto lab2;
    }
    if (abs(w) >= 128) {
        {
            dvi_buf[dvi_ptr] = o + 1;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        if (w < 0)
            w = w + 65536L;
        goto lab2;
    }
    {
        dvi_buf[dvi_ptr] = o;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    }
    if (w < 0)
        w = w + 256;
    goto lab1;
 lab2:{

        dvi_buf[dvi_ptr] = w / 256;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    }
 lab1:{

        dvi_buf[dvi_ptr] = w % 256;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    }
    return;
 lab40:                        /*found *//*629: */ mem[q].hh.v.LH = mem[p].hh.v.LH;
    if (mem[q].hh.v.LH == Y_HERE) {
        {
            dvi_buf[dvi_ptr] = o + 4;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        while (mem[q].hh.v.RH != p) {

            q = mem[q].hh.v.RH;
            switch (mem[q].hh.v.LH) {
            case 3:
                mem[q].hh.v.LH = Z_OK;
                break;
            case 4:
                mem[q].hh.v.LH = D_FIXED;
                break;
            default:
                ;
                break;
            }
        }
    } else {

        {
            dvi_buf[dvi_ptr] = o + 9;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        while (mem[q].hh.v.RH != p) {

            q = mem[q].hh.v.RH;
            switch (mem[q].hh.v.LH) {
            case 3:
                mem[q].hh.v.LH = Y_OK;
                break;
            case 5:
                mem[q].hh.v.LH = D_FIXED;
                break;
            default:
                ;
                break;
            }
        }
    }
}

void prune_movements(integer l)
{
    memory_word *mem = zmem; int32_t p;
    while (down_ptr != MIN_HALFWORD) {

        if (mem[down_ptr + 2].cint < l)
            goto done;
        p = down_ptr;
        down_ptr = mem[p].hh.v.RH;
        free_node(p, MOVEMENT_NODE_SIZE);
    }

done:
    while (right_ptr != MIN_HALFWORD) {

        if (mem[right_ptr + 2].cint < l)
            return;
        p = right_ptr;
        right_ptr = mem[p].hh.v.RH;
        free_node(p, MOVEMENT_NODE_SIZE);
    }
}

void special_out(int32_t p)
{
    memory_word *mem = zmem; unsigned char /*max_selector */ old_setting;
    pool_pointer k;
    if (cur_h != dvi_h) {
        movement(cur_h - dvi_h, RIGHT1);
        dvi_h = cur_h;
    }
    if (cur_v != dvi_v) {
        movement(cur_v - dvi_v, DOWN1);
        dvi_v = cur_v;
    }
    doing_special = true;
    old_setting = selector;
    selector = SELECTOR_NEW_STRING ;
    show_token_list(mem[mem[p + 1].hh.v.RH].hh.v.RH, MIN_HALFWORD, pool_size - pool_ptr);
    selector = old_setting;
    {
        if (pool_ptr + 1 > pool_size)
            overflow(S(pool_size), pool_size - init_pool_ptr);
    }
    if ((pool_ptr - str_start[(str_ptr) - 65536L]) < 256) {
        {
            dvi_buf[dvi_ptr] = XXX1;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = (pool_ptr - str_start[(str_ptr) - 65536L]);
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
    } else {

        {
            dvi_buf[dvi_ptr] = XXX4;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        dvi_four((pool_ptr - str_start[(str_ptr) - 65536L]));
    }
    {
        register integer for_end;
        k = str_start[(str_ptr) - 65536L];
        for_end = pool_ptr - 1;
        if (k <= for_end)
            do {
                dvi_buf[dvi_ptr] = str_pool[k];
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
            while (k++ < for_end);
    }
    pool_ptr = str_start[(str_ptr) - 65536L];
    doing_special = false;
}

void write_out(int32_t p)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    unsigned char /*max_selector */ old_setting;
    integer old_mode;
    small_number j;
    int32_t q, r;
    integer d;

    q = get_avail();
    mem[q].hh.v.LH = (RIGHT_BRACE_TOKEN + 125);
    r = get_avail();
    mem[q].hh.v.RH = r;
    mem[r].hh.v.LH = (CS_TOKEN_FLAG + 2243234);
    begin_token_list(q, INSERTED);
    begin_token_list(mem[p + 1].hh.v.RH, WRITE_TEXT);
    q = get_avail();
    mem[q].hh.v.LH = (LEFT_BRACE_TOKEN + 123);
    begin_token_list(q, INSERTED);
    old_mode = cur_list.mode;
    cur_list.mode = 0;
    cur_cs = write_loc;
    q = scan_toks(false, true);
    get_token();
    if (cur_tok != (CS_TOKEN_FLAG + 2243234)) {     /*1412: */
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Unbalanced_write_command));
        }
        {
            help_ptr = 2;
            help_line[1] = 66744L /*"On this page there's a \write with fewer real _'s than _'s." */ ;
            help_line[0] = S(I_can_t_handle_that_very_wel/*l; good luck.*/);
        }
        error();
        do {
            get_token();
        } while (!(cur_tok == (CS_TOKEN_FLAG + 2243234)));
    }
    cur_list.mode = old_mode;
    end_token_list();
    old_setting = selector;
    j = mem[p + 1].hh.v.LH;
    if (j == 18)
        selector = SELECTOR_NEW_STRING ;
    else if (write_open[j])
        selector = j;
    else {

        if ((j == 17) && (selector == SELECTOR_TERM_AND_LOG))
            selector = SELECTOR_LOG_ONLY;
        print_nl(S());
    }
    token_show(def_ref);
    print_ln();
    flush_list(def_ref);
    if (j == 18) {
        if ((INTPAR(tracing_online) <= 0))
            selector = SELECTOR_LOG_ONLY;
        else
            selector = SELECTOR_TERM_AND_LOG;
        if (!log_opened)
            selector = SELECTOR_TERM_ONLY;
        print_nl(S(runsystem_));
        {
            register integer for_end;
            d = 0;
            for_end = (pool_ptr - str_start[(str_ptr) - 65536L]) - 1;
            if (d <= for_end)
                do {
                    print(str_pool[str_start[(str_ptr) - 65536L] + d]);
                }
                while (d++ < for_end);
        }
        print(S(_____Z2/*")..."*/));
        print(S(disabled));
        print_char(46 /*"." */ );
        print_nl(S());
        print_ln();
        pool_ptr = str_start[(str_ptr) - 65536L];
    }
    selector = old_setting;
}

void pic_out(int32_t p)
{
    memory_word *mem = zmem; unsigned char /*max_selector */ old_setting;
    integer i;
    pool_pointer k;
    if (cur_h != dvi_h) {
        movement(cur_h - dvi_h, RIGHT1);
        dvi_h = cur_h;
    }
    if (cur_v != dvi_v) {
        movement(cur_v - dvi_v, DOWN1);
        dvi_v = cur_v;
    }
    old_setting = selector;
    selector = SELECTOR_NEW_STRING ;
    print(S(pdf_image_));
    print(S(matrix_));
    print_scaled(mem[p + 5].hh.v.LH);
    print(32 /*" " */ );
    print_scaled(mem[p + 5].hh.v.RH);
    print(32 /*" " */ );
    print_scaled(mem[p + 6].hh.v.LH);
    print(32 /*" " */ );
    print_scaled(mem[p + 6].hh.v.RH);
    print(32 /*" " */ );
    print_scaled(mem[p + 7].hh.v.LH);
    print(32 /*" " */ );
    print_scaled(mem[p + 7].hh.v.RH);
    print(32 /*" " */ );
    print(S(page_));
    print_int(mem[p + 4].hh.u.B1);
    print(32 /*" " */ );
    switch (mem[p + 8].hh.u.B0) {
    case 1:
        print(S(pagebox_cropbox_));
        break;
    case 2:
        print(S(pagebox_mediabox_));
        break;
    case 3:
        print(S(pagebox_bleedbox_));
        break;
    case 5:
        print(S(pagebox_artbox_));
        break;
    case 4:
        print(S(pagebox_trimbox_));
        break;
    default:
        ;
        break;
    }
    print(40 /*"(" */ );
    {
        register integer for_end;
        i = 0;
        for_end = mem[p + 4].hh.u.B0 - 1;
        if (i <= for_end)
            do
                print_raw_char(pic_path_byte(p, i), true);
            while (i++ < for_end);
    }
    print(41 /*")" */ );
    selector = old_setting;
    if ((pool_ptr - str_start[(str_ptr) - 65536L]) < 256) {
        {
            dvi_buf[dvi_ptr] = XXX1;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = (pool_ptr - str_start[(str_ptr) - 65536L]);
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
    } else {

        {
            dvi_buf[dvi_ptr] = XXX4;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        dvi_four((pool_ptr - str_start[(str_ptr) - 65536L]));
    }
    {
        register integer for_end;
        k = str_start[(str_ptr) - 65536L];
        for_end = pool_ptr - 1;
        if (k <= for_end)
            do {
                dvi_buf[dvi_ptr] = str_pool[k];
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
            while (k++ < for_end);
    }
    pool_ptr = str_start[(str_ptr) - 65536L];
}

void out_what(int32_t p)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    small_number j;
    unsigned char /*max_selector */ old_setting;

    switch (mem[p].hh.u.B1) {
    case 0:
    case 1:
    case 2:
        if (!doing_leaders) {
            j = mem[p + 1].hh.v.LH;
            if (mem[p].hh.u.B1 == WRITE_NODE)
                write_out(p);
            else {

                if (write_open[j])
                    ttstub_output_close(write_file[j]);
                if (mem[p].hh.u.B1 == CLOSE_NODE)
                    write_open[j] = false;
                else if (j < 16) {
                    cur_name = mem[p + 1].hh.v.RH;
                    cur_area = mem[p + 2].hh.v.LH;
                    cur_ext = mem[p + 2].hh.v.RH;
                    if (cur_ext == S())
                        cur_ext = S(_tex);
                    pack_file_name(cur_name, cur_area, cur_ext);
		    write_file[j] = ttstub_output_open (name_of_file + 1, 0);
		    if (write_file[j] == NULL)
			_tt_abort ("cannot open output file \"%s\"", name_of_file + 1);
                    write_open[j] = true;
                    if (log_opened) {
                        old_setting = selector;
                        if ((INTPAR(tracing_online) <= 0))
                            selector = SELECTOR_LOG_ONLY;
                        else
                            selector = SELECTOR_TERM_AND_LOG;
                        print_nl(S(_openout));
                        print_int(j);
                        print(S(_____Z3/*" = `"*/));
                        print_file_name(cur_name, cur_area, cur_ext);
                        print(S(___Z10/*"'."*/));
                        print_nl(S());
                        print_ln();
                        selector = old_setting;
                    }
                }
            }
        }
        break;
    case 3:
        special_out(p);
        break;
    case 4:
        ;
        break;
    default:
        confusion(S(ext4));
        break;
    }
}

int32_t new_edge(small_number s, scaled w)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(EDGE_NODE_SIZE);
    mem[p].hh.u.B0 = EDGE_NODE;
    mem[p].hh.u.B1 = s;
    mem[p + 1].cint = w;
    mem[p + 2].cint = 0;
    Result = p;
    return Result;
}

int32_t reverse(int32_t this_box, int32_t t, scaled * cur_g, double * cur_glue)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t l;
    int32_t p;
    int32_t q;
    glue_ord g_order;
    unsigned char /*shrinking */ g_sign;
    double glue_temp;
    int32_t m, n;
    g_order = mem[this_box + 5].hh.u.B1;
    g_sign = mem[this_box + 5].hh.u.B0;
    l = t;
    p = temp_ptr;
    m = MIN_HALFWORD;
    n = MIN_HALFWORD;
    while (true) {

        while (p != MIN_HALFWORD)        /*1511: */
 lab21:                        /*reswitch */ if ((p >= hi_mem_min))
                do {
                    f = mem[p].hh.u.B0;
                    c = mem[p].hh.u.B1;
                    cur_h =
                        cur_h + font_info[width_base[f] +
                                          font_info[char_base[f] + effective_char(true, f, c)].qqqq.u.B0].cint;
                    q = mem[p].hh.v.RH;
                    mem[p].hh.v.RH = l;
                    l = p;
                    p = q;
                } while (!(!(p >= hi_mem_min)));
            else {              /*1512: */

                q = mem[p].hh.v.RH;
                switch (mem[p].hh.u.B0) {
                case 0:
                case 1:
                case 2:
                case 11:
                    rule_wd = mem[p + 1].cint;
                    break;
                case 8:
                    if ((mem[p].hh.u.B1 == NATIVE_WORD_NODE) || (mem[p].hh.u.B1 == NATIVE_WORD_NODE_AT)
                        || (mem[p].hh.u.B1 == GLYPH_NODE) || (mem[p].hh.u.B1 == PIC_NODE)
                        || (mem[p].hh.u.B1 == PDF_NODE))
                        rule_wd = mem[p + 1].cint;
                    else
                        goto lab15;
                    break;
                case 10:
                    {
                        g = mem[p + 1].hh.v.LH;
                        rule_wd = mem[g + 1].cint - *cur_g;
                        if (g_sign != NORMAL) {
                            if (g_sign == STRETCHING) {
                                if (mem[g].hh.u.B0 == g_order) {
                                    *cur_glue = *cur_glue + mem[g + 2].cint;
                                    glue_temp = mem[this_box + 6].gr * *cur_glue;
                                    if (glue_temp > 1000000000.0)
                                        glue_temp = 1000000000.0;
                                    else if (glue_temp < -1000000000.0)
                                        glue_temp = -1000000000.0;
                                    *cur_g = tex_round(glue_temp);
                                }
                            } else if (mem[g].hh.u.B1 == g_order) {
                                *cur_glue = *cur_glue - mem[g + 3].cint;
                                glue_temp = mem[this_box + 6].gr * *cur_glue;
                                if (glue_temp > 1000000000.0)
                                    glue_temp = 1000000000.0;
                                else if (glue_temp < -1000000000.0)
                                    glue_temp = -1000000000.0;
                                *cur_g = tex_round(glue_temp);
                            }
                        }
                        rule_wd = rule_wd + *cur_g;
                        if ((((g_sign == STRETCHING) && (mem[g].hh.u.B0 == g_order))
                             || ((g_sign == SHRINKING) && (mem[g].hh.u.B1 == g_order)))) {
                            {
                                if (mem[g].hh.v.RH == MIN_HALFWORD)
                                    free_node(g, GLUE_SPEC_SIZE);
                                else
                                    mem[g].hh.v.RH--;
                            }
                            if (mem[p].hh.u.B1 < A_LEADERS) {
                                mem[p].hh.u.B0 = KERN_NODE;
                                mem[p + 1].cint = rule_wd;
                            } else {

                                g = get_node(GLUE_SPEC_SIZE);
                                mem[g].hh.u.B0 = (FILLL + 1);
                                mem[g].hh.u.B1 = (FILLL + 1);
                                mem[g + 1].cint = rule_wd;
                                mem[g + 2].cint = 0;
                                mem[g + 3].cint = 0;
                                mem[p + 1].hh.v.LH = g;
                            }
                        }
                    }
                    break;
                case 6:
                    {
                        flush_node_list(mem[p + 1].hh.v.RH);
                        temp_ptr = p;
                        p = get_avail();
                        mem[p] = mem[temp_ptr + 1];
                        mem[p].hh.v.RH = q;
                        free_node(temp_ptr, SMALL_NODE_SIZE);
                        goto lab21;
                    }
                    break;
                case 9:
                    {
                        rule_wd = mem[p + 1].cint;
                        if (odd(mem[p].hh.u.B1)) {

                            if (mem[LR_ptr].hh.v.LH != (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3)) {
                                mem[p].hh.u.B0 = KERN_NODE;
                                LR_problems++;
                            } else {

                                {
                                    temp_ptr = LR_ptr;
                                    LR_ptr = mem[temp_ptr].hh.v.RH;
                                    {
                                        mem[temp_ptr].hh.v.RH = avail;
                                        avail = temp_ptr;
                                    }
                                }
                                if (n > MIN_HALFWORD) {
                                    n--;
                                    mem[p].hh.u.B1--;
                                } else {

                                    mem[p].hh.u.B0 = KERN_NODE;
                                    if (m > MIN_HALFWORD)
                                        m--;
                                    else {      /*1517: */

                                        free_node(p, MEDIUM_NODE_SIZE);
                                        mem[t].hh.v.RH = q;
                                        mem[t + 1].cint = rule_wd;
                                        mem[t + 2].cint = -(integer) cur_h - rule_wd;
                                        goto done;
                                    }
                                }
                            }
                        } else {

                            {
                                temp_ptr = get_avail();
                                mem[temp_ptr].hh.v.LH = (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3);
                                mem[temp_ptr].hh.v.RH = LR_ptr;
                                LR_ptr = temp_ptr;
                            }
                            if ((n > MIN_HALFWORD) || ((mem[p].hh.u.B1 / R_CODE) != cur_dir)) {
                                n++;
                                mem[p].hh.u.B1++;
                            } else {

                                mem[p].hh.u.B0 = KERN_NODE;
                                m++;
                            }
                        }
                    }
                    break;
                case 14:
                    confusion(S(LR2));
                    break;
                default:
                    goto lab15;
                    break;
                }
                cur_h = cur_h + rule_wd;
 lab15:                        /*next_p */ mem[p].hh.v.RH = l;
                if (mem[p].hh.u.B0 == KERN_NODE) {

                    if ((rule_wd == 0) || (l == MIN_HALFWORD)) {
                        free_node(p, MEDIUM_NODE_SIZE);
                        p = l;
                    }
                }
                l = p;
                p = q;
            }
        if ((t == MIN_HALFWORD) && (m == MIN_HALFWORD) && (n == MIN_HALFWORD))
            goto done;
        p = new_math(0, mem[LR_ptr].hh.v.LH);
        LR_problems = LR_problems + 10000;
    }

done:
    Result = l;
    return Result;
}

void hlist_out(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    scaled base_line;
    scaled left_edge;
    scaled save_h, save_v;
    int32_t this_box;
    glue_ord g_order;
    unsigned char /*shrinking */ g_sign;
    int32_t p;
    integer save_loc;
    int32_t leader_box;
    scaled leader_wd;
    scaled lx;
    boolean outer_doing_leaders;
    scaled edge;
    int32_t prev_p;
    integer len;
    int32_t q, r;
    integer k, j;
    double glue_temp;
    double cur_glue;
    scaled cur_g;

    cur_g = 0;
    cur_glue = 0.0;
    this_box = temp_ptr;
    g_order = mem[this_box + 5].hh.u.B1;
    g_sign = mem[this_box + 5].hh.u.B0;

    if (STATEINT(xetex_interword_space_shaping) > 1) {
        p = mem[this_box + 5].hh.v.RH;
        prev_p = this_box + 5;
        while (p != MIN_HALFWORD) {

            if (mem[p].hh.v.RH != MIN_HALFWORD) {
                if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
                      && ((mem[p].hh.u.B1 == NATIVE_WORD_NODE)
                          || (mem[p].hh.u.B1 == NATIVE_WORD_NODE_AT))))
                    && (font_letter_space[mem[p + 4].qqqq.u.B1] == 0)) {
                    r = p;
                    k = mem[r + 4].qqqq.u.B2;
                    q = mem[p].hh.v.RH;
 lab1236:          /*check_next *//*641: */ while ((q != MIN_HALFWORD) && !(q >= hi_mem_min)
                                                    && ((mem[q].hh.u.B0 == PENALTY_NODE)
                                                        || (mem[q].hh.u.B0 == INS_NODE)
                                                        || (mem[q].hh.u.B0 == MARK_NODE)
                                                        || (mem[q].hh.u.B0 == ADJUST_NODE)
                                                        || ((mem[q].hh.u.B0 == WHATSIT_NODE)
                                                            && (mem[q].hh.u.B1 <= 4))))
                        q = mem[q].hh.v.RH /*:641 */ ;
                    if ((q != MIN_HALFWORD) && !(q >= hi_mem_min)) {
                        if ((mem[q].hh.u.B0 == GLUE_NODE) && (mem[q].hh.u.B1 == NORMAL)) {
                            if ((mem[q + 1].hh.v.LH == font_glue[mem[r + 4].qqqq.u.B1])) {
                                q = mem[q].hh.v.RH;
                                while ((q != MIN_HALFWORD) && !(q >= hi_mem_min)
                                       && ((mem[q].hh.u.B0 == PENALTY_NODE) || (mem[q].hh.u.B0 == INS_NODE)
                                           || (mem[q].hh.u.B0 == MARK_NODE)
                                           || (mem[q].hh.u.B0 == ADJUST_NODE)
                                           || ((mem[q].hh.u.B0 == WHATSIT_NODE) && (mem[q].hh.u.B1 <= 4))))
                                    q = mem[q].hh.v.RH /*:641 */ ;
                                if ((((q) != MIN_HALFWORD && (!(q >= hi_mem_min))
                                      && (mem[q].hh.u.B0 == WHATSIT_NODE)
                                      && ((mem[q].hh.u.B1 == NATIVE_WORD_NODE)
                                          || (mem[q].hh.u.B1 == NATIVE_WORD_NODE_AT))))
                                    && (mem[q + 4].qqqq.u.B1 == mem[r + 4].qqqq.u.B1)) {
                                    p = q;
                                    k = k + 1 + mem[q + 4].qqqq.u.B2;
                                    q = mem[q].hh.v.RH;
                                    goto lab1236;
                                }
                            } else
                                q = mem[q].hh.v.RH;
                            if ((q != MIN_HALFWORD) && !(q >= hi_mem_min) && (mem[q].hh.u.B0 == KERN_NODE)
                                && (mem[q].hh.u.B1 == SPACE_ADJUSTMENT)) {
                                q = mem[q].hh.v.RH;
                                while ((q != MIN_HALFWORD) && !(q >= hi_mem_min)
                                       && ((mem[q].hh.u.B0 == PENALTY_NODE) || (mem[q].hh.u.B0 == INS_NODE)
                                           || (mem[q].hh.u.B0 == MARK_NODE)
                                           || (mem[q].hh.u.B0 == ADJUST_NODE)
                                           || ((mem[q].hh.u.B0 == WHATSIT_NODE) && (mem[q].hh.u.B1 <= 4))))
                                    q = mem[q].hh.v.RH /*:641 */ ;
                                if ((((q) != MIN_HALFWORD && (!(q >= hi_mem_min))
                                      && (mem[q].hh.u.B0 == WHATSIT_NODE)
                                      && ((mem[q].hh.u.B1 == NATIVE_WORD_NODE)
                                          || (mem[q].hh.u.B1 == NATIVE_WORD_NODE_AT))))
                                    && (mem[q + 4].qqqq.u.B1 == mem[r + 4].qqqq.u.B1)) {
                                    p = q;
                                    k = k + 1 + mem[q + 4].qqqq.u.B2;
                                    q = mem[q].hh.v.RH;
                                    goto lab1236;
                                }
                            }
                            goto lab1237;
                        }
                        if ((((q) != MIN_HALFWORD && (!(q >= hi_mem_min)) && (mem[q].hh.u.B0 == WHATSIT_NODE)
                              && ((mem[q].hh.u.B1 == NATIVE_WORD_NODE)
                                  || (mem[q].hh.u.B1 == NATIVE_WORD_NODE_AT))))
                            && (mem[q + 4].qqqq.u.B1 == mem[r + 4].qqqq.u.B1)) {
                            p = q;
                            q = mem[q].hh.v.RH;
                            goto lab1236;
                        }
                    }
 lab1237:                      /*end_node_run */ if (p != r) {
                        {
                            if (pool_ptr + k > pool_size)
                                overflow(S(pool_size), pool_size - init_pool_ptr);
                        }
                        k = 0;
                        q = r;
                        while (true) {

                            if (mem[q].hh.u.B0 == WHATSIT_NODE) {
                                if ((mem[q].hh.u.B1 == NATIVE_WORD_NODE)
                                    || (mem[q].hh.u.B1 == NATIVE_WORD_NODE_AT)) {
                                    {
                                        register integer for_end;
                                        j = 0;
                                        for_end = mem[q + 4].qqqq.u.B2 - 1;
                                        if (j <= for_end)
                                            do {
                                                str_pool[pool_ptr] = get_native_char(q, j);
                                                pool_ptr++;
                                            }
                                            while (j++ < for_end);
                                    }
                                    k = k + mem[q + 1].cint;
                                }
                            } else if (mem[q].hh.u.B0 == GLUE_NODE) {
                                {
                                    str_pool[pool_ptr] = 32 /*" " */ ;
                                    pool_ptr++;
                                }
                                g = mem[q + 1].hh.v.LH;
                                k = k + mem[g + 1].cint;
                                if (g_sign != NORMAL) {
                                    if (g_sign == STRETCHING) {
                                        if (mem[g].hh.u.B0 == g_order) {
                                            k = k + tex_round(mem[this_box + 6].gr * mem[g + 2].cint);
                                        }
                                    } else {

                                        if (mem[g].hh.u.B1 == g_order) {
                                            k = k - tex_round(mem[this_box + 6].gr * mem[g + 3].cint);
                                        }
                                    }
                                }
                            } else if (mem[q].hh.u.B0 == KERN_NODE) {
                                k = k + mem[q + 1].cint;
                            }
                            if (q == p)
                                break;
                            else
                                q = mem[q].hh.v.RH;
                        }
                        q = new_native_word_node(mem[r + 4].qqqq.u.B1, (pool_ptr - str_start[(str_ptr) - 65536L]));
                        mem[q].hh.u.B1 = mem[r].hh.u.B1;
                        {
                            register integer for_end;
                            j = 0;
                            for_end = (pool_ptr - str_start[(str_ptr) - 65536L]) - 1;
                            if (j <= for_end)
                                do
                                    set_native_char(q, j, str_pool[str_start[(str_ptr) - 65536L] + j]);
                                while (j++ < for_end);
                        }
                        mem[q + 1].cint = k;
                        set_justified_native_glyphs(q);
                        mem[prev_p].hh.v.RH = q;
                        mem[q].hh.v.RH = mem[p].hh.v.RH;
                        mem[p].hh.v.RH = MIN_HALFWORD;
                        prev_p = r;
                        p = mem[r].hh.v.RH;
                        while (p != MIN_HALFWORD) {

                            if (!(p >= hi_mem_min)
                                && ((mem[p].hh.u.B0 == PENALTY_NODE) || (mem[p].hh.u.B0 == INS_NODE)
                                    || (mem[p].hh.u.B0 == MARK_NODE) || (mem[p].hh.u.B0 == ADJUST_NODE)
                                    || ((mem[p].hh.u.B0 == WHATSIT_NODE) && (mem[p].hh.u.B1 <= 4)))) {
                                mem[prev_p].hh.v.RH = mem[p].hh.v.RH;
                                mem[p].hh.v.RH = mem[q].hh.v.RH;
                                mem[q].hh.v.RH = p;
                                q = p;
                            }
                            prev_p = p;
                            p = mem[p].hh.v.RH;
                        }
                        flush_node_list(r);
                        pool_ptr = str_start[(str_ptr) - 65536L];
                        p = q;
                    }
                }
                prev_p = p;
            }
            p = mem[p].hh.v.RH;
        }
    }
    p = mem[this_box + 5].hh.v.RH;
    cur_s++;
    if (cur_s > 0) {
        dvi_buf[dvi_ptr] = PUSH;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    }
    if (cur_s > max_push)
        max_push = cur_s;
    save_loc = dvi_offset + dvi_ptr;
    base_line = cur_v;
    prev_p = this_box + 5;

    temp_ptr = get_avail();
    mem[temp_ptr].hh.v.LH = BEFORE;
    mem[temp_ptr].hh.v.RH = LR_ptr;
    LR_ptr = temp_ptr;

    if (mem[this_box].hh.u.B1 == DLIST) {
        if (cur_dir == RIGHT_TO_LEFT) {
            cur_dir = LEFT_TO_RIGHT;
            cur_h = cur_h - mem[this_box + 1].cint;
        } else {
            mem[this_box].hh.u.B1 = 0;
        }
    }

    if (cur_dir == RIGHT_TO_LEFT && mem[this_box].hh.u.B1 != REVERSED) {
        /*1508: */
        save_h = cur_h;
        temp_ptr = p;
        p = new_kern(0);
        mem[p + 2].hh.v.LH = 0;
        mem[prev_p].hh.v.RH = p;
        cur_h = 0;
        mem[p].hh.v.RH = reverse(this_box, MIN_HALFWORD, &cur_g, &cur_glue);
        mem[p + 1].cint = -(integer) cur_h;
        cur_h = save_h;
        mem[this_box].hh.u.B1 = REVERSED;
    }

    left_edge = cur_h;
    synctex_hlist(this_box);
    while (p != MIN_HALFWORD)    /*642: */
 lab21:                        /*reswitch */ if ((p >= hi_mem_min)) {
            if (cur_h != dvi_h) {
                movement(cur_h - dvi_h, RIGHT1);
                dvi_h = cur_h;
            }
            if (cur_v != dvi_v) {
                movement(cur_v - dvi_v, DOWN1);
                dvi_v = cur_v;
            }
            do {
                f = mem[p].hh.u.B0;
                c = mem[p].hh.u.B1;
                if ((p != mem_top - 12) && (font_mapping[f] != NULL))
                    c = apply_tfm_font_mapping(font_mapping[f], c);
                if (f != dvi_f) {       /*643: */
                    if (!font_used[f]) {
                        dvi_font_def(f);
                        font_used[f] = true;
                    }
                    if (f <= 64) {
                        dvi_buf[dvi_ptr] = f + 170;
                        dvi_ptr++;
                        if (dvi_ptr == dvi_limit)
                            dvi_swap();
                    } else if (f <= 256) {
                        {
                            dvi_buf[dvi_ptr] = FNT1;
                            dvi_ptr++;
                            if (dvi_ptr == dvi_limit)
                                dvi_swap();
                        }
                        {
                            dvi_buf[dvi_ptr] = f - 1;
                            dvi_ptr++;
                            if (dvi_ptr == dvi_limit)
                                dvi_swap();
                        }
                    } else {

                        {
                            dvi_buf[dvi_ptr] = (FNT1 + 1);
                            dvi_ptr++;
                            if (dvi_ptr == dvi_limit)
                                dvi_swap();
                        }
                        {
                            dvi_buf[dvi_ptr] = (f - 1) / 256;
                            dvi_ptr++;
                            if (dvi_ptr == dvi_limit)
                                dvi_swap();
                        }
                        {
                            dvi_buf[dvi_ptr] = (f - 1) % 256;
                            dvi_ptr++;
                            if (dvi_ptr == dvi_limit)
                                dvi_swap();
                        }
                    }
                    dvi_f = f;
                }
                if (font_ec[f] >= c) {

                    if (font_bc[f] <= c) {

                        if ((font_info[char_base[f] + c].qqqq.u.B0 > 0)) {
                            if (c >= 128) {
                                dvi_buf[dvi_ptr] = SET1;
                                dvi_ptr++;
                                if (dvi_ptr == dvi_limit)
                                    dvi_swap();
                            }
                            {
                                dvi_buf[dvi_ptr] = c;
                                dvi_ptr++;
                                if (dvi_ptr == dvi_limit)
                                    dvi_swap();
                            }
                            cur_h = cur_h + font_info[width_base[f] + font_info[char_base[f] + c].qqqq.u.B0].cint;
                            goto lab22;
                        }
                    }
                }

            lab22: /*continue */
                prev_p = mem[prev_p].hh.v.RH;
                p = mem[p].hh.v.RH;
            } while (!(!(p >= hi_mem_min)));
            synctex_current();
            dvi_h = cur_h;
        } else {                /*644: */

            switch (mem[p].hh.u.B0) {
            case 0:
            case 1:
                if (mem[p + 5].hh.v.RH == MIN_HALFWORD) {
                    if (mem[p].hh.u.B0 == VLIST_NODE) {
                        synctex_void_vlist(p, this_box);
                    } else {

                        synctex_void_hlist(p, this_box);
                    }
                    cur_h = cur_h + mem[p + 1].cint;
                } else {

                    save_h = dvi_h;
                    save_v = dvi_v;
                    cur_v = base_line + mem[p + 4].cint;
                    temp_ptr = p;
                    edge = cur_h + mem[p + 1].cint;
                    if (cur_dir == RIGHT_TO_LEFT)
                        cur_h = edge;
                    if (mem[p].hh.u.B0 == VLIST_NODE)
                        vlist_out();
                    else
                        hlist_out();
                    dvi_h = save_h;
                    dvi_v = save_v;
                    cur_h = edge;
                    cur_v = base_line;
                }
                break;
            case 2:
                {
                    rule_ht = mem[p + 3].cint;
                    rule_dp = mem[p + 2].cint;
                    rule_wd = mem[p + 1].cint;
                    goto lab14;
                }
                break;
            case 8:
                {
                    switch (mem[p].hh.u.B1) {
                    case 40:
                    case 41:
                    case 42:
                        {
                            if (cur_h != dvi_h) {
                                movement(cur_h - dvi_h, RIGHT1);
                                dvi_h = cur_h;
                            }
                            if (cur_v != dvi_v) {
                                movement(cur_v - dvi_v, DOWN1);
                                dvi_v = cur_v;
                            }
                            f = mem[p + 4].qqqq.u.B1;
                            if (f != dvi_f) {   /*643: */
                                if (!font_used[f]) {
                                    dvi_font_def(f);
                                    font_used[f] = true;
                                }
                                if (f <= 64) {
                                    dvi_buf[dvi_ptr] = f + 170;
                                    dvi_ptr++;
                                    if (dvi_ptr == dvi_limit)
                                        dvi_swap();
                                } else if (f <= 256) {
                                    {
                                        dvi_buf[dvi_ptr] = FNT1;
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                    {
                                        dvi_buf[dvi_ptr] = f - 1;
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                } else {

                                    {
                                        dvi_buf[dvi_ptr] = (FNT1 + 1);
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                    {
                                        dvi_buf[dvi_ptr] = (f - 1) / 256;
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                    {
                                        dvi_buf[dvi_ptr] = (f - 1) % 256;
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                }
                                dvi_f = f;
                            }
                            if (mem[p].hh.u.B1 == GLYPH_NODE) {
                                {
                                    dvi_buf[dvi_ptr] = SET_GLYPHS;
                                    dvi_ptr++;
                                    if (dvi_ptr == dvi_limit)
                                        dvi_swap();
                                }
                                dvi_four(mem[p + 1].cint);
                                dvi_two(1);
                                dvi_four(0);
                                dvi_four(0);
                                dvi_two(mem[p + 4].qqqq.u.B2);
                                cur_h = cur_h + mem[p + 1].cint;
                            } else {

                                if (mem[p].hh.u.B1 == NATIVE_WORD_NODE_AT) {
                                    if ((mem[p + 4].qqqq.u.B2 > 0) || (mem[p + 5].ptr != NULL)) {
                                        {
                                            dvi_buf[dvi_ptr] = SET_TEXT_AND_GLYPHS;
                                            dvi_ptr++;
                                            if (dvi_ptr == dvi_limit)
                                                dvi_swap();
                                        }
                                        len = mem[p + 4].qqqq.u.B2;
                                        dvi_two(len);
                                        {
                                            register integer for_end;
                                            k = 0;
                                            for_end = len - 1;
                                            if (k <= for_end)
                                                do {
                                                    dvi_two(get_native_char(p, k));
                                                }
                                                while (k++ < for_end);
                                        }
                                        len = make_xdv_glyph_array_data(p);
                                        {
                                            register integer for_end;
                                            k = 0;
                                            for_end = len - 1;
                                            if (k <= for_end)
                                                do {
                                                    dvi_buf[dvi_ptr] = xdv_buffer[k];
                                                    dvi_ptr++;
                                                    if (dvi_ptr == dvi_limit)
                                                        dvi_swap();
                                                }
                                                while (k++ < for_end);
                                        }
                                    }
                                } else {

                                    if (mem[p + 5].ptr != NULL) {
                                        {
                                            dvi_buf[dvi_ptr] = SET_GLYPHS;
                                            dvi_ptr++;
                                            if (dvi_ptr == dvi_limit)
                                                dvi_swap();
                                        }
                                        len = make_xdv_glyph_array_data(p);
                                        {
                                            register integer for_end;
                                            k = 0;
                                            for_end = len - 1;
                                            if (k <= for_end)
                                                do {
                                                    dvi_buf[dvi_ptr] = xdv_buffer[k];
                                                    dvi_ptr++;
                                                    if (dvi_ptr == dvi_limit)
                                                        dvi_swap();
                                                }
                                                while (k++ < for_end);
                                        }
                                    }
                                }
                                cur_h = cur_h + mem[p + 1].cint;
                            }
                            dvi_h = cur_h;
                        }
                        break;
                    case 43:
                    case 44:
                        {
                            save_h = dvi_h;
                            save_v = dvi_v;
                            cur_v = base_line;
                            edge = cur_h + mem[p + 1].cint;
                            pic_out(p);
                            dvi_h = save_h;
                            dvi_v = save_v;
                            cur_h = edge;
                            cur_v = base_line;
                        }
                        break;
                    case 6:
                        {
                            pdf_last_x_pos = cur_h + cur_h_offset;
                            pdf_last_y_pos = cur_page_height - cur_v - cur_v_offset;
                        }
                        break;
                    default:
                        out_what(p);
                        break;
                    }
                }
                break;
            case 10:
                {
                    g = mem[p + 1].hh.v.LH;
                    rule_wd = mem[g + 1].cint - cur_g;
                    if (g_sign != NORMAL) {
                        if (g_sign == STRETCHING) {
                            if (mem[g].hh.u.B0 == g_order) {
                                cur_glue = cur_glue + mem[g + 2].cint;
                                glue_temp = mem[this_box + 6].gr * cur_glue;
                                if (glue_temp > 1000000000.0)
                                    glue_temp = 1000000000.0;
                                else if (glue_temp < -1000000000.0)
                                    glue_temp = -1000000000.0;
                                cur_g = tex_round(glue_temp);
                            }
                        } else if (mem[g].hh.u.B1 == g_order) {
                            cur_glue = cur_glue - mem[g + 3].cint;
                            glue_temp = mem[this_box + 6].gr * cur_glue;
                            if (glue_temp > 1000000000.0)
                                glue_temp = 1000000000.0;
                            else if (glue_temp < -1000000000.0)
                                glue_temp = -1000000000.0;
                            cur_g = tex_round(glue_temp);
                        }
                    }
                    rule_wd = rule_wd + cur_g;

/*1486: */
                    if ((g_sign == STRETCHING && mem[g].hh.u.B0 == g_order) ||
                        (g_sign == SHRINKING && mem[g].hh.u.B1 == g_order)) {
                        if (mem[g].hh.v.RH == MIN_HALFWORD)
                            free_node(g, GLUE_SPEC_SIZE);
                        else
                            mem[g].hh.v.RH--;

                        if (mem[p].hh.u.B1 < A_LEADERS) {
                            mem[p].hh.u.B0 = KERN_NODE;
                            mem[p + 1].cint = rule_wd;
                        } else {
                            g = get_node(GLUE_SPEC_SIZE);
                            mem[g].hh.u.B0 = (FILLL + 1);
                            mem[g].hh.u.B1 = (FILLL + 1);
                            mem[g + 1].cint = rule_wd;
                            mem[g + 2].cint = 0;
                            mem[g + 3].cint = 0;
                            mem[p + 1].hh.v.LH = g;
                        }
                    }

                    if (mem[p].hh.u.B1 >= A_LEADERS) {  /*648: */
                        leader_box = mem[p + 1].hh.v.RH;
                        if (mem[leader_box].hh.u.B0 == RULE_NODE) {
                            rule_ht = mem[leader_box + 3].cint;
                            rule_dp = mem[leader_box + 2].cint;
                            goto lab14;
                        }
                        leader_wd = mem[leader_box + 1].cint;
                        if ((leader_wd > 0) && (rule_wd > 0)) {
                            rule_wd = rule_wd + 10;
                            if (cur_dir == RIGHT_TO_LEFT)
                                cur_h = cur_h - 10;
                            edge = cur_h + rule_wd;
                            lx = 0;
                            if (mem[p].hh.u.B1 == A_LEADERS) {
                                save_h = cur_h;
                                cur_h = left_edge + leader_wd * ((cur_h - left_edge) / leader_wd);
                                if (cur_h < save_h)
                                    cur_h = cur_h + leader_wd;
                            } else {

                                lq = rule_wd / leader_wd;
                                lr = rule_wd % leader_wd;
                                if (mem[p].hh.u.B1 == C_LEADERS)
                                    cur_h = cur_h + (lr / 2);
                                else {

                                    lx = lr / (lq + 1);
                                    cur_h = cur_h + ((lr - (lq - 1) * lx) / 2);
                                }
                            }
                            while (cur_h + leader_wd <= edge) { /*650: */

                                cur_v = base_line + mem[leader_box + 4].cint;
                                if (cur_v != dvi_v) {
                                    movement(cur_v - dvi_v, DOWN1);
                                    dvi_v = cur_v;
                                }
                                save_v = dvi_v;
                                if (cur_h != dvi_h) {
                                    movement(cur_h - dvi_h, RIGHT1);
                                    dvi_h = cur_h;
                                }
                                save_h = dvi_h;
                                temp_ptr = leader_box;
                                if (cur_dir == RIGHT_TO_LEFT)
                                    cur_h = cur_h + leader_wd;
                                outer_doing_leaders = doing_leaders;
                                doing_leaders = true;
                                if (mem[leader_box].hh.u.B0 == VLIST_NODE)
                                    vlist_out();
                                else
                                    hlist_out();
                                doing_leaders = outer_doing_leaders;
                                dvi_v = save_v;
                                dvi_h = save_h;
                                cur_v = base_line;
                                cur_h = save_h + leader_wd + lx;
                            }
                            if (cur_dir == RIGHT_TO_LEFT)
                                cur_h = edge;
                            else
                                cur_h = edge - 10;
                            goto lab15;
                        }
                    }
                    goto lab13;
                }
                break;
            case 40:
                {
                    cur_h = cur_h + mem[p + 1].cint;
                }
                break;
            case 11:
                {
                    synctex_kern(p, this_box);
                    cur_h = cur_h + mem[p + 1].cint;
                }
                break;
            case 9:
                synctex_math(p, this_box);
/*1504: */
                if (odd(mem[p].hh.u.B1)) {
                    if (mem[LR_ptr].hh.v.LH == (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3)) {
                        temp_ptr = LR_ptr;
                        LR_ptr = mem[temp_ptr].hh.v.RH;
                        mem[temp_ptr].hh.v.RH = avail;
                        avail = temp_ptr;
                    } else {
                        if (mem[p].hh.u.B1 > L_CODE)
                            LR_problems++;
                    }
                } else {
                    temp_ptr = get_avail();
                    mem[temp_ptr].hh.v.LH = (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3);
                    mem[temp_ptr].hh.v.RH = LR_ptr;
                    LR_ptr = temp_ptr;

                    if ((mem[p].hh.u.B1 / R_CODE) != cur_dir) {
                        /*1509: */
                        save_h = cur_h;
                        temp_ptr = mem[p].hh.v.RH;
                        rule_wd = mem[p + 1].cint;
                        free_node(p, MEDIUM_NODE_SIZE);
                        cur_dir = 1 - cur_dir;
                        p = new_edge(cur_dir, rule_wd);
                        mem[prev_p].hh.v.RH = p;
                        cur_h = cur_h - left_edge + rule_wd;
                        mem[p].hh.v.RH = reverse(this_box, new_edge(1 - cur_dir, 0), &cur_g, &cur_glue);
                        mem[p + 2].cint = cur_h;
                        cur_dir = 1 - cur_dir;
                        cur_h = save_h;
                        goto lab21;
                    }
                }

                mem[p].hh.u.B0 = KERN_NODE;
                cur_h = cur_h + mem[p + 1].cint;
                break;
            case 6:
                {
                    mem[mem_top - 12] = mem[p + 1];
                    mem[mem_top - 12].hh.v.RH = mem[p].hh.v.RH;
                    p = mem_top - 12;
                    xtx_ligature_present = true;
                    goto lab21;
                }
                break;
            case 14:
                {
                    cur_h = cur_h + mem[p + 1].cint;
                    left_edge = cur_h + mem[p + 2].cint;
                    cur_dir = mem[p].hh.u.B1;
                }
                break;
            default:
                ;
                break;
            }
            goto lab15;
 lab14:                        /*fin_rule *//*646: */ if ((rule_ht == NULL_FLAG))
                rule_ht = mem[this_box + 3].cint;
            if ((rule_dp == NULL_FLAG))
                rule_dp = mem[this_box + 2].cint;
            rule_ht = rule_ht + rule_dp;
            if ((rule_ht > 0) && (rule_wd > 0)) {
                if (cur_h != dvi_h) {
                    movement(cur_h - dvi_h, RIGHT1);
                    dvi_h = cur_h;
                }
                cur_v = base_line + rule_dp;
                if (cur_v != dvi_v) {
                    movement(cur_v - dvi_v, DOWN1);
                    dvi_v = cur_v;
                }
                {
                    dvi_buf[dvi_ptr] = SET_RULE;
                    dvi_ptr++;
                    if (dvi_ptr == dvi_limit)
                        dvi_swap();
                }
                dvi_four(rule_ht);
                dvi_four(rule_wd);
                cur_v = base_line;
                dvi_h = dvi_h + rule_wd;
            }
 lab13:                        /*move_past */  {

                cur_h = cur_h + rule_wd;
                synctex_horizontal_rule_or_glue(p, this_box);
            }
 lab15:                        /*next_p */ prev_p = p;
            p = mem[p].hh.v.RH;
        }
    synctex_tsilh(this_box);

    while (mem[LR_ptr].hh.v.LH != BEFORE) {
        if (mem[LR_ptr].hh.v.LH > L_CODE)
            LR_problems = LR_problems + 10000;

        temp_ptr = LR_ptr;
        LR_ptr = mem[temp_ptr].hh.v.RH;
        mem[temp_ptr].hh.v.RH = avail;
        avail = temp_ptr;
    }

    temp_ptr = LR_ptr;
    LR_ptr = mem[temp_ptr].hh.v.RH;
    mem[temp_ptr].hh.v.RH = avail;
    avail = temp_ptr;

    if ((mem[this_box].hh.u.B1) == DLIST)
        cur_dir = RIGHT_TO_LEFT;

    prune_movements(save_loc);
    if (cur_s > 0)
        dvi_pop(save_loc);
    cur_s--;
}

void vlist_out(void)
{
    memory_word *mem = zmem; scaled left_edge;
    scaled top_edge;
    scaled save_h, save_v;
    int32_t this_box;
    glue_ord g_order;
    unsigned char /*shrinking */ g_sign;
    int32_t p;
    integer save_loc;
    int32_t leader_box;
    scaled leader_ht;
    scaled lx;
    boolean outer_doing_leaders;
    scaled edge;
    double glue_temp;
    double cur_glue;
    scaled cur_g;
    boolean upwards;
    cur_g = 0;
    cur_glue = 0.0;
    this_box = temp_ptr;
    g_order = mem[this_box + 5].hh.u.B1;
    g_sign = mem[this_box + 5].hh.u.B0;
    p = mem[this_box + 5].hh.v.RH;
    upwards = (mem[this_box].hh.u.B1 == 1);
    cur_s++;
    if (cur_s > 0) {
        dvi_buf[dvi_ptr] = PUSH;
        dvi_ptr++;
        if (dvi_ptr == dvi_limit)
            dvi_swap();
    }
    if (cur_s > max_push)
        max_push = cur_s;
    save_loc = dvi_offset + dvi_ptr;
    left_edge = cur_h;
    synctex_vlist(this_box);
    if (upwards)
        cur_v = cur_v + mem[this_box + 2].cint;
    else
        cur_v = cur_v - mem[this_box + 3].cint;
    top_edge = cur_v;
    while (p != MIN_HALFWORD) {  /*652: */

        if ((p >= hi_mem_min))
            confusion(S(vlistout));
        else {                  /*653: */

            switch (mem[p].hh.u.B0) {
            case 0:
            case 1:
                if (mem[p + 5].hh.v.RH == MIN_HALFWORD) {
                    if (upwards)
                        cur_v = cur_v - mem[p + 2].cint;
                    else
                        cur_v = cur_v + mem[p + 3].cint;
                    if (mem[p].hh.u.B0 == VLIST_NODE) {
                        synctex_void_vlist(p, this_box);
                    } else {

                        synctex_void_hlist(p, this_box);
                    }
                    if (upwards)
                        cur_v = cur_v - mem[p + 3].cint;
                    else
                        cur_v = cur_v + mem[p + 2].cint;
                } else {

                    if (upwards)
                        cur_v = cur_v - mem[p + 2].cint;
                    else
                        cur_v = cur_v + mem[p + 3].cint;
                    if (cur_v != dvi_v) {
                        movement(cur_v - dvi_v, DOWN1);
                        dvi_v = cur_v;
                    }
                    save_h = dvi_h;
                    save_v = dvi_v;
                    if (cur_dir == RIGHT_TO_LEFT)
                        cur_h = left_edge - mem[p + 4].cint;
                    else
                        cur_h = left_edge + mem[p + 4].cint;
                    temp_ptr = p;
                    if (mem[p].hh.u.B0 == VLIST_NODE)
                        vlist_out();
                    else
                        hlist_out();
                    dvi_h = save_h;
                    dvi_v = save_v;
                    if (upwards)
                        cur_v = save_v - mem[p + 3].cint;
                    else
                        cur_v = save_v + mem[p + 2].cint;
                    cur_h = left_edge;
                }
                break;
            case 2:
                {
                    rule_ht = mem[p + 3].cint;
                    rule_dp = mem[p + 2].cint;
                    rule_wd = mem[p + 1].cint;
                    goto lab14;
                }
                break;
            case 8:
                {
                    switch (mem[p].hh.u.B1) {
                    case 42:
                        {
                            cur_v = cur_v + mem[p + 3].cint;
                            cur_h = left_edge;
                            if (cur_h != dvi_h) {
                                movement(cur_h - dvi_h, RIGHT1);
                                dvi_h = cur_h;
                            }
                            if (cur_v != dvi_v) {
                                movement(cur_v - dvi_v, DOWN1);
                                dvi_v = cur_v;
                            }
                            f = mem[p + 4].qqqq.u.B1;
                            if (f != dvi_f) {   /*643: */
                                if (!font_used[f]) {
                                    dvi_font_def(f);
                                    font_used[f] = true;
                                }
                                if (f <= 64) {
                                    dvi_buf[dvi_ptr] = f + 170;
                                    dvi_ptr++;
                                    if (dvi_ptr == dvi_limit)
                                        dvi_swap();
                                } else if (f <= 256) {
                                    {
                                        dvi_buf[dvi_ptr] = FNT1;
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                    {
                                        dvi_buf[dvi_ptr] = f - 1;
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                } else {

                                    {
                                        dvi_buf[dvi_ptr] = (FNT1 + 1);
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                    {
                                        dvi_buf[dvi_ptr] = (f - 1) / 256;
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                    {
                                        dvi_buf[dvi_ptr] = (f - 1) % 256;
                                        dvi_ptr++;
                                        if (dvi_ptr == dvi_limit)
                                            dvi_swap();
                                    }
                                }
                                dvi_f = f;
                            }
                            {
                                dvi_buf[dvi_ptr] = SET_GLYPHS;
                                dvi_ptr++;
                                if (dvi_ptr == dvi_limit)
                                    dvi_swap();
                            }
                            dvi_four(0);
                            dvi_two(1);
                            dvi_four(0);
                            dvi_four(0);
                            dvi_two(mem[p + 4].qqqq.u.B2);
                            cur_v = cur_v + mem[p + 2].cint;
                            cur_h = left_edge;
                        }
                        break;
                    case 43:
                    case 44:
                        {
                            save_h = dvi_h;
                            save_v = dvi_v;
                            cur_v = cur_v + mem[p + 3].cint;
                            pic_out(p);
                            dvi_h = save_h;
                            dvi_v = save_v;
                            cur_v = save_v + mem[p + 2].cint;
                            cur_h = left_edge;
                        }
                        break;
                    case 6:
                        {
                            pdf_last_x_pos = cur_h + cur_h_offset;
                            pdf_last_y_pos = cur_page_height - cur_v - cur_v_offset;
                        }
                        break;
                    default:
                        out_what(p);
                        break;
                    }
                }
                break;
            case 10:
                {
                    g = mem[p + 1].hh.v.LH;
                    rule_ht = mem[g + 1].cint - cur_g;
                    if (g_sign != NORMAL) {
                        if (g_sign == STRETCHING) {
                            if (mem[g].hh.u.B0 == g_order) {
                                cur_glue = cur_glue + mem[g + 2].cint;
                                glue_temp = mem[this_box + 6].gr * cur_glue;
                                if (glue_temp > 1000000000.0)
                                    glue_temp = 1000000000.0;
                                else if (glue_temp < -1000000000.0)
                                    glue_temp = -1000000000.0;
                                cur_g = tex_round(glue_temp);
                            }
                        } else if (mem[g].hh.u.B1 == g_order) {
                            cur_glue = cur_glue - mem[g + 3].cint;
                            glue_temp = mem[this_box + 6].gr * cur_glue;
                            if (glue_temp > 1000000000.0)
                                glue_temp = 1000000000.0;
                            else if (glue_temp < -1000000000.0)
                                glue_temp = -1000000000.0;
                            cur_g = tex_round(glue_temp);
                        }
                    }
                    rule_ht = rule_ht + cur_g;
                    if (mem[p].hh.u.B1 >= A_LEADERS) {  /*657: */
                        leader_box = mem[p + 1].hh.v.RH;
                        if (mem[leader_box].hh.u.B0 == RULE_NODE) {
                            rule_wd = mem[leader_box + 1].cint;
                            rule_dp = 0;
                            goto lab14;
                        }
                        leader_ht = mem[leader_box + 3].cint + mem[leader_box + 2].cint;
                        if ((leader_ht > 0) && (rule_ht > 0)) {
                            rule_ht = rule_ht + 10;
                            edge = cur_v + rule_ht;
                            lx = 0;
                            if (mem[p].hh.u.B1 == A_LEADERS) {
                                save_v = cur_v;
                                cur_v = top_edge + leader_ht * ((cur_v - top_edge) / leader_ht);
                                if (cur_v < save_v)
                                    cur_v = cur_v + leader_ht;
                            } else {

                                lq = rule_ht / leader_ht;
                                lr = rule_ht % leader_ht;
                                if (mem[p].hh.u.B1 == C_LEADERS)
                                    cur_v = cur_v + (lr / 2);
                                else {

                                    lx = lr / (lq + 1);
                                    cur_v = cur_v + ((lr - (lq - 1) * lx) / 2);
                                }
                            }
                            while (cur_v + leader_ht <= edge) { /*659: */

                                if (cur_dir == RIGHT_TO_LEFT)
                                    cur_h = left_edge - mem[leader_box + 4].cint;
                                else
                                    cur_h = left_edge + mem[leader_box + 4].cint;
                                if (cur_h != dvi_h) {
                                    movement(cur_h - dvi_h, RIGHT1);
                                    dvi_h = cur_h;
                                }
                                save_h = dvi_h;
                                cur_v = cur_v + mem[leader_box + 3].cint;
                                if (cur_v != dvi_v) {
                                    movement(cur_v - dvi_v, DOWN1);
                                    dvi_v = cur_v;
                                }
                                save_v = dvi_v;
                                temp_ptr = leader_box;
                                outer_doing_leaders = doing_leaders;
                                doing_leaders = true;
                                if (mem[leader_box].hh.u.B0 == VLIST_NODE)
                                    vlist_out();
                                else
                                    hlist_out();
                                doing_leaders = outer_doing_leaders;
                                dvi_v = save_v;
                                dvi_h = save_h;
                                cur_h = left_edge;
                                cur_v = save_v - mem[leader_box + 3].cint + leader_ht + lx;
                            }
                            cur_v = edge - 10;
                            goto lab15;
                        }
                    }
                    goto lab13;
                }
                break;
            case 11:
                if (upwards)
                    cur_v = cur_v - mem[p + 1].cint;
                else
                    cur_v = cur_v + mem[p + 1].cint;
                break;
            default:
                ;
                break;
            }
            goto lab15;
 lab14:                        /*fin_rule *//*655: */ if ((rule_wd == NULL_FLAG))
                rule_wd = mem[this_box + 1].cint;
            rule_ht = rule_ht + rule_dp;
            if (upwards)
                cur_v = cur_v - rule_ht;
            else
                cur_v = cur_v + rule_ht;
            if ((rule_ht > 0) && (rule_wd > 0)) {
                if (cur_dir == RIGHT_TO_LEFT)
                    cur_h = cur_h - rule_wd;
                if (cur_h != dvi_h) {
                    movement(cur_h - dvi_h, RIGHT1);
                    dvi_h = cur_h;
                }
                if (cur_v != dvi_v) {
                    movement(cur_v - dvi_v, DOWN1);
                    dvi_v = cur_v;
                }
                {
                    dvi_buf[dvi_ptr] = PUT_RULE;
                    dvi_ptr++;
                    if (dvi_ptr == dvi_limit)
                        dvi_swap();
                }
                dvi_four(rule_ht);
                dvi_four(rule_wd);
                cur_h = left_edge;
            }
            goto lab15;
 lab13:                        /*move_past */ if (upwards)
                cur_v = cur_v - rule_ht;
            else
                cur_v = cur_v + rule_ht;
        }
 lab15:                        /*next_p */ p = mem[p].hh.v.RH;
    }
    synctex_tsilv(this_box);
    prune_movements(save_loc);
    if (cur_s > 0)
        dvi_pop(save_loc);
    cur_s--;
}

void ship_out(int32_t p)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer page_loc;
    unsigned char j, k;
    pool_pointer s;
    unsigned char /*max_selector */ old_setting;

    synctex_sheet(INTPAR(mag));
    {
        if (job_name == 0)
            open_log_file();
        if (INTPAR(tracing_output) > 0) {
            print_nl(S());
            print_ln();
            print(S(Completed_box_being_shipped_/*out*/));
        }
        if (term_offset > max_print_line - 9)
            print_ln();
        else if ((term_offset > 0) || (file_offset > 0))
            print_char(32 /*" " */ );
        print_char(91 /*"[" */ );
        j = 9;
        while ((COUNT_REG(j) == 0) && (j > 0))
            j--;
        {
            register integer for_end;
            k = 0;
            for_end = j;
            if (k <= for_end)
                do {
                    print_int(COUNT_REG(k));
                    if (k < j)
                        print_char(46 /*"." */ );
                }
                while (k++ < for_end);
        }
        ttstub_output_flush (rust_stdout);
        if (INTPAR(tracing_output) > 0) {
            print_char(93 /*"]" */ );
            begin_diagnostic();
            show_box(p);
            end_diagnostic(true);
        }
        if ((mem[p + 3].cint > MAX_HALFWORD) || (mem[p + 2].cint > MAX_HALFWORD)
            || (mem[p + 3].cint + mem[p + 2].cint + DIMENPAR(v_offset) > MAX_HALFWORD)
            || (mem[p + 1].cint + DIMENPAR(h_offset) > MAX_HALFWORD)) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Huge_page_cannot_be_shipped_/*out*/));
            }
            {
                help_ptr = 2;
                help_line[1] = S(The_page_just_created_is_mor/*e than 18 feet tall or*/);
                help_line[0] = S(more_than_18_feet_wide__so_I/* suspect something went wrong.*/);
            }
            error();
            if (INTPAR(tracing_output) <= 0) {
                begin_diagnostic();
                print_nl(S(The_following_box_has_been_d/*eleted:*/));
                show_box(p);
                end_diagnostic(true);
            }
            goto done;
        }
        if (mem[p + 3].cint + mem[p + 2].cint + DIMENPAR(v_offset) > max_v)
            max_v = mem[p + 3].cint + mem[p + 2].cint + DIMENPAR(v_offset);
        if (mem[p + 1].cint + DIMENPAR(h_offset) > max_h)
            max_h = mem[p + 1].cint + DIMENPAR(h_offset) /*:663 */ ;
        dvi_h = 0;
        dvi_v = 0;
        cur_h = DIMENPAR(h_offset);
        dvi_f = FONT_BASE;
	/* 4736287 = round(0xFFFF * 72.27) ; i.e., 1 inch expressed as a scaled */
        cur_h_offset = DIMENPAR(h_offset) + 4736287;
        cur_v_offset = DIMENPAR(v_offset) + 4736287;
        if (DIMENPAR(pdf_page_width) != 0)
            cur_page_width = DIMENPAR(pdf_page_width);
        else
            cur_page_width = mem[p + 1].cint + 2 * cur_h_offset;
        if (DIMENPAR(pdf_page_height) != 0)
            cur_page_height = DIMENPAR(pdf_page_height);
        else
            cur_page_height = mem[p + 3].cint + mem[p + 2].cint + 2 * /*:1405 */ cur_v_offset;
        if (output_file_name == 0) {
            if (job_name == 0)
                open_log_file();
            pack_job_name(output_file_extension);
	    dvi_file = ttstub_output_open (name_of_file + 1, 0);
	    if (dvi_file == NULL)
		_tt_abort ("cannot open output file \"%s\"", name_of_file + 1);
            output_file_name = make_name_string();
        }
        if (total_pages == 0) {
            {
                dvi_buf[dvi_ptr] = PRE;
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
            {
                dvi_buf[dvi_ptr] = ID_BYTE;
                dvi_ptr++;
                if (dvi_ptr == dvi_limit)
                    dvi_swap();
            }
            dvi_four(25400000L);
            dvi_four(473628672L);
            prepare_mag();
            dvi_four(INTPAR(mag));
            if (output_comment) {
                l = strlen(output_comment);
                {
                    dvi_buf[dvi_ptr] = l;
                    dvi_ptr++;
                    if (dvi_ptr == dvi_limit)
                        dvi_swap();
                }
                {
                    register integer for_end;
                    s = 0;
                    for_end = l - 1;
                    if (s <= for_end)
                        do {
                            dvi_buf[dvi_ptr] = output_comment[s];
                            dvi_ptr++;
                            if (dvi_ptr == dvi_limit)
                                dvi_swap();
                        }
                        while (s++ < for_end);
                }
            } else {

                old_setting = selector;
                selector = SELECTOR_NEW_STRING ;
                print(S(_XeTeX_output_));
                print_int(INTPAR(year));
                print_char(46 /*"." */ );
                print_two(INTPAR(month));
                print_char(46 /*"." */ );
                print_two(INTPAR(day));
                print_char(58 /*":" */ );
                print_two(INTPAR(time) / 60);
                print_two(INTPAR(time) % 60);
                selector = old_setting;
                {
                    dvi_buf[dvi_ptr] = (pool_ptr - str_start[(str_ptr) - 65536L]);
                    dvi_ptr++;
                    if (dvi_ptr == dvi_limit)
                        dvi_swap();
                }
                {
                    register integer for_end;
                    s = str_start[(str_ptr) - 65536L];
                    for_end = pool_ptr - 1;
                    if (s <= for_end)
                        do {
                            dvi_buf[dvi_ptr] = str_pool[s];
                            dvi_ptr++;
                            if (dvi_ptr == dvi_limit)
                                dvi_swap();
                        }
                        while (s++ < for_end);
                }
                pool_ptr = str_start[(str_ptr) - 65536L];
            }
        }
        page_loc = dvi_offset + dvi_ptr;
        {
            dvi_buf[dvi_ptr] = BOP;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            register integer for_end;
            k = 0;
            for_end = 9;
            if (k <= for_end)
                do
                    dvi_four(COUNT_REG(k));
                while (k++ < for_end);
        }
        dvi_four(last_bop);
        last_bop = page_loc;
        old_setting = selector;
        selector = SELECTOR_NEW_STRING ;
        print(S(pdf_pagesize_));
        if ((DIMENPAR(pdf_page_width) > 0) && (DIMENPAR(pdf_page_height) > 0)) {
            print(S(width));
            print(32 /*" " */ );
            print_scaled(DIMENPAR(pdf_page_width));
            print(S(pt));
            print(32 /*" " */ );
            print(S(height));
            print(32 /*" " */ );
            print_scaled(DIMENPAR(pdf_page_height));
            print(S(pt));
        } else
            print(S(default));
        selector = old_setting;
        {
            dvi_buf[dvi_ptr] = XXX1;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = (pool_ptr - str_start[(str_ptr) - 65536L]);
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            register integer for_end;
            s = str_start[(str_ptr) - 65536L];
            for_end = pool_ptr - 1;
            if (s <= for_end)
                do {
                    dvi_buf[dvi_ptr] = str_pool[s];
                    dvi_ptr++;
                    if (dvi_ptr == dvi_limit)
                        dvi_swap();
                }
                while (s++ < for_end);
        }
        pool_ptr = str_start[(str_ptr) - 65536L];
        cur_v = mem[p + 3].cint + DIMENPAR(v_offset);
        temp_ptr = p;
        if (mem[p].hh.u.B0 == VLIST_NODE)
            vlist_out();
        else
            hlist_out();
        {
            dvi_buf[dvi_ptr] = EOP;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        total_pages++;
        cur_s = -1; /*:662 */

    done:
/*1518: */
        if (LR_problems > 0) {
            print_ln();
            print_nl(S(_endL_or__endR_problem__));
            print_int(LR_problems / 10000);
            print(S(_missing__));
            print_int(LR_problems % 10000);
            print(S(_extra));
            LR_problems = 0;
            print_char(41 /*")" */ );
            print_ln();
        }

        if (LR_ptr != MIN_HALFWORD || cur_dir != LEFT_TO_RIGHT)
            confusion(S(LR3));

        if (INTPAR(tracing_output) <= 0)
            print_char(93 /*"]" */ );
        dead_cycles = 0;
        ttstub_output_flush (rust_stdout);
        flush_node_list(p);
    }
    synctex_teehs();
}

void scan_spec(group_code c, boolean three_codes)
{
    integer s;
    unsigned char /*additional */ spec_code;
    if (three_codes)
        s = save_stack[save_ptr + 0].cint;
    if (scan_keyword(S(to)))
        spec_code = EXACTLY;
    else if (scan_keyword(S(spread)))
        spec_code = ADDITIONAL;
    else {

        spec_code = ADDITIONAL;
        cur_val = 0;
        goto lab40;
    }
    scan_dimen(false, false, false);
 lab40:                        /*found */ if (three_codes) {
        save_stack[save_ptr + 0].cint = s;
        save_ptr++;
    }
    save_stack[save_ptr + 0].cint = spec_code;
    save_stack[save_ptr + 1].cint = cur_val;
    save_ptr = save_ptr + 2;
    new_save_level(c);
    scan_left_brace();
}

scaled char_pw(int32_t p, small_number side)
{
    register scaled Result;
    memory_word *mem = zmem; internal_font_number f;
    integer c;
    Result = 0;
    if (side == 0)
        last_leftmost_char = MIN_HALFWORD;
    else
        last_rightmost_char = MIN_HALFWORD;
    if (p == MIN_HALFWORD)
        return Result;
    if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
          && ((mem[p].hh.u.B1 == NATIVE_WORD_NODE) || (mem[p].hh.u.B1 == NATIVE_WORD_NODE_AT))))) {
        if (mem[p + 5].ptr != NULL) {
            f = mem[p + 4].qqqq.u.B1;
            Result =
                round_xn_over_d(font_info[QUAD_CODE + param_base[f]].cint, get_native_word_cp(p, side), 1000);
        }
        return Result;
    }
    if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
          && (mem[p].hh.u.B1 == GLYPH_NODE)))) {
        f = mem[p + 4].qqqq.u.B1;
        Result =
            round_xn_over_d(font_info[QUAD_CODE + param_base[f]].cint, get_cp_code(f, mem[p + 4].qqqq.u.B2, side),
                            1000);
        return Result;
    }
    if (!(p >= hi_mem_min)) {
        if (mem[p].hh.u.B0 == LIGATURE_NODE)
            p = p + 1;
        else
            return Result;
    }
    f = mem[p].hh.u.B0;
    c = get_cp_code(f, mem[p].hh.u.B1, side);
    switch (side) {
    case 0:
        last_leftmost_char = p;
        break;
    case 1:
        last_rightmost_char = p;
        break;
    }
    if (c == 0)
        return Result;
    Result = round_xn_over_d(font_info[QUAD_CODE + param_base[f]].cint, c, 1000);
    return Result;
}

int32_t new_margin_kern(scaled w, int32_t p, small_number side)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t k;
    k = get_node(MARGIN_KERN_NODE_SIZE);
    mem[k].hh.u.B0 = MARGIN_KERN_NODE;
    mem[k].hh.u.B1 = side;
    mem[k + 1].cint = w;
    Result = k;
    return Result;
}

int32_t hpack(int32_t p, scaled w, small_number m)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    memory_word *mem = zmem; int32_t r;
    int32_t q;
    scaled h, d, x;
    scaled s;
    int32_t g;
    glue_ord o;
    internal_font_number f;
    four_quarters i;
    eight_bits hd;
    int32_t pp, ppp;
    integer total_chars, k;

    last_badness = 0;
    r = get_node(BOX_NODE_SIZE);
    mem[r].hh.u.B0 = HLIST_NODE;
    mem[r].hh.u.B1 = 0;
    mem[r + 4].cint = 0;
    q = r + 5;
    mem[q].hh.v.RH = p;
    h = 0;
    d = 0;
    x = 0;
    total_stretch[NORMAL] = 0;
    total_shrink[NORMAL] = 0;
    total_stretch[FIL] = 0;
    total_shrink[FIL] = 0;
    total_stretch[FILL] = 0;
    total_shrink[FILL] = 0;
    total_stretch[FILLL] = 0;
    total_shrink[FILLL] = 0 /*:673 */ ;
    if ((eqtb[ETEX_STATE_BASE].cint > 0)) {    /*1497: */
        temp_ptr = get_avail();
        mem[temp_ptr].hh.v.LH = BEFORE;
        mem[temp_ptr].hh.v.RH = LR_ptr;
        LR_ptr = temp_ptr;
    }
    while (p != MIN_HALFWORD) {  /*674: */

 lab21:                                                /*reswitch */ while ((p >= hi_mem_min)) {
                                                        /*677: */

            f = mem[p].hh.u.B0;
            i = font_info[char_base[f] + effective_char(true, f, mem[p].hh.u.B1)].qqqq;
            hd = i.u.B1;
            x = x + font_info[width_base[f] + i.u.B0].cint;
            s = font_info[height_base[f] + (hd) / 16].cint;
            if (s > h)
                h = s;
            s = font_info[depth_base[f] + (hd) % 16].cint;
            if (s > d)
                d = s;
            p = mem[p].hh.v.RH;
        }
        if (p != MIN_HALFWORD) {
            switch (mem[p].hh.u.B0) {
            case 0:
            case 1:
            case 2:
            case 13:
                {
                    x = x + mem[p + 1].cint;
                    if (mem[p].hh.u.B0 >= RULE_NODE)
                        s = 0;
                    else
                        s = mem[p + 4].cint;
                    if (mem[p + 3].cint - s > h)
                        h = mem[p + 3].cint - s;
                    if (mem[p + 2].cint + s > d)
                        d = mem[p + 2].cint + s;
                }
                break;
            case 3:
            case 4:
            case 5:
                if ((adjust_tail != MIN_HALFWORD) || (pre_adjust_tail != MIN_HALFWORD)) { /*680: */
                    while (mem[q].hh.v.RH != p)
                        q = mem[q].hh.v.RH;
                    if (mem[p].hh.u.B0 == ADJUST_NODE) {
                        if (mem[p].hh.u.B1 != 0) {
                            if (pre_adjust_tail == MIN_HALFWORD)
                                confusion(S(pre_vadjust));
                            mem[pre_adjust_tail].hh.v.RH = mem[p + 1].cint;
                            while (mem[pre_adjust_tail].hh.v.RH != MIN_HALFWORD)
                                pre_adjust_tail = mem[pre_adjust_tail].hh.v.RH;
                        } else {

                            if (adjust_tail == MIN_HALFWORD)
                                confusion(S(pre_vadjust));
                            mem[adjust_tail].hh.v.RH = mem[p + 1].cint;
                            while (mem[adjust_tail].hh.v.RH != MIN_HALFWORD)
                                adjust_tail = mem[adjust_tail].hh.v.RH;
                        }
                        p = mem[p].hh.v.RH;
                        free_node(mem[q].hh.v.RH, SMALL_NODE_SIZE);
                    } else {

                        mem[adjust_tail].hh.v.RH = p;
                        adjust_tail = p;
                        p = mem[p].hh.v.RH;
                    }
                    mem[q].hh.v.RH = p;
                    p = q;
                }
                break;
            case 8:
                {
                    switch (mem[p].hh.u.B1) {
                    case 40:
                    case 41:
                        {
                            if ((q != r + 5) && (mem[q].hh.u.B0 == DISC_NODE))
                                k = mem[q].hh.u.B1;
                            else
                                k = 0;
                            while ((mem[q].hh.v.RH != p)) {

                                k--;
                                q = mem[q].hh.v.RH;
                                if (mem[q].hh.u.B0 == DISC_NODE)
                                    k = mem[q].hh.u.B1;
                            }
                            pp = mem[p].hh.v.RH;
 lab20:                        /*restart */ if ((k <= 0) && (pp != MIN_HALFWORD) && (!(pp >= hi_mem_min))) {
                                if ((mem[pp].hh.u.B0 == WHATSIT_NODE)
                                    && ((mem[pp].hh.u.B1 == NATIVE_WORD_NODE)
                                        || (mem[pp].hh.u.B1 == NATIVE_WORD_NODE_AT))
                                    && (mem[pp + 4].qqqq.u.B1 == mem[p + 4].qqqq.u.B1)) {
                                    pp = mem[pp].hh.v.RH;
                                    goto lab20;
                                } else if ((mem[pp].hh.u.B0 == DISC_NODE)) {
                                    ppp = mem[pp].hh.v.RH;
                                    if ((((ppp) != MIN_HALFWORD && (!(ppp >= hi_mem_min))
                                          && (mem[ppp].hh.u.B0 == WHATSIT_NODE)
                                          && ((mem[ppp].hh.u.B1 == NATIVE_WORD_NODE)
                                              || (mem[ppp].hh.u.B1 == NATIVE_WORD_NODE_AT))))
                                        && (mem[ppp + 4].qqqq.u.B1 == mem[p + 4].qqqq.u.B1)) {
                                        pp = mem[ppp].hh.v.RH;
                                        goto lab20;
                                    }
                                }
                            }
                            if ((pp != mem[p].hh.v.RH)) {
                                total_chars = 0;
                                p = mem[q].hh.v.RH;
                                while ((p != pp)) {

                                    if ((mem[p].hh.u.B0 == WHATSIT_NODE))
                                        total_chars = total_chars + mem[p + 4].qqqq.u.B2;
                                    ppp = p;
                                    p = mem[p].hh.v.RH;
                                }
                                p = mem[q].hh.v.RH;
                                pp = new_native_word_node(mem[p + 4].qqqq.u.B1, total_chars);
                                mem[pp].hh.u.B1 = mem[p].hh.u.B1;
                                mem[q].hh.v.RH = pp;
                                mem[pp].hh.v.RH = mem[ppp].hh.v.RH;
                                mem[ppp].hh.v.RH = MIN_HALFWORD;
                                total_chars = 0;
                                ppp = p;
                                do {
                                    if ((mem[ppp].hh.u.B0 == WHATSIT_NODE)) {
                                        register integer for_end;
                                        k = 0;
                                        for_end = mem[ppp + 4].qqqq.u.B2 - 1;
                                        if (k <= for_end)
                                            do {
                                                set_native_char(pp, total_chars, get_native_char(ppp, k));
                                                total_chars++;
                                            }
                                            while (k++ < for_end);
                                    }
                                    ppp = mem[ppp].hh.v.RH;
                                } while (!((ppp == MIN_HALFWORD)));
                                flush_node_list(p);
                                p = mem[q].hh.v.RH;
                                set_native_metrics(p, (STATEINT(xetex_use_glyph_metrics) > 0));
                            }
                            if (mem[p + 3].cint > h)
                                h = mem[p + 3].cint;
                            if (mem[p + 2].cint > d)
                                d = mem[p + 2].cint;
                            x = x + mem[p + 1].cint;
                        }
                        break;
                    case 42:
                    case 43:
                    case 44:
                        {
                            if (mem[p + 3].cint > h)
                                h = mem[p + 3].cint;
                            if (mem[p + 2].cint > d)
                                d = mem[p + 2].cint;
                            x = x + mem[p + 1].cint;
                        }
                        break;
                    default:
                        ;
                        break;
                    }
                }
                break;
            case 10:
                {
                    g = mem[p + 1].hh.v.LH;
                    x = x + mem[g + 1].cint;
                    o = mem[g].hh.u.B0;
                    total_stretch[o] = total_stretch[o] + mem[g + 2].cint;
                    o = mem[g].hh.u.B1;
                    total_shrink[o] = total_shrink[o] + mem[g + 3].cint;
                    if (mem[p].hh.u.B1 >= A_LEADERS) {
                        g = mem[p + 1].hh.v.RH;
                        if (mem[g + 3].cint > h)
                            h = mem[g + 3].cint;
                        if (mem[g + 2].cint > d)
                            d = mem[g + 2].cint;
                    }
                }
                break;
            case 11:
                x = x + mem[p + 1].cint;
                break;
            case 40:
                x = x + mem[p + 1].cint;
                break;
            case 9:
                {
                    x = x + mem[p + 1].cint;
                    if ((eqtb[ETEX_STATE_BASE].cint > 0)) {    /*1498: */

                        if (odd(mem[p].hh.u.B1)) {

                            if (mem[LR_ptr].hh.v.LH == (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3)) {
                                temp_ptr = LR_ptr;
                                LR_ptr = mem[temp_ptr].hh.v.RH;
                                {
                                    mem[temp_ptr].hh.v.RH = avail;
                                    avail = temp_ptr;
                                }
                            } else {

                                LR_problems++;
                                mem[p].hh.u.B0 = KERN_NODE;
                                mem[p].hh.u.B1 = EXPLICIT;
                            }
                        } else {

                            temp_ptr = get_avail();
                            mem[temp_ptr].hh.v.LH = (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3);
                            mem[temp_ptr].hh.v.RH = LR_ptr;
                            LR_ptr = temp_ptr;
                        }
                    }
                }
                break;
            case 6:
                {
                    mem[mem_top - 12] = mem[p + 1];
                    mem[mem_top - 12].hh.v.RH = mem[p].hh.v.RH;
                    p = mem_top - 12;
                    xtx_ligature_present = true;
                    goto lab21;
                }
                break;
            default:
                ;
                break;
            }
            p = mem[p].hh.v.RH;
        }
    }
    if (adjust_tail != MIN_HALFWORD)
        mem[adjust_tail].hh.v.RH = MIN_HALFWORD;
    if (pre_adjust_tail != MIN_HALFWORD)
        mem[pre_adjust_tail].hh.v.RH = MIN_HALFWORD;
    mem[r + 3].cint = h;
    mem[r + 2].cint = d;
    if (m == ADDITIONAL)
        w = x + w;
    mem[r + 1].cint = w;
    x = w - x;
    if (x == 0) {
        mem[r + 5].hh.u.B0 = NORMAL;
        mem[r + 5].hh.u.B1 = NORMAL;
        mem[r + 6].gr = 0.0;
        goto exit;
    } else if (x > 0) {         /*683: */
        if (total_stretch[FILLL] != 0)
            o = FILLL;
        else if (total_stretch[FILL] != 0)
            o = FILL;
        else if (total_stretch[FIL] != 0)
            o = FIL;
        else
            o = 0 /*normal *//*:684 */ ;
        mem[r + 5].hh.u.B1 = o;
        mem[r + 5].hh.u.B0 = STRETCHING;
        if (total_stretch[o] != 0)
            mem[r + 6].gr = x / ((double)total_stretch[o]);
        else {

            mem[r + 5].hh.u.B0 = NORMAL;
            mem[r + 6].gr = 0.0;
        }
        if (o == NORMAL) {

            if (mem[r + 5].hh.v.RH != MIN_HALFWORD) {    /*685: */
                last_badness = badness(x, total_stretch[NORMAL]);
                if (last_badness > INTPAR(hbadness)) {
                    print_ln();
                    if (last_badness > 100)
                        print_nl(S(Underfull));
                    else
                        print_nl(S(Loose));
                    print(S(__hbox__badness_));
                    print_int(last_badness);
                    goto lab50;
                }
            }
        }
        goto exit;
    } else {                    /*689: */

        if (total_shrink[FILLL] != 0)
            o = FILLL;
        else if (total_shrink[FILL] != 0)
            o = FILL;
        else if (total_shrink[FIL] != 0)
            o = FIL;
        else
            o = 0 /*normal *//*:690 */ ;
        mem[r + 5].hh.u.B1 = o;
        mem[r + 5].hh.u.B0 = SHRINKING;
        if (total_shrink[o] != 0)
            mem[r + 6].gr = (-(integer) x) / ((double)total_shrink[o]);
        else {

            mem[r + 5].hh.u.B0 = NORMAL;
            mem[r + 6].gr = 0.0;
        }
        if ((total_shrink[o] < -(integer) x) && (o == NORMAL) && (mem[r + 5].hh.v.RH != MIN_HALFWORD)) {
            last_badness = 1000000L;
            mem[r + 6].gr = 1.0;
            if ((-(integer) x - total_shrink[NORMAL] > DIMENPAR(hfuzz))
                || (INTPAR(hbadness) < 100)) {
                if ((DIMENPAR(overfull_rule) > 0)
                    && (-(integer) x - total_shrink[NORMAL] > DIMENPAR(hfuzz))) {
                    while (mem[q].hh.v.RH != MIN_HALFWORD)
                        q = mem[q].hh.v.RH;
                    mem[q].hh.v.RH = new_rule();
                    mem[mem[q].hh.v.RH + 1].cint = DIMENPAR(overfull_rule);
                }
                print_ln();
                print_nl(S(Overfull__hbox__));
                print_scaled(-(integer) x - total_shrink[NORMAL]);
                print(S(pt_too_wide));
                goto lab50;
            }
        } else if (o == NORMAL) {

            if (mem[r + 5].hh.v.RH != MIN_HALFWORD) {    /*692: */
                last_badness = badness(-(integer) x, total_shrink[NORMAL]);
                if (last_badness > INTPAR(hbadness)) {
                    print_ln();
                    print_nl(S(Tight__hbox__badness_));
                    print_int(last_badness);
                    goto lab50;
                }
            }
        }
        goto exit;
    }
 lab50:                        /*common_ending *//*688: */ if (output_active)
        print(S(__has_occurred_while__output/* is active*/));
    else {

        if (pack_begin_line != 0) {
            if (pack_begin_line > 0)
                print(S(__in_paragraph_at_lines_));
            else
                print(S(__in_alignment_at_lines_));
            print_int(abs(pack_begin_line));
            print(S(___Z15/*"--"*/));
        } else
            print(S(__detected_at_line_));
        print_int(line);
    }
    print_ln();
    font_in_short_display = FONT_BASE;
    short_display(mem[r + 5].hh.v.RH);
    print_ln();
    begin_diagnostic();
    show_box(r);
    end_diagnostic(true);

exit:
    if ((eqtb[ETEX_STATE_BASE].cint > 0)) {
	/*1499: */
        if (mem[LR_ptr].hh.v.LH != BEFORE) {
            while (mem[q].hh.v.RH != MIN_HALFWORD)
                q = mem[q].hh.v.RH;
            do {
                temp_ptr = q;
                q = new_math(0, mem[LR_ptr].hh.v.LH);
                mem[temp_ptr].hh.v.RH = q;
                LR_problems = LR_problems + 10000;
                {
                    temp_ptr = LR_ptr;
                    LR_ptr = mem[temp_ptr].hh.v.RH;
                    {
                        mem[temp_ptr].hh.v.RH = avail;
                        avail = temp_ptr;
                    }
                }
            } while (!(mem[LR_ptr].hh.v.LH == BEFORE));
        }
        if (LR_problems > 0) {
            {
                print_ln();
                print_nl(S(_endL_or__endR_problem__));
                print_int(LR_problems / 10000);
                print(S(_missing__));
                print_int(LR_problems % 10000);
                print(S(_extra));
                LR_problems = 0;
            }
            goto lab50;
        }
        {
            temp_ptr = LR_ptr;
            LR_ptr = mem[temp_ptr].hh.v.RH;
            {
                mem[temp_ptr].hh.v.RH = avail;
                avail = temp_ptr;
            }
        }
        if (LR_ptr != MIN_HALFWORD)
            confusion(S(LR1));
    }
    Result = r;
    return Result;
}

int32_t vpackage(int32_t p, scaled h, small_number m, scaled l)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    memory_word *mem = zmem; int32_t r;
    scaled w, d, x;
    scaled s;
    int32_t g;
    glue_ord o;

    last_badness = 0;
    r = get_node(BOX_NODE_SIZE);
    mem[r].hh.u.B0 = VLIST_NODE;
    if ((STATEINT(xetex_upwards) > 0))
        mem[r].hh.u.B1 = 1;
    else
        mem[r].hh.u.B1 = 0;
    mem[r + 4].cint = 0;
    mem[r + 5].hh.v.RH = p;
    w = 0;
    d = 0;
    x = 0;
    total_stretch[NORMAL] = 0;
    total_shrink[NORMAL] = 0;
    total_stretch[FIL] = 0;
    total_shrink[FIL] = 0;
    total_stretch[FILL] = 0;
    total_shrink[FILL] = 0;
    total_stretch[FILLL] = 0;
    total_shrink[FILLL] = 0 /*:673 */ ;
    while (p != MIN_HALFWORD) {  /*694: */

        if ((p >= hi_mem_min))
            confusion(S(vpack));
        else
            switch (mem[p].hh.u.B0) {
            case 0:
            case 1:
            case 2:
            case 13:
                {
                    x = x + d + mem[p + 3].cint;
                    d = mem[p + 2].cint;
                    if (mem[p].hh.u.B0 >= RULE_NODE)
                        s = 0;
                    else
                        s = mem[p + 4].cint;
                    if (mem[p + 1].cint + s > w)
                        w = mem[p + 1].cint + s;
                }
                break;
            case 8:
                {
                    if ((mem[p].hh.u.B1 == PIC_NODE) || (mem[p].hh.u.B1 == PDF_NODE)) {
                        x = x + d + mem[p + 3].cint;
                        d = mem[p + 2].cint;
                        if (mem[p + 1].cint > w)
                            w = mem[p + 1].cint;
                    }
                }
                break;
            case 10:
                {
                    x = x + d;
                    d = 0;
                    g = mem[p + 1].hh.v.LH;
                    x = x + mem[g + 1].cint;
                    o = mem[g].hh.u.B0;
                    total_stretch[o] = total_stretch[o] + mem[g + 2].cint;
                    o = mem[g].hh.u.B1;
                    total_shrink[o] = total_shrink[o] + mem[g + 3].cint;
                    if (mem[p].hh.u.B1 >= A_LEADERS) {
                        g = mem[p + 1].hh.v.RH;
                        if (mem[g + 1].cint > w)
                            w = mem[g + 1].cint;
                    }
                }
                break;
            case 11:
                {
                    x = x + d + mem[p + 1].cint;
                    d = 0;
                }
                break;
            default:
                ;
                break;
            }
        p = mem[p].hh.v.RH;
    }
    mem[r + 1].cint = w;
    if (d > l) {
        x = x + d - l;
        mem[r + 2].cint = l;
    } else
        mem[r + 2].cint = d;
    if (m == ADDITIONAL)
        h = x + h;
    mem[r + 3].cint = h;
    x = h - x;
    if (x == 0) {
        mem[r + 5].hh.u.B0 = NORMAL;
        mem[r + 5].hh.u.B1 = NORMAL;
        mem[r + 6].gr = 0.0;
        goto exit;
    } else if (x > 0) {         /*698: */
        if (total_stretch[FILLL] != 0)
            o = FILLL;
        else if (total_stretch[FILL] != 0)
            o = FILL;
        else if (total_stretch[FIL] != 0)
            o = FIL;
        else
            o = 0 /*normal *//*:684 */ ;
        mem[r + 5].hh.u.B1 = o;
        mem[r + 5].hh.u.B0 = STRETCHING;
        if (total_stretch[o] != 0)
            mem[r + 6].gr = x / ((double)total_stretch[o]);
        else {

            mem[r + 5].hh.u.B0 = NORMAL;
            mem[r + 6].gr = 0.0;
        }
        if (o == NORMAL) {

            if (mem[r + 5].hh.v.RH != MIN_HALFWORD) {    /*699: */
                last_badness = badness(x, total_stretch[NORMAL]);
                if (last_badness > INTPAR(vbadness)) {
                    print_ln();
                    if (last_badness > 100)
                        print_nl(S(Underfull));
                    else
                        print_nl(S(Loose));
                    print(S(__vbox__badness_));
                    print_int(last_badness);
                    goto lab50;
                }
            }
        }
        goto exit;
    } else {                    /*701: */

        if (total_shrink[FILLL] != 0)
            o = FILLL;
        else if (total_shrink[FILL] != 0)
            o = FILL;
        else if (total_shrink[FIL] != 0)
            o = FIL;
        else
            o = 0 /*normal *//*:690 */ ;
        mem[r + 5].hh.u.B1 = o;
        mem[r + 5].hh.u.B0 = SHRINKING;
        if (total_shrink[o] != 0)
            mem[r + 6].gr = (-(integer) x) / ((double)total_shrink[o]);
        else {

            mem[r + 5].hh.u.B0 = NORMAL;
            mem[r + 6].gr = 0.0;
        }
        if ((total_shrink[o] < -(integer) x) && (o == NORMAL) && (mem[r + 5].hh.v.RH != MIN_HALFWORD)) {
            last_badness = 1000000L;
            mem[r + 6].gr = 1.0;
            if ((-(integer) x - total_shrink[NORMAL] > DIMENPAR(vfuzz))
                || (INTPAR(vbadness) < 100)) {
                print_ln();
                print_nl(S(Overfull__vbox__));
                print_scaled(-(integer) x - total_shrink[NORMAL]);
                print(S(pt_too_high));
                goto lab50;
            }
        } else if (o == NORMAL) {

            if (mem[r + 5].hh.v.RH != MIN_HALFWORD) {    /*703: */
                last_badness = badness(-(integer) x, total_shrink[NORMAL]);
                if (last_badness > INTPAR(vbadness)) {
                    print_ln();
                    print_nl(S(Tight__vbox__badness_));
                    print_int(last_badness);
                    goto lab50;
                }
            }
        }
        goto exit;
    }
 lab50:                        /*common_ending *//*700: */ if (output_active)
        print(S(__has_occurred_while__output/* is active*/));
    else {

        if (pack_begin_line != 0) {
            print(S(__in_alignment_at_lines_));
            print_int(abs(pack_begin_line));
            print(S(___Z15/*"--"*/));
        } else
            print(S(__detected_at_line_));
        print_int(line);
        print_ln();
    }
    begin_diagnostic();
    show_box(r);
    end_diagnostic(true);

exit:
    Result = r;
    return Result;
}

void append_to_vlist(int32_t b)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    scaled d;
    int32_t p;
    boolean upwards;

    upwards = (STATEINT(xetex_upwards) > 0);
    if (cur_list.aux.cint > -65536000L) {
        if (upwards)
            d = mem[GLUEPAR(baseline_skip) + 1].cint - cur_list.aux.cint - mem[b + 2].cint;
        else
            d = mem[GLUEPAR(baseline_skip) + 1].cint - cur_list.aux.cint - mem[b + 3].cint;
        if (d < DIMENPAR(line_skip_limit))
            p = new_param_glue(GLUE_PAR__line_skip);
        else {

            p = new_skip_param(GLUE_PAR__baseline_skip);
            mem[temp_ptr + 1].cint = d;
        }
        mem[cur_list.tail].hh.v.RH = p;
        cur_list.tail = p;
    }
    mem[cur_list.tail].hh.v.RH = b;
    cur_list.tail = b;
    if (upwards)
        cur_list.aux.cint = mem[b + 3].cint;
    else
        cur_list.aux.cint = mem[b + 2].cint;
}

int32_t new_noad(void)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(NOAD_SIZE);
    mem[p].hh.u.B0 = ORD_NOAD;
    mem[p].hh.u.B1 = NORMAL;
    mem[p + 1].hh = empty;
    mem[p + 3].hh = empty;
    mem[p + 2].hh = empty;
    Result = p;
    return Result;
}

int32_t new_style(small_number s)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(STYLE_NODE_SIZE);
    mem[p].hh.u.B0 = STYLE_NODE;
    mem[p].hh.u.B1 = s;
    mem[p + 1].cint = 0;
    mem[p + 2].cint = 0;
    Result = p;
    return Result;
}

int32_t new_choice(void)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = get_node(STYLE_NODE_SIZE);
    mem[p].hh.u.B0 = CHOICE_NODE;
    mem[p].hh.u.B1 = 0;
    mem[p + 1].hh.v.LH = MIN_HALFWORD;
    mem[p + 1].hh.v.RH = MIN_HALFWORD;
    mem[p + 2].hh.v.LH = MIN_HALFWORD;
    mem[p + 2].hh.v.RH = MIN_HALFWORD;
    Result = p;
    return Result;
}

void show_info(void)
{
    memory_word *mem = zmem;

    show_node_list(mem[temp_ptr].hh.v.LH);
}

scaled math_x_height(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 5);
    else
        rval = font_info[5 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled math_quad(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 6);
    else
        rval = font_info[6 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled num1(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 8);
    else
        rval = font_info[8 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled num2(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 9);
    else
        rval = font_info[9 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled num3(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 10);
    else
        rval = font_info[10 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled denom1(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 11);
    else
        rval = font_info[11 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled denom2(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 12);
    else
        rval = font_info[12 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled sup1(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 13);
    else
        rval = font_info[13 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled sup2(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 14);
    else
        rval = font_info[14 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled sup3(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 15);
    else
        rval = font_info[15 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled sub1(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 16);
    else
        rval = font_info[16 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled sub2(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 17);
    else
        rval = font_info[17 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled sup_drop(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 18);
    else
        rval = font_info[18 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled sub_drop(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 19);
    else
        rval = font_info[19 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled delim1(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 20);
    else
        rval = font_info[20 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled delim2(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 21);
    else
        rval = font_info[21 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled axis_height(integer size_code)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(2 + size_code);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathsy_param(f, 22);
    else
        rval = font_info[22 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled default_rule_thickness(void)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 8);
    else
        rval = font_info[8 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled big_op_spacing1(void)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 9);
    else
        rval = font_info[9 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled big_op_spacing2(void)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 10);
    else
        rval = font_info[10 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled big_op_spacing3(void)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 11);
    else
        rval = font_info[11 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled big_op_spacing4(void)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 12);
    else
        rval = font_info[12 + param_base[f]].cint;
    Result = rval;
    return Result;
}

scaled big_op_spacing5(void)
{
    CACHE_THE_EQTB;
    register scaled Result;
    integer f;
    scaled rval;

    f = MATH_FONT(3 + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rval = get_native_mathex_param(f, 13);
    else
        rval = font_info[13 + param_base[f]].cint;
    Result = rval;
    return Result;
}

int32_t fraction_rule(scaled t)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    p = new_rule();
    mem[p + 3].cint = t;
    mem[p + 2].cint = 0;
    Result = p;
    return Result;
}

int32_t overbar(int32_t b, scaled k, scaled t)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p, q;
    p = new_kern(k);
    mem[p].hh.v.RH = b;
    q = fraction_rule(t);
    mem[q].hh.v.RH = p;
    p = new_kern(t);
    mem[p].hh.v.RH = q;
    Result = vpackage(p, 0, ADDITIONAL, MAX_HALFWORD);
    return Result;
}

int32_t char_box(internal_font_number f, integer c)
{
    register int32_t Result;
    memory_word *mem = zmem; four_quarters q;
    eight_bits hd;
    int32_t b, p;
    if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))) {
        b = new_null_box();
        p = new_native_character(f, c);
        mem[b + 5].hh.v.RH = p;
        mem[b + 3].cint = mem[p + 3].cint;
        mem[b + 1].cint = mem[p + 1].cint;
        if (mem[p + 2].cint < 0)
            mem[b + 2].cint = 0;
        else
            mem[b + 2].cint = mem[p + 2].cint;
    } else {

        q = font_info[char_base[f] + effective_char(true, f, c)].qqqq;
        hd = q.u.B1;
        b = new_null_box();
        mem[b + 1].cint = font_info[width_base[f] + q.u.B0].cint + font_info[italic_base[f] + (q.u.B2) / 4].cint;
        mem[b + 3].cint = font_info[height_base[f] + (hd) / 16].cint;
        mem[b + 2].cint = font_info[depth_base[f] + (hd) % 16].cint;
        p = get_avail();
        mem[p].hh.u.B1 = c;
        mem[p].hh.u.B0 = f;
    }
    mem[b + 5].hh.v.RH = p;
    Result = b;
    return Result;
}

void stack_into_box(int32_t b, internal_font_number f, uint16_t c)
{
    memory_word *mem = zmem; int32_t p;
    p = char_box(f, c);
    mem[p].hh.v.RH = mem[b + 5].hh.v.RH;
    mem[b + 5].hh.v.RH = p;
    mem[b + 3].cint = mem[p + 3].cint;
}

scaled height_plus_depth(internal_font_number f, uint16_t c)
{
    register scaled Result;
    four_quarters q;
    eight_bits hd;
    q = font_info[char_base[f] + effective_char(true, f, c)].qqqq;
    hd = q.u.B1;
    Result = font_info[height_base[f] + (hd) / 16].cint + font_info[depth_base[f] + (hd) % 16].cint;
    return Result;
}

void stack_glyph_into_box(int32_t b, internal_font_number f, integer g)
{
    memory_word *mem = zmem; int32_t p, q;
    p = get_node(GLYPH_NODE_SIZE);
    mem[p].hh.u.B0 = WHATSIT_NODE;
    mem[p].hh.u.B1 = GLYPH_NODE;
    mem[p + 4].qqqq.u.B1 = f;
    mem[p + 4].qqqq.u.B2 = g;
    set_native_glyph_metrics(p, 1);
    if (mem[b].hh.u.B0 == HLIST_NODE) {
        q = mem[b + 5].hh.v.RH;
        if (q == MIN_HALFWORD)
            mem[b + 5].hh.v.RH = p;
        else {

            while (mem[q].hh.v.RH != MIN_HALFWORD)
                q = mem[q].hh.v.RH;
            mem[q].hh.v.RH = p;
            if ((mem[b + 3].cint < mem[p + 3].cint))
                mem[b + 3].cint = mem[p + 3].cint;
            if ((mem[b + 2].cint < mem[p + 2].cint))
                mem[b + 2].cint = mem[p + 2].cint;
        }
    } else {

        mem[p].hh.v.RH = mem[b + 5].hh.v.RH;
        mem[b + 5].hh.v.RH = p;
        mem[b + 3].cint = mem[p + 3].cint;
        if ((mem[b + 1].cint < mem[p + 1].cint))
            mem[b + 1].cint = mem[p + 1].cint;
    }
}

void stack_glue_into_box(int32_t b, scaled min, scaled max)
{
    memory_word *mem = zmem; int32_t p, q;
    q = new_spec(0);
    mem[q + 1].cint = min;
    mem[q + 2].cint = max - min;
    p = new_glue(q);
    if (mem[b].hh.u.B0 == HLIST_NODE) {
        q = mem[b + 5].hh.v.RH;
        if (q == MIN_HALFWORD)
            mem[b + 5].hh.v.RH = p;
        else {

            while (mem[q].hh.v.RH != MIN_HALFWORD)
                q = mem[q].hh.v.RH;
            mem[q].hh.v.RH = p;
        }
    } else {

        mem[p].hh.v.RH = mem[b + 5].hh.v.RH;
        mem[b + 5].hh.v.RH = p;
        mem[b + 3].cint = mem[p + 3].cint;
        mem[b + 1].cint = mem[p + 1].cint;
    }
}

int32_t build_opentype_assembly(internal_font_number f, void *a, scaled s, boolean horiz)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t b;
    integer n;
    integer i, j;
    integer g;
    int32_t p;
    scaled s_max, o, oo, prev_o, min_o;
    boolean no_extenders;
    scaled nat, str;
    b = new_null_box();
    if (horiz)
        mem[b].hh.u.B0 = HLIST_NODE;
    else
        mem[b].hh.u.B0 = VLIST_NODE;
    n = -1;
    no_extenders = true;
    min_o = ot_min_connector_overlap(f);
    do {
        n = n + 1;
        s_max = 0;
        prev_o = 0;
        {
            register integer for_end;
            i = 0;
            for_end = ot_part_count(a) - 1;
            if (i <= for_end)
                do {
                    if (ot_part_is_extender(a, i)) {
                        no_extenders = false;
                        {
                            register integer for_end;
                            j = 1;
                            for_end = n;
                            if (j <= for_end)
                                do {
                                    o = ot_part_start_connector(f, a, i);
                                    if (min_o < o)
                                        o = min_o;
                                    if (prev_o < o)
                                        o = prev_o;
                                    s_max = s_max - o + ot_part_full_advance(f, a, i);
                                    prev_o = ot_part_end_connector(f, a, i);
                                }
                                while (j++ < for_end);
                        }
                    } else {

                        o = ot_part_start_connector(f, a, i);
                        if (min_o < o)
                            o = min_o;
                        if (prev_o < o)
                            o = prev_o;
                        s_max = s_max - o + ot_part_full_advance(f, a, i);
                        prev_o = ot_part_end_connector(f, a, i);
                    }
                }
                while (i++ < for_end);
        }
    } while (!((s_max >= s) || no_extenders));
    prev_o = 0;
    {
        register integer for_end;
        i = 0;
        for_end = ot_part_count(a) - 1;
        if (i <= for_end)
            do {
                if (ot_part_is_extender(a, i)) {
                    {
                        register integer for_end;
                        j = 1;
                        for_end = n;
                        if (j <= for_end)
                            do {
                                o = ot_part_start_connector(f, a, i);
                                if (prev_o < o)
                                    o = prev_o;
                                oo = o;
                                if (min_o < o)
                                    o = min_o;
                                if (oo > 0)
                                    stack_glue_into_box(b, -(integer) oo, -(integer) o);
                                g = ot_part_glyph(a, i);
                                stack_glyph_into_box(b, f, g);
                                prev_o = ot_part_end_connector(f, a, i);
                            }
                            while (j++ < for_end);
                    }
                } else {

                    o = ot_part_start_connector(f, a, i);
                    if (prev_o < o)
                        o = prev_o;
                    oo = o;
                    if (min_o < o)
                        o = min_o;
                    if (oo > 0)
                        stack_glue_into_box(b, -(integer) oo, -(integer) o);
                    g = ot_part_glyph(a, i);
                    stack_glyph_into_box(b, f, g);
                    prev_o = ot_part_end_connector(f, a, i);
                }
            }
            while (i++ < for_end);
    }
    p = mem[b + 5].hh.v.RH;
    nat = 0;
    str = 0;
    while (p != MIN_HALFWORD) {

        if (mem[p].hh.u.B0 == WHATSIT_NODE) {
            if (horiz)
                nat = nat + mem[p + 1].cint;
            else
                nat = nat + mem[p + 3].cint + mem[p + 2].cint;
        } else if (mem[p].hh.u.B0 == GLUE_NODE) {
            nat = nat + mem[mem[p + 1].hh.v.LH + 1].cint;
            str = str + mem[mem[p + 1].hh.v.LH + 2].cint;
        }
        p = mem[p].hh.v.RH;
    }
    o = 0;
    if ((s > nat) && (str > 0)) {
        o = (s - nat);
        if ((o > str))
            o = str;
        mem[b + 5].hh.u.B1 = NORMAL;
        mem[b + 5].hh.u.B0 = STRETCHING;
        mem[b + 6].gr = o / ((double)str);
        if (horiz)
            mem[b + 1].cint = nat + tex_round(str * mem[b + 6].gr);
        else
            mem[b + 3].cint = nat + tex_round(str * mem[b + 6].gr);
    } else if (horiz)
        mem[b + 1].cint = nat;
    else
        mem[b + 3].cint = nat;
    Result = b;
    return Result;
}

int32_t var_delimiter(int32_t d, integer s, scaled v)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    memory_word *mem = zmem;
    int32_t b;
    void *ot_assembly_ptr;
    internal_font_number f, g;
    uint16_t c, x, y;
    integer m, n;
    scaled u;
    scaled w;
    four_quarters q;
    eight_bits hd;
    four_quarters r;
    integer z;
    boolean large_attempt;

    f = FONT_BASE;
    w = 0;
    large_attempt = false;
    z = (mem[d].qqqq.u.B0 % 256);
    x = (mem[d].qqqq.u.B1 + (mem[d].qqqq.u.B0 / 256) * 65536L);
    ot_assembly_ptr = NULL;
    while (true) {

        if ((z != 0) || (x != 0)) {
            z = z + s + 256;
            do {
                z = z - 256;
                g = MATH_FONT(z);
                if (g != FONT_BASE) {   /*734: */

                    if (((font_area[g] == OTGR_FONT_FLAG) && (usingOpenType(font_layout_engine[g])))) {
                        x = map_char_to_glyph(g, x);
                        f = g;
                        c = x;
                        w = 0;
                        n = 0;
                        do {
                            y = get_ot_math_variant(g, x, n, &u, 0);
                            if (u > w) {
                                c = y;
                                w = u;
                                if (u >= v)
                                    goto lab40;
                            }
                            n = n + 1;
                        } while (!(u < 0));
                        ot_assembly_ptr = get_ot_assembly_ptr(g, x, 0);
                        if (ot_assembly_ptr != NULL)
                            goto lab40;
                    } else {

                        y = x;
                        if ((y >= font_bc[g]) && (y <= font_ec[g])) {
 lab22:                        /*continue */ q = font_info[char_base[g] + y].qqqq;
                            if ((q.u.B0 > 0)) {
                                if (((q.u.B2) % 4) == EXT_TAG) {
                                    f = g;
                                    c = y;
                                    goto lab40;
                                }
                                hd = q.u.B1;
                                u = font_info[height_base[g] + (hd) / 16].cint + font_info[depth_base[g] +
                                                                                           (hd) % 16].cint;
                                if (u > w) {
                                    f = g;
                                    c = y;
                                    w = u;
                                    if (u >= v)
                                        goto lab40;
                                }
                                if (((q.u.B2) % 4) == LIST_TAG) {
                                    y = q.u.B3;
                                    goto lab22;
                                }
                            }
                        }
                    }
                }
            } while (!(z < SCRIPT_SIZE));
        }
        if (large_attempt)
            goto lab40;
        large_attempt = true;
        z = (mem[d].qqqq.u.B2 % 256);
        x = (mem[d].qqqq.u.B3 + (mem[d].qqqq.u.B2 / 256) * 65536L);
    }
 lab40:/*found */ if (f != FONT_BASE) {
        if (!((font_area[f] == OTGR_FONT_FLAG) && (usingOpenType(font_layout_engine[f])))) {       /*736: */

            if (((q.u.B2) % 4) == EXT_TAG) {      /*739: */
                b = new_null_box();
                mem[b].hh.u.B0 = VLIST_NODE;
                r = font_info[exten_base[f] + q.u.B3].qqqq;
                c = r.u.B3;
                u = height_plus_depth(f, c);
                w = 0;
                q = font_info[char_base[f] + effective_char(true, f, c)].qqqq;
                mem[b + 1].cint = font_info[width_base[f] + q.u.B0].cint + font_info[italic_base[f] + (q.u.B2) / 4].cint;
                c = r.u.B2;
                if (c != 0)
                    w = w + height_plus_depth(f, c);
                c = r.u.B1;
                if (c != 0)
                    w = w + height_plus_depth(f, c);
                c = r.u.B0;
                if (c != 0)
                    w = w + height_plus_depth(f, c);
                n = 0;
                if (u > 0)
                    while (w < v) {

                        w = w + u;
                        n++;
                        if (r.u.B1 != 0)
                            w = w + u;
                    }
                c = r.u.B2;
                if (c != 0)
                    stack_into_box(b, f, c);
                c = r.u.B3;
                {
                    register integer for_end;
                    m = 1;
                    for_end = n;
                    if (m <= for_end)
                        do
                            stack_into_box(b, f, c);
                        while (m++ < for_end);
                }
                c = r.u.B1;
                if (c != 0) {
                    stack_into_box(b, f, c);
                    c = r.u.B3;
                    {
                        register integer for_end;
                        m = 1;
                        for_end = n;
                        if (m <= for_end)
                            do
                                stack_into_box(b, f, c);
                            while (m++ < for_end);
                    }
                }
                c = r.u.B0;
                if (c != 0)
                    stack_into_box(b, f, c);
                mem[b + 2].cint = w - mem[b + 3].cint;
            } else
                b = char_box(f, c) /*:736 */ ;
        } else {

            if (ot_assembly_ptr != NULL)
                b = build_opentype_assembly(f, ot_assembly_ptr, v, 0);
            else {

                b = new_null_box();
                mem[b].hh.u.B0 = VLIST_NODE;
                mem[b + 5].hh.v.RH = get_node(GLYPH_NODE_SIZE);
                mem[mem[b + 5].hh.v.RH].hh.u.B0 = WHATSIT_NODE;
                mem[mem[b + 5].hh.v.RH].hh.u.B1 = GLYPH_NODE;
                mem[mem[b + 5].hh.v.RH + 4].qqqq.u.B1 = f;
                mem[mem[b + 5].hh.v.RH + 4].qqqq.u.B2 = c;
                set_native_glyph_metrics(mem[b + 5].hh.v.RH, 1);
                mem[b + 1].cint = mem[mem[b + 5].hh.v.RH + 1].cint;
                mem[b + 3].cint = mem[mem[b + 5].hh.v.RH + 3].cint;
                mem[b + 2].cint = mem[mem[b + 5].hh.v.RH + 2].cint;
            }
        }
    } else {

        b = new_null_box();
        mem[b + 1].cint = DIMENPAR(null_delimiter_space);
    }
    mem[b + 4].cint = half(mem[b + 3].cint - mem[b + 2].cint) - axis_height(s);
    Result = b;
    return Result;
}

int32_t rebox(int32_t b, scaled w)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    internal_font_number f;
    scaled v;
    if ((mem[b + 1].cint != w) && (mem[b + 5].hh.v.RH != MIN_HALFWORD)) {
        if (mem[b].hh.u.B0 == VLIST_NODE)
            b = hpack(b, 0, ADDITIONAL);
        p = mem[b + 5].hh.v.RH;
        if (((p >= hi_mem_min)) && (mem[p].hh.v.RH == MIN_HALFWORD)) {
            f = mem[p].hh.u.B0;
            v = font_info[width_base[f] + font_info[char_base[f] + effective_char(true, f, mem[p].hh.u.B1)].qqqq.u.B0].cint;
            if (v != mem[b + 1].cint)
                mem[p].hh.v.RH = new_kern(mem[b + 1].cint - v);
        }
        free_node(b, BOX_NODE_SIZE);
        b = new_glue(12);
        mem[b].hh.v.RH = p;
        while (mem[p].hh.v.RH != MIN_HALFWORD)
            p = mem[p].hh.v.RH;
        mem[p].hh.v.RH = new_glue(12);
        Result = hpack(b, w, EXACTLY);
    } else {

        mem[b + 1].cint = w;
        Result = b;
    }
    return Result;
}

int32_t math_glue(int32_t g, scaled m)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t p;
    integer n;
    scaled f;
    n = x_over_n(m, 65536L);
    f = tex_remainder;
    if (f < 0) {
        n--;
        f = f + 65536L;
    }
    p = get_node(GLUE_SPEC_SIZE);
    mem[p + 1].cint = mult_and_add(n, mem[g + 1].cint, xn_over_d(mem[g + 1].cint, f, 65536L), MAX_HALFWORD);
    mem[p].hh.u.B0 = mem[g].hh.u.B0;
    if (mem[p].hh.u.B0 == NORMAL)
        mem[p + 2].cint = mult_and_add(n, mem[g + 2].cint, xn_over_d(mem[g + 2].cint, f, 65536L), MAX_HALFWORD);
    else
        mem[p + 2].cint = mem[g + 2].cint;
    mem[p].hh.u.B1 = mem[g].hh.u.B1;
    if (mem[p].hh.u.B1 == NORMAL)
        mem[p + 3].cint = mult_and_add(n, mem[g + 3].cint, xn_over_d(mem[g + 3].cint, f, 65536L), MAX_HALFWORD);
    else
        mem[p + 3].cint = mem[g + 3].cint;
    Result = p;
    return Result;
}

void math_kern(int32_t p, scaled m)
{
    memory_word *mem = zmem; integer n;
    scaled f;
    if (mem[p].hh.u.B1 == MU_GLUE) {
        n = x_over_n(m, 65536L);
        f = tex_remainder;
        if (f < 0) {
            n--;
            f = f + 65536L;
        }
        mem[p + 1].cint = mult_and_add(n, mem[p + 1].cint, xn_over_d(mem[p + 1].cint, f, 65536L), MAX_HALFWORD);
        mem[p].hh.u.B1 = EXPLICIT;
    }
}

void flush_math(void)
{
    memory_word *mem = zmem; flush_node_list(mem[cur_list.head].hh.v.RH);
    flush_node_list(cur_list.aux.cint);
    mem[cur_list.head].hh.v.RH = MIN_HALFWORD;
    cur_list.tail = cur_list.head;
    cur_list.aux.cint = MIN_HALFWORD;
}

int32_t clean_box(int32_t p, small_number s)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t q;
    small_number save_style;
    int32_t x;
    int32_t r;
    switch (mem[p].hh.v.RH) {
    case 1:
        {
            cur_mlist = new_noad();
            mem[cur_mlist + 1] = mem[p];
        }
        break;
    case 2:
        {
            q = mem[p].hh.v.LH;
            goto lab40;
        }
        break;
    case 3:
        cur_mlist = mem[p].hh.v.LH;
        break;
    default:
        {
            q = new_null_box();
            goto lab40;
        }
        break;
    }
    save_style = cur_style;
    cur_style = s;
    mlist_penalties = false;
    mlist_to_hlist();
    q = mem[mem_top - 3].hh.v.RH;
    cur_style = save_style;
    {
        if (cur_style < SCRIPT_STYLE)
            cur_size = TEXT_SIZE;
        else
            cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
        cur_mu = x_over_n(math_quad(cur_size), 18);
    }
 lab40:                        /*found */ if ((q >= hi_mem_min) || (q == MIN_HALFWORD))
        x = hpack(q, 0, ADDITIONAL);
    else if ((mem[q].hh.v.RH == MIN_HALFWORD) && (mem[q].hh.u.B0 <= VLIST_NODE) && (mem[q + 4].cint == 0))
        x = q;
    else
        x = hpack(q, 0, ADDITIONAL);
    q = mem[x + 5].hh.v.RH;
    if ((q >= hi_mem_min)) {
        r = mem[q].hh.v.RH;
        if (r != MIN_HALFWORD) {

            if (mem[r].hh.v.RH == MIN_HALFWORD) {

                if (!(r >= hi_mem_min)) {

                    if (mem[r].hh.u.B0 == KERN_NODE) {
                        free_node(r, MEDIUM_NODE_SIZE);
                        mem[q].hh.v.RH = MIN_HALFWORD;
                    }
                }
            }
        }
    }
    Result = x;
    return Result;
}

void fetch(int32_t a)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;

    cur_c = (unsigned short) mem[a].hh.u.B1;
    cur_f = MATH_FONT((mem[a].hh.u.B0 % 256) + cur_size);
    cur_c = cur_c + (mem[a].hh.u.B0 / 256) * 65536L;
    if (cur_f == FONT_BASE) {   /*749: */
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S());
        }
        print_size(cur_size);
        print_char(32 /*" " */ );
        print_int((mem[a].hh.u.B0 % 256));
        print(S(_is_undefined__character_));
        print(cur_c);
        print_char(41 /*")" */ );
        {
            help_ptr = 4;
            help_line[3] = S(Somewhere_in_the_math_formul/*a just ended, you used the*/);
            help_line[2] = S(stated_character_from_an_und/*efined font family. For example,*/);
            help_line[1] = S(plain_TeX_doesn_t_allow__it_/*or \sl in subscripts. Proceed,*/);
            help_line[0] = S(and_I_ll_try_to_forget_that_/*I needed that character.*/);
        }
        error();
        cur_i = null_character;
        mem[a].hh.v.RH = EMPTY;
    } else if (((font_area[cur_f] == AAT_FONT_FLAG) || (font_area[cur_f] == OTGR_FONT_FLAG))) {
        cur_i = null_character;
    } else {

        if ((cur_c >= font_bc[cur_f]) && (cur_c <= font_ec[cur_f]))
            cur_i = font_info[char_base[cur_f] + cur_c].qqqq;
        else
            cur_i = null_character;
        if (!((cur_i.u.B0 > 0))) {
            char_warning(cur_f, cur_c);
            mem[a].hh.v.RH = EMPTY;
        }
    }
}

void make_over(int32_t q)
{
    memory_word *mem = zmem;
        mem[q + 1].hh.v.LH =
        overbar(clean_box(q + 1, 2 * (cur_style / 2) + 1), 3 * default_rule_thickness(), default_rule_thickness());
    mem[q + 1].hh.v.RH = SUB_BOX;
}

void make_under(int32_t q)
{
    memory_word *mem = zmem; int32_t p, x, y;
    scaled delta;
    x = clean_box(q + 1, cur_style);
    p = new_kern(3 * default_rule_thickness());
    mem[x].hh.v.RH = p;
    mem[p].hh.v.RH = fraction_rule(default_rule_thickness());
    y = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
    delta = mem[y + 3].cint + mem[y + 2].cint + default_rule_thickness();
    mem[y + 3].cint = mem[x + 3].cint;
    mem[y + 2].cint = delta - mem[y + 3].cint;
    mem[q + 1].hh.v.LH = y;
    mem[q + 1].hh.v.RH = SUB_BOX;
}

void make_vcenter(int32_t q)
{
    memory_word *mem = zmem; int32_t v;
    scaled delta;
    v = mem[q + 1].hh.v.LH;
    if (mem[v].hh.u.B0 != VLIST_NODE)
        confusion(S(vcenter));
    delta = mem[v + 3].cint + mem[v + 2].cint;
    mem[v + 3].cint = axis_height(cur_size) + half(delta);
    mem[v + 2].cint = delta - mem[v + 3].cint;
}

void make_radical(int32_t q)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t x, y;
    internal_font_number f;
    scaled rule_thickness;
    scaled delta, clr;

    f = MATH_FONT((mem[q + 4].qqqq.u.B0 % 256) + cur_size);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f]))))
        rule_thickness = get_ot_math_constant(f, RADICALRULETHICKNESS);
    else
        rule_thickness = default_rule_thickness();
    x = clean_box(q + 1, 2 * (cur_style / 2) + 1);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f])))) {
        if (cur_style < TEXT_STYLE)
            clr = get_ot_math_constant(f, RADICALDISPLAYSTYLEVERTICALGAP);
        else
            clr = get_ot_math_constant(f, RADICALVERTICALGAP);
    } else {

        if (cur_style < TEXT_STYLE)
            clr = rule_thickness + (abs(math_x_height(cur_size)) / 4);
        else {

            clr = rule_thickness;
            clr = clr + (abs(clr) / 4);
        }
    }
    y = var_delimiter(q + 4, cur_size, mem[x + 3].cint + mem[x + 2].cint + clr + rule_thickness);
    if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f])))) {
        mem[y + 2].cint = mem[y + 3].cint + mem[y + 2].cint - rule_thickness;
        mem[y + 3].cint = rule_thickness;
    }
    delta = mem[y + 2].cint - (mem[x + 3].cint + mem[x + 2].cint + clr);
    if (delta > 0)
        clr = clr + half(delta);
    mem[y + 4].cint = -(integer) (mem[x + 3].cint + clr);
    mem[y].hh.v.RH = overbar(x, clr, mem[y + 3].cint);
    mem[q + 1].hh.v.LH = hpack(y, 0, ADDITIONAL);
    mem[q + 1].hh.v.RH = SUB_BOX;
}

scaled compute_ot_math_accent_pos(int32_t p)
{
    register scaled Result;
    memory_word *mem = zmem; int32_t q, r;
    scaled s, g;
    if ((mem[p + 1].hh.v.RH == MATH_CHAR)) {
        fetch(p + 1);
        q = new_native_character(cur_f, cur_c);
        g = get_native_glyph(q, 0);
        s = get_ot_math_accent_pos(cur_f, g);
    } else {

        if ((mem[p + 1].hh.v.RH == SUB_MLIST)) {
            r = mem[p + 1].hh.v.LH;
            if ((r != MIN_HALFWORD) && (mem[r].hh.u.B0 == ACCENT_NOAD))
                s = compute_ot_math_accent_pos(r);
            else
                s = TEX_INFINITY;
        } else
            s = TEX_INFINITY;
    }
    Result = s;
    return Result;
}

void make_math_accent(int32_t q)
{
    memory_word *mem = zmem; int32_t p, x, y;
    integer a;
    integer c, g;
    internal_font_number f;
    four_quarters i;
    scaled s, sa;
    scaled h;
    scaled delta;
    scaled w, w2;
    void *ot_assembly_ptr;
    fetch(q + 4);
    x = MIN_HALFWORD;
    if (((font_area[cur_f] == AAT_FONT_FLAG) || (font_area[cur_f] == OTGR_FONT_FLAG))) {
        c = cur_c;
        f = cur_f;
        if (!((mem[q].hh.u.B1 == BOTTOM_ACC) || (mem[q].hh.u.B1 == (BOTTOM_ACC + 1))))
            s = compute_ot_math_accent_pos(q);
        else
            s = 0;
        x = clean_box(q + 1, 2 * (cur_style / 2) + 1);
        w = mem[x + 1].cint;
        h = mem[x + 3].cint;
    } else if ((cur_i.u.B0 > 0)) {
        i = cur_i;
        c = cur_c;
        f = cur_f;
        s = 0;
        if (mem[q + 1].hh.v.RH == MATH_CHAR) {
            fetch(q + 1);
            if (((cur_i.u.B2) % 4) == LIG_TAG) {
                a = lig_kern_base[cur_f] + cur_i.u.B3;
                cur_i = font_info[a].qqqq;
                if (cur_i.u.B0 > 128) {
                    a = lig_kern_base[cur_f] + 256 * cur_i.u.B2 + cur_i.u.B3 + 32768L - 256 * (128);
                    cur_i = font_info[a].qqqq;
                }
                while (true) {

                    if (cur_i.u.B1 == skew_char[cur_f]) {
                        if (cur_i.u.B2 >= 128) {

                            if (cur_i.u.B0 <= 128)
                                s = font_info[kern_base[cur_f] + 256 * cur_i.u.B2 + cur_i.u.B3].cint;
                        }
                        goto lab31;
                    }
                    if (cur_i.u.B0 >= 128)
                        goto lab31;
                    a = a + cur_i.u.B0 + 1;
                    cur_i = font_info[a].qqqq;
                }
            }
        }
 lab31:                        /*done1 *//*:768 */ ;
        x = clean_box(q + 1, 2 * (cur_style / 2) + 1);
        w = mem[x + 1].cint;
        h = mem[x + 3].cint;
        while (true) {

            if (((i.u.B2) % 4) != LIST_TAG)
                goto done;
            y = i.u.B3;
            i = font_info[char_base[f] + y].qqqq;
            if (!(i.u.B0 > 0))
                goto done;
            if (font_info[width_base[f] + i.u.B0].cint > w)
                goto done;
            c = y;
        }
        /*:767*/
    done:
        ;
    }
    if (x != MIN_HALFWORD) {
        if (((font_area[f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[f])))) {

            if (((mem[q].hh.u.B1 == BOTTOM_ACC) || (mem[q].hh.u.B1 == (BOTTOM_ACC + 1))))
                delta = 0;
            else if (h < get_ot_math_constant(f, ACCENTBASEHEIGHT))
                delta = h;
            else
                delta = get_ot_math_constant(f, ACCENTBASEHEIGHT);
        } else if (h < font_info[X_HEIGHT_CODE + param_base[f]].cint)
            delta = h;
        else
            delta = font_info[X_HEIGHT_CODE + param_base[f]].cint;
        if ((mem[q + 2].hh.v.RH != EMPTY) || (mem[q + 3].hh.v.RH != EMPTY)) {

            if (mem[q + 1].hh.v.RH == MATH_CHAR) {      /*769: */
                flush_node_list(x);
                x = new_noad();
                mem[x + 1] = mem[q + 1];
                mem[x + 2] = mem[q + 2];
                mem[x + 3] = mem[q + 3];
                mem[q + 2].hh = empty;
                mem[q + 3].hh = empty;
                mem[q + 1].hh.v.RH = SUB_MLIST;
                mem[q + 1].hh.v.LH = x;
                x = clean_box(q + 1, cur_style);
                delta = delta + mem[x + 3].cint - h;
                h = mem[x + 3].cint;
            }
        }
        y = char_box(f, c);
        if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))) {
            p = get_node(GLYPH_NODE_SIZE);
            mem[p].hh.u.B0 = WHATSIT_NODE;
            mem[p].hh.u.B1 = GLYPH_NODE;
            mem[p + 4].qqqq.u.B1 = f;
            mem[p + 4].qqqq.u.B2 = get_native_glyph(mem[y + 5].hh.v.RH, 0);
            set_native_glyph_metrics(p, 1);
            free_node(mem[y + 5].hh.v.RH, mem[mem[y + 5].hh.v.RH + 4].qqqq.u.B0);
            mem[y + 5].hh.v.RH = p;
            if (odd(mem[q].hh.u.B1))
                set_native_glyph_metrics(p, 1);
            else {

                c = mem[p + 4].qqqq.u.B2;
                a = 0;
                do {
                    g = get_ot_math_variant(f, c, a, &w2, 1);
                    if ((w2 > 0) && (w2 <= w)) {
                        mem[p + 4].qqqq.u.B2 = g;
                        set_native_glyph_metrics(p, 1);
                        a++;
                    }
                } while (!((w2 < 0) || (w2 >= w)));
                if ((w2 < 0)) {
                    ot_assembly_ptr = get_ot_assembly_ptr(f, c, 1);
                    if (ot_assembly_ptr != NULL) {
                        free_node(p, GLYPH_NODE_SIZE);
                        p = build_opentype_assembly(f, ot_assembly_ptr, w, 1);
                        mem[y + 5].hh.v.RH = p;
                        goto lab40;
                    }
                } else
                    set_native_glyph_metrics(p, 1);
            }
 lab40:                        /*found */ mem[y + 1].cint = mem[p + 1].cint;
            mem[y + 3].cint = mem[p + 3].cint;
            mem[y + 2].cint = mem[p + 2].cint;
            if (((mem[q].hh.u.B1 == BOTTOM_ACC) || (mem[q].hh.u.B1 == (BOTTOM_ACC + 1)))) {
                if (mem[y + 3].cint < 0)
                    mem[y + 3].cint = 0;
            } else if (mem[y + 2].cint < 0)
                mem[y + 2].cint = 0;
            if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
                  && (mem[p].hh.u.B1 == GLYPH_NODE)))) {
                sa = get_ot_math_accent_pos(f, mem[p + 4].qqqq.u.B2);
                if (sa == TEX_INFINITY)
                    sa = half(mem[y + 1].cint);
            } else
                sa = half(mem[y + 1].cint);
            if (((mem[q].hh.u.B1 == BOTTOM_ACC) || (mem[q].hh.u.B1 == (BOTTOM_ACC + 1))) || (s == TEX_INFINITY))
                s = half(w);
            mem[y + 4].cint = s - sa;
        } else
            mem[y + 4].cint = s + half(w - mem[y + 1].cint);
        mem[y + 1].cint = 0;
        if (((mem[q].hh.u.B1 == BOTTOM_ACC) || (mem[q].hh.u.B1 == (BOTTOM_ACC + 1)))) {
            mem[x].hh.v.RH = y;
            y = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
            mem[y + 4].cint = -(integer) (h - mem[y + 3].cint);
        } else {

            p = new_kern(-(integer) delta);
            mem[p].hh.v.RH = x;
            mem[y].hh.v.RH = p;
            y = vpackage(y, 0, ADDITIONAL, MAX_HALFWORD);
            if (mem[y + 3].cint < h) {  /*765: */
                p = new_kern(h - mem[y + 3].cint);
                mem[p].hh.v.RH = mem[y + 5].hh.v.RH;
                mem[y + 5].hh.v.RH = p;
                mem[y + 3].cint = h;
            }
        }
        mem[y + 1].cint = mem[x + 1].cint;
        mem[q + 1].hh.v.LH = y;
        mem[q + 1].hh.v.RH = SUB_BOX;
    }
}

void make_fraction(int32_t q)
{
    memory_word *mem = zmem; int32_t p, v, x, y, z;
    scaled delta, delta1, delta2, shift_up, shift_down, clr;
    if (mem[q + 1].cint == 1073741824L)
        mem[q + 1].cint = default_rule_thickness();
    x = clean_box(q + 2, cur_style + 2 - 2 * (cur_style / 6));
    z = clean_box(q + 3, 2 * (cur_style / 2) + 3 - 2 * (cur_style / 6));
    if (mem[x + 1].cint < mem[z + 1].cint)
        x = rebox(x, mem[z + 1].cint);
    else
        z = rebox(z, mem[x + 1].cint);
    if (cur_style < TEXT_STYLE) {
        shift_up = num1(cur_size);
        shift_down = denom1(cur_size);
    } else {

        shift_down = denom2(cur_size);
        if (mem[q + 1].cint != 0)
            shift_up = num2(cur_size);
        else
            shift_up = num3(cur_size);
    }
    if (mem[q + 1].cint == 0) { /*772: */
        if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
            if (cur_style < TEXT_STYLE)
                clr = get_ot_math_constant(cur_f, STACKDISPLAYSTYLEGAPMIN);
            else
                clr = get_ot_math_constant(cur_f, STACKGAPMIN);
        } else {

            if (cur_style < TEXT_STYLE)
                clr = 7 * default_rule_thickness();
            else
                clr = 3 * default_rule_thickness();
        }
        delta = half(clr - ((shift_up - mem[x + 2].cint) - (mem[z + 3].cint - shift_down)));
        if (delta > 0) {
            shift_up = shift_up + delta;
            shift_down = shift_down + delta;
        }
    } else {                    /*773: */

        if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
            delta = half(mem[q + 1].cint);
            if (cur_style < TEXT_STYLE)
                clr = get_ot_math_constant(cur_f, FRACTIONNUMDISPLAYSTYLEGAPMIN);
            else
                clr = get_ot_math_constant(cur_f, FRACTIONNUMERATORGAPMIN);
            delta1 = clr - ((shift_up - mem[x + 2].cint) - (axis_height(cur_size) + delta));
            if (cur_style < TEXT_STYLE)
                clr = get_ot_math_constant(cur_f, FRACTIONDENOMDISPLAYSTYLEGAPMIN);
            else
                clr = get_ot_math_constant(cur_f, FRACTIONDENOMINATORGAPMIN);
            delta2 = clr - ((axis_height(cur_size) - delta) - (mem[z + 3].cint - shift_down));
        } else {

            if (cur_style < TEXT_STYLE)
                clr = 3 * mem[q + 1].cint;
            else
                clr = mem[q + 1].cint;
            delta = half(mem[q + 1].cint);
            delta1 = clr - ((shift_up - mem[x + 2].cint) - (axis_height(cur_size) + delta));
            delta2 = clr - ((axis_height(cur_size) - delta) - (mem[z + 3].cint - shift_down));
        }
        if (delta1 > 0)
            shift_up = shift_up + delta1;
        if (delta2 > 0)
            shift_down = shift_down + delta2;
    }
    v = new_null_box();
    mem[v].hh.u.B0 = VLIST_NODE;
    mem[v + 3].cint = shift_up + mem[x + 3].cint;
    mem[v + 2].cint = mem[z + 2].cint + shift_down;
    mem[v + 1].cint = mem[x + 1].cint;
    if (mem[q + 1].cint == 0) {
        p = new_kern((shift_up - mem[x + 2].cint) - (mem[z + 3].cint - shift_down));
        mem[p].hh.v.RH = z;
    } else {

        y = fraction_rule(mem[q + 1].cint);
        p = new_kern((axis_height(cur_size) - delta) - (mem[z + 3].cint - shift_down));
        mem[y].hh.v.RH = p;
        mem[p].hh.v.RH = z;
        p = new_kern((shift_up - mem[x + 2].cint) - (axis_height(cur_size) + delta));
        mem[p].hh.v.RH = y;
    }
    mem[x].hh.v.RH = p;
    mem[v + 5].hh.v.RH = /*:774 */ x;
    if (cur_style < TEXT_STYLE)
        delta = delim1(cur_size);
    else
        delta = delim2(cur_size);
    x = var_delimiter(q + 4, cur_size, delta);
    mem[x].hh.v.RH = v;
    z = var_delimiter(q + 5, cur_size, delta);
    mem[v].hh.v.RH = z;
    mem[q + 1].cint = hpack(x, 0, ADDITIONAL) /*:775 */ ;
}

scaled make_op(int32_t q)
{
    register scaled Result;
    memory_word *mem = zmem; scaled delta;
    int32_t p, v, x, y, z;
    uint16_t c;
    four_quarters i;
    scaled shift_up, shift_down;
    scaled h1, h2;
    integer n, g;
    void *ot_assembly_ptr;
    internal_font_number save_f;
    if ((mem[q].hh.u.B1 == NORMAL) && (cur_style < TEXT_STYLE))
        mem[q].hh.u.B1 = LIMITS;
    delta = 0;
    if (mem[q + 1].hh.v.RH == MATH_CHAR) {
        fetch(q + 1);
        if (!((font_area[cur_f] == OTGR_FONT_FLAG) && (usingOpenType(font_layout_engine[cur_f])))) {
            if ((cur_style < TEXT_STYLE) && (((cur_i.u.B2) % 4) == LIST_TAG)) {
                c = cur_i.u.B3;
                i = font_info[char_base[cur_f] + c].qqqq;
                if ((i.u.B0 > 0)) {
                    cur_c = c;
                    cur_i = i;
                    mem[q + 1].hh.u.B1 = c;
                }
            }
            delta = font_info[italic_base[cur_f] + (cur_i.u.B2) / 4].cint;
        }
        x = clean_box(q + 1, cur_style);
        if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
            p = mem[x + 5].hh.v.RH;
            if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
                  && (mem[p].hh.u.B1 == GLYPH_NODE)))) {
                if (cur_style < TEXT_STYLE) {
                    h1 = get_ot_math_constant(cur_f, DISPLAYOPERATORMINHEIGHT);
                    if (h1 < (mem[p + 3].cint + mem[p + 2].cint) * 5 / ((double)4))
                        h1 = (mem[p + 3].cint + mem[p + 2].cint) * 5 / ((double)4);
                    c = mem[p + 4].qqqq.u.B2;
                    n = 0;
                    do {
                        g = get_ot_math_variant(cur_f, c, n, &h2, 0);
                        if (h2 > 0) {
                            mem[p + 4].qqqq.u.B2 = g;
                            set_native_glyph_metrics(p, 1);
                        }
                        n++;
                    } while (!((h2 < 0) || (h2 >= h1)));
                    if ((h2 < 0)) {
                        ot_assembly_ptr = get_ot_assembly_ptr(cur_f, c, 0);
                        if (ot_assembly_ptr != NULL) {
                            free_node(p, GLYPH_NODE_SIZE);
                            p = build_opentype_assembly(cur_f, ot_assembly_ptr, h1, 0);
                            mem[x + 5].hh.v.RH = p;
                            delta = 0;
                            goto lab40;
                        }
                    } else
                        set_native_glyph_metrics(p, 1);
                }
                delta = get_ot_math_ital_corr(cur_f, mem[p + 4].qqqq.u.B2);
 lab40:                        /*found */ mem[x + 1].cint = mem[p + 1].cint;
                mem[x + 3].cint = mem[p + 3].cint;
                mem[x + 2].cint = mem[p + 2].cint;
            }
        }
        if ((mem[q + 3].hh.v.RH != EMPTY) && (mem[q].hh.u.B1 != LIMITS))
            mem[x + 1].cint = mem[x + 1].cint - delta;
        mem[x + 4].cint = half(mem[x + 3].cint - mem[x + 2].cint) - axis_height(cur_size);
        mem[q + 1].hh.v.RH = SUB_BOX;
        mem[q + 1].hh.v.LH = x;
    }
    save_f = cur_f;
    if (mem[q].hh.u.B1 == LIMITS) {       /*777: */
        x = clean_box(q + 2, 2 * (cur_style / 4) + 4 + (cur_style % 2));
        y = clean_box(q + 1, cur_style);
        z = clean_box(q + 3, 2 * (cur_style / 4) + 5);
        v = new_null_box();
        mem[v].hh.u.B0 = VLIST_NODE;
        mem[v + 1].cint = mem[y + 1].cint;
        if (mem[x + 1].cint > mem[v + 1].cint)
            mem[v + 1].cint = mem[x + 1].cint;
        if (mem[z + 1].cint > mem[v + 1].cint)
            mem[v + 1].cint = mem[z + 1].cint;
        x = rebox(x, mem[v + 1].cint);
        y = rebox(y, mem[v + 1].cint);
        z = rebox(z, mem[v + 1].cint);
        mem[x + 4].cint = half(delta);
        mem[z + 4].cint = -(integer) mem[x + 4].cint;
        mem[v + 3].cint = mem[y + 3].cint;
        mem[v + 2].cint = mem[y + 2].cint;
        cur_f = save_f;
        if (mem[q + 2].hh.v.RH == EMPTY) {
            free_node(x, BOX_NODE_SIZE);
            mem[v + 5].hh.v.RH = y;
        } else {

            shift_up = big_op_spacing3() - mem[x + 2].cint;
            if (shift_up < big_op_spacing1())
                shift_up = big_op_spacing1();
            p = new_kern(shift_up);
            mem[p].hh.v.RH = y;
            mem[x].hh.v.RH = p;
            p = new_kern(big_op_spacing5());
            mem[p].hh.v.RH = x;
            mem[v + 5].hh.v.RH = p;
            mem[v + 3].cint = mem[v + 3].cint + big_op_spacing5() + mem[x + 3].cint + mem[x + 2].cint + shift_up;
        }
        if (mem[q + 3].hh.v.RH == EMPTY)
            free_node(z, BOX_NODE_SIZE);
        else {

            shift_down = big_op_spacing4() - mem[z + 3].cint;
            if (shift_down < big_op_spacing2())
                shift_down = big_op_spacing2();
            p = new_kern(shift_down);
            mem[y].hh.v.RH = p;
            mem[p].hh.v.RH = z;
            p = new_kern(big_op_spacing5());
            mem[z].hh.v.RH = p;
            mem[v + 2].cint = mem[v + 2].cint + big_op_spacing5() + mem[z + 3].cint + mem[z + 2].cint + shift_down;
        }
        mem[q + 1].cint = v;
    }
    Result = delta;
    return Result;
}

void make_ord(int32_t q)
{
    memory_word *mem = zmem; integer a;
    int32_t p, r;
 lab20:/*restart */ if (mem[q + 3].hh.v.RH == EMPTY) {

        if (mem[q + 2].hh.v.RH == EMPTY) {

            if (mem[q + 1].hh.v.RH == MATH_CHAR) {
                p = mem[q].hh.v.RH;
                if (p != MIN_HALFWORD) {

                    if ((mem[p].hh.u.B0 >= ORD_NOAD) && (mem[p].hh.u.B0 <= PUNCT_NOAD)) {

                        if (mem[p + 1].hh.v.RH == MATH_CHAR) {

                            if ((mem[p + 1].hh.u.B0 % 256) == (mem[q + 1].hh.u.B0 % 256)) {
                                mem[q + 1].hh.v.RH = MATH_TEXT_CHAR;
                                fetch(q + 1);
                                if (((cur_i.u.B2) % 4) == LIG_TAG) {
                                    a = lig_kern_base[cur_f] + cur_i.u.B3;
                                    cur_c = mem[p + 1].hh.u.B1;
                                    cur_i = font_info[a].qqqq;
                                    if (cur_i.u.B0 > 128) {
                                        a = lig_kern_base[cur_f] + 256 * cur_i.u.B2 + cur_i.u.B3 + 32768L - 256 * (128);
                                        cur_i = font_info[a].qqqq;
                                    }
                                    while (true) {

                                        if (cur_i.u.B1 == cur_c) {

                                            if (cur_i.u.B0 <= 128) {

                                                if (cur_i.u.B2 >= 128) {
                                                    p = new_kern(font_info
                                                                 [kern_base[cur_f] + 256 * cur_i.u.B2 + cur_i.u.B3].cint);
                                                    mem[p].hh.v.RH = mem[q].hh.v.RH;
                                                    mem[q].hh.v.RH = p;
                                                    return;
                                                } else {
                                                    switch (cur_i.u.B2) {
                                                    case 1:
                                                    case 5:
                                                        mem[q + 1].hh.u.B1 = cur_i.u.B3;
                                                        break;
                                                    case 2:
                                                    case 6:
                                                        mem[p + 1].hh.u.B1 = cur_i.u.B3;
                                                        break;
                                                    case 3:
                                                    case 7:
                                                    case 11:
                                                        {
                                                            r = new_noad();
                                                            mem[r + 1].hh.u.B1 = cur_i.u.B3;
                                                            mem[r + 1].hh.u.B0 = (mem[q + 1].hh.u.B0 % 256);
                                                            mem[q].hh.v.RH = r;
                                                            mem[r].hh.v.RH = p;
                                                            if (cur_i.u.B2 < 11)
                                                                mem[r + 1].hh.v.RH = MATH_CHAR;
                                                            else
                                                                mem[r + 1].hh.v.RH = MATH_TEXT_CHAR;
                                                        }
                                                        break;
                                                    default:
                                                        {
                                                            mem[q].hh.v.RH = mem[p].hh.v.RH;
                                                            mem[q + 1].hh.u.B1 = cur_i.u.B3;
                                                            mem[q + 3] = mem[p + 3];
                                                            mem[q + 2] = mem[p + 2];
                                                            free_node(p, NOAD_SIZE);
                                                        }
                                                        break;
                                                    }
                                                    if (cur_i.u.B2 > 3)
                                                        return;
                                                    mem[q + 1].hh.v.RH = MATH_CHAR;
                                                    goto lab20;
                                                }
                                            }
                                        }
                                        if (cur_i.u.B0 >= 128)
                                            return;
                                        a = a + cur_i.u.B0 + 1;
                                        cur_i = font_info[a].qqqq;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

int32_t attach_hkern_to_new_hlist(int32_t q, scaled delta)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t y, z;
    z = new_kern(delta);
    if (mem[q + 1].cint == MIN_HALFWORD)
        mem[q + 1].cint = z;
    else {

        y = mem[q + 1].cint;
        while (mem[y].hh.v.RH != MIN_HALFWORD)
            y = mem[y].hh.v.RH;
        mem[y].hh.v.RH = z;
    }
    Result = mem[q + 1].cint;
    return Result;
}

void make_scripts(int32_t q, scaled delta)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p, x, y, z;
    scaled shift_up, shift_down, clr, sub_kern, sup_kern;
    int32_t script_c;
    uint16_t script_g;
    internal_font_number script_f;
    integer t;
    internal_font_number save_f;

    p = mem[q + 1].cint;
    script_c = MIN_HALFWORD;
    script_g = 0;
    script_f = 0;
    sup_kern = 0;
    sub_kern = 0;
    if ((p >= hi_mem_min)
        ||
        (((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
          && (mem[p].hh.u.B1 == GLYPH_NODE)))) {
        shift_up = 0;
        shift_down = 0;
    } else {

        z = hpack(p, 0, ADDITIONAL);
        if (cur_style < SCRIPT_STYLE)
            t = SCRIPT_SIZE;
        else
            t = SCRIPT_SCRIPT_SIZE;
        shift_up = mem[z + 3].cint - sup_drop(t);
        shift_down = mem[z + 2].cint + sub_drop(t);
        free_node(z, BOX_NODE_SIZE);
    }
    if (mem[q + 2].hh.v.RH == EMPTY) {  /*784: */
        save_f = cur_f;
        x = clean_box(q + 3, 2 * (cur_style / 4) + 5);
        cur_f = save_f;
        mem[x + 1].cint = mem[x + 1].cint + DIMENPAR(script_space);
        if (shift_down < sub1(cur_size))
            shift_down = sub1(cur_size);
        if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f]))))
            clr = mem[x + 3].cint - get_ot_math_constant(cur_f, SUBSCRIPTTOPMAX);
        else
            clr = mem[x + 3].cint - (abs(math_x_height(cur_size) * 4) / 5);
        if (shift_down < clr)
            shift_down = clr;
        mem[x + 4].cint = shift_down;
        if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {   /*787: */
            if (mem[q + 3].hh.v.RH == MATH_CHAR) {
                save_f = cur_f;
                fetch(q + 3);
                if (((font_area[cur_f] == OTGR_FONT_FLAG)
                     && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                    script_c = new_native_character(cur_f, cur_c);
                    script_g = get_native_glyph(script_c, 0);
                    script_f = cur_f;
                } else {

                    script_g = 0;
                    script_f = 0;
                }
                cur_f = save_f;
            }
            if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
                  && (mem[p].hh.u.B1 == GLYPH_NODE))))
                sub_kern =
                    get_ot_math_kern(mem[p + 4].qqqq.u.B1, mem[p + 4].qqqq.u.B2, script_f, script_g, SUB_CMD,
                                     shift_down);
            if (sub_kern != 0)
                p = attach_hkern_to_new_hlist(q, sub_kern);
        }
    } else {

        {
            save_f = cur_f;
            x = clean_box(q + 2, 2 * (cur_style / 4) + 4 + (cur_style % 2));
            cur_f = save_f;
            mem[x + 1].cint = mem[x + 1].cint + DIMENPAR(script_space);
            if (odd(cur_style))
                clr = sup3(cur_size);
            else if (cur_style < TEXT_STYLE)
                clr = sup1(cur_size);
            else
                clr = sup2(cur_size);
            if (shift_up < clr)
                shift_up = clr;
            if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f]))))
                clr = mem[x + 2].cint + get_ot_math_constant(cur_f, SUPERSCRIPTBOTTOMMIN);
            else
                clr = mem[x + 2].cint + (abs(math_x_height(cur_size)) / 4);
            if (shift_up < clr)
                shift_up = clr;
            if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {       /*788: */
                if (mem[q + 2].hh.v.RH == MATH_CHAR) {
                    save_f = cur_f;
                    fetch(q + 2);
                    if (((font_area[cur_f] == OTGR_FONT_FLAG)
                         && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                        script_c = new_native_character(cur_f, cur_c);
                        script_g = get_native_glyph(script_c, 0);
                        script_f = cur_f;
                    } else {

                        script_g = 0;
                        script_f = 0;
                    }
                    cur_f = save_f;
                }
                if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
                      && (mem[p].hh.u.B1 == GLYPH_NODE))))
                    sup_kern =
                        get_ot_math_kern(mem[p + 4].qqqq.u.B1, mem[p + 4].qqqq.u.B2, script_f, script_g, SUP_CMD,
                                         shift_up);
                if ((sup_kern != 0) && (mem[q + 3].hh.v.RH == EMPTY))
                    p = attach_hkern_to_new_hlist(q, sup_kern);
            }
        }
        if (mem[q + 3].hh.v.RH == EMPTY)
            mem[x + 4].cint = -(integer) shift_up;
        else {                  /*786: */

            save_f = cur_f;
            y = clean_box(q + 3, 2 * (cur_style / 4) + 5);
            cur_f = save_f;
            mem[y + 1].cint = mem[y + 1].cint + DIMENPAR(script_space);
            if (shift_down < sub2(cur_size))
                shift_down = sub2(cur_size);
            if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f]))))
                clr =
                    get_ot_math_constant(cur_f,
                                         SUBSUPERSCRIPTGAPMIN) - ((shift_up - mem[x + 2].cint) -
                                                                           (mem[y + 3].cint - shift_down));
            else
                clr = 4 * default_rule_thickness() - ((shift_up - mem[x + 2].cint) - (mem[y + 3].cint - shift_down));
            if (clr > 0) {
                shift_down = shift_down + clr;
                if (((font_area[cur_f] == OTGR_FONT_FLAG)
                     && (isOpenTypeMathFont(font_layout_engine[cur_f]))))
                    clr =
                        get_ot_math_constant(cur_f,
                                             SUPERSCRIPTBOTTOMMAXWITHSUBSCRIPT) - (shift_up - mem[x + 2].cint);
                else
                    clr = (abs(math_x_height(cur_size) * 4) / 5) - (shift_up - mem[x + 2].cint);
                if (clr > 0) {
                    shift_up = shift_up + clr;
                    shift_down = shift_down - clr;
                }
            }
            if (((font_area[cur_f] == OTGR_FONT_FLAG) && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                {
                    if (mem[q + 3].hh.v.RH == MATH_CHAR) {
                        save_f = cur_f;
                        fetch(q + 3);
                        if (((font_area[cur_f] == OTGR_FONT_FLAG)
                             && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                            script_c = new_native_character(cur_f, cur_c);
                            script_g = get_native_glyph(script_c, 0);
                            script_f = cur_f;
                        } else {

                            script_g = 0;
                            script_f = 0;
                        }
                        cur_f = save_f;
                    }
                    if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
                          && (mem[p].hh.u.B1 == GLYPH_NODE))))
                        sub_kern =
                            get_ot_math_kern(mem[p + 4].qqqq.u.B1, mem[p + 4].qqqq.u.B2, script_f, script_g,
                                             SUB_CMD, shift_down);
                    if (sub_kern != 0)
                        p = attach_hkern_to_new_hlist(q, sub_kern);
                }
                {
                    if (mem[q + 2].hh.v.RH == MATH_CHAR) {
                        save_f = cur_f;
                        fetch(q + 2);
                        if (((font_area[cur_f] == OTGR_FONT_FLAG)
                             && (isOpenTypeMathFont(font_layout_engine[cur_f])))) {
                            script_c = new_native_character(cur_f, cur_c);
                            script_g = get_native_glyph(script_c, 0);
                            script_f = cur_f;
                        } else {

                            script_g = 0;
                            script_f = 0;
                        }
                        cur_f = save_f;
                    }
                    if ((((p) != MIN_HALFWORD && (!(p >= hi_mem_min)) && (mem[p].hh.u.B0 == WHATSIT_NODE)
                          && (mem[p].hh.u.B1 == GLYPH_NODE))))
                        sup_kern =
                            get_ot_math_kern(mem[p + 4].qqqq.u.B1, mem[p + 4].qqqq.u.B2, script_f, script_g,
                                             SUP_CMD, shift_up);
                    if ((sup_kern != 0) && (mem[q + 3].hh.v.RH == EMPTY))
                        p = attach_hkern_to_new_hlist(q, sup_kern);
                }
            }
            mem[x + 4].cint = sup_kern + delta - sub_kern;
            p = new_kern((shift_up - mem[x + 2].cint) - (mem[y + 3].cint - shift_down));
            mem[x].hh.v.RH = p;
            mem[p].hh.v.RH = y;
            x = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
            mem[x + 4].cint = shift_down;
        }
    }
    if (mem[q + 1].cint == MIN_HALFWORD)
        mem[q + 1].cint = x;
    else {

        p = mem[q + 1].cint;
        while (mem[p].hh.v.RH != MIN_HALFWORD)
            p = mem[p].hh.v.RH;
        mem[p].hh.v.RH = x;
    }
}

small_number make_left_right(int32_t q, small_number style, scaled max_d, scaled max_h)
{
    CACHE_THE_EQTB;
    register small_number Result;
    memory_word *mem = zmem;
    scaled delta, delta1, delta2;

    cur_style = style;
    {
        if (cur_style < SCRIPT_STYLE)
            cur_size = TEXT_SIZE;
        else
            cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
        cur_mu = x_over_n(math_quad(cur_size), 18);
    }
    delta2 = max_d + axis_height(cur_size);
    delta1 = max_h + max_d - delta2;
    if (delta2 > delta1)
        delta1 = delta2;
    delta = (delta1 / 500) * INTPAR(delimiter_factor);
    delta2 = delta1 + delta1 - DIMENPAR(delimiter_shortfall);
    if (delta < delta2)
        delta = delta2;
    mem[q + 1].cint = var_delimiter(q + 1, cur_size, delta);
    Result = mem[q].hh.u.B0 - ((LEFT_NOAD - 20));
    return Result;
}

void mlist_to_hlist(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t mlist;
    boolean penalties;
    small_number style;
    small_number save_style;
    int32_t q;
    int32_t r;
    small_number r_type;
    small_number t;
    int32_t p, x, y, z;
    integer pen;
    small_number s;
    scaled max_h, max_d;
    scaled delta;

    mlist = cur_mlist;
    penalties = mlist_penalties;
    style = cur_style;
    q = mlist;
    r = MIN_HALFWORD;
    r_type = OP_NOAD;
    max_h = 0;
    max_d = 0;
    {
        if (cur_style < SCRIPT_STYLE)
            cur_size = TEXT_SIZE;
        else
            cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
        cur_mu = x_over_n(math_quad(cur_size), 18);
    }
    while (q != MIN_HALFWORD) {  /*753: */

 lab21:                        /*reswitch */ delta = 0;
        switch (mem[q].hh.u.B0) {
        case 18:
            switch (r_type) {
            case 18:
            case 17:
            case 19:
            case 20:
            case 22:
            case 30:
                {
                    mem[q].hh.u.B0 = ORD_NOAD;
                    goto lab21;
                }
                break;
            default:
                ;
                break;
            }
            break;
        case 19:
        case 21:
        case 22:
        case 31:
            {
                if (r_type == BIN_NOAD)
                    mem[r].hh.u.B0 = 16 /*ord_noad *//*:755 */ ;
                if (mem[q].hh.u.B0 == RIGHT_NOAD)
                    goto lab80;
            }
            break;
        case 30:
            goto lab80;
            break;
        case 25:
            {
                make_fraction(q);
                goto lab82;
            }
            break;
        case 17:
            {
                delta = make_op(q);
                if (mem[q].hh.u.B1 == LIMITS)
                    goto lab82;
            }
            break;
        case 16:
            make_ord(q);
            break;
        case 20:
        case 23:
            ;
            break;
        case 24:
            make_radical(q);
            break;
        case 27:
            make_over(q);
            break;
        case 26:
            make_under(q);
            break;
        case 28:
            make_math_accent(q);
            break;
        case 29:
            make_vcenter(q);
            break;
        case 14:
            {
                cur_style = mem[q].hh.u.B1;
                {
                    if (cur_style < SCRIPT_STYLE)
                        cur_size = TEXT_SIZE;
                    else
                        cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
                    cur_mu = x_over_n(math_quad(cur_size), 18);
                }
                goto lab81;
            }
            break;
        case 15:
            {
                switch (cur_style / 2) {
                case 0:
                    {
                        p = mem[q + 1].hh.v.LH;
                        mem[q + 1].hh.v.LH = MIN_HALFWORD;
                    }
                    break;
                case 1:
                    {
                        p = mem[q + 1].hh.v.RH;
                        mem[q + 1].hh.v.RH = MIN_HALFWORD;
                    }
                    break;
                case 2:
                    {
                        p = mem[q + 2].hh.v.LH;
                        mem[q + 2].hh.v.LH = MIN_HALFWORD;
                    }
                    break;
                case 3:
                    {
                        p = mem[q + 2].hh.v.RH;
                        mem[q + 2].hh.v.RH = MIN_HALFWORD;
                    }
                    break;
                }
                flush_node_list(mem[q + 1].hh.v.LH);
                flush_node_list(mem[q + 1].hh.v.RH);
                flush_node_list(mem[q + 2].hh.v.LH);
                flush_node_list(mem[q + 2].hh.v.RH);
                mem[q].hh.u.B0 = STYLE_NODE;
                mem[q].hh.u.B1 = cur_style;
                mem[q + 1].cint = 0;
                mem[q + 2].cint = 0;
                if (p != MIN_HALFWORD) {
                    z = mem[q].hh.v.RH;
                    mem[q].hh.v.RH = p;
                    while (mem[p].hh.v.RH != MIN_HALFWORD)
                        p = mem[p].hh.v.RH;
                    mem[p].hh.v.RH = z;
                }
                goto lab81;
            }
            break;
        case 3:
        case 4:
        case 5:
        case 8:
        case 12:
        case 7:
            goto lab81;
            break;
        case 2:
            {
                if (mem[q + 3].cint > max_h)
                    max_h = mem[q + 3].cint;
                if (mem[q + 2].cint > max_d)
                    max_d = mem[q + 2].cint;
                goto lab81;
            }
            break;
        case 10:
            {
                if (mem[q].hh.u.B1 == MU_GLUE) {
                    x = mem[q + 1].hh.v.LH;
                    y = math_glue(x, cur_mu);
                    delete_glue_ref(x);
                    mem[q + 1].hh.v.LH = y;
                    mem[q].hh.u.B1 = NORMAL;
                } else if ((cur_size != TEXT_SIZE) && (mem[q].hh.u.B1 == COND_MATH_GLUE)) {
                    p = mem[q].hh.v.RH;
                    if (p != MIN_HALFWORD) {

                        if ((mem[p].hh.u.B0 == GLUE_NODE) || (mem[p].hh.u.B0 == KERN_NODE)) {
                            mem[q].hh.v.RH = mem[p].hh.v.RH;
                            mem[p].hh.v.RH = MIN_HALFWORD;
                            flush_node_list(p);
                        }
                    }
                }
                goto lab81;
            }
            break;
        case 11:
            {
                math_kern(q, cur_mu);
                goto lab81;
            }
            break;
        default:
            confusion(S(mlist1));
            break;
        }
        switch (mem[q + 1].hh.v.RH) {
        case 1:
        case 4:
            {
                fetch(q + 1);
                if (((font_area[cur_f] == AAT_FONT_FLAG)
                     || (font_area[cur_f] == OTGR_FONT_FLAG))) {
                    z = new_native_character(cur_f, cur_c);
                    p = get_node(GLYPH_NODE_SIZE);
                    mem[p].hh.u.B0 = WHATSIT_NODE;
                    mem[p].hh.u.B1 = GLYPH_NODE;
                    mem[p + 4].qqqq.u.B1 = cur_f;
                    mem[p + 4].qqqq.u.B2 = get_native_glyph(z, 0);
                    set_native_glyph_metrics(p, 1);
                    free_node(z, mem[z + 4].qqqq.u.B0);
                    delta = get_ot_math_ital_corr(cur_f, mem[p + 4].qqqq.u.B2);
                    if ((mem[q + 1].hh.v.RH == MATH_TEXT_CHAR)
                        &&
                        (!((font_area[cur_f] == OTGR_FONT_FLAG)
                           && (isOpenTypeMathFont(font_layout_engine[cur_f]))) != 0))
                        delta = 0;
                    if ((mem[q + 3].hh.v.RH == EMPTY) && (delta != 0)) {
                        mem[p].hh.v.RH = new_kern(delta);
                        delta = 0;
                    }
                } else if ((cur_i.u.B0 > 0)) {
                    delta = font_info[italic_base[cur_f] + (cur_i.u.B2) / 4].cint;
                    p = new_character(cur_f, cur_c);
                    if ((mem[q + 1].hh.v.RH == MATH_TEXT_CHAR)
                        && (font_info[SPACE_CODE + param_base[cur_f]].cint != 0))
                        delta = 0;
                    if ((mem[q + 3].hh.v.RH == EMPTY) && (delta != 0)) {
                        mem[p].hh.v.RH = new_kern(delta);
                        delta = 0;
                    }
                } else
                    p = MIN_HALFWORD;
            }
            break;
        case 0:
            p = MIN_HALFWORD;
            break;
        case 2:
            p = mem[q + 1].hh.v.LH;
            break;
        case 3:
            {
                cur_mlist = mem[q + 1].hh.v.LH;
                save_style = cur_style;
                mlist_penalties = false;
                mlist_to_hlist();
                cur_style = save_style;
                {
                    if (cur_style < SCRIPT_STYLE)
                        cur_size = TEXT_SIZE;
                    else
                        cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
                    cur_mu = x_over_n(math_quad(cur_size), 18);
                }
                p = hpack(mem[mem_top - 3].hh.v.RH, 0, ADDITIONAL);
            }
            break;
        default:
            confusion(S(mlist2));
            break;
        }
        mem[q + 1].cint = p;
        if ((mem[q + 3].hh.v.RH == EMPTY) && (mem[q + 2].hh.v.RH == EMPTY))
            goto lab82;
        make_scripts(q, delta);
 lab82:/*check_dimensions */ z = hpack(mem[q + 1].cint, 0, ADDITIONAL);
        if (mem[z + 3].cint > max_h)
            max_h = mem[z + 3].cint;
        if (mem[z + 2].cint > max_d)
            max_d = mem[z + 2].cint;
        free_node(z, BOX_NODE_SIZE);
 lab80:                        /*done_with_noad */ r = q;
        r_type = mem[r].hh.u.B0;
        if (r_type == RIGHT_NOAD) {
            r_type = LEFT_NOAD;
            cur_style = style;
            {
                if (cur_style < SCRIPT_STYLE)
                    cur_size = TEXT_SIZE;
                else
                    cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
                cur_mu = x_over_n(math_quad(cur_size), 18);
            }
        }
 lab81:                        /*done_with_node */ q = mem[q].hh.v.RH;
    }
    if (r_type == BIN_NOAD)
        mem[r].hh.u.B0 = 16 /*ord_noad *//*:755 */ ;
    p = mem_top - 3;
    mem[p].hh.v.RH = MIN_HALFWORD;
    q = mlist;
    r_type = 0;
    cur_style = style;
    {
        if (cur_style < SCRIPT_STYLE)
            cur_size = TEXT_SIZE;
        else
            cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
        cur_mu = x_over_n(math_quad(cur_size), 18);
    }
    while (q != MIN_HALFWORD) {

        t = ORD_NOAD;
        s = NOAD_SIZE;
        pen = INF_PENALTY;
        switch (mem[q].hh.u.B0) {
        case 17:
        case 20:
        case 21:
        case 22:
        case 23:
            t = mem[q].hh.u.B0;
            break;
        case 18:
            {
                t = BIN_NOAD;
                pen = INTPAR(bin_op_penalty);
            }
            break;
        case 19:
            {
                t = REL_NOAD;
                pen = INTPAR(rel_penalty);
            }
            break;
        case 16:
        case 29:
        case 27:
        case 26:
            ;
            break;
        case 24:
            s = RADICAL_NOAD_SIZE;
            break;
        case 28:
            s = ACCENT_NOAD_SIZE;
            break;
        case 25:
            {
                t = INNER_NOAD;
                s = FRACTION_NOAD_SIZE;
            }
            break;
        case 30:
        case 31:
            t = make_left_right(q, style, max_d, max_h);
            break;
        case 14:
            {
                cur_style = mem[q].hh.u.B1;
                s = STYLE_NODE_SIZE;
                {
                    if (cur_style < SCRIPT_STYLE)
                        cur_size = TEXT_SIZE;
                    else
                        cur_size = SCRIPT_SIZE * ((cur_style - 2) / 2);
                    cur_mu = x_over_n(math_quad(cur_size), 18);
                }
                goto lab83;
            }
            break;
        case 8:
        case 12:
        case 2:
        case 7:
        case 5:
        case 3:
        case 4:
        case 10:
        case 11:
            {
                mem[p].hh.v.RH = q;
                p = q;
                q = mem[q].hh.v.RH;
                mem[p].hh.v.RH = MIN_HALFWORD;
                goto done;
            }
            break;
        default:
            confusion(S(mlist3));
            break;
        }
        if (r_type > 0) {
            switch (str_pool[r_type * 8 + t + magic_offset]) {
            case 48:
                x = 0;
                break;
            case 49:
                if (cur_style < SCRIPT_STYLE)
                    x = GLUE_PAR__thin_mu_skip;
                else
                    x = 0;
                break;
            case 50:
                x = GLUE_PAR__thin_mu_skip;
                break;
            case 51:
                if (cur_style < SCRIPT_STYLE)
                    x = GLUE_PAR__med_mu_skip;
                else
                    x = 0;
                break;
            case 52:
                if (cur_style < SCRIPT_STYLE)
                    x = GLUE_PAR__thick_mu_skip;
                else
                    x = 0;
                break;
            default:
                confusion(S(mlist4));
                break;
            }
            if (x != 0) {
                y = math_glue(eqtb[GLUE_BASE + x].hh.v.RH, cur_mu);
                z = new_glue(y);
                mem[y].hh.v.RH = MIN_HALFWORD;
                mem[p].hh.v.RH = z;
                p = z;
                mem[z].hh.u.B1 = x + 1;
            }
        }
        if (mem[q + 1].cint != MIN_HALFWORD) {
            mem[p].hh.v.RH = mem[q + 1].cint;
            do {
                p = mem[p].hh.v.RH;
            } while (!(mem[p].hh.v.RH == MIN_HALFWORD));
        }
        if (penalties) {

            if (mem[q].hh.v.RH != MIN_HALFWORD) {

                if (pen < INF_PENALTY) {
                    r_type = mem[mem[q].hh.v.RH].hh.u.B0;
                    if (r_type != PENALTY_NODE) {

                        if (r_type != REL_NOAD) {
                            z = new_penalty(pen);
                            mem[p].hh.v.RH = z;
                            p = z;
                        }
                    }
                }
            }
        }
        if (mem[q].hh.u.B0 == RIGHT_NOAD)
            t = OPEN_NOAD;
        r_type = t;
 lab83:                        /*delete_q */ r = q;
        q = mem[q].hh.v.RH;
        free_node(r, s);
    done:
        ;
    }
}

void push_alignment(void)
{
    memory_word *mem = zmem; int32_t p;
    p = get_node(ALIGN_STACK_NODE_SIZE);
    mem[p].hh.v.RH = align_ptr;
    mem[p].hh.v.LH = cur_align;
    mem[p + 1].hh.v.LH = mem[mem_top - 8].hh.v.RH;
    mem[p + 1].hh.v.RH = cur_span;
    mem[p + 2].cint = cur_loop;
    mem[p + 3].cint = align_state;
    mem[p + 4].hh.v.LH = cur_head;
    mem[p + 4].hh.v.RH = cur_tail;
    mem[p + 5].hh.v.LH = cur_pre_head;
    mem[p + 5].hh.v.RH = cur_pre_tail;
    align_ptr = p;
    cur_head = get_avail();
    cur_pre_head = get_avail();
}

void pop_alignment(void)
{
    memory_word *mem = zmem; int32_t p;
    {
        mem[cur_head].hh.v.RH = avail;
        avail = cur_head;
    }
    {
        mem[cur_pre_head].hh.v.RH = avail;
        avail = cur_pre_head;
    }
    p = align_ptr;
    cur_tail = mem[p + 4].hh.v.RH;
    cur_head = mem[p + 4].hh.v.LH;
    cur_pre_tail = mem[p + 5].hh.v.RH;
    cur_pre_head = mem[p + 5].hh.v.LH;
    align_state = mem[p + 3].cint;
    cur_loop = mem[p + 2].cint;
    cur_span = mem[p + 1].hh.v.RH;
    mem[mem_top - 8].hh.v.RH = mem[p + 1].hh.v.LH;
    cur_align = mem[p].hh.v.LH;
    align_ptr = mem[p].hh.v.RH;
    free_node(p, ALIGN_STACK_NODE_SIZE);
}

void get_preamble_token(void)
{
    CACHE_THE_EQTB;

lab20:      /*restart */
    get_token();

    while ((cur_chr == SPAN_CODE) && (cur_cmd == TAB_MARK)) {

        get_token();
        if (cur_cmd > MAX_COMMAND) {
            expand();
            get_token();
        }
    }
    if (cur_cmd == ENDV)
        fatal_error(S(_interwoven_alignment_preamb/*les are not allowed)*/));
    if ((cur_cmd == ASSIGN_GLUE) && (cur_chr == (GLUE_BASE + 11))) {
        scan_optional_equals();
        scan_glue(GLUE_VAL);
        if (INTPAR(global_defs) > 0)
            geq_define((GLUE_BASE + 11), GLUE_REF, cur_val);
        else
            eq_define((GLUE_BASE + 11), GLUE_REF, cur_val);
        goto lab20;
    }
}

void init_align(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t save_cs_ptr;
    int32_t p;

    save_cs_ptr = cur_cs;
    push_alignment();
    align_state = -1000000L;
    if ((cur_list.mode == MMODE)
        && ((cur_list.tail != cur_list.head) || (cur_list.aux.cint != MIN_HALFWORD))) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Improper_));
        }
        print_esc(S(halign));
        print(S(_inside____s));
        {
            help_ptr = 3;
            help_line[2] = S(Displays_can_use_special_ali/*gnments (like \eqalignno)*/);
            help_line[1] = S(only_if_nothing_but_the_alig/*nment itself is between $$'s.*/);
            help_line[0] = S(So_I_ve_deleted_the_formulas/* that preceded this alignment.*/);
        }
        error();
        flush_math();
    }
    push_nest();
    if (cur_list.mode == MMODE) {
        cur_list.mode = -1;
        cur_list.aux.cint = nest[nest_ptr - 2].aux.cint;
    } else if (cur_list.mode > 0)
        cur_list.mode = -(integer) cur_list.mode /*:804 */ ;
    scan_spec(ALIGN_GROUP, false);
    mem[mem_top - 8].hh.v.RH = MIN_HALFWORD;
    cur_align = mem_top - 8;
    cur_loop = MIN_HALFWORD;
    scanner_status = ALIGNING;
    warning_index = save_cs_ptr;
    align_state = -1000000L;
    while (true) {

        mem[cur_align].hh.v.RH = new_param_glue(GLUE_PAR__tab_skip);
        cur_align = mem[cur_align].hh.v.RH /*:807 */ ;
        if (cur_cmd == CAR_RET)
            goto done;
        p = mem_top - 4;
        mem[p].hh.v.RH = MIN_HALFWORD;
        while (true) {

            get_preamble_token();
            if (cur_cmd == MAC_PARAM)
                goto lab31;
            if ((cur_cmd <= CAR_RET) && (cur_cmd >= TAB_MARK) && (align_state == -1000000L)) {

                if ((p == mem_top - 4) && (cur_loop == MIN_HALFWORD) && (cur_cmd == TAB_MARK))
                    cur_loop = cur_align;
                else {

                    {
                        if (file_line_error_style_p)
                            print_file_line();
                        else
                            print_nl(S(__/*"! "*/));
                        print(S(Missing___inserted_in_alignm/*ent preamble*/));
                    }
                    {
                        help_ptr = 3;
                        help_line[2] = S(There_should_be_exactly_one_/*# between &'s, when an*/);
                        help_line[1] = S(_halign_or__valign_is_being_/*set up. In this case you had*/);
                        help_line[0] = S(none__so_I_ve_put_one_in__ma/*ybe that will work.*/);
                    }
                    back_error();
                    goto lab31;
                }
            } else if ((cur_cmd != SPACER) || (p != mem_top - 4)) {
                mem[p].hh.v.RH = get_avail();
                p = mem[p].hh.v.RH;
                mem[p].hh.v.LH = cur_tok;
            }
        }
 lab31:                        /*done1 *//*:812 */ ;
        mem[cur_align].hh.v.RH = new_null_box();
        cur_align = mem[cur_align].hh.v.RH;
        mem[cur_align].hh.v.LH = mem_top - 9;
        mem[cur_align + 1].cint = NULL_FLAG;
        mem[cur_align + 3].cint = mem[mem_top - 4].hh.v.RH;
        p = mem_top - 4;
        mem[p].hh.v.RH = MIN_HALFWORD;
        while (true) {

 lab22:                        /*continue */ get_preamble_token();
            if ((cur_cmd <= CAR_RET) && (cur_cmd >= TAB_MARK) && (align_state == -1000000L))
                goto lab32;
            if (cur_cmd == MAC_PARAM) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Only_one___is_allowed_per_ta/*b*/));
                }
                {
                    help_ptr = 3;
                    help_line[2] = S(There_should_be_exactly_one_/*# between &'s, when an*/);
                    help_line[1] = S(_halign_or__valign_is_being_/*set up. In this case you had*/);
                    help_line[0] = S(more_than_one__so_I_m_ignori/*ng all but the first.*/);
                }
                error();
                goto lab22;
            }
            mem[p].hh.v.RH = get_avail();
            p = mem[p].hh.v.RH;
            mem[p].hh.v.LH = cur_tok;
        }
 lab32:                        /*done2 */ mem[p].hh.v.RH = get_avail();
        p = mem[p].hh.v.RH;
        mem[p].hh.v.LH = 35797662L /*cs_token_flag 2243231 *//*:813 */ ;
        mem[cur_align + 2].cint = mem[mem_top - 4].hh.v.RH /*:808 */ ;
    }
done:
    scanner_status = 0 /*normal *//*:806 */ ;
    new_save_level(ALIGN_GROUP);
    if (LOCAL(every_cr) != MIN_HALFWORD)
        begin_token_list(LOCAL(every_cr), EVERY_CR_TEXT);
    align_peek();
}

void init_span(int32_t p)
{
    push_nest();
    if (cur_list.mode == -104)
        cur_list.aux.hh.v.LH = 1000;
    else {

        cur_list.aux.cint = -65536000L;
        normal_paragraph();
    }
    cur_span = p;
}

void init_row(void)
{
    memory_word *mem = zmem; push_nest();
    cur_list.mode = (-105) - cur_list.mode;
    if (cur_list.mode == -104)
        cur_list.aux.hh.v.LH = 0;
    else
        cur_list.aux.cint = 0;
    {
        mem[cur_list.tail].hh.v.RH = new_glue(mem[mem[mem_top - 8].hh.v.RH + 1].hh.v.LH);
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
    mem[cur_list.tail].hh.u.B1 = (GLUE_PAR__tab_skip + 1);
    cur_align = mem[mem[mem_top - 8].hh.v.RH].hh.v.RH;
    cur_tail = cur_head;
    cur_pre_tail = cur_pre_head;
    init_span(cur_align);
}

void init_col(void)
{
    memory_word *mem = zmem; mem[cur_align + 5].hh.v.LH = cur_cmd;
    if (cur_cmd == OMIT)
        align_state = 0;
    else {

        back_input();
        begin_token_list(mem[cur_align + 3].cint, U_TEMPLATE);
    }
}

boolean fin_col(void)
{
    register boolean Result;
    memory_word *mem = zmem; int32_t p;
    int32_t q, r;
    int32_t s;
    int32_t u;
    scaled w;
    glue_ord o;
    int32_t n;
    if (cur_align == MIN_HALFWORD)
        confusion(S(endv));
    q = mem[cur_align].hh.v.RH;
    if (q == MIN_HALFWORD)
        confusion(S(endv));
    if (align_state < 500000L)
        fatal_error(S(_interwoven_alignment_preamb/*les are not allowed)*/));
    p = mem[q].hh.v.RH;
    if ((p == MIN_HALFWORD) && (mem[cur_align + 5].hh.v.LH < CR_CODE)) {

        if (cur_loop != MIN_HALFWORD) {  /*822: */
            mem[q].hh.v.RH = new_null_box();
            p = mem[q].hh.v.RH;
            mem[p].hh.v.LH = mem_top - 9;
            mem[p + 1].cint = NULL_FLAG;
            cur_loop = mem[cur_loop].hh.v.RH;
            q = mem_top - 4;
            r = mem[cur_loop + 3].cint;
            while (r != MIN_HALFWORD) {

                mem[q].hh.v.RH = get_avail();
                q = mem[q].hh.v.RH;
                mem[q].hh.v.LH = mem[r].hh.v.LH;
                r = mem[r].hh.v.RH;
            }
            mem[q].hh.v.RH = MIN_HALFWORD;
            mem[p + 3].cint = mem[mem_top - 4].hh.v.RH;
            q = mem_top - 4;
            r = mem[cur_loop + 2].cint;
            while (r != MIN_HALFWORD) {

                mem[q].hh.v.RH = get_avail();
                q = mem[q].hh.v.RH;
                mem[q].hh.v.LH = mem[r].hh.v.LH;
                r = mem[r].hh.v.RH;
            }
            mem[q].hh.v.RH = MIN_HALFWORD;
            mem[p + 2].cint = mem[mem_top - 4].hh.v.RH /*:823 */ ;
            cur_loop = mem[cur_loop].hh.v.RH;
            mem[p].hh.v.RH = new_glue(mem[cur_loop + 1].hh.v.LH);
        } else {

            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Extra_alignment_tab_has_been/* changed to */));
            }
            print_esc(S(cr));
            {
                help_ptr = 3;
                help_line[2] = S(You_have_given_more__span_or/* & marks than there were*/);
                help_line[1] = S(in_the_preamble_to_the__hali/*gn or \valign now in progress.*/);
                help_line[0] = S(So_I_ll_assume_that_you_mean/*t to type \cr instead.*/);
            }
            mem[cur_align + 5].hh.v.LH = CR_CODE;
            error();
        }
    }
    if (mem[cur_align + 5].hh.v.LH != SPAN_CODE) {
        unsave();
        new_save_level(ALIGN_GROUP);
        {
            if (cur_list.mode == -104) {
                adjust_tail = cur_tail;
                pre_adjust_tail = cur_pre_tail;
                u = hpack(mem[cur_list.head].hh.v.RH, 0, ADDITIONAL);
                w = mem[u + 1].cint;
                cur_tail = adjust_tail;
                adjust_tail = MIN_HALFWORD;
                cur_pre_tail = pre_adjust_tail;
                pre_adjust_tail = MIN_HALFWORD;
            } else {

                u = vpackage(mem[cur_list.head].hh.v.RH, 0, ADDITIONAL, 0);
                w = mem[u + 3].cint;
            }
            n = 0;
            if (cur_span != cur_align) {        /*827: */
                q = cur_span;
                do {
                    n++;
                    q = mem[mem[q].hh.v.RH].hh.v.RH;
                } while (!(q == cur_align));
                if (n > UINT16_MAX)
                    confusion(S(too_many_spans));
                q = cur_span;
                while (mem[mem[q].hh.v.LH].hh.v.RH < n)
                    q = mem[q].hh.v.LH;
                if (mem[mem[q].hh.v.LH].hh.v.RH > n) {
                    s = get_node(SPAN_NODE_SIZE);
                    mem[s].hh.v.LH = mem[q].hh.v.LH;
                    mem[s].hh.v.RH = n;
                    mem[q].hh.v.LH = s;
                    mem[s + 1].cint = w;
                } else if (mem[mem[q].hh.v.LH + 1].cint < w)
                    mem[mem[q].hh.v.LH + 1].cint = w;
            } else if (w > mem[cur_align + 1].cint)
                mem[cur_align + 1].cint = w;
            mem[u].hh.u.B0 = UNSET_NODE;
            mem[u].hh.u.B1 = n;
            if (total_stretch[FILLL] != 0)
                o = FILLL;
            else if (total_stretch[FILL] != 0)
                o = FILL;
            else if (total_stretch[FIL] != 0)
                o = FIL;
            else
                o = 0 /*normal *//*:684 */ ;
            mem[u + 5].hh.u.B1 = o;
            mem[u + 6].cint = total_stretch[o];
            if (total_shrink[FILLL] != 0)
                o = FILLL;
            else if (total_shrink[FILL] != 0)
                o = FILL;
            else if (total_shrink[FIL] != 0)
                o = FIL;
            else
                o = 0 /*normal *//*:690 */ ;
            mem[u + 5].hh.u.B0 = o;
            mem[u + 4].cint = total_shrink[o];
            pop_nest();
            mem[cur_list.tail].hh.v.RH = u;
            cur_list.tail = u;
        }
        {
            mem[cur_list.tail].hh.v.RH = new_glue(mem[mem[cur_align].hh.v.RH + 1].hh.v.LH);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        mem[cur_list.tail].hh.u.B1 = 12 /*tab_skip_code 1 *//*:824 */ ;
        if (mem[cur_align + 5].hh.v.LH >= CR_CODE) {
            Result = true;
            return Result;
        }
        init_span(p);
    }
    align_state = 1000000L;
    do {
        get_x_or_protected();
    } while (!(cur_cmd != SPACER));
    cur_align = p;
    init_col();
    Result = false;
    return Result;
}

void fin_row(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;

    if (cur_list.mode == -104) {
        p = hpack(mem[cur_list.head].hh.v.RH, 0, ADDITIONAL);
        pop_nest();
        if (cur_pre_head != cur_pre_tail) {
            mem[cur_list.tail].hh.v.RH = mem[cur_pre_head].hh.v.RH;
            cur_list.tail = cur_pre_tail;
        }
        append_to_vlist(p);
        if (cur_head != cur_tail) {
            mem[cur_list.tail].hh.v.RH = mem[cur_head].hh.v.RH;
            cur_list.tail = cur_tail;
        }
    } else {

        p = vpackage(mem[cur_list.head].hh.v.RH, 0, ADDITIONAL, MAX_HALFWORD);
        pop_nest();
        mem[cur_list.tail].hh.v.RH = p;
        cur_list.tail = p;
        cur_list.aux.hh.v.LH = 1000;
    }
    mem[p].hh.u.B0 = UNSET_NODE;
    mem[p + 6].cint = 0;
    if (LOCAL(every_cr) != MIN_HALFWORD)
        begin_token_list(LOCAL(every_cr), EVERY_CR_TEXT);
    align_peek();
}

void fin_align(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p, q, r, s, u, v;
    scaled t, w;
    scaled o;
    int32_t n;
    scaled rule_save;
    memory_word aux_save;

    if (cur_group != ALIGN_GROUP)
        confusion(S(align1));
    unsave();
    if (cur_group != ALIGN_GROUP)
        confusion(S(align0));
    unsave();
    if (nest[nest_ptr - 1].mode == MMODE)
        o = DIMENPAR(display_indent);
    else
        o = 0;
    q = mem[mem[mem_top - 8].hh.v.RH].hh.v.RH;
    do {
        flush_list(mem[q + 3].cint);
        flush_list(mem[q + 2].cint);
        p = mem[mem[q].hh.v.RH].hh.v.RH;
        if (mem[q + 1].cint == NULL_FLAG) {  /*831: */
            mem[q + 1].cint = 0;
            r = mem[q].hh.v.RH;
            s = mem[r + 1].hh.v.LH;
            if (s != 0) {
                mem[0].hh.v.RH++;
                delete_glue_ref(s);
                mem[r + 1].hh.v.LH = 0;
            }
        }
        if (mem[q].hh.v.LH != mem_top - 9) {    /*832: */
            t = mem[q + 1].cint + mem[mem[mem[q].hh.v.RH + 1].hh.v.LH + 1].cint;
            r = mem[q].hh.v.LH;
            s = mem_top - 9;
            mem[s].hh.v.LH = p;
            n = 1;
            do {
                mem[r + 1].cint = mem[r + 1].cint - t;
                u = mem[r].hh.v.LH;
                while (mem[r].hh.v.RH > n) {

                    s = mem[s].hh.v.LH;
                    n = mem[mem[s].hh.v.LH].hh.v.RH + 1;
                }
                if (mem[r].hh.v.RH < n) {
                    mem[r].hh.v.LH = mem[s].hh.v.LH;
                    mem[s].hh.v.LH = r;
                    mem[r].hh.v.RH--;
                    s = r;
                } else {

                    if (mem[r + 1].cint > mem[mem[s].hh.v.LH + 1].cint)
                        mem[mem[s].hh.v.LH + 1].cint = mem[r + 1].cint;
                    free_node(r, SPAN_NODE_SIZE);
                }
                r = u;
            } while (!(r == mem_top - 9));
        }
        mem[q].hh.u.B0 = UNSET_NODE;
        mem[q].hh.u.B1 = 0;
        mem[q + 3].cint = 0;
        mem[q + 2].cint = 0;
        mem[q + 5].hh.u.B1 = NORMAL;
        mem[q + 5].hh.u.B0 = NORMAL;
        mem[q + 6].cint = 0;
        mem[q + 4].cint = 0;
        q = p;
    } while (!(q == MIN_HALFWORD /*:830 */ ));
    save_ptr = save_ptr - 2;
    pack_begin_line = -(integer) cur_list.ml;
    if (cur_list.mode == -1) {
        rule_save = DIMENPAR(overfull_rule);
        DIMENPAR(overfull_rule) = 0;
        p = hpack(mem[mem_top - 8].hh.v.RH, save_stack[save_ptr + 1].cint, save_stack[save_ptr + 0].cint);
        DIMENPAR(overfull_rule) = rule_save;
    } else {

        q = mem[mem[mem_top - 8].hh.v.RH].hh.v.RH;
        do {
            mem[q + 3].cint = mem[q + 1].cint;
            mem[q + 1].cint = 0;
            q = mem[mem[q].hh.v.RH].hh.v.RH;
        } while (!(q == MIN_HALFWORD));
        p = vpackage(mem[mem_top - 8].hh.v.RH, save_stack[save_ptr + 1].cint, save_stack[save_ptr + 0].cint,
                     MAX_HALFWORD);
        q = mem[mem[mem_top - 8].hh.v.RH].hh.v.RH;
        do {
            mem[q + 1].cint = mem[q + 3].cint;
            mem[q + 3].cint = 0;
            q = mem[mem[q].hh.v.RH].hh.v.RH;
        } while (!(q == MIN_HALFWORD));
    }
    pack_begin_line = 0 /*:833 */ ;
    q = mem[cur_list.head].hh.v.RH;
    s = cur_list.head;
    while (q != MIN_HALFWORD) {

        if (!(q >= hi_mem_min)) {

            if (mem[q].hh.u.B0 == UNSET_NODE) {  /*836: */
                if (cur_list.mode == -1) {
                    mem[q].hh.u.B0 = HLIST_NODE;
                    mem[q + 1].cint = mem[p + 1].cint;
                    if (nest[nest_ptr - 1].mode == MMODE)
                        mem[q].hh.u.B1 = DLIST;
                } else {

                    mem[q].hh.u.B0 = VLIST_NODE;
                    mem[q + 3].cint = mem[p + 3].cint;
                }
                mem[q + 5].hh.u.B1 = mem[p + 5].hh.u.B1;
                mem[q + 5].hh.u.B0 = mem[p + 5].hh.u.B0;
                mem[q + 6].gr = mem[p + 6].gr;
                mem[q + 4].cint = o;
                r = mem[mem[q + 5].hh.v.RH].hh.v.RH;
                s = mem[mem[p + 5].hh.v.RH].hh.v.RH;
                do {
                    /*837: */ n = mem[r].hh.u.B1;
                    t = mem[s + 1].cint;
                    w = t;
                    u = mem_top - 4;
                    mem[r].hh.u.B1 = 0;
                    while (n > 0) {

                        n--;
                        s = mem[s].hh.v.RH;
                        v = mem[s + 1].hh.v.LH;
                        mem[u].hh.v.RH = new_glue(v);
                        u = mem[u].hh.v.RH;
                        mem[u].hh.u.B1 = (GLUE_PAR__tab_skip + 1);
                        t = t + mem[v + 1].cint;
                        if (mem[p + 5].hh.u.B0 == STRETCHING) {
                            if (mem[v].hh.u.B0 == mem[p + 5].hh.u.B1)
                                t = t + tex_round(mem[p + 6].gr * mem[v + 2].cint);
                        } else if (mem[p + 5].hh.u.B0 == SHRINKING) {
                            if (mem[v].hh.u.B1 == mem[p + 5].hh.u.B1)
                                t = t - tex_round(mem[p + 6].gr * mem[v + 3].cint);
                        }
                        s = mem[s].hh.v.RH;
                        mem[u].hh.v.RH = new_null_box();
                        u = mem[u].hh.v.RH;
                        t = t + mem[s + 1].cint;
                        if (cur_list.mode == -1)
                            mem[u + 1].cint = mem[s + 1].cint;
                        else {

                            mem[u].hh.u.B0 = VLIST_NODE;
                            mem[u + 3].cint = mem[s + 1].cint;
                        }
                    }
                    if (cur_list.mode == -1) {    /*839: */
                        mem[r + 3].cint = mem[q + 3].cint;
                        mem[r + 2].cint = mem[q + 2].cint;
                        if (t == mem[r + 1].cint) {
                            mem[r + 5].hh.u.B0 = NORMAL;
                            mem[r + 5].hh.u.B1 = NORMAL;
                            mem[r + 6].gr = 0.0;
                        } else if (t > mem[r + 1].cint) {
                            mem[r + 5].hh.u.B0 = STRETCHING;
                            if (mem[r + 6].cint == 0)
                                mem[r + 6].gr = 0.0;
                            else
                                mem[r + 6].gr = (t - mem[r + 1].cint) / ((double)mem[r + 6].cint);
                        } else {

                            mem[r + 5].hh.u.B1 = mem[r + 5].hh.u.B0;
                            mem[r + 5].hh.u.B0 = SHRINKING;
                            if (mem[r + 4].cint == 0)
                                mem[r + 6].gr = 0.0;
                            else if ((mem[r + 5].hh.u.B1 == NORMAL) && (mem[r + 1].cint - t > mem[r + 4].cint))
                                mem[r + 6].gr = 1.0;
                            else
                                mem[r + 6].gr = (mem[r + 1].cint - t) / ((double)mem[r + 4].cint);
                        }
                        mem[r + 1].cint = w;
                        mem[r].hh.u.B0 = HLIST_NODE;
                    } else {    /*840: */

                        mem[r + 1].cint = mem[q + 1].cint;
                        if (t == mem[r + 3].cint) {
                            mem[r + 5].hh.u.B0 = NORMAL;
                            mem[r + 5].hh.u.B1 = NORMAL;
                            mem[r + 6].gr = 0.0;
                        } else if (t > mem[r + 3].cint) {
                            mem[r + 5].hh.u.B0 = STRETCHING;
                            if (mem[r + 6].cint == 0)
                                mem[r + 6].gr = 0.0;
                            else
                                mem[r + 6].gr = (t - mem[r + 3].cint) / ((double)mem[r + 6].cint);
                        } else {

                            mem[r + 5].hh.u.B1 = mem[r + 5].hh.u.B0;
                            mem[r + 5].hh.u.B0 = SHRINKING;
                            if (mem[r + 4].cint == 0)
                                mem[r + 6].gr = 0.0;
                            else if ((mem[r + 5].hh.u.B1 == NORMAL) && (mem[r + 3].cint - t > mem[r + 4].cint))
                                mem[r + 6].gr = 1.0;
                            else
                                mem[r + 6].gr = (mem[r + 3].cint - t) / ((double)mem[r + 4].cint);
                        }
                        mem[r + 3].cint = w;
                        mem[r].hh.u.B0 = VLIST_NODE;
                    }
                    mem[r + 4].cint = 0;
                    if (u != mem_top - 4) {
                        mem[u].hh.v.RH = mem[r].hh.v.RH;
                        mem[r].hh.v.RH = mem[mem_top - 4].hh.v.RH;
                        r = u;
                    }
                    r = mem[mem[r].hh.v.RH].hh.v.RH;
                    s = mem[mem[s].hh.v.RH].hh.v.RH;
                } while (!(r == MIN_HALFWORD));
            } else if (mem[q].hh.u.B0 == RULE_NODE) {     /*835: */
                if ((mem[q + 1].cint == NULL_FLAG))
                    mem[q + 1].cint = mem[p + 1].cint;
                if ((mem[q + 3].cint == NULL_FLAG))
                    mem[q + 3].cint = mem[p + 3].cint;
                if ((mem[q + 2].cint == NULL_FLAG))
                    mem[q + 2].cint = mem[p + 2].cint;
                if (o != 0) {
                    r = mem[q].hh.v.RH;
                    mem[q].hh.v.RH = MIN_HALFWORD;
                    q = hpack(q, 0, ADDITIONAL);
                    mem[q + 4].cint = o;
                    mem[q].hh.v.RH = r;
                    mem[s].hh.v.RH = q;
                }
            }
        }
        s = q;
        q = mem[q].hh.v.RH;
    }
    flush_node_list(p);
    pop_alignment();
    aux_save = cur_list.aux;
    p = mem[cur_list.head].hh.v.RH;
    q = cur_list.tail;
    pop_nest();
    if (cur_list.mode == MMODE) {       /*1241: */
        do_assignments();
        if (cur_cmd != MATH_SHIFT) {    /*1242: */
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Missing____inserted));
            }
            {
                help_ptr = 2;
                help_line[1] = S(Displays_can_use_special_ali/*gnments (like \eqalignno)*/);
                help_line[0] = S(only_if_nothing_but_the_alig/*nment itself is between $$'s.*/);
            }
            back_error();
        } else {                /*1232: */

            get_x_token();
            if (cur_cmd != MATH_SHIFT) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Display_math_should_end_with/* $$*/));
                }
                {
                    help_ptr = 2;
                    help_line[1] = S(The_____that_I_just_saw_supp/*osedly matches a previous `$$'.*/);
                    help_line[0] = S(So_I_shall_assume_that_you_t/*yped `$$' both times.*/);
                }
                back_error();
            }
        }
        flush_node_list(cur_list.eTeX_aux);
        pop_nest();
        {
            mem[cur_list.tail].hh.v.RH = new_penalty(INTPAR(pre_display_penalty));
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        {
            mem[cur_list.tail].hh.v.RH = new_param_glue(GLUE_PAR__above_display_skip);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        mem[cur_list.tail].hh.v.RH = p;
        if (p != MIN_HALFWORD)
            cur_list.tail = q;
        {
            mem[cur_list.tail].hh.v.RH = new_penalty(INTPAR(post_display_penalty));
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        {
            mem[cur_list.tail].hh.v.RH = new_param_glue(GLUE_PAR__below_display_skip);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        cur_list.aux.cint = aux_save.cint;
        resume_after_display();
    } else {

        cur_list.aux = aux_save;
        mem[cur_list.tail].hh.v.RH = p;
        if (p != MIN_HALFWORD)
            cur_list.tail = q;
        if (cur_list.mode == VMODE)
            build_page();
    }
}

void align_peek(void)
{
 lab20:      /*restart */ align_state = 1000000L;
    do {
        get_x_or_protected();
    } while (!(cur_cmd != SPACER));
    if (cur_cmd == NO_ALIGN) {
        scan_left_brace();
        new_save_level(NO_ALIGN_GROUP);
        if (cur_list.mode == -1)
            normal_paragraph();
    } else if (cur_cmd == RIGHT_BRACE)
        fin_align();
    else if ((cur_cmd == CAR_RET) && (cur_chr == CR_CR_CODE))
        goto lab20;
    else {

        init_row();
        init_col();
    }
}

int32_t finite_shrink(int32_t p)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t q;
    if (no_shrink_error_yet) {
        no_shrink_error_yet = false;
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Infinite_glue_shrinkage_foun/*d in a paragraph*/));
        }
        {
            help_ptr = 5;
            help_line[4] = S(The_paragraph_just_ended_inc/*ludes some glue that has*/);
            help_line[3] = S(infinite_shrinkability__e_g_/*, `\hskip 0pt minus 1fil'.*/);
            help_line[2] = S(Such_glue_doesn_t_belong_the/*re---it allows a paragraph*/);
            help_line[1] = S(of_any_length_to_fit_on_one_/*line. But it's safe to proceed,*/);
            help_line[0] = S(since_the_offensive_shrinkab/*ility has been made finite.*/);
        }
        error();
    }
    q = new_spec(p);
    mem[q].hh.u.B1 = NORMAL;
    delete_glue_ref(p);
    Result = q;
    return Result;
}

void push_node(int32_t p)
{
    if (hlist_stack_level > MAX_HLIST_STACK)
        pdf_error(S(push_node), S(stack_overflow));
    hlist_stack[hlist_stack_level] = p;
    hlist_stack_level = hlist_stack_level + 1;
}

int32_t pop_node(void)
{
    register int32_t Result;
    hlist_stack_level = hlist_stack_level - 1;
    if (hlist_stack_level < 0)
        pdf_error(S(pop_node), S(stack_underflow__internal_er/*ror)*/));
    Result = hlist_stack[hlist_stack_level];
    return Result;
}

int32_t find_protchar_left(int32_t l, boolean d)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t t;
    boolean run;
    if ((mem[l].hh.v.RH != MIN_HALFWORD) && (mem[l].hh.u.B0 == HLIST_NODE) && (mem[l + 1].cint == 0)
        && (mem[l + 3].cint == 0) && (mem[l + 2].cint == 0) && (mem[l + 5].hh.v.RH == MIN_HALFWORD))
        l = mem[l].hh.v.RH;
    else if (d)
        while ((mem[l].hh.v.RH != MIN_HALFWORD) && (!((l >= hi_mem_min) || (mem[l].hh.u.B0 < MATH_NODE))))
            l = mem[l].hh.v.RH;
    hlist_stack_level = 0;
    run = true;
    do {
        t = l;
        while (run && (mem[l].hh.u.B0 == HLIST_NODE) && (mem[l + 5].hh.v.RH != MIN_HALFWORD)) {

            push_node(l);
            l = mem[l + 5].hh.v.RH;
        }
        while (run
               && (!(l >= hi_mem_min)
                   && ((mem[l].hh.u.B0 == INS_NODE) || (mem[l].hh.u.B0 == MARK_NODE)
                       || (mem[l].hh.u.B0 == ADJUST_NODE) || (mem[l].hh.u.B0 == PENALTY_NODE)
                       || ((mem[l].hh.u.B0 == DISC_NODE) && (mem[l + 1].hh.v.LH == MIN_HALFWORD)
                           && (mem[l + 1].hh.v.RH == MIN_HALFWORD) && (mem[l].hh.u.B1 == 0))
                       || ((mem[l].hh.u.B0 == MATH_NODE) && (mem[l + 1].cint == 0))
                       || ((mem[l].hh.u.B0 == KERN_NODE)
                           && ((mem[l + 1].cint == 0) || (mem[l].hh.u.B1 == NORMAL)))
                       || ((mem[l].hh.u.B0 == GLUE_NODE) && (mem[l + 1].hh.v.LH == 0))
                       || ((mem[l].hh.u.B0 == HLIST_NODE) && (mem[l + 1].cint == 0) && (mem[l + 3].cint == 0)
                           && (mem[l + 2].cint == 0) && (mem[l + 5].hh.v.RH == MIN_HALFWORD))))) {

            while ((mem[l].hh.v.RH == MIN_HALFWORD) && (hlist_stack_level > 0)) {

                l = pop_node();
            }
            if (mem[l].hh.v.RH != MIN_HALFWORD)
                l = mem[l].hh.v.RH;
            else if (hlist_stack_level == 0)
                run = false;
        }
    } while (!(t == l));
    Result = l;
    return Result;
}

int32_t find_protchar_right(int32_t l, int32_t r)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t t;
    boolean run;
    Result = MIN_HALFWORD;
    if (r == MIN_HALFWORD)
        return Result;
    hlist_stack_level = 0;
    run = true;
    do {
        t = r;
        while (run && (mem[r].hh.u.B0 == HLIST_NODE) && (mem[r + 5].hh.v.RH != MIN_HALFWORD)) {

            push_node(l);
            push_node(r);
            l = mem[r + 5].hh.v.RH;
            r = l;
            while (mem[r].hh.v.RH != MIN_HALFWORD)
                r = mem[r].hh.v.RH;
        }
        while (run
               && (!(r >= hi_mem_min)
                   && ((mem[r].hh.u.B0 == INS_NODE) || (mem[r].hh.u.B0 == MARK_NODE)
                       || (mem[r].hh.u.B0 == ADJUST_NODE) || (mem[r].hh.u.B0 == PENALTY_NODE)
                       || ((mem[r].hh.u.B0 == DISC_NODE) && (mem[r + 1].hh.v.LH == MIN_HALFWORD)
                           && (mem[r + 1].hh.v.RH == MIN_HALFWORD) && (mem[r].hh.u.B1 == 0))
                       || ((mem[r].hh.u.B0 == MATH_NODE) && (mem[r + 1].cint == 0))
                       || ((mem[r].hh.u.B0 == KERN_NODE)
                           && ((mem[r + 1].cint == 0) || (mem[r].hh.u.B1 == NORMAL)))
                       || ((mem[r].hh.u.B0 == GLUE_NODE) && (mem[r + 1].hh.v.LH == 0))
                       || ((mem[r].hh.u.B0 == HLIST_NODE) && (mem[r + 1].cint == 0) && (mem[r + 3].cint == 0)
                           && (mem[r + 2].cint == 0) && (mem[r + 5].hh.v.RH == MIN_HALFWORD))))) {

            while ((r == l) && (hlist_stack_level > 0)) {

                r = pop_node();
                l = pop_node();
            }
            if ((r != l) && (r != MIN_HALFWORD))
                r = prev_rightmost(l, r);
            else if ((r == l) && (hlist_stack_level == 0))
                run = false;
        }
    } while (!(t == r));
    Result = r;
    return Result;
}

scaled total_pw(int32_t q, int32_t p)
{
    register scaled Result;
    memory_word *mem = zmem; int32_t l, r;
    integer n;
    if (mem[q + 1].hh.v.RH == MIN_HALFWORD)
        l = first_p;
    else
        l = mem[mem[q + 1].hh.v.RH + 1].hh.v.RH;
    r = prev_rightmost(global_prev_p, p);
    if ((p != MIN_HALFWORD) && (mem[p].hh.u.B0 == DISC_NODE) && (mem[p + 1].hh.v.LH != MIN_HALFWORD)) {
        r = mem[p + 1].hh.v.LH;
        while (mem[r].hh.v.RH != MIN_HALFWORD)
            r = mem[r].hh.v.RH;
    } else
        r = find_protchar_right(l, r);
    if ((l != MIN_HALFWORD) && (mem[l].hh.u.B0 == DISC_NODE)) {
        if (mem[l + 1].hh.v.RH != MIN_HALFWORD) {
            l = mem[l + 1].hh.v.RH;
            goto done;
        } else {

            n = mem[l].hh.u.B1;
            l = mem[l].hh.v.RH;
            while (n > 0) {

                if (mem[l].hh.v.RH != MIN_HALFWORD)
                    l = mem[l].hh.v.RH;
                n--;
            }
        }
    }
    l = find_protchar_left(l, true);

done:
    Result = char_pw(l, 0) + char_pw(r, 1);
    return Result;
}

void try_break(integer pi, small_number break_type)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t r;
    int32_t prev_r;
    int32_t old_l;
    boolean no_break_yet;
    int32_t prev_prev_r;
    int32_t s;
    int32_t q;
    int32_t v;
    integer t;
    internal_font_number f;
    int32_t l;
    boolean node_r_stays_active;
    scaled line_width;
    unsigned char /*tight_fit */ fit_class;
    int32_t b;
    integer d;
    boolean artificial_demerits;
    scaled shortfall;
    scaled g;

    if (abs(pi) >= INF_PENALTY) {
        if (pi > 0)
            goto exit;
        else
            pi = -10000 /*eject_penalty *//*:860 */ ;
    }

    no_break_yet = true;
    prev_r = mem_top - 7;
    old_l = 0;
    cur_active_width[1] = active_width[1];
    cur_active_width[2] = active_width[2];
    cur_active_width[3] = active_width[3];
    cur_active_width[4] = active_width[4];
    cur_active_width[5] = active_width[5];
    cur_active_width[6] = active_width[6];
    while (true) {

 lab22:                        /*continue */ r = mem[prev_r].hh.v.RH;
        if (mem[r].hh.u.B0 == DELTA_NODE) {
            cur_active_width[1] = cur_active_width[1] + mem[r + 1].cint;
            cur_active_width[2] = cur_active_width[2] + mem[r + 2].cint;
            cur_active_width[3] = cur_active_width[3] + mem[r + 3].cint;
            cur_active_width[4] = cur_active_width[4] + mem[r + 4].cint;
            cur_active_width[5] = cur_active_width[5] + mem[r + 5].cint;
            cur_active_width[6] = cur_active_width[6] + mem[r + 6].cint;
            prev_prev_r = prev_r;
            prev_r = r;
            goto lab22;
        }
        {
            l = mem[r + 1].hh.v.LH;
            if (l > old_l) {
                if ((minimum_demerits < MAX_HALFWORD) && ((old_l != easy_line) || (r == mem_top - 7))) { /*865: */
                    if (no_break_yet) { /*866: */
                        no_break_yet = false;
                        break_width[1] = background[1];
                        break_width[2] = background[2];
                        break_width[3] = background[3];
                        break_width[4] = background[4];
                        break_width[5] = background[5];
                        break_width[6] = background[6];
                        s = cur_p;
                        if (break_type > UNHYPHENATED) {

                            if (cur_p != MIN_HALFWORD) { /*869: */
                                t = mem[cur_p].hh.u.B1;
                                v = cur_p;
                                s = mem[cur_p + 1].hh.v.RH;
                                while (t > 0) {

                                    t--;
                                    v = mem[v].hh.v.RH;
                                    if ((v >= hi_mem_min)) {
                                        f = mem[v].hh.u.B0;
                                        break_width[1] =
                                            break_width[1] - font_info[width_base[f] +
                                                                       font_info[char_base[f] +
                                                                                 effective_char(true, f,
                                                                                                mem[v].hh.u.B1)].qqqq.u.B0].
                                            cint;
                                    } else
                                        switch (mem[v].hh.u.B0) {
                                        case 6:
                                            {
                                                f = mem[v + 1].hh.u.B0;
                                                xtx_ligature_present = true;
                                                break_width[1] =
                                                    break_width[1] - font_info[width_base[f] +
                                                                               font_info[char_base[f] +
                                                                                         effective_char(true, f,
                                                                                                        mem[v +
                                                                                                            1].hh.u.B1)].
                                                                               qqqq.u.B0].cint;
                                            }
                                            break;
                                        case 0:
                                        case 1:
                                        case 2:
                                        case 11:
                                            break_width[1] = break_width[1] - mem[v + 1].cint;
                                            break;
                                        case 8:
                                            if ((mem[v].hh.u.B1 == NATIVE_WORD_NODE)
                                                || (mem[v].hh.u.B1 == NATIVE_WORD_NODE_AT)
                                                || (mem[v].hh.u.B1 == GLYPH_NODE)
                                                || (mem[v].hh.u.B1 == PIC_NODE)
                                                || (mem[v].hh.u.B1 == PDF_NODE))
                                                break_width[1] = break_width[1] - mem[v + 1].cint;
                                            else
                                                confusion(S(disc1a));
                                            break;
                                        default:
                                            confusion(S(disc1));
                                            break;
                                        }
                                }
                                while (s != MIN_HALFWORD) {

                                    if ((s >= hi_mem_min)) {
                                        f = mem[s].hh.u.B0;
                                        break_width[1] =
                                            break_width[1] + font_info[width_base[f] +
                                                                       font_info[char_base[f] +
                                                                                 effective_char(true, f,
                                                                                                mem[s].hh.u.B1)].qqqq.u.B0].
                                            cint;
                                    } else
                                        switch (mem[s].hh.u.B0) {
                                        case 6:
                                            {
                                                f = mem[s + 1].hh.u.B0;
                                                xtx_ligature_present = true;
                                                break_width[1] =
                                                    break_width[1] + font_info[width_base[f] +
                                                                               font_info[char_base[f] +
                                                                                         effective_char(true, f,
                                                                                                        mem[s +
                                                                                                            1].hh.u.B1)].
                                                                               qqqq.u.B0].cint;
                                            }
                                            break;
                                        case 0:
                                        case 1:
                                        case 2:
                                        case 11:
                                            break_width[1] = break_width[1] + mem[s + 1].cint;
                                            break;
                                        case 8:
                                            if ((mem[s].hh.u.B1 == NATIVE_WORD_NODE)
                                                || (mem[s].hh.u.B1 == NATIVE_WORD_NODE_AT)
                                                || (mem[s].hh.u.B1 == GLYPH_NODE)
                                                || (mem[s].hh.u.B1 == PIC_NODE)
                                                || (mem[s].hh.u.B1 == PDF_NODE))
                                                break_width[1] = break_width[1] + mem[s + 1].cint;
                                            else
                                                confusion(S(disc2a));
                                            break;
                                        default:
                                            confusion(S(disc2));
                                            break;
                                        }
                                    s = mem[s].hh.v.RH;
                                }
                                break_width[1] = break_width[1] + disc_width;
                                if (mem[cur_p + 1].hh.v.RH == MIN_HALFWORD)
                                    s = mem[v].hh.v.RH;
                            }
                        }
                        while (s != MIN_HALFWORD) {

                            if ((s >= hi_mem_min))
                                goto done;
                            switch (mem[s].hh.u.B0) {
                            case 10:
                                {
                                    v = mem[s + 1].hh.v.LH;
                                    break_width[1] = break_width[1] - mem[v + 1].cint;
                                    break_width[2 + mem[v].hh.u.B0] = break_width[2 + mem[v].hh.u.B0] - mem[v + 2].cint;
                                    break_width[6] = break_width[6] - mem[v + 3].cint;
                                }
                                break;
                            case 12:
                                ;
                                break;
                            case 9:
                                break_width[1] = break_width[1] - mem[s + 1].cint;
                                break;
                            case 11:
                                if (mem[s].hh.u.B1 != EXPLICIT)
                                    goto done;
                                else
                                    break_width[1] = break_width[1] - mem[s + 1].cint;
                                break;
                            default:
                                goto done;
                                break;
                            }
                            s = mem[s].hh.v.RH;
                        }
                    done:
                        ;
                    }
                    if (mem[prev_r].hh.u.B0 == DELTA_NODE) {
                        mem[prev_r + 1].cint = mem[prev_r + 1].cint - cur_active_width[1] + break_width[1];
                        mem[prev_r + 2].cint = mem[prev_r + 2].cint - cur_active_width[2] + break_width[2];
                        mem[prev_r + 3].cint = mem[prev_r + 3].cint - cur_active_width[3] + break_width[3];
                        mem[prev_r + 4].cint = mem[prev_r + 4].cint - cur_active_width[4] + break_width[4];
                        mem[prev_r + 5].cint = mem[prev_r + 5].cint - cur_active_width[5] + break_width[5];
                        mem[prev_r + 6].cint = mem[prev_r + 6].cint - cur_active_width[6] + break_width[6];
                    } else if (prev_r == mem_top - 7) {
                        active_width[1] = break_width[1];
                        active_width[2] = break_width[2];
                        active_width[3] = break_width[3];
                        active_width[4] = break_width[4];
                        active_width[5] = break_width[5];
                        active_width[6] = break_width[6];
                    } else {

                        q = get_node(DELTA_NODE_SIZE);
                        mem[q].hh.v.RH = r;
                        mem[q].hh.u.B0 = DELTA_NODE;
                        mem[q].hh.u.B1 = 0;
                        mem[q + 1].cint = break_width[1] - cur_active_width[1];
                        mem[q + 2].cint = break_width[2] - cur_active_width[2];
                        mem[q + 3].cint = break_width[3] - cur_active_width[3];
                        mem[q + 4].cint = break_width[4] - cur_active_width[4];
                        mem[q + 5].cint = break_width[5] - cur_active_width[5];
                        mem[q + 6].cint = break_width[6] - cur_active_width[6];
                        mem[prev_r].hh.v.RH = q;
                        prev_prev_r = prev_r;
                        prev_r = q;
                    }
                    if (abs(INTPAR(adj_demerits)) >= MAX_HALFWORD - minimum_demerits)
                        minimum_demerits = 1073741822L;
                    else
                        minimum_demerits = minimum_demerits + abs(INTPAR(adj_demerits));
                    {
                        register integer for_end;
                        fit_class = VERY_LOOSE_FIT;
                        for_end = TIGHT_FIT;
                        if (fit_class <= for_end)
                            do {
                                if (minimal_demerits[fit_class] <= minimum_demerits) {  /*874: */
                                    q = get_node(PASSIVE_NODE_SIZE);
                                    mem[q].hh.v.RH = passive;
                                    passive = q;
                                    mem[q + 1].hh.v.RH = cur_p;
                                    mem[q + 1].hh.v.LH = best_place[fit_class];
                                    q = get_node(active_node_size);
                                    mem[q + 1].hh.v.RH = passive;
                                    mem[q + 1].hh.v.LH = best_pl_line[fit_class] + 1;
                                    mem[q].hh.u.B1 = fit_class;
                                    mem[q].hh.u.B0 = break_type;
                                    mem[q + 2].cint = minimal_demerits[fit_class];
                                    if (do_last_line_fit) {     /*1639: */
                                        mem[q + 3].cint = best_pl_short[fit_class];
                                        mem[q + 4].cint = best_pl_glue[fit_class];
                                    }
                                    mem[q].hh.v.RH = r;
                                    mem[prev_r].hh.v.RH = q;
                                    prev_r = q;
                                }
                                minimal_demerits[fit_class] = MAX_HALFWORD;
                            }
                            while (fit_class++ < for_end);
                    }
                    minimum_demerits = MAX_HALFWORD;
                    if (r != mem_top - 7) {
                        q = get_node(DELTA_NODE_SIZE);
                        mem[q].hh.v.RH = r;
                        mem[q].hh.u.B0 = DELTA_NODE;
                        mem[q].hh.u.B1 = 0;
                        mem[q + 1].cint = cur_active_width[1] - break_width[1];
                        mem[q + 2].cint = cur_active_width[2] - break_width[2];
                        mem[q + 3].cint = cur_active_width[3] - break_width[3];
                        mem[q + 4].cint = cur_active_width[4] - break_width[4];
                        mem[q + 5].cint = cur_active_width[5] - break_width[5];
                        mem[q + 6].cint = cur_active_width[6] - break_width[6];
                        mem[prev_r].hh.v.RH = q;
                        prev_prev_r = prev_r;
                        prev_r = q;
                    }
                }
                if (r == mem_top - 7)
                    goto exit;
                if (l > easy_line) {
                    line_width = second_width;
                    old_l = 1073741822L;
                } else {

                    old_l = l;
                    if (l > last_special_line)
                        line_width = second_width;
                    else if (LOCAL(par_shape) == MIN_HALFWORD)
                        line_width = first_width;
                    else
                        line_width = mem[LOCAL(par_shape) + 2 * l].cint;
                }
            }
        }
        {
            artificial_demerits = false;
            shortfall = line_width - cur_active_width[1];
            if (INTPAR(xetex_protrude_chars) > 1)
                shortfall = shortfall + total_pw(r, cur_p);
            if (shortfall > 0) {        /*881: */

                if ((cur_active_width[3] != 0) || (cur_active_width[4] != 0) || (cur_active_width[5] != 0)) {
                    if (do_last_line_fit) {
                        if (cur_p == MIN_HALFWORD) {     /*1634: */
                            if ((mem[r + 3].cint == 0) || (mem[r + 4].cint <= 0))
                                goto lab45;
                            if ((cur_active_width[3] != fill_width[0]) || (cur_active_width[4] != fill_width[1])
                                || (cur_active_width[5] != fill_width[2]))
                                goto lab45;
                            if (mem[r + 3].cint > 0)
                                g = cur_active_width[2];
                            else
                                g = cur_active_width[6];
                            if (g <= 0)
                                goto lab45;
                            arith_error = false;
                            g = fract(g, mem[r + 3].cint, mem[r + 4].cint, MAX_HALFWORD);
                            if (INTPAR(last_line_fit) < 1000)
                                g = fract(g, INTPAR(last_line_fit), 1000, MAX_HALFWORD);
                            if (arith_error) {

                                if (mem[r + 3].cint > 0)
                                    g = MAX_HALFWORD;
                                else
                                    g = -MAX_HALFWORD;
                            }
                            if (g > 0) {        /*1635: */
                                if (g > shortfall)
                                    g = shortfall;
                                if (g > 7230584L) {

                                    if (cur_active_width[2] < 1663497L) {
                                        b = INF_BAD;
                                        fit_class = VERY_LOOSE_FIT;
                                        goto lab40;
                                    }
                                }
                                b = badness(g, cur_active_width[2]);
                                if (b > 12) {

                                    if (b > 99)
                                        fit_class = VERY_LOOSE_FIT;
                                    else
                                        fit_class = LOOSE_FIT;
                                } else
                                    fit_class = DECENT_FIT;
                                goto lab40;
                            } else if (g < 0) { /*1636: */
                                if (-(integer) g > cur_active_width[6])
                                    g = -(integer) cur_active_width[6];
                                b = badness(-(integer) g, cur_active_width[6]);
                                if (b > 12)
                                    fit_class = TIGHT_FIT;
                                else
                                    fit_class = DECENT_FIT;
                                goto lab40;
                            }
 lab45:                        /*not_found */ ;
                        }
                        shortfall = 0;
                    }
                    b = 0;
                    fit_class = DECENT_FIT;
                } else {

                    if (shortfall > 7230584L) {

                        if (cur_active_width[2] < 1663497L) {
                            b = INF_BAD;
                            fit_class = VERY_LOOSE_FIT;
                            goto lab31;
                        }
                    }
                    b = badness(shortfall, cur_active_width[2]);
                    if (b > 12) {

                        if (b > 99)
                            fit_class = VERY_LOOSE_FIT;
                        else
                            fit_class = LOOSE_FIT;
                    } else
                        fit_class = DECENT_FIT;
 lab31:                        /*done1 */ ;
                }
            } else {            /*882: */

                if (-(integer) shortfall > cur_active_width[6])
                    b = (INF_BAD + 1);
                else
                    b = badness(-(integer) shortfall, cur_active_width[6]);
                if (b > 12)
                    fit_class = TIGHT_FIT;
                else
                    fit_class = DECENT_FIT;
            }
            if (do_last_line_fit) {     /*1637: */
                if (cur_p == MIN_HALFWORD)
                    shortfall = 0;
                if (shortfall > 0)
                    g = cur_active_width[2];
                else if (shortfall < 0)
                    g = cur_active_width[6];
                else
                    g = 0;
            }
 lab40:    /*found */ if ((b > INF_BAD) || (pi == -EJECT_PENALTY)) {   /*883: */
                if (final_pass && (minimum_demerits == MAX_HALFWORD) && (mem[r].hh.v.RH == mem_top - 7)
                    && (prev_r == mem_top - 7))
                    artificial_demerits = true;
                else if (b > threshold)
                    goto lab60;
                node_r_stays_active = false;
            } else {

                prev_r = r;
                if (b > threshold)
                    goto lab22;
                node_r_stays_active = true;
            }
            if (artificial_demerits)
                d = 0;
            else {              /*888: */

                d = INTPAR(line_penalty) + b;
                if (abs(d) >= 10000)
                    d = 100000000L;
                else
                    d = d * d;
                if (pi != 0) {

                    if (pi > 0)
                        d = d + pi * pi;
                    else if (pi > -EJECT_PENALTY)
                        d = d - pi * pi;
                }
                if ((break_type == HYPHENATED) && (mem[r].hh.u.B0 == HYPHENATED)) {

                    if (cur_p != MIN_HALFWORD)
                        d = d + INTPAR(double_hyphen_demerits);
                    else
                        d = d + INTPAR(final_hyphen_demerits);
                }
                if (abs(fit_class - mem[r].hh.u.B1) > 1)
                    d = d + INTPAR(adj_demerits);
            }
            d = d + mem[r + 2].cint;
            if (d <= minimal_demerits[fit_class]) {
                minimal_demerits[fit_class] = d;
                best_place[fit_class] = mem[r + 1].hh.v.RH;
                best_pl_line[fit_class] = l;
                if (do_last_line_fit) { /*1638: */
                    best_pl_short[fit_class] = shortfall;
                    best_pl_glue[fit_class] = g;
                }
                if (d < minimum_demerits)
                    minimum_demerits = d;
            }
            if (node_r_stays_active)
                goto lab22;

	lab60: /* deactivate *//*889: */
	    mem[prev_r].hh.v.RH = mem[r].hh.v.RH;
            free_node(r, active_node_size);
            if (prev_r == mem_top - 7) {        /*890: */
                r = mem[mem_top - 7].hh.v.RH;
                if (mem[r].hh.u.B0 == DELTA_NODE) {
                    active_width[1] = active_width[1] + mem[r + 1].cint;
                    active_width[2] = active_width[2] + mem[r + 2].cint;
                    active_width[3] = active_width[3] + mem[r + 3].cint;
                    active_width[4] = active_width[4] + mem[r + 4].cint;
                    active_width[5] = active_width[5] + mem[r + 5].cint;
                    active_width[6] = active_width[6] + mem[r + 6].cint;
                    cur_active_width[1] = active_width[1];
                    cur_active_width[2] = active_width[2];
                    cur_active_width[3] = active_width[3];
                    cur_active_width[4] = active_width[4];
                    cur_active_width[5] = active_width[5];
                    cur_active_width[6] = active_width[6];
                    mem[mem_top - 7].hh.v.RH = mem[r].hh.v.RH;
                    free_node(r, DELTA_NODE_SIZE);
                }
            } else if (mem[prev_r].hh.u.B0 == DELTA_NODE) {
                r = mem[prev_r].hh.v.RH;
                if (r == mem_top - 7) {
                    cur_active_width[1] = cur_active_width[1] - mem[prev_r + 1].cint;
                    cur_active_width[2] = cur_active_width[2] - mem[prev_r + 2].cint;
                    cur_active_width[3] = cur_active_width[3] - mem[prev_r + 3].cint;
                    cur_active_width[4] = cur_active_width[4] - mem[prev_r + 4].cint;
                    cur_active_width[5] = cur_active_width[5] - mem[prev_r + 5].cint;
                    cur_active_width[6] = cur_active_width[6] - mem[prev_r + 6].cint;
                    mem[prev_prev_r].hh.v.RH = mem_top - 7;
                    free_node(prev_r, DELTA_NODE_SIZE);
                    prev_r = prev_prev_r;
                } else if (mem[r].hh.u.B0 == DELTA_NODE) {
                    cur_active_width[1] = cur_active_width[1] + mem[r + 1].cint;
                    cur_active_width[2] = cur_active_width[2] + mem[r + 2].cint;
                    cur_active_width[3] = cur_active_width[3] + mem[r + 3].cint;
                    cur_active_width[4] = cur_active_width[4] + mem[r + 4].cint;
                    cur_active_width[5] = cur_active_width[5] + mem[r + 5].cint;
                    cur_active_width[6] = cur_active_width[6] + mem[r + 6].cint;
                    mem[prev_r + 1].cint = mem[prev_r + 1].cint + mem[r + 1].cint;
                    mem[prev_r + 2].cint = mem[prev_r + 2].cint + mem[r + 2].cint;
                    mem[prev_r + 3].cint = mem[prev_r + 3].cint + mem[r + 3].cint;
                    mem[prev_r + 4].cint = mem[prev_r + 4].cint + mem[r + 4].cint;
                    mem[prev_r + 5].cint = mem[prev_r + 5].cint + mem[r + 5].cint;
                    mem[prev_r + 6].cint = mem[prev_r + 6].cint + mem[r + 6].cint;
                    mem[prev_r].hh.v.RH = mem[r].hh.v.RH;
                    free_node(r, DELTA_NODE_SIZE);
                }
            }
        }
    }

exit:
    ;
}

void post_line_break(boolean d)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t q, r, s;
    int32_t p, k;
    scaled w;
    boolean glue_break;
    int32_t ptmp;
    boolean disc_break;
    boolean post_disc_break;
    scaled cur_width;
    scaled cur_indent;
    uint16_t t;
    integer pen;
    int32_t cur_line;
    int32_t LR_ptr;

    LR_ptr = cur_list.eTeX_aux;
    q = mem[best_bet + 1].hh.v.RH;
    cur_p = MIN_HALFWORD;

    do {
        r = q;
        q = mem[q + 1].hh.v.LH;
        mem[r + 1].hh.v.LH = cur_p;
        cur_p = r;
    } while (!(q == MIN_HALFWORD /*:907 */ ));

    cur_line = cur_list.pg + 1;
    do {
        /*909: */ if ((eqtb[ETEX_STATE_BASE].cint > 0)) {      /*1494: */
            q = mem[mem_top - 3].hh.v.RH;
            if (LR_ptr != MIN_HALFWORD) {
                temp_ptr = LR_ptr;
                r = q;
                do {
                    s = new_math(0, (mem[temp_ptr].hh.v.LH - 1));
                    mem[s].hh.v.RH = r;
                    r = s;
                    temp_ptr = mem[temp_ptr].hh.v.RH;
                } while (!(temp_ptr == MIN_HALFWORD));
                mem[mem_top - 3].hh.v.RH = r;
            }
            while (q != mem[cur_p + 1].hh.v.RH) {

                if (!(q >= hi_mem_min)) {

                    if (mem[q].hh.u.B0 == MATH_NODE) {    /*1495: */

                        if (odd(mem[q].hh.u.B1)) {
                            if (LR_ptr != MIN_HALFWORD) {

                                if (mem[LR_ptr].hh.v.LH == (L_CODE * (mem[q].hh.u.B1 / L_CODE) + 3)) {
                                    temp_ptr = LR_ptr;
                                    LR_ptr = mem[temp_ptr].hh.v.RH;
                                    {
                                        mem[temp_ptr].hh.v.RH = avail;
                                        avail = temp_ptr;
                                    }
                                }
                            }
                        } else {

                            temp_ptr = get_avail();
                            mem[temp_ptr].hh.v.LH = (L_CODE * (mem[q].hh.u.B1 / L_CODE) + 3);
                            mem[temp_ptr].hh.v.RH = LR_ptr;
                            LR_ptr = temp_ptr;
                        }
                    }
                }
                q = mem[q].hh.v.RH;
            }
        }
        q = mem[cur_p + 1].hh.v.RH;
        disc_break = false;
        post_disc_break = false;
        glue_break = false;
        if (q != MIN_HALFWORD) {

            if (mem[q].hh.u.B0 == GLUE_NODE) {
                delete_glue_ref(mem[q + 1].hh.v.LH);
                mem[q + 1].hh.v.LH = GLUEPAR(right_skip);
                mem[q].hh.u.B1 = (GLUE_PAR__right_skip + 1);
                mem[GLUEPAR(right_skip)].hh.v.RH++;
                glue_break = true;
                goto done;
            } else {

                if (mem[q].hh.u.B0 == DISC_NODE) {        /*911: */
                    t = mem[q].hh.u.B1;
                    if (t == 0)
                        r = mem[q].hh.v.RH;
                    else {

                        r = q;
                        while (t > 1) {

                            r = mem[r].hh.v.RH;
                            t--;
                        }
                        s = mem[r].hh.v.RH;
                        r = mem[s].hh.v.RH;
                        mem[s].hh.v.RH = MIN_HALFWORD;
                        flush_node_list(mem[q].hh.v.RH);
                        mem[q].hh.u.B1 = 0;
                    }
                    if (mem[q + 1].hh.v.RH != MIN_HALFWORD) {    /*913: */
                        s = mem[q + 1].hh.v.RH;
                        while (mem[s].hh.v.RH != MIN_HALFWORD)
                            s = mem[s].hh.v.RH;
                        mem[s].hh.v.RH = r;
                        r = mem[q + 1].hh.v.RH;
                        mem[q + 1].hh.v.RH = MIN_HALFWORD;
                        post_disc_break = true;
                    }
                    if (mem[q + 1].hh.v.LH != MIN_HALFWORD) {    /*914: */
                        s = mem[q + 1].hh.v.LH;
                        mem[q].hh.v.RH = s;
                        while (mem[s].hh.v.RH != MIN_HALFWORD)
                            s = mem[s].hh.v.RH;
                        mem[q + 1].hh.v.LH = MIN_HALFWORD;
                        q = s;
                    }
                    mem[q].hh.v.RH = r;
                    disc_break = true;
                } else if (mem[q].hh.u.B0 == KERN_NODE)
                    mem[q + 1].cint = 0;
                else if (mem[q].hh.u.B0 == MATH_NODE) {
                    mem[q + 1].cint = 0;
                    if ((eqtb[ETEX_STATE_BASE].cint > 0)) {    /*1495: */

                        if (odd(mem[q].hh.u.B1)) {
                            if (LR_ptr != MIN_HALFWORD) {

                                if (mem[LR_ptr].hh.v.LH == (L_CODE * (mem[q].hh.u.B1 / L_CODE) + 3)) {
                                    temp_ptr = LR_ptr;
                                    LR_ptr = mem[temp_ptr].hh.v.RH;
                                    {
                                        mem[temp_ptr].hh.v.RH = avail;
                                        avail = temp_ptr;
                                    }
                                }
                            }
                        } else {

                            temp_ptr = get_avail();
                            mem[temp_ptr].hh.v.LH = (L_CODE * (mem[q].hh.u.B1 / L_CODE) + 3);
                            mem[temp_ptr].hh.v.RH = LR_ptr;
                            LR_ptr = temp_ptr;
                        }
                    }
                }
            }
        } else {

            q = mem_top - 3;
            while (mem[q].hh.v.RH != MIN_HALFWORD)
                q = mem[q].hh.v.RH;
        }
    done:
        if (INTPAR(xetex_protrude_chars) > 0) {
            if (disc_break && ((q >= hi_mem_min) || (mem[q].hh.u.B0 != DISC_NODE))) {
                p = q;
                ptmp = p;
            } else {

                p = prev_rightmost(mem[mem_top - 3].hh.v.RH, q);
                ptmp = p;
                p = find_protchar_right(mem[mem_top - 3].hh.v.RH, p);
            }
            w = char_pw(p, 1);
            if (w != 0) {
                k = new_margin_kern(-(integer) w, last_rightmost_char, 1);
                mem[k].hh.v.RH = mem[ptmp].hh.v.RH;
                mem[ptmp].hh.v.RH = k;
                if ((ptmp == q))
                    q = mem[q].hh.v.RH;
            }
        }
        if (!glue_break) {
            r = new_param_glue(GLUE_PAR__right_skip);
            mem[r].hh.v.RH = mem[q].hh.v.RH;
            mem[q].hh.v.RH = r;
            q = /*:915 */ r;
        }
        if ((eqtb[ETEX_STATE_BASE].cint > 0)) {        /*1496: */

            if (LR_ptr != MIN_HALFWORD) {
                s = mem_top - 3;
                r = mem[s].hh.v.RH;
                while (r != q) {

                    s = r;
                    r = mem[s].hh.v.RH;
                }
                r = LR_ptr;
                while (r != MIN_HALFWORD) {

                    temp_ptr = new_math(0, mem[r].hh.v.LH);
                    mem[s].hh.v.RH = temp_ptr;
                    s = temp_ptr;
                    r = mem[r].hh.v.RH;
                }
                mem[s].hh.v.RH = q;
            }
        }
        r = mem[q].hh.v.RH;
        mem[q].hh.v.RH = MIN_HALFWORD;
        q = mem[mem_top - 3].hh.v.RH;
        mem[mem_top - 3].hh.v.RH = r;
        if (INTPAR(xetex_protrude_chars) > 0) {
            p = q;
            p = find_protchar_left(p, false);
            w = char_pw(p, 0);
            if (w != 0) {
                k = new_margin_kern(-(integer) w, last_leftmost_char, 0);
                mem[k].hh.v.RH = q;
                q = k;
            }
        }
        if (GLUEPAR(left_skip) != 0) {
            r = new_param_glue(GLUE_PAR__left_skip);
            mem[r].hh.v.RH = q;
            q = r;
        }
        if (cur_line > last_special_line) {
            cur_width = second_width;
            cur_indent = second_indent;
        } else if (LOCAL(par_shape) == MIN_HALFWORD) {
            cur_width = first_width;
            cur_indent = first_indent;
        } else {

            cur_width = mem[LOCAL(par_shape) + 2 * cur_line].cint;
            cur_indent = mem[LOCAL(par_shape) + 2 * cur_line - 1].cint;
        }
        adjust_tail = mem_top - 5;
        pre_adjust_tail = mem_top - 14;
        just_box = hpack(q, cur_width, EXACTLY);
        mem[just_box + 4].cint = /*:918 */ cur_indent;
        if (mem_top - 14 != pre_adjust_tail) {
            mem[cur_list.tail].hh.v.RH = mem[mem_top - 14].hh.v.RH;
            cur_list.tail = pre_adjust_tail;
        }
        pre_adjust_tail = MIN_HALFWORD;
        append_to_vlist(just_box);
        if (mem_top - 5 != adjust_tail) {
            mem[cur_list.tail].hh.v.RH = mem[mem_top - 5].hh.v.RH;
            cur_list.tail = adjust_tail;
        }
        adjust_tail = MIN_HALFWORD /*:917 */ ;
        if (cur_line + 1 != best_line) {
            q = eqtb[INTER_LINE_PENALTIES_LOC].hh.v.RH;
            if (q != MIN_HALFWORD) {
                r = cur_line;
                if (r > mem[q + 1].cint)
                    r = mem[q + 1].cint;
                pen = mem[q + r + 1].cint;
            } else
                pen = INTPAR(inter_line_penalty);
            q = eqtb[CLUB_PENALTIES_LOC].hh.v.RH;
            if (q != MIN_HALFWORD) {
                r = cur_line - cur_list.pg;
                if (r > mem[q + 1].cint)
                    r = mem[q + 1].cint;
                pen = pen + mem[q + r + 1].cint;
            } else if (cur_line == cur_list.pg + 1)
                pen = pen + INTPAR(club_penalty);
            if (d)
                q = eqtb[DISPLAY_WIDOW_PENALTIES_LOC].hh.v.RH;
            else
                q = eqtb[WIDOW_PENALTIES_LOC].hh.v.RH;
            if (q != MIN_HALFWORD) {
                r = best_line - cur_line - 1;
                if (r > mem[q + 1].cint)
                    r = mem[q + 1].cint;
                pen = pen + mem[q + r + 1].cint;
            } else if (cur_line + 2 == best_line) {

                if (d)
                    pen = pen + INTPAR(display_widow_penalty);
                else
                    pen = pen + INTPAR(widow_penalty);
            }
            if (disc_break)
                pen = pen + INTPAR(broken_penalty);
            if (pen != 0) {
                r = new_penalty(pen);
                mem[cur_list.tail].hh.v.RH = r;
                cur_list.tail = r;
            }
        }
        cur_line++;
        cur_p = mem[cur_p + 1].hh.v.LH;
        if (cur_p != MIN_HALFWORD) {

            if (!post_disc_break) {     /*908: */
                r = mem_top - 3;
                while (true) {

                    q = mem[r].hh.v.RH;
                    if (q == mem[cur_p + 1].hh.v.RH)
                        goto lab31;
                    if ((q >= hi_mem_min))
                        goto lab31;
                    if ((mem[q].hh.u.B0 < MATH_NODE))
                        goto lab31;
                    if (mem[q].hh.u.B0 == KERN_NODE) {

                        if ((mem[q].hh.u.B1 != EXPLICIT) && (mem[q].hh.u.B1 != SPACE_ADJUSTMENT))
                            goto lab31;
                    }
                    r = q;
                    if (mem[q].hh.u.B0 == MATH_NODE) {

                        if ((eqtb[ETEX_STATE_BASE].cint > 0)) {        /*1495: */

                            if (odd(mem[q].hh.u.B1)) {
                                if (LR_ptr != MIN_HALFWORD) {

                                    if (mem[LR_ptr].hh.v.LH == (L_CODE * (mem[q].hh.u.B1 / L_CODE) + 3)) {
                                        temp_ptr = LR_ptr;
                                        LR_ptr = mem[temp_ptr].hh.v.RH;
                                        {
                                            mem[temp_ptr].hh.v.RH = avail;
                                            avail = temp_ptr;
                                        }
                                    }
                                }
                            } else {

                                temp_ptr = get_avail();
                                mem[temp_ptr].hh.v.LH = (L_CODE * (mem[q].hh.u.B1 / L_CODE) + 3);
                                mem[temp_ptr].hh.v.RH = LR_ptr;
                                LR_ptr = temp_ptr;
                            }
                        }
                    }
                }
 lab31:                        /*done1 */ if (r != mem_top - 3) {
                    mem[r].hh.v.RH = MIN_HALFWORD;
                    flush_node_list(mem[mem_top - 3].hh.v.RH);
                    mem[mem_top - 3].hh.v.RH = q;
                }
            }
        }
    } while (!(cur_p == MIN_HALFWORD));
    if ((cur_line != best_line) || (mem[mem_top - 3].hh.v.RH != MIN_HALFWORD))
        confusion(S(line_breaking));
    cur_list.pg = best_line - 1;
    cur_list.eTeX_aux = LR_ptr;
}

small_number reconstitute(small_number j, small_number n, int32_t bchar, int32_t hchar)
{
    register small_number Result;
    memory_word *mem = zmem; int32_t p;
    int32_t t;
    four_quarters q;
    int32_t cur_rh;
    int32_t test_char;
    scaled w;
    font_index k;
    hyphen_passed = 0;
    t = mem_top - 4;
    w = 0;
    mem[mem_top - 4].hh.v.RH = MIN_HALFWORD;
    cur_l = hu[j];
    cur_q = t;
    if (j == 0) {
        ligature_present = init_lig;
        p = init_list;
        if (ligature_present)
            lft_hit = init_lft;
        while (p > MIN_HALFWORD) {

            {
                mem[t].hh.v.RH = get_avail();
                t = mem[t].hh.v.RH;
                mem[t].hh.u.B0 = hf;
                mem[t].hh.u.B1 = mem[p].hh.u.B1;
            }
            p = mem[p].hh.v.RH;
        }
    } else if (cur_l < TOO_BIG_CHAR) {
        mem[t].hh.v.RH = get_avail();
        t = mem[t].hh.v.RH;
        mem[t].hh.u.B0 = hf;
        mem[t].hh.u.B1 = cur_l;
    }
    lig_stack = MIN_HALFWORD;
    {
        if (j < n)
            cur_r = hu[j + 1];
        else
            cur_r = bchar;
        if (odd(hyf[j]))
            cur_rh = hchar;
        else
            cur_rh = TOO_BIG_CHAR;
    }
 lab22:/*continue *//*944: */ if (cur_l == TOO_BIG_CHAR) {
        k = bchar_label[hf];
        if (k == NON_ADDRESS)
            goto done;
        else
            q = font_info[k].qqqq;
    } else {

        q = font_info[char_base[hf] + effective_char(true, hf, cur_l)].qqqq;
        if (((q.u.B2) % 4) != LIG_TAG)
            goto done;
        k = lig_kern_base[hf] + q.u.B3;
        q = font_info[k].qqqq;
        if (q.u.B0 > 128) {
            k = lig_kern_base[hf] + 256 * q.u.B2 + q.u.B3 + 32768L - 256 * (128);
            q = font_info[k].qqqq;
        }
    }
    if (cur_rh < TOO_BIG_CHAR)
        test_char = cur_rh;
    else
        test_char = cur_r;
    while (true) {

        if (q.u.B1 == test_char) {

            if (q.u.B0 <= 128) {

                if (cur_rh < TOO_BIG_CHAR) {
                    hyphen_passed = j;
                    hchar = TOO_BIG_CHAR;
                    cur_rh = TOO_BIG_CHAR;
                    goto lab22;
                } else {

                    if (hchar < TOO_BIG_CHAR) {

                        if (odd(hyf[j])) {
                            hyphen_passed = j;
                            hchar = TOO_BIG_CHAR;
                        }
                    }
                    if (q.u.B2 < 128) {   /*946: */
                        if (cur_l == TOO_BIG_CHAR)
                            lft_hit = true;
                        if (j == n) {

                            if (lig_stack == MIN_HALFWORD)
                                rt_hit = true;
                        }
                        switch (q.u.B2) {
                        case 1:
                        case 5:
                            {
                                cur_l = q.u.B3;
                                ligature_present = true;
                            }
                            break;
                        case 2:
                        case 6:
                            {
                                cur_r = q.u.B3;
                                if (lig_stack > MIN_HALFWORD)
                                    mem[lig_stack].hh.u.B1 = cur_r;
                                else {

                                    lig_stack = new_lig_item(cur_r);
                                    if (j == n)
                                        bchar = TOO_BIG_CHAR;
                                    else {

                                        p = get_avail();
                                        mem[lig_stack + 1].hh.v.RH = p;
                                        mem[p].hh.u.B1 = hu[j + 1];
                                        mem[p].hh.u.B0 = hf;
                                    }
                                }
                            }
                            break;
                        case 3:
                            {
                                cur_r = q.u.B3;
                                p = lig_stack;
                                lig_stack = new_lig_item(cur_r);
                                mem[lig_stack].hh.v.RH = p;
                            }
                            break;
                        case 7:
                        case 11:
                            {
                                if (ligature_present) {
                                    p = new_ligature(hf, cur_l, mem[cur_q].hh.v.RH);
                                    if (lft_hit) {
                                        mem[p].hh.u.B1 = 2;
                                        lft_hit = false;
                                    }
                                    if (false) {

                                        if (lig_stack == MIN_HALFWORD) {
                                            mem[p].hh.u.B1++;
                                            rt_hit = false;
                                        }
                                    }
                                    mem[cur_q].hh.v.RH = p;
                                    t = p;
                                    ligature_present = false;
                                }
                                cur_q = t;
                                cur_l = q.u.B3;
                                ligature_present = true;
                            }
                            break;
                        default:
                            {
                                cur_l = q.u.B3;
                                ligature_present = true;
                                if (lig_stack > MIN_HALFWORD) {
                                    if (mem[lig_stack + 1].hh.v.RH > MIN_HALFWORD) {
                                        mem[t].hh.v.RH = mem[lig_stack + 1].hh.v.RH;
                                        t = mem[t].hh.v.RH;
                                        j++;
                                    }
                                    p = lig_stack;
                                    lig_stack = mem[p].hh.v.RH;
                                    free_node(p, SMALL_NODE_SIZE);
                                    if (lig_stack == MIN_HALFWORD) {
                                        if (j < n)
                                            cur_r = hu[j + 1];
                                        else
                                            cur_r = bchar;
                                        if (odd(hyf[j]))
                                            cur_rh = hchar;
                                        else
                                            cur_rh = TOO_BIG_CHAR;
                                    } else
                                        cur_r = mem[lig_stack].hh.u.B1;
                                } else if (j == n)
                                    goto done;
                                else {

                                    {
                                        mem[t].hh.v.RH = get_avail();
                                        t = mem[t].hh.v.RH;
                                        mem[t].hh.u.B0 = hf;
                                        mem[t].hh.u.B1 = cur_r;
                                    }
                                    j++;
                                    {
                                        if (j < n)
                                            cur_r = hu[j + 1];
                                        else
                                            cur_r = bchar;
                                        if (odd(hyf[j]))
                                            cur_rh = hchar;
                                        else
                                            cur_rh = TOO_BIG_CHAR;
                                    }
                                }
                            }
                            break;
                        }
                        if (q.u.B2 > 4) {

                            if (q.u.B2 != 7)
                                goto done;
                        }
                        goto lab22;
                    }
                    w = font_info[kern_base[hf] + 256 * q.u.B2 + q.u.B3].cint;
                    goto done;
                }
            }
        }
        if (q.u.B0 >= 128) {

            if (cur_rh == TOO_BIG_CHAR)
                goto done;
            else {

                cur_rh = TOO_BIG_CHAR;
                goto lab22;
            }
        }
        k = k + q.u.B0 + 1;
        q = font_info[k].qqqq;
    } /*:944*/
done:
    if (ligature_present) {
        p = new_ligature(hf, cur_l, mem[cur_q].hh.v.RH);
        if (lft_hit) {
            mem[p].hh.u.B1 = 2;
            lft_hit = false;
        }
        if (rt_hit) {

            if (lig_stack == MIN_HALFWORD) {
                mem[p].hh.u.B1++;
                rt_hit = false;
            }
        }
        mem[cur_q].hh.v.RH = p;
        t = p;
        ligature_present = false;
    }
    if (w != 0) {
        mem[t].hh.v.RH = new_kern(w);
        t = mem[t].hh.v.RH;
        w = 0;
        mem[t + 2].hh.v.LH = 0;
    }
    if (lig_stack > MIN_HALFWORD) {
        cur_q = t;
        cur_l = mem[lig_stack].hh.u.B1;
        ligature_present = true;
        {
            if (mem[lig_stack + 1].hh.v.RH > MIN_HALFWORD) {
                mem[t].hh.v.RH = mem[lig_stack + 1].hh.v.RH;
                t = mem[t].hh.v.RH;
                j++;
            }
            p = lig_stack;
            lig_stack = mem[p].hh.v.RH;
            free_node(p, SMALL_NODE_SIZE);
            if (lig_stack == MIN_HALFWORD) {
                if (j < n)
                    cur_r = hu[j + 1];
                else
                    cur_r = bchar;
                if (odd(hyf[j]))
                    cur_rh = hchar;
                else
                    cur_rh = TOO_BIG_CHAR;
            } else
                cur_r = mem[lig_stack].hh.u.B1;
        }
        goto lab22;
    }
    Result = j;
    return Result;
}

void hyphenate(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    short /*hyphenatable_length_limit 2 */ i, j, l;
    int32_t q, r, s;
    int32_t bchar;
    int32_t major_tail, minor_tail;
    UnicodeScalar c;
    short /*hyphenatable_length_limit */ c_loc;
    integer r_count;
    int32_t hyf_node;
    trie_pointer z;
    integer v;
    hyph_pointer h;
    str_number k;
    pool_pointer u;

    {
        register integer for_end;
        j = 0;
        for_end = hn;
        if (j <= for_end)
            do
                hyf[j] = 0;
            while (j++ < for_end);
    }
    h = hc[1];
    hn++;
    hc[hn] = cur_lang;
    {
        register integer for_end;
        j = 2;
        for_end = hn;
        if (j <= for_end)
            do
                h = (h + h + hc[j]) % HYPH_PRIME;
            while (j++ < for_end);
    }
    while (true) {

        k = hyph_word[h];
        if (k == 0)
            goto lab45;
        if (length(k) == hn) {
            j = 1;
            u = str_start[(k) - 65536L];
            do {
                if (str_pool[u] != hc[j])
                    goto done;
                j++;
                u++;
            } while (!(j > hn));
            s = hyph_list[h];
            while (s != MIN_HALFWORD) {

                hyf[mem[s].hh.v.LH] = 1;
                s = mem[s].hh.v.RH;
            }
            hn--;
            goto lab40;
        } /*:966 */
    done:
        h = hyph_link[h];
        if (h == 0)
            goto lab45;
        h--;
    }
 lab45:                        /*not_found */ hn--;
    if (trie_trc[cur_lang + 1] != cur_lang)
        return;
    hc[0] = 0;
    hc[hn + 1] = 0;
    hc[hn + 2] = max_hyph_char;
    {
        register integer for_end;
        j = 0;
        for_end = hn - r_hyf + 1;
        if (j <= for_end)
            do {
                z = trie_trl[cur_lang + 1] + hc[j];
                l = j;
                while (hc[l] == trie_trc[z]) {

                    if (trie_tro[z] != min_trie_op) {   /*959: */
                        v = trie_tro[z];
                        do {
                            v = v + op_start[cur_lang];
                            i = l - hyf_distance[v];
                            if (hyf_num[v] > hyf[i])
                                hyf[i] = hyf_num[v];
                            v = hyf_next[v];
                        } while (!(v == min_trie_op));
                    }
                    l++;
                    z = trie_trl[z] + hc[l];
                }
            }
            while (j++ < for_end);
    }
 lab40:                        /*found */  {
        register integer for_end;
        j = 0;
        for_end = l_hyf - 1;
        if (j <= for_end)
            do
                hyf[j] = 0;
            while (j++ < for_end);
    }
    {
        register integer for_end;
        j = 0;
        for_end = r_hyf - 1;
        if (j <= for_end)
            do
                hyf[hn - j] = 0 /*:958 */ ;
            while (j++ < for_end);
    }
    {
        register integer for_end;
        j = l_hyf;
        for_end = hn - r_hyf;
        if (j <= for_end)
            do
                if (odd(hyf[j]))
                    goto lab41;
            while (j++ < for_end) ;
    }
    return;
 lab41:                        /*found1 *//*:936 */ ;
    if ((((ha) != MIN_HALFWORD && (!(ha >= hi_mem_min)) && (mem[ha].hh.u.B0 == WHATSIT_NODE)
          && ((mem[ha].hh.u.B1 == NATIVE_WORD_NODE) || (mem[ha].hh.u.B1 == NATIVE_WORD_NODE_AT))))) {
        s = cur_p;
        while (mem[s].hh.v.RH != ha)
            s = mem[s].hh.v.RH;
        hyphen_passed = 0;
        {
            register integer for_end;
            j = l_hyf;
            for_end = hn - r_hyf;
            if (j <= for_end)
                do {
                    if (odd(hyf[j])) {
                        q = new_native_word_node(hf, j - hyphen_passed);
                        mem[q].hh.u.B1 = mem[ha].hh.u.B1;
                        {
                            register integer for_end;
                            i = 0;
                            for_end = j - hyphen_passed - 1;
                            if (i <= for_end)
                                do
                                    set_native_char(q, i, get_native_char(ha, i + hyphen_passed));
                                while (i++ < for_end);
                        }
                        set_native_metrics(q, (STATEINT(xetex_use_glyph_metrics) > 0));
                        mem[s].hh.v.RH = q;
                        s = q;
                        q = new_disc();
                        mem[q + 1].hh.v.LH = new_native_character(hf, hyf_char);
                        mem[s].hh.v.RH = q;
                        s = q;
                        hyphen_passed = j;
                    }
                }
                while (j++ < for_end);
        }
        hn = mem[ha + 4].qqqq.u.B2;
        q = new_native_word_node(hf, hn - hyphen_passed);
        mem[q].hh.u.B1 = mem[ha].hh.u.B1;
        {
            register integer for_end;
            i = 0;
            for_end = hn - hyphen_passed - 1;
            if (i <= for_end)
                do
                    set_native_char(q, i, get_native_char(ha, i + hyphen_passed));
                while (i++ < for_end);
        }
        set_native_metrics(q, (STATEINT(xetex_use_glyph_metrics) > 0));
        mem[s].hh.v.RH = q;
        s = q;
        q = mem[ha].hh.v.RH;
        mem[s].hh.v.RH = q;
        mem[ha].hh.v.RH = MIN_HALFWORD;
        flush_node_list(ha);
    } else {

        q = mem[hb].hh.v.RH;
        mem[hb].hh.v.RH = MIN_HALFWORD;
        r = mem[ha].hh.v.RH;
        mem[ha].hh.v.RH = MIN_HALFWORD;
        bchar = hyf_bchar;
        if ((ha >= hi_mem_min)) {

            if (mem[ha].hh.u.B0 != hf)
                goto lab42;
            else {

                init_list = ha;
                init_lig = false;
                hu[0] = mem[ha].hh.u.B1;
            }
        } else if (mem[ha].hh.u.B0 == LIGATURE_NODE) {

            if (mem[ha + 1].hh.u.B0 != hf)
                goto lab42;
            else {

                init_list = mem[ha + 1].hh.v.RH;
                init_lig = true;
                init_lft = (mem[ha].hh.u.B1 > 1);
                hu[0] = mem[ha + 1].hh.u.B1;
                if (init_list == MIN_HALFWORD) {

                    if (init_lft) {
                        hu[0] = max_hyph_char;
                        init_lig = false;
                    }
                }
                free_node(ha, SMALL_NODE_SIZE);
            }
        } else {

            if (!(r >= hi_mem_min)) {

                if (mem[r].hh.u.B0 == LIGATURE_NODE) {

                    if (mem[r].hh.u.B1 > 1)
                        goto lab42;
                }
            }
            j = 1;
            s = ha;
            init_list = MIN_HALFWORD;
            goto lab50;
        }
        s = cur_p;
        while (mem[s].hh.v.RH != ha)
            s = mem[s].hh.v.RH;
        j = 0;
        goto lab50;
 lab42:                        /*found2 */ s = ha;
        j = 0;
        hu[0] = max_hyph_char;
        init_lig = false;
        init_list = MIN_HALFWORD;
 lab50:                        /*common_ending */ flush_node_list(r);
        do {
            l = j;
            j = reconstitute(j, hn, bchar, hyf_char) + 1;
            if (hyphen_passed == 0) {
                mem[s].hh.v.RH = mem[mem_top - 4].hh.v.RH;
                while (mem[s].hh.v.RH > MIN_HALFWORD)
                    s = mem[s].hh.v.RH;
                if (odd(hyf[j - 1])) {
                    l = j;
                    hyphen_passed = j - 1;
                    mem[mem_top - 4].hh.v.RH = MIN_HALFWORD;
                }
            }
            if (hyphen_passed > 0)      /*949: */
                do {
                    r = get_node(SMALL_NODE_SIZE);
                    mem[r].hh.v.RH = mem[mem_top - 4].hh.v.RH;
                    mem[r].hh.u.B0 = DISC_NODE;
                    major_tail = r;
                    r_count = 0;
                    while (mem[major_tail].hh.v.RH > MIN_HALFWORD) {

                        major_tail = mem[major_tail].hh.v.RH;
                        r_count++;
                    }
                    i = hyphen_passed;
                    hyf[i] = 0;
                    minor_tail = MIN_HALFWORD;
                    mem[r + 1].hh.v.LH = MIN_HALFWORD;
                    hyf_node = new_character(hf, hyf_char);
                    if (hyf_node != MIN_HALFWORD) {
                        i++;
                        c = hu[i];
                        hu[i] = hyf_char;
                        {
                            mem[hyf_node].hh.v.RH = avail;
                            avail = hyf_node;
                        }
                    }
                    while (l <= i) {

                        l = reconstitute(l, i, font_bchar[hf], TOO_BIG_CHAR) + 1;
                        if (mem[mem_top - 4].hh.v.RH > MIN_HALFWORD) {
                            if (minor_tail == MIN_HALFWORD)
                                mem[r + 1].hh.v.LH = mem[mem_top - 4].hh.v.RH;
                            else
                                mem[minor_tail].hh.v.RH = mem[mem_top - 4].hh.v.RH;
                            minor_tail = mem[mem_top - 4].hh.v.RH;
                            while (mem[minor_tail].hh.v.RH > MIN_HALFWORD)
                                minor_tail = mem[minor_tail].hh.v.RH;
                        }
                    }
                    if (hyf_node != MIN_HALFWORD) {
                        hu[i] = c;
                        l = i;
                        i--;
                    }
                    minor_tail = MIN_HALFWORD;
                    mem[r + 1].hh.v.RH = MIN_HALFWORD;
                    c_loc = 0;
                    if (bchar_label[hf] != NON_ADDRESS) {
                        l--;
                        c = hu[l];
                        c_loc = l;
                        hu[l] = max_hyph_char;
                    }
                    while (l < j) {

                        do {
                            l = reconstitute(l, hn, bchar, TOO_BIG_CHAR) + 1;
                            if (c_loc > 0) {
                                hu[c_loc] = c;
                                c_loc = 0;
                            }
                            if (mem[mem_top - 4].hh.v.RH > MIN_HALFWORD) {
                                if (minor_tail == MIN_HALFWORD)
                                    mem[r + 1].hh.v.RH = mem[mem_top - 4].hh.v.RH;
                                else
                                    mem[minor_tail].hh.v.RH = mem[mem_top - 4].hh.v.RH;
                                minor_tail = mem[mem_top - 4].hh.v.RH;
                                while (mem[minor_tail].hh.v.RH > MIN_HALFWORD)
                                    minor_tail = mem[minor_tail].hh.v.RH;
                            }
                        } while (!(l >= j));
                        while (l > j) { /*952: */

                            j = reconstitute(j, hn, bchar, TOO_BIG_CHAR) + 1;
                            mem[major_tail].hh.v.RH = mem[mem_top - 4].hh.v.RH;
                            while (mem[major_tail].hh.v.RH > MIN_HALFWORD) {

                                major_tail = mem[major_tail].hh.v.RH;
                                r_count++;
                            }
                        }
                    }
                    if (r_count > 127) {
                        mem[s].hh.v.RH = mem[r].hh.v.RH;
                        mem[r].hh.v.RH = MIN_HALFWORD;
                        flush_node_list(r);
                    } else {

                        mem[s].hh.v.RH = r;
                        mem[r].hh.u.B1 = r_count;
                    }
                    s = /*:953 */ major_tail;
                    hyphen_passed = j - 1;
                    mem[mem_top - 4].hh.v.RH = MIN_HALFWORD;
                } while (!(!odd(hyf[j - 1]) /*:949 */ ));
        } while (!(j > hn));
        mem[s].hh.v.RH = /*:948 */ q;
        flush_list(init_list);
    }
}

integer max_hyphenatable_length(void)
{
    CACHE_THE_EQTB;

    if (STATEINT(xetex_hyphenatable_length) > HYPHENATABLE_LENGTH_LIMIT)
        return HYPHENATABLE_LENGTH_LIMIT;
    return STATEINT(xetex_hyphenatable_length);
}

boolean eTeX_enabled(boolean b, uint16_t j, int32_t k)
{
    register boolean Result;
    if (!b) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Improper_));
        }
        print_cmd_chr(j, k);
        {
            help_ptr = 1;
            help_line[0] = S(Sorry__this_optional_e_TeX_f/*eature has been disabled.*/);
        }
        error();
    }
    Result = b;
    return Result;
}

void show_save_groups(void)
{
    memory_word *mem = zmem; integer p;
    short /*mmode */ m;
    save_pointer v;
    uint16_t l;
    group_code c;
    signed char a;
    integer i;
    uint16_t j;
    str_number s;
    p = nest_ptr;
    nest[p] = cur_list;
    v = save_ptr;
    l = cur_level;
    c = cur_group;
    save_ptr = cur_boundary;
    cur_level--;
    a = 1;
    print_nl(S());
    print_ln();
    while (true) {

        print_nl(S(____/*"### "*/));
        print_group(true);
        if (cur_group == BOTTOM_LEVEL)
            goto done;
        do {
            m = nest[p].mode;
            if (p > 0)
                p--;
            else
                m = VMODE;
        } while (!(m != HMODE));
        print(S(___Z2/*" ("*/));
        switch (cur_group) {
        case 1:
            {
                p++;
                goto lab42;
            }
            break;
        case 2:
        case 3:
            s = S(hbox);
            break;
        case 4:
            s = S(vbox);
            break;
        case 5:
            s = S(vtop);
            break;
        case 6:
            if (a == 0) {
                if (m == -1)
                    s = S(halign);
                else
                    s = S(valign);
                a = 1;
                goto lab41;
            } else {

                if (a == 1)
                    print(S(align_entry));
                else
                    print_esc(S(cr));
                if (p >= a)
                    p = p - a;
                a = 0;
                goto lab40;
            }
            break;
        case 7:
            {
                p++;
                a = -1;
                print_esc(S(noalign));
                goto lab42;
            }
            break;
        case 8:
            {
                print_esc(S(output));
                goto lab40;
            }
            break;
        case 9:
            goto lab42;
            break;
        case 10:
        case 13:
            {
                if (cur_group == DISC_GROUP)
                    print_esc(S(discretionary));
                else
                    print_esc(S(mathchoice));
                {
                    register integer for_end;
                    i = 1;
                    for_end = 3;
                    if (i <= for_end)
                        do
                            if (i <= save_stack[save_ptr - 2].cint)
                                print(66232L /*"__" */ );
                        while (i++ < for_end) ;
                }
                goto lab42;
            }
            break;
        case 11:
            {
                if (save_stack[save_ptr - 2].cint == 255)
                    print_esc(S(vadjust));
                else {

                    print_esc(S(insert));
                    print_int(save_stack[save_ptr - 2].cint);
                }
                goto lab42;
            }
            break;
        case 12:
            {
                s = S(vcenter);
                goto lab41;
            }
            break;
        case 14:
            {
                p++;
                print_esc(S(begingroup));
                goto lab40;
            }
            break;
        case 15:
            {
                if (m == MMODE)
                    print_char(36 /*"$" */ );
                else if (nest[p].mode == MMODE) {
                    print_cmd_chr(EQ_NO, save_stack[save_ptr - 2].cint);
                    goto lab40;
                }
                print_char(36 /*"$" */ );
                goto lab40;
            }
            break;
        case 16:
            {
                if (mem[nest[p + 1].eTeX_aux].hh.u.B0 == LEFT_NOAD)
                    print_esc(S(left));
                else
                    print_esc(S(middle));
                goto lab40;
            }
            break;
        }
        i = save_stack[save_ptr - 4].cint;
        if (i != 0) {

            if (i < 1073741824L) {
                if (abs(nest[p].mode) == VMODE)
                    j = HMOVE;
                else
                    j = VMOVE;
                if (i > 0)
                    print_cmd_chr(j, 0);
                else
                    print_cmd_chr(j, 1);
                print_scaled(abs(i));
                print(S(pt));
            } else if (i < 1073807360L) {
                if (i >= 1073774592L) {
                    print_esc(S(global));
                    i = i - (32768L);
                }
                print_esc(S(setbox));
                print_int(i - 1073741824L);
                print_char(61 /*"=" */ );
            } else
                print_cmd_chr(LEADER_SHIP, i - (1073807261L));
        }
 lab41:                        /*found1 */ print_esc(s);
        if (save_stack[save_ptr - 2].cint != 0) {
            print_char(32 /*" " */ );
            if (save_stack[save_ptr - 3].cint == EXACTLY)
                print(S(to));
            else
                print(S(spread));
            print_scaled(save_stack[save_ptr - 2].cint);
            print(S(pt));
        }
 lab42:/*found2 */ print_char(123 /*"_" */ );
 lab40:/*found */ print_char(41 /*")" */ );
        cur_level--;
        cur_group = save_stack[save_ptr].hh.u.B1;
        save_ptr = save_stack[save_ptr].hh.v.RH;
    }
done:
    save_ptr = v;
    cur_level = l;
    cur_group = c;
}

int32_t vert_break(int32_t p, scaled h, scaled d)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t prev_p;
    int32_t q, r;
    integer pi;
    integer b;
    integer least_cost;
    int32_t best_place;
    scaled prev_dp;
    small_number t;
    prev_p = p;
    least_cost = MAX_HALFWORD;
    active_width[1] = 0;
    active_width[2] = 0;
    active_width[3] = 0;
    active_width[4] = 0;
    active_width[5] = 0;
    active_width[6] = 0;
    prev_dp = 0;
    while (true) {

        if (p == MIN_HALFWORD)
            pi = -EJECT_PENALTY;
        else                    /*1008: */
            switch (mem[p].hh.u.B0) {
            case 0:
            case 1:
            case 2:
                {
                    active_width[1] = active_width[1] + prev_dp + mem[p + 3].cint;
                    prev_dp = mem[p + 2].cint;
                    goto lab45;
                }
                break;
            case 8:
                {
                    if ((mem[p].hh.u.B1 == PIC_NODE) || (mem[p].hh.u.B1 == PDF_NODE)) {
                        active_width[1] = active_width[1] + prev_dp + mem[p + 3].cint;
                        prev_dp = mem[p + 2].cint;
                    }
                    goto lab45;
                }
                break;
            case 10:
                if ((mem[prev_p].hh.u.B0 < MATH_NODE))
                    pi = 0;
                else
                    goto lab90;
                break;
            case 11:
                {
                    if (mem[p].hh.v.RH == MIN_HALFWORD)
                        t = PENALTY_NODE;
                    else
                        t = mem[mem[p].hh.v.RH].hh.u.B0;
                    if (t == GLUE_NODE)
                        pi = 0;
                    else
                        goto lab90;
                }
                break;
            case 12:
                pi = mem[p + 1].cint;
                break;
            case 4:
            case 3:
                goto lab45;
                break;
            default:
                confusion(S(vertbreak));
                break;
            }
        if (pi < INF_PENALTY) {
            if (active_width[1] < h) {

                if ((active_width[3] != 0) || (active_width[4] != 0) || (active_width[5] != 0))
                    b = 0;
                else
                    b = badness(h - active_width[1], active_width[2]);
            } else if (active_width[1] - h > active_width[6])
                b = MAX_HALFWORD;
            else
                b = badness(active_width[1] - h, active_width[6]) /*:1010 */ ;
            if (b < MAX_HALFWORD) {

                if (pi <= -EJECT_PENALTY)
                    b = pi;
                else if (b < INF_BAD)
                    b = b + pi;
                else
                    b = 100000L;
            }
            if (b <= least_cost) {
                best_place = p;
                least_cost = b;
                best_height_plus_depth = active_width[1] + prev_dp;
            }
            if ((b == MAX_HALFWORD) || (pi <= -EJECT_PENALTY))
                goto done;
        }
        if ((mem[p].hh.u.B0 < GLUE_NODE) || (mem[p].hh.u.B0 > KERN_NODE))
            goto lab45;
 lab90:/*update_heights *//*1011: */ if (mem[p].hh.u.B0 == KERN_NODE)
            q = p;
        else {

            q = mem[p + 1].hh.v.LH;
            active_width[2 + mem[q].hh.u.B0] = active_width[2 + mem[q].hh.u.B0] + mem[q + 2].cint;
            active_width[6] = active_width[6] + mem[q + 3].cint;
            if ((mem[q].hh.u.B1 != NORMAL) && (mem[q + 3].cint != 0)) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Infinite_glue_shrinkage_foun_Z1/*"Infinite glue shrinkage found in box being split"*/));
                }
                {
                    help_ptr = 4;
                    help_line[3] = S(The_box_you_are__vsplitting_/*contains some infinitely*/);
                    help_line[2] = S(shrinkable_glue__e_g_____vss/*' or `\vskip 0pt minus 1fil'.*/);
                    help_line[1] = S(Such_glue_doesn_t_belong_the_Z1/*"Such glue doesn't belong there; but you can safely proceed,"*/);
                    help_line[0] = S(since_the_offensive_shrinkab/*ility has been made finite.*/);
                }
                error();
                r = new_spec(q);
                mem[r].hh.u.B1 = NORMAL;
                delete_glue_ref(q);
                mem[p + 1].hh.v.LH = r;
                q = r;
            }
        }
        active_width[1] = active_width[1] + prev_dp + mem[q + 1].cint;
        prev_dp = 0 /*:1011 */ ;
 lab45:                        /*not_found */ if (prev_dp > d) {
            active_width[1] = active_width[1] + prev_dp - d;
            prev_dp = d;
        }
        prev_p = p;
        p = mem[prev_p].hh.v.RH;
    }
done:
    Result = best_place;
    return Result;
}

int32_t vsplit(int32_t n, scaled h)
{
    CACHE_THE_EQTB;
    register int32_t Result;
    memory_word *mem = zmem;
    int32_t v;
    int32_t p;
    int32_t q;

    cur_val = n;
    if (cur_val < 256)
        v = BOX_REG(cur_val);
    else {

        find_sa_element(4, cur_val, false);
        if (cur_ptr == MIN_HALFWORD)
            v = MIN_HALFWORD;
        else
            v = mem[cur_ptr + 1].hh.v.RH;
    }
    flush_node_list(disc_ptr[VSPLIT_CODE]);
    disc_ptr[VSPLIT_CODE] = MIN_HALFWORD;
    if (sa_root[MARK_VAL] != MIN_HALFWORD) {

        if (do_marks(0, 0, sa_root[MARK_VAL]))
            sa_root[MARK_VAL] = MIN_HALFWORD;
    }
    if (cur_mark[SPLIT_FIRST_MARK_CODE] != MIN_HALFWORD) {
        delete_token_ref(cur_mark[SPLIT_FIRST_MARK_CODE]);
        cur_mark[SPLIT_FIRST_MARK_CODE] = MIN_HALFWORD;
        delete_token_ref(cur_mark[SPLIT_BOT_MARK_CODE]);
        cur_mark[SPLIT_BOT_MARK_CODE] = MIN_HALFWORD;
    }
    if (v == MIN_HALFWORD) {
        Result = MIN_HALFWORD;
        return Result;
    }
    if (mem[v].hh.u.B0 != VLIST_NODE) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S());
        }
        print_esc(S(vsplit));
        print(S(_needs_a_));
        print_esc(S(vbox));
        {
            help_ptr = 2;
            help_line[1] = S(The_box_you_are_trying_to_sp/*lit is an \hbox.*/);
            help_line[0] = S(I_can_t_split_such_a_box__so/* I'll leave it alone.*/);
        }
        error();
        Result = MIN_HALFWORD;
        return Result;
    }
    q = vert_break(mem[v + 5].hh.v.RH, h, DIMENPAR(split_max_depth));
    p = mem[v + 5].hh.v.RH;
    if (p == q)
        mem[v + 5].hh.v.RH = MIN_HALFWORD;
    else
        while (true) {

            if (mem[p].hh.u.B0 == MARK_NODE) {

                if (mem[p + 1].hh.v.LH != 0) {  /*1615: */
                    find_sa_element(MARK_VAL, mem[p + 1].hh.v.LH, true);
                    if (mem[cur_ptr + 2].hh.v.RH == MIN_HALFWORD) {
                        mem[cur_ptr + 2].hh.v.RH = mem[p + 1].hh.v.RH;
                        mem[mem[p + 1].hh.v.RH].hh.v.LH++;
                    } else
                        delete_token_ref(mem[cur_ptr + 3].hh.v.LH);
                    mem[cur_ptr + 3].hh.v.LH = mem[p + 1].hh.v.RH;
                    mem[mem[p + 1].hh.v.RH].hh.v.LH++;
                } else if (cur_mark[SPLIT_FIRST_MARK_CODE] == MIN_HALFWORD) {
                    cur_mark[SPLIT_FIRST_MARK_CODE] = mem[p + 1].hh.v.RH;
                    cur_mark[SPLIT_BOT_MARK_CODE] = cur_mark[SPLIT_FIRST_MARK_CODE];
                    mem[cur_mark[SPLIT_FIRST_MARK_CODE]].hh.v.LH =
                        mem[cur_mark[SPLIT_FIRST_MARK_CODE]].hh.v.LH + 2;
                } else {

                    delete_token_ref(cur_mark[SPLIT_BOT_MARK_CODE]);
                    cur_mark[SPLIT_BOT_MARK_CODE] = mem[p + 1].hh.v.RH;
                    mem[cur_mark[SPLIT_BOT_MARK_CODE]].hh.v.LH++;
                }
            }
            if (mem[p].hh.v.RH == q) {
                mem[p].hh.v.RH = MIN_HALFWORD;
                goto done;
            }
            p = mem[p].hh.v.RH;
        } /*:1014*/
done:
    q = prune_page_top(q, INTPAR(saving_vdiscards) > 0);
    p = mem[v + 5].hh.v.RH;
    free_node(v, BOX_NODE_SIZE);
    if (q != MIN_HALFWORD)
        q = vpackage(q, 0, ADDITIONAL, MAX_HALFWORD);
    if (cur_val < 256)
        BOX_REG(cur_val) = q;
    else {

        find_sa_element(4, cur_val, false);
        if (cur_ptr != MIN_HALFWORD) {
            mem[cur_ptr + 1].hh.v.RH = q;
            mem[cur_ptr + 1].hh.v.LH++;
            delete_sa_ref(cur_ptr);
        }
    }
    Result = vpackage(p, h, EXACTLY, DIMENPAR(split_max_depth));
    return Result;
}

void print_totals(void)
{
    print_scaled(page_so_far[1]);
    if (page_so_far[2] != 0) {
        print(S(_plus_));
        print_scaled(page_so_far[2]);
        print(S());
    }
    if (page_so_far[3] != 0) {
        print(S(_plus_));
        print_scaled(page_so_far[3]);
        print(S(fil));
    }
    if (page_so_far[4] != 0) {
        print(S(_plus_));
        print_scaled(page_so_far[4]);
        print(S(fill));
    }
    if (page_so_far[5] != 0) {
        print(S(_plus_));
        print_scaled(page_so_far[5]);
        print(S(filll));
    }
    if (page_so_far[6] != 0) {
        print(S(_minus_));
        print_scaled(page_so_far[6]);
    }
}

void freeze_page_specs(small_number s)
{
    CACHE_THE_EQTB;

    page_contents = s;
    page_so_far[0] = DIMENPAR(vsize);
    page_max_depth = DIMENPAR(max_depth);
    page_so_far[7] = 0;
    page_so_far[1] = 0;
    page_so_far[2] = 0;
    page_so_far[3] = 0;
    page_so_far[4] = 0;
    page_so_far[5] = 0;
    page_so_far[6] = 0;
    least_page_cost = MAX_HALFWORD;
}

void box_error(eight_bits n)
{
    CACHE_THE_EQTB;

    error();
    begin_diagnostic();
    print_nl(S(The_following_box_has_been_d/*eleted:*/));
    show_box(BOX_REG(n));
    end_diagnostic(true);
    flush_node_list(BOX_REG(n));
    BOX_REG(n) = MIN_HALFWORD;
}

void ensure_vbox(eight_bits n)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;
    p = BOX_REG(n);

    if (p != MIN_HALFWORD) {

        if (mem[p].hh.u.B0 == HLIST_NODE) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Insertions_can_only_be_added/* to a vbox*/));
            }
            {
                help_ptr = 3;
                help_line[2] = S(Tut_tut__You_re_trying_to__i/*nsert into a*/);
                help_line[1] = S(_box_register_that_now_conta/*ins an \hbox.*/);
                help_line[0] = S(Proceed__and_I_ll_discard_it/*s present contents.*/);
            }
            box_error(n);
        }
    }
}

void fire_up(int32_t c)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p, q, r, s;
    int32_t prev_p;
    unsigned char /*biggest_reg */ n;
    boolean wait;
    integer save_vbadness;
    scaled save_vfuzz;
    int32_t save_split_top_skip;

    if (mem[best_page_break].hh.u.B0 == PENALTY_NODE) {
        geq_word_define(INT_BASE + INT_PAR__output_penalty, mem[best_page_break + 1].cint);
        mem[best_page_break + 1].cint = INF_PENALTY;
    } else
        geq_word_define(INT_BASE + INT_PAR__output_penalty, INF_PENALTY);
    if (sa_root[MARK_VAL] != MIN_HALFWORD) {

        if (do_marks(1, 0, sa_root[MARK_VAL]))
            sa_root[MARK_VAL] = MIN_HALFWORD;
    }
    if (cur_mark[BOT_MARK_CODE] != MIN_HALFWORD) {
        if (cur_mark[TOP_MARK_CODE] != MIN_HALFWORD)
            delete_token_ref(cur_mark[TOP_MARK_CODE]);
        cur_mark[TOP_MARK_CODE] = cur_mark[BOT_MARK_CODE];
        mem[cur_mark[TOP_MARK_CODE]].hh.v.LH++;
        delete_token_ref(cur_mark[FIRST_MARK_CODE]);
        cur_mark[FIRST_MARK_CODE] = MIN_HALFWORD;
    }
    if (c == best_page_break)
        best_page_break = MIN_HALFWORD;
    if (BOX_REG(255) != MIN_HALFWORD) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S());
        }
        print_esc(S(box));
        print(S(255_is_not_void));
        {
            help_ptr = 2;
            help_line[1] = S(You_shouldn_t_use__box255_ex/*cept in \output routines.*/);
            help_line[0] = S(Proceed__and_I_ll_discard_it/*s present contents.*/);
        }
        box_error(255);
    }
    insert_penalties = 0;
    save_split_top_skip = GLUEPAR(split_top_skip);
    if (INTPAR(holding_inserts) <= 0) {   /*1053: */
        r = mem[mem_top].hh.v.RH;
        while (r != mem_top) {

            if (mem[r + 2].hh.v.LH != MIN_HALFWORD) {
                n = mem[r].hh.u.B1;
                ensure_vbox(n);
                if (BOX_REG(n) == MIN_HALFWORD)
                    BOX_REG(n) = new_null_box();
                p = BOX_REG(n) + 5;
                while (mem[p].hh.v.RH != MIN_HALFWORD)
                    p = mem[p].hh.v.RH;
                mem[r + 2].hh.v.RH = p;
            }
            r = mem[r].hh.v.RH;
        }
    }
    q = mem_top - 4;
    mem[q].hh.v.RH = MIN_HALFWORD;
    prev_p = mem_top - 2;
    p = mem[prev_p].hh.v.RH;
    while (p != best_page_break) {

        if (mem[p].hh.u.B0 == INS_NODE) {
            if (INTPAR(holding_inserts) <= 0) {   /*1055: */
                r = mem[mem_top].hh.v.RH;
                while (mem[r].hh.u.B1 != mem[p].hh.u.B1)
                    r = mem[r].hh.v.RH;
                if (mem[r + 2].hh.v.LH == MIN_HALFWORD)
                    wait = true;
                else {

                    wait = false;
                    s = mem[r + 2].hh.v.RH;
                    mem[s].hh.v.RH = mem[p + 4].hh.v.LH;
                    if (mem[r + 2].hh.v.LH == p) {      /*1056: */
                        if (mem[r].hh.u.B0 == SPLIT_UP) {

                            if ((mem[r + 1].hh.v.LH == p) && (mem[r + 1].hh.v.RH != MIN_HALFWORD)) {
                                while (mem[s].hh.v.RH != mem[r + 1].hh.v.RH)
                                    s = mem[s].hh.v.RH;
                                mem[s].hh.v.RH = MIN_HALFWORD;
                                GLUEPAR(split_top_skip) = mem[p + 4].hh.v.RH;
                                mem[p + 4].hh.v.LH = prune_page_top(mem[r + 1].hh.v.RH, false);
                                if (mem[p + 4].hh.v.LH != MIN_HALFWORD) {
                                    temp_ptr = vpackage(mem[p + 4].hh.v.LH, 0, ADDITIONAL, MAX_HALFWORD);
                                    mem[p + 3].cint = mem[temp_ptr + 3].cint + mem[temp_ptr + 2].cint;
                                    free_node(temp_ptr, BOX_NODE_SIZE);
                                    wait = true;
                                }
                            }
                        }
                        mem[r + 2].hh.v.LH = MIN_HALFWORD;
                        n = mem[r].hh.u.B1;
                        temp_ptr = mem[BOX_REG(n) + 5].hh.v.RH;
                        free_node(BOX_REG(n), BOX_NODE_SIZE);
                        BOX_REG(n) =
                            vpackage(temp_ptr, 0, ADDITIONAL, MAX_HALFWORD);
                    } else {

                        while (mem[s].hh.v.RH != MIN_HALFWORD)
                            s = mem[s].hh.v.RH;
                        mem[r + 2].hh.v.RH = s;
                    }
                }
                mem[prev_p].hh.v.RH = mem[p].hh.v.RH;
                mem[p].hh.v.RH = MIN_HALFWORD;
                if (wait) {
                    mem[q].hh.v.RH = p;
                    q = p;
                    insert_penalties++;
                } else {

                    delete_glue_ref(mem[p + 4].hh.v.RH);
                    free_node(p, INS_NODE_SIZE);
                }
                p = /*:1057 */ prev_p;
            }
        } else if (mem[p].hh.u.B0 == MARK_NODE) {

            if (mem[p + 1].hh.v.LH != 0) {      /*1618: */
                find_sa_element(MARK_VAL, mem[p + 1].hh.v.LH, true);
                if (mem[cur_ptr + 1].hh.v.RH == MIN_HALFWORD) {
                    mem[cur_ptr + 1].hh.v.RH = mem[p + 1].hh.v.RH;
                    mem[mem[p + 1].hh.v.RH].hh.v.LH++;
                }
                if (mem[cur_ptr + 2].hh.v.LH != MIN_HALFWORD)
                    delete_token_ref(mem[cur_ptr + 2].hh.v.LH);
                mem[cur_ptr + 2].hh.v.LH = mem[p + 1].hh.v.RH;
                mem[mem[p + 1].hh.v.RH].hh.v.LH++;
            } else {            /*1051: */

                if (cur_mark[FIRST_MARK_CODE] == MIN_HALFWORD) {
                    cur_mark[FIRST_MARK_CODE] = mem[p + 1].hh.v.RH;
                    mem[cur_mark[FIRST_MARK_CODE]].hh.v.LH++;
                }
                if (cur_mark[BOT_MARK_CODE] != MIN_HALFWORD)
                    delete_token_ref(cur_mark[BOT_MARK_CODE]);
                cur_mark[BOT_MARK_CODE] = mem[p + 1].hh.v.RH;
                mem[cur_mark[BOT_MARK_CODE]].hh.v.LH++;
            }
        }
        prev_p = p;
        p = mem[prev_p].hh.v.RH;
    }
    GLUEPAR(split_top_skip) = save_split_top_skip;
    if (p != MIN_HALFWORD) {
        if (mem[mem_top - 1].hh.v.RH == MIN_HALFWORD) {

            if (nest_ptr == 0)
                cur_list.tail = page_tail;
            else
                nest[0].tail = page_tail;
        }
        mem[page_tail].hh.v.RH = mem[mem_top - 1].hh.v.RH;
        mem[mem_top - 1].hh.v.RH = p;
        mem[prev_p].hh.v.RH = MIN_HALFWORD;
    }
    save_vbadness = INTPAR(vbadness);
    INTPAR(vbadness) = INF_BAD;
    save_vfuzz = DIMENPAR(vfuzz);
    DIMENPAR(vfuzz) = MAX_HALFWORD;
    BOX_REG(255) =
        vpackage(mem[mem_top - 2].hh.v.RH, best_size, EXACTLY, page_max_depth);
    INTPAR(vbadness) = save_vbadness;
    DIMENPAR(vfuzz) = save_vfuzz;
    if (last_glue != MAX_HALFWORD)
        delete_glue_ref(last_glue);
    page_contents = EMPTY;
    page_tail = mem_top - 2;
    mem[mem_top - 2].hh.v.RH = MIN_HALFWORD;
    last_glue = MAX_HALFWORD;
    last_penalty = 0;
    last_kern = 0;
    last_node_type = -1;
    page_so_far[7] = 0;
    page_max_depth = 0 /*:1026 */ ;
    if (q != mem_top - 4) {
        mem[mem_top - 2].hh.v.RH = mem[mem_top - 4].hh.v.RH;
        page_tail = q;
    }
    r = mem[mem_top].hh.v.RH;
    while (r != mem_top) {

        q = mem[r].hh.v.RH;
        free_node(r, PAGE_INS_NODE_SIZE);
        r = q;
    }
    mem[mem_top].hh.v.RH = /*:1054 *//*:1049 */ mem_top;
    if (sa_root[MARK_VAL] != MIN_HALFWORD) {

        if (do_marks(2, 0, sa_root[MARK_VAL]))
            sa_root[MARK_VAL] = MIN_HALFWORD;
    }
    if ((cur_mark[TOP_MARK_CODE] != MIN_HALFWORD) && (cur_mark[FIRST_MARK_CODE] == MIN_HALFWORD)) {
        cur_mark[FIRST_MARK_CODE] = cur_mark[TOP_MARK_CODE];
        mem[cur_mark[TOP_MARK_CODE]].hh.v.LH++;
    }
    if (LOCAL(output_routine) != MIN_HALFWORD) {

        if (dead_cycles >= INTPAR(max_dead_cycles)) {     /*1059: */
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Output_loop___));
            }
            print_int(dead_cycles);
            print(S(_consecutive_dead_cycles));
            {
                help_ptr = 3;
                help_line[2] = S(I_ve_concluded_that_your__ou/*tput is awry; it never does a*/);
                help_line[1] = S(_shipout__so_I_m_shipping__b/*ox255 out myself. Next time*/);
                help_line[0] = S(increase__maxdeadcycles_if_y/*ou want me to be more patient!*/);
            }
            error();
        } else {                /*1060: */

            output_active = true;
            dead_cycles++;
            push_nest();
            cur_list.mode = -1;
            cur_list.aux.cint = -65536000L;
            cur_list.ml = -(integer) line;
            begin_token_list(LOCAL(output_routine), OUTPUT_TEXT);
            new_save_level(OUTPUT_GROUP);
            normal_paragraph();
            scan_left_brace();
            return;
        }
    }
    {
        if (mem[mem_top - 2].hh.v.RH != MIN_HALFWORD) {
            if (mem[mem_top - 1].hh.v.RH == MIN_HALFWORD) {

                if (nest_ptr == 0)
                    cur_list.tail = page_tail;
                else
                    nest[0].tail = page_tail;
            } else
                mem[page_tail].hh.v.RH = mem[mem_top - 1].hh.v.RH;
            mem[mem_top - 1].hh.v.RH = mem[mem_top - 2].hh.v.RH;
            mem[mem_top - 2].hh.v.RH = MIN_HALFWORD;
            page_tail = mem_top - 2;
        }
        flush_node_list(disc_ptr[LAST_BOX_CODE]);
        disc_ptr[LAST_BOX_CODE] = MIN_HALFWORD;
        ship_out(BOX_REG(255));
        BOX_REG(255) = MIN_HALFWORD;
    }
}

void build_page(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;
    int32_t q, r;
    integer b, c;
    integer pi;
    unsigned char /*biggest_reg */ n;
    scaled delta, h, w;

    if ((mem[mem_top - 1].hh.v.RH == MIN_HALFWORD) || output_active)
        return;
    do {
 lab22:                        /*continue */ p = mem[mem_top - 1].hh.v.RH;
        if (last_glue != MAX_HALFWORD)
            delete_glue_ref(last_glue);
        last_penalty = 0;
        last_kern = 0;
        last_node_type = mem[p].hh.u.B0 + 1;
        if (mem[p].hh.u.B0 == GLUE_NODE) {
            last_glue = mem[p + 1].hh.v.LH;
            mem[last_glue].hh.v.RH++;
        } else {

            last_glue = MAX_HALFWORD;
            if (mem[p].hh.u.B0 == PENALTY_NODE)
                last_penalty = mem[p + 1].cint;
            else if (mem[p].hh.u.B0 == KERN_NODE)
                last_kern = mem[p + 1].cint;
        }
        switch (mem[p].hh.u.B0) {
        case 0:
        case 1:
        case 2:
            if (page_contents < BOX_THERE) {    /*1036: */
                if (page_contents == EMPTY)
                    freeze_page_specs(BOX_THERE);
                else
                    page_contents = BOX_THERE;
                q = new_skip_param(GLUE_PAR__top_skip);
                if (mem[temp_ptr + 1].cint > mem[p + 3].cint)
                    mem[temp_ptr + 1].cint = mem[temp_ptr + 1].cint - mem[p + 3].cint;
                else
                    mem[temp_ptr + 1].cint = 0;
                mem[q].hh.v.RH = p;
                mem[mem_top - 1].hh.v.RH = q;
                goto lab22;
            } else {            /*1037: */

                page_so_far[1] = page_so_far[1] + page_so_far[7] + mem[p + 3].cint;
                page_so_far[7] = mem[p + 2].cint;
                goto lab80;
            }
            break;
        case 8:
            {
                if ((mem[p].hh.u.B1 == PIC_NODE) || (mem[p].hh.u.B1 == PDF_NODE)) {
                    page_so_far[1] = page_so_far[1] + page_so_far[7] + mem[p + 3].cint;
                    page_so_far[7] = mem[p + 2].cint;
                }
                goto lab80;
            }
            break;
        case 10:
            if (page_contents < BOX_THERE)
                goto lab31;
            else if ((mem[page_tail].hh.u.B0 < MATH_NODE))
                pi = 0;
            else
                goto lab90;
            break;
        case 11:
            if (page_contents < BOX_THERE)
                goto lab31;
            else if (mem[p].hh.v.RH == MIN_HALFWORD)
                return;
            else if (mem[mem[p].hh.v.RH].hh.u.B0 == GLUE_NODE)
                pi = 0;
            else
                goto lab90;
            break;
        case 12:
            if (page_contents < BOX_THERE)
                goto lab31;
            else
                pi = mem[p + 1].cint;
            break;
        case 4:
            goto lab80;
            break;
        case 3:
            {
                if (page_contents == EMPTY)
                    freeze_page_specs(INSERTS_ONLY);
                n = mem[p].hh.u.B1;
                r = mem_top;
                while (n >= mem[mem[r].hh.v.RH].hh.u.B1)
                    r = mem[r].hh.v.RH;
                n = n;
                if (mem[r].hh.u.B1 != n) {        /*1044: */
                    q = get_node(PAGE_INS_NODE_SIZE);
                    mem[q].hh.v.RH = mem[r].hh.v.RH;
                    mem[r].hh.v.RH = q;
                    r = q;
                    mem[r].hh.u.B1 = n;
                    mem[r].hh.u.B0 = INSERTING;
                    ensure_vbox(n);
                    if (BOX_REG(n) == MIN_HALFWORD)
                        mem[r + 3].cint = 0;
                    else
                        mem[r + 3].cint =
                            mem[BOX_REG(n) + 3].cint +
                            mem[BOX_REG(n) + 2].cint;
                    mem[r + 2].hh.v.LH = MIN_HALFWORD;
                    q = SKIP_REG(n);
                    if (COUNT_REG(n) == 1000)
                        h = mem[r + 3].cint;
                    else
                        h = x_over_n(mem[r + 3].cint, 1000) * COUNT_REG(n);
                    page_so_far[0] = page_so_far[0] - h - mem[q + 1].cint;
                    page_so_far[2 + mem[q].hh.u.B0] = page_so_far[2 + mem[q].hh.u.B0] + mem[q + 2].cint;
                    page_so_far[6] = page_so_far[6] + mem[q + 3].cint;
                    if ((mem[q].hh.u.B1 != NORMAL) && (mem[q + 3].cint != 0)) {
                        {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Infinite_glue_shrinkage_inse/*rted from */));
                        }
                        print_esc(S(skip));
                        print_int(n);
                        {
                            help_ptr = 3;
                            help_line[2] = S(The_correction_glue_for_page/* breaking with insertions*/);
                            help_line[1] = S(must_have_finite_shrinkabili/*ty. But you may proceed,*/);
                            help_line[0] = S(since_the_offensive_shrinkab/*ility has been made finite.*/);
                        }
                        error();
                    }
                }
                if (mem[r].hh.u.B0 == SPLIT_UP)
                    insert_penalties = insert_penalties + mem[p + 1].cint;
                else {

                    mem[r + 2].hh.v.RH = p;
                    delta = page_so_far[0] - page_so_far[1] - page_so_far[7] + page_so_far[6];
                    if (COUNT_REG(n) == 1000)
                        h = mem[p + 3].cint;
                    else
                        h = x_over_n(mem[p + 3].cint, 1000) * COUNT_REG(n);
                    if (((h <= 0) || (h <= delta))
                        && (mem[p + 3].cint + mem[r + 3].cint <= SCALED_REG(n))) {
                        page_so_far[0] = page_so_far[0] - h;
                        mem[r + 3].cint = mem[r + 3].cint + mem[p + 3].cint;
                    } else {    /*1045: */

                        if (COUNT_REG(n) <= 0)
                            w = MAX_HALFWORD;
                        else {

                            w = page_so_far[0] - page_so_far[1] - page_so_far[7];
                            if (COUNT_REG(n) != 1000)
                                w = x_over_n(w, COUNT_REG(n)) * 1000;
                        }
                        if (w > SCALED_REG(n) - mem[r + 3].cint)
                            w = SCALED_REG(n) - mem[r + 3].cint;
                        q = vert_break(mem[p + 4].hh.v.LH, w, mem[p + 2].cint);
                        mem[r + 3].cint = mem[r + 3].cint + best_height_plus_depth;
                        if (COUNT_REG(n) != 1000)
                            best_height_plus_depth =
                                x_over_n(best_height_plus_depth, 1000) * COUNT_REG(n);
                        page_so_far[0] = page_so_far[0] - best_height_plus_depth;
                        mem[r].hh.u.B0 = SPLIT_UP;
                        mem[r + 1].hh.v.RH = q;
                        mem[r + 1].hh.v.LH = p;
                        if (q == MIN_HALFWORD)
                            insert_penalties = insert_penalties - 10000;
                        else if (mem[q].hh.u.B0 == PENALTY_NODE)
                            insert_penalties = insert_penalties + mem[q + 1].cint;
                    }
                }
                goto lab80;
            }
            break;
        default:
            confusion(S(page));
            break;
        }
        if (pi < INF_PENALTY) {
            if (page_so_far[1] < page_so_far[0]) {

                if ((page_so_far[3] != 0) || (page_so_far[4] != 0) || (page_so_far[5] != 0))
                    b = 0;
                else
                    b = badness(page_so_far[0] - page_so_far[1], page_so_far[2]);
            } else if (page_so_far[1] - page_so_far[0] > page_so_far[6])
                b = MAX_HALFWORD;
            else
                b = badness(page_so_far[1] - page_so_far[0], page_so_far[6]) /*:1042 */ ;
            if (b < MAX_HALFWORD) {

                if (pi <= -EJECT_PENALTY)
                    c = pi;
                else if (b < INF_BAD)
                    c = b + pi + insert_penalties;
                else
                    c = 100000L;
            } else
                c = b;
            if (insert_penalties >= 10000)
                c = MAX_HALFWORD;
            if (c <= least_page_cost) {
                best_page_break = p;
                best_size = page_so_far[0];
                least_page_cost = c;
                r = mem[mem_top].hh.v.RH;
                while (r != mem_top) {

                    mem[r + 2].hh.v.LH = mem[r + 2].hh.v.RH;
                    r = mem[r].hh.v.RH;
                }
            }
            if ((c == MAX_HALFWORD) || (pi <= -EJECT_PENALTY)) {
                fire_up(p);
                if (output_active)
                    return;
                goto done;
            }
        }
        if ((mem[p].hh.u.B0 < GLUE_NODE) || (mem[p].hh.u.B0 > KERN_NODE))
            goto lab80;
 lab90:/*update_heights *//*1039: */ if (mem[p].hh.u.B0 == KERN_NODE)
            q = p;
        else {

            q = mem[p + 1].hh.v.LH;
            page_so_far[2 + mem[q].hh.u.B0] = page_so_far[2 + mem[q].hh.u.B0] + mem[q + 2].cint;
            page_so_far[6] = page_so_far[6] + mem[q + 3].cint;
            if ((mem[q].hh.u.B1 != NORMAL) && (mem[q + 3].cint != 0)) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Infinite_glue_shrinkage_foun_Z2/*"Infinite glue shrinkage found on current page"*/));
                }
                {
                    help_ptr = 4;
                    help_line[3] = S(The_page_about_to_be_output_/*contains some infinitely*/);
                    help_line[2] = S(shrinkable_glue__e_g_____vss/*' or `\vskip 0pt minus 1fil'.*/);
                    help_line[1] = S(Such_glue_doesn_t_belong_the_Z1/*"Such glue doesn't belong there; but you can safely proceed,"*/);
                    help_line[0] = S(since_the_offensive_shrinkab/*ility has been made finite.*/);
                }
                error();
                r = new_spec(q);
                mem[r].hh.u.B1 = NORMAL;
                delete_glue_ref(q);
                mem[p + 1].hh.v.LH = r;
                q = r;
            }
        }
        page_so_far[1] = page_so_far[1] + page_so_far[7] + mem[q + 1].cint;
        page_so_far[7] = 0 /*:1039 */ ;
 lab80:                        /*contribute *//*1038: */ if (page_so_far[7] > page_max_depth) {
            page_so_far[1] = page_so_far[1] + page_so_far[7] - page_max_depth;
            page_so_far[7] = page_max_depth;
        }
        mem[page_tail].hh.v.RH = p;
        page_tail = p;
        mem[mem_top - 1].hh.v.RH = mem[p].hh.v.RH;
        mem[p].hh.v.RH = MIN_HALFWORD;
        goto done;
 lab31:                        /*done1 *//*1034: */ mem[mem_top - 1].hh.v.RH = mem[p].hh.v.RH;
        mem[p].hh.v.RH = MIN_HALFWORD;
        if (INTPAR(saving_vdiscards) > 0) {
            if (disc_ptr[LAST_BOX_CODE] == MIN_HALFWORD)
                disc_ptr[LAST_BOX_CODE] = p;
            else
                mem[disc_ptr[COPY_CODE]].hh.v.RH = p;
            disc_ptr[COPY_CODE] = p;
        } else
            flush_node_list(p); /*:1032*/
    done:
        ;
    } while (!(mem[mem_top - 1].hh.v.RH == MIN_HALFWORD));
    if (nest_ptr == 0)
        cur_list.tail = mem_top - 1;
    else
        nest[0].tail = mem_top - 1 /*:1030 */ ;
}

void app_space(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t q;

    if ((cur_list.aux.hh.v.LH >= 2000) && (GLUEPAR(xspace_skip) != 0))
        q = new_param_glue(GLUE_PAR__xspace_skip);
    else {

        if (GLUEPAR(space_skip) != 0)
            main_p = GLUEPAR(space_skip);
        else {                  /*1077: */

            main_p = font_glue[eqtb[CUR_FONT_LOC].hh.v.RH];
            if (main_p == MIN_HALFWORD) {
                main_p = new_spec(0);
                main_k = param_base[eqtb[CUR_FONT_LOC].hh.v.RH] + 2;
                mem[main_p + 1].cint = font_info[main_k].cint;
                mem[main_p + 2].cint = font_info[main_k + 1].cint;
                mem[main_p + 3].cint = font_info[main_k + 2].cint;
                font_glue[eqtb[CUR_FONT_LOC].hh.v.RH] = main_p;
            }
        }
        main_p = new_spec(main_p);
        if (cur_list.aux.hh.v.LH >= 2000)
            mem[main_p + 1].cint =
                mem[main_p + 1].cint + font_info[EXTRA_SPACE_CODE +
                                                 param_base[eqtb[CUR_FONT_LOC].hh.v.RH]].cint;
        mem[main_p + 2].cint = xn_over_d(mem[main_p + 2].cint, cur_list.aux.hh.v.LH, 1000);
        mem[main_p + 3].cint = xn_over_d(mem[main_p + 3].cint, 1000, cur_list.aux.hh.v.LH) /*:1079 */ ;
        q = new_glue(main_p);
        mem[main_p].hh.v.RH = MIN_HALFWORD;
    }
    mem[cur_list.tail].hh.v.RH = q;
    cur_list.tail = q;
}

void insert_dollar_sign(void)
{
    back_input();
    cur_tok = (MATH_SHIFT_TOKEN + 36);
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Missing___inserted_Z1/*"Missing $ inserted"*/));
    }
    {
        help_ptr = 2;
        help_line[1] = S(I_ve_inserted_a_begin_math_e/*nd-math symbol since I think*/);
        help_line[0] = S(you_left_one_out__Proceed__w/*ith fingers crossed.*/);
    }
    ins_error();
}

void you_cant(void)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(You_can_t_use__));
    }
    print_cmd_chr(cur_cmd, cur_chr);
    print_in_mode(cur_list.mode);
}

void report_illegal_case(void)
{
    you_cant();
    {
        help_ptr = 4;
        help_line[3] = S(Sorry__but_I_m_not_programme/*d to handle this case;*/);
        help_line[2] = S(I_ll_just_pretend_that_you_d/*idn't ask for it.*/);
        help_line[1] = S(If_you_re_in_the_wrong_mode_/* you might be able to*/);
        help_line[0] = 66421L /*"return to the right one by typing `I_' or `I$' or `I\par'." */ ;
    }
    error();
}

boolean privileged(void)
{
    register boolean Result;
    if (cur_list.mode > 0)
        Result = true;
    else {

        report_illegal_case();
        Result = false;
    }
    return Result;
}

boolean its_all_over(void)
{
    CACHE_THE_EQTB;
    register boolean Result;
    memory_word *mem = zmem;

    if (privileged()) {
        if ((mem_top - 2 == page_tail) && (cur_list.head == cur_list.tail) && (dead_cycles == 0)) {
            Result = true;
            return Result;
        }
        back_input();
        {
            mem[cur_list.tail].hh.v.RH = new_null_box();
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        mem[cur_list.tail + 1].cint = DIMENPAR(hsize);
        {
            mem[cur_list.tail].hh.v.RH = new_glue(8);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        {
            mem[cur_list.tail].hh.v.RH = new_penalty(NULL_FLAG);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        build_page();
    }
    Result = false;
    return Result;
}

void append_glue(void)
{
    memory_word *mem = zmem; small_number s;
    s = cur_chr;
    switch (s) {
    case 0:
        cur_val = 4;
        break;
    case 1:
        cur_val = 8;
        break;
    case 2:
        cur_val = 12;
        break;
    case 3:
        cur_val = 16;
        break;
    case 4:
        scan_glue(GLUE_VAL);
        break;
    case 5:
        scan_glue(MU_VAL);
        break;
    }
    {
        mem[cur_list.tail].hh.v.RH = new_glue(cur_val);
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
    if (s >= SKIP_CODE) {
        mem[cur_val].hh.v.RH--;
        if (s > SKIP_CODE)
            mem[cur_list.tail].hh.u.B1 = MU_GLUE;
    }
}

void append_kern(void)
{
    memory_word *mem = zmem; uint16_t s;
    s = cur_chr;
    scan_dimen(s == MU_GLUE, false, false);
    {
        mem[cur_list.tail].hh.v.RH = new_kern(cur_val);
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
    mem[cur_list.tail].hh.u.B1 = s;
}

void off_save(void)
{
    memory_word *mem = zmem; int32_t p;
    if (cur_group == BOTTOM_LEVEL) {    /*1101: */
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Extra_));
        }
        print_cmd_chr(cur_cmd, cur_chr);
        {
            help_ptr = 1;
            help_line[0] = S(Things_are_pretty_mixed_up__/*but I think the worst is over.*/);
        }
        error();
    } else {

        back_input();
        p = get_avail();
        mem[mem_top - 3].hh.v.RH = p;
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Missing_));
        }
        switch (cur_group) {
        case 14:
            {
                mem[p].hh.v.LH = (CS_TOKEN_FLAG + 2243228);
                print_esc(S(endgroup));
            }
            break;
        case 15:
            {
                mem[p].hh.v.LH = (MATH_SHIFT_TOKEN + 36);
                print_char(36 /*"$" */ );
            }
            break;
        case 16:
            {
                mem[p].hh.v.LH = (CS_TOKEN_FLAG + 2243229);
                mem[p].hh.v.RH = get_avail();
                p = mem[p].hh.v.RH;
                mem[p].hh.v.LH = (OTHER_TOKEN + 46);
                print_esc(S(right_));
            }
            break;
        default:
            {
                mem[p].hh.v.LH = (RIGHT_BRACE_TOKEN + 125);
                print_char(125 /*"_" */ );
            }
            break;
        }
        print(S(_inserted));
        begin_token_list(mem[mem_top - 3].hh.v.RH, INSERTED);
        {
            help_ptr = 5;
            help_line[4] = S(I_ve_inserted_something_that/* you may have forgotten.*/);
            help_line[3] = S(_See_the__inserted_text__abo/*ve.)*/);
            help_line[2] = S(With_luck__this_will_get_me_/*unwedged. But if you*/);
            help_line[1] = S(really_didn_t_forget_anythin/*g, try typing `2' now; then*/);
            help_line[0] = S(my_insertion_and_my_current_/*dilemma will both disappear.*/);
        }
        error();
    }
}

void extra_right_brace(void)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(66444L /*"Extra _, or forgotten " */ );
    }
    switch (cur_group) {
    case 14:
        print_esc(S(endgroup));
        break;
    case 15:
        print_char(36 /*"$" */ );
        break;
    case 16:
        print_esc(S(right));
        break;
    }
    {
        help_ptr = 5;
        help_line[4] = S(I_ve_deleted_a_group_closing/* symbol because it seems to be*/);
        help_line[3] = 66446L /*"spurious, as in `$x_$'. But perhaps the _ is legitimate and" */ ;
        help_line[2] = 66447L /*"you forgot something else, as in `\hbox_$x_'. In such cases" */ ;
        help_line[1] = S(the_way_to_recover_is_to_ins/*ert both the forgotten and the*/);
        help_line[0] = 66449L /*"deleted material, e.g., by typing `I$_'." */ ;
    }
    error();
    align_state++;
}

void normal_paragraph(void)
{
    CACHE_THE_EQTB;

    if (INTPAR(looseness) != 0)
        eq_word_define(INT_BASE + INT_PAR__looseness, 0);
    if (DIMENPAR(hang_indent) != 0)
        eq_word_define(DIMEN_BASE + DIMEN_PAR__hang_indent, 0);
    if (INTPAR(hang_after) != 1)
        eq_word_define(INT_BASE + INT_PAR__hang_after, 1);
    if (LOCAL(par_shape) != MIN_HALFWORD)
        eq_define(LOCAL_BASE + LOCAL__par_shape, SHAPE_REF, MIN_HALFWORD);
    if (eqtb[INTER_LINE_PENALTIES_LOC].hh.v.RH != MIN_HALFWORD)
        eq_define(INTER_LINE_PENALTIES_LOC, SHAPE_REF, MIN_HALFWORD);
}

void box_end(integer box_context)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;
    small_number a;

    if (box_context < 1073741824L) {    /*1111: */
        if (cur_box != MIN_HALFWORD) {
            mem[cur_box + 4].cint = box_context;
            if (abs(cur_list.mode) == VMODE) {
                if (pre_adjust_tail != MIN_HALFWORD) {
                    if (mem_top - 14 != pre_adjust_tail) {
                        mem[cur_list.tail].hh.v.RH = mem[mem_top - 14].hh.v.RH;
                        cur_list.tail = pre_adjust_tail;
                    }
                    pre_adjust_tail = MIN_HALFWORD;
                }
                append_to_vlist(cur_box);
                if (adjust_tail != MIN_HALFWORD) {
                    if (mem_top - 5 != adjust_tail) {
                        mem[cur_list.tail].hh.v.RH = mem[mem_top - 5].hh.v.RH;
                        cur_list.tail = adjust_tail;
                    }
                    adjust_tail = MIN_HALFWORD;
                }
                if (cur_list.mode > 0)
                    build_page();
            } else {

                if (abs(cur_list.mode) == HMODE)
                    cur_list.aux.hh.v.LH = 1000;
                else {

                    p = new_noad();
                    mem[p + 1].hh.v.RH = SUB_BOX;
                    mem[p + 1].hh.v.LH = cur_box;
                    cur_box = p;
                }
                mem[cur_list.tail].hh.v.RH = cur_box;
                cur_list.tail = cur_box;
            }
        }
    } else if (box_context < 1073807360L) {     /*1112: */
        if (box_context < 1073774592L) {
            cur_val = box_context - 1073741824L;
            a = 0;
        } else {

            cur_val = box_context - 1073774592L;
            a = 4;
        }
        if (cur_val < 256) {

            if ((a >= 4))
                geq_define(BOX_BASE + cur_val, BOX_REF, cur_box);
            else
                eq_define(BOX_BASE + cur_val, BOX_REF, cur_box);
        } else {

            find_sa_element(4, cur_val, true);
            if ((a >= 4))
                gsa_def(cur_ptr, cur_box);
            else
                sa_def(cur_ptr, cur_box);
        }
    } else if (cur_box != MIN_HALFWORD) {

        if (box_context > 1073807360L) {        /*1113: */
            do {
                get_x_token();
            } while (!((cur_cmd != SPACER) && (cur_cmd != RELAX) /*:422 */ ));
            if (((cur_cmd == HSKIP) && (abs(cur_list.mode) != VMODE))
                || ((cur_cmd == VSKIP) && (abs(cur_list.mode) == VMODE))) {
                append_glue();
                mem[cur_list.tail].hh.u.B1 = box_context - (1073807261L);
                mem[cur_list.tail + 1].hh.v.RH = cur_box;
            } else {

                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Leaders_not_followed_by_prop/*er glue*/));
                }
                {
                    help_ptr = 3;
                    help_line[2] = S(You_should_say___leaders__bo/*x or rule><hskip or vskip>'.*/);
                    help_line[1] = S(I_found_the__box_or_rule___b/*ut there's no suitable*/);
                    help_line[0] = S(_hskip_or_vskip___so_I_m_ign/*oring these leaders.*/);
                }
                back_error();
                flush_node_list(cur_box);
            }
        } else
            ship_out(cur_box);
    }
}

void begin_box(integer box_context)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p, q;
    int32_t r;
    boolean fm;
    int32_t tx;
    uint16_t m;
    int32_t k;
    int32_t n;

    switch (cur_chr) {
    case 0:
        {
            scan_register_num();
            if (cur_val < 256)
                cur_box = BOX_REG(cur_val);
            else {

                find_sa_element(4, cur_val, false);
                if (cur_ptr == MIN_HALFWORD)
                    cur_box = MIN_HALFWORD;
                else
                    cur_box = mem[cur_ptr + 1].hh.v.RH;
            }
            if (cur_val < 256)
                BOX_REG(cur_val) = MIN_HALFWORD;
            else {

                find_sa_element(4, cur_val, false);
                if (cur_ptr != MIN_HALFWORD) {
                    mem[cur_ptr + 1].hh.v.RH = MIN_HALFWORD;
                    mem[cur_ptr + 1].hh.v.LH++;
                    delete_sa_ref(cur_ptr);
                }
            }
        }
        break;
    case 1:
        {
            scan_register_num();
            if (cur_val < 256)
                q = BOX_REG(cur_val);
            else {

                find_sa_element(4, cur_val, false);
                if (cur_ptr == MIN_HALFWORD)
                    q = MIN_HALFWORD;
                else
                    q = mem[cur_ptr + 1].hh.v.RH;
            }
            cur_box = copy_node_list(q);
        }
        break;
    case 2:
        {
            cur_box = MIN_HALFWORD;
            if (abs(cur_list.mode) == MMODE) {
                you_cant();
                {
                    help_ptr = 1;
                    help_line[0] = S(Sorry__this__lastbox_will_be/* void.*/);
                }
                error();
            } else if ((cur_list.mode == VMODE) && (cur_list.head == cur_list.tail)) {
                you_cant();
                {
                    help_ptr = 2;
                    help_line[1] = S(Sorry___I_usually_can_t_take/* things from the current page.*/);
                    help_line[0] = S(This__lastbox_will_therefore/* be void.*/);
                }
                error();
            } else {

                tx = cur_list.tail;
                if (!(tx >= hi_mem_min)) {

                    if ((mem[tx].hh.u.B0 == MATH_NODE) && (mem[tx].hh.u.B1 == END_M_CODE)) {
                        r = cur_list.head;
                        do {
                            q = r;
                            r = mem[q].hh.v.RH;
                        } while (!(r == tx));
                        tx = q;
                    }
                }
                if (!(tx >= hi_mem_min)) {

                    if ((mem[tx].hh.u.B0 == HLIST_NODE) || (mem[tx].hh.u.B0 == VLIST_NODE)) {       /*1116: */
                        q = cur_list.head;
                        p = MIN_HALFWORD;
                        do {
                            r = p;
                            p = q;
                            fm = false;
                            if (!(q >= hi_mem_min)) {

                                if (mem[q].hh.u.B0 == DISC_NODE) {
                                    {
                                        register integer for_end;
                                        m = 1;
                                        for_end = mem[q].hh.u.B1;
                                        if (m <= for_end)
                                            do
                                                p = mem[p].hh.v.RH;
                                            while (m++ < for_end);
                                    }
                                    if (p == tx)
                                        goto done;
                                } else if ((mem[q].hh.u.B0 == MATH_NODE)
                                           && (mem[q].hh.u.B1 == BEGIN_M_CODE))
                                    fm = true;
                            }
                            q = mem[p].hh.v.RH;
                        } while (!(q == tx));
                        q = mem[tx].hh.v.RH;
                        mem[p].hh.v.RH = q;
                        mem[tx].hh.v.RH = MIN_HALFWORD;
                        if (q == MIN_HALFWORD) {

                            if (fm)
                                confusion(S(tail1));
                            else
                                cur_list.tail = p;
                        } else if (fm) {
                            cur_list.tail = r;
                            mem[r].hh.v.RH = MIN_HALFWORD;
                            flush_node_list(p);
                        }
                        cur_box = tx;
                        mem[cur_box + 4].cint = 0;
                    }
                }
            done:
                ;
            }
        }
        break;
    case 3:
        {
            scan_register_num();
            n = cur_val;
            if (!scan_keyword(S(to))) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Missing__to__inserted));
                }
                {
                    help_ptr = 2;
                    help_line[1] = S(I_m_working_on___vsplit_box_/*number> to <dimen>';*/);
                    help_line[0] = S(will_look_for_the__dimen__ne/*xt.*/);
                }
                error();
            }
            scan_dimen(false, false, false);
            cur_box = vsplit(n, cur_val);
        }
        break;
    default:
        {
            k = cur_chr - 4;
            save_stack[save_ptr + 0].cint = box_context;
            if (k == HMODE) {

                if ((box_context < 1073741824L) && (abs(cur_list.mode) == VMODE))
                    scan_spec(ADJUSTED_HBOX_GROUP, true);
                else
                    scan_spec(HBOX_GROUP, true);
            } else {

                if (k == VMODE)
                    scan_spec(VBOX_GROUP, true);
                else {

                    scan_spec(VTOP_GROUP, true);
                    k = VMODE;
                }
                normal_paragraph();
            }
            push_nest();
            cur_list.mode = -(integer) k;
            if (k == VMODE) {
                cur_list.aux.cint = -65536000L;
                if (LOCAL(every_vbox) != MIN_HALFWORD)
                    begin_token_list(LOCAL(every_vbox), EVERY_VBOX_TEXT);
            } else {

                cur_list.aux.hh.v.LH = 1000;
                if (LOCAL(every_hbox) != MIN_HALFWORD)
                    begin_token_list(LOCAL(every_hbox), EVERY_HBOX_TEXT);
            }
            return;
        }
        break;
    }
    box_end(box_context);
}

void scan_box(integer box_context)
{
    do {
        get_x_token();
    } while (!((cur_cmd != SPACER) && (cur_cmd != RELAX) /*:422 */ ));
    if (cur_cmd == MAKE_BOX)
        begin_box(box_context);
    else if ((box_context >= 1073807361L) && ((cur_cmd == HRULE) || (cur_cmd == VRULE))) {
        cur_box = scan_rule_spec();
        box_end(box_context);
    } else {

        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(A__box__was_supposed_to_be_h/*ere*/));
        }
        {
            help_ptr = 3;
            help_line[2] = S(I_was_expecting_to_see__hbox/* or \vbox or \copy or \box or*/);
            help_line[1] = S(something_like_that__So_you_/*might find something missing in*/);
            help_line[0] = S(your_output__But_keep_trying/*; you can fix this later.*/);
        }
        back_error();
    }
}

void package(small_number c)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    scaled h;
    int32_t p;
    scaled d;
    integer u, v;

    d = DIMENPAR(box_max_depth);
    u = STATEINT(xetex_upwards);
    unsave();
    save_ptr = save_ptr - 3;
    v = STATEINT(xetex_upwards);
    STATEINT(xetex_upwards) = u;
    if (cur_list.mode == -104)
        cur_box = hpack(mem[cur_list.head].hh.v.RH, save_stack[save_ptr + 2].cint, save_stack[save_ptr + 1].cint);
    else {

        cur_box =
            vpackage(mem[cur_list.head].hh.v.RH, save_stack[save_ptr + 2].cint, save_stack[save_ptr + 1].cint, d);
        if (c == VTOP_CODE) {   /*1122: */
            h = 0;
            p = mem[cur_box + 5].hh.v.RH;
            if (p != MIN_HALFWORD) {

                if (mem[p].hh.u.B0 <= RULE_NODE)
                    h = mem[p + 3].cint;
            }
            mem[cur_box + 2].cint = mem[cur_box + 2].cint - h + mem[cur_box + 3].cint;
            mem[cur_box + 3].cint = h;
        }
    }
    STATEINT(xetex_upwards) = v;
    pop_nest();
    box_end(save_stack[save_ptr + 0].cint);
}

small_number norm_min(integer h)
{
    register small_number Result;
    if (h <= 0)
        Result = 1;
    else if (h >= 63)
        Result = 63;
    else
        Result = h;
    return Result;
}

void new_graf(boolean indented)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    cur_list.pg = 0;

    if ((cur_list.mode == VMODE) || (cur_list.head != cur_list.tail)) {
        mem[cur_list.tail].hh.v.RH = new_param_glue(GLUE_PAR__par_skip);
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }

    push_nest();
    cur_list.mode = HMODE;
    cur_list.aux.hh.v.LH = 1000;
    if (INTPAR(language) <= 0)
        cur_lang = 0;
    else if (INTPAR(language) > BIGGEST_LANG)
        cur_lang = 0;
    else
        cur_lang = INTPAR(language);
    cur_list.aux.hh.v.RH = cur_lang;
    cur_list.pg =
        (norm_min(INTPAR(left_hyphen_min)) * 64 +
         norm_min(INTPAR(right_hyphen_min))) * 65536L + cur_lang;
    if (indented) {
        cur_list.tail = new_null_box();
        mem[cur_list.head].hh.v.RH = cur_list.tail;
        mem[cur_list.tail + 1].cint = eqtb[DIMEN_BASE].cint;
        if ((insert_src_special_every_par))
            insert_src_special();
    }
    if (LOCAL(every_par) != MIN_HALFWORD)
        begin_token_list(LOCAL(every_par), EVERY_PAR_TEXT);
    if (nest_ptr == 1)
        build_page();
}

void indent_in_hmode(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p, q;

    if (cur_chr > 0) {
        p = new_null_box();
        mem[p + 1].cint = eqtb[DIMEN_BASE].cint;
        if (abs(cur_list.mode) == HMODE)
            cur_list.aux.hh.v.LH = 1000;
        else {

            q = new_noad();
            mem[q + 1].hh.v.RH = SUB_BOX;
            mem[q + 1].hh.v.LH = p;
            p = q;
        }
        {
            mem[cur_list.tail].hh.v.RH = p;
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
    }
}

void head_for_vmode(void)
{
    if (cur_list.mode < 0) {

        if (cur_cmd != HRULE)
            off_save();
        else {

            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(You_can_t_use__));
            }
            print_esc(S(hrule));
            print(S(__here_except_with_leaders));
            {
                help_ptr = 2;
                help_line[1] = S(To_put_a_horizontal_rule_in_/*an hbox or an alignment,*/);
                help_line[0] = S(you_should_use__leaders_or__/*hrulefill (see The TeXbook).*/);
            }
            error();
        }
    } else {

        back_input();
        cur_tok = par_token;
        back_input();
        cur_input.index = INSERTED;
    }
}

void end_graf(void)
{
    if (cur_list.mode == HMODE) {
        if (cur_list.head == cur_list.tail)
            pop_nest();
        else
            line_break(false);
        if (cur_list.eTeX_aux != MIN_HALFWORD) {
            flush_list(cur_list.eTeX_aux);
            cur_list.eTeX_aux = MIN_HALFWORD;
        }
        normal_paragraph();
        error_count = 0;
    }
}

void begin_insert_or_adjust(void)
{
    if (cur_cmd == VADJUST)
        cur_val = 255;
    else {

        scan_eight_bit_int();
        if (cur_val == 255) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(You_can_t_));
            }
            print_esc(S(insert));
            print_int(255);
            {
                help_ptr = 1;
                help_line[0] = S(I_m_changing_to__insert0__bo/*x 255 is special.*/);
            }
            error();
            cur_val = 0;
        }
    }
    save_stack[save_ptr + 0].cint = cur_val;
    if ((cur_cmd == VADJUST) && scan_keyword(S(pre)))
        save_stack[save_ptr + 1].cint = 1;
    else
        save_stack[save_ptr + 1].cint = 0;
    save_ptr = save_ptr + 2;
    new_save_level(INSERT_GROUP);
    scan_left_brace();
    normal_paragraph();
    push_nest();
    cur_list.mode = -1;
    cur_list.aux.cint = -65536000L;
}

void make_mark(void)
{
    memory_word *mem = zmem; int32_t p;
    int32_t c;
    if (cur_chr == 0)
        c = 0;
    else {

        scan_register_num();
        c = cur_val;
    }
    p = scan_toks(false, true);
    p = get_node(SMALL_NODE_SIZE);
    mem[p + 1].hh.v.LH = c;
    mem[p].hh.u.B0 = MARK_NODE;
    mem[p].hh.u.B1 = 0;
    mem[p + 1].hh.v.RH = def_ref;
    mem[cur_list.tail].hh.v.RH = p;
    cur_list.tail = p;
}

void append_penalty(void)
{
    memory_word *mem = zmem; scan_int();
    {
        mem[cur_list.tail].hh.v.RH = new_penalty(cur_val);
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
    if (cur_list.mode == VMODE)
        build_page();
}

void delete_last(void)
{
    memory_word *mem = zmem; int32_t p, q;
    int32_t r;
    boolean fm;
    int32_t tx;
    uint16_t m;
    if ((cur_list.mode == VMODE) && (cur_list.tail == cur_list.head)) {       /*1141: */
        if ((cur_chr != GLUE_NODE) || (last_glue != MAX_HALFWORD)) {
            you_cant();
            {
                help_ptr = 2;
                help_line[1] = S(Sorry___I_usually_can_t_take/* things from the current page.*/);
                help_line[0] = S(Try__I_vskip__lastskip__inst/*ead.*/);
            }
            if (cur_chr == KERN_NODE)
                help_line[0] = (S(Try__I_kern__lastkern__inste/*ad.*/));
            else if (cur_chr != GLUE_NODE)
                help_line[0] = (S(Perhaps_you_can_make_the_out/*put routine do it.*/));
            error();
        }
    } else {

        tx = cur_list.tail;
        if (!(tx >= hi_mem_min)) {

            if ((mem[tx].hh.u.B0 == MATH_NODE) && (mem[tx].hh.u.B1 == END_M_CODE)) {
                r = cur_list.head;
                do {
                    q = r;
                    r = mem[q].hh.v.RH;
                } while (!(r == tx));
                tx = q;
            }
        }
        if (!(tx >= hi_mem_min)) {

            if (mem[tx].hh.u.B0 == cur_chr) {
                q = cur_list.head;
                p = MIN_HALFWORD;
                do {
                    r = p;
                    p = q;
                    fm = false;
                    if (!(q >= hi_mem_min)) {

                        if (mem[q].hh.u.B0 == DISC_NODE) {
                            {
                                register integer for_end;
                                m = 1;
                                for_end = mem[q].hh.u.B1;
                                if (m <= for_end)
                                    do
                                        p = mem[p].hh.v.RH;
                                    while (m++ < for_end);
                            }
                            if (p == tx)
                                return;
                        } else if ((mem[q].hh.u.B0 == MATH_NODE) && (mem[q].hh.u.B1 == BEGIN_M_CODE))
                            fm = true;
                    }
                    q = mem[p].hh.v.RH;
                } while (!(q == tx));
                q = mem[tx].hh.v.RH;
                mem[p].hh.v.RH = q;
                mem[tx].hh.v.RH = MIN_HALFWORD;
                if (q == MIN_HALFWORD) {

                    if (fm)
                        confusion(S(tail1));
                    else
                        cur_list.tail = p;
                } else if (fm) {
                    cur_list.tail = r;
                    mem[r].hh.v.RH = MIN_HALFWORD;
                    flush_node_list(p);
                }
                flush_node_list(tx);
            }
        }
    }
}

void unpackage(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;
    int32_t r;
    unsigned char /*copy_code */ c;

    if (cur_chr > COPY_CODE) {  /*1651: */
        mem[cur_list.tail].hh.v.RH = disc_ptr[cur_chr];
        disc_ptr[cur_chr] = MIN_HALFWORD;
        goto done;
    }
    c = cur_chr;
    scan_register_num();
    if (cur_val < 256)
        p = BOX_REG(cur_val);
    else {

        find_sa_element(4, cur_val, false);
        if (cur_ptr == MIN_HALFWORD)
            p = MIN_HALFWORD;
        else
            p = mem[cur_ptr + 1].hh.v.RH;
    }
    if (p == MIN_HALFWORD)
        return;
    if ((abs(cur_list.mode) == MMODE)
        || ((abs(cur_list.mode) == VMODE) && (mem[p].hh.u.B0 != VLIST_NODE))
        || ((abs(cur_list.mode) == HMODE) && (mem[p].hh.u.B0 != HLIST_NODE))) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Incompatible_list_can_t_be_u/*nboxed*/));
        }
        {
            help_ptr = 3;
            help_line[2] = S(Sorry__Pandora___You_sneaky_/*devil.)*/);
            help_line[1] = S(I_refuse_to_unbox_an__hbox_i/*n vertical mode or vice versa.*/);
            help_line[0] = S(And_I_can_t_open_any_boxes_i/*n math mode.*/);
        }
        error();
        return;
    }
    if (c == COPY_CODE)
        mem[cur_list.tail].hh.v.RH = copy_node_list(mem[p + 5].hh.v.RH);
    else {

        mem[cur_list.tail].hh.v.RH = mem[p + 5].hh.v.RH;
        if (cur_val < 256)
            BOX_REG(cur_val) = MIN_HALFWORD;
        else {

            find_sa_element(4, cur_val, false);
            if (cur_ptr != MIN_HALFWORD) {
                mem[cur_ptr + 1].hh.v.RH = MIN_HALFWORD;
                mem[cur_ptr + 1].hh.v.LH++;
                delete_sa_ref(cur_ptr);
            }
        }
        free_node(p, BOX_NODE_SIZE);
    }
done:
    while (mem[cur_list.tail].hh.v.RH != MIN_HALFWORD) {

        r = mem[cur_list.tail].hh.v.RH;
        if (!(r >= hi_mem_min) && (mem[r].hh.u.B0 == MARGIN_KERN_NODE)) {
            mem[cur_list.tail].hh.v.RH = mem[r].hh.v.RH;
            free_node(r, MARGIN_KERN_NODE_SIZE);
        }
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
}

void append_italic_correction(void)
{
    memory_word *mem = zmem; int32_t p;
    internal_font_number f;
    if (cur_list.tail != cur_list.head) {
        if ((cur_list.tail >= hi_mem_min))
            p = cur_list.tail;
        else if (mem[cur_list.tail].hh.u.B0 == LIGATURE_NODE)
            p = cur_list.tail + 1;
        else if ((mem[cur_list.tail].hh.u.B0 == WHATSIT_NODE)) {
            if ((mem[cur_list.tail].hh.u.B1 == NATIVE_WORD_NODE)
                || (mem[cur_list.tail].hh.u.B1 == NATIVE_WORD_NODE_AT)) {
                {
                    mem[cur_list.tail].hh.v.RH = new_kern(get_native_italic_correction(cur_list.tail));
                    cur_list.tail = mem[cur_list.tail].hh.v.RH;
                }
                mem[cur_list.tail].hh.u.B1 = EXPLICIT;
            } else if ((mem[cur_list.tail].hh.u.B1 == GLYPH_NODE)) {
                {
                    mem[cur_list.tail].hh.v.RH =
                        new_kern(get_native_glyph_italic_correction(cur_list.tail));
                    cur_list.tail = mem[cur_list.tail].hh.v.RH;
                }
                mem[cur_list.tail].hh.u.B1 = EXPLICIT;
            }
            return;
        } else
            return;
        f = mem[p].hh.u.B0;
        {
            mem[cur_list.tail].hh.v.RH =
                new_kern(font_info
                         [italic_base[f] +
                          (font_info[char_base[f] + effective_char(true, f, mem[p].hh.u.B1)].qqqq.u.B2) / 4].cint);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        mem[cur_list.tail].hh.u.B1 = EXPLICIT;
    }
}

void append_discretionary(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer c;

    mem[cur_list.tail].hh.v.RH = new_disc();
    cur_list.tail = mem[cur_list.tail].hh.v.RH;

    if (cur_chr == 1) {
        c = hyphen_char[eqtb[CUR_FONT_LOC].hh.v.RH];
        if (c >= 0) {

            if (c <= BIGGEST_CHAR)
                mem[cur_list.tail + 1].hh.v.LH = new_character(eqtb[CUR_FONT_LOC].hh.v.RH, c);
        }
    } else {

        save_ptr++;
        save_stack[save_ptr - 1].cint = 0;
        new_save_level(DISC_GROUP);
        scan_left_brace();
        push_nest();
        cur_list.mode = -104;
        cur_list.aux.hh.v.LH = 1000;
    }
}

void build_discretionary(void)
{
    memory_word *mem = zmem; int32_t p, q;
    integer n;
    unsave();
    q = cur_list.head;
    p = mem[q].hh.v.RH;
    n = 0;
    while (p != MIN_HALFWORD) {

        if (!(p >= hi_mem_min)) {

            if (mem[p].hh.u.B0 > RULE_NODE) {

                if (mem[p].hh.u.B0 != KERN_NODE) {

                    if (mem[p].hh.u.B0 != LIGATURE_NODE) {

                        if ((mem[p].hh.u.B0 != WHATSIT_NODE)
                            || ((mem[p].hh.u.B1 != NATIVE_WORD_NODE)
                                && (mem[p].hh.u.B1 != NATIVE_WORD_NODE_AT)
                                && (mem[p].hh.u.B1 != GLYPH_NODE))) {
                            {
                                if (file_line_error_style_p)
                                    print_file_line();
                                else
                                    print_nl(S(__/*"! "*/));
                                print(S(Improper_discretionary_list));
                            }
                            {
                                help_ptr = 1;
                                help_line[0] = S(Discretionary_lists_must_con/*tain only boxes and kerns.*/);
                            }
                            error();
                            begin_diagnostic();
                            print_nl(S(The_following_discretionary_/*sublist has been deleted:*/));
                            show_box(p);
                            end_diagnostic(true);
                            flush_node_list(p);
                            mem[q].hh.v.RH = MIN_HALFWORD;
                            goto done;
                        }
                    }
                }
            }
        }
        q = p;
        p = mem[q].hh.v.RH;
        n++;
    } /*:1156 */
done:
    p = mem[cur_list.head].hh.v.RH;
    pop_nest();
    switch (save_stack[save_ptr - 1].cint) {
    case 0:
        mem[cur_list.tail + 1].hh.v.LH = p;
        break;
    case 1:
        mem[cur_list.tail + 1].hh.v.RH = p;
        break;
    case 2:
        {
            if ((n > 0) && (abs(cur_list.mode) == MMODE)) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Illegal_math_));
                }
                print_esc(S(discretionary));
                {
                    help_ptr = 2;
                    help_line[1] = S(Sorry__The_third_part_of_a_d/*iscretionary break must be*/);
                    help_line[0] = S(empty__in_math_formulas__I_h/*ad to delete your third part.*/);
                }
                flush_node_list(p);
                n = 0;
                error();
            } else
                mem[cur_list.tail].hh.v.RH = p;
            if (n <= UINT16_MAX)
                mem[cur_list.tail].hh.u.B1 = n;
            else {

                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Discretionary_list_is_too_lo/*ng*/));
                }
                {
                    help_ptr = 2;
                    help_line[1] = S(Wow___I_never_thought_anybod/*y would tweak me here.*/);
                    help_line[0] = S(You_can_t_seriously_need_suc/*h a huge discretionary list?*/);
                }
                error();
            }
            if (n > 0)
                cur_list.tail = q;
            save_ptr--;
            return;
        }
        break;
    }
    save_stack[save_ptr - 1].cint++;
    new_save_level(DISC_GROUP);
    scan_left_brace();
    push_nest();
    cur_list.mode = -104;
    cur_list.aux.hh.v.LH = 1000;
}

void make_accent(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    double s, t;
    int32_t p, q, r;
    internal_font_number f;
    scaled a, h, x, w, delta, lsb, rsb;
    four_quarters i;

    scan_char_num();
    f = eqtb[CUR_FONT_LOC].hh.v.RH;
    p = new_character(f, cur_val);

    if (p != MIN_HALFWORD) {
        x = font_info[X_HEIGHT_CODE + param_base[f]].cint;
        s = font_info[SLANT_CODE + param_base[f]].cint / ((double)65536.0);
        if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))) {
            a = mem[p + 1].cint;
            if (a == 0)
                get_native_char_sidebearings(f, cur_val, &lsb, &rsb);
        } else
            a = font_info[width_base[f] + font_info[char_base[f] + effective_char(true, f, mem[p].hh.u.B1)].qqqq.u.B0].cint;
        do_assignments();
        q = MIN_HALFWORD;
        f = eqtb[CUR_FONT_LOC].hh.v.RH;
        if ((cur_cmd == LETTER) || (cur_cmd == OTHER_CHAR) || (cur_cmd == CHAR_GIVEN)) {
            q = new_character(f, cur_chr);
            cur_val = cur_chr;
        } else if (cur_cmd == CHAR_NUM) {
            scan_char_num();
            q = new_character(f, cur_val);
        } else
            back_input();
        if (q != MIN_HALFWORD) { /*1160: */
            t = font_info[SLANT_CODE + param_base[f]].cint / ((double)65536.0);
            if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))) {
                w = mem[q + 1].cint;
                get_native_char_height_depth(f, cur_val, &h, &delta);
            } else {

                i = font_info[char_base[f] + effective_char(true, f, mem[q].hh.u.B1)].qqqq;
                w = font_info[width_base[f] + i.u.B0].cint;
                h = font_info[height_base[f] + (i.u.B1) / 16].cint;
            }
            if (h != x) {
                p = hpack(p, 0, ADDITIONAL);
                mem[p + 4].cint = x - h;
            }
            if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))
                && (a == 0))
                delta = tex_round((w - lsb + rsb) / ((double)2.0) + h * t - x * s);
            else
                delta = tex_round((w - a) / ((double)2.0) + h * t - x * s);
            r = new_kern(delta);
            mem[r].hh.u.B1 = ACC_KERN;
            mem[cur_list.tail].hh.v.RH = r;
            mem[r].hh.v.RH = p;
            cur_list.tail = new_kern(-(integer) a - delta);
            mem[cur_list.tail].hh.u.B1 = ACC_KERN;
            mem[p].hh.v.RH = cur_list.tail;
            p = q;
        }
        mem[cur_list.tail].hh.v.RH = p;
        cur_list.tail = p;
        cur_list.aux.hh.v.LH = 1000;
    }
}

void align_error(void)
{
    if (abs(align_state) > 2) {      /*1163: */
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Misplaced_));
        }
        print_cmd_chr(cur_cmd, cur_chr);
        if (cur_tok == (TAB_TOKEN + 38)) {
            {
                help_ptr = 6;
                help_line[5] = S(I_can_t_figure_out_why_you_w/*ould want to use a tab mark*/);
                help_line[4] = S(here__If_you_just_want_an_am/*persand, the remedy is*/);
                help_line[3] = S(simple__Just_type__I____now_/* But if some right brace*/);
                help_line[2] = S(up_above_has_ended_a_previou/*s alignment prematurely,*/);
                help_line[1] = S(you_re_probably_due_for_more/* error messages, and you*/);
                help_line[0] = S(might_try_typing__S__now_jus/*t to see what is salvageable.*/);
            }
        } else {

            {
                help_ptr = 5;
                help_line[4] = S(I_can_t_figure_out_why_you_w/*ould want to use a tab mark*/);
                help_line[3] = S(or__cr_or__span_just_now__If/* something like a right brace*/);
                help_line[2] = S(up_above_has_ended_a_previou/*s alignment prematurely,*/);
                help_line[1] = S(you_re_probably_due_for_more/* error messages, and you*/);
                help_line[0] = S(might_try_typing__S__now_jus/*t to see what is salvageable.*/);
            }
        }
        error();
    } else {

        back_input();
        if (align_state < 0) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(65980L /*"Missing _ inserted" */ );
            }
            align_state++;
            cur_tok = (LEFT_BRACE_TOKEN + 123);
        } else {

            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(66508L /*"Missing _ inserted" */ );
            }
            align_state--;
            cur_tok = (RIGHT_BRACE_TOKEN + 125);
        }
        {
            help_ptr = 3;
            help_line[2] = S(I_ve_put_in_what_seems_to_be/* necessary to fix*/);
            help_line[1] = S(the_current_column_of_the_cu/*rrent alignment.*/);
            help_line[0] = S(Try_to_go_on__since_this_mig/*ht almost work.*/);
        }
        ins_error();
    }
}

void no_align_error(void)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Misplaced_));
    }
    print_esc(S(noalign));
    {
        help_ptr = 2;
        help_line[1] = S(I_expect_to_see__noalign_onl/*y after the \cr of*/);
        help_line[0] = S(an_alignment__Proceed__and_I/*'ll ignore this case.*/);
    }
    error();
}

void omit_error(void)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Misplaced_));
    }
    print_esc(S(omit));
    {
        help_ptr = 2;
        help_line[1] = S(I_expect_to_see__omit_only_a/*fter tab marks or the \cr of*/);
        help_line[0] = S(an_alignment__Proceed__and_I/*'ll ignore this case.*/);
    }
    error();
}

void do_endv(void)
{
    base_ptr = input_ptr;
    input_stack[base_ptr] = cur_input;
    while ((input_stack[base_ptr].index != V_TEMPLATE) && (input_stack[base_ptr].loc == MIN_HALFWORD)
           && (input_stack[base_ptr].state == TOKEN_LIST))
        base_ptr--;
    if ((input_stack[base_ptr].index != V_TEMPLATE) || (input_stack[base_ptr].loc != MIN_HALFWORD)
        || (input_stack[base_ptr].state != TOKEN_LIST))
        fatal_error(S(_interwoven_alignment_preamb/*les are not allowed)*/));
    if (cur_group == ALIGN_GROUP) {
        end_graf();
        if (fin_col())
            fin_row();
    } else
        off_save();
}

void cs_error(void)
{
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Extra_));
    }
    print_esc(S(endcsname));
    {
        help_ptr = 1;
        help_line[0] = S(I_m_ignoring_this__since_I_w/*asn't doing a \csname.*/);
    }
    error();
}

void push_math(group_code c)
{
    push_nest();
    cur_list.mode = -207;
    cur_list.aux.cint = MIN_HALFWORD;
    new_save_level(c);
}

void just_copy(int32_t p, int32_t h, int32_t t)
{
    memory_word *mem = zmem; int32_t r;
    unsigned char words;
    while (p != MIN_HALFWORD) {

        words = 1;
        if ((p >= hi_mem_min))
            r = get_avail();
        else
            switch (mem[p].hh.u.B0) {
            case 0:
            case 1:
                {
                    r = get_node(BOX_NODE_SIZE);
                    mem[r + 7].hh.v.LH = mem[p + 7].hh.v.LH;
                    mem[r + 7].hh.v.RH = mem[p + 7].hh.v.RH;
                    mem[r + 6] = mem[p + 6];
                    mem[r + 5] = mem[p + 5];
                    words = 5;
                    mem[r + 5].hh.v.RH = MIN_HALFWORD;
                }
                break;
            case 2:
                {
                    r = get_node(RULE_NODE_SIZE);
                    words = RULE_NODE_SIZE;
                }
                break;
            case 6:
                {
                    r = get_avail();
                    mem[r] = mem[p + 1];
                    goto lab40;
                }
                break;
            case 11:
            case 9:
                {
                    words = MEDIUM_NODE_SIZE;
                    r = get_node(words);
                }
                break;
            case 10:
                {
                    r = get_node(MEDIUM_NODE_SIZE);
                    mem[mem[p + 1].hh.v.LH].hh.v.RH++;
                    mem[r + 2].hh.v.LH = mem[p + 2].hh.v.LH;
                    mem[r + 2].hh.v.RH = mem[p + 2].hh.v.RH;
                    mem[r + 1].hh.v.LH = mem[p + 1].hh.v.LH;
                    mem[r + 1].hh.v.RH = MIN_HALFWORD;
                }
                break;
            case 8:
                switch (mem[p].hh.u.B1) {
                case 0:
                    {
                        r = get_node(OPEN_NODE_SIZE);
                        words = OPEN_NODE_SIZE;
                    }
                    break;
                case 1:
                case 3:
                    {
                        r = get_node(WRITE_NODE_SIZE);
                        mem[mem[p + 1].hh.v.RH].hh.v.LH++;
                        words = WRITE_NODE_SIZE;
                    }
                    break;
                case 2:
                case 4:
                    {
                        r = get_node(SMALL_NODE_SIZE);
                        words = SMALL_NODE_SIZE;
                    }
                    break;
                case 40:
                case 41:
                    {
                        words = mem[p + 4].qqqq.u.B0;
                        r = get_node(words);
                        while (words > 0) {

                            words--;
                            mem[r + words] = mem[p + words];
                        }
                        mem[r + 5].ptr = NULL;
                        mem[r + 4].qqqq.u.B3 = 0;
                        copy_native_glyph_info(p, r);
                    }
                    break;
                case 42:
                    {
                        r = get_node(GLYPH_NODE_SIZE);
                        words = GLYPH_NODE_SIZE;
                    }
                    break;
                case 43:
                case 44:
                    {
                        words =
                            (PIC_NODE_SIZE +
                             (mem[p + 4].hh.u.B0 + sizeof(memory_word) - 1) / sizeof(memory_word));
                        r = get_node(words);
                    }
                    break;
                case 6:
                    r = get_node(SMALL_NODE_SIZE);
                    break;
                default:
                    confusion(S(ext2));
                    break;
                }
                break;
            default:
                goto lab45;
                break;
            }
        while (words > 0) {

            words--;
            mem[r + words] = mem[p + words];
        }
 lab40:                        /*found */ mem[h].hh.v.RH = r;
        h = r;
 lab45:                        /*not_found */ p = mem[p].hh.v.RH;
    }
    mem[h].hh.v.RH = t;
}

void just_reverse(int32_t p)
{
    memory_word *mem = zmem;
    int32_t l;
    int32_t t;
    int32_t q;
    int32_t m, n;
    m = MIN_HALFWORD;
    n = MIN_HALFWORD;
    if (mem[mem_top - 3].hh.v.RH == MIN_HALFWORD) {
        just_copy(mem[p].hh.v.RH, mem_top - 3, MIN_HALFWORD);
        q = mem[mem_top - 3].hh.v.RH;
    } else {

        q = mem[p].hh.v.RH;
        mem[p].hh.v.RH = MIN_HALFWORD;
        flush_node_list(mem[mem_top - 3].hh.v.RH);
    }
    t = new_edge(cur_dir, 0);
    l = t;
    cur_dir = 1 - cur_dir;
    while (q != MIN_HALFWORD)
        if ((q >= hi_mem_min))
            do {
                p = q;
                q = mem[p].hh.v.RH;
                mem[p].hh.v.RH = l;
                l = p;
            } while (!(!(q >= hi_mem_min)));
        else {

            p = q;
            q = mem[p].hh.v.RH;
            if (mem[p].hh.u.B0 == MATH_NODE) {    /*1527: */

                if (odd(mem[p].hh.u.B1)) {

                    if (mem[LR_ptr].hh.v.LH != (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3)) {
                        mem[p].hh.u.B0 = KERN_NODE;
                        LR_problems++;
                    } else {

                        {
                            temp_ptr = LR_ptr;
                            LR_ptr = mem[temp_ptr].hh.v.RH;
                            {
                                mem[temp_ptr].hh.v.RH = avail;
                                avail = temp_ptr;
                            }
                        }
                        if (n > MIN_HALFWORD) {
                            n--;
                            mem[p].hh.u.B1--;
                        } else {

                            if (m > MIN_HALFWORD)
                                m--;
                            else {

                                mem[t + 1].cint = mem[p + 1].cint;
                                mem[t].hh.v.RH = q;
                                free_node(p, MEDIUM_NODE_SIZE);
                                goto done;
                            }
                            mem[p].hh.u.B0 = KERN_NODE;
                        }
                    }
                } else {

                    {
                        temp_ptr = get_avail();
                        mem[temp_ptr].hh.v.LH = (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3);
                        mem[temp_ptr].hh.v.RH = LR_ptr;
                        LR_ptr = temp_ptr;
                    }
                    if ((n > MIN_HALFWORD) || ((mem[p].hh.u.B1 / R_CODE) != cur_dir)) {
                        n++;
                        mem[p].hh.u.B1++;
                    } else {

                        mem[p].hh.u.B0 = KERN_NODE;
                        m++;
                    }
                }
            }
            mem[p].hh.v.RH = l;
            l = p;
        }
    goto done;
    mem[t + 1].cint = mem[p + 1].cint;
    mem[t].hh.v.RH = q;
    free_node(p, SMALL_NODE_SIZE);
done:
    mem[mem_top - 3].hh.v.RH = l;
}

void init_math(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    scaled w;
    int32_t j;
    integer x;
    scaled l;
    scaled s;
    int32_t p;
    int32_t q;
    internal_font_number f;
    integer n;
    scaled v;
    scaled d;

    get_token();

    if ((cur_cmd == MATH_SHIFT) && (cur_list.mode > 0)) { /*1180: */
        j = MIN_HALFWORD;
        w = -MAX_HALFWORD;
        if (cur_list.head == cur_list.tail) {       /*1520: */
            pop_nest();
            if (cur_list.eTeX_aux == MIN_HALFWORD)
                x = 0;
            else if (mem[cur_list.eTeX_aux].hh.v.LH >= R_CODE)
                x = -1;
            else
                x = 1 /*:1519 */ ;
        } else {
            line_break(true);
            /*1528: */
            if (GLUEPAR(right_skip) == 0)
                j = new_kern(0);
            else
                j = new_param_glue(GLUE_PAR__right_skip);

            if (GLUEPAR(left_skip) == 0)
                p = new_kern(0);
            else
                p = new_param_glue(GLUE_PAR__left_skip);

            mem[p].hh.v.RH = j;

            j = new_null_box();
            mem[j + 1].cint = mem[just_box + 1].cint;
            mem[j + 4].cint = mem[just_box + 4].cint;
            mem[j + 5].hh.v.RH = p;
            mem[j + 5].hh.u.B1 = mem[just_box + 5].hh.u.B1;
            mem[j + 5].hh.u.B0 = mem[just_box + 5].hh.u.B0;
            mem[j + 6].gr = mem[just_box + 6].gr;

            v = mem[just_box + 4].cint;
            if (cur_list.eTeX_aux == MIN_HALFWORD)
                x = 0;
            else if (mem[cur_list.eTeX_aux].hh.v.LH >= R_CODE)
                x = -1;
            else
                x = 1 /*:1519 */ ;
            if (x >= 0) {
                p = mem[just_box + 5].hh.v.RH;
                mem[mem_top - 3].hh.v.RH = MIN_HALFWORD;
            } else {

                v = -(integer) v - mem[just_box + 1].cint;
                p = new_math(0, BEGIN_L_CODE);
                mem[mem_top - 3].hh.v.RH = p;
                just_copy(mem[just_box + 5].hh.v.RH, p, new_math(0, END_L_CODE));
                cur_dir = RIGHT_TO_LEFT;
            }
            v = v + 2 * font_info[QUAD_CODE + param_base[eqtb[CUR_FONT_LOC].hh.v.RH]].cint;
            if ((eqtb[ETEX_STATE_BASE].cint > 0)) {    /*1497: */
                temp_ptr = get_avail();
                mem[temp_ptr].hh.v.LH = BEFORE;
                mem[temp_ptr].hh.v.RH = LR_ptr;
                LR_ptr = temp_ptr;
            }
            while (p != MIN_HALFWORD) {

 lab21:                        /*reswitch */ if ((p >= hi_mem_min)) {
                    f = mem[p].hh.u.B0;
                    d = font_info[width_base[f] +
                                  font_info[char_base[f] + effective_char(true, f, mem[p].hh.u.B1)].qqqq.u.B0].cint;
                    goto lab40;
                }
                switch (mem[p].hh.u.B0) {
                case 0:
                case 1:
                case 2:
                    {
                        d = mem[p + 1].cint;
                        goto lab40;
                    }
                    break;
                case 6:
                    {
                        mem[mem_top - 12] = mem[p + 1];
                        mem[mem_top - 12].hh.v.RH = mem[p].hh.v.RH;
                        p = mem_top - 12;
                        xtx_ligature_present = true;
                        goto lab21;
                    }
                    break;
                case 11:
                    d = mem[p + 1].cint;
                    break;
                case 40:
                    d = mem[p + 1].cint;
                    break;
                case 9:
                    {
                        d = mem[p + 1].cint;
                        if ((eqtb[ETEX_STATE_BASE].cint > 0)) {        /*1525: */

                            if (odd(mem[p].hh.u.B1)) {
                                if (mem[LR_ptr].hh.v.LH == (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3)) {
                                    temp_ptr = LR_ptr;
                                    LR_ptr = mem[temp_ptr].hh.v.RH;
                                    {
                                        mem[temp_ptr].hh.v.RH = avail;
                                        avail = temp_ptr;
                                    }
                                } else if (mem[p].hh.u.B1 > L_CODE) {
                                    w = MAX_HALFWORD;
                                    goto done;
                                }
                            } else {

                                {
                                    temp_ptr = get_avail();
                                    mem[temp_ptr].hh.v.LH = (L_CODE * (mem[p].hh.u.B1 / L_CODE) + 3);
                                    mem[temp_ptr].hh.v.RH = LR_ptr;
                                    LR_ptr = temp_ptr;
                                }
                                if ((mem[p].hh.u.B1 / R_CODE) != cur_dir) {
                                    just_reverse(p);
                                    p = mem_top - 3;
                                }
                            }
                        } else if (mem[p].hh.u.B1 >= L_CODE) {
                            w = MAX_HALFWORD;
                            goto done;
                        }
                    }
                    break;
                case 14:
                    {
                        d = mem[p + 1].cint;
                        cur_dir = mem[p].hh.u.B1;
                    }
                    break;
                case 10:
                    {
                        q = mem[p + 1].hh.v.LH;
                        d = mem[q + 1].cint;
                        if (mem[just_box + 5].hh.u.B0 == STRETCHING) {
                            if ((mem[just_box + 5].hh.u.B1 == mem[q].hh.u.B0) && (mem[q + 2].cint != 0))
                                v = MAX_HALFWORD;
                        } else if (mem[just_box + 5].hh.u.B0 == SHRINKING) {
                            if ((mem[just_box + 5].hh.u.B1 == mem[q].hh.u.B1) && (mem[q + 3].cint != 0))
                                v = MAX_HALFWORD;
                        }
                        if (mem[p].hh.u.B1 >= A_LEADERS)
                            goto lab40;
                    }
                    break;
                case 8:
                    if ((mem[p].hh.u.B1 == NATIVE_WORD_NODE) || (mem[p].hh.u.B1 == NATIVE_WORD_NODE_AT)
                        || (mem[p].hh.u.B1 == GLYPH_NODE) || (mem[p].hh.u.B1 == PIC_NODE)
                        || (mem[p].hh.u.B1 == PDF_NODE)) {
                        d = mem[p + 1].cint;
                        goto lab40;
                    } else
                        d = 0 /*:1398 */ ;
                    break;
                default:
                    d = 0;
                    break;
                }
                if (v < MAX_HALFWORD)
                    v = v + d;
                goto lab45;
 lab40:                        /*found */ if (v < MAX_HALFWORD) {
                    v = v + d;
                    w = v;
                } else {

                    w = MAX_HALFWORD;
                    goto done;
                }
 lab45:                        /*not_found */ p = mem[p].hh.v.RH;
            } /*1523:*/
        done:
            if ((eqtb[ETEX_STATE_BASE].cint > 0)) {
                while (LR_ptr != MIN_HALFWORD) {

                    temp_ptr = LR_ptr;
                    LR_ptr = mem[temp_ptr].hh.v.RH;
                    {
                        mem[temp_ptr].hh.v.RH = avail;
                        avail = temp_ptr;
                    }
                }
                if (LR_problems != 0) {
                    w = MAX_HALFWORD;
                    LR_problems = 0;
                }
            }
            cur_dir = LEFT_TO_RIGHT;
            flush_node_list(mem[mem_top - 3].hh.v.RH);
        }
        if (LOCAL(par_shape) == MIN_HALFWORD) {

            if ((DIMENPAR(hang_indent) != 0)
                &&
                (((INTPAR(hang_after) >= 0)
                  && (cur_list.pg + 2 > INTPAR(hang_after)))
                 || (cur_list.pg + 1 < -(integer) INTPAR(hang_after)))) {
                l = DIMENPAR(hsize) - abs(DIMENPAR(hang_indent));
                if (DIMENPAR(hang_indent) > 0)
                    s = DIMENPAR(hang_indent);
                else
                    s = 0;
            } else {

                l = DIMENPAR(hsize);
                s = 0;
            }
        } else {

            n = mem[LOCAL(par_shape)].hh.v.LH;
            if (cur_list.pg + 2 >= n)
                p = LOCAL(par_shape) + 2 * n;
            else
                p = LOCAL(par_shape) + 2 * (cur_list.pg + 2);
            s = mem[p - 1].cint;
            l = mem[p].cint;
        }
        push_math(MATH_SHIFT_GROUP);
        cur_list.mode = MMODE;
        eq_word_define(INT_BASE + INT_PAR__cur_fam, -1);
        eq_word_define(DIMEN_BASE + DIMEN_PAR__pre_display_size, w);
        cur_list.eTeX_aux = j;
        eq_word_define(INT_BASE + INT_PAR__pre_display_correction, x);
        eq_word_define(DIMEN_BASE + DIMEN_PAR__display_width, l);
        eq_word_define(DIMEN_BASE + DIMEN_PAR__display_indent, s);
        if (LOCAL(every_display) != MIN_HALFWORD)
            begin_token_list(LOCAL(every_display), EVERY_DISPLAY_TEXT);
        if (nest_ptr == 1)
            build_page();
    } else {

        back_input();
        {
            push_math(MATH_SHIFT_GROUP);
            eq_word_define(INT_BASE + INT_PAR__cur_fam, -1);
            if ((insert_src_special_every_math))
                insert_src_special();
            if (LOCAL(every_math) != MIN_HALFWORD)
                begin_token_list(LOCAL(every_math), EVERY_MATH_TEXT);
        }
    }
}

void start_eq_no(void)
{
    CACHE_THE_EQTB;

    save_stack[save_ptr + 0].cint = cur_chr;
    save_ptr++;

    push_math(MATH_SHIFT_GROUP);
    eq_word_define(INT_BASE + INT_PAR__cur_fam, -1);
    if (insert_src_special_every_math)
        insert_src_special();
    if (LOCAL(every_math) != MIN_HALFWORD)
        begin_token_list(LOCAL(every_math), EVERY_MATH_TEXT);
}

void scan_math(int32_t p)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer c;

 lab20:                        /*restart *//*422: */
    do {
        get_x_token();
    } while (!((cur_cmd != SPACER) && (cur_cmd != RELAX) /*:422 */ ));
 lab21:                        /*reswitch */ switch (cur_cmd) {
    case 11:
    case 12:
    case 68:
        {
            c = MATH_CODE(cur_chr);
            if (math_char(c) == ACTIVE_MATH_CHAR) {
                {
                    cur_cs = cur_chr + 1;
                    cur_cmd = eqtb[cur_cs].hh.u.B0;
                    cur_chr = eqtb[cur_cs].hh.v.RH;
                    x_token();
                    back_input();
                }
                goto lab20;
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
    case 17:
        if (cur_chr == 2) {
            scan_math_class_int();
            c = set_class(cur_val);
            scan_math_fam_int();
            c = c + set_family(cur_val);
            scan_usv_num();
            c = c + cur_val;
        } else if (cur_chr == 1) {
            scan_xetex_math_char_int();
            c = cur_val;
        } else {

            scan_fifteen_bit_int();
            c = set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) + (cur_val % 256);
        }
        break;
    case 69:
        {
            c = set_class(cur_chr / 4096) + set_family((cur_chr % 4096) / 256) + (cur_chr % 256);
        }
        break;
    case 70:
        c = cur_chr;
        break;
    case 15:
        {
            if (cur_chr == 1) {
                scan_math_class_int();
                c = set_class(cur_val);
                scan_math_fam_int();
                c = c + set_family(cur_val);
                scan_usv_num();
                c = c + cur_val;
            } else {

                scan_delimiter_int();
                c = cur_val / 4096;
                c = set_class(c / 4096) + set_family((c % 4096) / 256) + (c % 256);
            }
        }
        break;
    default:
        {
            back_input();
            scan_left_brace();
            save_stack[save_ptr + 0].cint = p;
            save_ptr++;
            push_math(MATH_GROUP);
            return;
        }
        break;
    }
    mem[p].hh.v.RH = MATH_CHAR;
    mem[p].hh.u.B1 = c % 65536L;
    if ((math_class(c) == 7)
        && ((INTPAR(cur_fam) >= 0)
            && (INTPAR(cur_fam) < NUMBER_MATH_FAMILIES)))
        mem[p].hh.u.B0 = INTPAR(cur_fam);
    else
        mem[p].hh.u.B0 = (math_fam(c));
    mem[p].hh.u.B0 = mem[p].hh.u.B0 + (math_char(c) / 65536L) * 256;
}

void set_math_char(integer c)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;
    UnicodeScalar ch;

    if (math_char(c) == ACTIVE_MATH_CHAR) {        /*1187: */
        cur_cs = cur_chr + 1;
        cur_cmd = eqtb[cur_cs].hh.u.B0;
        cur_chr = eqtb[cur_cs].hh.v.RH;
        x_token();
        back_input();
    } else {

        p = new_noad();
        mem[p + 1].hh.v.RH = MATH_CHAR;
        ch = math_char(c);
        mem[p + 1].hh.u.B1 = ch % 65536L;
        mem[p + 1].hh.u.B0 = math_fam(c);
        if (math_class(c) == 7) {
            if (((INTPAR(cur_fam) >= 0)
                 && (INTPAR(cur_fam) < NUMBER_MATH_FAMILIES)))
                mem[p + 1].hh.u.B0 = INTPAR(cur_fam);
            mem[p].hh.u.B0 = ORD_NOAD;
        } else
            mem[p].hh.u.B0 = ORD_NOAD + math_class(c);
        mem[p + 1].hh.u.B0 = mem[p + 1].hh.u.B0 + (ch / 65536L) * 256;
        mem[cur_list.tail].hh.v.RH = p;
        cur_list.tail = p;
    }
}

void math_limit_switch(void)
{
    memory_word *mem = zmem; if (cur_list.head != cur_list.tail) {

        if (mem[cur_list.tail].hh.u.B0 == OP_NOAD) {
            mem[cur_list.tail].hh.u.B1 = cur_chr;
            return;
        }
    }
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(Limit_controls_must_follow_a/* math operator*/));
    }
    {
        help_ptr = 1;
        help_line[0] = S(I_m_ignoring_this_misplaced_/*\limits or \nolimits command.*/);
    }
    error();
}

void scan_delimiter(int32_t p, boolean r)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;

    if (r) {
        if (cur_chr == 1) {
            cur_val1 = 1073741824L;
            scan_math_fam_int();
            cur_val1 = cur_val1 + cur_val * 2097152L;
            scan_usv_num();
            cur_val = cur_val1 + cur_val;
        } else
            scan_delimiter_int();
    } else {

        do {
            get_x_token();
        } while (!((cur_cmd != SPACER) && (cur_cmd != RELAX) /*:422 */ ));
        switch (cur_cmd) {
        case 11:
        case 12:
            {
                cur_val = DEL_CODE(cur_chr);
            }
            break;
        case 15:
            if (cur_chr == 1) {
                cur_val1 = 1073741824L;
                scan_math_class_int();
                scan_math_fam_int();
                cur_val1 = cur_val1 + cur_val * 2097152L;
                scan_usv_num();
                cur_val = cur_val1 + cur_val;
            } else
                scan_delimiter_int();
            break;
        default:
            {
                cur_val = -1;
            }
            break;
        }
    }
    if (cur_val < 0) {
        {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Missing_delimiter____inserte/*d)*/));
            }
            {
                help_ptr = 6;
                help_line[5] = 66531L /*"I was expecting to see something like `(' or `\_' or" */ ;
                help_line[4] = 66532L /*"`\_' here. If you typed, e.g., `_' instead of `\_', you" */ ;
                help_line[3] = 66533L /*"should probably delete the `_' by typing `1' now, so that" */ ;
                help_line[2] = S(braces_don_t_get_unbalanced_/* Otherwise just proceed.*/);
                help_line[1] = S(Acceptable_delimiters_are_ch/*aracters whose \delcode is*/);
                help_line[0] = S(nonnegative__or_you_can_use_/*`\delimiter <delimiter code>'.*/);
            }
            back_error();
            cur_val = 0;
        }
    }
    if (cur_val >= 1073741824L) {
        mem[p].qqqq.u.B0 = ((cur_val % 2097152L) / 65536L) * 256 + (cur_val / 2097152L) % 256;
        mem[p].qqqq.u.B1 = cur_val % 65536L;
        mem[p].qqqq.u.B2 = 0;
        mem[p].qqqq.u.B3 = 0;
    } else {

        mem[p].qqqq.u.B0 = (cur_val / 1048576L) % 16;
        mem[p].qqqq.u.B1 = (cur_val / 4096) % 256;
        mem[p].qqqq.u.B2 = (cur_val / 256) % 16;
        mem[p].qqqq.u.B3 = cur_val % 256;
    }
}

void math_radical(void)
{
    memory_word *mem = zmem; {
        mem[cur_list.tail].hh.v.RH = get_node(RADICAL_NOAD_SIZE);
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
    mem[cur_list.tail].hh.u.B0 = RADICAL_NOAD;
    mem[cur_list.tail].hh.u.B1 = NORMAL;
    mem[cur_list.tail + 1].hh = empty;
    mem[cur_list.tail + 3].hh = empty;
    mem[cur_list.tail + 2].hh = empty;
    scan_delimiter(cur_list.tail + 4, true);
    scan_math(cur_list.tail + 1);
}

void math_ac(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer c;

    if (cur_cmd == ACCENT) {   /*1201: */
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Please_use_));
        }
        print_esc(S(mathaccent));
        print(S(_for_accents_in_math_mode));
        {
            help_ptr = 2;
            help_line[1] = S(I_m_changing__accent_to__mat/*haccent here; wish me luck.*/);
            help_line[0] = S(_Accents_are_not_the_same_in/* formulas as they are in text.)*/);
        }
        error();
    }
    {
        mem[cur_list.tail].hh.v.RH = get_node(ACCENT_NOAD_SIZE);
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
    mem[cur_list.tail].hh.u.B0 = ACCENT_NOAD;
    mem[cur_list.tail].hh.u.B1 = NORMAL;
    mem[cur_list.tail + 1].hh = empty;
    mem[cur_list.tail + 3].hh = empty;
    mem[cur_list.tail + 2].hh = empty;
    mem[cur_list.tail + 4].hh.v.RH = MATH_CHAR;
    if (cur_chr == 1) {
        if (scan_keyword(S(fixed)))
            mem[cur_list.tail].hh.u.B1 = FIXED_ACC;
        else if (scan_keyword(S(bottom))) {
            if (scan_keyword(S(fixed)))
                mem[cur_list.tail].hh.u.B1 = (BOTTOM_ACC + 1);
            else
                mem[cur_list.tail].hh.u.B1 = BOTTOM_ACC;
        }
        scan_math_class_int();
        c = set_class(cur_val);
        scan_math_fam_int();
        c = c + set_family(cur_val);
        scan_usv_num();
        cur_val = cur_val + c;
    } else {

        scan_fifteen_bit_int();
        cur_val = set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) + (cur_val % 256);
    }
    mem[cur_list.tail + 4].hh.u.B1 = cur_val % 65536L;
    if ((math_class(cur_val) == 7)
        && ((INTPAR(cur_fam) >= 0)
            && (INTPAR(cur_fam) < NUMBER_MATH_FAMILIES)))
        mem[cur_list.tail + 4].hh.u.B0 = INTPAR(cur_fam);
    else
        mem[cur_list.tail + 4].hh.u.B0 = math_fam(cur_val);
    mem[cur_list.tail + 4].hh.u.B0 = mem[cur_list.tail + 4].hh.u.B0 + (math_char(cur_val) / 65536L) * 256;
    scan_math(cur_list.tail + 1);
}

void append_choices(void)
{
    memory_word *mem = zmem; {
        mem[cur_list.tail].hh.v.RH = new_choice();
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
    save_ptr++;
    save_stack[save_ptr - 1].cint = 0;
    push_math(MATH_CHOICE_GROUP);
    scan_left_brace();
}

int32_t fin_mlist(int32_t p)
{
    register int32_t Result;
    memory_word *mem = zmem; int32_t q;
    if (cur_list.aux.cint != MIN_HALFWORD) {       /*1220: */
        mem[cur_list.aux.cint + 3].hh.v.RH = SUB_MLIST;
        mem[cur_list.aux.cint + 3].hh.v.LH = mem[cur_list.head].hh.v.RH;
        if (p == MIN_HALFWORD)
            q = cur_list.aux.cint;
        else {

            q = mem[cur_list.aux.cint + 2].hh.v.LH;
            if ((mem[q].hh.u.B0 != LEFT_NOAD) || (cur_list.eTeX_aux == MIN_HALFWORD))
                confusion(S(right));
            mem[cur_list.aux.cint + 2].hh.v.LH = mem[cur_list.eTeX_aux].hh.v.RH;
            mem[cur_list.eTeX_aux].hh.v.RH = cur_list.aux.cint;
            mem[cur_list.aux.cint].hh.v.RH = p;
        }
    } else {

        mem[cur_list.tail].hh.v.RH = p;
        q = mem[cur_list.head].hh.v.RH;
    }
    pop_nest();
    Result = q;
    return Result;
}

void build_choices(void)
{
    memory_word *mem = zmem; int32_t p;
    unsave();
    p = fin_mlist(MIN_HALFWORD);
    switch (save_stack[save_ptr - 1].cint) {
    case 0:
        mem[cur_list.tail + 1].hh.v.LH = p;
        break;
    case 1:
        mem[cur_list.tail + 1].hh.v.RH = p;
        break;
    case 2:
        mem[cur_list.tail + 2].hh.v.LH = p;
        break;
    case 3:
        {
            mem[cur_list.tail + 2].hh.v.RH = p;
            save_ptr--;
            return;
        }
        break;
    }
    save_stack[save_ptr - 1].cint++;
    push_math(MATH_CHOICE_GROUP);
    scan_left_brace();
}

void sub_sup(void)
{
    memory_word *mem = zmem; small_number t;
    int32_t p;
    t = EMPTY;
    p = MIN_HALFWORD;
    if (cur_list.tail != cur_list.head) {

        if ((mem[cur_list.tail].hh.u.B0 >= ORD_NOAD)
            && (mem[cur_list.tail].hh.u.B0 < LEFT_NOAD)) {
            p = cur_list.tail + 2 + cur_cmd - 7;
            t = mem[p].hh.v.RH;
        }
    }
    if ((p == MIN_HALFWORD) || (t != EMPTY)) {   /*1212: */
        {
            mem[cur_list.tail].hh.v.RH = new_noad();
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        p = cur_list.tail + 2 + cur_cmd - 7;
        if (t != EMPTY) {
            if (cur_cmd == SUP_MARK) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Double_superscript));
                }
                {
                    help_ptr = 1;
                    help_line[0] = 66544L /*"I treat `x^1^2' essentially like `x^1__^2'." */ ;
                }
            } else {

                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Double_subscript));
                }
                {
                    help_ptr = 1;
                    help_line[0] = 66546L /*"I treat `x_1_2' essentially like `x_1___2'." */ ;
                }
            }
            error();
        }
    }
    scan_math(p);
}

void math_fraction(void)
{
    memory_word *mem = zmem; small_number c;
    c = cur_chr;
    if (cur_list.aux.cint != MIN_HALFWORD) {       /*1218: */
        if (c >= DELIMITED_CODE) {
            scan_delimiter(mem_top - 12, false);
            scan_delimiter(mem_top - 12, false);
        }
        if (c % DELIMITED_CODE == ABOVE_CODE)
            scan_dimen(false, false, false);
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(66553L /*"Ambiguous; you need another _ and _" */ );
        }
        {
            help_ptr = 3;
            help_line[2] = S(I_m_ignoring_this_fraction_s/*pecification, since I don't*/);
            help_line[1] = S(know_whether_a_construction_/*like `x \over y \over z'*/);
            help_line[0] = 66556L /*"means `_x \over y_ \over z' or `x \over _y \over z_'." */ ;
        }
        error();
    } else {

        cur_list.aux.cint = get_node(FRACTION_NOAD_SIZE);
        mem[cur_list.aux.cint].hh.u.B0 = FRACTION_NOAD;
        mem[cur_list.aux.cint].hh.u.B1 = NORMAL;
        mem[cur_list.aux.cint + 2].hh.v.RH = SUB_MLIST;
        mem[cur_list.aux.cint + 2].hh.v.LH = mem[cur_list.head].hh.v.RH;
        mem[cur_list.aux.cint + 3].hh = empty;
        mem[cur_list.aux.cint + 4].qqqq = null_delimiter;
        mem[cur_list.aux.cint + 5].qqqq = null_delimiter;
        mem[cur_list.head].hh.v.RH = MIN_HALFWORD;
        cur_list.tail = cur_list.head;
        if (c >= DELIMITED_CODE) {
            scan_delimiter(cur_list.aux.cint + 4, false);
            scan_delimiter(cur_list.aux.cint + 5, false);
        }
        switch (c % DELIMITED_CODE) {
        case 0:
            {
                scan_dimen(false, false, false);
                mem[cur_list.aux.cint + 1].cint = cur_val;
            }
            break;
        case 1:
            mem[cur_list.aux.cint + 1].cint = 1073741824L;
            break;
        case 2:
            mem[cur_list.aux.cint + 1].cint = 0;
            break;
        }
    }
}

void math_left_right(void)
{
    memory_word *mem = zmem; small_number t;
    int32_t p;
    int32_t q;
    t = cur_chr;
    if ((t != LEFT_NOAD) && (cur_group != MATH_LEFT_GROUP)) { /*1227: */
        if (cur_group == MATH_SHIFT_GROUP) {
            scan_delimiter(mem_top - 12, false);
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Extra_));
            }
            if (t == 1) {
                print_esc(S(middle));
                {
                    help_ptr = 1;
                    help_line[0] = S(I_m_ignoring_a__middle_that_/*had no matching \left.*/);
                }
            } else {

                print_esc(S(right));
                {
                    help_ptr = 1;
                    help_line[0] = S(I_m_ignoring_a__right_that_h/*ad no matching \left.*/);
                }
            }
            error();
        } else
            off_save();
    } else {

        p = new_noad();
        mem[p].hh.u.B0 = t;
        scan_delimiter(p + 1, false);
        if (t == 1) {
            mem[p].hh.u.B0 = RIGHT_NOAD;
            mem[p].hh.u.B1 = 1;
        }
        if (t == LEFT_NOAD)
            q = p;
        else {

            q = fin_mlist(p);
            unsave();
        }
        if (t != RIGHT_NOAD) {
            push_math(MATH_LEFT_GROUP);
            mem[cur_list.head].hh.v.RH = q;
            cur_list.tail = p;
            cur_list.eTeX_aux = p;
        } else {

            {
                mem[cur_list.tail].hh.v.RH = new_noad();
                cur_list.tail = mem[cur_list.tail].hh.v.RH;
            }
            mem[cur_list.tail].hh.u.B0 = INNER_NOAD;
            mem[cur_list.tail + 1].hh.v.RH = SUB_MLIST;
            mem[cur_list.tail + 1].hh.v.LH = q;
        }
    }
}

void app_display(int32_t j, int32_t b, scaled d)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    scaled z;
    scaled s;
    scaled e;
    integer x;
    int32_t p, q, r, t, u;

    s = DIMENPAR(display_indent);
    x = INTPAR(pre_display_correction);

    if (x == 0)
        mem[b + 4].cint = s + d;
    else {

        z = DIMENPAR(display_width);
        p = b;
        if (x > 0)
            e = z - d - mem[p + 1].cint;
        else {

            e = d;
            d = z - e - mem[p + 1].cint;
        }
        if (j != MIN_HALFWORD) {
            b = copy_node_list(j);
            mem[b + 3].cint = mem[p + 3].cint;
            mem[b + 2].cint = mem[p + 2].cint;
            s = s - mem[b + 4].cint;
            d = d + s;
            e = e + mem[b + 1].cint - z - s;
        }
        if ((mem[p].hh.u.B1) == DLIST)
            q = p;
        else {

            r = mem[p + 5].hh.v.RH;
            free_node(p, BOX_NODE_SIZE);
            if (r == MIN_HALFWORD)
                confusion(S(LR4));
            if (x > 0) {
                p = r;
                do {
                    q = r;
                    r = mem[r].hh.v.RH;
                } while (!(r == MIN_HALFWORD));
            } else {

                p = MIN_HALFWORD;
                q = r;
                do {
                    t = mem[r].hh.v.RH;
                    mem[r].hh.v.RH = p;
                    p = r;
                    r = t;
                } while (!(r == MIN_HALFWORD));
            }
        }
        if (j == MIN_HALFWORD) {
            r = new_kern(0);
            t = new_kern(0);
        } else {

            r = mem[b + 5].hh.v.RH;
            t = mem[r].hh.v.RH;
        }
        u = new_math(0, END_M_CODE);
        if (mem[t].hh.u.B0 == GLUE_NODE) {
            j = new_skip_param(GLUE_PAR__right_skip);
            mem[q].hh.v.RH = j;
            mem[j].hh.v.RH = u;
            j = mem[t + 1].hh.v.LH;
            mem[temp_ptr].hh.u.B0 = mem[j].hh.u.B0;
            mem[temp_ptr].hh.u.B1 = mem[j].hh.u.B1;
            mem[temp_ptr + 1].cint = e - mem[j + 1].cint;
            mem[temp_ptr + 2].cint = -(integer) mem[j + 2].cint;
            mem[temp_ptr + 3].cint = -(integer) mem[j + 3].cint;
            mem[u].hh.v.RH = t;
        } else {

            mem[t + 1].cint = e;
            mem[t].hh.v.RH = u;
            mem[q].hh.v.RH = t;
        }
        u = new_math(0, BEGIN_M_CODE);
        if (mem[r].hh.u.B0 == GLUE_NODE) {
            j = new_skip_param(GLUE_PAR__left_skip);
            mem[u].hh.v.RH = j;
            mem[j].hh.v.RH = p;
            j = mem[r + 1].hh.v.LH;
            mem[temp_ptr].hh.u.B0 = mem[j].hh.u.B0;
            mem[temp_ptr].hh.u.B1 = mem[j].hh.u.B1;
            mem[temp_ptr + 1].cint = d - mem[j + 1].cint;
            mem[temp_ptr + 2].cint = -(integer) mem[j + 2].cint;
            mem[temp_ptr + 3].cint = -(integer) mem[j + 3].cint;
            mem[r].hh.v.RH = u;
        } else {

            mem[r + 1].cint = d;
            mem[r].hh.v.RH = p;
            mem[u].hh.v.RH = r;
            if (j == MIN_HALFWORD) {
                b = hpack(u, 0, ADDITIONAL);
                mem[b + 4].cint = s;
            } else
                mem[b + 5].hh.v.RH = u;
        }
    }
    append_to_vlist(b);
}

void after_math(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    boolean l;
    boolean danger;
    integer m;
    int32_t p;
    int32_t a;
    int32_t b;
    scaled w;
    scaled z;
    scaled e;
    scaled q;
    scaled d;
    scaled s;
    small_number g1, g2;
    int32_t r;
    int32_t t;
    int32_t pre_t;
    int32_t j;

    danger = false;

    if (cur_list.mode == MMODE)
        j = cur_list.eTeX_aux /*:1530 */ ;
    if (((font_params[MATH_FONT(2)] < TOTAL_MATHSY_PARAMS)
         &&
         (!((font_area[MATH_FONT(2)] == OTGR_FONT_FLAG)
            && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2)])))))
        || ((font_params[MATH_FONT(2 + SCRIPT_SIZE)] < TOTAL_MATHSY_PARAMS)
            &&
            (!((font_area[MATH_FONT(2 + SCRIPT_SIZE)] == OTGR_FONT_FLAG)
               && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2 + SCRIPT_SIZE)])))))
        || ((font_params[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)] < TOTAL_MATHSY_PARAMS)
            &&
            (!((font_area[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)] == OTGR_FONT_FLAG)
               && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)])))))) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Math_formula_deleted__Insuff/*icient symbol fonts*/));
        }
        {
            help_ptr = 3;
            help_line[2] = S(Sorry__but_I_can_t_typeset_m/*ath unless \textfont 2*/);
            help_line[1] = S(and__scriptfont_2_and__scrip/*tscriptfont 2 have all*/);
            help_line[0] = S(the__fontdimen_values_needed/* in math symbol fonts.*/);
        }
        error();
        flush_math();
        danger = true;
    } else
        if (((font_params[MATH_FONT(3 + TEXT_SIZE)] < TOTAL_MATHEX_PARAMS)
             &&
             (!((font_area[MATH_FONT(3 + TEXT_SIZE)] == OTGR_FONT_FLAG)
                && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + TEXT_SIZE)])))))
            || ((font_params[MATH_FONT(3 + SCRIPT_SIZE)] < TOTAL_MATHEX_PARAMS)
                &&
                (!((font_area[MATH_FONT(3 + SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                   && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + SCRIPT_SIZE)])))))
            || ((font_params[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)] < TOTAL_MATHEX_PARAMS)
                &&
                (!((font_area[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                   && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)])))))) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Math_formula_deleted__Insuff_Z1/*"Math formula deleted: Insufficient extension fonts"*/));
        }
        {
            help_ptr = 3;
            help_line[2] = S(Sorry__but_I_can_t_typeset_m_Z1/*"Sorry, but I can't typeset math unless \textfont 3"*/);
            help_line[1] = S(and__scriptfont_3_and__scrip/*tscriptfont 3 have all*/);
            help_line[0] = S(the__fontdimen_values_needed_Z1/*"the \fontdimen values needed in math extension fonts."*/);
        }
        error();
        flush_math();
        danger = true;
    }
    m = cur_list.mode;
    l = false;
    p = fin_mlist(MIN_HALFWORD);
    if (cur_list.mode == -(integer) m) {
        {
            get_x_token();
            if (cur_cmd != MATH_SHIFT) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Display_math_should_end_with/* $$*/));
                }
                {
                    help_ptr = 2;
                    help_line[1] = S(The_____that_I_just_saw_supp/*osedly matches a previous `$$'.*/);
                    help_line[0] = S(So_I_shall_assume_that_you_t/*yped `$$' both times.*/);
                }
                back_error();
            }
        }
        cur_mlist = p;
        cur_style = TEXT_STYLE;
        mlist_penalties = false;
        mlist_to_hlist();
        a = hpack(mem[mem_top - 3].hh.v.RH, 0, ADDITIONAL);
        mem[a].hh.u.B1 = DLIST;
        unsave();
        save_ptr--;
        if (save_stack[save_ptr + 0].cint == 1)
            l = true;
        danger = false;
        if (cur_list.mode == MMODE)
            j = cur_list.eTeX_aux /*:1530 */ ;
        if (((font_params[MATH_FONT(2)] < TOTAL_MATHSY_PARAMS)
             &&
             (!((font_area[MATH_FONT(2)] == OTGR_FONT_FLAG)
                && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2)])))))
            || ((font_params[MATH_FONT(2 + SCRIPT_SIZE)] < TOTAL_MATHSY_PARAMS)
                &&
                (!((font_area[MATH_FONT(2 + SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                   && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2 + SCRIPT_SIZE)])))))
            || ((font_params[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)] < TOTAL_MATHSY_PARAMS)
                &&
                (!((font_area[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                   && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(2 + SCRIPT_SCRIPT_SIZE)])))))) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Math_formula_deleted__Insuff/*icient symbol fonts*/));
            }
            {
                help_ptr = 3;
                help_line[2] = S(Sorry__but_I_can_t_typeset_m/*ath unless \textfont 2*/);
                help_line[1] = S(and__scriptfont_2_and__scrip/*tscriptfont 2 have all*/);
                help_line[0] = S(the__fontdimen_values_needed/* in math symbol fonts.*/);
            }
            error();
            flush_math();
            danger = true;
        } else
            if (((font_params[MATH_FONT(3 + TEXT_SIZE)] < TOTAL_MATHEX_PARAMS)
                 &&
                 (!((font_area[MATH_FONT(3 + TEXT_SIZE)] == OTGR_FONT_FLAG)
                    && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + TEXT_SIZE)])))))
                || ((font_params[MATH_FONT(3 + SCRIPT_SIZE)] < TOTAL_MATHEX_PARAMS)
                    &&
                    (!((font_area[MATH_FONT(3 + SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                       && (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + SCRIPT_SIZE)])))))
                || ((font_params[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)] < TOTAL_MATHEX_PARAMS)
                    &&
                    (!((font_area[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)] == OTGR_FONT_FLAG)
                       &&
                       (isOpenTypeMathFont(font_layout_engine[MATH_FONT(3 + SCRIPT_SCRIPT_SIZE)])))))) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Math_formula_deleted__Insuff_Z1/*"Math formula deleted: Insufficient extension fonts"*/));
            }
            {
                help_ptr = 3;
                help_line[2] = S(Sorry__but_I_can_t_typeset_m_Z1/*"Sorry, but I can't typeset math unless \textfont 3"*/);
                help_line[1] = S(and__scriptfont_3_and__scrip/*tscriptfont 3 have all*/);
                help_line[0] = S(the__fontdimen_values_needed_Z1/*"the \fontdimen values needed in math extension fonts."*/);
            }
            error();
            flush_math();
            danger = true;
        }
        m = cur_list.mode;
        p = fin_mlist(MIN_HALFWORD);
    } else
        a = MIN_HALFWORD;
    if (m < 0) {                /*1231: */
        {
            mem[cur_list.tail].hh.v.RH = new_math(DIMENPAR(math_surround), BEFORE);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        cur_mlist = p;
        cur_style = TEXT_STYLE;
        mlist_penalties = (cur_list.mode > 0);
        mlist_to_hlist();
        mem[cur_list.tail].hh.v.RH = mem[mem_top - 3].hh.v.RH;
        while (mem[cur_list.tail].hh.v.RH != MIN_HALFWORD)
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        {
            mem[cur_list.tail].hh.v.RH = new_math(DIMENPAR(math_surround), AFTER);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        cur_list.aux.hh.v.LH = 1000;
        unsave();
    } else {

        if (a == MIN_HALFWORD) { /*1232: */
            get_x_token();
            if (cur_cmd != MATH_SHIFT) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Display_math_should_end_with/* $$*/));
                }
                {
                    help_ptr = 2;
                    help_line[1] = S(The_____that_I_just_saw_supp/*osedly matches a previous `$$'.*/);
                    help_line[0] = S(So_I_shall_assume_that_you_t/*yped `$$' both times.*/);
                }
                back_error();
            }
        }
        cur_mlist = p;
        cur_style = DISPLAY_STYLE;
        mlist_penalties = false;
        mlist_to_hlist();
        p = mem[mem_top - 3].hh.v.RH;
        adjust_tail = mem_top - 5;
        pre_adjust_tail = mem_top - 14;
        b = hpack(p, 0, ADDITIONAL);
        p = mem[b + 5].hh.v.RH;
        t = adjust_tail;
        adjust_tail = MIN_HALFWORD;
        pre_t = pre_adjust_tail;
        pre_adjust_tail = MIN_HALFWORD;
        w = mem[b + 1].cint;
        z = DIMENPAR(display_width);
        s = DIMENPAR(display_indent);
        if (INTPAR(pre_display_correction) < 0)
            s = -(integer) s - z;
        if ((a == MIN_HALFWORD) || danger) {
            e = 0;
            q = 0;
        } else {

            e = mem[a + 1].cint;
            q = e + math_quad(TEXT_SIZE);
        }
        if (w + q > z) {        /*1236: */
            if ((e != 0)
                && ((w - total_shrink[NORMAL] + q <= z) || (total_shrink[FIL] != 0)
                    || (total_shrink[FILL] != 0) || (total_shrink[FILLL] != 0))) {
                free_node(b, BOX_NODE_SIZE);
                b = hpack(p, z - q, EXACTLY);
            } else {

                e = 0;
                if (w > z) {
                    free_node(b, BOX_NODE_SIZE);
                    b = hpack(p, z, EXACTLY);
                }
            }
            w = mem[b + 1].cint;
        }
        mem[b].hh.u.B1 = DLIST;
        d = half(z - w);
        if ((e > 0) && (d < 2 * e)) {
            d = half(z - w - e);
            if (p != MIN_HALFWORD) {

                if (!(p >= hi_mem_min)) {

                    if (mem[p].hh.u.B0 == GLUE_NODE)
                        d = 0;
                }
            }
        }
        {
            mem[cur_list.tail].hh.v.RH = new_penalty(INTPAR(pre_display_penalty));
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        if ((d + s <= DIMENPAR(pre_display_size)) || l) {
            g1 = GLUE_PAR__above_display_skip;
            g2 = GLUE_PAR__below_display_skip;
        } else {
            g1 = GLUE_PAR__above_display_short_skip;
            g2 = GLUE_PAR__below_display_short_skip;
        }
        if (l && (e == 0)) {
            app_display(j, a, 0);
            {
                mem[cur_list.tail].hh.v.RH = new_penalty(INF_PENALTY);
                cur_list.tail = mem[cur_list.tail].hh.v.RH;
            }
        } else {

            mem[cur_list.tail].hh.v.RH = new_param_glue(g1);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        if (e != 0) {
            r = new_kern(z - w - e - d);
            if (l) {
                mem[a].hh.v.RH = r;
                mem[r].hh.v.RH = b;
                b = a;
                d = 0;
            } else {

                mem[b].hh.v.RH = r;
                mem[r].hh.v.RH = a;
            }
            b = hpack(b, 0, ADDITIONAL);
        }
        app_display(j, b, d);
        if ((a != MIN_HALFWORD) && (e == 0) && !l) {
            {
                mem[cur_list.tail].hh.v.RH = new_penalty(INF_PENALTY);
                cur_list.tail = mem[cur_list.tail].hh.v.RH;
            }
            app_display(j, a, z - mem[a + 1].cint);
            g2 = 0;
        }
        if (t != mem_top - 5) {
            mem[cur_list.tail].hh.v.RH = mem[mem_top - 5].hh.v.RH;
            cur_list.tail = t;
        }
        if (pre_t != mem_top - 14) {
            mem[cur_list.tail].hh.v.RH = mem[mem_top - 14].hh.v.RH;
            cur_list.tail = pre_t;
        }
        {
            mem[cur_list.tail].hh.v.RH = new_penalty(INTPAR(post_display_penalty));
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        if (g2 > 0) {
            mem[cur_list.tail].hh.v.RH = new_param_glue(g2);
            cur_list.tail = mem[cur_list.tail].hh.v.RH;
        }
        flush_node_list(j);
        resume_after_display();
    }
}

void resume_after_display(void)
{
    CACHE_THE_EQTB;

    if (cur_group != MATH_SHIFT_GROUP)
        confusion(S(display));

    unsave();
    cur_list.pg = cur_list.pg + 3;
    push_nest();
    cur_list.mode = HMODE;
    cur_list.aux.hh.v.LH = 1000;
    if (INTPAR(language) <= 0)
        cur_lang = 0;
    else if (INTPAR(language) > BIGGEST_LANG)
        cur_lang = 0;
    else
        cur_lang = INTPAR(language);
    cur_list.aux.hh.v.RH = cur_lang;
    cur_list.pg =
        (norm_min(INTPAR(left_hyphen_min)) * 64 +
         norm_min(INTPAR(right_hyphen_min))) * 65536L + cur_lang;
    {
        get_x_token();
        if (cur_cmd != SPACER)
            back_input();
    }
    if (nest_ptr == 1)
        build_page();
}

void get_r_token(void)
{
 lab20:     /*restart */
    do {
        get_token();
    } while (!(cur_tok != SPACE_TOKEN));
    if ((cur_cs == 0) || (cur_cs > eqtb_top)
        || ((cur_cs > FROZEN_CONTROL_SEQUENCE) && (cur_cs <= EQTB_SIZE))) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Missing_control_sequence_ins/*erted*/));
        }
        {
            help_ptr = 5;
            help_line[4] = 66589L /*"Please don't say `\def cs_..._', say `\def\cs_..._'." */ ;
            help_line[3] = S(I_ve_inserted_an_inaccessibl/*e control sequence so that your*/);
            help_line[2] = S(definition_will_be_completed/* without mixing me up too badly.*/);
            help_line[1] = S(You_can_recover_graciously_f/*rom this error, if you're*/);
            help_line[0] = S(careful__see_exercise_27_2_i/*n The TeXbook.*/);
        }
        if (cur_cs == 0)
            back_input();
        cur_tok = (CS_TOKEN_FLAG + 2243226);
        ins_error();
        goto lab20;
    }
}

void trap_zero_glue(void)
{
    memory_word *mem = zmem;

    if ((mem[cur_val + 1].cint == 0) && (mem[cur_val + 2].cint == 0) && (mem[cur_val + 3].cint == 0)) {
        mem[0].hh.v.RH++;
        delete_glue_ref(cur_val);
        cur_val = 0;
    }
}

void do_register_command(small_number a)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t l, q, r, s;
    unsigned char /*mu_val */ p;
    boolean e;
    integer w;

    q = cur_cmd;
    e = false;

    {
        if (q != REGISTER) {
            get_x_token();
            if ((cur_cmd >= ASSIGN_INT) && (cur_cmd <= ASSIGN_MU_GLUE)) {
                l = cur_chr;
                p = cur_cmd - 74;
                goto lab40;
            }
            if (cur_cmd != REGISTER) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(You_can_t_use__));
                }
                print_cmd_chr(cur_cmd, cur_chr);
                print(S(__after_));
                print_cmd_chr(q, 0);
                {
                    help_ptr = 1;
                    help_line[0] = S(I_m_forgetting_what_you_said_Z1/*"I'm forgetting what you said and not changing anything."*/);
                }
                error();
                return;
            }
        }
        if (cur_chr < 0 || cur_chr > 19) {
            l = cur_chr;
            p = (mem[l].hh.u.B0 / 64);
            e = true;
        } else {
            p = cur_chr;
            scan_register_num();
            if (cur_val > 255) {
                find_sa_element(p, cur_val, true);
                l = cur_ptr;
                e = true;
            } else
                switch (p) {
                case 0:
                    l = cur_val + 8938824L;
                    break;
                case 1:
                    l = cur_val + 10053215L;
                    break;
                case 2:
                    l = cur_val + 2252259L;
                    break;
                case 3:
                    l = cur_val + 2252515L;
                    break;
                }
        }
    }
 lab40:/*found */ if (p < GLUE_VAL) {

        if (e)
            w = mem[l + 2].cint;
        else
            w = eqtb[l].cint;
    } else if (e)
        s = mem[l + 1].hh.v.RH;
    else
        s = eqtb[l].hh.v.RH /*:1272 */ ;
    if (q == REGISTER)
        scan_optional_equals();
    else if (scan_keyword(S(by))) ;
    arith_error = false;
    if (q < MULTIPLY) {        /*1273: */

        if (p < GLUE_VAL) {
            if (p == INT_VAL)
                scan_int();
            else
                scan_dimen(false, false, false);
            if (q == ADVANCE)
                cur_val = cur_val + w;
        } else {

            scan_glue(p);
            if (q == ADVANCE) {        /*1274: */
                q = new_spec(cur_val);
                r = s;
                delete_glue_ref(cur_val);
                mem[q + 1].cint = mem[q + 1].cint + mem[r + 1].cint;
                if (mem[q + 2].cint == 0)
                    mem[q].hh.u.B0 = NORMAL;
                if (mem[q].hh.u.B0 == mem[r].hh.u.B0)
                    mem[q + 2].cint = mem[q + 2].cint + mem[r + 2].cint;
                else if ((mem[q].hh.u.B0 < mem[r].hh.u.B0) && (mem[r + 2].cint != 0)) {
                    mem[q + 2].cint = mem[r + 2].cint;
                    mem[q].hh.u.B0 = mem[r].hh.u.B0;
                }
                if (mem[q + 3].cint == 0)
                    mem[q].hh.u.B1 = NORMAL;
                if (mem[q].hh.u.B1 == mem[r].hh.u.B1)
                    mem[q + 3].cint = mem[q + 3].cint + mem[r + 3].cint;
                else if ((mem[q].hh.u.B1 < mem[r].hh.u.B1) && (mem[r + 3].cint != 0)) {
                    mem[q + 3].cint = mem[r + 3].cint;
                    mem[q].hh.u.B1 = mem[r].hh.u.B1;
                }
                cur_val = q;
            }
        }
    } else {                    /*1275: */

        scan_int();
        if (p < GLUE_VAL) {

            if (q == MULTIPLY) {

                if (p == INT_VAL)
                    cur_val = mult_and_add(w, cur_val, 0, TEX_INFINITY);
                else
                    cur_val = mult_and_add(w, cur_val, 0, MAX_HALFWORD);
            } else
                cur_val = x_over_n(w, cur_val);
        } else {

            r = new_spec(s);
            if (q == MULTIPLY) {
                mem[r + 1].cint = mult_and_add(mem[s + 1].cint, cur_val, 0, MAX_HALFWORD);
                mem[r + 2].cint = mult_and_add(mem[s + 2].cint, cur_val, 0, MAX_HALFWORD);
                mem[r + 3].cint = mult_and_add(mem[s + 3].cint, cur_val, 0, MAX_HALFWORD);
            } else {

                mem[r + 1].cint = x_over_n(mem[s + 1].cint, cur_val);
                mem[r + 2].cint = x_over_n(mem[s + 2].cint, cur_val);
                mem[r + 3].cint = x_over_n(mem[s + 3].cint, cur_val);
            }
            cur_val = r;
        }
    }
    if (arith_error) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Arithmetic_overflow));
        }
        {
            help_ptr = 2;
            help_line[1] = S(I_can_t_carry_out_that_multi/*plication or division,*/);
            help_line[0] = S(since_the_result_is_out_of_r/*ange.*/);
        }
        if (p >= GLUE_VAL)
            delete_glue_ref(cur_val);
        error();
        return;
    }
    if (p < GLUE_VAL) {

        if (e) {

            if ((a >= 4))
                gsa_w_def(l, cur_val);
            else
                sa_w_def(l, cur_val);
        } else if ((a >= 4))
            geq_word_define(l, cur_val);
        else
            eq_word_define(l, cur_val);
    } else {

        trap_zero_glue();
        if (e) {

            if ((a >= 4))
                gsa_def(l, cur_val);
            else
                sa_def(l, cur_val);
        } else if ((a >= 4))
            geq_define(l, GLUE_REF, cur_val);
        else
            eq_define(l, GLUE_REF, cur_val);
    }
}

void alter_aux(void)
{
    int32_t c;
    if (cur_chr != abs(cur_list.mode))
        report_illegal_case();
    else {

        c = cur_chr;
        scan_optional_equals();
        if (c == VMODE) {
            scan_dimen(false, false, false);
            cur_list.aux.cint = cur_val;
        } else {

            scan_int();
            if ((cur_val <= 0) || (cur_val > 32767)) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Bad_space_factor));
                }
                {
                    help_ptr = 1;
                    help_line[0] = S(I_allow_only_values_in_the_r/*ange 1..32767 here.*/);
                }
                int_error(cur_val);
            } else
                cur_list.aux.hh.v.LH = cur_val;
        }
    }
}

void alter_prev_graf(void)
{
    integer p;
    nest[nest_ptr] = cur_list;
    p = nest_ptr;
    while (abs(nest[p].mode) != VMODE)
        p--;
    scan_optional_equals();
    scan_int();
    if (cur_val < 0) {
        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Bad_));
        }
        print_esc(S(prevgraf));
        {
            help_ptr = 1;
            help_line[0] = S(I_allow_only_nonnegative_val/*ues here.*/);
        }
        int_error(cur_val);
    } else {

        nest[p].pg = cur_val;
        cur_list = nest[nest_ptr];
    }
}

void alter_page_so_far(void)
{
    unsigned char c;
    c = cur_chr;
    scan_optional_equals();
    scan_dimen(false, false, false);
    page_so_far[c] = cur_val;
}

void alter_integer(void)
{
    small_number c;
    c = cur_chr;
    scan_optional_equals();
    scan_int();
    if (c == 0)
        dead_cycles = /*1483: */ cur_val;
    else if (c == 2) {
        if ((cur_val < BATCH_MODE) || (cur_val > ERROR_STOP_MODE)) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Bad_interaction_mode));
            }
            {
                help_ptr = 2;
                help_line[1] = S(Modes_are_0_batch__1_nonstop/*, 2=scroll, and*/);
                help_line[0] = S(3_errorstop__Proceed__and_I_/*ll ignore this case.*/);
            }
            int_error(cur_val);
        } else {

            cur_chr = cur_val;
            new_interaction();
        }
    } else
        insert_penalties = cur_val;
}

void alter_box_dimen(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    small_number c;
    int32_t b;

    c = cur_chr;
    scan_register_num();
    if (cur_val < 256)
        b = BOX_REG(cur_val);
    else {

        find_sa_element(4, cur_val, false);
        if (cur_ptr == MIN_HALFWORD)
            b = MIN_HALFWORD;
        else
            b = mem[cur_ptr + 1].hh.v.RH;
    }
    scan_optional_equals();
    scan_dimen(false, false, false);
    if (b != MIN_HALFWORD)
        mem[b + c].cint = cur_val;
}

void new_font(small_number a)
{
    CACHE_THE_EQTB;
    int32_t u;
    scaled s;
    internal_font_number f;
    str_number t;
    unsigned char /*max_selector */ old_setting;

    if (job_name == 0)
        open_log_file();

    get_r_token();
    u = cur_cs;
    if (u >= HASH_BASE)
        t = hash[u].v.RH;
    else if (u >= SINGLE_BASE) {

        if (u == NULL_CS)
            t = S(FONT);
        else
            t = u - 1114113L;
    } else {

        old_setting = selector;
        selector = SELECTOR_NEW_STRING ;
        print(S(FONT));
        print(u - 1);
        selector = old_setting;
        {
            if (pool_ptr + 1 > pool_size)
                overflow(S(pool_size), pool_size - init_pool_ptr);
        }
        t = make_string();
    }
    if ((a >= 4))
        geq_define(u, SET_FONT, FONT_BASE);
    else
        eq_define(u, SET_FONT, FONT_BASE);
    scan_optional_equals();
    scan_file_name();
    name_in_progress = true;
    if (scan_keyword(S(at))) {      /*1294: */
        scan_dimen(false, false, false);
        s = cur_val;
        if ((s <= 0) || (s >= 134217728L)) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Improper__at__size__));
            }
            print_scaled(s);
            print(S(pt___replaced_by_10pt));
            {
                help_ptr = 2;
                help_line[1] = S(I_can_only_handle_fonts_at_p/*ositive sizes that are*/);
                help_line[0] = S(less_than_2048pt__so_I_ve_ch/*anged what you said to 10pt.*/);
            }
            error();
            s = 10 * 65536L;
        }
    } else if (scan_keyword(S(scaled))) {
        scan_int();
        s = -(integer) cur_val;
        if ((cur_val <= 0) || (cur_val > 32768L)) {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Illegal_magnification_has_be/*en changed to 1000*/));
            }
            {
                help_ptr = 1;
                help_line[0] = S(The_magnification_ratio_must/* be between 1 and 32768.*/);
            }
            int_error(cur_val);
            s = -1000;
        }
    } else
        s = -1000;
    name_in_progress = false /*:1293 */ ;
    {
        register integer for_end;
        f = (FONT_BASE + 1);
        for_end = font_ptr;
        if (f <= for_end)
            do {
                if (str_eq_str(font_name[f], cur_name)
                    &&
                    (((cur_area == S())
                      && ((font_area[f] == AAT_FONT_FLAG)
                          || (font_area[f] == OTGR_FONT_FLAG))) || str_eq_str(font_area[f], cur_area))) {
                    if (s > 0) {
                        if (s == font_size[f])
                            goto lab50;
                    } else if (font_size[f] == xn_over_d(font_dsize[f], -(integer) s, 1000))
                        goto lab50;
                }
                append_str(cur_area);
                append_str(cur_name);
                append_str(cur_ext);
                if (str_eq_str(font_name[f], make_string())) {
                    {
                        str_ptr--;
                        pool_ptr = str_start[(str_ptr) - 65536L];
                    }
                    if (((font_area[f] == AAT_FONT_FLAG) || (font_area[f] == OTGR_FONT_FLAG))) {
                        if (s > 0) {
                            if (s == font_size[f])
                                goto lab50;
                        } else if (font_size[f] == xn_over_d(font_dsize[f], -(integer) s, 1000))
                            goto lab50;
                    }
                } else {

                    str_ptr--;
                    pool_ptr = str_start[(str_ptr) - 65536L];
                }
            }
            while (f++ < for_end);
    }
    f = read_font_info(u, cur_name, cur_area, s);
 lab50:                        /*common_ending */ if ((a >= 4))
        geq_define(u, SET_FONT, f);
    else
        eq_define(u, SET_FONT, f);
    eqtb[FONT_ID_BASE + f] = eqtb[u];
    hash[FONT_ID_BASE + f].v.RH = t;
}

void new_interaction(void)
{
    print_ln();
    interaction = cur_chr;
    if (interaction == BATCH_MODE)
        selector = SELECTOR_NO_PRINT;
    else
        selector = SELECTOR_TERM_ONLY/*:79 */ ;
    if (log_opened)
        selector = selector + 2;
}

void issue_message(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    unsigned char /*max_selector */ old_setting;
    unsigned char c;
    str_number s;

    c = cur_chr;
    mem[mem_top - 12].hh.v.RH = scan_toks(false, true);
    old_setting = selector;
    selector = SELECTOR_NEW_STRING ;
    token_show(def_ref);
    selector = old_setting;
    flush_list(def_ref);
    {
        if (pool_ptr + 1 > pool_size)
            overflow(S(pool_size), pool_size - init_pool_ptr);
    }
    s = make_string();
    if (c == 0) {               /*1315: */
        if (term_offset + length(s) > max_print_line - 2)
            print_ln();
        else if ((term_offset > 0) || (file_offset > 0))
            print_char(32 /*" " */ );
        print(s);
        ttstub_output_flush (rust_stdout);
    } else {                    /*1318: */

        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S());
        }
        print(s);
        if (LOCAL(err_help) != MIN_HALFWORD)
            use_err_help = true;
        else if (long_help_seen) {
            help_ptr = 1;
            help_line[0] = S(_That_was_another__errmessag/*e.)*/);
        } else {

            if (interaction < ERROR_STOP_MODE)
                long_help_seen = true;
            {
                help_ptr = 4;
                help_line[3] = S(This_error_message_was_gener/*ated by an \errmessage*/);
                help_line[2] = S(command__so_I_can_t_give_any/* explicit help.*/);
                help_line[1] = S(Pretend_that_you_re_Hercule_/*Poirot: Examine all clues,*/);
                help_line[0] = S(and_deduce_the_truth_by_orde/*r and method.*/);
            }
        }
        error();
        use_err_help = false;
    }
    {
        str_ptr--;
        pool_ptr = str_start[(str_ptr) - 65536L];
    }
}

void shift_case(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t b;
    int32_t p;
    int32_t t;
    integer c;

    b = cur_chr;
    p = scan_toks(false, false);
    p = mem[def_ref].hh.v.RH;

    while (p != MIN_HALFWORD) {

        t = mem[p].hh.v.LH;
        if (t < (CS_TOKEN_FLAG + 1114113)) {
            c = t % MAX_CHAR_VAL;
            if (eqtb[b + c].hh.v.RH != 0)
                mem[p].hh.v.LH = t - c + eqtb[b + c].hh.v.RH;
        }
        p = mem[p].hh.v.RH;
    }
    begin_token_list(mem[def_ref].hh.v.RH, BACKED_UP);
    {
        mem[def_ref].hh.v.RH = avail;
        avail = def_ref;
    }
}

void show_whatever(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p;
    small_number t;
    unsigned char /*or_code */ m;
    integer l;
    integer n;

    switch (cur_chr) {
    case 3:
        {
            begin_diagnostic();
            show_activities();
        }
        break;
    case 1:
        {
            scan_register_num();
            if (cur_val < 256)
                p = BOX_REG(cur_val);
            else {

                find_sa_element(4, cur_val, false);
                if (cur_ptr == MIN_HALFWORD)
                    p = MIN_HALFWORD;
                else
                    p = mem[cur_ptr + 1].hh.v.RH;
            }
            begin_diagnostic();
            print_nl(S(___box));
            print_int(cur_val);
            print_char(61 /*"=" */ );
            if (p == MIN_HALFWORD)
                print(S(void));
            else
                show_box(p);
        }
        break;
    case 0:
        {
            get_token();
            print_nl(S(___Z18/*"> "*/));
            if (cur_cs != 0) {
                sprint_cs(cur_cs);
                print_char(61 /*"=" */ );
            }
            print_meaning();
            goto lab50;
        }
        break;
    case 4:
        {
            begin_diagnostic();
            show_save_groups();
        }
        break;
    case 6:
        {
            begin_diagnostic();
            print_nl(S());
            print_ln();
            if (cond_ptr == MIN_HALFWORD) {
                print_nl(S(____/*"### "*/));
                print(S(no_active_conditionals));
            } else {

                p = cond_ptr;
                n = 0;
                do {
                    n++;
                    p = mem[p].hh.v.RH;
                } while (!(p == MIN_HALFWORD));
                p = cond_ptr;
                t = cur_if;
                l = if_line;
                m = if_limit;
                do {
                    print_nl(S(____level_));
                    print_int(n);
                    print(S(___Z3/*": "*/));
                    print_cmd_chr(IF_TEST, t);
                    if (m == FI_CODE)
                        print_esc(S(else));
                    if (l != 0) {
                        print(S(_entered_on_line_));
                        print_int(l);
                    }
                    n--;
                    t = mem[p].hh.u.B1;
                    l = mem[p + 1].cint;
                    m = mem[p].hh.u.B0;
                    p = mem[p].hh.v.RH;
                } while (!(p == MIN_HALFWORD));
            }
        }
        break;
    default:
        {
            p = the_toks();
            print_nl(S(___Z18/*"> "*/));
            token_show(mem_top - 3);
            flush_list(mem[mem_top - 3].hh.v.RH);
            goto lab50;
        }
        break;
    }
    end_diagnostic(true);
    {
        if (file_line_error_style_p)
            print_file_line();
        else
            print_nl(S(__/*"! "*/));
        print(S(OK));
    }
    if (selector == SELECTOR_TERM_AND_LOG) {

        if (INTPAR(tracing_online) <= 0) {
            selector = SELECTOR_TERM_ONLY;
            print(S(__see_the_transcript_file_));
            selector = SELECTOR_TERM_AND_LOG;
        }
    }
 lab50:/*common_ending */ if (interaction < ERROR_STOP_MODE) {
        help_ptr = 0;
        error_count--;
    } else if (INTPAR(tracing_online) > 0) {
        {
            help_ptr = 3;
            help_line[2] = S(This_isn_t_an_error_message_/* I'm just \showing something.*/);
            help_line[1] = S(Type__I_show_____to_show_mor/*e (e.g., \show\cs,*/);
            help_line[0] = S(_showthe_count10___showbox25/*5, \showlists).*/);
        }
    } else {

        {
            help_ptr = 5;
            help_line[4] = S(This_isn_t_an_error_message_/* I'm just \showing something.*/);
            help_line[3] = S(Type__I_show_____to_show_mor/*e (e.g., \show\cs,*/);
            help_line[2] = S(_showthe_count10___showbox25/*5, \showlists).*/);
            help_line[1] = S(And_type__I_tracingonline_1_/*show...' to show boxes and*/);
            help_line[0] = S(lists_on_your_terminal_as_we/*ll as in the transcript file.*/);
        }
    }
    error();
}

void new_write_whatsit(small_number w)
{
    memory_word *mem = zmem; new_whatsit(cur_chr, w);
    if (w != WRITE_NODE_SIZE)
        scan_four_bit_int();
    else {

        scan_int();
        if (cur_val < 0)
            cur_val = 17;
        else if ((cur_val > 15) && (cur_val != 18))
            cur_val = 16;
    }
    mem[cur_list.tail + 1].hh.v.LH = cur_val;
}

void load_picture(boolean is_pdf)
{
    memory_word *mem = zmem; char *pic_path;
    real_rect bounds;
    transform_t t, t2;
    real_point corners[4];
    double x_size_req, y_size_req;
    boolean check_keywords;
    double xmin, xmax, ymin, ymax;
    small_number i;
    integer page;
    integer pdf_box_type;
    integer result;
    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
    pdf_box_type = 0;
    page = 0;
    if (is_pdf) {
        if (scan_keyword(S(page))) {
            scan_int();
            page = cur_val;
        }
        pdf_box_type = pdfbox_none;
        if (scan_keyword(S(crop)))
            pdf_box_type = pdfbox_crop;
        else if (scan_keyword(S(media)))
            pdf_box_type = pdfbox_media;
        else if (scan_keyword(S(bleed)))
            pdf_box_type = pdfbox_bleed;
        else if (scan_keyword(S(trim)))
            pdf_box_type = pdfbox_trim;
        else if (scan_keyword(S(art)))
            pdf_box_type = pdfbox_art;
    }
    if (pdf_box_type == pdfbox_none)
        result = find_pic_file(&pic_path, &bounds, pdfbox_crop, page);
    else
        result = find_pic_file(&pic_path, &bounds, pdf_box_type, page);
    SET_POINT(corners[0], bounds.x, bounds.y);
    SET_POINT(corners[1], corners[0].x, bounds.y + bounds.ht);
    SET_POINT(corners[2], bounds.x + bounds.wd, corners[1].y);
    SET_POINT(corners[3], corners[2].x, corners[0].y);
    x_size_req = 0.0;
    y_size_req = 0.0;
    make_identity(&t);
    check_keywords = true;
    while (check_keywords) {

        if (scan_keyword(S(scaled))) {
            scan_int();
            if ((x_size_req == 0.0) && (y_size_req == 0.0)) {
                make_scale(&t2, cur_val / ((double)1000.0), cur_val / ((double)1000.0));
                {
                    register integer for_end;
                    i = 0;
                    for_end = 3;
                    if (i <= for_end)
                        do
                            transform_point(&corners[i], &t2);
                        while (i++ < for_end);
                }
                transform_concat(&t, &t2);
            }
        } else if (scan_keyword(S(xscaled))) {
            scan_int();
            if ((x_size_req == 0.0) && (y_size_req == 0.0)) {
                make_scale(&t2, cur_val / ((double)1000.0), 1.0);
                {
                    register integer for_end;
                    i = 0;
                    for_end = 3;
                    if (i <= for_end)
                        do
                            transform_point(&corners[i], &t2);
                        while (i++ < for_end);
                }
                transform_concat(&t, &t2);
            }
        } else if (scan_keyword(S(yscaled))) {
            scan_int();
            if ((x_size_req == 0.0) && (y_size_req == 0.0)) {
                make_scale(&t2, 1.0, cur_val / ((double)1000.0));
                {
                    register integer for_end;
                    i = 0;
                    for_end = 3;
                    if (i <= for_end)
                        do
                            transform_point(&corners[i], &t2);
                        while (i++ < for_end);
                }
                transform_concat(&t, &t2);
            }
        } else if (scan_keyword(S(width))) {
            scan_dimen(false, false, false);
            if (cur_val <= 0) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Improper_image_));
                }
                print(S(size__));
                print_scaled(cur_val);
                print(S(pt__will_be_ignored));
                {
                    help_ptr = 2;
                    help_line[1] = S(I_can_t_scale_images_to_zero/* or negative sizes,*/);
                    help_line[0] = S(so_I_m_ignoring_this_);
                }
                error();
            } else
                x_size_req = Fix2D(cur_val);
        } else if (scan_keyword(S(height))) {
            scan_dimen(false, false, false);
            if (cur_val <= 0) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Improper_image_));
                }
                print(S(size__));
                print_scaled(cur_val);
                print(S(pt__will_be_ignored));
                {
                    help_ptr = 2;
                    help_line[1] = S(I_can_t_scale_images_to_zero/* or negative sizes,*/);
                    help_line[0] = S(so_I_m_ignoring_this_);
                }
                error();
            } else
                y_size_req = Fix2D(cur_val);
        } else if (scan_keyword(S(rotated))) {
            scan_decimal();
            if ((x_size_req != 0.0) || (y_size_req != 0.0)) {
                {
                    xmin = 1000000.0;
                    xmax = -(integer) xmin;
                    ymin = xmin;
                    ymax = xmax;
                    {
                        register integer for_end;
                        i = 0;
                        for_end = 3;
                        if (i <= for_end)
                            do {
                                if (corners[i].x < xmin)
                                    xmin = corners[i].x;
                                if (corners[i].x > xmax)
                                    xmax = corners[i].x;
                                if (corners[i].y < ymin)
                                    ymin = corners[i].y;
                                if (corners[i].y > ymax)
                                    ymax = corners[i].y;
                            }
                            while (i++ < for_end);
                    }
                }
                if (x_size_req == 0.0) {
                    make_scale(&t2, y_size_req / ((double)(ymax - ymin)),
                               y_size_req / ((double)(ymax - ymin)));
                } else if (y_size_req == 0.0) {
                    make_scale(&t2, x_size_req / ((double)(xmax - xmin)),
                               x_size_req / ((double)(xmax - xmin)));
                } else {

                    make_scale(&t2, x_size_req / ((double)(xmax - xmin)),
                               y_size_req / ((double)(ymax - ymin)));
                }
                {
                    register integer for_end;
                    i = 0;
                    for_end = 3;
                    if (i <= for_end)
                        do
                            transform_point(&corners[i], &t2);
                        while (i++ < for_end);
                }
                x_size_req = 0.0;
                y_size_req = 0.0;
                transform_concat(&t, &t2);
            }
            make_rotation(&t2, Fix2D(cur_val) * 3.141592653589793 / ((double)180.0));
            {
                register integer for_end;
                i = 0;
                for_end = 3;
                if (i <= for_end)
                    do
                        transform_point(&corners[i], &t2);
                    while (i++ < for_end);
            }
            {
                xmin = 1000000.0;
                xmax = -(integer) xmin;
                ymin = xmin;
                ymax = xmax;
                {
                    register integer for_end;
                    i = 0;
                    for_end = 3;
                    if (i <= for_end)
                        do {
                            if (corners[i].x < xmin)
                                xmin = corners[i].x;
                            if (corners[i].x > xmax)
                                xmax = corners[i].x;
                            if (corners[i].y < ymin)
                                ymin = corners[i].y;
                            if (corners[i].y > ymax)
                                ymax = corners[i].y;
                        }
                        while (i++ < for_end);
                }
            }
            SET_POINT(corners[0], xmin, ymin);
            SET_POINT(corners[1], xmin, ymax);
            SET_POINT(corners[2], xmax, ymax);
            SET_POINT(corners[3], xmax, ymin);
            transform_concat(&t, &t2);
        } else
            check_keywords = false;
    }
    if ((x_size_req != 0.0) || (y_size_req != 0.0)) {
        {
            xmin = 1000000.0;
            xmax = -(integer) xmin;
            ymin = xmin;
            ymax = xmax;
            {
                register integer for_end;
                i = 0;
                for_end = 3;
                if (i <= for_end)
                    do {
                        if (corners[i].x < xmin)
                            xmin = corners[i].x;
                        if (corners[i].x > xmax)
                            xmax = corners[i].x;
                        if (corners[i].y < ymin)
                            ymin = corners[i].y;
                        if (corners[i].y > ymax)
                            ymax = corners[i].y;
                    }
                    while (i++ < for_end);
            }
        }
        if (x_size_req == 0.0) {
            make_scale(&t2, y_size_req / ((double)(ymax - ymin)), y_size_req / ((double)(ymax - ymin)));
        } else if (y_size_req == 0.0) {
            make_scale(&t2, x_size_req / ((double)(xmax - xmin)), x_size_req / ((double)(xmax - xmin)));
        } else {

            make_scale(&t2, x_size_req / ((double)(xmax - xmin)), y_size_req / ((double)(ymax - ymin)));
        }
        {
            register integer for_end;
            i = 0;
            for_end = 3;
            if (i <= for_end)
                do
                    transform_point(&corners[i], &t2);
                while (i++ < for_end);
        }
        x_size_req = 0.0;
        y_size_req = 0.0;
        transform_concat(&t, &t2);
    }
    {
        xmin = 1000000.0;
        xmax = -(integer) xmin;
        ymin = xmin;
        ymax = xmax;
        {
            register integer for_end;
            i = 0;
            for_end = 3;
            if (i <= for_end)
                do {
                    if (corners[i].x < xmin)
                        xmin = corners[i].x;
                    if (corners[i].x > xmax)
                        xmax = corners[i].x;
                    if (corners[i].y < ymin)
                        ymin = corners[i].y;
                    if (corners[i].y > ymax)
                        ymax = corners[i].y;
                }
                while (i++ < for_end);
        }
    }
    make_translation(&t2, -(integer) xmin * 72 / ((double)72.27), -(integer) ymin * 72 / ((double)72.27));
    transform_concat(&t, &t2);
    if (result == 0) {
        new_whatsit(PIC_NODE,
                    PIC_NODE_SIZE + (strlen(pic_path) + sizeof(memory_word) - 1) / sizeof(memory_word));
        if (is_pdf) {
            mem[cur_list.tail].hh.u.B1 = PDF_NODE;
        }
        mem[cur_list.tail + 4].hh.u.B0 = strlen(pic_path);
        mem[cur_list.tail + 4].hh.u.B1 = page;
        mem[cur_list.tail + 8].hh.u.B0 = pdf_box_type;
        mem[cur_list.tail + 1].cint = D2Fix(xmax - xmin);
        mem[cur_list.tail + 3].cint = D2Fix(ymax - ymin);
        mem[cur_list.tail + 2].cint = 0;
        mem[cur_list.tail + 5].hh.v.LH = D2Fix(t.a);
        mem[cur_list.tail + 5].hh.v.RH = D2Fix(t.b);
        mem[cur_list.tail + 6].hh.v.LH = D2Fix(t.c);
        mem[cur_list.tail + 6].hh.v.RH = D2Fix(t.d);
        mem[cur_list.tail + 7].hh.v.LH = D2Fix(t.x);
        mem[cur_list.tail + 7].hh.v.RH = D2Fix(t.y);
        memcpy(&mem[cur_list.tail + 9], pic_path, strlen(pic_path));
        free(pic_path);
    } else {

        {
            if (file_line_error_style_p)
                print_file_line();
            else
                print_nl(S(__/*"! "*/));
            print(S(Unable_to_load_picture_or_PD/*F file '*/));
        }
        print_file_name(cur_name, cur_area, cur_ext);
        print(39 /*"'" */ );
        if (result == -43) {
            {
                help_ptr = 2;
                help_line[1] = S(The_requested_image_couldn_t/* be read because*/);
                help_line[0] = S(the_file_was_not_found_);
            }
        } else {

            {
                help_ptr = 2;
                help_line[1] = S(The_requested_image_couldn_t/* be read because*/);
                help_line[0] = S(it_was_not_a_recognized_imag/*e format.*/);
            }
        }
        error();
    }
}

void scan_and_pack_name(void)
{
    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
}

void do_extension(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer i, j, k;
    int32_t p;

    switch (cur_chr) {
    case 0:
        {
            new_write_whatsit(OPEN_NODE_SIZE);
            scan_optional_equals();
            scan_file_name();
            mem[cur_list.tail + 1].hh.v.RH = cur_name;
            mem[cur_list.tail + 2].hh.v.LH = cur_area;
            mem[cur_list.tail + 2].hh.v.RH = cur_ext;
        }
        break;
    case 1:
        {
            k = cur_cs;
            new_write_whatsit(WRITE_NODE_SIZE);
            cur_cs = k;
            p = scan_toks(false, false);
            mem[cur_list.tail + 1].hh.v.RH = def_ref;
        }
        break;
    case 2:
        {
            new_write_whatsit(WRITE_NODE_SIZE);
            mem[cur_list.tail + 1].hh.v.RH = MIN_HALFWORD;
        }
        break;
    case 3:
        {
            new_whatsit(SPECIAL_NODE, WRITE_NODE_SIZE);
            mem[cur_list.tail + 1].hh.v.LH = MIN_HALFWORD;
            p = scan_toks(false, true);
            mem[cur_list.tail + 1].hh.v.RH = def_ref;
        }
        break;
    case 4:
        {
            get_x_token();
            if ((cur_cmd == EXTENSION) && (cur_chr <= CLOSE_NODE)) {
                p = cur_list.tail;
                do_extension();
                out_what(cur_list.tail);
                flush_node_list(cur_list.tail);
                cur_list.tail = p;
                mem[p].hh.v.RH = MIN_HALFWORD;
            } else
                back_input();
        }
        break;
    case 5:
        if (abs(cur_list.mode) != HMODE)
            report_illegal_case();
        else {

            new_whatsit(LANGUAGE_NODE, SMALL_NODE_SIZE);
            scan_int();
            if (cur_val <= 0)
                cur_list.aux.hh.v.RH = 0;
            else if (cur_val > 255)
                cur_list.aux.hh.v.RH = 0;
            else
                cur_list.aux.hh.v.RH = cur_val;
            mem[cur_list.tail + 1].hh.v.RH = cur_list.aux.hh.v.RH;
            mem[cur_list.tail + 1].hh.u.B0 = norm_min(INTPAR(left_hyphen_min));
            mem[cur_list.tail + 1].hh.u.B1 = norm_min(INTPAR(right_hyphen_min));
        }
        break;
    case 41:
        if (abs(cur_list.mode) == MMODE)
            report_illegal_case();
        else
            load_picture(false);
        break;
    case 42:
        if (abs(cur_list.mode) == MMODE)
            report_illegal_case();
        else
            load_picture(true);
        break;
    case 43:
        {
            if (abs(cur_list.mode) == VMODE) {
                back_input();
                new_graf(true);
            } else if (abs(cur_list.mode) == MMODE)
                report_illegal_case();
            else {

                if (((font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == AAT_FONT_FLAG)
                     || (font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == OTGR_FONT_FLAG))) {
                    new_whatsit(GLYPH_NODE, GLYPH_NODE_SIZE);
                    scan_int();
                    if ((cur_val < 0) || (cur_val > 65535L)) {
                        {
                            if (file_line_error_style_p)
                                print_file_line();
                            else
                                print_nl(S(__/*"! "*/));
                            print(S(Bad_glyph_number));
                        }
                        {
                            help_ptr = 2;
                            help_line[1] = S(A_glyph_number_must_be_betwe/*en 0 and 65535.*/);
                            help_line[0] = S(I_changed_this_one_to_zero_);
                        }
                        int_error(cur_val);
                        cur_val = 0;
                    }
                    mem[cur_list.tail + 4].qqqq.u.B1 = eqtb[CUR_FONT_LOC].hh.v.RH;
                    mem[cur_list.tail + 4].qqqq.u.B2 = cur_val;
                    set_native_glyph_metrics(cur_list.tail, (STATEINT(xetex_use_glyph_metrics) > 0));
                } else
                    not_native_font_error(EXTENSION, GLYPH_CODE,
                                          eqtb[CUR_FONT_LOC].hh.v.RH);
            }
        }
        break;
    case 44:
        {
            scan_and_pack_name();
            i = get_encoding_mode_and_info(&j);
            if (i == XETEX_INPUT_MODE_AUTO) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Encoding_mode__auto__is_not_/*valid for \XeTeXinputencoding*/));
                }
                {
                    help_ptr = 2;
                    help_line[1] = S(You_can_t_use__auto__encodin/*g here, only for \XeTeXdefaultencoding.*/);
                    help_line[0] = S(I_ll_ignore_this_and_leave_t/*he current encoding unchanged.*/);
                }
                error();
            } else
                set_input_file_encoding(input_file[in_open], i, j);
        }
        break;
    case 45:
        {
            scan_and_pack_name();
            i = get_encoding_mode_and_info(&j);
            STATEINT(xetex_default_input_mode) = i;
            STATEINT(xetex_default_input_encoding) = j;
        }
        break;
    case 46:
        {
            scan_file_name();
            if (length(cur_name) == 0)
                INTPAR(xetex_linebreak_locale) = 0;
            else
                INTPAR(xetex_linebreak_locale) = cur_name;
        }
        break;
    case 6:
        {
            new_whatsit(PDFTEX_FIRST_EXTENSION_CODE, SMALL_NODE_SIZE);
        }
        break;
    default:
        confusion(S(ext1));
        break;
    }
}

void fix_language(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    UTF16_code l;

    if (INTPAR(language) <= 0)
        l = 0;
    else if (INTPAR(language) > 255)
        l = 0;
    else
        l = INTPAR(language);
    if (l != cur_list.aux.hh.v.RH) {
        new_whatsit(LANGUAGE_NODE, SMALL_NODE_SIZE);
        mem[cur_list.tail + 1].hh.v.RH = l;
        cur_list.aux.hh.v.RH = l;
        mem[cur_list.tail + 1].hh.u.B0 = norm_min(INTPAR(left_hyphen_min));
        mem[cur_list.tail + 1].hh.u.B1 = norm_min(INTPAR(right_hyphen_min));
    }
}

void insert_src_special(void)
{
    memory_word *mem = zmem; int32_t toklist, p, q;
    if ((source_filename_stack[in_open] > 0 && is_new_source(source_filename_stack[in_open], line))) {
        toklist = get_avail();
        p = toklist;
        mem[p].hh.v.LH = (CS_TOKEN_FLAG + 2243236);
        mem[p].hh.v.RH = get_avail();
        p = mem[p].hh.v.RH;
        mem[p].hh.v.LH = (LEFT_BRACE_TOKEN + 123);
        q = str_toks(make_src_special(source_filename_stack[in_open], line));
        mem[p].hh.v.RH = mem[mem_top - 3].hh.v.RH;
        p = q;
        mem[p].hh.v.RH = get_avail();
        p = mem[p].hh.v.RH;
        mem[p].hh.v.LH = (RIGHT_BRACE_TOKEN + 125);
        begin_token_list(toklist, INSERTED);
        remember_source_info(source_filename_stack[in_open], line);
    }
}

void append_src_special(void)
{
    register memory_word *mem = zmem;

    if ((source_filename_stack[in_open] > 0 && is_new_source(source_filename_stack[in_open], line))) {
        new_whatsit(SPECIAL_NODE, WRITE_NODE_SIZE);
        mem[cur_list.tail + 1].hh.v.LH = 0;
        def_ref = get_avail();
        mem[def_ref].hh.v.LH = MIN_HALFWORD;
        str_toks(make_src_special(source_filename_stack[in_open], line));
        mem[def_ref].hh.v.RH = mem[mem_top - 3].hh.v.RH;
        mem[cur_list.tail + 1].hh.v.RH = def_ref;
        remember_source_info(source_filename_stack[in_open], line);
    }
}

void handle_right_brace(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    int32_t p, q;
    scaled d;
    integer f;

    switch (cur_group) {
    case 1:
        unsave();
        break;
    case 0:
        {
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(66440L /*"Too many _'s" */ );
            }
            {
                help_ptr = 2;
                help_line[1] = S(You_ve_closed_more_groups_th/*an you opened.*/);
                help_line[0] = S(Such_booboos_are_generally_h/*armless, so keep going.*/);
            }
            error();
        }
        break;
    case 14:
    case 15:
    case 16:
        extra_right_brace();
        break;
    case 2:
        package(0);
        break;
    case 3:
        {
            adjust_tail = mem_top - 5;
            pre_adjust_tail = mem_top - 14;
            package(0);
        }
        break;
    case 4:
        {
            end_graf();
            package(0);
        }
        break;
    case 5:
        {
            end_graf();
            package(VTOP_CODE);
        }
        break;
    case 11:
        {
            end_graf();
            q = GLUEPAR(split_top_skip);
            mem[q].hh.v.RH++;
            d = DIMENPAR(split_max_depth);
            f = INTPAR(floating_penalty);
            unsave();
            save_ptr = save_ptr - 2;
            p = vpackage(mem[cur_list.head].hh.v.RH, 0, ADDITIONAL, MAX_HALFWORD);
            pop_nest();
            if (save_stack[save_ptr + 0].cint < 255) {
                {
                    mem[cur_list.tail].hh.v.RH = get_node(INS_NODE_SIZE);
                    cur_list.tail = mem[cur_list.tail].hh.v.RH;
                }
                mem[cur_list.tail].hh.u.B0 = INS_NODE;
                mem[cur_list.tail].hh.u.B1 = save_stack[save_ptr + 0].cint;
                mem[cur_list.tail + 3].cint = mem[p + 3].cint + mem[p + 2].cint;
                mem[cur_list.tail + 4].hh.v.LH = mem[p + 5].hh.v.RH;
                mem[cur_list.tail + 4].hh.v.RH = q;
                mem[cur_list.tail + 2].cint = d;
                mem[cur_list.tail + 1].cint = f;
            } else {

                {
                    mem[cur_list.tail].hh.v.RH = get_node(SMALL_NODE_SIZE);
                    cur_list.tail = mem[cur_list.tail].hh.v.RH;
                }
                mem[cur_list.tail].hh.u.B0 = ADJUST_NODE;
                mem[cur_list.tail].hh.u.B1 = save_stack[save_ptr + 1].cint;
                mem[cur_list.tail + 1].cint = mem[p + 5].hh.v.RH;
                delete_glue_ref(q);
            }
            free_node(p, BOX_NODE_SIZE);
            if (nest_ptr == 0)
                build_page();
        }
        break;
    case 8:
        {
            if ((cur_input.loc != MIN_HALFWORD) || ((cur_input.index != OUTPUT_TEXT) && (cur_input.index != BACKED_UP))) {     /*1062: */
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Unbalanced_output_routine));
                }
                {
                    help_ptr = 2;
                    help_line[1] = 66409L /*"Your sneaky output routine has problematic _'s and/or _'s." */ ;
                    help_line[0] = S(I_can_t_handle_that_very_wel/*l; good luck.*/);
                }
                error();
                do {
                    get_token();
                } while (!(cur_input.loc == MIN_HALFWORD));
            }
            end_token_list();
            end_graf();
            unsave();
            output_active = false;
            insert_penalties = 0;
            if (BOX_REG(255) != MIN_HALFWORD) {
                {
                    if (file_line_error_style_p)
                        print_file_line();
                    else
                        print_nl(S(__/*"! "*/));
                    print(S(Output_routine_didn_t_use_al/*l of */));
                }
                print_esc(S(box));
                print_int(255);
                {
                    help_ptr = 3;
                    help_line[2] = S(Your__output_commands_should/* empty \box255,*/);
                    help_line[1] = S(e_g___by_saying___shipout_bo/*x255'.*/);
                    help_line[0] = S(Proceed__I_ll_discard_its_pr/*esent contents.*/);
                }
                box_error(255);
            }
            if (cur_list.tail != cur_list.head) {
                mem[page_tail].hh.v.RH = mem[cur_list.head].hh.v.RH;
                page_tail = cur_list.tail;
            }
            if (mem[mem_top - 2].hh.v.RH != MIN_HALFWORD) {
                if (mem[mem_top - 1].hh.v.RH == MIN_HALFWORD)
                    nest[0].tail = page_tail;
                mem[page_tail].hh.v.RH = mem[mem_top - 1].hh.v.RH;
                mem[mem_top - 1].hh.v.RH = mem[mem_top - 2].hh.v.RH;
                mem[mem_top - 2].hh.v.RH = MIN_HALFWORD;
                page_tail = mem_top - 2;
            }
            flush_node_list(disc_ptr[LAST_BOX_CODE]);
            disc_ptr[LAST_BOX_CODE] = MIN_HALFWORD;
            pop_nest();
            build_page();
        }
        break;
    case 10:
        build_discretionary();
        break;
    case 6:
        {
            back_input();
            cur_tok = (CS_TOKEN_FLAG + 2243227);
            {
                if (file_line_error_style_p)
                    print_file_line();
                else
                    print_nl(S(__/*"! "*/));
                print(S(Missing_));
            }
            print_esc(S(cr));
            print(S(_inserted));
            {
                help_ptr = 1;
                help_line[0] = S(I_m_guessing_that_you_meant_/*to end an alignment here.*/);
            }
            ins_error();
        }
        break;
    case 7:
        {
            end_graf();
            unsave();
            align_peek();
        }
        break;
    case 12:
        {
            end_graf();
            unsave();
            save_ptr = save_ptr - 2;
            p = vpackage(mem[cur_list.head].hh.v.RH, save_stack[save_ptr + 1].cint, save_stack[save_ptr + 0].cint,
                         MAX_HALFWORD);
            pop_nest();
            {
                mem[cur_list.tail].hh.v.RH = new_noad();
                cur_list.tail = mem[cur_list.tail].hh.v.RH;
            }
            mem[cur_list.tail].hh.u.B0 = VCENTER_NOAD;
            mem[cur_list.tail + 1].hh.v.RH = SUB_BOX;
            mem[cur_list.tail + 1].hh.v.LH = p;
        }
        break;
    case 13:
        build_choices();
        break;
    case 9:
        {
            unsave();
            save_ptr--;
            mem[save_stack[save_ptr + 0].cint].hh.v.RH = SUB_MLIST;
            p = fin_mlist(MIN_HALFWORD);
            mem[save_stack[save_ptr + 0].cint].hh.v.LH = p;
            if (p != MIN_HALFWORD) {

                if (mem[p].hh.v.RH == MIN_HALFWORD) {

                    if (mem[p].hh.u.B0 == ORD_NOAD) {
                        if (mem[p + 3].hh.v.RH == EMPTY) {

                            if (mem[p + 2].hh.v.RH == EMPTY) {
                                mem[save_stack[save_ptr + 0].cint].hh = mem[p + 1].hh;
                                free_node(p, NOAD_SIZE);
                            }
                        }
                    } else if (mem[p].hh.u.B0 == ACCENT_NOAD) {

                        if (save_stack[save_ptr + 0].cint == cur_list.tail + 1) {

                            if (mem[cur_list.tail].hh.u.B0 == ORD_NOAD) {  /*1222: */
                                q = cur_list.head;
                                while (mem[q].hh.v.RH != cur_list.tail)
                                    q = mem[q].hh.v.RH;
                                mem[q].hh.v.RH = p;
                                free_node(cur_list.tail, NOAD_SIZE);
                                cur_list.tail = p;
                            }
                        }
                    }
                }
            }
        }
        break;
    default:
        confusion(S(rightbrace));
        break;
    }
}

void main_control(void)
{
    CACHE_THE_EQTB;
    memory_word *mem = zmem;
    integer t;

    if (LOCAL(every_job) != MIN_HALFWORD)
        begin_token_list(LOCAL(every_job), EVERY_JOB_TEXT);

lab60: /* big_switch */
    get_x_token();

lab21: /* reswitch */
    /*1066: */

    if (INTPAR(tracing_commands) > 0)
        show_cur_cmd_chr();
    switch (abs(cur_list.mode) + cur_cmd) {
    case 115:
    case 116:
    case 172:
        goto lab70;
        break;
    case 120:
        {
            scan_usv_num();
            cur_chr = cur_val;
            goto lab70;
        }
        break;
    case 169:
        {
            get_x_token();
            if ((cur_cmd == LETTER) || (cur_cmd == OTHER_CHAR) || (cur_cmd == CHAR_GIVEN)
                || (cur_cmd == CHAR_NUM))
                cancel_boundary = true;
            goto lab21;
        }
        break;
    default:
        {
            if (abs(cur_list.mode) == HMODE) {

                if ((STATEINT(xetex_inter_char_tokens) > 0) && (space_class != CHAR_CLASS_LIMIT)
                    && (prev_class != ((CHAR_CLASS_LIMIT - 1)))) {
                    prev_class = ((CHAR_CLASS_LIMIT - 1));
                    find_sa_element(INTER_CHAR_VAL,
                                    space_class * CHAR_CLASS_LIMIT + ((CHAR_CLASS_LIMIT - 1)),
                                    false);
                    if (cur_ptr != MIN_HALFWORD) {
                        if (cur_cs == 0) {
                            if (cur_cmd == CHAR_NUM)
                                cur_cmd = OTHER_CHAR;
                            cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                        } else
                            cur_tok = CS_TOKEN_FLAG + cur_cs;
                        back_input();
                        begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
                        goto lab60;
                    }
                }
            }
            switch (abs(cur_list.mode) + cur_cmd) {
            case 114:
                if (cur_list.aux.hh.v.LH == 1000)
                    goto lab120;
                else
                    app_space();
                break;
            case 168:
            case 271:
                goto lab120;
                break;
            case 1:
            case 104:
            case 207:
            case 11:
            case 217:
            case 272:
                ;
                break;
            case 40:
            case 143:
            case 246:
                {
                    if (cur_chr == 0) {
                        do {
                            get_x_token();
                        } while (!(cur_cmd != 10 /*spacer *//*:424 */ ));
                        goto lab21;
                    } else {

                        t = scanner_status;
                        scanner_status = NORMAL;
                        get_next();
                        scanner_status = t;
                        if (cur_cs < HASH_BASE)
                            cur_cs = prim_lookup(cur_cs - 257);
                        else
                            cur_cs = prim_lookup(hash[cur_cs].v.RH);
                        if (cur_cs != UNDEFINED_PRIMITIVE) {
                            cur_cmd = prim_eqtb[cur_cs].hh.u.B0;
                            cur_chr = prim_eqtb[cur_cs].hh.v.RH;
                            goto lab21;
                        }
                    }
                }
                break;
            case 15:
                if (its_all_over())
                    return;
                break;
            case 23:
            case 125:
            case 228:
            case 72:
            case 175:
            case 278:
            case 39:
            case 45:
            case 49:
            case 152:
            case 7:
            case 110:
            case 213:
                report_illegal_case();
                break;
            case 8:
            case 111:
            case 9:
            case 112:
            case 18:
            case 121:
            case 70:
            case 173:
            case 71:
            case 174:
            case 51:
            case 154:
            case 16:
            case 119:
            case 50:
            case 153:
            case 53:
            case 156:
            case 67:
            case 170:
            case 54:
            case 157:
            case 55:
            case 158:
            case 57:
            case 160:
            case 56:
            case 159:
            case 31:
            case 134:
            case 52:
            case 155:
            case 29:
            case 132:
            case 47:
            case 150:
            case 216:
            case 220:
            case 221:
            case 234:
            case 231:
            case 240:
            case 243:
                insert_dollar_sign();
                break;
            case 37:
            case 139:
            case 242:
                {
                    {
                        mem[cur_list.tail].hh.v.RH = scan_rule_spec();
                        cur_list.tail = mem[cur_list.tail].hh.v.RH;
                    }
                    if (abs(cur_list.mode) == VMODE)
                        cur_list.aux.cint = -65536000L;
                    else if (abs(cur_list.mode) == HMODE)
                        cur_list.aux.hh.v.LH = 1000;
                }
                break;
            case 28:
            case 130:
            case 233:
            case 235:
                append_glue();
                break;
            case 30:
            case 133:
            case 236:
            case 237:
                append_kern();
                break;
            case 2:
            case 105:
                new_save_level(SIMPLE_GROUP);
                break;
            case 62:
            case 165:
            case 268:
                new_save_level(SEMI_SIMPLE_GROUP);
                break;
            case 63:
            case 166:
            case 269:
                if (cur_group == SEMI_SIMPLE_GROUP)
                    unsave();
                else
                    off_save();
                break;
            case 3:
            case 106:
            case 209:
                handle_right_brace();
                break;
            case 22:
            case 126:
            case 229:
                {
                    t = cur_chr;
                    scan_dimen(false, false, false);
                    if (t == 0)
                        scan_box(cur_val);
                    else
                        scan_box(-(integer) cur_val);
                }
                break;
            case 32:
            case 135:
            case 238:
                scan_box(1073807261L + cur_chr);
                break;
            case 21:
            case 124:
            case 227:
                begin_box(0);
                break;
            case 44:
                new_graf(cur_chr > 0);
                break;
            case 12:
            case 13:
            case 17:
            case 69:
            case 4:
            case 24:
            case 36:
            case 46:
            case 48:
            case 27:
            case 34:
            case 65:
            case 66:
                {
                    back_input();
                    new_graf(true);
                }
                break;
            case 147:
            case 250:
                indent_in_hmode();
                break;
            case 14:
                {
                    normal_paragraph();
                    if (cur_list.mode > 0)
                        build_page();
                }
                break;
            case 117:
                {
                    if (align_state < 0)
                        off_save();
                    end_graf();
                    if (cur_list.mode == VMODE)
                        build_page();
                }
                break;
            case 118:
            case 131:
            case 140:
            case 128:
            case 136:
                head_for_vmode();
                break;
            case 38:
            case 141:
            case 244:
            case 142:
            case 245:
                begin_insert_or_adjust();
                break;
            case 19:
            case 122:
            case 225:
                make_mark();
                break;
            case 43:
            case 146:
            case 249:
                append_penalty();
                break;
            case 26:
            case 129:
            case 232:
                delete_last();
                break;
            case 25:
            case 127:
            case 230:
                unpackage();
                break;
            case 148:
                append_italic_correction();
                break;
            case 251:
                {
                    mem[cur_list.tail].hh.v.RH = new_kern(0);
                    cur_list.tail = mem[cur_list.tail].hh.v.RH;
                }
                break;
            case 151:
            case 254:
                append_discretionary();
                break;
            case 149:
                make_accent();
                break;
            case 6:
            case 109:
            case 212:
            case 5:
            case 108:
            case 211:
                align_error();
                break;
            case 35:
            case 138:
            case 241:
                no_align_error();
                break;
            case 64:
            case 167:
            case 270:
                omit_error();
                break;
            case 33:
                init_align();
                break;
            case 137:
                if (cur_chr > 0) {
                    if (eTeX_enabled((eqtb[ETEX_STATE_BASE].cint > 0), cur_cmd, cur_chr)) {
                        mem[cur_list.tail].hh.v.RH = new_math(0, cur_chr);
                        cur_list.tail = mem[cur_list.tail].hh.v.RH;
                    }
                } else          /*:1490 */
                    init_align();
                break;
            case 239:
                if (privileged()) {

                    if (cur_group == MATH_SHIFT_GROUP)
                        init_align();
                    else
                        off_save();
                }
                break;
            case 10:
            case 113:
                do_endv();
                break;
            case 68:
            case 171:
            case 274:
                cs_error();
                break;
            case 107:
                init_math();
                break;
            case 255:
                if (privileged()) {

                    if (cur_group == MATH_SHIFT_GROUP)
                        start_eq_no();
                    else
                        off_save();
                }
                break;
            case 208:
                {
                    {
                        mem[cur_list.tail].hh.v.RH = new_noad();
                        cur_list.tail = mem[cur_list.tail].hh.v.RH;
                    }
                    back_input();
                    scan_math(cur_list.tail + 1);
                }
                break;
            case 218:
            case 219:
            case 275:
                set_math_char(MATH_CODE(cur_chr));
                break;
            case 223:
                {
                    scan_char_num();
                    cur_chr = cur_val;
                    set_math_char(MATH_CODE(cur_chr));
                }
                break;
            case 224:
                if (cur_chr == 2) {
                    scan_math_class_int();
                    t = set_class(cur_val);
                    scan_math_fam_int();
                    t = t + set_family(cur_val);
                    scan_usv_num();
                    t = t + cur_val;
                    set_math_char(t);
                } else if (cur_chr == 1) {
                    scan_xetex_math_char_int();
                    set_math_char(cur_val);
                } else {

                    scan_fifteen_bit_int();
                    set_math_char(set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) +
                                  (cur_val % 256));
                }
                break;
            case 276:
                {
                    set_math_char(set_class(cur_chr / 4096) + set_family((cur_chr % 4096) / 256) +
                                  (cur_chr % 256));
                }
                break;
            case 277:
                set_math_char(cur_chr);
                break;
            case 222:
                {
                    if (cur_chr == 1) {
                        scan_math_class_int();
                        t = set_class(cur_val);
                        scan_math_fam_int();
                        t = t + set_family(cur_val);
                        scan_usv_num();
                        t = t + cur_val;
                        set_math_char(t);
                    } else {

                        scan_delimiter_int();
                        cur_val = cur_val / 4096;
                        set_math_char(set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) +
                                      (cur_val % 256));
                    }
                }
                break;
            case 257:
                {
                    {
                        mem[cur_list.tail].hh.v.RH = new_noad();
                        cur_list.tail = mem[cur_list.tail].hh.v.RH;
                    }
                    mem[cur_list.tail].hh.u.B0 = cur_chr;
                    scan_math(cur_list.tail + 1);
                }
                break;
            case 258:
                math_limit_switch();
                break;
            case 273:
                math_radical();
                break;
            case 252:
            case 253:
                math_ac();
                break;
            case 263:
                {
                    scan_spec(VCENTER_GROUP, false);
                    normal_paragraph();
                    push_nest();
                    cur_list.mode = -1;
                    cur_list.aux.cint = -65536000L;
                    if ((insert_src_special_every_vbox))
                        insert_src_special();
                    if (LOCAL(every_vbox) != MIN_HALFWORD)
                        begin_token_list(LOCAL(every_vbox), EVERY_VBOX_TEXT);
                }
                break;
            case 260:
                {
                    mem[cur_list.tail].hh.v.RH = new_style(cur_chr);
                    cur_list.tail = mem[cur_list.tail].hh.v.RH;
                }
                break;
            case 262:
                {
                    {
                        mem[cur_list.tail].hh.v.RH = new_glue(0);
                        cur_list.tail = mem[cur_list.tail].hh.v.RH;
                    }
                    mem[cur_list.tail].hh.u.B1 = COND_MATH_GLUE;
                }
                break;
            case 261:
                append_choices();
                break;
            case 215:
            case 214:
                sub_sup();
                break;
            case 259:
                math_fraction();
                break;
            case 256:
                math_left_right();
                break;
            case 210:
                if (cur_group == MATH_SHIFT_GROUP)
                    after_math();
                else
                    off_save();
                break;
            case 73:
            case 176:
            case 279:
            case 74:
            case 177:
            case 280:
            case 75:
            case 178:
            case 281:
            case 76:
            case 179:
            case 282:
            case 77:
            case 180:
            case 283:
            case 78:
            case 181:
            case 284:
            case 79:
            case 182:
            case 285:
            case 80:
            case 183:
            case 286:
            case 81:
            case 184:
            case 287:
            case 82:
            case 185:
            case 288:
            case 83:
            case 186:
            case 289:
            case 84:
            case 187:
            case 290:
            case 85:
            case 188:
            case 291:
            case 86:
            case 189:
            case 292:
            case 87:
            case 190:
            case 293:
            case 88:
            case 191:
            case 294:
            case 89:
            case 192:
            case 295:
            case 90:
            case 193:
            case 296:
            case 91:
            case 194:
            case 297:
            case 92:
            case 195:
            case 298:
            case 93:
            case 196:
            case 299:
            case 94:
            case 197:
            case 300:
            case 95:
            case 198:
            case 301:
            case 96:
            case 199:
            case 302:
            case 97:
            case 200:
            case 303:
            case 98:
            case 201:
            case 304:
            case 99:
            case 202:
            case 305:
            case 100:
            case 203:
            case 306:
            case 101:
            case 204:
            case 307:
            case 102:
            case 205:
            case 308:
            case 103:
            case 206:
            case 309:
                prefixed_command();
                break;
            case 41:
            case 144:
            case 247:
                {
                    get_token();
                    after_token = cur_tok;
                }
                break;
            case 42:
            case 145:
            case 248:
                {
                    get_token();
                    save_for_after(cur_tok);
                }
                break;
            case 61:
            case 164:
            case 267:
                open_or_close_in();
                break;
            case 59:
            case 162:
            case 265:
                issue_message();
                break;
            case 58:
            case 161:
            case 264:
                shift_case();
                break;
            case 20:
            case 123:
            case 226:
                show_whatever();
                break;
            case 60:
            case 163:
            case 266:
                do_extension();
                break;
            }
        }
        break;
    }
    goto lab60;
 lab70:                        /*main_loop *//*1069: */ if (((cur_list.head == cur_list.tail) && (cur_list.mode > 0))) {
        if ((insert_src_special_auto))
            append_src_special();
    }
    prev_class = ((CHAR_CLASS_LIMIT - 1));
    if (((font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == AAT_FONT_FLAG)
         || (font_area[eqtb[CUR_FONT_LOC].hh.v.RH] == OTGR_FONT_FLAG))) {
        if (cur_list.mode > 0) {

            if (INTPAR(language) != cur_list.aux.hh.v.RH)
                fix_language();
        }
        main_h = 0;
        main_f = eqtb[CUR_FONT_LOC].hh.v.RH;
        native_len = 0;
 lab71:/*collect_native */ main_s = SF_CODE(cur_chr) % 65536L;
        if (main_s == 1000)
            cur_list.aux.hh.v.LH = 1000;
        else if (main_s < 1000) {
            if (main_s > 0)
                cur_list.aux.hh.v.LH = main_s;
        } else if (cur_list.aux.hh.v.LH < 1000)
            cur_list.aux.hh.v.LH = 1000;
        else
            cur_list.aux.hh.v.LH = main_s;
        cur_ptr = MIN_HALFWORD;
        space_class = SF_CODE(cur_chr) / 65536L;
        if ((STATEINT(xetex_inter_char_tokens) > 0) && space_class != CHAR_CLASS_LIMIT) {
            if (prev_class == ((CHAR_CLASS_LIMIT - 1))) {
                if ((cur_input.state != TOKEN_LIST) || (cur_input.index != BACKED_UP_CHAR)) {
                    find_sa_element(INTER_CHAR_VAL,
                                    ((CHAR_CLASS_LIMIT - 1)) * CHAR_CLASS_LIMIT + space_class,
                                    false);
                    if (cur_ptr != MIN_HALFWORD) {
                        if (cur_cmd != LETTER)
                            cur_cmd = OTHER_CHAR;
                        cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                        back_input();
                        cur_input.index = BACKED_UP_CHAR;
                        begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
                        goto lab60;
                    }
                }
            } else {

                find_sa_element(INTER_CHAR_VAL, prev_class * CHAR_CLASS_LIMIT + space_class, false);
                if (cur_ptr != MIN_HALFWORD) {
                    if (cur_cmd != LETTER)
                        cur_cmd = OTHER_CHAR;
                    cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                    back_input();
                    cur_input.index = BACKED_UP_CHAR;
                    begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
                    prev_class = ((CHAR_CLASS_LIMIT - 1));
                    goto lab72;
                }
            }
            prev_class = space_class;
        }
        if ((cur_chr > 65535L)) {
            while (native_text_size <= native_len + 2) {

                native_text_size = native_text_size + 128;
                native_text = xrealloc(native_text, native_text_size * sizeof(UTF16_code));
            }
            {
                native_text[native_len] = (cur_chr - 65536L) / 1024 + 55296L;
                native_len++;
            }
            {
                native_text[native_len] = (cur_chr - 65536L) % 1024 + 56320L;
                native_len++;
            }
        } else {

            while (native_text_size <= native_len + 1) {

                native_text_size = native_text_size + 128;
                native_text = xrealloc(native_text, native_text_size * sizeof(UTF16_code));
            }
            {
                native_text[native_len] = cur_chr;
                native_len++;
            }
        }
        is_hyph = (cur_chr == hyphen_char[main_f]) || ((STATEINT(xetex_dash_break) > 0)
                                                       && ((cur_chr == 8212) || (cur_chr == 8211)));
        if ((main_h == 0) && is_hyph)
            main_h = native_len;
        get_next();
        if ((cur_cmd == LETTER) || (cur_cmd == OTHER_CHAR) || (cur_cmd == CHAR_GIVEN))
            goto lab71;
        x_token();
        if ((cur_cmd == LETTER) || (cur_cmd == OTHER_CHAR) || (cur_cmd == CHAR_GIVEN))
            goto lab71;
        if (cur_cmd == CHAR_NUM) {
            scan_usv_num();
            cur_chr = cur_val;
            goto lab71;
        }
        if ((STATEINT(xetex_inter_char_tokens) > 0) && (space_class != CHAR_CLASS_LIMIT)
            && (prev_class != ((CHAR_CLASS_LIMIT - 1)))) {
            prev_class = ((CHAR_CLASS_LIMIT - 1));
            find_sa_element(INTER_CHAR_VAL,
                            space_class * CHAR_CLASS_LIMIT + ((CHAR_CLASS_LIMIT - 1)), false);
            if (cur_ptr != MIN_HALFWORD) {
                if (cur_cs == 0) {
                    if (cur_cmd == CHAR_NUM)
                        cur_cmd = OTHER_CHAR;
                    cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                } else
                    cur_tok = CS_TOKEN_FLAG + cur_cs;
                back_input();
                begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
                goto lab72;
            }
        }
 lab72:                        /*collected */ if ((font_mapping[main_f] != 0)) {
            main_k = apply_mapping(font_mapping[main_f], native_text, native_len);
            native_len = 0;
            while (native_text_size <= native_len + main_k) {

                native_text_size = native_text_size + 128;
                native_text = xrealloc(native_text, native_text_size * sizeof(UTF16_code));
            }
            main_h = 0;
            {
                register integer for_end;
                main_p = 0;
                for_end = main_k - 1;
                if (main_p <= for_end)
                    do {
                        {
                            native_text[native_len] = mapped_text[main_p];
                            native_len++;
                        }
                        if ((main_h == 0)
                            && ((mapped_text[main_p] == hyphen_char[main_f])
                                || ((STATEINT(xetex_dash_break) > 0)
                                    && ((mapped_text[main_p] == 8212) || (mapped_text[main_p] == 8211)))))
                            main_h = native_len;
                    }
                    while (main_p++ < for_end);
            }
        }
        if (INTPAR(tracing_lost_chars) > 0) {
            temp_ptr = 0;
            while ((temp_ptr < native_len)) {

                main_k = native_text[temp_ptr];
                temp_ptr++;
                if ((main_k >= 55296L) && (main_k < 56320L)) {
                    main_k = 65536L + (main_k - 55296L) * 1024;
                    main_k = main_k + native_text[temp_ptr] - 56320L;
                    temp_ptr++;
                }
                if (map_char_to_glyph(main_f, main_k) == 0)
                    char_warning(main_f, main_k);
            }
        }
        main_k = native_len;
        main_pp = cur_list.tail;
        if (cur_list.mode == HMODE) {
            main_ppp = cur_list.head;
            if (main_ppp != main_pp)
                while ((mem[main_ppp].hh.v.RH != main_pp)) {

                    if ((!(main_ppp >= hi_mem_min)) && (mem[main_ppp == DISC_NODE].hh.u.B0)) {
                        temp_ptr = main_ppp;
                        {
                            register integer for_end;
                            main_p = 1;
                            for_end = mem[temp_ptr].hh.u.B1;
                            if (main_p <= for_end)
                                do
                                    main_ppp = mem[main_ppp].hh.v.RH;
                                while (main_p++ < for_end);
                        }
                    }
                    if (main_ppp != main_pp)
                        main_ppp = mem[main_ppp].hh.v.RH;
                }
            temp_ptr = 0;
            do {
                if (main_h == 0)
                    main_h = main_k;
                if ((((main_pp) != MIN_HALFWORD && (!(main_pp >= hi_mem_min))
                      && (mem[main_pp].hh.u.B0 == WHATSIT_NODE)
                      && ((mem[main_pp].hh.u.B1 == NATIVE_WORD_NODE)
                          || (mem[main_pp].hh.u.B1 == NATIVE_WORD_NODE_AT))))
                    && (mem[main_pp + 4].qqqq.u.B1 == main_f) && (main_ppp != main_pp) && (!(main_ppp >= hi_mem_min))
                    && (mem[main_ppp].hh.u.B0 != DISC_NODE)) {
                    main_k = main_h + mem[main_pp + 4].qqqq.u.B2;
                    while (native_text_size <= native_len + main_k) {

                        native_text_size = native_text_size + 128;
                        native_text = xrealloc(native_text, native_text_size * sizeof(UTF16_code));
                    }
                    save_native_len = native_len;
                    {
                        register integer for_end;
                        main_p = 0;
                        for_end = mem[main_pp + 4].qqqq.u.B2 - 1;
                        if (main_p <= for_end)
                            do {
                                native_text[native_len] = get_native_char(main_pp, main_p);
                                native_len++;
                            }
                            while (main_p++ < for_end);
                    }
                    {
                        register integer for_end;
                        main_p = 0;
                        for_end = main_h - 1;
                        if (main_p <= for_end)
                            do {
                                native_text[native_len] = native_text[temp_ptr + main_p];
                                native_len++;
                            }
                            while (main_p++ < for_end);
                    }
                    do_locale_linebreaks(save_native_len, main_k);
                    native_len = save_native_len;
                    main_k = native_len - main_h - temp_ptr;
                    temp_ptr = main_h;
                    main_h = 0;
                    while ((main_h < main_k) && (native_text[temp_ptr + main_h] != hyphen_char[main_f])
                           && ((!(STATEINT(xetex_dash_break) > 0))
                               || ((native_text[temp_ptr + main_h] != 8212)
                                   && (native_text[temp_ptr + main_h] != 8211))))
                        main_h++;
                    if ((main_h < main_k))
                        main_h++;
                    mem[main_ppp].hh.v.RH = mem[main_pp].hh.v.RH;
                    mem[main_pp].hh.v.RH = MIN_HALFWORD;
                    flush_node_list(main_pp);
                    main_pp = cur_list.tail;
                    while ((mem[main_ppp].hh.v.RH != main_pp))
                        main_ppp = mem[main_ppp].hh.v.RH;
                } else {

                    do_locale_linebreaks(temp_ptr, main_h);
                    temp_ptr = temp_ptr + main_h;
                    main_k = main_k - main_h;
                    main_h = 0;
                    while ((main_h < main_k) && (native_text[temp_ptr + main_h] != hyphen_char[main_f])
                           && ((!(STATEINT(xetex_dash_break) > 0))
                               || ((native_text[temp_ptr + main_h] != 8212)
                                   && (native_text[temp_ptr + main_h] != 8211))))
                        main_h++;
                    if ((main_h < main_k))
                        main_h++;
                }
                if ((main_k > 0) || is_hyph) {
                    {
                        mem[cur_list.tail].hh.v.RH = new_disc();
                        cur_list.tail = mem[cur_list.tail].hh.v.RH;
                    }
                    main_pp = cur_list.tail;
                }
            } while (!(main_k == 0));
        } else {

            main_ppp = cur_list.head;
            if (main_ppp != main_pp)
                while ((mem[main_ppp].hh.v.RH != main_pp)) {

                    if ((!(main_ppp >= hi_mem_min)) && (mem[main_ppp == DISC_NODE].hh.u.B0)) {
                        temp_ptr = main_ppp;
                        {
                            register integer for_end;
                            main_p = 1;
                            for_end = mem[temp_ptr].hh.u.B1;
                            if (main_p <= for_end)
                                do
                                    main_ppp = mem[main_ppp].hh.v.RH;
                                while (main_p++ < for_end);
                        }
                    }
                    if (main_ppp != main_pp)
                        main_ppp = mem[main_ppp].hh.v.RH;
                }
            if ((((main_pp) != MIN_HALFWORD && (!(main_pp >= hi_mem_min)) && (mem[main_pp].hh.u.B0 == WHATSIT_NODE)
                  && ((mem[main_pp].hh.u.B1 == NATIVE_WORD_NODE)
                      || (mem[main_pp].hh.u.B1 == NATIVE_WORD_NODE_AT)))) && (mem[main_pp + 4].qqqq.u.B1 == main_f)
                && (main_ppp != main_pp) && (!(main_ppp >= hi_mem_min)) && (mem[main_ppp].hh.u.B0 != DISC_NODE)) {
                mem[main_pp].hh.v.RH = new_native_word_node(main_f, main_k + mem[main_pp + 4].qqqq.u.B2);
                cur_list.tail = mem[main_pp].hh.v.RH;
                {
                    register integer for_end;
                    main_p = 0;
                    for_end = mem[main_pp + 4].qqqq.u.B2 - 1;
                    if (main_p <= for_end)
                        do
                            set_native_char(cur_list.tail, main_p, get_native_char(main_pp, main_p));
                        while (main_p++ < for_end);
                }
                {
                    register integer for_end;
                    main_p = 0;
                    for_end = main_k - 1;
                    if (main_p <= for_end)
                        do
                            set_native_char(cur_list.tail, main_p + mem[main_pp + 4].qqqq.u.B2,
                                            native_text[main_p]);
                        while (main_p++ < for_end);
                }
                set_native_metrics(cur_list.tail, (STATEINT(xetex_use_glyph_metrics) > 0));
                main_p = cur_list.head;
                if (main_p != main_pp)
                    while (mem[main_p].hh.v.RH != main_pp)
                        main_p = mem[main_p].hh.v.RH;
                mem[main_p].hh.v.RH = mem[main_pp].hh.v.RH;
                mem[main_pp].hh.v.RH = MIN_HALFWORD;
                flush_node_list(main_pp);
            } else {

                mem[main_pp].hh.v.RH = new_native_word_node(main_f, main_k);
                cur_list.tail = mem[main_pp].hh.v.RH;
                {
                    register integer for_end;
                    main_p = 0;
                    for_end = main_k - 1;
                    if (main_p <= for_end)
                        do
                            set_native_char(cur_list.tail, main_p, native_text[main_p]);
                        while (main_p++ < for_end);
                }
                set_native_metrics(cur_list.tail, (STATEINT(xetex_use_glyph_metrics) > 0));
            }
        }
        if (STATEINT(xetex_interword_space_shaping) > 0) {
            main_p = cur_list.head;
            main_pp = MIN_HALFWORD;
            while (main_p != cur_list.tail) {

                if ((((main_p) != MIN_HALFWORD && (!(main_p >= hi_mem_min))
                      && (mem[main_p].hh.u.B0 == WHATSIT_NODE)
                      && ((mem[main_p].hh.u.B1 == NATIVE_WORD_NODE)
                          || (mem[main_p].hh.u.B1 == NATIVE_WORD_NODE_AT)))))
                    main_pp = main_p;
                main_p = mem[main_p].hh.v.RH;
            }
            if ((main_pp != MIN_HALFWORD)) {
                if ((mem[main_pp + 4].qqqq.u.B1 == main_f)) {
                    main_p = mem[main_pp].hh.v.RH;
                    while (!(main_p >= hi_mem_min)
                           && ((mem[main_p].hh.u.B0 == PENALTY_NODE) || (mem[main_p].hh.u.B0 == INS_NODE)
                               || (mem[main_p].hh.u.B0 == MARK_NODE) || (mem[main_p].hh.u.B0 == ADJUST_NODE)
                               || ((mem[main_p].hh.u.B0 == WHATSIT_NODE) && (mem[main_p].hh.u.B1 <= 4))))
                        main_p = mem[main_p].hh.v.RH;
                    if (!(main_p >= hi_mem_min) && (mem[main_p].hh.u.B0 == GLUE_NODE)) {
                        main_ppp = mem[main_p].hh.v.RH;
                        while (!(main_ppp >= hi_mem_min)
                               && ((mem[main_ppp].hh.u.B0 == PENALTY_NODE)
                                   || (mem[main_ppp].hh.u.B0 == INS_NODE)
                                   || (mem[main_ppp].hh.u.B0 == MARK_NODE)
                                   || (mem[main_ppp].hh.u.B0 == ADJUST_NODE)
                                   || ((mem[main_ppp].hh.u.B0 == WHATSIT_NODE) && (mem[main_ppp].hh.u.B1 <= 4))))
                            main_ppp = mem[main_ppp].hh.v.RH;
                        if (main_ppp == cur_list.tail) {
                            temp_ptr =
                                new_native_word_node(main_f,
                                                     mem[main_pp + 4].qqqq.u.B2 + 1 + mem[cur_list.tail +
                                                                                        4].qqqq.u.B2);
                            main_k = 0;
                            {
                                register integer for_end;
                                t = 0;
                                for_end = mem[main_pp + 4].qqqq.u.B2 - 1;
                                if (t <= for_end)
                                    do {
                                        set_native_char(temp_ptr, main_k, get_native_char(main_pp, t));
                                        main_k++;
                                    }
                                    while (t++ < for_end);
                            }
                            set_native_char(temp_ptr, main_k, 32 /*" " */ );
                            main_k++;
                            {
                                register integer for_end;
                                t = 0;
                                for_end = mem[cur_list.tail + 4].qqqq.u.B2 - 1;
                                if (t <= for_end)
                                    do {
                                        set_native_char(temp_ptr, main_k, get_native_char(cur_list.tail, t));
                                        main_k++;
                                    }
                                    while (t++ < for_end);
                            }
                            set_native_metrics(temp_ptr, (STATEINT(xetex_use_glyph_metrics) > 0));
                            t = mem[temp_ptr + 1].cint - mem[main_pp + 1].cint - mem[cur_list.tail + 1].cint;
                            free_node(temp_ptr, mem[temp_ptr + 4].qqqq.u.B0);
                            if (t != mem[font_glue[main_f] + 1].cint) {
                                temp_ptr = new_kern(t - mem[font_glue[main_f] + 1].cint);
                                mem[temp_ptr].hh.u.B1 = SPACE_ADJUSTMENT;
                                mem[temp_ptr].hh.v.RH = mem[main_p].hh.v.RH;
                                mem[main_p].hh.v.RH = temp_ptr;
                            }
                        }
                    }
                }
            }
        }
        if (cur_ptr != MIN_HALFWORD)
            goto lab60;
        else
            goto lab21;
    }
    main_s = SF_CODE(cur_chr) % 65536L;
    if (main_s == 1000)
        cur_list.aux.hh.v.LH = 1000;
    else if (main_s < 1000) {
        if (main_s > 0)
            cur_list.aux.hh.v.LH = main_s;
    } else if (cur_list.aux.hh.v.LH < 1000)
        cur_list.aux.hh.v.LH = 1000;
    else
        cur_list.aux.hh.v.LH = main_s;
    cur_ptr = MIN_HALFWORD;
    space_class = SF_CODE(cur_chr) / 65536L;
    if ((STATEINT(xetex_inter_char_tokens) > 0) && space_class != CHAR_CLASS_LIMIT) {
        if (prev_class == ((CHAR_CLASS_LIMIT - 1))) {
            if ((cur_input.state != TOKEN_LIST) || (cur_input.index != BACKED_UP_CHAR)) {
                find_sa_element(INTER_CHAR_VAL,
                                ((CHAR_CLASS_LIMIT - 1)) * CHAR_CLASS_LIMIT + space_class, false);
                if (cur_ptr != MIN_HALFWORD) {
                    if (cur_cmd != LETTER)
                        cur_cmd = OTHER_CHAR;
                    cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                    back_input();
                    cur_input.index = BACKED_UP_CHAR;
                    begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
                    goto lab60;
                }
            }
        } else {

            find_sa_element(INTER_CHAR_VAL, prev_class * CHAR_CLASS_LIMIT + space_class, false);
            if (cur_ptr != MIN_HALFWORD) {
                if (cur_cmd != LETTER)
                    cur_cmd = OTHER_CHAR;
                cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                back_input();
                cur_input.index = BACKED_UP_CHAR;
                begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
                prev_class = ((CHAR_CLASS_LIMIT - 1));
                goto lab60;
            }
        }
        prev_class = space_class;
    }
    main_f = eqtb[CUR_FONT_LOC].hh.v.RH;
    bchar = font_bchar[main_f];
    false_bchar = font_false_bchar[main_f];
    if (cur_list.mode > 0) {

        if (INTPAR(language) != cur_list.aux.hh.v.RH)
            fix_language();
    }
    {
        lig_stack = avail;
        if (lig_stack == MIN_HALFWORD)
            lig_stack = get_avail();
        else {

            avail = mem[lig_stack].hh.v.RH;
            mem[lig_stack].hh.v.RH = MIN_HALFWORD;
        }
    }
    mem[lig_stack].hh.u.B0 = main_f;
    cur_l = cur_chr;
    mem[lig_stack].hh.u.B1 = cur_l;
    cur_q = cur_list.tail;
    if (cancel_boundary) {
        cancel_boundary = false;
        main_k = NON_ADDRESS;
    } else
        main_k = bchar_label[main_f];
    if (main_k == NON_ADDRESS)
        goto lab92;
    cur_r = cur_l;
    cur_l = TOO_BIG_CHAR;
    goto lab111;
 lab80:/*main_loop_wrapup *//*1070: */ if (cur_l < TOO_BIG_CHAR) {
        if (mem[cur_q].hh.v.RH > MIN_HALFWORD) {

            if (mem[cur_list.tail].hh.u.B1 == hyphen_char[main_f])
                ins_disc = true;
        }
        if (ligature_present) {
            main_p = new_ligature(main_f, cur_l, mem[cur_q].hh.v.RH);
            if (lft_hit) {
                mem[main_p].hh.u.B1 = 2;
                lft_hit = false;
            }
            if (rt_hit) {

                if (lig_stack == MIN_HALFWORD) {
                    mem[main_p].hh.u.B1++;
                    rt_hit = false;
                }
            }
            mem[cur_q].hh.v.RH = main_p;
            cur_list.tail = main_p;
            ligature_present = false;
        }
        if (ins_disc) {
            ins_disc = false;
            if (cur_list.mode > 0) {
                mem[cur_list.tail].hh.v.RH = new_disc();
                cur_list.tail = mem[cur_list.tail].hh.v.RH;
            }
        }
    }
 lab90:                        /*main_loop_move *//*1071: */ if (lig_stack == MIN_HALFWORD)
        goto lab21;
    cur_q = cur_list.tail;
    cur_l = mem[lig_stack].hh.u.B1;
 lab91:                        /*main_loop_move 1 */ if (!(lig_stack >= hi_mem_min))
        goto lab95;
 lab92:                        /*main_loop_move 2 */ if ((effective_char(false, main_f, cur_chr) > font_ec[main_f])
                              || (effective_char(false, main_f, cur_chr) < font_bc[main_f])) {
        char_warning(main_f, cur_chr);
        {
            mem[lig_stack].hh.v.RH = avail;
            avail = lig_stack;
        }
        goto lab60;
    }
    main_i = effective_char_info(main_f, cur_l);
    if (!(main_i.u.B0 > 0)) {
        char_warning(main_f, cur_chr);
        {
            mem[lig_stack].hh.v.RH = avail;
            avail = lig_stack;
        }
        goto lab60;
    }
    mem[cur_list.tail].hh.v.RH = lig_stack;
    cur_list.tail = /*:1071 */ lig_stack;
 lab100:                       /*main_loop_lookahead *//*1073: */ get_next();
    if (cur_cmd == LETTER)
        goto lab101;
    if (cur_cmd == OTHER_CHAR)
        goto lab101;
    if (cur_cmd == CHAR_GIVEN)
        goto lab101;
    x_token();
    if (cur_cmd == LETTER)
        goto lab101;
    if (cur_cmd == OTHER_CHAR)
        goto lab101;
    if (cur_cmd == CHAR_GIVEN)
        goto lab101;
    if (cur_cmd == CHAR_NUM) {
        scan_char_num();
        cur_chr = cur_val;
        goto lab101;
    }
    if (cur_cmd == NO_BOUNDARY)
        bchar = TOO_BIG_CHAR;
    cur_r = bchar;
    lig_stack = MIN_HALFWORD;
    goto lab110;
 lab101:/*main_loop_lookahead 1 */ main_s = SF_CODE(cur_chr) % 65536L;
    if (main_s == 1000)
        cur_list.aux.hh.v.LH = 1000;
    else if (main_s < 1000) {
        if (main_s > 0)
            cur_list.aux.hh.v.LH = main_s;
    } else if (cur_list.aux.hh.v.LH < 1000)
        cur_list.aux.hh.v.LH = 1000;
    else
        cur_list.aux.hh.v.LH = main_s;
    cur_ptr = MIN_HALFWORD;
    space_class = SF_CODE(cur_chr) / 65536L;
    if ((STATEINT(xetex_inter_char_tokens) > 0) && space_class != CHAR_CLASS_LIMIT) {
        if (prev_class == ((CHAR_CLASS_LIMIT - 1))) {
            if ((cur_input.state != TOKEN_LIST) || (cur_input.index != BACKED_UP_CHAR)) {
                find_sa_element(INTER_CHAR_VAL,
                                ((CHAR_CLASS_LIMIT - 1)) * CHAR_CLASS_LIMIT + space_class, false);
                if (cur_ptr != MIN_HALFWORD) {
                    if (cur_cmd != LETTER)
                        cur_cmd = OTHER_CHAR;
                    cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                    back_input();
                    cur_input.index = BACKED_UP_CHAR;
                    begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
                    goto lab60;
                }
            }
        } else {

            find_sa_element(INTER_CHAR_VAL, prev_class * CHAR_CLASS_LIMIT + space_class, false);
            if (cur_ptr != MIN_HALFWORD) {
                if (cur_cmd != LETTER)
                    cur_cmd = OTHER_CHAR;
                cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
                back_input();
                cur_input.index = BACKED_UP_CHAR;
                begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
                prev_class = ((CHAR_CLASS_LIMIT - 1));
                goto lab60;
            }
        }
        prev_class = space_class;
    }
    {
        lig_stack = avail;
        if (lig_stack == MIN_HALFWORD)
            lig_stack = get_avail();
        else {

            avail = mem[lig_stack].hh.v.RH;
            mem[lig_stack].hh.v.RH = MIN_HALFWORD;
        }
    }
    mem[lig_stack].hh.u.B0 = main_f;
    cur_r = cur_chr;
    mem[lig_stack].hh.u.B1 = cur_r;
    if (cur_r == false_bchar)
        cur_r = 65536L /*too_big_char *//*:1073 */ ;
 lab110:/*main_lig_loop *//*1074: */ if (((main_i.u.B2) % 4) != LIG_TAG)
        goto lab80;
    if (cur_r == TOO_BIG_CHAR)
        goto lab80;
    main_k = lig_kern_base[main_f] + main_i.u.B3;
    main_j = font_info[main_k].qqqq;
    if (main_j.u.B0 <= 128)
        goto lab112;
    main_k = lig_kern_base[main_f] + 256 * main_j.u.B2 + main_j.u.B3 + 32768L - 256 * (128);
 lab111:                       /*main_lig_loop 1 */ main_j = font_info[main_k].qqqq;
 lab112:                       /*main_lig_loop 2 */ if (main_j.u.B1 == cur_r) {

        if (main_j.u.B0 <= 128) { /*1075: */
            if (main_j.u.B2 >= 128) {
                if (cur_l < TOO_BIG_CHAR) {
                    if (mem[cur_q].hh.v.RH > MIN_HALFWORD) {

                        if (mem[cur_list.tail].hh.u.B1 == hyphen_char[main_f])
                            ins_disc = true;
                    }
                    if (ligature_present) {
                        main_p = new_ligature(main_f, cur_l, mem[cur_q].hh.v.RH);
                        if (lft_hit) {
                            mem[main_p].hh.u.B1 = 2;
                            lft_hit = false;
                        }
                        if (rt_hit) {

                            if (lig_stack == MIN_HALFWORD) {
                                mem[main_p].hh.u.B1++;
                                rt_hit = false;
                            }
                        }
                        mem[cur_q].hh.v.RH = main_p;
                        cur_list.tail = main_p;
                        ligature_present = false;
                    }
                    if (ins_disc) {
                        ins_disc = false;
                        if (cur_list.mode > 0) {
                            mem[cur_list.tail].hh.v.RH = new_disc();
                            cur_list.tail = mem[cur_list.tail].hh.v.RH;
                        }
                    }
                }
                {
                    mem[cur_list.tail].hh.v.RH =
                        new_kern(font_info[kern_base[main_f] + 256 * main_j.u.B2 + main_j.u.B3].cint);
                    cur_list.tail = mem[cur_list.tail].hh.v.RH;
                }
                goto lab90;
            }
            if (cur_l == TOO_BIG_CHAR)
                lft_hit = true;
            else if (lig_stack == MIN_HALFWORD)
                rt_hit = true;
            switch (main_j.u.B2) {
            case 1:
            case 5:
                {
                    cur_l = main_j.u.B3;
                    main_i = font_info[char_base[main_f] + effective_char(true, main_f, cur_l)].qqqq;
                    ligature_present = true;
                }
                break;
            case 2:
            case 6:
                {
                    cur_r = main_j.u.B3;
                    if (lig_stack == MIN_HALFWORD) {
                        lig_stack = new_lig_item(cur_r);
                        bchar = TOO_BIG_CHAR;
                    } else if ((lig_stack >= hi_mem_min)) {
                        main_p = lig_stack;
                        lig_stack = new_lig_item(cur_r);
                        mem[lig_stack + 1].hh.v.RH = main_p;
                    } else
                        mem[lig_stack].hh.u.B1 = cur_r;
                }
                break;
            case 3:
                {
                    cur_r = main_j.u.B3;
                    main_p = lig_stack;
                    lig_stack = new_lig_item(cur_r);
                    mem[lig_stack].hh.v.RH = main_p;
                }
                break;
            case 7:
            case 11:
                {
                    if (cur_l < TOO_BIG_CHAR) {
                        if (mem[cur_q].hh.v.RH > MIN_HALFWORD) {

                            if (mem[cur_list.tail].hh.u.B1 == hyphen_char[main_f])
                                ins_disc = true;
                        }
                        if (ligature_present) {
                            main_p = new_ligature(main_f, cur_l, mem[cur_q].hh.v.RH);
                            if (lft_hit) {
                                mem[main_p].hh.u.B1 = 2;
                                lft_hit = false;
                            }
                            if (false) {

                                if (lig_stack == MIN_HALFWORD) {
                                    mem[main_p].hh.u.B1++;
                                    rt_hit = false;
                                }
                            }
                            mem[cur_q].hh.v.RH = main_p;
                            cur_list.tail = main_p;
                            ligature_present = false;
                        }
                        if (ins_disc) {
                            ins_disc = false;
                            if (cur_list.mode > 0) {
                                mem[cur_list.tail].hh.v.RH = new_disc();
                                cur_list.tail = mem[cur_list.tail].hh.v.RH;
                            }
                        }
                    }
                    cur_q = cur_list.tail;
                    cur_l = main_j.u.B3;
                    main_i = font_info[char_base[main_f] + effective_char(true, main_f, cur_l)].qqqq;
                    ligature_present = true;
                }
                break;
            default:
                {
                    cur_l = main_j.u.B3;
                    ligature_present = true;
                    if (lig_stack == MIN_HALFWORD)
                        goto lab80;
                    else
                        goto lab91;
                }
                break;
            }
            if (main_j.u.B2 > 4) {

                if (main_j.u.B2 != 7)
                    goto lab80;
            }
            if (cur_l < TOO_BIG_CHAR)
                goto lab110;
            main_k = bchar_label[main_f];
            goto lab111;
        }
    }
    if (main_j.u.B0 == 0)
        main_k++;
    else {

        if (main_j.u.B0 >= 128)
            goto lab80;
        main_k = main_k + main_j.u.B0 + 1;
    }
    goto lab111;
 lab95:                        /*main_loop_move_lig *//*1072: */ main_p = mem[lig_stack + 1].hh.v.RH;
    if (main_p > MIN_HALFWORD) {
        mem[cur_list.tail].hh.v.RH = main_p;
        cur_list.tail = mem[cur_list.tail].hh.v.RH;
    }
    temp_ptr = lig_stack;
    lig_stack = mem[temp_ptr].hh.v.RH;
    free_node(temp_ptr, SMALL_NODE_SIZE);
    main_i = font_info[char_base[main_f] + effective_char(true, main_f, cur_l)].qqqq;
    ligature_present = true;
    if (lig_stack == MIN_HALFWORD) {

        if (main_p > MIN_HALFWORD)
            goto lab100;
        else
            cur_r = bchar;
    } else
        cur_r = mem[lig_stack].hh.u.B1;
    goto lab110;
 lab120:/*append_normal_space */ if ((STATEINT(xetex_inter_char_tokens) > 0)
                                 && (space_class != CHAR_CLASS_LIMIT)
                                 && (prev_class != ((CHAR_CLASS_LIMIT - 1)))) {
        prev_class = ((CHAR_CLASS_LIMIT - 1));
        find_sa_element(INTER_CHAR_VAL,
                        space_class * CHAR_CLASS_LIMIT + ((CHAR_CLASS_LIMIT - 1)), false);
        if (cur_ptr != MIN_HALFWORD) {
            if (cur_cs == 0) {
                if (cur_cmd == CHAR_NUM)
                    cur_cmd = OTHER_CHAR;
                cur_tok = (cur_cmd * MAX_CHAR_VAL) + cur_chr;
            } else
                cur_tok = CS_TOKEN_FLAG + cur_cs;
            back_input();
            begin_token_list(mem[cur_ptr + 1].hh.v.RH, INTER_CHAR_TEXT);
            goto lab60;
        }
    }
    if (GLUEPAR(space_skip) == 0) {
        {
            main_p = font_glue[eqtb[CUR_FONT_LOC].hh.v.RH];
            if (main_p == MIN_HALFWORD) {
                main_p = new_spec(0);
                main_k = param_base[eqtb[CUR_FONT_LOC].hh.v.RH] + 2;
                mem[main_p + 1].cint = font_info[main_k].cint;
                mem[main_p + 2].cint = font_info[main_k + 1].cint;
                mem[main_p + 3].cint = font_info[main_k + 2].cint;
                font_glue[eqtb[CUR_FONT_LOC].hh.v.RH] = main_p;
            }
        }
        temp_ptr = new_glue(main_p);
    } else
        temp_ptr = new_param_glue(GLUE_PAR__space_skip);
    mem[cur_list.tail].hh.v.RH = temp_ptr;
    cur_list.tail = temp_ptr;
    goto lab60;
}

void give_err_help(void)
{
    CACHE_THE_EQTB;
    token_show(LOCAL(err_help));
}

void close_files_and_terminate(void)
{
    CACHE_THE_EQTB;
    integer k;

    terminate_font_manager();
    {
        register integer for_end;
        k = 0;
        for_end = 15;
        if (k <= for_end)
            do
                if (write_open[k])
                    ttstub_output_close (write_file[k]);
            while (k++ < for_end) ;
    }

    while (cur_s > -1) {
        if (cur_s > 0) {
            dvi_buf[dvi_ptr] = POP;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        } else {
	    dvi_buf[dvi_ptr] = EOP;
	    dvi_ptr++;
	    if (dvi_ptr == dvi_limit)
		dvi_swap();
            total_pages++;
        }
        cur_s--;
    }

    if (total_pages == 0)
        print_nl(S(No_pages_of_output_));
    else if (cur_s != -2) {
	dvi_buf[dvi_ptr] = POST;
	dvi_ptr++;
	if (dvi_ptr == dvi_limit)
	    dvi_swap();
        dvi_four(last_bop);
        last_bop = dvi_offset + dvi_ptr - 5;
        dvi_four(25400000L);
        dvi_four(473628672L);
        prepare_mag();
        dvi_four(INTPAR(mag));
        dvi_four(max_v);
        dvi_four(max_h);
        {
            dvi_buf[dvi_ptr] = max_push / 256;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = max_push % 256;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = (total_pages / 256) % 256;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        {
            dvi_buf[dvi_ptr] = total_pages % 256;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        while (font_ptr > FONT_BASE) {

            if (font_used[font_ptr])
                dvi_font_def(font_ptr);
            font_ptr--;
        }
        {
            dvi_buf[dvi_ptr] = POST_POST;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        dvi_four(last_bop);
        {
            dvi_buf[dvi_ptr] = ID_BYTE;
            dvi_ptr++;
            if (dvi_ptr == dvi_limit)
                dvi_swap();
        }
        ;

        k = 4 + ((dvi_buf_size - dvi_ptr) % 4);

        while (k > 0) {
	    dvi_buf[dvi_ptr] = 223;
	    dvi_ptr++;
	    if (dvi_ptr == dvi_limit)
		dvi_swap();
            k--;
        }

        if (dvi_limit == half_buf)
            write_to_dvi(half_buf, dvi_buf_size - 1);

        if (dvi_ptr > (TEX_INFINITY - dvi_offset)) {
            cur_s = -2;
            fatal_error(S(dvi_length_exceeds__7FFFFFFF/**/));
        }

        if (dvi_ptr > 0)
            write_to_dvi(0, dvi_ptr - 1);

        k = ttstub_output_close(dvi_file);

        if (k == 0) {
            print_nl(S(Output_written_on_));
            print(output_file_name);
            print(S(___Z2/*" ("*/));
            print_int(total_pages);
            if (total_pages != 1)
                print(S(_pages));
            else
                print(S(_page));
            print(S(___Z13/*", "*/));
            print_int(dvi_offset + dvi_ptr);
            print(S(_bytes__));
        } else {
            print_nl(S(Error_));
            print_int(k);
            print(S(___Z2/*" ("*/));
            print_c_string(strerror(k));
            print(S(__generating_output_));
            print_nl(S(file_));
            print(output_file_name);
            print(S(_may_not_be_valid_));
        }
    }

    synctex_terminate(log_opened);
    if (log_opened) {
	ttstub_output_putc (log_file, '\n');
        ttstub_output_close (log_file);
        selector = selector - 2;
        if (selector == SELECTOR_TERM_ONLY) {
            print_nl(S(Transcript_written_on_));
            print(texmf_log_name);
            print_char(46 /*"." */ );
        }
    }

    print_ln();
}

void flush_str(str_number s)
{
    if ((s == str_ptr - 1)) {
        str_ptr--;
        pool_ptr = str_start[(str_ptr) - 65536L];
    }
}

str_number tokens_to_string(int32_t p)
{
    register str_number Result;
    memory_word *mem = zmem; if (selector == SELECTOR_NEW_STRING )
        pdf_error(S(tokens), S(tokens_to_string___called_wh/*ile selector = new_string*/));
    old_setting = selector;
    selector = SELECTOR_NEW_STRING ;
    show_token_list(mem[p].hh.v.RH, MIN_HALFWORD, pool_size - pool_ptr);
    selector = old_setting;
    Result = make_string();
    return Result;
}

void scan_pdf_ext_toks(void)
{
    scan_toks(false, true);
}

void compare_strings(void)
{
    str_number s1, s2;
    pool_pointer i1, i2, j1, j2;
    {
        if (scan_toks(false, true) != 0) ;
    }
    s1 = tokens_to_string(def_ref);
    delete_token_ref(def_ref);
    {
        if (scan_toks(false, true) != 0) ;
    }
    s2 = tokens_to_string(def_ref);
    delete_token_ref(def_ref);
    i1 = str_start[(s1) - 65536L];
    j1 = str_start[(s1 + 1) - 65536L];
    i2 = str_start[(s2) - 65536L];
    j2 = str_start[(s2 + 1) - 65536L];
    while ((i1 < j1) && (i2 < j2)) {

        if (str_pool[i1] < str_pool[i2]) {
            cur_val = -1;
            goto done;
        }
        if (str_pool[i1] > str_pool[i2]) {
            cur_val = 1;
            goto done;
        }
        i1++;
        i2++;
    }
    if ((i1 == j1) && (i2 == j2))
        cur_val = 0;
    else if (i1 < j1)
        cur_val = 1;
    else
        cur_val = -1;
done:
    flush_str(s2);
    flush_str(s1);
    cur_val_level = INT_VAL;
}
