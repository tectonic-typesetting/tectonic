/* help.h: help messages for web2c programs.

   This is included by everyone, from cpascal.h.  This is better than
   putting the help messages directly in the change files because (1)
   multiline strings aren't supported by tangle, and it would be a pain
   to make up a new syntax for them in web2c, and (2) when a help msg
   changes, we need only recompile, not retangle or reconvert.  The
   downside is that everything gets recompiled when any msg changes, but
   that's better than having umpteen separate tiny files.  (For one
   thing, the messages have a lot in common, so it's nice to have them
   in one place.)

Copyright 1995, 1996, 2009, 2011, 2012 Karl Berry.
Copyright 2008 Taco Hoekwater.
Copyright 2001, 2003, 2004 Olaf Weber.

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2, or (at your option)
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, see <http://www.gnu.org/licenses/>.  */

/* Help messages for TeX & MF family are in texmfmp-help.h, to
   prevent multiple-definition errors. */

#ifndef HELP_H
#define HELP_H

#ifdef BIBTEX
const_string BIBTEXHELP[] = {
    "Usage: bibtex [OPTION]... AUXFILE[.aux]",
    "  Write bibliography for entries in AUXFILE to AUXFILE.bbl,",
    "  along with a log file AUXFILE.blg."
    "",
    "-min-crossrefs=NUMBER  include item after NUMBER cross-refs; default 2",
    "-terse                 do not print progress reports",
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};    
#endif /* BIBTEX */

#ifdef CWEB
const_string CTANGLEHELP[] = {
    "Usage: ctangle [OPTIONS] WEBFILE[.w] [{CHANGEFILE[.ch]|-} [OUTFILE[.c]]]",
    "  Tangle WEBFILE with CHANGEFILE into a C/C++ program.",
    "  Default CHANGEFILE is " DEV_NULL ";",
    "  C output goes to the basename of WEBFILE extended with `.c'",
    "  unless otherwise specified by OUTFILE; in this case, '-' specifies",
    "  a null CHANGEFILE.",
    "",
    "-b          suppress banner line on terminal",
    "-h          suppress success message on completion",
    "-p          suppress progress report messages",
    "+s          print usage statistics",
    "--help      display this help and exit",
    "--version   output version information and exit",
    NULL
};

const_string CWEAVEHELP[] = {
    "Usage: cweave [OPTIONS] WEBFILE[.w] [{CHANGEFILE[.ch]|-} [OUTFILE[.tex]]]",
    "  Weave WEBFILE with CHANGEFILE into a TeX document.",
    "  Default CHANGEFILE is " DEV_NULL ";",
    "  TeX output goes to the basename of WEBFILE extended with `.tex'",
    "  unless otherwise specified by OUTFILE; in this case, '-' specifies",
    "  a null CHANGEFILE.",
    "",
    "-b          suppress banner line on terminal",
    "-f          do not force a newline after every C statement in output",
    "-h          suppress success message on completion",
    "-p          suppress progress report messages",
    "-x          omit indices, section names, table of contents",
    "+e          enclose C material in \\PB{...}",
    "+s          print usage statistics",
    "--help      display this help and exit",
    "--version   output version information and exit",
    NULL
};
#endif /* CWEB */

#ifdef DVICOPY
const_string DVICOPYHELP[] = {
    "Usage: dvicopy [OPTION]... [INDVI[.dvi] [OUTDVI[.dvi]]]",
    "  Expand virtual font references in INDVI to OUTDVI.",
    "  Defaults are standard input and standard output, respectively.",
    "",
    "-magnification=NUMBER  override existing magnification with NUMBER",
    "-max-pages=NUMBER      process NUMBER pages; default one million",
    "-page-start=PAGE-SPEC  start at PAGE-SPEC, for example `2' or `5.*.-2'",
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};
#endif /* DVICOPY */

#ifdef DVITYPE
const_string DVITYPEHELP[] = {
    "Usage: dvitype [OPTION]... DVIFILE[.dvi]",
    "  Verify and translate DVIFILE to human-readable form,",
    "  written to standard output.",
    "",
    "-dpi=REAL              set resolution to REAL pixels per inch; default 300.0",
    "-magnification=NUMBER  override existing magnification with NUMBER",
    "-max-pages=NUMBER      process NUMBER pages; default one million",
    "-output-level=NUMBER   verbosity level, from 0 to 4; default 4",
    "-page-start=PAGE-SPEC  start at PAGE-SPEC, for example `2' or `5.*.-2'",
    "-show-opcodes          show numeric opcodes (in decimal)",
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};
#endif /* DVITYPE */

