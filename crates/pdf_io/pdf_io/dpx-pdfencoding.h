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

#ifndef _PDFENCODING_H_
#define _PDFENCODING_H_

#include "tectonic_bridge_core.h"

#include "dpx-pdfobj.h"

void      pdf_init_encodings          (void);
void      pdf_close_encodings         (void);

/* Creates Encoding resource and ToUnicode CMap
 * for all non-predefined encodings.
 */
void      pdf_encoding_complete       (void);

/* enc_name here is .enc file name or the name of predefined
 * encodings.
 */
int       pdf_encoding_findresource   (const char *enc_name);

/* Returns the Encoding resource object.
 */
pdf_obj  *pdf_get_encoding_obj        (int enc_id);

int       pdf_encoding_is_predefined  (int enc_id);
void      pdf_encoding_used_by_type3  (int enc_id);

/* WARNING:
 * Pointer(s) may change after another encoding is loaded.
 */
char     *pdf_encoding_get_name       (int enc_id);
char    **pdf_encoding_get_encoding   (int enc_id);

/*
 * pdf_create_ToUnicode_CMap() returns stream object but not
 * reference. This need to be renamed to other name like
 * pdf_create_ToUnicode_stream().
 */
pdf_obj  *pdf_create_ToUnicode_CMap   (const char *enc_name,
                                              char **enc_vec,
                                              const char *is_used);

/* pdf_encoding_copy_usedchars adds the given vector of used characters
 * to the corresponding vector of the encoding.
 */
void      pdf_encoding_add_usedchars (int encoding_id,
                                              const char *is_used);

pdf_obj * pdf_encoding_get_tounicode  (int encoding_id);

/* Just load CMap identified with 'ident'. (parsed)
 * PDF stream object (not reference) returned.
 */
pdf_obj  *pdf_load_ToUnicode_stream   (const char *ident);

#endif /* _PDFENCODINGS_H_ */
