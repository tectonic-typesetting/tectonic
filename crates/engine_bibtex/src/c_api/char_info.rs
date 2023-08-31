use crate::c_api::ASCIICode;

macro_rules! const_for {
    ($start:literal..$end:literal => $arr:ident[..] = $expr:expr) => {
        let mut i = $start;
        while i < $end {
            $arr[i as usize] = $expr;
            i += 1;
        }
    };
    ($start:literal..=$end:literal => $arr:ident[..] = $expr:expr) => {
        let mut i = $start;
        while i <= $end {
            $arr[i as usize] = $expr;
            i += 1;
        }
    };
}

/// The lexer class of a character - this represents whether the parser considers it to be alphabetic,
/// numeric, etc. Illegal represents tokens that shouldn't show up at all, such as ASCII backspace.
///
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
    /// Get the `LexClass` of a character
    pub fn of(char: ASCIICode) -> LexClass {
        LEX_CLASS[char as usize]
    }
}

/// Whether a token counts as valid in an identifier
///
/// cbindgen:rename-all=ScreamingSnakeCase
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(C)]
pub enum IdClass {
    IllegalIdChar = 0,
    LegalIdChar = 1,
}

impl IdClass {
    /// Get the `IdClass` of a character
    pub const fn of(char: ASCIICode) -> IdClass {
        ID_CLASS[char as usize]
    }
}

#[no_mangle]
pub static LEX_CLASS: [LexClass; 256] = {
    let mut lex_class = [LexClass::Other; 256];

    const_for!(128..=255 => lex_class[..] = LexClass::Alpha);
    // NUL..=US
    const_for!(0..32 => lex_class[..] = LexClass::Illegal);

    // invalid_code
    lex_class[127] = LexClass::Illegal;
    lex_class[b'\t' as usize] = LexClass::Whitespace;
    lex_class[b'\r' as usize] = LexClass::Whitespace;
    lex_class[b' ' as usize] = LexClass::Whitespace;
    lex_class[b'~' as usize] = LexClass::Sep;
    lex_class[b'-' as usize] = LexClass::Sep;

    const_for!(b'0'..=b'9' => lex_class[..] = LexClass::Numeric);
    const_for!(b'A'..=b'Z' => lex_class[..] = LexClass::Alpha);
    const_for!(b'a'..=b'z' => lex_class[..] = LexClass::Alpha);

    lex_class
};

pub const ID_CLASS: [IdClass; 256] = {
    let mut id_class = [IdClass::LegalIdChar; 256];

    // NUL..=US
    const_for!(0..32 => id_class[..] = IdClass::IllegalIdChar);

    id_class[b'\t' as usize] = IdClass::IllegalIdChar;
    id_class[b' ' as usize] = IdClass::IllegalIdChar;
    id_class[b'"' as usize] = IdClass::IllegalIdChar;
    id_class[b'#' as usize] = IdClass::IllegalIdChar;
    id_class[b'%' as usize] = IdClass::IllegalIdChar;
    id_class[b'\'' as usize] = IdClass::IllegalIdChar;
    id_class[b'(' as usize] = IdClass::IllegalIdChar;
    id_class[b')' as usize] = IdClass::IllegalIdChar;
    id_class[b',' as usize] = IdClass::IllegalIdChar;
    id_class[b'=' as usize] = IdClass::IllegalIdChar;
    id_class[b'{' as usize] = IdClass::IllegalIdChar;
    id_class[b'}' as usize] = IdClass::IllegalIdChar;

    id_class
};

