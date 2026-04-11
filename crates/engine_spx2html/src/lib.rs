// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

//! Convert Tectonic’s SPX format to HTML.
//!
//! SPX is essentially the same thing as XDV, but we identify it differently to
//! mark that the semantics of the content wil be set up for HTML output.

use std::path::{Path, PathBuf};
use tectonic_bridge_core::DriverHooks;
use tectonic_errors::prelude::*;
use tectonic_status_base::StatusBackend;
use tectonic_xdv::{FileType, XdvEvents, XdvParser};

mod assets;
mod emission;
mod finalization;
mod fontfile;
mod fonts;
mod html;
mod initialization;
mod specials;
mod templating;

use self::{
    assets::Assets, emission::EmittingState, finalization::FinalizingState, fonts::FontEnsemble,
    initialization::InitializationState, specials::Special,
};

/// An engine that converts SPX to HTML.
#[derive(Debug, Default)]
pub struct Spx2HtmlEngine {
    output: OutputState,
    precomputed_assets: Option<AssetSpecification>,
    assets_spec_path: Option<String>,
    do_not_emit_assets: bool,
}

#[derive(Debug, Default)]
enum OutputState {
    #[default]
    Undefined,
    NoOutput,
    Path(PathBuf),
}

impl Spx2HtmlEngine {
    /// Emit an asset specification file and not actual assets.
    ///
    /// "Assets" are files like fonts and images that accompany the HTML output
    /// generated during processing. SPX files contain commands that implicitly
    /// and explicitly create assets. By default, these are emitted during
    /// processing. If this method is called, the assets will *not* be created,
    /// as if you called [`Self::do_not_emit_assets`]. Instead, an "asset
    /// specification" file will be emitted to the given output path. This
    /// specification file contains the information needed to generate the
    /// assets upon a later invocation. Asset specification files can be merged,
    /// allowing the results of multiple separate TeX compilations to be
    /// synthesized into one HTML output tree.
    ///
    /// Currently, the asset specification is written in JSON format, although
    /// it is not guaranteed that this will always be the case. It will always
    /// be a UTF8-encoded, line-oriented textual format, though.
    pub fn assets_spec_path<S: ToString>(&mut self, path: S) -> &mut Self {
        self.assets_spec_path = Some(path.to_string());
        self
    }

    /// Specify that this session should use a precomputed asset specification.
    ///
    /// If this function is used, subsequent runs will generate HTML outputs
    /// assuming the information given in the asset specification. If the input
    /// calls for new assets or different options inconsistent with the
    /// specification, processing will abort with an error.
    ///
    /// The purpose of this mode is to allow for a unified set of assets to be
    /// created from multiple independent runs of the SPX-to-HTML stage. First,
    /// the different inputs should be processed independently, and their
    /// individual assets should saved. These should then be merged. Then the
    /// inputs should be reprocessed, all using the merged asset specification.
    /// In one — but only one — of these sessions, the assets should actually be
    /// emitted.
    pub fn precomputed_assets(&mut self, assets: AssetSpecification) -> &mut Self {
        self.precomputed_assets = Some(assets);
        self
    }

    /// Specify that templated output files should not actually be created.
    ///
    /// You probably want this engine to actually write its outputs to the
    /// filesystem. If you call this function, it will not. This mode can be
    /// useful if the main purpose of the processing run is to gather
    /// information about the assets that will be generated.
    pub fn do_not_emit_files(&mut self) -> &mut Self {
        self.output = OutputState::NoOutput;
        self
    }

    /// Specify that supporting "asset" files should not actually be created.
    ///
    /// You probably want this engine to actually write these assets to the
    /// filesystem. If you call this function, it will not. This mode can be
    /// useful if the main purpose of the processing run is to gather
    /// information about the assets that will be generated.
    ///
    /// Calling [`Self::assets_spec_path`] has the same effect as this function,
    /// but also causes an asset specification file to be written to in
    /// Tectonic's virtual I/O backend.
    pub fn do_not_emit_assets(&mut self) -> &mut Self {
        self.do_not_emit_assets = true;
        self
    }

