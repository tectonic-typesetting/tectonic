// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! Convert Tectonic’s SPX format to HTML.

use percent_encoding::{utf8_percent_encode, CONTROLS};
use std::{
    collections::HashMap,
    fmt::Write as FmtWrite,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use tectonic_bridge_core::DriverHooks;
use tectonic_errors::prelude::*;
use tectonic_io_base::OpenResult;
use tectonic_status_base::{tt_warning, StatusBackend};
use tectonic_xdv::{FileType, XdvEvents, XdvParser};

use crate::font::{FontData, MapEntry};

mod font;

/// An engine that converts SPX to HTML.
#[derive(Default)]
pub struct Spx2HtmlEngine {}

impl Spx2HtmlEngine {
    /// Process SPX into HTML.
    ///
    /// Because this driver will, in the generic case, produce a tree of HTML
    /// output files that are not going to be used as a basis for any subsequent
    /// engine stages, it outputs directly to disk (via `out_base`) rather than
    /// using the I/O layer. I don't like hardcoding use of the filesystem, but
    /// I don't want to build up some extra abstraction layer right now.
    pub fn process_to_filesystem(
        &mut self,
        hooks: &mut dyn DriverHooks,
        status: &mut dyn StatusBackend,
        spx: &str,
        out_base: &Path,
    ) -> Result<()> {
        let mut input = hooks.io().input_open_name(spx, status).must_exist()?;

        {
            let state = EngineState::new(hooks, status, out_base);
            let state = XdvParser::process_with_seeks(&mut input, state)?;
            state.finished()?;
        }

        let (name, digest_opt) = input.into_name_digest();
        hooks.event_input_closed(name, digest_opt, status);
        Ok(())
    }
}

struct EngineState<'a> {
    common: Common<'a>,
    state: State,
}

struct Common<'a> {
    hooks: &'a mut dyn DriverHooks,
    status: &'a mut dyn StatusBackend,
    out_base: &'a Path,
}

impl<'a> EngineState<'a> {
    pub fn new(
        hooks: &'a mut dyn DriverHooks,
        status: &'a mut dyn StatusBackend,
        out_base: &'a Path,
    ) -> Self {
        Self {
            common: Common {
                hooks,
                status,
                out_base,
            },
            state: State::Initializing(InitializationState::default()),
        }
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum State {
    /// This variant is needed to implement state changes.
    Invalid,
    Initializing(InitializationState),
    Emitting(EmittingState),
}

impl<'a> EngineState<'a> {
    pub fn finished(mut self) -> Result<()> {
        if let State::Emitting(mut s) = self.state {
            if !s.current_content.is_empty() {
                s.finish_file(&mut self.common)?;
            }
        }

        Ok(())
    }
}

impl<'a> XdvEvents for EngineState<'a> {
    type Error = Error;

    fn handle_header(&mut self, filetype: FileType, _comment: &[u8]) -> Result<()> {
        if filetype != FileType::Spx {
            bail!("file should be SPX format but got {}", filetype);
        }

        Ok(())
    }

    fn handle_special(&mut self, x: i32, y: i32, contents: &[u8]) -> Result<()> {
        let contents = atry!(std::str::from_utf8(contents); ["could not parse \\special as UTF-8"]);

        if contents == "tdux:emit" || contents.starts_with("tdux:provideFile") {
            self.state.ensure_initialized()?;
        }

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(s) => s.handle_special(contents, &mut self.common),
            State::Emitting(s) => s.handle_special(x, y, contents, &mut self.common),
        }
    }

    fn handle_text_and_glyphs(
        &mut self,
        font_num: i32,
        text: &str,
        _width: i32,
        glyphs: &[u16],
        x: &[i32],
        y: &[i32],
    ) -> Result<()> {
        self.state.ensure_initialized()?;

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(_) => unreachable!(),
            State::Emitting(s) => {
                s.handle_text_and_glyphs(font_num, text, glyphs, x, y, &mut self.common)?
            }
        }

        Ok(())
    }

