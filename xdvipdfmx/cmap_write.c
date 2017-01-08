/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

/*
 * References:
 *
 *  PostScript Language Reference Manual, 3rd. ed. (Adobe Systems Inc.)
 *    5.11.4 CMap Dictionaries
 *    5.11.5 FMapType 9 Composite Fonts
 *  Building CMap Files for CID-Keyed Fonts, Adobe Technical Note #5099
 *  CID-Keyed Font Technology Overview, Adobe Technical Note #5092
 *  Adobe CMap and CIDFont Files Specification, Adobe Technical Specification #5014
 *
 *  Undefined Character Handling:
 *    PLRM 3rd. ed., sec. 5.11.5., "Handling Undefined Characters"
 *
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"
#include "dpxutil.h"

#include "pdfobj.h"
#include "pdfresource.h"

#include "cmap_p.h"
#include "cmap.h"

#include "cmap_write.h"

struct sbuf {
  char *buf;
  char *curptr;
  char *limptr;
};

static int write_map (mapDef *mtab, int count,
		      unsigned char *codestr, int depth,
		      struct sbuf *wbuf, pdf_obj *stream);
#if 0
/* Not completed yet */

/* used_slot...
 * This is for collecting only used CIDs.
 * add_inverse_map never supports code-to-code of
 * cid-to-code mapping.
 */
static int add_inverse_map (CMap *icmap, mapDef *mtab,
			    unsigned char *codestr, int depth,
			    unsigned char *used_slot);
static int add_map         (CMap *cmap,  mapDef *mtab,
			    unsigned char *codestr, int depth);
static CMap *invert_cmap  (CMap *cmap, unsigned char *used_slot);
static CMap *flatten_cmap (CMap *cmap);
#endif /* 0 */

static int
block_count (mapDef *mtab, int c)
{
  int count = 0, n;

  n  = mtab[c].len - 1;
  c += 1;
  for (; c < 256; c++) {
    if (LOOKUP_CONTINUE(mtab[c].flag) ||
	!MAP_DEFINED(mtab[c].flag)     ||
	(MAP_TYPE(mtab[c].flag) != MAP_IS_CID &&
	 MAP_TYPE(mtab[c].flag) != MAP_IS_CODE) ||
	mtab[c-1].len != mtab[c].len)
      break;
    else if (!memcmp(mtab[c-1].code, mtab[c].code, n) &&
	     mtab[c-1].code[n] < 255 &&
	     mtab[c-1].code[n] + 1 == mtab[c].code[n])
      count++;
    else {
      break;
    }
  }

  return count;
}

static int
sputx (unsigned char c, char **s, char *end)
{
  char hi = (c >> 4), lo = c & 0x0f;

  if (*s + 2 > end)
    ERROR("Buffer overflow.");
  **s = (hi < 10) ? hi + '0' : hi + '7';
  *(*s+1) = (lo < 10) ? lo + '0' : lo + '7';
  *s += 2;

  return 2;
}

static int
write_map (mapDef *mtab, int count,
	   unsigned char *codestr, int depth,
	   struct sbuf *wbuf, pdf_obj *stream)
{
  int     c, i, block_length;
  mapDef *mtab1;
  /* Must be greater than 1 */
#define BLOCK_LEN_MIN 2
  struct {
    int start, count;
  } blocks[256/BLOCK_LEN_MIN+1];
  int num_blocks = 0;

