// Copyright 2018-2022 the Tectonic Project
// Licensed under the MIT License.

//! State relating to handling the Tera templating and file emission.

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use tectonic_errors::prelude::*;
use tectonic_status_base::tt_warning;

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

    pub(crate) fn handle_set_template_variable(
        &mut self,
        remainder: &str,
        common: &mut Common,
    ) -> Result<()> {
        if let Some((varname, varval)) = remainder.split_once(' ') {
            self.set_variable(varname, varval);
        } else {
            tt_warning!(
                common.status,
                "ignoring malformatted tdux:setTemplateVariable special `{}`",
                remainder
            );
        }

        Ok(())
    }

    pub(crate) fn set_variable<S: AsRef<str>>(&mut self, name: &str, value: S) {
        // Unfortunately tera doesn't seem to give us a way to move an owned
        // value directly into the context object.
        self.context.insert(name, value.as_ref());
    }

    pub(crate) fn emit(&mut self, common: &mut Common) -> Result<()> {
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

        // Save it. Unless we shouldn't, actually.

        if !common.do_not_emit {
            let mut out_file = atry!(
                File::create(&out_path);
                ["cannot open output file `{}`", out_path.display()]
            );

            atry!(
                out_file.write_all(rendered.as_bytes());
                ["cannot write output file `{}`", out_path.display()]
            );
        }

        Ok(())
    }
}
