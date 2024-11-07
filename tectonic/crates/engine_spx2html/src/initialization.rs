// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

//! The initialization stage of SPX processing.

use std::{collections::HashMap, io::Read, path::PathBuf};
use tectonic_errors::prelude::*;
use tectonic_io_base::OpenResult;
use tectonic_status_base::tt_warning;

use crate::{
    fonts::FontEnsemble, html::Element, specials::Special, templating::Templating, Common,
    EmittingState, FixedPoint, TexFontNum,
};

#[derive(Debug)]
pub(crate) struct InitializationState {
    templates: HashMap<String, String>,
    next_template_path: String,
    next_output_path: String,

    fonts: FontEnsemble,
    main_body_font_num: Option<i32>,
    tag_associations: HashMap<Element, TexFontNum>,

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
    /// Return true if we're in not in the midst of a multi-step construct like
    /// startDefineFontFamily. In such situations, if we see an event that is
    /// associated with the beginning of the actual content, we should end the
    /// initialization phase.
    pub(crate) fn in_endable_init(&self) -> bool {
        self.cur_font_family_definition.is_none() && self.cur_font_family_tag_associations.is_none()
    }

    /// Handle a "native" font definition.
    ///
    /// The font *name* comes directly from the SPX file and currently
    /// corresponds to the TeX path of a font file that can be opened as an
    /// input, potentially without its extension. In the future, it is possible
    /// that the font name might be something symbolic like "Times New Roman"
    /// that might not work well as a file path.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn handle_define_native_font(
        &mut self,
        name: &str,
        font_num: TexFontNum,
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

        // Figure out the TeX path of the font source. At the moment, this is
        // just the name or something similar, but in principle we might do a
        // lookup based on something like symbolic name.

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

        let ih = a_ok_or!(ih;
            ["failed to find a font file associated with the name `{}`", name]
        );

        // Now that we have that, we can pass off to the font manager.

        self.fonts.register_tex_font(
            font_num, size, face_index, color_rgba, extend, slant, embolden, texpath, ih, common,
        )
    }

    pub(crate) fn handle_special(
        &mut self,
        special: Special<'_>,
        common: &mut Common,
    ) -> Result<()> {
        match special {
            Special::AddTemplate(t) => self.handle_add_template(t, common),
            Special::SetTemplate(t) => self.handle_set_template(t, common),
            Special::SetOutputPath(t) => self.handle_set_output_path(t, common),
            Special::SetTemplateVariable(t) => self.handle_set_template_variable(t, common),
            Special::StartDefineFontFamily => self.handle_start_define_font_family(),
            Special::EndDefineFontFamily => self.handle_end_define_font_family(common),
            Special::StartFontFamilyTagAssociations => {
                self.handle_start_font_family_tag_associations()
            }

            Special::EndFontFamilyTagAssociations => {
                self.handle_end_font_family_tag_associations(common)
            }

            Special::ProvideFile(_) => {
                tt_warning!(common.status, "ignoring too-soon tdux:provideFile special");
                Ok(())
            }

            _ => Ok(()),
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
        texpath.clone_into(&mut self.next_template_path);
        Ok(())
    }

    fn handle_set_output_path(&mut self, texpath: &str, _common: &mut Common) -> Result<()> {
        texpath.clone_into(&mut self.next_output_path);
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
                .register_family(family_name, regular, bold, italic, bold_italic)?;
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
    pub(crate) fn handle_text_and_glyphs(
        &mut self,
        font_num: TexFontNum,
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

    pub(crate) fn initialization_finished(mut self, common: &mut Common) -> Result<EmittingState> {
        // If we have precomputed assets, now is the time to confirm that the
        // fonts defined in this run are a subset of those in the precomputed
        // session, and copy over variant-glyph definitions to be used during
        // the bulk processing.

        if let Some(precomputed) = common.precomputed_assets.as_ref() {
            precomputed.check_runtime_fonts(&mut self.fonts, common)?;
        }

        let mut context = tera::Context::default();

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

        let templating = Templating::new(
            tera,
            context,
            self.next_template_path,
            self.next_output_path,
        );

        // Ready to hand off.

        EmittingState::new_from_init(
            self.fonts,
            self.main_body_font_num,
            templating,
            self.tag_associations,
        )
    }
}

#[derive(Debug, Default)]
struct FontFamilyBuilder {
    family_name: String,
    regular: Option<TexFontNum>,
    bold: Option<TexFontNum>,
    italic: Option<TexFontNum>,
    bold_italic: Option<TexFontNum>,
}

#[derive(Debug, Default)]
struct FontFamilyTagAssociator {
    assoc: HashMap<Element, TexFontNum>,
}
