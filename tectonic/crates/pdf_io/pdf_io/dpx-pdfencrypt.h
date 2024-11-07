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

typedef struct pdf_sec pdf_sec;
extern pdf_sec *pdf_enc_init (const unsigned char *id,
                              int keybits, uint32_t permission,
                              const char *opasswd, const char *upasswd,
                              int use_aes, int encrypt_metadata);
extern void     pdf_enc_close (pdf_sec **p_sec);

extern void     pdf_enc_set_label      (pdf_sec *p_sec, unsigned label);
extern void     pdf_enc_set_generation (pdf_sec *p_sec, unsigned generation);
extern void     pdf_encrypt_data (pdf_sec *p_sec, const unsigned char *plain, size_t plain_len,
                                  unsigned char **cipher, size_t *cipher_len);
extern pdf_obj *pdf_enc_get_encrypt_dict (pdf_sec *p_sec);
extern pdf_obj *pdf_enc_get_extension_dict (pdf_sec *p_sec);

#endif /* _PDFENCRYPT_H_ */