pub static CHAR_WIDTH: [i32; 256] = {
    let mut char_width = [0; 256];

    // Values taken from the bibtex web2c implementation
    char_width[b' ' as usize] = 278;
    char_width[b'!' as usize] = 278;
    char_width[b'"' as usize] = 500;
    char_width[b'#' as usize] = 833;
    char_width[b'$' as usize] = 500;
    char_width[b'%' as usize] = 833;
    char_width[b'&' as usize] = 778;
    char_width[b'\'' as usize] = 278;
    char_width[b'(' as usize] = 389;
    char_width[b')' as usize] = 389;
    char_width[b'*' as usize] = 500;
    char_width[b'+' as usize] = 778;
    char_width[b',' as usize] = 278;
    char_width[b'-' as usize] = 333;
    char_width[b'.' as usize] = 278;
    char_width[b'/' as usize] = 500;
    char_width[b'0' as usize] = 500;
    char_width[b'1' as usize] = 500;
    char_width[b'2' as usize] = 500;
    char_width[b'3' as usize] = 500;
    char_width[b'4' as usize] = 500;
    char_width[b'5' as usize] = 500;
    char_width[b'6' as usize] = 500;
    char_width[b'7' as usize] = 500;
    char_width[b'8' as usize] = 500;
    char_width[b'9' as usize] = 500;
    char_width[b':' as usize] = 278;
    char_width[b';' as usize] = 278;
    char_width[b'<' as usize] = 278;
    char_width[b'=' as usize] = 778;
    char_width[b'>' as usize] = 472;
    char_width[b'?' as usize] = 472;
    char_width[b'@' as usize] = 778;
    char_width[b'A' as usize] = 750;
    char_width[b'B' as usize] = 708;
    char_width[b'C' as usize] = 722;
    char_width[b'D' as usize] = 764;
    char_width[b'E' as usize] = 681;
    char_width[b'F' as usize] = 653;
    char_width[b'G' as usize] = 785;
    char_width[b'H' as usize] = 750;
    char_width[b'I' as usize] = 361;
    char_width[b'J' as usize] = 514;
    char_width[b'K' as usize] = 778;
    char_width[b'L' as usize] = 625;
    char_width[b'M' as usize] = 917;
    char_width[b'N' as usize] = 750;
    char_width[b'O' as usize] = 778;
    char_width[b'P' as usize] = 681;
    char_width[b'Q' as usize] = 778;
    char_width[b'R' as usize] = 736;
    char_width[b'S' as usize] = 556;
    char_width[b'T' as usize] = 722;
    char_width[b'U' as usize] = 750;
    char_width[b'V' as usize] = 750;
    char_width[b'W' as usize] = 1028;
    char_width[b'X' as usize] = 750;
    char_width[b'Y' as usize] = 750;
    char_width[b'Z' as usize] = 611;
    char_width[b'[' as usize] = 278;
    char_width[b'\\' as usize] = 500;
    char_width[b']' as usize] = 278;
    char_width[b'^' as usize] = 500;
    char_width[b'_' as usize] = 278;
    char_width[b'`' as usize] = 278;
    char_width[b'a' as usize] = 500;
    char_width[b'b' as usize] = 556;
    char_width[b'c' as usize] = 444;
    char_width[b'd' as usize] = 556;
    char_width[b'e' as usize] = 444;
    char_width[b'f' as usize] = 306;
    char_width[b'g' as usize] = 500;
    char_width[b'h' as usize] = 556;
    char_width[b'i' as usize] = 278;
    char_width[b'j' as usize] = 306;
    char_width[b'k' as usize] = 528;
    char_width[b'l' as usize] = 278;
    char_width[b'm' as usize] = 833;
    char_width[b'n' as usize] = 556;
    char_width[b'o' as usize] = 500;
    char_width[b'p' as usize] = 556;
    char_width[b'q' as usize] = 528;
    char_width[b'r' as usize] = 392;
    char_width[b's' as usize] = 394;
    char_width[b't' as usize] = 389;
    char_width[b'u' as usize] = 556;
    char_width[b'v' as usize] = 528;
    char_width[b'w' as usize] = 722;
    char_width[b'x' as usize] = 528;
    char_width[b'y' as usize] = 528;
    char_width[b'z' as usize] = 444;
    char_width[b'{' as usize] = 500;
    char_width[b'|' as usize] = 1000;
    char_width[b'}' as usize] = 500;
    char_width[b'~' as usize] = 500;

    char_width
};
