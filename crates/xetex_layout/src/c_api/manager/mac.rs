use crate::c_api::manager::{
    base_get_op_size_rec_and_style_flags, FontInfo, FontManager, FontManagerBackend, FontMaps,
    NameCollection,
};
use crate::c_api::PlatformFontRef;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use tectonic_mac_core::{
    CFArray, CFDictionary, CFSet, CFString, CFUrl, CTFont, CTFontDescriptor, CoreType,
    FontAttribute, FontNameKey,
};

fn find_fonts_with_name(name: CFString, key: FontAttribute) -> CFArray<CTFontDescriptor> {
    let attributes = CFDictionary::new([(key.to_str(), name.into_ty())]);
    let descriptor = CTFontDescriptor::new_with_attrs(&attributes);

    let mandatory_attributes = CFSet::new(&[key.to_str()]);
    descriptor.matching_font_descriptors(&mandatory_attributes)
}

fn find_font_with_name(name: CFString, key: FontAttribute) -> Option<CTFontDescriptor> {
    let matches = find_fonts_with_name(name, key);

    let mut matched = None;
    if !matches.is_empty() {
        matched = Some(matches[0].clone());
    }
    matched
}

fn append_name_to_list(font: &CTFont, name_list: &mut Vec<CString>, name_key: FontNameKey) {
    let name = font.name(name_key);
    if let Some(name) = name {
        FontManager::append_to_list(name_list, name.as_cstr());
    }
    let name = font.localized_name(name_key);
    if let Some(name) = name {
        FontManager::append_to_list(name_list, name.as_cstr());
    }
}

pub struct MacBackend {}

impl MacBackend {
    pub fn new() -> MacBackend {
        MacBackend {}
    }

    fn add_fonts_to_caches(&self, maps: &mut FontMaps, members: CFArray<CTFontDescriptor>) {
        for i in 0..members.len() {
            let font = &members[i];
            let names = self.read_names(font.clone());
            maps.add_to_maps(self, font.clone(), &names)
        }
    }

    fn add_font_and_siblings_to_caches(&self, maps: &mut FontMaps, font: &CTFontDescriptor) {
        let font = CTFont::new_descriptor(font, 10.0);
        let family = font
            .attr(FontAttribute::FamilyName)
            .unwrap()
            .downcast::<CFString>()
            .unwrap();
        let matched = find_fonts_with_name(family, FontAttribute::FamilyName);
        self.add_fonts_to_caches(maps, matched);
    }

    fn add_family_to_caches(&self, maps: &mut FontMaps, family: CTFontDescriptor) {
        let name_str = family
            .attr(FontAttribute::FamilyName)
            .and_then(|ty| ty.downcast::<CFString>().ok());
        if let Some(name_str) = name_str {
            let members = find_fonts_with_name(name_str, FontAttribute::FamilyName);
            self.add_fonts_to_caches(maps, members);
        }
    }
}

impl FontManagerBackend for MacBackend {
    fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> Cow<'a, CStr> {
        let mut path = Cow::Borrowed(cstr!("[unknown]"));

        let ct_font = CTFont::new_descriptor(font, 0.0);
        let url = ct_font
            .attr(FontAttribute::URL)
            .and_then(|ty| ty.downcast::<CFUrl>().ok());

        if let Some(url) = url {
            if let Some(fs_path) = url.fs_representation() {
                path = Cow::Owned(fs_path);
            }
        }

        path
    }

    fn get_op_size_rec_and_style_flags(&self, font: &mut FontInfo) {
        base_get_op_size_rec_and_style_flags(font);
    }

    fn search_for_host_platform_fonts(&mut self, maps: &mut FontMaps, name: &CStr) {
        let name_str = CFString::new(name);
        let matched = find_font_with_name(name_str.clone(), FontAttribute::DisplayName);
        if let Some(matched) = matched {
            self.add_font_and_siblings_to_caches(maps, &matched);
            return;
        }

        let hyph = name.to_bytes().iter().copied().position(|c| c == b'-');
        if let Some(hyph) = hyph {
            let family = CString::new(&name.to_bytes()[..hyph]).unwrap();
            let family_str = CFString::new(&*family);
            let family_members =
                find_fonts_with_name(family_str.clone(), FontAttribute::FamilyName);
            if !family_members.is_empty() {
                self.add_fonts_to_caches(maps, family_members);
                return;
            }

            let matched = find_font_with_name(family_str, FontAttribute::FamilyName);
            if let Some(matched) = matched {
                self.add_family_to_caches(maps, matched);
                return;
            }
        }

        let matched = find_font_with_name(name_str.clone(), FontAttribute::Name);
        if let Some(matched) = matched {
            self.add_font_and_siblings_to_caches(maps, &matched);
            return;
        }

        let family_members = find_fonts_with_name(name_str.clone(), FontAttribute::FamilyName);
        if !family_members.is_empty() {
            self.add_fonts_to_caches(maps, family_members);
            return;
        }

        let matched = find_font_with_name(name_str, FontAttribute::FamilyName);
        if let Some(matched) = matched {
            self.add_family_to_caches(maps, matched);
            return;
        }
    }

    fn read_names(&self, font: PlatformFontRef) -> NameCollection {
        let mut names = NameCollection::default();

        let ps_name = match font.attr(FontAttribute::Name) {
            Some(ps_name) => ps_name,
            None => return names,
        };
        let ps_name = ps_name.downcast::<CFString>().unwrap();

        names.ps_name = Some(ps_name.get_cstring());

        let font = CTFont::new_descriptor(&font, 0.0);
        append_name_to_list(&font, &mut names.full_names, FontNameKey::Full);
        append_name_to_list(&font, &mut names.family_names, FontNameKey::Family);
        append_name_to_list(&font, &mut names.style_names, FontNameKey::Style);

        names
    }
}
