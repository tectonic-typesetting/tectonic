// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

//! The finalization phase of SPX to HTML processing.

use std::{fs::File, path::Path};
use tectonic_errors::prelude::*;
use tectonic_status_base::tt_warning;

use crate::{fontfamily::FontEnsemble, specials::Special, templating::Templating, Common};

#[derive(Debug)]
pub(crate) struct FinalizingState {
    //fonts: FontEnsemble,
    templating: Templating,
    warning_issued: bool,
}

impl FinalizingState {
    pub(crate) fn new(
        mut fonts: FontEnsemble,
        mut templating: Templating,
        out_base: &Path,
    ) -> Result<Self> {
        // *For now*, set up to emit font CSS here.
        let faces = fonts.emit(out_base)?;
        templating.set_variable("tduxFontFaces", &faces);

        Ok(FinalizingState {
            templating,
            //fonts,
            warning_issued: false,
        })
    }

    fn warn_finished_content(&mut self, detail: &str, common: &mut Common) {
        if !self.warning_issued {
            tt_warning!(common.status, "dropping post-finish content ({})", detail);
            self.warning_issued = true;
        }
    }

    pub(crate) fn handle_special(
        &mut self,
        special: Special<'_>,
        common: &mut Common,
    ) -> Result<()> {
        match special {
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

            Special::ProvideFile(spec) => self.handle_provide_file(spec, common),

            other => {
                self.warn_finished_content(&format!("special {}", other), common);
                Ok(())
            }
        }
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

    pub(crate) fn handle_text_and_glyphs(&mut self, text: &str, common: &mut Common) -> Result<()> {
        self.warn_finished_content(&format!("text `{}`", text), common);
        Ok(())
    }

    pub(crate) fn handle_glyph_run(&mut self, common: &mut Common) -> Result<()> {
        self.warn_finished_content("glyph run", common);
        Ok(())
    }

    fn finish_file(&mut self, common: &mut Common) -> Result<()> {
        self.templating.set_variable("tduxContent", "");
        self.templating.emit(common)?;
        Ok(())
    }

    pub(crate) fn finished(&mut self, _common: &mut Common) -> Result<()> {
        // For now, nothing to do here.
        Ok(())
    }
}
