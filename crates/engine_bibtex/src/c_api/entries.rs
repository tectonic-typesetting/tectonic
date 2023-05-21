use crate::c_api::cite::with_cites;
use crate::c_api::xbuf::XBuf;
use crate::c_api::ASCIICode;
use std::cell::RefCell;

pub const ENT_STR_SIZE: usize = 250;

pub struct EntryData {
    num_entry_ints: i32,
    num_entry_strs: i32,
    sort_key_num: i32,
    entry_ints: Option<XBuf<i32>>,
    entry_strs: Option<XBuf<ASCIICode>>,
}

impl EntryData {
    fn new() -> EntryData {
        EntryData {
            num_entry_ints: 0,
            num_entry_strs: 0,
            sort_key_num: 0,
            entry_ints: None,
            entry_strs: None,
        }
    }

    pub fn strs(&self, pos: usize) -> ASCIICode {
        self.entry_strs.as_ref().unwrap()[pos]
    }

    pub fn num_ent_strs(&self) -> i32 {
        self.num_entry_strs
    }

    pub fn set_num_ent_strs(&mut self, val: i32) {
        self.num_entry_strs = val;
    }

    pub fn sort_key_num(&self) -> i32 {
        self.sort_key_num
    }

    pub fn set_sort_key_num(&mut self, val: i32) {
        self.sort_key_num = val;
    }
}

thread_local! {
    pub static ENTRIES: RefCell<EntryData> = RefCell::new(EntryData::new());
}

pub fn reset() {
    ENTRIES.with(|entries| *entries.borrow_mut() = EntryData::new());
}

pub fn with_entries<T>(f: impl FnOnce(&EntryData) -> T) -> T {
    ENTRIES.with(|entries| f(&entries.borrow()))
}

pub fn with_entries_mut<T>(f: impl FnOnce(&mut EntryData) -> T) -> T {
    ENTRIES.with(|entries| f(&mut entries.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn init_entry_ints() {
    with_entries_mut(|entries| {
        let num_cites = with_cites(|cites| cites.num_cites());
        entries.entry_ints = Some(XBuf::new(
            ((entries.num_entry_ints + 1) * (num_cites + 1)) as usize,
        ));
    })
}

#[no_mangle]
pub extern "C" fn init_entry_strs() {
    with_entries_mut(|entries| {
        let num_cites = with_cites(|cites| cites.num_cites());
        let mut new_buf = XBuf::new(
            (entries.num_entry_strs + 1) as usize * (num_cites + 1) as usize * (ENT_STR_SIZE + 1),
        );
        new_buf.fill(127);
        entries.entry_strs = Some(new_buf);
    })
}

#[no_mangle]
pub extern "C" fn num_ent_ints() -> i32 {
    with_entries(|entries| entries.num_entry_ints)
}

#[no_mangle]
pub extern "C" fn set_num_ent_ints(val: i32) {
    with_entries_mut(|entries| entries.num_entry_ints = val)
}

#[no_mangle]
pub extern "C" fn num_ent_strs() -> i32 {
    with_entries(|entries| entries.num_entry_strs)
}

#[no_mangle]
pub extern "C" fn set_num_ent_strs(val: i32) {
    with_entries_mut(|entries| entries.num_entry_strs = val)
}

#[no_mangle]
pub extern "C" fn entry_ints(pos: i32) -> i32 {
    with_entries(|entries| entries.entry_ints.as_ref().unwrap()[pos as usize])
}

#[no_mangle]
pub extern "C" fn set_entry_ints(pos: i32, val: i32) {
    with_entries_mut(|entries| entries.entry_ints.as_mut().unwrap()[pos as usize] = val)
}

#[no_mangle]
pub extern "C" fn entry_strs(pos: i32) -> ASCIICode {
    with_entries(|entries| entries.entry_strs.as_ref().unwrap()[pos as usize])
}

#[no_mangle]
pub extern "C" fn set_entry_strs(pos: i32, val: ASCIICode) {
    with_entries_mut(|entries| entries.entry_strs.as_mut().unwrap()[pos as usize] = val)
}
