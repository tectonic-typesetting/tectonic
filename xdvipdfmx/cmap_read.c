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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"

#include "dpxutil.h"
#include "pst.h"

#include "cmap_p.h"
#include "cmap.h"

#include "cmap_read.h"

static int __verbose = 0;

#define CMAP_PARSE_DEBUG_STR "CMap_parse:"
#define CMAP_PARSE_DEBUG     3

#ifdef HAVE_DPXFILE
#include "dpxfile.h"
#else
#include "mfileio.h"

typedef struct {
  unsigned char *cursor;
  unsigned char *endptr;

  unsigned char *buf;
  size_t  max;
  FILE   *fp;
  size_t  unread;
} ifreader;

static ifreader *ifreader_create  (FILE *fp, size_t remain, size_t bufsize);
static size_t    ifreader_read    (ifreader *reader, size_t size);
static void      ifreader_destroy (ifreader *reader);

static ifreader *
ifreader_create (FILE *fp, size_t size, size_t bufsize)
{
  ifreader *reader;

  reader = NEW(1, ifreader);
  reader->buf = NEW(bufsize+1, unsigned char);
  reader->max = bufsize;
  reader->fp  = fp;
  reader->unread = size;

  reader->cursor = reader->endptr = reader->buf;
  *reader->endptr = 0;

  return reader;
}

static void
ifreader_destroy (ifreader *reader)
{
  ASSERT(reader);
  if (reader->buf)
    RELEASE(reader->buf);
  RELEASE(reader);
}


static size_t
ifreader_read (ifreader *reader, size_t size)
{
  size_t bytesread = 0, bytesrem = 0;

  ASSERT(reader);
  bytesrem = (size_t) reader->endptr - (size_t) reader->cursor;
  if (size > reader->max) {
    if (__verbose)
      MESG("\nExtending buffer (%ld bytes)...\n", size);
    reader->buf = RENEW(reader->buf, size+1, unsigned char);
    reader->max = size;
  }
  if (reader->unread > 0 && bytesrem < size) {
    bytesread = MIN(reader->max - bytesrem, reader->unread);
    memmove(reader->buf, reader->cursor, bytesrem);
    reader->cursor = reader->buf;
    reader->endptr = reader->buf + bytesrem;
    if (fread(reader->endptr, 1, bytesread, reader->fp) != bytesread)
      ERROR("Reading file failed.");
    reader->endptr += bytesread;
    reader->unread -= bytesread;
    if (__verbose)
      MESG("Reading more %ld bytes (%ld bytes remains in buffer)...\n", bytesread, bytesrem);
  }

  *reader->endptr = 0;

  return bytesread + bytesrem;
}
#define ifreader_need(f,s) ifreader_read((f),(s))

#endif /* HAVE_DPXFILE */

static int check_next_token  (ifreader *input, const char *key);
static int get_coderange     (ifreader *input, unsigned char *codeLo, unsigned char *codeHi, int *dim, int maxlen);

static int handle_codearray  (CMap *cmap, ifreader *input, unsigned char *codeLo, int dim, int count);
static int do_codespacerange (CMap *cmap, ifreader *input, int count);
static int do_notdefrange    (CMap *cmap, ifreader *input, int count);
static int do_bfrange        (CMap *cmap, ifreader *input, int count);
static int do_cidrange       (CMap *cmap, ifreader *input, int count);
static int do_notdefchar     (CMap *cmap, ifreader *input, int count);
static int do_bfchar         (CMap *cmap, ifreader *input, int count);
static int do_cidchar        (CMap *cmap, ifreader *input, int count);

#define TOKEN_LEN_MAX 127

static int
check_next_token (ifreader *input, const char *key)
{
  int      cmp;
  pst_obj *token;
  char    *str;

  if (ifreader_need(input, strlen(key)) == 0)
    return -1;
  if ((token = pst_get_token(&(input->cursor), input->endptr)) == NULL)
    return -1;

  str = (char *) pst_getSV(token);
  cmp = strcmp(str, key) ? -1 : 0;
  if (str)
    RELEASE(str);
  pst_release_obj(token);

  return cmp;
}

