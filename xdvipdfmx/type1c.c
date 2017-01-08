/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2008-2016 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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

/*
 * CFF/OpenType Font support:
 *
 *  Adobe Technical Note #5176, "The Compact Font Format Specfication"
 *
 * NOTE:
 *
 *  Many CFF/OpenType does not have meaningful/correct CFF encoding.
 *  Encoding should be expilicitly supplied in the fontmap.
 *
 */ 

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"

#include "dpxfile.h"

#include "pdfobj.h"
#include "pdffont.h"

#include "pdfencoding.h"
#include "unicode.h"

/* Font info. from OpenType tables */
#include "sfnt.h"
#include "tt_aux.h"

#include "cff_types.h"
#include "cff_limits.h"
#include "cff.h"
#include "cff_dict.h"
#include "cs_type2.h"

#include "type1c.h"

#include "tfm.h"

int
pdf_font_open_type1c (pdf_font *font)
{
  char     *fontname;
  char     *ident;
  FILE     *fp = NULL;
  sfnt     *sfont;
  cff_font *cffont;
  pdf_obj  *descriptor, *tmp;
  unsigned  offset = 0;
  int       encoding_id, embedding;

  ASSERT(font);

  ident       = pdf_font_get_ident   (font);
  encoding_id = pdf_font_get_encoding(font);

  fp = DPXFOPEN(ident, DPX_RES_TYPE_OTFONT);
  if (!fp)
    return -1;

  sfont = sfnt_open(fp);
  if (!sfont ||
      sfont->type != SFNT_TYPE_POSTSCRIPT     ||
      sfnt_read_table_directory(sfont, 0) < 0) {
    ERROR("Not a CFF/OpenType font (9)?");
  }

  offset = sfnt_find_table_pos(sfont, "CFF ");
  if (offset < 1) {
    ERROR("No \"CFF \" table found; not a CFF/OpenType font (10)?");
  }

  cffont = cff_open(sfont->stream, offset, 0);
  if (!cffont) {
    ERROR("Could not read CFF font data");
  }

  if (cffont->flag & FONTTYPE_CIDFONT) {
    cff_close (cffont);
    sfnt_close(sfont);
    if (fp)
      DPXFCLOSE(fp);
    return -1;
  }

  fontname = cff_get_name(cffont);
  if (!fontname) {
    ERROR("No valid FontName found in CFF/OpenType font.");
  }
  pdf_font_set_fontname(font, fontname);
  RELEASE(fontname);

  cff_close(cffont);

  /*
   * Font like AdobePiStd does not have meaningful built-in encoding.
   * Some software generate CFF/OpenType font with incorrect encoding.
   */
  if (encoding_id < 0) {
    WARN("Built-in encoding used for CFF/OpenType font.");
    WARN("CFF font in OpenType font sometimes have strange built-in encoding.");
    WARN("If you find text is not encoded properly in the generated PDF file,");
    WARN("please specify appropriate \".enc\" file in your fontmap.");
  }
  pdf_font_set_subtype (font, PDF_FONT_FONTTYPE_TYPE1C);

  embedding  = pdf_font_get_flag(font, PDF_FONT_FLAG_NOEMBED) ? 0 : 1;
  descriptor = pdf_font_get_descriptor(font);
  /*
   * Create font descriptor from OpenType tables.
   * We can also use CFF TOP DICT/Private DICT for this.
   */
  tmp = tt_get_fontdesc(sfont, &embedding, -1, 1, fontname);
  if (!tmp) {
    ERROR("Could not obtain neccesary font info from OpenType table.");
    return -1;
  }
  pdf_merge_dict (descriptor, tmp); /* copy */
  pdf_release_obj(tmp);
  if (!embedding) { /* tt_get_fontdesc may have changed this */
    pdf_font_set_flags(font, PDF_FONT_FLAG_NOEMBED);
  }

  sfnt_close(sfont);
  if (fp)
    DPXFCLOSE(fp);

  return 0;
}

