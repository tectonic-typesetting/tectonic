/* tectonic/xetex-constants.h: symbolic constants from the WEB code
   Copyright 2017-2018 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_CONSTANTS_H
#define TECTONIC_CONSTANTS_H

/* Limits of various built-in types */

#define MIN_HALFWORD -0x0FFFFFFF
#define MAX_HALFWORD  0x3FFFFFFF

#define TEX_NULL     MIN_HALFWORD /* a null "pointer" */
#define TEX_INFINITY 0x7FFFFFFF /* "the largest positive value that TeX knows" */
#define NULL_FLAG   -0x40000000 /* "signifies a missing item" in rule nodes */
#define DEFAULT_CODE 0x40000000 /* "denotes default_rule_thickness" */

/* characters
 *
 * TeX thinks there are only 256 character but we know better. We use UTF16
 * codepoints. Actual Unicode character codes can exceed this, up to
 * BIGGEST_USV. "USV" here means Unicode Scalar Value. */

#define BIGGEST_CHAR 0xFFFF /* must be <= max_quarterword */
#define BIGGEST_USV  0x10FFFF
#define NUMBER_USVS  (BIGGEST_USV + 1)
#define TOO_BIG_USV  (BIGGEST_USV + 1)

/* Various buffer sizes */

#define HASH_SIZE 15000 /* max number of control sequences */
#define HASH_PRIME 8501 /* "a prime number equal to about 85% of hash_size" */

#define MAX_FONT_MAX 9000

#define NUMBER_MATH_FAMILIES 256
#define TEXT_SIZE 0
#define SCRIPT_SIZE NUMBER_MATH_FAMILIES
#define SCRIPT_SCRIPT_SIZE (2 * NUMBER_MATH_FAMILIES)
#define NUMBER_MATH_FONTS (3 * NUMBER_MATH_FAMILIES)

#define NUMBER_REGS 256

/* the size of our main "mem" array, minus 1; classically this is
 * configurable, but we hardcode it. */
#define MEM_TOP 4999999

/* fixed locations in the "mem" array */
#define PAGE_INS_HEAD MEM_TOP
#define CONTRIB_HEAD (MEM_TOP - 1)
#define PAGE_HEAD (MEM_TOP - 2)
#define TEMP_HEAD (MEM_TOP - 3)
#define HOLD_HEAD (MEM_TOP - 4)
#define ADJUST_HEAD (MEM_TOP - 5)
#define ACTIVE_LIST (MEM_TOP - 7) /* note: two words */
#define ALIGN_HEAD (MEM_TOP - 8)
#define END_SPAN (MEM_TOP - 9)
#define OMIT_TEMPLATE (MEM_TOP - 10)
#define NULL_LIST (MEM_TOP - 11)
#define LIG_TRICK (MEM_TOP - 12)
#define GARBAGE (MEM_TOP - 12) /* note: same as LIG_TRICK */
#define BACKUP_HEAD (MEM_TOP - 13)
#define PRE_ADJUST_HEAD (MEM_TOP - 14)

/* equivalents table offsets */

#define ACTIVE_BASE 1 /* "region 1": active character equivalents */
#define SINGLE_BASE (ACTIVE_BASE + NUMBER_USVS)
#define NULL_CS (SINGLE_BASE + NUMBER_USVS)
#define HASH_BASE (NULL_CS + 1) /* "region 2": hash table */
#define FROZEN_CONTROL_SEQUENCE (HASH_BASE + HASH_SIZE)
#define FROZEN_PROTECTION (FROZEN_CONTROL_SEQUENCE + 0)
#define FROZEN_CR (FROZEN_CONTROL_SEQUENCE + 1)
#define FROZEN_END_GROUP (FROZEN_CONTROL_SEQUENCE + 2)
#define FROZEN_RIGHT (FROZEN_CONTROL_SEQUENCE + 3)
#define FROZEN_FI (FROZEN_CONTROL_SEQUENCE + 4)
#define FROZEN_END_TEMPLATE (FROZEN_CONTROL_SEQUENCE + 5)
#define FROZEN_ENDV (FROZEN_CONTROL_SEQUENCE + 6)
#define FROZEN_RELAX (FROZEN_CONTROL_SEQUENCE + 7)
#define END_WRITE (FROZEN_CONTROL_SEQUENCE + 8)
#define FROZEN_DONT_EXPAND (FROZEN_CONTROL_SEQUENCE + 9)
#define FROZEN_SPECIAL (FROZEN_CONTROL_SEQUENCE + 10)
#define FROZEN_PRIMITIVE (FROZEN_CONTROL_SEQUENCE + 11)
#define PRIM_SIZE 500
#define FROZEN_NULL_FONT (FROZEN_CONTROL_SEQUENCE + 12 + PRIM_SIZE)
#define FONT_ID_BASE FROZEN_NULL_FONT /* nominally minus FONT_BASE, but that's 0 */
#define UNDEFINED_CONTROL_SEQUENCE (FROZEN_NULL_FONT + MAX_FONT_MAX + 1)
#define PRIM_EQTB_BASE (FROZEN_PRIMITIVE + 1)

#define GLUE_BASE (UNDEFINED_CONTROL_SEQUENCE + 1) /* "region 3": glue values */

#define GLUE_PAR__line_skip 0
#define GLUE_PAR__baseline_skip 1
#define GLUE_PAR__par_skip 2
#define GLUE_PAR__above_display_skip 3
#define GLUE_PAR__below_display_skip 4
#define GLUE_PAR__above_display_short_skip 5
#define GLUE_PAR__below_display_short_skip 6
#define GLUE_PAR__left_skip 7
#define GLUE_PAR__right_skip 8
#define GLUE_PAR__top_skip 9
#define GLUE_PAR__split_top_skip 10
#define GLUE_PAR__tab_skip 11
#define GLUE_PAR__space_skip 12
#define GLUE_PAR__xspace_skip 13
#define GLUE_PAR__par_fill_skip 14
#define GLUE_PAR__xetex_linebreak_skip 15
#define GLUE_PAR__thin_mu_skip 16
#define GLUE_PAR__med_mu_skip 17
#define GLUE_PAR__thick_mu_skip 18
#define GLUE_PARS 19

