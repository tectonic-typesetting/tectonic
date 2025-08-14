use super::{sys, CFArray, CFDictionary, CFSet, CFString, CFType, CoreType, FontAttribute};
use std::ptr::NonNull;

cfty! {
    /// A font descriptor, a typeface that can be paired with a size and transform to get a specific
    /// font.
    CTFontDescriptor : CTFontDescriptorGetTypeID
}

impl CTFontDescriptor {
    /// Create a new descriptor from a dictionary of its attributes. Unrecognized attributes will
    /// be ignored.
    pub fn new_with_attrs(attrs: &CFDictionary<CFString, CFType>) -> CTFontDescriptor {
        // SAFETY: Attrs is guaranteed to be valid
        let ptr = unsafe { sys::CTFontDescriptorCreateWithAttributes(attrs.as_type_ref()) };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap();
        // SAFETY: If non-null, pointer from CTFontDescriptorCreateWithAttributes is a new, owned CTFontDescriptor.
        unsafe { CTFontDescriptor::new_owned(ptr) }
    }

    /// Create a copy of this descriptor, but with additional attributes as specified by `attrs`.
    /// Overlapping attributes are replaced.
    pub fn copy_with_attrs(&self, attrs: &CFDictionary<CFString, CFType>) -> CTFontDescriptor {
        // SAFETY: Self and attrs are guaranteed to be valid
        let ptr = unsafe {
            sys::CTFontDescriptorCreateCopyWithAttributes(self.as_type_ref(), attrs.as_type_ref())
        };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap();
        // SAFETY: If non-null, pointer from CTFontDescriptorCreateCopyWithAttributes is a new, owned CTFontDescriptor.
        unsafe { CTFontDescriptor::new_owned(ptr) }
    }

    /// Find all loaded fonts that contain the provided mandatory properties.
    pub fn matching_font_descriptors(
        &self,
        mandatory: &CFSet<CFString>,
    ) -> CFArray<CTFontDescriptor> {
        // SAFETY: Self and mandatory are guaranteed to be valid
        let ptr = unsafe {
            sys::CTFontDescriptorCreateMatchingFontDescriptors(
                self.as_type_ref(),
                mandatory.as_type_ref(),
            )
        };
        NonNull::new(ptr.cast_mut())
            // SAFETY: If non-null, pointer from CTFontDescriptorCreateMatchingFontDescriptors is a
            //         new, owned CFArray of CTFontDescriptor
            .map(|ptr| unsafe { CFArray::new_owned(ptr) })
            .unwrap_or_else(CFArray::empty)
    }

    /// Get an attribute of this font descriptor, if present.
    pub fn attr(&self, attr: FontAttribute) -> Option<CFType> {
        // SAFETY: Self and attr are guaranteed to be valid
        let ptr = unsafe { sys::CTFontDescriptorCopyAttribute(self.as_type_ref(), attr.to_raw()) };
        // SAFETY: If non-null, pointer from CTFontDescriptorCreateMatchingFontDescriptors is a new, owned CFType instance
        NonNull::new(ptr.cast_mut()).map(|ptr| unsafe { CFType::new_owned(ptr) })
    }
}
