%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% AASTeX.cls                                    %%
%% March 2, 2016                                 %%
%% Copyright 2016 American Astronomical Society  %%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\def\currversion{6.0}

%%
%%    These files are distributed
%%    WITHOUT ANY WARRANTY; without even the implied warranty of
%%    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
%% 
%% \CharacterTable
%%  {Upper-case    \A\B\C\D\E\F\G\H\I\J\K\L\M\N\O\P\Q\R\S\T\U\V\W\X\Y\Z
%%   Lower-case    \a\b\c\d\e\f\g\h\i\j\k\l\m\n\o\p\q\r\s\t\u\v\w\x\y\z
%%   Digits        \0\1\2\3\4\5\6\7\8\9
%%   Exclamation   \!     Double quote  \"     Hash (number) \#
%%   Dollar        \$     Percent       \%     Ampersand     \&
%%   Acute accent  \'     Left paren    \(     Right paren   \)
%%   Asterisk      \*     Plus          \+     Comma         \,
%%   Minus         \-     Point         \.     Solidus       \/
%%   Colon         \:     Semicolon     \;     Less than     \<
%%   Equals        \=     Greater than  \>     Question mark \?
%%   Commercial at \@     Left bracket  \[     Backslash     \\
%%   Right bracket \]     Circumflex    \^     Underscore    \_
%%   Grave accent  \`     Left brace    \{     Vertical bar  \|
%%   Right brace   \}     Tilde         \~}%
%%
%%%     date            = "2016/01/4",
%%%     author          = "Amy Hendrickson (mailto:amyh@texnology.com)" 
%%%                            including 
%%%                       1) code from emulateapj.cls, version 01/23/15(marked below)
%%%                            written by Alexey Vikhlinin, which relies on RevTeX4-1
%%%                            for much functionality;
%%%                       2) some code from aastex (marked below)
%%%                       3) Original work by Amy Hendrickson (marked below) for
%%%                       extensive table tools, including easy decimal entry and splitting
%%%                       tables horizontally into 2 or 3 pieces, for tabular and deluxetable
%%%                       environments; rotate tables; gridlines for use in positioning figures, 
%%%                       track changes.



%%% This work may be distributed and/or modified under the 
%%% conditions of the LaTeX Project Public License, either version 1.3c 
%%% of this license or (at your option) any later version.
%%% The latest version of this license is in
%%%  http://www.latex-project.org/lppl.txt
%%% and version 1.3 or later is part of all distributions of LaTeX 
%%% version 2005/12/01 or later.
%
%%% This work has the LPPL maintenance status `maintained'.
%
%%% The Current Maintainer of this work is Greg Schwarz <greg.schwarz@aas.org> 
%%% This work consists of the file aastex.cls

%%%     copyright       = "Copyright (C) 2016 American Astronomical Society
%%%
%%%                        This work may be distributed and/or modified under the
%%%                        conditions of the LaTeX Project Public License, either version 1.3
%%%                        of this license or (at your option) any later version.
%%%                        The latest version of this license is in
%%%                        http://www.latex-project.org/lppl.txt
%%%                        and version 1.3 or later is part of all distributions of LaTeX
%%%                        version 2003/12/01 or later.
%%%
%%%                        This work has the LPPL maintenance status "maintained".
%%%
%%%                        The Current Maintainer of this work is the American Astronomical Society.
%%%
%%%                        This work consists of all files listed in the document README.
%%%
%%%     address         = "American Astronomical Society
%%%                        USA",
%%%     telephone       = "+1 ???",
%%%     FAX             = "",
%%%     email           = "aastex-help@aas.org",
%%%     codetable       = "ISO/ASCII",
%%%     keywords        = "latex, AAS, journal",
%%%     supported       = "yes",
%%%     abstract        = "formatter for AAS journal submissions",
%%%  }

