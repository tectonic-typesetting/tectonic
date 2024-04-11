use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;
use std::ops::Index;
use std::ptr::NonNull;

pub mod sys;

#[macro_use]
mod macros;
mod array;
mod dict;
mod font;
mod font_desc;
mod set;
mod string;
mod url;

pub use array::CFArray;
pub use dict::CFDictionary;
pub use font::{CTFont, FontAttribute, FontNameKey};
pub use font_desc::CTFontDescriptor;
pub use set::CFSet;
pub use string::CFString;
pub use url::CFUrl;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CFTypeId(libc::c_ulong);

impl CFTypeId {
    pub fn of<T: CoreType>() -> CFTypeId {
        T::type_id()
    }

    pub fn of_val<T: CoreType>(val: &T) -> CFTypeId {
        CFTypeId(unsafe { sys::CFGetTypeID(val.as_type_ref().cast()) })
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct CFType(NonNull<()>);

impl CFType {
    pub fn downcast<T: CoreType>(self) -> Result<T, Self> {
        if CFTypeId::of_val(&self) == CFTypeId::of::<T>() {
            let this = ManuallyDrop::new(self);
            Ok(T::new_owned(this.0.cast()))
        } else {
            Err(self)
        }
    }
}

unsafe impl CoreType for CFType {
    type SysTy = ();

    fn type_id() -> CFTypeId {
        CFTypeId(0)
    }

    fn new_owned(ptr: NonNull<Self::SysTy>) -> Self {
        CFType(ptr)
    }

    fn new_borrowed(ptr: NonNull<Self::SysTy>) -> Self {
        unsafe { sys::CFRetain(ptr.as_ptr()) };
        CFType::new_owned(ptr)
    }

    fn as_type_ref(&self) -> *const Self::SysTy {
        self.0.as_ptr().cast_const()
    }

    fn into_ty(self) -> CFType {
        self
    }
}

impl Drop for CFType {
    fn drop(&mut self) {
        unsafe { sys::CFRelease(self.0.as_ptr()) }
    }
}

/// # Safety
///
/// Types that implement this trait must be equivalent to a CFTypeRef - they must be pointer-sized
/// and have an equivalent repr.
pub unsafe trait CoreType: Sized {
    type SysTy;

    fn type_id() -> CFTypeId;

    fn new_owned(ptr: NonNull<Self::SysTy>) -> Self;

    fn new_borrowed(ptr: NonNull<Self::SysTy>) -> Self;

    fn as_type_ref(&self) -> *const Self::SysTy;

    fn into_ty(self) -> CFType;
}
