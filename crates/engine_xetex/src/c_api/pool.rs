use crate::ty::StrNumber;
use std::cell::RefCell;
use std::ops::Range;
use std::ptr;

pub const TOO_BIG_CHAR: usize = 65536;
pub const EMPTY_STRING: StrNumber = 65536 + 1;

thread_local! {
    pub static STRING_POOL: RefCell<StringPool> = const { RefCell::new(StringPool::new()) };
}

pub struct StringPool {
    pub(crate) str_pool: Vec<u16>,
    pub(crate) str_start: Vec<u32>,
    pub(crate) pool_ptr: usize,
    pub(crate) str_ptr: usize,
    pub(crate) pool_size: usize,
    pub(crate) max_strings: usize,
}

impl StringPool {
    const fn new() -> StringPool {
        StringPool {
            str_pool: Vec::new(),
            str_start: Vec::new(),
            pool_ptr: 0,
            str_ptr: 0,
            pool_size: 0,
            max_strings: 565536,
        }
    }

    pub fn str(&self, str: StrNumber) -> &[u16] {
        let str = str as usize;
        &self.str_pool[self.str_start[str] as usize..self.str_start[str + 1] as usize]
    }

    pub fn char_at(&self, idx: usize) -> u16 {
        self.str_pool[idx]
    }

    pub fn str_range(&self, str: StrNumber) -> Range<usize> {
        let str = str as usize;
        self.str_start[str] as usize..self.str_start[str + 1] as usize
    }
}

#[no_mangle]
pub extern "C" fn resize_str_pool(size: usize) {
    STRING_POOL.with_borrow_mut(|strings| strings.str_pool.resize(size, 0))
}

#[no_mangle]
pub extern "C" fn clear_str_pool() {
    STRING_POOL.with_borrow_mut(|strings| strings.str_pool.clear());
}

#[no_mangle]
pub extern "C" fn str_pool(idx: usize) -> u16 {
    STRING_POOL.with_borrow(|strings| strings.str_pool[idx])
}

#[no_mangle]
pub extern "C" fn str_pool_ptr(idx: usize) -> *mut u16 {
    STRING_POOL.with_borrow_mut(|strings| ptr::from_mut(&mut strings.str_pool[idx..]).cast())
}

#[no_mangle]
pub extern "C" fn set_str_pool(idx: usize, val: u16) {
    STRING_POOL.with_borrow_mut(|strings| strings.str_pool[idx] = val)
}

#[no_mangle]
pub extern "C" fn str_start(idx: usize) -> u32 {
    STRING_POOL.with_borrow(|strings| strings.str_start[idx])
}

#[no_mangle]
pub extern "C" fn str_start_ptr(idx: usize) -> *mut u32 {
    STRING_POOL.with_borrow_mut(|strings| ptr::from_mut(&mut strings.str_start[idx..]).cast())
}

#[no_mangle]
pub extern "C" fn resize_str_start(size: usize) {
    STRING_POOL.with_borrow_mut(|strings| strings.str_start.resize(size, 0))
}

#[no_mangle]
pub extern "C" fn clear_str_start() {
    STRING_POOL.with_borrow_mut(|strings| strings.str_start.clear());
}

#[no_mangle]
pub extern "C" fn set_str_start(idx: usize, val: u32) {
    STRING_POOL.with_borrow_mut(|strings| strings.str_start[idx] = val)
}

#[no_mangle]
pub extern "C" fn pool_ptr() -> usize {
    STRING_POOL.with_borrow(|strings| strings.pool_ptr)
}

#[no_mangle]
pub extern "C" fn set_pool_ptr(val: usize) {
    STRING_POOL.with_borrow_mut(|strings| strings.pool_ptr = val)
}

#[no_mangle]
pub extern "C" fn str_ptr() -> usize {
    STRING_POOL.with_borrow(|strings| strings.str_ptr)
}

#[no_mangle]
pub extern "C" fn set_str_ptr(val: usize) {
    STRING_POOL.with_borrow_mut(|strings| strings.str_ptr = val)
}

#[no_mangle]
pub extern "C" fn pool_size() -> usize {
    STRING_POOL.with_borrow(|strings| strings.pool_size)
}

#[no_mangle]
pub extern "C" fn set_pool_size(val: usize) {
    STRING_POOL.with_borrow_mut(|strings| strings.pool_size = val)
}

#[no_mangle]
pub extern "C" fn max_strings() -> usize {
    STRING_POOL.with_borrow(|strings| strings.max_strings)
}

#[no_mangle]
pub extern "C" fn set_max_strings(val: usize) {
    STRING_POOL.with_borrow_mut(|strings| strings.max_strings = val)
}

pub fn rs_make_string(pool: &mut StringPool) -> StrNumber {
    if pool.str_ptr == pool.max_strings {
        todo!("overflow(\"number of strings\", max_strings() - init_str_ptr);");
    }

    pool.str_ptr += 1;
    pool.str_start[pool.str_ptr - TOO_BIG_CHAR] = pool.pool_ptr as u32;
    (pool.str_ptr - 1) as StrNumber
}

#[no_mangle]
pub extern "C" fn make_string() -> StrNumber {
    STRING_POOL.with_borrow_mut(rs_make_string)
}
