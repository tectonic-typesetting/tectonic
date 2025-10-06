use crate::{
    auxi::AuxCommand,
    bibs::BibCommand,
    bst::BstCommand,
    exec::ControlSeq,
    pool,
    pool::{StrNumber, StringPool},
    CiteNumber, LookupRes,
};
use indexmap::{set::MutableValues, Equivalent, IndexSet};
use std::{
    fmt,
    hash::{BuildHasher, Hash, Hasher},
    marker::PhantomData,
};

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

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StrIlk {
    Text = 0x0001,
    Integer = 0x0002,
    AuxCommand = 0x0004,
    AuxFile = 0x0008,
    BstCommand = 0x0010,
    BstFile = 0x0020,
    BibFile = 0x0040,
    FileExt = 0x0080,
    Cite = 0x0100,
    LcCite = 0x0200,
    BstFn = 0x0400,
    BibCommand = 0x0800,
    Macro = 0x1000,
    ControlSeq = 0x2000,
}

pub trait Ilk {
    type Extra;
    fn ilk() -> StrIlk;
    fn get(slot: &ExtraSlot) -> &Self::Extra;

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        let _ = (slot, extra);
    }

    fn insert(slot: &mut ExtraSlot, extra: Self::Extra) {
        Self::set(slot, extra);
        slot.set_present(Self::ilk());
    }
}

pub struct Text;
impl Ilk for Text {
    type Extra = ();

    fn ilk() -> StrIlk {
        StrIlk::Text
    }

    fn get(_: &ExtraSlot) -> &Self::Extra {
        &()
    }
}

pub struct Integer;
impl Ilk for Integer {
    type Extra = i64;

    fn ilk() -> StrIlk {
        StrIlk::Integer
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.0
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.0 = extra;
    }
}

impl Ilk for AuxCommand {
    type Extra = Self;

    fn ilk() -> StrIlk {
        StrIlk::AuxCommand
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.1
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.1 = extra;
    }
}

pub struct AuxFile;
impl Ilk for AuxFile {
    type Extra = ();

    fn ilk() -> StrIlk {
        StrIlk::AuxFile
    }

    fn get(_: &ExtraSlot) -> &Self::Extra {
        &()
    }
}

impl Ilk for BstCommand {
    type Extra = Self;

    fn ilk() -> StrIlk {
        StrIlk::BstCommand
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.2
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.2 = extra;
    }
}

pub struct BibFile;
impl Ilk for BibFile {
    type Extra = ();

    fn ilk() -> StrIlk {
        StrIlk::BibFile
    }

    fn get(_: &ExtraSlot) -> &Self::Extra {
        &()
    }
}

pub struct BstFile;
impl Ilk for BstFile {
    type Extra = ();

    fn ilk() -> StrIlk {
        StrIlk::BstFile
    }

    fn get(_: &ExtraSlot) -> &Self::Extra {
        &()
    }
}

pub struct FileExt;
impl Ilk for FileExt {
    type Extra = ();

    fn ilk() -> StrIlk {
        StrIlk::FileExt
    }

    fn get(_: &ExtraSlot) -> &Self::Extra {
        &()
    }
}

pub struct Cite;
impl Ilk for Cite {
    type Extra = CiteNumber;

    fn ilk() -> StrIlk {
        StrIlk::Cite
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.3
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.3 = extra;
    }
}

pub struct LcCite;
impl Ilk for LcCite {
    type Extra = HashPointer<Cite>;

    fn ilk() -> StrIlk {
        StrIlk::LcCite
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.4
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.4 = extra;
    }
}

impl Ilk for BstFn {
    type Extra = Self;

    fn ilk() -> StrIlk {
        StrIlk::BstFn
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.5
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.5 = extra;
    }
}

impl Ilk for BibCommand {
    type Extra = BibCommand;

    fn ilk() -> StrIlk {
        StrIlk::BibCommand
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.6
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.6 = extra;
    }
}

pub struct Macro;
impl Ilk for Macro {
    type Extra = StrNumber;

    fn ilk() -> StrIlk {
        StrIlk::Macro
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.7
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.7 = extra;
    }
}

impl Ilk for ControlSeq {
    type Extra = Self;

    fn ilk() -> StrIlk {
        StrIlk::ControlSeq
    }

    fn get(slot: &ExtraSlot) -> &Self::Extra {
        &slot.data.8
    }

    fn set(slot: &mut ExtraSlot, extra: Self::Extra) {
        slot.data.8 = extra;
    }
}

pub struct ExtraSlot {
    exists: u16,
    data: (
        i64,
        AuxCommand,
        BstCommand,
        CiteNumber,
        HashPointer<Cite>,
        BstFn,
        BibCommand,
        StrNumber,
        ControlSeq,
    ),
}

impl ExtraSlot {
    fn new() -> ExtraSlot {
        ExtraSlot {
            exists: 0,
            // SAFETY: All values should be valid as zeroed
            data: unsafe { core::mem::zeroed() },
        }
    }

    fn contains(&self, ilk: StrIlk) -> bool {
        let idx = ilk as u16;
        (self.exists & idx) != 0
    }

    fn set_present(&mut self, ilk: StrIlk) {
        let idx = ilk as u16;
        self.exists |= idx;
    }
}

struct Node {
    hash_val: u64,
    text: StrNumber,
    extra: ExtraSlot,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.hash_val == other.hash_val && self.text == other.text
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_val.hash(state)
    }
}

