use crate::c_api::mac_core::{
    cf_to_cstr, kCFStringEncodingUTF8, kCFTypeDictionaryKeyCallBacks,
    kCFTypeDictionaryValueCallBacks, kCFTypeSetCallBacks, kCTFontDisplayNameAttribute,
    kCTFontFamilyNameAttribute, kCTFontFamilyNameKey, kCTFontFullNameKey, kCTFontNameAttribute,
    kCTFontStyleNameKey, kCTFontURLAttribute, CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef,
    CFDictionaryCreate, CFIndex, CFRelease, CFRetain, CFSetCreate, CFStringCreateWithCString,
    CFStringRef, CFURLGetFileSystemRepresentation, CTFontCopyAttribute, CTFontCopyLocalizedName,
    CTFontCopyName, CTFontCreateWithFontDescriptor, CTFontDescriptorCopyAttribute,
    CTFontDescriptorCreateMatchingFontDescriptors, CTFontDescriptorCreateWithAttributes,
    CTFontDescriptorRef, CTFontManagerCopyAvailableFontFamilyNames, CTFontRef,
};
use crate::c_api::manager::{
    base_get_op_size_rec_and_style_flags, Font, FontManager, FontManagerBackend, FontMaps,
    NameCollection,
};
use crate::c_api::PlatformFontRef;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::ptr;

unsafe fn find_fonts_with_name(name: CFStringRef, key: CFStringRef) -> CFArrayRef {
    let mut keys = &[key];
    let mut values = &[name];

    let attributes = CFDictionaryCreate(
        ptr::null_mut(),
        &mut (keys as *const [_]).cast(),
        &mut (values as *const [_]).cast(),
        1,
        &kCFTypeDictionaryKeyCallBacks,
        &kCFTypeDictionaryValueCallBacks,
    );
    let descriptor = CTFontDescriptorCreateWithAttributes(attributes);
    CFRelease(attributes.cast());

    let mandatory_attributes = CFSetCreate(
        ptr::null_mut(),
        &mut (keys as *const [_]).cast(),
        1,
        &kCFTypeSetCallBacks,
    );
    let matches = CTFontDescriptorCreateMatchingFontDescriptors(descriptor, mandatory_attributes);
    CFRelease(mandatory_attributes.cast());
    CFRelease(descriptor.cast());
    matches
}

unsafe fn find_font_with_name(name: CFStringRef, key: CFStringRef) -> CTFontDescriptorRef {
    let matches = find_fonts_with_name(name, key);

    let mut matched = ptr::null();
    if !matches.is_null() {
        if CFArrayGetCount(matches) != 0 {
            matched = CFArrayGetValueAtIndex(matches, 0);
            CFRetain(matched);
        }
        CFRelease(matches.cast());
    }
    matched.cast()
}

unsafe fn append_name_to_list(
    font: CTFontRef,
    name_list: &mut Vec<CString>,
    name_key: CFStringRef,
) {
    let name = CTFontCopyName(font, name_key);
    if !name.is_null() {
        FontManager::append_to_list(name_list, cf_to_cstr(name));
        CFRelease(name.cast());
    }
    let name = CTFontCopyLocalizedName(font, name_key, ptr::null_mut());
    if !name.is_null() {
        FontManager::append_to_list(name_list, cf_to_cstr(name));
        CFRelease(name.cast());
    }
}

pub struct MacBackend {}

impl MacBackend {
    pub fn new() -> MacBackend {
        MacBackend {}
    }

    unsafe fn add_fonts_to_caches(&self, maps: &mut FontMaps, members: CFArrayRef) {
        for i in 0..CFArrayGetCount(members) {
            let font = CFArrayGetValueAtIndex(members, i).cast();
            let names = self.read_names(font);
            maps.add_to_maps(self, font, &names)
        }
    }

    unsafe fn add_font_and_siblings_to_caches(
        &self,
        maps: &mut FontMaps,
        font: CTFontDescriptorRef,
    ) {
        let font = CTFontCreateWithFontDescriptor(font, 10.0, ptr::null_mut());
        if font.is_null() {
            return;
        }

        let family = CTFontCopyAttribute(font, kCTFontFamilyNameAttribute);
        CFRelease(font.cast());
        let matched = find_fonts_with_name(family.cast(), kCTFontFamilyNameAttribute);
        CFRelease(family.cast());
        self.add_fonts_to_caches(maps, matched);
        CFRelease(matched.cast());
    }

    unsafe fn add_family_to_caches(&self, maps: &mut FontMaps, family: CTFontDescriptorRef) {
        let name_str = CTFontDescriptorCopyAttribute(family, kCTFontFamilyNameAttribute);
        if !name_str.is_null() {
            let members = find_fonts_with_name(name_str.cast(), kCTFontFamilyNameAttribute);
            CFRelease(name_str);
            self.add_fonts_to_caches(maps, members);
            CFRelease(members.cast());
        }
    }
}

