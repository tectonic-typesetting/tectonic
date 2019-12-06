# 0.1.12 (2019 Dec 6)

It has been just more than a year since the last Tectonic release, mainly
because I (@pkgw)
[started a new job](https://newton.cx/~peter/2018/operation-innovation/) that
has massively restructured how I spend my time. But that is not to say that
things have been quiet for Tectonic! I count 81 pull requests merged since
0.1.11. (Ignoring automated ones issued by Dependabot.)

User-facing improvements:

- Thanks to @efx we now have the beginnings of
  [a Tectonic book](https://tectonic-typesetting.github.io/book/)! It is
  currently very sparse, but we hope to gradually flesh it out. The book is
  updated automatically upon merges to `master` and with tagged releases as
  well (if @pkgw wired up the infrastructure correctly). (#427, #444, #445,
  #447, #505; @efx @PHPirates @pkgw)
- Tectonic’s caching scheme is now much smarter, saving a local copy of the
  table-of-contents file associated with each online bundle. This means that
  Tectonic doesn’t need to hit the network at all if a new file is referenced
  that is not present in the bundle, and saves a large download if a new
  needed file *is* present in the bundle. (#431; @rekka)
- Performance has been improved by avoiding the computation of SHA256 hashes
  for read-only files. Since these files are known not to change, we don’t
  have to monitor their contents. (#453; @rekka)
- Warnings are only flagged if they occur on the final pass of the TeX engine,
  since sometimes ones that occur in the first pass get fixed by subsequent
  reruns. (#458; @rekka)

There have been a *ton* of developer-facing improvements:

- Tectonic is now built using the Rust 2018 edition! (#388; @Mrmaxmeier)
- @Mrmaxmeier built an amazing system to start doing
  [crater](https://github.com/rust-lang/crater)-like runs of Tectonic on the
  [arxiv.org](https://arxiv.org) corpus, yielding bug fixes across the
  codebase including issues with obscure PNG formats. (#401; @Mrmaxmeier)
- It is now possible to build various statically-linked versions of Tectonic.
  One way to accomplish this is to turn off the new `serialization` Cargo
  feature. This eliminates the use of Rust “procedural macros” in the build
  process which in turn allows Tectonic to be built on statically-linked
  platforms. (Note, however, that it is possible to build Tectonic for
  statically-linked platforms *with* the serialization feature by
  cross-compiling it from a dynamically-linked platform. This is the tactic
  used by the Tectonic CI build system.) @efx also
  [wrote instructions for how to accomplish a mostly-static build on macOS using vcpkg](https://tectonic-typesetting.github.io/book/latest/cookbook/vcpkg.html)
  as well as
  [how to do it on Linux/musl using Docker](https://github.com/tectonic-typesetting/tectonic/tree/master/dist/docker/x86_64-alpine-linux-musl)
  (#260, #261, #325, #425, #451; @efx, @malbarbo, @pkgw)
- Tectonic now distributes a continuous-deployment
  [AppImage](https://appimage.org/) build. (#283, #285; @pkgw, @probonopd,
  @xtaniguchimasaya)
- The size of the binary has decreased somewhat by using a smaller collection
  of compression libraries; avoiding the use of exceptions and RTTI in the
  compiled C++ code; avoiding the use of the `aho_corasick` crate; and making
  the `toml` crate optional. (#428, #439, #440, #491; @malbarbo)
- Tectonic now uses the
  [reqwest](https://docs.rs/reqwest/0.10.0-alpha.2/reqwest/) crate as its HTTP
  backend instead of direct use of [hyper](https://hyper.rs/). Reqwest offers
  a simpler interface and adds better support for things like HTTP proxies and
  cookie handling. These new features do increase the binary
  size somewhat. (#330, @spl)
- Tectonic can now be built on `x86_64-pc-windows-msvc` by using
  [vcpkg](https://github.com/microsoft/vcpkg) to discover dependent libraries.
  This can be activated by setting the new environment variable
  `TECTONIC_DEP_BACKEND=vcpkg` during the build process. (#420; @mcgoo)
- Potential issues with cross-compilation are fixed by properly respecting
  `CARGO_TARGET_*` environment variables rather than using `cfg!()` macros,
  which have the wrong values in the `build.rs` script. This support is
  provided by a new `tectonic_cfg_support` crate that may be of interest to
  other projects. (#477; @pkgw @ratmice)
- Tectonic now comes with beta-level fuzzing support using
  [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz). It is hoped that
  eventually this infrastructure will help identify and close some truly
  obscure and gnarly bugs in the Tectonic language implementation. At present,
  the usefulness of the fuzzer is limited by memory leaks within multiple
  reruns of the Tectonic engine, although in the process of setting up the
  fuzzer several egregious leaks were fixed. (#315; @cyplo @pkgw)
- The Rust codebase is now formatted according to
  [rustfmt](https://github.com/rust-lang/rustfmt) and generates no
  [clippy](https://github.com/rust-lang/rust-clippy) complaints, and the CI
  system now checs for these. (#282, #336, #337, #338, #339, #340, #341, #342,
  #343, #344, #345, #346, #347, #348, #349, #352, #353; @pkgw @spl)
- A new `profile` feature allows building a debug version of the program
  suitable for profiling. (#511; @ratmice)
- The test suite now covers the bibtex tool. (#407; @Mrmaxmeier)
- The test suite also now covers the local cache and tar bundle code. (#441;
  @rekka)
- The CLI client now parses arguments using the `structopt` crate. (#465, #474;
  @efx @Mrmaxmeier)
- A new `DirBundle` bundle backend provides a simple way for the engine to
  access a bunch of files in a directory, although it is not yet wired up
  to the CLI interface in a convenient way. (#492; @malbarbo)
- The current date tracked by the TeX engine is now settable from the Rust
  level. (#486; @Mrmaxmeier).
- More cleanups and symbolification of the C/C++ code (#317, #327, #350, #398;
  @Mrmaxmeier @pkgw @spl)
- C++ compilation on certain versions of g++ was fixed (#265; @pkgw)
- Fix deprecation warnings from the `error_chain` crate (#351; @spl)
- Improvements to the Travis CI infrastructure, output clarity, and
  reliability. (#354, #360, #362, #394, #424, #443; @efx @rekka @spl)
- Attempts were made to increase the reliability of the Circle CI build, which
  uses QEMU to compile Tectonic for a big-endian architecture. Unfortunately
  it still just times out sometimes. (#290, #296; @pkgw)
- The deprecated `tempdir` crate has been replaced with `tempfile`. (#387;
  @ratmice)
- Usage of `app_dirs` directories is now encapsulated better. (#429, #432;
  @malbarbo @rekka)
- Bugs in reading unusual PDF files were fixed. (#396; @pkgw)
- A missing space in bibtex error messages is now back. (#485; @jneem)
- A memory corruption issue in the bibtex engine was fixed. (#493; @jneem)

# 0.1.11 (2018 Nov 5)

This release is mainly about the following change:

- The URL embedded in the code that points to the default bundle has been
  changed to point to the archive.org domain. Hopefully this will result in
  more reliable service — there have been problems with SSL certificate
  updates on purl.org in the past
  ([#253](https://github.com/tectonic-typesetting/tectonic/pull/253)).

Developer-facing improvements:

- The main crate now provides an all-in-one function,
  `tectonic::latex_to_pdf()`, that does what it says, using “sensible”
  defaults. Run a full TeX processing session, end-to-end, in a single
  function call!
  ([#252](https://github.com/tectonic-typesetting/tectonic/pull/252))
- In support of the previous change, the behavior of the Rust code was changed
  to use a static global
  [mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html) to serialize
  invocations of the C/C++ engine implementations, which currently include
  massive amounts of global state and thus cannot be run in a multithreaded
  fashion. The recommended approach used to be for users of the library to
  provide such a mutex themselves. [@pkgw](https://github.com/pkgw) was
  initially reluctant to include such a mutex at the crate level since he
  feared the possibility of weird surprising behavior … but the *real* weird
  surprising behavior is when you try to run the engine in a multithreaded
  program and it blows up on you!
- *Also* in support of the previous change, the framework for running the test
  suite has been revamped and improved. We can now run doctests that invoke
  the full engine, and the tests of the executable artifacts now activate a
  special debug mode that prevents accesses of the network and/or the calling
  user’s personal resource file cache.
- The usual work on tidying the C/C++ code, and also more work towards the
  planned HTML output mode. Activating the experimental “semantic pagination”
  mode now alters the engine behavior in two key ways: it disables the
  linebreaker and custom output routines. This breaks processing of all extant
  documents, but [@pkgw](https://github.com/pkgw) believes that these changes
  are important steps toward reliable generation of HTML output.
  ([#237](https://github.com/tectonic-typesetting/tectonic/pull/237),
  [#239](https://github.com/tectonic-typesetting/tectonic/pull/239),
  [#245](https://github.com/tectonic-typesetting/tectonic/pull/245),
  [#250](https://github.com/tectonic-typesetting/tectonic/pull/250))


# 0.1.10 (2018 Sep 28)

This release is mainly about upgrading a dependency related to SSL/TLS to
increase the range of systems on which Tectonic can be compiled.

User-facing improvements:

- Tectonic now correctly handles Unicode filenames — even ones containing
  emoji! — without crashing
  ([#165](https://github.com/tectonic-typesetting/tectonic/pull/165)).

Developer/packager-facing improvements:

- Tectonic now depends on the 0.3.x series of
  [hyper-native-tls](https://crates.io/crates/hyper-native-tls), which can
  build against the 1.1.x series of [OpenSSL](https://www.openssl.org/).


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
  into the Tectonic library and has been made (more) reusable
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
