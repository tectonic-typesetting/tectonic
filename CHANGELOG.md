# The next version

The following changes will appear in the next release:

- None yet

# 0.1.9 (2018 Sep 15)

User-facing improvements:

- Tectonic is now available on Windows!
  ([#210](https://github.com/tectonic-typesetting/tectonic/pull/210),
  [#231](https://github.com/tectonic-typesetting/tectonic/pull/231)). There
  are likely to be rough edges to both the developer and user experience, but
  the test suite passes and Windows is now included in the CI infrastructure.
  Big thanks to new contributor [@crlf0710](https://github.com/crlf0710) who
  really got the ball rolling on this important milestone.
- Fully offline operation is now much improved:
  - There is a new `--only-cached` (AKA `-C`) option that will avoid all
    Internet connections
    ([#203](https://github.com/tectonic-typesetting/tectonic/pull/203)). While
    Tectonic takes pains to avoid needing an Internet connection when compiling
    documents, there are still times when you can get more done by explicitly
    preventing it from even trying to talk to the network.
  - The `--bundle` and `--web-bundle` options finally work again. The switch
    to on-the-fly generation of format files broke them due to an internal
    implementation problem; this has now been fixed
    ([[#181](https://github.com/tectonic-typesetting/tectonic/pull/181)).
  - If you put a `file://` URL into your Tectonic configuration file as your
    default bundle, Tectonic will now load it correctly
    ([#211](https://github.com/tectonic-typesetting/tectonic/pull/211)).

Internal improvements:

- Tectonic now avoids panicking from Rust into C code, which is not supported
  behavior ([#91](https://github.com/tectonic-typesetting/tectonic/pull/91)).
  Thanks to [@rekka](https://github.com/rekka) for persistence in getting this
  one across the finish line.
- Tectonic now avoids crashing when trying to open empty filenames
  ([#212](https://github.com/tectonic-typesetting/tectonic/pull/212)).

Developer-facing improvements:

- Tectonic is now more up-front about the fact that it requires Harfbuzz
  version 1.4 or higher.
- Much of the code that drives compilation for the CLI tool has been moved
  into the Tectonic library and mode (more) reusable
  ([#184](https://github.com/tectonic-typesetting/tectonic/pull/184)). Thanks
  to new contributor [@jneem](https://github.com/jneem) for doing this!

# 0.1.8 (2018 Jun 17)

This release contains a variety of bugfixes and features-in-development.

User-facing improvements:

- A prominent warning is now emitted when missing characters are encountered
  in a font. The hope is that this will help un-confuse users who include
  Unicode characters in their input files without loading a Unicode-capable
  font. Before this change, such characters would just not appear in the
  output document.
- Fix the implementation of the DVI “POP” operator, which was broken due to a
  typo. This should fix various corner-case failures to generate output.
- The `.toc` and `.snm` output files emitted by Beamer are now treated as
  intermediate files, and therefore not saved to disk by default (contributed
  by Norbert Pozar).
- Various hardcoded `bibtex` buffer sizes are increased, allowing larger
  bibliographies to be handled.
- Format files are now stored uncompressed. The compression did not save a ton
  of disk space, but it did slow down debug builds significantly (contributed
  by @Mrmaxmeier).
- The C code has been synchronized with XeTeX as of its Subversion
  revision 46289. The chief difference from before is the use of newer
  [Harfbuzz](https://www.freedesktop.org/wiki/Software/HarfBuzz/) features for
  rendering OpenType math fonts, which should substantially improve “Unicode
  math” output.

Work towards HTML output:

- The first steps have been taken! In particular, the engine now has an
  internal flag to enable output to a new “SPX” format instead of XDV. SPX
  stands for Semantically Paginated XDV — based on my (PKGW’s) research, to
  achieve the best HTML output, the engine will have to emit intermediate data
  that are incompatible with XDV. At the moment, SPX is the same as XDV except
  with different identifying bytes, but this will change as the work towards
  excellent HTML output continues. The command-line tool does **not** provide
  access to this output format yet, so this work is currently purely internal.
- In addition, there is a stub engine called `spx2html` that will translate
  SPX to HTML. At the moment it is a barely-functional proof-of-concept hook,
  and it is not exposed to users.
- A new internal crate, `tectonic_xdv`, is added. It can parse XDV and SPX
  files, and is used by the `spx2html` engine.

Test suite improvements:

- The test suite now supports reliable byte-for-byte validation of PDF output
  files, through the following improvements:
  - It is now possible for the engine to disable PDF compression (contributed
    by @Mrmaxmeier).
  - `xdvipdfmx` gained a mode to reproducibly generate the “unique tags”
    associated with fonts.
- The testing support code is now centralized in a single crate (contributed
  by @Mrmaxmeier).
- Continuous integration (CI) coverage now includes Linux and a big-endian
  platform.
- The CI coverage now includes code coverage monitoring.

Internal improvements:

- Much of the command-line rebuild code has been moved inside the `tectonic`
  crate so that it can be reused in a library context (contributed by @jneem).

Improvements to the C code. As usual, there has been a great deal of tidying
that aims to make the code more readable and hackable without altering the
semantics. Many such changes are omitted below.

- Tectonic’s synchronization with XeTeX is now tracked in version control
  formally, by referencing the
  [tectonic_staging](https://github.com/tectonic-typesetting/tectonic-staging)
  repository as a Git submodule. It is not actually necessary to check out
  this submodule to build Tectonic, however.
- The C code now requires, and takes advantage of, features in the
  [C11](https://en.wikipedia.org/wiki/C11_(C_standard_revision)) revision of
  the language standard.
- All remaining pieces of C code that needed porting to the Rust I/O backend
  have been ported or removed.
- Virtually all hardcoded strings in the string pool have been removed
  (contributed by @Mrmaxmeier).
- The C code has been split into a few more files. Some subsystems, like the
  “shipout” code, use a lot of global variables that have been made static
  thanks to the splitting.
- A big effort to clarify the pervasive and unintuitive `memory_word`
  structure.
- Effort to tidy the `line_break()` function and significantly increase its
  readability. This is in support of the goal of producing HTML output, for
  which I believe it is going to be necessary to essentially defuse this
  function.


# 0.1.7 (2017 Nov 15)

(Copy-pasted from [the relevant forum post](https://tectonic.newton.cx/t/announcing-tectonic-0-1-7/76)).

This is a fairly modest release — things have been very quiet lately as real
life and the day job have dominated my time.

The most visible change is that I just figured out how to fix
[issue #58](https://github.com/tectonic-typesetting/tectonic/issues/58) —
Tectonic would fail to parse certain PDF images if one tried to include them
in a document. There was a bit of a silly bug in the Rust I/O layer that was
not getting exposed except in fairly specialized circumstances. It’s squashed
now! So certain documents that used to fail to compile should work now.

There’s also been yet more nice work behind the scenes by some of our
indefatigable contributors:

- @Mrmaxmeier contributed a whole bunch of cleanups to the C code as well as
  fixes that should allow you to generate multiple PDFs inside a single
  process.
- Ronny Chevalier updated the compilation infrastructure to work in parallel
  and in the end contributed some features to Rust’s upstream [gcc] crate!

There have been intermittent problems lately with the SSL certificate to the
purl.org domain which we use to seed the default “bundle” of LaTeX files. It’s
working for me at the moment, so it’s not totally busted, but the problem
seems to have come and gone over the past few weeks. See
[this thread](https://tectonic.newton.cx/t/problems-caused-by-expired-ssl-certificate-for-purl-org/75/1)
for more information and a workaround.


# 0.1.7 (2017 Jul 9)

(Copy-pasted from
[the relevant forum post](https://tectonic.newton.cx/t/announcing-tectonic-0-1-6/45)).

The version increment may be small but the project has seen an enormous amount
of work since the previous release, thanks to an awesome group of new
contributors. Here are some of the highlights:

- Tectonic is now available for installation on Arch Linux as
  [an AUR package](https://aur.archlinux.org/packages/tectonic/) and on macOS
  as [a Homebrew formula](http://brewformulas.org/Tectonic), thanks to the
  hard work of Alexander Bauer, Alexander Regueiro, Jan Tojnar, Kevin Yap, and
  @ilovezfs.
- The web fetching is more robust and safer, using HTTPS by default
  ([#69](https://github.com/tectonic-typesetting/tectonic/pull/69), Ronny
  Chevalier) and more properly handling CDN redirections
  ([#114](https://github.com/tectonic-typesetting/tectonic/pull/114),
  @Mrmaxmeier)
- Input and output filenames with spaces and non-local paths are now handled
  much better all around
  ([#44](https://github.com/tectonic-typesetting/tectonic/pull/44), Alexander
  Bauer; [#89](https://github.com/tectonic-typesetting/tectonic/pull/89),
  Norbert Pozar;
  [#94](https://github.com/tectonic-typesetting/tectonic/pull/94), Peter
  Williams)
- SyncTeX output is now fully supported, activated with the new `--synctex`
  option ([#55](https://github.com/tectonic-typesetting/tectonic/pull/55),
  [#73](https://github.com/tectonic-typesetting/tectonic/pull/73), Norbert
  Pozar)
- The output files can be placed in a directory other than the input directory
  if the new `--outdir` or `-o` option is specified
  ([#104](https://github.com/tectonic-typesetting/tectonic/pull/104), Felix
  Döring)
- Tectonic will cleanly process TeX code provided on standard input if the
  input path argument is `-`
  ([#94](https://github.com/tectonic-typesetting/tectonic/pull/94), Peter
  Williams)
- Tectonic's first new primitive,
  [\TectonicCodaTokens](https://tectonic.newton.cx/t/engine-extension-tectoniccodatokens/16),
  has been added to allow
  [boilerplate-free document processing](https://tectonic.newton.cx/t/boilerplate-free-latex-documents/29)
  (Peter Williams).
- The API docs can be, and are, built on [docs.rs](https://docs.rs/tectonic)
  as of this release.

Furthermore, I've launched a
[Tectonic forum site](https://tectonic.newton.cx/) (running an instance of the
[Discourse.org](https://www.discourse.org/) software). This is a bit of an
experiment since the project is so young and there are of course other venues,
like [GitHub issues](https://github.com/tectonic-typesetting/tectonic/issues)
and the [TeX StackExchange](https://tex.stackexchange.com/), for having
relevant discussions. But, by launching the Discourse site, we gain a venue
for project news (like this announcement!), more open-ended technical
discussions, Tectonic-specific tips and tricks that may not fit the
StackExchange model, and a knowledge base of answers to the roadblocks that
are so common in the TeX/LaTeX ecosystem. We hope that the forums will become
a valuable complement to the other community areas that are already out there.

Here are some more improvements since the 0.1.5 release:

- Some early work has occurred to make it possible to build Tectonic on
  Android ([#105](https://github.com/tectonic-typesetting/tectonic/pull/105),
  Marco Barbosa)
- The project’s build infrastructure is now more efficient
  ([#60](https://github.com/tectonic-typesetting/tectonic/pull/60), Norbert
  Pozar; [#116](https://github.com/tectonic-typesetting/tectonic/pull/116),
  Ronny Chevalier)
- The style of the translated C code has been improved enormously thanks to
  both manual interventions and the use of the neat tool
  [Coccinelle](http://coccinelle.lip6.fr/), reducing warnings and increasing
  cleanliness and portability
  ([#66](https://github.com/tectonic-typesetting/tectonic/pull/66),
  [#76](https://github.com/tectonic-typesetting/tectonic/pull/76),
  [#83](https://github.com/tectonic-typesetting/tectonic/pull/83),
  [#92](https://github.com/tectonic-typesetting/tectonic/pull/92),
  [#107](https://github.com/tectonic-typesetting/tectonic/pull/107),
  [#112](https://github.com/tectonic-typesetting/tectonic/pull/112), Ronny
  Chevalier;
  [#105](https://github.com/tectonic-typesetting/tectonic/pull/105), Norbert
  Pozar; [#94](https://github.com/tectonic-typesetting/tectonic/pull/94),
  [#98](https://github.com/tectonic-typesetting/tectonic/pull/98), Peter
  Williams )
- The test suite now covers behaviors of the Tectonic command-line program
  itself ([#84](https://github.com/tectonic-typesetting/tectonic/pull/84),
  Alexander Bauer)
- We now correctly run `bibtex` when using the `amsrefs` package
  ([#48](https://github.com/tectonic-typesetting/tectonic/pull/48), Norbert
  Pozar)
- Tectonic will correctly try a wider variety of file extensions when trying
  to open resources
  ([#93](https://github.com/tectonic-typesetting/tectonic/pull/93), Marek
  Šuppa; [#100](https://github.com/tectonic-typesetting/tectonic/pull/100),
  Norbert Pozar)
- Cached bundle files are now made read-only
  ([#55](https://github.com/tectonic-typesetting/tectonic/pull/55), Alexander
  Bauer)
- We’ve fixed a subtle path handling issue that was harming generation of the
  standard LaTeX format
  ([#77](https://github.com/tectonic-typesetting/tectonic/pull/77), Norbert
  Pozar)
- Very large bibliographies are now better supported
  ([#87](https://github.com/tectonic-typesetting/tectonic/pull/87), Marek
  Šuppa)
- The UI now makes it clearer that network failures are not likely Tectonic’s
  fault ([#88](https://github.com/tectonic-typesetting/tectonic/pull/88),
  Marek Šuppa)
- It is now theoretically possible to load Omega font metrics files
  ([#97](https://github.com/tectonic-typesetting/tectonic/pull/97), Peter
  Williams)
- Output log files are now produced if `--keep-logs` is specified and an error
  occurs ([#103](https://github.com/tectonic-typesetting/tectonic/pull/103),
  Norbert Pozar)

There are a few known problems with this release:

- Tectonic doesn’t support HTTP proxies, and in some parts of the world you
  can’t access the [purl.org](https://purl.org/) website that Tectonic checks
  for its bundle. You can work around this by
  [creating a custom configuration file](https://tectonic.newton.cx/t/how-to-use-tectonic-if-you-can-t-access-purl-org/44).
- Tectonic doesn’t have a mechanism to invoke the
  [biber](http://biblatex-biber.sourceforge.net/) tool, so it cannot easily
  work for anyone that uses
  [biblatex](http://mirrors.rit.edu/CTAN/help/Catalogue/entries/biblatex.html).
  This is a common complaint so it would be great to see a workaround be
  devised
  ([relevant issue](https://github.com/tectonic-typesetting/tectonic/issues/35))!

Enormous thanks are in order to everyone who’s started contributing to the
project.

# Previous releases

Are not documented here. Consult the Git history.
