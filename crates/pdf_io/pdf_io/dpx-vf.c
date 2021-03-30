/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include "dpx-vf.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dvi.h"
#include "dpx-dvicodes.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
/* pdfdev... */
#include "dpx-pdfdev.h"
#include "dpx-tfm.h"

#define VF_ALLOC_SIZE  16u

#define VF_ID 202
#define FIX_WORD_BASE 1048576.0
#define TEXPT2PT (72.0/72.27)
#define FW2PT (TEXPT2PT/((double)(FIX_WORD_BASE)))

struct font_def {
    int32_t font_id /* id used internally in vf file */;
    uint32_t checksum, size, design_size;
    char *directory, *name;
    int tfm_id;  /* id returned by TFM module */
    int dev_id;  /* id returned by DEV module */
};

struct vf
{
    char *tex_name;
    spt_t ptsize;
    uint32_t design_size; /* A fixword-pts quantity */
    unsigned int num_dev_fonts, max_dev_fonts;
    struct font_def *dev_fonts;
    unsigned char **ch_pkt;
    uint32_t *pkt_len;
    unsigned int num_chars;
};

static struct vf *vf_fonts = NULL;
static unsigned int num_vf_fonts = 0;
static unsigned int max_vf_fonts = 0;


void
vf_reset_global_state(void)
{
    num_vf_fonts = 0;
    max_vf_fonts = 0;
    vf_fonts = NULL;
}

static void
read_header(rust_input_handle_t vf_handle, int thisfont)
{
    if (tt_get_unsigned_byte (vf_handle) != PRE || tt_get_unsigned_byte (vf_handle) != VF_ID) {
        fprintf (stderr, "VF file may be corrupt\n");
        return;
    }

    /* skip comment */
    tt_skip_bytes (tt_get_unsigned_byte (vf_handle), vf_handle);

    /* Skip checksum */
    tt_skip_bytes (4, vf_handle);
    vf_fonts[thisfont].design_size = tt_get_positive_quad(vf_handle, "VF", "design_size");
}


static void resize_vf_fonts(int size)
{
    int i;
    if (size > max_vf_fonts) {
        vf_fonts = RENEW (vf_fonts, size, struct vf);
        for (i=max_vf_fonts; i<size; i++) {
            vf_fonts[i].num_dev_fonts = 0;
            vf_fonts[i].max_dev_fonts = 0;
            vf_fonts[i].dev_fonts = NULL;
        }
        max_vf_fonts = size;
    }
    return;
}

static void resize_one_vf_font (struct vf *a_vf, unsigned int size)
{
    unsigned int i;
    if (size > (a_vf->num_chars)) {
        size = MAX (size, a_vf->num_chars+256);
        a_vf->ch_pkt = RENEW (a_vf->ch_pkt, size, unsigned char *);
        a_vf->pkt_len = RENEW (a_vf->pkt_len, size, uint32_t);
        for (i=a_vf->num_chars; i<size; i++) {
            (a_vf->ch_pkt)[i] = NULL;
            (a_vf->pkt_len)[i] = 0;
        }
        a_vf->num_chars = size;
    }
}


static void
read_a_char_def(rust_input_handle_t vf_handle, int thisfont, uint32_t pkt_len, uint32_t ch)
{
    unsigned char *pkt;

    /* Resize and initialize character arrays if necessary */
    if (ch >= vf_fonts[thisfont].num_chars)
        resize_one_vf_font (vf_fonts + thisfont, ch + 1);

    if (pkt_len > 0) {
        pkt = NEW (pkt_len, unsigned char);
        if (ttstub_input_read (vf_handle, (char *) pkt, pkt_len) != pkt_len)
            _tt_abort("VF file ended prematurely.");
        vf_fonts[thisfont].ch_pkt[ch] = pkt;
    }

    vf_fonts[thisfont].pkt_len[ch] = pkt_len;
}