static int
get_coderange (ifreader *input,
	       unsigned char *codeLo, unsigned char *codeHi, int *dim, int maxlen)
{
  pst_obj *tok1, *tok2;
  int      dim1, dim2;

  if ((tok1 = pst_get_token(&(input->cursor), input->endptr)) == NULL)
    return -1;
  if ((tok2 = pst_get_token(&(input->cursor), input->endptr)) == NULL) {
    pst_release_obj(tok1);
    return -1;
  }

  if (!PST_STRINGTYPE(tok1) || !PST_STRINGTYPE(tok2)) {
    pst_release_obj(tok1);
    pst_release_obj(tok2);
    return -1;
  }

  dim1 = pst_length_of(tok1);
  dim2 = pst_length_of(tok2);
  if (dim1 != dim2 || dim1 > maxlen) {
    pst_release_obj(tok1);
    pst_release_obj(tok2);
    return -1;
  }

  memcpy(codeLo, pst_data_ptr(tok1), dim1);
  memcpy(codeHi, pst_data_ptr(tok2), dim2);
  pst_release_obj(tok1);
  pst_release_obj(tok2);

  *dim = dim1;
  return 0;
}

static int
do_codespacerange (CMap *cmap, ifreader *input, int count)
{
  unsigned char codeLo[TOKEN_LEN_MAX], codeHi[TOKEN_LEN_MAX];
  int dim;

  while (count-- > 0) { 
    if (get_coderange(input, codeLo, codeHi, &dim, TOKEN_LEN_MAX) < 0)
      return -1;
    CMap_add_codespacerange(cmap, codeLo, codeHi, dim);
  }

  return check_next_token(input, "endcodespacerange");
}

/*
 * bfrange
 *  <codeLo> <codeHi> [destCode1 destCode2 ...]
 */
static int
handle_codearray (CMap *cmap, ifreader *input, unsigned char *codeLo, int dim, int count)
{
  pst_obj *tok = NULL;

  if (dim < 1)
    ERROR("Invalid code range.");
  while (count-- > 0) {
    if ((tok = pst_get_token(&(input->cursor), input->endptr)) == NULL)
      return -1;
    else if (PST_STRINGTYPE(tok)) {
      CMap_add_bfchar(cmap, codeLo, dim, (unsigned char *) pst_data_ptr(tok), pst_length_of(tok));
    } else if (PST_MARKTYPE(tok) || !PST_NAMETYPE(tok))
      ERROR("%s: Invalid CMap mapping record.", CMAP_PARSE_DEBUG_STR);
    else
      ERROR("%s: Mapping to charName not supported.", CMAP_PARSE_DEBUG_STR);
    pst_release_obj(tok);
    codeLo[dim-1] += 1;
  }

  return check_next_token(input, "]");
}

static int
do_notdefrange (CMap *cmap, ifreader *input, int count)
{
  pst_obj *tok;
  unsigned char   codeLo[TOKEN_LEN_MAX], codeHi[TOKEN_LEN_MAX];
  int      dstCID;
  int      dim;

  while (count-- > 0) { 
    if (ifreader_need(input, TOKEN_LEN_MAX*3) == 0)
      return -1;
    if (get_coderange(input, codeLo, codeHi, &dim, TOKEN_LEN_MAX) < 0 ||
	(tok = pst_get_token(&(input->cursor), input->endptr)) == NULL)
      return -1;
    if (PST_INTEGERTYPE(tok)) {
      dstCID = pst_getIV(tok);
      if (dstCID >= 0 && dstCID <= CID_MAX)
	CMap_add_notdefrange(cmap, codeLo, codeHi, dim, (CID) dstCID);
    } else
      WARN("%s: Invalid CMap mapping record. (ignored)", CMAP_PARSE_DEBUG_STR);
    pst_release_obj(tok);
  }

  return check_next_token(input, "endnotdefrange");
}

static int
do_bfrange (CMap *cmap, ifreader *input, int count)
{
  pst_obj *tok; 
  unsigned char   codeLo[TOKEN_LEN_MAX], codeHi[TOKEN_LEN_MAX];
  int      srcdim;

  while (count-- > 0) { 
    if (ifreader_need(input, TOKEN_LEN_MAX*3) == 0)
      return -1;
    if (get_coderange(input, codeLo, codeHi, &srcdim, TOKEN_LEN_MAX) < 0    ||
	(tok = pst_get_token(&(input->cursor), input->endptr)) == NULL)
      return -1;
    if (PST_STRINGTYPE(tok)) {
      CMap_add_bfrange(cmap, codeLo, codeHi, srcdim,
		       (unsigned char *) pst_data_ptr(tok), pst_length_of(tok));
    } else if (PST_MARKTYPE(tok)) {
      if (handle_codearray(cmap, input, codeLo, srcdim,
			   codeHi[srcdim-1] - codeLo[srcdim-1] + 1) < 0) {
	pst_release_obj(tok);
	return -1;
      }
    } else
      WARN("%s: Invalid CMap mapping record. (ignored)", CMAP_PARSE_DEBUG_STR);
    pst_release_obj(tok);
  }
  
  return check_next_token(input, "endbfrange");
}

