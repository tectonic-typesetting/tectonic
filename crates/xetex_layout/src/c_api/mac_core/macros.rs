macro_rules! cfty {
    ($sysname:ident $name:ident $(<$($phantom:ident),+>)? : $id_fn:ident) => {
        #[repr(transparent)]
        pub struct $name<$($($phantom),*)>(std::ptr::NonNull<sys::$name> $($(, PhantomData<$phantom>)*)*);

        unsafe impl <$($( $phantom: CoreType ),*)*> CoreType for $name<$($( $phantom ),*)*> {
            type SysTy = sys::$sysname;

            fn type_id() -> CFTypeId {
                CFTypeId(unsafe { sys::$id_fn() })
            }

            fn new_owned(ptr: std::ptr::NonNull<Self::SysTy>) -> Self {
                Self(ptr, $($(PhantomData::<$phantom>)*)*)
            }

            fn new_borrowed(ptr: std::ptr::NonNull<Self::SysTy>) -> Self {
                unsafe { sys::CFRetain(ptr.as_ptr().cast()) };
                Self::new_owned(ptr)
            }

            fn as_type_ref(&self) -> *const Self::SysTy {
                self.0.as_ptr().cast_const()
            }

            fn into_ty(self) -> CFType {
                let this = std::mem::ManuallyDrop::new(self);
                CFType::new_owned(this.0)
            }
        }

        impl <$($( $phantom: CoreType ),*)*> Clone for $name<$($( $phantom ),*)*> {
            fn clone(&self) -> Self {
                Self::new_borrowed(self.0)
            }
        }

        impl <$($( $phantom: CoreType ),*)*> Drop for $name<$($( $phantom ),*)*> {
            fn drop(&mut self) {
                unsafe { sys::CFRelease(self.0.as_ptr().cast()) }
            }
        }
    };
    ($name:ident $(<$($phantom:ident),+>)? : $id_fn:ident) => {
        cfty!($name $name$(<$($phantom),*>)* : $id_fn);
    };
}
