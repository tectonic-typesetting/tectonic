use crate::c_api::ASCIICode;
macro_rules! const_for {
    ($start:literal..$end:literal => $arr:ident[..] = $expr:expr) => {
        let mut i = $start;
        while i < $end {
            $arr[i] = $expr;
            i += 1;
        }
    };
    ($start:literal..=$end:literal => $arr:ident[..] = $expr:expr) => {
        let mut i = $start;
        while i <= $end {
            $arr[i] = $expr;
            i += 1;
        }
    };
}

/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(C)]
pub enum LexClass {
    Illegal = 0,
    Whitespace = 1,
    Alpha = 2,
    Numeric = 3,
    Sep = 4,
    Other = 5,
}

impl LexClass {
    pub fn of(char: ASCIICode) -> LexClass {
        LEX_CLASS[char as usize]
    }
}

/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(C)]
pub enum IdClass {
    IllegalIdChar = 0,
    LegalIdChar = 1,
}

impl IdClass {
    pub fn of(char: ASCIICode) -> IdClass {
        ID_CLASS[char as usize]
    }
}

#[no_mangle]
pub static LEX_CLASS: [LexClass; 256] = {
    let mut lex_class = [LexClass::Other; 256];

    const_for!(128..=255 => lex_class[..] = LexClass::Alpha);
    const_for!(0..32 => lex_class[..] = LexClass::Illegal);

    // invalid_code
    lex_class[127] = LexClass::Illegal;
    // tab
    lex_class[9] = LexClass::Whitespace;
    lex_class[13] = LexClass::Whitespace;
    // space
    lex_class[32] = LexClass::Whitespace;
    // tie
    lex_class[126] = LexClass::Sep;
    // hyphen
    lex_class[45] = LexClass::Sep;

    const_for!(48..58 => lex_class[..] = LexClass::Numeric);
    const_for!(65..91 => lex_class[..] = LexClass::Alpha);
    const_for!(97..123 => lex_class[..] = LexClass::Alpha);

    lex_class
};

#[no_mangle]
pub static ID_CLASS: [IdClass; 256] = {
    let mut id_class = [IdClass::LegalIdChar; 256];

    const_for!(0..32 => id_class[..] = IdClass::IllegalIdChar);
    // tab
    id_class[9] = IdClass::IllegalIdChar;
    // space
    id_class[32] = IdClass::IllegalIdChar;
    // double quote
    id_class[34] = IdClass::IllegalIdChar;
    // number sign
    id_class[35] = IdClass::IllegalIdChar;
    // comment
    id_class[37] = IdClass::IllegalIdChar;
    // single quote
    id_class[39] = IdClass::IllegalIdChar;
    // left paren
    id_class[40] = IdClass::IllegalIdChar;
    // right paren
    id_class[41] = IdClass::IllegalIdChar;
    // comma
    id_class[44] = IdClass::IllegalIdChar;
    // equals sign
    id_class[61] = IdClass::IllegalIdChar;
    // left brace
    id_class[123] = IdClass::IllegalIdChar;
    // right brace
    id_class[125] = IdClass::IllegalIdChar;

    id_class
};

#[no_mangle]
pub static CHAR_WIDTH: [i32; 256] = {
    let mut char_width = [0; 256];

    char_width[32] = 278;
    char_width[33] = 278;
    char_width[34] = 500;
    char_width[35] = 833;
    char_width[36] = 500;
    char_width[37] = 833;
    char_width[38] = 778;
    char_width[39] = 278;
    char_width[40] = 389;
    char_width[41] = 389;
    char_width[42] = 500;
    char_width[43] = 778;
    char_width[44] = 278;
    char_width[45] = 333;
    char_width[46] = 278;
    char_width[47] = 500;
    char_width[48] = 500;
    char_width[49] = 500;
    char_width[50] = 500;
    char_width[51] = 500;
    char_width[52] = 500;
    char_width[53] = 500;
    char_width[54] = 500;
    char_width[55] = 500;
    char_width[56] = 500;
    char_width[57] = 500;
    char_width[58] = 278;
    char_width[59] = 278;
    char_width[60] = 278;
    char_width[61] = 778;
    char_width[62] = 472;
    char_width[63] = 472;
    char_width[64] = 778;
    char_width[65] = 750;
    char_width[66] = 708;
    char_width[67] = 722;
    char_width[68] = 764;
    char_width[69] = 681;
    char_width[70] = 653;
    char_width[71] = 785;
    char_width[72] = 750;
    char_width[73] = 361;
    char_width[74] = 514;
    char_width[75] = 778;
    char_width[76] = 625;
    char_width[77] = 917;
    char_width[78] = 750;
    char_width[79] = 778;
    char_width[80] = 681;
    char_width[81] = 778;
    char_width[82] = 736;
    char_width[83] = 556;
    char_width[84] = 722;
    char_width[85] = 750;
    char_width[86] = 750;
    char_width[87] = 1028;
    char_width[88] = 750;
    char_width[89] = 750;
    char_width[90] = 611;
    char_width[91] = 278;
    char_width[92] = 500;
    char_width[93] = 278;
    char_width[94] = 500;
    char_width[95] = 278;
    char_width[96] = 278;
    char_width[97] = 500;
    char_width[98] = 556;
    char_width[99] = 444;
    char_width[100] = 556;
    char_width[101] = 444;
    char_width[102] = 306;
    char_width[103] = 500;
    char_width[104] = 556;
    char_width[105] = 278;
    char_width[106] = 306;
    char_width[107] = 528;
    char_width[108] = 278;
    char_width[109] = 833;
    char_width[110] = 556;
    char_width[111] = 500;
    char_width[112] = 556;
    char_width[113] = 528;
    char_width[114] = 392;
    char_width[115] = 394;
    char_width[116] = 389;
    char_width[117] = 556;
    char_width[118] = 528;
    char_width[119] = 722;
    char_width[120] = 528;
    char_width[121] = 528;
    char_width[122] = 444;
    char_width[123] = 500;
    char_width[124] = 1000;
    char_width[125] = 500;
    char_width[126] = 500;

    char_width
};
