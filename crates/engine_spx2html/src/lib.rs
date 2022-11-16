// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

#![deny(missing_docs)]

//! Convert Tectonic’s SPX format to HTML.
//!
//! SPX is essentially the same thing as XDV, but we identify it differently to
//! mark that the semantics of the content wil be set up for HTML output.

use std::{
    collections::HashMap,
    fmt::{Arguments, Error as FmtError, Write as FmtWrite},
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    result::Result as StdResult,
};
use tectonic_bridge_core::DriverHooks;
use tectonic_errors::prelude::*;
use tectonic_io_base::OpenResult;
use tectonic_status_base::{tt_warning, StatusBackend};
use tectonic_xdv::{FileType, XdvEvents, XdvParser};

use crate::fontfamily::{FamilyRelativeFontId, FontEnsemble, FontFamilyAnalysis, PathToNewFont};

mod fontfamily;
mod fontfile;
mod html;

use html::Element;

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
            if !s.content.is_empty() {
                s.finish_file(&mut self.common)?;
            }
        }

        Ok(())
    }

    /// Return true if we're in the initializing phase, but not in the midst of
    /// a multi-step construct like startDefineFontFamily. In such situations,
    /// if we see an event that is associated with the beginning of the actual
    /// content, we should end the initialization phase.
    fn in_endable_init(&self) -> bool {
        match &self.state {
            State::Invalid => false,
            State::Initializing(s) => {
                s.cur_font_family_definition.is_none()
                    && s.cur_font_family_tag_associations.is_none()
            }
            State::Emitting(_) => false,
        }
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

        // str.split_once() would be nice but it was introduced in 1.52 which is
        // a bit recent for us.

        let mut pieces = contents.splitn(2, ' ');

        let (tdux_command, remainder) = if let Some(p) = pieces.next() {
            if let Some(cmd) = p.strip_prefix("tdux:") {
                (Some(cmd), pieces.next().unwrap_or_default())
            } else {
                (None, contents)
            }
        } else {
            (None, contents)
        };

        // Might we need to end the initialization phase?

        if self.in_endable_init() {
            let end_init = matches!(
                tdux_command.unwrap_or("none"),
                "emit" | "provideFile" | "asp" | "aep" | "cs" | "ce" | "mfs" | "me" | "dt"
            );

            if end_init {
                self.state.ensure_initialized()?;
            }
        }

        // Ready to dispatch.

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(s) => s.handle_special(tdux_command, remainder, &mut self.common),
            State::Emitting(s) => s.handle_special(x, y, tdux_command, remainder, &mut self.common),
        }
    }

    fn handle_text_and_glyphs(
        &mut self,
        font_num: FontNum,
        text: &str,
        _width: i32,
        glyphs: &[u16],
        x: &[i32],
        y: &[i32],
    ) -> Result<()> {
        if self.in_endable_init() {
            self.state.ensure_initialized()?;
        }

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(s) => {
                s.handle_text_and_glyphs(font_num, text, glyphs, x, y, &mut self.common)?
            }
            State::Emitting(s) => {
                s.handle_text_and_glyphs(font_num, text, glyphs, x, y, &mut self.common)?
            }
        }

        Ok(())
    }

    fn handle_define_native_font(
        &mut self,
        name: &str,
        font_num: FontNum,
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
        font_num: FontNum,
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

    fonts: FontEnsemble,
    main_body_font_num: Option<i32>,
    tag_associations: HashMap<Element, FontNum>,

    cur_font_family_definition: Option<FontFamilyBuilder>,
    cur_font_family_tag_associations: Option<FontFamilyTagAssociator>,

    variables: HashMap<String, String>,
}

impl Default for InitializationState {
    fn default() -> Self {
        InitializationState {
            templates: Default::default(),
            next_template_path: Default::default(),
            next_output_path: "index.html".to_owned(),

            fonts: Default::default(),
            main_body_font_num: None,
            tag_associations: Default::default(),

            cur_font_family_definition: None,
            cur_font_family_tag_associations: None,

            variables: Default::default(),
        }
    }
}

