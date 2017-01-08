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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#ifdef HAVE_SYS_TYPES_H
# include <sys/types.h>
#endif
#include <string.h>
#include <stdlib.h>

#include "mem.h"
#include "error.h"
#include "numbers.h"
#include "dpxcrypt.h"

static void _gcry_burn_stack (int bytes)
{
  char buf[64];
    
  memset(buf, 0, sizeof buf);
  bytes -= sizeof buf;
  if (bytes > 0) _gcry_burn_stack(bytes);
}


/* Rotate a 32 bit integer by n bytes */
#define rol(x,n) ( ((x) << (n)) | ((x) >> (32-(n))) )
#define ror(x,n) ( ((x) >> ((n)&(32-1))) | ((x) << ((32-(n))&(32-1))) )

/*
 * The following codes for MD5 Message-Digest Algorithm were modified
 * by Jin-Hwan Cho on August 5, 2003 based on libgrypt-1.1.42.
 *
 * Copyright (C) 1995,1996,1998,1999,2001,2002,2003 Free Software Foundation, Inc.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA
 *
 * According to the definition of MD5 in RFC 1321 from April 1992.
 * NOTE: This is *not* the same file as the one from glibc.
 * Written by Ulrich Drepper <drepper@gnu.ai.mit.edu>, 1995. 
 * heavily modified for GnuPG by Werner Koch <wk@gnupg.org> 
 */

void MD5_init (MD5_CONTEXT *ctx)
{
  ctx->A = 0x67452301U;
  ctx->B = 0xefcdab89U;
  ctx->C = 0x98badcfeU;
  ctx->D = 0x10325476U;

  ctx->nblocks = 0;
  ctx->count = 0;
}

/* These are the four functions used in the four steps of the MD5 algorithm
 * and defined in the RFC 1321. The first function is a little bit optimized
 * (as found in Colin Plumbs public domain implementation). */
/* #define FF(b, c, d) ((b & c) | (~b & d)) */
#define FF(b, c, d) (d ^ (b & (c ^ d)))
#define FG(b, c, d) FF(d, b, c)
#define FH(b, c, d) (b ^ c ^ d)
#define FI(b, c, d) (c ^ (b | ~d))

/* transform n*64 bytes */
static void transform (MD5_CONTEXT *ctx, const unsigned char *data)
{
  uint32_t correct_words[16];
  uint32_t A = ctx->A;
  uint32_t B = ctx->B;
  uint32_t C = ctx->C;
  uint32_t D = ctx->D;
  uint32_t *cwp = correct_words;

#ifdef WORDS_BIGENDIAN
  { int i; const unsigned char *p1; unsigned char *p2;
    for (i = 0, p1 = data, p2 = (unsigned char *)correct_words; i < 16; i++, p2 += 4 ) {
      p2[3] = *p1++; p2[2] = *p1++; p2[1] = *p1++; p2[0] = *p1++;
    }
  }
#else
  memcpy(correct_words, data, sizeof(uint32_t) * 16);
#endif

#define OP(a, b, c, d, s, T) \
  do { a += FF(b, c, d) + (*cwp++) + T; a = rol(a, s); a += b; } while (0)

  /* Before we start, one word about the strange constants.
   * They are defined in RFC 1321 as
   *
   *   T[i] = (int) (4294967296.0 * fabs (sin (i))), i=1..64
   */

  /* Round 1. */
  OP(A, B, C, D,  7, 0xd76aa478);
  OP(D, A, B, C, 12, 0xe8c7b756);
  OP(C, D, A, B, 17, 0x242070db);
  OP(B, C, D, A, 22, 0xc1bdceee);
  OP(A, B, C, D,  7, 0xf57c0faf);
  OP(D, A, B, C, 12, 0x4787c62a);
  OP(C, D, A, B, 17, 0xa8304613);
  OP(B, C, D, A, 22, 0xfd469501);
  OP(A, B, C, D,  7, 0x698098d8);
  OP(D, A, B, C, 12, 0x8b44f7af);
  OP(C, D, A, B, 17, 0xffff5bb1);
  OP(B, C, D, A, 22, 0x895cd7be);
  OP(A, B, C, D,  7, 0x6b901122);
  OP(D, A, B, C, 12, 0xfd987193);
  OP(C, D, A, B, 17, 0xa679438e);
  OP(B, C, D, A, 22, 0x49b40821);

#undef OP
#define OP(f, a, b, c, d, k, s, T) \
  do { a += f(b, c, d) + correct_words[k] + T; a = rol(a, s); a += b; } while (0)

  /* Round 2. */
  OP(FG, A, B, C, D,  1,  5, 0xf61e2562);
  OP(FG, D, A, B, C,  6,  9, 0xc040b340);
  OP(FG, C, D, A, B, 11, 14, 0x265e5a51);
  OP(FG, B, C, D, A,  0, 20, 0xe9b6c7aa);
  OP(FG, A, B, C, D,  5,  5, 0xd62f105d);
  OP(FG, D, A, B, C, 10,  9, 0x02441453);
  OP(FG, C, D, A, B, 15, 14, 0xd8a1e681);
  OP(FG, B, C, D, A,  4, 20, 0xe7d3fbc8);
  OP(FG, A, B, C, D,  9,  5, 0x21e1cde6);
  OP(FG, D, A, B, C, 14,  9, 0xc33707d6);
  OP(FG, C, D, A, B,  3, 14, 0xf4d50d87);
  OP(FG, B, C, D, A,  8, 20, 0x455a14ed);
  OP(FG, A, B, C, D, 13,  5, 0xa9e3e905);
  OP(FG, D, A, B, C,  2,  9, 0xfcefa3f8);
  OP(FG, C, D, A, B,  7, 14, 0x676f02d9);
  OP(FG, B, C, D, A, 12, 20, 0x8d2a4c8a);

  /* Round 3. */
  OP(FH, A, B, C, D,  5,  4, 0xfffa3942);
  OP(FH, D, A, B, C,  8, 11, 0x8771f681);
  OP(FH, C, D, A, B, 11, 16, 0x6d9d6122);
  OP(FH, B, C, D, A, 14, 23, 0xfde5380c);
  OP(FH, A, B, C, D,  1,  4, 0xa4beea44);
  OP(FH, D, A, B, C,  4, 11, 0x4bdecfa9);
  OP(FH, C, D, A, B,  7, 16, 0xf6bb4b60);
  OP(FH, B, C, D, A, 10, 23, 0xbebfbc70);
  OP(FH, A, B, C, D, 13,  4, 0x289b7ec6);
  OP(FH, D, A, B, C,  0, 11, 0xeaa127fa);
  OP(FH, C, D, A, B,  3, 16, 0xd4ef3085);
  OP(FH, B, C, D, A,  6, 23, 0x04881d05);
  OP(FH, A, B, C, D,  9,  4, 0xd9d4d039);
  OP(FH, D, A, B, C, 12, 11, 0xe6db99e5);
  OP(FH, C, D, A, B, 15, 16, 0x1fa27cf8);
  OP(FH, B, C, D, A,  2, 23, 0xc4ac5665);

  /* Round 4.  */
  OP(FI, A, B, C, D,  0,  6, 0xf4292244);
  OP(FI, D, A, B, C,  7, 10, 0x432aff97);
  OP(FI, C, D, A, B, 14, 15, 0xab9423a7);
  OP(FI, B, C, D, A,  5, 21, 0xfc93a039);
  OP(FI, A, B, C, D, 12,  6, 0x655b59c3);
  OP(FI, D, A, B, C,  3, 10, 0x8f0ccc92);
  OP(FI, C, D, A, B, 10, 15, 0xffeff47d);
  OP(FI, B, C, D, A,  1, 21, 0x85845dd1);
  OP(FI, A, B, C, D,  8,  6, 0x6fa87e4f);
  OP(FI, D, A, B, C, 15, 10, 0xfe2ce6e0);
  OP(FI, C, D, A, B,  6, 15, 0xa3014314);
  OP(FI, B, C, D, A, 13, 21, 0x4e0811a1);
  OP(FI, A, B, C, D,  4,  6, 0xf7537e82);
  OP(FI, D, A, B, C, 11, 10, 0xbd3af235);
  OP(FI, C, D, A, B,  2, 15, 0x2ad7d2bb);
  OP(FI, B, C, D, A,  9, 21, 0xeb86d391);

  /* Put checksum in context given as argument. */
  ctx->A += A;
  ctx->B += B;
  ctx->C += C;
  ctx->D += D;
}

/* The routine updates the message-digest context to
 * account for the presence of each of the characters inBuf[0..inLen-1]
 * in the message whose digest is being computed. */
void MD5_write (MD5_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen)
{
  if (hd->count == 64) { /* flush the buffer */
    transform(hd, hd->buf);
    _gcry_burn_stack(80+6*sizeof(void*));
    hd->count = 0;
    hd->nblocks++;
  }
  if (!inbuf) return;
  if (hd->count) {
    for (; inlen && hd->count < 64; inlen--)
      hd->buf[hd->count++] = *inbuf++;
    MD5_write(hd, NULL, 0);
    if (!inlen) return;
  }
  _gcry_burn_stack(80+6*sizeof(void*));

  while (inlen >= 64) {
    transform(hd, inbuf);
    hd->count = 0;
    hd->nblocks++;
    inlen -= 64;
    inbuf += 64;
  }
  for (; inlen && hd->count < 64; inlen--)
    hd->buf[hd->count++] = *inbuf++;
}

/* The routine final terminates the message-digest computation and
 * ends with the desired message digest in mdContext->digest[0...15].
 * The handle is prepared for a new MD5 cycle.
 * Returns 16 bytes representing the digest. */

void MD5_final (unsigned char *outbuf, MD5_CONTEXT *hd)
{
  uint32_t t, msb, lsb;
  unsigned char *p;

  MD5_write(hd, NULL, 0); /* flush */

  t = hd->nblocks;
  /* multiply by 64 to make a byte count */
  lsb = t << 6;
  msb = t >> 26;
  /* add the count */
  t = lsb;
  if ((lsb += hd->count) < t) msb++;
  /* multiply by 8 to make a bit count */
  t = lsb;
  lsb <<= 3;
  msb <<= 3;
  msb |= t >> 29;

  if (hd->count < 56) { /* enough room */
    hd->buf[hd->count++] = 0x80; /* pad */
    while (hd->count < 56) hd->buf[hd->count++] = 0; /* pad */
  } else { /* need one extra block */
    hd->buf[hd->count++] = 0x80; /* pad character */
    while (hd->count < 64) hd->buf[hd->count++] = 0;
    MD5_write(hd, NULL, 0); /* flush */
    memset(hd->buf, 0, 56); /* fill next block with zeroes */
  }
  /* append the 64 bit count */
  hd->buf[56] = lsb & 0xff;
  hd->buf[57] = (lsb >> 8) & 0xff;
  hd->buf[58] = (lsb >> 16) & 0xff;
  hd->buf[59] = (lsb >> 24) & 0xff;
  hd->buf[60] = msb & 0xff;
  hd->buf[61] = (msb >> 8) & 0xff;
  hd->buf[62] = (msb >> 16) & 0xff;
  hd->buf[63] = (msb >> 24) & 0xff;
  transform(hd, hd->buf);
  _gcry_burn_stack(80+6*sizeof(void*));

  p = outbuf; /* p = hd->buf; */
#ifdef WORDS_BIGENDIAN
#define X(a) do { *p++ = hd->a; *p++ = hd->a >> 8; \
	          *p++ = hd->a >> 16; *p++ = hd->a >> 24; } while (0)
#else /* little endian */
#define X(a) do { *(uint32_t *)p = (*hd).a ; p += sizeof(uint32_t); } while (0)
#endif
  X(A);
  X(B);
  X(C);
  X(D);
#undef X
}

/*
 * The following codes for the SHA256 hash function are taken from
 * libgrypt-1.6.3. (slightly modified)
 *
 * sha256.c - SHA256 hash function
 * Copyright (C) 2003, 2006, 2008, 2009 Free Software Foundation, Inc.
 *
 * This file is part of Libgcrypt.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, see <http://www.gnu.org/licenses/>.
 */


static uint32_t
_gcry_bswap32(uint32_t x)
{
  return ((rol(x, 8) & 0x00ff00ffL) | (ror(x, 8) & 0xff00ff00L));
}

static uint64_t
_gcry_bswap64(uint64_t x)
{
  return ((uint64_t)_gcry_bswap32(x) << 32) | (_gcry_bswap32(x >> 32));
}

/* Endian dependent byte swap operations.  */
#ifdef WORDS_BIGENDIAN
# define le_bswap32(x) _gcry_bswap32(x)
# define be_bswap32(x) ((uint32_t)(x))
# define le_bswap64(x) _gcry_bswap64(x)
# define be_bswap64(x) ((uint64_t)(x))
#else
# define le_bswap32(x) ((uint32_t)(x))
# define be_bswap32(x) _gcry_bswap32(x)
# define le_bswap64(x) ((uint64_t)(x))
# define be_bswap64(x) _gcry_bswap64(x)
#endif

