use super::{sys, CFString, CFType, CTFontDescriptor, CoreType};
use std::ptr;
use std::ptr::NonNull;

#[derive(Copy, Clone)]
pub enum FontAttribute {
    Name,
    FamilyName,
    DisplayName,
    URL,
    CascadeList,
}

impl FontAttribute {
    pub fn to_str(self) -> CFString {
        CFString::new_borrowed(NonNull::new(self.to_raw().cast_mut()).unwrap())
    }

    pub fn to_raw(self) -> sys::CFStringRef {
        match self {
            FontAttribute::Name => unsafe { sys::kCTFontNameAttribute },
            FontAttribute::FamilyName => unsafe { sys::kCTFontFamilyNameAttribute },
            FontAttribute::DisplayName => unsafe { sys::kCTFontDisplayNameAttribute },
            FontAttribute::URL => unsafe { sys::kCTFontURLAttribute },
            FontAttribute::CascadeList => unsafe { sys::kCTFontCascadeListAttribute },
        }
    }
}

#[derive(Copy, Clone)]
pub enum FontNameKey {
    Full,
    Family,
    Style,
    PostScript,
}

impl FontNameKey {
    pub fn to_str(self) -> CFString {
        CFString::new_borrowed(NonNull::new(self.to_raw().cast_mut()).unwrap())
    }

    fn to_raw(self) -> sys::CFStringRef {
        match self {
            FontNameKey::Full => unsafe { sys::kCTFontFullNameKey },
            FontNameKey::Family => unsafe { sys::kCTFontFamilyNameKey },
            FontNameKey::Style => unsafe { sys::kCTFontStyleNameKey },
            FontNameKey::PostScript => unsafe { sys::kCTFontPostScriptNameKey },
        }
    }
}

cfty! {
    CTFont : CTFontGetTypeID
}

impl CTFont {
    pub fn new_descriptor(descriptor: &CTFontDescriptor, size: f64) -> CTFont {
        let ptr = unsafe {
            sys::CTFontCreateWithFontDescriptor(
                descriptor.as_type_ref(),
                size as sys::CGFloat,
                ptr::null_mut(),
            )
        };
        CTFont::new_owned(NonNull::new(ptr.cast_mut()).unwrap())
    }

    pub fn attr(&self, attr: FontAttribute) -> Option<CFType> {
        let ptr = unsafe { sys::CTFontCopyAttribute(self.as_type_ref(), attr.to_raw()) };
        NonNull::new(ptr.cast_mut()).map(CFType::new_owned)
    }

    pub fn name(&self, name: FontNameKey) -> Option<CFString> {
        let ptr = unsafe { sys::CTFontCopyName(self.as_type_ref(), name.to_raw()) };
        NonNull::new(ptr.cast_mut()).map(CFString::new_owned)
    }

    pub fn localized_name(&self, name: FontNameKey) -> Option<CFString> {
        let ptr = unsafe {
            sys::CTFontCopyLocalizedName(self.as_type_ref(), name.to_raw(), ptr::null_mut())
        };
        NonNull::new(ptr.cast_mut()).map(CFString::new_owned)
    }
}
