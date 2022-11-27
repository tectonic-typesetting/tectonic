// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

//! The finalization phase of SPX to HTML processing.

use tectonic_errors::prelude::*;
use tectonic_status_base::tt_warning;

use crate::{
    assets::Assets, fonts::FontEnsemble, specials::Special, templating::Templating, Common,
};

#[derive(Debug)]
pub(crate) struct FinalizingState {
    fonts: FontEnsemble,
    templating: Templating,
    assets: Assets,
    warning_issued: bool,
}

impl FinalizingState {
    pub(crate) fn new(fonts: FontEnsemble, templating: Templating, assets: Assets) -> Result<Self> {
        Ok(FinalizingState {
            templating,
            fonts,
            assets,
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

            Special::ProvideFile(_) | Special::ProvideSpecial(_) => {
                self.assets.try_handle_special(special, common);
                Ok(())
            }

            other => {
                self.warn_finished_content(&format!("special {}", other), common);
                Ok(())
            }
        }
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

    pub(crate) fn finished(self) -> (FontEnsemble, Assets) {
        (self.fonts, self.assets)
    }
}