#ifdef GFTODVI
const_string GFTODVIHELP[] = {
    "Usage: gftodvi [OPTION]... GFNAME",
    "  Translate each character in GFNAME to a page in a DVI file,",
    "  which is named with the basename of GFNAME extended with `.dvi'.",
    "",
    "-overflow-label-offset=REAL  override 2.1in offset for overflow labels",
    "-help                        display this help and exit",
    "-verbose                     display progress reports",
    "-version                     output version information and exit",
    NULL
};
#endif /* GFTODVI */

#ifdef GFTOPK
const_string GFTOPKHELP[] = {
    "Usage: gftopk [OPTION]... GFNAME [PKFILE]",
    "  Translate the bitmap font GFNAME to PKFILE.",
    "  Default PKFILE is basename of GFNAME extended with `pk'.",
    "",
    "-help       display this help and exit",
    "-verbose    display progress reports",
    "-version    output version information and exit",
    NULL
};
#endif /* GFTOPK */

#ifdef GFTYPE
const_string GFTYPEHELP[] = {
    "Usage: gftype [OPTION]... GFNAME",
    "  Verify and translate the bitmap font GFNAME to human-readable form,",
    "  written to standard output.",
    "",
    "-images       show characters as pixels",
    "-mnemonics    translate all GF commands",
    "-help         display this help and exit",
    "-version      output version information and exit",
    NULL
};
#endif /* GFTYPE */

#ifdef MFT
const_string MFTHELP[] = {
    "Usage: mft [OPTION]... NAME[.mf|.mp]",
    "  Translate MFNAME to TeX for printing, using the mftmac.tex (or",
    "  mptmac.tex) macros.  Output goes to basename of NAME extended",
    "  with `.tex'.",
    "",
    "-change=CHFILE  apply the change file CHFILE as with tangle and weave",
    "-metapost       assume NAME is a METAPOST source file",
    "-style=MFTNAME  use MFTNAME instead of plain.mft (or mplain.mft)",
    "                 (this option can be given more than once)",
    "-help           display this help and exit",
    "-version        output version information and exit",
    NULL
};
#endif /* MFT */

#ifdef ODVICOPY
const_string ODVICOPYHELP[] = {
    "Usage: odvicopy [OPTION]... [INDVI[.dvi] [OUTDVI[.dvi]]]",
    "  Expand virtual font references in INDVI to OUTDVI.",
    "  Defaults are standard input and standard output, respectively.",
    "",
    "-magnification=NUMBER  override existing magnification with NUMBER",
    "-max-pages=NUMBER      process NUMBER pages; default one million",
    "-page-start=PAGE-SPEC  start at PAGE-SPEC, for example `2' or `5.*.-2'",
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};
#endif /* ODVICOPY */

#ifdef ODVITYPE
const_string ODVITYPEHELP[] = {
    "Usage: odvitype [OPTION]... DVIFILE[.dvi]",
    "  Verify and translate DVIFILE to human-readable form,",
    "  written to standard output.",
    "",
    "-dpi=REAL              set resolution to REAL pixels per inch; default 300.0",
    "-magnification=NUMBER  override existing magnification with NUMBER",
    "-max-pages=NUMBER      process NUMBER pages; default one million",
    "-output-level=NUMBER   verbosity level, from 0 to 4; default 4",
    "-page-start=PAGE-SPEC  start at PAGE-SPEC, for example `2' or `5.*.-2'",
    "-show-opcodes          show numeric opcodes (in decimal)",
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};
#endif /* ODVITYPE */

