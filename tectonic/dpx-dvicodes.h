/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/

#ifndef _DVICODES_H_
#define _DVICODES_H_

#include "tectonic_bridge_core.h"

/* DVI op codes */
#define SET_CHAR_0 0
#define SET_CHAR_1 1
/* etc. */
#define SET_CHAR_127 127
#define SET1   128 /* Typesets its single operand between 128 and 255 */
#define SET2   129 /* Typesets its single two byte unsigned operand */
#define SET3   130 /* Typesets its single three byte unsigned operand */
#define SET4   131 /* Typesets its single four byte signed operand */
#define SET_RULE 132 /* Sets a rule of height param1(four bytes) and width param2(four bytes) */
                     /* These are *signed*.  Nothing typeset for nonpositive values */
                     /* However, negative value *do* change current point */
#define PUT1   133 /* Like SET1, but point doesn't change */
#define PUT2   134 /* Like SET2 */
#define PUT3   135 /* Like SET3 */
#define PUT4   136 /* Like SET4 */
#define PUT_RULE 137 /* Like SET_RULE */
#define NOP    138
#define BOP    139 /* Followed by 10 four byte count registers (signed?).  Last parameter points to */
                   /* previous BOP (backward linked, first BOP has -1).  BOP clears stack and resets current point. */
#define EOP    140
#define PUSH   141 /* Pushes h,v,w,x,y,z */
#define POP    142 /* Opposite of push*/
#define RIGHT1 143 /* Move right by one byte signed operand */
#define RIGHT2 144 /* Move right by two byte signed operand */
#define RIGHT3 145 /* Move right by three byte signed operand */
#define RIGHT4 146 /* Move right by four byte signed operand */
#define W0     147 /* Move right w */
#define W1     148 /* w <- single byte signed operand.  Move right by same amount */
#define W2     149 /* Same as W1 with two byte signed operand */
#define W3     150 /* Three byte signed operand */
#define W4     151 /* Four byte signed operand */
#define X0     152 /* Move right x */
#define X1     153 /* Like W1 */
#define X2     154 /* Like W2 */
#define X3     155 /* Like W3 */
#define X4     156 /* Like W4 */
#define DOWN1  157 /* Move down by one byte signed operand */
#define DOWN2  158 /* Two byte signed operand */
#define DOWN3  159 /* Three byte signed operand */
#define DOWN4  160 /* Four byte signed operand */
#define Y0     161 /* Move down by y */
#define Y1     162 /* Move down by one byte signed operand, which replaces Y */
#define Y2     163 /* Two byte signed operand */
#define Y3     164 /* Three byte signed operand */
#define Y4     165 /* Four byte signed operand */
#define Z0     166 /* Like Y0, but use z */
#define Z1     167 /* Like Y1 */
#define Z2     168 /* Like Y2 */
#define Z3     169 /* Like Y3 */
#define Z4     170 /* Like Y4 */
#define FNT_NUM_0 171 /* Switch to font 0 */
#define FNT_NUM_1 172 /* Switch to font 1 */
/* etc. */
#define FNT_NUM_63 234 /* Switch to font 63 */
#define FNT1       235 /* Switch to font described by single byte unsigned operand */
#define FNT2       236 /* Switch to font described by two byte unsigned operand */
#define FNT3       237 /* Three byte font descriptor */
#define FNT4       238 /* Four byte operator (Knuth says signed, but what would be the point? */
#define XXX1       239 /* Special.  Operand is one byte length.  Special follows immediately */
#define XXX2       240 /* Two byte operand */
#define XXX3       241 /* Three byte operand */
#define XXX4       242 /* Four byte operand (Knuth says TeX uses only XXX1 and XXX4 */
#define FNT_DEF1   243 /* One byte font number, four byte checksum, four byte magnified size (DVI units),
                          four byte designed size, single byte directory length, single byte name length,
                          followed by complete name (area+name) */
#define FNT_DEF2   244 /* Same for two byte font number */
#define FNT_DEF3   245 /* Same for three byte font number */
#define FNT_DEF4   246 /* Four byte font number (Knuth says signed) */
#define PRE        247 /* Preamble:
                              one byte DVI version (should be 2)
                              four byte unsigned numerator
                              four byte unsigned denominator -- one DVI unit = den/num*10^(-7) m
                              four byte magnification (multiplied by 1000)
                              one byte unsigned comment length followed by comment. */
#define DVI_ID     2    /* ID Byte for current DVI file */
#define DVIV_ID    3    /* with Ascii pTeX VW mode extension */
#define XDV_ID_OLD 6    /* older XeTeX ".xdv" output that does not have XDV_TEXT_AND_GLYPHS */
#define XDV_ID     7    /* XeTeX ".xdv" output that uses XDV opcodes below */
#define POST       248  /* Postamble- -- similar to preamble
                              four byte pointer to final bop
                              four byte numerator
                              four byte denominator
                              four byte mag
                              four byte maximum height (signed?)
                              four byte maximum width
                              two byte max stack depth required to process file
                              two byte number of pages */
#define POST_POST  249  /* End of postamble
                              four byte pointer to POST command
                              Version byte (same as preamble)
                              Padded by four or more 223's to the end of the file. */
#define PADDING    223

#define BEGIN_REFLECT       250 /* TeX-XeT begin_reflect */
#define END_REFLECT         251 /* TeX-XeT end_reflect */

                    /* XeTeX ".xdv" codes */
#define XDV_NATIVE_FONT_DEF 252 /* fontdef for native platform font */
#define XDV_GLYPHS          253 /* string of glyph IDs with X and Y positions */
#define XDV_TEXT_AND_GLYPHS 254 /* like XDV_GLYPHS plus original Unicode text */

#define PTEXDIR             255 /* Ascii pTeX DIR command */

#endif /* _DVICODES_H_ */
