// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

//! The main "emission" phase of SPX to HTML processing.

use std::{
    collections::HashMap,
    fmt::{Arguments, Error as FmtError, Write as FmtWrite},
    result::Result as StdResult,
};
use tectonic_errors::prelude::*;
use tectonic_status_base::tt_warning;

use crate::{
    assets::Assets,
    finalization::FinalizingState,
    fonts::{FamilyRelativeFontId, FontEnsemble, FontFamilyAnalysis, PathToNewFont},
    html::Element,
    specials::Special,
    templating::Templating,
    Common, FixedPoint, TexFontNum,
};

#[derive(Debug)]
pub(crate) struct EmittingState {
    fonts: FontEnsemble,
    content: ContentState,
    templating: Templating,
    assets: Assets,
    tag_associations: HashMap<Element, TexFontNum>,
    rems_per_tex: f32,
    elem_stack: Vec<ElementState>,
    current_canvas: Option<CanvasState>,
}

#[derive(Debug, Default)]
struct ContentState {
    current_content: String,
    last_content_x: i32,
    last_content_space_width: Option<FixedPoint>,
}

impl ContentState {
    fn is_empty(&self) -> bool {
        self.current_content.is_empty()
    }

    fn push_str(&mut self, text: &str) {
        self.current_content.push_str(text);
    }

    fn push_char(&mut self, ch: char) {
        self.current_content.push(ch);
    }

    fn push_close_tag(&mut self, tag: &str) {
        self.current_content.push('<');
        self.current_content.push('/');
        self.current_content.push_str(tag);
        self.current_content.push('>');
    }

    fn push_with_html_escaping<S: AsRef<str>>(&mut self, raw_text: S) {
        html_escape::encode_safe_to_string(raw_text, &mut self.current_content);
    }

    fn push_with_html_double_quoted_attribute_escaping<S: AsRef<str>>(&mut self, raw_text: S) {
        html_escape::encode_double_quoted_attribute_to_string(raw_text, &mut self.current_content);
    }

    fn push_with_html_unquoted_attribute_escaping<S: AsRef<str>>(&mut self, raw_text: S) {
        html_escape::encode_unquoted_attribute_to_string(raw_text, &mut self.current_content);
    }

    fn take(&mut self) -> String {
        std::mem::take(&mut self.current_content)
    }

    /// Figure out if we need to push a space into the text content right now.
    fn is_space_needed(
        &self,
        x0: i32,
        cur_space_width: Option<FixedPoint>,
        do_auto_spaces: bool,
    ) -> bool {
        // We never want a leading space.
        if self.current_content.is_empty() {
            return false;
        }

        // Auto-spaces can be disabled.
        if !do_auto_spaces {
            return false;
        }

        // TODO: RTL ASSUMPTION!!!!!
        //
        // If the "next" x is smaller than the last one, assume that we've
        // started a new line. We ignore Y values since those are going to
        // get hairy with subscripts, etc.

        if x0 < self.last_content_x {
            return true;
        }

        // Check the advance against the size of the space, which can be
        // determined from either the most recent content or the new content,
        // since in various circumstances either one or the other might not
        // be defined. If both are defined, use whatever's smaller. There's
        // probably a smoother way to do this logic?

        let space_width = match (&self.last_content_space_width, &cur_space_width) {
            (Some(w1), Some(w2)) => FixedPoint::min(*w1, *w2),
            (Some(w), None) => *w,
            (None, Some(w)) => *w,
            (None, None) => 0,
        };

        // If the x difference is larger than 1/4 of the space_width, let's say that
        // we need a space. I made up the 1/4.
        4 * (x0 - self.last_content_x) > space_width
    }

    fn update_content_pos(&mut self, x: i32, cur_space_width: Option<FixedPoint>) {
        self.last_content_x = x;

        if cur_space_width.is_some() {
            self.last_content_space_width = cur_space_width;
        }
    }

