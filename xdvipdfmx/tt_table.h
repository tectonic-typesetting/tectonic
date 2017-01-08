/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _TT_TABLE_H_
#define _TT_TABLE_H_

#include "sfnt.h"

#define TT_HEAD_TABLE_SIZE 54UL
#define TT_MAXP_TABLE_SIZE 32UL
#define TT_HHEA_TABLE_SIZE 36UL
#define TT_VHEA_TABLE_SIZE 36UL

struct tt_head_table
{
  Fixed  version;
  Fixed  fontRevision;
  ULONG  checkSumAdjustment;
  ULONG  magicNumber;
  USHORT flags;
  USHORT unitsPerEm;
  BYTE   created[8];
  BYTE   modified[8];
  FWord  xMin, yMin, xMax, yMax;
  USHORT macStyle;
  USHORT lowestRecPPEM;
  SHORT  fontDirectionHint;
  SHORT  indexToLocFormat;
  SHORT  glyphDataFormat;
};

struct tt_hhea_table
{
  Fixed  version;
  FWord  ascent;
  FWord  descent;
  FWord  lineGap;
  uFWord advanceWidthMax;
  FWord  minLeftSideBearing;
  FWord  minRightSideBearing;
  FWord  xMaxExtent;
  SHORT  caretSlopeRise;
  SHORT  caretSlopeRun;
  FWord  caretOffset;
  SHORT  reserved[4]; /* set to 0 */
  SHORT  metricDataFormat;
  USHORT numOfLongHorMetrics;
  USHORT numOfExSideBearings; /* extra information */
};

struct tt_vhea_table
{
  Fixed  version;
  SHORT  vertTypoAscender;  /* v.1.1 name */
  SHORT  vertTypoDescender; /* v.1.1 name */
  SHORT  vertTypoLineGap;   /* v.1.1 name */ 
  SHORT  advanceHeightMax;
  SHORT  minTopSideBearing;
  SHORT  minBottomSideBearing;
  SHORT  yMaxExtent;
  SHORT  caretSlopeRise;
  SHORT  caretSlopeRun;
  SHORT  caretOffset;
  SHORT  reserved[4]; /* set to 0 */
  SHORT  metricDataFormat;
  USHORT numOfLongVerMetrics;
  USHORT numOfExSideBearings; /* extra information */
};

struct tt_maxp_table
{
  Fixed  version;
  USHORT numGlyphs;
  USHORT maxPoints;
  USHORT maxContours;
  USHORT maxComponentPoints;
  USHORT maxComponentContours;
  USHORT maxZones;
  USHORT maxTwilightPoints;
  USHORT maxStorage;
  USHORT maxFunctionDefs;
  USHORT maxInstructionDefs;
  USHORT maxStackElements;
  USHORT maxSizeOfInstructions;
  USHORT maxComponentElements;
  USHORT maxComponentDepth;
};

struct tt_os2__table
{
  USHORT  version; /* 0x0001 or 0x0002 */
  SHORT   xAvgCharWidth;  
  USHORT  usWeightClass;  
  USHORT  usWidthClass;   
  SHORT   fsType;  /* if (faType & 0x08) editable_embedding */
  SHORT   ySubscriptXSize;        
  SHORT   ySubscriptYSize;      
  SHORT   ySubscriptXOffset;
  SHORT   ySubscriptYOffset;      
  SHORT   ySuperscriptXSize;      
  SHORT   ySuperscriptYSize;      
  SHORT   ySuperscriptXOffset;    
  SHORT   ySuperscriptYOffset;    
  SHORT   yStrikeoutSize; 
  SHORT   yStrikeoutPosition;     
  SHORT   sFamilyClass;   
  BYTE    panose[10];
  ULONG   ulUnicodeRange1;
  ULONG   ulUnicodeRange2;
  ULONG   ulUnicodeRange3;
  ULONG   ulUnicodeRange4;
  CHAR    achVendID[4];   
  USHORT  fsSelection;    
  USHORT  usFirstCharIndex;
  USHORT  usLastCharIndex;
  SHORT   sTypoAscender;  /* TTF spec. from MS is wrong */
  SHORT   sTypoDescender; /* TTF spec. from MS is wrong */
  SHORT   sTypoLineGap;   /* TTF spec. from MS is wrong */
  USHORT  usWinAscent;   
  USHORT  usWinDescent;    
  ULONG   ulCodePageRange1;
  ULONG   ulCodePageRange2;
  /* version 0x0002 */
  SHORT   sxHeight;
  SHORT   sCapHeight;
  USHORT  usDefaultChar;
  USHORT  usBreakChar;
  USHORT  usMaxContext;
};

struct tt_vertOriginYMetrics {
  USHORT glyphIndex;
  SHORT  vertOriginY;
};

struct tt_VORG_table {
  SHORT  defaultVertOriginY;
  USHORT numVertOriginYMetrics;
  struct tt_vertOriginYMetrics *vertOriginYMetrics;
};

/* hmtx and vmtx */
struct tt_longMetrics
{
  USHORT advance;
  SHORT  sideBearing;
};


/* head, hhea, maxp */
extern char  *tt_pack_head_table (struct tt_head_table *table);
extern struct tt_head_table *tt_read_head_table (sfnt *sfont);
extern char  *tt_pack_hhea_table (struct tt_hhea_table *table);
extern struct tt_hhea_table *tt_read_hhea_table (sfnt *sfont);
extern char  *tt_pack_maxp_table (struct tt_maxp_table *table);
extern struct tt_maxp_table *tt_read_maxp_table (sfnt *sfont);

/* vhea */
#if 0
extern char  *tt_pack_vhea_table (struct tt_vhea_table *table);
#endif
extern struct tt_vhea_table *tt_read_vhea_table (sfnt *sfont);

/* VORG */
extern struct tt_VORG_table *tt_read_VORG_table (sfnt *sfont);

/* hmtx and vmtx */
extern struct tt_longMetrics *tt_read_longMetrics (sfnt *sfont,
						   USHORT numGlyphs, USHORT numLongMetrics, USHORT numExSideBearings);

/* OS/2 table */
extern struct tt_os2__table *tt_read_os2__table (sfnt *sfont);

/* name table */
extern USHORT tt_get_ps_fontname (sfnt *sfont, char *dest, USHORT destlen);

#endif /* _TT_TABLE_H_ */
