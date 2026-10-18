// Copyright 2025 the Tectonic Project
// Licensed under the MIT License.

//! Parsing of makeindex `.ist` style files.
//!
//! Only the directives that affect the standard LaTeX/nomencl pipelines are
//! interpreted. Unknown directives are ignored, matching makeindex's
//! forward-compatible behavior.

/// The input-syntax characters and output templates used by makeindex.
#[derive(Clone, Debug)]
pub(crate) struct Style {
    pub keyword: String,
    pub quote: char,
    pub actual: char,
    pub level: char,
    pub encap: char,
    pub escape: char,
    pub arg_open: char,
    pub arg_close: char,
    pub range_open: char,
    pub range_close: char,
    pub page_compositor: String,

    pub preamble: String,
    pub postamble: String,
    pub group_skip: String,
    pub headings_flag: i32,
    pub heading_prefix: String,
    pub heading_suffix: String,
    pub symhead_positive: String,
    pub symhead_negative: String,
    pub numhead_positive: String,
    pub numhead_negative: String,
    pub lethead_flag: i32,
    pub lethead_prefix: String,
    pub lethead_suffix: String,
    pub item_0: String,
    pub item_1: String,
    pub item_2: String,
    pub item_01: String,
    pub item_x1: String,
    pub item_12: String,
    pub item_x2: String,
    pub delim_0: String,
    pub delim_1: String,
    pub delim_2: String,
    pub delim_n: String,
    pub delim_r: String,
    pub delim_t: String,
    pub encap_prefix: String,
    pub encap_infix: String,
    pub encap_suffix: String,
    pub line_max: usize,
    pub indent_space: String,
    pub indent_length: usize,
    pub suffix_2p: String,
    pub suffix_3p: String,
    pub suffix_mp: String,
    pub page_precedence: String,
    pub setpage_prefix: String,
    pub setpage_suffix: String,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            keyword: "\\indexentry".to_owned(),
            quote: '"',
            actual: '@',
            level: '!',
            encap: '|',
            escape: '\\',
            arg_open: '{',
            arg_close: '}',
            range_open: '(',
            range_close: ')',
            page_compositor: "-".to_owned(),

            preamble: "\\begin{theindex}\n".to_owned(),
            postamble: "\n\n\\end{theindex}\n".to_owned(),
            group_skip: "\n\n  \\indexspace\n".to_owned(),
            headings_flag: 0,
            heading_prefix: String::new(),
            heading_suffix: String::new(),
            symhead_positive: "Symbols".to_owned(),
            symhead_negative: "symbols".to_owned(),
            numhead_positive: "Numbers".to_owned(),
            numhead_negative: "numbers".to_owned(),
            lethead_flag: 0,
            lethead_prefix: String::new(),
            lethead_suffix: String::new(),
            item_0: "\n  \\item ".to_owned(),
            item_1: "\n    \\subitem ".to_owned(),
            item_2: "\n      \\subsubitem ".to_owned(),
            item_01: "\n    \\subitem ".to_owned(),
            item_x1: "\n    \\subitem ".to_owned(),
            item_12: "\n      \\subsubitem ".to_owned(),
            item_x2: "\n      \\subsubitem ".to_owned(),
            delim_0: ", ".to_owned(),
            delim_1: ", ".to_owned(),
            delim_2: ", ".to_owned(),
            delim_n: ", ".to_owned(),
            delim_r: "--".to_owned(),
            delim_t: String::new(),
            encap_prefix: "\\".to_owned(),
            encap_infix: "{".to_owned(),
            encap_suffix: "}".to_owned(),
            line_max: 72,
            indent_space: "\t\t".to_owned(),
            indent_length: 16,
            suffix_2p: String::new(),
            suffix_3p: String::new(),
            suffix_mp: String::new(),
            page_precedence: "rnaRA".to_owned(),
            setpage_prefix: "\n  \\setcounter{page}{".to_owned(),
            setpage_suffix: "}\n".to_owned(),
        }
    }
}

impl Style {
    /// Parse a style file. Invalid or unknown directives are ignored.
    pub(crate) fn parse(data: &[u8]) -> Style {
        let mut style = Style::default();
        let text = String::from_utf8_lossy(data);

        for raw_line in text.lines() {
            let line = raw_line.trim_start();
            if line.is_empty() || line.starts_with('%') {
                continue;
            }
            let Some(split) = line.find(char::is_whitespace) else {
                continue;
            };
            let key = &line[..split];
            let rest = line[split..].trim_start();

            if let Some(value) = quoted_value(rest) {
                style.set_string(key, value);
            } else if let Some(value) = char_value(rest) {
                style.set_char(key, value);
            } else if let Ok(value) = rest.parse::<i32>() {
                style.set_int(key, value);
            }
        }

        style
    }

