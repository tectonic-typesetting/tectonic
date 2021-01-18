/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxcrypt.h"
#include "dpx-dpxutil.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfobj.h"
#include "dpx-system.h"
#include "dpx-unicode.h"

/* Encryption support
 *
 * Supported: 40-128 bit RC4, 128 bit AES, 256 bit AES
 *
 * TODO: Convert password to PDFDocEncoding. SASLPrep stringpref for AESV3.
 */

static void pdf_enc_set_passwd (unsigned int bits, unsigned int perm,
                                const char *oplain, const char *uplain);

/* PDF-2.0 is not published yet. */
#define USE_ADOBE_EXTENSION 1

#ifdef USE_ADOBE_EXTENSION
#include "dpx-pdfdoc.h"
#endif

#include "dpx-pdfencrypt.h"

static struct pdf_sec {
   unsigned char key[32];
   int           key_size;

   unsigned char ID[16];
   unsigned char O[48], U[48];
   unsigned char OE[32], UE[32];
   int     V, R;
   int32_t P;

   struct {
     int use_aes;
     int encrypt_metadata;
     int need_adobe_extension;
   } setting;

   struct {
     uint64_t objnum;
     uint16_t gennum;
   } label;
} sec_data;

static const unsigned char padding_bytes[32] = {
  0x28, 0xbf, 0x4e, 0x5e, 0x4e, 0x75, 0x8a, 0x41,
  0x64, 0x00, 0x4e, 0x56, 0xff, 0xfa, 0x01, 0x08,
  0x2e, 0x2e, 0x00, 0xb6, 0xd0, 0x68, 0x3e, 0x80,
  0x2f, 0x0c, 0xa9, 0xfe, 0x64, 0x53, 0x69, 0x7a
};

int
pdf_init_encryption (struct pdf_enc_setting settings, const unsigned char *trailer_id)
{
  time_t          current_time;
  struct pdf_sec *p = &sec_data;

  if (trailer_id) {
    memcpy(p->ID, trailer_id, 16);
  } else {
    memset(p->ID, 0, 16);
  }
  current_time = dpx_util_get_unique_time_if_given();
  if (current_time == INVALID_EPOCH_VALUE)
    current_time = time(NULL);
  srand(current_time); /* For AES IV */
  p->setting.use_aes = settings.use_aes;
  p->setting.encrypt_metadata = settings.encrypt_metadata;
  p->setting.need_adobe_extension = 0;

  pdf_enc_set_passwd(settings.key_size, settings.permission,
                     settings.oplain, settings.uplain);
  return 0;
}

static void
passwd_padding (const char *src, unsigned char *dst)
{
  int len;

  len = MIN(32, strlen(src));

  memcpy(dst, src, len);
  memcpy(dst + len, padding_bytes, 32 - len);
}

static void
compute_owner_password (struct pdf_sec *p,
                        const char *opasswd, const char *upasswd)
{
  int  i, j;
  unsigned char padded[32];
  MD5_CONTEXT   md5;
  ARC4_CONTEXT  arc4;
  unsigned char hash[32];

  passwd_padding((strlen(opasswd) > 0 ? opasswd : upasswd), padded);

  MD5_init (&md5);
  MD5_write(&md5, padded, 32);
  MD5_final(hash, &md5);
  if (p->R >= 3) {
    for (i = 0; i < 50; i++) {
      /*
       * NOTE: We truncate each MD5 hash as in the following step.
       *       Otherwise Adobe Reader won't decrypt the PDF file.
       */
      MD5_init (&md5);
      MD5_write(&md5, hash, p->key_size);
      MD5_final(hash, &md5);
    }
  }
  ARC4_set_key(&arc4, p->key_size, hash);
  passwd_padding(upasswd, padded);
  {
    unsigned char tmp1[32], tmp2[32];
    unsigned char key[16];

    ARC4(&arc4, 32, padded, tmp1);
    if (p->R >= 3) {
      for (i = 1; i <= 19; i++) {
        memcpy(tmp2, tmp1, 32);
        for (j = 0; j < p->key_size; j++)
          key[j] = hash[j] ^ i;
        ARC4_set_key(&arc4, p->key_size, key);
        ARC4(&arc4, 32, tmp2, tmp1);
      }
    }
    memcpy(p->O, tmp1, 32);
  }
}

