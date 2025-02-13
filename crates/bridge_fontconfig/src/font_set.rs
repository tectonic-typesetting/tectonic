use crate::{sys, ObjectSetRef, PatternRef};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::slice;

#[derive(Copy, Clone)]
pub struct FontSetRef<'a>(NonNull<sys::FcFontSet>, PhantomData<&'a sys::FcFontSet>);

impl<'a> FontSetRef<'a> {
    fn as_ptr(self) -> *mut sys::FcFontSet {
        self.0.as_ptr()
    }

    pub fn fonts(self) -> &'a [PatternRef<'a>] {
        // SAFETY: Internal pointer guaranteed valid
        let ptr = unsafe { (*self.as_ptr()).fonts.cast() };
        // SAFETY: Internal pointer guaranteed valid
        let len = unsafe { (*self.as_ptr()).nfont } as usize;
        // SAFETY: Fonts pointer guaranteed to be to a valid array of length nfont
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

pub struct FontSet(NonNull<sys::FcFontSet>);

impl FontSet {
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
