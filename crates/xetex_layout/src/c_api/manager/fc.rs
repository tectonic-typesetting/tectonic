use super::{
    base_get_op_size_rec_and_style_flags, Font, FontManager, FontManagerBackend, FontMaps,
    NameCollection,
};
use crate::c_api::fc::sys::{
    FcConfigGetCurrent, FcFalse, FcFontList, FcFontSet, FcFontSetDestroy, FcInit, FcObjectSetBuild,
    FcObjectSetDestroy, FC_FAMILY, FC_FILE, FC_FONTFORMAT, FC_FULLNAME, FC_INDEX, FC_SLANT,
    FC_STYLE, FC_WEIGHT, FC_WIDTH,
};
use crate::c_api::{fc, PlatformFontRef};
use std::borrow::Cow;
use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::ptr;
use tectonic_bridge_freetype2 as ft;
use tectonic_bridge_icu::{
    ucnv_close, ucnv_fromUChars, ucnv_open, ucnv_toUChars, UConverter, U_SUCCESS, U_ZERO_ERROR,
};

pub const FONT_FAMILY_NAME: libc::c_ushort = 1;
pub const FONT_STYLE_NAME: libc::c_ushort = 2;
pub const FONT_FULL_NAME: libc::c_ushort = 4;
pub const PREFERRED_FAMILY_NAME: libc::c_ushort = 16;
pub const PREFERRED_SUBFAMILY_NAME: libc::c_ushort = 17;

thread_local! {
    static MAC_ROMAN_CONV: Cell<*mut UConverter> = const { Cell::new(ptr::null_mut()) };
    static UTF16_BE_CONV: Cell<*mut UConverter> = const { Cell::new(ptr::null_mut()) };
    static UTF8_CONV: Cell<*mut UConverter> = const { Cell::new(ptr::null_mut()) };
}

unsafe fn convert_to_utf8(conv: *mut UConverter, name: &[u8]) -> CString {
    let buf_size = 2 * name.len() + 100;
    let mut buffer1 = vec![0; buf_size];
    let mut buffer2 = vec![0; buf_size];

    let mut status = U_ZERO_ERROR;
    let len = ucnv_toUChars(
        conv,
        buffer1.as_mut_ptr(),
        buf_size as _,
        name.as_ptr().cast(),
        name.len() as i32,
        &mut status,
    );
    let len = ucnv_fromUChars(
        UTF8_CONV.get(),
        buffer2.as_mut_ptr() as _,
        buf_size as _,
        buffer1.as_ptr(),
        len,
        &mut status,
    ) as usize;
    buffer2[len] = 0;
    buffer2.truncate(len + 1);
    CString::from_vec_with_nul(buffer2).unwrap()
}

pub struct FcBackend {
    all_fonts: *mut FcFontSet,
    cached_all: bool,
}

impl FcBackend {
    pub fn new() -> FcBackend {
        FcBackend {
            all_fonts: ptr::null_mut(),
            cached_all: false,
        }
    }

    unsafe fn cache_family_members(&mut self, maps: &mut FontMaps, names: &[CString]) {
        if names.is_empty() {
            return;
        }

        'outer: for f in 0..(*self.all_fonts).nfont as usize {
            let pat = fc::Pattern::from_raw(*(*self.all_fonts).fonts.add(f)).unwrap();
            if maps.platform_ref_to_font.contains_key(&pat) {
                continue;
            }

            let mut i = 0;
            while let Ok(str) = pat.get::<fc::pat::Family>(i) {
                for name in names {
                    if **name == *str {
                        let names = self.read_names(pat.clone());
                        maps.add_to_maps(self, pat.clone(), &names);
                        continue 'outer;
                    }
                }

                i += 1;
            }
        }
    }
}

impl FontManagerBackend for FcBackend {
    unsafe fn initialize(&mut self) {
        if FcInit() == FcFalse {
            panic!("fontconfig initialization failed");
        }

        ft::init();

        let mut err = U_ZERO_ERROR;

        MAC_ROMAN_CONV.set(ucnv_open(c!("macintosh"), &mut err));
        if !U_SUCCESS(err) {
            err = U_ZERO_ERROR;
            MAC_ROMAN_CONV.set(ptr::null_mut());
        }

        UTF16_BE_CONV.set(ucnv_open(c!("UTF16BE"), &mut err));
        UTF8_CONV.set(ucnv_open(c!("UTF8"), &mut err));
        if !U_SUCCESS(err) {
            panic!("cannot read font names");
        }

        let pat = fc::OwnPattern::from_name(cstr!(":outline=true")).unwrap();
        let os = FcObjectSetBuild(
            FC_FAMILY,
            FC_STYLE,
            FC_FILE,
            FC_INDEX,
            FC_FULLNAME,
            FC_WEIGHT,
            FC_WIDTH,
            FC_SLANT,
            FC_FONTFORMAT,
            ptr::null::<()>(),
        );
        self.all_fonts = FcFontList(FcConfigGetCurrent(), pat.as_raw(), os);
        FcObjectSetDestroy(os);
        self.cached_all = false;
    }

