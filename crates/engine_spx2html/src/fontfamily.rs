// Copyright 2022 the Tectonic Project
// Licensed under the MIT License.

//! Manage families of related fonts.
//!
//! Here a "font family" is interpreted in the HTML sense, meaning a set of
//! related fonts. In typography you might call this a typeface.

use percent_encoding::{utf8_percent_encode, CONTROLS};
use std::{collections::HashMap, fmt::Write, path::Path};
use tectonic_errors::prelude::*;
use tectonic_status_base::{tt_warning, StatusBackend};

use crate::{
    assets::syntax,
    fontfile::{FontFileData, GlyphId, GlyphMetrics, MapEntry},
    FixedPoint, FontNum,
};

/// Information about an ensemble of font families.
///
/// A given document may declare multiple families of related fonts.
#[derive(Debug, Default)]
pub struct FontEnsemble {
    /// Information about fonts declared in the SPX file. There may be
    /// a number of "native" fonts with different size/color/etc info
    /// that all reference the same underlying font file.
    tex_fonts: HashMap<FontNum, TexFontInfo>,

    /// Information about the individual font files referenced by the TeX fonts.
    /// These are keyed by "font file data keys" that are just intended to save
    /// a bit of memory by making it so that not every TeX font has to store the
    /// filename it's associated with
    font_file_data: HashMap<usize, FontFileData>,

    /// Mapping filenames and face indices to font file data keys.
    ffd_keys: HashMap<(String, u32), usize>,

    /// Information about font families. This is keyed by the font-num of the
    /// "regular" font.
    font_families: HashMap<FontNum, FontFamily>,
}

impl FontEnsemble {
    /// Test whether this ensemble contains a font identified by the given SPX
    /// font number.
    pub fn contains(&self, f: FontNum) -> bool {
        self.tex_fonts.contains_key(&f)
    }

    /// Register a new "native" font with this data structure. Font-family
    /// relations aren't recorded here.
    ///
    /// Options like the *color_rgba* and *slant* are currently ignored.
    #[allow(clippy::too_many_arguments)]
    pub fn register(
        &mut self,
        name: String,
        font_num: FontNum,
        size: FixedPoint,
        face_index: u32,
        color_rgba: Option<u32>,
        extend: Option<u32>,
        slant: Option<u32>,
        embolden: Option<u32>,
        basename: String,
        contents: Vec<u8>,
    ) -> Result<()> {
        let ffd_key = (name, face_index);
        let next_id = self.ffd_keys.len();
        let ffd_key = *self.ffd_keys.entry(ffd_key).or_insert(next_id);

        let info = TexFontInfo {
            rel_url: utf8_percent_encode(&basename, CONTROLS).to_string(),
            family_name: format!("tdux{}", font_num),
            family_relation: FamilyRelativeFontId::Regular,
            ffd_key,
            size,
            face_index,
            color_rgba,
            extend,
            slant,
            embolden,
        };

        if ffd_key == next_id {
            let map = atry!(
                FontFileData::from_opentype(basename.clone(), contents, face_index);
                ["unable to load glyph data for font `{}`", basename]
            );
            self.font_file_data.insert(ffd_key, map);
        }

        self.tex_fonts.insert(font_num, info);
        Ok(())
    }

    /// Register a font-family relation.
    ///
    /// For the time being, the full quartet of bold/italic variations must be
    /// defined in order to declare a family
    pub fn register_family(
        &mut self,
        family_name: String,
        regular: FontNum,
        bold: FontNum,
        italic: FontNum,
        bold_italic: FontNum,
    ) {
        self.font_families.insert(
            regular,
            FontFamily {
                regular,
                bold,
                italic,
                bold_italic,
            },
        );

        // Now update the info records for the relevant fonts to capture the
        // established relationship.

        if let Some(info) = self.tex_fonts.get_mut(&regular) {
            info.family_name = family_name.clone();
            info.family_relation = FamilyRelativeFontId::Regular;
        }

        if let Some(info) = self.tex_fonts.get_mut(&bold) {
            info.family_name = family_name.clone();
            info.family_relation = FamilyRelativeFontId::Bold;
        }

        if let Some(info) = self.tex_fonts.get_mut(&italic) {
            info.family_name = family_name.clone();
            info.family_relation = FamilyRelativeFontId::Italic;
        }

        if let Some(info) = self.tex_fonts.get_mut(&bold_italic) {
            info.family_name = family_name;
            info.family_relation = FamilyRelativeFontId::BoldItalic;
        }
    }

