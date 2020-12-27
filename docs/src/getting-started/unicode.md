# Getting Started: Use a Unicode Font

You’ve [created your first Tectonic document][first-document]. Great! Now, let’s
start exploring the ways in which Tectonic helps you create modern [technical
documents][tech-docs].

[first-document]: ./first-document.md
[tech-docs]: ../introduction/index.md#technical-documents

We’ve already seen a few ways that Tectonic differs from traditional TeX
engines. Perhaps the most fundamentally important difference, however, is the
one that we’ll explore in this section.

**Note:** *This [Getting Started][gs-index] guide uses what we call the [“V2”
interface][v2cli] to the Tectonic command-line tool. The V2 interface coexists
with, but has a fairly different approach than, the [“V1”interface][v1cli]. We
are gradually migrating from V1 to V2. Neither interface (V1 or V2) is the same
as the one exposed by classic TeX tools such as `pdflatex`.*

[gs-index]: ./index.md
[v2cli]: ../ref/v2cli.md
[v1cli]: ../ref/v1cli.md


## Unicode

When TeX was first developed [more than 40 years ago][tex-history], digital
systems for representing human writing were pretty primitive. Because TeX needed
to represent a variety of languages *and* mathematics, it was endowed with
comparatively sophisticated tools to both accept and emit characters that aren’t
found in standard English. TeX’s good multi-lingual support was one of the
things that made it groundbreaking. Eventually, however, a consortium of major
technology companies developed the much more comprehensive [Unicode] standards
for representing the world’s writing systems digitally. By now, they’re
universally adopted. So while the TeX ecosystem started out ahead of the curve,
some of its core systems are designed around an incompatible and, frankly,
dead-end framework.

[tex-history]: https://en.wikipedia.org/wiki/TeX#History
[Unicode]: https://home.unicode.org/

That is not to imply that the whole TeX universe is stuck in the past! Many
people have worked extremely hard to bridge the worlds of TeX and Unicode. The
code in Tectonic, in particular, is based on [XeTeX], which adds Unicode support
to the classic TeX experience. Without the efforts of the [XeTeX] team and many
other dedicated volunteers in the TeX world, Tectonic wouldn’t be able to offer
Unicode support.

[XeTeX]: http://xetex.sourceforge.net/


## Modern Fonts

Tectonic’s support for Unicode allows it to take advantage of modern fonts that
are distributed using formats such as [OpenType]. Besides opening up access to a
whole world of typographic progress — a good font is the result of *years* of
expert effort — this support positions Tectonic to create outputs in not just
[PDF] but [HTML] formats. HTML capability is still under development, but it’s
one of the prime reasons that the Tectonic project was started.

[OpenType]: https://en.wikipedia.org/wiki/OpenType
[PDF]: https://en.wikipedia.org/wiki/PDF
[HTML]: https://en.wikipedia.org/wiki/HTML

The choice of fonts is foundational to TeX’s document processing, so modern
fonts aren’t automatically activated. To start using a nice new font like [TeX
Gyre Pagella][pagella] (derived from [Palatino]), open up your
`src/_preamble.tex` file and add the following lines after the `\documentclass`
command:

[pagella]: https://www.fontsquirrel.com/fonts/tex-gyre-pagella
[Palatino]: https://en.wikipedia.org/wiki/Palatino

```tex
\usepackage{fontspec}
\setmainfont{texgyrepagella}[
  Extension = .otf,
  UprightFont = *-regular,
  BoldFont = *-bold,
  ItalicFont = *-italic,
  BoldItalicFont = *-bolditalic,
]
```

Now rebuild your document:

```sh
$ tectonic -X build
```

You’ll probably see Tectonic download some files: namely, the new font files
that you have started using. Tectonic’s ability to fetch such files on the fly
is why its installation is so much easier than a traditional TeX engine.

If you open up your rebuilt document, it will be using your new font, although
the difference can be difficult to detect with such a small amount of sample
text.


## Unicode Input Text

Tectonic’s support for Unicode broadens its *output* capabilities through the
use of modern fonts. But that’s not all: Unicode also broadens Tectonic’s the
*inputs* that Tectonic accepts.

With Tectonic, you can type non-English characters directly into your input TeX
files, which are parsed assuming the virtually-universal [UTF-8 Unicode text
encoding][utf8]. For instance, you can open up your `src/index.tex` file and
copy-paste in the following [verse][bateau] with accented characters:

```tex
Ô que ma quille éclate! Ô que j’aille à la mer!
```

[utf8]: https://en.wikipedia.org/wiki/UTF-8
[bateau]: https://en.wikipedia.org/wiki/Le_Bateau_ivre

Rebuild your document and see your new text in the output:

```sh
$ tectonic -X build
```

*Whatever*, you might say. *I know how to get these accented characters with TeX
commands:*

```tex
\^O que ma quille \'eclate! \^O que j'aille \`a la mer!
```

Fair enough. But now try typing in [Bashô’s “old pond” haiku][old-pond]:

```tex
古池や蛙飛び込む水の音
  ふるいけやかわずとびこむみずのおと
```

We’ll wait.

[old-pond]: https://www.japantimes.co.jp/news/2019/10/19/national/history/frog-jump-bashos-pond/

(Note, however, that if you copy-paste this text into our sample document, it
won’t work: you haven’t activated a font able to handle the Japanese characters.
You’ll get a lot of warnings to that effect.)
