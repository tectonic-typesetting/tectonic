use crate::c_api::PlatformFontRef;
use crate::manager::{
    base_get_op_size_rec_and_style_flags, FontInfo, FontManager, FontManagerBackend, FontMaps,
    NameCollection,
};
use core_foundation::array::CFArray;
use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionary;
use core_foundation::set::CFSet;
use core_foundation::string::CFString;
use core_text::font::{CTFont, CTFontNameSpecifier};
use core_text::font_descriptor::{
    kCTFontDisplayNameAttribute, kCTFontFamilyNameAttribute, kCTFontNameAttribute,
    CTFontDescriptor, CTFontDescriptorCreateMatchingFontDescriptors,
};
use core_text::{font, font_descriptor};
use std::borrow::Cow;
use std::ffi::{CStr, CString};

fn find_fonts_with_name(name: CFString, key: CFString) -> CFArray<CTFontDescriptor> {
    let attributes = CFDictionary::from_CFType_pairs(&[(key.clone(), name.as_CFType())]);
    let descriptor = font_descriptor::new_from_attributes(&attributes);

    let mandatory_attributes = CFSet::from_slice(&[key]);
    let matching = unsafe {
        CTFontDescriptorCreateMatchingFontDescriptors(
            descriptor.as_concrete_TypeRef(),
            mandatory_attributes.as_concrete_TypeRef(),
        )
    };
    unsafe { CFArray::wrap_under_create_rule(matching) }
}

fn find_font_with_name(name: CFString, key: CFString) -> Option<CTFontDescriptor> {
    let matches = find_fonts_with_name(name, key);

    let mut matched = None;
    if !matches.is_empty() {
        matched = Some(matches.get(0).unwrap().clone());
    }
    matched
}

fn append_name_to_list(font: &CTFont, name_list: &mut Vec<CString>, name_key: CTFontNameSpecifier) {
    let name = font.get_string_by_name_key(name_key);
    if let Some(name) = name {
        FontManager::append_to_list(name_list, CString::new(name.into_bytes()).unwrap());
    }
    // TODO: Not yet supported
    // let name = font.get_string_by_name_key(name_key);
    // if let Some(name) = name {
    //     FontManager::append_to_list(name_list, CString::new(name.into_bytes()).unwrap());
    // }
}

pub struct MacBackend {}

impl MacBackend {
    pub fn new() -> MacBackend {
        MacBackend {}
    }

    fn add_fonts_to_caches(&self, maps: &mut FontMaps, members: CFArray<CTFontDescriptor>) {
        for i in 0..members.len() {
            let font = &members.get(i).unwrap();
            let names = self.read_names(CTFontDescriptor::clone(font));
            maps.add_to_maps(self, CTFontDescriptor::clone(font), &names)
        }
    }

    fn add_font_and_siblings_to_caches(&self, maps: &mut FontMaps, font: &CTFontDescriptor) {
        let font = font::new_from_descriptor(font, 10.0);
        let family = font.family_name();
        let matched = find_fonts_with_name(CFString::new(&family), unsafe {
            CFString::wrap_under_get_rule(kCTFontFamilyNameAttribute)
        });
        self.add_fonts_to_caches(maps, matched);
    }

    fn add_family_to_caches(&self, maps: &mut FontMaps, family: CTFontDescriptor) {
        let name_str = family.family_name();
        let members = find_fonts_with_name(CFString::new(&name_str), unsafe {
            CFString::wrap_under_get_rule(kCTFontFamilyNameAttribute)
        });
        self.add_fonts_to_caches(maps, members);
    }
}

impl FontManagerBackend for MacBackend {
    fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> Cow<'a, CStr> {
        let mut path = Cow::Borrowed(c"[unknown]");

        let ct_font = font::new_from_descriptor(font, 0.0);
        let url = ct_font.url();

        if let Some(url) = url {
            if let Some(fs_path) = url.to_path() {
                path = Cow::Owned(fs_path);
            }
        }

        path
    }

    fn get_op_size_rec_and_style_flags(&self, font: &mut FontInfo) {
        base_get_op_size_rec_and_style_flags(font);
    }

    fn search_for_host_platform_fonts(&mut self, maps: &mut FontMaps, name: &CStr) {
        let name_str = CFString::new(&name.to_string_lossy());
        let matched = find_font_with_name(name_str.clone(), unsafe {
            CFString::wrap_under_get_rule(kCTFontDisplayNameAttribute)
        });
        if let Some(matched) = matched {
            self.add_font_and_siblings_to_caches(maps, &matched);
            return;
        }

        let hyph = name.to_bytes().iter().copied().position(|c| c == b'-');
        if let Some(hyph) = hyph {
            let family = &name.to_bytes()[..hyph];
            let family_str = CFString::new(&String::from_utf8_lossy(family));
            let family_members = find_fonts_with_name(family_str.clone(), unsafe {
                CFString::wrap_under_get_rule(kCTFontFamilyNameAttribute)
            });
            if !family_members.is_empty() {
                self.add_fonts_to_caches(maps, family_members);
                return;
            }

            let matched = find_font_with_name(family_str, unsafe {
                CFString::wrap_under_get_rule(kCTFontFamilyNameAttribute)
            });
            if let Some(matched) = matched {
                self.add_family_to_caches(maps, matched);
                return;
            }
        }

        let matched = find_font_with_name(name_str.clone(), unsafe {
            CFString::wrap_under_get_rule(kCTFontNameAttribute)
        });
        if let Some(matched) = matched {
            self.add_font_and_siblings_to_caches(maps, &matched);
            return;
        }

        let family_members = find_fonts_with_name(name_str.clone(), unsafe {
            CFString::wrap_under_get_rule(kCTFontFamilyNameAttribute)
        });
        if !family_members.is_empty() {
            self.add_fonts_to_caches(maps, family_members);
            return;
        }

        let matched = find_font_with_name(name_str, unsafe {
            CFString::wrap_under_get_rule(kCTFontFamilyNameAttribute)
        });
        if let Some(matched) = matched {
            self.add_family_to_caches(maps, matched);
            return;
        }
    }

    fn read_names(&self, font: PlatformFontRef) -> NameCollection {
        let mut names = NameCollection::default();

        let ps_name = font.font_name();

        names.ps_name = Some(CString::new(ps_name.into_bytes()).unwrap());

        let font = font::new_from_descriptor(&font, 0.0);
        append_name_to_list(&font, &mut names.full_names, CTFontNameSpecifier::Full);
        append_name_to_list(&font, &mut names.family_names, CTFontNameSpecifier::Family);
        append_name_to_list(&font, &mut names.style_names, CTFontNameSpecifier::Style);

        names
    }
}