  for (c = 0; c < 256; c++) {
    codestr[depth] = (unsigned char) (c & 0xff);
    if (LOOKUP_CONTINUE(mtab[c].flag)) {
      mtab1 = mtab[c].next;
      count = write_map(mtab1, count,
			codestr, depth + 1, wbuf, stream);
    } else {
      if (MAP_DEFINED(mtab[c].flag)) {
	switch (MAP_TYPE(mtab[c].flag)) {
	case MAP_IS_CID: case MAP_IS_CODE:
	  block_length = block_count(mtab, c);
	  if (block_length >= BLOCK_LEN_MIN) {
	    blocks[num_blocks].start = c;
	    blocks[num_blocks].count = block_length;
	    num_blocks++;
	    c += block_length;
	  } else {
	    *(wbuf->curptr)++ = '<';
	    for (i = 0; i <= depth; i++)
	      sputx(codestr[i], &(wbuf->curptr), wbuf->limptr);
	    *(wbuf->curptr)++ = '>';
	    *(wbuf->curptr)++ = ' ';
	    *(wbuf->curptr)++ = '<';
	    for (i = 0; i < mtab[c].len; i++)
	      sputx(mtab[c].code[i], &(wbuf->curptr), wbuf->limptr);
	    *(wbuf->curptr)++ = '>';
	    *(wbuf->curptr)++ = '\n';
	    count++;
	  }
	  break;
	case MAP_IS_NAME:
	  ERROR("%s: Unexpected error...", CMAP_DEBUG_STR);
	  break;
	case MAP_IS_NOTDEF:
	  break;
	default:
	  ERROR("%s: Unknown mapping type: %d",
		CMAP_DEBUG_STR, MAP_TYPE(mtab[c].flag));
	}
      }
    }

    /* Flush if necessary */
    if (count >= 100 ||
	wbuf->curptr >= wbuf->limptr ) {
      char fmt_buf[32];
      if (count > 100)
	ERROR("Unexpected error....: %d", count);
      sprintf(fmt_buf, "%d beginbfchar\n", count);
      pdf_add_stream(stream, fmt_buf,  strlen(fmt_buf));
      pdf_add_stream(stream,
		     wbuf->buf, (int) (wbuf->curptr - wbuf->buf));
      wbuf->curptr = wbuf->buf;
      pdf_add_stream(stream,
		     "endbfchar\n", strlen("endbfchar\n"));
      count = 0;
    }
  }

  if (num_blocks > 0) {
    char fmt_buf[32];

    if (count > 0) {
      sprintf(fmt_buf, "%d beginbfchar\n", count);
      pdf_add_stream(stream, fmt_buf,  strlen(fmt_buf));
      pdf_add_stream(stream,
		     wbuf->buf, (int) (wbuf->curptr - wbuf->buf));
      wbuf->curptr = wbuf->buf;
      pdf_add_stream(stream,
		     "endbfchar\n", strlen("endbfchar\n"));
      count = 0;
    }
    sprintf(fmt_buf, "%d beginbfrange\n", num_blocks);
    pdf_add_stream(stream, fmt_buf, strlen(fmt_buf));
    for (i = 0; i < num_blocks; i++) {
      int j;

      c = blocks[i].start;
      *(wbuf->curptr)++ = '<';
      for (j = 0; j < depth; j++)
	sputx(codestr[j], &(wbuf->curptr), wbuf->limptr);
      sputx((unsigned char)c, &(wbuf->curptr), wbuf->limptr);
      *(wbuf->curptr)++ = '>';
      *(wbuf->curptr)++ = ' ';
      *(wbuf->curptr)++ = '<';
      for (j = 0; j < depth; j++)
	sputx(codestr[j], &(wbuf->curptr), wbuf->limptr);
      sputx((unsigned char)(c + blocks[i].count), &(wbuf->curptr), wbuf->limptr);
      *(wbuf->curptr)++ = '>';
      *(wbuf->curptr)++ = ' ';
      *(wbuf->curptr)++ = '<';
      for (j = 0; j < mtab[c].len; j++)
	sputx(mtab[c].code[j], &(wbuf->curptr), wbuf->limptr);
      *(wbuf->curptr)++ = '>';
      *(wbuf->curptr)++ = '\n';
    }
    pdf_add_stream(stream,
		   wbuf->buf, (int) (wbuf->curptr - wbuf->buf));
    wbuf->curptr = wbuf->buf;
    pdf_add_stream(stream,
		   "endbfrange\n", strlen("endbfrange\n"));
  }

  return count;
}

#define CMAP_BEGIN "\
/CIDInit /ProcSet findresource begin\n\
12 dict begin\n\
begincmap\n\
"

#define CMAP_END "\
endcmap\n\
CMapName currentdict /CMap defineresource pop\n\
end\n\
end\n\
"