static uint32_t buf_get_be32(const void *_buf)
{
  const uint8_t *in = _buf;
  return ((uint32_t)in[0] << 24) | ((uint32_t)in[1] << 16) | \
         ((uint32_t)in[2] << 8) | (uint32_t)in[3];
}

static void buf_put_be32(void *_buf, uint32_t val)
{
  uint8_t *out = _buf;
  out[0] = val >> 24;
  out[1] = val >> 16;
  out[2] = val >> 8;
  out[3] = val;
}

static uint64_t buf_get_be64(const void *_buf)
{
  const uint8_t *in = _buf;
  return ((uint64_t)in[0] << 56) | ((uint64_t)in[1] << 48) | \
         ((uint64_t)in[2] << 40) | ((uint64_t)in[3] << 32) | \
         ((uint64_t)in[4] << 24) | ((uint64_t)in[5] << 16) | \
         ((uint64_t)in[6] << 8) | (uint64_t)in[7];
}

static void buf_put_be64(void *_buf, uint64_t val)
{
  uint8_t *out = _buf;
  out[0] = val >> 56;
  out[1] = val >> 48;
  out[2] = val >> 40;
  out[3] = val >> 32;
  out[4] = val >> 24;
  out[5] = val >> 16;
  out[6] = val >> 8;
  out[7] = val;
}

void
SHA256_init (SHA256_CONTEXT *hd)
{
  hd->h0 = 0x6a09e667;
  hd->h1 = 0xbb67ae85;
  hd->h2 = 0x3c6ef372;
  hd->h3 = 0xa54ff53a;
  hd->h4 = 0x510e527f;
  hd->h5 = 0x9b05688c;
  hd->h6 = 0x1f83d9ab;
  hd->h7 = 0x5be0cd19;

  hd->nblocks = 0;
  hd->count   = 0;
}

/*
  Transform the message X which consists of 16 32-bit-words. See FIPS
  180-2 for details.  */
#define S0(x) (ror ((x), 7) ^ ror ((x), 18) ^ ((x) >> 3))       /* (4.6) */
#define S1(x) (ror ((x), 17) ^ ror ((x), 19) ^ ((x) >> 10))     /* (4.7) */
#define R(a,b,c,d,e,f,g,h,k,w) do                                 \
          {                                                       \
            t1 = (h) + Sum1((e)) + Cho((e),(f),(g)) + (k) + (w);  \
            t2 = Sum0((a)) + Maj((a),(b),(c));                    \
            h = g;                                                \
            g = f;                                                \
            f = e;                                                \
            e = d + t1;                                           \
            d = c;                                                \
            c = b;                                                \
            b = a;                                                \
            a = t1 + t2;                                          \
          } while (0)


#define Cho(x,y,z) ((z) ^ ((x) & ((y) ^ (z))))
#define Maj(x,y,z) (((x) & (y)) | ((z) & ((x)|(y))))
#define Sum0(x) (ror((x), 2) ^ ror((x), 13) ^ ror((x), 22))
#define Sum1(x) (ror((x), 6) ^ ror((x), 11) ^ ror((x), 25))

static unsigned int
_SHA256_transform (SHA256_CONTEXT *hd, const unsigned char *data)
{
  static const uint32_t K[64] = {
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
  };

  uint32_t a,b,c,d,e,f,g,h,t1,t2;
  uint32_t w[64];
  int i;

  a = hd->h0;
  b = hd->h1;
  c = hd->h2;
  d = hd->h3;
  e = hd->h4;
  f = hd->h5;
  g = hd->h6;
  h = hd->h7;

  for (i=0; i < 16; i++)
    w[i] = buf_get_be32(data + i * 4);
  for (; i < 64; i++)
    w[i] = S1(w[i-2]) + w[i-7] + S0(w[i-15]) + w[i-16];

  for (i=0; i < 64;)
    {
#if 0
      R(a,b,c,d,e,f,g,h,K[i],w[i]);
      i++;
#else
      t1 = h + Sum1 (e) + Cho (e, f, g) + K[i] + w[i];
      t2 = Sum0 (a) + Maj (a, b, c);
      d += t1;
      h  = t1 + t2;

      t1 = g + Sum1 (d) + Cho (d, e, f) + K[i+1] + w[i+1];
      t2 = Sum0 (h) + Maj (h, a, b);
      c += t1;
      g  = t1 + t2;

      t1 = f + Sum1 (c) + Cho (c, d, e) + K[i+2] + w[i+2];
      t2 = Sum0 (g) + Maj (g, h, a);
      b += t1;
      f  = t1 + t2;

      t1 = e + Sum1 (b) + Cho (b, c, d) + K[i+3] + w[i+3];
      t2 = Sum0 (f) + Maj (f, g, h);
      a += t1;
      e  = t1 + t2;

      t1 = d + Sum1 (a) + Cho (a, b, c) + K[i+4] + w[i+4];
      t2 = Sum0 (e) + Maj (e, f, g);
      h += t1;
      d  = t1 + t2;

      t1 = c + Sum1 (h) + Cho (h, a, b) + K[i+5] + w[i+5];
      t2 = Sum0 (d) + Maj (d, e, f);
      g += t1;
      c  = t1 + t2;

      t1 = b + Sum1 (g) + Cho (g, h, a) + K[i+6] + w[i+6];
      t2 = Sum0 (c) + Maj (c, d, e);
      f += t1;
      b  = t1 + t2;

      t1 = a + Sum1 (f) + Cho (f, g, h) + K[i+7] + w[i+7];
      t2 = Sum0 (b) + Maj (b, c, d);
      e += t1;
      a  = t1 + t2;

      i += 8;
#endif
    }

  hd->h0 += a;
  hd->h1 += b;
  hd->h2 += c;
  hd->h3 += d;
  hd->h4 += e;
  hd->h5 += f;
  hd->h6 += g;
  hd->h7 += h;

  return /*burn_stack*/ 74*4+32;
}
#undef S0
#undef S1
#undef R

void SHA256_write (SHA256_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen)
{
  unsigned int stack_burn = 0;

  if (hd->count == 64) { /* flush the buffer */
    stack_burn = _SHA256_transform(hd, hd->buf);
    _gcry_burn_stack(stack_burn);
    hd->count = 0;
    hd->nblocks++;
  }
  if (!inbuf) return;
  if (hd->count) {
    for (; inlen && hd->count < 64; inlen--)
      hd->buf[hd->count++] = *inbuf++;
    SHA256_write(hd, NULL, 0);
    if (!inlen) return;
  }
  _gcry_burn_stack(stack_burn);

  while (inlen >= 64) {
    stack_burn = _SHA256_transform(hd, inbuf);
    hd->count = 0;
    hd->nblocks++;
    inlen -= 64;
    inbuf += 64;
  }
  for (; inlen && hd->count < 64; inlen--)
    hd->buf[hd->count++] = *inbuf++;
}

/*
   The routine finally terminates the computation and returns the
   digest.  The handle is prepared for a new cycle, but adding bytes
   to the handle will the destroy the returned buffer.  Returns: 32
   bytes with the message the digest.  */
void
SHA256_final(unsigned char *outbuf, SHA256_CONTEXT *hd)
{
  uint32_t       t, msb, lsb;
  unsigned char *p;
  unsigned int   burn;

  SHA256_write(hd, NULL, 0); /* flush */;

  t   = hd->nblocks;

  /* multiply by 64 to make a byte count */
  lsb = t << 6;
  msb = t >> 26;
  /* add the count */
  t = lsb;
  if ((lsb += hd->count) < t)
    msb++;
  /* multiply by 8 to make a bit count */
  t     = lsb;
  lsb <<= 3;
  msb <<= 3;
  msb |= t >> 29;

  if (hd->count < 56)
    { /* enough room */
      hd->buf[hd->count++] = 0x80; /* pad */
      while (hd->count < 56)
        hd->buf[hd->count++] = 0;  /* pad */
    }
  else
    { /* need one extra block */
      hd->buf[hd->count++] = 0x80; /* pad character */
      while (hd->count < 64)
        hd->buf[hd->count++] = 0;
      SHA256_write(hd, NULL, 0);  /* flush */;
      memset (hd->buf, 0, 56 ); /* fill next block with zeroes */
    }
  /* append the 64 bit count */
  buf_put_be32(hd->buf + 56, msb);
  buf_put_be32(hd->buf + 60, lsb);
  burn = _SHA256_transform(hd, hd->buf);
  _gcry_burn_stack(burn);

  p = outbuf;
#define X(a) do { *(uint32_t*)p = be_bswap32(hd->h##a); p += 4; } while(0)
  X(0);
  X(1);
  X(2);
  X(3);
  X(4);
  X(5);
  X(6);
  X(7);
#undef X
}

#undef Cho
#undef Maj
#undef Sum0
#undef Sum1

/* The following code are taken from libgcrypt-1.6.3. (slightly modified):
 *
 * sha512.c - SHA384 and SHA512 hash functions
 * Copyright (C) 2003, 2008, 2009 Free Software Foundation, Inc.
 *
 * This file is part of Libgcrypt.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser general Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, see <http://www.gnu.org/licenses/>.
 */

#define ROTR(x,n) (((x) >> (n)) | ((x) << (64 - (n))))
#define Ch(x,y,z) (((x) & (y)) ^ ( ~(x) & (z)))
#define Maj(x,y,z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))
#define Sum0(x) (ROTR((x), 28) ^ ROTR((x), 34) ^ ROTR((x), 39))
#define Sum1(x) (ROTR((x), 14) ^ ROTR((x), 18) ^ ROTR((x), 41))
#define U64_C(c) (c ## UL)

void
SHA512_init (SHA512_CONTEXT *ctx)
{
  SHA512_STATE *hd = &ctx->state;

  hd->h0 = U64_C(0x6a09e667f3bcc908);
  hd->h1 = U64_C(0xbb67ae8584caa73b);
  hd->h2 = U64_C(0x3c6ef372fe94f82b);
  hd->h3 = U64_C(0xa54ff53a5f1d36f1);
  hd->h4 = U64_C(0x510e527fade682d1);
  hd->h5 = U64_C(0x9b05688c2b3e6c1f);
  hd->h6 = U64_C(0x1f83d9abfb41bd6b);
  hd->h7 = U64_C(0x5be0cd19137e2179);

  ctx->nblocks = 0;
  ctx->count   = 0;
}

void
SHA384_init (SHA512_CONTEXT *ctx)
{
  SHA512_STATE *hd = &ctx->state;

  hd->h0 = U64_C(0xcbbb9d5dc1059ed8);
  hd->h1 = U64_C(0x629a292a367cd507);
  hd->h2 = U64_C(0x9159015a3070dd17);
  hd->h3 = U64_C(0x152fecd8f70e5939);
  hd->h4 = U64_C(0x67332667ffc00b31);
  hd->h5 = U64_C(0x8eb44a8768581511);
  hd->h6 = U64_C(0xdb0c2e0d64f98fa7);
  hd->h7 = U64_C(0x47b5481dbefa4fa4);

  ctx->nblocks = 0;
  ctx->count   = 0;
}