#ifdef OFM2OPL
const_string OFM2OPLHELP[] = {
    "Usage: ofm2opl [OPTION]... OFMNAME[.ofm] [OPLFILE[.opl]]",
    "  Translate the font metrics OFMNAME to human-readable property list file",
    "  OPLFILE or standard output.",
    "",
#ifdef OMFONTS
    "-char-format=TYPE      output character codes according to TYPE,",
    "                        either `num' or `ascii'; default is num,",
    "                        ascii = ascii letters and digits, num for all else",
    "-num-format=TYPE       output numbers according to TYPE,",
    "                        either `hex' or `octal'; default is hex",
    "-text-format=TYPE      output coding scheme and family according to TYPE,",
    "                        either `mixed' or `upper' case; default is mixed",
    "-charcode-format=TYPE  (just for compatibility)",
    "                       output character codes according to TYPE,",
    "                        `hex' , `octal' or `ascii'; default is hex,",
    "                        ascii = ascii letters and digits, hex for all else",
#else
    "-charcode-format=TYPE  output character codes according to TYPE,",
    "                        either `hex' or `ascii'; default is hex,",
    "                        ascii = ascii letters and digits, hex for all else",
#endif
    "-help                  display this help and exit",
    "-verbose               display progress reports",
    "-version               output version information and exit",
    NULL
};
#endif /* OFM2OPL */

#ifdef OPL2OFM
const_string OPL2OFMHELP[] = {
    "Usage: opl2ofm [OPTION]... OPLFILE[.opl] [OFMFILE[.ofm]]",
    "  Translate the property list OPLFILE to OFMFILE.",
    "  Default OFMFILE is basename of OPLFILE extended with `.ofm'.",
    "",
    "-help       display this help and exit",
    "-verbose    display progress reports",
    "-version    output version information and exit",
    NULL
};
#endif /* OPL2OFM */

#if defined (OTANGLE) || defined (OTANGLEBOOT)
const_string OTANGLEHELP[] = {
    "Usage: otangle [OPTION]... WEBFILE[.web] [CHANGEFILE[.ch]]",
    "  Tangle WEBFILE with CHANGEFILE into a Pascal program.",
    "  Default CHANGEFILE is " DEV_NULL ";",
    "  Pascal output goes to the basename of WEBFILE extended with `.p',",
    "  and a string pool file, if necessary, to the same extended with `.pool'.",
    "",
    "-help       display this help and exit",
    "-version    output version information and exit",
    NULL
};
#endif /* OTANGLE */

#ifdef OVF2OVP
const_string OVF2OVPHELP[] = {
    "Usage: ovf2ovp [OPTION]... OVFNAME[.ovf] [OFMNAME[.ofm] [OVPFILE[.ovp]]]",
    "  Translate OVFNAME and companion OFMNAME to human-readable",
    "  virtual property list file OVPFILE or standard output.",
    "  If OFMNAME is not specified, OVFNAME (with `.ovf' removed) is used.",
    "",
#ifdef OMFONTS
    "-char-format=TYPE      output character codes according to TYPE,",
    "                        either `num' or `ascii'; default is num,",
    "                        ascii = ascii letters and digits, num for all else",
    "-num-format=TYPE       output numbers according to TYPE,",
    "                        either `hex' or `octal'; default is hex",
    "-text-format=TYPE      output coding scheme and family according to TYPE,",
    "                        either `mixed' or `upper' case; default is mixed",
    "-charcode-format=TYPE  (just for compatibility)",
    "                       output character codes according to TYPE,",
    "                        `hex' , `octal' or `ascii'; default is hex,",
    "                        ascii = ascii letters and digits, hex for all else",
#else
    "-charcode-format=TYPE  output character codes according to TYPE,",
    "                        either `hex' or `ascii'; default is hex,",
    "                        ascii = ascii letters and digits, hex for all else",
#endif
    "-help                  display this help and exit",
    "-verbose               display progress reports",
    "-version               output version information and exit",
    NULL
};
#endif /* OVF2OVP */

#ifdef OVP2OVF
const_string OVP2OVFHELP[] = {
    "Usage: ovp2ovf [OPTION]... OVPFILE[.ovp] [OVFFILE[.ovf] [OFMFILE[.ofm]]]",
    "  Translate OVPFILE to OVFFILE and companion OFMFILE.",
    "  Default OVFFILE is basename of OVPFILE extended with `.ovf'.",
    "  Default OFMFILE is basename of OVFFILE extended with `.ofm'.",
    "",
    "-help                  display this help and exit",
    "-verbose               display progress reports",
    "-version               output version information and exit",
    NULL
};
#endif /* OVP2OVF */