static void
read_a_font_def(rust_input_handle_t vf_handle, int32_t font_id, int thisfont)
{
    struct font_def *dev_font;
    int dir_length, name_length;

    if (vf_fonts[thisfont].num_dev_fonts >= vf_fonts[thisfont].max_dev_fonts) {
        vf_fonts[thisfont].max_dev_fonts += VF_ALLOC_SIZE;
        vf_fonts[thisfont].dev_fonts = RENEW (vf_fonts[thisfont].dev_fonts,
                                              vf_fonts[thisfont].max_dev_fonts,
                                              struct font_def);
    }

    dev_font = vf_fonts[thisfont].dev_fonts + vf_fonts[thisfont].num_dev_fonts;
    dev_font->font_id = font_id;
    dev_font->checksum = tt_get_unsigned_quad (vf_handle);
    dev_font->size = tt_get_positive_quad (vf_handle, "VF", "font_size");
    dev_font->design_size = tt_get_positive_quad (vf_handle, "VF", "font_design_size");
    dir_length = tt_get_unsigned_byte (vf_handle);
    name_length = tt_get_unsigned_byte (vf_handle);

    dev_font->directory = NEW (dir_length+1, char);
    if (ttstub_input_read (vf_handle, dev_font->directory, dir_length) != dir_length)
        _tt_abort("directory read failed");

    dev_font->name = NEW (name_length+1, char);
    if (ttstub_input_read (vf_handle, dev_font->name, name_length) != name_length)
        _tt_abort("directory read failed");

    dev_font->directory[dir_length] = 0;
    dev_font->name[name_length] = 0;
    vf_fonts[thisfont].num_dev_fonts += 1;

    dev_font->tfm_id = tfm_open (dev_font->name, 1); /* must exist */
    dev_font->dev_id = dvi_locate_font (dev_font->name,
                                        sqxfw (vf_fonts[thisfont].ptsize, dev_font->size));
}


static void
process_vf_file (rust_input_handle_t vf_handle, int thisfont)
{
    int eof = 0, code;
    uint32_t font_id;

    while (!eof) {
        code = tt_get_unsigned_byte (vf_handle);

        switch (code) {
        case FNT_DEF1: case FNT_DEF2: case FNT_DEF3: case FNT_DEF4:
            font_id = tt_get_unsigned_num (vf_handle, code - FNT_DEF1);
            read_a_font_def (vf_handle, font_id, thisfont);
            break;

        default:
            if (code < 242) {
                /* For a short packet, code is the pkt_len */
                uint32_t ch = tt_get_unsigned_byte (vf_handle);
                /* Skip over TFM width since we already know it */
                tt_skip_bytes (3, vf_handle);
                read_a_char_def (vf_handle, thisfont, code, ch);
                break;
            }

            if (code == 242) {
                uint32_t pkt_len = tt_get_positive_quad (vf_handle, "VF", "pkt_len");
                uint32_t ch = tt_get_unsigned_quad (vf_handle);
                /* Skip over TFM width since we already know it */
                tt_skip_bytes (4, vf_handle);
                if (ch < 0x1000000U)
                    read_a_char_def (vf_handle, thisfont, pkt_len, ch);
                else {
                    fprintf (stderr, "char=%u\n", ch);
                    _tt_abort("Long character (>24 bits) in VF file.\nI can't handle long characters!\n");
                }
                break;
            }
            if (code == POST) {
                eof = 1;
                break;
            }
            fprintf (stderr, "Quitting on code=%d\n", code);
            eof = 1;
            break;
        }
    }
}

/* Unfortunately, the following code isn't smart enough
   to load the vf only once for multiple point sizes.
   You will get a separate copy of each VF in memory (and a separate
   opening and reading of the file) for
   each point size.  Since VFs are pretty small, I guess
   this is tolerable for now.  In any case,
   the PDF file will never repeat a physical font name */
/* Note: This code needs to be able to recurse */
/* Global variables such as num_vf_fonts require careful attention */
int vf_locate_font (const char *tex_name, spt_t ptsize)
{
    int thisfont = -1, i;
    rust_input_handle_t vf_handle = NULL;

    /* Has this name and ptsize already been loaded as a VF? */
    for (i = 0; i < num_vf_fonts; i++) {
        if (streq_ptr(vf_fonts[i].tex_name, tex_name) && vf_fonts[i].ptsize == ptsize)
            break;
    }

    if (i != num_vf_fonts)
        return i;

    vf_handle = ttstub_input_open (tex_name, TTBC_FILE_FORMAT_VF, 0);

    if (vf_handle == NULL)
        vf_handle = ttstub_input_open (tex_name, TTBC_FILE_FORMAT_OVF, 0);

    if (vf_handle == NULL)
        return -1;

    if (dpx_conf.verbose_level > 0)
        fprintf (stderr, "(VF:%s", tex_name);

    if (num_vf_fonts >= max_vf_fonts)
        resize_vf_fonts (max_vf_fonts + VF_ALLOC_SIZE);

    thisfont = num_vf_fonts++;

    /* Initialize some pointers and such */
    vf_fonts[thisfont].tex_name = NEW (strlen(tex_name)+1, char);
    strcpy (vf_fonts[thisfont].tex_name, tex_name);
    vf_fonts[thisfont].ptsize = ptsize;
    vf_fonts[thisfont].num_chars = 0;
    vf_fonts[thisfont].ch_pkt = NULL;
    vf_fonts[thisfont].pkt_len = NULL;

    read_header(vf_handle, thisfont);
    process_vf_file (vf_handle, thisfont);

    if (dpx_conf.verbose_level > 0)
        fprintf (stderr, ")");

    ttstub_input_close (vf_handle);
    return thisfont;
}