#define GLUEPAR(X) (eqtb[GLUE_BASE + GLUE_PAR__##X].b32.s1)

#define SKIP_BASE (GLUE_BASE + GLUE_PARS)
#define SKIP_REG(n) (eqtb[SKIP_BASE + (n)].b32.s1)

#define MU_SKIP_BASE (SKIP_BASE + NUMBER_REGS)
#define MU_SKIP_REG(n) (eqtb[MU_SKIP_BASE + (n)].b32.s1)

/* "region 4": local halfword values like baselineskip. Some of these are
 * used as arguments to ASSIGN_TOKS, SET_SHAPE, etc. */

#define LOCAL_BASE (MU_SKIP_BASE + NUMBER_REGS)
#define LOCAL__par_shape 0
#define LOCAL__output_routine 1
#define LOCAL__every_par 2
#define LOCAL__every_math 3
#define LOCAL__every_display 4
#define LOCAL__every_hbox 5
#define LOCAL__every_vbox 6
#define LOCAL__every_job 7
#define LOCAL__every_cr 8
#define LOCAL__err_help 9
#define LOCAL__every_eof 10
#define LOCAL__xetex_inter_char 11
#define LOCAL__TectonicCodaTokens 12
#define NUM_LOCALS 13
#define LOCAL(n) (eqtb[LOCAL_BASE + LOCAL__##n].b32.s1)

#define TOKS_BASE (LOCAL_BASE + NUM_LOCALS)
#define TOKS_REG(n) (eqtb[TOKS_BASE + (n)].b32.s1)

#define ETEX_PEN_BASE (TOKS_BASE + NUMBER_REGS)
#define INTER_LINE_PENALTIES_LOC (ETEX_PEN_BASE + 0)
#define CLUB_PENALTIES_LOC (ETEX_PEN_BASE + 1)
#define WIDOW_PENALTIES_LOC (ETEX_PEN_BASE + 2)
#define DISPLAY_WIDOW_PENALTIES_LOC (ETEX_PEN_BASE + 3)
#define ETEX_PENS (ETEX_PEN_BASE + 4)

#define BOX_BASE ETEX_PENS
#define BOX_REG(n) (eqtb[BOX_BASE + (n)].b32.s1)

#define CUR_FONT_LOC (BOX_BASE + NUMBER_REGS)
#define MATH_FONT_BASE (CUR_FONT_LOC + 1)
#define MATH_FONT(n) (eqtb[MATH_FONT_BASE + (n)].b32.s1)

#define CAT_CODE_BASE (MATH_FONT_BASE + NUMBER_MATH_FONTS)
#define CAT_CODE(n) (eqtb[CAT_CODE_BASE + (n)].b32.s1)

#define LC_CODE_BASE (CAT_CODE_BASE + NUMBER_USVS)
#define LC_CODE(n) (eqtb[LC_CODE_BASE + (n)].b32.s1)

#define UC_CODE_BASE (LC_CODE_BASE + NUMBER_USVS)
#define UC_CODE(n) (eqtb[UC_CODE_BASE + (n)].b32.s1)

#define SF_CODE_BASE (UC_CODE_BASE + NUMBER_USVS)
#define SF_CODE(n) (eqtb[SF_CODE_BASE + (n)].b32.s1)

#define MATH_CODE_BASE (SF_CODE_BASE + NUMBER_USVS)
#define MATH_CODE(n) (eqtb[MATH_CODE_BASE + (n)].b32.s1)

#define CHAR_SUB_CODE_BASE (MATH_CODE_BASE + NUMBER_USVS)
#define CHAR_SUB_CODE(n) (eqtb[CHAR_SUB_CODE_BASE + (n)].b32.s1)

/* "region 5": current fullword integers like hyphenation penalty */

#define INT_BASE (CHAR_SUB_CODE_BASE + NUMBER_USVS)

