# Getting Started: Use a Unicode Font

You’ve [created your first Tectonic document][first-document]. Great! Now, let’s
start exploring the ways in which Tectonic helps you create modern [technical
documents][tech-docs].

[first-document]: ./first-document.md
[tech-docs]: ../introduction/index.md#technical-documents

We’ve already seen a few ways that Tectonic differs from traditional TeX
engines. Perhaps the most fundamentally important difference, however, is the
one that we’ll explore in this section.


## Unicode

When TeX was first developed [more than 40 years ago][tex-history], digital
systems for representing human writing were pretty primitive. Because TeX needed
to represent a variety of languages as well as a mathematics, it was endowed
with comparatively sophisticated tools to both accept and emit characters that
aren’t found in standard English. TeX’s good multi-lingual support was one of
the things that made it groundbreaking.

[tex-history]: https://en.wikipedia.org/wiki/TeX#History

Eventually, a consortium of major technology companies developed the set of
[Unicode] standards for representing the world’s writing systems digitally. By
now, they’re universally adopted. So while the TeX ecosystem started out ahead
of the curve, some of its core systems are designed around an incompatible and,
frankly, dead-end framework.

[Unicode]: https://home.unicode.org/

That is not to imply that the whole TeX universe is stuck in the past! Many
people have worked extremely hard to bridge the worlds of TeX and Unicode. The
code in Tectonic, in particular, is based the [XeTeX] project, which adds
Unicode support to the classic TeX experience. Without the efforts of the
[XeTeX] team and many other dedicated volunteers in the TeX world, Tectonic
wouldn’t be able to offer Unicode support.

[XeTeX]: http://xetex.sourceforge.net/


## Modern Fonts
