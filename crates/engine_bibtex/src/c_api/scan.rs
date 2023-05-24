use crate::c_api::buffer::{with_buffers, with_buffers_mut, BufTy};
use crate::c_api::char_info::{IdClass, LexClass};
use crate::c_api::{ASCIICode, BufPointer};

/// cbindgen:rename-all=ScreamingSnakeCase
#[repr(C)]
pub enum ScanRes {
    IdNull = 0,
    SpecifiedCharAdjacent = 1,
    OtherCharAdjacent = 2,
    WhitespaceAdjacent = 3,
}

pub struct Scan<'a> {
    chars: &'a [ASCIICode],
    not_class: Option<LexClass>,
    class: Option<LexClass>,
}

impl<'a> Scan<'a> {
    fn new(chars: &'a [ASCIICode]) -> Scan<'a> {
        Scan {
            chars,
            not_class: None,
            class: None,
        }
    }

    fn not_class(mut self, class: LexClass) -> Self {
        self.not_class = Some(class);
        self
    }

    fn class(mut self, class: LexClass) -> Self {
        self.class = Some(class);
        self
    }

    fn match_char(&self, char: ASCIICode) -> bool {
        self.not_class
            .map_or(false, |class| LexClass::of(char) != class)
            || self
                .class
                .map_or(false, |class| LexClass::of(char) == class)
            || self.chars.contains(&char)
    }

    fn scan_till(&self, last: BufPointer) -> bool {
        with_buffers_mut(|buffers| {
            buffers.set_offset(BufTy::Base, 1, buffers.offset(BufTy::Base, 2));

            let mut idx = buffers.offset(BufTy::Base, 2);
            while idx < last && !self.match_char(buffers.at(BufTy::Base, idx)) {
                idx += 1;
            }
            buffers.set_offset(BufTy::Base, 2, idx);

            idx < last
        })
    }

    fn scan_till_nonempty(&self, last: BufPointer) -> bool {
        with_buffers_mut(|buffers| {
            let start = buffers.offset(BufTy::Base, 2);
            buffers.set_offset(BufTy::Base, 1, start);

            let mut idx = start;
            while idx < last && !self.match_char(buffers.at(BufTy::Base, idx)) {
                idx += 1;
            }
            buffers.set_offset(BufTy::Base, 2, idx);

            idx - start != 0
        })
    }
}

#[no_mangle]
pub extern "C" fn scan1(char1: ASCIICode) -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[char1]).scan_till(last)
}

#[no_mangle]
pub extern "C" fn scan1_white(char1: ASCIICode) -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[char1])
        .class(LexClass::Whitespace)
        .scan_till(last)
}

#[no_mangle]
pub extern "C" fn scan2(char1: ASCIICode, char2: ASCIICode) -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[char1, char2]).scan_till(last)
}

#[no_mangle]
pub extern "C" fn scan2_white(char1: ASCIICode, char2: ASCIICode) -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[char1, char2])
        .class(LexClass::Whitespace)
        .scan_till(last)
}

#[no_mangle]
pub extern "C" fn scan3(char1: ASCIICode, char2: ASCIICode, char3: ASCIICode) -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[char1, char2, char3]).scan_till(last)
}

#[no_mangle]
pub extern "C" fn scan_alpha() -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[])
        .not_class(LexClass::Alpha)
        .scan_till_nonempty(last)
}

#[no_mangle]
pub extern "C" fn scan_white_space() -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[])
        .not_class(LexClass::Whitespace)
        .scan_till(last)
}

#[no_mangle]
pub extern "C" fn scan_identifier(char1: ASCIICode, char2: ASCIICode, char3: ASCIICode) -> ScanRes {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        let start = buffers.offset(BufTy::Base, 2);
        buffers.set_offset(BufTy::Base, 1, start);

        let mut idx = start;
        let char = buffers.at(BufTy::Base, idx);
        if LexClass::of(char) != LexClass::Numeric {
            while idx < last && IdClass::of(buffers.at(BufTy::Base, idx)) == IdClass::LegalIdChar {
                idx += 1;
            }
            buffers.set_offset(BufTy::Base, 2, idx);
        }

        let char = buffers.at(BufTy::Base, idx);
        if idx - start == 0 {
            ScanRes::IdNull
        } else if LexClass::of(char) == LexClass::Whitespace || idx == last {
            ScanRes::WhitespaceAdjacent
        } else if char == char1 || char == char2 || char == char3 {
            ScanRes::SpecifiedCharAdjacent
        } else {
            ScanRes::OtherCharAdjacent
        }
    })
}

#[no_mangle]
pub extern "C" fn scan_nonneg_integer() -> bool {
    let last = with_buffers(|buffers| buffers.init(BufTy::Base));
    Scan::new(&[])
        .not_class(LexClass::Numeric)
        .scan_till_nonempty(last)
}

#[no_mangle]
pub extern "C" fn scan_integer(token_value: &mut i32) -> bool {
    with_buffers_mut(|buffers| {
        let last = buffers.init(BufTy::Base);
        let start = buffers.offset(BufTy::Base, 2);
        buffers.set_offset(BufTy::Base, 1, start);

        let mut idx = start;
        let sign = if buffers.at(BufTy::Base, idx) == b'-' {
            idx += 1;
            true
        } else {
            false
        };

        *token_value = 0;
        let mut char = buffers.at(BufTy::Base, idx);
        while idx < last && LexClass::of(char) == LexClass::Numeric {
            *token_value = *token_value * 10 + (char - 48) as i32;
            idx += 1;
            char = buffers.at(BufTy::Base, idx);
        }
        buffers.set_offset(BufTy::Base, 2, idx);

        if sign {
            *token_value *= -1;
        }

        idx - start != if sign { 1 } else { 0 }
    })
}
