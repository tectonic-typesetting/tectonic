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

    pub fn tex_str(&self, str: StrNumber) -> &[u16] {
        if str < 0x10000 {
            &[]
        } else {
            self.str(str - 0x10000)
        }
    }

    pub fn char_at(&self, idx: usize) -> u16 {
        self.str_pool[idx]
    }

    pub fn str_range(&self, str: StrNumber) -> Range<usize> {
        let str = str as usize;
        self.str_start[str] as usize..self.str_start[str + 1] as usize
    }

    /// The length of the current string in the pool
    pub fn cur_length(&self) -> usize {
        self.pool_ptr - self.str_start[self.str_ptr - TOO_BIG_CHAR] as usize
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

pub fn rs_slow_make_string(pool: &mut StringPool) -> StrNumber {
    let t = rs_make_string(pool);
    let s = rs_search_string(pool, t);

    if s > 0 {
        pool.str_ptr -= 1;
        pool.pool_ptr = pool.str_start[pool.str_ptr - TOO_BIG_CHAR] as usize;
        s
    } else {
        t
    }
}

#[no_mangle]
pub extern "C" fn make_string() -> StrNumber {
    STRING_POOL.with_borrow_mut(rs_make_string)
}

#[no_mangle]
pub extern "C" fn slow_make_string() -> StrNumber {
    STRING_POOL.with_borrow_mut(|pool| rs_slow_make_string(pool))
}

pub fn rs_str_length(pool: &StringPool, s: StrNumber) -> usize {
    if s >= 0x10000 {
        pool.str(s - 0x10000).len()
    } else if (32..127).contains(&s) {
        1
    } else if s <= 127 {
        3
    } else if s < 256 {
        4
    } else {
        8
    }
}

pub fn rs_str_eq_str(pool: &StringPool, s1: StrNumber, s2: StrNumber) -> bool {
    let s1_len = rs_str_length(pool, s1);
    let s2_len = rs_str_length(pool, s2);
    if s1_len != s2_len {
        return false;
    }

    if s1_len == 1 {
        let c1 = if s1 < 0x10000 {
            s1 as u16
        } else {
            pool.str_pool[pool.str_start[(s1 - 0x10000) as usize] as usize]
        };
        let c2 = if s2 < 0x10000 {
            s2 as u16
        } else {
            pool.str_pool[pool.str_start[(s2 - 0x10000) as usize] as usize]
        };
        c1 == c2
    } else {
        pool.str(s1 - 0x10000) == pool.str(s2 - 0x10000)
    }
}

pub fn rs_search_string(pool: &StringPool, search: StrNumber) -> StrNumber {
    let len = rs_str_length(pool, search);
    if len == 0 {
        EMPTY_STRING
    } else {
        let mut s = search - 1;
        while s > 0x10000 {
            if rs_str_eq_str(pool, s, search) {
                return s;
            }
            s -= 1;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn length(s: StrNumber) -> usize {
    STRING_POOL.with_borrow(|pool| rs_str_length(pool, s))
}

#[no_mangle]
pub extern "C" fn str_eq_str(s1: StrNumber, s2: StrNumber) -> bool {
    STRING_POOL.with_borrow(|pool| rs_str_eq_str(pool, s1, s2))
}

#[no_mangle]
pub extern "C" fn search_string(search: StrNumber) -> StrNumber {
    STRING_POOL.with_borrow(|pool| rs_search_string(pool, search))
}
