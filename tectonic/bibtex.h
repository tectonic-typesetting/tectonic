void print_a_newline(void);
void mark_warning(void);
void mark_error(void);
void mark_fatal(void);
void print_overflow(void);
void print_confusion(void);
void buffer_overflow(void);
boolean zinput_ln(alpha_file f);
#define input_ln(f) zinput_ln((alpha_file) (f))
void zout_pool_str(alpha_file f, str_number s);
#define out_pool_str(f, s) zout_pool_str((alpha_file) (f), (str_number) (s))
void zprint_a_pool_str(str_number s);
#define print_a_pool_str(s) zprint_a_pool_str((str_number) (s))
void pool_overflow(void);
void zout_token(alpha_file f);
#define out_token(f) zout_token((alpha_file) (f))
void print_a_token(void);
void print_bad_input_line(void);
void print_skipping_whatever_remains(void);
void sam_too_long_file_name_print(void);
void sam_wrong_file_name_print(void);
void print_aux_name(void);
void log_pr_aux_name(void);
void aux_err_print(void);
void zaux_err_illegal_another_print(integer cmd_num);
#define aux_err_illegal_another_print(cmd_num) zaux_err_illegal_another_print((integer) (cmd_num))
void aux_err_no_right_brace_print(void);
void aux_err_stuff_after_right_brace_print(void);
void aux_err_white_space_in_argument_print(void);
void print_bib_name(void);
void log_pr_bib_name(void);
void print_bst_name(void);
void log_pr_bst_name(void);
void hash_cite_confusion(void);
void zcheck_cite_overflow(cite_number last_cite);
#define check_cite_overflow(last_cite) zcheck_cite_overflow((cite_number) (last_cite))
void aux_end1_err_print(void);
void aux_end2_err_print(void);
void bst_ln_num_print(void);
void bst_err_print_and_look_for_blank_line(void);
void bst_warn_print(void);
void eat_bst_print(void);
void unknwn_function_class_confusion(void);
void zprint_fn_class(hash_loc fn_loc);
#define print_fn_class(fn_loc) zprint_fn_class((hash_loc) (fn_loc))
void ztrace_pr_fn_class(hash_loc fn_loc);
#define trace_pr_fn_class(fn_loc) ztrace_pr_fn_class((hash_loc) (fn_loc))
void id_scanning_confusion(void);
void bst_id_print(void);
void bst_left_brace_print(void);
void bst_right_brace_print(void);
void zalready_seen_function_print(hash_loc seen_fn_loc);
#define already_seen_function_print(seen_fn_loc) zalready_seen_function_print((hash_loc) (seen_fn_loc))
void bib_ln_num_print(void);
void bib_err_print(void);
void bib_warn_print(void);
void zcheck_field_overflow(integer total_fields);
#define check_field_overflow(total_fields) zcheck_field_overflow((integer) (total_fields))
void eat_bib_print(void);
void zbib_one_of_two_print(ASCII_code char1, ASCII_code char2);
#define bib_one_of_two_print(char1, char2) zbib_one_of_two_print((ASCII_code) (char1), (ASCII_code) (char2))
void bib_equals_sign_print(void);
void bib_unbalanced_braces_print(void);
void bib_field_too_long_print(void);
void macro_warn_print(void);
void bib_id_print(void);
void bib_cmd_confusion(void);
void cite_key_disappeared_confusion(void);
void zbad_cross_reference_print(str_number s);
#define bad_cross_reference_print(s) zbad_cross_reference_print((str_number) (s))
void nonexistent_cross_reference_error(void);
void zprint_missing_entry(str_number s);
#define print_missing_entry(s) zprint_missing_entry((str_number) (s))
void bst_ex_warn_print(void);
void bst_mild_ex_warn_print(void);
void bst_cant_mess_with_entries_print(void);
void illegl_literal_confusion(void);
void unknwn_literal_confusion(void);
void zprint_stk_lit(integer stk_lt, stk_type stk_tp);
#define print_stk_lit(stk_lt, stk_tp) zprint_stk_lit((integer) (stk_lt), (stk_type) (stk_tp))
void zprint_lit(integer stk_lt, stk_type stk_tp);
#define print_lit(stk_lt, stk_tp) zprint_lit((integer) (stk_lt), (stk_type) (stk_tp))
void output_bbl_line(void);
void bst_1print_string_size_exceeded(void);
void bst_2print_string_size_exceeded(void);
void zbraces_unbalanced_complaint(str_number pop_lit_var);
#define braces_unbalanced_complaint(pop_lit_var) zbraces_unbalanced_complaint((str_number) (pop_lit_var))
void case_conversion_confusion(void);
void trace_and_stat_printing(void);
void zstart_name(str_number file_name);
#define start_name(file_name) zstart_name((str_number) (file_name))
void zadd_extension(str_number ext);
#define add_extension(ext) zadd_extension((str_number) (ext))
str_number make_string(void);
boolean zstr_eq_buf(str_number s, buf_type buf, buf_pointer bf_ptr, buf_pointer len);
#define str_eq_buf(s, buf, bf_ptr, len) zstr_eq_buf((str_number) (s), (buf_type) (buf), (buf_pointer) (bf_ptr), (buf_pointer) (len))
boolean zstr_eq_str(str_number s1, str_number s2);
#define str_eq_str(s1, s2) zstr_eq_str((str_number) (s1), (str_number) (s2))
void zlower_case(buf_type buf, buf_pointer bf_ptr, buf_pointer len);
#define lower_case(buf, bf_ptr, len) zlower_case((buf_type) (buf), (buf_pointer) (bf_ptr), (buf_pointer) (len))
void zupper_case(buf_type buf, buf_pointer bf_ptr, buf_pointer len);
#define upper_case(buf, bf_ptr, len) zupper_case((buf_type) (buf), (buf_pointer) (bf_ptr), (buf_pointer) (len))
hash_loc zstr_lookup(buf_type buf, buf_pointer j, buf_pointer l, str_ilk ilk, boolean insert_it);
#define str_lookup(buf, j, l, ilk, insert_it) zstr_lookup((buf_type) (buf), (buf_pointer) (j), (buf_pointer) (l), (str_ilk) (ilk), (boolean) (insert_it))
void zpre_define(pds_type pds, pds_len len, str_ilk ilk);
#define pre_define(pds, len, ilk) zpre_define((pds_type) (pds), (pds_len) (len), (str_ilk) (ilk))
void zzint_to_ASCII(integer the_int, buf_type int_buf, buf_pointer int_begin, buf_pointer * int_end);
#define int_to_ASCII(the_int, int_buf, int_begin, int_end) zzint_to_ASCII((integer) (the_int), (buf_type) (int_buf), (buf_pointer) (int_begin), (buf_pointer *) &(int_end))
void zzadd_database_cite(cite_number * new_cite);
#define add_database_cite(new_cite) zzadd_database_cite((cite_number *) &(new_cite))
boolean zfind_cite_locs_for_this_cite_key(str_number cite_str);
#define find_cite_locs_for_this_cite_key(cite_str) zfind_cite_locs_for_this_cite_key((str_number) (cite_str))
void zswap(cite_number swap1, cite_number swap2);
#define swap(swap1, swap2) zswap((cite_number) (swap1), (cite_number) (swap2))
boolean zless_than(cite_number arg1, cite_number arg2);
#define less_than(arg1, arg2) zless_than((cite_number) (arg1), (cite_number) (arg2))
void zquick_sort(cite_number left_end, cite_number right_end);
#define quick_sort(left_end, right_end) zquick_sort((cite_number) (left_end), (cite_number) (right_end))
void zzbuild_in(pds_type pds, pds_len len, hash_loc * fn_hash_loc, blt_in_range blt_in_num);
#define build_in(pds, len, fn_hash_loc, blt_in_num) zzbuild_in((pds_type) (pds), (pds_len) (len), (hash_loc *) &(fn_hash_loc), (blt_in_range) (blt_in_num))
void pre_def_certain_strings(void);
boolean zscan1(ASCII_code char1);
#define scan1(char1) zscan1((ASCII_code) (char1))
boolean zscan1_white(ASCII_code char1);
#define scan1_white(char1) zscan1_white((ASCII_code) (char1))
boolean zscan2(ASCII_code char1, ASCII_code char2);
#define scan2(char1, char2) zscan2((ASCII_code) (char1), (ASCII_code) (char2))
boolean zscan2_white(ASCII_code char1, ASCII_code char2);
#define scan2_white(char1, char2) zscan2_white((ASCII_code) (char1), (ASCII_code) (char2))
boolean zscan3(ASCII_code char1, ASCII_code char2, ASCII_code char3);
#define scan3(char1, char2, char3) zscan3((ASCII_code) (char1), (ASCII_code) (char2), (ASCII_code) (char3))
boolean scan_alpha(void);
void zscan_identifier(ASCII_code char1, ASCII_code char2, ASCII_code char3);
#define scan_identifier(char1, char2, char3) zscan_identifier((ASCII_code) (char1), (ASCII_code) (char2), (ASCII_code) (char3))
boolean scan_nonneg_integer(void);
boolean scan_integer(void);
boolean scan_white_space(void);
boolean eat_bst_white_space(void);
void skip_token_print(void);
void print_recursion_illegal(void);
void skp_token_unknown_function_print(void);
void skip_illegal_stuff_after_token_print(void);
void zscan_fn_def(hash_loc fn_hash_loc);
#define scan_fn_def(fn_hash_loc) zscan_fn_def((hash_loc) (fn_hash_loc))
boolean eat_bib_white_space(void);
boolean compress_bib_white(void);
boolean scan_balanced_braces(void);
boolean scan_a_field_token_and_eat_white(void);
boolean scan_and_store_the_field_value_and_eat_white(void);
void zdecr_brace_level(str_number pop_lit_var);
#define decr_brace_level(pop_lit_var) zdecr_brace_level((str_number) (pop_lit_var))
void zcheck_brace_level(str_number pop_lit_var);
#define check_brace_level(pop_lit_var) zcheck_brace_level((str_number) (pop_lit_var))
void zname_scan_for_and(str_number pop_lit_var);
#define name_scan_for_and(pop_lit_var) zname_scan_for_and((str_number) (pop_lit_var))
boolean von_token_found(void);
void von_name_ends_and_last_name_starts_stuff(void);
void skip_stuff_at_sp_brace_level_greater_than_one(void);
void brace_lvl_one_letters_complaint(void);
boolean zenough_text_chars(buf_pointer enough_chars);
#define enough_text_chars(enough_chars) zenough_text_chars((buf_pointer) (enough_chars))
void figure_out_the_formatted_name(void);
void zpush_lit_stk(integer push_lt, stk_type push_type);
#define push_lit_stk(push_lt, push_type) zpush_lit_stk((integer) (push_lt), (stk_type) (push_type))
void zzpop_lit_stk(integer * pop_lit, stk_type * pop_type);
#define pop_lit_stk(pop_lit, pop_type) zzpop_lit_stk((integer *) &(pop_lit), (stk_type *) &(pop_type))
void zprint_wrong_stk_lit(integer stk_lt, stk_type stk_tp1, stk_type stk_tp2);
#define print_wrong_stk_lit(stk_lt, stk_tp1, stk_tp2) zprint_wrong_stk_lit((integer) (stk_lt), (stk_type) (stk_tp1), (stk_type) (stk_tp2))
void pop_top_and_print(void);
void pop_whole_stack(void);
void init_command_execution(void);
void check_command_execution(void);
void add_pool_buf_and_push(void);
void zadd_buf_pool(str_number p_str);
#define add_buf_pool(p_str) zadd_buf_pool((str_number) (p_str))
void zadd_out_pool(str_number p_str);
#define add_out_pool(p_str) zadd_out_pool((str_number) (p_str))
void x_equals(void);
void x_greater_than(void);
void x_less_than(void);
void x_plus(void);
void x_minus(void);
void x_concatenate(void);
void x_gets(void);
void x_add_period(void);
void x_change_case(void);
void x_chr_to_int(void);
void x_cite(void);
void x_duplicate(void);
void x_empty(void);
void x_format_name(void);
void x_int_to_chr(void);
void x_int_to_str(void);
void x_missing(void);
void x_num_names(void);
void x_preamble(void);
void x_purify(void);
void x_quote(void);
void x_substring(void);
void x_swap(void);
void x_text_length(void);
void x_text_prefix(void);
void x_type(void);
void x_warning(void);
void x_width(void);
void x_write(void);
void zexecute_fn(hash_loc ex_fn_loc);
#define execute_fn(ex_fn_loc) zexecute_fn((hash_loc) (ex_fn_loc))
void get_the_top_level_aux_file_name(void);
void aux_bib_data_command(void);
void aux_bib_style_command(void);
void aux_citation_command(void);
void aux_input_command(void);
void pop_the_aux_stack(void);
void get_aux_command_and_process(void);
void last_check_for_aux_errors(void);
void bst_entry_command(void);
boolean bad_argument_token(void);
void bst_execute_command(void);
void bst_function_command(void);
void bst_integers_command(void);
void bst_iterate_command(void);
void bst_macro_command(void);
void get_bib_command_or_entry_and_process(void);
void bst_read_command(void);
void bst_reverse_command(void);
void bst_sort_command(void);
void bst_strings_command(void);
void get_bst_command_and_process(void);
void setup_params(void);
void compute_hash_prime(void);
void initialize(void);
void parse_arguments(void);
