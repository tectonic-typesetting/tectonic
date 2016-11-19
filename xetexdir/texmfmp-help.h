/* The help messages for TeX & MF family of programs.

Copyright 1995, 1996, 2008-2016 Karl Berry.
Copyright 2001-05 Olaf Weber.

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2, or (at your option)
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.  */

#ifndef TEXMFMP_HELP
#define TEXMFMP_HELP

#ifdef Aleph
const_string ALEPHHELP[] = {
    "Usage: aleph [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: aleph [OPTION]... \\FIRST-LINE",
    "   or: aleph [OPTION]... &FMT ARGS",
    "  Run Aleph on TEXNAME, usually creating TEXNAME.dvi.",
    "  Any remaining COMMANDS are processed as Aleph input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `aleph'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of Aleph input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-etex                   enable e-TeX extensions",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-fmt=FMTNAME            use FMTNAME instead of program name or a %& line",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be inialeph, for dumping formats; this is implicitly",
    "                          true if the program name is `inialeph'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
#ifdef IPC
    "-ipc                    send DVI output to a socket as well as the usual",
    "                          output file",
    "-ipc-start              as -ipc, and also start the server at the other end",
#endif /* IPC */
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to the",
    "                          bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-output-comment=STRING  use STRING for DVI file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder (always on)",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
    "-src-specials           insert source specials into the DVI file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the DVI file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    NULL
};
#endif /* Aleph */

#ifdef epTeX
const_string based_prog_name = "e-TeX";
const_string EPTEXHELP[] = {
    "Usage: eptex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: eptex [OPTION]... \\FIRST-LINE",
    "   or: eptex [OPTION]... &FMT ARGS",
    "  Run e-pTeX on TEXNAME, usually creating TEXNAME.dvi.",
    "  Any remaining COMMANDS are processed as e-pTeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `eptex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of e-pTeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-etex                   enable e-TeX extensions",
    "-fmt=NAME               use NAME instead of program name or %&format.",
#if defined(WIN32)
    "[-no]-guess-input-enc   disable/enable to guess input file encoding",
#endif
    "-halt-on-error          stop processing at the first error",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-ini                    be iniptex.",
    "-interaction=STRING     set interaction mode (STRING=batchmode|nonstopmode|",
    "                          scrollmode|errorstopmode)",
#ifdef IPC
    "-ipc                    send DVI output to a socket as well as the usual",
    "                          output file",
    "-ipc-start              as -ipc, and also start the server at the other end",
#endif /* IPC */
    "-jobname=STRING         set the job name to STRING",
    "-kanji=STRING           set Japanese encoding (STRING=euc|jis|sjis|utf8)",
    "-kanji-internal=STRING  set Japanese internal encoding (STRING=euc|sjis)",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for DVI file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
#if defined(WIN32)
    "-sjis-terminal          always output to stdout and stderr by CP932",
#endif
    "-src-specials           insert source specials into the DVI file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the DVI file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
#if defined(__SyncTeX__)
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
#endif
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-help                   print this message and exit.",
    "-version                print version information and exit.",
    NULL
};
#endif /* epTeX */

#ifdef eTeX
const_string ETEXHELP[] = {
    "Usage: etex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: etex [OPTION]... \\FIRST-LINE",
    "   or: etex [OPTION]... &FMT ARGS",
    "  Run e-TeX on TEXNAME, usually creating TEXNAME.dvi.",
    "  Any remaining COMMANDS are processed as e-TeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `etex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of e-TeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-enc                    enable encTeX extensions such as \\mubyte",
    "-etex                   enable e-TeX extensions",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-fmt=FMTNAME            use FMTNAME instead of program name or a %& line",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be einitex, for dumping formats; this is implicitly",
    "                          true if the program name is `einitex'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
#ifdef IPC
    "-ipc                    send DVI output to a socket as well as the usual",
    "                          output file",
    "-ipc-start              as -ipc, and also start the server at the other end",
#endif /* IPC */
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for DVI file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
    "-src-specials           insert source specials into the DVI file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the DVI file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
#if defined(__SyncTeX__)
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
#endif
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-8bit                   make all characters printable by default",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    NULL
};
#endif /* eTeX */

