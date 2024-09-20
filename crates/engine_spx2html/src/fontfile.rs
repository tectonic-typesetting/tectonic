// Copyright 2021-2022 the Tectonic Project
// Licensed under the MIT License.

//! Data pertaining to a specific (OpenType) font file.
//!
//! The most interesting functionality here is our "variant glyph"
//! infrastructure used to be able to show specific glyphs out of the font when
//! we don't know a Unicode character that will reliably produce it. Whenever
//! possible we try to get "ActualText" info out of the engine so that we don't
//! have to do this, but for math and potentially other situations this is
//! sometimes necessary.

use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use percent_encoding::{utf8_percent_encode, CONTROLS};
use pinot::{
    math::MathVariants,
    otl::{Feature, SubtableKind},
    types::{FWord, Tag, UfWord},
    FontDataRef, TableProvider,
};
use std::{collections::HashMap, num::Wrapping, path::Path};
use tectonic_errors::prelude::*;

use crate::FixedPoint;

/// A numerical identifier of a glyph in a font.
pub type GlyphId = u16;

/// A Unicode Scalar Value.
///
/// Valid USVs fall into two ranges: 0 to 0xD7FF, and 0xE000 to 0x10FFFF (both
/// inclusive). Values within this range can be converted to Rust "char" values.
pub type Usv = u32;

const SSTY: Tag = Tag(0x73_73_74_79);

/// A type for retrieving data about the glyphs used in a particular font.
#[derive(Debug)]
pub struct FontFileData {
    /// The complete font data.
    ///
    /// Currently, this must be an OpenType font.
    buffer: Vec<u8>,

    /// Information about how glyphs can be reverse-mapped to Unicode input
    gmap: HashMap<GlyphId, MapEntry>,

    /// The glyph for the basic space character, or zero (typically .notdef) if
    /// it can't be found.
    space_glyph: GlyphId,

    units_per_em: UfWord,

    hmetrics: Vec<HorizontalMetrics>,
    ascender: FWord,

    /// This value is typically negative.
    descender: FWord,

    /// The fractional position of the baseline within the character cell:
    /// `ascender / (ascender - descender)`, keeping in mind that `descender` is
    /// typically negative.
    baseline_factor: f32,

    /// Map from Unicode charactors to how many variant character map records
    /// have been allocated for them. We need this to know how "deep" into the
    /// list of variants we need to push if a new glyph<->char pair has to be
    /// handled.
    variant_map_counts: HashMap<char, usize>,

    /// Map from glyph ID to variant character map setting.
    variant_map_allocations: HashMap<GlyphId, GlyphVariantMapping>,

    /// When we've been initialized to match a precomputed set of assets,
    /// we're not allowed to allocate any new variant glyph mappings.
    no_new_variants: bool,

    /// The index of the CMAP table record in the font data structure. We need
    /// this for the variant cmap munging.
    fontdata_cmap_trec_idx: usize,

    /// The offset of the HEAD table within the font data. We need
    /// this for the variant cmap munging.
    fontdata_head_offset: u32,
}

/// Information about the reverse-mapping of a glyph to Unicode.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MapEntry {
    /// The glyph corresponds directly to the specified Unicode character.
    Direct(char),

    /// The glyph corresponds to the sub/super-script form of the specified
    /// Unicode character.
    ///
    /// In an OpenType/TrueType font, this glyph representation is obtained with
    /// the first glyph substitution obtained using the `ssty` feature. If the
    /// associated bool is false, the glyph was the first variant form, used
    /// for sub/super-scripts on regular equation terms. If it is true, it is a
    /// "double" sub/super-script, e.g. the "z" in `x^{y^z}`.
    SubSuperScript(char, bool),

    /// The glyph corresponds to an enlarged version of a math symbol.
    ///
    /// If true, the boolean field indicates a vertically growing variant.
    /// Otherwise, it is horizontal. The u16 is the variant number in the
    /// sequence of growing variants.
    MathGrowingVariant(char, bool, u16),
}

impl MapEntry {
    fn get_char(&self) -> char {
        match *self {
            MapEntry::Direct(c) => c,
            MapEntry::SubSuperScript(c, _) => c,
            MapEntry::MathGrowingVariant(c, _, _) => c,
        }
    }
}

