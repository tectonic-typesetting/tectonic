/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2020 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

   Copyright (C) 2012-2015 by Khaled Hosny <khaledhosny@eglug.org>

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

#include "dpx-dvi.h"

#include <fcntl.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>

#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_types.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-dvicodes.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfresource.h"
#include "dpx-sfnt.h"
#include "dpx-specials.h"
#include "dpx-subfont.h"
#include "dpx-t1_char.h"
#include "dpx-t1_load.h"
#include "dpx-tfm.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_table.h"
#include "dpx-vf.h"

#define DVI_STACK_DEPTH_MAX  256u
#define TEX_FONTS_ALLOC_SIZE 16u
#define VF_NESTING_MAX       16u

/* UTF-32 over U+FFFF -> UTF-16 surrogate pair */
#define UTF32toUTF16HS(x)  (0xd800 + (((x-0x10000) >> 10) & 0x3ff))
#define UTF32toUTF16LS(x)  (0xdc00 + (  x                 & 0x3ff))

/* Interal Variables */
static rust_input_handle_t dvi_handle = NULL;
static char linear = 0; /* set to 1 for strict linear processing of the input */

static uint32_t *page_loc  = NULL;
static unsigned int num_pages = 0;

static uint32_t dvi_file_size = 0;

static struct dvi_header
{
    uint32_t unit_num;
    uint32_t unit_den;
    uint32_t mag;
    uint32_t media_width, media_height;
    unsigned int stackdepth;
    char  comment[257];
} dvi_info = {
    25400000 , /* num */
    473628672, /* den */
    1000,      /* mag */
    0, 0,      /* media width and height */
    0,         /* stackdepth */
    {'\0'}     /* comment */
};

static double dev_origin_x = 72.0, dev_origin_y = 770.0;

double get_origin (int x)
{
    return x ? dev_origin_x : dev_origin_y;
}

#define LTYPESETTING    0 /* typesetting from left to right */
#define RTYPESETTING    1 /* typesetting from right to left */
#define SKIMMING        2 /* skimming through reflected segment measuring its width */
#define REVERSE(MODE)   (LTYPESETTING + RTYPESETTING - MODE)

struct dvi_lr
{
    int state, font;
    unsigned int buf_index;
};

static struct dvi_lr lr_state;                            /* state at start of current skimming  */
static int           lr_mode;                             /* current direction or skimming depth */
static uint32_t      lr_width;                            /* total width of reflected segment    */
static uint32_t      lr_width_stack[DVI_STACK_DEPTH_MAX];
static unsigned int lr_width_stack_depth = 0;

#define PHYSICAL 1
#define VIRTUAL  2
#define SUBFONT  3
#define NATIVE   4
#define DVI      1
#define VF       2

static struct loaded_font
{
    int    type;     /* Type is physical or virtual */
    int    font_id;  /* id returned by dev (for PHYSICAL fonts)
                      * or by vf module for (VIRTUAL fonts)
                      */
    int   subfont_id; /* id returned by subfont_locate_font() */
    int   tfm_id;
    spt_t size;
    int   source;     /* Source is either DVI or VF */
    uint32_t rgba_color;
    int   xgs_id;   /* Transparency ExtGState */
    struct tt_longMetrics *hvmt;
    int   ascent;
    int   descent;
    unsigned int unitsPerEm;
    cff_font *cffont;
    unsigned int numGlyphs;
    int   layout_dir;
    float extend;
    float slant;
    float embolden;
} *loaded_fonts = NULL;
static unsigned int num_loaded_fonts = 0, max_loaded_fonts = 0;

static void
need_more_fonts (unsigned int n)
{
    if (num_loaded_fonts + n > max_loaded_fonts) {
        max_loaded_fonts += TEX_FONTS_ALLOC_SIZE;
        loaded_fonts = RENEW (loaded_fonts, max_loaded_fonts, struct loaded_font);
    }
}

static struct font_def
{
    uint32_t tex_id;
    spt_t  point_size;
    spt_t  design_size;
    char  *font_name;
    int    font_id;   /* index of _loaded_ font in loaded_fonts array */
    int    used;
    int    native; /* bool */
    uint32_t rgba_color;   /* only used for native fonts in XeTeX */
    uint32_t face_index;
    int    layout_dir; /* 1 = vertical, 0 = horizontal */
    int    extend;
    int    slant;
    int    embolden;
} *def_fonts = NULL;

#define XDV_FLAG_VERTICAL       0x0100
#define XDV_FLAG_COLORED        0x0200
#define XDV_FLAG_FEATURES       0x0400
#define XDV_FLAG_EXTEND         0x1000
#define XDV_FLAG_SLANT          0x2000
#define XDV_FLAG_EMBOLDEN       0x4000

static unsigned int num_def_fonts = 0, max_def_fonts = 0;
static int compute_boxes = 0, link_annot    = 1;

#define DVI_PAGE_BUF_CHUNK              0x10000U        /* 64K should be plenty for most pages */

static unsigned char* dvi_page_buffer;
static unsigned int   dvi_page_buf_size;
static unsigned int   dvi_page_buf_index;

/* functions to read numbers from the dvi file and store them in dvi_page_buffer */
static int
get_and_buffer_unsigned_byte (rust_input_handle_t handle)
{
    int ch;

    if ((ch = ttstub_input_getc (handle)) < 0)
        _tt_abort("File ended prematurely\n");

    if (dvi_page_buf_index >= dvi_page_buf_size) {
        dvi_page_buf_size += DVI_PAGE_BUF_CHUNK;
        dvi_page_buffer = RENEW(dvi_page_buffer, dvi_page_buf_size, unsigned char);
    }

    dvi_page_buffer[dvi_page_buf_index++] = ch;
    return ch;
}

static unsigned int
get_and_buffer_unsigned_pair (rust_input_handle_t handle)
{
    unsigned int pair = get_and_buffer_unsigned_byte(handle);
    pair = (pair << 8) | get_and_buffer_unsigned_byte(handle);
    return pair;
}

static void
get_and_buffer_bytes(rust_input_handle_t handle, unsigned int count)
{
    if (dvi_page_buf_index + count >= dvi_page_buf_size) {
        dvi_page_buf_size = dvi_page_buf_index + count + DVI_PAGE_BUF_CHUNK;
        dvi_page_buffer = RENEW(dvi_page_buffer, dvi_page_buf_size, unsigned char);
    }

    if (ttstub_input_read(handle, (char *) dvi_page_buffer + dvi_page_buf_index, count) != count)
        _tt_abort("File ended prematurely\n");

    dvi_page_buf_index += count;
}

/* functions to fetch values from dvi_page_buffer */

static int
get_buffered_unsigned_byte (void)
{
    return dvi_page_buffer[dvi_page_buf_index++];
}

static unsigned int
get_buffered_unsigned_pair (void)
{
    unsigned int pair = dvi_page_buffer[dvi_page_buf_index++];
    pair = (pair << 8) | dvi_page_buffer[dvi_page_buf_index++];
    return pair;
}

static int32_t
get_buffered_signed_quad(void)
{
    unsigned int i;
    int32_t quad = dvi_page_buffer[dvi_page_buf_index++];
    /* Check sign on first byte before reading others */
    if (quad >= 0x80)
        quad -= 0x100;
    for (i=0; i<3; i++) {
        quad = (quad << 8) | dvi_page_buffer[dvi_page_buf_index++];
    }
    return quad;
}

static int32_t
get_buffered_signed_num(unsigned char num)
{
    int32_t quad = dvi_page_buffer[dvi_page_buf_index++];
    if (quad > 0x7f)
        quad -= 0x100;
    switch (num) {
    case 3: quad = (quad << 8) | dvi_page_buffer[dvi_page_buf_index++];
    case 2: quad = (quad << 8) | dvi_page_buffer[dvi_page_buf_index++];
    case 1: quad = (quad << 8) | dvi_page_buffer[dvi_page_buf_index++];
    default: break;
    }
    return quad;
}

static int32_t
get_buffered_unsigned_num(unsigned char num)
{
    int32_t quad = dvi_page_buffer[dvi_page_buf_index++];
    switch (num) {
    case 3: if (quad > 0x7f)
            quad -= 0x100;
        quad = (quad << 8) | dvi_page_buffer[dvi_page_buf_index++];
    case 2: quad = (quad << 8) | dvi_page_buffer[dvi_page_buf_index++];
    case 1: quad = (quad << 8) | dvi_page_buffer[dvi_page_buf_index++];
    default: break;
    }
    return quad;
}

#define skip_bufferd_bytes(n) dvi_page_buf_index += n

unsigned int
dvi_npages (void)
{
    return num_pages;
}

static const char invalid_signature[] =
    "Something is wrong. Are you sure this is a DVI file?";

#define range_check_loc(loc)                    \
    if ((loc) > dvi_file_size) {                \
        _tt_abort(invalid_signature);           \
    }

static int pre_id_byte, post_id_byte, is_ptex = 0, has_ptex = 0;

static void
check_id_bytes (void) {
    if (pre_id_byte != post_id_byte && (pre_id_byte != DVI_ID || post_id_byte != DVIV_ID))
        _tt_abort("Inconsistent DVI id_bytes %d (pre) and %d (post)", pre_id_byte, post_id_byte);
}

static void
need_XeTeX (int c) {
    if (dpx_conf.compat_mode != dpx_mode_xdv_mode)
        _tt_abort("DVI opcode %i only valid for XeTeX", c);
}

static void
need_pTeX (int c) {
    if (!is_ptex)
        _tt_abort("DVI opcode %i only valid for Ascii pTeX", c);
    has_ptex = 1;
}

