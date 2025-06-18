use super::{
    base_get_op_size_rec_and_style_flags, FontInfo, FontManager, FontManagerBackend, FontMaps,
    NameCollection,
};
use crate::c_api::PlatformFontRef;
use enrede::encoding::{MacRoman, Utf16BE, Utf8};
use enrede::Str;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use tectonic_bridge_fontconfig as fc;
use tectonic_bridge_freetype2 as ft;

pub const FONT_FAMILY_NAME: libc::c_ushort = 1;
pub const FONT_STYLE_NAME: libc::c_ushort = 2;
pub const FONT_FULL_NAME: libc::c_ushort = 4;
pub const PREFERRED_FAMILY_NAME: libc::c_ushort = 16;
pub const PREFERRED_SUBFAMILY_NAME: libc::c_ushort = 17;

pub struct FcBackend {
    all_fonts: fc::FontSet,
    cached_all: bool,
}

impl FcBackend {
    pub fn new() -> FcBackend {
        if !fc::init() {
            panic!("fontconfig initialization failed");
        }
        ft::init();

        let pat = fc::Pattern::from_name(c":outline=true").unwrap();
        let os = fc::ObjectSet::new(&[
            fc::Property::Family,
            fc::Property::Style,
            fc::Property::File,
            fc::Property::Index,
            fc::Property::FullName,
            fc::Property::Weight,
            fc::Property::Width,
            fc::Property::Slant,
            fc::Property::FontFormat,
        ]);
        let all_fonts = fc::FontSet::new(pat.as_ref(), os.as_ref());

        FcBackend {
            all_fonts,
            cached_all: false,
        }
    }

    fn cache_family_members(&mut self, maps: &mut FontMaps, names: &[CString]) {
        if names.is_empty() {
            return;
        }

        let font_set = self.all_fonts.as_ref();
        'outer: for &pat in font_set.fonts() {
            let pat = pat.upgrade();
            if maps.platform_ref_to_font.contains_key(&pat) {
                continue;
            }

            let mut i = 0;
            while let Ok(str) = pat.as_ref().get::<fc::pat::Family>(i) {
                for name in names {
                    if **name == *str {
                        let names = self.read_names(pat.clone());
                        maps.add_to_maps(self, pat, &names);
                        continue 'outer;
                    }
                }

                i += 1;
            }
        }
    }
}

