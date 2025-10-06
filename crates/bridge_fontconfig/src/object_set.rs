use crate::{sys, Property};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// A borrowed reference to an [`ObjectSet`]
pub struct ObjectSetRef<'a>(NonNull<sys::FcObjectSet>, PhantomData<&'a sys::FcObjectSet>);

impl ObjectSetRef<'_> {
    /// Convert into a raw pointer to the inner [`sys::FcObjectSet`] struct
    pub fn as_ptr(&self) -> *mut sys::FcObjectSet {
        self.0.as_ptr()
    }
}

/// A set of strings used to represent properties to load in a [`FontSet`](crate::FontSet).
pub struct ObjectSet(NonNull<sys::FcObjectSet>);

impl ObjectSet {
    /// Create a new object set that contains the provided [Properties](Property)
    pub fn new(props: &[Property]) -> ObjectSet {
        super::init();
        // SAFETY: This is always safe to call
        let ptr = unsafe { sys::FcObjectSetCreate() };
        let ptr = NonNull::new(ptr).unwrap();
        for prop in props {
            // SAFETY: The ptr value is guaranteed valid if non-null, property `to_raw` returns a
            // valid C-string.
            unsafe { sys::FcObjectSetAdd(ptr.as_ptr(), prop.to_raw().as_ptr()) };
        }
        ObjectSet(ptr)
    }

    /// Get all [Properties](Property) as an [`ObjectSet`]
    pub fn all() -> ObjectSet {
        use Property::*;
        ObjectSet::new(&[
            Family, Style, Slant, Weight, Width, File, Index, FullName, FontFormat,
        ])
    }

    /// Convert into a borrowed reference.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_add() {
        let _ = ObjectSet::new(&[Property::File, Property::Width]).as_ref();
    }
}
