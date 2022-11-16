// Copyright 2022 the Tectonic Project
// Licensed under the MIT License.

//! Manage families of related fonts.

use percent_encoding::{utf8_percent_encode, CONTROLS};
use std::{collections::HashMap, fmt::Write, path::Path};
use tectonic_errors::prelude::*;
use tectonic_status_base::{tt_warning, StatusBackend};

use crate::{
    fontfile::{FontFileData, GlyphId, GlyphMetrics, MapEntry},
    FixedPoint, FontNum,
};

#[derive(Debug)]
pub struct FontEnsemble {
    fonts: HashMap<FontNum, FontInfo>,
    font_data_keys: HashMap<(String, u32), usize>,
    font_data: HashMap<usize, FontFileData>,
    /// Keyed by the "regular" font-num
    font_families: HashMap<FontNum, FontFamily>,
}

impl Default for FontEnsemble {
    fn default() -> Self {
        FontEnsemble {
            fonts: Default::default(),
            font_data_keys: Default::default(),
            font_data: Default::default(),
            font_families: Default::default(),
        }
    }
}

impl FontEnsemble {
    /// Test whether this ensemble contains a font identified by the given SPX
    /// font number.
    pub fn contains(&self, f: FontNum) -> bool {
        self.fonts.contains_key(&f)
    }

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
        let fd_key = (name, face_index);
        let next_id = self.font_data_keys.len();
        let fd_key = *self.font_data_keys.entry(fd_key).or_insert(next_id);

        let info = FontInfo {
            rel_url: utf8_percent_encode(&basename, CONTROLS).to_string(),
            family_name: format!("tdux{}", font_num),
            family_relation: FamilyRelativeFontId::Regular,
            fd_key,
            size,
            face_index,
            color_rgba,
            extend,
            slant,
            embolden,
        };

        if fd_key == next_id {
            let map = atry!(
                FontFileData::from_opentype(basename.clone(), contents, face_index);
                ["unable to load glyph data for font `{}`", basename]
            );
            self.font_data.insert(fd_key, map);
        }

