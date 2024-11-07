% yahapj.bst -- yet another ApJ bibtex style file
% Git SHA1 hash: $Id$
% Reference repository: https://github.com/pkgw/yahapj/
%
% The main features compared to other ApJ style files is that ArXiV
% and DOI links are added. ArXiV information is only shown for those
% entries that don't have more formal reference information. There are
% other improvements too.
%
% Because this style file embeds links, your LaTeX file must use the
% hyperref package. For non-butt-ugly links, try:
%
% \usepackage[breaklinks,colorlinks,urlcolor=blue,citecolor=blue,linkcolor=blue]{hyperref}
%
% Copyright notice: This work is dedicated to the public domain. You
% have unlimited permission to copy, distribute, and modify it.
%
% For more information about this file and its provenance, consult
% README.md in the Git repository from which it was obtained. You can
% track these down using the SHA1 hash listed at the top of this file.
% (Googling for the hash value should hopefully be sufficient.)

ENTRY
  { address
    archive
    archivePrefix
    author
    booktitle
    chapter
    doi
    edition
    editor
    eprint
    howpublished
    institution
    isbn
    journal
    key
    month
    note
    number
    organization
    pages
    primaryClass
    publisher
    school
    series
    title
    type
    url
    volume
    year
  }
  {}
  { label extra.label sort.label short.list }

INTEGERS { output.state before.all mid.sentence after.sentence after.block work.strlen }

FUNCTION {init.state.consts}
{ #0 'before.all :=
  #1 'mid.sentence :=
  #2 'after.sentence :=
  #3 'after.block :=
}

STRINGS { s t }

FUNCTION {output.nonnull}
{ 's :=
  output.state mid.sentence =
    { ", " * write$ }
    { output.state after.block =
        { add.period$ write$
          newline$
          "\newblock " write$
        }
        { output.state before.all =
            'write$
            { add.period$ " " * write$ }
          if$
        }
      if$
      mid.sentence 'output.state :=
    }
  if$
  s
}

FUNCTION {output}
{ duplicate$ empty$
    'pop$
    'output.nonnull
  if$
}

FUNCTION {output.check}
{ 't :=
  duplicate$ empty$
    { pop$ "empty " t * " in " * cite$ * warning$ }
    'output.nonnull
  if$
}

FUNCTION {fin.entry}
{ duplicate$ empty$
    'pop$
    'write$
  if$
  newline$
}