static void
compute_encryption_key (struct pdf_sec *p, const char *passwd)
{
  int  i;
  unsigned char hash[32], padded[32];
  MD5_CONTEXT   md5;

  passwd_padding(passwd, padded);
  MD5_init (&md5);
  MD5_write(&md5, padded, 32);
  MD5_write(&md5, p->O, 32);
  {
    unsigned char tmp[4];

    tmp[0] = (unsigned char)(p->P) & 0xFF;
    tmp[1] = (unsigned char)(p->P >> 8) & 0xFF;
    tmp[2] = (unsigned char)(p->P >> 16) & 0xFF;
    tmp[3] = (unsigned char)(p->P >> 24) & 0xFF;
    MD5_write(&md5, tmp, 4);
  }
  MD5_write(&md5, p->ID, 16);
  MD5_final(hash, &md5);

  if (p->R >= 3) {
    for (i = 0; i < 50; i++) {
      /*
       * NOTE: We truncate each MD5 hash as in the following step.
       *       Otherwise Adobe Reader won't decrypt the PDF file.
       */
      MD5_init (&md5);
      MD5_write(&md5, hash, p->key_size);
      MD5_final(hash, &md5);
    }
  }
  memcpy(p->key, hash, p->key_size);
}

static void
compute_user_password (struct pdf_sec *p, const char *uplain)
{
  int           i, j;
  ARC4_CONTEXT  arc4;
  MD5_CONTEXT   md5;
  unsigned char upasswd[32];

  compute_encryption_key(p, uplain);

  switch (p->R) {
  case 2:
    ARC4_set_key(&arc4, p->key_size, p->key);
    ARC4(&arc4, 32, padding_bytes, upasswd);
    break;
  case 3: case 4:
    {
      unsigned char hash[32];
      unsigned char tmp1[32], tmp2[32];

      MD5_init (&md5);
      MD5_write(&md5, padding_bytes, 32);

      MD5_write(&md5, p->ID, 16);
      MD5_final(hash, &md5);

      ARC4_set_key(&arc4, p->key_size, p->key);
      ARC4(&arc4, 16, hash, tmp1);

    for (i = 1; i <= 19; i++) {
        unsigned char key[16];

        memcpy(tmp2, tmp1, 16);
        for (j = 0; j < p->key_size; j++)
          key[j] = p->key[j] ^ i;
        ARC4_set_key(&arc4, p->key_size, key);
        ARC4(&arc4, 16, tmp2, tmp1);
      }
      memcpy(upasswd, tmp1, 32);
    }
    break;
  default:
    _tt_abort("Invalid revision number.");
  }

  memcpy(p->U, upasswd, 32);
}

