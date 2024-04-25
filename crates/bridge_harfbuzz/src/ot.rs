use crate::{sys, FaceRef, GTag, LayoutSizeParams, Tag};
use std::ptr;

#[derive(Copy, Clone)]
pub struct Layout<'a>(pub(crate) FaceRef<'a>);

impl<'a> Layout<'a> {
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
            Some(LayoutSizeParams {
                design_size: design_size as u32,
                subfamily_id: subfamily_id as u32,
                subfamily_name_id,
                start: start as u32,
                end: end as u32,
            })
        } else {
            None
        }
    }

    pub fn table(self, tag: GTag) -> Table<'a> {
        Table(self.0, tag)
    }
}

#[derive(Copy, Clone)]
pub struct Table<'a>(FaceRef<'a>, GTag);

impl<'a> Table<'a> {
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

#[derive(Copy, Clone)]
pub struct Script<'a> {
    face: FaceRef<'a>,
    table: GTag,
    script: usize,
}

impl<'a> Script<'a> {
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

    pub fn swap_table(self) -> Option<Script<'a>> {
        let table = match self.table {
            GTag::GPos => GTag::GSub,
            GTag::GSub => GTag::GPos,
        };
        Table(self.face, table).script(self.script)
    }
}

#[derive(Copy, Clone)]
pub struct Language<'a> {
    face: FaceRef<'a>,
    table: GTag,
    script: usize,
    lang: usize,
}

impl<'a> Language<'a> {
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