static const uint64_t k[] =
  {
    U64_C(0x428a2f98d728ae22), U64_C(0x7137449123ef65cd),
    U64_C(0xb5c0fbcfec4d3b2f), U64_C(0xe9b5dba58189dbbc),
    U64_C(0x3956c25bf348b538), U64_C(0x59f111f1b605d019),
    U64_C(0x923f82a4af194f9b), U64_C(0xab1c5ed5da6d8118),
    U64_C(0xd807aa98a3030242), U64_C(0x12835b0145706fbe),
    U64_C(0x243185be4ee4b28c), U64_C(0x550c7dc3d5ffb4e2),
    U64_C(0x72be5d74f27b896f), U64_C(0x80deb1fe3b1696b1),
    U64_C(0x9bdc06a725c71235), U64_C(0xc19bf174cf692694),
    U64_C(0xe49b69c19ef14ad2), U64_C(0xefbe4786384f25e3),
    U64_C(0x0fc19dc68b8cd5b5), U64_C(0x240ca1cc77ac9c65),
    U64_C(0x2de92c6f592b0275), U64_C(0x4a7484aa6ea6e483),
    U64_C(0x5cb0a9dcbd41fbd4), U64_C(0x76f988da831153b5),
    U64_C(0x983e5152ee66dfab), U64_C(0xa831c66d2db43210),
    U64_C(0xb00327c898fb213f), U64_C(0xbf597fc7beef0ee4),
    U64_C(0xc6e00bf33da88fc2), U64_C(0xd5a79147930aa725),
    U64_C(0x06ca6351e003826f), U64_C(0x142929670a0e6e70),
    U64_C(0x27b70a8546d22ffc), U64_C(0x2e1b21385c26c926),
    U64_C(0x4d2c6dfc5ac42aed), U64_C(0x53380d139d95b3df),
    U64_C(0x650a73548baf63de), U64_C(0x766a0abb3c77b2a8),
    U64_C(0x81c2c92e47edaee6), U64_C(0x92722c851482353b),
    U64_C(0xa2bfe8a14cf10364), U64_C(0xa81a664bbc423001),
    U64_C(0xc24b8b70d0f89791), U64_C(0xc76c51a30654be30),
    U64_C(0xd192e819d6ef5218), U64_C(0xd69906245565a910),
    U64_C(0xf40e35855771202a), U64_C(0x106aa07032bbd1b8),
    U64_C(0x19a4c116b8d2d0c8), U64_C(0x1e376c085141ab53),
    U64_C(0x2748774cdf8eeb99), U64_C(0x34b0bcb5e19b48a8),
    U64_C(0x391c0cb3c5c95a63), U64_C(0x4ed8aa4ae3418acb),
    U64_C(0x5b9cca4f7763e373), U64_C(0x682e6ff3d6b2b8a3),
    U64_C(0x748f82ee5defb2fc), U64_C(0x78a5636f43172f60),
    U64_C(0x84c87814a1f0ab72), U64_C(0x8cc702081a6439ec),
    U64_C(0x90befffa23631e28), U64_C(0xa4506cebde82bde9),
    U64_C(0xbef9a3f7b2c67915), U64_C(0xc67178f2e372532b),
    U64_C(0xca273eceea26619c), U64_C(0xd186b8c721c0c207),
    U64_C(0xeada7dd6cde0eb1e), U64_C(0xf57d4f7fee6ed178),
    U64_C(0x06f067aa72176fba), U64_C(0x0a637dc5a2c898a6),
    U64_C(0x113f9804bef90dae), U64_C(0x1b710b35131c471b),
    U64_C(0x28db77f523047d84), U64_C(0x32caab7b40c72493),
    U64_C(0x3c9ebe0a15c9bebc), U64_C(0x431d67c49c100d4c),
    U64_C(0x4cc5d4becb3e42b6), U64_C(0x597f299cfc657e2a),
    U64_C(0x5fcb6fab3ad6faec), U64_C(0x6c44198c4a475817)
  };

/****************
 * Transform the message W which consists of 16 64-bit-words
 */
static unsigned int
__transform (SHA512_STATE *hd, const unsigned char *data)
{
  uint64_t a, b, c, d, e, f, g, h;
  uint64_t w[16];
  int t;

  /* get values from the chaining vars */
  a = hd->h0;
  b = hd->h1;
  c = hd->h2;
  d = hd->h3;
  e = hd->h4;
  f = hd->h5;
  g = hd->h6;
  h = hd->h7;

  for ( t = 0; t < 16; t++ )
    w[t] = buf_get_be64(data + t * 8);

#define S0(x) (ROTR((x),1) ^ ROTR((x),8) ^ ((x)>>7))
#define S1(x) (ROTR((x),19) ^ ROTR((x),61) ^ ((x)>>6))


  for (t = 0; t < 80 - 16; )
    {
      uint64_t t1, t2;

      /* Performance on a AMD Athlon(tm) Dual Core Processor 4050e
         with gcc 4.3.3 using gcry_md_hash_buffer of each 10000 bytes
         initialized to 0,1,2,3...255,0,... and 1000 iterations:

         Not unrolled with macros:  440ms
         Unrolled with macros:      350ms
         Unrolled with inline:      330ms
      */
#if 0 /* Not unrolled.  */
      t1 = h + Sum1 (e) + Ch (e, f, g) + k[t] + w[t%16];
      w[t%16] += S1 (w[(t - 2)%16]) + w[(t - 7)%16] + S0 (w[(t - 15)%16]);
      t2 = Sum0 (a) + Maj (a, b, c);
      h = g;
      g = f;
      f = e;
      e = d + t1;
      d = c;
      c = b;
      b = a;
      a = t1 + t2;
      t++;
#else /* Unrolled to interweave the chain variables.  */
      t1 = h + Sum1 (e) + Ch (e, f, g) + k[t] + w[0];
      w[0] += S1 (w[14]) + w[9] + S0 (w[1]);
      t2 = Sum0 (a) + Maj (a, b, c);
      d += t1;
      h = t1 + t2;

      t1 = g + Sum1 (d) + Ch (d, e, f) + k[t+1] + w[1];
      w[1] += S1 (w[15]) + w[10] + S0 (w[2]);
      t2 = Sum0 (h) + Maj (h, a, b);
      c += t1;
      g  = t1 + t2;

      t1 = f + Sum1 (c) + Ch (c, d, e) + k[t+2] + w[2];
      w[2] += S1 (w[0]) + w[11] + S0 (w[3]);
      t2 = Sum0 (g) + Maj (g, h, a);
      b += t1;
      f  = t1 + t2;

      t1 = e + Sum1 (b) + Ch (b, c, d) + k[t+3] + w[3];
      w[3] += S1 (w[1]) + w[12] + S0 (w[4]);
      t2 = Sum0 (f) + Maj (f, g, h);
      a += t1;
      e  = t1 + t2;

      t1 = d + Sum1 (a) + Ch (a, b, c) + k[t+4] + w[4];
      w[4] += S1 (w[2]) + w[13] + S0 (w[5]);
      t2 = Sum0 (e) + Maj (e, f, g);
      h += t1;
      d  = t1 + t2;

      t1 = c + Sum1 (h) + Ch (h, a, b) + k[t+5] + w[5];
      w[5] += S1 (w[3]) + w[14] + S0 (w[6]);
      t2 = Sum0 (d) + Maj (d, e, f);
      g += t1;
      c  = t1 + t2;

      t1 = b + Sum1 (g) + Ch (g, h, a) + k[t+6] + w[6];
      w[6] += S1 (w[4]) + w[15] + S0 (w[7]);
      t2 = Sum0 (c) + Maj (c, d, e);
      f += t1;
      b  = t1 + t2;

      t1 = a + Sum1 (f) + Ch (f, g, h) + k[t+7] + w[7];
      w[7] += S1 (w[5]) + w[0] + S0 (w[8]);
      t2 = Sum0 (b) + Maj (b, c, d);
      e += t1;
      a  = t1 + t2;

      t1 = h + Sum1 (e) + Ch (e, f, g) + k[t+8] + w[8];
      w[8] += S1 (w[6]) + w[1] + S0 (w[9]);
      t2 = Sum0 (a) + Maj (a, b, c);
      d += t1;
      h  = t1 + t2;

      t1 = g + Sum1 (d) + Ch (d, e, f) + k[t+9] + w[9];
      w[9] += S1 (w[7]) + w[2] + S0 (w[10]);
      t2 = Sum0 (h) + Maj (h, a, b);
      c += t1;
      g  = t1 + t2;

      t1 = f + Sum1 (c) + Ch (c, d, e) + k[t+10] + w[10];
      w[10] += S1 (w[8]) + w[3] + S0 (w[11]);
      t2 = Sum0 (g) + Maj (g, h, a);
      b += t1;
      f  = t1 + t2;

      t1 = e + Sum1 (b) + Ch (b, c, d) + k[t+11] + w[11];
      w[11] += S1 (w[9]) + w[4] + S0 (w[12]);
      t2 = Sum0 (f) + Maj (f, g, h);
      a += t1;
      e  = t1 + t2;

      t1 = d + Sum1 (a) + Ch (a, b, c) + k[t+12] + w[12];
      w[12] += S1 (w[10]) + w[5] + S0 (w[13]);
      t2 = Sum0 (e) + Maj (e, f, g);
      h += t1;
      d  = t1 + t2;

      t1 = c + Sum1 (h) + Ch (h, a, b) + k[t+13] + w[13];
      w[13] += S1 (w[11]) + w[6] + S0 (w[14]);
      t2 = Sum0 (d) + Maj (d, e, f);
      g += t1;
      c  = t1 + t2;

      t1 = b + Sum1 (g) + Ch (g, h, a) + k[t+14] + w[14];
      w[14] += S1 (w[12]) + w[7] + S0 (w[15]);
      t2 = Sum0 (c) + Maj (c, d, e);
      f += t1;
      b  = t1 + t2;

      t1 = a + Sum1 (f) + Ch (f, g, h) + k[t+15] + w[15];
      w[15] += S1 (w[13]) + w[8] + S0 (w[0]);
      t2 = Sum0 (b) + Maj (b, c, d);
      e += t1;
      a  = t1 + t2;

      t += 16;
#endif
    }

  for (; t < 80; )
    {
      uint64_t t1, t2;

#if 0 /* Not unrolled.  */
      t1 = h + Sum1 (e) + Ch (e, f, g) + k[t] + w[t%16];
      t2 = Sum0 (a) + Maj (a, b, c);
      h = g;
      g = f;
      f = e;
      e = d + t1;
      d = c;
      c = b;
      b = a;
      a = t1 + t2;
      t++;
#else /* Unrolled to interweave the chain variables.  */
      t1 = h + Sum1 (e) + Ch (e, f, g) + k[t] + w[0];
      t2 = Sum0 (a) + Maj (a, b, c);
      d += t1;
      h  = t1 + t2;

      t1 = g + Sum1 (d) + Ch (d, e, f) + k[t+1] + w[1];
      t2 = Sum0 (h) + Maj (h, a, b);
      c += t1;
      g  = t1 + t2;

      t1 = f + Sum1 (c) + Ch (c, d, e) + k[t+2] + w[2];
      t2 = Sum0 (g) + Maj (g, h, a);
      b += t1;
      f  = t1 + t2;

      t1 = e + Sum1 (b) + Ch (b, c, d) + k[t+3] + w[3];
      t2 = Sum0 (f) + Maj (f, g, h);
      a += t1;
      e  = t1 + t2;

      t1 = d + Sum1 (a) + Ch (a, b, c) + k[t+4] + w[4];
      t2 = Sum0 (e) + Maj (e, f, g);
      h += t1;
      d  = t1 + t2;

      t1 = c + Sum1 (h) + Ch (h, a, b) + k[t+5] + w[5];
      t2 = Sum0 (d) + Maj (d, e, f);
      g += t1;
      c  = t1 + t2;

      t1 = b + Sum1 (g) + Ch (g, h, a) + k[t+6] + w[6];
      t2 = Sum0 (c) + Maj (c, d, e);
      f += t1;
      b  = t1 + t2;

      t1 = a + Sum1 (f) + Ch (f, g, h) + k[t+7] + w[7];
      t2 = Sum0 (b) + Maj (b, c, d);
      e += t1;
      a  = t1 + t2;

      t1 = h + Sum1 (e) + Ch (e, f, g) + k[t+8] + w[8];
      t2 = Sum0 (a) + Maj (a, b, c);
      d += t1;
      h  = t1 + t2;

      t1 = g + Sum1 (d) + Ch (d, e, f) + k[t+9] + w[9];
      t2 = Sum0 (h) + Maj (h, a, b);
      c += t1;
      g  = t1 + t2;

      t1 = f + Sum1 (c) + Ch (c, d, e) + k[t+10] + w[10];
      t2 = Sum0 (g) + Maj (g, h, a);
      b += t1;
      f  = t1 + t2;

      t1 = e + Sum1 (b) + Ch (b, c, d) + k[t+11] + w[11];
      t2 = Sum0 (f) + Maj (f, g, h);
      a += t1;
      e  = t1 + t2;

      t1 = d + Sum1 (a) + Ch (a, b, c) + k[t+12] + w[12];
      t2 = Sum0 (e) + Maj (e, f, g);
      h += t1;
      d  = t1 + t2;

      t1 = c + Sum1 (h) + Ch (h, a, b) + k[t+13] + w[13];
      t2 = Sum0 (d) + Maj (d, e, f);
      g += t1;
      c  = t1 + t2;

      t1 = b + Sum1 (g) + Ch (g, h, a) + k[t+14] + w[14];
      t2 = Sum0 (c) + Maj (c, d, e);
      f += t1;
      b  = t1 + t2;

      t1 = a + Sum1 (f) + Ch (f, g, h) + k[t+15] + w[15];
      t2 = Sum0 (b) + Maj (b, c, d);
      e += t1;
      a  = t1 + t2;

      t += 16;
#endif
    }

  /* Update chaining vars.  */
  hd->h0 += a;
  hd->h1 += b;
  hd->h2 += c;
  hd->h3 += d;
  hd->h4 += e;
  hd->h5 += f;
  hd->h6 += g;
  hd->h7 += h;

  return /* burn_stack */ (8 + 16) * sizeof(uint64_t) + sizeof(uint32_t) +
                          3 * sizeof(void*);
}


static unsigned int
_SHA512_transform (SHA512_CONTEXT *ctx, const unsigned char *data)
{
  return __transform(&ctx->state, data) + 3 * sizeof(void*);
}

/* The routine final terminates the computation and
 * returns the digest.
 * The handle is prepared for a new cycle, but adding bytes to the
 * handle will the destroy the returned buffer.
 * Returns: 64 bytes representing the digest.  When used for sha384,
 * we take the leftmost 48 of those bytes.
 */

