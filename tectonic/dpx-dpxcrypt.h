/* This is DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2003-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _DPXCRYPT_H_
#define _DPXCRYPT_H_

#include "tectonic_bridge_core.h"

#include <stdio.h>
#include <stdint.h>

/* libgcrypt md5 */
typedef struct {
  uint32_t      A, B, C, D; /* chaining variables */
  size_t        nblocks;
  unsigned char buf[64];
  int count;
} MD5_CONTEXT;

void MD5_init (MD5_CONTEXT *ctx);
void MD5_write(MD5_CONTEXT *ctx, const unsigned char *inbuf, unsigned int inlen);
void MD5_final(unsigned char *outbuf, MD5_CONTEXT *ctx);

typedef struct {
  uint32_t      h0,h1,h2,h3,h4,h5,h6,h7;
  size_t        nblocks;
  unsigned char buf[64];
  int           count;
} SHA256_CONTEXT;

typedef struct
{
  uint64_t h0, h1, h2, h3, h4, h5, h6, h7;
} SHA512_STATE;

typedef struct
{
  SHA512_STATE  state;
  size_t        nblocks;
  unsigned char buf[128];
  int           count;
} SHA512_CONTEXT;

void SHA256_init (SHA256_CONTEXT *ctx);
void SHA256_write(SHA256_CONTEXT *ctx,
                  const unsigned char *inbuf, unsigned int inlen);
void SHA256_final(unsigned char *outbuf, SHA256_CONTEXT *ctx);

void SHA384_init (SHA512_CONTEXT *ctx);
#define SHA384_write(c,b,l) SHA512_write((c),(b),(l))
#define SHA384_final(b,c)   SHA512_final((b),(c))
void SHA512_init (SHA512_CONTEXT *ctx);
void SHA512_write(SHA512_CONTEXT *ctx,
                  const unsigned char *inbuf, unsigned int inlen);
void SHA512_final(unsigned char *outbuf, SHA512_CONTEXT *ctx);

/* libgcrypt arcfour */
typedef struct {
  int idx_i, idx_j;
  unsigned char sbox[256];
} ARC4_CONTEXT;

#define AES_BLOCKSIZE 16

void ARC4 (ARC4_CONTEXT *ctx, unsigned int len, const unsigned char *inbuf, unsigned char *outbuf);
void ARC4_set_key (ARC4_CONTEXT *ctx, unsigned int keylen, const unsigned char *key);

void AES_ecb_encrypt (const unsigned char *key,    size_t  key_len,
                      const unsigned char *plain,  size_t  plain_len,
                      unsigned char      **cipher, size_t *cipher_len);

void AES_cbc_encrypt_tectonic (const unsigned char *key,    size_t  key_len,
                               const unsigned char *iv,     int     padding,
                               const unsigned char *plain,  size_t  plain_len,
                               unsigned char      **cipher, size_t *cipher_len);

#endif /* _DPXCRYPT_H_ */