FUNCTION {new.block}
{ output.state before.all =
    'skip$
    { after.block 'output.state := }
  if$
}

FUNCTION {new.sentence}
{ output.state after.block =
    'skip$
    { output.state before.all =
        'skip$
        { after.sentence 'output.state := }
      if$
    }
  if$
}

FUNCTION {add.blank}
{  " " * before.all 'output.state :=
}

FUNCTION {not}
{   { #0 }
    { #1 }
  if$
}

FUNCTION {and}
{   'skip$
    { pop$ #0 }
  if$
}

FUNCTION {or}
{   { pop$ #1 }
    'skip$
  if$
}

FUNCTION {new.block.checkb}
{ empty$
  swap$ empty$
  and
    'skip$
    'new.block
  if$
}

FUNCTION {field.or.null}
{ duplicate$ empty$
    { pop$ "" }
    'skip$
  if$
}

FUNCTION {capitalize}
{ "u" change.case$ "t" change.case$ }

FUNCTION {space.word}
{ " " swap$ * " " * }

% Here's how BibTeX does internationalization:

FUNCTION {bbl.and}
{ "and"}

FUNCTION {bbl.editors}
{ "eds." }

FUNCTION {bbl.editor}
{ "ed." }

FUNCTION {bbl.edby}
{ "edited by" }

FUNCTION {bbl.edition}
{ "edn." }

FUNCTION {bbl.volume}
{ "Vol." }

FUNCTION {bbl.of}
{ "of" }

FUNCTION {bbl.number}
{ "no." }

FUNCTION {bbl.nr}
{ "no." }

FUNCTION {bbl.in}
{ "in" }

FUNCTION {bbl.pages}
{ "" }

FUNCTION {bbl.page}
{ "" }

FUNCTION {bbl.chapter}
{ "Ch." }

FUNCTION {bbl.techrep}
{ "Tech. Rep." }

FUNCTION {bbl.mthesis}
{ "Master's thesis" }

FUNCTION {bbl.phdthesis}
{ "PhD thesis" }

FUNCTION {bbl.first}
{ "1st" }

FUNCTION {bbl.second}
{ "2nd" }

FUNCTION {bbl.third}
{ "3rd" }

FUNCTION {bbl.fourth}
{ "4th" }

FUNCTION {bbl.fifth}
{ "5th" }

FUNCTION {bbl.st}
{ "st" }

FUNCTION {bbl.nd}
{ "nd" }

FUNCTION {bbl.rd}
{ "rd" }

FUNCTION {bbl.th}
{ "th" }

MACRO {jan} {"Jan."}

MACRO {feb} {"Feb."}

MACRO {mar} {"Mar."}

MACRO {apr} {"Apr."}

MACRO {may} {"May"}

MACRO {jun} {"Jun."}

MACRO {jul} {"Jul."}

MACRO {aug} {"Aug."}

MACRO {sep} {"Sep."}

MACRO {oct} {"Oct."}

MACRO {nov} {"Nov."}

MACRO {dec} {"Dec."}

FUNCTION {eng.ord}
{ duplicate$ "1" swap$ *
  #-2 #1 substring$ "1" =
     { bbl.th * }
     { duplicate$ #-1 #1 substring$
       duplicate$ "1" =
         { pop$ bbl.st * }
         { duplicate$ "2" =
             { pop$ bbl.nd * }
             { "3" =
                 { bbl.rd * }
                 { bbl.th * }
               if$
             }
           if$
          }
       if$
     }
   if$
}

INTEGERS { nameptr namesleft numnames numtrunc }

FUNCTION {format.names}
{ 's :=
  #0 'numtrunc :=
  #1 'nameptr :=
  s num.names$ 'numnames :=
  numnames 'namesleft :=
  numnames #5 >
    { #3 'numtrunc := }
    {}
  if$
  { namesleft #0 > }
  { s nameptr
    "{vv~}{ll}{, jj}{, f.}" format.name$
    't :=
    nameptr #1 >
      {
        nameptr #1 - numtrunc =
          {
            #1 'namesleft :=
            "others" 't :=
          }
          {}
        if$

        namesleft #1 >
          { ", " * t * }
          {
            numnames #1 >
              { "," * }
              'skip$
            if$
            s nameptr "{ll}" format.name$ duplicate$ "others" =
              { 't := }
              { pop$ }
            if$
            t "others" =
              {
                " {et~al.}" *
              }
              { " \& " * t * }
            if$
          }
        if$
      }
      't
    if$
    nameptr #1 + 'nameptr :=
    namesleft #1 - 'namesleft :=
  }
  while$
}

FUNCTION {format.names.ed}
{ 's :=
  #1 'nameptr :=
  s num.names$ 'numnames :=
  numnames 'namesleft :=
    { namesleft #0 > }
    { s nameptr
      "{f.~}{vv~}{ll}{, jj}"
      format.name$
      't :=
      nameptr #1 >
        {
          namesleft #1 >
            { ", " * t * }
            {
              numnames #2 >
                { "," * }
                'skip$
              if$
              s nameptr "{ll}" format.name$ duplicate$ "others" =
                { 't := }
                { pop$ }
              if$
              t "others" =
                {
                  " {et~al.}" *
                }
                { " \& " * t * }
              if$
            }
          if$
        }
        't
      if$
      nameptr #1 + 'nameptr :=
      namesleft #1 - 'namesleft :=
    }
  while$
}

FUNCTION {format.key}
{ empty$
    { key field.or.null }
    { "" }
  if$
}

FUNCTION {format.authors}
{ author empty$
    { "" }
    { author format.names }
  if$
}

FUNCTION {format.editors}
{ editor empty$
    { "" }
    { editor format.names
      editor num.names$ #1 >
        { ", " * bbl.editors * }
        { ", " * bbl.editor * }
      if$
    }
  if$
}

FUNCTION {format.in.editors}
{ editor empty$
    { "" }
    { editor format.names.ed
    }
  if$
}

FUNCTION {format.note}
{ note empty$
    { "" }
    { note #1 #1 substring$
      duplicate$ "{" =
        'skip$
        { output.state mid.sentence =
          { "l" }
          { "u" }
        if$
        change.case$
        }
      if$
      note #2 global.max$ substring$ *
    }
  if$
}

FUNCTION {format.title}
{ title empty$
    { "" }
    { title
    }
  if$
}

FUNCTION {format.full.names}
{'s :=
  #1 'nameptr :=
  s num.names$ 'numnames :=
  numnames 'namesleft :=
    { namesleft #0 > }
    { s nameptr
      "{vv~}{ll}" format.name$
      't :=
      nameptr #1 >
        {
          namesleft #1 >
            { ", " * t * }
            {
              numnames #2 >
                { "," * }
                'skip$
              if$
              s nameptr "{ll}" format.name$ duplicate$ "others" =
                { 't := }
                { pop$ }
              if$
              t "others" =
                {
                  " {et~al.}" *
                }
                { " \& " * t * }
              if$
            }
          if$
        }
        't
      if$
      nameptr #1 + 'nameptr :=
      namesleft #1 - 'namesleft :=
    }
  while$
}

FUNCTION {author.editor.key.full}
{ author empty$
    { editor empty$
        { key empty$
            { cite$ #1 #3 substring$ }
            'key
          if$
        }
        { editor format.full.names }
      if$
    }
    { author format.full.names }
  if$
}

FUNCTION {author.key.full}
{ author empty$
    { key empty$
         { cite$ #1 #3 substring$ }
          'key
      if$
    }
    { author format.full.names }
  if$
}

FUNCTION {editor.key.full}
{ editor empty$
    { key empty$
         { cite$ #1 #3 substring$ }
          'key
      if$
    }
    { editor format.full.names }
  if$
}

FUNCTION {make.full.names}
{ type$ "book" =
  type$ "inbook" =
  or
    'author.editor.key.full
    { type$ "proceedings" =
        'editor.key.full
        'author.key.full
      if$
    }
  if$
}

FUNCTION {output.bibitem}
{ newline$
  "\bibitem[{" write$
  label write$
  ")" make.full.names duplicate$ short.list =
     { pop$ }
     { * }
   if$
  "}]{" * write$
  cite$ write$
  "}" write$
  newline$
  ""
  before.all 'output.state :=
}

FUNCTION {format.archive}
{ archivePrefix empty$
    { "" }
    { archivePrefix ":" *}
  if$
}

FUNCTION {format.primaryClass}
{ primaryClass empty$
    { "" }
    { " [" primaryClass * "]" *}
  if$
}

FUNCTION {format.eprint}
% Only print eprint info if no page info and no booktitle info --
% what we're really going for is print the eprint info only if there's
% no "official" reference info to work with.
{ eprint empty$
  pages empty$ not
  booktitle empty$ not or or
    { "" }
     { archive empty$
          {"\href{http://arxiv.org/abs/" eprint * "}" *
             "{{\sffamily " * format.archive * eprint *
              format.primaryClass * "}}" *}
          {"\href{" archive *  "/" * eprint * "}" *
             "{{\sffamily " * format.archive * eprint *
              format.primaryClass * "}}" *}
       if$
     }
  if$
}


FUNCTION {n.dashify}
{
  't :=
  ""
    { t empty$ not }
    { t #1 #1 substring$ "-" =
        { t #1 #2 substring$ "--" = not
            { "--" *
              t #2 global.max$ substring$ 't :=
            }
            {   { t #1 #1 substring$ "-" = }
                { "-" *
                  t #2 global.max$ substring$ 't :=
                }
              while$
            }
          if$
        }
        { t #1 #1 substring$ *
          t #2 global.max$ substring$ 't :=
        }
      if$
    }
  while$
}

FUNCTION {word.in}
{ bbl.in
  " " * }

FUNCTION {format.date}
{ year duplicate$ empty$
    { "empty year in " cite$ * "; set to ????" * warning$
       pop$ "????" }
    'skip$
  if$
  extra.label *
  before.all 'output.state :=
  after.sentence 'output.state :=
}

FUNCTION {format.btitle}
{ title
}

FUNCTION {tie.or.space.connect}
{ duplicate$ text.length$ #3 <
    { "~" }
    { " " }
  if$
  swap$ * *
}

FUNCTION {either.or.check}
{ empty$
    'pop$
    { "can't use both " swap$ * " fields in " * cite$ * warning$ }
  if$
}

FUNCTION {format.bvolume}
{ volume empty$
    { "" }
    { bbl.volume volume tie.or.space.connect
      series empty$
        'skip$
        { bbl.of space.word * series * }
      if$
      "volume and number" number either.or.check
    }
  if$
}

FUNCTION {format.number.series}
{ volume empty$
    { number empty$
        { series field.or.null }
        { output.state mid.sentence =
            { bbl.number }
            { bbl.number capitalize }
          if$
          number tie.or.space.connect
          series empty$
            { "there's a number but no series in " cite$ * warning$ }
            { bbl.in space.word * series * }
          if$
        }
      if$
    }
    { "" }
  if$
}

FUNCTION {is.num}
{ chr.to.int$
  duplicate$ "0" chr.to.int$ < not
  swap$ "9" chr.to.int$ > not and
}

FUNCTION {extract.num}
{ duplicate$ 't :=
  "" 's :=
  { t empty$ not }
  { t #1 #1 substring$
    t #2 global.max$ substring$ 't :=
    duplicate$ is.num
      { s swap$ * 's := }
      { pop$ "" 't := }
    if$
  }
  while$
  s empty$
    'skip$
    { pop$ s }
  if$
}

FUNCTION {convert.edition}
{ edition extract.num "l" change.case$ 's :=
  s "first" = s "1" = or
    { bbl.first 't := }
    { s "second" = s "2" = or
        { bbl.second 't := }
        { s "third" = s "3" = or
            { bbl.third 't := }
            { s "fourth" = s "4" = or
                { bbl.fourth 't := }
                { s "fifth" = s "5" = or
                    { bbl.fifth 't := }
                    { s #1 #1 substring$ is.num
                        { s eng.ord 't := }
                        { edition 't := }
                      if$
                    }
                  if$
                }
              if$
            }
          if$
        }
      if$
    }
  if$
  t
}

FUNCTION {format.edition}
{ edition empty$
    { "" }
    { output.state mid.sentence =
        { convert.edition "l" change.case$ " " * bbl.edition * }
        { convert.edition "t" change.case$ " " * bbl.edition * }
      if$
    }
  if$
}

INTEGERS { multiresult }

FUNCTION {multi.page.check}
{ 't :=
  #0 'multiresult :=
    { multiresult not
      t empty$ not
      and
    }
    { t #1 #1 substring$
      duplicate$ "-" =
      swap$ duplicate$ "," =
      swap$ "+" =
      or or
        { #1 'multiresult := }
        { t #2 global.max$ substring$ 't := }
      if$
    }
  while$
  multiresult
}

FUNCTION {string.length}
{ % Copied from "Taming the BeaST", by Nicolas Markey, Fig 4
  #1 'work.strlen :=
  { duplicate$ duplicate$ #1 work.strlen substring$ = not }
    { work.strlen #1 + 'work.strlen := }
  while$
  pop$ work.strlen
}

FUNCTION {first.page}
{ 't :=
  ""
    {  t empty$ not t #1 #1 substring$ "-" = not and }
    { t #1 #1 substring$ *
      t #2 global.max$ substring$ 't :=
    }
  while$
  % remove possible trailing plus sign
  duplicate$ duplicate$ string.length #1 substring$ "+" =
    { duplicate$ string.length #1 - #1 swap$ substring$ }
    { }
  if$
}

FUNCTION {format.pages}
{ pages empty$
    { "" }
    { pages first.page }
  if$
}

FUNCTION {format.journal.pages}
{ pages empty$
    'skip$
    { duplicate$ empty$
        { pop$ format.pages }
        {
          ", " *
          pages first.page *
        }
      if$
    }
  if$
}

FUNCTION {format.vol.num.pages}
{ volume field.or.null
}

FUNCTION {format.chapter.pages}
{ chapter empty$
    { "" }
    { type empty$
        { bbl.chapter }
        { type "l" change.case$ }
      if$
      chapter tie.or.space.connect
    }
  if$
}

FUNCTION {format.in.ed.booktitle}
{ booktitle empty$
    { "" }
    { editor empty$
        { word.in booktitle * }
        { word.in booktitle *
          ", " *
          editor num.names$ #1 >
            { bbl.editors }
            { bbl.editor }
          if$
          * " " *
          format.in.editors *
        }
      if$
    }
  if$
}

FUNCTION {format.thesis.type}
{ type empty$
    'skip$
    { pop$
      type "t" change.case$
    }
  if$
}

FUNCTION {format.tr.number}
{ type empty$
    { bbl.techrep }
    'type
  if$
  number empty$
    { "t" change.case$ }
    { number tie.or.space.connect }
  if$
}

FUNCTION {format.article.crossref}
{
  word.in
  " \cite{" * crossref * "}" *
}

FUNCTION {format.book.crossref}
{ volume empty$
    { "empty volume in " cite$ * "'s crossref of " * crossref * warning$
      word.in
    }
    { bbl.volume volume tie.or.space.connect
      bbl.of space.word *
    }
  if$
  " \cite{" * crossref * "}" *
}

FUNCTION {format.incoll.inproc.crossref}
{
  word.in
  " \cite{" * crossref * "}" *
}

FUNCTION {format.publisher}
{ publisher empty$
    { "empty publisher in " cite$ * warning$ }
    'skip$
  if$
  ""
  address empty$ publisher empty$ and
    'skip$
    {
      add.blank "(" *
      address empty$
        'skip$
        { address * }
      if$
      publisher empty$
        'skip$
        { address empty$
            'skip$
            { ": " * }
          if$
          publisher *
        }
      if$
      ")" *
    }
  if$
  output
}

STRINGS {oldname}

FUNCTION {name.or.dash}
{ 's :=
   oldname empty$
     { s 'oldname := s }
     { s oldname =
         { "---" }
         { s 'oldname := s }
       if$
     }
   if$
}

FUNCTION {format.edn.btitle} % Title should be on stack.
{ duplicate$ empty$ edition empty$ or
    'skip$
    { ", " * format.edition * }
  if$
}

FUNCTION {format.ed.booktitle} % The title should be on the stack.
{ duplicate$ empty$
  { "no book title in " cite$ * warning$ "" pop$ }
  { editor empty$
    author empty$ or  % Empty author means editor already given.
      'format.edn.btitle
      { format.edn.btitle ", " * bbl.editor * " " * format.in.editors * }
      if$
  }
  if$
}

FUNCTION {format.full.book.spec} % The title should be on the stack.
{ series empty$
    { format.ed.booktitle
      volume empty$
	{ number empty$
	    'skip$
	    { " there's a number but no series in " cite$ * warning$
	      " No." number tie.or.space.connect * }
	    if$
	}
	{ ", Vol." volume tie.or.space.connect *
	  number empty$
	    'skip$
	    {"Both volume and number fields in " * cite$ * warning$ }
	    if$
	}
	if$
    }
    { volume empty$
	{ format.ed.booktitle ", " * series *
	  number empty$
	    'skip$
	    { " No." number tie.or.space.connect * }
	    if$
	}
	{ series ", Vol." volume tie.or.space.connect *
          ", " * swap$ format.ed.booktitle *
	  number empty$
	    'skip$
	    {"Both volume and number fields in " * cite$ * warning$ }
	    if$
	}
        if$
    }
    if$
}

FUNCTION {maybe.doi.link.prepend}
{ % Expects some citation text to be on the stack
 doi empty$
  { url empty$
     { }
     { "\href{" url * "}{" * swap$ * }
    if$
  }
  { "\href{http://dx.doi.org/" doi * "}{" * swap$ * }
 if$
}

FUNCTION {maybe.doi.link.end}
{ % Expects some citation text to be on the stack
 doi empty$ url empty$ and
  { }
  { "}" * }
 if$
}

FUNCTION {format.journal}
{
  journal empty$
    { "" } % warning will be issued by output.check
    { "\JournalTitle{" journal * "}" * }
  if$
}

FUNCTION {article}
{ output.bibitem
  format.authors "author" output.check
  author format.key output
  name.or.dash
  format.date "year" output.check
  crossref missing$
    { format.journal "journal" output.check
      maybe.doi.link.prepend
      format.vol.num.pages output
    }
    { format.article.crossref
      maybe.doi.link.prepend
      output.nonnull
      format.pages output
    }
  if$
  format.journal.pages
  maybe.doi.link.end
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {book}
{ output.bibitem
  author empty$
    { format.editors "author and editor" output.check
      editor format.key output
      name.or.dash
    }
    { format.authors output.nonnull
      name.or.dash
      crossref missing$
        { "author and editor" editor either.or.check }
        'skip$
      if$
    }
  if$
  format.date "year" output.check
  title format.full.book.spec output
  format.publisher
  format.pages output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {booklet}
{ output.bibitem
  format.authors output
  author format.key output
  name.or.dash
  format.date "year" output.check
  format.title "title" output.check
  howpublished output
  address output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {inbook}
{ output.bibitem
  author empty$
    { format.editors "author and editor" output.check
      editor format.key output
      name.or.dash
    }
    { format.authors output.nonnull
      name.or.dash
      crossref missing$
        { "author and editor" editor either.or.check }
        'skip$
      if$
    }
  if$
  format.date "year" output.check
  title format.full.book.spec output
  format.publisher
  format.pages "pages" output.check
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {incollection}
{ output.bibitem
  format.authors "author" output.check
  author format.key output
  name.or.dash
  format.date "year" output.check
  bbl.in " " * booktitle format.full.book.spec *
  maybe.doi.link.prepend maybe.doi.link.end output
  format.publisher
  format.pages "pages" output.check
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {inproceedings}
{ output.bibitem
  format.authors "author" output.check
  author format.key output            % added
  format.date "year" output.check
  bbl.in " " * booktitle format.full.book.spec *
  maybe.doi.link.prepend maybe.doi.link.end output
  publisher empty$
    { organization output
      address output
    }
    { organization output
      format.publisher
    }
  if$
  format.pages output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {conference} { inproceedings }

FUNCTION {manual}
{ output.bibitem
  format.authors output
  author format.key output
  name.or.dash
  format.date "year" output.check
  format.btitle "title" output.check
  format.edition output
  organization output
  address output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {mastersthesis}
{ output.bibitem
  format.authors "author" output.check
  author format.key output
  name.or.dash
  format.date "year" output.check
  bbl.mthesis format.thesis.type output.nonnull
  school "school" output.check
  address output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {misc}
{ output.bibitem
  format.authors output
  author format.key output
  name.or.dash
  format.date "year" output.check
  format.title output
  howpublished output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {phdthesis}
{ output.bibitem
  format.authors "author" output.check
  author format.key output
  name.or.dash
  format.date "year" output.check
  bbl.phdthesis format.thesis.type output.nonnull
  school "school" output.check
  address output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {proceedings}
{ output.bibitem
  editor empty$
    { organization output
      organization format.key output }
    { format.editors output }
  if$
  name.or.dash
  format.date "year" output.check
  title format.full.book.spec output
  publisher empty$ not      % No need for warning if no pub.
    { format.publisher }
    { editor empty$         % For empty editor, organization was already given.
        'skip$
        { organization output }
      if$
      address output
    }
  if$
  format.pages output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {techreport}
{ output.bibitem
  format.authors "author" output.check
  author format.key output
  name.or.dash
  format.date "year" output.check
  format.title "title" output.check
  format.tr.number output.nonnull
  institution "institution" output.check
  address output
  format.eprint output
  format.note output
  fin.entry
}

FUNCTION {unpublished}
{ output.bibitem
  format.authors "author" output.check
  author format.key output
  name.or.dash
  format.date "year" output.check
  format.eprint output
  format.note "note" output.check
  fin.entry
}

FUNCTION {default.type} { misc }

READ

FUNCTION {sortify}
{ purify$
  "l" change.case$
}

INTEGERS { len }

FUNCTION {chop.word}
{ 's :=
  'len :=
  s #1 len substring$ =
    { s len #1 + global.max$ substring$ }
    's
  if$
}

FUNCTION {format.lab.names}
{ 's :=
  s #1 "{vv~}{ll}" format.name$
  s num.names$ duplicate$
  #2 >
    { pop$
      " {et~al.}" *
    }
    { #2 <
        'skip$
        { s #2 "{ff }{vv }{ll}{ jj}" format.name$ "others" =
            {
              " {et~al.}" *
            }
            { " \& " * s #2 "{vv~}{ll}" format.name$
              * }
          if$
        }
      if$
    }
  if$
}

FUNCTION {author.key.label}
{ author empty$
    { key empty$
        { cite$ #1 #3 substring$ }
        'key
      if$
    }
    { author format.lab.names }
  if$
}

FUNCTION {author.editor.key.label}
{ author empty$
    { editor empty$
        { key empty$
            { cite$ #1 #3 substring$ }
            'key
          if$
        }
        { editor format.lab.names }
      if$
    }
    { author format.lab.names }
  if$
}

FUNCTION {editor.key.label}
{ editor empty$
    { key empty$
        { cite$ #1 #3 substring$ }
        'key
      if$
    }
    { editor format.lab.names }
  if$
}

FUNCTION {calc.short.authors}
{ type$ "book" =
  type$ "inbook" =
  or
    'author.editor.key.label
    { type$ "proceedings" =
        'editor.key.label
        'author.key.label
      if$
    }
  if$
  'short.list :=
}

FUNCTION {calc.label}
{ calc.short.authors
  short.list
  "("
  *
  year duplicate$ empty$
     { pop$ "????" }
     'skip$
  if$
  *
  'label :=
}

FUNCTION {sort.format.names}
{ 's :=
  #1 'nameptr :=
  ""
  s num.names$ 'numnames :=
  numnames #8 >
    { s nameptr
      "{vv{ } }{ll{ }}{  f{ }}{  jj{ }}"
      format.name$ 't :=
      t sortify *
      "   "  *
      "zzzzz" *
    }
    { numnames 'namesleft :=
      { namesleft #0 > }
      { s nameptr
        "{vv{ } }{ll{ }}{  f{ }}{  jj{ }}"
        format.name$ 't :=
        nameptr #1 >
          {
            "   "  *
            namesleft #1 = t "others" = and
              { "zzzzz" * }
              { t sortify * }
            if$
          }
          { t sortify * }
        if$
        nameptr #1 + 'nameptr :=
        namesleft #1 - 'namesleft :=
      }
    while$
    }
  if$
}

FUNCTION {sort.format.title}
{ 't :=
  "A " #2
    "An " #3
      "The " #4 t chop.word
    chop.word
  chop.word
  sortify
  #1 global.max$ substring$
}

FUNCTION {author.sort}
{ author empty$
    { key empty$
        { "to sort, need author or key in " cite$ * warning$
          ""
        }
        { key sortify }
      if$
    }
    { author sort.format.names }
  if$
}

FUNCTION {author.editor.sort}
{ author empty$
    { editor empty$
        { key empty$
            { "to sort, need author, editor, or key in " cite$ * warning$
              ""
            }
            { key sortify }
          if$
        }
        { editor sort.format.names }
      if$
    }
    { author sort.format.names }
  if$
}

FUNCTION {editor.sort}
{ editor empty$
    { key empty$
        { "to sort, need editor or key in " cite$ * warning$
          ""
        }
        { key sortify }
      if$
    }
    { editor sort.format.names }
  if$
}

FUNCTION {presort}
{ calc.label
  label sortify
  "    "
  *
  type$ "book" =
  type$ "inbook" =
  or
    'author.editor.sort
    { type$ "proceedings" =
        'editor.sort
        'author.sort
      if$
    }
  if$
  #1 entry.max$ substring$
  'sort.label :=
  sort.label
  *
  "    "
  *
  title field.or.null
  sort.format.title
  *
  #1 entry.max$ substring$
  'sort.key$ :=
}

ITERATE {presort}

SORT

STRINGS { last.label next.extra }

INTEGERS { last.extra.num number.label }

FUNCTION {initialize.extra.label.stuff}
{ #0 int.to.chr$ 'last.label :=
  "" 'next.extra :=
  #0 'last.extra.num :=
  #0 'number.label :=
}

FUNCTION {forward.pass}
{ last.label label =
    { last.extra.num #1 + 'last.extra.num :=
      last.extra.num int.to.chr$ 'extra.label :=
    }
    { "a" chr.to.int$ 'last.extra.num :=
      "" 'extra.label :=
      label 'last.label :=
    }
  if$
  number.label #1 + 'number.label :=
}

FUNCTION {reverse.pass}
{ next.extra "b" =
    { "a" 'extra.label := }
    'skip$
  if$
  extra.label 'next.extra :=
  extra.label
  duplicate$ empty$
    'skip$
    { "{\natexlab{" swap$ * "}}" * }
  if$
  'extra.label :=
  label extra.label * 'label :=
}

EXECUTE {initialize.extra.label.stuff}

ITERATE {forward.pass}

REVERSE {reverse.pass}

FUNCTION {bib.sort.order}
{ sort.label
  "    "
  *
  year field.or.null sortify
  *
  "    "
  *
  title field.or.null
  sort.format.title
  *
  #1 entry.max$ substring$
  'sort.key$ :=
}

ITERATE {bib.sort.order}

SORT

FUNCTION {begin.bib}
{ preamble$ empty$
    'skip$
    { preamble$ write$ newline$ }
  if$
  "\begin{thebibliography}{}"
  write$ newline$
  "\providecommand\natexlab[1]{#1}"
  write$ newline$
  "\providecommand\JournalTitle[1]{#1}"
  write$ newline$
}

EXECUTE {begin.bib}

EXECUTE {init.state.consts}

ITERATE {call.type$}

FUNCTION {end.bib}
{ newline$
  "\end{thebibliography}" write$ newline$
}

EXECUTE {end.bib}
