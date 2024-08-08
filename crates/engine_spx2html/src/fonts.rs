// Copyright 2022 the Tectonic Project
// Licensed under the MIT License.

//! Manage families of related fonts.
//!
//! Here a "font family" is interpreted in the HTML sense, meaning a set of
//! related fonts. In typography you might call this a typeface.

use std::{collections::HashMap, fmt::Write, io::Read, path::Path};
use tectonic_errors::prelude::*;
use tectonic_io_base::InputHandle;
use tectonic_status_base::{tt_warning, StatusBackend};

use crate::{
    assets::syntax,
    fontfile::{FontFileData, GlyphId, GlyphMetrics, MapEntry},
    Common, FixedPoint, TexFontNum,
};

/// An identifier for a "font file" (which may be one face in a collection).
type FontId = usize;

/// Information about an ensemble of font families.
///
/// A given document may declare multiple families of related fonts.
#[derive(Debug, Default)]
pub struct FontEnsemble {
    /// Information about fonts declared in the SPX file. There may be
    /// a number of "native" fonts with different size/color/etc info
    /// that all reference the same underlying font file.
    tex_fonts: HashMap<TexFontNum, TexFontInfo>,

    /// Information about the individual font files used in this build. Although
    /// we call these "font files", there may be multiple Fonts for one file on
    /// disk, if that file is a collection containing multiple faces. There may
    /// also be font files that do not have corresponding TeX fonts, if we are
    /// loading a merged asset specification.
    font_files: Vec<Font>,

    /// Information about font families. This is keyed by the font-id of the
    /// "regular" font.
    font_families: HashMap<FontId, FontFamily>,

    /// Mapping font source TeX-paths and face indices to font IDs. This tuple
    /// of info uniquely identifies a "font file", in our terminology.
    src_index_map: HashMap<(String, u32), FontId>,
}

impl FontEnsemble {
    /// Test whether this ensemble contains a font identified by the given SPX
    /// font number.
    pub fn contains(&self, f: TexFontNum) -> bool {
        self.tex_fonts.contains_key(&f)
    }

    #[inline(always)]
    fn lookup_tex(&self, fnum: TexFontNum) -> Result<&TexFontInfo> {
        Ok(a_ok_or!(
            self.tex_fonts.get(&fnum);
            ["undeclared font number {}", fnum]
        ))
    }

