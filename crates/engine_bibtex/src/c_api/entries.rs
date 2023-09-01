use crate::c_api::{cite::with_cites, xbuf::XBuf, ASCIICode};
use std::cell::RefCell;

pub(crate) const ENT_STR_SIZE: usize = 250;

pub(crate) struct EntryData {
    num_entry_ints: usize,
    num_entry_strs: usize,
    sort_key_num: usize,
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

    pub fn ints(&self, pos: usize) -> i32 {
        self.entry_ints.as_ref().unwrap()[pos]
    }

    pub fn set_int(&mut self, pos: usize, val: i32) {
        self.entry_ints.as_mut().unwrap()[pos] = val;
    }

    pub fn strs(&self, start: usize) -> &[ASCIICode] {
        let start = start * (ENT_STR_SIZE + 1);
        let strs = self.entry_strs.as_ref().unwrap();
        let end_pos = strs[start..start + ENT_STR_SIZE + 1]
            .iter()
            .position(|c| *c == 127)
            .unwrap_or(ENT_STR_SIZE + 1);

        &strs[start..start + end_pos]
    }

    pub fn set_str(&mut self, pos: usize, val: &[ASCIICode]) {
        assert!(val.len() <= ENT_STR_SIZE);
        let pos = pos * (ENT_STR_SIZE + 1);
        let strs = self.entry_strs.as_mut().unwrap();
        strs[pos..pos + val.len()].copy_from_slice(val);
        strs[pos + val.len()] = 127;
    }

    pub fn num_ent_ints(&self) -> usize {
        self.num_entry_ints
    }

    pub fn set_num_ent_ints(&mut self, val: usize) {
        self.num_entry_ints = val;
    }

    pub fn num_ent_strs(&self) -> usize {
        self.num_entry_strs
    }

    pub fn set_num_ent_strs(&mut self, val: usize) {
        self.num_entry_strs = val;
    }

    pub fn sort_key_num(&self) -> usize {
        self.sort_key_num
    }

    pub fn set_sort_key_num(&mut self, val: usize) {
        self.sort_key_num = val;
    }
}

thread_local! {
    static ENTRIES: RefCell<EntryData> = RefCell::new(EntryData::new());
}

pub fn reset() {
    ENTRIES.with(|entries| *entries.borrow_mut() = EntryData::new());
}

pub(crate) fn with_entries<T>(f: impl FnOnce(&EntryData) -> T) -> T {
    ENTRIES.with(|entries| f(&entries.borrow()))
}

pub(crate) fn with_entries_mut<T>(f: impl FnOnce(&mut EntryData) -> T) -> T {
    ENTRIES.with(|entries| f(&mut entries.borrow_mut()))
}

#[no_mangle]
pub extern "C" fn init_entry_ints() {
    with_entries_mut(|entries| {
        let num_cites = with_cites(|cites| cites.num_cites());
        entries.entry_ints = Some(XBuf::new((entries.num_entry_ints + 1) * (num_cites + 1)));
    })
}

#[no_mangle]
pub extern "C" fn init_entry_strs() {
    with_entries_mut(|entries| {
        let num_cites = with_cites(|cites| cites.num_cites());
        let mut new_buf =
            XBuf::new((entries.num_entry_strs + 1) * (num_cites + 1) * (ENT_STR_SIZE + 1));
        new_buf.fill(127);
        entries.entry_strs = Some(new_buf);
    })
}