static int32_t
find_post (void)
{
    off_t dvi_size;
    int32_t  current;
    int   ch;

    dvi_size = ttstub_input_get_size (dvi_handle);
    if (dvi_size > 0x7FFFFFFF)
        _tt_abort("DVI file size exceeds 31-bit");
    dvi_file_size = dvi_size;

    ttstub_input_seek (dvi_handle, 0, SEEK_END);
    current = dvi_size;

    /* Scan backwards through PADDING */
    do {
        ttstub_input_seek (dvi_handle, --current, SEEK_SET);
    } while ((ch = ttstub_input_getc(dvi_handle)) == PADDING && current > 0);

    /* file_position now points to last non padding character or
     * beginning of file */
    if (dvi_file_size - current < 4 || current == 0 ||
        !(ch == DVI_ID || ch == DVIV_ID || ch == XDV_ID || ch == XDV_ID_OLD)) {
        dpx_message("DVI ID = %d\n", ch);
        _tt_abort(invalid_signature);
    }

    post_id_byte = ch;
    if (ch == XDV_ID || ch == XDV_ID_OLD)
        dpx_conf.compat_mode = dpx_mode_xdv_mode;
    is_ptex = ch == DVIV_ID;

    /* Make sure post_post is really there */
    current = current - 5;
    ttstub_input_seek (dvi_handle, current, SEEK_SET);
    if ((ch = ttstub_input_getc(dvi_handle)) != POST_POST) {
        dpx_message("Found %d where post_post opcode should be\n", ch);
        _tt_abort(invalid_signature);
    }

    current = tt_get_signed_quad(dvi_handle);
    ttstub_input_seek (dvi_handle, current, SEEK_SET);
    if ((ch = ttstub_input_getc(dvi_handle)) != POST) {
        dpx_message("Found %d where post_post opcode should be\n", ch);
        _tt_abort(invalid_signature);
    }

    /* Finally check the ID byte in the preamble */
    /* An Ascii pTeX DVI file has id_byte DVI_ID in the preamble but DVIV_ID in the postamble. */
    ttstub_input_seek (dvi_handle, 0, SEEK_SET);
    if ((ch = tt_get_unsigned_byte(dvi_handle)) != PRE) {
        dpx_message("Found %d where PRE was expected\n", ch);
        _tt_abort(invalid_signature);
    }
    ch = tt_get_unsigned_byte(dvi_handle);
    if (!(ch == DVI_ID || ch == XDV_ID || ch == XDV_ID_OLD)) {
        dpx_message("DVI ID = %d\n", ch);
        _tt_abort(invalid_signature);
    }
    pre_id_byte = ch;
    check_id_bytes();

    return current;
}

static void
get_page_info (int32_t post_location)
{
    int  i;

    ttstub_input_seek (dvi_handle, post_location + 27, SEEK_SET);
    num_pages = tt_get_unsigned_pair(dvi_handle);
    if (num_pages == 0) {
        _tt_abort("Page count is 0!");
    }
    if (dpx_conf.verbose_level > 2) {
        dpx_message("Page count:\t %4d\n", num_pages);
    }

    page_loc = NEW(num_pages, uint32_t);

    ttstub_input_seek (dvi_handle, post_location + 1, SEEK_SET);
    page_loc[num_pages-1] = tt_get_unsigned_quad(dvi_handle);
    range_check_loc(page_loc[num_pages-1] + 41);
    for (i = num_pages - 2; i >= 0; i--) {
        ttstub_input_seek (dvi_handle, page_loc[i+1] + 41, SEEK_SET);
        page_loc[i] = tt_get_unsigned_quad(dvi_handle);
        range_check_loc(page_loc[num_pages-1] + 41);
    }
}

/* Following are computed "constants" used for unit conversion */
static double dvi2pts = 1.52018, total_mag = 1.0;

double
dvi_tell_mag (void)
{
    return total_mag;
}

static void
do_scales (double mag)
{
    total_mag = (double) dvi_info.mag / 1000.0 * mag;
    dvi2pts   = (double) dvi_info.unit_num / (double) dvi_info.unit_den;
    dvi2pts  *= (72.0 / 254000.0);
}

static void
get_dvi_info (int32_t post_location)
{
    ttstub_input_seek (dvi_handle, post_location + 5, SEEK_SET);

    dvi_info.unit_num = tt_get_unsigned_quad(dvi_handle);
    dvi_info.unit_den = tt_get_unsigned_quad(dvi_handle);
    dvi_info.mag      = tt_get_unsigned_quad(dvi_handle);

    dvi_info.media_height = tt_get_unsigned_quad(dvi_handle);
    dvi_info.media_width  = tt_get_unsigned_quad(dvi_handle);

    dvi_info.stackdepth   = tt_get_unsigned_pair(dvi_handle);

    if (dvi_info.stackdepth > DVI_STACK_DEPTH_MAX) {
        dpx_warning("DVI need stack depth of %d,", dvi_info.stackdepth);
        dpx_warning("but DVI_STACK_DEPTH_MAX is %d.", DVI_STACK_DEPTH_MAX);
        _tt_abort("Capacity exceeded.");
    }

    if (dpx_conf.verbose_level > 2) {
        dpx_message("DVI File Info\n");
        dpx_message("Unit: %" PRIu32 " / %" PRIu32 "\n", dvi_info.unit_num, dvi_info.unit_den);
        dpx_message("Magnification: %" PRIu32 "\n",      dvi_info.mag);
        dpx_message("Media Height: %" PRIu32 "\n",       dvi_info.media_height);
        dpx_message("Media Width: %" PRIu32 "\n",        dvi_info.media_width);
        dpx_message("Stack Depth: %u\n",                 dvi_info.stackdepth);
    }
}

const char *
dvi_comment (void)
{
    return dvi_info.comment;
}

static void
read_font_record (uint32_t tex_id)
{
    int       dir_length, name_length;
    uint32_t  point_size, design_size;
    char     *directory, *font_name;

    if (num_def_fonts >= max_def_fonts) {
        max_def_fonts += TEX_FONTS_ALLOC_SIZE;
        def_fonts = RENEW (def_fonts, max_def_fonts, struct font_def);
    }
    tt_get_unsigned_quad(dvi_handle);
    point_size  = tt_get_positive_quad(dvi_handle, "DVI", "point_size");
    design_size = tt_get_positive_quad(dvi_handle, "DVI", "design_size");
    dir_length  = tt_get_unsigned_byte(dvi_handle);
    name_length = tt_get_unsigned_byte(dvi_handle);

    directory   = NEW(dir_length + 1, char);
    if (ttstub_input_read(dvi_handle, directory, dir_length) != dir_length) {
        _tt_abort(invalid_signature);
    }
    directory[dir_length] = '\0';
    free(directory); /* unused */

    font_name   = NEW(name_length + 1, char);
    if (ttstub_input_read(dvi_handle, font_name, name_length) != name_length) {
        _tt_abort(invalid_signature);
    }

    font_name[name_length] = '\0';
    def_fonts[num_def_fonts].tex_id      = tex_id;
    def_fonts[num_def_fonts].font_name   = font_name;
    def_fonts[num_def_fonts].point_size  = point_size;
    def_fonts[num_def_fonts].design_size = design_size;
    def_fonts[num_def_fonts].used        = 0;
    def_fonts[num_def_fonts].native      = 0;
    def_fonts[num_def_fonts].rgba_color  = 0xffffffff;
    def_fonts[num_def_fonts].face_index  = 0;
    def_fonts[num_def_fonts].layout_dir  = 0;
    def_fonts[num_def_fonts].extend      = 0x00010000; /* 1.0 */
    def_fonts[num_def_fonts].slant       = 0;
    def_fonts[num_def_fonts].embolden    = 0;
    num_def_fonts++;

    return;
}

static void
read_native_font_record (uint32_t tex_id)
{
    unsigned int  flags;
    uint32_t      point_size;
    char         *font_name;
    int           len;
    uint32_t      index;

    if (num_def_fonts >= max_def_fonts) {
        max_def_fonts += TEX_FONTS_ALLOC_SIZE;
        def_fonts = RENEW (def_fonts, max_def_fonts, struct font_def);
    }
    point_size  = tt_get_positive_quad(dvi_handle, "DVI", "point_size");
    flags       = tt_get_unsigned_pair(dvi_handle);

    len = (int) tt_get_unsigned_byte(dvi_handle); /* font name length */
    font_name = NEW(len + 1, char);
    if (ttstub_input_read(dvi_handle, font_name, len) != len) {
        _tt_abort(invalid_signature);
    }
    font_name[len] = '\0';

    index = tt_get_positive_quad(dvi_handle, "DVI", "index");

    def_fonts[num_def_fonts].tex_id      = tex_id;
    def_fonts[num_def_fonts].font_name   = font_name;
    def_fonts[num_def_fonts].face_index  = index;
    def_fonts[num_def_fonts].point_size  = point_size;
    def_fonts[num_def_fonts].design_size = 655360; /* hard-code as 10pt for now, not used anyway */
    def_fonts[num_def_fonts].used        = 0;
    def_fonts[num_def_fonts].native      = 1;

    def_fonts[num_def_fonts].layout_dir  = 0;
    def_fonts[num_def_fonts].rgba_color  = 0xffffffff;
    def_fonts[num_def_fonts].extend      = 0x00010000;
    def_fonts[num_def_fonts].slant       = 0;
    def_fonts[num_def_fonts].embolden    = 0;

    if (flags & XDV_FLAG_VERTICAL)
        def_fonts[num_def_fonts].layout_dir = 1;

    if (flags & XDV_FLAG_COLORED)
        def_fonts[num_def_fonts].rgba_color  = tt_get_unsigned_quad(dvi_handle);

    if (flags & XDV_FLAG_EXTEND)
        def_fonts[num_def_fonts].extend = tt_get_signed_quad(dvi_handle);

    if (flags & XDV_FLAG_SLANT)
        def_fonts[num_def_fonts].slant = tt_get_signed_quad(dvi_handle);

    if (flags & XDV_FLAG_EMBOLDEN)
        def_fonts[num_def_fonts].embolden = tt_get_signed_quad(dvi_handle);

    num_def_fonts++;

    return;
}

static void
get_dvi_fonts (int32_t post_location)
{
    int      code;

    ttstub_input_seek (dvi_handle, post_location + 29, SEEK_SET);
    while ((code = tt_get_unsigned_byte(dvi_handle)) != POST_POST) {
        switch (code) {
        case FNT_DEF1: case FNT_DEF2: case FNT_DEF3: case FNT_DEF4:
            read_font_record(tt_get_unsigned_num(dvi_handle, code-FNT_DEF1));
            break;
        case XDV_NATIVE_FONT_DEF:
            need_XeTeX(code);
            read_native_font_record(tt_get_signed_quad(dvi_handle));
            break;
        default:
            dpx_message("Unexpected op code: %3d\n", code);
            _tt_abort(invalid_signature);
        }
    }
    if (dpx_conf.verbose_level > 2) {
        unsigned int i;

        dpx_message("\n");
        dpx_message("DVI file font info\n");
        for (i = 0; i < num_def_fonts; i++) {
            dpx_message("TeX Font: %10s loaded at ID=%5d, ",
                        def_fonts[i].font_name, def_fonts[i].tex_id);
            dpx_message("size=%5.2fpt (scaled %4.1f%%)",
                        def_fonts[i].point_size * dvi2pts,
                        100.0 * ((double) def_fonts[i].point_size / def_fonts[i].design_size));
            dpx_message("\n");
        }
    }
}