void SHA512_write (SHA512_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen)
{
  unsigned int stack_burn = 0;

  if (hd->count == 128) { /* flush the buffer */
    stack_burn = _SHA512_transform(hd, hd->buf);
    _gcry_burn_stack(stack_burn);
    hd->count = 0;
    hd->nblocks++;
  }
  if (!inbuf) return;
  if (hd->count) {
    for (; inlen && hd->count < 128; inlen--)
      hd->buf[hd->count++] = *inbuf++;
    SHA512_write(hd, NULL, 0);
    if (!inlen) return;
  }
  _gcry_burn_stack(stack_burn);

  while (inlen >= 128) {
    stack_burn = _SHA512_transform(hd, inbuf);
    hd->count = 0;
    hd->nblocks++;
    inlen -= 128;
    inbuf += 128;
  }
  for (; inlen && hd->count < 128; inlen--)
    hd->buf[hd->count++] = *inbuf++;
}

void
SHA512_final (unsigned char *outbuf, SHA512_CONTEXT *hd)
{
  unsigned int   stack_burn_depth;
  uint64_t       t, msb, lsb;
  unsigned char *p;

  SHA512_write(hd, NULL, 0); /* flush */ ;

  t  = hd->nblocks;
  /* multiply by 128 to make a byte count */
  lsb = t << 7;
  msb = t >> 57;
  /* add the count */
  t = lsb;
  if ((lsb += hd->count) < t)
    msb++;
  /* multiply by 8 to make a bit count */
  t = lsb;
  lsb <<= 3;
  msb <<= 3;
  msb |= t >> 61;

  if (hd->count < 112)
    {       /* enough room */
      hd->buf[hd->count++] = 0x80;  /* pad */
      while (hd->count < 112)
        hd->buf[hd->count++] = 0; /* pad */
    }
  else
    {       /* need one extra block */
      hd->buf[hd->count++] = 0x80;  /* pad character */
      while (hd->count < 128)
        hd->buf[hd->count++] = 0;
      SHA512_write(hd, NULL, 0); /* flush */ ;
      memset(hd->buf, 0, 112);  /* fill next block with zeroes */
    }
  /* append the 128 bit count */
  buf_put_be64(hd->buf + 112, msb);
  buf_put_be64(hd->buf + 120, lsb);
  stack_burn_depth = _SHA512_transform(hd, hd->buf);
  _gcry_burn_stack(stack_burn_depth);

  p = outbuf;
#define X(a) do { *(uint64_t*)p = be_bswap64(hd->state.h##a) ; p += 8; } while (0)
  X (0);
  X (1);
  X (2);
  X (3);
  X (4);
  X (5);
  /* Note that these last two chunks are included even for SHA384.
     We just ignore them. */
  X (6);
  X (7);
#undef X
}

#undef ROTR
#undef Ch
#undef Maj
#undef Sum0
#undef Sum1

/*
 * The following codes for the arcfour stream cipher were modified
 * by Jin-Hwan Cho on August 5, 2003 based on libgrypt-1.1.42.
 *
 * Copyright (C) 2000,2001,2002,2003 Free Software Foundation, Inc.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA
 *
 * According to the definition of MD5 in RFC 1321 from April 1992.
 * NOTE: This is *not* the same file as the one from glibc.
 * Written by Ulrich Drepper <drepper@gnu.ai.mit.edu>, 1995. 
 * heavily modified for GnuPG by Werner Koch <wk@gnupg.org> 
 */

static void do_encrypt_stream (ARC4_CONTEXT *ctx, unsigned char *outbuf, const unsigned char *inbuf, unsigned int len)
{
  int i = ctx->idx_i;
  int j = ctx->idx_j;
  unsigned char *sbox = ctx->sbox;
  int t;

  while (len--) {
    i++;
    i = i & 255; /* and seems to be faster than mod */
    j += sbox[i];
    j &= 255;
    t = sbox[i]; sbox[i] = sbox[j]; sbox[j] = t;
    *outbuf++ = *inbuf++ ^ sbox[(sbox[i] + sbox[j]) & 255];
  }
  
  ctx->idx_i = i;
  ctx->idx_j = j;
}

void ARC4 (ARC4_CONTEXT *ctx, unsigned int len, const unsigned char *inbuf, unsigned char *outbuf)
{
  do_encrypt_stream(ctx, outbuf, inbuf, len);
  _gcry_burn_stack(64);
}

static void do_arcfour_setkey (ARC4_CONTEXT *ctx, const unsigned char *key, unsigned int keylen)
{
  int i, j;
  unsigned char karr[256];

  ctx->idx_i = ctx->idx_j = 0;
  for (i = 0; i < 256; i++) ctx->sbox[i] = i;
  for (i = 0; i < 256; i++) karr[i] = key[i%keylen];
  for (i = j = 0; i < 256; i++) {
    int t;
    j = (j + ctx->sbox[i] + karr[i]) % 256;
    t = ctx->sbox[i];
    ctx->sbox[i] = ctx->sbox[j];
    ctx->sbox[j] = t;
  } 
  memset(karr, 0, 256);
}

void ARC4_set_key (ARC4_CONTEXT *ctx, unsigned int keylen, const unsigned char *key)
{
  do_arcfour_setkey(ctx, key, keylen);
  _gcry_burn_stack(300);
}

/* AES Support */

static int  rijndaelSetupEncrypt(uint32_t *rk, const uint8_t *key, int keybits);
static void rijndaelEncrypt(const uint32_t *rk, int nrounds,
                            const uint8_t plaintext[16], uint8_t ciphertext[16]);

#define KEYLENGTH(keybits) ((keybits)/8)
#define RKLENGTH(keybits)  ((keybits)/8+28)
#define NROUNDS(keybits)   ((keybits)/32+6)

#define KEYBITS   256

typedef struct {
  int           nrounds;
  uint32_t      rk[60];
  unsigned char iv[AES_BLOCKSIZE];
} AES_CONTEXT;

void
AES_ecb_encrypt (const unsigned char *key,    size_t  key_len,
                 const unsigned char *plain,  size_t  plain_len,
                 unsigned char      **cipher, size_t *cipher_len)
{
  AES_CONTEXT *ctx, aes;
  const unsigned char *inptr;
  unsigned char *outptr;
  size_t len;

  ctx = &aes;

  *cipher_len = plain_len;
  *cipher     = NEW(*cipher_len, unsigned char);

  ctx->nrounds = rijndaelSetupEncrypt(ctx->rk, key, key_len * 8);

  inptr = plain; outptr = *cipher;
  for (len = plain_len; len >= AES_BLOCKSIZE; len -= AES_BLOCKSIZE) {
    rijndaelEncrypt(ctx->rk, ctx->nrounds, inptr, outptr);
    inptr  += AES_BLOCKSIZE;
    outptr += AES_BLOCKSIZE;
  }
  if (len > 0) {
    unsigned char block[AES_BLOCKSIZE];

    memcpy(block, inptr, len);
    rijndaelEncrypt(ctx->rk, ctx->nrounds, block, outptr);
    inptr  += len;
    outptr += AES_BLOCKSIZE;
  }
}

/* NULL iv means here "use random IV". */
void
AES_cbc_encrypt (const unsigned char *key,    size_t  key_len,
                 const unsigned char *iv,     int     padding,
                 const unsigned char *plain,  size_t  plain_len,
                 unsigned char      **cipher, size_t *cipher_len)
{
  AES_CONTEXT *ctx, aes;
  const unsigned char *inptr;
  unsigned char *outptr, block[AES_BLOCKSIZE];
  size_t len;
  int    i;
  int    padbytes;

  ctx = &aes;

  if (iv)
    memcpy(ctx->iv, iv, AES_BLOCKSIZE);
  else {
    for (i = 0; i < AES_BLOCKSIZE; i++)
      ctx->iv[i] = rand() % 256;
  }
  /* 16 bytes aligned.
   * Note that when padding is enabled there can be excess 16-byte
   * filled with 0x10. It occurs when size of the input data is multiple
   * of 16.
   */
  padbytes = padding ? AES_BLOCKSIZE - (plain_len % AES_BLOCKSIZE) : \
                       ((plain_len % AES_BLOCKSIZE) ? \
                         AES_BLOCKSIZE - (plain_len % AES_BLOCKSIZE) : 0);

  /* We do NOT write IV to the output stream if IV is explicitly specified. */
  *cipher_len = plain_len + (iv ? 0 : AES_BLOCKSIZE) + padbytes;
  *cipher     = NEW(*cipher_len, unsigned char);

  ctx->nrounds = rijndaelSetupEncrypt(ctx->rk, key, key_len * 8);

  inptr = plain; outptr = *cipher;
  if (!iv) {
    memcpy(outptr, ctx->iv, AES_BLOCKSIZE);
    outptr += AES_BLOCKSIZE;
  }
  for (len = plain_len; len >= AES_BLOCKSIZE; len -= AES_BLOCKSIZE) {
    for (i = 0; i < AES_BLOCKSIZE; i++)
      block[i] = inptr[i] ^ ctx->iv[i];
    rijndaelEncrypt(ctx->rk, ctx->nrounds, block, outptr);
    memcpy(ctx->iv, outptr, AES_BLOCKSIZE);
    inptr  += AES_BLOCKSIZE;
    outptr += AES_BLOCKSIZE;
  }
  if (len > 0 || padding) {
    for (i = 0; i < len; i++)
      block[i] = inptr[i] ^ ctx->iv[i];
    for (i = len; i < AES_BLOCKSIZE; i++)
      block[i] = padbytes ^ ctx->iv[i];
    rijndaelEncrypt(ctx->rk, ctx->nrounds, block, outptr);
    memcpy(ctx->iv, outptr, AES_BLOCKSIZE);
    inptr  += AES_BLOCKSIZE;
    outptr += AES_BLOCKSIZE;
  }
}

/* The following section contains a Rijndael encryption implementation
 * based on code from Philip J. Erdelsky's public domain one.
 * They can be obtained from
 *
 *   http://www.efgh.com/software/rijndael.htm
 *   http://www.efgh.com/software/rijndael.txt
 */

#define FULL_UNROLL

