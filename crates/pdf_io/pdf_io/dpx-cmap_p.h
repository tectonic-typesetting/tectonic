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

#ifndef _CMAP_P_H_
#define _CMAP_P_H_

#include "tectonic_bridge_core.h"

#include "dpx-cid.h"

/* Mapping types, MAP_IS_NAME is not supported. */
#define MAP_IS_CID      (1 << 0)
#define MAP_IS_NAME     (1 << 1)
#define MAP_IS_CODE     (1 << 2)
#define MAP_IS_NOTDEF   (1 << 3)

#define MAP_IS_UNDEF    0
#define MAP_TYPE_MASK   0x00f

#define MAP_DEFINED(e)  (((e) & MAP_TYPE_MASK) != MAP_IS_UNDEF ? 1 : 0)
#define MAP_TYPE(e)     ((e) & MAP_TYPE_MASK)

/* Lookup flags */
#define MAP_LOOKUP_END      0
#define MAP_LOOKUP_CONTINUE (1 << 4)
#define LOOKUP_CONTINUE(f) ((f) & MAP_LOOKUP_CONTINUE)
#define LOOKUP_END(f)      (!LOOKUP_CONTINUE((f)))

/* DEBUG */
#define CMAP_DEBUG_STR "CMap"
#define CMAP_DEBUG     3

/* Codespacerange */
typedef struct rangeDef {
  size_t dim;            /* Dimension of this codespacerange */
  unsigned char *codeLo; /* Lower bounds of valid input code */
  unsigned char *codeHi; /* Upper bounds of valid input code */
} rangeDef;

typedef struct mapDef {
  int            flag;
  size_t         len;  /* 2 for CID, variable for Code..  */
  unsigned char *code; /* CID (as 16-bit BE), Code ...    */
  struct mapDef *next; /* Next Subtbl for LOOKUP_CONTINUE */
} mapDef;

#define MEM_ALLOC_SIZE  4096
typedef struct mapData {
  unsigned char  *data; /* CID, Code... MEM_ALLOC_SIZE bytes  */
  struct mapData *prev; /* Previous mapData data segment      */
  int             pos;  /* Position of next free data segment */
} mapData;

struct CMap {
  char  *name;
  int    type;     /* CMapType: 1 for usual CMaps,
                    *           2 for ToUnicode CMaps,
                    *           0 for IDENTITY is also defined for convenience.
                    */
  int    wmode;    /* WMode: 0 for Horizontal, 1 for Vertical. */
  CIDSysInfo *CSI; /* CIDSystemInfo */

  struct CMap *useCMap;

  struct {
    unsigned int num;
    unsigned int max;
    rangeDef  *ranges;
  } codespace;

  mapDef  *mapTbl;  /* First 256 segment of mapping table */
  mapData *mapData; /* Storage for actual CMap data       */

  /* Additional data used by cmap.c, etc. */

  int flags; /* Decoder flags Not used yet. */

  struct {
    size_t minBytesIn;
    size_t maxBytesIn;
    size_t minBytesOut;
    size_t maxBytesOut;
  } profile;
};

#endif /* _CMAP_P_H_ */
