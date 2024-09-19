// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

//! State relating to handling the Tera templating and file emission.

use std::{
    fs::File,
    io::{Read, Write},
};
use tectonic_errors::prelude::*;
use tracing::warn;

use crate::Common;

#[derive(Debug)]
pub(crate) struct Templating {
    tera: tera::Tera,
    context: tera::Context,
    next_template_path: String,
    next_output_path: String,
}

impl Templating {
    pub(crate) fn new(
        tera: tera::Tera,
        context: tera::Context,
        next_template_path: String,
        next_output_path: String,
    ) -> Self {
        Templating {
            tera,
            context,
            next_template_path,
            next_output_path,
        }
    }

    pub(crate) fn handle_set_template<S: ToString>(&mut self, arg: S) {
        self.next_template_path = arg.to_string();
    }

    pub(crate) fn handle_set_output_path<S: ToString>(&mut self, arg: S) {
        self.next_output_path = arg.to_string();
    }

    pub(crate) fn handle_set_template_variable(&mut self, remainder: &str) -> Result<()> {
        if let Some((varname, varval)) = remainder.split_once(' ') {
            self.set_variable(varname, varval);
        } else {
            warn!(
                tectonic_log_source = "spx2html",
                "ignoring malformatted tdux:setTemplateVariable special `{}`", remainder
            );
        }

        Ok(())
    }

    pub(crate) fn set_variable<S: AsRef<str>>(&mut self, name: &str, value: S) {
        // Unfortunately tera doesn't seem to give us a way to move an owned
        // value directly into the context object.
        self.context.insert(name, value.as_ref());
    }

    pub(crate) fn ready_to_output(&self) -> bool {
        !self.next_template_path.is_empty() && !self.next_output_path.is_empty()
    }

    pub(crate) fn emit(&mut self, common: &mut Common) -> Result<()> {
        if self.next_template_path.is_empty() {
            bail!("need to emit HTML content but no template has been specified; is your document HTML-compatible?");
        }

        if self.next_output_path.is_empty() {
            bail!("need to emit HTML content but no output path has been specified; is your document HTML-compatible?");
        }

        let (out_path, n_levels) =
            crate::assets::create_output_path(&self.next_output_path, common)?;

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

        let mut ih = atry!(
            common.hooks.io().input_open_name(&self.next_template_path).must_exist();
            ["unable to open input HTML template `{}`", &self.next_template_path]
        );

        let mut template = String::new();
        atry!(
            ih.read_to_string(&mut template);
            ["unable to read input HTML template `{}`", &self.next_template_path]
        );

        let (name, digest_opt) = ih.into_name_digest();
        common.hooks.event_input_closed(name, digest_opt);

        // Ready to render!

        let rendered = atry!(
            self.tera.render_str(&template, &self.context);
            ["failed to render HTML template `{}` while creating `{}`", &self.next_template_path, &self.next_output_path]
        );

        // Save it. Unless we shouldn't, actually.

        if let Some(out_path) = out_path {
            let mut out_file = atry!(
                File::create(&out_path);
                ["cannot open output file `{}`", out_path.display()]
            );

            atry!(
                out_file.write_all(rendered.as_bytes());
                ["cannot write output file `{}`", out_path.display()]
            );
        }

        // Clear the output path, because we don't want people to be accidentally
        // overwriting the same file by failing to update it.

        self.next_output_path.clear();

        Ok(())
    }
}