#ifdef PATGEN
const_string PATGENHELP[] = {
    "Usage: patgen [OPTION]... DICTIONARY PATTERNS OUTPUT TRANSLATE",
    "  Generate the OUTPUT hyphenation file for use with TeX",
    "  from the DICTIONARY, PATTERNS, and TRANSLATE files.",
    "",
    "-help           display this help and exit",
    "-version        output version information and exit",
    NULL
};
#endif /* PATGEN */

#ifdef PBIBTEX
const_string based_prog_name = "BibTeX";
const_string PBIBTEXHELP[] = {
    "Usage: pbibtex [OPTION]... AUXFILE[.aux]",
    "  Write bibliography for entries in AUXFILE to AUXFILE.bbl,",
    "  along with a log file AUXFILE.blg."
    "",
    "-kanji=STRING          set Japanese encoding (STRING=euc|jis|sjis|utf8)",
    "-min-crossrefs=NUMBER  include item after NUMBER cross-refs; default 2",
#if defined(WIN32)
    "-sjis-terminal         always output to stdout and stderr by CP932",
#endif
    "-terse                 do not print progress reports",
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};
#endif /* PBIBTEX */

#ifdef PDVITYPE
const_string based_prog_name = "DVItype";
const_string PDVITYPEHELP[] = {
    "Usage: pdvitype [OPTION]... DVIFILE[.dvi]",
    "  Verify and translate DVIFILE to human-readable form,",
    "  written to standard output.",
    "",
    "-dpi=REAL              set resolution to REAL pixels per inch; default 300.0",
    "-kanji=STRING          set Japanese encoding (STRING=euc|jis|sjis|utf8)",
    "-magnification=NUMBER  override existing magnification with NUMBER",
    "-max-pages=NUMBER      process NUMBER pages; default one million",
    "-output-level=NUMBER   verbosity level, from 0 to 4; default 4",
    "-page-start=PAGE-SPEC  start at PAGE-SPEC, for example `2' or `5.*.-2'",
    "-show-opcodes          show numeric opcodes (in decimal)",
#if defined(WIN32)
    "-sjis-terminal         always output to stdout and stderr by CP932",
#endif
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};
#endif /* PDVITYPE */

#ifdef PKTOGF
const_string PKTOGFHELP[] = {
    "Usage: pktogf [OPTION]... PKNAME [GFFILE]",
    "  Translate the bitmap font PKNAME to GFFILE.",
    "  Default GFFILE is basename of PKNAME extended with `gf'.",
    "",
    "-help       display this help and exit",
    "-verbose    display progress reports",
    "-version    output version information and exit",
    NULL
};
#endif /* PKTOGF */

#ifdef PKTYPE
const_string PKTYPEHELP[] = {
    "Usage: pktype [OPTION]... PKNAME",
    "  Verify and translate the bitmap font PKNAME to human-readable form,",
    "  written to standard output.",
    "",
    "-help       display this help and exit",
    "-version    output version information and exit",
    NULL
};
#endif /* PKTYPE */

#ifdef PLTOTF
const_string PLTOTFHELP[] = {
    "Usage: pltotf [OPTION]... PLFILE[.pl] [TFMFILE[.tfm]]",
    "  Translate the property list PLFILE to TFMFILE.",
    "  Default TFMFILE is basename of PLFILE extended with `.tfm'.",
    "",
    "-help       display this help and exit",
    "-verbose    display progress reports",
    "-version    output version information and exit",
    NULL
};
#endif /* PLTOTF */

#ifdef POOLTYPE
const_string POOLTYPEHELP[] = {
    "Usage: pooltype [OPTION]... POOLFILE[.pool]",
    "  Display the string number of each string in POOLFILE.",
    "",
    "-help       display this help and exit",
    "-version    output version information and exit",
    NULL
};
#endif /* POOLTYPE */

#ifdef PPLTOTF
const_string based_prog_name = "PLtoTF";
const_string PPLTOTFHELP[] = {
    "Usage: ppltotf [OPTION]... PLFILE[.pl] [TFMFILE[.tfm]]",
    "  Translate the property list PLFILE to TFMFILE.",
    "  Default TFMFILE is basename of PLFILE extended with `.tfm'.",
    "",
    "-kanji=STRING          set Japanese encoding (STRING=euc|jis|sjis|utf8)",
#if defined(WIN32)
    "-sjis-terminal         always output to stdout and stderr by CP932",
#endif
    "-help                  print this message and exit.",
    "-verbose               output progress reports.",
    "-version               print version information and exit.",
    NULL
};
#endif /* PPLTOTF */