    fn handle_define_native_font(
        &mut self,
        name: &str,
        font_num: i32,
        size: i32,
        face_index: u32,
        color_rgba: Option<u32>,
        extend: Option<u32>,
        slant: Option<u32>,
        embolden: Option<u32>,
    ) -> Result<(), Self::Error> {
        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(s) => s.handle_define_native_font(
                name,
                font_num,
                size,
                face_index,
                color_rgba,
                extend,
                slant,
                embolden,
                &mut self.common,
            ),
            _ => Ok(()),
        }
    }

    fn handle_glyph_run(
        &mut self,
        font_num: i32,
        glyphs: &[u16],
        x: &[i32],
        y: &[i32],
    ) -> Result<(), Self::Error> {
        self.state.ensure_initialized()?;

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(_) => unreachable!(),
            State::Emitting(s) => s.handle_glyph_run(font_num, glyphs, x, y, &mut self.common),
        }
    }
}

impl State {
    fn ensure_initialized(&mut self) -> Result<()> {
        // Is this the least-bad way to do this??
        let mut work = std::mem::replace(self, State::Invalid);

        if let State::Initializing(s) = work {
            work = State::Emitting(s.initialization_finished()?);
        }

        std::mem::swap(self, &mut work);
        Ok(())
    }
}

#[derive(Debug)]
struct InitializationState {
    templates: HashMap<String, String>,
    next_template_path: String,
    next_output_path: String,
    fonts: HashMap<i32, FontInfo>,
    main_body_font_size: FixedPoint,
    font_data_keys: HashMap<(String, u32), usize>,
    font_data: HashMap<usize, FontData>,
    variables: HashMap<String, String>,
}

impl Default for InitializationState {
    fn default() -> Self {
        InitializationState {
            templates: Default::default(),
            next_template_path: Default::default(),
            next_output_path: "index.html".to_owned(),
            fonts: Default::default(),
            main_body_font_size: 0,
            font_data_keys: Default::default(),
            font_data: Default::default(),
            variables: Default::default(),
        }
    }
}

impl InitializationState {
    #[allow(clippy::too_many_arguments)]
    fn handle_define_native_font(
        &mut self,
        name: &str,
        font_num: i32,
        size: FixedPoint,
        face_index: u32,
        color_rgba: Option<u32>,
        extend: Option<u32>,
        slant: Option<u32>,
        embolden: Option<u32>,
        common: &mut Common,
    ) -> Result<()> {
        if self.fonts.contains_key(&font_num) {
            // Should we override the definition or something?
            return Ok(());
        }

        // TODO: often there are multiple font_nums with the same "name". We
        // only need to copy the file once.

        let io = common.hooks.io();
        let mut texpath = String::default();
        let mut ih = None;

        for ext in &["", ".otf"] {
            texpath = format!("{}{}", name, ext);

            match io.input_open_name(&texpath, common.status) {
                OpenResult::Ok(h) => {
                    ih = Some(h);
                    break;
                }

                OpenResult::NotAvailable => continue,

                OpenResult::Err(e) => return Err(e),
            };
        }

        let mut ih = a_ok_or!(ih;
            ["failed to find a font file associated with the name `{}`", name]
        );

        let mut contents = Vec::new();
        atry!(
            ih.read_to_end(&mut contents);
            ["unable to read input font file `{}`", &texpath]
        );
        let (name, digest_opt) = ih.into_name_digest();
        common
            .hooks
            .event_input_closed(name.clone(), digest_opt, common.status);

        let mut out_path = common.out_base.to_owned();
        let basename = texpath.rsplit('/').next().unwrap();
        out_path.push(basename);

        {
            let mut out_file = atry!(
                File::create(&out_path);
                ["cannot open output file `{}`", out_path.display()]
            );

            atry!(
                out_file.write_all(&contents);
                ["cannot write output file `{}`", out_path.display()]
            );
        }

        let fd_key = (name, face_index);
        let next_id = self.font_data_keys.len();
        let fd_key = *self.font_data_keys.entry(fd_key).or_insert(next_id);

        if fd_key == next_id {
            let map = atry!(
                FontData::from_opentype(basename.to_owned(), contents, face_index);
                ["unable to load glyph data from font `{}`", texpath]
            );
            self.font_data.insert(fd_key, map);
        }

        // TODO: actually handle font roles. Here we intentionally overwrite
        // main_body_font_size with every new font because when we're scanning
        // the postamble, the last font is the main body font. In my one
        // example.
        self.main_body_font_size = size;

        let info = FontInfo {
            role: FontRole::MainBody,
            rel_url: utf8_percent_encode(basename, CONTROLS).to_string(),
            fd_key,
            size,
            face_index,
            color_rgba,
            extend,
            slant,
            embolden,
        };

        self.fonts.insert(font_num, info);
        Ok(())
    }