#define next_byte() (*((*start)++))
static int unsigned_byte (unsigned char **start, unsigned char *end)
{
    int byte = 0;
    if (*start < end)
        byte = next_byte();
    else
        _tt_abort("Premature end of DVI byte stream in VF font\n");
    return byte;
}

static int32_t get_pkt_signed_num (unsigned char **start, unsigned char *end,
                                   unsigned char num)
{
    int32_t val = 0;
    if (end-*start > num) {
        val = next_byte();
        if (val > 0x7f)
            val -= 0x100;
        switch (num) {
        case 3: val = (val << 8) | next_byte();
        case 2: val = (val << 8) | next_byte();
        case 1: val = (val << 8) | next_byte();
        default: break;
        }
    } else
        _tt_abort("Premature end of DVI byte stream in VF font\n");
    return val;
}

static int32_t get_pkt_unsigned_num (unsigned char **start, unsigned char *end,
                                     unsigned char num)
{
    int32_t val = 0;
    if (end-*start > num) {
        val = next_byte();
        switch (num) {
        case 3: if (val > 0x7f)
                val -= 0x100;
            val = (val << 8) | next_byte();
        case 2: val = (val << 8) | next_byte();
        case 1: val = (val << 8) | next_byte();
        default: break;
        }
    } else
        _tt_abort("Premature end of DVI byte stream in VF font\n");
    return val;
}

static void vf_putrule (unsigned char **start, unsigned char *end, spt_t ptsize)
{
    int32_t height = get_pkt_signed_num (start, end, 3);
    int32_t width = get_pkt_signed_num (start, end, 3);
    dvi_rule (sqxfw (ptsize, width), sqxfw (ptsize, height));
}

static void vf_setrule (unsigned char **start, unsigned char *end, spt_t ptsize)
{
    int32_t height = get_pkt_signed_num (start, end, 3);
    int32_t s_width = sqxfw (ptsize, get_pkt_signed_num (start, end, 3));
    dvi_rule (s_width, sqxfw (ptsize, height));
    dvi_right (s_width);
}

static void vf_fnt (int32_t font_id, int vf_font)
{
    int i;
    for (i=0; i<vf_fonts[vf_font].num_dev_fonts; i++) {
        if (font_id == ((vf_fonts[vf_font].dev_fonts)[i]).font_id) {
            break;
        }
    }
    if (i < vf_fonts[vf_font].num_dev_fonts) { /* Font was found */
        dvi_set_font ((vf_fonts[vf_font].dev_fonts[i]).dev_id);
    } else {
        fprintf (stderr, "Font_id: %d not found in VF\n", font_id);
    }
}

/* identical to do_xxx in dvi.c */
static void vf_xxx (int32_t len, unsigned char **start, unsigned char *end)
{
    if (*start <= end - len) {
        unsigned char *buffer = NEW(len+1, unsigned char);
        memcpy(buffer, *start, len);
        buffer[len] = '\0';
        {
            unsigned char *p = buffer;

            while (p < buffer+len && *p == ' ') p++;
            /*
             * Warning message from virtual font.
             */
            if (!memcmp((char *)p, "Warning:", 8)) {
                if (dpx_conf.verbose_level > 0)
                    dpx_warning("VF:%s", p+8);
            } else {
                dvi_do_special(buffer, len);
            }
        }
        free(buffer);
    } else {
        _tt_abort("Premature end of DVI byte stream in VF font.");
    }

    *start += len;
    return;
}

