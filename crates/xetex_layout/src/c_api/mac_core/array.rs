use super::{sys, CFType, CoreType};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Index;
use std::ptr;
use std::ptr::NonNull;

cfty! {
    CFArray<T> : CFArrayGetTypeID
}

impl<T: CoreType> CFArray<T> {
    pub fn new(values: &[T]) -> CFArray<T> {
        let ptr = unsafe {
            sys::CFArrayCreate(
                ptr::null_mut(),
                values.as_ptr().cast_mut().cast(),
                values.len() as sys::CFIndex,
                &sys::kCFTypeArrayCallBacks,
            )
        };
        unsafe { CFArray::new_owned(NonNull::new(ptr.cast_mut()).unwrap()) }
    }

    pub fn len(&self) -> usize {
        unsafe { sys::CFArrayGetCount(self.0.as_ptr()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: CoreType> Index<usize> for CFArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let ptr =
            unsafe { sys::CFArrayGetValueAtIndex(self.0.cast().as_ptr(), index as sys::CFIndex) }
                .cast::<T>();
        if ptr.is_null() {
            panic!("Index {index} out of bounds for CFArray");
        }
        unsafe { &*ptr }
    }
}
