use crate::{cite::CiteInfo, ASCIICode};

pub(crate) const ENT_STR_SIZE: usize = 250;

pub(crate) struct EntryData {
    num_entry_ints: usize,
    num_entry_strs: usize,
    sort_key_num: usize,
    entry_ints: Option<Vec<i32>>,
    entry_strs: Option<Vec<ASCIICode>>,
}

impl EntryData {
    pub fn new() -> EntryData {
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

    pub fn init_entries(&mut self, cites: &CiteInfo) {
        let num_cites = cites.num_cites();
        self.entry_ints = Some(vec![0; (self.num_entry_ints + 1) * (num_cites + 1) + 1]);
        self.entry_strs = Some(vec![
            127;
            (self.num_entry_strs + 1)
                * (num_cites + 1)
                * (ENT_STR_SIZE + 1)
                + 1
        ]);
    }
}