void vf_set_char(int32_t ch, int vf_font)
{
    unsigned char opcode;
    unsigned char *start, *end;
    spt_t ptsize;
    int default_font = -1;
    if (vf_font < num_vf_fonts) {
        /* Initialize to the first font or -1 if undefined */
        ptsize = vf_fonts[vf_font].ptsize;
        if (vf_fonts[vf_font].num_dev_fonts > 0)
            default_font = ((vf_fonts[vf_font].dev_fonts)[0]).dev_id;
        dvi_vf_init (default_font);
        if (ch >= vf_fonts[vf_font].num_chars ||
            !(start = (vf_fonts[vf_font].ch_pkt)[ch])) {
            fprintf (stderr, "\nchar=0x%x(%d)\n", ch, ch);
            fprintf (stderr, "Tried to set a nonexistent character in a virtual font");
            start = end = NULL;
        } else {
            end = start + (vf_fonts[vf_font].pkt_len)[ch];
        }
        while (start && start < end) {
            opcode = *(start++);
#ifdef DEBUG
            fprintf (stderr, "VF opcode: %d", opcode);
            if (isprint (opcode)) fprintf (stderr, " (\'%c\')\n", opcode);
            else fprintf (stderr, "\n");
#endif
            switch (opcode)
            {
            case SET1: case SET2: case SET3:
                dvi_set (get_pkt_unsigned_num (&start, end, opcode-SET1));
                break;
            case SET4:
                _tt_abort("Multibyte (>24 bits) character in VF packet.\nI can't handle this!");
                break;
            case SET_RULE:
                vf_setrule(&start, end, ptsize);
                break;
            case PUT1: case PUT2: case PUT3:
                dvi_put (get_pkt_unsigned_num (&start, end, opcode-PUT1));
                break;
            case PUT4:
                _tt_abort("Multibyte (>24 bits) character in VF packet.\nI can't handle this!");
                break;
            case PUT_RULE:
                vf_putrule(&start, end, ptsize);
                break;
            case NOP:
                break;
            case PUSH:
                dvi_push();
                break;
            case POP:
                dpx_dvi_pop();
                break;
            case RIGHT1: case RIGHT2: case RIGHT3: case RIGHT4:
                dvi_right (sqxfw (ptsize, get_pkt_signed_num (&start, end, opcode-RIGHT1)));
                break;
            case W0:
                dvi_w0();
                break;
            case W1: case W2: case W3: case W4:
                dvi_w (sqxfw (ptsize, get_pkt_signed_num (&start, end, opcode-W1)));
                break;
            case X0:
                dvi_x0();
                break;
            case X1: case X2: case X3: case X4:
                dvi_x (sqxfw (ptsize, get_pkt_signed_num (&start, end, opcode-X1)));
                break;
            case DOWN1: case DOWN2: case DOWN3: case DOWN4:
                dvi_down (sqxfw (ptsize, get_pkt_signed_num (&start, end, opcode-DOWN1)));
                break;
            case Y0:
                dvi_y0();
                break;
            case Y1: case Y2: case Y3: case Y4:
                dvi_y (sqxfw (ptsize, get_pkt_signed_num (&start, end, opcode-Y1)));
                break;
            case Z0:
                dvi_z0();
                break;
            case Z1: case Z2: case Z3: case Z4:
                dvi_z (sqxfw (ptsize, get_pkt_signed_num (&start, end, opcode-Z1)));
                break;
            case FNT1: case FNT2: case FNT3: case FNT4:
                vf_fnt (get_pkt_signed_num (&start, end, opcode-FNT1), vf_font);
                break;
            case XXX1: case XXX2: case XXX3: case XXX4:
            {
                int32_t len = get_pkt_unsigned_num (&start, end, opcode-XXX1);
                if (len < 0)
                    dpx_warning("VF: Special with %d bytes???", len);
                else
                    vf_xxx (len, &start, end);
            }
            break;
            case PTEXDIR:
                dvi_dirchg (unsigned_byte (&start, end));
                break;
            default:
                if (opcode <= SET_CHAR_127) {
                    dvi_set (opcode);
                } else if (opcode >= FNT_NUM_0 && opcode <= FNT_NUM_63) {
                    vf_fnt (opcode - FNT_NUM_0, vf_font);
                } else {
                    fprintf (stderr, "Unexpected opcode: %d\n", opcode);
                    _tt_abort("Unexpected opcode in vf file\n");
                }
            }
        }
        dvi_vf_finish();
    } else {
        fprintf (stderr, "vf_set_char: font: %d", vf_font);
        _tt_abort("Font not loaded\n");
    }
    return;
}


void vf_close_all_fonts(void)
{
    unsigned int i, j;
    struct font_def *one_font;
    for (i=0; i<num_vf_fonts; i++) {
        /* Release the packet for each character */
        if (vf_fonts[i].ch_pkt) {
            for (j=0; j<vf_fonts[i].num_chars; j++) {
                free ((vf_fonts[i].ch_pkt)[j]);
            }
            free (vf_fonts[i].ch_pkt);
        }
        free (vf_fonts[i].pkt_len);
        free (vf_fonts[i].tex_name);
        /* Release each font record */
        for (j=0; j<vf_fonts[i].num_dev_fonts; j++) {
            one_font = &(vf_fonts[i].dev_fonts)[j];
            free (one_font -> directory);
            free (one_font -> name);
        }
        free (vf_fonts[i].dev_fonts);
    }
    free (vf_fonts);
    return;
}