#define INT_PAR__pretolerance 0
#define INT_PAR__tolerance 1
#define INT_PAR__line_penalty 2
#define INT_PAR__hyphen_penalty 3
#define INT_PAR__ex_hyphen_penalty 4
#define INT_PAR__club_penalty 5
#define INT_PAR__widow_penalty 6
#define INT_PAR__display_widow_penalty 7
#define INT_PAR__broken_penalty 8
#define INT_PAR__bin_op_penalty 9
#define INT_PAR__rel_penalty 10
#define INT_PAR__pre_display_penalty 11
#define INT_PAR__post_display_penalty 12
#define INT_PAR__inter_line_penalty 13
#define INT_PAR__double_hyphen_demerits 14
#define INT_PAR__final_hyphen_demerits 15
#define INT_PAR__adj_demerits 16
#define INT_PAR__mag 17
#define INT_PAR__delimiter_factor 18
#define INT_PAR__looseness 19
#define INT_PAR__time 20
#define INT_PAR__day 21
#define INT_PAR__month 22
#define INT_PAR__year 23
#define INT_PAR__show_box_breadth 24
#define INT_PAR__show_box_depth 25
#define INT_PAR__hbadness 26
#define INT_PAR__vbadness 27
#define INT_PAR__pausing 28
#define INT_PAR__tracing_online 29
#define INT_PAR__tracing_macros 30
#define INT_PAR__tracing_stats 31
#define INT_PAR__tracing_paragraphs 32
#define INT_PAR__tracing_pages 33
#define INT_PAR__tracing_output 34
#define INT_PAR__tracing_lost_chars 35
#define INT_PAR__tracing_commands 36
#define INT_PAR__tracing_restores 37
#define INT_PAR__uc_hyph 38
#define INT_PAR__output_penalty 39
#define INT_PAR__max_dead_cycles 40
#define INT_PAR__hang_after 41
#define INT_PAR__floating_penalty 42
#define INT_PAR__global_defs 43
#define INT_PAR__cur_fam 44
#define INT_PAR__escape_char 45
#define INT_PAR__default_hyphen_char 46
#define INT_PAR__default_skew_char 47
#define INT_PAR__end_line_char 48
#define INT_PAR__new_line_char 49
#define INT_PAR__language 50
#define INT_PAR__left_hyphen_min 51
#define INT_PAR__right_hyphen_min 52
#define INT_PAR__holding_inserts 53
#define INT_PAR__error_context_lines 54
#define INT_PAR__char_sub_def_min 55 /* = TEX_INT_PARS = WEB2C_INT_BASE */
#define INT_PAR__char_sub_def_max 56
#define INT_PAR__tracing_char_sub_def 57
#define INT_PAR__tracing_assigns 58 /* = WEB2C_INT_PARS = ETEX_INT_BASE */
#define INT_PAR__tracing_groups 59
#define INT_PAR__tracing_ifs 60
#define INT_PAR__tracing_scan_tokens 61
#define INT_PAR__tracing_nesting 62
#define INT_PAR__pre_display_correction 63
#define INT_PAR__last_line_fit 64
#define INT_PAR__saving_vdiscards 65
#define INT_PAR__saving_hyphs 66
#define INT_PAR__suppress_fontnotfound_error 67
#define INT_PAR__xetex_linebreak_locale 68
#define INT_PAR__xetex_linebreak_penalty 69
#define INT_PAR__xetex_protrude_chars 70
#define INT_PAR__texxet 71
#define INT_PAR__xetex_dash_break 72
#define INT_PAR__xetex_upwards 73
#define INT_PAR__xetex_use_glyph_metrics 74
#define INT_PAR__xetex_inter_char_tokens 75
#define INT_PAR__xetex_input_normalization 76
#define INT_PAR__xetex_default_input_mode 77
#define INT_PAR__xetex_default_input_encoding 78
#define INT_PAR__xetex_tracing_fonts 79
#define INT_PAR__xetex_interword_space_shaping 80
#define INT_PAR__xetex_generate_actual_text 81
#define INT_PAR__xetex_hyphenatable_length 82
#define INT_PAR__synctex 83
#define INT_PAR__pdfoutput 84
#define INT_PARS 85

#define INTPAR(x) (eqtb[INT_BASE + INT_PAR__##x].b32.s1)

#define COUNT_BASE (INT_BASE + INT_PARS)
#define COUNT_REG(n) (eqtb[COUNT_BASE + (n)].b32.s1)

#define DEL_CODE_BASE (COUNT_BASE + NUMBER_REGS)
#define DEL_CODE(n) (eqtb[DEL_CODE_BASE + (n)].b32.s1)

/* "region 6": current fullword dimensions like hsize */

#define DIMEN_BASE (DEL_CODE_BASE + NUMBER_USVS)

#define DIMEN_PAR__par_indent 0
#define DIMEN_PAR__math_surround 1
#define DIMEN_PAR__line_skip_limit 2
#define DIMEN_PAR__hsize 3
#define DIMEN_PAR__vsize 4
#define DIMEN_PAR__max_depth 5
#define DIMEN_PAR__split_max_depth 6
#define DIMEN_PAR__box_max_depth 7
#define DIMEN_PAR__hfuzz 8
#define DIMEN_PAR__vfuzz 9
#define DIMEN_PAR__delimiter_shortfall 10
#define DIMEN_PAR__null_delimiter_space 11
#define DIMEN_PAR__script_space 12
#define DIMEN_PAR__pre_display_size 13
#define DIMEN_PAR__display_width 14
#define DIMEN_PAR__display_indent 15
#define DIMEN_PAR__overfull_rule 16
#define DIMEN_PAR__hang_indent 17
#define DIMEN_PAR__h_offset 18
#define DIMEN_PAR__v_offset 19
#define DIMEN_PAR__emergency_stretch 20
#define DIMEN_PAR__pdf_page_width 21
#define DIMEN_PAR__pdf_page_height 22
#define DIMEN_PARS 23

#define DIMENPAR(x) (eqtb[DIMEN_BASE + DIMEN_PAR__##x].b32.s1)

#define SCALED_BASE (DIMEN_BASE + DIMEN_PARS)
#define SCALED_REG(n) (eqtb[SCALED_BASE + (n)].b32.s1)

#define EQTB_SIZE (SCALED_BASE + NUMBER_REGS - 1)

#define LEVEL_ZERO 0 /* "really" MIN_QUARTERWORD */
#define LEVEL_ONE 1


/* SET_INTERACTION */
#define BATCH_MODE 0
#define NONSTOP_MODE 1
#define SCROLL_MODE 2
#define ERROR_STOP_MODE 3
#define UNSPECIFIED_MODE 4

#define LEFT_TO_RIGHT 0
#define RIGHT_TO_LEFT 1

/* How many memory words are needed for storing synctex information on various
 * kinds of nodes. This extra size is already included in the *_NODE_SIZE
 * definitions below.
 */
#define SYNCTEX_FIELD_SIZE 1

