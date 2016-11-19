void initialize(void);
#define initialize_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void print_ln(void);
#define print_ln_regmem
void zprint_raw_char(UTF16_code s, boolean incr_offset);
#define print_raw_char(s, incr_offset) zprint_raw_char((UTF16_code) (s), (boolean) (incr_offset))
#define print_raw_char_regmem
void zprint_char(integer s);
#define print_char(s) zprint_char((integer) (s))
#define print_char_regmem register memoryword *eqtb=zeqtb;
void zprint(integer s);
#define print(s) zprint((integer) (s))
#define print_regmem register memoryword *eqtb=zeqtb;
void zprint_nl(str_number s);
#define print_nl(s) zprint_nl((str_number) (s))
#define print_nl_regmem
void zprint_esc(str_number s);
#define print_esc(s) zprint_esc((str_number) (s))
#define print_esc_regmem register memoryword *eqtb=zeqtb;
void zprint_the_digs(eight_bits k);
#define print_the_digs(k) zprint_the_digs((eight_bits) (k))
#define print_the_digs_regmem
void zprint_int(integer n);
#define print_int(n) zprint_int((integer) (n))
#define print_int_regmem
void zprint_cs(integer p);
#define print_cs(p) zprint_cs((integer) (p))
#define print_cs_regmem register memoryword *eqtb=zeqtb;
void zsprint_cs(halfword p);
#define sprint_cs(p) zsprint_cs((halfword) (p))
#define sprint_cs_regmem
void zprint_file_name(integer n, integer a, integer e);
#define print_file_name(n, a, e) zprint_file_name((integer) (n), (integer) (a), (integer) (e))
#define print_file_name_regmem
void zprint_size(integer s);
#define print_size(s) zprint_size((integer) (s))
#define print_size_regmem
void zprint_write_whatsit(str_number s, halfword p);
#define print_write_whatsit(s, p) zprint_write_whatsit((str_number) (s), (halfword) (p))
#define print_write_whatsit_regmem register memoryword *mem=zmem;
void zprint_native_word(halfword p);
#define print_native_word(p) zprint_native_word((halfword) (p))
#define print_native_word_regmem register memoryword *mem=zmem;
void zprint_sa_num(halfword q);
#define print_sa_num(q) zprint_sa_num((halfword) (q))
#define print_sa_num_regmem register memoryword *mem=zmem;
void zprint_csnames(integer hstart, integer hfinish);
#define print_csnames(hstart, hfinish) zprint_csnames((integer) (hstart), (integer) (hfinish))
#define print_csnames_regmem
void print_file_line(void);
#define print_file_line_regmem
WEB2C_NORETURN void jump_out(void);
#define jump_out_regmem
void error(void);
#define error_regmem
WEB2C_NORETURN void zfatal_error(str_number s);
#define fatal_error(s) zfatal_error((str_number) (s))
#define fatal_error_regmem
WEB2C_NORETURN void zoverflow(str_number s, integer n);
#define overflow(s, n) zoverflow((str_number) (s), (integer) (n))
#define overflow_regmem
WEB2C_NORETURN void zconfusion(str_number s);
#define confusion(s) zconfusion((str_number) (s))
#define confusion_regmem
boolean init_terminal(void);
#define init_terminal_regmem
integer zlength(str_number s);
#define length(s) zlength((str_number) (s))
#define length_regmem
str_number make_string(void);
#define make_string_regmem
void zappend_str(str_number s);
#define append_str(s) zappend_str((str_number) (s))
#define append_str_regmem
boolean zstr_eq_buf(str_number s, integer k);
#define str_eq_buf(s, k) zstr_eq_buf((str_number) (s), (integer) (k))
#define str_eq_buf_regmem
boolean zstr_eq_str(str_number s, str_number t);
#define str_eq_str(s, t) zstr_eq_str((str_number) (s), (str_number) (t))
#define str_eq_str_regmem
str_number zsearch_string(str_number search);
#define search_string(search) zsearch_string((str_number) (search))
#define search_string_regmem
str_number slow_make_string(void);
#define slow_make_string_regmem
boolean get_strings_started(void);
#define get_strings_started_regmem
void zprint_two(integer n);
#define print_two(n) zprint_two((integer) (n))
#define print_two_regmem
void zprint_hex(integer n);
#define print_hex(n) zprint_hex((integer) (n))
#define print_hex_regmem
void zprint_roman_int(integer n);
#define print_roman_int(n) zprint_roman_int((integer) (n))
#define print_roman_int_regmem
void print_current_string(void);
#define print_current_string_regmem
void term_input(void);
#define term_input_regmem
void zint_error(integer n);
#define int_error(n) zint_error((integer) (n))
#define int_error_regmem
void normalize_selector(void);
#define normalize_selector_regmem
void pause_for_instructions(void);
#define pause_for_instructions_regmem
integer zhalf(integer x);
#define half(x) zhalf((integer) (x))
#define half_regmem
scaled zround_decimals(small_number k);
#define round_decimals(k) zround_decimals((small_number) (k))
#define round_decimals_regmem
void zprint_scaled(scaled s);
#define print_scaled(s) zprint_scaled((scaled) (s))
#define print_scaled_regmem
scaled zmult_and_add(integer n, scaled x, scaled y, scaled max_answer);
#define mult_and_add(n, x, y, max_answer) zmult_and_add((integer) (n), (scaled) (x), (scaled) (y), (scaled) (max_answer))
#define mult_and_add_regmem
scaled zx_over_n(scaled x, integer n);
#define x_over_n(x, n) zx_over_n((scaled) (x), (integer) (n))
#define x_over_n_regmem
scaled zxn_over_d(scaled x, integer n, integer d);
#define xn_over_d(x, n, d) zxn_over_d((scaled) (x), (integer) (n), (integer) (d))
#define xn_over_d_regmem
halfword zbadness(scaled t, scaled s);
#define badness(t, s) zbadness((scaled) (t), (scaled) (s))
#define badness_regmem
void zprint_word(memory_word w);
#define print_word(w) zprint_word((memory_word) (w))
#define print_word_regmem
void zshow_token_list(integer p, integer q, integer l);
#define show_token_list(p, q, l) zshow_token_list((integer) (p), (integer) (q), (integer) (l))
#define show_token_list_regmem register memoryword *mem=zmem;
void runaway(void);
#define runaway_regmem register memoryword *mem=zmem;
halfword get_avail(void);
#define get_avail_regmem register memoryword *mem=zmem;
void zflush_list(halfword p);
#define flush_list(p) zflush_list((halfword) (p))
#define flush_list_regmem register memoryword *mem=zmem;
halfword zget_node(integer s);
#define get_node(s) zget_node((integer) (s))
#define get_node_regmem register memoryword *mem=zmem;
void zfree_node(halfword p, halfword s);
#define free_node(p, s) zfree_node((halfword) (p), (halfword) (s))
#define free_node_regmem register memoryword *mem=zmem;
void sort_avail(void);
#define sort_avail_regmem register memoryword *mem=zmem;
halfword new_null_box(void);
#define new_null_box_regmem register memoryword *mem=zmem;
halfword new_rule(void);
#define new_rule_regmem register memoryword *mem=zmem;
halfword znew_ligature(internal_font_number f, quarterword c, halfword q);
#define new_ligature(f, c, q) znew_ligature((internal_font_number) (f), (quarterword) (c), (halfword) (q))
#define new_ligature_regmem register memoryword *mem=zmem;
halfword znew_lig_item(quarterword c);
#define new_lig_item(c) znew_lig_item((quarterword) (c))
#define new_lig_item_regmem register memoryword *mem=zmem;
halfword new_disc(void);
#define new_disc_regmem register memoryword *mem=zmem;
void zcopy_native_glyph_info(halfword src, halfword dest);
#define copy_native_glyph_info(src, dest) zcopy_native_glyph_info((halfword) (src), (halfword) (dest))
#define copy_native_glyph_info_regmem register memoryword *mem=zmem;
halfword znew_math(scaled w, small_number s);
#define new_math(w, s) znew_math((scaled) (w), (small_number) (s))
#define new_math_regmem register memoryword *mem=zmem;
halfword znew_spec(halfword p);
#define new_spec(p) znew_spec((halfword) (p))
#define new_spec_regmem register memoryword *mem=zmem;
halfword znew_param_glue(small_number n);
#define new_param_glue(n) znew_param_glue((small_number) (n))
#define new_param_glue_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword znew_glue(halfword q);
#define new_glue(q) znew_glue((halfword) (q))
#define new_glue_regmem register memoryword *mem=zmem;
halfword znew_skip_param(small_number n);
#define new_skip_param(n) znew_skip_param((small_number) (n))
#define new_skip_param_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword znew_kern(scaled w);
#define new_kern(w) znew_kern((scaled) (w))
#define new_kern_regmem register memoryword *mem=zmem;
halfword znew_penalty(integer m);
#define new_penalty(m) znew_penalty((integer) (m))
#define new_penalty_regmem register memoryword *mem=zmem;
void zcheck_mem(boolean print_locs);
#define check_mem(print_locs) zcheck_mem((boolean) (print_locs))
#define check_mem_regmem register memoryword *mem=zmem;
void zsearch_mem(halfword p);
#define search_mem(p) zsearch_mem((halfword) (p))
#define search_mem_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zpdf_error(str_number t, str_number p);
#define pdf_error(t, p) zpdf_error((str_number) (t), (str_number) (p))
#define pdf_error_regmem
halfword zprev_rightmost(halfword s, halfword e);
#define prev_rightmost(s, e) zprev_rightmost((halfword) (s), (halfword) (e))
#define prev_rightmost_regmem register memoryword *mem=zmem;
scaled zround_xn_over_d(scaled x, integer n, integer d);
#define round_xn_over_d(x, n, d) zround_xn_over_d((scaled) (x), (integer) (n), (integer) (d))
#define round_xn_over_d_regmem
void zshort_display(integer p);
#define short_display(p) zshort_display((integer) (p))
#define short_display_regmem register memoryword *mem=zmem;
void zprint_font_and_char(integer p);
#define print_font_and_char(p) zprint_font_and_char((integer) (p))
#define print_font_and_char_regmem register memoryword *mem=zmem;
void zprint_mark(integer p);
#define print_mark(p) zprint_mark((integer) (p))
#define print_mark_regmem register memoryword *mem=zmem;
void zprint_rule_dimen(scaled d);
#define print_rule_dimen(d) zprint_rule_dimen((scaled) (d))
#define print_rule_dimen_regmem
void zprint_glue(scaled d, integer order, str_number s);
#define print_glue(d, order, s) zprint_glue((scaled) (d), (integer) (order), (str_number) (s))
#define print_glue_regmem
void zprint_spec(integer p, str_number s);
#define print_spec(p, s) zprint_spec((integer) (p), (str_number) (s))
#define print_spec_regmem register memoryword *mem=zmem;
void zprint_fam_and_char(halfword p);
#define print_fam_and_char(p) zprint_fam_and_char((halfword) (p))
#define print_fam_and_char_regmem register memoryword *mem=zmem;
void zprint_delimiter(halfword p);
#define print_delimiter(p) zprint_delimiter((halfword) (p))
#define print_delimiter_regmem register memoryword *mem=zmem;
void zprint_subsidiary_data(halfword p, UTF16_code c);
#define print_subsidiary_data(p, c) zprint_subsidiary_data((halfword) (p), (UTF16_code) (c))
#define print_subsidiary_data_regmem register memoryword *mem=zmem;
void zprint_style(integer c);
#define print_style(c) zprint_style((integer) (c))
#define print_style_regmem
void zprint_skip_param(integer n);
#define print_skip_param(n) zprint_skip_param((integer) (n))
#define print_skip_param_regmem
void zshow_node_list(integer p);
#define show_node_list(p) zshow_node_list((integer) (p))
#define show_node_list_regmem register memoryword *mem=zmem;
void zshow_box(halfword p);
#define show_box(p) zshow_box((halfword) (p))
#define show_box_regmem register memoryword *eqtb=zeqtb;
void zshort_display_n(integer p, integer m);
#define short_display_n(p, m) zshort_display_n((integer) (p), (integer) (m))
#define short_display_n_regmem
void zdelete_token_ref(halfword p);
#define delete_token_ref(p) zdelete_token_ref((halfword) (p))
#define delete_token_ref_regmem register memoryword *mem=zmem;
void zdelete_glue_ref(halfword p);
#define delete_glue_ref(p) zdelete_glue_ref((halfword) (p))
#define delete_glue_ref_regmem register memoryword *mem=zmem;
void zflush_node_list(halfword p);
#define flush_node_list(p) zflush_node_list((halfword) (p))
#define flush_node_list_regmem register memoryword *mem=zmem;
halfword zcopy_node_list(halfword p);
#define copy_node_list(p) zcopy_node_list((halfword) (p))
#define copy_node_list_regmem register memoryword *mem=zmem;
void zprint_mode(integer m);
#define print_mode(m) zprint_mode((integer) (m))
#define print_mode_regmem
void zprint_in_mode(integer m);
#define print_in_mode(m) zprint_in_mode((integer) (m))
#define print_in_mode_regmem
void push_nest(void);
#define push_nest_regmem
void pop_nest(void);
#define pop_nest_regmem register memoryword *mem=zmem;
void show_activities(void);
#define show_activities_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zprint_param(integer n);
#define print_param(n) zprint_param((integer) (n))
#define print_param_regmem
void begin_diagnostic(void);
#define begin_diagnostic_regmem register memoryword *eqtb=zeqtb;
void zend_diagnostic(boolean blank_line);
#define end_diagnostic(blank_line) zend_diagnostic((boolean) (blank_line))
#define end_diagnostic_regmem
void zprint_length_param(integer n);
#define print_length_param(n) zprint_length_param((integer) (n))
#define print_length_param_regmem
void zprint_cmd_chr(quarterword cmd, halfword chr_code);
#define print_cmd_chr(cmd, chr_code) zprint_cmd_chr((quarterword) (cmd), (halfword) (chr_code))
#define print_cmd_chr_regmem register memoryword *mem=zmem;
void znot_aat_font_error(integer cmd, integer c, integer f);
#define not_aat_font_error(cmd, c, f) znot_aat_font_error((integer) (cmd), (integer) (c), (integer) (f))
#define not_aat_font_error_regmem
void znot_aat_gr_font_error(integer cmd, integer c, integer f);
#define not_aat_gr_font_error(cmd, c, f) znot_aat_gr_font_error((integer) (cmd), (integer) (c), (integer) (f))
#define not_aat_gr_font_error_regmem
void znot_ot_font_error(integer cmd, integer c, integer f);
#define not_ot_font_error(cmd, c, f) znot_ot_font_error((integer) (cmd), (integer) (c), (integer) (f))
#define not_ot_font_error_regmem
void znot_native_font_error(integer cmd, integer c, integer f);
#define not_native_font_error(cmd, c, f) znot_native_font_error((integer) (cmd), (integer) (c), (integer) (f))
#define not_native_font_error_regmem
void zshow_eqtb(halfword n);
#define show_eqtb(n) zshow_eqtb((halfword) (n))
#define show_eqtb_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword zid_lookup(integer j, integer l);
#define id_lookup(j, l) zid_lookup((integer) (j), (integer) (l))
#define id_lookup_regmem
halfword zprim_lookup(str_number s);
#define prim_lookup(s) zprim_lookup((str_number) (s))
#define prim_lookup_regmem
void zprimitive(str_number s, quarterword c, halfword o);
#define primitive(s, c, o) zprimitive((str_number) (s), (quarterword) (c), (halfword) (o))
#define primitive_regmem register memoryword *eqtb=zeqtb;
void zrestore_trace(halfword p, str_number s);
#define restore_trace(p, s) zrestore_trace((halfword) (p), (str_number) (s))
#define restore_trace_regmem
void zprint_group(boolean e);
#define print_group(e) zprint_group((boolean) (e))
#define print_group_regmem
void zgroup_trace(boolean e);
#define group_trace(e) zgroup_trace((boolean) (e))
#define group_trace_regmem
boolean pseudo_input(void);
#define pseudo_input_regmem register memoryword *mem=zmem;
void pseudo_close(void);
#define pseudo_close_regmem register memoryword *mem=zmem;
void group_warning(void);
#define group_warning_regmem register memoryword *eqtb=zeqtb;
void if_warning(void);
#define if_warning_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void file_warning(void);
#define file_warning_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zdelete_sa_ref(halfword q);
#define delete_sa_ref(q) zdelete_sa_ref((halfword) (q))
#define delete_sa_ref_regmem register memoryword *mem=zmem;
void zshow_sa(halfword p, str_number s);
#define show_sa(p, s) zshow_sa((halfword) (p), (str_number) (s))
#define show_sa_regmem register memoryword *mem=zmem;
void zsa_save(halfword p);
#define sa_save(p) zsa_save((halfword) (p))
#define sa_save_regmem register memoryword *mem=zmem;
void zsa_destroy(halfword p);
#define sa_destroy(p) zsa_destroy((halfword) (p))
#define sa_destroy_regmem register memoryword *mem=zmem;
void zsa_def(halfword p, halfword e);
#define sa_def(p, e) zsa_def((halfword) (p), (halfword) (e))
#define sa_def_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zsa_w_def(halfword p, integer w);
#define sa_w_def(p, w) zsa_w_def((halfword) (p), (integer) (w))
#define sa_w_def_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zgsa_def(halfword p, halfword e);
#define gsa_def(p, e) zgsa_def((halfword) (p), (halfword) (e))
#define gsa_def_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zgsa_w_def(halfword p, integer w);
#define gsa_w_def(p, w) zgsa_w_def((halfword) (p), (integer) (w))
#define gsa_w_def_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void sa_restore(void);
#define sa_restore_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void znew_save_level(group_code c);
#define new_save_level(c) znew_save_level((group_code) (c))
#define new_save_level_regmem register memoryword *eqtb=zeqtb;
void zeq_destroy(memory_word w);
#define eq_destroy(w) zeq_destroy((memory_word) (w))
#define eq_destroy_regmem register memoryword *mem=zmem;
void zeq_save(halfword p, quarterword l);
#define eq_save(p, l) zeq_save((halfword) (p), (quarterword) (l))
#define eq_save_regmem register memoryword *eqtb=zeqtb;
void zeq_define(halfword p, quarterword t, halfword e);
#define eq_define(p, t, e) zeq_define((halfword) (p), (quarterword) (t), (halfword) (e))
#define eq_define_regmem register memoryword *eqtb=zeqtb;
void zeq_word_define(halfword p, integer w);
#define eq_word_define(p, w) zeq_word_define((halfword) (p), (integer) (w))
#define eq_word_define_regmem register memoryword *eqtb=zeqtb;
void zgeq_define(halfword p, quarterword t, halfword e);
#define geq_define(p, t, e) zgeq_define((halfword) (p), (quarterword) (t), (halfword) (e))
#define geq_define_regmem register memoryword *eqtb=zeqtb;
void zgeq_word_define(halfword p, integer w);
#define geq_word_define(p, w) zgeq_word_define((halfword) (p), (integer) (w))
#define geq_word_define_regmem register memoryword *eqtb=zeqtb;
void zsave_for_after(halfword t);
#define save_for_after(t) zsave_for_after((halfword) (t))
#define save_for_after_regmem
void unsave(void);
#define unsave_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void prepare_mag(void);
#define prepare_mag_regmem register memoryword *eqtb=zeqtb;
void ztoken_show(halfword p);
#define token_show(p) ztoken_show((halfword) (p))
#define token_show_regmem register memoryword *mem=zmem;
void print_meaning(void);
#define print_meaning_regmem
void show_cur_cmd_chr(void);
#define show_cur_cmd_chr_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void show_context(void);
#define show_context_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zbegin_token_list(halfword p, quarterword t);
#define begin_token_list(p, t) zbegin_token_list((halfword) (p), (quarterword) (t))
#define begin_token_list_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void end_token_list(void);
#define end_token_list_regmem
void back_input(void);
#define back_input_regmem register memoryword *mem=zmem;
void back_error(void);
#define back_error_regmem
void ins_error(void);
#define ins_error_regmem
void begin_file_reading(void);
#define begin_file_reading_regmem
void end_file_reading(void);
#define end_file_reading_regmem
void clear_for_error_prompt(void);
#define clear_for_error_prompt_regmem
void check_outer_validity(void);
#define check_outer_validity_regmem register memoryword *mem=zmem;
void get_next(void);
#define get_next_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void firm_up_the_line(void);
#define firm_up_the_line_regmem register memoryword *eqtb=zeqtb;
void get_token(void);
#define get_token_regmem
void macro_call(void);
#define macro_call_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void insert_relax(void);
#define insert_relax_regmem
void znew_index(quarterword i, halfword q);
#define new_index(i, q) znew_index((quarterword) (i), (halfword) (q))
#define new_index_regmem register memoryword *mem=zmem;
void zfind_sa_element(small_number t, halfword n, boolean w);
#define find_sa_element(t, n, w) zfind_sa_element((small_number) (t), (halfword) (n), (boolean) (w))
#define find_sa_element_regmem register memoryword *mem=zmem;
void expand(void);
#define expand_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void get_x_token(void);
#define get_x_token_regmem
void x_token(void);
#define x_token_regmem
void scan_left_brace(void);
#define scan_left_brace_regmem
void scan_optional_equals(void);
#define scan_optional_equals_regmem
boolean zscan_keyword(str_number s);
#define scan_keyword(s) zscan_keyword((str_number) (s))
#define scan_keyword_regmem register memoryword *mem=zmem;
void mu_error(void);
#define mu_error_regmem
void zscan_glyph_number(internal_font_number f);
#define scan_glyph_number(f) zscan_glyph_number((internal_font_number) (f))
#define scan_glyph_number_regmem
void scan_char_class(void);
#define scan_char_class_regmem
void scan_char_class_not_ignored(void);
#define scan_char_class_not_ignored_regmem
void scan_eight_bit_int(void);
#define scan_eight_bit_int_regmem
void scan_usv_num(void);
#define scan_usv_num_regmem
void scan_char_num(void);
#define scan_char_num_regmem
void scan_xetex_math_char_int(void);
#define scan_xetex_math_char_int_regmem
void scan_math_class_int(void);
#define scan_math_class_int_regmem
void scan_math_fam_int(void);
#define scan_math_fam_int_regmem
void scan_four_bit_int(void);
#define scan_four_bit_int_regmem
void scan_fifteen_bit_int(void);
#define scan_fifteen_bit_int_regmem
void scan_delimiter_int(void);
#define scan_delimiter_int_regmem
void scan_register_num(void);
#define scan_register_num_regmem
void scan_four_bit_int_or_18(void);
#define scan_four_bit_int_or_18_regmem
void get_x_or_protected(void);
#define get_x_or_protected_regmem register memoryword *mem=zmem;
integer zeffective_char(boolean err_p, internal_font_number f, quarterword c);
#define effective_char(err_p, f, c) zeffective_char((boolean) (err_p), (internal_font_number) (f), (quarterword) (c))
#define effective_char_regmem register memoryword *eqtb=zeqtb;
void scan_font_ident(void);
#define scan_font_ident_regmem register memoryword *eqtb=zeqtb;
void zfind_font_dimen(boolean writing);
#define find_font_dimen(writing) zfind_font_dimen((boolean) (writing))
#define find_font_dimen_regmem
void zscan_something_internal(small_number level, boolean negative);
#define scan_something_internal(level, negative) zscan_something_internal((small_number) (level), (boolean) (negative))
#define scan_something_internal_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void scan_int(void);
#define scan_int_regmem
void zxetex_scan_dimen(boolean mu, boolean inf, boolean shortcut, boolean requires_units);
#define xetex_scan_dimen(mu, inf, shortcut, requires_units) zxetex_scan_dimen((boolean) (mu), (boolean) (inf), (boolean) (shortcut), (boolean) (requires_units))
#define xetex_scan_dimen_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zscan_dimen(boolean mu, boolean inf, boolean shortcut);
#define scan_dimen(mu, inf, shortcut) zscan_dimen((boolean) (mu), (boolean) (inf), (boolean) (shortcut))
#define scan_dimen_regmem
void scan_decimal(void);
#define scan_decimal_regmem
void zscan_glue(small_number level);
#define scan_glue(level) zscan_glue((small_number) (level))
#define scan_glue_regmem register memoryword *mem=zmem;
integer zadd_or_sub(integer x, integer y, integer max_answer, boolean negative);
#define add_or_sub(x, y, max_answer, negative) zadd_or_sub((integer) (x), (integer) (y), (integer) (max_answer), (boolean) (negative))
#define add_or_sub_regmem
integer zquotient(integer n, integer d);
#define quotient(n, d) zquotient((integer) (n), (integer) (d))
#define quotient_regmem
integer zfract(integer x, integer n, integer d, integer max_answer);
#define fract(x, n, d, max_answer) zfract((integer) (x), (integer) (n), (integer) (d), (integer) (max_answer))
#define fract_regmem
void scan_expr(void);
#define scan_expr_regmem register memoryword *mem=zmem;
void scan_normal_glue(void);
#define scan_normal_glue_regmem
void scan_mu_glue(void);
#define scan_mu_glue_regmem
halfword scan_rule_spec(void);
#define scan_rule_spec_regmem register memoryword *mem=zmem;
void scan_general_text(void);
#define scan_general_text_regmem register memoryword *mem=zmem;
void pseudo_start(void);
#define pseudo_start_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword zstr_toks_cat(pool_pointer b, small_number cat);
#define str_toks_cat(b, cat) zstr_toks_cat((pool_pointer) (b), (small_number) (cat))
#define str_toks_cat_regmem register memoryword *mem=zmem;
halfword zstr_toks(pool_pointer b);
#define str_toks(b) zstr_toks((pool_pointer) (b))
#define str_toks_regmem
halfword the_toks(void);
#define the_toks_regmem register memoryword *mem=zmem;
void ins_the_toks(void);
#define ins_the_toks_regmem register memoryword *mem=zmem;
void conv_toks(void);
#define conv_toks_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword zscan_toks(boolean macro_def, boolean xpand);
#define scan_toks(macro_def, xpand) zscan_toks((boolean) (macro_def), (boolean) (xpand))
#define scan_toks_regmem register memoryword *mem=zmem;
void zread_toks(integer n, halfword r, halfword j);
#define read_toks(n, r, j) zread_toks((integer) (n), (halfword) (r), (halfword) (j))
#define read_toks_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void pass_text(void);
#define pass_text_regmem register memoryword *eqtb=zeqtb;
void zchange_if_limit(small_number l, halfword p);
#define change_if_limit(l, p) zchange_if_limit((small_number) (l), (halfword) (p))
#define change_if_limit_regmem register memoryword *mem=zmem;
void conditional(void);
#define conditional_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void begin_name(void);
#define begin_name_regmem
boolean zmore_name(UTF16_code c);
#define more_name(c) zmore_name((UTF16_code) (c))
#define more_name_regmem
void end_name(void);
#define end_name_regmem
void zpack_file_name(str_number n, str_number a, str_number e);
#define pack_file_name(n, a, e) zpack_file_name((str_number) (n), (str_number) (a), (str_number) (e))
#define pack_file_name_regmem
void zpack_buffered_name(small_number n, integer a, integer b);
#define pack_buffered_name(n, a, b) zpack_buffered_name((small_number) (n), (integer) (a), (integer) (b))
#define pack_buffered_name_regmem
str_number make_name_string(void);
#define make_name_string_regmem
str_number zzu_make_name_string(unicode_file * f);
#define u_make_name_string(f) zzu_make_name_string((unicode_file *) &(f))
#define u_make_name_string_regmem
str_number za_make_name_string(alpha_file f);
#define a_make_name_string(f) za_make_name_string((alpha_file) (f))
#define a_make_name_string_regmem
str_number zb_make_name_string(byte_file f);
#define b_make_name_string(f) zb_make_name_string((byte_file) (f))
#define b_make_name_string_regmem
str_number zzw_make_name_string(word_file * f);
#define w_make_name_string(f) zzw_make_name_string((word_file *) &(f))
#define w_make_name_string_regmem
void scan_file_name(void);
#define scan_file_name_regmem
void zpack_job_name(str_number s);
#define pack_job_name(s) zpack_job_name((str_number) (s))
#define pack_job_name_regmem
void zprompt_file_name(str_number s, str_number e);
#define prompt_file_name(s, e) zprompt_file_name((str_number) (s), (str_number) (e))
#define prompt_file_name_regmem
void open_log_file(void);
#define open_log_file_regmem register memoryword *eqtb=zeqtb;
void start_input(void);
#define start_input_regmem register memoryword *eqtb=zeqtb;
four_quarters zeffective_char_info(internal_font_number f, quarterword c);
#define effective_char_info(f, c) zeffective_char_info((internal_font_number) (f), (quarterword) (c))
#define effective_char_info_regmem register memoryword *eqtb=zeqtb;
void zchar_warning(internal_font_number f, integer c);
#define char_warning(f, c) zchar_warning((internal_font_number) (f), (integer) (c))
#define char_warning_regmem register memoryword *eqtb=zeqtb;
halfword znew_native_word_node(internal_font_number f, integer n);
#define new_native_word_node(f, n) znew_native_word_node((internal_font_number) (f), (integer) (n))
#define new_native_word_node_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword znew_native_character(internal_font_number f, UnicodeScalar c);
#define new_native_character(f, c) znew_native_character((internal_font_number) (f), (UnicodeScalar) (c))
#define new_native_character_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zfont_feature_warning(void_pointer featureNameP, integer featLen, void_pointer settingNameP, integer setLen);
#define font_feature_warning(featureNameP, featLen, settingNameP, setLen) zfont_feature_warning((void_pointer) (featureNameP), (integer) (featLen), (void_pointer) (settingNameP), (integer) (setLen))
#define font_feature_warning_regmem
void zfont_mapping_warning(void_pointer mappingNameP, integer mappingNameLen, integer warningType);
#define font_mapping_warning(mappingNameP, mappingNameLen, warningType) zfont_mapping_warning((void_pointer) (mappingNameP), (integer) (mappingNameLen), (integer) (warningType))
#define font_mapping_warning_regmem
void graphite_warning(void);
#define graphite_warning_regmem
internal_font_number zload_native_font(halfword u, str_number nom, str_number aire, scaled s);
#define load_native_font(u, nom, aire, s) zload_native_font((halfword) (u), (str_number) (nom), (str_number) (aire), (scaled) (s))
#define load_native_font_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zdo_locale_linebreaks(integer s, integer len);
#define do_locale_linebreaks(s, len) zdo_locale_linebreaks((integer) (s), (integer) (len))
#define do_locale_linebreaks_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void bad_utf8_warning(void);
#define bad_utf8_warning_regmem
integer get_input_normalization_state(void);
#define get_input_normalization_state_regmem register memoryword *eqtb=zeqtb;
integer get_tracing_fonts_state(void);
#define get_tracing_fonts_state_regmem register memoryword *eqtb=zeqtb;
internal_font_number zread_font_info(halfword u, str_number nom, str_number aire, scaled s);
#define read_font_info(u, nom, aire, s) zread_font_info((halfword) (u), (str_number) (nom), (str_number) (aire), (scaled) (s))
#define read_font_info_regmem register memoryword *eqtb=zeqtb;
halfword znew_character(internal_font_number f, UTF16_code c);
#define new_character(f, c) znew_character((internal_font_number) (f), (UTF16_code) (c))
#define new_character_regmem register memoryword *mem=zmem;
void dvi_swap(void);
#define dvi_swap_regmem
void zdvi_four(integer x);
#define dvi_four(x) zdvi_four((integer) (x))
#define dvi_four_regmem
void zdvi_two(UTF16_code s);
#define dvi_two(s) zdvi_two((UTF16_code) (s))
#define dvi_two_regmem
void zdvi_pop(integer l);
#define dvi_pop(l) zdvi_pop((integer) (l))
#define dvi_pop_regmem
void zdvi_native_font_def(internal_font_number f);
#define dvi_native_font_def(f) zdvi_native_font_def((internal_font_number) (f))
#define dvi_native_font_def_regmem
void zdvi_font_def(internal_font_number f);
#define dvi_font_def(f) zdvi_font_def((internal_font_number) (f))
#define dvi_font_def_regmem
void zmovement(scaled w, eight_bits o);
#define movement(w, o) zmovement((scaled) (w), (eight_bits) (o))
#define movement_regmem register memoryword *mem=zmem;
void zprune_movements(integer l);
#define prune_movements(l) zprune_movements((integer) (l))
#define prune_movements_regmem register memoryword *mem=zmem;
void zspecial_out(halfword p);
#define special_out(p) zspecial_out((halfword) (p))
#define special_out_regmem register memoryword *mem=zmem;
void zwrite_out(halfword p);
#define write_out(p) zwrite_out((halfword) (p))
#define write_out_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zpic_out(halfword p);
#define pic_out(p) zpic_out((halfword) (p))
#define pic_out_regmem register memoryword *mem=zmem;
void zout_what(halfword p);
#define out_what(p) zout_what((halfword) (p))
#define out_what_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword znew_edge(small_number s, scaled w);
#define new_edge(s, w) znew_edge((small_number) (s), (scaled) (w))
#define new_edge_regmem register memoryword *mem=zmem;
halfword zzreverse(halfword this_box, halfword t, scaled * cur_g, real * cur_glue);
#define reverse(this_box, t, cur_g, cur_glue) zzreverse((halfword) (this_box), (halfword) (t), (scaled *) &(cur_g), (real *) &(cur_glue))
#define reverse_regmem register memoryword *mem=zmem;
void hlist_out(void);
#define hlist_out_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void vlist_out(void);
#define vlist_out_regmem register memoryword *mem=zmem;
void zship_out(halfword p);
#define ship_out(p) zship_out((halfword) (p))
#define ship_out_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zscan_spec(group_code c, boolean three_codes);
#define scan_spec(c, three_codes) zscan_spec((group_code) (c), (boolean) (three_codes))
#define scan_spec_regmem
scaled zchar_pw(halfword p, small_number side);
#define char_pw(p, side) zchar_pw((halfword) (p), (small_number) (side))
#define char_pw_regmem register memoryword *mem=zmem;
halfword znew_margin_kern(scaled w, halfword p, small_number side);
#define new_margin_kern(w, p, side) znew_margin_kern((scaled) (w), (halfword) (p), (small_number) (side))
#define new_margin_kern_regmem register memoryword *mem=zmem;
halfword zhpack(halfword p, scaled w, small_number m);
#define hpack(p, w, m) zhpack((halfword) (p), (scaled) (w), (small_number) (m))
#define hpack_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword zvpackage(halfword p, scaled h, small_number m, scaled l);
#define vpackage(p, h, m, l) zvpackage((halfword) (p), (scaled) (h), (small_number) (m), (scaled) (l))
#define vpackage_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zappend_to_vlist(halfword b);
#define append_to_vlist(b) zappend_to_vlist((halfword) (b))
#define append_to_vlist_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword new_noad(void);
#define new_noad_regmem register memoryword *mem=zmem;
halfword znew_style(small_number s);
#define new_style(s) znew_style((small_number) (s))
#define new_style_regmem register memoryword *mem=zmem;
halfword new_choice(void);
#define new_choice_regmem register memoryword *mem=zmem;
void show_info(void);
#define show_info_regmem register memoryword *mem=zmem;
scaled zmath_x_height(integer size_code);
#define math_x_height(size_code) zmath_x_height((integer) (size_code))
#define math_x_height_regmem register memoryword *eqtb=zeqtb;
scaled zmath_quad(integer size_code);
#define math_quad(size_code) zmath_quad((integer) (size_code))
#define math_quad_regmem register memoryword *eqtb=zeqtb;
scaled znum1(integer size_code);
#define num1(size_code) znum1((integer) (size_code))
#define num1_regmem register memoryword *eqtb=zeqtb;
scaled znum2(integer size_code);
#define num2(size_code) znum2((integer) (size_code))
#define num2_regmem register memoryword *eqtb=zeqtb;
scaled znum3(integer size_code);
#define num3(size_code) znum3((integer) (size_code))
#define num3_regmem register memoryword *eqtb=zeqtb;
scaled zdenom1(integer size_code);
#define denom1(size_code) zdenom1((integer) (size_code))
#define denom1_regmem register memoryword *eqtb=zeqtb;
scaled zdenom2(integer size_code);
#define denom2(size_code) zdenom2((integer) (size_code))
#define denom2_regmem register memoryword *eqtb=zeqtb;
scaled zsup1(integer size_code);
#define sup1(size_code) zsup1((integer) (size_code))
#define sup1_regmem register memoryword *eqtb=zeqtb;
scaled zsup2(integer size_code);
#define sup2(size_code) zsup2((integer) (size_code))
#define sup2_regmem register memoryword *eqtb=zeqtb;
scaled zsup3(integer size_code);
#define sup3(size_code) zsup3((integer) (size_code))
#define sup3_regmem register memoryword *eqtb=zeqtb;
scaled zsub1(integer size_code);
#define sub1(size_code) zsub1((integer) (size_code))
#define sub1_regmem register memoryword *eqtb=zeqtb;
scaled zsub2(integer size_code);
#define sub2(size_code) zsub2((integer) (size_code))
#define sub2_regmem register memoryword *eqtb=zeqtb;
scaled zsup_drop(integer size_code);
#define sup_drop(size_code) zsup_drop((integer) (size_code))
#define sup_drop_regmem register memoryword *eqtb=zeqtb;
scaled zsub_drop(integer size_code);
#define sub_drop(size_code) zsub_drop((integer) (size_code))
#define sub_drop_regmem register memoryword *eqtb=zeqtb;
scaled zdelim1(integer size_code);
#define delim1(size_code) zdelim1((integer) (size_code))
#define delim1_regmem register memoryword *eqtb=zeqtb;
scaled zdelim2(integer size_code);
#define delim2(size_code) zdelim2((integer) (size_code))
#define delim2_regmem register memoryword *eqtb=zeqtb;
scaled zaxis_height(integer size_code);
#define axis_height(size_code) zaxis_height((integer) (size_code))
#define axis_height_regmem register memoryword *eqtb=zeqtb;
scaled default_rule_thickness(void);
#define default_rule_thickness_regmem register memoryword *eqtb=zeqtb;
scaled big_op_spacing1(void);
#define big_op_spacing1_regmem register memoryword *eqtb=zeqtb;
scaled big_op_spacing2(void);
#define big_op_spacing2_regmem register memoryword *eqtb=zeqtb;
scaled big_op_spacing3(void);
#define big_op_spacing3_regmem register memoryword *eqtb=zeqtb;
scaled big_op_spacing4(void);
#define big_op_spacing4_regmem register memoryword *eqtb=zeqtb;
scaled big_op_spacing5(void);
#define big_op_spacing5_regmem register memoryword *eqtb=zeqtb;
halfword zfraction_rule(scaled t);
#define fraction_rule(t) zfraction_rule((scaled) (t))
#define fraction_rule_regmem register memoryword *mem=zmem;
halfword zoverbar(halfword b, scaled k, scaled t);
#define overbar(b, k, t) zoverbar((halfword) (b), (scaled) (k), (scaled) (t))
#define overbar_regmem register memoryword *mem=zmem;
halfword zchar_box(internal_font_number f, integer c);
#define char_box(f, c) zchar_box((internal_font_number) (f), (integer) (c))
#define char_box_regmem register memoryword *mem=zmem;
void zstack_into_box(halfword b, internal_font_number f, quarterword c);
#define stack_into_box(b, f, c) zstack_into_box((halfword) (b), (internal_font_number) (f), (quarterword) (c))
#define stack_into_box_regmem register memoryword *mem=zmem;
scaled zheight_plus_depth(internal_font_number f, quarterword c);
#define height_plus_depth(f, c) zheight_plus_depth((internal_font_number) (f), (quarterword) (c))
#define height_plus_depth_regmem
void zstack_glyph_into_box(halfword b, internal_font_number f, integer g);
#define stack_glyph_into_box(b, f, g) zstack_glyph_into_box((halfword) (b), (internal_font_number) (f), (integer) (g))
#define stack_glyph_into_box_regmem register memoryword *mem=zmem;
void zstack_glue_into_box(halfword b, scaled min, scaled max);
#define stack_glue_into_box(b, min, max) zstack_glue_into_box((halfword) (b), (scaled) (min), (scaled) (max))
#define stack_glue_into_box_regmem register memoryword *mem=zmem;
halfword zbuild_opentype_assembly(internal_font_number f, void_pointer a, scaled s, boolean horiz);
#define build_opentype_assembly(f, a, s, horiz) zbuild_opentype_assembly((internal_font_number) (f), (void_pointer) (a), (scaled) (s), (boolean) (horiz))
#define build_opentype_assembly_regmem register memoryword *mem=zmem;
halfword zvar_delimiter(halfword d, integer s, scaled v);
#define var_delimiter(d, s, v) zvar_delimiter((halfword) (d), (integer) (s), (scaled) (v))
#define var_delimiter_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword zrebox(halfword b, scaled w);
#define rebox(b, w) zrebox((halfword) (b), (scaled) (w))
#define rebox_regmem register memoryword *mem=zmem;
halfword zmath_glue(halfword g, scaled m);
#define math_glue(g, m) zmath_glue((halfword) (g), (scaled) (m))
#define math_glue_regmem register memoryword *mem=zmem;
void zmath_kern(halfword p, scaled m);
#define math_kern(p, m) zmath_kern((halfword) (p), (scaled) (m))
#define math_kern_regmem register memoryword *mem=zmem;
void flush_math(void);
#define flush_math_regmem register memoryword *mem=zmem;
halfword zclean_box(halfword p, small_number s);
#define clean_box(p, s) zclean_box((halfword) (p), (small_number) (s))
#define clean_box_regmem register memoryword *mem=zmem;
void zfetch(halfword a);
#define fetch(a) zfetch((halfword) (a))
#define fetch_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zmake_over(halfword q);
#define make_over(q) zmake_over((halfword) (q))
#define make_over_regmem register memoryword *mem=zmem;
void zmake_under(halfword q);
#define make_under(q) zmake_under((halfword) (q))
#define make_under_regmem register memoryword *mem=zmem;
void zmake_vcenter(halfword q);
#define make_vcenter(q) zmake_vcenter((halfword) (q))
#define make_vcenter_regmem register memoryword *mem=zmem;
void zmake_radical(halfword q);
#define make_radical(q) zmake_radical((halfword) (q))
#define make_radical_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
scaled zcompute_ot_math_accent_pos(halfword p);
#define compute_ot_math_accent_pos(p) zcompute_ot_math_accent_pos((halfword) (p))
#define compute_ot_math_accent_pos_regmem register memoryword *mem=zmem;
void zmake_math_accent(halfword q);
#define make_math_accent(q) zmake_math_accent((halfword) (q))
#define make_math_accent_regmem register memoryword *mem=zmem;
void zmake_fraction(halfword q);
#define make_fraction(q) zmake_fraction((halfword) (q))
#define make_fraction_regmem register memoryword *mem=zmem;
scaled zmake_op(halfword q);
#define make_op(q) zmake_op((halfword) (q))
#define make_op_regmem register memoryword *mem=zmem;
void zmake_ord(halfword q);
#define make_ord(q) zmake_ord((halfword) (q))
#define make_ord_regmem register memoryword *mem=zmem;
halfword zattach_hkern_to_new_hlist(halfword q, scaled delta);
#define attach_hkern_to_new_hlist(q, delta) zattach_hkern_to_new_hlist((halfword) (q), (scaled) (delta))
#define attach_hkern_to_new_hlist_regmem register memoryword *mem=zmem;
void zmake_scripts(halfword q, scaled delta);
#define make_scripts(q, delta) zmake_scripts((halfword) (q), (scaled) (delta))
#define make_scripts_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
small_number zmake_left_right(halfword q, small_number style, scaled max_d, scaled max_h);
#define make_left_right(q, style, max_d, max_h) zmake_left_right((halfword) (q), (small_number) (style), (scaled) (max_d), (scaled) (max_h))
#define make_left_right_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void mlist_to_hlist(void);
#define mlist_to_hlist_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void push_alignment(void);
#define push_alignment_regmem register memoryword *mem=zmem;
void pop_alignment(void);
#define pop_alignment_regmem register memoryword *mem=zmem;
void get_preamble_token(void);
#define get_preamble_token_regmem register memoryword *eqtb=zeqtb;
void init_align(void);
#define init_align_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zinit_span(halfword p);
#define init_span(p) zinit_span((halfword) (p))
#define init_span_regmem
void init_row(void);
#define init_row_regmem register memoryword *mem=zmem;
void init_col(void);
#define init_col_regmem register memoryword *mem=zmem;
boolean fin_col(void);
#define fin_col_regmem register memoryword *mem=zmem;
void fin_row(void);
#define fin_row_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void fin_align(void);
#define fin_align_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void align_peek(void);
#define align_peek_regmem
halfword zfinite_shrink(halfword p);
#define finite_shrink(p) zfinite_shrink((halfword) (p))
#define finite_shrink_regmem register memoryword *mem=zmem;
void zpush_node(halfword p);
#define push_node(p) zpush_node((halfword) (p))
#define push_node_regmem
halfword pop_node(void);
#define pop_node_regmem
halfword zfind_protchar_left(halfword l, boolean d);
#define find_protchar_left(l, d) zfind_protchar_left((halfword) (l), (boolean) (d))
#define find_protchar_left_regmem register memoryword *mem=zmem;
halfword zfind_protchar_right(halfword l, halfword r);
#define find_protchar_right(l, r) zfind_protchar_right((halfword) (l), (halfword) (r))
#define find_protchar_right_regmem register memoryword *mem=zmem;
scaled ztotal_pw(halfword q, halfword p);
#define total_pw(q, p) ztotal_pw((halfword) (q), (halfword) (p))
#define total_pw_regmem register memoryword *mem=zmem;
void ztry_break(integer pi, small_number break_type);
#define try_break(pi, break_type) ztry_break((integer) (pi), (small_number) (break_type))
#define try_break_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zpost_line_break(boolean d);
#define post_line_break(d) zpost_line_break((boolean) (d))
#define post_line_break_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
small_number zreconstitute(small_number j, small_number n, halfword bchar, halfword hchar);
#define reconstitute(j, n, bchar, hchar) zreconstitute((small_number) (j), (small_number) (n), (halfword) (bchar), (halfword) (hchar))
#define reconstitute_regmem register memoryword *mem=zmem;
void hyphenate(void);
#define hyphenate_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
integer max_hyphenatable_length(void);
#define max_hyphenatable_length_regmem register memoryword *eqtb=zeqtb;
trie_opcode znew_trie_op(small_number d, small_number n, trie_opcode v);
#define new_trie_op(d, n, v) znew_trie_op((small_number) (d), (small_number) (n), (trie_opcode) (v))
#define new_trie_op_regmem
trie_pointer ztrie_node(trie_pointer p);
#define trie_node(p) ztrie_node((trie_pointer) (p))
#define trie_node_regmem
trie_pointer zcompress_trie(trie_pointer p);
#define compress_trie(p) zcompress_trie((trie_pointer) (p))
#define compress_trie_regmem
void zfirst_fit(trie_pointer p);
#define first_fit(p) zfirst_fit((trie_pointer) (p))
#define first_fit_regmem
void ztrie_pack(trie_pointer p);
#define trie_pack(p) ztrie_pack((trie_pointer) (p))
#define trie_pack_regmem
void ztrie_fix(trie_pointer p);
#define trie_fix(p) ztrie_fix((trie_pointer) (p))
#define trie_fix_regmem
void new_patterns(void);
#define new_patterns_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void init_trie(void);
#define init_trie_regmem
void zline_break(boolean d);
#define line_break(d) zline_break((boolean) (d))
#define line_break_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
boolean zeTeX_enabled(boolean b, quarterword j, halfword k);
#define eTeX_enabled(b, j, k) zeTeX_enabled((boolean) (b), (quarterword) (j), (halfword) (k))
#define eTeX_enabled_regmem
void show_save_groups(void);
#define show_save_groups_regmem register memoryword *mem=zmem;
void new_hyph_exceptions(void);
#define new_hyph_exceptions_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
halfword zprune_page_top(halfword p, boolean s);
#define prune_page_top(p, s) zprune_page_top((halfword) (p), (boolean) (s))
#define prune_page_top_regmem register memoryword *mem=zmem;
halfword zvert_break(halfword p, scaled h, scaled d);
#define vert_break(p, h, d) zvert_break((halfword) (p), (scaled) (h), (scaled) (d))
#define vert_break_regmem register memoryword *mem=zmem;
boolean zdo_marks(small_number a, small_number l, halfword q);
#define do_marks(a, l, q) zdo_marks((small_number) (a), (small_number) (l), (halfword) (q))
#define do_marks_regmem register memoryword *mem=zmem;
halfword zvsplit(halfword n, scaled h);
#define vsplit(n, h) zvsplit((halfword) (n), (scaled) (h))
#define vsplit_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void print_totals(void);
#define print_totals_regmem
void zfreeze_page_specs(small_number s);
#define freeze_page_specs(s) zfreeze_page_specs((small_number) (s))
#define freeze_page_specs_regmem register memoryword *eqtb=zeqtb;
void zbox_error(eight_bits n);
#define box_error(n) zbox_error((eight_bits) (n))
#define box_error_regmem register memoryword *eqtb=zeqtb;
void zensure_vbox(eight_bits n);
#define ensure_vbox(n) zensure_vbox((eight_bits) (n))
#define ensure_vbox_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zfire_up(halfword c);
#define fire_up(c) zfire_up((halfword) (c))
#define fire_up_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void build_page(void);
#define build_page_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void app_space(void);
#define app_space_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void insert_dollar_sign(void);
#define insert_dollar_sign_regmem
void you_cant(void);
#define you_cant_regmem
void report_illegal_case(void);
#define report_illegal_case_regmem
boolean privileged(void);
#define privileged_regmem
boolean its_all_over(void);
#define its_all_over_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void append_glue(void);
#define append_glue_regmem register memoryword *mem=zmem;
void append_kern(void);
#define append_kern_regmem register memoryword *mem=zmem;
void off_save(void);
#define off_save_regmem register memoryword *mem=zmem;
void extra_right_brace(void);
#define extra_right_brace_regmem
void normal_paragraph(void);
#define normal_paragraph_regmem register memoryword *eqtb=zeqtb;
void zbox_end(integer box_context);
#define box_end(box_context) zbox_end((integer) (box_context))
#define box_end_regmem register memoryword *mem=zmem;
void zbegin_box(integer box_context);
#define begin_box(box_context) zbegin_box((integer) (box_context))
#define begin_box_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zscan_box(integer box_context);
#define scan_box(box_context) zscan_box((integer) (box_context))
#define scan_box_regmem
void zpackage(small_number c);
#define package(c) zpackage((small_number) (c))
#define package_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
small_number znorm_min(integer h);
#define norm_min(h) znorm_min((integer) (h))
#define norm_min_regmem
void znew_graf(boolean indented);
#define new_graf(indented) znew_graf((boolean) (indented))
#define new_graf_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void indent_in_hmode(void);
#define indent_in_hmode_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void head_for_vmode(void);
#define head_for_vmode_regmem
void end_graf(void);
#define end_graf_regmem
void begin_insert_or_adjust(void);
#define begin_insert_or_adjust_regmem
void make_mark(void);
#define make_mark_regmem register memoryword *mem=zmem;
void append_penalty(void);
#define append_penalty_regmem register memoryword *mem=zmem;
void delete_last(void);
#define delete_last_regmem register memoryword *mem=zmem;
void unpackage(void);
#define unpackage_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void append_italic_correction(void);
#define append_italic_correction_regmem register memoryword *mem=zmem;
void append_discretionary(void);
#define append_discretionary_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void build_discretionary(void);
#define build_discretionary_regmem register memoryword *mem=zmem;
void make_accent(void);
#define make_accent_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void align_error(void);
#define align_error_regmem
void no_align_error(void);
#define no_align_error_regmem
void omit_error(void);
#define omit_error_regmem
void do_endv(void);
#define do_endv_regmem
void cs_error(void);
#define cs_error_regmem
void zpush_math(group_code c);
#define push_math(c) zpush_math((group_code) (c))
#define push_math_regmem
void zjust_copy(halfword p, halfword h, halfword t);
#define just_copy(p, h, t) zjust_copy((halfword) (p), (halfword) (h), (halfword) (t))
#define just_copy_regmem register memoryword *mem=zmem;
void zjust_reverse(halfword p);
#define just_reverse(p) zjust_reverse((halfword) (p))
#define just_reverse_regmem register memoryword *mem=zmem;
void init_math(void);
#define init_math_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void start_eq_no(void);
#define start_eq_no_regmem register memoryword *eqtb=zeqtb;
void zscan_math(halfword p);
#define scan_math(p) zscan_math((halfword) (p))
#define scan_math_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void zset_math_char(integer c);
#define set_math_char(c) zset_math_char((integer) (c))
#define set_math_char_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void math_limit_switch(void);
#define math_limit_switch_regmem register memoryword *mem=zmem;
void zscan_delimiter(halfword p, boolean r);
#define scan_delimiter(p, r) zscan_delimiter((halfword) (p), (boolean) (r))
#define scan_delimiter_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void math_radical(void);
#define math_radical_regmem register memoryword *mem=zmem;
void math_ac(void);
#define math_ac_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void append_choices(void);
#define append_choices_regmem register memoryword *mem=zmem;
halfword zfin_mlist(halfword p);
#define fin_mlist(p) zfin_mlist((halfword) (p))
#define fin_mlist_regmem register memoryword *mem=zmem;
void build_choices(void);
#define build_choices_regmem register memoryword *mem=zmem;
void sub_sup(void);
#define sub_sup_regmem register memoryword *mem=zmem;
void math_fraction(void);
#define math_fraction_regmem register memoryword *mem=zmem;
void math_left_right(void);
#define math_left_right_regmem register memoryword *mem=zmem;
void zapp_display(halfword j, halfword b, scaled d);
#define app_display(j, b, d) zapp_display((halfword) (j), (halfword) (b), (scaled) (d))
#define app_display_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void after_math(void);
#define after_math_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void resume_after_display(void);
#define resume_after_display_regmem register memoryword *eqtb=zeqtb;
void get_r_token(void);
#define get_r_token_regmem
void trap_zero_glue(void);
#define trap_zero_glue_regmem register memoryword *mem=zmem;
void zdo_register_command(small_number a);
#define do_register_command(a) zdo_register_command((small_number) (a))
#define do_register_command_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void alter_aux(void);
#define alter_aux_regmem
void alter_prev_graf(void);
#define alter_prev_graf_regmem
void alter_page_so_far(void);
#define alter_page_so_far_regmem
void alter_integer(void);
#define alter_integer_regmem
void alter_box_dimen(void);
#define alter_box_dimen_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void znew_font(small_number a);
#define new_font(a) znew_font((small_number) (a))
#define new_font_regmem register memoryword *eqtb=zeqtb;
void new_interaction(void);
#define new_interaction_regmem
void prefixed_command(void);
#define prefixed_command_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void do_assignments(void);
#define do_assignments_regmem
void open_or_close_in(void);
#define open_or_close_in_regmem register memoryword *eqtb=zeqtb;
void issue_message(void);
#define issue_message_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void shift_case(void);
#define shift_case_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void show_whatever(void);
#define show_whatever_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void store_fmt_file(void);
#define store_fmt_file_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void znew_whatsit(small_number s, small_number w);
#define new_whatsit(s, w) znew_whatsit((small_number) (s), (small_number) (w))
#define new_whatsit_regmem register memoryword *mem=zmem;
void znew_write_whatsit(small_number w);
#define new_write_whatsit(w) znew_write_whatsit((small_number) (w))
#define new_write_whatsit_regmem register memoryword *mem=zmem;
void zload_picture(boolean is_pdf);
#define load_picture(is_pdf) zload_picture((boolean) (is_pdf))
#define load_picture_regmem register memoryword *mem=zmem;
void scan_and_pack_name(void);
#define scan_and_pack_name_regmem
void do_extension(void);
#define do_extension_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void fix_language(void);
#define fix_language_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void insert_src_special(void);
#define insert_src_special_regmem register memoryword *mem=zmem;
void append_src_special(void);
#define append_src_special_regmem register memoryword *mem=zmem;
void handle_right_brace(void);
#define handle_right_brace_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void main_control(void);
#define main_control_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void give_err_help(void);
#define give_err_help_regmem register memoryword *eqtb=zeqtb;
boolean open_fmt_file(void);
#define open_fmt_file_regmem
boolean load_fmt_file(void);
#define load_fmt_file_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void close_files_and_terminate(void);
#define close_files_and_terminate_regmem register memoryword *eqtb=zeqtb;
void final_cleanup(void);
#define final_cleanup_regmem register memoryword *mem=zmem;
void init_prim(void);
#define init_prim_regmem register memoryword *eqtb=zeqtb;
void debug_help(void);
#define debug_help_regmem register memoryword *mem=zmem, *eqtb=zeqtb;
void main_body(void);
#define main_body_regmem register memoryword *eqtb=zeqtb;
void zflush_str(str_number s);
#define flush_str(s) zflush_str((str_number) (s))
#define flush_str_regmem
str_number ztokens_to_string(halfword p);
#define tokens_to_string(p) ztokens_to_string((halfword) (p))
#define tokens_to_string_regmem register memoryword *mem=zmem;
void scan_pdf_ext_toks(void);
#define scan_pdf_ext_toks_regmem
void compare_strings(void);
#define compare_strings_regmem
str_number get_nullstr(void);
#define get_nullstr_regmem
/* Some definitions that get appended to the `coerce.h' file that web2c
   outputs.  */
