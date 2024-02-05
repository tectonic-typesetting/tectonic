# tectonic 0.15.0 (2024-02-05)

This release contains a grab-bag of nice improvements:

- The port of Tectonic’s BibTeX engine to a pure-Rust implementation has been
  completed by [@CraftSpider] ([#1077], [#1083], [#1127], [#1129])! It continues
  to be the case that this change should be invisible to users, but it shows
  that large chunks of Tectonic’s legacy C/C++ code can be migrated to cleaner,
  safer, more maintainable Rust over time.
- The official Linux builds of Tectonic now link against OpenSSL 3.x, instead of
  the old 1.1 series ([#1092], [@pkgw]). This should improve binary
  compatibility on mainstream platforms. If you need a build that uses the older
  series, you’ll have to compile it yourself.
- The `--web-bundle` flag can now be used in more situations, specifically
  `tectonic -X new` and `tectonic -X init` ([#1132], [@bryango]).
- As part of the above work, you can now activate the "V2" interface with the
  `-X` flag in more places on the Tectonic command line.
- The `Tectonic.toml` file used by the "V2" interface now supports a
  `[metadata]` section for arbitrary structured user metadata ([#1120],
  [@rm-dr]). This is useful for custom tools that build on top of Tectonic,
  where you might want to have some custom pieces of information about each
  document in a group.
- The "V2" interface also now supports external commands ([#1103], [@rm-dr]). If
  you have a program named `tectonic-blah` in your search path, running
  `tectonic -X blah` will execute it.
- Running `tectonic -Z help` (as opposed to `tectonic -Zhelp`) now actually
  works ([#1064], [#1084], [@pkgw]).

Build changes:

- You can now cross-compile Tectonic to the `aarch64-unknown-linux-musl` target
  ([#1089], [@pkgw]).
- MIPS is no longer supported as a build target ([#1076], [@CraftSpider]). It
  dropped to Tier 3 support by the Rust language, which makes it difficult to
  support in our continuous integration systems. Sorry, MIPS!

Documentation fixes:

- Update the Arch Linux package URL ([#1119], [@felixonmars])
- Document `shell_escape_cwd` in `Tectonic.toml` ([#1116], [@eljamm])
- Fix a type in the Unicode section ([#1111], [@goyalyashpal])
- Various other corrections ([#1065], [#1068], [@ColeFrench])

[@CraftSpider]: https://github.com/CraftSpider
[#1064]: https://github.com/tectonic-typesetting/tectonic/pull/1064
[#1065]: https://github.com/tectonic-typesetting/tectonic/pull/1065
[#1068]: https://github.com/tectonic-typesetting/tectonic/pull/1068
[#1076]: https://github.com/tectonic-typesetting/tectonic/pull/1076
[#1077]: https://github.com/tectonic-typesetting/tectonic/pull/1077
[#1083]: https://github.com/tectonic-typesetting/tectonic/pull/1083
[#1084]: https://github.com/tectonic-typesetting/tectonic/pull/1084
[#1089]: https://github.com/tectonic-typesetting/tectonic/pull/1089
[#1092]: https://github.com/tectonic-typesetting/tectonic/pull/1092
[#1103]: https://github.com/tectonic-typesetting/tectonic/pull/1103
[#1111]: https://github.com/tectonic-typesetting/tectonic/pull/1111
[#1116]: https://github.com/tectonic-typesetting/tectonic/pull/1116
[#1119]: https://github.com/tectonic-typesetting/tectonic/pull/1119
[#1120]: https://github.com/tectonic-typesetting/tectonic/pull/1120
[#1127]: https://github.com/tectonic-typesetting/tectonic/pull/1127
[#1129]: https://github.com/tectonic-typesetting/tectonic/pull/1129
[#1132]: https://github.com/tectonic-typesetting/tectonic/pull/1132
[@bryango]: https://github.com/bryango
[@rm-dr]: https://github.com/rm-dr
[@pkgw]: https://github.com/pkgw
[@ColeFrench]: https://github.com/ColeFrench
[@goyalyashpal]: https://github.com/goyalyashpal
[@eljamm]: https://github.com/eljamm
[@felixonmars]: https://github.com/felixonmars


# tectonic 0.14.1 (2023-06-15)

This is a bugfix release:

- [@giammirove] wins the prize for discovering the first bug in the Rust
  translation of the BibTeX engine (issue [#1054]) — no small feat since our
  test suite includes nearly 8000 ArXiv submissions! Correcting a line of code
  that resizes an internal buffer fixes the problem ([#1055], [@CraftSpider]).
- The updated “watch” implementation failed if the path to the Tectonic
  executable contained whitespace (issue [#1003], reported by [@m-haug]).
  Proper quoting addresses the issue ([#1053], [@xinslu]).

[#1003]: https://github.com/tectonic-typesetting/tectonic/issues/1003
[#1053]: https://github.com/tectonic-typesetting/tectonic/pull/1053
[#1054]: https://github.com/tectonic-typesetting/tectonic/issues/1054
[#1055]: https://github.com/tectonic-typesetting/tectonic/pull/1055
[@giammirove]: https://github.com/giammirove
[@CraftSpider]: https://github.com/CraftSpider
[@m-haug]: https://github.com/m-haug
[@xinslu]: https://github.com/xinslu


# tectonic 0.14.0 (2023-06-12)

This release features a significant, but hopefully invisible, change: Tectonic’s
BibTeX engine has been partially translated from C to Rust ([#1032], [#1037],
[#1039], [@CraftSpider]). The intention is that the engine’s behavior should be
completely unchanged, and there are not plans to make any significant
alterations in the near future, but this effort demonstrates how Tectonic’s
legacy C/C++ code can be migrated to cleaner, safer, more maintainable Rust over
time.

There are also a few new features:

- Add a new `--target` option to `tectonic -X build` allowing you to
  specify which output to build ([#1043], [@xinslu]).
- Add a new unstable option, `-Z deterministic-mode`, that turns on some
  features that help create fully deterministic outputs ([#1036], [@Mrmaxmeier]).
  These settings aren’t desirable for day-to-day use, but they help create
  byte-for-byte reproducible outputs for automated testing. This option
  is part of further work by [@Mrmaxmeier] to bring the valuable
  [tectonic-on-arXiv] testing service back into regular operation.

As well as some developer improvements and bugfixes:

- Fix some corner-case bugs in the HTML output ([#1052], [@pkgw]).
- Update the vendored version of Harfbuzz to the latest upstream release,
  version 7.3.0 ([#1042], [@pkgw]).

Thanks to our new contributor [@xinslu] and everyone else contributing to this
release!

[#1032]: https://github.com/tectonic-typesetting/tectonic/pull/1032
[#1036]: https://github.com/tectonic-typesetting/tectonic/pull/1036
[#1037]: https://github.com/tectonic-typesetting/tectonic/pull/1037
[#1039]: https://github.com/tectonic-typesetting/tectonic/pull/1039
[#1042]: https://github.com/tectonic-typesetting/tectonic/pull/1042
[#1043]: https://github.com/tectonic-typesetting/tectonic/pull/1043
[#1052]: https://github.com/tectonic-typesetting/tectonic/pull/1052
[@CraftSpider]: https://github.com/CraftSpider
[@Mrmaxmeier]: https://github.com/Mrmaxmeier
[@pkgw]: https://github.com/pkgw
[@xinslu]: https://github.com/xinslu
[tectonic-on-arXiv]: https://github.com/Mrmaxmeier/tectonic-on-arXiv


# tectonic 0.13.1 (2023-05-22)

- Fix a bug introduced in the previous release where custom commands in the
  `tectonic -X watch` mode broke (reported by [@LudvigHz] in [#1040], fixed by
  [@CraftSpider] in [#1041]).
- Update the version of the vendored Harfbuzz library to the latest, 7.3.0
  ([@pkgw], [#1042]), and a general update of Cargo dependencies

Big thanks to [@LudvigHz] and [@CraftSpider] for the prompt report and solution!

[#1040]: https://github.com/tectonic-typesetting/tectonic/issues/1040
[#1041]: https://github.com/tectonic-typesetting/tectonic/pull/1041
[#1042]: https://github.com/tectonic-typesetting/tectonic/pull/1042
[@LudvigHz]: https://github.com/LudvigHz
[@CraftSpider]: https://github.com/CraftSpider
[@pkgw]: https://github.com/pkgw



# tectonic 0.13.0 (2023-05-18)

This release of Tectonic adds some quality-of-life features and improves the
project's technical infrastructure in several important ways.

- Add a new V2 command [`tectonic -X init`] that is to [`tectonic -X new`] in the
  same way that `cargo init` and `cargo new` are related ([#983], [@caiogeraldes]).
  It initializes a new document in the current directory, rather than creating a
  new directory.
- Setting the [`shell-escape-cwd`] unstable option now implies activation of the
  shell-escape feature (reported by [@mskblackbelt] in [#933], fixed by [@pkgw] in
  [#966]).

[`tectonic -X init`]: https://tectonic-typesetting.github.io/book/latest/v2cli/init.html
[`tectonic -X new`]: https://tectonic-typesetting.github.io/book/latest/v2cli/new.html
[`shell-escape-cwd`]: https://tectonic-typesetting.github.io/book/latest/v2cli/compile.html#unstable-options
[#983]: https://github.com/tectonic-typesetting/tectonic/pull/983
[#966]: https://github.com/tectonic-typesetting/tectonic/pull/966
[#933]: https://github.com/tectonic-typesetting/tectonic/issues/933
[@caiogeraldes]: https://github.com/caiogeraldes
[@pkgw]: https://github.com/pkgw
[@mskblackbelt]: https://github.com/mskblackbelt

On the infrastructure side:

- Update many developer dependencies to newer versions ([#1033], [#1038],
  [@CraftSpider]). Namely, `md-5` and `sha2` are now at 0.10, `open` is at 4.0,
  `quick-xml` is at 0.28, `toml` is at 0.7, `watchexec` is at 2.3, and `zip` is
  at 0.6. In the test suite, `hyper` is brought up to 0.14. Many thanks to
  [@CraftSpider] for taking on this arduous task!
- Builds on Windows using [`cargo-vcpkg`] to manage [vcpkg]-based dependencies
  now default to a custom target triplet named `x64-windows-static-release` that
  only builds release executables, significantly speeding up builds ([#961],
  [@pkgw]). You may need to set an environment variable `VCPKGRS_TRIPLET` to
  this value to satisfy the `cargo-vcpkg` build system.
- Internally, there were massive updates to the `spx2html` engine to improve
  Tectonic's still-experimental support for HTML output ([#1016], [@pkgw]). This
  support is still not exposed usefully in the main Tectonic program, however.
  The current work is in support of the prototype [Tectonopedia] project. Note
  that these changes break the current version of the [tt-weave] program, which
  will need updates to fix up its behavior.
- There were also many internal code tweaks to satisy the latest versions of
  [`cargo clippy`].

[`cargo-vcpkg`]: https://crates.io/crates/cargo-vcpkg
[vcpkg]: https://vcpkg.io/
[Tectonopedia]: https://github.com/tectonic-typesetting/tectonopedia
[`cargo clippy`]: https://github.com/rust-lang/rust-clippy
[tt-weave]: https://github.com/tectonic-typesetting/tt-weave
[#961]: https://github.com/tectonic-typesetting/tectonic/pull/961
[#1016]: https://github.com/tectonic-typesetting/tectonic/pull/1016
[#1033]: https://github.com/tectonic-typesetting/tectonic/pull/1033
[#1038]: https://github.com/tectonic-typesetting/tectonic/pull/1038
[@CraftSpider]: https://github.com/CraftSpider

Thank you to our new contributors [@caiogeraldes] and [@CraftSpider]! You will
also note that this release marks Tectonic’s 1000th pull-request-or-issue on
GitHub. Here’s looking to the next thousand!


# tectonic 0.12.0 (2022-10-27)

This release has only a few code updates:

- Partial support for the `dvipdfmx:config` special has been added (#953,
  @vlasakm). This should fix some aspects of PDF generation, including named
  anchors created by `hyperref`. Other fixes might help with the `attachfile`
  package, although that is awaiting further confirmation.
- A dumb crash was fixed when attempting to create HTML output with an input
  that has not been set up for the Tectonic HTML compilation framework (#955,
  @pkgw). Note, however, that generic documents will still fail to build in HTML
  mode. The program just won't crash now. As of this release, the *only* example
  of working HTML output from Tectonic is the [tt-weave] system (see below).

More noteworthy are several non-code improvements!

- A preliminary official build for the Apple Metal platform
  (`aarch64-apple-darwin`) is now available (#959, @pkgw). Due to lack of
  support in the continuous integration system we can't test the build
  thoroughly, but it appears to work.
- @KaranAhlawat contributed a [how-to guide for using Tectonic in Emacs AucTeX][auctex].
- @mnrvwl has done a fantastic job reviewing our GitHub issues, gathering more
  information when needed, and closing out ones that have been solved.
- @pkgw has published *[XeTeX: A Pseudoprogram][xap]*, a digital book that
  derives from Knuth's *[TeX: The Program][ttp]*. This book is generated from
  the reference XeTeX code underlaying Tectonic’s typesetting using a new
  processor called [tt-weave]. See [the book’s preface][xap] for more
  information.

Thank you to all of our contributors!

[auctex]: https://tectonic-typesetting.github.io/book/latest/howto/auctex-setup/
[tt-weave]: https://github.com/tectonic-typesetting/tt-weave/
[xap]: https://stacks.fullyjustified.net/xap/2022.0/
[ttp]: https://www.worldcat.org/title/876762639


# tectonic 0.11.0 (2022-10-04)

- Many updates to the experimental, unstable `spx2html` engine for creating HTML
  output (#941, @pkgw). They will not be documented here because there are a lot
  and the aforementioned experimental-ness and instability. This work is in
  service of the [tt-weave] demo, which is almost ready for a preliminary
  release.
- Add a tweak to the Harfbuzz build script that should hopefully fix builds on
  macOS against old SDKs, as seen in conda-forge (#944, @pkgw).

[tt-weave]: https://github.com/pkgw/tt-weave/


# tectonic 0.10.0 (2022-10-03)

This release updates Tectonic to support TeXLive 2022.0! There are not many code
changes in the engines, so the primary user-visible changes will stem from the
many package updates incorporated into the new TeXLive 2022.0 bundle. To switch
a preexisting Tectonic document to use the new bundle, update the `doc.bundle`
field in `Tectonic.toml` to
`https://data1.fullyjustified.net/tlextras-2022.0r0.tar`. Newly-created
documents will use this bundle (or subsequent updates) by default.

This release also adds a new “drop-in” installation method. This adds a way to
quickly install Tectonic in the popular `curl`/`sh` style. On a Unix-like
operating system, run:

```sh
curl --proto '=https' --tlsv1.2 -fsSL https://drop-sh.fullyjustified.net |sh
```

... to drop a system-appropriate `tectonic` binary in the current working directory.
On Windows, run the following in a PowerShell terminal:

```ps1
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://drop-ps1.fullyjustified.net'))
```

Other changes:

- Make it so that running `tectonic -Zhelp` works (#929, @pkgw). Before it would
  error out because the argument parser wanted an input filename.
- Fix `-Z continue-on-errors` (#917, @vlasakm). This was broken in an earlier
  refactoring.
- Add a `-Z shell-escape-cwd=<dir>` unstable option (#909, @0x00002a). This can
  work around issues in Tectonic's handing of shell-escape processing, which is
  very conservative about defaulting to launching programs in a limited
  environment. In particular, if you set the directory to the document source
  directory, commands like `\inputminted` can work.
- It is possible for one `.tex` file to generate multiple `.aux` files. Even if
  more than one of those files should have triggered its own `bibtex` run,
  Tectonic only ran `bibtex` once. This is now fixed (#906, #907, @Starrah).
- Give some more context in the error message if an external (shell-escape) tool
  tries to open a file that's missing (#899, @matz-e).

The known issue relating to OpenSSL 3 is believed to still be relevant:

- The generic prebuilt Tectonic binaries for Linux are built for the version 1.1
  series of OpenSSL. The latest Ubuntu release, 22.04 (Jammy Jellyfish), now
  targets OpenSSL 3, with no compatibility fallback, which means that the
  prebuilt binaries won’t run. To run Tectonic on these systems, compile it
  yourself, use the “semistatic” MUSL Linux builds, or install a package built
  specifically for this OS. To be clear, there are no actual issues with OpenSSL
  3 compatibility — we just need to provide an alternative set of builds. See
  #892 for updates.

Thank you to everyone who contributed to this release!


# tectonic 0.9.0 (2022-04-27)

This release updates Tectonic to correspond with TeXLive 2021.3, jumping
forward from the previous sync point of TeXLive 2020.0.

- The primary user-visible changes will stem from the many package updates
  incorporated into the new TeXLive 2021.3 bundle. We don't actually have a
  listing of all of the updates, but they are numerous. To switch a preexisting
  Tectonic document to use the new bundle, update the `doc.bundle` field in
  `Tectonic.toml` to `https://data1.fullyjustified.net/tlextras-2021.3r1.tar`.
  Newly-created documents will use this bundle (or subsequent updates) by
  default.
- The XeTeX engine has mostly low-level updates, but there was a significant
  rework of OpenType math kerning and sub/super-scripting. There is a new
  `\tracingstacklevels` integer parameter. See [the changelog for the
  `tectonic_engine_xetex` Rust crate][excl] for more details.
- The xdvipdfmx engine has numerous updates including improvements for Japanese
  font fallbacks. See [the changelog for the `tectonic_engine_xdvipdfmx` Rust
  crate][edcl] for more details.

[excl]: https://github.com/tectonic-typesetting/tectonic/releases/tag/tectonic_engine_xetex%400.3.0
[edcl]: https://github.com/tectonic-typesetting/tectonic/releases/tag/tectonic_engine_xdvipdfmx%400.2.0

Separately, the “GitHub Discussions” feature for the Tectonic repository has
been activated:

### <https://github.com/tectonic-typesetting/tectonic/discussions>

@pkgw has found himself unable to pay proper attention to the
`tectonic.newton.cx` Discourse service, which has been fairly moribund. The
intention is to sunset it.

We have one known issue worth highlighting:

- The generic prebuilt Tectonic binaries for Linux are built for the version 1.1
  series of OpenSSL. The latest Ubuntu release, 22.04 (Jammy Jellyfish), now
  targets OpenSSL 3, with no compatibility fallback, which means that the
  prebuilt binaries won’t run. To run Tectonic on these systems, compile it
  yourself, use the “semistatic” MUSL Linux builds, or install a package built
  specifically for this OS. To be clear, there are no actual issues with OpenSSL
  3 compatibility — we just need to provide an alternative set of builds. See
  #892 for updates.

Other improvements include:

- Some TeX packages attempt to read input from external processed using a “pipe”
  syntax. This capability is not currently implemented in Tectonic. Such uses
  now trigger a warning (#859, #888, @pkgw).
- The location of the Tectonic cache is now customizable by setting the
  `TECTONIC_CACHE_DIR` environment variable (#884, @wischi-chr). People are
  encouraged to use the default whenever possible, but flexibility here can be
  useful in some circumstances.
- Document the V2 `-X` flag better in the documentation and CLI output (#877,
  @clbarnes).
- Some memory leaks during failed builds have been plugged as part of an ongoing
  (albeit slow) effort to get it so that Tectonic can be used with modern input
  fuzzing tools (@pkgw).
- Allow basic `\openin` of un-closed `\openout` files to succeed (#882, @pkgw).
  This should get `hyperxmp` working (#862).


# tectonic 0.8.2 (2022-03-02)

No code changes here. This release uses the newly-released version 0.1.4 of the
[pinot] font parsing crate, which includes what were previously
Tectonic-specific extensions (#870, @pkgw). The "patching" build feature that we
were using turned out to break `cargo install tectonic`. Thanks to [@dfrg] for
prompt follow-up!

[pinot]: https://crates.io/crates/pinot
[@dfrg]: https://github.com/dfrg


# tectonic 0.8.1 (2022-02-28)

- The most important change in this release is a fix for issue [#844], wherein
  due to an implementation oversight Tectonic could obliterate `biber` input
  files whose locations were given as absolute paths ([#868], @pkgw). This
  should now be solved.
- This release also includes improved (i.e., "not broken") handling of `biber`
  inputs in subdirectories ([#843], [#845], @KevoSoftworks)
- A long-standing issue where outputs could vary slightly from one platform to
  the next, depending on system-dependent floating-point rounding with PNG images,
  was fixed ([#847], @pkgw).

There are also two big under-the-hood changes that won't make a noticeable difference
for now, but lay the groundwork for future work:

- The internal parameters and definitions of the Tectonic/XeTeX engine are now
  introspectable thanks to a new crate, [`tectonic_xetex_format`][xf]. This
  crate is now used to emit the C/C++ headers used to compile the engine. It is
  also able to introspect the "format files" that store engine state, adding the
  capability to answer questions such as "What are the definitions of all of the
  control strings defined by this format?" This should enable some *really*
  interesting supporting tools in the future!
- *Very* preliminary support for native HTML output has been added ([#865],
  @pkgw). This support isn't yet generally useful since it's undocumented and
  requires a suite of support files that's still being developed, but
  prototyping indicates that the generated output has promise for very
  high-quality mathematical rendering. The new [`tectonic_engine_spx2html`][s2h]
  crate provides the main new implementation. Hopefully there will be more to
  report soon!

[#843]: https://github.com/tectonic-typesetting/tectonic/issues/843
[#844]: https://github.com/tectonic-typesetting/tectonic/issues/844
[#845]: https://github.com/tectonic-typesetting/tectonic/pull/845
[#847]: https://github.com/tectonic-typesetting/tectonic/pull/847
[#865]: https://github.com/tectonic-typesetting/tectonic/pull/865
[#868]: https://github.com/tectonic-typesetting/tectonic/pull/868
[s2h]: https://crates.io/crates/tectonic_engine_spx2html
[xf]: https://crates.io/crates/tectonic_xetex_format

This release also includes the usual updates to internal dependencies, build and
testing infrastructure, and so on.


# tectonic 0.8.0 (2021-10-11)

This release fixes a showstopping issue introduced by recent changes to the
`archive.org` PURL ([persistent URL]) service. All users are advised to upgrade
immediately, although it is possible to continue using older releases in some
limited circumstances.

[persistent URL]: https://purl.prod.archive.org/help

By default, Tectonic downloads (La)TeX resource files from the internet as
needed. Before this release, Tectonic would query a PURL in order to know where
to locate the most recent “bundle” of resource files. On Wednesday,
`archive.org` updated the implementation of its service in a way that interacted
catastrophically with the way that Tectonic processes URL redirections. The
result was that Tectonic became unable to download any of its resource files,
breaking essential functionality. Thanks to [@rikhuijzer] for providing early
reporting and diagnosis of the problem.

[@rikhuijzer]: https://github.com/rikhuijzer

This release fixes the redirection functionality ([#832], [@pkgw]), but more
importantly it switches from querying `archive.org` to using a new dedicated
webservice hosted on the domain `fullyjustified.net` ([#833], [@pkgw]). The
motivation for this switch is that besides this particular incident,
`archive.org` has had low-level reliability problems in the past, and more
important, it is blocked in China, preventing a potentially large userbase from
trying Tectonic.

[#832]: https://github.com/tectonic-typesetting/tectonic/pull/832
[#833]: https://github.com/tectonic-typesetting/tectonic/pull/833

The new URL that is queried is:

https://relay.fullyjustified.net/default_bundle.tar

The redirection is implemented with a simple nginx server defined in the new
[tectonic-relay-service] repo and hosted on Microsoft Azure cloud infrastructure
defined in Terraform configuration in the [tectonic-cloud-infra] repo. [@pkgw] owns
the domain name and Azure subscription.

[tectonic-relay-service]: https://github.com/tectonic-typesetting/tectonic-relay-service
[tectonic-cloud-infra]: https://github.com/tectonic-typesetting/tectonic-cloud-infra
[@pkgw]: https://github.com/pkgw

Along with the above change, this release contains the following improvements:

- Add the [`tectonic -X dump`] V2 CLI command, which runs a partial document
  build and outputs a requested intermediate file. This can help integrate
  external tools into a Tectonic-based document processing workflow (#810,
  @pkgw)
- Add support for custom support file search directories with the `-Z
  search-path=<path>` [unstable option][sp] (#814, @ralismark)
- Fix the `watch` V2 CLI command on Windows (#828, @Sinofine)
- Fix repeated updates in the `watch` V2 CLI command (#807, @jeffa5)
- Fix an incorrect error message when running V2 CLI commands outside of a
  workspace (#813, @ralismark)
- Add a more helpful warning if an input produces empty output (#817,
  @ralismark)
- Prevent an incorrect warning when reading some kinds of EXIF metadata (#822,
  @korrat)
- Reject `-Z shell-escape=false`, which would be parsed as *enabling*
  shell-escape (#823, @ratmice)

[`tectonic -X dump`]: https://tectonic-typesetting.github.io/book/latest/v2cli/dump.html
[sp]: https://tectonic-typesetting.github.io/book/latest/v2cli/compile.html#unstable-options


# tectonic 0.7.1 (2021-07-04)

- Improve launching of `biber` by parsing the `.run.xml` file to find out which
  resource files are needed. This should hopefully allow Tectonic to process
  many more documents that use `biblatex` ([#796], [#804], [@pkgw]).
- Avoid misplaced newlines in warning output ([#803], [@ralismark]).
- Fix the build on Rust 1.46, which will be helpful for the conda-forge package.
  We really ought to define and monitor a Minimum Supported Rust Version (MSRV)
  for Tectonic, but we haven't set that up just yet ([#802], [@pkgw]).

[#796]: https://github.com/tectonic-typesetting/tectonic/issues/796
[#802]: https://github.com/tectonic-typesetting/tectonic/pull/802
[#803]: https://github.com/tectonic-typesetting/tectonic/pull/803
[#804]: https://github.com/tectonic-typesetting/tectonic/pull/804
[@pkgw]: https://github.com/pkgw
[@ralismark]: https://github.com/ralismark


# tectonic 0.7.0 (2021-06-19)

This release of Tectonic, at long last, adds support for [biber] to enable full
use of [biblatex][biber]! Biber is a complex Perl program, so, unlike the other
Tectonic “engines,” we can’t practically embed it within the Tectonic program.
This means that document builds using biber will have lessened reproducibility
and portability, but it’s better to have that than to fail to build the document
at all.

[biber]: http://biblatex-biber.sourceforge.net/

Here's a sample document that should now get fully processed:

```tex
% adapted from https://tex.stackexchange.com/a/34136/135094:
\documentclass{article}
\usepackage[autostyle]{csquotes}
\usepackage[
    backend=biber,
    style=authoryear-icomp,
    sortlocale=de_DE,
    natbib=true,
    url=false,
    doi=true,
    eprint=false
]{biblatex}
\addbibresource{biblatex-examples.bib}

\usepackage[]{hyperref}
\hypersetup{
    colorlinks=true,
}

\begin{document}

Lorem ipsum dolor sit amet~\citep{kastenholz}. At vero eos et accusam et justo
duo dolores et ea rebum~\citet{sigfridsson}.

\printbibliography
\end{document}
```

Tectonic’s new support detects a need to run `biber` by checking for the
creation of a file whose name ends in `.bcf`, and executes the `biber` program
inside a temporary directory, slurping any files that it creates into Tectonic’s
virtualized I/O subsystem. We’ll probably need to add a few new “knobs” to allow
users to control how and when biber is run — please file an issue if you run
into any limitations!

Under the hood, the implementation includes the beginnings of a more generic
subsystem for including external tool programs in document builds. This may turn
out to be more generally useful going forward.


# tectonic 0.6.4 (2021-06-17)

- Yet another new release to try to fix the docs.rs build. I think this one may
  get it right.


# tectonic 0.6.3 (2021-06-17)

- Another attempt to fix the docs.rs build.
- Update Cargo dependencies while we're at it.


# tectonic 0.6.2 (2021-06-16)

- Attempt to fix the i686 Arch Linux package specification
- Attempt to fix the docs.rs build, hopefully. We might have to try a few
  different approaches here before we find one that works.


# tectonic 0.6.1 (2021-06-15)

- No code changes; the attempt to publish 0.6.0 to Crates.io failed spuriously,
  so we're retriggering the release automation.


# tectonic 0.6.0 (2021-06-15)

This release adds some helpful new utilities and internal cleanups, which
involve breaking API changes (see below).

- New V2 command `tectonic -X show user-cache-dir` to print out the
  location of the per-user cache directory. FAQ, answered! (@pkgw, #786)
- New V2 command `tectonic -X bundle search` to print out listings of files
  contained in the "bundle" of TeX support files. If run in a workspace
  containing a `Tectonic.toml` file, the workspace’s bundle is queried;
  otherwise, the default bundle is queried. (@pkgw, #786)
- New V2 command `tectonic -X bundle cat` to print out one of the support files,
  with the same general behavior as the `search` command. You could also use
  this to ensure that a particular file has been loaded into the local cache.
  (@pkgw, #786).
- Improved security model regarding the "shell-escape" feature, which has the
  potential to be abused by untrusted inputs. A new `--untrusted` argument to
  the V1 CLI and `tectonic -X build` disables the use of shell-escape, and any
  other known-insecure features, regardless of the presence of `-Z shell-escape`
  or any other options. Therefore, if you're writing a script that processes
  untrusted input, if you make sure to run `tectonic --untrusted ...` you can be
  confident that further command-line arguments can't undo your sandboxing.
  Furthermore, if the environment variable `$TECTONIC_UNTRUSTED_MODE` is set to
  a non-empty value, the effect is as if `--untrusted` had been provided.
  (@pkgw, #787)
- You know what ... get rid of the "beta" message in the V1 CLI.
- Fix SyncTeX output, we hope (e.g., #720, #744; @hulloanson, @pkgw, #762).
  Tectonic's SyncTeX files should now include correct, absolute filesystem paths
  when appropriate.
- Fix some broken low-level XeTeX built-ins, reported by @burrbull (@pkgw, #714,
  #783)

A few more more words on the security model: the terminology is a bit slippery
here since we of course never intend to deliver a product that has security
flaws. But features like shell-escape, while providing useful functionality, can
certainly be abused to do malicious things given a hostile input. The default UX
aims to be conservative about these features, but if a user wants to enable
them, we'll allow them -- in the same way that Rust/Cargo will compile and run
`build.rs` scripts that in principle could do just about anything on your
machine. Our main security distinction is therefore whether the input is trusted
by the user running Tectonic. The decision of whether to "trust" an input or not
is something that fundamentally has to be made at a level higher above Tectonic
itself. Therefore the goal of Tectonic in this area is to provide the user with
straightforward and effective tools to express that decision.

For developers, this release adds two new Cargo crates to the Tectonic
ecosystem: `tectonic_docmodel`, allowing manipulation of `Tectonic.toml` files
and their related data structures; and `tectonic_bundles`, allowing manipulation
of the Tectonic support file bundles. In both cases, third-party tools might
wish to use these formats without having to pull in all of the heavyweight
dependencies of the main `tectonic` crate. And in both cases, the separation has
led to many API improvements and cleanups that greatly improve the overall code
structure. These changes break the API of the `tectonic` crate by removing some
old modules and changing the particular traits and types used to implement these
systems. (@pkgw, #785, #786)


# tectonic 0.5.2 (2021-06-08)

- Update dependencies, including [`watchexec`]. We believe that this should fix
  the issues with the official Windows executables that have been reported
  ([#780], [#782], [@pkgw])

[`watchexec`]: https://github.com/watchexec/watchexec
[#780]: https://github.com/tectonic-typesetting/tectonic/issues/780
[#782]: https://github.com/tectonic-typesetting/tectonic/pull/782
[@pkgw]: https://github.com/pkgw


# tectonic 0.5.1 (2021-06-07)

**Note:** we have reports that the official 0.5.0 Windows executables don’t
work, or don’t always work ([#780]). This is under investigation but hasn’t been
addressed yet.

- No code changes to the main crate
- Update the Arch Linux specification files to comply better with guidelines
  ([#779], [@lmartinez-mirror])

[#779]: https://github.com/tectonic-typesetting/tectonic/pull/779
[@lmartinez-mirror]: https://github.com/lmartinez-mirror
[#780]: https://github.com/tectonic-typesetting/tectonic/issues/780


# tectonic 0.5.0 (2021-06-06)

This is an exciting release! After [literally years of requests][i38], Tectonic
now supports the TeX “shell escape” mechanism required by some packages like the
[minted] code highlighter ([#708]). This is chiefly thanks to [@ralismark] who
put in the work to deliver a solid implementation and track ongoing changes to
the Tectonic backend. Thank you, [@ralismark]!

[i38]: https://github.com/tectonic-typesetting/tectonic/issues/38
[minted]: https://ctan.org/pkg/minted
[#708]: https://github.com/tectonic-typesetting/tectonic/pull/708
[@ralismark]: https://github.com/ralismark

Shell-escape remains disabled by default because it is, frankly, a hack that
detracts from the reproducibility and portability of document builds. It also
has significant security implications — you should never process untrusted input
with shell-escape enabled. But in those cases where shell-escape is necessary,
you can activate it with an [unstable option] in the [“V1”] command-line
interface:

[unstable option]: https://tectonic-typesetting.github.io/book/latest/v2cli/compile.html#unstable-options
[“V1”]: https://tectonic-typesetting.github.io/book/latest/ref/v1cli.html

```
tectonic -Z shell-escape my-shell-escape-document.tex
```

In the [“V2”] model, you can activate shell-escape by adding the following line
to one or more `[output]` sections in your [`Tectonic.toml`] file:

[“V2”]: https://tectonic-typesetting.github.io/book/latest/ref/v2cli.html
[`Tectonic.toml`]: https://tectonic-typesetting.github.io/book/latest/ref/tectonic-toml.html

```toml
[output]
name = 'default'
type = 'pdf'
shell_escape = true  # <== add this
```

The other major change associated with this release is for developers. The
Tectonic implementation has now been split into a number of specialized [Rust
crates][crate], each focusing on a specific piece of the overall Tectonic
functionality. Besides helping clarify and organize the large amount of code
that goes into Tectonic, this will make it easier for developers to create
Tectonic-based tools that use part of the codebase without having to depend on
every piece of it.

[crate]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html

This change was made possible by adopting a new release automation tool called
[Cranko] that project lead [@pkgw] created last summer. Cranko is based on a
novel [“just-in-time versioning”][jitv] release workflow and extensive use of
Azure Pipelines continuous integration and deployment services — together these
make it feasible to manage versioning and releases of the 20 different crates
that now live within the Tectonic [monorepo]. This may not sound like the most
exciting kind of code to write, but Cranko has made it possible to almost
entirely automate the Tectonic release processes in a way that’s been nothing
short of transformative.

[Cranko]: https://pkgw.github.io/cranko/
[@pkgw]: https://github.com/pkgw
[jitv]: https://pkgw.github.io/cranko/book/latest/jit-versioning/index.html
[monorepo]: https://en.wikipedia.org/wiki/Monorepo

This change comes with a bit of a downside, in that there have been a lot of API
breaks in the `tectonic` crate, as numerous internal APIs have been improved and
rationalized. If you only use the [`tectonic::driver`] module, changes should be
minimal, but lots of support systems have changed. It is likely that there will
be additional breaks in subsequent releases as a few remaining subsystems are
split out. The good news is that the APIs in the new sub-crates should be much
better designed and better documented than many of their former incarnations in
the main crate.

[`tectonic::driver`]: https://docs.rs/tectonic/*/tectonic/driver/index.html

There’s the usual collection of smaller improvements as well:

- If a document referenced a filename that corresponded to a directory that
  lived on the filesystem, you could get a hard-to-interpret error. Now,
  directories are ignored when looking for files.
  ([#754], [#759], [@pkgw])
- A floating-point precision issue was fixed that broke the reproducibility of
  builds on 32-bit versus 64-bit systems
  ([#749], [#758], [@pkgw])
- Fix potential undefined behavior in the `tectonic_xdv` crate reported by
  [@sslab-gatech]
  ([#752], [#753], [@pkgw])
- Add the ability to customize the preamble, postamble, and index files in
  V2 documents ([#745], [#746], [@jeffa5])
- Add a V2 `tectonic -X watch` command to auto-rebuild documents when their
  source files get updated ([#719], [#734], [@jeffa5])
- Add an `--open` option to `tectonic -X build` to open the document(s)
  after the build finishes ([#109], [#733], [@jeffa5])
- The usual updates to dependencies, build fixes, and documentation tweaks

[#109]: https://github.com/tectonic-typesetting/tectonic/issues/109
[#719]: https://github.com/tectonic-typesetting/tectonic/issues/719
[#745]: https://github.com/tectonic-typesetting/tectonic/issues/745
[#749]: https://github.com/tectonic-typesetting/tectonic/issues/749
[#752]: https://github.com/tectonic-typesetting/tectonic/issues/752
[#754]: https://github.com/tectonic-typesetting/tectonic/issues/754
[#733]: https://github.com/tectonic-typesetting/tectonic/pull/733
[#734]: https://github.com/tectonic-typesetting/tectonic/pull/734
[#746]: https://github.com/tectonic-typesetting/tectonic/pull/746
[#753]: https://github.com/tectonic-typesetting/tectonic/pull/753
[#758]: https://github.com/tectonic-typesetting/tectonic/pull/758
[#759]: https://github.com/tectonic-typesetting/tectonic/pull/759
[@sslab-gatech]: https://github.com/sslab-gatech
[@jeffa5]: https://github.com/jeffa5


# tectonic 0.4.1 (2021-01-03)

- Add support for aarch64-apple-darwin when building with vcpkg
- Prototype release automation infrastructure to update the new
  [tectonic-bin](https://aur.archlinux.org/packages/tectonic-bin/) AUR package.


# tectonic 0.4.0 (2020-12-28)

- Introduce a prototype new “V2” command line interface, accessible by running
  Tectonic with an initial `-X` argument: `tectonic -X new`. This interface is
  oriented around a new document model defined by a `Tectonic.toml` definition
  file. Documentation is under development in [the
  book](https://tectonic-typesetting.github.io/book/latest/). Eventually, this
  new interface will become the default, after a migration period. It is
  currently fairly basic, but will be fleshed out in the 0.4.x release series.
- Handle USV 0xFFFF in `print()` (#678, #682, @burrbull, @pkgw)
- Update the Arch Linux `makedepends` definitions (#691, @snowinmars)
- Update various Cargo dependencies.


# tectonic 0.3.3 (2020-11-16)

- When testing whether the engine needs rerunning, compare the new file to the
  entire old file, not just the part that was read by the engine. Should fix
  unnecessary reruns in some less-common cases. (#679, #681, @pkgw)


# tectonic 0.3.2 (2020-11-14)

- Slightly alter how some filenames are looked up. Before, if the TeX code
  requested a file whose name contained an extension, e.g. `foo.bar`, if no such
  file was available in the bundle we gave up immediately. Now we also check for
  `foo.bar.tex` and friends. This fixes the `lipsum` package in TeXLive 2020.0
  (#669, #680, @pkgw), and quite possibly some other miscellaneous packages as
  well.


# tectonic 0.3.1 (2020-11-02)

- Fix compilation on Windows/MSVC (`sys/time.h` is not available)
- Don't print an empty `error:` line in the CLI (#665, #670)


# tectonic 0.3.0 (2020-11-01)

The 0.3 series updates the core Tectonic engines to align with the code in
[TeXLive 2020.0][tl2020.0]. The default “bundle” of support files will soon be
updated to match TeXLive 2020.0 as well. Standard usages should work if you use
an older version of Tectonic with the new bundle, and vice versa, but we
recommend that you update your installations to the 0.3 promptly if you can.

[tl2020.0]: https://www.tug.org/texlive/

This release introduces a straightforward but **breaking change** to the API of
the `tectonic` Rust crate, documented below.

For context, Tectonic’s core TeX implementation is forked from the [XeTeX]
engine. Accumulated changes to XeTeX are periodically reviewed and imported into
Tectonic, a process that must be done manually because Tectonic’s modernized
developer and user experiences demand a huge amount of customization. (The
scripts to support the first stage of this process may be found in the
[tectonic-staging] repository.) It has been a while since the last
synchronization, but this release incorporates the new changes introduced
between the last update and the release of TeXLive 2020.0.

[XeTeX]: https://tug.org/xetex/
[tectonic-staging]: https://github.com/tectonic-typesetting/tectonic-staging

The changes for TeXLive 2020.0 include:

- New low-level primitives including `\filemoddate`, `\filedump`,
  `\uniformvariate`, `\elapsedtime`, and a few others.
- Tweaks to how font design sizes are mapped to TeX values
- New magic numbers used in PDF last x/y position accounting,
  instead of `cur_[hv]_offset`.
- Don't `print_raw_char` in `show_context` with `trick_buf`
- Back up `cur_cs` in `scan_keyword` and `compare_strings`.
- Handle `XETEX_MATH_GIVEN` in `scan_something_internal`
- If encountering an unexpandable primitive in `scan_something_internal`,
  try to deal with it. Ditto for `scan_int`.
- Do something different with active characters in `str_toks_cat`
- Rework how file names are scanned.
- Defend against undefined eTeX registers in `main_control`
- Some `uc_hyph` tweak deep in the linebreaking algorithm

There are also numerous changes in Tectonic’s included `xdvipdfmx`.

The implementation of the `\filemoddate` API required a **breaking change** to
the API of the `tectonic` Rust crate:

- We needed to add more data to the data structures of the `MemoryIo` provider.
  Migration should be pretty easy: instead of `files` containing a bunch of
  `Vec<u8>`s, it now contains a bunch of `MemoryFileInfo` structs that contain a
  `Vec<u8>` field named `data`. So you just need to add some `.data` field
  accessors to existing code. This API clearly needs some polish to allow
  greater stability going forward.

Other changes:

- Issue a warning if `xdvipdfmx` needs to translate a VF font to PK format,
  which is unimplemented in Tectonic (it relies on `mktexpk`) and so causes
  failures on certain documents that work with XeTeX.
- The Windows [vcpkg]-based build is temporarily disabled, as vcpkg currently has
  [a debilitating issue][vcpkg-issue] relating to SSL on Windows.
- There is a new `-Z continue-on-errors` unstable option that tells the engine
  to emulate the classic TeX style of plunging on ahead even in the face of
  severe errors. This is a nice example of the possibilities unlocked by the new
  `-Z` infrastructure introduced in 0.2!

[vcpkg]: https://github.com/microsoft/vcpkg
[vcpkg-issue]: https://github.com/tectonic-typesetting/tectonic/issues/668


# tectonic 0.2.0 (2020-10-21)

The 0.2 series finally adds "unstable" `-Z` flags! These allow you to configure
engine options that are relatively low-level. The hope is to eventually set
these kinds of things in a `Tectonic.toml` file, so their use is mildly
discouraged, and long-term availability is not guaranteed. But there are plenty
of times when such flags can be helpful. The currently supported options are:

- `-Z min-crossrefs=<num>` controls the `-min-crossrefs` option of standalone `bibtex`
- `-Z paper-size=<spec>` lets you control the output paper size, rather than
  hardcoding it to be US Letter.

Enormous thanks to @ralismark for finally implementing this! (#657) Now that the
infrastructure is in place, suggestions for additional flags are more than
welcome.


# tectonic 0.1.17 (2020-10-13)

- Fix source-based installation by updating to cbindgen 0.15, after later
  releases in the 0.14 series were yanked (@qtfkwk, #656)
- Fix unreachable code in the CID/Type0 code (@burrbull, @pkgw, #639, #646)
- Update the `cargo vcpkg` build process to use a newer version of `vcpkg`, fixing
  Windows builds when msys2.org is unreliable (@pkgw).


# tectonic 0.1.16 (2020-10-02)

- Add a "plain" backend for reporting status, used when the program is not
  attached to a TTY. It will print out reports with colorization. (#636,
  @ralismark)
- Start adding infrastructure to automate the creation of bindings from the
  C/C++ code to the Rust code, using `cbindgen`. (#643, @ralismark)
- Update the code-coverage infrastructure to gather coverage information
  from invocations of the CLI executable inside the test suite (@pkgw)
- Fully automated deployment should really actually totally work this time.


# tectonic 0.1.15 (2020-09-10)

- Building on the work done in 0.1.13, we now capture and report diagnostics
  nearly everywhere! Great work contributed by @ralismark (#635).
- Yet more revisions to the automated deployment system. Maybe *this* will be
  the time that it all works (@pkgw, #637).


# tectonic 0.1.14 (2020-09-08)

- No code changes from 0.1.13. Just trying to iron out some of the automated
  deployment systems. Is this the time that the Arch Linux auto-deployment
  finally works??


# tectonic 0.1.13 (2020-09-07)

It has once more been a long time since the last release. But this new release
features a move to a new release automation framework, [Cranko], which is
intended to promote a more aggressive release policy going forward. Cranko is
the result of a *lot* of careful thinking and design — resulting in a scheme
called [just-in-time versioning][jitv] — and it should offer a tractable and
low-friction framework for making releases even when there are many crates in
one repository.

[Cranko]: https://github.com/pkgw/cranko
[jitv]: https://pkgw.github.io/cranko/book/latest/jit-versioning/

User-facing improvements:

- Select core TeX warnings — notably, over/underfull boxes — are now surfaced as
  Tectonic warnings, and not just reported in the detailed log files! The
  infrastructure is now available to capture many more such warnings as needed.
  (#625; @ralismark, @pkgw)
- Fix a few algorithmic mistakes introduced in manual editing of the C code.
  Great catches by @burrbull! (#617, #624)
- Improve log formatting with backticks around filenames and human-friendly file
  sizes (#539; @as-f)
- Fix segfaults (!) upon errors (#579, #606; @fmgoncalves)
- Default bibtex's `min_crossrefs` to 2, not 0 (#534; @jneem)
- Help debug "lost characters" with their detailed hex codes (#600; @pkgw)

Developer-facing improvements:

- CI system has been completely revamped to use [Cranko] and route entirely
  through Azure Pipelines. Maintainability should be massively improved (@pkgw)
- Releases should now include pre-built binaries for a variety of architectures
  (@pkgw).
- Switched to `app_dirs2`, since `app_dirs` is unmaintained (#620; @kornelski)
- Enable reproducible-ish builds through `cargo vcpkg` (#593; @mcgoo)
- Update to the 0.9 series of the `rust-crypto` packages (#596; @pkgw)
- Attempt to fix automated Arch Linux build (#587; @thomaseizinger)
- Fix a memory leak (#536; @elliott-wen)
- The usual large number of dependency updates with DependaBot.


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


# 0.1.6 (2017 Jul 9)

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
