use super::{sys, CoreType};
use std::marker::PhantomData;
use std::ops::Index;
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
}

impl<T: CoreType> Index<usize> for CFArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            panic!("Index {index} out of bounds for CFArray");
        }
        // SAFETY: Internal pointer is guaranteed valid. Index has been verified in-bounds.
        let ptr =
            unsafe { sys::CFArrayGetValueAtIndex(self.0.cast().as_ptr(), index as sys::CFIndex) }
                .cast::<T>();
        // SAFETY: API contracts ensure all values are of the correct type and live for our lifetime.
        unsafe { &*ptr }
    }
}