    /// Get the size at which the specified SPX font is defined.
    pub fn get_font_size(&self, fnum: FontNum) -> FixedPoint {
        self.tex_fonts.get(&fnum).unwrap().size
    }

    /// Get the width of the space character in a SPX font.
    ///
    /// This width is not always known, depending on the font file structure.
    /// For convenience, this function's input font number is also optional.
    pub fn maybe_get_font_space_width(&self, font_num: Option<FontNum>) -> Option<FixedPoint> {
        font_num.and_then(|fnum| {
            if let Some(fi) = self.tex_fonts.get(&fnum) {
                let fd = self.font_file_data.get(&fi.ffd_key).unwrap();
                fd.space_width(fi.size)
            } else {
                None
            }
        })
    }

    /// Get the metrics for a glyph in a font.
    ///
    /// The return value is only `Err` if the font number is undeclared. If the
    /// glyph's metrics are not defined in the font, `Ok(None)` is returned.
    pub fn get_glyph_metrics(
        &mut self,
        fnum: FontNum,
        glyph: GlyphId,
    ) -> Result<Option<GlyphMetrics>> {
        let fi = a_ok_or!(
            self.tex_fonts.get(&fnum);
            ["undeclared font number {}", fnum]
        );
        let fd = self.font_file_data.get_mut(&fi.ffd_key).unwrap();
        Ok(fd.lookup_metrics(glyph, fi.size))
    }

    /// Get information needed to render a glyph in a canvas context.
    ///
    /// The return value is a tuple `(text_info, size, baseline_factor)`. In
    /// turn, `text_info` is an optional tuple of `(ch, style)`, where `ch` is
    /// the Unicode character to yield the desired glyph and `style` is a bit of
    /// CSS to go into an HTML `style` attribute in order to select the font
    /// that will map `ch` to the correct glyph.
    ///
    /// If we're unable to figure out a way to render the desired glyph, a
    /// warning is logged to the status backend.
    pub fn process_glyph_for_canvas(
        &mut self,
        fnum: FontNum,
        glyph: GlyphId,
        status: &mut dyn StatusBackend,
    ) -> (Option<(char, String)>, FixedPoint, f32) {
        let fi = self.tex_fonts.get(&fnum).unwrap();
        let fd = self.font_file_data.get_mut(&fi.ffd_key).unwrap();
        let text_info = get_text_info(fi, fd, glyph, status);
        (text_info, fi.size, fd.baseline_factor())
    }