static int
do_cidrange (CMap *cmap, ifreader *input, int count)
{
  pst_obj *tok;
  unsigned char   codeLo[TOKEN_LEN_MAX], codeHi[TOKEN_LEN_MAX];
  int      dstCID;
  int      dim;

  while (count-- > 0) { 
    if (ifreader_need(input, TOKEN_LEN_MAX*3) == 0)
      return -1;
    if (get_coderange(input, codeLo, codeHi, &dim, TOKEN_LEN_MAX) < 0 ||
	(tok = pst_get_token(&(input->cursor), input->endptr)) == NULL)
      return -1;
    if (PST_INTEGERTYPE(tok)) {
      dstCID = pst_getIV(tok);
      if (dstCID >= 0 && dstCID <= CID_MAX)
	CMap_add_cidrange(cmap, codeLo, codeHi, dim, (CID) dstCID);
    } else
      WARN("%s: Invalid CMap mapping record. (ignored)", CMAP_PARSE_DEBUG_STR);
    pst_release_obj(tok);
  }

  return check_next_token(input, "endcidrange");
}

static int
do_notdefchar (CMap *cmap, ifreader *input, int count)
{
  pst_obj *tok1, *tok2;
  int      dstCID;

  while (count-- > 0) { 
    if (ifreader_need(input, TOKEN_LEN_MAX*2) == 0)
      return -1;
    if ((tok1 = pst_get_token(&(input->cursor), input->endptr)) == NULL)
      return -1;
    if ((tok2 = pst_get_token(&(input->cursor), input->endptr)) == NULL) {
      pst_release_obj(tok1);
      return -1;
    }
    if (PST_STRINGTYPE(tok1) && PST_INTEGERTYPE(tok2)) {
      dstCID = pst_getIV(tok2);
      if (dstCID >= 0 && dstCID <= CID_MAX)
	CMap_add_notdefchar(cmap, pst_data_ptr(tok1), pst_length_of(tok1), (CID) dstCID);
    } else
      WARN("%s: Invalid CMap mapping record. (ignored)", CMAP_PARSE_DEBUG_STR);
    pst_release_obj(tok1);
    pst_release_obj(tok2);
  }

  return check_next_token(input, "endnotdefchar");
}

static int
do_bfchar (CMap *cmap, ifreader *input, int count)
{
  pst_obj *tok1, *tok2;

  while (count-- > 0) { 
    if (ifreader_need(input, TOKEN_LEN_MAX*2) == 0)
      return -1;
    if ((tok1 = pst_get_token(&(input->cursor), input->endptr)) == NULL)
      return -1;
    if ((tok2 = pst_get_token(&(input->cursor), input->endptr)) == NULL) {
      pst_release_obj(tok1);
      return -1;
    }
    /* We only support single CID font as descendant font, charName should not come here. */
    if (PST_STRINGTYPE(tok1) && PST_STRINGTYPE(tok2)) {
      CMap_add_bfchar(cmap,
		      (unsigned char *) pst_data_ptr(tok1), pst_length_of(tok1),
		      (unsigned char *) pst_data_ptr(tok2), pst_length_of(tok2));
    } else if (PST_NAMETYPE(tok2))
      ERROR("%s: Mapping to charName not supported.", CMAP_PARSE_DEBUG_STR);
    else
      WARN("%s: Invalid CMap mapping record. (ignored)", CMAP_PARSE_DEBUG_STR);
    pst_release_obj(tok1);
    pst_release_obj(tok2);
  }

  return check_next_token(input, "endbfchar");
}