static void get_comment (void)
{
    int length;

    ttstub_input_seek (dvi_handle, 14, SEEK_SET);
    length = tt_get_unsigned_byte(dvi_handle);
    if (ttstub_input_read(dvi_handle, dvi_info.comment, length) != length) {
        _tt_abort(invalid_signature);
    }
    dvi_info.comment[length] = '\0';
    if (dpx_conf.verbose_level > 0) {
        dpx_message("DVI Comment: %s\n", dvi_info.comment);
    }
}

/*
 * The section below this line deals with the actual processing of the
 * dvi file.
 *
 * The dvi file processor state is contained in the following variables:
 */

struct dvi_registers
{
    int32_t h, v, w, x, y, z;
    unsigned int d;
};

static struct   dvi_registers dvi_state;
static struct   dvi_registers dvi_stack[DVI_STACK_DEPTH_MAX];
static int      dvi_stack_depth = 0 ;
static int      current_font    = -1;
static int      processing_page = 0 ;

static void
clear_state (void)
{
    dvi_state.h = 0; dvi_state.v = 0; dvi_state.w = 0;
    dvi_state.x = 0; dvi_state.y = 0; dvi_state.z = 0;
    dvi_state.d = 0; /* direction */
    pdf_dev_set_dirmode(0);
    dvi_stack_depth = 0;
    current_font    = -1;
}

/* Migrated from pdfdev.c:
 * The following codes are originally put into pdfdev.c.
 * But they are moved to here to make PDF output independent
 * from DVI input.
 * pdfdoc, pdfspecial and htex are also modified. pdfspecial
 * and htex does tag/untag depth. pdfdev and pdfdoc now does
 * not care about line-breaking at all.
 */
static int marked_depth =  0;
static int tagged_depth = -1;

static void
dvi_mark_depth (void)
{
    /* If decreasing below tagged_depth */
    if (link_annot &&
        marked_depth    == tagged_depth &&
        dvi_stack_depth == tagged_depth - 1) {
        /*
         * See if this appears to be the end of a "logical unit"
         * that's been broken.  If so, flush the logical unit.
         */
        pdf_doc_break_annot();
    }
    marked_depth = dvi_stack_depth;
}

/*
 * The following routines setup and tear down a callback at a
 * certain stack depth. This is used to handle broken (linewise)
 * links.
 */
void
dvi_tag_depth (void)
{
    tagged_depth = marked_depth;
    dvi_compute_boxes(1);
}

void
dvi_untag_depth (void)
{
    tagged_depth = -1;
    dvi_compute_boxes(0);
}

void
dvi_compute_boxes (int flag)
{
    compute_boxes = flag;
}

void
dvi_link_annot (int flag)
{
    link_annot = flag;
}

bool
dvi_is_tracking_boxes(void)
{
    return compute_boxes && link_annot && marked_depth >= tagged_depth;
}

void
dvi_do_special (const void *buffer, int32_t size)
{
    double x_user, y_user, mag;
    const char *p;

    graphics_mode();

    p = (const char *) buffer;

    x_user =  dvi_state.h * dvi2pts;
    y_user = -dvi_state.v * dvi2pts;
    mag    =  dvi_tell_mag();

    if (spc_exec_special(p, size, x_user, y_user, mag) < 0) {
        if (dpx_conf.verbose_level > 0) {
            dump(p, p + size);
        }
    }

    return;
}

double
dvi_unit_size (void)
{
    return dvi2pts;
}


unsigned int
dvi_locate_font (const char *tfm_name, spt_t ptsize)
{
    unsigned int cur_id;
    const char   *name = tfm_name;
    int           subfont_id = -1, font_id; /* VF or device font ID */
    fontmap_rec  *mrec;

    if (dpx_conf.verbose_level > 0)
        dpx_message("<%s@%.2fpt", tfm_name, ptsize * dvi2pts);

    need_more_fonts(1);

    /* This routine needs to be recursive/reentrant. Load current high water
     * mark into an automatic variable.
     */
    cur_id = num_loaded_fonts++;

    mrec = pdf_lookup_fontmap_record(tfm_name);
    /* Load subfont mapping table */
    if (mrec && mrec->charmap.sfd_name && mrec->charmap.subfont_id) {
        subfont_id = sfd_load_record(mrec->charmap.sfd_name, mrec->charmap.subfont_id);
    }

    memset(&loaded_fonts[cur_id], 0, sizeof (struct loaded_font));

    /* TFM must exist here. */
    loaded_fonts[cur_id].tfm_id     = tfm_open(tfm_name, 1);
    loaded_fonts[cur_id].subfont_id = subfont_id;
    loaded_fonts[cur_id].size       = ptsize;
    /* This will be reset later if it was really generated by the dvi file. */
    loaded_fonts[cur_id].source     = VF;

    /* The order of searching fonts is as follows:
     *
     * 1. If mrec is null, that is, there is no map entry matching
     *    with tfm_name, then search a virtual font matching with
     *    tfm_name at first. If no virtual font is found, search a
     *    PK font matching with tfm_name.
     *
     * 2. If mrec is non-null, search a physical scalable font.
     *
     * 3. Notice that every subfont gets non-null mrec. In this case,
     *    enc_name corresponding to mrec will be used instead of mrec.
     *    That is enc_name is NULL, search a virtual font for Omega (.ovf)
     *    matching with the base name of the subfont. If no virtual font
     *    for Omega is found, it is a fatal error because there is no PK font
     *    for Omega.
     */
    if (!mrec) {
        font_id = vf_locate_font(tfm_name, ptsize);
        if (font_id >= 0) {
            loaded_fonts[cur_id].type    = VIRTUAL;
            loaded_fonts[cur_id].font_id = font_id;
            if (dpx_conf.verbose_level > 0)
                dpx_message("(VF)>");
            return  cur_id;
        }
    }
#if  1
    /* Sorry, I don't understand this well... Please fix.
     * The purpose of this seems to be:
     *
     *   Map 8-bit char codes in subfont to 16-bit code with SFD mapping
     *   and map subfonts to single OVF font.
     *
     * But it apparently only does TFM -> OVF mapping but no character
     * code mapping. Please see dvi_set(), you can't have both font->type
     * VIRTUAL and font->subfont_id >= 0. Am I missing something?
     */
    else if (subfont_id >= 0 && mrec->map_name)
    {
        fontmap_rec  *mrec1 = pdf_lookup_fontmap_record(mrec->map_name);
        /* enc_name=NULL should be used only for 'built-in' encoding.
         * Please fix this!
         */
        if (mrec1 && !mrec1->enc_name) {
            font_id = vf_locate_font(mrec1->font_name, ptsize);
            if (font_id < 0)
                dpx_warning("Could not locate Omega Virtual Font \"%s\" for \"%s\".",
                            mrec1->font_name, tfm_name);
            else {
                loaded_fonts[cur_id].type    = VIRTUAL;
                loaded_fonts[cur_id].font_id = font_id;
                if (dpx_conf.verbose_level > 0)
                    dpx_message("(OVF)>");
                return  cur_id;
            }
        }
    }
#endif  /* 1 */

    /* Failed to load a virtual font so we try to load a physical font. */

    /* If mrec->map_name is not NULL, font name identified in PDF output
     * is different than tfm_name, this can happen for subfonts grouped
     * into a single "intermediate" font foo@SFD@.
     * This is necessary for optimal output; to avoid unnecessary creation
     * of multiple instances of a same font, to avoid frequent font selection
     * and break of string_mode.
     */
    if (mrec && mrec->map_name) {
        name = mrec->map_name;
    } else {
        name = tfm_name;
    }

    /* We need ptsize for PK font creation. */
    font_id = pdf_dev_locate_font(name, ptsize);
    if (font_id < 0) {
        dpx_warning("Could not locate a virtual/physical font for TFM \"%s\".", tfm_name);
        if (mrec && mrec->map_name) { /* has map_name */
            fontmap_rec  *mrec1 = pdf_lookup_fontmap_record(mrec->map_name);
            dpx_warning(">> This font is mapped to an intermediate 16-bit font \"%s\" with SFD charmap=<%s,%s>,",
                        mrec->map_name, mrec->charmap.sfd_name, mrec->charmap.subfont_id);
            if (!mrec1)
                dpx_warning(">> but I couldn't find font mapping for \"%s\".", mrec->map_name);
            else {
                dpx_warning(">> and then mapped to a physical font \"%s\" by fontmap.", mrec1->font_name);
                dpx_warning(">> Please check if kpathsea library can find this font: %s", mrec1->font_name);
            }
        } else if (mrec && !mrec->map_name) {
            char *finaldot = strrchr(mrec->font_name, '.');
            if (finaldot && strcasecmp(finaldot, ".pfa") == 0) {
                /* type1 fonts with pfa format are not supported */
                dpx_warning("This font is mapped to a physical font \"%s\".", mrec->font_name);
                _tt_abort("Sorry, pfa format not supported; please convert the font to pfb, e.g., with t1binary.");
            } else {
                dpx_warning(">> This font is mapped to a physical font \"%s\".", mrec->font_name);
                dpx_warning(">> Please check if kpathsea library can find this font: %s", mrec->font_name);
            }
        } else {
            dpx_warning(">> There are no valid font mapping entry for this font.");
            dpx_warning(">> Font file name \"%s\" was assumed but failed to locate that font.", tfm_name);
        }
        _tt_abort("Cannot proceed without .vf or \"physical\" font for PDF output...");
    }
    loaded_fonts[cur_id].type    = PHYSICAL;
    loaded_fonts[cur_id].font_id = font_id;

    if (dpx_conf.verbose_level > 0)
        dpx_message(">");

    return  cur_id;
}

