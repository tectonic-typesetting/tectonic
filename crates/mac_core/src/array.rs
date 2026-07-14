use super::{sys, CoreType};
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

cfty! {
    /// A homogeneous array of CFType values, similar to [`Vec`].
    CFArray<T> : CFArrayGetTypeID
}

impl<T: CoreType> CFArray<T> {
    /// Create a new, empty [`CFArray`].
    pub fn empty() -> CFArray<T> {
        // SAFETY: Valid to call with all null and zero length + default callbacks
        let ptr = unsafe {
            sys::CFArrayCreate(
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                &sys::kCFTypeArrayCallBacks,
            )
        };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap();
        // SAFETY: If non-null, pointer from CFArrayCreate is a new, owned CFArray.
        unsafe { CFArray::new_owned(ptr) }
    }

    /// Create a new [`CFArray`] that contains the provided values.
    pub fn new(values: &[T]) -> CFArray<T> {
        // SAFETY: Length matches provided slice and values are `CoreType` so must be valid
        //         CFTypeRefs.
        let ptr = unsafe {
            sys::CFArrayCreate(
                ptr::null_mut(),
                values.as_ptr().cast_mut().cast(),
                values.len() as sys::CFIndex,
                &sys::kCFTypeArrayCallBacks,
            )
        };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap();
        // SAFETY: If non-null, pointer from CFCArrayCreate is a new, owned CFArray.
        unsafe { CFArray::new_owned(ptr) }
    }

    /// Get the length of this array
    pub fn len(&self) -> usize {
        // SAFETY: Internal pointer is guaranteed valid.
        unsafe { sys::CFArrayGetCount(self.0.as_ptr()) as usize }
    }

    /// Check whether this array is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a borrowed value at the given index. The returned value has its reference count
    /// incremented and will be released when dropped.
    ///
    /// `CFArrayGetValueAtIndex` returns the stored `CFTypeRef` value directly (not a pointer
    /// to it), so we must construct a new borrowed `T` from it rather than casting to `&T`.
    pub fn get(&self, index: usize) -> T {
        if index >= self.len() {
            panic!("Index {index} out of bounds for CFArray");
        }
        // SAFETY: Internal pointer is guaranteed valid. Index has been verified in-bounds.
        let value =
            unsafe { sys::CFArrayGetValueAtIndex(self.0.cast().as_ptr(), index as sys::CFIndex) };
        let ptr = NonNull::new(value.cast_mut()).unwrap();
        // SAFETY: The returned value is a valid CFTypeRef for type T.
        //         new_borrowed calls CFRetain, giving us ownership of a new reference.
        unsafe { T::new_borrowed(ptr.cast()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CFString;

    #[test]
    fn test_array_index() {
        let foo = CFString::new("foo");
        let bar = CFString::new("bar");

        let arr = CFArray::new(&[foo.clone(), bar.clone()]);

        assert_eq!(arr.get(0).as_type_ref(), foo.as_type_ref());
        assert_eq!(arr.get(1).as_type_ref(), bar.as_type_ref());
        assert_eq!(arr.get(0).as_str(), "foo");
        assert_eq!(arr.get(1).as_str(), "bar");
    }

    #[test]
    fn test_array_len() {
        let empty = CFArray::<CFString>::empty();
        let two = CFArray::new(&[CFString::new("foo"), CFString::new("bar")]);

        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);

        assert!(!two.is_empty());
        assert_eq!(two.len(), 2);
    }
}
