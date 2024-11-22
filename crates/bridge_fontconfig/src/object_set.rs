use crate::{sys, Property};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct ObjectSetRef<'a>(NonNull<sys::FcObjectSet>, PhantomData<&'a sys::FcObjectSet>);

impl ObjectSetRef<'_> {
    pub fn as_ptr(&self) -> *mut sys::FcObjectSet {
        self.0.as_ptr()
    }
}

pub struct ObjectSet(NonNull<sys::FcObjectSet>);

impl ObjectSet {
    pub fn new(props: &[Property]) -> ObjectSet {
        super::init();
        // SAFETY: This is always safe to call
        let ptr = unsafe { sys::FcObjectSetCreate() };
        let ptr = NonNull::new(ptr).unwrap();
        for prop in props {
            // SAFETY: The ptr value is guaranteed valid if non-null, property `to_raw` returns a
            // valid C-string.
            unsafe { sys::FcObjectSetAdd(ptr.as_ptr(), prop.to_raw()) };
        }
        ObjectSet(ptr)
    }

    pub fn as_ref(&self) -> ObjectSetRef<'_> {
        ObjectSetRef(self.0, PhantomData)
    }
}

impl Drop for ObjectSet {
    fn drop(&mut self) {
        // SAFETY: Internal pointer guaranteed valid, we own the pointer
        unsafe { sys::FcObjectSetDestroy(self.0.as_ptr()) }
    }
}