#define HLIST_NODE 0
#define VLIST_NODE 1
#define DELTA_NODE 2
#define RULE_NODE 2
#define INS_NODE 3
#define MARK_NODE 4
#define ADJUST_NODE 5
#define LIGATURE_NODE 6
#define DISC_NODE 7
#define WHATSIT_NODE 8
#define MATH_NODE 9
#define GLUE_NODE 10
#define KERN_NODE 11
#define PENALTY_NODE 12
#define UNSET_NODE 13
#define EDGE_NODE 14
#define STYLE_NODE 14
#define CHOICE_NODE 15
#define MARGIN_KERN_NODE 40
#define NATIVE_WORD_NODE 40
#define NATIVE_WORD_NODE_AT 41
#define GLYPH_NODE 42 /* not to be confused with GLYPH_CODE = 43! */
#define PIC_NODE 43 /* not to be confused with PIC_FILE_CODE = 41! */
#define PDF_NODE 44 /* not to be confused with PDF_FILE_CODE = 42! */

#define IF_NODE_SIZE 2
#define PASSIVE_NODE_SIZE 2
#define POINTER_NODE_SIZE 2
#define SMALL_NODE_SIZE 2
#define SPAN_NODE_SIZE 2
#define WRITE_NODE_SIZE 2
#define ACTIVE_NODE_SIZE_NORMAL 3
#define EDGE_NODE_SIZE 3
#define MARGIN_KERN_NODE_SIZE 3
#define MEDIUM_NODE_SIZE 3
#define MOVEMENT_NODE_SIZE 3
#define OPEN_NODE_SIZE 3
#define STYLE_NODE_SIZE 3
#define WORD_NODE_SIZE 3
#define EXPR_NODE_SIZE 4
#define GLUE_SPEC_SIZE 4
#define MARK_CLASS_NODE_SIZE 4
#define PAGE_INS_NODE_SIZE 4
#define ACTIVE_NODE_SIZE_EXTENDED 5
#define GLYPH_NODE_SIZE 5
#define INS_NODE_SIZE 5
#define RULE_NODE_SIZE 5
#define ALIGN_STACK_NODE_SIZE 6
#define NATIVE_NODE_SIZE 6
#define DELTA_NODE_SIZE 7
#define BOX_NODE_SIZE 8
#define PIC_NODE_SIZE 9
#define INDEX_NODE_SIZE 33

#define NOAD_SIZE 4
#define ACCENT_NOAD_SIZE 5
#define RADICAL_NOAD_SIZE 5
#define FRACTION_NOAD_SIZE 6

/* MATH_COMP and others */
#define ORD_NOAD 16
#define OP_NOAD 17
#define BIN_NOAD 18
#define REL_NOAD 19
#define OPEN_NOAD 20
#define CLOSE_NOAD 21
#define PUNCT_NOAD 22
#define INNER_NOAD 23
#define RADICAL_NOAD 24
#define FRACTION_NOAD 25
#define UNDER_NOAD 26
#define OVER_NOAD 27
#define ACCENT_NOAD 28
#define VCENTER_NOAD 29
#define LEFT_NOAD 30
#define RIGHT_NOAD 31

/* args to TOP_BOT_MARK */
#define TOP_MARK_CODE 0
#define FIRST_MARK_CODE 1
#define BOT_MARK_CODE 2
#define SPLIT_FIRST_MARK_CODE 3
#define SPLIT_BOT_MARK_CODE 4

/* MATH_NODE stuff with L/R typesetting extras */
#define BEFORE 0
#define AFTER 1
#define BEGIN_M_CODE 2
#define END_M_CODE 3
#define L_CODE 4
#define R_CODE 8

#define EXPR_NONE 0
#define EXPR_ADD 1
#define EXPR_SUB 2
#define EXPR_MULT 3
#define EXPR_DIV 4
#define EXPR_SCALE 5

#define BOTTOM_LEVEL 0
#define SIMPLE_GROUP 1
#define HBOX_GROUP 2
#define ADJUSTED_HBOX_GROUP 3
#define VBOX_GROUP 4
#define VTOP_GROUP 5
#define ALIGN_GROUP 6
#define NO_ALIGN_GROUP 7
#define OUTPUT_GROUP 8
#define MATH_GROUP 9
#define DISC_GROUP 10
#define INSERT_GROUP 11
#define VCENTER_GROUP 12
#define MATH_CHOICE_GROUP 13
#define SEMI_SIMPLE_GROUP 14
#define MATH_SHIFT_GROUP 15
#define MATH_LEFT_GROUP 16

#define SUP_CMD 0
#define SUB_CMD 1

#define FIL 1
#define FILL 2
#define FILLL 3

#define LIG_TAG 1
#define LIST_TAG 2
#define EXT_TAG 3

/* scanner_status values: */
#define NORMAL 0
#define SKIPPING 1
#define DEFINING 2
#define MATCHING 3
#define ALIGNING 4
#define ABSORBING 5

/* commands */

#undef IGNORE /* Windows OS headers sometimes define this */