static const uint32_t Te0[256] =
{
  0xc66363a5U, 0xf87c7c84U, 0xee777799U, 0xf67b7b8dU,
  0xfff2f20dU, 0xd66b6bbdU, 0xde6f6fb1U, 0x91c5c554U,
  0x60303050U, 0x02010103U, 0xce6767a9U, 0x562b2b7dU,
  0xe7fefe19U, 0xb5d7d762U, 0x4dababe6U, 0xec76769aU,
  0x8fcaca45U, 0x1f82829dU, 0x89c9c940U, 0xfa7d7d87U,
  0xeffafa15U, 0xb25959ebU, 0x8e4747c9U, 0xfbf0f00bU,
  0x41adadecU, 0xb3d4d467U, 0x5fa2a2fdU, 0x45afafeaU,
  0x239c9cbfU, 0x53a4a4f7U, 0xe4727296U, 0x9bc0c05bU,
  0x75b7b7c2U, 0xe1fdfd1cU, 0x3d9393aeU, 0x4c26266aU,
  0x6c36365aU, 0x7e3f3f41U, 0xf5f7f702U, 0x83cccc4fU,
  0x6834345cU, 0x51a5a5f4U, 0xd1e5e534U, 0xf9f1f108U,
  0xe2717193U, 0xabd8d873U, 0x62313153U, 0x2a15153fU,
  0x0804040cU, 0x95c7c752U, 0x46232365U, 0x9dc3c35eU,
  0x30181828U, 0x379696a1U, 0x0a05050fU, 0x2f9a9ab5U,
  0x0e070709U, 0x24121236U, 0x1b80809bU, 0xdfe2e23dU,
  0xcdebeb26U, 0x4e272769U, 0x7fb2b2cdU, 0xea75759fU,
  0x1209091bU, 0x1d83839eU, 0x582c2c74U, 0x341a1a2eU,
  0x361b1b2dU, 0xdc6e6eb2U, 0xb45a5aeeU, 0x5ba0a0fbU,
  0xa45252f6U, 0x763b3b4dU, 0xb7d6d661U, 0x7db3b3ceU,
  0x5229297bU, 0xdde3e33eU, 0x5e2f2f71U, 0x13848497U,
  0xa65353f5U, 0xb9d1d168U, 0x00000000U, 0xc1eded2cU,
  0x40202060U, 0xe3fcfc1fU, 0x79b1b1c8U, 0xb65b5bedU,
  0xd46a6abeU, 0x8dcbcb46U, 0x67bebed9U, 0x7239394bU,
  0x944a4adeU, 0x984c4cd4U, 0xb05858e8U, 0x85cfcf4aU,
  0xbbd0d06bU, 0xc5efef2aU, 0x4faaaae5U, 0xedfbfb16U,
  0x864343c5U, 0x9a4d4dd7U, 0x66333355U, 0x11858594U,
  0x8a4545cfU, 0xe9f9f910U, 0x04020206U, 0xfe7f7f81U,
  0xa05050f0U, 0x783c3c44U, 0x259f9fbaU, 0x4ba8a8e3U,
  0xa25151f3U, 0x5da3a3feU, 0x804040c0U, 0x058f8f8aU,
  0x3f9292adU, 0x219d9dbcU, 0x70383848U, 0xf1f5f504U,
  0x63bcbcdfU, 0x77b6b6c1U, 0xafdada75U, 0x42212163U,
  0x20101030U, 0xe5ffff1aU, 0xfdf3f30eU, 0xbfd2d26dU,
  0x81cdcd4cU, 0x180c0c14U, 0x26131335U, 0xc3ecec2fU,
  0xbe5f5fe1U, 0x359797a2U, 0x884444ccU, 0x2e171739U,
  0x93c4c457U, 0x55a7a7f2U, 0xfc7e7e82U, 0x7a3d3d47U,
  0xc86464acU, 0xba5d5de7U, 0x3219192bU, 0xe6737395U,
  0xc06060a0U, 0x19818198U, 0x9e4f4fd1U, 0xa3dcdc7fU,
  0x44222266U, 0x542a2a7eU, 0x3b9090abU, 0x0b888883U,
  0x8c4646caU, 0xc7eeee29U, 0x6bb8b8d3U, 0x2814143cU,
  0xa7dede79U, 0xbc5e5ee2U, 0x160b0b1dU, 0xaddbdb76U,
  0xdbe0e03bU, 0x64323256U, 0x743a3a4eU, 0x140a0a1eU,
  0x924949dbU, 0x0c06060aU, 0x4824246cU, 0xb85c5ce4U,
  0x9fc2c25dU, 0xbdd3d36eU, 0x43acacefU, 0xc46262a6U,
  0x399191a8U, 0x319595a4U, 0xd3e4e437U, 0xf279798bU,
  0xd5e7e732U, 0x8bc8c843U, 0x6e373759U, 0xda6d6db7U,
  0x018d8d8cU, 0xb1d5d564U, 0x9c4e4ed2U, 0x49a9a9e0U,
  0xd86c6cb4U, 0xac5656faU, 0xf3f4f407U, 0xcfeaea25U,
  0xca6565afU, 0xf47a7a8eU, 0x47aeaee9U, 0x10080818U,
  0x6fbabad5U, 0xf0787888U, 0x4a25256fU, 0x5c2e2e72U,
  0x381c1c24U, 0x57a6a6f1U, 0x73b4b4c7U, 0x97c6c651U,
  0xcbe8e823U, 0xa1dddd7cU, 0xe874749cU, 0x3e1f1f21U,
  0x964b4bddU, 0x61bdbddcU, 0x0d8b8b86U, 0x0f8a8a85U,
  0xe0707090U, 0x7c3e3e42U, 0x71b5b5c4U, 0xcc6666aaU,
  0x904848d8U, 0x06030305U, 0xf7f6f601U, 0x1c0e0e12U,
  0xc26161a3U, 0x6a35355fU, 0xae5757f9U, 0x69b9b9d0U,
  0x17868691U, 0x99c1c158U, 0x3a1d1d27U, 0x279e9eb9U,
  0xd9e1e138U, 0xebf8f813U, 0x2b9898b3U, 0x22111133U,
  0xd26969bbU, 0xa9d9d970U, 0x078e8e89U, 0x339494a7U,
  0x2d9b9bb6U, 0x3c1e1e22U, 0x15878792U, 0xc9e9e920U,
  0x87cece49U, 0xaa5555ffU, 0x50282878U, 0xa5dfdf7aU,
  0x038c8c8fU, 0x59a1a1f8U, 0x09898980U, 0x1a0d0d17U,
  0x65bfbfdaU, 0xd7e6e631U, 0x844242c6U, 0xd06868b8U,
  0x824141c3U, 0x299999b0U, 0x5a2d2d77U, 0x1e0f0f11U,
  0x7bb0b0cbU, 0xa85454fcU, 0x6dbbbbd6U, 0x2c16163aU,
};

static const uint32_t Te1[256] =
{
  0xa5c66363U, 0x84f87c7cU, 0x99ee7777U, 0x8df67b7bU,
  0x0dfff2f2U, 0xbdd66b6bU, 0xb1de6f6fU, 0x5491c5c5U,
  0x50603030U, 0x03020101U, 0xa9ce6767U, 0x7d562b2bU,
  0x19e7fefeU, 0x62b5d7d7U, 0xe64dababU, 0x9aec7676U,
  0x458fcacaU, 0x9d1f8282U, 0x4089c9c9U, 0x87fa7d7dU,
  0x15effafaU, 0xebb25959U, 0xc98e4747U, 0x0bfbf0f0U,
  0xec41adadU, 0x67b3d4d4U, 0xfd5fa2a2U, 0xea45afafU,
  0xbf239c9cU, 0xf753a4a4U, 0x96e47272U, 0x5b9bc0c0U,
  0xc275b7b7U, 0x1ce1fdfdU, 0xae3d9393U, 0x6a4c2626U,
  0x5a6c3636U, 0x417e3f3fU, 0x02f5f7f7U, 0x4f83ccccU,
  0x5c683434U, 0xf451a5a5U, 0x34d1e5e5U, 0x08f9f1f1U,
  0x93e27171U, 0x73abd8d8U, 0x53623131U, 0x3f2a1515U,
  0x0c080404U, 0x5295c7c7U, 0x65462323U, 0x5e9dc3c3U,
  0x28301818U, 0xa1379696U, 0x0f0a0505U, 0xb52f9a9aU,
  0x090e0707U, 0x36241212U, 0x9b1b8080U, 0x3ddfe2e2U,
  0x26cdebebU, 0x694e2727U, 0xcd7fb2b2U, 0x9fea7575U,
  0x1b120909U, 0x9e1d8383U, 0x74582c2cU, 0x2e341a1aU,
  0x2d361b1bU, 0xb2dc6e6eU, 0xeeb45a5aU, 0xfb5ba0a0U,
  0xf6a45252U, 0x4d763b3bU, 0x61b7d6d6U, 0xce7db3b3U,
  0x7b522929U, 0x3edde3e3U, 0x715e2f2fU, 0x97138484U,
  0xf5a65353U, 0x68b9d1d1U, 0x00000000U, 0x2cc1ededU,
  0x60402020U, 0x1fe3fcfcU, 0xc879b1b1U, 0xedb65b5bU,
  0xbed46a6aU, 0x468dcbcbU, 0xd967bebeU, 0x4b723939U,
  0xde944a4aU, 0xd4984c4cU, 0xe8b05858U, 0x4a85cfcfU,
  0x6bbbd0d0U, 0x2ac5efefU, 0xe54faaaaU, 0x16edfbfbU,
  0xc5864343U, 0xd79a4d4dU, 0x55663333U, 0x94118585U,
  0xcf8a4545U, 0x10e9f9f9U, 0x06040202U, 0x81fe7f7fU,
  0xf0a05050U, 0x44783c3cU, 0xba259f9fU, 0xe34ba8a8U,
  0xf3a25151U, 0xfe5da3a3U, 0xc0804040U, 0x8a058f8fU,
  0xad3f9292U, 0xbc219d9dU, 0x48703838U, 0x04f1f5f5U,
  0xdf63bcbcU, 0xc177b6b6U, 0x75afdadaU, 0x63422121U,
  0x30201010U, 0x1ae5ffffU, 0x0efdf3f3U, 0x6dbfd2d2U,
  0x4c81cdcdU, 0x14180c0cU, 0x35261313U, 0x2fc3ececU,
  0xe1be5f5fU, 0xa2359797U, 0xcc884444U, 0x392e1717U,
  0x5793c4c4U, 0xf255a7a7U, 0x82fc7e7eU, 0x477a3d3dU,
  0xacc86464U, 0xe7ba5d5dU, 0x2b321919U, 0x95e67373U,
  0xa0c06060U, 0x98198181U, 0xd19e4f4fU, 0x7fa3dcdcU,
  0x66442222U, 0x7e542a2aU, 0xab3b9090U, 0x830b8888U,
  0xca8c4646U, 0x29c7eeeeU, 0xd36bb8b8U, 0x3c281414U,
  0x79a7dedeU, 0xe2bc5e5eU, 0x1d160b0bU, 0x76addbdbU,
  0x3bdbe0e0U, 0x56643232U, 0x4e743a3aU, 0x1e140a0aU,
  0xdb924949U, 0x0a0c0606U, 0x6c482424U, 0xe4b85c5cU,
  0x5d9fc2c2U, 0x6ebdd3d3U, 0xef43acacU, 0xa6c46262U,
  0xa8399191U, 0xa4319595U, 0x37d3e4e4U, 0x8bf27979U,
  0x32d5e7e7U, 0x438bc8c8U, 0x596e3737U, 0xb7da6d6dU,
  0x8c018d8dU, 0x64b1d5d5U, 0xd29c4e4eU, 0xe049a9a9U,
  0xb4d86c6cU, 0xfaac5656U, 0x07f3f4f4U, 0x25cfeaeaU,
  0xafca6565U, 0x8ef47a7aU, 0xe947aeaeU, 0x18100808U,
  0xd56fbabaU, 0x88f07878U, 0x6f4a2525U, 0x725c2e2eU,
  0x24381c1cU, 0xf157a6a6U, 0xc773b4b4U, 0x5197c6c6U,
  0x23cbe8e8U, 0x7ca1ddddU, 0x9ce87474U, 0x213e1f1fU,
  0xdd964b4bU, 0xdc61bdbdU, 0x860d8b8bU, 0x850f8a8aU,
  0x90e07070U, 0x427c3e3eU, 0xc471b5b5U, 0xaacc6666U,
  0xd8904848U, 0x05060303U, 0x01f7f6f6U, 0x121c0e0eU,
  0xa3c26161U, 0x5f6a3535U, 0xf9ae5757U, 0xd069b9b9U,
  0x91178686U, 0x5899c1c1U, 0x273a1d1dU, 0xb9279e9eU,
  0x38d9e1e1U, 0x13ebf8f8U, 0xb32b9898U, 0x33221111U,
  0xbbd26969U, 0x70a9d9d9U, 0x89078e8eU, 0xa7339494U,
  0xb62d9b9bU, 0x223c1e1eU, 0x92158787U, 0x20c9e9e9U,
  0x4987ceceU, 0xffaa5555U, 0x78502828U, 0x7aa5dfdfU,
  0x8f038c8cU, 0xf859a1a1U, 0x80098989U, 0x171a0d0dU,
  0xda65bfbfU, 0x31d7e6e6U, 0xc6844242U, 0xb8d06868U,
  0xc3824141U, 0xb0299999U, 0x775a2d2dU, 0x111e0f0fU,
  0xcb7bb0b0U, 0xfca85454U, 0xd66dbbbbU, 0x3a2c1616U,
};

