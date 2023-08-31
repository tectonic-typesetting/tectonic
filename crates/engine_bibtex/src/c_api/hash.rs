use crate::c_api::{
    pool,
    xbuf::{SafelyZero, XBuf},
    HashPointer, StrIlk, StrNumber,
};
use std::cell::RefCell;

pub const HASH_BASE: usize = 1;
pub const HASH_SIZE: usize = if pool::MAX_STRINGS > 5000 {
    pool::MAX_STRINGS
} else {
    5000
};
const HASH_MAX: usize = HASH_SIZE + HASH_BASE - 1;
pub const HASH_PRIME: usize = compute_hash_prime();

/// Calculate a prime number for use in hashing that's at least 17/20 of `HASH_SIZE`
const fn compute_hash_prime() -> usize {
    const HASH_WANT: usize = HASH_SIZE / 20 * 17;

    let mut primes = [0; HASH_SIZE];
    let mut sieve = [0; HASH_SIZE];

    let mut j = 1;
    let mut k = 1;
    let mut hash_prime = 2;
    primes[k] = hash_prime;
    let mut o = 2;
    let mut square = 9;

    while hash_prime < HASH_WANT {
        loop {
            j += 2;
            if j == square {
                sieve[o] = j;
                j += 2;
                o += 1;
                square = primes[o] * primes[o];
            }

            let mut n = 2;
            let mut j_prime = true;
            while n < o && j_prime {
                while sieve[n] < j {
                    sieve[n] += 2 * primes[n];
                }
                if sieve[n] == j {
                    j_prime = false;
                }
                n += 1;
            }

            if j_prime {
                break;
            }
        }

        k += 1;
        hash_prime = j;
        primes[k] = hash_prime;
    }

    hash_prime
}

/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone, PartialEq)]
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

// SAFETY: The FnClass type is valid at zero as FnClass::Builtin
unsafe impl SafelyZero for FnClass {}

// TODO: Split string-pool stuff into string pool, executor stuff into execution context
pub struct HashData {
    hash_next: XBuf<HashPointer>,
    hash_text: XBuf<StrNumber>,
    hash_ilk: XBuf<StrIlk>,
    ilk_info: XBuf<i32>,
    fn_type: XBuf<FnClass>,
    hash_used: usize,
}

impl HashData {
    pub(crate) fn new() -> HashData {
        HashData {
            hash_next: XBuf::new(HASH_MAX),
            hash_text: XBuf::new(HASH_MAX),
            hash_ilk: XBuf::new(HASH_MAX),
            ilk_info: XBuf::new(HASH_MAX),
            fn_type: XBuf::new(HASH_MAX),
            hash_used: HASH_MAX + 1,
        }
    }

    #[no_mangle]
    pub extern "C" fn undefined() -> usize {
        HASH_MAX + 1
    }

    pub fn end_of_def() -> usize {
        HASH_MAX + 1
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

    pub fn set_ty(&mut self, pos: usize, class: FnClass) {
        self.fn_type[pos] = class;
    }

    pub fn used(&self) -> usize {
        self.hash_used
    }

    pub fn set_used(&mut self, val: usize) {
        self.hash_used = val;
    }

    pub fn prime(&self) -> usize {
        HASH_PRIME
    }

    pub fn hash_ilk(&self, pos: usize) -> StrIlk {
        self.hash_ilk[pos]
    }

    pub fn set_hash_ilk(&mut self, pos: usize, val: StrIlk) {
        self.hash_ilk[pos] = val;
    }

    pub fn ilk_info(&self, pos: usize) -> i32 {
        self.ilk_info[pos]
    }

    pub fn set_ilk_info(&mut self, pos: usize, info: i32) {
        self.ilk_info[pos] = info;
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
pub extern "C" fn fn_type(pos: HashPointer) -> FnClass {
    with_hash(|hash| hash.fn_type[pos])
}

#[no_mangle]
pub extern "C" fn set_fn_type(pos: HashPointer, ty: FnClass) {
    with_hash_mut(|hash| hash.fn_type[pos] = ty)
}

#[no_mangle]
pub extern "C" fn hash_text(pos: HashPointer) -> StrNumber {
    with_hash(|hash| hash.hash_text[pos])
}

#[no_mangle]
pub extern "C" fn ilk_info(pos: HashPointer) -> i32 {
    with_hash(|hash| hash.ilk_info[pos])
}

#[no_mangle]
pub extern "C" fn set_ilk_info(pos: HashPointer, val: i32) {
    with_hash_mut(|hash| hash.ilk_info[pos] = val)
}

#[no_mangle]
pub extern "C" fn hash_size() -> i32 {
    HASH_SIZE as i32
}

#[no_mangle]
pub extern "C" fn hash_prime() -> usize {
    HASH_PRIME
}