#define ESCAPE 0
#define RELAX 0 /* = ESCAPE */
#define LEFT_BRACE 1
#define RIGHT_BRACE 2
#define MATH_SHIFT 3
#define TAB_MARK 4
#define CAR_RET 5
#define OUT_PARAM 5 /* = CAR_RET */
#define MAC_PARAM 6
#define SUP_MARK 7
#define SUB_MARK 8
#define IGNORE 9
#define ENDV 9 /* = IGNORE */
#define SPACER 10
#define LETTER 11
#define OTHER_CHAR 12
#define ACTIVE_CHAR 13
#define PAR_END 13 /* = ACTIVE_CHAR */
#define MATCH 13 /* = ACTIVE_CHAR */
#define COMMENT 14
#define END_MATCH 14 /* = COMMENT */
#define STOP 14 /* = COMMENT */
#define INVALID_CHAR 15
#define DELIM_NUM 15 /* = INVALID_CHAR */
#define CHAR_NUM 16
#define MATH_CHAR_NUM 17
#define MARK 18
#define XRAY 19
#define MAKE_BOX 20
#define HMOVE 21
#define VMOVE 22
#define UN_HBOX 23
#define UN_VBOX 24
#define REMOVE_ITEM 25
#define HSKIP 26
#define VSKIP 27
#define MSKIP 28
#define KERN 29
#define MKERN 30
#define LEADER_SHIP 31
#define HALIGN 32
#define VALIGN 33
#define NO_ALIGN 34
#define VRULE 35
#define HRULE 36
#define INSERT 37
#define VADJUST 38
#define IGNORE_SPACES 39
#define AFTER_ASSIGNMENT 40
#define AFTER_GROUP 41
#define BREAK_PENALTY 42
#define START_PAR 43
#define ITAL_CORR 44
#define ACCENT 45
#define MATH_ACCENT 46
#define DISCRETIONARY 47
#define EQ_NO 48
#define LEFT_RIGHT 49
#define MATH_COMP 50
#define LIMIT_SWITCH 51
#define ABOVE 52
#define MATH_STYLE 53
#define MATH_CHOICE 54
#define NON_SCRIPT 55
#define VCENTER 56
#define CASE_SHIFT 57
#define MESSAGE 58
#define EXTENSION 59
#define IN_STREAM 60
#define BEGIN_GROUP 61
#define END_GROUP 62
#define OMIT 63
#define EX_SPACE 64
#define NO_BOUNDARY 65
#define RADICAL 66
#define END_CS_NAME 67
#define CHAR_GIVEN 68
#define MIN_INTERNAL 68
#define MATH_GIVEN 69
#define XETEX_MATH_GIVEN 70
#define LAST_ITEM 71
#define MAX_NON_PREFIXED_COMMAND 71
#define TOKS_REGISTER 72
#define ASSIGN_TOKS 73
#define ASSIGN_INT 74
#define ASSIGN_DIMEN 75
#define ASSIGN_GLUE 76
#define ASSIGN_MU_GLUE 77
#define ASSIGN_FONT_DIMEN 78
#define ASSIGN_FONT_INT 79
#define SET_AUX 80
#define SET_PREV_GRAF 81
#define SET_PAGE_DIMEN 82
#define SET_PAGE_INT 83
#define SET_BOX_DIMEN 84
#define SET_SHAPE 85
#define DEF_CODE 86
#define XETEX_DEF_CODE 87
#define DEF_FAMILY 88
#define SET_FONT 89
#define DEF_FONT 90
#define MAX_INTERNAL 91
#define REGISTER 91
#define ADVANCE 92
#define MULTIPLY 93
#define DIVIDE 94
#define PREFIX 95
#define LET 96
#define SHORTHAND_DEF 97
#define READ_TO_CS 98
#define DEF 99
#define SET_BOX 100
#define HYPH_DATA 101
#define SET_INTERACTION 102
#define EXPAND_AFTER 104
#define NO_EXPAND 105
#define INPUT 106
#define IF_TEST 107
#define FI_OR_ELSE 108
#define CS_NAME 109
#define CONVERT 110
#define THE 111
#define TOP_BOT_MARK 112

/* args to SET_BOX_DIMEN */
#define WIDTH_OFFSET 1
#define DEPTH_OFFSET 2
#define HEIGHT_OFFSET 3

/* args to LAST_ITEM -- heavily overloaded by (X)eTeX for extensions */
#define INT_VAL 0
#define DIMEN_VAL 1
#define GLUE_VAL 2
#define LAST_NODE_TYPE_CODE 3
#define INPUT_LINE_NO_CODE 4
#define BADNESS_CODE 5
#define PDF_LAST_X_POS_CODE 12
#define PDF_LAST_Y_POS_CODE 13
#define ELAPSED_TIME_CODE 16
#define PDF_SHELL_ESCAPE_CODE 17
#define RANDOM_SEED_CODE 18
#define ETEX_VERSION_CODE 19
#define CURRENT_GROUP_LEVEL_CODE 20
#define CURRENT_GROUP_TYPE_CODE 21
#define CURRENT_IF_LEVEL_CODE 22
#define CURRENT_IF_TYPE_CODE 23
#define CURRENT_IF_BRANCH_CODE 24
#define GLUE_STRETCH_ORDER_CODE 25
#define GLUE_SHRINK_ORDER_CODE 26
#define XETEX_INT 27 /* base number for XeTeX special integer codes */
#define XETEX_VERSION_CODE 27
#define XETEX_COUNT_GLYPHS_CODE 28
#define XETEX_COUNT_VARIATIONS_CODE 29
#define XETEX_VARIATION_CODE 30
#define XETEX_FIND_VARIATION_BY_NAME_CODE 31
#define XETEX_VARIATION_MIN_CODE 32
#define XETEX_VARIATION_MAX_CODE 33
#define XETEX_VARIATION_DEFAULT_CODE 34
#define XETEX_COUNT_FEATURES_CODE 35
#define XETEX_FEATURE_CODE_CODE 36
#define XETEX_FIND_FEATURE_BY_NAME_CODE 37
#define XETEX_IS_EXCLUSIVE_FEATURE_CODE 38
#define XETEX_COUNT_SELECTORS_CODE 39
#define XETEX_SELECTOR_CODE_CODE 40
#define XETEX_FIND_SELECTOR_BY_NAME_CODE 41
#define XETEX_IS_DEFAULT_SELECTOR_CODE 42
#define XETEX_OT_COUNT_SCRIPTS_CODE 43
#define XETEX_OT_COUNT_LANGUAGES_CODE 44
#define XETEX_OT_COUNT_FEATURES_CODE 45
#define XETEX_OT_SCRIPT_CODE 46
#define XETEX_OT_LANGUAGE_CODE 47
#define XETEX_OT_FEATURE_CODE 48
#define XETEX_MAP_CHAR_TO_GLYPH_CODE 49
#define XETEX_GLYPH_INDEX_CODE 50
#define XETEX_FONT_TYPE_CODE 51
#define XETEX_FIRST_CHAR_CODE 52
#define XETEX_LAST_CHAR_CODE 53
#define XETEX_PDF_PAGE_COUNT_CODE 54
#define XETEX_LAST_ITEM_CODES XETEX_PDF_PAGE_COUNT_CODE /*54*/
#define XETEX_DIM (XETEX_LAST_ITEM_CODES + 1) /*55*/
#define XETEX_GLYPH_BOUNDS_CODE (XETEX_DIM + 0) /*55*/
#define XETEX_LAST_DIM_CODES XETEX_GLYPH_BOUNDS_CODE /*55*/
#define ETEX_DIM (XETEX_LAST_DIM_CODES + 1) /*56*/
#define ETEX_GLUE (ETEX_DIM + 9) /*65*/
#define ETEX_MU (ETEX_GLUE + 1) /*66*/
#define FONT_CHAR_WD_CODE 56
#define FONT_CHAR_HT_CODE 57
#define FONT_CHAR_DP_CODE 58
#define FONT_CHAR_IC_CODE 59
#define PAR_SHAPE_LENGTH_CODE 60
#define PAR_SHAPE_INDENT_CODE 61
#define PAR_SHAPE_DIMEN_CODE 62
#define GLUE_STRETCH_CODE 63
#define GLUE_SHRINK_CODE 64
#define MU_TO_GLUE_CODE 65
#define GLUE_TO_MU_CODE 66
#define ETEX_EXPR 67 /* = ETEX_MU + 1 */