#ifdef eupTeX
const_string based_prog_name = "e-TeX";
const_string EUPTEXHELP[] = {
    "Usage: euptex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: euptex [OPTION]... \\FIRST-LINE",
    "   or: euptex [OPTION]... &FMT ARGS",
    "  Run e-upTeX on TEXNAME, usually creating TEXNAME.dvi.",
    "  Any remaining COMMANDS are processed as e-upTeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `euptex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of e-upTeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-etex                   enable e-TeX extensions",
    "-fmt=NAME               use NAME instead of program name or %&format.",
#if defined(WIN32)
    "[-no]-guess-input-enc   disable/enable to guess input file encoding",
#endif
    "-halt-on-error          stop processing at the first error",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-ini                    be iniptex.",
    "-interaction=STRING     set interaction mode (STRING=batchmode|nonstopmode|",
    "                          scrollmode|errorstopmode)",
#ifdef IPC
    "-ipc                    send DVI output to a socket as well as the usual",
    "                          output file",
    "-ipc-start              as -ipc, and also start the server at the other end",
#endif /* IPC */
    "-jobname=STRING         set the job name to STRING",
    "-kanji=STRING           set Japanese encoding (STRING=euc|jis|sjis|utf8|uptex)",
    "-kanji-internal=STRING  set Japanese internal encoding (STRING=euc|sjis|uptex)",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for DVI file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
#if defined(WIN32)
    "-sjis-terminal          always output to stdout and stderr by CP932",
#endif
    "-src-specials           insert source specials into the DVI file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the DVI file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
#if defined(__SyncTeX__)
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
#endif
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-help                   print this message and exit.",
    "-version                print version information and exit.",
    NULL
};
#endif /* eupTeX */

#ifdef onlyMF
const_string MFHELP[] = {
    "Usage: mf [OPTION]... [MFNAME[.mf]] [COMMANDS]",
    "   or: mf [OPTION]... \\FIRST-LINE",
    "   or: mf [OPTION]... &BASE ARGS",
    "  Run Metafont on MFNAME, usually creating MFNAME.tfm and MFNAME.NNNNgf,",
    "  where NNNN is the resolution of the specified mode (2602 by default).",
    "  Any following COMMANDS are processed as Metafont input,",
    "  after MFNAME is read.",
    "  If the first line of MFNAME is %&BASE, and BASE is an existing .base file,",
    "  use it.  Else use `NAME.base', where NAME is the program invocation name,",
    "  most commonly `mf'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of Metafont input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the BASE to read, overriding all else. Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-base=BASENAME          use BASENAME instead of program name or a %& line",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be inimf, for dumping bases; this is implicitly",
    "                          true if the program name is `inimf'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=mf)",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and base) name to STRING",
    "-recorder               enable filename recorder",
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-8bit                   make all characters printable by default",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    NULL
};
#endif /* onlyMF */

#ifdef MFLua
const_string MFLUAHELP[] = {
    "Usage: mflua [OPTION]... [MFNAME[.mf]] [COMMANDS]",
    "   or: mflua [OPTION]... \\FIRST-LINE",
    "   or: mflua [OPTION]... &BASE ARGS",
    "  Run MFLua on MFNAME, usually creating MFNAME.tfm and MFNAME.NNNNgf,",
    "  where NNNN is the resolution of the specified mode (2602 by default).",
    "  Any following COMMANDS are processed as Metafont input,",
    "  after MFNAME is read.",
    "  If the first line of MFNAME is %&BASE, and BASE is an existing .base file,",
    "  use it.  Else use `NAME.base', where NAME is the program invocation name,",
    "  most commonly `mf'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of Metafont input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the BASE to read, overriding all else. Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-base=BASENAME          use BASENAME instead of program name or a %& line",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be inimf, for dumping bases; this is implicitly",
    "                          true if the program name is `inimf'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=mflua)",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and base) name to STRING",
    "-recorder               enable filename recorder",
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-8bit                   make all characters printable by default",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    NULL
};
#endif /* MFLua */

#ifdef MFLuaJIT
const_string MFLUAJITHELP[] = {
    "Usage: mfluajit [OPTION]... [MFNAME[.mf]] [COMMANDS]",
    "   or: mfluajit [OPTION]... \\FIRST-LINE",
    "   or: mfluajit [OPTION]... &BASE ARGS",
    "  Run MFLuaJIT on MFNAME, usually creating MFNAME.tfm and MFNAME.NNNNgf,",
    "  where NNNN is the resolution of the specified mode (2602 by default).",
    "  Any following COMMANDS are processed as Metafont input,",
    "  after MFNAME is read.",
    "  If the first line of MFNAME is %&BASE, and BASE is an existing .base file,",
    "  use it.  Else use `NAME.base', where NAME is the program invocation name,",
    "  most commonly `mf'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of Metafont input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the BASE to read, overriding all else. Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-base=BASENAME          use BASENAME instead of program name or a %& line",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be inimf, for dumping bases; this is implicitly",
    "                          true if the program name is `inimf'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=mflua)",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and base) name to STRING",
    "-recorder               enable filename recorder",
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-8bit                   make all characters printable by default",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    NULL
};
#endif /* MFLuaJIT */