    fn handle_special(&mut self, contents: &str, common: &mut Common) -> Result<()> {
        if let Some(texpath) = contents.strip_prefix("tdux:addTemplate ") {
            self.handle_add_template(texpath, common)
        } else if let Some(texpath) = contents.strip_prefix("tdux:setTemplate ") {
            self.handle_set_template(texpath, common)
        } else if let Some(texpath) = contents.strip_prefix("tdux:setOutputPath ") {
            self.handle_set_output_path(texpath, common)
        } else if let Some(remainder) = contents.strip_prefix("tdux:setTemplateVariable ") {
            self.handle_set_template_variable(remainder, common)
        } else if let Some(_remainder) = contents.strip_prefix("tdux:provideFile ") {
            tt_warning!(common.status, "ignoring too-soon tdux:provideFile special");
            Ok(())
        } else {
            Ok(())
        }
    }

    fn handle_add_template(&mut self, texpath: &str, common: &mut Common) -> Result<()> {
        let mut ih = atry!(
            common.hooks.io().input_open_name(texpath, common.status).must_exist();
            ["unable to open input HTML template `{}`", texpath]
        );

        let mut contents = String::new();
        atry!(
            ih.read_to_string(&mut contents);
            ["unable to read input HTML template `{}`", texpath]
        );

        self.templates.insert(texpath.to_owned(), contents);

        let (name, digest_opt) = ih.into_name_digest();
        common
            .hooks
            .event_input_closed(name, digest_opt, common.status);
        Ok(())
    }

    fn handle_set_template(&mut self, texpath: &str, _common: &mut Common) -> Result<()> {
        self.next_template_path = texpath.to_owned();
        Ok(())
    }

    fn handle_set_output_path(&mut self, texpath: &str, _common: &mut Common) -> Result<()> {
        self.next_output_path = texpath.to_owned();
        Ok(())
    }

    fn handle_set_template_variable(&mut self, remainder: &str, common: &mut Common) -> Result<()> {
        if let Some((varname, varval)) = remainder.split_once(' ') {
            self.variables.insert(varname.to_owned(), varval.to_owned());
        } else {
            tt_warning!(
                common.status,
                "ignoring malformatted tdux:setTemplateVariable special `{}`",
                remainder
            );
        }

        Ok(())
    }

    fn initialization_finished(self) -> Result<EmittingState> {
        // Tera requires that we give it a filesystem path to look for
        // templates, even if we're going to be adding all of our templates
        // later. So I guess we have to create an empty tempdir.

        let tempdir = atry!(
            tempfile::Builder::new().prefix("tectonic_tera_workaround").tempdir();
            ["couldn't create empty temporary directory for Tera"]
        );

        let mut p = PathBuf::from(tempdir.path());
        p.push("*");

        let p = a_ok_or!(
            p.to_str();
            ["couldn't convert Tera temporary directory name to UTF8 as required"]
        );

        let mut tera = atry!(
            tera::Tera::parse(p);
            ["couldn't initialize Tera templating engine in temporary directory `{}`", p]
        );

        atry!(
            tera.add_raw_templates(self.templates.iter());
            ["couldn't compile Tera templates"]
        );

        // Set up the context.

        let mut context = tera::Context::default();

        for (varname, varvalue) in self.variables {
            context.insert(varname, &varvalue);
        }

        // All done!

        Ok(EmittingState {
            tera,
            context,
            fonts: self.fonts,
            rems_per_tex: 1.0 / (self.main_body_font_size as f32),
            font_data: self.font_data,
            next_template_path: self.next_template_path,
            next_output_path: self.next_output_path,
            current_content: String::default(),
            elem_stack: vec![ElementState {
                elem: None,
                do_auto_tags: true,
                do_auto_spaces: true,
            }],
            current_canvas: None,
            content_finished: false,
            content_finished_warning_issued: false,
            last_content_x: 0,
            last_content_space_width: None,
        })
    }
}