/* args to CONVERT -- also heavily overloaded */
#define NUMBER_CODE 0
#define ROMAN_NUMERAL_CODE 1
#define STRING_CODE 2
#define MEANING_CODE 3
#define FONT_NAME_CODE 4
#define ETEX_REVISION_CODE 5 /* = ETEX_CONVERT_BASE */
#define EXPANDED_CODE 6 /* = ETEX_CONVERT_CODES */
#define LEFT_MARGIN_KERN_CODE 16
#define RIGHT_MARGIN_KERN_CODE 17
#define PDF_STRCMP_CODE 18
#define PDF_CREATION_DATE_CODE 22
#define PDF_FILE_MOD_DATE_CODE 23
#define PDF_FILE_SIZE_CODE 24
#define PDF_MDFIVE_SUM_CODE 25
#define PDF_FILE_DUMP_CODE 26
#define UNIFORM_DEVIATE_CODE 29
#define NORMAL_DEVIATE_CODE 30
#define XETEX_VARIATION_NAME_CODE 32
#define XETEX_REVISION_CODE 33
#define XETEX_FEATURE_NAME_CODE 35
#define XETEX_SELECTOR_NAME_CODE 36
#define XETEX_GLYPH_NAME_CODE 37
#define XETEX_UCHAR_CODE 38
#define XETEX_UCHARCAT_CODE 39
#define JOB_NAME_CODE 40

/* args to IF_TEST */
#define IF_CHAR_CODE 0
#define IF_CODE 1
#define IF_CAT_CODE 1
#define IF_INT_CODE 2
#define IF_DIM_CODE 3
#define IF_ODD_CODE 4
#define IF_VMODE_CODE 5
#define IF_HMODE_CODE 6
#define IF_MMODE_CODE 7
#define IF_INNER_CODE 8
#define IF_VOID_CODE 9
#define IF_HBOX_CODE 10
#define IF_VBOX_CODE 11
#define IFX_CODE 12
#define IF_EOF_CODE 13
#define IF_TRUE_CODE 14
#define IF_FALSE_CODE 15
#define IF_CASE_CODE 16
#define IF_DEF_CODE 17
#define IF_CS_CODE 18
#define IF_FONT_CHAR_CODE 19
#define IF_IN_CSNAME_CODE 20
#define IF_PRIMITIVE_CODE 21

/* args to FI_OR_ELSE */
#define FI_CODE 2
#define ELSE_CODE 3
#define OR_CODE 4

/* special args for TAB_MARK, CAR_RET */
#define SPAN_CODE (BIGGEST_USV + 2)
#define CR_CODE (BIGGEST_USV + 3)
#define CR_CR_CODE (BIGGEST_USV + 4)

/* HSKIP, VSKIP, MSKIP */
#define FIL_CODE 0
#define FILL_CODE 1
#define SS_CODE 2
#define FIL_NEG_CODE 3
#define SKIP_CODE 4
#define MSKIP_CODE 5

/* MAKE_BOX, UN_HBOX, UN_VBOX */
#define BOX_CODE 0
#define COPY_CODE 1
#define LAST_BOX_CODE 2
#define VSPLIT_CODE 3
#define VTOP_CODE 4

/* LEADER_SHIP */
#define A_LEADERS 100
#define C_LEADERS 101
#define X_LEADERS 102

/* LIMIT_SWITCH */
/* also NORMAL = 0 */
#define LIMITS 1
#define NO_LIMITS 2

/* MATH_STYLE */
#define DISPLAY_STYLE 0
#define TEXT_STYLE 2
#define SCRIPT_STYLE 4
#define SCRIPT_SCRIPT_STYLE 6

/* ABOVE */
#define ABOVE_CODE 0
#define OVER_CODE 1
#define ATOP_CODE 2
#define DELIMITED_CODE 3

