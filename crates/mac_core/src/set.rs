use super::{sys, CoreType};
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

cfty! {
    /// A set of CFType values, similar to [`HashSet`](std::collections::HashSet).
    CFSet<T> : CFSetGetTypeID
}

impl<T: CoreType> CFSet<T> {
    /// Create a new [`CFSet`] containing the provided values.
    pub fn new(values: &[T]) -> CFSet<T> {
        // SAFETY: Length is same as number of elements in slice, `CoreType` bound ensures all items
        //         are valid to treat as a CFTypeRef.
        let ptr = unsafe {
            sys::CFSetCreate(
                ptr::null_mut(),
                values.as_ptr().cast_mut().cast(),
                values.len() as sys::CFIndex,
                &sys::kCFTypeSetCallBacks,
            )
        };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap();
        // SAFETY: If non-null, the pointer returned by CFSetCreate is guaranteed to be a valid CFSet.
        unsafe { CFSet::new_owned(ptr) }
    }
}
