// Copyright 2025 the Tectonic Project
// Licensed under the MIT License.

//! Parsing of makeindex `.idx` input files.

use crate::style::Style;

/// One index entry as it appears in the input file.
#[derive(Clone, Debug)]
pub(crate) struct RawEntry {
    /// Sort and display text for each nesting level.
    pub levels: Vec<Level>,
    /// The page argument as written in the input.
    pub page: String,
    /// The `|` encapsulator, if any.
    pub encap: Option<String>,
}

/// The sort/display pair for one nesting level.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Level {
    pub sort: String,
    pub display: String,
}

/// Parse all well-formed entries from an `.idx` file.
///
/// Malformed lines are skipped; makeindex normally reports them, but a
/// partially-written `.idx` must not abort the whole build.
pub(crate) fn parse(data: &[u8], style: &Style) -> (Vec<RawEntry>, Vec<String>) {
    let text = String::from_utf8_lossy(data);
    let mut entries = Vec::new();
    let mut warnings = Vec::new();

    for (line_no, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim_end_matches('\r');
        let Some(rest) = line.strip_prefix(&style.keyword) else {
            continue;
        };
        let rest = rest.trim_start();
        let Some((key, rest)) = take_group(rest, style) else {
            warnings.push(format!("line {}: malformed entry key", line_no + 1));
            continue;
        };
        let Some((page, _rest)) = take_group(rest.trim_start(), style) else {
            warnings.push(format!("line {}: malformed page argument", line_no + 1));
            continue;
        };

        let mut parts = split_top_level(&key, style.level, style.quote, style.escape);
        let encap = if let Some(last) = parts.last_mut() {
            let mut encap_parts = split_top_level(last, style.encap, style.quote, style.escape);
            if encap_parts.len() > 1 {
                let encap = encap_parts.pop().unwrap();
                *last = encap_parts.join(&style.encap.to_string());
                Some(decode(&encap, style.quote, style.escape))
            } else {
                None
            }
        } else {
            None
        };

        let levels: Vec<Level> = parts
            .iter()
            .map(|part| {
                let mut pieces = split_top_level(part, style.actual, style.quote, style.escape);
                let display = pieces.pop().unwrap_or_default();
                let sort = if pieces.is_empty() {
                    display.clone()
                } else {
                    pieces.join(&style.actual.to_string())
                };
                Level {
                    sort: decode(&sort, style.quote, style.escape),
                    display: decode(&display, style.quote, style.escape),
                }
            })
            .collect();

        if levels.is_empty() || levels.iter().all(|level| level.display.trim().is_empty()) {
            continue;
        }

        entries.push(RawEntry {
            levels,
            page: decode(&page, style.quote, style.escape),
            encap,
        });
    }

    (entries, warnings)
}

/// Consume `{...}` (or the style's configured delimiters) and return the inner
/// text plus the remainder.
fn take_group<'a>(input: &'a str, style: &Style) -> Option<(String, &'a str)> {
    let mut chars = input.char_indices();
    let (_, first) = chars.next()?;
    if first != style.arg_open {
        return None;
    }
    let open_len = style.arg_open.len_utf8();

    let mut depth = 1usize;
    let mut escaped = false;
    let mut quoted = false;
    for (idx, c) in chars {
        if escaped {
            escaped = false;
            continue;
        }
        if c == style.escape {
            escaped = true;
            continue;
        }
        if quoted {
            quoted = false;
            continue;
        }
        if c == style.quote {
            quoted = true;
            continue;
        }
        if c == style.arg_open {
            depth += 1;
        } else if c == style.arg_close {
            depth -= 1;
            if depth == 0 {
                let inner = &input[open_len..idx];
                return Some((inner.to_owned(), &input[idx + c.len_utf8()..]));
            }
        }
    }
    None
}

/// Split on a top-level separator, respecting quote and escape characters.
fn split_top_level(input: &str, sep: char, quote: char, escape: char) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let mut escaped = false;
    let mut quoted = false;

    for c in input.chars() {
        if escaped {
            current.push(c);
            escaped = false;
            continue;
        }
        if c == escape {
            current.push(c);
            escaped = true;
            continue;
        }
        if quoted {
            current.push(c);
            quoted = false;
            continue;
        }
        if c == quote {
            current.push(c);
            quoted = true;
            continue;
        }
        if c == sep {
            out.push(current);
            current = String::new();
        } else {
            current.push(c);
        }
    }
    out.push(current);
    out
}

/// Remove quote characters, but keep escape sequences for TeX.
fn decode(input: &str, quote: char, escape: char) -> String {
    let mut out = String::new();
    let mut escaped = false;
    for c in input.chars() {
        if escaped {
            out.push(c);
            escaped = false;
            continue;
        }
        if c == escape {
            out.push(c);
            escaped = true;
            continue;
        }
        if c == quote {
            continue;
        }
        out.push(c);
    }
    out
}
