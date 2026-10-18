// Copyright 2025 the Tectonic Project
// Licensed under the MIT License.

//! Makeindex-compatible-ish ordering.
//!
//! The default is "word ordering": spaces sort before letters, numbers sort
//! numerically, and letters sort case-insensitively with uppercase before
//! lowercase when otherwise equal. This covers the common LaTeX index cases;
//! `-g`/`-l` and locale-specific collation are follow-up work.

use std::cmp::Ordering;

/// Compare two sort keys using makeindex's default word ordering.
pub(crate) fn compare(a: &str, b: &str) -> Ordering {
    let ta = tokens(a);
    let tb = tokens(b);
    ta.cmp(&tb)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    Space,
    Symbol(char),
    Number(u64),
    Letter(char, bool), // lowercase char, is_uppercase
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        use Token::*;
        match (self, other) {
            (Space, Space) => Ordering::Equal,
            (Space, _) => Ordering::Less,
            (_, Space) => Ordering::Greater,
            (Symbol(a), Symbol(b)) => a.cmp(b),
            (Symbol(_), _) => Ordering::Less,
            (_, Symbol(_)) => Ordering::Greater,
            (Number(a), Number(b)) => a.cmp(b),
            (Number(_), _) => Ordering::Less,
            (_, Number(_)) => Ordering::Greater,
            (Letter(a, au), Letter(b, bu)) => a.cmp(b).then_with(|| bu.cmp(au)),
        }
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn tokens(s: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            out.push(Token::Space);
            while chars.peek().is_some_and(|c| c.is_whitespace()) {
                chars.next();
            }
            continue;
        }
        if c.is_ascii_digit() {
            let mut n = u64::from(c.to_digit(10).unwrap());
            while let Some(next) = chars.peek().and_then(|c| c.to_digit(10)) {
                n = n.saturating_mul(10).saturating_add(u64::from(next));
                chars.next();
            }
            out.push(Token::Number(n));
            continue;
        }
        if c.is_alphabetic() {
            out.push(Token::Letter(
                c.to_lowercase().next().unwrap_or(c),
                c.is_uppercase(),
            ));
            continue;
        }
        out.push(Token::Symbol(c));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::compare;
    use std::cmp::Ordering;

    #[test]
    fn numbers_sort_numerically() {
        assert_eq!(compare("9", "10"), Ordering::Less);
    }

    #[test]
    fn letters_are_case_insensitive_with_uppercase_first() {
        assert_eq!(compare("Alpha", "alpha"), Ordering::Less);
    }
}
