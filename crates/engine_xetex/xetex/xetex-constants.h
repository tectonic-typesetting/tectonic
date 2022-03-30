/* tectonic/xetex-constants.h: symbolic constants from the WEB code
   Copyright 2017-2021 the Tectonic Project
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

/* Various buffer sizes not fixed in xetex_format.h */

#define NUMBER_MATH_FAMILIES 256
#define NUMBER_MATH_FONTS (3 * NUMBER_MATH_FAMILIES)

#define NUMBER_REGS 256

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

/* equivalents table locations not detailed in xetex_format.h */

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

#define FONT_ID_BASE FROZEN_NULL_FONT /* nominally minus FONT_BASE, but that's 0 */

#define GLUEPAR(p) (eqtb[GLUE_BASE + GLUE_PAR__##p].b32.s1)
#define SKIP_REG(n) (eqtb[SKIP_BASE + (n)].b32.s1)
#define MU_SKIP_REG(n) (eqtb[MU_SKIP_BASE + (n)].b32.s1)
#define LOCAL(p) (eqtb[LOCAL_BASE + LOCAL__##p].b32.s1)
#define TOKS_REG(n) (eqtb[TOKS_BASE + (n)].b32.s1)
#define ETEX_PENALTIES_PAR(p) (eqtb[ETEX_PEN_BASE + ETEX_PENALTIES_PAR__##p].b32.s1)
#define BOX_REG(n) (eqtb[BOX_BASE + (n)].b32.s1)
#define MATH_FONT(n) (eqtb[MATH_FONT_BASE + (n)].b32.s1)
#define CAT_CODE(n) (eqtb[CAT_CODE_BASE + (n)].b32.s1)
#define LC_CODE(n) (eqtb[LC_CODE_BASE + (n)].b32.s1)
#define UC_CODE(n) (eqtb[UC_CODE_BASE + (n)].b32.s1)
#define SF_CODE(n) (eqtb[SF_CODE_BASE + (n)].b32.s1)
#define MATH_CODE(n) (eqtb[MATH_CODE_BASE + (n)].b32.s1)
#define CHAR_SUB_CODE(n) (eqtb[CHAR_SUB_CODE_BASE + (n)].b32.s1)
#define INTPAR(n) (eqtb[INT_BASE + INT_PAR__##n].b32.s1)
#define COUNT_REG(n) (eqtb[COUNT_BASE + (n)].b32.s1)
#define DEL_CODE(n) (eqtb[DEL_CODE_BASE + (n)].b32.s1)
#define DIMENPAR(n) (eqtb[DIMEN_BASE + DIMEN_PAR__##n].b32.s1)
#define SCALED_REG(n) (eqtb[SCALED_BASE + (n)].b32.s1)

#define LEVEL_ZERO 0 /* "really" MIN_QUARTERWORD */
#define LEVEL_ONE 1

/* SET_INTERACTION */
#define UNSPECIFIED_MODE 4

#define LEFT_TO_RIGHT 0
#define RIGHT_TO_LEFT 1

/* How many memory words are needed for storing synctex information on various
 * kinds of nodes. This extra size is already included in the *_NODE_SIZE
 * definitions below.
 */
#define SYNCTEX_FIELD_SIZE 1

#define DELTA_NODE RULE_NODE
#define EDGE_NODE STYLE_NODE

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

/* MATH_NODE L/R typesetting extras */
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

/* ABOVE */
#define DELIMITED_CODE 3

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
#define FIXED_ACC 1
#define HYPHENATED 1
#define IF_CODE 1
#define JUST_OPEN 1
#define MATH_CHAR 1
#define PRIM_BASE 1
#define RESTORE_ZERO 1
#define REVERSED 1
#define SLANT_CODE 1
#define SPLIT_UP 1
#define STRETCHING 1
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
#define SUB_MLIST 3
#define MU_VAL 3
#define IDENT_VAL 4
#define MATH_TEXT_CHAR 4
#define RESTORE_SA 4
#define SPACE_SHRINK_CODE 4
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
#define SUBSCRIPTTOPMAX 9
#define NATIVE_GLYPH_INFO_SIZE 10
#define CARRIAGE_RETURN 13
#define SUPERSCRIPTBOTTOMMIN 13
#define TOTAL_MATHEX_PARAMS 13
#define HI_MEM_STAT_USAGE 15
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
#define MAX_COMMAND 102
#define DIMEN_VAL_LIMIT 128
#define BIGGEST_LANG 255
#define MU_VAL_LIMIT 256
#define TOO_BIG_LANG 256
#define BOX_VAL_LIMIT 320
#define TOK_VAL_LIMIT 384
#define PRIM_PRIME 431
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
