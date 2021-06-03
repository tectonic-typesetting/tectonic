/*

    This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2018 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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

#ifndef _PDFENCRYPT_H_
#define _PDFENCRYPT_H_

#include "tectonic_bridge_core.h"

#include <stddef.h>

#include "dpx-pdfdoc.h"
#include "dpx-pdfobj.h"

#define MAX_PWD_LEN 127

int pdf_init_encryption(struct pdf_enc_setting, const unsigned char *trailer_id);
void pdf_enc_set_label (unsigned label);
void pdf_enc_set_generation (unsigned generation);
void pdf_encrypt_data (const unsigned char *plain, size_t plain_len,
                              unsigned char **cipher, size_t *cipher_len);
pdf_obj *pdf_encrypt_obj (void);

#endif /* _PDFENCRYPT_H_ */