#ifdef pdfTeX
const_string PDFTEXHELP[] = {
    "Usage: pdftex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: pdftex [OPTION]... \\FIRST-LINE",
    "   or: pdftex [OPTION]... &FMT ARGS",
    "  Run pdfTeX on TEXNAME, usually creating TEXNAME.pdf.",
    "  Any remaining COMMANDS are processed as pdfTeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `pdftex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of pdfTeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-draftmode              switch on draft mode (generates no output PDF)",
    "-enc                    enable encTeX extensions such as \\mubyte",
    "-etex                   enable e-TeX extensions",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-fmt=FMTNAME            use FMTNAME instead of program name or a %& line",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be pdfinitex, for dumping formats; this is implicitly",
    "                          true if the program name is `pdfinitex'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
#ifdef IPC
    "-ipc                    send DVI output to a socket as well as the usual",
    "                          output file",
    "-ipc-start              as -ipc, and also start the server at the other end",
#endif /* IPC */
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm/pk)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for DVI file comment instead of date",
    "                          (no effect for PDF)",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "-output-format=FORMAT   use FORMAT for job output; FORMAT is `dvi' or `pdf'",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
    "-src-specials           insert source specials into the DVI file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the DVI file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
#if defined(__SyncTeX__)
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
#endif
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-8bit                   make all characters printable by default",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    "",
    "pdfTeX home page: <http://pdftex.org>",
    NULL
};
#endif /* pdfTeX */

#ifdef pTeX
const_string based_prog_name = "TeX";
const_string PTEXHELP[] = {
    "Usage: ptex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: ptex [OPTION]... \\FIRST-LINE",
    "   or: ptex [OPTION]... &FMT ARGS",
    "  Run pTeX on TEXNAME, usually creating TEXNAME.dvi.",
    "  Any remaining COMMANDS are processed as pTeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `ptex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of pTeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-fmt=NAME               use NAME instead of program name or %&format.",
#if defined(WIN32)
    "[-no]-guess-input-enc   disable/enable to guess input file encoding",
#endif
    "-halt-on-error          stop processing at the first error",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-ini                    be iniptex.",
    "-interaction=STRING     set interaction mode (STRING=batchmode|nonstopmode|",
    "                          scrollmode|errorstopmode)",
#ifdef IPC
    "-ipc                    send DVI output to a socket as well as the usual",
    "                          output file",
    "-ipc-start              as -ipc, and also start the server at the other end",
#endif /* IPC */
    "-jobname=STRING         set the job name to STRING",
    "-kanji=STRING           set Japanese encoding (STRING=euc|jis|sjis|utf8)",
    "-kanji-internal=STRING  set Japanese internal encoding (STRING=euc|sjis)",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for DVI file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
#if defined(WIN32)
    "-sjis-terminal          always output to stdout and stderr by CP932",
#endif
    "-src-specials           insert source specials into the DVI file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the DVI file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
#if defined(__SyncTeX__)
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
#endif
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-help                   print this message and exit.",
    "-version                print version information and exit.",
    NULL
};
#endif /* pTeX */

#ifdef onlyTeX
const_string TEXHELP[] = {
    "Usage: tex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: tex [OPTION]... \\FIRST-LINE",
    "   or: tex [OPTION]... &FMT ARGS",
    "  Run TeX on TEXNAME, usually creating TEXNAME.dvi.",
    "  Any remaining COMMANDS are processed as TeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `tex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of TeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-enc                    enable encTeX extensions such as \\mubyte",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-fmt=FMTNAME            use FMTNAME instead of program name or a %& line",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be initex, for dumping formats; this is implicitly",
    "                          true if the program name is `initex'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
#ifdef IPC
    "-ipc                    send DVI output to a socket as well as the usual",
    "                          output file",
    "-ipc-start              as -ipc, and also start the server at the other end",
#endif /* IPC */
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for DVI file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
    "-src-specials           insert source specials into the DVI file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the DVI file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
#if defined(__SyncTeX__)
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
#endif
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-8bit                   make all characters printable by default",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    NULL
};
#endif /* onlyTeX */

