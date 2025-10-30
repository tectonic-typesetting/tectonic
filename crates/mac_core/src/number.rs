use crate::sys::CFIndex;
use crate::{sys, CoreType};
use std::ptr;

cfty! {
    /// Generic scalar numeric type - can be retrieved as specific base number types
    CFNumber : CFNumberGetTypeID
}

impl CFNumber {
    /// Retrieve this number as a specific concrete value type.
    pub fn value<T: NumValue>(&self) -> Option<T> {
        let mut out = T::default();
        let res = unsafe {
            sys::CFNumberGetValue(
                self.as_type_ref(),
                T::ty().as_raw(),
                ptr::from_mut(&mut out).cast(),
            )
        };
        if res {
            Some(out)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum NumberType {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl NumberType {
    fn as_raw(self) -> CFIndex {
        match self {
            NumberType::I8 => sys::kCFNumberSInt8Type,
            NumberType::I16 => sys::kCFNumberSInt16Type,
            NumberType::I32 => sys::kCFNumberSInt32Type,
            NumberType::I64 => sys::kCFNumberSInt64Type,
            NumberType::F32 => sys::kCFNumberFloat32Type,
            NumberType::F64 => sys::kCFNumberFloat64Type,
        }
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Types that can be the concrete value of a [`CFNumber`].
pub trait NumValue: Default + sealed::Sealed {
    #[doc(hidden)]
    fn ty() -> NumberType;
}

impl NumValue for i8 {
    fn ty() -> NumberType {
        NumberType::I8
    }
}

impl NumValue for i16 {
    fn ty() -> NumberType {
        NumberType::I16
    }
}

impl NumValue for i32 {
    fn ty() -> NumberType {
        NumberType::I32
    }
}

impl NumValue for i64 {
    fn ty() -> NumberType {
        NumberType::I64
    }
}

impl NumValue for f32 {
    fn ty() -> NumberType {
        NumberType::F32
    }
}

impl NumValue for f64 {
    fn ty() -> NumberType {
        NumberType::F64
    }
}