    /// Create an iterator for rendering glyphs as Unicode text.
    ///
    /// The iterator yields tuples of `(index, text_info, advance)`, where
    /// `index` is the index of the glyph in the passed-in array, `text_info` is
    /// an optional tuple of information about how to get the glyph to appear in
    /// HTML, and `advance` is the horizontal advance length associated with the
    /// glyph in question, according to the font's metrics. If not None,
    /// `text_info` is a tuple of `(ch, style)`, where `ch` is the Unicode
    /// character to yield the desired glyph and `style` is a bit of CSS to go
    /// into an HTML `style` attribute in order to select the font that will map
    /// `ch` to the correct glyph.
    ///
    /// If we're unable to figure out a way to render the desired glyph, a
    /// warning is logged to the status backend.
    pub fn process_glyphs_as_text<'a>(
        &'a mut self,
        font_num: FontNum,
        glyphs: &'a [GlyphId],
        status: &'a mut dyn StatusBackend,
    ) -> Result<impl Iterator<Item = (usize, Option<(char, String)>, FixedPoint)> + 'a> {
        let fi = a_ok_or!(
            self.tex_fonts.get(&font_num);
            ["undeclared font {} in glyph run", font_num]
        );

        let fd = self.font_file_data.get_mut(&fi.ffd_key).unwrap();

        Ok(GlyphTextProcessingIterator {
            fi,
            fd,
            glyphs,
            status,
            next: 0,
        })
    }

    /// Determine how an SPX font relates to a font family.
    ///
    /// The *fnum* argument is some font number. The *cur_ffid* argument is the
    /// identifier of a font family, which is defined as the fontnum of its
    /// "regular" font. The *cur_af* argument defines the currently active font
    /// within that family, as identified with a [`FamilyRelativeFontId`].
    pub fn analyze_font_for_family(
        &self,
        fnum: FontNum,
        cur_ffid: FontNum,
        cur_af: FamilyRelativeFontId,
    ) -> FontFamilyAnalysis {
        if let Some(cur_fam) = self.font_families.get(&cur_ffid) {
            // Already set up for the right font? If so, great!
            if cur_fam.relative_id_to_font_num(cur_af) == fnum {
                FontFamilyAnalysis::AlreadyActive
            } else {
                // No. Figure out what we need to do.
                let desired_af = cur_fam.font_num_to_relative_id(fnum);
                FontFamilyAnalysis::Reachable(
                    cur_fam.path_to_new_font(cur_af, desired_af),
                    desired_af,
                )
            }
        } else {
            FontFamilyAnalysis::NoMatch
        }
    }

    /// Write HTML code for an open `<span>` element that activates a font.
    ///
    /// The font size is specified in CSS "rem" units, which need to be
    /// calculated with the *rems_per_tex* parameter.
    pub fn write_styling_span_html<W: Write>(
        &self,
        fnum: FontNum,
        rems_per_tex: f32,
        mut dest: W,
    ) -> Result<()> {
        let fi = self.tex_fonts.get(&fnum).unwrap();
        let rel_size = fi.size as f32 * rems_per_tex;

        write!(
            dest,
            "<span style=\"font-size: {}rem; {}\">",
            rel_size,
            fi.selection_style_text(None)
        )
        .map_err(|e| e.into())
    }

    /// Emit the font files and return CSS code setting up the files.
    ///
    /// This function clears this object's internal data structures, making it
    /// effectively unusable for subsequent operations.
    pub fn emit(&mut self, out_base: &Path) -> Result<String> {
        // The reason we're doing all this: we can now emit our customized font
        // files that provide access to glyphs that we can't get the browser to
        // display directly. First, emit the font files via the font data.

        let mut emitted_info = HashMap::new();

        for (ffd_key, data) in self.font_file_data.drain() {
            let emi = data.emit(out_base)?;
            emitted_info.insert(ffd_key, emi);
        }

        // Now we can generate the CSS.

        let mut faces = String::default();

        for fi in self.tex_fonts.values() {
            let emi = emitted_info.get(&fi.ffd_key).unwrap();

            for (alt_index, css_src) in emi {
                let _ignored = writeln!(
                    faces,
                    r#"@font-face {{
    {}
    src: {};
}}"#,
                    fi.font_face_text(*alt_index),
                    css_src,
                );
            }
        }

        Ok(faces)
    }

    pub(crate) fn into_serialize(mut self) -> (syntax::Assets, syntax::FontEnsembleAssetData) {
        let mut assets: syntax::Assets = Default::default();
        let mut css_data: syntax::FontEnsembleAssetData = Default::default();
        let mut fnum_to_filename = HashMap::new();
        let mut ffd_to_filename = HashMap::new();

        for (ffd_key, data) in self.font_file_data.drain() {
            let ffad = data.into_serialize();
            let filename = ffad.source.clone();
            assets.insert(filename.clone(), syntax::AssetOrigin::FontFile(ffad));
            ffd_to_filename.insert(ffd_key, filename);
        }

        for (fnum, fi) in &self.tex_fonts {
            fnum_to_filename.insert(fnum, ffd_to_filename.get(&fi.ffd_key).cloned().unwrap());
        }

        for (reg_fnum, ffi) in &self.font_families {
            let fam_name = self.tex_fonts.get(&reg_fnum).unwrap().family_name.clone();
            let mut faces = HashMap::new();

            faces.insert(
                syntax::FaceType::Regular,
                fnum_to_filename.get(&ffi.regular).cloned().unwrap(),
            );
            faces.insert(
                syntax::FaceType::Bold,
                fnum_to_filename.get(&ffi.bold).cloned().unwrap(),
            );
            faces.insert(
                syntax::FaceType::Italic,
                fnum_to_filename.get(&ffi.italic).cloned().unwrap(),
            );
            faces.insert(
                syntax::FaceType::BoldItalic,
                fnum_to_filename.get(&ffi.bold_italic).cloned().unwrap(),
            );
            css_data.insert(fam_name, syntax::FontFamilyAssetData { faces });
        }

        (assets, css_data)
    }

    /// Check that the fonts defined here are a subset of those defined in a
    /// serialized set of assets, and set up the runtime variant glyphs to align
    /// with the precomputed ones.
    pub(crate) fn match_to_precomputed(&mut self, precomputed: &syntax::Assets) -> Result<()> {
        let mut fnum_to_filename = HashMap::new();
        let mut ffd_to_filename = HashMap::new();

        // For the font file data, we need to check that they're present and the
        // basenames match. We'll replace the runtime variant-glyph mappings
        // with the precomputed ones.

        for (ffd_key, data) in &mut self.font_file_data {
            match precomputed.get(data.basename()) {
                Some(syntax::AssetOrigin::FontFile(ff)) => {
                    ensure!(
                        ff.source == data.basename(),
                        "precomputed font asset `{}` \
                        should have an origin of `{}`, but in this session it is `{}`",
                        data.basename(),
                        data.basename(),
                        ff.source
                    );

                    data.match_to_precomputed(ff);
                }

                Some(other) => bail!(
                    "precomputed asset `{}` should be a font file, but it is {}",
                    data.basename(),
                    other
                ),

                None => bail!(
                    "precomputed assets for this session should contain a font file named `{}`",
                    data.basename()
                ),
            }

            ffd_to_filename.insert(ffd_key, data.basename());
        }

        // We currently don't have any consistency checking to do with the TeX
        // fonts, since if their font-file-datas are OK, that's all we need. But
        // for the family checking, we do need to map fnum to output basename.

        for (fnum, fi) in &self.tex_fonts {
            fnum_to_filename.insert(fnum, *ffd_to_filename.get(&fi.ffd_key).unwrap());
        }

        // This is a bit awkward, but our system currently lets there be
        // multiple font-CSS outputs that could in principle declare different
        // font families. To check consistency, we want to scan all of those. We
        // ignore the possibility that different CSS files might define
        // different families with the same name.

        let mut precomputed_families = HashMap::new();

        for origin in precomputed.values() {
            if let syntax::AssetOrigin::FontCss(fe) = origin {
                for (fam_name, ff) in fe {
                    precomputed_families.insert(fam_name.to_owned(), ff);
                }
            }
        }

        // Now we can check the runtime families. A helper closure uses
        // fnum_to_filename to deal with the different faces to check.

        let check_face = |fam_name: &str,
                          fnum: FontNum,
                          ft: syntax::FaceType,
                          pff: &syntax::FontFamilyAssetData|
         -> Result<()> {
            let runtime_file = fnum_to_filename.get(&fnum).unwrap();

            if let Some(pre_file) = pff.faces.get(&ft) {
                ensure!(
                    pre_file == runtime_file,
                    "font family {} face {:?} should \
                    point to file `{}`, but in this session it is `{}`",
                    fam_name,
                    ft,
                    pre_file,
                    runtime_file
                );
            } else {
                bail!(
                    "this session defines unexpected face {:?} for font family {}",
                    ft,
                    fam_name
                );
            }

            Ok(())
        };

        for (reg_fnum, ffi) in &self.font_families {
            let fam_name = &self.tex_fonts.get(&reg_fnum).unwrap().family_name;

            if let Some(pff) = precomputed_families.get(fam_name) {
                check_face(fam_name, ffi.regular, syntax::FaceType::Regular, pff)?;
                check_face(fam_name, ffi.bold, syntax::FaceType::Bold, pff)?;
                check_face(fam_name, ffi.italic, syntax::FaceType::Italic, pff)?;
                check_face(fam_name, ffi.bold_italic, syntax::FaceType::BoldItalic, pff)?;
            } else {
                bail!(
                    "precomputed assets for this session should define a font family named {}",
                    fam_name
                );
            }
        }

        // All OK!

        Ok(())
    }
}

