use std::ops::ControlFlow;
use crate::{
    auxi::AuxCommand,
    bibs::BibCommand,
    bst::BstCommand,
    exec::ControlSeq,
    pool,
    pool::{StrNumber, StringPool},
    CiteNumber, LookupRes, StrIlk,
};
use slotmap::{KeyData, SlotMap};

pub(crate) const HASH_SIZE: usize = if pool::MAX_STRINGS > 5000 {
    pool::MAX_STRINGS
} else {
    5000
};
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum BstBuiltin {
    Eq,
    Gt,
    Lt,
    Plus,
    Minus,
    Concat,
    Set,
    AddPeriod,
    CallType,
    ChangeCase,
    ChrToInt,
    Cite,
    Duplicate,
    Empty,
    FormatName,
    If,
    IntToChr,
    IntToStr,
    Missing,
    Newline,
    NumNames,
    Pop,
    Preamble,
    Purify,
    Quote,
    Skip,
    Stack,
    Substring,
    Swap,
    TextLength,
    TextPrefix,
    Top,
    Type,
    Warning,
    While,
    Width,
    Write,
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum BstFn {
    Builtin(BstBuiltin),
    Wizard(usize),
    Field(usize),
    IntEntry(usize),
    StrEntry(usize),
    IntGlbl(i64),
    StrGlbl(usize),
}

#[derive(Clone, Default, Debug)]
pub enum HashExtra {
    #[default]
    Text,
    Integer(i64),
    AuxCommand(AuxCommand),
    AuxFile,
    BstCommand(BstCommand),
    BstFile,
    BibFile,
    FileExt,
    Cite(CiteNumber),
    LcCite(HashPointer),
    BstFn(BstFn),
    BibCommand(BibCommand),
    Macro(StrNumber),
    ControlSeq(ControlSeq),
}

impl HashExtra {
    pub(crate) fn kind(&self) -> StrIlk {
        match self {
            HashExtra::Text => StrIlk::Text,
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
}

#[derive(Debug)]
pub struct Node {
    text: StrNumber,
    extra: HashExtra,
}

impl Node {
    pub fn text(&self) -> StrNumber {
        self.text
    }

    pub fn extra(&self) -> &HashExtra {
        &self.extra
    }

    pub fn extra_mut(&mut self) -> &mut HashExtra {
        &mut self.extra
    }

    pub fn kind(&self) -> StrIlk {
        self.extra.kind()
    }
}

slotmap::new_key_type! {
    pub struct HashPointer;
}

pub(crate) struct HashData {
    data: SlotMap<HashPointer, Node>,
}

impl HashData {
    pub fn new() -> HashData {
        HashData {
            data: SlotMap::with_key(),
        }
    }

    pub fn undefined() -> HashPointer {
        HashPointer::from(KeyData::from_ffi(0x0000_0001_FFFF_FFFE))
    }

    pub fn get(&self, pos: HashPointer) -> &Node {
        &self.data[pos]
    }

    pub fn get_mut(&mut self, pos: HashPointer) -> &mut Node {
        &mut self.data[pos]
    }

    pub fn lookup_str(&self, pool: &StringPool, str: &[u8], ilk: StrIlk) -> Option<HashPointer> {
        self.data.iter()
            .find_map(|(key, value)| {
                if pool.get_str(value.text) == str && value.kind() == ilk {
                    Some(key)
                } else {
                    None
                }
            })
    }

    /// Lookup a string, inserting it if it isn't found. Note that this returns `Ok` whether the
    /// string is found or not, only returning `Err` if a called function fails.
    pub fn lookup_str_insert(
        &mut self,
        pool: &mut StringPool,
        str: &[u8],
        ilk: HashExtra,
    ) -> LookupRes {
        let kind = ilk.kind();

        enum Found {
            Found(HashPointer),
            Str(StrNumber),
            NotFound,
        }
        let (ControlFlow::Break(found) | ControlFlow::Continue(found)) = self.data.iter()
            .try_fold(Found::NotFound, |acc, (key, value)| {
                if pool.get_str(value.text) == str {
                    if value.kind() == kind {
                        ControlFlow::Break(Found::Found(key))
                    } else {
                        ControlFlow::Continue(Found::Str(value.text))
                    }
                } else {
                    ControlFlow::Continue(acc)
                }
            });
        match found {
            Found::Found(loc) => {
                LookupRes { loc, exists: true }
            }
            Found::Str(text) => {
                let loc = self.data.insert(Node { extra: ilk, text });
                LookupRes { loc, exists: false }
            }
            Found::NotFound => {
                let text = pool.add_string(str);
                let loc = self.data.insert(Node { extra: ilk, text });
                LookupRes { loc, exists: false }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pool::StringPool;

    #[test]
    fn test_lookup_str() {
        let mut hash = HashData::new();
        let mut pool = StringPool::new();
        let res = hash
            .lookup_str_insert(&mut pool, b"a cool string", HashExtra::Text);
        assert!(!res.exists);
        assert_eq!(
            pool.try_get_str(hash.get(res.loc).text()),
            Some(b"a cool string" as &[_])
        );

        let res2 = hash
            .lookup_str_insert(&mut pool, b"a cool string", HashExtra::Text);
        assert!(res2.exists);
        assert_eq!(
            pool.try_get_str(hash.get(res2.loc).text()),
            Some(b"a cool string" as &[_])
        );

        let res3 = hash.lookup_str(&pool, b"a cool string", StrIlk::Text)
            .unwrap();
        assert_eq!(
            pool.try_get_str(hash.get(res3).text()),
            Some(b"a cool string" as &[_])
        );

        assert!(hash.lookup_str(&pool, b"a bad string", StrIlk::Text).is_none());
    }
}
