use super::{sys, CoreType};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use std::{mem, ptr};

pub trait AsPtr {
    fn as_ptr(&self) -> *const ();
    #[allow(clippy::len_without_is_empty)]
    fn len(&self) -> usize;
}

impl<T> AsPtr for Vec<T> {
    fn as_ptr(&self) -> *const () {
        self.as_ptr().cast()
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> AsPtr for [T; N] {
    fn as_ptr(&self) -> *const () {
        <[T]>::as_ptr(self).cast()
    }

    fn len(&self) -> usize {
        N
    }
}

pub trait Pairs {
    type Keys: AsPtr;
    type Values: AsPtr;

    fn into_pairs(self) -> (Self::Keys, Self::Values);
}

impl<K, V, const N: usize> Pairs for [(K, V); N] {
    type Keys = [K; N];
    type Values = [V; N];

    fn into_pairs(self) -> (Self::Keys, Self::Values) {
        let mut keys: [MaybeUninit<K>; N] = [const { MaybeUninit::uninit() }; N];
        let mut values: [MaybeUninit<V>; N] = [const { MaybeUninit::uninit() }; N];

        for (idx, (key, val)) in self.into_iter().enumerate() {
            keys[idx].write(key);
            values[idx].write(val);
        }

        // SAFETY: All values in both arrays have been initialized
        unsafe {
            (
                mem::transmute_copy::<_, [K; N]>(&keys),
                mem::transmute_copy::<_, [V; N]>(&values),
            )
        }
    }
}

impl<K, V> Pairs for Vec<(K, V)> {
    type Keys = Vec<K>;
    type Values = Vec<V>;

    fn into_pairs(self) -> (Self::Keys, Self::Values) {
        self.into_iter().unzip()
    }
}

cfty! {
    /// A dictionary / map of CFType keys to values, similar to [`HashMap`](std::collections::HashMap)
    CFDictionary<K, V> : CFDictionaryGetTypeID
}

impl<K: CoreType, V: CoreType> CFDictionary<K, V> {
    /// Create a new [`CFDictionary`] that contains the provided key/value pairs.
    pub fn new<P: Pairs>(pairs: P) -> CFDictionary<K, V> {
        let (keys, values) = pairs.into_pairs();

        // SAFETY: Length matches provided slices and values are `CoreType` so must be valid
        //         CFTypeRefs.
        let ptr = unsafe {
            sys::CFDictionaryCreate(
                ptr::null_mut(),
                keys.as_ptr().cast_mut().cast(),
                values.as_ptr().cast_mut().cast(),
                keys.len() as sys::CFIndex,
                &sys::kCFTypeDictionaryKeyCallBacks,
                &sys::kCFTypeDictionaryValueCallBacks,
            )
        };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap();
        // SAFETY: If non-null, pointer from CFDictionaryCreate is a new, owned CFDictionary.
        unsafe { CFDictionary::new_owned(ptr) }
    }
}