#[derive(Debug)]
struct EmittingState {
    tera: tera::Tera,
    context: tera::Context,
    fonts: HashMap<i32, FontInfo>,
    rems_per_tex: f32,
    font_data: HashMap<usize, FontData>,
    next_template_path: String,
    next_output_path: String,
    current_content: String,
    elem_stack: Vec<ElementState>,
    current_canvas: Option<CanvasState>,
    content_finished: bool,
    content_finished_warning_issued: bool,
    last_content_x: i32,
    last_content_space_width: Option<FixedPoint>,
}

#[derive(Debug)]
struct ElementState {
    elem: Option<String>,
    do_auto_tags: bool,
    do_auto_spaces: bool,
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
    font_num: i32,
    glyph: u16,
}

impl EmittingState {
    fn warn_finished_content(&mut self, detail: &str, common: &mut Common) {
        if !self.content_finished_warning_issued {
            tt_warning!(common.status, "dropping post-finish content ({})", detail);
            self.content_finished_warning_issued = true;
        }
    }

    #[inline(always)]
    fn cur_elstate(&self) -> &ElementState {
        self.elem_stack.last().unwrap()
    }

    fn push_elem<D: std::fmt::Display>(&mut self, name: D) {
        let new_item = ElementState {
            elem: Some(name.to_string()),
            ..*self.cur_elstate()
        };

        self.elem_stack.push(new_item);
    }

