// Copyright 2025 the Tectonic Project
// Licensed under the MIT License.

//! Build the nested index tree and render a `.ind` file.

use std::{cmp::Ordering, collections::BTreeMap};

use crate::{
    idx::{Level, RawEntry},
    page::{pages_for, render_pages, PageItem},
    sort,
    style::Style,
};

/// The result of one makeindex run.
pub(crate) struct Generated {
    pub ind: String,
    pub warnings: Vec<String>,
}

/// The identity of a node in the index tree. The custom ordering uses
/// makeindex's word-ordering rules rather than plain lexicographic ordering.
#[derive(Clone, Debug)]
struct NodeKey {
    sort: String,
    display: String,
}

impl NodeKey {
    fn new(level: &Level) -> Self {
        NodeKey {
            sort: level.sort.clone(),
            display: level.display.clone(),
        }
    }
}

impl PartialEq for NodeKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for NodeKey {}

impl Ord for NodeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        sort::compare(&self.sort, &other.sort).then_with(|| self.display.cmp(&other.display))
    }
}

impl PartialOrd for NodeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Default)]
struct Node {
    pages: Vec<PageItem>,
    children: BTreeMap<NodeKey, Node>,
}

/// Generate the output index.
pub(crate) fn generate(entries: Vec<RawEntry>, style: &Style) -> Generated {
    let mut warnings = Vec::new();
    let mut root = BTreeMap::new();

    for (levels, group) in group_entries(entries) {
        let (pages, mut group_warnings) = pages_for(&group, style);
        warnings.append(&mut group_warnings);
        insert_node(&mut root, &levels, 0, pages);
    }

    let mut out = style.preamble.clone();
    let mut previous_group = None;
    render_nodes(&mut out, &root, style, 0, &mut previous_group, false);
    out.push_str(&style.postamble);

    Generated { ind: out, warnings }
}

fn group_entries(entries: Vec<RawEntry>) -> Vec<(Vec<Level>, Vec<RawEntry>)> {
    let mut groups: BTreeMap<Vec<Level>, Vec<RawEntry>> = BTreeMap::new();
    for entry in entries {
        groups.entry(entry.levels.clone()).or_default().push(entry);
    }
    groups.into_iter().collect()
}