    /// Maybe push a space into the text content right now, if we think we need one.
    fn push_space_if_needed(
        &mut self,
        x0: i32,
        cur_space_width: Option<FixedPoint>,
        do_auto_spaces: bool,
    ) {
        if self.is_space_needed(x0, cur_space_width, do_auto_spaces) {
            self.current_content.push(' ');
        }

        // This parameter should be updated almost-instantaneously
        // if a run of glyphs is being rendered, but this is a good start:
        self.update_content_pos(x0, cur_space_width);
    }
}

impl FmtWrite for ContentState {
    fn write_str(&mut self, s: &str) -> StdResult<(), FmtError> {
        self.current_content.write_str(s)
    }

    fn write_char(&mut self, c: char) -> StdResult<(), FmtError> {
        self.current_content.write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> StdResult<(), FmtError> {
        self.current_content.write_fmt(args)
    }
}

#[derive(Debug)]
pub(crate) struct ElementState {
    /// The associated HTML element. This is None for the bottom item in the
    /// stack, or for changes in state that are not associated with actual HTML
    /// tags.
    elem: Option<Element>,

    /// The origin of this element/state-change.
    origin: ElementOrigin,

    /// Whether HTML tags that are automatically generated by the TeX
    /// engine, such as <p> and </p> at the start and end of paragraphs,
    /// should be emitted (true) or ignored (false).
    do_auto_tags: bool,

    /// Whether this library should automatically insert spaces into text
    /// content. This is done by looking at the horizontal positions of
    /// different runs of text and applying a threshold for the amount of space
    /// between the end of the previous one and the start of the next one.
    do_auto_spaces: bool,

    /// The font-num of the regular font associated with the current font
    /// family. This code is currently only exercised with a single "font
    /// family" defined in a document, but there could be multiple.
    font_family_id: TexFontNum,

    /// The currently active font, as we understand it, relative to the
    /// currently active font family.
    active_font: FamilyRelativeFontId,
}

impl ElementState {
    /// Should this element automatically be closed if a new tag starts or ends?
    fn is_auto_close(&self) -> bool {
        matches!(self.origin, ElementOrigin::FontAuto)
    }
}

/// How a particular ElementState ended up on the stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ElementOrigin {
    /// This is the root element in our stack.
    Root,

    /// The element was manually inserted by the TeX code.
    Manual,

    /// The element was automatically inserted by the TeX engine.
    EngineAuto,

    /// The element was automatically inserted by us to
    /// activate the desired font.
    FontAuto,
}

#[derive(Debug)]
struct CanvasState {
    kind: String,
    depth: usize,
    x0: i32,
    y0: i32,
    glyphs: Vec<GlyphInfo>,
}