impl InitializationState {
    #[allow(clippy::too_many_arguments)]
    fn handle_define_native_font(
        &mut self,
        name: &str,
        font_num: FontNum,
        size: FixedPoint,
        face_index: u32,
        color_rgba: Option<u32>,
        extend: Option<u32>,
        slant: Option<u32>,
        embolden: Option<u32>,
        common: &mut Common,
    ) -> Result<()> {
        if self.fonts.contains(font_num) {
            // Should we override the definition or something?
            return Ok(());
        }

        // TODO: often there are multiple font_nums with the same "name". We
        // only need to copy the file once.

        let io = common.hooks.io();
        let mut texpath = String::default();
        let mut ih = None;

        for ext in &["", ".otf"] {
            texpath = format!("{name}{ext}");

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

        self.fonts.register(
            name,
            font_num,
            size,
            face_index,
            color_rgba,
            extend,
            slant,
            embolden,
            basename.to_owned(),
            contents,
        )
    }

    fn handle_special(
        &mut self,
        tdux_command: Option<&str>,
        remainder: &str,
        common: &mut Common,
    ) -> Result<()> {
        if let Some(cmd) = tdux_command {
            match cmd {
                "addTemplate" => self.handle_add_template(remainder, common),
                "setTemplate" => self.handle_set_template(remainder, common),
                "setOutputPath" => self.handle_set_output_path(remainder, common),
                "setTemplateVariable" => self.handle_set_template_variable(remainder, common),

                "startDefineFontFamily" => self.handle_start_define_font_family(),
                "endDefineFontFamily" => self.handle_end_define_font_family(common),

                "startFontFamilyTagAssociations" => {
                    self.handle_start_font_family_tag_associations()
                }

                "endFontFamilyTagAssociations" => {
                    self.handle_end_font_family_tag_associations(common)
                }

                "provideFile" => {
                    tt_warning!(common.status, "ignoring too-soon tdux:provideFile special");
                    Ok(())
                }

                _ => Ok(()),
            }
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

    // "Font family" definitions, allowing us to synthesize bold/italic tags
    // based on tracking font changes, and also to know what the main body font
    // is.

    fn handle_start_define_font_family(&mut self) -> Result<()> {
        self.cur_font_family_definition = Some(FontFamilyBuilder::default());
        Ok(())
    }

    fn handle_end_define_font_family(&mut self, common: &mut Common) -> Result<()> {
        if let Some(b) = self.cur_font_family_definition.take() {
            let family_name = b.family_name;
            let regular = a_ok_or!(b.regular; ["no regular face defined"]);
            let bold = a_ok_or!(b.bold; ["no bold face defined"]);
            let italic = a_ok_or!(b.italic; ["no italic face defined"]);
            let bold_italic = a_ok_or!(b.bold_italic; ["no bold-italic face defined"]);

            self.fonts
                .register_family(family_name, regular, bold, italic, bold_italic);
        } else {
            tt_warning!(
                common.status,
                "end of font-family definition block that didn't start"
            );
        }

        Ok(())
    }

    // "Font family tag associations", telling us which font family is the
    // default depending on which tag we're in. For instance, typical templates
    // will default to the monospace font inside `<code>` tags.

    fn handle_start_font_family_tag_associations(&mut self) -> Result<()> {
        self.cur_font_family_tag_associations = Some(FontFamilyTagAssociator::default());
        Ok(())
    }

    fn handle_end_font_family_tag_associations(&mut self, common: &mut Common) -> Result<()> {
        if let Some(mut a) = self.cur_font_family_tag_associations.take() {
            for (k, v) in a.assoc.drain() {
                self.tag_associations.insert(k, v);
            }
        } else {
            tt_warning!(
                common.status,
                "end of font-family tag-association block that didn't start"
            );
        }

        Ok(())
    }

    /// In the initialization state, this should only get called if we're in a
    /// font-family definition (in which case we're using the contents to learn
    /// the definition of a font family). Otherwise, the higher-level callback
    /// will declare initialization done and move to the emitting state.
    fn handle_text_and_glyphs(
        &mut self,
        font_num: FontNum,
        text: &str,
        _glyphs: &[u16],
        _xs: &[i32],
        _ys: &[i32],
        common: &mut Common,
    ) -> Result<()> {
        if let Some(b) = self.cur_font_family_definition.as_mut() {
            if text.starts_with("bold-italic") {
                b.bold_italic = Some(font_num);
            } else if text.starts_with("bold") {
                b.bold = Some(font_num);
            } else if text.starts_with("italic") {
                b.italic = Some(font_num);
            } else {
                b.regular = Some(font_num);
                b.family_name = if let Some(fname) = text.strip_prefix("family-name:") {
                    fname.to_owned()
                } else {
                    format!("tdux{font_num}")
                };

                // Say that the "regular" font of the first font family definition
                // is the main body font.
                if self.main_body_font_num.is_none() {
                    self.main_body_font_num = Some(font_num);
                }
            }
        } else if let Some(a) = self.cur_font_family_tag_associations.as_mut() {
            for tagname in text.split_whitespace() {
                let el: Element = tagname.parse().unwrap();
                a.assoc.insert(el, font_num);
            }
        } else {
            // This shouldn't happen; the top-level processor should exit init
            // phase if it's invoked and none of the above cases hold.
            tt_warning!(
                common.status,
                "internal bug; losing text `{}` in initialization phase",
                text
            );
        }

        Ok(())
    }

    fn initialization_finished(self) -> Result<EmittingState> {
        let mut context = tera::Context::default();

        // Set up font stuff.

        let rems_per_tex = if let Some(fnum) = self.main_body_font_num {
            1.0 / (self.fonts.get_font_size(fnum) as f32)
        } else {
            1. / 65536.
        };

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

        // Other context initialization, with the possibility of overriding
        // stuff that's been set up earlier.

        for (varname, varvalue) in self.variables {
            context.insert(varname, &varvalue);
        }

        // All done!

        Ok(EmittingState {
            tera,
            context,
            fonts: self.fonts,
            tag_associations: self.tag_associations,
            rems_per_tex,
            next_template_path: self.next_template_path,
            next_output_path: self.next_output_path,
            content: Default::default(),
            elem_stack: vec![ElementState {
                elem: None,
                origin: ElementOrigin::Root,
                do_auto_tags: true,
                do_auto_spaces: true,
                font_family_id: self.main_body_font_num.unwrap_or_default(),
                active_font: FamilyRelativeFontId::Regular,
            }],
            current_canvas: None,
            content_finished: false,
            content_finished_warning_issued: false,
        })
    }
}

#[derive(Debug)]
struct EmittingState {
    tera: tera::Tera,
    context: tera::Context,
    fonts: FontEnsemble,
    content: ContentState,
    tag_associations: HashMap<Element, FontNum>,

    rems_per_tex: f32,
    next_template_path: String,
    next_output_path: String,
    elem_stack: Vec<ElementState>,
    current_canvas: Option<CanvasState>,
    content_finished: bool,
    content_finished_warning_issued: bool,
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
struct ElementState {
    /// The associated HTML element. This is None for the bottom item in the
    /// stack, or for changes in state that are not associated with actual HTML
    /// tags.
    elem: Option<html::Element>,

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
    font_family_id: FontNum,

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
enum ElementOrigin {
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
    font_num: FontNum,
    glyph: u16,
}

impl EmittingState {
    fn warn_finished_content(&mut self, detail: &str, common: &mut Common) {
        if !self.content_finished_warning_issued {
            tt_warning!(common.status, "dropping post-finish content ({})", detail);
            self.content_finished_warning_issued = true;
        }
    }

    /// Convenience helper that applies the right defaults here.
    ///
    /// We can't always use this function because sometimes we need mutable
    /// access to the `fonts` and `content` items separately.
    fn push_space_if_needed(&mut self, x0: i32, fnum: Option<FontNum>) {
        let cur_space_width = self.fonts.maybe_get_font_space_width(fnum);
        self.content
            .push_space_if_needed(x0, cur_space_width, self.cur_elstate().do_auto_spaces);
    }

    fn create_elem(&self, name: &str, is_start: bool, common: &mut Common) -> Element {
        // Parsing can never fail since we offer an `Other` element type
        let el: html::Element = name.parse().unwrap();

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

    fn handle_special(
        &mut self,
        x: i32,
        y: i32,
        tdux_command: Option<&str>,
        remainder: &str,
        common: &mut Common,
    ) -> Result<()> {
        if let Some(cmd) = tdux_command {
            match cmd {
                "asp" => {
                    if self.content_finished {
                        self.warn_finished_content("auto start paragraph", common);
                    } else if self.cur_elstate().do_auto_tags {
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

                "aep" => {
                    if self.content_finished {
                        self.warn_finished_content("auto end paragraph", common);
                    } else if self.cur_elstate().do_auto_tags {
                        self.pop_elem("div", common);
                    }
                    Ok(())
                }

                "cs" => {
                    if self.content_finished {
                        self.warn_finished_content("canvas start", common);
                    } else if let Some(canvas) = self.current_canvas.as_mut() {
                        canvas.depth += 1;
                    } else {
                        self.current_canvas = Some(CanvasState::new(remainder, x, y));
                    }
                    Ok(())
                }

                "ce" => {
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
                            "ignoring unpaired tdux:c[anvas]e[nd] special for `{}`",
                            remainder
                        );
                    }
                    Ok(())
                }

                "mfs" => {
                    if self.content_finished {
                        self.warn_finished_content(
                            &format!("manual flexible start tag {remainder:?}"),
                            common,
                        );
                        Ok(())
                    } else {
                        self.handle_flexible_start_tag(x, y, remainder, common)
                    }
                }

                "me" => {
                    if self.content_finished {
                        self.warn_finished_content(
                            &format!("manual end tag </{remainder}>"),
                            common,
                        );
                    } else {
                        self.pop_elem(remainder, common);
                    }
                    Ok(())
                }

                "dt" => {
                    if self.content_finished {
                        self.warn_finished_content("direct text", common);
                    } else {
                        self.content.push_with_html_escaping(remainder);
                    }
                    Ok(())
                }

                "emit" => self.finish_file(common),

                "setTemplate" => {
                    self.next_template_path = remainder.to_owned();
                    Ok(())
                }

                "setOutputPath" => {
                    self.next_output_path = remainder.to_owned();
                    Ok(())
                }

                "setTemplateVariable" => self.handle_set_template_variable(remainder, common),

                "provideFile" => self.handle_provide_file(remainder, common),

                "contentFinished" => self.content_finished(common),

                other => {
                    tt_warning!(
                        common.status,
                        "ignoring unrecognized special: tdux:{} {}",
                        other,
                        remainder
                    );
                    Ok(())
                }
            }
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
        font_num: FontNum,
        text: &str,
        glyphs: &[u16],
        xs: &[i32],
        ys: &[i32],
        common: &mut Common,
    ) -> Result<()> {
        if self.content_finished {
            self.warn_finished_content(&format!("text `{text}`"), common);
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

    fn handle_glyph_run(
        &mut self,
        font_num: FontNum,
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

                    write!(self.content, "<span style=\"{}\">", font_sel).unwrap();
                    self.content.push_with_html_escaping(ch_as_str);
                    write!(self.content, "</span>").unwrap();
                }

                self.content
                    .update_content_pos(xs[idx] + advance, cur_space_width);
            }
        }

        Ok(())
    }

    fn set_up_for_font(&mut self, x0: i32, fnum: FontNum, common: &mut Common) {
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
            FontFamilyAnalysis::NoMatch => {
                // We don't seem to be in a defined "family". So we have to
                // select it explicitly.
                let path = PathToNewFont {
                    close_all: true,
                    select_explicitly: true,
                    ..Default::default()
                };

                let desired_af = FamilyRelativeFontId::Other(fnum);
                (path, desired_af)
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
                elem: Some(html::Element::B),
                origin: ElementOrigin::FontAuto,
                active_font: af,
                ..*self.cur_elstate()
            });
        }

        if let Some(af) = path.open_i {
            self.push_space_if_needed(x0, Some(fnum));
            self.content.push_str("<i>");
            self.elem_stack.push(ElementState {
                elem: Some(html::Element::I),
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
                elem: Some(html::Element::Span),
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
            let (size, baseline_factor, text_info) =
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

        // Unfortunately tera doesn't seem to give us a way to put the content
        // string in without having to clone it.
        self.context.insert("tduxContent", &self.content.take());

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
        // something fancy with rewriting it. If that setting is empty, probably
        // the user is compiling the document in HTML mode without all of the
        // TeX infrastructure that Tectonic needs to make it work.

        if self.next_template_path.is_empty() {
            bail!("need to emit HTML content but no template has been specified; is your document HTML-compatible?");
        }

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

        let cur_space_width = self.fonts.maybe_get_font_space_width(None);
        self.content.update_content_pos(0, cur_space_width);
        Ok(())
    }

    fn content_finished(&mut self, common: &mut Common) -> Result<()> {
        if !self.content.is_empty() {
            tt_warning!(common.status, "un-emitted content at end of HTML output");
            self.content.take(); // clear out the content
        }

        let faces = self.fonts.emit(common.out_base)?;
        self.context.insert("tduxFontFaces", &faces);

        // OK.
        self.content_finished = true;
        Ok(())
    }
}

type FixedPoint = i32;
type FontNum = i32;

#[derive(Debug, Default)]
struct FontFamilyBuilder {
    family_name: String,
    regular: Option<FontNum>,
    bold: Option<FontNum>,
    italic: Option<FontNum>,
    bold_italic: Option<FontNum>,
}

#[derive(Debug, Default)]
struct FontFamilyTagAssociator {
    assoc: HashMap<Element, FontNum>,
}
