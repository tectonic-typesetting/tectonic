//! Bindings to macOS foundation and core libraries.
//!
//! This currently includes only Foundation and CoreText, as those are the libraries needed by
//! Tectonic.

#![cfg(target_os = "macos")]

use std::mem::ManuallyDrop;
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

/// An ID representing different [`CFType`] subclasses (and of course, [`CFType`]. itself). Can
/// be used for dynamic upcasting and downcasting of values.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CFTypeId(libc::c_ulong);

impl CFTypeId {
    /// Get the [`CFTypeId`] that represents an instance of a [`CoreType`].
    pub fn of<T: CoreType>() -> CFTypeId {
        T::type_id()
    }

    /// Get the runtime [`CFTypeId`] of a value. This may not be the same as the type ID of T, if
    /// the value has been upcast.
    pub fn of_val<T: CoreType>(val: &T) -> CFTypeId {
        // SAFETY: T: CoreType is guaranteed to be valid CFTypeRef
        CFTypeId(unsafe { sys::CFGetTypeID(val.as_type_ref().cast()) })
    }
}

/// The base of all CoreFoundation types. This is roughly equivalent to `Object` in Java. May be
/// used in situations where a type is only dynamically known at runtime.
#[derive(Debug)]
#[repr(transparent)]
pub struct CFType(NonNull<()>);

impl CFType {
    /// Attempt to convert this type into a more specific child type. If the cast fails, `Err`
    /// contains the original [`CFType`].
    ///
    /// # Safety
    ///
    /// This method cannot check the validity of casting from `CFArray<T> -> CFType -> CFArray<U>`
    /// in general. It is an obligation of the caller to ensure the container type is preserved.
    pub unsafe fn downcast<T: CoreType>(self) -> Result<T, Self> {
        if CFTypeId::of_val(&self) == CFTypeId::of::<T>() {
            let this = ManuallyDrop::new(self);
            // SAFETY: If the CFTypeId of self matches T, then it is guaranteed a valid pointer to T.
            //         ManuallyDrop above means we are giving up ownership of the pointer to the new T.
            Ok(unsafe { T::new_owned(this.0.cast()) })
        } else {
            Err(self)
        }
    }
}

// SAFETY: This type is the canonical CFTypeRef
unsafe impl CoreType for CFType {
    type SysTy = ();

    fn type_id() -> CFTypeId {
        CFTypeId(0)
    }

    unsafe fn new_owned(ptr: NonNull<Self::SysTy>) -> Self {
        CFType(ptr)
    }

    unsafe fn new_borrowed(ptr: NonNull<Self::SysTy>) -> Self {
        // SAFETY: Pointer is guaranteed valid by precondition
        unsafe { sys::CFRetain(ptr.as_ptr()) };
        // SAFETY: We have just retained the pointer, meaning we now have ownership of it
        unsafe { CFType::new_owned(ptr) }
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
        // SAFETY: Inner pointer is guaranteed valid
        unsafe { sys::CFRelease(self.0.as_ptr()) }
    }
}

/// # Safety
///
/// Types that implement this trait must be equivalent to a CFTypeRef - they must be pointer-sized
/// and have an equivalent repr.
pub unsafe trait CoreType: Sized {
    /// The system type for this value. This is a pointer type in the `sys` module generally.
    type SysTy;

    /// The [`CFTypeId`] that this type represents. This is insufficient to ensure equality - for
    /// example, [`CFArray<A>`]` == `[`CFArray<B>`]
    fn type_id() -> CFTypeId;

    /// # Safety
    ///
    /// - The provided pointer must be a valid pointer to an instance of this type.
    /// - The pointer must be 'owned' - valid to call `CFRelease` on without a paired `CFRetain`.
    unsafe fn new_owned(ptr: NonNull<Self::SysTy>) -> Self;

    /// # Safety
    ///
    /// - The provided pointer must be a valid pointer to an instance of this type.
    unsafe fn new_borrowed(ptr: NonNull<Self::SysTy>) -> Self;

    #[doc(hidden)]
    fn as_type_ref(&self) -> *const Self::SysTy;

    #[doc(hidden)]
    fn into_type_ref(self) -> *const Self::SysTy {
        let this = ManuallyDrop::new(self);
        this.as_type_ref()
    }

    /// Upcast this value into a [`CFType`] for usage in generic contexts.
    fn into_ty(self) -> CFType;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_id() {
        let arr = CFArray::<CFType>::empty();
        assert_eq!(CFTypeId::of::<CFArray<CFType>>(), CFTypeId::of_val(&arr));
        assert_eq!(
            CFTypeId::of::<CFArray<CFString>>(),
            CFTypeId::of::<CFArray<CFUrl>>()
        );
        assert_ne!(CFTypeId::of::<CFString>(), CFTypeId::of::<CFUrl>());
    }

    #[test]
    fn test_upcast() {
        let arr = CFArray::<CFType>::empty();
        let ty = arr.into_ty();

        assert_eq!(CFTypeId::of_val(&ty), CFTypeId::of::<CFArray<CFType>>());
        drop(ty);
    }
}
