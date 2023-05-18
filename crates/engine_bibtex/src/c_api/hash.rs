use crate::c_api::xbuf::XBuf;
use crate::c_api::{pool, HashPointer, StrIlk, StrNumber};
use std::cell::RefCell;

pub const HASH_BASE: usize = 1;
pub const HASH_SIZE: usize = if pool::MAX_STRINGS > 5000 {
    pool::MAX_STRINGS
} else {
    5000
};
const HASH_MAX: usize = HASH_SIZE + HASH_BASE - 1;

/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone)]
#[repr(C)]
pub enum FnClass {
    Builtin = 0,
    Wizard = 1,
    IntLit = 2,
    StrLit = 3,
    Field = 4,
    IntEntryVar = 5,
    StrEntryVar = 6,
    IntGlblVar = 7,
    StrGlblVar = 8,
}

pub struct HashData {
    hash_next: XBuf<HashPointer>,
    hash_text: XBuf<StrNumber>,
    hash_ilk: XBuf<StrIlk>,
    ilk_info: XBuf<i32>,
    fn_type: XBuf<FnClass>,
    hash_used: i32,
    hash_prime: i32,
}

impl HashData {
    fn new() -> HashData {
        HashData {
            hash_next: XBuf::new(HASH_MAX),
            hash_text: XBuf::new(HASH_MAX),
            hash_ilk: XBuf::new(HASH_MAX),
            ilk_info: XBuf::new(HASH_MAX),
            fn_type: XBuf::new(HASH_MAX),
            hash_used: HASH_MAX as i32 + 1,
            hash_prime: 0,
        }
    }

    pub fn text(&self, pos: usize) -> StrNumber {
        self.hash_text[pos]
    }

    pub fn set_text(&mut self, pos: usize, val: StrNumber) {
        self.hash_text[pos] = val;
    }

    pub fn next(&self, pos: usize) -> HashPointer {
        self.hash_next[pos]
    }

    pub fn set_next(&mut self, pos: usize, val: HashPointer) {
        self.hash_next[pos] = val
    }

    pub fn ty(&self, pos: usize) -> FnClass {
        self.fn_type[pos]
    }

    pub fn used(&self) -> i32 {
        self.hash_used
    }

    pub fn set_used(&mut self, val: i32) {
        self.hash_used = val;
    }

    pub fn prime(&self) -> i32 {
        self.hash_prime
    }

    pub fn hash_ilk(&self, pos: usize) -> StrIlk {
        self.hash_ilk[pos]
    }

    pub fn set_hash_ilk(&mut self, pos: usize, val: StrIlk) {
        self.hash_ilk[pos] = val;
    }
}

thread_local! {
    pub static HASHES: RefCell<HashData> = RefCell::new(HashData::new());
}

pub fn reset() {
    HASHES.with(|hash| *hash.borrow_mut() = HashData::new());
}

pub fn with_hash<T>(f: impl FnOnce(&HashData) -> T) -> T {
    HASHES.with(|h| f(&h.borrow()))
}

pub fn with_hash_mut<T>(f: impl FnOnce(&mut HashData) -> T) -> T {
    HASHES.with(|h| f(&mut h.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn reset_after_compute() {
    with_hash_mut(|hash| {
        hash.hash_next[1..].fill(0);
        hash.hash_text[1..].fill(0);
    })
}

#[no_mangle]
pub extern "C" fn end_of_def() -> i32 {
    HASH_MAX as i32 + 1
}

#[no_mangle]
pub extern "C" fn undefined() -> i32 {
    HASH_MAX as i32 + 1
}

#[no_mangle]
pub extern "C" fn fn_type(pos: HashPointer) -> FnClass {
    with_hash(|hash| hash.fn_type[pos as usize])
}

#[no_mangle]
pub extern "C" fn set_fn_type(pos: HashPointer, ty: FnClass) {
    with_hash_mut(|hash| hash.fn_type[pos as usize] = ty)
}

#[no_mangle]
pub extern "C" fn hash_text(pos: HashPointer) -> StrNumber {
    with_hash(|hash| hash.hash_text[pos as usize])
}

#[no_mangle]
pub extern "C" fn set_hash_text(pos: HashPointer, num: StrNumber) {
    with_hash_mut(|hash| hash.hash_text[pos as usize] = num)
}

#[no_mangle]
pub extern "C" fn ilk_info(pos: HashPointer) -> i32 {
    with_hash(|hash| hash.ilk_info[pos as usize])
}

#[no_mangle]
pub extern "C" fn set_ilk_info(pos: HashPointer, val: i32) {
    with_hash_mut(|hash| hash.ilk_info[pos as usize] = val)
}

#[no_mangle]
pub extern "C" fn hash_next(pos: HashPointer) -> HashPointer {
    with_hash(|hash| hash.hash_next[pos as usize])
}

#[no_mangle]
pub extern "C" fn set_hash_next(pos: HashPointer, val: HashPointer) {
    with_hash_mut(|hash| hash.hash_next[pos as usize] = val)
}

#[no_mangle]
pub extern "C" fn hash_size() -> i32 {
    HASH_SIZE as i32
}

#[no_mangle]
pub extern "C" fn hash_prime() -> i32 {
    with_hash(|hash| hash.hash_prime)
}

#[no_mangle]
pub extern "C" fn set_hash_prime(val: i32) {
    with_hash_mut(|hash| hash.hash_prime = val)
}