pdf_obj *
CMap_create_stream (CMap *cmap)
{
  pdf_obj         *stream;
  pdf_obj         *stream_dict;
  CIDSysInfo      *csi;
  struct sbuf      wbuf;
  struct rangeDef *ranges;
  unsigned char   *codestr;
  int              i, j, count = 0;

  if (!cmap || !CMap_is_valid(cmap)) {
    WARN("Invalid CMap");
    return NULL;
  }

  if (cmap->type == CMAP_TYPE_IDENTITY)
    return NULL;

  stream      = pdf_new_stream(STREAM_COMPRESS);
  stream_dict = pdf_stream_dict(stream);

  csi = CMap_get_CIDSysInfo(cmap);
  if (!csi) {
    csi = (cmap->type != CMAP_TYPE_TO_UNICODE) ?
      &CSI_IDENTITY : &CSI_UNICODE;
  }

  if (cmap->type != CMAP_TYPE_TO_UNICODE) {
    pdf_obj *csi_dict;

    csi_dict = pdf_new_dict();
    pdf_add_dict(csi_dict,
		 pdf_new_name("Registry"),
		 pdf_new_string(csi->registry, strlen(csi->registry)));
    pdf_add_dict(csi_dict,
		 pdf_new_name("Ordering"),
		 pdf_new_string(csi->ordering, strlen(csi->ordering)));
    pdf_add_dict(csi_dict,
		 pdf_new_name("Supplement"),
		 pdf_new_number(csi->supplement));

    pdf_add_dict(stream_dict,
		 pdf_new_name("Type"),
		 pdf_new_name("CMap"));
    pdf_add_dict(stream_dict,
		 pdf_new_name("CMapName"),
		 pdf_new_name(cmap->name));
    pdf_add_dict(stream_dict,
		 pdf_new_name("CIDSystemInfo"), csi_dict);
    if (cmap->wmode != 0)
      pdf_add_dict(stream_dict,
		   pdf_new_name("WMode"),
		   pdf_new_number(cmap->wmode));
  }

  /* TODO:
   * Predefined CMaps need not to be embedded.
   */
  if (cmap->useCMap) {
    ERROR("UseCMap found (not supported yet)...");
    if (CMap_is_Identity(cmap->useCMap)) { /* not sure */
      if (CMap_get_wmode(cmap) == 1) {
	pdf_add_dict(stream_dict,
		     pdf_new_name("UseCMap"),
		     pdf_new_name("Identity-V"));
      } else {
    	pdf_add_dict(stream_dict,
		     pdf_new_name("UseCMap"),
		     pdf_new_name("Identity-H"));
      }
    } else {
      int      res_id;
      pdf_obj *ucmap_ref;

      res_id = pdf_findresource("CMap", CMap_get_name(cmap->useCMap));
      if (res_id >= 0) {
	ucmap_ref = pdf_get_resource_reference(res_id);
      } else {
	pdf_obj *ucmap_obj;

	ucmap_obj = CMap_create_stream(cmap->useCMap);
	if (!ucmap_obj) {
	  ERROR("Uh ah. I cannot continue...");
	}

	res_id = pdf_defineresource("CMap",
				    CMap_get_name(cmap->useCMap),
				    ucmap_obj, PDF_RES_FLUSH_IMMEDIATE);
	ucmap_ref = pdf_get_resource_reference(res_id);
      }
      pdf_add_dict(stream_dict, pdf_new_name("UseCMap"), ucmap_ref);
    }
  }

#define WBUF_SIZE 4096
  wbuf.buf = NEW(WBUF_SIZE, char);
  codestr  = NEW(cmap->profile.maxBytesIn, unsigned char);
  memset(codestr, 0, cmap->profile.maxBytesIn);

  wbuf.curptr = wbuf.buf;
  wbuf.limptr = wbuf.buf + WBUF_SIZE -
    2 * (cmap->profile.maxBytesIn + cmap->profile.maxBytesOut) + 16;

  /* Start CMap */
  pdf_add_stream(stream, (const void *) CMAP_BEGIN, strlen(CMAP_BEGIN));

  wbuf.curptr += sprintf(wbuf.curptr, "/CMapName /%s def\n", cmap->name);
  wbuf.curptr += sprintf(wbuf.curptr, "/CMapType %d def\n" , cmap->type);
  if (cmap->wmode != 0 &&
      cmap->type != CMAP_TYPE_TO_UNICODE)
    wbuf.curptr += sprintf(wbuf.curptr, "/WMode %d def\n", cmap->wmode);

#define CMAP_CSI_FMT "/CIDSystemInfo <<\n\
  /Registry (%s)\n\
  /Ordering (%s)\n\
  /Supplement %d\n\
>> def\n"
  wbuf.curptr += sprintf(wbuf.curptr, CMAP_CSI_FMT,
			 csi->registry, csi->ordering, csi->supplement);
  pdf_add_stream(stream, wbuf.buf, (int)(wbuf.curptr - wbuf.buf));
  wbuf.curptr = wbuf.buf;

  /* codespacerange */
  ranges = cmap->codespace.ranges;
  wbuf.curptr += sprintf(wbuf.curptr,
			 "%d begincodespacerange\n", cmap->codespace.num);
  for (i = 0; i < cmap->codespace.num; i++) {
    *(wbuf.curptr)++ = '<';
    for (j = 0; j < ranges[i].dim; j++) {
      sputx(ranges[i].codeLo[j], &(wbuf.curptr), wbuf.limptr);
    }
    *(wbuf.curptr)++ = '>';
    *(wbuf.curptr)++ = ' ';
    *(wbuf.curptr)++ = '<';
    for (j = 0; j < ranges[i].dim; j++) {
      sputx(ranges[i].codeHi[j], &(wbuf.curptr), wbuf.limptr);
    }
    *(wbuf.curptr)++ = '>';
    *(wbuf.curptr)++ = '\n';
  }
  pdf_add_stream(stream, wbuf.buf, (int)(wbuf.curptr - wbuf.buf));
  wbuf.curptr = wbuf.buf;
  pdf_add_stream(stream,
		 "endcodespacerange\n", strlen("endcodespacerange\n"));

  /* CMap body */
  if (cmap->mapTbl) {
    count = write_map(cmap->mapTbl,
		      0, codestr, 0, &wbuf, stream); /* Top node */
    if (count > 0) { /* Flush */
      char fmt_buf[32];
      if (count > 100)
	ERROR("Unexpected error....: %d", count);
      sprintf(fmt_buf, "%d beginbfchar\n", count);
      pdf_add_stream(stream, fmt_buf,  strlen(fmt_buf));
      pdf_add_stream(stream,
		     wbuf.buf, (int) (wbuf.curptr - wbuf.buf));
      pdf_add_stream(stream,
		     "endbfchar\n", strlen("endbfchar\n"));
      count = 0;
      wbuf.curptr = wbuf.buf;
    }
  }
  /* End CMap */
  pdf_add_stream(stream, CMAP_END, strlen(CMAP_END));

  RELEASE(codestr);
  RELEASE(wbuf.buf);

  return stream;
}

