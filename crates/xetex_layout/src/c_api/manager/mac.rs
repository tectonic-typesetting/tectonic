use crate::c_api::mac_core::{
    cf_to_cstr, kCFStringEncodingUTF8, kCFTypeDictionaryKeyCallBacks,
    kCFTypeDictionaryValueCallBacks, kCFTypeSetCallBacks, kCTFontDisplayNameAttribute,
    kCTFontFamilyNameAttribute, kCTFontFamilyNameKey, kCTFontFullNameKey, kCTFontNameAttribute,
    kCTFontStyleNameKey, CFArrayGetCount, CFArrayGetValueAtIndex, CFDictionaryCreate, CFRelease,
    CFRetain, CFSetCreate, CFStringCreateWithCString, CFStringRef, CTFontCopyLocalizedName,
    CTFontCopyName, CTFontCreateWithFontDescriptor, CTFontDescriptorCopyAttribute,
    CTFontDescriptorCreateMatchingFontDescriptors, CTFontDescriptorCreateWithAttributes,
    CTFontDescriptorRef, CTFontManagerCopyAvailableFontFamilyNames, CTFontRef,
};
use crate::c_api::manager::{Font, FontManager, FontManagerBackend, FontMaps, NameCollection};
use crate::c_api::PlatformFontRef;
use std::ffi::{CStr, CString};
use std::ptr;

unsafe fn find_font_with_name(name: CFStringRef, key: CFStringRef) -> CTFontDescriptorRef {
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
}

impl FontManagerBackend for MacBackend {
    unsafe fn initialize(&mut self) {
        todo!()
    }

    unsafe fn terminate(&mut self) {
        todo!()
    }

    fn get_platform_font_desc<'a>(&'a self, font: &'a PlatformFontRef) -> &'a CStr {
        todo!()
    }

    unsafe fn get_op_size_rec_and_style_flags(&self, font: &mut Font) {
        todo!()
    }

    unsafe fn search_for_host_platform_fonts(&mut self, maps: &mut FontMaps, name: &CStr) {
        let name_str = CFStringCreateWithCString(ptr::null(), name.as_ptr(), kCFStringEncodingUTF8);
        let matched = find_font_with_name(name_str, kCTFontDisplayNameAttribute);
        if !matched.is_null() {
            self.add_font_and_siblings_to_caches(matched);
            CFRelease(matched);
            return;
        }

        let hyph = name.to_bytes().iter().copied().position(|c| c == '-');
        if let Some(hyph) = hyph {
            let family = CString::new(&name.to_bytes()[..hyph]).unwrap();
            let family_str =
                CFStringCreateWithCString(ptr::null(), family.as_ptr(), kCFStringEncodingUTF8);

            let family_members = todo!("[[NSFontManager sharedFontManager] availableMembersOfFontFamily: (NSString*)familyStr]");
            if CFArrayGetCount(family_members) > 0 {
                self.add_fonts_to_caches(family_members);
            }

            let matched = find_font_with_name(family_str, kCTFontFamilyNameAttribute);
            if !matched.is_null() {
                self.add_family_to_caches(matched);
                CFRelease(matched);
                return;
            }
        }

        let matched = find_font_with_name(name_str, kCTFontNameAttribute);
        if !matched.is_null() {
            self.add_font_and_siblings_to_caches(matched);
            CFRelease(matched);
            return;
        }

        let family_members = todo!("[[NSFontManager sharedFontManager] availableMembersOfFontFamily: (NSString*)familyStr]");
        if CFArrayGetCount(family_members) > 0 {
            self.add_fonts_to_caches(family_members);
            return;
        }

        let matched = find_font_with_name(name_str, kCTFontFamilyNameAttribute);
        if !matched.is_null() {
            self.add_family_to_caches(matched);
            CFRelease(matched);
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