static int
dvi_locate_native_font (const char *filename, uint32_t index,
                        spt_t ptsize, int layout_dir, int extend, int slant, int embolden)
{
    int           cur_id = -1;
    fontmap_rec  *mrec;
    char         *fontmap_key;
    rust_input_handle_t handle;
    sfnt         *sfont;
    ULONG         offset = 0;
    struct tt_head_table *head;
    struct tt_maxp_table *maxp;
    struct tt_hhea_table *hhea;
    int is_dfont = 0, is_type1 = 0;

    if (dpx_conf.verbose_level > 0)
        dpx_message("<%s@%.2fpt", filename, ptsize * dvi2pts);

    if ((handle = dpx_open_dfont_file(filename)) != NULL)
        is_dfont = 1;
    else if ((handle = dpx_open_type1_file(filename)) != NULL)
        is_type1 = 1;
    else if (((handle = dpx_open_opentype_file(filename)) == NULL
              && (handle = dpx_open_truetype_file(filename)) == NULL)) {
        _tt_abort("Cannot proceed without the font: %s", filename);
    }
    need_more_fonts(1);

    cur_id = num_loaded_fonts++;

    fontmap_key = xmalloc(strlen(filename) + 40); // CHECK this is enough
    sprintf(fontmap_key, "%s/%u/%c/%d/%d/%d", filename, index, layout_dir == 0 ? 'H' : 'V', extend, slant, embolden);
    mrec = pdf_lookup_fontmap_record(fontmap_key);
    if (mrec == NULL) {
        if ((mrec = pdf_insert_native_fontmap_record(filename, index, layout_dir, extend, slant, embolden)) == NULL) {
            _tt_abort("Failed to insert font record for font: %s", filename);
        }
    }

    memset(&loaded_fonts[cur_id], 0, sizeof (struct loaded_font));

    loaded_fonts[cur_id].font_id = pdf_dev_locate_font(fontmap_key, ptsize);
    loaded_fonts[cur_id].size    = ptsize;
    loaded_fonts[cur_id].type    = NATIVE;
    free(fontmap_key);

    if (is_type1) {
        cff_font *cffont;
        char     *enc_vec[256];

        /*if (!is_pfb(fp))
         *  _tt_abort("Failed to read Type 1 font \"%s\".", filename);
         */
        dpx_warning("skipping PFB sanity check -- needs Tectonic I/O update");

        memset(enc_vec, 0, 256 * sizeof(char *));
        cffont = t1_load_font (enc_vec, 0, handle);
        if (!cffont)
            _tt_abort("Failed to read Type 1 font \"%s\".", filename);

        loaded_fonts[cur_id].cffont = cffont;

        if (cff_dict_known(cffont->topdict, "FontBBox")) {
            loaded_fonts[cur_id].ascent = cff_dict_get(cffont->topdict, "FontBBox", 3);
            loaded_fonts[cur_id].descent = cff_dict_get(cffont->topdict, "FontBBox", 1);
        } else {
            loaded_fonts[cur_id].ascent = 690;
            loaded_fonts[cur_id].descent = -190;
        }

        loaded_fonts[cur_id].unitsPerEm = 1000;
        loaded_fonts[cur_id].numGlyphs = cffont->num_glyphs;

        ttstub_input_close (handle);
    } else {
        if (is_dfont)
            sfont = dfont_open(handle, index);
        else
            sfont = sfnt_open(handle);
        if (sfont->type == SFNT_TYPE_TTC)
            offset = ttc_read_offset(sfont, index);
        else if (sfont->type == SFNT_TYPE_DFONT)
            offset = sfont->offset;
        sfnt_read_table_directory(sfont, offset);
        head = tt_read_head_table(sfont);
        maxp = tt_read_maxp_table(sfont);
        hhea = tt_read_hhea_table(sfont);
        loaded_fonts[cur_id].ascent = hhea->ascent;
        loaded_fonts[cur_id].descent = hhea->descent;
        loaded_fonts[cur_id].unitsPerEm = head->unitsPerEm;
        loaded_fonts[cur_id].numGlyphs = maxp->numGlyphs;
        if (layout_dir == 1 && sfnt_find_table_pos(sfont, "vmtx") > 0) {
            struct tt_vhea_table *vhea = tt_read_vhea_table(sfont);
            sfnt_locate_table(sfont, "vmtx");
            loaded_fonts[cur_id].hvmt = tt_read_longMetrics(sfont, maxp->numGlyphs, vhea->numOfLongVerMetrics, vhea->numOfExSideBearings);
            free(vhea);
        } else {
            sfnt_locate_table(sfont, "hmtx");
            loaded_fonts[cur_id].hvmt = tt_read_longMetrics(sfont, maxp->numGlyphs, hhea->numOfLongHorMetrics, hhea->numOfExSideBearings);
        }
        free(hhea);
        free(maxp);
        free(head);
        sfnt_close(sfont);
        ttstub_input_close(handle);
    }

    loaded_fonts[cur_id].layout_dir = layout_dir;
    loaded_fonts[cur_id].extend = mrec->opt.extend;
    loaded_fonts[cur_id].slant = mrec->opt.slant;
    loaded_fonts[cur_id].embolden = mrec->opt.bold;

    if (dpx_conf.verbose_level > 0)
        dpx_message(">");

    return cur_id;
}

double
dvi_dev_xpos (void)
{
    return dvi_state.h * dvi2pts;
}

double
dvi_dev_ypos (void)
{
    return -(dvi_state.v * dvi2pts);
}

static void do_moveto (int32_t x, int32_t y)
{
    dvi_state.h = x;
    dvi_state.v = y;
}

/* FIXME: dvi_forward() might be a better name */
void dvi_right (int32_t x)
{
    if (lr_mode >= SKIMMING) {
        lr_width += x;
        return;
    }

    if (lr_mode == RTYPESETTING)
        x = -x;

    switch (dvi_state.d) {
    case 0:
        dvi_state.h += x; break;
    case 1:
        dvi_state.v += x; break;
    case 3:
        dvi_state.v -= x; break;
    }
}

void dvi_down (int32_t y)
{
    if (lr_mode < SKIMMING) {
        switch (dvi_state.d) {
        case 0:
            dvi_state.v += y; break;
        case 1:
            dvi_state.h -= y; break;
        case 3:
            dvi_state.h += y; break;
        }
    }
}

/* _FIXME_
 * CMap decoder wants multibyte strings as input but
 * how DVI char codes are converted to multibyte sting
 * is not clear.
 */
void
dvi_set (int32_t ch)
{
    struct loaded_font *font;
    spt_t               width, height, depth;
    unsigned char       wbuf[4];

    if (current_font < 0) {
        _tt_abort("No font selected!");
    }
    /* The division by dvi2pts seems strange since we actually know the
     * "dvi" size of the fonts contained in the DVI file.  In other
     * words, we converted from DVI units to pts and back again!
     * The problem comes from fonts defined in VF files where we don't know
     * the DVI size.  It's keeping me sane to keep *point sizes* of *all*
     * fonts in the dev.c file and convert them back if necessary.
     */
    font  = &loaded_fonts[current_font];

    width = tfm_get_fw_width(font->tfm_id, ch);
    width = sqxfw(font->size, width);

    if (lr_mode >= SKIMMING) {
        lr_width += width;
        return;
    }

    if (lr_mode == RTYPESETTING)
        dvi_right(width); /* Will actually move left */

    switch (font->type) {
    case  PHYSICAL:
        if (ch > 65535) { /* _FIXME_ */
            wbuf[0] = (UTF32toUTF16HS(ch) >> 8) & 0xff;
            wbuf[1] =  UTF32toUTF16HS(ch)       & 0xff;
            wbuf[2] = (UTF32toUTF16LS(ch) >> 8) & 0xff;
            wbuf[3] =  UTF32toUTF16LS(ch)       & 0xff;
            pdf_dev_set_string(dvi_state.h, -dvi_state.v, wbuf, 4,
                               width, font->font_id, 2);
        } else if (ch > 255) { /* _FIXME_ */
            wbuf[0] = (ch >> 8) & 0xff;
            wbuf[1] =  ch & 0xff;
            pdf_dev_set_string(dvi_state.h, -dvi_state.v, wbuf, 2,
                               width, font->font_id, 2);
        } else if (font->subfont_id >= 0) {
            unsigned short uch = lookup_sfd_record(font->subfont_id, (unsigned char) ch);
            wbuf[0] = (uch >> 8) & 0xff;
            wbuf[1] =  uch & 0xff;
            pdf_dev_set_string(dvi_state.h, -dvi_state.v, wbuf, 2,
                               width, font->font_id, 2);
        } else {
            wbuf[0] = (unsigned char) ch;
            pdf_dev_set_string(dvi_state.h, -dvi_state.v, wbuf, 1,
                               width, font->font_id, 1);
        }
        if (dvi_is_tracking_boxes()) {
            pdf_rect rect;

            height = tfm_get_fw_height(font->tfm_id, ch);
            depth  = tfm_get_fw_depth (font->tfm_id, ch);
            height = sqxfw(font->size, height);
            depth  = sqxfw(font->size, depth);

            pdf_dev_set_rect  (&rect, dvi_state.h, -dvi_state.v,
                               width, height, depth);
            pdf_doc_expand_box(&rect);
        }
        break;
    case  VIRTUAL:
        vf_set_char(ch, font->font_id); /* push/pop invoked */
        break;
    }

    if (lr_mode == LTYPESETTING)
        dvi_right(width);

}

void
dvi_put (int32_t ch)
{
    struct loaded_font *font;
    spt_t               width, height, depth;
    unsigned char       wbuf[4];

    if (current_font < 0) {
        _tt_abort("No font selected!");
    }

    font = &loaded_fonts[current_font];

    switch (font->type) {
    case  PHYSICAL:
        width = tfm_get_fw_width(font->tfm_id, ch);
        width = sqxfw(font->size, width);

        /* Treat a single character as a one byte string and use the
         * string routine.
         */
        if (ch > 65535) { /* _FIXME_ */
            wbuf[0] = (UTF32toUTF16HS(ch) >> 8) & 0xff;
            wbuf[1] =  UTF32toUTF16HS(ch)       & 0xff;
            wbuf[2] = (UTF32toUTF16LS(ch) >> 8) & 0xff;
            wbuf[3] =  UTF32toUTF16LS(ch)       & 0xff;
            pdf_dev_set_string(dvi_state.h, -dvi_state.v, wbuf, 4,
                               width, font->font_id, 2);
        } else if (ch > 255) { /* _FIXME_ */
            wbuf[0] = (ch >> 8) & 0xff;
            wbuf[1] =  ch & 0xff;
            pdf_dev_set_string(dvi_state.h, -dvi_state.v, wbuf, 2,
                               width, font->font_id, 2);
        } else if (font->subfont_id >= 0) {
            unsigned int uch;

            uch = lookup_sfd_record(font->subfont_id, (unsigned char) ch);
            wbuf[0] = (uch >> 8) & 0xff;
            wbuf[1] =  uch & 0xff;
            pdf_dev_set_string(dvi_state.h, -dvi_state.v, wbuf, 2,
                               width, font->font_id, 2);
        } else {
            wbuf[0] = (unsigned char) ch;
            pdf_dev_set_string(dvi_state.h, -dvi_state.v, wbuf, 1,
                               width, font->font_id, 1);
        }
        if (dvi_is_tracking_boxes()) {
            pdf_rect rect;

            height = tfm_get_fw_height(font->tfm_id, ch);
            depth  = tfm_get_fw_depth (font->tfm_id, ch);
            height = sqxfw(font->size, height);
            depth  = sqxfw(font->size, depth);

            pdf_dev_set_rect  (&rect, dvi_state.h, -dvi_state.v,
                               width, height, depth);
            pdf_doc_expand_box(&rect);
        }
        break;
    case  VIRTUAL:
        vf_set_char(ch, font->font_id);
        break;
    }

    return;
}