/* Algorithm 2.B from ISO 32000-1 chapter 7 */
static void
compute_hash_V5 (unsigned char       *hash,
                 const char          *passwd,
                 const unsigned char *salt,
                 const unsigned char *user_key, int R /* revision */)
{
  unsigned char  K[64];
  size_t         K_len;
  int            nround;

  {
    SHA256_CONTEXT sha;

    SHA256_init (&sha);
    SHA256_write(&sha, (const unsigned char *)passwd, strlen(passwd));
    SHA256_write(&sha, salt, 8);
    if (user_key)
      SHA256_write(&sha, user_key, 48);
    SHA256_final(hash, &sha);
  }

  assert( R ==5 || R == 6 );

  if (R == 5)
    return;

  memcpy(K, hash, 32); K_len = 32;
  for (nround = 1; ; nround++) { /* Initial K count as nround 0. */
    unsigned char K1[256], *Kr, *E;
    size_t        K1_len, E_len;
    int           i, c, E_mod3 = 0;

    K1_len = strlen(passwd) + K_len + (user_key ? 48 : 0);
    assert(K1_len < 240);
    memcpy(K1, passwd, strlen(passwd));
    memcpy(K1 + strlen(passwd), K, K_len);
    if (user_key)
      memcpy(K1 + strlen(passwd) + K_len, user_key, 48);

    Kr = NEW(K1_len * 64, unsigned char);
    for (i = 0; i < 64; i++)
      memcpy(Kr + i * K1_len, K1, K1_len);
    AES_cbc_encrypt_tectonic(K, 16, K + 16, 0, Kr, K1_len * 64, &E, &E_len);
    free(Kr);

    for (i = 0; i < 16; i++)
      E_mod3 += E[i];
    E_mod3 %= 3;

    switch (E_mod3) {
    case 0:
      {
        SHA256_CONTEXT sha;

        SHA256_init (&sha);
        SHA256_write(&sha, E, E_len);
        SHA256_final(K, &sha);
        K_len = 32;
      }
      break;
    case 1:
      {
        SHA512_CONTEXT sha;

        SHA384_init (&sha);
        SHA384_write(&sha, E, E_len);
        SHA384_final(K, &sha);
        K_len = 48;
      }
      break;
    case 2:
      {
        SHA512_CONTEXT sha;

        SHA512_init (&sha);
        SHA512_write(&sha, E, E_len);
        SHA512_final(K, &sha);
        K_len = 64;
      }
      break;
    }
    c = (uint8_t) E[E_len - 1];
    free(E);
    if (nround >= 64 && c <= nround - 32)
        break;
  }
  memcpy(hash, K, 32);
}

static void
compute_owner_password_V5 (struct pdf_sec *p, const char *oplain)
{
  unsigned char  vsalt[8], ksalt[8], hash[32];
  unsigned char *OE, iv[AES_BLOCKSIZE];
  size_t         OE_len;
  int  i;

  for (i = 0; i < 8 ; i++) {
    vsalt[i] = rand() % 256;
    ksalt[i] = rand() % 256;
  }

  compute_hash_V5(hash, oplain, vsalt, p->U, p->R);
  memcpy(p->O,      hash,  32);
  memcpy(p->O + 32, vsalt,  8);
  memcpy(p->O + 40, ksalt,  8);

  compute_hash_V5(hash, oplain, ksalt, p->U, p->R);
  memset(iv, 0, AES_BLOCKSIZE);
  AES_cbc_encrypt_tectonic(hash, 32, iv, 0, p->key, p->key_size, &OE, &OE_len);
  memcpy(p->OE, OE, 32);
  free(OE);
}

static void
compute_user_password_V5 (struct pdf_sec *p, const char *uplain)
{
  unsigned char  vsalt[8], ksalt[8], hash[32];
  unsigned char *UE, iv[AES_BLOCKSIZE];
  size_t         UE_len;
  int  i;

  for (i = 0; i < 8 ; i++) {
    vsalt[i] = rand() % 256;
    ksalt[i] = rand() % 256;
  }

  compute_hash_V5(hash, uplain, vsalt, NULL, p->R);
  memcpy(p->U,      hash,  32);
  memcpy(p->U + 32, vsalt,  8);
  memcpy(p->U + 40, ksalt,  8);

  compute_hash_V5(hash, uplain, ksalt, NULL, p->R);
  memset(iv, 0, AES_BLOCKSIZE);
  AES_cbc_encrypt_tectonic(hash, 32, iv, 0, p->key, p->key_size, &UE, &UE_len);
  memcpy(p->UE, UE, 32);
  free(UE);
}


static void
check_version (struct pdf_sec *p, int version)
{
  if (p->V > 2 && version < 14) {
    dpx_warning("Current encryption setting requires PDF version >= 1.4.");
    p->V = 1;
    p->key_size = 5;
  } else if (p->V == 4 && version < 15) {
    dpx_warning("Current encryption setting requires PDF version >= 1.5.");
    p->V = 2;
  } else if (p->V ==5 && version < 17) {
    dpx_warning("Current encryption setting requires PDF version >= 1.7" \
         " (plus Adobe Extension Level 3).");
    p->V = 4;
    p->key_size = 16;
  }
  if (p->V == 5 && version < 20) {
    p->setting.need_adobe_extension = 1;
  }
}

