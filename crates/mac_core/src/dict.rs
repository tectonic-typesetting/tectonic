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
        let mut keys: [MaybeUninit<K>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut values: [MaybeUninit<V>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for (idx, (key, val)) in self.into_iter().enumerate() {
            keys[idx].write(key);
            values[idx].write(val);
        }

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
    CFDictionary<K, V> : CFDictionaryGetTypeID
}

impl<K: CoreType, V: CoreType> CFDictionary<K, V> {
    pub fn new<P: Pairs>(pairs: P) -> CFDictionary<K, V> {
        let (keys, values) = pairs.into_pairs();

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
        CFDictionary::new_owned(NonNull::new(ptr.cast_mut()).unwrap())
    }
}