#ifdef PTFTOPL
const_string based_prog_name = "TFtoPL";
const_string PTFTOPLHELP[] = {
    "Usage: ptftopl [OPTION]... TFMNAME[.tfm] [PLFILE[.pl]]",
    "  Translate the font metrics TFMNAME to human-readable property list file",
    "  PLFILE or standard output.",
    "",
    "-charcode-format=TYPE  output character codes according to TYPE,",
    "                        either `octal' or `ascii'; default is ascii for",
    "                        letters and digits, octal for all else",
    "-kanji=STRING          set Japanese encoding (STRING=euc|jis|sjis|utf8)",
#if defined(WIN32)
    "-sjis-terminal         always output to stdout and stderr by CP932",
#endif
    "-help                  display this help and exit",
    "-verbose               display progress reports",
    "-version               output version information and exit",
    NULL
};
#endif /* PTFTOPL */

#if defined (TANGLE) || defined (TANGLEBOOT)
const_string TANGLEHELP[] = {
    "Usage: tangle [OPTION]... WEBFILE[.web] [CHANGEFILE[.ch]]",
    "  Tangle WEBFILE with CHANGEFILE into a Pascal program.",
    "  Default CHANGEFILE is " DEV_NULL ";",
    "  Pascal output goes to the basename of WEBFILE extended with `.p',",
    "  and a string pool file, if necessary, to the same extended with `.pool'.",
    "",
    "-length=NUMBER the first NUMBER characters of an identifier have to be",
    "                unique (default 32)",
    "-loose         honor the upper/lower/mixedcase and underline options when",
    "                comparing identifiers (default)",
    "-lowercase     make all identifiers lowercase",
    "-mixedcase     retain the case of identifiers unchanged (default)",
    "-strict        always smash case and remove underlines when comparing",
    "                identifiers",
    "-underline     do not remove underline characters from indentifiers",
    "-uppercase     make all identifiers uppercase",
    "-help          display this help and exit",
    "-version       output version information and exit",
    NULL
};
#endif /* TANGLE */

#ifdef TFTOPL
const_string TFTOPLHELP[] = {
    "Usage: tftopl [OPTION]... TFMNAME[.tfm] [PLFILE[.pl]]",
    "  Translate the font metrics TFMNAME to human-readable property list file",
    "  PLFILE or standard output.",
    "",
    "-charcode-format=TYPE  output character codes according to TYPE,",
    "                        either `octal' or `ascii'; default is ascii for",
    "                        letters and digits, octal for all else",
    "-help                  display this help and exit",
    "-verbose               display progress reports",
    "-version               output version information and exit",
    NULL
};
#endif /* TFTOPL */

#ifdef UPBIBTEX
const_string based_prog_name = "BibTeX";
const_string UPBIBTEXHELP[] = {
    "Usage: upbibtex [OPTION]... AUXFILE[.aux]",
    "  Write bibliography for entries in AUXFILE to AUXFILE.bbl,",
    "  along with a log file AUXFILE.blg."
    "",
    "-kanji=STRING          set Japanese encoding (STRING=euc|jis|sjis|utf8|uptex)",
    "-kanji-internal=STRING set Japanese internal encoding (STRING=euc|uptex)",
    "-min-crossrefs=NUMBER  include item after NUMBER cross-refs; default 2",
#if defined(WIN32)
    "-sjis-terminal         always output to stdout and stderr by CP932",
#endif
    "-terse                 do not print progress reports",
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};
#endif /* UPBIBTEX */

