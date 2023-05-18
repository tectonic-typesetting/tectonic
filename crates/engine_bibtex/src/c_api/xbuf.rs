use crate::c_api::{xcalloc, xrealloc};
use std::ops::{Deref, DerefMut};
use std::{mem, ptr, slice};

pub unsafe fn xcalloc_zeroed<T>(len: usize) -> &'static mut [T] {
    let ptr = xcalloc(len, mem::size_of::<T>());
    ptr::write_bytes(ptr, 0, len * mem::size_of::<T>());
    slice::from_raw_parts_mut(ptr.cast(), len)
}

pub unsafe fn xrealloc_zeroed<T>(old: &'static mut [T], new_len: usize) -> &'static mut [T] {
    debug_assert!(new_len >= old.len());
    let old_len = old.len();
    let old_size = old_len * mem::size_of::<T>();
    let new_size = new_len * mem::size_of::<T>();
    let ptr = xrealloc((old as *mut [_]).cast(), new_size).cast::<T>();
    ptr::write_bytes(ptr.add(old_len), 0, new_size - old_size);
    slice::from_raw_parts_mut(ptr.cast(), new_len)
}

#[derive(Debug)]
pub(crate) struct XBuf<T: Copy + 'static>(&'static mut [T]);

impl<T: Copy + 'static> XBuf<T> {
    pub fn new(init_len: usize) -> XBuf<T> {
        // TODO: Only sound for T that can be zero
        XBuf(unsafe { xcalloc_zeroed(init_len + 1) })
    }

    pub fn grow(&mut self, grow_by: usize) {
        let slice = mem::replace(&mut self.0, &mut []);
        let old_len = slice.len();
        self.0 = unsafe { xrealloc_zeroed(slice, grow_by + old_len) };
    }
}

impl<T: Copy + 'static> Deref for XBuf<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Copy + 'static> DerefMut for XBuf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<T: Copy + 'static> Drop for XBuf<T> {
    fn drop(&mut self) {
        unsafe { libc::free((self.0 as *mut [_]).cast()) };
    }
}
