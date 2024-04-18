use crate::c_api::fc::{sys, ObjectSet, Pattern};
use std::ops::Deref;
use std::ptr::NonNull;
use std::slice;

pub struct FontSet(NonNull<sys::FcFontSet>);

impl FontSet {
    fn from_raw(ptr: *mut sys::FcFontSet) -> Option<FontSet> {
        NonNull::new(ptr).map(FontSet)
    }

    fn as_raw(&self) -> *mut sys::FcFontSet {
        self.0.as_ptr()
    }

    pub fn fonts(&self) -> &[Pattern] {
        let ptr = unsafe { (*self.as_raw()).fonts.cast() };
        let len = unsafe { (*self.as_raw()).nfont } as usize;
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

pub struct OwnFontSet(FontSet);

impl OwnFontSet {
    pub fn new(pattern: &Pattern, objects: &ObjectSet) -> OwnFontSet {
        super::init();
        let ptr = unsafe {
            sys::FcFontList(
                sys::FcConfigGetCurrent(),
                pattern.as_raw(),
                objects.as_raw(),
            )
        };
        OwnFontSet(FontSet::from_raw(ptr).unwrap())
    }
}

impl Deref for OwnFontSet {
    type Target = FontSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for OwnFontSet {
    fn drop(&mut self) {
        unsafe { sys::FcFontSetDestroy(self.0.as_raw()) }
    }
}