/* Dummy routine for stringprep - NOT IMPLEMENTED YET
 *
 * Preprocessing of a user-provided password consists first of
 * normalizing its representation by applying the "SASLPrep" profile (RFC 4013)
 * of the "stringprep" algorithm (RFC 3454) to the supplied password using the
 * Normalize and BiDi options.
 */
typedef int Stringprep_profile_flags;
#define STRINGPREP_OK     0
#define STRINGPREP_ERROR -1
static int
stringprep_profile(const char *input, char **output, const char *profile,
                   Stringprep_profile_flags flags)
{
  const char *p, *endptr;

  p = input; endptr = p + strlen(p);
  while (p < endptr) {
    int32_t ucv = UC_UTF8_decode_char((const unsigned char **)&p,
                                      (const unsigned char *)endptr);
    if (!UC_is_valid(ucv))
      return STRINGPREP_ERROR;
  }

  *output = NEW(strlen(input) + 1, char);
  strcpy(*output, input);

  (void) profile;
  (void) flags;

  return STRINGPREP_OK;
}

static int
preproc_password (const char *passwd, char *outbuf, int V)
{
  char *saslpwd = NULL;
  int   error   = 0;

  memset(outbuf, 0, 128);
  switch (V) {
  case 1: case 2: case 3: case 4:
    {
      size_t i;
       /* Need to be converted to PDFDocEncoding - UNIMPLEMENTED */
      for (i = 0; i < strlen(passwd); i++) {
        if (passwd[i] < 0x20 || passwd[i] > 0x7e)
          dpx_warning("Non-ASCII-printable character found in password.");
      }
      memcpy(outbuf, passwd, MIN(127, strlen(passwd)));
    }
    break;
  case 5:
    /* This is a dummy routine - not actually stringprep password... */
    if (stringprep_profile(passwd, &saslpwd,
                           "SASLprep", 0) != STRINGPREP_OK)
       return -1;
    else if (saslpwd) {
      memcpy(outbuf, saslpwd, MIN(127, strlen(saslpwd)));
      free(saslpwd);
    }
    break;
  default:
    error = -1;
    break;
  }

  return error;
}

static void
pdf_enc_set_passwd (unsigned int bits, unsigned int perm,
                    const char *oplain, const char *uplain)
{
  struct pdf_sec *p = &sec_data;
  char            opasswd[128], upasswd[128];
  int             version;
  char            empty_passwd[1] = "\0";

  version = pdf_get_version();

  p->key_size = (int) (bits / 8);
  if (p->key_size == 5) /* 40bit */
    p->V = 1;
  else if (p->key_size <= 16) {
    p->V = p->setting.use_aes ? 4 : 2;
  } else if (p->key_size == 32) {
    p->V = 5;
  } else {
    dpx_warning("Key length %d unsupported.", bits);
    p->key_size = 5;
    p->V = 2;
  }
  check_version(p, version);

  p->P = (int32_t) (perm | 0xC0U);
  /* Bit position 10 shall be always set to 1 for PDF >= 2.0. */
  if (version >= 20)
    p->P |= (1 << 9);
  switch (p->V) {
  case 1:
    p->R = (p->P < 0x100L) ? 2 : 3;
    break;
  case 2: case 3:
    p->R = 3;
    break;
  case 4:
    p->R = 4;
    break;
  case 5:
#if USE_ADOBE_EXTENSION
    p->R = 6;
#else
    dpx_warning("Encryption V 5 unsupported.");
    p->R = 4; p->V = 4;
#endif
    break;
  default:
    p->R = 3;
    break;
  }

  memset(opasswd, 0, 128);
  memset(upasswd, 0, 128);
  /* Password must be preprocessed. */
  if (oplain) {
    if (preproc_password(oplain, opasswd, p->V) < 0)
      dpx_warning("Invaid UTF-8 string for password.");
  } else {
    preproc_password(empty_passwd, opasswd, p->V);
  }

  if (uplain) {
    if (preproc_password(uplain, upasswd, p->V) < 0)
      dpx_warning("Invalid UTF-8 string for passowrd.");
  } else {
    preproc_password(empty_passwd, upasswd, p->V);    
  }

  if (p->R >= 3)
    p->P |= 0xFFFFF000U;

  if (p->V < 5) {
    compute_owner_password(p, opasswd, upasswd);
    compute_user_password (p, upasswd);
  } else if (p->V == 5) {
    int i;

    for (i = 0; i < 32; i++)
      p->key[i] = rand() % 256;
    p->key_size = 32;
    /* Order is important here */
    compute_user_password_V5 (p, upasswd);
    compute_owner_password_V5(p, opasswd); /* uses p->U */
  }
}