fn insert_node(
    nodes: &mut BTreeMap<NodeKey, Node>,
    levels: &[Level],
    index: usize,
    pages: Vec<PageItem>,
) {
    let Some(level) = levels.get(index) else {
        return;
    };
    let node = nodes.entry(NodeKey::new(level)).or_default();
    if index + 1 == levels.len() {
        node.pages = pages;
    } else {
        insert_node(&mut node.children, levels, index + 1, pages);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Group {
    Symbols,
    Numbers,
    Letter(char),
}

fn group_of(sort_key: &str) -> Group {
    for c in sort_key.chars() {
        if c.is_ascii_digit() {
            return Group::Numbers;
        }
        if c.is_alphabetic() {
            return Group::Letter(c.to_ascii_uppercase());
        }
        if !c.is_whitespace() {
            return Group::Symbols;
        }
    }
    Group::Symbols
}

fn group_heading(group: Group, style: &Style) -> Option<String> {
    if style.headings_flag == 0 {
        return None;
    }
    let positive = style.headings_flag > 0;
    let heading = match group {
        Group::Symbols => {
            if positive {
                &style.symhead_positive
            } else {
                &style.symhead_negative
            }
        }
        Group::Numbers => {
            if positive {
                &style.numhead_positive
            } else {
                &style.numhead_negative
            }
        }
        Group::Letter(c) => {
            let c = if positive { c } else { c.to_ascii_lowercase() };
            let heading = c.to_string();
            return Some(if style.lethead_flag != 0 {
                format!(
                    "{}{}{}",
                    style.lethead_prefix, heading, style.lethead_suffix
                )
            } else {
                format!(
                    "{}{}{}",
                    style.heading_prefix, heading, style.heading_suffix
                )
            });
        }
    };
    Some(if style.lethead_flag != 0 {
        format!(
            "{}{}{}",
            style.lethead_prefix, heading, style.lethead_suffix
        )
    } else {
        format!(
            "{}{}{}",
            style.heading_prefix, heading, style.heading_suffix
        )
    })
}

fn render_nodes(
    out: &mut String,
    nodes: &BTreeMap<NodeKey, Node>,
    style: &Style,
    level: usize,
    previous_group: &mut Option<Group>,
    parent_has_pages: bool,
) {
    for (index, (key, node)) in nodes.iter().enumerate() {
        if level == 0 {
            let group = group_of(&key.sort);
            if previous_group.is_some() && *previous_group != Some(group) {
                out.push_str(&style.group_skip);
                if let Some(heading) = group_heading(group, style) {
                    out.push_str(&heading);
                }
            } else if previous_group.is_none() {
                if let Some(heading) = group_heading(group, style) {
                    // The first heading is emitted after the preamble without
                    // a leading group skip, matching makeindex's output.
                    out.push_str(&heading);
                }
            }
            *previous_group = Some(group);
        }

        out.push_str(item_command(style, level, index, parent_has_pages));
        out.push_str(&key.display);
        out.push_str(&render_pages(&node.pages, style, level));

        if !node.children.is_empty() {
            render_nodes(
                out,
                &node.children,
                style,
                level + 1,
                previous_group,
                !node.pages.is_empty(),
            );
        }
    }
}

fn item_command(style: &Style, level: usize, index: usize, parent_has_pages: bool) -> &str {
    match (level, index) {
        (0, _) => &style.item_0,
        (1, 0) if parent_has_pages => &style.item_01,
        (1, 0) => &style.item_x1,
        (1, _) => &style.item_1,
        (2, 0) if parent_has_pages => &style.item_12,
        (2, 0) => &style.item_x2,
        _ => &style.item_2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(key: &str, page: &str, encap: Option<&str>) -> RawEntry {
        RawEntry {
            levels: key
                .split('!')
                .map(|level| Level {
                    sort: level.to_owned(),
                    display: level.to_owned(),
                })
                .collect(),
            page: page.to_owned(),
            encap: encap.map(str::to_owned),
        }
    }

    #[test]
    fn renders_simple_index() {
        let style = Style::default();
        let generated = generate(
            vec![
                entry("alpha", "1", None),
                entry("alpha!beta", "3", None),
                entry("alpha!phi", "5", None),
                entry("alpha!phi!sigma", "6", None),
                entry("alpha!phi!sigma", "12", None),
                entry("alpha!phi!sigma", "13", None),
                entry("alpha!phi!sigma", "14", None),
            ],
            &style,
        );
        assert!(generated.ind.contains("\\item alpha, 1"));
        assert!(generated.ind.contains("\\subitem beta, 3"));
        assert!(generated.ind.contains("\\subsubitem sigma, 6, 12--14"));
    }

    #[test]
    fn renders_explicit_range_and_encap() {
        let style = Style::default();
        let generated = generate(
            vec![
                entry("random text", "2", Some("bold")),
                entry("random text", "9", Some("(")),
                entry("random text", "11", Some(")")),
            ],
            &style,
        );
        assert!(generated.ind.contains("\\bold{2}"));
        assert!(generated.ind.contains("9--11"));
    }

    #[test]
    fn uses_transition_commands_based_on_parent_pages() {
        let style = Style {
            item_0: "<0>".to_owned(),
            item_1: "<1>".to_owned(),
            item_2: "<2>".to_owned(),
            item_01: "<01>".to_owned(),
            item_x1: "<x1>".to_owned(),
            item_12: "<12>".to_owned(),
            item_x2: "<x2>".to_owned(),
            ..Style::default()
        };
        let generated = generate(
            vec![
                entry("a", "1", None),
                entry("a!b", "2", None),
                entry("a!c", "3", None),
                entry("a!b!d", "4", None),
                entry("a!b!e", "5", None),
                entry("x!y!z", "6", None),
            ],
            &style,
        );

        assert!(generated.ind.contains("<01>a!b") || generated.ind.contains("<01>"));
        assert!(generated.ind.contains("<1>"));
        assert!(generated.ind.contains("<12>"));
        assert!(generated.ind.contains("<2>"));
        assert!(generated.ind.contains("<x1>"));
        assert!(generated.ind.contains("<x2>"));
    }
}