/// A helper type for the [`FontEnsemble::process_glyphs_as_text`] method.
struct GlyphTextProcessingIterator<'a> {
    fi: &'a TexFontInfo,
    fd: &'a mut FontFileData,
    glyphs: &'a [GlyphId],
    status: &'a mut dyn StatusBackend,
    next: usize,
}

impl<'a> Iterator for GlyphTextProcessingIterator<'a> {
    type Item = (usize, Option<(char, String)>, FixedPoint);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.glyphs.len() {
            return None;
        }

        let glyph = self.glyphs[self.next];

        // Get the advance info:

        let gm = self.fd.lookup_metrics(glyph, self.fi.size);

        let advance = match gm {
            Some(gm) => gm.advance,
            None => 0,
        };

        // Get the textualization info:

        let text_info = get_text_info(self.fi, self.fd, glyph, self.status);

        // And that's it!

        let idx = self.next;
        self.next += 1;
        Some((idx, text_info, advance))
    }
}

/// Get information about how to render a desired glyph from a font.
fn get_text_info(
    fi: &TexFontInfo,
    fd: &mut FontFileData,
    glyph: GlyphId,
    status: &mut dyn StatusBackend,
) -> Option<(char, String)> {
    let text_info = fd.lookup_mapping(glyph).map(|mc| {
        let (mut ch, need_alt) = match mc {
            MapEntry::Direct(c) => (c, false),
            MapEntry::SubSuperScript(c, _) => (c, true),
            MapEntry::MathGrowingVariant(c, _, _) => (c, true),
        };

        let alt_index = if need_alt {
            let map = fd.request_alternative(glyph, ch);
            ch = map.usv;
            Some(map.alternate_map_index)
        } else {
            None
        };

        // For later: might help to allow some context about the active font so
        // that we can maybe use a simpler selection string here.
        let font_sel = fi.selection_style_text(alt_index);

        (ch, font_sel)
    });

    if text_info.is_none() {
        tt_warning!(
            status,
            "unable to reverse-map glyph {} in font `{}` (face {})",
            glyph,
            fi.rel_url,
            fi.face_index
        );
    }

    text_info
}