struct FindNode<'a> {
    hash_val: u64,
    pool: &'a StringPool,
    str: &'a [u8],
}

impl Hash for FindNode<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_val.hash(state);
    }
}

impl Equivalent<Node> for FindNode<'_> {
    fn equivalent(&self, key: &Node) -> bool {
        self.hash_val == key.hash_val && self.str == self.pool.get_str(key.text)
    }
}

pub struct HashPointer<T>(usize, PhantomData<T>);

impl<T> PartialEq for HashPointer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for HashPointer<T> {}

impl<T> Default for HashPointer<T> {
    fn default() -> Self {
        HashPointer(0, PhantomData)
    }
}

impl<T> fmt::Debug for HashPointer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("HashPointer").field(&self.0).finish()
    }
}

impl<T> Clone for HashPointer<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for HashPointer<T> {}

impl<T: Ilk> HashPointer<T> {
    pub fn is_null(self) -> bool {
        self.0 == 0
    }
}

pub struct HashVal<'a, T: Ilk> {
    text: StrNumber,
    extra: &'a T::Extra,
}

impl<T: Ilk> HashVal<'_, T> {
    pub fn text(&self) -> StrNumber {
        self.text
    }

    pub fn extra(&self) -> &T::Extra {
        self.extra
    }
}

pub(crate) struct HashData {
    data: IndexSet<Node>,
}

impl HashData {
    pub fn new() -> HashData {
        HashData {
            data: IndexSet::new(),
        }
    }

    pub fn undefined() -> HashPointer<BstFn> {
        HashPointer(usize::MAX, PhantomData)
    }

    pub fn get<T: Ilk>(&self, pos: HashPointer<T>) -> HashVal<'_, T> {
        let node = self.data.get_index(pos.0).unwrap();
        HashVal {
            text: node.text,
            extra: T::get(&node.extra),
        }
    }

    pub fn set_extra<T: Ilk>(&mut self, pos: HashPointer<T>, val: T::Extra) {
        let node = self.data.get_index_mut2(pos.0).unwrap();
        T::insert(&mut node.extra, val);
    }

    pub fn lookup_str<T: Ilk>(&self, pool: &StringPool, str: &[u8]) -> Option<HashPointer<T>> {
        let hash_val = self.data.hasher().hash_one(str);
        self.data
            .get_index_of(&FindNode {
                hash_val,
                str,
                pool,
            })
            .filter(|node| self.data[*node].extra.contains(T::ilk()))
            .map(|node| HashPointer(node, PhantomData))
    }

    /// Lookup a string, inserting it if it isn't found. Note that this returns `Ok` whether the
    /// string is found or not, only returning `Err` if a called function fails.
    pub fn lookup_str_insert<T: Ilk>(
        &mut self,
        pool: &mut StringPool,
        str: &[u8],
        extra: T::Extra,
    ) -> LookupRes<T> {
        let hash_val = self.data.hasher().hash_one(str);
        match self.data.get_index_of(&FindNode {
            hash_val,
            str,
            pool,
        }) {
            Some(idx) => {
                let node = self.data.get_index_mut2(idx).unwrap();
                if node.extra.contains(T::ilk()) {
                    LookupRes {
                        exists: true,
                        loc: HashPointer(idx, PhantomData),
                    }
                } else {
                    T::insert(&mut node.extra, extra);
                    LookupRes {
                        exists: false,
                        loc: HashPointer(idx, PhantomData),
                    }
                }
            }
            None => {
                let text = pool.add_string(str);
                let mut slot = ExtraSlot::new();
                T::insert(&mut slot, extra);
                let (idx, _) = self.data.insert_full(Node {
                    hash_val,
                    text,
                    extra: slot,
                });
                LookupRes {
                    exists: false,
                    loc: HashPointer(idx, PhantomData),
                }
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
        let res = hash.lookup_str_insert::<Text>(&mut pool, b"a cool string", ());
        assert!(!res.exists);
        assert_eq!(
            pool.try_get_str(hash.get(res.loc).text()),
            Some(b"a cool string" as &[_])
        );

        let res2 = hash.lookup_str_insert::<Text>(&mut pool, b"a cool string", ());
        assert!(res2.exists);
        assert_eq!(
            pool.try_get_str(hash.get(res2.loc).text()),
            Some(b"a cool string" as &[_])
        );

        let res3 = hash.lookup_str::<Text>(&pool, b"a cool string").unwrap();
        assert_eq!(
            pool.try_get_str(hash.get(res3).text()),
            Some(b"a cool string" as &[_])
        );

        assert!(hash.lookup_str::<Text>(&pool, b"a bad string").is_none());
    }

    #[test]
    fn test_lookup_ilk() {
        let mut hash = HashData::new();
        let mut pool = StringPool::new();

        let res = hash.lookup_str_insert::<Text>(&mut pool, b"Hello World!", ());
        assert!(!res.exists);
        let res2 = hash.lookup_str_insert::<Integer>(&mut pool, b"Hello World!", 1);
        assert!(!res.exists);

        assert_eq!(res.loc.0, res2.loc.0);

        let res3 = hash.lookup_str::<Integer>(&pool, b"Hello World!");
        assert_eq!(res3, Some(res2.loc));

        let res4 = hash.lookup_str::<Cite>(&pool, b"Hello World!");
        assert_eq!(res4, None);
    }
}
