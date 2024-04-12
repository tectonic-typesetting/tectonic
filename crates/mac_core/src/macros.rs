macro_rules! cfty {
    ($sysname:ident $name:ident $(<$($phantom:ident),+>)? : $id_fn:ident) => {
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name<$($($phantom),*)*>(std::ptr::NonNull<$crate::sys::$sysname> $($(, PhantomData<$phantom>)*)*);

        unsafe impl <$($( $phantom: $crate::CoreType ),*)*> $crate::CoreType for $name<$($( $phantom ),*)*> {
            type SysTy = $crate::sys::$sysname;

            fn type_id() -> $crate::CFTypeId {
                $crate::CFTypeId(unsafe { $crate::sys::$id_fn() })
            }

            fn new_owned(ptr: std::ptr::NonNull<Self::SysTy>) -> Self {
                Self(ptr, $($(PhantomData::<$phantom>),*)*)
            }

            fn new_borrowed(ptr: std::ptr::NonNull<Self::SysTy>) -> Self {
                unsafe { $crate::sys::CFRetain(ptr.as_ptr().cast()) };
                Self::new_owned(ptr)
            }

            fn as_type_ref(&self) -> *const Self::SysTy {
                self.0.as_ptr().cast_const()
            }

            fn into_ty(self) -> $crate::CFType {
                let this = std::mem::ManuallyDrop::new(self);
                $crate::CFType::new_owned(this.0.cast())
            }
        }

        impl <$($( $phantom: $crate::CoreType ),*)*> Clone for $name<$($( $phantom ),*)*> {
            fn clone(&self) -> Self {
                Self::new_borrowed(self.0)
            }
        }

        impl <$($( $phantom ),*)*> Drop for $name<$($( $phantom ),*)*> {
            fn drop(&mut self) {
                unsafe { $crate::sys::CFRelease(self.0.as_ptr().cast()) }
            }
        }
    };
    ($name:ident $(<$($phantom:ident),+>)? : $id_fn:ident) => {
        cfty!($name $name$(<$($phantom),*>)* : $id_fn);
    };
}
