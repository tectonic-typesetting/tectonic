use crate::{pool, HashPointer, StrIlk, StrNumber};

pub(crate) const HASH_BASE: usize = 1;
pub(crate) const HASH_SIZE: usize = if pool::MAX_STRINGS > 5000 {
    pool::MAX_STRINGS
} else {
    5000
};
const HASH_MAX: usize = HASH_SIZE + HASH_BASE - 1;
pub(crate) const HASH_PRIME: usize = compute_hash_prime();

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

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum FnClass {
    Builtin,
    Wizard,
    IntLit,
    StrLit,
    Field,
    IntEntryVar,
    StrEntryVar,
    IntGlblVar,
    StrGlblVar,
}

// TODO: Split string-pool stuff into string pool, executor stuff into execution context
pub(crate) struct HashData {
    hash_next: Vec<HashPointer>,
    hash_text: Vec<StrNumber>,
    hash_ilk: Vec<StrIlk>,
    ilk_info: Vec<i32>,
    fn_type: Vec<FnClass>,
    hash_used: usize,
}

impl HashData {
    pub(crate) fn new() -> HashData {
        HashData {
            hash_next: vec![0; HASH_MAX],
            hash_text: vec![0; HASH_MAX],
            hash_ilk: vec![StrIlk::Text; HASH_MAX],
            ilk_info: vec![0; HASH_MAX],
            fn_type: vec![FnClass::Builtin; HASH_MAX],
            hash_used: HASH_MAX + 1,
        }
    }

    pub fn undefined() -> usize {
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