    fn set_string(&mut self, key: &str, value: String) {
        match key {
            "keyword" => self.keyword = value,
            "page_compositor" => self.page_compositor = value,
            "preamble" => self.preamble = value,
            "postamble" => self.postamble = value,
            "group_skip" => self.group_skip = value,
            "heading_prefix" => self.heading_prefix = value,
            "heading_suffix" => self.heading_suffix = value,
            "symhead_positive" => self.symhead_positive = value,
            "symhead_negative" => self.symhead_negative = value,
            "numhead_positive" => self.numhead_positive = value,
            "numhead_negative" => self.numhead_negative = value,
            "lethead_prefix" => self.lethead_prefix = value,
            "lethead_suffix" => self.lethead_suffix = value,
            "item_0" => self.item_0 = value,
            "item_1" => self.item_1 = value,
            "item_2" => self.item_2 = value,
            "item_01" => self.item_01 = value,
            "item_x1" => self.item_x1 = value,
            "item_12" => self.item_12 = value,
            "item_x2" => self.item_x2 = value,
            "delim_0" => self.delim_0 = value,
            "delim_1" => self.delim_1 = value,
            "delim_2" => self.delim_2 = value,
            "delim_n" => self.delim_n = value,
            "delim_r" => self.delim_r = value,
            "delim_t" => self.delim_t = value,
            "encap_prefix" => self.encap_prefix = value,
            "encap_infix" => self.encap_infix = value,
            "encap_suffix" => self.encap_suffix = value,
            "indent_space" => self.indent_space = value,
            "suffix_2p" => self.suffix_2p = value,
            "suffix_3p" => self.suffix_3p = value,
            "suffix_mp" => self.suffix_mp = value,
            "page_precedence" => self.page_precedence = value,
            "setpage_prefix" => self.setpage_prefix = value,
            "setpage_suffix" => self.setpage_suffix = value,
            _ => {}
        }
    }

    fn set_char(&mut self, key: &str, value: char) {
        match key {
            "quote" => self.quote = value,
            "actual" => self.actual = value,
            "level" => self.level = value,
            "encap" => self.encap = value,
            "escape" => self.escape = value,
            "arg_open" => self.arg_open = value,
            "arg_close" => self.arg_close = value,
            "range_open" => self.range_open = value,
            "range_close" => self.range_close = value,
            _ => {}
        }
    }

    fn set_int(&mut self, key: &str, value: i32) {
        match key {
            "headings_flag" => self.headings_flag = value,
            "lethead_flag" => self.lethead_flag = value,
            "line_max" => self.line_max = value.max(0) as usize,
            "indent_length" => self.indent_length = value.max(0) as usize,
            _ => {}
        }
    }
}

/// Extract a double-quoted value, processing makeindex's small escape set.
fn quoted_value(input: &str) -> Option<String> {
    let mut chars = input.chars();
    if chars.next()? != '"' {
        return None;
    }
    let mut out = String::new();
    let mut escaped = false;
    for c in chars {
        if escaped {
            match c {
                'n' => out.push('\n'),
                't' => out.push('\t'),
                '\\' => out.push('\\'),
                '"' => out.push('"'),
                '{' => out.push('{'),
                '}' => out.push('}'),
                '%' => out.push('%'),
                other => {
                    out.push('\\');
                    out.push(other);
                }
            }
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '"' {
            return Some(out);
        } else {
            out.push(c);
        }
    }
    Some(out)
}

/// Extract a single-quoted character value.
fn char_value(input: &str) -> Option<char> {
    let mut chars = input.chars();
    if chars.next()? != '\'' {
        return None;
    }
    let c = chars.next()?;
    let c = if c == '\\' { chars.next()? } else { c };
    Some(c)
}

#[cfg(test)]
mod tests {
    use super::Style;

    #[test]
    fn unescapes_style_strings() {
        let style = Style::parse(
            br#"
delim_0 "\{\\glossaryentrynumbers\{\relax "
postamble "\%\n"
heading_prefix "\\glsgroupheading\{"
"#,
        );
        assert_eq!(style.delim_0, "{\\glossaryentrynumbers{\\relax ");
        assert_eq!(style.postamble, "%\n");
        assert_eq!(style.heading_prefix, "\\glsgroupheading{");
    }
}