static int
do_cidchar (CMap *cmap, ifreader *input, int count)
{
  pst_obj *tok1, *tok2;
  int      dstCID;

  while (count-- > 0) { 
    if (ifreader_need(input, TOKEN_LEN_MAX*2) == 0)
      return -1;
    if ((tok1 = pst_get_token(&(input->cursor), input->endptr)) == NULL)
      return -1;
    if ((tok2 = pst_get_token(&(input->cursor), input->endptr)) == NULL) {
      pst_release_obj(tok1);
      return -1;
    }
    if (PST_STRINGTYPE(tok1) && PST_INTEGERTYPE(tok2)) {
      dstCID = pst_getIV(tok2);
      if (dstCID >= 0 && dstCID <= CID_MAX)
	CMap_add_cidchar(cmap, pst_data_ptr(tok1), pst_length_of(tok1), (CID) dstCID);
    } else
      WARN("%s: Invalid CMap mapping record. (ignored)", CMAP_PARSE_DEBUG_STR);
    pst_release_obj(tok1);
    pst_release_obj(tok2);
  }

  return check_next_token(input, "endcidchar");
}


#define MATCH_NAME(t,n) (PST_NAMETYPE((t))    && !memcmp(pst_data_ptr((t)),(n),strlen((n))))
#define MATCH_OP(t,n)   (PST_UNKNOWNTYPE((t)) && !memcmp(pst_data_ptr((t)),(n),strlen((n))))

static int
do_cidsysteminfo (CMap *cmap, ifreader *input)
{
  pst_obj   *tok1, *tok2;
  CIDSysInfo csi = {NULL, NULL, -1};
  int        simpledict = 0;
  int        error = 0;

  ifreader_need(input, TOKEN_LEN_MAX*2);
  /*
   * Assuming /CIDSystemInfo 3 dict dup begin .... end def
   * or /CIDSystemInfo << ... >> def
   */
  while ((tok1 = pst_get_token(&(input->cursor), input->endptr)) != NULL) {
    if (PST_MARKTYPE(tok1)) {
      simpledict = 1;
      pst_release_obj(tok1);
      break;
    } else if (MATCH_OP(tok1, "begin")) {
      simpledict = 0;
      pst_release_obj(tok1);
      break;
    } else {
      pst_release_obj(tok1);
      /* continue */
    }
  }
  tok1 = tok2 = NULL;
  while (!error &&
         (tok1 = pst_get_token(&(input->cursor), input->endptr)) != NULL) {
    if (MATCH_OP(tok1, ">>") && simpledict) {
      pst_release_obj(tok1);
      break;
    } else if (MATCH_OP(tok1, "end") && !simpledict) {
      pst_release_obj(tok1);
      break;
    } else if (MATCH_NAME(tok1, "Registry") &&
               (tok2 = pst_get_token(&(input->cursor), input->endptr)) != NULL) {
      if (!PST_STRINGTYPE(tok2))
        error = -1;
      else if (!simpledict &&
                check_next_token(input, "def"))
        error = -1;
      if (!error)
        csi.registry = (char *) pst_getSV(tok2);
    } else if (MATCH_NAME(tok1, "Ordering") &&
               (tok2 = pst_get_token(&(input->cursor), input->endptr)) != NULL) {
      if (!PST_STRINGTYPE(tok2))
        error = -1;
      else if (!simpledict &&
                check_next_token(input, "def"))
        error = -1;
      if (!error)
        csi.ordering = (char *) pst_getSV(tok2);
    } else if (MATCH_NAME(tok1, "Supplement") &&
               (tok2 = pst_get_token(&(input->cursor), input->endptr)) != NULL) {
      if (!PST_INTEGERTYPE(tok2))
        error = -1;
      else if (!simpledict &&
                check_next_token(input, "def"))
        error = -1;
      if (!error)
        csi.supplement = pst_getIV(tok2);
    }
    if (tok2)
      pst_release_obj(tok2);
    if (tok1)
      pst_release_obj(tok1);
    tok1 = tok2 = NULL;
  }
  if (!error &&
       check_next_token(input, "def"))
    error = -1;

  if (!error &&
       csi.registry && csi.ordering &&
       csi.supplement >= 0) {
    CMap_set_CIDSysInfo(cmap, &csi);
  }

  if (csi.registry)
    RELEASE(csi.registry);
  if (csi.ordering)
    RELEASE(csi.ordering);

  return  error;
}

