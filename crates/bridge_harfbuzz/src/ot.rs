//! OpenType layout support for Harfbuzz

use crate::{sys, FaceRef, GTag, OtNameId, Tag};
use std::ptr;

/// Optical size data for a face
pub struct LayoutSizeParams {
    // TODO: Better doc comments
    /// Design size
    pub design_size: u32,
    /// Sub-family ID
    pub subfamily_id: u32,
    /// Sub-family Name ID
    pub subfamily_name_id: OtNameId,
    /// Start
    pub start: u32,
    /// End
    pub end: u32,
}

/// OpenType layout helper for [`FaceRef`] - helper for calling methods related to opentype layout.
#[derive(Copy, Clone)]
pub struct Layout<'a>(pub(crate) FaceRef<'a>);

impl<'a> Layout<'a> {
    /// Get the optical size feature data for this face.
    pub fn size_params(self) -> Option<LayoutSizeParams> {
        let mut design_size = 0;
        let mut subfamily_id = 0;
        let mut subfamily_name_id = 0;
        let mut start = 0;
        let mut end = 0;

        // SAFETY: Internal pointer guaranteed valid
        let res = unsafe {
            sys::hb_ot_layout_get_size_params(
                self.0.as_ptr(),
                &mut design_size,
                &mut subfamily_id,
                &mut subfamily_name_id,
                &mut start,
                &mut end,
            )
        };

        if res != 0 {
            #[allow(clippy::useless_conversion)]
            Some(LayoutSizeParams {
                design_size: design_size.into(),
                subfamily_id: subfamily_id.into(),
                subfamily_name_id,
                start: start.into(),
                end: end.into(),
            })
        } else {
            None
        }
    }

    /// Get information about an OpenType glyph table
    pub fn table(self, tag: GTag) -> Table<'a> {
        Table(self.0, tag)
    }
}

/// Information associated with an OpenType glyph table for a specific face
#[derive(Copy, Clone)]
pub struct Table<'a>(FaceRef<'a>, GTag);

impl<'a> Table<'a> {
    /// Number of script tags present on this table
    pub fn script_tags_len(self) -> usize {
        let tag = self.1.as_tag();

        // SAFETY: Internal pointer guaranteed valid
        let len = unsafe {
            sys::hb_ot_layout_table_get_script_tags(
                self.0.as_ptr(),
                tag.0,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        len as usize
    }

    /// Find a script by tag, if present in this table
    pub fn find_script(self, script: Tag) -> Option<Script<'a>> {
        let tag = self.1.as_tag();

        let mut pos = 0;
        // SAFETY: Internal pointer guaranteed valid
        let found = unsafe {
            sys::hb_ot_layout_table_find_script(
                self.0.as_ptr(),
                tag.to_raw(),
                script.to_raw(),
                &mut pos,
            )
        };
        if found != 0 {
            Some(Script {
                face: self.0,
                table: self.1,
                script: pos as usize,
            })
        } else {
            None
        }
    }

    /// Get the script information for a given index
    pub fn script(self, idx: usize) -> Option<Script<'a>> {
        if idx < self.script_tags_len() {
            Some(Script {
                face: self.0,
                table: self.1,
                script: idx,
            })
        } else {
            None
        }
    }
}

/// A script in a table for a specific face
#[derive(Copy, Clone)]
pub struct Script<'a> {
    face: FaceRef<'a>,
    table: GTag,
    script: usize,
}

impl<'a> Script<'a> {
    /// The tag associated with this script
    pub fn tag(self) -> Tag {
        let mut len = 1;
        let mut out = 0;
        // SAFETY: TODO
        unsafe {
            sys::hb_ot_layout_table_get_script_tags(
                self.face.as_ptr(),
                self.table.as_tag().to_raw(),
                self.script as libc::c_uint,
                &mut len,
                &mut out,
            )
        };
        assert_eq!(len, 1);
        Tag::new(out)
    }