impl FontManagerBackend for MacBackend {
    unsafe fn initialize(&mut self) {}

    unsafe fn terminate(&mut self) {}

    unsafe fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> Cow<'a, CStr> {
        let mut path = Cow::Borrowed(cstr!("[unknown]"));

        let ct_font = CTFontCreateWithFontDescriptor(*font, 0.0, ptr::null_mut());
        if !ct_font.is_null() {
            #[cfg(feature = "MACOS_LE_10_6")]
            let url = {
                let mut fsref = ptr::null();
                let ats_font = CTFontGetPlatformFont(ct_font, ptr::null_mut());
                let status = ATSFontGetFileReference(ats_font, &mut fsref);
                if status == noErr {
                    CFURLCreateFromFSRef(ptr::null_mut(), &mut fsref)
                } else {
                    ptr::null_mut()
                }
            };
            #[cfg(not(feature = "MACOS_LE_10_6"))]
            let url = CTFontCopyAttribute(ct_font, kCTFontURLAttribute);

            if !url.is_null() {
                let mut buf = [0u8; libc::PATH_MAX as usize];
                if CFURLGetFileSystemRepresentation(
                    url.cast(),
                    true,
                    buf.as_mut_ptr(),
                    libc::PATH_MAX as CFIndex,
                ) {
                    let pos = buf.iter().rposition(|c| **c != 0).unwrap();
                    path = Cow::Owned(CString::new(buf[..pos]).unwrap());
                }
                CFRelease(url);
            }
            CFRelease(ct_font.cast());
        }

        path
    }

    unsafe fn get_op_size_rec_and_style_flags(&self, font: &mut Font) {
        base_get_op_size_rec_and_style_flags(font);
    }

    unsafe fn search_for_host_platform_fonts(&mut self, maps: &mut FontMaps, name: &CStr) {
        let name_str = CFStringCreateWithCString(ptr::null(), name.as_ptr(), kCFStringEncodingUTF8);
        let matched = find_font_with_name(name_str, kCTFontDisplayNameAttribute);
        if !matched.is_null() {
            self.add_font_and_siblings_to_caches(maps, matched);
            CFRelease(matched.cast());
            return;
        }

        let hyph = name.to_bytes().iter().copied().position(|c| c == b'-');
        if let Some(hyph) = hyph {
            let family = CString::new(&name.to_bytes()[..hyph]).unwrap();
            let family_str =
                CFStringCreateWithCString(ptr::null(), family.as_ptr(), kCFStringEncodingUTF8);

            let family_members = find_fonts_with_name(family_str, kCTFontFamilyNameAttribute);
            if CFArrayGetCount(family_members) > 0 {
                self.add_fonts_to_caches(maps, family_members);
                CFRelease(family_members.cast());
                return;
            }
            CFRelease(family_members.cast());

            let matched = find_font_with_name(family_str, kCTFontFamilyNameAttribute);
            if !matched.is_null() {
                self.add_family_to_caches(maps, matched);
                CFRelease(matched.cast());
                return;
            }
        }

        let matched = find_font_with_name(name_str, kCTFontNameAttribute);
        if !matched.is_null() {
            self.add_font_and_siblings_to_caches(maps, matched);
            CFRelease(matched.cast());
            return;
        }

        let family_members = find_fonts_with_name(name_str, kCTFontFamilyNameAttribute);
        if CFArrayGetCount(family_members) > 0 {
            self.add_fonts_to_caches(maps, family_members);
            CFRelease(family_members.cast());
            return;
        }
        CFRelease(family_members.cast());

        let matched = find_font_with_name(name_str, kCTFontFamilyNameAttribute);
        if !matched.is_null() {
            self.add_family_to_caches(maps, matched);
            CFRelease(matched.cast());
            return;
        }
    }

    unsafe fn read_names(&self, font: PlatformFontRef) -> NameCollection {
        let mut names = NameCollection::default();

        let ps_name = CTFontDescriptorCopyAttribute(font, kCTFontNameAttribute);
        if ps_name.is_null() {
            return names;
        }

        names.ps_name = Some(cf_to_cstr(ps_name.cast()));
        CFRelease(ps_name);

        let font = CTFontCreateWithFontDescriptor(font, 0.0, ptr::null());
        append_name_to_list(font, &mut names.full_names, kCTFontFullNameKey);
        append_name_to_list(font, &mut names.family_names, kCTFontFamilyNameKey);
        append_name_to_list(font, &mut names.style_names, kCTFontStyleNameKey);
        CFRelease(font.cast());

        names
    }
}