static const uint32_t Te2[256] =
{
  0x63a5c663U, 0x7c84f87cU, 0x7799ee77U, 0x7b8df67bU,
  0xf20dfff2U, 0x6bbdd66bU, 0x6fb1de6fU, 0xc55491c5U,
  0x30506030U, 0x01030201U, 0x67a9ce67U, 0x2b7d562bU,
  0xfe19e7feU, 0xd762b5d7U, 0xabe64dabU, 0x769aec76U,
  0xca458fcaU, 0x829d1f82U, 0xc94089c9U, 0x7d87fa7dU,
  0xfa15effaU, 0x59ebb259U, 0x47c98e47U, 0xf00bfbf0U,
  0xadec41adU, 0xd467b3d4U, 0xa2fd5fa2U, 0xafea45afU,
  0x9cbf239cU, 0xa4f753a4U, 0x7296e472U, 0xc05b9bc0U,
  0xb7c275b7U, 0xfd1ce1fdU, 0x93ae3d93U, 0x266a4c26U,
  0x365a6c36U, 0x3f417e3fU, 0xf702f5f7U, 0xcc4f83ccU,
  0x345c6834U, 0xa5f451a5U, 0xe534d1e5U, 0xf108f9f1U,
  0x7193e271U, 0xd873abd8U, 0x31536231U, 0x153f2a15U,
  0x040c0804U, 0xc75295c7U, 0x23654623U, 0xc35e9dc3U,
  0x18283018U, 0x96a13796U, 0x050f0a05U, 0x9ab52f9aU,
  0x07090e07U, 0x12362412U, 0x809b1b80U, 0xe23ddfe2U,
  0xeb26cdebU, 0x27694e27U, 0xb2cd7fb2U, 0x759fea75U,
  0x091b1209U, 0x839e1d83U, 0x2c74582cU, 0x1a2e341aU,
  0x1b2d361bU, 0x6eb2dc6eU, 0x5aeeb45aU, 0xa0fb5ba0U,
  0x52f6a452U, 0x3b4d763bU, 0xd661b7d6U, 0xb3ce7db3U,
  0x297b5229U, 0xe33edde3U, 0x2f715e2fU, 0x84971384U,
  0x53f5a653U, 0xd168b9d1U, 0x00000000U, 0xed2cc1edU,
  0x20604020U, 0xfc1fe3fcU, 0xb1c879b1U, 0x5bedb65bU,
  0x6abed46aU, 0xcb468dcbU, 0xbed967beU, 0x394b7239U,
  0x4ade944aU, 0x4cd4984cU, 0x58e8b058U, 0xcf4a85cfU,
  0xd06bbbd0U, 0xef2ac5efU, 0xaae54faaU, 0xfb16edfbU,
  0x43c58643U, 0x4dd79a4dU, 0x33556633U, 0x85941185U,
  0x45cf8a45U, 0xf910e9f9U, 0x02060402U, 0x7f81fe7fU,
  0x50f0a050U, 0x3c44783cU, 0x9fba259fU, 0xa8e34ba8U,
  0x51f3a251U, 0xa3fe5da3U, 0x40c08040U, 0x8f8a058fU,
  0x92ad3f92U, 0x9dbc219dU, 0x38487038U, 0xf504f1f5U,
  0xbcdf63bcU, 0xb6c177b6U, 0xda75afdaU, 0x21634221U,
  0x10302010U, 0xff1ae5ffU, 0xf30efdf3U, 0xd26dbfd2U,
  0xcd4c81cdU, 0x0c14180cU, 0x13352613U, 0xec2fc3ecU,
  0x5fe1be5fU, 0x97a23597U, 0x44cc8844U, 0x17392e17U,
  0xc45793c4U, 0xa7f255a7U, 0x7e82fc7eU, 0x3d477a3dU,
  0x64acc864U, 0x5de7ba5dU, 0x192b3219U, 0x7395e673U,
  0x60a0c060U, 0x81981981U, 0x4fd19e4fU, 0xdc7fa3dcU,
  0x22664422U, 0x2a7e542aU, 0x90ab3b90U, 0x88830b88U,
  0x46ca8c46U, 0xee29c7eeU, 0xb8d36bb8U, 0x143c2814U,
  0xde79a7deU, 0x5ee2bc5eU, 0x0b1d160bU, 0xdb76addbU,
  0xe03bdbe0U, 0x32566432U, 0x3a4e743aU, 0x0a1e140aU,
  0x49db9249U, 0x060a0c06U, 0x246c4824U, 0x5ce4b85cU,
  0xc25d9fc2U, 0xd36ebdd3U, 0xacef43acU, 0x62a6c462U,
  0x91a83991U, 0x95a43195U, 0xe437d3e4U, 0x798bf279U,
  0xe732d5e7U, 0xc8438bc8U, 0x37596e37U, 0x6db7da6dU,
  0x8d8c018dU, 0xd564b1d5U, 0x4ed29c4eU, 0xa9e049a9U,
  0x6cb4d86cU, 0x56faac56U, 0xf407f3f4U, 0xea25cfeaU,
  0x65afca65U, 0x7a8ef47aU, 0xaee947aeU, 0x08181008U,
  0xbad56fbaU, 0x7888f078U, 0x256f4a25U, 0x2e725c2eU,
  0x1c24381cU, 0xa6f157a6U, 0xb4c773b4U, 0xc65197c6U,
  0xe823cbe8U, 0xdd7ca1ddU, 0x749ce874U, 0x1f213e1fU,
  0x4bdd964bU, 0xbddc61bdU, 0x8b860d8bU, 0x8a850f8aU,
  0x7090e070U, 0x3e427c3eU, 0xb5c471b5U, 0x66aacc66U,
  0x48d89048U, 0x03050603U, 0xf601f7f6U, 0x0e121c0eU,
  0x61a3c261U, 0x355f6a35U, 0x57f9ae57U, 0xb9d069b9U,
  0x86911786U, 0xc15899c1U, 0x1d273a1dU, 0x9eb9279eU,
  0xe138d9e1U, 0xf813ebf8U, 0x98b32b98U, 0x11332211U,
  0x69bbd269U, 0xd970a9d9U, 0x8e89078eU, 0x94a73394U,
  0x9bb62d9bU, 0x1e223c1eU, 0x87921587U, 0xe920c9e9U,
  0xce4987ceU, 0x55ffaa55U, 0x28785028U, 0xdf7aa5dfU,
  0x8c8f038cU, 0xa1f859a1U, 0x89800989U, 0x0d171a0dU,
  0xbfda65bfU, 0xe631d7e6U, 0x42c68442U, 0x68b8d068U,
  0x41c38241U, 0x99b02999U, 0x2d775a2dU, 0x0f111e0fU,
  0xb0cb7bb0U, 0x54fca854U, 0xbbd66dbbU, 0x163a2c16U,
};

static const uint32_t Te3[256] =
{
  0x6363a5c6U, 0x7c7c84f8U, 0x777799eeU, 0x7b7b8df6U,
  0xf2f20dffU, 0x6b6bbdd6U, 0x6f6fb1deU, 0xc5c55491U,
  0x30305060U, 0x01010302U, 0x6767a9ceU, 0x2b2b7d56U,
  0xfefe19e7U, 0xd7d762b5U, 0xababe64dU, 0x76769aecU,
  0xcaca458fU, 0x82829d1fU, 0xc9c94089U, 0x7d7d87faU,
  0xfafa15efU, 0x5959ebb2U, 0x4747c98eU, 0xf0f00bfbU,
  0xadadec41U, 0xd4d467b3U, 0xa2a2fd5fU, 0xafafea45U,
  0x9c9cbf23U, 0xa4a4f753U, 0x727296e4U, 0xc0c05b9bU,
  0xb7b7c275U, 0xfdfd1ce1U, 0x9393ae3dU, 0x26266a4cU,
  0x36365a6cU, 0x3f3f417eU, 0xf7f702f5U, 0xcccc4f83U,
  0x34345c68U, 0xa5a5f451U, 0xe5e534d1U, 0xf1f108f9U,
  0x717193e2U, 0xd8d873abU, 0x31315362U, 0x15153f2aU,
  0x04040c08U, 0xc7c75295U, 0x23236546U, 0xc3c35e9dU,
  0x18182830U, 0x9696a137U, 0x05050f0aU, 0x9a9ab52fU,
  0x0707090eU, 0x12123624U, 0x80809b1bU, 0xe2e23ddfU,
  0xebeb26cdU, 0x2727694eU, 0xb2b2cd7fU, 0x75759feaU,
  0x09091b12U, 0x83839e1dU, 0x2c2c7458U, 0x1a1a2e34U,
  0x1b1b2d36U, 0x6e6eb2dcU, 0x5a5aeeb4U, 0xa0a0fb5bU,
  0x5252f6a4U, 0x3b3b4d76U, 0xd6d661b7U, 0xb3b3ce7dU,
  0x29297b52U, 0xe3e33eddU, 0x2f2f715eU, 0x84849713U,
  0x5353f5a6U, 0xd1d168b9U, 0x00000000U, 0xeded2cc1U,
  0x20206040U, 0xfcfc1fe3U, 0xb1b1c879U, 0x5b5bedb6U,
  0x6a6abed4U, 0xcbcb468dU, 0xbebed967U, 0x39394b72U,
  0x4a4ade94U, 0x4c4cd498U, 0x5858e8b0U, 0xcfcf4a85U,
  0xd0d06bbbU, 0xefef2ac5U, 0xaaaae54fU, 0xfbfb16edU,
  0x4343c586U, 0x4d4dd79aU, 0x33335566U, 0x85859411U,
  0x4545cf8aU, 0xf9f910e9U, 0x02020604U, 0x7f7f81feU,
  0x5050f0a0U, 0x3c3c4478U, 0x9f9fba25U, 0xa8a8e34bU,
  0x5151f3a2U, 0xa3a3fe5dU, 0x4040c080U, 0x8f8f8a05U,
  0x9292ad3fU, 0x9d9dbc21U, 0x38384870U, 0xf5f504f1U,
  0xbcbcdf63U, 0xb6b6c177U, 0xdada75afU, 0x21216342U,
  0x10103020U, 0xffff1ae5U, 0xf3f30efdU, 0xd2d26dbfU,
  0xcdcd4c81U, 0x0c0c1418U, 0x13133526U, 0xecec2fc3U,
  0x5f5fe1beU, 0x9797a235U, 0x4444cc88U, 0x1717392eU,
  0xc4c45793U, 0xa7a7f255U, 0x7e7e82fcU, 0x3d3d477aU,
  0x6464acc8U, 0x5d5de7baU, 0x19192b32U, 0x737395e6U,
  0x6060a0c0U, 0x81819819U, 0x4f4fd19eU, 0xdcdc7fa3U,
  0x22226644U, 0x2a2a7e54U, 0x9090ab3bU, 0x8888830bU,
  0x4646ca8cU, 0xeeee29c7U, 0xb8b8d36bU, 0x14143c28U,
  0xdede79a7U, 0x5e5ee2bcU, 0x0b0b1d16U, 0xdbdb76adU,
  0xe0e03bdbU, 0x32325664U, 0x3a3a4e74U, 0x0a0a1e14U,
  0x4949db92U, 0x06060a0cU, 0x24246c48U, 0x5c5ce4b8U,
  0xc2c25d9fU, 0xd3d36ebdU, 0xacacef43U, 0x6262a6c4U,
  0x9191a839U, 0x9595a431U, 0xe4e437d3U, 0x79798bf2U,
  0xe7e732d5U, 0xc8c8438bU, 0x3737596eU, 0x6d6db7daU,
  0x8d8d8c01U, 0xd5d564b1U, 0x4e4ed29cU, 0xa9a9e049U,
  0x6c6cb4d8U, 0x5656faacU, 0xf4f407f3U, 0xeaea25cfU,
  0x6565afcaU, 0x7a7a8ef4U, 0xaeaee947U, 0x08081810U,
  0xbabad56fU, 0x787888f0U, 0x25256f4aU, 0x2e2e725cU,
  0x1c1c2438U, 0xa6a6f157U, 0xb4b4c773U, 0xc6c65197U,
  0xe8e823cbU, 0xdddd7ca1U, 0x74749ce8U, 0x1f1f213eU,
  0x4b4bdd96U, 0xbdbddc61U, 0x8b8b860dU, 0x8a8a850fU,
  0x707090e0U, 0x3e3e427cU, 0xb5b5c471U, 0x6666aaccU,
  0x4848d890U, 0x03030506U, 0xf6f601f7U, 0x0e0e121cU,
  0x6161a3c2U, 0x35355f6aU, 0x5757f9aeU, 0xb9b9d069U,
  0x86869117U, 0xc1c15899U, 0x1d1d273aU, 0x9e9eb927U,
  0xe1e138d9U, 0xf8f813ebU, 0x9898b32bU, 0x11113322U,
  0x6969bbd2U, 0xd9d970a9U, 0x8e8e8907U, 0x9494a733U,
  0x9b9bb62dU, 0x1e1e223cU, 0x87879215U, 0xe9e920c9U,
  0xcece4987U, 0x5555ffaaU, 0x28287850U, 0xdfdf7aa5U,
  0x8c8c8f03U, 0xa1a1f859U, 0x89898009U, 0x0d0d171aU,
  0xbfbfda65U, 0xe6e631d7U, 0x4242c684U, 0x6868b8d0U,
  0x4141c382U, 0x9999b029U, 0x2d2d775aU, 0x0f0f111eU,
  0xb0b0cb7bU, 0x5454fca8U, 0xbbbbd66dU, 0x16163a2cU,
};