#ifdef upTeX
const_string based_prog_name = "TeX";
const_string UPTEXHELP[] = {
    "Usage: uptex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: uptex [OPTION]... \\FIRST-LINE",
    "   or: uptex [OPTION]... &FMT ARGS",
    "  Run upTeX on TEXNAME, usually creating TEXNAME.dvi.",
    "  Any remaining COMMANDS are processed as upTeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `uptex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of upTeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-fmt=NAME               use NAME instead of program name or %&format.",
#if defined(WIN32)
    "[-no]-guess-input-enc   disable/enable to guess input file encoding",
#endif
    "-halt-on-error          stop processing at the first error",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-ini                    be iniptex.",
    "-interaction=STRING     set interaction mode (STRING=batchmode|nonstopmode|",
    "                          scrollmode|errorstopmode)",
#ifdef IPC
    "-ipc                    send DVI output to a socket as well as the usual",
    "                          output file",
    "-ipc-start              as -ipc, and also start the server at the other end",
#endif /* IPC */
    "-jobname=STRING         set the job name to STRING",
    "-kanji=STRING           set Japanese encoding (STRING=euc|jis|sjis|utf8|uptex)",
    "-kanji-internal=STRING  set Japanese internal encoding (STRING=euc|sjis|uptex)",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for DVI file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
#if defined(WIN32)
    "-sjis-terminal          always output to stdout and stderr by CP932",
#endif
    "-src-specials           insert source specials into the DVI file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the DVI file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
#if defined(__SyncTeX__)
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
#endif
    "-translate-file=TCXNAME use the TCX file TCXNAME",
    "-help                   print this message and exit.",
    "-version                print version information and exit.",
    NULL
};
#endif /* upTeX */

#ifdef XeTeX
const_string XETEXHELP[] = {
    "Usage: xetex [OPTION]... [TEXNAME[.tex]] [COMMANDS]",
    "   or: xetex [OPTION]... \\FIRST-LINE",
    "   or: xetex [OPTION]... &FMT ARGS",
    "  Run XeTeX on TEXNAME, usually creating TEXNAME.pdf.",
    "  Any remaining COMMANDS are processed as XeTeX input, after TEXNAME is read.",
    "  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,",
    "  use it.  Else use `NAME.fmt', where NAME is the program invocation name,",
    "  most commonly `xetex'.",
    "",
    "  Alternatively, if the first non-option argument begins with a backslash,",
    "  interpret all non-option arguments as a line of XeTeX input.",
    "",
    "  Alternatively, if the first non-option argument begins with a &, the",
    "  next word is taken as the FMT to read, overriding all else.  Any",
    "  remaining arguments are processed as above.",
    "",
    "  If no arguments or options are specified, prompt for input.",
    "",
    "-etex                   enable e-TeX extensions",
    "[-no]-file-line-error   disable/enable file:line:error style messages",
    "-fmt=FMTNAME            use FMTNAME instead of program name or a %& line",
    "-halt-on-error          stop processing at the first error",
    "-ini                    be xeinitex, for dumping formats; this is implicitly",
    "                          true if the program name is `xeinitex'",
    "-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/",
    "                          scrollmode/errorstopmode)",
    "-jobname=STRING         set the job name to STRING",
    "-kpathsea-debug=NUMBER  set path searching debugging flags according to",
    "                          the bits of NUMBER",
    "[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm)",
    "-mltex                  enable MLTeX extensions such as \\charsubdef",
    "-output-comment=STRING  use STRING for XDV file comment instead of date",
    "-output-directory=DIR   use existing DIR as the directory to write files in",
    "-output-driver=CMD      use CMD as the XDV-to-PDF driver instead of xdvipdfmx",
    "-no-pdf                 generate XDV (extended DVI) output rather than PDF",
    "[-no]-parse-first-line  disable/enable parsing of first line of input file",
    "-papersize=STRING       set PDF media size to STRING",
    "-progname=STRING        set program (and fmt) name to STRING",
    "-recorder               enable filename recorder",
    "[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}",
    "-shell-restricted       enable restricted \\write18",
    "-src-specials           insert source specials into the XDV file",
    "-src-specials=WHERE     insert source specials in certain places of",
    "                          the XDV file. WHERE is a comma-separated value",
    "                          list: cr display hbox math par parend vbox",
#if defined(__SyncTeX__)
    "-synctex=NUMBER         generate SyncTeX data for previewers if nonzero",
#endif
    "-translate-file=TCXNAME (ignored)",
    "-8bit                   make all characters printable, don't use ^^X sequences",
    "-help                   display this help and exit",
    "-version                output version information and exit",
    NULL
};
#endif /* XeTeX */
#endif /* TEXMFMP_HELP */