    /// Register a new "native" font with this data structure. Font-family
    /// relations aren't recorded here.
    ///
    /// At this point, the calling function has checked whether this particular
    /// font-num has already been registered. But there can be multiple
    /// font-nums that point at the same source path and face index, which means
    /// they will have the same backing "font file" in our terminology. In
    /// particular, different sizes of the same font get different font-nums.
    ///
    /// The styling options like *color_rgba* and *slant* are currently stored
    /// but unused.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn register_tex_font(
        &mut self,
        font_num: TexFontNum,
        size: FixedPoint,
        face_index: u32,
        color_rgba: Option<u32>,
        extend: Option<u32>,
        slant: Option<u32>,
        embolden: Option<u32>,
        texpath: String,
        ih: InputHandle,
        common: &mut Common,
    ) -> Result<()> {
        let fid = self.ensure_font_file(&texpath, face_index, ih, common)?;

        let info = TexFontInfo {
            fid,
            size,
            color_rgba,
            extend,
            slant,
            embolden,
        };

        self.tex_fonts.insert(font_num, info);
        Ok(())
    }

    /// Make sure that a font file is loaded. Font files are uniquely identified
    /// by their source TeX paths and face indices.
    fn ensure_font_file(
        &mut self,
        texpath: &str,
        face_index: u32,
        mut ih: InputHandle,
        common: &mut Common,
    ) -> Result<FontId> {
        // Figure out if we've already loaded the appropriate font file.

        let si_key = (texpath.to_string(), face_index);
        let next_id = self.font_files.len();
        let fid = *self.src_index_map.entry(si_key).or_insert(next_id);

        if fid == next_id {
            // No, we haven't. Load it up.

            let mut contents = Vec::new();
            atry!(
                ih.read_to_end(&mut contents);
                ["unable to read input font file `{}`", texpath]
            );

            let (name, digest_opt) = ih.into_name_digest();
            common
                .hooks
                .event_input_closed(name, digest_opt, common.status);

            let ffd = atry!(
                FontFileData::from_opentype(contents, face_index);
                ["unable to load glyph data for font `{}`", texpath]
            );

            // Figure out the output path that we'll use for this font. For now,
            // that's just the basename. TODO: make sure that we don't have
            // basename clashes! This will happen trivially if we ever actually
            // use font collections that contain more than one face.

            let out_rel_path = texpath.rsplit('/').next().unwrap();

            // That's all we need.

            self.font_files
                .push(Font::new(texpath, face_index, out_rel_path, ffd));
        }

        Ok(fid)
    }

    /// Load a font that is *not* defined in the input SPX tile.
    ///
    /// This should only be called for fonts that are definitely not loaded by
    /// TeX, because we don't do any checks to prevent creating duplicate fonts
    /// outputs.
    fn load_external_font(
        &mut self,
        texpath: impl Into<String>,
        face_index: u32,
        common: &mut Common,
    ) -> Result<FontId> {
        let texpath = texpath.into();

        // All we have to do is open up the file and pass off to the shared
        // implementation.

        let io = common.hooks.io();

        let ih = atry!(
            io.input_open_name(&texpath, common.status).must_exist();
            ["failed to find a font file `{}`", texpath]
        );

        self.ensure_font_file(&texpath, face_index, ih, common)
    }

    /// Register a font-family relation.
    ///
    /// For the time being, the full quartet of bold/italic variations must be
    /// defined in order to declare a family.
    pub fn register_family(
        &mut self,
        name: String,
        regular: TexFontNum,
        bold: TexFontNum,
        italic: TexFontNum,
        bold_italic: TexFontNum,
    ) -> Result<()> {
        let regular = self.lookup_tex(regular)?.fid;
        let bold = self.lookup_tex(bold)?.fid;
        let italic = self.lookup_tex(italic)?.fid;
        let bold_italic = self.lookup_tex(bold_italic)?.fid;

        // Update the info records for the relevant fonts to capture the
        // established relationship.

        self.font_files[regular].family_name.clone_from(&name);
        self.font_files[regular].family_relation = FamilyRelativeFontId::Regular;
        self.font_files[bold].family_name.clone_from(&name);
        self.font_files[bold].family_relation = FamilyRelativeFontId::Bold;
        self.font_files[italic].family_name.clone_from(&name);
        self.font_files[italic].family_relation = FamilyRelativeFontId::Italic;
        self.font_files[bold_italic].family_name.clone_from(&name);
        self.font_files[bold_italic].family_relation = FamilyRelativeFontId::BoldItalic;

        self.font_families.insert(
            regular,
            FontFamily {
                name,
                regular,
                bold,
                italic,
                bold_italic,
            },
        );

        Ok(())
    }

    /// Get the size at which the specified SPX font is defined.
    ///
    /// If the TeX font number is undefined, a default of 10.0 is returned.
    pub fn get_font_size(&self, fnum: TexFontNum) -> FixedPoint {
        self.tex_fonts
            .get(&fnum)
            .map(|tfi| tfi.size)
            .unwrap_or(655360)
    }

    /// Get the width of the space character in a SPX font.
    ///
    /// This width is not always known, depending on the font file structure.
    /// For convenience, this function's input font number is also optional.
    pub fn maybe_get_font_space_width(&self, font_num: Option<TexFontNum>) -> Option<FixedPoint> {
        font_num
            .and_then(|fnum| self.tex_fonts.get(&fnum))
            .and_then(|tfi| self.font_files[tfi.fid].details.space_width(tfi.size))
    }

    /// Get the metrics for a glyph in a font.
    ///
    /// The return value is only `Err` if the font number is undeclared. If the
    /// glyph's metrics are not defined in the font, `Ok(None)` is returned.
    pub fn get_glyph_metrics(
        &mut self,
        fnum: TexFontNum,
        glyph: GlyphId,
    ) -> Result<Option<GlyphMetrics>> {
        let tfi = self.lookup_tex(fnum)?;
        Ok(self.font_files[tfi.fid]
            .details
            .lookup_metrics(glyph, tfi.size))
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
        fnum: TexFontNum,
        glyph: GlyphId,
        status: &mut dyn StatusBackend,
    ) -> (Option<(char, String)>, FixedPoint, f32) {
        // Can't borrow `self` in the map() closure.
        let font_files = &mut self.font_files;

        self.tex_fonts
            .get(&fnum)
            .map(|tfi| {
                let text_info = get_text_info(&mut font_files[tfi.fid], glyph, status);
                let size = tfi.size;
                let baseline_factor = font_files[tfi.fid].details.baseline_factor();

                (text_info, size, baseline_factor)
            })
            .unwrap_or((None, 655360, 1.0))
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
        font_num: TexFontNum,
        glyphs: &'a [GlyphId],
        status: &'a mut dyn StatusBackend,
    ) -> Result<impl Iterator<Item = (usize, Option<(char, String)>, FixedPoint)> + 'a> {
        // Can't use lookup_tex() here since the borrow checker treats it as
        // borrowing all of `self`, not just the `tex_fonts` member.
        let fi = a_ok_or!(
            self.tex_fonts.get(&font_num);
            ["undeclared font number {}", font_num]
        );
        let font = &mut self.font_files[fi.fid];

        Ok(GlyphTextProcessingIterator {
            fi,
            font,
            glyphs,
            status,
            next: 0,
        })
    }

    /// Determine how an SPX font relates to a font family.
    ///
    /// The *fnum* argument is some font number. The *cur_ffid* argument is the
    /// identifier of a font family, which is defined as the TexFontNum of its
    /// "regular" font. The *cur_af* argument defines the currently active font
    /// within that family, as identified with a [`FamilyRelativeFontId`].
    pub fn analyze_font_for_family(
        &self,
        fnum: TexFontNum,
        cur_ffid: TexFontNum,
        cur_af: FamilyRelativeFontId,
    ) -> FontFamilyAnalysis {
        if let Ok(tf) = self.lookup_tex(fnum) {
            if let Ok(tc) = self.lookup_tex(cur_ffid) {
                if let Some(cur_fam) = self.font_families.get(&tc.fid) {
                    // Already set up for the right font? If so, great!
                    return if cur_fam.relative_id_to_font_num(cur_af) == tf.fid {
                        FontFamilyAnalysis::AlreadyActive
                    } else {
                        // No. Figure out what we need to do.
                        let desired_af = cur_fam.font_num_to_relative_id(tf.fid);
                        FontFamilyAnalysis::Reachable(
                            cur_fam.path_to_new_font(cur_af, desired_af),
                            desired_af,
                        )
                    };
                }
            }

            FontFamilyAnalysis::NoMatch(tf.fid)
        } else {
            FontFamilyAnalysis::Unrecognized
        }
    }

    /// Write HTML code for an open `<span>` element that activates a font.
    ///
    /// The font size is specified in CSS "rem" units, which need to be
    /// calculated with the *rems_per_tex* parameter.
    pub fn write_styling_span_html<W: Write>(
        &self,
        fnum: TexFontNum,
        rems_per_tex: f32,
        mut dest: W,
    ) -> Result<()> {
        let tfi = self.lookup_tex(fnum)?;
        let rel_size = tfi.size as f32 * rems_per_tex;

        write!(
            dest,
            "<span style=\"font-size: {}rem; {}\">",
            rel_size,
            self.font_files[tfi.fid].selection_style_text(None)
        )
        .map_err(|e| e.into())
    }

    /// Emit the font files and return CSS code setting up the files.
    ///
    /// This function clears this object's internal data structures, making it
    /// effectively unusable for subsequent operations.
    pub fn emit(&mut self, out_base: Option<&Path>) -> Result<String> {
        let mut faces = String::default();

        for font in self.font_files.drain(..) {
            font.emit(out_base, &mut faces)?;
        }

        Ok(faces)
    }

    pub(crate) fn into_serialize(mut self) -> (syntax::Assets, syntax::FontEnsembleAssetData) {
        let mut assets: syntax::Assets = Default::default();
        let mut css_data: syntax::FontEnsembleAssetData = Default::default();
        let mut fid_to_filename = Vec::new();

        for font in self.font_files.drain(..) {
            let vglyphs = font.details.into_vglyphs();

            let ffad = syntax::FontFileAssetData {
                source: font.src_tex_path,
                face_index: font.face_index,
                vglyphs,
            };

            let filename = ffad.source.clone();
            assets
                .0
                .insert(filename.clone(), syntax::AssetOrigin::FontFile(ffad));
            fid_to_filename.push(filename);
        }

        for ffi in self.font_families.values() {
            let mut faces = HashMap::new();

            faces.insert(
                syntax::FaceType::Regular,
                fid_to_filename[ffi.regular].clone(),
            );
            faces.insert(syntax::FaceType::Bold, fid_to_filename[ffi.bold].clone());
            faces.insert(
                syntax::FaceType::Italic,
                fid_to_filename[ffi.italic].clone(),
            );
            faces.insert(
                syntax::FaceType::BoldItalic,
                fid_to_filename[ffi.bold_italic].clone(),
            );
            css_data
                .0
                .insert(ffi.name.clone(), syntax::FontFamilyAssetData { faces });
        }

        (assets, css_data)
    }

    /// Check that the fonts defined at runtime match the serialized assets, and
    /// set up the runtime variant glyphs to align with the precomputed ones.
    pub(crate) fn match_to_precomputed(
        &mut self,
        precomputed: &syntax::Assets,
        common: &mut Common,
    ) -> Result<()> {
        let mut fid_to_filename = Vec::new();

        // For the existing font file data, we need to check that they're
        // present and the basenames match. We'll replace the runtime
        // variant-glyph mappings with the precomputed ones.

        for font in &mut self.font_files {
            match precomputed.0.get(&font.out_rel_path) {
                Some(syntax::AssetOrigin::FontFile(ff)) => {
                    ensure!(
                        ff.source == font.out_rel_path,
                        "precomputed font asset `{}` \
                        should have an origin of `{}`, but in this session it is `{}`",
                        font.out_rel_path,
                        font.out_rel_path,
                        ff.source
                    );

                    font.details.match_to_precomputed(ff);
                }

                Some(other) => bail!(
                    "precomputed asset `{}` should be a font file, but it is {}",
                    font.out_rel_path,
                    other
                ),

                None => bail!(
                    "precomputed assets for this session should contain a font file named `{}`",
                    font.out_rel_path
                ),
            }

            fid_to_filename.push(font.out_rel_path.clone());
        }

        // Now create new records for any fonts in the precomputed assets that
        // we're missing. By definition, we shouldn't need them during our main
        // processing, but we still might be responsible for creating the final
        // output files at the end.

        for origin in precomputed.0.values() {
            if let syntax::AssetOrigin::FontFile(ff) = origin {
                let fid = atry!(
                    self.load_external_font(&ff.source, ff.face_index, common);
                    ["failed to load face #{} of font `{}` from precomputed assets", ff.face_index, ff.source]
                );

                self.font_files[fid].details.match_to_precomputed(ff);
            }
        }

        // This is a bit awkward, but our system currently lets there be
        // multiple font-CSS outputs that could in principle declare different
        // font families. To check consistency, we want to scan all of those. We
        // ignore the possibility that different CSS files might define
        // different families with the same name.

        let mut precomputed_families = HashMap::new();

        for origin in precomputed.0.values() {
            if let syntax::AssetOrigin::FontCss(fe) = origin {
                for (fam_name, ff) in &fe.0 {
                    precomputed_families.insert(fam_name.to_owned(), ff);
                }
            }
        }

        // Now we can check the runtime families. A helper closure uses
        // fnum_to_filename to deal with the different faces to check.

        let check_face = |fam_name: &str,
                          fid: FontId,
                          ft: syntax::FaceType,
                          pff: &syntax::FontFamilyAssetData|
         -> Result<()> {
            let runtime_file = &fid_to_filename[fid];

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

        for ffi in self.font_families.values() {
            let fam_name = &ffi.name;

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
    font: &'a mut Font,
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

        let gm = self.font.details.lookup_metrics(glyph, self.fi.size);

        let advance = match gm {
            Some(gm) => gm.advance,
            None => 0,
        };

        // Get the textualization info:

        let text_info = get_text_info(self.font, glyph, self.status);

        // And that's it!

        let idx = self.next;
        self.next += 1;
        Some((idx, text_info, advance))
    }
}

/// Get information about how to render a desired glyph from a font.
fn get_text_info(
    font: &mut Font,
    glyph: GlyphId,
    status: &mut dyn StatusBackend,
) -> Option<(char, String)> {
    let text_info = font.details.lookup_mapping(glyph).map(|mc| {
        let (mut ch, need_alt) = match mc {
            MapEntry::Direct(c) => (c, false),
            MapEntry::SubSuperScript(c, _) => (c, true),
            MapEntry::MathGrowingVariant(c, _, _) => (c, true),
        };

        let var_index = if need_alt {
            if let Some(map) = font.details.request_variant(glyph, ch) {
                ch = map.usv;
                Some(map.variant_map_index)
            } else {
                tt_warning!(
                    status,
                    "prohibited from defining new variant glyph {} in font `{}` (face {})",
                    glyph,
                    font.out_rel_path,
                    font.face_index
                );
                None
            }
        } else {
            None
        };

        // For later: might help to allow some context about the active font so
        // that we can maybe use a simpler selection string here.
        let font_sel = font.selection_style_text(var_index);

        (ch, font_sel)
    });

    if text_info.is_none() {
        tt_warning!(
            status,
            "unable to reverse-map glyph {} in font `{}` (face {})",
            glyph,
            font.out_rel_path,
            font.face_index
        );
    }

    text_info
}

/// The return type for [`FontEnsemble::analyze_font_for_family`].
#[derive(Debug)]
pub enum FontFamilyAnalysis {
    /// The desired font is already active.
    AlreadyActive,

    /// The desired font isn't active, but it can be "reached" in the context of
    /// the current family by closing and/or opening tags like `<b>`.
    Reachable(PathToNewFont, FamilyRelativeFontId),

    /// The desired font can't be reached in the context of this family. We
    /// can't activate it in a semantically-clean way. The associated value is
    /// the mapped font-id of the input TeX font num.
    NoMatch(FontId),

    /// The desired TeX font-num is unrecognized. This should only happen if the
    /// SPX file is corrupt.
    Unrecognized,
}

/// Information about a "native font" declared in the SPX file that's specific
/// to the TeX font, not the "font file" data structure.
#[allow(dead_code)]
#[derive(Debug)]
struct TexFontInfo {
    /// The font file pointed to by this TeX font.
    fid: FontId,

    /// The size at which this font is rendered, in TeX units.
    size: FixedPoint,

    /// Unused TeX/SPX setting.
    color_rgba: Option<u32>,

    /// Unused TeX/SPX setting.
    extend: Option<u32>,

    /// Unused TeX/SPX setting.
    slant: Option<u32>,

    /// Unused TeX/SPX setting.
    embolden: Option<u32>,
}

#[derive(Debug)]
struct Font {
    /// The TeX path of the file from which this font was loaded. In conjunction
    /// with face_index, this uniquely identifies a Font.
    src_tex_path: String,

    /// The index number of the particular face in the font file that was loaded.
    face_index: u32,

    /// The path that this font file will be output to.
    out_rel_path: String,

    details: FontFileData,

    /// The name of the family that this font is associated with. This may be a
    /// user-given name, if they explicitly define a font family; but by default
    /// it is automatically generated, so that we have *some* reliable way to
    /// name the font in our output.
    family_name: String,

    /// This font's role in relation to its family. Here, the `Other` enum
    /// variant is illegal.
    family_relation: FamilyRelativeFontId,
}

impl Font {
    pub fn new(
        src_tex_path: impl Into<String>,
        face_index: u32,
        out_rel_path: impl Into<String>,
        details: FontFileData,
    ) -> Self {
        let src_tex_path = src_tex_path.into();
        let out_rel_path = out_rel_path.into();
        let family_name = out_rel_path.replace(|c: char| !c.is_alphanumeric(), "_");

        Font {
            src_tex_path,
            face_index,
            out_rel_path,
            details,
            family_name,
            family_relation: FamilyRelativeFontId::Regular,
        }
    }

    fn emit<W: Write>(self, out_base: Option<&Path>, mut dest: W) -> Result<()> {
        for (var_index, css_src) in self.details.emit(out_base, &self.out_rel_path)? {
            // This is almost identical to `selection_style_text`. A major
            // factor is that we're consuming `self`, with `self.details`
            // already consumed by the `emit()` call, so we can't borrow &self.
            // Also, here we have double quotes around the font-family
            // specifier, which we want to have in the CSS but shouldn't have
            // (maybe???) in the HTML `style` attribute.
            let var_text = var_index.map(|i| format!("vg{i}")).unwrap_or_default();

            let extra = match self.family_relation {
                FamilyRelativeFontId::Regular => "",
                FamilyRelativeFontId::Bold => "\n    font-weight: bold;",
                FamilyRelativeFontId::Italic => "\n    font-style: italic;",
                FamilyRelativeFontId::BoldItalic => {
                    "\n    font-weight: bold;\n    font-style: italic;"
                }
                FamilyRelativeFontId::Other(_) => unreachable!(),
            };

            writeln!(
                dest,
                r#"@font-face {{
    font-family: "{}{}";{}
    src: {};
}}"#,
                self.family_name, var_text, extra, css_src,
            )?;
        }

        Ok(())
    }

    /// Generate a snippet of CSS for an HTML `style` attribute that will select
    /// the appropriate font, given that we might need to select one of the
    /// "variants" generated to make unusual glyphs available.
    fn selection_style_text(&self, variant_map_index: Option<usize>) -> String {
        let var_text = variant_map_index
            .map(|i| format!("vg{i}"))
            .unwrap_or_default();

        let extra = match self.family_relation {
            FamilyRelativeFontId::Regular => "",
            FamilyRelativeFontId::Bold => "; font-weight: bold",
            FamilyRelativeFontId::Italic => "; font-style: italic",
            FamilyRelativeFontId::BoldItalic => "; font-weight: bold; font-style: italic",
            FamilyRelativeFontId::Other(_) => unreachable!(),
        };

        format!("font-family: {}{}{}", self.family_name, var_text, extra)
    }
}

/// The definition of a family of fonts.
#[derive(Clone, Debug, Eq, PartialEq)]
struct FontFamily {
    name: String,
    regular: FontId,
    bold: FontId,
    italic: FontId,
    bold_italic: FontId,
}

impl FontFamily {
    fn font_num_to_relative_id(&self, fnum: FontId) -> FamilyRelativeFontId {
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

    fn relative_id_to_font_num(&self, relid: FamilyRelativeFontId) -> FontId {
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
    Other(FontId),
}