void
dvi_rule (int32_t width, int32_t height)
{
    if (width > 0 && height > 0) {
        do_moveto(dvi_state.h, dvi_state.v);

        switch (dvi_state.d) {
        case 0:
            pdf_dev_set_rule(dvi_state.h, -dvi_state.v,  width, height);
            break;
        case 1:
            pdf_dev_set_rule(dvi_state.h, -dvi_state.v - width, height, width);
            break;
        case 3:
            pdf_dev_set_rule(dvi_state.h - height, -dvi_state.v , height, width);
            break;
        }
    }
}

void
dvi_dirchg (unsigned char dir)
{
    if (dpx_conf.verbose_level > 0)
        fprintf(stderr, "  > dvi_dirchg %d\n", dir);
    dvi_state.d = dir;
    pdf_dev_set_dirmode(dvi_state.d); /* 0: horizontal, 1,3: vertical */
}

static void
do_setrule (void)
{
    int32_t width, height;

    height = get_buffered_signed_quad();
    width  = get_buffered_signed_quad();
    switch (lr_mode) {
    case LTYPESETTING:
        dvi_rule(width, height);
        dvi_right(width);
        break;
    case RTYPESETTING:
        dvi_right(width);
        dvi_rule(width, height);
        break;
    default:
        lr_width += width;
        break;
    }
}

static void
do_putrule (void)
{
    int32_t width, height;

    height = get_buffered_signed_quad ();
    width  = get_buffered_signed_quad ();
    switch (lr_mode) {
    case LTYPESETTING:
        dvi_rule(width, height);
        break;
    case RTYPESETTING:
        dvi_right(width);
        dvi_rule(width, height);
        dvi_right(-width);
        break;
    default:
        break;
    }
}

void
dvi_push (void)
{
    if (dvi_stack_depth >= DVI_STACK_DEPTH_MAX)
        _tt_abort("DVI stack exceeded limit.");

    dvi_stack[dvi_stack_depth++] = dvi_state;
}

void
dpx_dvi_pop (void)
{
    if (dvi_stack_depth <= 0)
        _tt_abort("Tried to pop an empty stack.");

    dvi_state = dvi_stack[--dvi_stack_depth];
    do_moveto(dvi_state.h, dvi_state.v);
    pdf_dev_set_dirmode(dvi_state.d); /* 0: horizontal, 1,3: vertical */
}


void
dvi_w (int32_t ch)
{
    dvi_state.w = ch;
    dvi_right(ch);
}

void
dvi_w0 (void)
{
    dvi_right(dvi_state.w);
}

void
dvi_x (int32_t ch)
{
    dvi_state.x = ch;
    dvi_right(ch);
}

void
dvi_x0 (void)
{
    dvi_right(dvi_state.x);
}

void
dvi_y (int32_t ch)
{
    dvi_state.y = ch;
    dvi_down(ch);
}

void
dvi_y0 (void)
{
    dvi_down(dvi_state.y);
}

void
dvi_z (int32_t ch)
{
    dvi_state.z = ch;
    dvi_down(ch);
}

void
dvi_z0 (void)
{
    dvi_down(dvi_state.z);
}

static void
skip_fntdef (void)
{
    int area_len, name_len;

    tt_skip_bytes(12, dvi_handle);
    area_len = tt_get_unsigned_byte(dvi_handle);
    name_len = tt_get_unsigned_byte(dvi_handle);
    tt_skip_bytes(area_len + name_len, dvi_handle);
}

/* when pre-scanning the page, we process fntdef
   and remove the fntdef opcode from the buffer */
static void
do_fntdef (uint32_t tex_id)
{
    if (linear)
        read_font_record(tex_id);
    else
        skip_fntdef();
    --dvi_page_buf_index;
}

void
dvi_set_font (int font_id)
{
    current_font = font_id;
}

static void
do_fnt (uint32_t tex_id)
{
    unsigned int i;

    for (i = 0; i < num_def_fonts; i++) {
        if (def_fonts[i].tex_id == tex_id)
            break;
    }

    if (i == num_def_fonts) {
        _tt_abort("Tried to select a font that hasn't been defined: id=%d", tex_id);
    }

    if (!def_fonts[i].used) {
        unsigned int font_id;

        if (def_fonts[i].native) {
            font_id = dvi_locate_native_font(def_fonts[i].font_name,
                                             def_fonts[i].face_index,
                                             def_fonts[i].point_size,
                                             def_fonts[i].layout_dir,
                                             def_fonts[i].extend,
                                             def_fonts[i].slant,
                                             def_fonts[i].embolden);
        } else {
            font_id = dvi_locate_font(def_fonts[i].font_name,
                                      def_fonts[i].point_size);
        }
        loaded_fonts[font_id].rgba_color = def_fonts[i].rgba_color;
        /* Opacity: 0xff is fully opaque. */
        if ((loaded_fonts[font_id].rgba_color & 0xff) == 0xff) {
            loaded_fonts[font_id].xgs_id = -1;
        } else {
            pdf_obj *xgs_dict;
            int      a = loaded_fonts[font_id].rgba_color & 0xff;
        
            /* Inefficient but don't care as transparency is not expected to be frequently used. */
            xgs_dict = pdf_new_dict();
            pdf_add_dict(xgs_dict, pdf_new_name("Type"), pdf_new_name("ExtGState"));
            pdf_add_dict(xgs_dict, pdf_new_name("ca"), pdf_new_number(a/255.0));
            pdf_add_dict(xgs_dict, pdf_new_name("CA"), pdf_new_number(a/255.0));
            loaded_fonts[font_id].xgs_id = pdf_defineresource("ExtGState", NULL, xgs_dict, 0);
        }
        loaded_fonts[font_id].source = DVI;
        def_fonts[i].used    = 1;
        def_fonts[i].font_id = font_id;
    }
    current_font = def_fonts[i].font_id;
}

static void
do_xxx (int32_t size)
{
    if (lr_mode < SKIMMING)
        dvi_do_special(dvi_page_buffer + dvi_page_buf_index, size);
    dvi_page_buf_index += size;
}

static void
do_bop (void)
{
    unsigned int i;

    if (processing_page)
        _tt_abort("Got a bop in the middle of a page!");

    /* For now, ignore TeX's count registers */
    for (i = 0; i < 10; i++) {
        skip_bufferd_bytes(4);
    }
    /* Ignore previous page pointer since we have already
     * saved this information
     */
    skip_bufferd_bytes(4);
    clear_state();
    processing_page = 1;

    pdf_doc_begin_page(dvi_tell_mag(), dev_origin_x, dev_origin_y);
    spc_exec_at_begin_page();

    return;
}

static void
do_eop (void)
{
    processing_page = 0;

    if (dvi_stack_depth != 0) {
        _tt_abort("DVI stack depth is not zero at end of page");
    }
    spc_exec_at_end_page();

    pdf_doc_end_page();

    return;
}

static void
do_dir (void)
{
    dvi_state.d = get_buffered_unsigned_byte();
    pdf_dev_set_dirmode(dvi_state.d); /* 0: horizontal, 1,3: vertical */
}

static void
lr_width_push (void)
{
    if (lr_width_stack_depth >= DVI_STACK_DEPTH_MAX)
        _tt_abort("Segment width stack exceeded limit.");

    lr_width_stack[lr_width_stack_depth++] = lr_width;
}

static void
lr_width_pop (void)
{
    if (lr_width_stack_depth <= 0)
        _tt_abort("Tried to pop an empty segment width stack.");

    lr_width = lr_width_stack[--lr_width_stack_depth];
}

static void
dvi_begin_reflect (void)
{
    if (lr_mode >= SKIMMING) {
        ++lr_mode;
    } else {
        lr_state.buf_index = dvi_page_buf_index;
        lr_state.font = current_font;
        lr_state.state = lr_mode;
        lr_mode = SKIMMING;
        lr_width = 0;
    }
}

static void
dvi_end_reflect (void)
{
    switch (lr_mode) {
    case SKIMMING:
        current_font = lr_state.font;
        dvi_page_buf_index = lr_state.buf_index;
        lr_mode = REVERSE(lr_state.state); /* must precede dvi_right */
        dvi_right(-(int32_t)lr_width);
        lr_width_push();
        break;
    case LTYPESETTING:
    case RTYPESETTING:
        lr_width_pop();
        dvi_right(-(int32_t)lr_width);
        lr_mode = REVERSE(lr_mode);
        break;
    default:                     /* lr_mode > SKIMMING */
        lr_mode--;
    }
}

static void
skip_native_font_def (void)
{
    unsigned int flags;
    int name_length;

    tt_skip_bytes(4, dvi_handle); /* skip point size */
    flags = tt_get_unsigned_pair(dvi_handle);
    name_length = tt_get_unsigned_byte(dvi_handle);
    tt_skip_bytes(name_length + 4, dvi_handle);

    if (flags & XDV_FLAG_COLORED)
        tt_skip_bytes(4, dvi_handle);

    if (flags & XDV_FLAG_EXTEND)
        tt_skip_bytes(4, dvi_handle);

    if (flags & XDV_FLAG_SLANT)
        tt_skip_bytes(4, dvi_handle);

    if (flags & XDV_FLAG_EMBOLDEN)
        tt_skip_bytes(4, dvi_handle);
}

static void
do_native_font_def (int32_t tex_id)
{
    if (linear)
        read_native_font_record(tex_id);
    else
        skip_native_font_def();
    --dvi_page_buf_index; /* don't buffer the opcode */
}

static void
skip_glyphs (void)
{
    unsigned int i, slen = 0;
    slen = (unsigned int) get_buffered_unsigned_pair();
    for (i = 0; i < slen; i++) {
        skip_bufferd_bytes(4);
        skip_bufferd_bytes(4);
        skip_bufferd_bytes(2);
    }
}