#if 0
/* Not completed yet */

static int
add_inverse_map (CMap *icmap, mapDef *mtab,
		 unsigned char *codestr, int depth,
		 unsigned char *used_slot)
{
  CID     cid;
  int     c;
  mapDef *mtab1;

  for (c = 0; c < 256; c++) {
    codestr[depth] = (unsigned char) (c & 0xff);
    if (LOOKUP_CONTINUE(mtab[c].flag)) {
      mtab1 = mtab[c].next;
      add_inverse_map(icmap, mtab1, codestr, depth + 1, used_slot);
    } else {
      if (MAP_DEFINED(mtab[c].flag)) {
	switch (MAP_TYPE(mtab[c].flag)) {
	  /* We should restrict it to to-CID mapping.
	   * However...
	   */
	case MAP_IS_CID: case MAP_IS_CODE:
	  if (mtab[c].len == 2) {
	    cid = (mtab[c].code[0] << 8)|mtab[c].code[1];
#ifndef is_used_char2
#define is_used_char2(b,c) (((b)[(c)/8]) & (1 << (7-((c)%8))))
#endif
	    if (!used_slot ||
		is_used_char2(used_slot, cid)) {
	      CMap_add_bfchar(icmap,
			      mtab[c].code, mtab[c].len,
			      codestr, depth + 1);
	    }
	  }
	  break;
	case MAP_IS_NAME:
	  ERROR("%s: Unexpected error...", CMAP_DEBUG_STR);
	  break;
	case MAP_IS_NOTDEF:
	  break;
	default:
	  ERROR("%s: Unknown mapping type: %d",
		CMAP_DEBUG_STR, MAP_TYPE(mtab[c].flag));
	}
      }
    }
  }

  return 0;
}