#ifdef UPDVITYPE
const_string based_prog_name = "DVItype";
const_string UPDVITYPEHELP[] = {
    "Usage: updvitype [OPTION]... DVIFILE[.dvi]",
    "  Verify and translate DVIFILE to human-readable form,",
    "  written to standard output.",
    "",
    "-dpi=REAL              set resolution to REAL pixels per inch; default 300.0",
    "-kanji=STRING          set Japanese encoding (STRING=euc|jis|sjis|utf8|uptex)",
    "-magnification=NUMBER  override existing magnification with NUMBER",
    "-max-pages=NUMBER      process NUMBER pages; default one million",
    "-output-level=NUMBER   verbosity level, from 0 to 4; default 4",
    "-page-start=PAGE-SPEC  start at PAGE-SPEC, for example `2' or `5.*.-2'",
    "-show-opcodes          show numeric opcodes (in decimal)",
#if defined(WIN32)
    "-sjis-terminal         always output to stdout and stderr by CP932",
#endif
    "-help                  display this help and exit",
    "-version               output version information and exit",
    NULL
};
#endif /* UPDVITYPE */

#ifdef UPPLTOTF
const_string based_prog_name = "PLtoTF";
const_string UPPLTOTFHELP[] = {
    "Usage: uppltotf [OPTION]... PLFILE[.pl] [TFMFILE[.tfm]]",
    "  Translate the property list PLFILE to TFMFILE.",
    "  Default TFMFILE is basename of PLFILE extended with `.tfm'.",
    "",
    "-kanji=STRING          set Japanese encoding (STRING=euc|jis|sjis|utf8|uptex)",
#if defined(WIN32)
    "-sjis-terminal         always output to stdout and stderr by CP932",
#endif
    "-help                  print this message and exit.",
    "-verbose               output progress reports.",
    "-version               print version information and exit.",
    NULL
};
#endif /* UPPLTOTF */

#ifdef UPTFTOPL
const_string based_prog_name = "TFtoPL";
const_string UPTFTOPLHELP[] = {
    "Usage: uptftopl [OPTION]... TFMNAME[.tfm] [PLFILE[.pl]]",
    "  Translate the font metrics TFMNAME to human-readable property list file",
    "  PLFILE or standard output.",
    "",
    "-charcode-format=TYPE  output character codes according to TYPE,",
    "                        either `octal' or `ascii'; default is ascii for",
    "                        letters and digits, octal for all else",
    "-kanji=STRING          set Japanese encoding (STRING=euc|jis|sjis|utf8|uptex)",
#if defined(WIN32)
    "-sjis-terminal         always output to stdout and stderr by CP932",
#endif
    "-help                  display this help and exit",
    "-verbose               display progress reports",
    "-version               output version information and exit",
    NULL
};
#endif /* UPTFTOPL */

#ifdef VFTOVP
const_string VFTOVPHELP[] = {
    "Usage: vftovp [OPTION]... VFNAME[.vf] [TFMNAME[.tfm] [VPLFILE[.vpl]]]",
    "  Translate VFNAME and companion TFMNAME to human-readable",
    "  virtual property list file VPLFILE or standard output.",
    "  If TFMNAME is not specified, VFNAME (with `.vf' removed) is used.",
    "",
    "-charcode-format=TYPE  output character codes according to TYPE,",
    "                        either `octal' or `ascii'; default is ascii for",
    "                        letters and digits, octal for all else",
    "-help                  display this help and exit",
    "-verbose               display progress reports",
    "-version               output version information and exit",
    NULL
};
#endif /* VFTOVP */

#ifdef VPTOVF
const_string VPTOVFHELP[] = {
    "Usage: vptovf [OPTION]... VPLFILE[.vpl] [VFFILE[.vf] [TFMFILE[.tfm]]]",
    "  Translate VPLFILE to VFFILE and companion TFMFILE.",
    "  Default VFFILE is basename of VPLFILE extended with `.vf'.",
    "  Default TFMFILE is basename of VFFILE extended with `.tfm'.",
    "",
    "-help                  display this help and exit",
    "-verbose               display progress reports",
    "-version               output version information and exit",
    NULL
};
#endif /* VPTOVF */

#ifdef WEAVE
const_string WEAVEHELP[] = {
    "Usage: weave [OPTION]... WEBFILE[.web] [CHANGEFILE[.ch]]",
    "  Weave WEBFILE with CHANGEFILE into a TeX document.",
    "  Default CHANGEFILE is " DEV_NULL ";",
    "  TeX output goes to the basename of WEBFILE extended with `.tex'.",
    "",
    "-x          omit cross-reference information",
    "-help       display this help and exit",
    "-version    output version information and exit",
    NULL
};
#endif /* WEAVE */

#endif /* not HELP_H */