static void
do_glyphs (int do_actual_text)
{
    struct loaded_font *font;
    spt_t  width, height, depth, *xloc, *yloc, glyph_width = 0;
    unsigned char wbuf[2];
    unsigned int i, glyph_id, slen = 0;

    if (current_font < 0)
        _tt_abort("No font selected!");

    font  = &loaded_fonts[current_font];

    if (do_actual_text) {
        slen = (unsigned int) get_buffered_unsigned_pair();
        if (lr_mode >= SKIMMING) {
            for (i = 0; i < slen; i++) {
                skip_bufferd_bytes(2);
            }
        } else {
            uint16_t *unicodes = NEW(slen, uint16_t);
            for (i = 0; i < slen; i++) {
                unicodes[i] = (uint16_t) get_buffered_unsigned_pair();
            }
            pdf_dev_begin_actualtext (unicodes, slen);
            free(unicodes);
        }
    }

    width = get_buffered_signed_quad();

    if (lr_mode >= SKIMMING) {
        lr_width += width;
        skip_glyphs();
        return;
    }

    if (lr_mode == RTYPESETTING)
        dvi_right(width); /* Will actually move left */

    slen = (unsigned int) get_buffered_unsigned_pair();
    xloc = NEW(slen, spt_t);
    yloc = NEW(slen, spt_t);
    for (i = 0; i < slen; i++) {
        xloc[i] = get_buffered_signed_quad();
        yloc[i] = get_buffered_signed_quad();
    }

    if (font->rgba_color != 0xffffffff) {
        pdf_color color;
        pdf_color_rgbcolor(&color,
                           (double)((unsigned char)(font->rgba_color >> 24) & 0xff) / 255,
                           (double)((unsigned char)(font->rgba_color >> 16) & 0xff) / 255,
                           (double)((unsigned char)(font->rgba_color >>  8) & 0xff) / 255);
        pdf_color_push(&color, &color);
        /* Opacity:
        * Enter graphics_mode and then enclose with save/resotre
        * since pdf_color_pop() may not restore graphics state.
        */
        if (font->xgs_id >= 0) {
            pdf_obj *ref;
            char     resname[16];
            char     content[22];
        
            sprintf(resname, "Xtx_Gs_%08x", current_font);
            ref = pdf_get_resource_reference(font->xgs_id);
            pdf_doc_add_page_resource("ExtGState", resname, ref);
            graphics_mode();
            pdf_dev_gsave();
            sprintf(content, " /%s gs ", resname);
            pdf_doc_add_page_content(content, strlen(content));
        }
    }

    for (i = 0; i < slen; i++) {
        glyph_id = get_buffered_unsigned_pair(); /* freetype glyph index */
        if (glyph_id < font->numGlyphs) {
            unsigned int advance;
            double ascent = (double)font->ascent;
            double descent = (double)font->descent;

            if (font->cffont) {
                cff_index *cstrings = font->cffont->cstrings;
                t1_ginfo gm;

                /* If .notdef is not the 1st glyph in CharStrings, glyph_id given by
                   FreeType should be increased by 1 */
                if (font->cffont->is_notdef_notzero)
                    glyph_id += 1;

                t1char_get_metrics(cstrings->data + cstrings->offset[glyph_id] - 1,
                                   cstrings->offset[glyph_id + 1] - cstrings->offset[glyph_id],
                                   font->cffont->subrs[0], &gm);

                advance = font->layout_dir == 0 ? gm.wx : gm.wy;
                ascent = gm.bbox.ury;
                descent = gm.bbox.lly;
            } else {
                advance = font->hvmt[glyph_id].advance;
            }

            glyph_width    = (double)font->size * (double)advance / (double)font->unitsPerEm;
            glyph_width    = glyph_width * font->extend;

            if (dvi_is_tracking_boxes()) {
                pdf_rect rect;
                height = (double)font->size * ascent / (double)font->unitsPerEm;
                depth  = (double)font->size * -descent / (double)font->unitsPerEm;
                pdf_dev_set_rect(&rect, dvi_state.h + xloc[i], -dvi_state.v - yloc[i], glyph_width, height, depth);
                pdf_doc_expand_box(&rect);
            }
        }

        wbuf[0] = glyph_id >> 8;
        wbuf[1] = glyph_id & 0xff;
        pdf_dev_set_string(dvi_state.h + xloc[i], -dvi_state.v - yloc[i], wbuf, 2,
                           glyph_width, font->font_id, -1);
    }

    if (font->rgba_color != 0xffffffff) {
        if (font->xgs_id >= 0) {
            graphics_mode(); 
            pdf_dev_grestore();
        }
        pdf_color_pop();
    }
    free(xloc);
    free(yloc);

    if (do_actual_text) {
        pdf_dev_end_actualtext();
    }

    if (lr_mode == LTYPESETTING)
        dvi_right(width);

    return;
}

static void
check_postamble (void)
{
    int code;

    tt_skip_bytes(28, dvi_handle);
    while ((code = tt_get_unsigned_byte(dvi_handle)) != POST_POST) {
        switch (code) {
        case FNT_DEF1: case FNT_DEF2: case FNT_DEF3: case FNT_DEF4:
            tt_skip_bytes(code + 1 - FNT_DEF1, dvi_handle);
            skip_fntdef();
            break;
        case XDV_NATIVE_FONT_DEF:
            tt_skip_bytes(4, dvi_handle);
            skip_native_font_def();
            break;
        default:
            _tt_abort("Unexpected op code (%u) in postamble", code);
        }
    }
    tt_skip_bytes(4, dvi_handle);
    post_id_byte = tt_get_unsigned_byte(dvi_handle);
    if (!(post_id_byte == DVI_ID || post_id_byte == DVIV_ID || post_id_byte == XDV_ID || post_id_byte == XDV_ID_OLD)) {
        dpx_message("DVI ID = %d\n", post_id_byte);
        _tt_abort(invalid_signature);
    }
    check_id_bytes();
    if (has_ptex && post_id_byte != DVIV_ID)
        _tt_abort("Saw opcode %i in DVI file not for Ascii pTeX", PTEXDIR);

    num_pages = 0; /* force loop to terminate */
}

/* Most of the work of actually interpreting
 * the dvi file is here.
 */
void
dvi_do_page (double page_paper_height, double hmargin, double vmargin)
{
    unsigned char opcode;

    /* before this is called, we have scanned the page for papersize specials
       and the complete DVI data is now in dvi_page_buffer */
    dvi_page_buf_index = 0;

    /* DVI coordinate */
    dev_origin_x = hmargin;
    dev_origin_y = page_paper_height - vmargin;

    dvi_stack_depth = 0;
    for (;;) {
        opcode = get_buffered_unsigned_byte();

        if (opcode <= SET_CHAR_127) {
            dvi_set(opcode);
            continue;
        }

        /* If we are here, we have an opcode that is something
         * other than SET_CHAR.
         */
        if (opcode >= FNT_NUM_0 && opcode <= FNT_NUM_63) {
            do_fnt(opcode-FNT_NUM_0);
            continue;
        }

        switch (opcode) {
        case SET1: case SET2: case SET3:
            dvi_set(get_buffered_unsigned_num(opcode-SET1)); break;
        case SET4:
            _tt_abort("Multibyte (>24 bits) character not supported!");
            break;

        case SET_RULE:
            do_setrule();
            break;

        case PUT1: case PUT2: case PUT3:
            dvi_put(get_buffered_unsigned_num(opcode-PUT1)); break;
        case PUT4:
            _tt_abort("Multibyte (>24 bits) character not supported!");
            break;

        case PUT_RULE:
            do_putrule();
            break;

        case NOP:
            break;

        case BOP:
            do_bop();
            break;
        case EOP:
            do_eop();
            if (linear) {
                if ((opcode = tt_get_unsigned_byte(dvi_handle)) == POST)
                    check_postamble();
                else
                    ttstub_input_ungetc(dvi_handle, opcode);
            }
            return;

        case PUSH:
            dvi_push();
            if (lr_mode >= SKIMMING)
                lr_width_push();
            /* The following line needs to go here instead of in
             * dvi_push() since logical structure of document is
             * oblivous to virtual fonts. For example the last line on a
             * page could be at stack level 3 and the page footer should
             * be at stack level 3.  However, if the page footer contains
             * virtual fonts (or other nested constructions), it could
             * fool the link breaker into thinking it was a continuation
             * of the link */
            dvi_mark_depth();
            break;
        case POP:
            dpx_dvi_pop();
            if (lr_mode >= SKIMMING)
                lr_width_pop();
            /* Above explanation holds for following line too */
            dvi_mark_depth();
            break;

        case RIGHT1: case RIGHT2: case RIGHT3: case RIGHT4:
            dvi_right(get_buffered_signed_num(opcode-RIGHT1)); break;

        case W0: dvi_w0(); break;
        case W1: case W2: case W3: case W4:
            dvi_w(get_buffered_signed_num(opcode-W1)); break;

        case X0: dvi_x0(); break;
        case X1: case X2: case X3: case X4:
            dvi_x(get_buffered_signed_num(opcode-X1)); break;

        case DOWN1: case DOWN2: case DOWN3: case DOWN4:
            dvi_down(get_buffered_signed_num(opcode-DOWN1)); break;

        case Y0: dvi_y0(); break;
        case Y1: case Y2: case Y3: case Y4:
            dvi_y(get_buffered_signed_num(opcode-Y1)); break;

        case Z0: dvi_z0(); break;
        case Z1: case Z2: case Z3: case Z4:
            dvi_z(get_buffered_signed_num(opcode-Z1)); break;

        case FNT1: case FNT2: case FNT3: case FNT4:
            do_fnt(get_buffered_unsigned_num(opcode-FNT1)); break;

            /* Specials */
        case XXX1: case XXX2: case XXX3: case XXX4:
        {
            int32_t size = get_buffered_unsigned_num(opcode-XXX1);
            if (size < 0)
                dpx_warning("DVI: Special with %d bytes???", size);
            else
                do_xxx(size);
            break;
        }

        /* These should not occur - processed during pre-scanning */
        case FNT_DEF1: case FNT_DEF2: case FNT_DEF3: case FNT_DEF4:
            break;

            /* pTeX extension */
        case PTEXDIR:
            need_pTeX(opcode);
            do_dir();
            break;

            /* XeTeX extensions */
        case XDV_GLYPHS:
            need_XeTeX(opcode);
            do_glyphs(0);
            break;
        case XDV_TEXT_AND_GLYPHS:
            need_XeTeX(opcode);
            do_glyphs(1);
            break;
            /* should not occur - processed during pre-scanning */
        case XDV_NATIVE_FONT_DEF:
            need_XeTeX(opcode);
            break;
        case BEGIN_REFLECT:
            need_XeTeX(opcode);
            dvi_begin_reflect();
            break;
        case END_REFLECT:
            need_XeTeX(opcode);
            dvi_end_reflect();
            break;

        case POST:
            if (linear && !processing_page) {
                /* for linear processing, this means there are no more pages */
                num_pages = 0; /* force loop to terminate */
                return;
            }
            /* else fall through to error case */
        case PRE: case POST_POST:
            _tt_abort("Unexpected preamble or postamble in dvi file");
            break;
        default:
            _tt_abort("Unexpected opcode or DVI file ended prematurely");
        }
    }
}