static void
calculate_key (struct pdf_sec *p, unsigned char *key)
{
  int           len = p->key_size + 5;
  unsigned char tmp[25];
  MD5_CONTEXT   md5;

  memcpy(tmp, p->key, p->key_size);
  tmp[p->key_size  ] = (unsigned char) p->label.objnum        & 0xFF;
  tmp[p->key_size+1] = (unsigned char)(p->label.objnum >>  8) & 0xFF;
  tmp[p->key_size+2] = (unsigned char)(p->label.objnum >> 16) & 0xFF;
  tmp[p->key_size+3] = (unsigned char)(p->label.gennum)       & 0xFF;
  tmp[p->key_size+4] = (unsigned char)(p->label.gennum >>  8) & 0xFF;
  if (p->V >= 4) {
    tmp[p->key_size + 5] = 0x73;
    tmp[p->key_size + 6] = 0x41;
    tmp[p->key_size + 7] = 0x6c;
    tmp[p->key_size + 8] = 0x54;
    len += 4;
  }
  MD5_init (&md5);
  MD5_write(&md5, tmp, len);
  MD5_final(key, &md5);
}

void
pdf_encrypt_data (const unsigned char *plain, size_t plain_len,
                  unsigned char **cipher, size_t *cipher_len)
{
  struct pdf_sec *p = &sec_data;
  unsigned char   key[32];

  switch (p->V) {
  case 1: case 2:
    calculate_key(p, key);
    {
      ARC4_CONTEXT arc4;

      *cipher_len = plain_len;
      *cipher     = NEW(*cipher_len, unsigned char);
      ARC4_set_key(&arc4, MIN(16, p->key_size + 5), key);
      ARC4(&arc4, plain_len, plain, *cipher);
    }
    break;
  case 4:
    calculate_key(p, key);
    AES_cbc_encrypt_tectonic(key, MIN(16, p->key_size + 5), NULL, 1,
                             plain, plain_len, cipher, cipher_len);
    break;
  case 5:
    AES_cbc_encrypt_tectonic(p->key, p->key_size, NULL, 1,
                             plain, plain_len, cipher, cipher_len);
    break;
  default:
    _tt_abort("pdfencrypt: Unexpected V value: %d", p->V);
    break;
  }
}