/// The return type for [`FontEnsemble::analyze_font_for_family`].
#[derive(Debug)]
pub enum FontFamilyAnalysis {
    AlreadyActive,
    NoMatch,
    Reachable(PathToNewFont, FamilyRelativeFontId),
}

/// Information about a "native font" declared in the SPX file.
#[allow(dead_code)]
#[derive(Debug)]
struct TexFontInfo {
    /// Relative URL to the font data file
    rel_url: String,

    /// CSS name of the font family with which this font is associated;
    /// autogenerated if not specified during initialization.
    family_name: String,

    /// This font's "relationship" to its family. Defaults to Regular to
    /// if it's not associated with a full-fledged family.
    family_relation: FamilyRelativeFontId,

    /// Integer key used to relate this TeX font to its FontFileData. Multiple
    /// fonts may use the same FontFileData, if they refer to the same backing
    /// file.
    ffd_key: usize,

    /// The size at which this font is rendered, in TeX units.
    size: FixedPoint,

    /// Which face in the font file is being used.
    face_index: u32,

    /// Unused TeX/SPX setting.
    color_rgba: Option<u32>,

    /// Unused TeX/SPX setting.
    extend: Option<u32>,

    /// Unused TeX/SPX setting.
    slant: Option<u32>,

    /// Unused TeX/SPX setting.
    embolden: Option<u32>,
}

