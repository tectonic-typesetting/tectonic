// Copyright 2025 the Tectonic Project
// Licensed under the MIT License.

//! Page-number parsing, ordering, compression, and rendering.

use std::cmp::Ordering;

use crate::{idx::RawEntry, style::Style};

/// One page-list element for a rendered index entry.
#[derive(Clone, Debug)]
pub(crate) enum PageItem {
    Single(PageNumber, Option<String>),
    Range {
        start: PageNumber,
        end: PageNumber,
        encap: Option<String>,
        suffix: Option<String>,
    },
    CrossRef(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum PageKind {
    RomanLower,
    Arabic,
    AlphaLower,
    RomanUpper,
    AlphaUpper,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PageNumber {
    raw: String,
    kind: PageKind,
    value: i64,
}

/// Turn the raw page arguments for one entry into a sorted, compressed list.
pub(crate) fn pages_for(entries: &[RawEntry], style: &Style) -> (Vec<PageItem>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut items = Vec::new();
    let mut open_range: Option<(PageNumber, Option<String>, PageNumber)> = None;

    for entry in entries {
        let page = parse_page(&entry.page);
        let encap = entry.encap.clone();

        if let Some(encap_text) = encap.as_deref() {
            if encap_text.starts_with(style.range_open) {
                if let Some(page) = page {
                    let suffix = encap_text[style.range_open.len_utf8()..].to_owned();
                    let suffix = if suffix.is_empty() {
                        None
                    } else {
                        Some(suffix)
                    };
                    open_range = Some((page.clone(), suffix, page));
                } else {
                    warnings.push("explicit range opened without a page number".to_owned());
                }
                continue;
            }
            if encap_text.starts_with(style.range_close) {
                if let Some((start, suffix, last)) = open_range.take() {
                    let end = page.unwrap_or(last);
                    items.push(PageItem::Range {
                        start,
                        end,
                        encap: suffix,
                        suffix: None,
                    });
                } else {
                    warnings.push("explicit range closed without an opening".to_owned());
                }
                continue;
            }
            if encap_text.starts_with("see") {
                items.push(PageItem::CrossRef(encap_text.to_owned()));
                continue;
            }
        }

        let Some(page) = page else {
            continue;
        };
        if let Some((_, _, last)) = open_range.as_mut() {
            *last = page.clone();
        }
        items.push(PageItem::Single(page, encap));
    }

    if let Some((start, suffix, last)) = open_range {
        warnings.push("explicit range was not closed".to_owned());
        items.push(PageItem::Range {
            start,
            end: last,
            encap: suffix,
            suffix: None,
        });
    }

    (normalize_pages(items, style), warnings)
}

fn normalize_pages(items: Vec<PageItem>, style: &Style) -> Vec<PageItem> {
    let mut cross_refs = Vec::new();
    let mut singles: Vec<(PageNumber, Option<String>)> = Vec::new();
    let mut ranges = Vec::new();

    for item in items {
        match item {
            PageItem::Single(page, encap) => singles.push((page, encap)),
            PageItem::Range {
                start,
                end,
                encap,
                suffix,
            } => ranges.push(PageItem::Range {
                start,
                end,
                encap,
                suffix,
            }),
            PageItem::CrossRef(command) => cross_refs.push(PageItem::CrossRef(command)),
        }
    }

    singles.sort_by(|a, b| compare_pages(&a.0, &b.0, style));
    singles.dedup();

    let mut compressed = Vec::new();
    let mut index = 0;
    while index < singles.len() {
        let (page, encap) = &singles[index];
        let mut end = index + 1;
        while end < singles.len() {
            let (next, next_encap) = &singles[end];
            if next_encap != encap
                || next.kind != page.kind
                || next.value != page.value + (end - index) as i64
            {
                break;
            }
            end += 1;
        }

        let run = end - index;
        if run >= 3 {
            let suffix = if !style.suffix_mp.is_empty() {
                Some(style.suffix_mp.clone())
            } else if run == 3 && !style.suffix_3p.is_empty() {
                Some(style.suffix_3p.clone())
            } else {
                None
            };
            compressed.push(PageItem::Range {
                start: page.clone(),
                end: singles[end - 1].0.clone(),
                encap: encap.clone(),
                suffix,
            });
            index = end;
        } else if run == 2 && !style.suffix_2p.is_empty() {
            compressed.push(PageItem::Range {
                start: page.clone(),
                end: singles[index + 1].0.clone(),
                encap: encap.clone(),
                suffix: Some(style.suffix_2p.clone()),
            });
            index += 2;
        } else {
            for (page, encap) in &singles[index..end] {
                compressed.push(PageItem::Single(page.clone(), encap.clone()));
            }
            index = end;
        }
    }

    compressed.extend(ranges);
    compressed.sort_by(|a, b| compare_items(a, b, style));
    compressed.extend(cross_refs);
    compressed
}

/// Render the page list for one entry at the given nesting level.
pub(crate) fn render_pages(items: &[PageItem], style: &Style, level: usize) -> String {
    if items.is_empty() {
        return String::new();
    }
    let delimiter = match level {
        0 => &style.delim_0,
        1 => &style.delim_1,
        _ => &style.delim_2,
    };
    let rendered: Vec<String> = items.iter().map(|item| render_item(item, style)).collect();
    format!(
        "{}{}{}",
        delimiter,
        rendered.join(&style.delim_n),
        style.delim_t
    )
}

fn render_item(item: &PageItem, style: &Style) -> String {
    match item {
        PageItem::Single(page, encap) => wrap_encap(&page.raw, encap.as_deref(), style),
        PageItem::Range {
            start,
            end,
            encap,
            suffix,
        } => {
            let body = match suffix {
                Some(suffix) => format!("{}{}", start.raw, suffix),
                None => format!("{}{}{}", start.raw, style.delim_r, end.raw),
            };
            wrap_encap(&body, encap.as_deref(), style)
        }
        PageItem::CrossRef(command) => {
            if command.starts_with("see") {
                format!("{}{}", style.encap_prefix, command)
            } else {
                wrap_encap("", Some(command), style)
            }
        }
    }
}

fn wrap_encap(text: &str, encap: Option<&str>, style: &Style) -> String {
    match encap {
        None | Some("") => text.to_owned(),
        Some(encap) => format!(
            "{}{}{}{}{}",
            style.encap_prefix, encap, style.encap_infix, text, style.encap_suffix
        ),
    }
}

fn compare_items(a: &PageItem, b: &PageItem, style: &Style) -> Ordering {
    match (a, b) {
        (PageItem::CrossRef(_), PageItem::CrossRef(_)) => Ordering::Equal,
        (PageItem::CrossRef(_), _) => Ordering::Greater,
        (_, PageItem::CrossRef(_)) => Ordering::Less,
        _ => compare_pages(start_page(a), start_page(b), style),
    }
}

fn start_page(item: &PageItem) -> &PageNumber {
    match item {
        PageItem::Single(page, _) => page,
        PageItem::Range { start, .. } => start,
        PageItem::CrossRef(_) => unreachable!("cross references are compared separately"),
    }
}

fn compare_pages(a: &PageNumber, b: &PageNumber, style: &Style) -> Ordering {
    page_precedence(&a.kind, style)
        .cmp(&page_precedence(&b.kind, style))
        .then_with(|| a.value.cmp(&b.value))
        .then_with(|| a.raw.cmp(&b.raw))
}

fn page_precedence(kind: &PageKind, style: &Style) -> usize {
    let letter = match kind {
        PageKind::RomanLower => 'r',
        PageKind::Arabic => 'n',
        PageKind::AlphaLower => 'a',
        PageKind::RomanUpper => 'R',
        PageKind::AlphaUpper => 'A',
        PageKind::Other => '?',
    };
    style.page_precedence.find(letter).unwrap_or(usize::MAX)
}

fn parse_page(raw: &str) -> Option<PageNumber> {
    let raw = raw.trim().trim_matches(|c| c == '{' || c == '}').trim();
    if raw.is_empty() || raw.contains("--") {
        return None;
    }
    if raw.chars().all(|c| c.is_ascii_digit()) {
        return Some(PageNumber {
            raw: raw.to_owned(),
            kind: PageKind::Arabic,
            value: raw.parse().unwrap_or(0),
        });
    }
    if raw.chars().all(|c| "ivxlcdm".contains(c)) {
        return Some(PageNumber {
            raw: raw.to_owned(),
            kind: PageKind::RomanLower,
            value: roman_value(raw),
        });
    }
    if raw.chars().all(|c| "IVXLCDM".contains(c)) {
        return Some(PageNumber {
            raw: raw.to_owned(),
            kind: PageKind::RomanUpper,
            value: roman_value(&raw.to_ascii_lowercase()),
        });
    }
    if raw.chars().all(|c| c.is_ascii_lowercase()) {
        return Some(PageNumber {
            raw: raw.to_owned(),
            kind: PageKind::AlphaLower,
            value: alpha_value(raw),
        });
    }
    if raw.chars().all(|c| c.is_ascii_uppercase()) {
        return Some(PageNumber {
            raw: raw.to_owned(),
            kind: PageKind::AlphaUpper,
            value: alpha_value(&raw.to_ascii_lowercase()),
        });
    }
    Some(PageNumber {
        raw: raw.to_owned(),
        kind: PageKind::Other,
        value: 0,
    })
}

fn roman_value(raw: &str) -> i64 {
    let mut total = 0;
    let mut previous = 0;
    for c in raw.chars().rev() {
        let value = match c {
            'i' => 1,
            'v' => 5,
            'x' => 10,
            'l' => 50,
            'c' => 100,
            'd' => 500,
            'm' => 1000,
            _ => 0,
        };
        if value < previous {
            total -= value;
        } else {
            total += value;
            previous = value;
        }
    }
    total
}

fn alpha_value(raw: &str) -> i64 {
    raw.chars()
        .fold(0, |acc, c| acc * 26 + i64::from(c as u8 - b'a' + 1))
}
