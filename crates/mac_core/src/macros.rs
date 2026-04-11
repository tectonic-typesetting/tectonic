macro_rules! cfty {
    ($(#[doc = $doc:expr])* $sysname:ident $name:ident $(<$($phantom:ident),+>)? : $id_fn:ident) => {

        $(#[doc = $doc])*
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name<$($($phantom),*)*>(std::ptr::NonNull<$crate::sys::$sysname> $($(, PhantomData<$phantom>)*)*);

        // SAFETY: Internal macro only used for CFTypeRef-safe types. The above struct definition
        //         ensures this type is just a pointer wrapper.
        unsafe impl <$($( $phantom: $crate::CoreType ),*)*> $crate::CoreType for $name<$($( $phantom ),*)*> {
            type SysTy = $crate::sys::$sysname;

            fn type_id() -> $crate::CFTypeId {
                // SAFETY: The provided identity function, assuming it type checks, should be always
                //         safe to call.
                $crate::CFTypeId(unsafe { $crate::sys::$id_fn() })
            }

            unsafe fn new_owned(ptr: std::ptr::NonNull<Self::SysTy>) -> Self {
                Self(ptr, $($(PhantomData::<$phantom>),*)*)
            }

            unsafe fn new_borrowed(ptr: std::ptr::NonNull<Self::SysTy>) -> Self {
                // SAFETY: Provided pointer is required a valid pointer for this type
                unsafe { $crate::sys::CFRetain(ptr.as_ptr().cast()) };
                Self::new_owned(ptr)
            }

            fn as_type_ref(&self) -> *const Self::SysTy {
                self.0.as_ptr().cast_const()
            }

            fn into_ty(self) -> $crate::CFType {
                let this = std::mem::ManuallyDrop::new(self);
                // SAFETY: The internal pointer is guaranteed valid
                unsafe { $crate::CFType::new_owned(this.0.cast()) }
            }
        }

        impl <$($( $phantom: $crate::CoreType ),*)*> Clone for $name<$($( $phantom ),*)*> {
            fn clone(&self) -> Self {
                // SAFETY: The internal pointer is guaranteed valid
                unsafe { Self::new_borrowed(self.0) }
            }
        }

        impl <$($( $phantom ),*)*> Drop for $name<$($( $phantom ),*)*> {
            fn drop(&mut self) {
                // SAFETY: The internal pointer is guaranteed valid and owned
                unsafe { $crate::sys::CFRelease(self.0.as_ptr().cast()) }
            }
        }
    };
    (
        $( #[doc = $doc:expr] )*
        $name:ident $(<$($phantom:ident),+>)? : $id_fn:ident) => {
        cfty!($(#[doc = $doc])* $name $name$(<$($phantom),*>)* : $id_fn);
    };
}