impl TexFontInfo {
    /// Generate a snippet of CSS for an HTML `style` attribute that will select
    /// the appropriate font, given that we might need to select one of the
    /// "variants" generated to make unusual glyphs available.
    fn selection_style_text(&self, alternate_map_index: Option<usize>) -> String {
        let alt_text = alternate_map_index
            .map(|i| format!("vg{}", i))
            .unwrap_or_default();

        let extra = match self.family_relation {
            FamilyRelativeFontId::Regular => "",
            FamilyRelativeFontId::Bold => "; font-weight: bold",
            FamilyRelativeFontId::Italic => "; font-style: italic",
            FamilyRelativeFontId::BoldItalic => "; font-weight: bold; font-style: italic",
            FamilyRelativeFontId::Other(_) => unreachable!(),
        };

        format!("font-family: {}{}{}", self.family_name, alt_text, extra)
    }

    /// This can probably be merged with `selection_style_text`. The key
    /// difference is double quotes around the font-family specifier, which we
    /// want to have in the CSS but shouldn't have (maybe???) in the HTML
    /// `style` attribute.
    fn font_face_text(&self, alternate_map_index: Option<usize>) -> String {
        let alt_text = alternate_map_index
            .map(|i| format!("vg{}", i))
            .unwrap_or_default();

        let extra = match self.family_relation {
            FamilyRelativeFontId::Regular => "",
            FamilyRelativeFontId::Bold => "\n    font-weight: bold;",
            FamilyRelativeFontId::Italic => "\n    font-style: italic;",
            FamilyRelativeFontId::BoldItalic => "\n    font-weight: bold;\n    font-style: italic;",
            FamilyRelativeFontId::Other(_) => unreachable!(),
        };

        format!(
            r#"font-family: "{}{}";{}"#,
            self.family_name, alt_text, extra
        )
    }
}

/// TeX/SPX font numbers for a family of fonts.
#[derive(Clone, Debug, Eq, PartialEq)]
struct FontFamily {
    regular: FontNum,
    bold: FontNum,
    italic: FontNum,
    bold_italic: FontNum,
}

impl FontFamily {
    fn font_num_to_relative_id(&self, fnum: FontNum) -> FamilyRelativeFontId {
        if fnum == self.regular {
            FamilyRelativeFontId::Regular
        } else if fnum == self.bold {
            FamilyRelativeFontId::Bold
        } else if fnum == self.italic {
            FamilyRelativeFontId::Italic
        } else if fnum == self.bold_italic {
            FamilyRelativeFontId::BoldItalic
        } else {
            FamilyRelativeFontId::Other(fnum)
        }
    }

    fn relative_id_to_font_num(&self, relid: FamilyRelativeFontId) -> FontNum {
        match relid {
            FamilyRelativeFontId::Regular => self.regular,
            FamilyRelativeFontId::Bold => self.bold,
            FamilyRelativeFontId::Italic => self.italic,
            FamilyRelativeFontId::BoldItalic => self.bold_italic,
            FamilyRelativeFontId::Other(fnum) => fnum,
        }
    }