static const uint32_t Te4[256] =
{
  0x63636363U, 0x7c7c7c7cU, 0x77777777U, 0x7b7b7b7bU,
  0xf2f2f2f2U, 0x6b6b6b6bU, 0x6f6f6f6fU, 0xc5c5c5c5U,
  0x30303030U, 0x01010101U, 0x67676767U, 0x2b2b2b2bU,
  0xfefefefeU, 0xd7d7d7d7U, 0xababababU, 0x76767676U,
  0xcacacacaU, 0x82828282U, 0xc9c9c9c9U, 0x7d7d7d7dU,
  0xfafafafaU, 0x59595959U, 0x47474747U, 0xf0f0f0f0U,
  0xadadadadU, 0xd4d4d4d4U, 0xa2a2a2a2U, 0xafafafafU,
  0x9c9c9c9cU, 0xa4a4a4a4U, 0x72727272U, 0xc0c0c0c0U,
  0xb7b7b7b7U, 0xfdfdfdfdU, 0x93939393U, 0x26262626U,
  0x36363636U, 0x3f3f3f3fU, 0xf7f7f7f7U, 0xccccccccU,
  0x34343434U, 0xa5a5a5a5U, 0xe5e5e5e5U, 0xf1f1f1f1U,
  0x71717171U, 0xd8d8d8d8U, 0x31313131U, 0x15151515U,
  0x04040404U, 0xc7c7c7c7U, 0x23232323U, 0xc3c3c3c3U,
  0x18181818U, 0x96969696U, 0x05050505U, 0x9a9a9a9aU,
  0x07070707U, 0x12121212U, 0x80808080U, 0xe2e2e2e2U,
  0xebebebebU, 0x27272727U, 0xb2b2b2b2U, 0x75757575U,
  0x09090909U, 0x83838383U, 0x2c2c2c2cU, 0x1a1a1a1aU,
  0x1b1b1b1bU, 0x6e6e6e6eU, 0x5a5a5a5aU, 0xa0a0a0a0U,
  0x52525252U, 0x3b3b3b3bU, 0xd6d6d6d6U, 0xb3b3b3b3U,
  0x29292929U, 0xe3e3e3e3U, 0x2f2f2f2fU, 0x84848484U,
  0x53535353U, 0xd1d1d1d1U, 0x00000000U, 0xededededU,
  0x20202020U, 0xfcfcfcfcU, 0xb1b1b1b1U, 0x5b5b5b5bU,
  0x6a6a6a6aU, 0xcbcbcbcbU, 0xbebebebeU, 0x39393939U,
  0x4a4a4a4aU, 0x4c4c4c4cU, 0x58585858U, 0xcfcfcfcfU,
  0xd0d0d0d0U, 0xefefefefU, 0xaaaaaaaaU, 0xfbfbfbfbU,
  0x43434343U, 0x4d4d4d4dU, 0x33333333U, 0x85858585U,
  0x45454545U, 0xf9f9f9f9U, 0x02020202U, 0x7f7f7f7fU,
  0x50505050U, 0x3c3c3c3cU, 0x9f9f9f9fU, 0xa8a8a8a8U,
  0x51515151U, 0xa3a3a3a3U, 0x40404040U, 0x8f8f8f8fU,
  0x92929292U, 0x9d9d9d9dU, 0x38383838U, 0xf5f5f5f5U,
  0xbcbcbcbcU, 0xb6b6b6b6U, 0xdadadadaU, 0x21212121U,
  0x10101010U, 0xffffffffU, 0xf3f3f3f3U, 0xd2d2d2d2U,
  0xcdcdcdcdU, 0x0c0c0c0cU, 0x13131313U, 0xececececU,
  0x5f5f5f5fU, 0x97979797U, 0x44444444U, 0x17171717U,
  0xc4c4c4c4U, 0xa7a7a7a7U, 0x7e7e7e7eU, 0x3d3d3d3dU,
  0x64646464U, 0x5d5d5d5dU, 0x19191919U, 0x73737373U,
  0x60606060U, 0x81818181U, 0x4f4f4f4fU, 0xdcdcdcdcU,
  0x22222222U, 0x2a2a2a2aU, 0x90909090U, 0x88888888U,
  0x46464646U, 0xeeeeeeeeU, 0xb8b8b8b8U, 0x14141414U,
  0xdedededeU, 0x5e5e5e5eU, 0x0b0b0b0bU, 0xdbdbdbdbU,
  0xe0e0e0e0U, 0x32323232U, 0x3a3a3a3aU, 0x0a0a0a0aU,
  0x49494949U, 0x06060606U, 0x24242424U, 0x5c5c5c5cU,
  0xc2c2c2c2U, 0xd3d3d3d3U, 0xacacacacU, 0x62626262U,
  0x91919191U, 0x95959595U, 0xe4e4e4e4U, 0x79797979U,
  0xe7e7e7e7U, 0xc8c8c8c8U, 0x37373737U, 0x6d6d6d6dU,
  0x8d8d8d8dU, 0xd5d5d5d5U, 0x4e4e4e4eU, 0xa9a9a9a9U,
  0x6c6c6c6cU, 0x56565656U, 0xf4f4f4f4U, 0xeaeaeaeaU,
  0x65656565U, 0x7a7a7a7aU, 0xaeaeaeaeU, 0x08080808U,
  0xbabababaU, 0x78787878U, 0x25252525U, 0x2e2e2e2eU,
  0x1c1c1c1cU, 0xa6a6a6a6U, 0xb4b4b4b4U, 0xc6c6c6c6U,
  0xe8e8e8e8U, 0xddddddddU, 0x74747474U, 0x1f1f1f1fU,
  0x4b4b4b4bU, 0xbdbdbdbdU, 0x8b8b8b8bU, 0x8a8a8a8aU,
  0x70707070U, 0x3e3e3e3eU, 0xb5b5b5b5U, 0x66666666U,
  0x48484848U, 0x03030303U, 0xf6f6f6f6U, 0x0e0e0e0eU,
  0x61616161U, 0x35353535U, 0x57575757U, 0xb9b9b9b9U,
  0x86868686U, 0xc1c1c1c1U, 0x1d1d1d1dU, 0x9e9e9e9eU,
  0xe1e1e1e1U, 0xf8f8f8f8U, 0x98989898U, 0x11111111U,
  0x69696969U, 0xd9d9d9d9U, 0x8e8e8e8eU, 0x94949494U,
  0x9b9b9b9bU, 0x1e1e1e1eU, 0x87878787U, 0xe9e9e9e9U,
  0xcecececeU, 0x55555555U, 0x28282828U, 0xdfdfdfdfU,
  0x8c8c8c8cU, 0xa1a1a1a1U, 0x89898989U, 0x0d0d0d0dU,
  0xbfbfbfbfU, 0xe6e6e6e6U, 0x42424242U, 0x68686868U,
  0x41414141U, 0x99999999U, 0x2d2d2d2dU, 0x0f0f0f0fU,
  0xb0b0b0b0U, 0x54545454U, 0xbbbbbbbbU, 0x16161616U,
};

static const uint32_t rcon[] =
{
  0x01000000, 0x02000000, 0x04000000, 0x08000000,
  0x10000000, 0x20000000, 0x40000000, 0x80000000,
  0x1B000000, 0x36000000,
  /* for 128-bit blocks, Rijndael never uses more than 10 rcon values */
};

#define GETU32(plaintext) (((uint32_t)(plaintext)[0] << 24) ^ \
                    ((uint32_t)(plaintext)[1] << 16) ^ \
                    ((uint32_t)(plaintext)[2] <<  8) ^ \
                    ((uint32_t)(plaintext)[3]))

#define PUTU32(ciphertext, st) { (ciphertext)[0] = (uint8_t)((st) >> 24); \
                         (ciphertext)[1] = (uint8_t)((st) >> 16); \
                         (ciphertext)[2] = (uint8_t)((st) >>  8); \
                         (ciphertext)[3] = (uint8_t)(st); }

/**
 * Expand the cipher key into the encryption key schedule.
 *
 * @return the number of rounds for the given cipher key size.
 */
int rijndaelSetupEncrypt(uint32_t *rk, const uint8_t *key, int keybits)
{
  int       i = 0;
  uint32_t  temp;

  rk[0] = GETU32(key     );
  rk[1] = GETU32(key +  4);
  rk[2] = GETU32(key +  8);
  rk[3] = GETU32(key + 12);
  if (keybits == 128)
  {
    for (;;)
    {
      temp  = rk[3];
      rk[4] = rk[0] ^
        (Te4[(temp >> 16) & 0xff] & 0xff000000) ^
        (Te4[(temp >>  8) & 0xff] & 0x00ff0000) ^
        (Te4[(temp      ) & 0xff] & 0x0000ff00) ^
        (Te4[(temp >> 24)       ] & 0x000000ff) ^
        rcon[i];
      rk[5] = rk[1] ^ rk[4];
      rk[6] = rk[2] ^ rk[5];
      rk[7] = rk[3] ^ rk[6];
      if (++i == 10)
        return 10;
      rk += 4;
    }
  }
  rk[4] = GETU32(key + 16);
  rk[5] = GETU32(key + 20);
  if (keybits == 192)
  {
    for (;;)
    {
      temp = rk[ 5];
      rk[ 6] = rk[ 0] ^
        (Te4[(temp >> 16) & 0xff] & 0xff000000) ^
        (Te4[(temp >>  8) & 0xff] & 0x00ff0000) ^
        (Te4[(temp      ) & 0xff] & 0x0000ff00) ^
        (Te4[(temp >> 24)       ] & 0x000000ff) ^
        rcon[i];
      rk[ 7] = rk[ 1] ^ rk[ 6];
      rk[ 8] = rk[ 2] ^ rk[ 7];
      rk[ 9] = rk[ 3] ^ rk[ 8];
      if (++i == 8)
        return 12;
      rk[10] = rk[ 4] ^ rk[ 9];
      rk[11] = rk[ 5] ^ rk[10];
      rk += 6;
    }
  }
  rk[6] = GETU32(key + 24);
  rk[7] = GETU32(key + 28);
  if (keybits == 256)
  {
    for (;;)
    {
      temp = rk[ 7];
      rk[ 8] = rk[ 0] ^
        (Te4[(temp >> 16) & 0xff] & 0xff000000) ^
        (Te4[(temp >>  8) & 0xff] & 0x00ff0000) ^
        (Te4[(temp      ) & 0xff] & 0x0000ff00) ^
        (Te4[(temp >> 24)       ] & 0x000000ff) ^
        rcon[i];
      rk[ 9] = rk[ 1] ^ rk[ 8];
      rk[10] = rk[ 2] ^ rk[ 9];
      rk[11] = rk[ 3] ^ rk[10];
      if (++i == 7)
        return 14;
      temp = rk[11];
      rk[12] = rk[ 4] ^
        (Te4[(temp >> 24)       ] & 0xff000000) ^
        (Te4[(temp >> 16) & 0xff] & 0x00ff0000) ^
        (Te4[(temp >>  8) & 0xff] & 0x0000ff00) ^
        (Te4[(temp      ) & 0xff] & 0x000000ff);
      rk[13] = rk[ 5] ^ rk[12];
      rk[14] = rk[ 6] ^ rk[13];
      rk[15] = rk[ 7] ^ rk[14];
      rk += 8;
    }
  }
  return 0;
}