    unsafe fn terminate(&mut self) {
        if !self.all_fonts.is_null() {
            FcFontSetDestroy(self.all_fonts);
            self.all_fonts = ptr::null_mut();
        }

        if !MAC_ROMAN_CONV.get().is_null() {
            ucnv_close(MAC_ROMAN_CONV.get());
            MAC_ROMAN_CONV.set(ptr::null_mut());
        }
        if !UTF16_BE_CONV.get().is_null() {
            ucnv_close(UTF16_BE_CONV.get());
            UTF16_BE_CONV.set(ptr::null_mut());
        }
        if !UTF8_CONV.get().is_null() {
            ucnv_close(UTF8_CONV.get());
            UTF8_CONV.set(ptr::null_mut());
        }
    }

    unsafe fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> Cow<'a, CStr> {
        if let Ok(str) = font.get::<fc::pat::File>(0) {
            Cow::Borrowed(str)
        } else {
            Cow::Borrowed(cstr!("[unknown]"))
        }
    }

    unsafe fn get_op_size_rec_and_style_flags(&self, font: &mut Font) {
        base_get_op_size_rec_and_style_flags(font);

        if font.weight == 0 && font.width == 0 {
            let pat = &font.font_ref;
            if let Ok(weight) = pat.get::<fc::pat::Weight>(0) {
                font.weight = weight as u16;
            }
            if let Ok(width) = pat.get::<fc::pat::Width>(0) {
                font.width = width as u16;
            }
            if let Ok(slant) = pat.get::<fc::pat::Slant>(0) {
                font.slant = slant as i16;
            }
        }
    }

    unsafe fn search_for_host_platform_fonts(&mut self, maps: &mut FontMaps, name: &CStr) {
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
            'outer: for f in 0..(*self.all_fonts).nfont as usize {
                let pat = fc::Pattern::from_raw(*(*self.all_fonts).fonts.add(f)).unwrap();
                if maps.platform_ref_to_font.contains_key(&pat) {
                    continue;
                }

                if self.cached_all {
                    let names = self.read_names(pat.clone());
                    maps.add_to_maps(self, pat.clone(), &names);
                    continue;
                }

                let mut i = 0;
                while let Ok(str) = pat.get::<fc::pat::FullName>(i) {
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
                while let Ok(str) = pat.get::<fc::pat::Family>(i) {
                    if name == str || (hyph != 0 && fam_name == str.to_bytes()) {
                        let names = self.read_names(pat.clone());
                        maps.add_to_maps(self, pat.clone(), &names);
                        self.cache_family_members(maps, &names.family_names);
                        found = true;
                        continue 'outer;
                    }
                    let mut j = 0;
                    while let Ok(style) = pat.get::<fc::pat::Style>(j) {
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

    unsafe fn read_names(&self, pat: PlatformFontRef) -> NameCollection {
        let mut names = NameCollection::default();

        let pathname = match pat.get::<fc::pat::File>(0) {
            Ok(name) => name,
            Err(_) => return names,
        };

        let index = match pat.get::<fc::pat::Index>(0) {
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
                        let roman_conv = MAC_ROMAN_CONV.get();
                        if !roman_conv.is_null()
                            && name_rec.platform_id == ft::PlatformId::MACINTOSH
                            && name_rec.encoding_id == ft::EncodingId::MAC_ROMAN
                            && name_rec.language_id == ft::LanguageId::MAC_ENGLISH
                        {
                            utf8_name = Some(convert_to_utf8(roman_conv, name_rec.string));
                            preferred_name = true;
                        } else if name_rec.platform_id == ft::PlatformId::APPLE_UNICODE
                            || name_rec.platform_id == ft::PlatformId::MICROSOFT
                        {
                            utf8_name = Some(convert_to_utf8(UTF16_BE_CONV.get(), name_rec.string));
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
                                FontManager::prepend_to_list(name_list, name);
                            } else {
                                FontManager::append_to_list(name_list, name);
                            }
                        }
                    }
                    _ => (),
                }
            }
        } else {
            let mut index = 0;
            while let Ok(name) = pat.get::<fc::pat::FullName>(index) {
                index += 1;
                FontManager::append_to_list(&mut names.full_names, name);
            }
            index = 0;
            while let Ok(fam) = pat.get::<fc::pat::Family>(index) {
                index += 1;
                FontManager::append_to_list(&mut names.family_names, fam);
            }
            index = 0;
            while let Ok(name) = pat.get::<fc::pat::FullName>(index) {
                index += 1;
                FontManager::append_to_list(&mut names.style_names, name);
            }
        }

        names
    }
}