static void
add_SimpleMetrics (pdf_font *font, cff_font *cffont,
		   double *widths, card16 num_glyphs)
{
  pdf_obj *fontdict;
  int      code, firstchar, lastchar, tfm_id;
  char    *usedchars;
  pdf_obj *tmp_array;
  double   scaling;

  fontdict  = pdf_font_get_resource(font);
  usedchars = pdf_font_get_usedchars(font);

  /* The widhts array in the font dictionary must be given relative
   * to the default scaling of 1000:1, not relative to the scaling
   * given by the font matrix.
   */
  if (cff_dict_known(cffont->topdict, "FontMatrix"))
    scaling = 1000*cff_dict_get(cffont->topdict, "FontMatrix", 0);
  else
    scaling = 1;

  tmp_array = pdf_new_array();
  if (num_glyphs <= 1) {
    /* This should be error. */
    firstchar = lastchar = 0;
    pdf_add_array(tmp_array, pdf_new_number(0.0));
  } else {
    firstchar = 255; lastchar = 0;
    for (code = 0; code < 256; code++) {
      if (usedchars[code]) {
	if (code < firstchar) firstchar = code;
	if (code > lastchar)  lastchar  = code;
      }
    }
    if (firstchar > lastchar) {
      ERROR("No glyphs used at all!");
      pdf_release_obj(tmp_array);
      return;
    }
    tfm_id = tfm_open(pdf_font_get_mapname(font), 0);
    for (code = firstchar; code <= lastchar; code++) {
      if (usedchars[code]) {
        double width;
        if (tfm_id < 0) /* tfm is not found */
          width = scaling * widths[code];
        else {
          double diff;
          width = 1000. * tfm_get_width(tfm_id, code);
          diff  = width - scaling * widths[code];
          if (fabs(diff) > 1.) {
            WARN("Glyph width mismatch for TFM and font (%s)",
                 pdf_font_get_mapname(font));
            WARN("TFM: %g vs. CFF font: %g", width, widths[code]);
            }
	pdf_add_array(tmp_array,
		      pdf_new_number(ROUND(width, 0.1)));
        }
      } else {
	pdf_add_array(tmp_array, pdf_new_number(0.0));
      }
    }
  }

  if (pdf_array_length(tmp_array) > 0) {
    pdf_add_dict(fontdict,
		 pdf_new_name("Widths"),  pdf_ref_obj(tmp_array));
  }
  pdf_release_obj(tmp_array);

  pdf_add_dict(fontdict,
	       pdf_new_name("FirstChar"), pdf_new_number(firstchar));
  pdf_add_dict(fontdict,
	       pdf_new_name("LastChar"),  pdf_new_number(lastchar));

  return;
}