/* SHORTHAND_DEF */
#define CHAR_DEF_CODE 0
#define MATH_CHAR_DEF_CODE 1
#define COUNT_DEF_CODE 2
#define DIMEN_DEF_CODE 3
#define SKIP_DEF_CODE 4
#define MU_SKIP_DEF_CODE 5
#define TOKS_DEF_CODE 6
#define CHAR_SUB_DEF_CODE 7
#define XETEX_MATH_CHAR_NUM_DEF_CODE 8
#define XETEX_MATH_CHAR_DEF_CODE 9

/* XRAY */
#define SHOW_CODE 0
#define SHOW_BOX_CODE 1
#define SHOW_THE_CODE 2
#define SHOW_LISTS 3
#define SHOW_GROUPS 4
#define SHOW_TOKENS 5
#define SHOW_IFS 6

/* EXTENSION */
#define OPEN_NODE 0
#define WRITE_NODE 1
#define CLOSE_NODE 2
#define SPECIAL_NODE 3
#define LANGUAGE_NODE 4
#define IMMEDIATE_CODE 4
#define SET_LANGUAGE_CODE 5
#define PDFTEX_FIRST_EXTENSION_CODE 6
#define PDF_SAVE_POS_NODE (PDFTEX_FIRST_EXTENSION_CODE + 15)
#define RESET_TIMER_CODE (PDFTEX_FIRST_EXTENSION_CODE + 25)
#define SET_RANDOM_SEED_CODE (PDFTEX_FIRST_EXTENSION_CODE + 27)
#define PIC_FILE_CODE 41 /* not to be confused with PIC_NODE = 43! */
#define PDF_FILE_CODE 42 /* not to be confused with PDF_NODE = 44! */
#define GLYPH_CODE 43 /* not to be confused with GLYPH_NODE = 42! */
#define XETEX_INPUT_ENCODING_EXTENSION_CODE 44
#define XETEX_DEFAULT_ENCODING_EXTENSION_CODE 45
#define XETEX_LINEBREAK_LOCALE_EXTENSION_CODE 46

/* VALIGN overloads */
#define BEGIN_L_CODE 6
#define END_L_CODE 7
#define BEGIN_R_CODE 10
#define END_R_CODE 11

/* begin_token_list() types */
#define PARAMETER 0
#define U_TEMPLATE 1
#define V_TEMPLATE 2
#define BACKED_UP 3
#define BACKED_UP_CHAR 4
#define INSERTED 5
#define MACRO 6
#define OUTPUT_TEXT 7
#define EVERY_PAR_TEXT 8
#define EVERY_MATH_TEXT 9
#define EVERY_DISPLAY_TEXT 10
#define EVERY_HBOX_TEXT 11
#define EVERY_VBOX_TEXT 12
#define EVERY_JOB_TEXT 13
#define EVERY_CR_TEXT 14
#define MARK_TEXT 15
#define EVERY_EOF_TEXT 16
#define INTER_CHAR_TEXT 17
#define WRITE_TEXT 18
#define TECTONIC_CODA_TEXT 19

/* input state */
#define MID_LINE 1
#define SKIP_BLANKS 17
#define NEW_LINE 33

/* DVI format codes */
#define XDV_ID_BYTE 7
#define SPX_ID_BYTE 100

/* page_contents possibilities (EMPTY is overloaded) */
#define EMPTY 0
#define INSERTS_ONLY 1
#define BOX_THERE 2

#define SET1 128
#define SET_RULE 132
#define PUT_RULE 137
#define BOP 139
#define EOP 140
#define PUSH 141
#define POP 142
#define RIGHT1 143
#define DOWN1 157
#define FNT1 235
#define XXX1 239
#define XXX4 242
#define FNT_DEF1 243
#define PRE 247
#define POST 248
#define POST_POST 249
#define DEFINE_NATIVE_FONT 252
#define SET_GLYPHS 253
#define SET_TEXT_AND_GLYPHS 254


