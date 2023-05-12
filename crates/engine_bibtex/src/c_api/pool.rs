use crate::c_api::{xcalloc_zeroed, ASCIICode, PoolPointer, StrNumber};
use std::cell::RefCell;
use std::mem;

const POOL_SIZE: usize = 65000;
const MAX_STRINGS: usize = 35307;

pub struct StringPool {
    strings: &'static mut [u8],
    // Stores string starting locations in the string pool
    // length of string `s` is offsets[s + 1] - offsets[s]
    offsets: &'static mut [usize],
    ptr: usize,
}

impl StringPool {
    fn new() -> StringPool {
        StringPool {
            strings: unsafe { xcalloc_zeroed(POOL_SIZE, mem::size_of::<ASCIICode>()) },
            offsets: unsafe { xcalloc_zeroed(MAX_STRINGS, mem::size_of::<usize>()) },
            ptr: 0,
        }
    }

    pub fn try_get_str(&self, s: usize) -> Option<&[u8]> {
        if s >= self.ptr + 3 || s >= MAX_STRINGS {
            None
        } else {
            Some(&self.strings[self.offsets[s]..self.offsets[s + 1]])
        }
    }

    pub fn get_str(&self, s: usize) -> &[u8] {
        self.try_get_str(s)
            .expect(&format!("Invalid string number {}", s))
    }

    pub fn grow(&mut self) {
        // TODO: xrealloc_zeroed
        let new_strings =
            unsafe { xcalloc_zeroed(self.strings.len() + POOL_SIZE, mem::size_of::<ASCIICode>()) };
        new_strings.copy_from_slice(&self.strings);
        self.strings = new_strings;
    }
}

impl Drop for StringPool {
    fn drop(&mut self) {
        unsafe { libc::free((self.strings as *mut [_]).cast()) };
        unsafe { libc::free((self.offsets as *mut [_]).cast()) };
    }
}

thread_local! {
    static STRING_POOL: RefCell<StringPool> = RefCell::new(StringPool::new());
}

pub(crate) fn reset() {
    STRING_POOL.with(|pool| *pool.borrow_mut() = StringPool::new());
}

pub fn with_pool<T>(f: impl FnOnce(&StringPool) -> T) -> T {
    STRING_POOL.with(|pool| f(&pool.borrow()))
}

pub fn with_pool_mut<T>(f: impl FnOnce(&mut StringPool) -> T) -> T {
    STRING_POOL.with(|pool| f(&mut pool.borrow_mut()))
}

#[no_mangle]
pub unsafe extern "C" fn str_ends_with(s: StrNumber, ext: StrNumber) -> bool {
    with_pool(|pool| {
        let str = pool.get_str(s as usize);
        let ext = pool.get_str(ext as usize);
        str.ends_with(ext)
    })
}

#[no_mangle]
pub unsafe extern "C" fn bib_str_eq_str(s1: StrNumber, s2: StrNumber) -> bool {
    with_pool(|pool| pool.get_str(s1 as usize) == pool.get_str(s2 as usize))
}

#[no_mangle]
pub unsafe extern "C" fn pool_overflow() {
    with_pool_mut(|pool| pool.grow());
}

#[no_mangle]
pub unsafe extern "C" fn bib_str_pool(idx: PoolPointer) -> ASCIICode {
    with_pool(|pool| pool.strings[idx])
}

#[no_mangle]
pub unsafe extern "C" fn bib_set_str_pool(idx: PoolPointer, code: ASCIICode) {
    with_pool_mut(|pool| pool.strings[idx] = code)
}

#[no_mangle]
pub unsafe extern "C" fn bib_str_ptr() -> PoolPointer {
    with_pool(|pool| pool.ptr)
}

#[no_mangle]
pub unsafe extern "C" fn bib_set_str_ptr(ptr: PoolPointer) {
    with_pool_mut(|pool| pool.ptr = ptr);
}

#[no_mangle]
pub unsafe extern "C" fn bib_str_start(s: StrNumber) -> PoolPointer {
    with_pool(|pool| pool.offsets[s as usize])
}

#[no_mangle]
pub unsafe extern "C" fn bib_set_str_start(s: StrNumber, ptr: PoolPointer) {
    with_pool_mut(|pool| pool.offsets[s as usize] = ptr)
}

#[no_mangle]
pub unsafe extern "C" fn bib_pool_size() -> i32 {
    with_pool(|pool| pool.strings.len() as i32)
}
