use crate::c_api::buffer::{with_buffers, BufTy, GlobalBuffer};
use crate::c_api::hash::{with_hash_mut, HashData};
use crate::c_api::log::{print_overflow, write_logs};
use crate::c_api::xbuf::XBuf;
use crate::c_api::{
    hash, ASCIICode, BufPointer, CResultLookup, CResultStr, HashPointer, LookupRes, PoolPointer,
    StrIlk, StrNumber,
};
use std::cell::RefCell;

const POOL_SIZE: usize = 65000;
pub(crate) const MAX_STRINGS: usize = 35307;

pub struct StringPool {
    strings: XBuf<u8>,
    // Stores string starting locations in the string pool
    // length of string `s` is offsets[s + 1] - offsets[s]
    offsets: XBuf<usize>,
    pool_ptr: PoolPointer,
    str_ptr: StrNumber,
}

impl StringPool {
    fn new() -> StringPool {
        StringPool {
            strings: XBuf::new(POOL_SIZE),
            offsets: XBuf::new(MAX_STRINGS),
            pool_ptr: 0,
            str_ptr: 0,
        }
    }

    pub fn try_get_str(&self, s: usize) -> Option<&[u8]> {
        if s >= self.str_ptr as usize + 3 || s >= MAX_STRINGS {
            None
        } else {
            Some(&self.strings[self.offsets[s]..self.offsets[s + 1]])
        }
    }

    pub fn get_str(&self, s: usize) -> &[u8] {
        self.try_get_str(s)
            .unwrap_or_else(|| panic!("Invalid string number {}", s))
    }

    pub fn grow(&mut self) {
        self.strings.grow(POOL_SIZE);
    }

    /// Used while defining strings - declare the current `pool_ptr` as the end of the current
    /// string, increment the `str_ptr`, and return the new string's `StrNumber`
    fn make_string(&mut self) -> CResultStr {
        if self.str_ptr as usize == MAX_STRINGS {
            print_overflow();
            write_logs(&format!("number of strings {}\n", MAX_STRINGS));
            return CResultStr::Error;
        }
        self.str_ptr += 1;
        self.offsets[self.str_ptr as usize] = self.pool_ptr;
        CResultStr::Ok(self.str_ptr - 1)
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
pub extern "C" fn bib_str_eq_str(s1: StrNumber, s2: StrNumber) -> bool {
    with_pool(|pool| pool.get_str(s1 as usize) == pool.get_str(s2 as usize))
}

#[no_mangle]
pub extern "C" fn pool_overflow() {
    with_pool_mut(|pool| pool.grow());
}

#[no_mangle]
pub extern "C" fn bib_str_pool(idx: PoolPointer) -> ASCIICode {
    with_pool(|pool| pool.strings[idx])
}

#[no_mangle]
pub extern "C" fn bib_set_str_pool(idx: PoolPointer, code: ASCIICode) {
    with_pool_mut(|pool| pool.strings[idx] = code)
}

#[no_mangle]
pub extern "C" fn bib_str_ptr() -> StrNumber {
    with_pool(|pool| pool.str_ptr)
}

#[no_mangle]
pub extern "C" fn bib_set_str_ptr(ptr: StrNumber) {
    with_pool_mut(|pool| pool.str_ptr = ptr);
}

#[no_mangle]
pub extern "C" fn bib_str_start(s: StrNumber) -> PoolPointer {
    with_pool(|pool| pool.offsets[s as usize])
}

#[no_mangle]
pub extern "C" fn bib_set_str_start(s: StrNumber, ptr: PoolPointer) {
    with_pool_mut(|pool| pool.offsets[s as usize] = ptr)
}

#[no_mangle]
pub extern "C" fn bib_pool_size() -> usize {
    with_pool(|pool| pool.strings.len())
}

#[no_mangle]
pub extern "C" fn bib_max_strings() -> usize {
    MAX_STRINGS
}

#[no_mangle]
pub extern "C" fn bib_pool_ptr() -> PoolPointer {
    with_pool(|pool| pool.pool_ptr)
}

#[no_mangle]
pub extern "C" fn bib_set_pool_ptr(ptr: PoolPointer) {
    with_pool_mut(|pool| pool.pool_ptr = ptr)
}

#[no_mangle]
pub extern "C" fn bib_make_string() -> CResultStr {
    with_pool_mut(|pool| pool.make_string())
}

#[no_mangle]
pub extern "C" fn str_lookup(
    buf: BufTy,
    ptr: BufPointer,
    len: BufPointer,
    ilk: StrIlk,
    insert: bool,
) -> CResultLookup {
    let f = |buffers: &GlobalBuffer, pool: &mut StringPool, hash: &mut HashData| {
        let mut h = 0;
        let mut str_num = 0;

        let str = &buffers.buffer(buf)[ptr as usize..(ptr + len) as usize];

        let hash_prime = hash.prime() as usize;

        for &c in str {
            h += h + c as usize;
            while h >= hash_prime {
                h -= hash_prime;
            }
        }

        let mut p = (h + hash::HASH_BASE) as HashPointer;

        loop {
            let existing = hash.text(p as usize);
            if existing > 0 && pool.get_str(existing as usize) == str {
                if hash.hash_ilk(p as usize) == ilk {
                    return CResultLookup::Ok(LookupRes {
                        loc: p,
                        exists: true,
                    });
                } else {
                    str_num = existing;
                }
            }

            if hash.next(p as usize) == 0 {
                if !insert {
                    return CResultLookup::Ok(LookupRes {
                        loc: p,
                        exists: false,
                    });
                }

                if existing > 0 {
                    loop {
                        if hash.used() as usize == hash::HASH_BASE {
                            print_overflow();
                            write_logs(&format!("hash size {}\n", hash::HASH_SIZE));
                            return CResultLookup::Error;
                        }
                        hash.set_used(hash.used() - 1);

                        if hash.text(hash.used() as usize) == 0 {
                            break;
                        }
                    }
                    hash.set_next(p as usize, hash.used());
                    p = hash.used();
                }

                if str_num > 0 {
                    hash.set_text(p as usize, str_num);
                } else {
                    while pool.pool_ptr + str.len() > pool.strings.len() {
                        pool.grow();
                    }
                    pool.strings[pool.pool_ptr..pool.pool_ptr + str.len()].copy_from_slice(str);
                    pool.pool_ptr += str.len();

                    match pool.make_string() {
                        CResultStr::Ok(str) => hash.set_text(p as usize, str),
                        _ => return CResultLookup::Error,
                    }
                }

                hash.set_hash_ilk(p as usize, ilk);

                return CResultLookup::Ok(LookupRes {
                    loc: p,
                    exists: false,
                });
            }

            p = hash.next(p as usize);
        }
    };

    with_buffers(|buffers| with_pool_mut(|pool| with_hash_mut(|hash| f(buffers, pool, hash))))
}