pdf_obj *
pdf_encrypt_obj (void)
{
  struct pdf_sec *p = &sec_data;
  pdf_obj *doc_encrypt;

  doc_encrypt = pdf_new_dict();

  pdf_add_dict(doc_encrypt,  pdf_new_name("Filter"), pdf_new_name("Standard"));
  pdf_add_dict(doc_encrypt,  pdf_new_name("V"),      pdf_new_number(p->V));
#if 0
  /* PDF reference describes it as:
   *
   *   (Optional; PDF 1.4; only if V is 2 or 3)
   *
   * but Acrobat *requires* this even for V 5!
   */
  if (p->V > 1 && p->V < 4)
#endif
    pdf_add_dict(doc_encrypt,
                 pdf_new_name("Length"), pdf_new_number(p->key_size * 8));
  if (p->V >= 4) {
    pdf_obj *CF, *StdCF;
    CF    = pdf_new_dict();
    StdCF = pdf_new_dict();
    pdf_add_dict(StdCF, pdf_new_name("CFM"),
                 pdf_new_name( (p->V == 4) ? "AESV2" : "AESV3" ));
    pdf_add_dict(StdCF, pdf_new_name("AuthEvent"), pdf_new_name("DocOpen"));
    pdf_add_dict(StdCF, pdf_new_name("Length"),    pdf_new_number(p->key_size));
    pdf_add_dict(CF, pdf_new_name("StdCF"), StdCF);
    pdf_add_dict(doc_encrypt, pdf_new_name("CF"), CF);
    pdf_add_dict(doc_encrypt, pdf_new_name("StmF"), pdf_new_name("StdCF"));
    pdf_add_dict(doc_encrypt, pdf_new_name("StrF"), pdf_new_name("StdCF"));
#if 0
    /* NOT SUPPORTED YET */
    if (!p->setting.encrypt_metadata)
      pdf_add_dict(doc_encrypt,
                   pdf_new_name("EncryptMetadata"), pdf_new_boolean(false));
#endif
  }
  pdf_add_dict(doc_encrypt, pdf_new_name("R"), pdf_new_number(p->R));
  if (p->V < 5) {
    pdf_add_dict(doc_encrypt, pdf_new_name("O"), pdf_new_string(p->O, 32));
    pdf_add_dict(doc_encrypt, pdf_new_name("U"), pdf_new_string(p->U, 32));
  } else if (p->V == 5) {
    pdf_add_dict(doc_encrypt, pdf_new_name("O"), pdf_new_string(p->O, 48));
    pdf_add_dict(doc_encrypt, pdf_new_name("U"), pdf_new_string(p->U, 48));
  }
  pdf_add_dict(doc_encrypt, pdf_new_name("P"), pdf_new_number(p->P));

  if (p->V == 5) {
    unsigned char perms[16], *cipher = NULL;
    size_t        cipher_len = 0;

    pdf_add_dict(doc_encrypt, pdf_new_name("OE"), pdf_new_string(p->OE, 32));
    pdf_add_dict(doc_encrypt, pdf_new_name("UE"), pdf_new_string(p->UE, 32));
    perms[0] =  p->P        & 0xff;
    perms[1] = (p->P >>  8) & 0xff;
    perms[2] = (p->P >> 16) & 0xff;
    perms[3] = (p->P >> 24) & 0xff;
    perms[4] = 0xff;
    perms[5] = 0xff;
    perms[6] = 0xff;
    perms[7] = 0xff;
    perms[8] = p->setting.encrypt_metadata ? 'T' : 'F';
    perms[9]  = 'a';
    perms[10] = 'd';
    perms[11] = 'b';
    perms[12] = 0;
    perms[13] = 0;
    perms[14] = 0;
    perms[15] = 0;
    AES_ecb_encrypt(p->key, p->key_size, perms, 16, &cipher, &cipher_len);
    pdf_add_dict(doc_encrypt,
                 pdf_new_name("Perms"), pdf_new_string(cipher, cipher_len));
    free(cipher);
  }

#ifdef USE_ADOBE_EXTENSION
  if (p->R > 5 && p->setting.need_adobe_extension != 0) {
    pdf_obj *catalog = pdf_doc_catalog();
    pdf_obj *ext  = pdf_new_dict();
    pdf_obj *adbe = pdf_new_dict();

    pdf_add_dict(adbe, pdf_new_name("BaseVersion"), pdf_new_name("1.7"));
    pdf_add_dict(adbe, pdf_new_name("ExtensionLevel"),
                       pdf_new_number(p->R == 5 ? 3 : 8));
    pdf_add_dict(ext, pdf_new_name("ADBE"), adbe);
    pdf_add_dict(catalog, pdf_new_name("Extensions"), ext);
  }
#endif

  return doc_encrypt;
}

void pdf_enc_set_label (unsigned label)
{
  struct pdf_sec *p = &sec_data;

  p->label.objnum = label;
}

void pdf_enc_set_generation (unsigned generation)
{
  struct pdf_sec *p = &sec_data;

  p->label.gennum = generation;
}
