use super::{sys, CoreType};
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

cfty! {
    CFSet<T> : CFSetGetTypeID
}

impl<T: CoreType> CFSet<T> {
    pub fn new(values: &[T]) -> CFSet<T> {
        let ptr = unsafe {
            sys::CFSetCreate(
                ptr::null_mut(),
                values.as_ptr().cast_mut().cast(),
                values.len() as sys::CFIndex,
                &sys::kCFTypeSetCallBacks,
            )
        };
        CFSet::new_owned(NonNull::new(ptr.cast_mut()).unwrap())
    }
}
