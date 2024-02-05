# tectonic_engine_xetex 0.4.4 (2024-02-05)

- Support aarch64-unknown-linux-musl as a cross-buildable architecture (#1089,
  @pkgw). This includes one of the gnarliest build hacks I've ever had to
  perpetrate, as documented in `xetex/xetex-engine-interface.c`.


# tectonic_engine_xetex 0.4.3 (2023-06-12)

- Address a C compiler warning (#1050, @pkgw).


# tectonic_engine_xetex 0.4.2 (2023-05-18)

- Remove the automatic insertion of paragraph tags in HTML mode (#1016, @pkgw).
  It turns out that in TeX's internals, the starts and ends of "paragraphs"
  occur much more frequently than is apparent in the document source. And
  TeXLive 2022 introduces new LaTeX-level hooks for paragraph starts and ends
  that align much better with linguistic paragraphs. (This is not a coincidence,
  since the LaTeX core team is being funded to add support for creating properly
  semantically tagged PDFs.) So, for HTML output going forward, we'll use those
  hooks, and then there's no need for paragraph tagging support to be built into
  the engine here.


# tectonic_engine_xetex 0.4.1 (2022-10-04)

- When emitting in HTML mode, express paragraphs with `<div class="tdux-p">`
  instead of `<p>` (#941, @pkgw). This might seem wrong, but matches TeX's
  semantics better to the HTML specification, which is quite explicit that the
  `<p>` element does not have any special semantic meaning, and in fact
  recommends grouping semantic paragraphs with `<div>`s. You can't nest an
  `<ol>` inside a `<p>`, for instance, which does not align with TeX's view of
  things.


# tectonic_engine_xetex 0.4.0 (2022-10-03)

- Synchronize with TeXLive 2022.0 (#936, @pkgw)! Not many changes:
  - Update the internal TECKit to 2.5.11, corresponding to
    Unicode 14.0.0.
  - Update the engine format version to 33, which removes unused
    MLTeX `char_sub` parameters and expands the primitives table
    because we've passed 500 of them.
  - Update the XeTeX revision code to `.999994`.
  - Remove some vestigial MLTeX code related to the above.
  - Fix cleanup of TECKit in a few places
  - Other upstream changes are not relevant to Tectonic.
- Remove C's `time_t` from internal FFI APIs to avoid portability issues. This
  should avoid issues with Linux Musl builds.


# tectonic_engine_xetex 0.3.0 (2022-04-26)

Update the XeTeX engine for TeXLive 2021 (#882, @pkgw).

- Present as XeTeX revision 0.999993
- Update the XeTeX format specification to the new version 32
- Import [\Ucharcat update from 2018][ucc] that I seem to have missed before
- Fixes for [TeX bugs][tex82] 430-440
  - 430: not relevant to Tectonic (interactive features)
  - 431: not relevant to Tectonic (interactive features)
  - 432: skipped (date/time in system variables; no discernable impact on Tectonic)
  - 433: "After nine parameters, delete both # and the token that follows" — breaking change!
  - 434: Don't accept an implicit left brace after # in macro head
  - 435: Keep garbage out of the buffer if a |\read| end unexpectedly
  - 436: Zero out nonexistent chars, to prevent rogue TFM files
  - 437: Don't classify fraction noads as inner noads
  - 438: Properly identify tabskip glue when tracing repeated templates
  - 439: not relevant to Tectonic
  - 440: Normalize newlinechar when printing the final stats
- Significant rework/improvement of OpenType math kerning and super/sub-scripting
- Honor `PRIM_SIZE` correctly now that we have to change it!
- Implement `\tracingstacklevels`
- Guard against expansion depth overflow
- When reporting "lost characters", provide hex/UCS codes
- TECkit updated to TL21: version 2.5.10, upgrading from 2.5.9
  - This updates Unicode character names and normalization data to 13.0.0

[ucc]: https://github.com/TeX-Live/xetex/commit/0b12b29abb4748a9a85cc3e195ad388eba0d674e
[tex82]: https://ctan.math.utah.edu/ctan/tex-archive/systems/knuth/dist/errata/tex82.bug

Also:

- Allow `\openin` of `\openout` files to succeed (addresses #862, @pkgw).


# tectonic_engine_xetex 0.2.0 (2022-02-28)

- Use the new `tectonic_xetex_format` crate as part of the build process (#851,
  #848, @pkgw). This crate defines all of the metadata about the XeTeX engine
  internals, with versioning, and generates the necessary header files and
  macros. It also contains code for decoding XeTeX/Tectonic format files, so
  that we'll be able to introspect engine data structures such as macro
  definitions.
- Plumb in some specials that will be used by the prototype HTML output
  mode (#865, @pkgw)
- Tidy up some of the auto-generated C code
- Fix an internal transcription error: `pre_display_direction`, not
  `pre_display_correction`
- Fix a long-standing test issue with PNG image dimensions occasionally leading
  to not-quite-reproducible output (#847, @pkgw)


# tectonic_engine_xetex 0.1.4 (2021-07-04)

- Avoid misplaced newlines in warning output ([#803], [@ralismark])
- Fix new warnings reported by Clippy 1.53.0

[#803]: https://github.com/tectonic-typesetting/tectonic/pull/803
[@ralismark]: https://github.com/ralismark


# tectonic_engine_xetex 0.1.3 (2021-06-17)

- Switch from running [cbindgen] at build time to having the developer run it
  manually. This really ought to fix the crate builds on docs.rs ([#788]), and
  should speed builds too.

[cbindgen]: https://github.com/eqrion/cbindgen
[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788


# tectonic_engine_xetex 0.1.2 (2021-06-17)

- Attempt to fix crate builds on docs.rs — see [#788]. This works around an
  issue in Tectonic’s usage of [cbindgen] by configuring Cargo to operate in
  offline mode when building on docs.rs, which builds crates with network access
  turned off.

[#788]: https://github.com/tectonic-typesetting/tectonic/issues/788
[cbindgen]: https://github.com/eqrion/cbindgen


# tectonic_engine_xetex 0.1.1 (2021-06-15)

- Fix SyncTeX output (@hulloanson, @pkgw, #720, #744). We needed to include
  absolute paths and properly deal with file renames, etc. The only way to
  really do this right is to have the I/O backend provide filesystem paths when
  it has them, so we've extended the lower-level crates to make this possible.
- Fix the implementation of some special XeTeX commands, reported by @burrbull
  (@pkgw, #714, #783). This requires a bump in the format file serial number. We
  believe that this fix includes a fix to an upstream XeTeX bug, which has been
  reported.


# tectonic_engine_xetex 0.1.0 (2021-06-03)

This crate introduces the XeTeX engine as a standalone crate, building on the
new "core bridge" functionality.

Compared to the implementation previously provided in the main `tectonic` crate,
it also adds shell-escape functionality and iterates the Rust API somewhat.