        self.fonts.insert(font_num, info);
        Ok(())
    }

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

        if let Some(info) = self.fonts.get_mut(&regular) {
            info.family_name = family_name.clone();
            info.family_relation = FamilyRelativeFontId::Regular;
        }

        if let Some(info) = self.fonts.get_mut(&bold) {
            info.family_name = family_name.clone();
            info.family_relation = FamilyRelativeFontId::Bold;
        }

        if let Some(info) = self.fonts.get_mut(&italic) {
            info.family_name = family_name.clone();
            info.family_relation = FamilyRelativeFontId::Italic;
        }

        if let Some(info) = self.fonts.get_mut(&bold_italic) {
            info.family_name = family_name;
            info.family_relation = FamilyRelativeFontId::BoldItalic;
        }
    }

    pub fn get_font_size(&self, fnum: FontNum) -> FixedPoint {
        self.fonts.get(&fnum).unwrap().size
    }

    pub fn maybe_get_font_space_width(&self, font_num: Option<FontNum>) -> Option<FixedPoint> {
        font_num.and_then(|fnum| {
            if let Some(fi) = self.fonts.get(&fnum) {
                let fd = self.font_data.get(&fi.fd_key).unwrap();
                fd.space_width(fi.size)
            } else {
                None
            }
        })
    }

    pub fn get_glyph_metrics(
        &mut self,
        fnum: FontNum,
        glyph: GlyphId,
    ) -> Result<Option<GlyphMetrics>> {
        let fi = a_ok_or!(
            self.fonts.get(&fnum);
            ["undeclared font number {}", fnum]
        );
        let fd = self.font_data.get_mut(&fi.fd_key).unwrap();
        Ok(fd.lookup_metrics(glyph, fi.size))
    }

    pub fn process_glyph_for_canvas(
        &mut self,
        fnum: FontNum,
        glyph: GlyphId,
        status: &mut dyn StatusBackend,
    ) -> (FixedPoint, f32, Option<(char, String)>) {
        let fi = self.fonts.get(&fnum).unwrap();
        let fd = self.font_data.get_mut(&fi.fd_key).unwrap();

        let text_info = fd.lookup_mapping(glyph).map(|mc| {
            // Sometimes we need to render a glyph in one of our input fonts
            // that isn't directly associated with a specific Unicode
            // character. For instance, in math, we may need to draw a big
            // integral sign, but the Unicode integral character maps to a
            // small one. The way we handle this is by *creating new fonts*
            // with custom character map tables that *do* map Unicode
            // characters directly to the specific glyphs we want.

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

            (ch, fi.selection_style_text(alt_index))
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

        (fi.size, fd.baseline_factor(), text_info)
    }

    pub fn process_glyphs_as_text<'a>(
        &'a mut self,
        font_num: FontNum,
        glyphs: &'a [GlyphId],
        status: &'a mut dyn StatusBackend,
    ) -> Result<GlyphTextProcessingIterator<'a>> {
        let fi = a_ok_or!(
            self.fonts.get(&font_num);
            ["undeclared font {} in glyph run", font_num]
        );

        let fd = self.font_data.get_mut(&fi.fd_key).unwrap();

        Ok(GlyphTextProcessingIterator {
            fi,
            fd,
            glyphs,
            status,
            next: 0,
        })
    }

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

    pub fn write_styling_span_html<W: Write>(
        &self,
        fnum: FontNum,
        rems_per_tex: f32,
        mut dest: W,
    ) -> Result<()> {
        let fi = self.fonts.get(&fnum).unwrap();
        let rel_size = fi.size as f32 * rems_per_tex;

        write!(
            dest,
            "<span style=\"font-size: {}rem; {}\">",
            rel_size,
            fi.selection_style_text(None)
        )
        .map_err(|e| e.into())
    }

    /// Emit the font files and return CSS setting up the files.kx
    pub fn emit(&mut self, out_base: &Path) -> Result<String> {
        // The reason we're doing all this: we can now emit our customized font
        // files that provide access to glyphs that we can't get the browser to
        // display directly. First, emit the font files via the font data.

        let mut emitted_info = HashMap::new();

        for (fd_key, data) in self.font_data.drain() {
            let emi = data.emit(out_base)?;
            emitted_info.insert(fd_key, emi);
        }

        // Now we can generate the CSS.

        let mut faces = String::default();

        for fi in self.fonts.values() {
            let emi = emitted_info.get(&fi.fd_key).unwrap();

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
}

pub struct GlyphTextProcessingIterator<'a> {
    fi: &'a FontInfo,
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

        // Get info needed by automatic space insertion:

        let gm = self.fd.lookup_metrics(glyph, self.fi.size);

        let advance = match gm {
            Some(gm) => gm.advance,
            None => 0,
        };

        // Now see if we can actually render it to Unicode

        let text_info = match self.fd.lookup_mapping(glyph) {
            Some(mc) => {
                let (mut ch, need_alt) = match mc {
                    MapEntry::Direct(c) => (c, false),
                    MapEntry::SubSuperScript(c, _) => (c, true),
                    MapEntry::MathGrowingVariant(c, _, _) => (c, true),
                };

                let alt_index = if need_alt {
                    let map = self.fd.request_alternative(glyph, ch);
                    ch = map.usv;
                    Some(map.alternate_map_index)
                } else {
                    None
                };

                // For later: we could select the "default" font at an outer
                // level and only emit tags as needed in here.
                let font_sel = self.fi.selection_style_text(alt_index);

                Some((ch, font_sel))
            }

            None => None,
        };

        if text_info.is_none() {
            tt_warning!(
                self.status,
                "unable to reverse-map glyph {} in font `{}` (face {})",
                glyph,
                self.fi.rel_url,
                self.fi.face_index
            );
        }

        let idx = self.next;
        self.next += 1;
        Some((idx, text_info, advance))
    }
}

#[derive(Debug)]
pub enum FontFamilyAnalysis {
    AlreadyActive,
    NoMatch,
    Reachable(PathToNewFont, FamilyRelativeFontId),
}

#[allow(dead_code)]
#[derive(Debug)]
struct FontInfo {
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
    fd_key: usize,

    size: FixedPoint,
    face_index: u32,
    color_rgba: Option<u32>,
    extend: Option<u32>,
    slant: Option<u32>,
    embolden: Option<u32>,
}

impl FontInfo {
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
