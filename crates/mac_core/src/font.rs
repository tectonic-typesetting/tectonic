use super::{sys, CFString, CFType, CTFontDescriptor, CoreType};
use std::ptr;
use std::ptr::NonNull;

/// An attribute that may be found in a font, such as family name or URL.
#[derive(Copy, Clone)]
pub enum FontAttribute {
    /// Name
    Name,
    /// Family Name
    FamilyName,
    /// Display Name
    DisplayName,
    /// URL
    URL,
    /// Cascade List - fallbacks to use if glyph not present in this font.
    CascadeList,
}

impl FontAttribute {
    /// Convert this attribute into its matching [`CFString`].
    pub fn to_str(self) -> CFString {
        let ptr = NonNull::new(self.to_raw().cast_mut()).unwrap();
        // SAFETY: The value returned by `to_raw` is guaranteed to be a valid CFStringRef
        unsafe { CFString::new_borrowed(ptr) }
    }

    /// Convert this attribute into a raw pointer to a [`CFString`].
    pub fn to_raw(self) -> sys::CFStringRef {
        match self {
            // SAFETY: Static guaranteed to exist and by a valid CFStringRef
            FontAttribute::Name => unsafe { sys::kCTFontNameAttribute },
            // SAFETY: sic
            FontAttribute::FamilyName => unsafe { sys::kCTFontFamilyNameAttribute },
            // SAFETY: sic
            FontAttribute::DisplayName => unsafe { sys::kCTFontDisplayNameAttribute },
            // SAFETY: sic
            FontAttribute::URL => unsafe { sys::kCTFontURLAttribute },
            // SAFETY: sic
            FontAttribute::CascadeList => unsafe { sys::kCTFontCascadeListAttribute },
        }
    }
}

/// A type of font name that may be available.
#[derive(Copy, Clone)]
pub enum FontNameKey {
    /// Full name
    Full,
    /// Family name
    Family,
    /// Style name
    Style,
    /// PostScript name
    PostScript,
}

impl FontNameKey {
    /// Convert this attribute into its matching [`CFString`].
    pub fn to_str(self) -> CFString {
        let ptr = NonNull::new(self.to_raw().cast_mut()).unwrap();
        // SAFETY: The value returned by `to_raw` is guaranteed to be a valid CFStringRef
        unsafe { CFString::new_borrowed(ptr) }
    }

    fn to_raw(self) -> sys::CFStringRef {
        match self {
            // SAFETY: Static guaranteed to exist and by a valid CFStringRef
            FontNameKey::Full => unsafe { sys::kCTFontFullNameKey },
            // SAFETY: sic
            FontNameKey::Family => unsafe { sys::kCTFontFamilyNameKey },
            // SAFETY: sic
            FontNameKey::Style => unsafe { sys::kCTFontStyleNameKey },
            // SAFETY: sic
            FontNameKey::PostScript => unsafe { sys::kCTFontPostScriptNameKey },
        }
    }
}

cfty! {
    /// A font, combining a descriptor and a size, as well as any other necessary transforms to
    /// render glyphs.
    CTFont : CTFontGetTypeID
}

impl CTFont {
    /// Create a new font from a [`CTFontDescriptor`] and a size to render at.
    pub fn new_descriptor(descriptor: &CTFontDescriptor, size: f64) -> CTFont {
        // SAFETY: Provided descriptor is guaranteed valid. Any size will work. Null matrix is always
        //         allowed.
        let ptr = unsafe {
            sys::CTFontCreateWithFontDescriptor(
                descriptor.as_type_ref(),
                size as sys::CGFloat,
                ptr::null_mut(),
            )
        };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap();
        // SAFETY: If non-null, pointer from CTFontCreateWithFontDescriptor is a new, owned CTFont.
        unsafe { CTFont::new_owned(ptr) }
    }

    /// Get an attribute of this font, if present
    pub fn attr(&self, attr: FontAttribute) -> Option<CFType> {
        // SAFETY: Internal pointer and attribute string guaranteed valid.
        let ptr = unsafe { sys::CTFontCopyAttribute(self.as_type_ref(), attr.to_raw()) };
        // SAFETY: In non-null, returned name guaranteed valid and owned.
        NonNull::new(ptr.cast_mut()).map(|ptr| unsafe { CFType::new_owned(ptr) })
    }

    /// Get a name value of this font, if present
    pub fn name(&self, name: FontNameKey) -> Option<CFString> {
        // SAFETY: Internal pointer and name string guaranteed valid.
        let ptr = unsafe { sys::CTFontCopyName(self.as_type_ref(), name.to_raw()) };
        // SAFETY: In non-null, returned name guaranteed valid and owned.
        NonNull::new(ptr.cast_mut()).map(|ptr| unsafe { CFString::new_owned(ptr) })
    }

    /// Get the name of this font, localized to a specific language
    pub fn localized_name(&self, name: FontNameKey) -> Option<CFString> {
        // SAFETY: Internal pointer and name string guaranteed valid.
        let ptr = unsafe {
            sys::CTFontCopyLocalizedName(self.as_type_ref(), name.to_raw(), ptr::null_mut())
        };
        // SAFETY: In non-null, returned name guaranteed valid and owned.
        NonNull::new(ptr.cast_mut()).map(|ptr| unsafe { CFString::new_owned(ptr) })
    }
}