int
pdf_font_load_type1c (pdf_font *font)
{
  pdf_obj      *fontdict, *descriptor;
  pdf_obj      *pdfcharset; /* Actually string object */
  char         *usedchars;
  char         *fontname, *uniqueTag, *ident, *fullname;
  FILE         *fp = NULL;
  int           encoding_id;
  pdf_obj      *fontfile, *stream_dict;
  char        **enc_vec;
  sfnt         *sfont;
  cff_font     *cffont;
  cff_index    *charstrings, *topdict, *cs_idx;
  cff_charsets *charset  = NULL;
  cff_encoding *encoding = NULL;
  int           topdict_offset, private_size;
  int           charstring_len, max_len;
  int           size, offset = 0;
  int           stream_data_len = 0;
  card8        *stream_data_ptr, *data;
  card16        num_glyphs, cs_count, code;
  cs_ginfo      ginfo;
  double        nominal_width, default_width, notdef_width;
  double        widths[256];
  int           verbose;

  ASSERT(font);

  verbose = pdf_font_get_verbose();

  if (!pdf_font_is_in_use(font)) {
    return 0;
  }

  if (pdf_font_get_flag(font, PDF_FONT_FLAG_NOEMBED)) {
    ERROR("Only embedded font supported for CFF/OpenType font.");
  }

  usedchars = pdf_font_get_usedchars (font);
  fontname  = pdf_font_get_fontname  (font);
  ident     = pdf_font_get_ident     (font);
  uniqueTag = pdf_font_get_uniqueTag (font);
  if (!usedchars ||
      !fontname  || !ident) {
    ERROR("Unexpected error....");
  }

  fontdict    = pdf_font_get_resource  (font);
  descriptor  = pdf_font_get_descriptor(font);
  encoding_id = pdf_font_get_encoding  (font);

  fp = DPXFOPEN(ident, DPX_RES_TYPE_OTFONT);
  if (!fp) {
    ERROR("Could not open OpenType font: %s", ident);
  }

  sfont = sfnt_open(fp);
  if (!sfont) {
    ERROR("Could not open OpenType font: %s", ident);
  }
  if (sfnt_read_table_directory(sfont, 0) < 0) {
    ERROR("Could not read OpenType table directory: %s", ident);
  }
  if (sfont->type != SFNT_TYPE_POSTSCRIPT ||
      (offset = sfnt_find_table_pos(sfont, "CFF ")) == 0) {
    ERROR("Not a CFF/OpenType font (11)?");
  }

  cffont = cff_open(fp, offset, 0);
  if (!cffont) {
    ERROR("Could not open CFF font.");
  }
  if (cffont->flag & FONTTYPE_CIDFONT) {
    ERROR("This is CIDFont...");
  }

  fullname = NEW(strlen(fontname) + 8, char);
  sprintf(fullname, "%6s+%s", uniqueTag, fontname);

  /* Offsets from DICTs */
  cff_read_charsets(cffont);
  if (encoding_id < 0)
    cff_read_encoding(cffont);
  cff_read_private(cffont);
  cff_read_subrs  (cffont);

  /* FIXME */
  cffont->_string = cff_new_index(0);

  /* New Charsets data */
  charset = NEW(1, cff_charsets);
  charset->format      = 0;
  charset->num_entries = 0;
  charset->data.glyphs = NEW(256, s_SID);

  /*
   * Encoding related things.
   */
  enc_vec = NULL;
  if (encoding_id >= 0) {
    enc_vec = pdf_encoding_get_encoding(encoding_id);
  } else {
    pdf_obj *tounicode;

    /*
     * Create enc_vec and ToUnicode CMap for built-in encoding.
     */
    enc_vec = NEW(256, char *);
    for (code = 0; code < 256; code++) {
      if (usedchars[code]) {
	card16  gid;

	gid = cff_encoding_lookup(cffont, code);
	enc_vec[code] = cff_get_string(cffont,
				       cff_charsets_lookup_inverse(cffont, gid));
      } else {
	enc_vec[code] = NULL;
      }
    }
    if (!pdf_lookup_dict(fontdict, "ToUnicode")) {
      tounicode = pdf_create_ToUnicode_CMap(fullname,
					    enc_vec, usedchars);
      if (tounicode) {
	pdf_add_dict(fontdict,
                     pdf_new_name("ToUnicode"),
                     pdf_ref_obj (tounicode));
	pdf_release_obj(tounicode);
      }
    }
  }

  /*
   * New Encoding data:
   *
   *  We should not use format 0 here.
   *  The number of encoded glyphs (num_entries) is limited to 255 in format 0,
   *  and hence it causes problem for encodings that uses full 256 code-points.
   *  As we always sort glyphs by encoding, we can avoid this problem simply
   *  by using format 1; Using full range result in a single range, 0 255.
   *
   *  Creating actual encoding date is delayed to eliminate character codes to
   *  be mapped to .notdef and to handle multiply-encoded glyphs.
   */
  encoding = NEW(1, cff_encoding);
  encoding->format      = 1;
  encoding->num_entries = 0;
  encoding->data.range1 = NEW(255, cff_range1);
  encoding->num_supps   = 0;
  encoding->supp        = NEW(255, cff_map);

  /*
   * Charastrings.
   */
  offset = cff_dict_get(cffont->topdict, "CharStrings", 0);
  cff_seek_set(cffont, offset);
  cs_idx = cff_get_index_header(cffont);

  /* Offset is now absolute offset ... fixme */
  offset   = cff_tell(cffont);
  cs_count = cs_idx->count;
  if (cs_count < 2) {
    ERROR("No valid charstring data found.");
  }

  /* New CharStrings INDEX */
  charstrings       = cff_new_index(257);   /* 256 + 1 for ".notdef" glyph */
  max_len           = 2 * CS_STR_LEN_MAX;
  charstrings->data = NEW(max_len, card8);
  charstring_len    = 0;

  /*
   * Information from OpenType table is rough estimate. Replace with accurate value.
   */
  if (cffont->private[0] &&
      cff_dict_known(cffont->private[0], "StdVW")) {
    double stemv;

    stemv = cff_dict_get(cffont->private[0], "StdVW", 0);
    pdf_add_dict(descriptor,
		 pdf_new_name("StemV"), pdf_new_number(stemv));
  }
  
  /*
   * Widths
   */
  if (cffont->private[0] &&
      cff_dict_known(cffont->private[0], "defaultWidthX")) {
    default_width = (double) cff_dict_get(cffont->private[0], "defaultWidthX", 0);
  } else {
    default_width = CFF_DEFAULTWIDTHX_DEFAULT;
  }
  if (cffont->private[0] &&
      cff_dict_known(cffont->private[0], "nominalWidthX")) {
    nominal_width = (double) cff_dict_get(cffont->private[0], "nominalWidthX", 0);
  } else {
    nominal_width = CFF_NOMINALWIDTHX_DEFAULT;
  }

  data = NEW(CS_STR_LEN_MAX, card8);

  /* First we add .notdef glyph.
   * All Type 1 font requires .notdef glyph to be present.
   */
  if (verbose > 2) {
    MESG("[glyphs:/.notdef");
  }
  size = cs_idx->offset[1] - cs_idx->offset[0];
  if (size > CS_STR_LEN_MAX) {
    ERROR("Charstring too long: gid=%u, %ld bytes", 0, size);
  }
  charstrings->offset[0] = charstring_len + 1;
  cff_seek(cffont, offset + cs_idx->offset[0] - 1);
  cff_read_data(data, size, cffont);
  charstring_len += cs_copy_charstring(charstrings->data + charstring_len,
				       max_len - charstring_len,
				       data, size,
				       cffont->gsubr, cffont->subrs[0],
				       default_width, nominal_width, &ginfo);
  notdef_width = ginfo.wx;

  /*
   * Subset font
   */
  num_glyphs = 1;
  pdfcharset = pdf_new_stream(0);
  for (code = 0; code < 256; code++) {
    card16 gid, j;
    s_SID  sid_orig, sid;

    widths[code] = notdef_width;

    if (!usedchars[code] || !enc_vec[code] ||
	!strcmp(enc_vec[code], ".notdef"))
      continue;

    /*
     * FIXME:
     *  cff_get_sid() obtain SID from original String INDEX.
     *  It should be cff_string_get_sid(string, ...).
     *  cff_add_string(cff, ...) -> cff_string_add(string, ...).
     */
    sid_orig = cff_get_sid   (cffont, enc_vec[code]);
    sid      = sid_orig < CFF_STDSTR_MAX ?
                 sid_orig : cff_add_string(cffont, enc_vec[code], 0);
    /*
     * We use "unique = 0" because duplicate strings are impossible
     * at this stage unless the original font already had duplicates.
     */

    /*
     * Check if multiply-encoded glyph.
     */
    for (j = 0; j < charset->num_entries; j++) {
      if (sid == charset->data.glyphs[j]) {
	/* Already have this glyph. */
	encoding->supp[encoding->num_supps].code  = code;
	encoding->supp[encoding->num_supps].glyph = sid;
	usedchars[code] = 0; /* Used but multiply-encoded. */
	encoding->num_supps += 1;
	break;
      }
    }
    if (j < charset->num_entries) {
      continue; /* Prevent duplication. */
    }

    /* This is new encoding entry. */
    gid = cff_charsets_lookup(cffont, sid_orig); /* FIXME */
    if (gid == 0) {
      WARN("Glyph \"%s\" missing in font \"%s\".", enc_vec[code], fontname);
      WARN("Maybe incorrect encoding specified.");
      usedchars[code] = 0; /* Set unused for writing correct encoding */
      continue;
    }
    pdf_add_stream(pdfcharset, "/", 1);
    pdf_add_stream(pdfcharset, enc_vec[code], strlen(enc_vec[code]));

    if (verbose > 2) {
      MESG("/%s", enc_vec[code]);
    }

    size = cs_idx->offset[gid+1] - cs_idx->offset[gid];
    if (size > CS_STR_LEN_MAX) {
      ERROR("Charstring too long: gid=%u, %ld bytes", gid, size);
    }

    if (charstring_len + CS_STR_LEN_MAX >= max_len) {
      max_len = charstring_len + 2 * CS_STR_LEN_MAX;
      charstrings->data = RENEW(charstrings->data, max_len, card8);
    }
    charstrings->offset[num_glyphs] = charstring_len + 1;
    cff_seek(cffont, offset + cs_idx->offset[gid] - 1);
    cff_read_data(data, size, cffont);
    charstring_len += cs_copy_charstring(charstrings->data + charstring_len,
					 max_len - charstring_len,
					 data, size,
					 cffont->gsubr, cffont->subrs[0],
					 default_width, nominal_width, &ginfo);
    widths[code] = ginfo.wx;
    charset->data.glyphs[charset->num_entries] = sid;
    charset->num_entries  += 1;
    num_glyphs++;
  }
  if (verbose > 2) {
    MESG("]");
  }
  RELEASE(data);

  /*
   * Now we create encoding data.
   */
  if (encoding->num_supps > 0)
    encoding->format |= 0x80; /* Have supplemantary data. */
  else {
    RELEASE(encoding->supp); /* FIXME */
  }
  for (code = 0; code < 256; code++) {
    if (!usedchars[code] ||
	!enc_vec[code]   || !strcmp(enc_vec[code], ".notdef"))
      continue;
    encoding->data.range1[encoding->num_entries].first  = code;
    encoding->data.range1[encoding->num_entries].n_left = 0;
    code++;
    while (code < 256 && usedchars[code] &&
	   enc_vec[code] && strcmp(enc_vec[code], ".notdef")) {
      encoding->data.range1[encoding->num_entries].n_left += 1;
      code++;
    }
    encoding->num_entries += 1;
    /* The above while() loop stopped at unused char or code == 256. */
  }
  
  /* cleanup */
  if (encoding_id < 0 && enc_vec) {
    for (code = 0; code < 256; code++) {
      if (enc_vec[code]) {
	RELEASE(enc_vec[code]);
      }
    }
    RELEASE(enc_vec);
  }

  cff_release_index(cs_idx);

  charstrings->offset[num_glyphs] = charstring_len + 1;
  charstrings->count = num_glyphs;
  charstring_len     = cff_index_size(charstrings);
  cffont->num_glyphs = num_glyphs;

  /*
   * Discard old one, set new data.
   */
  if (cffont->charsets)
    cff_release_charsets(cffont->charsets);
  cffont->charsets = charset;
  if (cffont->encoding)
    cff_release_encoding(cffont->encoding);
  cffont->encoding = encoding;
  /*
   * We don't use subroutines at all.
   */
  if (cffont->gsubr)
    cff_release_index(cffont->gsubr);
  cffont->gsubr = cff_new_index(0);
  if (cffont->subrs[0])
    cff_release_index(cffont->subrs[0]);
  cffont->subrs[0] = NULL;

  /*
   * Flag must be reset since cff_pack_encoding(charset) does not write
   * encoding(charset) if HAVE_STANDARD_ENCODING(CHARSET) is set. We are
   * re-encoding font.
   */
  cffont->flag = FONTTYPE_FONT;

  /*
   * FIXME:
   *  Update String INDEX to delete unused strings.
   */
  cff_dict_update(cffont->topdict, cffont);
  if (cffont->private[0])
    cff_dict_update(cffont->private[0], cffont);
  cff_update_string(cffont);

  /*
   * Calculate sizes of Top DICT and Private DICT.
   * All offset values in DICT are set to long (32-bit) integer
   * in cff_dict_pack(), those values are updated later.
   */
  topdict = cff_new_index(1);

  cff_dict_remove(cffont->topdict, "UniqueID");
  cff_dict_remove(cffont->topdict, "XUID");

  /*
   * Force existence of Encoding.
   */
  if (!cff_dict_known(cffont->topdict, "Encoding"))
    cff_dict_add(cffont->topdict, "Encoding", 1);
  topdict->offset[1] = cff_dict_pack(cffont->topdict,
				     (card8 *) work_buffer,
				     WORK_BUFFER_SIZE) + 1;
  private_size = 0;
  if (cffont->private[0]) {
    cff_dict_remove(cffont->private[0], "Subrs"); /* no Subrs */
    private_size = cff_dict_pack(cffont->private[0],
				 (card8 *) work_buffer, WORK_BUFFER_SIZE);
  }

  /*
   * Estimate total size of fontfile.
   */
  stream_data_len = 4; /* header size */

  stream_data_len += cff_set_name(cffont, fullname);
  RELEASE(fullname);

  stream_data_len += cff_index_size(topdict);
  stream_data_len += cff_index_size(cffont->string);
  stream_data_len += cff_index_size(cffont->gsubr);

  /* We are using format 1 for Encoding and format 0 for charset.
   * TODO: Should implement cff_xxx_size().
   */
  stream_data_len += 2 + (encoding->num_entries)*2 + 1 + (encoding->num_supps)*3;
  stream_data_len += 1 + (charset->num_entries)*2;
  stream_data_len += charstring_len;
  stream_data_len += private_size;

  /*
   * Now we create FontFile data.
   */
  stream_data_ptr = NEW(stream_data_len, card8);

  /*
   * Data Layout order as described in CFF spec., sec 2 "Data Layout".
   */
  offset = 0;
  /* Header */
  offset += cff_put_header(cffont, stream_data_ptr + offset, stream_data_len - offset);
  /* Name */
  offset += cff_pack_index(cffont->name, stream_data_ptr + offset, stream_data_len - offset);
  /* Top DICT */
  topdict_offset = offset;
  offset += cff_index_size(topdict);
  /* Strings */
  offset += cff_pack_index(cffont->string,
			   stream_data_ptr + offset, stream_data_len - offset);
  /* Global Subrs */
  offset += cff_pack_index(cffont->gsubr,
			   stream_data_ptr + offset, stream_data_len - offset);
  /* Encoding */
  cff_dict_set(cffont->topdict, "Encoding", 0, offset);
  offset += cff_pack_encoding(cffont,
			      stream_data_ptr + offset, stream_data_len - offset);
  /* charset */
  cff_dict_set(cffont->topdict, "charset", 0, offset);
  offset += cff_pack_charsets(cffont,
			      stream_data_ptr + offset, stream_data_len - offset);
  /* CharStrings */
  cff_dict_set(cffont->topdict, "CharStrings", 0, offset);
  offset += cff_pack_index(charstrings,
			   stream_data_ptr + offset, charstring_len);
  cff_release_index(charstrings);
  /* Private */
  cff_dict_set(cffont->topdict, "Private", 1, offset);
  if (cffont->private[0] && private_size > 0)
    private_size = cff_dict_pack(cffont->private[0],
				 stream_data_ptr + offset, private_size);
  cff_dict_set(cffont->topdict, "Private", 0, private_size);
  offset += private_size;

  /* Finally Top DICT */
  topdict->data = NEW(topdict->offset[1] - 1, card8);
  cff_dict_pack (cffont->topdict, topdict->data, topdict->offset[1] - 1);
  cff_pack_index(topdict,
		 stream_data_ptr + topdict_offset, cff_index_size(topdict));
  cff_release_index(topdict);

  /* Copyright and Trademark Notice ommited. */

  /* Handle Widths in fontdict. */
  add_SimpleMetrics(font, cffont, widths, num_glyphs);

  /* Close font */
  cff_close (cffont);
  sfnt_close(sfont);

  if (fp)
    DPXFCLOSE(fp);

  if (verbose > 1) {
    MESG("[%u/%u glyphs][%ld bytes]", num_glyphs, cs_count, offset);
  }

  /*
   * CharSet
   */
  pdf_add_dict(descriptor,
               pdf_new_name("CharSet"),
               pdf_new_string(pdf_stream_dataptr(pdfcharset),
                              pdf_stream_length(pdfcharset)));
  pdf_release_obj(pdfcharset);
  /*
   * Write PDF FontFile data.
   */
  fontfile    = pdf_new_stream(STREAM_COMPRESS);
  stream_dict = pdf_stream_dict(fontfile);
  pdf_add_dict(descriptor,
	       pdf_new_name("FontFile3"), pdf_ref_obj (fontfile));
  pdf_add_dict(stream_dict,
	       pdf_new_name("Subtype"),   pdf_new_name("Type1C"));
  pdf_add_stream (fontfile, (void *) stream_data_ptr, offset);
  pdf_release_obj(fontfile);

  RELEASE(stream_data_ptr);

  return 0;
}