impl CanvasState {
    fn new(kind: &str, x0: i32, y0: i32) -> Self {
        CanvasState {
            kind: kind.to_owned(),
            depth: 1,
            x0,
            y0,
            glyphs: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct GlyphInfo {
    dx: i32,
    dy: i32,
    font_num: TexFontNum,
    glyph: u16,
}

impl EmittingState {
    pub(crate) fn new_from_init(
        fonts: FontEnsemble,
        main_body_font_num: Option<TexFontNum>,
        templating: Templating,
        tag_associations: HashMap<Element, TexFontNum>,
    ) -> Result<Self> {
        let rems_per_tex = 1.0
            / main_body_font_num
                .map(|fnum| fonts.get_font_size(fnum))
                .unwrap_or(65536) as f32;

        Ok(EmittingState {
            templating,
            fonts,
            tag_associations,
            rems_per_tex,
            content: Default::default(),
            assets: Default::default(),
            elem_stack: vec![ElementState {
                elem: None,
                origin: ElementOrigin::Root,
                do_auto_tags: true,
                do_auto_spaces: true,
                font_family_id: main_body_font_num.unwrap_or_default(),
                active_font: FamilyRelativeFontId::Regular,
            }],
            current_canvas: None,
        })
    }

    /// Convenience helper that applies the right defaults here.
    ///
    /// We can't always use this function because sometimes we need mutable
    /// access to the `fonts` and `content` items separately.
    fn push_space_if_needed(&mut self, x0: i32, fnum: Option<TexFontNum>) {
        let cur_space_width = self.fonts.maybe_get_font_space_width(fnum);
        self.content
            .push_space_if_needed(x0, cur_space_width, self.cur_elstate().do_auto_spaces);
    }

    fn create_elem(&self, name: &str, is_start: bool, common: &mut Common) -> Element {
        // Parsing can never fail since we offer an `Other` element type
        let el: Element = name.parse().unwrap();

        if el.is_deprecated() {
            tt_warning!(
                common.status,
                "HTML element `{}` is deprecated; templates should be updated to avoid it",
                name
            );
        }

        if is_start && el.is_empty() {
            tt_warning!(
                common.status,
                "HTML element `{}` is an empty element; insert it with `tdux:mfe`, not as a start-tag",
                name
            );
        }

        if let Some(cur) = self.cur_elstate().elem.as_ref() {
            if cur.is_autoclosed_by(&el) {
                tt_warning!(
                    common.status,
                    "currently open HTML element `{}` will be implicitly closed by new \
                    element `{}`; explicit closing tags are strongly encouraged",
                    cur.name(),
                    name
                );
            }
        }

        el
    }

    #[inline(always)]
    fn cur_elstate(&self) -> &ElementState {
        self.elem_stack.last().unwrap()
    }

    /// Close the topmost element in the stack.
    fn close_one(&mut self) {
        // Refuse the close the root element
        if self.elem_stack.len() > 1 {
            let cur = self.elem_stack.pop().unwrap();

            if let Some(e) = cur.elem.as_ref() {
                self.content.push_close_tag(e.name());
            }
        }
    }

    /// Close an auto-close elements that are currently at the top of the stack.
    /// These elements are things like <b> tags that were automatically
    /// generated with the detection of the use of the bold font face.
    fn close_automatics(&mut self) {
        while self.elem_stack.len() > 1 {
            let close_it = self.cur_elstate().is_auto_close();

            if close_it {
                self.close_one();
            } else {
                break;
            }
        }
    }

    fn push_elem(&mut self, el: Element, origin: ElementOrigin) {
        self.close_automatics();

        let new_item = {
            let cur = self.cur_elstate();

            let font_family_id = self
                .tag_associations
                .get(&el)
                .copied()
                .unwrap_or(cur.font_family_id);

            ElementState {
                elem: Some(el),
                origin,
                font_family_id,
                ..*cur
            }
        };

        self.elem_stack.push(new_item);
    }

    /// TODO: may need to hone semantics when element nesting isn't as expected.
    fn pop_elem(&mut self, name: &str, common: &mut Common) {
        self.close_automatics();

        let mut n_closed = 0;

        while self.elem_stack.len() > 1 {
            let cur = self.elem_stack.pop().unwrap();

            if let Some(e) = cur.elem.as_ref() {
                self.content.push_close_tag(e.name());
                n_closed += 1;

                if e.name() == name {
                    break;
                }
            }
        }

        if n_closed != 1 {
            tt_warning!(
                common.status,
                "imbalanced tags; had to close {} to find `{}`",
                n_closed,
                name
            );
        }
    }

    pub(crate) fn handle_special(
        &mut self,
        x: i32,
        y: i32,
        special: Special<'_>,
        common: &mut Common,
    ) -> Result<()> {
        match special {
            Special::AutoStartParagraph => {
                if self.cur_elstate().do_auto_tags {
                    // Why are we using <div>s instead of <p>? As the HTML spec
                    // emphasizes, <p> tags are structural, not semantic. You cannot
                    // put tags like <ul> or <div> inside <p> -- they automatically
                    // close the paragraph. This does not align with TeX's idea of a
                    // paragraph, and there's no upside to trying to use <p>'s -- as
                    // the spec notes, the <p> tag does not activate any important
                    // semantics itself. The HTML spec explicitly recommends that
                    // you can use <div> elements to group logical paragraphs. So
                    // that's what we do.
                    let el = self.create_elem("div", true, common);
                    self.push_space_if_needed(x, None);
                    self.content.push_str("<div class=\"tdux-p\">");
                    self.push_elem(el, ElementOrigin::EngineAuto);
                }
                Ok(())
            }

            Special::AutoEndParagraph => {
                if self.cur_elstate().do_auto_tags {
                    self.pop_elem("div", common);
                }
                Ok(())
            }

            Special::CanvasStart(kind) => {
                if let Some(canvas) = self.current_canvas.as_mut() {
                    canvas.depth += 1;
                } else {
                    self.current_canvas = Some(CanvasState::new(kind, x, y));
                }
                Ok(())
            }

            Special::CanvasEnd(kind) => {
                if let Some(canvas) = self.current_canvas.as_mut() {
                    canvas.depth -= 1;
                    if canvas.depth == 0 {
                        self.handle_end_canvas(common)?;
                    }
                } else {
                    tt_warning!(
                        common.status,
                        "ignoring unpaired tdux:c[anvas]e[nd] special for `{}`",
                        kind
                    );
                }
                Ok(())
            }

            Special::ManualFlexibleStart(spec) => {
                self.handle_flexible_start_tag(x, y, spec, common)
            }

            Special::ManualEnd(tag) => {
                self.pop_elem(tag, common);
                Ok(())
            }

            Special::DirectText(text) => {
                self.content.push_with_html_escaping(text);
                Ok(())
            }

            Special::Emit => self.finish_file(common),

            Special::SetTemplate(path) => {
                self.templating.handle_set_template(path);
                Ok(())
            }

            Special::SetOutputPath(path) => {
                self.templating.handle_set_output_path(path);
                Ok(())
            }

            Special::SetTemplateVariable(spec) => {
                self.templating.handle_set_template_variable(spec, common)
            }

            Special::ProvideFile(_) | Special::ProvideSpecial(_) => {
                self.assets.try_handle_special(special, common);
                Ok(())
            }

            other => {
                tt_warning!(common.status, "ignoring unrecognized special: {}", other);
                Ok(())
            }
        }
    }

    /// Handle a "flexible" start tag.
    ///
    /// These start tags are built with a line-oriented structure that aims to
    /// make it so that the TeX code doesn't have to worry too much about
    /// escaping, etc. The general format is:
    ///
    /// ```notest
    /// \special{tdux:mfs tagname
    /// Cclass % add a CSS class
    /// Sname value % Add a CSS setting in the style attr
    /// Uname value % Add an unquoted attribute
    /// Dname value % Add a double-quoted attribute
    /// NAS % Turn off automatic space insertion while processing this tag
    /// NAT % Turn off automatic tag insertion while processing this tag
    /// }
    /// ```
    ///
    /// More ...
    fn handle_flexible_start_tag(
        &mut self,
        x: i32,
        _y: i32,
        remainder: &str,
        common: &mut Common,
    ) -> Result<()> {
        let mut lines = remainder.lines();

        let tagname = match lines.next() {
            Some(t) => t,
            None => {
                tt_warning!(
                    common.status,
                    "ignoring TDUX flexible start tag -- no tag name: {:?}",
                    remainder
                );
                return Ok(());
            }
        };

        if !tagname.chars().all(char::is_alphanumeric) {
            tt_warning!(
                common.status,
                "ignoring TDUX flexible start tag -- invalid tag name: {:?}",
                remainder
            );
            return Ok(());
        }

        let el = self.create_elem(tagname, true, common);

        let mut elstate = {
            let cur = self.cur_elstate();

            let font_family_id = self
                .tag_associations
                .get(&el)
                .copied()
                .unwrap_or(cur.font_family_id);

            ElementState {
                elem: Some(el),
                origin: ElementOrigin::Manual,
                font_family_id,
                ..*cur
            }
        };

        let mut classes = Vec::new();
        let mut styles = Vec::new();
        let mut unquoted_attrs = Vec::new();
        let mut double_quoted_attrs = Vec::new();

        for line in lines {
            if let Some(cls) = line.strip_prefix('C') {
                // For later: apply any restrictions to allowed class names?
                if !cls.is_empty() {
                    classes.push(cls.to_owned());
                } else {
                    tt_warning!(
                        common.status,
                        "ignoring TDUX flexible start tag class -- invalid name: {:?}",
                        cls
                    );
                }
            } else if let Some(rest) = line.strip_prefix('S') {
                // For later: apply any restrictions to names/values here?
                let mut bits = rest.splitn(2, ' ');
                let name = match bits.next() {
                    Some(n) => n,
                    None => {
                        tt_warning!(
                            common.status,
                            "ignoring TDUX flexible start tag style -- no name: {:?}",
                            rest
                        );
                        continue;
                    }
                };
                let value = match bits.next() {
                    Some(v) => v,
                    None => {
                        tt_warning!(
                            common.status,
                            "ignoring TDUX flexible start tag style -- no value: {:?}",
                            rest
                        );
                        continue;
                    }
                };
                styles.push((name.to_owned(), value.to_owned()));
            } else if let Some(rest) = line.strip_prefix('U') {
                // For later: apply any restrictions to names/values here?
                let mut bits = rest.splitn(2, ' ');
                let name = match bits.next() {
                    Some("class") | Some("style") => {
                        tt_warning!(
                            common.status,
                            "ignoring TDUX flexible start tag attr -- use C/S command: {:?}",
                            rest
                        );
                        continue;
                    }
                    Some(n) => n,
                    None => {
                        tt_warning!(
                            common.status,
                            "ignoring TDUX flexible start tag attr -- no name: {:?}",
                            rest
                        );
                        continue;
                    }
                };
                unquoted_attrs.push((name.to_owned(), bits.next().map(|v| v.to_owned())));
            } else if let Some(rest) = line.strip_prefix('D') {
                // For later: apply any restrictions to names/values here?
                let mut bits = rest.splitn(2, ' ');
                let name = match bits.next() {
                    Some("class") | Some("style") => {
                        tt_warning!(
                            common.status,
                            "ignoring TDUX flexible start tag attr -- use C/S command: {:?}",
                            rest
                        );
                        continue;
                    }
                    Some(n) => n,
                    None => {
                        tt_warning!(
                            common.status,
                            "ignoring TDUX flexible start tag attr -- no name: {:?}",
                            rest
                        );
                        continue;
                    }
                };
                double_quoted_attrs.push((name.to_owned(), bits.next().map(|v| v.to_owned())));
            } else if line == "NAS" {
                elstate.do_auto_spaces = false;
            } else if line == "NAT" {
                elstate.do_auto_tags = false;
            } else {
                tt_warning!(
                    common.status,
                    "ignoring unrecognized TDUX flexible start tag command: {:?}",
                    line
                );
            }
        }

        self.push_space_if_needed(x, None);
        self.content.push_char('<');
        self.content.push_with_html_escaping(tagname);

        if !classes.is_empty() {
            self.content.push_str(" class=\"");

            let mut first = true;
            for c in &classes {
                if first {
                    first = false;
                } else {
                    self.content.push_char(' ');
                }

                self.content
                    .push_with_html_double_quoted_attribute_escaping(c);
            }

            self.content.push_char('\"');
        }

        if !styles.is_empty() {
            self.content.push_str(" style=\"");

            let mut first = true;
            for (name, value) in &styles {
                if first {
                    first = false;
                } else {
                    self.content.push_char(';');
                }

                self.content
                    .push_with_html_double_quoted_attribute_escaping(name);
                self.content.push_char(':');
                self.content
                    .push_with_html_double_quoted_attribute_escaping(value);
            }

            self.content.push_char('\"');
        }

        for (name, maybe_value) in &unquoted_attrs {
            self.content.push_char(' ');
            self.content.push_with_html_escaping(name);

            if let Some(v) = maybe_value {
                self.content.push_char('=');
                self.content.push_with_html_unquoted_attribute_escaping(v);
            }
        }

        for (name, maybe_value) in &double_quoted_attrs {
            self.content.push_char(' ');
            self.content.push_with_html_escaping(name);
            self.content.push_str("=\"");

            if let Some(v) = maybe_value {
                self.content
                    .push_with_html_double_quoted_attribute_escaping(v);
            }

            self.content.push_char('\"');
        }

        self.content.push_char('>');
        self.elem_stack.push(elstate);
        Ok(())
    }

    pub(crate) fn handle_text_and_glyphs(
        &mut self,
        font_num: TexFontNum,
        text: &str,
        glyphs: &[u16],
        xs: &[i32],
        ys: &[i32],
        common: &mut Common,
    ) -> Result<()> {
        if let Some(c) = self.current_canvas.as_mut() {
            for i in 0..glyphs.len() {
                c.glyphs.push(GlyphInfo {
                    dx: xs[i] - c.x0,
                    dy: ys[i] - c.y0,
                    glyph: glyphs[i],
                    font_num,
                });
            }
        } else if !glyphs.is_empty() {
            self.set_up_for_font(xs[0], font_num, common);
            self.push_space_if_needed(xs[0], Some(font_num));
            self.content.push_with_html_escaping(text);

            // To figure out when we need spaces, we need to care about the last
            // glyph's actual width (well, its advance).
            //
            // TODO: RTL correctness!!!!

            let idx = glyphs.len() - 1;
            let gm = atry!(
                self.fonts.get_glyph_metrics(font_num, glyphs[idx]);
                ["undeclared font {} in canvas", font_num]
            );
            let advance = match gm {
                Some(gm) => gm.advance,
                None => 0,
            };

            let cur_space_width = self.fonts.maybe_get_font_space_width(Some(font_num));
            self.content
                .update_content_pos(xs[idx] + advance, cur_space_width);
        }

        Ok(())
    }

    pub(crate) fn handle_glyph_run(
        &mut self,
        font_num: TexFontNum,
        glyphs: &[u16],
        xs: &[i32],
        ys: &[i32],
        common: &mut Common,
    ) -> Result<()> {
        if let Some(c) = self.current_canvas.as_mut() {
            for i in 0..glyphs.len() {
                c.glyphs.push(GlyphInfo {
                    dx: xs[i] - c.x0,
                    dy: ys[i] - c.y0,
                    glyph: glyphs[i],
                    font_num,
                });
            }
        } else {
            let cur_space_width = self.fonts.maybe_get_font_space_width(Some(font_num));
            let do_auto_spaces = self.cur_elstate().do_auto_spaces;
            let mut ch_str_buf = [0u8; 4];

            // Ideally, the vast majority of the time we are using
            // handle_text_and_glyphs and not this function, outside of
            // canvases. But sometimes we get spare glyphs outside of the canvas
            // context. We can use our glyph-mapping infrastructure to try to
            // translate them to Unicode, hoping for the best that the naive
            // inversion suffices.

            self.set_up_for_font(xs[0], font_num, common);

            let fonts = &mut self.fonts;

            let iter = atry!(
                fonts.process_glyphs_as_text(font_num, glyphs, common.status);
                ["undeclared font {} in glyph run", font_num]
            );

            for (idx, text_info, advance) in iter {
                if let Some((ch, font_sel)) = text_info {
                    let ch_as_str = ch.encode_utf8(&mut ch_str_buf);

                    // XXX this is (part of) push_space_if_needed
                    if self
                        .content
                        .is_space_needed(xs[idx], cur_space_width, do_auto_spaces)
                    {
                        self.content.push_char(' ');
                    }

                    write!(self.content, "<span style=\"{font_sel}\">").unwrap();
                    self.content.push_with_html_escaping(ch_as_str);
                    write!(self.content, "</span>").unwrap();
                }

                self.content
                    .update_content_pos(xs[idx] + advance, cur_space_width);
            }
        }

        Ok(())
    }

    fn set_up_for_font(&mut self, x0: i32, fnum: TexFontNum, common: &mut Common) {
        let (cur_ffid, cur_af, cur_is_autofont) = {
            let cur = self.cur_elstate();
            (
                cur.font_family_id,
                cur.active_font,
                cur.origin == ElementOrigin::FontAuto,
            )
        };

        let (path, desired_af) = match self.fonts.analyze_font_for_family(fnum, cur_ffid, cur_af) {
            FontFamilyAnalysis::AlreadyActive => return,
            FontFamilyAnalysis::Reachable(p, d) => (p, d),
            FontFamilyAnalysis::NoMatch(fid) => {
                // We don't seem to be in a defined "family". So we have to
                // select it explicitly.
                let path = PathToNewFont {
                    close_all: true,
                    select_explicitly: true,
                    ..Default::default()
                };

                let desired_af = FamilyRelativeFontId::Other(fid);
                (path, desired_af)
            }

            FontFamilyAnalysis::Unrecognized => {
                tt_warning!(common.status, "undeclared font number {}", fnum);
                return;
            }
        };

        if path.close_one_and_retry {
            if cur_is_autofont {
                self.close_one();
                return self.set_up_for_font(x0, fnum, common);
            } else {
                // This is a logic error in our implementation -- this
                // should never happen.
                tt_warning!(
                    common.status,
                    "font selection failed (ffid={}, active={:?}, desired={})",
                    cur_ffid,
                    cur_af,
                    fnum
                );
                return;
            }
        }

        if path.close_all {
            self.close_automatics();
        }

        if let Some(af) = path.open_b {
            self.push_space_if_needed(x0, Some(fnum));
            self.content.push_str("<b>");
            self.elem_stack.push(ElementState {
                elem: Some(Element::B),
                origin: ElementOrigin::FontAuto,
                active_font: af,
                ..*self.cur_elstate()
            });
        }

        if let Some(af) = path.open_i {
            self.push_space_if_needed(x0, Some(fnum));
            self.content.push_str("<i>");
            self.elem_stack.push(ElementState {
                elem: Some(Element::I),
                origin: ElementOrigin::FontAuto,
                active_font: af,
                ..*self.cur_elstate()
            });
        }

        if path.select_explicitly {
            self.push_space_if_needed(x0, Some(fnum));
            self.fonts
                .write_styling_span_html(fnum, self.rems_per_tex, &mut self.content)
                .unwrap();
            self.elem_stack.push(ElementState {
                elem: Some(Element::Span),
                origin: ElementOrigin::FontAuto,
                active_font: desired_af,
                ..*self.cur_elstate()
            });
        }
    }

    fn handle_end_canvas(&mut self, common: &mut Common) -> Result<()> {
        let mut canvas = self.current_canvas.take().unwrap();

        // This is the *end* of a canvas, but we haven't pushed anything into
        // the content since whatever started the canvas, so we need this:
        self.push_space_if_needed(canvas.x0, None);

        let inline = match canvas.kind.as_ref() {
            "math" => true,
            "dmath" => false,
            _ => false,
        };

        // First pass: get overall bounds of all the glyphs from their metrics.
        // We need to gather this information first because as we emit glyphs we
        // have to specify their positions relative to the edges of the
        // containing canvas box, and the size of that box is defined by the
        // extents of all of the glyphs it contains. The bounds are measured in
        // TeX units.

        let mut first = true;
        let mut x_min_tex = 0;
        let mut x_max_tex = 0;
        let mut y_min_tex = 0;
        let mut y_max_tex = 0;

        for gi in &canvas.glyphs[..] {
            let gm = atry!(
                self.fonts.get_glyph_metrics(gi.font_num, gi.glyph);
                ["undeclared font {} in canvas", gi.font_num]
            );

            if let Some(gm) = gm {
                // to check: RTL correctness
                let xmin = gi.dx - gm.lsb;
                let xmax = gi.dx + gm.advance;
                let ymin = gi.dy - gm.ascent;
                let ymax = gi.dy - gm.descent; // note: descent is negative

                if first {
                    x_min_tex = xmin;
                    x_max_tex = xmax;
                    y_min_tex = ymin;
                    y_max_tex = ymax;
                    first = false;
                } else {
                    x_min_tex = std::cmp::min(x_min_tex, xmin);
                    x_max_tex = std::cmp::max(x_max_tex, xmax);
                    y_min_tex = std::cmp::min(y_min_tex, ymin);
                    y_max_tex = std::cmp::max(y_max_tex, ymax);
                }
            }
        }

        // Now that we have that information, we can lay out the individual
        // glyphs.
        //
        // A resource I found very helpful:
        // https://iamvdo.me/en/blog/css-font-metrics-line-height-and-vertical-align

        let mut inner_content = String::default();
        let mut ch_str_buf = [0u8; 4];

        for gi in canvas.glyphs.drain(..) {
            let (text_info, size, baseline_factor) =
                self.fonts
                    .process_glyph_for_canvas(gi.font_num, gi.glyph, common.status);

            // The size of the font being used for this glyph, in rems; that is,
            // relative to the main body font.
            let rel_size = size as f32 * self.rems_per_tex;

            if let Some((ch, font_sel)) = text_info {
                // dy gives the target position of this glyph's baseline
                // relative to the canvas's baseline. For our `position:
                // absolute` layout, we have to convert that into the distance
                // between the top of this glyph's box and the top of the
                // overall canvas box (or bottom/bottom).
                //
                // In order to do this, we need to know the size of this glyph's
                // box according to CSS, and the position of the glyph's
                // baseline within that box.
                //
                // The baseline position is straightforward: it is given by what
                // we call the font's "baseline factor". This is true no matter
                // the specific size of the CSS box relative to the font
                // rendering size, due to the way in which the drawn glyph is
                // centered vertically within its CSS box.
                //
                // The CSS glyph box height can be funky: it depends on the
                // font-size setting, font metrics (not just ascender/descender
                // but "line gap") and `line-height` setting in "exciting" ways.
                // One convenient approach is to set `line-height: 1` in the
                // container, in which case the box height is the `font-size`
                // setting.

                let top_rem =
                    (-y_min_tex + gi.dy) as f32 * self.rems_per_tex - baseline_factor * rel_size;

                // Stringify the character so that we can use html_escape in
                // case it's a `<` or whatever.
                let ch_as_str = ch.encode_utf8(&mut ch_str_buf);

                write!(
                    inner_content,
                    "<span class=\"ci\" style=\"top: {}rem; left: {}rem; font-size: {}rem; {}\">",
                    top_rem,
                    gi.dx as f32 * self.rems_per_tex,
                    rel_size,
                    font_sel,
                )
                .unwrap();
                html_escape::encode_text_to_string(ch_as_str, &mut inner_content);
                write!(inner_content, "</span>").unwrap();
            }
        }

        let (element, layout_class, valign) = if inline {
            // A numerical vertical-align setting positions the bottom edge of
            // this block relative to the containing line's baseline. This is
            // the best (only?) way to make sure that this block's baseline
            // lines up with that of its container.
            (
                "span",
                "canvas-inline",
                format!(
                    "; vertical-align: {}rem",
                    -y_max_tex as f32 * self.rems_per_tex
                ),
            )
        } else {
            ("div", "canvas-block", "".to_owned())
        };

        let element = self.create_elem(element, true, common);

        write!(
            self.content,
            "<{} class=\"canvas {}\" style=\"width: {}rem; height: {}rem; padding-left: {}rem{}\">",
            element.name(),
            layout_class,
            (x_max_tex - x_min_tex) as f32 * self.rems_per_tex,
            (y_max_tex - y_min_tex) as f32 * self.rems_per_tex,
            -x_min_tex as f32 * self.rems_per_tex,
            valign,
        )
        .unwrap();
        self.content.push_str(&inner_content);
        write!(self.content, "</{}>", element.name()).unwrap();
        let cur_space_width = self.fonts.maybe_get_font_space_width(None);
        self.content
            .update_content_pos(x_max_tex + canvas.x0, cur_space_width);
        Ok(())
    }

    fn finish_file(&mut self, common: &mut Common) -> Result<()> {
        self.templating
            .set_variable("tduxContent", self.content.take());
        self.templating.emit(common)?;

        let cur_space_width = self.fonts.maybe_get_font_space_width(None);
        self.content.update_content_pos(0, cur_space_width);
        Ok(())
    }

    pub(crate) fn emission_finished(mut self, common: &mut Common) -> Result<FinalizingState> {
        if !self.content.is_empty() {
            tt_warning!(
                common.status,
                "non-empty content left at the end without an explicit `emit` in HTML output"
            );

            if self.templating.ready_to_output() {
                self.finish_file(common)?;
            }
        }

        FinalizingState::new(self.fonts, self.templating, self.assets)
    }
}
