# Introduction

This book describes the Tectonic typesetting system. The goal of the Tectonic
project is to empower people to create beautiful, effective technical
documents.

## Tectonic and TeX

At the core of Tectonic is a modernized, complete, self-contained
[TeX]/[LaTeX] engine, powered by [XeTeX] and [TeXLive].

[TeX]: https://en.wikipedia.org/wiki/TeX
[LaTeX]: https://www.latex-project.org/
[XeTeX]: http://xetex.sourceforge.net/
[TeXLive]: https://www.tug.org/texlive/

For those new to it, [TeX] is a programming language that you use to create
typeset documents rather than computer software. TeX is quite archaic in some
ways, but it’s still the tool of choice for what we call *technical
documents*. What makes a document technical?

- *Mathematics*. Some documents are technical because they involve
  mathematical equations. Beautiful mathematical typesetting is *extremely
  hard* and requires deep integration with the overall typesetting system. The
  absolutely unique feature of [TeX] is its unparalleled ability to typeset
  math.
- *Cross-references*. Technical documents often involve extensive internal and
  external cross-references (links), and managing such links is a nightmare
  without extremely good tooling support. The [TeX] ecosystem provides
  infrastructure for deep and rich cross-referencing with programs like
  [bibtex].
- *Rich content*. Technical documents also generally include a great deal of
  rich content beyond their text, such as figures, tables, and code. In order
  to be useful, all of this content needs to be seamlessly integrated into the
  document presentation, with precise author control over that presentation.
  This is another hallmark of the [TeX] ecosystem.
- *Hackability*. Finally, we also believe that the ideal technical documents
  is “hackable,” meaning that people should be able to see how it works “under
  the hood” and use it as a basis for their own experiments. Because [TeX] is
  a programming language for creating documents, [TeX]-based documents can be
  hackable in exactly the same way as the open-source programs that underly so
  much of the modern software ecosystem.

[bibtex]: http://www.bibtex.org/

The fundamental principle underlying the Tectonic project is that [TeX] is,
and can continue to be, the best language out there for creating the
beautiful, effective technical documents that the world deserves. The [TeX]
language is an amazingly clever piece of engineering, and the fact that it’s
still in use 40 years (!) after its creation speaks for itself. But by the
same token, there is a *lot* about the [TeX] software ecosystem that is
archaic and outdated. The goal of Tectonic is to take the good stuff and leave
behind the things that don’t make sense anymore.

In particular, Tectonic is derived from the source code that powers the
[XeTeX] engine, and the bulk of its code is the same core engine that
implements the complex, Unicode-aware typesetting performed by [XeTeX].
Tectonic provides both a new user experience around that engine, and several
key interventions that enable the engine to be used in fundamentally new ways.

## The goals of Tectonic

As stated above, the overall goal of the Tectonic project is to enable people
to create beautiful, effective technical documents. In particular, there are
several elements of the existing [TeX] ecosystem that Tectonic aims to improve
upon:

- *User experience*. Many aspects of the classic [TeX] user experience (UX)
  are bizarre and unpleasant to modern users. In particular, its error
  messages and diagnostic output can be utterly mystifying to new users.
  Tectonic chooses to break compatibility with classic [TeX] when doing so
  offers the chance to improve the UX.
- *Embeddability*. The modern [TeX] software system consists of a suite of
  interacting command-line programs modifying a complex tree of thousands of
  support files. This makes it extremely unpleasant to embed the [TeX] engine
  within other software systems, which prevents a whole host of exciting use
  cases. Tectonic delivers its engine as a reusable software library and aims
  to make that library easy to embed and reuse anywhere code can run.
- *Reproducibility*. For the same reason that the classic [TeX] experience is
  difficult to embed, it is difficult to guarantee reproducible document
  builds. For many technical documents, reproducibility is a highly-respected
  virtue if not an outright requirement. Tectonic aims to enable easy,
  byte-for-byte reproducible builds.
- *Web output*. Modern displays and Web browsers are incredibly powerful,
  versatile tools. One of the motivations for the founding of the Tectonic
  project was the belief that current Web-based technical documents are
  falling far short of what should be possible, and the belief that some
  changes in the core [TeX] engine are necessary to fully unlock its ability
  to produce excellent Web-based output.
