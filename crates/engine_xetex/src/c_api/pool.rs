use std::cell::RefCell;
use std::ptr;

thread_local! {
    pub static STRING_POOL: RefCell<StringPool> = const { RefCell::new(StringPool::new()) };
}

pub struct StringPool {
    str_pool: Vec<u16>,
    str_start: Vec<u32>,
    pool_ptr: usize,
    str_ptr: usize,
    pool_size: usize,
}

impl StringPool {
    const fn new() -> StringPool {
        StringPool {
            str_pool: Vec::new(),
            str_start: Vec::new(),
            pool_ptr: 0,
            str_ptr: 0,
            pool_size: 0,
        }
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
