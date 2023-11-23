use crate::{
    entries::EntryData, hash::HashData, other::OtherData, pool::StringPool, xbuf::XBuf, CiteNumber,
    FindCiteLocs, HashPointer, StrIlk, StrNumber,
};
use std::{cmp::Ordering, ops::IndexMut};

pub(crate) const MAX_CITES: usize = 750;

pub(crate) struct CiteInfo {
    cite_list: XBuf<StrNumber>,
    cite_info: XBuf<StrNumber>,
    type_list: XBuf<HashPointer>,
    entry_exists: XBuf<bool>,
    cite_ptr: CiteNumber,

    entry_cite_ptr: CiteNumber,
    num_cites: CiteNumber,
    old_num_cites: CiteNumber,
    all_marker: CiteNumber,
}

impl CiteInfo {
    pub fn new() -> CiteInfo {
        CiteInfo {
            cite_list: XBuf::new(MAX_CITES),
            cite_info: XBuf::new(MAX_CITES),
            type_list: XBuf::new(MAX_CITES),
            entry_exists: XBuf::new(MAX_CITES),
            cite_ptr: 0,
            entry_cite_ptr: 0,
            num_cites: 0,
            old_num_cites: 0,
            all_marker: 0,
        }
    }

    pub fn grow(&mut self) {
        self.cite_list.grow(MAX_CITES);
        self.cite_info.grow(MAX_CITES);
        self.type_list.grow(MAX_CITES);
        self.entry_exists.grow(MAX_CITES);
    }

    pub fn get_cite(&self, offset: usize) -> StrNumber {
        self.cite_list[offset]
    }

    pub fn set_cite(&mut self, offset: usize, num: StrNumber) {
        self.cite_list[offset] = num;
    }

    pub fn info(&self, offset: usize) -> StrNumber {
        self.cite_info[offset]
    }

    pub fn set_info(&mut self, offset: usize, num: StrNumber) {
        self.cite_info[offset] = num;
    }

    pub fn get_type(&self, offset: usize) -> HashPointer {
        self.type_list[offset]
    }

    pub fn set_type(&mut self, offset: usize, ty: HashPointer) {
        self.type_list[offset] = ty;
    }

    pub fn exists(&self, offset: usize) -> bool {
        self.entry_exists[offset]
    }

    pub fn set_exists(&mut self, offset: usize, exists: bool) {
        self.entry_exists[offset] = exists;
    }

    pub fn ptr(&self) -> CiteNumber {
        self.cite_ptr
    }

    pub fn set_ptr(&mut self, ptr: CiteNumber) {
        self.cite_ptr = ptr;
    }

    pub fn entry_ptr(&self) -> CiteNumber {
        self.entry_cite_ptr
    }

    pub fn set_entry_ptr(&mut self, val: CiteNumber) {
        self.entry_cite_ptr = val;
    }

    pub fn num_cites(&self) -> CiteNumber {
        self.num_cites
    }

    pub fn set_num_cites(&mut self, num: CiteNumber) {
        self.num_cites = num;
    }

    pub fn old_num_cites(&self) -> CiteNumber {
        self.old_num_cites
    }

    pub fn set_old_num_cites(&mut self, num: CiteNumber) {
        self.old_num_cites = num;
    }

    pub fn len(&self) -> usize {
        self.cite_list.len()
    }

    pub fn all_marker(&self) -> CiteNumber {
        self.all_marker
    }

    pub fn set_all_marker(&mut self, val: CiteNumber) {
        self.all_marker = val;
    }

    pub fn sort_info<I>(&mut self, entries: &EntryData, r: I)
    where
        [usize]: IndexMut<I, Output = [usize]>,
    {
        self.cite_info[r].sort_by(|a, b| less_than(entries, a, b))
    }
}

fn less_than(entries: &EntryData, arg1: &CiteNumber, arg2: &CiteNumber) -> Ordering {
    let ptr1 = arg1 * entries.num_ent_strs() + entries.sort_key_num();
    let ptr2 = arg2 * entries.num_ent_strs() + entries.sort_key_num();

    let str1 = entries.strs(ptr1);
    let str2 = entries.strs(ptr2);

    Ord::cmp(str1, str2)
}

pub(crate) fn add_database_cite(
    cites: &mut CiteInfo,
    other: &mut OtherData,
    hash: &mut HashData,
    new_cite: CiteNumber,
    cite_loc: CiteNumber,
    lc_cite_loc: CiteNumber,
) -> CiteNumber {
    if new_cite == cites.cite_list.len() {
        cites.grow();
    }
    other.check_field_overflow(other.num_fields() * (new_cite + 1));

    cites.set_cite(new_cite, hash.text(cite_loc));
    hash.set_ilk_info(cite_loc, new_cite as i32);
    hash.set_ilk_info(lc_cite_loc, cite_loc as i32);
    new_cite + 1
}

pub(crate) fn find_cite_locs_for_this_cite_key(
    pool: &StringPool,
    hash: &HashData,
    cite_str: StrNumber,
) -> FindCiteLocs {
    let val = pool.get_str(cite_str);

    let cite_hash = pool.lookup_str(hash, val, StrIlk::Cite);
    let lc_cite_hash = pool.lookup_str(hash, &val.to_ascii_lowercase(), StrIlk::LcCite);

    FindCiteLocs {
        cite_loc: cite_hash.loc,
        cite_found: cite_hash.exists,
        lc_cite_loc: lc_cite_hash.loc,
        lc_found: lc_cite_hash.exists,
    }
}
