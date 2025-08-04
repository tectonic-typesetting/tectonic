use crate::{
    entries::EntryData,
    hash,
    hash::{Cite, HashData, LcCite},
    other::OtherData,
    pool::{StrNumber, StringPool},
    CiteNumber, FindCiteLocs, HashPointer,
};
use std::{cmp::Ordering, ops::IndexMut};

pub(crate) const MAX_CITES: usize = 750;

pub(crate) struct CiteInfo {
    cite_list: Vec<StrNumber>,
    cite_info: Vec<StrNumber>,
    type_list: Vec<HashPointer<hash::BstFn>>,
    entry_exists: Vec<bool>,
    cite_ptr: CiteNumber,

    entry_cite_ptr: CiteNumber,
    num_cites: CiteNumber,
    old_num_cites: CiteNumber,
    all_marker: CiteNumber,
}

impl CiteInfo {
    pub fn new() -> CiteInfo {
        CiteInfo {
            cite_list: vec![StrNumber::invalid(); MAX_CITES + 1],
            cite_info: vec![StrNumber::invalid(); MAX_CITES + 1],
            type_list: vec![HashPointer::default(); MAX_CITES + 1],
            entry_exists: vec![false; MAX_CITES + 1],
            cite_ptr: 0,
            entry_cite_ptr: 0,
            num_cites: 0,
            old_num_cites: 0,
            all_marker: 0,
        }
    }

    pub fn grow(&mut self) {
        self.cite_list
            .resize(self.cite_list.len() + MAX_CITES, StrNumber::invalid());
        self.cite_info
            .resize(self.cite_info.len() + MAX_CITES, StrNumber::invalid());
        self.type_list
            .resize(self.type_list.len() + MAX_CITES, HashPointer::default());
        self.entry_exists
            .resize(self.entry_exists.len() + MAX_CITES, false);
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

    pub fn get_type(&self, offset: usize) -> HashPointer<hash::BstFn> {
        self.type_list[offset]
    }

    pub fn set_type(&mut self, offset: usize, ty: HashPointer<hash::BstFn>) {
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
        Vec<StrNumber>: IndexMut<I, Output = [StrNumber]>,
    {
        self.cite_info[r]
            .sort_by(|a, b| less_than(entries, a.to_raw_dangerous(), b.to_raw_dangerous()))
    }
}

fn less_than(entries: &EntryData, arg1: CiteNumber, arg2: CiteNumber) -> Ordering {
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
    cite_loc: HashPointer<Cite>,
    lc_cite_loc: HashPointer<LcCite>,
) -> CiteNumber {
    if new_cite == cites.cite_list.len() {
        cites.grow();
    }
    other.check_field_overflow(other.num_fields() * (new_cite + 1));

    cites.set_cite(new_cite, hash.get(cite_loc).text());
    hash.set_extra(cite_loc, new_cite);
    hash.set_extra(lc_cite_loc, cite_loc);
    new_cite + 1
}

pub(crate) fn find_cite_locs_for_this_cite_key(
    pool: &StringPool,
    hash: &HashData,
    cite_str: StrNumber,
) -> FindCiteLocs {
    let val = pool.get_str(cite_str);

    let cite = hash.lookup_str::<hash::Cite>(pool, val);
    let lc_cite = hash.lookup_str::<hash::LcCite>(pool, &val.to_ascii_lowercase());

    FindCiteLocs { cite, lc_cite }
}