    /// Specify the root path for output files.
    ///
    /// Because this driver will, in the generic case, produce a tree of HTML
    /// output files that are not going to be used as a basis for any subsequent
    /// engine stages, it outputs directly to disk rather than using the I/O
    /// layer. I don't like hardcoding use of the filesystem, but I don't want
    /// to build up some extra abstraction layer right now.
    pub fn output_base(&mut self, out_base: impl Into<PathBuf>) -> &mut Self {
        self.output = OutputState::Path(out_base.into());
        self
    }

    /// Process SPX into HTML.
    ///
    /// Before calling this function, you must explicitly specify the output
    /// mode by calling either [`Self::do_not_emit_files`] or
    /// [`Self::output_base`]. If you do not, this function will panic.
    pub fn process_to_filesystem(
        &mut self,
        hooks: &mut dyn DriverHooks,
        status: &mut dyn StatusBackend,
        spx: &str,
    ) -> Result<()> {
        let mut input = hooks.io().input_open_name(spx, status).must_exist()?;

        let out_base = match self.output {
            OutputState::NoOutput => None,
            OutputState::Path(ref p) => Some(p.as_ref()),
            OutputState::Undefined => panic!("spx2html output mode not specified"),
        };

        {
            let state = EngineState::new(hooks, status, out_base, self.precomputed_assets.as_ref());
            let state = XdvParser::process_with_seeks(&mut input, state)?;
            let (fonts, assets, mut common) = state.finished()?;

            if let Some(asp) = self.assets_spec_path.as_ref() {
                let ser = assets.into_serialize(fonts);
                let mut output = hooks.io().output_open_name(asp).must_exist()?;
                serde_json::to_writer_pretty(&mut output, &ser)?;
                let (name, digest) = output.into_name_digest();
                hooks.event_output_closed(name, digest);
            } else if !self.do_not_emit_assets {
                assets.emit(fonts, &mut common)?;
            }
        }

        let (name, digest_opt) = input.into_name_digest();
        hooks.event_input_closed(name, digest_opt, status);
        Ok(())
    }
}

pub use assets::AssetSpecification;

struct EngineState<'a> {
    common: Common<'a>,
    state: State,
}

struct Common<'a> {
    hooks: &'a mut dyn DriverHooks,
    status: &'a mut dyn StatusBackend,
    out_base: Option<&'a Path>,
    precomputed_assets: Option<&'a AssetSpecification>,
}