#define XETEX_INPUT_MODE_AUTO 0
#define XETEX_VERSION 0
#define EXACTLY 0
#define FONT_BASE 0
#define INSERTING 0
#define NON_ADDRESS 0
#define RESTORE_OLD_VALUE 0
#define TOKEN_LIST 0
#define UNDEFINED_PRIMITIVE 0
#define UNHYPHENATED 0
#define ADDITIONAL 1
#define EXPLICIT 1
#define FIXED_ACC 1
#define HYPHENATED 1
#define JUST_OPEN 1
#define MATH_CHAR 1
#define PRIM_BASE 1
#define RESTORE_ZERO 1
#define REVERSED 1
#define SLANT_CODE 1
#define SPLIT_UP 1
#define STRETCHING 1
#define VMODE 1
#define ACC_KERN 2
#define BOTTOM_ACC 2
#define CLOSED 2
#define DLIST 2
#define ETEX_VERSION 2
#define INSERT_TOKEN 2
#define SHRINKING 2
#define SPACE_CODE 2
#define SUB_BOX 2
#define DISPLAYOPERATORMINHEIGHT 3
#define LEVEL_BOUNDARY 3
#define MATH_SHIFT 3
#define SPACE_ADJUSTMENT 3
#define SUB_MLIST 3
#define MU_VAL 3
#define IDENT_VAL 4
#define MATH_TEXT_CHAR 4
#define RESTORE_SA 4
#define SPACE_SHRINK_CODE 4
#define OUT_PARAM 5
#define TOK_VAL 5
#define X_HEIGHT_CODE 5
#define ACCENTBASEHEIGHT 6
#define INTER_CHAR_VAL 6
#define MAC_PARAM 6
#define QUAD_CODE 6
#define EXTRA_SPACE_CODE 7
#define MARK_VAL 7
#define SUP_MARK 7
#define VAR_FAM_CLASS 7
#define IGNORE 9
#define SUBSCRIPTTOPMAX 9
#define NATIVE_GLYPH_INFO_SIZE 10
#define ACTIVE_CHAR 13
#define CARRIAGE_RETURN 13
#define SUPERSCRIPTBOTTOMMIN 13
#define TOTAL_MATHEX_PARAMS 13
#define COMMENT 14
#define HI_MEM_STAT_USAGE 15
#define INVALID_CHAR 15
#define MAX_CHAR_CODE 15
#define SUBSUPERSCRIPTGAPMIN 15
#define SUPERSCRIPTBOTTOMMAXWITHSUBSCRIPT 16
#define TOTAL_MATHSY_PARAMS 22
#define STACKGAPMIN 26
#define STACKDISPLAYSTYLEGAPMIN 27
#define UNLESS_CODE 32
#define VRULE 35
#define FRACTIONNUMERATORGAPMIN 36
#define FRACTIONNUMDISPLAYSTYLEGAPMIN 37
#define FRACTIONDENOMINATORGAPMIN 39
#define FRACTIONDENOMDISPLAYSTYLEGAPMIN 40
#define RADICALVERTICALGAP 49
#define RADICALDISPLAYSTYLEVERTICALGAP 50
#define RADICALRULETHICKNESS 51
#define COND_MATH_GLUE 98
#define MU_GLUE 99
#define MAX_COMMAND 102
#define UNDEFINED_CS 103
#define HMODE 104
#define CALL 113
#define LONG_CALL 114
#define OUTER_CALL 115
#define LONG_OUTER_CALL 116
#define END_TEMPLATE 117
#define DONT_EXPAND 118
#define GLUE_REF 119
#define SHAPE_REF 120
#define BOX_REF 121
#define DATA 122
#define DIMEN_VAL_LIMIT 128
#define MMODE 207
#define BIGGEST_LANG 255
#define MU_VAL_LIMIT 256
#define TOO_BIG_LANG 256
#define BOX_VAL_LIMIT 320
#define TOK_VAL_LIMIT 384
#define PRIM_PRIME 431
#define PRIM_SIZE 500
#define MAX_HLIST_STACK 512
#define HYPH_PRIME 607
#define HYPHENATABLE_LENGTH_LIMIT 4095
#define CHAR_CLASS_LIMIT 4096
#define EJECT_PENALTY -10000
#define INF_BAD 10000
#define INF_PENALTY 10000
#define DEFAULT_RULE 26214
#define TOO_BIG_CHAR 65536
#define NO_EXPAND_FLAG (BIGGEST_USV + 2)

#define ACTIVE_MATH_CHAR 0x1FFFFF

/* Token codes */

#define MAX_CHAR_VAL 0x200000 /* 1 << 21 */
#define CS_TOKEN_FLAG 0x1FFFFFF
#define LEFT_BRACE_TOKEN 0x200000 /* LEFT_BRACE << 21 */
#define LEFT_BRACE_LIMIT 0x400000 /* (LEFT_BRACE + 1) << 21 */
#define RIGHT_BRACE_TOKEN 0x400000 /* RIGHT_BRACE << 21 */
#define RIGHT_BRACE_LIMIT 0x600000 /* (RIGHT_BRACE + 1) << 21 */
#define MATH_SHIFT_TOKEN 0x600000 /* MATH_SHIFT << 21 */
#define TAB_TOKEN 0x800000 /* TAB_MARK << 21 */
#define OUT_PARAM_TOKEN 0xA00000 /* OUT_PARAM << 21 */
#define SPACE_TOKEN 0x1400020 /* SPACER << 21 + ord(' ') */
#define LETTER_TOKEN 0x1600000 /* LETTER << 21 */
#define OTHER_TOKEN 0x1800000 /* OTHER_CHAR << 21 */
#define MATCH_TOKEN 0x1A00000 /* MATCH << 21 */
#define END_MATCH_TOKEN 0x1C00000 /* END_MATCH << 21 */
#define PROTECTED_TOKEN (END_MATCH_TOKEN + 1)

#define A_TOKEN (LETTER_TOKEN + 'A')
#define OTHER_A_TOKEN (OTHER_TOKEN + 'A')
#define HEX_TOKEN (OTHER_TOKEN + '"')
#define OCTAL_TOKEN (OTHER_TOKEN + '\'')
#define CONTINENTAL_POINT_TOKEN (OTHER_TOKEN + ',')
#define POINT_TOKEN (OTHER_TOKEN + '.')
#define ZERO_TOKEN (OTHER_TOKEN + '0')
#define ALPHA_TOKEN (OTHER_TOKEN + '`')

#define BOX_FLAG 0x40000000
#define GLOBAL_BOX_FLAG 0x40008000
#define SHIP_OUT_FLAG 0x40010000
#define LEADER_FLAG 0x40010001

#define LP_CODE_BASE 2
#define RP_CODE_BASE 3

/* modes to do_marks() */
#define VSPLIT_INIT 0
#define FIRE_UP_INIT 1
#define FIRE_UP_DONE 2
#define DESTROY_MARKS 3

#define MARKS_CODE 5


#define IGNORE_DEPTH -65536000

#define MIDDLE_NOAD 1

/* movement() */
#define MOV_NONE_SEEN 0
#define MOV_Y_HERE 1
#define MOV_Z_HERE 2
#define MOV_YZ_OK 3
#define MOV_Y_OK 4
#define MOV_Z_OK 5
#define MOV_Y_SEEN 6
#define MOV_D_FIXED 6
#define MOV_Z_SEEN 12

#endif /* not TECTONIC_CONSTANTS_H */
