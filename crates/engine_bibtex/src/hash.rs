use crate::{auxi::AuxCommand, bibs::BibCommand, bst::BstCommand, exec::ControlSeq, pool, pool::StrNumber, ASCIICode, Bibtex, BibtexError, CiteNumber, FnDefLoc, HashPointer, LookupRes, StrIlk};
use crate::log::print_overflow;
use crate::pool::StringPool;

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
    Wizard(FnDefLoc),
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
    len: usize,
}

impl HashData {
    pub fn new() -> HashData {
        HashData {
            hash_data: vec![HashNode::default(); HASH_MAX + 1],
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

    fn set_text(&mut self, pos: usize, val: StrNumber) {
        self.hash_data[pos].text = val;
    }

    fn next(&self, pos: usize) -> HashPointer {
        self.hash_data[pos].next
    }

    fn set_next(&mut self, pos: usize, val: HashPointer) {
        self.hash_data[pos].next = val
    }

    fn len(&self) -> usize {
        self.len
    }

    fn set_len(&mut self, val: usize) {
        self.len = val;
    }

    fn prime(&self) -> usize {
        HASH_PRIME
    }

    fn hash_str(&self, str: &[ASCIICode]) -> usize {
        let prime = self.prime();
        str.iter()
            .fold(0, |acc, &c| ((2 * acc) + c as usize) % prime)
    }

    pub fn lookup_str(&self, pool: &StringPool, str: &[ASCIICode], ilk: StrIlk) -> LookupRes {
        let h = self.hash_str(str);
        let mut p = h as HashPointer + HASH_BASE as HashPointer;

        let exists = loop {
            let existing = self.text(p);

            if !existing.is_invalid() && pool.get_str(existing) == str && self.node(p).kind() == ilk {
                break true;
            }

            if self.next(p) == 0 {
                break false;
            }

            p = self.next(p);
        };

        LookupRes { loc: p, exists }
    }

    /// Lookup a string, inserting it if it isn't found. Note that this returns `Ok` whether the
    /// string is found or not, only returning `Err` if a called function fails.
    pub fn lookup_str_insert(
        &mut self,
        ctx: &mut Bibtex<'_, '_>,
        pool: &mut StringPool,
        str: &[ASCIICode],
        ilk: HashExtra,
    ) -> Result<LookupRes, BibtexError> {
        // Hash string using simple hash function. This hash is capped to HASH_PRIME
        let h = self.hash_str(str);
        let mut str_num = StrNumber::default();
        // Get position by adding HASH_BASE
        let mut p = (h + HASH_BASE) as HashPointer;

        // Look for an existing match, or the last slot
        let existing = loop {
            // Get the current text at the position
            let existing = self.text(p);
            // If the text exists and is the same as the text we're adding
            if pool.try_get_str(existing) == Some(str) {
                // If an existing hash entry exists for this type, return it
                if self.node(p).kind() == ilk.kind() {
                    return Ok(LookupRes {
                        loc: p,
                        exists: true,
                    });
                } else {
                    str_num = existing;
                }
            }

            if self.next(p) == 0 {
                break existing;
            }

            p = self.next(p);
        };

        // If we hit the end and the slot is already in use
        if !existing.is_invalid() {
            // Walk backwards from our current len to our first empty slot.
            // If all slots are full, error
            loop {
                if self.len() == HASH_BASE {
                    print_overflow(ctx);
                    ctx.write_logs(&format!("hash size {}\n", HASH_SIZE));
                    return Err(BibtexError::Fatal);
                }
                self.set_len(self.len() - 1);

                if self.text(self.len()).is_invalid() {
                    break;
                }
            }
            // Set the next item to our new lowest open slot
            self.set_next(p, self.len());
            // Operate on the new empty slot
            p = self.len();
        }

        // We found the string in the string pool while hunting for a slot
        if !str_num.is_invalid() {
            self.set_text(p, str_num);
        // The string isn't in the string pool - add it
        } else {
            self.set_text(p, pool.add_string(str));
        }

        // Set the type of this slot
        self.node_mut(p).extra = ilk;

        Ok(LookupRes {
            loc: p,
            exists: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bibtex, BibtexConfig};
    use crate::pool::StringPool;
    use crate::test_utils::with_cbs;

    #[test]
    fn test_lookup_str() {
        with_cbs(|cbs| {
            let mut ctx = Bibtex::new(cbs, BibtexConfig::default());
            let mut hash = HashData::new();
            let mut pool = StringPool::new();
            let res = hash
                .lookup_str_insert(&mut ctx, &mut pool, b"a cool string", HashExtra::Text)
                .unwrap();
            assert!(!res.exists);
            assert_eq!(
                pool.try_get_str(hash.text(res.loc)),
                Some(b"a cool string" as &[_])
            );

            let res2 = hash
                .lookup_str_insert(&mut ctx, &mut pool, b"a cool string", HashExtra::Text)
                .unwrap();
            assert!(res2.exists);
            assert_eq!(
                pool.try_get_str(hash.text(res2.loc)),
                Some(b"a cool string" as &[_])
            );

            let res3 = hash.lookup_str(&pool, b"a cool string", StrIlk::Text);
            assert!(res3.exists);
            assert_eq!(
                pool.try_get_str(hash.text(res3.loc)),
                Some(b"a cool string" as &[_])
            );

            let res4 = hash.lookup_str(&pool, b"a bad string", StrIlk::Text);
            assert!(!res4.exists);
            assert_eq!(
                pool.try_get_str(hash.text(res4.loc)),
                None,
            );
        })
    }
}
