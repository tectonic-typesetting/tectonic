use crate::c_api::mac_core::{
    sys, CFArray, CFDictionary, CFSet, CFString, CFType, CoreType, FontAttribute,
};
use std::ptr::NonNull;

cfty! {
    CTFontDescriptor : CTFontDescriptorGetTypeID
}

impl CTFontDescriptor {
    pub fn new_with_attrs(attrs: &CFDictionary<CFString, CFType>) -> CTFontDescriptor {
        let ptr = unsafe { sys::CTFontDescriptorCreateWithAttributes(attrs.as_type_ref()) };
        CTFontDescriptor::new_owned(NonNull::new(ptr.cast_mut()).unwrap())
    }

    pub fn matching_font_descriptors(
        &self,
        mandatory: &CFSet<CFString>,
    ) -> CFArray<CTFontDescriptor> {
        let ptr = unsafe {
            sys::CTFontDescriptorCreateMatchingFontDescriptors(
                self.as_type_ref(),
                mandatory.as_type_ref(),
            )
        };
        CFArray::new_owned(NonNull::new(ptr.cast_mut()).unwrap())
    }

    pub fn attr(&self, attr: FontAttribute) -> Option<CFType> {
        let ptr = unsafe { sys::CTFontDescriptorCopyAttribute(self.as_type_ref(), attr.to_raw()) };
        NonNull::new(ptr.cast_mut()).map(CFType::new_owned)
    }
}