double
dvi_init (const char *dvi_filename, double mag)
{
    int32_t post_location;

    if (!dvi_filename)
        _tt_abort("filename must be specified");

    dvi_handle = ttstub_input_open (dvi_filename, TTBC_FILE_FORMAT_PROGRAM_DATA, 0);
    if (dvi_handle == NULL)
        _tt_abort("cannot open \"%s\"", dvi_filename);

    /* DVI files are most easily read backwards by searching for post_post and
     * then post opcode.
     */
    post_location = find_post();
    get_dvi_info(post_location);
    do_scales(mag);
    get_page_info(post_location);
    get_comment();
    get_dvi_fonts(post_location);
    clear_state();

    dvi_page_buf_size = DVI_PAGE_BUF_CHUNK;
    dvi_page_buffer = NEW(dvi_page_buf_size, unsigned char);

    return dvi2pts;
}

void
dvi_close (void)
{
    unsigned int i;

    if (linear) {
        /* probably reading a pipe from xetex; consume any remaining data */
        while (ttstub_input_getc(dvi_handle) != EOF)
            ;
    }

    /* We add comment in dvi_close instead of dvi_init so user
     * has a change to overwrite it.  The docinfo dictionary is
     * treated as a write-once record.
     */

    /* Do some house cleaning */
    ttstub_input_close(dvi_handle);
    dvi_handle = NULL;

    if (def_fonts) {
        for (i = 0; i < num_def_fonts; i++) {
            def_fonts[i].font_name = mfree(def_fonts[i].font_name);
        }
        free(def_fonts);
    }
    def_fonts = NULL;

    page_loc = mfree(page_loc);
    num_pages = 0;

    for (i = 0; i < num_loaded_fonts; i++)
    {
        free(loaded_fonts[i].hvmt);

        loaded_fonts[i].hvmt = NULL;

        if (loaded_fonts[i].cffont != NULL)
            cff_close(loaded_fonts[i].cffont);

        loaded_fonts[i].cffont = NULL;
    }

    loaded_fonts = mfree(loaded_fonts);
    num_loaded_fonts = 0;

    vf_close_all_fonts();
    tfm_close_all ();

    if (dvi_page_buffer) {
        dvi_page_buffer = mfree(dvi_page_buffer);
        dvi_page_buf_size = 0;
    }
}

/* The following are need to implement virtual fonts
   According to documentation, the vf "subroutine"
   must have state pushed and must have
   w,v,y, and z set to zero.  The current font
   is determined by the virtual font header, which
   may be undefined */

static int saved_dvi_font[VF_NESTING_MAX];
static unsigned int num_saved_fonts = 0;

void
dvi_vf_init (int dev_font_id)
{
    dvi_push();

    dvi_state.w = 0; dvi_state.x = 0;
    dvi_state.y = 0; dvi_state.z = 0;

    /* do not reset dvi_state.d. */
    if (num_saved_fonts < VF_NESTING_MAX) {
        saved_dvi_font[num_saved_fonts++] = current_font;
    } else
        _tt_abort("Virtual fonts nested too deeply!");
    current_font = dev_font_id;
}

/* After VF subroutine is finished, we simply pop the DVI stack */
void
dvi_vf_finish (void)
{
    dpx_dvi_pop();
    if (num_saved_fonts > 0)
        current_font = saved_dvi_font[--num_saved_fonts];
    else {
        _tt_abort("Tried to pop an empty font stack");
    }
}


/* Scan various specials */
#include "dpx-dpxutil.h"

/* This need to allow 'true' prefix for unit and
 * length value must be divided by current magnification.
 */
/* XXX: there are four quasi-redundant versions of this; grp for K_UNIT__PT */
static int
read_length (double *vp, double mag, const char **pp, const char *endptr)
{
    char   *q;
    const char *p = *pp;
    double  v, u = 1.0;
    const char *_ukeys[] = {
#define K_UNIT__PT  0
#define K_UNIT__IN  1
#define K_UNIT__CM  2
#define K_UNIT__MM  3
#define K_UNIT__BP  4
#define K_UNIT__PC  5
#define K_UNIT__DD  6
#define K_UNIT__CC  7
#define K_UNIT__SP  8
        "pt", "in", "cm", "mm", "bp", "pc", "dd", "cc", "sp",
        NULL
    };
    int     k, error = 0;

    q = parse_float_decimal(&p, endptr);
    if (!q) {
        *vp = 0.0; *pp = p;
        return  -1;
    }

    v = atof(q);
    free(q);

    skip_white(&p, endptr);
    q = parse_c_ident(&p, endptr);
    if (q) {
        char *qq = q; /* remember this for free, because q may be advanced */
        if (strlen(q) >= strlen("true") &&
            !memcmp(q, "true", strlen("true"))) {
            u /= mag != 0.0 ? mag : 1.0; /* inverse magnify */
            q += strlen("true");
        }
        if (strlen(q) == 0) { /* "true" was a separate word from the units */
            free(qq);
            skip_white(&p, endptr);
            qq = q = parse_c_ident(&p, endptr);
        }
        if (q) {
            for (k = 0; _ukeys[k] && strcmp(_ukeys[k], q); k++);
            switch (k) {
            case K_UNIT__PT: u *= 72.0 / 72.27; break;
            case K_UNIT__IN: u *= 72.0; break;
            case K_UNIT__CM: u *= 72.0 / 2.54 ; break;
            case K_UNIT__MM: u *= 72.0 / 25.4 ; break;
            case K_UNIT__BP: u *= 1.0 ; break;
            case K_UNIT__PC: u *= 12.0 * 72.0 / 72.27 ; break;
            case K_UNIT__DD: u *= 1238.0 / 1157.0 * 72.0 / 72.27 ; break;
            case K_UNIT__CC: u *= 12.0 * 1238.0 / 1157.0 * 72.0 / 72.27 ; break;
            case K_UNIT__SP: u *= 72.0 / (72.27 * 65536) ; break;
            default:
                dpx_warning("Unknown unit of measure: %s", q);
                error = -1;
                break;
            }
            free(qq);
        }
        else {
            dpx_warning("Missing unit of measure after \"true\"");
            error = -1;
        }
    }

    *vp = v * u; *pp = p;
    return  error;
}

#include "dpx-pdfencrypt.h"

static int
scan_special_encrypt (int *key_bits, int32_t *permission, char *opassword, char *upassword,
                      const char **curptr, const char *endptr)
{
  int   error = 0;
  const char *p = *curptr;

  skip_white(&p, endptr);

  opassword[0] = '\0';
  upassword[0] = '\0';
  while (!error && p < endptr) {
    char  *kp = parse_c_ident(&p, endptr);
    if (!kp)
      break;
    else {
      pdf_obj *obj;
      skip_white(&p, endptr);
      if (!strcmp(kp, "ownerpw")) {
        if ((obj = parse_pdf_string(&p, endptr))) {
          if (pdf_string_value(obj)) {
            int str_length = (MAX_PWD_LEN - 1 > pdf_string_length(obj)
                ? pdf_string_length(obj) : MAX_PWD_LEN - 1);
            strncpy(opassword, pdf_string_value(obj), str_length);
            opassword[str_length] = '\0';
          }
          pdf_release_obj(obj);
        } else
          error = -1;
      } else if (!strcmp(kp, "userpw")) {
        if ((obj = parse_pdf_string(&p, endptr))) {
          if (pdf_string_value(obj)) {
            int str_length = (MAX_PWD_LEN - 1 > pdf_string_length(obj)
                ? pdf_string_length(obj) : MAX_PWD_LEN - 1);
            strncpy(upassword, pdf_string_value(obj), str_length);
            upassword[str_length] = '\0';
          }
          pdf_release_obj(obj);
        } else
          error = -1;
      } else if (!strcmp(kp, "length")) {
        if ((obj = parse_pdf_number(&p, endptr)) && PDF_OBJ_NUMBERTYPE(obj)) {
          *key_bits = (unsigned) pdf_number_value(obj);
        } else
          error = -1;
        if (obj)
          pdf_release_obj(obj);
      } else if (!strcmp(kp, "perm")) {
        if ((obj = parse_pdf_number(&p, endptr)) && PDF_OBJ_NUMBERTYPE(obj)) {
          *permission = (unsigned) pdf_number_value(obj);
        } else
          error = -1;
        if (obj)
          pdf_release_obj(obj);
      } else
        error = -1;
      free(kp);
    }
    skip_white(&p, endptr);
  }
  *curptr = p;

  return error;
}

static int
scan_special_trailerid (unsigned char *id1, unsigned char *id2,
                        const char **curptr, const char *endptr)
{
  int         error = 0;
  pdf_obj    *id_array;
  const char *p = *curptr;

  skip_white(&p, endptr);
  id_array = parse_pdf_array(&p, endptr, NULL);
  if (id_array) {
    if (pdf_array_length(id_array) == 2) {
      pdf_obj *tmp1, *tmp2;
      tmp1 = pdf_get_array(id_array, 0);
      tmp2 = pdf_get_array(id_array, 1);
      if (PDF_OBJ_STRINGTYPE(tmp1) && pdf_string_length(tmp1) == 16 &&
          PDF_OBJ_STRINGTYPE(tmp2) && pdf_string_length(tmp2) == 16) {
        memcpy(id1, pdf_string_value(tmp1), 16);
        memcpy(id2, pdf_string_value(tmp2), 16);
      } else {
        error = -1;
      }
    } else {
      error = -1;
    }
    pdf_release_obj(id_array);
  } else {
    error = -1;
  }
  skip_white(&p, endptr);
  *curptr = p;

  return error;
}