void rijndaelEncrypt(const uint32_t *rk, int nrounds,
                     const uint8_t plaintext[16], uint8_t ciphertext[16])
{
  uint32_t s0, s1, s2, s3, t0, t1, t2, t3;
  #ifndef FULL_UNROLL
    int r;
  #endif /* ?FULL_UNROLL */
  /*
   * map byte array block to cipher state
   * and add initial round key:
  */
  s0 = GETU32(plaintext     ) ^ rk[0];
  s1 = GETU32(plaintext +  4) ^ rk[1];
  s2 = GETU32(plaintext +  8) ^ rk[2];
  s3 = GETU32(plaintext + 12) ^ rk[3];
  #ifdef FULL_UNROLL
    /* round 1: */
    t0 = Te0[s0 >> 24] ^ Te1[(s1 >> 16) & 0xff] ^ Te2[(s2 >>  8) & 0xff] ^ Te3[s3 & 0xff] ^ rk[ 4];
    t1 = Te0[s1 >> 24] ^ Te1[(s2 >> 16) & 0xff] ^ Te2[(s3 >>  8) & 0xff] ^ Te3[s0 & 0xff] ^ rk[ 5];
    t2 = Te0[s2 >> 24] ^ Te1[(s3 >> 16) & 0xff] ^ Te2[(s0 >>  8) & 0xff] ^ Te3[s1 & 0xff] ^ rk[ 6];
    t3 = Te0[s3 >> 24] ^ Te1[(s0 >> 16) & 0xff] ^ Te2[(s1 >>  8) & 0xff] ^ Te3[s2 & 0xff] ^ rk[ 7];
    /* round 2: */
    s0 = Te0[t0 >> 24] ^ Te1[(t1 >> 16) & 0xff] ^ Te2[(t2 >>  8) & 0xff] ^ Te3[t3 & 0xff] ^ rk[ 8];
    s1 = Te0[t1 >> 24] ^ Te1[(t2 >> 16) & 0xff] ^ Te2[(t3 >>  8) & 0xff] ^ Te3[t0 & 0xff] ^ rk[ 9];
    s2 = Te0[t2 >> 24] ^ Te1[(t3 >> 16) & 0xff] ^ Te2[(t0 >>  8) & 0xff] ^ Te3[t1 & 0xff] ^ rk[10];
    s3 = Te0[t3 >> 24] ^ Te1[(t0 >> 16) & 0xff] ^ Te2[(t1 >>  8) & 0xff] ^ Te3[t2 & 0xff] ^ rk[11];
    /* round 3: */
    t0 = Te0[s0 >> 24] ^ Te1[(s1 >> 16) & 0xff] ^ Te2[(s2 >>  8) & 0xff] ^ Te3[s3 & 0xff] ^ rk[12];
    t1 = Te0[s1 >> 24] ^ Te1[(s2 >> 16) & 0xff] ^ Te2[(s3 >>  8) & 0xff] ^ Te3[s0 & 0xff] ^ rk[13];
    t2 = Te0[s2 >> 24] ^ Te1[(s3 >> 16) & 0xff] ^ Te2[(s0 >>  8) & 0xff] ^ Te3[s1 & 0xff] ^ rk[14];
    t3 = Te0[s3 >> 24] ^ Te1[(s0 >> 16) & 0xff] ^ Te2[(s1 >>  8) & 0xff] ^ Te3[s2 & 0xff] ^ rk[15];
    /* round 4: */
    s0 = Te0[t0 >> 24] ^ Te1[(t1 >> 16) & 0xff] ^ Te2[(t2 >>  8) & 0xff] ^ Te3[t3 & 0xff] ^ rk[16];
    s1 = Te0[t1 >> 24] ^ Te1[(t2 >> 16) & 0xff] ^ Te2[(t3 >>  8) & 0xff] ^ Te3[t0 & 0xff] ^ rk[17];
    s2 = Te0[t2 >> 24] ^ Te1[(t3 >> 16) & 0xff] ^ Te2[(t0 >>  8) & 0xff] ^ Te3[t1 & 0xff] ^ rk[18];
    s3 = Te0[t3 >> 24] ^ Te1[(t0 >> 16) & 0xff] ^ Te2[(t1 >>  8) & 0xff] ^ Te3[t2 & 0xff] ^ rk[19];
    /* round 5: */
    t0 = Te0[s0 >> 24] ^ Te1[(s1 >> 16) & 0xff] ^ Te2[(s2 >>  8) & 0xff] ^ Te3[s3 & 0xff] ^ rk[20];
    t1 = Te0[s1 >> 24] ^ Te1[(s2 >> 16) & 0xff] ^ Te2[(s3 >>  8) & 0xff] ^ Te3[s0 & 0xff] ^ rk[21];
    t2 = Te0[s2 >> 24] ^ Te1[(s3 >> 16) & 0xff] ^ Te2[(s0 >>  8) & 0xff] ^ Te3[s1 & 0xff] ^ rk[22];
    t3 = Te0[s3 >> 24] ^ Te1[(s0 >> 16) & 0xff] ^ Te2[(s1 >>  8) & 0xff] ^ Te3[s2 & 0xff] ^ rk[23];
    /* round 6: */
    s0 = Te0[t0 >> 24] ^ Te1[(t1 >> 16) & 0xff] ^ Te2[(t2 >>  8) & 0xff] ^ Te3[t3 & 0xff] ^ rk[24];
    s1 = Te0[t1 >> 24] ^ Te1[(t2 >> 16) & 0xff] ^ Te2[(t3 >>  8) & 0xff] ^ Te3[t0 & 0xff] ^ rk[25];
    s2 = Te0[t2 >> 24] ^ Te1[(t3 >> 16) & 0xff] ^ Te2[(t0 >>  8) & 0xff] ^ Te3[t1 & 0xff] ^ rk[26];
    s3 = Te0[t3 >> 24] ^ Te1[(t0 >> 16) & 0xff] ^ Te2[(t1 >>  8) & 0xff] ^ Te3[t2 & 0xff] ^ rk[27];
    /* round 7: */
    t0 = Te0[s0 >> 24] ^ Te1[(s1 >> 16) & 0xff] ^ Te2[(s2 >>  8) & 0xff] ^ Te3[s3 & 0xff] ^ rk[28];
    t1 = Te0[s1 >> 24] ^ Te1[(s2 >> 16) & 0xff] ^ Te2[(s3 >>  8) & 0xff] ^ Te3[s0 & 0xff] ^ rk[29];
    t2 = Te0[s2 >> 24] ^ Te1[(s3 >> 16) & 0xff] ^ Te2[(s0 >>  8) & 0xff] ^ Te3[s1 & 0xff] ^ rk[30];
    t3 = Te0[s3 >> 24] ^ Te1[(s0 >> 16) & 0xff] ^ Te2[(s1 >>  8) & 0xff] ^ Te3[s2 & 0xff] ^ rk[31];
    /* round 8: */
    s0 = Te0[t0 >> 24] ^ Te1[(t1 >> 16) & 0xff] ^ Te2[(t2 >>  8) & 0xff] ^ Te3[t3 & 0xff] ^ rk[32];
    s1 = Te0[t1 >> 24] ^ Te1[(t2 >> 16) & 0xff] ^ Te2[(t3 >>  8) & 0xff] ^ Te3[t0 & 0xff] ^ rk[33];
    s2 = Te0[t2 >> 24] ^ Te1[(t3 >> 16) & 0xff] ^ Te2[(t0 >>  8) & 0xff] ^ Te3[t1 & 0xff] ^ rk[34];
    s3 = Te0[t3 >> 24] ^ Te1[(t0 >> 16) & 0xff] ^ Te2[(t1 >>  8) & 0xff] ^ Te3[t2 & 0xff] ^ rk[35];
    /* round 9: */
    t0 = Te0[s0 >> 24] ^ Te1[(s1 >> 16) & 0xff] ^ Te2[(s2 >>  8) & 0xff] ^ Te3[s3 & 0xff] ^ rk[36];
    t1 = Te0[s1 >> 24] ^ Te1[(s2 >> 16) & 0xff] ^ Te2[(s3 >>  8) & 0xff] ^ Te3[s0 & 0xff] ^ rk[37];
    t2 = Te0[s2 >> 24] ^ Te1[(s3 >> 16) & 0xff] ^ Te2[(s0 >>  8) & 0xff] ^ Te3[s1 & 0xff] ^ rk[38];
    t3 = Te0[s3 >> 24] ^ Te1[(s0 >> 16) & 0xff] ^ Te2[(s1 >>  8) & 0xff] ^ Te3[s2 & 0xff] ^ rk[39];
    if (nrounds > 10)
    {
      /* round 10: */
      s0 = Te0[t0 >> 24] ^ Te1[(t1 >> 16) & 0xff] ^ Te2[(t2 >>  8) & 0xff] ^ Te3[t3 & 0xff] ^ rk[40];
      s1 = Te0[t1 >> 24] ^ Te1[(t2 >> 16) & 0xff] ^ Te2[(t3 >>  8) & 0xff] ^ Te3[t0 & 0xff] ^ rk[41];
      s2 = Te0[t2 >> 24] ^ Te1[(t3 >> 16) & 0xff] ^ Te2[(t0 >>  8) & 0xff] ^ Te3[t1 & 0xff] ^ rk[42];
      s3 = Te0[t3 >> 24] ^ Te1[(t0 >> 16) & 0xff] ^ Te2[(t1 >>  8) & 0xff] ^ Te3[t2 & 0xff] ^ rk[43];
      /* round 11: */
      t0 = Te0[s0 >> 24] ^ Te1[(s1 >> 16) & 0xff] ^ Te2[(s2 >>  8) & 0xff] ^ Te3[s3 & 0xff] ^ rk[44];
      t1 = Te0[s1 >> 24] ^ Te1[(s2 >> 16) & 0xff] ^ Te2[(s3 >>  8) & 0xff] ^ Te3[s0 & 0xff] ^ rk[45];
      t2 = Te0[s2 >> 24] ^ Te1[(s3 >> 16) & 0xff] ^ Te2[(s0 >>  8) & 0xff] ^ Te3[s1 & 0xff] ^ rk[46];
      t3 = Te0[s3 >> 24] ^ Te1[(s0 >> 16) & 0xff] ^ Te2[(s1 >>  8) & 0xff] ^ Te3[s2 & 0xff] ^ rk[47];
      if (nrounds > 12)
      {
        /* round 12: */
        s0 = Te0[t0 >> 24] ^ Te1[(t1 >> 16) & 0xff] ^ Te2[(t2 >>  8) & 0xff] ^ Te3[t3 & 0xff] ^ rk[48];
        s1 = Te0[t1 >> 24] ^ Te1[(t2 >> 16) & 0xff] ^ Te2[(t3 >>  8) & 0xff] ^ Te3[t0 & 0xff] ^ rk[49];
        s2 = Te0[t2 >> 24] ^ Te1[(t3 >> 16) & 0xff] ^ Te2[(t0 >>  8) & 0xff] ^ Te3[t1 & 0xff] ^ rk[50];
        s3 = Te0[t3 >> 24] ^ Te1[(t0 >> 16) & 0xff] ^ Te2[(t1 >>  8) & 0xff] ^ Te3[t2 & 0xff] ^ rk[51];
        /* round 13: */
        t0 = Te0[s0 >> 24] ^ Te1[(s1 >> 16) & 0xff] ^ Te2[(s2 >>  8) & 0xff] ^ Te3[s3 & 0xff] ^ rk[52];
        t1 = Te0[s1 >> 24] ^ Te1[(s2 >> 16) & 0xff] ^ Te2[(s3 >>  8) & 0xff] ^ Te3[s0 & 0xff] ^ rk[53];
        t2 = Te0[s2 >> 24] ^ Te1[(s3 >> 16) & 0xff] ^ Te2[(s0 >>  8) & 0xff] ^ Te3[s1 & 0xff] ^ rk[54];
        t3 = Te0[s3 >> 24] ^ Te1[(s0 >> 16) & 0xff] ^ Te2[(s1 >>  8) & 0xff] ^ Te3[s2 & 0xff] ^ rk[55];
      }
    }
    rk += nrounds << 2;
  #else  /* !FULL_UNROLL */
    /*
    * nrounds - 1 full rounds:
    */
    r = nrounds >> 1;
    for (;;)
    {
      t0 =
        Te0[(s0 >> 24)       ] ^
        Te1[(s1 >> 16) & 0xff] ^
        Te2[(s2 >>  8) & 0xff] ^
        Te3[(s3      ) & 0xff] ^
        rk[4];
      t1 =
        Te0[(s1 >> 24)       ] ^
        Te1[(s2 >> 16) & 0xff] ^
        Te2[(s3 >>  8) & 0xff] ^
        Te3[(s0      ) & 0xff] ^
        rk[5];
      t2 =
        Te0[(s2 >> 24)       ] ^
        Te1[(s3 >> 16) & 0xff] ^
        Te2[(s0 >>  8) & 0xff] ^
        Te3[(s1      ) & 0xff] ^
        rk[6];
      t3 =
        Te0[(s3 >> 24)       ] ^
        Te1[(s0 >> 16) & 0xff] ^
        Te2[(s1 >>  8) & 0xff] ^
        Te3[(s2      ) & 0xff] ^
        rk[7];
        rk += 8;
        if (--r == 0)
            break;
      s0 =
        Te0[(t0 >> 24)       ] ^
        Te1[(t1 >> 16) & 0xff] ^
        Te2[(t2 >>  8) & 0xff] ^
        Te3[(t3      ) & 0xff] ^
        rk[0];
      s1 =
        Te0[(t1 >> 24)       ] ^
        Te1[(t2 >> 16) & 0xff] ^
        Te2[(t3 >>  8) & 0xff] ^
        Te3[(t0      ) & 0xff] ^
        rk[1];
      s2 =
        Te0[(t2 >> 24)       ] ^
        Te1[(t3 >> 16) & 0xff] ^
        Te2[(t0 >>  8) & 0xff] ^
        Te3[(t1      ) & 0xff] ^
        rk[2];
      s3 =
        Te0[(t3 >> 24)       ] ^
        Te1[(t0 >> 16) & 0xff] ^
        Te2[(t1 >>  8) & 0xff] ^
        Te3[(t2      ) & 0xff] ^
        rk[3];
     }
 #endif /* ?FULL_UNROLL */
  /*
  * apply last round and
  * map cipher state to byte array block:
  */
  s0 =
    (Te4[(t0 >> 24)       ] & 0xff000000) ^
    (Te4[(t1 >> 16) & 0xff] & 0x00ff0000) ^
    (Te4[(t2 >>  8) & 0xff] & 0x0000ff00) ^
    (Te4[(t3      ) & 0xff] & 0x000000ff) ^
    rk[0];
  PUTU32(ciphertext     , s0);
  s1 =
    (Te4[(t1 >> 24)       ] & 0xff000000) ^
    (Te4[(t2 >> 16) & 0xff] & 0x00ff0000) ^
    (Te4[(t3 >>  8) & 0xff] & 0x0000ff00) ^
    (Te4[(t0      ) & 0xff] & 0x000000ff) ^
    rk[1];
  PUTU32(ciphertext +  4, s1);
  s2 =
    (Te4[(t2 >> 24)       ] & 0xff000000) ^
    (Te4[(t3 >> 16) & 0xff] & 0x00ff0000) ^
    (Te4[(t0 >>  8) & 0xff] & 0x0000ff00) ^
    (Te4[(t1      ) & 0xff] & 0x000000ff) ^
    rk[2];
  PUTU32(ciphertext +  8, s2);
  s3 =
    (Te4[(t3 >> 24)       ] & 0xff000000) ^
    (Te4[(t0 >> 16) & 0xff] & 0x00ff0000) ^
    (Te4[(t1 >>  8) & 0xff] & 0x0000ff00) ^
    (Te4[(t2      ) & 0xff] & 0x000000ff) ^
    rk[3];
  PUTU32(ciphertext + 12, s3);
}