/// Information about an "variant mapping" to be used for a glyph.
///
/// When parsing XDV output, we may encounter glyphs that do not directly map to
/// an originating Unicode character (e.g., it maps with a MapEntry like
/// MathGrowingVariant). We handle this by creating modified font files with
/// custom character maps that *do* map some Unicode character directly to the
/// glyph we want. This makes it so that we can treat these special glyphs as if
/// they were just standard characters in a different font, and it turns out
/// that manipulating the font file to do this isn't so hard.
///
/// We need to maintain a sequence of these variant maps because we may wish
/// to map several different glyphs to the same Unicode character in this
/// fashion.
///
/// This would be a great application for OpenType font collections, since they
/// can have multiple "fonts" that share glyph data. But it looks like browser
/// CSS support for those is currently poor.
///
/// We might also one day wish to extend this system to emit a subsetted version
/// of the original font.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GlyphVariantMapping {
    /// The USV that the glyph should be mapped to
    pub usv: char,

    /// Which variant-mapped font to use. These indices start at zero.
    pub variant_map_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GlyphMetrics {
    /// Advance width in TeX units
    pub advance: FixedPoint,

    /// Left side bearing in TeX units
    pub lsb: FixedPoint,

    /// Ascent in TeX units
    pub ascent: FixedPoint,

    /// Descent in TeX units. This value is typically negative.
    pub descent: FixedPoint,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct HorizontalMetrics {
    /// Advance width in font units
    advance: UfWord,

    /// Left side bearing in font units
    lsb: FWord,
}

impl FontFileData {
    /// Load glyph data from OpenType font data.
    ///
    /// We take ownership of the font data that we're given.
    pub fn from_opentype(buffer: Vec<u8>, face_index: u32) -> Result<Self> {
        let font_data = a_ok_or!(
            FontDataRef::new(&buffer);
            ["unable to parse buffer as OpenType font"]
        );

        let font = a_ok_or!(
            font_data.get(face_index);
            ["unable to load face #{} in the OpenType font", face_index]
        );

        let head = a_ok_or!(
            font.head();
            ["unable to parse OpenType font: missing/invalid HEAD table"]
        );

        let units_per_em = head.units_per_em();

        // Get the direct mappings. While we're at it, figure out the glyph for
        // the space character, so that we can know how wide spaces are, so that
        // we can guess when to insert spaces into our HTML content.

        let cmap = a_ok_or!(
            font.cmap();
            ["unable to parse OpenType font: missing/invalid CMAP table"]
        );

        let mut gmap = HashMap::new();
        let mut space_glyph = 0;

        for usv in valid_usvs() {
            let c = char::from_u32(usv).unwrap();

            let gidx = match cmap.map(usv) {
                Some(g) if g != 0 => g,
                _ => {
                    continue;
                }
            };

            if c == ' ' {
                space_glyph = gidx;
            }

            gmap.insert(gidx, MapEntry::Direct(c));
        }

        // Check for additional substitution-based mappings.

        let dglyphs: Vec<_> = gmap.keys().copied().collect();

        if let Some(gsub) = font.gsub() {
            for feat in gsub.features() {
                if feat.record.tag == SSTY {
                    load_ssty_mappings(&mut gmap, &feat, &dglyphs[..])?;
                }
            }
        }

        // Check for math extras.

        if let Some(math) = font.math() {
            if let Some(variants) = math.variants() {
                load_math_variants(&mut gmap, &variants, &dglyphs[..])?;
            }
        }

        // Get horizontal metrics data. Note that pinot doesn't currently
        // provide an "owned" version of FontRef, as far as I can tell, so life
        // is a lot easier if we just copy out the data instead of trying to
        // hold a reference to the FontRef in the created struct.

        let hhea = a_ok_or!(
            font.hhea();
            ["unable to parse OpenType font: missing/invalid HMTX table"]
        );

        let ascender = hhea.ascender();
        let descender = hhea.descender();

        // Recall that descender < 0 in the relevant convention:
        let baseline_factor = ascender as f32 / (ascender - descender) as f32;

        let hmtx = a_ok_or!(
            font.hmtx();
            ["unable to parse OpenType font: missing/invalid HMTX table"]
        );

        let mut hmetrics = Vec::new();

        for hm in hmtx.hmetrics() {
            hmetrics.push(HorizontalMetrics {
                advance: hm.advance_width,
                lsb: hm.lsb,
            });
        }

        let advance = hmetrics[hmetrics.len() - 1].advance;

        for lsb in hmtx.lsbs() {
            hmetrics.push(HorizontalMetrics { advance, lsb });
        }

        // Get some parameters that we'll if we end up emitting any font
        // variants with hacked character maps. We know that HEAD and CMAP are
        // there so we don't worry about these variables never getting
        // initialized. (Famous last words?)

        let mut fontdata_cmap_trec_idx = 0;
        let mut fontdata_head_offset = 0;

        for (idx, trec) in font.records().iter().enumerate() {
            if trec.tag == pinot::head::HEAD {
                fontdata_head_offset = trec.offset;
            } else if trec.tag == pinot::cmap::CMAP {
                fontdata_cmap_trec_idx = idx;
            }
        }

        // All done!

        Ok(FontFileData {
            buffer,
            gmap,
            space_glyph,
            units_per_em,
            hmetrics,
            ascender,
            descender,
            baseline_factor,
            variant_map_counts: HashMap::new(),
            variant_map_allocations: HashMap::new(),
            no_new_variants: false,
            fontdata_head_offset,
            fontdata_cmap_trec_idx,
        })
    }

    /// Attempt to retrieve a mapping entry for the given glyph.
    pub fn lookup_mapping(&self, glyph: GlyphId) -> Option<MapEntry> {
        self.gmap.get(&glyph).copied()
    }

    /// Get the position of the baseline within the standard glyph cell.
    ///
    /// This value gives the position of the baseline in the glyph cell as a
    /// fractional distance from the top. For instance, if the baseline factor
    /// is 0.9 and a glyph is 100 units high, the baseline is located 90 units
    /// from the cell top, or 10 units from the cell bottom.
    pub fn baseline_factor(&self) -> f32 {
        self.baseline_factor
    }

    /// Attempt to retrieve metrics information for the given glyph.
    pub fn lookup_metrics(&self, glyph: GlyphId, tex_size: FixedPoint) -> Option<GlyphMetrics> {
        // As of Rust 1.45, the float-to-int cast saturates, which I think
        // is what we want here as a least-bad fallback. We don't want to
        // have to deal with fallibility in this conversion.

        let fword_to_tex = |f: FWord| -> FixedPoint {
            (f as f64 * tex_size as f64 / self.units_per_em as f64) as FixedPoint
        };

        let ufword_to_tex = |f: UfWord| -> FixedPoint {
            (f as f64 * tex_size as f64 / self.units_per_em as f64) as FixedPoint
        };

        self.hmetrics.get(glyph as usize).map(|hm| GlyphMetrics {
            advance: ufword_to_tex(hm.advance),
            lsb: fword_to_tex(hm.lsb),
            ascent: fword_to_tex(self.ascender),
            descent: fword_to_tex(self.descender),
        })
    }

    /// Get the width of the space character as a TeX size.
    pub fn space_width(&self, tex_size: FixedPoint) -> Option<FixedPoint> {
        if self.space_glyph == 0 {
            None
        } else {
            self.hmetrics.get(self.space_glyph as usize).map(|hm| {
                (hm.advance as f64 * tex_size as f64 / self.units_per_em as f64) as FixedPoint
            })
        }
    }

    /// Request that a variant mapping be allocated for a glyph.
    ///
    /// The caller must suggest a Unicode character to use for the variant, but
    /// if a different variant has already been allocated, that suggestion may
    /// be ignored.
    ///
    /// This function may return None if a new variant would need to be
    /// allocated, but that has been prohibited.
    pub fn request_variant(
        &mut self,
        glyph: GlyphId,
        suggested: char,
    ) -> Option<GlyphVariantMapping> {
        let map_entry = self.variant_map_allocations.entry(glyph);

        if self.no_new_variants {
            if let std::collections::hash_map::Entry::Vacant(_) = map_entry {
                return None;
            }
        }

        let new_index = self
            .variant_map_counts
            .get(&suggested)
            .copied()
            .unwrap_or(0);

        let map = map_entry.or_insert(GlyphVariantMapping {
            usv: suggested,
            variant_map_index: new_index,
        });

        if map.usv == suggested && map.variant_map_index == new_index {
            // If this is the case, we just created the mapping,
            // and need to bump the associated character's index for
            // the next glyph that wants to map to it.
            self.variant_map_counts.insert(suggested, new_index + 1);
        }

        Some(*map)
    }

    /// Emit customized fonts to the filesystem and return information so that
    /// appropriate CSS can be generated. Consumes the object.
    ///
    /// `rel_path` is the path, relative to the output root, where the font
    /// file(s) shouldb emitted. Currently, this may not contain any directory
    /// components, due to the way that the "variant" font file paths are
    /// constructed. This wouldn't be too hard to change.
    ///
    /// `out_base` is the output directory, or None if we shouldn't be writing
    /// anything to disk.
    ///
    /// Return value is a vec of (variant-map-index, CSS-src-field).
    pub fn emit(
        self,
        out_base: Option<&Path>,
        rel_path: &str,
    ) -> Result<Vec<(Option<usize>, String)>> {
        // Write the main font file ... maybe.

        let mut out_path = out_base.map(|p| p.to_owned());

        if let Some(out_path) = out_path.as_mut() {
            out_path.push(rel_path);
            let display_path = out_path.clone();
            atry!(
                std::fs::write(out_path, &self.buffer);
                ["cannot write output file `{}`", display_path.display()]
            );
        }

        // CSS info for the main font.

        let rel_url = utf8_percent_encode(rel_path, CONTROLS).to_string();
        let mut rv = vec![(None, format!(r#"url("{rel_url}") format("opentype")"#))];

        // Variants until we're done

        let mut buffer = self.buffer;
        let orig_len = buffer.len();

        for cur_map_index in 0.. {
            let mut mappings = Vec::new();

            for (glyph, altmap) in &self.variant_map_allocations {
                if altmap.variant_map_index == cur_map_index {
                    mappings.push((altmap.usv, *glyph));
                }
            }

            if mappings.is_empty() {
                break;
            }

            // We have some variants to emit! If we're not actually writing
            // files, we might not have much work to actually do though.

            let varname = format!("vg{cur_map_index}{rel_path}");

            if let Some(out_path) = out_path.as_mut() {
                // Step 1: create new CMAP, appending to buffer.
                //
                // Might be nice to sort mappings as we construct it, rather than
                // after the fact?

                buffer.truncate(orig_len);
                mappings.sort_unstable();
                append_simple_cmap(&mut buffer, &mappings[..]);
                let cmap_size = buffer.len() - orig_len;

                // step 2: modify CMAP table record

                let cs = opentype_checksum(&buffer[orig_len..]);
                let ofs = 12 + self.fontdata_cmap_trec_idx * 16;
                BigEndian::write_u32(&mut buffer[ofs + 4..ofs + 8], cs); // checksum
                BigEndian::write_u32(&mut buffer[ofs + 8..ofs + 12], orig_len as u32); // offset
                BigEndian::write_u32(&mut buffer[ofs + 12..ofs + 16], cmap_size as u32); // length

                // step 3: update HEAD "checksum adjustment" field

                let cs = opentype_checksum(&buffer[..]);
                let chkadj = Wrapping(0xB1B0AFBA) - Wrapping(cs);
                let ofs = self.fontdata_head_offset as usize + 8;
                BigEndian::write_u32(&mut buffer[ofs..ofs + 4], chkadj.0);

                // step 4: write new file

                out_path.pop();
                out_path.push(&varname);
                let display_path = out_path.clone();
                atry!(
                    std::fs::write(out_path, &buffer);
                    ["cannot write output file `{}`", display_path.display()]
                );
            }

            // step 5: update CSS

            let rel_url = utf8_percent_encode(&varname, CONTROLS).to_string();
            rv.push((
                Some(cur_map_index),
                format!(r#"url("{rel_url}") format("opentype")"#),
            ));
        }

        // All done!

        Ok(rv)
    }

    /// Emit customized fonts to the filesystem and return information so that
    /// appropriate CSS can be generated. Consumes the object.
    ///
    /// Return value is a vec of (variant-map-index, CSS-src-field).
    pub fn into_vglyphs(mut self) -> HashMap<String, crate::assets::syntax::GlyphVariantMapping> {
        let mut vglyphs = HashMap::default();

        for (glyph, altmap) in self.variant_map_allocations.drain() {
            vglyphs.insert(glyph.to_string(), altmap.into());
        }

        vglyphs
    }

    /// Update this "runtime" information to match the precomputed asset
    /// information. At the moment the only thing we need to change is the table
    /// of variant glyphs.
    pub(crate) fn match_to_precomputed(&mut self, ffad: &crate::assets::syntax::FontFileAssetData) {
        self.variant_map_counts.clear();
        self.variant_map_allocations.clear();

        for (gid, mapping) in &ffad.vglyphs {
            let gid: GlyphId = gid.parse().unwrap();

            self.variant_map_allocations.insert(gid, (*mapping).into());

            let c = self.variant_map_counts.entry(mapping.usv).or_default();
            *c = std::cmp::max(mapping.index + 1, *c);
        }

        self.no_new_variants = true;
    }
}

fn load_ssty_mappings(
    map: &mut HashMap<GlyphId, MapEntry>,
    feat: &Feature,
    dglyphs: &[GlyphId],
) -> Result<()> {
    for look in feat.lookups() {
        for st in look.subtables() {
            for glyph in dglyphs {
                let c = map.get(glyph).unwrap().get_char();

                if let Some(cov) = st.covered(*glyph) {
                    // Implement more subtable kinds as needed ...
                    if let SubtableKind::AlternateSubst1(t) = st.kind() {
                        if let Some(sl) = t.get(cov) {
                            if let Some(g) = sl.get(0) {
                                map.insert(g, MapEntry::SubSuperScript(c, false));
                            }

                            if let Some(g) = sl.get(1) {
                                map.insert(g, MapEntry::SubSuperScript(c, true));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn load_math_variants(
    map: &mut HashMap<GlyphId, MapEntry>,
    variants: &MathVariants,
    dglyphs: &[GlyphId],
) -> Result<()> {
    let maybe_vcov = variants.vert_glyph_coverage();
    let maybe_hcov = variants.horiz_glyph_coverage();

    for glyph in dglyphs {
        let c = map.get(glyph).unwrap().get_char();

        if let Some(vvars) = maybe_vcov
            .and_then(|c| c.get(*glyph))
            .and_then(|i| variants.vert_glyph_construction(i))
            .and_then(|c| c.variants())
        {
            for (idx, vinfo) in vvars.iter().enumerate() {
                map.insert(
                    vinfo.variant_glyph,
                    MapEntry::MathGrowingVariant(c, true, idx as u16),
                );
            }
        }

        if let Some(hvars) = maybe_hcov
            .and_then(|c| c.get(*glyph))
            .and_then(|i| variants.horiz_glyph_construction(i))
            .and_then(|c| c.variants())
        {
            for (idx, vinfo) in hvars.iter().enumerate() {
                map.insert(
                    vinfo.variant_glyph,
                    MapEntry::MathGrowingVariant(c, false, idx as u16),
                );
            }
        }
    }

    Ok(())
}

fn valid_usvs() -> impl Iterator<Item = Usv> {
    (0..0xD800).chain(0xE000..0x11_0000)
}

fn opentype_checksum(data: &[u8]) -> u32 {
    let mut iter = data.chunks_exact(4);
    let cs: Wrapping<u32> = iter
        .by_ref()
        .map(|c| Wrapping(BigEndian::read_u32(c)))
        .sum();

    let rem = iter.remainder();
    let mut padded = [0u8; 4];
    padded[..rem.len()].copy_from_slice(rem);
    (cs + Wrapping(BigEndian::read_u32(&padded[..]))).0
}

/// Append a dumb OpenType CMAP table to a buffer.
///
/// The input *map* must be sorted by USV value.
fn append_simple_cmap(buf: &mut Vec<u8>, map: &[(char, GlyphId)]) {
    buf.write_u16::<BigEndian>(0).unwrap(); // version
    buf.write_u16::<BigEndian>(1).unwrap(); // numTables

    buf.write_u16::<BigEndian>(0).unwrap(); // EncodingRecord.platformId = Unicode
    buf.write_u16::<BigEndian>(4).unwrap(); // EncodingRecord.encodingId = Unicode all planes
    buf.write_u32::<BigEndian>(12).unwrap(); // EncodingRecord.subtableOffset

    buf.write_u16::<BigEndian>(12).unwrap(); // Format 12 subtable identifier
    buf.write_u16::<BigEndian>(0).unwrap(); // reserved

    let subtable_len = 16 + 12 * map.len() as u32;
    buf.write_u32::<BigEndian>(subtable_len).unwrap(); // subtable byte length
    buf.write_u32::<BigEndian>(0).unwrap(); // subtable language
    buf.write_u32::<BigEndian>(map.len() as u32).unwrap(); // subtable number of groups

    // We could actually try to be smart here, but based on the expected usage
    // of our glyph variant scheme, I think it is unlikely that we'd realize
    // any significant efficiencies.

    for (usv, gid) in map {
        buf.write_u32::<BigEndian>(*usv as u32).unwrap(); // start char
        buf.write_u32::<BigEndian>(*usv as u32).unwrap(); // end char
        buf.write_u32::<BigEndian>(*gid as u32).unwrap(); // glyph id
    }
}

impl From<crate::assets::syntax::GlyphVariantMapping> for GlyphVariantMapping {
    fn from(m: crate::assets::syntax::GlyphVariantMapping) -> Self {
        GlyphVariantMapping {
            usv: m.usv,
            variant_map_index: m.index,
        }
    }
}