static int
scan_special (double *wd, double *ht, double *xo, double *yo, int *lm,
              int *majorversion, int *minorversion,
              int *enable_encryption, int *key_bits, int32_t *permission,
              char *opassword, char *upassword,
              int *has_id, unsigned char *id1, unsigned char *id2,
              const char *buf, uint32_t size)
{
    char  *q;
    const char *p = buf, *endptr;
    int    ns_pdf = 0, ns_dvipdfmx = 0, error = 0;
    double tmp;

    endptr = p + size;

    skip_white(&p, endptr);

    q = parse_c_ident(&p, endptr);
    if (streq_ptr(q, "pdf")) {
        skip_white(&p, endptr);
        if (p < endptr && *p == ':') {
            p++;
            skip_white(&p, endptr);
            free(q);
            q = parse_c_ident(&p, endptr); ns_pdf = 1;
        }
    }
    else if (streq_ptr(q, "x")) {
        skip_white(&p, endptr);
        if (p < endptr && *p == ':') {
            p++;
            skip_white(&p, endptr);
            free(q);
            q = parse_c_ident(&p, endptr);
        }
    }
    else if (streq_ptr(q, "dvipdfmx")) {
        skip_white(&p, endptr);
        if (p < endptr && *p == ':') {
            p++;
            skip_white(&p, endptr);
            free(q);
            q = parse_c_ident(&p, endptr); ns_dvipdfmx = 1;
        }
    }
    skip_white(&p, endptr);

    if (q) {
        if (streq_ptr(q, "landscape")) {
            *lm = 1;
        } else if (ns_pdf && streq_ptr(q, "pagesize")) {
            while (!error && p < endptr) {
                char  *kp = parse_c_ident(&p, endptr);
                if (!kp)
                    break;
                else {
                    skip_white(&p, endptr);
                    if (streq_ptr(kp, "width")) {
                        error = read_length(&tmp, dvi_tell_mag(), &p, endptr);
                        if (!error)
                            *wd = tmp * dvi_tell_mag();
                    } else if (streq_ptr(kp, "height")) {
                        error = read_length(&tmp, dvi_tell_mag(), &p, endptr);
                        if (!error)
                            *ht = tmp * dvi_tell_mag();
                    } else if (streq_ptr(kp, "xoffset")) {
                        error = read_length(&tmp, dvi_tell_mag(), &p, endptr);
                        if (!error)
                            *xo = tmp * dvi_tell_mag();
                    } else if (streq_ptr(kp, "yoffset")) {
                        error = read_length(&tmp, dvi_tell_mag(), &p, endptr);
                        if (!error)
                            *yo = tmp * dvi_tell_mag();
                    } else if (streq_ptr(kp, "default")) {
                        *wd = paper_width;
                        *ht = paper_height;
                        *lm = landscape_mode;
                        *xo = *yo = 72.0;
                    }
                    free(kp);
                }
                skip_white(&p, endptr);
            }
        } else if (streq_ptr(q, "papersize")) {
            char  qchr = 0;
            if (*p == '=') p++;
            skip_white(&p, endptr);
            if (p < endptr && (*p == '\'' || *p == '\"')) {
                qchr = *p; p++;
                skip_white(&p, endptr);
            }
            error = read_length(&tmp, 1.0, &p, endptr);
            if (!error) {
                double tmp1;

                skip_white(&p, endptr);
                if (p < endptr && *p == ',') {
                    p++; skip_white(&p, endptr);
                }
                error = read_length(&tmp1, 1.0, &p, endptr);
                if (!error) {
                    *wd = tmp;
                    *ht = tmp1;
                    skip_white(&p, endptr);
                }
            }
            if (!error && qchr) { /* Check if properly quoted */
                if (p >= endptr || *p != qchr)
                    error = -1;
            }
            if (error == 0) {
                paper_width  = *wd;
                paper_height = *ht;
            }
        } else if (minorversion && ns_pdf && streq_ptr(q, "minorversion")) {
            char *kv;
            if (*p == '=') p++;
            skip_white(&p, endptr);
            kv = parse_float_decimal(&p, endptr);
            if (kv) {
                *minorversion = (int)strtol(kv, NULL, 10);
                free(kv);
            }
        } else if (majorversion && ns_pdf && streq_ptr(q, "majorversion")) {
            char *kv;
            if (*p == '=') p++;
            skip_white(&p, endptr);
            kv = parse_float_decimal(&p, endptr);
            if (kv) {
                *majorversion = (int)strtol(kv, NULL, 10);
                free(kv);
            }
        } else if (enable_encryption && ns_pdf && streq_ptr(q, "encrypt")) {
            *enable_encryption = 1;
            error = scan_special_encrypt(key_bits, permission, opassword, upassword, &p, endptr);
        } else if (ns_dvipdfmx && streq_ptr(q, "config")) {
            dpx_warning("Tectonic does not support `config' special. Ignored.");
        } else if (has_id && id1 && id2 && ns_pdf && !strcmp(q, "trailerid")) {
            error = scan_special_trailerid(id1, id2, &p, endptr);
            if (error) {
                dpx_warning("Invalid argument for pdf:trailerid special.");
                *has_id = 0;
            } else {
                *has_id = 1;
            } 
        }
        free(q);
    }

    return  error;
}

static int buffered_page = -1;
void
dvi_scan_specials (int page_no,
                   double *page_width, double *page_height,
                   double *x_offset, double *y_offset, int *landscape,
                   int *majorversion, int *minorversion,
                   int *do_enc, int *key_bits, int32_t *permission,
                   char *owner_pw, char *user_pw,
                   int *has_id, unsigned char *id1, unsigned char *id2)
{
    uint32_t       offset;
    unsigned char  opcode;
    unsigned int len;

    if (page_no == buffered_page || num_pages == 0)
        return; /* because dvipdfmx wants to scan first page twice! */
    buffered_page = page_no;

    dvi_page_buf_index = 0;

    if (!linear) {
        if (page_no >= num_pages)
            _tt_abort("Invalid page number: %u", page_no);
        offset = page_loc[page_no];

        ttstub_input_seek (dvi_handle, offset, SEEK_SET);
    }

    while ((opcode = get_and_buffer_unsigned_byte(dvi_handle)) != EOP) {
        if (opcode <= SET_CHAR_127 ||
            (opcode >= FNT_NUM_0 && opcode <= FNT_NUM_63))
            continue;
        else if (opcode == XXX1 || opcode == XXX2 ||
                 opcode == XXX3 || opcode == XXX4) {
            uint32_t size = get_and_buffer_unsigned_byte(dvi_handle);
            switch (opcode) {
            case XXX4: size = size * 0x100u + get_and_buffer_unsigned_byte(dvi_handle);
                if (size > 0x7fff)
                    dpx_warning("Unsigned number starting with %x exceeds 0x7fffffff", size);
            case XXX3: size = size * 0x100u + get_and_buffer_unsigned_byte(dvi_handle);
            case XXX2: size = size * 0x100u + get_and_buffer_unsigned_byte(dvi_handle);
            default: break;
            }
            if (dvi_page_buf_index + size >= dvi_page_buf_size) {
                dvi_page_buf_size = (dvi_page_buf_index + size + DVI_PAGE_BUF_CHUNK);
                dvi_page_buffer = RENEW(dvi_page_buffer, dvi_page_buf_size, unsigned char);
            }
#define buf ((char*)(dvi_page_buffer + dvi_page_buf_index))
            if (ttstub_input_read(dvi_handle, buf, size) != size)
                _tt_abort("Reading DVI file failed!");
            if (scan_special(page_width, page_height, x_offset, y_offset, landscape,
                             majorversion, minorversion,
                             do_enc, key_bits, permission, owner_pw, user_pw,
                             has_id, id1, id2,
                             buf, size))
                dpx_warning("Reading special command failed: \"%.*s\"", size, buf);
#undef buf
            dvi_page_buf_index += size;
            continue;
        }

        /* Skipping... */
        switch (opcode) {
        case BOP:
            get_and_buffer_bytes(dvi_handle, 44);
            break;
        case NOP: case PUSH: case POP:
        case W0: case X0: case Y0: case Z0:
            break;
        case SET1: case PUT1: case RIGHT1:  case DOWN1:
        case W1: case X1: case Y1: case Z1: case FNT1:
            get_and_buffer_bytes(dvi_handle, 1);
            break;

        case SET2: case PUT2: case RIGHT2: case DOWN2:
        case W2: case X2: case Y2: case Z2: case FNT2:
            get_and_buffer_bytes(dvi_handle, 2);
            break;

        case SET3: case PUT3: case RIGHT3: case DOWN3:
        case W3: case X3: case Y3: case Z3: case FNT3:
            get_and_buffer_bytes(dvi_handle, 3);
            break;

        case SET4: case PUT4: case RIGHT4: case DOWN4:
        case W4: case X4: case Y4: case Z4: case FNT4:
            get_and_buffer_bytes(dvi_handle, 4);
            break;

        case SET_RULE: case PUT_RULE:
            get_and_buffer_bytes(dvi_handle, 8);
            break;

        case FNT_DEF1: case FNT_DEF2: case FNT_DEF3: case FNT_DEF4:
            do_fntdef(tt_get_unsigned_num(dvi_handle, opcode-FNT_DEF1));
            break;
        case XDV_GLYPHS:
            need_XeTeX(opcode);
            get_and_buffer_bytes(dvi_handle, 4);            /* width */
            len = get_and_buffer_unsigned_pair(dvi_handle); /* glyph count */
            get_and_buffer_bytes(dvi_handle, len * 10);     /* 2 bytes ID + 8 bytes x,y-location per glyph */
            break;
        case XDV_TEXT_AND_GLYPHS:
            need_XeTeX(opcode);
            len = get_and_buffer_unsigned_pair(dvi_handle); /* utf16 code unit count */
            get_and_buffer_bytes(dvi_handle, len * 2);      /* 2 bytes per code unit */
            get_and_buffer_bytes(dvi_handle, 4);            /* width */
            len = get_and_buffer_unsigned_pair(dvi_handle); /* glyph count */
            get_and_buffer_bytes(dvi_handle, len * 10);     /* 2 bytes ID + 8 bytes x,y-location per glyph */
            break;
        case XDV_NATIVE_FONT_DEF:
            need_XeTeX(opcode);
            do_native_font_def(tt_get_signed_quad(dvi_handle));
            break;
        case BEGIN_REFLECT:
        case END_REFLECT:
            need_XeTeX(opcode);
            break;

        case PTEXDIR:
            need_pTeX(opcode);
            get_and_buffer_bytes(dvi_handle, 1);
            break;

        case POST:
            if (linear && dvi_page_buf_index == 1) {
                /* this is actually an indication that we've reached the end of the input */
                return;
            }
            /* else fall through to error case */
        default: /* case PRE: case POST_POST: and others */
            _tt_abort("Unexpected opcode %d" /*"at pos=0x%x"*/, opcode);
            break;
        }
    }

    return;
}

void
dvi_reset_global_state(void)
{
    buffered_page = -1;
    num_def_fonts = 0;
    max_def_fonts = 0;
    compute_boxes = 0;
    link_annot = 1;
    num_loaded_fonts = 0; 
    max_loaded_fonts = 0;
}
