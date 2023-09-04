use crate::c_api::{
    entries::with_entries,
    hash::{with_hash, with_hash_mut, HashData},
    other::{with_other_mut, OtherData},
    pool::with_pool,
    xbuf::XBuf,
    CiteNumber, FindCiteLocs, HashPointer, StrIlk, StrNumber,
};
use std::{cell::RefCell, cmp::Ordering};

pub const MAX_CITES: usize = 750;

pub struct CiteInfo {
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
    fn new() -> CiteInfo {
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

    fn grow(&mut self) {
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

    pub fn get_info(&self, offset: usize) -> StrNumber {
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

    pub fn get_exists(&self, offset: usize) -> bool {
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

    pub fn set_entry_ptr(&mut self, ptr: CiteNumber) {
        self.entry_cite_ptr = ptr;
    }

    pub fn num_cites(&self) -> CiteNumber {
        self.num_cites
    }

    pub fn old_num_cites(&self) -> CiteNumber {
        self.old_num_cites
    }
}

thread_local! {
    pub static CITE_INFO: RefCell<CiteInfo> = RefCell::new(CiteInfo::new());
}

pub fn reset() {
    CITE_INFO.with(|ci| *ci.borrow_mut() = CiteInfo::new());
}

pub fn with_cites<T>(f: impl FnOnce(&CiteInfo) -> T) -> T {
    CITE_INFO.with(|ci| f(&ci.borrow()))
}

pub fn with_cites_mut<T>(f: impl FnOnce(&mut CiteInfo) -> T) -> T {
    CITE_INFO.with(|ci| f(&mut ci.borrow_mut()))
}

fn less_than(arg1: &CiteNumber, arg2: &CiteNumber) -> Ordering {
    with_entries(|entries| {
        let ptr1 = arg1 * entries.num_ent_strs() + entries.sort_key_num();
        let ptr2 = arg2 * entries.num_ent_strs() + entries.sort_key_num();

        let str1 = entries.strs(ptr1);
        let str2 = entries.strs(ptr2);

        Ord::cmp(str1, str2)
    })
}

#[no_mangle]
pub extern "C" fn quick_sort(left_end: CiteNumber, right_end: CiteNumber) {
    with_cites_mut(|cites| cites.cite_info[left_end..=right_end].sort_by(less_than))
}

#[no_mangle]
pub extern "C" fn cite_list(num: CiteNumber) -> StrNumber {
    with_cites(|cites| cites.get_cite(num))
}

#[no_mangle]
pub extern "C" fn set_cite_list(num: CiteNumber, str: StrNumber) {
    with_cites_mut(|cites| cites.set_cite(num, str))
}

#[no_mangle]
pub extern "C" fn cite_ptr() -> CiteNumber {
    with_cites(|cites| cites.ptr())
}

#[no_mangle]
pub extern "C" fn set_cite_ptr(num: CiteNumber) {
    with_cites_mut(|cites| cites.set_ptr(num))
}

#[no_mangle]
pub extern "C" fn check_cite_overflow(last_cite: CiteNumber) {
    with_cites_mut(|cites| {
        if last_cite == cites.cite_list.len() {
            cites.grow();
        }
    })
}

#[no_mangle]
pub extern "C" fn max_cites() -> usize {
    with_cites(|cites| cites.cite_list.len())
}

#[no_mangle]
pub extern "C" fn cite_info(num: CiteNumber) -> StrNumber {
    with_cites(|cites| cites.get_info(num))
}

#[no_mangle]
pub extern "C" fn set_cite_info(num: CiteNumber, info: StrNumber) {
    with_cites_mut(|cites| cites.set_info(num, info))
}

#[no_mangle]
pub extern "C" fn type_list(num: CiteNumber) -> HashPointer {
    with_cites(|cites| cites.get_type(num))
}

#[no_mangle]
pub extern "C" fn set_type_list(num: CiteNumber, ty: HashPointer) {
    with_cites_mut(|cites| cites.set_type(num, ty))
}

#[no_mangle]
pub extern "C" fn entry_exists(num: CiteNumber) -> bool {
    with_cites(|cites| cites.get_exists(num))
}

#[no_mangle]
pub extern "C" fn set_entry_exists(num: CiteNumber, exists: bool) {
    with_cites_mut(|cites| cites.set_exists(num, exists))
}

#[no_mangle]
pub extern "C" fn entry_cite_ptr() -> CiteNumber {
    with_cites(|cites| cites.entry_cite_ptr)
}

#[no_mangle]
pub extern "C" fn set_entry_cite_ptr(val: CiteNumber) {
    with_cites_mut(|cites| cites.entry_cite_ptr = val)
}

#[no_mangle]
pub extern "C" fn num_cites() -> CiteNumber {
    with_cites(|cites| cites.num_cites)
}

#[no_mangle]
pub extern "C" fn set_num_cites(val: CiteNumber) {
    with_cites_mut(|cites| cites.num_cites = val)
}

#[no_mangle]
pub extern "C" fn old_num_cites() -> CiteNumber {
    with_cites(|cites| cites.old_num_cites)
}

#[no_mangle]
pub extern "C" fn set_old_num_cites(val: CiteNumber) {
    with_cites_mut(|cites| cites.old_num_cites = val)
}

#[no_mangle]
pub extern "C" fn all_marker() -> CiteNumber {
    with_cites(|cites| cites.all_marker)
}

#[no_mangle]
pub extern "C" fn set_all_marker(val: CiteNumber) {
    with_cites_mut(|cites| cites.all_marker = val)
}

pub fn rs_add_database_cite(
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

#[no_mangle]
pub extern "C" fn add_database_cite(
    new_cite: CiteNumber,
    cite_loc: CiteNumber,
    lc_cite_loc: CiteNumber,
) -> CiteNumber {
    with_cites_mut(|cites| {
        with_other_mut(|other| {
            with_hash_mut(|hash| {
                rs_add_database_cite(cites, other, hash, new_cite, cite_loc, lc_cite_loc)
            })
        })
    })
}

#[no_mangle]
pub extern "C" fn find_cite_locs_for_this_cite_key(cite_str: StrNumber) -> FindCiteLocs {
    with_pool(|pool| {
        let val = pool.get_str(cite_str);

        let (cite_hash, lc_cite_hash) = with_hash(|hash| {
            let cite_hash = pool.lookup_str(hash, val, StrIlk::Cite);
            let lc_cite_hash = pool.lookup_str(hash, &val.to_ascii_lowercase(), StrIlk::LcCite);
            (cite_hash, lc_cite_hash)
        });

        FindCiteLocs {
            cite_loc: cite_hash.loc,
            cite_found: cite_hash.exists,
            lc_cite_loc: lc_cite_hash.loc,
            lc_found: lc_cite_hash.exists,
        }
    })
}
