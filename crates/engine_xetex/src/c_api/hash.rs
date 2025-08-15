use crate::c_api::engine::B32x2;
use std::cell::RefCell;
use std::ptr;

pub const HASH_OFFSET: usize = 514;
pub const HASH_BASE: usize = 0x220002;

thread_local! {
    pub static HASH_CTX: RefCell<HashCtx> = const { RefCell::new(HashCtx::new()) };
}

pub struct HashCtx {
    hash: Vec<B32x2>,
    hash_used: i32,
    hash_extra: i32,
    hash_top: i32,
}

impl HashCtx {
    const fn new() -> HashCtx {
        HashCtx {
            hash: Vec::new(),
            hash_used: 0,
            hash_extra: 0,
            hash_top: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn resize_hash(len: usize) {
    HASH_CTX.with_borrow_mut(|hash| hash.hash.resize(len, B32x2 { s0: 0, s1: 0 }));
}

#[no_mangle]
pub extern "C" fn hash(idx: usize) -> B32x2 {
    HASH_CTX.with_borrow(|hash| hash.hash[idx - HASH_BASE])
}

#[no_mangle]
pub extern "C" fn set_hash(idx: usize, val: B32x2) {
    HASH_CTX.with_borrow_mut(|hash| hash.hash[idx - HASH_BASE] = val)
}

#[no_mangle]
pub extern "C" fn hash_ptr(idx: usize) -> *mut B32x2 {
    HASH_CTX.with_borrow_mut(|hash| ptr::from_mut(&mut hash.hash[idx - HASH_BASE]))
}

#[no_mangle]
pub extern "C" fn clear_hash() {
    HASH_CTX.with_borrow_mut(|hash| hash.hash.clear());
}

#[no_mangle]
pub extern "C" fn hash_used() -> i32 {
    HASH_CTX.with_borrow(|hash| hash.hash_used)
}

#[no_mangle]
pub extern "C" fn set_hash_used(val: i32) {
    HASH_CTX.with_borrow_mut(|hash| hash.hash_used = val)
}

#[no_mangle]
pub extern "C" fn hash_extra() -> i32 {
    HASH_CTX.with_borrow(|hash| hash.hash_extra)
}

#[no_mangle]
pub extern "C" fn set_hash_extra(val: i32) {
    HASH_CTX.with_borrow_mut(|hash| hash.hash_extra = val)
}

#[no_mangle]
pub extern "C" fn hash_top() -> i32 {
    HASH_CTX.with_borrow(|hash| hash.hash_top)
}

#[no_mangle]
pub extern "C" fn set_hash_top(val: i32) {
    HASH_CTX.with_borrow_mut(|hash| hash.hash_top = val)
}