    /// TODO: may need to hone semantics when element nesting isn't as expected.
    fn pop_elem(&mut self, name: &str, common: &mut Common) {
        let mut n_closed = 0;

        while self.elem_stack.len() > 1 {
            let cur = self.elem_stack.pop().unwrap();

            if let Some(e) = cur.elem.as_ref() {
                self.current_content.push('<');
                self.current_content.push('/');
                self.current_content.push_str(e);
                self.current_content.push('>');
                n_closed += 1;

                if e == name {
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

    fn maybe_get_font_space_width(&self, font_num: Option<i32>) -> Option<FixedPoint> {
        font_num.and_then(|fnum| {
            if let Some(fi) = self.fonts.get(&fnum) {
                let fd = self.font_data.get(&fi.fd_key).unwrap();
                fd.space_width(fi.size)
            } else {
                None
            }
        })
    }

    /// Figure out if we need to push a space into the text content right now.
    fn is_space_needed(&mut self, x0: i32, cur_font_num: Option<i32>) -> bool {
        // We never want a leading space.
        if self.current_content.is_empty() {
            return false;
        }

        // Auto-spaces can be disabled.
        if !self.cur_elstate().do_auto_spaces {
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

        let cur_space_width = self.maybe_get_font_space_width(cur_font_num);

        let space_width = match (&self.last_content_space_width, &cur_space_width) {
            (Some(w1), Some(w2)) => FixedPoint::min(*w1, *w2),
            (Some(w), None) => *w,
            (None, Some(w)) => *w,
            (None, None) => 0,
        };

        // If the x difference is larger than 1/4 of the space_width, let's say that
        // we need a space. I made up the 1/4.
        return 4 * (x0 - self.last_content_x) > space_width;
    }

    fn update_content_pos(&mut self, x: i32, font_num: Option<i32>) {
        self.last_content_x = x;

        let cur_space_width = self.maybe_get_font_space_width(font_num);
        if cur_space_width.is_some() {
            self.last_content_space_width = cur_space_width;
        }
    }

    /// Maybe push a space into the text content right now, if we think we need one.
    fn push_space_if_needed(&mut self, x0: i32, cur_font_num: Option<i32>) {
        if self.is_space_needed(x0, cur_font_num) {
            self.current_content.push(' ');
        }

        // This parameter should be updated almost-instantaneously
        // if a run of glyphs is being rendered, but this is a good start:
        self.update_content_pos(x0, cur_font_num);
    }

    fn handle_special(
        &mut self,
        x: i32,
        y: i32,
        contents: &str,
        common: &mut Common,
    ) -> Result<()> {
        if let Some(element) = contents.strip_prefix("tdux:as ") {
            if self.content_finished {
                self.warn_finished_content(&format!("auto start tag <{}>", element), common);
            } else if self.cur_elstate().do_auto_tags {
                self.push_space_if_needed(x, None);
                self.current_content.push('<');
                self.current_content.push_str(element);
                self.current_content.push('>');
                self.push_elem(element);
            }
            Ok(())
        } else if let Some(element) = contents.strip_prefix("tdux:ae ") {
            if self.content_finished {
                self.warn_finished_content(&format!("auto end tag </{}>", element), common);
            } else if self.cur_elstate().do_auto_tags {
                self.pop_elem(element, common);
            }
            Ok(())
        } else if let Some(kind) = contents.strip_prefix("tdux:cs ") {
            if self.content_finished {
                self.warn_finished_content("canvas start", common);
            } else if let Some(canvas) = self.current_canvas.as_mut() {
                canvas.depth += 1;
            } else {
                self.current_canvas = Some(CanvasState::new(kind, x, y));
            }
            Ok(())
        } else if let Some(_kind) = contents.strip_prefix("tdux:ce ") {
            if self.content_finished {
                self.warn_finished_content("canvas end", common);
            } else if let Some(canvas) = self.current_canvas.as_mut() {
                canvas.depth -= 1;
                if canvas.depth == 0 {
                    self.handle_end_canvas(common)?;
                }
            } else {
                tt_warning!(
                    common.status,
                    "ignoring unpaired tdux:c[anvas]e[nd] special `{}`",
                    contents
                );
            }
            Ok(())
        } else if let Some(remainder) = contents.strip_prefix("tdux:mfs ") {
            if self.content_finished {
                self.warn_finished_content(
                    &format!("manual flexible start tag {:?}", remainder),
                    common,
                );
                Ok(())
            } else {
                self.handle_flexible_start_tag(x, y, remainder, common)
            }
        } else if let Some(element) = contents.strip_prefix("tdux:me ") {
            if self.content_finished {
                self.warn_finished_content(&format!("manual end tag </{}>", element), common);
            } else {
                self.pop_elem(element, common);
            }
            Ok(())
        } else if let Some(direct_text) = contents.strip_prefix("tdux:dt ") {
            if self.content_finished {
                self.warn_finished_content("direct text", common);
            } else {
                html_escape::encode_safe_to_string(direct_text, &mut self.current_content);
            }
            Ok(())
        } else if contents == "tdux:emit" {
            self.finish_file(common)
        } else if let Some(texpath) = contents.strip_prefix("tdux:setTemplate ") {
            self.next_template_path = texpath.to_owned();
            Ok(())
        } else if let Some(texpath) = contents.strip_prefix("tdux:setOutputPath ") {
            self.next_output_path = texpath.to_owned();
            Ok(())
        } else if let Some(remainder) = contents.strip_prefix("tdux:setTemplateVariable ") {
            self.handle_set_template_variable(remainder, common)
        } else if let Some(remainder) = contents.strip_prefix("tdux:provideFile ") {
            self.handle_provide_file(remainder, common)
        } else if contents == "tdux:contentFinished" {
            self.content_finished(common)
        } else if let Some(remainder) = contents.strip_prefix("tdux:") {
            tt_warning!(
                common.status,
                "ignoring unrecognized special: tdux:{}",
                remainder
            );
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Handle a "flexible" start tag.
    ///
    /// These start tags are built with a line-oriented structure that aims to
    /// make it so that the TeX code doesn't have to worry too much about
    /// escaping, etc. The general format is:
    ///
    /// ```
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

        let mut elstate = ElementState {
            elem: Some(tagname.to_owned()),
            ..*self.cur_elstate()
        };
        let mut classes = Vec::new();
        let mut styles = Vec::new();
        let mut unquoted_attrs = Vec::new();
        let mut double_quoted_attrs = Vec::new();

        for line in lines {
            if let Some(cls) = line.strip_prefix("C") {
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
            } else if let Some(rest) = line.strip_prefix("S") {
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
            } else if let Some(rest) = line.strip_prefix("U") {
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
            } else if let Some(rest) = line.strip_prefix("D") {
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
        self.current_content.push('<');
        html_escape::encode_safe_to_string(tagname, &mut self.current_content);

        if !classes.is_empty() {
            self.current_content.push_str(" class=\"");

            let mut first = true;
            for c in &classes {
                if first {
                    first = false;
                } else {
                    self.current_content.push(' ');
                }

                html_escape::encode_double_quoted_attribute_to_string(c, &mut self.current_content);
            }

            self.current_content.push('\"');
        }

        if !styles.is_empty() {
            self.current_content.push_str(" style=\"");

            let mut first = true;
            for (name, value) in &styles {
                if first {
                    first = false;
                } else {
                    self.current_content.push(';');
                }

                html_escape::encode_double_quoted_attribute_to_string(
                    name,
                    &mut self.current_content,
                );
                self.current_content.push(':');
                html_escape::encode_double_quoted_attribute_to_string(
                    value,
                    &mut self.current_content,
                );
            }

            self.current_content.push('\"');
        }

        for (name, maybe_value) in &unquoted_attrs {
            self.current_content.push(' ');
            html_escape::encode_safe_to_string(name, &mut self.current_content);

            if let Some(v) = maybe_value {
                self.current_content.push('=');
                html_escape::encode_unquoted_attribute_to_string(v, &mut self.current_content);
            }
        }

        for (name, maybe_value) in &double_quoted_attrs {
            self.current_content.push(' ');
            html_escape::encode_safe_to_string(name, &mut self.current_content);
            self.current_content.push_str("=\"");

            if let Some(v) = maybe_value {
                html_escape::encode_double_quoted_attribute_to_string(v, &mut self.current_content);
            }

            self.current_content.push('\"');
        }

        self.current_content.push('>');
        self.elem_stack.push(elstate);
        Ok(())
    }

    fn handle_set_template_variable(&mut self, remainder: &str, common: &mut Common) -> Result<()> {
        if let Some((varname, varval)) = remainder.split_once(' ') {
            self.context.insert(varname, varval);
        } else {
            tt_warning!(
                common.status,
                "ignoring malformatted tdux:setTemplateVariable special `{}`",
                remainder
            );
        }

        Ok(())
    }

    fn handle_provide_file(&mut self, remainder: &str, common: &mut Common) -> Result<()> {
        let (src_tex_path, dest_path) = match remainder.split_once(' ') {
            Some(t) => t,
            None => {
                tt_warning!(
                    common.status,
                    "ignoring malformatted tdux:provideFile special `{}`",
                    remainder
                );
                return Ok(());
            }
        };

        // Set up input?

        let mut ih = atry!(
            common.hooks.io().input_open_name(src_tex_path, common.status).must_exist();
            ["unable to open provideFile source `{}`", &src_tex_path]
        );

        // Set up output? TODO: create parent directories!

        let mut out_path = common.out_base.to_owned();

        for piece in dest_path.split('/') {
            if piece.is_empty() {
                continue;
            }

            if piece == ".." {
                bail!(
                    "illegal provideFile dest path `{}`: it contains a `..` component",
                    &dest_path
                );
            }

            let as_path = Path::new(piece);

            if as_path.is_absolute() || as_path.has_root() {
                bail!(
                    "illegal provideFile path `{}`: it contains an absolute/rooted component",
                    &dest_path,
                );
            }

            out_path.push(piece);
        }

        // Copy!

        {
            let mut out_file = atry!(
                File::create(&out_path);
                ["cannot open output file `{}`", out_path.display()]
            );

            atry!(
                std::io::copy(&mut ih, &mut out_file);
                ["cannot copy to output file `{}`", out_path.display()]
            );
        }

        // All done.

        let (name, digest_opt) = ih.into_name_digest();
        common
            .hooks
            .event_input_closed(name, digest_opt, common.status);

        Ok(())
    }

    fn handle_text_and_glyphs(
        &mut self,
        font_num: i32,
        text: &str,
        glyphs: &[u16],
        xs: &[i32],
        ys: &[i32],
        common: &mut Common,
    ) -> Result<()> {
        if self.content_finished {
            self.warn_finished_content(&format!("text `{}`", text), common);
            return Ok(());
        }

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
            self.push_space_if_needed(xs[0], Some(font_num));
            html_escape::encode_text_to_string(text, &mut self.current_content);

            // To figure out when we need spaces, we need to care about the last
            // glyph's actual width (well, its advance).
            //
            // TODO: RTL correctness!!!!

            let idx = glyphs.len() - 1;
            let fi = a_ok_or!(
                self.fonts.get(&font_num);
                ["undeclared font {} in canvas", font_num]
            );
            let fd = self.font_data.get_mut(&fi.fd_key).unwrap();
            let gm = fd.lookup_metrics(glyphs[idx], fi.size);
            let advance = match gm {
                Some(gm) => gm.advance,
                None => 0,
            };
            self.update_content_pos(xs[idx] + advance, Some(font_num));
        }

        Ok(())
    }

    fn handle_glyph_run(
        &mut self,
        font_num: i32,
        glyphs: &[u16],
        xs: &[i32],
        ys: &[i32],
        common: &mut Common,
    ) -> Result<()> {
        if self.content_finished {
            self.warn_finished_content("glyph run", common);
            return Ok(());
        }

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
            let fi = a_ok_or!(
                self.fonts.get(&font_num);
                ["undeclared font {} in canvas", font_num]
            );

            tt_warning!(
                common.status,
                "TODO HANDLE glyph_run OUTSIDE OF CANVAS: {} {:?}",
                fi.rel_url,
                glyphs
            );
        }

        Ok(())
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
            let fi = a_ok_or!(
                self.fonts.get(&gi.font_num);
                ["undeclared font {} in canvas", gi.font_num]
            );

            let fd = self.font_data.get_mut(&fi.fd_key).unwrap();
            let gm = fd.lookup_metrics(gi.glyph, fi.size);

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
            let fi = self.fonts.get(&gi.font_num).unwrap();

            // The size of the font being used for this glyph, in rems; that is,
            // relative to the main body font.
            let rel_size = fi.size as f32 * self.rems_per_tex;
            let fd = self.font_data.get_mut(&fi.fd_key).unwrap();
            let mc = fd.lookup_mapping(gi.glyph);

            if let Some(mc) = mc {
                // Sometimes we need to render a glyph in one of our input fonts
                // that isn't directly associated with a specific Unicode
                // character. For instance, in math, we may need to draw a big
                // integral sign, but the Unicode integral character maps to a
                // small one. The way we handle this is by *creating new fonts*
                // with custom character map tables that *do* map Unicode
                // characters directly to the specific glyphs we want.

                let (mut ch, need_alt) = match mc {
                    MapEntry::Direct(c) => (c, false),
                    MapEntry::SubSuperScript(c, _) => (c, true),
                    MapEntry::MathGrowingVariant(c, _, _) => (c, true),
                };

                let font_fam = if need_alt {
                    let map = fd.request_alternative(gi.glyph, ch);
                    ch = map.usv;
                    format!("tdux{}vg{}", fi.fd_key, map.alternate_map_index)
                } else {
                    format!("tdux{}", fi.fd_key)
                };

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

                let top_rem = (-y_min_tex + gi.dy) as f32 * self.rems_per_tex
                    - fd.baseline_factor() * rel_size;

                // Stringify the character so that we can use html_escape in
                // case it's a `<` or whatever.
                let ch_as_str = ch.encode_utf8(&mut ch_str_buf);

                write!(
                    inner_content,
                    "<span class=\"ci\" style=\"top: {}rem; left: {}rem; font-size: {}rem; font-family: {}\">",
                    top_rem,
                    gi.dx as f32 * self.rems_per_tex,
                    rel_size,
                    font_fam,
                )
                .unwrap();
                html_escape::encode_text_to_string(ch_as_str, &mut inner_content);
                write!(inner_content, "</span>").unwrap();
            } else {
                tt_warning!(
                    common.status,
                    "unable to reverse-map glyph {} in font `{}` (face {})",
                    gi.glyph,
                    fi.rel_url,
                    fi.face_index
                );
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

        write!(
            self.current_content,
            "<{} class=\"canvas {}\" style=\"width: {}rem; height: {}rem; padding-left: {}rem{}\">",
            element,
            layout_class,
            (x_max_tex - x_min_tex) as f32 * self.rems_per_tex,
            (y_max_tex - y_min_tex) as f32 * self.rems_per_tex,
            -x_min_tex as f32 * self.rems_per_tex,
            valign,
        )
        .unwrap();
        self.current_content.push_str(&inner_content);
        write!(self.current_content, "</{}>", element).unwrap();
        self.update_content_pos(x_max_tex + canvas.x0, None);
        Ok(())
    }

    fn finish_file(&mut self, common: &mut Common) -> Result<()> {
        // Prep the output path

        let mut out_path = common.out_base.to_owned();
        let mut n_levels = 0;

        for piece in self.next_output_path.split('/') {
            if piece.is_empty() {
                continue;
            }

            if piece == ".." {
                bail!(
                    "illegal HTML output path `{}`: it contains a `..` component",
                    &self.next_output_path
                );
            }

            let as_path = Path::new(piece);

            if as_path.is_absolute() || as_path.has_root() {
                bail!(
                    "illegal HTML output path `{}`: it contains an absolute/rooted component",
                    &self.next_output_path
                );
            }

            out_path.push(piece);
            n_levels += 1;
        }

        self.context.insert("tduxContent", &self.current_content);

        if n_levels < 2 {
            self.context.insert("tduxRelTop", "");
        } else {
            let mut rel_top = String::default();

            for _ in 0..(n_levels - 1) {
                rel_top.push_str("../");
            }

            self.context.insert("tduxRelTop", &rel_top);
        }

        // Read in the template. Let's not cache it, in case someone wants to do
        // something fancy with rewriting it.

        let mut ih = atry!(
            common.hooks.io().input_open_name(&self.next_template_path, common.status).must_exist();
            ["unable to open input HTML template `{}`", &self.next_template_path]
        );

        let mut template = String::new();
        atry!(
            ih.read_to_string(&mut template);
            ["unable to read input HTML template `{}`", &self.next_template_path]
        );

        let (name, digest_opt) = ih.into_name_digest();
        common
            .hooks
            .event_input_closed(name, digest_opt, common.status);

        // Ready to render!

        let rendered = atry!(
            self.tera.render_str(&template, &self.context);
            ["failed to render HTML template `{}` while creating `{}`", &self.next_template_path, &self.next_output_path]
        );

        // Save it.

        {
            let mut out_file = atry!(
                File::create(&out_path);
                ["cannot open output file `{}`", out_path.display()]
            );

            atry!(
                out_file.write_all(rendered.as_bytes());
                ["cannot write output file `{}`", out_path.display()]
            );
        }

        self.current_content = String::default();
        self.update_content_pos(0, None);
        Ok(())
    }

    fn content_finished(&mut self, common: &mut Common) -> Result<()> {
        if !self.current_content.is_empty() {
            tt_warning!(common.status, "un-emitted content at end of HTML output");
            self.current_content = String::default();
        }

        // The reason we're doing all this: we can now emit our customized font
        // files that provide access to glyphs that we can't get the browser to
        // display directly.

        let mut faces = String::default();

        for (fd_key, data) in self.font_data.drain() {
            data.emit(common.out_base, &format!("tdux{}", fd_key), &mut faces)?;
        }

        self.context.insert("tduxFontFaces", &faces);

        for info in self.fonts.values() {
            if info.role == FontRole::MainBody {
                self.context
                    .insert("tduxMainBodyFontFamily", &format!("tdux{}", info.fd_key));
            }
        }

        // OK.
        self.content_finished = true;
        Ok(())
    }
}

type FixedPoint = i32;

#[allow(dead_code)]
#[derive(Debug)]
struct FontInfo {
    role: FontRole,
    rel_url: String,
    fd_key: usize,
    size: FixedPoint,
    face_index: u32,
    color_rgba: Option<u32>,
    extend: Option<u32>,
    slant: Option<u32>,
    embolden: Option<u32>,
}

#[derive(Debug, Eq, PartialEq)]
enum FontRole {
    MainBody,
}
