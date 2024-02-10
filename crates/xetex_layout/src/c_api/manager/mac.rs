use crate::c_api::mac_core::{
    kCFStringEncodingUTF8, kCFTypeDictionaryKeyCallBacks, kCFTypeDictionaryValueCallBacks,
    kCFTypeSetCallBacks, kCTFontFamilyNameKey, kCTFontFullNameKey, kCTFontNameAttribute,
    kCTFontStyleNameKey, CFArrayGetCount, CFArrayGetValueAtIndex, CFDictionaryCreate, CFRelease,
    CFRetain, CFSetCreate, CFStringGetCString, CFStringGetLength, CFStringRef,
    CTFontCreateWithFontDescriptor, CTFontDescriptorCopyAttribute,
    CTFontDescriptorCreateMatchingFontDescriptors, CTFontDescriptorCreateWithAttributes,
    CTFontDescriptorRef, CTFontRef,
};
use crate::c_api::manager::{Font, FontManagerBackend, FontMaps, NameCollection};
use crate::c_api::{xmalloc, PlatformFontRef};
use std::ffi::{CStr, CString};
use std::ptr;

unsafe fn find_font_with_name(name: CFStringRef, key: CFStringRef) -> CTFontDescriptorRef {
    let mut keys = &[key];
    let mut values = &[name];

    let attributes = CFDictionaryCreate(
        ptr::null_mut(),
        &mut (keys as *const _).cast(),
        &mut (values as *const _).cast(),
        1,
        &kCFTypeDictionaryKeyCallBacks,
        &kCFTypeDictionaryValueCallBacks,
    );
    let descriptor = CTFontDescriptorCreateWithAttributes(attributes);
    CFRelease(attributes.cast());

    let mandatory_attributes = CFSetCreate(
        ptr::null_mut(),
        &mut (keys as *const _).cast(),
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

fn append_name_to_list(font: CTFontRef, name_list: &mut Vec<CString>, name_key: CFStringRef) {
    todo!()
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
        todo!()
    }

    unsafe fn read_names(&self, font: PlatformFontRef) -> NameCollection {
        let mut names = NameCollection::default();

        let ps_name = CTFontDescriptorCopyAttribute(font, kCTFontNameAttribute);
        if ps_name.is_null() {
            return names;
        }

        let mut len = CFStringGetLength(ps_name.cast());
        len = len * 6 + 1;
        let buf = xmalloc(len as _);
        if CFStringGetCString(ps_name.cast(), buf, len, kCFStringEncodingUTF8) {
            // TODO: This is dubious, since pointer didn't come from `into_raw`
            //       Better handling should be possible once things work
            names.ps_name = Some(CString::from_raw(buf));
        } else {
            panic!();
        }
        CFRelease(ps_name);

        let font = CTFontCreateWithFontDescriptor(font, 0.0, ptr::null());
        append_name_to_list(font, &mut names.full_names, kCTFontFullNameKey);
        append_name_to_list(font, &mut names.family_names, kCTFontFamilyNameKey);
        append_name_to_list(font, &mut names.style_names, kCTFontStyleNameKey);
        CFRelease(font.cast());

        names
    }
}