#define INPUT_BUF_SIZE 4096
#define CMAP_SIG_MAX   64
int
CMap_parse_check_sig (FILE *fp)
{
  int  result = -1;
  char sig[CMAP_SIG_MAX+1];

  if (!fp)
    return -1;

  rewind(fp);
  if (fread(sig, sizeof(char), CMAP_SIG_MAX, fp) != CMAP_SIG_MAX)
    result = -1;
  else {
    sig[CMAP_SIG_MAX] = 0;
    if (strncmp(sig, "%!PS", 4))
      result = -1;
    else if (strstr(sig+4, "Resource-CMap"))
      result = 0;
  }
  rewind(fp);

  return result;
}

int
CMap_parse (CMap *cmap, FILE *fp)
{
  pst_obj  *tok1, *tok2;
  ifreader *input;
  int       status = 0, tmpint = -1;

  ASSERT(cmap && fp);

  input = ifreader_create(fp, file_size(fp), INPUT_BUF_SIZE-1);

  while (status >= 0) {
    tok1 = tok2 = NULL;
    ifreader_read(input, INPUT_BUF_SIZE/2);
    tok1 = pst_get_token(&(input->cursor), input->endptr);
    if (tok1 == NULL)
      break;
    else if (MATCH_NAME(tok1, "CMapName")) {
      if ((tok2 = pst_get_token(&(input->cursor), input->endptr)) == NULL ||
	  !(PST_NAMETYPE(tok2) || PST_STRINGTYPE(tok2)) ||
	  check_next_token(input, "def") < 0)
	status = -1;
      else
	CMap_set_name(cmap, pst_data_ptr(tok2));
    } else if (MATCH_NAME(tok1, "CMapType")) {
      if ((tok2 = pst_get_token(&(input->cursor), input->endptr)) == NULL ||
	  !PST_INTEGERTYPE(tok2) ||
	  check_next_token(input, "def") < 0)
	status = -1;
      else
	CMap_set_type(cmap, pst_getIV(tok2));
    } else if (MATCH_NAME(tok1, "WMode")) {
      if ((tok2 = pst_get_token(&(input->cursor), input->endptr)) == NULL ||
	  !PST_INTEGERTYPE(tok2) ||
	  check_next_token(input, "def") < 0)
	status = -1;
      else
	CMap_set_wmode(cmap, pst_getIV(tok2));
    } else if (MATCH_NAME(tok1, "CIDSystemInfo")) {
      status = do_cidsysteminfo(cmap, input);
    } else if (MATCH_NAME(tok1, "Version") ||
	       MATCH_NAME(tok1, "UIDOffset") ||
	       MATCH_NAME(tok1, "XUID")) {
	/* Ignore */
    } else if (PST_NAMETYPE(tok1)) {
      /* Possibly usecmap comes next */
      if ((tok2 = pst_get_token(&(input->cursor), input->endptr)) != NULL &&
	  MATCH_OP(tok2, "usecmap")) {
	int   id;
	CMap *ucmap;
	id = CMap_cache_find(pst_data_ptr(tok1));
	if (id < 0)
	  status = -1;
	else {
	  ucmap = CMap_cache_get(id);
	  CMap_set_usecmap(cmap, ucmap);
	}
      }
    } else if (MATCH_OP(tok1, "begincodespacerange")) {
      status = do_codespacerange(cmap, input, tmpint);
    } else if (MATCH_OP(tok1, "beginnotdefrange")) {
      status = do_notdefrange(cmap, input, tmpint);
    } else if (MATCH_OP(tok1, "beginnotdefchar")) {
      status = do_notdefchar(cmap, input, tmpint);
    } else if (MATCH_OP(tok1, "beginbfrange")) {
      status = do_bfrange(cmap, input, tmpint);
    } else if (MATCH_OP(tok1, "beginbfchar")) {
      status =  do_bfchar(cmap, input, tmpint);
    } else if (MATCH_OP(tok1, "begincidrange")) {
      status = do_cidrange(cmap, input, tmpint);
    } else if (MATCH_OP(tok1, "begincidchar")) {
      status =  do_cidchar(cmap, input, tmpint);
    } else if (PST_INTEGERTYPE(tok1)) {
      tmpint = pst_getIV(tok1);
    } /* else Simply ignore */
    if (tok1)
      pst_release_obj(tok1);
    if (tok2)
      pst_release_obj(tok2);
  }

  ifreader_destroy(input);

  return (status < 0) ? -1 : CMap_is_valid(cmap);
}
