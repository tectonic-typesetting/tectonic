use std::{fmt, fmt::Formatter, ops::Range};

const POOL_SIZE: usize = 65536;
const STRINGS_SIZE: usize = 2048;
pub(crate) const MAX_PRINT_LINE: usize = 79;
pub(crate) const MIN_PRINT_LINE: usize = 3;
pub(crate) const MAX_STRINGS: usize = 35307;

#[derive(Default, Debug, PartialEq, PartialOrd, Copy, Clone, Hash)]
pub struct StrNumber(usize);

impl fmt::Display for StrNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StrNumber {
    pub fn invalid() -> StrNumber {
        StrNumber(0)
    }

    pub fn from_raw_dangerous(val: usize) -> StrNumber {
        StrNumber(val)
    }

    pub fn to_raw_dangerous(self) -> usize {
        self.0
    }

    pub fn is_invalid(self) -> bool {
        self.0 == 0
    }
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub(crate) struct Checkpoint(usize);

impl Checkpoint {
    pub fn is_before(self, num: StrNumber) -> bool {
        num.0 >= self.0
    }
}

pub(crate) struct PoolCursor<'a> {
    pool: &'a mut StringPool,
    start: usize,
    end: usize,
}

impl PoolCursor<'_> {
    pub fn extend(&mut self, len: usize) {
        self.end += len;
    }

    pub fn append(&mut self, c: u8) {
        self.pool.strings[self.end] = c;
        self.end += 1;
    }

    pub fn append_str(&mut self, str: StrNumber) {
        self.pool.copy_raw(str, self.end);
        self.end += self.pool.get_str(str).len();
    }

    pub fn append_substr(&mut self, str: StrNumber, range: Range<usize>) {
        let start = self.pool.str_start(str);
        self.pool
            .copy_range_raw(start + range.start..start + range.end, self.end);
    }

    pub fn insert_str(&mut self, str: StrNumber, offset: usize) {
        self.pool.copy_raw(str, self.start + offset);
        let len = self.pool.get_str(str).len();
        if self.start + offset + len > self.end {
            self.end = self.start + offset + len;
        }
    }

    pub fn bytes(&mut self) -> &mut [u8] {
        &mut self.pool.strings[self.start..self.end]
    }
}

pub(crate) struct StringPool {
    strings: Vec<u8>,
    // Stores string starting locations in the string pool
    // length of string `s` is offsets[s] - offsets[s-1]
    offsets: Vec<usize>,
    pool_ptr: usize,
    cur_strs: usize,
}

impl StringPool {
    pub fn new() -> StringPool {
        let mut offsets = Vec::with_capacity(STRINGS_SIZE);
        // First string always starts at 0
        offsets.push(0);
        StringPool {
            strings: Vec::with_capacity(POOL_SIZE),
            offsets,
            pool_ptr: 0,
            cur_strs: 1,
        }
    }

    fn str_start(&self, str: StrNumber) -> usize {
        self.offsets[str.0 - 1]
    }

    fn str_end(&self, str: StrNumber) -> usize {
        self.offsets[str.0]
    }

    pub fn try_get_str(&self, s: StrNumber) -> Option<&[u8]> {
        // This is plus three because bst does weird stuff by popping and then sometimes re-adding
        // strings.
        // TODO: Fix bst execution to not rely on this behavior
        if s.is_invalid() || s.0 >= self.cur_strs + 3 || s.0 + 1 > self.offsets.len() {
            None
        } else {
            Some(&self.strings[self.str_start(s)..self.str_end(s)])
        }
    }

    pub fn get_str(&self, s: StrNumber) -> &[u8] {
        self.try_get_str(s)
            .unwrap_or_else(|| panic!("String number {s} doesn't exist"))
    }

    pub fn add_string(&mut self, str: &[u8]) -> StrNumber {
        while self.pool_ptr + str.len() > self.strings.len() {
            self.grow();
        }
        self.strings[self.pool_ptr..self.pool_ptr + str.len()].copy_from_slice(str);
        self.pool_ptr += str.len();
        self.make_string()
    }

    pub fn write_str(&mut self, f: impl FnOnce(&mut PoolCursor)) -> StrNumber {
        let mut cursor = PoolCursor {
            start: self.pool_ptr,
            end: self.pool_ptr,
            pool: self,
        };
        f(&mut cursor);
        self.pool_ptr = cursor.end;
        self.make_string()
    }

    /// Check if the provided string is the last. If it is, remove it from the pool and return true.
    /// Otherwise, return false.
    pub fn remove_last_str(&mut self, str: StrNumber) -> bool {
        if str.0 != self.cur_strs - 1 {
            false
        } else {
            self.cur_strs -= 1;
            self.pool_ptr = self.str_start(str);
            true
        }
    }

    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.cur_strs)
    }

    pub fn is_at(&self, check: Checkpoint) -> bool {
        self.cur_strs == check.0
    }

    fn grow(&mut self) {
        self.strings.resize(self.strings.len() + POOL_SIZE, 0);
    }

    /// Used while defining strings - declare the current `pool_ptr` as the end of the current
    /// string, increment `cur_strs`, and return the new string's `StrNumber`
    fn make_string(&mut self) -> StrNumber {
        if self.cur_strs + 1 > self.offsets.len() {
            self.offsets.push(self.pool_ptr)
        } else {
            self.offsets[self.cur_strs] = self.pool_ptr
        }
        self.cur_strs += 1;
        StrNumber(self.cur_strs - 1)
    }

    fn copy_raw(&mut self, str: StrNumber, pos: usize) {
        let start = self.str_start(str);
        let end = self.str_end(str);

        while pos + (end - start) > self.strings.len() {
            self.grow();
        }

        self.strings.copy_within(start..end, pos);
    }

    fn copy_range_raw(&mut self, range: Range<usize>, pos: usize) {
        while pos + (range.end - range.start) > self.strings.len() {
            self.grow();
        }
        self.strings.copy_within(range, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_str() {
        let pool = StringPool::new();
        assert_eq!(pool.try_get_str(StrNumber::invalid()), None);
    }

    #[test]
    fn test_add_str() {
        let mut pool = StringPool::new();

        let str1 = pool.add_string(b"String 1");
        let str2 = pool.add_string(b"String 2");

        assert_ne!(str1, str2);
        assert_eq!(pool.get_str(str1), b"String 1");
        assert_eq!(pool.get_str(str2), b"String 2");
    }

    #[test]
    fn test_write_str() {
        let mut pool = StringPool::new();

        let str = pool.add_string(b"Hello World!");

        let new_str = pool.write_str(|cursor| {
            cursor.append_str(str);
        });
        assert_ne!(str, new_str);
        assert_eq!(pool.get_str(str), pool.get_str(new_str));

        assert!(pool.remove_last_str(new_str));
        // Ensure we can get length of removed string
        let str_len = pool.get_str(new_str).len();
        // Ensure extending by that length restores the string
        let new_str_2 = pool.write_str(|cursor| {
            cursor.extend(str_len);
        });
        assert_eq!(new_str, new_str_2);
        assert_eq!(pool.get_str(str), pool.get_str(new_str_2));
    }
}
