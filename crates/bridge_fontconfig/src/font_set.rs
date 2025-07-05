use crate::{sys, ObjectSet, ObjectSetRef, Pattern, PatternRef};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

/// A borrowed reference to a [`FontSet`]
#[derive(Copy, Clone)]
pub struct FontSetRef<'a>(NonNull<sys::FcFontSet>, PhantomData<&'a sys::FcFontSet>);

impl<'a> FontSetRef<'a> {
    /// Convert into a raw pointer to the inner [`sys::FcFontSet`] struct
    fn as_ptr(self) -> *mut sys::FcFontSet {
        self.0.as_ptr()
    }

    /// Get a list of all fonts referenced by this [`FontSet`]
    pub fn fonts(self) -> &'a [PatternRef<'a>] {
        // SAFETY: Internal pointer guaranteed valid
        let ptr = unsafe { (*self.as_ptr()).fonts.cast() };
        // SAFETY: Internal pointer guaranteed valid
        let len = unsafe { (*self.as_ptr()).nfont } as usize;
        // SAFETY: Fonts pointer guaranteed to be to a valid array of length nfont
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

/// An owned font set. This is a list of [`Pattern`]s that identify unique fonts, with properties
/// loaded based on an [`ObjectSet`]
pub struct FontSet(NonNull<sys::FcFontSet>);

impl FontSet {
    /// Get all available fonts
    pub fn all() -> FontSet {
        FontSet::new(
            Pattern::from_name(c"").unwrap().as_ref(),
            ObjectSet::all().as_ref(),
        )
    }

    /// Load all fonts matching the provided pattern, getting the properties specified in the object
    /// set.
    pub fn new(pattern: PatternRef<'_>, objects: ObjectSetRef<'_>) -> FontSet {
        super::init();
        // SAFETY: Pattern and object font reference pointers guaranteed valid
        let ptr = unsafe {
            sys::FcFontList(
                sys::FcConfigGetCurrent(),
                pattern.as_ptr(),
                objects.as_ptr(),
            )
        };
        FontSet(NonNull::new(ptr).unwrap())
    }

    /// Convert into a borrowed reference.
    pub fn as_ref(&self) -> FontSetRef<'_> {
        FontSetRef(self.0, PhantomData)
    }
}

impl Drop for FontSet {
    fn drop(&mut self) {
        // SAFETY: Internal pointer guaranteed valid, we own the pointer
        unsafe { sys::FcFontSetDestroy(self.0.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_set() {
        let all = FontSet::all();
        let fs = all.as_ref();

        assert!(!fs.fonts().is_empty());
    }
}