    /// Figure out how to get "to" a desired font based on the current one. This
    /// function should only be called if it has been established that the
    /// desired font is in fact different than the current font. However, there
    /// are some noop cases below so that we can make the compiler happy about
    /// covering all of our enum variants.
    fn path_to_new_font(
        &self,
        cur: FamilyRelativeFontId,
        desired: FamilyRelativeFontId,
    ) -> PathToNewFont {
        match desired {
            FamilyRelativeFontId::Other(_) => PathToNewFont {
                close_all: true,
                select_explicitly: true,
                ..Default::default()
            },

            FamilyRelativeFontId::Regular => PathToNewFont {
                close_all: true,
                ..Default::default()
            },

            FamilyRelativeFontId::Bold => match cur {
                FamilyRelativeFontId::Regular => PathToNewFont {
                    open_b: Some(desired),
                    ..Default::default()
                },

                FamilyRelativeFontId::Bold => Default::default(),

                FamilyRelativeFontId::Italic | FamilyRelativeFontId::Other(_) => PathToNewFont {
                    close_all: true,
                    open_b: Some(desired),
                    ..Default::default()
                },

                FamilyRelativeFontId::BoldItalic => PathToNewFont {
                    close_one_and_retry: true,
                    ..Default::default()
                },
            },

            FamilyRelativeFontId::Italic => match cur {
                FamilyRelativeFontId::Regular => PathToNewFont {
                    open_i: Some(desired),
                    ..Default::default()
                },

                FamilyRelativeFontId::Italic => Default::default(),

                FamilyRelativeFontId::Bold | FamilyRelativeFontId::Other(_) => PathToNewFont {
                    close_all: true,
                    open_i: Some(desired),
                    ..Default::default()
                },

                FamilyRelativeFontId::BoldItalic => PathToNewFont {
                    close_one_and_retry: true,
                    ..Default::default()
                },
            },

            FamilyRelativeFontId::BoldItalic => match cur {
                FamilyRelativeFontId::Regular => PathToNewFont {
                    open_i: Some(desired),
                    open_b: Some(FamilyRelativeFontId::Bold), // <= the whole reason these aren't bools
                    ..Default::default()
                },

                FamilyRelativeFontId::Italic => PathToNewFont {
                    open_b: Some(desired),
                    ..Default::default()
                },

                FamilyRelativeFontId::Bold => PathToNewFont {
                    open_i: Some(desired),
                    ..Default::default()
                },

                FamilyRelativeFontId::BoldItalic => Default::default(),

                FamilyRelativeFontId::Other(_) => PathToNewFont {
                    close_one_and_retry: true,
                    ..Default::default()
                },
            },
        }
    }
}

/// How to "get to" a desired font based on the current font family and recently
/// active tags.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PathToNewFont {
    /// Close all open automatically-generated font-selection tags.
    pub close_all: bool,

    /// Close one automatically-generated font-selection tag, and try again.
    pub close_one_and_retry: bool,

    /// Issue a `<span>` element to explicitly choose the font; this is
    /// our get-out-of-jail-free card.
    pub select_explicitly: bool,

    /// If Some, open a `<b>` tag. The value is the "family-relative" font that
    /// will be active after doing so. If both this and `open_i` are Some, this
    /// should be evaluated first.
    pub open_b: Option<FamilyRelativeFontId>,

    /// If Some, open an `<i>` tag. The value is the "family-relative" font that
    /// will be active after doing so. If both this and `open_b` are Some, the
    /// `<b>` tag should be evaluated first.
    pub open_i: Option<FamilyRelativeFontId>,
}

/// A font's role relative to some font family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FamilyRelativeFontId {
    /// This font is the regular font of the current family.
    Regular,

    /// This font is the bold font of the current family.
    Bold,

    /// This font is the italic font of the current family.
    Italic,

    /// This font is the bold-italic font of the current family.
    BoldItalic,

    /// This font is some other font with no known relation to the current
    /// family.
    Other(FontNum),
}