/* $Id$ */

/* The C compiler ignores most unnecessary casts (i.e., casts of
   something to its own type).  However, for structures, it doesn't.
   Therefore, we have to redefine these macros so they don't cast
   their argument (of type memoryword or fourquarters, respectively).  */

#ifdef	printword
#undef	printword
#define	printword(x)	zprintword (x)
#endif

#ifdef	tfmqqqq
#undef	tfmqqqq
#define tfmqqqq(x)	ztfmqqqq (x)
#endif

#ifdef	eqdestroy
#undef	eqdestroy
#define	eqdestroy(x)	zeqdestroy(x)
#endif

/* And we use the opportunity to declare a few functions that could not be
   declared in texmfmp.h, because they need typedefs not yet known at that
   point.  */
extern str_number get_job_name(str_number);

#ifdef XeTeX
/* XeTeX redefines "ASCII" types.... */
typedef packed_UTF16_code packedASCIIcode;
#endif
extern void call_edit(packedASCIIcode *, pool_pointer, integer, integer);

#ifdef MF
extern void blankrectangle(screencol, screencol, screenrow, screenrow);
extern void paintrow(screenrow, pixelcolor, transspec, screencol);
#if defined(MFLua) || defined(MFLuaJIT)
#include <mfluadir/mfluac.h>
#endif
#endif

extern str_number make_full_name_string(void);

#ifdef TeX
extern string gettexstring(str_number);
/* Prototypes for source-specials functions... */
extern boolean is_new_source(str_number, int);
extern pool_pointer make_src_special(str_number, int);
extern void remember_source_info(str_number, int);

#ifdef pdfTeX
#include <pdftexdir/pdftex.h>
#endif                          /* pdfTeX */

#ifdef XeTeX
#include <xetexdir/xetex.h>
#endif                          /* XeTeX */

#ifdef __SyncTeX__
#include <synctexdir/synctex.h>
#endif
#endif                          /* TeX */
