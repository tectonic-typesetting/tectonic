use crate::c_api::{xcalloc, xrealloc};
use std::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
    ptr::NonNull,
    slice,
};

/// # Safety
/// The all-zero byte pattern must be a valid instance of this type
pub unsafe trait SafelyZero {}

// SAFETY: The bool zero value is false
unsafe impl SafelyZero for bool {}
// SAFETY: All integer types are sound to init as all zeroes
unsafe impl SafelyZero for u8 {}
// SAFETY: All integer types are sound to init as all zeroes
unsafe impl SafelyZero for usize {}
// SAFETY: All integer types are sound to init as all zeroes
unsafe impl SafelyZero for i32 {}
// SAFETY: All integer types are sound to init as all zeroes
unsafe impl SafelyZero for i64 {}
// SAFETY: Pointers are sound to init as all-zero, that's just null
unsafe impl<T> SafelyZero for *mut T {}
// SAFETY: Option<NonNull<T>> has the same layout as *mut T
unsafe impl<T> SafelyZero for Option<NonNull<T>> {}
// SAFETY: Option<&mut T> has the same layout as *mut T
unsafe impl<T> SafelyZero for Option<&mut T> {}

pub fn xcalloc_zeroed<T: SafelyZero>(len: usize) -> Option<&'static mut [T]> {
    if len == 0 || mem::size_of::<T>() == 0 {
        return None;
    }
    // SAFETY: We're not asking for zero size because of above check
    let ptr = unsafe { xcalloc(len, mem::size_of::<T>()) };
    if ptr.is_null() {
        None
    } else {
        // SAFETY: Allocating returns a valid pointer if non-null, it's valid for len * size_of::<T>()
        //         bytes because that's what we requested
        unsafe { ptr::write_bytes(ptr, 0, len) };
        // SAFETY: Same as above, plus `SafelyZero` means it's sound to return a reference to all-zero T
        Some(unsafe { slice::from_raw_parts_mut(ptr.cast(), len) })
    }
}

pub fn xrealloc_zeroed<T: SafelyZero>(
    old: &'static mut [T],
    new_len: usize,
) -> Option<&'static mut [T]> {
    let old_len = old.len();
    let new_size = new_len * mem::size_of::<T>();
    // SAFETY: realloc can be called with any size, even 0, that will just deallocate and return null
    let ptr = unsafe { xrealloc((old as *mut [_]).cast(), new_size) }.cast::<T>();
    if ptr.is_null() {
        None
    } else {
        if new_len > old_len {
            // SAFETY: If new_len is bigger than old_len, ptr will be at least new_len * mem::size_of::<T>()
            //         bytes, and it's safe to zero-init the trailing bytes
            unsafe { ptr::write_bytes(ptr.add(old_len), 0, new_len - old_len) };
        }
        // SAFETY: realloc guarantees `new_size` bytes valid, plus `SafelyZero` means it's sound to
        //         return a reference to all-zero T
        Some(unsafe { slice::from_raw_parts_mut(ptr.cast(), new_len) })
    }
}

#[derive(Debug)]
pub struct XBuf<T: SafelyZero + 'static>(&'static mut [T]);

impl<T: SafelyZero + 'static> XBuf<T> {
    pub fn new(init_len: usize) -> XBuf<T> {
        XBuf(xcalloc_zeroed(init_len + 1).unwrap())
    }

    pub fn grow(&mut self, grow_by: usize) {
        let slice = mem::take(&mut self.0);
        let old_len = slice.len();
        self.0 = xrealloc_zeroed(slice, grow_by + old_len).unwrap();
    }
}

impl<T: SafelyZero + 'static> Deref for XBuf<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: SafelyZero + 'static> DerefMut for XBuf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<T: SafelyZero + 'static> Drop for XBuf<T> {
    fn drop(&mut self) {
        // SAFETY: Inner pointer is guaranteed valid and not previously freed
        unsafe { libc::free((self.0 as *mut [_]).cast()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let buf = XBuf::<u8>::new(16);
        assert_eq!(buf.len(), 17);
        assert_eq!(&*buf, &[0; 17]);

        let buf2 = XBuf::<i64>::new(16);
        assert_eq!(buf2.len(), 17);
        assert_eq!(&*buf2, &[0; 17]);
    }

    #[test]
    fn test_grow() {
        let mut buf = XBuf::<usize>::new(16);
        buf.iter_mut().enumerate().for_each(|(idx, val)| *val = idx);
        buf.grow(16);
        assert_eq!(buf.len(), 33);
        let expected = &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(&*buf, expected);
    }
}