\ProvidesClass{aastex}%
 [2016/02/16 1.0/AAS markup document class]%
{}
\ClassInfo{aastex}{%
^^J
^^J
 Original \protect\LaTeX2.09 style
 by Chris Biemesderfer (chris@seagoat.com).
^^J
 Adapted to \protect\LaTeXe\space
 by A. Ogawa (ogawa@teleport.com)%
^^J
emulateapj.cls included, Copyright 2000-2015 Alexey Vikhlinin
^^J
Rewrite and update of emulateapj.cls,
revised and enhanced with table macros, gridlines, draft watermark,^^J
ORCID identifier and more, by Amy Hendrickson,%
^^J
 (amyh@texnology.com, http://www.texnology.com)
^^J
^^J
}%

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% aastex6.cls version 1.0 is a based on emulateapj.cls, version January 23, 2015,
%% and an older version of aastex.cls, with many additional functionalities.
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% Options

% These are the available options:
%   manuscript	: onecolumn, doublespace, 12pt fonts
%   preprint	: onecolumn, single space, 10pt fonts
%   preprint2	: twocolumn, single space, 10pt fonts
%   revtex4 	: will produce an error message and demand to download revtex4-1.
%   twocolumn	: a two column article. Probably not needed, but here just in case.
%   onecolumn	: a one column article; default option.
%   twocolappendix: make 2 column appendix
%   onecolappendix: make 1 column appendix is the default. 
%   astrosymb	: Loads Astrosymb font and define \astrocommands. 
%   tighten	: Makes baselineskip slightly smaller
%   times	: uses times font instead of the default
%   linenumbers	: turn on lineno package.
%   trackchanges : required to see the revision mark up and print output
%   numberedappendix: Needed to label appendix sections A, B, .
%   appendixfloats: Needed. Resets figure and table counters to zero


\newif\ifdoublespace
\newif\if@two@col

 \DeclareOption{manuscript}{\@two@colfalse\doublespacetrue}% single column, double space
 \DeclareOption{preprint}{\@two@colfalse\doublespacefalse}% single column, single space
 \DeclareOption{preprint2}{\@two@coltrue\doublespacefalse}% double column, single space
 \DeclareOption{revtex4}{\typeout{=================^^J^^J!!! The revtex4 option no longer
 allowed!^^J^^J Please download revtex4-1.cls^^J^^J
 Thank-you!^^J^^J=================}\stop}

%% left in from older version, in case it could be useful:
\DeclareOption{twocolumn}{\@two@coltrue}
\DeclareOption{onecolumn}{\@two@colfalse}



\newif\if@two@col@app
\DeclareOption{twocolappendix}{\@two@col@apptrue}
\DeclareOption{onecolappendix}{\@two@col@appfalse}%% this is default

\newlength{\bibbaselineskip}\setlength{\bibbaselineskip}{3.075mm}

 \DeclareOption{astrosymb}{
   \def\astro#1{\leavevmode\hbox{\astro@font#1}}%
   \def\load@astro@font{%
     \font\astro@font=Astrosym at\f@size pt
   }%
   \AtBeginDocument{\load@astro@font}
 }

   \def\apjsecfont{\normalsize}
   \def\secnum@size{\small}
   \def\subsecnum@size{\normalsize}

    \def\AppendixApjSectionMarkInTitle#1{\relax}
\newif\ifnumberedappendix
     \DeclareOption{numberedappendix}{\global\numberedappendixtrue
       \def\AppendixApjSectionMarkInTitle#1{#1.\ }
     }

     \newif\if@number@appendix@floats\@number@appendix@floatsfalse
     \DeclareOption{appendixfloats}{
       \@number@appendix@floatstrue
     }

%%%%%%%%%%%%%
%% AH

%% This option changes the baselineskips in the definitions of \normalsize etc
%% to make them a bit smaller.
\newif\iftighten
\DeclareOption{tighten}{\global\tightentrue}

%% AH
\newif\iftimes
\DeclareOption{times}{\global\timestrue}

\newif\ifnumlines
\DeclareOption{linenumbers}{\global\numlinestrue}

\newif\iftrack
\DeclareOption{trackchanges}{\global\tracktrue}

%% End AH Options

\ExecuteOptions{onecolumn,onecolappendix,numberedappendix}% 

\ProcessOptions	

%%%%%%%%%%%%%

 \iftimes
%% would be better to use apjfonts, from Alexey Vikhlinin's website
%% http://hea-www.harvard.edu/~alexey/emulateapj/apjfonts.sty
%%\usepackage{apjfonts}
%% but times.sty is ubiquitous, and many people may not have apjfonts.sty
%% or the fonts that it calls.
%% default fonts:
\usepackage{times}
 \fi

%%%%%%%%%%%%%
%% Line numbering
\usepackage{lineno}

%%%%%%%%%%%%%

 \IfFileExists{revtex4-1.cls}{
    \def\@revtex@cls{revtex4-1}
  }{
\typeout{^^J^^J Please update your system to include
revtex4-1.cls^^J^^J}\stop
  }

\def\@revtex@cls{revtex4-1}
\let\@startsection@from@latex=\@startsection

\if@two@col
  \LoadClass[twocolumn,nofootinbib,showkeys,twoside,floatfix,unsortedaddress,flushbottom,10pt,aps,pra]
{\@revtex@cls}
\else
  \LoadClass[nofootinbib,showkeys,twoside,floatfix,unsortedaddress,flushbottom,notitlepage,10pt,aps,pra]
{\@revtex@cls}
\fi

\def\blankaffiliation{~}%

%% needed?
 \def\@eapj@cap@font{\bfseries}
 \def\@eapj@figname{Figure}
 \def\@eapj@tabname{Table}

% Restore natbib package without sorting. 
\def\NAT@sort{0}
\def\NAT@cmprs{0}
% only redefine sort@cites for natbib versions < 2009
\@ifundefined{NAT@sort@cites@}{\def\NAT@sort@cites#1{\edef\NAT@cite@list{#1}}}%

\RequirePackage{latexsym}% 
\RequirePackage{graphicx}% 
\RequirePackage{amssymb}% 

\newcommand{\bibstyle@aas}{\bibpunct{(}{)}{;}{a}{}{,}}% 
\@citestyle{aas}% 

\let\@startsection=\@startsection@from@latex
\def\baselinestretch{1.0}

%% a bit more vertical space between horizontal lines, makes table more readable
\AtBeginDocument{\def\arraystretch{1.2}}

%% needed?
\def\raggedcolumn@sw#1#2{#2}  % implement flushbottom as it was in revtex4

\RequirePackage{epsf,graphicx}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Font families
%% modification of bk10.clo, standard LaTeX distribution
%%
%% Implements doublespace used in `manuscript' option
%% Implements `tighten' option: make baselineskip smaller if `tighten' is used
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\renewcommand\normalsize{%
\ifdoublespace
\iftighten
   \@setfontsize\normalsize\@xiipt{24}%
\else
   \@setfontsize\normalsize\@xiipt{28}%
\fi
\else % not doublespace
\iftighten
   \@setfontsize\normalsize\@xpt{11.5}%
\else
   \@setfontsize\normalsize\@xpt{12.5}%
\fi
\fi%% end doublespace branch
   \abovedisplayskip 2.2mm \@plus2\p@ \@minus1\p@
   \abovedisplayshortskip \z@ \@plus3\p@
   \belowdisplayshortskip 2.2mm \@plus2\p@ \@minus1\p@
   \belowdisplayskip \abovedisplayskip
   \let\@listi\@listI}

\normalsize

\renewcommand\small{%
\ifdoublespace
\iftighten
   \@setfontsize\small\@xipt{20}%
\else
   \@setfontsize\small\@xipt{22}%
\fi
\else % not doublespace
\iftighten
   \@setfontsize\small\@ixpt{8.68}%
\else
   \@setfontsize\small\@ixpt{9.68}%
\fi
\fi%% end doublespace branch
   \abovedisplayskip 8.5\p@ \@plus3\p@ \@minus4\p@
   \abovedisplayshortskip \z@ \@plus2\p@
   \belowdisplayshortskip 4\p@ \@plus2\p@ \@minus2\p@
   \def\@listi{\leftmargin\leftmargini
               \topsep 4\p@ \@plus2\p@ \@minus2\p@
               \parsep 2\p@ \@plus\p@ \@minus\p@
               \itemsep \parsep}%
   \belowdisplayskip \abovedisplayskip
}

\renewcommand\footnotesize{%
\ifdoublespace
\iftighten
   \@setfontsize\footnotesize\@xpt{22}%
\else
   \@setfontsize\footnotesize\@xpt{24}%
\fi
\else % not doublespace
\iftighten
   \@setfontsize\footnotesize\@viiipt{7.36}%
\else
   \@setfontsize\footnotesize\@viiipt{8.36}%
\fi
\fi%% end doublespace branch
   \abovedisplayskip 6\p@ \@plus2\p@ \@minus4\p@
   \abovedisplayshortskip \z@ \@plus\p@
   \belowdisplayshortskip 3\p@ \@plus\p@ \@minus2\p@
   \def\@listi{\leftmargin\leftmargini
               \topsep 3\p@ \@plus\p@ \@minus\p@
               \parsep 2\p@ \@plus\p@ \@minus\p@
               \itemsep \parsep}%
   \belowdisplayskip \abovedisplayskip
}

\ifdoublespace

\renewcommand\scriptsize{\@setfontsize\scriptsize\@viiipt{16}}
\renewcommand\tiny{\@setfontsize\tiny\@vipt{12}}
\renewcommand\large{\@setfontsize\large\@xivpt{24}}
\renewcommand\Large{\@setfontsize\Large\@xviipt{24}}
\renewcommand\LARGE{\@setfontsize\LARGE\@xxpt{30}}
\renewcommand\huge{\@setfontsize\huge\@xxvpt{36}}
\let\Huge=\huge

\else
\renewcommand\scriptsize{\@setfontsize\scriptsize\@viipt\@viiipt}
\renewcommand\tiny{\@setfontsize\tiny\@vpt\@vipt}
\renewcommand\large{\@setfontsize\large\@xiipt{14}}
\renewcommand\Large{\@setfontsize\Large\@xivpt{18}}
\renewcommand\LARGE{\@setfontsize\LARGE\@xviipt{22}}
\renewcommand\huge{\@setfontsize\huge\@xxpt{25}}
\renewcommand\Huge{\@setfontsize\Huge\@xxvpt{30}}
\fi

\setlength\smallskipamount{3\p@ \@plus 1\p@ \@minus 1\p@}
\setlength\medskipamount{6\p@ \@plus 2\p@ \@minus 2\p@}
\setlength\bigskipamount{12\p@ \@plus 4\p@ \@minus 4\p@}
\setlength\headheight{12\p@}
\def\@listi{\leftmargin\leftmargini
            \parsep 4\p@ \@plus2\p@ \@minus\p@
            \topsep 8\p@ \@plus2\p@ \@minus4\p@
            \itemsep4\p@ \@plus2\p@ \@minus\p@}
\let\@listI\@listi
\@listi
\def\@listii {\leftmargin\leftmarginii
              \labelwidth\leftmarginii
              \advance\labelwidth-\labelsep
              \topsep    4\p@ \@plus2\p@ \@minus\p@
              \parsep    2\p@ \@plus\p@  \@minus\p@
              \itemsep   \parsep}
\def\@listiii{\leftmargin\leftmarginiii
              \labelwidth\leftmarginiii
              \advance\labelwidth-\labelsep
              \topsep    2\p@ \@plus\p@\@minus\p@
              \parsep    \z@
              \partopsep \p@ \@plus\z@ \@minus\p@
              \itemsep   \topsep}
\def\@listiv {\leftmargin\leftmarginiv
              \labelwidth\leftmarginiv
              \advance\labelwidth-\labelsep}
\def\@listv  {\leftmargin\leftmarginv
              \labelwidth\leftmarginv
              \advance\labelwidth-\labelsep}
\def\@listvi {\leftmargin\leftmarginvi
              \labelwidth\leftmarginvi
              \advance\labelwidth-\labelsep}

%  ****************************************************
%  *             TWO AND SINGLE ONE COLUMN STYLES: AV *
%  ****************************************************
%

%%%%% include atbeginend.sty by AV:

% Copy \begin and \end to \BeginEnvironment and \EndEnvironment
\let\BeginEnvironment=\begin
\let\EndEnvironment=\end
%% \ifundefined from TeXbook
\def\IfUnDef#1{\expandafter\ifx\csname#1\endcsname\relax}
% Null command needed to eat its argument
\def\NullCom#1{}
\def\begin#1{%
% if defined \BeforeBeg for this environment, execute it
\IfUnDef{BeforeBeg#1}\else\csname BeforeBeg#1\endcsname\fi%
\IfUnDef{AfterBeg#1}% This is done to skip the command for environments
                     % which can take arguments, like multicols; YOU MUST NOT
                     % USE \AfterBegin{...}{...} for such environments! 
        \let\SaveBegEnd=\BeginEnvironment%
\else
        % Start this environment
                \BeginEnvironment{#1}%
        % and execute code after \begin{environment}
                \csname AfterBeg#1\endcsname
        \let\SaveBegEnd=\NullCom
\fi
\SaveBegEnd{#1}
}
\def\end#1{%
% execute code before \end{environment}
\IfUnDef{BeforeEnd#1}\else\csname BeforeEnd#1\endcsname\fi%
% close this environment
\EndEnvironment{#1}
% and execute code after \begin{environment}
\IfUnDef{AfterEnd#1}\else\csname AfterEnd#1\endcsname\fi%
}
\long\def\BeforeBegin#1#2{\expandafter\gdef\csname BeforeBeg#1\endcsname {#2}}
\long\def\BeforeEnd  #1#2{\expandafter\gdef\csname BeforeEnd#1\endcsname {#2}}
\long\def\AfterBegin #1#2{\expandafter\gdef\csname AfterBeg#1\endcsname {#2}}
\long\def\AfterEnd   #1#2{\expandafter\gdef\csname AfterEnd#1\endcsname{#2}}

%%%% end of atbeginend.sty

%  ****************************************
%  *            PAGE LAYOUT               *
%  ****************************************

% Page size, spacing parameters, etc. 

\textwidth=7.1in
\columnsep=0.3125in
\parindent=0.125in
\voffset=-20mm
\hoffset=-7.5mm

\topmargin=0in
\headheight=.15in
\headsep=0.5in
\oddsidemargin=0in
\evensidemargin=0in
\parskip=0cm

%\tolerance=600          % 3x "normal" value; cuts down on overfull complaints

%% AV,MM, to have 64 lines per column, with textheight 25cm:
\textheight=64\baselineskip
\textheight=\baselinestretch\textheight
\ifdim\textheight>25.2cm\textheight=25.0cm\fi

\advance\textheight -36pt

\topskip\baselineskip
\maxdepth\baselineskip


\let\tighten=\relax
\let\tightenlines=\tighten
\let\singlespace=\relax
\let\doublespace=\relax

\def\eqsecnum{
    \@newctr{equation}[section]
    \def\theequation{\hbox{\normalsize\arabic{section}-\arabic{equation}}}}

%% running heads:
\def\lefthead#1{\gdef\@versohead{#1}} \lefthead{\relax}
\def\righthead#1{\gdef\@rectohead{#1}} \righthead{\relax}
\let\shorttitle\lefthead
\let\shortauthors\righthead

\def\ps@apjheads{\let\@mkboth\markboth
     \def\@evenfoot{}
    \def\@evenhead{\lower9mm\hbox to\textwidth{
                     \rm\thepage\hfil \rm\textsc{\@rectohead} \hfil}}}
    \def\@oddfoot{}
    \def\@oddhead{\lower9mm\hbox to\textwidth{
                     \hfil\rm\textsc{\@versohead}\hfil \rm\thepage}}

\pagestyle{apjheads}

\@twosidetrue

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%    TITLE PAGE                                                  %
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\long\def\frontmatter@title@above{
\vspace*{-\headsep}\vspace*{\headheight}
\footnotesize
{\footnotesize\textsc{\@journalinfo}}\par
{\scriptsize Preprint typeset using \LaTeX\ style AASTeX6 v.\ 1.0
}\par\vspace*{-\baselineskip}\vspace*{0.625in}
}%
\def\frontmatter@title@produce{%
 \begingroup
 \frontmatter@title@above
  \frontmatter@title@format
  \@title
\vskip12pt
  \@ifx{\@title@aux\@title@aux@cleared}{}{%
   \expandafter\frontmatter@footnote\expandafter{\@title@aux}%
  }%
  \par
%% \class@warn{Unused short title ignored}%
  \frontmatter@title@below
 \endgroup
}%

\newcount\fncount
\newcount\endfncount
\long\def\tempfootnote#1{\global\advance\fncount by 1\relax%
\expandafter\gdef\csname footnote\the\fncount\endcsname{#1}}

\def\tempfootnoteintitle#1{\hskip-1.5pt$^{\fnsymbol{footnote}}$}

\renewcommand\title[2][]{%
\def\@title{\setbox0=\vbox{\let\footnote\tempfootnote #2}%
{\let\footnote\tempfootnoteintitle\uppercase{#2}}
\loop\advance\endfncount by 1
\expandafter\ifx\csname footnote\the\endfncount\endcsname\relax\else
\footnote{\csname footnote\the\endfncount\endcsname}
\repeat
\ifdoublespace\vskip 3\baselineskip
\fi
}%
 \def\@shorttitle{#1}%
 \let\@AF@join\@title@join
}%

\def\frontmatter@title@format{\normalsize\centering}
\def\frontmatter@title@below{\vspace*{-2.63\baselineskip\vspace*{0.25in}}}%

\def\frontmatter@above@affilgroup{\vspace*{0.25in}}%
\def\frontmatter@authorformat{\small\scshape\centering}%
\def\frontmatter@affiliationfont{\normalfont\footnotesize}%

\def\frontmatter@finalspace{\vspace*{-\baselineskip}\vspace*{0.375in}}

\def\@separator{\\}%

\def\frontmatter@footnote#1{%
 \begingroup
  \@booleanfalse\temp@sw
  \def\@tempa{#1}%
  \let\@tempb\@empty
  \def\@TBN@opr{\y@match\@tempa}%
  \@FMN@list
  \temp@sw{%
   \expandafter\frontmatter@footnotemark
   \expandafter{\@tempb}%
  }{%
    \def\@thefnmark{\relax}
    \frontmatter@footnotetext{#1}
  }%
 \endgroup
}%

\newcounter{front@matter@foot@note}\setcounter{front@matter@foot@note}{0}

\def\frontmatter@above@affiliation{\vspace*{0.5mm}}%
\renewcommand\affiliation[1]{%
 \move@AU\move@AF%
 \begingroup%
  \@affiliation{\hspace*{2mm}\advance\baselineskip2pt #1\vskip1sp}%
}%

\def\affil#1{\expandafter\gdef\csname
  xaffil\the\authorcount\endcsname{#1}
\affiliation{#1}}

\def\affil@mark#1{\textsuperscript{#1}}

\def\affile@mark@pad{0.2em}

% Fight poor revtex algorithm for altaffiliation:
\def\@alt@affil@kill@next@symbol#1{\relax}
\newtoks\alt@affil@toks \alt@affil@toks{\@alt@affil@kill@next@symbol}
\newtoks\alt@affil@toks@count \alt@affil@toks@count{}

\let\@author@finish@revtex=\@author@finish
\def\@author@finish{%
\if!\the\alt@affil@toks@count!%
\relax
\else
\altaffiliation{\the\alt@affil@toks}\the\alt@affil@toks@count
\fi
\@author@finish@revtex
}

\let\old@foot@note@mark=\footnotemark
\let\old@foot@note@text=\footnotetext
\def\footnotemark[#1]{\altaffilmark{#1}}
\def\footnotetext[#1]#2{\altaffiltext{#1}{#2}}

%   ABSTRACT
 \def\frontmatter@abstractfont{\normalsize\parindent=0.0in}%
\def\frontmatter@abstractwidth{6in}
\def\frontmatter@preabstractspace{0.0\baselineskip}
\def\frontmatter@postabstractspace{0.0\baselineskip}

\def\abstractname{ABSTRACT}
\long\def\frontmatter@abstractheading{%
\makeaffils
  \vspace*{-\baselineskip}\vspace*{1.5pt}
%  \centerline{\itshape\footnotesize\@submitted}
  \vspace*{0.13189in}
 \begingroup
  \centering
  \abstractname
  \vskip 1mm
  \par
 \endgroup
 \everypar{\rightskip=0.5in\leftskip=\rightskip}\par
}%

% Redefine the LaTeX \and command for this style. 
\def\and{\ifnum\AuthorCallLimit>0\else\xauthor{\vspace*{-0.2in}and\vspace*{-0.558cm}}\noaffiliation\fi}
\def\@listand{\@ifnum{\@tempcnta=\tw@}{\andname\space}{}}%

%  ****************************************
%  *             KEYWORDS                 *
%  ****************************************


  \def\@keys@name{\textit{Keywords:}\/~\mbox{}}%

\newlength{\keys@width}
\def\frontmatter@keys@format{\vspace*{0.5mm}%
  \settowidth{\keys@width}{\normalsize\@keys@name}%
  \rightskip=0.5in\leftskip=\rightskip\parindent=0pt%
    \hangindent=\keys@width\hangafter=1\normalsize}%
\def\@keywords@produce#1{%
 \showKEYS@sw{%
  \begingroup%
   \frontmatter@keys@format%
   \@keys@name#1\par
  \endgroup
 }{%
  \@if@empty{#1}{}{%
   \class@warn{If you want your keywords to appear in your output, use document class option showkeys}%
  }%
 }%
}%

\let\subjectheadings=\keywords

%  ****************************************
%  *             FOOTNOTES                *
%  ****************************************

% Modify code from revtex:
\def\@xfootnotemark@ltx[#1]{%
 \begingroup
%   \csname c@\@mpfn\endcsname #1\relax
   \unrestored@protected@xdef\@thefnmark{#1}%
 \endgroup
 \H@@footnotemark
}%
% Modify code from LaTeX to fix footnotes:
\def\@xfootnotenext[#1]{%
  \begingroup
%  \csname c@\@mpfn\endcsname #1\relax
  \unrestored@protected@xdef\@thefnmark{#1}%
  \endgroup
  \@footnotetext}



% Footnotes on the last page: user issues \lastpagefootnotes. It catches all
% footnotes and issues them before \begin{refernces} or
% \begin{\thebibliography} or \begin{appendix}, whichever comes first,
% at the end of right column.  
\newcounter{lastfootnote}
\let\orig@footnote=\footnote
%
\def\spit@out@footnotes@{\addtocounter{footnote}{-\c@lastfootnote}\vspace*{-\baselineskip}\vspace*{\skip\footins}\bgroup\footnotesize\lastfootnote\par\egroup\let\footnote=\orig@footnote}
%
\let\spit@out@footnotes\relax
%
\long\def\lastpagefootnotes{%
\raggedbottom% Have to use flushbottom with revtex4-1 but it eats away
             % last page footnotes
\setcounter{lastfootnote}{0}
\long\gdef\lastfootnote{\mbox{}}
\let\spit@out@footnotes\spit@out@footnotes@
\long\def\footnote##1{\refstepcounter{lastfootnote}\footnotemark\g@addto@macro\lastfootnote{\\[\footnotesep]\refstepcounter{footnote}\mbox{}\hspace*{3mm}\textsuperscript{\thefootnote}~##1}}
% We need to mess with footnotes in appendix only if it is in a
% different mode (twocol vs onecol) than the main text
\def\mess@with@appendix@footnotes{
\BeforeBegin{appendix}{\spit@out@footnotes\let\spit@out@footnotes\relax}
\BeforeEnd{appendix}{\spit@out@footnotes\let\spit@out@footnotes\relax}
\AfterEnd{appendix}{\let\footnote=\orig@footnote}
}

\if@two@col
  \if@two@col@app\else
    \mess@with@appendix@footnotes
  \fi
\fi

\BeforeBegin{references}{\spit@out@footnotes\let\spit@out@footnotes\relax}
\BeforeBegin{thebibliography}{\spit@out@footnotes\let\spit@out@footnotes\relax}
% in case \lastpagefootnotes were given inside \begin{appendix}\end{appendix},
\AfterEnd{references}{\let\footnote=\orig@footnote}
\AfterEnd{thebibliography}{\let\footnote=\orig@footnote}
}
\let\lastpagefootnote=\lastpagefootnotes

\ifdoublespace %used for manuscript
\skip\footins 36pt
\footnotesep 12pt
\else
\footnotesep 12pt
\skip\footins 24pt
\fi

\let\footnoterule\relax

\def\@makefntext#1{\mbox{}\hspace*{3mm}\@makefnmark~#1}

\def\notetoeditor#1{}%   % We do not need notes to editor in the preprint
\def\placetable#1{}%   % We do not need notes to editor in the preprint
\def\placefigure#1{}%   % We do not need notes to editor in the preprint

%  ****************************************
%  *             SECTIONS                 *
%  ****************************************

\setcounter{secnumdepth}{3}

\newif\if@firstsection \@firstsectiontrue

\def\section{%
    \if@firstsection\@firstsectionfalse
      \maketitle\setcounter{footnote}{\thefront@matter@foot@note}%
      \let\footnotetext=\old@foot@note@text
      \let\footnotemark=\old@foot@note@mark
    \fi
    \@startsection{section}{1}{\z@}{9pt plus 2pt minus 1pt}{4pt}{\apjsecfont\center}}
  
\def\subsection{\@startsection{subsection}{2}{\z@}{9pt plus 1pt minus 1pt}{4pt}%
  {\normalsize\itshape \center}}

\def\subsubsection{\@startsection{subsubsection}{3}{\z@}%
  {2ex plus 1ex minus .2ex}{1ex plus .2ex}{\small\itshape \center}}

\def\paragraph{\@startsection{paragraph}{4}{\z@}%
  {1.5ex plus 1ex minus .2ex}{-0.5\parindent}{\small\it}}

\def\subparagraph{\@startsection{subparagraph}{5}{\z@}%
  {1ex plus 1ex minus .2ex}{-0.5\parindent}{\small\it}}

\def\thesection{\@arabic{\c@section}}
\def\thesubsection{\thesection.\@arabic{\c@subsection}}
\def\thesubsubsection{\thesubsection.\@arabic{\c@subsubsection}}
\def\theparagraph{\thesubsubsection.\@arabic{\c@paragraph}}
\def\p@section        {}
\def\p@subsection     {}
\def\p@subsubsection  {}
\def\p@paragraph      {}
\def\p@subparagraph   {}


\def\sec@upcase#1{\uppercase{#1}}
\def\subsec@upcase#1{\relax{#1}}

%
% How the section number will appear in the section title - AV
\def\ApjSectionMarkInTitle#1{#1.\ }
\def\ApjSectionpenalty{0}

\def\@sect#1#2#3#4#5#6[#7]#8%
{\@tempskipa #5\relax 
 \ifdim \@tempskipa >\z@ \begingroup
     #6\relax 
  \ifnum #2>\c@secnumdepth \def \@svsec {}\else 
    \refstepcounter{#1} \edef \@svsec {\ApjSectionMarkInTitle 
    {\csname the#1\endcsname}}\fi
  \@hangfrom {\hskip #3\relax 
    \ifnum #2=1{\secnum@size {\rm\@svsec~}}%
    \else {\subsecnum@size {\rm\@svsec~}}\fi }%
  {\interlinepenalty \@M 
   \ifnum #2=1\sec@upcase{#8}%
   \else \subsec@upcase{#8}\fi\par}\endgroup 
  \csname #1mark\endcsname {#7}\addcontentsline{toc}{#1}%
  {\ifnum #2>\c@secnumdepth \else \protect \numberline 
     {\csname the#1\endcsname }\fi #7}%
  \else 
  \ifnum #2>\c@secnumdepth \def \@svsec {}\else 
    \refstepcounter{#1} \edef \@svsec {\ApjSectionMarkInTitle 
    {\csname the#1\endcsname}}\fi
    \def \@svsechd {#6\hskip #3%
    \ifnum #2=1{\secnum@size{\rm\@svsec~}}\else{\subsecnum@size{\rm\@svsec~}}\fi%
    \ifnum #2=1\sec@upcase{#8}\else\subsec@upcase{#8}\fi%
    \ifnum #2=4\hskip 0.4ex{\rm ---}\fi%
     \csname #1mark\endcsname {#7}\addcontentsline{toc}{#1}%
      {\ifnum #2>\c@secnumdepth \else \protect \numberline {\csname
      the#1\endcsname }\fi #7}}\fi \@xsect {#5} \penalty \ApjSectionpenalty
%-refstepcounter is now within a group. So \@currentlabel, which is normally
%-set by \refstepcounter is hidden within a group. Need to set it manually. 
\protected@edef\@currentlabel{\csname p@#1\endcsname\csname the#1\endcsname}
}


\def\@ssect#1#2#3#4#5{\@tempskipa #3\relax
   \ifdim \@tempskipa>\z@
     \begingroup #4\@hangfrom{\hskip #1}{\interlinepenalty \@M
       \sec@upcase{#5}\par}\endgroup
   \else \def\@svsechd{#4\hskip #1\relax \sec@upcase{#5}}\fi
    \@xsect{#3}
% MM:
\penalty \ApjSectionpenalty}


\def\acknowledgments{\vskip 5.8mm plus 1mm minus 1mm}
\let\acknowledgements=\acknowledgments                  % second spelling


%  ****************************************
%  *          APPENDIX                    *
%  ****************************************

\newcounter{remember@figure@num}
\newcounter{remember@table@num}


% this was an environment earlier, which doesn't make sense since we don't
% do \begin{appendix}...\end{appendix}. Changed to \appendix which is how it is used.

%   numberedappendix: Needed to label appendix sections A, B, .
%   appendixfloats: Needed. Resets figure and table counters to zero, also needs letters

\def\appendix{
    \if@two@col@app\else
      \onecolumngrid
    \fi 
% \vrule used for extra space; otherwise revtex4-1 sometimes eats
% away the last line before appendix
        \noindent\mbox{}\vrule height 24pt width0pt\hfill{\apjsecfont APPENDIX}\hfill\mbox{}\par
        \nopagebreak\medskip\@nobreaktrue\def\ApjSectionpenalty{\@M}
        \@firstsectionfalse
          \setcounter{section}{0}
          \setcounter{subsection}{0}
          \setcounter{equation}{0}
%%
\ifnumberedappendix
          \def\thesection{\Alph{section}}
          \def\theequation{\hbox{\Alph{section}\arabic{equation}}}
\setcounter{table}{0}
\setcounter{figure}{0}
\else
%%
\if@number@appendix@floats
\setcounter{table}{0}
\setcounter{figure}{0}
\setcounter{equation}{0}
          \def\thesection{\Alph{section}}
          \def\theequation{\hbox{\Alph{section}\arabic{equation}}}
          \def\section{
\@startsection {section}{1}{\z@} 
            {10pt}{5pt}{\centering\scshape\apjsecfont}}
\else
% Do not use appendix numbers in the titles
          \def\ApjSectionMarkInTitle{\AppendixApjSectionMarkInTitle}
\fi\fi
%%
%
%%  this will work for either lettered or not lettered section heads, since
%% the definition of \thesection will be dependant on conditionals above.
\def\thefigure{\thesection\arabic{figure}}
\def\thetable{\thesection\arabic{table}}}



%  ****************************************
%  *          BIBLIOGRAPHY                *
%  ****************************************

\renewenvironment{references}[0]{
  \onecolumngrid
  \par
  \vspace{10pt plus 3pt}
  \noindent \makebox[\textwidth][c]{\small\scshape REFERENCES}
  \par
  \vspace*{4pt plus 3pt}
  \set@column@hsize{2}\twocolumngrid
  \parindent=0cm \parskip=0cm
  \def\baselinestretch{1.0}
  \footnotesize \baselineskip=\baselinestretch\bibbaselineskip plus 1pt
  minus 1pt \interlinepenalty \@M
  \hyphenpenalty=10000
  \frenchspacing    % AV - to get right spacing after ``et al.'' 
  \def\refpar{\par\hangindent=1.2em\hangafter=1}}
{
  \onecolumngrid
}

% redefine thebibliography
% redefine thebibliography

% remove numbers from the reference list
\def\@biblabel#1{\relax}

\bibliographystyle{aasjournal}
\newdimen\bibindent
\renewenvironment{thebibliography}[1]{  
\onecolumngrid
\vspace{20pt}
    \hbox to\textwidth{\hss\normalsize REFERENCES\hss}
\vspace{4pt}
\twocolumngrid 
 \par
 \raggedright
 \vspace{10pt plus 3pt}
 \par
 \list{}%
   {
     \parindent=0pt \parskip=1pt \parsep=0pt % AV
     \bibindent=0pt                          %
    \footnotesize 
\baselineskip=10pt plus .01pt \interlinepenalty \@M  % AV
     \frenchspacing    % AV - to get right spacing after ``et al.'' 
     \hyphenpenalty=10000
     \itemindent=-1.0em                      %
     \itemsep=0pt                            %
     \listparindent=0pt                      %
     \settowidth\labelwidth{0pt} %
     \labelsep=0pt                           %
     \leftmargin=1.0em
     \advance\leftmargin\labelsep
%%%      \usecounter{enumiv}%
      \let\p@enumiv\@empty
      \renewcommand\theenumiv{\relax}}%
    \sloppy\clubpenalty4000\widowpenalty4000%
    \sfcode`\.\@m}
  {\def\@noitemerr
    {\@latex@warning{Empty `thebibliography' environment}}%
    \endlist
    \onecolumngrid % to balance references
    \if@two@col \twocolumngrid \fi
  }

% %% AV: 

\def\reference{\@ifnextchar\bgroup {\@reference}
        {\@latexerr{Missing key on reference command}
        {Each reference command should have a key corresponding to a markcite somewhere in the text}}}
\def\@reference#1{\refpar}

%% subtitle header and journal info:
\def\submitted#1{\gdef\@submitted{#1}}
\def\journalinfo#1{\gdef\@journalinfo{#1}}

%% AH: modification in definition of subtitle to use aastex instead of emulateapj:
\def\subtitle{
  \vspace*{-12mm}
  \noindent
  {\scriptsize {\sc \@journalinfo} \\
        Preprint typeset using \LaTeX\ style AASTeX 6 v.\ 1.0}
}
\let\slugcomment\submitted   % slugcomment == submitted
\submitted{Draft version \today}
\journalinfo{\@submitted}


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%  Equations                                             %
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% EQNARRAY with reduced spacing around tab characters - AV
\def\eqnarray{%
  \stepcounter{equation}%
  \def\@currentlabel{\p@equation\theequation}%
  \global\@eqnswtrue
  \setlength{\arraycolsep}{0.25\arraycolsep}
  \m@th
  \global\@eqcnt\z@
  \tabskip\@centering
  \let\\\@eqncr
  $$\everycr{}\halign to\displaywidth\bgroup
  \hskip\@centering$\displaystyle\tabskip\z@skip{##}$\@eqnsel
  &\global\@eqcnt\@ne\hskip \tw@\arraycolsep \hfil${##}$\hfil
  &\global\@eqcnt\tw@ \hskip \tw@\arraycolsep
  $\displaystyle{##}$\hfil\tabskip\@centering
  &\global\@eqcnt\thr@@ \hb@xt@\z@\bgroup\hss##\egroup
  \tabskip\z@skip
  \cr
  }

%  ****************************************
%  *         TABLES AND FIGURES           *
%  ****************************************

\def\@arstrut@hline@clnc{0.5\p@}% % Klootch: magic number

\setcounter{topnumber}{7}

\setlength{\belowcaptionskip}{4pt}% 
\setlength{\abovecaptionskip}{5pt}

\setlength{\textfloatsep}{-\baselinestretch\baselineskip}
\setlength{\dbltextfloatsep}{-\baselinestretch\baselineskip}
%\if@apj@letter
%  \addtolength{\textfloatsep}{5mm plus 10mm minus 10mm}
%  \addtolength{\dbltextfloatsep}{5mm plus 10mm minus 10mm}
%\else
  \addtolength{\textfloatsep}{7mm plus 10mm minus 10mm}
  \addtolength{\dbltextfloatsep}{7mm plus 10mm minus 10mm}
%\fi

\renewcommand{\topfraction}{1.0}
\renewcommand{\bottomfraction}{1.0}
\renewcommand{\textfraction}{0.0}
\renewcommand{\dbltopfraction}{0.85}
\renewcommand{\dblfloatpagefraction}{0.85}
\setcounter{topnumber}{9}
\setcounter{bottomnumber}{9}
\setcounter{totalnumber}{20}
\setcounter{dbltopnumber}{9} 

% Copied from revtex4.cls; without it, captions are centered
\def\@xfloat@prep{%
  \let\footnote\footnote@latex
  \def\@mpfn{mpfootnote}%
  \def\thempfn{\thempfootnote}%
  \c@mpfootnote\z@
  \let\@footnotetext\@mpfootnotetext
  \let\H@@footnotetext\@mpfootnotetext
  \let\@makefntext\@mpmakefntext
}%


%%%%%%%%%%%% \cutinhead

%% Number of \pt@ncol set when \tablecolumns{} used at beginning of
%% deluxetable.

\def\@ptabularcr{\\}
\newcommand\cutinhead[1]{% 
 \noalign{\vskip 1.5ex}% 
 \hline 
 \@ptabularcr 
 \noalign{\vskip -1.5ex}% 
 \multicolumn{\pt@ncol}{c}{#1}% 
 \@ptabularcr 
 \noalign{\vskip .8ex}% 
 \hline 
 \@ptabularcr 
 \noalign{\vskip -2ex}% 
}% 

\newcommand\sidehead[1]{% 
 \noalign{\vskip 1.5ex}% 
%  \hline 
%  \@ptabularcr 
%  \noalign{\vskip -1.5ex}% 
 \multicolumn{\pt@ncol}{l}{#1}% 
 \@ptabularcr 
 \noalign{\vskip .8ex}% 
%  \hline 
%  \@ptabularcr 
%  \noalign{\vskip -2ex}% 
}% 


\newcommand\figcaption{\@testopt{\@xfigcaption}{}}% 
%% suggested by Joern Wilms
\def\@figcaption#1{{\def\@captype{figure}\caption{\footnotesize #1}}}
\def\@xfigcaption[#1]#2{{\def\@captype{figure}\caption{\footnotesize #2}}}

\def\thefigure{\@arabic\c@figure}

\def\fnum@figure{{\footnotesize{\@eapj@cap@font
\mbox{\@eapj@figname~\thefigure}% <== was period here
}}}

\def\thetable{\@arabic\c@table}
%%% Table captions without making a floating table
\def\tabcaption{\@ifnextchar[{\@xtabcaption}{\@tabcaption}}
\def\@tabcaption#1{{\def\@captype{table}\caption{#1}}}
\def\@xtabcaption[#1]#2{{\def\@captype{table}\caption{#2}}}
\def\fnum@table{{\@eapj@cap@font \@eapj@tabname~\thetable}}

 \let\fnum@ptable=\fnum@table
 \def\fnum@ptablecont{{\centering{\scshape Table~\thetable}---{\itshape
 Continued}}}% 
 \long\def\@make@caption@text#1#2{% 
   {\small\centering#1{\scshape #2}\par\vskip1.4ex}
 }% 

\long\def\@makecaption@plano@cont#1#2{% 
  {\small \centering#1\par}\vskip1.4ex\relax
}% 


%% Commands from aastex.cls:
\newcommand\dataset{\@testopt\@dataset{[}}%
\def\@dataset[#1]#2{#2}%
\newcommand\facility{\@testopt\@facility{[}}%
\def\@facility[#1]#2{{\vskip6pt{\large\it Facility:} #2}}%

\newcommand\facilities{\@testopt\@facilities{[}}%
\def\@facilities[#1]#2{{\vskip6pt{\large\it Facilities:} #2}}%

\newcommand\software{\@testopt\@software{[}}%
\def\@software[#1]#2{{\vskip6pt{\large\it Software:} #2}}%

\newcommand\object{\@testopt\@object{[}}%
\def\@object[#1]#2{#2}%
\newcommand\objectname{\@testopt\@objectname{[]}}% 
\def\@objectname[#1]#2{#2}% 
\newlength{\plot@width}
\def\eps@scaling{1.0}% 
\newcommand\epsscale[1]{\def\eps@scaling{#1}}% 
\newcommand\plotone[1]{% 
 \centering 
 \leavevmode 
 \setlength{\plot@width}{0.85\linewidth}
 \includegraphics[width={\eps@scaling\plot@width}]{#1}% 
}% 
\newcommand\plottwo[2]{% 
 \centering 
 \leavevmode 
 \setlength{\plot@width}{0.425\linewidth}
 \includegraphics[width={\eps@scaling\plot@width}]{#1}% 
 \hfil 
 \includegraphics[width={\eps@scaling\plot@width}]{#2}% 
}% 
\newcommand\plotfiddle[7]{% 
 \centering 
 \leavevmode 
 \vbox\@to#2{\rule{\z@}{#2}}% 
 \includegraphics[% 
  scale=#4, 
  angle=#3, 
  origin=c 
 ]{#1}% 
}% 
\newcommand\figurenum[1]{% 
 \def\thefigure{#1}% 
 \let\@currentlabel\thefigure 
 \addtocounter{figure}{\m@ne}% 
}% 

\newcommand\phn{\phantom{0}}% 
\newcommand\phd{\phantom{.}}% 
\newcommand\phs{\phantom{$-$}}% 
\newcommand\phm[1]{\phantom{#1}}% 
\let\la=\lesssim            % For Springer A&A compliance... 
\let\ga=\gtrsim 
\newcommand\sq{\mbox{\rlap{$\sqcap$}$\sqcup$}}% 
\newcommand\arcdeg{\mbox{$^\circ$}}% 
\newcommand\arcmin{\mbox{$^\prime$}}% 
\newcommand\arcsec{\mbox{$^{\prime\prime}$}}% 
\newcommand\fd{\mbox{$.\!\!^{\mathrm d}$}}% 
\newcommand\fh{\mbox{$.\!\!^{\mathrm h}$}}% 
\newcommand\fm{\mbox{$.\!\!^{\mathrm m}$}}% 
\newcommand\fs{\mbox{$.\!\!^{\mathrm s}$}}% 
\newcommand\fdg{\mbox{$.\!\!^\circ$}}% 
\newcommand\farcm@mss{\mbox{$.\mkern-4mu^\prime$}}% 
\let\farcm\farcm@mss 
\newcommand\farcs@mss{\mbox{$.\!\!^{\prime\prime}$}}% 
\let\farcs\farcs@mss 
\newcommand\fp{\mbox{$.\!\!^{\scriptscriptstyle\mathrm p}$}}% 
\newcommand\micron{\mbox{$\mu$m}}% 
\def\farcm@apj{% 
 \mbox{.\kern -0.7ex\raisebox{.9ex}{\scriptsize$\prime$}}% 
}% 
\def\farcs@apj{% 
 \mbox{% 
  \kern  0.13ex.% 
  \kern -0.95ex\raisebox{.9ex}{\scriptsize$\prime\prime$}% 
  \kern -0.1ex% 
 }% 
}% 
 

\newcommand\ion[2]{#1$\;${%
\ifx\@currsize\normalsize\small \else
\ifx\@currsize\small\footnotesize \else
\ifx\@currsize\footnotesize\scriptsize \else
\ifx\@currsize\scriptsize\tiny \else
\ifx\@currsize\large\normalsize \else
\ifx\@currsize\Large\large
\fi\fi\fi\fi\fi\fi
\rmfamily\@Roman{#2}}\relax}% 

\renewcommand\case[2]{\mbox{$\frac{#1}{#2}$}}% 
\renewcommand\slantfrac{\case}% 
\newcommand\onehalf{\slantfrac{1}{2}}% 
\newcommand\onethird{\slantfrac{1}{3}}% 
\newcommand\twothirds{\slantfrac{2}{3}}% 
\newcommand\onequarter{\slantfrac{1}{4}}% 
\newcommand\threequarters{\slantfrac{3}{4}}% 
\newcommand\ubvr{\mbox{$U\!BV\!R$}}%% UBVR system 
\newcommand\ub{\mbox{$U\!-\!B$}}%   % U-B 
\newcommand\bv{\mbox{$B\!-\!V$}}%   % B-V 
\renewcommand\vr{\mbox{$V\!-\!R$}}%   % V-R 
\newcommand\ur{\mbox{$U\!-\!R$}}%   % U-R 

%% need this change so that it works correctly in tables:
{\catcode`\$=\active
\gdef\nodata{ ~$\cdots$~ }}% 

\newcommand\diameter{\ooalign{\hfil/\hfil\crcr\mathhexbox20D}}% 
\newcommand\degr{\arcdeg}% 
\newcommand\Sun{\sun}% 
\newcommand\Sol{\sun}% 
\newcommand\sun{\odot}% 
\newcommand\Mercury{\astro{\char1}}% Mercury symbol, "1" 
\newcommand\Venus{\astro{\char2}}% Venus symbol, "2" 
\newcommand\Earth{\earth}% 
\newcommand\Terra{\earth}% 
\newcommand\earth{\oplus}% 
\newcommand\Mars{\astro{\char4}}% Mars symbol, "4" 
\newcommand\Jupiter{\astro{\char5}}% Jupiter symbol, "5" 
\newcommand\Saturn{\astro{\char6}}% Saturn symbol, "6" 
\newcommand\Uranus{\astro{\char7}}% Uranus symbol, "7" 
\newcommand\Neptune{\astro{\char8}}% Neptune symbol, "8" 
\newcommand\Pluto{\astro{\char9}}% Pluo symbol, "9" 
\newcommand\Moon{\astro{\char10}}% Moon symbol, "M" 
\newcommand\Luna{\Moon}% 
\newcommand\Aries{\astro{\char11}}% 
\newcommand\VEq{\Aries}% vernal equinox (Aries) 
\newcommand\Taurus{\astro{\char12}}% 
\newcommand\Gemini{\astro{\char13}}% 
\newcommand\Cancer{\astro{\char14}}% 
\newcommand\Leo{\astro{\char15}}% 
\newcommand\Virgo{\astro{\char16}}% 
\newcommand\Libra{\astro{\char17}}% 
\newcommand\AEq{\Libra}% autumnal equinox (Libra) 
\newcommand\Scorpius{\astro{\char18}}% 
\newcommand\Sagittarius{\astro{\char19}}% 
\newcommand\Capricornus{\astro{\char20}}% 
\newcommand\Aquarius{\astro{\char21}}% 
\newcommand\Pisces{\astro{\char22}}% 
 
\newcommand\sbond{\chem@bnd{\@sbnd}}%
\newcommand\dbond{\chem@bnd{\@dbnd}}%
\newcommand\tbond{\chem@bnd{\@tbnd}}%
\def\chem@bnd#1{%
 {%
  \kern.1em\relax
  \setbox\z@\hbox{M}%
  \dimen@ii.8em\relax
  \p@=.1em\relax
  \dimen@.5\ht\z@\dimen@i-\dimen@
  \advance\dimen@1.5\p@\advance\dimen@i-1.0\p@
  #1%
  \kern.1em\relax
  }%
 }%
\def\@sbnd{%
 \advance\dimen@-1.5\p@\advance\dimen@i1.5\p@
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 }%
\def\@dbnd{%
 \advance\dimen@-0.5\p@\advance\dimen@i0.5\p@
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 \advance\dimen@-1.5\p@\advance\dimen@i1.5\p@
 \hskip-\dimen@ii
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 }%
\def\@tbnd{%
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 \advance\dimen@-1.5\p@\advance\dimen@i1.5\p@
 \hskip-\dimen@ii
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 \advance\dimen@-1.5\p@\advance\dimen@i1.5\p@
 \hskip-\dimen@ii
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 }%

\newcommand\anchor[2]{#2}% 
\renewcommand\url{\@dblarg\@url}% 
\def\@url[#1]{\anchor{#1}}% 

\let\jnl@style=\rmfamily 
\def\ref@jnl#1{{\jnl@style#1}}% 
\newcommand\aj{\ref@jnl{AJ}}%        % Astronomical Journal 
\newcommand\araa{\ref@jnl{ARA\&A}}%  % Annual Review of Astron and Astrophys 
\renewcommand\apj{\ref@jnl{ApJ}}%    % Astrophysical Journal 
\newcommand\apjl{\ref@jnl{ApJL}}     % Astrophysical Journal, Letters 
\newcommand\apjs{\ref@jnl{ApJS}}%    % Astrophysical Journal, Supplement 
\renewcommand\ao{\ref@jnl{ApOpt}}%   % Applied Optics 
\newcommand\apss{\ref@jnl{Ap\&SS}}%  % Astrophysics and Space Science 
\newcommand\aap{\ref@jnl{A\&A}}%     % Astronomy and Astrophysics 
\newcommand\aapr{\ref@jnl{A\&A~Rv}}%  % Astronomy and Astrophysics Reviews 
\newcommand\aaps{\ref@jnl{A\&AS}}%    % Astronomy and Astrophysics, Supplement 
\newcommand\azh{\ref@jnl{AZh}}%       % Astronomicheskii Zhurnal 
\newcommand\baas{\ref@jnl{BAAS}}%     % Bulletin of the AAS 
\newcommand\icarus{\ref@jnl{Icarus}}% % Icarus
\newcommand\jrasc{\ref@jnl{JRASC}}%   % Journal of the RAS of Canada 
\newcommand\memras{\ref@jnl{MmRAS}}%  % Memoirs of the RAS 
\newcommand\mnras{\ref@jnl{MNRAS}}%   % Monthly Notices of the RAS 
\renewcommand\pra{\ref@jnl{PhRvA}}% % Physical Review A: General Physics 
\renewcommand\prb{\ref@jnl{PhRvB}}% % Physical Review B: Solid State 
\renewcommand\prc{\ref@jnl{PhRvC}}% % Physical Review C 
\renewcommand\prd{\ref@jnl{PhRvD}}% % Physical Review D 
\renewcommand\pre{\ref@jnl{PhRvE}}% % Physical Review E 
\renewcommand\prl{\ref@jnl{PhRvL}}% % Physical Review Letters 
\newcommand\pasp{\ref@jnl{PASP}}%     % Publications of the ASP 
\newcommand\pasj{\ref@jnl{PASJ}}%     % Publications of the ASJ 
\newcommand\qjras{\ref@jnl{QJRAS}}%   % Quarterly Journal of the RAS 
\newcommand\skytel{\ref@jnl{S\&T}}%   % Sky and Telescope 
\newcommand\solphys{\ref@jnl{SoPh}}% % Solar Physics 
\newcommand\sovast{\ref@jnl{Soviet~Ast.}}% % Soviet Astronomy 
\newcommand\ssr{\ref@jnl{SSRv}}% % Space Science Reviews 
\newcommand\zap{\ref@jnl{ZA}}%       % Zeitschrift fuer Astrophysik 
\renewcommand\nat{\ref@jnl{Nature}}%  % Nature 
\newcommand\iaucirc{\ref@jnl{IAUC}}% % IAU Cirulars 
\newcommand\aplett{\ref@jnl{Astrophys.~Lett.}}%  % Astrophysics Letters 
\newcommand\apspr{\ref@jnl{Astrophys.~Space~Phys.~Res.}}% % Astrophysics Space Physics Research 
\newcommand\bain{\ref@jnl{BAN}}% % Bulletin Astronomical Institute of the Netherlands 
\newcommand\fcp{\ref@jnl{FCPh}}%   % Fundamental Cosmic Physics 
\newcommand\gca{\ref@jnl{GeoCoA}}% % Geochimica Cosmochimica Acta 
\newcommand\grl{\ref@jnl{Geophys.~Res.~Lett.}}%  % Geophysics Research Letters 
\renewcommand\jcp{\ref@jnl{JChPh}}%     % Journal of Chemical Physics 
\newcommand\jgr{\ref@jnl{J.~Geophys.~Res.}}%     % Journal of Geophysics Research 
\newcommand\jqsrt{\ref@jnl{JQSRT}}%   % Journal of Quantitiative Spectroscopy and Radiative Trasfer 
\newcommand\memsai{\ref@jnl{MmSAI}}% % Mem. Societa Astronomica Italiana 
\newcommand\nphysa{\ref@jnl{NuPhA}}%     % Nuclear Physics A 
\newcommand\physrep{\ref@jnl{PhR}}%       % Physics Reports 
\newcommand\physscr{\ref@jnl{PhyS}}%        % Physica Scripta 
\newcommand\planss{\ref@jnl{Planet.~Space~Sci.}}%  % Planetary Space Science 
\newcommand\procspie{\ref@jnl{Proc.~SPIE}}%      % Proceedings of the SPIE 

\newcommand\actaa{\ref@jnl{AcA}}%  % Acta Astronomica
\newcommand\caa{\ref@jnl{ChA\&A}}%  % Chinese Astronomy and Astrophysics
\newcommand\cjaa{\ref@jnl{ChJA\&A}}%  % Chinese Journal of Astronomy and Astrophysics
\newcommand\jcap{\ref@jnl{JCAP}}%  % Journal of Cosmology and Astroparticle Physics
\newcommand\na{\ref@jnl{NewA}}%  % New Astronomy
\newcommand\nar{\ref@jnl{NewAR}}%  % New Astronomy Review
\newcommand\pasa{\ref@jnl{PASA}}%  % Publications of the Astron. Soc. of Australia
\newcommand\rmxaa{\ref@jnl{RMxAA}}%  % Revista Mexicana de Astronomia y Astrofisica

%% added feb 9, 2016
\newcommand\maps{\ref@jnl{M\&PS}}% Meteoritics and Planetary Science
\newcommand\aas{\ref@jnl{AAS Meeting Abstracts}}% American Astronomical Society Meeting Abstracts
\newcommand\dps{\ref@jnl{AAS/DPS Meeting Abstracts}}% American Astronomical Society/Division for Planetary Sciences Meeting Abstracts



\let\astap=\aap 
\let\apjlett=\apjl 
\let\apjsupp=\apjs 
\let\applopt=\ao 

\newcommand\ulap[1]{\vbox\@to\z@{{\vss#1}}}% 
\newcommand\dlap[1]{\vbox\@to\z@{{#1\vss}}}% 


% Implementation of deluxetables via the longtable package disabled.

\newcounter{table@save}
% doesn't seem needed:
%\def\save@label#1{
 % \if@filesw\immediate\write\@auxout{%
  %  \string\newlabel{#1}{{\@currentlabel}{\thepage}}}%
%  \fi%
%}


\def\tablenum#1{%
  \setcounter{table@save}{\the\c@table}
  \gdef\use@tablenum{1}
  \setcounter{table}{#1}\def\thetable{#1}\def\@currentlabel{#1}
  %\def\label##1{\save@label{##1}}
}%
\gdef\use@tablenum{0}
\def\restore@tablenum{\if\use@tablenum0\else\setcounter{table}{\the\c@table@save}\addtocounter{table}{\m@ne}\gdef\use@tablenum{0}\fi}

\def\tabletypesize#1{\gdef\currtabletypesize{#1}
\def\@table@type@size{#1}}%

\tabletypesize{\small}
\let\tablefontsize=\tabletypesize % for compatibility with old documents

\gdef\@tablecaption{}
\def\tablecaption#1{\gdef\@tablecaption{#1}}

\def\LT@endpbox{%
  \@finalstrut\@arstrutbox
  \egroup
  \the\LT@p@ftn
  \global\LT@p@ftn{}%
  \hfil}

%% Used for longtable
\def\LT@makecaption#1#2#3{%
  \LT@mcol\LT@cols c{\hbox to\z@{\hss\parbox[t]\LTcapwidth
{%
\def \@currentlabel{\thetable}
     \sbox\@tempboxa{\small #2. #3}%
     \ifdim\wd\@tempboxa>\hsize
      \small #2. #3%
     \else
       \hbox to\hsize{\hfil\box\@tempboxa\hfil}%
     \fi
    \endgraf\vskip\baselineskip}%
  \hss}}}

\let\LT@makecaption@rtx=\LT@makecaption % to fight redefinition in Revtex-4.1

\def\xfigure{figure}
%% from book.cls
\long\def\@makecaption#1#2{%
  \vskip\abovecaptionskip
\ifx\@captype\xfigure
\gdef \@currentlabel{\thefigure}
\else
\gdef \@currentlabel{\thetable}\fi
%% \small added to keep currtabletypesize from determining size of caption
  \sbox\@tempboxa{\small
#1. #2}%
  \ifdim \wd\@tempboxa >\hsize
\small
    #1. #2\par 
  \else
    \global \@minipagefalse
    \hb@xt@\hsize{\hfill\box\@tempboxa\hfill}%
  \fi
  \vskip\belowcaptionskip}

\newdimen\@abovenoteskip
\newcommand\tablerefs[1]{\ifdim\@abovenoteskip=0pt\global\@abovenoteskip=10pt\fi
{\small\@tableref{\parfillskip\z@ plus1fil\relax #1\endgraf}}}%

\def\@tableref#1{%
 \par
 \vspace*{3ex}%
 {%\parbox{\pt@width} %%%%
 {\hskip1em\rm References. --- #1}\par}%
}%


\global\def\tablenotemark#1{{\normalfont\textsuperscript{#1}}}
\global\def\tablenotetext#1#2{\footnotetext[#1]{\currtabletypesize\relax#2}}

%% redefined by AH below, since it wasn't working with tabular table
\global\def\tablenotetext#1#2{\vskip-8pt\vskip1sp\flushleft{\currtabletypesize
\noindent\hskip1em $^{#1}$ #2}\vskip1sp}


\newif\if@table@not@headed

%% this doesn't seem to be used:
\def\tablehead#1{\@table@not@headedfalse%
  \kill
  \caption{\\%
    \@tablecaption}%
    \\\hline\hline%
  #1\vrule height 12pt depth 10pt width 0pt\relax 
\hskip\tabcolsep\\[.7ex]
  \hline\\[-1.5ex]
  \endfirsthead
  \caption[]{--- \emph{Continued}}\\
  \hline
  \hline\\[-1.7ex]
  #1\hskip\tabcolsep\\[.7ex]
  \hline\\[-1.5ex]
  \endhead
  \hline
  \endfoot%
}

\newlength{\table@note@skip}
\setlength{\table@note@skip}{0.5ex}
\newlength{\deluxe@table@width}
\newlength{\@d@t@a}
\newcounter{deluxe@table@num}
\newdimen\LTcapwidth

\def\colhead#1{\multicolumn{1}{c}{\vrule depth 3pt width 0pt#1}}
\def\twocolhead#1{\multicolumn{2}{c}{\hss\vrule depth 3pt width
0pt#1\hss}}
\def\nocolhead#1{\multicolumn{1}{h}{}}
\def\dcolhead#1{\multicolumn{1}{c}{$\vrule depth 3pt width0pt\relax#1$}}

%\newcounter{LT@tables}
\def\tablewidth#1{%
  \ifdim#1=\z@
  \else
  \gdef\@d@t@@flag{1}
  \if@filesw\immediate\write\@auxout{%
   \gdef\expandafter\noexpand
   \csname deluxe@table@width@\romannumeral\c@LT@tables\endcsname
   {#1}}
  \fi
  \fi
}


\def\save@natural@width{%
      \ifnum\@d@t@@flag=0
        \setlength{\@d@t@a}{0pt}%
        \let\@d@t@b=\LT@entry%
        \def\LT@entry##1##2{\addtolength{\@d@t@a}{##2}%
        }%
        \expandafter\csname LT@\romannumeral\c@deluxe@table@num\endcsname
        \setlength{\@d@t@a}{-\@d@t@a}
        \tablewidth{\the\@d@t@a}
        \def\LT@entry{\@d@t@b}
      \fi
}

\def\lt@expand@linewidth@one{\setlength\LTleft{0pt}\setlength\LTright{0pt}}
\def\lt@expand@linewidth@two{@{\extracolsep{0pt plus 1filll}}}
\def\find@table@width{%
%%% set table width using aux file and command \tablewidth
    \setcounter{deluxe@table@num}{\c@LT@tables}
    \refstepcounter{deluxe@table@num}
    \expandafter\ifx\csname deluxe@table@width@\romannumeral\c@deluxe@table@num\endcsname\relax
      \def\@d@t@{0.999\linewidth}
    \else
      \edef\@d@t@{\expandafter\csname deluxe@table@width@\romannumeral\c@deluxe@table@num\endcsname}
    \fi
    \ifdim\@d@t@<\z@% then natural width is used
      \setlength{\deluxe@table@width}{-\@d@t@}
      \setlength{\LTcapwidth}{-\@d@t@}
      \def\lt@expand@linewidth{\relax}
      \def\lt@expand@linewidth@{}
    \else% we will enclose table in the minipage of the given width and make
         % longtable to span the full minipage width
      \ifdim\@d@t@>\z@\else\def\@d@t@{0.999\linewidth}\fi
      \setlength{\deluxe@table@width}{\@d@t@}
      \setlength{\LTcapwidth}{\@d@t@}
      \def\lt@expand@linewidth{\lt@expand@linewidth@one}
      \def\lt@expand@linewidth@{\lt@expand@linewidth@two}
    \fi
}

\newlength{\abovedeluxetableskip}
\newlength{\belowdeluxetableskip}
\setlength{\abovedeluxetableskip}{0pt}
\setlength{\belowdeluxetableskip}{0pt}
\setlength{\tabcolsep}{5pt}

\setlength\doublerulesep{1.5pt}
\newdimen\lastrowheight
\def\set@last@row@height{\setlength{\lastrowheight}{\ht\strutbox}\addtolength{\lastrowheight}{\dp\strutbox}\setlength{\lastrowheight}{-\arraystretch\lastrowheight}}

\let\tableline=\hline % Revtex said: Command \tableline is obsolete; Use \colrule instead.. 

%% not used?
\newtoks\DT@p@ftn 
\global\def\xtablenotetext@DT#1#2{
%  \edef\@tempa{\the\DT@p@ftn\noexpand\footnotetext[#1]}
  \edef\@tempa{\the\DT@p@ftn\noexpand\tablenotemark{#1}~}
  \global\DT@p@ftn\expandafter{\@tempa{\@table@type@size#2}\par}}%

%%% AH %%%%%%%%%%%%%%%%%%%%%
\let\savedollar$
\catcode`\$=\active
\let$\savedollar

%%

\def\resetdecimals{\global\let\zdoit\relax\global\let\ddoit\relax}

\def\tableheadfrac#1{}
\newcount\pt@column 
\newcount\pt@ncol 

\newcommand\tablecolumns[1]{% 
 \pt@column=#1\relax 
 \pt@ncol=#1\relax 
 \global\let\pt@addcol\@empty 
}% 

%% from older version, probably can delete:
\def\@tablecom#1{% 
 \vspace*{\table@note@skip}
\par 
{\parbox{\linewidth}{\hskip1em\rmfamily {\@eapj@cap@font Note}. --- #1}\par}% 
}% 
\def\@tableref#1{% 
 \vspace*{\table@note@skip}
\par 
{\parbox{\linewidth}{\hskip1em\rmfamily {\@eapj@cap@font References}. --- #1}\par}% 
}% 
\def\spew@tblnotes{% 
 \@ifx@empty\tblref@list{}{% 
  \@tablenotes{\tblref@list}%
  \vspace*{\table@note@skip}%
  \global\let\tblref@list\@empty 
 }% 
 \@ifx@empty\tblnote@list{}{% 
  \@tablenotes{\tblnote@list}% 
  \vspace*{\table@note@skip}%
  \global\let\tblnote@list\@empty
 }
 \the\DT@p@ftn%
}% 

%% for notes on emulateapj please see http://hea-www.harvard.edu/~alexey/emulateapj
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% End of code taken from emulateapj.cls %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%


%%% New as of august 2015:

%% These commands requested AAS
\newcommand{\noprint}[1]{}
\newcommand{\figsetstart}{{\bf Fig. Set} }
\newcommand{\figsetend}{}
\newcommand{\figsetgrpstart}{}
\newcommand{\figsetgrpend}{}
\newcommand{\figsetnum}[1]{{\bf #1.}}
\newcommand{\figsettitle}[1]{ {\bf #1} }
\newcommand{\figsetgrpnum}[1]{\noprint{#1}}
\newcommand{\figsetgrptitle}[1]{\noprint{#1}}
\newcommand{\figsetplot}[1]{\noprint{#1}}
\newcommand{\figsetgrpnote}[1]{\noprint{#1}}

%% for url's in document, will allow them to break over lines.
\usepackage{url}

%% for tables continuing over pages
\usepackage{longtable}

%% In response to request from AAS
 \usepackage[bookmarks=false,         % show bookmarks bar?
     pdfnewwindow=true,      % links in new window
     colorlinks=true,    % false: boxed links; true: colored links
     linkcolor=cyan,     % color of internal links
     citecolor=cyan,     % color of links to bibliography
     filecolor=cyan,  % color of file links
     urlcolor=cyan,      % color of external links
 ]{hyperref}

%% for editing changes
\usepackage{xcolor}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%% Additions to  AASTeX by Amy Hendrickson, TeXnology Inc, August 17, 2015

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Table Tools, written by Amy Hendrickson
%% for American Astronomical Society
%% August 17, 2015
%% array package is necessary:
\usepackage{array}

%%%%%%%%
%% 1) h in table preamble will makes text in that column ignored.

%%%%%%%%
%% 2) uppercase C L or R will make those columns in math mode.

%%%%%%%%
%% 3) \begin{splittabular}{<preamble B preamble>}...\end{splittabular}
%%    `B' in the preamble will show where the table should be broken. It may
%%    be used once in a preamble to break the table into two parts, or 
%%    two times in a preamble to break the table into three parts.

%%    The resulting table will position left half of table above right half of table.

%%    Care should be taken with \multicolumn so that it won't be broken between
%%    the two parts of the table. You may need to shorten the \multicolumn argument
%%    for number of columns to be appropriate to the number of columns in
%%    the new split table.

%%     In this example, for instance, we start with:
%%        \begin{tabular}{ccccccc}
%%        \multicolumn{7}{c}{\textsc{Table 1}}
%%     and change it to:
%%        \begin{splittabular}{ccccBccc}
%%        \multicolumn{4}{c}{\textsc{Table 1}}\\
%% 
%%     If you'd like to have a table number in the second half of the table,
%%     you can ask for a second multicolumn command that will position at the
%%     top of the second half of the table:
%%
%%        \begin{splittabular}{ccccBccc}
%%        \multicolumn{4}{c}{\textsc{Table 1}}&\multicolumn{3}{c}{\textsc{Table 1, Continued}}\\
%%
%%     You can do the same for the caption, and give a Continued caption for
%%     the second half of the table. Originally:
%%        \multicolumn{7}{c}{\textsc{Log of \textit{HST} Observations for
%%         NGC~6388}}
%%     Now, changed so that we get a caption on both halves of the table:
%%        \multicolumn{4}{c}{\textsc{Log of \textit{HST} Observations for
%%         NGC~6388}}&\multicolumn{3}{c}{\textsc{Log of \textit{HST} Observations for
%%         NGC~6388, Continued}}\\
%%     
%%     Similarly, you can rearrange table notes to appear at the bottom of
%%     the appropriate half of the table. For instance, starting with this,
%%     which would make table notes both appear at the bottom of the top half
%%     of the table:
%%        \multicolumn{4}{l}{\small{$\dagger$ Average distance of data set from
%%            cluster center.}}\\
%%        \multicolumn{4}{l}{$^{\rm a}$SNAP program.}\\
%%     
%%     We may substitute the following:
%%        \multicolumn{4}{l}{$^{\rm a}$SNAP program.}&\multicolumn{3}{l}{\small{$\dagger$ Average distance of data set from
%%            cluster center.}}\\
%%     
%%     Which will produce the first endnote underneath the top half of the
%%     table, and the second underneath the bottom half.
%%     
%%     Another option would be to have both endnotes appear below the bottom
%%     half of the table. Easily done:
%%        &&&&\multicolumn{3}{l}{$^{\rm a}$SNAP program.}\\
%%        &&&&\multicolumn{3}{l}{\small{$\dagger$ Average distance of data set from
%%            cluster center.}}\\
%%     %%     %%     %%     %%     %%     %%     

%%%%%%%%
%% 4) \colnumbers will make line with column numbers automatic. It will work with splittabular
%%     and splitdeluxetable as well as tabular.
%%     To use: type in \colnumbers within the table whereever you'd like it to appear, typically
%%     underneath the column headers, before the lines of data.

%%%%%%%%
%% 5) Easy Decimal numbering
%%    How to make decimal numbers in tables line up on the period:

%%     Use D (for decimal column) in table preamble for every decimal number. The decimal
%%     numbers will use two columns, one for the left part of the decimal
%%     number and one for the right part.

%%     D may be used more than once in a table preamble.

%%     If you want to type in a column header  over the decimal
%%     numbers, please use \multicolumn2c{} to span both columns.

%%     After the column headers, to start decimal numbering, 
%%     type in \decimals in the body of the table.

%%     When entering decimal numbers
%%     remember to leave a space after the decimal number, before the following &.
%%     For instance: &22.3 &35.96 \\

%%     If you'd like an empty entry, please supply a period and a space: & . &. You
%%     will not see the period in the resulting table.

%%     example:
%    \begin{tabular}{rDD}
%     \hline
%     &&&\multicolumn2c{\bf More}\\
%     &\multicolumn2c{\bf Decimals}&\multicolumn2c{\bf Decimals}\\
%     \hline
%     \hline
%     \decimals
%     one& . &34.2 \\
%     two &567.0 &21345 \\
%     three&.0 &62.5 \\
%     four&245 &5034.349923 \\
%     five&21 & \\
%     six& &21.6 
%    \end{tabular}
%%
%    Decimal numbering works within \begin{splittabular}...\end{splittabular}
%    \begin{splitdeluxetable} and \end{splitdeluxetable} for tables broken into two
%    or three parts. \colnumbers will number the columns counting both sides 
%    of the decimal number as one column; \tablehead{\colhead{}...} will make
%    the column headers position above the two columns used for one decimal
%    number as well.
%
%    Decimal numbers will be in math mode so that plus and minus signs are printed
%    correctly, expressions like `$\pm$ 1.2' or `\pm 1.2', will both work
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\let\savedollar$
\catcode`\$=\active

\let\savetabular\tabular
\def\tabular{\catcode`\&=\active\relax\catcode`\$=\active\relax\savetabular}
\long\gdef\eatone{\setbox0=\hbox\bgroup\savedollar\let$\relax}
\gdef\endeatone{\savedollar\egroup\hskip-2\tabcolsep}


%% Hide, important because it allows us to split tables horizontally
\newcolumntype{h}{>\eatone c<\endeatone}

\newcolumntype{C}{>{\bgroup\savedollar\let$\relax}c<{\savedollar\egroup}}
\newcolumntype{L}{>{\bgroup\savedollar\let$\relax}l<{\savedollar\egroup}}
\newcolumntype{R}{>{\bgroup\savedollar\let$\relax}r<{\savedollar\egroup}}

\newcolumntype{B}{>\eatone c<\endeatone}  %% used for \splittabular to indicate break in
                                          %% two or three parts of table

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%% Split tables into 2 or 3 parts; stack the parts
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% Process table preamble

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Count number of columns in preamble

\newcount\colcount
\newcount\firstcolcount
\newcount\secondcolcount
\newcount\thirdcolcount
\newcount\columncount

%% \makeatother is necessary to keep @ from being treated as a letter
%% when counting the number of columns in table.
\makeatother
\def\xD{D}
\newcount\howmanyDs
\newcount\firsthowmanyDs
\newcount\secondhowmanyDs

\newif\ifD

\def\countcols#1{\ifcat#1c
\global\advance\colcount by 1\relax\fi
%%
\def\lookforD{#1}
\ifx\lookforD\xD
\global\advance\howmanyDs by 1
\global\advance\colcount by1\relax
\fi
%%
\futurelet\next\lookatnext}
\makeatletter

\def\xeatone#1{\countcols}
\def\xrelax{\relax}

%% second conditional deals with expressions like @{} in
%% the preamble.
\def\lookatnext{\if\next\xrelax\let\go\relax
\else
\ifx\next\bgroup\let\go\xeatone\else
\let\go\countcols\fi\fi\go}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% reset line counting at \\

%% arraycr redefined to match revtex4-1
\def\new@arraycr{\relax 
\global\columncount=0\relax
\global\colheadcount=0\relax
\iffalse {\fi \ifnum 0=`}\fi \@ifstar {\global \@tbpen \@M \@xarraycr}
{\global \@tbpen \intertabularlinepenalty \@xarraycr}}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Redefining & is necessary to get around \omit in \multicolumn,
%% which prevents @{} from being used to change the meaning of multicolumn.
%% Now column number and top or bottom table is used to determine whether
%% multicolumn should be turned on or off.
\let\saveampersand&

\newif\iffirstbox
\newif\ifsecondbox
\newif\ifthirdbox

\long\gdef\CheckNumberAndSwitch{\global\advance\columncount by 1\relax%
\saveampersand}


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Parse and count preamble

\def\catchfirstpreamble#1B#2Z{%firstpreamble
\def\leftpreamble{#1\relax}
\colcount=0
\global\howmanyDs=0\relax
\expandafter\countcols\leftpreamble
\global\firstcolcount\the\colcount
\global\firsthowmanyDs\the\howmanyDs
\ifnum\firstcolcount>25
\typeout{^^J
-----------------------------------------------------
^^J
Warning!
^^J^^J 
Too many columns in first part of table!
^^J^^J
Maximum number of columns in each part of the table is 25. Each `D' counts as two columns.
^^J
-----------------------------------------------------
}\fi
\global\howmanyDs=0\relax
\global\colcount=0\relax
\newcolumntype{A}{#1}}

\def\catchsecondpreamble#1B#2Z{%secondpreamble
\def\rightpreamble{#2\relax}
\colcount=0
\expandafter\countcols\rightpreamble
\global\secondcolcount\colcount
\global\secondhowmanyDs\the\howmanyDs
\ifnum\secondcolcount>25
\typeout{^^J-----------------------------------------------------
^^J
Warning!
^^J^^J 
Too many columns in second part of table!
^^J^^J
Maximum number of columns in each part of the table is 25. Each `D' counts as two columns.
^^J
-----------------------------------------------------
}\fi
\global\totalcolumns=\firstcolcount
\global\advance\totalcolumns by \secondcolcount
\global\colcount=0\relax
\newcolumntype{Z}{#2}}


\def\catchsecondofthreepreamble#1B#2B#3Z{%secondpreamble
\def\rightpreamble{#2\relax}
\colcount=0
\howmanyDs=0
\expandafter\countcols\rightpreamble
\global\secondcolcount\colcount
\global\secondhowmanyDs\the\howmanyDs
\global\totalcolumns=\firstcolcount
\global\advance\totalcolumns by \secondcolcount
\global\colcount=0\relax
\newcolumntype{Z}{#2}}


\def\catchthirdpreamble#1B#2B#3Z{%firstpreamble
\def\thirdpreamble{#3}
\colcount=0
\expandafter\countcols\thirdpreamble
\global\thirdcolcount\the\colcount
\global\advance\totalcolumns by \thirdcolcount
\global\colcount=0\relax
\newcolumntype{z}{#3}}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Define splittabular/endsplittabular

\newcount\PartsOfTable
\newif\ifbreaktab

\def\checkforB #1B#2B#3B{
%% defaults, may be used in \colnumbers
\gdef\tabfullpreamble{#1#2#3\relax}
\gdef\firstpreamble{#1}
\gdef\secondpreamble{#2}
\gdef\thirdpreamble{#3}
\ifx\thirdpreamble\empty
\global\PartsOfTable=2
\else
\global\PartsOfTable=3
\fi
}

\def\lookforD#1{\def\zone{#1}\ifx\zone\xD
\global\Dtrue\else\Dfalse\fi}

\newcount\columnoneandtwo
\newif\ifdonullmulticol

\newcount\colnumsused
\newcount\loopnum

\catcode`\&=\active
\let&\CheckNumberAndSwitch
\let$\savedollar

%sss
\def\splittabular{\catcode`\&=\active
\catcode`\$=\active
\global\colnumbersonfalse
\let\colnumbers\savesplitplaincolnumbers
\let\splitdecimalcolnumbers\savesplitdecimalcolnumbers
\def\decimalcolnumbers{\splitdecimalcolnumbers}
\xsplittabular}

\def\xsplittabular#1#2\end{{
\global\firsttimetrue
\let&\CheckNumberAndSwitch
\let$\savedollar
%%%
\def\endtabular{\endarray
\global\let\zdoit\relax
\global\let\ddoit\relax
\global\Dfalse}
\setbox0=\hbox{\def\firstarg{#1}\expandafter\lookforD\firstarg}
\PartsOfTable=0
\def\tempfullpreamble{#1BB}
\setbox0=\hbox{\expandafter\checkforB\tempfullpreamble}
%% Now PartsOfTable is either =2 or 3
%%
\global\colnumsused=0
\global\breaktabtrue
\global\colcount=0
%%
\ifnum\PartsOfTable=3
\let\multicolumn\threebreakmulticolumn
\else
\let\multicolumn\breakmulticolumn
\fi
%%
\def\one{#1Z}%% full preamble
\ifnum\PartsOfTable=3
\expandafter\catchfirstpreamble\one
\expandafter\catchsecondofthreepreamble\one
\expandafter\catchthirdpreamble\one
\makefirstdummycolumns %f
\makeseconddummycolumns %F
\makethirddummycolumns %E
\else %% PartsOfTable=2
\expandafter\catchfirstpreamble\one
\expandafter\catchsecondpreamble\one
\makefirstdummycolumns %f
\makeseconddummycolumns %F
\fi
%%%
\global\columncount=0
\ifnum\PartsOfTable=3
%% For table divided into three parts !!
\firstboxtrue\secondboxfalse\thirdboxfalse
%
\setbox\firsttablebox=\hbox{%
\begin{tabular}{AFE}% 
#2\crcr
\end{tabular}}
%
\hbox to \hsize{\hss\unhbox\firsttablebox\hss}
\vskip6pt
\hrule
\vskip6pt
\global\columncount=0
\firstboxfalse\secondboxtrue\thirdboxfalse
\ifcolnumberson
\let\colnumbers\xplain
\setbox\secondtablebox=\hbox{%
\begin{tabular}{fZE}% fZE
#2\crcr\end{tabular}}
\else
\setbox\secondtablebox=\hbox{
\begin{tabular}{fZE}% fZE
#2
\end{tabular}}
\fi
\hbox to \hsize{\hss\unhbox\secondtablebox\hss}
%%
\vskip6pt
\hrule
\vskip6pt
\firstboxfalse\secondboxfalse\thirdboxtrue
\global\columncount=0
\ifcolnumberson
\let\colnumbers\xplain
\setbox\thirdtablebox=\hbox{%
\begin{tabular}{fFz}% fFz
#2\crcr\end{tabular}}
\else
\setbox\thirdtablebox=\hbox{
\begin{tabular}{fFz}% fFz
#2
\end{tabular}}
\fi
\hbox to \hsize{\hss\unhbox\thirdtablebox\hss}
\else
%%
%% For table divided into two parts:
%%
\global\columncount=0
\secondboxfalse
 \setbox\firsttablebox=\hbox{%
\begin{tabular}{AF}% should be AF
#2
\end{tabular}
}
 \hbox to \hsize{\hss\unhbox\firsttablebox\hss}
\vskip6pt
\hrule
\vskip6pt
\global\columncount=0
\secondboxtrue
\ifcolnumberson
\setbox\secondtablebox=\hbox{\let\colnumbers\xplain
\begin{tabular}{fZ}% should be fZ
#2
\end{tabular}
}
\else
\setbox\secondtablebox=\hbox{
\begin{tabular}{fZ}% should be fZ
#2
\end{tabular}
}
\fi
\hbox to \hsize{\hss\unhbox\secondtablebox\hss}
%%
%% end of conditional testing for 2 or 3 part table:
\fi
%\global\breaktabfalse
}
\firsthowmanyDs=0
\secondhowmanyDs=0
\resetdecimals
\end
}

%yyy
\def\endsplittabular{\global\firsttimefalse\global\colnumbersonfalse
\global\let\splitplaincolnumbers\savesplitplaincolnumbers
\global\breaktabfalse}

\long\gdef\breakmulticolumn#1#2#3{%
\multispan{#1}%
\let&\CheckNumberAndSwitch
\ifsecondbox%
\ifnum\columncount<\firstcolcount%
\global\donullmulticoltrue%
\else%
\global\donullmulticolfalse%
\fi%
%%%
   \else% first box
\ifnum\columncount<\firstcolcount%
\global\donullmulticolfalse%
\else%
\global\donullmulticoltrue%
\fi%
\fi%
\ifdonullmulticol%
\global\advance\colheadcount by 1\relax
\begingroup
   \def\@addamp{\if@firstamp \@firstampfalse \else
                \@preamerr 5\fi}%
   \@mkpream{h}\@addtopreamble\@empty
   \endgroup
   \def\@sharp{}%
%% comment out \@arstrut to prevent blank line where multicolumn was found
%% on other parts of the table, but not on the current part.
   \@arstrut 
\@preamble
   \null\ignorespaces
\else%
\begingroup
   \def\@addamp{\if@firstamp \@firstampfalse \else
                \@preamerr 5\fi}%
   \@mkpream{#2}\@addtopreamble\@empty
   \endgroup
   \def\@sharp{#3}%
   \@arstrut \@preamble
   \null\ignorespaces
\fi%
\global\donullmulticolfalse%
\global\advance\columncount by #1\relax%
\global\advance\columncount-1\relax%
}


\long\gdef\threebreakmulticolumn#1#2#3{%
\multispan{#1}%
\columnoneandtwo=\firstcolcount\relax%
\advance\columnoneandtwo by \secondcolcount\relax%
%%
\global\donullmulticoltrue\relax%
\iffirstbox\relax%
\ifnum\columncount<\firstcolcount%
\global\donullmulticolfalse%
\fi\relax%
\ifnum\columncount=\firstcolcount\relax%
\donullmulticoltrue\fi\relax%
\else\relax%
%%
\ifsecondbox%
\ifnum\columncount=\firstcolcount\relax%
\global\donullmulticolfalse\fi\relax%
\ifnum\columncount>\firstcolcount%
\ifnum\columncount<\columnoneandtwo\relax%
\global\donullmulticolfalse\relax%
\fi\fi\relax%
\else\relax%
%%%
\ifthirdbox%
\ifnum\columncount<\columnoneandtwo%
\global\donullmulticoltrue%
\else\relax%
\global\donullmulticolfalse%
\fi\relax%
\fi%% end if third box
\fi%% end if second box
\fi%% end if first box
%%
\ifdonullmulticol%
\begingroup\relax%
\global\advance\colheadcount by 1\relax%
   \def\@addamp{\if@firstamp \@firstampfalse \else%
                \@preamerr 5\fi}%
   \@mkpream{h}\@addtopreamble\@empty%
   \endgroup%
   \def\@sharp{}%
%% comment out \@arstrut to prevent blank line where multicolumn was found
%% on other parts of the table, but not on the current part.
   \@arstrut \@preamble%
   \null\ignorespaces
\else%
\begingroup%
   \def\@addamp{\if@firstamp \@firstampfalse \else%
                \@preamerr 5\fi}%
   \@mkpream{#2}\@addtopreamble\@empty%
   \endgroup%
   \def\@sharp{#3}%
   \@arstrut \@preamble%
   \null\ignorespaces
\fi%
\global\donullmulticolfalse%
\global\advance\columncount by #1\relax%
\global\advance\columncount-1\relax%
}

%% First dummy columns is the part of the table that will not print,
%% in this case the first part to the left.
%%\makefirstdummycolumns
%%
%% and second dummy columns
%% \makeseconddummycolumns.
\gdef\makefirstdummycolumns{
\ifcase\firstcolcount
\or% 1
\newcolumntype{f}{h} % no D possible
%%%%%%%%%%%%%%%%%%
\or% 2
  \ifcase\firsthowmanyDs
  \newcolumntype{f}{hh}%% 0
  \or
  \newcolumntype{f}{d} %% 1
  \fi
%%%%%%%%%%%%%%%%%%
\or% 3
  \ifcase\firsthowmanyDs%
  \newcolumntype{f}{hhh}% 0 
  \or%
  \newcolumntype{f}{dh}% 1
  \fi%
%%%%%%%%%%%%%%%%%%
\or% 4
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhh} % 0 
    \or
   \newcolumntype{f}{dhh} % 1
   \or
   \newcolumntype{f}{dd} % 2 
   \fi
%%%%%%%%%%%%%%%%%%
\or% 5
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhh} % 0 
    \or
   \newcolumntype{f}{dhhh} % 1
   \or
   \newcolumntype{f}{ddh} % 2 
   \fi
%%%%%%%%%%%%%%%%%%
\or% 6
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhh} % 1
   \or
   \newcolumntype{f}{ddhh} % 2 
    \or
   \newcolumntype{f}{ddd} % 3
   \fi
%%%%%%%%%%%%%%%
\or % 7
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhh} % 2 
   \or
   \newcolumntype{f}{dddh} % 3
   \fi
%%%%%%%%%%%%%%%%%%
\or % 8
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhh} % 2 
   \or
   \newcolumntype{f}{dddhh} % 3
   \or
   \newcolumntype{f}{dddd} % 4
   \fi
%%%%%%%%%%%%%%%%%%
\or % 9
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhh} % 3
   \or
   \newcolumntype{f}{ddddh} % 4
   \fi
%%%%%%%%%%%%%%%%%%
\or %10
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhh} % 3
   \or
   \newcolumntype{f}{ddddhh} % 4
   \or
   \newcolumntype{f}{ddddd} % 5
   \fi
%%%%%%%%%%%%%%%%%%
\or % 11
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhh} % 4
   \or
   \newcolumntype{f}{dddddh} % 5
   \fi
%%%%%%%%%%%%%%%%%%
\or % 12
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhh} % 4
   \or
   \newcolumntype{f}{dddddhh} % 5
   \or
   \newcolumntype{f}{dddddd} % 6
   \fi
%%%%%%%%%%%%%%%%%%
\or %13
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhh} % 5
   \or
   \newcolumntype{f}{ddddddh} % 6
   \fi
%%%%%%%%%%%%%%%%%%
\or %14
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhh} % 6
   \or
   \newcolumntype{f}{ddddddd} % 7
   \fi
%%%%%%%%%%%%%%%%%%
\or %15
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhh} % 6
   \or
   \newcolumntype{f}{dddddddh} % 7
   \fi
%%%%%%%%%%%%%%%%%%
\or %16
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhh} % 7
   \or
   \newcolumntype{f}{dddddddd} % 8
\fi
%%%%%%%%%%%%%%%%%%
\or %17
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhhh} % 7
   \or
   \newcolumntype{f}{ddddddddh} % 8
\fi
%%%%%%%%%%%%%%%%%%
\or %18
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhhh} % 6
   \or
   \newcolumntype{f}{ddddddhhhh} % 7
   \or
   \newcolumntype{f}{ddddddddhh} % 8
   \or
   \newcolumntype{f}{ddddddddd} % 9
\fi
%%%%%%%%%%%%%%%%%%
\or %19
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhhhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhhhhh} % 7
   \or
   \newcolumntype{f}{ddddddddhhh} % 8
   \or
   \newcolumntype{f}{ddddddddddh} % 9
\fi
%%%%%%%%%%%%%%%%%%
\or %20
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhh hhhhh hhhhh hhhhh} % 0 
    \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhhhhhh} % 7
   \or
   \newcolumntype{f}{ddddddddhhhh} % 8
   \or
   \newcolumntype{f}{dddddddddhh} % 9
   \or
   \newcolumntype{f}{dddddddddd} % 10
   \fi
%%%%%%%%%%%%%%%%%%
\or %21
  \ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhh hhhhh hhhhh hhhhh h} % 0 
   \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhhhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhhhhhhh} % 7
   \or
   \newcolumntype{f}{ddddddddhhhhh} % 8
   \or
   \newcolumntype{f}{ddd ddd ddd hhh} % 9
   \or
   \newcolumntype{f}{ddd ddd ddd d h} % 10
   \fi
%%%%%%%%%%%%%%%%%%
\or %22
\ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhh hhhhh hhhhh hhhhh hh} % 0 
   \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhhhhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhhhhhhhh} % 7
   \or
   \newcolumntype{f}{ddddddddhhhhhh} % 8
   \or
   \newcolumntype{f}{ddd ddd ddd hhhh} % 9
   \or
   \newcolumntype{f}{ddd ddd ddd dhh} % 10
   \or
   \newcolumntype{f}{ddddddddddd} % 11
   \fi
%%%%%%%%%%%%%%%%%%
\or %23
\ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhh hhhhh hhhhh hhhhh hhh} % 0 
   \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhhhhhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhhhhhhhhh} % 7
   \or
   \newcolumntype{f}{ddddddddhhhhhhh} % 8
   \or
   \newcolumntype{f}{ddd ddd ddd hhhhh} % 9
   \or
   \newcolumntype{f}{ddddddddddhhh} % 10
   \or
   \newcolumntype{f}{ddddd ddddd dh} % 11
   \fi
%%%%%%%%%%%%%%%%%%
\or %24
\ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhh hhhhh hhhhh hhhhh hhhh} % 0 
   \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhhhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhhhhhhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhhhhhhhhhh} % 7
   \or
   \newcolumntype{f}{ddddddddhhhhhhhh} % 8
   \or
   \newcolumntype{f}{ddd ddd ddd hhhhhh} % 9
   \or
   \newcolumntype{f}{ddddddddddhhhh} % 10
   \or
   \newcolumntype{f}{ddddd ddddd dhh} % 11
   \or
   \newcolumntype{f}{ddddd ddddd dd} % 12
   \fi
%%%%%%%%%%%%%%%%%%
\or %25
\ifcase\firsthowmanyDs
   \newcolumntype{f}{hhhhh hhhhh hhhhh hhhhh hhhhh} % 0 
   \or
   \newcolumntype{f}{dhhhhhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{f}{ddhhhhhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{f}{dddhhhhhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{f}{ddddhhhhhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{f}{dddddhhhhhhhhhhhhhhh} % 5
   \or
   \newcolumntype{f}{ddddddhhhhhhhhhhhhh} % 6
   \or
   \newcolumntype{f}{dddddddhhhhhhhhhhh} % 7
   \or
   \newcolumntype{f}{ddddddddhhhhhhhhh} % 8
   \or
   \newcolumntype{f}{ddd ddd ddd hhhhhhh} % 9
   \or
   \newcolumntype{f}{ddddddddddhhhhh} % 10
   \or
   \newcolumntype{f}{dddddddddddhhh} % 11
   \or
   \newcolumntype{f}{ddddddddddddh} % 12
   \fi
%%%%%%%%%%%%%%%%%%
\else
\typeout{^^J----------------------------------------------------- ^^J
Warning!^^J^^J 
Too many Columns using in Splittabular. 
^^J
25 column maximum in
each part of the table.^^J Each`T' counts as two columns.
^^J-----------------------------------------------------^^J}
\fi
}


%% Second dummy columns is the part of the table that will not print,
%% in this case the part to the right.
%%\makeseconddummycolumns

\gdef\makeseconddummycolumns{%
\ifcase\secondcolcount
%0
\or
%1
\newcolumntype{F}{h} % no D possible
%%%%%%%%%%%%%%%%%%
\or
%2
  \ifcase\secondhowmanyDs
  \newcolumntype{F}{hh}%% 0
  \or
  \newcolumntype{F}{d} %% 1
  \fi
%%%%%%%%%%%%%%%%%%
\or
%3
  \ifcase\secondhowmanyDs
  \newcolumntype{F}{hhh} % 0 
  \or
  \newcolumntype{F}{dh} % 1 
  \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhh} % 0 
    \or
   \newcolumntype{F}{dhh} % 1
   \or
   \newcolumntype{F}{dd} % 2 
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhh} % 0 
    \or
   \newcolumntype{F}{dhhh} % 1
   \or
   \newcolumntype{F}{ddh} % 2 
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhh} % 1
   \or
   \newcolumntype{F}{ddhh} % 2 
   \or
   \newcolumntype{F}{ddd} % 3
   \fi
%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhh} % 2 
   \or
   \newcolumntype{F}{dddh} % 3
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhh} % 2 
   \or
   \newcolumntype{F}{dddhh} % 3
   \or
   \newcolumntype{F}{dddd} % 4
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhh} % 3
   \or
   \newcolumntype{F}{ddddh} % 4
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhh} % 3
   \or
   \newcolumntype{F}{ddddhh} % 4
   \or
   \newcolumntype{F}{ddddd} % 5
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhh} % 4
   \or
   \newcolumntype{F}{dddddh} % 5
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhh} % 4
   \or
   \newcolumntype{F}{dddddhh} % 5
   \or
   \newcolumntype{F}{dddddd} % 6
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhh} % 5
   \or
   \newcolumntype{F}{ddddddh} % 6
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhh} % 6
   \or
   \newcolumntype{F}{ddddddd} % 7
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhh} % 6
   \or
   \newcolumntype{F}{dddddddh} % 7
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhh} % 7
   \or
   \newcolumntype{F}{dddddddd} % 8
\fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhh} % 7
   \or
   \newcolumntype{F}{ddddddddh} % 8
\fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhhh} % 7
   \or
   \newcolumntype{F}{ddddddddhh} % 8
   \or
   \newcolumntype{F}{ddddddddd} % 9
\fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhhhhhhhhhhhhhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhhhh} % 7
   \or
   \newcolumntype{F}{ddddddddhhh} % 8
   \or
   \newcolumntype{F}{dddddddddh} % 9
\fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhh hhhhh hhhhh hhhhh} % 0 
    \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhhhhh} % 7
   \or
   \newcolumntype{F}{ddddddddhhhh} % 8
   \or
   \newcolumntype{F}{dddddddddhh} % 9
   \or
   \newcolumntype{F}{dddddddddd} % 10
   \fi
%%%%%%%%%%%%%%%%%%
\or
  \ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhh hhhhh hhhhh hhhhh h} % 0 
   \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhhhhhh} % 7
   \or
   \newcolumntype{F}{ddddddddhhhhh} % 8
   \or
   \newcolumntype{F}{ddd ddd ddd hhh} % 9
   \or
   \newcolumntype{F}{ddd ddd ddd d h} % 10
   \fi
%%%%%%%%%%%%%%%%%%
\or
\ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhh hhhhh hhhhh hhhhh hh} % 0 
   \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhhhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhhhhhhh} % 7
   \or
   \newcolumntype{F}{ddddddddhhhhhh} % 8
   \or
   \newcolumntype{F}{ddd ddd ddd hhhh} % 9
   \or
   \newcolumntype{F}{ddddddddddhh} % 10
   \or
   \newcolumntype{F}{ddddddddddd} % 11
   \fi
%%%%%%%%%%%%%%%%%%
\or
\ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhh hhhhh hhhhh hhhhh hhh} % 0 
   \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhhhhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhhhhhhhh} % 7
   \or
   \newcolumntype{F}{ddddddddhhhhhhh} % 8
   \or
   \newcolumntype{F}{ddd ddd ddd hhhhh} % 9
   \or
   \newcolumntype{F}{ddddddddddhhh} % 10
   \or
   \newcolumntype{F}{ddddd ddddd dh} % 11
   \fi
%%%%%%%%%%%%%%%%%%
\or
\ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhh hhhhh hhhhh hhhhh hhhh} % 0 
   \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhhhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhhhhhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhhhhhhhhh} % 7
   \or
   \newcolumntype{F}{ddddddddhhhhhhhh} % 8
   \or
   \newcolumntype{F}{ddd ddd ddd hhhhhh} % 9
   \or
   \newcolumntype{F}{ddddddddddhhhh} % 10
   \or
   \newcolumntype{F}{ddddd ddddd dhh} % 11
   \or
   \newcolumntype{F}{ddddd ddddd dd} % 12
   \fi
%%%%%%%%%%%%%%%%%%
\or
\ifcase\secondhowmanyDs
   \newcolumntype{F}{hhhhh hhhhh hhhhh hhhhh hhhhh} % 0 
   \or
   \newcolumntype{F}{dhhhhhhhhhhhhhhhhhhhhhhh} % 1
   \or
   \newcolumntype{F}{ddhhhhhhhhhhhhhhhhhhhhh} % 2 
   \or
   \newcolumntype{F}{dddhhhhhhhhhhhhhhhhhhh} % 3
   \or
   \newcolumntype{F}{ddddhhhhhhhhhhhhhhhhh} % 4
   \or
   \newcolumntype{F}{dddddhhhhhhhhhhhhhhh} % 5
   \or
   \newcolumntype{F}{ddddddhhhhhhhhhhhhh} % 6
   \or
   \newcolumntype{F}{dddddddhhhhhhhhhhh} % 7
   \or
   \newcolumntype{F}{ddddddddhhhhhhhhh} % 8
   \or
   \newcolumntype{F}{ddd ddd ddd hhhhhhh} % 9
   \or
   \newcolumntype{F}{ddddddddddhhhhh} % 10
   \or
   \newcolumntype{F}{ddddd ddddd dhhh} % 11
   \or
   \newcolumntype{F}{ddddd ddddd ddh} % 12
   \fi
%%%%%%%%%%%%%%%%%%
\else
\typeout{^^J----------------------------------------------------- ^^J
Warning!^^J^^J 
Too many Columns using in Splittabular. 
^^J
25 column maximum in
each part of the table.^^J Each`D' counts as two columns.
^^J-----------------------------------------------------^^J}
\fi
}

%% we don't have to worry about D's in this section, fortunately.
\gdef\makethirddummycolumns{
\ifcase\thirdcolcount 
\or\newcolumntype{E}{h}
\or\newcolumntype{E}{hh}
\or\newcolumntype{E}{hhh}
\or\newcolumntype{E}{hhhh}
\or\newcolumntype{E}{hhhhh}
\or\newcolumntype{E}{hhhhhh}
\or\newcolumntype{E}{hhhhhhh}
\or\newcolumntype{E}{hhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhhhhhhhhh}
\or\newcolumntype{E}{hhhhhhhhhhhhhhhhhhhhhhhhh} %% up to 25 columns
\else \typeout{^^J-----------------------------------------------------^^J
Warning!
^^J^^J Too many Columns in Splittabular. 
^^J
25 column maximum in
each part of the table.^^J
Each `D' counts as two columns.
^^J-----------------------------------------------------^^J}\fi
}

%%%%%%%%%%%%%%%%%
%% Colnums

%% defaults

\gdef\settabnumdefaults{%
\loopnum=0\relax\loop%
\ifnum\loopnum<31\relax%
\expandafter\gdef\csname tab\the\loopnum\endcsname{%
\multicolumn1{c}{\global\advance\colnumsused by 1\relax%
(\the\colnumsused)}}%
\global\advance\loopnum by1\repeat}

%%%%%%%%%%%%%%%%%%%%%%%%%
%% this version of countcols  is for \colnumbers
\newcount\totalcolumncount
\def\xH{h}

\makeatother
\def\tabcountcols#1{\ifcat#1c
\global\advance\colcount by 1\relax\fi%
%%
\def\lookforD{#1}%
\ifx\lookforD\xD%
\expandafter\gdef\csname tab\the\colcount\endcsname{%
\multicolumn2{c}{\global\advance\colnumsused by 1\relax%
(\the\colnumsused)}}%
\fi%
\ifx\lookforD\xH%
\expandafter\gdef\csname tab\the\colcount\endcsname{%
\multicolumn1{h}{}}%
\fi%
%%
\futurelet\next\tablookatnext}

\def\tabxeatone#1{\tabcountcols}

%% second conditional deals with expressions like @{} in
%% the preamble.
\def\tablookatnext{\if\next\xrelax\let\xgo\relax\else%
\ifx\next\bgroup\let\xgo\tabxeatone\else\let\xgo\tabcountcols\fi\fi\xgo}
\makeatletter
%%%%%%%%%%%%%%%

\newcount\totalcolumns
\newbox\firsttablebox
\newbox\secondtablebox
\newbox\thirdtablebox
\newif\ifdbreaktab

%% A loop would be more elegant, of course, but using & in a loop
%% within a table column produces errors.
%% was\tabnumberline, now \colnumbers

%% Is this test necessary?
%\newif\ifcolumnums

\newif\ifcolnumberson
\def\deluxecolnumbers{\global\colnumbersontrue}

\def\colnumbers{\omit\\\omit\xcolnumbers}
\def\xcolnumbers{\global\let\colnumbers\xcolnumbers%
\global\colnumbersontrue\let&\CheckNumberAndSwitch%
\global\colcount=0\relax%
\global\totalcolumncount=0\relax%
%% above here
\settabnumdefaults%
%% Now, change the columns that have `D' to be \multicolumn2c, by redefining
%% the \csname tab<number>\endcsname
%%
\expandafter\tabcountcols\tabfullpreamble%
\global\totalcolumncount=\the\colcount\relax%
\global\colcount=0\relax%
\ifbreaktab\else\ifdbreaktab\else\global\colnumsused=0\relax\fi\fi%
%%
%
\ifdeluxe\ifbreaktab\\\else\\[-6pt]\fi%\hline% not above
\else%
\\[-14pt]\fi%
\ifcase\totalcolumncount%
\or%
\csname tab1\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname%
\or
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname%
\or%22
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname%
\or% 24
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname%
\or% 25
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname%
\or% 
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname%
\or% 27
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname%
\or% 28
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname%
\or% 29
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname&%
\csname tab29\endcsname%
\or% 30
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname&%
\csname tab29\endcsname&\csname tab30\endcsname%
\else%
\typeout{^^J^^J
Sorry, more than 30 columns cannot be numbered with^^J
\string\colnumbers. Please number the columns manually.^^J
Thank you!^^J}\fi%
%\ifbreaktab
\\\hline% below, hline is wanted
%\else
\noalign{\vskip-13pt}
%\vrule height 28pt width0pt %the \vrule is in the line below colnumbers; 
                          % it causes all the vrules on that line to grow to 28pt.
                          % The -14pt will cause the lower line to overlap the upper line.
                          % Complication, needed to add this vrule to Z and z in order to
                          % have it also work for split tabular.
%\fi
}


\let\savecolnumbers\colnumbers

\gdef\plaincolnumbers{%
\omit\\\omit%
\global\colnumbersontrue\let&\CheckNumberAndSwitch%
\global\colcount=0\relax%
\global\totalcolumncount=0\relax%
%% above here
\settabnumdefaults%
%% Now, change the columns that have `D' to be \multicolumn2c, by redefining
%% the \csname tab<number>\endcsname
%%
\expandafter\tabcountcols\tabfullpreamble%
\global\totalcolumncount=\the\colcount\relax%
\global\colcount=0\relax%
\ifbreaktab\else\ifdbreaktab\else\global\colnumsused=0\relax\fi\fi%
%%
\\\ifcase\totalcolumncount%
\or%
\csname tab1\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname%
\or
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname%
\or%22
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname%
\or% 24
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname%
\or% 25
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname%
\or% 
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname%
\or% 27
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname%
\or% 28
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname%
\or% 29
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname&%
\csname tab29\endcsname%
\or% 30
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname&%
\csname tab29\endcsname&\csname tab30\endcsname%
\else%
\typeout{^^J^^J
Sorry, more than 30 columns cannot be numbered with^^J
\string\colnumbers. Please number the columns manually.^^J
Thank you!^^J}\fi%
\\\hline% below, hline is wanted
%\\\noalign{\vskip-30pt}
%\vrule height 28pt width0pt %the \vrule is in the line below colnumbers; 
                          % it causes all the vrules on that line to grow to 28pt.
                          % The -14pt will cause the lower line to overlap the upper line.
                          % Complication, needed to add this vrule to Z and z in order to
                          % have it also work for split tabular.
}

\let\saveplaincolnumbers\plaincolnumbers


\def\splitplaincolnumbers{%
\omit\\\omit%
\xplain}

\let\savesplitplaincolnumbers\splitplaincolnumbers

\newif\iffirsttime
\firsttimetrue
\gdef\xplain{%
\global\let\plaincolnumbers\xplain%
\global\colnumbersontrue\let&\CheckNumberAndSwitch%
\global\colcount=0\relax%
\global\totalcolumncount=0\relax%
%% above here
\settabnumdefaults%
%% Now, change the columns that have `D' to be \multicolumn2c, by redefining
%% the \csname tab<number>\endcsname
%%
\expandafter\tabcountcols\tabfullpreamble%
\global\totalcolumncount=\the\colcount\relax%
\global\colcount=0\relax%
\ifbreaktab\else\ifdbreaktab\else\global\colnumsused=0\relax\fi\fi%
%%
\iffirsttime
\\%\hline% above not wanted
\else
\\[-12pt]%\hline% above not wanted
\fi
\ifcase\totalcolumncount%
\or%
\csname tab1\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname%
\or
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname%
\or%22
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname%
\or%
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname%
\or% 24
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname%
\or% 25
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname%
\or% 
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname%
\or% 27
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname%
\or% 28
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname%
\or% 29
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname&%
\csname tab29\endcsname%
\or% 30
\csname tab1\endcsname&\csname tab2\endcsname&%
\csname tab3\endcsname&\csname tab4\endcsname&%
\csname tab5\endcsname&\csname tab6\endcsname&%
\csname tab7\endcsname&\csname tab8\endcsname&%
\csname tab9\endcsname&\csname tab10\endcsname&%
\csname tab11\endcsname&\csname tab12\endcsname&%
\csname tab13\endcsname&\csname tab14\endcsname&%
\csname tab15\endcsname&\csname tab16\endcsname&%
\csname tab17\endcsname&\csname tab18\endcsname&%
\csname tab19\endcsname&\csname tab20\endcsname&%
\csname tab21\endcsname&\csname tab22\endcsname&%
\csname tab23\endcsname&\csname tab24\endcsname&%
\csname tab25\endcsname&\csname tab26\endcsname&%
\csname tab27\endcsname&\csname tab28\endcsname&%
\csname tab29\endcsname&\csname tab30\endcsname%
\else%
\typeout{^^J^^J
Sorry, more than 30 columns cannot be numbered with^^J
\string\colnumbers. Please number the columns manually.^^J
Thank you!^^J}\fi%
\\\hline% below, hline is wanted
\iffirsttime
\\\noalign{\vskip-30pt}
\vrule height 28pt width0pt %the \vrule is in the line below colnumbers; 
                          % it causes all the vrules on that line to grow to 28pt.
                          % The -14pt will cause the lower line to overlap the upper line.
                          % Complication, needed to add this vrule to Z and z in order to
                          % have it also work for split tabular.
\fi\global\firsttimefalse
}

\def\decimalcolnumbers{\crcr\saveplaincolnumbers\\%
\noalign{\global\savetabskip\tabskip
\tabskip=0pt
\global\let\extracolsep\relax
\global\let\ddoit\saveddoit
\global\let\zdoit\savezdoit
\vskip-32pt
}{\vrule height28pt depth0pt width0pt}\relax}

\def\splitdecimalcolnumbers{\\[-15pt]%
\savesplitplaincolnumbers
\global\colnumbersonfalse
\\\noalign{\vskip-32pt %!!!
\global\savetabskip\tabskip
\tabskip=0pt
\global\let\extracolsep\relax
\global\let\ddoit\saveddoit
\global\let\zdoit\savezdoit
}\vrule height 28pt width0pt %!!!
\global\let\splitdecimalcolnumbers\secondsplitdecimalcolnumbers}

\def\secondsplitdecimalcolnumbers{\omit\\%[-15pt]
\plaincolnumbers
\global\colnumbersonfalse
\\\noalign{\vskip-26pt %!!! was -28pt
\global\savetabskip\tabskip
\tabskip=0pt
\global\let\extracolsep\relax
\global\let\ddoit\saveddoit
\global\let\zdoit\savezdoit
}\\\relax}

\let\savesplitdecimalcolnumbers\splitdecimalcolnumbers

%%AAA
\newif\ifdeluxestar
\expandafter\def\csname
deluxetable*\endcsname{\onecolumngrid\deluxestartrue\bgroup
\hsize=\textwidth
\deluxetable}

\expandafter\def\csname enddeluxetable*\endcsname{\enddeluxetable
\egroup
\global\colnumbersonfalse
\global\deluxedecimalsfalse
\global\deluxestarfalse
\if@two@col
\hsize=\columnwidth
\twocolumngrid\fi
}

\def\deluxedecimalcolnumbers{\deluxedecimalstrue\colnumbersontrue}
\def\deluxetablecaption#1{\gdef\@tablecaption{#1}}

\newif\iffloat
\def\floattable{\global\deluxestartrue\global\floattrue}

\def\deluxetable{\global\deluxetrue
\catcode`\&=\active
\catcode`\$=\active
\let\tablecaption\deluxetablecaption
\deluxetablecaption{}
\iffloat
\let\go\ydeluxetable
\else\let\go\longdeluxetable\fi\go}

\def\ydeluxetable#1{%
\@ifnextchar[{\xdeluxetable{#1}}{\zdeluxetable{#1}}}

% x and z are the same, except that xdeluxetable allows optional square bracket arg, like [h].
\def\xdeluxetable#1[#2]{\global\breaktabtrue
\let\colnumbers\deluxecolnumbers
\global\deluxedecimalsfalse
\let\decimals\deluxedecimals
\let\decimalcolnumbers\deluxedecimalcolnumbers
\let\tablehead\ztablehead
\gdef\tabfullpreamble{#1}%<<== needed
\def\endtabular{\endarray
\global\let\zdoit\relax
\global\let\tdoit\relax
\global\Dfalse}
\global\colnumsused=0
\global\breaktabtrue
\global\colcount=0
%%
%%%%%%%%%%%%
  \lineskiplimit=\z@ % restore default setting
\ifdeluxestar
\begin{table*}[#2]\hsize=\textwidth\else
  \begin{table}[#2]\fi
\noindent\setbox\splitbox=\vtop\bgroup%
\currtabletypesize
    \vspace*{\abovedeluxetableskip}
%%
\def\startdata##1\enddata{%
\currtabletypesize
\setbox0=\hbox{
\begin{tabular}{#1}
\pt@head
##1\end{tabular}}
\expandafter\ifx\csname @tablecaption\endcsname\empty\else

\noindent\hbox
to\hsize{\hss\vtop{\hsize=\wd0
\caption{\@tablecaption}}\hss}\vskip3pt\fi
\global\setbox3\hbox{%
\begin{tabular}{#1}%
\hline\hline\noalign{\vskip-9pt}%
\pt@head%
##1\omit\\\omit\\\hline\end{tabular}}

%\noindent\hskip-1.25em %% kludge! but seems necessary
\hbox to\hsize{\hss\copy3\hss}
\global\setbox4\vtop\bgroup\ifdeluxestar\hsize=\textwidth\else\hsize=\wd0\fi\leftskip6pt\parindent-6pt
}}

\def\zdeluxetable#1{\global\breaktabtrue
\let\colnumbers\deluxecolnumbers
\global\deluxedecimalsfalse
\let\decimals\deluxedecimals
\let\decimalcolnumbers\deluxedecimalcolnumbers
\let\tablehead\ztablehead
\gdef\tabfullpreamble{#1}%<<== needed
\def\endtabular{\endarray
\global\let\zdoit\relax
\global\let\tdoit\relax
\global\Dfalse}
\global\breaktabtrue
\global\colnumsused=0
\global\colcount=0
%%
%%%%%%%%%%%%
  \lineskiplimit=\z@ % restore default setting
\ifdeluxestar
\begin{table*}\hsize=\textwidth\else
  \begin{table}\fi
\noindent\setbox\splitbox=\vtop\bgroup%
\currtabletypesize
    \vspace*{\abovedeluxetableskip}
%%
\def\startdata##1\enddata{%
\currtabletypesize
\setbox0=\hbox{\colnumbersonfalse
\begin{tabular}{#1}
\pt@head
##1\end{tabular}}
\expandafter\ifx\csname @tablecaption\endcsname\empty\else

\noindent\hbox
to\hsize{\hss\vtop{\hsize=\wd0
\caption{\@tablecaption}}\hss}\vskip3pt\fi
\global\setbox3\hbox{%
\begin{tabular}{#1}%
\hline\hline\noalign{\vskip-9pt}%
\pt@head%

##1\omit\\\omit\\\hline\end{tabular}}
\vskip-\parskip
\noindent\hbox to\hsize{\hss\copy3\hss}
\global\setbox4\vtop\bgroup\hsize=\wd0\relax
\leftskip6pt\parindent-6pt
}}

%% go to \end{deluxetable}, after longdeluxetable

\newbox\longtablebox
\def\longdeluxetable#1{
\global\rotateonfalse
\let\colnumbers\deluxecolnumbers
\global\deluxedecimalsfalse
\let\decimals\deluxedecimals
\let\decimalcolnumbers\deluxedecimalcolnumbers
\let\tablehead\ztablehead
\gdef\tabfullpreamble{#1}%<<== needed
\def\endtabular{\endarray
\global\let\zdoit\relax
\global\let\tdoit\relax
\global\Dfalse}
\global\colnumsused=0
\global\colcount=0
%%
%%%%%%%%%%%%
  \lineskiplimit=\z@ % restore default setting
\let\enddeluxetable\endlongdeluxetable
    \vspace*{\abovedeluxetableskip}
%%
\def\startdata##1\enddata{%
\global\setbox\longtablebox=\hbox{\currtabletypesize
\tabcolsep=3pt
\begin{tabular}{#1}
\pt@head
##1\end{tabular}}
%%%
\bgroup\centering
\def\table@hook{\currtabletypesize}
      \LTcapwidth=\wd\longtablebox
\begin{longtable}{#1}
\caption{\hsize=\wd\longtablebox 
\advance\baselineskip2pt
\@tablecaption}\\
\hline\hline\noalign{\vskip-9pt}% prob ok
\pt@head%
\endfirsthead

\noalign{\centerline{\small 
Table \thetable\ \it(continued)}\vskip6pt}
\hline\hline\\[-20pt]
\pt@head%
\endhead

\\[-3pt]
\hline
\\[-8pt]
\multicolumn{\totalcolumns}{r}{\small\it Table \thetable\
continued  \if@two@col\else on next page\fi}\\ 
\endfoot

\\[-6pt]
\hline
\endlastfoot

##1
\end{longtable}
\egroup
\global\setbox4=\vtop\bgroup\hsize=\wd\longtablebox
\leftskip=6pt
\parindent=-6pt
\currtabletypesize
}% end data, endlongtable
}


\def\endlongdeluxetable{\vrule depth 6pt width 0pt
\vskip1sp\egroup
\ifdim\dp4>6pt
\vglue-1.5\baselineskip
\hbox to \columnwidth{\hfill\copy4\hfill}\fi
\vglue2\baselineskip
\global\colnumbersonfalse
\global\deluxedecimalsfalse
\global\rotateonfalse
\global\floatfalse
\relax}

%% bbb
%% \end{deluxetable}:
\def\enddeluxetable{%
\egroup%end box 4
\ifdim\dp4>6pt
\hbox to\hsize{\hss\copy4\hss}
\fi
\egroup% end splitbox
\centering
\ifrotateon\global\rotateonfalse
\rotatebox{90}{\hbox to\textheight{\hfill\vbox{
\unvbox\splitbox
\vspace*{\belowdeluxetableskip}}\hfill}}
\else
\centering\unvbox\splitbox
\vspace*{\belowdeluxetableskip}
\vspace*{24pt}
\fi
\ifdeluxestar\end{table*}\global\deluxestarfalse\else\end{table}\fi
\gdef\colnumbers{\saveplaincolnumbers}
\global\breaktabfalse
\global\deluxefalse
\global\colnumbersonfalse
\global\deluxedecimalsfalse
\global\rotateonfalse
\global\floatfalse
}



\newbox\splitbox
\newif\ifdeluxe
\def\splitdeluxetable{\global\deluxetrue\catcode`\&=\active
\catcode`\$=\active
\xsplitdeluxetable}

\def\xsplitdeluxetable#1{\global\breaktabtrue
\let\colnumbers\deluxecolnumbers
\let\decimals\deluxedecimals
\global\deluxedecimalsfalse
\let\decimalcolnumbers\deluxedecimalcolnumbers
\let\tablehead\xtablehead
\gdef\tabfullpreamble{#1}
\def\endtabular{\endarray
\global\let\zdoit\relax
\global\let\tdoit\relax
\global\Dfalse}
%%% modification of emulateapj \deluxetable %%%%
\setbox0=\hbox{\def\firstarg{#1}\expandafter\lookforD\firstarg}
\gdef\temppreamble{#1\relax}
\PartsOfTable=0
\def\tempfullpreamble{#1BB}
\setbox0=\hbox{\expandafter\checkforB\tempfullpreamble}
%% Now PartsOfTable is either =2 or 3
%%
\gdef\one{#1Z}
\global\colnumsused=0
\global\breaktabtrue
\global\colcount=0
%%
\ifnum\PartsOfTable=3
\let\multicolumn\threebreakmulticolumn
\else
\let\multicolumn\breakmulticolumn
\fi
%%%%%%%%%%%%
  \lineskiplimit=\z@ % restore default setting
  \gdef\tblnote@list{}
  \gdef\tblref@list{}
\ifsplitstar
  \begin{table*}\else
  \begin{table}\fi
\noindent\setbox\splitbox=\vtop\bgroup%
\currtabletypesize
    \vspace*{\abovedeluxetableskip}
\let\startdata\xstartdata}
%% \end{splitdeluxetable}:

\def\endsplitdeluxetable{\vskip1sp\egroup%% end box with tablenotes
\hbox to \hsize{\hss\copy4\hss}
\egroup
\vskip1pt
\ifrotateon\global\rotateonfalse
\rotatebox{90}{\vbox{
\noindent\unvbox\splitbox
\vspace*{\belowdeluxetableskip}
}}\else
\noindent\unvbox\splitbox
\vspace*{\belowdeluxetableskip}
\fi
\ifsplitstar
\end{table*}
\else
\end{table}\fi
\gdef\colnumbers{\saveplaincolnumbers}
\tabletypesize{\small}
\global\breaktabfalse
\global\deluxefalse
\global\splitstarfalse
\global\colnumbersonfalse
\global\deluxedecimalsfalse
}

\newif\ifsplitstar
\expandafter\def\csname splitdeluxetable*\endcsname{\global\splitstartrue\splitdeluxetable}
\expandafter\def\csname endsplitdeluxetable*\endcsname{\endsplitdeluxetable\global\splitstarfalse 
\global\colnumbersonfalse
\global\deluxedecimalsfalse
}

\newdimen\maxtablewidth

\gdef\NoTableCaption{\global\@table@not@headedtrue}

%% For splitdeluxetable

%%% ====================================================== %%%
\gdef\dbreaktabular{\catcode`\&=\active
\let&\CheckNumberAndSwitch\xdbreaktabular}

\long\gdef\xdbreaktabular#1{%% #1=contents of table
{\global\maxtablewidth=0pt
\let&\CheckNumberAndSwitch
\let$\savedollar
\global\colnumsused=0
%%
\global\dbreaktabtrue
\global\colcount=0
%%
\ifnum\PartsOfTable=3
\let\multicolumn\threebreakmulticolumn
\else
\let\multicolumn\breakmulticolumn
\fi
%%
\ifnum\PartsOfTable=3
\expandafter\catchfirstpreamble\one
\expandafter\catchsecondofthreepreamble\one
\expandafter\catchthirdpreamble\one
\makefirstdummycolumns
\makeseconddummycolumns
\makethirddummycolumns
\else %% PartsOfTable=2
\ifnum\PartsOfTable=2
\expandafter\catchfirstpreamble\one
\expandafter\catchsecondpreamble\one
\makefirstdummycolumns
\makeseconddummycolumns
\fi\fi
%%%
\global\columncount=0
\ifnum\PartsOfTable=3
%% For table divided into three parts
\firstboxtrue\secondboxfalse\thirdboxfalse
\setbox\firsttablebox=\hbox{%
\begin{tabular}{AFE}
\ifx\csname pt@head\endcsname\relax\else
\pt@head\fi#1\crcr\end{tabular}}
\global\maxtablewidth=\wd\firsttablebox

\noindent\hbox to \hsize{\hss\unhbox\firsttablebox\hss}
\vskip6pt
\hrule
\vskip6pt
\global\columncount=0
\firstboxfalse\secondboxtrue\thirdboxfalse
\ifcolnumberson%
\setbox\secondtablebox=\hbox{%
\let\savecolnumbers\xcolnumbers%
\notfirsttrue%
\begin{tabular}{fZE} %fZE !!!
\ifx\csname pt@head\endcsname\relax\else%
\pt@head\fi#1\crcr\end{tabular}}
\else
%%
\setbox\secondtablebox=\hbox{%
\begin{tabular}{fZE}%
\ifx\csname pt@head\endcsname\relax\else%
\pt@head\fi\\[-14pt]%
#1\crcr%
\end{tabular}}
\fi
\ifdim\wd\secondtablebox>\maxtablewidth
\global\maxtablewidth\wd\secondtablebox\fi

\noindent\hbox to \hsize{\hss\unhbox\secondtablebox\hss}
%%
\vskip6pt
\hrule
\vskip6pt
\firstboxfalse\secondboxfalse\thirdboxtrue
\global\columncount=0
\ifcolnumberson
\notfirsttrue
\setbox\thirdtablebox=\hbox{
\let\savecolnumbers\xcolnumbers
\notfirsttrue
\begin{tabular}{fFz}
\ifx\csname pt@head\endcsname\relax\else
\pt@head\fi#1\crcr\end{tabular}}
\else
\setbox\thirdtablebox=\hbox{
\begin{tabular}{fFz}
\ifx\csname pt@head\endcsname\relax\else
\pt@head\fi\\[-14pt]
#1
\end{tabular}}
\fi
\ifdim\wd\thirdtablebox>\maxtablewidth
\global\maxtablewidth\wd\thirdtablebox\fi

\noindent\hbox to \hsize{\hss\unhbox\thirdtablebox\hss}
\vskip6pt
\hrule
\else
%% Original for table divided into two parts
\global\columncount=0
\secondboxfalse
\setbox\firsttablebox=\hbox{%
\begin{tabular}{AF}
\ifx\csname pt@head\endcsname\relax\else
\pt@head\fi%
#1\end{tabular}}
\global\maxtablewidth=\wd\firsttablebox
\hbox to \hsize{\hss\unhbox\firsttablebox\hss}
\vskip6pt
\hrule
\vskip6pt
\global\columncount=0
\secondboxtrue
\ifcolnumberson
\setbox\secondtablebox=\hbox{%
\let\savecolnumbers\xcolnumbers
\notfirsttrue
\begin{tabular}{fZ}%%
\ifx\csname pt@head\endcsname\relax\else%
\pt@head\fi#1\crcr%%= tablecontents
\end{tabular}}%
\else
\setbox\secondtablebox=\hbox{%
\begin{tabular}{fZ}%% was 12
\ifx\csname pt@head\endcsname\relax\else%
\pt@head\fi\\[-14pt]%
#1%%= tablecontents
\end{tabular}}\fi
\ifdim\wd\secondtablebox>\maxtablewidth
\global\maxtablewidth\wd\secondtablebox\fi

\noindent\hbox to \hsize{\hss\unhbox\secondtablebox\hss}
\vskip6pt
\hrule
\secondboxtrue
%%
%% end of conditional testing for 2 or 3 part table:
\fi
%%
\global\dbreaktabfalse}}
%%% ====================================================== %%%

\newif\ifnotfirst
%% @@@@
%% xstartdata, modified from \startdata in emulateapj, for splitdeluxetable
\gdef\xstartdata#1\enddata{\def\tablecontents{%
\ifcolnumberson%
\\ \savecolnumbers\\[2pt]\fi%
\ifdeluxedecimals\savedecimals\fi%
#1}%
\currtabletypesize%
\setbox2=\vtop{\dbreaktabular{\tablecontents}}%
%
\expandafter\ifx\csname @tablecaption\endcsname\empty\else

\noindent\hbox
to\hsize{\hss\vtop{\hsize=\maxtablewidth\caption{\@tablecaption}}\hss}\vskip3pt\fi
\dbreaktabular{\tablecontents\noalign{\global\let\zdoit\relax
\global\let\ddoit\relax}}
\parindent=0pt
\global\setbox4=\vtop\bgroup% egroup in enddeluxetable
\currtabletypesize
\ifsplitstar
\hsize=\textwidth\else\hsize=245.3pt\fi
\parindent=-6pt \leftskip=6pt 
}

\def\tablecomments#1{{\small\vskip3pt\indent\vrule height 11pt depth 2pt
width 0pt\currtabletypesize{\sc Note}---{#1}\vskip1sp}}

\def\tablenotetext#1#2{{\currtabletypesize\vskip1pt\indent\vrule
height 11pt depth
2pt width0pt\relax$^{\hbox to 5pt{$#1$}}$#2\vskip1sp}}

\def\tablerefs#1{{\small\vskip3pt\indent\vrule height 11pt depth 2pt
width 0pt\currtabletypesize{\bf References}---{#1}\vskip1sp}}

\let\tablereferences\tablerefs

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% 5. Simple entry decimal numbers

%% extra unused args are to get rid of unnecessary commands revtex puts in
\gdef\lookfordecimal#1#2#3#4.#5 {\gdef\xone{{\let$\relax\savedollar#4\savedollar}}
\gdef\xtwo{#5}\ifx\xtwo\empty\else\gdef\xtwo{{\let$\relax\savedollar#5\savedollar}}\fi}

%% D +++
\def\newdoit{\setbox0=\hbox\bgroup\zdoit}
\def\endnewdoit{\egroup\unhbox0}

\long\gdef\zdoit#1 {\let$\relax
\def\bothcols{#1 }%
\setbox0=\hbox{\let$\relax\expandafter\lookfordecimal\bothcols{}{}{}{}.{} }%
\xdoit}

\global\let\savezdoit\zdoit
\let\zdoit\relax

\gdef\xdoit{\relax\hskip\tabcolsep\relax\xone&\tabskip=0pt%
\ifx\xtwo\empty\let\go\relax\else.\def\go{\xtwo}\fi\go}

%% d
\def\xnewdoit{\setbox0=\hbox\bgroup\ddoit}
\def\xendnewdoit{\egroup}

\long\gdef\ddoit#1#2#3{\xxdoit}
\let\saveddoit\ddoit
\let\ddoit\relax

%% to delete all D material: 

\gdef\xxdoit{&}

\newcolumntype{D}{>\newdoit r<{\endnewdoit} @{}l}%% 
\newcolumntype{d}{>\xnewdoit h<{\xendnewdoit} @{}h} %% 

\newdimen\savetabskip
\gdef\decimals{\noalign{\global\savetabskip\tabskip
\tabskip=0pt
\global\let\extracolsep\relax
\global\let\ddoit\saveddoit
\global\let\zdoit\savezdoit
}}

\let\savedecimals\decimals

%% to allow \decimals to be used before \startdata:
\newif\ifdeluxedecimals
\def\deluxedecimals{\global\deluxedecimalstrue}

%%%%%%%%%
%% This needed to be redefined so that \colhead in \tablehead would span both parts
%% of decimal numbers.

%% in case there isn't a table head
\let\pt@head\relax

%% for splitdeluxetable
\def\xtablehead{\catcode`\&=\active%
\catcode`\$=\active%
\xxtablehead}

\def\xxtablehead#1{%
\let&\CheckNumberAndSwitch%
\gdef\pt@head{%
\hline\hline%
\multicolumn1c{
\tempcounter=0
\expandafter\getDs\tabfullpreamble\relax
\global\colcount=0
\global\colheadcount=0}%
\\[-10pt]% this is right
#1\ifcolnumberson%
\ifnotfirst\\[-22pt]\fi
\else\\\hline\fi%
}%
%
}

\def\ztablehead{\catcode`\&=\active%
\catcode`\$=\active%
\zztablehead}

%%+++
\def\zztablehead#1{\let&\CheckNumberAndSwitch%
\gdef\pt@head{%
%% this is needed:
\multicolumn1c{\expandafter\getDs\tabfullpreamble\relax
\global\tempcounter=0
\global\colcount=0
\global\colheadcount=0}\\%
#1
\ifcolnumberson\\[6pt]
\savecolnumbers\vrule height 11pt depth 4pt width 0pt\relax%
\\[6pt]
\ifdeluxedecimals\\[-14pt] % ??? 
\savedecimals\fi
\else \vrule depth 6pt width0pt\\\ifdeluxedecimals\savedecimals\fi\hline\fi%
}}


\newcount\tempcounter

\gdef\getDs#1{\let&\CheckNumberAndSwitch
\def\lookforD{#1}%
\ifx\lookforD\xD%
\expandafter\gdef\csname col\the\tempcounter\endcsname{2}\else
\expandafter\gdef\csname col\the\tempcounter\endcsname{1}\fi 
\global\advance\tempcounter by 1
\futurelet\next\checkingpreamble}

\gdef\checkingpreamble{\ifx\next\relax
\let\go\relax\else\let\go\getDs\fi\go}

%%% Something like this could be used to automate the width of colheads in the tablehead,
%% making those that match the D in preamble are multicolumn2c instead of
%% multicolumn 1c.
\newcount\colheadcount
\newcommand\xcolhead[1]{\multicolumn{\expandafter\ifx\csname
col\the\colheadcount\endcsname\relax 1\else\expandafter%
\csname col\the\colheadcount\endcsname\fi}{c}{\vrule depth 4pt width
0pt#1\relax\global\advance\colheadcount
by 1\relax}\ignorespaces}%
%

\extrarowheight=2pt
%% from revtex4-1, additions on top
\def\@array@array@new[#1]#2{%
\ifbreaktab
\else
%%% added so that we can use \colnumbers in any tabular environment
%%% and avoid error messages for breaktabular
\def\catchpreamble{#2\relax}
\gdef\tabfullpreamble{#2\relax}
{\colcount=0
\expandafter\countcols\catchpreamble
\global\totalcolumns=\colcount}
\fi
%%% end of additions
  \@tempdima\ht\strutbox
  \advance\@tempdima by\extrarowheight
  \setbox\@arstrutbox\hbox{%
   \vrule \@height\arraystretch\@tempdima
          \@depth \arraystretch\dp\strutbox
          \@width \z@
  }%
  \begingroup
   \@mkpream{#2}%
   \xdef\@preamble{\@preamble}%
  \endgroup
  \prepdef\@preamble{%
   \tabskip\tabmid@skip
    \@arstrut
  }%
  \appdef\@preamble{%
   \tabskip\tabright@skip
   \cr
   \array@row@pre
  }%
  \@arrayleft
  \@nameuse{@array@align@#1}%
  \m@th
  \let\\\new@arraycr %<<== defined above, original plus reset counters
  \let\tabularnewline\\%
  \let\par\@empty
  \let\@sharp##%
  \set@typeset@protect
  \lineskip\z@\baselineskip\z@
  \tabskip\tableft@skip
  \everycr{}%
  \expandafter\halign\expandafter\@halignto\expandafter\bgroup\@preamble
}%

%% \rotate 
\newif\ifrotateon
\def\rotate{\iffloat\global\let\go\relax\global\rotateontrue\else
\global\rotateonfalse 
\def\go{\vskip48pt\huge
{\tt \string\floattable\space must precede table using
\string\rotate !!}}\fi\go}

\newbox\rotatingbox
\def\rotatetable{\setbox\rotatingbox=\vbox\bgroup}
\def\endrotatetable{\egroup
\rotatebox{90}{\vtop{\unvbox\rotatingbox}}
}


\def\colnumbers{\plaincolnumbers}
%% set &, $ catcode back to normal:

\catcode`&=4
\catcode`$=3


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% end table tools
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Gridlines for positioning multiple illustrations in 
%% one figure environment.

\def\gridline#1{\vskip6pt\hbox to\hsize{#1}\vskip6pt}

\def\boxedfig#1#2#3{\hfill\fbox{\vbox{\parskip=0pt\relax
\hsize=#2
\includegraphics[width=#2]{#1}\vskip2pt\vtop{\hsize=#2
\centerline{#3}}}}\hfill}

\def\fig#1#2#3{\hfill\vbox{\parskip=0pt\hsize=#2
\includegraphics[width=#2]{#1}\vskip2pt\vtop{\centering
\footnotesize
\hsize=#2
#3\vskip1pt
}}\hfill}

\def\leftfig#1#2#3{\vbox{\parskip=0pt\relax\hsize=#2
\includegraphics[width=#2]{#1}\vskip2pt\vtop{\hsize=#2
\centering
#3\vskip1sp\vskip1sp}}\hfill}

\def\rightfig#1#2#3{\hfill\vbox{\parskip=0pt\relax\hsize=#2
\includegraphics[width=#2]{#1}\vskip2pt\vtop{\hsize=#2
\centering#3\vskip1sp}}}

\def\rotatefig#1#2#3#4{\hfill\vbox{\centering\parskip=0pt\hsize=#3
\includegraphics[width=#3,angle=#1]{#2}\vskip2pt\vtop{\centering
\footnotesize
\hsize=#3
#4\vskip1pt
}}\hfill}

%%%%%%%%%% End Grid line Macros %%%%%%%%%%

%%%%%%%%%% Color Editing Macros %%%%%%%%%%
\newif\ifturnoffedit
\def\turnoffedit{\global\turnoffedittrue}
\def\turnonedit{\global\turnoffeditfalse}

%% Other choices can be made, but this should be
%% standardized, so didn't make an user interface
%% to change the colors easily.

\expandafter\def\csname editcolor1\endcsname{magenta}
\expandafter\def\csname editcolor2\endcsname{blue}
\expandafter\def\csname editcolor3\endcsname{violet}

\newcount\colorcount

\def\edit#1#2{\colorcount=#1
\ifturnoffedit #2\else
\textcolor{\expandafter\csname
editcolor\the\colorcount\endcsname}{\ifnum\colorcount=1{\bf #2}\else
\ifnum\colorcount=2{\it #2}\else
\ifnum\colorcount=3{#2}\fi\fi\fi}\fi}

\def\collaborationcomment#1#2{\ifturnoffedit\else \colorcount=#1{\textcolor{\expandafter\csname
editcolor\the\colorcount\endcsname} 
{\ifnum\colorcount=1{\bf Collaboration~note: #2}\else
\ifnum\colorcount=2{\it Collaboration~note: #2}\else
\ifnum\colorcount=3{Collaboration~note: #2}\fi\fi\fi}}\fi}

\let\authorcomment\collaborationcomment


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%% Draft watermark

\definecolor{ltgray}{gray}{.9}% .89
\font\bighelv=phvr at 2in %
\def\setwatermarkfontsize#1{\font\bighelv=phvr at #1}

\def\watermark#1{\def\thewatermark{\hbox to\textwidth{\vtop to
1.1\textheight{\vss
\hskip24pt\rotatebox{60}{\hbox{\bighelv \color{ltgray} 
\uppercase{#1}}}\vss}}\hss}}
\let\thewatermark\empty

%% Used in titlepage definition, as \pagestyle{titlepage}
\def\ps@titlepage{%
  \let\@mkboth\@gobbletwo
\def\@oddhead{\ifx\thewatermark\empty\hfill\else
\hbox to \textwidth{\rlap{\thewatermark}\hfill}\fi}
\let\@oddfoot\@empty
\let\@evenhead\@empty\let\@evenfoot\@empty}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% ORCID identifier

\def\ORCID#1{%
  \aftermaketitle@chk{\ORCIDs}%
  \gdef\@Orcid{\vskip2pt\noindent\hskip.5in{\it ORCID: \rm #1}\vskip1pt}%
}%


\def\titleblock@produce{%
 \begingroup
  \ltx@footnote@pop
  \def\@mpfn{mpfootnote}%
  \def\thempfn{\thempfootnote}%
  \c@mpfootnote\z@
  \let\@makefnmark\frontmatter@makefnmark
  \frontmatter@setup
  \thispagestyle{titlepage}\label{FirstPage}%
  \frontmatter@title@produce
  \groupauthors@sw{%
   \frontmatter@author@produce@group
  }{%
   \frontmatter@author@produce@script
  }%
  \frontmatter@RRAPformat{%
   \expandafter\produce@RRAP\expandafter{\@date}%
   \expandafter\produce@RRAP\expandafter{\@received}%
   \expandafter\produce@RRAP\expandafter{\@revised}%
   \expandafter\produce@RRAP\expandafter{\@accepted}%
   \expandafter\produce@RRAP\expandafter{\@published}%
  }%
  \frontmatter@abstract@produce
  \@ifx@empty\@pacs{}{%
   \@pacs@produce\@pacs
  }%
  \@ifx@empty\@keywords{}{%
   \@keywords@produce\@keywords
%%% Addition to produce ORCID identifier(s)
\expandafter\ifx\csname @Orcid\endcsname\relax\else
\@Orcid\global\let\@Orcid\relax\fi
  }%
  \par
  \frontmatter@finalspace
 \endgroup
}%


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Highlight Figure
\definecolor{ltblue}{cmyk}{.5,.1,.1,0}

\long\def\highlightfigure#1{\centerline{
\fboxrule=4pt
\fboxsep=12pt
\fcolorbox{ltblue}{white}{\hbox to
.93\textwidth{\hss$\vcenter{\advance\hsize -24pt #1}$\hss}}}\gdef\highlightfigure{\typeout{^^J^^JERROR!^^J^^J
Only One Highlighted Figure per Article!^^J^^J}\stop}}

%%%%%%%%%%%%%%%%%%%%%%%%%
%% Track Changes
%% Amy Hendrickson, Nov 2015
%% Change Jan 2016, to allow list of changes
%% to give line and page numbers for more than one entry on the same line.
%% Change Feb 2016, to allow optional argument for time/date, and/or editor initials, etc.

\ifnumlines
\linenumbers*[1]
\if@two@col
\advance\linenumbersep -4pt
\else
\advance\linenumbersep 12pt
\fi\fi

\providecolor{trackchange}{rgb}{1,0,0}
\providecolor{explain}{rgb}{.5,0,.5}

\newcount\refchangenumber
\def\added{\@ifnextchar[{\xadded}{\yadded}}

\long\def\xadded[#1]#2{%
\iftrack {\global\advance\refchangenumber by 1\relax%
\vtop to 0pt{\vss
\hypertarget{link\the\refchangenumber}{}
\vskip14pt}
\ifnumlines
\xdef\doit{\noexpand\linelabel{\the\refchangenumber}}\doit\else%
\xdef\doit{\noexpand\label{\the\refchangenumber}{}{}{}}\doit\fi%
\color{trackchange}(Added: #2)}%%
\expandafter\gdef\csname
changenum\the\refchangenumber\endcsname{Added: [#1]
\textcolor{trackchange}{#2}, }\else#2\fi}

\long\def\yadded#1{%
\iftrack{\global\advance\refchangenumber by 1\relax%
\vtop to 0pt{\vss
\hypertarget{link\the\refchangenumber}{}
\vskip14pt}
\ifnumlines
\xdef\doit{\noexpand\linelabel{\the\refchangenumber}}\doit\else%
\xdef\doit{\noexpand\label{\the\refchangenumber}{}{}{}}\doit\fi%
\color{trackchange}(Added: #1)}%%
\expandafter\gdef\csname changenum\the\refchangenumber\endcsname{Added:
\textcolor{trackchange}{#1}, }\else#1\fi}

\def\deleted{\@ifnextchar[{\xdeleted}{\ydeleted}}

\long\def\xdeleted[#1]#2{
\iftrack
\global\advance\refchangenumber by 1
\vtop to 0pt{\vss
\hypertarget{link\the\refchangenumber}{}
\vskip14pt}
\ifnumlines\xdef\doit{\noexpand\linelabel{\the\refchangenumber}}\doit\else%
\xdef\doit{\noexpand\label{\the\refchangenumber}}\doit\fi%
{\color{trackchange}(Deleted: \sout{#2})}%
\expandafter\def\csname
changenum\the\refchangenumber\endcsname{Deleted: [#1]
{\color{trackchange}\sout{#2}}, }\fi}

\long\def\ydeleted#1{
\iftrack
\global\advance\refchangenumber by 1
\vtop to 0pt{\vss
\hypertarget{link\the\refchangenumber}{}
\vskip14pt}
\ifnumlines\xdef\doit{\noexpand\linelabel{\the\refchangenumber}}\doit\else%
\xdef\doit{\noexpand\label{\the\refchangenumber}}\doit\fi%
{\color{trackchange}(Deleted: \sout{#1})}%
\expandafter\def\csname changenum\the\refchangenumber\endcsname{Deleted:
{\color{trackchange}\sout{#1}}, }\fi}

\def\replaced{\@ifnextchar[{\xreplaced}{\yreplaced}}

\long\def\xreplaced[#1]#2#3{%
\iftrack
\global\advance\refchangenumber by 1
\vtop to 0pt{\vss
\hypertarget{link\the\refchangenumber}{}
\vskip14pt}
\ifnumlines\xdef\doit{\noexpand\linelabel{\the\refchangenumber}}\doit\else%
\xdef\doit{\noexpand\label{\the\refchangenumber}}\doit\fi%
{\color{trackchange}(Replaced: \sout{#2}}
{\color{black}replaced with:} {\color{trackchange} #3)}%
\expandafter\gdef\csname
changenum\the\refchangenumber\endcsname{Replaced: [#1]
{\color{trackchange}\sout{#2}} {\color{black} replaced with:}
{\color{trackchange}#3}, }\else#3\fi}

\long\def\yreplaced#1#2{%
\iftrack
\global\advance\refchangenumber by 1
\vtop to 0pt{\vss
\hypertarget{link\the\refchangenumber}{}
\vskip14pt}
\ifnumlines\xdef\doit{\noexpand\linelabel{\the\refchangenumber}}\doit\else%
\xdef\doit{\noexpand\label{\the\refchangenumber}}\doit\fi%
{\color{trackchange}(Replaced: \sout{#1}}
{\color{black}replaced with:} {\color{trackchange} #2)}%
\expandafter\gdef\csname changenum\the\refchangenumber\endcsname{Replaced:
{\color{trackchange}\sout{#1}} {\color{black} replaced with:}
{\color{trackchange}#2}, }\else#2\fi}

\def\explain{\@ifnextchar[{\xexplain}{\yexplain}}

\def\xexplain[#1]#2{\iftrack\ {\it\color{explain} [Explanation of change:
#2 (#1)]\ }\fi}

\def\yexplain#1{\iftrack\ {\it\color{explain} [Explanation of change:
#1]\ }\fi}

\newcount\listchangenum

\def\listofchanges{
\clearpage
\iftrack
\ifnum\refchangenumber>0
\ifnumlines\nolinenumbers\else
\hypersetup{linkcolor=black}
\fi
\vskip36pt
%\vtop{
\hrule
\noindent{\vrule height 14pt width0pt depth 6pt\large\bf List of Changes}
\hrule
\vskip18pt%}
\nobreak
{\parskip=4pt \parindent=0pt
\loop
\vskip-1pt\relax
\global\advance\listchangenum by 1\relax
\expandafter\ifx\csname changenum\the\listchangenum\endcsname\relax
\else
\csname changenum\the\listchangenum\endcsname\ 
on page
%% can't get hyperlink to work correctly for page numbers, yet:
%\hyperlink{link\the\listchangenum}{
\pageref{\the\listchangenum}%}%
\ifnumlines
\setbox0=\hbox{\lineref{\the\listchangenum}}%
\ifdim\wd0>20pt%
\else
, line\
\hyperlink{link\the\listchangenum}{\lineref{\the\listchangenum}}%
\fi\fi.
\repeat}
\fi\fi
\thispagestyle{empty}
}

\PassOptionsToPackage{normalem}{ulem}
\usepackage{ulem}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% Limit number of printed author/affiliations


\def\collaborationName#1{\def\thecollaborationname{#1}}
\def\fullcollaborationName#1{\def\thefullcollaborationname{#1}}

\collaborationName{}

\collaborationName{{\tt Please Enter \string\collaborationName\string{< name
for Collaboration >\string}}}

\def\collaborationlist#1{\vskip24pt
\noindent The \expandafter\ifx\csname
fullcollaborationname\endcsname\relax
\thecollaborationname\else
\thefullcollaborationname\fi\ is #1}

\newcount\AuthorCallLimit
%% no limit for default
\AuthorCallLimit=100
\let\saveauthor\author
\def\xauthor#1{\saveauthor{#1}}

\long\def\eatentry#1{}
\long\def\almosteatentry#1{\expandafter\gdef\csname 
xaffil\the\authorcount\endcsname{#1}}

\def\eatarg#1{}
\def\eattwo#1#2{}
\newcount\authorcount

\def\eatnine#1#2#3#4#5#6#7#8#9{}

\def\author#1{
\ifnum\AuthorCallLimit<1
\xauthor{#1}
  \else
  \ifnum\authorcount<\AuthorCallLimit
  \xauthor{#1}
    \else
     \ifnum\authorcount=\AuthorCallLimit
\def\frontmatter@collaboration@above{\vskip12pt}
\collaboration{and 
\expandafter\ifx\csname thefullcollaborationname\endcsname\relax
the \thecollaborationname\ collaboration
\else
\thefullcollaborationname\fi
}
\global\let\affil\almosteatentry
\global\let\email\eatentry
\global\let\thecollaboration\relax
\let\and\relax
\def\@listand{\@ifnum{\@tempcnta=\tw@}{\space}{}}%
\def\@listcomma@comma{\@ifnum{\@tempcnta>\@ne}{}{}}%
\def\@listcomma@comma@UK{\@ifnum{\@tempcnta>\tw@}{}{}}%
    \fi%% authorcount=authorcallimit
\fi% authorcount less than authorcalllimit
\fi% end test to see if authorcalllimit was used
\global\advance\authorcount by 1\relax
\expandafter\gdef\csname
author\the\authorcount\endcsname{{\sc #1}}}

\newcount\endauthorcount
\def\allauthors{%% this conditional keeps \allauthors from turning on
%%                 unless \AuthorCallLimit is used:
\ifnum\AuthorCallLimit>0
\onecolumngrid
{\vskip6pt\vskip1sp\section*{All Authors and
Affiliations}\nobreak
\let\parsecomma\eatnine%
\endauthorcount=1%
{\parskip=0pt
\centering
\loop\expandafter\ifx\csname author\the\endauthorcount\endcsname\relax\else
\ifnum\endauthorcount<\authorcount
\csname author\the\endauthorcount\endcsname,\\
\expandafter\ifx\csname xaffil\the\endauthorcount\endcsname\relax
\else
\csname xaffil\the\endauthorcount\endcsname \\[2pt]
\fi
\else\vskip1sp {\sc And}\\
\csname author\the\endauthorcount\endcsname.\\
\csname xaffil\the\endauthorcount\endcsname
\fi
\global\advance\endauthorcount by 1
\vskip3pt
\repeat
\vskip1pt}
\centering
\vskip12pt
\endauthorcount=1
\loop\ifnum\endauthorcount<\affilcount
\csname endaffil\the\endauthorcount\endcsname,
\global\advance\endauthorcount by 1
\repeat
\csname endaffil\the\endauthorcount\endcsname
\vskip12pt}\fi}

\newcount\affilnum
\newcount\curraffilnum
\newcount\tempaffilnum

\def\parsecomma#1,#2,#3,#4,#5,#6,#7,#8,#9,{\sendnumber{#1}
\def\two{#2}\ifx\two\empty\else\sendnumber{#2}\fi
\def\three{#3}\ifx\three\empty\else\sendnumber{#3}\fi
\def\four{#4}\ifx\four\empty\else\sendnumber{#4}\fi
\def\five{#5}\ifx\five\empty\else\sendnumber{#5}\fi
\def\six{#6}\ifx\six\empty\else\sendnumber{#6}\fi
\def\seven{#7}\ifx\seven\empty\else\sendnumber{#7}\fi
\def\eight{#8}\ifx\eight\empty\else\sendnumber{#8}\fi
\def\nine{#9}\ifx\nine\empty\else\sendnumber{#9}\fi
}

\def\sendnumber#1{%
\immediate\write\@auxout{\string\ifnum\string\curraffilnum < #1
\string\global\string\curraffilnum=#1\relax\string\fi}}

\def\altaffilmark#1{%
\setbox0=\hbox{\parsecomma#1,{},{},{},{},{},{},{},{},}%
\affil@mark{#1}}

\newcount\affilcount
\def\altaffiltext#1#2{%% for allauthors at end of paper
\global\advance\affilcount by 1
\expandafter\gdef\csname endaffil\the\affilcount\endcsname{
\vskip.6pt
{\noindent\llap{$^{#1}$}\footnotesize\relax#2}}
%%%% end for all authors
\global\advance\affilnum by 1
\ifnum\affilnum <\curraffilnum
\expandafter\gdef\csname affil\the\affilnum\endcsname{\vskip.6pt
{\noindent\llap{$^{#1}$}\footnotesize\relax#2}}
\else
\expandafter\gdef\csname affil\the\affilnum\endcsname{\relax}
\fi
\ifnum\affilnum =\curraffilnum
\expandafter\gdef\csname affil\the\affilnum\endcsname{\vskip.6pt
{\noindent\llap{$^{#1}$}\footnotesize\relax#2}}\fi
}

\def\makeaffils{\loopnum=1\relax\loop
\expandafter\ifx\csname affil\the\loopnum\endcsname\relax
\else
\expandafter\csname affil\the\loopnum\endcsname\relax
% resetting:
\expandafter\gdef\csname affil\the\loopnum\endcsname{\relax}
\global\advance\loopnum by 1
\repeat
\expandafter\csname affil\the\loopnum\endcsname\relax
\vskip12pt
}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% From earlier version of AASTeX, for usefulness and 
%% backward compatibility, with some requested additions

\let\jnl@style=\rmfamily
\def\ref@jnl#1{{\jnl@style#1}}%

%%%
\renewcommand\memsai{\ref@jnl{Mem.~Soc.~Astron.~Italiana}}%
          % Mem. Societa Astronomica Italiana
%% longer version:
\newcommand\memsocai{Mem. Societ\`a Astronomica Italiana}
\newcommand\aspconf{Ast. Soc. of the Pac. Conference Series}

\let\astap=\aap
\let\apjlett=\apjl
\let\apjsupp=\apjs
\let\applopt=\ao

%%% More useful commands from Earlier version of Aastex:
\let\la=\lesssim            % For Springer A&A compliance...
\let\ga=\gtrsim

\let\farcm\farcm@mss
\let\farcs\farcs@mss

\def\farcm@apj{%
 \mbox{.\kern -0.7ex\raisebox{.9ex}{\scriptsize$\prime$}}%
}%

\def\farcs@apj{%
 \mbox{%
  \kern  0.13ex.%
  \kern -0.95ex\raisebox{.9ex}{\scriptsize$\prime\prime$}%
  \kern -0.1ex%
 }%
}%

\def\load@astro{%
 \dimen@=1\aas@ptsize\p@
 \font\astro@font=Astrosym at\dimen@
}%
\def\astro#1{\leavevmode\hbox{\astro@font#1}}%
\def\astro@font{%
 \ClassWarning{aastex}{%
  Please use class option `astro', since you are using the astro font.%
 }%
}%

\def\chem@bnd#1{%
 {%
  \kern.1em\relax
  \setbox\z@\hbox{M}%
  \dimen@ii.8em\relax
  \p@=.1em\relax
  \dimen@.5\ht\z@\dimen@i-\dimen@
  \advance\dimen@1.5\p@\advance\dimen@i-1.0\p@
  #1%
  \kern.1em\relax
  }%
 }%
\def\@sbnd{%
 \advance\dimen@-1.5\p@\advance\dimen@i1.5\p@
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 }%
\def\@dbnd{%
 \advance\dimen@-0.5\p@\advance\dimen@i0.5\p@
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 \advance\dimen@-1.5\p@\advance\dimen@i1.5\p@
 \hskip-\dimen@ii
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 }%
\def\@tbnd{%
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 \advance\dimen@-1.5\p@\advance\dimen@i1.5\p@
 \hskip-\dimen@ii
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 \advance\dimen@-1.5\p@\advance\dimen@i1.5\p@
 \hskip-\dimen@ii
 \vrule\@height\dimen@\@depth\dimen@i\@width\dimen@ii\nobreak
 }%

\renewcommand\LaTeX{%
 \leavevmode
 L%
 \raise.42ex\hbox{%
  \count@=\the\fam
  $\fam\count@\scriptstyle\kern-.3em A$%
 }%
 \kern-.15em\TeX
}%

%% added Feb 2, 2016, redefining definition of \caption made with hyperref
%% package. Making \caption[] be the default, to prevent sending caption 
%% text to listoftables or listoffigures, which we are not going to use
%% anyway. This change enables track changes commands to work in captions.

\def\caption{\numlinesfalse
\ifx\@captype\@undefined 
\@latex@error {\noexpand \caption outside float}\@ehd 
\expandafter \@gobble \else 
\H@refstepcounter \@captype \let \Hy@tempa \@caption \@ifundefined
{float@caption}{}
{\expandafter \ifx \csname @float@c@\@captype\endcsname
\float@caption\let\Hy@tempa\Hy@float@caption\fi} 
\expandafter\@firstofone\fi 
{\@dblarg {\Hy@tempa \@captype}}[]}


\def\@caption#1[#2]#3{\small\expandafter \ifx \csname if@capstart\expandafter \endcsname 
\csname iftrue\endcsname \global \let \@currentHref \hc@currentHref \else \hyper@makecurrent {\@captype }\fi 
\@ifundefined {NR@gettitle}{\def \@currentlabelname{\thesection\relax #2}}
{\NR@gettitle {#2}}\par 
%\addcontentsline {\csname ext@#1\endcsname }{#1}{\protect \numberline {\csname the#1\endcsname }{\ignorespaces #2}}
\begingroup \@parboxrestore \if@minipage \@setminipage \fi \normalsize 
\expandafter\ifx \csname if@capstart\expandafter \endcsname 
\csname iftrue\endcsname \global \@capstartfalse 
\@makecaption {\csname fnum@#1\endcsname }{\ignorespaces #3}
\else \@makecaption {\csname fnum@#1\endcsname }{\ignorespaces \ifHy@nesting 
\expandafter \hyper@@anchor \expandafter {\@currentHref }{#3}\else 
\Hy@raisedlink {\expandafter \hyper@@anchor \expandafter
{\@currentHref }{\relax }}#3\fi }\fi \par \endgroup}


\newcommand\tablebreak{\\[-11pt]\noalign{\break}\\ }

%% As suggested by Greg Schwarz, Gus Meunch, Feb 11

\def\dataset{\@ifnextchar[{\ydataset}{\xdataset}}
\def\ydataset[#1]#2{\href{#2}{[#1]}}
\def\xdataset#1{\href{#1}{[DATASET]}}

\advance\columnsep6pt

\endinput

%% Change Log

Feb 29, changes to table macros to center caption even if table is wider than
width of text; give error message if user asks to rotate table but
forgets to use \floattable before table; make rotating table be full
text width even if called for in double columns.

March 1, reworking figure and table numbering for appendices. 
Deleted recent definition of \appendix, returned to earlier version
with additions.

%% note:
%% this keeps tables from being set to \small, code used in Revtex4-1
\let\table@hook\relax