    /// Number of language tags associated with this script
    pub fn language_tags_len(self) -> usize {
        let tag = self.table.as_tag();

        // SAFETY: Internal pointer guaranteed valid
        let len = unsafe {
            sys::hb_ot_layout_script_get_language_tags(
                self.face.as_ptr(),
                tag.0,
                self.script as libc::c_uint,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        len as usize
    }

    /// Attempt to find a language matching one of the provided tags under this script. If none
    /// are found, an `Err` is returned containing the default language.
    pub fn select_lang(self, langs: &[Tag]) -> Result<Language<'a>, Language<'a>> {
        let tag = self.table.as_tag();

        let mut out_idx = 0;
        // SAFETY: Internal pointer guaranteed valid. Provided buffer not written past `len`.
        let found = unsafe {
            sys::hb_ot_layout_script_select_language(
                self.face.as_ptr(),
                tag.0,
                self.script as libc::c_uint,
                langs.len() as libc::c_uint,
                langs.as_ptr().cast(),
                &mut out_idx,
            )
        };
        let lang = Language {
            face: self.face,
            table: self.table,
            script: self.script,
            lang: out_idx as usize,
        };
        if found != 0 {
            Ok(lang)
        } else {
            Err(lang)
        }
    }

    /// Get the language information for a given index
    pub fn lang(self, idx: usize) -> Option<Language<'a>> {
        if idx < self.language_tags_len() {
            Some(Language {
                face: self.face,
                table: self.table,
                script: self.script,
                lang: idx,
            })
        } else {
            None
        }
    }

    /// Attempt to get this script on the opposite [`GTag`] from the current one. If not present
    /// on that table, returns `None`.
    pub fn swap_table(self) -> Option<Script<'a>> {
        let table = match self.table {
            GTag::GPos => GTag::GSub,
            GTag::GSub => GTag::GPos,
        };
        Table(self.face, table).script(self.script)
    }
}

/// A language in a table for a specific script and face
#[derive(Copy, Clone)]
pub struct Language<'a> {
    face: FaceRef<'a>,
    table: GTag,
    script: usize,
    lang: usize,
}

impl Language<'_> {
    /// The tag associated with this language
    pub fn tag(self) -> Tag {
        let mut len = 1;
        let mut out = 0;
        // SAFETY: Face pointer is guaranteed valid. We provide length of one and one place to write
        //         to.
        unsafe {
            sys::hb_ot_layout_script_get_language_tags(
                self.face.as_ptr(),
                self.table.as_tag().to_raw(),
                self.script as libc::c_uint,
                self.lang as libc::c_uint,
                &mut len,
                &mut out,
            )
        };
        assert_eq!(len, 1);
        Tag::new(out)
    }

    /// Number of feature tags associated with this language
    pub fn feature_tags_len(self) -> usize {
        let tag = self.table.as_tag();

        // SAFETY: Internal pointer guaranteed valid
        let len = unsafe {
            sys::hb_ot_layout_language_get_feature_tags(
                self.face.as_ptr(),
                tag.0,
                self.script as libc::c_uint,
                self.lang as libc::c_uint,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        len as usize
    }

    /// Get the feature tag for a given index
    pub fn feature(self, idx: usize) -> Option<Tag> {
        let mut len = 1;
        let mut out = 0;
        // SAFETY: Face pointer is guaranteed valid. We provide length of one and one place to write
        //         to.
        unsafe {
            sys::hb_ot_layout_language_get_feature_tags(
                self.face.as_ptr(),
                self.table.as_tag().to_raw(),
                self.script as libc::c_uint,
                self.lang as libc::c_uint,
                idx as libc::c_uint,
                &mut len,
                &mut out,
            )
        };
        if len != 1 {
            None
        } else {
            Some(Tag::new(out))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::test_faces;

    #[test]
    fn test_ot_layout_tables() {
        for (_, face) in test_faces() {
            let layout = face.as_ref().ot_layout();
            for table in [layout.table(GTag::GSub), layout.table(GTag::GPos)] {
                assert_eq!(table.script_tags_len(), 3);
            }
        }
    }

    #[test]
    fn test_ot_layout_table_scripts() {
        for (_, face) in test_faces() {
            let layout = face.as_ref().ot_layout();
            for table in [layout.table(GTag::GSub), layout.table(GTag::GPos)] {
                let script = table.script(0).unwrap();
                assert_eq!(script.tag(), Tag::from_str("DFLT"));
                assert_eq!(script.language_tags_len(), 0);
                assert_eq!(
                    script.script,
                    table.find_script(script.tag()).unwrap().script
                );

                script.swap_table().unwrap();

                let script2 = table.script(1).unwrap();

                assert_eq!(script2.tag(), Tag::from_str("cyrl"));
                assert_eq!(script2.language_tags_len(), 0);
                assert_eq!(
                    script2.script,
                    table.find_script(script2.tag()).unwrap().script
                );

                script2.swap_table().unwrap();
            }
        }
    }
}
