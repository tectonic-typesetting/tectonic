use crate::{pool, CiteNumber, HashPointer, StrIlk, StrNumber};

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

#[derive(Clone, Debug)]
pub enum HashExtra {
    Text(StrNumber),
    Integer(i32),
    AuxCommand(i32), // TODO: Make Enum
    AuxFile,
    BstCommand(i32), // TODO: Make Enum
    BstFile,
    BibFile,
    FileExt,
    Cite(CiteNumber),
    LcCite(HashPointer),
    BstFn(i32),
    BibCommand(StrNumber), // TODO: Make Enum
    Macro(StrNumber),
    ControlSeq(i32),
}

impl HashExtra {
    pub(crate) fn kind(&self) -> StrIlk {
        match self {
            HashExtra::Text(_) => StrIlk::Text,
            HashExtra::Integer(_) => StrIlk::Integer,
            HashExtra::AuxCommand(_) => StrIlk::AuxCommand,
            HashExtra::AuxFile => StrIlk::AuxFile,
            HashExtra::BstCommand(_) => StrIlk::BstCommand,
            HashExtra::BstFile => StrIlk::BstFile,
            HashExtra::BibFile => StrIlk::BibFile,
            HashExtra::FileExt => StrIlk::FileExt,
            HashExtra::Cite(_) => StrIlk::Cite,
            HashExtra::LcCite(_) => StrIlk::LcCite,
            HashExtra::BstFn(_) => StrIlk::BstFn,
            HashExtra::BibCommand(_) => StrIlk::BibCommand,
            HashExtra::Macro(_) => StrIlk::Macro,
            HashExtra::ControlSeq(_) => StrIlk::ControlSeq,
        }
    }

    fn data_i32(&self) -> i32 {
        match self {
            HashExtra::Text(s) => *s as i32,
            HashExtra::Integer(i) => *i,
            HashExtra::AuxCommand(c) => *c,
            HashExtra::AuxFile => todo!(),
            HashExtra::BstCommand(c) => *c,
            HashExtra::BstFile => todo!(),
            HashExtra::BibFile => todo!(),
            HashExtra::FileExt => todo!(),
            HashExtra::Cite(c) => *c as i32,
            HashExtra::LcCite(c) => *c as i32,
            HashExtra::BstFn(c) => *c,
            HashExtra::BibCommand(c) => *c as i32,
            HashExtra::Macro(c) => *c as i32,
            HashExtra::ControlSeq(c) => *c,
        }
    }
}

impl Default for HashExtra {
    fn default() -> Self {
        HashExtra::Text(0)
    }
}

#[derive(Clone, Default, Debug)]
pub struct HashNode {
    next: HashPointer,
    text: StrNumber,
    pub(crate) extra: HashExtra,
}

impl HashNode {
    pub(crate) fn kind(&self) -> StrIlk {
        self.extra.kind()
    }
}

// TODO: Split string-pool stuff into string pool, executor stuff into execution context
pub(crate) struct HashData {
    hash_data: Vec<HashNode>,
    fn_type: Vec<FnClass>,
    len: usize,
}

impl HashData {
    pub(crate) fn new() -> HashData {
        HashData {
            hash_data: vec![HashNode::default(); HASH_MAX + 1],
            fn_type: vec![FnClass::Builtin; HASH_MAX + 1],
            len: HASH_MAX + 1,
        }
    }

    pub fn undefined() -> usize {
        HASH_MAX + 1
    }

    pub fn end_of_def() -> usize {
        HASH_MAX + 1
    }

    pub fn node(&self, pos: usize) -> &HashNode {
        &self.hash_data[pos]
    }

    pub fn node_mut(&mut self, pos: usize) -> &mut HashNode {
        &mut self.hash_data[pos]
    }

    pub fn text(&self, pos: usize) -> StrNumber {
        self.hash_data[pos].text
    }

    pub fn set_text(&mut self, pos: usize, val: StrNumber) {
        self.hash_data[pos].text = val;
    }

    pub fn next(&self, pos: usize) -> HashPointer {
        self.hash_data[pos].next
    }

    pub fn set_next(&mut self, pos: usize, val: HashPointer) {
        self.hash_data[pos].next = val
    }

    pub fn ty(&self, pos: usize) -> FnClass {
        self.fn_type[pos]
    }

    pub fn set_ty(&mut self, pos: usize, class: FnClass) {
        self.fn_type[pos] = class;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn set_len(&mut self, val: usize) {
        self.len = val;
    }

    pub fn prime(&self) -> usize {
        HASH_PRIME
    }

    pub fn ilk_info(&self, pos: usize) -> i32 {
        self.node(pos).extra.data_i32()
    }
}