impl<'a> EngineState<'a> {
    pub fn new(
        hooks: &'a mut dyn DriverHooks,
        status: &'a mut dyn StatusBackend,
        out_base: Option<&'a Path>,
        precomputed_assets: Option<&'a AssetSpecification>,
    ) -> Self {
        Self {
            common: Common {
                hooks,
                status,
                out_base,
                precomputed_assets,
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
    Finalizing(FinalizingState),
}

impl<'a> EngineState<'a> {
    pub fn finished(mut self) -> Result<(FontEnsemble, Assets, Common<'a>)> {
        self.state.ensure_finalizing(&mut self.common)?;

        if let State::Finalizing(s) = self.state {
            let (fonts, mut assets) = s.finished();

            // If we have precomputed assets, make sure that this run didn't
            // define anything surprising, and sync up the runtime manifest with
            // the precomputed one so that we emit everything if needed.
            if let Some(precomputed) = self.common.precomputed_assets {
                precomputed.check_runtime_assets(&mut assets)?;
            }

            Ok((fonts, assets, self.common))
        } else {
            panic!("invalid spx2html finalization state leaked");
        }
    }

    /// Return true if we're in the initializing phase, but not in the midst of
    /// a multi-step construct like startDefineFontFamily. In such situations,
    /// if we see an event that is associated with the beginning of the actual
    /// content, we should end the initialization phase.
    fn in_endable_init(&self) -> bool {
        match &self.state {
            State::Initializing(s) => s.in_endable_init(),
            _ => false,
        }
    }
}

impl XdvEvents for EngineState<'_> {
    type Error = Error;

    fn handle_header(&mut self, filetype: FileType, _comment: &[u8]) -> Result<()> {
        if filetype != FileType::Spx {
            bail!("file should be SPX format but got {}", filetype);
        }

        Ok(())
    }

    fn handle_special(&mut self, x: i32, y: i32, contents: &[u8]) -> Result<()> {
        let contents = atry!(std::str::from_utf8(contents); ["could not parse \\special as UTF-8"]);

        let special = match Special::parse(contents, self.common.status) {
            Some(s) => s,
            None => return Ok(()),
        };

        // Might we need to end the initialization phase?

        if self.in_endable_init() && special.ends_initialization() {
            self.state.ensure_initialized(&mut self.common)?;
        }

        // Might we be entering the finalization phase?

        if let Special::ContentFinished = special {
            return self.state.ensure_finalizing(&mut self.common);
        }

        // Ready to dispatch.

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(s) => s.handle_special(special, &mut self.common),
            State::Emitting(s) => s.handle_special(x, y, special, &mut self.common),
            State::Finalizing(s) => s.handle_special(special, &mut self.common),
        }
    }

    fn handle_text_and_glyphs(
        &mut self,
        font_num: TexFontNum,
        text: &str,
        _width: i32,
        glyphs: &[u16],
        x: &[i32],
        y: &[i32],
    ) -> Result<()> {
        if self.in_endable_init() {
            self.state.ensure_initialized(&mut self.common)?;
        }

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(s) => {
                s.handle_text_and_glyphs(font_num, text, glyphs, x, y, &mut self.common)?
            }
            State::Emitting(s) => {
                s.handle_text_and_glyphs(font_num, text, glyphs, x, y, &mut self.common)?
            }
            State::Finalizing(s) => s.handle_text_and_glyphs(text, &mut self.common)?,
        }

        Ok(())
    }

    fn handle_define_native_font(
        &mut self,
        name: &str,
        font_num: TexFontNum,
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
        font_num: TexFontNum,
        glyphs: &[u16],
        x: &[i32],
        y: &[i32],
    ) -> Result<(), Self::Error> {
        self.state.ensure_initialized(&mut self.common)?;

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(_) => unreachable!(),
            State::Emitting(s) => s.handle_glyph_run(font_num, glyphs, x, y, &mut self.common),
            State::Finalizing(s) => s.handle_glyph_run(&mut self.common),
        }
    }

    fn handle_rule(&mut self, x: i32, y: i32, height: i32, width: i32) -> Result<(), Self::Error> {
        self.state.ensure_initialized(&mut self.common)?;

        match &mut self.state {
            State::Invalid => panic!("invalid spx2html state leaked"),
            State::Initializing(_) => unreachable!(),
            State::Emitting(s) => s.handle_rule(x, y, height, width, &mut self.common),
            State::Finalizing(s) => s.handle_rule(&mut self.common),
        }
    }
}

impl State {
    fn ensure_initialized(&mut self, common: &mut Common) -> Result<()> {
        // Is this the least-bad way to do this??
        let mut work = std::mem::replace(self, State::Invalid);

        if let State::Initializing(s) = work {
            work = State::Emitting(s.initialization_finished(common)?);
        }

        std::mem::swap(self, &mut work);
        Ok(())
    }

    fn ensure_finalizing(&mut self, common: &mut Common) -> Result<()> {
        self.ensure_initialized(common)?;

        let mut work = std::mem::replace(self, State::Invalid);

        if let State::Emitting(s) = work {
            work = State::Finalizing(s.emission_finished(common)?);
        }

        std::mem::swap(self, &mut work);
        Ok(())
    }
}

type FixedPoint = i32;
type TexFontNum = i32;