impl FontManagerBackend for FcBackend {
    fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> Cow<'a, CStr> {
        if let Ok(str) = font.as_ref().get::<fc::pat::File>(0) {
            Cow::Borrowed(str)
        } else {
            Cow::Borrowed(c"[unknown]")
        }
    }

    fn get_op_size_rec_and_style_flags(&self, font: &mut FontInfo) {
        base_get_op_size_rec_and_style_flags(font);

        if font.weight == 0 && font.width == 0 {
            let pat = &font.font_ref;
            if let Ok(weight) = pat.as_ref().get::<fc::pat::Weight>(0) {
                font.weight = weight as u16;
            }
            if let Ok(width) = pat.as_ref().get::<fc::pat::Width>(0) {
                font.width = width as u16;
            }
            if let Ok(slant) = pat.as_ref().get::<fc::pat::Slant>(0) {
                font.slant = slant as i16;
            }
        }
    }

    fn search_for_host_platform_fonts(&mut self, maps: &mut FontMaps, name: &CStr) {
        if self.cached_all {
            return;
        }

        let bytes = name.to_bytes();
        let split = bytes
            .iter()
            .position(|c| *c == b'-')
            .map(|index| (&bytes[..index], &bytes[index + 1..]));

        let (fam_name, hyph) = match split {
            Some((fam, _)) => (fam, fam.len()),
            None => (&[] as &[_], 0),
        };

        let mut found = false;
        loop {
            'outer: for pos in 0..self.all_fonts.as_ref().fonts().len() {
                let font_set = self.all_fonts.as_ref();
                let pat = font_set.fonts()[pos].upgrade();
                if maps.platform_ref_to_font.contains_key(&pat) {
                    continue;
                }

                if self.cached_all {
                    let names = self.read_names(pat.clone());
                    maps.add_to_maps(self, pat.clone(), &names);
                    continue;
                }

                let mut i = 0;
                while let Ok(str) = pat.as_ref().get::<fc::pat::FullName>(i) {
                    if name == str {
                        let names = self.read_names(pat.clone());
                        maps.add_to_maps(self, pat.clone(), &names);
                        self.cache_family_members(maps, &names.family_names);
                        found = true;
                        continue 'outer;
                    }
                    i += 1;
                }

                let mut i = 0;
                while let Ok(str) = pat.as_ref().get::<fc::pat::Family>(i) {
                    if name == str || (hyph != 0 && fam_name == str.to_bytes()) {
                        let names = self.read_names(pat.clone());
                        maps.add_to_maps(self, pat.clone(), &names);
                        self.cache_family_members(maps, &names.family_names);
                        found = true;
                        continue 'outer;
                    }
                    let mut j = 0;
                    while let Ok(style) = pat.as_ref().get::<fc::pat::Style>(j) {
                        let mut full = str.to_bytes().to_owned();
                        full.push(b' ');
                        full.extend(style.to_bytes());

                        if name.to_bytes() == full {
                            let names = self.read_names(pat.clone());
                            maps.add_to_maps(self, pat.clone(), &names);
                            self.cache_family_members(maps, &names.family_names);
                            found = true;
                            continue 'outer;
                        }

                        j += 1;
                    }
                    i += 1;
                }
            }

            if found || self.cached_all {
                break;
            }
            self.cached_all = true;
        }
    }

    fn read_names(&self, pat: PlatformFontRef) -> NameCollection {
        let mut names = NameCollection::default();

        let pathname = match pat.as_ref().get::<fc::pat::File>(0) {
            Ok(name) => name,
            Err(_) => return names,
        };

        let index = match pat.as_ref().get::<fc::pat::Index>(0) {
            Ok(index) => index,
            Err(_) => return names,
        };

        let face = match ft::Face::new(pathname, index as usize) {
            Ok(face) => face,
            Err(_) => return names,
        };

        let name = match face.get_postscript_name() {
            Some(name) => name,
            None => return names,
        };

        names.ps_name = Some(name.to_owned());

        if face.is_sfnt() {
            let mut family_names = Vec::new();
            let mut sub_family_names = Vec::new();

            for i in 0..face.get_sfnt_name_count() {
                let mut utf8_name = None;
                let name_rec = match face.get_sfnt_name(i) {
                    Ok(name) => name,
                    Err(_) => continue,
                };

                match name_rec.name_id {
                    FONT_FULL_NAME
                    | FONT_FAMILY_NAME
                    | FONT_STYLE_NAME
                    | PREFERRED_FAMILY_NAME
                    | PREFERRED_SUBFAMILY_NAME => {
                        let mut preferred_name = false;
                        if name_rec.platform_id == ft::PlatformId::MACINTOSH
                            && name_rec.encoding_id == ft::EncodingId::MAC_ROMAN
                            && name_rec.language_id == ft::LanguageId::MAC_ENGLISH
                        {
                            let str = Str::<MacRoman>::from_bytes_infallible(name_rec.string);
                            utf8_name = Some(
                                enrede::CString::try_from(str.recode::<Utf8>().unwrap()).unwrap(),
                            );
                            preferred_name = true;
                        } else if name_rec.platform_id == ft::PlatformId::APPLE_UNICODE
                            || name_rec.platform_id == ft::PlatformId::MICROSOFT
                        {
                            let str = Str::<Utf16BE>::from_bytes(name_rec.string).unwrap();
                            utf8_name = Some(
                                enrede::CString::try_from(str.recode::<Utf8>().unwrap()).unwrap(),
                            );
                        }

                        if let Some(name) = utf8_name {
                            let name_list = match name_rec.name_id {
                                FONT_FULL_NAME => &mut names.full_names,
                                FONT_FAMILY_NAME => &mut names.family_names,
                                FONT_STYLE_NAME => &mut names.style_names,
                                PREFERRED_FAMILY_NAME => &mut family_names,
                                PREFERRED_SUBFAMILY_NAME => &mut sub_family_names,
                                _ => unreachable!(),
                            };

                            if preferred_name {
                                FontManager::prepend_to_list(name_list, name.into_std());
                            } else {
                                FontManager::append_to_list(name_list, name.into_std());
                            }
                        }
                    }
                    _ => (),
                }
            }
        } else {
            let mut index = 0;
            while let Ok(name) = pat.as_ref().get::<fc::pat::FullName>(index) {
                index += 1;
                FontManager::append_to_list(&mut names.full_names, name);
            }
            index = 0;
            while let Ok(fam) = pat.as_ref().get::<fc::pat::Family>(index) {
                index += 1;
                FontManager::append_to_list(&mut names.family_names, fam);
            }
            index = 0;
            while let Ok(name) = pat.as_ref().get::<fc::pat::FullName>(index) {
                index += 1;
                FontManager::append_to_list(&mut names.style_names, name);
            }
        }

        names
    }
}