static int
add_map (CMap *cmap, mapDef *mtab,
	 unsigned char *codestr, int depth)
{
  int     c;
  mapDef *mtab1;

  for (c = 0; c < 256; c++) {
    codestr[depth] = (unsigned char) (c & 0xff);
    if (LOOKUP_CONTINUE(mtab[c].flag)) {
      mtab1 = mtab[c].next;
      add_map(cmap, mtab1, codestr, depth + 1);
    } else {
      if (MAP_DEFINED(mtab[c].flag)) {
	switch (MAP_TYPE(mtab[c].flag)) {
	case MAP_IS_CID: case MAP_IS_CODE:
	  CMap_add_bfchar(cmap,
			  codestr, depth + 1,
			  mtab[c].code, mtab[c].len);
	  break;
	case MAP_IS_NAME:
	  ERROR("%s: Unexpected error...", CMAP_DEBUG_STR);
	  break;
	case MAP_IS_NOTDEF:
	  break;
	default:
	  ERROR("%s: Unknown mapping type: %d",
		CMAP_DEBUG_STR, MAP_TYPE(mtab[c].flag));
	}
      }
    }
  }

  return 0;
}


static CMap *
invert_cmap (CMap *cmap, unsigned char *used_slot)
{
  CMap *icmap;
  unsigned char *codestr;
  unsigned char  range_min[2] = {0x00, 0x00}; /* CID */
  unsigned char  range_max[2] = {0xff, 0xfe}; /* CID */

  cmap = flatten_cmap(cmap);
  ASSERT(cmap);

  icmap = CMap_new();
  CMap_set_type(icmap, CMAP_TYPE_CID_TO_CODE);
  CMap_add_codespacerange(icmap, range_min, range_max, 2);

  if (cmap->mapTbl) {
    codestr = NEW(cmap->profile.maxBytesIn, unsigned char);
    memset(codestr, 0, cmap->profile.maxBytesIn);
    add_inverse_map(icmap, cmap->mapTbl,
		    codestr, 0, used_slot); /* top node */
    RELEASE(codestr);
  }

  CMap_release(cmap);

  return icmap;
}

static CMap *
flatten_cmap (CMap *cmap)
{
  CMap     *fcmap;
  rangeDef *range;
  int       i;

  ASSERT(cmap);

  fcmap = CMap_new();

  for (i = 0; i < cmap->codespace.num; i++) {
    range = &(cmap->codespace.ranges[i]);
    CMap_add_codespacerange(fcmap, range->codeLo, range->codeHi, range->dim);
  }
  CMap_set_CIDSysInfo(fcmap, cmap->CSI);
  CMap_set_type (fcmap, cmap->type );
  CMap_set_wmode(fcmap, cmap->wmode);

  fcmap->flags   = cmap->flags;
  fcmap->version = cmap->version;

  while (cmap) {
    if (cmap->mapTbl) {
      unsigned char *codestr;

      codestr = NEW(cmap->profile.maxBytesIn, unsigned char);
      memset(codestr, 0, cmap->profile.maxBytesIn);
      add_map(fcmap, cmap->mapTbl, codestr, 0); /* top node */
      RELEASE(codestr);
    }
    cmap = cmap->useCMap;
  }

  return fcmap;
}

pdf_obj *
CMap_ToCode_stream (CMap *cmap, const char *cmap_name,
		    CIDSysInfo *csi, int cmap_type,
		    unsigned char *used_slot, int flags)
{
  pdf_obj *stream = NULL;
  CMap    *icmap;

  ASSERT(cmap && cmap_name);

  if (cmap->type !=
      CMAP_TYPE_CODE_TO_CID)
    return NULL;

  icmap = invert_cmap(cmap, used_slot);
  if (icmap) {
    CMap_set_name(icmap, cmap_name);
    if (csi)
      CMap_set_CIDSysInfo(icmap, csi);
    else {
      CMap_set_CIDSysInfo(icmap, &(CSI_IDENTITY));
    }
    CMap_set_type(icmap, cmap_type);

    stream = CMap_create_stream(icmap, flags);

    CMap_release(icmap);
  }

  return stream;
}
#endif /* 0 */
